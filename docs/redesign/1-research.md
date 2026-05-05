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

## Implications-mining clusters (cycles 62-69)

The Family-level observations above record cross-system patterns
from the first-pass per-system reads (cycles 14-32). A deeper-pass
over the same six deep-dive systems ran across cycles 62-69 under
the polarity inversion of [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829),
mining ~10 per-system implications per cycle and clustering them by
architectural concern. The mining produced 45 implications across
9 architectural clusters across 6 systems. The per-cycle docs live
under [`_notes/cycle-62-autogen-implications.md`](_notes/cycle-62-autogen-implications.md)
through [`_notes/cycle-69-voyager-implications.md`](_notes/cycle-69-voyager-implications.md);
a first synthesis at
[`_notes/cycle-65-cross-implications-synthesis.md`](_notes/cycle-65-cross-implications-synthesis.md)
covered the AutoGen + LangGraph layer. This section is the elevation
to the index that cycles 65/66/67/68/69 deferred to a future
synthesis cycle (now cycle 70).

The clusters are a deeper layer than the Families: Families are
named pattern themes; clusters are named *architectural concerns*
each containing multiple sub-shapes (different mechanisms within the
same concern). The relationship between clusters and Families is
not 1:1 — many clusters intersect Family patterns; some clusters
(notably H, I) surface concerns that the Family layer didn't
foreground. The clusters add (a) sub-shape granularity the Families
don't surface, (b) the per-system implications evidence base used to
evaluate Phase 2 candidate variations, and (c) sub-axis catalogues
for stratification patterns Phase 2 candidates can compose against.

The six systems mined: AutoGen
([cycle 62](_notes/cycle-62-autogen-implications.md))
+ LangGraph ([cycle 64](_notes/cycle-64-langgraph-implications.md))
+ Cognition Devin ([cycle 66](_notes/cycle-66-cognition-devin-implications.md))
+ openclaw ([cycle 67](_notes/cycle-67-openclaw-implications.md))
+ OpenAI harness-engineering ([cycle 68](_notes/cycle-68-openai-harness-implications.md))
+ Voyager ([cycle 69](_notes/cycle-69-voyager-implications.md)).
Substrate diversity at maximum across five orthogonal axes:
library/product/research-artifact, cloud/local, enterprise/personal/research,
software-development/game-environment, external-publishable/internal-tooling.
Patterns surviving 6-system cross-substrate convergence are robustly
transferable.

### Cluster table (post cycle 69)

| Cluster | Theme | Depth | Implications | Sub-shapes/sub-axes |
|---|---|---|---|---|
| A | Cycle-internal boundaries with state-write semantics | 6-system clean | 11 | 9 sub-shapes |
| B | Cross-cycle artifact organization | 6-system clean | 10 | 9 sub-shapes |
| D | Documentation honesty | 5-system clean (+ Voyager partial) | 11 | 9 sub-shapes |
| F | Tool-suite stratification (multi-axis) | 5-system convergent | 8 | 8 sub-axes |
| H | Post-session feedback / cross-session learning | 4-system convergent | 4 | 4 sub-shapes |
| C | Lifecycle operations beyond resume | 4-system clean | 6 | 5 sub-shapes |
| E | Typed boundary semantics | 3-system convergent | 3 | 2 sub-shapes |
| G | Role-asymmetric context | 2-system convergent | 2 | 2 sub-shapes |
| I | Harness-enforced security/policy boundaries | 2-system convergent (substrate-correlated) | 3 | 2 sub-shapes |

Clusters A, B, D are tied as the most-foregrounded clusters in the
corpus (5+ system clean depth). Cluster F is the multi-axis-king
(8 sub-axes — largest single-cycle sub-shape growth occurred when
Voyager was mined cycle 69, +4 sub-axes). Cluster I is at lower
2-system convergent depth but is *substrate-correlated* to v1's
own substrate (cloud-anchored multi-actor with audit) — Phase 2
candidates SHOULD weight cluster I patterns highly despite the low
depth count.

### Cluster A: cycle-internal boundaries with state-write semantics

`[6-system clean]` AutoGen + LangGraph + Cognition + openclaw +
OpenAI harness + Voyager.

The orchestrator's cycle has internal boundaries across which
state-writes must respect explicit semantics. The principle —
cycle-internal phasing has explicit semantics rather than implicit
sequencing — is shared across all six systems; the mechanisms
diverge widely. Sub-shapes (9) include super-step semantics with
per-channel reducer rules (LangGraph I-L1), phase-boundary state
semantics (AutoGen I-3 + LangGraph I-L1 cross-system convergent,
elevated cycle 65), termination predicates as cycle-internal phase
delimiters (AutoGen I-3), per-key reducers with explicit merge
rules (LangGraph I-L2), lane-aware FIFO queue with per-lane
concurrency caps and per-session serialization (openclaw I-O3),
stuck-session watchdog as recovery-without-abort lifecycle
operation (openclaw I-O5), sync invariants asserted at session
init for dual-storage components with fail-fast remediation hint
(Voyager I-V4), bounded retries with critic-feedback fed forward
into next-attempt prompt (Voyager I-V7), and (cycle 68) process-
isolation discipline at session level via ephemeral worktrees
(OpenAI harness).

**Phase 2 implication**: cluster A is near-mandatory. v1's failure
modes (stale-reference accumulation, abandonment cascade,
chronic-category currency loop) all map to gaps in cycle-internal
phasing. Sub-shape variety (9) gives Phase 2 candidates significant
combinatoric room for differentiation.

### Cluster B: cross-cycle artifact organization

`[6-system clean]` AutoGen + LangGraph + Cognition + openclaw +
OpenAI harness + Voyager. Newly upgraded to 6-system clean cycle 69
via Voyager I-V3/I-V5/I-V6/I-V8.

