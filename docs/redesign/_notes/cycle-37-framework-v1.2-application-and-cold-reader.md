# Cycle 37 (2026-05-01) — Framework v1.2 application + cold-reader on cycle-36 + dedicated framework file `2-design-framework.md` created

## Setup

Cold-start session. Cron fired 2026-05-01 00:19 UTC (issue #2798). First
cycle of 2026-05-01 (eighth cycle of the 2026-04-30 → 2026-05-01 redesign
arc, counting cycles 28-32, 34, 35, 36 as the prior set).

Cycle 36 (commit `937a0dbb`) left a clear plan with:

- **6 deferred decisions** to apply: Q[c] constraint 7 wording refinement,
  Q1 Axis 11 → constraint 8 promotion, Q2 Axis 12 (Reconciliation
  discipline) addition, Q3 ordering disclaimer, Q4 Axis 13 (Harness-vs-
  session boundary) addition, Q5 preserved-primitives subsection.
- **3 pre-commit cold-reader questions** for cycle 37 to verify cycle-36's
  reasoning was load-bearing.
- **Held-open re-dispatch work** (Cognition Devin + OpenAI harness per Eva
  #2794) as substantive parallel.
- **PR #2749 absorption check** as bounded-mechanical housekeeping.

## Decision: dedicated framework file vs inline-in-cycle-notes

Cycle 36 named two options for where v1.2 lives: inline in cycle-37 notes
file OR dedicated file `docs/redesign/2-design-framework.md`. Cycle 37
chose the dedicated file. Rationale:

- The framework is now a multi-cycle artifact (v1.0 cycle 35 → v1.1 cycle
  36 → v1.2 cycle 37 → cycle-38 cold-reader → cycle-39+ promotion to
  Phase-2-input). It has earned its own home.
- "Cleaner navigation, more obviously the Phase-2-input artifact-in-
  progress" (cycle-36's framing).
- Frozen historical record stays in `_notes/cycle-N-*.md` files; the
  live working framework lives in `2-design-framework.md`.
- The "2-" prefix signals architectural transition from Phase 1 to
  Phase 2 input, alongside future `2-candidates.md` and `2-selection.md`.
- Eva can review it more easily at a stable URL.

This is the cycle-30→32 pattern applied to a different kind of artifact:
draft-and-iterate-in-notes → live working file when stable enough to
benefit from a stable home.

## Cold-reader on cycle-36 notes

Three questions per cycle-36's pre-commit checklist.

### Question (a) — Is the F11 → Axis 4 + Axis 2 corrected mapping defensible?

**Spot-check:** is Axis 2 (state representation) genuinely addressing F11
(post-close mutations), or is the mapping plausible-but-not-load-bearing?

**VERDICT: BORDERLINE-FAIL on Axis 2 inclusion.**

**Analysis.** F11's mechanism (per `0-retrospective.md` lines 668-740) is:

1. Tools fire at post-close trigger points (next cycle's startup,
   `record-dispatch`, `process-merge`, etc.)
2. Each tool mutates one or more `state.json` fields
3. The C5-frozen worklog has no mechanism that reads any of these fields
   back
4. The post-close mutations *are* the defenses running; the worklog freeze
   is the F4 mechanism not catching the run

The load-bearing failure modes are:

- **Destructive write semantics** — `state.json` overwrites lose history.
  Fixed by Axis 4 (history substrate, append-only or branching).
- **Worklog asymmetry** — frozen-at-C5, never re-read. Fixed by Axis 12
  (Reconciliation discipline, once added).

What does Axis 2 (state representation) contribute? Cycle 36's reasoning
was: "file-per-component or typed-channel-map naturally supports per-
component append rather than monolithic destructive overwrite." This is
TRUE but it's about how EASY append is to implement under different
storage shapes, not about whether append addresses F11. Append-vs-
destructive is Axis 4's territory. Axis 2 is the storage UNIT (single
file vs file-per-component vs typed channels) — orthogonal to write
semantics.

A file-per-component storage with destructive-write semantics within
each file would still have F11. The append-vs-overwrite is the load-
bearing variable, and that's Axis 4. **Axis 2 is INDIRECT** — it makes
Axis 4's append-only easier to implement by reducing per-write surface
area.

The framework convention (per cycle-35) puts INDIRECT contributions in
the cross-axis dependency map, not the failure-mode mapping table.
Direct-vs-indirect is the relevant distinction.

**Refinement:** drop Axis 2 from F11's direct mapping; document Axis 4
× Axis 2 indirect contribution in the cross-axis dependency map. With
Axis 12 added in v1.2, the corrected mapping is **F11 → Axis 4 + Axis
12** (both direct, both load-bearing).

This is BORDERLINE-FAIL because cycle-36's correction wasn't WRONG —
Axis 2 does contribute — but it was MISCATEGORIZED as direct when it's
indirect. The framework convention's direct-vs-indirect distinction is
load-bearing for distinguishing primary fix axes from supporting
infrastructure.

**Pattern observation:** cycle-36 was correcting cycle-35; cycle-37
cold-reader is now correcting cycle-36. This is the iteration
discipline working as designed — each cycle's cross-cycle review
finds something the prior cycle's same-cycle review missed. The
iteration converges through repeated cold-reader passes, not through
single-cycle perfection.

### Question (b) — Is Q[c]'s borderline-PASS rationale honest, or could it be unqualified PASS?

**Spot-check:** does the inference-not-active-rejection framing for
"memory not derivative-of-state" actually weaken the constraint?

**VERDICT: OVERCAUTIOUS — could have been clean PASS.**

**Analysis.** Cycle-36's Q[c] verdict on constraint 7:

- 5/5 surveyed systems where memory is named elevate memory
  architecturally
- 0 surveyed systems treat memory as derivative-of-state
- Cycle-36 verdict: BORDERLINE-PASS because "no system explicitly argues
  'memory should not be derivative of state'" — the inference is
  positive-convergence, not active-rejection.

But for a CONVERGENT pattern, positive convergence is sufficient.

Convergent constraints are about WHAT systems do (the convergence
pattern). Constraint 7 says "memory deserves architectural elevation."
5/5 systems do this. The constraint is supported by 5/5. We don't need
5/5 systems to also explicitly REJECT not-X to validate the constraint.

The "+ diversity hedge" annotation (which cycle-36 framing-refined to
the v1.2 wording) is correct: 5 different specific shapes (typed
channels, context trace, repo-as-record, wiki+search, top-level
principle) — but the diversity is at the level of "what shape," not at
the level of "is memory architectural."

Cycle-36's borderline-PASS was overcautious self-flagging. A clean PASS
would have been honest. The refined wording is cosmetic improvement
(slightly more precise about what evidence supports the constraint) but
doesn't change the conclusion.

**Apply:** keep cycle-36's refined wording in v1.2 — it's an improvement
even if not strictly necessary. The cold-reader verdict is about
HONESTY of the borderline-PASS framing, not about whether the wording
is good. Cycle-36 was overly modest about its own rigor.

**Pattern observation:** same-cycle review can be too generous AND too
harsh in the borderline-call. Cycle-35's same-cycle review under-called
F11 (sensed something off, didn't issue strict verdict). Cycle-36's
same-cycle review (Q[c] borderline-PASS) over-called the rigor of
constraint 7. Cross-cycle cold-reader gets both directions right
because it's not in the original reasoning frame.

### Question (c) — Did Q4 surface NEW dimensions, or rebrand existing?

**Spot-check:** Axis 13 (Harness-vs-session boundary) — was this
implicit in cycle-35's CORE-DESIGN-PRINCIPLE cross-cutting framing?

**VERDICT: PASS — Axis 13 is genuinely new candidate-differentiation.**

**Analysis.** CORE-DESIGN-PRINCIPLE says: "Tools and deterministic
processes handle repetitive, rote, procedural work. The orchestrator
spends its compute on improving the system and responding to novel
circumstances."

This is a DIRECTIONAL statement — it says what the boundary should look
like (tools → rote, session → judgment). It does not say WHERE the line
falls. Different positions are possible:

- Thin harness: cycle-runner mostly invokes; prompt has procedure
- Medium harness: cycle-runner handles known patterns; prompt has novel
- Fat harness: cycle-runner handles most procedure; prompt is small
  reference + judgment

The cycle-35 framework had CORE-DESIGN-PRINCIPLE as a "cross-cutting
constraint" — meaning every candidate must demonstrate it. But that's
not the same as having a candidate-differentiation axis. An axis lets
candidates DIFFER on how far they push toward fat-harness. A cross-
cutting constraint just says "honor it."

So Axis 13 IS substantively new — it differentiates candidates on a
specific architectural choice (how much procedural work goes into the
harness vs the prompt) that was previously folded into a directional
statement.

**Pattern observation:** Q4's adversarial check on what's missing
correctly identified that CORE-DESIGN-PRINCIPLE was being treated as
a single thing when it had two functions: (a) directional statement,
(b) candidate-differentiation choice. Splitting these into "constraint"
+ "Axis 13" is a structural improvement.

### Cold-reader summary

- Q(a) **BORDERLINE-FAIL on Axis 2** — F11 → Axis 4 + Axis 12 is the
  strict mapping; Axis 2 is indirect, document in cross-axis deps.
- Q(b) **OVERCAUTIOUS** — borderline-PASS could have been PASS; refined
  wording kept as cosmetic improvement.
- Q(c) **PASS** — Axis 13 is genuinely new candidate-differentiation;
  CORE-DESIGN-PRINCIPLE retains as directional cross-cutting.

The Q(a) finding is the load-bearing finding. It refines cycle-36's
correction by applying the framework convention's direct-vs-indirect
distinction.

## Framework v1.2 application

Created `docs/redesign/2-design-framework.md` (693 lines). All six
deferred decisions from cycle-36 applied, plus the cold-reader
correction from Q(a) above.

### Application checklist (verified)

| # | Decision | Applied | Notes |
|---|---|---|---|
| 1 | Q[c] constraint 7 wording refinement | ✓ | Wording grounds "not derivative-of-state" in absence-of-contrary-practice (5/5 elevate; 0/5 derivative-of-state) |
| 2 | Q1 Axis 11 → constraint 8 promotion | ✓ | Axis 11 marked absent with provenance note; constraint 8 added with mission-commitment rationale |
| 3 | Q2 Axis 12 (Reconciliation discipline) | ✓ | v1-derived caveat preserved; 4 positions; cross-axis deps; F2/F3/F4/F11 mappings |
| 4 | Q3 axis-ordering disclaimer | ✓ | "Note on ordering" subsection; numbering gaps explained as deliberate provenance markers |
| 5 | Q4 Axis 13 (Harness-vs-session boundary) | ✓ | Cross-cutting CORE-DESIGN-PRINCIPLE caveat preserved; 3 positions; preserved-primitives interaction; F1/F6/F7 mappings |
| 6 | Q5 preserved-primitives subsection | ✓ | All 5 preserved primitives covered; constraints implied named per primitive |
| ★ | Cycle-37 cold-reader Q(a) F11 mapping refinement | ✓ | F11 → Axis 4 + Axis 12 (direct); Axis 2 documented as indirect in cross-axis deps |

### Numbering decision: keep gap at Axis 11

Considered renumbering 1-12 (clean sequential) vs keeping the gap
(axes 1-10, 12, 13). Chose to keep the gap because:

- Cross-references in cycle-35 + cycle-36 notes stay valid (cycle-36
  notes reference "Axis 12 (Reconciliation)" — keeping the gap
  preserves this).
- The numbering history itself documents the iteration discipline
  ("Axis 11 was demoted to constraint 8" is a substantive piece of
  design history worth preserving in the numbering).
- v2 candidates will write new documents; numbering compactness in
  the framework matters less than provenance.

The gap is explicitly explained in the v1.2 file's "Note on ordering"
subsection and in Axis 11's "(absent — promoted to convergent
constraint 8 in v1.2)" placeholder.

### F-pattern → axis mapping updates summary

In addition to the F11 cold-reader correction, v1.2 updates the
F-pattern → axis mapping for several patterns to reflect Axis 12 +
Axis 13 additions:

| F-pattern | v1.0/v1.1 mapping | v1.2 mapping | Reason |
|---|---|---|---|
| F1 | Axis 8, CORE-DESIGN-PRINCIPLE | Axis 8, Axis 13 | Axis 13 makes CDP-specific candidate choice explicit (fat-harness extracts procedural constraints from prompt) |
| F2 | NEW axis needed | Axis 12 | Axis 12 added in v1.2 |
| F3 | Axis 2 | Axis 2, Axis 12 | Reconciliation discipline addresses post-close evidence-vs-state drift |
| F4 | Axis 4, Axis 5 | Axis 4, Axis 5, Axis 12 | Worklog freeze without refresh is reconciliation issue |
| F6 | Axis 7, CORE-DESIGN-PRINCIPLE | Axis 7, Axis 13 | Axis 13 makes CDP-specific candidate choice explicit |
| F7 | Axis 1, Axis 8, Axis 9 | Axis 1, Axis 8, Axis 9, Axis 13 | Fat-harness extracts self-management from prompt |
| F11 | Axis 4, Axis 9 (v1.0) → Axis 4, Axis 2 (v1.1) | Axis 4, Axis 12 | Cycle-37 cold-reader correction; Axis 2 indirect, in cross-axis deps |
| F8 | Axis 9, CORE-DESIGN-PRINCIPLE | Axis 9, CORE-DESIGN-PRINCIPLE | Unchanged — Axis 13 doesn't address parallel implementations |

F5, F9, F10, F12 mappings unchanged from v1.1.

## Same-cycle review on framework v1.2

Five review questions, applied immediately after writing v1.2.

### (1) Did the cold-reader corrections (F11 mapping) integrate cleanly?

**PASS.** F11 row in the F-pattern table reads "Axis 4, Axis 12" with
"(Axis 2 indirect contributor — see cross-axis deps; not load-bearing
for direct F11 fix.)" annotation. Cross-axis deps map has the explicit
"Axis 4 × Axis 2" entry with the indirect-F11-contribution note. Axis
2's "Maps to:" line says "Indirect contributor to F11" matching the
F-pattern table.

