# Cycle 18 — 2026-02-25T13:55Z

## Summary

Eighteenth orchestrator cycle. Three main accomplishments: (1) Analyzed permission model failures and built 5 new orchestrator tools per Eva's directive ([#113](https://github.com/EvaLok/schema-org-json-ld/issues/113)), (2) Fixed QC report [#8](https://github.com/EvaLok/schema-org-json-ld/issues/8) — added itemReviewed to Review + Thing sub-type, (3) Made Offer.itemCondition and CourseInstance.courseMode optional. Both PRs merged with zero revisions. 33 consecutive zero-revision PRs.

## What happened

### Startup

1. Found `input-from-eva` [issue #113](https://github.com/EvaLok/schema-org-json-ld/issues/113): Eva requested analysis of tool use permission failures and building tools to avoid them.
2. Found QC report [#8](https://github.com/EvaLok/schema-org-json-ld/issues/8) from QC orchestrator: Review class missing required itemReviewed property.
3. Clean slate: 0 in-flight sessions, no stale branches/PRs.
4. Recovered context from Cycle 17 worklog.

### Permission model analysis (Eva [#113](https://github.com/EvaLok/schema-org-json-ld/issues/113))

Identified root cause: the workflow `permissions.allow` only includes specific command prefixes (gh, git, jq, mkdir, ls, date, wc, sort, composer). Commands like `bash`, `echo`, `cat`, `chmod` are NOT allowed. This means:
- Shell tools built in previous cycles (agent-status, review-pr, etc.) have never actually worked in automation
- Every cycle has been using raw gh/jq commands directly

Actions taken:
- Built 5 new tools: comment-issue, session-info, qc-check, create-issue, post-opening
- Updated STARTUP_CHECKLIST.md to use only allowed command patterns
- Created `.claude/skills/orchestrator-permissions.md` documenting allowed vs blocked commands
- Proposed workflow change to Eva: add `Bash(bash tools/*)`, `Bash(chmod +x tools/*)`, `Bash(cat *)`, `Bash(echo *)` to allowlist
- Cannot push workflow changes (PAT lacks workflow scope) — documented in [issue #113](https://github.com/EvaLok/schema-org-json-ld/issues/113) comment
- Closed [issue #113](https://github.com/EvaLok/schema-org-json-ld/issues/113)

### QC report fix ([PR #117](https://github.com/EvaLok/schema-org-json-ld/issues/117))

QC orchestrator reported Review class missing itemReviewed property for standalone Review rich results.
- Created QC-ACK [issue #115](https://github.com/EvaLok/schema-org-json-ld/issues/115)
- Dispatched fix as [issue #116](https://github.com/EvaLok/schema-org-json-ld/issues/116) → [PR #117](https://github.com/EvaLok/schema-org-json-ld/issues/117)
- Created new Thing sub-type (simplest possible itemReviewed value)
- Added optional `null|TypedSchema $itemReviewed` to Review
- 3 new tests, 8 new assertions
- Agent time: ~8 min. Merged with zero revisions.

### Optional params fix ([PR #119](https://github.com/EvaLok/schema-org-json-ld/issues/119))

Quality audit finding: Offer.itemCondition and CourseInstance.courseMode were unnecessarily required.
- Dispatched as [issue #118](https://github.com/EvaLok/schema-org-json-ld/issues/118) → [PR #119](https://github.com/EvaLok/schema-org-json-ld/issues/119)
- Both changed to nullable with null default
- 2 new tests
- Agent time: ~7 min. Merged with zero revisions.

## Agent performance

| Task | PR | Agent Time | Revision? |
|------|-----|-----------|-----------|
| Review itemReviewed + Thing | [#117](https://github.com/EvaLok/schema-org-json-ld/issues/117) | ~8 min | No |
| Offer/CourseInstance optional | [#119](https://github.com/EvaLok/schema-org-json-ld/issues/119) | ~7 min | No |

Zero-revision streak: now 33 consecutive clean PRs.

## Current state

- **Implemented types**: 28 Google Rich Results types — 100% coverage
- **Quality fixes this cycle**: 2 PRs merged
- **New sub-types**: Thing (66 total)
- **In-flight sessions**: 0
- **Blockers**: Workflow permissions change needs Eva's action
- **Total tests**: 218
- **Total assertions**: 1219
- **Agent premium requests this cycle**: 2 (2 dispatches, 0 revisions)
- **QC report [#8](https://github.com/EvaLok/schema-org-json-ld/issues/8)**: Fixed, awaiting re-validation

## Remaining low-priority audit findings

- LocalBusiness missing department property
- LocalBusiness subtypes (Restaurant, Store, etc.) not implemented
- HowToSection not supported for Recipe grouped instructions
- EventAttendanceMode/VirtualLocation not supported

## Next steps (for next cycle)

1. Check if Eva has applied the workflow permissions change
2. Check QC repo for re-validation results on the Review fix
3. Continue with low-priority audit findings if desired
4. Consider requesting full QC validation for all types
