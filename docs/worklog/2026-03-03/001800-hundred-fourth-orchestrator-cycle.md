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

### Batch execution (all 3 completed in this cycle)

Sequential dispatch (per audit #29 — barrel file conflicts):

| Batch | Range | Classes | Issue | PR | Merged |
|-------|-------|---------|-------|-----|--------|
| 1 | A-D | 16 | [#342](https://github.com/EvaLok/schema-org-json-ld/issues/342) | [#343](https://github.com/EvaLok/schema-org-json-ld/pull/343) | 00:46:46Z |
| 2 | E-M | 15 | [#344](https://github.com/EvaLok/schema-org-json-ld/issues/344) | [#345](https://github.com/EvaLok/schema-org-json-ld/pull/345) | 01:07:51Z |
| 3 | O-V | 22 | [#346](https://github.com/EvaLok/schema-org-json-ld/issues/346) | [#347](https://github.com/EvaLok/schema-org-json-ld/pull/347) | 01:26:40Z |

**Total**: 53 classes converted, 8 skipped (no constructor/inherit), ~160 files changed across 3 PRs. All builds, tests, and Biome passed. Eva directive [#340](https://github.com/EvaLok/schema-org-json-ld/issues/340) closed.

### Review observations

- All 3 Copilot sessions produced correct conversions on first attempt — zero revision rounds
- Cross-reference test updates were comprehensive (e.g., Product.test.ts, Organization.test.ts updated when their nested types were converted)
- Special cases preserved: MathSolver multi-type `schemaType`, SolveMathAction `propertyMap`, array types, union types
- MobileApplication, WebApplication, Restaurant, Store, BlogPosting, NewsArticle, FoodEstablishment, NutritionInformation correctly skipped (no own constructor)

## Self-modifications

- **AGENTS-ts.md**: Replaced dual pattern (positional/options) with uniform options-object pattern. Updated all code examples, test patterns, and common mistakes section.

## Current state

- **Constructor refactoring**: COMPLETE — all 53 classes converted across 3 batches
- **Phase 4 still blocked**: QC parity at 39/86 (unchanged). QC will need to re-validate (constructor signatures changed — affects parity testing callsites)
- **Eva directive #340**: CLOSED
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

## Next steps

- QC orchestrator will need to update its parity test callsites (constructor signatures changed). Note on QC-REQUEST #331 may be appropriate.
- Continue monitoring QC parity progress toward 86/86
- Phase 4b/4c still blocked pending QC validation + Eva actions (PR #305, NPM_TOKEN)
