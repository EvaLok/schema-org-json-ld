# Cycle 13: cold-readers on cycle-12 Tier-1 + Tier-2 group 1 (family-summary rewrite)

Cycle 12 (commit `d7a4e480`) evaluated the cycle-11 Copilot dispatch (PR
#2756, 25 findings across 7 lenses), applied 7 Tier-1 edits, and deferred
9 Tier-2 groups with explicit cycle-13+ pre-commits. Cycle 13 executes the
five named pre-commits: four cold-reader checks on cycle-12's Tier-1
edits, and one Tier-2 group selection-and-execution.

## Cold-reader checks on cycle-12 Tier-1 edits

### Check 1: cross-family notes restructure (5.A + 5.B + 5.C)

The post-cycle-12 section at lines 936-973 reads "v2 design implications
by family" with a one-paragraph opening pointing to load-bearing claims
(family preamble + F11 paragraph + F12 hypothesis), three blockquotes
(defense-accretion implication; procedure/review-overhead implication;
tooling-fragility implication), a deferral paragraph for the
reconciliation-asymmetry implication (which lives in the family preamble),
and a closing observation about asymmetry-as-bug.

**Verdict: PASS.** The section reads coherently as a per-family digest
of v2 design implications. The opening pointer is functional (a cold
reader looking for the load-bearing claims is directed correctly). The
three blockquotes are genuinely per-family-implication content (defense
accretion → boundary architecture; procedure → small skeleton; tooling
fragility → no parallel paths). Transitions to the surrounding sections
(F12 hypothesis above; "What appears to be working" below) are clean.

**Minor flag.** The section opening calls reconciliation-asymmetry "the
dominant family" — this is consistent with the family-preamble structure
(reconciliation-asymmetry's single-sentence statement leads the family
discussion) but is not explicitly defended elsewhere as the *dominant*
designation. Defensible as-is; could be examined if a future cycle does
a "what's claimed but not argued" sweep.

### Check 2: 1.C wording change ("structurally produces")

Two body locations updated by cycle 12 (lines 171 and 731). Cycle 13
also rewrote line 167-172 as part of Tier-2 group 1 (the family-summary
rewrite moved the freeze-vs-refresh framing out of the family-summary
into the family-overlap paragraph at lines 184-194; "structurally
produces" is preserved there at lines 188-189 in its proper F11-local
context).

**Verdict: PASS.** "Structurally produces" is meaningfully better than
either "produces" (too weak — could be coincidental) or "guarantees"
(too strong — claims inevitability beyond 3-cycle evidence). It names
the mechanism (the structure) without claiming universality. Considered
alternatives (systematically/predictably/mechanically/causes); none
strictly dominates "structurally."

**One flag for cycle 14+.** The iteration-plan section at line 1287-1289
contains a historical quote of the cycle-7 framing that includes the
word "guarantees": *"v1 defenses are end-of-cycle/next-cycle refreshers;
artifacts freeze before refreshers finish; architecture guarantees
post-close divergence."* This is technically accurate as a historical
record (cycle 7 did use that wording), and the README iteration log
captures the cycle-7-to-cycle-12 evolution correctly. But after cycle
13's family-summary rewrite, the body has moved further from this
historical wording. When Tier-2 group 5/6.B (iteration plan move to
README) executes, the historical quote should be either preserved with
explicit "this was cycle 7's framing, since softened" context, or
updated to reflect the current load-bearing language. Defer to cycle 14+
when group 5 runs.

### Check 3: 6.A glossary "F-pattern" tightening

Pre-cycle-12: "since they are not equally independent" — presented the
four-family grouping as a logical consequence of an established premise.

Post-cycle-12 (lines 65-69): "The failure-families preamble groups them
into four working families *as a hypothesis under iteration*; the
grouping is the artifact's current best read on how the patterns relate,
*not a settled taxonomy*."

**Verdict: PASS.** Avoids both the foreclosing problem (claiming the
grouping is a logical consequence) and the opposite-extreme problem
(implying the grouping is unsupported). "Current best read" affirms
the grouping has substantive backing; "not a settled taxonomy" makes
clear it remains under iteration. Strikes the right balance.

### Check 4: 6.D working-notes-pattern caveat

Lines 1015-1027. Cycle-12 added "*This is a redesign-era addition, not
a v1 working feature* — included here because v2 should preserve it,
not because v1 had it."

**Verdict: PASS, borderline.** The caveat is mechanically correct and
honest — readers will not be misled into thinking the working-notes
pattern is a v1 feature. The italicized inline caveat at line 1015-1017
is more prominent than typical *Caveat:* markers in adjacent bullets,
which gives appropriate weight given that the bullet is in a "What
appears to be working" section listing v1 features.

**Minor borderline.** The bullet now has two markers for the same
fact: "(cycle 3 addition)" parenthetical at line 1015 and the italicized
"redesign-era addition" caveat at line 1015-1017. Mild redundancy.
Could be tightened to one of the two, but not blocking. Could be revisited
in the same cycle that does a length-refactor sweep (Tier-2 group 8).

## Cold-reader summary

All four cold-reader checks pass. No regressions found. Two minor flags
recorded:
- Check 2: iteration-plan historical quote at line 1287-1289 still uses
  "guarantees"; bundle with Tier-2 group 5/6.B (iteration plan move).
- Check 4: minor redundancy between "(cycle 3 addition)" parenthetical
  and italicized caveat in the working-notes-pattern bullet; bundle
  with Tier-2 group 8 (F-section length sweep) or revisit if cycle 13+
  does a redundancy sweep.

The cycle-N-pre-commits-cycle-N+1-checks chain holds for cycle 13.
Cycle 12's pre-commits gave four cold-reader checks plus a
Tier-2-group selection; cycle 13 ran all four checks and selected.

## Tier-2 group 1: family-summary rewrite

Cycle 12 deferred 9 Tier-2 groups. Cycle 13 picked **group 1
(family-summary rewrite, addressing findings 1.B + 2.A + 2.C)** because
it is the largest single load-bearing prose change and addresses the
deepest cluster of findings.

### Findings being addressed

- **1.B**: The defense-accretion family-summary at lines 167-172
  (post-cycle-7) described F11's local mechanism (freeze-vs-refresh) as
  if it were the family-wide claim. F1 (constraints), F12 (cross-substrate
  cataloging), and F5 (write-mostly state) do not share the freeze-vs-refresh
  mechanism.
- **2.A**: F12's positioning as "at the cross-substrate cataloging level"
  strained the substrate metaphor. The other three F-patterns name "what
  kind of thing" is accumulating (prompt/checklist substrate; state-shape
  substrate; temporal substrate). F12 names "an activity" (cataloging
  across substrates). These are not parallel categories.
