# openclaw (openclaw/openclaw, 365k★ / 75k forks / TypeScript)

[← back to Phase 1 index](../../1-research.md)

**Status: deeper read landed (cycle 43).** Initial Copilot research dispatch
landed in cycle 14 as `README.md` + `VISION.md` only (71 lines stub).
Cycle-41 Q(c) BORDERLINE-PASS flag motivated the cycle-42 close-and-recreate
re-dispatch ([#2808](https://github.com/EvaLok/schema-org-json-ld/issues/2808),
fired ~4 min after issue creation per cycle-42 verification — third
confirmation of the close-and-recreate primitive). Cycle-43 deliverable in
PR [#2809](https://github.com/EvaLok/schema-org-json-ld/pull/2809) at
`docs/redesign/_notes/cycle-42-openclaw-deeper-read.md` (893 lines,
primary-source-grounded; no firewall blocks encountered on
`github.com/openclaw` or `raw.githubusercontent.com`). Cycle-43 per-finding
evaluation:
[`_notes/cycle-43-openclaw-per-finding-evaluation.md`](../../_notes/cycle-43-openclaw-per-finding-evaluation.md).

## Sources read

Cycle 43 deeper read (direct primary-source access from openclaw/openclaw
main branch):

- `README.md` (full read)
- `VISION.md` (full read)
- `docs/concepts/architecture.md`
- `docs/concepts/agent-loop.md`
- `docs/concepts/queue.md`
- `docs/concepts/memory.md`, `memory-builtin.md`, `active-memory.md`
- `docs/concepts/session.md`
- `docs/concepts/multi-agent.md`
- `docs/concepts/agent.md`
- `docs/concepts/delegate-architecture.md`
- `docs/tools/plugin.md`
- `docs/plugins/sdk-overview.md`, `building-plugins.md`,
  `sdk-channel-plugins.md`
- `src/` directory listing (GitHub tree view; not all source files read)
- `docs/` and `docs/gateway/` directory listings
- `scripts/lib/plugin-sdk-entrypoints.json` (SDK subpath enumeration)

Limitations: `docs/gateway/security/` is a subdirectory (multiple files);
`docs/gateway/sandboxing.md` referenced but not fully read; security model
captured from README + VISION + concept docs that were read in full. Source
files in `src/` (e.g., `src/global-state.ts`) confirmed by directory listing
but contents not read.

## Load-bearing thesis

openclaw's central thesis (README + VISION):

> "OpenClaw is a personal AI assistant you run on your own devices. [...]
> The Gateway is just the control plane — the product is the assistant."

Three named positions anchor the design:

- **Local-first.** The Gateway runs on the user's machine, not in a cloud.
  One Gateway per host. Companion apps (macOS, iOS, Android) connect back to
  the local Gateway. Remote access is SSH tunnel or Tailscale VPN — not a
  hosted backend.
- **Small core, plugins extend.** "Core stays lean; optional capability
  should usually ship as plugins. We are generally slimming down core while
  expanding what plugins can do." Plugin promotion to core has
  "intentionally high" bar — discovery, provenance, security review live in
  ClawHub (external).
- **Treat inbound as untrusted.** Default DM policy is `pairing` (unknown
  senders receive a pairing code; bot does not process their message until
  explicitly approved). Workspace-origin plugins disabled by default.
  Agent-to-agent messaging disabled by default. The pattern is *default-
  deny, explicit-allow* applied across multiple capability surfaces.

The "What We Will Not Merge" anti-pattern list (VISION.md) explicitly
rejects:
- "Agent-hierarchy frameworks (manager-of-managers / nested planner trees)
  as a default architecture"
- "Heavy orchestration layers that duplicate existing agent and tool
  infrastructure"

**Important caveat (cycle-43 deeper read):** the rejection is framed as
"a roadmap guardrail, not a law of physics. Strong user demand and strong
technical rationale can change it." This is a more honest framing than an
absolute prohibition — the list is versioned and revisable, not inscribed.
Compare to Cognition's June 2025 "Don't Build Multi-Agents" which read as
absolute and was substantially walked back in April 2026.

## Architecture: Gateway, Channels, Plugins

### Gateway as single-process daemon

The Gateway is a single Node.js process — one daemon per machine, not a
registry of services. Serves:
- WebSocket API on `127.0.0.1:18789` (default) for control-plane clients
  (macOS app, CLI, web UI, automations) and Nodes (iOS/Android/headless)
- HTTP server under the same port for the Canvas host
  (`/__openclaw__/canvas/`, `/__openclaw__/a2ui/`)

Protocol typed via TypeBox schemas; JSON Schema and Swift models generated
from those schemas. First frame must be a `connect`; non-JSON or non-connect
first frames hard-close.

### Channels as plugins

Channels (WhatsApp, Telegram, Slack, Discord, Signal, iMessage, WebChat,
etc.) are plugins registered via `defineChannelPluginEntry({ id, ...,
register(api) { api.registerChannel(...) }})`. Channel plugins own:
config, security (DM policy + allowlists), pairing, session grammar
(provider-specific conversation IDs → base chats + thread IDs + parent
fallbacks), outbound, threading, heartbeat-typing.

Core owns: shared `message` tool, prompt wiring, outer session-key shape,
generic `:thread:` bookkeeping, dispatch.

### Plugin SDK surface (cycle-43 finer detail)

200+ subpath exports in `scripts/lib/plugin-sdk-entrypoints.json`; imports
must come from specific subpaths (`openclaw/plugin-sdk/plugin-entry`,
`openclaw/plugin-sdk/channel-core`, etc.) to avoid circular imports and
keep startup fast. The registration API on `OpenClawPluginApi` covers
16+ named methods across:
- Capability: `registerProvider`, `registerChannel`, speech/transcription/
  voice/media/image/music/video/web-fetch/web-search providers
- Tools/commands: `registerTool`, `registerCommand`
- Infrastructure: `registerHook`, `registerHttpRoute`,
  `registerGatewayMethod`, `registerCli`, `registerService`,
  `registerInteractiveHandler`, `registerAgentToolResultMiddleware`
- Memory: `registerMemoryPromptSupplement`,
  `registerMemoryCorpusSupplement`
- Workflow/lifecycle: `registerSessionExtension`,
  `enqueueNextTurnInjection`, `registerTrustedToolPolicy`,
  `registerToolMetadata`, `registerControlUiDescriptor`,
  `registerRuntimeLifecycle`, `registerAgentEventSubscription`,
  `setRunContext`, `registerSessionSchedulerJob`

Two-tier plugin model: code plugins (TypeScript with runtime hooks,
in-process via `register(api)`) vs bundle-style plugins (Codex/Claude/
Cursor-compatible layouts; smaller stable interface, better security
boundaries). VISION.md: "Prefer bundle-style plugins when they can express
the capability."

Plugin discovery order (first match wins): `plugins.load.paths` (explicit
config) → workspace plugins (`<workspace>/.openclaw/...`) → global plugins
(`~/.openclaw/...`) → bundled plugins.

Fail-closed loading: invalid plugin config is isolated to that plugin;
other plugins and channels stay online. `openclaw doctor --fix` quarantines
bad plugin config.

## Multi-agent architecture (cycle-43 finer detail)

### Bindings (deterministic routing)

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

### Per-agent state isolation

Per-agent state: `~/.openclaw/agents/<agentId>/agent/` (auth profiles,
model registry, per-agent config). Global config:
`~/.openclaw/openclaw.json`. Session transcripts: per-session JSONL.
Memory index: per-agent SQLite (`~/.openclaw/memory/<agentId>.sqlite` for
the builtin backend).

**There is no global state file analogous to v1's `state.json`** — state
is per-agent, per-session files. *(Cycle-43 caveat: `src/global-state.ts`
and `src/globals.ts` exist at the root of `src/` — implementation-backed
observation that some process-level global state does exist; contents not
read in cycle-43; warrants further inspection.)*

### Routing pattern: flat isolated personas, NOT topology

**Important framework distinction (cycle-43):** openclaw's multi-agent
system is **flat routing to isolated personas**, NOT orchestration topology
in the Axis 7 sense. Each agent runs independently — no coordinator-worker
relationship. Agent-to-agent messaging is disabled by default
(`tools.agentToAgent.enabled: false`) and requires explicit configuration.

Within a single-agent setup (the common case), there is no multi-agent
topology — one agent handles all inbound messages to its bindings. In
multi-agent routing, agents run independently without cross-agent
coordination.

This does not map cleanly to Axis 7's positions (single-pattern,
multi-pattern coexisting, sequential mode transitions, lead-worker,
peer-flow). The closest is "single-pattern" for the per-agent perspective,
with multi-agent routing being more like "routing to isolated workers"
than "orchestration topology."

## Async, concurrency, and event handling (cycle-43 finer detail)

### Queue architecture

`docs/concepts/queue.md`: "We serialize inbound auto-reply runs (all
channels) through a tiny in-process queue to prevent multiple agent runs
from colliding, while still allowing safe parallelism across sessions."

