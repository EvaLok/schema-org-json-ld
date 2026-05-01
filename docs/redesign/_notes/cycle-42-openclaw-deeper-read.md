# Cycle 42 — openclaw deeper read (primary-source)

**Cycle:** 42 (2026-05-01)
**Issue:** [redesign-research] openclaw deeper read with primary-source repo access
**Supersedes:** cycle-14 stub at `docs/redesign/1-research/systems/openclaw.md`
**Deliverable location:** `docs/redesign/_notes/cycle-42-openclaw-deeper-read.md`
**Model:** claude-opus-4-7

---

## Sources read

All sources are primary (direct repo and docs access). No firewall blocks
encountered; `github.com/openclaw/openclaw` and `raw.githubusercontent.com` were
accessible. The official docs site (`docs.openclaw.ai`) redirects content served
from the repo `docs/` tree, and the raw GitHub content was read directly rather
than via the rendered docs site.

- `README.md` (openclaw/openclaw, main branch) — full read
- `VISION.md` (openclaw/openclaw, main branch) — full read
- `docs/concepts/architecture.md` — full read
- `docs/concepts/agent-loop.md` — full read
- `docs/concepts/queue.md` — full read
- `docs/concepts/memory.md` — full read
- `docs/concepts/memory-builtin.md` — full read
- `docs/concepts/active-memory.md` — full read
- `docs/concepts/session.md` — full read
- `docs/concepts/multi-agent.md` — full read
- `docs/concepts/agent.md` — full read
- `docs/concepts/delegate-architecture.md` — full read
- `docs/tools/plugin.md` — full read
- `docs/plugins/sdk-overview.md` — full read
- `docs/plugins/building-plugins.md` — full read
- `docs/plugins/sdk-channel-plugins.md` — full read
- `src/` directory listing (GitHub tree view)
- `docs/` directory listing (GitHub tree view)
- `docs/gateway/` directory listing (GitHub tree view)
- `docs/concepts/` directory listing (GitHub tree view)

Secondary source limitations: `docs/gateway/security/` is a subdirectory (not a
single file at `docs/gateway/security.md`); not read in full but security model
is documented extensively in README and VISION.md which were read.

---

## 1. Overall philosophy and load-bearing thesis

openclaw's central thesis is stated in the README:

> "OpenClaw is a _personal AI assistant_ you run on your own devices. It answers
> you on the channels you already use. [...] The Gateway is just the control
> plane — the product is the assistant."

Three named positions anchor the design:

**Local-first.** The Gateway runs on the user's machine (not in a cloud service).
`docs/concepts/architecture.md`: "One Gateway per host; it is the only place
that opens a WhatsApp session." Companion apps (macOS, iOS, Android) connect
back to the local Gateway. Remote access is explicitly an SSH tunnel or
Tailscale VPN to the local machine — not a hosted cloud backend. This is the
primary differentiator the README leads with.

**Small core, plugins extend.** VISION.md: "Core stays lean; optional capability
should usually ship as plugins. We are generally slimming down core while
expanding what plugins can do." The corollary: "If a useful feature cannot be
built as a plugin yet, we welcome PRs and design discussions that extend the
plugin API instead of adding one-off core behavior." This is not aspirational
framing; the `src/` directory shows 60+ subdirectories suggesting real
complexity, but the extension contract (SDK subpaths, `register(api)` callbacks)
is the design boundary.

**Treat inbound as untrusted.** README: "Treat inbound DMs as **untrusted
input**." Default DM policy is `pairing`: unknown senders receive a pairing code
and the bot does not process their message until explicitly approved
(`openclaw pairing approve <channel> <code>`). Public DMs require explicit
operator opt-in. `docs/gateway/sandboxing.md` is referenced extensively;
`docs/gateway/security/` is a subdirectory suggesting this is a multi-page
concern.

**What problem openclaw argues most agent designs fail at.** VISION.md does not
name a competitor failure mode explicitly, but the "What We Will Not Merge"
list reveals it by negation: the rejected patterns are "agent-hierarchy
frameworks (manager-of-managers / nested planner trees) as a default
architecture" and "heavy orchestration layers that duplicate existing agent and
tool infrastructure." The implied failure is building too much orchestration
into the core — openclaw's solution shape is lean core + principled extension
boundary.

**Why TypeScript?** VISION.md: "OpenClaw is primarily an orchestration system:
prompts, tools, protocols, and integrations. TypeScript was chosen to keep
OpenClaw hackable by default. It is widely known, fast to iterate in, and easy
to read, modify, and extend." This is a deliberate tradeoff statement, not
default-language inertia.

---

## 2. Architecture: Gateway, Channels, Plugins *(primary depth lens)*

### Gateway structure

`docs/concepts/architecture.md`:

> "A single long-lived **Gateway** owns all messaging surfaces (WhatsApp via
> Baileys, Telegram via grammY, Slack, Discord, Signal, iMessage, WebChat)."
>
> "One Gateway per host; it is the only place that opens a WhatsApp session."

The Gateway is a **single Node.js process** — one daemon per machine, not a
registry of services. It serves:

