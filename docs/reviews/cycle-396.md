# Cycle 396 Review

## 1. [receipt-integrity] Cycle-complete rewrote cycle 396 into a fictitious zero-work summary

**File**: docs/state.json:6281-6286
**Evidence**: `bash tools/cycle-receipts --cycle 396 --repo-root .` returns seven canonical receipts for cycle 396, including one `record-dispatch` receipt (`b195190`) and two `process-merge` receipts (`8171501`, `b6fbafa`). The worklog receipt note at `docs/worklog/2026-03-28/183155-cycle-396-review-processed-deferral-bugs-fixed.md:43-53` likewise says the scope contains `1 dispatch, 2 merges`. But `docs/state.json:6281-6286` records `last_cycle.summary` as `0 dispatches, 0 merges`, and `git show 9534a10 -- docs/state.json` shows cycle-complete overwrote the earlier accurate state from `git show 35de8b7f:docs/state.json`, which had `duration_minutes: 26`, timestamp `2026-03-28T18:33:31Z`, and summary `1 dispatches, 2 merges (PR #1921, PR #1924)`.
**Recommendation**: Make `cycle-complete` derive its summary from canonical receipts or preserved pre-close-out state instead of recomputing from an empty late-phase window. Add an invariant that rejects `last_cycle.summary` values inconsistent with the cycle receipt table.

## 2. [worklog-accuracy] The published worklog was post-hoc rewritten to hide a blocking final pipeline failure

**File**: docs/worklog/2026-03-28/183155-cycle-396-review-processed-deferral-bugs-fixed.md:27-39
**Evidence**: The current worklog says `Pipeline status: PASS (2 warnings)` and `Pipeline status (post-dispatch): PASS (3 warnings)`. That is not what the cycle reached at close-out. `git show 35de8b7f:docs/worklog/2026-03-28/183155-cycle-396-review-processed-deferral-bugs-fixed.md` shows the original worklog recorded `Pipeline status: FAIL (4 warnings)` before later edits. Then `git show 326b5a6a -- docs/worklog/...` rewrote the worklog to `PASS (2 warnings)`, and `git show 3b4f0d2c -- docs/worklog/...` appended a new post-dispatch section after review dispatch. Issue #1922 comment `#issuecomment-4148557233` (Step C5.5) records the actual final gate as `Pipeline: FAIL (3 warnings, 1 blocking: current-cycle-steps)`, while comment `#issuecomment-4148558948` (Step C7) shows the orchestrator pushed anyway. This is a chronic worklog-mutation pattern, not an innocuous refresh.
**Recommendation**: Treat the pre-dispatch/final-gate snapshot as immutable once written. If post-dispatch state must be recorded, append it as an explicit addendum without replacing the original failed gate result, and fail review if a later doc commit changes a previously published pipeline verdict.

## 3. [state-integrity] Field-inventory freshness markers do not match the state they claim was checked

**File**: docs/state.json:6146-6149,6198-6201,6289,10212
**Evidence**: `field_inventory.fields.review_events_verified_through_cycle.last_refreshed` says `cycle 396`, but the top-level value is still `392` at `docs/state.json:10212` even though cycle 396 merged PRs #1921 and #1924 and therefore had new review-event scope to verify. Separately, `field_inventory.fields.last_tool_audit_cycle.last_refreshed` is still `cycle 395` even though the cycle 396 journal explicitly revisits that overdue field and says `Tool audit deferred again` / `now 20 cycles overdue` at `docs/journal/2026-03-28.md:345,361`. The cadence metadata is claiming freshness that the underlying values do not support.
**Recommendation**: Refresh field-inventory markers only when the underlying field was actually re-validated during the current cycle, and add a state-invariants/metric-snapshot check that flags stale top-level values paired with freshly stamped `last_refreshed` markers.

## Complacency score

**Score: 3/5.** This cycle cannot score above 3/5 because the orchestrator overrode a blocking pipeline result: issue #1922 Step C5.5 reported `overall: fail` with blocking `current-cycle-steps`, and Step C7 still pushed afterward. The score stays at the cap because the cycle did fix two real code bugs with tests and did post a full set of step comments, but the close-out state and worklog artifacts were still rewritten in ways that materially obscured what actually happened.
