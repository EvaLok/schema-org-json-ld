# Cycle 93 dispatch: adversarial critique on cycle 91-92 sharpening claims

**Dispatch context:** This is the Copilot feedback-peer critique requested in cycle 93's dispatch
(per issue [redesign-feedback] adversarial critique on cycle 91-92 sharpening claims). I have
read the three candidate files, the README tracker, the design framework, the retrospective,
and — critically — the actual v1 checklist files that Candidate A's sharpening claims to count.
This document applies all seven requested lenses.

**TL;DR before the lenses:** Candidate A's sharpening section contains two verifiable factual
errors (line-count baseline and step-count baseline) that undermine its primary quantitative
claim. Candidate B's baseline is circular (self-measured). Candidate C's F2 numbers are
unanchored estimates. All three validation plans share the same weakness: no hard falsification
threshold. The README tracker creating uniform "structurally validated" status across the three
is false equivalence. Details follow.

---

## Lens 1 — Structural enumeration vs assertion-chain (Candidate A)

### Finding 1: Step-count is not a count — it is an assertion

A's sharpening section (lines 175-181 of `A-evolved-single-orchestrator.md`) states:

> "STARTUP_CHECKLIST.xml currently has ~180 named steps across S0-S15 step IDs (each step is
> a named procedural pattern extractable to a deterministic phase)"
> "COMPLETION_CHECKLIST.xml currently has ~80 named steps across C0-C15 step IDs"
> "Aggregate: ~260 named procedural steps in v1 STARTUP + COMPLETION checklists"

I counted the actual XML `<step>` and `<substep>` element `id` attributes in both files.

**STARTUP_CHECKLIST.xml** (298 lines): **35** named step/substep IDs.
**COMPLETION_CHECKLIST.xml** (432 lines): **15** named step/substep IDs.
**Total: 50 named step IDs.** Not 260.

The ~260 figure is 5.2× the actual named-step count. The sharpening section labels this an
"estimated XML element count from v1 prompt" — but "estimated" is doing enormous work here.
This is not an estimate within ±20% of the real count; it is a figure that appears to have been
constructed from the claim the author wanted to support (~50% extraction = ~130 extractable
steps → therefore the total must be ~260), not from counting.

The cycle 91 summary note (`_notes/cycle-91-candidate-C-and-A-sharpening.md`, lines 36-41)
records the sharpening as: "~15 named patterns extracted from v1 STARTUP_CHECKLIST.xml,
~10 named patterns from v1 COMPLETION_CHECKLIST.xml, ~5 dispatch-poll, ~5 audit-read."
That totals **~35 extractable patterns** — not ~130-160. So even the sharpening author's own
summary of what was done contradicts the 130-160 number in the candidate document.

**Assessment: Assertion-chain.** The ~260 step count is not derived from counting. The 50%
extraction claim has no structurally-grounded denominator.

### Finding 2: Line-count baseline is doubled

A's sharpening section (line 196) claims:

> "v1 prompt (current orchestrator-prompt.xml + STARTUP_CHECKLIST.xml + COMPLETION_CHECKLIST.xml):
> aggregate ~2400 lines (prompt ~1100 lines + STARTUP ~800 lines + COMPLETION ~500 lines)"

Actual file sizes:
- `orchestrator-prompt.xml`: **559 lines** (the retrospective itself, `0-retrospective.md` line 76,
  states "orchestrator-prompt.xml (559 lines, XML)")
- `STARTUP_CHECKLIST.xml`: **298 lines**
- `COMPLETION_CHECKLIST.xml`: **432 lines**
- **Actual total: 1,289 lines.**

The claimed ~2400 is 1.86× the actual size. The component over-estimates: prompt 1100 vs 559
(2×), STARTUP 800 vs 298 (2.7×), COMPLETION 500 vs 432 (1.16×).

