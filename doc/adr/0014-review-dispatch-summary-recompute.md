# ADR 0014: Review Dispatch Recomputes the Sealed Cycle Summary

Date: 2026-04-22

## Status

Accepted

## Decision

When `record-dispatch` records a review-agent dispatch for the cycle that has just been sealed, it MUST recompute `last_cycle.summary` from the post-dispatch state instead of preserving the pre-dispatch frozen summary verbatim. This keeps `last_cycle.summary`, `dispatch_log_latest`, the post-dispatch worklog delta, and `state-invariants` aligned on the same live dispatch count even when the cycle has already transitioned to `close_out` or `complete`; only the sealed `cycle_phase.completed_at` snapshot remains frozen.