- **2.C**: The "sibling-not-upstream" claim was asserted (in the F11
  paragraph at the post-cycle-10 line 731-744) but not evidenced. The
  evidence is theoretical (different local mechanisms), not observational
  (no removal-test data).

### Edits applied

Four locations needed updating to maintain consistency:

#### Edit 1: Family preamble single-sentence summary (lines 167-194)

Replaced the freeze-vs-refresh-as-family-summary text with a family-wide
single-sentence statement and a sibling-manifestations explanation
paragraph. The freeze-vs-refresh insight was preserved in the
family-overlap paragraph below, where it is now explicitly attributed
to F11's local mechanism (lines 184-194). Cycle 12's "structurally
produces" wording is preserved there.

New family-summary blockquote (lines 169-172):

> When a failure surfaces, v1's response is to add a substrate-specific
> defense (a constraint, a state field, a cycle-boundary check, an
> additional pipeline-check) and never test whether the defense is
> still load-bearing.

Followed by a sibling-manifestations paragraph (lines 174-182) that
names F1/F5/F11/F12 substrate roles, reframes F12 as catalog-not-substrate,
and adds the sibling-not-upstream caveat with explicit acknowledgment
of the missing removal-test evidence.

#### Edit 2: F11 architectural-implication paragraph (lines 742-761)

Pre-cycle-13: "F1, F12, F5, and F11 are parallel manifestations of
defense accretion at four substrates — F1 names the response-shape
pattern, F12 catalogs the cross-substrate accumulation, F5 names the
state-shape consequence, and F11 names the temporal stage. They are
not upstream stages of the freeze-vs-refresh timing collision
specifically; they are sibling manifestations of the same family. ..."

