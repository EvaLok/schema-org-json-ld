# Phase 2 design framework — convergent constraints + design axes for v2 candidates

## Status

**v1.2 (cycle 37, 2026-05-01).** Phase-2-input artifact-in-progress. Subject to
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
| Single-threaded linear | Cognition Devin (named-rejection of multi-agent) | Strongest published anti-stance ("Don't Build Multi-Agents") |
| Small fixed team with role-separation | Voyager (4 agents), AutoGen Magentic-One (lead + workers), oh-my-codex (30 named role prompts) | 3+/3 with Cognition contradiction |
| Multi-agent peer (uncontrolled) | None | Rejected by 3+/6 systems as default |

**v1's position:** single-threaded with Copilot dispatches as parallel workers
(off-process). The dispatches are not "agents" in the small-fixed-team sense
— they're per-task externally-delegated work.

**v2 candidate space:** retain dispatch-as-worker (current shape) vs adopt
small-fixed-team within the orchestrator session itself (e.g., planner /
executor / critic / curator). Cognition's named-rejection makes single-
threaded a defensible default.

**Cross-axis dependency:** Axis 1 × Axis 7 (orchestration topology) —
single-threaded forces single-topology; small-fixed-team enables but doesn't
force multi-topology coexistence.

### Axis 2 — State representation primitive

**The choice:** what is the unit of persistent state?

| Position | Systems supporting | Notes |
|---|---|---|
| Single global state file | None | v1's `state.json` is the explicit anti-example; 3+/5 systems agree |
| File-per-component | AutoGen, Voyager (`ckpt/<agent>/`), oh-my-codex (`.omx/state/<mode>-state.json`) | 3+/5 + diversity hedge |
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
| Singleton plugin slot (one mechanism active, replaceable) | openclaw | Persistent divergence — one pole |
| Top-level architectural principle | PAI Principle 13 | Persistent divergence — other pole |
| Context trace (everything-the-agent-has-done) | Cognition Devin | Strongest "memory is the trace" framing |
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

**Considered-and-folded:** polyglot / multi-language schema strategy is
schema-domain-specific. Phase 3 prototype includes one polyglot end-to-end
test. The polyglot strategy is part of Phase 3 design, not the v2
prompt-level axes — language-port tools are extensions and fold into Axis 6.

### Axis 7 — Orchestration topology

**The choice:** how do agents/components coordinate?

| Position | Systems supporting | Notes |
|---|---|---|
| Single-pattern (one shape only) | Cognition (single-threaded linear) | Forces simplicity at cost of flexibility |
| Multi-pattern coexisting | AutoGen (round-robin/selector/swarm/graph), LangGraph (chaining/routing/parallelization/orchestrator-worker/ReAct/subgraphs/supervisor) | 3+/3 in surveyed |
| Sequential mode transitions with deterministic transition policy | oh-my-codex (`STATE_MODEL.md` allowlist) | Modes governed by allowlist preventing illegal shifts |
| Lead-worker hierarchy | AutoGen Magentic-One | Specialized workers under orchestrator |
| Peer-flow | Voyager (curriculum → action → critic → skill) | Round-robin among role-specialized peers |

**v1's position:** rigid checklist-driven sequence (STARTUP → C phases →
COMPLETION). One topology, encoded in two checklist files.

**v2 candidate space:** the rigid-checklist position has been explicitly
named as Phase 2 anti-pattern (per CORE-DESIGN-PRINCIPLE). Multi-pattern
coexisting with deterministic transition policy (oh-my-codex shape) is the
strongest match for "orchestrator handles novel situations" because
different situations may call for different topologies.

**Cross-axis dependency:** Axis 7 × Axis 1 (decomposition) — see Axis 1.

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
| Runtime ceiling | Cognition Devin (45-min session limit, *documented-claim*) | 1 adjacent partial — bounds runtime not iteration |
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
| Active polling: each outbound channel paired with a reader producing state transitions | High-cost; requires per-channel discipline |
| Event-driven: state changes reactively when external events arrive | Requires inbound trigger infrastructure (webhook, GitHub Actions on event) |
| Hybrid: polling for low-frequency channels, event-driven for high-frequency | Most likely v2 candidate position |

**Status:** v1-derived axis; no external system surveyed has an Eva-equivalent
that would constrain the choice. Candidates that address Axis 12 are doing
more design work than those addressing externally-validated axes; candidates
may also choose to fold this into existing axes (e.g., Axis 4 history
substrate where event-driven means "git events trigger state recompute")
rather than treating as separate.

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
procedure), CORE-DESIGN-PRINCIPLE explicitly.

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
  natural memory primitive — file-per-component → memory-as-component-file;
  typed-channel-map → memory-as-channel; repo-as-state → memory-as-files-
  in-repo.
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
- **Constraint 8 (goal-driven) × Axis 1 (decomposition):** Goal-driven
  pairs naturally with single-threaded long-running (Cognition); goal-
  driven within small-fixed-team requires explicit goal-coordination
  primitive.

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
| F8 (abandonment cascades) | Tooling fragility | Axis 9, CORE-DESIGN-PRINCIPLE | Bounded loops + single-implementation discipline (no parallel implementations) |
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