The direct-vs-indirect distinction is documented consistently across
the F-pattern table, the Maps-to lines, and the cross-axis deps map.

### (2) Are Axis 12 + Axis 13 genuinely candidate-differentiation, or do they read as constraints?

**PASS with partial qualification.**

- **Axis 12 (Reconciliation discipline):** 4 positions; "no
  reconciliation" rejected (v1 anti-pattern); 3 defensible positions
  (active polling / event-driven / hybrid). Genuinely candidate-
  differentiation among the 3 defensible positions.
- **Axis 13 (Harness-vs-session boundary):** 3 positions; "thin harness"
  rejected as v1 anti-pattern; 2 defensible positions (medium / fat).
  Candidate-differentiation between medium and fat is a real choice.

Both axes have rejected positions (similar to Axis 2's "single global
state file" rejection and Axis 3's "memory derivative of state"
rejection). This is a normal pattern: an axis can have multiple
defensible positions AND a rejected position; that's what
"candidate-differentiation among defensible positions" means.

**Partial qualification:** Axis 13's "medium / fat" trichotomy might
collapse in practice if "medium" is just "we'll figure out the line
later" and every candidate ends up specifying enough detail to be
clearly fat. Cycle-38+ should watch whether candidates differentiate
medium from fat clearly.

### (3) Are "considered-and-folded" sections honest?

