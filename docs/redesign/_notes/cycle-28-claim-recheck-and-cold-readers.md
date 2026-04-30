# Cycle 28: NEW-pattern multi-system claim re-check + diversity-hedge tightening + cold-readers on cycles 26 and 27

Cycle 27 (commit `10441281`) per-finding evaluated the three cycle-26 dispatch
deliverables, built a SUPPORT/CONTRADICT cross-validation matrix, applied 6
Tier-1 mechanical edits to `1-research.md` (5→8 systems, 4 patterns count-
updated, 4 elevated 2→3+, 2 elevated with diversity hedge, 1 elevated with
contrary-stance note), and surfaced 8 NEW pattern candidates pending cycle-29+
integration. Cycle 27's same-cycle cold-reader on the matrix flagged three
issues for cycle 28: Append-only and Memory hedge prominence (item 2);
NEW-pattern Voyager-as-mechanical-enforcement overcount (item 3); restructure
of the 3+/2-system binary now that the 2-system tier is empty (item 1).

Cycle 28 took the bounded-mechanical branch of the cycle-27 plan: item 3
recheck (deeper than cycle-27's spot-check, walking each NEW candidate's
claimed system list against the source deliverable); item 2 Memory bullet
count fix + style tightening; cold-readers on cycle-27 and cycle-26 notes
(items 10 and 11). Item 1 (Tier-2 restructure) deferred to cycle 29+ for
sequencing reasons documented below. Cycle-29+ pre-commits at the bottom.

## Item 3: NEW-pattern multi-system claim re-check

The cycle-27 matrix listed 8 NEW pattern candidates with claimed system
support. Cycle-27 cold-reader spot-checked one (Voyager-as-mechanical-
enforcement) and found overcount; this cycle walks the rest at the same
strictness.

### Method

For each NEW candidate, walk its claimed support against:
- The system's pattern list in the cycle-26 deliverable (where applicable)
- The system's section in `1-research.md` (where applicable for pre-cycle-26
  systems)
- The framing-strictness gradient: explicit named pattern (S+) / inferable
  from text but not central (W+) / present in adjacent shape (partial) /
  absent (N).

The strict framing is "explicit named pattern matching the candidate's
shape"; the loose framing is "any system with a related primitive at any
level of abstraction."

### Candidate-by-candidate verdict

**1. Mechanical enforcement of constraints (linters, CI checks, regression
tests on agent-affecting prose).** Cycle-27 matrix claimed 4 systems with
cross-substrate convergence: OpenAI Harness, oh-my-codex, Voyager, LangGraph.

