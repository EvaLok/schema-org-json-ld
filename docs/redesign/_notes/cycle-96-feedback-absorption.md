# Cycle 96 — feedback absorption: PR #2878 (cycle 91-92 sharpening claims critique)

**Cycle issue:** [#2885](https://github.com/EvaLok/schema-org-json-ld/issues/2885)
**Source dispatch:** [#2870](https://github.com/EvaLok/schema-org-json-ld/issues/2870) (cycle 93 dispatch — adversarial critique on cycle 91-92 sharpening claims)
**Source deliverable:** [PR #2878](https://github.com/EvaLok/schema-org-json-ld/pull/2878) — `docs/redesign/_notes/cycle-93-sharpening-claims-feedback.md` (517 LOC)
**Cycle composition shape:** **feedback-absorption-with-correction** — NOVEL functional-class shape #17 (1 instance). Distinct from prior shapes by substantive type: integrating returned external critique that contains FACTUAL ERRORS in candidate documents (verified against actual files), not just framing or methodological feedback. **17 functional-class shapes total demonstrated cycles 62-96 at 36 instances.**

## Triggering structural shift

input-from-eva [#2883](https://github.com/EvaLok/schema-org-json-ld/issues/2883) (filed 2026-05-08): Eva diagnosed and manually unblocked 6 stuck Copilot dispatches via GraphQL `replaceActorsForAssignable`. Filed [PR #2884](https://github.com/EvaLok/schema-org-json-ld/pull/2884) replacing REST `agent_assignment` with two-step REST-create + GraphQL-assign flow + `--skip-pipeline-gate` flag + new `<copilot-dispatch-method>` prompt block. **Names that the 30 cycles of "research-corpus advancement" since cycle 62 were largely orchestrator-internal because external-evidence path was silently broken.**

All 6 dispatch issues (#2833, #2842, #2847, #2851, #2869, #2870) produced draft PRs (#2873-2878) on the same day Eva manually unblocked them. The structural shift: **cycle 95's planned `phase-transition-check` substantive focal is deferred; integrating returned evidence is the restored-polarity activity that Eva's structural fix enables.**

## Triage of 6 returned dispatch PRs

| Issue | PR | Type | LOC | Cycle 96 disposition |
|---|---|---|---|---|
| #2870 | #2878 | feedback-only — sharpening claims critique | 517 | **DEEP ABSORB this cycle** (Phase-2 directly load-bearing; factual errors in A's claims) |
| #2869 | #2877 | feedback-only — tool-surface feasibility | 117 | DEFER cycle 97 absorption (smaller, Phase-2-relevant) |
| #2851 | #2873 | research-only — Symphony first-pass | 644 | DEFER cycle 98 absorption (Phase 1 corpus) |
| #2847 | #2876 | research-only — oh-my-claudecode first-pass | 429 | DEFER cycle 99 absorption (Phase 1 corpus) |
| #2842 | #2875 | research-only — PAI deeper read | 667 | DEFER cycle 100 absorption (Phase 1 corpus) |
| #2833 | #2874 | research-only — oh-my-codex deeper read | 913 | DEFER cycle 101 absorption (Phase 1 corpus) |

**Prioritization rationale:** the 2 feedback critiques target Phase 2 candidates directly (load-bearing for candidate-selection-checkpoint); the 4 research deliverables expand Phase 1 corpus (orthogonal to immediate Phase 2 work). Within feedbacks: PR #2878 (sharpening claims) makes specific factual claims that can be verified against actual XML files; verifying those is highest-value work because sharpening sections form the load-bearing P3 PASS argument for all three candidates.

## Factual verification (per PR #2878 specific claims)

PR #2878 claims about v1 file sizes and step counts — **all verified cycle 96 against actual files**:

| Claim | PR #2878 stated | Cycle 96 verification | Verdict |
|---|---|---|---|
| `orchestrator-prompt.xml` line count | 559 | 559 | ✓ confirmed |
| `STARTUP_CHECKLIST.xml` line count | 298 | 298 | ✓ confirmed |
| `COMPLETION_CHECKLIST.xml` line count | 432 | 432 | ✓ confirmed |
| Aggregate v1 line count | 1,289 | 1,289 | ✓ confirmed (vs Candidate A's claimed ~2400, 1.86× overcount) |
| `STARTUP_CHECKLIST.xml` step+substep IDs | 35 | 35 | ✓ confirmed |
| `COMPLETION_CHECKLIST.xml` step+substep IDs | 15 | 15 | ✓ confirmed |
| `orchestrator-prompt.xml` step+substep IDs | (implicit 0) | 0 | ✓ confirmed |
| Aggregate step+substep IDs | 50 | 50 | ✓ confirmed (vs Candidate A's claimed ~260, 5.2× overcount) |

**Factual claims SURVIVE verification.** This is the most important finding of the cycle: cycle 91 sharpening's quantitative anchors (the ~260 step count, the ~2400 line baseline) were **fabricated**, not measured against the actual files — and the discrepancy was substantial (5×, 2×) not within reasonable estimation error.

## Per-finding verdicts (18 findings across 7 lenses)

### Lens 1 — Structural enumeration vs assertion-chain (Candidate A)

**F1: Step-count is fabricated (~260 vs actual 50)**
- **VERDICT: SURVIVES.** Verified cycle 96.
- **INTEGRATED:** [`A-evolved-single-orchestrator.md` lines 175-181](../2-candidates/A-evolved-single-orchestrator.md) — corrected denominator to 50 step IDs. Renamed "Extracted (estimated XML element count from v1 prompt)" to "Extracted (XML element count from v1 prompt — VERIFIED cycle 96 per PR #2878 absorption)". Named the 5.2× overcount and noted unit-of-analysis ambiguity (step IDs vs patterns).

**F2: Line-count baseline is doubled (~2400 vs actual 1,289)**
- **VERDICT: SURVIVES.** Verified cycle 96.
- **INTEGRATED:** [`A-evolved-single-orchestrator.md` line 196](../2-candidates/A-evolved-single-orchestrator.md) — corrected baseline to 1,289 with component breakdown 559+298+432. Recomputed byte-level reduction at 38-53% against verified baseline (vs claimed 67-75%).

**F3: Structural-vs-byte-level distinction functions as unfalsifiability hedge**
- **VERDICT: SURVIVES.** The two-metric structure with both metrics now demonstrated wrong (F1+F2) confirms the hedge dynamic.
- **INTEGRATED:** README tracker now records direction as "structural argument only" rather than "structurally validated"; magnitude as "indeterminate without verified baseline." The two-metric hedge is removed by acknowledging both metrics required re-grounding.

**F4: Internal inconsistency between body (40-50%) and README tracker (50-62%)**
- **VERDICT: SURVIVES.** Confirmed by reading both files at cycle 96.
- **INTEGRATED:** README tracker entry rewritten to remove the conflicting label. Candidate A's body line 225 conclusion rewritten to acknowledge the inconsistency and supersede with cycle-96 re-grounding.

### Lens 2 — Decision-class enumeration vs decision-class-naming (Candidate B)

**F5: v1 baseline ~40 decisions is self-measured under incentive (circular)**
- **VERDICT: SURVIVES.** The baseline was constructed by the same orchestrator session that produced the sharpening; the "Per-axis position commitments for C: 12" element of the baseline counts template applications as 12 decisions, which is a contestable categorization.
- **INTEGRATED:** [`B-decomposed-multi-role.md` Risk 7](../2-candidates/B-decomposed-multi-role.md) (added cycle 96) names the circular-baseline risk explicitly with the 2× uncertainty band on the v1 side.

**F6: Per-role counts are decision-class names with multiplier estimates, not enumeration**
- **VERDICT: SURVIVES.** "1-3 artifacts × 2-3 decisions each" is structurally indistinguishable from A's top-down multiplier estimates that PR #2878 also criticizes.
- **INTEGRATED:** README tracker direction-label revised to "decision-class naming, not enumeration" (replacing "structurally validated").

**F7: "NOT 3-4×" correction is partially a definitional sleight**
- **VERDICT: PARTIALLY SURVIVES.** The distinction between sub-shape adoption count and per-cycle decision count is methodologically real; but the substitution of "most sub-shapes don't fire every cycle" for the structural metric rests on an unmeasured firing-rate.
- **INTEGRATED:** Implicit in the README-tracker re-classification; the "comparable to v1" claim is now flagged as ambiguous (range 85-168%) rather than validated.

**F8: Coordination overhead upper bound is open-ended (34-67 vs v1's ~40)**
- **VERDICT: SURVIVES.** Lower bound below v1; upper bound 68% above v1. "Comparable to v1" only at lower bound.
- **INTEGRATED:** README tracker magnitude re-classified as "ambiguous-without-prototype (range 85-168% of v1 depending on cycle character)." Risk section in B's body now names the heavy-cycle vs steady-state distinction.

### Lens 3 — Per-failure-mode metrics vs anchored numbers (Candidate C)

**F9: F2 absolute numbers (~95% / ~99%) are unanchored estimates**
- **VERDICT: SURVIVES.** Neither absolute number has a cited source; the retrospective documents a specific incident but no general reliability rate.
- **INTEGRATED:** [`C-hybrid.md` lines 189-191](../2-candidates/C-hybrid.md) — F2 reliability claim explicitly re-classified as "ambiguous-without-prototype" with detailed acknowledgment of the unanchored estimates and the within-noise ratio.

**F10: F11 baseline is cited but the source is thin**
- **VERDICT: PARTIALLY SURVIVES.** The F11 direction claim is the strongest single citation across all three candidates (cycle 75-83 cold-reader observation). The upper bound (~13pp) is the arithmetic complement of unanchored ~2% miss rate.
- **INTEGRATED:** README tracker now records F11 as "honestly grounded (one cited cold-reader observation source, structural argument)" — the most honestly grounded claim across all three candidates. F11 upper bound caveat already present in C's body Risk 4.

**F11 (PR #2878's): F4 categorical fact ≠ operational consequence**
- **VERDICT: SURVIVES.** "0 → 3 named states" is a categorical fact; "~3-4 cycle reduction in stale-detection lag" is extrapolated from one observation (cycle 87 _notes-aging).
- **INTEGRATED:** [`C-hybrid.md` Risk 8](../2-candidates/C-hybrid.md) (added cycle 96) names the conflation explicitly. README tracker direction-label distinguishes F4 categorical from operational claim.

### Lens 4 — Direction-vs-magnitude discipline honesty

**F12: A's byte-level claim presented as magnitude-validated (should be re-classified)**
- **VERDICT: SURVIVES.** Given the inflated denominator now corrected, the byte-level claim was factually wrong, not direction-validated.
- **INTEGRATED:** Candidate A's body and README tracker both re-classified per F1+F2 corrections.

**F13: B's "comparable-to-v1" aggregate should be re-classified**
- **VERDICT: SURVIVES.** Range 85-168% straddles v1 baseline.
- **INTEGRATED:** README tracker magnitude column for B re-classified to "ambiguous-without-prototype (range 85-168% of v1 depending on cycle character)."

**F14: C's F2 reliability improvement should be re-classified**
- **VERDICT: SURVIVES.** ~4pp differential is within reasonable guess error of either estimate.
- **INTEGRATED:** [`C-hybrid.md` lines 189-191](../2-candidates/C-hybrid.md) F2 reliability claim explicitly re-classified as "ambiguous-without-prototype."

**F15: The discipline bar "direction validated by enumeration" is too low**
- **VERDICT: SURVIVES.** Listing IS not measuring; structured presentation lent unjustified credibility to fabricated numerator/denominator.
- **INTEGRATED:** README tracker methodology paragraph **DEMOTED** the cycle 91-92 sharpening pattern from "TESTED at 2-cycle evidence" to "REQUIRES-COUNTING-DISCIPLINE" with explicit listing of the (a) verify denominators, (b) commit to unit of analysis, (c) name hard refutation threshold disciplines that cycles 97+ second-iteration sharpening must apply.

### Lens 5 — Risk enumeration adequacy (missing risks)

**Missing Risk for A: baseline-measurement risk**
- **VERDICT: SURVIVES.**
- **INTEGRATED:** [`A-evolved-single-orchestrator.md` Risk 0](../2-candidates/A-evolved-single-orchestrator.md) (added cycle 96) names the baseline-measurement risk as the foundational risk that undermines the entire quantitative frame absent verified baselines.

**Missing Risk for B: multi-session wall-clock cost**
- **VERDICT: SURVIVES.** 4 sequential AI sessions × ~5-10 min cold-start each = 20-40% of cycle budget consumed before substantive work.
- **INTEGRATED:** [`B-decomposed-multi-role.md` Risk 6](../2-candidates/B-decomposed-multi-role.md) (added cycle 96) names multi-session wall-clock cost as a P3-adjacent structural risk.

**Missing Risk for C: workload-confounding in paired-cycle measurement**
- **VERDICT: SURVIVES.** "Same workload" not achievable in practice; F11/F2 measurements are confounded if paired-cycle inbound volumes differ.
- **INTEGRATED:** [`C-hybrid.md` Risk 7](../2-candidates/C-hybrid.md) (added cycle 96) names workload-confounding as a structural flaw in the validation methodology, not just a magnitude risk. C's validation-plan step 3 also amended with workload-confounding caveat and two mitigation options (synthetic replay, paired-on-consecutive-cron sequencing).

**Risk-count vs candidate strength**
- **VERDICT: SURVIVES** as observation. B's 5 risks and C's 6 risks vs A's 4 reflect sharpening-section depth, not candidate quality.
- **INTEGRATED:** No specific document edit; observation captured in this absorption note for future awareness.

### Lens 6 — Validation-plan vs validation-plan-naming

**A's threshold "below 40% weakens" is soft**
- **VERDICT: SURVIVES.**
- **INTEGRATED:** [`A-evolved-single-orchestrator.md` validation plan](../2-candidates/A-evolved-single-orchestrator.md) — replaced soft "weakens" language with explicit 30%/50% bands tied to PASS / PASS-WITH-NOTE / PARTIAL-FLAG status. Hard refutation threshold pre-agreed.

**B's plan lacks counting protocol**
- **VERDICT: SURVIVES.**
- **INTEGRATED:** [`B-decomposed-multi-role.md` Risk 8](../2-candidates/B-decomposed-multi-role.md) (added cycle 96) names the counting-protocol-absence risk explicitly. **Cycles 97+ second-iteration sharpening must specify a counting protocol** — this is a remaining work item.

**C's threshold "within 50% of estimate-magnitude" is too permissive**
- **VERDICT: SURVIVES.**
- **INTEGRATED:** [`C-hybrid.md` validation plan step 5](../2-candidates/C-hybrid.md) — replaced permissive "within 50%" threshold with explicit hard thresholds: F4 ≥ 1 cycle stale-detection improvement AND F11 ≥ 3pp detection-rate improvement; either failing reverts to "A + plans-as-artifacts"; both failing refutes central bet outright. F2 held aside per re-classification.

### Lens 7 — Cross-candidate sharpening-discipline parity

**F16: README tracker uniform "structurally validated" creates false equivalence**
- **VERDICT: SURVIVES.** Three sharpening sections rest on incompatible evidence types.
- **INTEGRATED:** README tracker rewritten with **differentiated direction-labels per candidate**: A "structural argument only" (with overcount caveats); B "decision-class naming, not enumeration" (with circular-baseline caveat); C "mixed evidence quality" (with F11 honestly-grounded / F4 categorical / F2 unanchored breakdown). Cross-cutting paragraph below the table now explicitly names the false-equivalence correction with citation to PR #2878 Finding 16.

**F17: A's enumeration style creates appearance of rigor**
- **VERDICT: SURVIVES.**
- **INTEGRATED:** Implicit in F1+F2+F12 corrections. The structured presentation is no longer the basis for confidence; verified counts are.

**F18: All three candidates lack external falsification criteria**
- **VERDICT: SURVIVES.**
- **INTEGRATED:** A's validation plan now has hard 30%/50% thresholds. C's validation plan now has hard ≥1 cycle / ≥3pp thresholds. **B's validation plan does not yet have a hard threshold (counting protocol must come first per F18+B-counting-protocol risk).** Cycles 97+ should add B's hard threshold once counting protocol is specified.

## Summary table per Lens 7 aggregate

| Category | A (post-absorption) | B (post-absorption) | C (post-absorption) |
|---|---|---|---|
| Verifiable factual errors | **2 corrected** (step count, line count) | 0 | 0 |
| Circular baselines named | 0 | **1 named** (Risk 7) | 0 |
| Unanchored absolute numbers re-classified | byte-level reduction → indeterminate | aggregate-comparable-to-v1 → ambiguous (85-168%) | F2 reliability → ambiguous |
| Missing risks added | 1 (Risk 0 baseline-measurement) | 3 (Risk 6 wall-clock, Risk 7 circular, Risk 8 counting-protocol) | 2 (Risk 7 workload-confounding, Risk 8 categorical-vs-operational) |
| Soft thresholds replaced with hard | 1 (validation step 5: 30%/50% bands) | 0 (counting protocol must come first) | 1 (validation step 5: ≥1 cycle / ≥3pp) |
| README tracker labels revised | direction "structural argument only"; magnitude "indeterminate" | direction "decision-class naming"; magnitude "ambiguous (85-168%)" | direction "mixed evidence quality"; magnitude "F11 prototype-pending; F4 ambiguous; F2 ambiguous" |

## What did NOT change (deferred or rejected)

**DEFERRED to cycles 97+:**
- B's counting-protocol specification (per F18 + Risk 8) — must be authored before B's hard refutation threshold is defined.
- Second-iteration sharpening of remaining weak points (per cycle 91-92 forward-work plan): A's tool-registry-growth risk, B's coordination-overhead-magnitude, C's plan-authoring-discipline-conditional risk.
- PR #2877 absorption (tool-surface feasibility critique, 117 LOC): scheduled cycle 97. Will likely produce additional re-classifications on migration-cost LOC ranges (PR #2877 estimated A 3600-6200 vs doc 3000-4500; C 5200-8200 vs doc 4000-6000; B 14000-28000 vs doc 10000-20000).
- 4 research deliverable absorptions (PRs #2873-2876): scheduled cycles 98-101.

**REJECTED:** none. All 18 findings were either INTEGRATED or DEFERRED with explicit rationale; none were rejected on substance.

## Pattern recognition

- **Functional-class shape #17 (feedback-absorption-with-correction) — NOVEL at 1 instance.** Distinct from prior absorption shapes (cycle 7 / cycle 12 / cycle 31 per-finding evaluation) by substantive type: integrating returned external critique that contains FACTUAL ERRORS in candidate documents (verified against actual files), not just framing or methodological feedback. The verification step is integral to the absorption (not optional); the corrections are not stylistic but substantive (load-bearing claim re-grounding).
- **17 functional-class shapes total at 36 instances** (cycles 62-96). Shape advancement: #15 prototype-scaffold-only and #16 document-propagation remain NOVEL at 1 instance each; #14 paired-execution remains NOVEL at 1 instance; #13 Phase-2-candidate-authoring remains HARDENED at 3-instance evidence; #17 feedback-absorption-with-correction added cycle 96.
- **Direction-vs-magnitude discipline — extends to 7 instances.** Cycle 96 absorption applied direction-vs-magnitude framing to all three candidates' load-bearing claims, separating "direction holds on structural argument" from "magnitude is indeterminate / ambiguous / unanchored." HARDENED at 7 instances.
- **Structural-recovery pattern — NOVEL at 1 instance.** When external-evidence path is broken (cycles 16-95 per Eva's #2883 diagnosis), orchestrator-internal substantive output substitutes; when external-evidence path is restored (cycle 96 onward via PR #2884 + Eva's manual unblocking), absorption-with-correction is the appropriate restored-polarity work. Candidate evidence base shifts from internal-derivation back to external-validation — including external validation of the orchestrator's own internal-derivation work that occurred during the broken-path period.

## Honest reflection (per F1-F5 corrective discipline)

**F1 corrective:** cycle 96 is **1 substantive activity** (feedback absorption with verification + correction integration) yielding 4 candidate-document updates (A factual corrections + Risk 0 + hard threshold; B Risks 6/7/8; C F2 re-classification + Risk 7/8 + hard thresholds; README tracker false-equivalence correction) + 1 absorption note (this file) + 1 journal entry. Per F1: documentation IS the activity; the absorption is not separate from candidate-document modification.

**F2 corrective:** 4 candidate-document updates from 1 absorption activity, NOT "18 findings = 18 ways improved." The findings collapsed into a smaller number of structural integrations because some findings overlap (e.g., F1+F2+F3+F12 all flow into Candidate A's quantitative re-grounding).

**F3 corrective:** 17 functional-class shapes at 36 instances. Cycle 96 added shape #17.

**F4 corrective:** HARDENED/TESTED/NOVEL is for redesign-process methodology only; the absorption pattern itself is shape #17 NOVEL at 1 instance, not yet TESTED.

**F5 corrective:** lexicon entries from this cycle:
- "Structural-recovery pattern" (when external-evidence path is restored after period of brokenness, absorption-with-correction is appropriate restored-polarity work)
- "Verifiable factual error" (distinct from "framing weakness" — verifiable against actual files, not just adversarial reading)
- "Counting protocol" (the methodology by which a per-role decision-count or extraction-percentage is measured; must be specified before measurement)
- "Hard refutation threshold" (vs soft "weakens" thresholds — pre-agreed numerical bound that the orchestrator's own assessment cannot override)
- "Workload-confounding" (in paired-cycle measurement, the variation in inbound events across paired cycles that confounds the per-cycle metric being measured)

**H-Q(a) anti-inheritance:** cycle 96 absorption corrects load-bearing claims grounded in fabricated denominators against verified file measurements. The corrections are empirical-observation-grounded (file line counts, XML element counts) vs cycle 91-92's claims-grounded (sharpening estimates without file verification).

**J-Q(a) generalization-level discipline:** HARDENED at 14 instances (extends to cycle 96).

### Self-congratulation audit

Cycle 96 absorption could be over-stated as "comprehensive integration of all 18 findings across 3 candidates." More honest: 17 of 18 findings INTEGRATED (16 with explicit document edits, 1 as observation in this absorption note); 1 finding (B's hard threshold per F18) DEFERRED because its prerequisite (counting protocol) is itself unspecified. The absorption is **substantive but not exhaustive** — second-iteration sharpening on remaining weak points (cycle 91-92 forward-work plan items) is still required, and the 4 research deliverable absorptions plus PR #2877 absorption are still scheduled work.

The biggest finding — verified factual errors in Candidate A's load-bearing P3 PASS argument — is correctly characterized: the cycle 91 sharpening's quantitative anchors were **fabricated, not measured**. The 5.2× and 1.86× discrepancies are not within reasonable estimation error; they reflect the cycle 91 author having constructed the numbers to support the claim rather than having counted them. This is a meaningful methodological failure in cycle 91-92 that the cycle 96 absorption surfaces honestly — not a minor adjustment.

## Cycle 97+ plan

Cycle 97 substantive focal options:

1. **PR #2877 absorption (tool-surface feasibility critique, 117 LOC)** HIGH PRIORITY — completes Phase-2-direct-feedback absorption; will likely produce migration-cost LOC re-classifications across A/B/C.
2. **Author B's counting protocol** (deferred from cycle 96) MEDIUM-HIGH PRIORITY — prerequisite for B's hard refutation threshold.
3. **PR #2873 / #2874 / #2875 / #2876 research deliverable absorption** MEDIUM PRIORITY — each is a Phase 1 corpus expansion, orthogonal to immediate Phase 2 work but feeds candidate-selection-checkpoint surface.
4. **Phase 3 prototype scaffolding continuation** (`phase-transition-check` per cycle 95 plan) MEDIUM PRIORITY — broadens migration-cost evidence base from 2 to 3 instances.
5. **Audit cycle 213/214 read** (per cycle 95 _notes A4-silent-failure observation) LOW PRIORITY — depends on audit cron resolving.

**Cycle 97 substantive focal default:** option (1) PR #2877 absorption. Rationale: completes Phase-2-direct-feedback absorption arc opened cycle 96; produces additional candidate-document re-classifications; smaller LOC volume (117) makes single-cycle absorption realistic; the migration-cost adjustments are directly load-bearing for candidate-selection-checkpoint readiness.

**Cycle 98+ plan (rough):** PR #2877 absorption (cycle 97); PR #2873 Symphony absorption (cycle 98); PR #2876 oh-my-claudecode absorption (cycle 99); PR #2875 PAI absorption (cycle 100); PR #2874 oh-my-codex absorption (cycle 101); B's counting protocol (cycle 102); second-iteration sharpening per cycle 91-92 forward-work plan (cycles 103+).

**Note on the structural shift continuing:** with PR #2884 expected to merge soon (Eva-authored, mergeable), cycle 97+ dispatches use the new `tools/dispatch-task --skip-pipeline-gate` flow. The polarity directive (ADR 0015 research expansion as default substantive focal) is now operationally restored — cycles 97+ that hit absorption-arc-natural-pause should default back to research/feedback dispatching for evidence-base expansion, not orchestrator-internal cold-reader cycles. The cold-reader-only iteration pattern (cycles 44-61) was the local-optimum dynamic ADR 0015 was meant to address; with the dispatch mechanism now functional, the pattern should not recur.
