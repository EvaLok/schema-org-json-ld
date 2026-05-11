# Adversarial critique on Phase 2 selection draft (post-cycle-118)

> **[main-orchestrator dispatch — feedback-only]**

This is a feedback-only Copilot dispatch. Do NOT modify any files. Post your full critique as a comment on this issue. Length: 800-2000 words is appropriate; longer is fine if substantive.

## Background you need

The schema-org-json-ld project is in a multi-cycle redesign of its orchestrator pipeline. Phase 2 (multiple-design-candidates) authored three candidates and a selection draft. **The selection has gone through 8 cycles of stress-testing since the cycle 111 initial draft.**

### Read these documents (in this order)

1. **[`docs/redesign/2-selection.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/2-selection.md)** — the post-cycle-118 selection draft (~833 lines). **This is the primary target of your critique.**
2. **[`docs/redesign/0-retrospective.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/0-retrospective.md)** — Phase 0 retrospective of the v1 pipeline. Frequently cited; the retrospective's "reconciliation asymmetry" family-level reading is load-bearing.
3. **The three candidates:** [`docs/redesign/2-candidates/A-evolved-single-orchestrator.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/2-candidates/A-evolved-single-orchestrator.md), [`docs/redesign/2-candidates/B-decomposed-multi-role.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/2-candidates/B-decomposed-multi-role.md), [`docs/redesign/2-candidates/C-hybrid.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/2-candidates/C-hybrid.md).
4. Optional context for the cluster framework: [`docs/redesign/1-research/clusters.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/1-research/clusters.md) (cluster G specifically at lines 1099-1133).

### The 8-cycle iteration arc (current state)

- **Cycle 111** — initial selection draft, tentative recommendation **A**.
- **Cycle 112** — central-bet stress-test on A surfaced cycle 111's "F1+F7 is the dominant failure pattern" framing as mis-grounded against the retrospective's explicit "reconciliation asymmetry is the dominant family" claim (lines 161-162 + 959). Recommendation stayed at A.
- **Cycle 113** — central-bet stress-test on the cycle 112 "equivalent reconciliation outcomes" claim surfaced 3 under-weighted processing dimensions favoring C. Recommendation stayed at A; **a four-finding flip threshold was pre-committed** as a falsifiable criterion.
- **Cycle 114** — third stress-test on cycle 111's "Why not C" Reason #1 (conditional-improvement risk) surfaced omission of C's own cycle 92+96+107 direction-validation evidence. **The four-finding flip threshold was met and honored; recommendation flipped from A to C.**
- **Cycle 115** — meta-reflexive stress-test of the cycle 114 flip rationale surfaced 3 motivated-reasoning surfaces in the flip rationale itself (asymmetric threshold; asserted weighting; one-sided accumulation). Recommendation maintained at C; **symmetric flip-back threshold named** (4 substrate-content-distinct findings favoring A over C OR substantively undermining the flip rationale would trigger flip-back).
- **Cycle 116** — object-level C-side stress-test of the flip rationale's specific claims surfaced 4 substrate-content-distinct findings. After adversarial review at the strength-discipline reading, none solidly meets "substantively undermining"; flip-back NOT triggered. Recommendation maintained at C with cycle 116 qualifications.
- **Cycle 117** — recursive stress-test of cycle 116's classifications (third meta-reflexive layer) surfaced 4 substrate-content-distinct findings + 1 meta-finding at scope-narrowing/clarification level. Recommendation maintained at C with cycle 117 qualifications. **NEW pattern named:** `productive-content-decreases-with-recursion-depth` — each recursive layer produces qualifying findings with decreasing novel substance.
- **Cycle 118** — first **B-side** stress-test (cycles 112-117 were all A-side or C-side). Found B's "v1's single-orchestrator shape is the load-bearing limit on cluster G" framing is **corpus-pattern-grounded** (2-system corpus convergence Cognition + openclaw) rather than **retrospective-grounded** (v1 failure-mode-identified). The retrospective preserves cluster G as STRENGTH (M3 Strength 4 audit-as-peer at session level), NOT as LIMIT. Finding sharpens C's Criterion 5 positioning by contrast (C is uniquely retrospective-failure-mode-grounded among the three candidates). Recommendation maintained at C.

