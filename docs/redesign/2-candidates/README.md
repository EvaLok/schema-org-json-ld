# Phase 2 candidates

**Status:** Phase 2 candidate authoring begins **cycle 90** (2026-05-07) per audit#454 P5 toggle and cycle 89 hand-off. The M-item integration arc closed cycle 89; this directory contains Phase 2 candidate documents authored against the augmented synthesis surface ([`../2-design-framework.md`](../2-design-framework.md) v1.21+ and [`../1-research/clusters.md`](../1-research/clusters.md) M-item layers M1+M2+M3+M5/P6 + 9-cluster comparison).

This directory is the artifact for Phase 2 candidate-selection. Each candidate is a separate file applying the [`2-design-framework.md`](../2-design-framework.md) candidate template (12 axes + cross-axis commitments + F-mapping + preserved-primitives compliance + P1-P6 evaluation criteria + M3 v1 strengths preservation + M2 self-management cost inheritance).

## Cycle 90 candidates

| Candidate | File | Position summary |
|---|---|---|
| **A — Evolved Single-Orchestrator** | [`A-evolved-single-orchestrator.md`](./A-evolved-single-orchestrator.md) | Conservative path: preserve v1 substrate; structural cleanup of state.json + _notes-as-memory + medium-harness procedural extraction. Axis 1 = single-threaded. |
| **B — Decomposed Multi-Role** | [`B-decomposed-multi-role.md`](./B-decomposed-multi-role.md) | Aggressive path: 4-agent decomposition (planner/executor/curator/reconciler) with typed-channel-map + branching checkpoints + fat harness. Axis 1 = small-fixed-team. |

**Authoring rationale:** the redesign prompt requires "at least 2 distinct design candidates, ideally 3" and the cycle 89 hand-off named candidates differing significantly on **Axis 1 (agent decomposition)** as the cycle-90 authoring focal — to stress-test the cluster G role-asymmetric-context implications and the M5/P6 audit-as-peer preservation pattern. Candidates A and B differ on **every axis** materially, not just Axis 1, which provides a wider evaluation surface than Axis-1-only differentiation would.

## Forward work (cycle 91+)

- **Candidate C (hybrid / middle path)** — author as a third candidate adopting candidate A's substrate-preservation bet for cycle-internal decomposition (Axis 1 = single-threaded) while adopting candidate B's structural additions for inbound reconciliation (Axis 12 dedicated reconciler-sub-cycle, not separate role) and plans-as-artifacts (Axis 5 = Yes). Whether C is genuinely distinct from A + targeted additions, or just a partial-B, is the authoring question.
- **Audit critique solicitation** — this directory's content is read by the audit-repo orchestrator on its next cycle (audit cycle 213+). Audit's adversarial critique is part of the candidate-selection checkpoint per the redesign prompt's `<audit-as-peer>` directive.
- **Iterate per `ITERATION-UNTIL-APPROVAL`** — candidate-selection is one of the three hard checkpoints. Eva's explicit approval is the only stopping signal. While awaiting approval, sharpen candidates, solicit additional critique, stress-test load-bearing claims, revisit dismissed alternatives.

## Comparison structure (cycle 91+ work item)

A side-by-side comparison table will be added to this README in cycle 91+ once a third candidate or two iterations have produced enough surface for honest comparison. The current 2-candidate state is the **first iteration**, not the deliverable. Per `ITERATION-UNTIL-APPROVAL`: do not declare ready until Eva approves.

## Selection criteria

When the candidate-selection checkpoint arrives, the evaluation surface is:

1. **Convergent constraints compliance** — every candidate must honor 8 constraints from `2-design-framework.md`. Both A and B honor all 8 by construction.
2. **Failure-mode addressing** — F1-F12 per candidate. Both candidates structurally address F1-F12 via different mechanisms.
3. **Preserved-primitives compliance** — journal / cycle-issue / question-for-eva / git-safety / cycle-runner. Both candidates preserve.
4. **P1-P6 evaluation criteria** (audit#454 absorption, cycle 89) — co-equal evaluation lenses peer to failure-mode coverage.
5. **M3 v1 strengths preservation** — 5 strengths from the cluster-spanning M3 layer.
6. **M2 self-management cost inheritance** — aggregate per-cycle decision overhead.
7. **Migration cost** — Phase 3 prototype effort + Phase 4 cutover risk.
8. **What the candidate gives up** — honest list of weaknesses; trade-off legibility.

The **most-discriminating axes** between A and B for cycle 90's first iteration:

- **P3 (self-management-reduction):** A passes decisively (~20 sub-shapes adopted, low aggregate cost); B is PARTIAL-FLAG (~43 sub-shapes adopted, role-specialization mitigates per-role cost but aggregate is high).
- **Migration cost:** A is bounded (~9 new Rust crates); B is multi-cycle (~12+ crates + ~20-40 skill crates).
- **F-pattern structural depth:** A relies on Axis 13 medium-harness extraction; B relies on Axis 1 role-specialization. Different mechanisms; both structural.
- **Audit-as-peer P6 preservation:** A is 5/5 PASS with one PASS-WITH-WORK; B is 5/5 PASS with structural fit (reconciler agent IS the cluster G clean-context-reviewer pattern at session level).

## Iteration discipline

This directory is **not the deliverable** until Eva approves at the candidate-selection checkpoint. Per the redesign prompt's `ITERATION-UNTIL-APPROVAL`:

- **Awaiting approval is not idling.** Each cycle while awaiting approval should sharpen candidates, solicit critique, stress-test claims, deepen reference research, tighten security analysis, examine for self-congratulation, or check for over/under-prescription mismatches.
- **No internal "good enough" stopping signal.** If the candidates feel "done," look harder.
- **Honest negative results have value.** If a critique surfaces nothing real, document the critique and what it found, then move to a different angle.

Eva can approve a candidate, override the orchestrator's selection, request a third candidate, request modifications to an existing candidate, or extend the iteration window.
