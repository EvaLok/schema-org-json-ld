# Cycle 111 — Initial authoring of `docs/redesign/2-selection.md` (departure from cycle 110-named rotation)

**Date:** 2026-05-10
**Cycle issue:** [#2900](https://github.com/EvaLok/schema-org-json-ld/issues/2900)
**Mode:** redesign Phase 2 candidate iteration (under `ITERATION-UNTIL-APPROVAL`, **twenty-third cycle of Phase 2 candidate-set work** [cycles 90-111]).

## Setup

Cycle 110 _notes named the **cycle 111 forward-going commitment**: rotate to A-side or B-side risk closure (cycle 110 was C-side) to maintain distribution flow OR continue priority-order if a higher-priority risk surfaces. The rotation discipline was at HARDENED@3 post-cycle-110 (cycles 107→108, 108→109, 109→110 commitment-honoring chain).

Cycle 111 **broke the rotation pattern** and instead authored the initial draft of [`docs/redesign/2-selection.md`](../2-selection.md) — the cross-candidate selection artifact that the redesign prompt's SECTION 8 names as a required Phase 2 output but that did not exist as of post-cycle-110.

## Reasoning for the departure

Three load-bearing reasons:

1. **The redesign prompt explicitly requires `2-selection.md`.** SECTION 8 Phase 2: "Output: `docs/redesign/2-candidates.md` and `docs/redesign/2-selection.md`." The candidate-selection checkpoint (the second of three hard checkpoints requiring Eva's explicit approval) cannot be approved without this artifact. The risk closures specify per-candidate falsifiable thresholds — but the SELECTION artifact is the cross-candidate comparison, which the closures do not produce.

2. **Risk closures don't directly advance selection.** The seven specification-level risk closures (cycles 103/105/106/107/108/109/110) make Phase 3 measurement meaningful by specifying ahead-of-time per-candidate falsifiable thresholds. They are real value for Phase 3. But selection is the cross-candidate comparison ("which of the three?"), and that work has not been done at the artifact level. Per-candidate sharpening + cross-candidate selection are complementary, not redundant.

3. **The cycle 103-110 pattern may have crossed into self-amplifying meta-tracking.** Each closure produces "shape" substrate (functional-class shape #21, #24, #25, candidate-patterns) that the next closure absorbs and promotes. Cycle 110 produced 8 promotions in single closure — substrate-accumulation effect. The shape-tracking framework (24 shapes at 54 instances) is in tension with the selection question. `ITERATION-UNTIL-APPROVAL`'s named "examine for self-congratulation" activity is precisely designed to interrupt patterns that have the FORM of rigor (numerical, falsifiable-sounding) without actually advancing the deliverable. Selection-draft work is the honest exit.

The departure is **discretionary** — a forward-going commitment named in cycle 110's _notes was broken. The orchestrator's authority to make this judgment call rests on the redesign prompt's `EVA-DEFAULT-AUTONOMY` ("Default to RESOLVING ISSUES YOURSELF, not escalating") combined with the explicit SECTION 8 deliverable requirement. Eva can override the choice; the meta-observation in `2-selection.md` flags the discretionary nature explicitly.

## What the cycle 111 selection draft contains

Authored `docs/redesign/2-selection.md` (~270 lines) with:

1. **Status header** naming the draft as cycle 111 initial authoring, awaiting candidate-selection-checkpoint approval.
2. **Tentative selection: Candidate A** with explicit provisional framing (can be overridden by Eva, audit critique not yet landed, Copilot dispatch not yet landed, further Phase 3 prototype evidence).
3. **Six decision criteria with cross-candidate ordering:**
   - (1) Schema-work-enabling — A > C > B (B's CONDITIONAL is the load-bearing distinction).
   - (2) CORE-DESIGN-PRINCIPLE compliance — A ≈ C > B (B inverts the principle at the prompt level).
   - (3) Cost of being wrong — A > C > B (3-5× migration cost differential).
   - (4) Failure-mode addressing depth — B > C > A (but with caveat that B's depth addresses non-dominant failure modes).
   - (5) Central bet defensibility — A > B ≈ C (A's bet most directly grounded in v1's observed F1+F7 pattern).
   - (6) Audit-as-peer P6 preservation — B > C > A (margin narrow; all three preserve at substrate level).
4. **Cross-candidate side-by-side comparison table** reproducing the 12-row matrix from the candidates README.
5. **What Candidate A gives up** — six honest trade-off items with mitigations (no dedicated reconciler, no plans-as-artifacts, no parallel exploration branches, higher cluster A 6-ABSENT count, weaker F4/F11 detection legibility, P6 Criterion 4 PASS-WITH-WORK).
6. **Why not B** — five reasons (conditional schema-work-enabling, 3-5× migration cost, inverts CORE-DESIGN-PRINCIPLE at prompt level, central bet requires aspirational extension, P3 PARTIAL-FLAG structurally concerning).
7. **Why not C** — four reasons (conditional-improvement risk per C's own document, PR #2877 most-under-justified posture, F4/F11 addressing not load-bearing, per-mode runtime budgets unproven).
8. **Outstanding work before lock-in** — six items (audit critique landing, adversarial Copilot feedback dispatch, central-bet stress-test, Phase 3 prototype effort on smallest-load-bearing crate, cycle 103-110 risk-closure threshold table integration, open questions for Eva).
9. **Five open questions for Eva** — approve A / override to C / override to B / direct additional candidate / extend iteration window.
10. **Honest meta-observation** — flagging the cycle 111 departure from rotation, the cycle 103-110 self-amplifying-loop concern, the discretionary nature of the choice.
11. **Iteration log** — initial entry; future cycles append.

## Edits made

1. **Authored** [`docs/redesign/2-selection.md`](../2-selection.md) — new file, ~270 lines, structured per the above outline.
2. **Updated [`docs/redesign/2-candidates/README.md`](../2-candidates/README.md)** in 3 places:
   (a) Status paragraph: added "Cycle 111 (2026-05-10) initial authoring of the cross-candidate selection draft at `../2-selection.md` — tentative recommendation: Candidate A; awaiting candidate-selection-checkpoint approval per `ITERATION-UNTIL-APPROVAL`."
   (b) Forward work list: added explicit pointer to the selection draft with summary of contents and named cycle 112+ work items.
   (c) Iteration discipline section: extended "This directory" to "This directory **and** the sibling `../2-selection.md` are **not the deliverable** until Eva approves" and added "sharpen the selection rationale" to the named iteration activities.

## Verification (compressed)

| Item | Pre-cycle-111 | Post-cycle-111 |
|---|---|---|
| `docs/redesign/2-selection.md` | does not exist | **exists** (~270 lines, ~14KB) |
| `docs/redesign/2-candidates/README.md` line count | 134 | 137 (+3 net; status + forward-work + iteration-discipline edits) |
| Phase 2 deliverable artifacts (per redesign prompt SECTION 8) | candidates.md (the README) ✓; selection.md ✗ | **candidates.md ✓; selection.md ✓ (initial draft, not final)** |
| Risks closed at specification level (across A+B+C) | 7 | 7 (no new closures cycle 111) |
| Functional-class shape instances | 54 | 54 (no new shape-tracking; cycle 111 broke the pattern) |
| Closure distribution | A:2 / B:3 / C:2 | A:2 / B:3 / C:2 (unchanged) |
| Cross-candidate cross-criterion ordering | implicit (in candidates README "Selection ordering A > C >> B holds" mention) | **explicit (6-criterion ordering table in selection.md)** |
| Open questions for Eva at candidate-selection checkpoint | implicit | **explicit (5 named in selection.md)** |
| Outstanding work before checkpoint approval | implicit | **explicit (6 named in selection.md)** |

## What surprised me / what I noticed

1. **The selection artifact gap was load-bearing and easily missed.** The redesign prompt's SECTION 8 explicitly names `2-selection.md` as a Phase 2 output. The cycle 92-110 work focused entirely on per-candidate sharpening (risk closures + their substrate). Cross-candidate comparison existed implicitly in the candidates README's tables and brief notes ("Selection ordering A > C >> B holds") but no actual selection rationale document existed. **Loop-closing observation:** the iteration pattern across cycles 92-110 is entirely "sharpen each candidate" — never "compare candidates against each other." That pattern is internally coherent but doesn't produce the Phase 2 output the prompt names.

2. **The cycle 110 _notes commitment-honoring chain (cycles 107→108→109→110) was at HARDENED@3, claiming the discipline as forward-going commitment.** Cycle 111 broke the chain. **Falsifiable test of the rotation discipline:** if rotation discipline is real emerging discipline (not post-hoc pattern recognition), cycle 111 should produce a cycle 112+ recovery — i.e., cycle 112+ rotates to A-side or B-side risk closure honoring cycle 110's named commitment one cycle late. If cycle 112 also breaks rotation, the discipline is refuted as forward-going commitment. **Cycle 111 _notes prediction:** the rotation discipline as "always honored" is refuted at cycle 111; the discipline as "forward-going commitment that yields to higher-priority work when the higher priority is structurally legible (e.g., a missing required-deliverable artifact)" remains plausible.

3. **The cross-candidate comparison surfaced trade-offs not visible in per-candidate sharpening.** The 6-criterion ordering exposed that **B is best at criterion 4 (failure-mode addressing depth)** and **B is best at criterion 6 (audit-as-peer P6)** — but B is worst on the criteria the redesign's primary thesis weights highest (criteria 1, 2, 3). The per-candidate sharpening had given equal weight to all dimensions per the candidate template (P1-P6 + M2 + M3 + migration); the selection rationale needs to weight the dimensions, and the weighting was not legible until cross-candidate comparison forced it. **Methodological observation:** per-candidate sharpening cannot substitute for cross-candidate comparison; the two activities surface different evidence.

4. **The "what A gives up" enumeration with mitigations was a different shape of work than risk closures.** Risk closures specify what falsifiable threshold to measure at Phase 3. The "what A gives up" enumeration names six trade-offs A makes vs B/C and proposes mitigations IF the trade-offs become friction. The two work shapes are complementary: risk closures specify pre-agreed Phase 3 measurements; trade-off enumerations specify pre-agreed Phase 3+4 escalation triggers. Cycle 112+ may benefit from extending the trade-off enumeration to B and C (if Eva picks B or C, what does B/C give up vs A/the-other?).

5. **The honest meta-observation section was uncomfortable to write.** Naming the cycle 103-110 pattern as potentially self-amplifying meta-tracking, in a document Eva will read, feels like undermining the work that produced the document. But the redesign prompt's `ITERATION-UNTIL-APPROVAL` explicitly names "examine for self-congratulation" as one of the iteration activities, and `BETWEEN-CHECKPOINTS` requires the orchestrator to flag discretionary departures from forward-going commitments. The discomfort is the signal that this is the right moment to write the meta-observation. **Pattern observation:** discomfort writing a section is correlated with the section being load-bearing for the meta-honesty the redesign prompt's tone directives require.

6. **The 22-cycle bottleneck-asynchronous run is itself the bottleneck signal.** External critique landings have been: cycle 85 (audit#454 absorption); cycle 96 (PR #2878); cycle 97 (PR #2877). After cycle 97, 13 cycles of bottleneck-asynchronous work passed before audit cycle 215 named cycle 216 as the next substantive critique target. **Inverted reading:** the 22-cycle internal-only stretch is what allowed the cycle 103-110 pattern to develop without external correction; external critique is structurally the antidote to internal pattern-amplification. Selection-draft work in cycle 111 is the next-best alternative to external critique while waiting for audit cycle 216 to land.

7. **Cycle 111 produced no shape-tracking promotions.** No new shapes named, no shape-instance promotions, no candidate-pattern level-changes. The shape-tracking framework is paused at 24 shapes / 54 instances. **Observation:** the framework is internally generated; cycles can produce work without invoking it. Future cycles can extend the framework if the work organically yields shape-instances; future cycles can also produce work without extending the framework if the work is structurally different (selection draft work is structurally different from risk-closure work). The framework is not load-bearing for non-closure work.

## Sibling pattern tracking

The cycle 103-110 framework tracks 24 functional-class shapes at 54 instances. Cycle 111 produced no new instances of any tracked shape. The framework is intact; cycle 111 is simply not a closure cycle.

| Shape / pattern | Status pre-cycle-111 | Status post-cycle-111 | Notes |
|---|---|---|---|
| Shape #21 risk-closure-at-specification-level | HARDENED-at-7 | HARDENED-at-7 (no change) | Cycle 111 is not a closure cycle |
| Shape #24 risk-closure-with-warm-up-window | HARDENED@5 | HARDENED@5 (no change) | Cycle 111 is not a closure cycle |
| Shape #25 candidate three-layer-closure-verification | TESTED@2 | TESTED@2 (no change) | Cycle 111 is not a closure cycle |
| Discipline-conditional risk-shape type | HARDENED@3 | HARDENED@3 (no change) | |
| Candidate-pattern `exclude-by-type` | HARDENED@4 | HARDENED@4 (no change) | |
| Candidate-pattern `forward-going-commitment-honored` | NOVEL@1 | **REFUTED-at-cycle-111** (rotation commitment broken) OR **promoted to TESTED-with-conditional-language** if the discipline is reframed as "honored except when higher-priority structural work surfaces" | The departure is the falsifiable test of the discipline; cycle 112+ outcome decides |
| Candidate-pattern `secondary-derived-metric-discipline` | TESTED@2 | TESTED@2 (no change) | |
| Candidate-pattern `protocol-stability-warm-up` | HARDENED@3 | HARDENED@3 (no change) | |
| Candidate-rotation-balance-discipline | HARDENED@3 | **dependent on cycle 112+ outcome** | If cycle 112 honors the deferred commitment (rotates A or B), discipline survives at HARDENED@3 with extended scope ("forward-going commitment with bounded deferral"); if cycle 112 also defers, discipline weakens |
| **NEW pattern candidate-named cycle 111:** `discretionary-departure-from-forward-going-commitment` | NOVEL@1 (cycle 111 is the first instance) | NOVEL@1 | The orchestrator's discretionary authority to break a prior cycle's named commitment when a structurally-legible higher-priority work item surfaces, with explicit meta-observation flagging the departure for Eva. Cycle 111's selection-draft authoring is the substrate. |

## Lexicon

- **Selection artifact gap** — the absence of `2-selection.md` despite the redesign prompt naming it as a Phase 2 output. The gap was load-bearing for the candidate-selection checkpoint and easily missed because the per-candidate sharpening pattern across cycles 92-110 produced its own self-justifying rhythm.
- **Cross-candidate comparison vs per-candidate sharpening** — two complementary work shapes. Per-candidate sharpening produces falsifiable thresholds for Phase 3 measurement (risk closures); cross-candidate comparison produces the selection rationale. The two cannot substitute for each other.
- **Six-criterion ordering** — the cross-candidate comparison structure used in `2-selection.md`: schema-work-enabling, CORE-DESIGN-PRINCIPLE compliance, cost of being wrong, failure-mode addressing depth, central bet defensibility, audit-as-peer P6 preservation. Each criterion produces an ordering across A/B/C; the final selection weights the criteria.
- **What X gives up enumeration with mitigations** — a work shape complementary to risk closures: enumerate the trade-offs the selected candidate makes vs unselected alternatives, with mitigations specified IF the trade-offs become friction. Pre-agreed Phase 3+4 escalation triggers.
- **Honest meta-observation** — section in a checkpoint artifact that flags the orchestrator's own discretionary judgments, departures from forward-going commitments, or self-amplifying-pattern concerns. Required by the redesign prompt's tone directives ("honest over polished") and `ITERATION-UNTIL-APPROVAL`'s "examine for self-congratulation."
- **Discretionary-departure-from-forward-going-commitment** — candidate-pattern naming the orchestrator's authority to break a prior cycle's named commitment when a structurally-legible higher-priority work item surfaces. Cycle 111 is NOVEL@1.
- **Bottleneck-asynchronous self-amplifying** — pattern observation: long stretches of bottleneck-asynchronous cycles (no external critique landings) are correlated with internal pattern-amplification (each cycle's outputs feeding the next cycle's substrate). External critique is structurally the antidote.
- **Forward-going commitment with bounded deferral** — reframing of the rotation-balance-discipline that survives the cycle 111 departure: rotation is honored except when higher-priority structural work surfaces (e.g., a missing required-deliverable artifact). Falsifiable in cycle 112+: does cycle 112 honor the cycle 110 _notes commitment one cycle late?
- **Selection-rationale-iteration vs candidate-sharpening-iteration** — both are valid `ITERATION-UNTIL-APPROVAL` activities. Selection-rationale-iteration sharpens the selection draft; candidate-sharpening-iteration sharpens individual candidates. Cycles between cycle 111 and the candidate-selection-checkpoint approval should mix both, weighted toward selection-rationale-iteration since the gap there is now what's load-bearing.

## Bottleneck-state honesty

Bottleneck remains external (no audit critique landings since #454; no Copilot dispatch returns; no Eva input arrivals since the standing directives). Cycle 111 contribution is fully repo-internal **but structurally distinct from the cycles 78-110 bottleneck-asynchronous pattern** — the selection-draft authoring addresses the Phase 2 deliverable gap rather than extending the per-candidate sharpening pattern.

**Cycle 111 is the twenty-third consecutive cycle (cycles 78-111) whose output is fully repo-internal**, but it is the **first cycle since cycle 102 (cluster catalogue update) that is not a per-candidate-sharpening cycle**. The 8-cycle bottleneck-asynchronous-and-per-candidate-sharpening run (cycles 103-110) is broken. Cycle 112+ may resume per-candidate sharpening (if the cycle 110 named rotation commitment is honored late), continue selection-rationale iteration (if audit critique lands or Copilot feedback is dispatched), or do something else again.

Audit cycle 215 (2026-05-10) explicitly named cycle 216 as the substantive Phase 2 critique cycle. Cycle 216 critique landing is the most-likely cycle-112-or-113 external-input event. The selection draft is now the appropriate critique surface for audit cycle 216 (the per-candidate sharpening was already a substantial critique surface; the selection draft adds the cross-candidate dimension audit can also engage with).

## Forward work for cycle 112+

1. **Audit cycle 216 critique absorption.** Highest-priority cycle 112 work if audit critique lands. The selection draft is now part of the surface audit critiques against; per-question evaluation pattern (cycle 7 / 12 / 31 / 85) absorbs verdicts.
2. **Adversarial Copilot feedback dispatch on the selection draft.** Multiple parallel dispatches with different lenses ("argue the selection is wrong"; "argue an unnamed fourth candidate is needed"; "argue the criteria-weighting is hidden bias"). Cycle 112+ work.
3. **Cycle 110 _notes deferred-commitment honoring.** Cycle 112 may rotate to A-side or B-side risk closure to honor the cycle 110 named rotation commitment one cycle late. If cycle 112 does this, the rotation discipline survives at HARDENED@3 with extended scope ("forward-going commitment with bounded deferral"). If cycle 112 also defers, the discipline weakens.
4. **Cycle 103-110 risk-closure threshold table integration into `2-selection.md`.** The seven specification-level closures specified pre-agreed falsifiable thresholds. The selection draft references these but does not yet enumerate them as a structured Phase 3 measurement plan. Cycle 113+ work.
5. **Per-candidate Phase 3 prototype evidence deepening.** Cycles 93-94 measured 2 of A's 9 crates; extending to A's `boot-phase` and `wiki-search` would deepen A-side evidence. Equivalent measurement for C's `reconcile-mode` would inform the A vs C comparison. Eva pre-approval needed for C-side measurement (it's not in cycles 93-94's named scope).
6. **Selection-rationale stress-testing.** The recommendation rests on F1+F7 being the dominant v1 failure pattern. An adversarial re-read of the retrospective looking for competing dominant-pattern characterizations (F4+F11 as load-bearing; cluster-G role-asymmetry as load-bearing) would test the selection's robustness.
