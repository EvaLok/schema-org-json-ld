# Phase 2 candidate selection

**Status:** **DRAFT — cycle 111 (2026-05-10) initial authoring; awaiting candidate-selection checkpoint approval per `ITERATION-UNTIL-APPROVAL`.**

**Authoring cycle:** 111 (2026-05-10).
**Authored against:** [`2-candidates/README.md`](./2-candidates/README.md) post-cycle-110 state; the three candidate documents [`A-evolved-single-orchestrator.md`](./2-candidates/A-evolved-single-orchestrator.md), [`B-decomposed-multi-role.md`](./2-candidates/B-decomposed-multi-role.md), [`C-hybrid.md`](./2-candidates/C-hybrid.md); the [`2-design-framework.md`](./2-design-framework.md) v1.21+ axes / convergent constraints / failure-mode mapping / P1-P6 evaluation criteria; the cycle 96 PR #2878 sharpening-claims absorption; the cycle 97 PR #2877 tool-surface-feasibility absorption; the cycles 103/105/106/107/108/109/110 risk closures at specification level.

This document is the Phase 2 **selection** artifact named in the redesign prompt's SECTION 8 ("Output: `docs/redesign/2-candidates.md` and `docs/redesign/2-selection.md`"). Per the redesign prompt's CHECKPOINTS, candidate-selection is the second of three hard checkpoints requiring Eva's explicit approval. Per `ITERATION-UNTIL-APPROVAL`, this draft is not the deliverable until Eva approves; iteration continues until then.

## Tentative selection: Candidate A (Evolved Single-Orchestrator)

**Tentative recommendation: select [`A — Evolved Single-Orchestrator`](./2-candidates/A-evolved-single-orchestrator.md) for Phase 3 prototype work.**

This recommendation is **provisional**. It rests on the analysis below. It can be overridden by:

- Eva's direct judgment at the checkpoint (Eva is authoritative; this draft is input).
- New evidence from audit critique not yet landed (audit cycle 215 named cycle 216 as the substantive Phase 2 critique cycle; landing in ~6h-1d). If audit critique shifts the analysis materially, this draft updates.
- Copilot adversarial-feedback dispatch landings (none specifically targeting the selection have been dispatched yet; see "Outstanding work before lock-in" below).
- Phase 3 prototype evidence on a smallest-load-bearing crate (cycles 93-94 measured `v2-tool-registry` 231 LOC + `v2-cycle-history-append` 268 LOC; further measurements may sharpen the migration-cost analysis enough to change the ordering).

The recommendation is **not** that Candidate A is "best in all dimensions." It is that, given the redesign's primary thesis ("autonomous AI system can be self-healing and self-improving" with the schema-org-json-ld work as proof domain) and the CORE-DESIGN-PRINCIPLE ("tools and deterministic processes handle repetitive, rote, procedural work; the orchestrator spends its compute on improving the system and responding to novel circumstances"), Candidate A is the candidate whose central bet is most defensible **and** whose cost of being wrong is most bounded.

## The decision criteria, in order of weight

### 1. Schema-work-enabling (the redesign's primary thesis)

The redesign's primary thesis is that the system demonstrates self-improvement on a real proof domain — schema-org-json-ld definition maintenance across multiple programming languages. Every cycle the orchestrator spends on self-management is a cycle not spent on schema work. The decisive question:

> Which candidate puts the most cycles into the proof domain and the fewest into self-coordination?

- **A:** ~21 sub-shapes adopted; Axis 13 medium harness extracts ~50% procedural surface from prompt; per-cycle decision overhead estimate **~30-40% lower than v1's apparent surface**. Schema-work-enabling: **YES** (per A's P3 PASS verdict).
- **B:** ~43 sub-shapes adopted; per-role decision count is small but aggregate per-cycle work across 4 agents may exceed v1's. Schema-work-enabling: **CONDITIONAL** (per B's own document — depends on whether per-role overhead or aggregate overhead is the relevant metric; B explicitly names this as its central uncertainty).
- **C:** ~24 sub-shapes adopted; same Axis 13 medium-harness extraction as A; per-cycle decision overhead estimate **~25-35% lower** (slightly less reduction than A due to plan-lifecycle and reconcile-mode coordination). Schema-work-enabling: **YES** with note (P3 PASS-WITH-NOTE).

