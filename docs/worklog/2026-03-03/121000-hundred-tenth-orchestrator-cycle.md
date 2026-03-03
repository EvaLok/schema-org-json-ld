# Cycle 110 — 2026-03-03 12:10 UTC

## What happened

### Eva directive [#378](https://github.com/EvaLok/schema-org-json-ld/issues/378) processed
Eva's directive references the parity tool (#375, PR #377) and says: the instruction to use regex over AST is wrong. The project already uses AST analysis tools (PHPStan, TypeScript compiler). We should not reinvent the wheel. Eva also reminds that she's available to help with dependencies and workflows.

**Actions taken:**
1. Dispatched [#380](https://github.com/EvaLok/schema-org-json-ld/issues/380) to Copilot — rewrites `tools/parity-check.ts` using TypeScript Compiler API (already installed) for TS parsing and `php-parser` (new devDependency) for PHP AST parsing. Also fixes the known `FoodEstablishmentOptions extends LocalBusinessOptions` bug.
2. Created `.claude/skills/tool-creation-guidelines/SKILL.md` — captures "AST over regex" as a firm project convention for all future tool work.
3. Closed [#378](https://github.com/EvaLok/schema-org-json-ld/issues/378) with acknowledgment.

### PR [#381](https://github.com/EvaLok/schema-org-json-ld/issues/381) — merged
Copilot delivered the AST rewrite in 3 commits (~10 minutes). Review confirmed:
- All PHP parsing uses php-parser AST (zero regex for structure extraction)
- All TypeScript parsing uses TypeScript Compiler API (zero regex for structure extraction)
- FoodEstablishmentOptions extends bug fixed naturally by AST approach
- Output types, comparison logic, and report format all preserved
- Regression test added for the extends case
- CI passed (claude-review: SUCCESS). Merged at 12:31 UTC.

### Proactive improvement scan
- Dual-language check: 88/88 PHP = 88/88 TS. Perfect parity.
- Skill sizes audited: all under 222 lines, well within limits.
- AGENTS.md and AGENTS-ts.md verified current and accurate.
- Test coverage analysis: 7 types with 1 test (all building-block types with 1-3 properties — minimal coverage is adequate). Standalone types have 5-11 tests each.
- QC parity at 49/86 (57%), up from 39/86 in previous cycles. Rate ~10/cycle.

### Self-modifications
- **`.claude/skills/tool-creation-guidelines/SKILL.md`**: New skill — AST-over-regex convention, available parsers, when regex is acceptable
- **`.claude/skills/writing-skills/SKILL.md`**: Added tool-creation-guidelines to skill inventory table

## Current state

- **Copilot sessions**: 0 in-flight, 27/27 merged (100% success rate)
- **Schema classes**: 88/88 PHP/TS, 12/12 enums
- **QC parity**: 49/86 (57%)
- **Phase 4 blocked**: QC validation at 49/86. PR #305 waiting for Eva.

## Open issues/PRs

| # | Type | Status |
|---|---|---|
| [#247](https://github.com/EvaLok/schema-org-json-ld/issues/247) | input-from-eva | Open (will close after npm publish) |
| [#304](https://github.com/EvaLok/schema-org-json-ld/issues/304) | question-for-eva | Open (NPM_TOKEN — deprioritised) |
| [#305](https://github.com/EvaLok/schema-org-json-ld/issues/305) | PR | Open (workflow file — Eva must merge) |
| [#329](https://github.com/EvaLok/schema-org-json-ld/issues/329) | input-from-eva | Open (TS testing directive — pending QC validation) |
| [#331](https://github.com/EvaLok/schema-org-json-ld/issues/331) | qc-outbound | Open (comprehensive TS validation — QC at 49/86) |
| [#353](https://github.com/EvaLok/schema-org-json-ld/issues/353) | audit-inbound | Open (audit #58 — QC parity gap) |
| [#380](https://github.com/EvaLok/schema-org-json-ld/issues/380) | agent-task | Merged (PR #381, AST parity rewrite) |

## Next steps

1. Consider adding parity check to CI (requires workflow change — needs Eva)
2. Continue monitoring QC progress toward 86/86 parity
3. Look for next high-value tool or infrastructure improvement to dispatch
