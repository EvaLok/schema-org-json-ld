# ADR 0012: Dispatch-Review Enforcement Gap Fix

Date: 2026-03-19

## Status

Accepted

## Context

An adversarial review of the pipeline stabilization program (ADR 0011) identified two high-severity findings:

### Finding 1: `dispatch-review` bypasses pipeline gate enforcement

ADR 0011 Phase 1 hardened `record-dispatch` with two enforcement mechanisms:
- `enforce_pipeline_gate()` — refuses dispatch when `pipeline-check` fails (logs warning for review dispatches)
- `update_review_dispatch_tracking()` — tracks consecutive review-only dispatches and warns at 3+

Both functions were implemented in the `record-dispatch` library crate (`lib.rs`). However, `dispatch-review` — which calls `build_dispatch_patch()` and `apply_dispatch_patch()` from the same library — never called the enforcement functions. When ADR 0011 Phase 2b introduced `cycle-runner`, which routes review dispatches through `dispatch-review` instead of `record-dispatch`, the enforcement layer was silently bypassed for all cycles from 302 onward.

The gap is architectural: the enforcement was correctly placed in the shared library, but `dispatch-review` was written to use only the state-mutation functions, not the enforcement functions. This was not detected because `cycle-runner` independently runs `pipeline-check` at step C5.5 as a hard gate before invoking `dispatch-review` at C6 — so the gate was effectively enforced, but at the wrong layer (orchestration rather than tool).

### Finding 2: Stabilization counter retroactive exemption

Between cycles 301 and 302, three tool changes landed on master:
1. A pipeline fix whose own commit message stated "Counter resets to 0" — the reset never happened
2. The ADR 0011 amendment reducing the burn-in target from 50 to 12 and adding the Eva-authorized exemption clause — written 11 minutes after the pipeline fix
3. The `cycle-runner` crate (Phase 2b) — ADR 0011 itself states "Counter resets to 0 per stabilization rules (tool crate added)"

The Eva exemption clause was retroactively applied to the pipeline fix that predated it. The counter is arithmetically correct (6 genuinely clean cycles, each diff increments by exactly 1), but the exemption's temporal ordering undermines its procedural legitimacy.

## Decision

### Fix the enforcement gap

Wire `enforce_pipeline_gate()` and `update_review_dispatch_tracking()` into `dispatch-review`'s `record_created_issue()` function. Both functions are already public in the `record-dispatch` library crate — this is a two-line addition plus an import change.

The pipeline gate call uses `review_dispatch: true`, which logs a warning rather than blocking (review dispatches are exempt from the hard gate by design, but the call ensures the code path is exercised and the tracking counter increments).

### Reset the stabilization counter (split the difference)

Rather than continuing with a counter whose procedural legitimacy is questionable, or resetting and losing 6 cycles of genuine evidence:

- Reset `clean_cycle_counter` to 0
- Reduce the burn-in target from 12 to 6
- Clear `consecutive_clean_cycles`
- The 6 proven clean cycles validated the pre-fix enforcement model; the remaining 6 cycles validate the post-fix model with the enforcement gap closed

This means the stabilization program requires 6 more clean cycles to complete, starting from the cycle after this fix lands.

## Consequences

### Positive
- `dispatch-review` now participates in the same enforcement layer as `record-dispatch`
- The `review_dispatch_consecutive` warning counter is no longer dead for cycle-runner cycles
- The stabilization counter restart eliminates the retroactive-exemption ambiguity
- Future dispatch tools that use the `record-dispatch` library get enforcement automatically if they follow the same pattern

### Negative
- Stabilization completion is delayed by approximately 3 days (6 cycles at ~2 cycles/day)
- This fix is itself a tool change during stabilization, creating the same tension ADR 0011 identified

### Trade-offs
- The reduced target (6 instead of 12) reflects that the first 6 clean cycles already provided evidence of stability under the pre-fix model — repeating all 12 would be unnecessarily punitive
- Eva pushes this fix directly (not orchestrator-dispatched), consistent with the operator intervention pattern established in ADR 0011
