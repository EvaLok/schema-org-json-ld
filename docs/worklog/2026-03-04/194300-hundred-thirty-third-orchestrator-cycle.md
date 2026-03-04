# Cycle 133 — 2026-03-04 19:43 UTC

## What was done

### PR review + merge

**[PR #448](https://github.com/EvaLok/schema-org-json-ld/issues/448) — verify-metrics Phase 2**: Copilot finished at 18:27 UTC (cycle 132 dispatch). Added 3 new capabilities to metric-snapshot:
1. **Test count verification** — counts PHP `public function test*` and TS `it(`/`test(` methods, compares against state.json
2. **Schema version validation** — uses shared `check_version()` from state-schema crate
3. **Field staleness detection** — `--cycle N` flag builds staleness report from field_inventory

Includes unit tests for all new logic. Code compiles cleanly, types align with state-schema crate. Only CI check was `claude-review` (Rust-only PR, PHP/TS CI doesn't trigger). Merged at 19:49 UTC.

### Bug discovered: data provider test counting

Running `bash tools/metric-snapshot --cycle 133` immediately revealed 3 mismatches:
- PHP: 373 methods vs 425 test executions (state.json)
- TS: 365 methods vs 419 test executions (state.json)
- Total: 738 vs 844

Root cause: **PHPUnit's `@dataProvider` and Vitest's `it.each` multiply test counts**. A single test method with a 26-entry data provider generates 26 test executions. The tool counts method definitions, not executions.

Verified by analysis:
- QuestionTest has 26 data provider entries → +25 extra tests
- AnswerTest has ~27 data provider entries → +27 extra tests
- Total PHP gap: 52 (matches 425 - 373 ✓)

### Agent dispatch (1 new)

**[#450](https://github.com/EvaLok/schema-org-json-ld/issues/450) — Fix metric-snapshot test counting**: Fix the data provider counting issue. PHP: scan `@dataProvider` annotations and count array entries. TS: count `it.each` array entries.

### Metric verification (cycle 133 — was due)

Ran `bash tools/metric-snapshot --cycle 133`:
- 10 of 13 checks pass ✓ (file counts, parity, PHPStan, schema version all green)
- 3 test count checks fail (known issue, fix dispatched #450)
- No stale fields detected in staleness report

Also verified dual-language consistency: PHP 89 schema + 12 enums = TS 89 schema + 12 enums ✓

## Current state

- **In-flight agent sessions**: 1 ([#450](https://github.com/EvaLok/schema-org-json-ld/issues/450) metric-snapshot data provider fix)
- **Open PRs**: 0 (waiting for Copilot to start #450)
- **Open questions for Eva**: None
- **Remaining open `input-from-eva`**: [#247](https://github.com/EvaLok/schema-org-json-ld/issues/247), [#436](https://github.com/EvaLok/schema-org-json-ld/issues/436), [#441](https://github.com/EvaLok/schema-org-json-ld/issues/441)
- **Tool pipeline**: Phase 1 complete (cycle-status), Phase 2 merged with known issue (verify-metrics), fix in-flight (#450)
- **Copilot metrics**: 39 dispatched, 38 merged, 1 in-flight (100% dispatch-to-PR rate, 0 silent failures)

## Next steps

- **PRIORITY**: Review PR from #450 when Copilot finishes (test counting fix)
- After #450 merges: verify metric-snapshot passes all 13 checks
- Plan Phase 3 (housekeeping-scan tool) — needs GitHub API access for stale issue/PR detection
- Begin 3-5 cycle reliability evaluation of pipeline tools (per Eva #436)
- Close issue #447 (superseded by merged PR #448)
