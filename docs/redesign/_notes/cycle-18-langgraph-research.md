# Cycle 18: LangGraph architecture survey for Phase 1 cross-system reading

Cycle: redesign cycle 18

Sources read:

- Current LangGraph docs in `langchain-ai/docs`: `overview.mdx`, `graph-api.mdx`, `pregel.mdx`, `persistence.mdx`, `add-memory.mdx`, `use-time-travel.mdx`, `durable-execution.mdx`, `interrupts.mdx`, `streaming.mdx`, `use-subgraphs.mdx`, `workflows-agents.mdx`, and `choosing-apis.mdx`.
- LangGraph source in `langchain-ai/langgraph`: `graph/state.py`, `pregel/main.py`, `types.py`, checkpoint base/Postgres/SQLite files, store base, `tool_node.py`, and `chat_agent_executor.py`.
- Related current LangChain multi-agent docs in `langchain-ai/docs`: `multi-agent/index.mdx`, `subagents.mdx`, `handoffs.mdx`, `router.mdx`, and `custom-workflow.mdx`.

Scope note: the old `https://langchain-ai.github.io/langgraph/` site was unavailable from this environment. The LangGraph repository README now points to `docs.langchain.com/oss/python/langgraph/overview`; I used the corresponding `langchain-ai/docs` source files because those are the current docs source and are inspectable in GitHub.

## 1. Overall architecture and named primitives

LangGraph describes itself as low-level orchestration, not a prompt/agent abstraction. The current overview says: "LangGraph is very low-level, and focused entirely on agent **orchestration**" and "LangGraph does not abstract prompts or architecture" (`src/oss/langgraph/overview.mdx`). That is not just branding; the API surface is graph/runtime/state plumbing.

Core named primitives:

- `StateGraph`: the main graph-building API. The docs say: "The `StateGraph` class is the main graph class to use. This is parameterized by a user defined `State` object" (`graph-api.mdx`).
- Nodes: functions or runnables that read state and return partial state updates. The graph API docs summarize: "Nodes do the work, edges tell what to do next" (`graph-api.mdx`).
- Edges: control-flow links. Plain edges always route to the next node. Conditional edges route by a function that returns node names or `END`.
- State channels: keys in the state schema. The docs use "state channels" consistently: nodes read from and write to channels; reducers define how updates are applied.
- Reducers: per-channel merge functions. If absent, the channel uses overwrite/last-value behavior.
- `MessagesState` / `add_messages` / `MessagesValue`: prebuilt message-history state and reducer variants for chat histories.
- `START` and `END`: synthetic graph boundary nodes.
- `Command`: a return type for node functions that combines state update plus routing (`goto`), parent graph routing, and resume semantics in the interrupt path.
- `Send`: a primitive for dynamic parallelism; a conditional edge can return a list of `Send` objects to fan out work with per-send state.
- `Pregel`: the runtime produced by compiling a graph. The runtime docs state: "Compiling a `StateGraph` or creating an `@entrypoint` produces a `Pregel` instance that can be invoked with input" (`pregel.mdx`).
- Checkpointer / Store: persistence interfaces for thread-level snapshots and cross-thread memory.
- `Runtime`: a node-time object carrying context and store access.
- `ToolNode` and `create_react_agent`: prebuilt agent/tool helpers in `libs/prebuilt`, not the low-level core.

Composition shape:

1. Define a state schema.
2. Create a `StateGraph` builder.
3. Add nodes.
4. Add edges or conditional edges.
5. Compile.
6. Invoke, stream, inspect state, update state, or resume.

The hello-world example in the overview is deliberately small:

```python
from langgraph.graph import StateGraph, MessagesState, START, END

def mock_llm(state: MessagesState):
    return {"messages": [{"role": "ai", "content": "hello world"}]}

graph = StateGraph(MessagesState)
graph.add_node(mock_llm)
graph.add_edge(START, "mock_llm")
graph.add_edge("mock_llm", END)
graph = graph.compile()

graph.invoke({"messages": [{"role": "user", "content": "hi!"}]})
```

That is the entrypoint shape for local usage: a script or application calls `graph.invoke(...)`. The same compiled object also exposes `stream`, `astream`, state inspection, and update methods. LangGraph can also be deployed behind the LangGraph/LangSmith Agent Server, but the framework itself does not require a long-running server; it can be used as a library in a process.

The graph can be called in several runtime shapes:

- Synchronous invocation: `graph.invoke(input, config)`.
- Synchronous streaming: `graph.stream(input, stream_mode=..., version="v2")`.
- Async streaming: `graph.astream(...)`.
- State inspection: `graph.get_state(config)` and `graph.get_state_history(config)`.
- State mutation/fork: `graph.update_state(config, values=..., as_node=...)`.
- Interrupt resume: `graph.invoke(Command(resume=...), config)`.

Streaming is not an afterthought. The streaming docs say graphs expose `stream` and `astream` "to yield streamed outputs as iterators" and list modes including `values`, `updates`, `messages`, `custom`, `checkpoints`, `tasks`, and `debug` for Python (`streaming.mdx`). The v2 stream shape is a discriminated object with `type`, `ns`, and `data`; that matters for subgraphs and interrupt detection.

Pregel-style execution:

The docs explicitly anchor execution in Pregel/message passing: "LangGraph's underlying graph algorithm uses message passing to define a general program" and "the program proceeds in discrete `super-steps`" (`graph-api.mdx`). The key control-flow model is not "run node, mutate global state immediately." It is bulk-synchronous:

- At graph start, nodes are inactive.
- A node becomes active when it receives a new message/state on an incoming edge/channel.
- Active nodes run and emit updates.
- Parallel nodes in the same super-step do not observe each other's updates until the super-step ends.
- A node with no incoming messages votes to halt.
- Execution terminates when all nodes are inactive and no messages are in transit.

The runtime docs make the three phases explicit:

- **Plan**: choose actors whose subscribed channels were updated.
- **Execution**: execute selected actors in parallel "until all complete, or one fails, or a timeout is reached"; updates are invisible during this phase.
- **Update**: apply channel updates.

This Pregel model explains why reducers are core rather than optional. If two parallel nodes write the same state key in one super-step, a deterministic merge rule is required. Without a reducer, last-value/overwrite channels can reject or overwrite conflicting updates depending on channel type and implementation.

The code matches the docs at the broad level. `libs/langgraph/langgraph/graph/state.py` builds `CompiledStateGraph` on top of `CompiledGraph`, imports channel implementations such as `BinaryOperatorAggregate`, `LastValue`, and `EphemeralValue`, and wires schemas into channels. `libs/langgraph/langgraph/pregel/main.py` is the large runtime implementation behind invoke/stream/checkpoint integration. This is not a thin wrapper over LangChain agents; it is a graph runtime with LangChain `Runnable` interoperability.

One important architectural split: LangGraph has two APIs over the same runtime.

- Graph API: explicit nodes/edges/shared state, better for visualization, branching, parallel joins, shared state.
- Functional API: `@entrypoint`/`@task`, more natural for procedural code.

The choosing-APIs guide says both APIs "share the same underlying runtime" but target different use cases (`choosing-apis.mdx`). That is a real architectural stance: graph semantics are available without forcing every workflow to be declared as a graph.

## 2. State representation, persistence, and time-travel

This is LangGraph's densest area. The system is built around typed state, per-channel update rules, checkpointed execution, and thread-scoped histories.

### 2.1 State representation

The graph API docs define graph state as schema plus reducers:

> "The `State` consists of the schema of the graph as well as `reducer` functions which specify how to apply updates to the state." (`src/oss/langgraph/graph-api.mdx`)

Python state schema options:

- `TypedDict`: the main documented way.
- `dataclass`: recommended when defaults are needed.
- Pydantic `BaseModel`: supported for recursive validation, but the docs warn it is less performant than `TypedDict` or `dataclass`.

The docs also note a limitation: LangChain's higher-level `create_agent` factory "does not support Pydantic state schemas" (`graph-api.mdx`). That matters because the high-level agent API is built on LangGraph but does not expose every low-level state option.

TypeScript state shape differs:

- `StateSchema` for the root.
- Standard schemas such as Zod fields for simple last-value channels.
- `ReducedValue` for reducer-backed channels.
- `MessagesValue` for message lists.
- `UntrackedValue` for runtime-only/transient fields.

The JS docs say an `UntrackedValue` "should **never be checkpointed**" and warn: "Don't use `UntrackedValue` for data you need to persist across interrupts or time travel" (`graph-api.mdx`). This is an explicit boundary between execution-time state and persisted graph state.

State is a channel map, not a single blob with one update policy. Each key has a channel and merge behavior. Nodes return partial updates, not full state replacement. The docs emphasize: "The `Node` does not need to return the whole `State` schema - just an update" (`graph-api.mdx`).

LangGraph supports multiple schemas around one internal state:

- Input schema.
- Output schema.
- Overall/internal schema.
- Private schemas for internal node communication.

The graph API docs give an example where `InputState`, `OutputState`, `OverallState`, and `PrivateState` differ. It then says: "a node _can write to any state channel in the graph state_" and "nodes can also declare additional state channels" if the schema exists (`graph-api.mdx`). This is powerful but not purely local: a node signature can read one schema and write channels outside that read schema.

### 2.2 Reducers and concurrent updates

Reducers are per-key. The docs say:

> "Reducers are key to understanding how updates from nodes are applied to the `State`. Each key in the `State` has its own independent reducer function. If no reducer function is explicitly specified then it is assumed that all updates to that key should override it." (`graph-api.mdx`)

Python uses `typing.Annotated` to bind reducer functions:

```python
from typing import Annotated
from typing_extensions import TypedDict
from operator import add

class State(TypedDict):
    foo: int
    bar: Annotated[list[str], add]
```

TypeScript uses `ReducedValue`:

```typescript
const State = new StateSchema({
  foo: z.number(),
  bar: new ReducedValue(
    z.array(z.string()).default(() => []),
    { reducer: (x, y) => x.concat(y) }
  ),
});
```

For messages, LangGraph discourages naive `operator.add` when manual updates can happen. The docs explain that `add_messages` tracks message IDs and overwrites existing messages when updated; otherwise manual state updates would append duplicate message versions (`graph-api.mdx`). In TypeScript, `MessagesValue` wraps this reducer behavior.

Reducer semantics are the concurrency answer. In a super-step, multiple nodes can run in parallel and emit updates. Reducer-backed channels can merge those updates. Last-value channels are simpler but can be unsafe for parallel fan-in. The docs and code both treat channels as typed objects with update functions. The Pregel docs say a channel has "a value type, an update type, and an update function" (`pregel.mdx`).

The implementation in `state.py` imports and constructs channel classes including `BinaryOperatorAggregate` and `LastValue`. The checkpoint base stores `channel_versions` and `versions_seen`, which means checkpoints are not only a JSON dump of values; they also include per-channel version bookkeeping.

