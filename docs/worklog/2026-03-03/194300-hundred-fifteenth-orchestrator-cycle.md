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

Dispatched [#398](https://github.com/EvaLok/schema-org-json-ld/issues/398) to Copilot. [PR #399](https://github.com/EvaLok/schema-org-json-ld/issues/399) merged at 19:59Z. CI green on all 7 checks (Code Style, Static Analysis, PHP 8.1-8.5). PHPStan now at level max in config.

### Eva pushed npm publish workflow
Eva pushed commit `f59a531` directly to master with:
1. **`publish-npm.yml`**: OIDC-based npm trusted publishing (`--provenance`, `id-token: write`). No NPM_TOKEN secret needed.
2. **`verify-build.mjs` fix**: Brand constructor changed from `"Acme"` to `{ name: "Acme" }` (options-object pattern from cycle 104)
3. **README badge**: Node 22 badge added

This changes the publish infrastructure significantly:
- [PR #305](https://github.com/EvaLok/schema-org-json-ld/issues/305) appears superseded — posted comment asking Eva to confirm
- [#304](https://github.com/EvaLok/schema-org-json-ld/issues/304) (NPM_TOKEN) may be obsolete — OIDC doesn't need repository secrets
- Phase 4c now requires: Eva configures npm OIDC linking + creates GitHub Release

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

- **Copilot sessions**: 0 in-flight, 31/31 merged (100%)
- **Schema classes**: 89/89 PHP/TS parity (verified)
- **QC parity**: 73/73 standalone types (100%) — gate satisfied
- **Phase 4**: Eva pushed OIDC publish workflow. Awaiting Eva to configure npm linking + create release.
- **PHPStan**: Level max (upgraded from 9 via PR #399)

## Open issues/PRs

| # | Type | Status |
|---|---|---|
| [#247](https://github.com/EvaLok/schema-org-json-ld/issues/247) | input-from-eva | Open (will close after npm publish) |
| [#304](https://github.com/EvaLok/schema-org-json-ld/issues/304) | question-for-eva | Open (may be obsolete — OIDC approach doesn't need NPM_TOKEN) |
| [#305](https://github.com/EvaLok/schema-org-json-ld/issues/305) | PR | Open (likely superseded by Eva's direct push of publish-npm.yml) |
| [#329](https://github.com/EvaLok/schema-org-json-ld/issues/329) | input-from-eva | Open (QC validation fulfilled, awaiting Eva) |

## Next steps

1. Await Eva's response on PR #305 / #304 closure
2. When Eva creates a GitHub Release, publish workflow triggers automatically
3. Continue monitoring for audit recommendations