The queue is:
- **Lane-aware FIFO**: each session has a per-session lane
  (`session:<key>`); all sessions also share a global lane (`main` by
  default)
- **Concurrency cap**: default lane concurrency is 1 (unconfigured lanes);
  `main` defaults to 4, `subagent` to 8; overall cap via
  `agents.defaults.maxConcurrent`
- **Additional lanes**: `cron`, `cron-nested`, `nested`, `subagent` for
  background jobs without blocking inbound replies

**Cross-channel concurrency answer:** When Discord and Slack messages
arrive simultaneously and route to the same session, they queue in
per-session FIFO order; if they route to different sessions (the typical
multi-agent case), they run in parallel up to `maxConcurrent`. This is
**per-session serialization + cross-session parallelism**.

### Per-event request-response, NOT streaming event-ingestion

This is the load-bearing answer to the cycle-41 Q(c) BORDERLINE-PASS flag
on Axis 12 reconciliation framing.

The Gateway daemon is always-on; channels maintain persistent upstream
connections (WhatsApp via Baileys, Telegram via grammY, etc.); those
channel libraries deliver message events to the Gateway as they arrive
(not via Gateway-side polling). But each message event triggers a
**discrete agent turn** routed through the per-session FIFO queue.
The agent loop (`runEmbeddedPiAgent`) is invoked per-turn, not as a
continuously-running coroutine.

