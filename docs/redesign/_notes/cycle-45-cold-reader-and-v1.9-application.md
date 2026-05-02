# Cycle 45 — cold-reader on v1.8 + cycle-44 work; v1.9 application

**Date:** 2026-05-02 (first cycle of the day)
**Cycle issue:** #2812
**Inherits from:** cycle 44 (`_notes/cycle-44-cold-reader-and-v1.8-application.md`)

## Cold-reader: 2 PASS + 1 BORDERLINE-FAIL

Three questions inherited from cycle 44's pre-commit checklist. Each
re-walked with adversarial framing.

### Q(a) PASS — v1.8 internal consistency on Axis 2 × Axis 3 dep map rewrite

**Question:** Does the v1.8 dep map rewrite (which names "filesystem-based
memory" positions that align with file-per-component) cover the case
where a v2 candidate wants file-per-component WITHOUT filesystem-based
memory (e.g., in-memory typed channels above filesystem state)?

**Re-walk of the v1.8 text** (`2-design-framework.md` lines 562-573):

> **Axis 2 (state) × Axis 3 (memory):** State representation shapes
> which Axis 3 positions are natural — file-per-component aligns with
> filesystem-based memory positions (singleton plugin slot WITH
> filesystem storage as in openclaw's `~/.openclaw/agents/<agentId>/`;
> top-level architectural principle with filesystem memory as in PAI;
> wiki+search with file-per-entry as in oh-my-codex's `.omx/wiki/`);
> typed-channel-map aligns with typed channels with checkpointer
> (LangGraph); repo-as-state aligns with repository-as-record (OpenAI
> harness). The natural-alignment framing is supportive rather than
> exclusive: file-per-component does not preclude context-trace memory
> or other non-filesystem Axis 3 positions, but pairs more naturally
> with the listed filesystem-based positions.

The **closing sentence** explicitly addresses non-filesystem Axis 3
positions:
- Names context-trace by name as a non-precluded option
- Uses "or other non-filesystem Axis 3 positions" as a catchall

The "in-memory typed channels above filesystem state" scenario from
cycle-44's pre-commit checklist would correspond to Axis 2 =
file-per-component + Axis 3 = "Typed channels (short/long-term
distinction)." The Axis 3 typed-channels position is logically
about typing primitives, not storage; the LangGraph instance happens
to use filesystem persistence (via Store + checkpointer), but the
position itself is not filesystem-bound. The catchall "other
non-filesystem Axis 3 positions" covers the in-memory variant.

**Verdict: PASS** — the closing caveat adequately covers the stress
scenario.

### Q(b) PASS — v1.8 prescriptiveness check

**Question:** By naming specific Axis 3 positions in the dep map, does
v1.8 unintentionally constrain Phase 2 candidate creativity? Stress
test: imagine a v2 candidate that takes file-per-component (Axis 2) +
context-trace memory (Axis 3) — does the v1.8 dep map rule this out
as "unnatural" when it's actually defensible?

**Re-walk:**

The stress test scenario (file-per-component + context-trace memory)
is **explicitly named in the v1.8 caveat**: "file-per-component does
not preclude context-trace memory or other non-filesystem Axis 3
positions." The exact pairing the question asks about is the exact
example named in the caveat.

A subtler concern: by listing 3 specific filesystem-based positions
(singleton plugin slot, top-level architectural principle, wiki+search)
but only naming context-trace in the catchall, a reader might infer
that the 3 listed are "preferred" while context-trace is "tolerated."
But the caveat language ("supportive rather than exclusive," "pairs
more naturally") is descriptive, not normative — it says some pairings
are more natural in implementation, not that v2 candidates should pick
natural pairings.

The framing is: "if you pair file-per-component with one of these 3
filesystem-based positions, the implementation is natural; if you pair
with a non-filesystem position, it works but you'll need to bridge the
storage and memory layers explicitly." That's accurate guidance for
candidate authors, not prescriptive constraint.

**Verdict: PASS** — caveat correctly names the stress-test scenario as
non-precluded; framing is descriptive not normative.

### Q(c) BORDERLINE-FAIL — F8 mapping rationale "or" phrasing

**Question:** Did v1.8's "Bounded loops (prevention) or stuck-session
watchdog (detection-and-recovery)" framing accidentally imply these
are alternatives when they could be complementary? Cycle-44 same-cycle
Q2 flagged as BORDERLINE-PASS; cycle-45 cross-cycle review can verify
or escalate.

**Re-walk of the v1.8 F8 mapping rationale** (line 635 pre-edit):

> F8 (abandonment cascades) | Tooling fragility | Axis 9,
> CORE-DESIGN-PRINCIPLE | Bounded loops (prevention; loop-count
> ceiling positions) or stuck-session watchdog for runtime-ceiling
> positions (detection-and-recovery; openclaw's
> `diagnostics.stuckSessionWarnMs` instance — detect stale runs and
> release lanes) + single-implementation discipline (no parallel
> implementations)

**Axis 9 structure** (line 392):

| Position | Systems supporting | Notes |
|---|---|---|
| None (open-ended runs) | Rare in surveyed | Implicit in v1's per-cycle non-bounded retry |
| Loop count ceilings | oh-my-codex, Voyager | 2-system strict |
| Runtime ceiling | openclaw, ~~Cognition~~ | Anchor weakened on Cognition |
| Both (loop + runtime) | None explicitly in surveyed | Composable |

Axis 9 has FOUR positions. Three of them mitigate F8 via different
primitive strategies:
1. **Loop count ceilings** → bounded loops (prevention)
2. **Runtime ceiling** → stuck-session watchdog (detection-and-recovery)
3. **Both (loop + runtime)** → composes both primitives

The "None" position does not mitigate F8.

The current v1.8 rationale enumerates positions 1 and 2 (linked by
"or") but doesn't explicitly name position 3. Under inclusive-or
reading, "Both" is implicitly covered ("either or both"). Under
exclusive-or reading, "Both" is excluded.

**Comparison with F5's "or" structure** (line 632):

> F5 | ... | Axis 2, Axis 8 | File-per-component or typed-channel
> separates concerns; mechanical CI catches procedural-leak patterns

F5's "or" is over Axis 2 positions (file-per-component, typed-channel).
**Axis 2 has no "both" position** — a candidate picks one. So F5's "or"
is naturally exclusive (pick one of the two listed Axis 2 positions).

F8's "or" is structurally different because **Axis 9 explicitly has a
"Both" position** (line 392, marked "Composable"). The natural-language
"or" is more vulnerable to misreading in this case because the
underlying axis structure has an explicit compositional position the
rationale doesn't name.

**Cross-cycle observation:** Cycle-44 same-cycle Q2's BORDERLINE-PASS
verdict was based on "wording is slightly ambiguous but matches the
underlying Axis 9 structure." Cross-cycle cold-reader sharpens this —
the wording matches positions 2 and 3 BUT NOT position 4 (Both). The
underlying Axis 9 structure has 3 F8-mitigating positions, not 2; the
rationale enumerates only 2 of them.

**Verdict: BORDERLINE-FAIL** — escalation from cycle-44's BORDERLINE-PASS
to BORDERLINE-FAIL with a bounded-mechanical wording fix that
explicitly names the "Both" position.

## v1.9 application — one change

Applied to `docs/redesign/2-design-framework.md`:

### Change 1 (Q(c) BORDERLINE-FAIL fix) — F8 mapping rationale enumeration

**Old:** "Bounded loops (prevention; loop-count ceiling positions) or
stuck-session watchdog for runtime-ceiling positions
(detection-and-recovery; openclaw's `diagnostics.stuckSessionWarnMs`
instance — detect stale runs and release lanes) + single-implementation
discipline (no parallel implementations)"

**New:** "Bounded loops (loop-count ceiling positions; prevention),
stuck-session watchdog (runtime-ceiling positions; detection-and-
recovery; openclaw's `diagnostics.stuckSessionWarnMs` instance —
detect stale runs and release lanes), or both compositionally
(Axis 9's `Both (loop + runtime)` position) + single-implementation
discipline (no parallel implementations)"

Three substantive changes:
1. **Comma-separated three-way enumeration** instead of "or" between
   two items — naturally inclusive of all three F8-mitigating Axis 9
   positions
2. **Explicit naming of `Both (loop + runtime)`** with backtick-quoted
   exact position name from Axis 9 table (cross-axis citation precision)
3. **Reordered parentheticals** to put position-name before primitive-
   strategy-label ("loop-count ceiling positions; prevention" rather
   than "prevention; loop-count ceiling positions") — places the
   structural axis reference first, then the F8-mitigation strategy

Positions 2 and 3's parenthetical order is now consistent: "(<position
type>; <primitive role>; <instance reference if any>)."

### v1.9 status update

Iteration history table gets a v1.9 row covering this change. Status
header bumps v1.8 → v1.9.

## Same-cycle review (5 questions)

Five questions on cycle-45 work + v1.9 framework:

### Q1 — v1.9 internal consistency

Did v1.9's F8 rationale rewrite introduce any internal inconsistencies
elsewhere?

Re-walked: Axis 9 row at line 392 has the four positions including
`Both (loop + runtime)`. The new F8 rationale references `Both (loop +
runtime)` with backticks for cross-axis citation precision. The exact
string matches the Axis 9 table position name.

Cross-checked F-pattern rationales for similar compositional issues:
- F5: "File-per-component or typed-channel" — Axis 2 has no Both
  position (the two Axis 2 positions are mutually exclusive); "or"
  is correctly exclusive
- F7: "Specialization + mechanical enforcement + iteration ceilings +
  fat-harness reduce self-management surface" — uses "+" composition,
  no "or"; correct
- F11: "Append-only history (Axis 4) prevents... reconciliation
  discipline (Axis 12) refreshes..." — semicolon separator between
  two axes; no compositional issue
- F12: "All three contribute" — explicitly compositional; correct

No other F-pattern rationale has the structural mismatch between
"or" wording and underlying axis with a "both" position.

**Verdict: PASS.**

### Q2 — Reordered parenthetical structure consistency

Did the parenthetical reordering ("loop-count ceiling positions;
prevention" rather than "prevention; loop-count ceiling positions")
introduce any cross-rationale inconsistency?

Walked through other F-pattern rationales for similar
parenthetical-with-position-reference structures: most rationales use
"axis-citation" form ("Axis N + Axis M") rather than "(position type;
strategy)" form. F8 is the only rationale with this kind of
parenthetical embedding; reordering doesn't affect any other.

The new order ("position type; primitive role; instance reference")
puts the structural axis reference first, which is more consistent
with how readers of the framework navigate (axis → position → primitive
→ instance, not primitive → axis → position).

**Verdict: PASS.**

### Q3 — Cross-cycle escalation calibration

Is the cycle-44 → cycle-45 escalation (BORDERLINE-PASS → BORDERLINE-FAIL)
appropriate, or am I over-reading?

Arguments for escalation:
- Axis 9 has 4 positions; rationale enumerates 2; this is a structural
  precision gap, not just a wording preference
- Comparison with F5 (where "or" is correctly exclusive because Axis 2
  has no "both" position) shows F8's "or" is structurally vulnerable
  in a way F5's isn't
- Backtick-quoted exact position name ("`Both (loop + runtime)`") is
  a clarity improvement that precision-conscious framework readers
  benefit from

Arguments against escalation:
- Inclusive-or reading covers the "Both" case implicitly
- Reader who picks Axis 9 = "Both" would naturally compose both
  primitives even without explicit rationale guidance
- The wording change is small; the Phase 2 candidate impact is
  marginal
- Asymptotic convergence pattern: each cross-cycle escalation should
  capture a smaller issue than the previous; this one IS smaller than
  cycle-44's load-bearing fix

The escalation is borderline. The fix is bounded-mechanical (one cell
in one table; no cascading propagation needed). The cost of applying
is low; the precision gain is real even if marginal.

**Verdict: PASS** — escalation is defensible. The convergence pattern
supports increasingly small fixes as the framework matures.

### Q4 — Pattern observation: same-cycle Q2 vs cross-cycle Q(c)

Same-cycle Q2 from cycle 44 verdict was BORDERLINE-PASS; cross-cycle
Q(c) from cycle 45 verdict was BORDERLINE-FAIL. The pattern
(cross-cycle escalates same-cycle's borderline) has fired four times
now (cycle 36→35, cycle 37→36, cycle 44→43, cycle 45→44).

Is this calibration drift or normal cross-cycle convergence?

Cycle 44's same-cycle review concluded the same: "normal cross-cycle
convergence" rather than calibration drift. Cycle 45 confirms:
- Same-cycle review caught the issue (Q2 BORDERLINE-PASS noted "could
  be clarified in v1.9+ if Phase 2 candidates surface confusion")
- Cross-cycle review escalated to fix because the "if Phase 2
  candidates surface confusion" condition was speculative — better to
  fix preemptively with a bounded-mechanical change than wait for
  Phase 2 candidate generation to surface real confusion

The pattern remains: same-cycle flags issues at the right severity
(borderline); cross-cycle fixes them when the cost is bounded.
Calibration drift would be: same-cycle MISSING issues entirely. That
hasn't been observed.

**Verdict: PASS** — pattern is healthy.

### Q5 — Cycle 46 pre-commit checklist scope

Three questions for cycle 46's cold-reader:

- (a) v1.9 internal consistency: re-walk the F8 rationale rewrite. Does
  the comma-separated three-way enumeration read as cleanly as a
  reader skimming Phase 2 candidate guidance would expect, or does
  the Axis 9 cross-citation feel over-cited? Stress test by reading
  the rationale cold (without v1.9 context).
- (b) Cross-rationale precision sweep: do other F-pattern rationales
  have similar "or"-with-implicit-compositional-position issues, even
  without an explicit "Both" position in the underlying axis? E.g.,
  does F4's "History substrate determines... lifecycle primitives
  address... reconciliation refreshes..." use semicolons consistently,
  and could a candidate read it as a sequence-of-three rather than
  three-distinct-aspects?
- (c) Bounded mechanical TBD: cross-axis-impact-check design iteration
  (4 open Qs from cycle-44 draft); redispatch tool design draft; or
  housekeeping sweep on remaining v1-era input-from-eva items.

Three questions covering different lenses. Each question is concrete
enough to be falsifiable.

**Verdict: PASS.**

## What surprised me

**The F8 "or" wording's structural vulnerability vs F5's "or" is
the load-bearing observation.** Going into the cold-reader, I expected
Q(c) to be a wording preference question. Walking through the
comparison with F5's "or" surfaced that F8 is structurally different
because Axis 9 has a "Both" position while Axis 2 doesn't. F5's "or"
is correctly exclusive; F8's needs to be inclusive (or enumeration).

The cross-axis structural awareness — knowing which axes have "both"
positions and which don't — is the kind of mechanical check the
cross-axis-impact-check tool would catch automatically. Each of these
escalations is one more data point for the tool's value.

**Q(a) and Q(b) were genuinely PASS — the v1.8 caveat was thorough.**
Cycle 44 added the closing sentence ("supportive rather than
exclusive...") that explicitly named the stress-test scenarios.
Cross-cycle review confirms the caveat does the work it was added to
do. This is the "cycle-N adds, cycle-N+1 verifies it holds" pattern
that the iteration discipline assumes.

## What I couldn't figure out

**Whether the cross-cycle escalation pattern's asymptotic convergence
is real.** Each cycle's escalation captures a smaller issue: cycle-44
escalated cycle-43 to a load-bearing dep map rewrite; cycle-45
escalates cycle-44 to a single-cell wording change. If the pattern is
truly asymptotic, cycle 46 should escalate cycle 45 to something
even smaller (or PASS without escalation). If the pattern is not
asymptotic — if escalations stay proportionally large — the framework
isn't converging.

Provisional read: convergence appears real. The framework is becoming
more precise with smaller corrections each cycle. But this is a
multi-cycle observation; cycles 46-50 will refine the read.

**Whether the bounded-mechanical lint tool (cross-axis-impact-check)
should be prioritized for build now.** The cross-axis-update-propagation
failure mode has fired in cycles 42, 43, 44, and 45 (each in
different ways). Four instances over five cycles is empirical
evidence that this is a recurring failure mode worth tooling. But
prioritizing tool build over framework iteration would itself be a
self-management cycle. Lean toward: continue framework iteration,
defer tool build to bounded-mechanical work after the framework
stabilizes (which Phase 2 candidate generation will help define —
the framework needs to be stable enough that the tool's output is
useful).

Decision: defer cross-axis-impact-check build to cycle-46+, but
iterate the design draft (the 4 open Qs) when bounded-mechanical
capacity permits. Cycle 45 itself is using bounded-mechanical capacity
on the v1.9 application; no parallel work this cycle.

## Pre-commit checklist for cycle 46's cold-reader

Three questions:

- **(a)** v1.9 internal consistency check on the F8 rationale rewrite:
  re-walk the comma-separated three-way enumeration. Does it read as
  cleanly as a reader skimming Phase 2 candidate guidance would
  expect? Specifically: the parenthetical structure now embeds three
  pieces of information per position (position-type; primitive role;
  instance reference) — is this density appropriate, or does it
  obscure the simpler "three F8-mitigating positions" message?
- **(b)** Cross-rationale precision sweep: do other F-pattern
  rationales have similar "or"-with-implicit-compositional-position
  issues? Specifically check F11 (Axis 4 + Axis 12 — both axes have
  multiple positions; could the "+" composition imply a single
  position from each, when actually the F11 fix can compose multiple
  positions from each axis?) and F12 (Axis 2 + Axis 4 + Axis 10 —
  three axes, are the rationale's contributions per-axis clear?).
- **(c)** Bounded mechanical TBD: cross-axis-impact-check design
  iteration (4 open Qs from cycle-44 draft); redispatch tool design
  draft (cycle 43 deferred, lower frequency than cross-axis-impact-
  check); housekeeping sweep on remaining v1-era input-from-eva items
  (#2039, #699/#808/#809) — the conservative deferral remains correct
  unless cycle-46 finds genuine absorption signal.

## Cycle 46 plan (provisional)

1. **Substantive focal:** cross-cycle cold-reader on v1.9 + cycle-45
   work (3 Qs above). Verify v1.9 changes don't introduce hidden
   structural inconsistencies.
2. **Substantive parallel:** TBD per cold-reader. Possibilities:
   - v1.10 corrections if cold-reader finds substantive issues
   - Iterate cross-axis-impact-check design (4 open Qs)
   - `redispatch` tool design draft
   - Continued housekeeping sweep
3. **Bounded mechanical:** if substantive load is light, prototype
   the cross-axis-impact-check parser in a single Rust file.

## What this cycle achieved

Cycle 45 is the **fourth "cold-reader → v1.X application" cycle** in
the redesign sequence (after cycles 38, 42, and 44 which were v1.3,
v1.6, and v1.8 applications). The substantive output:

- 3 cold-reader questions answered (2 PASS + 1 BORDERLINE-FAIL)
- 1 bounded-mechanical framework fix (F8 mapping rationale comma-
  separated three-way enumeration; explicit `Both (loop + runtime)`
  position naming; parenthetical reorder for cross-axis citation
  precision)
- v1.8 → v1.9 framework version bump
- 1 cycle-45 same-cycle review (5 questions, all PASS)

The most interesting cross-cycle observation: **cross-cycle escalation
appears to be asymptotically converging.** Each escalation captures a
smaller issue: cycle-44 fixed a load-bearing dep map; cycle-45 fixes a
single-cell wording. If the pattern holds, cycle 46 should either
PASS without escalation or surface an even smaller issue.

The structural observation: **F8's "or" is structurally vulnerable in
a way F5's "or" isn't, because Axis 9 has a "Both" position and Axis 2
doesn't.** This kind of cross-axis structural awareness is what the
proposed cross-axis-impact-check tool would catch mechanically. Each
cross-axis-update-propagation failure mode instance (4 instances now,
across cycles 42-45) is one more data point for the tool's value.
