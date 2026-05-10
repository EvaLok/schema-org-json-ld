# Phase 2 candidate selection

**Status:** **DRAFT — cycle 111 (2026-05-10) initial authoring; cycle 112 (2026-05-10) central-bet stress-test on dominance-framing absorbed; cycle 113 (2026-05-10) central-bet stress-test on equivalent-outcomes claim absorbed; cycle 114 (2026-05-10) third central-bet stress-test on conditional-improvement-risk grounding triggered the four-finding flip threshold named cycle 113 — TENTATIVE RECOMMENDATION FLIPPED FROM A TO C; awaiting candidate-selection checkpoint approval per `ITERATION-UNTIL-APPROVAL`.**

**Authoring cycle:** 111 (2026-05-10) initial; 112 (2026-05-10) central-bet stress-test surfacing family-level vs pattern-level dominance tension; 113 (2026-05-10) central-bet stress-test surfacing detection-vs-processing distinction in the "equivalent outcomes" claim and naming the four-finding flip threshold; 114 (2026-05-10) third central-bet stress-test surfacing the cycle 111 conditional-improvement framing as omitting C's own cycle 92 + cycle 96 + cycle 107 direction-validation evidence — fourth load-bearing under-weighted dimension surfaces; flip threshold honored; tentative recommendation flipped from A to C.
**Authored against:** [`2-candidates/README.md`](./2-candidates/README.md) post-cycle-110 state; the three candidate documents [`A-evolved-single-orchestrator.md`](./2-candidates/A-evolved-single-orchestrator.md), [`B-decomposed-multi-role.md`](./2-candidates/B-decomposed-multi-role.md), [`C-hybrid.md`](./2-candidates/C-hybrid.md); the [`2-design-framework.md`](./2-design-framework.md) v1.21+ axes / convergent constraints / failure-mode mapping / P1-P6 evaluation criteria; the cycle 96 PR #2878 sharpening-claims absorption; the cycle 97 PR #2877 tool-surface-feasibility absorption; the cycles 103/105/106/107/108/109/110 risk closures at specification level.

This document is the Phase 2 **selection** artifact named in the redesign prompt's SECTION 8 ("Output: `docs/redesign/2-candidates.md` and `docs/redesign/2-selection.md`"). Per the redesign prompt's CHECKPOINTS, candidate-selection is the second of three hard checkpoints requiring Eva's explicit approval. Per `ITERATION-UNTIL-APPROVAL`, this draft is not the deliverable until Eva approves; iteration continues until then.

## Tentative selection: Candidate C (Hybrid) — flipped from A cycle 114 per the four-finding flip threshold

**Tentative recommendation: select [`C — Hybrid`](./2-candidates/C-hybrid.md) for Phase 3 prototype work.**

This recommendation is **provisional**. It rests on the cumulative analysis across cycles 111-114 (see "Why the recommendation flipped to C (post-cycle-114)" section below). It can be overridden by:

- Eva's direct judgment at the checkpoint (Eva is authoritative; this draft is input). The most plausible override paths are (a) flip back to A if migration-cost / Criteria 1+3 weight is judged to outrank Criteria 4+5 cumulative findings; (b) flip to B if the schema-work proof-domain is judged to need multi-role decomposition for parallelism; (c) direct an additional candidate not yet authored.
- New evidence from audit critique not yet landed (audit cycle 215 named cycle 216 as the substantive Phase 2 critique cycle; landing in ~next audit cron, ~8h from cycle 114). If audit critique shifts the analysis materially, this draft updates.
- Copilot adversarial-feedback dispatch landings (the cycle 112 dispatch body file remains ready for re-attempt once question-for-eva [#2903](https://github.com/EvaLok/schema-org-json-ld/issues/2903) resolves the runtime allowlist + PAT scope friction).
- Phase 3 prototype evidence on a smallest-load-bearing crate (cycles 93-94 measured `v2-tool-registry` 231 LOC + `v2-cycle-history-append` 268 LOC; further measurements — especially of C's `reconcile-mode` if Eva pre-approves — may sharpen the migration-cost vs structural-improvement trade-off).

The recommendation is **not** that Candidate C is "best in all dimensions." It is that, given the cumulative findings across cycles 111-114 — the cycle 111 selection-rationale rested on a chain of A-favoring framings, each of which was found mis-grounded under successive rounds of stress-testing — and given the cycle 113 four-finding flip threshold being honored cycle 114 (a falsifiable pre-committed criterion that interrupts `progressive-rationale-weakening-without-flip`), Candidate C is the orchestrator's tentative recommendation. **What C gives up vs A** (Criteria 1 schema-work-enabling and Criterion 3 cost-of-being-wrong both still favor A on migration-cost dimensions) is preserved as legitimate Eva-override territory; the orchestrator's analysis names the trade-off and does not pre-empt Eva's decision.

**Concise pre-flip vs post-flip summary:**

| Aspect | Cycles 111-113 (recommendation: A) | Cycle 114 (recommendation: C) |
|---|---|---|
| Criterion 1 (schema-work-enabling) ordering | A > C > B | A > C > B (unchanged; A still favored on migration-cost dimension) |
| Criterion 2 (CORE-DESIGN-PRINCIPLE) ordering | A ≈ C > B | A ≈ C > B (unchanged) |
| Criterion 3 (cost-of-being-wrong) ordering | A > C > B | A > C > B (unchanged; A still favored on rollback-surface dimension) |
| Criterion 4 (failure-mode-addressing) ordering | C > B > A under family-level reading; B > C > A under pattern-level reading | C > B > A (family-level reading now load-bearing per cycle 112 + 114 cumulative; pattern-level reading itself shown to depend on mis-grounded F1+F7 framing) |
| Criterion 5 (central-bet defensibility) ordering | A ≈ C > B (cycle 113) | C > A > B (cycle 114; A's central-bet rationale weakened across 4 cycles; C's central bet validated by cycle 92 + cycle 96 absorption + cycle 107 closure) |
| Criterion 6 (audit-as-peer P6) ordering | B > C > A (narrow margin) | B > C > A (unchanged; narrow margin) |
| Net cumulative weight | Criteria 1+2+3 favor A; Criteria 4+5+6 contested or favor C/B | Criteria 1+2+3 favor A on bounded-migration dimensions; Criteria 4+5 favor C on cumulative-finding dimensions; Criterion 6 favors B narrowly. **The shift in Criterion 5 from A ≈ C to C > A is the load-bearing change** — A's central bet has been progressively shown mis-grounded across 4 cycles of stress-testing, while C's central bet has accumulated direction-validating evidence. |

## Cycle 114 stress-test: conditional-improvement framing weakened, four-finding flip threshold honored

This section was added in cycle 114 (2026-05-10) as the substantive output of the third A-vs-C central-bet stress-test in the cycle 111-113-named pattern. It documents the cycle 114 finding, the application of the cycle 113 four-finding flip threshold, and the reasoning for honoring the threshold (rather than reasoning around it).

### The cycle 114 stress-test target

Cycle 113 named four candidates for cycle 114+ stress-tests: (1) B's cluster G aspirational extension grounding; (2) C's conditional-improvement risk grounding; (3) B's 3-5× migration cost methodology; (4) the 6-criterion ordering structure assumptions. Cycle 114 selected target #2 — **C's conditional-improvement risk grounding** — as the most directly A-vs-C-relevant of the four, and the one whose finding would either trigger the four-finding flip threshold or stabilize the recommendation at A.

The cycle 111 "Why not C" Reason #1 framing reads:

> *"Conditional-improvement risk. C's own document names this risk: 'if the additions don't carry their weight, C is just A with more migration surface.' The 3 additional sub-shapes (plan-lifecycle + reconcile-mode + per-mode budgets) are at most 25-35% per-cycle decision overhead reduction vs A's 30-40% — C pays additional cost for additional structural depth in F4/F11."*

The stress-test question: is this framing grounded in C's document text, or is it a cycle 111 selective citation?

### The finding

The cycle 111 framing cites C-hybrid.md line 73 (C's self-acknowledged risk language) and line 116 (P3 NOTE flagging the "central uncertainty") — both genuine references. **But the framing omits C-hybrid.md lines 222-228 (cycle 92 Aggregate central-bet validation) and lines 349-372 (Risk 2 status post-cycle-107 closure), which directly address the same conditional and resolve the direction component:**

- **C-hybrid.md lines 222-228 (cycle 92):** *"C's central bet — that reconcile-mode + plans-as-artifacts carry their weight — is **structurally validated by per-failure-mode enumeration**: F2: ~4pp reliability improvement + higher legibility (real but small). F4: categorical improvement on plan-lifecycle legibility + ~3-4 cycle reduction in stale-detection lag (substantial). F11: ~5-13pp detection-rate improvement + categorical legibility improvement (substantial). **Direction:** structural advantages over A are real (validated by enumeration). The 3 additional sub-shapes adopted by C beyond A... **carry their weight on F4 and F11**; F2 improvement is more marginal."*

- **C-hybrid.md lines 349-372 (cycle 107):** Risk 2 status post-cycle-107 closes the discipline-conditional risk at specification level — *"Direction continues to hold by construction... Stale-active-plan promotion lag ≤3 cycles steady-state... Mitigation specification is now concrete enough for Phase 3 prototype validation."* The closure separates direction (validated) from magnitude (Phase 3 measurement target) with falsifiable thresholds.

- **Cycle 96 PR #2878 absorption (C-hybrid.md line 191):** the F2 reliability claim was qualified as *"ambiguous-without-prototype"* — direction holds, magnitude unmeasured. **F2 was qualified; F4 and F11 were not.** F4 + F11 direction-validation survived adversarial review.

The cycle 111 conditional-improvement framing reads as if the **full** conditional ("do the additions carry their weight at all") is open. **In fact the direction component is resolved** by cycle 92 enumeration, confirmed by cycle 96 PR #2878 absorption (F4/F11 not qualified), and bounded for Phase 3 by cycle 107's closure with falsifiable thresholds. **The remaining open conditional is magnitude (Phase 3 prototype measurement) — a smaller conditional than the cycle 111 framing suggests.**

The cycle 111 evidence ("at most 25-35% per-cycle decision overhead reduction vs A's 30-40%") substitutes a **P3 metric** (decision overhead) for the **F4/F11 detection legibility evidence** the conditional was about. P3 is one criterion among several; the conditional-improvement risk is specifically about F4/F11 carrying their weight, not P3 decision-overhead percentages.

### The chain of motivated reasoning across cycles 111-114

The cycle 111 selection draft's case for A rests on a chain of framings, each of which has now been found mis-grounded:

1. **Cycle 111 Criterion 5 framing of A's central bet** as *"directly supported by the retrospective's identification of F1+F7 as the dominant failure pattern"* — **cycle 112 found this mis-grounded** against the retrospective's explicit "reconciliation asymmetry as the dominant family" claim (lines 161-162 + 959). The retrospective does not name F1+F7 as the dominant pattern at any reading; the cycle 111 framing crosses two family boundaries (defense-accretion + procedure-overhead) to construct a cross-family pattern the retrospective does not state.

2. **Cycle 112 reframing of A-vs-C as "legibility only"** — **cycle 113 found this under-weighted** on three processing dimensions: per-mode runtime budget allocation (C explicit ≤10min reconcile-mode budget vs A shared boot-phase budget), per-channel structured handlers (C typed-deltas vs A procedurally-loose), typed-delta output discipline (C single auditable stream vs A scattered). Cycle 113 named these as detection-vs-processing distinction.

3. **Cycle 111 "Why not C" Reason #1 (conditional-improvement risk)** — **cycle 114 finds this omits C's own cycle 92 + cycle 96 + cycle 107 evidence** that resolves the direction component of the conditional. The cycle 111 framing presents the conditional as fully open when direction is in fact validated; the open conditional is magnitude (Phase 3 work).

4. **Cycle 111 "Why not C" Reason #3 ("F4/F11 addressing improvements over A are unproven as load-bearing")** — depends on F1+F7 dominance (cycle 112 finding refutes); collapses under the family-level reading (F4 + F11 are members of the reconciliation asymmetry family the retrospective names as dominant).

**The four findings are structurally distinct and substrate-content-distinct:** cycle 112 found mis-grounding against retrospective text; cycle 113 found under-weighting in cycle 112's reframing; cycle 114 found omission of C's own validation evidence; the cycle 113 + 114 findings together also dismantle Reason #3's "unproven as load-bearing" framing. Each round of stress-testing surfaces a different category of cycle 111 framing weakness while the recommendation in pre-cycle-114 drafts continued to point at A.

### The four-finding flip threshold being honored

Cycle 113's selection-draft "Honest cumulative observation" named the threshold:

> **"If a fourth round of stress-testing (cycle 114+) surfaces a fourth load-bearing under-weighted dimension favoring C over A, the recommendation should be honestly flipped to C."**

Cycle 114 surfaces such a dimension (the conditional-improvement framing's omission of C's direction-validation evidence; structurally connected to and amplified by the family-level reading from cycle 112). **The threshold is honored: the tentative recommendation flips from A to C.**

The flip is the orchestrator's tentative recommendation update; **it does not pre-empt Eva's checkpoint approval**. Eva can override (back to A on migration-cost / Criteria 1+3 weighting; to B on multi-role-decomposition; to a not-yet-authored candidate). The flip rests on Criteria 4 + 5 cumulative cycle 111-114 findings outweighing Criteria 1 + 2 + 3 + 6 in the orchestrator's tentative analysis; Eva resolves whether Criteria 4 + 5 cumulative weight does in fact outrank the Criteria 1 + 3 cumulative weight.

### Why the flip is the honest move (not reasoning-around the threshold)

The cycle 113 _notes named the failure mode explicitly:

> *"`pre-committed-flip-threshold` candidate-pattern... applicability test is whether cycle 114+ honors the threshold or reasons-around it; if reasoned-around, the threshold reveals as protective fiction; if honored, it is a load-bearing discipline."*

Reasoning-around the threshold would look like: "the cycle 114 finding is connected to cycle 112's finding so it doesn't count as a fourth dimension," or "the conditional-improvement framing was acknowledging a real risk so the omission isn't material," or "the cycle 92 validation is internal to C so it doesn't count as evidence." Each of these is plausibly arguable. None of them changes that:

- The threshold criterion was named in advance specifically to interrupt motivated-reasoning patterns.
- The cycle 114 finding meets the criterion (substrate-content-distinct from cycles 112 + 113; favoring C; load-bearing for the cycle 111 case for A).
- If the threshold is set aside whenever a finding is "merely connected" to prior findings or "merely about citation patterns," the threshold provides no actual interrupt against motivated reasoning.

**Honoring the threshold is the load-bearing structural action.** The flip is the orchestrator demonstrating that pre-committed falsifiable criteria are real disciplines, not protective fictions.

### What the flip preserves and what it changes

**Preserved:** the cumulative reasoning chain across cycles 111-114 stays in the document (the "Why not C" section retains the cycle 111 reasoning as historical record; the cycle 112 + 113 + 114 updates document the progressive findings). Eva sees the full reasoning trajectory and the cycle 114 flip rationale; the flip does not bury the prior recommendation. The "Why not B" section is unchanged (cycle 114 produced no B-side findings).

**Changed:** the tentative selection (A → C); the Criterion 5 ordering (A ≈ C > B → C > A > B); the side-by-side comparison "Recommendation" framing (orchestrator's tentative recommendation now points at C); the "Why the recommendation stays as A" section is renamed "Why the recommendation flipped to C (post-cycle-114)" with substantively rewritten body; outstanding work item #3 absorbs cycle 114; open questions #1 + #2 invert (Q1 was "Approve A?" now "Approve C?"; Q2 was "Override to C?" now "Override to A?").

### Cycle 114 outcome for the candidate-pattern tracking

- **`pre-committed-flip-threshold`** candidate-pattern (NOVEL@1 cycle 113) → **promoted to TESTED@1-by-being-honored** (cycle 114 demonstrates the threshold is a load-bearing discipline; the test was naming the criterion in advance and then honoring it when triggered, not reasoning around it).
- **`progressive-rationale-weakening-without-flip`** candidate-pattern (NOVEL@1 cycle 113) → **interrupted by the flip threshold being honored** (cycle 114 breaks the pattern at NOVEL@1 + interrupted; the pattern would have HARDENED at NOVEL@2 if cycle 114 had reasoned around the threshold).
- **`central-bet-stress-test-finds-mis-grounding`** candidate-pattern (TESTED@2 cycle 113) → **promoted to HARDENED-at-3** (cycle 112 retrospective mis-grounding + cycle 113 under-weighted dimensions + cycle 114 omitted-validation-evidence; three substrate-content-distinct findings of the same shape).
- **`discretionary-departure-from-forward-going-commitment`** candidate-pattern (HARDENED-at-3 cycle 113) → **HARDENED-at-4** (cycle 110 named cycle 111+ rotation; cycles 111, 112, 113, 114 all departed; cycle 114's substrate is the threshold-honoring action, structurally distinct from per-candidate sharpening).

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

**Ordering on this criterion (post-cycle-114): C > A > B** (further downgrade from cycle 113's "A ≈ C > B"). The cycle 114 stress-test surfaces that C's central bet is direction-validated by cycle 92 enumeration ("**carry their weight on F4 and F11**"; F2 marginal), the F4/F11 portions survived cycle 96 PR #2878 absorption (only F2 was qualified as "ambiguous-without-prototype"), and Risk 2 is closed at specification level cycle 107 with falsifiable thresholds. **C's central bet has accumulated direction-validating evidence across 4 cycles of internal review.** A's central bet (cycle 111 framing as "directly supported by F1+F7 dominance") was found mis-grounded against the retrospective's "reconciliation asymmetry as dominant family" claim cycle 112; the cycle 113 + 114 findings further weakened the case for A. **The cumulative pattern across cycles 111-114 is: A's central-bet rationale rests on framings that successively fail adversarial review; C's central-bet rationale survives those same reviews.** Parsimony (the remaining A-over-C argument) is not the same as defensibility against the retrospective's framing; this is the cycle 113 observation extended to cycle 114. B's bet (cluster-G role-asymmetry as load-bearing for v2) still requires aspirational extension and remains the lowest-defensibility position.

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
| **Central bet defensibility** | **weakened progressively cycles 111→112→113→114** (cycle 111 "F1+F7 dominant" framing mis-aligned with retrospective lines 161-162 + 959; cycle 113 found "equivalent outcomes" claim under-weighted on processing dimensions; cycle 114 found "Why not C" Reason #1 omits C's cycle 92 + 96 + 107 direction-validation evidence — case for A rests on chain of motivated reasoning across 4 cycles) | **upgraded progressively cycles 112→114** (cycle 112: consistent with family-level dominance reading; cycle 114: direction-validated by cycle 92 enumeration + cycle 96 PR #2878 absorption F4/F11 not qualified + cycle 107 Risk 2 closure at specification level — direction resolved; magnitude is Phase 3 work) | medium (bet on cluster G role-asymmetry being load-bearing for v2 — no retrospective grounding at any reading) |
| **F1+F7 (dominant v1 failure pattern) addressing** | strong (Axis 13 harness extraction) | strong (same as A) + plan-lifecycle CI | strong (per-role specialization) |
| **F4 (frozen-artifact lifecycle) addressing** | weak (no plan-lifecycle) | strong (3-state plan-lifecycle) | strong (3-state plan-lifecycle + branching checkpoints) |
| **F11 (post-close mutations) addressing** | medium (boot-phase polling) | strong (reconcile-mode first-class) | strong (reconciler agent first-class) |
| **A-vs-C reconciliation distinction (cycle 112 → cycle 113 evolution)** | detection equivalent to C; processing differs on runtime-budget + per-channel-extensibility + audit-readability (cycle 113 sharpening of cycle 112 "legibility-only" framing) | first-class reconcile-mode = dedicated runtime budget + per-event handlers + typed-deltas; structurally distinct from A on processing | reconciler agent = clean-context separation; structurally distinct from both A and C |

## What Candidate A gives up

Honesty per the redesign prompt's SECTION 8 directive ("you pick one with rationale, naming what you give up"):

1. **No dedicated reconciler agent / no first-class reconcile-mode.** A's Axis 12 reconciliation interleaves with primary work in boot phase. If the volume of inbound channels grows substantially (Eva responses + audit posts + dispatch PR-merges + future channels), the boot phase becomes the bottleneck. C's reconcile-mode is the structural alternative; B's reconciler agent is the more aggressive alternative. **Post-cycle-113 sharpening:** the boot-phase-bottleneck risk is now characterized on three dimensions, not one — runtime budget allocation under burst load, per-channel extensibility for new inbound channels, and audit-readability of reconciliation state. **Mitigation:** Phase 3 measurement of reconcile-equivalent boot-phase work plus burst-load characterization (deliberate stress with multiple Eva responses + audit posts + dispatch returns same cycle); if measured to be substantial fraction of cycle budget OR if extensibility friction surfaces during channel additions, revisit C-style reconcile-mode addition retroactively.

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

**Post-cycle-114 stress-test update to "Why not C" (the section is now historical record post-flip):** the cycle 114 stress-test on Reason #1 (conditional-improvement risk) surfaces that the cycle 111 framing **omits C-hybrid.md's own cycle 92 + cycle 96 + cycle 107 evidence** that resolves the direction component of the conditional ("**carry their weight on F4 and F11**" per cycle 92 line 228; F4 + F11 survived cycle 96 PR #2878 absorption while only F2 was qualified; Risk 2 closure at cycle 107 specification level with falsifiable thresholds). The cycle 111 framing presents the conditional as fully open when direction is in fact validated; the open conditional is magnitude (Phase 3 work). Reason #1 is therefore weakened to "magnitude is Phase 3 work; direction is resolved." Reason #3 was already weakened post-cycle-112; cycle 114 confirms Reason #3's "F4/F11 unproven as load-bearing" framing depends on the cycle-112-refuted F1+F7-dominance basis. Reason #2 (PR #2877's under-justified-estimate flag) and Reason #4 (per-mode runtime budgets add complexity at unproven value — partially weakened by cycle 113's per-mode-budget-as-runtime-allocation finding) remain as the residual A-over-C arguments on the cycle 111 framing's terms. **The cumulative effect of cycles 112 + 113 + 114 stress-tests is that the cycle 111 case for A no longer holds at face value; the four-finding flip threshold is honored cycle 114; the recommendation flips to C.** This section is preserved as historical record showing the original cycle 111 reasoning and the progressive cycle 112-114 dismantling; the active recommendation lives in "Why the recommendation flipped to C (post-cycle-114)."

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

### Why this matters less than it first appears (with cycle 113 qualification)

**A and C have equivalent reconciliation DETECTION outcomes.** Reading A's candidate document (lines 36-37, 45) and C's (lines 22, 44-46) confirms:
- Both use cursor-file polling (`state/eva-input-cursor.json`).
- Both have 1-cycle latency for Eva-response detection (~6h at 4 cycles/day cron).
- Both detect post-close mutations via cycle-history append-only files (Axis 4 git-as-substrate).
- Both treat audit-as-peer with cross-repo reading discipline.

The detection mechanism and detection latency are equivalent.

**Cycle 113 finding: the "equivalent outcomes" claim covers DETECTION but glosses over PROCESSING.** A more careful reading of A's lines 36-37+45 and C's lines 20+22+44-46 surfaces three dimensions where A and C diverge that the cycle 112 reframing characterized as "legibility only":

1. **Per-mode runtime budget allocation.** C explicitly declares per-mode runtime budgets at Axis 9 (boot ≤5min / reconcile ≤10min / work ≤50min / close ≤10min); reconcile-mode has dedicated runtime independent of other boot work. A has no reconciliation-specific budget — reconciliation work shares boot-phase budget with state-load + cursor-advance + standing-directive check + gardening-sweep-pre-cycle. Under burst load (multiple Eva responses + audit posts queued same cycle), A's reconciliation may be truncated or its work spills into work-phase; C's reconcile-mode is bounded but dedicated. **This is not legibility — it is runtime allocation.**

2. **Per-channel structured handlers.** C's Axis 12 explicitly names *"Per-event handlers per channel; reconciler-mode emits typed-deltas to per-component state files"* (line 22). A's Axis 12 describes *"input-from-eva pull at session-start with cursor advance; question-for-eva-response check via existing `check-eva-responses` (or replacement)"* (line 36) — procedurally less structured. Adding a new inbound channel (e.g., a third-party orchestrator with a new label, a webhook-driven event source) to A's boot-phase polling requires a code change that interleaves with existing boot work; C's reconcile-mode adds a per-channel handler with a uniform shape. **This affects extensibility AND consistency-of-handling across channels.**

3. **Typed-delta output discipline.** C's reconcile-mode emits typed-deltas to per-component state files — a single auditable output stream per cycle. A's reconciliation outputs scatter across boot-phase activities (cursor advances, in-place state updates, journal appends). For audit-as-peer's ability to read reconciliation state across cycles, C's typed-deltas are more readable than A's scattered boot-phase outputs. **This is partly legibility but also partly auditability — they are not the same.**

The structural difference between A and C is therefore **wider than legibility** — it includes runtime budget allocation, extensibility of channel-handling code path, and audit-readability of reconciliation state. Cycle 112's characterization of the difference as "legibility only" was under-weighted; cycle 113 surfaces three dimensions cycle 112 did not name.

**Cycle 113 honest qualification:** detection latency genuinely is equivalent (1 cycle, set by cron cadence). Processing characteristics differ. The cycle 112 reframing of the A-vs-C trade-off as "named-phase legibility worth +~1000-2600 LOC" should now read "named-phase legibility + runtime budget allocation + per-channel extensibility + audit-readability worth +~1000-2600 LOC." The recommendation does not flip on this finding alone (Criteria 1+3 still favor A, the under-weighted dimensions are real but bounded), but the trade-off characterization is more honest.

**A reader of an A-system inferring reconciliation state has to read boot-phase logs; a reader of a C-system reads the named reconcile-mode artifact directly.** The detection works identically; the processing structure (budget, extensibility, output discipline) differs.

### Reframed selection question

The cycle 112 finding does not flip A → C. It reframes the A-vs-C question from:

> **Cycle 111 framing:** Is A's parsimony enough for v1's dominant failure pattern (F1+F7)?

To:

> **Cycle 112 framing:** Reconciliation asymmetry IS the family-level dominant failure pattern. A's interleaved reconciliation and C's first-class reconcile-mode produce equivalent outcomes (same latency, same mechanism). **Is named-phase reconciliation legibility worth +~1000-2600 LOC migration cost over A's interleaved approach?**

The cycle 113 stress-test further reframes (qualifying the cycle 112 framing):

> **Cycle 113 framing:** Reconciliation asymmetry IS the family-level dominant failure pattern. A's interleaved reconciliation and C's first-class reconcile-mode produce equivalent **detection** outcomes (same latency, same mechanism). The **processing** characteristics differ on three dimensions: runtime budget allocation, per-channel extensibility, audit-readability of reconciliation state. **Is named-phase reconciliation processing structure (legibility + dedicated budget + extensibility + auditability) worth +~1000-2600 LOC migration cost over A's interleaved approach?**

This further reframing makes the A-vs-C trade-off more complete:
- A's win on cost-of-being-wrong (Criterion 3) is real and quantifiable — A's net-add is ~3600-6200 LOC; C's is ~5200-8200 LOC; the delta is ~1000-2600 LOC.
- C's win on family-level-dominance addressing (Criterion 4 family-level reading + Criterion 5 post-cycle-112 update) is real and now (post-cycle-113) more structurally distinct from A than cycle 112 framed — the difference is wider than legibility, including runtime budget allocation + extensibility + auditability.
- The load-bearingness of C's win still depends on whether the structural characteristics are judged Eva-load-bearing — but the question is no longer "legibility yes/no" but "structural-processing-discipline yes/no."

### Why the recommendation flipped to C (post-cycle-114)

The recommendation tentatively flipped from A to C cycle 114 because the cycle 113 four-finding flip threshold was triggered by the cycle 114 stress-test on the conditional-improvement framing:

1. **The four-finding flip threshold was a pre-committed falsifiable criterion.** Cycle 113's "Honest cumulative observation" named: *"if a fourth round of stress-testing (cycle 114+) surfaces a fourth load-bearing under-weighted dimension favoring C over A, the recommendation should be honestly flipped to C."* Cycle 114 surfaces such a dimension (the cycle 111 conditional-improvement framing omits C-hybrid.md's own cycle 92 + cycle 96 + cycle 107 direction-validation evidence). The threshold is honored.

2. **The cumulative findings across cycles 111-114 dismantle the cycle 111 case for A.** Cycle 111 framed A's central bet as *"directly supported by the retrospective's identification of F1+F7 as the dominant failure pattern"*; cycle 112 found this mis-grounded against the retrospective's "reconciliation asymmetry as dominant family" claim. Cycle 112 reframed A-vs-C as "legibility only"; cycle 113 found this under-weighted on three processing dimensions (runtime budget, per-channel extensibility, audit-readability). Cycle 113 maintained the recommendation by citing "equivalent reconciliation outcomes"; cycle 114 found the supporting cycle 111 "Why not C" Reason #1 (conditional-improvement risk) omits C's direction-validation evidence. **Each cycle's finding is structurally distinct and substrate-content-distinct from prior findings; the four findings together are not piling on; they are a load-bearing pattern.**

3. **C's central-bet rationale has accumulated direction-validating evidence across 4 cycles of internal review while A's rationale has progressively failed adversarial review.** C-hybrid.md cycle 92 enumeration (lines 222-228): "carry their weight on F4 and F11; F2 marginal." Cycle 96 PR #2878 absorption: F4 + F11 not qualified (only F2 qualified as ambiguous-without-prototype). Cycle 107 Risk 2 closure: discipline-conditional risk closed at specification level with falsifiable thresholds. **C's central bet is direction-validated; magnitude is Phase 3 work.** A's central bet (cycle 111 framing) was never grounded in the retrospective at the family level the retrospective itself names; A's case for v1's "structural cleanup suffices" rests on a pattern-level dominance reading the retrospective does not prefer.

4. **What still favors A: Criteria 1 (schema-work-enabling) and 3 (cost-of-being-wrong) on migration-cost dimensions.** A's bounded migration cost (~3600-6200 LOC PR #2877 revised) means earlier exit from redesign and resumption of schema work; C's higher migration cost (~5200-8200 LOC) delays this by an estimated 1-2 cycles. A's smaller migration surface is easier to roll back if v2 fails. **These are real costs of selecting C.** The flip rests on the orchestrator's tentative analysis that Criterion 5 (central-bet defensibility) cumulative shift to C > A, combined with Criterion 4 (failure-mode addressing depth) cumulative shift to C > B > A under the family-level reading, outweighs Criteria 1 + 3 on the migration-cost dimension. **Eva resolves whether this weighting is correct.**

5. **Honoring the threshold is itself a load-bearing structural action.** Cycle 113 named the `pre-committed-flip-threshold` candidate-pattern with the applicability test: *"if reasoned-around, the threshold reveals as protective fiction; if honored, it is a load-bearing discipline."* Reasoning around the threshold (citing cycle 114's connection to prior cycles, or arguing the finding is "merely about citation patterns") would falsify the threshold as a discipline. **The flip demonstrates that pre-committed falsifiable criteria are real disciplines, not protective fictions** — a v2-design observation worth preserving (analogous to audit's commitment-thread discipline named cycle 215 audit-side, structurally equivalent at the cross-candidate selection level).

**Honest cumulative observation across cycles 111-114:** A's case has been progressively dismantled across 4 cycles of stress-testing while C's case has accumulated direction-validating evidence. The flip is consistent with the cumulative findings. **The flip is the orchestrator's tentative recommendation update; it does not pre-empt Eva's checkpoint approval.** Eva can override (back to A on migration-cost weighting; to B on multi-role decomposition; to a not-yet-authored candidate); the override pathway is preserved in the open questions section.

**What cycle 115+ should examine** (to test whether the flip itself is motivated reasoning rather than load-bearing): (a) does the cycle 114 finding survive adversarial review (audit cycle 216 critique landing; Copilot adversarial-feedback dispatch when [#2903](https://github.com/EvaLok/schema-org-json-ld/issues/2903) resolves)? (b) is the C > A on Criterion 5 ordering robust under stress-tests of C's central-bet? (c) does the +~1500-2000 LOC migration delta materially affect Phase 3 timeline given the cycle 113 absorption of PR #2877 revised LOC range? (d) are there fourth-or-fifth-round stress-tests of the cycle 114 finding itself that should surface? **The flip is not a stopping point; iteration continues per `ITERATION-UNTIL-APPROVAL`.**

### What this means for cycle 112+ outstanding work

- The "stress-test the central bet" item from cycle 111's outstanding-work list is now **partially absorbed** by this section. A more thorough stress-test would extend to other load-bearing claims in the selection rationale (e.g., the "B's central bet requires aspirational extension" claim — does B's cluster G framing have any retrospective grounding I'm missing?).
- **The adversarial Copilot feedback dispatch named for cycle 112+** should now also test whether the cycle 112 reframing is itself correct, OR whether there's a third reading of dominance the cycle 112 work missed.
- **Open question for Eva #6** (added below) names this as the central uncertainty Eva resolves.

## Outstanding work before lock-in

This draft is **not** the deliverable. Per `ITERATION-UNTIL-APPROVAL`, the following work is outstanding before the candidate-selection checkpoint should be considered ready for Eva's approval:

1. **Audit critique landing.** Audit cycle 215 (today) named cycle 216 as the substantive Phase 2 critique cycle. Audit critique on the candidate set should land on this draft and any updates produced in response. Cycle 112+ absorption work.

2. **Adversarial Copilot feedback dispatch on the selection.** No Copilot feedback dispatch has been targeted specifically at the selection decision. Dispatching one (or several with different lenses: "argue for B"; "argue for C"; "argue this analysis is missing a candidate not yet authored") is named in the redesign prompt's `ITERATION-UNTIL-APPROVAL` and would test whether the analysis survives external adversarial framing. Cycle 112+ work.

3. **Stress-test the central bet.** ~~The recommendation rests on F1+F7 being the dominant v1 failure pattern. If a careful adversarial re-read of the retrospective surfaces a competing dominant-pattern characterization (e.g., F4+F11 as load-bearing; or cluster-G role-asymmetry as load-bearing), the recommendation must be revisited.~~ **Substantially absorbed cycles 112 + 113 + 114; recommendation flipped cycle 114** — see "Cycle 114 stress-test: conditional-improvement framing weakened, four-finding flip threshold honored" section above + "Central uncertainty: family-level vs pattern-level dominance" section. Cycle 112 stress-test found cycle 111 "F1+F7 dominant" framing mis-grounded. Cycle 113 stress-test found cycle 112 "equivalent reconciliation outcomes" framing under-weighted on processing dimensions. Cycle 114 stress-test found cycle 111 "Why not C" Reason #1 (conditional-improvement risk) omits C's own cycle 92 + cycle 96 + cycle 107 direction-validation evidence; the cycle 113 four-finding flip threshold is honored; **recommendation flipped from A to C cycle 114**. **What remains for cycle 115+:** (a) stress-test the cycle 114 finding and the flip rationale itself for motivated reasoning (meta-reflexive stress-test); (b) B-side stress-tests (B's cluster-G aspirational extension grounding; B's 3-5× migration cost methodology defensibility against PR #2877's lens-5 critique); (c) the 6-criterion ordering structure assumptions; (d) audit-cycle-216 critique on the post-cycle-114 selection draft if landing surfaces concerns.

4. **Phase 3 prototype effort on a smallest-load-bearing crate from each candidate.** The redesign prompt's SECTION 8 names this as bounded preview: 1-2 of A's smallest Rust crates as proof-of-concept for migration-cost validation; same for C's `reconcile-mode` if Eva pre-approves; B's smallest crate is harder to identify because B's smallest infra is `several thousand LOC each` per B's own document. Cycle 93-94 already measured A's `tool-registry` (231 LOC) and `cycle-history-append` (268 LOC); extending to A's `boot-phase` and `wiki-search` would deepen A-side evidence; equivalent measurement for C's `reconcile-mode` would inform the A vs C comparison.

5. **Cycle 103-110 risk-closure substrate absorption.** The seven specification-level risk closures specified pre-agreed falsifiable thresholds for Phase 3 measurement. The selection rationale above does not yet enumerate these as part of "what Phase 3 needs to validate." A more complete selection draft would include the threshold table, organized by candidate. Cycle 112+ work.

6. **Open questions for Eva** (see next section). Some questions are genuinely Eva-only judgment calls; the answers shape the selection.

## Open questions for Eva at the candidate-selection checkpoint

These questions are explicitly Eva-only per `EVA-DEFAULT-AUTONOMY` (scope authorization, infrastructure access, the candidate-selection checkpoint itself). The orchestrator's analysis does not pre-empt them:

1. **Approve Candidate C for Phase 3 prototype work?** This is the affirmative question (post-cycle-114 flip). If yes: Phase 3 begins with C as the prototype target. The orchestrator's tentative recommendation flipped from A to C cycle 114 per the cycle 113 four-finding flip threshold being honored; the rationale rests on the cumulative findings across cycles 111-114 dismantling the cycle 111 case for A while C's central bet has accumulated direction-validating evidence (cycle 92 + cycle 96 absorption + cycle 107 closure).

2. **Override and pick Candidate A if migration cost / Criteria 1+3 weight outranks Criteria 4+5 cumulative findings?** This is the most plausible override (post-cycle-114 inversion of cycle 111-113 Q2). The orchestrator's analysis names that Criteria 1 (schema-work-enabling) and 3 (cost-of-being-wrong) still favor A on migration-cost dimensions (~3600-6200 LOC vs ~5200-8200 LOC PR #2877 revised), but judges that Criteria 4 (failure-mode-addressing depth under family-level reading) and 5 (central-bet defensibility per cumulative cycles 111-114 findings) cumulatively outrank the migration-cost dimensions. **Eva resolves whether this weighting is correct.** Selecting A means accepting the cumulative cycle 112-114 findings as not-load-bearing-enough to outrank migration cost; selecting C means accepting cycle 114's flip rationale.

3. **Pick Candidate B if the schema-work proof-domain genuinely needs multi-role decomposition for parallelism?** This is the maximally-aggressive override. The orchestrator's analysis names this as not-currently-evidenced, but Eva may have forward-looking judgment about the schema-work scope that the orchestrator does not.

4. **Direct an additional candidate not yet authored?** The redesign prompt's SECTION 8 names "Required: at least 2 distinct design candidates, ideally 3" — three are authored, but Eva may judge the candidate set as not spanning the right dimensions.

5. **Extend the iteration window before approval?** The orchestrator's default behavior is to continue iterating per `ITERATION-UNTIL-APPROVAL`. Eva can name a specific iteration target (audit critique landing, Copilot feedback dispatch landing, additional Phase 3 prototype evidence) before the approval is given.

6. **Resolve the family-level vs pattern-level dominance question (cycle 112 finding), the processing-structure trade-off (cycle 113 qualification), and the conditional-improvement framing's evidentiary basis (cycle 114 finding).** The cycle 112 stress-test surfaced that the cycle 111 draft's "F1+F7 is the dominant failure pattern" framing was mis-grounded against the retrospective's explicit "reconciliation asymmetry is the dominant family" framing (lines 161-162 + 959). The cycle 113 stress-test qualified the cycle 112 "equivalent reconciliation outcomes" claim: detection outcomes are equivalent (1-cycle latency, cursor-polling); processing characteristics differ on three dimensions (runtime budget allocation, per-channel structured handlers, typed-delta output discipline). The cycle 114 stress-test found the cycle 111 "Why not C" Reason #1 (conditional-improvement risk) omits C-hybrid.md's own cycle 92 + cycle 96 + cycle 107 direction-validation evidence — direction is resolved (F4 + F11 carry their weight per cycle 92; survived cycle 96 PR #2878 absorption; Risk 2 closed at specification level cycle 107); the open conditional is magnitude (Phase 3 work). **The selection question reduces to: is named-phase reconciliation processing structure (legibility + dedicated runtime budget + per-channel extensibility + audit-readability) worth +~1000-2600 LOC migration cost over interleaved-with-boot reconciliation that achieves the same DETECTION outcomes — given that C's direction-validation evidence has accumulated across cycles 92 / 96 / 107 / 112 / 113 / 114 while A's central-bet rationale has been progressively shown to rest on cycle 111 framings that successively fail adversarial review?** This is a value-judgment where Eva's perspective is the load-bearing input — the orchestrator's analysis names the trade-off and **the cycle 113 four-finding flip threshold is honored cycle 114 (the orchestrator's tentative recommendation flipped from A to C)** but does not pre-empt Eva's resolution. If Eva judges named-phase processing structure important AND the cycle 114 flip rationale defensible: select C. If Eva judges A's bounded migration cost more important than the cumulative cycle 111-114 findings: select A (override the flip).

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
| 113 (2026-05-10) | **Second central-bet stress-test absorbed.** Cycle 113 retried cycle 112's BLOCKED Copilot adversarial dispatch — confirmed runtime allowlist gap is structural (`.github/workflows/orchestrator.yml` lines 66-87 has no `tools/dispatch-task` entry); workflow-change PR opened to add `Bash(tools/dispatch-task *)` and `Bash(tools/dispatch-review *)` to the allow-list, awaiting Eva merge. Pivoted to substantive central-bet stress-test on the cycle 112 "A and C have equivalent reconciliation outcomes" claim. Finding: detection outcomes genuinely equivalent (1-cycle latency, cursor-polling); processing characteristics differ on three under-weighted dimensions cycle 112 named as "legibility only" — (1) per-mode runtime budget allocation (C explicit ≤10min reconcile-mode budget vs A implicit shared boot-phase budget); (2) per-channel structured handlers (C explicit per-event handlers + typed-deltas vs A procedurally-loose channel handling); (3) typed-delta output discipline (C single auditable stream vs A scattered boot-phase outputs). Updated "Why this matters less than it first appears" section adding cycle 113 qualification (~30 lines). Updated "Reframed selection question" with cycle-113 framing alongside cycle-112 framing. Updated "Why the recommendation stays as A" to acknowledge the progressive weakening across cycles 111→112→113 and named the threshold for honest flip (fourth round of under-weighted dimensions surfacing). Updated outstanding work item #3, open question #6, side-by-side table footer note. **Recommendation stays as A** because Criteria 1+3 still favor A and the under-weighted processing dimensions are real but bounded — but the case for A continues to weaken. **Cycle 112's `central-bet-stress-test-finds-mis-grounding` candidate-pattern (NOVEL@1) promotes to TESTED@2** with cycle 113 substrate-content-distinct (cycle 112 found mis-grounding against retrospective text; cycle 113 found under-weighted dimensions in the cycle 112 reframing itself). | Audit cycle 216 critique landing (~next audit cron, ~17h). Workflow-change PR for runtime allowlist awaiting Eva merge. Adversarial Copilot feedback dispatch on the post-cycle-113 selection draft (re-attempt once allowlist PR merges; otherwise dispatch body file remains ready for cycle 114+ once permitted). Further stress-tests of remaining load-bearing claims (B's cluster-G aspirational extension grounding; C's conditional-improvement risk grounding; the 6-criterion ordering structure assumptions). Per-candidate Phase 3 prototype evidence deepening (cycle 114+). |
| 114 (2026-05-10) | **Third central-bet stress-test absorbed; four-finding flip threshold honored; tentative recommendation flipped from A to C.** Cycle 114 stress-tested the cycle 111 "Why not C" Reason #1 (conditional-improvement risk: "if the additions don't carry their weight, C is just A with more migration surface"). Finding: the framing cites C-hybrid.md line 73 + 116 (C's self-acknowledged risk language) but **omits C-hybrid.md lines 222-228 (cycle 92 Aggregate central-bet validation: "carry their weight on F4 and F11"; F2 marginal) + cycle 96 PR #2878 absorption (line 191: only F2 qualified as "ambiguous-without-prototype"; F4 + F11 not qualified) + lines 349-372 (cycle 107 Risk 2 closure at specification level with falsifiable thresholds)**. The cycle 111 framing presents the conditional as fully open when direction is in fact validated; the open conditional is magnitude (Phase 3 work). Combined with cycle 112's finding that "Why not C" Reason #3 ("F4/F11 unproven as load-bearing") depends on the cycle-112-refuted F1+F7-dominance basis, the cycle 111 case for A is shown to rest on a chain of motivated reasoning across 4 cycles of stress-testing. **The cycle 113 four-finding flip threshold is met (fourth substrate-content-distinct under-weighted dimension favoring C surfaces); the threshold is honored.** Updated status header + tentative selection (line 10) → C; added new "Cycle 114 stress-test: conditional-improvement framing weakened, four-finding flip threshold honored" section (~120 lines); updated Criterion 5 ordering to C > A > B with cycle-114 rationale; added "Post-cycle-114 stress-test update" paragraph to "Why not C" preserving the section as historical record; renamed and rewrote "Why the recommendation stays as A" → "Why the recommendation flipped to C (post-cycle-114)" with 5-point rationale + cycle 115+ stress-test items; updated outstanding work item #3 with cycle 114 absorption; inverted open questions Q1 (Approve C?) + Q2 (Override to A?); updated Q6 with cycle 114 framing. **`pre-committed-flip-threshold` candidate-pattern promotes from NOVEL@1 to TESTED@1-by-being-honored** (the threshold was a falsifiable pre-committed criterion; honoring it cycle 114 demonstrates it is a load-bearing discipline, not protective fiction). **`progressive-rationale-weakening-without-flip` candidate-pattern interrupted at NOVEL@1 by the flip** (cycle 114 broke the pattern; the flip prevents the pattern from HARDENING at NOVEL@2). **`central-bet-stress-test-finds-mis-grounding` candidate-pattern promotes from TESTED@2 to HARDENED-at-3** (cycle 112 retrospective mis-grounding + cycle 113 under-weighted dimensions in cycle 112 reframing + cycle 114 omitted-validation-evidence in cycle 111 conditional-improvement framing — three substrate-content-distinct findings). **`discretionary-departure-from-forward-going-commitment` candidate-pattern HARDENED@4** (cycle 110 named cycle 111+ rotation; cycles 111+112+113+114 all departed; cycle 114's substrate is the threshold-honoring action). | Audit cycle 216 critique landing (~next audit cron, ~8h from cycle 114). Audit will see the post-cycle-114 selection draft including the flip; the flip is itself a load-bearing claim audit can stress-test cycle 216+. Workflow-change PR for runtime allowlist awaiting Eva merge ([#2903](https://github.com/EvaLok/schema-org-json-ld/issues/2903); 1-cycle into 5-cycle autonomy default). Adversarial Copilot feedback dispatch on the post-cycle-114 selection draft (re-attempt once [#2903](https://github.com/EvaLok/schema-org-json-ld/issues/2903) resolves; the dispatch should stress-test the flip rationale itself for motivated reasoning, not just the original A recommendation). Further stress-tests for cycle 115+: (a) meta-reflexive stress-test of the cycle 114 flip rationale — is the four-finding threshold itself motivated reasoning? does the flip survive adversarial framing? (b) B-side stress-tests still outstanding (cluster-G grounding; 3-5× migration cost methodology); (c) the 6-criterion ordering structure assumptions; (d) per-candidate Phase 3 prototype evidence deepening with C-side measurements (`reconcile-mode` if Eva pre-approves) now more load-bearing post-flip. |
