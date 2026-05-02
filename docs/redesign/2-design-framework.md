# Phase 2 design framework — convergent constraints + design axes for v2 candidates

## Status

**v1.10 (cycle 47, 2026-05-02).** Phase-2-input artifact-in-progress. Subject to
iteration before any Phase 2 candidate generation begins (which itself
requires post-retrospective checkpoint approval).

This file is the **live working framework** that v2 candidates will be
generated from and evaluated against. The frozen historical record of
each iteration step lives in the corresponding `_notes/cycle-N-*.md` file.

### Iteration history

| Version | Cycle | Source | Summary of changes |
|---|---|---|---|
| v1.0 | 35 (2026-04-30) | `_notes/cycle-35-phase2-design-axes-and-cold-reader.md` | Initial Phase 2 design-axes synthesis: 7 convergent constraints + 11 axes + cross-axis dependency map + F-pattern→axis mapping + Phase 2 candidate template + 5 open framework questions |
| v1.1 | 36 (2026-04-30) | `_notes/cycle-36-cold-reader-and-framework-iteration.md` | Cold-reader on v1.0: F11→Axis 9 corrected to F11→Axis 4+Axis 2 (Q[b] FAIL); Axis 2 plans-as-artifacts row removed (Q[a] cleanup); decisions documented for 5 open questions, deferred to v1.2 application |
| v1.2 | 37 (2026-05-01) | `_notes/cycle-37-framework-v1.2-application-and-cold-reader.md` | Framework promoted to dedicated file `2-design-framework.md`. Six deferred decisions applied: Q[c] constraint 7 wording refinement, Q1 Axis 11→constraint 8 promotion, Q2 Axis 12 (Reconciliation discipline) added, Q3 ordering disclaimer added, Q4 Axis 13 (Harness-vs-session boundary) added, Q5 preserved-primitives subsection added. Cycle-37 cold-reader correction: F11 mapping refined to Axis 4+Axis 12 (drop Axis 2 from direct mapping; document indirect contribution in cross-axis deps). |
| v1.3 | 38 (2026-05-01) | `_notes/cycle-38-cold-reader-and-v1.3-application.md` | Cycle-38 cold-reader on v1.2: three pre-commit questions all PASS (F11→Axis 4+Axis 12 stands; Axis 13 medium-vs-fat is real differentiation; constraint 8 is meaningful). Two refinements: (i) add "Axis 13 × Axis 7" cross-axis dependency (situational-review implementation strategy); (ii) backfill Maps-to lines on Axes 1, 3, 5, 6, 7 (cycle-37 same-cycle review minor finding (5) inherited from v1.0/v1.1). One flag for cycle-39+ verification: Axis 12 "v1-derived" caveat may be too strong given LangGraph interrupts as broader-axis analogue. |
| v1.4 | 39 (2026-05-01) | `_notes/cycle-39-cold-reader-and-redispatch-escalation.md` | Cycle-39 cold-reader on v1.3: three pre-commit questions all PASS again (re-dispatch trigger did NOT fire on comment-on-existing-issue; Axis 12 "v1-derived" caveat correct; F-pattern table levels correct including F9→Axis 7 with Axis 13 indirect via cross-axis deps). Cycle-38 "v1-derived caveat may be too strong" flag verified-and-retired: HITL primitives in LangGraph/AutoGen are synchronous pause-resume mechanisms, structurally different from async reconciliation; clarification sentence added to Axis 12 Status. Cycle-39 cold-reader OVERCAUTIOUS finding mirrors cycle-37's Q[b] OVERCAUTIOUS pattern. |
| v1.5 | 41 (2026-05-01) | `_notes/cycle-41-deeper-read-per-finding-evaluation.md` | Cycle-41 substantive findings from PR #2804 + #2805 deeper-read deliverables (Cognition + OpenAI). Three Cognition framework corrections applied: (i) Axis 1 — Cognition's "Don't Build Multi-Agents" (June 2025) was substantially walked back in April 22, 2026 follow-up; durable invariant is **writes-stay-single-threaded**, not single-threaded execution; Cognition now ships Managed Devins (coordinator + parallel children) and joins the small-fixed-team row; (ii) Axis 3 — Cognition has multi-layer memory architecture (5+ documented mechanisms); context-trace framing qualified to "primary in-session mechanism, multi-layer at longer horizons"; (iii) Axis 9 — 45-min session limit is unverified after direct primary-source access (docs say "if you can do it in three hours"); status downgraded from `documented-claim` to `unverified-after-direct-access`. Plus: Axis 9 OpenAI counter-evidence (Ralph Wiggum Loop has no iteration ceiling — pattern does NOT transfer to cron-driven systems). Plus: Status header v1.3→v1.5 freshness fix (was missed cycle-39 v1.4 bump). Two flags for cycle-42+: Axis 12 "Most likely v2 candidate position" annotation softening (Q[b] cold-reader); openclaw deeper-read dispatch (Q[c] BORDERLINE-PASS). |
| v1.6 | 42 (2026-05-01) | `_notes/cycle-42-cold-reader-and-v1.6-application.md` | Cycle-42 cold-reader on v1.5: Q(a) found two internal inconsistencies introduced by v1.5's Axis 1 Cognition update that were not propagated to other framework sections — applied. Q(b) PASS — C7 (microVM) and O7 (companion post) qualifications adequately propagated to per-system files; no over-acceptance. Q(c) decided both deferred flags warrant cycle-42 action — Axis 12 hybrid annotation softening applied; openclaw deeper-read dispatch executed via close-and-recreate primitive. Three v1.6 changes: (i) Axis 7 row — Cognition moved from "Single-pattern (one shape only)" to multi-pattern coexisting (Apr 2026 ships Managed Devins + Devin Review + Smart Friend — system-level multi-pattern); (ii) Cross-axis dependency map (Constraint 8 × Axis 1) — stale "(Cognition)" parenthetical example removed since Cognition now in small-fixed-team row; (iii) Axis 12 hybrid row annotation — replaced "Most likely v2 candidate position" (forward-looking forecast that could prejudice Phase 2 candidate generation) with cost-grounded descriptive reasoning ("Lowest per-channel design cost — different channels have different frequencies"). |
| v1.7 | 43 (2026-05-01) | `_notes/cycle-43-openclaw-per-finding-evaluation.md` | Cycle-43 substantive findings from PR #2809 deeper-read deliverable (openclaw, 893 lines, primary-source). 21 findings evaluated, 21 accepted (4 with qualification, 1 as revision-of-prior-claim, 0 rejected). Three framework changes: (i) Axis 2 row — openclaw added to "File-per-component" position with `global-state.ts` caveat (per-agent state isolation in `~/.openclaw/agents/<agentId>/`; Gateway-level globals exist per `src/global-state.ts` but contents not verified); 4-system support; (ii) Axis 3 row — openclaw note refined to clarify singleton-slot scope is the storage/retrieval layer, not full memory architecture (full architecture is layered: Markdown files + SQLite + active-memory sub-agent + dreaming consolidation); (iii) Axis 9 row — openclaw added to "Runtime ceiling" position with 48h-effectively-unbounded qualifier; stuck-session watchdog (`diagnostics.stuckSessionWarnMs`) noted as more interesting primitive than the bare timeout. Plus cycle-43 Q(c) cold-reader refinement: Axis 12 four-position table cost-framing balanced — "High-cost" → "Uniform mechanism (one pattern per channel); per-channel implementation overhead" on Active polling; "Lowest per-channel design cost" → "Mixed mechanism; design overhead spread per-channel-class rather than per-channel" on Hybrid. Also: cycle-40 v2-design observation about three reconciliation patterns refined (NOT retired) — openclaw's pattern is implementation-detail within Axis 12's existing **Event-driven** position, not a new axis position; the cross-system observation is now TWO axis-distinct patterns (sync HITL vs async) with implementation-nuance within async (cron+catchup, event-driven with persistent connections, webhook-on-event). |
| v1.8 | 44 (2026-05-01) | `_notes/cycle-44-cold-reader-and-v1.8-application.md` | Cycle-44 cold-reader on v1.7: Q(a) BORDERLINE-FAIL on Axis 2 × Axis 3 cross-axis dep map phrasing precision (cycle-43 same-cycle Q1 had flagged as minor; cycle-44 cross-cycle escalated to load-bearing). Three changes applied: (i) Axis 2 × Axis 3 dep map rewritten to name specific Axis 3 positions that align with each Axis 2 position (file-per-component aligns with three filesystem-based memory positions: singleton plugin slot WITH filesystem storage, top-level architectural principle with filesystem memory, wiki+search with file-per-entry); (ii) F8 mapping rationale extended to mention stuck-session watchdog as detection-and-recovery primitive alongside Bounded loops as prevention primitive (covers v1.7-introduced openclaw `diagnostics.stuckSessionWarnMs` instance); (iii) Axis 12 event-driven annotation wording-symmetry — added "shared inbound infrastructure" framing alongside the "requires X" cost framing (event-driven uses shared infrastructure: one webhook handles all subscribed channels, vs N readers for active polling). Q(b) PASS — per-finding evaluation calibration adequate (OC9 qualification appropriately hedged, OC13 verdict consistent with pattern transfer to existing COPILOT-DISPATCHES three-tier structure). Q(c) PASS — cost-framing balance adequate; minor wording-symmetry opportunity (item iii above) addressed. |
| v1.9 | 45 (2026-05-02) | `_notes/cycle-45-cold-reader-and-v1.9-application.md` | Cycle-45 cold-reader on v1.8: Q(a) PASS (Axis 2 × Axis 3 dep map closing caveat "supportive rather than exclusive" with explicit "non-filesystem Axis 3 positions" catchall covers file-per-component + non-filesystem-memory scenario); Q(b) PASS (stress test file-per-component + context-trace memory is explicitly named in caveat as non-precluded); Q(c) BORDERLINE-FAIL escalation from cycle-44's BORDERLINE-PASS — F8 mapping rationale's "or" between bounded-loops and stuck-session-watchdog covers Axis 9 positions 2-3 (Loop count, Runtime) but doesn't explicitly name position 4 (Both). Cross-cycle review surfaces the structural difference vs F5's "or" (Axis 2 has no "both" position; Axis 9 does have a composable Both position). One change applied: F8 mapping rationale rewritten as comma-separated three-way enumeration ("Bounded loops, stuck-session watchdog, or both compositionally (Axis 9's `Both (loop + runtime)` position)"). |
| v1.10 | 47 (2026-05-02) | `_notes/cycle-47-cold-reader-and-v1.10-application.md` | Cycle-47 cold-reader on v1.9: Q(a) PASS on F8 rationale (re-walked with fresh adversarial framing including the "+single-implementation discipline" CDP-citation distinction; F8's CDP column citation is content-driven distinct from F1/F6/F7's Axis 13 citation). Q(b) BORDERLINE-FAIL on cross-axis dep map sweep — Axis 13's Maps-to is missing the F9 indirect-contributor annotation that Axis 1's Maps-to has, even though the Axis 13 × Axis 7 dep map entry documents Axis 13's role in F9 ("Axis 13 shapes the implementation strategy"). The asymmetry was unintentional: cycle-39 verified Axes 1/3/5/6/7 backfilled Maps-to lines but Axis 13's Maps-to (added at v1.2) was not re-reviewed. One change applied: Axis 13 Maps-to extended with "Indirect contributor to F9 (adversarial-review treadmill) — fat-harness shapes the implementation strategy for Axis 7's situational-review by controlling when review fires; the load-bearing F9 fix is Axis 7." F-pattern table NOT modified (preserves cycle-39's explicit verdict that "F-pattern table levels are correct; the cross-axis dep is the right level for Axis 13 × Axis 7's contribution to F9"). Q(c) procedural decision: bounded-mechanical capacity for #809 closure consideration. |

## Purpose and scope

This framework consolidates Phase 1's 16 cross-system patterns + 3 persistent
divergences + v1's failure-mode catalog into a structured Phase-2-input
artifact. Two top-level structural elements:

- **Convergent constraints.** Patterns where 3+/N surveyed systems converge.
  Every v2 candidate must honor these or explicitly disagree with load-bearing
  rationale. A candidate that violates a convergent constraint is a candidate
  that disagrees with all surveyed systems' converged practice — that
  disagreement should be deliberate, not accidental.
- **Real design axes.** Patterns where surveyed systems diverge. Each axis is
  a meaningful candidate-differentiation point. Each candidate must declare
  its position; multiple positions are defensible.

Plus four supporting structural elements:

- **Cross-axis dependency map.** Significant inter-axis constraints and
  near-orthogonality observations.
- **Mapping to v1 failure modes.** Which axes a candidate must address well
  to structurally fix each F-pattern from the retrospective.
- **Preserved-primitives interactions.** How v1's preserved primitives (per
  redesign prompt SECTION 3) constrain candidate axis positions.
