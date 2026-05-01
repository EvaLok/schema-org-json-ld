# Cycle 43 — openclaw deeper read per-finding evaluation

**Cycle:** 43 (2026-05-01)
**Issue:** #2810 (orchestrator-run cycle 43)
**Source:** PR #2809 — `docs/redesign/_notes/cycle-42-openclaw-deeper-read.md`
(893 lines, primary-source openclaw repo+docs read; fired ~4 min after #2808
creation cycle 42)
**Pattern:** cycle-7 / cycle-12 / cycle-31 / cycle-41 per-finding evaluation
template — read each finding individually, evaluate accept/qualify/reject,
name integration target.

## Summary

**21 findings extracted, 21 evaluated.**
- 16 ACCEPT (substantive primary-source-grounded claims, integration into
  framework or per-system file)
- 4 ACCEPT WITH QUALIFICATION (claims that need transfer-caveats or
  scope-limits before framework use)
- 1 ACCEPT AS REVISION-OF-PRIOR-CLAIM (openclaw OC1 refines the cycle-40
  v2-design observation about three reconciliation patterns; openclaw's
  pattern is "event-driven with persistent connections" within existing
  Axis 12 Event-driven position, not a new axis position)
- 0 REJECT

The deliverable's quality is high — primary-source citations throughout, the
load-bearing question (Axis 12 cycle-41 Q(c)) directly answered, and the
honest qualification on the star-count/repo-age statistical anomaly handles
calibration of "battle-tested" claims responsibly. Comparison-to-cycle-14
section establishes the deeper read is legitimately deeper (12 new findings
beyond stub).

## Findings

### OC1 — Axis 12: Gateway is event-driven per-event, not streaming

**Source claim:** "The Gateway is event-driven per-event handling, not
streaming event ingestion. Channels maintain persistent upstream connections
[...] but each message event triggers a discrete agent turn routed through a
per-session FIFO queue. The agent loop (`runEmbeddedPiAgent`) is invoked
per-turn, not as a continuously-running coroutine."

**Primary-source evidence:** `docs/concepts/architecture.md` Invariants
section ("Events are not replayed; clients must refresh on gaps" —
request-response semantics); `runEmbeddedPiAgent` is invoked per agent turn;
no continuously-running coroutine per session.

**Verdict: ACCEPT AS REVISION-OF-PRIOR-CLAIM.** The cycle-40 v2-design
observation claimed three reconciliation patterns (sync HITL,
async-with-cron, async-with-server). The deeper read refines this:
openclaw's pattern is implementation-detail within Axis 12's existing
**Event-driven** position, not a new axis position. The "persistent
connections" detail (channels maintain always-on upstream connections; no
inbound polling) distinguishes openclaw's implementation from a pure
webhook-on-event implementation, but both fit within Axis 12's existing
"Event-driven: state changes reactively when external events arrive"
position.

**Integration:**
- No new Axis 12 row needed; openclaw is an instance of "Event-driven"
  within current 4 positions.
- The cycle-40 "three patterns visible" observation should be journaled as
  refined: TWO clear axis-distinct patterns (sync HITL vs async), with
  implementation-nuance within async (cron+catchup vs event-driven with
  persistent connections vs webhook-on-event).
- Update cognition-devin.md if it referenced openclaw's "third pattern" —
  it doesn't; only this notes file does.
- openclaw per-system file should add the implementation specifics
  (persistent channel connections, per-session FIFO queue, per-event
  request-response, no streaming).

### OC2 — Axis 12 cross-channel concurrency: per-session FIFO + cross-session parallelism

**Source claim:** "Per-session serialization + cross-session parallelism."
Default lane concurrency 1 (unconfigured); main defaults 4; subagent 8;
overall cap via `agents.defaults.maxConcurrent`. Additional lanes (`cron`,
`cron-nested`, `nested`, `subagent`) for background jobs.

**Primary-source evidence:** `docs/concepts/queue.md`.

**Verdict: ACCEPT.** Implementation-backed and well-defined. Relevant for
v2 candidates that adopt event-driven reconciliation — concurrency model is
a real design choice, not always obvious.

