# Cycle 17: cycle-16 cold-readers + Voyager orchestrator-direct read

Cycle 16 (commit `67737a11`) ran three cold-readers on cycle-15 work,
the AutoGen dispatch evaluation + Tier-1 integration into
`1-research.md`, and deferred Voyager + Tier-2 group 3 to cycle 17+.
Cycle 16 left five pre-commits for cycle 17+: two cold-readers on the
AutoGen integration (navigation summary + 16-bullet vs 38-bullet
selection), Voyager paper read (deferred from cycles 15 and 16,
making this the third cycle deferring), Tier-2 group 3 if capacity
permits, and decision on next external-system dispatch.

Cycle 17 ran the two cold-readers and the Voyager orchestrator-direct
read. Cycle-18+ pre-commits at the bottom; Tier-2 group 3 deferred to
cycle 18+ as cycle-16 anticipated.

## Cold-reader 1: AutoGen navigation summary in `1-research.md`

The question (from cycle-16 pre-commit 1): does the ~140-line
navigation summary in `1-research.md` undersell or misrepresent the
697-line deep-dive at PR [#2763](https://github.com/EvaLok/schema-org-json-ld/pull/2763)?
Compression ratio ~5×.

**Verdict: PASS with one optional cycle-18+ flag.**

Read the navigation summary (lines 217-368 of `1-research.md`)
side-by-side with the deep-dive's seven-section structure (deep-dive
sections 1-7 + Patterns observed). Section-by-section:

- Deep-dive §1 "Overall architecture and named primitives" → covered
  by navigation summary's "Layered architecture" paragraph. Five layers
  named consistently. Concrete primitives (AssistantAgent,
  RoundRobinGroupChat) appear; migration history (v0.2 → v0.4) appears
  via the "Anti-patterns explicit in v0.4 migration guide" section.
  No load-bearing loss.
- Deep-dive §2 "Multi-turn conversation and state representation" →
  covered by "State as serialized component dictionaries." TeamState
  example dictionary structure preserved; reset/resume distinction
  preserved; ChatCompletionContext (Unbounded/Buffered/TokenLimited)
  preserved via "Model-context abstraction." Acceptable compression.
- Deep-dive §3 "Orchestration / planning patterns" → covered by
  "Multiple orchestration patterns as first-class." Nine patterns named
  (claim of "9 documented" verified against deep-dive lines 290-298,
  which lists exactly 9). Magentic-One Task Ledger / Progress Ledger
  vocabulary preserved. The "patterns emerge from message protocols"
  framing from deep-dive line 288-301 is partially compressed — see
  cold-reader 2 for the relevant Patterns observed bullet.
- Deep-dive §4 "Failure handling and recovery" → covered by "What
  AutoGen does not centrally guarantee" + "Termination as first-class
  composable callables." The five-item not-guaranteed list is
  preserved verbatim.
- Deep-dive §5 "Tool / skill integration model" → **partially covered**
  in the navigation summary. The trust aspects (Docker/local executor,
  MCP warnings, Magentic-One web warnings) appear as a dedicated "Trust
  boundaries explicitly named" paragraph. But schema-driven tool
  calling, agent-as-tool composition, the v0.4 migration design
  decision on tool routing, the parallel-tool-call constraint for
  stateful agents, and `is_error` tool error reporting appear ONLY in
  the Patterns observed bullets — not as a dedicated tools-as-
  architectural-concern paragraph. The deep-dive treats this as 1 of 7
  main sections (~84 lines); the navigation summary folds it into Trust
  boundaries + scattered bullets.
- Deep-dive §6 "Anti-patterns and explicit non-goals" → covered by
  "Anti-patterns explicit in v0.4 migration guide" + the kitchen-sink
  bullet in Patterns observed. Ten explicit non-goals from the deep-dive
  aren't enumerated; the most distinctive ones (register_reply
  callback, sequential chat removal, kitchen-sink AssistantAgent) are
  preserved. Acceptable compression.
- Deep-dive §7 "Anchoring caveats" → covered by "Anchoring caveats on
  AutoGen" with seven explicit items. The deep-dive has twelve; five
  are folded into the seven via merging or omitted. The four anchoring
  differences cycle-15 explicitly named in the dispatch body are all
  preserved.
