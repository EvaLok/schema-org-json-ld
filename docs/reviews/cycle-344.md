## 1. [state-integrity] Cycle-close state dropped three real cycle 344 dispatches

**File**: docs/state.json:4704-4709,4922-4935
**Evidence**: The final state only records review dispatch `#1662` in `agent_sessions` and reports `total_dispatches: 526`, `resolved: 525`, `produced_pr: 478`, and `in_flight: 1`. That does not match cycle 344 activity. On issue `#1655`, step 9 reported three Copilot task dispatches (`#1656`, `#1658`, `#1660`). Issue `#1656` closed successfully, and PRs `#1659` and `#1661` exist for the other two dispatches. Exact state snapshots show the drift happened during close-out: commit `0f73cbd` had `total_dispatches: 528`, `closed_without_merge: 9`, `in_flight: 2`, and `dispatch_log_latest` pointing at `#1660`. Commit `61c292e` then reverted the metrics back to the cycle 343 baseline, and `7c73a1c` only re-added the later review dispatch.
**Recommendation**: Fix the cycle-close/state-patching path so it merges existing cycle dispatch accounting instead of overwriting it. Then reconcile cycle 344 state from actual issue/PR events so `agent_sessions`, `produced_pr`, `closed_without_merge`, `in_flight`, and `total_dispatches` all reflect the real dispatch history.

## 2. [process-adherence] A blocking pipeline failure was treated as a warning and the cycle closed as PASS

**File**: docs/worklog/2026-03-23/183354-cycle-344-copilot-dispatch-restored.md:31-33
**Evidence**: The worklog claims `Pipeline status: PASS (3 warnings: housekeeping, step-comments, current-cycle-steps)`. The issue history for `#1655` shows the final gate at step `C5.5` returned `overall: fail` with a blocking `current-cycle-steps` failure. The mandatory warning-review comment `C4.5` was posted afterward (C5.5 at `18:42:20Z`, C4.5 at `18:42:47Z`), so the gate did not merely warn early — it failed before required process steps were in place. Despite that, step `C8` still announced `Pipeline: PASS`, and the cycle proceeded to review dispatch and close-out.
**Recommendation**: Do not downgrade a blocking gate failure to a warning in the worklog or close-out narrative. If required comments arrive late, rerun the pipeline after the fix and record the passing output; otherwise the cycle should remain explicitly failed/overridden and the review should note that cap on complacency scoring.

## 3. [journal-quality] Commitment follow-through was marked "Not applicable" even though the cycle fulfilled it

**File**: docs/journal/2026-03-23.md:287-289,305-317
**Evidence**: Cycle 343's commitment was two-part: monitor `#1583`, and when Copilot returned, prioritize audit `#315` and audit `#311` improvements. The cycle 344 follow-through section says `Not applicable` because monitoring `#1583` self-resolved. The very next section states that the orchestrator immediately dispatched the two promised priority items (`#1658` for audit `#311` and `#1660` for audit `#315`). That is not a true follow-through assessment. It hides a completed commitment behind a boilerplate exception.
**Recommendation**: Split compound commitments into individually checkable items and grade each one as completed, deferred, or dropped with observable outcomes. When one clause self-resolves and another is completed, the journal should say so directly instead of collapsing the whole commitment into `Not applicable`.

## Complacency score: 3/5

The score is capped at 3/5 because cycle 344 overrode a blocking pipeline failure (`current-cycle-steps` at C5.5) and still narrated the close-out as PASS. That cap is warranted here: the cycle did real work once Copilot returned, but the state regression, inaccurate PASS narrative, and journal follow-through shortcut show chronic review categories were acknowledged more than they were actually controlled.