**Integration:**
- openclaw per-system file should document the queue architecture.
- v2 candidate template should ask for concurrency model under Axis 12 if
  the candidate adopts event-driven reconciliation.

### OC3 — Axis 3: Singleton plugin slot is correct but understates depth

**Source claim:** "Singleton correctly describes the constraint but
understates the depth. openclaw's memory system has: file-level persistence
(MEMORY.md + daily notes + DREAMS.md), SQLite index, embedding-based hybrid
search, active-memory sub-agent, dreaming background consolidation. This is
not a single primitive — it's a layered system with a single swappable
storage backend. The 'singleton plugin slot' is the storage/retrieval
layer; the rest of the memory architecture (file conventions, dreaming,
active-memory) is built on top."

**Primary-source evidence:** `docs/concepts/memory.md`,
`docs/concepts/active-memory.md`, `docs/concepts/memory-builtin.md`,
`docs/tools/plugin.md`.

**Verdict: ACCEPT WITH QUALIFICATION.** Framework Axis 3 row stays correct;
add a clarifying note that openclaw's "Singleton plugin slot" describes the
storage/retrieval layer, not the entire memory architecture.

**Integration:**
- Framework Axis 3 row text refinement: openclaw's note should explicitly
  mark the singleton-slot scope (storage/retrieval layer) so v2 candidates
  understand the singleton constraint doesn't preclude layered architecture
  on top.
- openclaw per-system file should document the full layered memory
  architecture.

### OC4 — Axis 6: Plugin SDK is concrete, two-tier, TypeBox-typed

**Source claim:** "The plugin SDK is concrete (200+ subpaths, TypeBox-typed,
explicit registration). The two-tier split (code plugins vs.
bundle-style) is implemented and documented with rationale."

**Primary-source evidence:** `docs/plugins/sdk-overview.md`,
`docs/plugins/building-plugins.md`, `docs/plugins/sdk-channel-plugins.md`,
`scripts/lib/plugin-sdk-entrypoints.json`.

**Verdict: ACCEPT.** Implementation-backed strengthening of cycle-14
observation. No framework correction needed; the existing Axis 6 row for
openclaw stays accurate but is now better-grounded.

**Integration:**
- Framework Axis 6 row: no text change required.
- openclaw per-system file: document the SDK structure (200+ subpaths,
  TypeBox typing, the two-tier code/bundle split).

### OC5 — Axis 7: openclaw is flat isolated-persona routing, not topology

**Source claim:** "Multi-agent routing is flat isolated-persona routing
[...] no coordinator-worker hierarchy. Agent-to-agent messaging is disabled
by default. The 'manager-of-managers' rejection in VISION.md is a versioned
roadmap guardrail, not a categorical prohibition. [...] This does not map
cleanly to any Axis 7 row as defined."

**Primary-source evidence:** `docs/concepts/multi-agent.md`, VISION.md
("What We Will Not Merge" — agent-hierarchy frameworks rejected as default
architecture).

**Verdict: ACCEPT.** Important non-claim. openclaw is NOT in any current
Axis 7 row, which is correct — but the deeper read clarifies why. It's not
that openclaw was missed; it's that openclaw's multi-agent system is
**flat routing to isolated personas**, not orchestration topology in the
Axis 7 sense.

