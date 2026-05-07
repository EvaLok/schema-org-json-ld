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
- **Tool migration:** ~12+ new Rust crates + ~20-40 skill crates. Aggregate net-add ~10000-20000 LOC. Multi-cycle effort to build out the full skill suite.
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
