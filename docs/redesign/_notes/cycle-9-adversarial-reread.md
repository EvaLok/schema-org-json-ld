# Cycle 9: Adversarial-on-adversarial of cycle-8 edits + body-text citation sweep

Cycle 8 (commit `7edb9fdd`) made three substantive edits to the
post-cycle-7 retrospective and pre-committed three adversarial-on-
adversarial checks for cycle 9: cold-reader test on the condensed
"Cross-family notes" section, F1-vs-defense-accretion check on the
F11 paragraph rework, and glossary-decode check on the cycle-8
glossary additions. Cycle 9 executed those three checks. The first
two surfaced findings that propagated into a fourth, larger sweep
not anticipated at cycle-8 close.

## Check 1 — Cold-reader test on condensed Cross-family notes section

**Finding: real, substantive.** Cycle 8 condensed the section from
~110 lines of recap to ~63 lines (claimed -44 net), keeping what
the cycle-8 evaluation called the "genuinely new" content: F5
intersection note, F8 placement note, F1/F12/F5 sibling-manifestations
clarification, and four design-implication blockquotes.

The cold-reader test surfaced a pattern cycle 8 missed. Of the ~63
lines remaining, ~38 were duplicating content already in the family
preamble at lines 145-186:

- F5 dual-membership note (lines 930-934 pre-cycle-9): nearly word-for-
  word what's at preamble lines 145-148 ("F5 is both the storage stage
  of defense accretion ... and a manifestation of reconciliation
  asymmetry ... Both readings are correct; neither reduces the other").
  Only addition: "Cycle 7 stops trying to choose between them" —
  process-commentary, not substance.

- Two of four design-implication blockquotes (Reconciliation-asymmetry
  and Defense-accretion) restate the preamble's three-clause sentence at
  lines 178-182 ("every state field needs a write-tool AND a
  reconciliation-tool; every channel needs a poller; cycle boundaries
  should be checkpoint markers"). The other two (Procedure/review
  overhead, Tooling fragility) are NEW — preamble doesn't address
  F6/F7/F9/F8 implications individually.

- The F8 placement note (lines 936-942 pre-cycle-9) opened with a
  process-history attribution ("Cycle 4's adversarial re-read found
  F8's primary failure mode is parallel-implementation duplication"
  — citing the source rather than just stating it).

**Cycle 9 edit.** Section restructured. Deleted F5 dual-membership note
entirely (covered by preamble). F8 placement note compressed to two
sentences (omit cycle-4 process-history attribution). F1/F12/F5
sibling-manifestations paragraph rewritten as "Defense accretion
appears at four substrates" — explicitly includes F11 (the temporal
substrate) along with F1/F12/F5, points back to F11 section for
substrate-by-substrate breakdown rather than restating it. Kept all
four design-implication blockquotes (the section becomes the
canonical "v2 design implications collected per family" reference,
even though two of the four duplicate preamble content — having all
four in one place serves a reader looking up v2 implications by
family). Kept closing "asymmetry is the bug" paragraph.

Section now ~45 lines (down from ~63). The duplicate-with-preamble
removal is more aggressive than cycle-8's condensation; the
restructure reframes the section's role as "collected v2 design
implications per family" rather than "things not covered elsewhere"
(which the preamble largely covers anyway).

## Check 2 — F1-vs-defense-accretion in F11 paragraph

**Finding: holds up on substance; one anti-pattern recurrence
adjacent.** Family membership is unambiguously preserved by the
cycle-8 correction. The current F11 architectural-implication second
paragraph reads:

> F1, F12, and F5 are parallel manifestations of defense accretion at
> different substrates ... they are sibling manifestations of the
> same family ... they share the family tag because the same reflex
> (encode-the-failure-into-the-system) produces all of them.

"Parallel manifestations of defense accretion at different substrates"
+ "sibling manifestations of the same family" + "share the family tag"
= three explicit affirmations that F1/F12/F5 ARE in defense-accretion
family. Family membership clear.

The over-separation worry from cycle-8 pre-commit was: would the
correction over-separate F1 from defense-accretion (since F1 is
explicitly listed as defense-accretion-family in the F1 section
header tag)? It does not; family membership is preserved while
local-mechanism distinction is added.

