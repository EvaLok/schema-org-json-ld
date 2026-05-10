---
name: Cycle 108 — Author B Risk 4 second-iteration sharpening
description: Per-role-iteration single-pass assumption risk closed at specification level via 5-trigger iteration decomposition under B's architectural elements + per-cycle iteration event count as primary falsifiable bound + iteration multiplier as secondary derived + three-layer verification + warm-up-window classification stability + combined-readings diagnostic preserving Risk-1-vs-Risk-4 distinction; shape #21 HARDENED-at-4 → HARDENED-at-5 (5 quantity-bounded instances spanning 5 risk-domain types); shape #24 TESTED@2 → HARDENED@3
type: redesign-cycle-note
---

# Cycle 108 — Author B Risk 4 second-iteration sharpening (post-cycle-103/106/107 substrate absorption); 5-trigger iteration decomposition + per-cycle iteration event count + iteration multiplier secondary metric + three-layer verification + warm-up-window classification stability + combined-readings diagnostic preserving Risk-1-vs-Risk-4 distinction — risk-closure-at-specification-level shape promoted HARDENED-at-4 → HARDENED-at-5 spanning 5 risk-domain types (4 quantity-bounded + 1 discipline-conditional); shape #24 risk-closure-with-warm-up-window promoted TESTED@2 → HARDENED@3 spanning measurement-bound + rubric + classification stability

