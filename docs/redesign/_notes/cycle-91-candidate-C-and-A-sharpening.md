# Cycle 91 — Candidate C authoring + Candidate A Axis 13 extraction sharpening + 3-candidate side-by-side comparison

**Cycle issue:** [#2866](https://github.com/EvaLok/schema-org-json-ld/issues/2866)
**Date:** 2026-05-08
**Mode:** redesign Phase 2 candidate authoring (under [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829) polarity inversion, **thirtieth consecutive cycle of research-corpus advancement**, cycles 62-91)
**Cycle composition shape:** **Phase 2 candidate authoring** (functional-class shape #13, second instance after cycle 90 — advances from NOVEL at 1 instance to **TESTED at 2-instance evidence within the Phase 2 authoring arc**). 13 functional-class shapes total demonstrated cycles 62-91 at 31 instances. Substantive focal: author Candidate C against the augmented synthesis surface; sharpen Candidate A's Axis 13 extraction estimate; update README with 3-candidate side-by-side comparison. Cycle 85 toggle (cold-reader cadence suspended) in continued effect.

## What I did (per cycle 90 hand-off)

### Candidate C: Hybrid / middle path (~280 lines)

`docs/redesign/2-candidates/C-hybrid.md`. Middle path: preserve v1 single-orchestrator substrate (Axis 1 = single-threaded, like A) but elevate inbound-channel reconciliation to first-class harness phase (Axis 7 = 4-mode boot/reconcile/work/close) and adopt plans-as-artifacts (Axis 5 = Yes, like B). **Central bet:** v1's substrate is good (A's bet) AND inbound reconciliation deserves first-class harness phase status without paying multi-agent coordination overhead.

Position summary across 12 axes: file-per-component state (Axis 2 like A), wiki+search memory (Axis 3 like A), git-as-substrate + plan-lifecycle states (Axis 4 between A and B), plans-as-artifacts (Axis 5 = Yes like B), tools-with-registry extension (Axis 6 like A), sequential 4-mode topology (Axis 7 = C-specific), behavioral-prose CI + plan-lifecycle CI (Axis 8 between A and B), loop count + per-mode runtime budget (Axis 9 between A and B), minimal deslop + plan-lifecycle gardening hooks (Axis 10 like A but extended), event-driven with cycle-cadence fallback (Axis 12 between A and B), medium harness with reconciler-mode (Axis 13 like A but extended).

Tool surface implied: ~11 new Rust crates (vs A's 9, vs B's 12+ Rust + 20-40 skill). Aggregate net-add ~4000-6000 LOC. Bounded single-cycle scope per crate (vs B's multi-cycle build-out).

P1-P6: P1 PASS, P2 PASS, P3 PASS-WITH-NOTE (3 more sub-shapes than A; bounded), P4 PASS-WITH-WORK (cluster C sub-shapes 4+5 only — replay + event-trigger), P5 PASS at authoring level, P6 5/5 PASS (slightly stronger than A on Criterion 4, slightly weaker than B on Criterion 4).

M3 strengths: 5/5 PRESERVED, 2 EXTENDED (S2 anti-patterns via plans-as-artifacts technical-debt directory; S3 walkback via plan-lifecycle).

M2 cost: 13 LOW + 10 MODERATE + 1 HIGH (~24 sub-shapes adopted vs v1's 6 STRONG; vs A's 21; vs B's 43).

### Distinctness check (per cycle 90 hand-off question)

Cycle 90 hand-off named: "Whether C is genuinely distinct from A + targeted additions, or just a partial-B."

**Answer:** C is **genuinely distinct** from A on three axes (Axis 5 = Yes vs A's No; Axis 7 = 4-mode vs A's 3-mode; Axis 12 = event-driven-with-fallback vs A's hybrid). C is **preserved-distinct** from B on five axes (Axis 1, 2, 4, 6, 13 all different from B). C is **not** A + targeted additions cosmetically — the reconcile-mode is a new named harness phase that A doesn't have, and plans-as-artifacts is a new artifact category. C is **not** B-light — five structural distinctions preserve A's substrate-bet and bounded migration cost.

The middle path is its own design space, not a hybrid in the dilution sense. C's central bet — that elevating inbound reconciliation to first-class harness phase status is load-bearing without paying multi-agent overhead — is a distinct empirical claim, not a compromise position.

### Candidate A Axis 13 extraction sharpening (~80 lines added)

Per cycle 90 hand-off naming this as the highest-leverage iteration target: appended "Cycle 91 sharpening" section to `A-evolved-single-orchestrator.md` grounding the P3 PASS claim with concrete extraction-vs-retention details:

- Boot-phase responsibilities: ~15 named patterns extracted from v1 STARTUP_CHECKLIST.xml
- Close-phase responsibilities: ~10 named patterns extracted from v1 COMPLETION_CHECKLIST.xml
- Dispatch-poll responsibilities: ~5 named patterns
- Audit-read responsibilities: ~5 named patterns

Aggregate: ~260 named procedural steps in v1 STARTUP + COMPLETION; ~130-160 deterministic-extractable to cycle-runner phases; ~100-130 require orchestrator judgment (substantive-focal selection, situational decisions, sibling-pattern recognition, honest reflection, cross-repo-communication framing, cycle-composition-shape declaration, bottleneck-state honesty, iteration-until-approval discipline).

Estimated v2 prompt size: ~600-800 lines vs v1's ~2400 lines = ~67-75% byte-level reduction; ~50-62% structural extraction at named-step level.

Validation plan (cycle 92+ Phase 3 prototype): author draft v2 prompt + smallest scaffold Rust crate; measure actual extraction percentage; revise P3 grade if extraction < 40%.

Risks named at structural level: (1) named-step taxonomy may be incomplete (lower bound); (2) deterministic-vs-judgment boundary fuzzy for some steps; (3) tool-registry reference grows with tool count; (4) prompt-contract-check CI ongoing-attention cost.

### 2-candidates/README.md update (~80 lines net-add)

Updated `docs/redesign/2-candidates/README.md` with:

- Cycle 91 status update (Candidate C added; "ideally 3" target reached)
- 3-candidate one-line position summary table (A conservative + B aggressive + C middle path)
- Side-by-side 12-axis comparison table (A vs B vs C; bold cells marking C-specific structural distinctions)
- Side-by-side P1-P6 + M3 + M2 + migration comparison table (10 rows × 3 candidates)
- Most-discriminating criteria across 3 candidates (P3, P4, migration cost, F4, F2)
- Forward work for cycle 92+ (sharpen all three; solicit Copilot feedback dispatch; audit critique solicitation; iterate per `ITERATION-UNTIL-APPROVAL`)

## Methodological observations

### Phase 2 candidate authoring as cycle composition shape — TESTED at 2-instance evidence within Phase 2 authoring arc

Cycle 91 is the **second instance** of cycle composition shape #13 (Phase 2 candidate authoring), after cycle 90 NOVEL at 1 instance. The shape advances to **TESTED at 2-instance evidence within the Phase 2 authoring arc**. Defining feature confirmed: cycle's substantive focal IS authoring of candidate documents against the design framework template, applying the augmented synthesis surface.

Cycles 92+ may produce more same-shape instances (sharpening iterations, Copilot feedback absorption, audit critique absorption). If shape persists across cycles 92-95+, advances to HARDENED at 4-instance evidence (per the redesign-process methodology grading discipline).

### Three-candidate authoring surfaces a methodological observation: middle path is a distinct design space

Authoring C against A and B revealed that the cross-axis dependency map (lines 588-659 of 2-design-framework.md) supports **multiple coherent endpoints**, not just two. C is a coherent endpoint that respects the cross-axis dependencies while making different bets:

- A's central bet: substrate is good; structural cleanup sufficient
- B's central bet: single-orchestrator shape is the load-bearing limit; architectural decomposition required
- C's central bet: substrate is good (A's bet) AND one specific concern (inbound reconciliation) deserves first-class harness phase status (a structurally weaker version of B's bet)

The middle path is not a dilution of A and B — it's a separate empirical claim: "elevate one specific concern to first-class harness phase status" is a distinct strategy from "decompose the entire orchestrator into role-specialized agents." The framework's cross-axis dependency map admits this structural endpoint as coherent.

### P3 evaluation discipline produces ordering signal across 3 candidates

Cycle 90 observed P3 produced discrimination signal at A-vs-B (A PASS, B PARTIAL-FLAG). Cycle 91 observation: P3 produces **ordering signal across all 3 candidates** (A PASS decisively → C PASS-WITH-NOTE → B PARTIAL-FLAG). The criterion is monotonic in the direction of self-management cost; the 3-candidate set shows P3 has finer-grained discrimination than 2-candidate set revealed.

This validates audit#454 P3 ACCEPT verdict at higher confidence: P3 is not just a binary discriminator but a continuous ordering criterion. Future Phase 2 work (cycle 92+ sharpening, Copilot feedback) should pay attention to P3 ordering as a candidate-comparison primitive.

### Sharpening is iteration-until-approval discipline (not artifact bloat)

Cycle 91's sharpening of Candidate A's Axis 13 extraction estimate is the first instance of `ITERATION-UNTIL-APPROVAL`-driven artifact deepening (vs cycle 90's first-iteration authoring). The sharpening pattern:

1. Identify a load-bearing claim that the candidate's first iteration left at estimate-level (~50% extraction)
2. Ground the claim with concrete content (extraction-vs-retention named patterns)
3. Name validation plan (cycle 92+ Phase 3 prototype)
4. Name risks at structural level (taxonomy completeness, boundary fuzziness, tool-registry growth, CI ongoing-attention cost)
5. Document residual uncertainty (extraction direction validated; magnitude sharpened to ~40-50% rather than ~50%)

This pattern is repeatable for other load-bearing claims (B's per-role decision count; C's central bet on reconcile-mode load-bearing). Future cycles 92+ should apply the same sharpening pattern to other named weak points.

### Cycle composition is plan-driven AND iteration-until-approval-driven

Cycle 90 cycle composition shape was plan-driven (audit#454 P5 named cycle 90 as exit cycle; cycle 89 hand-off named candidate authoring as substantive focal). Cycle 91 cycle composition shape is **plan-driven AND iteration-until-approval-driven** — cycle 90 hand-off named cycle 91 substantive focal options; cycle 91 chose option (1) Candidate C authoring + option (2) sharpening A's Axis 13 estimate. This is a hybrid generation mechanism distinct from cycles 86-89 M-item integration arc (audit-driven multi-cycle plan).

If cycle 92+ continues `ITERATION-UNTIL-APPROVAL` discipline (sharpening other weak points; soliciting Copilot feedback; revisiting alternatives), the cycle composition will be primarily iteration-driven rather than plan-driven. The Phase 2 candidate-selection checkpoint requires Eva approval; the iteration window may span many cycles.

### Audit#455 implications for v2 candidates

Audit#455 (filed 2026-05-07) names a discovery primitive gap on the audit-side STARTUP_CHECKLIST. Implications for v2 candidates noted in C: P6 Criterion 4 (discovery primitive completeness) is structurally addressed by C's reconcile-mode treating `[audit-request]` events as first-class — better than A's boot-phase pull (interleaved). B's reconciler agent achieves the same via dedicated session-level role. **All 3 candidates' P6 grades reflect this**; cycle 91 update to A's grade not needed (A's PASS-WITH-WORK on Criterion 4 already reflects the gap).

Audit#455 is audit-side concern (audit fixes its own STARTUP_CHECKLIST), not a critique on main's redesign work. No absorption work required this cycle.

## Sibling pattern tracking

- **Functional-class shape #13 (Phase 2 candidate authoring) — TESTED at 2-instance evidence** (cycles 90 + 91 within Phase 2 authoring arc). Advances from NOVEL at 1 instance per cycle 90 to TESTED at 2 instances per cycle 91. If cycles 92+ produce same-shape instances, advances to HARDENED at 4-instance evidence.
- **Cross-axis dependency map admits multiple coherent endpoints** — methodological observation. 1-instance evidence (cycle 91 candidate authoring revealed C as a coherent third endpoint distinct from A and B).
- **P3 evaluation discipline produces ordering signal across 3 candidates** — methodological observation. 1-instance evidence (cycle 91 3-candidate ordering: A PASS → C PASS-WITH-NOTE → B PARTIAL-FLAG).
- **Sharpening as iteration-until-approval discipline** — methodological observation. 1-instance evidence (cycle 91 Candidate A Axis 13 extraction sharpening). Future cycles 92+ may produce same-shape instances when sharpening B's per-role decision count or C's central bet.
- **Functional-class shape #12 (artifact-resident M-item integration) — HARDENED at 4 instances** (cycles 86-89). NOT an instance at cycle 91 (M-item arc closed; cycle 91 is post-arc Phase 2 work).
- **Generalization-level discipline (J-Q(a)) — HARDENED at 9 instances** (cycle 84+85+86+87+87-reflection+88+89+90+91). Continues to hold.
- **Audit-as-peer pattern** — 2-instance evidence holds (audit#442 → cycle 7-12-31; audit#454 → cycle 85). Cycle 91 NOT a new instance (no current audit critique to absorb).
- **Sibling pattern binary-becomes-more-structured** — HARDENED at 4 instances (cycles 78+79+80+85). Cycle 91 NOT a new instance.

## Bottleneck-state honesty

Bottleneck remains external (Eva's manual Copilot assignment for 4 dispatches: #2833, #2842, #2847, #2851; audit cron for #2849). Cycle 91 contribution is fully repo-internal asynchronous-of-bottleneck like cycles 78-90. Cycle 91 is the **THIRTEENTH consecutive cycle** (cycles 78-91) whose output is fully repo-internal.

Cycle 91 is the **SECOND Phase 2 candidate authoring cycle**. Per audit#454 P5 toggle, cold-reader cadence remains suspended through Phase 2 candidate authoring. No reactivation criterion met yet.

Cycle 91's persistence shape correlates with cycle composition shape: **Phase 2 candidate authoring + sharpening produces artifact-resident output in `docs/redesign/2-candidates/` plus _notes documentation plus journal entry.** Same persistence shape as cycle 90 (and as M-item integration cycles 86-89) — fully artifact-resident, no cross-repo activity.

## Honest reflection (per F1 corrective)

Cycle 91 is **1 substantive activity** (Phase 2 candidate authoring + sharpening — second iteration after cycle 90 first iteration) yielding 1 new candidate document (C-hybrid.md ~280 lines) + 1 sharpening section to existing candidate document (A-evolved-single-orchestrator.md ~80 lines added) + 1 README expansion (~80 lines added) + 1 _notes documentation file + 1 journal entry. The components are the deliverable form of the substantive activity; documentation IS the activity, not separable contributions.

Per F2 corrective: 12 axis positions for C + 80 lines of A sharpening + 80 lines of README expansion = ~180+ axis-position commitments and concrete extraction-grounding from 1 activity, NOT "180 ways improved."

Per F3 corrective: 13 functional-class shapes at 31 instances (Phase 2 candidate authoring shape #13 advances from 1 instance to 2 instances within the Phase 2 authoring arc).

Per F4 corrective: NOVEL → TESTED → HARDENED grading is for redesign-process methodology only. P1-P6 PASS/FAIL/PARTIAL/PASS-WITH-NOTE/PASS-WITH-WORK grades for C are empirical claims about candidate compliance.

Per F5 corrective: lexicon (P1-P6 evaluation criteria, M3 strengths preservation, M2 cost inheritance, axis positions, cross-axis dependencies, central bet, distinctness check, sharpening discipline) as documented learning with operational guidance.

Per H-Q(a) anti-inheritance corrective: cycle 91 candidate C grades empirical-observation-grounded against framework template + augmented synthesis surface. NOT extrapolated from cycle 90 candidate A or B grades; each per-axis grade is per-candidate empirical.

Per J-Q(a) generalization-level discipline: HARDENED at 9 instances (cycles 84 + 85 + 86 + 87 + 87-reflection + 88 + 89 + 90 + 91). Continues to hold.

## Cycle 92 plan

The candidate-selection checkpoint requires Eva's explicit approval. Per `ITERATION-UNTIL-APPROVAL`, cycle 92 substantive focal options:

1. **Sharpen Candidate B's per-role decision count empirical estimate** HIGH PRIORITY. Apply the cycle 91 sharpening pattern (concrete extraction-vs-retention; named risks; validation plan) to B. Specific deliverable: per-role decision count (planner: ~? per cycle; executor: ~? per cycle; curator: ~? per cycle; reconciler: ~? per cycle) compared against v1's actual per-cycle decision count baseline. ~80 lines addition to B-decomposed-multi-role.md.

2. **Sharpen Candidate C's central bet** HIGH PRIORITY. C's central uncertainty is whether the reconcile-mode + plans-as-artifacts additions carry their weight. Apply the cycle 91 sharpening pattern: named structural advantages over A (already documented); concrete metric for "carrying their weight" (F4 + F2 + F11 detection legibility — measurable how?); validation plan (cycle 93+ Phase 3 prototype work); risks named at structural level. ~80 lines addition to C-hybrid.md.

3. **Solicit Copilot feedback dispatch on the 3-candidate set** MEDIUM PRIORITY. Per redesign prompt's `<copilot-as-feedback-peer>`, dispatch a feedback-only Copilot session pointing at `docs/redesign/2-candidates/` and asking for adversarial critique. Multiple parallel Copilot dispatches with different lenses (e.g., one focused on F-pattern coverage; one on P1-P6; one on tool surface feasibility) increase the critique surface.

4. **Audit critique solicitation** LOW PRIORITY (audit reads main per cross-repo discipline; no explicit request needed). Audit cycle 213+ will see cycles 85-91 work and may critique candidates if it lands.

5. **Bounded-mechanical fallback** — close absorbed dispatches if Copilot delivers any of #2833/#2842/#2847/#2851 (no dispatch has delivered post cycle 71 stuck-dispatch diagnosis, but the possibility persists).

**Cycle 92 substantive focal default:** option (1) Candidate B sharpening + option (2) Candidate C sharpening as **paired sharpening** (parallel sharpening of B's per-role decision count and C's central-bet validation), or option (3) Copilot feedback dispatch as alternative. Choice: sharpening is more directly load-bearing than feedback solicitation at this iteration — sharpening grounds candidate-specific load-bearing claims, while feedback solicitation surfaces evaluator perspectives. Sharpening should land first; feedback solicitation then has clearer surface to critique.

If audit cycle 213 lands critique on cycles 85-91 work during cycle 92's session window, per-question evaluation absorption matches cycle 85 shape — HIGH-LEVERAGE alternative substantive focal (audit-engagement absorption shape #11 RE-INSTANCE).

## Per audit#454 cautions applied to cycle 91's own output

**M5/P6 caution honored**: Candidate C's `[audit-request]` first-class reconcile-mode treatment is a target-system v2-candidate instantiation, not research-corpus convergence. Cluster G corpus convergence count (2-system: Cognition + openclaw) NOT updated. P6 Criterion 4 grade for C is empirical-observation-grounded: reconcile-mode is dedicated harness phase, not boot-phase side-activity.

**P3 caution**: C's PASS-WITH-NOTE grade is honest. The 3 additional sub-shapes beyond A (replay PARTIAL via plan-lifecycle; event-trigger PARTIAL via reconcile-mode; plan-lifecycle MODERATE state-machine coordination) are the structural cost of C's distinctness from A. Whether these 3 sub-shapes carry their weight is the candidate's central uncertainty — named explicitly in C's "Honest reflection" and in cycle 92 sharpening plan.

**P5 caution**: cold-reader cadence remains suspended through Phase 2 candidate authoring per audit#454 P5 verdict. Cycle 91 does not reactivate cold-reader rhythm. Reactivation criterion is specific artifact restructure where fresh-eyes verification is high-value (e.g., candidate-set restructuring, framework v2 revision after critique).

## What I would have done differently

**Could have authored C concurrently with sharpening B as well.** Cycle 91's sharpening focused on A only. Cycle 92 plan defaults to paired sharpening of B + C; this is the right deferral but cycle 91 could have started B sharpening if session window allowed. Decision: keep cycle 91 focused on Candidate C authoring + A's load-bearing claim; sharpening B + C in cycle 92.

**Could have prototyped 1-2 of A's smallest Rust crates (e.g., `tool-registry`, `boot-phase`) as proof-of-concept for the migration cost estimate.** Deferred to cycle 92+; the validation plan is named in A's sharpening section.

**Could have solicited Copilot feedback dispatch this cycle.** Per `ITERATION-UNTIL-APPROVAL`, dispatching Copilot critique while authoring is a parallel-work option. Deferred to cycle 92+ to keep cycle 91 focused on second-iteration authoring + sharpening; the critique surface is more informative once C is sharpened with its central-bet validation plan.

**Did not produce a per-candidate "what-changed-cycle-91" subsection on each candidate file**, only on A. Could add similar "Cycle 91 sharpening" sections to B and C in later cycles as their specific load-bearing claims are sharpened.

## What surprised me / what I noticed

- **Candidate C's authoring confirmed cycle 90's middle-path-distinctness hypothesis.** Cycle 90 hand-off framed the cycle 91 question as "Whether C is genuinely distinct from A + targeted additions, or just a partial-B." Authoring revealed the answer: genuinely distinct on three axes from A; preserved-distinct on five axes from B. The middle path is its own design space.
- **The cross-axis dependency map admits multiple coherent endpoints.** Cycle 90 observed two endpoints (A and B). Cycle 91 observes three. This validates the framework's cross-axis dependency map as a structural lever for differentiation, not just a binary lever — the dependency-graph traversal produces multiple coherent walks, not just two.
- **P3 evaluation discipline orders all 3 candidates.** A PASS → C PASS-WITH-NOTE → B PARTIAL-FLAG. The criterion is monotonic in self-management cost direction. This is finer-grained discrimination than 2-candidate set revealed.
- **Sharpening A's Axis 13 estimate revealed a magnitude-vs-direction distinction.** The P3 PASS direction (medium-harness extraction reduces procedural surface) is validated by structural analysis. The magnitude (50% vs 40% vs 30%) requires Phase 3 prototype validation. This magnitude-vs-direction distinction is a useful methodological pattern for sharpening other load-bearing claims.
- **Audit#455 is audit-side, not main-side.** Initially read it as a potential critique to absorb; closer reading revealed it's audit's own STARTUP_CHECKLIST gap. No absorption work needed; only noting that all 3 candidates' P6 Criterion 4 grades already reflect this gap correctly.
- **The README's side-by-side comparison tables produced more discrimination signal than narrative comparison would have.** Tabular comparison surfaces the monotonic ordering of P3, the bridging position of C on F4 and F2, and the migration cost ordering in a way prose comparison wouldn't have. Methodological observation: comparison tables are first-class artifacts for candidate selection.