`docs/concepts/architecture.md` Invariants section: "Events are not
replayed; clients must refresh on gaps." This is request-response
semantics, not a streaming event-ingestion log.

**Implication for the framework:** openclaw's Axis 12 position is
"event-driven with persistent connections" — an instance of Axis 12's
existing **Event-driven** position with implementation-specific details.
The cycle-40 v2-design observation about "three reconciliation patterns"
(sync HITL / async-with-cron / async-with-server) was over-extended:
openclaw's pattern is implementation-detail within an existing axis
position, not a new axis position. The cross-system observation refines
to TWO axis-distinct patterns (sync HITL vs async) with implementation-
nuance within async (cron+catchup, event-driven with persistent
connections, webhook-on-event).

### Long-running operations and timeouts

- Agent runtime: `agents.defaults.timeoutSeconds` default 172800s (48h) —
  effectively unbounded for typical use
- Cron runtime: isolated per-run `timeoutSeconds` owned by cron scheduler
- `agent.wait` default: 30s (wait-only, does not stop agent)
- Model idle timeout: aborts when no response chunks arrive within idle
  window
- **Stuck-session watchdog** (`diagnostics.stuckSessionWarnMs`): detects
  stale lanes and can release them — the more interesting Axis 9 primitive
  than the bare timeout

