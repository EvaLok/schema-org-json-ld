# LangGraph (langchain-ai/langgraph, active; post-LangChain stateful spinout)

[← back to Phase 1 index](../../1-research.md)

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
thrown only as process exceptions. Durability of these writes is a
tunable: `compile(durability="exit"|"async"|"sync")` exposes the
performance/durability tradeoff explicitly rather than hiding it as a
default.

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
execution stopped."

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
- Explicit anti-patterns enumerated (replay-as-cache mistake, interrupts-
  as-line-continuations mistake, etc.)
- Honest implementation-vs-marketing-claims subsection in research
  evaluation discipline