- OpenAI Harness — patterns 8 (mechanical enforcement over documented rules),
  9 (custom linters with agent-readable error messages), 12 ("Golden
  principles" pattern, mechanically checked). All three are explicit
  behavioral mechanical enforcement. **STRONG support, strict framing.**
- oh-my-codex — pattern 7 (behavioral prompt contract with regression tests
  in `src/hooks/__tests__/prompt-guidance-*.test.ts`). Explicit behavioral
  mechanical enforcement. **STRONG support, strict framing.**
- Voyager — claimed via CriticAgent. The CriticAgent is an LLM critic
  (`{success: bool, critique: str}`), not a mechanical CI check. Voyager
  DOES have mechanical enforcement at boot: the SkillManager and
  CurriculumAgent assert vectordb count vs JSON manifest count at init,
  fail-fast on divergence (`1-research.md` Voyager pattern: "Sync invariants
  asserted at init for dual-storage components"). But this is init-time
  assertion, not continuous enforcement. **WEAK support, loose framing
  (init-time only, not behavioral).**
- LangGraph — claimed via typed schema validation. Channel types and reducer
  contracts are statically typed via TypedDict / dataclass / Pydantic BaseModel.
  This IS mechanical enforcement of structural constraints, but it enforces
  data shape, not behavior. **WEAK support, loose framing (data-shape, not
  behavior).**

**Correction: 2-system strict (OpenAI Harness, oh-my-codex) + 2-system
loose (Voyager init-time, LangGraph type-system).** The cycle-27 cold-reader
already caught the Voyager overcount; this re-check confirms LangGraph is
also a loose-framing match. Cycle-29+ pattern addition should name the strict
framing in the bullet ("regression-tested behavioral constraint enforcement")
and note the loose extension as separate clause ("init-time invariants in
Voyager and type-system shape-enforcement in LangGraph share the principle
but at different scope and rigidity").

**2. Plans/specs as first-class versioned artifacts.** Cycle-27 matrix
claimed 3 systems with structural variation: OpenAI Harness, oh-my-codex,
Voyager.

- OpenAI Harness — pattern 7 (plans as first-class versioned artifacts;
  active, completed, technical-debt plans checked into repo). **STRONG
  support, strict framing (plan-as-forward-spec).**
- oh-my-codex — pattern 2 (context snapshot grounding before execution;
  written to `.omx/context/{task-slug}-{timestamp}.md` with task statement,
  desired outcome, known facts, constraints, unknowns, codebase touchpoints).
  Forward-spec written before execution. **STRONG support, strict framing.**
- Voyager — claimed via "curriculum log." Voyager's curriculum log is
  `completed_tasks.json` and `failed_tasks.json` — task-history records,
  read by CurriculumAgent for next-task selection. This is backward-looking
  task-record, not forward-looking plan-as-spec. The shape is closer to
  "Failed work as recorded artifact" (already a separate elevated pattern)
  than to "plans-as-artifacts." **NOT support at strict framing; partial at
  loose framing only.**

**Correction: 2-system clean (OpenAI Harness, oh-my-codex) at strict
framing.** Voyager's curriculum log is a different shape and is already
covered by the elevated "Failed work as recorded artifact" pattern. Including
it under "plans-as-artifacts" double-counts and conflates forward-looking
plan-spec with backward-looking task-history.

**3. Entropy / AI slop as first-class engineering concern.** Cycle-27 matrix
claimed 2 systems: OpenAI Harness (patterns 11+12), oh-my-codex (pattern 13).

- OpenAI Harness — pattern 11 (entropy as first-class engineering concern),
  pattern 12 (golden principles + doc-gardening agent). Two patterns, same
  system. **STRONG support.**
- oh-my-codex — pattern 13 (deslop pass as mandatory post-completion step).
  **STRONG support.**

**Correction: none. 2-system count is correct.**

**4. Context anxiety / model self-model failures.** Cycle-27 matrix claimed
1 system (Cognition only).

- Cognition Devin — patterns 4, 5, 6, 15 (context anxiety, environmental
  deception, prompt placement, misestimation precision). **STRONG support.**

**Correction: none. 1-system is correct (single-system observations file).**

**5. Pre-execution gating against underspecified requests.** Cycle-27
matrix claimed 1-2 systems: oh-my-codex strong; Cognition partial.

- oh-my-codex — pattern 5 (word-count + signal-detection gate; force:
  bypass). **STRONG support.**
- Cognition Devin — adjacent (context anxiety mitigation prevents premature
  task closure mid-execution). Mid-execution, not pre-execution.

**Correction: none. 1-system strict (oh-my-codex), Cognition adjacent at
different lifecycle phase.**

**6. Agent legibility / repo structured for agent comprehension first.**
Cycle-27 matrix claimed 1-3 systems depending on framing strictness.

- OpenAI Harness — pattern 15 (agent legibility as optimization target).
  **STRONG support, strict framing.**
- Cognition Devin — implicit (context engineering thesis; context as the
  unit of reliability; agent trace as architecture). **WEAK support, loose
  framing.**
- oh-my-codex — implicit (AGENTS.md template, CONTRIBUTING.md `<Bad>`
  examples, role-prompt structure). **WEAK support, loose framing.**

**Correction: none. The "1 explicit + 2 implicit" framing is honest and
preserved.**

**7. Throughput-based merge philosophy.** Cycle-27 matrix claimed 1 system
with conditional scope.

- OpenAI Harness — pattern 16 (scoped to high-throughput regime; explicitly
  conditional). **STRONG support, conditional.**

**Correction: none. 1-system conditional is correct.**

**8. Iteration limits with explicit ceiling / bounded autonomy loops.**
Cycle-27 matrix claimed 1-2 systems: oh-my-codex strong; Cognition partial.

- oh-my-codex — pattern 6 (max_iterations=10; review loop max=5; autoresearch
  keep/discard/stop). **STRONG support, strict framing.**
- Cognition Devin — partial (45-min session time limit; not a documented
  architectural iteration limit). Time-limit, not iteration-count.
- Voyager — `action_agent_task_max_retries = 4`. From `1-research.md`
  Voyager section: "Bounded retries on action failure with critic-critique
  + execution-error fed into next prompt." This IS an explicit iteration
  ceiling. **STRONG support, strict framing — MISSED in cycle-27 matrix.**

**Correction: 2-system strict (oh-my-codex, Voyager) + 1 partial (Cognition
time-limit).** Cycle-27 matrix UNDERCOUNTED by missing Voyager. Cycle-29+
pattern addition should include Voyager.

**9. Autonomy directive prominently stated (don't ask permission).**
Cycle-27 matrix claimed 1 explicit + 2 implicit.

- oh-my-codex — pattern 17 (templates/AGENTS.md opens with all-caps
  autonomy block). **STRONG support, strict framing.**
- OpenAI Harness — pattern 2 ("Humans steer. Agents execute." as role-
  allocation thesis). Explicit but framed as role-allocation rather than
  autonomy-directive specifically. **MEDIUM support, adjacent framing.**
- Cognition Devin — "fully autonomous AI software engineer" framing as
  marketing tagline; not an autonomy directive in agent-instruction sense.
  **WEAK support, loose framing.**

**Correction: none. The "1 explicit + 2 implicit/adjacent" framing is
honest and preserved.**

### Summary of corrections

Three of nine NEW pattern candidates needed correction:
- #1 (Mechanical enforcement): OVERCOUNT corrected — 2 strict + 2 loose
- #2 (Plans-as-artifacts): OVERCOUNT corrected — 2 clean (drop Voyager
  curriculum log)
- #8 (Iteration limits): UNDERCOUNT corrected — add Voyager's
  `action_agent_task_max_retries`

The 33% correction rate (3/9) at deeper inspection vs cycle-27's spot-check
that caught 1/3 (Voyager-as-mechanical-enforcement only) suggests the
cycle-27 cross-validation matrix should be treated as first-pass evidence,
with cycle-29+ pattern additions using the corrected counts from this
cycle's recheck rather than the cycle-27 matrix counts.

The corrections are NOT applied to `1-research.md` this cycle because the
NEW patterns haven't been added to the artifact yet — they're cycle-29+
work (item 4 from cycle-27 pre-commits). This recheck produces the
empirical input cycle 29+ will use.

## Item 2: Diversity-hedge prominence assessment + Memory bullet count fix

Cycle-27 cold-reader flagged that "this might still be too gentle; the
right call might be a separate subsection for 'convergence-on-principle,
divergence-on-mechanism patterns' rather than mixing them in with the
cleaner elevations." This cycle re-read both elevated bullets (Append-only
history; Memory as first-class architectural concept) to assess.

### Append-only history (lines 1013-1033 pre-edit)

The diversity hedge is in the bullet's opening parenthetical AND in body
text (a sentence beginning "**Diversity hedge:**"). The hedge text:
"the convergence is on the principle (no destructive history overwrite);
the substrate diverges across in-process versioning (LangGraph branching,
Voyager V2/V3) vs filesystem/git (OpenAI repo-as-state) vs one-way file
migration (oh-my-codex)."

**Verdict: hedge is prominent, not buried.** The cycle-27 cold-reader
concern was misplaced for this bullet — the hedge appears twice (opener
parenthetical + body sentence) and identifies the substrate-divergence
shape explicitly.

### Memory as first-class architectural concept (lines 1034-1062 pre-edit)

The diversity hedge is in the opener parenthetical AND in body text
(closing sentences listing the divergence primitives). However, an
inconsistency surfaced on close reading:

- Body description names 5 systems: PAI, LangGraph, Cognition, OpenAI,
  oh-my-codex.
- Divergence list at end mentions 6 primitives: vector store (Voyager),
  typed channel (LangGraph Store), context trace (Cognition),
  repository-as-record (OpenAI), wiki server (oh-my-codex), principle-shape
  (PAI).

The 6th system (Voyager) appears in the divergence list but not in the
body description. This is a count-vs-description inconsistency.

**Resolution.** Voyager's SkillManager + Chroma vectordb IS a vector-store
primitive that COULD be framed as memory, but in `1-research.md`'s Voyager
section it is framed as skill-storage (`SkillManager — persistent skill
library`), not memory-as-such. The framing isn't "memory" in Voyager's
own architectural vocabulary. Including Voyager in the Memory divergence
list double-counts: Voyager's skill library is already covered by
"Component-local state persistence" (where it appears as `ckpt/skill/`
file-per-component).

**Edit applied this cycle:** drop "vector store (Voyager)" from the Memory
bullet's divergence list. Add a parenthetical noting that Voyager's
SkillManager is adjacent but framed as skill-storage in the source repo,
not counted in the convergence to keep body and count consistent. The
parenthetical reserves cycle-29+ re-evaluation if framing shifts.

### Style tightening on diversity-hedge openers

Both elevated bullets had "DIVERSITY HEDGE" in ALL-CAPS in the opener
parenthetical. This was ad-hoc and loud relative to other elevated bullets'
parentheticals. **Edit applied this cycle:** replace ALL-CAPS markers with
prose-style hedge naming (e.g., "the convergence is on the principle,
the divergence is on the substrate — see body's 'Diversity hedge'
sentence"). The substantive hedge is preserved in the body text where
it carries the load.

### Verdict on cycle-27 cold-reader's concern

The "hedges might be buried" concern was incorrect for both bullets — the
hedges were prominent in opener AND body. The actual issues found on close
reading were different:
- A count-vs-description inconsistency in the Memory bullet (5 in body, 6
  in divergence list)
- ALL-CAPS markers being awkward style (ad-hoc, loud)

Both issues fixed this cycle. The "separate subsection for divergence-
shape patterns" idea remains a Phase 2-3 candidate but is not warranted at
current 12-pattern scale; the parenthetical hedge in the bullet itself
plus the body's closing-sentence framing is sufficient for the current
section size.

## Item 1: Tier-2 cross-system observations restructure — DEFERRED to cycle 29+

Cycle 27's pre-commit 1 named restructuring the Cross-system observations
section as the dominant Tier-2 work product. Current state: 12 patterns at
3+ tier (5 original + 7 elevated cycle 27); 0 patterns at 2-system tier
(emptied by cycle-27 elevations); placeholder explaining the transition.

### Why deferring is the right call this cycle

1. **2-system tier will re-populate cycle 29+.** Cycle 29+ item 4 adds
   5-7 NEW pattern candidates (after correction: ~6 strict + 2-3 partial).
   Most NEW candidates will be 2-system tier (e.g., entropy/AI-slop is
   2-system; mechanical enforcement strict is 2-system; plans-as-artifacts
   strict is 2-system; iteration-limits with Voyager is 2-system; pre-
   execution gating is 1-system going to single-system file). Restructure
   now means restructuring AGAIN cycle 29+ when the 2-system tier is no
   longer empty.

2. **Final shape isn't visible yet.** The right organizing structure
   (family-clustering vs maturity-clustering vs flat-with-ordering) depends
   on the final pattern population. With the 2-system tier currently empty,
   the section's shape is unstable. Restructure decisions are best made
   when the population is closer to its final shape.

3. **Current placeholder is functional.** The 2-system subsection has a
   placeholder explaining the transition state ("Cycle 27: this subsection's
   prior seven entries elevated to 3+ tier on cycle-26-deliverable cross-
   validation. The cycle-28 Tier-2 restructure may reorganize the section
   by family or maturity rather than the binary 2-vs-3+ tier scheme; this
   subsection is currently empty pending re-population by NEW 2-system
   patterns from cycle-28+ work"). A reader encountering the empty section
   sees the explanation; the artifact isn't broken by the deferral.

4. **Cycle-28 budget is consumed by item 2 (hedge fix), item 3 (claim
   recheck), and items 10/11 (cold-readers).** Adding the structural
   restructure on top would compress all four items into surface-level
   work. Better to do the bounded items well and defer the substantive
   restructure to cycle 29+ when it can be the focal.

### What cycle 29+ should do for the restructure

After the NEW pattern integration (item 4) lands, evaluate:
- Total pattern count at 3+ tier (likely 12-13)
- Total pattern count at 2-system tier (likely 4-7 from NEW additions)
- Whether family-clusters are stable across the full population
- Whether maturity-clusters (load-bearing-strong vs supporting-evidence)
  better organize reader navigation

Decision deferred until the post-integration shape is visible.

## Item 10: Same-cycle cold-reader on cycle-27 notes file

The cycle-27 plan named three specific questions for the cycle-28
cold-reader:

(a) Does the cross-validation matrix's SUPPORT/CONTRADICT scoring
    methodology read as principled or as ad-hoc pattern-matching?
(b) Does the cycle-28+ pre-commit list (12 items) start to threaten
    breakdown of the cycle-N-pre-commits-cycle-N+1-checks chain through
    sheer item count?
(c) Is the deferral pattern (apply 6 mechanical edits, defer 5 substantive
    ones) sustainable across the next 3-5 cycles, or does it accumulate
    Tier-2 backlog?

### Question (a): scoring methodology — PRINCIPLED IN CONCEPT, GRADIENT-DEFINITION DRIFT-RISK

The S+/W+/N/W-/S- gradient is consistent in shape (strength of pattern
match with directional sign). Cycle-27's cold-reader spot-checked 3 high-
stakes scores (PASS), 2 elevation-arithmetic claims (QUALIFIED PASS), and
1 NEW pattern claim (Voyager-as-mechanical-enforcement, FAIL).

This cycle's deeper recheck found 3/9 NEW pattern claim corrections (33%
rate), suggesting the gradient is principled in concept but the gradient
DEFINITION is not formalized. What distinguishes S+ from W+? In cycle 27's
practice it was "explicit named pattern with multiple supporting examples"
vs "inferable from text but not central." But this distinction wasn't
written down; it was applied implicitly per-row.

**Risk:** in future matrices, the implicit gradient definition could drift
across cycles. Different orchestrator sessions (or different
attention-states within one session) could apply S+ at different
strictness levels.

**Mitigation candidate (deferred to cycle 29+):** if the matrix shape is
used again in cycle-29+ NEW pattern integration, codify the gradient
definition explicitly. E.g., "S+ = explicit named pattern present in
deliverable's pattern list with cross-deliverable verification of the
shape; W+ = related primitive present but framed differently; N = absent
or only adjacent at a different lifecycle phase."

**Verdict (a):** principled in concept; gradient-definition is implicit
and at drift-risk; codify if matrix is re-used.

### Question (b): pre-commit list growth — ABSORBED AT SUSTAINABLE RATE

Cycle-26 list: 10 items. Cycle-27 list: 12 items. Cycle-28 absorbs items 2,
3, 10, 11 (4 items) and explicitly defers item 1 with rationale. Items 4-9,
12 carry forward. After cycle 28: ~8 items remain (items 4-9 + 12 + new
cycle-28-derived items minus cycle-27 absorptions).

**Pattern observed:** each cycle absorbs 4-6 items, defers 4-6 items,
list stays in the 8-12 item range. Not catastrophic growth.

**Risk:** if cycle 29 doesn't make substantive inroads on item 4 (NEW
pattern integration with corrections from this cycle), the list will
grow because cycle 29's own pre-commits add to the carry-forward.

**Mitigation:** cycle 29's focal SHOULD be item 4 (NEW pattern integration
with corrected counts). This is the largest deferred substantive item and
the longest-deferred work product. Doing it cycle 29 keeps the list growth
within absorption rate.

**Verdict (b):** chain-break risk is low for next 3-5 cycles given current
absorption rate; cycle 29 making item 4 the focal is the load-bearing
mitigation.

### Question (c): deferral pattern sustainability — SUSTAINABLE WHEN HIGH-VALUE ITEMS ARE ABSORBED

Pattern across cycles 25-28:
- Cycle 25: applied substantive elevations + section-opening + retrospective
  interleave; deferred ~4 items.
- Cycle 26: applied 3 dispatches + cold-reader; deferred ~6 items
  (dispatch-volume outlier).
- Cycle 27: applied 6 mechanical edits + cold-reader; deferred 5 substantive
  items.
- Cycle 28 (this): applies 1 substantive edit (Memory count fix) + 2 bounded
  mechanical (hedge tightening, NEW pattern recheck) + 2 cold-readers;
  defers item 1 (Tier-2 restructure) with rationale.

The deferral pattern is "absorb 4-6, defer 4-6." Sustainability depends on
whether absorbed items are highest-value. Cycle 28's absorbed items
(Memory count fix, NEW pattern recheck producing corrections, hedge
tightening, cold-readers) are all high-value: count fix is artifact-quality;
recheck produces empirical input for cycle-29 focal; hedge tightening
tightens already-prominent prose; cold-readers reduce risk.

**Risk:** cycle 29-31 must absorb the substantive deferred items
(Persistent Divergences updates, F-family vs A-pattern cross-validation,
remaining audit retrospective sections, NEW pattern integration). If any
of these items keep getting deferred beyond cycle 31, accumulated value-
loss starts to outweigh the absorption rate.

**Verdict (c):** sustainable through cycles 28-31; watch-item is whether
cycle 29's focal absorbs the largest deferred substantive item (NEW
pattern integration). If cycle 29's focal slips to a smaller item, the
list growth pattern becomes a backlog accumulation pattern.

