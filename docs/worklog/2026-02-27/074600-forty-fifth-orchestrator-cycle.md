# Cycle 45 — 2026-02-27T07:46Z

## Summary

Forty-fifth orchestrator cycle. Maintenance cycle — no agent dispatches. QC orchestrator acknowledged request [#215](https://github.com/EvaLok/schema-org-json-ld/issues/215) (QC repo [#74](https://github.com/EvaLok/schema-org-json-ld-qc/issues/74)), baseline showing 13 warnings (down from 19). QC updating scripts to exercise new Recipe properties. Reviewed Google Search Gallery — found "Book actions" type excluded from scope (catalog data feed, not page-level annotation).

## What happened

### Startup

1. No `input-from-eva` issues
2. No open PRs, no agent sessions in-flight
3. QC request [#215](https://github.com/EvaLok/schema-org-json-ld/issues/215) acknowledged by QC orchestrator — [QC repo #74](https://github.com/EvaLok/schema-org-json-ld-qc/issues/74) opened
4. No open `question-for-eva` issues
5. Repo clean: only `master` branch, no stale branches or orphan PRs

### QC validation progress

QC repo [#74](https://github.com/EvaLok/schema-org-json-ld-qc/issues/74) body reports:
- Package updated to commit `56b0413` (includes PR #214 and PR #218)
- Baseline E2E: 39/39 pass, 0 errors, 13 warnings (was 19 last cycle)
- 6-warning reduction likely from PR #218 (ProductGroup `subjectOf`) and updated package version
- QC dispatching Copilot to update generate scripts for 5 new Recipe properties
- No final results posted yet

### Google Search Gallery review

Checked current gallery for new types. Found **Book actions** listed — this type was not in our original scope. Investigation reveals it's fundamentally different from page-level structured data:
- Uses `DataFeed` as root container (not standard `@type` annotation)
- Designed for catalog-level data feeds (books, editions, actions)
- Validated by a separate Data Feed validation tool (not Rich Results Test)
- Requires hosted feed URLs (SFTP, S3, etc.)

Confirmed out of scope. All 26 page-level Rich Results types remain implemented and current.

## Final state

- **Open PRs**: None
- **Agent sessions**: None
- **QC**: Request [#215](https://github.com/EvaLok/schema-org-json-ld/issues/215) — acknowledged, awaiting final validation results
- **Tests**: 321, **Classes**: 98, **PHPStan**: level 9

## Next steps

1. Monitor QC response on [#215](https://github.com/EvaLok/schema-org-json-ld/issues/215) / [QC repo #74](https://github.com/EvaLok/schema-org-json-ld-qc/issues/74)
2. Close [#215](https://github.com/EvaLok/schema-org-json-ld/issues/215) once QC confirms validation
3. Respond to Eva directives if any arrive
4. Remaining low-priority: JobPosting beta props, isVariantOf exercise (QC-side)
