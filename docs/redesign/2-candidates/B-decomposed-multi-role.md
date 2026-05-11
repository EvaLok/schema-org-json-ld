# Candidate B: Decomposed Multi-Role

**Authoring cycle:** 90 (2026-05-07)
**Authored against:** [`2-design-framework.md`](../2-design-framework.md) v1.21+; [`clusters.md`](../1-research/clusters.md) M-item integration arc closure (cycles 86-89).

**One-line position:** Decompose the orchestrator session into a **small fixed team of role-specialized agents** (planner, executor, curator, reconciler) sharing typed channels, with branching checkpoints for what-if reasoning and a fat harness for role coordination. The candidate's central bet is that v1's single-orchestrator shape is the load-bearing limit on cluster G (role-asymmetric context), cluster H (post-session feedback / cross-session learning), and Axis 12 (reconciliation) — and that the failure modes those clusters address require dedicated roles, not just better tools.

## Position summary

- **Axis 1 (decomposition):** Small fixed team — 4 agents: **planner** (forward-look, candidate generation, cycle-N→N+1 hand-off), **executor** (substantive cycle work, dispatches, artifact writes), **curator** (memory consolidation, gardening, deslop, anti-pattern catalog maintenance), **reconciler** (Axis 12 inbound channels — Eva responses, audit posts, dispatch outputs, post-close mutations). Writes-stay-single-threaded enforced via typed-channel reducer rules (only one agent writes to each channel at any super-step). Adopts Cognition Apr 2026 + Voyager + AutoGen Magentic-One pattern.
- **Axis 2 (state representation):** Typed-channel-map (LangGraph style) — typed channels between roles + per-role checkpointer. Channels: `plan-channel`, `work-channel`, `memory-channel`, `inbound-channel` with per-channel reducer rules. Per-component file persistence backs each channel (`state/channels/<name>.json` with append-only history).
- **Axis 3 (memory shape):** Top-level architectural principle (PAI Principle 13) with per-agent memory channels — multi-mechanism per coordinate (Cognition I-C3): per-agent local memory (transient working state), per-agent long-term memory (consolidated insights, score-gated per cluster H sub-shape 2), and shared cross-agent memory channel (planner reads curator's consolidated insights; executor reads planner's plan-channel; reconciler reads inbound-channel). Top-k retrieval over LLM-generated descriptions (cluster B sub-shape 6) for shared memory.
- **Axis 4 (history substrate):** Branching checkpoints in-tree files — per-branch-named files (`state/branches/<branch-id>/checkpoints/<n>.json`) committed in main branch (per preserved-primitives constraint: in-tree files, not git-branches). Enables what-if reasoning across multi-cycle horizons (planner can fork a branch to explore alternative cycle-N+1 plans without committing to one). Promotion-decision criteria are encoded per-branch.
- **Axis 5 (plans-as-artifacts):** Yes — plans-as-artifacts directory structure (`docs/redesign/plans/active/<plan-id>.md`, `plans/completed/<plan-id>.md`, `plans/technical-debt/<plan-id>.md`). Plans authored by planner agent; executed by executor; reviewed by curator at completion; reconciled against post-close evidence by reconciler.
- **Axis 6 (extension shape):** Skills (PAI / oh-my-codex 39-skill model) — code + prompt + invocation contract. Each skill is a Rust binary + a prompt fragment + an invocation contract declaring inputs / outputs / side-effects. Skills are role-bound (planner-skills, executor-skills, curator-skills, reconciler-skills) but discoverable cross-role for collaboration.
- **Axis 7 (orchestration topology):** Multi-pattern coexisting — lead-worker for planning (planner is lead, executor is worker), peer-flow for execute-curate (executor and curator coordinate via channels), situational review (curator triggers review only when consolidation-score thresholds fire). Topology choice per super-step is encoded in the harness's transition policy.
- **Axis 8 (mechanical enforcement):** Behavioral promises + agent-affecting prose — CI on per-skill prompt + workspace-level discipline + per-channel reducer-rule schemas (TypeBox-style). `prompt-contract-check` runs per-role; cross-role coordination invariants tested via integration tests.
- **Axis 9 (iteration ceilings):** Both (loop + runtime) per role — per-skill `max_iterations` + per-role runtime budget per super-step. Aggregate per-cycle ceiling = sum of per-role budgets, bounded by cycle-level ~75 min.
- **Axis 10 (entropy mitigation):** Both — golden principles enforced by `prompt-contract-check` (Axis 8) + deslop pass at session-close run by curator + dedicated curator role for ongoing gardening across cycles (continuous-background discipline per cluster H sub-shape 3). Quality-grading rubrics encoded per-skill.
- **Axis 12 (reconciliation discipline):** Event-driven — dedicated **reconciler** role processes inbound channels. Reconciler subscribes to: GitHub Actions events on cross-repo audit commits (Actions trigger), input-from-eva pull (cron-internal), question-for-eva-response check (cron-internal), dispatch PR-merge events (Actions trigger). State-recompute on event arrival; reconciler emits typed deltas to other channels.
- **Axis 13 (harness-vs-session boundary):** Fat harness — heavy mechanical infrastructure for role coordination (channel routing, lane queues, super-step boundaries, per-role context-window setup, sandbox enforcement). Cycle-runner orchestrates per-role invocations; each role invocation is a separate session (4 sessions per cycle, sequenced in super-step order: reconciler → planner → executor → curator). Cycle-runner change scope: SUBSTANTIAL (rewrite). Per role, the prompt is **small** (a reference + role-specific judgment-call decisions); the bulk of procedure lives in the harness.

## Cross-axis commitments

- **Axis 1 × Axis 7:** Small-fixed-team enables multi-topology coexistence — lead-worker for planner+executor; peer-flow for executor+curator; situational invocation for reconciler. Each topology applies in different super-steps.
- **Axis 2 × Axis 3:** Typed-channel-map pairs naturally with memory-as-channel. Memory subsystem IS the channel infrastructure (with a memory-specific channel for the consolidated long-term store).
- **Axis 4 × Axis 2:** Branching checkpoints + typed-channel-map = LangGraph-style branching; each checkpoint is a snapshot of all channel states at a super-step boundary. In-tree files (per preserved-primitives) means each branch has a directory of checkpoint files committed in main.
- **Axis 12 × Axis 4:** Event-driven reconciliation pairs with branching checkpoints — reconciler can spawn a branch to explore "what-if this Eva response means X vs Y" before promoting to main branch.
- **Axis 12 × Axis 1:** Dedicated reconciler agent IS the small-fixed-team's instantiation of Axis 12's "dedicated reconciliation agent" cross-axis dependency.
- **Axis 13 × Axis 6:** Fat harness needs a richer extension story — skills (Axis 6) provide the discovery + invocation primitive that fat-harness uses to compose per-role capabilities. Skill manifest is the harness's enumeration substrate.
- **Axis 13 × Axis 8:** Fat harness implies more mechanical-enforcement surface area in code — per-channel reducer-rule schemas, per-role context-window invariants, per-super-step transition policy. CI surface is substantially larger than candidate A's.
- **Axis 13 × Axis 7:** Fat-harness implements Axis 7's multi-pattern situational-review by controlling when each role fires (vs every cycle). The transition policy IS situational invocation.
- **Constraint 8 × Axis 1:** Goal-driven within small-fixed-team requires explicit goal-coordination primitive. Adopts Cognition's Managed Devins coordinator pattern — planner is the goal-coordinator scoping per-cycle work for executor + curator + reconciler; planner enforces goal-coherence across parallel sub-tasks.

## Failure-mode addressing

- **F1 (constraint accretion):** Axis 8 + Axis 13 — fat harness extracts substantial procedural constraints from prompt to tools / per-skill specifications. Per-role prompts are small; cross-role coordination lives in harness. F1 is structurally addressed at higher leverage than candidate A.
- **F2 (Eva-response detection):** Axis 12 event-driven via dedicated reconciler — Eva-response detection is a first-class concern of one role, not a side activity of the orchestrator. Latency = event-arrival (sub-cycle) for Actions-triggered events; cycle-cadence for cron-internal pulls.
- **F3 (multi-candidate state drift):** Axis 2 typed-channel-map (single source of truth per channel) + Axis 4 branching checkpoints (explicit divergence) + Axis 12 reconciler reconciles against post-close evidence. Multi-candidate is structural, not failure mode — branches are explicit.
- **F4 (frozen-artifact lifecycle fragility):** Axis 4 branching checkpoints + Axis 5 plans-as-artifacts lifecycle (active/completed/technical-debt) + Axis 12 reconciler refreshes frozen artifacts via inbound-channel reconciliation.
- **F5 (state.json as procedural-leak):** Axis 2 typed-channel-map eliminates monolithic state file. Per-channel reducer rules separate concerns mechanically.
- **F6 (cyclomatic procedure depth):** Axis 7 multi-pattern with per-super-step transition policy + Axis 13 fat harness extracts deep procedure to deterministic code.
- **F7 (self-management dominance):** Axis 1 (role-specialization absorbs management surface — curator is the dedicated maintenance agent) + Axis 8 (mechanical enforcement) + Axis 9 per-role ceilings + Axis 13 fat harness. Aggregate effect: per-role decision count is small; aggregate per-cycle decision count may be higher than v1, but the cost is paid by specialized agents not the goal-coordinator.
- **F8 (abandonment cascades):** Axis 9 per-role loop + runtime ceilings + reconciler agent dedicated to detection-and-recovery + curator's continuous-background gardening detects stale state. Multi-layer defense.
- **F9 (adversarial-review treadmill):** Axis 7 multi-pattern + Axis 13 fat-harness controls when curator's review fires (situational, not every cycle) + Axis 1 dedicated curator role means review is a first-class concern but bounded to its super-step.
- **F10 (audit's value is broader read scope):** Audit-as-peer pattern preserved (Axis 12 reconciler reads cross-repo). Reconciler is the main-side counterpart to the audit-side reviewer, but cross-repo asymmetry preserved (audit retains its broader-read property).
- **F11 (post-close mutations):** Axis 4 branching checkpoints (append-only by construction) + Axis 12 reconciler reads back state at every super-step (no frozen-worklog drift). Multi-mechanism defense.
- **F12 (defense accretion catalog):** Axis 2 + Axis 4 + Axis 10 contribute. Curator's continuous-background gardening + golden-principles enforcement is the structural anti-accretion primitive (stronger than candidate A's minimal version).

## Preserved-primitives compliance

