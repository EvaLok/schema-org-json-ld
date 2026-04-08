# Cycle 462 Review

## 1. [state-integrity] Cycle 462 said the cycle 461 review was processed, but left review issue #2312 marked `in_flight`

**File**: `docs/worklog/2026-04-08/233759-cycle-462-review-processed-pr-2311-merged-with-rebase-combined-pipeline-check-hardening-dispatched-audits-392-393-accepted.md:5`, `docs/state.json:6956-6961`
**Evidence**: The worklog says cycle 461 review processing was complete and that PR `#2313` was admin-merged. But the canonical `agent_sessions` ledger still keeps issue `#2312` (`[Cycle Review] Cycle 461 end-of-cycle review`) in `status: "in_flight"` with no merge/close metadata. GitHub issue `#2312` itself was already closed at `2026-04-08T23:21:07Z`, so cycle 462 closed while its own state ledger still contradicted reality.
**Recommendation**: Make the review-processing/close-out path reconcile the originating review issue row when the review PR merges or the review issue closes. If that cannot be done in the same step, add a mandatory cleanup tool/check so closed review issues cannot remain `in_flight`.

## 2. [worklog-accuracy] The published worklog reports one in-flight session even though the ledger had three

**File**: `docs/worklog/2026-04-08/233759-cycle-462-review-processed-pr-2311-merged-with-rebase-combined-pipeline-check-hardening-dispatched-audits-392-393-accepted.md:39-45`, `docs/state.json:6921-6925`, `docs/state.json:6956-6969`
**Evidence**: The worklog's cycle-state section says `In-flight agent sessions: 1`. The same worklog's next-step text immediately acknowledges a stale `#2312` in-flight row still present in state, and the state ledger also retained `#2301` as `in_flight` alongside the newly dispatched `#2317`. At the docs commit, the canonical count was therefore three (`#2301`, `#2312`, `#2317`), not one. This was not a harmless race: the worklog published a hand-curated number instead of the ledger-backed count.
**Recommendation**: Derive the worklog's in-flight count mechanically from `docs/state.json` at publication time, even when some rows are known-stale. If the cycle wants to distinguish “live” from “stale,” publish both numbers explicitly instead of replacing the canonical count.

## 3. [journal-quality] The journal dropped commitment #2301 by calling `--body-file` “equivalent,” but the required fix is still absent

**File**: `docs/journal/2026-04-08.md:320-321`, `tools/rust/crates/post-step/src/main.rs:17-43`, `tools/rust/crates/post-step/src/main.rs:108-120`
**Evidence**: The journal says the post-step re-dispatch can be dropped because existing `--body-file` support is “the equivalent.” But `post-step` still only accepts `--body` or `--body-file`; there is no `--body-stdin`, no `--allow-template-syntax`, and no validation for literal `$(...)`/`${...}` patterns in `resolve_body`. Issue `#2301` required those exact features plus tests, so the journal converted an unmet commitment into a narrative exemption rather than a verified completion or a real deferment.
**Recommendation**: Do not mark this commitment dropped. Re-open or re-dispatch the actual `post-step` hardening work, and require journal follow-through sections to verify the concrete acceptance criteria of the original issue before declaring an “equivalent” solution.

## 4. [worklog-accuracy] The worklog backdates a later C5.5 rerun into a 23:37 close-out snapshot

**File**: `docs/worklog/2026-04-08/233759-cycle-462-review-processed-pr-2311-merged-with-rebase-combined-pipeline-check-hardening-dispatched-audits-392-393-accepted.md:1`, `docs/worklog/2026-04-08/233759-cycle-462-review-processed-pr-2311-merged-with-rebase-combined-pipeline-check-hardening-dispatched-audits-392-393-accepted.md:40`, `docs/worklog/2026-04-08/233759-cycle-462-review-processed-pr-2311-merged-with-rebase-combined-pipeline-check-hardening-dispatched-audits-392-393-accepted.md:51`
**Evidence**: The artifact is titled as a `23:37 UTC` cycle worklog and its receipt note says the scope is only through `2026-04-08T23:37:04Z (cycle-complete)`. Yet the published pipeline line says `PASS (1 blocking warning, 2 warnings)`, which was only recorded after the later C5.5 rerun (`issuecomment-4210461088` at `23:46:50Z`) and the subsequent `state(pipeline): record C5.5 PASS for cycle 462` commit `2750b184` at `23:46:51Z`. The worklog therefore mixes post-close-out pipeline state into a supposedly 23:37-bounded snapshot without saying so.
**Recommendation**: When a worklog includes post-cycle-complete reruns or state commits, either update the worklog timestamp/scope to the actual publication state or keep the cycle-state section frozen to the cycle-complete snapshot and record later reruns in an explicit addendum.

## Complacency score

**2/5** — Cycle 462 did real investigation work: it left a detailed 29-step comment trail on issue `#2314`, ran the relevant tooling, and captured a meaningful journal entry instead of boilerplate. But it still published a worklog with a non-canonical in-flight count, left the just-processed review issue `#2312` stuck `in_flight`, backdated a later C5.5 rerun into the close-out snapshot, and talked itself out of the unresolved `post-step` commitment with an unsupported “equivalent” claim. Because the cycle overrode an initial pipeline failure before rerunning, the score is capped below 4/5 anyway; the evidence supports staying at **2/5**.