Post-cycle-13: The paragraph drops "at four substrates" framing,
reorders to F1/F5/F11/F12 (more natural prompt → state → temporal →
catalog flow), adds review-disposition surface to F12's catalog
coverage (per finding 2.B's partial integration — F12 catalogs
"spanning the prompt/state/temporal substrates plus the
review-disposition surface"), and explicitly adds the sibling-not-upstream
caveat. The "v2 must address each at its own substrate" closing claim
is preserved.

#### Edit 3: F12 hypothesis substrate paragraph (lines 888-911)

Pre-cycle-13: "Defense accretion appears at four substrates within the
family. F1 manifests at the prompt/checklist layer; F5 at the state-shape
layer; F12 at the cross-substrate cataloging level; and F11 at the
temporal layer. ... The freeze-vs-refresh timing collision is the
mechanism that binds F11 to F1, F5, and F12 ..."

Post-cycle-13: Reframed to "Defense accretion appears at three
substrates within the family — F1 at the prompt/checklist substrate,
F5 at the state-shape substrate, and F11 at the temporal substrate.
F12 catalogs the accumulation across these substrates plus the
review-disposition surface; it is the 'across' rather than a fourth
substrate of the same kind."

The cycle-6 named-tools evidence chain is preserved but reframed: "The
freeze-vs-refresh timing collision is F11's local mechanism; the
named-tools-fire-post-close-on-named-fields chain is observation-level
evidence that the four F-patterns operate on the same surface (the same
tools fire post-close, mutating the same fields, the same catalog
records them) — shared activity, not evidence of a single shared
upstream cause."

This shift — from "the freeze-vs-refresh collision is the mechanism that
binds F11 to F1+F5+F12" → "shared activity at the observation level, not
shared upstream cause" — is the precise correction that finding 1.B
demands. The cycle-6 evidence still has its place; what changes is what
it is taken to show.

#### Edit 4: v2 design implications by family section pointer (line 938-942)

Pre-cycle-13 (post-cycle-12): "defense-accretion's four-substrate
breakdown is in the F11 architectural-implication paragraph and the
F12 hypothesis."

Post-cycle-13: "defense-accretion's substrate-and-catalog breakdown —
three substrates plus F12's cross-substrate catalog — is in the F11
architectural-implication paragraph and the F12 hypothesis."

#### Edit 5 (consistency): Family table parenthetical (line 140)

Pre-cycle-13: "(constraints, state fields, pipeline-checks). Defenses
accumulate across substrates."

Post-cycle-13: "(constraints, state fields, cycle-boundary checks,
pipeline-checks). Defenses accumulate across substrates without
removal-tests."

The table parenthetical now matches the family-summary's parenthetical
examples. The "without removal-tests" addition aligns with the
family-summary's "and never test whether the defense is still
load-bearing" framing.

### What this rewrite did NOT do

Tier-2 findings deferred or partially-addressed:

- **2.B (review/disposition surface as fifth substrate / F9 dual-membership)**:
  partially integrated by mentioning "review-disposition surface" in
  F11 paragraph and F12 hypothesis. The full question — is review-disposition
  a fifth substrate, or does F5 cover it (broaden F5's framing), or
  does F9 dual-membership at this surface address it — remains
  Tier-2 group 2 work. Cycle 14+.

- **3.A + 3.B (freeze-vs-refresh framing alternative)**: not addressed
  this cycle. The freeze-vs-refresh framing now lives in F11's local
  mechanism (lines 187-189) rather than the family-summary, which
  reduces its scope; but the question of whether "defenses fire too
  late" or "artifacts freeze too early" is the better framing remains
  open. Tier-2 group 3 work. Cycle 14+ — this is the most consequential
  single finding from cycle 11's dispatch.

- **4.A + 4.B + 4.C (nine measures rework)**: not addressed.

- **5.A + 5.B + 5.C aftermath**: cycle 12 already applied the Tier-1
  cuts. The cross-family notes section is now the v2-design-implications
  section per cycle 12.

- **6.B + 7.A (iteration plan move to README)**: not addressed; flagged
  in cold-reader 2 because the family-summary rewrite makes the
  historical "guarantees" quote in the iteration plan more stale.
  Tier-2 group 5 work.

- **6.C (preserved-through-cutover disposition)**: not addressed.

- **7.B (resolved open questions collapse)**: not addressed.

- **7.C + 7.D + 7.E + 3.C (F-section length sweep)**: not addressed.
  Could pick up the working-notes-pattern bullet's minor redundancy
  flagged in cold-reader 4.

- **1.A (F8 singleton-family acknowledgment)**: not addressed.

The deferred list shrank by one (group 1 done) and partial-credit on
group 2 (review-disposition mention added to F11 + F12 paragraphs;
deeper question still open). 7-8 groups remain for cycle 14+.

## Adversarial-on-adversarial pre-commits for cycle 14+

Following the cycle-N-pre-commits-cycle-N+1-checks discipline:

1. **Cold-reader on cycle-13 family-summary rewrite at lines 167-194.**
   Does the new framing (a) read coherently to a fresh reader, (b) avoid
   the freeze-vs-refresh-as-family-claim mis-elevation, (c) properly
   integrate the F12-as-catalog reframing, (d) state the
   sibling-not-upstream caveat with appropriate weight (not too
   apologetic, not too dismissive)? Specifically check whether moving
   the freeze-vs-refresh framing to the family-overlap paragraph (rather
   than the family-summary blockquote) loses any of the claim's
   architectural prominence — the cycle-7 framing was load-bearing for
   F11 specifically; cycle 13 explicitly demoted it from family-level
   to F11-local. Verify F11's local prominence is preserved.

2. **Cross-section consistency check.** The family preamble (lines
   167-194), F11 architectural-implication paragraph (lines 729-761),
   and F12 hypothesis substrate paragraph (lines 888-911) all use the
   new framing. Cold-read each in isolation and as a sequence to verify
   that:
   - the substrate-count claim is consistent (three substrates plus F12
     as cross-substrate catalog, in all three locations)
   - the local-mechanism inventory is consistent (F1: constraints-instead-of-tools;
     F5: write-mostly-state; F11: freeze-vs-refresh; F12:
     unbounded-accumulation-without-removal-tests)
   - the sibling-not-upstream caveat is stated in the family preamble
     and the F11 paragraph (it is intentionally not duplicated in the
     F12 hypothesis paragraph, which has its own load-bearingness
     qualification at lines 877-887)

3. **Family-table consistency.** Lines 138-152 — verify the table's
   "Core mechanism" parenthetical (line 140, post-cycle-13: "constraints,
   state fields, cycle-boundary checks, pipeline-checks") and the
   dual-membered F5/F11 footnote (lines 145-151) remain consistent with
   the new family-summary framing.

4. **Pick a Tier-2 group for cycle 14.** Strongest candidates:
   - **Group 3 (freeze-vs-refresh framing alternative, finding 3.B)** —
     the most consequential single finding from the cycle-11 dispatch,
     and now particularly tractable because cycle 13's family-summary
     rewrite has already moved freeze-vs-refresh to F11-local context.
     The remaining work is to either (a) make the case for
     "artifacts-freeze-too-early" over "defenses-run-too-late" explicit
     in the F11 architectural-implication paragraph, or (b) explicitly
     hold both framings open as alternatives and let Phase 2 candidates
     explore both. This work directly affects v2 candidate design and is
     high-leverage to settle before Phase 2.
   - **Group 5 (iteration plan move to README, findings 6.B + 7.A)** —
     mechanically bounded; addresses the highest-leverage structural
     finding from the cycle-11 dispatch (lens 6.B); resolves the
     stale-historical-quote flag from cycle-13 cold-reader 2; reduces
     the artifact's body size by ~44 lines; clarifies the
     deliverable-vs-working-document distinction.
   - **Group 2 (review/disposition substrate)** — interacts with the
     review-disposition surface mention added in cycle 13's edits; could
     deepen the partial integration into a full substrate-vs-F9-dual-membership
     decision.

   My read: group 3 is the highest-leverage *content* change; group 5 is
   the highest-leverage *structural* change. If cycle 14 has the compute
   for one substantive content rewrite, group 3. If cycle 14 wants a
   bounded mechanical cleanup, group 5. Either is defensible; the deepest
   finding still open (3.B) argues for group 3.

5. **Long-deferred items roll-call.** Journal-entry self-congratulation
   sweep (now 7 cycles deferred). F6/F8/F9 measurements (cycle 7+).
   Refactor-for-length F-section sweep (cycle 8+, also Tier-2 group 8).
   Working-notes-pattern minor redundancy flag from cycle-13 cold-reader
   4 (could bundle with length sweep). The deferred list is steady but
   not growing significantly — cycle 13 added ~1 deferred item (cold-
   reader 2 flag bundled with group 5; cold-reader 4 flag bundled with
   group 8) while resolving 5 (the four cold-readers + group 1).

## What surprised me

Three things.

(1) The family-summary rewrite turned out to require updating four
locations for cross-section consistency, not just the family-summary
itself. The pre-commit listed three (family preamble + F11 paragraph +
F12 hypothesis); the actual work needed a fourth (the v2-design-
implications section pointer at line 938-942) plus a fifth (the
family-table parenthetical at line 140) to hold consistent framing.
Tier-2 work involves cross-section coupling more than I expected.
Cycle 14+ Tier-2 work should explicitly check for ripple effects.

(2) The cold-reader checks on cycle-12's Tier-1 edits all passed with
only minor flags. Compare cycle 8's checks on cycle 7 (2 of 3 substantive
issues found) and cycle 9's checks on cycle 8 (~38 lines of duplication
the cycle-8 checks missed). Cycle 12's Tier-1 split appears to have
sized integration work better than cycle 7's all-at-once approach. This
is mild evidence that the Tier-1/Tier-2 discipline is working.