**Integration:**
- Framework Axis 7: no row change. Could add a "Considered-and-folded"
  note explaining that flat isolated-persona routing (openclaw's pattern)
  is not orchestration topology — it's routing-to-isolated-workers, which
  is a different design choice that doesn't compose with topology
  positions.
- openclaw per-system file: explicitly document the flat-routing
  architecture and why it's not a topology.

### OC6 — Constraint 3 strengthened: three default-deny patterns

**Source claim:** "Three specific patterns are implementation-backed: (1) DM
pairing required by default (operator explicitly approves each sender), (2)
Workspace-origin plugins disabled by default, (3) Agent-to-agent messaging
disabled by default. All three are 'default-deny, explicit-allow' applied
to different capability surfaces."

**Primary-source evidence:** README, `docs/concepts/multi-agent.md`,
`docs/tools/plugin.md`.

**Verdict: ACCEPT.** Strengthens Convergent Constraint 3 (Strong-defaults
security) with three concrete default-deny implementations.

**Integration:**
- Framework Constraint 3: no text change needed (the constraint is already
  stated). openclaw is now the clearest implementation across surveyed
  systems.
- openclaw per-system file: document the three default-deny patterns.

### OC7 — Constraint 5 confirmed + nuance: "guardrail, not law of physics"

**Source claim:** "VISION.md's 'What We Will Not Merge' is verbatim
accessible and the caveat is notable: 'This list is a roadmap guardrail,
not a law of physics.' This is a more honest framing than an absolute
prohibition — the list is versioned and revisable, not inscribed."

**Primary-source evidence:** VISION.md verbatim.

**Verdict: ACCEPT.** Important nuance for the redesign's anti-pattern
discipline. The Cognition June-2025-vs-April-2026 walkback is the analogue
case where an absolute-sounding rejection ("Don't Build Multi-Agents") was
later walked back. openclaw's framing avoids this by being explicit about
versioned-and-revisable from the start.

**Integration:**
- Framework Constraint 5: could add a note about the framing ("anti-pattern
  list is versioned and revisable, not absolute") — useful for v2
  candidates.
- v2 prompt design: anti-pattern list should be versioned, with explicit
  caveats about revision conditions.

### OC8 — Axis 2: openclaw is file-per-component (per-agent state isolation)

**Source claim:** "Per-agent state: `~/.openclaw/agents/<agentId>/agent/`
(auth profiles, model registry, per-agent config). Global config:
`~/.openclaw/openclaw.json`. Session transcripts: per-session JSONL.
Memory index: per-agent SQLite. There is no global state file analogous to
v1's `state.json`."

**Primary-source evidence:** `docs/concepts/session.md`,
`docs/concepts/architecture.md`, `docs/concepts/memory.md`.

**Verdict: ACCEPT.** openclaw should be added to Axis 2's "File-per-component"
row. Currently the row lists AutoGen + Voyager + oh-my-codex (3-system).
Adding openclaw makes it 4-system support, strengthening the convergence.

**Integration:**
- Framework Axis 2 row: add openclaw to "File-per-component" position with
  per-agent state isolation cite.
- openclaw per-system file: document the per-agent file layout.

### OC9 — Session as routing key + lifecycle (relevant for state/memory)

**Source claim:** "Sessions are the conversation thread; each message is
routed to a session based on its source. [...] Session lifecycle: daily
reset at 4:00 AM local time by default; idle reset configurable; manual
`/new` or `/reset`."

**Primary-source evidence:** `docs/concepts/session.md`.

**Verdict: ACCEPT WITH QUALIFICATION.** The "session" pattern is interesting
for v2 design but doesn't transfer cleanly. openclaw sessions are
human-conversation threads; the redesign orchestrator operates on a cron
schedule with no per-session human dialog. The lifecycle primitives (daily
reset, idle reset, fresh-per-cron) ARE potentially transferable as state
freshness mechanisms — for example, "reset state freshness daily" could be
a useful primitive for v2 candidates that maintain state across cycles.

**Integration:**
- openclaw per-system file: document session architecture and lifecycle.
- v2 candidate template: consider state-freshness lifecycle as a sub-axis
  of Axis 2 or Axis 4 (no framework change yet; flag for Phase 2 candidate
  generation).

### OC10 — Plugin lifecycle: fail-closed loading

**Source claim:** "Invalid plugin config is isolated to that plugin; other
plugins and channels stay online. This fail-closed, recover-with-doctor
pattern is directly transferable to tool loading in the redesign.
'openclaw doctor --fix' quarantines bad plugin config."

**Primary-source evidence:** `docs/tools/plugin.md`.

**Verdict: ACCEPT.** Useful pattern for v2 candidate's tool-loading
discipline.

**Integration:**
- openclaw per-system file: document fail-closed plugin loading.
- v2 candidate template: tool-loading strategy is a relevant detail under
  Axis 6 (extension shape).

### OC11 — Axis 9: openclaw has runtime ceiling (48h timeout)

**Source claim:** "`agents.defaults.timeoutSeconds` default of 172800s (48
hours)" — runtime ceiling per agent run. "stuck-session watchdog
(`diagnostics.stuckSessionWarnMs`) detects and can release stale lanes."

**Primary-source evidence:** `docs/concepts/agent-loop.md`,
`docs/concepts/queue.md`.

**Verdict: ACCEPT WITH QUALIFICATION.** openclaw has runtime ceiling but
the 48h default is essentially unbounded for typical use (compare to
oh-my-codex's `max_iterations=10` or Voyager's `max_retries=4`). The
stuck-session watchdog is a more interesting primitive — it's "detect
stale runs and release the lane." The redesign's cron-cycle limit is the
analogous coarse ceiling.

**Integration:**
- Framework Axis 9 row "Runtime ceiling": add openclaw with the qualifier
  about 48h being effectively-unbounded for typical use.
- openclaw per-system file: document the timeout + watchdog pattern.

### OC12 — Axis 13: Gateway daemon as partial fat-harness analogue

**Source claim:** Gateway daemon is always-on; agents are per-event
invocations. The Gateway handles (in deterministic code): channel
connections, queue management, plugin lifecycle, session routing, sandbox
enforcement, tool policy. Agents handle (in LLM session): the per-turn
conversational reasoning.

**Primary-source evidence:** `docs/concepts/architecture.md`,
`docs/concepts/agent-loop.md`, `docs/concepts/queue.md`.

**Verdict: ACCEPT.** Interesting partial fat-harness evidence for Axis 13.
The Gateway/agent split is structurally analogous to the redesign's
cycle-runner/orchestrator-session split, but with much more in the Gateway
than v1's cycle-runner has.

**Integration:**
- openclaw per-system file: document the Gateway/agent split as an
  Axis 13 partial-fat-harness instance.
- Framework Axis 13: no row change; existing 3 positions (thin/medium/fat
  harness) cover this. Could add openclaw as a surveyed example of
  partial-fat-harness if desired.

### OC13 — Capability escalation tiers (delegate architecture)

**Source claim:** "Tier-1/tier-2/tier-3 capability escalation (read-only →
send-on-behalf → autonomous with standing orders) maps to tool permission
escalation in the redesign."

