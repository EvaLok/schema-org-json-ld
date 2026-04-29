# Phase 1: External Research

## Status

**Phase 1 initiated cycle 14** (commit pending). Phase 1 is authorized to
run in parallel with finishing Phase 0 per `input-from-eva` issue #2759
(2026-04-28) and the redesign-prompt update at commit
[`f77b4341`](https://github.com/EvaLok/schema-org-json-ld/commit/f77b4341).
Phase 2 candidate generation still requires explicit post-retrospective
checkpoint approval; Phase 1 reading is preparatory, not a
candidate-generation step.

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

## Initial findings (cycle 14)

### openclaw (openclaw/openclaw, 365k★ / 75k forks / TypeScript)

Cycle 14 read: `README.md`, `VISION.md`. Repo size 663MB suggests a
substantially larger codebase than the README captures; deeper reads
(directory survey, architecture pages, key source files) deferred to
later cycles.

**Architectural patterns named.** Local-first Gateway as single
control plane for sessions/channels/tools/events. Multi-channel
inbound (25+ messaging surfaces). Multi-agent routing (route channels
to isolated agents with workspaces + per-agent sessions). Plugin API
with two styles (code plugins for runtime extension, bundle-style
plugins for stable external surfaces). Memory as a special plugin
slot — only one memory plugin active at a time.

**Security posture.** "Treat inbound DMs as untrusted input." Default
DM policy is `pairing` (unknown senders get a pairing code, message
not processed; explicit approval required). Sandbox modes for
non-`main` sessions (Docker default; SSH and OpenShell backends).
Default sandbox allows `bash`, `process`, `read`, `write`, `edit`,
`sessions_*`; denies `browser`, `canvas`, `nodes`, `cron`,
`discord`, `gateway`. The pattern is *strong defaults with
operator-controlled knobs for trusted high-power workflows*.

**Roadmap guardrails (VISION.md "What We Will Not Merge").**
Explicit anti-patterns:
- "Agent-hierarchy frameworks (manager-of-managers / nested planner
  trees) as a default architecture"
- "Heavy orchestration layers that duplicate existing agent and tool
  infrastructure"

This is a **counter-signal to LangGraph/AutoGen-style hierarchical
agents**. The openclaw maintainers explicitly reject manager-of-
managers as a default. The reasoning is not given in VISION.md;
deeper reads should look for the actual rationale. *If our Phase 2
candidates trend toward hierarchical agents, this counter-signal is
worth weighing.*

**Tooling philosophy.** "Core stays lean; optional capability should
usually ship as plugins." Bundle-style plugins (preferred when
expressive enough) have "smaller, more stable interface and better
security boundaries." Plugin promotion to core has "intentionally
high" bar.

**Anchoring caveats on openclaw.**
- Single-user personal assistant; not multi-agent autonomous.
- Has human-in-the-loop on every interaction (user issues
  commands).
