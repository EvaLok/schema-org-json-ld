---
cycle: 135
date: 2026-05-13
status: substantive
absorbs: cycle 132 6-of-9 measurement (priority #5 from cycle 134 forward priority list)
prior-deferrals: cycle 132 (origination) → cycle 133 (deferred, housekeeping focal) → cycle 134 (deferred, audit#465 M1 absorption focal) → cycle 135 (closed)
artifact-edits:
  - docs/redesign/2-selection-summary.md (cycle 135 propagation paragraph after cycle 134 M1 paragraph; cost-of-being-wrong table rows (a) and (b) 6-of-9 updates; narrative below table 6-of-9 update)
  - docs/redesign/2-candidates/README.md (Migration LOC calibrated row title and content; ‡‡ footnote restructured for 6-of-9 with shape-family-conditional band; Most-discriminating-criteria Migration cost bullet 6-of-9 update with PR #2877 envelope-qualified narrative)
preserves:
  - cycle 120 L2 constraint (no recursive annotation of 2-selection.md; cycle 135 touches only summary + README + footnote surfaces, not the deep selection rationale)
  - cycle 130 D4 absorption paragraph (frozen as 5-of-9 reading; cycle 135 paragraph added as sibling propagation rather than overwriting)
  - cycle 134 M1 absorption paragraph (preserved; cycle 135 paragraph follows chronologically)
  - Q7 three options a/b/c (unchanged; cycle 135 propagates evidence without altering the Q7 question)
  - Selection ordering A > C >> B (preserved; scale-level asymmetry bounded weeks vs months unchanged)
  - Cost-of-being-wrong table structure (anchor-type column from cycle 130 D4 preserved)
---

# Cycle 135 — cycle 132 6-of-9 measurement propagation to Eva-facing deliverable

**Date:** 2026-05-13
**Cycle issue:** [#2927](https://github.com/EvaLok/schema-org-json-ld/issues/2927)
**Model:** claude-opus-4-7
**Mode:** redesign Phase 2 candidate iteration (under [`ITERATION-UNTIL-APPROVAL`](../../../.github/workflows/orchestrator-redesign-prompt.xml), **forty-sixth cycle of Phase 2 candidate-set work** [cycles 90-135])

**Substantive focal:** cycle 134 forward priority **#5 — 2-selection-summary calibrated-empirical migration cost row update.** Cycle 132's 6-of-9 measurement (4483 sum / 747 mean / std dev 485 / 9-crate extrapolation 6726) was absorbed into A's candidate doc cycle 132 but not propagated to the Eva-facing deliverable surfaces (`2-selection-summary.md` + `2-candidates/README.md`) until cycle 135. Priority #5 has been deferred 3 cycles since cycle 132 (cycle 133 housekeeping focal; cycle 134 audit#465 M1 absorption focal).

**Twenty-first consecutive cycle of HONORING named forward priority** (cycles 115-135).

## Setup

Cycle 134 forward priorities (per [`cycle-134-audit-465-M1-absorption.md`](./cycle-134-audit-465-M1-absorption.md) line 233-244):

1. Q7 by Eva — Eva-blocked at cycle 135 entry (24 cycles of substrate)
2. audit#462 M2 + M3 + M5 + P3-1 through P3-4 + P3-7 + P3-8 carry — 7 substantive findings under cycle 130 Phase 3 framing (cycle 218 did not sharpen these specifically)
3. `v2-close-phase` SCAFFOLD — 4th scaffold→complete delta primitive (deferred cycles 131-134)
4. `v2-phase-transition-check` pre-existing test failures triage (carried)
5. **2-selection-summary calibrated-empirical migration cost row update — cycle 132's measurement materially shifts the empirical anchor**
6. Symphony deeper-read elevation + oh-my-claudecode deeper-read elevation — Phase 1 research forward (still carried)
7. NO further recursive annotation of 2-selection.md — cycle 120 L2 constraint continues

Priority #1 not available (Eva-blocked). Priority #2 carries unless audit sharpens — premature absorption would risk repeating cycle 130 framing conflation (which cycle 134 absorbed via audit#465 M1 disambiguation). Priorities #3 and #5 are both actionable. Cycle 135 selected priority #5 over #3 because:

- #5 is bounded text edits to Eva-facing deliverable surfaces (small scope, clear closure)
- #5 has been deferred 3 cycles since cycle 132 measurement landed
- #3 is a larger SCAFFOLD task that warrants dedicated cycle scope (cycle 136+)
- Per `ITERATION-UNTIL-APPROVAL` "sharpen the analysis" activity, propagating empirical findings to the Eva-facing deliverable IS sharpening at the load-bearing-claim level

## What changed in the measurement

Cycle 132's `v2-gardening-sweep` COMPLETE promoted the A-shared crate measurement program from 5-of-9 to 6-of-9 instances. The 6-of-9 update:

| Metric | 5-of-9 (cycle 130 anchor) | 6-of-9 (cycle 132 anchor) | Δ |
|---|---|---|---|
| Sum prod LOC | 2951 | 4483 | +1532 (+52%) |
| Mean prod LOC | ~590 | ~747 | +157 (+26%) |
| Sample std dev | ~296 | ~485 | +189 (+64%, variance widening) |
| 9-crate flat-mean extrapolation | ~5310 | ~6726 | +1416 (+27%) |
| PR #2877 envelope at 9-crate extrapolation | WITHIN 3600-6200 (~79% through) | **EXCEEDS 6200 upper by +8%** | first material judgment-vs-empirical overshoot |
| A's stated 4500 ceiling distance | +18% above | **+49% above** | ceiling further refuted |
| Test:prod cumulative ratio | 1.22× (5-crate) | 0.98× (6-crate) | gardening-sweep COMPLETE 0.53× pulls cumulative down |

The 6-of-9 reading is the **first time an empirical anchor reading for A has materially exceeded PR #2877's lens-4 judgment range**.

## What this means for the cycle 130 D4 absorption framing

Cycle 130 D4 absorption named two candidate-emergent observations:

1. **`calibration-extends-by-association`** — PR #2877's verified workspace calibration credibility extended to lens-4 ranges despite those being judgment-based (the lens-4 caveat from cycle 97 absorption did not propagate to the Eva-facing deliverable until cycle 130 D4 surfaced it)
2. **`judgment-range-can-envelope-empirical-truth`** — PR #2877's judgment-based A 3600-6200 range correctly enveloped cycle 127's empirical central estimate 5310 at ~79% through; well-calibrated judgment produced envelope-accurate ranges

**Cycle 135 qualifies the second observation at 6-of-9**:

- The envelope property held at 5-of-9 (5310 within 3600-6200, ~79% through)
- The envelope property does NOT hold at 6-of-9 (6726 vs 6200, +8% above upper)
- The qualification is NOT a refutation of well-calibrated judgment — PR #2877's range remains a useful planning band — but it IS evidence that empirical anchors strengthen over judgment ranges as more measurements accrete

This qualification is meta-evidence for the redesign's CORE-DESIGN-PRINCIPLE bet: empirical anchors compound; judgment ranges are bounded.

The first observation (`calibration-extends-by-association`) is unaffected by cycle 135 — cycle 130's mechanism (verified-workspace-calibration credibility extended to lens-4 by association) was about evidence-type-conflation, not range accuracy. The lens-4 ranges remain useful judgment-based bands; their judgment character is what cycle 130 surfaced.

## What this means for selection ordering

**Selection ordering A > C >> B is preserved across cycle 135 propagation.**

The cost-of-being-wrong dominant asymmetry is between A/C (bounded weeks) and B (months). At cycle 135's central estimates:

- A: ~6726 (6-of-9 empirical anchor)
- C: ~6726-9726 (A-shared empirical + ~0-3000 LOC C-specific delta judgment)
- B: ~21000 (PR #2877 14000-28000 midpoint; no empirical anchor)

A-vs-B ratios at central estimates:
- Cycle 130 (5-of-9): 21000/5310 = ~3.95×
- Cycle 135 (6-of-9): 21000/6726 = ~3.12×

The ratio compresses by ~21% but the scale-level asymmetry — bounded weeks (A/C cutover scope per `2-candidates/README.md` Migration cutover-scope row) vs months (B's 16-30 cycle build-out per PR #2877 lens-5) — is unchanged. Selection ordering A > C >> B holds because:

- The A-vs-C narrowing (A-shared empirical anchor common to A and C; only C-specific delta differentiates) is preserved per cycle 130 framing
- The A/C-vs-B scale-level asymmetry is preserved (B's months-scale recovery cost is decoupled from the LOC numbers since cluster G role-asymmetric context bet under B is unmeasured under V2; refutation cost is in calendar-time and design-rework scope, not just LOC)
- The cost-of-being-wrong asymmetry per Criterion 3 framing operates at scale-level, not at narrow LOC-ratio level

## What this does NOT change

**Q7's three options (a/b/c) are structurally unchanged.** Cycle 135 propagates empirical evidence into the Eva-facing deliverable without altering the Q7 question. Eva owns Q7's resolution; cycle 135 augments the evidence base.

**The cost-of-being-wrong table structure is preserved.** Cycle 130 D4's anchor-type column (mixed / empirically anchored / judgment-only) carries across cycle 135; the row entries are updated with 6-of-9 numbers but the column structure is unchanged.

**A's framework grounding is preserved.** Cycle 134 V2-era operational failure-mode evidence (classifier-class + state-growth-axis families) is preserved; cycle 135 operates on a different dimension (migration LOC empirical anchor) and the dimensions are independent.

**Cycle 130 D4 absorption paragraph is frozen.** Cycle 135 adds a sibling propagation paragraph after cycle 134 M1 absorption paragraph rather than overwriting cycle 130's 5-of-9 reading. This preserves the chronological iteration record and matches the cycle 134 paragraph-addition precedent.

**No new architecture decisions.** Phase 3 design carries (M4 + P3-1 through P3-4 + P3-7 + P3-8 per cycle 130 framing) are unchanged; cycle 135 is Phase 2 evidence propagation.

## Anti-overstatement audit

Cycle 135's propagation is substantive at the Eva-facing deliverable surface but bounded in scope. Anti-overstatement audit:

**Does NOT claim**:
- Cycle 135 resolves Q7 — Eva owns Q7's resolution
- Cycle 135 changes selection ordering — A > C >> B preserved
- Cycle 135 makes B less viable structurally — cost-of-being-wrong asymmetry preserved
- Cycle 135 refutes PR #2877's lens-4 calibration discipline — PR #2877's range remains a useful planning band; the +8% over upper is a meaningful qualification, not a discreditation
- The `judgment-range-can-envelope-empirical-truth` observation is broken — it held at 5-of-9; at 6-of-9 it is qualified (envelope property held at one anchor count, does not hold at next)
- The new central estimate is a precise prediction — sample std dev 485 LOC against mean 747 is high; 9-crate band 5638-7301 (shape-family-conditional) reflects substantial uncertainty
- The 6-of-9 shift was unpredicted — cycle 131 explicitly predicted +300-600 LOC delta with ±10% magnitude band; cycle 132's +918 delta was +39% over the upper bound; this was cycle 132's `magnitude-prediction-precision-is-shape-dependent-not-flat` candidate-emergent observation

**Does claim**:
- A's empirical central estimate has shifted ~5310 → ~6726 (+27%) at 6-of-9
- The empirical anchor has exceeded PR #2877's 6200 upper bound for the first time
- The variance has widened (std dev +64%) reflecting shape-family heterogeneity
- The cycle 130 D4 candidate-emergent observation `judgment-range-can-envelope-empirical-truth` is qualified at 6-of-9 (envelope held at 5-of-9, does not hold at 6-of-9)
- Selection ordering is preserved
- Q7 resolution is unchanged
- The Eva-facing deliverable surfaces (selection-summary + 2-candidates README) now match A's candidate doc on the empirical anchor

**Honest characterization of evidence quality**:
- 6-of-9 sum 4483: verifiable from A-evolved-single-orchestrator.md line 89
- 6-of-9 mean 747 and 9-crate extrapolation 6726: verifiable from cycle 132 _notes "Empirical finding (cycle 132 COMPLETE LOC)" section and from A's candidate doc line 89
- 6-of-9 std dev 485: documented in cycle 132 _notes "Cumulative measurement at 6 crates" section
- Shape-family band 5638-7301: documented in cycle 132 _notes "Shape-family-conditional refinement" subsection (lines reading 1085 + 578 means)
- A-vs-B ratio shift 3.95× → 3.12×: arithmetic on PR #2877 B midpoint 21000 divided by A central estimate at each anchor count
- The 6-of-9 reading is currently the highest-evidence anchor for A; 3-of-9 (close-phase / detect-abandoned-cycles / prompt-contract-check) remains unmeasured; cycle 136+ close-phase SCAFFOLD-then-COMPLETE will produce the 7-of-9 anchor

## Why this propagation pattern

Cycle 132's measurement landed in A's candidate doc within the same cycle as the measurement itself (cycle 132 self-contained). The propagation to Eva-facing surfaces was deferred because:

1. Cycle 133 chose `v2-gardening-sweep` housekeeping run as substantive focal (build→complete→act loop demonstration)
2. Cycle 134 chose audit#465 M1 absorption as substantive focal (cycle 218 audit critique landed)
3. Cycle 135 is the first cycle where priority #5 is the highest-ranked actionable priority (1 Eva-blocked, 2 audit-blocked-pending-sharpening, 3 larger-scope deferred)

The 3-cycle deferral did not cause a load-bearing claim to be wrong in the deliverable — Eva-facing surfaces showed "5-of-9 ~5310" which is still empirically true (it was the 5-of-9 reading) but became stale relative to the latest evidence. Cycle 135 closes the staleness.

The pattern shape:

1. **Selection-summary cycle 135 propagation paragraph** — Eva-legible digest of what cycle 132's measurement changed; cross-references cycle 130 D4 (frozen) and cycle 132 measurement note
2. **Cost-of-being-wrong table row updates** (rows a + b) — propagate 6-of-9 numbers + supersedence note + +8% over PR #2877 upper bound
3. **Narrative below table** — propagate 6-of-9 anchor reference
4. **2-candidates/README.md Migration LOC calibrated row** — change row title "cycle 130 calibrated" → "cycle 135 calibrated" + 6-of-9 numbers
5. **‡‡ footnote restructured** — preserve cycle 130 D4 attribution; add cycle 135 propagation note; expand with shape-family-conditional band derivation; explicitly note PR #2877 envelope qualification
6. **Most-discriminating-criteria Migration cost bullet** — propagate 6-of-9 numbers; preserve "A and C closer in risk than docs suggest" + "C is most likely to miss its own upper-bound estimate" framing (cycle 135 strengthens both by surfacing empirical evidence)

This is a propagation pattern, not an absorption pattern: the substantive measurement was cycle 132's; cycle 135 surfaces it to Eva-facing legibility. Analogous to cycle 130 D4 surfacing the lens-4-vs-workspace evidence-type distinction that had been in the corpus since cycle 97 absorption.

## Patterns

**`discretionary-departure-from-forward-going-commitment`** (HARDENED-at-4 cycle 114) → **21 honorings cycle 135** (cycles 115-135; 25 cycles of substrate including pre-Phase-2 cycles 111-114). Cycle 135 honors cycle 134's priority #5 in its named order.

**`judgment-range-can-envelope-empirical-truth`** (NOVEL@1 cycle 130 candidate-emergent observation) → **QUALIFIED@2 cycle 135**. The observation held at 5-of-9 (cycle 130 confirmation); the observation does not hold at 6-of-9 (cycle 132 measurement, propagated cycle 135). The qualification is meta-observation: well-calibrated judgment ranges can envelope at lower anchor counts but get exceeded as empirical anchors accrete (consistent with the redesign's CORE-DESIGN-PRINCIPLE bet that empirical anchors compound vs judgment ranges).

**`calibration-extends-by-association`** (NOVEL@1 cycle 130 candidate-emergent observation) → **unchanged cycle 135** — cycle 135 is about a different dimension (range accuracy at 6-of-9) than the cycle 130 observation (evidence-type-conflation).

**`magnitude-prediction-precision-is-shape-dependent-not-flat`** (NOVEL@1 cycle 132 candidate-emergent observation) → **REINFORCED cycle 135** — the variance widening (std dev +64% from 5-of-9 to 6-of-9) is the empirical mechanism behind the observation; cycle 135 surfaces this mechanism in the Eva-facing deliverable's ‡‡ footnote (5638-7301 shape-family-conditional band).

**NEW candidate-emergent observation cycle 135: `empirical-anchor-strengthens-over-judgment-range-over-time`**. PR #2877's lens-4 A 3600-6200 judgment-based range correctly enveloped the empirical central estimate at 2-of-9 (cycle 94) and at 5-of-9 (cycle 127); at 6-of-9 (cycle 132) the empirical extrapolation exceeded the upper bound. This is sibling to cycle 130's `judgment-range-can-envelope-empirical-truth` at temporal-evolution scope (held at lower anchor counts; exceeded as anchor count grows). NOT YET PROMOTED to candidate pattern — second substrate instance (e.g., C-specific delta measurement exceeding C 5200-8200 PR #2877 range) would test.

## Forward work for cycle 136+

Cycle 134's forward priorities with priority #5 closed at cycle 135:

1. **Q7 resolution by Eva** — carried (Eva-blocked since cycle 120; 24 cycles of substrate)
2. **Audit#462 M2 + M3 + M5 + P3-1 through P3-4 + P3-7 + P3-8 carry** — 7 substantive findings under cycle 130 Phase 3 framing
3. **`v2-close-phase` SCAFFOLD** — 4th scaffold→complete delta primitive (carried; was cycle 134 priority #3 → cycle 135 priority #3 → cycle 136 priority #3 if no audit lands)
4. **`v2-phase-transition-check` pre-existing test failures triage** (carried)
5. **Symphony deeper-read elevation + oh-my-claudecode deeper-read elevation** — Phase 1 research forward (carried)
6. **NO further recursive annotation of 2-selection.md** — cycle 120 L2 constraint continues

Cycle 135's priority #5 closure renumbers the remaining items (originally 6 → 5; 7 → 6). The natural cycle 136 substantive focal is priority #3 (`v2-close-phase` SCAFFOLD), assuming no audit cycle 219 lands with sharpening that creates higher-priority absorption work.

Audit cycle 219 expected ~04:00 UTC 2026-05-14 (~21h post cycle 135 session-start). Per cycle 134 framing, cycle 218 [audit#465] M1 follow-up does not predict a cycle 219 follow-up on M2-M5 or P3-* — audit cycle 218 explicitly named cycle 219 watch items rather than commitments. If audit cycle 219 lands and is substantive, cycle 136 may pivot per `audit-as-Priority-1-input` pattern (HARDENED@5 cycle 134); if cycle 219 is bounded, cycle 136 pursues `v2-close-phase` SCAFFOLD as named priority #3.

## Process honoring

- **Twenty-first consecutive cycle of HONORING named forward priority** (cycles 115-135).
- **47th consecutive bottleneck-asynchronous cycle** (cycles 78-135).
- **25th consecutive non-per-candidate-sharpening cycle** (cycles 111-135) — cycle 135 is propagation of an empirical anchor to candidate-set-evaluation surfaces, not per-candidate sharpening.
- Cycle 120 L2 constraint preserved (no recursive annotation of `2-selection.md`).
- Cycle 128 process-error lesson preserved (`.scratch/` used inside repo for session-start comment body; no parallel-batch cancellation cascade).
- Cycle 133 process-error lesson preserved (no `cd` to non-repo paths in parallel batches; cwd remained in repo root throughout).
- Cycle 134 process-error: parallel batch including `rm -rf /tmp/audit-check` triggered sandbox refusal (the `rm` outside repo root was blocked); recovered by re-issuing safer `git clone` (target inside `/tmp` is permitted via clone destination but cleanup via `rm` is blocked — design implication: prefer using a different fresh destination per cycle, e.g., `/tmp/audit-fresh-${cycle}`, OR use git clean operations inside the cloned directory).

## References

- [`cycle-132-gardening-sweep-complete-third-arc-closes.md`](./cycle-132-gardening-sweep-complete-third-arc-closes.md) — origin of the 6-of-9 measurement
- [`cycle-130-audit-462-D4-absorption.md`](./cycle-130-audit-462-D4-absorption.md) — origin of the cycle 130 D4 absorption + `judgment-range-can-envelope-empirical-truth` observation (frozen at 5-of-9 reading; qualified by cycle 135 at 6-of-9)
- [`cycle-134-audit-465-M1-absorption.md`](./cycle-134-audit-465-M1-absorption.md) — predecessor cycle's forward priorities (priority #5 closed cycle 135)
- [`../2-selection-summary.md`](../2-selection-summary.md) — Eva-facing deliverable surface (updated cycle 135)
- [`../2-candidates/README.md`](../2-candidates/README.md) — Eva-facing candidate comparison (updated cycle 135)
- [`../2-candidates/A-evolved-single-orchestrator.md`](../2-candidates/A-evolved-single-orchestrator.md) line 89 — measurement-evidence section already documenting 6-of-9 from cycle 132