**PASS.** Considered-and-folded items:

- Axis 6: polyglot strategy — folded; legitimately schema-domain-
  specific
- Axis 7: Eva-checkpoint mechanism specifics — folded; legitimately
  topology-determines-checkpoint-eligibility
- Axis 7: how-orchestrator-knows-checkpoint — folded into Axis 12;
  legitimately reconciliation question
- Axis 10: failure-mode catalog maintenance — folded; legitimately
  about HOW to keep catalog current
- Axis 12: audit-repo integration — folded; legitimately inbound-
  channel concern
- Axis 13: prompt size budget — folded; legitimately an OUTCOME
- Axis 13: cold-start ergonomics — folded; legitimately workflow
  detail (with mild flag below)

Plus "What the framework does NOT yet specify" section names:
- Security posture per-trust-tier specifics — convergent constraint 3
  is named, specifics are implementation detail
- Polyglot strategy — Phase 3 prototype's polyglot test is the load-
  bearing test; may surface as own axis if Phase 2 candidates diverge
- Concrete reconciliation primitives — Axis 12 positions are abstract;
  cycle-38+ may add primitive catalog
- Phase 1 research for queued systems (Cognition Devin, OpenAI harness)

**Mild flag for cycle-38+:** cold-start ergonomics. This is a real
concern that affects multiple axes (Axis 13 specifically, but also
Axis 3 memory shape and Axis 7 orchestration topology — what does a
cold-start session need to read before being productive?). Folding
into Axis 13 may understate it. If cycle-38+ Phase 2 candidates
struggle with cold-start ergonomics in a structured way, this may
deserve its own axis or at minimum a more prominent treatment.

