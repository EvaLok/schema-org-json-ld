# Cycle 354 Review

## 1. [state-integrity] The cycle marked the `in_flight_sessions` fix as actioned even though state drift reappeared before the review dispatch

**File**: docs/state.json:5286-5289,5436,8526
**Evidence**: `field_inventory.fields.in_flight_sessions.last_refreshed` is recorded as `cycle 354`, the latest review note says `F3 state-integrity: actioned (in_flight_sessions fixed via derive-metrics)`, but top-level `in_flight_sessions` is still `0` while the latest `agent_sessions` entry is issue `#1724` with `status: "in_flight"`. The repository's own checker confirms the mismatch: `bash tools/state-invariants --json --repo-root .` currently fails `in_flight_sessions_consistency` with `in_flight_sessions expected 1 from agent_sessions but actual 0`.
**Recommendation**: Treat `in_flight_sessions` as derived state on every dispatch as well as every merge, and do not mark the previous state-integrity finding as actioned until the post-dispatch path is covered by the same source of truth and verified by `state-invariants`.

## 2. [worklog-accuracy] The corrected worklog still mixes post-dispatch counters with stale next steps and an inaccurate gate result

**File**: docs/worklog/2026-03-25/045999-cycle-354-merge-review-findings-merge-pr-validation.md:31-40
**Evidence**: The published "Current state" block reports `In-flight agent sessions: 1` and `550 dispatches`, which only became true after `state(record-dispatch): #1724 dispatched [cycle 354]`, but the "Next steps" block still says `Dispatch cycle-end review`. The same section also claims `Pipeline status: PASS (17/17 invariants, all metric checks pass)`, yet `bash tools/state-invariants --json --repo-root .` reports `passed: 16`, `failed: 1`, with the failing check named `in_flight_sessions_consistency`.
**Recommendation**: Publish the worklog from one consistent snapshot only. If post-dispatch numbers are patched in, recompute the next-steps block and rerun the gate summary in the same patch instead of editing only selected counters.

## 3. [process-adherence] The cycle declared `merge-pr` production-ready even though the merge landed with zero CI checks

**File**: docs/journal/2026-03-25.md:106
**Evidence**: The journal explicitly says `Merged merge-pr tool without CI checks (draft PR, no checks ran). Validated by building locally and running dry-run.` GitHub PR metadata confirms the gap: `pull_request_read(get_check_runs)` for PR `#1720` returned `total_count: 0`, while the PR itself merged `693` additions across `4` files. Despite that, the worklog states the tool was `ready for production use`.
**Recommendation**: Do not label new orchestration tools "ready for production use" unless their PR carries either successful CI evidence or a documented, explicit exception with equivalent local test transcripts attached to the durable record.

## Complacency score

**2/5.** The cycle did keep a traceable operational trail — issue `#1723` has 26 step comments, and the canonical receipt hashes for cycle 354 resolve cleanly once full history is fetched — but the substance is still complacent. A chronic state-integrity finding was marked actioned before the dispatch path was actually fixed, the corrected worklog still published contradictory state, and a 693-line automation tool was merged with zero check runs while being described as production-ready. Because the cycle overrode a failing `state-invariants` gate, the score is capped at 3/5; the repeated drift and overclaiming justify scoring below that cap.