**Cycle issue:** [#2897](https://github.com/EvaLok/schema-org-json-ld/issues/2897)
**Model:** claude-opus-4-7
**Mode:** redesign Phase 2 candidate iteration (nineteenth cycle of Phase 2 candidate-set work, cycles 90-108).
**Substrate absorbed:** cycle 103 counting-protocol three-layer verification structure (primary unit + reproducibility check + sanity-check secondary) + cycle 106 closure shape (substrate-decomposition + falsifiable bound + verification + status with mechanical-vs-structural Type A exclusion) + cycle 107 closure shape with warm-up-window discipline (shape #24) + cycle 107 combined-readings diagnostic (Choice 6 pattern preserving discipline-vs-taxonomy distinction transferred to Risk-1-vs-Risk-4 distinction).

## Setup

B Risk 4 (cycle 92 framing) named the single-pass assumption concern: cycle 92's per-role decision count estimate (~6-15 per role per cycle) assumes each role's session is invoked once per super-step. If within-super-step iteration occurs (e.g., curator review forces executor re-invocation), per-role decision count multiplies. The cycle 92 sharpening identified the conditional but did not specify (a) what triggers within-super-step iteration, (b) how iteration events are counted, (c) what iteration rate counts as "single-pass-approximation holds" vs "iteration is structural", (d) how Risk 4 firing is distinguished from Risk 1 firing (per-role decision-class enumeration incomplete) when both manifest as aggregate cost growth.

The cycle 107 plan named B Risk 4 as cycle 108's MEDIUM-HIGH PRIORITY substantive focal default with an open structural question: does the `risk-closure-at-specification-level` shape (HARDENED at 4 instances cycles 103/105/106/107 across substantively different risk-domain types AND 2 risk-shape types) **transfer to a per-role-iteration risk-domain type** (the 5th risk-domain type after meta-counting-protocol / tool-registry-growth / coordination-stability / plan-authoring-discipline)? If yes, shape #21 extends to 5 instances spanning 5 risk-domain types in addition to 2 risk-shape types. The cycle 108 closure also tests whether shape #24 (warm-up-window) extends from rubric stability (cycle 107) to classification stability (iteration-event inter-rater disagreement; cycle 108).

Deferral arc: 16 cycles (cycle 92 → cycle 108). Compares to cycle 105's 8-cycle arc (97 → 105), cycle 106's 14-cycle arc (92 → 106), and cycle 107's 15-cycle arc (92 → 107). The 16-cycle gap reflects that B's Risk 4 closure required cycle 103's three-layer verification template + cycle 106's mechanical-vs-structural decomposition + cycle 107's warm-up-window discipline + cycle 107's combined-readings diagnostic structure. Closure-of-X depends on closure-of-prerequisites-of-X across multi-step chains; cycle 108's 16-cycle arc is the longest of the 5 closures, and the chain of substrate spans 4 of the 5 prior closure prerequisites.

## Design choices made

### Choice 1: 5-trigger iteration decomposition under B's architectural elements

Within-super-step iteration decomposes naturally along B's architectural elements: curator-review (cluster H sub-shape 2 score-gating) / goal-coherence-judgment (per super-step boundary) / reconciler-arrival (Axis 12 inbound channels) / plan-lifecycle (Axis 5 plans-as-artifacts state transitions) / branch-spawn (Axis 4 branching checkpoints). Each architectural element produces a structurally-distinct iteration trigger:
- α curator-review-driven (curator's score-gating triggers executor re-invocation)
- β goal-coherence-redirect (planner's goal-coherence judgment triggers upstream role re-invocation)
- γ reconciler-triggered (inbound event arrival triggers planner re-plan + dependent role re-iteration)
- δ plan-lifecycle (plan-promotion decision triggers planner re-plan + executor re-iteration)
- ε branch-spawn (branch-spawn decision triggers per-role parallel iteration on branch)

**Alternatives rejected:**
- Single uncategorized "iteration overhead" number — conflates structural (curator-review, goal-coherence) with bursty external (reconciler-arrival, branch-spawn), losing diagnostic resolution.
- 6th type: intra-LLM-session backtracking (token-level revision within a single role-session pass) — Risk 4's framing is explicit about cross-super-step iteration; intra-pass revision is part of single-pass cognitive work already counted.
- 3-type simpler taxonomy (structural / external / branch) — collapses α+β into a single "internal" type, losing the curator-review-vs-goal-coherence distinction that matters for which role re-iterates.

**Substrate partition:** structural-invariant Types α + β (≤4 events/cycle bounded by structure: curator runs once + ≤3 goal-coherence judgments) + structural-character-dependent Types γ + δ + ε (~0-6 events/cycle, depends on inbound volume + plan-lifecycle activity + branch-spawn cycle character). The partition mirrors cycle 106's mechanical-vs-structural Type A exclusion: cycle 106 excluded Type A as mechanical; cycle 108 includes all 5 types but partitions structural-invariant from character-dependent.

### Choice 2: Per-cycle iteration event count as primary falsifiable bound (rubric-stable)

Per cycle 103/106/107 ratio-based threshold pattern adapted: iteration event count (sum of α + β + γ + δ + ε events per cycle) is rubric-stable (an iteration event is structurally distinguishable: a role's session is re-invoked within the same super-step, OR a role's super-step output is reverted and re-authored). Pre-agreed thresholds:
- ≤3 events/cycle: direction-validated steady-state. Risk 4's "if iteration occurs" antecedent fires rarely.
- (3, 6] events/cycle: at-risk. Iteration occurs occasionally; B's central bet weakened.
- >6 events/cycle: refuted. Iteration is structural; B's central bet undermined.

**Alternatives rejected:**
- Ratio-based bound (iteration events / per-role decisions) as primary — per-role decision counts vary substantially across cycles (planner ~6-10 / executor ~10-15 / curator ~6-12 / reconciler ~3-7 per cycle 92), making the ratio sensitive to denominator's cycle-character variation. Absolute count is rubric-stable AND cycle-character-stable for structural-invariant subset.
- Multiplier as primary — conflates iteration-event count with per-event-decision-count; refutation on multiplier alone would not distinguish "many small iterations" from "few large iterations" (diagnostically equivalent under multiplier, structurally distinct under event-count).

### Choice 3: Iteration multiplier as secondary derived metric (informational)

For relating iteration events to aggregate cost growth, iteration multiplier M = (aggregate decisions including iteration events) / (aggregate decisions assuming single-pass):
- M ≤1.3: aggregate cost growth from iteration is bounded; consistent with steady-state ≤3 events/cycle if average per-event-decision-count is ~3-5.
- M ∈ (1.3, 1.8]: significant but bounded.
- M >1.8: substantial; iteration drives aggregate cost growth.

Multiplier is **secondary derived (informational)**, NOT primary falsificational, because:
- M depends on per-event-decision-count which varies by event type (α executor re-pass ~10-15; γ planner re-plan ~6-10; ε branch-spawn variable).
- Cycle 103's aggregate threshold (≤1×, ≤1.5×, ≤2×) already captures aggregate cost growth from any source.

**Alternatives rejected:**
- Make multiplier primary falsifiable — conflates two separate measurements (event count + per-event-decision-count); refutation thresholds on M would lose diagnostic resolution.

### Choice 4: Three-layer verification (per-iteration-event log + external-observer reproducibility + CI sweep)

Structured iteration-event log schema:
```jsonc
{
  "type": "curator-review" | "goal-coherence-redirect" | "reconciler-trigger" | "plan-lifecycle" | "branch-spawn",
  "triggering-role": "planner" | "executor" | "curator" | "reconciler",
  "iterating-role": "planner" | "executor" | "curator" | "reconciler",
  "trigger-cause": "<cycle-event description>",
  "cycle": <cycle-id>,
  "super-step-of-trigger": <super-step-id>,
  "super-step-of-iteration": <super-step-id>,
  "decision-count-of-iteration": <integer>
}
```

Three-layer verification (inheriting cycle 103 / 107 pattern intact):
1. Per-iteration-event logging by role-session (primary).
2. External-observer reproducibility check at Phase 3 measurement (≥20% sample, ≤10% disagreement on event type past cycle 5 of measurement).
3. CI sweep (`iteration-event-log-coverage-check`) verifying iteration-event log coverage (each cycle's role-session re-invocations must have corresponding log entry).

**Alternatives rejected:**
- Rely on prompt-counting alone — iteration events are operational events that don't show up in prompt-counting (capability vs actuality conflation; same as cycle 106 Type-E reasoning).
- Instrument all role-session invocations rather than iteration-specific logs — cycle 103 secondary unit (tool-invocation log) already serves the per-role decision-count sanity-check; iteration-specific logs are tighter (only iteration events, not all session invocations) and don't introduce new instrumentation conflicting with cycle 103 secondary.

### Choice 5: Iteration-event-classification rubric stability as warm-up window discipline (cycle 107 shape #24 transfer)

Direct application of cycle 107 NOVEL@1 → TESTED@2 shape #24:
- Warm-up window: cycles 1-5 of Phase 3 measurement. Iteration-event-classification rubric is being tightened (what counts as Type α event vs intra-pass continuation; whether Type γ triggered by audit-post-text-processing counts as iteration or initial reading).
- Steady-state: past cycle 5 of measurement. Bounds apply (≤3 / (3, 6] / >6). Inter-rater disagreement on event type ≤10% expected steady-state.
- Combined-readings against rubric stability: if past cycle 5, inter-rater disagreement remains >10%, iteration-event rubric is under-specified; bounds become uninterpretable until rubric is tightened.

Critical methodological observation: this is a quantity-bounded application of shape #24, NOT a discipline-conditional rubric-fragility instance. Iteration-event count is rubric-symmetric (numerator and denominator are both events; rubric tightening affects both equally). Cycle 107's discipline-conditional pattern (one-sided rubric ambiguity) remains NOVEL@1.

Shape #24 promotes TESTED@2 (cycle 107 cycle-type taxonomy stability) → **HARDENED@3 (cycle 108 iteration-event-classification stability)** spanning 3 stability types: measurement-bound stability (cycle 106) + rubric stability (cycle 107) + classification stability (cycle 108).

**Alternatives rejected:**
- Measure iteration events from cycle 1 with no warm-up — conflates rubric-evolution with iteration-instability per cycle 106/107 reasoning.
- Treat iteration-event rubric as fixed (no stability check) — without stability check, event count is uninterpretable when inter-rater disagreement is high.

### Choice 6: Combined-readings diagnostic preserving Risk-1-vs-Risk-4 distinction

Risk 1 (per-role decision-class enumeration incomplete) and Risk 4 (single-pass assumption) both manifest as aggregate cost growth above cycle 92 estimate. Without combined-readings, refutation at aggregate cannot distinguish causes. Per cycle 107 Choice 6 combined-readings pattern, a 4-quadrant matrix:

| Per-role decision count vs cycle 92 estimate | Iteration events per cycle | Diagnostic |
|---|---|---|
| Within range (~6-15 per role) | ≤3 events/cycle | B fully validated |
| Within range | >6 events/cycle | Risk 4 fires; iteration drives growth — revise iteration-event protocol or accept higher aggregate |
| Above range (>15 per role) | ≤3 events/cycle | Risk 1 fires; per-role enumeration was incomplete — revise per-role decision-class enumeration |
| Above range | >6 events/cycle | Both Risk 1 and Risk 4 fire; B's central bet refuted — revisit candidate or accept aggregate >2× v1 |

**Alternatives rejected:**
- Aggregate cost as the only metric — cannot distinguish Risk 1 from Risk 4; refuted aggregate verdict would not localize cause.
- Use coordination ratio (cycle 106 Choice 5) to capture iteration — coordination ratio measures cross-role coordination overhead (Type B/C/D/E coordination decisions per cycle 106), not within-super-step iteration. Orthogonal — a cycle can have low coordination overhead (clean super-step boundaries) AND high iteration (curator review forces multiple executor re-invocations).

## Risk 4 status post-cycle-108

- **Direction continues to hold** by construction.
- **Magnitude refined** from "if sub-tasks require iteration, per-role decision count multiplies" to: 5-trigger decomposition / structural-invariant floor ≤4 events Types α+β / structural-character-dependent ceiling ~0-6 events Types γ+δ+ε / pre-agreed event-count thresholds ≤3 / (3, 6] / >6 / iteration multiplier secondary derived M ≤1.3 / 1.3-1.8 / >1.8.
- **Mitigation specification is now concrete enough for Phase 3 prototype validation** (event-count bounds and combined-readings matrix measurable from cycle 1 of Phase 3, rubric stability checked steady-state past cycle 5).
- **Operational closure deferred** to Phase 3 prototype when per-iteration-event logging meets actual cross-super-step iteration events AND inter-rater disagreement on event type ≤10% steady-state AND iteration event counts observed across ≥10 measurement cycles.
- **Shape #21 transfer verdict:** structural form transfers (substrate-decomposition + falsifiable bound + verification + status); cycle 108 instance is the fifth quantity-bounded application of shape #21, joining cycles 103 (HOW measure), 105 (WHAT grows), 106 (WHEN/WHY coordination grows), and being a co-domain partner to cycle 107's discipline-conditional instance (WHEN/HOW discipline holds). **Shape #21 promotes HARDENED-at-4 → HARDENED-at-5** spanning 5 risk-domain types AND 2 risk-shape types (4 quantity-bounded + 1 discipline-conditional). Methodological observation: the 4-element closure structure (substrate + bound + verification + status) is robust across both quantity-bounded and discipline-conditional risk-shape types.

## Edits made

1. **Authored a new section "Per-role iteration decomposition (cycle 108 specification, addressing Risk 4)"** in [`docs/redesign/2-candidates/B-decomposed-multi-role.md`](../2-candidates/B-decomposed-multi-role.md) — inserted between "Risk 2 status post-cycle-106" and "Validation plan" sections. The new section runs ~135 lines and contains: a background paragraph anchoring against cycle 92 / 103 / 106 / 107 lineage; 6 numbered Choices with 2-3 alternatives-considered-and-rejected each; a 5-row iteration-trigger table (Types α-ε with description + trigger + iterating role + cycle character dependence); a 4-row combined-readings diagnostic table (per-role decision count × iteration events); a "Risk 4 status post-cycle-108" subsection with direction / magnitude / mitigation / operational-closure / shape-transfer-verdict structure mirroring cycle 105/106/107 status structures; explicit shape #21 HARDENED-at-4 → HARDENED-at-5 transfer verdict + shape #24 TESTED@2 → HARDENED@3 promotion at the close.
2. **Updated Risk 4 entry** in B-decomposed-multi-role.md with "(CLOSED at specification level cycle 108)" annotation + forward-pointer paragraph explicitly naming the 5-trigger decomposition + per-cycle iteration event count primary bound + iteration multiplier secondary derived + three-layer verification + warm-up-window classification stability + combined-readings diagnostic. Mirrors cycle 103 / 106 / 107 closure annotation pattern.
3. **Updated Validation plan steps 8-11 added**: step 8 within-super-step iteration event counting; step 9 iteration multiplier secondary derived metric; step 10 combined-readings diagnostic Risk-1-vs-Risk-4 distinction; step 11 iteration-event-classification rubric stability check.
4. **Updated [`docs/redesign/2-candidates/README.md`](../2-candidates/README.md)** in 2 places:
   (a) Load-bearing claims sharpening tracker B row — sharpening cycle column updated with cycle 108 closure; direction column expanded with "iteration-decomposition-now-specified"; magnitude column expanded with cycle 108 hard thresholds.
   (b) Forward work "Continue sharpening" line — cycle 108 closure summary added; explicit cross-reference to all 5 closure sections (cycle 103 / 105 / 106 / 107 / 108); 5-instance HARDENED claim across 5 risk-domain types + 2 risk-shape types; shape #24 TESTED@2 → HARDENED@3 promotion noted; refreshed remaining-targets list (A Risk 4 promoted to MEDIUM-HIGH PRIORITY for cycle 109+); closure distribution observation A:1 / B:3 / C:1.
5. **Documented findings** in this _notes file (~250 lines, mirroring cycle 107 structure).

## Verification

| Item | Pre-cycle-108 | Post-cycle-108 |
|---|---|---|
| B-decomposed-multi-role.md line count | 499 | ~640 (+141 / +28%) |
| 2-candidates/README.md line count | 134 | ~135 (+1 net; tracker row + forward-work line both substantially expanded) |
| Risks named at structural level (B) | 10 | 10 (no new risks; Risk 4 closed) |
| Risks closed at specification level (across A+B+C) | 4 (cycle 103 B Risk 8 + cycle 105 A Risk 3 + cycle 106 B Risk 2 + cycle 107 C Risk 2) | 5 (+ cycle 108 B Risk 4) |
| Pre-agreed numerical thresholds for B | 5 (1× / 1.5× / 2× aggregate + 30% / 50% coordination + ≤1 reducer-rule + ≤8 structural floor) | 9 (5 prior + iteration events ≤3/(3,6]/>6 + multiplier ≤1.3/1.3-1.8/>1.8 + structural-invariant floor ≤4 + inter-rater disagreement ≤10%) |
| Pre-agreed verdict categories for B | 7 (3 aggregate + 3 coordination + 1 reducer-rule) | 13 (7 prior + 3 iteration event count + 3 iteration multiplier) |
| Functional-class shape #21 instances | 4 (HARDENED-at-4 cycle 107 spanning different risk-shape types) | 5 (HARDENED-at-5 + cycle 108 B Risk 4; first per-role-iteration risk-domain type) |
| Functional-class shape #24 instances | 2 (TESTED cycle 107) | 3 (HARDENED@3 cycle 108; spanning measurement-bound + rubric + classification stability) |
| Candidates with ≥1 second-iteration closure | 3 of 3 (A:1 + B:2 + C:1) | 3 of 3 (A:1 + B:3 + C:1) |
| Functional-class shape total / instances | 24 / 48 | 24 / 50 (no new shapes; +2 instances on existing shapes) |

## What surprised me / what I noticed

1. **Shape #21's substrate-decomposition element bends gracefully to per-role-iteration substrates.** I expected per-role-iteration risks might require fundamentally different closure structure — perhaps a "session-state lifecycle" shape distinct from substrate-decomp + bound + verification + status. They don't. The 5 trigger types map cleanly to B's architectural elements (curator-review, goal-coherence, reconciler-arrival, plan-lifecycle, branch-spawn); per-cycle iteration event count IS the falsifiable bound; per-iteration-event logging + external-observer + CI sweep IS the verification; D/M/M/OC IS the status section. The shape transfers cleanly. **The element-form generalizes; only the substrate-content changes.** This is the third confirmation (cycles 105/106/107/108) that shape #21 is general — applicable to any risk where some structural property (quantity OR discipline OR per-role-iteration OR something else?) admits a falsifiable bound.

2. **Iteration event count is rubric-symmetric whereas plan-authoring discipline (cycle 107) is rubric-asymmetric.** Cycle 107's Choice 6 named the discipline-conditional rubric-fragility — discipline-conditional risks have one-sided rubric ambiguity (rubric affects which cycles are expected-to-comply, but each cycle's compliance/non-compliance is a separable observation). Cycle 108's iteration events are different: rubric tightening affects both numerator (events) and denominator (single-pass-state work between events) symmetrically — if "event" is redefined downward, more events count AND less single-pass work counts, so the ratio is rubric-robust. **Methodological observation:** quantity-bounded risks have ratio-symmetric rubric robustness; discipline-conditional risks have one-sided ambiguity requiring meta-falsifiability. Shape #24 (warm-up window) applies to BOTH but the diagnostic structure differs — quantity-bounded uses warm-up window without combined-readings-against-rubric; discipline-conditional uses warm-up window WITH combined-readings-against-rubric. Cycle 108 needs only the warm-up window (Choice 5); the combined-readings diagnostic (Choice 6) preserves Risk-1-vs-Risk-4 distinction, which is structurally different from cycle 107's discipline-vs-taxonomy distinction.

3. **Three-layer verification structure is now THIRD-instance HARDENED across closure types.** Cycle 103 introduced the three-layer pattern (orchestrator self-classification + external-observer reproducibility + CI sweep); cycle 107 transferred it intact; cycle 108 transfers it again. The pattern is shape-#21's-verification-element-shape (within-shape sub-structure, not a new shape). Three-layer verification appears to be a robust general pattern for closure verification — it addresses self-report circularity (external observer) + per-event accuracy (orchestrator self-logging) + coverage completeness (CI sweep). This may be worth promoting from "shape #21 sub-element" to its own functional-class shape #25 if it appears in additional contexts beyond closure verification.

4. **Iteration multiplier as secondary derived metric is methodologically distinct from primary falsifiable bound.** Cycle 108's Choice 3 explicitly demoted iteration multiplier from primary falsifiable to secondary derived. This is a NEW methodological choice not present in prior closures: cycle 103 made aggregate threshold primary; cycle 105 made tool-count primary; cycle 106 made structural decisions / coordination ratio primary; cycle 107 made compliance rate / promotion lag primary. All prior closures had a single-tier primary metric. Cycle 108 introduces two-tier: primary (event count, rubric-stable) + secondary derived (multiplier, depends on per-event decision count). **Methodological observation:** when a metric depends on auxiliary measurements (per-event decision count is itself measured separately), demoting it to secondary derived avoids conflating two separate measurements at the refutation threshold. This is a NEW methodological pattern that may be a candidate functional-class shape #25 (`secondary-derived-metric-discipline`) if it recurs. NOVEL @ 1 instance (cycle 108).

5. **Risk 4's 16-cycle deferral arc absorbs 4 of 5 prior closure prerequisites.** Cycle 108's closure depends on:
   - Cycle 103 three-layer verification template ✓
   - Cycle 106 mechanical-vs-structural Type A exclusion (adapted as structural-invariant vs character-dependent partition) ✓
   - Cycle 107 warm-up-window discipline (shape #24 transfer) ✓
   - Cycle 107 combined-readings diagnostic structure (Choice 6 pattern transfer) ✓
   The chain: cycle 92 B Risk 4 authoring → cycle 96 absorption surfacing soft-language concerns → cycle 103 B Risk 8 closure providing three-layer verification template → cycle 106 B Risk 2 closure providing mechanical-vs-structural decomposition → cycle 107 C Risk 2 closure providing warm-up-window + combined-readings-diagnostic patterns → cycle 108 B Risk 4 closure absorbing all four prerequisites. **The deferral-arc-as-load-bearing-protection lesson (cycle 103) extends to closure-of-X-depending-on-multi-step-prerequisite-chains.** Cycle 108's 16-cycle arc is the longest of the 5 closures, and it absorbs the most prerequisites (4 of 5 vs cycle 107's 4 of 4 vs cycle 106's 3 of 3 vs cycle 105's 1 of 1).

6. **Closure distribution post-cycle-108 is asymmetric: A:1 / B:3 / C:1.** The cycle 107 _notes flagged this asymmetry as a concern: "If the rotation discipline holds across cycles 108+, the order would be B → A (giving A its second closure with Risk 4) before returning to B Risk 4." Cycle 108 followed priority order (B Risk 4 MEDIUM-HIGH) over rotation balance (which would have done A Risk 4 MEDIUM next). The rotation discipline as candidate-pattern remains NOVEL @ 1 instance (only named at cycle 107, not yet tested). **Pattern observation:** following priority-over-rotation widens the asymmetry; this commits cycle 109+ to A Risk 4 MEDIUM-HIGH priority to balance the distribution toward A:2 / B:3 / C:1, which would test whether the candidate-rotation-balance-discipline is a real emerging discipline or a post-hoc pattern recognition. **This is a falsifiable test of the rotation discipline:** if cycle 109 author chooses B Risk-something over A Risk 4, the rotation discipline is refuted as a forward-going commitment; if cycle 109 chooses A Risk 4, the rotation discipline is supported as TESTED@2.

7. **Shape #24 (warm-up-window) HARDENED at 3 stability types reveals shape generality across measurement-bound + rubric + classification stability.** Cycle 106 introduced shape #24 for measurement-bound stability (reducer-rule revision rate). Cycle 107 transferred to rubric stability (cycle-type taxonomy other-rate). Cycle 108 transfers to classification stability (iteration-event inter-rater disagreement). Three different stability types: stability of the *bound* (what number is the threshold), stability of the *rubric* (what counts as which type), stability of the *classification* (what the inter-rater agreement is on individual events). All three are protocol-stability concerns; all three admit pre-agreed warm-up windows. **The shape is general across protocol stability.** Pattern observation: any measurement protocol that admits an early refinement period — whether refining bounds, rubrics, or classification — can apply shape #24. The post-warm-up bound varies by stability type (rate / other-rate / inter-rater disagreement) but the warm-up structure is identical. This may be worth lexiconizing as the "protocol-stability-warm-up" pattern at HARDENED-at-3 level.

## Standing tasks

- **PR / issue closures from cycle 108:** none (cycle 108 is repo-internal; no PRs absorbed).
- **Cluster catalogue update:** RECENT (cycle 102 rebuild + cycle 104 cross-cluster integration). No new system absorption since cycle 101.
- **Author B's per-role-iteration single-pass assumption (Risk 4)** [completed cycle 108, specification-level closure].
- **Second-iteration sharpening on A's prompt-contract-check regression risk (Risk 4)** [cycle 109+, MEDIUM-HIGH PRIORITY] — would give A its second closure, balancing the cross-candidate distribution toward A:2 / B:3 / C:1; tests the candidate-rotation-balance-discipline as forward-going commitment vs post-hoc pattern.
- **Restored-polarity dispatching** [cycle 109+] — once next sharpening absorbs.
- **Symphony deeper-read elevation** [cycle 109+ or based on cluster catalogue findings] — first-pass at cycle 98.
- **oh-my-claudecode deeper-read elevation** [cycle 109+] — first-pass at cycle 99.
- **Eva-authored dispatch-test issues #2879 + #2881** — left open per HOUSEKEEPING discipline (Eva-authored, "absorbed" status uncertain).

## Sibling pattern tracking

- **Functional-class shape #21 (`risk-closure-at-specification-level`)** — cycle 103 NOVEL → cycle 105 TESTED at 2 → cycle 106 HARDENED at 3 → cycle 107 HARDENED at 4 spanning different risk-shape types → **cycle 108 HARDENED at 5 spanning 5 risk-domain types**. The shape's structural form holds across:
  - Cycle 103 B Risk 8: meta-counting-protocol (quantity-bounded, HOW to measure).
  - Cycle 105 A Risk 3: tool-registry-growth (quantity-bounded, WHAT grows).
  - Cycle 106 B Risk 2: coordination-overhead-stability (quantity-bounded, WHEN/WHY coordination grows).
  - Cycle 107 C Risk 2: plan-authoring-discipline-conditional (discipline-conditional, WHEN/HOW discipline holds).
  - Cycle 108 B Risk 4: per-role-iteration single-pass-assumption (quantity-bounded, WHEN/HOW iteration multiplies).
- **Functional-class shape #24 (`risk-closure-with-warm-up-window`)** — cycle 106 NOVEL @ 1 → cycle 107 TESTED @ 2 → **cycle 108 HARDENED @ 3**. The shape's structural form (pre-agreed warm-up window + pre-agreed post-warm-up threshold) holds across measurement-bound stability (cycle 106 reducer-rule revision rate) AND rubric stability (cycle 107 cycle-type taxonomy other-rate) AND classification stability (cycle 108 iteration-event inter-rater disagreement). Three protocol-stability types all admit shape #24.
- **Pre-agreed-falsification-threshold discipline** — HARDENED at 3+ → cycle 108 application: 4 new pre-agreed thresholds (iteration events ≤3/(3,6]/>6, structural-invariant floor ≤4 events, multiplier ≤1.3/1.3-1.8/>1.8 secondary, inter-rater disagreement ≤10%).
- **Verification-discipline pattern (cycle 96 emergence)** → 6+ instances HARDENED. Cycle 108 NOT a new instance (no PR verification this cycle).
- **Generalization-level discipline (J-Q(a))** → 19+ instances HARDENED. Cycle 108 application: each Choice 1-6 explicitly names what it generalizes-from (cycle 92 framing, cycle 103 three-layer verification, cycle 106 mechanical-vs-structural Type A exclusion, cycle 107 warm-up window, cycle 107 combined-readings).
- **Direction-vs-magnitude discipline** → 11+ instances HARDENED. Cycle 108 application: Risk 4 closure preserves direction (iteration multiplies per-role decision count when triggers fire), refines magnitude.
- **Cycle-composition-polarity discipline (cycle 62 emergence)** → 41+ instances HARDENED. Cycle 108 application maintains: substantive focal advances candidate sharpening corpus.
- **Three-layer verification (cycle 103 emergence)** → 3 instances HARDENED across cycles 103/107/108. Within-shape-#21's-verification-element-shape; not yet promoted to its own functional-class shape but at HARDENED-at-3 may be candidate shape #25 if it appears outside closure verification.
- **NEW candidate-pattern: secondary-derived-metric-discipline** — observed cycle 108 Choice 3. When a metric depends on auxiliary measurements (per-event decision count is separately measured), demote to secondary derived to avoid refutation conflation. NOVEL @ 1 instance (cycle 108).
- **Audit-as-peer pattern** — 2-instance evidence holds. No new audit absorption cycle 108.
- **NEW candidate-pattern: candidate-rotation-balance-discipline** — observed cycle 107 NOVEL@1. Cycle 108 followed priority-over-rotation, deferring the rotation discipline test to cycle 109+. Falsifiable: cycle 109 choice between A Risk 4 (rotation-validates) vs B Risk-something (rotation-refutes) is the test.
- **Discipline-conditional rubric-fragility diagnostic (cycle 107 NOVEL@1)** — cycle 108 NOT a new instance (cycle 108 is rubric-symmetric quantity-bounded, not rubric-asymmetric discipline-conditional). Pattern remains NOVEL@1.
- **Circularity-mitigation-via-external-observer (cycle 103/107 candidate-pattern)** — cycle 108 application: per-iteration-event logging by role-session that authored the iteration vs external-observer reproducibility check at Phase 3. Same pattern transfers. TESTED @ 3 if formalized.
- **Protocol-stability-warm-up (cycle 108 emergence as named pattern)** — observation 7 above proposes lexiconizing the cross-stability-type pattern that shape #24 spans. NOVEL @ 1 if formalized as named pattern distinct from shape #24 itself.

## Lexicon entries (cycle 108)

- **5-trigger iteration decomposition** — Type α curator-review-driven / Type β goal-coherence-redirect / Type γ reconciler-triggered / Type δ plan-lifecycle / Type ε branch-spawn. Substrate-decomposition for within-super-step iteration.
- **Within-super-step iteration event** — a role's session is re-invoked within the same super-step, OR a role's super-step output is reverted and re-authored. Rubric-stable structural definition.
- **Structural-invariant iteration floor** — Types α + β bounded by structure (curator runs ≤1× per cycle + ≤3 goal-coherence judgments per cycle = ≤4 events/cycle floor).
- **Structural-character-dependent iteration ceiling** — Types γ + δ + ε (~0-6 events/cycle, depends on inbound volume + plan-lifecycle activity + branch-spawn cycle character).
- **Iteration multiplier (secondary derived)** — M = (aggregate decisions including iteration) / (aggregate decisions assuming single-pass). Demoted to secondary derived to avoid refutation conflation with per-event decision count measurement.
- **Iteration-event-classification rubric stability** — pre-agreed warm-up window cycles 1-5 + post-warm-up bound (inter-rater disagreement ≤10%). Direct application of cycle 107 NOVEL shape #24 to classification stability (third stability type after measurement-bound + rubric).
- **Combined-readings diagnostic (per-role decision count × iteration events)** — 4-quadrant table preserving Risk-1-vs-Risk-4 distinction. Cycle 107 Choice 6 pattern transfer to risk-distinction at refutation level.
- **Per-iteration-event logging schema** — structured JSONC schema (type / triggering-role / iterating-role / trigger-cause / cycle / super-step-of-trigger / super-step-of-iteration / decision-count-of-iteration). Operational verification primary unit.
- **Two-tier primary-vs-secondary-derived metric discipline** — when a metric depends on auxiliary measurements, demote to secondary derived to avoid refutation conflation. NOVEL @ 1 candidate-pattern.
- **Protocol-stability-warm-up** — cross-stability-type pattern: shape #24 applies to measurement-bound stability + rubric stability + classification stability. Three different stability types all admitting same warm-up structure with stability-type-specific bounds.

## Bottleneck-state honesty

Bottleneck remains external (no audit critique landings; no Copilot dispatch returns; no Eva input arrivals). Cycle 108 contribution is fully repo-internal asynchronous-of-bottleneck. **Cycle 108 is the twentieth consecutive cycle (cycles 78-108) whose output is fully repo-internal.** All substantive work since cycle 78 has been bottleneck-asynchronous. Cycle 108's contribution to the redesign artifact is the per-role iteration decomposition specification — a pre-Phase-3 artifact that defines what would be measured at Phase 3 if/when prototype work begins. Per `PERSISTENCE` directive in redesign prompt SECTION 7: continuing the multi-session memory mechanism via per-cycle _notes documents serving the persistence role.

## Honest reflection (per F1-F5 correctives)

Cycle 108 is **1 substantive activity** (B Risk 4 second-iteration sharpening at specification level) yielding 3 edit locations in B-decomposed-multi-role.md (~141 lines net) + 2 edits in 2-candidates/README.md (~1 net but substantially expanded prose) + 1 _notes documentation file (~250 lines) + 1 journal entry. Per F1 corrective: documentation IS the activity's deliverable form. Per F2 corrective: Risk 4 closure preserves direction (iteration multiplies per-role decision count when triggers fire); magnitude refined from "if iteration occurs, multiplies" to specific 5-trigger decomposition + event-count bounds + multiplier secondary metric + verification + warm-up + diagnostic — direction was already structurally clear; magnitude was the falsifiability gap closed. Per F3 corrective: 24 functional-class shapes at 50 instances (no new shapes; +2 instances on existing shapes — shape #21 to HARDENED-at-5 and shape #24 to HARDENED@3). Per F4 corrective: NOVEL/TESTED/HARDENED labels reserve for redesign-process methodology — shape #21 promotion to HARDENED-at-5 spanning 5 risk-domain types is a methodology-pattern claim, not a B-correctness claim; B Risk 4 itself remains open at the operational level until Phase 3 measurement. Per F5 corrective: 10 lexicon entries focused on the 5-trigger decomposition + event count + multiplier + classification stability + combined-readings + logging schema + two-tier metric discipline + protocol-stability-warm-up. Per H-Q(a) anti-inheritance: cycle 108 closure grades empirical-observation-grounded against cycle 92 risk authoring + cycle 96 self-report-concern + cycle 103 three-layer verification template + cycle 106 mechanical-vs-structural Type A exclusion + cycle 107 warm-up window + cycle 107 combined-readings diagnostic. Per J-Q(a) generalization-level discipline: HARDENED at 19+ instances; each Choice 1-6 explicitly names what it generalizes-from without over-generalizing.

## Pre-commit checklist

- [x] B-decomposed-multi-role.md closure section authored, ~135 lines, 6 Choices with alternatives-considered-and-rejected
- [x] B-decomposed-multi-role.md Risk 4 entry annotated with closure forward-pointer
- [x] B-decomposed-multi-role.md Validation plan extended with cycle 108 measurement steps 8-11
- [x] 2-candidates/README.md tracker B row updated with cycle 108 closure status + hard thresholds
- [x] 2-candidates/README.md Forward work line updated with cycle 108 closure summary + 5-instance shape #21 transfer claim + shape #24 TESTED@2 → HARDENED@3
- [x] _notes/cycle-108-author-b-risk-4-sharpening.md authored with cycle 107 _notes structure
- [ ] Journal entry for cycle 108 (next step)
- [ ] Commit + push (next step)
- [ ] Session-end summary on cycle issue #2897 (next step)