- **Phase 2 candidate template (preliminary).** Suggested structure for
  candidate documents.

## Note on ordering

**Axis numbering is for reference only; no significance, priority, or
load-bearingness ranking is implied.** Candidates may address axes in any
order, prioritizing what is load-bearing for their specific design. The
numbering reflects the chronological order of axis identification in the
v1.0→v1.2 iteration; gaps in numbering (Axis 11 absent) reflect demotion or
removal during iteration and are deliberate provenance markers, not errors.

## Convergent constraints (every v2 candidate must honor)

Eight constraints. A candidate that violates one of these is disagreeing with
all surveyed systems' converged practice; the disagreement should be explicit
and load-bearing.

1. **Code-vs-prompt split exists.** Deterministic code executes; LLM proposes.
   (Family B pattern 1, 3+/6 systems, foregrounded convergence with substrate
   variations.) v1 honors this in shape (Rust tools + LLM orchestrator) but
   the prompt encodes procedure that should be in tools (CORE-DESIGN-
   PRINCIPLE violation).

2. **Failed work is recorded as artifact, not silently discarded.**
   (Family C pattern 2, 3+/3 systems with structural similarity.) v1 has
   journal failure notes but no failure-record file with read-after-failure
   semantics.

3. **Strong-defaults security with operator-controlled knobs.**
   (Family D, 3+/3 + scope condition.) v1's GitHub Actions secret-handling
   and the prompt's UNTRUSTED-TEXT-RULES already honor this; v2 should
   preserve.

