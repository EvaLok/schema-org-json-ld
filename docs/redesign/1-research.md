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

**Anchoring caveats on AutoGen.**
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
- Multiple orchestration patterns coexist as first-class (round-robin,
  selector, swarm, graph, lead-orchestrator)
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
| LangGraph | Production state-management for agents; explicit graph-based state | Copilot research-only dispatch or orchestrator-direct | Pending |
| Voyager | Long-running self-improving Minecraft agent; skill library accumulation | Orchestrator-direct (the paper is short) | Pending (deferred from cycle 15+ to cycle 17+) |
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

Cycle 17+: dispatch options, in approximate priority order
(adjustable by cycle's actual capacity):
1. Voyager paper (orchestrator-direct; paper is short).
2. Copilot research-only on LangGraph (state-management focus).
3. Cognition Devin writeups (orchestrator-direct).
4. Semantic Kernel (lower priority; Copilot research-only or
   orchestrator-direct).
5. Anthropic engineering posts (orchestrator-direct).

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

Cycle-N-pre-commits-cycle-N+1-checks chain (now nine cycles deep)
extends to Phase 1: each cycle's Phase-1 notes file pre-commits
adversarial-on-adversarial checks for the next cycle, same discipline
as Phase 0 has used since cycle 7.