- A WebSocket API on `127.0.0.1:18789` (default) for control-plane clients
  (macOS app, CLI, web UI, automations) and Nodes (iOS/Android/headless)
- An HTTP server under the same port for the Canvas host
  (`/__openclaw__/canvas/`, `/__openclaw__/a2ui/`)

The Gateway exposes a typed WebSocket API:

```
Requests:  {type:"req", id, method, params} → {type:"res", id, ok, payload|error}
Events:    {type:"event", event, payload, seq?, stateVersion?}
```

First frame must be a `connect`; the gateway hard-closes non-JSON or
non-connect first frames. Protocol is TypeBox-schema-defined; JSON Schema and
Swift models are generated from those schemas.

**Is the Gateway streaming event-ingestion or per-event request-response?**

This is the load-bearing architecture question from the Axis 12 cycle-41 flag
(Q(c) BORDERLINE-PASS). The answer from primary sources:

The Gateway is **event-driven per-event handling, not streaming event
ingestion**. Channels maintain persistent upstream connections (WhatsApp via
Baileys, Telegram via grammY, Discord/Slack via their respective libraries), and
those channel libraries deliver message events to the Gateway as they arrive —
the Gateway does not poll for them. But each message event triggers a **discrete
agent turn** routed through a per-session FIFO queue. The agent loop
(`runEmbeddedPiAgent`) is invoked per-turn, not as a continuously-running
coroutine. The Gateway daemon is always-on; agent execution is per-event
request-response, not a continuous streaming pipeline.

`docs/concepts/architecture.md` Invariants section: "Events are not replayed;
clients must refresh on gaps." This is request-response semantics, not a
streaming event-ingestion log.

### Channel registration

Channels are **plugins** registered via the plugin SDK:

```typescript
import { defineChannelPluginEntry } from "openclaw/plugin-sdk/channel-core";

export default defineChannelPluginEntry({
  id: "my-channel",
  // ...
  register(api) {
    api.registerChannel(...);
  },
});
```

`docs/plugins/sdk-overview.md` lists `api.registerChannel(...)` as a
"capability registration" method in the full plugin SDK. Channel plugins own:

- **Config** — account resolution and setup wizard
- **Security** — DM policy and allowlists
- **Pairing** — DM approval flow
- **Session grammar** — how provider-specific conversation ids map to base chats,
  thread ids, and parent fallbacks via `messaging.resolveSessionConversation(...)`
- **Outbound** — sending text, media, polls to the platform
- **Threading** — how replies are threaded
- **Heartbeat typing** — optional typing signals

Core owns the shared `message` tool, prompt wiring, outer session-key shape,
generic `:thread:` bookkeeping, and dispatch. `docs/plugins/sdk-channel-plugins.md`:

> "Channel plugins do not need their own send/edit/react tools. OpenClaw keeps
> one shared `message` tool in core."

Channel plugins are npm packages with `openclaw.extensions` declared in
`package.json`. Official channels are in the bundled package (Telegram, Discord,
Slack, WhatsApp, etc.); community channels are published to ClawHub.

### Multi-agent routing and topology

`docs/concepts/multi-agent.md` describes "bindings" — deterministic rules
mapping `(channel, accountId, peer)` tuples to `agentId`:

```json5
{
  bindings: [
    { agentId: "work", match: { channel: "whatsapp", accountId: "biz" } },
    { agentId: "main", match: { channel: "whatsapp" } },
  ]
}
```

Resolution is "most-specific wins" through a 7-tier hierarchy (peer match →
parentPeer match → guildId+roles → guildId → teamId → accountId → default).

**Are agents long-running daemons or per-request invocations?** The agents are
**per-request invocations**. `docs/concepts/agent-loop.md`:

> "An agentic loop is the full 'real' run of an agent: intake → context assembly →
> model inference → tool execution → streaming replies → persistence."

`runEmbeddedPiAgent` is called per agent turn; it "serializes runs via per-session
+ global queues" and "enforces timeout -> aborts run if exceeded." The Gateway
daemon is long-running; the agents themselves are per-invocation embedded within
that daemon process.

The `src/agents/` directory suggests agent management infrastructure; the
embedded runtime is `pi-agent-core` (a dependency, not openclaw-owned; the
runtime is invoked as `runEmbeddedPiAgent`).

### Plugin API contract

`docs/plugins/building-plugins.md` shows the entry point signature:

```typescript
import { definePluginEntry } from "openclaw/plugin-sdk/plugin-entry";

export default definePluginEntry({
  id: "my-plugin",
  name: "My Plugin",
  description: "...",
  register(api) {
    api.registerTool({ name: "my_tool", ... });
    api.registerHook("message_received", handler);
    // etc.
  },
});
```

Lifecycle: plugin is imported at Gateway startup; `register(api)` is called
once. No explicit unload hook is documented for runtime unloads (gateway restart
required for config changes). Plugins declare `activation.onStartup: true` in
their manifest when they need startup import for runtime-registered capabilities.

Plugin discovery order (first match wins):
1. `plugins.load.paths` — explicit config paths
2. `<workspace>/.openclaw/<plugin-root>/*.ts` — workspace plugins
3. `~/.openclaw/<plugin-root>/*.ts` — global plugins
4. Bundled plugins — shipped with openclaw

