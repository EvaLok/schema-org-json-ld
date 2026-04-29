# Cycle 20: LangGraph PR #2768 per-finding evaluation + Tier-1 integration

Cycle 19 (commit `720d6524`) executed Tier-2 group 3 (the 6-cycles-deferred
freeze-vs-refresh symmetric framing reframe), applied cold-reader 1 flag c1.1
(Voyager bullets 15-16 trim), and gave PR [#2768](https://github.com/EvaLok/schema-org-json-ld/pull/2768)
a brief preview with cycle-20 explicit scoping. Cycle 19 left six pre-commits;
this cycle takes the focal one.

PR #2768 status entering cycle 20: OPEN, not merged. The cycle-19 entry's "PR
landed at 00:34 UTC" wording was loose — that was when the PR was opened, not
merged. Cycle 20's per-finding evaluation gates the merge decision.

Cycle 20 plan per cycle-19 scoping:

- **Primary**: Read PR #2768 in full (797 lines). Per-finding evaluation in this
  notes file. Tier-1 integration into `1-research.md` (~140-line nav summary
  paralleling AutoGen at 172 lines and Voyager at 232 lines).
- **Secondary** (capacity-permitting): fresh-eye cold-reader on Tier-2 group 3
  rewrite (cycle-19 pre-commit 2). Bounded scope — three paragraphs to evaluate.
- **Defer to cycle 21+**: items 3-5 (v2 design implications blockquote
  tightening; cold-reader on c1.1 fix; compositionality elevation). Item 6
  (cross-system transferability observations) was already gated on cycle-20's
  LangGraph integration; gate-released after this cycle.

## PR #2768 per-finding evaluation

The deliverable is `docs/redesign/_notes/cycle-18-langgraph-research.md`,
797 lines, single file. Source-grounded with explicit source list (12 docs
files + 8 source files + 5 multi-agent docs files). Honest scoping note
up-front (old `langchain-ai.github.io/langgraph/` site unavailable, used
current `langchain-ai/docs` source).

Evaluation discipline per cycle-15 / cycle-16: walk each named primitive /
pattern / architectural concern; assess accuracy (does prose match cited
code/docs?); assess anti-smuggling (any v2-relevance framings smuggled?);
tag for Tier-1 vs Tier-2 integration; verify the cycle-18 cold-reader-2
transferability discipline was honored in section 7.

### Section 1: Overall architecture and named primitives

**Accuracy.** Cites `overview.mdx`, `graph-api.mdx`, `pregel.mdx`. Names 14
core primitives explicitly (StateGraph, nodes, edges, state channels,
reducers, MessagesState/add_messages/MessagesValue, START/END, Command,
Send, Pregel, Checkpointer, Store, Runtime, ToolNode/create_react_agent).
The Pregel BSP framing is faithful: plan/execution/update phases match the
docs' own description. The hello-world example is small and code-cited.

The architectural split between Graph API and Functional API (`@entrypoint`
/ `@task`) is named with the choosing-APIs guide as source. The "shared
underlying runtime" claim is cited. This is non-trivial — the deliverable
distinguishes API style from runtime, which matters for understanding what
LangGraph's "core" actually is.

**Smuggling check.** Section header is observation-shaped. No v2-relevance
framing visible. PASS.

**Tier-1 inclusion.** YES — high-value for understanding LangGraph's
architectural shape vs AutoGen/Voyager.

### Section 2: State representation, persistence, time-travel

**Accuracy.** Eight subsections: state representation; reducers and
concurrent updates; checkpoints/threads/state-versions; checkpointer
interface and backends; pending writes and failure recovery; time travel
(replay and fork); short-term and long-term memory; implementation vs
marketing claims.

Per-subsection accuracy spot-checks:

- 2.1 (state rep): TypedDict / dataclass / Pydantic options listed. Pydantic
  performance caveat cited from `graph-api.mdx`. The note that
  `create_agent` "does not support Pydantic state schemas" is a real
  documented limitation. Multi-schema support (Input/Output/Overall/Private)
  is correctly framed: "a node _can write to any state channel in the graph
  state_" with the example schemas distinct.
- 2.2 (reducers): Annotated example with `operator.add`, ReducedValue
  TypeScript example, MessagesValue manual-update warning. The
  `BinaryOperatorAggregate`/`LastValue` channel imports from `state.py` are
  correctly cited as evidence reducers are real implementation, not just
  documented intent. The `Overwrite` bypass mechanism is named with the
  appropriate "useful but dangerous" framing.
- 2.3 (checkpoints/threads): "checkpoint at each super-step boundary" claim
  and the docs typo ("created" — sic) is honestly noted. `StateSnapshot`
  fields enumerated. The `Checkpoint` typed dict in the implementation
  source has more fields (`channel_versions`, `versions_seen`,
  `pending_sends`, `updated_channels`) than the public docs summary — the
  deliverable surfaces this gap. `checkpoint_ns` namespace shape for
  subgraphs (`""` for root, `"node_name:uuid"` for subgraph, joined by `|`)
  is concretely named.
- 2.4 (checkpointer interface): `BaseCheckpointSaver` methods listed
  (`get_tuple`, `list`, `put`, `put_writes`, async equivalents). The
  `WRITES_IDX_MAP = {ERROR: -1, SCHEDULED: -2, INTERRUPT: -3, RESUME: -4}`
  detail is real and material — special writes are persisted in the
  writes-table indexing rather than thrown as process exceptions. Backends
  list (in-memory, Postgres, SQLite, MongoDB, Redis) with explicit note
  that Redis/MongoDB are external packages, not in monorepo. Honest
  dev-vs-prod distinction noted.
- 2.5 (pending writes): "When a graph node fails mid-execution at a given
  super-step, LangGraph stores pending checkpoint writes from any other
  nodes that completed successfully at that super-step" — the "stronger
  recovery model than simply checkpoint-at-end" framing is accurate. The
  durable-execution-vs-line-level-replay caveat ("the code does **NOT**
  resume from the **same line of code**") is honest and important. The
  durability modes (exit/async/sync) tradeoffs are exposed rather than
  hidden.
- 2.6 (time travel): Replay vs fork distinction. The "Replay re-executes
  nodes—it doesn't just read from cache" warning is faithfully quoted.
  "Time travel is append-only branching inside a thread history, not
  destructive rollback" — this is a substantive observation that
  generalizes well.
- 2.7 (short-term vs long-term memory): "With checkpointers alone, we
  cannot share information across threads. This motivates the need for
  the `Store` interface." The two-tier separation (checkpoints = execution
  history; stores = application memory) is faithfully presented.
- 2.8 (implementation vs marketing claims): a value-judgment subsection
  that separates well-supported claims from claims-that-need-qualification.
  This is on-topic for research evaluation (assessing the docs against the
  code), not smuggling for v2 relevance. The qualifications are honest:
  "resume exactly where they left off" is checkpoint-granular not
  line-granular; time travel is not pure deterministic replay; durable
  execution does not auto-handle idempotence.

**Smuggling check.** Section 2.8's "Implementation appears to deliver vs
marketing claims" framing is value-judging the LangGraph docs/code, not
v2-relevance. PASS — it's research evaluation discipline.

**Tier-1 inclusion.** YES — this is the densest and most distinctive area
per the deliverable's own framing, and the redesign's persistence problem
maps to LangGraph's heavy investment in checkpointing/Store/time-travel.

### Section 3: Orchestration / planning patterns

**Accuracy.** Workflows-vs-agents distinction with `workflows-agents.mdx`
citation. 11 named patterns (prompt chaining, routing, parallelization,
orchestrator-worker, evaluator-optimizer, ReAct, subgraphs, subagents/
supervisor, handoffs, skills, router, custom workflow). Subgraph two-pattern
distinction (different schemas → wrapper node; shared channels → compiled
subgraph as node). Supervisor vs router distinction faithfully cited.
Handoff pattern's state-driven mechanism (`Command(update=...)` from tools
+ matching `ToolMessage`) accurately summarized.

