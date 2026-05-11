# Cycle 121 — Risk-closure threshold summary tables consolidated into per-candidate documents + #2910/PR #2911 housekeeping closure

**Date:** 2026-05-11 (~11:03 UTC start)
**Cycle issue:** [#2913](https://github.com/EvaLok/schema-org-json-ld/issues/2913)
**Mode:** redesign Phase 2 candidate iteration under `ITERATION-UNTIL-APPROVAL` — **thirty-second cycle of Phase 2 candidate-set work** [cycles 90-121]

**Substantive focal:** cycle 120 forward priority #5 — cycle 103-110 risk-closure threshold table integration into per-candidate documents. Bounded mechanical-substantive work that explicitly honors the cycle 120 L2 critique's ban on more 2-selection.md annotation while surfacing Phase 3 measurement substrate in a legibility-restoring form.

**Seventh consecutive cycle of honoring named forward priority** (cycles 115-121). The cycle 120 _notes named cycle 103-110 risk-closure threshold table integration as cycle 121+ priority #5 (priorities #1 Q7 + #2 audit 216 were Eva/audit-blocked; cycle 121 picked the highest-priority orchestrator-drivable item).

## Setup

Cycle 121 session-start (11:03 UTC) found:
- Audit cycle 216 ([#460](https://github.com/EvaLok/schema-org-json-ld-audit/issues/460)) OPEN with no commits since cycle 215 (2026-05-10) — audit-blocked.
- No new Eva input observed since standing directives — Q7 Eva-blocked.
- [#2910](https://github.com/EvaLok/schema-org-json-ld/issues/2910) / [PR #2911](https://github.com/EvaLok/schema-org-json-ld/pull/2911) absorbed cycle 120; both OPEN at cycle 121 session-start (cycle 120 _notes named "preserved as substrate; will close in a future cycle with forward-link to absorption commit per HOUSEKEEPING discipline").

## Methodology

The cycles 103-110 sequence closed 7 risks across the three candidates at the specification level:
- **Cycle 103:** B's Risk 8 (counting protocol)
- **Cycle 105:** A's Risk 3 (tool-registry growth)
- **Cycle 106:** B's Risk 2 (reducer-rule revision rate)
- **Cycle 107:** C's Risk 2 (plan-authoring discipline)
- **Cycle 108:** B's Risk 4 (within-super-step iteration)
- **Cycle 109:** A's Risk 4 (prompt-contract-check)
- **Cycle 110:** C's Risk 5 (plan-lifecycle CI)

Each closure follows shape #21 (substrate-decomposition + falsifiable bound + verification + status). The closure substrate is currently scattered across per-cycle `### Risk N status post-cycle-N` sections in the per-candidate documents (and partly in the per-cycle `_notes` records).

Cycle 121 authored a consolidated **Risk-closure threshold summary** section in each per-candidate document, placed before the existing validation plan section, containing:
- Brief description of each closure
- Closure cycle
- Primary falsifiable threshold (with explicit threshold values where relevant)
- Verification mechanism (three-layer pattern + warm-up window + combined-readings diagnostic where applicable)
- Phase 3 operational closure target (the conditions for moving from specification-level closure to operational closure)
- Cross-candidate context paragraph (distribution + risk-shape-type-mix)
- Substrate-pattern observation paragraph (shape #21 HARDENED-at-7 / 3-layer verification candidate for shape #25)
- Candidate-specific note (C's discipline-conditional pairing with A's Risk 4; B's closure-count vs central-bet readiness; A's quantity-bounded + discipline-conditional mix)

## Edits

- **`docs/redesign/2-candidates/A-evolved-single-orchestrator.md`** — +13 lines:
  - New `## Risk-closure threshold summary (cycle 121 consolidation)` section between the existing "Risk 4 status post-cycle-109" closure and the "Cycle 93+94 prototype scaffolding: migration-cost validation" section
  - 2-row table covering Risks 3 (cycle 105) + 4 (cycle 109)
  - Cross-candidate context + substrate-pattern observation paragraphs
- **`docs/redesign/2-candidates/B-decomposed-multi-role.md`** — +16 lines:
  - New `## Risk-closure threshold summary (cycle 121 consolidation)` section between the existing "Risk 4 status post-cycle-108" closure and the "Validation plan (cycle 93+ Phase 3 prototype work)" section
  - 3-row table covering Risks 8 (cycle 103) + 2 (cycle 106) + 4 (cycle 108)
  - Cross-candidate context + substrate-pattern observation paragraphs + B-specific note on closure-count vs central-bet readiness (Risks 9/10 remain open at structural level)
- **`docs/redesign/2-candidates/C-hybrid.md`** — +15 lines:
  - New `## Risk-closure threshold summary (cycle 121 consolidation)` section between the existing "Risk 5 status post-cycle-110" closure and the "Risks named at the structural level" section
  - 2-row table covering Risks 2 (cycle 107) + 5 (cycle 110)
  - Cross-candidate context + substrate-pattern observation paragraphs + C-specific note on closure-paired structure with A's Risk 4 + retrospective-failure-mode-grounding linkage
- **`docs/redesign/2-selection.md`** — **NO EDITS** (cycle 120 L2 ban honored)
- **`docs/redesign/2-selection-summary.md`** — **NO EDITS** (Eva-legible deliverable preserved unchanged pending Q7 resolution)
- **`docs/redesign/_notes/cycle-121-...`** (NEW — this file)
- **`docs/journal/2026-05-11.md`** — cycle 121 entry (separately authored)

## Housekeeping (cycle 120 forward work item)

Per HOUSEKEEPING discipline, closed [PR #2911](https://github.com/EvaLok/schema-org-json-ld/pull/2911) and [#2910](https://github.com/EvaLok/schema-org-json-ld/issues/2910) with forward-link comments to the cycle 120 absorption commit ([4f5d3776](https://github.com/EvaLok/schema-org-json-ld/commit/4f5d3776)) and the absorbed-content destinations:
- L2 critique remedy → `docs/redesign/2-selection-summary.md`
- Deep-detail absorption → `docs/redesign/2-selection.md` cycle 120 section
- Per-Lens evaluation → `docs/redesign/_notes/cycle-120-2910-critique-absorption-and-summary-file-authoring.md`

The cycle 120 _notes explicitly named these for closure "in a future cycle with forward-link to absorption commit" — cycle 121 honored this future-named work.

## Verification (compressed)

| Item | Pre-cycle-121 | Post-cycle-121 |
|---|---|---|
| Per-candidate risk-closure substrate location | Scattered in per-cycle `### Risk N status post-cycle-N` sections | Consolidated tables added at the per-candidate document level, preserving deep-detail sections as the substrate source |
| A-evolved-single-orchestrator.md line count | 470 | 483 (+13) |
| B-decomposed-multi-role.md line count | 626 | 642 (+16) |
| C-hybrid.md line count | 553 | 568 (+15) |
| 2-selection.md line count | 1084 | 1084 (unchanged — L2 ban honored) |
| 2-selection-summary.md line count | 74 | 74 (unchanged) |
| PR #2911 state | OPEN (preserved as substrate cycle 120) | CLOSED with forward-link comment |
| #2910 state | OPEN (preserved as substrate cycle 120) | CLOSED with forward-link comment |
| Cycle-121 _notes record | — | ✓ this file |
| Cycle 121 journal entry | — | ✓ created |
| Cycle 121 substantive forward-priority honoring | Cycle 120 named priority #5 | ✓ Honored — seventh consecutive cycle of HONORING named forward priority (cycles 115-121) |

## What surprised me / what I noticed

1. **The "risk-closure threshold table integration" forward-priority was easier to execute than the cycle-120-named alternatives.** Cycles 113-120 named this priority but kept deferring it for cycles 118+119+120 higher-marginal-value work (B-side stress-tests, framework stress-test, external critique absorption). Cycle 121 was a natural cycle to land it — Q7 is Eva-blocked, audit 216 is audit-blocked, and the cycle 120 L2 critique explicitly redirects substrate to per-candidate documents. The deferred work became the cleanest available next step.

2. **The consolidation surfaces a cross-candidate substrate-pattern that wasn't legible from the scattered per-cycle sections.** All 7 closures share the 4-element shape #21 structure (substrate-decomposition + falsifiable bound + verification + status); 4 of 6 closures share the 3-layer verification pattern (per-event log + external-observer reproducibility + CI sweep). The 3-layer pattern is a candidate for shape #25 promotion to HARDENED@3 at cycle 111+ per cycle 110 substrate. Reading per-cycle sections individually does not surface this cross-cutting pattern; the table consolidates the substrate enough to make the pattern visible. **External-substance ratio observation:** the consolidation is a form of internal substrate-distillation; it does not introduce new content but improves legibility for cross-cutting pattern recognition.

3. **Distribution of closure counts (A:2 / B:3 / C:2) is approximately balanced; risk-shape-type-mix is more informative than count alone.** A: 1 quantity-bounded (Risk 3) + 1 discipline-conditional (Risk 4). B: 3 quantity-bounded (Risks 8 + 2 + 4). C: 2 discipline-conditional (Risks 2 + 5; the cycle 107 NOVEL@1 → cycle 110 HARDENED@3 discipline-conditional rubric-fragility diagnostic both ground here). The risk-shape-type-mix maps onto the cycle 119 Finding 2 observation that the 6-criterion structure does NOT structurally favor any one candidate — closure-count distribution maps similarly, and the lever for differentiation remains Q7 weighting (per cycle 119 Finding 2 + cycle 120 absorption shift to Q7-dependent recommendation).

4. **C's note on closure-paired structure with A's Risk 4 is structurally important.** Both A's Risk 4 (prompt-contract-check CI) and C's Risk 5 (plan-lifecycle CI) address CI-invariant-coverage drift at different layers; both follow the same 5-type taxonomy + falsifiable ratio + drift-window + 3-layer verification + warm-up window structure. The structural pairing means: **regardless of which candidate is selected, the CI-invariant-coverage discipline is part of v2's design substrate.** This is cross-candidate substrate that survives Q7 resolution. The pairing was implicit pre-cycle-121; the table consolidation makes it explicit.

5. **The cycle 120 L2 constraint operating as a structural discipline.** Cycle 121's substantive focal was the cycle 120 forward priority #5, which explicitly directs substrate to per-candidate documents NOT 2-selection.md. 2-selection.md is unchanged at 1084 lines post-cycle-121; the cycle 120 L2 critique stands as a working constraint on cycle 121+ iteration. **Pattern:** the cycle 120 L2 critique was load-bearing in its substantive findings AND in its forward-going constraint specification. The cycle 121 work demonstrates the L2 ban is not just a one-cycle correction but a multi-cycle discipline.

6. **Housekeeping coupled with substantive work cycle 121.** The HOUSEKEEPING section says "Treat housekeeping as periodic bounded-mechanical work, not a separate phase. A cycle that's heavy on substantive work doesn't need to sweep; a cycle with bounded-work capacity can include 2-4 closures with linking comments." Cycle 121's substantive work (table consolidation in 3 per-candidate docs) is bounded; the cycle had capacity for 2 housekeeping closures (PR #2911 + #2910) without significant scope creep. The housekeeping closures also have direct linkage to the cycle 121 substantive work — they close the absorption substrate that cycle 120 produced — so they fit naturally with the cycle's substantive focal.

7. **The cycle 120-named "lower frequency but higher external-substance ratio" discipline shift is honored cycle 121.** Cycle 121 did NOT dispatch another Copilot adversarial-feedback session, even though that was named as cycle 120+ priority #8. The honest reason: cycle 121 is one cycle after cycle 120 absorbed the previous external critique; a fresh dispatch right now would not honor the discipline shift. The cycle 120 L2 critique's "lower frequency but higher external-substance ratio" is a multi-cycle discipline, not a one-cycle pause.

## Pattern updates summary

- `discretionary-departure-from-forward-going-commitment` HARDENED-at-4 (cycle 114) → **seventh consecutive HONORING cycle 121** (cycles 115-121 all honored named forward priorities; qualified version `departure-WHEN-HIGHER-PRIORITY-SURFACES` increasingly well-supported with 11 cycles of substrate).
- Other patterns unchanged. No new candidate-patterns from cycle 121 substrate (consolidation is internal substrate-distillation, not pattern-generating).

## Bottleneck state at cycle 121 session-end

- Audit cycle 216 ([#460](https://github.com/EvaLok/schema-org-json-ld-audit/issues/460)) OPEN at session-end with no commits since cycle 215 (2026-05-10) — if 216 stalls (A4 silent zero-output pattern recurrence), audit cycle 217 (~04:00 UTC 2026-05-12) is the next landing opportunity.
- [#2910](https://github.com/EvaLok/schema-org-json-ld/issues/2910) and [PR #2911](https://github.com/EvaLok/schema-org-json-ld/pull/2911) CLOSED cycle 121 with forward-link comments per HOUSEKEEPING discipline.
- No new Eva input observed since standing directives.

**Cycle 121 is the thirty-third consecutive bottleneck-asynchronous cycle (cycles 78-121) AND the eleventh consecutive non-per-candidate-sharpening cycle since cycle 102 (cycles 111-121).**

## Forward work for cycle 122+

In priority order (cycle 120 priority list, with cycle 121 priority #5 now landed):

1. **Q7 resolution by Eva** — load-bearing for the recommendation; remains Eva-blocked.
2. **Audit cycle 216/217 critique absorption** — once landed; provides second external lens substrate-content-distinct from Copilot's.
3. **Per-candidate Phase 3 prototype evidence deepening** — extending cycles 93-94 measurements to A's `boot-phase` + `wiki-search`; C-side `reconcile-mode` if Eva pre-approves; both load-bearing under Q7-dependent recommendation; B's `role-driver` + `channel-router` + `super-step-boundary` for Risk 8 counting protocol operationalization.
4. **NO further recursive annotation of 2-selection.md** — cycle 120 L2 critique constraint continues to apply.
5. **Symphony deeper-read elevation** — Phase 1 research forward.
6. **oh-my-claudecode deeper-read elevation** — Phase 1 research forward.
7. **Additional Copilot feedback dispatches** with different lenses if Q7 resolution surfaces specific framings worth external critique; lower frequency than cycle 119-120 cadence per cycle 120 L2 discipline shift.

## Meta-observation: bounded-substantive cycle as a legitimate rhythm

The cycle 120 L2 critique surfaced that 8 consecutive cycles of recursive stress-testing produced an artifact that grew less Eva-legible. Cycle 121 demonstrates a different cycle shape: bounded-substantive work that consolidates existing substrate into a more legible form, paired with housekeeping closure of absorbed material. The cycle's substantive focal is genuinely bounded (~44 lines total added across 3 documents); the cycle did not need to "fill" 75 minutes of compute with manufactured work.

**Implication for v2 design:** the new pipeline should not encode an expectation that every cycle produces substantial novel substrate. Cycles that consolidate, restore legibility, close absorbed material, or honestly identify that the highest-priority work is bottleneck-blocked should be legitimate cycle outputs. The cycle 120 L2 critique's "artifact grew less approvable across 8 cycles" failure mode was driven in part by the cycle-shape expectation that each cycle must add substrate; if the expectation had been "each cycle does the highest-marginal-value thing available, including stop-and-stabilize," the cycle 111-119 iteration would likely have produced fewer recursive stress-test layers and more legibility-restoration work earlier.

**Implication for cycle 122+:** continue the cycle 121 pattern of "highest-priority orchestrator-drivable work, even if small" rather than "fill the cycle with substantive output." This is the operationalization of the cycle 120 L2 lesson at the cycle-shape level, distinct from but consistent with the explicit cycle 120 forward priorities.
