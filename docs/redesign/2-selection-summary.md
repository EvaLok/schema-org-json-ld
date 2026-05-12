# Phase 2 candidate selection — checkpoint summary

**Status (cycle 120, 2026-05-11):** **Tentative recommendation is Q7-dependent.** The 6-criterion structure is approximately neutral (cycle 119 Finding 2); the lever is Eva's resolution of **Q7 — the weighting between Criteria 4+5 cumulative findings vs Criteria 1+3 migration-cost**. Authored cycle 120 in response to [#2910 / PR #2911](https://github.com/EvaLok/schema-org-json-ld/pull/2911) Copilot adversarial-feedback critique (L2: the iterative selection draft has grown to ~990 lines — optimized for orchestrator self-reassurance rather than Eva-legibility; ITERATION-UNTIL-APPROVAL satisfied at letter, violated in spirit).

This summary IS the Eva-legible deliverable. The deep history of the cycle 111-120 iteration is preserved at [`2-selection.md`](./2-selection.md) (~990 lines) for those reading the full trajectory; this file is what the candidate-selection checkpoint approval rests on.

## The single load-bearing question

**Q7 — Which weighting does Eva approve?**

| Weighting choice | Selection |
|---|---|
| **(a)** Criteria 4+5 cumulative findings outrank Criteria 1+3 migration-cost weight | **Candidate C (Hybrid)** |
| **(b)** Primary-thesis Criterion 1 + cost-of-being-wrong Criterion 3 outrank Criteria 4+5 cumulative findings | **Candidate A (Evolved Single-Orchestrator)** |
| **(c)** Different weighting rule (e.g., Criterion 5 central-bet defensibility is the single load-bearing criterion; Criterion 1 is a constraint not the primary axis) | Eva names; orchestrator implements |

The three candidates are of comparable merit on different dimensions. The choice is a value resolution Eva owns — not a technical determination the orchestrator can make alone.

## The three candidates in one sentence each

- **[A — Evolved Single-Orchestrator](./2-candidates/A-evolved-single-orchestrator.md):** preserve v1's single-orchestrator substrate; extract medium-harness procedural surface via Axis 13; structural cleanup of state.json + _notes-as-memory. Migration: ~3600-6200 LOC, per-crate single-cycle cutover. Wins Criteria 1+2+3.
- **[B — Decomposed Multi-Role](./2-candidates/B-decomposed-multi-role.md):** 4-agent decomposition (planner/executor/curator/reconciler) with typed-channel-map + branching checkpoints + fat harness. Migration: ~14000-28000 LOC, 16-30 cycle cutover. Wins Criterion 6 narrowly (audit-as-peer fit).
- **[C — Hybrid](./2-candidates/C-hybrid.md):** preserve A's single-orchestrator substrate but elevate inbound-channel reconciliation to first-class harness phase (Axis 7 = 4-mode) and adopt plans-as-artifacts (Axis 5 = Yes). Migration: ~5200-8200 LOC, bounded single-cycle per A-shared crate + 2-3 cycles each for reconcile-mode + plan-lifecycle. Wins Criteria 4+5.

## What each gives up

| | Loses on | Why this matters |
|---|---|---|
| **A** | F2/F4/F11 detection legibility weaker than C/B (interleaves reconciliation with boot phase); no plans-as-artifacts; F4/F11 addressing depth depends on Phase 3 measurement | Under family-level reading of the retrospective (reconciliation asymmetry as dominant family per lines 161-162 + 959), legibility on the dominant family is structurally important |
| **B** | Migration cost 3-5× A's; inverts CORE-DESIGN-PRINCIPLE at prompt level (~50% larger aggregate prompt content); central bet on cluster G role-asymmetric context is corpus-pattern-grounded not retrospective-grounded (cycle 118 finding); P3 self-management-cost PARTIAL-FLAG | The redesign's primary thesis cannot rest on a candidate that's conditionally schema-work-enabling at 3-5× migration cost |
| **C** | Conditional-improvement risk (3 additional sub-shapes must carry their weight); PR #2877 names C as most under-justified estimate posture; per-mode runtime budgets add complexity at unproven value | "C is just A with more migration surface" if the additions don't carry their weight; direction is structurally validated (cycle 92 + cycle 96 + cycle 107), magnitude is Phase 3 measurement |