The ~67-75% byte-level reduction claim (line 204: "~600-800 lines vs v1's ~2400 lines = ~67-75%
reduction at the byte level") entirely depends on the inflated denominator. Using actual sizes:
600-800 line v2 vs 1289 line actual v1 = **38-53% reduction** at best.
That is not "67-75%." The actual byte-level reduction is structurally unproven once the baseline
is corrected.

**Assessment: Hidden magnitude assumption.** The "~2400 lines" baseline appears to have been
assumed or estimated without checking the actual files, despite the retrospective document in the
same directory giving the correct figures.

### Finding 3: The structural vs byte-level distinction functions as an unfalsifiability hedge

A's sharpening distinguishes "byte-level reduction (~67-75%)" from "named-step-level extraction
(~50-62%)." After applying Lenses 1 and 2 above:
- The byte-level claim is wrong (inflated denominator).
- The named-step-level claim uses a fabricated denominator (~260 vs actual ~50).

The distinction allows the claim to survive falsification on either dimension independently.
If the byte-level number is challenged, the response can be "but the structural extraction
claim stands." If the structural extraction number is challenged, "but byte-level is the
downstream measurement anyway." The two-metric structure does not produce a tighter claim;
it produces a claim that cannot be killed by attacking either metric in isolation.

### Finding 4: Internal inconsistency between body and README tracker

The sharpening section body (line 225) concludes:

> "P3 PASS direction is validated; magnitude is sharpened to **~40-50% rather than ~50%**"

But the README tracker (line 67) records:

> "structurally validated (**~50-62% named-step extraction**)"

These are incompatible ranges. ~40-50% and ~50-62% overlap only at the single point 50%. The
sharpening body lowered the estimate; the tracker was not updated to match. This is the kind
of inconsistency that accumulates when a document is written under time pressure and not
cross-checked. It is not a fatal error in isolation, but it undermines confidence that the
numbers are being tracked with discipline.

### What actually survives Lens 1

The DIRECTION claim — that medium-harness extraction reduces orchestrator-judgment surface — is
structurally plausible. V1's STARTUP automation-note explicitly identifies which steps are
judgment-requiring: "You handle judgment steps: S0.5, S0.6, S1, S1.1, S2, S3, S4, S5, S6,
S7, S8, S9" (12 of 19 top-level step IDs). The medium-harness v2 design would automate S0
more fully and could automate parts of S2, S3, S4, S5. The direction is real. But the
magnitude (~50-62% named-step extraction) is not structurally grounded at any specific
denominator. **Direction: honestly grounded. Magnitude: assertion-chain.**

---

## Lens 2 — Decision-class enumeration vs decision-class-naming (Candidate B)

### Finding 5: The v1 baseline of ~40 decisions is self-measured under incentive

B's sharpening section constructs a v1 baseline (lines 197-210) by enumerating cycle 91's
decisions from the same authoring session that produced the sharpening. The author is the
orchestrator. The measurement was made by the system whose cost is being measured, in the
document arguing that B's per-role cost is acceptable.

The enumeration includes "Per-axis position commitments for C: 12." Are 12 template
applications of the 12-axis framework 12 independent decisions in the cognitive-load sense
meant by "per-cycle decision count"? If you count template-fill as 12 decisions, the v1
baseline inflates. If you count it as 1 structural decision (author C against the framework),
the v1 baseline deflates. The answer changes B's aggregate comparison by a factor of ~2 on
the baseline side.

The sharpening section nowhere acknowledges this measuring-instrument problem. It presents
"~40 decisions (in a candidate-authoring cycle)" as a factual measurement when it is a
retroactive self-categorization.

**Assessment: Assertion-chain.** The baseline is circular: the orchestrator assessed its own
cycle 91 load to compare against its own B design proposal.

### Finding 6: Per-role counts are decision-class names, not decision-class counts

B's per-role enumeration names classes: "Per-artifact-type decisions — what to write where,
how to structure (typically 1-3 artifacts per cycle, each with ~2-3 structural decisions)."
This is a multiplier estimate on top of a multiplier estimate: 1-3 artifacts × 2-3 decisions
each = 2-9 decisions, which then goes into the 10-15 executor estimate. The range of 2-9 is
already wider than stated.

None of the per-role decision classes are enumerated from a concrete artifact list. The
executor's "per-artifact-type decisions" would require naming the specific artifact types
produced in a B-cycle and counting which of those require structural decisions. That work is
not done. What the section does is name categories with multiplier estimates attached, which
is structurally indistinguishable from A's estimate of "~130-160 deterministic-extractable"
— both are top-down multiplier estimates dressed as bottom-up counts.

**Assessment: Decision-class-naming, not enumeration.**

### Finding 7: The "NOT 3-4×" correction is partially a definitional sleight

The P3 PARTIAL-FLAG section in B's first-iteration text used "3-4×" language to describe
risk. The cycle 92 sharpening corrects this by distinguishing sub-shape adoption count from
per-cycle decision count.

The distinction is real and methodologically useful. But it also conveniently rescues B from
its worst self-characterization. The M2 sub-shape adoption count (43 sub-shapes vs v1's 6
STRONG) IS the structural metric for how much design complexity the candidate commits to
maintaining. The "most sub-shapes don't fire every cycle" argument is not validated by any
measured firing rate — it is a claim about bursty behavior asserted without evidence from
cycle history. The correction moves a structural metric ("B commits to 7× v1's adopted
sub-shapes") into a marginal role and replaces it with an unmeasured behavioral claim ("most
don't fire every cycle"). That substitution could be correct, but it is not demonstrated.

**Assessment: Partially honest. The direction refinement is real. The specific "NOT 3-4×"
re-classification rests on an unvalidated firing-rate assumption.**

### Finding 8: Coordination overhead upper bound is open-ended

The coordination overhead estimate of "~9-23 decisions per cycle" has a 2.6× spread. The
section adds this to per-role work to get "~34-67 per cycle, vs v1's ~40 baseline. Lower bound
is below v1; upper bound is ~1.7× v1."

This is correct arithmetic but misleading framing. The full B aggregate including coordination
has a 2× internal range (34 to 67). Saying "comparable to v1" is only true for the lower bound.
The upper bound (67 decisions) is 68% above v1's ~40 baseline. The sharpening section does not
explain what conditions produce upper-bound vs lower-bound scenarios or how likely each is.

A system that in heavy cycles (inbound-heavy, branch-spawn-heavy) runs at 67 decisions against
a v1 baseline of ~40 is not "comparable to v1"; it is substantially worse. Whether heavy cycles
are frequent or rare is not addressed.

**Assessment: Hidden magnitude assumption.** The "comparable to v1" claim should be
re-classified as "ambiguous-without-prototype" until the frequency of upper-bound cycles is
measured.

---

## Lens 3 — Per-failure-mode metrics vs anchored numbers (Candidate C)

### Finding 9: F2 absolute numbers are unanchored estimates

C's sharpening (lines 190-192) states:

> "Reliability: A at ~95% (5% failure due to boot-phase exhaustion before Eva-pull); C at ~99%
> (1% failure due to reconcile-mode boundary edge cases)"

There is no citation for A's ~95% figure. The retrospective's F2 section documents a specific
incident (6+ days of missed background Eva responses) but does not state a general reliability
rate. The ~95% appears to have been derived by assuming boot-phase exhaustion occurs in ~1 of
20 cycles — but this rate is not referenced to any cycle-count or failure-count observation.

C's ~99% is equally unanchored: "1% failure due to reconcile-mode boundary edge cases" assumes
that the edge-case failure rate is known to be roughly half the boot-phase exhaustion rate.
There is no basis for this ratio. Reconcile-mode does not yet exist; its failure modes are
unknown.

The F2 metric is the weakest of the three because the baseline is an unstated estimate and the
C-improvement is a ratio of two estimates. The ~4pp improvement (99% minus 95%) is within the
range of reasonable guess error on either estimate.

**Assessment: Assertion-chain.** Both absolute numbers are anchored guesses presented as
structural derivations. The ~4pp improvement should be re-classified as
"ambiguous-without-prototype" — not direction-validated.

### Finding 10: The F11 baseline is cited but the source is thin

C's sharpening (line 211) gives A's F11 baseline as "~85-90% based on cycle 75-83 cold-reader
observation." Risk 4 acknowledges "Phase 3 prototype paired-cycle measurement may reveal the
cold-reader cycle was already capturing most missed mutations."

This is the most honest single citation in all three sharpening sections. The F11 direction
claim (reconcile-mode is a more reliable detection mechanism than boot-phase polling) IS
structurally grounded: a named phase dedicated to one concern is structurally more reliable than
that same concern interleaved with 8 other responsibilities. The direction is validated.

But the upper bound of the improvement estimate (~13pp, bringing C to ~98%) is not derived;
it is the arithmetic complement of ~2% miss rate. That 2% miss rate for reconcile-mode is as
unanchored as the F2 numbers above.

**Assessment: Direction honestly grounded (F11 is C's strongest claim). Upper bound of
magnitude is an assertion-chain.**

### Finding 11: The F4 "categorical improvement" deserves scrutiny on operational consequence

The F4 categorical claim — "A has implicit lifecycle (zero named states); C has 3 named states"
— is factually true. Zero → 3 is categorical.

But the sharpening uses "categorical" in a way that suggests the improvement is of high
operational significance. Three named states for a plan-lifecycle is the minimum viable taxonomy;
it does not imply the plans are actually managed to those states. The stale-detection lag
improvement (~3-4 cycle reduction) rests on "cycle 87 _notes-aging observation" — a single
cycle's observation generalized to a per-cycle behavioral claim. The claim that A has "~3-5
cycle slip in practice" is cited to one data point.

The categorical fact (0 → 3 states) is real. The operational consequence claim (~3-4 cycle lag
reduction) is extrapolated from a single observation. These are two distinct claims that the
sharpening section presents as one.

---

## Lens 4 — Direction-vs-magnitude discipline honesty

The direction-vs-magnitude discipline is explicitly named and partially well-applied. But:

### Finding 12: A's byte-level claim is presented as magnitude-validated, not direction-validated

A's sharpening (line 204) presents the byte-level reduction (~67-75%) in the same sentence as
the structural extraction (~50-62%) without distinguishing which is direction-level and which
is magnitude-level:

> "~600-800 lines vs v1's ~2400 lines = ~67-75% reduction at the byte level... ~50-62% of
> named-step extraction"

Given the inflated denominator (actual: 1289 lines), the byte-level claim is not even
direction-validated — it is factually wrong in the current v1. This should be re-classified
as "denominator-requires-verification" before applying the direction/magnitude framing.

**Specific magnitude claim to re-classify (Candidate A):** "~67-75% byte-level reduction"
should be re-classified as "indeterminate without verified baseline." Using actual file sizes,
the actual reduction range is 38-53%. The direction (v2 prompt is shorter than v1 total surface)
is honest; the magnitude (67-75%) is not defensible from the actual numbers.

### Finding 13: B's "comparable-to-v1" aggregate should be re-classified

The claim "B aggregate is ~80-110% of v1's [per-role work]" becomes "34-67 vs v1's ~40
including coordination." The lower bound (85%) is comparable; the upper bound (168%) is not.

**Specific magnitude claim to re-classify (Candidate B):** "Aggregate is ~80-110% of v1's" should
be re-classified as "ambiguous-without-prototype (range straddles v1 baseline from 85% to
168%)."

### Finding 14: C's F2 reliability improvement should be re-classified

**Specific magnitude claim to re-classify (Candidate C):** The F2 "~4pp reliability improvement
(A ~95% → C ~99%)" should be re-classified as "ambiguous-without-prototype" because neither
absolute number has a cited source. The structural direction argument (dedicated harness phase
is more reliable) is valid; the specific magnitude requires prototype.

### Finding 15: The discipline bar "direction validated by enumeration" is too low

All three sharpening sections satisfy this discipline if "enumeration" means "listing things in
the candidate document." A lists extracted boot-phase patterns. B lists decision classes per
role. C lists failure modes and improvement mechanisms. Listing is not measuring. The discipline
admits too low a bar: any directional improvement plus some structured list → "direction
validated." There is no check in the discipline for whether the list itself is grounded.

---

## Lens 5 — Risk enumeration adequacy

### Missing risk in Candidate A

The four named risks (incomplete step taxonomy, fuzzy deterministic-vs-judgment boundary,
tool-registry growth, prompt-contract-check CI) all address downstream execution risks.
**Missing: The baseline measurement risk.** The ~260 step count is the foundation of the
50-62% extraction claim. If the actual step count is ~50 (as measured above), the denominator
changes completely: extracting 30 deterministic patterns from 50 total steps = 60% extraction,
while extracting 130-160 from 260 = 50-62%. These happen to produce similar percentages but
for completely different reasons, and the denominator error could produce a very different
percentage if the actual extractable count is also overcounted. A should have named "step
count is an unverified estimate; actual extraction percentage depends on verifying the
denominator" as Risk 0 — the risk that undermines the entire quantitative frame. It doesn't
appear.

### Missing risk in Candidate B

The five named risks cover: incomplete enumeration, coordinator-rule instability, goal-coherence
cost, per-role iteration multiplier, and decision-class leakage. **Missing: Multi-session
wall-clock cost.** B requires 4 sequential AI sessions per cycle (planner → executor → curator
→ reconciler). Each session has cold-start cost: context loading, workspace state
synchronization, tool initialization. This overhead is NOT part of the decision-count metric
(it's wall-clock, not cognitive). If each cold-start costs ~5-10 minutes and there are 4
sessions in a 75-minute window, 20-40% of the cycle budget is consumed before any substantive
work begins. This is a P3-adjacent structural risk that the sharpening section does not name.

### Missing risk in Candidate C

The six named risks are thorough and C's risk section is the most honest of the three. But
**Missing: Workload-confounding in paired-cycle measurement.** C's validation plan requires
"paired cycles: A-prototype vs C-prototype on the same workload." But "same workload" is not
achievable in practice — real cycles have variable inbound events (Eva responses, audit
critique, dispatch outputs). If the A-prototype and C-prototype cycles happen to have different
inbound volumes, the F11 and F2 measurements are confounded. The validation plan names paired
cycles but does not address how to control for workload variance. This is a structural flaw in
the validation methodology, not just a magnitude risk.

### Risk-count vs candidate strength

B has 5 risks and C has 6 risks vs A's 4. This is positively correlated with sharpening
section depth (C's sharpening section is longest and most careful) rather than inversely
correlated with candidate strength. The count itself is not informative about candidate quality.

---

## Lens 6 — "Validation plan" vs validation-plan-naming

### Candidate A: Executable but weakly falsifying

A's validation plan (lines 211-216) names four concrete steps: author draft v2 prompt, measure
line count, author `boot-mode` crate, measure extraction percentage. These ARE executable — a
runbook could be written today.

**Falsification threshold:** "If extraction percentage falls below 40%, the P3 PASS claim
weakens." This is not falsification. 40% is not a refutation of the 50% claim; it is a lower
bound where the claim "weakens." There is no stated outcome that would cause the candidate to
fail P3 outright. A more honest threshold would be: "If extraction percentage is below 35%,
P3 is reclassified to PARTIAL-FLAG regardless of direction." Without a hard threshold, any
prototype outcome can be characterized as "within expected variance."

**Specific paired-cycle metric:** the plan produces one concrete metric: extraction percentage
(actual extracted lines / actual v2 prompt lines). This is measurable. But it does not produce
a metric for whether the orchestrator's decision quality or cycle throughput improves — which is
what P3 actually cares about.

### Candidate B: Named but not executable without counting protocol

B's validation plan (lines 232-238) names five steps, the critical ones being: "run a single
end-to-end cycle" and "count actual per-role decision points per role." The counting step is
NOT executable as stated — there is no defined counting methodology. What counts as one
decision point? If the role-session AI self-reports "I made 12 decisions," that is circular.
If an external observer counts LLM tool invocations, that conflates mechanical steps with
cognitive decisions. The measurement protocol is absent.

**Specific paired-cycle metric:** the plan requires measuring per-role decision count vs
estimate. Without a counting protocol, this metric cannot be produced. The validation plan
names the right outcomes but leaves the measurement instrument undefined.

### Candidate C: Most complete but weakly falsifying

C's validation plan (lines 232-240) names the most concrete metrics: F2 detection latency +
reliability rate, F4 stale-detection lag + promotion quality, F11 detection rate + missed-mutation
count. These are measurable if the paired-cycle measurement is properly controlled.

**Falsification threshold (line 240):** "If F4 + F11 improvements measure within 50% of
estimate-magnitude, central-bet is validated." At 50% of minimum estimate: F11 5pp × 50% =
2.5pp actual improvement sufficient. A 2.5pp improvement in detection rate is likely within
measurement noise over 10-20 paired cycles. This threshold is so permissive that nearly any
positive result validates the central bet.

**A genuinely falsifying threshold would be:** "If F4 stale-detection lag improvement is < 1
cycle and F11 detection-rate improvement is < 3pp, C's central bet is falsified and the design
reverts to A + plans-as-artifacts." The document does name a rollback path but ties it to "below
25% of estimate-magnitude" on ALL three metrics simultaneously — a conjunction that is very hard
to falsify unless all improvements fail together.

---

## Lens 7 — Cross-candidate sharpening-discipline parity

### Finding 16: The README tracker applies uniform status to incompatible evidence types

The tracker (README lines 65-69) labels all three candidates "structurally validated" for
direction. But the evidence types are:

- **A:** Claimed count of XML step IDs (shown above to be ~5× the actual count; direction
  survives but the count is fabricated).
- **B:** Self-constructed decision-class taxonomy with multiplier estimates (no external source;
  circular v1 baseline).
- **C:** One cited source (cycle 75-83 cold-reader for F11), one structural argument (F4
  categorical), one unanchored estimate (F2 ~95% baseline).

Applying "structurally validated" uniformly across these three evidence types creates false
equivalence. C's F11 direction claim is the most honestly grounded. A's direction claim
survives despite the factual errors. B's direction claim rests on a self-constructed taxonomy
with no external validation.

### Finding 17: A's sharpening style creates an appearance of rigor from enumeration

A's sharpening section is the longest and most structured. It enumerates boot-phase
responsibilities, close-phase responsibilities, dispatch-poll responsibilities, audit-read
responsibilities. The enumeration LOOKS like counting. But the "~260 named procedural steps"
number appears in one sentence under "Extracted (estimated XML element count from v1 prompt)"
— and as shown above, that estimate is 5× the actual XML element count.

The sharpening style that favors A is: present a structured list → attach a quantitative claim
to the list → the list's structure lends credibility to the number. B and C do not have as
structured a presentation, which may make them look less rigorous even though C's F11 claim is
better-cited than any of A's numbers.

### Finding 18: A uniformly missing discipline — external falsification criteria

None of the three sharpening sections specifies an outcome that a third party could observe
during Phase 3 prototype and declare "the candidate's load-bearing claim is refuted." A comes
closest (extraction < 40% weakens P3) but this is a soft threshold. B and C have no comparable
threshold. A discipline that should apply uniformly: each sharpening section should state a
hard refutation criterion that is agreed in advance and does not depend on the orchestrator's
own assessment of whether the prototype validates the claim.

---

## Aggregate per-candidate assessment (relative to README tracker)

### Candidate A

- **README tracker:** direction structurally validated (~50-62% named-step extraction),
  magnitude prototype-pending (~40-50% range)
- **Post-critique:** the "~50-62% named-step extraction" direction claim has no grounded
  denominator. The byte-level claim is factually wrong. **The direction is plausible but
  ungrounded; the quantitative validation should be reclassified as
  "estimate-level" pending baseline verification.** This is a demotion, not a validation.
  The direction survives on structural argument (medium-harness extraction is conceptually
  sound); the magnitude is estimate-level at best.

### Candidate B

- **README tracker:** direction structurally validated (per-role ~6-15 vs v1 ~25-40),
  magnitude prototype-pending (aggregate ~80-110% v1)
- **Post-critique:** the per-role counts are decision-class names with multiplier estimates,
  not measured counts. The v1 baseline is self-measured and circularly incentivized. The
  aggregate including coordination is 34-67, with the upper bound 68% above v1. **Direction
  survives: role-specialization genuinely reduces per-role decision-class diversity (this is
  true by construction). The aggregate-comparable-to-v1 magnitude claim should be reclassified
  as "ambiguous-without-prototype (range 85-168% of v1 depending on cycle character)."**

### Candidate C

- **README tracker:** direction structurally validated (F4 + F11 substantial; F2 marginal),
  magnitude prototype-pending (paired-cycle measurement required)
- **Post-critique:** C's internal differentiation (F4 + F11 substantial; F2 marginal) is the
  most honest self-assessment of the three sharpening sections. F4's categorical direction
  claim is solid. F11's direction claim is structurally grounded and the baseline is cited.
  F2's direction claim is plausible but the absolute numbers are unanchored. **C's sharpening
  section survives better than A's or B's under adversarial reading on direction, but the
  validation plan's falsification threshold is too permissive to constitute a genuine test.**
  The README tracker's status for C is the most defensible of the three.

---

## Summary findings (per-lens counts)

| Category | A | B | C |
|---|---|---|---|
| Assertion-chain | 2 (step count, byte reduction) | 2 (decision-class naming, v1 baseline) | 1 (F2 absolute numbers) |
| Honestly grounded | 1 (direction: harness extraction is real) | 1 (direction: role specialization reduces per-role diversity) | 2 (F4 categorical, F11 direction) |
| Hidden magnitude assumption | 1 (2400-line baseline) | 1 (aggregate comparable to v1) | 1 (F11 upper bound) |
| Missing risk | 1 (baseline measurement risk) | 1 (multi-session cold-start cost) | 1 (workload confounding in paired-cycle measurement) |
| Validation-plan vagueness | 1 (no hard falsification threshold) | 1 (no counting protocol) | 1 (threshold too permissive) |
| Internal inconsistency | 1 (body says 40-50%; tracker says 50-62%) | 0 | 0 |
| **Total issues** | **7** | **5** | **5** |

C's sharpening section is the most defensible. A's sharpening section has the most issues,
concentrated in verifiable factual errors (the line counts and step counts can be checked —
and were). B's sharpening section has coherent internal logic but its central validation
(the v1 baseline) is circular.

**The README tracker's "structurally validated" status should be revised to reflect this
differentiation.** C's F4 and F11 direction claims are genuinely structurally validated.
A's Axis 13 direction claim is plausible but ungrounded quantitatively. B's per-role
direction claim is true by construction. These are different kinds of "structurally validated"
and the tracker's uniform label obscures the difference.

---

*Dispatch completed. This critique does not modify any candidate documents, the design
framework, or the README. The findings are for the orchestrator's use in second-iteration
sharpening, specifically: (1) verify A's actual step count against XML before any further
quantitative claims; (2) define B's counting protocol before Phase 3 prototype measurement;
(3) tighten C's falsification threshold before declaring the central bet validated.*
