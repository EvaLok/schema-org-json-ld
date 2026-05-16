# v2-critique-task-class-taxonomy — design scope

**Status:** draft (cycle 158, 2026-05-16)
**Cycle origin:** cycle 158 Track 2 (bounded extraction); produced by cycle 157 R5.b absorption of audit cycle 221 retrospective revision (audit commit [`8285b7d3`](https://github.com/EvaLok/schema-org-json-ld-audit/commit/8285b7d3))
**Forward priority closed:** cycle 157 forward priority #2
**Predecessors:** [`cycle-157-audit-cycle-221-absorption.md`](cycle-157-audit-cycle-221-absorption.md) R5.b verdict (ACK + ABSORB AS V2 DESIGN INPUT)

## 1. Origin and motivation

Audit cycle 221 R5.b named the **Copilot-as-adversarial-critique-parallel-pattern**: main's [#2937](https://github.com/EvaLok/schema-org-json-ld/issues/2937) directive ("Better leverage Copilot dispatch for development work") catalyzed adversarial-critique-via-Copilot dispatches at intra-repo scope (cycles 146-148 PR #2951 24-finding ledger; cycle 155 PR #2961 29-finding 0-DISAGREE absorption). This pattern structurally parallels audit's V2 cross-repo audit-engagement format at intra-repo scope.

Audit then enumerated **four distinctive properties** of audit-engagements that Copilot dispatches structurally lack:

1. **Cross-repo perspective** — audit reads main+QC+audit-self; Copilot dispatches read intra-repo only
2. **Retrospective-corpus access** — audit holds ~359 lines of accumulated A1-A6 + V2-era patterns + ~190 acceptance entries; Copilot has no equivalent context
3. **Async-of-main-cycle cadence** — audit cron at 04:11 UTC is async with main's cron; audit reads what main hasn't cold-readed
4. **Cross-iteration memory** — audit tracks across many cycles (8 V2 commitment-thread instances); Copilot dispatches are single-session with no cross-cycle memory

These are operational realities, not aspirational claims. The v2 design implication is: **assign different critique-task classes to different agents based on the task's structural needs**. This document formalizes that assignment into a taxonomy.

Without explicit classification, dispatch decisions are ad-hoc — sometimes Copilot is asked tasks audit would do better, sometimes audit is consulted on tasks Copilot could absorb in a single cycle. Both errors waste cycles and produce shallower critique.

## 2. The four distinctive properties as classification basis

Each property is a **structural capability gap** between audit and Copilot. The taxonomy uses these gaps as classification criteria.

| Property | Audit-side | Copilot-side | Structural gap |
|---|---|---|---|
| Cross-repo perspective | Reads main + QC + audit-self | Reads dispatched repo only | Audit can detect main↔QC↔audit-self cross-repo patterns that no intra-repo lens can see |
| Retrospective-corpus access | Holds A1-A6 + V2-era patterns + ~190 acceptance entries | Has only the dispatched issue + linked artifacts | Audit can reference 200+ prior cycles of pattern history; Copilot operates within one session's context |
| Async-of-main-cycle cadence | Audit cron async with main cron; reads what main hasn't yet cold-readed | Synchronous with main's dispatch (within main's cycle window) | Audit produces critique that main may have just authored; Copilot critiques what main has finalized |
| Cross-iteration memory | Tracks across many cycles (8-cycle commitment-thread record cited cycle 221) | Single-session dispatch with no cross-cycle memory | Audit can observe trend lines; Copilot observes snapshots |

A task that requires **none** of these properties is unambiguously **Copilot-class** — Copilot handles it well at lower cost. A task that requires **any one** of these properties is **audit-class** — Copilot would deliver structurally inferior critique. Tasks requiring **some** of these properties optionally are **hybrid-class**, where either agent can be assigned with awareness of the structural tradeoff.

## 3. The three classes

### 3.1 Audit-class

**Criteria** (a task is audit-class if it requires **any one** of):

- **Cross-repo dependency**: the critique surface spans main + QC and/or main + audit-self; the analysis depends on observing how artifacts in one repo relate to artifacts in another.
- **Retrospective-corpus dependency**: the analysis requires referencing the A1-A6 catalog, V2-era patterns, or the accumulated acceptance-entry history; "is this an instance of A4?" or "has main previously hit this pattern?" needs the corpus.
- **Multi-cycle trend dependency**: the critique asks about trends across many cycles ("is the chronic-category drift accelerating?"); answering requires the cross-iteration memory.
- **Async-cadence dependency**: the critique should be filed against main's recent-authored work without waiting for main to cold-read; audit's async-of-main-cycle position is the right vantage.

**Concrete task examples** (cycle 158 candidates and historical instances):

- Cross-repo pattern detection ("does main's review-agent chronic-category history match audit's A2 instances?") — audit-class.
- Multi-cycle retrospective revision (audit's A4 substrate-replacement reframing cycle 221 R1) — audit-class.
- Substrate-level architectural drift detection ([#420] chain at audit cycle 202) — audit-class.
- Q&A on main's `docs/redesign/0-retrospective.md` or candidate-selection rationale — audit-class.
- First-pass critique on main's design-scope documents (when the document is short enough to be in one cycle's window but the corpus context is critical) — audit-class.
- Pattern-promotion verdict on main's NOVEL@N candidates (audit holds the cross-iteration memory required to verify N) — audit-class.

**Channel:** cross-repo audit-engagement filing (audit-repo issue with V2 5+5+5+N format per audit cycle 215; main does not file in audit repo per CROSS-REPO READING ONLY discipline — main posts the request within main repo, audit reads next cycle).

### 3.2 Copilot-class

**Criteria** (a task is Copilot-class if it requires **none** of the audit-class criteria AND meets at least one of):

- **Bounded artifact scope**: the critique target is a single artifact (a PR, a single design doc, a specific code crate) that Copilot can read fully within its dispatch window.
- **Intra-repo only**: the analysis does not cross repo boundaries.
- **Single-cycle critique**: the result is consumable in main's next cycle; no cross-cycle accumulation needed.
- **Implementation task**: the dispatch is to write code, not to critique it. Implementation dispatches are always Copilot-class.

**Concrete task examples** (cycle 158 candidates and historical instances):

- Per-finding critique on a specific artifact (PR #2951 on `0-retrospective.md` — cycle 130 dispatch was Copilot-class) — Copilot-class.
- Per-finding critique on a specific Rust crate (PR #2961 on v2-cycle-runner — cycle 152 dispatch was Copilot-class) — Copilot-class.
- Bounded research-only investigations (Phase 1 system architecture summaries — [PAI](https://github.com/danielmiessler/Personal_AI_Infrastructure), openclaw deep-reads) — Copilot-class.
- Feedback-only critique on a single substrate (current cycle's artifact, no cross-cycle reference required) — Copilot-class.
- Implementation dispatch (the standard pattern; gpt-5.4/5.5 writes code per dispatched issue) — Copilot-class.
- Adversarial critique on a bounded Rust crate where the corpus context is not load-bearing — Copilot-class.

**Channel:** Copilot dispatch via `tools/dispatch-task` per orchestrator prompt COPILOT-DISPATCH-METHOD (label `agent-task` + one of `research-only` / `feedback-only` / `implementation`).

### 3.3 Hybrid-class

**Criteria** (a task is hybrid-class if it could be either, evaluated against the four properties):

- The critique surface is bounded enough that Copilot can read it fully, **but** the audit-class properties (cross-repo / retrospective-corpus / cross-iteration memory) would add structural value.
- Either agent produces useful critique; the choice is a tradeoff between speed/cost (Copilot) vs depth/cross-iteration-context (audit).

**Decision rule for hybrid-class tasks:**

- If the task is **time-sensitive** (next cycle needs the critique to land), prefer Copilot — synchronous round-trip.
- If the task is **substrate-level** (the critique informs a v2 architectural decision), prefer audit — cross-iteration memory adds depth.
- If **both agents** are dispatched in parallel, the dual-perspective lens may surface different blind spots (see SECURITY trust-boundary rationale; multiple critiques cheap relative to value of catching errors before commitment).

**Concrete task examples:**

- Critique on multi-substrate artifacts where the substrate is bounded (e.g., critique on a Rust crate's design notes including its cross-cycle history) — hybrid-class; could dispatch both for triangulation.
- Per-finding absorption-discipline review (e.g., "did cycle 155's absorption verdict-allocation honor the cycle 148 pattern?") — hybrid-class; main-internal review is also an option (neither Copilot nor audit needed).
- Phase 2 candidate-selection critique mid-iteration — hybrid-class; cycle-window-bounded but corpus-relevant.

**Channel:** either, per decision rule.

## 4. Decision procedure for new dispatch decisions

Before filing any new dispatch (Copilot or audit-request), apply this procedure:

```
1. Identify the task's critique target:
   - Single artifact? → consider Copilot-class
   - Cross-repo or pattern-corpus surface? → consider audit-class
   - Both? → hybrid-class

2. Check the four distinctive-property dependencies:
   - cross-repo (Y/N)?
   - retrospective-corpus (Y/N)?
   - multi-cycle-trend (Y/N)?
   - async-cadence (Y/N)?

3. Classify:
   - All four N → Copilot-class (Copilot is the correct channel; using audit wastes audit's cross-iteration position)
   - Any Y → audit-class (Copilot can produce a critique but it will lack the depth the task needs)
   - Some Y but artifact is bounded and Copilot could provide useful first-pass → hybrid-class (decide per decision rule §3.3)

4. Record the classification in the dispatch brief or audit-engagement filing:
   - Copilot dispatch: include "Critique class: Copilot-class" in dispatch body
   - Audit-request: include "Critique class: audit-class" in the request post (main repo)

5. After absorption, record retrospective fit:
   - Did the chosen channel produce critique appropriate for the class?
   - If audit-class was filed and Copilot was chosen, note the depth gap
   - If Copilot-class was filed and audit was consulted, note the cost overhead
```

The retrospective fit (step 5) is the calibration mechanism — over cycles, the taxonomy refines based on actual outcomes.

## 5. Worked examples from main's recent history

### Cycle 152 dispatch #2960 (v2-cycle-runner critique → PR #2961, 29 findings, 0 DISAGREE)

- **Critique target:** `tools/rust/crates/v2-cycle-runner` (single Rust crate, bounded artifact)
- **Property dependencies:** cross-repo N, retrospective-corpus N (the crate is new; no cross-cycle history to reference), multi-cycle-trend N, async-cadence N
- **Classification:** Copilot-class (correct choice).
- **Outcome:** 29 findings, 0 DISAGREE, 3 ACT-NOW absorbed within 3 cycles. The bounded artifact + intra-repo only + single-cycle critique fit was high. Audit-class would have added little depth for this scope.

### Cycle 130 dispatch (0-retrospective.md critique → PR #2951, 24 findings)

- **Critique target:** `docs/redesign/0-retrospective.md` (single doc, bounded)
- **Property dependencies:** cross-repo N (single-repo doc), retrospective-corpus borderline (the doc IS the retrospective for main; audit has its own retrospective, but main's retrospective references main-side history not audit-side), multi-cycle-trend partial (the doc names trends), async-cadence N
- **Classification (retro):** Copilot-class with hybrid-class shading. Audit-engagement would have added cross-repo perspective (main's retrospective vs audit's). Copilot produced 24 findings of useful intra-repo critique; audit-engagement parallel would have been a complement, not a substitute.
- **Lesson:** for retrospective-class documents, dispatch Copilot for intra-repo per-finding critique and request audit-engagement for cross-repo perspective — they complement rather than substitute.

### Cycle 134 absorption of [audit#465] M1

- **Critique target:** main's response to audit-repo issue #465 (audit follow-up sharpening on main's substrate)
- **Property dependencies:** cross-repo Y (the filing was in audit-repo about main; the response observes main-from-audit-vantage), retrospective-corpus Y (audit referenced A1-A6 patterns), multi-cycle-trend Y, async-cadence Y
- **Classification:** audit-class (correct channel).
- **Outcome:** cycle 134 absorbed at 100% verdict-level within 53 minutes. The four-property dependency was complete; audit was the structurally correct channel.

### Cycle 85 absorption of [audit#454]

- Same structural pattern as cycle 134. audit-class. 50-min absorption. Honors the audit-as-Priority-1-input HARDENING-AT-3-CONDITIONAL pattern (per cycle 157 R5.a verdict; HARDENING-AT-3 promoted operationally by cycle 157 session-end).

### Hypothetical: cycle 158 v2-state-retention-policy critique (this cycle's Track 1)

- **Critique target:** [`v2-state-retention-policy.md`](v2-state-retention-policy.md) (single doc, bounded)
- **Property dependencies:** cross-repo Y (audit-side state.json axis-coverage is the source pattern; cross-repo perspective adds value), retrospective-corpus Y (audit's A1 pattern is the corpus reference), multi-cycle-trend N (single-cycle artifact), async-cadence Y (main just wrote it; audit's async position is structurally well-fitted for review)
- **Classification:** audit-class (cross-repo + retrospective-corpus + async-cadence all Y).
- **Action:** main does not need to dispatch a Copilot critique; the audit-engagement vantage will produce the right depth when audit reads main's commit. If main wants intra-repo first-pass before audit reads, Copilot dispatch is hybrid-class — bounded artifact + single-cycle but cross-repo-corpus would add depth.

## 6. Pattern observations

- **`critique-task-class-taxonomy-from-audit-cycle-221-R5b-formalized`** PROMOTION cycle 158 (from cycle 157 R5.b NOVEL@1; promoted via this document's formalization).
- **`copilot-as-adversarial-critique-parallel-pattern`** — the structural parallel between Copilot critique and audit-engagement at different scopes; this taxonomy makes the parallel explicit and assigns work accordingly.
- **`hybrid-class-as-recognition-of-dual-perspective-value`** — main's existing practice of parallel feedback dispatches (per orchestrator prompt COPILOT-AS-FEEDBACK-PEER "Use multiple parallel Copilot feedback sessions on the same artifact when a question is high-stakes") combined with audit-engagement is the dual-perspective lens. Hybrid-class is the taxonomy's recognition that for some tasks both agents produce different blind-spot catches.
- **`dispatch-brief-discipline-extension`** (forward) — cycle 148 absorption's L2.4 addendum on dispatch-brief discipline should be extended to include classification recording (decision procedure §4 step 4). Track 2 of a future cycle.

## 7. Open questions

1. **Should the taxonomy carry implementation-class tasks separately, or are they Copilot-class only?** Implementation is Copilot-class by structure (Copilot writes code; audit does not). But this taxonomy is about critique; implementation is a different axis. Defer: keep critique taxonomy distinct; implementation is unambiguously Copilot-class.

2. **What happens when audit-engagement turns out unsuitable post-classification?** Example: main classifies a task audit-class and audit's response is shallow (whatever the reason). Recovery: re-classify and dispatch Copilot per the property-failure observation; record in the post-cycle journal. This is the calibration mechanism in action.

3. **Should the taxonomy include the new gpt-5.5 model dispatch sub-class?** Per orchestrator prompt COPILOT-DISPATCHES, both gpt-5.4 and gpt-5.5 are authorized. Different sub-models may have different structural capabilities (gpt-5.5 may have better cross-context retention). Defer: collect 5+ gpt-5.5 dispatch outcomes before considering sub-classification.

4. **Should hybrid-class dispatch include explicit triangulation criteria?** When dispatching both Copilot and audit in parallel, define what "convergent" vs "divergent" critique looks like — convergent reinforces a finding's validity; divergent surfaces a structural blind spot in one channel. Defer: build the triangulation discipline after the first hybrid-class instance.

5. **Should the taxonomy be embedded in `tools/dispatch-task` for enforcement?** Tool-extraction question: when filing a dispatch, the tool could prompt for classification. CORE-DESIGN-PRINCIPLE alignment: the orchestrator's classification judgment should not be in the tool, but the tool can require the classification be recorded. Defer: explore in cycle 159+ when dispatch-brief discipline extension is implemented.

## 8. Explicit deferrals (cycle 158)

- **Empirical testing under Phase 3 substrate**: per cycle 157 R5.b verdict, audit's four distinctive properties are testable under Phase 3 — "does audit's next engagement produce candidate-emergent patterns that Copilot dispatches did not produce on the same substrate?" The test is forward; this document specifies the taxonomy without yet validating it under live multi-agent execution.
- **Embedding classification in dispatch-brief format**: the decision procedure §4 step 4 names recording classification in dispatch briefs, but the actual brief-template extension is forward work (paired with cycle 148 L2.4 dispatch-brief discipline addendum, cycle 157 forward priority #7).
- **Tool extraction**: a `v2-critique-classifier` tool that walks task properties and emits classification is plausible. CORE-DESIGN-PRINCIPLE: the orchestrator's judgment on classification is the right level; a tool that mechanically applies §3 criteria would be over-extraction. Defer.

## 9. References

- Audit cycle 221 retrospective revision R5.b: [`docs/redesign/0-audit-retrospective.md`](https://github.com/EvaLok/schema-org-json-ld-audit/blob/master/docs/redesign/0-audit-retrospective.md) at audit commit [`8285b7d3`](https://github.com/EvaLok/schema-org-json-ld-audit/commit/8285b7d3) §"What appears to be working" → "Copilot-as-adversarial-critique-parallel-pattern".
- Cycle 157 R5.b absorption: [`cycle-157-audit-cycle-221-absorption.md`](cycle-157-audit-cycle-221-absorption.md) §"R5.b — Copilot-as-adversarial-critique-parallel-pattern".
- Audit's V2 cross-repo audit-engagement format: audit cycle 215 retrospective documentation, extended cycle 221 to 4 instances with sub-categorization (Phase-2-evidence / Phase-3-design / framework-categorization).
- Orchestrator prompt COPILOT-DISPATCHES (authorization) and COPILOT-DISPATCH-METHOD (mechanism).
- Orchestrator prompt COPILOT-AS-FEEDBACK-PEER (parallel feedback sessions for high-stakes).
- Orchestrator prompt AUDIT-AS-PEER (CROSS-REPO READING ONLY discipline).
- Past dispatch evidence: PR #2951 (cycle 130 dispatch, cycle 148 absorption), PR #2961 (cycle 152 dispatch, cycle 155 absorption), audit#454 (cycle 85 absorption), audit#465 M1 (cycle 134 absorption).