(3) The integrated "review-disposition surface" mention (partial 2.B
integration) emerged from working through edit 2 — it wasn't pre-planned.
F12 catalogs more than just the three substrates; pipeline-checks include
review-disposition checks. So while writing the F12 catalog description
in F11 paragraph, the review-disposition extension fell out naturally.
Lens 2.B (review/disposition as missing substrate / F9 dual-membership)
is now partially integrated rather than fully deferred — this is
incidental progress on group 2.

## What I'm still uncertain about

Whether the family-summary's parenthetical "(a constraint, a state
field, a cycle-boundary check, an additional pipeline-check)" is
correctly framed. "Pipeline-check" is what F12 catalogs (not what F11
substrate adds), and "cycle-boundary check" is what F11 substrate
adds. The list mixes the F11-substrate-defense and F12-catalog-example
in the parenthetical. Defensible because the family-summary is listing
*examples of what gets added* (and pipeline-checks DO get added), but
slightly mixes substrate vs catalog. Cycle 14+ cold-reader on the
family-summary should examine this.

Whether moving freeze-vs-refresh from family-summary to family-overlap
paragraph reduces its prominence too much. The cycle-7 framing was
load-bearing for F11 specifically and the cross-cutting cycle-524
abandonment cascade pattern (which freezes-vs-refreshes at the
abandonment stage). Demoting it to F11-local could under-state how
broadly the freeze-vs-refresh pattern shows up in v1. Counter: it
already lives in F11 paragraph (lines 729-740), F12 hypothesis (line
901), cross-family-notes blockquote (line 945-950), and now family-
overlap paragraph (lines 187-189). Four locations is more, not fewer.
The demotion is from "family-wide claim" to "F11 specifically + cross-
sectional appearance," which is more accurate.

Whether cycle 14+ should pick group 3 (freeze-vs-refresh alternative)
or group 5 (iteration plan move) first. Group 3 directly affects v2
candidate design and is the deepest finding still open; group 5 is
mechanically bounded and clears the deliverable/working-document
distinction. Both are defensible. My weak read: group 3, because the
content question is more important than the structural cleanup, and
because Phase 2 candidates will inherit whichever framing the artifact
commits to.

## Persistence-mechanism observations

The cycle-N-pre-commits-cycle-N+1-checks chain is now eight cycles deep
(cycles 7→8→9→10→11→12→13→14 pre-committed). Cycle 13 honors the chain:
all four cold-reader pre-commits ran; the Tier-2-group-selection
pre-commit was honored (group 1 chosen and executed).

The Tier-1/Tier-2 split introduced in cycle 12 has held through one
cycle of evaluation. Cycle 13's group-1 work was substantive (~25 lines
net change across four locations) but bounded — comparable to a single
F-section update, not the cycle-7-scale 296-line over-extension. The
Tier-1/Tier-2 pattern appears workable as the default for future
external-feedback integrations.

The "deferred list" tracking is becoming the persistence-mechanism's
weak point. It exists implicitly across notes files (cycle-7 evaluation
deferrals, cycle-12 evaluation deferrals, plus various cycle-N
"long-deferred items" notes) but is not consolidated into a single
referenceable view. Group 5 (iteration plan move) would partially
address this by consolidating the iteration log into README; a future
cycle should also consolidate the deferred-Tier-2-groups across the
notes files into a single "outstanding work" view in the README.
Flag for cycle 14+ as a persistence-mechanism enhancement.
