# ADR 0010: Deprecate dispatch-docs documentation agent path

## Status

Accepted (cycle 237, 2026-03-12)

## Context

The phased cycle architecture (introduced cycle 221) used a two-path documentation model:

1. **Primary**: `dispatch-docs` — dispatch a Copilot agent to generate worklog/journal from committed state
2. **Fallback**: `write-entry` — orchestrator writes documentation directly using structured CLI input

The dispatch-docs path was designed to separate the "work" role from the "documentation" role, reducing self-reporting inaccuracies. However, it introduced structural problems:

- **Receipt timing trap**: Phase B/C commits create receipt gaps that the doc agent cannot resolve
- **0% success rate**: All four cycles since the last audit (233-236) used the write-entry fallback
- **Maintenance burden**: Three Rust crates (dispatch-docs, check-doc-pr, cycle-phase doc phases), shell wrappers, Phase B checklist steps, and review_iteration tracking all maintained dead infrastructure
- **False framing**: The "fallback" label obscured that dispatch-docs was never successfully used in recent cycles

Meanwhile, `write-entry` was hardened (PR #1111, cycle 236) to validate receipts against canonical `cycle-receipts` output and auto-derive self-modifications from git history. The hardened path has not yet been fully validated in production — cycle 237 (the first cycle after hardening) still produced a worklog that failed `validate-docs` due to receipt gaps from late commits. The strategic deprecation decision rests on write-entry being the *improvable* path, not on it being proven.

## Decision

Deprecate `dispatch-docs` (Option A from audit #216). Remove the following:

- `dispatch-docs` and `check-doc-pr` Rust crates and shell wrappers
- `doc_dispatched` and `doc_review` cycle phases
- Phase B documentation review flow from COMPLETION_CHECKLIST.md
- `review_iteration`, `review_max`, `doc_pr` from cycle_phase state

`write-entry` becomes the sole documentation path. The cycle state machine simplifies to: `work` -> `close_out` -> `complete`.

## Consequences

- Tool count reduced by 2-3 (dispatch-docs, check-doc-pr, cycle-phase simplification)
- COMPLETION_CHECKLIST.md reduced by ~113 lines
- Eliminated a class of failure modes (Phase B resume logic, doc agent timeouts, receipt timing traps)
- Documentation accuracy now depends entirely on write-entry's validation — the receipt hardening from PR #1111 is the intended quality gate, pending production validation
- No more multi-session cycle completion — cycles complete in a single session

## Alternatives Considered

**Option B — Fix dispatch-docs**: Diagnose the receipt timing trap and make dispatch-docs work reliably. Rejected because: (a) write-entry with hardened receipts already achieves the quality goal, (b) the additional complexity of agent-generated docs was not producing better output, (c) 0% success over 4 cycles suggests fundamental design issues rather than fixable bugs.
