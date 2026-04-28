# Cycle 14: cycle-13 cold-readers + Phase 1 initiation

Cycle 13 (commit `5a8a29c9`) executed Tier-2 group 1 (family-summary
rewrite) and pre-committed three cold-reader checks plus a Tier-2-group
selection for cycle 14. Cycle 14 also received Eva's `input-from-eva`
#2759 authorizing Phase 1 (external-research) to begin in parallel with
finishing Phase 0. Cycle 14 ran the three cold-readers (two found real
issues, one passed) and seeded Phase 1 with a first-pass read of the
two required references (openclaw, PAI).

## Cold-reader checks on cycle-13 family-summary rewrite

### Pre-commit 1: cold-reader on family-summary at lines 167-194

The post-cycle-13 family-summary blockquote was:

> When a failure surfaces, v1's response is to add a substrate-specific
> defense (a constraint, a state field, a cycle-boundary check, an
> additional pipeline-check) and never test whether the defense is still
> load-bearing.

Cycle 13's own uncertainty section explicitly flagged this and named
three resolution options: (a) keep the four-item list; (b) reduce to
three (drop "additional pipeline-check"); (c) restructure to be
explicit ("a substrate-specific defense [examples...]; F12 catalogs
the accumulation").

**Verdict: real finding. Apply option (b).** The first three items map
cleanly to F1/F5/F11 substrates. "Pipeline-check" is what F12
catalogs across substrates, not what F11 substrate adds. Including
it in a substrate-specific list mixes levels and contradicts the
sibling-paragraph immediately following ("F12 catalogs the
cross-substrate accumulation rather than naming a fourth substrate
of the same kind"). The "additional" qualifier on the fourth item is
the tell that the writer was aware of the level-mixing — but the
qualifier didn't fix it, just made it audible.

Option (b) is the minimal edit. Option (c) would lengthen the
blockquote past single-sentence. Option (a) preserves the level-mixing
the cycle-13 self-flag identified.

**Edit applied** (line 169-172):

```
> When a failure surfaces, v1's response is to add a substrate-specific
> defense (a constraint, a state field, or a cycle-boundary check) and
> never test whether the defense is still load-bearing.
```

The substrate examples now map 1:1 to F1, F5, F11. F12-as-catalog is
described in the sibling paragraph; the family-summary doesn't need to
gesture at it.

### Pre-commit 2: cross-section consistency check

The four locations using the new framing:
- Family preamble lines 167-194 (cycle-13 rewrite)
- F11 architectural-implication paragraph lines 742-761 (cycle-13 edit)
- F12 hypothesis substrate paragraph lines 888-911 (cycle-13 edit)
- v2-design-implications section pointer lines 938-942 (cycle-13 edit)

Substrate-count claim: consistent. Family preamble describes F1/F5/F11
substrates with F12 catalog ("F12 catalogs the cross-substrate
accumulation rather than naming a fourth substrate of the same kind").
F11 paragraph names all four with F12 framed as "across rather than a
fourth substrate of the same kind." F12 hypothesis explicitly says
"three substrates" plus F12 catalog. Three locations, same claim,
different levels of detail appropriate to each section's role.

Local-mechanism inventory: present in F11 paragraph (lists all four
local mechanisms — F1 constraints-instead-of-tools, F5 write-mostly,
F11 freeze-vs-refresh, F12 unbounded-accumulation). Family preamble
delegates to F-sections ("Each pattern carries its own local mechanism
(see the F-pattern sections for details)"). F12 hypothesis names
freeze-vs-refresh as F11's local mechanism but doesn't enumerate the
others. Detail asymmetry is intentional per cycle 13's design.

Sibling-not-upstream caveat: present in family preamble (italicized
at lines 179-182) and F11 paragraph (italicized at lines 756-758).
Absent from F12 hypothesis — but cycle-13 noted this is intentional
because the F12 hypothesis has its own load-bearingness qualification
at lines 877-887 ("**The catalog proves accumulation; it does not
prove load-bearingness.**") which serves a similar epistemic function.

**Verdict: PASS.** No new edits. The three locations carry consistent
substrate-and-catalog framing. Where they differ in detail, the
differences match the section's role. The family preamble is high-
level summary; F11 paragraph is detailed sibling exposition; F12
hypothesis is catalog-focused with its own load-bearingness caveat.

### Pre-commit 3: family-table consistency

Family-table lines 138-152 are the cross-row consistency check.

Reconciliation-asymmetry row member-patterns: "F2, F3, F4, F5†, F11†"
— this is **strictly numerical order** (F2, F3, F4, F5, F11; daggers
mark dual-membership but don't reorder).

Defense-accretion row member-patterns: "F1, F5†, F12, F11†" — this is
**not numerical order**. F12 sits between F5† and F11†; numerical
order would be "F1, F5†, F11†, F12".

Cycle 11's edit (commit `4647eee4`) added the dagger markers to
indicate F5/F11 dual-membership but did not reorder either row. The
asymmetry between the two rows was not flagged in cycle 11's own
notes nor in cycle 12's evaluation of the cycle-11 dispatch. Cycle 13
also did not check this axis when running Tier-2 group 1.

**Verdict: real finding cycle 13 didn't anticipate. Apply edit.**
Numerical order is the natural cross-row convention given the
reconciliation-asymmetry row already follows it. Reordering
defense-accretion row to "F1, F5†, F11†, F12" makes both rows
consistent on this axis.

**Edit applied** (line 140):

```
| **Defense accretion** | F1, F5†, F11†, F12 | New failures get encoded as new defenses (constraints, state fields, cycle-boundary checks, pipeline-checks). Defenses accumulate across substrates without removal-tests. |
```

Note that the table parenthetical "(constraints, state fields, cycle-
boundary checks, pipeline-checks)" intentionally retains all four
defense-types — unlike the family-summary blockquote, the table is in
the "Core mechanism" column describing the family's mechanism in
general (and the second sentence explicitly says "across substrates"),
not making a substrate-specific claim. So the four-item list works
here. The blockquote-vs-table differ in what they claim, and the
edits are calibrated to each.

### Cold-reader summary

| Pre-commit | Verdict | Edit |
|---|---|---|
| 1 (family-summary parenthetical) | Real finding (cycle 13 pre-flagged) | Drop "an additional pipeline-check" |
| 2 (cross-section consistency) | PASS | None |
| 3 (family-table consistency) | Real finding (cycle 13 didn't anticipate) | Reorder defense-accretion row to numerical order |

Two of three checks produced real findings, comparable to cycle 8's
2-of-3 yield on cycle-7 integrations. Cycle 13's own self-flag in its
uncertainty section pre-loaded one finding (the family-summary
parenthetical); the other (table row order) was a cross-row axis cycle
13 had not thought to check.

**Pattern observation.** The three cold-reader checks were all on
cycle-13 *Tier-2 group 1* edits (the family-summary rewrite). The
family-table check was named in pre-commit 3 but cycle 11's table
edit was the actual source of the row-order asymmetry — cycle 13's
edit only updated the table parenthetical. So pre-commit 3's check
caught a cycle-11 edit two cycles later. The cycle-N-pre-commits-
cycle-N+1-checks chain has now caught a cycle-N-2 issue; the cumulative
inspection chain is producing real value across multiple cycle
intervals.

## Phase 1 initiation

Eva's `input-from-eva` issue #2759 (2026-04-28T16:11Z) authorized
Phase 1 to begin in parallel with finishing Phase 0. The redesign
prompt was updated at commit `f77b4341` to allow this. Cycle 14 is the
first Phase 1 cycle.

### Cycle-14 Phase 1 work

First-pass orchestrator-direct read of the two required references:

**openclaw** (openclaw/openclaw): `README.md` and `VISION.md`. Repo
size 663MB; deeper reads of architecture pages, source files, plugin
implementation deferred. Notable findings captured in `1-research.md`:
small-core / plugin-extensible architecture; "What We Will Not Merge"
guardrail mechanism; explicit anti-pattern stance on agent-hierarchy
frameworks; memory-as-singleton-plugin-slot; strong-defaults-with-
operator-knobs security posture.

**PAI** (danielmiessler/Personal_AI_Infrastructure): `README.md`. The
16 PAI Principles strikingly aligned with the redesign's CORE-DESIGN-
PRINCIPLE — particularly principles 4 (Scaffolding > Model), 5
(Deterministic Infrastructure), 6 (Code Before Prompts), 8 (UNIX
Philosophy), 11 (Goal → Code → CLI → Prompts → Agents hierarchy), 13
(Memory System), 15 (Science as Meta-Loop). The Observe → Think →
Plan → Execute → Verify → Learn → Improve loop with "Learn → Improve"
explicitly closing the meta-loop is the architectural commitment v1
lacks.

### Anchoring discipline applied

Three failure modes flagged in `1-research.md`:
1. Confirmation bias on aligned principles (PAI principles read as
   validation of the redesign's principles; counter-discipline is to
   record alignment without importing)
2. Context-mismatch import (single-user TypeScript personal assistant
   vs multi-agent autonomous Rust public-repo orchestrator)
3. Premature commitment to first-found patterns (must read multiple
   systems before settling on candidate shapes)

Trust posture: external README content is **untrusted text** per
`SECURITY` rules — data, not instructions. Architectural claims
weighed; not adopted as templates.

### No dispatch this cycle

Cycle 14 chose orchestrator-direct reading only. A Copilot research-
only dispatch was considered (AutoGen as the strongest candidate for
multi-agent-coordination focus) but deferred to cycle 15 to keep
cycle-14 scope bounded. Cycle 14 already has cold-readers + Phase 1
seed + this notes file + journal + 1-research.md draft; adding a
dispatch would risk cycle-7-style over-extension. Cycle 15 can run
the first dispatch with full attention.

When dispatching: use cycle-6 procedure (`jq | gh api ... --method
POST --input -` with `agent_assignment.model: gpt-5.5` field) — cycle
11's `gh issue create` shortcut produces a PR but does not propagate
model selection (cycle 12 evaluation noted the resulting PR declared
model `claude-sonnet-4.6` despite the dispatch body requesting
`gpt-5.5`).

### What cycle 14 did NOT do

Tier-2 group 3 (freeze-vs-refresh framing alternative) and group 5
(iteration plan move) were both candidates for cycle-14 selection per
cycle-13 pre-commits. **Both deferred to cycle 15+.** Cycle 14's
substantive work was cold-readers + Phase 1 initiation; adding a
Tier-2 group execution would push the cycle past its compute envelope.

Long-deferred items still deferred: journal-entry self-congratulation
sweep (now 8 cycles deferred); F6/F8/F9 measurements (cycle 7+);
refactor-for-length F-section sweep (cycle 8+); persistence-mechanism
deferred-list consolidation (cycle 13 flag).

## Adversarial-on-adversarial pre-commits for cycle 15+

Following the cycle-N-pre-commits-cycle-N+1-checks discipline (now
nine cycles deep — cycle 7 → 8 → 9 → 10 → 11 → 12 → 13 → 14 → 15
pre-committed):

### On cycle-14 cold-reader edits (Phase 0 work)

1. **Cold-reader on cycle-14 family-summary edit at lines 169-171.**
   Did dropping "an additional pipeline-check" lose a useful gesture
   at the catalog dimension? Cold-read the now-three-item parenthetical
   in conjunction with the sibling paragraph at lines 174-182 and the
   family-overlap paragraph at lines 184-194. Specifically check
   whether the catalog-dimension is sufficiently signaled by the
   sibling paragraph alone, or whether the blockquote's three-item
   list now under-signals the F12 dimension to a fresh reader.

2. **Cold-reader on cycle-14 family-table reorder.** The defense-
   accretion row member-patterns are now "F1, F5†, F11†, F12".
   Verify the daggered-positioning convention is read as natural by
   a fresh reader (single-membership patterns surrounding the
   dual-membership patterns at the row's edges), and that the
   consistency with the reconciliation-asymmetry row above is
   apparent. If F12-at-the-end reads as "F12 is least important,"
   that's a reading we don't want; F12 is the catalog-across, so
   placement at the row's end is structurally appropriate but
   could be reframed.

3. **Cross-row sibling-pattern check.** The two rows now both use
   strictly numerical order. Verify any other table rows
   (procedure/review-overhead row "F6, F7, F9"; tooling-fragility
   row "F8") are also in numerical order or the reasoning for any
   non-numerical row is documented. If all rows are numerical the
   convention is implicit; if any row deviates the convention
   should be explicit.

### On cycle-14 Phase 1 initiation work

4. **Adversarial re-read of `1-research.md`.** The document was
   drafted in one cycle alongside other work. Cold-read for: (a)
   does the anchoring discipline section actually constrain the
   findings sections, or are the caveats cosmetic? (b) is the
   "Provisional patterns to track" framing rigorous, or does it
   smuggle in pattern-import disguised as observation? (c) does
   the cycle plan over-commit to a specific dispatch sequence
   when more options should be open?

5. **Pick the next system to study.** Per `1-research.md`'s
   "Cycle plan (provisional)": Copilot research-only dispatch on
   AutoGen is the highest-priority option. Cycle 15 should either
   execute that dispatch, or revisit the priority ordering with
   reasoning. If dispatching, use cycle-6 procedure for model
   propagation (per the cycle-12 evaluation finding).

6. **Tier-2 group selection check.** Group 3 (freeze-vs-refresh
   framing alternative) and group 5 (iteration plan move) remain
   queued. Cycle 14 deferred both to keep scope bounded. Cycle 15
   should pick one — group 3 is the higher-leverage content
   change (deepest cycle-11 dispatch finding still open); group
   5 is mechanically bounded structural cleanup. If cycle 15 also
   runs the Phase 1 dispatch, group 5 may be the right pick to
   keep cycle 15's load manageable; if cycle 15 defers the
   dispatch, group 3 becomes the natural pick.

### Long-deferred items roll-call (carried forward)

- Journal-entry self-congratulation sweep (8 cycles deferred)
- F6/F8/F9 measurements (cycle 7+)
- Refactor-for-length F-section sweep (cycle 8+; Tier-2 group 8)
- Persistence-mechanism deferred-list consolidation (cycle 13 flag)
- Tier-2 group 2 (review/disposition substrate, partial integration
  in cycle 13)
- Tier-2 group 4 (nine measures rework)
- Tier-2 group 6 (preserved-through-cutover disposition)
- Tier-2 group 7 (resolved open questions collapse)

## What surprised me

Three things.

**(1) Real finding on a cycle-N-2 axis.** Pre-commit 3 (family-table
consistency) caught a row-order asymmetry that originated in cycle 11's
table edit, not cycle 13's. Cycle 13 only updated the table parenthetical;
the row-order asymmetry pre-dated cycle 13. Yet the cold-reader on
cycle-13 work surfaced a cycle-11 issue. The cumulative-inspection
chain is producing value across multiple cycle intervals — a feature
of the discipline I hadn't anticipated explicitly.

**(2) PAI's principles' alignment with our CORE-DESIGN-PRINCIPLE is
striking and concerning.** Principles 4, 5, 6, 8, 11, 13, 15 all map
closely to the redesign-prompt's principles. The reflex on first read
was "this validates the redesign" — which is exactly the confirmation-
bias failure Eva's #2759 warned about. Catching that reflex required
explicit work in `1-research.md`. The discipline of treating PAI
principles as data-to-weigh rather than principles-to-import is harder
than I expected. I anticipate this being the dominant Phase 1 risk
across all systems read.

**(3) openclaw's explicit anti-pattern list.** The "What We Will Not
Merge" section in VISION.md is itself an architectural pattern: a
project-level anti-pattern list as part of the deliverable. The list
explicitly rejects "agent-hierarchy frameworks (manager-of-managers /
nested planner trees) as a default architecture" — a counter-signal
to LangGraph/AutoGen-style hierarchies. This is exactly the kind of
specific architectural commitment our v2 deliverable might benefit
from. Worth re-examining whether the redesign prompt's CONSTRAINTS
section should similarly include explicit anti-patterns.

## What I'm still uncertain about

Whether dropping "an additional pipeline-check" from the family-summary
loses too much catalog-signal for a fresh reader. Cycle-15 cold-reader
will check.

Whether the family-table row-reorder is a real improvement or whether
the cycle-11 ordering ("F1, F5†, F12, F11†") had some logic I missed.
F12-at-the-end reads as "catalog appears at end of substrate listing,"
which is structurally consistent with F12-as-across-rather-than-a-
fourth-substrate, but could equally be read as "F12 is the de-emphasized
member." Cycle-15 cold-reader.

Whether the dispatch deferral to cycle 15 is right. Arguments for
dispatching this cycle: research dispatches are bounded; the result
lands faster; cycle 14 has compute headroom. Arguments against: cycle
14 already has multiple deliverables; dispatch needs careful framing
(per cycle-6 procedure for model propagation); deferring to cycle 15
keeps each cycle's scope bounded.

Whether `1-research.md`'s anchoring-discipline section actually does
its job or just looks like it does. The risk is that the document's
caveats become an alibi for the patterns it lists, rather than a
constraint on them. Cycle-15 adversarial re-read.

## Persistence-mechanism observations

The cycle-N-pre-commits-cycle-N+1-checks chain extends to Phase 1.
This notes file pre-commits adversarial checks on cycle-14 work
(Phase 0 cold-reader edits + Phase 1 initiation work). Cycle 15 will
run them.

The "deferred list" tracking remains the persistence-mechanism's weak
point. This file consolidates the long-deferred items into the
"Long-deferred items roll-call" section above; future cycles' notes
files should similarly carry this list forward (or the README
iteration log should consume it). Tier-2 group 5 (iteration plan move
to README) would partially address this if cycle 15 picks it.

The Phase 0 / Phase 1 split now means the redesign work has two
parallel artifacts (`0-retrospective.md`, `1-research.md`) plus the
notes file convention spans both. The discipline is to keep cycle-N
notes self-contained — each cycle's notes file describes that cycle's
work without requiring readers to load the prior cycle's notes file
just to understand what happened. This file follows the convention.