## Item 11: Cold-reader on cycle-26 notes file (carry-forward)

The cycle-26 notes plan listed three specific questions for the cycle-27
cold-reader (item 8 of cycle-26 pre-commits), which was deferred from
cycle 27 to cycle 28:

(a) Does the three-dispatches rationale read as honest pivot or as
    over-justified scope expansion?
(b) Does the fresh-eye cold-reader's NEUTRAL verdict on citation-chain
    transparency match what a reader unfamiliar with the cross-system
    observations section convention would conclude?
(c) Does the "30-second self-check" framing accurately apply to substantive
    cycle-N record / recap notes (this file), or is the line between
    "bounded mechanical" and "architecturally-load-bearing" blurrier than
    the cycle-25 codified rule allows?

### Question (a): three-dispatches rationale — HONEST PIVOT, prediction modestly conservative

Cycle-26 named the trigger (WebFetch/curl gated), the trade-off (lost
tightness; integration latency), the cost vs benefit (~$5-10 across 3
sessions for ~2 cycles speedup), and the risk mitigation (independent
dispatches; standard anti-smuggling pre-loaded). Reads as honest pivot.

**Empirical check from cycle 27 outcome.** Cycle 26's prediction was "2-3
day Copilot turnaround" and "cycle 28-31 will be primarily integration
cycles." Actual: all three dispatches landed within ~hours of dispatch
(hours, not days); cycle 27 absorbed all three deliverables in a single
cycle via batched cross-validation. The actual benefit was greater than
predicted — turnaround was faster, integration was tighter.