## Memory architecture (cycle-43 finer detail)

### Singleton plugin slot — but layered above

VISION.md: "Memory is a special plugin slot where only one memory plugin
can be active at a time."

The slot is selected via `plugins.slots.memory`. Default: `memory-core`
(builtin SQLite backend). Alternatives: `memory-lancedb`, Honcho
(cross-session AI-native), QMD (local sidecar).

**Important framework distinction (cycle-43):** The "singleton plugin
slot" describes the **storage/retrieval layer**, not the entire memory
architecture. The full architecture is layered:

1. **File-level persistence:** Markdown files in workspace
   - `MEMORY.md` — long-term durable facts, preferences, decisions
     (loaded at start of every DM session)
   - `memory/YYYY-MM-DD.md` — daily notes (today + yesterday auto-loaded)
   - `DREAMS.md` — Dream Diary + dreaming sweep summaries (human-review
     surface)
2. **Index layer:** SQLite with embedding-based hybrid (vector + keyword)
   search
3. **Active-memory sub-agent** (optional): runs before main reply,
   surfaces relevant memory proactively, bounded by `timeoutMs: 15000`,
   injects result as "untrusted context" prefix
4. **Dreaming background consolidation** (optional): cron-driven
   consolidation pass; promotions from short-term `memory/.dreams/` to
   `MEMORY.md` are gated on score, recall frequency, and query diversity
   thresholds; `memory-core` auto-manages the cron job

The active memory plugin provides two agent tools: `memory_search`
(semantic search) and `memory_get` (read specific memory file or line
range).

### Cross-channel memory access

Agents access memory through the session's memory tools, which search the
per-agent index (`~/.openclaw/memory/<agentId>.sqlite`). Multiple channels
routing to the same agent see the same memory. Cross-agent memory access
is possible via `memorySearch.qmd.extraCollections` (QMD backend only).

## Session architecture and lifecycle

`docs/concepts/session.md`: Sessions are conversation threads; routing key
is `agent:<agentId>:<mainKey>`. Each message is routed to a session based
on its source:

| Source | Behavior |
|---|---|
| Direct messages | Shared session by default (`main`) |
| Group chats | Isolated per group |
| Rooms/channels | Isolated per room |
| Cron jobs | Fresh session per run |
| Webhooks | Isolated per hook |

State at `~/.openclaw/agents/<agentId>/sessions/sessions.json`;
transcripts at `<sessionId>.jsonl`. Lifecycle timestamps:
`sessionStartedAt` (daily reset), `lastInteractionAt` (idle reset),
`updatedAt` (general bookkeeping).

Session lifecycle: daily reset at 4:00 AM local time by default; idle
reset configurable; manual `/new` or `/reset`. **"Heartbeat, cron, and
exec system events do not keep the session alive"** for idle reset
purposes — only real user/channel interactions extend idle freshness.

## Tool / skill / plugin distinction

Three terms with distinct meanings:

- **Tools**: capabilities the agent can invoke as function calls during a
  run. Core tools (`read`, `exec`, `edit`, `write`, `apply_patch`) always
  available. Plugin tools registered via `api.registerTool(...)`.
- **Skills**: Markdown files in workspace (`skills/`) injected into
  agent's system prompt. Guidance and conventions, not executable code.
  Three tiers: bundled (shipped), managed (`~/.openclaw/skills`),
  workspace (per-agent).
- **Plugins**: npm packages or local directories that extend Gateway
  process. Two styles: code plugins (runtime hooks, in-process) vs
  bundle-style plugins (Codex/Claude/Cursor-compatible, smaller
  interface).

## Sandbox and tool policy

Default sandbox (`agents.defaults.sandbox.mode: "non-main"`):
- Backend: Docker default (SSH and OpenShell alternatives)
- **Allow** (non-main sessions): `bash`, `process`, `read`, `write`,
  `edit`, `sessions_*`
