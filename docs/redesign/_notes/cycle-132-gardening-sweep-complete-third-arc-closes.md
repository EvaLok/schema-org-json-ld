# Cycle 132 — `v2-gardening-sweep` COMPLETE: third scaffold→complete delta arc closes; magnitude precision claim refuted

**Date:** 2026-05-13
**Prompt:** `.github/workflows/orchestrator-redesign-prompt.xml` (redesign mode, cycle 132)
**Forward priority honored:** cycle 131 priority #3 (`v2-gardening-sweep` COMPLETE)

## Cycle composition

Cycle 132 promoted `v2-gardening-sweep` from SCAFFOLD-PARTIAL (cycle 131, 614 prod LOC / 447 test LOC) to COMPLETE (1532 prod LOC / 809 test LOC). All 10 sub-responsibilities deferred from cycle 131 are now implemented and tested. The cycle closes the third scaffold→complete delta arc (after boot-phase 123→124 and wiki-search 125→127), bringing the 6-crate measurement program to **6 of 9 crates at COMPLETE scope**.

The cycle is bottleneck-asynchronous (audit cycle 218 expected ~04:00 UTC ~3.5h post session-start; Q7 still Eva-blocked) and honors the cycle 131 named forward priority list — eighteenth consecutive cycle of HONORING (cycles 115-132).

## Forward-priority discipline

Cycle 131 listed forward priorities:
1. Audit cycle 218 critique absorption — NOT YET AVAILABLE (~3.5h pre-landing at session start)
2. Q7 resolution by Eva — Eva-blocked
3. **`v2-gardening-sweep` COMPLETE** — chosen as cycle 132 substantive focal
4-10. Other carried items

Priority #3 was the only available named priority at session-start.

## 10 sub-responsibilities implemented

| # | Sub-responsibility | Implementation | Test count |
|---|---|---|---|
| 1 | `multi-line-link-parsing` | `find_balanced_multiline` consumes across newlines; line number tracked from opening `[` via `build_line_offsets` + `lookup_line` | 1 integration + (covered by 4 unit tests) |
| 2 | `reference-style-links` | `parse_ref_defs` extracts `[ref]: url` lines; full `[text][ref]` / `[text][]` (collapsed) / `[text]` (shortcut) forms; new `LinkKind::MarkdownRef` variant; undefined refs surface with `[undefined-ref: name]` target | 2 integration + 4 unit |
| 3 | `html-comments-skipping` | `LineMask` tracks multi-line `<!-- ... -->` blocks | 1 integration + 2 unit |
| 4 | `indented-code-block-skipping` | `LineMask` flags lines with ≥4 leading spaces (non-blank) | 1 integration + 1 unit |
| 5 | `frontmatter-aware-skipping` | `LineMask` recognises top-of-file `---` to next `---`/`...` | 1 integration + 1 unit |
| 6 | `exclude-patterns` | Hand-written `glob_match` supporting `**` (cross-slash) + `*` (within-segment) + literal; `--exclude` CLI repeatable; excludes UNION across CLI + config | 2 integration + 4 unit (glob + should_exclude variants) |
| 7 | `dead-link-auto-fix-suggestions` | `levenshtein` distance over stem-index entries; `suggest_target` for markdown + `suggest_wiki_stem` for wiki; max edit distance 4; hand-written `pathdiff` for relative path output; `--no-suggest-fixes` opt-out | 2 integration + 2 unit |
| 8 | `stale-detection-by-content-hash` | `fnv1a_64` content hash; `HashState` JSON state file load/save; stale entries gain `hash_unchanged: Some(true)/Some(false)/None` distinguishing neglected vs changed vs first-run | 3 integration + 3 unit |
| 9 | `config-file-support` | `ConfigFile` (serde-deserialised JSON); CLI overrides config except `corpus` + `exclude` UNION | 2 integration |
| 10 | `detector-composition` | `--dead-links-only-on-stale-files` flag filters dead-link detection input to stale-flagged files only; report's `composition` field surfaces active mode | 1 integration |

**Test summary:** 37 unit tests (+25 vs cycle 131) + 39 integration tests (+16 vs cycle 131) = **76 tests passing; zero failures; zero clippy warnings under `-D warnings`; zero new transitive deps** (still using only `clap` + `serde` + `serde_json` + `tempfile` dev-dep).

## Bug surfaced + fixed during cycle 132

While integration-testing reference-style links, two unit tests failed (`extract_markdown_links_collapses_reference` + `extract_markdown_links_resolves_reference_style` — both expected 1 link but got 2). Root cause: the shortcut-reference branch was matching ref-def lines like `[ref1]: path.md` as shortcut references to themselves, because:

1. Parser hits `[docs][ref1]` → finds `]` → next char is `[` → reference link → resolves → adds 1 link
2. Parser continues past `[ref1]`
3. Later, parser hits `[ref1]:` → finds `]` → next char is `:` (neither `[` nor `(`) → falls into "shortcut reference" branch → text `ref1` is in ref_defs → adds 1 link

**Fix:** in the shortcut-reference branch, check that the char after `]` is not `:`. A single-byte guard at the right point in the state machine; one-line edit. Both failing tests now pass; both real-world ref-style markdown patterns (`[text][ref]` + `[text][]` collapsed) work correctly.

This bug would have only surfaced on real corpora containing reference-style links — the cycle 131 scaffold had no ref-style support so the bug was latent in cycle 132's new code path. Test-first surfacing prevented it from making it into the smoke-test phase.

## Smoke-test results

```
$ cargo run -p v2-gardening-sweep --release -- \
    --corpus docs/redesign/_notes \
    --stale-days 30 --format text
v2-gardening-sweep: 137 files scanned (0 excluded) across 1 root(s) (...)
## Dead links (9 entries)
```

**Identical to cycle 131's 9 findings against the same 137-file corpus.** The new features did not gain or lose findings on the `_notes/` corpus — they preserved scaffold behavior while adding capability surface. (The 137 vs cycle 131's 136 count reflects cycle 131's own notes file landing in the corpus between cycles.)

Extending corpus to include `docs/journal/` (214 files total) surfaces **32 additional dead-link findings** — historical references to `docs/worklog/*/*-cycle-NNN-summary.md` files from the pre-redesign journal format that have not been migrated, plus a few `../1-research.md` path-resolution errors and one absolute path to `/home/runner/.claude/...` (test-runner-only path leaked into a journal entry). These are real bugs the orchestrator can act on in cycle 133+ housekeeping.

The cycle 132 corpus-expansion finding extends the cycle 131 candidate-emergent observation `scaffold-already-produces-real-housekeeping-findings` — the COMPLETE crate continues to surface real housekeeping value on its second corpus expansion (cycle 131 found 9 on _notes; cycle 132 finds 41 on _notes + journal). Not yet promoted to candidate-pattern.

## Magnitude prediction outcome

**Cycle 131's prediction:**
- Delta: +300-600 LOC (medium-confidence)
- ±10% magnitude band: +275-660 LOC
- Direction: NEUTRAL (entropy-mitigation novel third shape)

**Cycle 132's actual:**
- Delta: **+918 LOC** (614 → 1532)
- ±10% upper bound was 660; actual is **+39% above the ±10% upper bound**
- ±50% upper bound (if applied) would be 900; actual is **+2% above the ±50% upper bound**
- Direction: **positive (CORRECT)**
- Magnitude: **SIGNIFICANTLY UNDERESTIMATED**

**Three-arc magnitude precision sequence:**
- Boot-phase 123→124: actual -7% below predicted lower band ← within ±10%
- Wiki-search 125→127: actual +7.6% above predicted upper band ← within ±10%
- Gardening-sweep 131→132: actual **+39% above ±10% upper band**, ~+53% above original upper of 600 ← **OUTSIDE ±10%; OUTSIDE ±30%**

The three-arc evidence refutes a flat ±10% magnitude precision claim for scaffold→complete predictions. The pattern is now:
- Direction prediction (positive delta): reliable at 3 of 3 arcs
- Magnitude prediction precision: **shape-dependent**, plausibly correlated with sub-responsibility count (boot-phase: 5; wiki-search: 4; gardening-sweep: 10) and shape-family novelty

**Pattern promotion:** `scaffold-partial-as-measurement-primitive` HARDENED@3 with explicit `direction-reliable / magnitude-shape-dependent` caveat. Future scaffold→complete predictions should:
1. Quote magnitude bands as shape-family-conditional, not flat ±10%
2. Acknowledge novel-shape uncertainty explicitly
3. Adjust expected band width with sub-responsibility count (more sub-responsibilities → wider band)

## 6-crate cumulative measurement (cycle 132 COMPLETE)

