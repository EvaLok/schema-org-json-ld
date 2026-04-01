## 1. [worklog-accuracy] Post-dispatch worklog refresh mixed live and stale reality

**File**: docs/worklog/2026-04-01/125457-cycle-429-closed-stale-copilot-dispatches-re-dispatched-c5-5-gate-fix-housekeeping.md:24-49
**Evidence**: The post-dispatch refresh added `#2128` as an in-flight review issue at lines 24-26, but the same file still says the "actual" in-flight set is only `#2126` at line 36 and tells the next cycle to wait on closed stale issues `#2123` and `#2124` at lines 47-48. GitHub issue metadata shows `#2123` and `#2124` were already closed at 12:19Z, `#2126` was open from 12:23Z, and `#2128` was created at 13:00Z. The frozen refresh commit `0ff47fa` shows the post-dispatch sections were appended without reconciling the pre-dispatch note or next steps.
**Recommendation**: Regenerate all post-dispatch worklog sections from one post-dispatch snapshot. Once `#2128` is added, the in-flight note and next steps should reference the live open set (`#2126`, `#2128`) and must not keep closed stale issues as pending work.

## 2. [state-integrity] Successful re-dispatch `#2126` never made it into the state ledger

**File**: docs/state.json:6378-6397,7154-7160
**Evidence**: The active `agent_sessions` ledger contains stale closed issues `#2123` and `#2124` plus the later review dispatch `#2128`, but there is no entry for the re-dispatch `#2126`. A repository search for `2126` in `docs/state.json` returns no matches. That omission flows into `last_cycle.summary`, which still says `"0 dispatches, 0 merges"` even though both the worklog (`docs/worklog/...:6`) and journal (`docs/journal/2026-04-01.md:138-142`) say cycle 429 successfully re-dispatched the C5.5 fix as `#2126`, and GitHub issue `#2126` is open.
**Recommendation**: Make dispatch recording fail-closed: if a cycle creates or re-dispatches an agent task, `agent_sessions` and `last_cycle.summary` must be updated in the same cycle before close-out can pass. Add a reconciliation check against live issue state so a missing dispatch like `#2126` cannot be masked by internally self-consistent counters.

## 3. [field-inventory] `dispatch_log_latest` freshness marker contradicts the value it describes

**File**: docs/state.json:6881,6992-6995
**Evidence**: `dispatch_log_latest` now points to `#2128 [Cycle Review] Cycle 429 end-of-cycle review (cycle 429)`, proving the field changed this cycle. But the corresponding `field_inventory` entry still says `last_refreshed: "cycle 428"` even though its cadence is `"every dispatch or merge"`. This is a direct freshness-marker mismatch in the exact area the review brief asked to verify.
**Recommendation**: Refresh field-inventory markers whenever the underlying field changes, not just when cadence age exceeds a threshold. Add a targeted invariant that compares dispatch/merge-mutated fields such as `dispatch_log_latest` against their freshness markers.

## Complacency score

**2/5** — Cycle 429 did keep some fundamentals intact: `cycle-receipts` matches the worklog receipt table, `state-invariants`/`metric-snapshot` pass, and issue `#2125` has full step-comment coverage (27 step comments total; `pipeline-check` reports all mandatory current-cycle steps present). But the cycle still published a post-dispatch worklog with stale next steps, lost a real dispatch from the state ledger, and carried a freshness marker that contradicts the field it is supposed to certify. That is not clean execution; it is a cycle that passed its own gates while core bookkeeping remained unreliable.