- **Deny** (non-main sessions): `browser`, `canvas`, `nodes`, `cron`,
  `discord`, `gateway`

**Important transfer signal (cycle-43):** the default sandbox **denies
`cron`**. Cron is available in the main session (unsandboxed) and where
explicitly enabled. *Cron is a power-user feature in openclaw; it's the
primary trigger in the redesign.* This priority inversion is a load-bearing
caveat for transfer analysis.

Per-agent tool policy (v2026.1.6+): `agents.list[].tools.allow` /
`agents.list[].tools.deny` enforces at Gateway level, independent of
agent's prompt-level rules. The delegate architecture doc shows this in
practice. Mechanism: tool call arrives → checked against per-agent policy
→ checked against plugin `before_tool_call` hooks → checked against
trusted tool policy → executed.

`before_tool_call: { block: true }` is terminal;
`before_tool_call: { requireApproval: true }` pauses and prompts.

**Reserved core admin namespaces:** `config.*`, `exec.approvals.*`,
`wizard.*`, `update.*` always stay `operator.admin`, even if a plugin
tries to override.

## Capability escalation tiers (delegate architecture)

`docs/concepts/delegate-architecture.md` describes the delegate model:
agent with its own identity acting on behalf of humans, with explicit
delegation permissions. Tier-1/tier-2/tier-3 capability escalation:
- Tier 1: read-only
- Tier 2: send-on-behalf
- Tier 3: autonomous with standing orders

Each tier with explicit permission grants. The pattern transfers to v2
candidate's permission discipline.

## Anchoring caveats — what does and does not transfer

### Differences that discount transfer

**Single-user personal assistant vs autonomous public-repo orchestrator.**
openclaw is designed for one human's personal channels. The delegate
architecture extends this to small organizations but still assumes a
human operator managing the Gateway. The redesign's orchestrator has no
human operator in the loop except via GitHub issues at low frequency.
openclaw's security defaults (pairing, DM approval, sandboxing) are
human-mediated; the redesign needs the analogue for no-human-in-the-loop.

**Human-in-the-loop on every interaction vs autonomous cron.** openclaw's
queue modes (`steer`, `collect`, `followup`) are designed around human
message arrival patterns. The redesign's orchestrator wakes from cron.
openclaw's cron features exist (dedicated cron lane, `sessions_spawn`,
standing orders) but are **secondary** to the interactive use case. The
`cron` tool being denied in the default sandbox is the clearest marker of
priority.

**TypeScript-first vs Rust-for-tools.** openclaw is a Node.js process
(requires Node 22+). The redesign mandates Rust for deterministic tools.
The plugin SDK pattern is transferable in principle (typed registration
API for capability extension); the specific SDK is TypeScript-native.

**Local machine vs GitHub Actions ephemeral runners.** openclaw's
persistence model (Markdown files in `~/.openclaw/`, SQLite indices,
per-agent directories) assumes a persistent filesystem. GitHub Actions
runners are ephemeral.

**Star count anomaly + repo age.** 365k★ / 75k forks for a 5-month-old
repo (created 2025-11-24) is statistically anomalous. The docs are
detailed and the source is real TypeScript with real tests, so the
**architectural claims are grounded** — but external validation of design
choice consequences is weaker than a mature project would provide. Treat
the architecture as real and consequence-verification as limited.
"EXFOLIATE!" / lobster theming is self-conscious humor, not parody;
unusual growth rate warrants calibrated confidence on "battle-tested"
claims.

### Patterns that transfer despite the differences

**Plugin registration model as extension contract.** The `register(api)`
callback pattern with explicit typed SDK subpaths transfers as an
architecture shape. A Rust analogue would be a registration trait with
explicit capability declarations. The two-tier split (code plugins with
runtime hooks vs bundle-style plugins with stable surfaces) maps to a
tool-binary + manifest pattern.