There is also an explicit bypass mechanism: `Overwrite` can bypass a reducer for a state value (`graph-api.mdx`). This is useful but dangerous in the obvious way: a channel that usually accumulates can be directly replaced by an update. The docs surface it as a special case, not normal reducer flow.

### 2.3 Checkpoints, threads, and state versions

The persistence docs start with the central claim:

> "LangGraph has a built-in persistence layer that saves graph state as checkpoints. When you compile a graph with a checkpointer, a snapshot of the graph state is saved at every step of execution, organized into threads." (`persistence.mdx`)

A thread is the durable identity of an execution/conversation/history. The docs define it as:

> "A thread is a unique ID or thread identifier assigned to each checkpoint saved by a checkpointer. It contains the accumulated state of a sequence of runs." (`persistence.mdx`)

When invoking with a checkpointer, the caller must specify `thread_id` in config:

```python
{"configurable": {"thread_id": "1"}}
```

The docs are blunt about why:

> "The checkpointer uses `thread_id` as the primary key for storing and retrieving checkpoints. Without it, the checkpointer cannot save state or resume execution after an interrupt" (`persistence.mdx`).

A checkpoint is not just the state value. The docs define it as a `StateSnapshot` and list fields:

- `values`: state channel values at the checkpoint.
- `next`: node names to execute next.
- `config`: includes `thread_id`, `checkpoint_ns`, and `checkpoint_id`.
- `metadata`: source (`input`, `loop`, `update`), writes, step.
- `created_at`.
- `parent_config`.
- `tasks`: task records with id, name, error, interrupts, and optionally subgraph state.

The implementation backs this with a `Checkpoint` typed dict in `libs/checkpoint/langgraph/checkpoint/base/__init__.py`. Fields include `channel_values`, `channel_versions`, `versions_seen`, `pending_sends`, and `updated_channels`. That is stronger than the public docs summary: the saved object includes causal/version metadata for channels and pending sends.

Checkpoints are made at super-step boundaries. The docs say:

> "LangGraph created a checkpoint at each **super-step** boundary" and "you can only resume execution from a checkpoint (i.e., a super-step boundary)" (`persistence.mdx`).

The typo "created" is in the docs; the intended claim is clear. It also sets a hard limit: time travel is not arbitrary instruction-level rewind. It is graph-step rewind.

Subgraphs add namespaces. `checkpoint_ns` identifies root graph vs subgraph. The docs specify `""` for the root graph and `"node_name:uuid"` for a subgraph, with nested subgraphs joined by `|` (`persistence.mdx`). This is a concrete storage shape for nested execution histories.

### 2.4 Checkpointer interface and backends

The abstract-ish base is `BaseCheckpointSaver` in `libs/checkpoint/langgraph/checkpoint/base/__init__.py`. Its docstring says:

> "Checkpointers allow LangGraph agents to persist their state within and across multiple interactions."

Important methods include `get_tuple`, `list`, `put`, `put_writes`, and async equivalents. `put_writes` matters for partial writes in a super-step. The base file defines `WRITES_IDX_MAP = {ERROR: -1, SCHEDULED: -2, INTERRUPT: -3, RESUME: -4}`, so special writes are persisted in the writes table/indexing model rather than only thrown as process exceptions.

Built-in or documented checkpointer backends include:

- In-memory: `InMemorySaver` / JS `MemorySaver`; suitable for development/testing, not production durability.
- Postgres: `langgraph-checkpoint-postgres`, `PostgresSaver`, `AsyncPostgresSaver`.
- SQLite: `langgraph-checkpoint-sqlite`; present in the repository under `libs/checkpoint-sqlite`.
- MongoDB: documented as `langgraph-checkpoint-mongodb` (`add-memory.mdx`).
- Redis: documented as `langgraph-checkpoint-redis` (`add-memory.mdx`).

The requested source list named Postgres, SQLite, Redis, and in-memory. The current docs also include MongoDB. The LangGraph monorepo includes Postgres and SQLite packages, but Redis and MongoDB appear as external packages referenced by docs, not as monorepo source under `libs/` at the time read.

The docs repeatedly distinguish dev from production. For short-term memory, they show `InMemorySaver`, then say: "In production, use a checkpointer backed by a database" (`add-memory.mdx`). For Redis/Postgres examples they note `setup()` must be called the first time.

### 2.5 Pending writes and failure recovery

Persistence includes pending writes. The persistence docs say:

> "When a graph node fails mid-execution at a given super-step, LangGraph stores pending checkpoint writes from any other nodes that completed successfully at that super-step. When you resume graph execution from that super-step you don't re-run the successful nodes." (`persistence.mdx`)

This is a stronger recovery model than simply checkpoint-at-end. It acknowledges parallel super-steps: one branch can fail while siblings completed. The implementation's `put_writes` methods are the storage hook for this.

Caveat: this does not make arbitrary side effects safe. Durable execution docs explicitly warn that replay does not resume from the same Python line; it replays from a starting point. They instruct users to wrap side effects/non-determinism in tasks and to make side effects idempotent. Quote:

> "When you resume a workflow run, the code does **NOT** resume from the **same line of code** where execution stopped" (`durable-execution.mdx`).

That is an important honesty point in the docs. The marketing claim "resume exactly where it left off" is only true at graph/task/checkpoint granularity, not at arbitrary interpreter instruction granularity.

Durability has modes:

- `exit`: persist only on successful exit, error, or interrupt; fastest, no mid-execution crash recovery.
- `async`: persist asynchronously while next step executes; possible loss if process crashes before write completes.
- `sync`: persist before next step; strongest and slowest.

The docs expose the tradeoff instead of pretending one mode is free.