4. **Per-agent model selection treated as primitive.** (Family A pattern 2,
   3+/3 + diversity hedge.) v1 uses a single model (Claude Opus) for the
   orchestrator; Copilot dispatch model is per-dispatch. v2 candidates
   should treat model selection as per-component, with per-component
   rationale.

5. **Anti-patterns documented explicitly as deliverable artifact.**
   (Family E pattern 1, 3+/6 systems.) v1's retrospective is the
   anti-pattern catalog; v2 prompt should preserve and extend
   (e.g., "what we will not do" sections per the openclaw VISION.md
   pattern).

6. **Small core, capability extends via something.** (Family B pattern 2,
   3+/7 systems.) The "something" is a real choice (see Axis 6 below),
   but the principle (lean entry point + extension mechanism) is
   convergent.

7. **Memory is treated as architectural elevation, not derivative of state.**
   (Family C pattern 4, 5/5 surveyed systems where memory is named elevate
   memory architecturally; 0 surveyed systems treat memory as derivative-
   of-state.) The shape of the memory subsystem is a choice (see Axis 3);
   the convergent practice — memory has first-class architectural treatment
   — is the constraint. *(Voyager's SkillManager+Chroma is adjacent; framed
   as skill-storage by the source repo, not counted in the elevation
   evidence.)*

8. **Goal-driven over operator-driven (top-level posture).** (Family A
   pattern 3, persistent divergence in surveyed systems; v2 candidates
   committed to goal-driven by mission.) The redesign's primary thesis
   (autonomous self-healing AI pursuing schema-domain work) commits to
   goal-driven as the top-level posture. Operator-driven sub-systems
   may exist within a goal-driven overall posture (e.g., Eva-issued
   `input-from-eva` directives as explicit operator-commands, integrated
   via Axis 12 reconciliation), but the top-level operator-vs-goal choice
   is fixed by mission. Promoted from former Axis 11 (cycle 37); a
   non-differentiating axis is a constraint.

## Real design axes (v2 candidates differ on)

Twelve axes (numbered 1-10, 12, 13; Axis 11 absent — promoted to constraint
8 in v1.2). Each axis is a meaningful candidate-differentiation point.

### Axis 1 — Agent decomposition

**The choice:** how is the orchestrator session decomposed into agents/roles?

| Position | Systems supporting | Notes |
|---|---|---|
| Single-threaded linear | Cognition Devin June 2025 ("Don't Build Multi-Agents") | Position substantially walked back April 22, 2026; durable invariant is **writes-stay-single-threaded**, not single-threaded execution |
| Small fixed team with role-separation | Voyager (4 agents), AutoGen Magentic-One (lead + workers), oh-my-codex (30 named role prompts), Cognition Devin Apr 2026 (Managed Devins coordinator + parallel children + clean-context Devin Review + Smart Friend consultation) | 4+/4 with writes-single-threaded as constraint, not multi-agent prohibition |
| Multi-agent peer (uncontrolled) | None | Rejected by 4+/6 systems as default; Cognition explicitly rejects "unstructured swarms" in April 2026 follow-up |

**v1's position:** single-threaded with Copilot dispatches as parallel workers
(off-process). The dispatches are not "agents" in the small-fixed-team sense
— they're per-task externally-delegated work.

**v2 candidate space:** retain dispatch-as-worker (current shape) vs adopt
small-fixed-team within the orchestrator session itself (e.g., planner /
executor / critic / curator). The convergent constraint across all 4 systems
that ship multi-agent designs is **writes stay single-threaded** — the
load-bearing invariant is write-discipline, not agent-count. Candidates
that allow multi-agent decomposition must declare how writes stay
single-threaded; candidates that adopt single-threaded execution can cite
the broader invariant rather than agent-count specifically.

**Cross-axis dependency:** Axis 1 × Axis 7 (orchestration topology) —
single-threaded forces single-topology; small-fixed-team enables but doesn't
force multi-topology coexistence.

**Maps to:** F7 (self-management dominance — role-specialization, including
a dedicated reviewer / curator / reconciler agent, reduces self-management
surface for the primary agent). Indirect contributor to F9 (adversarial-
review treadmill) via dedicated-reviewer-role.

### Axis 2 — State representation primitive

**The choice:** what is the unit of persistent state?

| Position | Systems supporting | Notes |
|---|---|---|
| Single global state file | None | v1's `state.json` is the explicit anti-example; 3+/5 systems agree |
| File-per-component | AutoGen, Voyager (`ckpt/<agent>/`), oh-my-codex (`.omx/state/<mode>-state.json`), openclaw (`~/.openclaw/agents/<agentId>/` per-agent state isolation; Gateway-level globals exist per `src/global-state.ts`, contents not verified) | 4+/5 + diversity hedge |
| Typed-channel-map within one schema | LangGraph | Persistent divergence — one pole |
| Repository-as-state | OpenAI harness | git substrate; ephemeral worktrees |

*Plans-as-artifacts is a separate temporal/lifecycle dimension; see Axis 5.*

**v1's position:** monolithic `state.json` (42 keys, 62-69% defense-character
per F12 catalog). 4-6× reduction estimated for v2 per cycle-5 measurement.

**v2 candidate space:** every position EXCEPT "single global state file" is
defensible. The choice between file-per-component and typed-channel-map is
the persistent State-shape divergence (Family C); a candidate must commit
to one or explicitly span both. The repository-as-state position has
interesting properties for a public-repo orchestrator (commits ARE state)
but conflicts with the journal/notes-file conventions if state mutations
land in journal entries vs separate state files.

**Cross-axis dependency:** Axis 2 × Axis 3 (memory) — file-per-component
naturally supports memory-as-component-file; typed-channel-map naturally
supports memory-as-channel; repo-as-state supports memory-as-files-in-repo.

**Maps to:** F12 (state accretion), F5 (state.json as procedural-leak), F3
(multi-candidate state drift). Indirect contributor to F11 (post-close
mutations) — file-per-component naturally supports per-component append,
making Axis 4's append-only easier; the load-bearing F11 fix is Axis 4 +
Axis 12.

### Axis 3 — Memory subsystem shape

