# Cycle 131 — `v2-gardening-sweep` scaffold: third scaffold→complete delta primitive begun; entropy-mitigation novel third shape

**Cycle:** 131 (2026-05-12 ~22:24 UTC start)
**Cycle issue:** [#2923](https://github.com/EvaLok/schema-org-json-ld/issues/2923)
**Mode:** redesign Phase 2 candidate iteration under `ITERATION-UNTIL-APPROVAL` — **forty-second cycle of Phase 2 candidate-set work** [cycles 90-131].
**Substantive focal:** cycle 130 forward priority #3 — third scaffold→complete delta primitive. Priority #1 (audit cycle 218 critique landing) not available — cycle 218 expected ~04:00 UTC 2026-05-13 (~5.5h post session-start; latest audit substrate is [#463](https://github.com/EvaLok/schema-org-json-ld-audit/issues/463) audit-side `question-for-eva` on `recommendations.accepted` retention shape). Priority #2 (Q7 resolution by Eva) still Eva-blocked.

**Seventeenth consecutive cycle of HONORING named forward priority** (cycles 115-131).

## Setup

Cycle 130 forward priority #3 was named as "Third scaffold→complete delta primitive — `close-phase` or `gardening-sweep` — would push `scaffold-partial-as-measurement-primitive` from VALIDATED-VIA-COMPLETION-AT-SECOND-CRATE (cycle 127) to HARDENED@3."

**Choice between `close-phase` and `gardening-sweep`:** gardening-sweep selected for three substrate reasons:

1. **Dependency ordering** — A's tool surface ([`2-candidates/A-evolved-single-orchestrator.md`](../2-candidates/A-evolved-single-orchestrator.md) line 75) names `close-phase` as "close-phase orchestration (gardening-sweep, cycle-history append, journal commit + push, cycle-issue close-comment)." close-phase USES gardening-sweep as a sub-step. Building gardening-sweep first avoids stub-then-replace work when close-phase is built later.

2. **Substrate-content-distinct for pattern hardening** — for `scaffold-partial-as-measurement-primitive` to harden to **HARDENED@3** rather than just TESTED@3, the third instance should be substrate-content-distinct from the prior two:
   - Cycle 123→124: `v2-boot-phase` (orchestration-hub shape; +280 LOC delta, +46%; 7% below predicted lower bound)
   - Cycle 125→127: `v2-wiki-search` (top-k-retrieval shape; +484 LOC delta, +108%; 7.6% above predicted upper bound)
   - **Cycle 131→132 candidate:** `v2-gardening-sweep` (entropy-mitigation shape) — substrate-content-distinct from both. `close-phase` (orchestration-hub shape) would echo cycle 124's substrate, reducing pattern-hardening evidence-strength.

3. **Leaf-level (no orchestration deps)** — gardening-sweep's dependencies are filesystem walk + markdown link parsing; no other v2-* crates required. Lower implementation risk; better fit for one-cycle scaffold scope.

## What I did

- Posted session-start comment on [#2923](https://github.com/EvaLok/schema-org-json-ld/issues/2923) naming gardening-sweep scaffold as substantive focal.
- Inspected [`v2-boot-phase/Cargo.toml`](../../../tools/rust/crates/v2-boot-phase/Cargo.toml) and [`v2-wiki-search/Cargo.toml`](../../../tools/rust/crates/v2-wiki-search/Cargo.toml) for dependency-discipline convention (clap + serde + serde_json + dev-dep tempfile, zero new deps).
- Inspected [`v2-wiki-search/tests/integration.rs`](../../../tools/rust/crates/v2-wiki-search/tests/integration.rs) for the `Command` + `binary_path()` test convention.
- **Built `tools/rust/crates/v2-gardening-sweep/`** with:
  - [`Cargo.toml`](../../../tools/rust/crates/v2-gardening-sweep/Cargo.toml): four production deps (clap, serde, serde_json) + one dev dep (tempfile) — **zero new transitive deps added to workspace** (cycle 131 extends cycle 127's dependency-discipline pattern from 5 to 6 boundaries).
  - [`src/main.rs`](../../../tools/rust/crates/v2-gardening-sweep/src/main.rs): **614 production LOC**. CLI with clap-derive (corpus paths, stale-days threshold, no-stale / no-dead-links toggles, extension filter, format json/text, output path, strict mode). Recursive directory walker. Stale-detection by file mtime. Dead-link detection for markdown `[text](path)` (with fragment stripping, external/anchor-only skip, fenced-code skip, inline-code-span skip) and wiki `[[name]]` references (resolved by stem-match across corpus). JSON output schema `v2-gardening-sweep/v1`. Text output for human-readable summaries.
  - [`tests/integration.rs`](../../../tools/rust/crates/v2-gardening-sweep/tests/integration.rs): **447 test LOC** across 23 integration tests covering empty corpus / stale detection (over and under threshold) / markdown dead links (with multiple shapes: simple, external, anchor-only, fragment-stripped, alive) / wiki dead links (with alias form) / inline-code skip / fenced-code skip / strict mode (with and without findings) / multiple corpus roots / text format / no-stale and no-dead-links toggles / recursive walk / non-markdown extension filter / custom extension / output-to-file. Plus **12 unit tests** in `src/main.rs` for the pure helper functions (link extraction, fragment stripping, wiki stem, external-link detection, file-age calculation).
- Ran `cargo test -p v2-gardening-sweep`: **35 tests pass, 0 failures, 0 warnings** (after removing one unused import in tests).
- **Smoke-tested against `docs/redesign/_notes` corpus** (136 files): found 0 stale entries (all files modified within 30 days — consistent with active redesign cycles) + **9 real dead-link findings**:
  - 4 cycle-* notes (cycles 93, 94, 95, 104) reference `../../.github/workflows/orchestrator-redesign-prompt.xml` with one fewer `..` than the file location requires (correct relative path from `docs/redesign/_notes/` is `../../../.github/workflows/`).
  - 2 lines in cycle-129 reference `../redesign/2-selection-summary.md` and `../redesign/2-selection.md` with an extra `redesign/` segment (correct relative path is `../2-selection-summary.md`).
  - 1 line in cycle-34 references `1-research/systems/voyager.md` (system was added to the inventory but deferred; the file does not exist).
  - 2 lines in cycle-93 reference historical `../../tools/rust/crates/v2-tool-registry/` and `../../tools/v2-tool-registry` paths that the cycle-93 notes describe but that resolve oddly from the file's location.
- Verified one finding manually: `grep -n "orchestrator-redesign-prompt.xml" docs/redesign/_notes/cycle-104-cross-cluster-intersection-updates.md` shows the link at line 5; `ls .github/workflows/orchestrator-redesign-prompt.xml` confirms the file exists from repo root; the dead-link finding is a real path-error in cycle-104.

**Cycle 131 LOC counts:**

| File | LOC |
|---|---|
| `src/main.rs` | 614 |
| `tests/integration.rs` | 447 |
| **Total** | **1061** |

**Test:prod ratio for cycle 131 scaffold:** 447 / 614 = **0.73×** — LOWER than cycle 125's 1.85× scaffold ratio and lower than the 5-crate cumulative 1.22×. This is informative about shape-dependent test surface variance: gardening-sweep's surface is dominated by detector orchestration + walker logic which has fewer per-feature CLI-option edge cases than wiki-search's TF-IDF + multi-corpus + frontmatter parsing surface.

## 6-crate cumulative measurement absorbed into A's candidate doc

| Cycle | Crate | Prod LOC | Test LOC | Test:prod ratio | Shape |
|---|---|---|---|---|---|
| 93 | `v2-tool-registry` | 231 | 347 | 1.50× | catalog-enumeration |
| 94 | `v2-cycle-history-append` | 268 | 510 | 1.90× | append-only-writer |
| 122 | `v2-phase-transition-check` | 638 | 550 | 0.86× | state-machine-validator |
| 124 | `v2-boot-phase` COMPLETE | 883 | 971 | 1.10× | orchestration-hub-complete |
| 127 | `v2-wiki-search` COMPLETE | 931 | 1219 | 1.31× | top-k-retrieval-complete |
| **131** | **`v2-gardening-sweep` SCAFFOLD-PARTIAL** | **614** | **447** | **0.73×** | **entropy-mitigation-scaffold (NOVEL THIRD SHAPE)** |

**6-crate cumulative measurement (cycle 131):**
- Sum prod LOC: 231 + 268 + 638 + 883 + 931 + 614 = **3565**
- Mean prod LOC: 3565 / 6 = **~594** per crate
- 9-crate extrapolation: 594 × 9 = **~5346 prod LOC** (vs cycle 130's 5310; +0.7% shift)
- Sample standard deviation: ~289 LOC (substantial per-crate variance; cycle 131's 614 sits near the 6-crate mean)
- 6-crate cumulative test:prod ratio: 4044 / 3565 = **~1.13×** (DOWN from cycle 127's 5-crate 1.22×; scaffold's lower test:prod ratio shifts the cumulative downward; will rebound at cycle 132 COMPLETE)
- 9-crate extrapolation with tests: 5346 × (1 + 1.13) = **~11385 total LOC** (vs cycle 127's 11800; -3.5% shift)

**Position vs A's ceiling and PR #2877's range:** 9-crate extrapolation 5346 prod LOC is ~19% over A's 4500 ceiling and within the upper half of PR #2877's 3600-6200 judgment-based range (~84% through the range). Cycle 131's addition (a 614-LOC scaffold near the 6-crate mean) does NOT materially shift the trajectory.

## Scaffold→complete delta prediction for cycle 132+

Based on the two prior scaffold→complete arcs (boot-phase 123→124 +280 LOC, wiki-search 125→127 +484 LOC), and gardening-sweep's structural shape:

**Predicted delta:** **+300-600 LOC** (medium-confidence; entropy-mitigation shape is novel third substrate).

**Features deferred from scaffold to complete:**
1. Multi-line link parsing — markdown links spanning multiple lines (currently single-line only)
2. Reference-style markdown links — `[text][ref]` form with `[ref]: path` definitions
3. HTML comments skipping — `<!-- ... -->` blocks
4. Indented-code-block skipping — 4-space indentation per CommonMark
5. Frontmatter-aware skipping — don't treat YAML frontmatter content as markdown body
6. Exclude patterns — `--exclude` glob to skip specific paths
7. Dead-link auto-fix suggestions — when a target is missing, search stem-index for a near-match and suggest it
8. Stale-detection by content-hash equality — detect "stale by neglect" vs "deliberately preserved unchanged"
9. Configurable corpus exclusions via config file
10. Detector composition — allow chaining (e.g., dead-links-only-on-stale-files)

**Predicted complete LOC:** ~900-1200 prod LOC (614 + 300-600 delta).

**±10% magnitude band:** delta likely lands at +275-660 LOC; complete at ~890-1275 LOC.

**Direction prediction:** the entropy-mitigation shape is novel; could land either above or below the predicted upper bound. Wiki-search 125→127 came in 7.6% above upper; boot-phase 123→124 came in 7% below lower. No strong directional prior for cycle 131→132.

## Pattern updates

- **`scaffold-partial-as-measurement-primitive`** (VALIDATED-VIA-COMPLETION-AT-SECOND-CRATE cycle 127) → **APPLIED-AT-THIRD-SCAFFOLD cycle 131**. Third scaffold has now been built. HARDENED@3 lands at cycle 132 COMPLETE measurement (when the +300-600 delta prediction is tested against actual). Cycle 131 builds the substrate; cycle 132 closes the arc.

- **`dependency-footprint-once-at-first-crate`** (HARDENED-AT-3-BOUNDARIES cycle 127: external services → in-process utilities → IR primitives) → **EXTENDED-TO-4-BOUNDARIES cycle 131** (filesystem-walk + markdown link parsing primitives use `std::fs` + hand-written link extraction; **zero new transitive deps added**, including no `walkdir`, no `regex`, no `pulldown-cmark`). The dependency-discipline pattern now spans entropy-mitigation as a fourth distinct domain boundary.

- **`crate-shape-dependent-test-prod-ratio`** (TESTED@5 cycle 127) → **TESTED@6 cycle 131**. Cycle 131's 0.73× scaffold ratio is the LOWEST among the 6 measured crates. Six-crate distribution: 0.73 (gardening-sweep scaffold) / 0.86 (state-machine) / 1.10 (orchestration-hub) / 1.31 (top-k-retrieval) / 1.50 (catalog-enumeration) / 1.90 (append-only-writer). Range 0.73-1.90 (~2.6× spread). Shape-dependence dimension: **detector-walker shapes** (gardening-sweep, phase-transition-check) cluster low at 0.73-0.86; **orchestration-hub + retrieval** cluster middle at 1.10-1.31; **catalog + writer** cluster high at 1.50-1.90. Test-surface concentration appears related to per-CLI-feature edge-case density rather than per-prod-LOC behavior count.

- **`discretionary-departure-from-forward-going-commitment`** HARDENED-at-4 → **seventeenth consecutive HONORING cycle 131** (cycles 115-131); 21 cycles of substrate.

- **NEW candidate-emergent observation `scaffold-already-produces-real-housekeeping-findings`** — cycle 131's gardening-sweep scaffold, run against the actual `docs/redesign/_notes/` corpus on its first invocation, surfaced 9 real dead-link findings that the orchestrator can act on in housekeeping. The 9 findings are not test artifacts; they are real path-errors in cycle-93/94/95/104/129 notes + a deferred-system reference in cycle-34. **Implication:** scaffolds in this codebase aren't pure proof-of-concept — they immediately produce substantive value on first execution against real substrate, validating CORE-DESIGN-PRINCIPLE's bet that simple per-crate scope yields high marginal utility. Sibling to `calibration-extends-by-association` (cycle 130) and `judgment-range-can-envelope-empirical-truth` (cycle 130). Not yet promoted to candidate-pattern (would require a second instance where a scaffold-on-first-run surfaces real corpus issues).

## Bottleneck state at cycle 131 session-end

- Audit cycle 218 expected ~04:00 UTC 2026-05-13 (~5.5h post session-start).
- Q7 still Eva-blocked.
- Audit#462 M1-M5 + P3-1/P3-2/P3-3/P3-4/P3-7/P3-8 carried for cycle 132+ (8 of 13 findings remaining).
- 0 open Copilot dispatches.
- Pre-existing `v2-phase-transition-check` test failures (2/25 panic; cycle-122-vintage) carried for cycle 132+ triage.
- **43rd consecutive bottleneck-asynchronous cycle (cycles 78-131)** AND **21st consecutive non-per-candidate-sharpening cycle (cycles 111-131)**.

## Forward work for cycle 132+

In priority order:

1. **Audit cycle 218 critique absorption** (if landed; expected ~04:00 UTC 2026-05-13). If a `redesign-feedback` issue parallel to audit#462 has landed, it's the absorption priority.
2. **Q7 resolution by Eva** — Eva-blocked.
3. **`v2-gardening-sweep` COMPLETE** — promote scaffold to complete; test +300-600 LOC delta prediction; promote `scaffold-partial-as-measurement-primitive` to HARDENED@3.
4. **Audit#462 M1-M5 absorption** — audit retrospective consultation; fourth-candidate option naming; Eva-legibility positive criterion; liveness + commitment-thread observability for Phase 3; state.json size discipline. (M5 has fresh substrate via audit#463 question-for-eva on `recommendations.accepted` retention — read audit's options + outcome and integrate the Phase 3 design requirement.)
5. **Audit#462 P3-3 absorption** — formalize Eva's [audit#455](https://github.com/EvaLok/schema-org-json-ld-audit/issues/455) Option C/D `audit-request` label.
6. **Audit#462 P3-1 + P3-2 absorption** — liveness assertion + commitment-thread observability mechanisms for Phase 3 design requirements.
7. **`v2-phase-transition-check` pre-existing test failures triage**.
8. **`v2-close-phase` scaffold** — fourth scaffold→complete delta primitive (after gardening-sweep COMPLETE provides the sub-step dep).
9. **NO further recursive annotation of 2-selection.md** — cycle 120 L2 constraint continues.
10. **Symphony deeper-read elevation** + **oh-my-claudecode deeper-read elevation** — Phase 1 research forward.

## Honest characterization

Cycle 131 is a SCAFFOLD-BUILDING cycle — the substrate it produces is a measurement candidate for cycle 132's COMPLETE comparison, not a settled deliverable. The pattern-hardening claim (`scaffold-partial-as-measurement-primitive` → HARDENED@3) is DEFERRED to cycle 132 when the COMPLETE measurement validates or refutes the +300-600 LOC delta prediction.

The 6-crate cumulative trajectory (5346 9-crate extrapolation) is essentially unchanged from cycle 127's 5-crate trajectory (5310). The dominant signals from cycle 131 are:
1. **Substrate-content-distinct evidence** for the third scaffold→complete arc (entropy-mitigation novel third shape, complementing orchestration-hub and top-k-retrieval).
2. **Dependency-discipline extension** to 4 distinct domain boundaries.
3. **Real housekeeping value** produced on first execution against actual corpus (9 dead-link findings) — a candidate-emergent observation about scaffold marginal utility in this codebase.

Cycle 131 does NOT change Q7's resolution surface (Q7 stable across cycles 126→128→129→130 absorption arc; cycle 131 doesn't engage Q7-relevant evidence directly).
