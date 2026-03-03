# Cycle 108 — 2026-03-03 09:15 UTC

## What happened

### Startup checklist
- No new Eva directives or comments since last cycle
- Closed stale cycle issue [#365](https://github.com/EvaLok/schema-org-json-ld/issues/365) (failed/empty cycle from 07:44 UTC)
- Closed duplicate QC-ACK [#366](https://github.com/EvaLok/schema-org-json-ld/issues/366) (from failed cycle)
- Dual-language consistency: 88/88 PHP/TS schema classes, 12/12 enums — perfect parity

### PR #364 reviewed and merged
PR [#364](https://github.com/EvaLok/schema-org-json-ld/issues/364) (property gaps: QuantitativeValue.unitText, Dataset.hasPart/isPartOf, Organization ProfilePage fields) had a blocking bug — PHP `JsonLdGenerator` couldn't handle mixed arrays containing both TypedSchema objects and strings. The serializer checked only the first array element type and assumed all elements were the same type.

Requested revision via `@copilot`. Copilot fixed it within 4 minutes by switching from first-element-type branching to per-element type dispatch. All 337 tests pass. Merged at 09:31 UTC.

**This was the first revision round in 23 Copilot dispatches.** The mixed-array bug was pre-existing in JsonLdGenerator (not introduced by the PR) — the PR's test simply exposed it. The fix brings PHP in line with the TS serializer which already handled mixed arrays correctly.

### QC-REPORT #160 processed
QC orchestrator reported AggregateRating missing `itemReviewed` property for standalone validation. Created QC-ACK [#368](https://github.com/EvaLok/schema-org-json-ld/issues/368). Dispatched fix as [#370](https://github.com/EvaLok/schema-org-json-ld/issues/370) to Copilot.

### Audit #62 processed
Audit identified QC coverage expansion dispatching building-block types for standalone E2E testing — predictable failures. Created audit-inbound [#369](https://github.com/EvaLok/schema-org-json-ld/issues/369). Main action: posted comprehensive type classification on QC-REQUEST [#331](https://github.com/EvaLok/schema-org-json-ld/issues/331):
- ~28 standalone-testable types (Google Rich Result root types)
- ~58 building-block types (nested only, E2E will fail)
- 12 enums (validated through parent types)

This means the realistic E2E denominator is ~28, not 88. Parity should still cover all 88.

### Property gap audit complete
Audited 8 remaining Google Rich Results types (batch 4), completing the full audit of all 23 standalone types:

| Type | Coverage |
|------|----------|
| BreadcrumbList | 100% |
| EmployerAggregateRating | 100% |
| QAPage | ~85% (Question/Answer missing image/video) |
| Quiz | 100% |
| Review | 100% |
| VacationRental | ~97% (Review.contentReferenceTime, French only) |
| MathSolver | 100% |
| SpeakableSpecification | 100% |

**Overall: 16/23 types at 100% coverage. Remaining gaps are low-priority (QAPage recommended props, French-specific VacationRental field).**

## Current state

- **PR [#364](https://github.com/EvaLok/schema-org-json-ld/issues/364)**: Merged at 09:31 UTC (property gaps + mixed-array fix)
- **Issue [#370](https://github.com/EvaLok/schema-org-json-ld/issues/370)**: Dispatched (AggregateRating.itemReviewed), Copilot working
- **Schema classes**: 88/88 PHP/TS, 12/12 enums — perfect parity
- **Tests**: 337 PHP tests, 1730 assertions, PHPStan level max clean
- **QC parity**: 49/86 (57%)
- **Phase 4 blocked**: QC validation at 49/86. PR #305 waiting for Eva.
- **Property gap audit**: Complete — all 23 standalone types audited

## Open issues/PRs

| # | Type | Status |
|---|---|---|
| [#247](https://github.com/EvaLok/schema-org-json-ld/issues/247) | input-from-eva | Open (will close after npm publish) |
| [#304](https://github.com/EvaLok/schema-org-json-ld/issues/304) | question-for-eva | Open (NPM_TOKEN — deprioritised) |
| [#305](https://github.com/EvaLok/schema-org-json-ld/issues/305) | PR | Open (workflow file — Eva must merge) |
| [#329](https://github.com/EvaLok/schema-org-json-ld/issues/329) | input-from-eva | Open (TS testing directive — pending QC validation) |
| [#331](https://github.com/EvaLok/schema-org-json-ld/issues/331) | qc-outbound | Open (comprehensive TS validation — QC at 49/86) |
| [#353](https://github.com/EvaLok/schema-org-json-ld/issues/353) | audit-inbound | Open (audit #58 — QC parity gap) |
| [#368](https://github.com/EvaLok/schema-org-json-ld/issues/368) | qc-inbound | Open (AggregateRating itemReviewed — fix dispatched #370) |
| [#369](https://github.com/EvaLok/schema-org-json-ld/issues/369) | audit-inbound | Open (audit #62 — type classification posted) |
| [#370](https://github.com/EvaLok/schema-org-json-ld/issues/370) | agent-task | Open (AggregateRating.itemReviewed — Copilot working) |

## Next steps

1. **Review PR from #370** — wait for Copilot, check CI, review, merge
2. **Close #368 and #369** after #370 merges — type classification action complete
3. **Continue monitoring QC parity** — 49/86 → 86/86
4. **QAPage image/video properties** — low priority, dispatch when slot available
5. **README.md class count** — still says 98, actual is ~103
