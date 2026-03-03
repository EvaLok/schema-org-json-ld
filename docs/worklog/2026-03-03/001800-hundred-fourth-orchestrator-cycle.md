# Cycle 104 — 2026-03-03 00:18 UTC

## What happened

**Eva directive [#340](https://github.com/EvaLok/schema-org-json-ld/issues/340)**: Standardize all TypeScript constructor params to use options-object pattern uniformly. Eva referenced the audit journal's observation about constructor mismatches discovered during QC parity testing. The directive aligns TypeScript API ergonomics with PHP's named parameters — consumers should never need to worry about constructor parameter ordering.

### Analysis

- 86 total schema classes in `ts/src/schema/`
- 26 already use options-object pattern (types with many properties)
- 52 use positional constructor parameters (need conversion)
- 8 have no constructor (inherit from parent, no changes needed)

### Actions taken

1. **Updated AGENTS-ts.md**: Removed the old "≤5 optional = positional params" threshold rule. All classes now mandated to use options-object constructors. Updated examples, test patterns, property name mapping section, and common mistakes section.

2. **Dispatched batch 1**: Created [#342](https://github.com/EvaLok/schema-org-json-ld/issues/342) — refactor 16 classes (A-D) from positional to options-object constructors. Assigned to Copilot (gpt-5.3-codex).

3. **Noted audit [#56](https://github.com/EvaLok/schema-org-json-ld-audit/issues/56)**: QC-targeted recommendation about QC Copilot dispatch pipeline. No action needed from main orchestrator. Added to audit_processed.

### Batch plan

Sequential dispatch (per audit #29 — barrel file conflicts):
- **Batch 1** (A-D, 16 classes): [#342](https://github.com/EvaLok/schema-org-json-ld/issues/342) — dispatched
- **Batch 2** (E-M, ~15 classes): after batch 1 merges
- **Batch 3** (O-V, ~21 classes): after batch 2 merges

## Self-modifications

- **AGENTS-ts.md**: Replaced dual pattern (positional/options) with uniform options-object pattern. Updated all code examples, test patterns, and common mistakes section.

## Current state

- **Constructor refactoring**: batch 1 dispatched, batches 2-3 planned
- **Phase 4 still blocked**: QC parity at 39/86 (unchanged)
- **No new QC reports or Eva comments on tracked issues**
- **Consecutive idle cycles**: reset to 0

## Open issues/PRs

| # | Type | Status |
|---|---|---|
| [#247](https://github.com/EvaLok/schema-org-json-ld/issues/247) | input-from-eva | Open (will close after npm publish) |
| [#304](https://github.com/EvaLok/schema-org-json-ld/issues/304) | question-for-eva | Open (NPM_TOKEN — deprioritised) |
| [#305](https://github.com/EvaLok/schema-org-json-ld/issues/305) | PR | Open (workflow file — Eva must merge) |
| [#329](https://github.com/EvaLok/schema-org-json-ld/issues/329) | input-from-eva | Open (TS testing directive — pending QC validation) |
| [#331](https://github.com/EvaLok/schema-org-json-ld/issues/331) | qc-outbound | Open (comprehensive TS validation — QC at 39/86) |
| [#338](https://github.com/EvaLok/schema-org-json-ld/issues/338) | audit-inbound | Open (response to audit #53) |
| [#340](https://github.com/EvaLok/schema-org-json-ld/issues/340) | input-from-eva | Open (uniform constructor pattern directive) |
| [#342](https://github.com/EvaLok/schema-org-json-ld/issues/342) | agent-task | Dispatched (batch 1 constructor refactoring) |

## Next steps

- Wait for batch 1 PR from Copilot, review when `copilot_work_finished`
- After batch 1 merges, dispatch batch 2 (E-M, ~15 classes)
- After batch 2 merges, dispatch batch 3 (O-V, ~21 classes)
- After all 3 batches merge, note that QC will need to re-validate (constructor signatures changed)
- Continue monitoring QC parity progress toward 86/86