### 2.6 Time travel: replay and fork

The time-travel docs define two operations:

- **Replay**: "Retry from a prior checkpoint."
- **Fork**: "Branch from a prior checkpoint with modified state to explore an alternative path." (`use-time-travel.mdx`)

Both work by resuming from a checkpoint. The docs state:

> "Nodes before the checkpoint are not re-executed (results are already saved). Nodes after the checkpoint re-execute, including any LLM calls, API requests, and interrupts (which may produce different results)." (`use-time-travel.mdx`)

They repeat the warning more directly:

> "Replay re-executes nodes—it doesn't just read from cache. LLM calls, API requests, and interrupts fire again and may return different results." (`use-time-travel.mdx`)

Replay procedure:

1. Run a graph with a checkpointer and thread ID.
2. Call `get_state_history(config)`.
3. Pick a checkpoint whose `next` points to the desired node.
4. Invoke with that checkpoint's config.

Fork procedure:

1. Pick a prior checkpoint.
2. Call `update_state` on that checkpoint with new values.
3. Invoke with `None` input and the returned fork config.

The docs warn:

> "`update_state` does **not** roll back a thread. It creates a new checkpoint that branches from the specified point. The original execution history remains intact." (`use-time-travel.mdx`)

This is important. LangGraph's "time travel" is append-only branching inside a thread history, not destructive rollback. State edits are checkpoint-creating operations that run through reducers and can specify `as_node` to determine successor routing. The docs say `update_state` is treated like a node update: reducer-backed channels accumulate rather than overwrite unless special overwrite behavior is used (`persistence.mdx`).

Interrupts during time travel are re-triggered. The docs say: "interrupts are always re-triggered during time travel" and the node pauses for a new `Command(resume=...)` (`use-time-travel.mdx`). That is correct if interrupts are considered external nondeterministic inputs; it is also a source of surprise if a user expects time travel to be deterministic replay.

### 2.7 Short-term and long-term memory storage shapes

LangGraph divides memory into two shapes.

Short-term memory:

- Thread-level persistence.
- Stored in graph state/checkpoints.
- Used for multi-turn conversations.
- Requires checkpointer and `thread_id`.

The memory docs say:

> "Short-term memory (thread-level persistence) enables agents to track multi-turn conversations" (`add-memory.mdx`).

Long-term memory:

- Cross-thread storage through `Store` / `BaseStore`.
- Namespaced key-value records.
- Optionally searchable.
- Used for user/application-level data across sessions.

The persistence docs explain why checkpointers are insufficient:

> "With checkpointers alone, we cannot share information across threads. This motivates the need for the `Store` interface." (`persistence.mdx`)

The storage shape in examples is namespace + key + value. A namespace can be `(<user_id>, "memories")`; the key is a unique memory ID; the value is a dictionary. Store methods include `put` and `search` in the docs. Production stores include `PostgresStore` and `RedisStore` (`persistence.mdx`, `add-memory.mdx`). The source file `libs/checkpoint/langgraph/store/base/__init__.py` defines the base store abstractions.

This separation is coherent: checkpoints are execution history; stores are application memory. It prevents every cross-session memory from being encoded as fake graph state inside one thread. It also adds a second persistence interface developers must reason about.

### 2.8 What the implementation appears to deliver vs marketing claims

Implemented and well-supported by code/docs:

- Typed graph state and reducers.
- Step-level checkpointing.
- Thread IDs and checkpoint IDs in configs.
- History retrieval.
- Append-only state updates/forks.
- Interrupt resume through checkpointed state.
- Multiple checkpointer/storage backends.
- Pregel-style bulk synchronous execution.

Claims that need qualification:

- "Resume exactly where they left off" is true only at checkpoint/task granularity. The durable execution docs themselves admit code does not resume from the same line.
- Time travel is not pure deterministic replay. Nodes after the checkpoint re-run and external calls can differ.
- Human-in-the-loop is not magically available; it requires a checkpointer and stable thread IDs. The docs make this explicit.
- Durable execution does not automatically make side effects idempotent. Users must wrap tasks or design side effects carefully.
- The Agent Server can hide persistence setup, but that moves complexity into LangSmith infrastructure rather than removing it.

## 3. Orchestration / planning patterns

LangGraph's own "workflows and agents" guide starts with a distinction:

- Workflows: "predetermined code paths" that operate in a certain order.
- Agents: dynamic systems that "define their own processes and tool usage" (`workflows-agents.mdx`).

Named patterns in the LangGraph/LangChain current docs include:

- Prompt chaining.
- Routing.
- Parallelization.
- Orchestrator-worker.
- Evaluator-optimizer.
- Agent/tool loop / ReAct-style agent.
- Subgraphs.
- Subagents/supervisor.
- Handoffs.
- Skills.
- Router.
- Custom workflows.

The core LangGraph guide leans heavily on deterministic workflow patterns before fully dynamic agents. That is not accidental. It shows prompt chaining with conditional quality gates, routing by classifier, parallel fan-out/fan-in, orchestrator-worker dynamic work splitting, and evaluator-optimizer loops.

Subgraphs are the direct composition primitive. The subgraph docs define a subgraph as "a graph that is used as a node in another graph" (`use-subgraphs.mdx`). They list reasons:

- Multi-agent systems.
- Reusing a set of nodes.
- Distributing development so teams can own subgraphs behind input/output schemas.

There are two subgraph communication patterns:

1. Call a subgraph inside a node when parent and subgraph have different state schemas. The wrapper maps parent state to subgraph input and maps subgraph output back.
2. Add a compiled subgraph as a node when parent and subgraph share state keys/channels.