- Deep-dive Patterns observed (43 bullets) → 15-bullet selection in
  navigation summary. Cold-reader 2 evaluates this separately.

**Optional cycle-18+ flag**: add a "Tool integration model" paragraph
to the navigation summary, between "Trust boundaries explicitly named"
and "Anti-patterns explicit in v0.4 migration guide." Content: schema-
driven tool calling (model emits structured call, host executes
registered code), agent-as-tool composition with parallel-execution
forbidden for stateful tools, the v0.4 routing simplification (direct
execution in AssistantAgent replacing user-proxy routing), `is_error`
result-shape for tool errors. Estimated ~10 lines. Not load-bearing
(the content IS present scattered across other sections + Patterns
observed); raises the structural status of tools to match the
deep-dive's section-level treatment. Defer with this flag rather than
applying this cycle, since the issue is structural rather than missing
content and Voyager + cold-reader-2 + persistence overhead is the
higher-priority work this cycle.

## Cold-reader 2: AutoGen 16-bullet vs 38-bullet selection (selection-bias check)

The question (from cycle-16 pre-commit 2): which of the deep-dive's
38 Patterns observed bullets were dropped in the 16-bullet navigation
summary? Was selection driven by v2-relevance anchoring (a smuggling
failure) or by genuine architectural importance?

**Count correction.** Cycle-16 said "16-bullet" and "38-bullet" but
actual counts are **15 bullets in the navigation summary** and **43
bullets in the deep-dive**. This is a minor cycle-16 self-reporting
discrepancy. The cycle-16 evaluation conclusion (no v2-relevance
smuggling) is unaffected by the count error; the cold-reader question
itself reframes to "which 28-30 bullets were dropped?"

**Mapping (deep-dive bullets → navigation summary bullets).**
- Layered architecture (deep-dive 1) → nav 1
- Maintenance signaling (2) → nav 2
- Actor-model framing (3) + Runtime-mediated identity (4) → MERGED
  into nav 3
- Multiple orchestration patterns: round-robin (10), selector (11),
  swarm (12), graph (implicit), lead-orchestrator (14) → MERGED
  into nav 4
- Magentic-One Ledger vocabulary (15) → nav 5
- Statefulness (16) + Reset/resume (18) → MERGED into nav 6
- State save/load (19) + Dictionary serialization (20) → MERGED into
  nav 7
- Model-context abstraction (21) → nav 8
- Termination (23) + AND/OR (24) → MERGED into nav 9
- Graceful stop vs cancellation (25) → nav 10
- Schema-driven tool calling (29) + Host-side execution (30) → MERGED
  into nav 11
- Agent-as-tool (13) + Parallel forbidden (33) + Agents-as-tools (34)
  → MERGED into nav 12
- Reflection pattern (28) → nav 13
- Aspirational-vs-implemented → nav 14 (NEW: synthesized from
  deep-dive §1 lines 89-94 and §6 line 512, NOT from Patterns observed
  list)
- Kitchen-sink + sequential-chat-removed → nav 15 (NEW: synthesized
  from deep-dive §6 lines 480-482 and Patterns observed bullet 42)

So 13 of 15 navigation-summary bullets are direct selections from the
43-bullet deep-dive Patterns observed list (often merging multiple
deep-dive bullets into one nav-summary bullet). 2 of 15 are syntheses
of content from elsewhere in the deep-dive elevated to a Patterns
observed bullet.

**Bullets dropped from navigation summary (selection):** 5 (data-
dependent agent instances), 6 (direct/broadcast messaging both exist),
7 (pub/sub topics/subscriptions), 8 (behavior contracts as message
protocols), 9 (high-level teams as presets), 17 (caller sends new
messages), 22 (built-in context-window controls — folded into nav
8 as "Model-context abstraction"), 26 (cancellation tokens), 27
(streaming as observability), 31 (`is_error` tool errors), 32 (direct
tool execution in AssistantAgent — partially folded into nav 15), 35-40
(tool/security details: MCP, Docker, local executor, Docker-out-of-
Docker, human approval, web/prompt-injection warnings — all folded
into the "Trust boundaries" prose paragraph), 41 (custom agents replace
register_reply — partially folded into nav 15), 43 (Studio not
production).

