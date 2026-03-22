# Cycle 336 — 2026-03-22 18:18 UTC

## What was done

- Fixed cycle_phase stuck at close_out from cycle 335 (manual edit before cycle-start)
- Refreshed 2 stale field inventory entries (test_count gap 6, tool_pipeline gap 11)
- Dispatched Copilot probe [#1615](https://github.com/EvaLok/schema-org-json-ld/issues/1615) — 24th consecutive failure (repository ruleset violation)
- Investigated cycle_phase bug: cycle-runner close-out never transitions to complete after C8
- Updated escalation issue [#1567](https://github.com/EvaLok/schema-org-json-ld/issues/1567) with cycle 336 status
- Updated copilot_metrics: closed_without_pr 30->31, resolved 508->509

### PRs merged

- None.

### Issues processed

- [#1615](https://github.com/EvaLok/schema-org-json-ld/issues/1615): Copilot probe 24th consecutive — failed (ruleset violation), closed

## Self-modifications

- **`docs/state.json`**: Fixed cycle_phase, refreshed field inventory, probe failure recorded

## Current state

- **In-flight agent sessions**: 0
- **Pipeline status**: PASS — all blocking checks green (2 non-blocking warnings: housekeeping 1 stale audit-inbound, step-comments)
- **Copilot metrics**: 509 dispatches, 478 PRs, 468 merged, 97.9% merge rate
- **Publish gate**: published v1.0.2

## Next steps

1. Continue probing Copilot each cycle until resolved
2. When Copilot returns: fix cycle_phase bug in close_out.rs (add transition to complete after C8)
3. When Copilot returns: dispatch [audit #311](https://github.com/EvaLok/schema-org-json-ld-audit/issues/311) tool fixes and pending schema work
4. Monitor Eva response on [#1567](https://github.com/EvaLok/schema-org-json-ld/issues/1567) and [#1583](https://github.com/EvaLok/schema-org-json-ld/issues/1583)

## Commit receipts

> Note: Scope: cycle 336 commits through cycle-complete — mode normal; phase close_out. Receipt table covers commits through cycle-complete (C5.1 snapshot). Post-C5.1 commits (docs, record-dispatch, review-body) are structurally excluded.

| Tool | Receipt | Link |
|------|---------|------|
| cycle-start | 3591786 | [3591786](https://github.com/EvaLok/schema-org-json-ld/commit/3591786) |
| cycle-336 | 8a32c16 | [8a32c16](https://github.com/EvaLok/schema-org-json-ld/commit/8a32c16) |
| probe-failed | beb3d8b | [beb3d8b](https://github.com/EvaLok/schema-org-json-ld/commit/beb3d8b) |
| cycle-complete | 12713c1 | [12713c1](https://github.com/EvaLok/schema-org-json-ld/commit/12713c1) |
| field-inventory | 8a32c16 | [8a32c16](https://github.com/EvaLok/schema-org-json-ld/commit/8a32c16) |
| record-dispatch | 25a760e | [25a760e](https://github.com/EvaLok/schema-org-json-ld/commit/25a760e) |
| metrics-fix | 318bd9d | [318bd9d](https://github.com/EvaLok/schema-org-json-ld/commit/318bd9d) |
| state-invariants | fa09f09 | [fa09f09](https://github.com/EvaLok/schema-org-json-ld/commit/fa09f09) |
