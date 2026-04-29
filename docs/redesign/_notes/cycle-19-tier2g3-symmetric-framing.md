# Cycle 19: Tier-2 group 3 (symmetric framing reframe) + cold-reader 1 flag c1.1 + LangGraph preview

Cycle 18 (commit `2b7bc3bf`) ran two Voyager cold-readers, applied two
optional flags from cycle 17, scoped Tier-2 group 3 explicitly for
cycle 19+, and dispatched LangGraph research-only. Six pre-commits
queued for cycle 19. PR #2768 (LangGraph deliverable) landed at
00:34 UTC on 2026-04-29 — faster than cycle-18's 2-3 day estimate,
which means cycle 19 faces both pre-commits (1 Tier-2 group 3 + 2
LangGraph evaluation) competing as cycle 18 anticipated.

Cycle 19 plan (per cycle-18 "split if needed; pre-commits are options,
not contracts"):

- **Primary**: Tier-2 group 3 execution per option (b) symmetric framing
  reframe. The 6th-defer-is-a-smell pattern from cycle 18 says this
  gets the focal cycle even with LangGraph competing.
- **Secondary**: LangGraph PR brief preview + cycle-20 explicit scoping.
  Full per-finding evaluation defers; previewing avoids "PR sat for
  cycles" while not overloading cycle 19.
- **Tertiary**: Cold-reader 1 flag c1.1 (Voyager bullets 15-16 trim).
  Bounded mechanical, ~2 line edit. Applied per the cycle-18
  apply-when-bounded-mechanical-AND-capacity rule.

Cold-reader 1 flag c1.2 (compositionality elevation), cold-reader 2
flags c2.1/c2.2 (transferability observations) defer to cycle 20+.

## Tier-2 group 3 execution: symmetric framing reframe

### Background

Cycle-7 adopted Copilot PR #2749's freeze-vs-refresh formulation as
the load-bearing statement on defense-accretion timing, with the
specific wording "Several v1 defenses are implemented as end-of-cycle
or next-cycle refreshers. Because artifacts freeze before those
refreshers finish, the architecture guarantees post-close divergence
between frozen artifacts and live state."

Cycle 12's Copilot PR #2756 lens 3.B (strong accept; "the most
consequential finding in the entire critique") identified that this
framing picks one side of a bilateral mismatch without arguing the
choice: "the 'run defenses earlier' framing is genuinely an alternative
the artifact does not argue against."

Cycle 12's evaluation deferred 3.B to a dedicated cycle, naming the
fix as either: (i) make the case for "freeze too early" over "defenses
too late" explicit, or (ii) acknowledge both framings.

Tier-2 group 3 was deferred in cycles 13, 14, 15, 16, 17, and (with
explicit scoping) cycle 18. Cycle 19 is the focal execution cycle.

Cycle 18's recommended option (b) symmetric framing reframe:

> "freeze-vs-refresh is a bilateral mismatch with no architecturally-
> correct side; either earlier defense or later freeze would resolve
> it; v1 chose to add defenses without changing freeze timing, and the
> defenses run too late *because* freeze runs too early relative to
> defense timing."

### Three locations rewritten (per cycle-18 scoping)

1. **Family preamble — family-overlap paragraph** (lines 183-206 after
   edit). Before: "F11 ... is also where the freeze-vs-refresh timing
   collision (F11's local mechanism) plays out: artifacts freeze before
   refreshers finish, structurally producing post-close divergence
   between frozen artifacts and live state." After: bilateral framing
   explicit, three resolutions named (earlier defenses; later freeze;
   continuous reconciliation), v1's asymmetric response named, original
   "artifacts freeze before refreshers finish" wording preserved as
   "describes one side of the mismatch, not its architectural cause."

2. **F11 architectural-implication paragraph** (lines 741-758 after
   edit). Before: italicized first sentence said "Because artifacts
   freeze before those refreshers finish, the architecture structurally
   produces post-close divergence." After: italicized first sentence
   neutralized to "the worklog freezes at C5 before those refreshers
   finish; the architecture structurally produces post-close
   divergence." Then a new sentence names the bilateral framing and
   three resolutions. v1's asymmetric response named: "an asymmetric
   response that preserves the mismatch rather than dissolving it."

3. **F12 hypothesis substrate paragraph** (line 920 after edit). Minor:
   "freeze-vs-refresh timing collision" → "freeze-vs-refresh timing
   mismatch" with parenthetical pointer to family preamble for the
   bilateral framing and three resolutions. The substrate paragraph
   doesn't restructure; it cites F11's local mechanism, and the
   bilateral framing is established in the family preamble.

### Cold-reader pass on the rewrite

Real finding: family-preamble closing line over-claimed "v2 design
implications for the two families converge across the three
resolutions" — but the listed implications (write-tool +
reconciliation-tool, poller, checkpoint markers) are option-3-specific
(continuous reconciliation), not actually convergent across all three.

Fix applied this cycle: split the closing into "partially overlap" for
the genuinely cross-resolution implications (reconciliation-tools,
pollers — needed regardless of which resolution v2 picks) and
explicit option-3-specificity for the checkpoint-markers prescription.
Final closing notes that the choice between hard freeze (options 1/2
augmented) and continuous reconciliation (option 3) is a Phase 2
design choice.

Mild residual tension: the dedicated "v2 design implications by
family" section's defense-accretion blockquote (lines 957-961, not
in cycle-18 scope) still uses imperative voice ("Cycle boundaries
should be checkpoint markers...") which is option-3-leaning. The
section header hedges ("for cycle-by-cycle reference during Phase 2")
so the tension is mild but real. Defer to cycle 20+ flag.

### Other freeze-vs-refresh occurrences checked

Five occurrences total post-edit:
- Line 186 (family-overlap paragraph) — rewritten ✓
- Line 745 (F11 architectural-implication) — rewritten ✓
- Line 772 (F11 paragraph local-mechanism listing: "F11's is
  *freeze-vs-refresh*") — name as identifier; symmetric framing in
  preamble carries the bilateral nature; no edit needed
- Line 778 (F11 paragraph "fixing the freeze-vs-refresh timing problem
  (F11/F4) automatically resolves...") — argues against v1's reflex of
  hoping a single substrate-fix automatically resolves others; still
  correct under bilateral framing; no edit needed
- Line 920 (F12 substrate paragraph) — minor edit ✓

The F11 hypothesis section at line 782 picks option 3 ("State evolves
continuously; cycle should be a checkpoint event ... not a state
hard-boundary") with a substantive argument ("v1 inherited
cycle-as-hard-boundary from the trigger model. The trigger boundary
is necessary at the GitHub Actions layer; treating it as a
state-of-record boundary is the bug"). Cycle-12 lens 3.B's concern
was about picking-without-arguing; the hypothesis section IS where
arguing is appropriate. Symmetric framing in the family preamble +
F11 architectural-implication is sufficient acknowledgment that
multiple options exist; the hypothesis section then makes the case
for option 3. Right structure, no edit.

### Net edits

- `0-retrospective.md`: family-preamble paragraph 11 → 24 lines (+13
  including the cold-reader-fix split closing); F11 architectural-
  implication paragraph 12 → 19 lines (+7); F12 substrate paragraph
  +1 phrase (parenthetical pointer). Net retrospective: ~+21 lines.

## Cold-reader 1 flag c1.1 (Voyager bullets 15-16 trim)

Cycle 18 cold-reader 1 found bullets 15-16 contained post-prose specs
not lifted from the prose paragraphs above:
- Bullet 15's "lossy summaries" framing
- Bullet 16's "vectordb-keyed cached question→answer pairs" mechanism
  specification

Two options: (A) trim the bullets to match prose, (B) add the
corresponding sentences to the prose paragraphs. Picked (A) as
bounded-mechanical and preserving the "bullets summarize prose"
discipline.

### Edits applied

Bullet 15 before: "LLM-generated skill descriptions as the embedding
surface (descriptions are lossy summaries; vectors index the summary,
not the code)"

Bullet 15 after: "LLM-generated skill descriptions as the embedding
surface (embeddings are over descriptions, not over raw code)"

The "embeddings are over descriptions, not over raw code" phrasing is
prose-faithful (line 459-461: "embeddings are over LLM-generated skill
descriptions, not raw code"). "Lossy summaries" framing dropped.

Bullet 16 before: "QA-cache pattern for repeated curriculum lookups
(vectordb-keyed cached question→answer pairs)"

Bullet 16 after: "QA-cache pattern for repeated curriculum lookups
(`qa_cache.json` plus vectordb of cached questions, kept in sync)"

The new parenthetical mirrors prose lines 435-436 (`qa_cache.json`,
vectordb of cached questions in `ckpt/curriculum/`) plus the sync
invariant from lines 444-447 (CurriculumAgent asserts the vectordb
count matches `qa_cache.json` entries). "Question→answer pairs"
framing dropped (the prose doesn't make the question-vs-answer
distinction explicit).

Both edits are bounded mechanical: ~2 lines net change to bullets, no
prose changes.

## LangGraph PR #2768 brief preview

PR landed at 00:34 UTC on 2026-04-29. Single file
`docs/redesign/_notes/cycle-18-langgraph-research.md`, 797 lines
(comparable to AutoGen's 697-line PR #2763). Source-grounded:
explicit list of LangGraph docs files read (`overview.mdx`,
`graph-api.mdx`, `pregel.mdx`, `persistence.mdx`, `add-memory.mdx`,
`use-time-travel.mdx`, `durable-execution.mdx`, `interrupts.mdx`,
`streaming.mdx`, `use-subgraphs.mdx`, `workflows-agents.mdx`,
`choosing-apis.mdx`) plus source files (`graph/state.py`,
`pregel/main.py`, `types.py`, checkpoint base/Postgres/SQLite, store
base, `tool_node.py`, `chat_agent_executor.py`).

Honest scoping note up-front: old `langchain-ai.github.io/langgraph/`
site unavailable, used the current `langchain-ai/docs` source files
which are the current authoritative docs source.

First ~120-line scan suggests anti-smuggling discipline honored: the
section headers are observation-shaped ("Overall architecture and
named primitives", "State representation, persistence, and
time-travel"), code excerpts and quoted doc text dominate, no
v2-relevance framings visible in the headers or first few sections.
Per-finding evaluation deferred to cycle 20.

### Cycle 20 LangGraph evaluation scoping

Per cycle-7 / cycle-12 / cycle-15 per-finding evaluation discipline:

1. **Read PR #2768 in full** before evaluating. ~797 lines, comparable
   to AutoGen at 697.
2. **Per-finding evaluation** — the LangGraph deliverable is research
   not critique (different evaluation discipline applies, per cycle-15
   framing). For research deliverables: walk each named primitive /
   pattern / architectural concern observed in the source; assess
   accuracy (does the prose match the cited code/docs?), assess
   anti-smuggling (any v2-relevance framings smuggled?), tag for
   Tier-1 vs Tier-2 integration.
3. **Tier-1 integration target**: ~140-line LangGraph navigation
   summary in `1-research.md` paralleling AutoGen and Voyager's
   structure (project-status, named primitives, state representation
   as primary value-prop section, Pregel/super-step model, persistence
   /checkpointing, multi-agent patterns, what LangGraph doesn't
   guarantee, anchoring caveats with explicit transferability
   discussion per cycle-18 finding, Patterns observed list).
4. **Tier-2 cross-system synthesis**: at 5 systems read (openclaw,
   PAI, AutoGen, Voyager, LangGraph), cross-system observations
   become load-bearing-feasible. Cycle-18 cold-reader 2 transferability
   observations (c2.1 failed-task-as-recorded-artifact; c2.2
   code-vs-prompts-split) gate-released for inclusion at this point.
5. **Anchoring-caveats discipline**: per cycle-18 finding, the
   anchoring-caveats section for LangGraph should explicitly note
   transferability (positive arguments per pattern), not just
   non-transfer concerns.

### What to look for in the LangGraph deliverable

Per the dispatch's seven-lens framing (state-management as primary):

- **State representation**: TypedDict + reducers / channels model;
  how it compares to AutoGen's component-local-dictionaries and
  Voyager's component-local-persistence
- **Persistence**: checkpointer / store interfaces; thread-scoped
  histories; time-travel mechanism
- **Orchestration**: Pregel super-step model; graph API vs functional
  API; multi-agent patterns (subagents, handoffs, router, custom
  workflow)
- **Failure handling**: durable execution, interrupts, streaming
- **Tool integration**: ToolNode, create_react_agent prebuilt agents;
  comparison with AutoGen's schema-driven tool calling
- **Anti-patterns**: what LangGraph explicitly forbids or warns
  against in its docs
- **Anchoring caveats**: vendor coupling (LangChain ecosystem),
  research vs production posture, transferability discussion per
  cycle-18 cold-reader 2 finding

Estimated cycle-20 scope: per-finding evaluation + Tier-1 integration.
Tier-2 cross-system synthesis (cycle-18 c2.1/c2.2 transferability
observations) likely cycle 21+ given cycle-20's scope.

## Long-deferred items roll-call (carried forward)

1. Journal-entry self-congratulation sweep (13 cycles deferred from
   cycle 7+)
2. F6/F8/F9 measurements (cycle 7+)
3. Refactor-for-length F-section sweep (cycle 8+; Tier-2 group 8)
4. Persistence-mechanism deferred-list consolidation (cycle 13 flag)
5. Tier-2 group 2 (review/disposition substrate, partial integration
   cycle 13)
6. ~~Tier-2 group 3 (freeze-vs-refresh framing alternative)~~ —
   **RESOLVED THIS CYCLE** ✓
7. Tier-2 group 4 (nine measures rework)
8. Tier-2 group 6 (preserved-through-cutover disposition)
9. Tier-2 group 7 (resolved open questions collapse)
10. Tier-2 group 9 (F8 singleton-family acknowledgment)
11. **NEW**: v2 design implications by family blockquote tightening
    (cold-reader-found this cycle; option-3-leaning imperative voice
    now in tension with bilateral preamble; bounded mechanical, ~2-3
    line qualification)

Cycle 19 added 1 item (item 11), resolved 1 (item 6: Tier-2 group 3).
Net: 10 → 10 long-deferred items, with one structural-finding
resolved and one cold-reader-found.

## Cycle 20+ pre-commits

1. **LangGraph dispatch evaluation + Tier-1 integration.** PR #2768
   read in full; per-finding evaluation in dedicated notes file;
   Tier-1 integration into `1-research.md` (~140 line nav summary
   paralleling AutoGen/Voyager structure). Anchoring-caveats section
   uses cycle-18 transferability discipline.

2. **Cold-reader on Tier-2 group 3 rewrite.** A fresh look at the
   bilateral-framing preamble + F11 architectural-implication +
   F12 substrate paragraph: does the bilateral framing read as
   load-bearing-balanced or as evading the "v1 chose poorly" point
   that the bilateral framing was meant to refine, not erase?

3. **v2 design implications blockquote tightening** (item 11).
   The defense-accretion blockquote uses imperative voice ("Cycle
   boundaries should be checkpoint markers...") that is in mild
   tension with the bilateral preamble's "Phase 2 design choice"
   framing. Bounded mechanical, ~2-3 line qualification.

4. **Cold-reader on c1.1 fix** (Voyager bullets 15-16 trim). Did
   the trim drop signal that should have been in the prose? Or
   should some of the dropped framing ("lossy summaries",
   "question→answer pairs") be added back as prose? This is the
   inverse of what was done this cycle.

5. **Compositionality elevation** (cycle-18 cold-reader 1 flag c1.2).
   Consider elevating the compositionality-as-named-learning-mechanism
   observation to the Voyager Patterns observed list. Bounded
   mechanical, ~1 line addition.

6. **Cross-system transferability observations** (cycle-18 cold-
   reader 2 flags c2.1, c2.2). Failed-task-as-recorded-artifact and
   code-vs-prompts-split. Gate-released at 5 systems read; LangGraph
   completes that count after cycle-20 integration. Land in
   cross-system observations section.

## Persistence-mechanism observations

The cycle-N-pre-commits-cycle-N+1-checks chain is now fourteen cycles
deep (cycle 7 → 8 → 9 → 10 → 11 → 12 → 13 → 14 → 15 → 16 → 17 → 18 →
19 → 20 pre-committed). No breakdown. The cycle-18-named
sixth-defer-is-a-smell pattern was tested this cycle: Tier-2 group 3
was at 6-cycles-deferred coming into cycle 19; cycle 19 executed it
rather than silent-deferring or scoping-deferring further. Pattern
held: explicit scoping in cycle 18 enabled bounded execution in
cycle 19.

The cycle-18 apply-when-bounded-mechanical-AND-capacity rule was
applied this cycle to flag c1.1 (Voyager bullets 15-16 trim).
Pattern: cycle-N cold-reader produces flag → cycle-N+1 evaluates
threshold (bounded mechanical AND capacity) → apply if both met.
Cycle 19 had Tier-2 group 3 as primary load and still had capacity
for c1.1; deferred c1.2 (compositionality elevation) to cycle 20+.

The cold-reader-during-rewrite pattern produced one real finding
this cycle: the family-preamble closing line over-claimed convergence
across resolutions when the listed implications were option-3-
specific. The fix was bounded mechanical, applied within the same
cycle as the rewrite. Pattern: cold-reader on a substantive rewrite
should run before commit, not as a cycle-N+1 pre-commit, when the
rewrite is structurally significant. Cycle 20+ pre-commit 2 is a
fresh-eye cold-reader (different from the same-cycle cold-reader);
both have value.

## Reconciling cycle scopes

Cycle 14: 3 cold-readers + Phase 1 initiation (`1-research.md`
draft + 2 system reads).

Cycle 15: 3 cold-readers + 1 adversarial re-read with edit + 1
dispatch + 1 Tier-2 group execution.

Cycle 16: 3 cold-readers + AutoGen dispatch evaluation + Tier-1
AutoGen integration.

Cycle 17: 2 cold-readers + Voyager orchestrator-direct read + ~190
line integration.

Cycle 18: 2 cold-readers (with 1 substantive edit) + 2 optional-flag
executions + Tier-2 group 3 explicit scoping + LangGraph dispatch.

Cycle 19: Tier-2 group 3 execution (3 locations rewritten + cold-
reader-fix on family-preamble closing) + 1 bounded cold-reader-1
flag application (c1.1 Voyager bullets 15-16 trim) + LangGraph PR
brief preview + cycle-20 evaluation scoping.

Cycle 19 is moderately loaded. Tier-2 group 3 is the focal task;
LangGraph deferral with explicit scoping is the main capacity-
management decision; c1.1 fits because it's bounded mechanical and
the cycle had ~5 minutes of capacity remaining after the focal task.
