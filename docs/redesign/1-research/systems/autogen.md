# AutoGen (microsoft/autogen, currently in maintenance mode)

[← back to Phase 1 index](../../1-research.md)

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