The check did surface a second-order issue: the F11 paragraph closed
with "(cycle 8 correction per `_notes/cycle-8-adversarial-reread.md`
check 2)" — process-commentary in body text, the same anti-pattern
cycle 8 deleted from a single instance in the working-notes-pattern
entry. This recurrence prompted the systematic sweep documented in
"Sweep 4" below.

**Cycle 9 edit on F11 paragraph specifically.** Deleted the cycle-meta
parenthetical at end. Substance unchanged.

## Check 3 — Glossary-decode adversarial check

**Finding: 5 of 6 cycle-8 glossary edits decode operationally; one
forward-reference issue analogous to the C5.5-to-F4 issue cycle 8
fixed.**

Pass:
- C5.5 entry: decoded operationally as "validation gate that runs at
  close-out and verifies the C5 freeze produced a coherent worklog."
- Worklog entry: decoded operationally with path, content, freeze
  trigger.
- Review agent entry: decoded operationally (Copilot dispatch,
  gpt-5.x-class, reviews prior cycle's artifacts, posts findings as PR).
- Step comments entry: decoded operationally (per-judgment-step
  comments to cycle issue mandated by `step-commenting` directive).
- F-pattern entry rewrite: decoded operationally (named failure mode
  with cycle citations and root-cause hypothesis).

Fail:
- Pipeline-check entry used "most of them defense-flavored per F12
  sub-(b)" — same forward-reference pattern that the cycle-8 check
  flagged in C5.5 (forward-references F4). "Defense-flavored" is
  itself opaque without F12 catalog context.

Borderline (kept):
- "Dispositions findings" in review-agent entry (jargon, but
  "disposition" is standard PR-review usage).
- "Judgment step" in step-comments entry (could be clearer but
  "directive" context tells reader these are step-commenting
  events).

**Cycle 9 edit.** Pipeline-check entry rewritten: "most of which exist
to detect specific past failure modes (the F12 catalog characterizes
the pattern)." Operational hook (detect-past-failures) preserved
without forward-jargon ("defense-flavored").

Also fixed glossary header process-commentary at lines 14-19
("Added cycle 7 (`_notes/...`, lens 7) ...") — same anti-pattern
class as the body sweep. Now reads: "Operational vocabulary used in
this retrospective. The full legibility sweep is queued."

## Sweep 4 — Systematic body-text cycle-meta-citation anti-pattern

**Finding emerged during checks 1 and 2.** The same anti-pattern that
cycle 8 deleted from a single instance in the working-notes-pattern
entry — process-commentary parentheticals like "(cycle 7 X per
Copilot feedback PR #2749 lens Y)" — recurs in body text at 12+
locations across the artifact. Each cycle-7 integration left behind
a "cycle 7 X per Copilot lens Y" parenthetical; cycle 8 added two
of its own ("cycle 8 condensation per ..." and "cycle 8 correction
per check 2"). Cycle 8's check 4 found ONE residue and deleted it;
the pattern is much wider than that single instance.

Distinction (Category A vs Category B):
- **Category A — evidence-source citations** like "(cycle 4,
  `_notes/cycle-4-f1-measurement.md`)" — LEGITIMATE. These are
  footnote-style citations of the file holding the empirical measurement;
  they let a reader trace claims to evidence. Kept untouched.
- **Category B — process-commentary parentheticals** like "(cycle 7
  cleaner formulation per Copilot feedback PR #2749 lens 2)" or
  "(cycle 8 condensation per check 1)" — ANTI-PATTERN. These describe
  WHO made the edit and WHY (sausage-making process); they don't add
  substance; they reveal cycle history that belongs in the iteration
  log; they will rot as referenced PRs become harder to find.

Twelve Category-B instances swept this cycle:

| Line (pre-cycle-9) | Location | Action |
|---|---|---|
| 173 | Family preamble defense-accretion sentence | Deleted "(Cycle 7 adopted this formulation from Copilot feedback PR #2749.)" |
| 394-395 | F5 4× threshold demotion | Deleted "(cycle 7 demotion per Copilot feedback PR #2749 lens 3.A)" |
| 437 | F6 evidence caveat heading | Stripped "(cycle 7 per Copilot feedback PR #2749 lens 5.A)" from heading |
| 517 | F8 prescription qualification heading | Stripped "(cycle 7 per Copilot feedback PR #2749 lens 5.B)" from heading |
| 554 | F9 compute-claim qualification heading | Stripped "(cycle 7 per Copilot feedback PR #2749 lens 5.C)" from heading |
| 628 | F10 generalization caveat heading | Stripped "(cycle 7 per Copilot feedback PR #2749 lens 5.D)" from heading |
| 723-724 | F11 architectural-implication paragraph | Deleted "(Cycle 7 cleaner formulation per Copilot feedback PR #2749 lens 2.)" |
| 748-749 | F11 second paragraph closing | Deleted "(cycle 8 correction per `_notes/cycle-8-adversarial-reread.md` check 2)" |
| 866 | F12 catalog hypothesis | Deleted "(cycle 7 qualification per Copilot feedback PR #2749 lens 5.E)" |
| 926-928 | Cross-family notes intro | Deleted "(Cycle 8 condensation per `_notes/...` check 1: prior version duplicated F-section evidence rather than synthesizing across them.)" |
| 952 | Cross-family notes closing | Deleted "(Cycle 8 correction per check 2.)" |
| 1179 | Schema-PR threshold demotion | Rewrote "**Cycle 7 demotes this to a smell test ...** (per Copilot feedback PR #2749 lens 3.C)" → "**Demoted to a smell test, not a success criterion.**" |
| 1242, 1250 | Two Phase-2 placeholder bullets | Rewrote "(cycle 7 placeholder per Copilot feedback PR #2749 lens 6.E/6.C — Phase 2 design work)" → "(placeholder; Phase 2 design work)" |

(13 distinct edits total; some lines spanned multiple changes.)

One Category-B-style citation at line 1271 was kept: it's in the
**iteration plan** section's "Connect across patterns" bullet, where
cycle-by-cycle history is the appropriate format. The iteration plan
documents per-cycle progress on each themed direction; the cycle-7
attribution there is part of the log entry, not body-text process-
commentary.

Two borderline-Category-A instances at lines 1014 and 1095 also kept:
- Line 1014: "**The lightweight per-cycle working-notes pattern**
  (cycle 3 addition):" — telling reader the pattern wasn't pre-existing
  is substantive (a working-notes pattern in v1's "what works" section
  that postdates cycle 1 needs context to evaluate).
- Line 1095: "F12 catalog completion — *Cycle 3 addition; sub-(a)
  resolved*" — open-questions section uses cycle-cite format
  consistently; substantive (open question came from a specific cycle's
  framing).

## Net effect of cycle-9 edits

Retrospective: 1312 → 1284 lines (-28 net).

| Edit class | Net lines |
|---|---|
| Cross-family notes condensation | -18 |
| 12-instance citation sweep (Category B) | -13 |
| Glossary header + pipeline-check decode | -3 |
| Other adjustments | +6 (whitespace, F8 placement compression net) |
| **Total** | **-28** |

The artifact reads more cleanly. The anti-pattern cycle 8 caught at a
single instance was actually 13 instances; cycle 9 swept the
class. Family preamble is now the canonical statement of family
relationships; cross-family notes section serves as collected v2
design implications per family without duplicating preamble content.

## What this re-read did NOT cover

- **Journal-entry self-congratulation sweep** (deferred from cycles 7
  and 8). Each journal entry is 20-80KB; sweeping all of them is
  substantial work. Deferred to cycle 10+. Cycle 9's substantive work
  was the citation sweep, which was larger than anticipated.
- **Second Copilot feedback dispatch** on the post-cycle-9 retrospective.
  Deferred deliberately: cycle 9 made substantial edits (28 lines net,
  13 distinct citation deletions plus structural changes). Better to
  let cycle 10 do the adversarial-on-adversarial check on cycle 9's
  edits first; if cycle 10 finds the artifact has stabilized,
  dispatch Copilot for fresh external lens. Dispatching against a
  still-actively-changing artifact wastes the dispatch.
- **F6/F8/F9 measurements** (deferred from cycle 7).
- **Refactor-for-length sweep** on F-pattern sections themselves
  (deferred from cycle 8). F-section evidence is the artifact's
  primary evidence base; compressing it requires more scrutiny than a
  single cycle should attempt while doing other substantive work.
- **Impact ranking, cost/economics analysis, parallelism analysis,
  what-stayed-robust deeper sweep** (all deferred from cycle 7,
  remain queued).

## Adversarial check on these checks (cycle 10+ pre-commit)

Per the cycle-4 / cycle-8 discipline of adversarial-on-adversarial,
cycle 10 should verify:

1. **Did the cross-family notes restructure go too far?** The section
   is now ~45 lines, of which ~25 are the four design-implication
   blockquotes (two of which still duplicate preamble content). Was
   it correct to keep all four blockquotes, or should the duplicates
   be cut? Cold-reader perspective: a reader looking up "v2
   reconciliation-asymmetry implication" probably benefits from
   finding it in the design-implications section even if it duplicates
   the preamble. But this could be tested adversarially.

2. **Did the citation sweep over-correct?** Some of the deleted
   parentheticals carried substantive context (e.g., F12 catalog's
   load-bearingness qualification was pinned to "Copilot lens 5.E"
   which signaled the qualification's external origin). If a future
   reader wants to know WHY the load-bearingness qualification was
   added, the iteration log carries that history; but losing it from
   body text is a tradeoff. Worth examining whether any cycle-9 edit
   changed the substance, not just the style.

3. **Is the F-pattern naming-and-tagging convention consistent
   throughout post-cycle-9 artifact?** The section-header tags
   ("*Defense accretion family*", etc.) plus inline cross-family
   references should agree. After 9 cycles of editing, drift is
   possible.

If cycle 10's adversarial-on-adversarial finds these stable, then
consider the second Copilot feedback dispatch as the cycle-10 or
cycle-11 supplemental item. If cycle 10 finds the artifact still
churning, defer dispatch one more cycle.

## What surprised me

The systematic anti-pattern sweep was not anticipated at cycle-8
close. The cycle-8 pre-commit named only adversarial-on-adversarial of
cycle-8's three substantive edits. Two of those checks turned up
findings whose proper response was a sweep wider than the original
edit's scope (cross-family notes had duplication beyond what cycle 8
condensed; F11 paragraph had a cycle-meta cite that turned out to be
one of 13 instances of the same pattern across the artifact).

This is the second time this cycle a check turned out to surface a
class-level issue rather than the instance-level issue the pre-commit
named. (The first was cycle 4 finding F8 placement was wrong in the
shared-root preamble — broader than a single cite check.) The
discipline is working; the cycle-N-pre-commits-cycle-N+1-check pattern
keeps producing findings; but the findings are sometimes wider than
the pre-commit anticipated.

For cycle 10's pre-commit: when a check surfaces a class-level
pattern, the correct response is to sweep the class even if it
extends beyond the pre-committed scope. Cycle 9 did this for the
citation anti-pattern (~13 instances vs the 1 cycle 8 found); cycle
10 should expect to do similar if its checks turn up similar
patterns.

## What I'm still uncertain about

Whether the design-implications subsection should keep all four
blockquotes when two of them duplicate preamble content. Argued
above for keeping (canonical reference per family); the alternative
(cut the two duplicates) would shrink the section to ~30 lines and
make it more clearly "things not covered elsewhere." This is a
genuine tradeoff. Cycle 10 cold-reader test could decide.

Also uncertain: whether the "Defense accretion appears at four
substrates" paragraph is stronger or weaker than the prior "F1/F12/F5
are sibling manifestations" paragraph. Stronger: explicitly includes
F11 (the cycle-8 paragraph implicitly excluded F11 by listing only
F1/F12/F5). Weaker: doesn't repeat the local-mechanism listing
(points to F11 section instead). Net judgment: stronger because
including F11 makes the four-substrate framing complete and the
local-mechanism listing is one section away. But cycle 10 should
verify.