---

## 3. State, memory, and persistence *(second primary depth lens)*

### Session architecture

`docs/concepts/session.md`: Sessions are the conversation thread; each message
is routed to a session based on its source:

| Source | Behavior |
|---|---|
| Direct messages | Shared session by default (`main`) |
| Group chats | Isolated per group |
| Rooms/channels | Isolated per room |
| Cron jobs | Fresh session per run |
| Webhooks | Isolated per hook |

Session state lives at `~/.openclaw/agents/<agentId>/sessions/sessions.json`;
transcripts at `~/.openclaw/agents/<agentId>/sessions/<sessionId>.jsonl`.
Sessions have separate lifecycle timestamps (`sessionStartedAt` for daily reset,
`lastInteractionAt` for idle reset, `updatedAt` for general bookkeeping).

Session lifecycle: daily reset at 4:00 AM local time by default; idle reset
configurable; manual `/new` or `/reset`. Note: "heartbeat, cron, and exec system
events do not keep the session alive" for idle reset purposes — only real
user/channel interactions extend idle freshness.

A **session** is architecturally: a conversation thread with its own transcript
file, routing key (`agent:<agentId>:<mainKey>`), and lifecycle timestamps. Not
a process or a long-lived object; the per-session lane in the queue is the
runtime representation.

### Memory singleton plugin slot

`VISION.md`:

> "Memory is a special plugin slot where only one memory plugin can be active at
> a time."

`docs/tools/plugin.md`: The slot is selected via `plugins.slots.memory`. The
default is `memory-core` (built-in SQLite backend). Alternatives:
`memory-lancedb`, Honcho (cross-session AI-native), QMD (local sidecar).

**Is the singleton process-level or per-session?** Process-level (per-Gateway).
`plugins.slots.memory` is a config key at the root level, not per-session or
per-agent. Memory storage is per-agent (`~/.openclaw/memory/<agentId>.sqlite`
for the builtin backend) but the plugin that handles memory is one plugin for
the whole Gateway.

`docs/concepts/memory.md`:

> "Your agent has three memory-related files:
> - `MEMORY.md` — long-term memory. Durable facts, preferences, and decisions.
>   Loaded at the start of every DM session.
> - `memory/YYYY-MM-DD.md` — daily notes. Today and yesterday's notes are loaded
>   automatically.
> - `DREAMS.md` — Dream Diary and dreaming sweep summaries."

This confirms: memory is **Markdown files** in the workspace, indexed into
SQLite, accessed via `memory_search` and `memory_get` tools provided by the
active memory plugin.

### Memory plugin API

The active memory plugin provides two agent tools: `memory_search` (semantic
search via hybrid vector+keyword retrieval) and `memory_get` (read a specific
memory file or line range). Plugin hooks relevant to memory:
`registerMemoryPromptSupplement(builder)` (additive memory-adjacent prompt
section) and `registerMemoryCorpusSupplement(adapter)` (additive memory
search/read corpus). Memory also integrates with the compaction pipeline via
`before_compaction` / `after_compaction` hooks.

**Active memory** is a separate optional plugin-owned sub-agent that runs
before the main reply to surface relevant memory proactively. It runs a bounded
pass (configurable `timeoutMs: 15000`) using the memory search tools, injects
the result as an "untrusted context" prefix. It is scoped to specific agents and
chat types — not a default behavior.

**Dreaming**: optional background consolidation pass. When enabled,
`memory-core` auto-manages a cron job for full dreaming sweeps. Promotions from
short-term (`memory/.dreams/`) to `MEMORY.md` are gated on score, recall
frequency, and query diversity thresholds. DREAMS.md is the human-review
surface. The cron job runs under the same Gateway process.

### Cross-channel memory access

Agents access memory through the session's memory tools, which search the
per-agent index (`~/.openclaw/memory/<agentId>.sqlite`). Multiple channels
routing to the same agent see the same memory. Cross-agent memory access is
possible via `memorySearch.qmd.extraCollections` (QMD backend only).

### State storage

Per-agent state: `~/.openclaw/agents/<agentId>/agent/` (auth profiles, model
registry, per-agent config). Global config: `~/.openclaw/openclaw.json`. Session
transcripts: per-session JSONL. Memory index: per-agent SQLite. There is no
global state file analogous to v1's `state.json`; state is per-agent, per-session
files.

---

## 4. Async, concurrency, and event handling

### Queue architecture

`docs/concepts/queue.md`:

> "We serialize inbound auto-reply runs (all channels) through a tiny in-process
> queue to prevent multiple agent runs from colliding, while still allowing safe
> parallelism across sessions."

The queue is:
- **Lane-aware FIFO**: each session has a per-session lane (`session:<key>`);
  all sessions also share a global lane (`main` by default)
- **Concurrency cap**: default lane concurrency is 1 (unconfigured lanes); main
  defaults to 4, subagent to 8; overall cap via `agents.defaults.maxConcurrent`
- **Additional lanes**: `cron`, `cron-nested`, `nested`, `subagent` for
  background jobs to run without blocking inbound replies