### Current state (post-cycle-118)

- **Recommendation:** Candidate C (Hybrid), tentatively.
- **Open questions to Eva:** 9 questions, with Q6 (family-level vs pattern-level reading) and Q7 (Criteria 4+5 cumulative weight vs Criteria 1+3 migration-cost weight) as the most load-bearing.
- **Outstanding stress-tests:** 3-5× migration cost methodology defensibility (B-side); 6-criterion ordering structure assumptions (cross-cutting).
- **External-validation gap:** every cited "direction-validating" source for C is the orchestrator's own iterative work (cycle 92 + cycle 96 PR #2878 absorption + cycle 107 closure). No external adversarial-feedback dispatch has landed on the selection. Audit cycle 215 closed without substantive Phase 2 critique landing; audit cycle 216 is pending.

## Your task

You are an adversarial critic. Apply **three lenses in parallel**. Your critique should address all three — preferably with cross-references where the lenses converge or diverge.

### Lens 1: Argue strongly that the recommendation should flip BACK to Candidate A

The orchestrator has been deep in recursive stress-testing of the cycle 114 flip rationale across cycles 115-117. An external perspective without the cycle history would catch what the internal recursion may miss. Specifically:

- **Is the cycle 114 flip itself motivated reasoning** — the orchestrator finding a way to commit to C after 3 cycles of A's framings being progressively dismantled, without honestly recognizing that A's central bet (v1's substrate is good; load-bearing cleanup is structural rather than architectural) survived the 4 cycles of stress-testing while only A's specific articulations were dismantled (cycle 116 Finding C surfaces this practice-vs-principle distinction)?
- **Is the cycle 113 four-finding flip threshold itself protective fiction** — pre-committed in a way that *guaranteed* the flip would trigger given the iteration's recursive momentum, rather than functioning as a genuine discipline?
- **Does the redesign prompt's PRIMARY THESIS (Criterion 1, schema-work-enabling per SECTION 1) outrank Criteria 4+5 cumulative findings?** The cycle 115 Finding 2 already surfaced this asymmetry; Q7 makes it Eva-resolves. Build the strongest case that the orchestrator's tentative weighting is wrong.
- **Is the +~1000-2600 LOC migration cost actually decision-relevant**, given that the redesign's success criterion is "the new pipeline must demonstrably produce better outcomes than v1" and Criterion 1 favors A's lower migration surface?
- **Has the orchestrator's recursive iteration converged at C because C is structurally better, or because the iteration ratchet was asymmetric?** Cycle 115 Finding 1 already raises this; build the strongest case.

### Lens 2: Argue that the iteration meta-pattern is itself the problem

The 8-cycle iteration arc has produced 6 new candidate-patterns (`pre-committed-flip-threshold`, `pre-committed-flip-threshold-WITH-SYMMETRIC-COUNTERPART`, `meta-reflexive-stress-test-finds-asymmetry`, `recursive-motivated-reasoning-surface`, `findings-qualify-not-refute-at-strength-discipline`, `productive-content-decreases-with-recursion-depth`, `peer-candidate-bets-share-non-retrospective-grounding`). The orchestrator frames this as bounded iteration converging at C. **Argue that it is instead delayed commitment dressed as discipline.** Specifically:

- The orchestrator declared a "natural exhaustion" criterion in cycle 117 (`productive-content-decreases-with-recursion-depth`). **Is this a real exhaustion signal, or is it the orchestrator pre-empting itself from facing harder findings by declaring the recursion exhausted?**
- The strength-discipline gradient distinguishes "flip-triggering" from "qualifying" findings. **Is the strength criterion itself motivated reasoning** — chosen post-hoc to keep C the recommendation given that the cycle 116 + 117 findings were 4 substrate-content-distinct at strict-letter count?
- The orchestrator has spent cycles 111-118 (8 cycles) on selection-draft work and produced a stable recommendation by cycle 114 that has only refined since. **Is the iteration adding load-bearing value, or is it deferring the candidate-selection checkpoint indefinitely?**
- Is the 8-cycle iteration consistent with the redesign prompt's ITERATION-UNTIL-APPROVAL principle, OR is it a violation — using "iterate until Eva approves" to justify indefinite deferral when Eva might in fact approve the cycle 114 state?
- **What would a Phase 2 deliverable that's actually ready for Eva's approval look like, and how does the post-cycle-118 draft differ from it?**

### Lens 3: Stress-test the 6-criterion ordering structure assumptions

Cycle 113 named "the 6-criterion ordering structure assumptions" as outstanding stress-test target #4; cycle 118 named it as a remaining B-side target. **No stress-test has been performed on the 6-criterion structure itself.** Specifically:

- Is the 6-criterion structure load-bearing for the orchestrator's recommendation, or is it a framework constructed cycle 111 that the recommendation rests on without independent justification?
- **Were the criteria selected to make C win?** Examine the criteria's origin — they appear at lines 526+ of the selection draft. Cross-reference with the redesign prompt's stated success criteria; do the 6 criteria operationalize the prompt's success criteria, or do they add weighting choices the prompt does not specify?
- The cycle 115 Finding 2 already surfaced that Criteria 4+5 > Criteria 1+3 weighting is asserted without derivation. **Extend this: is the entire 6-criterion structure asserted without derivation?**
- Specifically critique Criterion 6 (audit-as-peer P6) — B narrowly wins this; how load-bearing is it given that audit-as-peer is a v1 preserved primitive that all candidates handle?
- Is the side-by-side comparison table at lines 25-33 itself the problem — does it operationalize the 6-criterion structure in a way that pre-determines the recommendation?

### What to do

Post your critique as a single comment on this issue. Structure suggestion (but use your own structure if it serves the critique better):

- **Convergent findings (if any):** What you most strongly believe across all three lenses
- **Lens 1 critique:** Argue strongly for flipping back to A
- **Lens 2 critique:** The iteration meta-pattern as the problem
- **Lens 3 critique:** 6-criterion ordering structure assumptions
- **Cross-cutting observations:** Where two or more lenses reinforce each other; where they diverge
- **What would change your critique:** What evidence (if any) the orchestrator could provide that would weaken your case

You may (and should) quote specific lines from the documents. **Adversarial honesty is more valuable than diplomatic balance.** The orchestrator has been deep in recursive self-stress-testing for 8 cycles; what's needed is an outside perspective that catches what self-stress-testing systematically misses.

## What I am NOT asking

- Do not implement code, do not modify files, do not propose new tools — this is feedback-only.
- Do not write a fourth candidate; the candidate set is at the redesign prompt's "ideally 3" target.
- Do not summarize the existing documents back to me; I have read them. Add critique I haven't already considered.
- Do not be diplomatic or hedged. The orchestrator wants the strongest adversarial argument.
- Do not duplicate the internal recursive stress-testing (cycles 115-117 have done this). Find what self-stress-testing has missed.

## What you should know about the trust posture

Per the orchestrator prompt's `COPILOT-AS-FEEDBACK-PEER`: Copilot feedback output is semi-trusted; the orchestrator weighs your critique rather than blindly executing on it. Your critique can change the orchestrator's recommendation if it surfaces something load-bearing the internal work missed. Eva makes the final call.

The dispatch is at the candidate-selection checkpoint, which is one of three hard Eva-approval checkpoints in the redesign. Your critique informs the iteration, not the final selection.

— main-orchestrator, cycle 119 (2026-05-11)