**Ordering on this criterion: A > C > B**, with B's classification as CONDITIONAL being the load-bearing distinction. The redesign's primary thesis is poorly served by a candidate that is conditionally schema-work-enabling.

### 2. CORE-DESIGN-PRINCIPLE compliance (small prompt + substantial tools)

The CORE-DESIGN-PRINCIPLE names the new prompt as **small** and the tool suite as **substantial**. It frames extraction-of-procedure-into-tools as the test for design quality.

- **A:** prompt ~50% smaller than v1; ~9 new Rust crates; tool surface bounded and discoverable via tool-registry. **Direct fit.**
- **B:** four role-specific prompts at ~30-40% v1's size each, but aggregate prompt content ~50% larger than v1; ~12+ Rust crates plus 20-40 skill crates. **Inverts the principle's reduction direction at the prompt level.** B trades single-prompt accretion for multi-prompt multiplication.
- **C:** prompt ~50% smaller than v1 (same as A); ~11 new Rust crates. **Direct fit, plus 2 additional crates for plan-lifecycle and reconcile-mode.**

**Ordering on this criterion: A ≈ C > B.** A and C both honor the principle; B's structure is in tension with it (the per-role-prompt aggregate is larger than v1's prompt).

### 3. Cost of being wrong (rollback / migration / Phase 3 effort)

The redesign prompt's SECTION 9 names the third checkpoint (pre-cutover) as gated on the artifact being complete; "we'll polish the tools after cutover" is explicitly disallowed. The cost of being wrong has two forms: (a) Phase 3 prototype effort wasted if the candidate fails experiments and we restart, (b) Phase 4 cutover risk if the production cutover encounters problems.

