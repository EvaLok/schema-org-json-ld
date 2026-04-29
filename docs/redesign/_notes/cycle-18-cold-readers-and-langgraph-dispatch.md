# Cycle 18: Voyager cold-readers + LangGraph dispatch + optional flags

Cycle 17 (commit `1375d100`) did two cold-readers on cycle-15/16
AutoGen integration and added a ~190-line Voyager orchestrator-direct
read to `1-research.md`. Six pre-commits queued for cycle 18:

1. LangGraph Copilot research-only dispatch (canonical cycle-15 procedure)
2. Cold-reader on Voyager Patterns observed list (16 bullets) — internal
3. Cold-reader on Voyager anchoring caveats (8 enumerated)
4. Tier-2 group 3 (freeze-vs-refresh framing alternative; 5 cycles deferred)
5. Optional cold-reader-1 flag: AutoGen Tool-integration paragraph (~10 lines)
6. Optional cold-reader-2 flag: AutoGen nav-bullet-4 behavior-contracts framing (~1 sentence)

Cycle 18 ran cold-readers (2, 3), executed both optional flags (5, 6),
applied one substantive cold-reader-2 finding to `1-research.md`,
scoped Tier-2 group 3 explicitly (rather than executing or deferring
silently for the sixth time), and dispatched LangGraph research-only.
Cycle-19+ pre-commits at the bottom.

## Cold-reader 1: Voyager Patterns observed list vs prose paragraphs

The question (cycle-18 pre-commit 2): the Voyager section was written
orchestrator-direct (no separate Copilot deep-dive to compare against),
so the cold-reader is internal — does the 16-bullet Patterns observed
list at lines 535-569 of `1-research.md` accurately reflect the prose
paragraphs above it (lines 379-533)?

**Verdict: PASS with two minor flags.**

### Bullet → prose mapping

For each of the 16 Patterns observed bullets, locate the source prose
paragraph or sub-claim:

| # | Bullet | Source in prose |
|---|---|---|
| 1 | Four-agent architecture with explicit named roles | Paragraph "Four-agent architecture with explicit named roles" — direct match |
| 2 | Cost-tiering across agents | Paragraph "Cost tiering across agents" — direct match |
| 3 | Component-local persistence with `resume=True` opt-in | Paragraph "Component-local persistence" — direct match including resume opt-in claim |
| 4 | Sync invariants asserted at init | Paragraph "Sync invariants asserted at init" — direct match |
| 5 | Skill versioning as append-on-disk + replace-in-vectordb | Paragraph "Skill versioning is append-on-disk, replace-in-vectordb" — direct match |
| 6 | Top-k semantic skill retrieval over generated descriptions | Paragraph "Skill retrieval as semantic similarity" — direct match |
| 7 | Bounded retries on action failure with critic-critique | Paragraph "Iteration mechanism with bounded retries" — direct match |
| 8 | Failed-task accumulation in dedicated JSON file | **SUB-CLAIM** elevated from within "Iteration mechanism with bounded retries" paragraph (the `failed_tasks.json` sentence) — promotion is faithful but bullet treats it as peer pattern |
| 9 | Human-in-the-loop as configurable mode per agent | Paragraph "Mode toggleability for human-in-the-loop" — direct match |
| 10 | Curriculum warm-up gating observation-field disclosure | Paragraph "Curriculum warm-up gates context disclosure based on progress" — direct match |
| 11 | Explicit no-fine-tuning architectural commitment | Paragraph "No model fine-tuning" — direct match |
| 12 | Two-layer capability composition | Paragraph "Two-layer capability composition" — direct match |
| 13 | Prompts as external files split by sub-task | Paragraph "Prompts as external files split by sub-task" — direct match |
| 14 | Structured critic output (`{success: bool, critique: str}`) rather than free-form review | **SUB-CLAIM** elevated from within "Four-agent architecture" paragraph (the CriticAgent role description) — peer-status promotion |
| 15 | LLM-generated skill descriptions as the embedding surface (descriptions are lossy summaries; vectors index the summary, not the code) | Sub-claim within "Skill retrieval as semantic similarity" paragraph PLUS the parenthetical "lossy summaries" framing is **NEW content** not in prose |
| 16 | QA-cache pattern for repeated curriculum lookups (vectordb-keyed cached question→answer pairs) | Mentioned in "Component-local persistence" + "Cost tiering" paragraphs as `qa_cache.json` and as a gpt-3.5-turbo target, but the mechanism specification "vectordb-keyed cached question→answer pairs" is **NEW content** not in prose |