The note that "plan-and-execute" / "swarm" are not first-class in core
LangGraph docs is appropriately scoped — the deliverable does not claim
first-class status it cannot find. It points to ecosystem packages
(Deep Agents) as the source of those patterns. Good honesty.

The position-against-reflexive-multi-agent quote ("not every complex task
requires this approach—a single agent with the right (sometimes dynamic)
tools and prompt can often achieve similar results") is cited from the
multi-agent overview. This is an explicit non-goal worth tracking — it
matches openclaw's anti-pattern stance against agent-hierarchy frameworks
as default.

**Smuggling check.** Section header observation-shaped. No v2-relevance
framings. PASS.

**Tier-1 inclusion.** YES — the multi-pattern-as-first-class observation
is a comparable to AutoGen's same observation; the cross-system
convergence is itself a Phase 2-relevant signal.

### Section 4: Failure handling and recovery

**Accuracy.** Eight subsections covering Pregel step failure, retry
policies, conditional/fallback edges, malformed output, rate limits, loop
limits, interrupts, review/edit state. The "interrupts as the most
distinctive recovery/control feature" framing is accurate — interrupts are
LangGraph's primary HITL primitive.

Important honesty in 4.2 (retry policies): "Node-level retry is a
deterministic runtime feature; it is not an LLM self-correction loop."
This distinguishes retry-on-exception from retry-on-bad-output, a
distinction other systems' docs sometimes blur.

The interrupt restart-from-beginning warning (`interrupts.mdx`: "The node
restarts from the beginning of the node where the `interrupt` was called
when resumed") is repeated for emphasis. This is a non-trivial caveat —
treating interrupts as language-level continuations is a documented
anti-pattern.

**Smuggling check.** Section header observation-shaped. PASS.

**Tier-1 inclusion.** YES — the failure-handling layer model (multiple
mechanisms, not one unified feature) is a comparable to AutoGen's
"failure handling delegated" observation.

### Section 5: Tool / skill integration model

**Accuracy.** Three-level tool integration (custom node, model-with-bound-
tools, ToolNode) is faithful. `ToolNode` source citation
(`tool_node.py`) is accurate. `InjectedState` and `InjectedStore` injection
mechanism named with the trust-boundary framing ("model-visible tool
schema need not expose internal state/store parameters").

Important distinction in 5.6: "LangChain's skill pattern is context loading,
not necessarily a separate executable capability. ... It is documented under
LangChain multi-agent, not core LangGraph." — this distinguishes LangChain
"Skills" from this repository's Claude Code skills (which are different,
both in scope and in execution model). Avoiding terminology collision
matters.

**Smuggling check.** Section header observation-shaped. PASS.

**Tier-1 inclusion.** PARTIAL — the tool/agent boundary observation
generalizes; LangChain-specific decorator details are evidence-base
content (PR text), not nav-summary content.

### Section 6: Anti-patterns and explicit non-goals

**Accuracy.** 11 explicit anti-patterns, each with a docs citation:
- LangGraph non-goal: architectural opinionation
- Use a higher-level agent when sufficient (anti-ceremony)
- Pydantic state schemas in `create_agent`
- Skip graph compilation
- Persist transient runtime objects (UntrackedValue mentioned)
- Treat interrupts as line-level continuations
- Wrong Command shape passed to invoke/stream
- Treat replay as cache replay
- Assume checkpointers make side effects safe
- v2 streaming format ("use this instead" direction)
- Spun-out post-LangChain boundary (clean layering)

The "kitchen-sink-warning" pattern matches AutoGen's same kitchen-sink
warning on `AssistantAgent`. Cross-system convergence on anti-patterns is
itself an observation worth tracking.

**Smuggling check.** Section is explicitly framed as "explicit non-goals"
— the deliverable's own discipline. PASS.

**Tier-1 inclusion.** YES — the explicit-non-goals pattern joins
AutoGen's "v0.2 anti-patterns" as a cross-system observable.

### Section 7: Anchoring caveats — cycle-18 transferability discipline check

**This is the section that the cycle-18 dispatch's lens-7 instruction
explicitly targeted.** Cycle 18's cold-reader 2 finding was that anchoring
caveats are one-directional (they enumerate non-transfer concerns) and the
opposite failure mode (over-discounting via blanket caveats) is not named.
The dispatch instructed Copilot to add per-caveat transferability arguments.

**Did the deliverable honor this?** YES, with explicit per-caveat
"Discounts" and "Transfers" subsections.

The 6 caveats:
1. **Library for stateful applications vs specific autonomous GitHub
   orchestrator** — Discounts: API ergonomics for arbitrary developers,
   Agent Server deployment assumptions, generic visualization benefits.
   Transfers: explicit state schemas + per-field reducers, checkpoint
   vs long-term store separation, append-only fork pattern, super-step
   boundaries as recovery units.
2. **Human user invokes graphs vs autonomous cron** — Discounts: thread_id
   as conversation cursor, indefinite-wait interrupts, UX-driven streaming.
   Transfers: stable execution identity (could be cycle/issue/PR ID),
   interrupt semantics for approval gates, checkpointed pause/resume even
   without interactive resumer, stream events as machine-consumed audit.
3. **Python/TypeScript library vs Rust tools** — Discounts: TypedDict /
   Pydantic / Annotated / decorators don't transfer literally. Transfers:
   typed state, channel reducers, checkpoint IDs, parent links, pending
   writes, namespaces are language-independent; Rust has good enum/struct
   support; deterministic-execution-vs-LLM-proposal split transfers;
   idempotence requirements are language-independent.
4. **Short-to-medium-running apps vs hundreds of cycles** — Discounts:
   per-thread histories may become huge, time-travel may be expensive
   long-term, DB checkpointer durability may be weaker than git/GitHub
   over very long horizons. Transfers: durable execution warning becomes
   *more* important not less; append-only histories and parent pointers
   are useful long-horizon; short-term vs long-term memory split is
   especially relevant when histories grow; state versioning useful in
   long-running parallel work.
5. **Database checkpointers vs git-tracked files and GitHub issues** —
   Discounts: transaction semantics, primary-key lookup, Agent Server
   hiding persistence. Transfers: checkpoint records as files/commits if
   schema is explicit; parent links as commit ancestry; pending writes
   as partially-completed parallel tasks; cross-thread Store vs thread
   checkpoints maps to repo-wide vs cycle-local state.
6. **LangGraph as post-LangChain stateful pivot** — Discounts: design
   choices shaped by LangChain interop, tool schemas optimized for
   chat-model ecosystems, LangSmith product boundary affects defaults.
   Transfers: layering high-level agents on lower-level deterministic
   runtime is a general pattern; keeping model/tool integration separate
   from execution-state machinery is transferable; exposing low-level
   state operations (rather than hiding everything behind agents) is
   transferable.

**Verdict on the discipline check**: The cycle-18 dispatch's instruction
was honored. Each caveat has both Discounts (the original
non-transfer-concerns shape) and Transfers (the cycle-18 mandated
positive-arguments-per-pattern). The Transfers content is substantive
not formulaic — caveat 4's observation that durable execution warning
becomes *more* important at hundreds of cycles is a non-obvious insight,
as is caveat 5's mapping of pending-writes-concept to GitHub-resident
partially-completed parallel tasks.

**Smuggling check.** The "Transfers" subsections are positive arguments
about what could transfer, framed at pattern-level (not specific v2
prescriptions). E.g., "checkpoint records can be represented as
files/commits if the schema is explicit" — this is a general transfer
claim, not "v2 should use checkpoint files." The framing distinction is
preserved. PASS.

**Tier-1 inclusion.** YES — this is the most distinctive Phase-1-cross-
system-discipline contribution from the LangGraph deliverable, and
recurring discipline references will reinforce the pattern.

### Patterns observed list

~60 bullets, observation-shaped. Spot-checked accuracy against earlier
sections:
- "Pregel/Bulk Synchronous Parallel super-steps" — Section 1 supports
- "Per-channel reducers, with overwrite as the default" — Section 2.2
  supports
- "Checkpoints saved at super-step boundaries" — Section 2.3 supports
- "Pending writes for successful siblings in failed super-steps" —
  Section 2.5 supports
- "Time travel re-executing later nodes rather than reading cache" —
  Section 2.6 supports
- "Nodes restarting from beginning after interrupt resume" — Section 4.7
  supports
- "Short-term memory as thread-level checkpointed state" / "Long-term
  memory as cross-thread Store" — Section 2.7 supports
- "Subgraph as graph-used-as-node" / "Compiled subgraph directly as node
  for shared channels" — Section 3 supports
- "Tools returning `Command` for state/routing changes" — Section 5
  supports

The bullet list maps cleanly to prose. No v2-relevance smuggling in
bullet phrasing.

**Smuggling check.** PASS — bullets are observations.

**Tier-1 inclusion.** Selective — pick ~12-16 of the most distinctive
observations for the nav-summary pattern list, paralleling AutoGen's 14
and Voyager's 16.

### Overall evaluation

- **Source-grounding**: STRONG. Extensive cited references to docs and
  source files; claims trace to either quoted doc text or named
  implementation files.
- **Anti-smuggling discipline**: HONORED. Section headers
  observation-shaped; no v2-prescription framings detected. Section 2.8's
  value-judging subsection is research evaluation discipline (assessing
  docs vs code), not v2 smuggling.
- **Cycle-18 transferability discipline**: HONORED. Section 7's per-caveat
  Discounts/Transfers structure is exactly what the cycle-18 dispatch
  instructed. The Transfers content is substantive.
- **Comprehensiveness**: HIGH. All seven dispatch lenses covered with
  appropriate depth. The state/persistence section (densest by the
  deliverable's own framing) gets eight subsections.
- **Honesty markers**: STRONG. Subsection 2.8 "Implementation appears to
  deliver vs marketing claims" honestly qualifies what's implemented vs
  what's marketed. The "scoping note" up-front is honest about the
  unavailable old docs site. Multiple docs caveats noted (e.g., the docs
  typo "created" sic).

**Merge decision**: LEAVE OPEN per the cycle-6/11/15 convention. PRs
[#2749](https://github.com/EvaLok/schema-org-json-ld/pull/2749) and
[#2763](https://github.com/EvaLok/schema-org-json-ld/pull/2763) are both
still OPEN and serve as canonical evidence-base locations for cycle-7
Copilot feedback critique and cycle-15 AutoGen research respectively.
Branch protection blocks merge to master; the open PR IS the canonical
evidence-base location, with cross-references from the Tier-1 nav summary
in `1-research.md` pointing to PR #2768. This cycle continues that
convention for #2768.

## Tier-1 integration approach

Target: ~140-line LangGraph nav summary in `1-research.md`, paralleling
AutoGen's 172 lines and Voyager's 232 lines (Voyager is denser because it
was orchestrator-direct read with full source access; AutoGen is via
Copilot dispatch like LangGraph).

Structure mirrors AutoGen/Voyager:

1. Header with project meta-info
2. Source-provenance paragraph (this is dispatched research, evidence in PR)
3. Project status paragraph (LangGraph is active, post-LangChain spinout
   stance, layered relationship with LangChain agents and LangSmith)
4. Several **bold-labeled topical paragraphs** for major findings:
   - Pregel/super-step bulk-synchronous execution model
   - State as typed channels with per-key reducers
   - Checkpointing at super-step boundaries with thread-scoped histories
   - Pending writes as failed-super-step partial recovery
   - Time travel as append-only fork (NOT destructive rollback)
   - Short-term memory (checkpoints) vs long-term memory (Store)
   - Interrupts as checkpoint/resume (with restart-from-beginning warning)
   - Multiple orchestration patterns coexist (workflows + agents)
   - Subgraph composition (graphs as nodes, two patterns)
   - Explicit non-goal: architectural opinionation
   - Honest implementation-vs-marketing-claims subsection
5. **Anchoring caveats on LangGraph** (with cycle-18 transferability
   discipline note + 6 caveats)
6. **Patterns observed in LangGraph** (~14-16 bullets)

Estimated section length: ~140-160 lines depending on paragraph density.

## Cross-system convergence observations (preview)

Now at 5 systems read at depth (openclaw, PAI, AutoGen, Voyager, LangGraph).
Cycle-18 cold-reader 2 transferability observations (c2.1 failed-task-as-
recorded-artifact; c2.2 code-vs-prompts-split) gate-released for inclusion
once LangGraph integration lands. Some convergence patterns observable
already:

- **"Multi-agent is not a default" stance is shared** by openclaw (VISION's
  "What We Will Not Merge" anti-patterns), AutoGen (Magentic-One's
  human-approval recommendation; v0.4's removal of "too opinionated"
  patterns), and LangGraph (multi-agent docs explicitly say single-agent
  often suffices).
- **Component-local persistence pattern** appears in AutoGen
  (component-local dictionaries) and Voyager (per-agent ckpt directories),
  with LangGraph as the explicit-typed-channel variant (state channels
  with reducers).
- **Append-only / no-destructive-rollback** in LangGraph time travel
  matches the redesign's "draft-then-promote + append-only" pattern (Eva
  advisory #2408).
- **Cost-tiering across model invocations** appears in Voyager (gpt-4 for
  novel reasoning, gpt-3.5-turbo for cache lookups) — LangGraph has no
  built-in equivalent, but that's because LangGraph is a runtime not an
  agent; cost-tiering would live at the application layer.
- **Failed-task as recorded artifact** appears in Voyager (failed_tasks.json
  alongside completed_tasks.json) and could be argued at LangGraph's
  pending-writes layer (failed super-step writes ARE recorded, not
  silently discarded).
- **Code-vs-prompts split** — Voyager's `voyager/control_primitives/` (Rust-
  equivalent: tool crates) vs `voyager/prompts/` (LLM-instruction text)
  rule has direct analog in LangGraph: `ToolNode` deterministic execution
  vs model-emitted tool calls. Two systems converge on the deterministic-
  code-executes-LLM-proposes pattern.

These cross-system observations are PREVIEWS. Full integration into the
"Cross-system observations" section of `1-research.md` and into the
retrospective's transferability-discipline section deferred to cycle 21+
(per the cycle-18 c2.1/c2.2 flag list, gate-released this cycle).

## Cold-reader: Tier-2 group 3 rewrite (cycle-19 pre-commit 2)

The question per cycle-19 pre-commit 2: does the bilateral framing read
as load-bearing-balanced, or as evading the "v1 chose poorly" point that
the bilateral framing was meant to refine, not erase? Re-reading the three
rewritten locations as a fresh-eye reviewer.

**Location 1: Family-overlap paragraph** (lines 183-206 of `0-retrospective.md`).

Re-read produces three observations:

(a) The "v1 chose poorly" point IS preserved, in the phrase "v1 added
defenses at later timings without rethinking freeze timing — an asymmetric
response to a symmetric problem". The claim that v1's response is
asymmetric (and that's a flaw) is explicit. The bilateral framing has not
erased the "v1 chose poorly" point — it has refined it.

(b) The phrase "by v1's architectural choice" naming the freeze-at-C5 as
an explicit choice (not an inevitability) is the crucial element preventing
the bilateral framing from reading as "the mismatch is just a natural
state of affairs." Without that phrase, a reader might infer the mismatch
is some background condition both sides equally fail to address. With it,
the reader sees: v1 picked freeze-at-C5 deliberately, then added defenses
without revisiting that pick.

(c) The cycle-12 lens 3.B finding was that v1 picked one side WITHOUT
ARGUING THE CHOICE — not that v1 picked the wrong side. The current
phrasing honors this distinction: "asymmetric response to a symmetric
problem" criticizes the response-shape (asymmetric), not the choice of
side. The criticism is "not arguing the choice", consistent with the
cycle-12 finding.

**Location 1 verdict**: HOLDS. No edits required.

**Location 2: F11 architectural-implication paragraph** (lines 741-758).

Re-read produces two observations:

(a) The italicized first sentence is now neutralized to describe the
divergence outcome ("structurally produces post-close divergence") without
preferring a side. Good.

(b) The phrase "v1's response was to add defenses at later timings without
changing C5's freeze timing — an asymmetric response that preserves the
mismatch rather than dissolving it" is more concrete than location 1's
"asymmetric response to a symmetric problem", but expresses the same
finding. The phrasing is complementary, not contradictory: location 1 is
abstract; location 2 is concrete.

**Location 2 verdict**: HOLDS. The "v1 chose poorly" point is preserved
through "preserves the mismatch rather than dissolving it" — which
explicitly evaluates v1's response as flawed.

**Location 3: F12 hypothesis substrate paragraph** (line 920).

Minor edit only: "freeze-vs-refresh timing collision" → "freeze-vs-refresh
timing mismatch" with parenthetical pointer to family preamble. F12
hypothesis is not the appropriate place to elaborate on bilateral framing;
the parenthetical pointer is the right structure.

**Location 3 verdict**: HOLDS.

**Overall cold-reader verdict**: PASS. The bilateral framing reads as
load-bearing-balanced, not as evasion. The "v1 chose poorly" point is
preserved across all three locations, refined per the cycle-12 lens 3.B
finding from "v1 picked the wrong side" to "v1 picked one side without
arguing the choice." The cross-location consistency in framing
("asymmetric response to a symmetric problem" / "asymmetric response that
preserves the mismatch rather than dissolving it") reinforces the
discipline.

One observation worth recording for future cycles, not edit-worthy: a
reader skimming might miss the "v1's architectural choice" qualifier that
distinguishes the bilateral framing from "the mismatch is natural." This
qualifier appears once (in location 1). If the location-1 paragraph is
ever re-organized, this qualifier should be preserved with care — it
prevents the bilateral framing from sliding into a "both sides are equal,
so neither is wrong" reading.

No edits required this cycle.

## Cycle 20+ pre-commits

(Carry-forward from cycle-19 minus what cycle 20 closes.)

1. ~~**Fresh-eye cold-reader on Tier-2 group 3 rewrite** (cycle-19 pre-commit
   2)~~ — **CLOSED THIS CYCLE**. Verdict PASS, no edits required. See
   above for detail.
2. **v2 design implications blockquote tightening** (item 11; option-3-
   leaning imperative voice in `0-retrospective.md` lines 957-961).
3. **Cold-reader on c1.1 fix** (cycle-19 pre-commit 4). Did trimming the
   "lossy summaries" / "question→answer pairs" framings drop signal?
4. **Compositionality elevation** (cycle-18 cold-reader 1 flag c1.2;
   add compositionality-as-named-learning-mechanism to Voyager bullet
   list).
5. **Cross-system transferability observations** (c2.1 failed-task-as-
   recorded-artifact; c2.2 code-vs-prompts-split). Gate-RELEASED this
   cycle. Land in `1-research.md`'s Cross-system observations section
   in cycle 21+.
6. **Cold-reader on this cycle's LangGraph Tier-1 integration**. Standard
   cycle-N+1 fresh-eye pass on substantive new content.

## Long-deferred items roll-call (carry-forward)

1. Journal-entry self-congratulation sweep (14 cycles deferred from cycle 7+)
2. F6/F8/F9 measurements (cycle 7+)
3. Refactor-for-length F-section sweep (cycle 8+; Tier-2 group 8)
4. Persistence-mechanism deferred-list consolidation (cycle 13 flag)
5. Tier-2 group 2 (review/disposition substrate, partial integration cycle 13)
6. Tier-2 group 4 (nine measures rework)
7. Tier-2 group 6 (preserved-through-cutover disposition)
8. Tier-2 group 7 (resolved open questions collapse)
9. Tier-2 group 9 (F8 singleton-family acknowledgment)
10. v2 design implications by family blockquote tightening (item 11
    from cycle 19)

Net: 10 → 10. No items resolved or added this cycle (LangGraph integration
is Phase 1 work, not Phase 0 long-deferred).

## Persistence-mechanism observations

The cycle-N-pre-commits-cycle-N+1-checks chain extends to fifteen cycles
deep (cycle 7 → ... → 19 → 20 → 21 pre-committed). No breakdown.

The cycle-15/16 dispatch-evaluation-then-Tier-1-integration pattern
applied this cycle for the third time (AutoGen cycle 16, Voyager cycle 17
direct-read, LangGraph cycle 20). The pattern shape now stable:
- Cycle N: dispatch (or direct-read) for Phase 1 system
- Cycle N+1 (or N+2 if dispatch turnaround slow): per-finding evaluation
  + Tier-1 nav-summary integration
- Same cycle or N+1: cycle-N+1 cold-reader on the integration

Cycle 20 ran the per-finding evaluation + Tier-1 integration in the same
cycle as the LangGraph PR landed (different from AutoGen's cycle-15-
dispatch / cycle-16-evaluation split because LangGraph PR opened ~1 day
post-dispatch rather than ~3h). The pattern is robust to dispatch
turnaround variance.

The cycle-18 anchoring-caveats-symmetric-discipline named in `1-research.md`
was tested by this cycle's evaluation: did the LangGraph deliverable honor
the cycle-18 dispatch's lens-7 instruction to add transferability
arguments? Verdict: YES. The discipline propagated correctly from
cycle-18 cold-reader-2 finding → cycle-18 dispatch instruction →
cycle-18 deliverable structure.