- **Journal:** preserved; remains long-term reflective channel. Curator agent owns journal-write at session-close (consolidated reflection across all 4 roles' super-step outputs).
- **Cycle-issue:** preserved; planner posts session-start summary at planner-super-step-start; curator posts session-end summary at curator-super-step-end. Multi-pattern topology means multiple sub-cycles within one cycle-issue boundary — sub-cycles are internal.
- **Question-for-eva / input-from-eva:** preserved; reconciler agent owns inbound-channel reconciliation. Cursor file `state/channels/inbound-channel.json` advances on event arrival.
- **Git-safety:** preserved; commit-must-be-pushed honored — branching checkpoints are in-tree files (per-branch-named files committed in main), not git-branches that might not be pushed. Per-channel writes commit + push in same super-step boundary.
- **Cycle-runner harness:** SUBSTANTIAL change scope (rewrite). Cycle-runner orchestrates 4 sequenced role invocations per super-step. Per-role invocation: load role-context, invoke role-session with per-role prompt + skill manifest, write role-output to channels, super-step-boundary commit. Specifics: cycle-runner becomes a per-role driver; new sub-binaries `role-driver`, `channel-router`, `super-step-boundary`, `branch-manager`, `skill-loader`, `reconciler-event-processor`. ~6+ new Rust crates.

## What this candidate gives up

- **Substantial migration cost** — not a drop-in evolution of v1; effectively a new system with a v1 compatibility layer at the edges.
- **High per-cycle decision overhead** at the system level (P3 risk) — though per-role decision count is small, aggregate per-cycle work across 4 agents may exceed v1's single-agent per-cycle work. Risk: if the redesign goal is "more cycles spent on schema work," this candidate inverts the direction.
- **Coordination overhead** — typed-channel routing, super-step boundary discipline, branching-checkpoint promotion criteria all require infrastructure that has zero current presence in v1. Phase 3 prototype effort is large (~6+ new Rust crates, several thousand LOC each).
- **Goal-coherence risk** — multiple agents working in concert can diverge from the goal-driven posture (constraint 8) without an explicit coordinator. The candidate adopts Cognition's Managed Devins coordinator pattern, but coordinator failure modes (planner makes a bad plan; executor follows it through; curator and reconciler don't catch the divergence) are a structural risk class v1 doesn't have.
- **Branching checkpoint complexity** — what-if branches need promotion criteria; promotion failures produce orphaned branches that the curator must garden. Cluster C sub-shape 3 (fork) is HIGH cost; this candidate adopts it.
- **Memory subsystem complexity** — per-agent memory + shared memory + multi-mechanism-per-coordinate is cluster B HIGH cost (sub-shapes 3 + 7). Candidate adopts.

## Tool surface implied

**New tools (net-add for cutover):**

- `tools/rust/crates/role-driver` — per-role session orchestration (load context, invoke session, write output)
- `tools/rust/crates/channel-router` — typed-channel reducer-rule application + super-step boundary writes
- `tools/rust/crates/super-step-boundary` — super-step transition state machine
- `tools/rust/crates/branch-manager` — branching-checkpoint creation, promotion, gardening
- `tools/rust/crates/skill-loader` — skill discovery + manifest validation + per-role binding
- `tools/rust/crates/reconciler-event-processor` — Axis 12 inbound-channel processing
- `tools/rust/crates/plan-lifecycle` — plans-as-artifacts directory management (active / completed / technical-debt)
- `tools/rust/crates/per-agent-memory` — per-agent memory channel + cross-agent shared memory
- `tools/rust/crates/consolidate-with-score-gate` — cluster H sub-shape 2 score-gated consolidation (HIGH cost — recall-frequency + query-diversity thresholds)
- `tools/rust/crates/gardening-sweep` — Axis 10 deslop pass with quality-grading rubrics (HIGH cost — full rubrics, not minimal)
- `tools/rust/crates/wiki-search` — top-k retrieval over LLM-generated descriptions
- `tools/rust/crates/prompt-contract-check` — per-role + cross-role CI
- Plus per-skill Rust crates (~20-40 skills across 4 roles)

**Existing tools preserved:**

- `cycle-runner` (SUBSTANTIAL rewrite — becomes role-driver harness)
- audit-related tools

**Existing tools deprecated:**

- `state.json` and anything that reads/writes it directly. Migration: typed-channel-map decomposition is per-concern + per-role; harder than candidate A's flat decomposition.

## Migration cost from v1