## How the recommendation got to "Q7-dependent" cycle 120

Cycles 111-119 iterated extensively on the cross-candidate comparison. The orchestrator's tentative recommendation moved A (cycles 111-113) → C (cycle 114 flip per the four-finding flip threshold) → C-with-progressive-qualifications (cycles 115-119). The cycle 119 [#2910](https://github.com/EvaLok/schema-org-json-ld/issues/2910) Copilot adversarial-feedback dispatch surfaced three substantive findings that shift the tentative recommendation to **Q7-dependent** rather than a unilateral C:

1. **The cycle 113 → cycle 115 flip-back threshold introduced a strength criterion ("substantively undermining") that the original four-finding threshold did not apply** ([#2911](https://github.com/EvaLok/schema-org-json-ld/pull/2911) L1 Finding 2). The asymmetry is real: cycle 114 honored the original threshold to flip A→C without applying the strength criterion; cycles 116-117 then applied the strength criterion to classify subsequent findings as "qualifying-not-substantively-undermining." This is an asymmetry of application, not language. Honest action: the orchestrator's classification of cycle 116-117 findings is self-referential; Eva's Q7 resolution should be the deciding lever, not the orchestrator's strength-criterion judgment.

2. **Migration cost can be classified as Criterion 1 (schema-work-enabling) rather than only Criterion 3 (cost-of-being-wrong)** ([#2911](https://github.com/EvaLok/schema-org-json-ld/pull/2911) L1 Finding 3). Migration cycles ARE cycles not in the proof domain — directly a Criterion 1 operand. Under this reframing, A's migration cost advantage (~3600-6200 LOC vs C's ~5200-8200 LOC PR #2877 revised) becomes additive to A's Criterion 1 lead. The cycle 111 classification of migration cost as Criterion 3 separately from Criterion 1 is a framing choice that systematically tilted the analysis toward C cycle 114+. Honest action: name this as part of Q7's resolution space.

3. **Zero operational failure evidence exists for either A or C** ([#2911](https://github.com/EvaLok/schema-org-json-ld/pull/2911) L1 Finding 4). All evidence is theoretical / structural. The choice depends on Eva's weighting of theoretical arguments, not on empirical disambiguation that hasn't happened yet. Honest action: name this symmetric uncertainty rather than presenting C's direction-validation as decisive evidence.

The orchestrator's tentative recommendation therefore shifts cycle 120 to **Q7-dependent** rather than C-unilaterally. This is more honest than the cycle 114-119 framing.

**Cycle 126 [audit#462](https://github.com/EvaLok/schema-org-json-ld-audit/issues/462) refinement** (audit cycle 217 substantive critique; third V2 audit-engagement, first via implicit-ask channel): two additions to Q7's resolution surface.

- **D1 — cycles 122-125 A-shared scaffolding is A∪C-scoped architecture evidence**, not domain-agnostic evidence. The cumulative LOC trajectory (~4440 prod / 9-crate aggregate; ~10212 with tests) estimates LOC for *orchestrator-invoking-primitives at single-orchestrator substrate*. 4 of 5 measured crates (`v2-tool-registry`, `v2-cycle-history-append`, `v2-phase-transition-check`, `v2-wiki-search`) are **no-regret primitives** whose implementations would carry residual value in B with different usage patterns; only `v2-boot-phase` (cycles 123-124) is A∪C-specific in its 5-sub-responsibility bundling that B would split across planner/executor/reconciler agents. The cumulative architecture extrapolation does NOT generalize to B's multi-agent topology.
- **D5 — cost-of-being-wrong asymmetry across Q7 options should be explicit**, not only implicit in Criterion 3. The asymmetry is dominantly between A/C (bounded weeks) and B (months by 2.7×-4.5×). See the table in "What this means for Phase 3" below.

Other audit#462 findings (D2 D3 D4 + missing patterns M1-M5 + Phase 3 implications P3-1 through P3-8) are absorbed in [`_notes/cycle-126-audit-217-absorption-D1-D5.md`](./_notes/cycle-126-audit-217-absorption-D1-D5.md) for cycle 127+ work and Phase 3 design requirements.

## What this means for Phase 3

- **If Eva approves Q7 option (a) — select C:** Phase 3 begins with C's `reconcile-mode` + plan-lifecycle as the prototype targets; the orchestrator's tentative C recommendation is confirmed.
- **If Eva approves Q7 option (b) — select A:** Phase 3 begins with A's `boot-phase` + `wiki-search` + medium-harness extraction as prototype targets; the cycle 114 flip is reversed honestly per Q7's resolution.
- **If Eva approves Q7 option (c) — different weighting rule:** the orchestrator implements the rule and proceeds accordingly.
- **If Eva extends the iteration window (Q5):** the orchestrator continues per `ITERATION-UNTIL-APPROVAL`, with priority on legibility-restoration over more recursive annotation (the [#2911](https://github.com/EvaLok/schema-org-json-ld/pull/2911) L2 critique stands as a constraint on further iteration).

**Cost-of-being-wrong asymmetry across Q7 options** (cycle 126 D5 absorption per [audit#462](https://github.com/EvaLok/schema-org-json-ld-audit/issues/462)):

| Q7 option | Selection | Rollback LOC if Phase 3 refutes the central bet | Recovery time-frame |
|---|---|---|---|
| (a) | C | ~5200-8200 | bounded weeks |
| (b) | A | ~3600-6200 | bounded weeks (lowest) |
| (c) selecting B | B | ~14000-28000 | months (2.7×-4.5× higher) |

The asymmetry is dominantly between A/C (bounded weeks, recoverable within Phase 3's normal iteration window) and B (months). Within A/C, the difference is bounded. Selecting B materially raises the recovery cost should Phase 3 measurement refute B's central bet on cluster G role-asymmetric context. The dimension is implicit in Criterion 3 (cost-of-being-wrong) but is structurally pivotal between A/C and B — material to Q7 option (c).

## Other Eva-resolves items (lower priority than Q7)

Q1-Q9 in [`2-selection.md`](./2-selection.md#open-questions-for-eva-at-the-candidate-selection-checkpoint) detail nine resolvable questions. The cycle 120 honest assessment is that **Q7 subsumes most of the others**:
- Q1+Q2 (approve C / override to A) reduce to Q7's resolution.
- Q6+Q9 (family-level vs pattern-level dominance reading) affect Criterion 4+5 ordering under Q7 option (a).
- Q8 (strength-criterion interpretation) reduces to Q7's framing if the orchestrator's strength-classification is treated as self-referential (per [#2911](https://github.com/EvaLok/schema-org-json-ld/pull/2911) L2).
- Q3 (pick B) and Q4 (additional candidate) are separate dimensions Eva can name independently.
- Q5 (extend iteration window) is procedural — orchestrator continues iterating until approval lands.

## How to approve

- **At the candidate-selection checkpoint:** Eva resolves Q7 explicitly (option a, b, or c). The orchestrator implements per the resolution.
- **If Eva wants more time / different framing:** comment on [#2912](https://github.com/EvaLok/schema-org-json-ld/issues/2912) (the cycle 120 issue) or open an `[input-from-eva]` issue; the orchestrator continues iterating with the named focus.
- **If Eva wants to override the candidate set entirely:** name the alternative; the orchestrator authors a fourth candidate and re-runs the comparison.

## What deep-detail readers want

- **Cycle-by-cycle iteration log:** [`2-selection.md#iteration-log`](./2-selection.md#iteration-log) (cycles 111-120).
- **The cycle 114 flip rationale + cycle 115-119 qualifications:** [`2-selection.md`](./2-selection.md) sections "Cycle 114 stress-test" through "Cycle 119 6-criterion ordering structure stress-test."
- **The Copilot critique that prompted the cycle 120 recommendation shift:** [PR #2911](https://github.com/EvaLok/schema-org-json-ld/pull/2911) body.
- **The per-candidate documents:** [`2-candidates/A-evolved-single-orchestrator.md`](./2-candidates/A-evolved-single-orchestrator.md), [`B-decomposed-multi-role.md`](./2-candidates/B-decomposed-multi-role.md), [`C-hybrid.md`](./2-candidates/C-hybrid.md).
- **The candidate-comparison framework:** [`2-design-framework.md`](./2-design-framework.md) (P1-P6 criteria, F-pattern mapping, M3 strengths preservation, M2 self-management cost).
- **The retrospective grounding the failure-mode discussion:** [`0-retrospective.md`](./0-retrospective.md).
