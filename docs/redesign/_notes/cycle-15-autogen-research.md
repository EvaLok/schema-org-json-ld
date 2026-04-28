# Cycle 15 AutoGen research notes

Cycle: redesign cycle 15
Source task: external Phase 1 research dispatch on Microsoft's AutoGen
Primary sources read:
- `microsoft/autogen` `README.md` at SHA `25f7cc162ae92c3988966d85cce173ff6df48020`
- `microsoft/autogen` `python/docs/src/user-guide/agentchat-user-guide/index.md`
- `microsoft/autogen` `python/docs/src/user-guide/agentchat-user-guide/tutorial/agents.ipynb`
- `microsoft/autogen` `python/docs/src/user-guide/agentchat-user-guide/tutorial/teams.ipynb`
- `microsoft/autogen` `python/docs/src/user-guide/agentchat-user-guide/tutorial/termination.ipynb`
- `microsoft/autogen` `python/docs/src/user-guide/agentchat-user-guide/tutorial/state.ipynb`
- `microsoft/autogen` `python/docs/src/user-guide/agentchat-user-guide/migration-guide.md`
- `microsoft/autogen` `python/docs/src/user-guide/agentchat-user-guide/magentic-one.md`
- `microsoft/autogen` `python/docs/src/user-guide/core-user-guide/index.md`
- `microsoft/autogen` `python/docs/src/user-guide/core-user-guide/core-concepts/*.md`
- `microsoft/autogen` `python/docs/src/user-guide/core-user-guide/components/{tools,model-context,command-line-code-executors}.ipynb`

Scope note: I read the current main-branch documentation in the AutoGen repo, not just the public landing page. The docs now describe AutoGen as in maintenance
mode and recommend Microsoft Agent Framework for new users. That status is relevant evidence, not a command to follow.

## 1. Overall architecture and named primitives

AutoGen is not one monolithic orchestrator. It is a layered framework for building multi-agent applications.

The README states the top-level identity plainly:

> "AutoGen is a framework for creating multi-agent AI applications that can act autonomously or work alongside humans." (`microsoft/autogen` `README.md`)

The same README says AutoGen is now in maintenance mode:

> "AutoGen is now in maintenance mode. It will not receive new features or enhancements and is community managed going forward." (`README.md`)

That matters because current docs are partly a stable artifact and partly a migration signpost. Architectural claims should be read as describing an existing
framework, not an actively expanding design frontier.

The named layers in the README are:

- Core API.
- AgentChat API.
- Extensions API.
- AutoGen Studio.
- AutoGen Bench.

The README describes the layer split this way:

> "The autogen framework uses a layered and extensible design. Layers have clearly divided responsibilities and build on top of layers below." (`README.md`)

It then assigns responsibilities:

> "Core API implements message passing, event-driven agents, and local and distributed runtime for flexibility and power." (`README.md`)

> "AgentChat API implements a simpler but opinionated API for rapid prototyping. This API is built on top of the Core API and is closest to what users of v0.2
> are familiar with and supports common multi-agent patterns such as two-agent chat or group chats." (`README.md`)

> "Extensions API enables first- and third-party extensions continuously expanding framework capabilities. It support specific implementation of LLM clients
> (e.g., OpenAI, AzureOpenAI), and capabilities such as code execution." (`README.md`)

AgentChat is the high-level entry point. Its docs say:

> "AgentChat is a high-level API for building multi-agent applications. It is built on top of the autogen-core package. For beginner users, AgentChat is the
> recommended starting point." (`python/docs/src/user-guide/agentchat-user-guide/index.md`)

Core is the lower-level event framework. Its docs say:

> "AutoGen core offers an easy way to quickly build event-driven, distributed, scalable, resilient AI agent systems. Agents are developed by using the Actor
> model." (`python/docs/src/user-guide/core-user-guide/index.md`)

The Core docs define an agent generically:

> "An agent is a software entity that communicates via messages, maintains its own state, and performs actions in response to received messages or changes in
> its state." (`core-concepts/agent-and-multi-agent-application.md`)