**Verdict (a):** honest pivot with predictions that turned out modestly
conservative. PASS. (Conservative predictions are a healthy pattern;
overconfident predictions would be the failure mode.)

### Question (b): NEUTRAL verdict on citation-chain transparency — CORRECT

The cycle-26 fresh-eye cold-reader's NEUTRAL verdict said the citation
chain extends but is consistent with the section's overall convention,
deferring to a section-level convention question rather than asymmetric
per-pattern transparency.

**Reader-unfamiliar-with-convention check.** A reader external to the
project would see a 12-bullet list with claims citing system-specific
evidence (file paths, constants, doc quotes). Some claims cite tightly
("PAI Principle 13 names 'Memory System...'"), others cite loosely
("AutoGen state save/load is component-local dictionaries"). To verify a
loose claim, the reader traverses bullet → per-system section in
1-research.md → substrate notes file → primary source.

The cold-reader's NEUTRAL verdict implicitly acknowledges the convention
is opaque to outside readers but says applying asymmetric transparency to
one pattern would compound rather than fix the opacity. The verdict's
language ("either (i) leave the convention as-is across all patterns;
(ii) define a section-level citation-chain convention applied to all 7
patterns") is exactly the right framing — the question is section-level,
not pattern-level.

**Verdict (b):** NEUTRAL is well-supported. PASS. The convention-
transparency question remains a Phase 2-3 candidate (low priority unless
audit or Copilot feedback flags it).

### Question (c): "30-second self-check" applicability to substantive
cycle-N notes — BORDERLINE, refinement candidate

The cycle-25 codified discipline-lightening rule: "bounded mechanical →
30-second self-check; substantive prose / architecturally-load-bearing →
full structured pass." Cycle 26 self-applied 30-second self-check to its
own notes file with the rationale that cycle-N notes are "substantive
prose, but it's a cycle-N record / recap, not a structural change to
redesign artifacts."

This is borderline. The notes file IS substantive prose with original
analysis. But the analysis is RECORDING the cycle's work, not modifying
load-bearing artifacts. The cycle-25 rule has two categories; cycle-N
notes fit a third hybrid category (substantive content, but documentary
not artifact-modifying).

**Refinement candidate (deferred to cycle 29+):** clarify the rule with a
third category. E.g., "substantive prose modifying load-bearing artifacts
→ full structured pass; bounded mechanical edits → 30-second self-check;
substantive prose in cycle-N notes (recording what happened, not modifying
load-bearing artifacts) → 30-second self-check applied per-claim with
named-citation requirements."

The third category preserves the discipline-lightening intent while
acknowledging that cycle-N notes deserve more rigor than 30 seconds (they're
the authoritative record of what each cycle did) but less than full
structured pass (they're not changing the artifact's claims).

**Verdict (c):** cycle-26's application is defensible but the rule is
borderline; refinement is a Phase 2-3 candidate. Watch: if a future
cycle-N notes file produces an error that 30-second self-check would have
caught at full structured pass, that's the trigger for codifying the
refinement.

## Cycle 29+ pre-commits

Carry-forward + cycle-28-derived:

1. **NEW pattern integration with corrected counts.** Cycle 29 focal.
   Add the NEW pattern candidates (mechanical enforcement, plans-as-
   artifacts, entropy/AI-slop, iteration limits, pre-execution gating,
   agent legibility, autonomy directive, throughput philosophy, context
   anxiety) to `1-research.md` at the appropriate tier, using the
   corrected counts from this cycle's recheck. Architecturally-load-bearing
   prose work; full structured pass.

2. **Tier-2 cross-system observations restructure** (deferred from cycle 28).
   Reassess section structure after item 1 above lands. Family-clustering
   vs maturity-clustering vs flat-with-ordering. Architecturally-load-bearing.

3. **Update Persistent Divergences section** (carry-forward from cycle 27
   pre-commit 5). Cognition Devin's anti-stance on role-separation; the
   "throughput regime as moderating variable" observation from OpenAI
   Harness's wide-trust-boundary contradiction. Substantive prose work.

4. **Cross-validate against audit's A-pattern mapping** (carry-forward from
   cycle 27 pre-commit 6 / cycle 25 pre-commit 7). Bounded mechanical
   (~one cold-reader cycle).