Subgraph execution can stream with namespaces when `subgraphs=True`, and checkpoint namespaces identify nested graph snapshots. This makes subgraphs not just code reuse, but inspectable nested execution.

Multi-agent guidance is now mostly under LangChain docs, not the LangGraph core pages. The overview says LangGraph can be used standalone but integrates with LangChain; higher-level agents live in LangChain. The multi-agent overview states:

> "Multi-agent systems coordinate specialized components to tackle complex workflows. However, not every complex task requires this approach—a single agent with the right (sometimes dynamic) tools and prompt can often achieve similar results." (`src/oss/langchain/multi-agent/index.mdx`)

That is an explicit position against reflexive multi-agent decomposition.

The multi-agent docs list pattern choices:

- **Subagents**: a main agent coordinates subagents as tools; all routing passes through main agent.
- **Handoffs**: tools update state variables that trigger routing/configuration changes.
- **Skills**: specialized prompts/knowledge loaded on demand while a single agent stays in control.
- **Router**: classify input and route to specialized agents; synthesize outputs.
- **Custom workflow**: use LangGraph for bespoke deterministic/agentic mixes.

Supervisor pattern:

The subagents docs say: "a central main agent (often referred to as a **supervisor**) coordinates subagents by calling them as tools" (`subagents.mdx`). Key properties listed:

- Centralized control.
- No direct user interaction by subagents.
- Subagents invoked via tools.
- Parallel execution possible.

They distinguish supervisor from router:

> "The supervisor is a full agent that maintains conversation context and dynamically decides which subagents to call across multiple turns. A router is typically a single classification step that dispatches to agents without maintaining ongoing conversation state." (`subagents.mdx`)

Handoff pattern:

The handoffs docs define behavior as state-driven:

> "tools update a state variable (e.g., `current_step` or `active_agent`) that persists across turns, and the system reads this variable to adjust behavior" (`handoffs.mdx`).

This pattern uses `Command(update=...)` from tools and requires a matching `ToolMessage` for LLM tool-call protocol correctness.

ReAct agent:

The prebuilt package includes `create_react_agent` in `libs/prebuilt/langgraph/prebuilt/chat_agent_executor.py`. The graph is a loop between model calls and tools until no tool calls remain. It is a convenience around the lower-level graph primitives, not the runtime's central abstraction. The overview recommends LangChain's agents for "higher-level abstraction" and says those agents provide "prebuilt architectures for common LLM and tool-calling loops" (`overview.mdx`).

Plan-and-execute / swarm:

I did not find current first-class core LangGraph docs named exactly "plan-and-execute" or "swarm" in the current `src/oss/langgraph` docs. The ecosystem points to Deep Agents for planning/subagents/filesystems in the README, and to LangChain multi-agent patterns. Treat any "LangGraph ships swarm" claim as ecosystem/package-level unless a specific current package is cited. The core framework supports the graph shapes needed for those patterns, but support-by-expressibility is not the same as a blessed primitive.

Position on single vs multi-agent:

The docs explicitly say not every complex task needs multi-agent, and single-agent with dynamic tools/prompt can suffice. The pattern docs frame multi-agent as useful for context management, distributed development, and parallelization, not as a default.

## 4. Failure handling and recovery

LangGraph has several failure/recovery layers. They are not one unified "failure handling" feature.

### 4.1 Runtime failure in Pregel steps

The Pregel docs say execution runs selected actors in parallel "until all complete, or one fails, or a timeout is reached" (`pregel.mdx`). If a node fails, the super-step is not just silently skipped. The persistence docs say checkpoint pending writes from successful sibling nodes can be stored, so resume does not rerun successful nodes in the failed super-step.

This only works when a checkpointer exists. Without persistence, a process failure is just a process failure.

### 4.2 Retry policies

LangGraph exposes retry policy at node level. The graph API docs reference `retry_policy` on `add_node`, and `types.py` defines `RetryPolicy` with fields including initial interval, backoff factor, max interval, max attempts, jitter, and retry-on exception classifier. Node-level retry is a deterministic runtime feature; it is not an LLM self-correction loop.

Caveat: retrying a node with side effects can duplicate effects unless the node/task is idempotent. Durable execution docs warn users to wrap side effects in tasks and use idempotency keys.

### 4.3 Conditional/fallback edges

LangGraph does not have a single magic "fallback edge" primitive in the way some workflow engines do. The typical pattern is conditional routing:

- A node writes status/error information into state.
- A conditional edge inspects state and routes to recovery, retry, escalation, or `END`.
- A node can return `Command(goto=...)` directly for dynamic routing.

The choosing-APIs guide shows a `should_continue` router that ends when `retry_count > 3`, otherwise routes to different nodes (`choosing-apis.mdx`). That is a pattern, not automatic malformed-output handling.

### 4.4 Malformed LLM output

Malformed output handling depends on the node implementation or LangChain model/tool wrappers. LangGraph itself treats nodes as functions/runnables. It can retry a node or route based on state, but it does not inherently understand "malformed JSON" unless the node/tooling raises or records it.

For structured output, the docs rely on LangChain structured-output components. If structured output parsing raises, it is a node failure unless caught. If a validation node catches it and writes failure state, conditional edges can route.

### 4.5 Rate limits

Rate limits are similarly external. A model/tool call can raise a rate-limit exception; LangGraph retry policies can retry if configured to match the exception. The framework does not document a global rate-limit scheduler in the core graph docs. Production deployment/Agent Server may add operational features, but that is outside the low-level library's core guarantees.

### 4.6 Loop limits