They also name multi-agent applications as the composition unit:

> "These systems, composed of multiple interacting agents, are referred to as multi-agent applications." (`core-concepts/agent-and-multi-agent-application.md`)

Core's runtime primitives are agent runtimes, direct messages, broadcast messages, topics, subscriptions, agent IDs, agent types, and agent keys.

Agent identity is explicit:

> "Agent ID uniquely identifies an agent instance within an agent runtime -- including distributed runtime. It is the 'address' of the agent instance for
> receiving messages. It has two components: agent type and agent key." (`core-concepts/agent-identity-and-lifecycle.md`)

Lifecycle is runtime-managed, not app-new-object-everywhere:

> "When a runtime delivers a message to an agent instance given its ID, it either fetches the instance, or creates it if it does not exist." (`core-
> concepts/agent-identity-and-lifecycle.md`)

The docs are honest that one lifecycle feature is not implemented:

> "The runtime is also responsible for 'paging in' or 'out' agent instances to conserve resources and balance load across multiple machines. This is not
> implemented yet." (`core-concepts/agent-identity-and-lifecycle.md`)

That is a useful example of an aspirational architecture claim separated from implemented behavior.

Message routing has two main forms:

> "There are two ways for runtime to deliver messages, direct messaging or broadcast. Direct messaging is one to one: the sender must provide the recipient's
> agent ID. On the other hand, broadcast is one to many and the sender does not provide recipients' agent IDs." (`core-concepts/topic-and-subscription.md`)

Broadcast is implemented with topics and subscriptions:

> "In essence, agent runtime implements a publish-subscribe model through its broadcast API: when publishing a message, the topic must be specified." (`core-
> concepts/topic-and-subscription.md`)

AgentChat's named primitives differ from old AutoGen 0.2 names. Current docs center `AssistantAgent`, `BaseChatAgent`, `UserProxyAgent`, `CodeExecutorAgent`,
`RoundRobinGroupChat`, `SelectorGroupChat`, `MagenticOneGroupChat`, `Swarm`, `GraphFlow`, `TerminationCondition`, `TaskResult`, messages, tools, model clients,
model contexts, memory, workbenches, and component serialization.

The issue prompt names `ConversableAgent` and `GroupChat`. Those are important historically, but the current docs treat them primarily as migration artifacts.
The migration guide says v0.4 is a rewrite:

> "Based on that feedback, we built AutoGen v0.4, a from-the-ground-up rewrite adopting an asynchronous, event-driven architecture to address issues such as
> observability, flexibility, interactive control, and scale." (`agentchat-user-guide/migration-guide.md`)

For `ConversableAgent`, the migration guide shows old v0.2 code using `register_reply`, then says:

> "Rather than guessing what the reply_func does, all its parameters, and what the position should be, in v0.4, we can simply create a custom agent and
> implement the on_messages, on_reset, and produced_message_types methods." (`agentchat-user-guide/migration-guide.md`)

For old `GroupChat`, the migration guide says:

> "In v0.2, you need to create a GroupChat class and pass it into a GroupChatManager, and have a participant that is a user proxy to initiate the chat."
> (`migration-guide.md`)

The v0.4 replacement in the same section is `RoundRobinGroupChat` or `SelectorGroupChat`:

> "In v0.4, you can use the RoundRobinGroupChat to achieve the same behavior." (`migration-guide.md`)

Entrypoint shape: mostly application code. The README quickstart is an `asyncio.run(main())` script that creates a model client, creates an `AssistantAgent`,
runs a task, and closes the client. AgentChat team examples also use scripts. Core can run single-process runtimes or distributed runtimes. Studio is a GUI
server-ish developer tool, but the framework itself is a library embedded into applications.

The Core architecture docs distinguish standalone and distributed runtime:

> "Standalone runtime is suitable for single-process applications where all agents are implemented in the same programming language and running in the same
> process." (`core-concepts/architecture.md`)

> "Distributed runtime is suitable for multi-process applications where agents may be implemented in different programming languages and running on different
> machines." (`core-concepts/architecture.md`)