**What happens when a Discord message arrives while a Slack message is being
processed?** If they route to the same session (same agent): the Discord message
is queued in that session's FIFO lane and waits. If they route to different
sessions: they run in parallel (up to `maxConcurrent`). This is the architectural
answer to multi-channel concurrency: **per-session serialization + cross-session
parallelism**.

The queue runs the default `steer` mode: queue new messages into the active
runtime rather than starting a new turn. The active model turn receives queued
steering messages "after the current assistant turn finishes executing its tool
calls, before the next LLM call." If steering is unavailable, it falls back to
`followup` (a separate turn after the current run ends).

### Is there an "always-on" loop?

The Gateway daemon is always-on (runs as a launchd/systemd user service). The
agent loop is not always-on — each agent turn is a discrete invocation of
`runEmbeddedPiAgent`. There is no continuously-running coroutine per session.

The `src/daemon/` directory manages the daemon process; `src/gateway/` contains
the Gateway itself. The Gateway listens on WebSocket and dispatches events to
the queue system.

### Long-running operations

`docs/concepts/agent-loop.md` documents timeout handling:
- Agent runtime: `agents.defaults.timeoutSeconds` default 172800s (48 hours)
- Cron runtime: isolated per-run `timeoutSeconds` owned by the cron scheduler
- `agent.wait` default: 30s (wait-only, does not stop agent)
- Model idle timeout: aborts when no response chunks arrive within the idle
  window

Long-running LLM calls and tool invocations are handled via async/await in the
embedded pi-agent-core runtime. The queue lane stays occupied until the run ends
or times out; the stuck-session watchdog (`diagnostics.stuckSessionWarnMs`)
detects and can release stale lanes.

### Cron interaction

`docs/concepts/queue.md`:
> "Cron jobs → Fresh session per run"

> "Additional lanes may exist (e.g. `cron`, `cron-nested`, `nested`, `subagent`)
> so background jobs can run in parallel without blocking inbound replies."

Cron runs in an isolated lane, separate from inbound channel replies. The cron
scheduler owns the outer timeout; the inner agent execution uses a nested lane.
Cron sessions do not extend daily/idle freshness.

**Note on sandbox:** The default sandbox (for non-main sessions including most
cron and agent-spawned sessions) **denies `cron`**. README: "Typical sandbox
default: allow `bash`, `process`, `read`, `write`, `edit`, `sessions_list`,
`sessions_history`, `sessions_send`, `sessions_spawn`; deny `browser`, `canvas`,
`nodes`, `cron`, `discord`, `gateway`." Cron is available in the main session
(unsandboxed) and where explicitly enabled.

---

## 5. Tool / skill / plugin integration model

### Tools vs. skills vs. plugins

openclaw uses three terms with distinct meanings:

- **Tools**: capabilities the agent can invoke as function calls during a run.
  Core tools (`read`, `exec`, `edit`, `write`, `apply_patch`) are always
  available. Plugin tools are registered via `api.registerTool(...)`.
- **Skills**: Markdown files in the workspace (`skills/`) that are injected into
  the agent's system prompt. They provide guidance and conventions, not
  executable code. Skills can be "bundled" (shipped with openclaw), "managed"
  (`~/.openclaw/skills`), or "workspace" (per-agent).
- **Plugins**: npm packages (or local directories) that extend the Gateway
  process with new capabilities. Two plugin styles:
  - **Native/code plugins**: runtime module (`openclaw.plugin.json` + TypeScript
    entry), executes in-process via `register(api)` callback.
  - **Bundle-style plugins**: Codex/Claude/Cursor-compatible layout
    (`.codex-plugin/`, `.claude-plugin/`, `.cursor-plugin/`); maps to openclaw
    features. Smaller interface, better security boundaries.

VISION.md: "Prefer bundle-style plugins when they can express the capability.
They have a smaller, more stable interface and better security boundaries. Use
code plugins when the capability needs runtime hooks, providers, channels, tools,
or other in-process extension points."

### Sandbox allowlist/denylist mechanism

`docs/gateway/sandboxing.md` is referenced but the specific implementation is
in `src/security/` and `docs/gateway/sandbox-vs-tool-policy-vs-elevated.md`
(not fully read). From README and concept docs:

- Sandbox mode is `agents.defaults.sandbox.mode: "non-main"` for group/channel
  safety.
- Default sandbox backend: Docker (SSH and OpenShell alternatives).
- The sandbox allowlist/denylist maps tool names to allow/deny. Non-main
  sessions: deny includes `browser`, `canvas`, `nodes`, `cron`, `discord`,
  `gateway`. Allow includes `bash`, `process`, `read`, `write`, `edit`,
  `sessions_*`.
- Per-agent tool policy (v2026.1.6+): `agents.list[].tools.allow` /
  `agents.list[].tools.deny` enforces at Gateway level, independent of the
  agent's prompt-level rules. The delegate architecture doc shows this in
  practice:
  ```json5
  { tools: { allow: ["read", "exec", "message", "cron"],
             deny: ["write", "edit", "apply_patch", "browser", "canvas"] } }
  ```