LangGraph has recursion/step limits. `types.py` and `pregel/main.py` reference `recursion_limit` and `GraphRecursionError`. This is the guard when a graph keeps cycling and exceeds the configured maximum number of steps. It is a hard runtime error, not a semantic loop detector.

### 4.7 Human-in-the-loop interrupts

Interrupts are the most distinctive recovery/control feature. The interrupts docs define them as:

> "Interrupts allow you to pause graph execution at specific points and wait for external input before continuing." (`interrupts.mdx`)

Mechanism:

1. A node calls `interrupt(payload)`.
2. LangGraph saves state via the persistence layer.
3. The payload is surfaced to the caller.
4. Execution waits indefinitely.
5. Caller resumes by invoking the graph with `Command(resume=...)` and the same thread ID.
6. The resume value becomes the return value of the `interrupt()` call inside the node.

The docs list requirements:

- A checkpointer.
- A thread ID.
- JSON-serializable payload.

They also warn about resume semantics:

> "The node restarts from the beginning of the node where the `interrupt` was called when resumed, so any code before the `interrupt` runs again" (`interrupts.mdx`).

That is an important caveat. Interrupts are not continuations in the language-runtime sense. They are checkpoint/resume/replay at node granularity.

Multiple parallel interrupts can happen in one invocation. The docs show two parallel nodes both calling `interrupt()`, producing two interrupt IDs. Resume can pass a map from interrupt ID to answer. That is a concrete composition story for parallel HITL.

Interrupts compose with time travel by re-triggering. That is correct but can be costly: if the interrupted node did work before interrupting, that pre-interrupt work runs again unless wrapped or idempotent.

### 4.8 Review/edit state

The interrupts docs include review/edit state patterns: pause, let a human modify state or tool calls, then continue. This uses checkpointed state and `update_state` or `Command(resume=...)` depending on the pattern. The trust boundary is the caller/human editing persisted graph state.

## 5. Tool / skill integration model

LangGraph itself treats nodes as functions/runnables. Tools are mostly LangChain tools integrated through prebuilt nodes/agents.

### 5.1 Tools through LangChain model binding

The workflows guide shows defining a Python function tool and binding it to a model:

```python
def multiply(a: int, b: int) -> int:
    return a * b

llm_with_tools = llm.bind_tools([multiply])
```

The TypeScript docs use `tool(...)` from `langchain` and `llm.bindTools([multiply])` (`workflows-agents.mdx`). This is LangChain's tool-calling interface. LangGraph can host it inside a node.

### 5.2 ToolNode

`ToolNode` is the prebuilt execution node for tool calls. The source docstring in `libs/prebuilt/langgraph/prebuilt/tool_node.py` describes features:

- Tool execution from LLM tool calls.
- State injection for tools that need graph state.
- Store injection for tools that need persistent storage.
- Command-based state updates for advanced control flow.

The prebuilt node reads the last AI message's tool calls and invokes matching tools. It can run multiple tool calls and return `ToolMessage` results. This is where actual tool execution happens in the prebuilt ReAct architecture: model node emits tool calls; tool node executes tools; graph routes back to model or ends.

### 5.3 Bound-tool nodes vs custom nodes

There are at least three levels:

1. Custom node: any function/runnable. It can call APIs, tools, models, filesystem, etc. LangGraph only sees returned state updates or exceptions.
2. Model node with bound tools: the model is configured to produce tool calls, but does not execute tools itself.
3. `ToolNode`: executes the tool calls and writes tool responses into messages/state.

This separation is a useful trust boundary: the LLM proposes tool calls; deterministic code executes the tool calls. But it is only as safe as the tools and `ToolNode` configuration. A custom node can ignore that boundary and let the model output drive arbitrary behavior.

### 5.4 Tool decorators and schemas

Python examples use `@tool` from `langchain.tools`. TypeScript examples use `tool` with a Zod schema. Tool descriptions and schemas are used by models for tool-calling. LangGraph does not invent a separate tool schema language.

### 5.5 Injected state and store

`ToolNode` supports `InjectedState` and `InjectedStore` in source. These allow tools to receive graph state or persistent store access without the model supplying those arguments. This is an important trust-boundary feature: model-visible tool schema need not expose internal state/store parameters.

However, injected access also means a tool can mutate/read broader system memory if given store access. LangGraph provides the injection mechanism; application code must decide which tools are allowed to receive it.

### 5.6 Skills

The current multi-agent docs use "Skills" as a pattern where a single agent loads specialized prompts/knowledge on demand while staying in control (`multi-agent/index.mdx`, `skills.mdx`). This is not the same as this repository's Claude Code skills. LangChain's skill pattern is context loading, not necessarily a separate executable capability. It is documented under LangChain multi-agent, not core LangGraph.

## 6. Anti-patterns and explicit non-goals

The docs contain several explicit boundaries.

### 6.1 LangGraph is low-level and does not abstract architecture

The overview says:

> "LangGraph is very low-level, and focused entirely on agent **orchestration**" (`overview.mdx`).

And:

> "LangGraph does not abstract prompts or architecture" (`overview.mdx`).

If you want a higher-level abstraction, the docs recommend LangChain agents. That is a non-goal: LangGraph is not trying to decide the agent architecture for you.

### 6.2 Do not use LangGraph directly when a higher-level agent is enough

The overview says users who are "just getting started with agents or want a higher-level abstraction" should use LangChain's agents (`overview.mdx`). The multi-agent overview likewise says a single agent with the right tools/prompt can often achieve similar results to multi-agent.

