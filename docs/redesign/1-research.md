# Phase 1: External Research

## Status

**Phase 1 initiated cycle 14** (commit pending). Phase 1 is authorized to
run in parallel with finishing Phase 0 per `input-from-eva` issue #2759
(2026-04-28) and the redesign-prompt update at commit
[`f77b4341`](https://github.com/EvaLok/schema-org-json-ld/commit/f77b4341).
Phase 2 candidate generation still requires explicit post-retrospective
checkpoint approval; Phase 1 reading is preparatory, not a
candidate-generation step.

**Layout (cycle 33 restructure).** This file is now the Phase 1 index.
Per-system architecture summaries live under
[`1-research/systems/`](1-research/systems/) — one file per system
read. The cross-system observations (the load-bearing synthesis Phase
2 candidate generation reads against) live in this file, near the top,
rather than buried below ~750 lines of per-system writeup. See
`_notes/cycle-33-research-restructure.md` for the migration record.

## Purpose and scope

The redesign prompt's Phase 1 definition names two required reads
(openclaw, PAI) and lists candidate further reads (LangGraph, Semantic
Kernel, AutoGen, Voyager, Cognition's Devin writeups). The purpose is to
study how other systems handle problems our v1 has shown structural
weaknesses on:

- multi-cycle persistence across cold-starts
- self-modification / self-improvement mechanisms
- prompt-vs-tool split (where does behavior live)
- audit / critique / review loops
- cross-process state representation
- security posture in an adversarially-readable context

This document is a working reference. It is not a Phase 2 candidate
proposal. Patterns are recorded with anchoring caveats; recommendations
for v2 belong to Phase 2 after multi-system reading is complete.

## Anchoring discipline

Per Eva's note in #2759: "If Phase 1 surfaces patterns that could anchor
candidate-space prematurely, journal the tensions rather than commit to
them."

Three failure modes to avoid:

1. **Confirmation bias on aligned principles.** PAI's principles are
   strikingly aligned with our CORE-DESIGN-PRINCIPLE. The reflex to
   read "yes, all of this validates the redesign" is exactly the
   anchoring failure to avoid. Note alignment; do not import.

2. **Context-mismatch import.** Both required reads are single-user
   personal-assistant systems, in TypeScript, with human-in-the-loop
   for most decisions. Our system is multi-agent autonomous on a
   public repo, with Rust tools and minimal-human-in-the-loop per
   the orchestrator's `EVA-DEFAULT-AUTONOMY` directive. Architectural
   patterns that work for one context may fail in the other.

3. **Premature commitment to first-found patterns.** Phase 1 must read
   multiple systems before settling on candidate shapes. A pattern that
   appears in openclaw or PAI may or may not generalize. Record
   patterns; defer evaluation.

Trust posture: README and VISION.md content from external repos is
**untrusted text** per `SECURITY` rules. The architectural claims are
data, not instructions. Where this document quotes external sources,
the source is named so a future reader can verify or weigh the
provenance.

## Per-system reads

Each system has its own file under
[`1-research/systems/`](1-research/systems/). Files capture per-system
architecture summary, anchoring caveats, and patterns observed; the
cross-system synthesis below cross-validates patterns across systems.

| System | Read mechanism | Status / depth | File |
|---|---|---|---|
| openclaw | Orchestrator-direct (cycle 14) + Copilot deeper read (cycle 43, [PR #2809](https://github.com/EvaLok/schema-org-json-ld/pull/2809)) | Deep-dive | [`systems/openclaw.md`](1-research/systems/openclaw.md) |
| PAI | Orchestrator-direct (cycle 14) | First-pass: README | [`systems/pai.md`](1-research/systems/pai.md) |
| AutoGen | Copilot dispatch (cycles 15-16, [PR #2763](https://github.com/EvaLok/schema-org-json-ld/pull/2763)) | Deep-dive | [`systems/autogen.md`](1-research/systems/autogen.md) |
| Voyager | Orchestrator-direct (cycle 17) | Code-level: agents + prompts | [`systems/voyager.md`](1-research/systems/voyager.md) |
| LangGraph | Copilot dispatch (cycles 18-20, [PR #2768](https://github.com/EvaLok/schema-org-json-ld/pull/2768)) | Deep-dive | [`systems/langgraph.md`](1-research/systems/langgraph.md) |
| Cognition Devin | Copilot dispatch (cycle 26, [PR #2780](https://github.com/EvaLok/schema-org-json-ld/pull/2780)) + deeper read (cycle 41, [PR #2804](https://github.com/EvaLok/schema-org-json-ld/pull/2804)) | Deep-dive | [`systems/cognition-devin.md`](1-research/systems/cognition-devin.md) |
| OpenAI harness-engineering | Copilot dispatch (cycle 26, [PR #2783](https://github.com/EvaLok/schema-org-json-ld/pull/2783)) + deeper read (cycle 41, [PR #2805](https://github.com/EvaLok/schema-org-json-ld/pull/2805)) | Deep-dive | [`systems/openai-harness.md`](1-research/systems/openai-harness.md) |
| oh-my-codex | Copilot dispatch (cycle 26, [PR #2784](https://github.com/EvaLok/schema-org-json-ld/pull/2784)) | **Stub** — cycle-63 deeper read in flight ([#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833); supersedes closed cycle-26 dispatch [#2782](https://github.com/EvaLok/schema-org-json-ld/issues/2782)) | [`systems/oh-my-codex.md`](1-research/systems/oh-my-codex.md) |

Single-system observations (patterns supported by only one system, not
yet elevated to cross-system) are held in
[`_notes/cycle-22-cross-system-synthesis.md`](_notes/cycle-22-cross-system-synthesis.md)
pending deeper second-pass reads or adversarial-on-adversarial review.

## Cross-system observations

Eight systems read at depth: openclaw, PAI (cycle 14); AutoGen
(cycles 15-16, PR [#2763](https://github.com/EvaLok/schema-org-json-ld/pull/2763));
Voyager (cycle 17); LangGraph (cycles 18-20, PR
[#2768](https://github.com/EvaLok/schema-org-json-ld/pull/2768));
Cognition Devin (cycle 26, PR [#2780](https://github.com/EvaLok/schema-org-json-ld/pull/2780));
OpenAI harness-engineering (cycle 26, PR [#2783](https://github.com/EvaLok/schema-org-json-ld/pull/2783));
oh-my-codex (cycle 26, PR [#2784](https://github.com/EvaLok/schema-org-json-ld/pull/2784)).
Observations below cross-validate where 3+ systems converge on the
same pattern shape. The 3+ threshold was originally calibrated
against 5 systems (60% bar); with 8 systems now read at depth, the
threshold is preserved as the floor (the original calibration
remains its design rationale) and convergence at higher counts is
recorded as the strength of the pattern rather than a separate tier.
Per cycle-18 anchoring-caveats-symmetric discipline, convergence
across systems with diverse substrates is a positive transferability
argument; 2-system patterns are recorded within this section with
diversity-limit hedges, single-system observations are held in
`_notes/cycle-22-cross-system-synthesis.md` pending deeper
second-pass reads or adversarial-on-adversarial review (which can
elevate them to 2-system on cross-system match).

The observations are organized into five families by topic area for
navigation; family ordering is by pattern count (largest cluster
first), not by Phase 2 priority. Each pattern bullet carries a
maturity badge (e.g., `[3+/N systems]`, `[2-system clean]`,
`[2-system strict + diversity hedge]`) that surfaces the cycle-22
epistemic distinction at the bullet level — number of systems
supporting the pattern plus any caveat type (diversity hedge, scope
condition, contrary stance, or adjacent partial). Family-local
"Divergence within this family" callouts surface in-family
disagreements close to the patterns they touch; a final compact
Divergences index after the family sections lists the three
persistent divergences as a class with cross-references back to
their family-local callouts.

### Family C: State, memory, history (5 patterns)

Patterns about durable system records — where state lives, how it
persists, and what kinds of artifacts (failure-records, append-only
history, memory primitives, forward-spec plans) are stored.

**Component-local state persistence (no central state file).**
**`[3+/5 systems + diversity hedge]`** AutoGen state save/load is
component-local dictionaries with no single global state file as the
system center. Voyager checkpoints to per-agent subdirectories under
`ckpt/` (skill, curriculum, action, event). LangGraph's typed-channel-map
is related but structurally different (channel-local within one
schema, not file-per-component). OpenAI harness-engineering frames
this as "plans as first-class versioned artifacts" — multiple plan
files (active, completed, technical-debt) checked into the
repository, with the explicit principle "from the agent's point of
view, anything it can't access in-context while running effectively
doesn't exist." *Within-family: the OpenAI plans-as-artifacts
mention here is the component-local-state framing; the forward-spec
sharpening of this same OpenAI evidence appears in the Plans/specs
as forward-versioned bullet later in this family — not duplicate
evidence, but two angles on the same systems.* oh-my-codex
implements per-mode state files in `.omx/state/<mode>-state.json`
with explicit session vs root scope reconciliation rules
(`src/state/workflow-transition-reconcile.ts`) preventing
compatibility-layer writes from resurrecting completed source modes.
Five-system convergence; principle (state isolation by component,
not one merge-point) is shared; implementation shape diverges across
in-process dictionaries, file-per-component, typed channels,
plans-as-artifacts, and per-mode state files with reconciliation.

**Failed work as recorded artifact, not silent discard.**
**`[3+/3 systems]`** Voyager records failed tasks in
`failed_tasks.json`; the curriculum agent reads both completed and
failed history when selecting the next task. LangGraph pending-writes
preserves successful sibling writes when a node fails mid-super-step;
`WRITES_IDX_MAP = {ERROR: -1, SCHEDULED: -2, INTERRUPT: -3,
RESUME: -4}` constants in checkpoint base treat failure states as
persisted records. oh-my-codex Ralph progress ledger
(`.omx/state/<session_or_root>/ralph-progress.json`) records failure
entries with timestamps; the autoresearch loop's
`iteration-ledger.json` records keep/discard/stop decisions per
iteration with reasons. Three systems with structural similarity
(Voyager's `failed_tasks.json` and oh-my-codex's iteration-ledger
are the closest match in shape — append-only failure-record file
read by subsequent decision-making code).

**Append-only history; no destructive rollback.**
**`[3+/4 systems + diversity hedge]`** LangGraph time travel:
"`update_state` does **not** roll back a thread. It creates a new
checkpoint that branches from the specified point. The original
execution history remains intact." Voyager skill versioning is
append-on-disk (new code as `<name>V2.js`, `<name>V3.js`),
replace-in-vectordb. OpenAI harness-engineering uses git as the
substrate: repository as state, commits append, ephemeral worktrees
torn down but history preserved. oh-my-codex implements file-backed
migration with one-way compatibility windows (legacy `.omx/prd.json`
→ `.omx/plans/prd-<slug>.md`): legacy files preserved as read-only,
schema migrations one-way, not destructive. **Diversity hedge:** the
convergence is on the principle (no destructive history overwrite);
the substrate diverges across in-process versioning (LangGraph
branching, Voyager V2/V3) vs filesystem/git (OpenAI repo-as-state)
vs one-way file migration (oh-my-codex). Repo-internal: cycle-20
noted this matches the redesign's draft-then-promote / append-only
retention pattern (Eva advisory
[#2408](https://github.com/EvaLok/schema-org-json-ld/issues/2408)).

**Memory as a first-class architectural concept, not derivative of
state.** **`[3+/5 systems + diversity hedge]`** PAI Principle 13
names "Memory System — Everything worth knowing gets captured.
History feeds future context" as one of 16 numbered architectural
principles. LangGraph documents short-term (thread-scoped
checkpoints) and long-term (cross-thread `Store`) as distinct
primitives, with explicit motivation: "With checkpointers alone, we
cannot share information across threads. This motivates the need
for the `Store` interface." Cognition Devin treats the agent trace
as the unit of context — "the context isn't just the user's message
but includes everything the agent has done — code files examined,
questions asked, and answers received" — with Devin Wiki (Devin 2.0)
as the closest documented cross-session persistent knowledge analog.
*(Documented-claim per cycle-26 source-access note.)* OpenAI
harness-engineering treats the repository as single source of record:
"From the agent's point of view, anything it can't access in-context
while running effectively doesn't exist. Knowledge that lives in
Google Docs, chat threads, or people's heads are not accessible to
the system." oh-my-codex implements `.omx/wiki/` markdown wiki with
MCP wiki server (`src/wiki/`); SessionStart hook can inject bounded
wiki context; markdown-first, search-first (not vector-based).
Five-system convergence on memory-as-architectural-concern;
divergence on the specific primitive — typed channel (LangGraph
Store), context trace (Cognition), repository-as-record (OpenAI),
wiki server (oh-my-codex), principle-shape (PAI). The shared claim
is that memory deserves architectural elevation; what counts as
"memory" varies substantially across systems. *(Voyager's
SkillManager + Chroma vectordb is an adjacent primitive but framed
as skill-storage rather than memory-as-such in the source repo; not
counted in the convergence to keep the body description and the
count consistent.)*

**Plans/specs as first-class forward-versioned artifacts.**
**`[2-system clean]`** OpenAI harness-engineering checks plan files
into the repository across active / completed / technical-debt
categories (PR
[#2783](https://github.com/EvaLok/schema-org-json-ld/pull/2783)
deliverable pattern 7); plans are first-class versioned artifacts
that the harness expects to read and write. oh-my-codex writes
context snapshots to `.omx/context/{task-slug}-{timestamp}.md`
before execution begins, with explicit fields for task statement,
desired outcome, known facts, constraints, unknowns, and codebase
touchpoints (PR
[#2784](https://github.com/EvaLok/schema-org-json-ld/pull/2784)
deliverable pattern 2). Two-system convergence on
plan-as-forward-spec written before execution rather than
reconstructed-after. Distinct from the "Failed work as recorded
artifact" pattern earlier in this family (`[3+/3 systems]`) —
backward-history vs forward-spec are complementary, not duplicative.

**Divergence within this family.** Two persistent divergences cluster
in the State/Memory/History area:

- **Memory architectural stance: openclaw treats memory as a
  singleton plugin slot (one mechanism active, replaceable, not
  layered); PAI treats memory as a top-level Principle 13.**
  Architectural conservatism vs first-class-primitive framings of
  the persistence question.
- **State-shape divergence: file-per-component (AutoGen, Voyager) vs
  typed-channel-map (LangGraph).** Both honor "no monolithic state
  blob" but with different update granularities — whole-component
  records vs per-channel reducers within one schema.

**Family-internal coherence.** Patterns sequence from substrate
decisions (component-local state) through kinds of state
(failure-record, append-only history, memory-as-architectural-concept)
to forward-spec state (plans-as-artifacts). The two divergences
within this family record how systems disagree on persistence-layer
primitives even where the convergent patterns hold.

### Family E: Quality & discipline (4 patterns)

Patterns about output quality — what gets enforced, what's documented
as anti-pattern, what's bounded as iteration count, and what's
cleaned up as recurring infrastructure work.

**Anti-patterns explicit as deliverable artifact.**
**`[3+/6 systems]`** openclaw VISION.md "What We Will Not Merge"
lists patterns to avoid; AutoGen v0.4 migration guide names
`ConversableAgent.register_reply` and old user-proxy tool-routing as
patterns to avoid, and `AssistantAgent` itself is documented as a
"kitchen sink" prototype; LangGraph names replay-as-cache and
interrupts-as-line-continuations as common misreadings. Cognition
Devin's "Don't Build Multi-Agents" post is the canonical example: a
published anti-pattern argument framed as a prohibition (with
named-target framework rejections — OpenAI Swarm, Microsoft AutoGen).
OpenAI harness-engineering names "one big AGENTS.md" as the only
explicitly named anti-pattern in the writeup, with four failure
mechanisms (context crowding, salience collapse, rot,
unverifiability). oh-my-codex maintains anti-patterns at multiple
layers: CONTRIBUTING.md `<Bad>` examples ("Claiming completion
without verification: 'should work correctly. Task complete.'"),
explicit deprecations (`$web-clone` "hard-deprecated"), and
`templates/AGENTS.md` opening with negative directives. Six systems
publish anti-patterns alongside recommended patterns; Cognition
Devin's framing is the strongest (entire post-as-anti-pattern-
argument).

**Mechanical enforcement of regression-tested behavioral
constraints.** **`[2-system strict + diversity hedge]`** OpenAI
harness-engineering uses custom linters with agent-readable error
messages and treats "golden principles" as mechanically-checked
(PR [#2783](https://github.com/EvaLok/schema-org-json-ld/pull/2783)
deliverable patterns 8/9/12); rule violations surface as actionable
diagnostics rather than soft documentation. oh-my-codex maintains
behavioral prompt-contract regression tests in
`src/hooks/__tests__/prompt-guidance-*.test.ts` (PR
[#2784](https://github.com/EvaLok/schema-org-json-ld/pull/2784)
deliverable pattern 7) — agent-affecting prose has CI coverage on
exact wording. Two-system strict convergence on
behavioral-constraint-as-tested-artifact. **Diversity hedge:**
Voyager's SkillManager + CurriculumAgent assert
`vectordb._collection.count() == len(self.skills)` at construction
(see [`systems/voyager.md`](1-research/systems/voyager.md)) — this
is mechanical enforcement but init-time-only and on data-state, not
continuous behavior; LangGraph enforces channel types and reducer
contracts statically via TypedDict / dataclass / Pydantic BaseModel
— mechanical enforcement on data shape, not behavior. Both share the
principle (mechanically-checked invariants over documented rules) at
different scope and rigidity; counted as loose-framing convergence
on the broader mechanical-enforcement principle, not as 4-system
strict.
*Cross-family: distinct from Family B's "Deterministic code
executes; LLM proposes" pattern. Code-vs-prompts is where execution
authority lives; mechanical enforcement is whether behavioral
promises are regression-tested. Both involve deterministic code
constraining LLM behavior, but in different directions —
authority-allocation vs verification-discipline.*

**Iteration ceilings with explicit numerical limits.**
**`[2-system strict + 1 adjacent partial]`** oh-my-codex documents
`max_iterations=10` for tool-loops and `max=5` for review-loops,
plus an autoresearch loop with explicit keep/discard/stop
per-iteration decision (PR
[#2784](https://github.com/EvaLok/schema-org-json-ld/pull/2784)
deliverable pattern 6). Voyager's
`action_agent_task_max_retries = 4` (see
[`systems/voyager.md`](1-research/systems/voyager.md)) bounds
retries on action failure with critic-critique +
execution-error fed into the next prompt; failed tasks accumulate in
`failed_tasks.json` rather than retrying indefinitely. Two-system
strict convergence on bounded-autonomy-loop as architectural
primitive. Cognition Devin's documented 45-minute session time limit
is adjacent (bounds total runtime rather than iteration count;
*documented-claim per cycle-26 source-access note*) — the bounding
axis differs but the principle (autonomous loops need explicit
ceilings, not open-ended runs) is shared.

**Entropy / AI slop as first-class engineering concern.**
**`[2-system clean]`** OpenAI harness-engineering names entropy as
first-class engineering concern and pairs it with golden principles
+ a doc-gardening agent (PR
[#2783](https://github.com/EvaLok/schema-org-json-ld/pull/2783)
deliverable patterns 11/12) — the harness acknowledges agent-output
quality drift as something requiring active mitigation
infrastructure, not a one-time cleanup. oh-my-codex requires a
deslop pass as mandatory post-completion step (PR
[#2784](https://github.com/EvaLok/schema-org-json-ld/pull/2784)
deliverable pattern 13); the workflow embeds quality cleanup into
each task's completion contract. Two-system convergence on
recurring-cleanup-as-infrastructure (vs. accretion as design
feature). Inversely-related to v1's accretion-as-defense pattern
(F12 in `0-retrospective.md`) — these systems treat accretion as a
failure mode to clean, not a defensive structure to preserve.

**Family-internal coherence.** All four patterns are about
maintaining quality of agent behavior and agent-affecting artifacts;
they differ on what is bounded — output content (anti-patterns
documented), behavioral promises (mechanical enforcement), iteration
count (ceilings prevent runaway autonomy), or output drift
(entropy/AI slop cleanup). Iteration ceilings was placed here rather
than alongside security defaults because the pattern's center of
gravity is "autonomous loops need explicit ceilings, not open-ended
runs" — retry-loop discipline, not trust-boundary.

### Family A: Agent architecture (3 patterns)

Patterns about how agents are decomposed into roles, what model
selections attach to each role, and what shapes a fixed team takes
when role-separation is adopted.

**Multi-agent decomposition is not a default.** **`[3+/6 systems]`**
*(Cognition Devin's named-rejection is the strongest evidence.)*
openclaw VISION.md "Agent-hierarchy frameworks ... as a default
architecture" appears in "What We Will Not Merge"; AutoGen v0.4
removed built-in sequential chat as "too opinionated and not
flexible enough"; LangGraph multi-agent docs state "not every
complex task requires this approach—a single agent ... can often
achieve similar results." PAI's Principle 14 ("Agent Personalities")
gestures toward multi-agent without prescribing decomposition.
Cognition Devin's "Don't Build Multi-Agents" post (Walden Yan, June
2025) is the strongest named-rejection in the surveyed systems — it
explicitly calls OpenAI Swarm and Microsoft AutoGen "the wrong way
of building agents" and argues context fragmentation makes
multi-agent designs fragile (Flappy Bird example: independent agents
make incompatible implicit decisions). *(Documented-claim per
cycle-26 source-access note — `cognition.ai` blocked, content via
secondary sources.)* oh-my-codex's `$team` runtime exists as opt-in
but README explicitly says "$team is not the default onboarding
path." Six systems with foregrounded support; none assert the
opposite as a default.

**Per-agent model selection as architectural primitive.**
**`[3+/3 systems + diversity hedge]`** AutoGen's Extensions API
documents "model clients" as a layer abstraction; each
AssistantAgent takes its own `model_client`, so per-agent model
choice is architecturally first-class. Voyager assigns `gpt-4` to
ActionAgent, CurriculumAgent (main), and CriticAgent (novel
reasoning) and `gpt-3.5-turbo` to CurriculumAgent QA-cache lookups
and SkillManager skill-description generation (cached/derivative
work) — explicit cost-vs-novelty framing in the research artifact.
oh-my-codex extends per-agent model selection across providers:
`src/config/models.ts` declares supported models GPT-5.4,
GPT-5.4-mini, GPT-5.5, GPT-5.3-codex; the "mini composition seam"
gates exact-model behavior; `$ask-claude` and `$ask-gemini` skills
shell to non-OpenAI provider CLIs from within a Codex session. Three
systems with asymmetric rationale (Voyager: cost-tiering; AutoGen:
architectural flexibility without rationale prescription;
oh-my-codex: cross-provider invocation as a first-class skill).
Convergence on per-agent-model-selection as architectural-primitive;
divergence on the rationale framing.

**Small fixed team with explicit role-separation.**
**`[3+/3 systems + contrary stance]`** Voyager's `voyager/agents/`
defines four agents with named roles (ActionAgent: code generation;
CurriculumAgent: task selection; CriticAgent: verification;
SkillManager: storage) — the four agents are the system architecture.
AutoGen documents the Magentic-One pattern (`MagenticOneGroupChat`)
as a lead-orchestrator + specialized workers team with Task Ledger /
Progress Ledger vocabulary for planning and tracking. oh-my-codex
ships 30 named role prompts in `prompts/*.md` (Metis as analyst,
Ralph as persistent executor, plus planner / architect / critic /
verifier / researcher / etc.); workflow stages (`$deep-interview` →
`$ralplan` → `$ralph` → `$team`) hand off across role-named agents.
Three systems support; **Cognition Devin contradicts this pattern
explicitly**: the "Don't Build Multi-Agents" stance and the
single-threaded linear agent default reject task-decomposition into
role-separated sub-agents. The contradiction is substantive — not
absent-of-evidence, but published-anti-stance. Structural
asymmetries within the supporting three: Voyager runs peer-flow
(curriculum → action → critic → skill); Magentic-One runs
lead-worker hierarchy (orchestrator dispatches to workers);
oh-my-codex's named-keyword workflow runs sequential mode
transitions across role-named agents (planner → architect → critic
within `ralplan`, then handoff to executor in `ralph`).
*Cross-family: see Family B's Multiple orchestration patterns
coexist bullet — the role-separation surfaced here implements across
distinct topologies (Voyager peer-flow / Magentic-One lead-worker /
oh-my-codex sequential mode), so the two patterns intersect on
substrate.*

**Divergence within this family.** **Agent-hierarchy stance is
downstream of operator-vs-goal-driven framing.** openclaw is
operator-driven (user issues commands; system executes them); PAI is
goal-driven (system pursues user's long-running goals). openclaw
rejects agent-hierarchies; PAI Principle 14 gestures toward them.
The hierarchy choice follows from the prior operator/goal choice.

**Family-internal coherence.** The three patterns sequence as
"whether to decompose at all (multi-agent decomposition's
negative-default) → how agents are differentiated when decomposition
happens (per-agent model selection) → how roles are assigned within
a fixed team (small fixed team)." Cognition's contradiction on
small-fixed-team is a substantive in-family disagreement preserved
in the contrary-stance badge and body prose.

### Family B: Orchestration & system shape (3 patterns)

Patterns about how the system is partitioned into deterministic vs
LLM components, how it extends, and how multiple orchestration
topologies coexist.

**Deterministic code executes; LLM proposes (code-vs-prompts
split).** **`[3+/6 systems]`** PAI states this explicitly as
Principles 5/6/11 ("Deterministic Infrastructure" / "Code Before
Prompts" / "Goal → Code → CLI → Prompts → Agents"). Voyager
separates `voyager/control_primitives/` (deterministic JS) from
`voyager/prompts/` (LLM-driven). LangGraph's `ToolNode` executes
tools deterministically while the LLM emits structured calls.
AutoGen follows the same shape: model emits a schema-validated call
(name + JSON arguments); host executes registered code. openclaw's
plugin system separates extension code from the agent layer that
invokes it (architectural-shape match; LLM-proposes / code-executes
is less foregrounded here than in the four agent-frameworks).
OpenAI's harness-engineering writeup foregrounds this as a thesis:
"Humans steer. Agents execute" via mechanical enforcement layers
(custom linters, CI checks, AGENTS.md as table-of-contents to deeper
docs/) — the harness layer is deterministic, the agent loop is
LLM-driven. oh-my-codex makes the same split explicit at the
implementation level: MCP servers, the 44KB keyword detector
(deterministic pattern matching, not semantic classification per
pattern 21 in `_notes/cycle-26-oh-my-codex-research.md`), and the
Rust sparkshell harness all sit deterministically alongside the
LLM-driven Codex tool loop. Six-system foregrounded convergence with
openclaw architectural-shape match, across substrate variations
(research code, agent and graph-state frameworks,
personal-assistant, local-first gateway, harness-as-environment,
configuration-layer-on-top-of-CLI), spanning Python, TypeScript, and
Rust.

**Small core, capability extends via plugins/skills/tools/layers.**
**`[3+/7 systems]`** openclaw "Core stays lean; optional capability
should usually ship as plugins"; PAI 16 named principles plus
plugin/skill architecture; AutoGen Core / AgentChat / Extensions /
Studio / Bench layering; Voyager control primitives + skill library
+ prompts as three named layers; LangGraph low-level Pregel +
higher-level prebuilt agents. OpenAI harness-engineering describes
the harness as depth-first accumulation (capabilities added
iteratively as failures surfaced; not pre-designed) — the small
entry-point pattern is AGENTS.md (~100 lines as table of contents)
extended by the structured `docs/` directory plus
mechanical-enforcement layer plus per-task ephemeral worktrees.
oh-my-codex is explicit: "OMX does NOT replace Codex" (README,
repeatedly stated) — it is a configuration layer + hook harness with
39 skills, 30 role prompts, three first-party MCP servers, and a
Rust sparkshell extension all sitting on top of an unmodified Codex
CLI. Seven-system convergence with shape variations (plugins,
skills, layers, extensions, harness-accumulation,
configuration-layer-with-hooks) on the same architectural principle.

**Multiple orchestration patterns coexist as first-class.**
**`[3+/3 systems]`** AutoGen documents round-robin, selector, swarm,
graph, and lead-orchestrator; LangGraph documents prompt chaining,
routing, parallelization, orchestrator-worker, ReAct, subgraphs,
supervisor. Both express orchestration via message-protocol behavior
contracts rather than universal orchestrator objects. oh-my-codex
extends this with named workflow modes (`deep-interview`, `ralplan`,
`ralph`, `team`, `autopilot`, `ultrawork`, `ultraqa`) governed by an
explicit transition allowlist in `docs/STATE_MODEL.md` — multiple
orchestration patterns coexist with deterministic transition policy
preventing illegal mode shifts. Three systems with substrate
diversity now broader than just agent frameworks (oh-my-codex is a
configuration layer over an external CLI, not an agent framework
itself).

**Family-internal coherence.** The three patterns sequence as
"foundational architectural partition (deterministic code vs LLM
proposals) → extensibility shape (small core extending via plugins)
→ coordination flexibility (multiple orchestration topologies
coexisting)."

### Family D: Trust posture & security defaults (1 pattern, thin family)

A single-pattern family covering how the system bounds its trust of
external inputs and execution environments. The family is
acknowledged as thin (pattern count is 1); trust-boundary
configuration is a distinct architectural concern not naturally
absorbed into Agent architecture (decomposition framing) or
Orchestration (coordination framing). The thin-family framing has
its own structural strain (small-family ceremonialness) but is more
honest than artificial absorption: the strain is named explicitly
rather than absorbed into a family that doesn't accommodate the
pattern.

**Strong-defaults security with operator-controlled knobs.**
**`[3+/3 systems + scope condition]`** openclaw: default DM policy
`pairing` (unknown senders blocked); sandbox modes with allow/deny
lists; "Treat inbound DMs as untrusted." AutoGen: Docker code
executor as the safer default vs local; "Only connect to trusted MCP
servers" warning; Magentic-One docs name
prompt-injection-from-web-content as a concrete risk. PAI: explicit
strong-defaults posture in README. Three-system convergence;
LangGraph's durable-execution warnings are operational rather than
threat-model framing; Voyager's research-artifact status makes the
question less applicable. *Scope condition (single-system, OpenAI
Harness): high-throughput regimes condition when the security-stance
pattern applies — see `_notes/cycle-22-cross-system-synthesis.md`
for the throughput-vs-security trade-off observation.*

### Divergences index

The three persistent divergences across systems studied, with
cross-references to the family-local callouts where the substantive
treatment lives:

- **Agent-hierarchy stance** (operator-driven vs goal-driven;
  openclaw vs PAI) — see Family A's "Divergence within this family"
  callout above.
- **Memory architectural stance** (singleton plugin slot vs
  top-level architectural principle; openclaw vs PAI) — see Family
  C's "Divergence within this family" callout above.
- **State-shape divergence** (file-per-component vs typed-channel-map;
  AutoGen/Voyager vs LangGraph) — see Family C's "Divergence within
  this family" callout above.

The Phase-2-input section remains pending. These observations are
substrate, not prescription — Phase 2 candidates can draw from
3+-system convergence as positive evidence and from divergences as
design-space-spanning alternatives. Single-system observations
(captured in `_notes/cycle-22-cross-system-synthesis.md`) should not
yet shape candidate generation.

## Phase 1 work plan (subject to evolution)

### Required reads remaining

Both required reads have had a first-pass review (README + VISION.md
where available). Deeper reads queued:

- openclaw: directory survey, architecture pages, key source files
  (gateway core, session management, plugin loading, memory slot)
- PAI: `Tools/`, `Packs/`, `.claude/`, `Releases/v4.0.3/`, the
  scientific-method loop in code

### Further systems to study

Drawn from the redesign prompt's candidate list plus my own
identifications. Order not yet committed.

| System | Why relevant | Mechanism | Status |
|---|---|---|---|
| AutoGen | Microsoft's multi-agent framework; explicit conversation patterns between agents (relevant to my orchestrator + audit + Copilot setup) | Copilot research-only dispatch | Cycle 15 dispatched (PR #2763); cycle 16 integrated; per-system file [`systems/autogen.md`](1-research/systems/autogen.md) |
| LangGraph | Production state-management for agents; explicit graph-based state | Copilot research-only dispatch or orchestrator-direct | Cycle 18 dispatched (issue [#2767](https://github.com/EvaLok/schema-org-json-ld/issues/2767), gpt-5.5, canonical cycle-15 procedure with anti-smuggling discipline pre-loaded); per-system file [`systems/langgraph.md`](1-research/systems/langgraph.md) |
| Voyager | Long-running self-improving Minecraft agent; skill library accumulation | Orchestrator-direct (the paper is short) | Cycle 17 read; per-system file [`systems/voyager.md`](1-research/systems/voyager.md) |
| Cognition Devin writeups | Autonomous coding agent; production deployment patterns | Orchestrator-direct (blog posts, not a repo) | Deep-dive landed (cycle 41, [PR #2804](https://github.com/EvaLok/schema-org-json-ld/pull/2804); supersedes closed cycle-26 dispatch [#2779](https://github.com/EvaLok/schema-org-json-ld/issues/2779)) |
| OpenAI harness-engineering | Internal harness writeup; mechanical-enforcement and entropy-as-engineering-concern patterns | Copilot research-only dispatch | Deep-dive landed (cycle 41, [PR #2805](https://github.com/EvaLok/schema-org-json-ld/pull/2805); supersedes closed cycle-26 dispatch [#2781](https://github.com/EvaLok/schema-org-json-ld/issues/2781)) |
| oh-my-codex | Configuration layer + hook harness over Codex CLI; densest cross-system citation footprint | Copilot research-only dispatch | Stub from cycle-26 dispatch; cycle-63 deeper read in flight [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) (supersedes closed cycle-26 dispatch [#2782](https://github.com/EvaLok/schema-org-json-ld/issues/2782)) |
| Semantic Kernel | Microsoft's agent SDK; planner/skills split | Copilot research-only dispatch (lower priority) | Pending |
| Anthropic engineering posts | Claude Code, agent SDK, internal tooling experience | Orchestrator-direct | Pending |
| openai/symphony | Per Eva directive [#2775](https://github.com/EvaLok/schema-org-json-ld/issues/2775) | TBD | Pending |
| oh-my-claudecode | Per Eva directive [#2774](https://github.com/EvaLok/schema-org-json-ld/issues/2774) | TBD | Pending |

### Cycle plan (provisional)

Cycle 14 (2026-04-28): openclaw + PAI first-pass; this document
created; no dispatch.

Cycle 15 (2026-04-28): AutoGen Copilot research-only dispatch
executed (issue #2762, PR #2763, gpt-5.5, canonical cycle-6
procedure). Adversarial re-read of this document found smuggling in
per-system "Provisional patterns to track" sections; renamed to
"Patterns observed in [system]" with v2-relevance framings stripped.

Cycle 16 (2026-04-28): AutoGen system entry added to this document
(navigation summary; PR #2763 is evidence base). Voyager paper
read deferred to cycle 17+.

Cycle 17 (2026-04-28): Voyager orchestrator-direct read added
(abstract + code: voyager.py, agents/skill.py, agents/critic.py,
agents/curriculum.py first 150 lines, prompts/ listing). Cold-readers
on AutoGen navigation summary (PASS with one optional flag — tools
folded into Trust boundaries; deep-dive treats as 1 of 7 sections) and
on AutoGen Patterns observed selection (PASS — no v2-relevance smuggling
detected; cycle-16 count claims '16 / 38' actual is '15 / 43', minor
self-reporting discrepancy noted). LangGraph is the next dispatch
candidate (state-management focus, Copilot research-only).

Cycle 18 (2026-04-29): Cold-readers on Voyager Patterns observed list
(PASS with two minor flags — bullets 15/16 contain post-prose specs;
three prose observations not elevated) and on Voyager anchoring caveats
(PASS with one substantive finding — caveats are one-directional,
several over-discount transferable patterns; preamble paragraph added
to BOTH AutoGen and Voyager anchoring-caveats sections naming the
asymmetry). Optional cycle-17 flags 5/6 applied: AutoGen Tool
integration model paragraph added (~10 lines); AutoGen nav-bullet-4
enriched with behavior-contracts-as-message-protocols framing (~1
sentence). LangGraph dispatched (issue [#2767](https://github.com/EvaLok/schema-org-json-ld/issues/2767),
gpt-5.5, canonical cycle-15 procedure). Tier-2 group 3 explicitly
scoped for cycle 19+ execution (sixth-defer-without-scoping was the
failure mode declined this cycle).

Cycle 33 (2026-05-01): Research file restructure. Per-system
writeups extracted from monolithic `1-research.md` into
`1-research/systems/*.md` (one file per system). Index file (this
file) reordered so cross-system observations sit near the top rather
than buried below ~750 lines of per-system prose. Three previously-
inline-only systems (Cognition Devin, OpenAI harness-engineering,
oh-my-codex) given dedicated stub files marking the asymmetry with
the deeper-read systems explicitly. Migration recorded in
`_notes/cycle-33-research-restructure.md` per the redesign-prompt's
`evolve-the-mechanism` mandate.

Cycle 34+: dispatch options, in approximate priority order
(adjustable by cycle's actual capacity):
1. Deeper read on the three stub-marked systems (Cognition Devin,
   OpenAI harness, oh-my-codex) per the still-open
   issues #2779 / #2781 / #2782.
2. Semantic Kernel (lower priority; Copilot research-only or
   orchestrator-direct).
3. Anthropic engineering posts (orchestrator-direct).
4. Eva directives #2774 (oh-my-claudecode) and #2775 (openai/symphony).
5. Deeper second-pass orchestrator-direct on openclaw and PAI (cycle 16
   noted that the deliverable-size asymmetry biases cross-system
   synthesis toward the system with the richest evidence base; bringing
   openclaw and PAI to closer parity with AutoGen's deep-dive depth is
   an alternative use of cycles before committing to cross-system
   synthesis claims).

The dispatch sequence is tentative. Phase 1 reading priority should
adjust based on cycle capacity and any patterns that emerge as
load-bearing in cross-system observations.

### What Phase 1 will produce

A reference document (this file plus per-system files under
`1-research/systems/`) capturing:
- Each system studied (architecture summary, anchoring caveats) — in
  per-system files
- Cross-system patterns (with anchoring discipline) — in this file
- Tensions surfaced (alignment that may be confirmation bias;
  patterns that may not generalize to autonomous-public-repo
  context)
- A Phase-2-input section listing the patterns that survive
  multi-system reading and have load-bearing relevance to v2
  candidate generation

The Phase-2-input section is **not** to be drafted until at least
3-4 systems have been read. Premature commitment to first-found
patterns is the failure mode to avoid.

## Persistence-mechanism note

This file is the Phase 1 working surface, mirroring the role
`docs/redesign/0-retrospective.md` plays for Phase 0. The
`_notes/cycle-N-*.md` per-cycle convention from Phase 0 carries over:
each cycle's Phase 1 work gets a `_notes/cycle-N-*.md` file, and the
README iteration log (when cycle 15+ updates that section) tracks
Phase 1 cycle progression alongside Phase 0.

Cycle-N-pre-commits-cycle-N+1-checks chain (thirteen cycles deep as of
cycle 18) extends to Phase 1: each cycle's Phase-1 notes file
pre-commits adversarial-on-adversarial checks for the next cycle, same
discipline as Phase 0 has used since cycle 7.

**Cycle 33 mechanism evolution: split into index + per-system files.**
Per the redesign-prompt's `<evolve-the-mechanism>` mandate, this file
was restructured at cycle 33 (2026-05-01). Prior shape: one monolithic
file (~1422 lines / 78KB) holding status, anchoring discipline, all
per-system architecture summaries, cross-system observations, and work
plan. New shape: this file as the index (status + anchoring +
cross-system observations + work plan + persistence note); per-system
architecture summaries under [`1-research/systems/`](1-research/systems/).
Rationale: per-system writeups are independent and rarely cross-cite;
cross-system observations is the load-bearing readable Phase 2 will
work against and should sit near the top, not buried 800+ lines down;
the file was on a growth trajectory that would have made it 2500+ lines
once pending Phase 1 reads land. Migration is recorded in
[`_notes/cycle-33-research-restructure.md`](_notes/cycle-33-research-restructure.md).