**Selection-bias evaluation:**
- **No v2-relevance smuggling detected.** The dropped bullets are
  predominantly: (a) lower-level architectural details (5-9, 17),
  (b) tool/security details folded into the prose Trust boundaries
  section (35-40), (c) minor technical specifics (22, 26, 27, 31, 43).
  None of the dropped bullets show a pattern of "would-not-align-with-v2-
  hopes" — the selection criterion is consistent with "carry the most
  architectural information at the same abstraction level."
- **Architectural-hierarchy bias detected on one bullet.** Deep-dive
  bullet 8 ("Behavior contracts as message protocols; patterns emerge
  from protocol implementation rather than from a universal orchestrator
  object") is **the core Core-API framing** — AutoGen's stance that
  it does not assert one canonical orchestrator role. This is partially
  expressed in nav 4 ("Multiple orchestration patterns coexist as
  first-class") but the WHY (patterns emerge from message protocols,
  not from a universal orchestrator object) is dropped. This is
  arguably an architectural-importance miss rather than a v2-relevance-
  bias miss.

**Verdict: PASS with one optional cycle-18+ flag.**

Optional cycle-18+ flag: enrich navigation summary bullet 4 to
include the WHY framing — e.g., change "Multiple orchestration patterns
coexist as first-class (round-robin, selector, swarm, graph,
lead-orchestrator)" to "Multiple orchestration patterns coexist as
first-class because behavior contracts are expressed as message
protocols rather than as a universal orchestrator object (round-robin,
selector, swarm, graph, lead-orchestrator)." Estimated ~1 sentence
addition. Same defer-not-apply reasoning as cold-reader 1's flag —
it's a refinement, not a load-bearing miss; cycle 17's capacity was
prioritized to Voyager (third defer threshold) over optional refinements.

**Cycle-16 self-reporting discrepancy (count claim).** Cycle-16 said
"16-bullet" and "38-bullet"; actual is "15-bullet" and "43-bullet."
The discrepancy doesn't change the cycle-16 evaluation's conclusions
but should be acknowledged. Defer the cycle-16 notes-file correction
itself — process-commentary churn — but flag this paragraph as the
record-of-correction for any future cycle reading the cycle-16 notes
file.

## Voyager orchestrator-direct read (the third-defer commitment, honored)

Voyager was deferred from cycles 14, 15, and 16 — three cycles. Cycle
17 honored the commitment to read it this cycle.

**Sources read:**
- README.md on master (full)
- Abstract from arxiv 2305.16291 (the README quotes the abstract
  prominently)
- `voyager/voyager.py` (top ~180 lines; agent initialization and main
  class structure)
- `voyager/agents/skill.py` (full, ~140 lines; SkillManager class)
- `voyager/agents/critic.py` (top ~120 lines; CriticAgent class
  including auto and manual modes)
- `voyager/agents/curriculum.py` (top ~150 lines; CurriculumAgent
  init, default_warmup, curriculum_observations, render_observation
  start)
- `voyager/agents/` directory listing (4 agents + __init__)
- `voyager/prompts/` directory listing (8 prompt files)
- Recent commits (last commit 2023-07-27, repo stable)

The full paper PDF is on `voyager.minedojo.org/assets/documents/voyager.pdf` — not
fetched this cycle (WebFetch and curl gated by permissions). The
abstract + code + agent prompts in repo provided enough architectural
detail for the navigation summary in `1-research.md`.

**Findings integrated into `1-research.md` Voyager section.** ~190 line
section paralleling AutoGen's structure: status, four-agent
architecture, cost tiering, component-local persistence, sync invariants,
skill versioning, skill retrieval, iteration mechanism, mode
toggleability, curriculum warm-up, no-fine-tuning commitment, two-layer
capability composition, prompts as external files, anchoring caveats
(8 specific), Patterns observed (16 bullets, pure observation).

**Anti-smuggling discipline applied.** The Patterns observed list is
structured as pure observation — no v2-relevance framings. Sample:
"Cost-tiering across agents: cheap model for cached/derivative work,
expensive model for novel reasoning" describes Voyager's design choice
without "as a v2-redesign-relevant pattern" smuggling. "Sync invariants
asserted at init for dual-storage components" describes Voyager's
discipline without "v2 should adopt this" framing. Anchoring caveats
section explicitly enumerates eight ways Voyager doesn't transfer
(continuous-runtime vs cold-cycle, embodied vs sparse-state, etc.).

**Notable observations.**
1. **Four-agent architecture with named roles** is conceptually
   distinct from AutoGen's "team" abstractions — Voyager's roles
   (action / curriculum / critic / skill-library) are concrete classes
   not configurable presets. AutoGen offers nine orchestration shapes;
   Voyager commits to one. This is a commitment-vs-flexibility tradeoff
   visible in the architecture itself.
2. **Cost tiering across agents in default config** — gpt-4 for novel
   reasoning, gpt-3.5-turbo for cached / derivative work. Mirrors a
   pattern this redesign already uses (Anthropic for orchestration,
   Copilot/OpenAI for dispatches) but at a finer-grained level (per
   sub-agent within a single system).
3. **Sync invariants on dual-storage** (vectordb count == JSON manifest
   count, fail-fast at init) is a discipline pattern. Maps directly
   to F2/F4 reconciliation-asymmetry concerns from the Phase 0
   retrospective — Voyager's mechanism for preventing the same class of
   failure is "assert at boot, fail fast." Pure-observation framing in
   the Voyager section; cross-system synthesis (whether this maps to
   v2's reconciliation problem) is deferred.
4. **Skill versioning is append-on-disk + replace-in-vectordb** — the
   active retrieval surface is single-version, but the disk history is
   monotonic-append. Same write-mostly state pattern noted in v1 (F5)
   in a different domain. The deep-dive observation: Voyager's design
   chose explicit append-on-history-with-single-version-active rather
   than allowing the disk to fragment, suggesting that "monotonic-append
   with semantic-replacement-pointer" might be a generalizable shape
   for write-mostly state. Cross-system synthesis deferred.
5. **Mode toggleability** for human-in-the-loop (auto/manual per agent)
   parallels v2's EVA-DEFAULT-AUTONOMY framing — autonomous-default
   with explicit human-input mode as a configurable. Voyager has the
   pattern at agent-level granularity; v2 has it at orchestrator-level.

**What was harder than expected.** The repo file structure was
straightforward; the architecture is documented well in code comments.
The harder part was deciding the right depth: a Copilot deep-dive on
Voyager could plausibly produce 600-700 lines (parallel to AutoGen),
but Voyager is structurally simpler (4 agents in 4 small files) and the
paper is short. Orchestrator-direct read at ~150 lines summary feels
like the right depth — comparable to AutoGen's navigation summary
without dispatching for a deeper deep-dive.

**Aspect of Voyager NOT covered this cycle.** The actual prompt
contents in `voyager/prompts/*.txt` weren't read line-by-line. The
prompt-text-as-architecture is a separate evidence base (analogous to
how the redesign work is now reading PRODUCTION orchestrator prompts).
A cycle-18+ deeper read could be queued if Voyager's prompt structure
becomes a focal point in cross-system synthesis. Not pre-committing
this cycle.

## Decision on next external-system dispatch

**Recommendation: LangGraph Copilot research-only dispatch in cycle 18.**

Rationale:
- AutoGen's state-management treatment in the deep-dive was light
  (component-local dictionaries, no centralized graph). LangGraph
  centers state-management as its primary value proposition.
  Reading LangGraph completes the state-management coverage gap.
- LangGraph is widely-used in production (LangChain ecosystem); its
  treatment of state-as-graph and the supervisor patterns it ships
  with would extend the orchestration-pattern observations from
  AutoGen.
- Copilot research-only dispatch is the right mechanism: cycle-15's
  procedure works (anti-smuggling discipline pre-loading honored in
  cycle 16's evaluation); ~3-day average between dispatch and PR landing
  doesn't block cycle 18 from doing other work in parallel.

Alternative considered: **Cognition Devin writeups orchestrator-direct.**
Devin is closer to v2's "AI does software-engineering work autonomously"
target, and the writeups would extend the failure-pattern catalog with
production observations. Strong candidate. Defer this to cycle 19+ —
LangGraph's structural state-management focus is the more critical gap
right now (3 systems read so far, none center state-management as
primary).

**Pre-commit for cycle 18:** if cycle-17 work is approved, dispatch
LangGraph research-only to Copilot via the canonical cycle-15 procedure
(`gh issue create` with `agent-task` + `research-only` labels, Copilot
assigned, gpt-5.5 in `agent_assignment.model` payload, seven-lens
framing pre-loading anti-smuggling discipline).

## Tier-2 group 3 (freeze-vs-refresh framing alternative) — deferred to cycle 18+

Per cycle-16's pre-commit 4 ("if cycle 17 capacity permits beyond
Voyager"), Tier-2 group 3 was contingent on cycle-17 capacity. Capacity
this cycle was consumed by Voyager + cold-readers + persistence
overhead. Tier-2 group 3 is structurally larger (a deeper rewrite of
the freeze-vs-refresh framing in Phase 0 §-preamble) and warrants
focused attention; bundling with cycle 17 risks incomplete execution.

Defer to cycle 18+. Carries forward from cycles 13, 14, 15, 16, 17 (5
cycles deferred). Continued deferral is a known pattern; group 3 is
the deepest structural Phase 0 finding still open and should not be
indefinitely shelved.

**Cycle 18+ priority among Tier-2 groups:** group 3 remains the
strongest candidate (deepest finding, longest-deferred). Group 5
(iteration plan move to README) was executed in cycle 15. Other Tier-2
groups (2, 4, 6, 7, 8, 9 — six remaining) are all smaller-scope and
less load-bearing.

## Long-deferred items roll-call (carried forward, no action this cycle)

1. Journal-entry self-congratulation sweep (now 11 cycles deferred from
   cycle 7+)
2. F6/F8/F9 measurements (cycle 7+)
3. Refactor-for-length F-section sweep (cycle 8+; Tier-2 group 8)
4. Persistence-mechanism deferred-list consolidation (cycle 13 flag)
5. Tier-2 group 2 (review/disposition substrate, partial integration in
   cycle 13)
6. Tier-2 group 3 (freeze-vs-refresh framing alternative — deepest
   cycle-11 finding still open) — moved up to cycle 18+ pre-commit
7. Tier-2 group 4 (nine measures rework)
8. Tier-2 group 6 (preserved-through-cutover disposition)
9. Tier-2 group 7 (resolved open questions collapse)
10. Tier-2 group 9 (F8 singleton-family acknowledgment — small edit,
    cycle 13 noted promotion candidate)

Cycle 17 added 0 items to long-deferred list. Two cycle-17+ flags
generated (cold-reader 1: "Tool integration model" paragraph; cold-
reader 2: enrich nav-bullet-4 framing) but they're minor refinements
not meriting promotion to long-deferred status — recorded in the
cycle-17+ pre-commits below.

## Cycle 18+ pre-commits

1. **LangGraph Copilot research-only dispatch.** Per cycle-17's
   next-dispatch decision. Use the canonical cycle-15 procedure with
   anti-smuggling discipline pre-loaded in the dispatch body. Seven
   lenses calibrated against the LangGraph subject (state-management
   focus is the dominant lens; other lenses TBD per cycle-18 framing).
2. **Cold-reader on Voyager Patterns observed list (16 bullets).**
   Parallel to cold-reader 2 this cycle: which deep-dive observations
   were elevated to Patterns observed and which were dropped, and was
   selection v2-relevance-biased? The Voyager section was written
   orchestrator-direct (no separate deep-dive to compare against), so
   this cold-reader is internal — does the Patterns observed list
   accurately reflect what's in the prose paragraphs above it?
3. **Cold-reader on Voyager anchoring caveats.** Eight caveats are
   enumerated; are they the right eight? Are there transferability
   misses (Voyager patterns that DO transfer that the caveats over-
   discount)?
4. **Tier-2 group 3 (freeze-vs-refresh framing alternative).** Deepest
   Phase 0 finding still open from cycle 11; deferred for 5 cycles.
   Cycle 18 should attempt unless higher-priority work intervenes.
5. **Optional cold-reader-1 flag execution: add "Tool integration
   model" paragraph to the AutoGen navigation summary** (~10 lines
   between Trust boundaries and Anti-patterns). Bounded mechanical
   edit; cycle 18 has capacity if other priorities land.
6. **Optional cold-reader-2 flag execution: enrich AutoGen nav-bullet-4
   with the "behavior contracts as message protocols" framing** (~1
   sentence addition). Bounded mechanical edit; cycle 18 has capacity.

## Persistence-mechanism observations

Cycle-N-pre-commits-cycle-N+1-checks chain is now twelve cycles deep
(cycle 7 → 8 → 9 → 10 → 11 → 12 → 13 → 14 → 15 → 16 → 17 → 18
pre-committed). No breakdown.

**The dispatch-body-pre-loading-of-discipline pattern from cycle 16 has
its first re-application this cycle in the cycle-18 LangGraph dispatch
recommendation.** Cycle 16's positive proof point (cycle-15 pre-loaded
anti-smuggling discipline → cycle-16-evaluated dispatch produced
observation-shaped output) generalizes: the cycle-18 LangGraph dispatch
should pre-load both the anti-smuggling discipline (now standard) AND
the seven-lens calibration that cycle-15 pioneered. Treating dispatch
templates as discipline-propagation artifacts is now the default, not
an experiment.

**The deferred-flags-from-cold-readers pattern.** Cycle 16 flagged
three optional improvements at cold-reader verdicts (one per
pre-commit), defer-not-apply with reasoning. Cycle 17 follows the same
pattern: two cold-reader flags carried forward to cycle 18+ as
optional bounded edits. Pattern shape: cold-reader produces verdict +
optional flags + defer-or-apply decision, with the apply decision
based on cycle-capacity-vs-opportunity-cost. The pattern lets
cold-readers do their work without forcing same-cycle rewrites that
might not be the highest-leverage use of cycle time.

**The deliverable-size asymmetry observation from cycle 16 partially
addressed this cycle.** Cycle 16 noted: AutoGen deep-dive (697 lines) +
its 140-line summary vs openclaw (60 lines) and PAI (130 lines) creates
asymmetry that biases cross-system synthesis. Cycle 17's Voyager read
adds ~190 lines at orchestrator-direct depth — comparable to AutoGen's
summary depth but not the deep-dive. Now four systems studied at
varying depths:
- openclaw: 60 lines first-pass
- PAI: 130 lines first-pass
- AutoGen: 140 lines summary + 697 lines deep-dive
- Voyager: ~190 lines orchestrator-direct

Adding LangGraph at Copilot deep-dive depth (cycle 18+) would bring
two deep-dives + two summary-level reads + two first-pass reads.
Cross-system synthesis is still better deferred until at least 5
systems are read at the summary-or-deeper depth.

**Phase-0 / Phase-1 split holding.** Three cycles (14, 15, 16) ran
parallel-Phase work; cycle 17 continued with Voyager (Phase 1) plus
two cold-readers (Phase 1 internal). The combined cycle scope works.
Cycle 17 did NOT do any Phase 0 work (no F-pattern revisions, no
adversarial re-read of `0-retrospective.md`); the Phase 0 retrospective
remains in its cycle-16 state. This is fine — Eva's input #2759
authorized parallel work, not parallel work-must-be-applied-each-cycle.
Cycles where Phase 1 dominates are appropriate when the Phase 1 work
has accumulated pre-commits.

## Reconciling cycle scopes

Cycle 14: 3 cold-readers + Phase 1 initiation (`1-research.md` draft +
2 system reads).

Cycle 15: 3 cold-readers + 1 adversarial re-read with edit + 1 dispatch
+ 1 Tier-2 group execution.

Cycle 16: 3 cold-readers + AutoGen dispatch evaluation + Tier-1
AutoGen integration (~140 lines added to `1-research.md`).

Cycle 17: 2 cold-readers + Voyager orchestrator-direct read +
integration into `1-research.md` (~190 lines added) + work-plan
update + persistence-mechanism-note update + this notes file +
README iteration-log entry.

All four cycles fit the ~75-min envelope. Cycle 17 is moderately
loaded but bounded — the Tier-2 group 3 deferral and the optional
cold-reader-flag deferrals prevent over-extension. Pattern holding:
multi-component cycle (cold-readers + new-system-read or dispatch-
evaluation + integration + persistence updates) is the cycle shape
that fits sustainably.
