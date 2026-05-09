---
name: Cycle 107 — Author C Risk 2 second-iteration sharpening
description: Plan-authoring-discipline-conditional risk closed at specification level via 5-type cycle taxonomy + ratio-based compliance thresholds + warm-up-window discipline + discipline-conditional rubric-fragility diagnostic; first instance of shape #21 transfer to discipline-conditional risk-shape type
type: redesign-cycle-note
---

# Cycle 107 — Author C Risk 2 second-iteration sharpening (post-cycle-105/106 substrate absorption); plan-authoring discipline taxonomy + 5-type cycle classification + per-type ratio-based compliance thresholds + stale-active-plan promotion lag bound + three-layer verification + warm-up-window taxonomy stability + discipline-vs-taxonomy combined-readings diagnostic — risk-closure-at-specification-level shape promoted HARDENED-at-3 → HARDENED-at-4 spanning different risk-shape types (3 quantity-bounded + 1 discipline-conditional); shape #24 risk-closure-with-warm-up-window promoted NOVEL@1 → TESTED@2

**Cycle issue:** [#2896](https://github.com/EvaLok/schema-org-json-ld/issues/2896)
**Model:** claude-opus-4-7
**Mode:** redesign Phase 2 candidate iteration (eighteenth cycle of Phase 2 candidate-set work, cycles 90-107).
**Substrate absorbed:** cycle 105 closure shape (substrate-decomposition + falsifiable bound + verification + status); cycle 106 closure shape with warm-up-window discipline; cycle 103 counting-protocol three-layer verification structure (primary unit + reproducibility check + sanity-check secondary).

## Setup

C Risk 2 (cycle 92 framing) named the conditional-improvement concern: F4 stale-detection-lag improvement (~3-4 cycle reduction) is conditional on cycles producing structured plans. Cycles that don't author plans inherit A's F4 behavior — improvement degrades to ~0pp on those cycles. The cycle 92 sharpening identified the conditional but did not specify (a) which cycle-types must author/update plans, (b) what compliance rate counts as "discipline holds", (c) how compliance is measured non-circularly.

The cycle 106 plan named C Risk 2 as cycle 107's substantive focal default with an open structural question: does the `risk-closure-at-specification-level` shape (HARDENED at 3 instances cycles 103/105/106 across substantively different risk-domain types, all quantity-bounded) **transfer to discipline-conditional risks** — a substantively different risk-shape type? If yes, shape #21 extends to 4 instances spanning different risk-shape types in addition to different risk-domain types. If a new shape is required, that's itself a finding.

Deferral arc: 15 cycles (cycle 92 → cycle 107). Compares to cycle 105's 8-cycle arc (97 → 105) and cycle 106's 14-cycle arc (92 → 106). The 15-cycle gap reflects that C's risk closure required cycle 105/106 substrate (the 4-element closure shape; the warm-up window discipline; the ratio-based threshold pattern). Closure-of-X depends on closure-of-prerequisites-of-X.

## Design choices made

### Choice 1: 5-type cycle taxonomy as substrate-decomposition

Cycles vary substantially in what plan-discipline applies. Designed 5-type taxonomy: Type 1 substantive-novel (plan-authoring REQUIRED) / Type 2 substantive-iteration (plan-update REQUIRED) / Type 3 bounded-mechanical (plan-update OPTIONAL) / Type 4 reactive-only (plan-update REQUIRED-or-promotion) / Type 5 no-substantive (NO-PLAN annotation REQUIRED).

**Alternatives rejected:**
- Binary plan-discipline (cycles either author or don't) — conflates Type 1 with Type 3.
- Retrospective-only classification — not falsifiable; classifying after seeing plan-authoring outcome is circular.
- 3-type simpler taxonomy (substantive / mechanical / no-plan-required) — collapses Type 4 reactive into Type 2, losing the external-arrival-driven distinction that matters for plan-authoring discipline (reactive cycles may not affect any active plan if arrival is fully novel).

**Retrospective Phase 2 cycle-distribution:** ~4 Type-1 (90, 91, 93, 94) + ~13 Type-2 (92, 95-106) + 0 Type-3/4/5 in cycles 90-106. Phase 1 cycles 14-89 included substantial Type-3 (bounded-mechanical: 23, 32, 33, 46, 56, 60, 61) and Type-4 (reactive-only: 7, 27, 31, 41, 43, 85) populations.

### Choice 2: Per-cycle-type compliance rate as ratio-based bound

Per cycle 103/106 ratio-based threshold pattern, compliance rates extend the ratio discipline from quantity-ratios to discipline-compliance rates:
- Type 1+2: ≥90% steady-state direction-validated / 70-90% at-risk / <70% refuted
- Type 4: ≥80% steady-state direction-validated / 60-80% at-risk / <60% refuted (lower bar; reactive cycles legitimately have no affected plan if arrival is fully novel)
- Type 5: ≥95% NO-PLAN annotation rate (annotation IS the only discipline)

**Alternatives rejected:**
- Uniform compliance rate across all types — would be either too lenient on Type 1/2 or too strict on Type 4.
- Absolute count thresholds (at-least-N-authoring-cycles-per-N-cycles) — depend on cycle-type distribution; a stretch of mostly Type-3 mechanical cycles would fail an absolute count without indicating discipline failure.

### Choice 3: Stale-active-plan promotion lag bound

Independent F4 metric directly grounding cycle 92's "~3-4 cycle reduction in detection lag" claim:
- Stale detection: plan not updated in ≥5 cycles (default; configurable per plan)
- Promotion-lag bound: ≤3 cycles steady-state (≤3 = validated / 4-6 = supported / >6 = refuted)

**Alternatives rejected:**
- Auto-promote without orchestrator decision — loses discipline benefit (technical-debt becomes dump rather than catalog per M3 S2 EXTENDED).
- Weaker bound (≤5 cycles) — matches A's gardening-sweep baseline, voiding the C-vs-A improvement claim.

### Choice 4: Three-layer verification procedure (annotation + reproducibility + CI)

Each cycle session-end produces structured JSON classification annotation with cycle-type / plan-authoring-activity / active-plans-touched / stale-detected fields. To address self-classification circularity (parallel to cycle 96 self-report concern on B's protocol):
1. Per-cycle annotation by orchestrator (primary).
2. External-observer reproducibility check at Phase 3 measurement (≥20% sample re-classified by Eva, audit, or independent Copilot dispatch; inter-rater disagreement ≤10% steady-state).
3. CI sweep (`cycle-type-annotation-check`) verifying each cycle has annotation; missing annotations themselves count toward Type 5 non-compliance.

**Alternatives rejected:**
- Self-classification alone — exact cycle 96 self-report-circularity concern.
- Session-end summary text without structured annotation — unstructured prose requires LLM-classification per cycle, expensive and itself a rubric-stability concern.

### Choice 5: Cycle-type taxonomy stability as warm-up-window discipline (cycle 106 shape #24 transfer)

Direct application of cycle 106 NOVEL shape #24:
- Steady-state taxonomy stability past cycle 5 of Phase 3 measurement: ≤10% of cycles classified into "other"
- Higher rates in cycles 1-5 (taxonomy-tightening period) expected; sustained "other" >10% past cycle 5 = taxonomy under-specified

Shape #24 promotes from NOVEL@1 (cycle 106 reducer-rule revision rate ≤1/cycle past cycle 5) to TESTED@2 (cycle 107 taxonomy stability ≤10% other-rate past cycle 5).

**Alternatives rejected:**
- Measure stability from cycle 1 with no warm-up — conflates rubric-evolution with structural-instability per cycle 106 reasoning.
- Treat taxonomy as fixed (no stability check) — taxonomy is candidate-authored; without stability check, compliance-rate is uninterpretable when "other" rates are high.

### Choice 6: Discipline-conditional rubric-fragility diagnostic

Discipline-conditional closures depend more heavily on substrate-decomposition correctness than quantity-bounded closures. The cycle-type taxonomy IS the rubric for compliance — if it's wrong, the compliance rate is uninterpretable. This is structurally distinct from cycle 103/106 quantity-bounded closures where rubric ambiguity affects both numerator and denominator equally (ratio-robust): for discipline-conditional, rubric ambiguity affects WHICH cycles count as expected-to-comply but not WHICH cycles complied — a one-sided ambiguity that ratio-based bounds don't fully resolve.

Combined-readings diagnostic table preserves discipline-vs-taxonomy distinction: high compliance + low other-rate = discipline holds; high compliance + high other-rate = taxonomy is wrong (revise before reading); low compliance + low other-rate = discipline refuted; low compliance + high other-rate = both refuted.

**Alternatives rejected:**
- Compliance rate alone — high "other" rate masks discipline failure (cherry-picked compliance over a subset).
- Combine taxonomy stability into compliance rate (count "other" as non-compliant) — conflates two failure modes; loses diagnostic resolution for revising the candidate post-Phase-3.

## Risk 2 status post-cycle-107

- **Direction continues to hold** by construction.
- **Magnitude refined** from "F4 improvement is conditional on cycles producing structured plans" to: 5-type taxonomy / per-type ratio-based compliance / stale-promotion lag ≤3 cycles / three-layer verification / warm-up-window taxonomy stability / combined-readings diagnostic.
- **Mitigation specification is now concrete enough for Phase 3 prototype validation** (taxonomy applied per-cycle from prototype cycle 1; compliance rates measurable by cycle 5+ steady state).
- **Operational closure deferred** to Phase 3 prototype when cycle-type taxonomy meets actual cycle-distribution observations across ≥10 prototype cycles AND per-type compliance rates measured AND stale-promotion lag measured AND inter-rater disagreement measured.
- **Shape #21 transfer verdict:** structural form transfers (substrate-decomposition + falsifiable bound + verification + status); methodological extensions are within-shape sub-applications, not new shapes. **Shape #21 promotes HARDENED-at-3 → HARDENED-at-4** — now spanning different risk-shape types (3 quantity-bounded + 1 discipline-conditional) in addition to different risk-domain types.

## Edits made

1. **Authored a new section "Plan-authoring discipline taxonomy (cycle 107 specification, addressing Risk 2)"** in [`docs/redesign/2-candidates/C-hybrid.md`](../2-candidates/C-hybrid.md) — inserted between "Aggregate central-bet validation" and "Validation plan" sections. The new section runs ~140 lines and contains: a background paragraph anchoring against cycle 92 / 96 / 103 / 105 / 106 lineage; 6 numbered Choices with 2-3 alternatives-considered-and-rejected each; a 5-row cycle-type table (Types 1-5 with discipline expectation + character dependence); a 5-row diagnostic table (compliance × taxonomy-stability); a "Risk 2 status post-cycle-107" subsection with direction / magnitude / mitigation / operational-closure structure mirroring cycle 105/106 status structures; explicit shape #21 transfer verdict and shape #24 promotion at the close.
2. **Updated Risk 2 entry** in C-hybrid.md with "(CLOSED at specification level cycle 107)" annotation + forward-pointer paragraph explicitly naming the 5-type taxonomy + per-type compliance thresholds + stale-promotion lag bound + three-layer verification + warm-up-window discipline + discipline-vs-taxonomy diagnostic. Mirrors cycle 103/106 closure annotation pattern.
3. **Updated Validation plan steps 6-9 added**: step 6 plan-discipline measurement; step 7 stale-promotion lag tracking; step 8 external-observer reproducibility; step 9 cycle-type taxonomy stability check.
4. **Updated [`docs/redesign/2-candidates/README.md`](../2-candidates/README.md)** in 2 places:
   (a) Load-bearing claims sharpening tracker C row — direction column updated with "plan-authoring-discipline-now-specified" alongside existing "mixed evidence quality" prose; magnitude column expanded with cycle 107 hard thresholds.
   (b) Forward work "Continue sharpening" line — cycle 107 closure summary added; explicit cross-reference to all 4 closure sections (cycle 103 / 105 / 106 / 107); 4-instance HARDENED claim across substantively different risk-domain AND risk-shape types; shape #24 NOVEL@1 → TESTED@2 promotion noted; refreshed remaining-targets list (B Risk 4 + A Risk 4); each-candidate-has-at-least-one-closure observation.
5. **Documented findings** in this _notes file (~250 lines, mirroring cycle 106 structure).

## Verification

| Item | Pre-cycle-107 | Post-cycle-107 |
|---|---|---|
| C-hybrid.md line count | 261 | ~395 (+134 / +51%) |
| 2-candidates/README.md line count | 134 | ~135 (+1 net; tracker row + forward-work line both substantially expanded) |
| Risks named at structural level (C) | 9 | 9 (no new risks; Risk 2 closed) |
| Risks closed at specification level (across A+B+C) | 3 (cycle 103 B Risk 8 + cycle 105 A Risk 3 + cycle 106 B Risk 2) | 4 (+ cycle 107 C Risk 2) |
| Pre-agreed numerical thresholds for C | 2 (F4 ≥1 cycle / F11 ≥3pp from cycle 96) | 7 (2 prior + Type 1+2 ≥90%/70-90%/<70% + Type 4 ≥80%/60-80%/<60% + Type 5 ≥95% + promotion lag ≤3/4-6/>6 + taxonomy stability ≤10% + inter-rater disagreement ≤10%) |
| Pre-agreed verdict categories for C | 2 (binary central-bet validation) | 12 (2 prior + 3 Type 1+2 + 3 Type 4 + 1 Type 5 + 3 promotion-lag verdicts) |
| Functional-class shape #21 instances | 3 (HARDENED cycle 103 + 105 + 106) | 4 (HARDENED + cycle 107; first discipline-conditional risk-shape type) |
| Functional-class shape #24 instances | 1 (NOVEL cycle 106) | 2 (TESTED cycle 107) |
| Candidates with ≥1 second-iteration closure | 2 of 3 (A + B) | 3 of 3 (A + B + C) |
| Functional-class shape total / instances | 24 / 46 | 24 / 48 (no new shapes; +2 instances on existing shapes) |

## What surprised me / what I noticed

1. **Shape #21's substrate-decomposition element bends gracefully to non-quantitative substrates.** I expected discipline-conditional risks might require fundamentally different closure structure — perhaps a "discipline-monitoring procedure" shape distinct from the substrate-decomp + bound + verification + status pattern. They don't. Cycle-type taxonomy IS substrate-decomposition; per-type compliance rates ARE falsifiable bounds; per-cycle classification + reproducibility check + CI sweep IS verification procedure; D/M/M/OC IS the status section. The shape transfers cleanly. **The element-form generalizes; only the substrate-content changes.** This is methodologically encouraging — shape #21 may be more general than the cycle 103/105/106 instances suggested, applicable to any risk where some structural property (quantity OR discipline OR something else?) admits a falsifiable bound.

2. **Discipline-conditional closures admit one-sided rubric ambiguity that quantity-bounded closures don't.** Quantity-bounded closures (cycle 103/106) handle rubric ambiguity via ratio-based thresholds — the rubric affects both numerator and denominator equally, so ratios are robust. Discipline-conditional closures (cycle 107) have asymmetric rubric ambiguity: the cycle-type taxonomy affects WHICH cycles are expected-to-comply, but each cycle's compliance/non-compliance is a separable observation. A cycle either authored a plan or didn't — that fact doesn't change with rubric. But whether that cycle was "Type 1" (so authoring expected) varies with rubric. **Choice 6 names this asymmetry and adds the combined-readings diagnostic to preserve discipline-vs-taxonomy distinction.** Methodological observation: discipline-conditional closures need a meta-falsifiability layer (the taxonomy stability check) AND a structural distinction between discipline-failure and taxonomy-wrongness in the diagnostic. Quantity-bounded closures get away with simpler structure because ratio-based-thresholds collapse the distinction. **This is a NEW methodological observation specific to discipline-conditional closures and may be a candidate-functional-shape #25 if it recurs.**

3. **Three-layer verification (orchestrator self-classification + external-observer reproducibility + CI sweep) inherits cycle 103's three-layer pattern intact.** Cycle 103 used: structural-decision-points in role artifacts (primary unit) + external-observer reproducibility check + tool-invocation log (sanity-check secondary). Cycle 107 maps to: per-cycle classification annotation (primary) + external-observer at Phase 3 measurement (reproducibility) + CI sweep (sanity-check / coverage check). **The three-layer pattern is itself a shape that transfers across closure types**, not just the substrate-decomposition. Shape #21's verification element has internal structure inherited from cycle 103 — the cycle 103 protocol's protocol-shape is itself shape #21's verification-element-shape. Methodological observation: shape #21's verification element is sub-structured into (primary + reproducibility + coverage-or-sanity) layers; this is a within-shape-#21 finding rather than a new shape.

4. **Methodological transfer from quantity-bounded to discipline-conditional uses the warm-up-window pattern at the rubric level rather than the bound level.** Cycle 106's shape #24 NOVEL was: pre-agreed warm-up windows for *measurement bounds* (reducer-rule revision rate ≤1/cycle past cycle 5). Cycle 107's TESTED transfers shape #24 to: pre-agreed warm-up windows for *rubric stability* (cycle-type taxonomy ≤10% other-rate past cycle 5). The shape applies to both measurement and rubric — the warm-up window discipline is general across measurement-protocol stability concerns. **Pattern observation:** the warm-up-window discipline applies whenever a measurement protocol has an early period where the protocol itself is being refined, regardless of whether the refinement is at the bound level (cycle 106) or the rubric level (cycle 107). Shape #24's TESTED instance extends its applicability significantly.

5. **The 15-cycle deferral arc (cycle 92 → cycle 107) is the longest of the 4 closures.** Comparison: cycle 103 was 7-cycle (96 → 103); cycle 105 was 8-cycle (97 → 105); cycle 106 was 14-cycle (92 → 106); cycle 107 is 15-cycle (92 → 107). The lengthening arcs reflect that later closures depend on more accumulated substrate — cycle 106 needed cycle 103's rubric; cycle 107 needs cycle 105's closure shape + cycle 106's warm-up window + cycle 103's three-layer verification. **The deferral-arc-as-load-bearing-protection lesson (cycle 103) extends to closure-of-X depending on closure-of-prerequisites-of-X across multi-step chains.** The chain for cycle 107: cycle 92 C Risk 2 authoring → cycle 96 absorption surfacing soft-language concerns → cycle 103 B Risk 8 closure providing three-layer verification template → cycle 105 A Risk 3 closure providing 4-element shape template → cycle 106 B Risk 2 closure providing warm-up-window discipline → cycle 107 C Risk 2 closure absorbing all four prerequisites. Each step is a cycle in the chain. **Closure quality may genuinely improve with longer arcs** when the arc is genuinely accumulating substrate rather than procrastinating.

6. **Each candidate (A/B/C) now has ≥1 second-iteration closure.** Pre-cycle-107: A had 1 closure (Risk 3 cycle 105) / B had 2 closures (Risk 8 cycle 103, Risk 2 cycle 106) / C had 0 closures. Cycle 107: A 1 / B 2 / C 1. The candidate rotation B → A → B → C balances falsifiability completeness rather than widening asymmetry. **Cycle 108+ candidate rotation** would naturally continue: B → A → B → C → ? — the next sharpening target by priority is B Risk 4 (per-role-iteration single-pass assumption), which would give B a third closure. If the rotation discipline holds across cycles 108+, the order would be B → A (giving A its second closure with Risk 4) before returning to B Risk 4. **Pattern observation:** the rotation discipline is itself emergent (it wasn't pre-planned at cycle 92; it emerged from the cross-candidate balance imperative cycles 105+). This may be worth naming as an emerging discipline.

7. **Self-classification circularity is the cycle-107 analog of cycle-96 self-report concern on B.** Cycle 96 named "self-report by the role-session AI is circular" as Risk 7 / Risk 8 substrate on B. Cycle 103 closed it via external-observer enumeration. Cycle 107's Choice 4 reasoned through the same circularity for C's per-cycle classification annotation: orchestrator that authored the plan-discipline taxonomy is the same orchestrator measuring compliance. The mitigation transfers: external-observer reproducibility check at Phase 3 sample re-classification. **Methodological observation:** the self-report-circularity concern arises whenever the closure measurement is performed by the same agent that authored the closure. The cycle 103 mitigation (external observer with reproducibility check) is a general pattern for this concern, transferring across measurement contexts. This is a cross-closure methodological pattern — circularity-mitigation-via-external-observer — that has 2 instances (cycle 103 + cycle 107) and may be worth naming as a methodological discipline rather than a closure-shape sub-element.

## Standing tasks

- **PR / issue closures from cycle 107:** none (cycle 107 is repo-internal; no PRs absorbed).
- **Cluster catalogue update:** RECENT (cycle 102 rebuild + cycle 104 cross-cluster integration). No new system absorption since cycle 101.
- **Author C's plan-authoring-discipline-conditional (Risk 2)** [completed cycle 107, specification-level closure].
- **Second-iteration sharpening on B's per-role-iteration risk (Risk 4)** [cycle 108+, MEDIUM-HIGH PRIORITY] — gives B its third closure; rotation discipline supports this as the next target.
- **Second-iteration sharpening on A's prompt-contract-check regression risk (Risk 4)** [cycle 109+, MEDIUM PRIORITY] — would give A its second closure, balancing the cross-candidate distribution.
- **Restored-polarity dispatching** [cycle 108+] — once next sharpening absorbs.
- **Symphony deeper-read elevation** [cycle 108+ or based on cluster catalogue findings] — first-pass at cycle 98; could resolve cluster F.9 candidate status.
- **oh-my-claudecode deeper-read elevation** [cycle 109+] — first-pass at cycle 99.
- **Eva-authored dispatch-test issues #2879 + #2881** — left open per HOUSEKEEPING discipline (Eva-authored, "absorbed" status uncertain).
- **MEMORY.md initialized cycle 107** — first multi-session persistence artifact for this project. 7 memory files + index. Future cycles will iterate on what's worth persisting.

## Sibling pattern tracking

- **Functional-class shape #21 (`risk-closure-at-specification-level`)** — cycle 103 NOVEL → cycle 105 TESTED at 2 → cycle 106 HARDENED at 3 → **cycle 107 HARDENED at 4 spanning different risk-shape types**. The shape's structural form holds across:
  - Cycle 103 B Risk 8: meta-counting-protocol (quantity-bounded, HOW to measure).
  - Cycle 105 A Risk 3: tool-registry-growth (quantity-bounded, WHAT grows).
  - Cycle 106 B Risk 2: coordination-overhead-stability (quantity-bounded, WHEN/WHY coordination grows).
  - Cycle 107 C Risk 2: plan-authoring-discipline-conditional (**discipline-conditional**, WHEN/HOW discipline holds).
- **Functional-class shape #24 (`risk-closure-with-warm-up-window`)** — cycle 106 NOVEL @ 1 → **cycle 107 TESTED @ 2**. The shape's structural form (pre-agreed warm-up window + pre-agreed post-warm-up threshold) holds across measurement-bound stability (cycle 106 reducer-rule revision rate) AND rubric stability (cycle 107 cycle-type taxonomy other-rate). Shape applies wherever a measurement protocol admits an early refinement period.
- **Pre-agreed-falsification-threshold discipline** — HARDENED at 3+ → cycle 107 application: 7 new pre-agreed thresholds with ratio-based form (Type 1+2 ≥90%/70-90%/<70%, Type 4 ≥80%/60-80%/<60%, Type 5 ≥95%, promotion lag ≤3/4-6/>6, taxonomy stability ≤10%, inter-rater disagreement ≤10%).
- **Verification-discipline pattern (cycle 96 emergence)** → 6+ instances HARDENED. Cycle 107 NOT a new instance (no PR verification this cycle).
- **Generalization-level discipline (J-Q(a))** → 19+ instances HARDENED. Cycle 107 application: each Choice 1-6 explicitly names what it generalizes-from (cycle 92 framing, cycle 96 self-report concern, cycle 103 three-layer verification, cycle 105 closure shape, cycle 106 warm-up window).
- **Direction-vs-magnitude discipline** → 11+ instances HARDENED. Cycle 107 application: Risk 2 closure preserves direction (F4 conditional on discipline holds), refines magnitude.
- **Cycle-composition-polarity discipline (cycle 62 emergence)** → 41+ instances HARDENED. Cycle 107 application maintains: substantive focal advances candidate sharpening corpus.
- **Audit-as-peer pattern** — 2-instance evidence holds. No new audit absorption cycle 107.
- **NEW candidate-pattern: discipline-conditional-rubric-fragility-diagnostic** — observed cycle 107 Choice 6. Discipline-conditional closures have one-sided rubric ambiguity (rubric affects expected-to-comply but not did-comply), requiring meta-falsifiability + combined-readings diagnostic. NOVEL @ 1 instance. May be candidate functional-class shape #25 if it recurs.
- **NEW candidate-pattern: circularity-mitigation-via-external-observer** — observed at 2 instances (cycle 103 B counting protocol + cycle 107 C cycle-classification annotation). Pattern: when closure measurement is performed by the agent that authored the closure, external-observer reproducibility check is the circularity-mitigation. May be a methodological discipline rather than a shape. TESTED @ 2 if formalized.
- **NEW candidate-pattern: candidate-rotation-balance-discipline** — observed across cycles 105 → 106 → 107 (B → B → C). Sharpening rotation balances falsifiability completeness across A/B/C. Emerged from cross-candidate balance imperative cycles 105+. NOVEL @ 1 instance (cycle 107 first explicit naming).

## Lexicon entries (cycle 107)

- **5-type cycle taxonomy** — Type 1 substantive-novel / Type 2 substantive-iteration / Type 3 bounded-mechanical / Type 4 reactive-only / Type 5 no-substantive. Substrate-decomposition for plan-authoring discipline.
- **Plan-authoring discipline expectation** — per-type expectation: REQUIRED (Type 1+2) / OPTIONAL (Type 3) / REQUIRED-or-promotion (Type 4) / NO-PLAN-annotation REQUIRED (Type 5).
- **Compliance rate as discipline-conditional ratio** — % of expected-to-comply cycles meeting per-type discipline. Extends cycle 103/106 ratio-based thresholds from quantity-ratios to discipline-compliance rates.
- **Stale-active-plan promotion lag** — cycles between detected staleness (active plan not updated ≥5 cycles) and orchestrator decision. Pre-agreed bound ≤3 cycles steady-state.
- **Three-layer verification procedure (extended)** — orchestrator self-classification (primary) + external-observer reproducibility (≥20% sample, ≤10% disagreement steady-state) + CI sweep (coverage). Inherits cycle 103 three-layer pattern.
- **Cycle-type taxonomy stability check** — pre-agreed warm-up window cycles 1-5 + pre-agreed post-warm-up threshold ≤10% other-rate. Direct application of cycle 106 NOVEL shape #24.
- **Discipline-conditional rubric-fragility** — closures of discipline-conditional risks have one-sided rubric ambiguity that ratio-based bounds don't fully resolve; requires meta-falsifiability check + combined-readings diagnostic. Distinct from quantity-bounded closure rubric ambiguity.
- **Combined-readings diagnostic (compliance × taxonomy-stability)** — 4-quadrant table preserving discipline-failure vs taxonomy-wrongness distinction. High compliance + low other-rate = validated; high + high = taxonomy wrong; low + low = discipline refuted; low + high = both refuted.
- **Risk-shape type vs risk-domain type** — closure shape transfer evidence: shape #21 spans 4 risk-domain types AND 2 risk-shape types (3 quantity-bounded + 1 discipline-conditional) at cycle 107.
- **Closure-of-X depends on closure-of-prerequisites-of-X** — multi-step deferral arcs accumulate substrate; later closures absorb earlier closure templates + verification structures + warm-up-window patterns. Cycle 107's 15-cycle arc absorbs cycles 96+103+105+106 prerequisites.

## Bottleneck-state honesty

Bottleneck remains external (no audit critique landings; no Copilot dispatch returns; no Eva input arrivals). Cycle 107 contribution is fully repo-internal asynchronous-of-bottleneck. **Cycle 107 is the nineteenth consecutive cycle (cycles 78-107) whose output is fully repo-internal** — though cycle 100/101 PR closures and cycle 100 MEMORY.md cross-runner work were repo-internal, all substantive work since cycle 78 has been bottleneck-asynchronous. Cycle 107 also initialized this orchestrator's MEMORY.md (7 memory files + index) — first multi-session persistence artifact for this project; per `PERSISTENCE` directive in redesign prompt SECTION 7 ("Solve this. Design your own persistence mechanism.").

## Honest reflection (per F1-F5 correctives)

Cycle 107 is **1 substantive activity** (C Risk 2 second-iteration sharpening at specification level) yielding 4 edit locations in C-hybrid.md (~140 lines net) + 2 edits in 2-candidates/README.md (~3 lines net but substantially expanded prose) + 1 _notes documentation file (~250 lines) + 7 MEMORY.md files (~200 lines combined) + 1 journal entry. Per F1 corrective: documentation IS the activity's deliverable form. Per F2 corrective: Risk 2 closure preserves direction (F4 improvement conditional on discipline holds); magnitude refined from "conditional on cycles producing structured plans" to specific bounds + rates + verification + warm-up + diagnostic — direction was already structurally clear; magnitude was the falsifiability gap closed. Per F3 corrective: 24 functional-class shapes at 48 instances (no new shapes; +2 instances on existing shapes — shape #21 and shape #24). Per F4 corrective: NOVEL/TESTED/HARDENED labels reserve for redesign-process methodology — shape #21 promotion to HARDENED-at-4 spanning different risk-shape types is a methodology-pattern claim, not a C-correctness claim; C Risk 2 itself remains open at the operational level until Phase 3 measurement. Per F5 corrective: 10 lexicon entries focused on the 5-type taxonomy + per-type expectations + compliance rate + promotion lag + three-layer verification + warm-up window + discipline-conditional fragility + combined-readings diagnostic + risk-shape vs risk-domain distinction + closure-arc dependency chain. Per H-Q(a) anti-inheritance: cycle 107 closure grades empirical-observation-grounded against cycle 92 risk authoring + cycle 96 self-report-concern + cycle 103 three-layer verification template + cycle 105 4-element closure shape template + cycle 106 warm-up-window discipline. Per J-Q(a) generalization-level discipline: HARDENED at 19+ instances; each Choice 1-6 explicitly names what it generalizes-from without over-generalizing.

## Pre-commit checklist

- [x] C-hybrid.md closure section authored, ~140 lines, 6 Choices with alternatives-considered-and-rejected
- [x] C-hybrid.md Risk 2 entry annotated with closure forward-pointer
- [x] C-hybrid.md Validation plan extended with cycle 107 measurement steps 6-9
- [x] 2-candidates/README.md tracker C row updated with cycle 107 closure status + hard thresholds
- [x] 2-candidates/README.md Forward work line updated with cycle 107 closure summary + 4-instance shape #21 transfer claim + shape #24 NOVEL→TESTED
- [x] _notes/cycle-107-author-c-risk-2-sharpening.md authored with cycle 106 _notes structure
- [x] MEMORY.md + 7 memory files initialized for first-time multi-session persistence
- [ ] Journal entry for cycle 107 (next step)
- [ ] Commit + push (next step)
- [ ] Session-end summary on cycle issue #2896 (next step)