The mechanism is: tool call arrives → checked against per-agent policy →
checked against plugin `before_tool_call` hooks → checked against trusted tool
policy (bundled-only seam) → executed. `before_tool_call: { block: true }` is
terminal; `before_tool_call: { requireApproval: true }` pauses and prompts the
user.

### Plugin promotion bar

VISION.md:

> "The bar for adding optional plugins to core is intentionally high. Plugin
> discovery, official publisher status, provenance, and security review live in
> [ClawHub](https://clawhub.ai/)."

The rationale: "If a useful feature cannot be built as a plugin yet, we welcome
PRs and design discussions that extend the plugin API instead of adding one-off
core behavior." The boundary is the plugin API surface; if the plugin API can't
express a capability, the right fix is extending the plugin API, not adding core
behavior.

### Plugin SDK surface

The SDK is narrow and explicit. `docs/plugins/sdk-overview.md` lists 200+
subpath exports in `scripts/lib/plugin-sdk-entrypoints.json`; imports must come
from specific subpaths (`openclaw/plugin-sdk/plugin-entry`,
`openclaw/plugin-sdk/channel-core`, etc.) to avoid circular imports and keep
startup fast. The registration API on `OpenClawPluginApi`:

- Capability: `registerProvider`, `registerChannel`, `registerSpeechProvider`,
  `registerRealtimeTranscriptionProvider`, `registerRealtimeVoiceProvider`,
  `registerMediaUnderstandingProvider`, `registerImageGenerationProvider`,
  `registerMusicGenerationProvider`, `registerVideoGenerationProvider`,
  `registerWebFetchProvider`, `registerWebSearchProvider`
- Tools/commands: `registerTool`, `registerCommand`
- Infrastructure: `registerHook`, `registerHttpRoute`, `registerGatewayMethod`,
  `registerGatewayDiscoveryService`, `registerCli`, `registerService`,
  `registerInteractiveHandler`, `registerAgentToolResultMiddleware`
- Memory: `registerMemoryPromptSupplement`, `registerMemoryCorpusSupplement`
- Workflow/lifecycle: `registerSessionExtension`, `enqueueNextTurnInjection`,
  `registerTrustedToolPolicy`, `registerToolMetadata`, `registerControlUiDescriptor`,
  `registerRuntimeLifecycle`, `registerAgentEventSubscription`, `setRunContext`,
  `registerSessionSchedulerJob`

The SDK is explicitly typed via TypeBox schemas throughout.

---

## 6. Anti-patterns and explicit non-goals

### "What We Will Not Merge" (VISION.md)

Verbatim:

> - New core skills when they can live on [ClawHub](https://clawhub.ai/)
> - Full-doc translation sets for all docs (deferred; we plan AI-generated
>   translations later)
> - Commercial service integrations that do not clearly fit the model-provider
>   category
> - Wrapper channels around already supported channels without a clear capability
>   or security gap
> - MCP work that duplicates existing MCP, ACPX, plugin, or ClawHub paths without
>   a clear product or security gap
> - **Agent-hierarchy frameworks (manager-of-managers / nested planner trees) as
>   a default architecture**
> - **Heavy orchestration layers that duplicate existing agent and tool
>   infrastructure**

The architectural items (last two) are the load-bearing ones. The rationale is
implicit in the design rather than explicitly stated in VISION.md — the closest
rationale is: the tool infrastructure and agent runtime already exist; adding a
coordination layer on top of them would duplicate their functionality without
adding value. This is an engineering-minimalism argument, not a principled
rejection of multi-agent coordination per se.

The caveats matter: "as a **default** architecture" and "for **now**." This list
is described as "a roadmap guardrail, not a law of physics. Strong user demand
and strong technical rationale can change it." So unlike Cognition's June 2025
"Don't Build Multi-Agents" (which was an analytical claim), openclaw's rejection
is explicitly framed as a priority decision, not a categorical prohibition.

### Other anti-patterns from docs

From `docs/concepts/agent-loop.md`:
- Plugins that prompt-inject (`allowPromptInjection=false` exists as a policy
  flag to disable prompt-mutating hooks including `before_prompt_build`)
- Reserved core admin namespaces (`config.*`, `exec.approvals.*`, `wizard.*`,
  `update.*`) — always stay `operator.admin`, even if a plugin tries to override

From `docs/tools/plugin.md`:
- Plugin config that fails validation: fails closed, logs the error, skips that
  plugin while keeping others online. "openclaw doctor --fix" quarantines bad
  plugin config. The fail-closed default is security-significant.
- Workspace-origin plugins are **disabled by default** (must be explicitly
  enabled) — untrusted local scripts are not auto-loaded.

From `docs/concepts/multi-agent.md`:
- Agent-to-agent messaging is **disabled by default** (`tools.agentToAgent.enabled: false`)
  and requires explicit allowlisting. Even in multi-agent setups, cross-agent
  messaging requires deliberate operator choice.
- Direct chats collapse to the agent's main session key, so "true isolation
  requires one agent per person" for multi-user setups.

From `docs/concepts/session.md`:
- DM isolation is opt-in; without it "all users share the same conversation
  context — Alice's private messages would be visible to Bob." The system leans
  toward showing this risk rather than hiding it.

From `docs/plugins/sdk-channel-plugins.md`:
- Channels should not send their own "approval went to DMs / another channel"
  messages — core owns the approval reroute notices. Authority split is a
  deliberate anti-fragmentation principle.

### Implicit anti-patterns observable from code structure

The `src/` directory (60+ subdirectories) suggests organic growth rather than
designed modularity, but the plugin SDK is the explicit boundary. The presence
of `src/plugin-activation-boundary.test.ts` as a test file in the root of `src/`
suggests activation boundary enforcement is actively tested. The presence of
`src/global-state.ts` and `src/globals.ts` at the root of `src/` suggests some
process-level global state does exist (implementation-backed observation — the
file exists; its contents not read).

---

## 7. Anchoring caveats

### Differences that discount transfer

**Single-user personal assistant vs. autonomous public-repo orchestrator.**
openclaw is designed for one human's personal channels. The delegate
architecture extends this to small organizations but still assumes a human
operator managing the Gateway. The redesign's orchestrator has no human operator
in the loop except via GitHub issues at low frequency. openclaw's security
defaults (pairing, DM approval, sandboxing) are human-mediated; the redesign
needs the analogue for no-human-in-the-loop. Specific patterns that don't
transfer: DM pairing flow, human takeover as recovery mechanism, operator-managed
allowlists.

**Human-in-the-loop on every interaction vs. autonomous cron.** openclaw's
queue modes (`steer`, `collect`, `followup`) are designed around human message
arrival patterns. The redesign's orchestrator wakes from cron. openclaw's cron
features exist (dedicated cron lane, `sessions_spawn`, standing orders) but are
secondary to the interactive use case. The `cron` tool being denied in the
default sandbox is the clearest marker of priority: cron is a power-user feature
in openclaw; it's the primary trigger in the redesign.

**TypeScript-first vs. Rust-for-tools.** openclaw is a Node.js process (requires
Node 22+). The redesign mandates Rust for deterministic tools. The plugin SDK
pattern is transferable in principle (a typed registration API for capability
extension) but the specific SDK (`openclaw/plugin-sdk/*`) is TypeScript-native.
Patterns transfer: plugin registration model, explicit capability contract,
two-tier extension (runtime hooks vs. stable bundle-style). Implementation
does not transfer.

**Local machine vs. GitHub Actions ephemeral runners.** openclaw's persistence
model (Markdown files in `~/.openclaw/`, SQLite indices, per-agent directories)
assumes a persistent filesystem. GitHub Actions runners are ephemeral;
`~/.openclaw/` doesn't survive across runs. The per-agent file structure pattern
is relevant but the specific storage implementation is not.

**Star count anomaly.** 365k★ / 75k forks for a 5-month-old repo (created
2025-11-24) is statistically anomalous. The docs are detailed and the source is
real TypeScript with real tests, so the architectural claims are grounded — but
external validation of design choice consequences is weaker than a mature project
would provide. The "EXFOLIATE!" / lobster theming is self-conscious humor, not
evidence of parody, but the unusual growth rate warrants calibrated confidence on
"has this been battle-tested in production" claims. Treat the architecture as
real and the consequence-verification as limited.

### Patterns that transfer despite the differences

**Plugin registration model as extension contract.** The `register(api)` callback
pattern with explicit typed SDK subpaths is directly transferable as an
architecture shape. A Rust analogue would be a registration trait with explicit
capability declarations. The two-tier split (code plugins with runtime hooks vs.
bundle-style plugins with stable surfaces) maps well to a tool-binary + manifest
pattern.

**Per-session serialization + cross-session parallelism.** The FIFO lane-per-
session queue (serialize within a session, allow parallelism across sessions) is
transferable to any multi-channel orchestrator. For the redesign, "session" maps
roughly to "issue/PR" — serialize work within one issue's conversation, allow
parallel work across different issues.

**Fail-closed plugin loading.** Invalid plugin config is isolated to that plugin;
other plugins and channels stay online. This fail-closed, recover-with-doctor
pattern is directly transferable to tool loading in the redesign.

**Memory as separate plugin slot.** The singleton slot constraint (one active
memory plugin at a time, replaceable) is a useful design primitive for avoiding
multiple competing memory subsystems. The redesign currently has organic markdown
files; formalizing the memory surface as a single swappable mechanism is
transferable.

**Anti-pattern list as deliverable artifact.** VISION.md's "What We Will Not
Merge" list is a deliverable artifact that gates PRs. The redesign's Phase 2
framework already has convergent constraint 5 (anti-patterns documented as
deliverable artifact); openclaw is the clearest implementation of this pattern
in the surveyed systems.

**Strong-defaults security.** The pairing-required-by-default DM policy,
workspace-plugins-disabled-by-default, and agent-to-agent-disabled-by-default
are all applications of the "default-deny, explicit-allow" security principle.
This transfers directly to the redesign's tool access model. The specific
mechanism (DM pairing) doesn't transfer, but the principle (operator must
explicitly unlock high-power paths) does.

**Delegate architecture as organizational pattern.** The delegate model (agent
with its own identity acting on behalf of humans, with explicit delegation
permissions) is a useful archetype even without a human in the loop — it
describes the trust structure for the redesign's orchestrator acting on behalf
of the project. The tier-1/tier-2/tier-3 capability escalation (read-only →
send-on-behalf → autonomous with standing orders) maps to tool permission
escalation in the redesign.

---

## Comparison to cycle-14 stub

The cycle-14 stub (71 lines, README+VISION only) captured the following, which
this deeper read **confirms**:

- Local-first Gateway as single control plane — **confirmed** (architecture.md
  primary source)
- Multi-channel inbound (25+ messaging surfaces) — **confirmed** (README lists
  25+ channels)
- Multi-agent routing to isolated agents with workspaces + per-agent sessions —
  **confirmed and extended** (multi-agent.md, delegate-architecture.md)
- Plugin API with two styles (code plugins / bundle-style) — **confirmed and
  extended** (sdk-overview.md, building-plugins.md)
- Memory as singleton plugin slot — **confirmed** (VISION.md verbatim, plus
  plugin SDK showing `plugins.slots.memory`)
- "Treat inbound DMs as untrusted input" — **confirmed**
- Default sandbox allows `bash`, `process`, `read`, `write`, `edit`, `sessions_*`;
  denies `browser`, `canvas`, `nodes`, `cron`, `discord`, `gateway` — **confirmed**
- "What We Will Not Merge" anti-patterns — **confirmed verbatim**
- Strong-defaults security posture — **confirmed and extended**
- Plugin promotion bar "intentionally high" — **confirmed verbatim**

The cycle-14 stub **missed or understated**:

- The Gateway is per-event request-response (not a streaming event pipeline) —
  **new finding**
- The queue architecture: per-session FIFO lanes with global concurrency cap —
  **new finding**
- Agents are per-request invocations, not long-running daemons — **new finding**
- The `steer`/`collect`/`followup` queue modes for multi-message handling —
  **new finding**
- The specific plugin SDK surface (200+ subpaths, TypeBox-typed, explicit
  registration API) — **new finding**
- The memory system structure: MEMORY.md + daily notes + dreaming + active-memory
  plugin — **new finding**
- The delegate architecture pattern (agent-as-named-delegate for org deployments)
  — **new finding**
- The session lifecycle (daily reset, idle reset, fresh-per-cron) — **new finding**
- `agents.defaults.timeoutSeconds` default of 172800s (48 hours) — **new finding**
- Agent-to-agent messaging disabled by default — **new finding**
- Workspace-origin plugins disabled by default — **new finding**
- `src/global-state.ts` exists (some process-level global state) — **new finding
  (implementation-backed, contents not read)**

---

## Phase 2 framework anchoring

Specific axis positions in `docs/redesign/2-design-framework.md` affected by
this deeper read:

### Axis 3 (Memory subsystem) — QUALIFIED

Framework position: "Singleton plugin slot (one mechanism active, replaceable)
| openclaw"

**Strengthened**: VISION.md verbatim confirms singleton slot. DEEPER: the memory
backends are multiple (builtin SQLite, QMD, Honcho, LanceDB), each a swappable
plugin, but only one active at a time. The active-memory plugin is a separate
optional layer (pre-reply sub-agent), not the memory slot itself. Memory files
are Markdown (MEMORY.md + daily notes); the plugin owns indexing and search.
This is implementation-backed, not just documented-claim.

**Qualification**: "Singleton" correctly describes the constraint but
understates the depth. openclaw's memory system has: file-level persistence
(MEMORY.md + daily notes + DREAMS.md), SQLite index, embedding-based hybrid
search, active-memory sub-agent, dreaming background consolidation. This is not
a single primitive — it's a layered system with a single swappable storage
backend. The "singleton plugin slot" is the storage/retrieval layer; the rest
of the memory architecture (file conventions, dreaming, active-memory) is built
on top. The framework's row is accurate but the depth is greater than the label
suggests.

### Axis 6 (Extension shape) — STRENGTHENED

Framework position: "Plugins | openclaw | 'Core stays lean; optional capability
ships as plugins'"

**Strengthened**: The plugin SDK is concrete (200+ subpaths, TypeBox-typed,
explicit registration). The two-tier split (code plugins vs. bundle-style) is
implemented and documented with rationale. The ClawHub discovery mechanism
provides plugin provenance and security review outside core. This is
implementation-backed.

### Axis 7 (Orchestration topology) — NEW EVIDENCE

Framework: cycle-14 read did not anchor openclaw on Axis 7.

**New evidence**: The multi-agent routing system (`bindings` + `agentId` scoping
+ per-session lane serialization) suggests **lead-worker** or **peer** topology,
but this is misleading. The agents in openclaw's multi-agent routing are
**isolated personas** (different workspaces, identities, contexts), not
coordinating agents. There is no coordinator-worker relationship; each agent
operates independently. Agent-to-agent messaging is disabled by default and
requires explicit configuration.

For Axis 7's **orchestration topology** question: within a single-agent setup
(the common case), there is no multi-agent topology — one agent handles all
inbound messages to its bindings. In multi-agent routing, agents run
independently without cross-agent coordination. This does not map cleanly to
any Axis 7 row as defined. The closest is "single-pattern (one shape only)" for
the per-agent perspective, with multi-agent routing being more like "routing to
isolated workers" than "orchestration topology" in the Axis 7 sense.

openclaw explicitly rejects the "manager-of-managers" and "orchestrator-worker
hierarchy" patterns as defaults. The multi-agent routing is flat: inbound routes
to agent, agent runs, agent responds. No coordinator.

### Axis 12 (Reconciliation discipline) — CONFIRMATORY + CORRECTION

Framework: "async-with-server (continuous-runtime): openclaw multi-channel —
UNDER-VERIFIED"

**Primary-source answer to Q(c)**: openclaw's Gateway is **always-on (daemon)
with per-event discrete agent turns** — not a streaming event-ingestion
continuous-runtime pipeline. The correct framing for Axis 12 purposes is:

- The Gateway daemon is continuous-runtime (always-on).
- Agent execution is per-event request-response (discrete turns triggered by
  inbound events).
- Inbound events arrive via persistent channel connections (no cron polling for
  new messages).

This is **structurally distinct from async-with-cron** (cron polls for new
events; openclaw's channels push events as they arrive). It is **not a
streaming event-ingestion pipeline** (events don't flow through a stream; each
triggers a discrete queue item). The best label for the Axis 12 table is:
"**event-driven with persistent connections** (channels maintain always-on
connections; Gateway handles events discretely as they arrive; no inbound
polling)." This is a third pattern distinct from both async-with-cron and
synchronous-HITL, but less dramatic than "continuous-runtime streaming" would
suggest.

**Implication for Axis 12**: The three-pattern framing (synchronous HITL,
async-with-cron, async-with-server) survives, but "async-with-server" should
be relabeled "event-driven with persistent connections" to avoid the inference
that agents run continuously. The Gateway's daemon nature is the "server"
element; the event-driven per-turn architecture is the mechanism. The fourth
row in the Axis 12 table ("No reconciliation: write-only outbound channels" —
the v1 anti-pattern) still exists; openclaw represents a different position
that the framework should distinguish clearly from v1.

**Cross-channel concurrency answer**: When Discord and Slack messages arrive
simultaneously, if routed to the same session they queue in per-session FIFO
order; if routed to different sessions (the typical multi-agent case) they run
in parallel up to `maxConcurrent`. This is well-defined and implementation-backed.

### Convergent constraint 3 (Strong-defaults security) — STRENGTHENED

The deeper read confirms and extends the cycle-14 observation. Three specific
patterns are implementation-backed:
1. DM pairing required by default (operator explicitly approves each sender)
2. Workspace-origin plugins disabled by default
3. Agent-to-agent messaging disabled by default

All three are "default-deny, explicit-allow" applied to different capability
surfaces.

### Convergent constraint 5 (Anti-patterns as deliverable artifact) — CONFIRMED

VISION.md's "What We Will Not Merge" is verbatim accessible and the caveat
is notable: "This list is a roadmap guardrail, not a law of physics." This is
a more honest framing than an absolute prohibition — the list is versioned and
revisable, not inscribed. The redesign's equivalent should similarly be
versioned and revisable with explicit rationale for changes.

---

## Patterns observed in openclaw

*(Relevance evaluation for v2 candidate generation deferred to Phase 2.)*

- Single-process Gateway daemon as unified control plane for all channel surfaces
- Event-driven per-session FIFO queue (serialize within session, parallelize
  across sessions)
- Per-request agent invocations within a long-running daemon
- TypeBox schemas for protocol typing and code generation
- Two-tier plugin model: code plugins (runtime hooks, in-process) vs.
  bundle-style plugins (stable external surfaces)
- Plugin registration via typed `register(api)` callback with explicit subpath
  SDK imports
- Memory as singleton plugin slot with swappable backends
- Memory system: Markdown files (MEMORY.md + daily notes) + SQLite index +
  embedding-based hybrid search + active-memory sub-agent + dreaming background
  consolidation
- Per-agent state isolation: separate workspace, agentDir, session store per agent
- Session as conversation thread with lifecycle timestamps (daily reset, idle
  reset, fresh-per-cron)
- Binding-based deterministic routing: most-specific-wins resolution hierarchy
- Default-deny security: pairing required for DMs, workspace plugins disabled by
  default, agent-to-agent messaging disabled by default
- Fail-closed plugin loading: invalid plugin config isolated, others stay online
- Operator-escalation tiers: capability tiers (read-only → send-on-behalf →
  autonomous with standing orders) with explicit permission grants
- Plugin promotion to ClawHub: discovery, official publisher status, provenance,
  security review outside core
- Memory flush before compaction (silent turn saves context before session
  summarization)
- Queue mode `steer` as default: new messages routed into active run, not
  queued as separate turns
- Tool policy at Gateway level: per-agent `tools.allow`/`tools.deny` enforced
  independently of agent prompt rules
- Anti-pattern list ("What We Will Not Merge") as versioned, revisable artifact
  with explicit caveat about non-permanence
- `global-state.ts` at `src/` root — process-level global state exists
  (implementation-backed; contents not read; warrants further inspection)