- TypeScript-first by stated design choice ("fast to iterate in,
  easy to read, modify, and extend") — our system mandates Rust for
  tools.
- 365k★/75k-fork scale is anomalous for a 5-month-old repo
  (created 2025-11-24); raises questions about whether the
  repository reflects an organic engineering project or has unusual
  amplification (parody/marketing/etc). The architecture is real
  regardless, but external validation of design choices is weaker
  than the star count suggests.
- The "EXFOLIATE!" slogan and lobster theming flag this as at least
  partly tongue-in-cheek; treat architectural claims on their own
  merit, not on project gravitas.

**Patterns observed in openclaw** (relevance evaluation deferred to
cross-system synthesis, gated on multi-system reading):
- Small-core architecture with plugin-extensible capability
- Anti-pattern list ("What We Will Not Merge") as a deliverable artifact
- Memory as a singleton plugin slot (one active mechanism, replaceable,
  not layered)
- Strong-defaults-with-operator-knobs security posture

### PAI (danielmiessler/Personal_AI_Infrastructure, 12k★ / 1.6k forks / TypeScript)

Cycle 14 read: `README.md` (extensive). Deeper reads of `Tools/`,
`Packs/`, `.claude/`, `Releases/v4.0.3/` deferred to later cycles.

**The 16 PAI Principles** (verbatim summaries from the PAI README):

| # | Principle | PAI's framing |
|---|-----------|---------------|
| 1 | User Centricity | Built around the user, not tooling |
| 2 | The Foundational Algorithm | Observe → Think → Plan → Build → Execute → Verify → Learn |
| 3 | Clear Thinking First | Good prompts come from clear thinking |
| 4 | Scaffolding > Model | System architecture matters more than which model |
| 5 | Deterministic Infrastructure | AI is probabilistic; infrastructure shouldn't be |
| 6 | Code Before Prompts | If you can solve it with bash, don't use AI |
| 7 | Spec / Test / Evals First | Specs and tests before building |
| 8 | UNIX Philosophy | One thing well; composable; text interfaces |
| 9 | ENG / SRE Principles | Treat AI infra like production software |
| 10 | CLI as Interface | CLI faster, more scriptable, more reliable than GUIs |
| 11 | Goal → Code → CLI → Prompts → Agents | Decision hierarchy for where behavior lives |
| 12 | Skill Management | Modular capabilities routing on context |
| 13 | Memory System | Everything worth knowing gets captured; history feeds context |
| 14 | Agent Personalities | Specialized agents with unique voices |
| 15 | Science as Meta-Loop | Hypothesis → Experiment → Measure → Iterate |
| 16 | Permission to Fail | Explicit permission to say "I don't know" |

**Strikingly aligned with our CORE-DESIGN-PRINCIPLE.** Principles 4, 5, 6,
8, 11 specifically describe the same posture our redesign is targeting:
deterministic tools handle procedure; orchestrator handles judgment.
Principle 11's hierarchy "Goal → Code → CLI → Prompts → Agents" is the
explicit decision-tree version of what our prompt names as "test for
violation: can a tool do this deterministically? If yes, build the
tool, do not embed the procedure in the prompt."

**Continuous-learning loop.** The PAI primary loop is:

> Observe → Think → Plan → Execute → Verify → **Learn** → Improve

The "Learn" closure is the explicit meta-loop for self-modification.
Compare v1 which has Observe (audit/review)-Plan (worklog)-Execute
(tools)-Verify (pipeline-check)-Learn (journal) but *no closed loop
to Improve* — the journal is reflective but does not produce
mechanical change. PAI claims (in the README) that PAI does close
this loop; verifying that claim requires reading the actual code,
not the README.

**Memory as a top-level principle.** Principle 13: "Memory System —
Everything worth knowing gets captured. History feeds future
context." Treating memory as a first-class architectural concern
(not scaffolding) aligns with our `PERSISTENCE` directive's framing
of multi-session memory as a design problem to solve, not an
implementation detail.

**Anchoring caveats on PAI.**
- Single-user personal assistant; user provides goals, system pursues.
  Our system has no human-provided goal stream beyond Eva's
  occasional input-from-eva — the orchestrator generates its own
  next-cycle work from prior cycles.
- "Memory" in PAI captures user preferences and history (what *the
  user* has done / wanted). Our "persistence" captures
  cross-session orchestrator state (what *the orchestrator* has
  decided / is investigating). These are different shapes despite
  the shared label.
- TypeScript / Bun-based. Our tools are Rust.
- The 16 principles are stated as design goals in the README;
  whether the PAI codebase actually instantiates each of them is
  not yet verified by code-reading.
- The README is a marketing surface (it includes the project's
  philosophical mission about "activating people"). Architecture
  detail will be in deeper docs and code; the README is the
  starting point, not the substance.

**Patterns observed in PAI** (relevance evaluation deferred to
cross-system synthesis, gated on multi-system reading):
- Principle-list published as part of the deliverable (16 named
  principles included in PAI's README)
- Decision hierarchy "Goal → Code → CLI → Prompts → Agents" for
  where capability lives
- Closed feedback loop (Observe→Think→Plan→Execute→Verify→Learn→Improve)
  with explicit Learn → Improve closure
- Explicit "I don't know" as a sanctioned response (PAI's Principle 16,
  "Permission to Fail")

### AutoGen (microsoft/autogen, currently in maintenance mode)

Cycle 15 dispatched a Copilot research-only session (gpt-5.5; issue
#2762; canonical cycle-6 procedure). Deliverable: 697-line deep-dive at
PR [#2763](https://github.com/EvaLok/schema-org-json-ld/pull/2763)
covering all seven dispatch lenses (architecture, state, orchestration,
failure, tools, anti-patterns, anchoring). This section is the
navigation summary; PR #2763 is the evidence base with quoted citations
from AutoGen documentation at SHA
`25f7cc162ae92c3988966d85cce173ff6df48020`.

The dispatch body explicitly pre-loaded the cycle-15 anti-smuggling
discipline (four named anchoring differences: library vs orchestrator,
human-in-the-loop vs minimal, Python vs Rust, short tasks vs
multi-cycle). The deliverable's "Patterns observed in AutoGen" section
is observation-shaped without v2-relevance framings — the discipline
was honored.

**Project status as observable evidence.** The README states:
"AutoGen is now in maintenance mode. It will not receive new features
or enhancements and is community managed going forward." New users are
directed to Microsoft Agent Framework. Maintenance-status signaling
and successor-framework recommendation in a project's primary README
are themselves an observable pattern.

**Layered architecture.** Five named layers: Core API (event-driven
actor runtime, message passing, distributed runtime), AgentChat API
(opinionated high-level presets), Extensions API (model clients,
tools, code execution), AutoGen Studio (developer GUI; explicitly
"not meant to be a production-ready app"), AutoGen Bench (benchmarking).
The unopinionated/opinionated split is explicit: Core says "It is not
tied to any specific agent abstraction or multi-agent pattern.";
AgentChat provides "intuitive defaults, such as Agents with preset
behaviors and Teams with predefined multi-agent design patterns."

**Multiple orchestration patterns as first-class.** AutoGen does not
assert one canonical orchestration shape. Documented patterns:
direct single-agent task execution, single-agent-with-tool-loop,
agent-as-tool composition (`AgentTool`), round-robin shared-context
group chat (`RoundRobinGroupChat`), centralized speaker selection
(`SelectorGroupChat`), localized handoff/swarm (`Swarm`),
directed-graph workflows (`GraphFlow`), lead-orchestrator with task
ledger and progress ledger (`MagenticOneGroupChat`), and custom Core
protocols. Notably, built-in sequential chat was REMOVED in v0.4 as
"too opinionated and not flexible enough" — a deliberate de-prescription
between major versions.

**State as serialized component dictionaries.** State save/load is
explicit, dictionary-based, component-local. The state tutorial
example shows concrete shape: `TeamState` containing `agent_states`,
`RoundRobinManagerState` with `message_thread`, `current_turn`,
`next_speaker_index`, and `AssistantAgentState` with `llm_messages`.
There is no single global state file as the system center; state
serializes through per-component APIs. Reset (clear) and resume
(continue) are distinct team operations.

**Termination as first-class composable callables.** Termination
conditions are objects evaluated against recent messages/events,
combinable with AND (`&`) and OR (`|`); types include
maximum-messages, text-mention, token-usage, timeout, source-match,
external-termination. External graceful stop is distinct from
immediate cancellation: the former lets the current agent's turn
finish before team stop ("keeping the team's state consistent"); the
latter is exception-based abort.

**What AutoGen does not centrally guarantee** (deep-dive lens 4 list):
semantic correctness of final answers; deadlock diagnosis beyond
timeouts/cancellation; global reconciliation of all component states;
automatic retry policy for malformed model output; durable recovery
from process crash without app-level persist/reload. Failure handling
is explicitly delegated: "define a protocol, stream/observe it,
terminate it, cancel if needed, persist state if the app needs resume."

**Trust boundaries explicitly named.** Local code executor is
documented as dangerous: "The local version will run code on your
local system. Use it with caution." Docker code executor is the safer
default. MCP integration carries an explicit warning: "Only connect
to trusted MCP servers as they may execute commands in your local
environment or expose sensitive information." Magentic-One docs warn
about prompt injection from web content and risky autonomous actions
(cookie agreements, recruiting humans). Trust posture: dangerous
capabilities are exposed, documented as application-operator
responsibility, not framework guarantee.

**Tool integration model.** Tool calling is schema-driven: the model
emits a structured tool-call (name + JSON arguments matching the
declared schema), and the host executes registered code. Agents
themselves can be exposed as tools via `AgentTool` composition; for
stateful agent-or-team tools, parallel execution is explicitly
forbidden ("the same agent or team cannot be used in parallel"). The
v0.4 migration replaces the v0.2 user-proxy tool-routing path with
direct tool execution inside `AssistantAgent` ("which is much simpler
and easier to understand"). Tool errors are reported as result objects
with `is_error: true` rather than thrown exceptions, keeping error
shape uniform with success shape.

**Anti-patterns explicit in v0.4 migration guide.** v0.2
`ConversableAgent.register_reply` callback registration is discouraged
("guessing what the reply_func does, all its parameters, and what the
position should be"). Old group-chat tool routing through user proxy
was problematic: "We have observed numerous issues with this
approach." `AssistantAgent` itself is documented as a "kitchen sink"
prototype: "Make sure you read the documentation and implementation
to understand the design choices. Once you fully understand the
design, you may want to implement your own agent." Some lifecycle
features are aspirational, not implemented (agent paging in/out:
"not implemented yet").

**Anchoring caveats on AutoGen.** These caveats argue *non-transfer*:
each names a difference between AutoGen's substrate and the redesign's
substrate that may discount specific patterns. The reverse — which
patterns DO transfer despite these differences — is not derivable from
this list alone. Transferability requires a positive argument per
pattern, not just absence of a discount-reason in this section.
- Library/framework vs autonomous orchestrator: AutoGen exposes
  abstractions for application developers to compose; the redesign
  target chooses one behavior contract and lives with it across many
  cycles. A pattern that's tolerable as a library option may be too
  underspecified for a concrete cron-run orchestrator.
- Application prompt/task vs scheduled cron: AutoGen examples assume
  explicit task boundary (`agent.run(task=...)`); the redesign's task
  boundary is partly inferred from repository state.
- Human-in-the-loop, especially for risky tools: Magentic-One docs
  recommend human approval for code execution; the redesign runs
  minimal-intervention per `EVA-DEFAULT-AUTONOMY`.
- Python-first with optional .NET; the redesign tooling is Rust on
  GitHub Actions. Architectural ideas transfer; runtime/library
  affordances do not.
- Developer-owned state persistence: AutoGen state is component-local
  dictionaries; the redesign's repo-resident state with cross-cycle
  ledger semantics is a different shape.
- Maintenance mode: any "framework will add this later" claim should
  be discounted.
- Short-to-medium tasks vs long-horizon multi-cycle work: AutoGen's
  documented examples are interactive or benchmark-style agentic
  tasks; the redesign target spans hundreds of cycles with
  institutional-memory problems AutoGen docs do not center.

**Patterns observed in AutoGen** (relevance evaluation deferred to
cross-system synthesis, gated on multi-system reading):
- Layered architecture with Core / AgentChat / Extensions / Studio /
  Bench separation
- Explicit maintenance-status signaling with successor-framework
  recommendation in the project README
- Actor-model framing with runtime-mediated identity
  (`AgentID = (Agent Type, Agent Key)`)
- Multiple orchestration patterns coexist as first-class because
  behavior contracts are expressed as message protocols rather than as
  a universal orchestrator object (round-robin, selector, swarm, graph,
  lead-orchestrator)
- Magentic-One's Task Ledger / Progress Ledger vocabulary for
  lead-orchestrator planning and tracking
- Stateful-by-default agents and teams with reset/resume distinction
- Component-local state save/load as dictionaries (no central state
  file)
- Model-context abstraction separating stored conversation history
  from the model-visible virtual view
- Composable termination conditions with AND/OR over events
- Graceful external stop distinct from exception-based cancellation
- Schema-driven tool calling: model emits structured call; host
  executes registered code
- Agent-as-tool composition with model-driven delegation, with
  parallel execution explicitly forbidden for stateful agent/team
  tools
- Reflection pattern (generator + critic loop) for robustness, named
  in design-patterns documentation
- Aspirational-vs-implemented distinction explicitly documented
  ("not implemented yet" called out in lifecycle docs)
- Explicit anti-pattern documentation including kitchen-sink-warning
  on the documented preset agent and removal of opinionated helpers
  ("too opinionated") between major versions

### Voyager (MineDojo/Voyager, research artifact, last commit 2023-07-27)

Cycle 17 orchestrator-direct read. Sources: README on master,
[arxiv 2305.16291](https://arxiv.org/abs/2305.16291) abstract,
`voyager/voyager.py`, `voyager/agents/skill.py`, `voyager/agents/critic.py`,
`voyager/agents/curriculum.py` (first 150 lines), `voyager/prompts/`
listing. The full paper PDF is on `voyager.minedojo.org` (not fetched
this cycle; abstract + code suffice for architecture reading).

**Status as observable evidence.** Voyager is a research-paper artifact
("the first LLM-powered embodied lifelong learning agent in Minecraft").
Last commit on master is 2023-07-27 — codebase stable as a research
reference, not actively maintained. README front-loads the experimental
claim and the three named components; no successor-framework
recommendation (parallel to AutoGen's maintenance-mode signaling, but
the rationale is "research artifact" rather than "framework deprecated").

**Four-agent architecture with explicit named roles.** `voyager/agents/`
contains four classes:
- `ActionAgent` — the "iterative prompting mechanism" in the paper.
  Generates JavaScript code (Mineflayer API) for the next task; consumes
  execution feedback for refinement.
- `CurriculumAgent` — automatic curriculum. Selects what task to attempt
  next from world state, completed-tasks history, and failed-tasks
  history.
- `CriticAgent` — self-verification. Returns structured
  `{success: bool, critique: str}` from environment events vs task
  statement.
- `SkillManager` — persistent skill library. Stores executable code +
  LLM-generated description + vector embeddings.

**Cost tiering across agents.** Different agents use different model
tiers in default configuration:
- ActionAgent, CurriculumAgent (main), CriticAgent: `gpt-4`
- CurriculumAgent QA-cache lookups, SkillManager skill-description
  generation: `gpt-3.5-turbo`

The pattern: more-expensive model for novel reasoning (action-code
generation, curriculum-task selection, critic-verification); cheaper
model for cached/derivative work (caching Q&A about world state,
generating descriptions of just-written code).

**Component-local persistence.** Each agent persists state in its own
subdirectory under `ckpt/`:
- `ckpt/skill/` — skill library (JSON manifest, per-skill `.js` and
  `.txt` files, Chroma vectordb)
- `ckpt/curriculum/` — `completed_tasks.json`, `failed_tasks.json`,
  `qa_cache.json`, vectordb of cached questions
- `ckpt/action/` — action-agent chat log
- `ckpt/event/` — event recorder for environment events

No central state file; resume is opt-in per-agent (`resume=True`).
Parallel to AutoGen's component-local-dictionaries pattern (different
domain, similar shape).

**Sync invariants asserted at init.** SkillManager asserts
`vectordb._collection.count() == len(self.skills)` at construction;
CurriculumAgent asserts the same for the QA-cache vectordb vs
`qa_cache.json`. Error messages name the failure mode and remediation
("Did you set resume=False ... You may need to manually delete the
vectordb directory"). Dual-storage divergence is a fail-fast condition
at boot, not a silent runtime error.

**Skill versioning is append-on-disk, replace-in-vectordb.** When
`add_new_skill` runs on an existing skill name, the vectordb entry is
deleted and re-added with the new version; the new code is written to
`<name>V2.js`, `<name>V3.js`, ... — old code is never deleted from the
filesystem. The active retrieval surface (vectordb) is single-version;
the disk is monotonic-append history.

**Skill retrieval as semantic similarity.** `retrieve_skills(query)`
returns top-k (default 5) skills by similarity to the query embedding,
where embeddings are over LLM-generated skill descriptions, not raw
code. Retrieved skills get composed into action prompts as available
context.

**Iteration mechanism with bounded retries.**
`action_agent_task_max_retries = 4`. On action failure, the critic's
critique + execution error feeds into the next action prompt; the
action agent rewrites code for the same task. Skills are added to the
library only on `success=True`. Failed tasks accumulate in
`failed_tasks.json`; the curriculum agent uses both completed and
failed history when selecting the next task. Failure is a recorded
artifact, not just a transient.

**Mode toggleability for human-in-the-loop.** CurriculumAgent:
`mode="auto"` (LLM-selected tasks) or `mode="manual"` (human-curated).
CriticAgent: `mode="auto"` (LLM-verified) or `mode="manual"` (human
verifies via stdin prompts). Human-in-the-loop is a configurable mode,
not the architectural default; the manual codepaths are explicit
methods (`human_check_task_success`).

**Curriculum warm-up gates context disclosure based on progress.** The
curriculum's `default_warmup` dict gates which world-observation fields
appear in the curriculum prompt based on completed-tasks count.
Example: `"context": 15` means context-elaboration is hidden until ≥15
tasks completed; `"hunger": 15` similarly delays hunger-reasoning.
Newer agents see less; matured agents see more. Capability disclosure
is a function of progress, not fixed configuration.

**No model fine-tuning.** Per the README: "Voyager interacts with GPT-4
via blackbox queries, which bypasses the need for model parameter
fine-tuning." Learning happens through skill-library accumulation and
prompt-context updates, not gradient updates. The architecture decision
is explicit and load-bearing in the paper's framing.

**Two-layer capability composition.** Two layers of code are made
available to the action agent in prompts:
- `voyager/control_primitives/` — hand-written low-level Mineflayer
  primitives (e.g., `mineBlock`, `craftItem`, `placeItem`).
- Skill library — LLM-generated compositions of primitives (and earlier
  skills).

Skills compose primitives; later skills compose earlier skills.
Compositionality is the paper's named learning mechanism.

**Prompts as external files split by sub-task.** `voyager/prompts/`
contains 8 distinct prompt templates: `action_template.txt`,
`action_response_format.txt`, `critic.txt`, `curriculum.txt`,
`curriculum_qa_step1_ask_questions.txt`,
`curriculum_qa_step2_answer_questions.txt`,
`curriculum_task_decomposition.txt`, `skill.txt`. Curriculum's
task-selection is split across three prompt files for sub-decisions.
Code handles variable injection; prompts hold instructions.

**Anchoring caveats on Voyager.** These caveats argue *non-transfer*:
each names a difference between Voyager's substrate and the redesign's
substrate that may discount specific patterns. The reverse — which
patterns DO transfer despite these differences — is not derivable from
this list alone. Transferability requires a positive argument per
pattern, not just absence of a discount-reason in this section. The
asymmetry is worth naming because confirmation-bias-on-aligned-
principles (failure mode #1 in the anchoring discipline section) has a
counterpart failure mode of over-discounting via blanket caveats.
- **Continuous-runtime vs cold-cycle.** Voyager runs as a single
  process holding agent state in memory across many tasks; the
  redesign target runs in 75-minute cycles with cold restarts between.
  Voyager's "lifelong" continuity is a runtime property; the redesign
  must reconstruct equivalent continuity from disk every cycle.
- **Embodied environment with rich observations vs sparse repository
  state.** Voyager's "world" is a Minecraft instance with
  biome/inventory/voxels/entities/health observations every step; the
  redesign's "world" is a git repo + GitHub issues + cron triggers,
  with much sparser per-step observation surface.
- **Concrete execution feedback vs fuzzy outcome feedback.** Voyager
  skills succeed or fail by concrete code execution + critic check on
  environment events; the redesign's outcome feedback is fuzzier
  (next-cycle retrospection, audit critique, schema-output quality).
- **Skill = executable code in a sandbox vs tool = build-time
  artifact.** Voyager's skills are LLM-generated JavaScript run in an
  external Node.js Mineflayer process; the redesign's tools are Rust
  binaries built at repo-time and reviewed by humans. Voyager's skill
  discipline is at runtime; the redesign's tool discipline is at
  construction.
- **Single agent vs multi-orchestrator.** Voyager has one agent in one
  runtime; the redesign system already runs two orchestrators (main +
  audit) on independent crons.
- **Internal curriculum vs externally-supplied curriculum.** Voyager's
  curriculum agent autonomously selects what to learn next; the
  redesign's "curriculum" is provided by Eva, schema-org work, and the
  retrospective F-patterns.
- **Single-LLM-vendor coupling vs multi-vendor.** Voyager hardcodes
  ChatOpenAI / OpenAIEmbeddings (langchain bindings); the redesign uses
  Claude (Anthropic) for orchestration and Copilot (multiple OpenAI
  models) for dispatches. Vendor-coupling assumptions in Voyager don't
  transfer.
- **Research artifact vs production-grade target.** Voyager is
  unmaintained since 2023-07-27; pattern observations should be
  evaluated as "the design choices a research project documented" not
  "the design choices a production-stable framework converged on."
  Some patterns may have been chosen for paper-narrative reasons rather
  than long-run robustness.

**Patterns observed in Voyager** (relevance evaluation deferred to
cross-system synthesis, gated on multi-system reading):
- Four-agent architecture with explicit named roles (action, curriculum,
  critic, skill-library)
- Cost-tiering across agents: cheap model for cached/derivative work,
  expensive model for novel reasoning
- Component-local persistence with `resume=True` opt-in per agent (no
  central state file)
- Sync invariants asserted at init for dual-storage components (vectordb
  count vs JSON manifest count, fail-fast on divergence)
- Skill versioning as append-on-disk + replace-in-vectordb (active
  surface single-version, history monotonic)
- Top-k semantic skill retrieval via vector similarity over generated
  descriptions (not raw code)
- Bounded retries on action failure with critic-critique + execution-error
  fed into next prompt
- Failed-task accumulation in dedicated JSON file alongside
  completed-task accumulation (failure as recorded artifact, not
  transient)
- Human-in-the-loop as configurable mode (`auto`/`manual`) per agent,
  not architectural default
- Curriculum warm-up gating which observation fields are disclosed based
  on completed-tasks count
- Explicit no-fine-tuning architectural commitment, with skill-library
  as the named learning mechanism
- Two-layer capability composition: hand-written primitives + LLM-composed
  skills over primitives
- Compositionality (skills compose primitives; later skills compose earlier
  skills) as the paper's named learning mechanism within the skill-library
  architecture
- Prompts as external files split by sub-task (curriculum decomposed
  across three prompt files for sub-decisions)
- Structured critic output (`{success: bool, critique: str}`) rather
  than free-form review
- LLM-generated skill descriptions as the embedding surface (embeddings
  are over descriptions, not over raw code)
- QA-cache pattern for repeated curriculum lookups (`qa_cache.json`
  plus vectordb of cached questions, kept in sync)

### LangGraph (langchain-ai/langgraph, active; post-LangChain stateful spinout)

Cycle 18 dispatched a Copilot research-only session (gpt-5.5; issue #2767;
canonical cycle-15 procedure). Deliverable: 797-line deep-dive at PR
[#2768](https://github.com/EvaLok/schema-org-json-ld/pull/2768) covering all
seven dispatch lenses (architecture, state, orchestration, failure, tools,
anti-patterns, anchoring). This section is the navigation summary; PR
#2768 is the evidence base with extensive citations to LangGraph docs in
`langchain-ai/docs` (12 docs files) and source files in `langchain-ai/
langgraph` (8 files including `pregel/main.py`, `graph/state.py`,
checkpoint base/Postgres/SQLite, store base, `tool_node.py`,
`chat_agent_executor.py`).

The dispatch body explicitly pre-loaded the cycle-18 anchoring-caveats
discipline (symmetric framing: caveats discount, transferability requires
positive arguments per pattern). Section 7 of the deliverable structures
each anchoring caveat with explicit "Discounts" and "Transfers"
subsections — the cycle-18 instruction was honored.

**Project status as observable evidence.** LangGraph is active and
explicitly framed as the post-LangChain stateful pivot. The overview says
"LangGraph is very low-level, and focused entirely on agent
**orchestration**" and "LangGraph does not abstract prompts or
architecture" (`overview.mdx`). The product layering is explicit: LangGraph
(low-level runtime) ← LangChain agents (prebuilt architectures on top) ←
LangSmith / Agent Server (deployment / observability infrastructure).
LangGraph "can be used without LangChain" but interoperates with LangChain
`Runnable` conventions. This is a different posture from AutoGen's
maintenance-mode-with-successor signaling: LangGraph signals continuing
investment with explicit non-goal of architectural opinion.

**Pregel / bulk-synchronous-parallel super-step execution.** The runtime
docs anchor execution in Pregel/message passing: nodes activate when
incoming channels update; active nodes run and emit updates; parallel
nodes within a super-step do NOT observe each other's writes until the
super-step ends; execution proceeds by plan/execution/update phases. This
is a substantively different orchestration shape from AutoGen's actor
model (where actors react to messages with no super-step boundary).
Reducers are core (not optional) precisely because parallel writes within
one super-step need a deterministic merge rule.

**State as typed channels with per-key reducers.** The graph API docs
define graph state as schema plus reducers, with `TypedDict` /
`dataclass` / `Pydantic BaseModel` schema options (Pydantic explicitly
flagged as less performant). Per-key reducers via `Annotated[T, reducer]`
in Python or `ReducedValue` in TypeScript. Default channel behavior is
overwrite/last-value; reducer-backed channels accumulate. `Overwrite`
explicit bypass mechanism exists. Multiple schemas (Input/Output/Overall/
Private) supported around one internal state. State is a channel map,
not a single blob with one update policy.

**Checkpointing at super-step boundaries with thread-scoped histories.**
Persistence is built around `BaseCheckpointSaver` and `thread_id`-keyed
storage. Checkpoints save at each super-step boundary; resume can only
happen from a checkpoint (graph-step rewind, not arbitrary instruction-
level rewind). `StateSnapshot` includes `values`, `next`, `config`,
`metadata`, `parent_config`, `tasks`. The implementation `Checkpoint`
type carries more than the docs summary: `channel_versions`,
`versions_seen`, `pending_sends`, `updated_channels` — causal/version
metadata, not just a state JSON dump. Subgraphs add `checkpoint_ns`
namespaces (`""` for root; `"node_name:uuid"` for subgraphs; nested
subgraphs joined by `|`). Backends include in-memory, Postgres, SQLite
(in monorepo), MongoDB, Redis (external packages).

**Pending writes for failed super-steps.** A LangGraph-specific recovery
mechanism: when a node fails mid-execution at a given super-step,
LangGraph stores pending checkpoint writes from any other nodes that
completed successfully at that super-step. When resuming, successful
nodes are not re-run. This is stronger than checkpoint-at-end recovery; it
acknowledges parallel super-steps where one branch can fail while
siblings completed. The `WRITES_IDX_MAP = {ERROR: -1, SCHEDULED: -2,
INTERRUPT: -3, RESUME: -4}` constant in checkpoint base shows special
writes are persisted in the writes-table indexing model rather than
thrown only as process exceptions.

**Time travel as append-only fork, not destructive rollback.** Two
operations: replay (retry from a prior checkpoint) and fork (branch from
a prior checkpoint with modified state). Both work by resuming from a
checkpoint; nodes after the checkpoint re-execute (LLM calls, API
requests, interrupts can return different results). The docs warn:
"`update_state` does **not** roll back a thread. It creates a new
checkpoint that branches from the specified point. The original execution
history remains intact." This is the architectural distinction:
LangGraph's "time travel" is append-only branching inside a thread
history.

**Short-term (thread-scoped) vs long-term (cross-thread) memory.** The
persistence docs explicitly motivate the split: "With checkpointers
alone, we cannot share information across threads. This motivates the
need for the `Store` interface." Checkpoints are execution history;
stores are application memory. Storage shape in examples: namespace +
key + value, with `BaseStore` providing `put`/`search`/`get`. Production
stores include `PostgresStore` and `RedisStore`.

**Interrupts as checkpoint/resume with restart-from-beginning warning.**
Interrupts are LangGraph's primary HITL primitive: a node calls
`interrupt(payload)`, LangGraph saves state via persistence, the payload
surfaces to the caller, execution waits indefinitely, caller resumes with
`Command(resume=...)` and the same thread ID. The docs warn: "the node
restarts from the beginning of the node where the `interrupt` was called
when resumed, so any code before the `interrupt` runs again." Interrupts
are not language-runtime continuations; they are checkpoint/resume/replay
at node granularity. Multiple parallel interrupts can compose with
resume-id maps.

**Multiple orchestration patterns coexist as first-class.** Like AutoGen,
LangGraph documents many patterns rather than asserting one canonical
shape: prompt chaining, routing, parallelization, orchestrator-worker,
evaluator-optimizer, agent/tool loop (ReAct-style), subgraphs, subagents
/ supervisor, handoffs, skills, router, custom workflow. The position
against reflexive multi-agent decomposition is explicit ("not every
complex task requires this approach—a single agent with the right
(sometimes dynamic) tools and prompt can often achieve similar results").
This matches openclaw's anti-pattern stance and AutoGen's v0.4 removal of
"too opinionated" helpers.

**Subgraph composition (graphs as nodes).** Two subgraph patterns:
(1) call subgraph inside a wrapper node when parent and subgraph have
different state schemas (parent maps state in/out); (2) compile subgraph
directly as a node when parent and subgraph share channels. Subgraph
streams can include namespaces; checkpoint namespaces identify nested
graph snapshots. This makes subgraphs not just code reuse, but
inspectable nested execution.

**Honest implementation-vs-marketing-claims discipline.** The deliverable's
section 2.8 separates well-supported claims from claims-that-need-
qualification: "resume exactly where they left off" is checkpoint-granular
not line-granular; time travel is not pure deterministic replay; durable
execution does not auto-handle idempotence; the Agent Server can hide
persistence setup but moves complexity into LangSmith infrastructure
rather than removing it. The durable-execution docs themselves admit
"the code does **NOT** resume from the **same line of code** where
execution stopped." This is research-evaluation honesty, not v2-relevance
smuggling.

**Anchoring caveats on LangGraph.** These caveats argue *non-transfer*:
each names a difference between LangGraph's substrate and the redesign's
substrate that may discount specific patterns. Per the cycle-18
anchoring-caveats-symmetric discipline, each caveat explicitly names what
DOES transfer despite the difference. Transferability is established at
pattern level (not specific v2 prescriptions) — the deliverable preserves
that framing.
- **Library for stateful applications vs autonomous GitHub orchestrator.**
  Discounts API ergonomics for arbitrary developers, Agent Server
  deployment assumptions, generic visualization. Transfers explicit state
  schemas + per-field reducers, checkpoint vs long-term store separation,
  append-only fork pattern, super-step boundaries as recovery units.
- **Human user invokes graphs vs autonomous cron.** Discounts `thread_id`
  as conversation cursor and indefinite-wait interrupts. Transfers stable
  execution identity (could be cycle / issue / PR ID), interrupt
  semantics for approval gates, checkpointed pause/resume even without
  interactive resumer, stream events as machine-consumed audit.
- **Python/TypeScript library vs Rust tools.** Discounts `TypedDict` /
  `Annotated` / decorators (don't transfer literally). Transfers the
  concepts of typed state, channel reducers, checkpoint IDs, parent
  links, pending writes, namespaces (language-independent); Rust has
  good enum/struct support; the deterministic-execution-vs-LLM-proposal
  split transfers; idempotence requirements are language-independent.
- **Short-to-medium-running apps vs hundreds of cycles.** Discounts
  per-thread history bloat, time-travel cost long-term, DB checkpointer
  durability vs git over very long horizons. Transfers the durable-
  execution warning becoming *more* important not less; append-only
  histories and parent pointers; short-term vs long-term memory split is
  especially relevant when histories grow; state versioning useful in
  long-running parallel work.
- **Database checkpointers vs git-tracked files and GitHub issues.**
  Discounts transaction semantics, primary-key lookup, Agent Server
  hiding persistence. Transfers checkpoint records as files/commits if
  schema is explicit; parent links as commit ancestry; pending writes as
  partially-completed parallel tasks; cross-thread Store vs thread
  checkpoints maps to repo-wide vs cycle-local state.
- **LangGraph as post-LangChain stateful pivot.** Discounts LangChain-
  shaped tool schemas, LangSmith product boundary affecting defaults.
  Transfers layering high-level agents on lower-level deterministic
  runtime; keeping model/tool integration separate from execution-state
  machinery; exposing low-level state operations rather than hiding all
  architecture behind agents.

**Patterns observed in LangGraph** (relevance evaluation deferred to
cross-system synthesis, gated on multi-system reading):
- Pregel/bulk-synchronous-parallel super-step execution model
- Plan/execution/update phases with parallel-write isolation within
  super-step
- State as named channels; per-channel reducers with overwrite default
- Multiple schemas (Input/Output/Overall/Private) around one internal
  state
- Checkpointing at super-step boundaries (graph-step rewind, not
  instruction-level)
- `thread_id` as required persistence cursor; `checkpoint_ns` namespace
  for subgraphs
- `StateSnapshot` containing values, next nodes, config, metadata,
  parent_config, and tasks
- Implementation-level checkpoint metadata (`channel_versions`,
  `versions_seen`, `pending_sends`) richer than docs summary
- Pending writes for successful siblings in failed super-steps
- Durability modes (`exit`/`async`/`sync`) exposing tradeoff explicitly
- Time travel as append-only fork (not destructive rollback)
- Replay re-executes nodes; not cache replay
- Short-term memory as thread-scoped checkpoints; long-term memory as
  cross-thread `Store` with namespace+key+value records
- Interrupts as checkpoint/resume; node restarts from beginning on
  resume (not language-runtime continuation)
- Subgraph composition as graphs-as-nodes (two patterns: wrapper for
  different schemas; direct for shared channels)
- Multiple orchestration patterns documented as first-class without one
  asserted canonical shape
- Explicit position against reflexive multi-agent decomposition
- Explicit non-goal: architectural opinionation ("LangGraph does not
  abstract prompts or architecture")
- Explicit anti-patterns enumerated (kitchen-sink avoidance, replay-as-
  cache mistake, interrupts-as-line-continuations mistake, etc.)
- Honest implementation-vs-marketing-claims subsection in research
  evaluation discipline

## Cross-system observations (preliminary)

Both required reads (openclaw, PAI) explicitly value:
- **Small core**, extension via plugins/skills/tools
- **Deterministic infrastructure** around probabilistic AI
- **Security posture** with strong defaults
- **Modular capability** that can be added without core changes

Differences:
- openclaw explicitly rejects agent-hierarchy frameworks; PAI lists
  Principle 14 ("Agent Personalities — specialized agents with
  unique voices") which gestures toward multi-agent.
- openclaw's "memory as a singleton plugin slot" suggests architectural
  conservatism on persistence; PAI elevates memory to a top-level
  principle.
- openclaw is operator-driven (user's commands run on user's host);
  PAI is goal-driven (system pursues user's goals).

These shouldn't yet inform Phase 2 candidates — multi-system reading
should establish what's idiosyncratic to each project vs what
cross-validates as a generalizable pattern.

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
| AutoGen | Microsoft's multi-agent framework; explicit conversation patterns between agents (relevant to my orchestrator + audit + Copilot setup) | Copilot research-only dispatch | Cycle 15 dispatched (PR #2763); cycle 16 integrated above |
| LangGraph | Production state-management for agents; explicit graph-based state | Copilot research-only dispatch or orchestrator-direct | Cycle 18 dispatched (issue [#2767](https://github.com/EvaLok/schema-org-json-ld/issues/2767), gpt-5.5, canonical cycle-15 procedure with anti-smuggling discipline pre-loaded) |
| Voyager | Long-running self-improving Minecraft agent; skill library accumulation | Orchestrator-direct (the paper is short) | Cycle 17 read above (orchestrator-direct, abstract + code) |
| Cognition Devin writeups | Autonomous coding agent; production deployment patterns | Orchestrator-direct (blog posts, not a repo) | Pending |
| Semantic Kernel | Microsoft's agent SDK; planner/skills split | Copilot research-only dispatch (lower priority) | Pending |
| Anthropic engineering posts | Claude Code, agent SDK, internal tooling experience | Orchestrator-direct | Pending |

### Cycle plan (provisional)

Cycle 14 (2026-04-28): openclaw + PAI first-pass; this document
created; no dispatch.

Cycle 15 (2026-04-28): AutoGen Copilot research-only dispatch
executed (issue #2762, PR #2763, gpt-5.5, canonical cycle-6
procedure). Adversarial re-read of this document found smuggling in
per-system "Provisional patterns to track" sections; renamed to
"Patterns observed in [system]" with v2-relevance framings stripped.

Cycle 16 (2026-04-28): AutoGen system entry added to this document
(navigation summary above; PR #2763 is evidence base). Voyager paper
read deferred to cycle 17+.

Cycle 17 (2026-04-28): Voyager orchestrator-direct read added above
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

Cycle 19+: dispatch options, in approximate priority order
(adjustable by cycle's actual capacity):
1. Cognition Devin writeups (orchestrator-direct; closest analog to v2's
   "AI does software-engineering work autonomously" target).
2. Semantic Kernel (lower priority; Copilot research-only or
   orchestrator-direct).
3. Anthropic engineering posts (orchestrator-direct).
4. Deeper second-pass orchestrator-direct on openclaw and PAI (cycle 16
   noted that the deliverable-size asymmetry biases cross-system
   synthesis toward the system with the richest evidence base; bringing
   openclaw and PAI to closer parity with AutoGen's deep-dive depth is
   an alternative use of cycles before committing to cross-system
   synthesis claims).

The dispatch sequence is tentative. Phase 1 reading priority should
adjust based on cycle capacity and any patterns that emerge as
load-bearing in cross-system observations.

### What Phase 1 will produce

A reference document (this file) capturing:
- Each system studied (architecture summary, anchoring caveats)
- Cross-system patterns (with anchoring discipline)
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
