# Cycle 93 — Tool-registry prototype scaffold + 2 Copilot feedback dispatches

**Cycle issue:** [#2868](https://github.com/EvaLok/schema-org-json-ld/issues/2868)
**Date:** 2026-05-08
**Mode:** redesign Phase 2 candidate iteration (under [`ITERATION-UNTIL-APPROVAL`](../../.github/workflows/orchestrator-redesign-prompt.xml), **fourth cycle of Phase 2 candidate-set work**)
**Cycle composition shape:** **Paired execution** (functional-class shape: simultaneous Phase-3-prototype-scaffold + 2-feedback-dispatch — first instance, NOVEL at 1 instance). 14 functional-class shapes total demonstrated through cycle 93. Substantive focal: depart from cycle 92's hand-off default (Copilot feedback dispatch alone) toward a paired-execution shape that combines bounded dispatch + first piece of executable redesign code.

## Why this shape (deviation from cycle 92 hand-off)

Cycle 92 named option (2) Copilot feedback dispatch as cycle 93 substantive focal default, with option (1) sharpening and option (4) prototype scaffold as alternatives. The paired execution chosen this cycle is a **bounded application of (2) + execution of (4)**, rationale:

1. **Eva backlog real but bounded.** Cycle 92's bottleneck honesty named "Eva's manual Copilot assignment for 4 dispatches" as the active constraint. 4 research dispatches ([#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833), [#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842), [#2847](https://github.com/EvaLok/schema-org-json-ld/issues/2847), [#2851](https://github.com/EvaLok/schema-org-json-ld/issues/2851)) are still assigned to EvaLok (not Copilot) as of cycle 93 fire. Adding the cycle 92 plan's full multi-lens dispatch set (4 dispatches: F-pattern coverage / P1-P6 ordering / tool-surface feasibility / sharpening-claim adversarial) would compound the backlog. **Bounded to 2 dispatches** with the most-discriminating lenses.

2. **Iteration-mix diversity.** Cycles 91-92 were both sharpening cycles. A third consecutive sharpening cycle risks the local-optimum cycling pattern that cycle 92's evaluation-discipline learnings called out (sub-shape adoption count vs per-cycle decision count distinction surfaced *because* sharpening produced an evaluation-discipline refinement). Diversifying iteration activities — moving from analysis-only (sharpening) to evidence-via-execution (prototype scaffold) — broadens the iteration mix per `ITERATION-UNTIL-APPROVAL`'s named activities.

3. **Prototype scaffold authorized at MEDIUM PRIORITY in cycle 92's plan.** Cycle 92 explicitly authorized "Phase 3 prototype scaffold for the smallest A-shared crate" as cycle 93 option (4) with the framing "NOT a full Phase 3 prototype — that requires candidate-selection-checkpoint approval. Cycle-bounded, scoped to migration-cost validation only." Building the smallest crate (`tool-registry`) is within that authorization.

4. **Paired execution is independent.** Dispatches and prototype scaffold are non-overlapping work surfaces. Filing 2 issues + building 1 crate fits ~75 min of cycle compute.

## What I did

### 1. Filed 2 Copilot feedback dispatches

Both dispatches use `agent-task,feedback-only` labels per cycle 30's `[redesign-feedback]` convention ([#2790](https://github.com/EvaLok/schema-org-json-ld/issues/2790) reference template).

**Dispatch #1: [#2869](https://github.com/EvaLok/schema-org-json-ld/issues/2869) — Tool-surface feasibility lens.** Adversarial critique on whether the per-candidate Rust crate counts and LOC ranges (A: ~9 crates / 3000-4500 LOC; C: ~11 crates / 4000-6000 LOC; B: ~12+ Rust + 20-40 skill / 10000-20000 LOC) are realistic given typical Rust idioms and per-crate functional contracts. Seven lenses: per-crate scope realism for each candidate; aggregate net-add LOC realism; cutover scope predictability; F-pattern structural coverage vs tool count; per-candidate tool-count growth risk. Deliverable: `docs/redesign/_notes/cycle-93-tool-surface-feasibility-feedback.md`.

**Dispatch #2: [#2870](https://github.com/EvaLok/schema-org-json-ld/issues/2870) — Sharpening-claim adversarial critique lens.** Adversarial critique on whether cycle 91-92 sharpening sections are genuinely structurally validated or assertion-chains dressed up as enumeration. Seven lenses: structural enumeration vs assertion-chain (A); decision-class enumeration vs decision-class-naming (B); per-failure-mode metrics vs anchored numbers (C); direction-vs-magnitude discipline honesty; risk enumeration adequacy; "validation plan" vs validation-plan-naming; cross-candidate sharpening-discipline parity. Deliverable: `docs/redesign/_notes/cycle-93-sharpening-claims-feedback.md`.

**Lens-selection rationale.** Among the four lenses cycle 92 named, tool-surface feasibility and sharpening-claim adversarial critique were chosen as the most-discriminating two:

- **Tool-surface feasibility** — directly stress-tests one of the most-discriminating criteria across A/B/C (migration cost; cited in `2-candidates/README.md` as key candidate-selection signal). Copilot is well-suited for this lens because the question is structural Rust engineering (crate scope realism), which Copilot can evaluate without the cluster framework / 12-axis design framework backdrop.
- **Sharpening-claim adversarial critique** — directly stress-tests the newest content (cycle 91-92 sharpening sections) which has not been externally critiqued. Adversarial reading produces critique surface that audit's structural critique (audit#454) didn't cover (audit critiqued the cluster framework, not the sharpening claims).

The other two lenses (F-pattern coverage; P1-P6 ordering) are deferred to potential future dispatches if the first two return critique surfaces that warrant deeper exploration.

### 2. Built `v2-tool-registry` Rust crate scaffold

Location: [`tools/rust/crates/v2-tool-registry/`](../../tools/rust/crates/v2-tool-registry/) + shell wrapper at [`tools/v2-tool-registry`](../../tools/v2-tool-registry).

**Functional contract.** Enumerate Rust crates under `tools/rust/crates/`, parse their `Cargo.toml` description fields, output as markdown / JSON / names. Used for orchestrator tool-discovery (Candidate A's Axis 6 mechanism: tools-with-registry).

**CLI surface:**
- `--repo-root <path>` (default `.`)
- `--format markdown|json|names` (default markdown)
- `--filter <substring>` (optional name filter)
- `--v2-only` flag (filter to v2-prefixed crates only)

**Implementation choices:**
- **No `toml` crate dependency** — hand-rolled `extract_description` parser handles `description = "..."` lines under `[package]` section. Trade-off: slightly fragile for unusual TOML constructs (multi-line strings, table-array syntax) but matches the constraint of minimal new dependencies. The 9 unit tests cover the supported cases; 1 test documents the limitation (multi-line strings return empty string, detectable by callers).
- **Self-exclusion** — the registry tool excludes itself from output (avoid recursion / discovery noise).
- **BrokenPipe handling** — explicit stdout writer with `io::ErrorKind::BrokenPipe` caught in `main()` and converted to clean `ExitCode::SUCCESS`. This is needed for `| head` and similar shell idioms.

**Test coverage:** 9 unit tests + 6 integration tests = 15 tests, all passing. Integration tests use `tempfile` to construct synthetic crates directories for isolated testing. Smoke-tested against the actual workspace (36 crates discovered correctly; v2-only filter returns empty as expected — only v2-tool-registry exists at v2-prefix and is self-excluded).

**Built artifact:** `target/release/v2-tool-registry` builds in ~1.24s (release mode). Shell wrapper auto-builds on first run if binary missing.

### 3. Migration-cost validation result

Cycle 92's load-bearing claim for Candidate A: "~9 new Rust crates, ~200-500 LOC each, ~3000-4500 LOC aggregate, single-cycle scope per crate."

**v2-tool-registry empirical measurement:**
- Production code: ~231 LOC (`main.rs` minus inline tests + Cargo.toml + wrapper script)
- Test code: ~347 LOC (`main.rs` inline tests + `integration.rs`)
- Total LOC: ~578 (Cargo.toml + main.rs + integration.rs + wrapper)
- Build time: ~1.24s release
- Test pass rate: 15/15
- Cycle-bounded scope: completed in approximately the second half of cycle 93 (~30-45 min of cycle compute), well within single-cycle scope.

**Direction:** validated. The single-cycle scope per crate is empirically achievable for at least this one crate. Production code (~231 LOC) is within A's stated ~200-500 LOC range; if test code is included, total LOC slightly exceeds the upper bound (~578 vs 500), but A's "~200-500 LOC" was specified for the production code surface, not the test surface.

**Magnitude:** prototype-pending across remaining 8 A-shared crates. The ~9 crates listed in A's tool surface differ in functional load — `phase-transition-check` is likely smaller than `wiki-search`; `gardening-sweep` may be larger if quality-grading rubrics are incorporated; `cycle-history-append` is straightforward append-only file write; `tool-registry` itself is one of the more I/O-tractable crates. Across A's full crate set, aggregate LOC may land at the upper bound (~4500 LOC) or slightly above.

**Risks named (per cycle 92 sharpening discipline):**

1. **Test-code amplification.** If subsequent A-shared crates produce similar 1.5× test-to-production code ratios, A's aggregate LOC should be re-cast as production-LOC + test-LOC = ~3000-4500 + ~4500-6750 = ~7500-11250 *with tests*. The candidate document's "~3000-4500 LOC aggregate" ambiguously refers to production code only. Open question: should the migration-cost claim include test code? If yes, A's range needs upward revision; if no, the candidate document should clarify.

2. **`wiki-search` is the most LOC-intensive A crate.** The `tool-registry` crate is straightforward (file enumeration + TOML field parsing). `wiki-search` requires top-k retrieval over LLM-generated descriptions, which involves either (a) a search index format + index update + query, or (b) external dependency (e.g., a search library). Either path exceeds 500 LOC plausibly. `wiki-search` may be the constraint that pushes A's aggregate above the upper bound.

3. **Cargo.lock churn.** Adding `tempfile` as a dev-dependency added ~30 transitive dependencies to the workspace lockfile (visible in `cargo build` output). Per-crate dependency footprints for the remaining 8 A-shared crates are not yet measured. If each crate adds ~30 transitive deps, the aggregate workspace dep count balloons. Workspace-shared dependencies should be considered.

4. **Limitations of the description parser.** The hand-rolled `extract_description` parser handles `description = "..."` only; multi-line strings (triple-quoted) return empty string. This is acceptable for v1 prototype because the workspace convention is single-line descriptions. If the `tool-registry` becomes part of the cutover deliverable, this should be re-evaluated against actual Cargo.toml content in the workspace at cutover time.

### 4. Functional-class shape observation

This cycle introduces functional-class shape #14: **Paired execution = bounded-dispatch + prototype-scaffold**. NOVEL at 1 instance (cycle 93). Distinct from cycle 90-92's Phase 2 candidate authoring shape because:

- Phase 2 candidate authoring is analysis-only (writing candidate documents, sharpening claims, side-by-side comparison)
- Paired execution combines analysis (dispatch issues) with execution (Rust crate)

If cycles 94+ produce same-shape instances (further dispatch + further prototype crates), this shape advances to TESTED. Eva's input may also redirect cycle 94's substantive focal — the prototype scaffolding shift is a deliberate diversification but Eva should weigh in on whether prototype work should continue at this iteration or revert to dispatch-and-await.

### 5. Standing tasks

- 5 open issues unchanged from cycle 92: #2833 (oh-my-codex 30+ cycles), #2842 (PAI 22+ cycles), #2847 (oh-my-claudecode 18+ cycles), #2849 (audit-engagement 16+ cycles awaiting audit cycle 213/214 read), #2851 (Symphony 16+ cycles)
- 2 open Copilot feedback dispatches filed cycle 93: #2869 (tool-surface feasibility), #2870 (sharpening-claim adversarial)
- 1 prototype scaffold landed cycle 93: `v2-tool-registry` crate + wrapper
- 7 standing input-from-eva directives unchanged

## Sibling pattern tracking

- **Functional-class shape #14 (paired execution = bounded-dispatch + prototype-scaffold) — NOVEL at 1 instance** (cycle 93). New shape introduced this cycle.
- **Functional-class shape #13 (Phase 2 candidate authoring) — HARDENED at 3-instance evidence** (cycles 90-92). NOT an instance at cycle 93.
- **Sharpening pattern as iteration-until-approval discipline — TESTED at 2-cycle evidence** (cycles 91 + 92). NOT an instance at cycle 93.
- **Direction-vs-magnitude discipline — extends to 4 instances** (cycle 91 A's Axis 13 + cycle 92 B's per-role decision count + cycle 92 C's central bet + cycle 93 prototype scaffold migration-cost validation). HARDENED holds.
- **Generalization-level discipline (J-Q(a)) — extends to 11 instances** (cycle 84+85+86+87+87-reflection+88+89+90+91+92+93). HARDENED holds.
- **Audit-as-peer pattern** — 2-instance evidence holds. Cycle 93 NOT a new instance.
- **Sibling pattern binary-becomes-more-structured** — HARDENED at 4 instances. Cycle 93 NOT a new instance.

## Bottleneck-state honesty

Bottleneck remains external (Eva's manual Copilot assignment for 4 dispatches; audit cron for #2849). Cycle 93's 2 new feedback dispatches add to the assignment-pending queue (4 → 6 dispatches awaiting Eva's manual assignment). The prototype scaffold is fully repo-internal. Cycle 93 is the **FIFTEENTH consecutive cycle** (cycles 78-93) whose output is fully repo-internal for the prototype-scaffold component, plus the cycle 93 dispatches that will benefit external work in subsequent cycles.

The decision to bound dispatch count at 2 (rather than the cycle-92-plan's full 4) reflects honest bottleneck accommodation: 4 dispatches simultaneously would compound Eva's assignment load. 2 dispatches still produces parallel critique surfaces while leaving cycle 94+ headroom for additional dispatch lenses if needed.

## Honest reflection (per F1 corrective)

Cycle 93 is **2 substantive activities** (paired execution = 2 feedback dispatches + 1 prototype crate scaffold) yielding 2 dispatch issues + 1 Rust crate (~578 LOC across 4 files including tests + wrapper) + 15 passing tests + 1 _notes documentation file + 1 journal entry. Per F1 corrective: components are deliverable form of substantive activity; documentation IS the activity. Per F2 corrective: 2 dispatch issues + 1 prototype crate from 2 paired activities, NOT "2 ways improved." Per F3: 14 functional-class shapes at 33 instances. Per F4: HARDENED/TESTED/NOVEL is for redesign-process methodology only. Per F5: lexicon (paired-execution shape, prototype-scaffold migration-cost-validation discipline, test-code amplification risk, dependency footprint risk) as documented learning. Per H-Q(a) anti-inheritance: cycle 93 prototype scaffold grades empirical-observation-grounded against single-crate measurement; cycle 93 dispatches grade per cycle 30 dispatch convention. Per J-Q(a) generalization-level discipline: HARDENED at 11 instances.

**Self-congratulation audit.** Cycle 93 prototype scaffold could be over-stated as "validates A's migration cost claim." More honest: validates the single-crate scope claim for one specific crate (`tool-registry`). The aggregate-LOC claim for A's full 9-crate set is NOT validated by cycle 93 — only one crate has been measured. Risk #2 (wiki-search likely largest) is a structural concern not addressed by the prototype.

## Cycle 94 plan

Per `ITERATION-UNTIL-APPROVAL`, cycle 94 substantive focal options:

1. **Continue prototype scaffolding** MEDIUM PRIORITY — build the next-smallest A-shared crate (`cycle-history-append` or `phase-transition-check`) to extend the migration-cost validation surface beyond single-crate measurement. Aggregate measurement across 2-3 crates produces stronger validation signal than 1.
2. **Second-iteration sharpening** MEDIUM-HIGH PRIORITY — apply cycle 91-92 sharpening pattern to remaining first-iteration risks: A's tool-registry-growth risk (Risk 3); B's coordination-overhead-magnitude (~9-23 estimate); C's plan-authoring-discipline-conditional risk (Risk 2).
3. **Wait for cycle 93 dispatch returns** LOW-MEDIUM PRIORITY — if Eva assigns the cycle 93 dispatches to Copilot before cycle 94 fires, cycle 94 can integrate returned critique. If not, cycle 94 proceeds with substantive option (1) or (2).
4. **Phase 3 prototype scaffold for next-smallest A-shared crate** — MEDIUM PRIORITY — `cycle-history-append` (smaller than `tool-registry` per A's description: append-only file write).
5. **Bounded-mechanical fallback** — close absorbed dispatches if any deliver.

**Cycle 94 substantive focal default:** option (1) or (4) — continue prototype scaffolding to extend migration-cost validation. Cycle 93 measured 1 crate; cycle 94 measuring 1-2 more crates moves from single-instance evidence to 2-3 instance evidence. Fallback to option (2) sharpening if prototype scaffolding hits unexpected scope.

## What surprised me / what I noticed

- **Test code is significant fraction of total LOC.** ~347 test LOC vs ~231 production LOC = 1.5× ratio. A's stated "~3000-4500 LOC aggregate" implicitly excludes test code; with test code, aggregate is ~7500-11250. This is a **cycle-93-emergent risk** that should propagate to A's candidate document if it survives further crate measurement.
- **Hand-rolled TOML parser is acceptable for prototype but limits.** Multi-line description strings return empty (test documents this). For cutover, this should either get the `toml` crate dependency or get a more robust parser. Risk named.
- **The `--v2-only` filter returns empty against actual workspace.** Only v2-tool-registry has the `v2-` prefix and it's self-excluded. The flag is only useful when ≥2 v2-prefixed crates exist. This is expected at cycle 93 (first v2 crate); will become useful as more v2 crates land.
- **`tempfile` dev-dependency added ~30 transitive deps.** First-time visibility into per-crate dependency footprint. Risk #3 (Cargo.lock churn) is a real concern across A's 9-crate set — workspace-shared dependencies should be considered as scaffolding lands.
- **BrokenPipe panic on stdio is a known Rust idiom.** Initial naïve implementation panicked when piped through `head`. Fix is straightforward (explicit stdout handle + BrokenPipe error catch). Pattern should be reused across all v2 tool prototypes that produce streaming output.
- **The dispatch-bound-to-2 decision was load-bearing.** Cycle 92's plan named 4 dispatches; cycle 93 bounded to 2. The Eva-backlog argument was decisive. Future cycles should weigh dispatch count against bottleneck state honestly rather than defaulting to "as many as the plan named."
- **Paired execution as iteration-mix diversification.** This cycle deliberately departed from the analysis-only iteration mix (cycles 90-92 were all candidate authoring + sharpening) toward execution-evidence (prototype scaffold). The diversification reflects ITERATION-UNTIL-APPROVAL's named activities being not just "sharpen / critique / stress-test" but also implicitly "produce evidence by execution." Whether this is a one-time shift or a sustained iteration shape depends on whether the prototype work produces unexpected discoveries that warrant continued execution-emphasis.