So AutoGen's default entrypoint is not a daemon owning all work by itself. It is usually a Python app or notebook, with optional distributed runtime underneath.

## 2. Multi-turn conversation and state representation

AutoGen state lives in several places depending on layer:

- Agent objects keep internal state.
- AgentChat teams keep a message thread / team state.
- Core model contexts store chat-completion messages.
- Core runtimes manage agent identity and lifecycle.
- Optional persistence writes dictionaries to files or databases.
- Memory components can add retrieval-like state, but the core conversation
  state is still messages and component state.

AgentChat's agent tutorial explicitly says agents are stateful:

> "Agents are expected to be stateful and this method is expected to be called with new messages, not complete history." (`agentchat-user-
> guide/tutorial/agents.ipynb`)

It also warns that `run` mutates internal history:

> "It is important to note that BaseChatAgent.run will update the internal state of the agent -- it will add the messages to the agent's message history."
> (`agents.ipynb`)

That is a strong design choice: callers should not repeatedly resend complete history to a stateful agent. They send the next messages; the agent remembers.

Teams are stateful too. The teams tutorial says:

> "Teams are stateful and maintains the conversation history and context after each run, unless you reset the team." (`agentchat-user-
> guide/tutorial/teams.ipynb`)

The same tutorial distinguishes reset vs resume:

> "It is usually a good idea to reset the team if the next task is not related to the previous task. However, if the next task is related to the previous task,
> you don't need to reset and you can instead resume the team." (`teams.ipynb`)

State persistence is explicit but not magic. The state tutorial opens with:

> "In many cases, it is useful to save the state of these components to disk and load them back later. This is particularly useful in a web application where
> stateless endpoints respond to requests and need to load the state of the application from persistent storage." (`agentchat-user-guide/tutorial/state.ipynb`)

For an `AssistantAgent`, the state tutorial notes:

> "For AssistantAgent, its state consists of the model_context." (`state.ipynb`)

For teams:

> "When we call save_state on a team, it saves the state of all the agents in the team." (`state.ipynb`)

The example printed in the state tutorial shows concrete state shape: a `TeamState` dictionary containing `agent_states`, a `RoundRobinManagerState` with
`message_thread`, `current_turn`, and `next_speaker_index`, and `AssistantAgentState` with `llm_messages`. That is not just a high-level claim; the saved
artifact is message-centric.

Persistence is dictionary-based:

> "State is a dictionary that can be serialized to a file or written to a database." (`state.ipynb`)

Long-context handling is delegated to model contexts. The migration guide says:

> "In v0.4, we introduce the ChatCompletionContext base class that manages message history and provides a virtual view of the history." (`migration-guide.md`)

The agent tutorial lists available contexts:

> "By default, AssistantAgent uses the UnboundedChatCompletionContext which sends the full conversation history to the model. To limit the context to the last n
> messages, you can use the BufferedChatCompletionContext. To limit the context by token count, you can use the TokenLimitedChatCompletionContext."
> (`agents.ipynb`)

The Core model-context docs define the component narrowly:

> "A model context supports storage and retrieval of Chat Completion messages. It is always used together with a model client to generate LLM-based responses."
> (`core-user-guide/components/model-context.ipynb`)

This means AutoGen's context management is not primarily an autonomous memory policy. It is an application-selectable view over message history, plus optional
memory/workbench components.

There is a clear state/context distinction:

- State can be saved and restored as component dictionaries.
- Model context is the LLM-visible chat history view.
- Team message thread is coordination state.
- Runtime identity/lifecycle maps messages to agent instances.

The docs do not present a single global state file as the center of the system. State is component-local and serialized through component APIs.

## 3. Orchestration / planning patterns

AutoGen does not take one single position that all orchestration must be a manager hierarchy, a peer chat, or a workflow. It exposes primitives and preset
patterns.

Core explicitly calls itself unopinionated:

> "AutoGen core is designed to be an unopinionated framework that can be used to build a wide variety of multi-agent applications. It is not tied to any
> specific agent abstraction or multi-agent pattern." (`core-concepts/application-stack.md`)