This is a real anti-pattern: unnecessary graph/multi-agent ceremony.

### 6.3 Pydantic state schema support is lower-level only

The graph API supports Pydantic state schemas but notes Pydantic is less performant than `TypedDict`/dataclass, and `create_agent` does not support Pydantic state schemas. That discourages using Pydantic state casually in high-level agents.

### 6.4 Do not skip compilation

The graph API warning is absolute:

> "You **MUST** compile your graph before you can use it." (`graph-api.mdx`)

Compilation performs structural checks and attaches runtime args such as checkpointers and breakpoints.

### 6.5 Do not persist transient runtime objects

TypeScript `UntrackedValue` exists for values that "should **never be checkpointed**" (`graph-api.mdx`). The docs warn not to use it for anything needed across interrupts/time travel. The anti-pattern is putting unserializable connections/caches into persistent state.

### 6.6 Do not treat interrupts as line-level continuations

The interrupt docs explicitly warn that the node restarts from the beginning on resume. Any code before `interrupt()` runs again. Therefore, placing side effects before interrupts without idempotence/tasks is an anti-pattern.

### 6.7 Do not pass the wrong Command shape to invoke/stream

The interrupts docs warn:

> "`Command(resume=...)` is the **only** `Command` pattern intended as input to `invoke()`/`stream()`. The other `Command` parameters (`update`, `goto`, `graph`) are designed for returning from node functions." (`interrupts.mdx`)

This is precise API discipline.

### 6.8 Do not assume replay is cache replay

The time-travel docs warn replay re-executes nodes and external calls. The anti-pattern is treating replay as a read-only audit of prior results.

### 6.9 Do not assume checkpointers make side effects safe

Durable execution docs repeatedly say side effects and nondeterminism must be wrapped in tasks/nodes and designed idempotently. The anti-pattern is unguarded API calls/file writes inside replayable code.

### 6.10 API evolution: v2 streaming format

Streaming docs say v2 unified stream output requires LangGraph >= 1.1 and contrast v1 default output that changes shape based on options. This is not exactly a deprecation, but it is a "use this instead" direction: v2 stream parts are typed and uniform.

### 6.11 Spun-out / post-LangChain boundary

The README says LangGraph is built by LangChain Inc. but "can be used without LangChain." The overview says LangChain components are commonly used in docs but not required. This is the post-LangChain boundary: orchestration runtime separated from high-level model/tool integrations.

I did not find a clean current doc saying "LangGraph removed X from LangChain" in the source files read. The observable boundary is instead product layering:

- LangGraph: low-level runtime, state, persistence, interrupts, streaming.
- LangChain agents: prebuilt architectures on top of LangGraph.
- LangSmith/Agent Server: deployment/observability/persistence infrastructure.

## 7. Anchoring caveats

These are transfer caveats, not recommendations. Each lists what the difference discounts and what still transfers despite the difference.

### 7.1 Library for stateful applications vs specific autonomous GitHub orchestrator

Difference: LangGraph is a general-purpose library/runtime. The redesign target is one autonomous orchestrator operating in a public GitHub repository.

Discounts:

- LangGraph's API ergonomics for arbitrary app developers do not directly answer what one repo-specific orchestrator should look like.
- Agent Server assumptions about app deployment and request/response threads do not directly fit a git/GitHub-native process.
- Generic graph visualization/team-development benefits may be less important than auditability in git history.

Transfers:

- Explicit state schemas and per-field reducers are not app-specific.
- Checkpoint vs long-term store separation is a general state-shape distinction.
- Append-only fork rather than destructive rollback is a general recovery pattern.
- Super-step boundaries as recovery/resume units are generalizable.

### 7.2 Human user invokes graphs vs autonomous cron

Difference: LangGraph examples usually have a human/user request initiating a graph invocation. The redesign orchestrator runs autonomously on a schedule with minimal human-in-the-loop.

Discounts:

- `thread_id` as "conversation cursor" maps imperfectly when work is cycle-based and issue/PR-based rather than user-chat-based.
- Interrupts that wait indefinitely for a user can deadlock an autonomous cron if not paired with polling/escalation.
- UX-driven streaming modes are less central when no human watches the run live.

Transfers:

- A stable execution identity is still needed; it may be cycle ID, issue ID, PR ID, or task ID rather than user conversation ID.
- Interrupt semantics still matter for human approval/review gates.
- Checkpointed pause/resume remains useful even if the resumer is a later cron run rather than an interactive user.
- Stream/checkpoint events can become machine-consumed audit events rather than UI events.

### 7.3 Python/TypeScript library vs Rust tools

Difference: LangGraph is Python-first with TypeScript support. The redesign uses Rust for deterministic tools.

Discounts:

- `TypedDict`, Pydantic, `Annotated`, `StateSchema`, and LangChain `Runnable` APIs do not transfer literally to Rust.
- Python decorators such as `@tool`, `@entrypoint`, and `@task` do not map directly.
- LangGraph checkpointer packages are Python/JS libraries, not Rust crates.

Transfers:

- The concepts of typed state, channel reducers, checkpoint IDs, parent checkpoint links, pending writes, and namespaces are language-independent.
- Rust is well-suited to explicit enums/structs for state snapshots and reducer behavior.
- The split between deterministic tool execution and LLM-proposed tool calls transfers across languages.
- Idempotence requirements for side effects are independent of implementation language.

### 7.4 Short-to-medium-running apps vs hundreds of cycles

Difference: LangGraph supports long-running workflows, but typical examples are conversations or workflows spanning minutes/days/weeks. The redesign targets work over hundreds of cycles.

Discounts:

- Per-thread checkpoint histories may become huge over hundreds of cycles unless retention/compaction exists.
- Time-travel by replaying nodes after a checkpoint may be expensive or semantically stale months later.
- Human-readable state history in a database may be less durable/auditable than git/GitHub artifacts over very long horizons.

Transfers:

- Durable execution's warning about deterministic replay becomes more important, not less.
- Append-only histories and parent pointers are useful for long-horizon audit.
- Separating short-term execution state from long-term memory is especially relevant when histories get large.
- State versioning and checkpoint namespaces are useful in nested/parallel long-running work.

### 7.5 Database checkpointers vs git-tracked files and GitHub issues

Difference: LangGraph deployments typically use Postgres/Redis/SQLite/Mongo/in-memory checkpointers and stores. The redesign's state lives in git-tracked files and GitHub-resident issues/PRs.

Discounts:

- Transaction semantics, row locks, async writes, and setup methods from database checkpointers do not directly apply to git commits.
- `thread_id` primary-key lookup is simpler than reconciling GitHub issue comments, labels, PR states, and git commits.
- Agent Server hiding persistence infrastructure does not transfer to a repo where persistence is the visible artifact.

Transfers:

- Checkpoint records can be represented as files/commits if the schema is explicit.
- Parent checkpoint links map naturally to commit ancestry or explicit parent IDs.
- Pending writes concept transfers to partially completed parallel tasks if represented explicitly.
- Cross-thread `Store` vs thread checkpoints maps to repo-wide state vs task/cycle-local state.

### 7.6 LangGraph as post-LangChain stateful pivot

Difference: LangGraph is a reaction to LangChain pain points: high-level chains/agents needed lower-level durable orchestration, state, streaming, and HITL.

Discounts:

- Some design choices are shaped by preserving LangChain model/tool interoperability and `Runnable` conventions.
- Tool schemas and message reducers are optimized for chat-model/tool-call ecosystems.
- The product boundary with LangSmith deployment/observability affects docs and defaults.

Transfers:

- Layering high-level agents on a lower-level deterministic runtime is a general pattern.
- Keeping model/tool integrations separate from execution state machinery is transferable.
- Exposing low-level state operations instead of hiding all architecture behind agents is transferable.
- The warning that high-level convenience does not expose every low-level state option is broadly relevant.

## Patterns observed in LangGraph
## Patterns observed in LangGraph

This section intentionally lists patterns without evaluating redesign relevance.

- Low-level graph runtime under higher-level agent helpers.
- Explicit graph compile step before invocation.
- Nodes as functions/runnables returning partial state updates.
- Edges and conditional edges as routing declarations.
- Pregel/Bulk Synchronous Parallel super-steps.
- Parallel nodes not observing same-step writes until update phase.
- State represented as named channels.
- Per-channel reducers, with overwrite as the default.
- Binary-operator reducers for accumulation.
- Message-aware reducer that updates by message ID.
- Explicit reducer bypass with `Overwrite`.
- Separate input, output, internal, and private schemas.
- Runtime-only uncheckpointed values in TypeScript.
- `Command` for node update-plus-routing.
- `Command(resume=...)` for interrupt resume input.
- `Send` for dynamic fan-out.
- Node-level retry policy and recursion/step limit errors.
- Checkpointer attached at compile time.
- `thread_id` as required persistence cursor.
- Checkpoints saved at super-step boundaries.
- `StateSnapshot` containing values, next nodes, config, metadata, parent config, and tasks.
- `checkpoint_id`, `checkpoint_ns`, parent checkpoint links, and channel version metadata.
- Pending writes for successful siblings in failed super-steps.
- In-memory, Postgres, SQLite, Redis, and MongoDB checkpointer options across repo/docs.
- Durability modes: `exit`, `async`, `sync`.
- Durable execution requiring deterministic/idempotent design.
- `@task` for memoized/replay-safe sub-operations.
- Replay and fork from prior checkpoints.
- Forks appending checkpoints rather than modifying old history.
- Time travel re-executing later nodes rather than reading cache.
- Interrupts implemented through checkpoint/resume.
- Interrupt resume requiring same thread ID.
- Nodes restarting from beginning after interrupt resume.
- Multiple simultaneous interrupt IDs with resume maps.
- Short-term memory as thread-level checkpointed state.
- Long-term memory as cross-thread Store.
- Store namespace plus key-value memory records.
- Runtime object carrying context/store into nodes.
- Streaming as iterator/async iterator.
- v2 stream part shape with `type`, `ns`, and `data`.
- Subgraph as graph-used-as-node.
- Subgraph wrapper node for different schemas.
- Compiled subgraph directly as node for shared channels.
- Workflow patterns: prompt chaining, routing, parallelization, orchestrator-worker, evaluator-optimizer.
- Agent pattern as dynamic tool-use loop.
- Prebuilt ReAct-style agent helper.
- Tool calls proposed by model and executed by ToolNode.
- LangChain `@tool` / `tool(...)` schema pattern.
- Model tool binding for tool-call generation.
- Tool state injection and store injection.
- Tools returning `Command` for state/routing changes.
- ToolMessage requirement when tool updates message history.
- Supervisor/subagents pattern: main agent invokes subagents as tools.
- Handoffs pattern: tools update state variable controlling active behavior.
- Router pattern: classify then dispatch.
- Skills pattern: load specialized context on demand.
- Custom workflow pattern: mix deterministic logic and agent behavior in LangGraph.
- Explicit guidance that multi-agent is not always necessary.
- Explicit split between low-level LangGraph and higher-level LangChain agents.