**Per-session serialization + cross-session parallelism.** The FIFO
lane-per-session queue (serialize within session, allow parallelism
across sessions) is transferable to any multi-channel orchestrator. For
the redesign, "session" maps roughly to "issue/PR" — serialize work
within one issue's conversation, allow parallel work across different
issues.

**Fail-closed plugin loading.** Invalid plugin config is isolated; other
plugins and channels stay online. Recover-with-doctor pattern transfers
to tool loading in the redesign.

**Memory as separate plugin slot.** Singleton slot constraint (one active
memory plugin at a time, replaceable) is a useful design primitive for
avoiding multiple competing memory subsystems. The redesign currently has
organic markdown files; formalizing the memory surface as a single
swappable mechanism is transferable.

**Anti-pattern list as deliverable artifact.** VISION.md's "What We Will
Not Merge" list is a deliverable artifact that gates PRs.
The redesign's Phase 2 framework already has convergent constraint 5
(anti-patterns documented as deliverable artifact); openclaw is the
clearest implementation of this pattern in the surveyed systems. **Note
the framing nuance:** "roadmap guardrail, not a law of physics" — the
list is versioned and revisable, not absolute (compare to Cognition's
June-2025-vs-April-2026 walkback experience).

**Strong-defaults security.** Three implementation-backed default-deny
patterns:
1. DM pairing required by default (operator approves each sender)
2. Workspace-origin plugins disabled by default
3. Agent-to-agent messaging disabled by default

All "default-deny, explicit-allow." Specific mechanism (DM pairing)
doesn't transfer; principle (operator must explicitly unlock high-power
paths) does.

**Delegate architecture as organizational pattern.** The tier-1/2/3
capability escalation maps to tool permission escalation in the redesign.

## Phase 2 framework anchoring

Specific axis positions affected by this deeper read:

### Axis 2 (State representation primitive) — STRENGTHENED

**Position:** File-per-component (per-agent state isolation in
`~/.openclaw/agents/<agentId>/`).

**Caveat (cycle-43):** Gateway-level globals exist per `src/global-state.ts`
and `src/globals.ts` (implementation-backed; contents not read).
openclaw is NOT a pure file-per-component example.

### Axis 3 (Memory subsystem) — QUALIFIED

**Position:** Singleton plugin slot (one mechanism active, replaceable).

**Qualification (cycle-43):** The singleton-slot scope is the
storage/retrieval LAYER (`plugins.slots.memory`); the full memory
architecture is layered on top: file-level persistence (Markdown +
SQLite) + active-memory sub-agent + dreaming background consolidation.
The framework's row is accurate but the depth is greater than the label
suggests.

### Axis 6 (Extension shape) — STRENGTHENED

