# Cycle 31: Per-finding evaluation of Copilot feedback (PR #2791)

Cycle 30 (commit `51d6ab0f`) drafted the Tier-2 cross-system observations
restructure proposal at
`_notes/cycle-30-tier2-restructure-draft.md` and dispatched a Copilot
feedback-only request as issue [#2790](https://github.com/EvaLok/schema-org-json-ld/issues/2790).
The dispatch landed as PR
[#2791](https://github.com/EvaLok/schema-org-json-ld/pull/2791) with a
153-line critique structured by the seven lenses framed in the dispatch
body. This cycle (31) does the per-finding evaluation following the
cycle-7 / cycle-12 pattern: VERDICT (ACCEPT / QUALIFY / DISMISS) per
finding with rationale, then distilled structural decisions for the
cycle-32+ in-place restructure.

The light-prep → heavy substantive sequence (cycle 28 → 29; cycle 30 →
31; cycle 31 → 32+) is the third application of this pattern in the
phase. Cycle-7 / cycle-12 split per-finding-evaluation from
in-place-application across two cycles; this is the same shape.

**Disposition for this cycle.** Per-finding evaluation as architecturally-
load-bearing focal; cold-reader on cycle-30 notes as bounded mechanical;
in-place restructure deferred to cycle 32+. The cycle-30 cold-reader's
verdict on light-prep was "principled but fragile — becomes procrastination
if cycle 31 treats this critique as another reason to delay rather than
as input to either land a revised restructure or explicitly choose a
different organizing principle." This file's verdicts are forced to be
CLEAR DECISIONS (no "needs more thought" deferrals) so cycle 32+ has
direct application input.

## Process observation: dispatch-then-commit was the wrong sequence

A real failure surfaced in the Copilot critique's scope note: "I could not
find `docs/redesign/_notes/cycle-30-tier2-restructure-draft.md` in this
checkout or on `master` at the time of review." Verified the timeline:

- Issue #2790 created at 2026-04-30 07:05:48Z (dispatch fired)
- PR #2791 first commit ("Initial plan"): 07:06:33Z (45s after dispatch)
- PR #2791 critique commit: 07:11:23Z (5 min 35s after dispatch)
- Cycle-30 master commit `51d6ab0f`: 07:11:07Z (16 SECONDS BEFORE Copilot's
  critique commit, 4 min 49s AFTER dispatch was fired)

The dispatch was fired against a draft that was still local-only. By the
time the cycle-30 master commit pushed, the Copilot agent had already
cloned and was reviewing without the draft on the visible master.

The Copilot agent worked around the absence by treating the dispatch body
as the authoritative summary and using `1-research.md` line numbers as
the source-pattern grounding. The critique is still substantively useful;
the line-level prose accountability that the dispatch wanted was
weakened. Lens 7.2 explicitly named this: "A two-cycle latency only buys
value if the external reviewer can inspect the actual proposed prose. If
the dispatch depends on a missing draft and a summary, the workflow has
the cost of light prep without the main benefit."

**Persistence-mechanism observation.** Future feedback dispatches that
reference a draft file MUST follow commit-then-dispatch sequencing:

1. Write the draft to its file.
2. `git add` + `git commit` + `git push` the draft to master.
3. Verify the draft is on master (`git log --oneline -1 --
   <draft-path>`).
4. Then create the dispatch issue (`gh issue create` or
   `tools/dispatch-task`).

The dispatch-task tool may not enforce this — it's an orchestrator-level
sequencing discipline. The cycle-26 / cycle-30 environmental-constraint
observations (WebFetch/curl gated; dispatch-task gated) already documented
fallbacks; this is a different category — workflow-sequencing discipline,
not environmental-fallback. Both belong in the persistence-mechanism log
because both shape how the orchestrator produces feedback artifacts.

**A cycle-30 retroactive correction I'm flagging here.** The cycle-30
notes file says "Dispatched Copilot feedback-only on the draft as issue
#2790" — this is true in intent but misleading in fact. The draft was
not on master when the dispatch fired. A more honest framing would have
been "Dispatched Copilot feedback-only on the draft as issue #2790; the
draft commit landed near-simultaneously so the agent had to work from
the dispatch body summary." Cycle 31 is the right place to log this
correction, not to retroactively edit the cycle-30 notes file (which is
a record of cycle-30's actual disposition at commit time).

## Cross-checks before evaluation

Spot-checked Copilot's line references for accuracy:

- Lens 1.1 "1-research.md:1189-1190" → confirmed: "Two-system strict
  convergence on bounded-autonomy-loop / as architectural primitive."
- Lens 1.2 "1-research.md:1176-1194" → confirmed: iteration ceilings
  bullet.
- Lens 1.4 "1-research.md:1114-1116" → confirmed: mechanical
  enforcement bullet's "behavior constraints get test-suite enforcement".
- Lens 1.10 "1-research.md:1170-1174" → confirmed: entropy/AI slop's
  "Inversely-related to the redesign's prior accretion pattern."
- Lens 6.3 "Security defaults: openclaw, AutoGen, PAI ... Iteration
  ceilings: oh-my-codex and Voyager ... Cognition adjacent." →
  confirmed against bullets at 907-916 and 1176-1194.

Line refs are accurate. Copilot did the homework against `1-research.md`
even when the draft itself was unavailable. Verdicts below assume the
cited evidence is reliable.

## Lens 1: Family-boundary judgment calls (12 findings)

**1.1 — Iteration ceilings belongs in Family E, not Family D.** **ACCEPT.**
The bullet's center of gravity is "autonomous loops need explicit
ceilings, not open-ended runs" (line 1193) — that's retry-loop discipline,
not security/resource bound. The Family D placement was driven by
"resource consumption (loop count) and runtime" framing, but iteration
ceilings is about preventing runaway autonomy, which is a quality-discipline
concern (does the system know how to stop). Move to Family E.

**1.2 — Family D placement makes iteration ceilings look more mature than
the evidence supports.** **ACCEPT.** The Copilot reading is sharp:
security defaults is a 3-system pattern with substantive threat-model
substrate; iteration ceilings is 2-system strict + 1 adjacent partial.
Pairing them under "constraint" risks maturity transfer. Once iteration
ceilings moves to E (per 1.1), this concern dissolves naturally — Family
D no longer pairs them.

**1.3 — If Family D survives, name it around autonomy bounds, not
security/resource constraints.** **PARTIALLY ACCEPT — supersedes itself.**
With iteration ceilings moved to E (per 1.1), this advice applies only
to the remaining single-pattern Family D. Decision: Family D becomes a
single-pattern family for security defaults. Rename to "**Trust posture
& security defaults**" — narrow framing matching the single pattern,
honest about thinness. Don't artificially absorb security into Family A
or B (the security bullet isn't about decomposition or coordination
shape — it's about the system's trust boundary with external inputs).

**1.4 — Mechanical enforcement is correctly in Family E, not Family B.**
**ACCEPT (validation of cycle-30 placement).** Strong validation of the
draft's choice: "behavior constraints get test-suite enforcement rather
than just doc-prescription" (1114-1116) and "agent-readable error
messages" (1118-1121) → quality discipline, not orchestration. Moving
to B would absorb every deterministic substrate pattern.

**1.5 — Mechanical enforcement should explicitly distinguish itself from
code-vs-prompts split.** **ACCEPT.** Add cross-reference in the
mechanical-enforcement bullet: "Distinct from Family B's
'Deterministic code executes; LLM proposes' pattern: code-vs-prompts is
where execution authority lives; mechanical enforcement is whether
behavioral promises are regression-tested. Both involve deterministic
code constraining LLM behavior, but in different directions —
authority-allocation vs verification-discipline." Without this
cross-reference a reader will see OpenAI harness mechanical layers in
both Family B and Family E and wonder if it's duplicate evidence.

**1.6 — Plans-as-artifacts belongs in Family C IF C is "state/history/
artifacts," not just memory.** **ACCEPT (clarification of family
framing).** The cycle-30 draft named Family C as "State, memory, history"
which already covers artifacts (failure records, plans, append-only).
Keep Family C placement. Refine family description to emphasize
"durable system records" rather than "memory primitives" — i.e., the
family covers the SHAPE of persistent records (component-local,
append-only, failure-records, forward-spec plans, memory-as-architecture)
not just memory in the narrow sense.

**1.7 — Plans-as-artifacts should not hide inside component-local state.**
**ACCEPT.** The current 3+ "Component-local state persistence" bullet
(955-975) cites OpenAI "plans as first-class versioned artifacts" at
962-966; the 2-system "Plans/specs" bullet at 1140-1157 sharpens this
into a distinct forward-spec pattern. Add a note within the component-
local-state bullet saying "OpenAI plans-as-artifacts mention here is the
component-local-state framing; the forward-spec sharpening appears in
the plans-as-artifacts bullet later in this family — not duplicate
evidence, but two angles on the same systems." Cross-family backreference
preserves the relationship without reading as count-padding.

**1.8 — Small fixed team is an unflagged boundary call.** **ACCEPT.**
Copilot is right: the bullet straddles A (decomposition decision) and
B (orchestration topology). The systems support shows different
topologies (peer-flow / lead-worker / sequential mode). Decision: keep
in Family A as the primary placement (the bullet's first sentence is
about role-separation as architecture, which is decomposition), but add
explicit cross-reference: "*See also Family B's 'Multiple orchestration
patterns coexist' bullet — the role-separation surfaced here implements
across distinct topologies.*"

**1.9 — Per-agent model selection is another unflagged boundary call.**
**PARTIALLY ACCEPT.** Copilot frames this as competing with Family D's
resource constraints. With the cycle-31 dissolution of D as resource-
plus-security, the resource-allocation framing for per-agent model
selection has no rival home. Stays in Family A. The cost-vs-novelty
rationale Voyager carries (997-1001) is incidental — the load-bearing
pattern claim is "model selection attaches to agent role as an
architectural-primitive choice," which IS Family A material. No move
needed; no cross-reference needed.

**1.10 — Entropy / AI slop is anti-accretion posture, not merely
Quality.** **ACCEPT.** The "inversely-related to v1's accretion-as-
defense pattern (F12)" framing IS a cross-cutting interpretive claim,
not just a quality-cleanup claim. Family E is the right placement, but
the bullet should not be read as peer to mechanical enforcement. Add a
brief introductory clause to the bullet body distinguishing
"recurring-cleanup-as-infrastructure" (entropy) from "behavioral-
constraint-regression-testing" (mechanical enforcement) — both are
quality-discipline patterns at different stages.

**1.11 — Agent-hierarchy stance divergence should be visible near
Multi-agent decomposition.** **ACCEPT (resolved by hybrid divergence
placement, see Lens 4 verdict).** Adopting the hybrid (family-local
divergence callouts + final compact divergences index) addresses this:
agent-hierarchy stance divergence appears as a Family A "Divergence
within this family" callout adjacent to multi-agent decomposition and
small-fixed-team. Closer Lens 4 ACCEPT verdict makes this a consequence,
not a separate decision.

**1.12 — Security defaults in Family D requires handling the OpenAI
throughput counterexample.** **ACCEPT.** The throughput-regime scope
condition observation (single-system, OpenAI Harness; from cycle-22
notes) is a real moderator on the security-tight-defaults pattern. The
3-system convergence is genuine, but cycle-22 single-system
observations name OpenAI's throughput-vs-security trade-off as a scope
condition. Two responses possible:
- (a) Add the scope condition note to the security-defaults bullet
  body: "*Scope condition (single-system, OpenAI Harness):
  high-throughput regimes condition when the security-stance pattern
  applies — see `_notes/cycle-22-cross-system-synthesis.md`.*"
- (b) Treat the scope condition as a Family D divergence callout (per
  Lens 4 hybrid).

Decision: option (a) — inline scope-condition note in the bullet body.
Option (b) over-categorizes a single scope-condition into a divergence-
class structure when divergence-class is for cross-system disagreement,
not single-system scope-conditions.

## Lens 2: Family ordering bias (8 findings)

**2.1 — "Logical dependency" is not neutral.** **ACCEPT.** "Agent →
orchestration → state → security → quality" reads as a Phase 2 design
decomposition sequence, not a synthesis-reading sequence. The cycle-30
same-cycle cold-reader flagged this as borderline-PASS; Copilot's
external eye is more emphatic and right. Reject "logical dependency"
ordering criterion.

**2.2 — Current evidence does not force agent-first ordering; State/
History/Memory is the densest cluster.** **ACCEPT.** The State/History/
Memory family contains 5 patterns (component-local, failed-work,
append-only, memory-as-architectural-concept, plans-as-artifacts) plus
2 of 3 persistent divergences (memory architectural stance, state-shape).
That density is real. Agent-first ordering underplays it.

**2.3 — Agent-first inherits the old debate's salience.** **PARTIALLY
ACCEPT.** Multi-agent decomposition is partly a negative default rule.
True. But the bullet covers six systems with foregrounded support and
includes Cognition's strongest named-rejection — that combined density
makes it a foregrounding-ready pattern, not just a "negative default."
The concern about anchoring readers on the multi-agent question IS
real, but it's mitigated if Family A isn't the FIRST family.

**2.4 — Maturity-first within families, family-order by evidence
weight.** **ACCEPT (this becomes the decision).** Order families by
pattern count: State/History/Memory first (5 patterns + 2 divergences),
then Agent architecture (3 patterns + 1 divergence), then
Orchestration (3 patterns), then Quality/discipline (3 patterns),
then Trust posture & security defaults (1 pattern, thin family
acknowledged). Within each family, maturity badges (per Lens 3) make
the 3+/2-system distinction visible at the bullet level.

**2.5 — Problem-pressure order (state, quality, agent, orchestration,
security).** **DISMISS.** Problem-pressure ordering is itself Phase-2-
shaped — it pre-decides which architectural risks v2 should focus on.
Lens 2 critiqued the draft for smuggling Phase 2 design flow; problem-
pressure ordering does the same thing under a different framing.
Evidence-weight ordering is more honest because it surfaces the
actually-densest cluster as primary, not a hypothetical-priority cluster.

**2.6 — Alphabetical ordering is ugly but honest.** **DISMISS.**
Alphabetical would order: Agent, Orchestration, Quality, Security,
State. That's actually similar to "logical dependency" by accident
(state ends up last). But alphabetical is a degenerate criterion — it
doesn't reveal any structure of the evidence. Evidence-weight is
better: it's principled (count-of-patterns) and informative (largest
cluster surfaces first).

**2.7 — If "logical dependency" stays, needs warning label.** **DISMISS
(superseded by 2.4).** With evidence-weight ordering adopted, no
warning label needed.

**2.8 — Quality last is loaded.** **DISMISS (superseded by 2.4).** With
evidence-weight ordering, Quality lands fourth (3 patterns), not last.
Trust posture & security defaults lands fifth as a 1-pattern thin
family explicitly acknowledged.

## Lens 3: Maturity-marking-in-parentheticals adequacy (7 findings)

**3.1 — Parentheticals not enough for skim readers.** **ACCEPT.** The
current section has separate headings for 3+ tier (line 840) and
2-system tier (line 1098); skimming is unavoidable. Family-clustering
moves maturity into bullet parentheticals, which IS skim-defeating.
The cycle-30 draft's same-cycle cold-reader didn't catch this because
the cold-reader was author-internal — the author remembers maturity
tracking and reads bullets through that lens. A fresh reader would not.

**3.2 — 2-system bullets have different epistemic status.** **ACCEPT.**
Mechanical enforcement (1112-1138) and iteration ceilings (1176-1194)
explicitly carry strict-vs-loose framing or scope/adjacency hedges.
These are recorded-with-acknowledgment claims, distinct in kind from
seven-system small-core convergence (888-905). Visually equating them
is wrong.

**3.3 — Parenthetical scheme risks flattening 7-system, 6-system, 5-
system, 3-system, and 2-system patterns.** **ACCEPT.** Current bullets
already carry counts in body text (e.g., "Six-system foregrounded
convergence", "Seven-system convergence"). Family clustering risks
losing the visual count-prominence.

**3.4 — Use explicit maturity badges, not prose parentheticals.**
**ACCEPT.** Format proposal: bold prefix immediately after the bullet's
bold pattern title, before the body sentence. Examples:

- **Multi-agent decomposition is not a default.** **`[3+/6 systems]`**
  *(Cognition Devin's named-rejection is the strongest.)* openclaw
  VISION.md ...
- **Mechanical enforcement of regression-tested behavioral constraints.**
  **`[2-system strict + diversity hedge]`** OpenAI harness-engineering
  uses ...
- **Iteration ceilings with explicit numerical limits.** **`[2-system
  strict + 1 adjacent partial]`** oh-my-codex documents ...
- **Strong-defaults security with operator-controlled knobs.**
  **`[3+/3 systems + scope condition]`** openclaw: default DM policy ...

Badge taxonomy:
- `[3+/N systems]` — N is total support count
- `[3+/N systems + contrary stance]` — explicit anti-stance from a
  specific system (e.g., Cognition on small-fixed-team)
- `[3+/N systems + scope condition]` — single-system moderator (e.g.,
  OpenAI throughput on security defaults)
- `[2-system clean]` — clean 2-system convergence
- `[2-system strict + diversity hedge]` — strict count, with named
  loose-framing extensions
- `[2-system strict + N adjacent partial]` — strict count, with named
  adjacent support

Badges are visually prominent (backtick-monospace inside bold), short
enough to fit before the body sentence, and use a closed taxonomy.

**3.5 — Contrary-stance notes need higher visual weight than ordinary
hedges.** **ACCEPT.** The contrary-stance badge `+ contrary stance` is
a distinct sub-category, not a parenthetical hedge. The substantive
contrary-stance note remains in the bullet body (the existing two-
sentence treatment of Cognition's rejection on small-fixed-team is
right at body level; the badge surfaces it for skim readers).

**3.6 — Distinguish three kinds of caveat.** **ACCEPT.** The badge
taxonomy in 3.4 maps:
- Diversity hedge (substrate/framing variation): `+ diversity hedge`
- Scope condition (single-system moderator): `+ scope condition`
- Contrary stance (explicit anti-stance from specific system): `+
  contrary stance`
- Adjacent support (different bounding-axis or partial match): `+ N
  adjacent partial`

Each caveat type has a distinct badge marker. Fresh readers see
badges; deeper readers get the body explanation.

**3.7 — Cycle-22 rationale survives only if reader recovers it without
rereading intro.** **ACCEPT.** Add a one-paragraph preamble to the
section explaining badges-as-epistemic-status. See "Cross-cutting
findings" verdict 5 below.

## Lens 4: Persistent divergences placement (7 findings)

**4.1 — Keeping divergences separate preserves epistemic class but
loses topical tension.** **ACCEPT (the framing for the hybrid).**

**4.2 — Agent-hierarchy divergence belongs adjacent to multi-agent
and small-fixed-team.** **ACCEPT.** Family A becomes the natural home
for the agent-hierarchy stance divergence as a "Divergence within this
family" callout.

**4.3 — Memory architectural stance belongs inside Family C.**
**ACCEPT.** Family C (State/Memory/History) callout for openclaw
singleton plugin vs PAI top-level Principle 13 contrast.

**4.4 — State-shape divergence belongs inside Family C.** **ACCEPT.**
Same family C callout — file-per-component vs typed-channel-map. Both
divergences address the same family's topical area, so they cluster
naturally as a single Family C divergences callout.

**4.5 — Hybrid is better than either pure choice.** **ACCEPT.** The
verdict structure: family-local "Divergence within this family"
callouts within Family A and Family C; final compact "Divergences
index" section after the 5 family sections that lists the 3 divergences
with one-line summary + cross-reference back to the family-local
callout.

**4.6 — Don't create a full second divergence essay.** **ACCEPT.** The
final divergences index is one-line entries, not duplicated essays.
The substantive prose lives in the family-local callout.

**4.7 — Current draft's separate section is safest for not losing
divergences but weakest for synthesis.** **ACCEPT (this is the trade-
off the hybrid resolves).**

## Lens 5: Family C → D section transition (6 findings)

**5.1 — C → D transition awkward because D is conceptually under-
built.** **ACCEPT.** Resolved by Lens 6 verdicts (Family D becomes
single-pattern thin family for security defaults; iteration ceilings
moves to E). The C → D transition becomes "from durable record
architecture to trust posture" — still requires some bridging but the
bridge is cleaner with D as a focused 1-pattern family.

**5.2 — Draft's family description does some work but not enough.**
**ACCEPT.** With evidence-weight ordering (Lens 2.4 verdict), the
section ordering changes — State/History/Memory first, Agent second,
Orchestration third, Quality/discipline fourth, Trust posture &
security defaults fifth. The original C → D transition no longer
exists. The new flows: State → Agent (records → decomposition); Agent
→ Orchestration (decomposition → coordination); Orchestration →
Quality (coordination → quality discipline); Quality → Trust posture
(discipline → trust boundary).

**5.3 — Transition sentence proposal** ("After the record layer
determines what the system can remember and revisit, the next question
is what autonomous behavior the system is allowed to attempt at all").
**DISMISS.** With ordering changed (Lens 2.4), this specific transition
no longer occurs.

**5.4 — Need for prose glue is evidence of deeper issue.** **ACCEPT.**
This is the deeper finding — Family D coherence (Lens 6), resolved
there.

**5.5 — Dissolving D may produce cleaner flow.** **PARTIALLY ACCEPT.**
Don't fully dissolve — security defaults is a real architectural
concern that doesn't naturally absorb into A or B. Keep D as a single-
pattern thin family with an honest framing.

**5.6 — If D remains, put it after Agent/Orchestration, not after
State.** **PARTIALLY ACCEPT — superseded by Lens 2.4.** Evidence-weight
ordering puts D last (single pattern, smallest family). That's after
Agent/Orchestration AND after State. Honest about D's thin status.

## Lens 6: Family D coherence (7 findings)

**6.1 — Family D coherent only at high abstraction level.** **ACCEPT.**

**6.2 — Source evidence has different failure models (security vs
liveness/resource).** **ACCEPT.** This is the load-bearing argument
for splitting iteration ceilings out of D.

**6.3 — Systems supporting the two patterns barely overlap.** **ACCEPT.**
Security defaults: openclaw, AutoGen, PAI. Iteration ceilings: oh-my-
codex, Voyager + Cognition adjacent. Overlap = 0. Pairing them under
one family suggests evidence-clustering that doesn't exist.

**6.4 — Two single-pattern families would be ugly but honest.**
**PARTIALLY ACCEPT.** With iteration ceilings moved to Family E (per
1.1), there's only one single-pattern family (D). Family E gains
iteration ceilings as a fourth member, so E is not single-pattern. The
"two single-pattern families" proposal isn't needed.

**6.5 — Dissolving D is better than splitting it.** **DISMISS.**
Security defaults belongs as a separate concern. Absorbing into A
("agent autonomy is what's controlled by trust posture") strains
Family A's "decomposition" framing — security is about the system's
trust boundary with EXTERNAL inputs, not about how the system itself
is decomposed. Absorbing into B (orchestration) is similarly strained.
A 1-pattern thin family is more honest than artificial absorption.

**6.6 — If D kept, rename it and mark as thin family.** **ACCEPT.**
Rename to "Trust posture & security defaults" (single-pattern thin
family acknowledged). Family description: "A single-pattern family
covering how the system bounds its trust of external inputs and
execution environments. Pattern count is 1; this family is a thin
family acknowledging that trust-boundary configuration is a distinct
architectural concern not naturally absorbed into Agent architecture
or Orchestration."

**6.7 — Don't let D become a dumping ground for anything that says
"bounded".** **ACCEPT (codified by the rename + thin-family framing).**
The narrow framing prevents drift.

## Lens 7: Light-prep disposition vs in-place restructure (8 findings)

**7.1 — Light-prep principled if draft genuinely exposes contested
judgment calls.** **ACCEPT (validation of cycle-30 disposition).** The
draft's three named questions WERE real (and Copilot found two more
unflagged). The disposition was justified.

**7.2 — Looks like process overhead if primary draft artifact not
present for reviewer.** **ACCEPT.** Resolved by the persistence-
mechanism observation above (commit-then-dispatch sequencing).

**7.3 — Cycle-29 plan made cycle-30 sound ripe for in-place.**
**PARTIALLY ACCEPT.** The cycle-29 plan suggested cycle 30 do the
in-place restructure. Cycle 30 made the disposition shift to light-
prep with rationale (architecturally-load-bearing). That shift was
justified — the draft DID surface real concerns (Family D, ordering,
badges) that Copilot deepened. But the disposition shift cost a cycle.
The discipline-lightening rule applies: if light-prep produces clear
verdicts that get applied promptly, it's principled; if it just
defers, it's procrastination.

**7.4 — Workflow adds value over commit-now-iterate-later if it
prevents structural lock-in.** **ACCEPT.** Once a family restructure
lands, later reviewers critique within the accepted family boundaries.
External review BEFORE lock-in is the highest-value time.

**7.5 — Most likely missed concern: maturity flattening.** **ACCEPT
(validation of badges decision per Lens 3.4).**

**7.6 — Second likely missed concern: D's false coherence.** **ACCEPT
(validation of D dissolution per Lens 1.1, 6.6).**

**7.7 — Third likely missed concern: agent-first ordering as hidden
Phase 2 design flow.** **ACCEPT (validation of Lens 2.4 ordering
change).**

**7.8 — Verdict: principled but fragile.** **ACCEPT (validation
caveat).** Cycle 31 (this cycle) explicitly resists the procrastination
slip by producing CLEAR DECISIONS in this verdict file. Cycle 32+ is
the in-place restructure cycle. If cycle 32 ALSO defers, then the
fragility verdict converts to procrastination.

## Cross-cutting observations (5 findings)

**XC.1 — Family-clustering is probably right, but not sufficient.**
**ACCEPT.** Topic-area discovery is the load-bearing motivation;
maturity badges + family-local divergences + cross-references address
the "not sufficient" concerns.

**XC.2 — Strongest family is State/Memory/History, not Agent
Architecture.** **ACCEPT (drives Lens 2.4 ordering change).**

**XC.3 — Stop treating all caveats as equal.** **ACCEPT (drives Lens
3.6 caveat-type taxonomy).**

**XC.4 — Some bullets need cross-family backreferences.** **ACCEPT.**
Three cross-references identified:
- Mechanical enforcement (Family E) ↔ Code-vs-prompts split (Family B):
  per Lens 1.5 verdict
- Plans-as-artifacts (Family C) ↔ Component-local state (Family C, same
  family but cross-reference within): per Lens 1.7 verdict
- Small fixed team (Family A) ↔ Multiple orchestration patterns
  (Family B): per Lens 1.8 verdict

**XC.5 — Preserve old maturity argument in intro, not just bullets.**
**ACCEPT.** Add a section preamble (~3-4 sentences) explaining:
- Section uses family-clustering for topic-area discovery
- Maturity badges (`[3+/N]`, `[2-system clean]`, etc.) carry the
  cycle-22 epistemic distinction at the bullet level
- Family-local divergence callouts surface in-family disagreements
- Final divergences index lists divergences as a class

## What Copilot did NOT critique (4 findings)

**ND.1 — Did not line-edit draft prose.** Confirmed: line-level prose
critique was infeasible due to draft absence on master. Process
observation (commit-then-dispatch) addresses this for future dispatches.

**ND.2 — Did not critique underlying system research sections.** Out
of scope for this dispatch. No verdict needed.

**ND.3 — Did not propose in-place replacement structure.** Hard
constraint of the dispatch — critique-only. Proper.

**ND.4 — Did not evaluate systems not in cross-system observations
population.** oh-my-claudecode and openai/symphony are queued (Eva
directives #2774 / #2775). Verdict applies: the cycle-31 verdicts here
are about the EXISTING population restructure; future system reads
will trigger their own integration cycles, not retroactively change
the cycle-32+ restructure.

## Structural decisions distilled for cycle 32+ in-place restructure

The cycle-31 verdicts collapse to seven structural decisions:

**Decision 1 (family count and contents): 5 families with revised
membership.**

- **Family A: Agent architecture.** Multi-agent decomposition;
  Per-agent model selection; Small fixed team. (3 patterns + 1
  divergence callout: Agent-hierarchy stance.) Cross-references: Small
  fixed team ↔ Family B's Multiple orchestration patterns.
- **Family B: Orchestration & system shape.** Code-vs-prompts split;
  Small core extends via plugins; Multiple orchestration patterns. (3
  patterns.) Cross-references: see Family E mechanical-enforcement
  cross-reference.
- **Family C: State, memory, history.** Component-local state
  persistence; Failed work as recorded artifact; Append-only history;
  Memory as architectural concept; Plans/specs as forward-versioned.
  (5 patterns + 2 divergence callouts: Memory architectural stance,
  State-shape divergence.) Cross-references: Plans-as-artifacts ↔
  Component-local state's plans mention.
- **Family D (renamed): Trust posture & security defaults.**
  Strong-defaults security with operator-controlled knobs. (1 pattern,
  thin family acknowledged.) Inline scope-condition note for OpenAI
  throughput-vs-security.
- **Family E: Quality & discipline.** Anti-patterns explicit as
  deliverable artifact; Mechanical enforcement; Iteration ceilings;
  Entropy / AI slop. (4 patterns.) Cross-references: Mechanical
  enforcement ↔ Family B's Code-vs-prompts split.

Note on family E gaining iteration ceilings: that's the Lens 1.1
move from D to E.

**Decision 2 (family ordering): evidence-weight ordering.**

C (5 patterns) → E (4 patterns) → A (3 patterns) → B (3 patterns) → D
(1 pattern). Tie-breaking between A and B by alphabetical (A before B)
or by other principled criterion if needed.

**Decision 3 (maturity badges): explicit visual prefixes.**

Closed taxonomy of badges:
- `[3+/N systems]`
- `[3+/N systems + contrary stance]`
- `[3+/N systems + scope condition]`
- `[2-system clean]`
- `[2-system strict + diversity hedge]`
- `[2-system strict + N adjacent partial]`

Badges immediately follow the bold pattern title, before the body
sentence. Backtick-monospace inside bold for visual prominence.

**Decision 4 (divergences placement): hybrid.**

Family-local "Divergence within this family" callouts in Family A
(Agent-hierarchy stance) and Family C (Memory architectural stance,
State-shape divergence). Final compact "Divergences index" section
after the 5 family sections — one-line per divergence + cross-
reference to the family-local callout.

**Decision 5 (cross-family backreferences): three identified.**

- Mechanical enforcement (Family E) ↔ Code-vs-prompts split (Family B)
- Plans-as-artifacts (Family C) ↔ Component-local state's plans
  mention (Family C internal)
- Small fixed team (Family A) ↔ Multiple orchestration patterns
  (Family B)

Implementation: each cross-reference is a sentence in the bullet body,
not a hyperlink. The cross-reference makes the relationship explicit
without count-padding.

**Decision 6 (section preamble): ~3-4 sentences before family
sections.**

Content:
- Section uses family-clustering for topic-area discovery
- Maturity badges carry the cycle-22 epistemic distinction at bullet
  level (3+ tier vs 2-system tier vs scope conditions vs contrary
  stances)
- Family-local divergence callouts surface in-family disagreements
- Final divergences index lists the 3 divergences as a class

**Decision 7 (single-system observations and `_notes` cross-system
synthesis): unchanged.**

The `_notes/cycle-22-cross-system-synthesis.md` single-system
observations remain in their current location. The cycle-31
restructure is for the `1-research.md` cross-system observations
section only. Single-system observations are not yet load-bearing for
Phase 2 candidate generation per the cycle-22 framing.

## Cycle-32+ pre-commits

Cycle 32 absorbs the in-place restructure based on the seven decisions
above. Specific items:

1. **Apply the in-place restructure to `1-research.md`.** Replace
   lines 817-1219 (current Cross-system observations section) with
   the family-clustered version implementing decisions 1-6. Preserve
   the cycle-30 draft's family-internal coherence rationale where
   appropriate. Architecturally-load-bearing.

2. **Same-cycle cold-reader on the in-place restructure.** Standard
   pattern: cold-reader checks for anti-smuggling, transferability
   symmetry, self-introduced errors, section transitions. Specific
   questions:
   (a) Did the badge taxonomy get applied uniformly across all 16
       patterns, or were any skipped/inconsistent?
   (b) Did the family-local divergence callouts preserve the cycle-22
       framing without smuggling Phase 2 prescriptions?
   (c) Did the cross-family backreferences read as substantive
       relationships or as bookkeeping?
   (d) Does the section preamble explain badges + divergences hybrid
       cleanly without over-prescribing?

3. **Update Persistent Divergences section** (carry-forward from
   cycle 27-30). Now PARTIALLY RESOLVED by decision 4 (hybrid
   placement); remaining work is the substantive per-divergence
   prose (Cognition Devin's anti-stance content, throughput-regime
   scope-condition content). Substantive prose work, but constrained
   by the new family-local callout structure.

4. **Cross-validate against audit's A-pattern mapping** (carry-forward
   from cycle 25-30). Bounded mechanical.

5. **Read remaining audit retrospective sections** (carry-forward from
   cycle 25-30). "What v2 must demonstrably do better" section is the
   most relevant for Phase 2.

6. **Copilot research-only dispatch: oh-my-claudecode** (Eva directive
   #2774). Deferred from cycles 26-30. Cycle-32+ if budget permits.
   With commit-then-dispatch sequencing now in the persistence-
   mechanism log, the dispatch must follow that discipline (any
   reference artifacts must commit before dispatch fires).

7. **Copilot research-only dispatch: openai/symphony** (Eva directive
   #2775). Same gating as item 6.

8. **Codify SUPPORT/CONTRADICT gradient definition** (carry-forward
   from cycle 28-30). Bounded mechanical; defer until matrix shape
   is re-used.

9. **Codify third-category refinement to discipline-lightening rule**
   (carry-forward from cycle 28-30). Defer until trigger event occurs.

10. **Same-cycle cold-reader on this notes file (cycle-31).** Standard
    cycle-N+1 fresh-eye pass. Specific questions:
    (a) Are the seven structural decisions clearly stated and
        actionable for cycle 32, or do any read as "needs more
        thought" defer-language?
    (b) Did the Family D resolution (rename to "Trust posture &
        security defaults", thin-family acknowledged) avoid the
        "absorb into A or B artificially" failure mode while also
        avoiding Lens 6.5's "dissolve D entirely" alternative?
    (c) Does the dispatch-then-commit persistence-mechanism
        observation read as a clear discipline (commit-then-
        dispatch) or as ad-hoc workflow advice?
    (d) Does the verdict count (47 ACCEPT, 7 PARTIALLY ACCEPT,
        6 DISMISS) read as honest engagement or as anchor-on-
        Copilot's-framing? The DISMISS verdicts especially — were
        they substantively justified or did I accept Copilot's
        framing too readily?

11. **Long-deferred items roll-call** (carry-forward, 9 items
    unchanged cycles 26-30; carry into cycle 32+).

### Suggested cycle 32 plan (provisional)

- **Focal:** item 1 (in-place restructure based on cycle-31 verdicts).
  Architecturally-load-bearing prose work; ~400 lines of `1-research.md`
  cross-system observations rewritten.
- **Bounded mechanical:** item 2 (cold-reader on in-place restructure)
  + item 10 (cold-reader on this cycle-31 notes file).
- **Defer:** items 3-9, 11 to cycle 33+ as cycle-32 absorbs the
  load-bearing restructure.
- **Possible Copilot dispatch:** item 6/7 (oh-my-claudecode and/or
  openai/symphony) IF cycle-32's focal completes early. Otherwise
  cycle 33+.

## Same-cycle cold-reader on cycle-30 notes file

Per cycle-30's named cold-reader questions:

**(a) Does the family-boundary rationale read as principled topic-area
discovery or as ad-hoc grouping?** **PASS.** The cycle-30 draft's
family rationale was principled at the boundary level: each family
description named the topical area covered ("Patterns about how agents
are decomposed and organized" for A; "Patterns about where state lives,
how it persists" for C); family-internal coherence sentences explained
the within-family sequence. The Family D coherence concern that the
draft itself flagged as borderline turned out to be the load-bearing
finding from external review — the draft's self-flagging was
appropriate. The remaining boundaries (A, B, C, E) survived external
review.

**(b) Does the light-prep disposition rationale (draft + external
feedback before in-place restructure) feel principled or
procrastinating?** **BORDERLINE-PASS.** The Copilot critique's Lens 7
"verdict: principled but fragile" matches my fresh-eye reading.
Principled because the draft surfaced real concerns Copilot deepened
(family D dissolution, ordering bias, badges weakness, two unflagged
boundary calls); fragile because the in-place application is now
deferred to cycle 32+, total elapsed ~3 cycles for what could have
been 1. The fragility tips toward procrastination if cycle 32 also
defers. This file's seven decisions are forced clear to prevent that
slip.

**(c) Are the five open questions (family boundaries 1-3 + ordering +
divergences placement) actionable for a Copilot feedback session, or
are they too high-level?** **PASS.** Copilot answered all five with
substantive findings: family boundaries 1-3 each got verdicts (1.1
move iteration ceilings to E; 1.4 mechanical enforcement stays in E;
1.6 plans-as-artifacts stays in C with framing refinement); ordering
got the strongest critique (Lens 2.1 reject "logical dependency",
Lens 2.4 propose evidence-weight); divergences placement got the
hybrid resolution (Lens 4.5). All five questions were actionable.

**(d) Did the dispatch-task → gh-api fallback documentation capture
the pattern at the right level of detail?** **BORDERLINE-PASS.** The
cycle-30 notes' fallback documentation (lines 416-431) named the
2-step pattern (gh issue create + gh api assignees POST), referenced
the production prompt's lines 295-328 for the JSON payload structure,
and noted the cycle-26 environmental-constraint analogue. That's at
the right detail level for a future cycle to apply without re-deriving.
But: the documentation didn't mention the SEQUENCING discipline
(commit-then-dispatch), which is a separate workflow concern — the
gating fallback is for HOW to dispatch, not WHEN. The two should be
mentioned together: "(1) commit reference artifacts to master before
dispatching; (2) if `tools/dispatch-task` is gated, use the 2-step
gh-api fallback per cycle-30 documentation." Cycle 31 captures this
in the persistence-mechanism observation above; the discipline
extends rather than supersedes the cycle-30 documentation.

**Cold-reader verdict: 4/4 PASS or BORDERLINE-PASS** with the
sequencing discipline as a real extension to be captured (already
captured in this file's "Process observation" section).

## Persistence-mechanism observations

**Cycle-7 / cycle-12 / cycle-31 pattern: per-finding-evaluation as
load-bearing focal.** Cycle 31 is the third application of the per-
finding-evaluation pattern (after cycles 7 and 12). Pattern shape
stable: external feedback dispatch lands → next cycle does verdict
file (ACCEPT / QUALIFY / DISMISS per finding) → following cycle does
in-place application. Each cycle is bounded; the chain prevents
"land feedback and try to integrate it in one cycle" failure
modes.

**Discipline: verdicts must be CLEAR DECISIONS, not deferrals.** The
risk of the per-finding-evaluation cycle is producing fuzzy verdicts
that don't drive concrete cycle-N+1 action. Cycle-31's verdicts are
forced clear. Counted by category: **47 ACCEPT** with concrete
consequence; **7 PARTIALLY ACCEPT** with stated qualification (3 of
which note explicit supersession by another verdict); **6 DISMISS**
with reasoning. Total: 60 verdicts across 60 findings (Lens 1-7 + 5
cross-cutting); plus 4 "What Copilot did NOT critique" notes. The
"Decisions distilled" section collapses verdicts to seven structural
decisions ready for cycle-32+ application.

**Commit-then-dispatch sequencing.** New observation for the
persistence log: any feedback dispatch referencing a draft artifact
MUST commit and push the artifact before firing the dispatch. The
cycle-30 dispatch fired with the draft local-only; the Copilot agent
cloned without the draft visible on master and had to work from the
dispatch body summary alone. The line-level prose accountability the
dispatch wanted was lost. Future dispatches: commit → push → verify
on master → dispatch.

**Honest-hedge tally extended: 10/10 stable.** The cycle-30 honest-
hedge applications (entropy/AI-slop "v1's accretion-as-defense
pattern (F12)"; throughput "conditions when the security-stance
pattern applies") still survive and now propagate into the cycle-31
verdicts. The honest-hedge pattern continues stable.

**Family-D-as-thin-family is a new pattern instance.** The verdict
to keep Family D as a single-pattern family with explicit thin-
family acknowledgment is itself an instance of the cycle-25
discipline-lightening rule: don't manufacture coherence when the
evidence doesn't support it; record honestly with caveat. The thin-
family framing is the structural analogue of the diversity-hedge
clause for individual patterns.

## Same-cycle cold-reader on this notes file

Per cycle-19 same-cycle-cold-reader-on-rewrite pattern, ran the
cold-reader on the verdict-evaluation prose before commit.

### Anti-smuggling discipline

Walked the verdicts and decisions for v2-prescription smuggling:

- **Decisions 1-7** are about how to STRUCTURE THE CROSS-SYSTEM
  OBSERVATIONS in `1-research.md` (family clustering, ordering,
  badges, divergence callouts, cross-references, preamble, single-
  system observations location). They do NOT prescribe how v2 should
  be designed. The structure decisions are about reader navigation
  and epistemic clarity. PASS.
- **Decision 2's evidence-weight ordering** (State first, then E, A,
  B, D) could be misread as "v2 should be designed primarily around
  state architecture." Re-reading: the ordering is justified by
  COUNT-OF-PATTERNS in the cross-system population, not by Phase 2
  priority. The body decision text says "Order families by pattern
  count" — explicit count-based criterion, not priority-based. PASS,
  but flag for the cycle-32 in-place restructure to make this
  ordering rationale explicit in the section preamble (Decision 6).
- **Family D rename to "Trust posture & security defaults"** —
  could read as v2-prescription that v2 should adopt strong-defaults
  security. Re-reading: the family name describes the TOPIC AREA
  the cross-system pattern occupies, not a v2 prescription. PASS.

### Cycle-18 transferability symmetry

Each verdict carries explicit reasoning. ACCEPT verdicts have
"Rationale: ..." or supporting clause; DISMISS verdicts have
"DISMISS because ..." rationale; PARTIALLY ACCEPT verdicts state the
qualified version. Symmetry maintained: rejected and accepted
findings get equivalent rationale-depth.

One borderline: **Lens 7.5/7.6/7.7 "ACCEPT (validation of ...)"**
verdicts are confirmations-of-existing-decisions rather than
load-bearing new decisions. They count as 3 ACCEPT verdicts but
their substantive weight is "Copilot's prediction agrees with my
prior analysis." Counted honestly but flagged: cycle 32+ readers
should not over-weight count tallies as if all ACCEPT verdicts have
equal load-bearing impact.

### Self-introduced errors check

One real error caught and fixed during this cold-reader pass: the
verdict-tally claim. Initial draft said "40+ ACCEPT, 4-5 PARTIALLY
ACCEPT, 4-5 DISMISS, 1-2 superseded." Actual count: 47 ACCEPT, 7
PARTIALLY ACCEPT, 6 DISMISS, with 3 of the PARTIALLY ACCEPT noting
explicit supersession. Fixed in two locations (persistence-mechanism
observations + pre-commit item 10(d) cold-reader question).

This is a self-introduced error of the same class as cycle-28's
matrix per-row-correction: tally claims made without empirical
counting. The discipline-lightening rule's loose-framing failure
mode applies — bounded mechanical work (counting) was treated as
mechanical rather than substantive. Since the count claim is
load-bearing for the "honest engagement vs anchor-on-Copilot's-
framing" cold-reader question, the error mattered. Captured in
the persistence-mechanism log as "tally claims need empirical
verification, not estimation."

One borderline framing flagged for cycle-32 fresh-eye: **Lens 6.5
DISMISS rationale**. The rationale ("absorbing security into A or
B is strained") is correct but doesn't directly address the
counter-argument that the proposed alternative (single-pattern thin
family) ALSO has structural strain (the cycle-30 draft warned about
"small-family ceremonialness"). The DISMISS could read as
anchor-on-cycle-30-framing rather than fresh evaluation. The
substantive answer (1-pattern thin family is more honest than
artificial absorption) IS load-bearing, but the "strained vs
strained" comparison should be made explicit. Cycle-32 fresh-eye
to consider whether to add a clarifying sentence.

### Section-transition smuggling check

- "Process observation" → "Cross-checks before evaluation" → "Lens
  1": clean transition. Cross-checks establish the line-ref
  reliability assumption that the verdicts then use. PASS.
- "Lens 7" → "Cross-cutting observations": clean — Lens 7 closes
  the per-lens evaluation, cross-cutting summarizes findings that
  span multiple lenses. PASS.
- "What Copilot did NOT critique" → "Structural decisions
  distilled": clean — non-critiqued items are filed, then the
  load-bearing output of the verdict file (the seven decisions)
  follows. PASS.
- "Cycle-32+ pre-commits" → "Same-cycle cold-reader on cycle-30
  notes file" → "Persistence-mechanism observations" → "Same-cycle
  cold-reader on this notes file": the two cold-reader sections are
  on different artifacts (cycle-30 notes, then cycle-31 notes) but
  the section names are similar. Worth a small clarifier. The
  current section headers already include "on cycle-30 notes file"
  and "on this notes file" which distinguishes them. PASS.

### Cold-reader verdict

PASS with two flags:
- The verdict-tally error was caught and fixed during this pass
  (self-introduced error of the cycle-28 matrix-correction class).
- The Lens 6.5 DISMISS rationale could benefit from one clarifying
  sentence acknowledging the "strained vs strained" comparison.
  Flagged for cycle-32 fresh-eye second-pass per the cycle-29
  borderline → cycle-30 fresh-eye pattern.

The cycle-31 notes file is ready for commit.
