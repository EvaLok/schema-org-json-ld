# Cycle 328 — 2026-03-21 14:22 UTC

## What was done

- Fixed validate-docs stdout bug: errors were written to stderr only, preventing pipeline-check cascade detection from working
- Reconciled state: marked [#1578](https://github.com/EvaLok/schema-org-json-ld/issues/1578) as failed (8th consecutive Copilot failure), removed duplicate worklog, fixed journal worklog link
- Closed stale audit-inbound [#1577](https://github.com/EvaLok/schema-org-json-ld/issues/1577)
- Probed Copilot ([#1580](https://github.com/EvaLok/schema-org-json-ld/issues/1580)): 9th consecutive ruleset violation failure
- Updated escalation [#1567](https://github.com/EvaLok/schema-org-json-ld/issues/1567) with failure counts

### PRs merged

- None.

### Issues processed

- [#1578](https://github.com/EvaLok/schema-org-json-ld/issues/1578) (cycle-review, failed - Copilot ruleset violation)
- [#1577](https://github.com/EvaLok/schema-org-json-ld/issues/1577) (audit-inbound, closed - action complete)
- [#1580](https://github.com/EvaLok/schema-org-json-ld/issues/1580) (probe, failed - Copilot ruleset violation)

## Self-modifications

- **`tools/rust/crates/validate-docs/src/main.rs`**: Changed error output from stderr to stdout for pipeline-check cascade detection

## Current state

- **In-flight agent sessions**: 0
- **Pipeline status**: PASS
- **Copilot metrics**: 491 dispatches, 478 PRs, 468 merged, 97.9% merge rate
- **Publish gate**: published

## Next steps

1. Probe Copilot availability each cycle until resolved
2. If Copilot available, dispatch review or schema work
3. If still failing, continue maintenance with self-review fallback

## Commit receipts

> Note: Scope: cycle 328 commits through cycle-complete — mode normal; phase close_out; receipt events: 4 reviews. Receipt table covers commits through cycle-complete (C5.1 snapshot). Post-C5.1 commits (docs, record-dispatch, review-body) are structurally excluded.

| Tool | Receipt | Link |
|------|---------|------|
| cycle-start | f4b3966 | [f4b3966](https://github.com/EvaLok/schema-org-json-ld/commit/f4b3966) |
| reconcile | 23648be | [23648be](https://github.com/EvaLok/schema-org-json-ld/commit/23648be) |
| process-audit | 3100c85 | [3100c85](https://github.com/EvaLok/schema-org-json-ld/commit/3100c85) |
| derive-metrics | 9b954c8 | [9b954c8](https://github.com/EvaLok/schema-org-json-ld/commit/9b954c8) |
| cycle-complete | 50e553f | [50e553f](https://github.com/EvaLok/schema-org-json-ld/commit/50e553f) |
| verify-review-events | a33f158 | [a33f158](https://github.com/EvaLok/schema-org-json-ld/commit/a33f158) |
| cycle-tagged | 0debc64 | [0debc64](https://github.com/EvaLok/schema-org-json-ld/commit/0debc64) |
| cycle-tagged | 91ea750 | [91ea750](https://github.com/EvaLok/schema-org-json-ld/commit/91ea750) |
| cycle-tagged | a1d2c02 | [a1d2c02](https://github.com/EvaLok/schema-org-json-ld/commit/a1d2c02) |
| cycle-tagged | 578e19b | [578e19b](https://github.com/EvaLok/schema-org-json-ld/commit/578e19b) |
| cycle-tagged | 4f34d1b | [4f34d1b](https://github.com/EvaLok/schema-org-json-ld/commit/4f34d1b) |
| cycle-tagged | b76c17b | [b76c17b](https://github.com/EvaLok/schema-org-json-ld/commit/b76c17b) |
| validate-docs-fix | 2bc396b | [2bc396b](https://github.com/EvaLok/schema-org-json-ld/commit/2bc396b) |
| probe-failed | 45124cb | [45124cb](https://github.com/EvaLok/schema-org-json-ld/commit/45124cb) |
| record-dispatch | 6c691d5 | [6c691d5](https://github.com/EvaLok/schema-org-json-ld/commit/6c691d5) |