The same page says patterns come from protocols:

> "Multi-agent patterns emerge from these behavior contracts." (`application-stack.md`)

The design-patterns intro repeats this:

> "A multi-agent design pattern is a structure that emerges from message protocols: it describes how agents interact with each other to solve problems." (`core-
> user-guide/design-patterns/intro.md`)

AgentChat is more opinionated and supplies presets:

> "AgentChat provides intuitive defaults, such as Agents with preset behaviors and Teams with predefined multi-agent design patterns." (`agentchat-user-
> guide/index.md`)

Group chat is one pattern. The Core group-chat notebook says:

> "Group chat is a design pattern where a group of agents share a common thread of messages: they all subscribe and publish to the same topic." (`core-user-
> guide/design-patterns/group-chat.ipynb`)

AgentChat `RoundRobinGroupChat` is a simple peer-shared-context pattern:

> "RoundRobinGroupChat is a simple yet effective team configuration where all agents share the same context and take turns responding in a round-robin fashion.
> Each agent, during its turn, broadcasts its response to all other agents, ensuring that the entire team maintains a consistent context." (`agentchat-user-
> guide/tutorial/teams.ipynb`)

`SelectorGroupChat` introduces a centralized selector, often model-based. The AgentChat index describes it as:

> "Multi-agent coordination through a shared context and centralized, customizable selector" (`agentchat-user-guide/index.md`)

`Swarm` is described as:

> "Multi-agent coordination through a shared context and localized, tool-based selector" (`agentchat-user-guide/index.md`)

The README also shows orchestration through `AgentTool`: an assistant agent can call specialized agents as tools. That is neither pure peer chat nor a classic
manager chat; it is model-driven tool selection where tools happen to be agents. The README labels the example:

> "You can use AgentTool to create a basic multi-agent orchestration setup." (`README.md`)

Magentic-One is the clearest manager/planner pattern. Its docs say:

> "Magentic-One work is based on a multi-agent architecture where a lead Orchestrator agent is responsible for high-level planning, directing other agents and
> tracking task progress." (`agentchat-user-guide/magentic-one.md`)

The orchestrator loop is ledger-based:

> "The Orchestrator begins by creating a plan to tackle the task, gathering needed facts and educated guesses in a Task Ledger that is maintained. At each step
> of its plan, the Orchestrator creates a Progress Ledger where it self-reflects on task progress and checks whether the task is completed." (`magentic-one.md`)

And recovery is integrated with replanning:

> "If the Orchestrator finds that progress is not being made for enough steps, it can update the Task Ledger and create a new plan." (`magentic-one.md`)

So AutoGen supports several orchestration shapes:

- Direct one-agent task execution.
- Single agent with tool loop.
- Agent-as-tool composition.
- Round-robin shared-context team.
- Selector-managed shared-context team.
- Handoff / swarm.
- Directed graph workflows (`GraphFlow`).
- Lead-orchestrator architecture (`MagenticOneGroupChat`).
- Core-level custom protocols over direct/broadcast messages.

The "orchestrator's job" is not defined globally. In AgentChat teams it may be speaker selection and termination. In Magentic-One it is planning, delegating,
tracking, and replanning. In Core it is whatever behavior contract the app implements.

## 4. Failure handling and recovery

AutoGen has explicit mechanisms for some failure classes and leaves others to application design.

Loop/stuck prevention is mostly termination conditions, max turns, token/time limits, external stop, and cancellation.

The termination tutorial states the problem directly:

> "However, a run can go on forever, and in many cases, we need to know when to stop them. This is the role of the termination condition." (`agentchat-user-
> guide/tutorial/termination.ipynb`)

Built-in termination conditions include maximum messages, text mention, token usage, timeout, handoff termination, source match, external termination, and
others. The same tutorial says conditions can be combined:

> "termination conditions can be combined using the AND (&) and OR (|) operators to create more complex termination logic." (`termination.ipynb`)

External termination is graceful rather than immediate:

> "Calling ExternalTermination.set ... will stop the team when the current agent's turn is over. Thus, the team may not stop immediately. This allows the
> current agent to finish its turn and broadcast the final message to the team before the team stops, keeping the team's state consistent." (`teams.ipynb`)

Cancellation is immediate and exception-based:

> "Different from stopping a team, aborting a team will immediately stop the team and raise a asyncio.CancelledError exception." (`teams.ipynb`)

This is a useful distinction: graceful stop preserves team state; abort is a hard interrupt.

Malformed tool-call arguments are handled in at least the Core tool-equipped agent example with `try/except`: it parses `call.arguments` as JSON, runs the tool,
and returns `FunctionExecutionResult(..., is_error=True, content=str(e))` on exception. That example is in `core-user-guide/components/tools.ipynb`. This is
sample code, not necessarily a universal framework guarantee.

Tool execution cancellation is supported via `CancellationToken`. The migration guide says:

> "You can also use CancellationToken to cancel a code execution if it takes too long." (`migration-guide.md`)

Disagreement is commonly represented by protocols rather than a built-in arbiter. The Core application-stack example uses Coder, Executor, Reviewer, and
messages. It says the Reviewer evaluates execution and either approves or sends a review back:

> "otherwise, it sends a ReviewMsg to the Coder Agent for another round of code generation." (`core-concepts/application-stack.md`)

The design-patterns intro calls reflection a robustness pattern:

> "group chat for task decomposition, and reflection for robustness." (`core-user-guide/design-patterns/intro.md`)

The AgentChat teams tutorial uses a writer/critic reflection pattern where a critic says `APPROVE` and `TextMentionTermination` stops the team. That is a
protocol-level resolution mechanism, not a framework-level truth detector.

Magentic-One has explicit progress tracking and replanning as noted above, but its docs describe the mechanism at architecture level. They do not prove from
code in the user guide that replanning always detects real stagnation or safely recovers. Treat as implemented feature if present in package, but as a docs
claim in this report.

Observability is a failure-handling support, not recovery itself. The migration guide says:

> "Your application can use these streams to observe the agents and teams in real-time." (`migration-guide.md`)

The README describes Studio as not production-ready and warns developers to add authentication and security themselves. That is a failure boundary: AutoGen
Studio is a prototype UI, not a deployed safety shell.

There is no evidence in the docs read that AutoGen centrally guarantees:

- semantic correctness of final answers;
- automatic deadlock diagnosis beyond terminations/timeouts/cancellation;
- global reconciliation of all component states;
- automatic retry policy for every malformed model output;
- durable recovery from process crash unless the app persists and reloads
  component state.

The failure story is: define a protocol, stream/observe it, stop it with termination conditions, cancel if needed, persist state if the app needs resume, and
implement recovery in agents or patterns.

## 5. Tool / skill integration model

AutoGen uses model tool/function calling, Python functions wrapped as tools, MCP workbenches, code executors, and agents-as-tools.

Core's tool docs define tools as executable code:

> "Tools are code that can be executed by an agent to perform actions." (`core-user-guide/components/tools.ipynb`)

They connect tools to model-generated function calls:

> "In the context of AI agents, tools are designed to be executed by agents in response to model-generated function calls." (`tools.ipynb`)

Function tools are schema-driven:

> "The FunctionTool class uses descriptions and type annotations to inform the LLM when and how to use a given function." (`tools.ipynb`)

Model clients generate tool calls from JSON schemas:

> "The model client takes the list of tools and generates a JSON schema for the parameters of each tool. Then, it generates a request to the model API with the
> tool's JSON schema and the other messages to obtain a result." (`tools.ipynb`)

The trust boundary is important. The model produces a structured call; the host application parses arguments and runs registered tool code. The LLM output is
not itself arbitrary execution unless the registered tool is a code executor. When the registered tool is a code executor, the danger moves to executor
configuration.

AgentChat's assistant can use tools directly:

> "AssistantAgent is a built-in agent that uses a language model and has the ability to use tools." (`agentchat-user-guide/tutorial/agents.ipynb`)

The v0.4 migration guide emphasizes a design change from v0.2 group-chat tool routing:

> "In v0.4, there is no need to register the tool functions on a user proxy, as the tools are directly executed within the AssistantAgent, which publishes the
> response from the tool to the group chat. So the group chat manager does not need to be involved in routing tool calls." (`migration-guide.md`)

The same guide says the old approach had issues:

> "We have observed numerous issues with this approach, such as the the tool call routing not working as expected, and the tool call request and result cannot
> be accepted by models without support for function calling." (`migration-guide.md`)

Agents can be tools:

> "Any BaseChatAgent can be used as a tool by wrapping it in an AgentTool. This allows for a dynamic, model-driven multi-agent workflow where the agent can call
> other agents as tools to solve tasks." (`agents.ipynb`)

But parallel calls are dangerous for stateful agents/teams. The agent tutorial says:

> "When using AgentTool or TeamTool, you must disable parallel tool calls to avoid concurrency issues. These tools cannot run concurrently as agents and teams
> maintain internal state that would conflict with parallel execution." (`agents.ipynb`)

That is one of AutoGen's clearest documented trust/state boundaries.

Code execution has two command-line executors:

> "Docker ... all commands are executed in a Docker container" and "Local ... all commands are executed on the host machine" (`core-user-
> guide/components/command-line-code-executors.ipynb`)

The Docker executor creates a container and runs commands there:

> "The DockerCommandLineCodeExecutor will create a Docker container and run all commands within that container." (`command-line-code-executors.ipynb`)

The local executor is bluntly warned:

> "The local version will run code on your local system. Use it with caution." (`command-line-code-executors.ipynb`)

The README's MCP example includes a warning:

> "Only connect to trusted MCP servers as they may execute commands in your local environment or expose sensitive information." (`README.md`)

Magentic-One adds stronger operational warnings because it browses, reads files, and executes code. Its docs say:

> "Using Magentic-One involves interacting with a digital world designed for humans, which carries inherent risks." (`agentchat-user-guide/magentic-one.md`)

It recommends containers, virtual environments, log monitoring, human oversight, limited access, and safeguarding data. It also warns:

> "Be aware that agents may occasionally attempt risky actions, such as recruiting humans for help or accepting cookie agreements without human involvement."
> (`magentic-one.md`)

And:

> "Magentic-One may be susceptible to prompt injection attacks from webpages." (`magentic-one.md`)

So AutoGen does not hide the trust boundary. It exposes potentially dangerous execution capabilities and repeatedly says isolation/oversight are the app
operator's job.

## 6. Anti-patterns and explicit non-goals

AutoGen's docs include several explicit warnings, deprecations, and non-goals.

First: AutoGen itself is no longer the recommended new-project framework. The README says:

> "New users should start with Microsoft Agent Framework." (`README.md`)

And later:

> "For new projects, we recommend Microsoft Agent Framework." (`README.md`)

Second: AutoGen Studio is not production-ready:

> "AutoGen Studio is meant to help you rapidly prototype multi-agent workflows and demonstrate an example of end user interfaces built with AutoGen. It is not
> meant to be a production-ready app." (`README.md`)

That is an explicit product boundary.

Third: Core is deliberately unopinionated, so relying on Core to provide a complete orchestration policy is a category error:

> "It is not tied to any specific agent abstraction or multi-agent pattern." (`core-concepts/application-stack.md`)

Fourth: `AssistantAgent` is documented as a kitchen-sink prototype/education agent:

> "AssistantAgent is a 'kitchen sink' agent for prototyping and educational purpose -- it is very general. Make sure you read the documentation and
> implementation to understand the design choices. Once you fully understand the design, you may want to implement your own agent." (`agentchat-user-
> guide/tutorial/agents.ipynb`)

That is a warning against treating the preset as a production architecture just because it runs.

Fifth: v0.2 `ConversableAgent.register_reply` style is effectively discouraged in the migration guide. The guide says v0.4 custom agents avoid guessing about
`reply_func` parameters and `position`:

> "Rather than guessing what the reply_func does, all its parameters, and what the position should be..." (`migration-guide.md`)

This is an anti-pattern: opaque callback registration where ordering and parameters are hard to reason about.

Sixth: old group-chat tool routing through user proxy is called problematic:

> "We have observed numerous issues with this approach..." (`migration-guide.md`)

The replacement is direct tool execution in the assistant agent, reducing manager/user-proxy routing complexity.

Seventh: built-in sequential chat was removed from AgentChat v0.4 because it was too opinionated:

> "Base on the feedback from the community, the initiate_chats function is too opinionated and not flexible enough to support the diverse set of scenarios that
> users want to implement." (`migration-guide.md`)

Then:

> "Therefore, in v0.4, we do not provide a built-in function for sequential chat in the AgentChat API." (`migration-guide.md`)

This is an explicit design choice: use basic Python or Core workflows rather than a too-rigid high-level helper.

Eighth: distributed/lifecycle docs contain aspirational notes. Paging agent instances in/out is named but "not implemented yet". That should prevent a reader
from crediting AutoGen with full actor-system lifecycle management.

Ninth: external model compatibility is not guaranteed. The migration guide says about OpenAI-compatible APIs:

> "We don't test all the OpenAI-Compatible APIs, and many of them works differently from the OpenAI API even though they may claim to suppor [sic] it. Please test
> them before using them." (`migration-guide.md`)

Tenth: tool and code-execution safety is delegated to the application/runtime operator. Warnings about local execution, trusted MCP servers, containers, human
oversight, and prompt injection are explicit.

## 7. Anchoring caveats

AutoGen's context is materially different from the schema-org-json-ld redesign context. The differences below are not recommendations; they are transfer
hazards.

1. AutoGen is a library/framework; the redesign target is one autonomous
   orchestrator for a public GitHub repository.

AutoGen exposes abstractions and says developers define behavior contracts. The redesign has to choose one behavior contract and live with it over time. A
pattern that is tolerable as a library option may be too underspecified for a concrete cron-run orchestrator.

2. AutoGen applications usually start from an application prompt/task; the
   redesign orchestrator runs on a schedule.

The README quickstart and AgentChat examples call `agent.run(task=...)` or `team.run_stream(task=...)`. The redesign target wakes up, polls state, acts, and
closes a cycle. AutoGen's examples assume an explicit task boundary; the redesign's task boundary is partly inferred from repository state.

3. AutoGen often keeps a human in the loop, especially for risky tools.

Magentic-One recommends human oversight and code-execution approval. The redesign context has minimal human-in-the-loop during routine cron cycles. Any AutoGen
pattern relying on live approval changes meaning in an autonomous setting.

4. AutoGen is Python-first with optional .NET; the redesign tooling base is
   Rust plus GitHub Actions.

AutoGen's extension ecosystem, function tools, notebooks, and model clients are Python-native. The redesign can transfer architectural ideas, but not the
runtime/library affordances directly.

5. AutoGen targets applications with developer-owned state persistence.

Its state tutorial says state is a dictionary that can be written to file or database. It does not impose one canonical repository-state file. The redesign has
already accumulated repo-resident state and cycle artifacts. Component-local save/load is not automatically equivalent to a durable, auditable, cross-cycle
ledger.

6. AutoGen supports distributed runtime, but some lifecycle claims are incomplete.

The docs say paging in/out agents is not implemented yet. Do not transfer a stronger actor-runtime guarantee than AutoGen actually documents.

7. AutoGen's documented tasks are short-to-medium interactive examples and
   benchmark-style agentic tasks.

Magentic-One handles open-ended web/file tasks, but the docs still present a task run with ledgers and completion. The redesign target spans hundreds of cycles,
PRs, issues, audits, and changing instructions. Long horizon creates state drift and institutional-memory problems that AutoGen docs do not center.

8. AutoGen's failure handling assumes applications pick termination and
   cancellation rules.

The redesign cannot leave these decisions to downstream app developers; it is the downstream app. AutoGen's flexibility becomes design work for the redesign.

9. AutoGen permits dangerous tools with warnings.

Local code execution, MCP servers, web browsing, and prompt injection are all explicitly risky. In a public-repo autonomous orchestrator, the blast radius and
audit requirements differ from a developer's local prototype.

10. AutoGen's current maintenance mode lowers confidence in future evolution.

The architecture is still worth studying, but any "the framework will add this later" claim should be discounted. The README points new users to a successor
framework.

11. AutoGen examples are application-code-centric.

Much orchestration is normal Python glue. That is a different governance surface from editing prompts, tools, workflow files, and persisted state in a GitHub
repo.

12. AutoGen's high-level AgentChat presets intentionally hide Core details.

That is useful for prototyping. It can also hide protocol/state edges that matter in a long-running autonomous system. The docs themselves warn that
`AssistantAgent` is a kitchen sink.

## Patterns observed in AutoGen

This section lists observed patterns without evaluating them for v2 relevance. Cross-system synthesis is deferred.

- Layered architecture: Core API for actor-style messaging/runtime, AgentChat
  for opinionated high-level agents/teams, Extensions for model clients/tools,
  Studio/Bench as developer tools.

- Explicit maintenance-status signaling: README front-loads maintenance mode
  and successor recommendation.

- Actor-model framing: agents have identities, state, messages, and runtime
  lifecycle.

- Runtime-mediated identity: `AgentID = (Agent Type, Agent Key)` rather than
  only direct object references.

- Data-dependent agent instances: topic source / agent key can create per-user,
  per-session, or per-request agent instances.

- Direct and broadcast messaging both exist.

- Publish/subscribe routing through topics and subscriptions.

- Behavior contracts as message protocols; patterns emerge from protocol
  implementation rather than from a universal orchestrator object.

- High-level teams as presets over lower-level runtime primitives.

- Shared-context round-robin group chat.

- Centralized speaker selection via `SelectorGroupChat`.

- Localized handoff / swarm-style selection.

- Agent-as-tool composition with model-driven delegation.

- Lead orchestrator pattern in Magentic-One with planning, delegation, progress
  tracking, and replanning.

- Ledger vocabulary in Magentic-One: Task Ledger and Progress Ledger.

- Statefulness by default for AgentChat agents and teams.

- Caller sends new messages, not complete history, to stateful agents.

- Reset/resume distinction for teams.

- State save/load APIs on agents, teams, and termination conditions.

- State serializes as dictionaries suitable for file/database persistence.

- Model context abstraction separates stored conversation history from the
  model-visible virtual view.

- Built-in context-window controls: unbounded, buffered, token-limited.

- Termination conditions as first-class callables over recent messages/events.

- Composable termination conditions with AND/OR.

- External graceful stop distinct from immediate cancellation.

- Cancellation tokens passed through agent/team/tool execution.

- Streaming as observability surface for agent inner messages and team
  conversations.

- Reflection pattern for robustness: generator plus critic/reviewer loop.

- Tool calling based on JSON schema generated from function signatures,
  annotations, and descriptions.

- Registered tool code runs on the host side; model output requests tool calls
  rather than executing by itself.

- Tool errors can be represented as function execution results with `is_error`.

- Direct tool execution inside `AssistantAgent` replaces older routing through
  user proxies in group chat.

- Parallel tool calls exist but are explicitly incompatible with stateful
  `AgentTool` / `TeamTool` concurrency.

- Agents and teams can themselves be wrapped as tools.

- MCP workbench integration, with trust warning.

- Docker code executor as isolated default-ish example for code execution.

- Local code executor allowed but warned as dangerous.

- Docker-out-of-Docker documented for containerized apps spawning sibling
  executor containers.

- Human approval function shown for Magentic-One code execution.

- Magentic-One docs explicitly warn about risky web actions, prompt injection,
  cookie agreements, and sensitive data exposure.

- Custom agents replace opaque `register_reply` callback ordering.

- Built-in sequential chat removed from AgentChat v0.4 as too opinionated;
  suggested alternatives are basic Python glue or Core workflows.

- Studio positioned as rapid prototype/demo UI, not production app.