State organized by component-of-origin, multi-surface, with
forward-spec + backward-history + active-retrieval-surface
distinction. The principle — state organized by component-of-origin
rather than central state file, multi-surface with separated
semantics — is shared across all six systems; the substrate
diverges across in-process dictionaries, file-per-component, typed
channels, repository-as-state, wiki servers, and skill-library +
vectordb hybrids. Sub-shapes (9) include component-local
persistence with per-component resume opt-in (Voyager I-V3 +
AutoGen I-6 cross-system parallel), short-term/long-term split via
typed-channel-map (LangGraph I-L5), multi-mechanism-per-coordinate
memory architecture with 7 distinct mechanisms (Cognition I-C3),
tripartite memory by content × temporal scope of durable / daily /
sweep-summary (openclaw I-O4), repository-as-state with ephemeral-
worktree task isolation (OpenAI harness I-OH4 / I-OH7), active-
surface-vs-monotonic-history storage discipline where active
retrieval surface is single-version while disk is monotonic-append
(Voyager I-V5), top-k semantic-retrieval over LLM-generated
descriptions rather than raw artifacts (Voyager I-V6), failure-as-
first-class-artifact (Voyager I-V8 + openclaw + OpenAI harness
sub-failure decomposition), and plans-as-forward-versioned-artifacts
(OpenAI harness; oh-my-codex stub-mention pending #2833 deeper
read).

**Phase 2 implication**: cluster B is the strongest case for
storage-architecture as a v2 candidate generator — Phase 2
candidates differ substantially based on which sub-shape
combination they adopt (parsimonious 3-sub-shape candidate vs
all-9-sub-shape rich candidate; repository-anchored vs file-per-
component vs hybrid).

### Cluster D: documentation honesty

`[5-system clean + Voyager partial]` AutoGen + LangGraph + Cognition
+ openclaw + OpenAI harness primary; Voyager partial via I-V8
failure-as-recorded-artifact contribution.