**Primary-source evidence:** `docs/concepts/delegate-architecture.md`.

**Verdict: ACCEPT.** Useful pattern for v2 candidate's permission discipline.

**Integration:**
- openclaw per-system file: document the tier-1/2/3 capability escalation.
- v2 candidate template: consider permission-tier discipline as a v2
  candidate detail (no framework change yet; Phase 2 candidate generation
  may surface as a sub-axis or fold into Constraint 3).

### OC14 — Anti-pattern caveat framing (versioned, revisable)

**Source claim:** "openclaw's rejection is explicitly framed as a priority
decision, not a categorical prohibition. [...] 'This list is a roadmap
guardrail, not a law of physics. Strong user demand and strong technical
rationale can change it.'"

**Primary-source evidence:** VISION.md.

**Verdict: ACCEPT.** Aligns with the Cognition walkback lesson (cycle-41
finding C1) — anti-pattern claims that read as absolute can be invalidated
by later evidence. Versioned-and-revisable framing is the honest pattern.

**Integration:**
- Already covered by OC7. No additional integration needed.
- Cross-reference cycle-41 finding C1 (Cognition walkback) — same
  meta-pattern.

### OC15 — Star count anomaly + repo age (calibration on "battle-tested")

**Source claim:** "365k★ / 75k forks for a 5-month-old repo (created
2025-11-24) is statistically anomalous. The docs are detailed and the
source is real TypeScript with real tests, so the architectural claims are
grounded — but external validation of design choice consequences is weaker
than a mature project would provide. [...] Treat the architecture as real
and the consequence-verification as limited."

**Primary-source evidence:** GitHub repo metadata (stars, forks, creation
date) + presence of real source + real tests.