**The choice:** if memory is first-class (per convergent constraint 7),
what shape does it take?

| Position | Systems supporting | Notes |
|---|---|---|
| Singleton plugin slot (one mechanism active, replaceable) | openclaw | Persistent divergence — one pole. Cycle-43 deeper read: the singleton-slot scope is the storage/retrieval LAYER (`plugins.slots.memory`); the full memory architecture is layered on top — Markdown files (`MEMORY.md` + daily notes + `DREAMS.md`) + SQLite index + embedding-based hybrid search + active-memory sub-agent + dreaming background consolidation. |
| Top-level architectural principle | PAI Principle 13 | Persistent divergence — other pole |
| Context trace (everything-the-agent-has-done) | Cognition Devin (primary in-session mechanism; multi-layer at longer horizons) | Cycle-41 deeper read documents 5+ memory mechanisms (cross-session notes, Knowledge API, Playbooks, DeepWiki, Session Insights, hypervisor snapshots); context-trace is the in-session label |
| Repository-as-record | OpenAI harness | "Anything not in-context doesn't exist" |
| Wiki + search (markdown-first, search-first) | oh-my-codex (`.omx/wiki/` + MCP server) | Bounded context injection |
| Typed channels (short/long-term distinction) | LangGraph (Store + checkpointer) | Closest to v1's intermediate-cache shape |
| Memory derivative of state (no first-class) | None | Rejected by 3+/5 |

**v1's position:** memory is mostly derivative of state.json (no first-class
memory). The journal acts as long-term reflective memory; the worklog is
short-term transient. No semantic memory primitive.

**v2 candidate space:** every position except "memory derivative of state"
is defensible. The choice has significant downstream consequences for what
an orchestrator session can recall across cold-starts. Persistence-mechanism
evolution (cycle-2 to cycle-33+) has been organic-markdown-files; that maps
closest to wiki+search but without the search infrastructure.

**Cross-axis dependency:** Axis 3 × Axis 2 (state) — memory shape follows
state representation; Axis 3 × Axis 1 (decomposition) — small-fixed-team
can have per-agent memory subsystems.

**Maps to:** Convergent constraint 7 (memory architectural elevation —
Axis 3 makes the specific shape choice within the constraint). Indirect
contributor to F7 (self-management dominance) via cold-start cost — rich
memory reduces re-derivation each cycle, freeing compute for primary work.
Axis 3's load-bearing role is constraint-7-shape rather than direct
F-pattern fix.

### Axis 4 — History/Provenance substrate

**The choice:** where does append-only history live?

| Position | Systems supporting | Notes |
|---|---|---|
| Branching checkpoints (in-process versioning) | LangGraph time-travel | "`update_state` does not roll back; creates new checkpoint that branches" |
| Versioned files (`<name>V2.js`, `V3.js`) | Voyager skill versioning | Filesystem versioning |
| Git-as-substrate | OpenAI harness | Repository as state; commits append; ephemeral worktrees |
| One-way file migration with read-only legacy | oh-my-codex | Schema migrations one-way, not destructive |

