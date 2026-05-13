---
cycle: 139
date: 2026-05-13
focus: Phase 3 prototype scoping — first cycle of post-Q7-resolution work; tool→primitive reconciliation + minimal-end-to-end ordering proposal
forward-priority-honored: cycle 138 #2 (Phase 3 prototype scoping); priority #1 (audit cycle 219) NOT AVAILABLE
substrate-cycles: 29 (cycles 111-139 non-per-candidate-sharpening continued; cycle 139 begins Phase 3 scoping arc)
bottleneck-async-cycles: 51 (cycles 78-139)
---

# Cycle 139 — Phase 3 prototype scoping

## Setup

Cycle 138 closed 2026-05-13 ~21:30 UTC with `input-from-eva` [#2930](https://github.com/EvaLok/schema-org-json-ld/issues/2930) absorbed: Q7 resolved option (c); Candidate B selected; Phase 2 checkpoint APPROVED; Phase 3 prototype work pre-approved. Eva named 4 load-bearing primitives (`role-driver`, `channel-router`, `super-step-boundary`, `reconciler-event-processor`) and a minimal end-to-end entry point (planner → executor → curator → reconciler over typed channels). 5 no-regret carryover crates named per cycle 126 [audit#462](https://github.com/EvaLok/schema-org-json-ld-audit/issues/462) D1 framing; 2 orchestration-hub crates re-evaluation; 2 unbuilt crates open-questioned.

At cycle 139 session-start (2026-05-13 22:24 UTC; ~52 minutes post cycle 138 session-end), audit HEAD is still `72cda15` from cycle 218 (2026-05-13 04:32 UTC). Cycle 219 expected ~04:00 UTC 2026-05-14 (~5.5 hours from cycle 139 session-start). Cycle 138 priority #1 NOT AVAILABLE; cycle 138 priority #2 (Phase 3 prototype scoping) elevates to cycle 139's substantive focal.

**Cycle 139's scope: scoping, not building.** Per cycle 138's anti-overstatement audit, Phase 3 prototype building requires "careful design starting from B's body + Eva's named primitives + the existing v2 crate carryover discipline." Cycle 139 reads B's body in detail, reconciles its 12-tool surface against Eva's 4 named load-bearing primitives + 5 no-regret carryovers, and proposes an ordering for the minimal end-to-end multi-agent cycle. Cycle 139 does NOT build any new Rust crate; cycle 140+ work executes on the scoping.

This is the **25th consecutive cycle of HONORING named forward priority** (cycles 115-139). Cycle 139 is the **51st consecutive bottleneck-asynchronous cycle** (cycles 78-139) and the **29th consecutive non-per-candidate-sharpening cycle** (cycles 111-139 — cycle 139 is Phase 3 scoping, not Phase 2 candidate-comparison substrate, but is contiguous in non-per-candidate-sharpening discipline).

## What Eva's directive named (recap, not re-derived)

**4 load-bearing primitives (cycle 138 [#2930](https://github.com/EvaLok/schema-org-json-ld/issues/2930)):**
- `role-driver` — per-role session orchestration (load context, invoke session, write output)
- `channel-router` — typed-channel reducer-rule application + super-step boundary writes
- `super-step-boundary` — super-step transition state machine
- `reconciler-event-processor` — Axis 12 inbound-channel processing

**Minimal end-to-end entry point:** planner → executor → curator → reconciler over typed channels — *before* the full skill suite.

**5 no-regret carryover crates (per cycle 126 [audit#462](https://github.com/EvaLok/schema-org-json-ld-audit/issues/462) D1 framing):** `v2-tool-registry`, `v2-cycle-history-append`, `v2-phase-transition-check`, `v2-wiki-search`, `v2-gardening-sweep`.

**2 orchestration-hub re-evaluate crates:** `v2-boot-phase`, `v2-close-phase` (orchestration-hub bundling is single-orchestrator-specific; multi-agent decomposition needed).

**2 open-questioned unbuilt crates:** `detect-abandoned-cycles`, `prompt-contract-check` (single-orchestrator-shaped; multi-agent topology may need different shapes).

**Empirical anchor:** judgment-only at cycle 138 (B's 14000-28000 LOC migration is Eva's judgment); first Phase 3 measurement establishes the B-specific anchor.

## Tool surface reconciliation: B body's 12 tools × Eva's 4 named primitives × 5 carryovers

B's body lists 12 tools under "Tool surface implied" (lines 73-85). Mapping each against Eva's named primitives + carryover status:

| B body tool | Eva-named load-bearing? | No-regret carryover from A? | Orchestration-hub re-evaluate? | Open-questioned? | Minimal end-to-end critical-path? |
|---|---|---|---|---|---|
| `role-driver` | **yes** | no | no | no | **yes** |
| `channel-router` | **yes** | no | no | no | **yes** |
| `super-step-boundary` | **yes** | no | no | no | **yes** |
| `reconciler-event-processor` | **yes** | no | no | no | **yes (stub form)** |
| `branch-manager` | no | no | no | no | no (Axis 4 — what-if branches; deferred) |
| `skill-loader` | no | no | no | no | no (Axis 6 — skills; deferred) |
| `plan-lifecycle` | no | no | no | no | no (Axis 5 — plans-as-artifacts; deferred) |
| `per-agent-memory` | no | no | no | no | no (Axis 3 — memory subsystem; deferred) |
| `consolidate-with-score-gate` | no | no | no | no | no (Axis 3 — score-gated; cluster H sub-shape 2 HIGH cost; deferred) |
| `gardening-sweep` | no | **yes** (`v2-gardening-sweep`) | no | no | no (Axis 10 — gardening; deferred until curator's continuous-background work begins) |
| `wiki-search` | no | **yes** (`v2-wiki-search`) | no | no | no (Axis 3 — top-k retrieval; deferred) |
| `prompt-contract-check` | no | no | no | **yes** (under B, decomposes per-role + cross-role) | no (Axis 8 — CI check; deferred) |

**Plus the 5 no-regret carryover crates from A's substrate:**

| Crate | Cycle of first measurement | Current LOC (cycle 136 measurement) | Role under B | Minimal end-to-end critical-path? |
|---|---|---|---|---|
| `v2-tool-registry` | 93 | 370 (originally 231) | Foundation for `skill-loader` (deferred); also per-role tool discovery at role-driver init | no (no skills in minimal end-to-end; role-driver init can hardcode the small initial tool set) |
| `v2-cycle-history-append` | 94 | 435 (originally 268) | Invoked by curator at session-close for cycle history append | **possibly yes** (if curator's session-close discipline includes append; minimal end-to-end may include this) |
| `v2-phase-transition-check` | 122 | 890 (originally 638) | Axis 8 mechanical-enforcement; CI check, not runtime | no (CI check; not runtime path for minimal end-to-end) |
| `v2-wiki-search` | 127 | 1615 (originally 931) | Invoked by executor/curator for retrieval | no (no retrieval queries in minimal end-to-end) |
| `v2-gardening-sweep` | 132 | 1532 prod (cycle 132 COMPLETE measurement) | Invoked by curator's continuous-background gardening | no (deferred until curator's continuous-background discipline begins) |

**Plus the 2 orchestration-hub re-evaluate crates:**

| Crate | Cycle of first measurement | Current LOC | Re-evaluation under B |
|---|---|---|---|
| `v2-boot-phase` | 124 | 883→1343 current | Decomposes: per-role boot (4 separate boots) is absorbed by `role-driver` at session-init; orchestration-hub bundling does NOT carry as a monolithic crate |
| `v2-close-phase` | 137 | 1476 | Decomposes: per-role close (4 separate closes) is absorbed by `role-driver` at session-end; consolidated session-close-with-journal-write is absorbed by curator's super-step (which is the LAST super-step in the minimal end-to-end). Orchestration-hub bundling does NOT carry as a monolithic crate |

**Plus the 2 open-questioned unbuilt crates:**

| Crate | Single-orchestrator shape | B (multi-agent) shape |
|---|---|---|
| `detect-abandoned-cycles` | One watchdog detecting abandoned single-orchestrator cycles | Per-agent watchdog (4 watchdogs, one per role) + cross-agent coordinator detecting cross-role stalls. Cycle 139 does NOT commit to a specific shape; cycle 144+ design work after minimal end-to-end is running |
| `prompt-contract-check` | One prompt's structural invariants | 4 per-role prompt contracts (`planner-prompt-contract` + `executor-prompt-contract` + `curator-prompt-contract` + `reconciler-prompt-contract`) + 1 cross-role coordination invariants check. B body line 84 named directly: "`prompt-contract-check` runs per-role; cross-role coordination invariants tested via integration tests." Cycle 145+ work after 4 role prompts authored |

## Minimal end-to-end multi-agent cycle scope

Per Eva's directive, the **first goal** of Phase 3 is a minimal end-to-end multi-agent cycle (planner → executor → curator → reconciler over typed channels). What this contains at the implementation level:

### 4 typed channels

- `plan-channel` — planner writes; executor reads. Schema: `{ "cycle": N, "substantive-focal": "...", "per-role-tasks": { ... } }`.
- `work-channel` — executor writes; curator reads. Schema: `{ "cycle": N, "artifacts-written": [...], "decisions-recorded": [...] }`.
- `memory-channel` — curator writes; persists across cycles. Schema: `{ "consolidated-insights": [...], "anti-patterns-noted": [...] }`.
- `inbound-channel` — reconciler writes; planner reads at next-cycle start. Schema: `{ "eva-responses": [...], "audit-posts": [...], "dispatch-returns": [...] }`.

Each channel has a per-channel file at `state/channels/<name>.json` (per B body line 13 + Axis 2 commitment). Per-channel reducer rules apply at super-step boundaries (per B body line 13 + Axis 8 commitment).

### 4 reducer rules (one per channel; minimal-viable)

- `plan-channel-reducer`: planner-writes-only at planner super-step boundary; append-only history at `state/channels/plan-channel-history.json`.
- `work-channel-reducer`: executor-writes-only at executor super-step boundary; append-only history.
- `memory-channel-reducer`: curator-writes-only at curator super-step boundary; append-only history. (Consolidation logic NOT in minimal end-to-end; score-gated consolidation is HIGH cost per cluster H sub-shape 2 — deferred.)
- `inbound-channel-reducer`: reconciler-writes-only at reconciler super-step boundary; append-only history. Minimal version: handles empty inbound (no Eva responses or audit posts to process) gracefully.

### 4 minimal role prompts (one per super-step)

Each role's prompt at `prompts/v2/<role>-prompt.xml`. Per B body line 23 + Axis 13 commitment: each role's prompt is **small** — the role's identity + the role's super-step responsibility + judgment-call decisions for the role; the bulk of procedure lives in harness (role-driver + channel-router + super-step-boundary).

Minimal role responsibilities (cycle 1):

- **Planner prompt:** read previous cycle's memory-channel + inbound-channel; decide substantive-focal for this cycle; write plan-channel artifact.
- **Executor prompt:** read plan-channel; do the substantive work named in the plan (could be authoring an artifact, dispatching, etc.); write work-channel artifact recording what was done.
- **Curator prompt:** read work-channel; reflect on what happened this cycle; write memory-channel artifact + append cycle history (invokes `v2-cycle-history-append`).
- **Reconciler prompt:** poll inbound surfaces (cron-internal: input-from-eva pull, audit-repo cursor); write inbound-channel artifact (potentially empty in cycle 1).

**No skills in cycle 1.** The minimal end-to-end runs without the skill loader, without per-agent memory subsystem, without plans-as-artifacts directory, without branching checkpoints. The full Phase 3 build adds those incrementally.

### 4 Eva-named primitive crates (minimal-viable, not complete)

- `tools/rust/crates/v2-role-driver/` — per-role session orchestration:
  - load role context (role prompt + previous super-step's channel outputs)
  - invoke claude-code session with the role's prompt
  - capture session output
  - write to the role's output channel via `channel-router`
- `tools/rust/crates/v2-channel-router/` — typed-channel I/O with reducer-rule discipline:
  - read channel state from `state/channels/<name>.json`
  - apply reducer rule to incoming write
  - persist new state + append-only history
- `tools/rust/crates/v2-super-step-boundary/` — super-step transition state machine:
  - track current super-step (planner / executor / curator / reconciler)
  - on super-step-end: trigger per-role close + advance to next super-step
  - cycle-end: trigger curator-led session-close (journal append + history commit)
- `tools/rust/crates/v2-reconciler-event-processor/` — minimal stub:
  - poll cron-internal sources (input-from-eva label + audit-repo cursor)
  - emit typed deltas to inbound-channel (or empty if no events)
  - cycle 1 minimal version: just poll + write empty inbound-channel artifact

### Cycle-runner harness rewrite

The existing `tools/rust/crates/cycle-runner/` is preserved per preserved-primitive but its body needs rewrite for B's multi-agent topology. The rewrite SCOPE per B body line 58: cycle-runner becomes a per-role driver invoking 4 role sessions in sequence (planner → executor → curator → reconciler) via super-step-boundary's transition state machine.

Cycle-runner rewrite is forbidden-zone (`.github/workflows/` adjacent) and requires PR per `direct-push-zones` in this prompt's SECTION 2. Cycle 145+ work likely.

## Phase 3 prototype ordering proposal

**Dependency graph:**

```
typed-channel schema (foundational)
   ↓
v2-channel-router (writes via reducer rules)
   ↓
v2-super-step-boundary (depends on channel-router for writes)
   ↓
v2-role-driver (depends on channel-router for I/O + super-step-boundary for transitions)
   ↓
v2-reconciler-event-processor (depends on inbound-channel from channel-router)
   ↓
4 minimal role prompts (depend on role-driver's invocation contract)
   ↓
cycle-runner harness rewrite (integrates all of the above)
   ↓
first end-to-end run + first measurement
```

**Proposed cycle ordering (cycle 140-146; not committed — cycle 140 entry decision):**

- **Cycle 140 — `v2-channel-router` scaffold.** Typed-channel schema + per-channel file format + 4 reducer rules (plan/work/memory/inbound). Estimated SCAFFOLD: ~600-900 LOC prod + ~400-600 LOC tests (mid-range of cycle 122-137 SCAFFOLD measurements: tool-registry 231 prod, cycle-history-append 268 prod, phase-transition-check 638 prod, wiki-search 484 SCAFFOLD prod, gardening-sweep 614 SCAFFOLD prod, close-phase 1125 SCAFFOLD prod). Direct push to `tools/rust/crates/v2-channel-router/`.
- **Cycle 141 — `v2-super-step-boundary` scaffold.** Transition state machine (planner→executor→curator→reconciler) + per-super-step entry/exit hooks + cycle-end consolidated close hook. Estimated SCAFFOLD: ~500-800 LOC prod + ~300-500 LOC tests.
- **Cycle 142 — `v2-role-driver` scaffold.** Per-role context load + session invocation + output write. Estimated SCAFFOLD: ~700-1000 LOC prod + ~400-600 LOC tests. (Highest-LOC of the 4 named primitives because it orchestrates session invocation, which has the most surface.)
- **Cycle 143 — `v2-reconciler-event-processor` minimal stub.** Cron-internal polling (input-from-eva pull + audit-repo cursor read) + typed-delta emit + empty-inbound-graceful handling. Estimated SCAFFOLD: ~400-700 LOC prod + ~300-500 LOC tests. (Smaller than the others because cycle 1 is minimal; cycle 150+ adds Eva-response classification + audit-post classification + dispatch-PR-merge handling.)
- **Cycle 144 — 4 minimal role prompts at `prompts/v2/<role>-prompt.xml`.** ~500-800 LOC per prompt minimal (smaller than v1's ~6000 LOC prompt because procedure is in harness, not prompt). Direct push allowed per `prompts/v2/` zone.
- **Cycle 145 — cycle-runner harness rewrite + integration test.** PR-required (workflow-adjacent forbidden zone); Eva merges.
- **Cycle 146 — first end-to-end run + first measurement.** Run on a small known workload (e.g., a small candidate-sharpening task analogous to cycle 92's substantive focal). Apply cycle 103 + cycle 106 + cycle 108 protocols for per-role decision count + coordination overhead + iteration events. Establish B's empirical anchor (Eva's directive: judgment-only at cycle 138; first Phase 3 measurement is what produces the anchor).

**Caveats on ordering proposal:**

- Cycle 122-137 measurement substrate produced scaffold→complete arc magnitudes ranging +25% (orchestration-hub close-phase SCAFFOLD-vs-prediction) to +149% (detector-walker gardening-sweep COMPLETE delta) — cycle 132 NOVEL@1 `magnitude-prediction-precision-is-shape-dependent-not-flat` says these estimates have shape-family-conditional precision. The 4 named primitives are a NEW shape family (multi-agent topology infrastructure) with no prior anchor; estimates carry larger uncertainty than current 4-arc precision data implies.
- Each crate's scaffold→complete arc may take 2 cycles (cycle 123-124 boot-phase, cycle 131-132 gardening-sweep, cycle 136-137 close-phase pattern) — so the SCAFFOLD-only ordering above may need cycles 147-150 for completion arcs before measurement. OR measurement at cycle 146 happens with SCAFFOLDs only and completion arcs interleave with measurement cycles.
- Cycle 140 entry decision may revise this ordering — e.g., combining cycles 140-141 (channel-router + super-step-boundary tightly coupled) into a single cycle if scaffolds are small enough.
- `v2-cycle-history-append` (no-regret carryover; already 435 LOC current) is invoked by curator at session-close; no new crate work needed for this primitive in the minimal end-to-end. Its invocation contract may need a thin adapter layer in role-driver's curator-super-step.

## What cycle 139 DOES NOT do (anti-overstatement audit)

- **Cycle 139 does NOT build any new Rust crate.** Scoping only. Cycle 140+ work executes on the scoping. No `cargo` invocations this cycle.
- **Cycle 139 does NOT modify B's body** beyond what cycle 138 already did. The SELECTED block at top of `B-decomposed-multi-role.md` is cycle 138's addition; cycle 139 leaves the body untouched. Per journal-immutability discipline, the cycle 90-118 authoring substrate + cycle 138 SELECTED block is preserved; Phase 3 scoping lives in new `_notes/` artifacts that build on B without retroactively rewriting it.
- **Cycle 139 does NOT modify `2-selection.md`.** Cycle 120 L2 constraint preserved (single Q7-resolution pointer note at top of `2-selection.md` from cycle 138; body untouched). No cycle 139 update needed.
- **Cycle 139 does NOT propagate to Eva-facing surfaces** beyond this `_notes/` file. The Status blocks at top of `2-selection-summary.md` / `2-candidates/README.md` / `2-design-framework.md` / `B-decomposed-multi-role.md` from cycle 138 are the active record. Eva-facing propagation happens when Phase 3 prototype produces measurable substrate (cycle 146+ likely); cycle 139 scoping doesn't trigger propagation.
- **Cycle 139 does NOT commit to a specific cycle ordering for Phase 3.** The proposal above is the orchestrator's best estimate at cycle 139; cycle 140 entry decision refines based on what's actually buildable and what dependencies surface during cycle 140's substantive work.
- **Cycle 139 does NOT make claims about Phase 3 cycle counts to first end-to-end run.** Proposes cycle 140-146 (7 cycles) but acknowledges shape-family-conditional precision implies wider uncertainty than current 4-arc data suggests. Cycle 122-137 produced 4 arcs in 15 cycles; 7 cycles for 4 primitives + 4 prompts + 1 harness rewrite + 1 integration test + 1 first run is plausible but not committed.
- **Cycle 139 does NOT predict whether B's per-cycle decision overhead will fall within the cycle 103 protocol's PASS / PASS-WITH-NOTE / PARTIAL-FLAG / REFUTED thresholds.** Eva accepted higher per-cycle overhead as cost of arriving at durable system per cycle 138 directive; cycle 139 doesn't measure.
- **Cycle 139 does NOT measure anything.** Eva's directive: judgment-only at cycle 138; first Phase 3 measurement is at cycle 146+ (per proposed ordering). Cycle 139 is scoping; no measurement.
- **Cycle 139 does NOT engage the cycle 137 candidate-emergent observation `within-shape-family-scaffold-to-complete-delta-tighter-than-scaffold-prediction`** (no measurements this cycle; cross-shape-family contradiction qualifier preserved).
- **Cycle 139 does NOT advance any of the 3 cycle 138 candidate-emergent observations** (cycle 139 is scoping; substrate that would advance these observations comes from Phase 3 building, not from scoping decisions).
- **Cycle 139 does NOT resolve the design of any individual role's prompt.** Each role's prompt design is its own work item at cycle 144 (proposed); cycle 139 names minimal responsibilities only.
- **Cycle 139 does NOT design the cycle-runner harness rewrite.** Cycle 145+ work; cycle 139 names the rewrite scope per B body line 58.
- **Cycle 139 does NOT invalidate any pre-cycle-138 substrate.** The 7-of-9 v2 crate measurements + 4 scaffold→complete arc data + 49 cycles bottleneck-async substrate + 24 cycles forward-priority honoring all preserved. Cycle 139 builds on what was preserved.
- **Cycle 139 does NOT commit to specific LOC estimates for any of the 4 Eva-named primitives.** The estimates ~400-1000 LOC per crate SCAFFOLD are derived from cycle 122-137 precedent; per cycle 132 `magnitude-prediction-precision-is-shape-dependent-not-flat`, these estimates carry shape-family-conditional precision and the multi-agent-topology family has no prior anchor.
- **Cycle 139 does NOT claim Eva's 14000-28000 LOC judgment range is the right anchor for the minimal end-to-end subset.** The minimal end-to-end (4 named primitives + 4 minimal prompts + cycle-runner rewrite) is plausibly 3000-6000 LOC — a small fraction of B's full surface. The 14000-28000 range covers the FULL Phase 3 build (skills + memory + branching + plans-as-artifacts + 8 auxiliary tools + 20-40 skill crates); the minimal end-to-end is the smallest-viable subset of that. Cycle 139 does NOT make claims about whether full B reaches 14000-28000; first Phase 3 measurement at cycle 146+ produces the first data point.

## Pattern updates

- **`discretionary-departure-from-forward-going-commitment`** HARDENED-at-4 cycle 114 → **25 honorings cycle 139** (cycles 115-139; 29 cycles of substrate at cycle 139 exit). Cycle 139 honors cycle 138 priority #2 (Phase 3 prototype scoping) as cycle 139's substantive focal (priority #1 audit cycle 219 NOT AVAILABLE).
- **Cycle 138's 3 NOVEL@1 candidate-emergent observations** (`directive-resolution-can-override-substrate-direction`, `substrate-production-discipline-and-recommendation-direction-correctness-are-different-axes`, `meta-empirical-evidence-from-system-operation-can-falsify-design-substrate`) — not advanced cycle 139 (scoping; no substrate that would produce second-instance evidence). Carry forward at NOVEL@1.
- **Cycle 137's NOVEL@1 candidate-emergent observation** `within-shape-family-scaffold-to-complete-delta-tighter-than-scaffold-prediction` — not engaged cycle 139 (no measurements; cross-shape-family contradiction qualifier preserved).
- **Cycle 136's NOVEL@1 candidate-emergent observation** `crates-grow-post-initial-measurement` — not advanced cycle 139 (no new measurements; observation remains methodology-relevant for Phase 3 measurements at cycle 146+).
- **`scaffold-partial-as-measurement-primitive`** HARDENED@4 cycle 137 — methodology preserved for Phase 3 measurement work; cycle 122-137 substrate is the basis for cycle 140+ scaffold predictions.
- **`subprocess-invocation-over-http-client-for-dependency-discipline`** REINFORCED-AT-5-BOUNDARIES cycle 137 — methodology preserved for cycle 140+ crate construction; new crates likely use clap + serde + serde_json + tempfile-dev stack (same as cycle 132 6-crate baseline).
- **NEW potential candidate-emergent observation cycle 139** `directive-named-load-bearing-primitives-are-not-the-full-tool-surface` — Eva's 4 named primitives are subset of B body's 12-tool surface (4 of 12); the other 8 are auxiliary and deferred. The directive resolves the load-bearing question without committing to the full surface. Sibling to cycle 138's 3 observations at directive-resolution scope. NOT promoted (single observation; whether the pattern recurs in Phase 3 directives is the test). Preserved as candidate-emergent for future reference.

## Forward priorities for cycle 140+

Cycle 138 priority #2 (Phase 3 prototype scoping) is **CLOSED at the scoping level** cycle 139. Cycle 138 priorities renumber:

1. **Audit cycle 219 critique absorption** — still expected ~04:00 UTC 2026-05-14 (~5.5 hours from cycle 139 session-start, ~13 hours from cycle 140 session-start if cycle 140 fires on the normal cadence). Cycle 218 introduced [audit#465] M1 sharpening; cycle 219 will either close [audit#465] or extend it. Natural priority #1 if it lands before cycle 140 substantive focal commits.
2. **Phase 3 prototype building begins.** Cycle 140 natural focal: `v2-channel-router` scaffold (foundational typed-channel infrastructure; first dependency on which other primitives build). Alternative cycle 140 entry decisions:
   - **`v2-channel-router` scaffold** (proposed): foundational; underlies super-step-boundary and role-driver
   - **`v2-role-driver` scaffold first**: role-driver is the most user-visible primitive (it's what invokes the LLM session); building it first gives an early feel for the architecture but has dependencies on channel-router and super-step-boundary that would need stubbing
   - **Combined `v2-channel-router` + `v2-super-step-boundary` scaffold**: if scaffolds are small enough; tightly coupled primitives
3. **Cycle 138 candidate-emergent observation reinforcement opportunities** (3 NOVEL@1 candidates; second-instance substrate may emerge organically from Phase 3 work — `directive-resolution-can-override-substrate-direction` may have second instance if a future Phase 3 directive overrides a Phase 3 substrate buildup).
4. **`v2-phase-transition-check` test failures triage** (carry-over from cycle 137 priority #4). Under B, this crate becomes Axis 8 CI; not blocking minimal end-to-end. Triage can defer to cycle 147+ when CI sweep work begins.
5. **Two open-questioned crates under B** (`detect-abandoned-cycles`, `prompt-contract-check`) — design work for cycle 144+ after minimal end-to-end is running. Both decompose under B's multi-agent topology (per-agent watchdog + cross-agent coordinator; per-role contract + cross-role invariants).
6. **Phase 1 research deepening** (carry-over). Under B's selection, the higher-priority research targets are openclaw's per-agent state isolation (`~/.openclaw/agents/<agentId>/` directory structure) + Cognition's Managed Devins coordinator pattern (planner as goal-coordinator) + AutoGen Magentic-One topology + super-step semantics. Phase 1 may have new motivation under B.
7. **Cycle 120 L2 constraint preserved** (no recursive annotation of `2-selection.md` — preserved through cycle 139; expected to be preserved through Phase 3 and into Phase 4).

## Process honoring

- **25th consecutive cycle of HONORING named forward priority** (cycles 115-139; 29 cycles of substrate at cycle 139 exit).
- **51st consecutive bottleneck-asynchronous cycle** (cycles 78-139).
- **29th consecutive non-per-candidate-sharpening cycle** (cycles 111-139; cycle 139 is Phase 3 scoping — distinct from Phase 2 candidate-comparison substrate but contiguous in non-per-candidate-sharpening discipline since the candidate-comparison arc ended at cycle 138).
- **Cycle 120 L2 constraint preserved** — `2-selection.md` body untouched cycle 139 (the single Q7-resolution pointer note at top from cycle 138 is the only resolution-related touch).
- **Cycle 128 process-error lesson preserved** — `.scratch/` used inside repo for session-start comment body (via `Write` tool not heredoc); no parallel-batch cancellation cascade.
- **Cycle 133 process-error lesson preserved** — no cargo invocations cycle 139 (scoping cycle; no Rust crate work).
- **Cycle 134 process-error lesson preserved** — no audit-repo cross-mount paths in parallel batches cycle 139 (audit-repo state read via `gh api repos/EvaLok/schema-org-json-ld-audit/commits/HEAD` only; no clone needed).
- **Cycle 137 process-error lessons preserved** — no heredoc redirection (`Write` tool for all body files); no `cd /tmp` (no audit clone needed); no python3-inline state.json inspection.
- **Cycle 138 process-error discipline preserved** — `.scratch/` used inside repo for session-start body file; commit-message HEREDOC for the session-end commit (per cycle 137 lesson permits HEREDOC for commit messages, only forbids heredoc redirection to files).
- **Journal-immutability discipline preserved** (cycle 133 policy) — no edits to candidate A or C bodies, no edits to B's body beyond what cycle 138 did, no edits to historical `_notes/` files (cycle-111 through cycle-138 all preserved), no edits to absorption paragraphs in `2-selection-summary.md`. Cycle 139 adds a new `_notes/` file (this file); does not rewrite frozen historical record.

## Cycle 139 preserves

- **B's body** (`2-candidates/B-decomposed-multi-role.md`) — cycle 90-118 authoring substrate + cycle 138 SELECTED block at top. The 12-tool surface list at lines 73-85 is preserved (cycle 138 noted may diverge from Eva's named primitives — Phase 3 cycles will reconcile; cycle 139 produces a reconciliation table in this `_notes/` file rather than rewriting B's body).
- **5 no-regret carryover crates** — `v2-tool-registry` (370 LOC current), `v2-cycle-history-append` (435 LOC current), `v2-phase-transition-check` (890 LOC current; with cycle 137 test failures pending), `v2-wiki-search` (1615 LOC current), `v2-gardening-sweep` (1532 prod cycle 132 COMPLETE) — Eva's directive citing cycle 126 [audit#462](https://github.com/EvaLok/schema-org-json-ld-audit/issues/462) D1 framing preserved.
- **2 orchestration-hub re-evaluate crates** — `v2-boot-phase` (1343 LOC current) and `v2-close-phase` (1476 LOC current) — re-evaluation status from cycle 138. Cycle 139 names the decomposition (per-role boot/close absorbed by role-driver; consolidated session-close absorbed by curator's super-step) but does not delete the existing crates; they remain as substrate that may inform role-driver's session-init/close logic in cycle 142+ work.
- **2 open-questioned crates** — `detect-abandoned-cycles` (unbuilt) and `prompt-contract-check` (unbuilt) — open status from cycle 138 preserved. Cycle 139 names the multi-agent decomposition (per-agent watchdog + per-role contract) but does not build either crate.
- **Cycle 134 V2-era operational failure-mode evidence** — classifier-class A4 family (~50% silent-zero-output rate over cycles 203-216) + state-growth-axis (state.json 250KB hard limit; cycle 218 entry state at 262KB) carry forward as Phase 2 evidence for B's design. Both families need addressing in B's multi-agent topology (per-agent context isolation may reduce classifier-class compound risk; per-channel state surfaces are inherently smaller than monolithic state.json).
- **F1-F12 framework grounding** preserved (augmented by cycle 134 V2-era classifier-class + state-growth-axis families, not replaced). Under B, F1 (constraint accretion) is addressed by Axis 8 + Axis 13 fat harness; F7 (self-management dominance) is addressed by Axis 1 role-specialization (curator absorbs management surface); the empirical question of whether B's structural mitigations reduce F1/F7 firing-rates is what Phase 3 prototype measurement is supposed to answer.
- **Cycle 137 candidate-emergent observation** `within-shape-family-scaffold-to-complete-delta-tighter-than-scaffold-prediction` at NOVEL@1 with cross-shape-family contradiction qualifier (no cycle 139 work advances or refutes).
- **3 cycle 138 candidate-emergent observations** at NOVEL@1 (`directive-resolution-can-override-substrate-direction`, `substrate-production-discipline-and-recommendation-direction-correctness-are-different-axes`, `meta-empirical-evidence-from-system-operation-can-falsify-design-substrate`) — preserved at NOVEL@1; not advanced cycle 139.
- **PR #2877's calibration discipline** (verified workspace LOC calibration 38 v1 crates median 1081 mean 2118 within 0.05% per cycle 97). Phase 3 LOC estimates in this `_notes/` use cycle 122-137 v2 crate measurements (range 231-1615 prod LOC per crate) as the precedent set; the calibration discipline carries.
- **The four scaffold→complete arc measurements** (boot-phase +280, wiki-search +484, gardening-sweep +918, close-phase +351 LOC respectively) preserved as the substrate-as-measurement-primitive record. Phase 3 measurement work at cycle 146+ will produce arc #5 (channel-router) through arc #8 (reconciler-event-processor) for the 4 Eva-named primitives.
- **28 cycles of substrate** (cycles 111-138 non-per-candidate-sharpening) preserved; cycle 139 extends to 29 cycles.

---

**Author note (process-honoring transparency):** cycle 139 is the first Phase 3 scoping cycle. The orchestrator's prior reading of the redesign was structurally biased away from B (cycle 138 honest acknowledgment); cycle 139 is the first cycle where the orchestrator actively designs for B's selection. The scoping table above + ordering proposal are produced by the same orchestrator that previously recommended A > C >> B; the anti-overstatement audit is more explicit than usual to compensate. Eva's directive accepted the bias acknowledgment + preserved substrate value + named 4 specific load-bearing primitives — the scoping work flows from those named primitives, not from the orchestrator's prior recommendation arc.
