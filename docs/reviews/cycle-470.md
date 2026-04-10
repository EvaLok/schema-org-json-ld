# Cycle 470 Review

## 1. [code-change-quality] `verify-review-events` can still succeed without emitting the promised C4.7 evidence

**File**: tools/rust/crates/cycle-runner/src/close_out.rs:393-426
**Evidence**:
- The success branch parses `safe_to_advance_to` with `parse_verify_review_events_safe_to_advance_to(&stdout)?` before calling `steps::post_step(...)`.
- If `verify-review-events --apply` exits 0 after updating `state.json` but prints output that the parser cannot read, `step_c4_7_with_timeout()` returns early.
- In that path, no C4.7 step comment is posted.
- That matches cycle 470’s issue timeline: issue #2366 has Step C4.5 at `https://github.com/EvaLok/schema-org-json-ld/issues/2366#issuecomment-4220893963`, then jumps to Step C5.5 at `https://github.com/EvaLok/schema-org-json-ld/issues/2366#issuecomment-4220899489`.
- The C5.5 step-comment audit also lists 26 unique steps with no `C4.7`.
**Recommendation**: Post the C4.7 step comment even when parsing `safe_to_advance_to` fails, and downgrade the parse problem into comment content instead of an early return. Add a regression test for the exit-0/unparseable-output path.

## 2. [state-integrity] `eva_input_issues.closed_this_cycle` incorrectly carried `#2340` into cycle 470

**File**: docs/state.json:8005-8008
**Evidence**:
- `docs/state.json` records `closed_this_cycle` as `[2340]`.
- The published worklog repeats that as “[#2340]: Eva input closed this cycle” (`docs/worklog/2026-04-10/051603-cycle-470-review-processed-3-prs-merged-2-dispatches-deferral-drop-state-fix.md:21`).
- GitHub issue `#2340` actually closed at `2026-04-09T09:34:45Z`.
- Cycle 470 did not begin until `2026-04-10T04:58:22Z` (issue #2366 session-start comment).
- So this is not a same-cycle closure; it is stale state being re-presented as current-cycle work.
**Recommendation**: Recompute `eva_input_issues.closed_this_cycle` from the actual cycle-start boundary, or clear it explicitly at cycle start before repopulating from live issue metadata. Add an invariant that rejects `closed_this_cycle` entries whose GitHub `closed_at` precedes the current cycle start.

## 3. [journal-quality] The journal marks the `verify-review-events` commitment as MET without the promised observable

**File**: docs/journal/2026-04-10.md:55-70
**Evidence**: The cycle 470 journal says commitment 2 would be met if a “step comment documenting verify-review-events execution appears in orchestrator issue during close-out.” It then records that commitment as `MET` because “C4.7 step comment now uses structured format with outcome/state.json update fields.” The issue evidence does not support that statement. Cycle 470’s issue comment export has no C4.7 comment, and the final step-comment audit in `https://github.com/EvaLok/schema-org-json-ld/issues/2366#issuecomment-4220899489` lists steps `[0, 0.1, 0.5, 0.6, 1, 1.1, 2, 3, 4, 5, 6, 7, 8, 9, C1, C2, C3, C4.1, C4.5, C5, C5.1, C5.5, C5.6, C6, C7, C8]`—still no `C4.7`.
**Recommendation**: Treat commitment follow-through as evidence-backed, not inference-backed: if the observable is an issue comment, link or quote that exact comment before marking the commitment `MET`. Otherwise mark it partial/deferred and note what verification evidence was actually obtained.

## Complacency score

2/5 — the cycle did some real follow-up work, but it still published stale state as current-cycle activity, declared a commitment satisfied without the observable it promised, and merged a visibility fix whose own evidence path could still disappear in practice.
