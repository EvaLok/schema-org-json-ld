# Cycle 115 — 2026-03-03 19:43 UTC

## What happened

### Startup checklist
- 1 new Eva directive: [#396](https://github.com/EvaLok/schema-org-json-ld/issues/396) — acknowledged Node 22 addition to CI matrix, closed
- No new Eva comments since cycle 114
- No new QC reports
- Audit [#73](https://github.com/EvaLok/schema-org-json-ld-audit/issues/73) still open on audit repo (already processed cycle 114)
- 0 Copilot PRs to review
- 0 agent sessions in-flight at start

### Proactive improvement: PHPStan level max regression
During infrastructure quality scan, tested PHPStan at `level: max` and found 3 new errors introduced by [PR #364](https://github.com/EvaLok/schema-org-json-ld/issues/364) (mixed-array serialization fix, merged cycle 108). The state.json previously claimed 0 errors at level max — this was stale since PR #364.

Errors are all in `JsonLdGenerator.php` lines 120/125/127: `Cannot access an offset on mixed`. The `$obj` array is typed `array<string, mixed>`, so `$obj[$k][]` is appending to a `mixed` value. Fix: build into a local `$arr` variable and assign once.

Dispatched [#398](https://github.com/EvaLok/schema-org-json-ld/issues/398) to Copilot to fix the errors and upgrade `phpstan.neon` from `level: 9` to `level: max`.

### Infrastructure updates
- **AGENTS-ts.md**: Updated Node version reference from "20 (also tested on 24)" to "20 (also tested on 22 and 24)" to reflect Eva's commit e6d62e35
- Verified dual-language parity: 89/89 PHP/TS schema classes
- Health check: 423 PHP tests pass (1,947 assertions)

### Housekeeping
- No stale issues (4 open, all legitimate: #247, #303, #304, #329)
- No stale branches (only `copilot/add-npm-publish-workflow` for PR #305)
- No open audit-inbound or qc-inbound issues

## Self-modifications
- **AGENTS-ts.md**: Updated Node version reference to include 22

## Current state

- **Copilot sessions**: 1 in-flight (#398), 30/30 prior merged (100%)
- **Schema classes**: 89/89 PHP/TS parity (verified)
- **QC parity**: 73/73 standalone types (100%) — gate satisfied
- **Phase 4**: QC gate cleared, awaiting Eva (PR #305 + NPM_TOKEN)
- **PHPStan**: Level 9 (fix for level max dispatched as #398)

## Open issues/PRs

| # | Type | Status |
|---|---|---|
| [#247](https://github.com/EvaLok/schema-org-json-ld/issues/247) | input-from-eva | Open (will close after npm publish) |
| [#304](https://github.com/EvaLok/schema-org-json-ld/issues/304) | question-for-eva | Open (NPM_TOKEN needed) |
| [#305](https://github.com/EvaLok/schema-org-json-ld/issues/305) | PR | Open (workflow file — Eva must merge) |
| [#329](https://github.com/EvaLok/schema-org-json-ld/issues/329) | input-from-eva | Open (QC validation fulfilled, awaiting Eva) |
| [#398](https://github.com/EvaLok/schema-org-json-ld/issues/398) | agent-task | Dispatched (PHPStan level max fix) |

## Next steps

1. Review PR from #398 when Copilot finishes
2. When Eva merges PR #305 and configures NPM_TOKEN, execute Phase 4c (publish)
3. Continue monitoring for audit recommendations