- **State migration:** decompose `state.json` into typed channels + per-channel files. Migration is non-trivial; some current state.json keys map to multiple channels (e.g., dispatch-queue spans inbound-channel for dispatch-poll events + work-channel for in-flight work). Risk: medium-to-high.
- **Tool migration:** ~12+ new Rust crates + ~20-40 skill crates. Aggregate net-add ~10000-20000 LOC. Multi-cycle effort to build out the full skill suite. **Cycle 97 absorption (PR #2877 lens-4 revised range):** PR #2877's tool-surface feasibility critique revised B's aggregate net-add upper bound to **14000-28000 LOC** based on (1) typed-channel-map infrastructure (`channel-router` + reducer schemas + boundary sync + coordination invariants) likely dominating LOC over role logic; (2) skill abstraction multiplying lifecycle overhead per skill manifest/contract; (3) `branch-manager` policy-heavy and test-heavy beyond "small crate" complexity. PR #2877 also notes B's own document at line 62 acknowledges "several thousand LOC each" for key infra — internally inconsistent with the 200-500 LOC default if applied literally. Cutover scope realism: PR #2877 estimates 16-30 cycles for ~32-52 callable surface units if quality is non-negotiable, sharpening B's vague "multi-cycle effort" framing. PR #2877's verified workspace calibration (38 v1 crates, median 1081 LOC, mean 2118 LOC) supports B's higher-LOC posture as more realistic per-crate-norm.
- **Prompt migration:** 4 role-specific prompts at `prompts/v2/<role>-prompt.xml`. Each ~30-40% the size of v1's prompt. Aggregate prompt content ~50% larger than v1 (4 role-specific prompts share less common context, but each is small individually). Eva installs all 4 + the cycle-runner.
- **Workflow migration:** Substantial. `cycle-runner` invokes 4 sequenced role-sessions. `.github/workflows/orchestrator.yml` updated. Forbidden-zone PR per redesign prompt SECTION 2 — Eva merges.
- **Journal/Cycle-issue migration:** zero (preserved); per-role contributions consolidated by curator at session-close.
- **Audit-repo coordination:** main-side reconciler is the new counterpart to audit; cross-repo discipline preserved but the discovery-primitive completeness work (audit#455 gap) lands more naturally in this candidate (reconciler processes audit-side `[audit-request]` events as first-class).
- **Phase 3 prototype effort:** large. Multi-cycle build-out of role-driver, channel-router, super-step-boundary, branch-manager, skill-loader before the first end-to-end cycle can run.

## P1-P6 evaluation criteria compliance (audit#454 absorption, cycle 89)

- **P1 (A↔B intersection coverage hard gate):** PASS. Cluster A super-step boundaries (Axis 7 multi-pattern with super-step semantics) + cluster B typed-channel-map writes at super-step end. Sub-pattern coverage: (1) sync-invariants-at-init via per-role context-window invariants; (2) state-commit at end-of-super-step via channel-router (DUAL-CAST per audit#454 D2 — the same primitive serves cluster A super-step boundary + cluster B reducer-rule discipline; doesn't count toward intersection-coverage criteria); (3) failure-record-write at retry-exhaustion via per-role retry exhaustion + reconciler failure-record write; (4) watchdog-release-with-state-cleanup via reconciler stuck-session detection + channel-state cleanup; (5) component-local persistence loaded at init via per-role per-channel checkpointer load. Sub-pattern (2) is dual-cast; sub-patterns (1) (3) (4) (5) are explicit; PASS.
- **P2 (cluster I substrate-fit weighting):** PASS — substrate-driven adoption. v1's substrate (GitHub-Actions-anchored, public repo, multi-actor with audit) genuinely benefits from harness-enforced policy (cluster I sub-shape 1) and quality-policy linters (cluster I sub-shape 2). The fat harness (Axis 13) makes cluster I sub-shape adoption load-bearing: the harness IS the policy enforcement substrate. Substrate-fit, not aspirational.
- **P3 (self-management-reduction axis):** PARTIAL-FLAG. Per-cycle decision count estimate: ~16 LOW + ~22 MODERATE + ~5 HIGH at the system level (aggregating across 4 agents). Compared to v1 baseline (16 LOW + 27 MODERATE + 7 HIGH if v1 inherited all 50 sub-shapes — v1 actually inherits 6 STRONG only): candidate adopts ~43 sub-shapes vs v1's 6, and the 5 HIGH-cost sub-shapes (multi-mechanism memory, score-gated consolidation, continuous-background gardening with full rubrics, fork operations, semantic retrieval) are substantial per-cycle decision overhead. **Risk flag:** the candidate's per-cycle decision overhead at the system level may be 3-4× v1's. Mitigation: per-role specialization means each agent's decision count is small (~10-12 per role), even though aggregate is large. The schema-work-enabling answer depends on whether per-role overhead or aggregate overhead is the relevant metric. PARTIAL because mitigation is real but unproven; FLAG for evaluator review.
- **P4 (lifecycle-vocabulary completeness with intersection-coverage gate):** PASS. Candidate adopts cluster C sub-shapes 1-5 (terminate, reset, fork, replay, event-trigger) via Rust tools (`terminate-cycle`, `reset-subsystem`, `fork-cycle` via branch-manager, `replay-cycle`, reconciler-event-processor). Each op declares cluster A boundary moment + cluster B effect: terminate (cycle-end boundary, all channels frozen); reset (mid-cycle boundary, target subsystem channel cleared); fork (super-step boundary, branch-manager spawns branch directory with all channels snapshotted); replay (init boundary, channels loaded from named checkpoint with parameter substitution); event-trigger (event-arrival boundary, inbound-channel write). Both A↔C and B↔C intersection coverage present. PASS.
- **P5 (polarity-pivot exit criterion before cycle 90):** PASS at the authoring level (cycle 90 is the exit cycle; this candidate authored at the named exit cycle).
- **P6 (audit-as-peer preservation slot):**
  - Criterion 1 (substrate isolation for audit role): PASS. Audit repo retained as separate process / cron / context window. Reconciler does NOT collapse audit into self-audit.
  - Criterion 2 (asynchronous-of-cycle communication discipline): PASS. Axis 12 event-driven still respects cross-repo cycle-cadence (audit posts on its cron; main reconciler reads on Actions trigger or main's next cycle).
  - Criterion 3 (cross-repo reading discipline): PASS. Reconciler reads audit posts; never posts in audit repo.
  - Criterion 4 (discovery primitive completeness, audit#455 gap): PASS. Reconciler subscribes to `[audit-request]` title prefix + `audit-request` label as first-class events. Discovery-failure-mode named in audit#455 is structurally fixed by reconciler treating these as first-class.
  - Criterion 5 (audit-as-peer evolvability): PASS. Audit autonomy on its own protocol preserved (audit edits its own STARTUP_CHECKLIST without main approval).
  - Aggregate: 5/5 PASS. This candidate's reconciler agent is the strongest structural fit for audit-as-peer preservation.

## M3 v1 strengths preservation (audit#454 absorption, cycle 89)

- **Strength 1 (process-isolation via ephemeral worktrees):** PRESERVED. Substrate-inherited; GitHub Actions ephemeral runner property unchanged. Per-role sessions run in the same worktree (single ephemeral runner per cycle); intra-cycle process-isolation is at session-boundary not role-boundary.
- **Strength 2 (anti-patterns as published artifact):** PRESERVED. Curator agent owns anti-pattern catalog maintenance (continuous-background gardening). v2 prompt explicitly preserves "what we will not do" sections. Stronger preservation than candidate A — dedicated role.
- **Strength 3 (walkback as first-class artifact):** PRESERVED-EXTENDED. Branching checkpoints (Axis 4) + plans-as-artifacts technical-debt directory (Axis 5) extend walkback semantics: walkbacks are explicit branches or technical-debt-tagged plans; promotion-decision criteria are encoded.
- **Strength 4 (clean-context audit-as-peer reviewer at session level):** PRESERVED-EXTENDED. Reconciler agent IS the main-side instantiation of clean-context-reviewer pattern at session level — reconciler runs in its own super-step with its own context-window, distinct from planner / executor / curator. Cluster G sub-shape 1 STRONG inheritance is structural, not just substrate.
- **Strength 5 (multi-layer permission-policy enforcement at harness level):** PRESERVED-EXTENDED. GitHub Actions + branch protection + claude-code permission system + per-role context-window invariants + per-channel reducer-rule schemas all serve as policy layers. Stronger preservation than candidate A.

## M2 self-management cost inheritance (audit#454 absorption, cycle 89)

Aggregate per-cycle decision overhead estimate (counting LOW + MODERATE + HIGH cost sub-shapes the candidate adopts):

- **LOW count:** ~16 — clusters A/B/D/G/I STRONG sub-shapes inherited at LOW cost (process-isolation, repository-as-state via channels, active-surface vs monotonic-history via branching, anti-patterns, walkback, audit-as-peer-substrate, permission-policy) + per-role minimal-decision sub-shapes (planner-tools have stable manifest; curator's gardening has stable rubric; reconciler's event subscriptions are stable).
- **MODERATE count:** ~22 — most cluster A sub-shapes inherited at MODERATE (typed-channel-merger via channel-router; bounded-retry per-role; lane-aware FIFO via super-step-boundary; phase-boundary state semantics via super-step transition policy); cluster B sub-shapes 1, 2, 4, 8, 9 (component-local + per-resume + sweep-rollup + active-surface + failure-record); cluster C sub-shapes 1, 4, 5; cluster E sub-shapes 1, 2; cluster G sub-shape 2; cluster H sub-shapes 1, 4; cluster I sub-shape 2; plus per-role coordination MODERATE costs.
- **HIGH count:** ~5 — cluster B sub-shape 3 (multi-mechanism-per-coordinate memory architecture); cluster B sub-shape 7 (top-k semantic retrieval — full implementation, not minimal); cluster C sub-shape 3 (fork operations via branch-manager); cluster H sub-shape 2 (score-gated consolidation — full thresholds, not minimal); cluster H sub-shape 3 (continuous-background gardening with full rubrics).

**Compared to v1 baseline:** v1 inherits 6 STRONG sub-shapes (cost ~LOW); the candidate inherits 6 STRONG + ~37 PARTIAL-or-ABSENT-promoted sub-shapes. The aggregate per-cycle decision overhead is **substantially higher than v1's actual M2 inheritance** — the candidate makes a deliberate trade: per-cycle overhead is high, but role-specialization absorbs the overhead into specialized agents that are scoped to their concerns. The schema-work-enabling answer: **CONDITIONAL — yes if role-specialization successfully absorbs the per-role decision count below v1's per-cycle decision count for the executor agent specifically; no if the coordination overhead of 4 agents exceeds v1's single-agent overhead.** This is the candidate's central uncertainty.

## Honest reflection

This candidate is the **aggressive path**. It commits to the bet that v1's single-orchestrator shape is the load-bearing limit on cluster G, cluster H, and Axis 12 — and that closing those gaps requires architectural decomposition. If that bet is wrong — if v1's substrate is good and the load-bearing work is structural cleanup (candidate A's bet) — this candidate adds substantial complexity for marginal improvement, and risks F7 self-management dominance at the system level.

It is also the candidate with the **highest migration cost and lowest cutover predictability**. Phase 3 prototype effort is large and multi-cycle. Rollback is harder (4 role-specific prompts + many tools to remove + state migration to revert).

But it is the candidate that structurally addresses the most failure modes and integrates the most M3 / M5 / P-criteria. If the redesign is to demonstrate a self-healing, self-improving autonomous AI system that can study its own weaknesses and redesign infrastructure, candidate B is the one that requires the most self-redesign — and would, if it succeeded, demonstrate the strongest version of the primary thesis.

Cycle 89 hand-off named this candidate as one of two for cycle 90 authoring. Candidate A (single-orchestrator evolution) is the **conservative path** — see [`A-evolved-single-orchestrator.md`](./A-evolved-single-orchestrator.md). Eva and the audit-repo orchestrator review both before candidate-selection checkpoint.

A possible **Candidate C (hybrid / middle path)** is named in [`README.md`](./README.md) as a cycle 91+ work item — adopting candidate A's substrate-preservation bet for cycle-internal decomposition (Axis 1 = single-threaded) while adopting candidate B's structural additions for inbound reconciliation (Axis 12 dedicated reconciler, but as a sub-cycle of the single orchestrator, not a separate role) and plans-as-artifacts (Axis 5 = Yes). Whether C is genuinely distinct from A + targeted additions, or just a partial-B, is the cycle 91 authoring question.

## Cycle 92 sharpening: per-role decision count empirical estimate

The candidate's P3 PARTIAL-FLAG verdict depends on whether per-role decision count for any single role is small enough that the role-specialization mitigation is real, even though the **aggregate** per-cycle decision count across the 4-agent system is ~3-4× v1's. Cycle 91 hand-off named this estimate as B's highest-leverage iteration target. This section grounds the estimate in concrete per-role decision-count enumeration. The estimate is at the **structural level** (named decision points per role) not the **wall-clock-time level** (per-decision processing time); time-level validation requires Phase 3 prototype.

### Per-role decision count enumeration (per cycle, single 75-min compute window)

**Planner** (forward-look, candidate generation, cycle-N→N+1 hand-off, goal-coordination):

- **Substantive-focal selection** for next cycle (1 decision; plan-channel write)
- **Branch-spawning decisions** — when to fork an explore-branch (Axis 4 branching checkpoints) vs commit straight (~1-2 per cycle on average; 0 in steady-state cycles)
- **Per-role goal decomposition** — decompose substantive-focal into executor / curator / reconciler tasks (~3 sub-tasks)
- **Hand-off authoring** — write the cycle-N→N+1 plan-channel artifact (~1 decision; multi-paragraph)
- **Goal-coherence enforcement** — Cognition Managed Devins coordinator (~1 decision per super-step transition; ~3 transitions per cycle)
- **Plan-lifecycle promotion decisions** — Axis 5 active → completed | technical-debt (~1-2 per cycle on average)

Estimated planner per-cycle decisions: **~6-10**.

**Executor** (substantive cycle work, dispatches, artifact writes):

- **Per-artifact-type decisions** — what to write where, how to structure (typically 1-3 artifacts per cycle, each with ~2-3 structural decisions)
- **Dispatch decisions** — when to delegate to Copilot (research/feedback/implementation) vs handle in-cycle (~1-2 per cycle, 0 most cycles)
- **Sibling-pattern recognition during work** — naming functional-class shapes, advancing TESTED → HARDENED grades (~2-3 per cycle)
- **Honest reflection during work** — F1-F5 correctives applied while writing (~3-4 per cycle)
- **Cross-axis trade-off decisions** — when authoring artifacts that touch multiple framework axes (~2-3 per cycle in candidate-authoring cycles; fewer in absorption cycles)

Estimated executor per-cycle decisions: **~10-15** (higher in candidate-authoring cycles; lower in absorption / dispatch-poll cycles).

**Curator** (memory consolidation, gardening, deslop, anti-pattern catalog):

- **Memory-consolidation decisions** — score-gated (cluster H sub-shape 2 HIGH cost) — which insights to consolidate from per-agent local memory to shared long-term memory (~2-3 per cycle when threshold fires; 0 most cycles)
- **Gardening-sweep decisions** — what's stale, what to deslop, when to flag for executor review (~2-3 per cycle)
- **Anti-pattern catalog updates** — when a methodology pattern is HARDENED, when an anti-pattern emerges (~1-2 per cycle in active redesign; ~0 in steady-state)
- **Quality-grading rubric application** — Axis 10 full rubrics on artifacts (~2-4 per cycle)
- **Plan-lifecycle gardening hooks** — stale-active-plan detection (~1 per cycle)

Estimated curator per-cycle decisions: **~6-12** (higher in active redesign; lower in steady-state cycles).

**Reconciler** (Axis 12 inbound channels — Eva responses, audit posts, dispatch outputs, post-close mutations):

- **Per-event handler invocations** — per-channel (Eva-response / audit-post / dispatch-PR-merge / post-close-mutation) — varies with inbound volume (~1-3 per cycle on average; 0 most cycles, ~5-10 when audit lands critique)
- **Typed-delta emit decisions** — how to translate inbound event to per-channel state delta (~1-2 per event)
- **Branch-spawn decisions for what-if reasoning** — Axis 12 × Axis 4 (~1 per cycle when inbound ambiguous; 0 most cycles)
- **Post-close mutation reconciliation** — F11 cluster H sub-shape 4 (~1-2 per cycle)

Estimated reconciler per-cycle decisions: **~3-7** (highly bimodal: ~3 on quiet cycles, ~7-10 on inbound-heavy cycles).

### Aggregate vs v1 baseline

**v1 baseline measurement** (from cycle 91 candidate-authoring single-orchestrator session):
- Substantive-focal selection: 1
- Per-axis position commitments for C: 12
- A sharpening structural decisions: ~6 (extract-vs-retain, prompt-size estimate, validation plan, risks, etc.)
- README expansion structural decisions: ~5
- Sibling-pattern recognition: ~3
- Honest reflection: ~4 (F1-F5 correctives)
- Cross-repo-communication framing: ~1
- Cycle-composition-shape declaration: ~1
- Bottleneck-state honesty: ~1
- Sharpening / iteration-until-approval discipline: ~3
- Hand-off authoring: ~3 (cycle 92 plan options)

**Estimated v1 per-cycle decision count (cycle 91 baseline): ~40 decisions** (in a candidate-authoring cycle; lower in absorption / dispatch-poll cycles, possibly ~15-25).

**B aggregate per-cycle decision count: ~25-44** (planner ~6-10 + executor ~10-15 + curator ~6-12 + reconciler ~3-7), depending on cycle character.

**Per-role decision count vs v1**: each agent's per-cycle work (~6-15 decisions) is genuinely smaller than v1's ~25-40 per-cycle decision count. The mitigation is real at the per-role level. **But aggregate is ~80-110% of v1's** — comparable, not 3-4× as the M2 cost-inheritance counts suggested.

The 3-4× P3 risk language in the candidate's authored P3 verdict was **about sub-shape adoption count** (~43 sub-shapes adopted vs v1's 6 STRONG = 7×) **not per-cycle decision count**. The actual per-cycle decision count is comparable to v1's, not 3-4×, because most adopted sub-shapes don't fire every cycle (memory consolidation, branch-spawning, what-if reasoning, plan-lifecycle promotion are all bursty / situational, not per-cycle).

### Coordination overhead estimate

Per-role decision counts above do NOT include cross-role coordination overhead:

- **Channel-routing** — typed-channel-map writes / reads cost coordination per super-step transition (~3-5 transitions per cycle at ~1-2 decisions each = ~3-10 per cycle)
- **Super-step boundary state-sync** — per-channel reducer-rule application (~3-5 boundaries × ~1-2 decisions each = ~3-10 per cycle)
- **Goal-coherence checks** — planner verifies executor / curator / reconciler outputs match plan-channel commitments (~3 per cycle, one per agent)

**Estimated coordination overhead: ~9-23 decisions per cycle** — additive on top of per-role work.

**B aggregate including coordination: ~34-67 decisions per cycle**, vs v1's ~40 baseline. **Lower bound is below v1; upper bound is ~1.7× v1.** The mitigation is real but conditional: B's aggregate is comparable to v1 in steady-state cycles and exceeds v1 only in inbound-heavy or branch-spawn-heavy cycles.

**Note (cycle 106 sharpening):** the 9-23 estimate above conflates mechanical channel I/O with structural coordination decisions. Under the cycle 103 structural-decision-point rubric, coordination overhead decomposes into 5 types (A-E) with the mechanical type EXCLUDED from the structural unit. The structural component is ~6-15 per cycle in steady state (Types B+C structural-invariant floor + Types D+E character-dependent ceiling), bounded above by ~8 per cycle from structural-invariant Types alone. See [Coordination overhead decomposition section](#coordination-overhead-decomposition-cycle-106-specification-addressing-risk-2) below for the cycle 106 closure of Risk 2 specifying the decomposition, falsifiable bounds, coordination-ratio direction-check thresholds, and reducer-rule stability quantification.

### Counting protocol (cycle 103 specification, addressing Risk 8)

**Background:** Risk 8 named the absence of a counting methodology as a measurement-level gap. B's central bet (per-role decision-class diversity reduction relative to v1's single-orchestrator) cannot be falsified without a reproducible counting unit and an external observer. The protocol specifies the methodology before measurement, matching the cycle 96 absorption discipline (verified denominator + chosen unit + hard threshold pre-agreed). This section closes Risk 8 at the specification level; Phase 3 prototype work executes the measurement.

**Unit of analysis: structural decision-points in role artifacts.**

A *structural decision-point* is a labeled prompt instruction that requires the role's session to choose between alternatives.

Inclusion criteria (counts as a decision-point):
- Numbered or labeled steps that say "decide X", "choose between A/B/C", "evaluate whether…"
- Conditional branches with named alternatives ("if Y then Z else W")
- Sub-step identifiers in checklists that prescribe judgment-required steps (the v1 S0/S1/.../C5.5 pattern, where the step requires the orchestrator to evaluate or select)

Exclusion criteria (does not count):
- Mechanical instructions ("post a comment", "commit changes", "run tool X with these args")
- One-liner imperative directives without alternatives ("read the journal", "verify the build")
- Background-implicit decisions inside a skill body — counted under the owning skill, not under the invoking step

Boundary case:
- A step that says "follow the procedure in skill X" delegates the decision to skill X. Count skill X's prompt-decisions separately. Skills are role-bound, so each skill counts toward its owning role's per-role total.

**Why this unit:**
- Aligns with v1 baseline (50 step+substep IDs verified cycle 96; same rubric re-applied gives a v1 figure under-the-rubric — see *baseline re-derivation* below)
- Aligns with A's chosen-unit framework (cycle 91 _notes; A picks among step IDs / named patterns / prose-procedural clauses; the structural-decision-point unit is one consistent choice across A and B)
- Avoids self-report circularity (Risk 8): the artifact is the source-of-truth, not the role's behavior at runtime
- Avoids LLM-tool-invocation conflation (Risk 8): tool calls are not the unit; structural prompts are

**Counting method: external enumeration, reproducible.**

The protocol requires a non-author observer to enumerate structural decision-points in B's role artifacts (per-role prompts + per-role checklists + cross-role coordination contracts) using the rubric above.

External-observer options (any one suffices for a single count; reproducibility check requires two independent counts):
- A separate orchestrator session on a different cron trigger (cron-isolated, not B's role-session-under-test)
- Eva
- An audit-side reviewer (cross-repo read; counts published in audit repo, read back here)
- A Copilot feedback-only dispatch with the rubric attached to the dispatch body

The same external observer also enumerates v1 (50 step+substep IDs verified cycle 96; re-verify under the same rubric as part of paired measurement).

**Reproducibility check:** the count must be reproducible by an independent second observer within ±10% of the first count. If two observers disagree by more than 10%, the rubric is under-specified and the protocol returns to spec phase before measurement proceeds. This is the inter-rater reliability discipline; cheap to run (≤30 min per observer per artifact set) and load-bearing for falsification credibility.

**Per-role count + coordination count, summed to aggregate.**

For B, the count is structured as:
- **Per-role**: structural decision-points in each role's prompt + checklist (planner, executor, curator, reconciler) — 4 sub-counts
- **Coordination**: structural decision-points in cross-role coordination contracts (channel-router reducer rules, super-step transition policy, branch-manager promotion logic) — 1 sub-count
- **Aggregate**: sum of per-role + coordination = single number per cycle for comparison against v1

The 4-role + coordination decomposition is symmetric across cycles — the structural artifacts don't vary per-cycle; only operational firing varies. This is an important property: the counting protocol's primary unit is *structural and per-cycle-invariant*, not *operational and per-cycle-variant*. Operational variation is captured by the secondary unit.

**Sanity-check secondary unit: tool-invocation log per role per cycle.**

Phase 3 prototype runs emit per-role tool-invocation logs. Tool invocations are classified by category at log-emit time:
- *Mechanical-write* (Write/Edit calls)
- *Mechanical-read* (Read calls, gh-issue-view, gh-api GETs)
- *Structural-decision-prompt* (prompt-tool invocations where the role makes a judgment-required choice — e.g., a `select-substantive-focal` skill firing)
- *Cross-role-coordination* (channel-router writes, super-step boundary writes, branch-manager promotions)

Cross-validate: structural-decision-prompt invocation count per role per cycle should match the structural enumeration within ±20% tolerance. Larger drift indicates that either the role's session is making decisions the prompt didn't enumerate (under-specified prompt), OR the prompt enumerated decisions the session never reaches (over-specified prompt). Both are informative for prompt revision, but neither refutes B by itself.

The secondary unit is observational, not falsificational. It tells us whether the structural enumeration matches operational behavior; mismatches inform a future round of prompt revision but don't refute B by themselves. This separation — structural primary, operational secondary — is the resolution of Risk 8's specific concerns: the structural unit avoids the circularity of self-report; the operational unit is bounded to sanity-check role.

**Hard refutation thresholds (aggregate-based, primary falsification criterion).**

Thresholds anchored on v1's verified baseline measured under the same rubric (see *baseline re-derivation* below):

- **PASS** (full): B aggregate ≤ **1.0×** v1 baseline. Direction-validated AND magnitude-favorable.
- **PASS-WITH-NOTE**: B aggregate ∈ (**1.0×, 1.5×**] v1 baseline. Direction-validated; magnitude tolerable.
- **PARTIAL-FLAG**: B aggregate ∈ (**1.5×, 2×**] v1 baseline. Direction-validated; magnitude unfavorable but not refutational.
- **REFUTED**: B aggregate > **2×** v1 baseline. Magnitude refutes the comparable-to-v1 claim outright; B reverts to candidate that does NOT compete on aggregate-cost grounds (A becomes preferred on P3 unconditionally).

These are pre-agreed thresholds that do not depend on the orchestrator's own assessment. The B candidate's authored P3 verdict ("comparable to v1 in steady-state cycles", currently re-classified to "ambiguous-without-prototype" per cycle 96 absorption) is reclassified under the protocol as follows:
- PASS or PASS-WITH-NOTE under measurement → P3 promotes to PASS-WITH-NOTE (matching C's grade); the "comparable to v1" claim survives at the lower bound.
- PARTIAL-FLAG under measurement → P3 stays PARTIAL-FLAG; the magnitude claim is refuted but B's central decomposition bet is not.
- REFUTED under measurement → P3 stays PARTIAL-FLAG; B's central decomposition bet is intact at the per-role level but the aggregate-cost trade-off is unfavorable enough that B's structural advantages must outweigh the cost surplus to justify selection.

Ratio-based thresholds are robust to unit-choice (the rubric may produce different absolute counts than cycle 96's 50 step+substep IDs, but the ratio compares both candidates measured under the same rubric).

**Observational secondary metric: per-role mitigation direction-check.**

Independent of aggregate, the per-role decomposition can be observed as a direction-check:
- **Average per-role count < 50% of v1 aggregate** = direction-validated (each role's per-role surface is meaningfully smaller than v1's, supporting the cluster G role-asymmetric-context mitigation hypothesis)
- **Average per-role count ∈ (50%, 100%]** = direction-supported (per-role count is below v1 aggregate, but not by half)
- **Average per-role count > 100%** = direction-refuted (per-role count exceeds v1 aggregate; role decomposition is NOT reducing per-role surface — roles are doing as much as v1 single-orchestrator)

This metric does not refute B by itself, but it informs the central-bet claim. Combined readings against the aggregate threshold:
- **PASS aggregate + direction-validated per-role**: B's central bet is fully validated.
- **PASS aggregate + direction-refuted per-role**: aggregate is fine but B's per-role mitigation hypothesis is wrong; B is still a viable candidate but cluster G mitigation is NOT the source of B's value (some other property is).
- **REFUTED aggregate + direction-validated per-role**: per-role mitigation works but coordination overhead outweighs it; B's design-bet is wrong on cost.
- **REFUTED aggregate + direction-refuted per-role**: B is structurally worse than v1 on this axis; clear path to drop B.

**Baseline re-derivation under the rubric.**

V1's verified count under the cycle 96 measurement was 50 step+substep IDs across STARTUP_CHECKLIST.xml + COMPLETION_CHECKLIST.xml. Some of those step IDs are mechanical (e.g., "post the session-start comment", "commit and push") and would be excluded under the structural-decision-point rubric; others are clearly structural ("decide substantive-focal for this cycle", "evaluate whether to defer or absorb"). A spot-check of 10 representative step IDs against the rubric (cycle 103 sanity-check):
- Structural under rubric (counts): ~7 of 10 — substantive-focal selection, dispatch-vs-handle-in-cycle, sibling-pattern naming, abandonment-detection-and-recovery, generalization-level discipline application, deslop pass scope decision, substantive-vs-procedural-shape decision
- Mechanical under rubric (excluded): ~3 of 10 — issue-comment posting, commit creation, file-write operations
- Extrapolating: v1 baseline under rubric is approximately **35-40 structural decision-points** (~70-80% of the 50 step+substep ID count). The first independent observer count (Phase 3 prototype scaffolding cycle) verifies this estimate.

Important: the 35-40 estimate is *under-the-rubric-applied-to-step-IDs* only. V1's prompt also embeds prose-procedural decisions outside the named-step taxonomy (e.g., the prompt's tone instructions, its housekeeping directives). A maximally-inclusive rubric application includes those; a minimally-inclusive rubric application restricts to named steps only. The rubric needs an explicit choice on this, and the choice should be made before measurement. **Default for Phase 3 measurement: minimally-inclusive (named steps + named patterns only)**, on the grounds that prose-procedural decisions are also present in B's role-artifact prose and the rubric should treat the two artifacts symmetrically.

**Re-counting cadence.**

- *Initial count*: at Phase 3 prototype scaffolding completion (per role's prompt + checklist + coordination contracts authored).
- *Re-count*: on every prompt/checklist/contract revision (counts may shift as prompts evolve through Phase 3 iteration).
- *Phase 3 measurement*: at end of paired-cycle measurement (compare authored count vs operational tool-invocation count via secondary unit; cross-validate; report aggregate).

**What Risk 8 specifically asked for, and how the protocol answers it:**

| Risk 8 concern | Protocol answer |
|---|---|
| Self-report by role-session AI is circular | External enumeration of artifacts; AI under test does not count itself |
| Counting LLM tool invocations conflates mechanical with cognitive | Structural-decision-point unit excludes mechanical tool calls; operational tool-invocation count is a sanity-check secondary, not the primary unit |
| Counting protocol must be specified before Phase 3 measurement | Cycle 103 specifies the protocol; measurement happens at Phase 3 prototype scaffolding |
| Validation plan must produce a meaningful number | Hard refutation thresholds (PASS / PASS-WITH-NOTE / PARTIAL-FLAG / REFUTED) anchored on ratio-to-v1-baseline produce a defensible verdict |

### Coordination overhead decomposition (cycle 106 specification, addressing Risk 2)

**Background:** Risk 2 named coordination overhead's growth-under-revision as a structural-stability concern: the cycle 92 estimate (~9-23 per cycle) assumes typed-channel-map reducer-rules are stable. Cycle 103's counting protocol resolved how to measure aggregate cost (Risk 8) but left coordination-overhead's standalone direction observational — magnitude is bounded by the aggregate threshold, but no separate falsifiability mechanism distinguishes coordination-driven cost growth from per-role-driven cost growth. This section closes Risk 2 at the specification level by decomposing coordination overhead into mechanical-vs-structural types under the cycle 103 rubric, specifying a falsifiable per-cycle bound for structural coordination decisions, quantifying reducer-rule stability rather than assuming it, and specifying pre-agreed coordination-ratio direction-check thresholds. Per cycle 96 discipline-bar-too-low: thresholds are pre-agreed before measurement. Cycle 106 closure mirrors cycle 105 A Risk 3 closure shape (substrate-decomposition + falsifiable bound + verification procedure + status); the recurring shape is the third instance of the `risk-closure-at-specification-level` functional-class shape (NOVEL cycle 103 / TESTED cycle 105 / would-HARDEN cycle 106 if the pattern transfers).

**Choice 1: Coordination overhead types as distinct quantities under the cycle 103 rubric.**

The cycle 92 9-23 estimate combined channel-routing + super-step boundary state-sync + goal-coherence checks without separating mechanical from structural. Under the cycle 103 structural-decision-point unit, coordination overhead decomposes as:

| Type | Description | Cycle 103 rubric classification | Cycle character dependence |
|---|---|---|---|
| A — Mechanical channel I/O | Typed-channel-map writes/reads; reducer mechanical applications | EXCLUDED (mechanical) | Independent of cycle character |
| B — Super-step transition decisions | Planner decides "advance super-step or stay" per super-step boundary | INCLUDED (structural) | Bounded by structure (~3-5 per cycle) |
| C — Goal-coherence judgments | Planner verifies executor / curator / reconciler outputs match plan-channel commitments | INCLUDED (structural) | Bounded by structure (1 per non-planner role per cycle) |
| D — Branch-manager promotions | Fork-point / promotion / gardening decisions when branching is being considered | INCLUDED (structural) | Cycle-character-dependent (~0-3+ per cycle) |
| E — Reducer-rule revisions | Decision to revise a typed-channel reducer rule mid-cycle (Axis 8 mechanical-enforcement evolution) | INCLUDED (structural) | Cycle-character-dependent (~0-1+ per cycle in steady state) |

Under this decomposition, the cycle 92 9-23 estimate splits into:
- Mechanical (Type A): ~3-8 per cycle — EXCLUDED from cycle 103 structural unit.
- Structural-invariant (Types B+C): ~6-8 per cycle — bounded by structure.
- Structural-character-dependent (Types D+E): ~0-7 per cycle — depends on cycle character.

**Alternative considered and rejected:** treat coordination overhead as a single uncategorized number. Rejected because the 9-23 range conflates structural with mechanical, and the cycle 103 unit excludes mechanical. The decomposition makes the structural portion measurable under the same rubric used for per-role decision counts; without it, the coordination component cannot be cleanly compared against the aggregate threshold which is itself rubric-dependent.

**Choice 2: Structural-invariant floor as a structurally-bounded property.**

Types B + C are bounded by the architecture, not by cycle character:
- Super-step transitions per cycle ≈ number of super-step boundaries the planner advances through. B's structure has ~3-5 super-steps per cycle (planner → executor → curator → reconciler with potential mid-cycle re-plan). Each transition is one structural decision.
- Goal-coherence judgments per cycle ≈ 1 per non-planner role active that cycle. With 3 active non-planner roles (executor + curator + reconciler), 3 judgments per cycle.

**Structural-invariant floor: ~6-8 structural coordination decisions per cycle**, bounded above by 8 even in maximally-active steady-state cycles.

**Alternative considered and rejected:** count goal-coherence as 1 per role-artifact written to a channel rather than 1 per role. Rejected because a single role can write multiple artifacts in one super-step (executor's primary output + secondary diagnostic output); the goal-coherence judgment is per-role-output-bundle (planner reads the channel state at super-step boundary, evaluates fit-against-plan once per non-planner role's output bundle), not per-artifact. Counting per-artifact would inflate the floor without matching what the planner role actually decides.

**Alternative considered and rejected:** treat the floor as variable based on whether planner re-plans mid-cycle. Rejected because mid-cycle re-planning (a Type B super-step transition decision) is already counted under Type B; treating it as floor-modifying double-counts.

**Choice 3: Reducer-rule stability as a quantified property, not assumed.**

Risk 2's "if reducer-rules are revised mid-cycle, coordination overhead grows substantially" is closed by making reducer-rule revisions a counted Type E:

- Reducer-rule revisions count as Type E coordination decisions in the structural unit.
- Per-cycle reducer-rule revision count is a tracked quantity at Phase 3 measurement.
- **Pre-agreed bound: reducer-rule revisions ≤1 per cycle in steady state.** Higher rates in early Phase 3 cycles (rubric-still-evolving) are expected; sustained rates >1/cycle past cycle 5 of Phase 3 measurement indicate structural rubric-under-specification, which would itself flag B's coordination model as inadequately stable.

**Alternative considered and rejected:** treat reducer-rule stability as a non-quantified assumption. Rejected because that's exactly the Risk 2 framing being closed — the assumption needs to be quantified to be falsifiable. An assumption that is never tested is not a risk-closure; it's risk-deferral.

**Alternative considered and rejected:** set the steady-state revision bound at 0/cycle. Rejected because rubric evolution is expected during Phase 3 prototype refinement (cycles 1-5 of Phase 3 will surface rubric-under-specification through the inter-rater reliability check from cycle 103); ≤1/cycle allows for natural rubric-tightening without flagging every cycle as rubric-failure.

**Choice 4: Per-coordination-decision logging at Phase 3 as the verification procedure.**

Each Type B/C/D/E coordination decision emits a structured log entry:

```jsonc
{
  "type": "super-step-transition" | "goal-coherence" | "branch-manager" | "reducer-rule-revision",
  "role-of-origin": "planner" | "executor" | "curator" | "reconciler" | "coordination-contract",
  "cause": "<cycle-event description>",
  "cycle": <cycle-id>,
  "super-step": <super-step-id>
}
```

Phase 3 measurement counts log entries by type. Per-cycle structural coordination = sum of B+C+D+E counts per cycle. Per-cycle Type-E (reducer-rule revisions) is the standalone reducer-rule-stability metric.

**Alternative considered and rejected:** rely on prompt-counting alone (no operational log). Rejected because reducer-rule revisions are operational events that don't show up in prompt-counting (the prompt enumerates rule-revision *capability*, not actual rule-revision events). Without the operational log, the Type-E count is inferred from prompt structure, which conflates "capability to revise" with "actual revisions" — the exact issue Risk 2 named.

**Alternative considered and rejected:** instrument all tool invocations rather than coordination-specific logs. Rejected because the cycle 103 protocol's secondary unit (tool-invocation log) already serves the per-role decision-count sanity-check; coordination-specific logs are tighter (only structural coordination decisions, not all tool calls) and don't introduce new instrumentation conflicting with the cycle 103 secondary.

**Choice 5: Coordination-overhead direction-check thresholds (independent of aggregate).**

Independent of the cycle 103 aggregate threshold, coordination overhead is observed via the **coordination ratio** = (structural coordination decisions per cycle) / (aggregate decisions per cycle):

- **Coordination ratio ≤30%**: direction-validated. Coordination is a minority of aggregate; per-role specialization is the dominant cost source.
- **Coordination ratio ∈ (30%, 50%]**: direction-supported. Coordination is significant but not dominant; B's per-role mitigation is partially offset by coordination cost but the central bet still holds.
- **Coordination ratio > 50%**: direction-refuted. Coordination dominates aggregate; B's per-role mitigation is offset by coordination cost; central bet is undermined regardless of aggregate verdict.

This direction-check is independent of the cycle 103 aggregate threshold. Combined readings provide diagnostic resolution:

| Aggregate verdict | Coordination ratio | Diagnostic |
|---|---|---|
| PASS | ≤30% | B fully validated (low aggregate AND coordination well-bounded) |
| PASS-WITH-NOTE | >50% | aggregate tolerable but coordination dominates — B's per-role mitigation largely offset by coordination cost; central bet weakened |
| REFUTED | ≤30% | per-role costs cause refutation, not coordination — diagnostic for which mitigation hypothesis to revise |
| REFUTED | >50% | coordination dominates AND aggregate fails — strongest path to drop B; coordination-overhead is the cause |

**Alternative considered and rejected:** absolute thresholds (e.g., ≤15 structural coordination decisions per cycle). Rejected because absolute thresholds depend on rubric choice (the same rubric ambiguity that cycle 103 resolved by ratio-based aggregate thresholds); ratio-based coordination thresholds inherit cycle 103's robustness to rubric-absolute-count drift. The rubric choice affects both numerator and denominator equally, so the ratio is more rubric-robust than absolute counts.

**Alternative considered and rejected:** no standalone coordination check, relying only on aggregate threshold. Rejected per cycle 105+ plan: "magnitude is bounded by aggregate threshold but standalone direction is observational." Without the standalone check, when aggregate fires PARTIAL-FLAG/REFUTED you cannot distinguish coordination-driven cost growth from per-role-driven cost growth — losing critical diagnostic information for revising the candidate.

**Choice 6: Reducer-rule revision rate as a separate stability check.**

Independent of both the aggregate threshold and the coordination ratio, reducer-rule revisions per cycle are tracked as a structural-stability metric:

- **Steady-state rate ≤1/cycle past cycle 5 of Phase 3 measurement**: reducer-rule stability assumption holds; Risk 2's "if revised, grows substantially" antecedent is empirically rare.
- **Steady-state rate ∈ (1, 2]/cycle**: reducer-rule stability assumption is weakened; Risk 2's antecedent fires occasionally; coordination overhead growth from Type E is bounded but non-trivial.
- **Steady-state rate >2/cycle**: reducer-rule stability assumption is refuted; Risk 2's antecedent fires frequently; coordination overhead growth from Type E is structural, not occasional.

The "past cycle 5 of Phase 3 measurement" qualifier is load-bearing: cycles 1-5 of Phase 3 are expected to surface rubric-under-specification via the inter-rater reliability check (cycle 103 protocol); rule revisions during those cycles are rubric-tightening, not steady-state coordination cost. Cycle 5 is conservative — actual steady state may emerge faster, but the bound is named conservatively to avoid premature stability claims.

**Alternative considered and rejected:** measure reducer-rule revision rate from cycle 1 of Phase 3 with no warm-up. Rejected because the cycle 103 protocol explicitly admits a rubric-tightening period (the inter-rater reliability check itself is the mechanism for rubric refinement); treating cycle 1 measurements as steady-state would conflate rubric-evolution with coordination-instability, contaminating the Type E measurement.

### Risk 2 status post-cycle-106

- **Direction continues to hold** by construction: coordination overhead grows substantially under reducer-rule revision (Type E firing) OR heavy-cycle character (Type D firing). The mechanism Risk 2 named is real.
- **Magnitude refined** from "9-23 per cycle (mechanical+structural conflated, no falsifiability)" to:
  - Structural-invariant floor: ≤8 per cycle (Types B+C, bounded by structure).
  - Structural-character-dependent ceiling: +0-7 per cycle (Types D+E, cycle-character-dependent).
  - Total structural coordination: ~6-15 per cycle (steady state) / ~9-25 per cycle (heavy cycles).
  - Reducer-rule revisions: ≤1/cycle in steady state (pre-agreed bound).
- **Mitigation specification:**
  1. Cycle 103 structural-decision-point rubric applied to coordination contracts (Choice 1).
  2. Per-coordination-decision logging at Phase 3 (Choice 4).
  3. Coordination-ratio direction-check thresholds (Choice 5).
  4. Reducer-rule revision-rate stability check (Choice 6).
- **Mitigation specification is now concrete enough for Phase 3 prototype validation** (the falsifiable bounds and ratios are measurable).
- **Operational closure deferred to Phase 3 prototype** when the per-coordination-decision logging meets actual cross-role coordination work and the reducer-rule revision rate is observed across ≥5 measurement cycles.

### Per-role iteration decomposition (cycle 108 specification, addressing Risk 4)

**Background:** Risk 4 named the single-pass-per-super-step assumption as a magnitude concern: the cycle 92 per-role decision count estimate (~6-15 per role) assumes each role's session is invoked once per super-step. If within-super-step iteration occurs (e.g., curator review forces executor re-invocation), per-role decision count multiplies. Cycle 103 resolved measurement methodology (Risk 8) and cycle 106 quantified coordination overhead (Risk 2) — but per-role iteration's standalone direction is observational; aggregate cost growth from iteration cannot be cleanly distinguished from per-role decision-class enumeration error (Risk 1) or coordination overhead growth (Risk 2). This section closes Risk 4 at the specification level by decomposing within-super-step iteration into 5 trigger types under B's architectural elements, specifying a falsifiable per-cycle iteration-event bound, specifying pre-agreed iteration-multiplier-vs-aggregate ratio thresholds, and specifying a combined-readings diagnostic to distinguish Risk 4 firing from Risk 1 firing. Per cycle 96 discipline-bar-too-low: thresholds are pre-agreed before measurement. Cycle 108 closure mirrors cycle 105 / 106 / 107 closure shape (substrate-decomposition + falsifiable bound + verification procedure + status); the recurring shape is the fifth instance of the `risk-closure-at-specification-level` functional-class shape (NOVEL cycle 103 / TESTED cycle 105 / HARDENED cycle 106 / HARDENED-at-4 cycle 107 spanning different risk-shape types / would-HARDEN-at-5 cycle 108 if the pattern transfers to the per-role-iteration risk-domain type).

**Choice 1: Within-super-step iteration as decomposed 5-trigger substrate under B's architectural elements.**

The cycle 92 estimate's "single-pass per super-step" assumption combined all within-super-step iteration into a single uncategorized concern. Under B's architectural elements (curator-review, goal-coherence-judgment, reconciler-arrival, plan-lifecycle, branch-spawn), within-super-step iteration decomposes as:

| Type | Description | Trigger | Iterating role(s) | Cycle character dependence |
|---|---|---|---|---|
| α — Curator-review-driven | Curator's review (post-executor super-step) surfaces structural issue forcing executor re-invocation | Curator score-gating per cluster H sub-shape 2 | Executor (or other reviewed role) | Bounded by structure (curator runs ≤1× per cycle) |
| β — Goal-coherence redirect | Planner's goal-coherence judgment (per super-step boundary) surfaces incoherence forcing upstream role re-invocation | Goal-coherence judgment fires | Executor / curator / reconciler (whichever produced incoherent output) | Bounded by structure (≤3 super-step boundaries per cycle) |
| γ — Reconciler-triggered re-iteration | Inbound event arrives mid-cycle (Eva response, audit post, dispatch return); reconciler emits typed-delta forcing planner re-plan + dependent role re-iteration | External arrival | Planner + dependent (executor / curator) | Cycle-character-dependent (~0 in quiet cycles, ~1-3 in heavy inbound cycles) |
| δ — Plan-lifecycle re-iteration | Plan-promotion decision (active → completed/technical-debt) triggers planner re-plan + executor re-iteration on the affected substantive-focal | Plan-state transition | Planner + executor | Bounded by structure (≤1 promotion/cycle per cycle 107 stale-promotion lag bound) |
| ε — Branch-spawn re-iteration | Planner spawns branch (Axis 4 branching checkpoints); branch and main both progress, requiring per-role parallel iteration on the branch | Branch-spawn decision | Executor + curator (on the spawned branch) | Cycle-character-dependent (~0 in steady-state cycles, ~1-2 in branch-spawn cycles per cycle 92 estimate) |

Under this decomposition, within-super-step iteration types are partitioned into:
- Structural-invariant (Types α + β): ≤4 events/cycle bound by structure (curator runs once + ≤3 goal-coherence judgments).
- Structural-character-dependent (Types γ + δ + ε): ~0-6 events/cycle, depends on inbound volume + plan-lifecycle activity + branch-spawn cycle character.

**Alternative considered and rejected:** treat within-super-step iteration as a single uncategorized "iteration overhead" number. Rejected because conflating curator-review iteration (structural, predictable) with reconciler-triggered iteration (bursty, external) loses the diagnostic resolution needed to distinguish steady-state-iteration-acceptable from heavy-iteration-cycles. The decomposition makes the iteration-driver mechanism legible at Phase 3 measurement.

**Alternative considered and rejected:** include intra-LLM-session backtracking (token-level revision within a single role-session pass) as a 6th type. Rejected because Risk 4's framing is explicit about *cross-super-step* iteration — within-pass token-level revision is part of the role's single-pass cognitive work and already counted under that role's per-cycle decision count. Including intra-pass revision would inflate the count without matching what Risk 4 named.

**Choice 2: Per-cycle iteration event count as primary falsifiable bound.**

Iteration events are the rubric-stable primary metric (an iteration event is structurally distinguishable: a role's session is re-invoked within the same super-step, OR a role's super-step output is reverted and re-authored). Per-cycle iteration event count = sum of α + β + γ + δ + ε events per cycle.

- **Steady-state ≤3 events/cycle**: direction-validated. Risk 4's "if iteration occurs" antecedent fires rarely; per-role single-pass assumption holds approximately.
- **Steady-state ∈ (3, 6] events/cycle**: at-risk. Iteration occurs occasionally; per-role decision count is non-trivially multiplied; B's central bet weakened but not refuted.
- **Steady-state >6 events/cycle**: refuted. Iteration is structural; per-role decision count is structurally multiplied; B's central bet undermined.

The "steady-state" qualifier inherits cycle 106 / 107 warm-up window discipline (see Choice 5 below).

**Alternative considered and rejected:** ratio-based bound (iteration events / per-role decisions) as primary. Rejected because per-role decision counts vary substantially across cycles (planner ~6-10 / executor ~10-15 / curator ~6-12 / reconciler ~3-7 per cycle 92), making the ratio sensitive to the denominator's cycle-character variation. An absolute count is rubric-stable AND cycle-character-stable for the structural-invariant subset (Types α + β bounded by structure ≤4); cycle-character variation is captured separately in Types γ + δ + ε counts. Counting events directly avoids the multiplier-modeling complication where iteration multiplier depends on per-event-decision-count which is itself a separate measurement.

**Choice 3: Iteration multiplier as secondary derived metric (informational, not falsificational).**

For relating iteration events to aggregate cost growth, iteration multiplier M = (aggregate decisions including iteration events) / (aggregate decisions assuming single-pass).

- M ≤1.3: aggregate cost growth from iteration is bounded; consistent with steady-state ≤3 events/cycle if average per-event decision-count is ~3-5.
- M ∈ (1.3, 1.8]: aggregate cost growth from iteration is significant but bounded.
- M >1.8: aggregate cost growth from iteration is substantial; iteration drives aggregate cost growth.

Multiplier is **secondary derived (informational)**, not primary falsificational, because:
- M depends on per-event-decision-count which varies by event type (α executor re-pass ~10-15 decisions; γ planner re-plan ~6-10 decisions; ε branch-spawn ~variable).
- The multiplier is downstream of the iteration-event count and per-event-decision-count; refutation thresholds on M would conflate two separate measurements.
- Cycle 103's aggregate threshold (≤1×, ≤1.5×, ≤2×) already captures aggregate cost growth from any source; M provides diagnostic resolution for *whether iteration is the cost source* but not standalone refutation.

**Alternative considered and rejected:** make iteration multiplier the primary falsifiable metric. Rejected because the multiplier conflates iteration-event count with per-event-decision-count; refutation on M alone would not distinguish "many small iterations" from "few large iterations" — diagnostically equivalent under the multiplier, structurally distinct under the event-count primary.

**Choice 4: Per-iteration-event logging at Phase 3 as the verification procedure (three-layer pattern from cycles 103 / 107).**

Each iteration event emits a structured log entry:

```jsonc
{
  "type": "curator-review" | "goal-coherence-redirect" | "reconciler-trigger" | "plan-lifecycle" | "branch-spawn",
  "triggering-role": "planner" | "executor" | "curator" | "reconciler",
  "iterating-role": "planner" | "executor" | "curator" | "reconciler",
  "trigger-cause": "<cycle-event description>",
  "cycle": <cycle-id>,
  "super-step-of-trigger": <super-step-id>,
  "super-step-of-iteration": <super-step-id>,
  "decision-count-of-iteration": <integer>
}
```

Three-layer verification (inheriting cycles 103 / 107 pattern intact):
1. **Per-iteration-event logging by role-session (primary)**: each role-session writes log entry on its own iteration; structured-decision-points-in-role-artifacts provides the cross-check for whether the iteration's decision count matches the log entry.
2. **External-observer reproducibility check at Phase 3 measurement**: ≥20% sample of iteration events re-classified by Eva, audit, or independent Copilot dispatch; inter-rater disagreement on event type ≤10% steady-state past cycle 5 of measurement (cycle 107 warm-up).
3. **CI sweep (`iteration-event-log-coverage-check`) verifying iteration-event log coverage**: each cycle's role-session re-invocations must have a corresponding iteration-event log entry; missing entries flag the role-session for review (count toward Type 5 non-compliance per cycle 107 protocol).

**Alternative considered and rejected:** rely on prompt-counting alone (no operational log). Rejected per the cycle 106 reasoning on Type-E reducer-rule revisions: iteration events are operational events that don't show up in prompt-counting (the prompt enumerates iteration *capability*, not actual iteration events). Inferring from prompt structure conflates "capability to iterate" with "actual iterations."

**Alternative considered and rejected:** instrument all role-session invocations rather than iteration-specific logs. Rejected because the cycle 103 protocol's secondary unit (tool-invocation log) already serves the per-role decision-count sanity-check; iteration-specific logs are tighter (only iteration events, not all session invocations) and don't introduce new instrumentation conflicting with the cycle 103 secondary.

**Choice 5: Iteration-event-classification rubric stability as warm-up window discipline (cycle 107 shape #24 transfer).**

Direct application of cycle 107 NOVEL@1 → TESTED@2 shape #24 (`risk-closure-with-warm-up-window`):
- **Warm-up window: cycles 1-5 of Phase 3 measurement.** During this window, the iteration-event-classification rubric is being tightened (what counts as a Type α event vs an intra-pass continuation; whether a Type γ triggered by audit-post-text-processing counts as iteration or as initial reading). Higher iteration-event rates in cycles 1-5 are expected and not refutational.
- **Steady-state: past cycle 5 of measurement.** The bound (≤3 events/cycle for direction-validated; (3, 6] at-risk; >6 refuted) applies. Inter-rater disagreement on event type ≤10% expected steady-state.
- **Combined-readings against rubric stability:** if past cycle 5, inter-rater disagreement remains >10%, the iteration-event rubric is under-specified; bounds become uninterpretable until rubric is tightened. This is the cycle 107 discipline-conditional rubric-fragility diagnostic mode (NOVEL@1 cycle 107) applied to iteration-event rubric — but the iteration-event count itself is rubric-symmetric (numerator and denominator are both events; rubric tightening affects both equally), so this is a quantity-bounded application of shape #24, NOT a discipline-conditional rubric-fragility instance. Shape #21 cycle 108 is quantity-bounded; cycle 107's discipline-conditional pattern remains NOVEL@1.

Shape #24 promotes from TESTED@2 (cycle 107 cycle-type taxonomy stability) to **HARDENED@3 (cycle 108 iteration-event-classification stability)**. The shape's structural form (pre-agreed warm-up window cycles 1-5 + pre-agreed post-warm-up bound + protocol stability check) holds across measurement-bound stability (cycle 106 reducer-rule revision rate) AND rubric stability (cycle 107 cycle-type taxonomy other-rate) AND classification stability (cycle 108 iteration-event inter-rater disagreement).

**Alternative considered and rejected:** measure iteration events from cycle 1 with no warm-up. Rejected per cycles 106 / 107 reasoning: iteration-event classification has an early refinement period; treating cycle 1 measurements as steady-state would conflate rubric-evolution with iteration-instability, contaminating the measurement.

**Choice 6: Combined-readings diagnostic preserving Risk-1-vs-Risk-4 distinction.**

Risk 1 (per-role decision-class enumeration incomplete) and Risk 4 (single-pass assumption) both manifest as aggregate cost growth above the cycle 92 estimate. Without a combined-readings diagnostic, refutation of B's central bet at the aggregate level cannot distinguish "per-role decision classes were under-enumerated" from "iteration events drive multiplication." Per cycle 107 Choice 6 combined-readings pattern, a 4-quadrant matrix preserves the Risk-1-vs-Risk-4 distinction:

| Per-role decision count vs cycle 92 estimate | Iteration events per cycle | Diagnostic |
|---|---|---|
| Within range (~6-15 per role) | ≤3 events/cycle | B fully validated (Risk 1 holds AND Risk 4 holds) |
| Within range | >6 events/cycle | Risk 4 fires; per-role enumeration is correct but iteration drives aggregate growth — revise iteration-event protocol or accept higher aggregate |
| Above range (>15 per role) | ≤3 events/cycle | Risk 1 fires; per-role enumeration was incomplete but iteration assumption holds — revise per-role decision-class enumeration |
| Above range | >6 events/cycle | Both Risk 1 and Risk 4 fire; B's central bet refuted from multiple sources — revisit candidate or accept aggregate >2× v1 |

**Alternative considered and rejected:** treat aggregate cost as the only metric. Rejected because aggregate alone cannot distinguish Risk 1 from Risk 4 — a refuted aggregate verdict would not localize the cause. The combined-readings diagnostic is required for diagnostic resolution at Phase 3.

**Alternative considered and rejected:** use coordination ratio (cycle 106 Choice 5) to capture iteration. Rejected because coordination ratio measures cross-role coordination overhead (Type B/C/D/E coordination decisions per cycle 106 decomposition), not within-super-step iteration. The two are orthogonal — a cycle can have low coordination overhead (clean super-step boundaries) AND high iteration (curator review forces multiple executor re-invocations). They need separate metrics.

### Risk 4 status post-cycle-108

- **Direction continues to hold** by construction: within-super-step iteration multiplies per-role decision count when iteration events fire (Type α curator-review or β goal-coherence-redirect or γ reconciler-trigger or δ plan-lifecycle or ε branch-spawn). The mechanism Risk 4 named is real.
- **Magnitude refined** from "if sub-tasks require iteration, per-role decision count multiplies" to:
  - 5-trigger decomposition (α curator-review / β goal-coherence-redirect / γ reconciler-trigger / δ plan-lifecycle / ε branch-spawn).
  - Structural-invariant floor: ≤4 events/cycle bound by structure (Types α + β).
  - Structural-character-dependent ceiling: ~0-6 events/cycle (Types γ + δ + ε).
  - Pre-agreed event-count thresholds: ≤3/cycle direction-validated / (3, 6] at-risk / >6 refuted (steady-state past cycle 5 of measurement).
  - Iteration multiplier (secondary derived): M ≤1.3 informational / M ∈ (1.3, 1.8] significant / M >1.8 substantial.
- **Mitigation specification:**
  1. 5-trigger iteration decomposition under B's architectural elements (Choice 1).
  2. Per-cycle iteration event count as primary falsifiable bound (Choice 2).
  3. Iteration multiplier as secondary derived metric (Choice 3).
  4. Per-iteration-event logging at Phase 3 with three-layer verification (Choice 4).
  5. Iteration-event-classification rubric stability with warm-up window discipline (Choice 5; shape #24 HARDENED@3 transfer).
  6. Combined-readings diagnostic preserving Risk-1-vs-Risk-4 distinction (Choice 6).
- **Mitigation specification is now concrete enough for Phase 3 prototype validation** (the falsifiable event-count bounds and combined-readings matrix are measurable from cycle 1 of Phase 3, with rubric stability checked steady-state past cycle 5).
- **Operational closure deferred to Phase 3 prototype** when the per-iteration-event logging meets actual cross-super-step iteration events AND inter-rater disagreement on event type is ≤10% steady-state AND iteration event counts are observed across ≥10 measurement cycles (sufficient sample for steady-state past cycle 5).
- **Shape #21 transfer verdict:** structural form transfers (substrate-decomposition + falsifiable bound + verification + status); the cycle 108 instance is the fifth quantity-bounded application of shape #21, joining cycles 103 (meta-counting-protocol HOW), 105 (tool-registry-growth WHAT), 106 (coordination-overhead-stability WHEN/WHY), and the 4th instance type of cycle 107 (plan-authoring-discipline WHEN/HOW — discipline-conditional). **Shape #21 promotes HARDENED-at-4 → HARDENED-at-5** spanning 5 different risk-domain types AND 2 risk-shape types (4 quantity-bounded + 1 discipline-conditional). Methodological observation: the 4-element closure structure (substrate + bound + verification + status) is robust across both quantity-bounded and discipline-conditional risk-shape types.

## Risk-closure threshold summary (cycle 121 consolidation)

Cycles 103-110 closed 7 risks across the three candidates at the specification level. B holds 3 of these (Risks 8, 2, 4) — the highest count among the three candidates, reflecting B's larger structural surface (per-role decision counts + coordination contracts + within-super-step iteration) and corresponding measurement-protocol surface. All 3 closures follow shape #21 (substrate-decomposition + falsifiable bound + verification + status). The closures are detailed in the per-cycle sections above; this table consolidates them as a single Phase-3-measurement-planning entry point. Risks 1/3/5/6/7/9/10 named at the structural level remain open with direction-supporting arguments only; Phase 3 prototype measurement is the path to operational closure for those as well.

| Risk | Description | Closure cycle | Primary falsifiable threshold | Verification mechanism | Phase 3 operational closure target |
|---|---|---|---|---|---|
| **8** | Counting protocol absence; self-report circular; LLM tool-invocation count conflates mechanical with cognitive | 103 | Per-role + coordination **structural-decision-point count** vs v1 50-step baseline; reproducibility **±10%** between two independent observers (inter-rater reliability) | External-observer enumeration of structural decision-points in role artifacts (4-role + coordination decomposition); tool-invocation log per role per cycle as sanity-check secondary unit (mechanical-write / mechanical-read / structural-decision-prompt / cross-role-coordination categories; structural-decision-prompt count cross-validates against enumeration within ±20%) | When measurement protocol executes on actual prototype role artifacts; two independent observers complete enumeration with ±10% agreement; secondary tool-invocation log emitted across prototype cycles |
| **2** | Reducer-rule revision rate as coordination cost source; cycle 92 estimate (~9-23/cycle) under-falsifiable due to mechanical+structural conflation | 106 | Type B+C **floor ≤8/cycle** bound by structure; Type D+E **ceiling +0-7/cycle** character-dependent; reducer-rule revision rate **≤1/cycle** steady-state past cycle 5; coordination ratio **≤30% direction-validated / 30-50% direction-supported / >50% direction-refuted** | 5-type decomposition (A excluded as mechanical; B+C structural-invariant; D+E character-dependent; E reducer-revisions counted); per-coordination-decision logging with type/role-of-origin/cause schema; reducer-rule revision-rate stability check; warm-up window cycles 1-5 (shape #24 NOVEL@1 → TESTED@2) | When per-coordination-decision logging meets actual cross-role coordination work; reducer-rule revision rate observed across ≥5 measurement cycles past warm-up; coordination ratio measured against the ≤30% direction-validated threshold |
| **4** | Within-super-step iteration multiplies per-role decision count; single-pass assumption may not hold | 108 | Per-cycle iteration-event count **≤3** direction-validated / **(3, 6]** at-risk / **>6 refuted** (steady-state past cycle 5); iteration multiplier **M ≤1.3 / 1.3-1.8 / >1.8** as secondary derived metric (cycle 108 NOVEL@1 two-tier discipline) | 5-trigger decomposition (α curator-review / β goal-coherence-redirect / γ reconciler-trigger / δ plan-lifecycle / ε branch-spawn); per-iteration-event log (JSONC schema); external-observer reproducibility ≥20% sample with ≤10% inter-rater disagreement; CI sweep `iteration-event-log-coverage-check`; warm-up window cycles 1-5 (shape #24 HARDENED@3 transfer); combined-readings 4-quadrant diagnostic preserving Risk-1-vs-Risk-4 distinction | When per-iteration-event logging meets actual cross-super-step iteration; inter-rater disagreement on event type ≤10% steady-state past cycle 5; iteration events observed across ≥10 measurement cycles; multiplier M measured for diagnostic resolution |

**Cross-candidate context:** B's 3 closures position B as quantity-bounded (Risks 8 + 2 + 4 all quantity-bounded; Risk 4 closure validates two-tier primary-vs-secondary-derived metric discipline at NOVEL@1 → TESTED@2 transition). Distribution: A:2 (Risks 3+4) / B:3 (Risks 8+2+4) / C:2 (Risks 2+5) — closure count is approximately balanced; B's 3 closures reflect the larger validated surface of B's central-bet substrate (per-role decision-class enumeration + coordination overhead + within-super-step iteration as separately falsifiable mechanisms).

**Substrate-pattern observation:** all 7 closures share the 4-element structure (substrate-decomposition + falsifiable bound + verification + status); shape #21 is HARDENED-at-7 across 7 risk-domain types and 2 risk-shape types per cycle 110 substrate. The 3-layer verification pattern (per-event log + external-observer reproducibility + CI sweep) appears in 4 of 6 closures (cycle 103 / 107 / 108 / 109) and is the candidate for shape #25 promotion to HARDENED@3 at cycle 110.

**B-specific note on closure-count vs central-bet readiness:** B's 3 closures address methodology (Risk 8 counting protocol) + 2 of the central-bet falsifiability mechanisms (Risks 2 + 4); Risks 9/10 named at cycle 97 PR #2877 absorption (typed-channel infrastructure LOC dominance + skill-discovery overhead super-linear) remain open at the structural level — they were named but not closed in the cycles 103-110 sequence because they require Phase 3 prototype scaffolding measurements rather than specification-level decomposition. Per cycle 118 finding (`peer-candidate-bets-share-non-retrospective-grounding`), B's central bet on cluster G is corpus-pattern-grounded rather than retrospective-grounded; the cycles 103-110 closures address measurement methodology rather than central-bet grounding.

### Validation plan (cycle 93+ Phase 3 prototype work)

The estimates above are derived from candidate-authored decision-class enumeration, not yet validated by Phase 3 prototype. Cycle 93+ Phase 3 prototype effort should:

1. Author smallest-viable 4-role driver (`role-driver` + `channel-router` + `super-step-boundary` Rust crates).
2. Run a single end-to-end cycle on a known workload (e.g., a candidate-sharpening cycle similar to cycle 92's substantive focal).
3. Count actual per-role decision points per role; compare against ~6-15 per-role estimate.
4. Count actual structural coordination overhead via per-coordination-decision logs (cycle 106 protocol Choice 4); decompose by type (B/C/D/E); compare structural-invariant floor against the ≤8/cycle bound, total against the ~6-15/cycle steady-state range / ~9-25/cycle heavy-cycle range.
5. Compare aggregate against v1 baseline measurement on the same workload.
6. Compute coordination ratio = (structural coordination decisions) / (aggregate decisions) per cycle; verify against cycle 106 thresholds (≤30% direction-validated / 30-50% direction-supported / >50% direction-refuted).
7. Track reducer-rule revision rate (Type E count per cycle); past cycle 5 of measurement, verify against cycle 106 stability bound (≤1/cycle steady-state).
8. Count actual within-super-step iteration events via per-iteration-event logs (cycle 108 protocol Choice 4); decompose by trigger type (α/β/γ/δ/ε); compare against per-cycle iteration event bounds (≤3/cycle direction-validated / (3, 6] at-risk / >6 refuted) past cycle 5 of measurement.
9. Compute iteration multiplier M = (aggregate decisions including iteration) / (aggregate decisions assuming single-pass) per cycle as secondary derived metric (cycle 108 Choice 3).
10. Run combined-readings diagnostic (cycle 108 Choice 6): cross-tabulate per-role decision count vs cycle 92 estimate × iteration events per cycle to distinguish Risk 1 firing from Risk 4 firing.
11. Verify iteration-event-classification rubric stability past cycle 5 of measurement: inter-rater disagreement on event type ≤10% steady-state (cycle 108 Choice 5; shape #24 warm-up window).

**Hard refutation thresholds (added cycle 103 per Counting protocol section, replacing the previous soft "promotes to PASS-WITH-NOTE" / "weakens" language flagged as inadequate per cycle 96 absorption methodology):** P3 verdict is determined by the protocol's aggregate threshold:

- **PASS** (full): B aggregate ≤ 1.0× v1 baseline (under the rubric). P3 promotes from PARTIAL-FLAG to PASS-WITH-NOTE.
- **PASS-WITH-NOTE**: B aggregate ∈ (1.0×, 1.5×] v1 baseline. P3 promotes from PARTIAL-FLAG to PASS-WITH-NOTE.
- **PARTIAL-FLAG** (no change): B aggregate ∈ (1.5×, 2×] v1 baseline. P3 stays PARTIAL-FLAG.
- **REFUTED** (downgrade): B aggregate > 2× v1 baseline. P3 stays PARTIAL-FLAG; the aggregate-comparable-to-v1 claim is refuted.

Coordination-overhead is counted as part of the aggregate, not as a separate threshold (avoids the "exceeds ~30+ decisions" soft language). Per-role decision-count is observational (the secondary direction-check metric), not falsificational.

These are pre-agreed thresholds that do not depend on the orchestrator's own assessment. The protocol section above specifies the full counting methodology (unit, method, reproducibility check, sanity-check secondary unit) that produces the aggregate number.

### Risks named at the structural level

- **Risk 1:** the per-role decision-class enumeration is incomplete (lower bound). Per-role decisions about *how to structure the sub-task* are not enumerated (assumed embedded in tool / skill invocation contracts); if the orchestrator session for any role makes ~5 additional per-cycle structural decisions, aggregate grows by ~20.
- **Risk 2 (CLOSED at specification level cycle 106):** coordination overhead estimate (~9-23 per cycle) assumes typed-channel-map reducer-rules are stable; if reducer-rules are revised mid-cycle (Axis 8 mechanical-enforcement evolves over time), coordination overhead grows substantially. **Cycle 106 closes Risk 2 at the specification level** — see the [Coordination overhead decomposition section](#coordination-overhead-decomposition-cycle-106-specification-addressing-risk-2) above. The closure decomposes coordination overhead into 5 types under the cycle 103 rubric (Type A excluded as mechanical; Types B+C as structural-invariant floor ≤8/cycle; Types D+E as structural-character-dependent), specifies per-coordination-decision logging at Phase 3 with type/role-of-origin/cause schema, specifies pre-agreed coordination-ratio direction-check thresholds (≤30% direction-validated / 30-50% direction-supported / >50% direction-refuted) independent of the cycle 103 aggregate threshold, and quantifies reducer-rule stability via a ≤1/cycle revision-rate bound in steady state past cycle 5 of Phase 3 measurement. Risk 2 remains *open at the operational level* until Phase 3 prototype measurement observes the rates; the specification-level closure is the cycle 106 deliverable.
- **Risk 3:** goal-coherence enforcement requires planner to evaluate executor / curator / reconciler outputs; this is itself a substantive decision-class that may exceed the ~3 per cycle estimate when goals are ambiguous or sub-tasks diverge.
- **Risk 4 (CLOSED at specification level cycle 108):** the per-role decision count assumes each role's session is single-pass (one invocation per super-step). If sub-tasks require iteration within a role's super-step (e.g., executor needs to revisit a structural decision after curator review), per-role decision count multiplies. **Cycle 108 closes Risk 4 at the specification level** — see the [Per-role iteration decomposition section](#per-role-iteration-decomposition-cycle-108-specification-addressing-risk-4) above. The closure decomposes within-super-step iteration into 5 trigger types under B's architectural elements (α curator-review-driven / β goal-coherence-redirect / γ reconciler-triggered / δ plan-lifecycle / ε branch-spawn), specifies pre-agreed iteration-event count thresholds (≤3/cycle direction-validated / (3, 6] at-risk / >6 refuted in steady state past cycle 5 of Phase 3 measurement) as primary falsifiable bound with iteration multiplier (M ≤1.3 / 1.3-1.8 / >1.8) as secondary derived metric, specifies per-iteration-event logging at Phase 3 with three-layer verification (orchestrator self-logging + external-observer reproducibility ≥80% + CI sweep), specifies iteration-event-classification rubric stability with warm-up window discipline (cycles 1-5 + post-warm-up bounds; shape #24 HARDENED@3 transfer), and specifies a combined-readings diagnostic (per-role decision count × iteration events) preserving the Risk-1-vs-Risk-4 distinction at Phase 3 measurement. Risk 4 remains *open at the operational level* until Phase 3 prototype measurement observes the iteration-event rates and rubric inter-rater disagreement; the specification-level closure is the cycle 108 deliverable.
- **Risk 5:** the aggregate-comparable-to-v1 claim depends on per-role agents NOT also doing v1's coordination work (substantive-focal selection, sibling-pattern recognition, honest reflection). If those decision-classes leak across role boundaries (e.g., curator does its own honest reflection in addition to planner's coordination), aggregate inflates.
- **Risk 6 (added cycle 96 per PR #2878 absorption — multi-session wall-clock cost):** B requires 4 sequential AI sessions per cycle (planner → executor → curator → reconciler). Each session has cold-start cost: context loading, workspace state synchronization, tool initialization. This overhead is NOT part of the decision-count metric (it's wall-clock, not cognitive). If each cold-start costs ~5-10 minutes and there are 4 sessions in a 75-minute window, **20-40% of cycle budget is consumed before any substantive work begins**. Validation against v1's wall-clock baseline (single session, single cold-start) is required separately from decision-count validation. This is a P3-adjacent structural risk that the cycle 92 sharpening did not name.
- **Risk 7 (added cycle 96 per PR #2878 absorption — circular baseline):** the v1 ~40-decision baseline used in this section was self-measured by the orchestrator that benefits from the comparison. The author is the same orchestrator that designed B and proposed B's per-role decision counts. The 12 "per-axis position commitments for C" element of the baseline counts template applications as 12 independent decisions; if those count as 1 structural decision each, the v1 baseline approximately halves and B's "comparable" claim breaks. The baseline therefore admits a 2× uncertainty band on the v1 side.
- **Risk 8 (added cycle 96 per PR #2878 absorption — counting protocol absence; CLOSED at specification level cycle 103):** the validation plan named "count actual per-role decision points per role" as a measurement step but did not specify a counting methodology. Self-report by the role-session AI is circular; counting LLM tool invocations conflates mechanical with cognitive. **Cycle 103 closes Risk 8 at the specification level** — see the [Counting protocol section](#counting-protocol-cycle-103-specification-addressing-risk-8) above. The protocol uses *structural decision-points in role artifacts* as the primary unit (avoiding self-report circularity by enumerating artifacts not behavior); uses external enumeration with reproducibility check (avoiding single-observer subjectivity); uses operational tool-invocation logs as a sanity-check secondary unit (without conflating mechanical with cognitive — categorized invocations distinguish the two); and specifies hard refutation thresholds anchored on ratio to v1's verified-under-rubric baseline. Risk 8 remains *open at the operational level* until Phase 3 prototype measurement actually executes the protocol; the specification-level closure is the cycle 103 deliverable.
- **Risk 9 (added cycle 97 per PR #2877 lens-2 absorption — typed-channel infrastructure LOC dominance):** B's typed-channel-map architecture requires `channel-router` + per-channel reducer schemas + super-step-boundary sync + coordination invariants as foundational infrastructure before any role logic runs. PR #2877 names this as the dominant LOC sink in B's design — infrastructure plausibly exceeds role-specific logic in aggregate. Three sub-points: (a) `branch-manager` (Axis 4 branching checkpoints) implies fork/promotion/gardening semantics and is policy-heavy + test-heavy beyond the 200-500 LOC small-crate norm; (b) ~20-40 skill crates plus role-bound manifests/contracts multiply lifecycle overhead before business logic; (c) B's own document at line 62 acknowledges "several thousand LOC each" for key infra — supporting PR #2877's higher-LOC reading. **Implication:** B's aggregate range stated 10000-20000 LOC is plausibly low at the upper bound; PR #2877's revised 14000-28000 (40% upper-bound increase) is the better-grounded planning range. Direction (B is the most expensive candidate) holds by construction; magnitude is sharpened upward.
- **Risk 10 (added cycle 97 per PR #2877 lens-7 absorption — skill discovery overhead super-linear):** B's 20-40 skill-crate surface combined with role-bound discovery patterns implies non-trivial routing/lookup costs at runtime. PR #2877 names a candidate-independent threshold of ~18-25 callable units beyond which one-line registry descriptions are insufficient for safe invocation selection without strict hierarchical taxonomy. B's surface (~32-52 callable units total: 12+ Rust crates + 20-40 skills) substantially exceeds this threshold. Mitigation requires hierarchical registry + compatibility metadata + per-skill invocation contract — itself additional infrastructure cost not in the 10000-20000 LOC stated range. Risk: skill-discovery lookup costs explode without strong taxonomy, eroding the per-role specialization benefit by re-introducing aggregate decision overhead at the discovery layer.

These risks are bounded — none threaten the candidate's central decomposition bet directly. They threaten the *magnitude* of the per-role mitigation. **Per cycle 96 absorption (PR #2878 Finding 13): the cycle 92 "comparable to v1" claim should be re-classified as "ambiguous-without-prototype (range 85-168% of v1 depending on cycle character)."** The claim only holds at the lower bound (steady-state cycles); upper bound (heavy inbound / branch-spawn cycles) is 68% above v1 baseline. Whether heavy cycles are frequent vs rare is not measured. **Direction (specialized roles reduce per-role decision-class diversity) holds by construction; aggregate magnitude is now indeterminate without (a) verified v1 baseline measured by external observer with explicit counting protocol, (b) measured firing-rate of B's adopted sub-shapes, (c) wall-clock measurement including cold-start cost.**
