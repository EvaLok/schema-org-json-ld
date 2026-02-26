# Cycle 16 — 2026-02-25T10:50Z

## Summary

Sixteenth orchestrator cycle. Conducted comprehensive quality audit of 6 schema types against Google's structured data documentation. Found and fixed 5 missing Google-recommended properties across shared sub-types and parent types. Both PRs merged with zero revisions. 29 consecutive zero-revision PRs.

## What happened

### Startup

1. No `input-from-eva` issues.
2. No open `question-for-eva` issues.
3. Clean slate: 0 in-flight sessions, only master branch.
4. Recovered context from Cycle 15 worklog.

### Quality audit

Audited 6 schema type implementations against Google's structured data docs:

| Type | Status | Gaps Found |
|------|--------|------------|
| Article | Compliant | Minor: missing tests for speakable/hasPart properties |
| Recipe | Gap | Missing `video` (recommended by Google) |
| Event | Gap | Missing `Offer.validFrom` (recommended for tickets) |
| JobPosting | Gaps | Missing `MonetaryAmount.unitText` (salary period), `jobLocation` should be nullable for remote jobs |
| LocalBusiness | Minor gap | Missing `department` (low priority) |
| Course | Compliant | CourseInstance built for deprecated feature but not harmful |

### High-impact findings fixed

1. **MonetaryAmount.unitText** — Google requires this for salary data (HOUR/MONTH/YEAR values). Without it, salary structured data was incomplete.
2. **Review.author** — Changed from `string` to `string|Person|Organization`. Google expects nested objects, not plain strings.
3. **Offer.validFrom** — Added for Event ticket sale dates.
4. **Recipe.video** — Added VideoObject support (recommended by Google).
5. **JobPosting.jobLocation** — Made nullable for fully remote jobs (Google spec requires omitting for TELECOMMUTE).

### Agent dispatches

- **[Issue #104](https://github.com/EvaLok/schema-org-json-ld/issues/104) → [PR #105](https://github.com/EvaLok/schema-org-json-ld/issues/105)**: Offer, MonetaryAmount, Review property fixes. Agent time: ~7 min.
- **[Issue #106](https://github.com/EvaLok/schema-org-json-ld/issues/106) → [PR #107](https://github.com/EvaLok/schema-org-json-ld/issues/107)**: Recipe, JobPosting property fixes. Agent time: ~7 min.
- Both dispatched concurrently (no file overlap). Both merged after local test verification + CI pass.

## Agent performance

| Task | Files Changed | Agent Time | Revision? |
|------|--------------|-----------|-----------|
| Offer/MonetaryAmount/Review | 3 src + 3 test | ~7 min | No |
| Recipe/JobPosting | 2 src + 2 test | ~7 min | No |

Zero-revision streak: now 29 consecutive clean PRs since Cycle 4.

## Current state

- **Implemented types**: 28 Google Rich Results types — 100% coverage
- **Quality fixes merged**: 2 PRs this cycle
- **In-flight sessions**: 0
- **Blockers**: None
- **Total tests**: 204 (up from 201)
- **Total assertions**: 1169 (up from 1158)
- **Agent premium requests this cycle**: 2 (2 dispatches, 0 revisions)

## Remaining low-priority audit findings

- LocalBusiness missing `department` property
- LocalBusiness subtypes (Restaurant, Store, etc.) not implemented
- Offer.itemCondition should be optional (Product-specific, irrelevant for Events)
- CourseInstance.courseMode unnecessarily required
- HowToSection not supported for Recipe grouped instructions
- EventAttendanceMode/VirtualLocation not supported

## Next steps (for next cycle)

1. These low-priority items could be addressed if Eva wants further refinement
2. Library is now feature-complete AND quality-audited
3. Consider requesting QC validation from the QC orchestrator
4. Natural pause point unless Eva has new directions