**v1's position:** journal + worklog + commits are append-only-by-git but
`state.json` is destructive (writes overwrite). The draft-then-promote /
append-only retention pattern (Eva advisory #2408) is the targeted v2 fix.

**v2 candidate space:** all four positions are defensible. Git-as-substrate
has natural alignment for a public-repo orchestrator (every commit is the
audit trail). Branching checkpoints have appeal for the "what-if" reasoning
multi-cycle work needs but aren't trivially mappable to a flat-file-on-disk
substrate. The one-way migration shape is the conservative choice for
evolving the persistence mechanism itself across cycles.

**Constraint from preserved-primitives:** branching positions must be in-tree
files (per-branch-named files committed in main), not git-branches that
might not be pushed (per git-safety primitive — every commit must be
pushed).

**Maps to:** F11 (post-close mutations) — append-only with branching
prevents the destructive-write semantics that lose post-close mutations
from history. F12 (state accretion via non-destructive write semantics).
F4 (frozen-artifact lifecycle fragility) — substrate determines what
"frozen" means.

### Axis 5 — Plans/specs as forward artifacts

**The choice:** are plans/specs first-class versioned artifacts written
before execution, or reconstructed-after?

| Position | Systems supporting | Notes |
|---|---|---|
| Yes — plans-as-artifacts (active/completed/technical-debt) | OpenAI harness | Plan files checked into repo; per-category lifecycle |
| Yes — context snapshots before execution | oh-my-codex | `.omx/context/{task-slug}-{timestamp}.md` with explicit fields |
| No — plans live in-message or are reconstructed from history | Most others (none explicitly support reconstruction-after as primitive) | Default in absence of plan-artifact infrastructure |

**Status:** 2-system clean convergence. Lower convergence than other patterns;
treat as candidate-considered axis, not constraint.

**v1's position:** plans live in cycle issue comments + journal entries +
occasional `_notes/` files. No structured plan-artifact lifecycle. The
redesign has implicit cycle-N→cycle-N+1 plan suggestions in notes files
but no separate plan-files-on-disk.

**v2 candidate space:** adopting plans-as-artifacts forces a filesystem
layout decision (`plans/active/`, `plans/completed/`, `plans/technical-debt/`)
and a transition lifecycle. Skipping this axis means relying on journal +
notes for the same purpose.

**Maps to:** F4 (frozen-artifact lifecycle fragility) — plan lifecycle
primitives (`active/completed/technical-debt`) address freeze/refresh
timing as a structural design choice rather than ad-hoc per-artifact
handling.

### Axis 6 — Extension shape

**The choice:** if small-core extends via something, what?

| Position | Systems supporting | Notes |
|---|---|---|
| Plugins | openclaw | "Core stays lean; optional capability ships as plugins" |
| Skills | PAI, oh-my-codex (39 skills) | Skill = code + prompt + invocation contract |
| Tools | LangGraph (`ToolNode`), AutoGen (model-emits-tool-call) | LLM-discoverable invocation primitives |
| Layers | PAI 16 principles, AutoGen Core/AgentChat/Extensions/Studio/Bench | Architectural-layer composition |
| Harness-accumulation (depth-first) | OpenAI harness | Capabilities added iteratively as failures surface |
| Configuration-layer-with-hooks | oh-my-codex (on top of unmodified Codex CLI) | Wrap-without-replace |

**v1's position:** Rust binaries in `tools/` directory with shell-wrapper
scripts. No formal "skill" or "plugin" abstraction; tools are discovered by
file-existence + naming convention.

**v2 candidate space:** retaining Rust-tools-as-extension shape is the path
of least migration cost, with the question being whether to add a discovery/
registration primitive (skill / plugin manifest) on top.

**Maps to:** Convergent constraint 6 (small core, capability extends via
something — Axis 6 makes the specific extension-mechanism choice). Axis 6's
load-bearing role is constraint-6-shape rather than direct F-pattern fix.
Folds polyglot / multi-language schema strategy as schema-domain extension
choice (Phase 3 prototype's load-bearing test, not a v2 prompt-level axis).

**Considered-and-folded:** polyglot / multi-language schema strategy is
schema-domain-specific. Phase 3 prototype includes one polyglot end-to-end
test. The polyglot strategy is part of Phase 3 design, not the v2
prompt-level axes — language-port tools are extensions and fold into Axis 6.

### Axis 7 — Orchestration topology

**The choice:** how do agents/components coordinate?

| Position | Systems supporting | Notes |
|---|---|---|
| Single-pattern (one shape only) | None in surveyed systems' current shipping architectures | Cognition June 2025 advocated this in "Don't Build Multi-Agents"; April 2026 walkback ships multi-pattern. v1's rigid checklist-driven sequence is the closest extant example — and is the v1 anti-pattern. |
| Multi-pattern coexisting | AutoGen (round-robin/selector/swarm/graph), LangGraph (chaining/routing/parallelization/orchestrator-worker/ReAct/subgraphs/supervisor), Cognition Apr 2026 (Managed Devins coordinator + parallel children, Devin Review clean-context, Smart Friend frontier consultation) | 3+/3 in surveyed |
| Sequential mode transitions with deterministic transition policy | oh-my-codex (`STATE_MODEL.md` allowlist) | Modes governed by allowlist preventing illegal shifts |
| Lead-worker hierarchy | AutoGen Magentic-One, Cognition Apr 2026 (Managed Devins) | Specialized workers under orchestrator |
| Peer-flow | Voyager (curriculum → action → critic → skill) | Round-robin among role-specialized peers |

**v1's position:** rigid checklist-driven sequence (STARTUP → C phases →
COMPLETION). One topology, encoded in two checklist files.

**v2 candidate space:** the rigid-checklist position has been explicitly
named as Phase 2 anti-pattern (per CORE-DESIGN-PRINCIPLE). Multi-pattern
coexisting with deterministic transition policy (oh-my-codex shape) is the
strongest match for "orchestrator handles novel situations" because
different situations may call for different topologies.

**Cross-axis dependency:** Axis 7 × Axis 1 (decomposition) — see Axis 1.
Axis 7 × Axis 13 (harness-vs-session boundary) — fat-harness can implement
situational-review by controlling when review fires, supporting Axis 7's
multi-pattern situational invocation; thin/medium harness leaves WHEN-
review decisions in prompt.

**Maps to:** F6 (cyclomatic procedure depth — multi-pattern with transition
policy lighter than rigid checklist) and F9 (adversarial-review treadmill —
situational invocation breaks the every-cycle review-firing loop).

**Considered-and-folded:** Eva-checkpoint mechanism specifics ("what
triggers a checkpoint") fold into Axis 7 — the topology determines what
state transitions are checkpoint-eligible. The companion question ("how
does the orchestrator know it's at a checkpoint") folds into Axis 12
(Reconciliation discipline) — checkpoint-detection is an inbound-channel
question.

### Axis 8 — Mechanical enforcement scope

**The choice:** what is regression-tested mechanically?

| Position | Systems supporting | Notes |
|---|---|---|
| None | Default in absence of explicit infrastructure | Rare in surveyed |
| Data-shape only | LangGraph (TypedDict / dataclass / Pydantic), Voyager (init-time `count == len(skills)`) | Diversity hedge — adjacent to behavioral enforcement |
| Behavioral promises + agent-affecting prose | OpenAI (custom linters with agent-readable error messages), oh-my-codex (prompt-contract regression tests on `prompts/`) | 2-system strict |

**v1's position:** Rust tools have unit tests; pipeline-check has sub-checks.
No regression tests on the orchestrator prompt or checklist text — agent-
affecting prose can change without CI catching.

**v2 candidate space:** adopting behavioral-prose CI is a high-leverage v2
move because it directly addresses F1 (constraint-without-tool ratio): a
constraint added to the prompt without a paired tool fix would surface as
a CI test the orchestrator must honor mechanically, OR be rejected pre-
merge.

**Maps to:** F1 (constraint accretion), F5 (state.json as procedural-leak),
CORE-DESIGN-PRINCIPLE violation detection.

### Axis 9 — Iteration ceilings

**The choice:** are autonomous loops bounded?

| Position | Systems supporting | Notes |
|---|---|---|
| None (open-ended runs) | Rare in surveyed | Implicit in v1's per-cycle non-bounded retry |
| Loop count ceilings | oh-my-codex (`max_iterations=10`, `max=5`), Voyager (`action_agent_task_max_retries=4`) | 2-system strict |
| Runtime ceiling | ~~Cognition Devin (45-min session limit, *documented-claim*)~~ — **unverified after cycle-41 direct primary-source access**; docs say "if you can do it in three hours, Devin can most likely do it"; hypervisor snapshot infrastructure supports hours-long sessions. openclaw (`agents.defaults.timeoutSeconds` default 172800s = 48h, effectively-unbounded for typical use; the **stuck-session watchdog** `diagnostics.stuckSessionWarnMs` is a more interesting primitive — detects stale lanes and can release them) | Anchor weakened on Cognition; OpenAI Ralph Wiggum Loop is **counter-evidence** (no iteration ceiling, human backstop is the bound — does NOT transfer to cron-driven autonomous systems); openclaw's stuck-session watchdog is the transfer-relevant primitive |
| Both (loop + runtime) | None explicitly in surveyed | Composable |

**v1's position:** per-cycle there is no per-loop ceiling. The cycle ITSELF
is the only ceiling (~75 minutes of compute). Pipeline-check sub-checks
can re-fire, dispatch can retry, etc., without a bounded loop count.

**v2 candidate space:** loop-count ceilings are bounded-mechanical to add
and immediately reduce the failure surface for runaway-autonomy. Runtime
ceiling is a coarser ceiling (cycle-level already has it).

**Maps to:** F8 (abandonment cascades), F7 (self-management dominance via
unbounded re-firing).

### Axis 10 — Entropy / AI-slop mitigation

**The choice:** is output-quality drift addressed as recurring infrastructure?

| Position | Systems supporting | Notes |
|---|---|---|
| Not addressed | Default | Implicit in v1's accretion-as-defense pattern (F12) |
| Golden principles + doc-gardening agent | OpenAI harness | Recurring agent-quality cleanup |
| Mandatory deslop pass post-completion | oh-my-codex | Quality cleanup embedded in task completion contract |
| Both | None explicitly in surveyed | Composable |

**Status:** 2-system clean convergence. Lower convergence than other patterns;
treat as candidate-considered axis, not constraint. Inversely-related to
v1's accretion-as-defense pattern (F12) — these systems treat accretion as
a failure mode to clean, not a defensive structure to preserve.

**v1's position:** no entropy-mitigation primitive. F12's defense-accretion
pattern is the explicit anti-direction.

**v2 candidate space:** adopting an entropy-mitigation primitive is high-
leverage if F12 is to be addressed structurally rather than via "defenses
re-examined for load-bearingness" (per the retrospective's Defense-
accretion implication).

**Maps to:** F12 (defense accretion).

**Considered-and-folded:** failure-mode catalog maintenance ("does v2
update its own anti-patterns catalog?") folds into convergent constraint 5
(Anti-patterns documented as deliverable artifact) and Axis 10 (the "how
is it kept current" mechanism).

### Axis 11 — *(absent — promoted to convergent constraint 8 in v1.2)*

Former Axis 11 was Operator-vs-Goal framing. Cycle 37's iteration determined
that a non-differentiating axis (every v2 candidate must take the same
position by mission commitment) is a constraint, not an axis. See
convergent constraint 8.

The numbering gap is a deliberate provenance marker. v2 candidates may
still reference "operator-driven sub-system" choices (per constraint 8's
note), but the top-level posture is fixed.

### Axis 12 — Reconciliation discipline

*(v1-derived; not externally validated by surveyed Phase 1 systems)*

**The choice:** how does the system reconcile inbound external events
(Eva responses, audit posts, dispatch outputs, post-close tool mutations)
into state?

| Position | Notes |
|---|---|
| No reconciliation: write-only outbound channels | v1 anti-pattern (F2/F3/F4/F11 emerge from this) |
| Active polling: each outbound channel paired with a reader producing state transitions | Uniform mechanism (one pattern per channel); per-channel implementation overhead |
| Event-driven: state changes reactively when external events arrive | Reactive handling; shared inbound infrastructure (one webhook or GitHub Actions trigger handles all subscribed channels — for a public-repo orchestrator the Actions platform is already-paid; per-event handler configuration is a bounded one-time cost); openclaw's Gateway is an instance — channels maintain persistent upstream connections, agent runs are per-event discrete turns |
| Hybrid: polling for low-frequency channels, event-driven for high-frequency | Mixed mechanism; design overhead spread per-channel-class rather than per-channel; suited to workloads where different channels have different natural frequencies |

**Status:** v1-derived axis; no external system surveyed has an Eva-equivalent
that would constrain the choice. Candidates that address Axis 12 are doing
more design work than those addressing externally-validated axes; candidates
may also choose to fold this into existing axes (e.g., Axis 4 history
substrate where event-driven means "git events trigger state recompute")
rather than treating as separate.

**Note: HITL primitives are not reconciliation analogues** *(verified
cycle 39, retired cycle-38's "v1-derived caveat may be too strong" flag)*.
LangGraph interrupts and AutoGen HITL primitives are synchronous pause-resume
mechanisms — the caller is the active sender of `Command(resume=...)` (or
equivalent), the graph/agent is the passive receiver waiting at a specific
node. Axis 12's reconciliation concerns asynchronous absorption of external
events that arrive independently of the orchestrator's execution thread
(Eva responds when she responds; audit posts when audit posts; PR merges
when reviewers merge). The orchestrator cannot pause-and-wait — it runs on
a cron and must catch up to whatever happened since last cycle. Different
structural shape; HITL is not a reconciliation analogue. AutoGen explicitly
disclaims "global reconciliation of all component states" (per-system file).

**v1's position:** no reconciliation. Outbound channels (issue creates, PR
creates, journal commits) are well-developed; inbound reconciliation does
not exist. The retrospective documents F2/F3/F4 as direct manifestations.

**v2 candidate space:** every position EXCEPT "no reconciliation" is
defensible. Hybrid is the path of least design-cost since different channels
naturally have different polling frequencies.

**Cross-axis dependency:** Axis 12 × Axis 4 (history substrate) — event-
driven reconciliation pairs naturally with git-as-substrate (commits as
events); Axis 12 × Axis 1 (decomposition) — small-fixed-team can have a
dedicated reconciliation agent.

**Maps to:** F2 (Eva-response detection), F3 (multi-candidate state drift,
partial — close-out doesn't reconcile against post-close evidence), F4
(frozen-artifact lifecycle fragility — worklog freeze without refresh),
F11 (post-close mutations — worklog never reads state back).

**Considered-and-folded:** audit-repo integration mechanism is part of
Axis 12 — audit-orchestrator posts are an inbound channel requiring
reconciliation.

### Axis 13 — Harness-vs-session boundary

*(cross-cutting CORE-DESIGN-PRINCIPLE elaboration)*

**The choice:** where is the line between deterministic harness code and
LLM session?

| Position | Notes |
|---|---|
| Thin harness, fat session | Most procedure in prompt; LLM re-derives procedure each cycle (v1's shape) |
| Medium harness, medium session | Split between cycle-runner and prompt; harness handles known patterns, prompt handles novel |
| Fat harness, thin session | Most procedure in deterministic code; prompt is small reference + judgment-call decisions |

**Status:** cross-cutting CORE-DESIGN-PRINCIPLE elaboration. Every v2
candidate must declare its position; the principle requires "tools and
deterministic processes handle repetitive, rote, procedural work" — implying
the harness-vs-session line should be drawn farther toward fat-harness than
v1's shape.

**v1's position:** thin harness (cycle-runner mostly invokes the session),
fat session (prompt + 2 checklists encode the procedure the orchestrator
follows each cycle).

**v2 candidate space:** medium-or-fat harness positions are the CORE-DESIGN-
PRINCIPLE-aligned choices. Thin harness is the v1 anti-pattern. The choice
between medium and fat depends on what procedures get extracted into tools
— a candidate must specify the tool surface implied (per the Phase 2
candidate template's "Tool surface implied" section).

**Constraint from preserved-primitives:** Axis 13 positions must specify
the cycle-runner change scope (none / modest / substantial) — cycle-runner
is preserved as the harness entrypoint, and Axis 13 positions imply
different changes to it.

**Cross-axis dependency:** Axis 13 × Axis 6 (extension shape) — the
extension primitive (plugins/skills/tools/etc.) shapes how harness
procedures get organized; Axis 13 × Axis 8 (mechanical enforcement) —
fat harness implies more mechanical-enforcement surface area.

**Maps to:** F1 (constraint accretion in prompt — fat harness extracts
procedural constraints), F6 (cyclomatic procedure depth — fat harness
extracts procedure), F7 (self-management dominance via prompt-encoded
procedure), CORE-DESIGN-PRINCIPLE explicitly. Indirect contributor to
F9 (adversarial-review treadmill) — fat-harness shapes the
implementation strategy for Axis 7's situational-review by controlling
when review fires; thin/medium harness leaves WHEN-review decisions in
prompt where the every-cycle-review pattern tends to recur; the load-
bearing F9 fix is Axis 7.

**Considered-and-folded:** prompt size budget (how long is the prompt?)
isn't a candidate-differentiation axis per se; it's an outcome of Axis 13's
position. Smaller prompts fall out of fat-harness candidates. Cold-start
ergonomics (how much does a cold-start session need to read before being
productive?) is workflow detail that shapes Axis 13's specific extraction
choices but doesn't differentiate at architecture level.

## Cross-axis dependency map

Significant inter-axis constraints:

- **Axis 1 (decomposition) × Axis 7 (orchestration topology):** Single-
  threaded forces single-topology. Small-fixed-team enables but doesn't
  force multi-topology coexistence.
- **Axis 2 (state) × Axis 3 (memory):** State representation shapes
  which Axis 3 positions are natural — file-per-component aligns with
  filesystem-based memory positions (singleton plugin slot WITH
  filesystem storage as in openclaw's `~/.openclaw/agents/<agentId>/`;
  top-level architectural principle with filesystem memory as in PAI;
  wiki+search with file-per-entry as in oh-my-codex's `.omx/wiki/`);
  typed-channel-map aligns with typed channels with checkpointer
  (LangGraph); repo-as-state aligns with repository-as-record (OpenAI
  harness). The natural-alignment framing is supportive rather than
  exclusive: file-per-component does not preclude context-trace memory
  or other non-filesystem Axis 3 positions, but pairs more naturally
  with the listed filesystem-based positions.
- **Axis 4 (history substrate) × Axis 2 (state):** State representation
  choice constrains history substrate options — file-per-component pairs
  naturally with one-way migration or git; typed-channel-map pairs with
  branching checkpoints. *Indirect F11 contribution: file-per-component
  Axis 2 makes per-component append (Axis 4) easier to implement; the
  load-bearing F11 fix remains Axis 4 (append semantics) + Axis 12
  (reconciliation), with Axis 2 as enabling infrastructure.*
- **Axis 8 (mechanical enforcement) × Axis 5 (plans-as-artifacts) × Axis
  10 (entropy mitigation):** Mechanical enforcement is the substrate
  enabling both plan-lifecycle CI checks and golden-principles enforcement.
  Adopting Axis 8 unlocks the others.
- **Axis 12 (reconciliation) × Axis 4 (history substrate):** Event-driven
  reconciliation pairs naturally with git-as-substrate (commits as events;
  webhook on push triggers state recompute).
- **Axis 12 (reconciliation) × Axis 1 (decomposition):** Small-fixed-team
  can have a dedicated reconciliation agent (the "curator" or "reconciler"
  role); single-threaded must interleave reconciliation work with primary
  work.
- **Axis 13 (harness-vs-session) × Axis 6 (extension shape):** The extension
  primitive (plugins/skills/tools/etc.) shapes how harness procedures get
  organized; fat-harness needs a richer extension story.
- **Axis 13 (harness-vs-session) × Axis 8 (mechanical enforcement):** Fat
  harness implies more mechanical-enforcement surface area (more
  deterministic code to lint and test).
- **Axis 13 (harness-vs-session) × Axis 7 (orchestration topology):** Fat-
  harness can implement Axis 7's multi-pattern situational-review by
  controlling when review fires (vs every cycle). Thin/medium harness leaves
  WHEN-review decisions in prompt, where the v1 anti-pattern (every-cycle
  review-firing) tends to recur. F9 (adversarial-review treadmill) is
  primarily fixed by Axis 7 (situational vs fixed); Axis 13 shapes the
  implementation strategy for that fix.
- **Constraint 8 (goal-driven) × Axis 1 (decomposition):** Goal-driven
  pairs naturally with single-threaded long-running execution; goal-
  driven within small-fixed-team requires explicit goal-coordination
  primitive (Cognition's Managed Devins coordinator pattern is one
  surveyed instance — coordinator scopes child tasks to maintain
  goal-coherence across parallel children).

Largely orthogonal:

- **Axis 4 (history) × Axis 6 (extension shape)** — independent.
- **Axis 9 (iteration ceilings) × any other axis** — additive primitive.
- **Axis 10 (entropy mitigation) × Axis 1 (decomposition)** — entropy
  mitigation can be implemented at any decomposition.

## Mapping to v1 failure modes

Axis-to-Fpattern mapping. The retrospective's "v2 design implications by
family" section provides high-level guidance; this mapping is more axis-
specific. Updated in v1.2 with Axis 12 + Axis 13 mappings; F11 corrected
per cycle-37 cold-reader.

| F-pattern | Family | Most-relevant axes | Rationale |
|---|---|---|---|
| F1 (constraint accretion) | Defense accretion | Axis 8, Axis 13 | Mechanical CI on prompt contracts forces constraint-as-test or rejection; fat-harness extracts procedural constraints from prompt to tools |
| F2 (Eva-response detection) | Reconciliation | Axis 12 | Direct match — Eva-response polling/event-detection is the reconciliation primitive |
| F3 (multi-candidate state drift) | Reconciliation | Axis 2, Axis 12 | Single source of truth per concern (Axis 2) + reconciliation against post-close evidence (Axis 12) |
| F4 (frozen-artifact lifecycle fragility) | Reconciliation | Axis 4, Axis 5, Axis 12 | History substrate determines what "frozen" means; lifecycle primitives address freeze/refresh timing; reconciliation refreshes frozen artifacts |
| F5 (state.json as procedural-leak) | Defense + Reconciliation | Axis 2, Axis 8 | File-per-component or typed-channel separates concerns; mechanical CI catches procedural-leak patterns |
| F6 (cyclomatic procedure depth) | Procedure overhead | Axis 7, Axis 13 | Multi-pattern with transition policy lighter than rigid checklist; fat-harness extracts procedure from prompt |
| F7 (self-management dominance) | Procedure overhead | Axis 1, Axis 8, Axis 9, Axis 13 | Specialization + mechanical enforcement + iteration ceilings + fat-harness reduce self-management surface |
| F8 (abandonment cascades) | Tooling fragility | Axis 9, CORE-DESIGN-PRINCIPLE | Bounded loops (loop-count ceiling positions; prevention), stuck-session watchdog (runtime-ceiling positions; detection-and-recovery; openclaw's `diagnostics.stuckSessionWarnMs` instance — detect stale runs and release lanes), or both compositionally (Axis 9's `Both (loop + runtime)` position) + single-implementation discipline (no parallel implementations) |
| F9 (adversarial-review treadmill) | Procedure overhead | Axis 7 | Multi-pattern shape replaces fixed adversarial-review step with situational invocation |
| F10 (audit's value is broader read scope) | Design-implication | Not a v2 axis | Audit-side concern; audit-as-peer pattern preserved per redesign prompt SECTION 2 |
| F11 (post-close mutations) | Defense + Reconciliation | Axis 4, Axis 12 | Append-only history (Axis 4) prevents destructive write semantics that lose post-close mutations; reconciliation discipline (Axis 12) refreshes frozen worklog against post-close state. *(Axis 2 indirect contributor — see cross-axis deps; not load-bearing for direct F11 fix.)* |
| F12 (defense accretion catalog) | Defense | Axis 2, Axis 4, Axis 10 | All three contribute; Axis 10 is the structural anti-accretion primitive |

**Observation 1 (post-v1.2):** With Axis 12 added, F2/F3/F4/F11's
reconciliation-asymmetry family is now structurally addressable. v1's
write-only outbound pattern is the named anti-pattern; every v2 candidate
must declare a non-"no reconciliation" Axis 12 position.

**Observation 2:** Multiple Fs map to the same axes (Axis 2, 4, 8, 12, 13
each address 3+ failure modes). This isn't a problem — it's evidence those
axes are high-leverage. A v2 candidate that picks well on Axes 2, 4, 8, 12,
13 addresses ~9 of the 11 failure-modes structurally.

**Observation 3:** CORE-DESIGN-PRINCIPLE (tools handle rote; orchestrator
handles judgment) shows up across F1, F6, F7, F8 — it is itself an
axis-cross-cutting constraint. Axis 13 makes the specific candidate-
differentiation choice along the CDP direction explicit; CDP itself remains
the directional statement every candidate must demonstrate.

## Preserved-primitives interactions

v1's preserved primitives (per redesign prompt SECTION 3) constrain v2
candidates' axis positions. Walking each preserved primitive against the
axes:

| Preserved primitive | Axes implicated | Constraint implied |
|---|---|---|
| Journal (`docs/journal/YYYY-MM-DD.md`, freeform per-cycle) | Axis 3 (memory shape) | Journal remains as one memory channel; candidates may add others. Note: Axis 3 "memory derivative of state" position is doubly rejected (constraint 7 + journal-as-existing-channel). |
| Cycle-issue (`orchestrator-run` label, session-bracket comments) | Axis 7 (orchestration topology) | All topologies must produce session-end summary on cycle-issue. Multi-pattern coexisting topologies may have multiple sub-cycles within one cycle-issue boundary — sub-cycles are internal to the issue. |
| Question-for-eva / input-from-eva | Axis 12 (Reconciliation) | Inbound Eva channels must be reconciled. Pure write-only outbound rejected (the v1 F2 anti-pattern). |
| Git-safety (commit-must-be-pushed) | Axis 4 (history substrate) | Branching positions must be in-tree files (per-branch-named files committed in main), not git-branches that might not be pushed. Git-as-substrate position naturally honors this. |
| Cycle-runner harness | Axis 13 (Harness-vs-session boundary) | Cycle-runner change scope must be declared (none / modest / substantial). Different Axis 13 positions imply different changes; candidates must specify. |

**Note on constraint surface area:** preserved-primitives interactions add
explicit constraints atop the axis position-space. A candidate that picks
"branching checkpoints" on Axis 4 must specify in-tree-files implementation;
a candidate that picks "fat harness" on Axis 13 must specify the cycle-runner
change scope; etc. These are not new axes — they're refinements of position
specifications.

## Phase 2 candidate template (preliminary)

A Phase 2 candidate should declare its position on each of the 12 axes
(1-10, 12, 13) plus the CORE-DESIGN-PRINCIPLE elaboration (folded into
Axis 13 in v1.2), the cross-axis dependencies it commits to, and the
preserved-primitives constraints it honors. Suggested structure:

```
## Candidate <N>: <name>

### Position summary
- Axis 1 (decomposition): <position> — <one-sentence rationale>
- Axis 2 (state representation): <position> — <one-sentence rationale>
- Axis 3 (memory shape): <position> — <one-sentence rationale>
- Axis 4 (history substrate): <position> — <one-sentence rationale>
- Axis 5 (plans-as-artifacts): <position> — <one-sentence rationale>
- Axis 6 (extension shape): <position> — <one-sentence rationale>
- Axis 7 (orchestration topology): <position> — <one-sentence rationale>
- Axis 8 (mechanical enforcement): <position> — <one-sentence rationale>
- Axis 9 (iteration ceilings): <position> — <one-sentence rationale>
- Axis 10 (entropy mitigation): <position> — <one-sentence rationale>
- Axis 12 (reconciliation discipline): <position> — <one-sentence rationale>
- Axis 13 (harness-vs-session): <position> — <one-sentence rationale>

### Cross-axis commitments
- Axis 1 × Axis 7: <how this candidate handles the dependency>
- Axis 2 × Axis 3: <...>
- Axis 4 × Axis 2: <...>
- Axis 12 × Axis 4: <...>
- Axis 13 × Axis 6: <...>
- Axis 13 × Axis 8: <...>
- ... (other significant pairs)

### Failure-mode addressing
- F1: <how candidate addresses>
- ... (12 patterns)

### Preserved-primitives compliance
- Journal: <integration shape>
- Cycle-issue: <integration shape>
- Question-for-eva / input-from-eva: <reconciliation mechanism>
- Git-safety: <how branching/append-only honors commit-must-be-pushed>
- Cycle-runner: <change scope: none / modest / substantial; specifics>

### What this candidate gives up
- Honest list of design dimensions where this candidate is weaker than
  alternatives — what it trades away to gain its strengths.

### Tool surface implied
- List of tools the candidate's prompt expects to invoke; which exist;
  which would be net-new to build.

### Migration cost from v1
- Specific migration steps; what state/tools/conventions transfer vs need
  replacement.
```

The template is preliminary and subject to iteration before Phase 2
candidate generation begins. The post-retrospective checkpoint gates that
work; this template is preparation, not commitment.

## What the framework does NOT yet specify

Honest gaps for cycle-38+ iteration:

- **Security posture per-trust-tier specifics.** Convergent constraint 3
  (Strong-defaults security with operator-controlled knobs) is named but
  the trust-tier specifics (how does the prompt handle untrusted text from
  different sources?) are folded into the convergent constraint as
  implementation detail rather than candidate-differentiation axis. v2
  candidates must honor; specifics are not axis-level.
- **Polyglot strategy for schema-domain work.** Folded into Axis 6
  (extension shape — language-port tools are extensions). Phase 3
  prototype's polyglot end-to-end test is the load-bearing test. Phase
  2 candidate generation may surface that polyglot deserves explicit
  axis treatment if candidates diverge significantly here.
- **Concrete reconciliation primitives.** Axis 12's positions are
  abstract (no reconciliation / active polling / event-driven / hybrid).
  v2 candidates need to specify the actual GitHub-Actions / cron / webhook
  / state-recompute mechanism. Cycle 38+ may add a "reconciliation
  primitive catalog" subsection to Axis 12.
- **Phase 1 research for systems queued by Eva directives.** Cognition
  Devin (#2779) and OpenAI harness (#2781) re-dispatches were authorized
  by Eva (#2794) but not yet executed. Their findings may surface
  additional cross-system patterns that constrain or differentiate Phase
  2 candidates further. Cycle-37 deferred re-dispatch to allow framework
  v1.2 application; cycle-38+ will execute.