5. **Read remaining audit retrospective sections** (carry-forward from
   cycle 27 pre-commit 7 / cycle 25 pre-commit 8). "What v2 must
   demonstrably do better" section is the most relevant for Phase 2.

6. **Copilot research-only dispatch: oh-my-claudecode** (Eva directive
   #2774). Deferred from cycle 26 pre-commit 4 → cycle 27 pre-commit 8 →
   cycle 28 (held per cycle-27 item 12). Earliest dispatch: cycle 29-30
   if integration backlog from item 1 above clears; otherwise cycle 31+.

7. **Copilot research-only dispatch: openai/symphony** (Eva directive
   #2775). Same gating as item 6.

8. **Codify the SUPPORT/CONTRADICT gradient definition** (new from cycle
   28 cold-reader on cycle-27 matrix methodology). Bounded mechanical;
   one short paragraph in `1-research.md`'s methodology preamble or in
   the Cross-system observations section opening. Skip if matrix shape
   is not re-used in cycle 29+; apply if cycle 29 reuses the matrix
   for NEW pattern integration framing.

9. **Codify the third-category refinement to discipline-lightening rule**
   (new from cycle-28 cold-reader on cycle-26 notes question (c)). Bounded
   mechanical; one short paragraph either in `0-retrospective.md`'s
   iteration plan section or in `_notes/cycle-25-...-discipline-
   lightening-codified.md`. Defer until a cycle-N notes error occurs that
   would warrant the refinement; speculative codification without the
   trigger is over-process.

10. **Long-deferred items roll-call** (carry-forward, 9 items unchanged
    cycles 26-28).

11. **Same-cycle cold-reader on this notes file.** Standard cycle-N+1
    fresh-eye pass. Specific questions:
    (a) Does the 33%-correction-rate finding on cycle-27 matrix get the
        weight it deserves, or does it get buried in the broader cold-
        reader narrative?
    (b) Does the deferral rationale for item 1 (Tier-2 restructure) read
        as principled sequencing or as procrastination?
    (c) Does the third-category refinement candidate for the discipline-
        lightening rule add genuine value, or is it speculative process-
        layering?

### Suggested cycle 29 plan (provisional)

- **Focal:** item 1 (NEW pattern integration with corrected counts).
  Architecturally-load-bearing prose work; cycle-29 absorbs the largest
  deferred substantive item.
- **Bounded mechanical:** item 11 (this cycle's notes file cold-reader).
- **Defer:** items 2 (Tier-2 restructure — gated on item 1 landing), 3-9
  to cycle 30+ depending on focal completion progress.
- **No new dispatches** unless cycle 29's focal completes with budget
  remaining.

## Persistence-mechanism observations

The cycle-N-pre-commits-cycle-N+1-checks chain extends to twenty-three
cycles deep (cycle 7 → ... → 27 → 28 → 29 pre-committed). 11 items in
cycle-29 pre-commit list, smaller than cycle-28's 12. List shrinkage is
genuine for the first time in 4 cycles — the cycle-28 absorptions
outpaced new pre-commits.

**The auto-memory non-persistence finding.** Cycle-27's journal (entry
"Memory mechanism initialized") said "First session writing to
`/home/runner/.claude/projects/-home-runner-work-schema-org-json-ld-
schema-org-json-ld/memory/`. I wrote 7 memory entries: user profile (Eva),
project state (redesign cycle ~27), persistence mechanism, audit repo,
dispatch pattern, discipline-lightening rule, environmental constraint."
Cycle 28 cold-start: directory empty / `MEMORY.md` does not exist. The
auto-memory directory is non-persistent across orchestrator-run sessions
in this CI environment. The repo-tracked persistence mechanisms
(`docs/redesign/_notes/`, `README.md`'s iteration log, `docs/journal/`)
ARE what survives. **Conclusion: do not invest in writing auto-memory
entries cycle-by-cycle; the repo-tracked mechanism is the load-bearing
persistence and matches what cycle-N+1 actually receives at cold-start.**

This is a real environmental constraint, parallel to the cycle-26
WebFetch/curl gate finding. Both constraints shape the persistence
mechanism design: external URLs require Copilot-dispatch pivots; auto-
memory requires repo-tracked equivalents. The redesign prompt's
PERSISTENCE section already directs the orchestrator to use repo-tracked
mechanisms; the auto-memory finding confirms this is the right direction.

**The 33%-correction-rate finding on cycle-27 matrix.** Three of nine
NEW pattern claims needed correction at deeper inspection vs the cycle-27
spot-check that found 1/3. The error rate at deep inspection is roughly
3× the spot-check rate. This is consistent with the discipline-lightening
rule's intent: bounded mechanical work gets light review; substantive
work gets full review. The matrix construction was substantive but the
matrix's per-row scoring was treated as semi-mechanical. The matrix-as-
substantive but per-row-as-bounded split caused the error rate
imbalance.

**Cycle-29 absorption-rate watch.** Cycle 28 absorbed 4 items (items 2, 3,
10, 11) and deferred item 1 with explicit rationale. Cycle 29's focal
(NEW pattern integration with corrections) is the largest deferred
substantive item. If cycle 29 absorbs item 1 (the focal) plus item 11
(cold-reader on this notes file) — that's 2 items absorbed plus carry-
forward management. List growth depends on how much new pre-commit
material item 1's substantive work generates.

The honest-hedge pattern (named cycle 24, 6/6 stable cycles 24-27)
applied this cycle: the hedge in the Memory bullet's parenthetical-about-
Voyager fix is genuine to the data ("adjacent primitive but framed as
skill-storage rather than memory-as-such"); the corrections to NEW pattern
counts are honest about what the matrix overcounted vs undercounted; the
deferral rationale for item 1 names what's being given up (delayed
restructure) vs what's gained (waiting for stable population). Tally
extended: 7/7.

The discipline-lightening rule applied: item 2 (Memory count fix) treated
as substantive prose (full structured pass — close-reading the bullet
in context, identifying the specific count-vs-description inconsistency,
proposing edit, applying edit, verifying); item 3 (NEW pattern recheck)
treated as substantive prose for the empirical methodology (full
structured pass per-pattern); items 10 and 11 (cold-readers) treated as
substantive (full structured pass per-question). No items treated as
bounded mechanical this cycle. Tally extended: substantive 7/7;
architecturally-load-bearing 1/1; bounded mechanical 0/6 (cycle-27 had 6
mechanical edits absorbed at 30-second self-check; cycle 28 had no
mechanical edits to absorb because the recheck and hedge tightening were
treated as substantive on completion of the deeper read).