### Prose observations NOT elevated to bullets

The reverse check: things in prose paragraphs that don't appear in the
Patterns observed list:

- **Error-message-naming-failure-mode-and-remediation pattern** (Sync
  invariants paragraph): "Error messages name the failure mode and
  remediation ('Did you set resume=False ... You may need to manually
  delete the vectordb directory'). Dual-storage divergence is a
  fail-fast condition at boot, not a silent runtime error."
  Distinguishable as a discipline pattern from sync-invariants itself —
  the invariant is the rule, the error-message convention is the UX.
- **Code-vs-prompts split as architectural rule** (Prompts as external
  files paragraph): "Code handles variable injection; prompts hold
  instructions." A boundary-of-responsibility claim, not just "prompts
  are external."
- **Compositionality as the paper's named learning mechanism**
  (Two-layer composition paragraph): "Skills compose primitives; later
  skills compose earlier skills. Compositionality is the paper's named
  learning mechanism." The named-learning-mechanism framing is dropped
  from bullet 12, which only captures the layering shape.

### Verdict

PASS. The bullet list reflects the prose with two minor issues:

(a) **Bullets 15 and 16 contain framing/specification not in prose.**
Bullet 15's "lossy summaries" and bullet 16's "vectordb-keyed cached
question→answer pairs" are accurate elaborations but not lifted from
the prose. They're observation-shaped (no v2-relevance smuggling) but
the bullet list is meant to summarize the prose, not extend it.

(b) **Three prose observations are not elevated to bullets** — the
error-message-naming pattern, the code-vs-prompts split rule, and the
compositionality-as-learning-mechanism framing. These are first-order
observations about Voyager's design. Their absence isn't a bug if the
16-bullet count is intentional (selection criterion: most architecturally
distinctive observations); but the inverse-check surfaces them as
candidates for inclusion if a future cycle expands the list.

**No v2-relevance smuggling detected.** Bullet 14's "rather than
free-form review" framing has a mild "this is better than the
alternative" smell but is observational about what Voyager does vs the
alternative it doesn't do; not a v2-prescription.

**Optional cycle-19+ flags** (defer-not-apply):
- (c1.1) Either trim bullets 15 and 16 of the post-prose specs, or add
  the corresponding sentences to the prose paragraphs.
- (c1.2) Consider elevating one or more of the three not-elevated
  observations to the Patterns list. The compositionality-as-learning-
  mechanism is the strongest candidate (it's a named architectural
  commitment, parallel to bullet 11's no-fine-tuning).

Both flags are bounded-mechanical. Defer rationale: cycle 18 has
substantial scope (this cold-reader, cold-reader 2, optional flags 5/6,
LangGraph dispatch, Tier-2 group 3 scoping). The flags are minor
refinements; per the cycle-17 deferred-flags-from-cold-readers pattern,
they sit as cycle-19+ pre-commits.

## Cold-reader 2: Voyager anchoring caveats — right 8? transferability misses?

The question (cycle-18 pre-commit 3): are the eight anchoring caveats
the right eight? Are there transferability misses (Voyager patterns
that DO transfer that the caveats over-discount)?

**Verdict: PASS with one substantive finding (applied this cycle) and
two minor flags (deferred).**

### The 8 anchoring caveats (lines 495-533)

1. Continuous-runtime vs cold-cycle
2. Embodied environment with rich observations vs sparse repository state
3. Concrete execution feedback vs fuzzy outcome feedback
4. Skill = executable code in a sandbox vs tool = build-time artifact
5. Single agent vs multi-orchestrator
6. Internal curriculum vs externally-supplied curriculum
7. Single-LLM-vendor coupling vs multi-vendor
8. Research artifact vs production-grade target

### Per-caveat assessment

For each, two checks: (a) is the non-transfer concern real? (b) does
the caveat over-discount transferable patterns?

**Caveat 1: Continuous-runtime vs cold-cycle.** (a) Real — Voyager runs
as a single process holding agent state in memory across many tasks;
the redesign target runs in 75-minute cycles with cold restarts. (b)
**Over-discounts.** Voyager has explicit on-disk persistence with
`resume=True` opt-in per agent and init-time sync invariants. The
cold-cycle case is exactly what Voyager's resume path implements. The
caveat as-written says the redesign "must reconstruct equivalent
continuity from disk every cycle" but doesn't acknowledge that
Voyager's evidence-base IS that this is feasible.

**Caveat 2: Embodied environment vs sparse repository state.** (a)
Real — Voyager's "world" is biome/inventory/voxels with per-step rich
observations; the redesign's "world" is git repo + GitHub issues. (b)
**Scoped concern.** This caveat applies to Voyager's PERCEPTION patterns
(curriculum's world-observation rendering, critic's environment-event
parsing) but not to its STATE-MANAGEMENT patterns (sync invariants,
skill versioning, component-local persistence). The caveat as-written
is over-broad — it doesn't distinguish which patterns it discounts.

**Caveat 3: Concrete execution feedback vs fuzzy outcome feedback.**
(a) Real — Voyager's critic gets concrete environment events; the
redesign's outcome feedback (next-cycle retrospection, audit critique)
is fuzzier. (b) **Partial over-discount.** The redesign DOES have
concrete feedback signals (CI pass/fail on dispatched PRs, schema-org
JSON validation, test-suite results, build success). The caveat treats
"fuzzy outcome feedback" as monolithic; the actual situation is
mixed-fidelity (some signals are concrete, some are fuzzy).

**Caveat 4: Skill = executable code vs tool = build-time artifact.**
(a) Real — Voyager's skills are LLM-generated JavaScript; the redesign's
tools are reviewed Rust binaries. (b) **Distinction is correct, scope
of discount is too broad.** The caveat correctly identifies the timing
difference (runtime vs construction-time) but Voyager's skill
*organizational* patterns (top-k semantic retrieval over descriptions,
versioning discipline, sync invariants) are independent of when the
artifact was created. The caveat reads as if the timing difference
disqualifies the organizational patterns; it doesn't.

**Caveat 5: Single agent vs multi-orchestrator.** (a) Mixed. The
redesign DOES have multi-orchestrator (main + audit). But Voyager has
**four named agent roles in one runtime** (action / curriculum / critic
/ skill-library), which IS relevant to within-orchestrator role
decomposition. (b) **Framing-misleading.** "Single agent" is technically
correct at the Voyager-runtime level but obscures the four-role
within-runtime architecture. The more accurate Voyager characterization
is "single runtime, four roles."

**Caveat 6: Internal curriculum vs externally-supplied curriculum.**
(a) Real — Voyager's curriculum agent autonomously selects what to
learn; the redesign's "curriculum" comes from Eva, schema-org work, and
F-patterns. (b) **Partial over-discount.** Voyager's curriculum
*mechanism* (gating context disclosure based on progress, using QA-cache
to ground task selection) is reusable for any curriculum source. The
caveat correctly distinguishes source but may discourage looking at
mechanism transferability.

**Caveat 7: Single-LLM-vendor coupling vs multi-vendor.** (a) Real —
Voyager hardcodes ChatOpenAI/OpenAIEmbeddings via langchain. (b)
**Over-discounts.** The cost-tiering pattern (cheap model for cached/
derivative work, expensive for novel reasoning) maps cleanly to
multi-vendor (the redesign already does this with Anthropic for
orchestration + Copilot for dispatches at coarser granularity). The
vendor-coupling concern is implementation-detail, not pattern-relevance.

**Caveat 8: Research artifact vs production-grade target.** (a) Real —
unmaintained since 2023-07-27; some patterns may have been chosen for
paper-narrative reasons. (b) **Partial over-discount per pattern.**
Some patterns (sync invariants, fail-fast at boot, append-on-disk +
replace-in-vectordb) are production-discipline patterns that
paper-narrative reasons would NOT specifically motivate. The
research-vs-production discount applies more to high-level architectural
choices than to discipline patterns within them.

### Meta-finding: anchoring caveats are one-directional

The 8 caveats all enumerate non-transfer concerns. None of them argues
which patterns DO transfer despite the surface differences. The
anchoring-discipline section at the top of `1-research.md` (lines 32-61)
explicitly names confirmation bias on aligned principles as failure
mode #1 — but the **opposite failure mode (over-discounting via blanket
caveats) is not named**.

If every Voyager pattern is hedged with "but doesn't transfer because
X," the cross-system synthesis loses signal. The discipline should be
symmetric: caveats discount, but transferability requires positive
arguments — not just absence of caveats.

This is a substantive finding. It applies to the Voyager section
specifically but is a structural pattern: any future per-system
anchoring-caveats section will benefit from a "transferability requires
positive argument" framing rather than just listing non-transfer
concerns.

### Edit applied this cycle

Added a one-paragraph preamble to the Voyager anchoring-caveats section
explicitly naming the asymmetry: caveats argue non-transfer; transfer
requires positive arguments per pattern. This sits at the top of the
caveats list and is meant to apply to the whole list.

Also: added a parallel preamble to the AutoGen anchoring-caveats
section (which has the same one-directional structure) so the framing
is consistent across systems already covered.

### Transferability-miss observations (deferred)

Two patterns where the caveats are silent on transferability that does
seem real:

(c2.1) **Failed-task accumulation in a dedicated JSON file alongside
completed-task accumulation.** The redesign has F-pattern retrospective
which is conceptually similar but isn't implemented as a "failed-cycle
ledger" alongside successes. Voyager's pattern of treating failure as a
recorded artifact (not a transient) at the same persistence level as
success transfers to v2 even though Voyager's specific failure shape
(action-execution-failures) doesn't.

(c2.2) **Code-vs-prompts split as architectural rule.** Voyager's
"Code handles variable injection; prompts hold instructions" is a
boundary-of-responsibility claim directly relevant to v2's prompt design
(any place prompts contain procedure that could be code is a candidate
for the inverse-extraction CORE-DESIGN-PRINCIPLE asks). The caveats
section is silent on this transferability.

Defer both as cycle-19+ flags. They surface in cross-system synthesis
naturally; pre-committing them now would just queue more flags. They
belong in the cross-system observations section once enough systems
are read at depth.

## Optional flag 5 (from cycle 17): AutoGen Tool integration model paragraph

Cycle 17's cold-reader 1 flagged: deep-dive §5 "Tool / skill integration
model" is partially covered in `1-research.md` AutoGen section. Trust
aspects appear as a dedicated "Trust boundaries explicitly named"
paragraph, but schema-driven tool calling, agent-as-tool composition,
v0.4 routing simplification, parallel-tool-call constraint for stateful
agents, and `is_error` reporting appear ONLY in the Patterns observed
bullets — not as a dedicated tools-as-architectural-concern paragraph.

**Decision: APPLY this cycle.** Bounded mechanical edit (~10 lines).
Adds a "Tool integration model" paragraph between "Trust boundaries
explicitly named" and "Anti-patterns explicit in v0.4 migration guide."

The flag would otherwise carry forward as a six-cycles-deferred minor
edit, which is the wrong shape — the work is small, the value is
moderate, and applying makes the AutoGen section's structural treatment
of tools match the deep-dive's section-level treatment.

## Optional flag 6 (from cycle 17): AutoGen nav-bullet-4 behavior-contracts framing

Cycle 17's cold-reader 2 flagged: deep-dive bullet 8 ("Behavior contracts
as message protocols; patterns emerge from protocol implementation
rather than from a universal orchestrator object") is the core
Core-API framing. Partially expressed in nav-bullet-4 ("Multiple
orchestration patterns coexist as first-class") but the WHY is dropped.

**Decision: APPLY this cycle.** Bounded mechanical edit (~1 sentence).
Enriches nav-bullet-4 with the WHY framing.

Same rationale as flag 5: bounded, low-cost, value-positive. Applying
now prevents the queue of deferred flags from becoming a permanent
backlog of cycle-N-cold-reader-found-cycle-N+1-deferred minor edits.

## Tier-2 group 3 scoping (deferred to cycle 19+ but explicitly scoped)

Tier-2 group 3 — the "freeze-vs-refresh framing alternative" — has been
deferred for 5 cycles (cycles 13, 14, 15, 16, 17). Sixth defer is a
smell: the prompt's ABORT-CRITERIA names "inability to deepen" as a
trigger and silent-deferral resembles avoidance.

Not executing this cycle (capacity is consumed by the cold-readers,
flag-5/6 application, LangGraph dispatch, this notes file, journal,
README update). But scoping it explicitly here so cycle 19 can pick up
without re-deriving the scope.

### What Tier-2 group 3 is

From cycle 12's Copilot feedback PR #2756 evaluation, lens 3.B was
"motivated-reasoning on freeze-vs-refresh — both 'defenses run too
late' and 'artifacts freeze too early' framings are equally consistent
with the evidence, and the artifact picks the second without arguing
the choice." Lens 3.A asked whether the freeze-vs-refresh framing
itself is the right framing alternative.

The current `0-retrospective.md` text uses the "artifacts freeze too
early" framing in:
- The family preamble (in the family-overlap paragraph at lines 184-194)
  attributing freeze-vs-refresh to F11's local mechanism
- The F11 architectural-implication paragraph (lines 742-761) leading
  with the freeze-vs-refresh framing
- The cycle-7-derived "structurally produces post-close divergence"
  language (per cycle 12's 1.C correction, "structurally produces" not
  "guarantees")

The framing PICKS that artifacts freeze (as the architectural cause)
rather than that defenses run late (as the equally-valid alternative
cause). Cycle 7's freeze-vs-refresh adoption from Copilot feedback was
not adversarially examined for the "which side of the dichotomy" choice.

### Three options for cycle 19+ execution

(a) **Argue the choice.** Add a paragraph (probably in the family
preamble or F11 paragraph) explicitly arguing why "artifacts freeze
too early" is the load-bearing framing — what makes it preferable to
"defenses run too late." Risk: post-hoc rationalization for a choice
made without argument.

(b) **Adopt symmetric framing.** Reframe to "freeze-vs-refresh is a
bilateral mismatch with no architecturally-correct side; either earlier
defense or later freeze would resolve it; v1 chose to add defenses
without changing freeze timing, and the defenses run too late
*because* freeze runs too early relative to defense timing." This
acknowledges both framings and shows their equivalence at the
mechanism level. Most honest option.

(c) **Switch framing.** Adopt "defenses run too late" as the primary
framing if it leads to better v2 design implications. Speculative —
would only justify if (b)'s symmetric exploration shows that the
"defenses run too late" framing surfaces architectural implications
the current framing misses.

Recommended order for cycle 19+: try (b) first (lowest risk,
addresses the lens 3.B motivated-reasoning concern directly), then
evaluate whether (c) adds anything beyond (b). (a) is the least
defensible and probably wrong.

### What the Tier-2 group 3 cycle would look like

- Read all freeze-vs-refresh occurrences in `0-retrospective.md` (grep
  for "freeze", "refresh", "post-close")
- Read cycle-7 notes file for Copilot's exact PR #2749 lens 2 framing
- Read cycle-12 notes file for the lens 3.B specific finding wording
- Apply option (b): symmetric framing reframe
- Update at least three locations: family-overlap paragraph, F11
  architectural-implication paragraph, F12 hypothesis substrate
  paragraph (where it cites freeze-vs-refresh)
- Cold-reader on the rewrite (does the symmetric framing lose the
  cycle-7-derived structural claim?)

Estimated cycle-19 scope: 1 cycle of focused work, no parallel cold-
readers, deferred Phase 1 work that cycle.

## LangGraph Copilot research-only dispatch (cycle-18 pre-commit 1)

Per cycle 17's next-dispatch decision: LangGraph is the cycle-18
external-system dispatch target. AutoGen's state-management treatment
in the deep-dive was light (component-local dictionaries, no centralized
graph); LangGraph centers state-management as primary value
proposition. Reading LangGraph completes the state-management coverage
gap.

Procedure: canonical cycle-15 dispatch path with anti-smuggling
discipline pre-loaded in the dispatch body. Seven lenses calibrated to
LangGraph's value-proposition (state-management as primary lens; other
lenses TBD per LangGraph's structure).

Dispatch executed in this cycle's work — see issue number in the
session-end summary on issue #2766.

## Long-deferred items roll-call (carried forward)

1. Journal-entry self-congratulation sweep (12 cycles deferred from cycle 7+)
2. F6/F8/F9 measurements (cycle 7+)
3. Refactor-for-length F-section sweep (cycle 8+; Tier-2 group 8)
4. Persistence-mechanism deferred-list consolidation (cycle 13 flag)
5. Tier-2 group 2 (review/disposition substrate, partial integration cycle 13)
6. Tier-2 group 3 (freeze-vs-refresh framing alternative — scoped this
   cycle, execution in cycle 19+)
7. Tier-2 group 4 (nine measures rework)
8. Tier-2 group 6 (preserved-through-cutover disposition)
9. Tier-2 group 7 (resolved open questions collapse)
10. Tier-2 group 9 (F8 singleton-family acknowledgment)

Cycle 18 added 0 items, resolved 0; "scoped" Tier-2 group 3 (item 6)
is partial-progress: explicit cycle-19+ execution spec exists now where
it didn't before.

## Cycle 19+ pre-commits

1. **Tier-2 group 3 execution.** Apply the option (b) symmetric framing
   reframe per the scoping above. Cold-reader on the rewrite. Single
   focal task; defer parallel Phase 1 work that cycle if needed.
2. **LangGraph dispatch evaluation + integration.** When PR lands
   (typical 2-3 days), evaluate per cycle-7 / cycle-12 / cycle-15-style
   per-finding evaluation. Tier-1 integration into `1-research.md` if
   findings warrant; Tier-2 integrations as new pre-commits.
3. **Cold-reader 1 flag (c1.1).** Either trim Voyager bullets 15-16 of
   post-prose specs, or add the corresponding sentences to the prose.
   Bounded mechanical, ~3 lines.
4. **Cold-reader 1 flag (c1.2).** Consider elevating the
   compositionality-as-learning-mechanism observation to the Voyager
   Patterns observed list. Bounded mechanical, ~1 line addition.
5. **Cold-reader 2 flag (c2.1).** Failed-task-as-recorded-artifact
   transferability observation — incorporate into cross-system
   observations section if cycle-19 capacity permits and at least one
   more system is read at depth (LangGraph would be the fifth).
6. **Cold-reader 2 flag (c2.2).** Code-vs-prompts split transferability
   observation — same gating as c2.1.

## Persistence-mechanism observations

Cycle-N-pre-commits-cycle-N+1-checks chain is now thirteen cycles deep
(cycle 7 → 8 → 9 → 10 → 11 → 12 → 13 → 14 → 15 → 16 → 17 → 18 → 19
pre-committed). No breakdown.

**The deferred-flags-from-cold-readers pattern got tested this cycle
in the opposite direction: APPLYING two cycle-17-flagged optional
edits that were eligible for further deferral.** Cycle 18's reasoning:
the flags are bounded mechanical edits, the cycle has capacity, and
keeping them in the queue indefinitely is itself a smell (the queue
becomes a permanent backlog of cycle-N-found-cycle-N+1-deferred items
with no convergence). Apply when the bounded-mechanical work is small
enough that doing it costs less than maintaining the queue entry.

Pattern shape now: cold-reader produces verdict + flag + apply-or-defer
decision. Apply when bounded-mechanical AND cycle has capacity. Defer
when load-bearing structural rewrite OR cycle is over-loaded. The
threshold is roughly "5-10 lines and 5 minutes of judgment-call" —
above that, defer to a focused cycle.

**The sixth-defer-is-a-smell pattern emerged this cycle.** Tier-2 group
3 was at five-cycles-deferred coming in; cycle 18 declined to make it
six-cycles-deferred WITHOUT scoping. Instead, this cycle invested
~25% of its capacity in explicitly scoping the cycle-19+ execution.
The scoping is itself partial progress: cycle 19 doesn't have to
re-derive the rewrite scope, just apply it.

This is a pattern worth naming explicitly: when an item has been
deferred N cycles where N is large enough to look like avoidance,
the next cycle's choices are (a) execute, (b) defer with explicit
scoping that brings the next cycle closer to execution, or (c) demote
the item if it's not actually load-bearing. Silent-defer is a fourth
option and a failure mode. (a) > (b) > (c) > silent-defer.

**The cycle-17 anti-smuggling-discipline-pre-loading pattern got its
second re-application this cycle in the LangGraph dispatch.** Cycle 16
pioneered, cycle 17 re-applied (Voyager via orchestrator-direct read),
cycle 18 re-applies (LangGraph via Copilot dispatch). Now the default
for both dispatch types and orchestrator-direct reads. Pattern is
stable.

## Reconciling cycle scopes

Cycle 14: 3 cold-readers + Phase 1 initiation (`1-research.md` draft +
2 system reads).

Cycle 15: 3 cold-readers + 1 adversarial re-read with edit + 1 dispatch
+ 1 Tier-2 group execution.

Cycle 16: 3 cold-readers + AutoGen dispatch evaluation + Tier-1
AutoGen integration.

Cycle 17: 2 cold-readers + Voyager orchestrator-direct read + ~190 line
integration into `1-research.md`.

Cycle 18: 2 cold-readers (with one substantive edit applied) + 2
optional-flag executions (~12 lines added across AutoGen section) +
Tier-2 group 3 explicit scoping + LangGraph dispatch + cycle notes +
journal + README update.

Cycle 18 is moderately loaded. The Tier-2 group 3 deferral with
scoping is the main capacity-management decision; without it, this
cycle would either silent-defer (failure mode) or over-extend.
