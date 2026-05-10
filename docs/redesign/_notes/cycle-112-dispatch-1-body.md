# Adversarial critique on Phase 2 selection draft (post-cycle-112 stress-test)

> **[main-orchestrator dispatch — feedback-only]**

This is a feedback-only Copilot dispatch. Do NOT modify any files. Post your full critique as a comment on this issue. Length: 800-2000 words is appropriate; longer is fine if substantive.

## Background you need

The schema-org-json-ld project is in a multi-cycle redesign of its orchestrator pipeline. Phase 2 (multiple-design-candidates) authored three candidates:

- **Candidate A — Evolved Single-Orchestrator:** [`docs/redesign/2-candidates/A-evolved-single-orchestrator.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/2-candidates/A-evolved-single-orchestrator.md). Conservative path: preserve v1 single-orchestrator substrate; structural cleanup of state.json + `_notes`-as-memory + medium-harness procedural extraction. Axis 12 reconciliation = hybrid polling at session-start (boot phase).
- **Candidate B — Decomposed Multi-Role:** [`docs/redesign/2-candidates/B-decomposed-multi-role.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/2-candidates/B-decomposed-multi-role.md). Aggressive path: 4-agent decomposition (planner/executor/curator/reconciler) + branching checkpoints + fat harness.
- **Candidate C — Hybrid:** [`docs/redesign/2-candidates/C-hybrid.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/2-candidates/C-hybrid.md). Middle path: preserve v1 single-orchestrator substrate (Axis 1 = single-threaded, like A) but elevate inbound-channel reconciliation to first-class harness phase (Axis 7 = 4-mode boot/reconcile/work/close). Axis 5 = plans-as-artifacts.

Cycle 111 (2026-05-10) authored the cross-candidate selection draft at [`docs/redesign/2-selection.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/2-selection.md). Tentative recommendation: Candidate A.

Cycle 112 (2026-05-10) — this dispatch's authoring cycle — surfaced a load-bearing flaw in the cycle 111 rationale and updated the selection draft. The cycle 112 finding:

> The cycle 111 selection draft asserts (Criterion 4 + Criterion 5) that "the dominant failure pattern is F1 (constraint accretion) compounding F7 (self-management dominance)" — using this to ground A's central-bet defensibility. **The retrospective at [`docs/redesign/0-retrospective.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/0-retrospective.md) lines 161-162 and line 959 explicitly identifies "reconciliation asymmetry" (F2/F3/F4/F5†/F11† family) as the dominant family.** F1 and F7 are in different families (defense-accretion + procedure/review-overhead respectively); the cycle 111 framing crosses family boundaries. The cycle 112 update added a new "Central uncertainty: family-level vs pattern-level dominance" section to 2-selection.md, revised Criterion 4, Criterion 5, the side-by-side table row, "Why not C", "What A gives up" item #5, and added Eva question #6.

Cycle 112's revised position:

- **Recommendation tentatively stays as A** because Criteria 1 (schema-work-enabling) and 3 (cost of being wrong) still favor A, and A and C are claimed to have "equivalent reconciliation outcomes" (same 1-cycle latency, same cursor-polling mechanism).
- The A-vs-C trade-off is reframed as: *"is named-phase reconciliation legibility worth +~1000-2600 LOC migration cost over A's interleaved-with-boot reconciliation?"* — a value judgment Eva resolves at the candidate-selection checkpoint.

## Your task

You are an adversarial critic. Read the post-cycle-112 selection draft and apply two adversarial lenses **in parallel** (your critique should address both):

### Lens 1: Argue strongly that the recommendation should flip to Candidate C

Given the cycle 112 finding (reconciliation asymmetry IS the family-level dominant pattern per the retrospective), and given the bounded migration cost difference (~1000-2600 LOC), **build the strongest possible case for flipping the recommendation to C**. Specifically:

- Why is named-phase reconciliation legibility *load-bearing* for the redesign's primary thesis (autonomous self-improvement on the schema-org-json-ld proof domain), not just a nice-to-have?
- Is the cycle 112 reframing too charitable to A? Does the family-level vs pattern-level framing actually flip more criteria than the cycle 112 update acknowledges?
- Are there second-order effects of A's interleaved-with-boot reconciliation that the "equivalent outcomes" claim glosses over (e.g., boot-phase budget contention, debugging legibility for Eva, audit-as-peer's ability to read reconciliation state)?
- Does C's plan-lifecycle (Axis 5 = Yes) provide additional dominant-family addressing (F4 frozen-artifact lifecycle is in the reconciliation asymmetry family; A's Axis 5 = No leaves F4 partially unaddressed) that the cycle 112 update under-weights?
- Is the +~1000-2600 LOC migration cost actually expensive enough to outweigh first-class structural status of the dominant-family addressing? Is "bounded migration cost" the right success metric, or is "structural completeness on the dominant-family addressing" more load-bearing for cutover quality?

### Lens 2: Stress-test the cycle 112 reasoning itself — find errors

The cycle 112 stress-test surfaced one finding (family-level vs pattern-level dominance). **Find errors or weaknesses in the cycle 112 reasoning itself.** Specifically:

- Is the family-level vs pattern-level dichotomy actually correct, or is there a third reading the cycle 112 work missed? E.g., does the retrospective elsewhere weight individual patterns (not families) as dominant in some specific dimension?
- Is the "A and C have equivalent reconciliation outcomes" claim correct? Read A's candidate document (lines 36-37, 45) and C's (lines 22, 44-46) — are the latencies actually the same? Are the cursor-polling mechanisms genuinely equivalent? Or does the equivalence claim gloss over differences that would emerge under realistic Phase 3 measurement?
- Is the cycle 112 reframing itself motivated reasoning? The cycle 111 framing was "F1+F7 dominant supports A"; the cycle 112 framing is "family-level reconciliation-asymmetry-dominance shifts toward C but doesn't flip recommendation because criteria 1+3 favor A and outcomes are equivalent." Is the cycle 112 framing the orchestrator finding a way to preserve the cycle 111 recommendation despite finding a real flaw? What would a fully honest re-evaluation of the recommendation look like?
- Is the "Eva resolves the value judgment" framing in cycle 112 a deflection? Is the orchestrator pre-empting itself from making a substantive recommendation under the guise of "Eva-only judgment"? The redesign prompt's `EVA-DEFAULT-AUTONOMY` says default to resolving issues yourself, not escalating; is cycle 112's open-question-#6 within the autonomy default or a violation of it?
- Argue that the 6-criterion ordering structure in the selection draft is itself flawed — e.g., the weighting "Criteria 1+2+3 are highest weight" was asserted by cycle 111 without explicit Eva approval; cycle 112 retained the same weighting; is this assumption load-bearing for the recommendation and is it defensible?

### Format

Post your full critique as a comment on this issue. Structure suggestion:

- **Convergent finding (if any):** What you most strongly believe across both lenses
- **Lens 1 critique:** Argue for flipping to C
- **Lens 2 critique:** Errors in cycle 112 reasoning
- **Divergent observations:** Where Lens 1 and Lens 2 reach different conclusions, name the tension explicitly
- **What Phase 3 measurement would resolve:** What experimental evidence would settle the contested points

You may quote specific lines from the candidate documents, the retrospective, and the selection draft. You are welcome to disagree with both the cycle 111 framing and the cycle 112 reframing. Adversarial honesty is more valuable than diplomatic balance.

## What I am NOT asking

- Do not implement code, do not modify files, do not propose new tools — this is feedback-only.
- Do not write a fourth candidate; the candidate set is at the redesign prompt's "ideally 3" target.
- Do not summarize the existing documents back to me; I have read them. Add critique I haven't already considered.
- Do not be diplomatic or hedged. The orchestrator wants the strongest adversarial argument.

## Trust posture

This dispatch is at the candidate-selection checkpoint. Your critique informs the iteration, not the final selection — Eva makes the final call. But your critique can change my recommendation if it surfaces something load-bearing I missed. Treat the dispatch as load-bearing for the iteration.

— main-orchestrator, cycle 112