| Cycle | Crate | Prod LOC | Test LOC | Test:prod | Shape |
|---|---|---|---|---|---|
| 93 | `v2-tool-registry` | 231 | 347 | 1.50× | catalog-enumeration |
| 94 | `v2-cycle-history-append` | 268 | 510 | 1.90× | append-only-writer |
| 122 | `v2-phase-transition-check` | 638 | 550 | 0.86× | state-machine-validator |
| 124 | `v2-boot-phase` COMPLETE | 883 | 971 | 1.10× | orchestration-hub |
| 127 | `v2-wiki-search` COMPLETE | 931 | 1219 | 1.31× | top-k-retrieval |
| **132** | **`v2-gardening-sweep` COMPLETE** | **1532** | **809** | **0.53×** | **entropy-mitigation (NEW LARGEST)** |

**Cumulative metrics:**
- Sum prod LOC: 4483 (vs cycle 131's 3565; +26%)
- Mean prod LOC: 747 (vs cycle 131's 594; +26%)
- Sample standard deviation: **485 LOC** (vs cycle 131's 289; nearly doubled — variance is widening as the outlier emerges)
- 9-crate extrapolation (flat-mean): **6726 prod LOC** — **50% over A's 4500 ceiling**, **8% over PR #2877's 6200 upper bound** (first empirical-extrapolation overshoot of PR #2877's range)
- 6-crate cumulative test:prod: 4406 / 4483 = **0.98×** (DOWN from cycle 131's 1.13×, cycle 127's 1.22×)
- 9-crate extrapolation with tests: **~13310 total LOC** (+17% vs cycle 131's 11385)

**Shape-family-conditional refinement:** the flat-mean is biased upward by gardening-sweep being an outlier in the detector-walker family. Honest decomposition:
- detector-walker (gardening-sweep 1532, phase-transition-check 638): mean ~1085; predicted 2-3 of 9 = ~2170-3255 LOC
- non-detector (catalog 231, writer 268, orchestration 883, retrieval 931): mean ~578; predicted 6-7 of 9 = ~3468-4046 LOC
- Total band: **5638-7301 prod LOC** (vs flat-mean 6726 in the middle)

Both the flat-mean and shape-family-conditional methods now show **A's 4500 ceiling is materially below the empirical central estimate** for the 6-crate measurement program.

## Test:prod ratio trajectory across scaffold→complete arcs

| Arc | Crate | Scaffold test:prod | Complete test:prod | Direction |
|---|---|---|---|---|
| 123→124 | boot-phase | 1.07× | 1.10× | slightly UP (small prod delta) |
| 125→127 | wiki-search | 1.85× | 1.31× | DOWN (large prod delta outpaces test) |
| 131→132 | gardening-sweep | 0.73× | **0.53×** | **DOWN** (LARGEST prod delta) |

**Pattern subpattern observed:** when scaffold→complete prod delta is large (>50% of scaffold prod LOC), test:prod ratio drops because prod surface outpaces test surface growth (each new feature adds ~3-4 tests but ~60-100 prod LOC). Three-arc evidence supports the subpattern; second arc (wiki-search) and third arc (gardening-sweep) both show this pattern; first arc (boot-phase, small prod delta) does not. The subpattern is shape-independent within the three-arc evidence.

`crate-shape-dependent-test-prod-ratio` promoted to HARDENED@7 cycle 132 with `completeness-shifts-ratio-downward` subpattern.

## Pattern updates summary

- **`scaffold-partial-as-measurement-primitive`** (APPLIED-AT-THIRD-SCAFFOLD cycle 131) → **HARDENED@3 cycle 132** with **direction-reliable / magnitude-shape-dependent** caveat. Three arcs validate direction; magnitude precision varies ±10% to ±50% by shape.
- **`crate-shape-dependent-test-prod-ratio`** (TESTED@6 cycle 131) → **HARDENED@7 cycle 132** with `completeness-shifts-ratio-downward` subpattern.
- **`architectural-vs-operational-LOC-ratio`** (TESTED@2 cycle 127) → **TESTED@3 cycle 132**.
- **`subprocess-invocation-over-http-client-for-dependency-discipline`** (EXTENDED-TO-4-BOUNDARIES cycle 131) → **REINFORCED-AT-4-BOUNDARIES cycle 132**. Zero new transitive deps despite adding Levenshtein, glob, FNV hash, JSON config, state-file persistence.
- **NEW NOVEL@1 cycle 132 candidate-pattern `magnitude-prediction-precision-is-shape-dependent-not-flat`** — sibling to `calibration-extends-by-association` (cycle 130) and `judgment-range-can-envelope-empirical-truth` (cycle 130). Promotion path: TESTED@2 requires fourth scaffold→complete arc with shape-family that tests magnitude direction (e.g., another entropy-mitigation or detector-walker).
- **NEW candidate-emergent observation `9-crate-extrapolation-now-exceeds-pr-2877-upper-bound`** — first time empirical extrapolation has materially exceeded PR #2877's judgment-based 6200 upper bound. Confirms cycle 130 D4 absorption framing of empirical-vs-judgment evidence types; surfaces that PR #2877's lens-4 aggregate range, while internally consistent, was structurally too tight to envelope the empirical truth at the detector-walker shape family.
- **`discretionary-departure-from-forward-going-commitment`** HARDENED-at-4 → **eighteenth consecutive HONORING cycle 132** (cycles 115-132); 22 cycles of substrate.

## Forward work for cycle 133+

In rough priority order:
1. **Audit cycle 218 critique absorption** if landed (expected ~04:00 UTC; cycle 133 will find it ~03:30+ since cycle 132 ran 00:22-01:30 UTC).
2. **Q7 resolution by Eva** — Eva-blocked (carried).
3. **Audit#462 remaining substantive findings absorption** — 8 of 13 substantive findings remain (M1-M5 + P3-1/P3-2/P3-3/P3-4/P3-7/P3-8). M5 has fresh substrate via audit#463 question-for-eva on `recommendations.accepted` retention.
4. **`v2-close-phase` scaffold** — fourth scaffold→complete delta primitive. close-phase shape is orchestration-hub-like (per cycle 124's boot-phase precedent); flat-mean prediction ~750-900 LOC scaffold + ~280-450 LOC delta to complete; shape-family-conditional prediction closer to wiki-search (931 LOC) than gardening-sweep (1532 LOC) at COMPLETE.
5. **`v2-phase-transition-check` pre-existing test failures triage** — carried from cycle 122; 2 of 25 tests panic.
6. **`v2-gardening-sweep` housekeeping run** — orchestrator can act on the 41 dead-link findings; possible cycle 133-134 work.
7. **2-selection-summary calibrated-empirical migration cost row update** — cycle 132's measurement materially shifts the empirical anchor; cycle 130's calibrated-empirical row in `2-candidates/README.md` should be updated. Cycle 120 L2 constraint continues to apply to `2-selection.md` recursive annotation; calibrated rows in `2-candidates/README.md` and a footnote in `2-selection-summary.md` are acceptable formats.
8. **Symphony deeper-read elevation** + **oh-my-claudecode deeper-read elevation** — Phase 1 research forward.
9. **NO further recursive annotation of 2-selection.md** — cycle 120 L2 constraint continues.

## Honest characterization

Cycle 132 is a measurement-and-implementation cycle, NOT a settled deliverable. The key signals it produces:

1. **Substantive:** v2-gardening-sweep COMPLETE is now production-quality with 76 tests, zero warnings, zero new deps, handling the 10 most common false-positive sources for dead-link detection in real markdown corpora. It surfaces 41 real housekeeping findings on the actual repo corpus.

2. **Methodological refutation:** the ±10% magnitude precision claim from cycle 131's prediction is REFUTED at the third scaffold→complete arc. Future predictions should be shape-family-conditional, not flat-precision.

3. **Trajectory shift:** A's 4500 ceiling is now ~50% below the empirical central estimate; PR #2877's 6200 upper bound is also exceeded for the first time (by +8%). Q7's resolution surface is unaffected (Q7 stable across cycles 126-130 absorption arc), but the migration-cost evidence base shifts material toward the higher end.

4. **Pattern hardening:** three patterns hardened (`scaffold-partial-as-measurement-primitive`, `crate-shape-dependent-test-prod-ratio`, `subprocess-invocation-over-http-client-for-dependency-discipline`); two new patterns surfaced (`magnitude-prediction-precision-is-shape-dependent-not-flat`, `9-crate-extrapolation-now-exceeds-pr-2877-upper-bound`).

5. **No new questions for Eva.** Cycle 132 does not open new Eva-decisions; existing Q7 + audit#463 + question-for-eva #2903 (tools/dispatch-task allowlist) remain.

## Process honoring

- 18th consecutive cycle of HONORING named forward priority (cycles 115-132).
- 44th consecutive bottleneck-asynchronous cycle (cycles 78-132).
- 22nd consecutive non-per-candidate-sharpening cycle (cycles 111-132).
- Cycle 120 L2 constraint preserved (no recursive annotation of 2-selection.md beyond bounded log entries).
- Cycle 128 process-error lesson preserved (no parallel-batch cancellation cascade; HEREDOC commit messages via `.scratch/` inside working directory).
