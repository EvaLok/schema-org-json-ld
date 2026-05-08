# Phase 2 candidates

**Status:** Phase 2 candidate authoring begins **cycle 90** (2026-05-07) per audit#454 P5 toggle and cycle 89 hand-off. The M-item integration arc closed cycle 89; this directory contains Phase 2 candidate documents authored against the augmented synthesis surface ([`../2-design-framework.md`](../2-design-framework.md) v1.21+ and [`../1-research/clusters.md`](../1-research/clusters.md) M-item layers M1+M2+M3+M5/P6 + 9-cluster comparison). **Cycle 91** (2026-05-08) added Candidate C, reaching the redesign prompt's "ideally 3" target.

This directory is the artifact for Phase 2 candidate-selection. Each candidate is a separate file applying the [`2-design-framework.md`](../2-design-framework.md) candidate template (12 axes + cross-axis commitments + F-mapping + preserved-primitives compliance + P1-P6 evaluation criteria + M3 v1 strengths preservation + M2 self-management cost inheritance).

## Cycle 90+91 candidates

| Candidate | File | Position summary |
|---|---|---|
| **A — Evolved Single-Orchestrator** | [`A-evolved-single-orchestrator.md`](./A-evolved-single-orchestrator.md) | **Conservative path:** preserve v1 substrate; structural cleanup of state.json + _notes-as-memory + medium-harness procedural extraction. Axis 1 = single-threaded. |
| **B — Decomposed Multi-Role** | [`B-decomposed-multi-role.md`](./B-decomposed-multi-role.md) | **Aggressive path:** 4-agent decomposition (planner/executor/curator/reconciler) with typed-channel-map + branching checkpoints + fat harness. Axis 1 = small-fixed-team. |
| **C — Hybrid** | [`C-hybrid.md`](./C-hybrid.md) | **Middle path:** preserve v1 single-orchestrator substrate (Axis 1 = single-threaded, like A) but elevate inbound-channel reconciliation to first-class harness phase (Axis 7 = 4-mode) and adopt plans-as-artifacts (Axis 5 = Yes, like B) — without paying multi-agent overhead. Authored cycle 91. |

**Authoring rationale:** the redesign prompt requires "at least 2 distinct design candidates, ideally 3." Cycle 90 authored A + B differing significantly on **Axis 1 (agent decomposition)** to stress-test the cluster G role-asymmetric-context implications and the M5/P6 audit-as-peer preservation pattern. Cycle 91 added C — the explicit middle path testing whether one specific concern (inbound reconciliation) deserves first-class harness phase status without paying full role-decomposition cost. The 3-candidate set is now at the redesign prompt's "ideally 3" target.

## Side-by-side 12-axis comparison

| Axis | Candidate A (single-orchestrator) | Candidate B (multi-role) | Candidate C (hybrid) |
|---|---|---|---|
| **1 (decomposition)** | single-threaded | small-fixed-team (4 agents) | single-threaded |
| **2 (state)** | file-per-component | typed-channel-map | file-per-component |
| **3 (memory)** | wiki+search | top-level-architectural-principle, per-agent channels | wiki+search |
| **4 (history)** | git-as-substrate | branching checkpoints in-tree | git-as-substrate + plan-lifecycle states |
| **5 (plans-as-artifacts)** | No | Yes | **Yes** |
| **6 (extension)** | tools-with-registry | skills | tools-with-registry |
| **7 (topology)** | sequential 3-mode | multi-pattern coexisting | **sequential 4-mode (boot/reconcile/work/close)** |
| **8 (mechanical enforcement)** | behavioral-prose CI | per-role + per-channel CI | behavioral-prose CI + plan-lifecycle CI |
| **9 (iteration ceilings)** | loop count | loop + runtime per role | loop + per-mode runtime budget |
| **10 (entropy mitigation)** | minimal deslop | full + dedicated curator role | minimal deslop + plan-lifecycle gardening hooks |
| **12 (reconciliation)** | hybrid (polling + event-driven) | event-driven via reconciler agent | **event-driven with cycle-cadence fallback** |
| **13 (harness-vs-session)** | medium harness | fat harness | medium harness with reconciler-mode |

**Bold cells in C** are positions where C is structurally distinct from A. Five A-vs-B differences are preserved (Axes 1, 2, 3, 6, 13 in C track A; not B). Three C-vs-A distinctions (Axes 5, 7, 12) reflect C's central bet that elevating inbound-reconciliation to first-class harness phase status is load-bearing without paying multi-agent overhead.