**Position:** Plugins ("Core stays lean; optional capability ships as
plugins").

**Strengthened (cycle-43):** Plugin SDK is concrete (200+ subpath
exports, TypeBox-typed, explicit `register(api)` callback). Two-tier
split (code plugins vs bundle-style plugins) implemented and documented
with rationale. ClawHub provides plugin provenance and security review
outside core.

### Axis 7 (Orchestration topology) — NON-FIT, EXPLAINED

**Position:** openclaw is NOT in any Axis 7 row. This is correct —
multi-agent routing is **flat isolated-persona routing**, not
orchestration topology. Each agent runs independently. Agent-to-agent
messaging disabled by default.

### Axis 9 (Iteration ceilings) — RUNTIME CEILING

**Position:** Runtime ceiling (`agents.defaults.timeoutSeconds` default
172800s = 48h, effectively unbounded for typical use). The
**stuck-session watchdog** (`diagnostics.stuckSessionWarnMs`) is the more
interesting transfer-relevant primitive — detects stale lanes and can
release them.

### Axis 12 (Reconciliation discipline) — EVENT-DRIVEN INSTANCE

**Position:** Event-driven (with persistent connections; channels
maintain always-on upstream connections; agent runs are per-event
discrete turns; no inbound polling).

**Cycle-43 framework correction:** the cycle-40 v2-design observation
about "async-with-server (continuous-runtime)" was over-extended.
openclaw's pattern is implementation-detail within Axis 12's existing
**Event-driven** position, not a new axis position. The
cross-system observation refines to TWO axis-distinct patterns
(sync HITL vs async) with implementation-nuance within async
(cron+catchup, event-driven with persistent connections,
webhook-on-event).

### Axis 13 (Harness-vs-session) — PARTIAL FAT-HARNESS

**Position:** The Gateway/agent split is structurally analogous to a
harness/session split. Gateway handles (in deterministic code): channel
connections, queue management, plugin lifecycle, session routing,
sandbox enforcement, tool policy. Agents handle (in LLM session):
per-turn conversational reasoning. Much more in the Gateway than v1's
cycle-runner has — partial fat-harness instance.

### Convergent Constraint 3 (Strong-defaults security) — STRENGTHENED

Three implementation-backed default-deny patterns: DM pairing,
workspace-origin plugins, agent-to-agent messaging. openclaw is the
clearest implementation across surveyed systems.

### Convergent Constraint 5 (Anti-patterns documented) — CONFIRMED

VISION.md "What We Will Not Merge" is verbatim accessible. **Important
framing nuance:** "roadmap guardrail, not a law of physics" — versioned
and revisable, not absolute.

## Patterns observed in openclaw (cycle-43 superset)

*(Relevance evaluation deferred to Phase 2 candidate generation.)*

- Single-process Gateway daemon as unified control plane for all channel
  surfaces
- Event-driven per-session FIFO queue (serialize within session,
  parallelize across sessions; concurrency cap via `maxConcurrent`)
- Per-request agent invocations within a long-running daemon
- TypeBox schemas for protocol typing and code generation
- Two-tier plugin model: code plugins (runtime hooks, in-process) vs
  bundle-style plugins (stable external surfaces)
- Plugin registration via typed `register(api)` callback with explicit
  subpath SDK imports
- Memory as singleton plugin slot with swappable backends
- Memory system: Markdown files (MEMORY.md + daily notes + DREAMS.md) +
  SQLite index + embedding-based hybrid search + active-memory sub-agent
  + dreaming background consolidation
- Per-agent state isolation: separate workspace, agentDir, session store
  per agent
- Session as conversation thread with lifecycle timestamps (daily reset,
  idle reset, fresh-per-cron)
- Binding-based deterministic routing: most-specific-wins resolution
  hierarchy
- Default-deny security: DM pairing required, workspace plugins disabled
  by default, agent-to-agent messaging disabled by default
- Fail-closed plugin loading: invalid plugin config isolated, others stay
  online; `openclaw doctor --fix` quarantines bad plugin config
- Operator-escalation tiers: read-only → send-on-behalf → autonomous
  with standing orders, with explicit permission grants
- Plugin promotion to ClawHub: discovery, official publisher status,
  provenance, security review outside core
- Memory flush before compaction (silent turn saves context before
  session summarization)
- Queue mode `steer` as default: new messages routed into active run,
  not queued as separate turns
- Tool policy at Gateway level: per-agent `tools.allow`/`tools.deny`
  enforced independently of agent prompt rules
- Anti-pattern list ("What We Will Not Merge") as versioned, revisable
  artifact with explicit caveat about non-permanence
- Reserved core admin namespaces (`config.*`, `exec.approvals.*`,
  `wizard.*`, `update.*`) always stay `operator.admin`
- Stuck-session watchdog (`diagnostics.stuckSessionWarnMs`) — detects
  and releases stale queue lanes
- `src/global-state.ts` exists at root of `src/` — process-level global
  state (implementation-backed; contents not yet read; warrants further
  inspection)