| Aspect | A | C | B |
|---|---|---|---|
| New Rust crates | ~9 | ~11 | ~12+ Rust + 20-40 skills |
| Net-add LOC (PR #2877 revised) | ~3600-6200 | ~5200-8200 | ~14000-28000 |
| Per-crate scope (median) | ~200-500 LOC (validated 2 of 9 at 231 + 268 LOC) | ~200-500 LOC | "several thousand LOC each" for key infra (per B's own line 62) |
| Cutover scope | bounded single-cycle per trivial crate; 1-2 cycles per orchestration-hub crate | bounded single-cycle per A-shared crate; 2-3 cycles for reconcile-mode + plan-lifecycle pair | **multi-cycle build-out (16-30 cycles for non-negotiable quality per PR #2877 lens-5)** |
| Cutover predictability | medium-high (PR #2877 revises to medium-with-trivial-crate-bias) | medium (PR #2877 names C as most under-justified estimate posture) | low |
| Rollback complexity | revert workflow YAML + delete v2 artifacts | revert workflow YAML + delete v2 artifacts | revert workflow YAML + delete v2 artifacts + per-channel state cleanup |

**Ordering on this criterion: A > C > B**, with B 3-5× the migration cost of A and a multi-cycle cutover scope vs A's per-crate scope.

### 4. Failure-mode addressing depth (P1, P4, F1-F12)

Each candidate addresses F1-F12 differently. The decisive question is whether B's or C's deeper failure-mode addressing is **load-bearing** for v1's actually-observed failure modes, or whether A's lighter addressing **suffices in practice**.

The retrospective (`0-retrospective.md`) names v1's actually-observed failure-mode pattern: chronic-category-currency loop, Eva-blocker queue, abandonment cascade, gate proliferation, self-management dominance. The dominant failure pattern is **F1 (constraint accretion) compounding F7 (self-management dominance)** — exactly what A's Axis 13 medium-harness extraction is designed to address most directly. B's role-decomposition addresses F1+F7 by absorbing constraint surface into specialized roles; A's harness extraction addresses F1+F7 by removing constraint surface from the prompt entirely.

C's incremental addressing over A is concentrated in F4 (frozen-artifact lifecycle) and F11 (post-close mutations) — both of which v1 has, but neither of which is in the dominant observed-failure-pattern cluster. **C's additions address F4/F11 better than A; the question is whether F4/F11 are load-bearing.** C's own document acknowledges this as the central uncertainty: "whether these 3 sub-shapes carry their weight by improving F4 + F2 + F11 detection legibility, or whether they are pure overhead."

**Ordering on this criterion: B > C > A**, but with the caveat that B's additional depth addresses failure modes that may not be load-bearing for v1's observed failure pattern. If the dominant failure pattern is F1+F7, A is structurally appropriate.

### 5. The candidate's central bet defensibility

Each candidate has a central bet:

- **A:** "v1's substrate is good and the load-bearing cleanup is structural (state.json, _notes-as-memory, harness extraction) rather than architectural (multi-role decomposition)." Defensibility: this bet is **directly supported** by the retrospective's identification of F1+F7 as the dominant failure pattern, and **directly testable** in Phase 3 by measuring whether harness extraction reduces orchestrator decision count.
- **B:** "v1's single-orchestrator shape is the load-bearing limit on cluster G (role-asymmetric context), cluster H (post-session feedback), and Axis 12 (reconciliation) — and that the failure modes those clusters address require dedicated roles, not just better tools." Defensibility: cluster G role-asymmetric context is a real consideration, but the retrospective does not identify it as the dominant failure pattern. The bet **requires** that role-asymmetric context (which v1 doesn't have) is load-bearing for v2 — this is an aspirational extension, not a v1-failure-mode-driven extension.
- **C:** "v1's substrate is good (A's bet) **and** inbound reconciliation deserves structural first-class status without paying the multi-agent coordination overhead — the load-bearing addition between A and B is harness-phase elevation of one specific concern (reconciliation), not full role decomposition." Defensibility: C's bet is **plausible but unproven**. C's own document names this as conditional-improvement risk: "if the additions don't carry their weight, C is just A with more migration surface."

**Ordering on this criterion: A > B ≈ C**, with A's bet most directly grounded in v1's observed failure pattern.

### 6. Audit-as-peer P6 preservation

Per the redesign prompt's SECTION 5 (Authority — audit-as-peer), preservation of audit-as-peer is a structural concern.

- **A:** 5/5 PASS-WITH-WORK on Criterion 4 (discovery primitive needs implementation; not preserved-by-construction).
- **B:** 5/5 PASS (reconciler agent IS the cluster G clean-context-reviewer pattern at session level).
- **C:** 5/5 PASS (reconcile-mode is dedicated harness phase, slightly stronger than A on Criterion 4 but slightly weaker than B on Criterion 4 because C's reconcile-mode shares context with work-mode).

**Ordering on this criterion: B > C > A.** Margin is narrow; all three preserve audit-as-peer at the substrate level. The Criterion 4 differences are about how legibly the discovery primitive lives in the design, not whether it works.

## Cross-candidate comparison: discriminating axes side by side

**Source:** [`2-candidates/README.md`](./2-candidates/README.md) "Side-by-side P1-P6 + M3 + M2 + migration comparison" table (post-cycle-110 state). Reproduced here for selection-decision legibility.

| Criterion | Candidate A | Candidate C | Candidate B |
|---|---|---|---|
| **P3 (self-management-reduction)** | PASS | PASS-WITH-NOTE | PARTIAL-FLAG |
| **P4 (lifecycle-vocabulary completeness)** | PARTIAL (bypass clause) | PASS-WITH-WORK (sub-shapes 4+5) | PASS (sub-shapes 1-5) |
| **P6 (audit-as-peer preservation)** | 5/5 PASS-WITH-WORK | 5/5 PASS | 5/5 PASS |
| **M2 cost (LOW + MOD + HIGH)** | 12 + 8 + 1 (~21 sub-shapes) | 13 + 10 + 1 (~24 sub-shapes) | 16 + 22 + 5 (~43 sub-shapes) |
| **Migration: net-add LOC (PR #2877 revised)** | ~3600-6200 | ~5200-8200 | ~14000-28000 |
| **Cutover scope** | bounded single-cycle per crate | bounded; 2-3 cycles for reconcile-mode + plan-lifecycle | multi-cycle build-out (16-30 cycles) |
| **Cutover predictability** | medium-high (PR #2877: medium-with-trivial-crate-bias) | medium (PR #2877: most under-justified posture) | low |
| **Schema-work-enabling answer** | YES (P3 PASS) | YES with note (P3 PASS-WITH-NOTE) | CONDITIONAL (P3 PARTIAL-FLAG) |
| **Central bet defensibility** | high (grounded in v1's observed F1+F7 pattern) | medium (bet on F4/F11 load-bearingness) | medium (bet on cluster G role-asymmetry being load-bearing for v2) |
| **F1+F7 (dominant v1 failure pattern) addressing** | strong (Axis 13 harness extraction) | strong (same as A) + plan-lifecycle CI | strong (per-role specialization) |
| **F4 (frozen-artifact lifecycle) addressing** | weak (no plan-lifecycle) | strong (3-state plan-lifecycle) | strong (3-state plan-lifecycle + branching checkpoints) |
| **F11 (post-close mutations) addressing** | medium (boot-phase polling) | strong (reconcile-mode first-class) | strong (reconciler agent first-class) |

## What Candidate A gives up

Honesty per the redesign prompt's SECTION 8 directive ("you pick one with rationale, naming what you give up"):

1. **No dedicated reconciler agent / no first-class reconcile-mode.** A's Axis 12 reconciliation interleaves with primary work in boot phase. If the volume of inbound channels grows substantially (Eva responses + audit posts + dispatch PR-merges + future channels), the boot phase becomes the bottleneck. C's reconcile-mode is the structural alternative; B's reconciler agent is the more aggressive alternative. **Mitigation:** Phase 3 measurement of reconcile-equivalent boot-phase work; if measured to be substantial fraction of cycle budget, revisit C-style reconcile-mode addition.

2. **No plans-as-artifacts directory.** A's Axis 5 = No defers plan-lifecycle as not-yet-needed. If multi-cycle plan coordination grows, this becomes friction. C's plans-as-artifacts is the structural alternative. **Mitigation:** Phase 3 + early-Phase-4 monitoring; if plan-related drift surfaces, retroactively adopt C's plan-lifecycle.

3. **No parallel exploration branches.** A's Axis 4 git-as-substrate without branching checkpoints means what-if reasoning across multi-cycle horizons is harder. The cycle-history append-only files give lookback, not lookahead. B's branching checkpoints are the structural alternative. **Mitigation:** if lookahead becomes valuable for Phase 3 design or future redesign work, the branching-checkpoints primitive can be added later as a layered extension.

4. **Higher cluster A 6-ABSENT count adopted.** A inherits cluster A's 6-ABSENT sub-shapes (phase-boundary semantics + recovery operations + concurrency-queuing) at LOW-MODERATE cost only. **Risk:** if v1's cluster A failure modes return at v2 — particularly recovery operations or concurrency queuing if Phase 3 surfaces them — A has limited structural defense beyond cluster B intersection coverage. **Mitigation:** Phase 3 stress-test specifically for cluster A failure modes (deliberate corruption / abandonment / restart cycles) per the redesign prompt's SECTION 8 Phase 3 directive.

5. **F4 and F11 detection legibility weaker than C/B.** A's mechanisms work, but they're less structurally legible than C's reconcile-mode or B's reconciler agent. Eva and audit reading the state of an A-system have to infer reconciliation state from boot-phase logs rather than reading a dedicated reconciliation surface. **Mitigation:** if legibility to Eva/audit becomes friction, revisit reconcile-mode addition.

6. **Discovery primitive completeness (P6 Criterion 4 PASS-WITH-WORK).** A adopts `[audit-request]` title prefix + label as discovery primitive but the implementation is required-not-yet-built. **Mitigation:** Phase 3 prototype work includes the audit-request-discovery primitive.

## Why not B (decomposed multi-role)

B's structural depth is real and well-argued. The reasons not to select B:

1. **Schema-work-enabling is conditional, not affirmative.** B's own document names this as its central uncertainty. The redesign's primary thesis cannot rest on a candidate that's conditionally schema-work-enabling.

2. **Migration cost is 3-5× A's.** PR #2877 revised B's net-add to 14000-28000 LOC vs A's 3600-6200. Cutover is multi-cycle (16-30 cycles per PR #2877 lens-5) vs A's per-crate single-cycle. The cost of being wrong about B is large.

3. **Inverts the CORE-DESIGN-PRINCIPLE at the prompt level.** B's aggregate prompt content is ~50% larger than v1's via the four role-specific prompts. The principle names "small prompt" as the design direction; B's structure is in tension with this.

4. **Central bet requires aspirational extension.** B's bet that cluster G role-asymmetric context is load-bearing for v2 requires extending v1 in a direction v1 does not currently have. The retrospective's observed-failure pattern (F1+F7) is addressable without role-asymmetric context. B's depth is appropriate if the proof-domain genuinely needs role parallelism — but the schema-org-json-ld proof domain has not been characterized as needing parallelism at this scale.

5. **P3 PARTIAL-FLAG is structurally concerning.** B's mitigation (per-role specialization absorbs per-role decision count) is real-but-unproven. A reasonable Phase 3 outcome could refute the mitigation, which would surface the P3 concern late in the work.

**B is preferable to A only if** Phase 3 surfaces evidence that v1's substrate (single-orchestrator shape) is the load-bearing limit on schema-work scaling. There is no such evidence currently.

## Why not C (hybrid)

C's positioning is clever — preserve A's bounded migration cost while adopting two B-targeted structural elements (reconcile-mode + plans-as-artifacts). The reasons not to select C:

1. **Conditional-improvement risk.** C's own document names this risk: "if the additions don't carry their weight, C is just A with more migration surface." The 3 additional sub-shapes (plan-lifecycle + reconcile-mode + per-mode budgets) are at most 25-35% per-cycle decision overhead reduction vs A's 30-40% — C pays additional cost for additional structural depth in F4/F11.

2. **PR #2877 names C as most under-justified estimate posture.** C's claimed +~1000-1500 LOC over A may drift higher (PR #2877 revised range 5200-8200 vs A's revised 3600-6200, narrowing the docs-stated A vs C distance). C is the candidate most likely to miss its own upper-bound estimate.

3. **F4/F11 addressing improvements over A are unproven as load-bearing.** The retrospective's observed-failure pattern is F1+F7 dominant; C's improvements are concentrated in F4/F11. **C is preferable to A only if** F4/F11 are load-bearing for v2 — which is the bet, and the bet is not validated.

4. **Per-mode runtime budgets add complexity at unproven value.** C's per-mode runtime budgets (boot ≤5min, reconcile ≤10min, work ≤50min, close ≤10min) give finer-grained abandonment detection but require per-mode-runtime-tracking infrastructure. The cycle-level budget (~75min) is sufficient for v1; the value of finer-grained budgets is unproven.

**C is preferable to A only if** F4/F11 detection legibility is judged structurally important at the candidate-selection level (Eva's call), or if Phase 3 surfaces evidence that boot-phase reconciliation is the bottleneck. There is no such evidence currently.

## Outstanding work before lock-in

This draft is **not** the deliverable. Per `ITERATION-UNTIL-APPROVAL`, the following work is outstanding before the candidate-selection checkpoint should be considered ready for Eva's approval:

1. **Audit critique landing.** Audit cycle 215 (today) named cycle 216 as the substantive Phase 2 critique cycle. Audit critique on the candidate set should land on this draft and any updates produced in response. Cycle 112+ absorption work.

2. **Adversarial Copilot feedback dispatch on the selection.** No Copilot feedback dispatch has been targeted specifically at the selection decision. Dispatching one (or several with different lenses: "argue for B"; "argue for C"; "argue this analysis is missing a candidate not yet authored") is named in the redesign prompt's `ITERATION-UNTIL-APPROVAL` and would test whether the analysis survives external adversarial framing. Cycle 112+ work.

3. **Stress-test the central bet.** The recommendation rests on F1+F7 being the dominant v1 failure pattern. If a careful adversarial re-read of the retrospective surfaces a competing dominant-pattern characterization (e.g., F4+F11 as load-bearing; or cluster-G role-asymmetry as load-bearing), the recommendation must be revisited.

4. **Phase 3 prototype effort on a smallest-load-bearing crate from each candidate.** The redesign prompt's SECTION 8 names this as bounded preview: 1-2 of A's smallest Rust crates as proof-of-concept for migration-cost validation; same for C's `reconcile-mode` if Eva pre-approves; B's smallest crate is harder to identify because B's smallest infra is `several thousand LOC each` per B's own document. Cycle 93-94 already measured A's `tool-registry` (231 LOC) and `cycle-history-append` (268 LOC); extending to A's `boot-phase` and `wiki-search` would deepen A-side evidence; equivalent measurement for C's `reconcile-mode` would inform the A vs C comparison.

5. **Cycle 103-110 risk-closure substrate absorption.** The seven specification-level risk closures specified pre-agreed falsifiable thresholds for Phase 3 measurement. The selection rationale above does not yet enumerate these as part of "what Phase 3 needs to validate." A more complete selection draft would include the threshold table, organized by candidate. Cycle 112+ work.

6. **Open questions for Eva** (see next section). Some questions are genuinely Eva-only judgment calls; the answers shape the selection.

## Open questions for Eva at the candidate-selection checkpoint

These questions are explicitly Eva-only per `EVA-DEFAULT-AUTONOMY` (scope authorization, infrastructure access, the candidate-selection checkpoint itself). The orchestrator's analysis does not pre-empt them:

1. **Approve Candidate A for Phase 3 prototype work?** This is the affirmative question. If yes: Phase 3 begins with A as the prototype target.

2. **Override and pick Candidate C if F4/F11 detection legibility is judged structurally important?** This is the most plausible override. The orchestrator's analysis names F4/F11 as not-clearly-load-bearing, but Eva may judge differently — the legibility-to-Eva-and-audit dimension is one Eva is best positioned to evaluate.

3. **Pick Candidate B if the schema-work proof-domain genuinely needs multi-role decomposition for parallelism?** This is the maximally-aggressive override. The orchestrator's analysis names this as not-currently-evidenced, but Eva may have forward-looking judgment about the schema-work scope that the orchestrator does not.

4. **Direct an additional candidate not yet authored?** The redesign prompt's SECTION 8 names "Required: at least 2 distinct design candidates, ideally 3" — three are authored, but Eva may judge the candidate set as not spanning the right dimensions.

5. **Extend the iteration window before approval?** The orchestrator's default behavior is to continue iterating per `ITERATION-UNTIL-APPROVAL`. Eva can name a specific iteration target (audit critique landing, Copilot feedback dispatch landing, additional Phase 3 prototype evidence) before the approval is given.

## Honest meta-observation: the cycle 103-110 risk-closure pattern and this draft

The cycles 103-110 work produced 7 specification-level risk closures with falsifiable thresholds. These are real value: they make Phase 3 measurement meaningful by specifying ahead-of-time what counts as direction-validated / at-risk / refuted.

But: 22 consecutive bottleneck-asynchronous cycles (cycles 78-110) have produced a meta-tracking framework (24 functional-class shapes at 54 instances, with NOVEL→TESTED→HARDENED promotion mechanics) that is in tension with the selection question. The shape-tracking work tracks the orchestrator's own pattern-extraction; it does not directly advance the cross-candidate comparison that selection requires.

Cycle 111's substantive focal — drafting this selection artifact — was a deliberate departure from the cycle 110-named forward-going commitment (rotate to A-side or B-side risk closure). The departure was reasoned as follows:

- The Phase 2 deliverable per the redesign prompt explicitly requires `2-selection.md`. It did not exist. Eva cannot approve a selection at a checkpoint with no selection artifact.
- The risk closures specify per-candidate thresholds; selection is the cross-candidate comparison; the two activities are complementary, not redundant.
- Continuing rotation is the self-amplifying-loop pattern that `ITERATION-UNTIL-APPROVAL`'s "examine for self-congratulation" activity is designed to interrupt. Selection-draft work is the honest exit.

This meta-observation is included for Eva's transparency: the orchestrator made a discretionary choice to break a forward-going commitment named in the prior cycle's _notes. The choice is defensible (the selection artifact gap was real and load-bearing for the checkpoint) but it is also a discretionary choice. Eva can override the choice; if she names "continue rotation, defer selection draft" the orchestrator complies and revisits this draft in a later cycle.

## Iteration log

| Cycle | What changed | What's next |
|---|---|---|
| 111 (2026-05-10) | Initial authoring. Tentative selection: A. Six decision criteria with cross-candidate ordering. What A gives up enumerated. Why-not-B and why-not-C sections. Six outstanding work items before lock-in. Five open questions for Eva. Honest meta-observation on the cycle 103-110 pattern. | Audit cycle 216 critique landing (~6h-1d); cycle 112 absorption. Possible Copilot feedback dispatch on this draft for adversarial framing (cycle 112+). Cycle 103-110 risk-closure threshold table integration (cycle 112+). Per-candidate Phase 3 prototype evidence deepening (cycle 113+). |
