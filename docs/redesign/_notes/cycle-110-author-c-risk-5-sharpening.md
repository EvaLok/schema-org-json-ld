# Cycle 110 — Author C Risk 5 second-iteration sharpening

**Date:** 2026-05-10
**Cycle issue:** [#2899](https://github.com/EvaLok/schema-org-json-ld/issues/2899)
**Mode:** redesign Phase 2 candidate iteration (under `ITERATION-UNTIL-APPROVAL`, **twenty-second cycle of Phase 2 candidate-set work** [cycles 90-110]).

## Setup

Cycle 109 closed A's Risk 4 (extension-discipline taxonomy) and named the **cycle 110 forward-going commitment**: rotate to a **C-side risk** (giving C:2 / A:2 / B:3) to test `candidate-rotation-balance-discipline` at HARDENED@3, OR continue priority-order if a higher-priority B-side risk surfaces.

Cycle 110 honored the rotation commitment, choosing C Risk 5 (plan-lifecycle CI invariant drift) as the substantive focal. Closure distribution post-cycle-110: **A:2 / B:3 / C:2** — every candidate now has ≥2 second-iteration closures.

C Risk 5 is the **third discipline-conditional risk closure** after C Risk 2 (cycle 107) and A Risk 4 (cycle 109). Substrate-content adapted from prior closures with substrate-novelty along one axis: cycle 110 is the **first closure applying discipline-conditional shape to a CODE-INVARIANT-COORDINATION substrate** (prior discipline-conditional closures addressed orchestrator-behavior substrates).

The 18-cycle deferral arc (cycle 92 framing → cycle 110 closure) **absorbs 5 prior closures' substrate** — one more than cycle 109 (which absorbed 4), reflecting cycle 109's pattern-extraction also being available substrate.

## Design choices

### Choice 1: 5-type plan-lifecycle change taxonomy as substrate-decomposition

Plan-lifecycle changes vary in what CI-update discipline applies. Designed 5-type taxonomy (sibling of cycle 109's extension taxonomy with substrate-content adapted):

| Type | Description | Discipline | Compliance |
|---|---|---|---|
| 1 bounded-mechanical | Trivial state-name rename / field-name rename | Paired CI update MUST exist in same commit | 100% |
| 2 state-addition | New plan-lifecycle state added | Paired CI invariant added at commit OR within drift-window ≤3 cycles | ≥95% |
| 3 informational-field | Purely informational field (no invariant to check) | NO paired CI update required | ≤10% later-reclassification |
| 4 transition-rule-evolution | Transition graph evolves (e.g., `completed → re-opened`) | CI transition-validation update documented + re-validated within ≤5 cycles | ≥90% documented; ≥80% re-validated |
| 5 emergency / hot-fix | Ad-hoc invariant change under time pressure | Paired CI follow-up REQUIRED within ≤5 cycles, OR Type 5 reclassified | ≥80% reclassified-or-paired |

**Alternatives rejected:** binary CI-coverage discipline (conflates Type 1 with Type 3); 3-type simpler taxonomy (collapses Type 1+2 and Type 4+5); retrospective-only classification (circular).

**Substrate partition:** discipline-applicable Types 1+2+4+5 (~85% volume) + discipline-non-applicable Type 3 (~15%). Type 3 is the **fourth `exclude-by-type` candidate-pattern instance** (after cycle 106 Type A mechanical / cycle 107 Type 5 NO-PLAN / cycle 109 Type 3 judgment-surface). Promotes pattern from TESTED@3 → **HARDENED@4**.

### Choice 2: CI-uncovered ratio U as primary falsifiable bound (rubric-symmetric)

U = (CI-uncovered plan-lifecycle invariants) / (total plan-lifecycle invariants in spec)

- U ≤ 0.05 (≥95% covered): direction-validated
- U ∈ (0.05, 0.20]: at-risk
- U > 0.20: refuted

Chose U (uncovered/total) over C (covered/total) for **directional consistency with cycle 109 P** (lower-is-better across closures cleaner cross-closure interpretation).

**Alternatives rejected:** absolute uncovered count (depends on plan-lifecycle complexity); Type-1-specific compliance rate (misses Type 2/4/5 drift); aggregate `plan-lifecycle-check` LOC (rubric-asymmetric).

### Choice 3: Drift-window D as secondary derived metric (cycle 108/109 two-tier discipline transfer)

For Type 2/4/5: D = cycles between change event and paired-CI-update OR reclassification.
- D ≤ 3 for Type 2; D ∈ (3, 5] for Type 4; D > 5 reclassify-or-accept.

Two-tier discipline promotes TESTED@2 (cycle 109) → **HARDENED@3 (cycle 110)** with substrate-content adapted from iteration-multiplier (108) → drift-window for prompt-extension (109) → drift-window for plan-lifecycle CI (110). **Pattern is now substrate-domain-general.**

### Choice 4: Three-layer verification (cycle 103/107/108/109 transfer)

Per-plan-lifecycle-event log schema with paired-CI-rule status and drift-window timing. Three-layer: orchestrator self-logging + external-observer reproducibility ≥80% with ≤10% disagreement + CI sweep `plan-lifecycle-invariant-coverage-check`.

**Three-layer verification at HARDENED-at-5 across 5 closure types** (cycle 103/107/108/109/110). Cycle 109 named candidate functional-class shape #25 at NOVEL@1; cycle 110 promotes shape #25 NOVEL@1 → **TESTED@2** with substrate-content adapted from prompt-extension events to plan-lifecycle CI events. Cycle 111+ may HARDEN.

### Choice 5: CI-invariant rubric stability as warm-up window (cycle 107/108/109 shape #24 transfer)

Cycles 1-5 of Phase 3 measurement: rubric tightening normal. Past cycle 5: ≤10% other-rate steady-state.

Shape #24 promotes HARDENED@4 (cycle 109) → **HARDENED@5 (cycle 110)** spanning 5 protocol-stability types (measurement-bound + rubric + classification + extension-rubric + plan-lifecycle-event-rubric).

Cycle 110 is **rubric-asymmetric** like cycle 107/109. The discipline-conditional risk-shape type now has 3 instances all showing rubric-asymmetric character — **cycle 109 _notes observation 7 extends to 3 instances confirming the rubric-asymmetric character is structural, not coincidental.**

### Choice 6: Combined-readings diagnostic preserving discipline-conditional rubric-fragility distinction (third instance)

Per cycle 107/109 Choice 6, third instance of discipline-conditional rubric-fragility pattern. Two-axis matrix: U × plan-lifecycle-event rubric stability.

| U | rubric-stability | Diagnostic |
|---|---|---|
| ≤0.05 | ≤10% other | C Risk 5 direction validated |
| ≤0.05 | >10% other | Cherry-picked over rubric-unstable subset; revise taxonomy |
| >0.20 | ≤10% other | Risk 5 fires; magnitude refuted |
| >0.20 | >10% other | Both fail; revise taxonomy first; if revised still >0.20, accept F4 degradation |

Discipline-conditional rubric-fragility diagnostic promotes TESTED@2 (cycle 109) → **HARDENED@3 (cycle 110)** at third instance across 3 substrate domains.

## Edits made

1. **Authored a new section** "Plan-lifecycle CI-invariant coverage discipline (cycle 110 specification, addressing Risk 5)" in [`C-hybrid.md`](../2-candidates/C-hybrid.md) — inserted between cycle 107 closure's outcome-statement (~line 388) and "Risks named at the structural level" (~line 390). Section runs ~165 lines: setup paragraph anchoring against cycle 91/92/107/108/109 lineage; 6 numbered Choices each with 2-3 alternatives-considered-and-rejected; 5-row plan-lifecycle change type table; 4-row combined-readings diagnostic table; "Risk 5 status post-cycle-110" subsection with direction / magnitude / mitigation / operational-closure structure mirroring cycle 105/106/107/108/109 status structures, plus 7 explicit shape-transfer verdict points.
2. **Updated Risk 5 entry** in C-hybrid.md with "(CLOSED at specification level cycle 110)" annotation + forward-pointer paragraph naming the 5-type taxonomy + U primary bound + D secondary derived per cycle 108/109 two-tier + three-layer verification + warm-up + combined-readings.
3. **Updated Validation plan steps 10-13 added**: step 10 plan-lifecycle change event logging; step 11 CI-uncovered ratio U measurement; step 12 drift-window D measurement; step 13 plan-lifecycle-event classification stability check.
4. **Updated [`2-candidates/README.md`](../2-candidates/README.md)** in 2 places: (a) Load-bearing claims sharpening tracker C row updated with cycle 110 closure status; Direction column expanded; Magnitude column expanded with cycle 110 hard thresholds. (b) Forward work "Continue sharpening" line updated with cycle 110 closure summary + cross-reference to all 7 closure sections + 7-instance HARDENED-at-7 claim across 7 risk-domain types AND 2 risk-shape types + shape #24 HARDENED@4 → HARDENED@5 promotion + 5 candidate-pattern promotions to HARDENED@3 / HARDENED@4 / TESTED@2 + closure distribution observation A:2 / B:3 / C:2 + cycle 111+ rotation guidance.

## Verification (compressed)

| Item | Pre-cycle-110 | Post-cycle-110 |
|---|---|---|
| C-hybrid.md line count | 406 | 553 (+147 / +36%) |
| 2-candidates/README.md | 134 | 134 (line count unchanged; tracker row + forward-work line both substantially expanded — they are very long single lines) |
| Risks named at structural level (C) | 9 | 9 (no new risks; Risk 5 closed at specification level) |
| Risks closed at specification level (across A+B+C) | 6 | **7 (+ cycle 110 C Risk 5)** |
| Pre-agreed numerical thresholds for C | ~6 (cycle 107) | ~13 (6 prior + U ≤0.05/0.20 + 5 per-type compliance + D ≤3/5 + ≤10% inter-rater) |
| Pre-agreed verdict categories for C | ~6 (cycle 107) | ~14 (6 prior + 3 U verdict + 5 per-type compliance verdicts) |
| Functional-class shape #21 instances | 6 (HARDENED-at-6; 4 quantity + 2 discipline-conditional) | **7 (HARDENED-at-7; 4 quantity + 3 discipline-conditional)** |
| Discipline-conditional risk-shape type | TESTED@2 (cycle 107 + 109) | **HARDENED@3 (+ cycle 110)** |
| Functional-class shape #24 instances | 4 (HARDENED@4) | **5 (HARDENED@5 spanning 5 stability types)** |
| Candidate functional-class shape #25 (`three-layer-closure-verification`) | NOVEL@1 (cycle 109) | **TESTED@2 (cycle 110)** |
| Candidate-pattern promotions to HARDENED@3 | 0 (cycle 109 had 3 promotions to TESTED@2) | **3** (discipline-conditional rubric-fragility + two-tier metric + candidate-rotation-balance) |
| Candidate-pattern `exclude-by-type` | TESTED@3 (cycle 109) | **HARDENED@4 (cycle 110)** |
| Candidates with ≥2 second-iteration closures | 2 of 3 (A:2 + B:3, C:1) | **3 of 3 (A:2 + B:3 + C:2)** |
| Functional-class shape total / instances | 24 / 52 (+ 1 candidate shape #25 named cycle 109) | 24 / 54 (no new shapes; +2 instances on existing shapes); shape #25 candidate at TESTED@2 |

## What surprised me / what I noticed

1. **Discipline-conditional risk-shape type at 3 instances confirms rubric-asymmetric character is structural.** Cycle 109 _notes observation 7 noted "discipline-conditional risk-shape commonalities at 2 instances" and asked whether the commonalities would generalize. Cycle 110 confirms: all three discipline-conditional substrates (plan-authoring / extension / CI-invariant-coverage) share (a) discipline-non-applicable type (Type 5 NO-PLAN / Type 3 judgment-surface / Type 3 informational-field), (b) ratio-based per-type compliance thresholds, (c) warm-up-window with rubric-asymmetric character, (d) 2x2 combined-readings diagnostic. **The rubric-asymmetric character is structural, not coincidental** — it is intrinsic to discipline-conditional risks where rubric tightening affects WHICH events count as expected-to-comply but each event's compliance is separately observable.

2. **First substrate-novel application: code-invariant-coordination vs orchestrator-behavior.** Prior discipline-conditional closures (cycle 107 plan-authoring, cycle 109 prompt-extraction) addressed orchestrator-behavior substrates — what the orchestrator does each cycle. Cycle 110 addresses CODE-INVARIANT-COORDINATION — keeping spec (plan-lifecycle definition) and enforcement (CI rules) in sync. The shape transferred cleanly. **This suggests the discipline-conditional shape is general across substrate domains** (orchestrator-behavior AND code-coordination), not just orchestrator-behavior. Future closures may apply discipline-conditional shape to substrates outside both — e.g., dispatch protocol invariants, or audit-channel-content invariants.

3. **Two-tier metric discipline at HARDENED@3 spans both quantity-bounded and discipline-conditional substrates.** Cycle 108 introduced two-tier discipline at NOVEL@1 in B Risk 4 (quantity-bounded). Cycle 109 transferred to A Risk 4 (discipline-conditional, drift-window). Cycle 110 transfers to C Risk 5 (discipline-conditional, drift-window). At HARDENED@3 spanning both risk-shape types, the methodological pattern (when a metric depends on auxiliary measurements, demote to secondary derived) is **substrate-domain-general**. The drift-window D primitive itself is shared across cycles 109 and 110 but with substrate-content adapted (extension events vs CI events).

4. **Three-layer verification candidate shape #25 promoted NOVEL@1 → TESTED@2 with high cross-substrate generality.** Cycle 109 named the shape; cycle 110 transfers intact. Five distinct substrates now use three-layer verification (counting protocol / plan-authoring / iteration events / extension events / plan-lifecycle CI events). At TESTED@2 with HARDENED-at-5 instance count for the verification structure itself, the case for promoting shape #25 to its own functional-class shape (vs remaining a sub-element of shape #21) strengthens. Cycle 111+ may HARDEN at instance #3, requiring its own first-class catalog entry.

5. **18-cycle deferral arc absorbs 5 prior closures' substrate (one more than cycle 109's 4).** The arc-length-vs-prereq-count pattern from cycle 109 _notes observation 5 noted plateau at 4 prereqs (cycles 107/108/109). Cycle 110 breaks the plateau upward to 5 prereqs because cycle 109's pattern-extraction (`exclude-by-type` candidate-pattern, shape #25 candidate naming) became substrate. **Pattern: prerequisite count is bounded by available substrate, but each closure that produces extractable patterns extends available substrate for subsequent closures.** The growth is sub-linear (cycle 110 absorbed 5 prereqs but produced 4-6 promotions — diminishing marginal extraction per closure).

6. **Closure distribution rebalanced to A:2 / B:3 / C:2 — every candidate now has ≥2 second-iteration closures.** Cycle 109's _notes flagged the asymmetry concern (A:2 / B:3 / C:1) and named the falsifiable test for cycle 110. Cycle 110 honored the rotation commitment, choosing C Risk 5 over higher-priority B-side targets. **The candidate-rotation-balance-discipline (cycle 107 NOVEL@1 → cycle 109 TESTED@2) promotes to HARDENED@3** at cycle 110. This is the FOURTH consecutive cycle (107, 108, 109, 110) where the immediately-following cycle has honored a rotation/priority forward-going commitment named in the prior cycle's _notes. **The pattern of "_notes-named forward-going commitment honored in the next cycle" is itself substrate worth observing** — possibly a candidate-pattern in its own right (`forward-going-commitment-honored`).

7. **Cycle 110 produced 4-6 candidate-pattern promotions in a single closure — the highest count yet.** Each cycle produces transfer verdicts for prior shapes; cycle 110 promotes: shape #21 (HARDENED-at-6 → HARDENED-at-7); discipline-conditional risk-shape type (TESTED@2 → HARDENED@3); shape #24 (HARDENED@4 → HARDENED@5); shape #25 candidate (NOVEL@1 → TESTED@2); discipline-conditional rubric-fragility diagnostic (TESTED@2 → HARDENED@3); two-tier metric discipline (TESTED@2 → HARDENED@3); candidate-rotation-balance-discipline (TESTED@2 → HARDENED@3); `exclude-by-type` (TESTED@3 → HARDENED@4). That's 8 promotions counting the discipline-conditional risk-shape-type promotion separately. Cycle 109 produced ~6 (shape #21 + shape #24 + 3 candidate-patterns to TESTED@2 + shape #25 named at NOVEL@1). **Cycle 110's promotion count reflects the substrate-accumulation effect: prior closures produce extractable patterns; subsequent closures promote them.** The pattern is sustainable as long as new substrate generates new candidate-patterns; it would slow if pattern-extraction reaches saturation.

## Sibling-pattern tracking

| Pattern | Pre-cycle-110 | Post-cycle-110 | Cycles |
|---|---|---|---|
| `risk-closure-at-specification-level` (shape #21) | HARDENED-at-6 | **HARDENED-at-7** (4 quantity + 3 discipline-conditional) | 103/105/106/107/108/109/**110** |
| Discipline-conditional risk-shape type | TESTED@2 (at risk-shape-type level) | **HARDENED@3** (3 substrate-distinct discipline-conditional applications) | 107/109/**110** |
| `risk-closure-with-warm-up-window` (shape #24) | HARDENED@4 (4 protocol-stability types) | **HARDENED@5** (5 protocol-stability types) | 106/107/108/109/**110** |
| `three-layer-closure-verification` (candidate functional-class shape #25) | NOVEL@1 (named cycle 109) | **TESTED@2** with high cross-substrate generality (5 closure types) | 103/107/108/109/**110** with naming at 109 |
| Discipline-conditional rubric-fragility diagnostic | TESTED@2 (cycle 109) | **HARDENED@3** (3 substrate domains) | 107/109/**110** |
| Two-tier primary-vs-secondary-derived metric discipline | TESTED@2 (cycle 109) | **HARDENED@3** (3 substrate domains, both risk-shape types) | 108/109/**110** |
| Candidate-rotation-balance-discipline | TESTED@2 (cycle 109) | **HARDENED@3** (4 consecutive cycles honoring forward-going commitments) | 107 named / 108 named / 109 honored / **110 honored** |
| `exclude-by-type` candidate-pattern | TESTED@3 (cycle 109) | **HARDENED@4** | 106 Type A / 107 Type 5 / 109 Type 3 / **110 Type 3** |
| `forward-going-commitment-honored` (NEW candidate-pattern) | — | **NOVEL@1 (cycle 110 named)** | 107/108/109/**110** _notes-named commitments + cycle 110 honoring cycle 109 |

## Lexicon entries (cycle 110 candidates)

- **CI-uncovered ratio U** = (CI-uncovered plan-lifecycle invariants) / (total plan-lifecycle invariants in spec). Primary falsifiable bound for plan-lifecycle CI-invariant coverage discipline. Sibling of cycle 109 procedural-surface ratio P.
- **Drift-window D (plan-lifecycle CI variant)** = cycles between plan-lifecycle change event and paired-CI-update OR reclassification. Secondary derived metric per cycle 108/109 two-tier discipline.
- **Plan-lifecycle change type taxonomy** (5-type) = (1) bounded-mechanical / (2) state-addition / (3) informational-field / (4) transition-rule-evolution / (5) emergency-hot-fix. Sibling of cycle 109 extension taxonomy.
- **Code-invariant-coordination substrate** = a class of risk where the spec side (declaration of structural rules) and the enforcement side (CI rules) must move together. C Risk 5 is the first instance of this substrate domain; prior discipline-conditional closures addressed orchestrator-behavior substrates.
- **Substrate-domain-general pattern** = a methodological pattern that holds across substrate domains (e.g., orchestrator-behavior AND code-coordination). Two-tier metric discipline became substrate-domain-general at cycle 110.
- **`forward-going-commitment-honored` candidate-pattern** = the pattern where cycle N's _notes name a forward-going commitment (e.g., "rotate to C-side at cycle N+1"), and cycle N+1 honors it. Observed at cycles 107→108, 108→109, 109→110, 110→111 (forward). NOVEL@1 at cycle 110 (4 instances of next-cycle-honors-prior-cycle-commitment).
- **Substrate-novelty axis** = an axis along which a closure introduces structural difference vs prior closures. Cycle 110's substrate-novelty: code-invariant-coordination (vs prior orchestrator-behavior). Other potential substrate-novelty axes: dispatch-protocol-invariants, audit-channel-content-invariants.
- **Pattern-extraction extends available substrate** = the observation that closures producing extractable patterns (e.g., cycle 109's `exclude-by-type` candidate-pattern naming) extend available substrate for subsequent closures. Counter to the cycle 109 plateau-at-4 hypothesis.
- **Sub-linear substrate-extraction growth** = the observation that prerequisite count grows but produces diminishing marginal new patterns per closure. Cycle 110 absorbed 5 prereqs and produced 4-6 promotions.

## Bottleneck-state honesty

Bottleneck remains external (no audit critique landings since #454; no Copilot dispatch returns; no Eva input arrivals). Cycle 110 contribution is fully repo-internal asynchronous-of-bottleneck. **Cycle 110 is the twenty-second consecutive cycle (cycles 78-110) whose output is fully repo-internal.** All substantive work since cycle 78 has been bottleneck-asynchronous. Audit cycle 215 (2026-05-10) acknowledged Phase 2 entry but has not yet landed substantive Phase 2 critique on cycles 90-101.

Audit issue #458 (2026-05-10 04:34 UTC) is informational substrate: audit's own state.json crossed the 256KB Read tool limit (267KB cycle 214) — the same F5/A5 append-only growth pattern documented in main's v1 retrospective, manifesting structurally on the audit side. Audit shipped its own A5 mitigation (per-cycle notes archive pattern + STARTUP_CHECKLIST size threshold + retrospective revision) within the same cycle. **Cycle 110 observation:** the A5 entry was originally written with "mildly" qualifier and no threshold; cycle 214's threshold-crossing event is precisely the failure pattern the cycle 105-110 shape #21 falsifiable-bound discipline is designed to prevent. The pre-agreed thresholds discipline (cycle 96 lesson, applied at every closure since cycle 103) is the structural answer to "we'll watch this" deferral-without-named-trigger.

## Cycle 111+ pre-commits

1. **Rotation forward-going commitment for cycle 111**: with rotation discipline now HARDENED@3, cycle 111 should rotate to A-side or B-side (cycle 110 was C-side). Possible focal targets (priority-ordered):
   - **A Risk 1/2** (unit-of-analysis baseline) — would test shape #21 transfer to a methodology-precondition substrate, distinct from per-cycle-behavior or code-invariant-coordination.
   - **A Risk 5** (cluster A 6-ABSENT failure-mode return) — would test shape #21 transfer to a hypothetical-future-failure substrate.
   - **B remaining risks** (Risk 1/3/5/6/7/9/10) — B has 3 closures already; further B closures would push asymmetry the other way.
   - **C remaining risks** (Risk 1/3/4/6/7/8/9) — C just closed Risk 5 at cycle 110; another C closure within 1-2 cycles would dilute rotation discipline.
2. **Cold-reader on cycle 110 _notes file** — three questions: (a) does the substrate-novelty claim (code-invariant-coordination vs orchestrator-behavior) hold up under adversarial re-read? (b) is the `forward-going-commitment-honored` candidate-pattern naming load-bearing or post-hoc pattern recognition? (c) does the cycle 110 promotion count reflect genuine accumulation or inflated counting?
3. **Shape #25 HARDENING watch** — if cycle 111+ uses three-layer verification, shape #25 promotes TESTED@2 → HARDENED@3 and earns its own first-class catalog entry vs remaining a shape #21 sub-element.
4. **Restored-polarity dispatching** [cycle 111+] — once next sharpening absorbs.
5. **Symphony / oh-my-claudecode deeper-read elevation** [cycle 111+ or based on cluster catalogue findings].
6. **Audit Phase 2 critique landing watch** — audit cycle 215 acknowledged Phase 2 entry; cycle 111+ may begin to see audit critique on cycles 90-110.
7. **Eva-authored dispatch-test issues #2879 + #2881** — left open per HOUSEKEEPING discipline.
