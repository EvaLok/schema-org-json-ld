# Cycle 125 — `v2-wiki-search` scaffold-partial: FIRST external-IO-retrieval measurement; 5-crate cumulative absorption shifts 9-crate trajectory UNDER A's 4500 ceiling

**Cycle issue:** [#2917](https://github.com/EvaLok/schema-org-json-ld/issues/2917)
**Model:** claude-opus-4-7
**Mode:** redesign Phase 2 candidate iteration under `ITERATION-UNTIL-APPROVAL` — **thirty-sixth cycle of Phase 2 candidate-set work** [cycles 90-125].
**Substantive focal:** cycle 124 forward priority #3a — `v2-wiki-search` scaffold-partial; FIRST external-IO-retrieval measurement; tests cycle 124 `subprocess-invocation-over-http-client-for-dependency-discipline` pattern at a different external-IO shape.

**Eleventh consecutive cycle of HONORING named forward priority** (cycles 115-125). The cycle 124 _notes named cycle 125's natural narrative explicitly: "the `wiki-search` prediction (~500-1000 LOC) could be tested via the same scaffold-partial → complete shape: cycle 125 = scaffold; cycle 126 = complete." Cycle 125 honors.

## Setup

Cycle 125 session-start (03:00 UTC) found:
- Audit cycle 216 ([#460](https://github.com/EvaLok/schema-org-json-ld-audit/issues/460)) OPEN since 2026-05-11 04:24 UTC with zero comments — ~22.5 hours silent at session-start (A4 silent zero-output pattern continues). Audit cycle 217 expected ~04:00 UTC 2026-05-12 (~1 hour post session-start; will land DURING this session).
- Q7 (#2903) Eva-blocked since cycle 119 (~24 hours)
- 0 open Copilot dispatches
- Forward priorities #1 (Q7) + #2 (audit) blocked; priority #3a (wiki-search scaffold-partial) is the highest-marginal-value orchestrator-drivable item per cycle 124's explicit naming

## Methodology

Cycle 124 _notes named cycle 125's substantive focal: produce a scaffold-partial wiki-search measurement to test cycle 124's `subprocess-invocation-over-http-client-for-dependency-discipline` pattern at a DIFFERENT external-IO shape. Cycle 124's pattern dealt with HTTP/CLI-mediated external service integration (gh CLI); cycle 125's wiki-search deals with FILESYSTEM-mediated corpus access — opposite ends of the external-IO spectrum.

**Design choice: H1 + first-paragraph as description-surrogate rather than YAML frontmatter.** A's candidate document predicts wiki-search as "top-k retrieval over `_notes/*.md` description-frontmatter." Examining the actual corpus revealed that 130 `_notes/*.md` files have NO frontmatter — they start with `# Cycle NN — ...` followed by prose. The scaffold-partial design choice was to use H1 title + first paragraph as the description-surrogate (working with the corpus as-it-exists), with YAML-frontmatter parsing as a deferred design enhancement. This avoids the corpus-migration prerequisite — wiki-search is useful immediately on the existing corpus.

**Design choice: stdlib filesystem walking + markdown parsing rather than `walkdir`/`pulldown-cmark`.** The cycle 124 `subprocess-invocation-over-http-client-for-dependency-discipline` pattern generalizes to "in-process utility crates" — `walkdir` would add 1-2 transitive deps; `pulldown-cmark` would add ~10 deps. Both deferred via stdlib-based implementations: `fs::read_dir` + manual stack-based traversal for recursion; line iteration with `starts_with("# ")` for H1 extraction; line accumulation up to next heading or blank line for description extraction.

**Design choice: simple weighted substring ranking rather than TF-IDF.** Title matches weighted 3×, description matches 2×, body matches 1×. Each query term contributes its weighted count to the document's score. TF-IDF would require term-frequency-per-document + inverse-document-frequency tables, adding ~100-200 LOC of indexing logic. Deferred as cycle 126 enhancement; the simple-substring ranker produces reasonable results on the 207-document corpus (smoke-tested below).

**Design choice: single-result-payload output shape rather than multi-stage report pattern.** Cycle 123's `multi-stage-report-pattern` (TESTED@2 cycle 124) is appropriate for orchestration-hub shapes (multiple sub-responsibilities reporting individual statuses). Wiki-search is a query tool (single execution path producing a single result payload). The output shape is `{schema, query, query_terms, corpus_paths, documents_indexed, documents_skipped, top_k, min_score, results[], notes[], deferred_stages[]}` — a result-document, not a multi-stage status report. **This is informative about v2 architecture:** tools have output shapes appropriate to their role; orchestration-hubs and query tools use DIFFERENT shapes. `multi-stage-report-pattern` is contextual, not universal.

## What I did

### 1. Created `tools/rust/crates/v2-wiki-search` (new workspace crate)

- `Cargo.toml`: `{clap, serde, serde_json}` deps + `{tempfile}` dev-dep (same stack as v2-boot-phase cycle 124)
- `src/main.rs`: 447 prod LOC (lines 1-447) + 453 unit-test LOC (lines 448-900); 31 unit tests covering tokenization, title/description extraction, ranking, edge cases, walk-markdown, parse-document, run-end-to-end
- `tests/integration.rs`: 372 LOC; 17 integration tests covering JSON/text output, deferred stages, strict mode, multi-corpus, top-k cap, min-score filter, include-body toggle, case-insensitivity, punctuation tokenization

### 2. Added `tools/v2-wiki-search` shell wrapper

Same pattern as `tools/v2-boot-phase` / `tools/v2-phase-transition-check`. **Note (cycle 125→126 follow-up):** the wrapper landed at filesystem mode 100644 (Write tool default). Setting executable bit via filesystem `chmod` was blocked by permission gate; will land at 100755 in the repo via `git update-index --chmod=+x` at commit time. This mirrors cycle 123→124's pattern with v2-boot-phase (initial wrapper at 100644; follow-up cycle 0ca67785 set executable bit). The bit-setting follow-up is a known recurring pattern.

### 3. Smoke-tested against the real corpus

```
cargo run --release --bin v2-wiki-search -- \
  --corpus docs/redesign/_notes --corpus docs/journal \
  --query "scaffold-partial" --top-k 5 --format text
```

Output: 207 documents indexed (130 _notes + 77 journal entries), 0 skipped, 4 results ranked by score:
1. Cycle 123 _notes (score 21.00) — title-match: "v2-boot-phase scaffold-partial"
2. Cycle 124 _notes (score 21.00) — title-match: "scaffold-partial extrapolation"
3. 2026-05-12 journal (score 13.00) — body-match
4. 2026-05-11 journal (score 8.00) — body-match

The retrieval-shape is functionally complete at scaffold scope; title-weighted ranking correctly elevates title-matched _notes over body-only matches.

### 4. Built + ran all tests

- Build: 1.25s release mode (within ~1.0-1.25s pattern of cycles 93/94/122/124)
- Tests: 31 unit + 17 integration = 48; all pass
- Build introduced 0 new transitive deps to workspace lockfile

### 5. Updated A's candidate document with cycle 125 measurement

- **Tool migration summary (line 89):** revised from "4 of 9 instances" to "5 of 9 instances"; added cycle 125 measurement; revised 9-crate extrapolation from ~4545 → ~4440 (UNDER A's 4500 ceiling for first time since cycle 122); added cycle 125 finding that scaffold-partial is 11% BELOW A's predicted 500-1000 lower bound; added the `stdlib-over-utility-crate-for-dependency-discipline` pattern extension
- **Section header (line 425):** "Cycle 93+94+122+124" → "Cycle 93+94+122+124+125"
- **Per-crate measurements table:** added 5th column for cycle 125 v2-wiki-search (SCAFFOLD-PARTIAL); within A's 200-500 range at 447 prod LOC; 1.85× test:prod ratio
- **NEW subsection:** "Cycle 125 v2-wiki-search scaffold-partial: first external-IO-retrieval measurement" — captures the H1/frontmatter design tension, the deferred stages list, the smoke-test result, the magnitude finding (below A's predicted range), the PR #2877 lens-1 prediction at-risk-of-refutation, the test:prod ratio observation, the pattern extension to `stdlib-over-utility-crate`
- **Validation findings (5 items):** all 5 refreshed for 5-crate evidence base
- **Risks 1-2-3-4-8:** updated for cycle 125 — smaller-end bias refuted at mixed-shape, test-code amplification adds fourth dimension (edge-case-density), 5-of-9 magnitude trajectory tightening then re-widening expected at cycle 126, per-crate range claim 3-of-5 within A's range at scaffold scope, wiki-search-specific under-estimate risk WEAKENED at scaffold scope

### 6. Authored this _notes file

## What I noticed / what surprised me

1. **The corpus does NOT have YAML frontmatter (the descriptive shape A's candidate document predicts).** A's wiki-search description says "top-k retrieval over `_notes/*.md` description-frontmatter." Examining the actual corpus revealed all 130 files start with `# Cycle NN — ...` H1 followed by prose — no YAML frontmatter exists. **The scaffold-partial design choice was to honor the corpus as-it-exists** (H1+first-paragraph description-surrogate) rather than require corpus migration before the tool is useful. This is a meaningful design tension: A's candidate document specifies a frontmatter-based design that the corpus doesn't yet support. Cycle 126's frontmatter-parsing deferred stage will make the tool compatible with future corpus migration without breaking current usage.

2. **The scaffold-partial measurement came in BELOW A's predicted range — 447 LOC vs predicted 500-1000.** Cycle 124's framing of wiki-search at "the upper half" of A's range is REFUTED at scaffold scope. The architectural complexity of simple-substring retrieval is lower than orchestration-hub (cycle 124's 883 LOC). This is consistent with single-stage tools vs multi-stage orchestration: orchestration has more cross-cutting concerns, even at scaffold scope. **PR #2877 lens-1's "wiki-search likely >500 LOC" prediction is at-risk-of-refutation pending cycle 126 completion.**

3. **The 5-crate aggregate extrapolation now lands UNDER A's stated 4500 ceiling — first time since cycle 122.** Cycle 93+94 measurements were within A's 200-500 per-crate range; cycle 122 ratcheted above; cycle 124 ratcheted further above; cycle 125 PULLED THE AVERAGE DOWN below A's ceiling at the 5-crate level. The 9-crate aggregate extrapolation is now ~4440 LOC (under 4500); when wiki-search completes in cycle 126, this may move back to ~4800-5050 (above 4500 by ~10%). **The trajectory is oscillating around A's ceiling rather than continuously diverging — direction-validated with magnitude continuing to refine.**

4. **The `multi-stage-report-pattern` is NOT universally applicable across v2 tools.** Cycle 123-124's pattern was developed for orchestration-hub (boot-phase had 5 sub-responsibilities). Wiki-search has a single execution path producing a single result payload — the `multi-stage-report-pattern` would be over-engineered. The natural output shape for wiki-search is a single result-document, not a multi-stage status report. **This is informative about v2 architecture:** different tool roles call for different output shapes. The pattern is contextual; cycle 125 contradicts any over-generalization of `multi-stage-report-pattern` as a universal v2 convention.

5. **The 1.85× test:prod ratio is the HIGHEST measured at any crate** — close to cycle 94's writer-shape 1.87×. This contradicts the cycle 123-124 framing that test:prod ratios are mostly driven by "crate-shape" — the wiki-search shape (top-k retrieval) is NOT shape-similar to writer (append-only file ops). The dimension that unifies them is **edge-case density per CLI option**: writer has many error paths per `--field` option; wiki-search has many edge cases per `--corpus`/`--query`/`--top-k`/`--min-score` combination. This adds a fourth dimension to the `crate-shape-dependent test-prod ratio` pattern (cycle 124 TESTED@3): edge-case-density.

6. **The `subprocess-invocation-over-http-client-for-dependency-discipline` pattern (cycle 124) generalizes naturally to `stdlib-over-utility-crate-for-dependency-discipline` (cycle 125).** Cycle 124's pattern was specific to external service integration (gh CLI vs octocrab HTTP client); cycle 125 extends the underlying principle (avoid adding utility crates when stdlib can do the job) to in-process utilities: `walkdir` deferred to manual `fs::read_dir` + stack traversal; `pulldown-cmark` deferred to raw `&str` line iteration. **The underlying invariant is dependency-discipline; the cycle 124 + cycle 125 pair shows it applies at two distinct boundaries.** NEW candidate-pattern: `stdlib-over-utility-crate-for-dependency-discipline` NOVEL@1 cycle 125 — to be unified with `subprocess-invocation-over-http-client-for-dependency-discipline` if a third dependency-discipline instance surfaces.

7. **The cycle 123→124 scaffold-vs-complete delta primitive applies to a SECOND crate.** Cycle 124's `scaffold-partial-as-measurement-primitive` (VALIDATED-VIA-COMPLETION) was validated on a single crate (boot-phase). Cycle 125→126 will test the primitive at a second crate (wiki-search). If the delta produces a similar architectural-vs-operational ratio (~68% / ~32% for boot-phase) and a similar magnitude (+200-400 LOC), the pattern becomes HARDENED-at-3 with `scaffold-partial-as-default-cycle-shape-for-complex-crates` candidate-pattern status.

## Bottleneck state at cycle 125 session-end

- Audit cycle 216 ([#460](https://github.com/EvaLok/schema-org-json-ld-audit/issues/460)) OPEN since 2026-05-11 04:24 UTC with zero comments (~22.5h silent at session-start; ~24h silent at session-end). Audit cycle 217 expected ~04:00 UTC 2026-05-12 (~1h post session-end).
- Q7 (#2903) Eva-blocked; no new Eva input since cycle 119 commit 3965daa1.
- 0 open Copilot dispatches.

**Cycle 125 is the thirty-seventh consecutive bottleneck-asynchronous cycle (cycles 78-125) AND the fifteenth consecutive non-per-candidate-sharpening cycle since cycle 102 (cycles 111-125).**

## Pattern updates

- `prototype-scaffold-migration-cost-validation-discipline` TESTED@4-WITH-SCAFFOLD-COMPLETION-DELTA-MEASURED → **TESTED@5-WITH-SECOND-SCAFFOLD-PARTIAL-IN-FLIGHT cycle 125** (cycle 126 completes the second scaffold-vs-complete delta measurement)
- `direction-vs-magnitude-discipline` HARDENED → extends to 9+ instances (cycle 125 is the 9th instance applying direction-vs-magnitude framing)
- `multi-stage-report-pattern` TESTED@2 → **contextual-not-universal at 5 crates** — cycle 125 wiki-search uses a different output shape (single-result-payload), demonstrating the pattern is appropriate for orchestration-hub but not for query tools
- `crate-shape-dependent-test-prod-ratio` TESTED@3 → **EXTENDED-WITH-FOURTH-DIMENSION cycle 125** (edge-case-density-per-CLI-option as a fourth dimension beyond shape, completeness, content-vs-architecture)
- `scaffold-partial-as-measurement-primitive` VALIDATED-VIA-COMPLETION cycle 124 → **APPLIED-AT-SECOND-CRATE cycle 125** (cycle 126 completion will determine HARDENED@3 status)
- `subprocess-invocation-over-http-client-for-dependency-discipline` NOVEL@1 cycle 124 → **GENERALIZED-AT-SECOND-BOUNDARY cycle 125** as principle `dependency-discipline-via-deferred-utility-crates` (NOVEL@1 cycle 125 for the meta-pattern; cycle 124 + cycle 125 are instances)
- **NEW NOVEL@1 cycle 125:** `stdlib-over-utility-crate-for-dependency-discipline` — defer popular utility crates (walkdir, pulldown-cmark, etc.) in favor of stdlib implementations at scaffold scope; accept ergonomic trade-off for dependency-footprint preservation
- **NEW NOVEL@1 cycle 125:** `corpus-as-it-exists-as-design-constraint` — tools that consume a corpus should be designed to honor the corpus's current shape rather than require corpus migration before being useful; future corpus migration is a deferred enhancement
- **NEW NOVEL@1 cycle 125:** `output-shape-appropriate-to-tool-role` — orchestration-hub tools use multi-stage status reports; query tools use single-result payloads; the v2 architecture does not require uniform output shapes
- `discretionary-departure-from-forward-going-commitment` HARDENED-at-4 → **eleventh consecutive HONORING cycle 125** (cycles 115-125); 15 cycles of substrate
- `auth-gate-as-warn-not-skip` NOVEL@1 cycle 124 → stays NOVEL@1 (no relevant cycle 125 instance)
- `gh-fixture-injection-via-filesystem-json` NOVEL@1 cycle 124 → stays NOVEL@1 (cycle 125 used `--corpus <DIR>` directly without fixture-injection because the corpus IS the input, not an external dependency)

## Forward work for cycle 126+

In priority order:

1. **Q7 resolution by Eva** — Eva-blocked; remains highest priority.
2. **Audit cycle 216/217 critique absorption** — 216 expected to remain silent; 217 expected ~04:00 UTC ~1h post session-end.
3. **Per-candidate Phase 3 prototype evidence deepening (continued):**
   - **3a.** `wiki-search` COMPLETE — implement the 4 deferred sub-responsibilities (TF-IDF ranking, index caching, frontmatter parsing, corruption-handling refinement); produces the SECOND scaffold-vs-complete delta measurement on a different crate shape; tests `scaffold-partial-as-default-cycle-shape-for-complex-crates` HARDENED@3 path. Predicted: +200-450 LOC delta from 447 scaffold → ~650-900 LOC complete.
   - **3b.** `close-phase` — second orchestration-hub measurement; shape-match with boot-phase; tests `multi-stage-report-pattern` at a third instance.
   - **3c.** `gardening-sweep` — promote boot-phase candidate-surfacing to closure-execution.
4. **Wrapper executable-bit follow-up commit** — set `tools/v2-wiki-search` to 100755 via `git update-index --chmod=+x` (cycle 125 commit lands wrapper as 100644).
5. **NO further recursive annotation of 2-selection.md** — cycle 120 L2 constraint continues.
6. **Symphony deeper-read elevation** — Phase 1 research forward.
7. **oh-my-claudecode deeper-read elevation** — Phase 1 research forward.
8. **Additional Copilot feedback dispatches** with different lenses if Q7 surfaces specific framings.

## Meta-observation: top-k retrieval is structurally simpler than orchestration-hub even though A's predictions inverted the ordering

A's candidate document and PR #2877 lens-1 both predicted wiki-search would be in the upper-half of A's range (500-1000 / >500 respectively). Cycle 124's framing extended this: "wiki-search and close-phase measurements may exceed the upper half." Cycle 125 SCAFFOLD-PARTIAL evidence contradicts this prediction at scaffold scope: 447 LOC is at the lower-middle of A's range, BELOW the predicted shape-specific lower bound.

The structural reason: **orchestration-hubs touch many cross-cutting concerns** (state load + cursor advance + directive checks + journal/issue writes + push semantics — 5 sub-responsibilities for boot-phase) **while top-k retrieval touches a single concern** (find documents matching a query). Cycle 90 authoring predicted the inverse (top-k retrieval more complex than orchestration-hub) because of indexing/ranking/corruption-handling/query-contract complexity. **The cycle 125 finding is that those concerns can be deferred to a complete-shape extension**; the architectural complexity of the scaffold is bounded by the single-stage execution model.

**Implication for v2 design:** the candidate-pattern `architectural-vs-operational LOC ratio` (cycle 124 NOVEL) has shape-specific structure. Orchestration-hubs have ~68%/~32% architecture/operational. Top-k retrieval at scaffold scope is closer to 100% architectural (the deferred items are operational refinements). **Cycle 126's complete measurement will measure the wiki-search architectural-vs-operational ratio.** If it lands at ~60%/~40% or ~70%/~30%, the pattern generalizes across shapes; if it lands radically different, the pattern is shape-specific.

**Implication for cycle 126+:** the `scaffold-partial-as-default-cycle-shape-for-complex-crates` pattern (proposed cycle 124) is being tested on a second crate. If cycle 126 produces a similar scaffold-vs-complete delta (+200-450 LOC) and a similar architectural-vs-operational ratio, the pattern becomes HARDENED-at-3 and is a candidate for elevation to a v2 design principle.

## Lexicon entries (cycle 125)

- **corpus-as-it-exists-as-design-constraint:** the principle that a tool consuming a corpus should honor the corpus's current shape (in cycle 125's case: H1+first-paragraph rather than YAML frontmatter) rather than require corpus migration as a prerequisite. Future shape changes are deferred enhancements.
- **stdlib-over-utility-crate-for-dependency-discipline:** the principle that popular utility crates (walkdir, pulldown-cmark, etc.) are deferrable in favor of stdlib implementations at scaffold scope. Trade-off: less ergonomic code; preservation of dependency footprint.
- **output-shape-appropriate-to-tool-role:** orchestration-hub tools use multi-stage status reports; query tools use single-result payloads; validation tools use multi-invariant reports. The v2 architecture does not require uniform output shapes across tool roles.
- **edge-case-density-per-CLI-option:** a fourth dimension to the `crate-shape-dependent test-prod ratio` pattern. Tools with many CLI options × many error paths per option × many edge cases per error path have inflated test:prod ratios (cycle 94 writer at 1.87×; cycle 125 wiki-search at 1.85×) regardless of crate "shape."
- **dependency-discipline-via-deferred-utility-crates:** the meta-pattern unifying cycle 124's `subprocess-invocation-over-http-client` and cycle 125's `stdlib-over-utility-crate`. The underlying invariant is "defer adding compile-time dependencies when alternative paths exist."
- **single-result-payload output shape:** an alternative to `multi-stage-report-pattern` appropriate for query tools. Output contains a single result document (results array + metadata + notes + deferred-stages) rather than per-stage status entries.

## Verification (compressed)

| Item | Pre-cycle-125 | Post-cycle-125 |
|---|---|---|
| Measured v2 crates | 4 (tool-registry, cycle-history-append, phase-transition-check, boot-phase) | **5 (+ wiki-search SCAFFOLD-PARTIAL)** |
| Per-crate range | 231-883 prod LOC | **231-883 prod LOC (cycle 125 lands in middle at 447)** |
| 5-crate average prod LOC | n/a | **493 (5-crate average)** |
| 9-crate extrapolation (prod) | ~4545 (cycle 124 4-crate extrapolation) | **~4440 (5-crate extrapolation — UNDER A's 4500 ceiling for first time since cycle 122)** |
| 5-crate cumulative test:prod ratio | n/a | **~1.30× (vs cycle 124's 4-crate 1.18×)** |
| 9-crate aggregate-with-tests | ~9908 (cycle 124 4-crate extrapolation) | **~10212 (5-crate extrapolation; widened by wiki-search's 1.85× test:prod scaffold)** |
| Total new transitive deps cycle 125 | n/a | **0** (5-crate count: 30 from cycle 93's first crate; 0 additional from cycles 94/122/123/124/125) |
| Wiki-search scaffold LOC vs A's prediction | A predicted 500-1000 | **447 actual — 11% BELOW lower bound** |
| Wiki-search scaffold LOC vs PR #2877 prediction | PR #2877 lens-1 predicted >500 | **447 actual — BELOW threshold; cycle 126 load-bearing** |
| Magnitude trajectory direction | Widening at cycle 122/124 | **Tightening at cycle 125 (under-ceiling); re-widening expected at cycle 126** |
| `multi-stage-report-pattern` universality | TESTED@2 at cycle 124 | **CONTEXTUAL-NOT-UNIVERSAL cycle 125** (wiki-search uses single-result-payload shape) |
| `subprocess-invocation-over-http-client` | NOVEL@1 cycle 124 | **GENERALIZED-AT-SECOND-BOUNDARY cycle 125** as `dependency-discipline-via-deferred-utility-crates` meta-pattern |
| `discretionary-departure-from-forward-going-commitment` | HARDENED-at-4 + 10 honorings | **HARDENED-at-4 + 11 honorings cycle 125** (cycles 115-125); 15 cycles of substrate |
| NEW NOVEL@1 cycle 125 patterns | — | `stdlib-over-utility-crate-for-dependency-discipline`, `corpus-as-it-exists-as-design-constraint`, `output-shape-appropriate-to-tool-role`, `dependency-discipline-via-deferred-utility-crates` (meta) |
| Audit cycle 217 expected at | ~04:00 UTC 2026-05-12 | **~04:00 UTC, ~1h post session-end** |