Published artifacts honestly disclose what was tried and what
didn't work, what was decided and why, what the position currently
is — including walkbacks of prior positions. Documentation is
treated as first-class engineering artifact, not optional cruft.
Sub-shapes (9) include anti-patterns as published artifact
alongside recommended patterns (six-system Family-pattern at
[3+/6 systems] depth — see Family E above for the elevation),
walkback as first-class artifact preserving prior thesis alongside
revised one (Cognition I-C1 — June 2025 single-threaded thesis
preserved alongside April 2026 walkback), invariants vs derivations
as separate published layers (Cognition I-C2 — durable invariants
vs revisable derivations stratified), migration guide as first-class
artifact with explicit deprecation semantics (AutoGen I-1),
AGENTS.md as table-of-contents to deeper docs/ rather than
single monolithic prompt (OpenAI harness I-OH1), structured
failure-mechanism decomposition for documented anti-patterns
(OpenAI harness I-OH3 — four named failure mechanisms for "one
big AGENTS.md"), failure-as-first-class-recorded-artifact
(Voyager I-V8), under-delegation as documented failure mode peer
to over-delegation (Cognition I-C9), and anti-pattern catalog with
explicit non-permanence framing — versioned and revisable rather
than absolute (openclaw I-O2 "roadmap-guardrail-not-law-of-physics"
qualifier; structurally distinct from Cognition's June-2025-vs-
April-2026 walkback because openclaw's catalog is upfront with
non-permanence built in).

**Phase 2 implication**: v1 already does this work via the
retrospective cadence and the journal. Cluster D additions for v2
are walkback as first-class protocol, invariants/derivations
stratification within the v2 prompt, anti-pattern catalog
("What v2 Will Not Do" alongside "What v2 Will Do"), and
structured failure records.

### Cluster F: tool-suite stratification (multi-axis)

`[5-system convergent]` openclaw + OpenAI harness + Cognition +
AutoGen + Voyager. Cycle 69 added Voyager as the 5th system with
the largest single-cycle sub-axis growth in the corpus (4 sub-axes
→ 8 sub-axes).

Tools, capabilities, models, terms, roles, and modes are stratified
along multiple parallel discrimination axes. The meta-architectural-
pattern: many distinct stratification axes coexist within a single
architectural domain. The 8 sub-axes:

1. **Version stratification** — active-version vs prior versions
   (openclaw plugin versioning; Voyager skill versioning V2/V3 on
   disk while vectordb keeps active version)
2. **Task-class stratification** — Playbook templates per task-class
   with outcome + steps + postconditions + advice + forbidden
   actions (Cognition I-C6)
3. **Capability-tier stratification** — Tier 1 read-only / Tier 2
   send-on-behalf / Tier 3 autonomous-with-standing-orders
   (openclaw I-O6)
4. **Terminology stratification** — explicit different meanings per
   term (openclaw tool / skill / plugin distinction, I-O8 — tools=
   function calls, skills=Markdown-injected, plugins=npm packages)
5. **Role stratification** — distinct named conceptual roles per
   responsibility (Voyager 4-agent architecture I-V1 — ActionAgent
   / CurriculumAgent / CriticAgent / SkillManager)
6. **Cost-tier stratification** — model-per-task-class within an
   agent ecosystem (Voyager I-V2 — gpt-4 for novel reasoning,
   gpt-3.5-turbo for cached/derivative work; v2 candidate extends
   within Anthropic family Opus / Sonnet / Haiku per task-class)
7. **Autonomy-mode stratification** — auto vs manual per-component
   with explicit human-in-the-loop method paths (Voyager I-V9;
   openclaw operator-tier-level)
8. **Capability-layer stratification** — primitives vs LLM-composed
   compositions (Voyager I-V10 — control_primitives + skill-library
   compose primitives + earlier skills; openclaw tool/skill
   distinction is parallel)

**Phase 2 implication**: stratification axes are mostly independent
and combinable — Phase 2 candidates can adopt any subset.
Candidate-shape implication: a candidate adopting all 8 sub-axes
will have a substantially more layered architecture than a
candidate adopting 2-3. The unified-vs-decomposed question (one
cluster F vs four narrower clusters — see "Open structural
questions" below) is itself a Phase 2 candidate-shape discriminator.

### Cluster H: post-session feedback / cross-session learning

`[4-system convergent]` Cognition + openclaw + OpenAI harness +
Voyager. Cycle 69 confirmed cluster H upgrade from 3-system to
4-system via H1 hypothesis (capability-accumulation as 4th
sub-shape).

Artifacts produced at the end of a cycle / session serve as input
for the next cycle / session — the system improves across cycles
through structured feedback-loop artifacts. Four distinct sub-shapes
(rather than one canonical mechanism):

- **Tight-cycle meta-feedback** — Cognition Session Insights
  (I-C7): post-session reflective capsule for next session,
  machine-readable manifest plus human-readable prose; next
  cold-start reads first
- **Score-gated consolidation** — openclaw dreaming (I-O9):
  cron-driven sweep promotes short-term entries (`memory/.dreams/`)
  to long-term (`MEMORY.md`) gated on recall-frequency and
  query-diversity thresholds
- **Continuous-background gardening** — OpenAI harness doc-gardening
  + quality-grading (I-OH6): ongoing background process maintaining
  artifacts vs episodic sweep
- **Capability-accumulation** — Voyager skill-library
  (I-V5/I-V6/I-V10): successful cycles produce reusable
  *capabilities* (skills) for future cycles, not just meta-lessons —
  next cycle inherits *what to do* not just *what to think about*

**Phase 2 implication**: Phase 2 candidates have a *spectrum of
mechanism choices* rather than one canonical shape. The four
sub-shapes are not mutually exclusive and can compose. The minimal
v2 commitment is at least one cluster H mechanism (else no
cross-session learning); the maximal commitment is all four
(substantial implementation effort, rich learning surface).

### Other clusters (C, E, G, I) — brief

**Cluster C — lifecycle operations beyond resume `[4-system clean]`.**
AutoGen + LangGraph + Cognition + openclaw. Lifecycle vocabulary
extends beyond `resume`: terminate, reset, fork, replay, event-
trigger / reactive-bot-comment-pickup, stuck-watchdog with stale-
lane release. Sub-shapes (5). v1's only lifecycle operation is
`resume`; cluster C surfaces 4-7 additional operations Phase 2
candidates may adopt selectively.

**Cluster E — typed boundary semantics `[3-system convergent]`.**
LangGraph + AutoGen + openclaw via TypeBox. Typed contracts on
data-shape at boundaries. Two sub-shapes: schema-discipline at
process-boundaries (TypeBox single-source-of-truth produces
validators in TypeScript / Swift / JSON-Schema) vs typed-channel-
merger-rules at within-process boundaries (LangGraph per-key
reducers). Voyager doesn't add cleanly: structured CriticAgent
output `{success: bool, critique: str}` is parallel to cluster E
patterns but isn't foregrounded as architectural axiom — it's a
JSON contract between two agents, not boundary discipline.

**Cluster G — role-asymmetric context `[2-system convergent]`.**
Cognition + openclaw. Two sub-shapes: (1) clean-context-reviewer
(Cognition Devin Review, I-C4) — different roles get different
trust/context semantics, reviewer role inverts share-full-traces
default; (2) untrusted-prefix sub-agent injection (openclaw
active-memory I-O7) — sub-agent output enters the main context as
untrusted prefix, cannot instruct main agent. Voyager's CriticAgent
↔ ActionAgent has role asymmetry but at per-action retry granularity
(landed in cluster A as bounded-retry-with-feedback) rather than
per-session role-context-shape (cluster G). The boundary between
cluster A bounded-retry and cluster G role-asymmetric is thin in
some cases; see "Open structural questions" below.

**Cluster I — harness-enforced security/policy boundaries
`[2-system convergent, substrate-correlated]`.** openclaw + OpenAI
harness. Two sub-shapes: permission-policy enforcement at the
harness level decoupled from prompt-level rules (openclaw I-O1 —
default-deny on multiple capability surfaces, before_tool_call.
block-true terminal enforcement, plugin discovery/promotion gated
by ClawHub security review) + quality-policy enforcement via
mechanical linters with agent-readable error messages (OpenAI
harness — golden principles mechanically checked). Substrate
observation: cluster I is *absent in research-artifact substrates*
(Voyager runs locally with full environment access; no need for
harness-enforced policy). Cluster I is correlated with
cloud-anchored multi-actor environments. v1's substrate
(GitHub-Actions-anchored multi-actor with audit) places it CLOSE
to the cluster I correlation; Phase 2 candidates SHOULD weight
cluster I patterns highly even at 2-system convergence depth,
because the substrate alignment is strong.

### Phase 2 design-input from clusters

Phase 2 candidates compose against the cluster sub-shapes as
options. The clusters establish *architectural concerns* that need
addressing; the sub-shapes are *mechanism options* within each
concern. This catalogue surfaces the load-bearing combinations.

**v1-failure-mode mapping (inverted-from-retrospective lens).**
Specific cluster sub-shapes map to named v1 failure modes:

- **Stale-reference accumulation** (v1's `1-research.md` summary
  table drift across cycles 30-50; the cleanup work in cycles 60-61):
  cluster A sync-invariants-at-init (Voyager I-V4 — `state-sync-check`
  Rust tool runs at session start, validates cross-storage consistency,
  exits with structured remediation text on divergence)
- **Dispatch-fails-orchestrator-decides-per-cycle gap** (v1 has no
  formal retry mechanism for dispatch failures): cluster A bounded-
  retry-with-feedback (Voyager I-V7 — `dispatch-with-retry` Rust
  tool wrapping `gh issue create` with prior-attempt-context +
  failure-diagnostic + max-retries semantic)
- **Abandonment cascade** (v1 loses track of failed cycles): cluster
  A lane-aware FIFO queue with stuck-watchdog (openclaw I-O3 +
  I-O5)
- **Chronic-category currency loop** (v1 self-management consuming
  cycles): cluster F autonomy-mode stratification + cluster H
  capability-accumulation (skills accumulate across cycles, reducing
  the per-cycle decision surface)
- **Forgotten-failure** (v1 abandoned dispatches and dropped
  patterns disappear into journal entries): cluster B failure-as-
  first-class-artifact + cluster D failure-as-recorded-artifact
  (Voyager I-V8 + structured `state/failures/` consulted at next-
  cycle-composition decision)
- **Implicit cycle-phasing** (v1 cycle structure is procedural-
  prompt rather than typed-state): cluster A super-step semantics +
  termination predicates + phase-boundary state semantics
- **No-recovery-without-abort** (v1 cycle has terminate but not
  release-stuck-lane): cluster A stuck-session-watchdog (openclaw
  I-O5 — `diagnostics.stuckSessionWarnMs` detects stale lanes and
  releases them; NEW lifecycle operation distinct from
  terminate/reset/resume/fork/event-trigger)

This mapping is *direct evidence* that the clusters address v1's
named failure modes, not generic architectural improvements.

**Storage-architecture sub-shape combinations (cluster B, 9
sub-shapes).** Concrete Phase 2 candidate-shape combinations:
- *Parsimonious* (3 sub-shapes): component-local + active-surface-
  vs-monotonic-history + failure-as-artifact. The minimum that
  addresses v1's stale-reference and forgotten-failure failure modes.
- *Maximal* (all 9 sub-shapes): rich storage architecture but high
  implementation complexity; substantial Rust tool surface.
- *Minimal-departure-from-v1* (2 sub-shapes): repository-as-state +
  plans-as-forward-versioned-artifacts. v1 already uses repo-as-state
  implicitly; this combination formalizes the existing pattern with
  minimal architectural churn.

**Cycle-internal phasing sub-shape combinations (cluster A, 9
sub-shapes).** Strong Phase 2 candidate-discriminators:
- *Boot-discipline focus* (3 sub-shapes): sync-invariants-at-init +
  bounded-retry-with-feedback + stuck-session-watchdog. Addresses
  v1's three most-foregrounded failure modes (stale-reference,
  dispatch-recovery, abandonment) with minimum complexity.
- *Typed-state focus* (3 sub-shapes): super-step semantics +
  termination predicates + per-key reducers. v2 with typed cycle-
  internal-state, larger architectural shift.
- *Lane-discipline focus* (2 sub-shapes): lane-aware FIFO queue +
  stuck-session-watchdog. v2 with explicit cycle-internal work-
  type lanes (substantive vs absorption vs housekeeping) and
  per-lane discipline.

**Stratification axes (cluster F, 8 sub-axes).** Phase 2 candidates
can adopt any subset. The 8 axes are mostly independent and
combinable. Candidate-shape implication:
- *Heavy stratification* (all 8 axes): substantially layered
  architecture, larger implementation surface, but strong
  separation-of-concerns properties
- *Light stratification* (2-3 axes — typically role + cost-tier +
  capability-layer): smaller implementation, retains the most
  load-bearing axes for v1's substrate
- The unified-vs-decomposed question (one cluster F vs four
  narrower clusters) is itself a candidate-shape discriminator —
  see "Open structural questions"

**Post-session feedback mechanisms (cluster H, 4 sub-shapes).**
Phase 2 candidates can adopt any combination; the four sub-shapes
are not mutually exclusive. The minimal v2 commitment is at least
one cluster H mechanism (else no cross-session learning); the
maximal commitment is all four (substantial implementation effort
but rich learning surface). Candidate-shape implication:
- *Tight-feedback-only*: Cognition I-C7-style Session Insights
  capsule, smallest implementation, captures meta-lessons
- *Capability-only*: Voyager I-V5/I-V6/I-V10-style skill-library,
  captures *capabilities* not lessons — different surface
- *Tight + Capability*: hybrid — meta-lessons for "what worked"
  and capabilities for "what we can now do." Strong candidate
  default
- *All four*: rich cross-cycle learning with episodic-sweep,
  continuous-gardening, capability-accumulation, and tight-cycle
  meta-feedback. Substantial Rust tool surface

**Documentation honesty sub-shape combinations (cluster D, 9
sub-shapes).** v1 already does this work via retrospective +
journal. Cluster D additions for v2:
- **Walkback as first-class artifact protocol** (Cognition I-C1) —
  explicit protocol for revising prior positions: `POSITIONS.md`
  artifact with version semantics
- **Invariants vs derivations stratified within v2 prompt**
  (Cognition I-C2) — INVARIANTS section vs DERIVATIONS section so
  revising derivations doesn't destabilize axioms
- **Anti-pattern catalog with explicit non-permanence framing**
  (openclaw I-O2) — `ANTI-PATTERNS.md` alongside `POSITIONS.md`
  and `MIGRATIONS.md` (the documentation-discipline triad)
- **Failure-as-first-class-recorded-artifact** (Voyager I-V8) —
  structured failure records consulted at next-cycle-composition
  decision

**Cluster I substrate-correlation observation.** v1's substrate
(GitHub-Actions-anchored multi-actor with audit) is *close* to the
cloud-anchored multi-actor substrate that correlates with cluster I.
Phase 2 candidates SHOULD weight cluster I patterns highly even at
2-system convergence depth. Specific mechanisms:
- **Harness-enforced tool-call policy decoupled from prompt-level
  rules** (openclaw I-O1) — Rust tools enforce what the LLM can
  invoke, not just prompt-level "don't do X" rules
- **Quality-policy enforcement via mechanical linters** (OpenAI
  harness) — agent-readable error messages on quality violations,
  CI-enforced

### Cross-cluster intersections (cycle 72 synthesis)

The within-cluster catalogues above (clusters A-I, ~9 sub-shapes
per foregrounded cluster) tell Phase 2 candidate authors *what
mechanisms exist for each architectural concern*. Cross-cluster
intersections tell Phase 2 candidate authors *how mechanisms
compose across architectural concerns to produce emergent
properties* — and where v1's failure modes are not just "missing
a sub-shape" but "missing the discipline of two clusters'
mechanisms composing at their boundary."

Cycle 70's hand-off named three priority intersections for cycle
72 synthesis: cluster A↔B (storage-discipline at cycle-boundary
moments), cluster F↔H (stratification of feedback mechanisms),
cluster D↔I (documentation-as-policy-enforcement). Each is examined
below with sub-patterns from the corpus and Phase 2 implications.
Four additional intersections (A↔C, B↔C, F↔I, E↔I) are flagged
briefly for future synthesis cycles.

#### A↔B: storage-discipline at cycle-boundary moments

Cluster A defines *when* in the cycle phase-boundary moments
occur; cluster B defines *what* gets persisted and *where*. Their
intersection is the discipline of cluster B storage writes
happening at named cluster A boundaries — which is what produces
consistent state across cycles.

Five sub-patterns from the corpus:

1. **Sync invariants asserted at session-init** (Voyager I-V4 +
   cluster B dual-storage discipline). Cluster A boundary (cycle
   init) + cluster B invariant (vectordb count vs JSON manifest
   count). The intersection IS a cluster A operation that enforces
   a cluster B consistency invariant. Without the intersection
   discipline, dual-storage drift accumulates silently across
   cycles.

2. **State-commit at end-of-super-step** (LangGraph I-L1 + I-L2).
   Cluster A super-step boundary + cluster B per-key reducer
   rules. Each super-step ends with a coordinated state-commit
   through reducers; cluster B reducer rules ARE cluster A
   boundary semantics. Without explicit super-step boundaries,
   reducer rules fire at fuzzy points and produce write-write
   conflicts.

3. **Failure-record-write at retry-exhaustion** (Voyager I-V7 +
   I-V8). Cluster A bounded-retry-with-feedback (retry exhaustion
   is a defined phase boundary) + cluster B failure-as-first-
   class-artifact (the write target). Retry exhaustion triggers a
   structured cluster B failure record. Without the intersection,
   exhausted retries produce ad-hoc journal mentions that get
   forgotten by the next cycle's composition decision.

4. **Watchdog-release-with-state-cleanup** (openclaw I-O5 +
   cluster B). Cluster A stuck-watchdog (lifecycle operation) +
   cluster B lane-state-and-failure-record (storage cleanup
   target). When the watchdog releases a stale lane, cluster B
   storage is coordinated cleanup (lane state cleared, optional
   failure record written). Without the intersection,
   watchdog-released lanes leave dangling state that the next
   cycle must reason about.

5. **Component-local persistence loaded at init** (AutoGen I-6 +
   Voyager I-V3 cross-system parallel). Cluster A init phase +
   cluster B component-local persistence per agent. Each component
   loads its own state at init; cluster A super-step boundary +
   cluster B distributed-storage. Without the intersection, init
   loads from inconsistent points and produces partial-state
   sessions where some components have current state and others
   have stale state.

**v1 failure modes addressed by A↔B**:

- **Stale-reference accumulation** (cleanup work cycles 60-61):
  cluster B writes happening at fuzzy cluster A boundaries
  produce stale references; sync-invariants-at-init (intersection
  sub-pattern 1) catches drift at the init boundary
- **Abandonment cascade**: dispatches dropped between sessions
  because cluster A has no explicit cycle-end boundary triggering
  cluster B failure-record write; intersection sub-pattern 3
  (failure-record-write-at-retry-exhaustion) provides the
  discipline
- **Forgotten-failure**: failed cycles disappear into journal
  entries because cluster B failure-as-artifact discipline isn't
  triggered at any cluster A boundary; intersection sub-patterns
  3 + 4 provide both the trigger and the write

**Phase 2 implication**: A↔B is the highest-priority intersection
for v1 failure-mode coverage. v2 candidates that adopt cluster A
boundaries AND cluster B mechanisms but **not** their intersection
will have super-step semantics + storage architecture but the
same stale-reference / abandonment / forgotten-failure failure
modes as v1. The intersection discipline (cluster B writes happen
at named cluster A boundaries) is what closes the gap. v2
candidate-shape implication: every cluster B write target should
name the cluster A boundary at which it fires; orphaned cluster
B writes (no associated boundary) are smell of within-cluster-
mechanism without intersection-discipline.

#### F↔H: stratification of feedback mechanisms

Cluster F provides 8 stratification axes (version, task-class,
capability-tier, terminology, role, cost-tier, autonomy-mode,
capability-layer). Cluster H provides 4 post-session feedback
sub-shapes (tight-cycle meta-feedback, score-gated consolidation,
continuous-background gardening, capability-accumulation). Their
intersection: feedback mechanisms differentiated along F-axes —
different feedback frequency, authority, gating, scope, or
content for different tiers.

Five sub-patterns from the corpus:

1. **Capability-layer × capability-accumulation** (Voyager I-V10 +
   I-V5/I-V6). Cluster F capability-layer (primitives vs
   LLM-composed compositions) + cluster H capability-accumulation.
   The capability layer determines what gets accumulated as
   feedback. Primitives are hand-written and don't accumulate;
   compositions/skills accumulate. Stratification of WHICH layer
   produces capability-accumulation feedback.

2. **Autonomy-mode × tight-cycle-vs-continuous** (Voyager I-V9 +
   openclaw operator-tier + cluster H tight + continuous).
   Cluster F autonomy-mode (auto vs manual) determines feedback
   frequency. Manual mode → tight-cycle Session-Insights-style
   (operator reads between cycles); auto mode → continuous-
   background gardening or score-gated consolidation (no human
   reads each cycle). Different autonomy modes get different
   feedback shapes.

3. **Cost-tier × score-gated consolidation** (Voyager I-V2 +
   openclaw I-O9). Cluster F cost-tier (model-per-task-class) +
   cluster H score-gated consolidation. High-cost tier (gpt-4 /
   Opus) needs gating to amortize cost across cycles; low-cost
   tier (gpt-3.5-turbo / Haiku) might do continuous-without-gating
   because per-action cost is low. Stratification of feedback
   economics.

4. **Role × clean-context-reviewer feedback** (Cognition I-C4 +
   cluster F role-stratification + cluster G role-asymmetric).
   When role-stratified architecture has a reviewer role
   (Cognition Devin Review, I-C4), the reviewer's feedback IS the
   cluster H mechanism for that role. Reviewer-role feedback
   (clean-context, no shared traces with action role) is shaped
   differently from action-role self-reflection. Stratification
   of feedback authority and trust.

5. **Task-class × Playbook-derived feedback** (Cognition I-C6 +
   cluster H tight-cycle). Cluster F task-class stratification
   (Playbook templates per task-class with outcome + steps +
   advice + forbidden) + cluster H tight-cycle. Each task class
   gets its own Playbook-derived meta-feedback shape; different
   task classes get different feedback templates and different
   `forbidden` lists. Stratification of feedback content per
   task class.

**v1 failure mode addressed by F↔H**: chronic-category currency
loop. v1 has uniform feedback for all chronic categories
regardless of category urgency, autonomy mode, or capability-
layer relevance. F↔H stratification produces differentiated
cross-cycle learning — high-priority chronic categories with
elevated feedback frequency, low-priority categories with
score-gated consolidation, capability-accumulating categories
at the appropriate layer.

**Phase 2 implication**: v2 candidates with cluster F sub-axes
AND cluster H mechanisms but **not** their intersection produce
uniform-feedback-across-tiers (v1's pattern). The intersection
is where stratification produces differentiated cross-cycle
learning. v2 candidate-shape implication: light-stratification
candidates (2-3 F sub-axes) need 1-2 H sub-shapes; heavy-
stratification candidates (all 8 F sub-axes) need at least 3
H sub-shapes to match the discrimination granularity. Orphaned
F sub-axes (axis with no associated H mechanism differentiation)
are smell of stratification-without-feedback-discipline.

#### D↔I: documentation-as-policy-enforcement

Cluster D produces honest documentation (anti-pattern catalogs,
walkbacks, invariants/derivations stratified, failure records,
MIGRATIONS.md). Cluster I produces harness-enforced security/
policy boundaries (default-deny, before_tool_call.block-true,
mechanical linters with agent-readable error messages). Their
intersection: documentation that is *machine-readable* and feeds
cluster I enforcement.

Five sub-patterns from the corpus:

1. **Anti-pattern catalog × mechanical-linter** (openclaw I-O2
   ANTI-PATTERNS.md + OpenAI harness mechanical linters). The
   anti-pattern catalog is documentation (cluster D) AND the
   source of linter rules (cluster I). The intersection IS the
   catalog being machine-readable enough to drive enforcement.
   Without the intersection, anti-patterns are advisory only —
   the catalog says "don't do X" but nothing prevents X.

2. **VISION.md "What We Will Not Merge" × ClawHub security
   review** (openclaw). Documentation declaration of forbidden
   patterns (cluster D) + harness-enforced policy at PR/dispatch
   time (cluster I). The "Will Not Merge" list IS the
   enforcement boundary; documentation and policy are the same
   artifact at different consumption layers.

3. **POSITIONS.md × tool-call validation** (Cognition I-C1
   walkback protocol + cluster I). Versioned positions
   documented (cluster D walkback as first-class artifact) AND
   enforced as constraints on tool calls (cluster I). The
   intersection: position changes propagate to enforcement
   without manual policy update.

4. **Invariants/Derivations stratified × axiom-enforcement**
   (Cognition I-C2 + cluster I). Invariants section is durable
   axiomatic documentation (cluster D invariants/derivations
   stratification) + enforcement that derivation-changes don't
   silently change axioms (cluster I machine check).
   Intersection: stratified documentation enables stratified
   enforcement (strict on invariants, advisory on derivations).

5. **Failure-record × failure-pattern-detection-watchdog**
   (Voyager I-V8 + cluster I). Structured failure records as
   documentation (cluster D failure-as-recorded-artifact)
   consumed by a watchdog/lint that detects re-occurring failure
   patterns (cluster I). Documentation-becomes-policy via
   repeat-detection: a documented failure that recurs N times
   becomes an enforced anti-pattern.

**v1 failure mode addressed by D↔I**: anti-patterns documented
in retrospective but not enforced. v1's retrospective cycle
catches anti-patterns retrospectively (lagging enforcement); the
system can re-introduce a documented anti-pattern and only catches
it on the next retrospective cycle. D↔I intersection produces
immediate enforcement at policy boundaries. v1's specific
instance: the chronic-category currency loop was a documented
anti-pattern (multiple retrospective cycles named it) that
re-occurred across many subsequent cycles because no I-level
enforcement prevented the orchestrator from re-entering it.

**Phase 2 implication**: v2 candidates with cluster D
documentation but no cluster I enforcement get
*lagging-corrective* behavior (retrospective-style catch). v2
candidates with both produce *immediate-prevention* behavior
(catch at point-of-violation). The intersection is what
determines whether documentation is advisory or load-bearing. v2
candidate-shape implication: candidates that adopt cluster D
anti-pattern catalogs MUST also adopt at least one cluster I
enforcement mechanism (mechanical linter, watchdog, harness-
policy) to convert documentation into prevention; otherwise the
catalog is decorative.

#### Additional intersections (flagged for future synthesis)

Four additional cross-cluster intersections are visible in the
corpus but not deeply mined this cycle. Brief observations:

- **A↔C: lifecycle operations beyond resume mapped to phase
  boundaries.** Cluster C lifecycle ops (terminate, fork, replay,
  reset, watchdog) happen AT cluster A boundaries. Sub-patterns:
  termination predicate (A) + terminate operation (C); stuck-
  watchdog (A) + lane-release (C); session-fork (C) + super-step-
  boundary (A). Without the intersection, lifecycle ops have
  ad-hoc execution semantics. Phase 2 implication: typed
  lifecycle requires explicit phase-boundary-where-lifecycle-op-
  applies pairings.

- **B↔C: storage operations on lifecycle-event boundaries.**
  Cluster B persistence + cluster C lifecycle ops. fork creates
  a branch in component-local persistence; replay reads existing
  failure-records to construct cycle context; reset truncates
  cluster B storage at a checkpoint. Phase 2 implication: rich
  cluster C lifecycle operations require coordinated cluster B
  mechanics — fork-without-storage-branching produces
  inconsistent forks; replay-without-failure-records produces
  context-free replays.

- **F↔I: tier-stratification of harness enforcement** (openclaw
  I-O6 capability-tier + I-O1 default-deny harness). Tier 1
  read-only enforced strictly; Tier 3 autonomous-with-standing-
  orders has fewer restrictions. Stratification of enforcement
  strictness per tier. Phase 2 implication: stratified
  enforcement allows graduated autonomy without blanket-permissive
  or blanket-restrictive policies.

- **E↔I: typed boundary discipline as enforcement substrate**
  (openclaw TypeBox + cluster I harness enforcement; LangGraph
  per-key reducers + boundary validation). TypeBox schemas (E)
  feed harness policy (I); boundary validation IS policy
  enforcement. Phase 2 implication: typed boundaries make
  I-level enforcement mechanical rather than ad-hoc; without
  typed boundaries, cluster I enforcement requires hand-written
  rules per boundary.

These four additional intersections are flagged but not deeply
mined this cycle. Future synthesis cycles can elevate them as
needed; PAI deeper-read [#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842)
or oh-my-codex deeper-read [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833)
returns may also surface new sub-patterns within these
intersections.

#### Meta-observation: cross-cluster as next-layer v2 design-input

The cross-cluster layer expresses *architectural-discipline-
emergent-from-mechanism-composition* rather than mechanism-by-
mechanism choices. Phase 2 candidates that adopt within-cluster
sub-shapes WITHOUT thinking about cross-cluster intersections
will produce systems where the mechanisms exist but don't compose
well. Concrete failure modes such candidates produce:

- *cluster A super-step semantics + cluster B storage but no
  A↔B intersection* → super-step boundaries don't trigger
  storage commits → same stale-reference failure as v1
- *cluster F sub-axes + cluster H mechanisms but no F↔H
  intersection* → uniform feedback across tiers → same
  chronic-category currency loop as v1
- *cluster D anti-pattern catalog + cluster I harness but no
  D↔I intersection* → anti-patterns documented but not
  enforced → same lagging-retrospective enforcement as v1

The within-cluster sub-shape catalogues from cycle 70 provide
WHAT mechanisms exist; the cross-cluster intersections from
cycle 72 provide HOW mechanisms compose to produce emergent
architectural properties. v2 candidate evaluation can be sharpened
by intersection coverage: how many of the cross-cluster
intersection disciplines does the candidate's architecture
explicitly address?

**Cluster I substrate-correlation revisited.** Cycle 70's
observation that cluster I is substrate-correlated to v1's
substrate (GitHub-Actions-anchored multi-actor with audit) is
strengthened by D↔I, F↔I, and E↔I intersections — three of the
seven flagged cross-cluster intersections involve cluster I. This
re-confirms cycle 70's recommendation: Phase 2 candidates SHOULD
weight cluster I patterns highly even at 2-system convergence
depth, AND they should weight cluster I's intersections (D↔I,
F↔I, E↔I) as part of the substrate-fit evaluation.

**Symmetric-vs-asymmetric intersection observation.** Some
intersections are symmetric — both clusters contribute mechanisms
that compose at their boundary (A↔B is symmetric: cluster A
boundaries trigger cluster B writes AND cluster B writes inform
cluster A boundary semantics). Others are asymmetric — one
cluster's artifacts feed the other's mechanisms (D↔I is
asymmetric: cluster D documentation feeds cluster I enforcement,
not the reverse; cluster I doesn't produce documentation that
feeds cluster D). v2 candidate-shape implication: asymmetric
intersections require explicit pipe-direction (which cluster's
artifact feeds which cluster's mechanism); symmetric intersections
require explicit composition-rule (how the two clusters'
mechanisms coordinate at their shared boundary).

### Open structural questions

Three structural questions surfaced by the implications-mining
cycles, deferred this synthesis:

1. **Should cluster F split into 2-4 narrower clusters?** Cycle 69's
   8 sub-axes partition naturally into four conceptual themes:
   *nomenclature* (version, terminology, role) / *resource semantics*
   (cost-tier, capability-tier) / *capability semantics* (task-class,
   capability-layer) / *governance semantics* (autonomy-mode).
   Splitting into 4 narrower clusters yields cleaner conceptual
   boundaries. Keeping cluster F unified preserves the
   meta-architectural-pattern that *stratification axes proliferate
   naturally where multi-agent architecture surfaces them*. Phase 2
   candidates can adopt either view; the synthesis layer here keeps
   the unified view and flags the question for revisitation when
   PAI deeper-read or oh-my-codex deeper-read return adds another
   stratification-foregrounding system.

2. **Cluster A vs cluster G boundary for per-action retry vs
   role-asymmetric context.** Voyager's CriticAgent ↔ ActionAgent
   loop has role asymmetry (different agents) but per-action
   granularity (each action gets a critic call) — cycle 69
   classified as cluster A (within-task retry, bounded-retry-with-
   feedback sub-shape). The argument for cluster G is non-trivial:
   critic and action have different trust semantics. The
   classification line is thin; future synthesis cycles or Phase 2
   candidate work may elevate the distinction. v2 candidates
   implementing this could choose either pattern shape — cluster A
   bounded-retry mechanism (within a single role's task lifecycle)
   or cluster G role-asymmetric context (different roles consume
   each other's outputs with explicit trust semantics).

3. **Orphan-pattern observation depends on within-system coherence.**
   Cycles 65-68 surfaced an orphan-pattern: implications without
   strong same-system neighbors are predominantly the ones
   anchoring cross-system clusters. Cycle 69 broke the pattern —
   Voyager has 0 orphans, all 10 implications participate in
   within-system intersections. Refinement: orphan pattern holds
   for moderately-coherent systems; tightly-coherent systems are
   exceptions. Methodological observation for future mining: the
   orphan-pattern correlates with within-system architectural
   coherence rather than being a universal mining-result property.
   v2 candidates that define explicit interaction protocols
   between named roles (high-coherence shape) will have implications
   participating in within-system intersections; v2 candidates with
   loose component-coupling will produce orphans whose value is in
   cross-system convergence.

4. **File-size threshold crossed; cluster section split deferred
   to next synthesis cycle.** Cycle 70's hand-off observed
   `1-research.md` approaching the cycle-33 restructure trigger
   (~1422 lines / 78KB). Cycle 72's cross-cluster intersections
   subsection (~310 lines added) puts the file past the trigger.
   Cycle 72 deferred the structural restructure (combining content
   addition with structural restructure in the same cycle is risky
   — link breakage, inconsistent diffs, and the editorial decisions
   compound). The next synthesis cycle (cycle 73+, or whenever new
   dispatch material arrives) should split the cluster section to a
   separate file like `1-research/clusters.md` mirroring the
   cycle-33 per-system-files split. The split criterion is now
   firmly met (line count > 1422 trigger; the `## Implications-
   mining clusters` section alone is ~1000 lines and could be its
   own file). v2 design-input from this observation: the
   file-size-driven restructure pattern is itself a meta-
   architectural-pattern; v2 candidates with active-surface
   artifacts should plan for periodic restructure triggers and
   have explicit thresholds (cycle-33 demonstrated 1422 lines as
   the threshold; cycle 72 demonstrates the same threshold reached
   via deeper-pass content rather than per-system content).

### Implications-mining cadence summary (cycles 62-72)

- Cycle 62: AutoGen mining (8 implications)
- Cycle 63: oh-my-codex deeper-read dispatch construction
  ([#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833),
  in flight 9+ cycles as of cycle 72)
- Cycle 64: LangGraph mining (8 implications)
- Cycle 65: cross-implications synthesis (first synthesis cycle;
  produced 6 elevation drafts, deferred actual elevation to a
  future synthesis cycle)
- Cycle 66: Cognition Devin mining (9 implications)
- Cycle 67: openclaw mining (10 implications)
- Cycle 68: OpenAI harness mining (10 implications)
- Cycle 69: Voyager mining (10 implications, exhausts unique
  deep-dive system pool)
- Cycle 70: synthesis with elevation of within-cluster sub-shape
  catalogues to this section (second synthesis cycle; completed
  the deferral cycle 65 set up — Family-format observations for
  clusters A/B/D/F/H + Phase 2 design-input + open structural
  questions, file 736→1241 lines)
- Cycle 71: PAI deeper-read dispatch construction
  ([#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842))
  + stuck-dispatch self-healing finding diagnosing #2833 8-cycle
  malformation (root cause: filed without `Copilot` as assignee;
  orchestrator-bot lacks GraphQL permission to self-fix; mitigation
  = diagnosis comments requesting Eva manually assign Copilot)
- Cycle 72 (this cycle): cross-cluster intersections synthesis
  (third synthesis cycle; A↔B, F↔H, D↔I primary deep-mine plus
  4 additional intersections A↔C/B↔C/F↔I/E↔I flagged for future
  synthesis; ~310 lines added; file-size threshold crossed,
  restructure deferred to cycle 73+)

Total: 45 implications across 6 systems across 9 clusters across
8 mining cycles, plus 3 synthesis cycles producing within-cluster
sub-shape catalogues + cross-cluster intersection patterns + per-
cycle process documents under `_notes/`. Implications-mining
cadence on unique deep-dive systems is exhausted post-cycle 69.
Future mining requires either dispatch deliveries (oh-my-codex
via [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833)
still in flight, PAI via [#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842)
dispatched cycle 71) OR re-mining existing systems at deeper
depth. Synthesis cycles continue extending upward (within-cluster
→ cross-cluster → potentially cross-system architectural-pattern
synthesis) while waiting for new mining material; the synthesis
arc has been substantively additive across cycles 65, 70, 72.

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