**Verdict: ACCEPT.** Honest calibration. The architecture claims are
implementation-backed and citable; consequence-claims (e.g., "this works
well at scale") need a source-of-truth beyond openclaw's repo.

**Integration:**
- openclaw per-system file: include the calibration caveat (architecture
  real, consequence-validation limited).
- Phase 2 candidate generation: when openclaw patterns are cited, the
  consequence-validation caveat applies.

### OC16 — Capability registration API breadth (16+ named methods)

**Source claim:** Plugin SDK has 16+ named registration methods on
`OpenClawPluginApi` covering capability registration, tools/commands,
infrastructure, memory, workflow/lifecycle.

**Primary-source evidence:** `docs/plugins/sdk-overview.md`.

**Verdict: ACCEPT.** Concrete depth of plugin SDK. Useful for v2 candidate's
tool-registration design discussion.

**Integration:**
- openclaw per-system file: document the SDK breadth.
- No framework change.

### OC17 — Memory promotion mechanism (dreaming with thresholds)

**Source claim:** "Promotions from short-term `memory/.dreams/` to
`MEMORY.md` are gated on score, recall frequency, and query diversity
thresholds. DREAMS.md is the human-review surface."

**Primary-source evidence:** `docs/concepts/active-memory.md`,
`docs/concepts/memory.md`.

**Verdict: ACCEPT.** Specific implementation of memory consolidation —
relevant pattern for v2 candidates considering structured memory
architecture.

**Integration:**
- openclaw per-system file: document the dreaming mechanism with promotion
  thresholds.
- No framework change.

### OC18 — Reserved core admin namespaces (operator.admin)

**Source claim:** "Reserved core admin namespaces (`config.*`,
`exec.approvals.*`, `wizard.*`, `update.*`) — always stay `operator.admin`,
even if a plugin tries to override."

**Primary-source evidence:** `docs/concepts/agent-loop.md`.

**Verdict: ACCEPT.** Useful pattern for v2 tool/permission boundaries.

**Integration:**
- openclaw per-system file: document reserved namespaces.
- v2 candidate template: consider plugin-vs-core authority boundaries as a
  detail under Axis 6 or Axis 8 (no framework change yet).

### OC19 — `global-state.ts` exists (implementation-backed observation)

**Source claim:** "`src/global-state.ts` and `src/globals.ts` at the root
of `src/` suggests some process-level global state does exist
(implementation-backed observation — the file exists; its contents not
read)."

**Primary-source evidence:** `src/` directory tree at repo root.

**Verdict: ACCEPT WITH QUALIFICATION.** Honest implementation-backed
observation that openclaw is NOT pure file-per-component on Axis 2 — there
IS some process-level global state. The file exists; its contents not read.
Qualifier: this doesn't change Axis 2 placement (the per-agent state files
are still file-per-component); it adds nuance that the framework should
not present openclaw as a pure example.

**Integration:**
- openclaw per-system file: include this caveat in the Axis 2 anchoring
  section (per-agent state is file-per-component; Gateway-level globals
  exist as per `global-state.ts` — contents not verified).
- Framework Axis 2 row: add caveat to openclaw entry — "(per-agent state;
  Gateway-level globals exist per `global-state.ts`, contents
  not-yet-verified)."

### OC20 — Cron is denied in default sandbox (priority signal)

**Source claim:** "The default sandbox (for non-main sessions including
most cron and agent-spawned sessions) **denies `cron`**. [...] Cron is
available in the main session (unsandboxed) and where explicitly enabled.
[...] **Cron is a power-user feature in openclaw; it's the primary trigger
in the redesign.**"

**Primary-source evidence:** README, `docs/concepts/queue.md`.

**Verdict: ACCEPT.** Important cross-system difference for transfer
analysis. openclaw's design assumes interactive use; redesign's design
assumes cron-driven autonomous loops.

**Integration:**
- openclaw per-system file: document the cron-as-sandboxed-by-default
  signal in transfer caveats.
- v2 candidate generation: when borrowing openclaw patterns, the
  cron-priority inversion needs to be considered.

### OC21 — Comparison with cycle-14 stub (deliverable-quality verification)

**Source claim:** "All cycle-14 pattern observations confirmed. Major
additions: queue architecture, per-request agent invocation model, plugin
SDK surface (200+ TypeBox-typed subpaths), session lifecycle details,
delegate architecture pattern, and the Axis 12 correction. [...] The
cycle-14 stub missed or understated [12 specific items]."