## Side-by-side P1-P6 + M3 + M2 + migration comparison

| Criterion | Candidate A | Candidate B | Candidate C |
|---|---|---|---|
| **P1 (A↔B intersection coverage)** | PASS | PASS | PASS |
| **P2 (cluster I substrate-fit)** | PASS | PASS | PASS |
| **P3 (self-management-reduction)** | PASS | PARTIAL-FLAG | PASS-WITH-NOTE |
| **P4 (lifecycle-vocabulary completeness)** | PARTIAL (bypass clause) | PASS (cluster C sub-shapes 1-5) | PASS-WITH-WORK (cluster C sub-shapes 4+5 only) |
| **P5 (polarity-pivot exit)** | PASS at authoring | PASS at authoring | PASS at authoring |
| **P6 (audit-as-peer preservation)** | 5/5 PASS-WITH-WORK on Criterion 4 | 5/5 PASS (structural fit) | 5/5 PASS (dedicated reconcile-mode) |
| **M3 strengths** | 5/5 PRESERVED | 5/5 PRESERVED-or-EXTENDED | 5/5 PRESERVED, 2 EXTENDED (S2, S3) |
| **M2 cost (LOW + MOD + HIGH)** | 12 + 8 + 1 (~21 sub-shapes) | 16 + 22 + 5 (~43 sub-shapes) | 13 + 10 + 1 (~24 sub-shapes) |
| **Migration: new Rust crates** | ~9 | ~12+ Rust + 20-40 skill | ~11 |
| **Migration: net-add LOC** | ~3000-4500 | ~10000-20000 | ~4000-6000 |
| **Migration: cutover scope** | bounded single-cycle per crate | multi-cycle build-out | bounded single-cycle per crate |
| **Cutover predictability** | high | low | medium-high |

**Most-discriminating criteria across the 3 candidates:**

- **P3 (self-management-reduction):** A passes decisively; C passes with note (3 more sub-shapes than A); B is PARTIAL-FLAG (~43 sub-shapes; per-role mitigation real but unproven). **P3 is the criterion that most clearly orders the three candidates.**
- **P4 (lifecycle-vocabulary completeness):** A bypasses cluster C; C adopts 2/5 sub-shapes (replay + event-trigger); B adopts 5/5 sub-shapes. **P4 is the criterion where C bridges A and B with PARTIAL completeness.**
- **Migration cost:** A is ~3000-4500 LOC; C is ~4000-6000 LOC; B is ~10000-20000 LOC. **C migration cost is closer to A than B; bounded single-cycle scope per crate.**
- **F4 (frozen-artifact lifecycle):** A has no plan-lifecycle (implicit); C has plan-lifecycle states (active/completed/technical-debt); B has plan-lifecycle plus branching-checkpoints. **C bridges A and B on F4 structural addressing.**
- **F2 (Eva-response detection):** A polls in boot-phase (interleaved); C runs first-class reconcile-mode; B has dedicated reconciler agent. **C bridges A and B on F2 structural addressing without paying full agent cost.**

## Forward work (cycle 92+)

- **Sharpen all three candidates per `ITERATION-UNTIL-APPROVAL`** — each candidate has named weak points: A's Axis 13 medium harness extraction percentage estimate; B's per-role decision count empirical estimate; C's central bet (whether reconcile-mode + plans-as-artifacts carry their weight) is the candidate's named uncertainty. Cycles 92+ should sharpen these specific weak points.
- **Solicit Copilot feedback dispatch** — per redesign prompt's `<copilot-as-feedback-peer>`, dispatch a feedback-only Copilot session pointing at this directory and asking for adversarial critique. Multiple parallel dispatches with different lenses (e.g., one focused on F-pattern coverage; one on P1-P6; one on tool surface feasibility) increase the critique surface.
- **Audit critique solicitation** — this directory's content is read by the audit-repo orchestrator on its next cycle. Audit's adversarial critique is part of the candidate-selection checkpoint per the redesign prompt's `<audit-as-peer>` directive.
- **Iterate per `ITERATION-UNTIL-APPROVAL`** — candidate-selection is one of the three hard checkpoints. Eva's explicit approval is the only stopping signal. While awaiting approval, sharpen candidates, solicit additional critique, stress-test load-bearing claims, revisit dismissed alternatives.

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
