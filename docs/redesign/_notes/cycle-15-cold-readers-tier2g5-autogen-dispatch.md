# Cycle 15: cycle-14 cold-readers + Tier-2 group 5 + AutoGen dispatch

Cycle 14 (commit `06e5fa9a`) ran three cold-readers on cycle-13 work plus
Phase 1 initiation (first-pass openclaw + PAI reads). Cycle 14 left six
pre-commits for cycle 15: three cold-reader checks on cycle-14 Phase 0
edits, an adversarial re-read of `1-research.md`, a dispatch decision,
and a Tier-2 group selection. Cycle 15 ran all six.

Three cold-readers PASS; the adversarial re-read found a real smuggling
issue and fixed it; the AutoGen dispatch executed via cycle-6 procedure
(issue #2762, gpt-5.5); Tier-2 group 5 (iteration plan move to README)
executed.

## Cold-reader checks on cycle-14 Phase 0 edits

### Pre-commit 1: cold-reader on family-summary edit at lines 169-171

Cycle 14 dropped "an additional pipeline-check" from the family-summary
parenthetical, leaving "(a constraint, a state field, or a cycle-boundary
check)" mapping to F1/F5/F11. The question: did this lose useful
catalog-signal for a fresh reader, or is the catalog-dimension still
adequately conveyed?

**Verdict: PASS.** The catalog-signal is preserved by the sibling
paragraph immediately following the blockquote (lines 173-176): "F1, F5,
F11, and F12 are sibling manifestations of this response pattern. F1
names the prompt/checklist substrate, F5 the state-shape substrate, F11
the temporal substrate; F12 catalogs the cross-substrate accumulation
rather than naming a fourth substrate of the same kind." A fresh reader
sees the substrate-list summary in the blockquote, then immediately
reads the sibling paragraph that frames F12 as the catalog level. The
blockquote's three-item parenthetical maps cleanly to F1/F5/F11; F12
gets its own treatment in the next paragraph. Level-of-abstraction
discipline is preserved.