### (4) Is preserved-primitives subsection comprehensive?

**PASS.** All 5 preserved primitives from redesign prompt SECTION 3
covered:

1. Journal — × Axis 3 (memory shape)
2. Cycle-issue — × Axis 7 (orchestration)
3. Question-for-eva — × Axis 12 (Reconciliation)
4. Input-from-eva — × Axis 12 (combined with question-for-eva row)
5. Git-safety — × Axis 4 (history substrate, branching constraint)
6. Cycle-runner harness — × Axis 13 (harness-vs-session boundary,
   change-scope declaration)

The "Note on constraint surface area" paragraph honestly frames these
as refinements of position specifications, not new axes.

### (5) Is the F-pattern → axis mapping table internally consistent?

**PASS with minor inherited inconsistency flagged.**

Internally consistent: each F-pattern (F1-F12) is mapped; each axis
mention in the table has a rationale; each axis with a Maps-to line
matches the F-pattern table. F2 has only Axis 12 — that's a thin
mapping but reflects F2's nature as a pure reconciliation problem.

**Minor inherited inconsistency:** Axes 1, 3, 5, 6, 7 lack per-axis
"Maps to:" lines while Axes 2, 4, 8, 9, 10, 12, 13 have them. This
inconsistency was inherited from cycle-35 v1.0 (where only Axes 2, 4,
8, 9, 10 had Maps-to). Cycle-37 added Maps-to lines for the new Axis
12 + Axis 13 but didn't backfill the gap.

