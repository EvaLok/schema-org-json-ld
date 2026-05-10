# Phase 2 candidate selection

**Status:** **DRAFT — cycle 111 (2026-05-10) initial authoring; cycle 112 (2026-05-10) central-bet stress-test absorbed; awaiting candidate-selection checkpoint approval per `ITERATION-UNTIL-APPROVAL`.**

**Authoring cycle:** 111 (2026-05-10) initial; 112 (2026-05-10) central-bet stress-test surfacing family-level vs pattern-level dominance tension.
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

The retrospective (`0-retrospective.md`) names v1's actually-observed failure-mode pattern: chronic-category-currency loop, Eva-blocker queue, abandonment cascade, gate proliferation, self-management dominance. **There are two valid readings of "dominance" in the retrospective, and the cycle 111 selection draft picked one without surfacing the other.**

- **Family-level dominance (per retrospective lines 161-162 and 959):** the retrospective explicitly identifies **reconciliation asymmetry (F2/F3/F4/F5†/F11† family) as the dominant family**, with the cleanest single-sentence statement: *"v1 has tools to **create** records and tools to **summarize** records; it lacks tools to **reconcile** records against later events."*
- **Pattern-level prevalence (cycle 111 framing, derived from cycle 2 measurements):** F1 (constraint accretion, defense-accretion family) and F7 (self-management dominance, procedure/review-overhead family) are the most-pervasive and most-measurable individual patterns — F7 has 130+ consecutive cycles of zero schema work; F1 has constraint-stack-on-stack evidence in `COMPLETION_CHECKLIST.xml`'s C3 step.

**The cycle 111 framing "the dominant failure pattern is F1 (constraint accretion) compounding F7 (self-management dominance)" is a pattern-level framing that crosses family boundaries** — F1 is in the defense-accretion family, F7 is in the procedure/review-overhead family — and **does not match the retrospective's family-level dominance claim**. The cycle 112 stress-test surfaces this as the central uncertainty (see "Central uncertainty: family-level vs pattern-level dominance" section below).

C's incremental addressing over A is concentrated in F4 (frozen-artifact lifecycle), F11 (post-close mutations), and F2 (Eva-response detection) — **all three of which are in the reconciliation asymmetry family that the retrospective identifies as dominant at the family level**. C addresses these via reconcile-mode as a first-class harness phase (vs A's boot-phase interleaved polling). **However: A and C have equivalent reconciliation OUTCOMES** — both have 1-cycle latency, both use cursor-file polling at session-start (`state/eva-input-cursor.json`). The structural difference between A and C on reconciliation is **first-class named-phase legibility, not detection mechanism or latency**.

**Ordering on this criterion under the family-level dominance reading: C > B > A** (C addresses F2/F4/F11 with equivalent outcomes to A but better structural legibility; B addresses with most depth but inverts the CORE-DESIGN-PRINCIPLE at the prompt level). **Ordering under the pattern-level prevalence reading: B > C > A** (B addresses F1+F7 most aggressively via role decomposition; A and C have similar F1+F7 addressing via Axis 13 medium-harness extraction). **The cycle 112 finding shifts this Criterion's ordering from cycle 111's "B > C > A with caveats" to "C > B > A under family-level reading OR B > C > A under pattern-level reading — the dominance framing is the central uncertainty Eva resolves."**

### 5. The candidate's central bet defensibility

Each candidate has a central bet:

- **A:** "v1's substrate is good and the load-bearing cleanup is structural (state.json, _notes-as-memory, harness extraction) rather than architectural (multi-role decomposition)." Defensibility: this bet is **consistent with the procedure/review-overhead family (F6/F7/F9) and the defense-accretion family (F1/F5/F11/F12) addressing**, and is **consistent with reconciliation asymmetry addressing at the outcome level** (A's Axis 12 hybrid polling matches C's reconcile-mode latency). **However, the cycle 111 framing "directly supported by the retrospective's identification of F1+F7 as the dominant failure pattern" was mis-grounded** — F1+F7 are individual patterns in different families, and the retrospective at lines 161-162 identifies reconciliation asymmetry (a different family) as dominant at the family level. A's bet remains **directly testable** in Phase 3 by measuring whether harness extraction reduces orchestrator decision count, but the central-bet defensibility is now contested rather than confidently supported.
- **B:** "v1's single-orchestrator shape is the load-bearing limit on cluster G (role-asymmetric context), cluster H (post-session feedback), and Axis 12 (reconciliation) — and that the failure modes those clusters address require dedicated roles, not just better tools." Defensibility: cluster G role-asymmetric context is a real consideration, but the retrospective does not identify it as the dominant failure pattern at any reading. The bet **requires** that role-asymmetric context (which v1 doesn't have) is load-bearing for v2 — this is an aspirational extension, not a v1-failure-mode-driven extension.
- **C:** "v1's substrate is good (A's bet) **and** inbound reconciliation deserves structural first-class status without paying the multi-agent coordination overhead — the load-bearing addition between A and B is harness-phase elevation of one specific concern (reconciliation), not full role decomposition." Defensibility: **post-cycle-112 stress-test, C's bet is now more directly defensible than cycle 111 framed it** — the retrospective's family-level dominance reading (reconciliation asymmetry as dominant family) is consistent with C's "inbound reconciliation deserves first-class status" claim. C's own document still names a residual conditional-improvement risk: "if the additions don't carry their weight, C is just A with more migration surface" — but the cycle 112 finding shifts the conditional from "if F4/F11 are load-bearing" to "if first-class structural legibility (named phase) is worth +~1500-2000 LOC over A's interleaved-with-boot reconciliation."

**Ordering on this criterion: A ≈ C > B** (downgrade from cycle 111's "A > B ≈ C"). A's bet (substrate-good + structural-cleanup) and C's bet (substrate-good + structural-cleanup + first-class reconciliation) are both consistent with the retrospective at different abstraction levels; B's bet (cluster-G role-asymmetry as load-bearing for v2) requires aspirational extension. **A vs C on this criterion is now contested rather than A-dominant**, because A's "structural cleanup suffices" claim is challenged by the family-level reconciliation-asymmetry-as-dominant reading. The remaining A-over-C argument on Criterion 5 is that A's bet is more parsimonious (fewer structural moving parts), but parsimony is not the same as defensibility against the retrospective's framing.

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
| **Central bet defensibility** | **contested post-cycle-112** (cycle 111 grounding "F1+F7 dominant" mis-aligned with retrospective family-level "reconciliation asymmetry dominant"; bet remains consistent with one valid reading but no longer "directly supported") | **upgraded post-cycle-112** to consistent-with-family-level-dominance reading (first-class reconciliation matches retrospective's dominant-family addressing) | medium (bet on cluster G role-asymmetry being load-bearing for v2 — no retrospective grounding at any reading) |
| **F1+F7 (dominant v1 failure pattern) addressing** | strong (Axis 13 harness extraction) | strong (same as A) + plan-lifecycle CI | strong (per-role specialization) |
| **F4 (frozen-artifact lifecycle) addressing** | weak (no plan-lifecycle) | strong (3-state plan-lifecycle) | strong (3-state plan-lifecycle + branching checkpoints) |
| **F11 (post-close mutations) addressing** | medium (boot-phase polling) | strong (reconcile-mode first-class) | strong (reconciler agent first-class) |

## What Candidate A gives up

Honesty per the redesign prompt's SECTION 8 directive ("you pick one with rationale, naming what you give up"):

1. **No dedicated reconciler agent / no first-class reconcile-mode.** A's Axis 12 reconciliation interleaves with primary work in boot phase. If the volume of inbound channels grows substantially (Eva responses + audit posts + dispatch PR-merges + future channels), the boot phase becomes the bottleneck. C's reconcile-mode is the structural alternative; B's reconciler agent is the more aggressive alternative. **Mitigation:** Phase 3 measurement of reconcile-equivalent boot-phase work; if measured to be substantial fraction of cycle budget, revisit C-style reconcile-mode addition.

2. **No plans-as-artifacts directory.** A's Axis 5 = No defers plan-lifecycle as not-yet-needed. If multi-cycle plan coordination grows, this becomes friction. C's plans-as-artifacts is the structural alternative. **Mitigation:** Phase 3 + early-Phase-4 monitoring; if plan-related drift surfaces, retroactively adopt C's plan-lifecycle.

3. **No parallel exploration branches.** A's Axis 4 git-as-substrate without branching checkpoints means what-if reasoning across multi-cycle horizons is harder. The cycle-history append-only files give lookback, not lookahead. B's branching checkpoints are the structural alternative. **Mitigation:** if lookahead becomes valuable for Phase 3 design or future redesign work, the branching-checkpoints primitive can be added later as a layered extension.

4. **Higher cluster A 6-ABSENT count adopted.** A inherits cluster A's 6-ABSENT sub-shapes (phase-boundary semantics + recovery operations + concurrency-queuing) at LOW-MODERATE cost only. **Risk:** if v1's cluster A failure modes return at v2 — particularly recovery operations or concurrency queuing if Phase 3 surfaces them — A has limited structural defense beyond cluster B intersection coverage. **Mitigation:** Phase 3 stress-test specifically for cluster A failure modes (deliberate corruption / abandonment / restart cycles) per the redesign prompt's SECTION 8 Phase 3 directive.

5. **F4 and F11 detection legibility weaker than C/B.** A's mechanisms work, but they're less structurally legible than C's reconcile-mode or B's reconciler agent. Eva and audit reading the state of an A-system have to infer reconciliation state from boot-phase logs rather than reading a dedicated reconciliation surface. **Post-cycle-112 update: this give-up is more salient than cycle 111 framed it** — under the retrospective's family-level dominance reading (reconciliation asymmetry as dominant family), legibility on F2/F4/F11 is legibility on the dominant-family addressing. The question of whether named-phase legibility is worth +~1000-2600 LOC migration cost over A's interleaved-with-boot legibility is now explicitly Eva's call (see "Central uncertainty: family-level vs pattern-level dominance" section). **Mitigation:** if Eva approves A, Phase 3 measurement of reconciliation-related boot-phase work becomes a load-bearing measurement; if measured to be substantial fraction of cycle budget, revisit C-style reconcile-mode addition retroactively.

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

**Post-cycle-112 stress-test update to "Why not C":** the cycle 112 finding (family-level vs pattern-level dominance — see next section) shifts the A-vs-C trade-off from "C must justify its additions against A's parsimony" toward "C must justify its additions against A's bounded migration cost, given that C's first-class reconciliation is consistent with the retrospective's family-level dominance reading." Reasons #1, #3, and #4 above remain — particularly the conditional-improvement risk and the migration-cost difference. Reason #2 (PR #2877's under-justified-estimate flag for C) remains relevant. **What changes post-cycle-112:** Reason #3's "F4/F11 addressing improvements over A are unproven as load-bearing" weakens, because under the family-level dominance reading reconciliation asymmetry is the dominant family and F2/F4/F11 addressing IS the dominant-family addressing. The remaining A-over-C argument on this criterion is thus the cost-benefit one: is named-phase reconciliation legibility worth +~1500-2000 LOC migration cost over interleaved-with-boot reconciliation that achieves the same outcomes?

## Central uncertainty: family-level vs pattern-level dominance (cycle 112 stress-test)

This section was added in cycle 112 (2026-05-10) as the substantive output of the central-bet stress-test named in the cycle 111 outstanding-work list ("Selection-rationale stress-testing — adversarial re-read of retrospective looking for competing dominant-pattern characterizations"). It surfaces a finding that the cycle 111 selection draft glossed over.

### The finding

The cycle 111 selection draft asserts (Criterion 4 paragraph 2 + Criterion 5 paragraph A): *"the dominant failure pattern is F1 (constraint accretion) compounding F7 (self-management dominance)"*, and uses this to ground A's central-bet defensibility as *"directly supported by the retrospective's identification of F1+F7 as the dominant failure pattern."*

**The retrospective does not identify F1+F7 as the dominant failure pattern.** At lines 161-162, the retrospective explicitly names **reconciliation asymmetry as the dominant family**, with the cleanest single-sentence statement: *"v1 has tools to **create** records and tools to **summarize** records; it lacks tools to **reconcile** records against later events."* At line 959, the v2-design-implications-by-family preamble repeats: *"reconciliation asymmetry as dominant family."*

**Reconciliation asymmetry is the F2/F3/F4/F5†/F11† family** (per the table at retrospective line 138-143). **F1 and F7 are in different families**:
- F1 is in the **defense-accretion family** (F1/F5†/F11†/F12) — alongside F11 dual-membered with reconciliation asymmetry.
- F7 is in the **procedure/review-overhead family** (F6/F7/F9) — disjoint from both reconciliation asymmetry and defense accretion.

The cycle 111 framing "F1 compounding F7" thus crosses two family boundaries (defense-accretion + procedure-overhead) to construct a cross-family "dominant pattern" that the retrospective does not name.

### Two valid readings of "dominance"

The retrospective supports two readings:

| Reading | Claim | What dominance means | Selection implication |
|---|---|---|---|
| **Family-level (per retrospective lines 161-162, 959)** | Reconciliation asymmetry is the dominant family | The structural shape of v1's failure is write-mostly state with no inbound reconciliation tools; F2/F3/F4/F5†/F11† are manifestations | C and B address this family more structurally (named reconciliation phase / dedicated reconciler agent) than A (interleaved-with-boot polling) |
| **Pattern-level prevalence (cycle 2 measurements)** | F1 (constraint accretion) and F7 (self-management dominance) are the most-pervasive and most-measurable individual patterns | F7 has 130+ consecutive cycles of zero schema work; F1 has constraint-stack-on-stack evidence in C3 step | A addresses F1 most directly via Axis 13 medium-harness extraction; A and B address F7 more aggressively than C |

The two readings are not contradictory; they are different abstraction levels. The cycle 111 selection draft picked the pattern-level reading without acknowledging the family-level reading exists in the retrospective.

### Why this matters less than it first appears

**A and C have equivalent reconciliation OUTCOMES.** Reading A's candidate document (lines 36-37, 45) and C's (lines 22, 44-46) confirms:
- Both use cursor-file polling (`state/eva-input-cursor.json`).
- Both have 1-cycle latency for Eva-response detection (~6h at 4 cycles/day cron).
- Both detect post-close mutations via cycle-history append-only files (Axis 4 git-as-substrate).
- Both treat audit-as-peer with cross-repo reading discipline.

The difference between A and C on reconciliation is **structural legibility, not detection mechanism or latency**:
- A's reconciliation is interleaved with boot-phase polling (one of several boot activities; not a named harness phase).
- C's reconciliation is `reconcile-mode` — a named harness phase between boot-mode and work-mode, with bounded runtime budget (≤10min), per-channel structured handlers, typed-deltas to per-component state files.

**A reader of an A-system inferring reconciliation state has to read boot-phase logs; a reader of a C-system reads the named reconcile-mode artifact directly.** The detection works identically; the legibility differs.

### Reframed selection question

The cycle 112 finding does not flip A → C. It reframes the A-vs-C question from:

> **Cycle 111 framing:** Is A's parsimony enough for v1's dominant failure pattern (F1+F7)?

To:

> **Cycle 112 framing:** Reconciliation asymmetry IS the family-level dominant failure pattern. A's interleaved reconciliation and C's first-class reconcile-mode produce equivalent outcomes (same latency, same mechanism). **Is named-phase reconciliation legibility worth +~1500-2000 LOC migration cost over A's interleaved approach?**

This reframing makes the A-vs-C trade-off more honest:
- A's win on cost-of-being-wrong (Criterion 3) is real and quantifiable — A's net-add is ~3600-6200 LOC; C's is ~5200-8200 LOC; the delta is ~1000-2600 LOC.
- C's win on family-level-dominance addressing (Criterion 4 family-level reading + Criterion 5 post-cycle-112 update) is real but its load-bearingness depends on whether legibility-to-Eva-and-audit is judged structurally important.

### Why the recommendation stays as A (post-cycle-112)

The recommendation tentatively stays as Candidate A despite the cycle 112 finding because:

1. **Criterion 1 (schema-work-enabling) and Criterion 3 (cost of being wrong) still favor A** — these are the highest-weighted criteria per the redesign's primary thesis, and the cycle 112 finding does not change them.
2. **A and C have equivalent reconciliation OUTCOMES** — the cycle 112 finding shows A is not actually worse on reconciliation outcomes; it is only worse on reconciliation legibility.
3. **A's central-bet rationale weakens but A's overall criteria-weighted ranking does not flip.** Criterion 5 ordering shifts from A > C to A ≈ C; Criteria 1, 2, 3 still favor A; Criterion 4 ordering becomes contested between two valid readings; Criterion 6 narrowly favors B at unchanged margin.
4. **The cost-benefit framing is now Eva's call.** The cycle 112 finding makes explicit a value judgment that the cycle 111 draft glossed over: is named-phase reconciliation legibility worth +~1500-2000 LOC? This is exactly the kind of trade-off Eva is best positioned to evaluate at the candidate-selection checkpoint.

### What this means for cycle 112+ outstanding work

- The "stress-test the central bet" item from cycle 111's outstanding-work list is now **partially absorbed** by this section. A more thorough stress-test would extend to other load-bearing claims in the selection rationale (e.g., the "B's central bet requires aspirational extension" claim — does B's cluster G framing have any retrospective grounding I'm missing?).
- **The adversarial Copilot feedback dispatch named for cycle 112+** should now also test whether the cycle 112 reframing is itself correct, OR whether there's a third reading of dominance the cycle 112 work missed.
- **Open question for Eva #6** (added below) names this as the central uncertainty Eva resolves.

## Outstanding work before lock-in

This draft is **not** the deliverable. Per `ITERATION-UNTIL-APPROVAL`, the following work is outstanding before the candidate-selection checkpoint should be considered ready for Eva's approval:

1. **Audit critique landing.** Audit cycle 215 (today) named cycle 216 as the substantive Phase 2 critique cycle. Audit critique on the candidate set should land on this draft and any updates produced in response. Cycle 112+ absorption work.

2. **Adversarial Copilot feedback dispatch on the selection.** No Copilot feedback dispatch has been targeted specifically at the selection decision. Dispatching one (or several with different lenses: "argue for B"; "argue for C"; "argue this analysis is missing a candidate not yet authored") is named in the redesign prompt's `ITERATION-UNTIL-APPROVAL` and would test whether the analysis survives external adversarial framing. Cycle 112+ work.

3. **Stress-test the central bet.** ~~The recommendation rests on F1+F7 being the dominant v1 failure pattern. If a careful adversarial re-read of the retrospective surfaces a competing dominant-pattern characterization (e.g., F4+F11 as load-bearing; or cluster-G role-asymmetry as load-bearing), the recommendation must be revisited.~~ **Partially absorbed cycle 112** — see "Central uncertainty: family-level vs pattern-level dominance" section above. Cycle 112 stress-test surfaced that the cycle 111 "F1+F7 dominant" framing was mis-grounded against the retrospective's "reconciliation asymmetry is the dominant family" claim (lines 161-162 + 959). The recommendation does not flip (A still wins on Criteria 1+3) but the rationale is reframed and a sixth Eva question is added. **What remains for cycle 113+:** stress-test of other load-bearing claims (B's cluster-G aspirational extension; C's conditional-improvement risk; the "A and C have equivalent reconciliation outcomes" claim — does Phase 3 measurement support equivalence or expose a difference?).

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

6. **Resolve the family-level vs pattern-level dominance question (cycle 112 finding).** The cycle 112 stress-test surfaced that the cycle 111 draft's "F1+F7 is the dominant failure pattern" framing was mis-grounded against the retrospective's explicit "reconciliation asymmetry is the dominant family" framing (lines 161-162 + 959). The cycle 112 reframing shows A and C have equivalent reconciliation OUTCOMES (same 1-cycle latency, same cursor-polling mechanism); the difference is structural legibility (first-class named phase vs interleaved with boot). **The selection question reduces to: is named-phase reconciliation legibility worth +~1000-2600 LOC migration cost over interleaved reconciliation?** This is a value-judgment where Eva's perspective is the load-bearing input — the orchestrator's analysis names the trade-off but does not pre-empt the resolution. If Eva judges named-phase legibility important: select C. If Eva judges A's bounded migration cost more important and accepts interleaved-with-boot legibility: select A.

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
| 112 (2026-05-10) | **Central-bet stress-test absorbed.** Surfaced that cycle 111's "F1+F7 is the dominant failure pattern" framing was mis-grounded against the retrospective's explicit "reconciliation asymmetry is the dominant family" claim (lines 161-162 + 959). Revised Criterion 4 paragraph (two valid dominance readings; ordering depends on reading). Revised Criterion 5 paragraph (A's central bet weakened from "directly supported" to "consistent with one reading"; ordering downgraded to A ≈ C from cycle 111's A > C). Added "Post-cycle-112 stress-test update" to "Why not C" reframing the A-vs-C trade-off. Added new "Central uncertainty: family-level vs pattern-level dominance" section between "Why not C" and "Outstanding work before lock-in" (~115 lines). Added open question #6 for Eva. **Recommendation stays as A** because Criteria 1+3 (highest-weighted) still favor A and A and C have equivalent reconciliation outcomes, but the central-bet rationale is now contested rather than confidently defended. | Audit cycle 216 critique landing (~ next audit cron, ~20h). Adversarial Copilot feedback dispatch on the post-cycle-112 selection draft (cycle 112 outstanding). Further stress-tests of other load-bearing claims (e.g., "B's central bet requires aspirational extension" — does B's cluster G framing have any retrospective grounding the cycle 112 stress-test missed?). Cycle 103-110 risk-closure threshold table integration (cycle 113+). Per-candidate Phase 3 prototype evidence deepening (cycle 113+). |
