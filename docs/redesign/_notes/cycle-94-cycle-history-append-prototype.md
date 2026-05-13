# Cycle 94 — `v2-cycle-history-append` prototype scaffold (second crate)

**Cycle issue:** [#2871](https://github.com/EvaLok/schema-org-json-ld/issues/2871)
**Date:** 2026-05-08
**Mode:** redesign Phase 2 candidate iteration (under [`ITERATION-UNTIL-APPROVAL`](../../../.github/workflows/orchestrator-redesign-prompt.xml), **fifth cycle of Phase 2 candidate-set work**)
**Cycle composition shape:** **Phase 3 prototype-scaffold-only** (functional-class shape: Phase 3 prototype scaffold without paired dispatch — distinct from cycle 93's paired-execution shape #14). 14 functional-class shapes total demonstrated through cycle 94 at 34 instances (cycle 93's shape #14 advances from NOVEL at 1 instance to TESTED at 1 instance + 1 close-relative; or remains NOVEL with cycle 94 introducing shape #15 prototype-only — see *Sibling pattern tracking*).

## What I did

### 1. Built `v2-cycle-history-append` Rust crate

Location: [`tools/rust/crates/v2-cycle-history-append/`](../../../tools/rust/crates/v2-cycle-history-append/) + shell wrapper at [`tools/v2-cycle-history-append`](../../../tools/v2-cycle-history-append).

**Functional contract.** Append a single cycle-history entry as a JSON file at `state/cycle-history/<N>.json`. Refuse-overwrite by default for append-only semantics; `--allow-overwrite` is the explicit escape hatch for tests and one-shot migrations. Validates required fields (`cycle_number`, `model`, `started_at`) before write; ensures payload `cycle_number` matches `--cycle-n` CLI argument; passes through unknown fields unchanged for forward compatibility.

**CLI surface:**
- `--cycle-n <num>` (required) — cycle number; must match payload `cycle_number` if both present
- `--repo-root <path>` (default `.`) — repository root containing the state directory
- `--state-dir <path>` (default `state/cycle-history`) — relative path under repo-root
- `--from-stdin` — read JSON object payload from stdin (mutually exclusive with `--from-json`)
- `--from-json <path>` — read JSON object payload from a file (mutually exclusive with `--from-stdin`)
- `--field KEY=VALUE` (repeatable) — set or override a field; values auto-parse as bool/u64/i64/f64, fall back to string
- `--allow-overwrite` — escape hatch for refuse-overwrite (DANGER: breaks append-only invariant)
- `--dry-run` — print path + content; do not write
- `--output path|json|quiet` (default `path`) — what to print to stdout after success

**Implementation choices:**
- **Pass-through schema** — payload is a `serde_json::Map<String, Value>`, not a typed struct. Required fields validated by name; unknown fields preserved verbatim. Forward-compatible: callers can add fields without tool change.
- **Refuse-overwrite invariant** — `target.exists()` check with explicit `--allow-overwrite` override. This makes the append-only contract structural (the tool itself protects it), not just convention. Maps directly to A's Axis 4 (git-as-substrate) — once written + committed, the file is immutable.
- **Atomic write** — `fs::write(tmp); fs::rename(tmp, target)` pattern. Prevents partial-write corruption if killed mid-write. Works on POSIX same-filesystem (which our state dir is by construction).
- **`--field` value auto-typing** — bool/integer/float parseable values get typed JSON values; otherwise string. Trade-off: ambiguity for values like `"true"` that should stay strings. Workaround: use `--from-json` for typed-sensitive payloads.
- **Cycle-N consistency check** — payload `cycle_number` must match `--cycle-n` arg. Catches caller errors (e.g., a stale payload re-used for a different cycle).
- **Pretty-printed JSON with trailing newline** — git-friendly, human-readable, plays well with diff/blame.

**Test coverage:** 19 unit + 12 integration = 31 tests, all passing. Integration tests use `tempfile` for isolated state directories. Coverage:
- Unit tests cover: `split_field` parsing (basic, numbers, negatives, floats, bools, equals-in-value, empty-key rejection, no-equals rejection); `parse_object` (accepts object, rejects array); `validate_payload` (complete/missing/cycle-mismatch/empty-model/non-integer-cycle); `serialize_payload` (round-trip); `atomic_write` (creates / overwrites / no-extension)
- Integration tests cover: `--field`-based write; refuse-overwrite (default); `--allow-overwrite`; `--dry-run`; missing-required-field rejection; `--from-json` with passthrough; CLI-field-overrides-JSON precedence; cycle-mismatch rejection; custom `--state-dir`; `--output quiet` / `--output json`; invalid `--field` format

**Built artifact:** `target/release/v2-cycle-history-append` builds in ~1.01s (release mode, fresh build with deps already in workspace cache). Shell wrapper auto-builds on first run if binary missing.

**Git executable bit set:** `git update-index --chmod=+x` applied so the wrapper is `100755` in the index (matching `tools/v2-tool-registry` convention).

### 2. Migration-cost data (cycle 94 measurement)

| Metric | Cycle 93 `tool-registry` | Cycle 94 `cycle-history-append` |
|---|---|---|
| Production LOC (main.rs minus inline tests) | ~231 | ~268 |
| Inline test LOC | ~167 | ~167 |
| Integration test LOC | ~180 | ~343 |
| Total test LOC (inline + integration) | ~347 | ~510 |
| Cargo.toml LOC | ~13 | ~13 |
| Wrapper LOC | ~13 | ~13 |
| **Total LOC (all files)** | **~578** | **~804** |
| Test count (unit + integration) | 9 + 6 = 15 | 19 + 12 = 31 |
| Test:Production LOC ratio | ~1.5× | ~1.9× |
| Build time (release, fresh) | ~1.24s | ~1.01s |
| New transitive deps added to workspace lockfile | ~30 | **0** |
| Single-cycle scope completed | yes (~30-45 min) | yes (~30-45 min) |

### 3. Comparison against cycle 93 risks

**Cycle 93 risk #1 (test-code amplification):** **PARTIALLY SUPPORTED.** Cycle 94 test:prod ratio is 1.9× (vs cycle 93's 1.5×). Direction holds (test code remains substantial); magnitude widened. Aggregate-LOC implication for A (per cycle 93): with N-crate average test:prod ratio of ~1.7×, A's 9-crate aggregate at the upper bound (~4500 prod LOC) yields ~7650 test LOC and ~12150 total LOC — exceeding A's stated "~3000-4500 LOC aggregate" by a factor of ~2.7×. This refines cycle 93's "~7500-11250 with tests" range upward to ~12000.

**Cycle 93 risk #3 (Cargo.lock churn from new dev-dependencies):** **REFUTED at the second-crate level.** Cycle 94 added ZERO new transitive deps to the workspace lockfile. The first crate to introduce a dep family (`tempfile` + clap + serde + serde_json) pays the transitive-dep cost; subsequent crates reusing the same deps are free. Cycle 93 risk #3 was a one-time cost, not a per-crate cost — provided subsequent crates converge on a stable dep stack. **Corollary:** if A's remaining 7 crates all use {clap, serde, serde_json, tempfile}, dependency-footprint cost is bounded at the cycle-93 measurement (~30 transitive deps once). The risk is recharacterized: it's a workspace-wide one-time cost, not a per-crate accretion.

**Cycle 93 risk #2 (`wiki-search` likely largest crate):** **NOT TESTED.** Cycle 94 measured `cycle-history-append`, not `wiki-search`. Risk #2 remains structural and unaddressed by cycle-94 evidence.

**Cycle 93 risk #4 (description-parser limitations):** **N/A** for cycle 94 — `cycle-history-append` does not parse Cargo.toml. Risk #4 is `tool-registry`-specific.

### 4. Migration-cost validation result (cumulative across 2 crates)

Cycle 92's load-bearing claim for Candidate A: "~9 new Rust crates, ~200-500 LOC each, ~3000-4500 LOC aggregate, single-cycle scope per crate."

**Cumulative measurement (2 of 9 A-shared crates):**
- Production code: 231 + 268 = 499 LOC across 2 crates (~250 LOC/crate average)
- Both within A's stated 200-500 LOC per-crate range; both at single-cycle scope
- Aggregate trajectory: at ~250 LOC/crate × 9 crates = ~2250 production LOC. **Below** A's stated 3000-4500 lower bound. Caveat: the 2 crates measured are explicitly the **smaller** end of A's tool surface; `wiki-search` (cluster B sub-shape 6 = top-k retrieval over LLM-generated descriptions) is structurally larger.
- With test code: ~750 total LOC/crate × 9 crates = ~6750 LOC. Closer to A's "with tests" range of ~7500-11250.

**Direction:** A's per-crate scope claim (~200-500 LOC, single-cycle) is **further validated** at 2 crates. **Magnitude:** aggregate prod LOC may land below A's stated ~3000-4500 range if subsequent crates remain near 250 LOC. Two countervailing factors:

1. **Smaller-end bias** — the 2 measured crates are the structurally-simplest. The remaining 7 (`boot-phase`, `close-phase`, `phase-transition-check`, `wiki-search`, `prompt-contract-check`, `detect-abandoned-cycles`, `gardening-sweep`) are likely larger.
2. **Test-code dominance** — total LOC including tests is the more honest measure, since tests are part of the deliverable per ARTIFACT-COMPOSITION.

**Refined aggregate estimate:** total LOC ~7000-12000 across 9 A-shared crates. Production LOC ~2500-4500.

### 5. Risks named for `cycle-history-append` (per cycle 91-93 sharpening discipline)

1. **Loose schema validation.** The tool only enforces 3 required fields; the rest of the schema is convention. As the cycle-history schema evolves, future tools that consume these files (e.g., `detect-abandoned-cycles`, trend-analysis tools) may need stricter contracts. Mitigation: defer until consumer pressure forces stricter validation; add `schema_version` field at first migration.

2. **`--field` value auto-typing ambiguity.** Values like `true` always parse as boolean; values like `123` always parse as integer. Edge case: a string field intended to hold "true" will silently become a boolean. Mitigation: use `--from-json` for typed-sensitive payloads. If this becomes a recurring issue, add `--field-string KEY=VALUE` for explicit string typing.

3. **No `started_at` format validation.** The tool accepts any non-empty string. If a caller writes a malformed timestamp, the file is written and the error surfaces only when a downstream tool tries to parse it. Mitigation: add RFC3339 parsing if downstream consumers require it.

4. **Atomic write assumes same-filesystem temp + target.** `fs::rename` is atomic on POSIX same-filesystem. The tool's default behavior keeps tmp + target on the same filesystem (via `target.with_extension("json.tmp")`). If a caller specifies a state-dir on a different mount than the repo, the rename could fall back to copy-and-delete (Linux `rename(2)` returns `EXDEV`). Mitigation: documented assumption; if cross-mount support is needed, switch to `tempfile::NamedTempFile::persist`.

5. **No JSON schema versioning.** If the cycle-history schema evolves, old cycle-history files won't have new required fields. Tools that consume the directory must handle missing-field gracefully. Mitigation: add `schema_version` field at first schema migration; defer until needed.

6. **Refuse-overwrite is filesystem-level, not git-level.** A file deleted by a caller (e.g., a bug in another tool) and then re-created via this tool would not be detected. The git-level invariant (`commit-must-be-pushed` + append-only history per A's Axis 4) is the substrate-level guarantee; this tool's filesystem-level check is a defense-in-depth complement, not a replacement.

### 6. Sibling pattern tracking

- **Functional-class shape #14 (paired execution = bounded-dispatch + prototype-scaffold) — remains NOVEL at 1 instance** (cycle 93). Cycle 94 is NOT a new instance because cycle 94 omits dispatch (Eva backlog at 6 awaiting assignment).
- **Functional-class shape #15 (Phase 3 prototype-scaffold-only) — NOVEL at 1 instance** (cycle 94). New shape: prototype scaffold without paired dispatch, distinct from cycle 93's paired-execution shape because dispatch is omitted as Eva-backlog accommodation.
- **Functional-class shape #13 (Phase 2 candidate authoring) — HARDENED at 3-instance evidence** (cycles 90-92). NOT an instance at cycle 94.
- **Sharpening pattern as iteration-until-approval discipline — TESTED at 2-cycle evidence** (cycles 91-92). NOT an instance at cycle 94 (no candidate-document sharpening this cycle).
- **Direction-vs-magnitude discipline — extends to 5 instances** (cycle 91 A's Axis 13 + cycle 92 B's per-role decision count + cycle 92 C's central bet + cycle 93 prototype scaffold migration-cost + cycle 94 prototype scaffold migration-cost #2). HARDENED holds.
- **Generalization-level discipline (J-Q(a)) — extends to 12 instances** (cycle 84+85+86+87+87-reflection+88+89+90+91+92+93+94). HARDENED holds.
- **Audit-as-peer pattern** — 2-instance evidence holds. Cycle 94 NOT a new instance.
- **Sibling pattern binary-becomes-more-structured** — HARDENED at 4 instances. Cycle 94 NOT a new instance.
- **Prototype-scaffold migration-cost-validation discipline (cycle 93 lexicon entry)** — extends to 2 instances (cycle 93 + cycle 94). Direction validated; magnitude refined per *Cumulative measurement* section above.

### 7. Bottleneck-state honesty

Bottleneck remains external (Eva's manual Copilot assignment for 6 dispatches: 4 research-only + 2 feedback-only filed cycle 93; audit cron for #2849). Cycle 94 is fully repo-internal (prototype-scaffold-only, no new dispatches). Cycle 94 is the **SIXTEENTH consecutive cycle** (cycles 78-94) whose substantive output is fully repo-internal.

The decision to NOT dispatch additional Copilot feedback this cycle reflects bottleneck accommodation: 6 dispatches awaiting assignment is already a substantial backlog. Adding more would compound the queue without adding parallel work surface (Eva is the bottleneck, not Copilot capacity).

### 8. Cycle 95 plan

Per `ITERATION-UNTIL-APPROVAL`, cycle 95 substantive focal options:

1. **Continue prototype scaffolding** MEDIUM PRIORITY — build the next-smallest A-shared crate. Candidates: `phase-transition-check` (state machine validation, likely ~300-400 LOC), `prompt-contract-check` (CI test for prompt drift, likely ~400-500 LOC). Both are within A's stated per-crate range.
2. **Pause prototype scaffolding; revert to second-iteration sharpening** MEDIUM PRIORITY — apply cycle 91-92 sharpening pattern to remaining first-iteration risks: A's tool-registry-growth risk; B's coordination-overhead-magnitude; C's plan-authoring-discipline-conditional risk.
3. **Wait for cycle 93 dispatch returns** LOW-MEDIUM PRIORITY — if Eva assigns cycle 93 dispatches to Copilot before cycle 95 fires, cycle 95 can integrate returned critique.
4. **Update Candidate A document with cycle-93+94 measurement evidence** MEDIUM PRIORITY — propagate the test-code amplification ratio refinement (1.7× average) and the dependency-footprint risk recharacterization (one-time, not per-crate) into A's `## Migration cost from v1` section.
5. **Bounded-mechanical fallback** — close absorbed dispatches if any deliver.

**Cycle 95 substantive focal default:** option (1) `phase-transition-check` — third crate measurement broadens evidence basis from 2 to 3 instances and tests a structurally different crate type (state-machine validation vs file write). OR option (4) document update — the cycle-93+94 measurement evidence is now substantial enough to warrant propagation into the candidate document. Either advances the candidate-selection-readiness state. Option (2) sharpening as fallback if prototype scaffolding hits unexpected scope.

**Trade-off note:** option (4) is the most candidate-selection-relevant if cycle 95 has limited compute; option (1) extends the empirical evidence base; both are valid. If cycle 95 compute permits paired execution, option (1) + (4) together is the highest-value combination.

## What surprised me / what I noticed

- **Dependency-footprint risk is bounded after first crate, not per-crate.** Cycle 93's risk #3 framing (Cargo.lock churn) implied per-crate accretion. Cycle 94's measurement refutes that: subsequent crates using {clap, serde, serde_json, tempfile} add zero transitive deps. This is a substantial cost reframe. **Implication for A's candidate document**: the dep-stack is paid once at the first crate; subsequent crates are dep-free if they converge. Workspace-shared dependency convention reinforces this naturally.

- **Test-code amplification ratio widened from 1.5× to 1.9× across 2 crates.** This is direction-supporting for cycle 93 risk #1, but also magnitude-widening: aggregate test-LOC implications scale up. The 1.9× is partly explained by the v2-cycle-history-append crate's larger surface (more CLI options, more validation paths, more error cases) — more surface area means more tests. Risk: if subsequent crates have similar surface area, A's aggregate LOC including tests is closer to ~12000 than ~7500. **Implication**: A's "~3000-4500 LOC aggregate" framing should be explicitly clarified as production-only OR upgraded to "~7000-12000 with tests."

- **Refuse-overwrite as a structural append-only enforcement primitive.** Setting up the refuse-overwrite check at the tool level (rather than relying on caller convention) makes A's Axis 4 (git-as-substrate) more robust. The tool itself enforces the invariant; an orchestrator bug or rogue manual edit cannot accidentally overwrite a cycle-history entry. This is a small but real architectural improvement vs v1 where state-mutation conventions were prompt-level (a procedural step the orchestrator was supposed to follow). Cycle-93's `tool-registry` is read-only so the question didn't arise; cycle-94's `cycle-history-append` is the first write tool, and refuse-overwrite establishes a pattern other v2 write tools should follow.

- **CLI-field-overrides-JSON precedence is intentional.** A caller using `--from-json file.json --field model=overrideX` gets the override semantically. This is useful when the JSON payload is mostly fixed but one field changes per invocation (e.g., `started_at` injected by a wrapper). Cycle 94's integration tests document this precedence explicitly so future maintainers know it's by design.

- **Pass-through schema is forward-compatible without versioning.** The tool only enforces 3 required fields and lets the rest pass through. New cycle-history schema fields can be added at the orchestrator level without tool change. This is a deliberate looseness — schema strictness is a future migration if downstream tools demand it. For v1 prototype, the looseness is correct.

- **The 0-new-transitive-deps observation is itself a workspace-architecture insight.** If A's remaining 7 crates converge on the same dep stack ({clap, serde, serde_json, tempfile}), the workspace dependency footprint is bounded. This argues for a `[workspace.dependencies]` convention in the eventual v2 workspace Cargo.toml — declare common deps at workspace level, members reference them with `{ workspace = true }`. Defer to the cutover phase, but the pattern is now visible.

- **Build time held steady (~1.0s).** Both crates build in ~1.0-1.2s release mode after dep cache is warm. This is a workspace property: shared deps mean only the crate's own code recompiles. Aggregate build time for A's full 9-crate set is plausibly bounded at ~10s release if all crates are similar in scope.

## Honest reflection (per F1 corrective)

Cycle 94 is **1 substantive activity** (Phase 3 prototype-scaffold-only — second crate measurement, cycle-history-append) yielding 1 Rust crate (~804 LOC across 4 files including tests + wrapper) + 31 passing tests + 1 _notes documentation file + 1 journal entry + Cargo.lock update (1 entry, 0 new transitive deps). Per F1 corrective: components are deliverable form of substantive activity; documentation IS the activity. Per F2 corrective: 1 prototype crate from 1 activity, NOT "31 tests = 31 ways improved." Per F3: 14 functional-class shapes at 34 instances (or 15 shapes if the prototype-only shape is counted as #15 distinct from cycle 93's paired-execution shape #14). Per F4: HARDENED/TESTED/NOVEL is for redesign-process methodology only. Per F5: lexicon (refuse-overwrite as structural append-only enforcement, dependency-footprint as one-time-not-per-crate, test-code amplification at 1.9×, pass-through schema forward-compatibility) as documented learning. Per H-Q(a) anti-inheritance: cycle 94 prototype scaffold grades empirical-observation-grounded against second-crate measurement vs cycle 93's first-crate measurement. Per J-Q(a) generalization-level discipline: HARDENED at 12 instances.

**Self-congratulation audit.** Cycle 94 prototype scaffold could be over-stated as "validates A's full migration cost claim across 2 crates." More honest: validates the per-crate scope claim for 2 of 9 crates, both at the smaller end of A's tool surface; aggregate-LOC claim depends critically on whether the larger crates (`wiki-search`, `boot-phase`, `close-phase`) come in similarly bounded. The 2-instance evidence is direction-supporting, not magnitude-validating. Risk #2 from cycle 93 (`wiki-search` likely largest) remains the structural concern not addressed by cycle 94.

**Specifically NOT validated:**
- A's "~3000-4500 LOC aggregate" production-only — cycle 94 evidence suggests trajectory may land below this.
- Aggregate-with-tests claim — cycle 94 widens the test ratio, suggesting actual aggregate-with-tests may exceed ~12000.
- Per-crate scope for `wiki-search`, `boot-phase`, `close-phase`, `gardening-sweep` — these are the structurally-larger crates.
- Build-time aggregate for full 9-crate set — extrapolation from 2 measurements is weak.
