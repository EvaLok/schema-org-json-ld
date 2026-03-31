## 1. [worklog-accuracy] Post-dispatch worklog refresh leaves the published worklog invalid against current state

**File**: docs/worklog/2026-03-31/103836-cycle-422-processed-review-merged-2-prs-dispatched-record-dispatch-and-validate-docs-fixes.md:33
**Evidence**: The refreshed worklog still reports the primary `In-flight agent sessions` value as `2`, while current `docs/state.json` reports `in_flight_sessions: 3` at `docs/state.json:6993`. The post-dispatch refresh commit `36933d2` only appended a secondary `In-flight agent sessions (post-dispatch): 3` line instead of reconciling the field the validator reads. A fresh `bash tools/pipeline-check` now fails `doc-validation` with `worklog validation failed: in-flight agent sessions mismatch: worklog reports 2, state.json has 3`.
**Recommendation**: Make the post-dispatch refresh path update the canonical in-flight count that doc-validation checks, or teach doc-validation to use the explicit post-dispatch field when present. Do not leave the published worklog in a state that immediately fails repository validation after C6/C6.5.

## 2. [state-integrity] `last_cycle` is still being rewritten after the frozen cycle-complete snapshot

**File**: docs/state.json:6994
**Evidence**: The `cycle-complete` commit `f2b041d` froze cycle 422 as `in_flight_sessions: 2` and `last_cycle.summary: "2 dispatches, 2 merges (PR #2082, PR #2084)"` at timestamp `2026-03-31T10:37:27Z`. After the review dispatch, commit `ba87ed2` rewrote the same frozen snapshot to `in_flight_sessions: 3` and `last_cycle.summary: "3 dispatches, 2 merges (PR #2082, PR #2084)"` without changing the timestamp; the current file still shows that mutated summary at `docs/state.json:6998-6999`. This is the same frozen-timestamp drift pattern the prior cycle review flagged, so the deferred state-integrity category was acknowledged but not actually corrected.
**Recommendation**: Freeze `last_cycle` at the true `cycle-complete` state and record later review-dispatch effects in separate live-state fields. If post-close activity must be summarized, add a distinct post-close summary/timestamp instead of mutating the frozen cycle snapshot in place.

## 3. [journal-quality] The new observability rule was violated immediately by a backwards commitment

**File**: docs/journal/2026-03-31.md:226
**Evidence**: Cycle 422 added a new C3 checklist constraint requiring journal commitments to name an exact check and expected outcome. But the very next journal commitment says: `Verify C3 pipeline-status-preliminary constraint: this worklog says preliminary. Observable: grep preliminary on worklog returns non-zero.` The referenced worklog does contain the word `preliminary` at `docs/worklog/2026-03-31/103836-cycle-422-processed-review-merged-2-prs-dispatched-record-dispatch-and-validate-docs-fixes.md:35`, so `grep preliminary` should return zero on success, not non-zero. The observable is inverted and therefore not gradable as written.
**Recommendation**: Write commitments with the exact command and the correct expected result semantics, e.g. `grep -q preliminary <worklog>` exits 0. Before publishing a journal entry, sanity-check that each stated observable actually matches the artifact it references.

## Complacency score

**2/5** — The cycle did real work and did not override a blocking final gate: C5.5 passed before review dispatch, receipt hashes resolve, `state-invariants` and `metric-snapshot` pass, and issue `#2085` has 27 distinct step comments covering all mandatory pre-gate steps. But the cycle still normalized two chronic review categories without actually closing them: the post-dispatch refresh immediately re-broke doc validation, `last_cycle` still drifts after the frozen timestamp, and the newly added journal observability rule was violated in the very next commitment.
