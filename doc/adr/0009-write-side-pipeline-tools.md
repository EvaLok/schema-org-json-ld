# 9. Write-Side Pipeline Tools for State Management

Date: 2026-03-06

## Status

Accepted

## Context

The orchestrator's `docs/state.json` file (~800 lines, ~38KB) is the central state store for cycle tracking, metrics, field inventory, agent sessions, and publish gates. Initially, the orchestrator read the entire file into context, modified it manually, and wrote it back. This was:

1. **Error-prone**: Manual edits frequently introduced inconsistencies (metrics wrong by 147%, freshness markers stale for 7-16 cycles)
2. **Context-wasteful**: Reading 800 lines of JSON displaced reasoning capacity
3. **Non-atomic**: Multiple fields needed updating together, but manual edits could partially complete

The read-side pipeline (pipeline-check, metric-snapshot, etc.) could verify state but not fix it. Corrections required manual editing.

## Decision

Build write-side Rust tools that atomically update owned sections of state.json:

| Tool | Owns | Trigger |
|------|------|---------|
| `cycle-complete` | `last_cycle.*`, freshness markers | End of cycle |
| `record-dispatch` | `copilot_metrics`, `agent_sessions` | Agent dispatch |
| `process-merge` | `copilot_metrics.merged`, session status | PR merge |
| `process-review` | `review_agent.*` | Review consumption |
| `process-audit` | `audit_processed` | Audit processing |
| `process-eva` | `eva_directives` | Eva directive handling |

Each tool:
- Reads only the fields it needs from state.json
- Validates inputs before writing
- Commits the change with a receipt hash
- Bumps its owned freshness markers automatically

The orchestrator never directly edits state.json — it calls tools instead.

## Consequences

- **Positive**: Eliminates manual state.json editing errors entirely
- **Positive**: Each tool owns its freshness markers — no more stale fields
- **Positive**: Receipt hashes enable audit verification of state changes
- **Positive**: Tools compose cleanly — `cycle-complete` runs after all event-driven tools
- **Negative**: Adding a new state field requires updating the relevant tool(s)
- **Trade-off**: More tools to maintain, but the error rate dropped from frequent to zero

## Alternatives Considered

1. **Manual editing with validation**: Keep manual edits but add a validator. Still error-prone at write time.
2. **Single monolithic state tool**: One tool that handles all state updates. Too complex and couples unrelated concerns.
3. **Database instead of JSON**: Use SQLite or similar. Overkill for the current scale and harder to version-control.