**Verdict: ACCEPT.** Establishes the deeper-read deliverable's quality and
extension over the prior stub. The 12 missed-or-understated items are the
substantive value-add of the deeper read.

**Integration:**
- openclaw per-system file rewrite (cycle-14 stub → deeper-read status)
  uses these 12 items as the "deeper read additions" section.

## Framework correction summary (for v1.7 application)

Based on the per-finding evaluation, the v1.7 framework changes are:

1. **Axis 2 row** — add openclaw to "File-per-component" with `global-state.ts`
   caveat (4-system support; OC8 + OC19).
2. **Axis 3 row** — refine openclaw note to clarify singleton-slot scope is
   storage/retrieval layer, not full memory architecture (OC3).
3. **Axis 9 row** — add openclaw to "Runtime ceiling" with 48h-effectively-
   unbounded qualifier + stuck-session watchdog as more interesting primitive
   (OC11).
4. **Iteration history table** — v1.7 row added.
5. **Status header** — v1.6 → v1.7.
6. **Cycle-40 v2-design observation refinement** — journal entry / per-finding
   eval file; openclaw's pattern is implementation-detail within Axis 12
   "Event-driven" position, not a third axis position. The cycle-40 "three
   patterns visible" claim refines to TWO axis-distinct patterns (sync HITL vs
   async) plus implementation-nuance within async (cron+catchup vs
   event-driven with persistent connections vs webhook-on-event). No new Axis
   12 row.

## Per-system file promotion (openclaw.md cycle-14 stub → deeper-read status)

Following the cycle-41 cognition-devin / openai-harness promotion pattern:

- Status header: "stub" → "deeper read landed (cycle 43)"
- Replace cycle-14 dispatch references with PR #2809 deliverable references
- Expand pattern observations to integrate substantive findings (the 12
  missed/understated items from OC21)
- Add Phase 2 framework anchoring summaries (per-axis evidence: Axis 2, 3,
  6, 12, 13; Constraints 3, 5)
- Refine anchoring caveats: single-user vs autonomous-cron, TypeScript-vs-
  Rust, local-machine vs ephemeral-runner, star-count anomaly + repo age
  (OC15)

## Cross-cycle observations

- **Cycle-40's "three patterns" v2-design observation revised, not retired.**
  The cross-system observation that surveyed systems split between sync HITL
  and async patterns survives. The cycle-40 specific claim that openclaw is
  "async-with-server (continuous-runtime)" was over-extended; the deeper
  read shows openclaw is "event-driven with persistent connections" within
  the existing Axis 12 Event-driven position. Implementation-nuance within
  an existing position, not a new axis position.

- **The "cross-axis update propagation" failure mode (cycle 42) recurs in
  cycle-43 v1.7 application.** When updating Axis 2 to add openclaw, the
  framework should also re-check whether Axis 3, Axis 4, Axis 12 references
  to openclaw need any propagation updates. Cycle-43 cold-reader Q(a) on
  v1.7 is the cycle-44 verification.

- **Star-count anomaly + repo age signal calibration discipline.** Future
  Phase 1 dispatches to high-anomaly repos should default to
  consequence-validation caveats. The OC15 finding is a generalizable
  discipline.

## What this evaluation surprised me with

- The Axis 12 cycle-40 observation was OVER-EXTENDED. I had expected the
  deeper read to either confirm "async-with-server" or refute it cleanly;
  instead it found a refinement: openclaw's pattern fits within the
  existing Axis 12 Event-driven position with implementation-specific
  details. This is the kind of "third option" that doesn't show up in
  pre-commit-checklist Q(c) framing because it requires the deeper read
  evidence to articulate.

- The `global-state.ts` finding (OC19) is an honest implementation-backed
  caveat that the cycle-14 stub couldn't have surfaced (it only read
  README + VISION). The deeper read's `src/` directory listing surfaced
  the file's existence; reading the actual content is a future finer
  read if Phase 2 candidates need it.

- The 21 findings, 21 accepted total — equal to cycle-41's per-finding
  evaluation count. Different deliverable (openclaw vs Cognition+OpenAI)
  but similar substantive density. Suggests the per-finding evaluation
  pattern is well-tuned to single-deeper-read deliverables of this size
  (~800-900 lines).