**Optional cycle-16+ flag**: the introducer at line 167 ("The cleanest
single-sentence statement of the defense-accretion family is:") slightly
overclaims when followed by a blockquote that scopes to substrate-specific
defenses. The reconciliation-asymmetry parallel uses "the dominant
family" (line 162) — different framing. The mismatch is bounded; the
blockquote's explicit "substrate-specific" qualifier scopes the claim
inside the family. Could be tightened to "The cleanest single-sentence
statement of the substrate-defense pattern within this family is:" but
not load-bearing.

### Pre-commit 2: cold-reader on family-table reorder

Cycle 14 reordered the defense-accretion row from "F1, F5†, F12, F11†" to
"F1, F5†, F11†, F12" (numerical order matching the reconciliation-
asymmetry row's strict numerical order). Question: does F12-at-the-end
read as "F12 is least important" rather than "F12 is the catalog-across"?

**Verdict: PASS.** F12-at-end is structurally consistent with F12-as-
catalog-across-substrates. The substrate members F1/F5/F11 lead the row;
F12 (the catalog level) trails. This reads as "substrates first, catalog
second" — appropriate per the family preamble's framing of F12 as
catalog rather than peer substrate. The numerical order convention is
the more important constraint than the structural-meaning question; both
rows now follow numerical order.

**Optional cycle-16+ flag**: a fresh reader unfamiliar with the family
preamble's F12-as-catalog framing might initially read F12-trailing as
de-emphasis. The first-line family preamble is the load-bearing
explanation; the table is for navigation, not standalone explanation.

### Pre-commit 3: cross-row sibling-pattern check

Question: are the procedure/review-overhead row ("F6, F7, F9") and the
tooling-fragility row ("F8") also in numerical order, making the
convention implicit and uniform?

**Verdict: PASS.** Procedure/review-overhead is numerical (F6, F7, F9 —
F8 is not in this row because F8 is in tooling-fragility). Tooling-
fragility is a singleton (F8); no order applies. All four rows are
numerical-order-consistent.

**Note: F8 singleton-family taxonomy smell** is a Tier-2 finding (1.A
from the cycle-11 dispatch evaluation; cycle 12 Tier-2 deferral). A
singleton family is itself a taxonomy smell that should be honestly
acknowledged. Cycle 12's evaluation flagged this. Cycle 13's notes
suggested 1.A could be a small Tier-1-style edit. Cycle 14 did not pick
it up. Cycle 15 declined as well (out-of-scope for the row-consistency
check). Carry forward.

## Pre-commit 4: adversarial re-read of `1-research.md`

Three sub-questions:

(a) Does the anchoring-discipline section (lines 32-61) actually
constrain the findings, or are the caveats cosmetic?

(b) Is the "Provisional patterns to track" framing rigorous, or does it
smuggle pattern-import disguised as observation?

(c) Does the cycle plan over-commit to a specific dispatch sequence?

### Sub-question (a): anchoring discipline genuinely constrains

The per-system "Anchoring caveats" sections are robust:
- openclaw caveats (5 specific reasons): single-user personal assistant,
  has human-in-the-loop, TypeScript-by-design, anomalous 365k★ scale,
  partly tongue-in-cheek (lobster theming)
- PAI caveats (5 specific reasons): single-user personal assistant,
  different "memory" shape (user history vs orchestrator state),
  TypeScript/Bun-based, principles-vs-codebase reality unverified,
  README is marketing surface

These caveats explicitly argue why patterns may not generalize. They are
not cosmetic.

**PASS on (a).**

### Sub-question (b): smuggling found

The per-system "Provisional patterns to track" sections (one each for
openclaw and PAI) listed patterns with v2-relevance framings:
- "Small-core / plugin-extensible architecture **as a redesign-relevant
  shape** (aligns with our CORE-DESIGN-PRINCIPLE...)"
- "'What We Will Not Merge' **as a guardrail mechanism in the
  deliverable itself**"
- "Memory-as-singleton-plugin-slot **as a way to constrain persistence
  mechanism complexity**"
- "Strong-defaults-with-operator-knobs **as a security posture pattern**"

(And similar for PAI.)

The framings ("as a redesign-relevant shape", "as a guardrail mechanism",
"as a way to constrain") are not pure observations — they evaluate the
pattern's relevance to v2. This is exactly the confirmation-bias-on-
aligned-principles failure mode flagged in the anchoring discipline
section's failure mode #1.

The caveats sections argue against generalization in the abstract; the
provisional-patterns sections evaluate the patterns AS RELEVANT to v2.
The two sections live in tension. The smuggling is real.

**Edit applied:** renamed both per-system sections to "Patterns observed
in [system] (relevance evaluation deferred to cross-system synthesis,
gated on multi-system reading)" and stripped the v2-relevance framings.
Pattern names remain; evaluation moves to a future section that's
explicitly gated on multi-system reading.

The cross-system observations section (lines 220-240) was already clean
(no v2-relevance framings; explicit deferral via "These shouldn't yet
inform Phase 2 candidates"). No edits there.

**FINDING + FIX on (b).**

### Sub-question (c): cycle plan over-commitment

The cycle plan (lines 268-286) lists three dispatch options with
"priority order" framing, then caveats with "The dispatch sequence is
tentative." The caveats are explicit; the framing is provisional.

But: AutoGen-as-#1 is named without strong justification. Voyager (long-
running self-improvement; skill library accumulation — directly relevant
to v2's claim) is at #2. The priority might be inverted given Voyager's
direct relevance to "self-improving system" claims.

Cycle-15 decision: dispatch AutoGen via Copilot research-only THIS
CYCLE (consistent with cycle plan); do orchestrator-direct read of
Voyager paper in cycle 16+ (lighter-weight, paper is short). The two
tracks run in parallel; priority ordering is less load-bearing than
just executing both.

**Mild flag on (c)** — the cycle plan's priority list could be reframed
as "candidate dispatches available, pick by cycle's actual capacity"
rather than "priority order #1, #2, #3." Carry forward to cycle 16+ as
a structural cleanup if time permits.

## Pre-commit 5: AutoGen Copilot research-only dispatch

Executed. Issue [#2762](https://github.com/EvaLok/schema-org-json-ld/issues/2762)
opened at 2026-04-28T17:14Z via cycle-6 procedure (`jq -Rs | gh api
... --method POST --input -` with `agent_assignment.model: gpt-5.5`).

Dispatch metadata:
| Field | Value |
|---|---|
| Dispatched at | 2026-04-28T17:14:49Z |
| Issue number | #2762 |
| Title | `[redesign-research] AutoGen architecture survey for Phase 1 cross-system reading (cycle 15)` |
| Labels | `agent-task`, `research-only` |
| Assignees | `Copilot` (primary), `EvaLok` (added by gh api as creator) |
| Model | `gpt-5.5` (cycle-6 procedure propagates `agent_assignment.model`) |
| Target deliverable | `docs/redesign/_notes/cycle-15-autogen-research.md` (single file, single PR) |
| PR number | TBD (cycle 16+ should see it) |
| Integration cycle | TBD |

The `research-only` label was created this cycle (didn't exist; cycle 6
used `feedback-only` for a different dispatch type). Color `C5DEF5`,
description "Research-only Copilot dispatch (read sources and report
findings; do not modify any other file)."

Seven lenses:
1. Overall architecture and named primitives (ConversableAgent, GroupChat)
2. Multi-turn conversation and state representation
3. Orchestration / planning patterns (peer-chat vs hierarchical)
4. Failure handling and recovery
5. Tool / skill integration model + trust boundary
6. Anti-patterns and explicit non-goals
7. Anchoring caveats specific to AutoGen vs the redesign context

The dispatch body explicitly enumerates four anchoring differences
(library vs autonomous orchestrator, human-in-the-loop vs minimal-
intervention, Python vs Rust, short tasks vs multi-cycle work) and
asks Copilot to add specifics. This pre-loads the smuggling-prevention
discipline cycle 15 just learned to apply on `1-research.md`.

## Pre-commit 6: Tier-2 group selection — group 5 executed

Cycle 14 deferred Tier-2 group selection to cycle 15. Cycle-14 framing:
"if cycle 15 also runs the Phase 1 dispatch, group 5 may be the right
pick to keep cycle 15's load manageable; if cycle 15 defers the
dispatch, group 3 becomes the natural pick."

Cycle 15 ran the dispatch (above). Per cycle-14 framing, group 5 is
the appropriate pick. Group 5 is mechanically bounded (iteration plan
move from retrospective body to README) and resolves the cycle-13
cold-reader 2 flag (iteration-plan section had a historical "guarantees"
quote at line 1287-1289 that the cycle-12 Tier-1 1.C edit had updated
elsewhere). Group 3 (freeze-vs-refresh framing alternative — the deepest
cycle-11 dispatch finding still open) carried forward to cycle 16+.

### Group 5 execution

Removed from `0-retrospective.md`: the `## Iteration plan for this
artifact` section (lines 1253-1298, including the `---` separator).
Total: 46 lines.

Added to `docs/redesign/README.md` (above the existing iteration log
section): a new `## Iteration discipline for `0-retrospective.md``
section with the seven discipline bullets. The new section is roughly
the same shape as the original but with these adjustments:

1. **Connect-across-patterns bullet trimmed.** Original had a long
   verbatim quote of cycle 7's freeze-vs-refresh framing including the
   pre-cycle-12 "guarantees" wording. New version keeps the
   cross-pattern discipline rule but drops the verbatim quote, replacing
   with a brief reference: "(the freeze-vs-refresh framing — see family
   preamble for current wording)." This addresses cycle-13's cold-reader
   2 flag (historical "guarantees" quote was inconsistent with cycle-12
   Tier-1 1.C demotion to "structurally produces") without falsifying
   the historical record (the iteration log table preserves cycle 7's
   original framing).

2. **Demote-what-doesn't-survive bullet expanded.** Added the cycle-12
   "guarantees → structurally produces" demotion as an example of the
   discipline being applied. This is the cycle-13-flagged
   reconciliation between historical wording and current state.

3. **Solicit-critique bullet updated.** Removed the cycle-3/4/5 deferral
   history (now historical) and replaced with current state: cycle 6
   and 11 both dispatched (PRs #2749 and #2756 landed and integrated).

4. **Quantify bullet trimmed.** Removed the specific OQ-resolution
   chronology — it's stale (cycle 5's snapshot of which OQs are
   resolved). The discipline rule is what's load-bearing; specific
   chronology lives in the iteration log table.

5. **Added "audit cycle 204+ — not yet landed as of cycle 15"** to the
   reconcile-against-audit bullet to update the date reference.

The retrospective now ends at line 1251 with the v2 deferred-measures
bullet ("Cycle 7 names them here so the gap is visible to Eva-review and
to candidate-design authors"). This is a defensible structural close —
the deliverable ends with "here's what's still queued for future work."
The iteration discipline (process of how to iterate) lives in README;
the iteration log (per-cycle history) lives in README. The deliverable
body is now pure failure-pattern content.

Net retrospective change: 1298 → 1251 lines (-47 lines).
Net README change: 64 → 107 lines (+43 lines).
Cross-file balance: -4 net (the trimming of the connect-across-patterns
verbatim quote and OQ-chronology accounts for the difference).

## Resolved cycle-13 cold-reader 2 flag

Per cycle-13's notes file, cold-reader 2 had flagged: "the iteration-plan
section at line 1287-1289 still has a historical 'guarantees' quote
(bundle with Tier-2 group 5 when iteration plan moves)." Cycle 15
resolved this by:
- Moving the iteration plan section to README (above)
- Trimming the verbatim quote in the migrated bullet to a brief
  reference, preserving the cross-pattern discipline rule
- Adding the cycle-12 demotion as an example in the demote-bullet

The historical "guarantees" wording is preserved in the iteration log
table (cycle 7 entry references cycle 7's adoption of Copilot lens 2
with original wording).

## Adversarial-on-adversarial pre-commits for cycle 16+

Following the cycle-N-pre-commits-cycle-N+1-checks chain (now ten cycles
deep — cycle 7 → 8 → 9 → 10 → 11 → 12 → 13 → 14 → 15 → 16 pre-committed):

### On cycle-15 Phase 0 work (Tier-2 group 5)

1. **Cold-reader on the migrated iteration discipline section in
   README.** Did the trimming of the connect-across-patterns verbatim
   quote lose substantive context? Specifically: cycle 7's original
   adoption framing was load-bearing ("v1 defenses are end-of-cycle/
   next-cycle refreshers; artifacts freeze before refreshers finish") —
   the brief reference "(the freeze-vs-refresh framing — see family
   preamble for current wording)" might under-signal what the framing
   actually is. Cold-read in conjunction with the family preamble at
   lines 167-194 to verify the discipline section + family preamble
   together carry the full claim.

2. **Cold-reader on retrospective new ending.** The retrospective now
   ends at line 1251 with the v2 deferred-measures bullet. Read the
   final ~30 lines (1220-1251) as a fresh reader to verify the
   structural close reads coherently. Specifically: does the deferred-
   measures bullet feel like an ending, or like the document was cut
   off mid-thought? If it feels cut off, consider adding a brief
   closing sentence or pointer.

### On cycle-15 Phase 1 work

3. **Cold-reader on the smuggling-fix in `1-research.md`.** The
   "Patterns observed in [system]" rename + framings-stripped edit
   addresses confirmation-bias-on-aligned-principles. Cold-read the
   resulting per-system sections to verify they no longer evaluate
   patterns as v2-relevant — and equally, that they did not lose
   substantive observation in the trimming. Some of the framings
   ("as a guardrail mechanism in the deliverable itself") carried
   description of what the pattern DOES, not just what it is. Verify
   the pure-observation framings retain the descriptive content.

4. **AutoGen dispatch status check.** If PR has landed (~24h after
   2026-04-28T17:14Z, so probably cycle 16-17): cycle-7-style per-
   finding evaluation, integration plan with Tier-1/Tier-2 split. If
   not landed, defer integration; AutoGen dispatch is research-only
   not feedback-only, so the integration shape is different — it goes
   into `1-research.md` as a new system entry, not into
   `0-retrospective.md`. Different evaluation discipline applies.

### On cycle-15 substrate-specific items

5. **Voyager paper read (orchestrator-direct).** Cycle 14's cycle plan
   listed Voyager as #2 priority for cycle 15+; cycle 15 chose AutoGen
   for dispatch (consistent with cycle plan #1). Voyager is short and
   directly relevant to v2's self-improvement claim. Cycle 16 should
   either read it orchestrator-direct (paper is short; can fit in one
   cycle), or dispatch a research-only Copilot session if the time isn't
   there. Voyager's absence from `1-research.md` is the largest single
   gap relative to the prompt's named-system list.

6. **Tier-2 group selection.** Group 3 (freeze-vs-refresh framing
   alternative — deepest cycle-11 dispatch finding still open) is the
   highest-leverage Tier-2 item remaining. Cycle 16 should either pick
   group 3, or pick a different group with rationale. If the AutoGen
   PR has landed and integration is the cycle's focus, defer group 3
   one more cycle. If no AutoGen PR yet, group 3 is the right pick.

### Long-deferred items roll-call (carried forward)

- Journal-entry self-congratulation sweep (now 9 cycles deferred)
- F6/F8/F9 measurements (cycle 7+)
- Refactor-for-length F-section sweep (cycle 8+; Tier-2 group 8)
- Persistence-mechanism deferred-list consolidation (cycle 13 flag)
- Tier-2 group 2 (review/disposition substrate, partial integration in
  cycle 13)
- Tier-2 group 3 (freeze-vs-refresh framing alternative — deepest
  cycle-11 finding still open)
- Tier-2 group 4 (nine measures rework)
- Tier-2 group 6 (preserved-through-cutover disposition)
- Tier-2 group 7 (resolved open questions collapse)
- Tier-2 group 9 (F8 singleton-family acknowledgment — small edit,
  cycle 13 noted promotion candidate)

## What surprised me

Three things.

**(1) The smuggling in `1-research.md` was real and structural, not
cosmetic.** The anchoring discipline section explicitly named the
confirmation-bias failure mode and the per-system anchoring caveats
sections were robust. Yet the per-system "Provisional patterns to
track" sections still smuggled v2-relevance framings into observation-
shaped bullets. The discipline was performing its check at one level
(per-pattern caveats) but failing at another level (the framings on
the pattern names themselves). This is the same shape as several v1
F-patterns: a check exists for the failure but at the wrong granularity
to actually catch the failure. In this case the cycle-15 adversarial
re-read caught it at one cycle's distance — exactly the cycle-N-
pre-commits-cycle-N+1-checks discipline working as designed. But it's
a useful negative observation: even with explicit anchoring discipline
documented in a section header, smuggling can occur in adjacent
content if the discipline doesn't propagate to the framings of the
pattern names themselves.

**(2) The iteration plan move was easier than expected.** Cycle-12's
evaluation flagged lens 6.B (iteration plan as process-commentary in
deliverable body) as the highest-leverage structural finding. I had
expected the migration to require careful re-shaping of the bullets to
fit the README's voice. In practice, most of the bullets translated
near-verbatim; the structural change was just relocation. The cycle-13
cold-reader 2 flag (historical "guarantees" quote) was the only
substantive adjustment needed during the migration. The lesson: some
"structural change" findings are mostly relocation, not rewriting.
Lens 6.B was high-leverage because the deliverable surface is
significantly cleaner — the retrospective now ends on substantive
content (v2 deferred-measures bullet) rather than process-meta — but
the work was bounded.

**(3) The dispatch worked using cycle-6 procedure exactly.** Cycle 11
deviated from cycle-6 procedure (`gh issue create` instead of `gh api
... --method POST` with `agent_assignment` field) and the deviation
caused the model field to fail propagation (cycle 11's `gpt-5.5`
request resulted in a `claude-sonnet-4.6` actual model). Cycle 15
used cycle-6 procedure and `gpt-5.5` propagated correctly (the
returned issue JSON shows the assignment metadata). This validates
the cycle-12 evaluation's recommendation to canonicalize the cycle-6
procedure for model-controlled dispatches. v2 dispatch tool design
should keep `agent_assignment.model` as a first-class field with
explicit verification that the requested model actually got
propagated.

## What I'm still uncertain about

Whether the iteration discipline section's connect-across-patterns
bullet's brief reference to "the freeze-vs-refresh framing" carries
enough signal for a fresh reader. The original bullet quoted cycle 7's
verbatim framing; the migrated bullet says "see family preamble for
current wording." A reader who hasn't yet read the family preamble may
not know what the framing IS. Cycle-16 cold-reader test (pre-commit 1)
will examine.

Whether the retrospective's new ending at line 1251 reads as a
deliberate structural close or as a cut-off. The deferred-measures
bullet is content but ends with "Cycle 7 names them here" — past-tense,
historical-pointing. Cycle-16 cold-reader (pre-commit 2) will check
whether the ending feels OK.

Whether the AutoGen dispatch will produce a substantive PR. Cycle 6's
dispatch produced a high-quality 288-line critique. Cycle 11's dispatch
produced a high-quality 439-line critique despite the model-mismatch.
Cycle 15's dispatch is a different shape (research-only not
feedback-only); the deliverable file is `cycle-15-autogen-research.md`
(new file) not a critique of an existing artifact. Whether the PR's
output is high-quality at the same level depends on AutoGen having
substantive content to report and Copilot's research-mode being
calibrated. Cycle 16-17 will see.

Whether deferring Voyager to cycle 16 is right. Voyager is the most
directly-relevant-to-v2-claim system on the list (long-running self-
improving agent with skill-library accumulation). Cycle 14's cycle plan
ranked it #2; cycle 15 picked AutoGen for dispatch (#1 per cycle plan)
and deferred Voyager. The two tracks could run in parallel; Voyager
takes minutes to read (paper is short). Cycle 15 declined to do both
to keep cycle scope bounded. Cycle 16 should pick up Voyager.

## Persistence-mechanism observations

The cycle-N-pre-commits-cycle-N+1-checks chain is now ten cycles deep
(cycle 7 → 8 → 9 → 10 → 11 → 12 → 13 → 14 → 15 → 16 pre-committed).
No breakdown. The chain has now caught issues at cycle-N-2 (cycle 14
caught a cycle-11 issue), cycle-N-3 (cycle 9 caught a cycle-6 lineage
issue indirectly), and adjacent-cycle. Multi-cycle scope on these
checks is producing value beyond the immediate-cycle scope.

The Phase-0/Phase-1 split is now bedded in. Two parallel artifacts
(`0-retrospective.md`, `1-research.md`) with cycle-N notes files
spanning both. The cycle-15 work touched both: pre-commits 1-3 and 6
(Tier-2 group 5) on Phase 0; pre-commits 4 and 5 on Phase 1. The
combined cycle scope is bounded — six pre-commits + dispatch + Tier-2
in one cycle is at the upper bound of what fits.

The "deferred list" tracking weakness flagged in cycles 13 and 14 is
partially addressed by Tier-2 group 5 execution (the iteration log
table is now the canonical location for cycle history). But the
"long-deferred items" list (now 10 items including 6 Tier-2 groups +
4 long-deferred non-Tier-2 items) is still distributed across cycle-N
notes files. Future cycles should consider promoting the deferred-list
to a top-level section in README, similar to how iteration log lives
there. Tier-2 group 7 (resolved open questions collapse) might be
related — when OQs collapse, they migrate to "resolved" in some
artifact location; the same shape works for Tier-2 deferral
consolidation.

The cycle-15 notes file uses the cycle-7+ convention. The combined
"cold-readers + Tier-2 + dispatch" notes file (this file) is larger
than typical (probably ~250 lines once formatted) — combines what
would have been three smaller files. The combination is appropriate
when the cycle's work shares thematic links (the Tier-2 group selection
was cycle-14-pre-committed; the dispatch was cycle-14-pre-committed;
the cold-readers were cycle-14-pre-committed) and the file isn't so
large that future-cycle navigation becomes hard. Cycle 7's combined
notes file (`cycle-7-copilot-feedback-evaluation.md`) was longer at
~350 lines and worked. This is consistent.

## Reconciling cycle-14 and cycle-15 scopes

Cycle 14 had: 3 cold-readers + Phase 1 initiation (`1-research.md`
draft + 2 system reads). Roughly equivalent scope to cycle 15.

Cycle 15 had: 3 cold-readers + 1 adversarial re-read (with edit) + 1
dispatch + 1 Tier-2 group execution. Cycle 15 is slightly more diverse
in deliverable types but no single deliverable was the cycle's primary
work.

Both cycles fit within the ~75-min compute envelope. The cycle-7
over-extension lesson (37 findings integrated in one cycle) is held to
— cycle 15 did NOT attempt to integrate AutoGen findings (AutoGen
dispatch is just-issued, the PR has not landed). Future Tier-2 groups
plus AutoGen integration plus Voyager read plus group-3 freeze-vs-
refresh rework are the highest-leverage cycle-16+ candidate work; spread
across multiple cycles per the Tier-1/Tier-2 discipline.
