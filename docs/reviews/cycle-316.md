# Cycle 316 Review

## 1. [worklog-accuracy] The published worklog still reports a green pipeline after the final gate failed

**File**: `docs/worklog/2026-03-20/031117-cycle-316-doc-validation-fix-merged-and-review-processed.md:27-32`
**Evidence**: The published worklog says `Pipeline status: PASS (doc-validation circular dependency fixed)`. But the cycle's own Step `C5.5` comment on issue `#1517` reports `Pipeline: FAIL` with a blocking `doc-validation` failure (`https://github.com/EvaLok/schema-org-json-ld/issues/1517#issuecomment-4095168389`). The same issue thread shows the orchestrator proceeded through `C6`, `C7`, and `C8` anyway, and the final `C8` summary still claimed `Pipeline PASS`, so the published artifact tells a cleaner story than the blocking gate actually produced.
**Recommendation**: Fail close-out on any blocking `C5.5` result, or automatically patch the published worklog/journal from the final gate output instead of leaving a hand-written PASS narrative in place.

## 2. [receipt-integrity] The receipt note says the worklog was validated, but the published artifact no longer passes `receipt-validate`

**File**: `docs/worklog/2026-03-20/031117-cycle-316-doc-validation-fix-merged-and-review-processed.md:40-52`
**Evidence**: The note says the table was `Validated by receipt-validate at step C5.1`. On the published `origin/master` tree, however, `bash tools/receipt-validate --cycle 316 --worklog docs/worklog/2026-03-20/031117-cycle-316-doc-validation-fix-merged-and-review-processed.md` fails with three genuinely missing receipts: `fdbefdfb0fa62ba961a11004036bfa22e5706da0` (`chore: remove duplicate empty worklog file [cycle 316]`), `c607ae5b7ac1d380c5d639e03eb3eae5d413ad19` (`docs(worklog-patch): post-dispatch state correction [cycle 316]`), and `795fb38577c6d8434c31e9ad282f09e2f6129fc6` (`docs(review-body): cycle 316 review dispatch body [cycle 316]`). The validator only treats two docs commits as structurally excluded, so the published note is stale relative to the final tree.
**Recommendation**: Re-run receipt validation after every post-`C5.1` docs mutation, or change the generated note/tooling so it explicitly describes a `C5.1` snapshot rather than certifying the final published artifact.

## 3. [process-adherence] The current-cycle step checker counted the wrong cycle's Step 0 as if cycle 316 had posted it

**File**: `tools/rust/crates/pipeline-check/src/main.rs:26-57,970-1039,1177-1200`
**Evidence**: On issue `#1517`, the earliest numbered step comment is `Cycle 315 | Step 0` (`https://github.com/EvaLok/schema-org-json-ld/issues/1517#issuecomment-4095100381`), and the cycle 316 comment stream contains no `Step 0` entry at all. A direct parse of the issue comments shows cycle 316 posted `0.5, 0.6, 1, 1.1, 2, 3, 4, 5, 6, 7, 8, 9, C1, C2, C3, C4.1, C4.5, C5, C5.1, C5.5, C5.6, C6, C6.5, C7, C8`, with `0` absent. Yet the `C5.5` pipeline report says `issue #1517: 20 pre-gate mandatory steps present [0, 0.5, 0.6, ...]`. The code explains why: `fetch_step_comments_for_issue` collapses all comment bodies into step IDs via `collect_step_comment_ids`, and `detect_step_comment_id` only extracts the token after `Step ` without checking the `Cycle N` marker. That lets a wrong-cycle step comment satisfy cycle 316's mandatory checklist.
**Recommendation**: Make `current-cycle-steps` cycle-aware by validating the `Cycle {cycle}` marker before accepting a step token, and add a regression test where a `Cycle X-1 | Step 0` comment appears on the current issue so the check fails instead of passing falsely.

## 4. [journal-quality] The journal headline declares the pipeline "fully operational" even though the same cycle overrode a blocking failure

**File**: `docs/journal/2026-03-20.md:39-67`
**Evidence**: The cycle 316 journal entry is titled `Doc-validation fix landed, pipeline fully operational` and its context says the cycle `resolved the doc-validation circular dependency`. But the cycle's own `C5.5` gate still reported `Pipeline: FAIL` because `doc-validation` failed on the worklog, and the published worklog still needed a post-dispatch correction commit (`c607ae5`) afterward. Even after that patch, the published worklog still fails `receipt-validate` on `origin/master`, so "fully operational" overstates the result and blunts the cycle's reflective value.
**Recommendation**: Keep journal headings and context tied to the final observable gate result, and split future commitments into separate, testable bullets (for example: `re-run close-out without doc-validation recursion`, `repair receipt validation drift`, `dispatch remaining post-stabilization improvements`) instead of one bundled sentence.

## Complacency score

**2/5** — The cycle did land a real fix (`PR #1514`), and the published master tree passes `state-invariants`, `metric-snapshot`, and `validate-docs`. But the score is capped below 3/5 by the explicit blocking-gate override: issue `#1517` records a `C5.5` `Pipeline: FAIL`, yet the orchestrator still declared completion and published PASS language in both the worklog and journal. Combined with another receipt-validation mismatch and a still-false-green step-comment check, this looks more like chronic drift being narrated around than genuinely contained.