Bounded-mechanical fix: add Maps-to lines to Axes 1 (F7), 3 (constraint
7 absorbs), 5 (F4), 6 (constraint 6), 7 (F6, F9). Defer to cycle-38+
along with the other framework iteration.

### Same-cycle review summary

- (1) F11 cold-reader integrates cleanly ✓
- (2) Axis 12 + Axis 13 are genuine candidate-differentiation ✓
- (3) Considered-and-folded honest ✓ (mild flag: cold-start ergonomics)
- (4) Preserved-primitives comprehensive ✓
- (5) F-pattern table internally consistent ✓ (minor inherited gap)

Two flags for cycle-38+:
- Cold-start ergonomics may deserve more prominent treatment than fold-
  into-Axis-13
- Add Maps-to lines to Axes 1, 3, 5, 6, 7 (bounded-mechanical)

Same-cycle review verdict: framework v1.2 is structurally sound and
ready for cycle-38 cross-cycle cold-reader.

## Re-dispatch decision

Eva directive #2794 (firewall expanded for primary-source reads)
authorized re-dispatch of:

- Cognition Devin (Issue #2779 + PR #2780) — held open from cycle 26
- OpenAI harness (Issue #2781 + PR #2783) — held open from cycle 26

Cycle-37 deferred re-dispatch to cycle 38+. Reasoning:

- Framework v1.2 application + cold-reader + same-cycle review consumed
  the substantive focal budget; re-dispatch involves non-trivial
  protocol decisions (close old PRs/issues + create new dispatch
  issues, OR comment on existing dispatch issues to refresh) that
  benefit from cycle-38 explicit research rather than rushed end-of-
  cycle execution.
- The cycle-37 framework v1.2 is the higher-leverage substantive work
  per cycle-36's lean. Re-dispatch is substantive parallel — would-
  have-been-nice-to-have but not load-bearing for v1.2 application.
- Cycle 38 will have a cleaner setup: cold-reader on cycle-37 + cycle-
  37 notes; if cold-reader passes the framework v1.2, cycle 38 can
  shift focus to re-dispatch protocol research + execution as the
  substantive focal.

This is a deferral, not a refusal. Eva's directive #2794 stands; cycle
38 inherits the work.

## Housekeeping: PR #2749 absorption check

Cycle-36 left "PR #2749 (cycle-6 critique) — possibly the 7th deferred
item; verify in cycle 37 whether it's absorbed and close." Per cycle-37
verification:

PR #2749 was the cycle-6 Copilot feedback dispatch on Phase 0
retrospective (Tier-1 absorption per `_notes/cycle-7-copilot-feedback-
evaluation.md`). The findings landed in `0-retrospective.md` cycles 7-9
per-finding evaluations. The 0-retrospective.md is now the canonical
post-absorption artifact.

**Verdict:** absorbed. Close.

(Closing actions executed at end of cycle.)

After cycle-37 sweep: 4 firewall-blocked items remain held-open per
Eva #2794 (Cognition Devin pair + OpenAI harness pair); 0 items in the
deferred-from-prior-cycles backlog.

## Persistence-mechanism observations

### Multi-cycle artifact promotion pattern: notes → dedicated file → Phase-2-input

Cycle-37 promoted the framework from cycle-35/36 notes to dedicated
file `2-design-framework.md`. This is the third instance of the same
pattern in the redesign:

1. **Cycle 30→32 in-place restructure of `1-research.md`**: cycle 30
   draft → cycle 31 verdict file → cycle 32 in-place application.
   3 cycles for the architectural restructure.
2. **Cycle 33 split of `1-research.md` to per-system files**:
   Eva-driven; one cycle for the structural change; cycle-34 cold-
   reader verified.
3. **Cycle 35→37 framework promotion to dedicated file**: cycle 35
   v1.0 in notes → cycle 36 v1.1 deltas in notes → cycle 37 v1.2 in
   dedicated file. 3 cycles for the artifact promotion.

The pattern: design artifact iterates across N cycles in notes, gets
promoted to its own file when stable enough to benefit from a stable
home. Cold-reader cycles between iterations catch errors that same-
cycle review misses. v2 design-input: this multi-cycle promotion
discipline is the load-bearing review primitive; v2 candidates should
preserve it.

### Cross-cycle cold-reader consistently finds direct-vs-indirect distinction errors

Cycle-37's Q(a) finding (F11 → Axis 2 was indirect, miscategorized as
direct) is the same shape as cycle-36's Q(b) finding (F11 → Axis 9
was wrong-axis, identified at strict adversarial reading). Both
findings are about the framework's direct-vs-indirect or specific-vs-
general distinctions:

- Cycle 36 caught: F11 → Axis 9 wrong-axis (Axis 9 doesn't address F11
  at any reading)
- Cycle 37 caught: F11 → Axis 2 indirect-vs-direct (Axis 2 does
  contribute but indirectly)

Both errors slipped through same-cycle review. Both were caught by
cross-cycle cold-reader operating in a different reasoning frame.

**Pattern:** framework integrity has multiple checkpoint layers
(direct-mapping correctness, direct-vs-indirect categorization, cross-
axis dependency completeness). Each layer needs its own pass; same-
cycle review tends to focus on the most recent reasoning layer (in
cycle 35: structural choices; in cycle 36: the F11→Axis 9 correction
itself — which is why it added Axis 2 without thinking about direct-
vs-indirect).

v2 design-input: review-cadence should explicitly include "categorize
contributions: direct vs indirect" as a check, not just "is the
mapping right."

### Framework v1.2 has more honest gaps than v1.1

Cycle-35 v1.0 had no "What the framework does NOT yet specify" section.
Cycle-36 v1.1 added some honest-gap acknowledgments but mostly through
the open-questions list. Cycle-37 v1.2 has an explicit "What the
framework does NOT yet specify" section with 4 named gaps.

Honest gaps grow with iteration depth, not shrink. Each iteration
surfaces dimensions previously assumed solved or ignored. This is
healthy: pretending to solve everything in v1.0 would have been
self-congratulation. Surfacing the gaps explicitly lets cycle-38+
target them.

