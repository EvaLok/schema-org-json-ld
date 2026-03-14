# Cycle 258 Review

## 1. [code-quality] PR #1230 still ignores the requested cycle when checking step comments

**File**: tools/rust/crates/pipeline-check/src/main.rs:181-192,745-806
**Evidence**: `main()` resolves `cli.cycle` and passes it into `run_pipeline`, but `verify_step_comments()` throws that away and calls `current_cycle_from_state(repo_root)` before `assess_step_comment_completeness()`. On the live repo, `bash tools/pipeline-check --cycle 254 --repo-root . --json` and the same command with `--cycle 258` both produced the identical step-comments result for issue `#1227` (`missing optional [1.1, 10]`), which shows the override path is not used for this check. The new PR #1230 tests only mutate `docs/state.json`; they never exercise the CLI override path that the issue required.
**Recommendation**: Thread the already-resolved `cycle` argument through `verify_step_comments()`, and add an end-to-end test that runs the pipeline with `--cycle` against a state file on a different current cycle.

## 2. [review-evidence] Cycle 258 recreated fake PR-review evidence immediately after merging the write-entry fix

**File**: docs/worklog/2026-03-14/162216-cycle-258-3-merges-audit-245-accepted-2-dispatches.md:8,22-25
**Evidence**: The worklog says cycle 257 review consumption had `1 actioned` finding and lists PRs `#1231` and `#1230` under `PRs reviewed`. But GitHub review metadata for both PRs is empty: `pull_request_read(..., method="get_reviews")`, `get_comments`, and `get_review_comments` all returned no review activity for PRs 1230 and 1231. The cycle 258 journal doubles down on the same claim (`docs/journal/2026-03-14.md:324-332`), saying the review-evidence category "should resolve now" and that `write-entry` "no longer creates phantom reviewed entries." The cycle issue's C1 comment (`https://github.com/EvaLok/schema-org-json-ld/issues/1234#issuecomment-4060817657`) says the early pipeline failure was fixed by reclassifying cycle 257 F3 to `actioned`, so the cycle turned a missing-evidence finding into a resolved finding without ever producing actual review evidence.
**Recommendation**: Keep `PRs reviewed` empty unless backed by GitHub review events, and do not mark review-evidence findings `actioned` until the published artifact is verified against actual review metadata.

## 3. [receipt-auditability] The review ledger repeated non-resolving receipt SHAs one cycle after the phantom-receipt finding

**File**: docs/state.json:5727
**Evidence**: The cycle 257 history note says `F3 review-evidence: actioned (PR #1231 merged, commit 0adbaf5)`, but `git rev-parse --verify --quiet 0adbaf5^{commit}` fails. The same pattern appears in cycle 258's Step 5 issue comment (`https://github.com/EvaLok/schema-org-json-ld/issues/1234#issuecomment-4060793904`), which cites `0adbaf5`, `fcf5293`, `1dc25a6`, and `f0e0e6a`; all four SHAs are non-resolving in this repository. Canonical cycle-258 receipts are instead `9957fcf`, `4c9bec2`, `9f0aaf9`, and `c17b3f3` per `bash tools/cycle-receipts --cycle 258 --repo-root .`. Cycle 257 review finding #1 was supposed to stop exactly this defect, but the cycle repeated it in both issue comments and review history.
**Recommendation**: Source review-history notes and step-comment receipts from canonical tool outputs (`cycle-receipts`, `process-review`, `process-merge`) and reject any SHA that does not resolve before it is written to `state.json` or posted to the cycle issue.

## 4. [process-adherence] The close-out comments were posted under the wrong checklist step IDs

**File**: COMPLETION_CHECKLIST.md:117-253
**Evidence**: The checklist defines C1 as the early pipeline check, C2 as state updates, C3 as documentation entries, C4.1 as documentation validation, C4.5 as ADR check, C5 as pre-dispatch commit/push, C5.1 as receipt validation, C5.5 as the final pipeline gate, C6 as review dispatch, C7 as dispatch-state push, and C8 as cycle close-out. Cycle 258's completion comments are shifted one step forward: comment `4060818173` labels `State updates (cycle-complete)` as `Step C1`, `4060821697` labels `Documentation entries` as `Step C2`, `4060821848` labels `Documentation validation` as `Step C3`, `4060822050` labels `ADR check` as `Step C4.1`, and the shift continues through `4060826004`, which labels cycle close-out as `Step C7`. The comments are individually posted, but the IDs no longer match the actions they certify, and `pipeline-check` still reported `missing mandatory [none]`, so the tooling accepted a semantically wrong completion trace.
**Recommendation**: Fix the orchestrator's `post-step` calls to use the checklist's actual step IDs, and harden `pipeline-check` so it validates expected titles/order for completion steps instead of only counting labels.

## Complacency score

**2/5** — Cycle 258 did merge real code and its published worklog receipt table passed scoped validation, so this was not pure theater. But the cycle still optimized for the appearance of closure over auditable proof: PR #1230's fix is incomplete, the worklog/journal recreated fake PR-review evidence, the review ledger repeated non-resolving SHAs immediately after a phantom-receipt finding, and the close-out comments used the wrong step IDs while the checker still passed them. That is a genuine improvement effort wrapped in too much evidence-shaped complacency.
