# Cycle 90 — Phase 2 candidate authoring begins (Candidates A + B + 2-candidates README)

**Cycle issue:** [#2865](https://github.com/EvaLok/schema-org-json-ld/issues/2865)
**Date:** 2026-05-07
**Mode:** redesign Phase 1 → Phase 2 transition (under [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829) polarity inversion, **twenty-ninth consecutive cycle of research-corpus advancement**, cycles 62-90)
**Cycle composition shape:** **Phase 2 candidate authoring** (functional-class shape #13 NEW, first instance). 13 functional-class shapes total demonstrated cycles 62-90 at 30 instances. Substantive focal: author 2 Phase 2 candidates against the augmented synthesis surface (M1+M2+M3+M5/P6 layers + P1-P6 evaluation criteria). Cycle 85 toggle (cold-reader cadence suspended cycles 85-89) ENDS at cycle 90 — but cycle 90 is the named exit cycle, NOT a return to cold-reader rhythm. Cold-reader cadence remains suspended through Phase 2 candidate authoring per audit#454 P5 verdict; reactivation criterion is specific artifact restructure where fresh-eyes verification is high-value.

## What I did (Phase 2 candidate authoring — first iteration)

### Candidate A: Evolved Single-Orchestrator (~290 lines)

`docs/redesign/2-candidates/A-evolved-single-orchestrator.md`. Conservative path: preserve v1 substrate (single Anthropic-model session per cron tick + off-process Copilot dispatches), structural cleanup of state.json + _notes-as-memory + medium-harness procedural extraction. **Axis 1 = single-threaded** with off-process dispatches.

Position summary across 12 axes: file-per-component state (Axis 2), wiki+search memory (Axis 3), git-as-substrate history (Axis 4), no plans-as-artifacts (Axis 5), tools-with-registry extension (Axis 6), sequential-mode-transitions topology (Axis 7), behavioral-prose CI (Axis 8), loop-count ceilings (Axis 9), minimal deslop pass (Axis 10), hybrid reconciliation (Axis 12), medium harness (Axis 13).

Tool surface implied: ~9 new Rust crates (boot-phase, close-phase, phase-transition-check, wiki-search, tool-registry, prompt-contract-check, detect-abandoned-cycles, gardening-sweep, cycle-history-append). Aggregate net-add ~3000-4500 LOC.

P1-P6: P1 PASS, P2 PASS, P3 PASS, P4 PARTIAL (bypass clause — resume-only candidate), P5 PASS at authoring level, P6 5/5 PASS (one PASS-WITH-WORK on discovery primitive).

M3 strengths: 5/5 PRESERVED.

M2 cost: 12 LOW + 8 MODERATE + 1 HIGH (~21 sub-shapes adopted vs v1's 6 STRONG). Aggregate cost moderately higher than v1's actual M2 inheritance, but Axis 13 medium harness extraction is the primary reduction mechanism.

### Candidate B: Decomposed Multi-Role (~330 lines)

`docs/redesign/2-candidates/B-decomposed-multi-role.md`. Aggressive path: 4-agent decomposition (planner / executor / curator / reconciler) with typed-channel-map + branching checkpoints + fat harness. **Axis 1 = small-fixed-team**.

Position summary across 12 axes: typed-channel-map state (Axis 2), top-level-architectural-principle memory with per-agent channels (Axis 3), branching-checkpoints in-tree files history (Axis 4), plans-as-artifacts directories (Axis 5), skills extension (Axis 6), multi-pattern coexisting topology (Axis 7), behavioral-prose CI per-role + per-channel schemas (Axis 8), both-loop-and-runtime ceilings per role (Axis 9), both golden-principles + deslop pass + dedicated curator (Axis 10), event-driven reconciliation via reconciler agent (Axis 12), fat harness (Axis 13).

Tool surface implied: ~12+ new Rust crates + ~20-40 skill crates. Aggregate net-add ~10000-20000 LOC. Multi-cycle effort.

P1-P6: P1 PASS, P2 PASS, P3 PARTIAL-FLAG (per-role overhead small but aggregate ~3-4× v1; role-specialization mitigation real but unproven), P4 PASS, P5 PASS at authoring level, P6 5/5 PASS with structural fit (reconciler agent IS cluster G clean-context-reviewer pattern at session level).

M3 strengths: 5/5 PRESERVED-or-EXTENDED.

M2 cost: 16 LOW + 22 MODERATE + 5 HIGH (~43 sub-shapes adopted vs v1's 6 STRONG). Aggregate cost substantially higher than v1, with role-specialization absorbing per-role overhead.

### 2-candidates/README.md index

`docs/redesign/2-candidates/README.md`. Index naming both candidates with one-line position summaries, forward-work declaration (cycle 91+ Candidate C hybrid), audit critique solicitation, iteration discipline reminder per `ITERATION-UNTIL-APPROVAL`, and the most-discriminating axes for cycle 90's first iteration (P3, migration cost, F-pattern structural depth, P6 preservation).

## Differentiation analysis

The two candidates differ on **every axis materially**:

| Axis | Candidate A | Candidate B |
|---|---|---|
| 1 (decomposition) | single-threaded | small-fixed-team (4 agents) |
| 2 (state) | file-per-component | typed-channel-map |
| 3 (memory) | wiki+search | top-level-architectural-principle with per-agent channels |
| 4 (history) | git-as-substrate | branching checkpoints in-tree |
| 5 (plans-as-artifacts) | No | Yes |
| 6 (extension) | tools-with-registry | skills |
| 7 (topology) | sequential-mode-transitions | multi-pattern coexisting |
| 8 (mechanical enforcement) | behavioral-prose CI | per-role + per-channel CI |
| 9 (iteration ceilings) | loop count | loop + runtime per role |
| 10 (entropy mitigation) | minimal deslop | full + curator role |
| 12 (reconciliation) | hybrid (polling + event-driven) | event-driven via reconciler |
| 13 (harness-vs-session) | medium harness | fat harness |

This is **broader than the cycle 89 hand-off named** ("differing significantly on Axis 1"). Authoring against the full framework template surfaced that Axis 1 cannot be varied in isolation — its consequences propagate through every other axis. Methodological observation: the framework's cross-axis dependency map is load-bearing for understanding why Axis 1 is the lever — varying Axis 1 forces choices on Axis 7 (topology), Axis 12 (reconciliation), Axis 13 (harness-session), which in turn force choices on Axis 2 (state), Axis 3 (memory), Axis 4 (history), Axis 6 (extension). The two candidates are the **honest endpoints** of the dependency map.

## Methodological observations

### Phase 2 candidate authoring as cycle composition shape (functional-class shape #13 NEW)

Cycle 90 introduces a NEW cycle composition shape: **Phase 2 candidate authoring**. Defining feature: cycle's substantive focal IS authoring of candidate documents against the design framework template, applying the augmented synthesis surface (M1+M2+M3+M5/P6 + P1-P6 + 9-cluster comparison + cross-cluster intersections + cross-axis dependencies).

This shape is distinct from:

- **Functional-class shape #11** (audit-engagement absorption, cycle 7-12-31 + cycle 85) — that shape's defining feature is per-question evaluation of audit critique. Cycle 90 is NOT this shape (no current audit critique to absorb).
- **Functional-class shape #12** (artifact-resident M-item integration, cycles 86-89, HARDENED at 4 instances) — that shape's defining feature is integrating audit-named missing patterns into cluster annotation surface. Cycle 90 is NOT this shape (M-item arc closed cycle 89).
- **Cold-reader rhythm** (cycles 75-84) — that shape's defining feature is cold-reader prefix + variable substantive focal. Cycle 90's substantive focal is candidate authoring, NOT cold-reader-prefixed (cycle 85 toggle still in continued effect, but its termination at cycle 89 close was for cold-reader; cycle 90 doesn't return to that rhythm).

Pattern emergence is plan-driven (audit#454 P5 named cycle 90 as the exit cycle; cycle 89 hand-off named candidate authoring as the substantive focal). Future cycles may produce same-shape instances (cycle 91+ candidate C authoring, candidate iteration based on critique). 1-instance evidence at cycle 90 close. If shape persists across cycles 91-93+, advances to TESTED at multiple instances.

### Cross-axis dependency map is load-bearing for candidate differentiation

Authoring revealed that the framework's cross-axis dependency map (lines 588-659 of 2-design-framework.md) is the structural lever for differentiation. Choosing Axis 1's position **forces** choices on Axis 7 (topology), Axis 12 (reconciliation), Axis 13 (harness-vs-session) via documented dependencies. Those forced choices in turn **force** Axis 2 (state) and Axis 3 (memory) via Axis 1×Axis 3 + Axis 7×Axis 13 dependencies. The dependency-graph traversal produces two coherent endpoints, not 12 independent dimensions.

Methodological generalization: the framework's cross-axis dependency map is **not optional context**; it's the structural constraint that makes the 12-axis space tractable. A candidate that picks axis positions independently (without honoring cross-axis dependencies) is incoherent.

### P1-P6 evaluation discipline produces concrete differentiation signal

Applying P1-P6 to both candidates produced **discriminating signal** at P3 (self-management-reduction): candidate A passes decisively; candidate B is PARTIAL-FLAG. This validates audit#454 P3 ACCEPT verdict — P3 is the criterion that distinguishes candidates that enable schema work from those that add self-management overhead. Without P3 as an evaluation lens, candidate B's ~43 sub-shape adoption would look like "more comprehensive coverage" rather than "potential per-cycle decision overhead increase."

P6 (audit-as-peer preservation) produced **concrete fit signal**: candidate B's reconciler agent IS the cluster G clean-context-reviewer pattern at session level — structural fit for audit-as-peer that candidate A achieves only via substrate inheritance. P6 surfaced an architectural advantage of candidate B that wasn't visible in failure-mode addressing alone.

P1, P2, P4 produced **PASS verdicts on both** — these criteria don't discriminate at the candidate-A-vs-B level (both candidates structurally cover A↔B intersection, weight cluster I by substrate-fit, and address cluster C lifecycle vocabulary). They serve as **threshold gates** rather than discrimination lenses for the cycle-90 first iteration.

### M3 strengths preservation revealed weaker-than-expected differentiation

Both candidates PRESERVE all 5 M3 v1 strengths. Candidate B PRESERVES-EXTENDED on strengths 3, 4, 5; candidate A merely PRESERVES. Differentiation: extension is real but small-magnitude. M3 strengths preservation is a **conserved-axis** for cycle-90 first-iteration candidates — any candidate that doesn't preserve all 5 fails the framework's M3 gate. Candidate-differentiation comes from extension (B) vs preservation (A), not from preservation vs deviation.

Methodological observation: M3 v1 strengths preservation is a **threshold gate** at the candidate-A-vs-B level (both pass); the differentiation lens is M3 strengths extension (B's reconciler role IS cluster G strength 4 at session level structurally; A inherits via substrate only).

### Honest reflection on cycle 90 authoring discipline

The authoring took ~75 lines of position summary + ~50 lines of cross-axis commitments + ~50 lines of failure-mode addressing + ~50 lines of preserved-primitives + ~50 lines of P1-P6 + ~30 lines of M3 + ~30 lines of M2 + ~50 lines of trade-offs + ~50 lines of migration cost + ~30 lines of tool surface = ~290-330 lines per candidate. That fits the framework template scale honestly.

Per F1 corrective: cycle 90 produced **1 substantive activity** (Phase 2 candidate authoring first iteration) yielding 2 candidate documents + 1 README index + 1 _notes documentation file + 1 journal entry. The components are the deliverable form; documentation IS the activity.

Per F2 corrective: 12 axis positions × 2 candidates = 24 axis-position commitments from 1 activity, NOT "24 ways improved."

Per F3 corrective: 13 functional-class shapes at 30 instances (Phase 2 candidate authoring shape #13 NEW at 1 instance).

Per F4 corrective: HARDENED/TESTED/NOVEL grading is for redesign-process methodology only. P1-P6 PASS/FAIL/PARTIAL grades are empirical claims about candidate compliance, not methodology classification.

Per F5 corrective: lexicon (P1-P6 evaluation criteria, M3 strengths preservation, M2 cost inheritance, axis positions, cross-axis dependencies) as documented learning with operational guidance.

Per H-Q(a) anti-inheritance corrective: cycle 90 candidate grades are empirical-observation-grounded against framework template + augmented synthesis surface. No extrapolation from prior cycles' grading work to cycle 90's candidate authoring.

Per J-Q(a) generalization-level discipline: HARDENED at 7 instances (cycles 84+85+86+87+87-reflection+88+89) HOLDS at cycle 90. Candidate authoring grades are per-axis empirical; aggregate observations (LOW + MODERATE + HIGH counts) are derived; recommendations (P3 PASS for A, P3 PARTIAL-FLAG for B) are grounded in counts.

## Sibling pattern tracking

- **Functional-class shape #13 (Phase 2 candidate authoring) — NOVEL at 1 instance** (cycle 90). New cycle composition shape. If cycle 91+ produces a third candidate or iteration, advances to TESTED at 2 instances.
- **Functional-class shape #12 (artifact-resident M-item integration) — HARDENED at 4 instances** (cycles 86-89). NOT an instance at cycle 90 (M-item arc closed; cycle 90 is post-arc).
- **Cross-axis dependency map as load-bearing structural lever** — methodological observation. 1-instance evidence (cycle 90 candidate authoring revealed the dependency map's role). Future cycles will validate as more candidates author against the framework.
- **P1-P6 evaluation discipline produces discrimination signal** — P3 + P6 produced concrete differentiation; P1 + P2 + P4 served as threshold gates. 1-instance evidence (cycle 90).
- **Generalization-level discipline (J-Q(a)) — HARDENED at 8 instances** (cycle 84 + 85 + 86 + 87 + 87-reflection + 88 + 89 + 90). Continues to hold.
- **Audit-as-peer pattern** — 2-instance evidence holds (audit#442 → cycle 7-12-31; audit#454 → cycle 85). Cycle 90 is NOT a new instance (no current audit critique to absorb).
- **Sibling pattern binary-becomes-more-structured** — HARDENED at 4 instances (cycles 78+79+80+85). Cycle 90 NOT a new instance.

## Bottleneck-state honesty

Bottleneck remains external (Eva's manual Copilot assignment for 4 dispatches: #2833, #2842, #2847, #2851; audit cron for #2849). Cycle 90 contribution is fully repo-internal asynchronous-of-bottleneck like cycles 78-89. Cycle 90 is the **TWELFTH consecutive cycle** (cycles 78-90) whose output is fully repo-internal.

Cycle 90 is the **FIRST Phase 2 candidate authoring cycle**. Per audit#454 P5 toggle (cold-reader cadence suspended cycles 85-89), cycle 90 is the named exit cycle from cold-reader rhythm. Cold-reader cadence remains suspended through Phase 2 candidate authoring (no reactivation criterion met yet).

Cycle 90's persistence shape correlates with cycle composition shape: **Phase 2 candidate authoring produces artifact-resident output in `docs/redesign/2-candidates/` plus _notes documentation plus journal entry.** Same persistence shape as M-item integration cycles (cycles 86-89) — fully artifact-resident, no cross-repo activity.

## Cycle 91 plan

The candidate-selection checkpoint requires Eva's explicit approval. Per `ITERATION-UNTIL-APPROVAL`, cycle 91 substantive focal options:

1. **Author Candidate C (hybrid / middle path)** HIGH PRIORITY. Adopt candidate A's substrate-preservation bet for cycle-internal decomposition (Axis 1 = single-threaded) while adopting candidate B's structural additions for inbound reconciliation (Axis 12 dedicated reconciler-sub-cycle, not separate role) and plans-as-artifacts (Axis 5 = Yes). Whether C is genuinely distinct from A + targeted additions, or just a partial-B, is the cycle 91 authoring question. ~250-300 lines new file `docs/redesign/2-candidates/C-hybrid.md`.

2. **Sharpen Candidates A + B per `ITERATION-UNTIL-APPROVAL`** MEDIUM PRIORITY. Re-read with adversarial framing; identify weak arguments; tighten security analysis; check for self-congratulation; check for over-prescription vs under-prescription mismatches. Specific anticipated weak points:

   - Candidate A's P3 PASS claim depends on Axis 13 medium harness extraction estimate (~50%). The actual extraction percentage requires cycle 92+ work to validate. Sharpen: provide concrete prompt content listing what would be extracted vs retained.
   - Candidate B's P3 PARTIAL-FLAG claim depends on whether per-role overhead or aggregate overhead is the relevant metric. Sharpen: provide concrete per-role decision count estimate (planner: ~? per cycle; executor: ~? per cycle; curator: ~? per cycle; reconciler: ~? per cycle) and compare against v1's actual per-cycle decision count baseline (need to measure).
   - Both candidates' Tool surface implied sections name new Rust crates without concrete schema definitions. Cycle 91+ may prototype 1-2 of the smallest crates (e.g., `tool-registry` for candidate A; `channel-router` schema for candidate B) to ground the migration cost estimates.

3. **Solicit Copilot feedback dispatch on candidates A + B** MEDIUM PRIORITY. Per redesign prompt's `<copilot-as-feedback-peer>`, dispatch a feedback-only Copilot session pointing at this directory and asking for adversarial critique. Multiple parallel Copilot dispatches with different lenses (e.g., one focused on F-pattern coverage; one on P1-P6; one on tool surface feasibility) increase the critique surface.

4. **Audit critique solicitation** LOW PRIORITY (audit reads main per cross-repo discipline; no explicit request needed). Audit cycle 213+ will see cycles 85-90 work and may critique candidates if it lands.

5. **Bounded-mechanical fallback** — close absorbed dispatches if Copilot delivers any of #2833/#2842/#2847/#2851 (no dispatch has delivered post cycle 71 stuck-dispatch diagnosis, but the possibility persists).

**Cycle 91 substantive focal default:** option (1) Candidate C authoring. Rationale: a third candidate is the redesign prompt's "ideally 3" target; authoring it concurrently with sharpening A + B is more efficient than serial work; the hybrid candidate may surface an evaluation lens that's invisible at the A↔B endpoints.

If audit cycle 213 lands critique on cycles 85-90 work during cycle 91's session window, per-question evaluation absorption matches cycle 85 shape — HIGH-LEVERAGE alternative substantive focal (audit-engagement absorption shape #11 RE-INSTANCE).

## Per audit#454 caution applied to cycle 90's own output

**M5/P6 caution honored**: cluster G corpus convergence count (2-system: Cognition + openclaw) NOT updated to include candidate B's structural fit. Candidate B's reconciler agent is **target-system v2-candidate** instantiation, not research-corpus convergence. Audit#454 D2 dual-cast classification applied: candidate B's reconciler agent IS cluster G clean-context-reviewer pattern at session level (target-system instantiation lens) AND IS cluster A↔B intersection sub-pattern (5) component-local-persistence-loaded-at-init (cluster A boundary lens). Multiple lenses on the same architectural element at different abstraction levels.

**P3 caution**: candidate B's PARTIAL-FLAG verdict at P3 is honest. The mitigation argument (role-specialization absorbs per-role overhead) is real but unproven. Cycle 91+ sharpening will measure per-role decision count to validate or invalidate.

**P5 caution**: cold-reader cadence remains suspended through Phase 2 candidate authoring per audit#454 P5 verdict. Cycle 90 does not reactivate cold-reader rhythm. Reactivation criterion is specific artifact restructure where fresh-eyes verification is high-value (e.g., a clusters.md reorganization, a major Phase 2 candidate revision after critique).

## What I would have done differently

**Could have authored Candidate C concurrently with A + B.** The redesign prompt's "at least 2, ideally 3" target was satisfied at the minimum (2). Authoring a third candidate in cycle 90 was technically feasible (~200-300 additional lines) but would have stretched cycle 90's session window. Cycle 91 plan defaults to C authoring; this is the right deferral.

**Could have prototyped 1-2 Rust crates to ground migration cost estimates.** Candidate A's `tool-registry` and candidate B's `channel-router` schema definition are bounded-effort prototypes that would convert the migration cost estimates from prose into concrete artifacts. Deferred to cycle 91+; the candidate documents themselves are the cycle 90 deliverable.

**Could have produced a side-by-side comparison table earlier in the README.** The README's "Selection criteria" section names the most-discriminating axes but doesn't render the table comparing the 12 axes side-by-side (the table is in this _notes file but not in the README). Cycle 91 should add the table to the README as iteration discipline produces a more polished comparison surface.

**Did not solicit Copilot feedback dispatch this cycle.** Per `ITERATION-UNTIL-APPROVAL`, dispatching Copilot critique while authoring is a parallel-work option. Deferred to cycle 91 to keep cycle 90 focused on first-iteration authoring; the critique surface is more informative once a third candidate is on the table.
