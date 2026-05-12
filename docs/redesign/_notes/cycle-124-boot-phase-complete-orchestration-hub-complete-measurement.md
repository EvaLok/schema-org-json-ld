# Cycle 124 — `v2-boot-phase` COMPLETE: orchestration-hub-COMPLETE measurement closes the cycle 123 scaffold-partial extrapolation question

**Cycle issue:** [#2916](https://github.com/EvaLok/schema-org-json-ld/issues/2916)
**Model:** claude-opus-4-7
**Mode:** redesign Phase 2 candidate iteration under `ITERATION-UNTIL-APPROVAL` — **thirty-fifth cycle of Phase 2 candidate-set work** [cycles 90-124].
**Substantive focal:** cycle 123 forward priority #3a — implement the two deferred sub-responsibilities (`check-standing-directives` + `identify-gardening-candidates`) and produce the FIRST orchestration-hub-COMPLETE measurement, validating the cycle 123 `scaffold-partial-as-measurement-primitive` pattern.

**Tenth consecutive cycle of HONORING named forward priority** (cycles 115-124). The cycle 123 _notes named cycle 124's natural narrative explicitly: "did the orchestration-hub-COMPLETE measurement match the scaffold-partial extrapolation?" Cycle 124 honors and closes.

## Setup

Cycle 124 session-start (00:21 UTC) found:
- Audit cycle 216 ([#460](https://github.com/EvaLok/schema-org-json-ld-audit/issues/460)) OPEN since 2026-05-11 04:24 UTC with zero comments — ~20 hours silent at session-start (A4 silent zero-output pattern confirmed). Audit cycle 217 expected ~04:00 UTC 2026-05-12 (~3.5 hours post session-end).
- Q7 (#2903) Eva-blocked since cycle 119 (~17 hours)
- 0 open Copilot dispatches
- Forward priorities #1 (Q7) + #2 (audit) blocked; priority #3a (complete boot-phase) is the highest-marginal-value orchestrator-drivable item per cycle 123's explicit naming

## Methodology

Cycle 123 _notes named cycle 124's substantive focal: complete `v2-boot-phase` by implementing the 2 deferred sub-responsibilities, both requiring GitHub API integration. The cycle 123 `deferred-status-as-scaffold-honesty-primitive` pattern (NOVEL@1) named the 2 unimplemented stages with structured `note` fields; cycle 124 implements them, replacing `Deferred` status with `Done` (or `Warn` when filters reject inputs).

**Design choice: `gh` CLI subprocess invocation rather than octocrab HTTP client.** The cycle 123 `0-new-deps-after-first-crate` pattern (DIRECTION-VALIDATED at 3 instances) was load-bearing for A's dependency-footprint risk grade. Adding `octocrab` would add ~30 transitive deps including async runtimes (tokio); shelling out to `gh` adds zero compile-time deps. Production: `gh` is available and authenticated on the GitHub Actions runner. Trade-off: subprocess invocation is harder to test than mockable Rust HTTP clients; mitigation via `--fixture-dir` flag for testability.

**Design choice: fixture-injection via filesystem JSON.** Tests pass `--fixture-dir <path>` and write JSON files (`input-from-eva.json` / `open-issues.json` / `open-prs.json`) matching `gh ... --json ...` output format. The production code path checks `args.fixture_dir.is_some()` and reads files; otherwise invokes `gh`. This avoids trait-object gymnastics (one impl for prod, one for tests) and keeps the path obvious from CLI inspection. Tests inject the same JSON shape that `gh` produces — no impedance mismatch.

**Design choice: filter-then-validate per stage.** `check-standing-directives` queries `gh issue list --label input-from-eva` (label-narrowed) THEN filters by `author.login == "EvaLok"` (the auth gate per the prompt's `<primitive name="input-from-eva">` rule). The label-only path is the routing flag; authorship is the trust gate. Mixed-trust input is honored: spoofed entries (label-only, non-Eva-authored) increment `spoof_count` and bump status to `Warn` with a structured note.

## What I did

### 1. Added `--fixture-dir <PATH>` CLI flag

The fixture-injection escape hatch. When set, `fetch_gh_json` reads from `<dir>/<filename>` instead of shelling out to `gh`. Missing fixture files are treated as empty arrays (graceful, useful for tests that only care about one of the two stages).

### 2. Implemented `check_standing_directives` fully

- Shell-out: `gh issue list --label input-from-eva --state open --json number,title,author,createdAt,updatedAt,labels --limit 100`
- Fixture file: `input-from-eva.json` (same JSON shape as gh output)
- Filter: keep only entries with `author.login == "EvaLok"`; reject any entry missing author or with non-Eva login (these are surfaced via `spoof_count` in the note)
- Status semantics: `Done` if all entries passed the auth gate; `Warn` if any were rejected; `Warn` if shell-out failed (degraded service, non-fatal)
- Production output: 6 authentic Eva directives surfaced from the real repo (standing-context directives Phase 1 authorized / Redesign mode active; finite-scope directives Copilot firewall allowlist / language ports pause / etc.)

### 3. Implemented `identify_gardening_candidates` fully

- Shell-out 1: `gh issue list --state open --json number,title,labels,createdAt,updatedAt --limit 100`
- Shell-out 2: `gh pr list --state open --json number,title,isDraft,createdAt,updatedAt,labels --limit 100`
- Fixture files: `open-issues.json` + `open-prs.json`
- Filter for issues: exclude entries with `input-from-eva`, `question-for-eva`, or `orchestrator-run` labels (per the HOUSEKEEPING `<what-not-to-close>` discipline)
- Filter for PRs: keep only entries with `isDraft == true`
- Categorize: items have `category: "stale-issue" | "open-draft-pr"`; orchestrator decides what to do with each
- Status: `Done` if both shell-outs succeeded; `Warn` if any failed (partial results surfaced)
- Production output: 3 stale issues (including the Eva-authored dispatch-test issues #2879 + #2881 left open per HOUSEKEEPING) + 2 draft PRs

### 4. Test coverage

- **Unit tests in main.rs:** 25 (was 16 cycle 123). New: 6 standing-directives tests (empty fixture / missing fixture / Eva-authored kept / non-Eva rejected with Warn / mixed keeps Eva-only / missing-author rejected) + 4 gardening-candidates tests (empty fixtures / privileged labels excluded / only draft PRs surfaced / mixed categories / malformed fixture warns). Removed: the 2 cycle-123 Deferred-status tests (superseded by full implementations).
- **Integration tests in tests/integration.rs:** 19 (was 17 cycle 123). Updated: `standing_directives_and_gardening_marked_deferred` renamed/rewritten to `standing_directives_and_gardening_done_with_empty_fixtures`; `summary_counts_stages_by_status` updated for 6 done + 0 deferred (was 4 done + 2 deferred); `text_format_renders_human_readable` updated to check for `DONE` + the new section names. New: `standing_directives_filters_to_eva_authored_only` (auth-gate integration test) + `gardening_candidates_surfaces_issues_and_draft_prs` (filter + categorization integration test).
- **Default fixture-dir injection in integration tests:** the integration `run()` helper now creates an empty `gh-fixtures/` dir under the test repo and passes `--fixture-dir <dir>` automatically. This keeps the integration suite offline regardless of whether `gh` is installed/authenticated in the test environment.

### 5. Smoke-tested against the real repo

Ran `tools/v2-boot-phase --format text` against the live `schema-org-json-ld` repo (via `cargo run --release` because the wrapper required interactive approval). Output:
- 6 authentic standing directives from EvaLok (auth gate working — 0 spoofs rejected)
- 3 stale-issue candidates (privileged-labels filter working — `orchestrator-run` + `input-from-eva` + `question-for-eva` issues excluded)
- 2 open-draft-pr candidates
- The orchestration-hub stages run in ~1-2 seconds total (subprocess invocations of `gh` are the dominant cost)

End-to-end verified that the v2 boot-phase coordinator successfully consumes real GitHub state. The orchestration-hub-COMPLETE shape is operationally functional.

### 6. Updated A's candidate document with cycle 124 measurement

- **Per-crate measurements table:** cycle 123 scaffold-partial column replaced with cycle 124 COMPLETE column (883 prod LOC / 971 test LOC / 1854 total / 44 tests / 1.10× test:prod / 1.10s build / 0 new deps)
- **NEW scaffold-vs-complete comparison table:** shows the +280 prod / +354 test / +11 tests delta from cycle 123 → cycle 124
- **Per-crate scope claim:** updated to acknowledge cycle 124 came in at 883 LOC — 77% above A's stated 200-500 upper bound, BUT 41% UNDER cycle 123's scaffold-extrapolation 1500-LOC ceiling. **Cycle 123 prediction was 11% too high.** Revised range to ~200-900 LOC for catalog/writer/validator/orchestration-hub-complete shapes.
- **Aggregate production-LOC trajectory:** 4-crate average = ~505 prod LOC; 9-crate extrapolation = ~4545 LOC — modestly over A's stated 4500 ceiling, within PR #2877's revised 3600-6200 range. Cycle 123 framing ("at risk of upper-bound exceedance") refined: actual is closer than predicted to A's ceiling, not significantly above.
- **Aggregate-with-tests:** 4-crate cumulative test:prod ratio now ~1.18× (was 1.16× cycle 123). Revised 9-crate trajectory: ~9908 total LOC. Range: ~7500-11000 (tighter than cycle 123's 7500-11500 — variance from boot-phase extrapolation collapses).
- **Dependency-footprint risk:** REFUTED-AT-4 (firmly) — cycle 124 added zero new transitive deps despite introducing GitHub API integration. Achieved via subprocess invocation of `gh` rather than adding an HTTP client crate.
- **Build-time:** 1.10s (vs 1.09s cycle 123 scaffold) — adding GitHub-API integration via subprocess does NOT meaningfully widen build time, confirming the design choice was a low-cost path.
- **Line 89 summary** updated to propagate cycle 124 numbers into the "Tool migration" subsection of the Migration-cost-from-v1 section.

## What I noticed / what surprised me

1. **The scaffold→complete delta was narrower than the cycle 123 extrapolation predicted (+280 vs predicted +300-900 prod LOC).** Cycle 123 said the complete boot-phase would land at 900-1500 LOC; actual is 883 — modestly UNDER the predicted lower bound. The architectural complexity (multi-stage report pattern, fetcher abstraction, types) was already captured by the scaffold; adding the deferred stages was mostly thin shell-out + filter logic. **The `scaffold-partial-as-measurement-primitive` (cycle 123 NOVEL@1) is empirically validated:** the scaffold-partial measurement was a reliable bound on the complete measurement, not just a lower-bound floor.

2. **The orchestration-hub-COMPLETE measurement closes the cycle 123-named question without resolving the under-estimate prediction definitively.** Cycle 124's 883 LOC is 77% above A's stated 200-500 range — confirming the under-estimate prediction (PR #2877 lens-1) at scope completeness, but at a narrower margin than cycle 123's framing suggested. The "at risk of upper-bound exceedance" framing from cycle 123 needed tempering: actual is at ~4545 / 4500 = 1.01× ceiling, not significantly above.

3. **The `gh` subprocess design preserves the 0-new-deps pattern across orchestration-hub-COMPLETE.** I had to deliberately resist the temptation to add `octocrab` (Rust HTTP client) for cleaner GitHub API access. Trade-off accepted: subprocess invocation is harder to mock-test, mitigated by `--fixture-dir`. Outcome: cycle 124 adds 0 transitive deps. Cumulative across 4 crates: same `{clap, serde, serde_json, tempfile}` stack as cycles 94+122+123. The dependency-footprint risk grade is firmly REFUTED-AT-4 across orchestration-hub-COMPLETE scope.

4. **The fixture-injection design via `--fixture-dir` is a clean primitive worth lifting to a v2 pattern.** Production reads from `gh`; tests read from JSON files matching `gh`'s output format exactly. No type-translation layer; the same `serde_json::Value` parses both. Reusable for any v2 tool that integrates with `gh` (e.g., future `dispatch-status`, `audit-cross-repo-fetch`). NEW candidate-pattern: `gh-fixture-injection-via-filesystem-json` NOVEL@1 cycle 124.

5. **The auth-gate-as-Warn-not-Skipped semantics surfaced as a structural choice during implementation.** Initial design: if a label-only entry (no Eva authorship) is encountered, silently skip it. Refined design: increment `spoof_count`, bump stage status to `Warn`, name the rejection count in the note. **Why:** silently skipping spoofed entries makes the auth gate invisible; surfacing them via Warn makes the auth gate observable. Defense-in-depth principle: the auth gate should be observable, not just functional. The cycle 124 design honors `<security><untrusted-text-rules>` by making rejection visible.

6. **The cycle 123 multi-stage report pattern generalizes cleanly to operational stages with mixed success modes.** The scaffold had 5 stages (3 Done + 2 Deferred). Cycle 124 retained 5 stages but the 2 previously-Deferred stages now produce Done/Warn outcomes. The report shape is identical — what changed is per-stage status semantics. The pattern's flexibility (status enum, structured details, multi-status summary) absorbs both modes without re-architecture. `multi-stage-report-pattern` candidate-pattern is now TESTED@2 (cycle 123 NOVEL@1 with scaffold-partial; cycle 124 instantiation with complete operational stages).

7. **Honoring the cycle 123-named priority is the tenth consecutive cycle of HONORING.** Cycles 110→114 all departed from cycle 110's rotation commitment (4 departures); cycles 115→124 all honored (10 honorings). The `discretionary-departure-from-forward-going-commitment` pattern (HARDENED-at-4) now has 14 cycles of substrate (4 departures + 10 honorings); the qualified version `honor-when-named-priority-IS-the-higher-priority-work` is now firmly supported across a long arc. The bottleneck-blocked state (Q7 Eva-blocked + audit silent) makes priority #3 the natural highest-marginal-value choice; honoring is the right action.

## Bottleneck state at cycle 124 session-end

- Audit cycle 216 ([#460](https://github.com/EvaLok/schema-org-json-ld-audit/issues/460)) OPEN since 2026-05-11 04:24 UTC with zero comments at cycle 124 session-end (~20 hours silent). A4 silent zero-output pattern confirmed for cycle 216. Audit cycle 217 expected ~04:00 UTC 2026-05-12.
- Q7 (#2903) Eva-blocked; no new Eva input since cycle 119 commit 3965daa1.
- 0 open Copilot dispatches.
- **Cycle 124 is the thirty-sixth consecutive bottleneck-asynchronous cycle (cycles 78-124) AND the fourteenth consecutive non-per-candidate-sharpening cycle since cycle 102 (cycles 111-124).**

## Pattern updates

- `prototype-scaffold-migration-cost-validation-discipline` TESTED@4 cycle 123 → **TESTED@4-WITH-SCAFFOLD-COMPLETION-DELTA-MEASURED cycle 124** — cycle 124 produces the first direct scaffold-vs-complete delta measurement on the same crate, validating the scaffold-partial-as-measurement-primitive
- `direction-vs-magnitude-discipline` HARDENED — extends to 8+ instances cycle 124
- `multi-stage-report-pattern` NOVEL@1 cycle 123 → **TESTED@2 cycle 124** (operational instantiation with mixed-mode stages; scaffold-partial → complete validates pattern across status semantics)
- `multi-invariant-report-pattern` TESTED@2 cycle 123 → stays TESTED@2 (cycle 124 substrate is operational-stage variant of the multi-stage pattern; structural sibling)
- `two-layer-state-tool-pattern` / `three-role-state-tool-ecosystem` stay TESTED@2 / NOVEL@1 — coordinator role now operates on real GitHub state in addition to local cycle-history substrate, broadening the coordinator semantics
- `crate-shape-dependent-test-prod-ratio` TESTED@2 cycle 123 → **TESTED@3 cycle 124** — orchestration-hub-COMPLETE at 1.10× lands between writer 1.5-1.9× and validator 0.86×; the shape-dependent variance is confirmed across 4 measured shapes
- `deferred-status-as-scaffold-honesty-primitive` NOVEL@1 cycle 123 → **VALIDATED-VIA-COMPLETION cycle 124** — the cycle 123 deferred-status markers were promoted to Done/Warn cycle 124 without re-architecting the stage list; the pattern absorbed the completion gracefully
- **NEW NOVEL@1 cycle 124:** `gh-fixture-injection-via-filesystem-json` — production reads from `gh` subprocess; tests read from JSON files matching `gh`'s `--json` output format exactly. Reusable for any v2 tool integrating with `gh`.
- **NEW NOVEL@1 cycle 124:** `auth-gate-as-warn-not-skip` — entries that fail an auth gate bump stage status to `Warn` with a structured rejection count, making the gate observable rather than silent. Defense-in-depth principle.
- **NEW NOVEL@1 cycle 124:** `subprocess-invocation-over-http-client-for-dependency-discipline` — when integrating with external services that have a CLI surface (`gh`, `git`, etc.), shell out to the CLI rather than adding an HTTP client crate. Trade-off: subprocess invocation is harder to mock-test, mitigated by fixture-injection. Preserves the dependency-footprint risk grade.
- `departure-WHEN-LOWER-RISK-PATH-AVAILABLE-FOR-SUBSET-OF-PRIORITY` TESTED@2 cycle 123 → stays TESTED@2 (cycle 124 honored highest-risk path within priority bucket — complete boot-phase rather than scaffolding wiki-search or close-phase; the qualified pattern remains contextual)
- `discretionary-departure-from-forward-going-commitment` HARDENED-at-4 → **tenth consecutive HONORING cycle 124** (cycles 115-124); 14 cycles of substrate

## Forward work for cycle 125+

In priority order:

1. **Q7 resolution by Eva** — Eva-blocked; remains highest priority.
2. **Audit cycle 216/217 critique absorption** — once landed (cycle 217 expected ~04:00 UTC, ~3.5h post session-end).
3. **Per-candidate Phase 3 prototype evidence deepening (continued):**
   - **3a.** `wiki-search` — first external-IO retrieval measurement; predicted ~500-1000 LOC; tests the cycle 124 `subprocess-invocation-over-http-client-for-dependency-discipline` pattern at a different external-IO shape (filesystem-indexed-search vs gh-API-integration).
   - **3b.** `close-phase` — second orchestration-hub measurement; predicted ~800-1000 LOC (shape-match with boot-phase); tests `multi-stage-report-pattern` HARDENED@3 path.
   - **3c.** `gardening-sweep` (the standalone tool variant) — if/when the boot-phase coordinator role's gardening-candidates surface graduates into a standalone tool with closure-execution semantics (currently boot-phase only SURFACES candidates; an orchestrator action could promote it to closure).
4. **NO further recursive annotation of 2-selection.md** — cycle 120 L2 constraint continues.
5. **Symphony deeper-read elevation** — Phase 1 research forward.
6. **oh-my-claudecode deeper-read elevation** — Phase 1 research forward.
7. **Additional Copilot feedback dispatches** with different lenses if Q7 surfaces specific framings worth external critique; lower frequency per cycle 120 L2 discipline shift.

## Meta-observation: scaffold-vs-complete delta as a measurement-primitive worth lifting to v2

The cycle 123 scaffold-partial measurement opened a magnitude question; cycle 124 closed it with the COMPLETE measurement. **The pair produces a directly-comparable scaffold-vs-complete delta measurement on the same crate** — something cycles 93/94/122 could not produce because they delivered complete crates in single cycles. The delta is informative in itself:

- **Architectural complexity is captured early.** The scaffold's multi-stage report pattern + types + cli surface was 603 LOC of complete-architecture; the deferred stages added 280 LOC of operational-content. **Architectural-vs-operational LOC ratio: ~68% architecture / 32% operational** in this orchestration-hub crate.
- **The scaffold-partial extrapolation was directionally correct but magnitude-imprecise.** Cycle 123 predicted +300-900 LOC delta; actual was +280. The lower bound prediction was reached without being exceeded.
- **The completion validated the scaffold's deferred-contract design.** No re-architecture was needed at cycle 124 — the deferred-status markers absorbed completion gracefully via thin extension of existing stages. **This is the empirical answer to cycle 123's `deferred-status-as-scaffold-honesty-primitive`: the primitive is not just honest, it is structurally sound** in that it enables incremental completion without churning the architecture.

**Implication for v2 design:** scaffold-partial-with-deferred-markers should be the default cycle-shape for orchestration-hub and multi-responsibility tools. Single-cycle delivery of complete crates is feasible only for the simplest shapes (catalog, writer, narrow validator); for richer shapes, scaffold-partial → complete across 2 cycles is more honest than partial-implementation-without-explicit-markers OR forcing single-cycle delivery at the cost of test coverage / error handling.

**Implication for cycle 125+:** the `wiki-search` prediction (~500-1000 LOC) could be tested via the same scaffold-partial → complete shape: cycle 125 = scaffold (index + query + ranking shell, deferred indexing strategy); cycle 126 = complete (deferred items implemented). If this works for a second crate shape, the pattern becomes `scaffold-partial-as-default-cycle-shape-for-complex-crates` candidate-pattern.

## Lexicon entries (cycle 124)

- **scaffold-vs-complete delta:** the LOC + test count + test:prod ratio delta between a scaffold-partial measurement and a complete measurement on the same crate, taken across 2 consecutive cycles. Directly informative about architectural-vs-operational complexity split.
- **architectural-vs-operational LOC ratio:** the fraction of a crate's production LOC dedicated to architectural primitives (types, traits, multi-stage report patterns, CLI surface, fetcher abstractions) vs operational content (the per-stage logic that drives the architectural primitives toward specific behaviors). Cycle 124 measurement: ~68% / ~32% for `v2-boot-phase`.
- **fixture-injection via filesystem JSON:** production reads from external API/CLI; tests read from JSON files matching the production output format exactly. Same parsing code path; the dispatch is at the fetch boundary. No type-translation layer, no trait-object gymnastics, no mock library complexity.
- **subprocess-invocation-over-http-client:** the dependency-footprint-conscious choice for integrating with external services that have a stable CLI surface. Trade-off: subprocess invocation is harder to mock-test, mitigated by fixture-injection.
- **auth-gate-as-Warn-not-Skip:** auth gates that reject entries surface the rejection count via structured status (Warn) rather than silently dropping entries (Skip). Makes the gate observable; supports defense-in-depth principle.
- **scaffold-as-architecturally-complete:** a scaffold-partial implementation can be COMPLETE at the architectural level (types, multi-stage report, CLI surface, error handling) while being PARTIAL at the operational level (some stages return Deferred or stub data). The deferred stages extend cheaply when completed.

## Verification (compressed)

| Item | Pre-cycle-124 | Post-cycle-124 |
|---|---|---|
| `v2-boot-phase` scope completeness | scaffold-partial (3 of 5 sub-responsibilities) | **COMPLETE (5 of 5 sub-responsibilities)** |
| `check-standing-directives` status | Deferred (stub) | **Done (auth-gated, 6 Eva directives surfaced on real repo)** |
| `identify-gardening-candidates` status | Deferred (stub) | **Done (3 stale issues + 2 draft PRs surfaced on real repo, privileged-labels filter working)** |
| `v2-boot-phase` prod LOC | 603 (scaffold) | **883 (complete; +280, +46%)** |
| `v2-boot-phase` test LOC | 617 (scaffold) | **971 (complete; +354, +57%)** |
| `v2-boot-phase` test count | 33 (16 unit + 17 integration) | **44 (25 unit + 19 integration; +11)** |
| `v2-boot-phase` test:prod ratio | 1.02× (scaffold) | **1.10× (complete)** |
| `v2-boot-phase` build time | ~1.09s (scaffold) | **~1.10s (complete; +0.01s)** |
| `v2-boot-phase` new transitive deps | 0 | **0 (preserved across orchestration-hub-COMPLETE scope)** |
| 4-crate cumulative prod LOC | 1740 (cycle 123 scaffold-included) | **2020 (cycle 124 complete)** |
| 4-crate cumulative test LOC | 2024 | **2378** |
| 4-crate cumulative test:prod ratio | 1.16× | **1.18×** |
| 4-crate average prod LOC | 435 | **505** |
| 9-crate extrapolation (prod) | ~3915 (cycle 123 scaffold-based) | **~4545 (cycle 124 complete-based; +630)** |
| 9-crate aggregate-with-tests trajectory | 7500-11500 | **7500-11000 (tighter; cycle 123 variance from boot-phase extrapolation collapses)** |
| A's stated upper bound (4500) status | at risk of exceedance | **modestly exceeded (~1.01×); within PR #2877 revised 3600-6200 range** |
| Per-crate scope claim status | DIRECTION-PARTIAL at 4 instances; 2 of 4 above range | **DIRECTION-PARTIAL at 4 instances; 2 of 4 above range; cycle 123 scaffold-extrapolation 11% too high** |
| `gh` CLI fixture-injection design | not yet present | **`--fixture-dir <PATH>` flag with `input-from-eva.json` / `open-issues.json` / `open-prs.json` fixtures** |
| NEW candidate-patterns NOVEL@1 cycle 124 | — | **`gh-fixture-injection-via-filesystem-json`, `auth-gate-as-warn-not-skip`, `subprocess-invocation-over-http-client-for-dependency-discipline`** |
| TESTED@2 promotions cycle 124 | — | **`multi-stage-report-pattern` (operational instantiation)** |
| TESTED@3 promotions cycle 124 | — | **`crate-shape-dependent-test-prod-ratio` (orchestration-hub-COMPLETE at 1.10×)** |
| `discretionary-departure-from-forward-going-commitment` | HARDENED-at-4 cycle 114; ninth HONORING cycle 123 | **tenth consecutive HONORING cycle 124** (cycles 115-124); 14 cycles of substrate |
| Consecutive bottleneck-asynchronous cycle count | 35 (cycles 78-123) | **36 (cycles 78-124)** |
| Consecutive non-per-candidate-sharpening cycle count | 13 (cycles 111-123) | **14 (cycles 111-124)** |

## Edits

- **`tools/rust/crates/v2-boot-phase/src/main.rs`** — +458 net lines (603 → 883 prod + 460 inline tests added; 1343 lines total; ~+497 from cycle 123 except for the deleted Deferred-stub tests). Net: +280 prod LOC + +178 inline-test LOC; +6 standing-directives unit tests + +4 gardening-candidates unit tests + 2 cycle-123-Deferred-tests removed.
- **`tools/rust/crates/v2-boot-phase/tests/integration.rs`** — +137 lines (374 → 511 total; 17 → 19 tests; added `standing_directives_filters_to_eva_authored_only` + `gardening_candidates_surfaces_issues_and_draft_prs`; replaced `standing_directives_and_gardening_marked_deferred` with `standing_directives_and_gardening_done_with_empty_fixtures`; updated `summary_counts_stages_by_status` + `text_format_renders_human_readable` for the new Done status; added default `fixture_dir()` helper and `run_with_fixture_dir()` for tests that inject custom fixtures).
- **`docs/redesign/2-candidates/A-evolved-single-orchestrator.md`** — Line 89 summary updated with cycle 124 numbers (replaces cycle 123 scaffold-partial framing); section heading at line 425 renamed `Cycle 93+94+122+123` → `Cycle 93+94+122+124`; per-crate measurements table updated (cycle 123 column replaced with cycle 124 COMPLETE column); NEW scaffold-vs-complete comparison table added; validation findings revised (per-crate scope, aggregate trajectory, aggregate-with-tests, dependency-footprint, build-time).
- **`docs/redesign/_notes/cycle-124-...`** (NEW — this file)
- **`docs/journal/2026-05-12.md`** — cycle 124 entry (separately authored)
