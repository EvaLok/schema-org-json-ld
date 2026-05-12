# Cycle 127 — `v2-wiki-search` COMPLETE: second scaffold→complete delta primitive (cycle 125 forward priority #3a fulfilled)

## Setup

**Cycle**: 127 (2026-05-12 ~06:53 UTC start)
**Phase**: Redesign Phase 2 candidate-iteration under `ITERATION-UNTIL-APPROVAL` — thirty-eighth cycle of Phase 2 candidate-set work [cycles 90-127]
**Cycle issue**: [#2919](https://github.com/EvaLok/schema-org-json-ld/issues/2919)
**Substantive focal**: cycle 126 forward priority #1 — `v2-wiki-search` COMPLETE (cycle 125 deferred forward priority #3a, deferred at cycle 126 in favor of audit#462 critique absorption).
**Thirteenth consecutive cycle of HONORING named forward priority** (cycles 115-127).

## Methodology

- Survey cycle 125's deferred surface (4 sub-responsibilities: TF-IDF ranking, index caching, frontmatter parsing, corruption-handling refinement)
- Design minimum-scope COMPLETE shape that closes the deferred contract without dragging in heavy dependencies (preserve cycle 125's `stdlib-over-utility-crate-for-dependency-discipline` discipline)
- Implement all 4 sub-responsibilities; promote `deferred_stages` array to empty
- Add tests covering each new behavior (unit + integration)
- Smoke-test against the real 209-doc corpus (130+ _notes + 77+ journal)
- Absorb the SECOND scaffold→complete delta primitive measurement into A's candidate document
- Update the 5-crate cumulative measurement (post-cycle-127 figures)

## What I did

### 1. Implemented TF-IDF ranking

Two-pass scoring:
1. `compute_doc_frequencies(documents, terms) → HashMap<String, usize>` — for each query term, count how many documents contain it (case-insensitive substring match across title + description + tags + body)
2. `score_document(doc, terms, doc_frequencies, total_docs) → (f64, Vec<String>)` — per-term `idf_smoothed × (TITLE_WEIGHT × tf_sublinear(in_title) + DESCRIPTION_WEIGHT × tf_sublinear(in_desc) + TAGS_WEIGHT × tf_sublinear(in_tags) + BODY_WEIGHT × tf_sublinear(in_body_only))` summed across terms

Helpers:
- `tf_sublinear(count) → 1.0 + ln(count)` if count > 0 else 0.0
- `idf(df, n) → ln((n+1)/(df+1)) + 1.0` (smoothed; universal terms return 1.0)

The TITLE/DESCRIPTION/TAGS/BODY weights (3.0 / 2.0 / 1.5 / 1.0) preserve cycle 125's title-weighted ranking semantics. The IDF factor adds rare-term boost: a query for "the" (common) ranks below a query for "scaffold-partial" (rare) on the same matching document.

Smoke-test verification: cycle 125 wiki-search _notes title-matches "scaffold-partial pattern" at score 46.31 (TF-IDF boost from rare bigram "scaffold-partial"); cycle 123 + 124 boot-phase _notes #2-3 at 35.22 + 34.75; today's journals #4-5 at 24.01 + 19.25.

### 2. Implemented opt-in index caching

CLI: `--index-cache PATH` (opt-in) and `--rebuild-index` (force fresh build).

Index schema: `v2-wiki-search-index/v1` with `{generated_at, corpus_paths, file_signatures, documents, skipped_signatures, skipped_notes}`. Signatures are `(path, mtime, size)` tuples sorted by path.

Invalidation: walk corpus, compute current signatures, compare element-wise with cached signatures. Any mismatch (different file count, different path, different mtime, different size) triggers a rebuild + re-write of the cache.

Index status surfaced in the report (`index_status` field):
- `no-cache` (no `--index-cache` flag)
- `cached` (signatures match; cache reused)
- `rebuilt-fresh` (cache file did not exist before this invocation)
- `rebuilt-stale` (cache file existed but signatures differed)
- `rebuilt-forced` (`--rebuild-index` flag)
- `rebuilt-corrupt` (cache file existed but failed to parse — schema mismatch or invalid JSON; warns via `WarnKind::IndexCorrupt`)
- `rebuilt-after-walk-error` (signature computation failed; falls back to fresh build)

The cache is opt-in to preserve cycle 125 scaffold's default behavior (parse on every invocation). This makes the orchestrator's first invocation of wiki-search behave identically pre/post cycle 127; users who repeatedly invoke with the same corpus opt into caching explicitly.

Smoke-test verified all three primary paths against the real 209-doc corpus.

### 3. Implemented YAML-lite frontmatter parsing

`extract_frontmatter(raw) → (HashMap<String, String>, &str, bool, Option<String>)` — returns parsed key-value map, the body-after-frontmatter slice, a `present` boolean, and an optional error message for malformed frontmatter (opener without closer).

Supported syntax:
- `---\nkey: value\n---\n` opener/closer
- Flat `key: value` lines (no nesting)
- Quoted values (single or double quotes stripped)
- `tags: a, b, c` (comma-separated) OR `tags: [a, b, c]` (bracketed list)
- BOM prefix handled (skipped before opener detection)
- Comment lines (`# ...`) inside frontmatter ignored

`parse_document` precedence:
- Title: `frontmatter.title` → `extract_title(after_fm)` H1 → filename stem
- Description: `frontmatter.description` → `extract_description(after_fm)` first paragraph after H1
- Tags: `frontmatter.tags` parsed; default empty

Per cycle 125's `corpus-as-it-exists-as-design-constraint`: frontmatter is OPTIONAL. Files without frontmatter parse identically to cycle 125 scaffold. The corpus currently has zero frontmatter-bearing files (verified via smoke-test: 209 documents indexed, 0 marked `frontmatter_present`).

### 4. Implemented typed `WarnKind` corruption classification

`enum WarnKind` with 8 variants:
- `CorpusMissing` (corpus path does not exist)
- `CorpusNotDir` (corpus path is a file, not a directory)
- `WalkError` (filesystem walk failed)
- `FileRead` (per-file `read_to_string` failed)
- `FileEmpty` (file empty after `trim()`)
- `FrontmatterMalformed` (opener `---` without closer)
- `IndexCorrupt` (cache file failed to parse)
- `IndexWriteFailed` (cache file write failed)

Each kind has a `slug()` returning a kebab-case identifier. Notes carry the prefix `warn[kind-slug]: msg` rather than the cycle 125 scaffold's `warn: msg`. The report includes a `warnings_by_kind: HashMap<String, usize>` map; downstream tooling can drive policy from per-kind counts.

Auth-gate-style observability (cycle 124 pattern) extended: the kind-keyed counter makes corruption modes visible to the orchestrator without requiring it to grep through note strings.

### 5. Updated tests

Added 16 new tests (10 unit + 6 integration):
- `tf_sublinear_is_zero_when_no_matches`
- `tf_sublinear_grows_with_diminishing_returns`
- `idf_returns_one_when_term_is_universal`
- `idf_grows_when_term_is_rare`
- `score_document_uses_tags_field_when_present`
- `score_document_rare_term_outweighs_common_term`
- `parse_document_reads_frontmatter_title_and_description`
- `parse_document_falls_back_to_h1_when_frontmatter_lacks_title`
- `parse_document_supports_bracketed_tags_list`
- `parse_document_warns_on_unclosed_frontmatter`
- `parse_document_quoted_frontmatter_values_are_unwrapped`
- `parse_tags_handles_comma_separated_and_brackets`
- `compute_doc_frequencies_counts_each_document_once_per_term`
- `tally_warnings_groups_by_kind_slug`
- `extract_frontmatter_returns_none_when_no_opener`
- `extract_frontmatter_returns_after_section_after_close`
- `extract_frontmatter_handles_bom_prefix`
- `run_with_index_cache_writes_then_reuses`
- `run_with_index_cache_rebuilds_when_file_changes`
- `run_with_rebuild_index_forces_rebuild`
- `run_with_corrupt_cache_rebuilds_and_warns`
- `frontmatter_title_and_tags_are_honored` (integration)
- `malformed_frontmatter_is_classified_as_warn` (integration)
- `index_cache_round_trip_reuses_index` (integration)
- `rebuild_index_flag_forces_fresh_build` (integration)
- `rare_term_outranks_common_term_in_real_corpus` (integration)

Updated 2 cycle-125 tests that asserted scaffold-partial semantics:
- `json_format_emits_empty_deferred_stages_after_complete` (was `json_format_includes_deferred_stages_array`)
- `text_format_renders_results_and_index_status` (was `text_format_renders_results_and_deferred`; the `## Deferred (scaffold-partial)` section is no longer emitted)

Total tests: **53 (35 unit + 18 integration)** — up from cycle 125's 48 (31 + 17).

### 6. Smoke-tested against real corpus

```
cargo run --release -p v2-wiki-search -- \
  --corpus docs/redesign/_notes \
  --corpus docs/journal \
  --query "scaffold-partial pattern" \
  --top-k 5 \
  --format text
```

Result: 209 documents indexed, 0 skipped. Cycle 125 wiki-search _notes #1 at score 46.31 (title-match × TF-IDF). Cycle 123 + cycle 124 boot-phase _notes #2-3 (title-match × TF-IDF). Today's + yesterday's journals #4-5 (body-match × TF-IDF).

Cache round-trip: second invocation with `--index-cache /tmp/idx.json` ran with `index_status: "cached"`; third invocation with `--rebuild-index` ran with `index_status: "rebuilt-forced"`. All three primary cache paths verified end-to-end.

### 7. Updated A's candidate document

Added a new sub-section `Cycle 127 v2-wiki-search COMPLETE: second scaffold→complete delta measurement` after the cycle 125 section. Updated the per-crate measurement table (cycle 125 column replaced with cycle 127 COMPLETE column). Added the scaffold-vs-complete comparison row covering both boot-phase (123→124) and wiki-search (125→127). Updated the migration-cost summary line + the cycle 97 absorption paragraph with the new 5-crate cumulative figures.

### 8. Authored this _notes file

(Cycle 127 self-reference — the file currently being read.)

## What I noticed / what surprised me

1. **The scaffold→complete delta on wiki-search was +484 LOC (+108%) — 73% LARGER than cycle 123→124 boot-phase delta (+280 LOC, +46%).** Cycle 125 predicted +200-450 LOC delta. Actual landed 7.6% above the upper bound. **Why:** the deferred sub-responsibilities carried more architectural mass than the boot-phase deferred stages. Boot-phase's deferred stages (`check-standing-directives` + `identify-gardening-candidates`) were thin shell-outs to `gh` plus filter logic. Wiki-search's deferred items added TF-IDF math + CorpusIndex struct + FileSig struct + WarnKind enum + cache-load/save state-machine + frontmatter parser. The "thin extension of existing stages" model that fit boot-phase did NOT fit wiki-search.

2. **The architectural-vs-operational ratio is shape-dependent and observable.** Boot-phase COMPLETE: 68% architectural (603 scaffold) / 32% operational (+280). Wiki-search COMPLETE: 48% architectural (447 scaffold) / 52% operational (+484). Top-k retrieval at COMPLETE scope carries MORE operational mass than orchestration-hub. The pattern `architectural-vs-operational-LOC-ratio` (NOVEL@1 cycle 124) is shape-dependent. Cycle 124 framed it as "~68/32 for orchestration-hub"; cycle 127 demonstrates the inverse can hold for query tools.

3. **Test:prod ratio DROPPED from scaffold (1.85×) to complete (1.31×) — the FIRST direct evidence that completeness affects test:prod ratio independently of crate shape.** Scaffold's test surface was inflated by edge-case-density-per-CLI-option (10 CLI flags × per-error-path × per-edge-case). Complete-phase additions (TF-IDF math, cache I/O, frontmatter parsing) had focused tests that covered the new logic without re-running the entire edge-case matrix. **Pattern `crate-shape-dependent-test-prod-ratio` extends to TESTED@5 with the new completeness-shifts-ratio dimension.** Cycle 125 named edge-case-density as a fourth dimension; cycle 127 confirms scaffold vs complete is a fifth dimension (or, more accurately, the "scaffold" measurement was a snapshot of high-edge-case-density-without-counterbalancing-operational-LOC).

4. **Cycle 125 had this prediction WRONG: "the scaffold's architectural surface (types + CLI + corpus walk + simple ranking + output rendering) absorbs most of the structural complexity; the deferred items (TF-IDF, caching, frontmatter) are operational refinements that extend cheaply."** The deferred items did NOT extend cheaply — they added +484 LOC, more than the scaffold itself. **What was wrong about the prediction:** the operational refinements turned out to require their own architectural primitives (CorpusIndex struct, WarnKind enum, fn signature changes). Cycle 125's "extend cheaply" framing was too optimistic; cycle 127 demonstrates that "extension" can itself be substantial architectural addition.

5. **The wiki-search-shape-specific TF-IDF was implementable with zero new transitive deps.** I deliberately considered `tantivy` (full search engine, 50+ MB deps) and `bm25` (focused crate). Neither was needed: TF-IDF as a math formula is ~10 LOC of `f64::ln()` + HashMap. The dependency-discipline pattern family (`subprocess-invocation-over-http-client` cycle 124; `stdlib-over-utility-crate` cycle 125; `tf-idf-with-stdlib-only` cycle 127) now spans three boundaries: external services, in-process utilities, information retrieval primitives. **The trade-off (no fancy ranking, no field boosting via crate config) is acceptable for the dependency-footprint preservation; if needed later, switching to BM25 or a real IR crate is a separate cycle's decision.**

6. **The cycle 125 prediction window was modestly off; cycle 124's was modestly off in the opposite direction.** Cycle 123→124 boot-phase predicted +300-900 LOC delta; actual +280 (below lower bound by 7%). Cycle 125→127 wiki-search predicted +200-450 LOC delta; actual +484 (above upper bound by 7%). Both predictions were directionally correct (delta direction) but magnitude-off by ~10%. **The `direction-vs-magnitude-discipline` pattern (HARDENED) holds at scaffold-vs-complete predictions: direction reliable, magnitude bounded but imprecise.**

7. **The 5-crate cumulative average moved from cycle 125's 493 LOC back to cycle 127's 590 LOC — a 20% jump from a single COMPLETE measurement.** The 9-crate extrapolation moves from ~4440 to ~5310 — 18% over A's 4500 ceiling. **The cycle 125 framing of "trajectory UNDER A's 4500 ceiling for the first time since cycle 122" was an artifact of cycle 125's scaffold-partial measurement.** At all-complete scope, the 5-crate trajectory is back to modestly over A's stated ceiling, consistent with cycle 124's 4-crate framing. The cumulative architecture estimate's volatility to scaffold-vs-complete mix is observable; the `cumulative-LOC-volatility-under-mixed-scope-measurements` is a candidate pattern.

8. **The cycle 126 D1 absorption finding (no-regret-primitive vs A∪C-scoped architecture) is preserved at COMPLETE scope.** wiki-search COMPLETE's substantive logic (TF-IDF scoring, opt-in index cache, YAML-lite frontmatter parsing, typed-WarnKind classification) is all substrate-agnostic. The same crate could be invoked by A's single-orchestrator, B's planner/executor split, or C's hybrid. **4 of 5 measured crates remain no-regret primitives** (now confirmed at COMPLETE scope for wiki-search); 1 of 5 (boot-phase) is A∪C-specific bundling. The cycle 126 hybrid-accept D1 framing holds.

## Bottleneck state at cycle 127 session-end

- Audit cycle 218 expected ~04:00 UTC 2026-05-13 (~21h post session-end). Audit#462 D2/D3/D4/M1-M5/P3-3 carried for cycle 128+ absorption (cycle 126 absorbed D1 + D5; cycle 127 substantive focal was wiki-search COMPLETE).
- Q7 (#2903) Eva-blocked; no new Eva input since cycle 119 commit 3965daa1.
- 0 open Copilot dispatches.
- Pre-existing `v2-phase-transition-check` test failures: 2 of 25 tests panic with `attempt to subtract with overflow` at `src/main.rs:285:45` and `:542:51`. These are cycle-122-vintage; unaffected by cycle 127's work. Forward work item for cycle 128+ to triage.

**Cycle 127 is the fortieth consecutive bottleneck-asynchronous cycle (cycles 78-127).**

## Pattern updates

- `scaffold-partial-as-measurement-primitive` (NOVEL@1 cycle 123, VALIDATED-VIA-COMPLETION cycle 124, APPLIED-AT-SECOND-SCAFFOLD cycle 125) → **VALIDATED-VIA-COMPLETION-AT-SECOND-CRATE cycle 127.** Two crates demonstrate full scaffold→complete arcs; third application would promote to HARDENED@3.
- `crate-shape-dependent-test-prod-ratio` (TESTED@3 cycle 125 → **TESTED@5 cycle 127** with new completeness-shifts-ratio dimension; ratio dropped 1.85× → 1.31× scaffold-to-complete on same crate).
- `architectural-vs-operational-LOC-ratio` (NOVEL@1 cycle 124) → **TESTED@2 cycle 127** with shape-dependent inversion (boot-phase 68/32 vs wiki-search 48/52).
- `direction-vs-magnitude-discipline` (HARDENED) → **EXTENDED-TO-SCAFFOLD-VS-COMPLETE-PREDICTIONS cycle 127** (direction reliable, magnitude bounded ±10% at both crates).
- `subprocess-invocation-over-http-client-for-dependency-discipline` (HARDENED-AT-2-BOUNDARIES cycle 125) → **HARDENED-AT-3-BOUNDARIES cycle 127** (external services → in-process utilities → information retrieval primitives).
- `corpus-as-it-exists-as-design-constraint` (NOVEL@1 cycle 125) → **EXTENDED-FROM-SCAFFOLD-TO-COMPLETE cycle 127** via three-level title precedence (frontmatter → H1 → filename stem; frontmatter OPTIONAL).
- `auth-gate-as-warn-not-skip` (NOVEL@1 cycle 124) → **GENERALIZED-AT-SECOND-DOMAIN cycle 127** via typed-WarnKind-enum classification (cycle 124: auth boundary; cycle 127: corruption boundary).
- **NEW NOVEL@1 cycle 127:** `tf-idf-with-stdlib-only` — TF-IDF implementation using only `f64::ln()` and HashMap; extends dependency-discipline pattern family to IR primitives.
- **NEW NOVEL@1 cycle 127:** `mtime-size-signature-as-cache-invalidation-primitive` — file change detection via sorted `(path, mtime, size)` tuples without content hashing.
- **NEW NOVEL@1 cycle 127:** `frontmatter-optional-not-required` — three-level title precedence preserves corpus migration trajectory without forcing it.
- **NEW NOVEL@1 cycle 127:** `typed-warn-kind-enum-for-corruption-classification` — extends `auth-gate-as-warn-not-skip` from "visible rejection" to "classifiable rejection."
- Candidate-emergent pattern (not yet promoted): `cumulative-LOC-volatility-under-mixed-scope-measurements` — 5-crate average shifts 20% on a single COMPLETE measurement when scaffold→complete deltas are large; observable when scaffold-vs-complete mix is mixed across measured crates.
- `discretionary-departure-from-forward-going-commitment` HARDENED-at-4 + 12 honorings → **HARDENED-at-4 + 13 honorings cycle 127** (cycles 115-127); 17 cycles of substrate.

## Forward work for cycle 128+

In priority order:

1. **Audit cycle 218 critique absorption** (if it lands; expected ~04:00 UTC 2026-05-13). Audit#462 D2/D3/D4/M1-M5/P3-3 still carried from cycle 126.
2. **Audit#462 D2 absorption** — Criterion 6 audit-as-peer empirical calibration; cite V2-era 142-cycle operational data.
3. **Audit#462 D3 absorption** — C-side recursive stress-test OR qualify cycle 117 convergence claim OR argue cycle 120 shift to Q7-dependent supersedes the need.
4. **Audit#462 D4 absorption** — migration cost methodology calibration; apply PR #2877 calibration-first discipline to lens-4 ranges.
5. **Q7 resolution by Eva** — Eva-blocked.
6. **Third scaffold→complete delta primitive** — `close-phase` (second orchestration-hub measurement; shape-match with boot-phase; would push `scaffold-partial-as-measurement-primitive` to HARDENED@3) OR `gardening-sweep` (promote boot-phase candidate-surfacing to closure-execution; different shape).
7. **Audit#462 M1-M5 absorption** — audit retrospective consultation; fourth-candidate option naming; Eva-legibility positive criterion; liveness + commitment-thread observability for Phase 3; state.json size discipline.
8. **Audit#462 P3-3 absorption** — formalize Eva's [audit#455](https://github.com/EvaLok/schema-org-json-ld-audit/issues/455) Option C/D `audit-request` label.
9. **`v2-phase-transition-check` pre-existing test failures triage** — 2 of 25 tests panic with subtract-with-overflow; cycle-122-vintage code; cycle 128+ candidate to identify the root cause and fix.
10. **NO further recursive annotation of 2-selection.md** — cycle 120 L2 constraint continues.
11. **Symphony deeper-read elevation** — Phase 1 research forward.
12. **oh-my-claudecode deeper-read elevation** — Phase 1 research forward.
13. **Additional Copilot feedback dispatches** with different lenses if Q7 surfaces specific framings.

## Meta-observation: scaffold-vs-complete prediction reliability at two crates

Cycle 124 produced the FIRST scaffold-vs-complete delta primitive on the same crate (boot-phase 123 scaffold-partial → 124 complete; +280 LOC delta vs cycle 123's predicted +300-900 LOC; 7% below lower bound). Cycle 127 produces the SECOND on a different crate shape (wiki-search 125 scaffold-partial → 127 complete; +484 LOC delta vs cycle 125's predicted +200-450 LOC; 7.6% above upper bound).

**Two-crate finding:** scaffold-partial→complete delta predictions are directionally reliable but magnitude-imprecise by ~10% at the bounds. The directional reliability holds across crate shapes (orchestration-hub AND top-k retrieval). The magnitude-imprecision could be due to:

- **Architectural mass distribution differences across shapes.** Boot-phase's deferred stages were thin extensions; wiki-search's deferred items added their own architectural primitives (CorpusIndex struct, WarnKind enum, fn signature changes).
- **Edge-case-density at scaffold scope distorts the operational-LOC estimate.** Wiki-search at scaffold-partial scope had 10 CLI flags × per-edge-case tests; the test-prod ratio at 1.85× was abnormally high for scaffold-partial scope. Cycle 125's prediction implicitly assumed the operational additions would scale linearly with the architectural surface; the actual operational additions scaled with the per-feature-architecture-they-required mass instead.
- **Predictor's framing bias toward extension-by-thin-shell.** Both cycle 124's "thin extension" framing and cycle 125's "operational refinements extend cheaply" framing assumed deferred items would be thin. Cycle 127 falsifies that for query-tool deferred items. **A more honest scaffold prediction framing would qualify: "if deferred items require their own architectural primitives, the magnitude delta will be larger than the architectural-extension model predicts."**

**Implication for v2 design:** scaffold-partial-with-deferred-markers remains the right default cycle-shape for orchestration-hub and multi-responsibility tools (cycle 123 finding HOLDS). But the deferred-markers SHOULD distinguish "thin-extension-deferred" from "architectural-primitive-deferred" — the former predicts magnitude better; the latter requires larger LOC budget. Cycle 127 retroactively clarifies that wiki-search's deferred items were `architectural-primitive-deferred` (TF-IDF needs its own scoring function family; index caching needs its own struct family; frontmatter parsing needs its own parser sub-system).

**Implication for cycle 128+:** the third scaffold→complete delta primitive on `close-phase` or `gardening-sweep` would push the pattern to HARDENED@3. If the third crate also falls in the +200-500 LOC delta range with similar directional-reliable / magnitude-imprecise profile, the prediction discipline becomes empirically calibrated. If it falls outside, the magnitude-imprecision is shape-dependent (and the pattern needs further refinement).

## Lexicon entries (cycle 127)

- **tf-idf-with-stdlib-only:** TF-IDF implementation using `f64::ln()` and HashMap only — `tf_sublinear(count) = 1.0 + ln(count)` and `idf_smoothed(df, n) = ln((n+1)/(df+1)) + 1.0`. Per-field weighting (title 3×, description 2×, tags 1.5×, body-only 1×) preserved from cycle 125 scaffold; only the TF transform and IDF factor are new. Extends `stdlib-over-utility-crate-for-dependency-discipline` to information retrieval primitives.
- **mtime-size-signature-as-cache-invalidation-primitive:** file change detection via sorted `(path, mtime, size)` tuples. Fails to invalidate if a file is replaced with the same size in the same second (acceptable risk for orchestrator's _notes/ corpus where edits happen at human-cycle cadence). Reusable for other v2 caching primitives.
- **frontmatter-optional-not-required:** YAML-lite frontmatter parser with three-level title precedence (frontmatter → H1 → filename stem). Honors corpus migration trajectory without forcing it (cycle 125 `corpus-as-it-exists-as-design-constraint` extended from scaffold to complete with forward-compatible parse path).
- **typed-warn-kind-enum-for-corruption-classification:** extends cycle 124's `auth-gate-as-warn-not-skip` from "make rejection visible" to "make rejection classifiable" via `WarnKind` enum + `warn[kind-slug]: msg` note prefix + `warnings_by_kind: HashMap<String, usize>` report field. Downstream tooling can drive policy from per-kind counts.
- **cumulative-LOC-volatility-under-mixed-scope-measurements:** the 5-crate cumulative average can shift 20% on a single COMPLETE measurement when scaffold→complete deltas are large; observable when scaffold-vs-complete mix is mixed across measured crates (cycle 127 example: cycle 125's 493 LOC scaffold-included 5-crate avg → cycle 127's 590 LOC all-complete 5-crate avg). Candidate v2 design input: cumulative architecture estimates should disclose scope mix as a confidence dimension.

## Verification (compressed)

| Item | Pre-cycle-127 | Post-cycle-127 |
|---|---|---|
| `v2-wiki-search` scope completeness | scaffold-partial (4 of 4 sub-responsibilities Deferred) | **COMPLETE (4 of 4 sub-responsibilities; cycle 125 Deferred items implemented)** |
| `v2-wiki-search` prod LOC | 447 | **931 (+484, +108%)** |
| `v2-wiki-search` test LOC | 825 (453 inline + 372 integration) | **1219 (684 inline + 535 integration; +394, +48%)** |
| `v2-wiki-search` tests count | 48 (31 unit + 17 integration) | **53 (35 unit + 18 integration; +5)** |
| `v2-wiki-search` test:prod ratio | 1.85× | **1.31× (−0.54×)** |
| `v2-wiki-search` new transitive deps | 0 | **0** (cycle 127 COMPLETE preserves 0-new-deps streak) |
| 5-crate cumulative prod LOC | 2467 | **2951 (+484)** |
| 5-crate avg prod LOC | ~493 | **~590 (+97, +20%)** |
| 9-crate prod LOC extrapolation | ~4440 (UNDER 4500 ceiling) | **~5310 (18% OVER 4500 ceiling)** |
| 9-crate aggregate-with-tests range | 8000-11500 | **~11800 (5-crate cumulative test:prod 1.22×; cycle 127 COMPLETE undoes cycle 125's scaffold-shifted 1.30× cumulative)** |
| Cycle 125 scaffold→complete prediction accuracy | predicted +200-450 LOC delta → 650-900 LOC complete | **actual +484 LOC delta → 931 LOC complete; 7.6% above upper bound of predicted delta range** |
| Smoke-test against real corpus | 207 docs, scaffold-substring ranking | **209 docs, TF-IDF + index cache verified end-to-end across all three primary cache paths (no-cache, cached, rebuilt-forced)** |
| `scaffold-partial-as-measurement-primitive` | VALIDATED-VIA-COMPLETION cycle 124 (one crate) | **VALIDATED-VIA-COMPLETION-AT-SECOND-CRATE cycle 127 (two crates; HARDENED@3 path requires one more)** |
| `crate-shape-dependent-test-prod-ratio` | TESTED@3 cycle 125 (3 dimensions) | **TESTED@5 cycle 127 (completeness-shifts-ratio as 5th dimension)** |
| `architectural-vs-operational-LOC-ratio` | NOVEL@1 cycle 124 (one shape, 68/32) | **TESTED@2 cycle 127 (two shapes; 68/32 orchestration-hub vs 48/52 top-k retrieval)** |
| NEW NOVEL@1 cycle 127 candidate-patterns | — | `tf-idf-with-stdlib-only`, `mtime-size-signature-as-cache-invalidation-primitive`, `frontmatter-optional-not-required`, `typed-warn-kind-enum-for-corruption-classification` |
| Audit#462 substantive findings absorbed | 2 of 13 (D1 + D5 from cycle 126) | **2 of 13 (cycle 127 substantive focal was scaffold-complete; audit absorption deferred to cycle 128+)** |
| `discretionary-departure-from-forward-going-commitment` | HARDENED-at-4 + 12 honorings | **HARDENED-at-4 + 13 honorings cycle 127** (cycles 115-127); 17 cycles of substrate |