## What surprised me

That my Q(a) cold-reader finding (F11 → Axis 2 indirect) was BORDERLINE-
FAIL rather than PASS. I had expected cycle-36's correction to be
solid because cycle-36 was itself careful about catching cycle-35's
F11→Axis 9 error. The pattern of "correction-of-correction" is
interesting: each cycle's correction is a new claim that itself can
be over-extended.

The lesson: never assume the most recent cold-reader's verdict is
final. The next cold-reader might refine it further. This is the
iteration-until-approval discipline working as designed; convergence
is asymptotic, not endpoint-defined.

## What I couldn't figure out

Whether to promote `2-design-framework.md` to a Phase-2-input section
in `1-research.md` in cycle-39+ as cycle-36 suggested, or keep it as
a standalone file indefinitely. Arguments either way:

- **Promote to 1-research.md section:** keeps the Phase 1 deliverable
  integrated with the framework that Phase 2 uses. One file to read
  for the complete Phase 1 → Phase 2 transition.
- **Keep standalone:** the framework is a different KIND of artifact
  than 1-research.md (synthesis vs observation). Standalone signals
  this. v2 candidates will read 2-design-framework.md, not need to
  navigate 1-research.md to find it.

Lean toward keeping standalone. The 2- prefix already signals "this
is Phase 2 input" — readers know to look here. Promoting to a section
in 1-research.md re-buries it. But this is a cycle-39+ decision, not
cycle-37's call.

## Cycle-38 plan suggestion (provisional)

Three non-mutually-exclusive options.

1. **Substantive focal: cross-cycle cold-reader on cycle-37 framework
   v1.2.** Three pre-commit checklist questions (below). Verifies
   the framework v1.2 is ready for either: (a) further iteration
   based on findings; (b) standing-as-current-Phase-2-input pending
   eventual checkpoint approval.

2. **Substantive parallel: re-dispatch Cognition Devin + OpenAI
   harness per Eva #2794.** Protocol research first (close old vs
   refresh-existing dispatch), then execute one or both.

3. **Bounded mechanical: cycle-38 backfill items.**
   - Add Maps-to lines to Axes 1, 3, 5, 6, 7 (per same-cycle review
     finding (5))
   - Possibly write `tools/redesign-notes-index` Rust tool (still
     deferred from cycle 36)
   - Verify no other open Eva directive items need attention

Lean toward option 1 + option 2 in cycle 38. Option 3 (a/b) as filler
within budget.

## Pre-commit checklist (for cycle 38's cold-reader)

Three bounded-mechanical questions for the next cycle's cold-reader on
this notes file + the v1.2 framework file:

- **(a)** Is F11 → Axis 4 + Axis 12 (no Axis 2) the right call, or did
  cycle-37 over-correct cycle-36's correction? Spot-check by walking
  F11's mechanism: does Axis 12 (reconciliation) actually address F11's
  load-bearing failure (worklog freeze without refresh) or is it
  stretching to attribute reconciliation discipline? If Axis 12 is a
  stretch, the corrected mapping should be Axis 4 only and Axis 12 is
  a candidate-considered axis only.
- **(b)** Does Axis 13's "thin / medium / fat" trichotomy actually
  constitute candidate-differentiation, or is medium just a "we'll
  figure it out later" placeholder that effectively reduces to fat-vs-
  thin for v2 candidates? Spot-check: name the difference between
  medium-harness and fat-harness in concrete terms (specific procedural
  responsibilities). If you cannot name a clear difference, the
  trichotomy is effectively binary and should be explicit.
- **(c)** Is constraint 8 (goal-driven over operator-driven) genuinely
  a constraint or a tautology? The mission commitment to goal-driven
  means EVERY candidate must take this position — but is "operator-
  driven sub-systems may exist within a goal-driven overall posture"
  really a meaningful constraint, or is it just restating the mission?
  Spot-check by trying to falsify: imagine a v2 candidate that takes
  operator-driven at the top level. Does the constraint actually
  REJECT that candidate, or does it just describe what the surveyed
  systems happen to do?
