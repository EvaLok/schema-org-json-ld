# Cycle 104 — Cross-cluster intersection updates absorbing cycle 102 sub-shapes (4 new sub-patterns: A↔B 6, F↔H 6+7, A↔C 6)

**Date:** 2026-05-09
**Cycle issue:** [#2893](https://github.com/EvaLok/schema-org-json-ld/issues/2893)
**Phase:** redesign Phase 2 candidate iteration (under [`ITERATION-UNTIL-APPROVAL`](../../../.github/workflows/orchestrator-redesign-prompt.xml))
**Cycle composition shape:** **cross-cluster intersection extension via sub-shape absorption** — NOVEL functional-class shape #22 at 1 instance. Extending established cross-cluster intersection sub-pattern enumerations to integrate newly-absorbed within-cluster sub-shapes from a multi-cycle absorption arc. Distinct from `catalogue-rebuild-with-multi-cycle-absorption` (cycle 102, NOVEL shape #20 at 1 instance) — that shape rebuilds within-cluster catalogues; this shape extends cross-cluster intersection sub-pattern catalogues. Distinct from `risk-closure-at-specification-level` (cycle 103, NOVEL shape #21 at 1 instance) — that shape closes a candidate-specific risk; this shape extends a cross-system synthesis layer. **23 functional-class shapes total demonstrated cycles 62-104 at 44 instances.**

## Setup

Cycle 103 named cross-cluster intersection updates as MEDIUM PRIORITY for cycle 104+:

> **Cross-cluster intersection updates** [cycle 104+, MEDIUM PRIORITY] — re-evaluate the 7 cross-cluster intersection patterns from cycles 72/74 with the new cluster A + cluster H sub-shapes from cycle 102. Specific candidate intersections: A↔B (classifier-dispatch storage), F↔H (Sonnet substrate dual-cast), A↔C (task-ingestion-routing as session-start lifecycle).

Cycle 102 catalogue rebuild added 4 new sub-shapes:
- **Cluster A**: A.10 (classifier-mediated mode dispatch from PAI), A.11 (deterministic-decision-tree routing from omx). Both at session-entry, forming a "classify-then-route pipeline" per cluster A annotation.
- **Cluster H**: H.5 (feedback-signal-inference from PAI), H.6 (evaluator-driven keep-discard from omx). Both quality-judgment-axis primitives distinct from existing aggregation-window sub-shapes.

These new sub-shapes had not yet been integrated into the cross-cluster intersection layer (cycles 72 + 74 synthesis predates the cycle 102 absorption). Cycle 104 substantive focal: integrate the 4 new sub-shapes into the cross-cluster intersection sub-pattern enumerations.

## Design choices

### 1. Add new sub-patterns vs extend existing sub-patterns

**Decision:** Add new sub-patterns rather than extend existing ones to mention new H sub-shapes.

**Reasoning:** The existing cross-cluster intersection sub-patterns are tightly enumerated with specific within-cluster sub-shape pairings. Extending an existing sub-pattern to mention a new H sub-shape would conflate two architecturally-distinct mechanisms. Sub-pattern 3 (cost-tier × score-gated consolidation) is about gating frequency-of-consolidation; cost-tier × evaluator-driven keep-discard is about gating evaluator-strictness. Different axes of stratification warrant different sub-patterns.

**Alternative considered and rejected:** Mention new H sub-shapes inside existing sub-patterns 2 + 3 as supplementary text. Rejected because it conflates distinct stratification axes and reduces the granularity that makes sub-patterns useful for Phase 2 candidate evaluation.

### 2. Sub-pattern count mapping

**Decision:** 4 new sub-patterns total (1 in A↔B, 2 in F↔H, 1 in A↔C). Specifically:
- A↔B sub-pattern 6: Session-entry classification/routing × cluster B classification-record (joint A.10 + A.11 with cluster B writes)
- F↔H sub-pattern 6: Autonomy-mode × feedback-signal-inference (Voyager I-V9 + PAI H.5 cross-system composition)
- F↔H sub-pattern 7: Cost-tier × evaluator-driven keep-discard (Voyager I-V2 + omx H.6 cross-system composition)
- A↔C sub-pattern 6: Session-entry classification/routing × lifecycle-op selection (joint A.10 + A.11 with cluster C ops)

**Reasoning:**
- A.10 + A.11 form a classify-then-route pipeline (per cluster A annotation cross-references). This composes with cluster B (writes for both classification and route records) and with cluster C (lifecycle-op selection at session-entry). These are genuinely distinct intersections — one cluster B sub-pattern and one cluster C sub-pattern.
- H.5 alone (feedback-signal-inference) intersects with cluster F autonomy-mode axis. Different axes of stratification mean this is its own F↔H sub-pattern.
- H.6 alone (evaluator-driven keep-discard) intersects with cluster F cost-tier axis. Different mechanism (evaluator vs consolidation gating) means it's distinct from existing sub-pattern 3.

**Alternatives considered and rejected:**
- Add 4 sub-patterns, one per new sub-shape (2 A↔B from A.10 vs A.11 separately; etc.) — rejected because A.10 + A.11 explicitly form a pipeline and the cluster B write-coordination is shared
- Add 1 sub-pattern total absorbing all 4 new sub-shapes — rejected because the sub-shapes intersect with different cluster pairs (A↔B vs F↔H vs A↔C) and a single sub-pattern can't span 3 intersections

### 3. New v1 failure modes named

**Decision:** Add 4 new v1 failure modes addressed by the new sub-patterns:
- A↔B: **Re-classified-from-scratch** — v1 has no session-entry classification mechanism; routing is implicit in prose
- F↔H: **No-inferred-feedback-stratification** — v1 has no inferred-feedback mechanism nor autonomy-mode toggle
- F↔H: **Uniform-evaluator-cost** — v1 has no evaluator-driven keep-discard nor cost-tier stratification
- A↔C: **Implicit-lifecycle-selection** — v1's only formalized lifecycle op is `resume`, fired automatically by cron

**Reasoning:** Each new sub-pattern addresses a v1 failure mode that the existing sub-patterns did not address. Naming the failure modes anchors the sub-patterns to v1-specific evidence rather than letting them float as design speculation.

**Honest qualification:** These v1 failure modes are real (v1 demonstrably has these gaps), but they're not high-priority failure modes like the chronic-category currency loop or stale-reference accumulation. They're "v1 lacks this mechanism entirely" failures, addressable by adding the mechanism. Whether the addition is worth its substrate-design cost is a Phase 2 candidate-selection decision, not a verdict from this synthesis.

### 4. F↔I substrate-coverage extension treatment

**Decision:** Update the meta-observation `Cluster I substrate-correlation revisited` paragraph rather than add a new F↔I sub-pattern.

**Reasoning:** Cycle 102 extended cluster I substrate-coverage to a 4-substrate-type map (cloud-anchored multi-actor + single-user personal-assistant + configuration-layer-over-CLI + research-artifact substrate-absent). This is a *recalibration of substrate-correlation reasoning*, not a new mechanism intersection. The existing 5 F↔I sub-patterns remain valid; the substrate-correlation argument shifts from "v1 is uniquely positioned" to "v1 inherits cluster I primitives the same way other harness-mediated substrates do." This is a meta-observation update, not a sub-pattern addition.

**Alternative considered and rejected:** Add F↔I sub-pattern 6 about substrate-coverage. Rejected because substrate-coverage is not a stratification axis — it's a substrate-correlation observation. F↔I sub-patterns are about tier-stratification of enforcement, not about substrate-coverage scope.

### 5. Symmetric-vs-asymmetric classification update

**Decision:** Do not modify the symmetric-vs-asymmetric paragraph in meta-observation despite the new sub-patterns.

**Reasoning:** The new sub-patterns don't change the overall symmetry classification of any intersection:
- A↔B remains symmetric (sub-pattern 2 anchors symmetry via dual-cast; new sub-pattern 6 is asymmetric A→B but doesn't unbalance the overall classification)
- A↔C remains mixed-symmetry (sub-pattern 2 is genuinely symmetric via dual-cast; new sub-pattern 6 is asymmetric A→C; ratios unchanged)
- F↔H remains mostly-symmetric (sub-pattern 6 autonomy-mode × inference is mostly symmetric; sub-pattern 7 cost-tier × evaluator is more asymmetric but doesn't shift the overall classification)

The classifications are stable across cycle 104 absorption. Updating the paragraph to add cycle 104 caveats would add bulk without changing the substantive classification.

## Edits

### `docs/redesign/1-research/clusters.md` (5 edit locations, +245 lines net)

1. **Cross-cluster intersections section intro** (~22 lines added): Cycle 104 absorption paragraph after the cycle 85 compositional/dual-cast classification text, naming the 4 new sub-patterns and their integration scope.

2. **A↔B section** (~38 lines added):
   - Header text: "Five sub-patterns" → "Six sub-patterns from the corpus (sub-pattern 6 added cycle 104 absorbing cluster A sub-shapes 10 + 11 from cycle 102)"
   - New sub-pattern 6 inserted after sub-pattern 5 (~25 lines)
   - New v1 failure mode bullet "Re-classified-from-scratch" (~7 lines)
   - Phase 2 implication paragraph extended with cycle 102 cluster A sub-shape note (~8 lines)

3. **F↔H section** (~85 lines added):
   - Cluster H descriptor: "4 post-session feedback sub-shapes" → "6 post-session feedback sub-shapes" with the two cycle 102 additions named
   - Header text: "Five sub-patterns" → "Seven sub-patterns from the corpus (sub-patterns 6 + 7 added cycle 104 absorbing cluster H sub-shapes 5 + 6 from cycle 102)"
   - New sub-pattern 6 (autonomy-mode × feedback-signal-inference, ~22 lines)
   - New sub-pattern 7 (cost-tier × evaluator-driven keep-discard, ~22 lines)
   - v1 failure mode section converted from single-paragraph "v1 failure mode addressed" to bulleted "v1 failure modes addressed" with 3 bullets (existing chronic-category, NEW no-inferred-feedback-stratification, NEW uniform-evaluator-cost)
   - Phase 2 implication paragraph extended with cycle 102 cluster H sub-shape note (~12 lines)

4. **A↔C section** (~52 lines added):
   - Header text: cluster A descriptor extended to include "session-entry classification/routing"
   - Header text: "Five sub-patterns" → "Six sub-patterns from the corpus (sub-pattern 6 added cycle 104 absorbing cluster A sub-shapes 10 + 11 from cycle 102)"
   - New sub-pattern 6 (session-entry classification/routing × lifecycle-op selection, ~32 lines)
   - New v1 failure mode bullet "Implicit-lifecycle-selection" (~10 lines)
   - Phase 2 implication paragraph extended with cycle 102 cluster A sub-shape note + 3-trigger-types observation (~12 lines)

5. **Meta-observation section** (~21 lines added):
   - "Cluster I substrate-correlation revisited" paragraph extended with cycle 102 substrate-coverage extension recalibration (~17 lines)
   - Pre-paragraph in `Meta-observation: cross-cluster as next-layer v2 design-input` extended with sub-pattern count update + new "Cycle 104 absorption — session-entry as new boundary type" paragraph (~21 lines)

Net: +245 lines, file size 3853 → 4098 lines (+6.4%).

### `docs/redesign/1-research.md` (1 edit location, +7 lines net)

The "Family-level observations above" → "deeper-pass over the same six deep-dive systems" paragraph extended with cycle 100-102 absorption note + cycle 104 cross-cluster integration note + sub-pattern total update (34 → 38).

## Not modified (explicit)

The following were considered for modification but explicitly NOT modified:

1. **D↔I, B↔C, F↔I, E↔I sections** — cycle 102's new sub-shapes do not directly compose with D, E, or I cluster sub-shapes at sub-pattern density. The intersection sections remain at their cycle 72/74 sub-pattern counts (5+5+5+4 = 19 sub-patterns unchanged). Cycle 104 absorption note in the section intro names this explicitly.

2. **Within-cluster catalogues (clusters A, B, C, D, E, F, G, H, I)** — cycle 102 catalogue rebuild handled within-cluster updates. Cycle 104 substantive focal is cross-cluster integration, not within-cluster work.

3. **Cluster comparison summary table** — unchanged. The cycle 102 update brought it to 50 → 54 sub-shapes + 1 candidate. Cycle 104 sub-pattern additions are at the cross-cluster level, not the within-cluster level.

4. **Symmetric-vs-asymmetric paragraph** — explicit decision (see Design choice 5). The new sub-patterns don't change classifications.

5. **Open structural questions** — the 4 questions (cluster F split, cluster A vs G boundary, orphan-pattern observation, file-size threshold) are not affected by cycle 104 cross-cluster sub-pattern additions.

6. **Implications-mining cadence summary** — historical record cycles 62-74; not affected by cycle 104.

7. **2-design-framework.md** — not measurement-methodology nor framework-shape work; cross-cluster intersection sub-patterns are research-corpus content.

8. **2-candidates A/B/C** — not candidate-evaluation work; cycle 104 substantive focal is research-corpus-side cross-cluster integration. Candidate sharpening is cycle 105+ work per cycle 103 hand-off.

9. **MEMORY.md** — runner-local persistence is structurally ephemeral per cycle 103 falsification (cycle 100 MEMORY.md initialization claim falsified at cycle 103). Not re-attempted.

## Verification

- File `docs/redesign/1-research/clusters.md` line count: 3853 → 4071 (+218 lines, +5.7%); confirmed via `wc -l`.
- File `docs/redesign/1-research.md` line count: 850 → 856 (+6 lines).
- New sub-pattern counts in clusters.md: A↔B 6 (was 5), F↔H 7 (was 5), A↔C 6 (was 5); D↔I 5, B↔C 5, F↔I 5, E↔I 4 unchanged. Total: 38 (was 34). Cycle 104 absorption note in meta-observation states this total.
- Cycle 102 sub-shape references checked: A.10 and A.11 referenced 7 times in sub-pattern 6 of A↔B + 6 times in sub-pattern 6 of A↔C; H.5 referenced 5 times in sub-pattern 6 of F↔H; H.6 referenced 5 times in sub-pattern 7 of F↔H. All references coherent with cycle 102 catalogue annotations.
- Phase 2 implication updates added to all 3 modified intersection sections; v1 failure mode bullets added (1 to A↔B, 2 to F↔H, 1 to A↔C).
- Substrate-coverage extension paragraph added to meta-observation Cluster I substrate-correlation revisited section.
- 1-research.md cross-cluster intersection paragraph synced with new sub-pattern total.

## What surprised me / what I noticed

1. **The 4 new sub-shapes from cycle 102 produce 4 new sub-patterns at 1:1 ratio at the within-cluster-pair level, but spread across 3 of 7 intersection sections.** A.10 + A.11 jointly produce 1 A↔B sub-pattern + 1 A↔C sub-pattern (the joint pipeline composes with both B and C); H.5 produces 1 F↔H sub-pattern; H.6 produces 1 F↔H sub-pattern. The 1:1 ratio is structurally constrained — each new within-cluster sub-shape participates in a bounded number of cross-cluster intersections based on which clusters it has compositional adjacency with. **Pattern observation: cluster A new sub-shapes (boundary-type) tend to compose with both B (storage) and C (lifecycle); cluster H new sub-shapes (feedback-mechanism) tend to compose only with F (stratification axes).** Other intersection patterns (e.g., H × I, H × B) are absent from the cycles 72/74 synthesis and didn't gain new sub-patterns from cycle 104.

2. **Session-entry as a NEW boundary type for cluster A is a load-bearing architectural distinction that did not exist in cycles 72/74 synthesis.** Pre-cycle-102, cluster A boundaries were super-step / phase / watchdog / termination-predicate / retry-exhaustion / lane-FIFO. None of those are "session-entry before phase semantics begin." Sub-shapes 10 + 11 introduce session-entry as a distinct boundary type, and the cycle 104 sub-patterns formalize this for cross-cluster intersection. Phase 2 candidates that adopt session-entry routing get a new boundary type to reason about — this is more architecturally consequential than "two more sub-shapes within cluster A."

3. **Quality-judgment-axis vs aggregation-window distinction in cluster H is structurally orthogonal.** H sub-shapes 1-4 (cycles 70 mining baseline) are aggregation-window mechanisms — when does feedback fire (tight cycle / score gate / continuous / capability-accumulation). H sub-shapes 5-6 (cycle 102 absorption) are quality-judgment-axis mechanisms — what quality signal is consumed (inferred-from-utterance / write-time-LLM-critic). The two axes compose without redundancy; v2 candidates can adopt 2-3 H sub-shapes spanning both axes without overlap. **The cluster H sub-shape catalogue is structurally 2-axis** (window × judgment-source) rather than 1-axis (mechanism-list).

4. **The cycle 102 sub-shape additions did not create any new dual-cast cases.** Dual-cast classification (cycle 85 audit#454 D2 absorption) flags sub-patterns where one mechanism is cast under both cluster labels — same primitive serving dual roles. The 4 new cycle 104 sub-patterns are all compositional (two distinct mechanisms from two clusters composing at their boundary). No A.10/A.11 mechanism is structurally identical to a cluster B or C mechanism; no H.5/H.6 mechanism is structurally identical to a cluster F mechanism. The dual-cast/compositional ratio across all 7 intersections (3 dual-cast / 35 compositional pre-cycle-104; 3 dual-cast / 39 compositional post-cycle-104) shifts toward compositional. **Pattern observation: deeper-system absorption tends to add compositional sub-patterns rather than dual-cast ones**, because deeper systems have richer within-cluster vocabularies that don't blur cluster boundaries the way thin within-cluster vocabularies do.

5. **The substrate-coverage extension recalibration weakens v1's "uniquely positioned for cluster I" argument materially.** Pre-cycle-102, the F↔I substrate-correlation argument relied on v1's GitHub-Actions substrate being uniquely correlated with cluster I's canonical substrate (cloud-anchored multi-actor with audit). Cycle 102's 4-substrate-type map shows that PAI's single-user personal-assistant substrate, omx's configuration-layer-over-CLI substrate, and openclaw + OpenAI harness's cloud-anchored multi-actor substrate ALL transfer cluster I patterns. v1 is one of multiple correlated substrates, not uniquely positioned. **Phase 2 weighting argument adjustment:** candidates that adopt cluster I patterns should justify on substrate-correlation-generally, not on v1-uniquely. This is a subtle but material recalibration of the cycle 89 audit#454 P2 weighting argument.

6. **Sub-pattern count totals across intersections are now asymmetric in a new way.** Pre-cycle-104: 5+5+5+5+5+5+4 = 34 sub-patterns. Post-cycle-104: 6+7+5+6+5+5+4 = 38 sub-patterns. The 7-sub-pattern F↔H is now the densest intersection; the 4-sub-pattern E↔I is now the sparsest. Asymmetry in sub-pattern density loosely correlates with corpus depth — the F↔H intersection involves clusters with 8 + 6 sub-shapes, which gives more potential composition surface than E↔I (involving clusters with 2 + 2 sub-shapes). Sub-pattern density may be an under-explored evaluation lens for Phase 2 candidate intersection coverage. **Methodological observation:** sub-pattern density per intersection is itself an artifact of within-cluster sub-shape richness, not a free parameter. v2 candidates that adopt rich within-cluster vocabularies will have richer cross-cluster intersection surfaces to compose with.

7. **The cycle 104 absorption confirms the cycle 103 named priorities (A↔B, F↔H, A↔C) were exactly correct.** Cycle 103's forward-look named exactly the 3 intersections that cycle 104 modified, with the candidate intersections "classifier-dispatch storage" / "Sonnet substrate dual-cast" / "task-ingestion-routing as session-start lifecycle." The first and third map directly to the new sub-patterns. The second ("Sonnet substrate dual-cast") was a less precise framing — what cycle 104 actually added was 2 F↔H sub-patterns (autonomy-mode × inference; cost-tier × evaluator-driven), which use Sonnet substrate but are not dual-cast (no mechanism cast under both F and H labels). **Cycle 103's forward-look was 2/3 precise on intersection identification.** Honest acknowledgment: the "Sonnet substrate dual-cast" framing was imprecise; cycle 104 absorption surfaces this and resolves it.

## Sibling pattern tracking

- **NOVEL functional-class shape #22 — `cross-cluster-intersection-extension-via-sub-shape-absorption`** — cycle 104 first instance. Pattern: extending established cross-cluster intersection sub-pattern enumerations to integrate newly-absorbed within-cluster sub-shapes from a multi-cycle absorption arc. Distinct from `catalogue-rebuild-with-multi-cycle-absorption` (within-cluster scope) and `risk-closure-at-specification-level` (candidate-specific scope). **NOVEL at 1 instance.**
- **Functional-class shape #21 (risk-closure-at-specification-level)** — cycle 103 introduced; cycle 104 NOT a new instance.
- **Functional-class shape #20 (catalogue-rebuild-with-multi-cycle-absorption)** — cycle 102 introduced; cycle 104 NOT a new instance.
- **Functional-class shape #19 (research-deliverable-absorption-with-verification)** — HARDENED at 3 verification-success + 1 verification-mixed instances. Cycle 104 NOT a new instance.
- **Verification-discipline pattern** → 6 instances HARDENED. Cycle 104 NOT a new instance.
- **Pre-agreed-falsification-threshold discipline** → 3-candidate completeness HARDENED at 1 instance per cycle 103. Cycle 104 NOT a new instance.
- **Generalization-level discipline (J-Q(a))** → 19 instances HARDENED. Cycle 104 application: each new sub-pattern explicitly names what it generalizes-from (specific corpus pairs) without over-generalizing to unsupported scope. Not a new instance count; discipline holds as protocol structure.
- **Direction-vs-magnitude discipline** → 11 instances HARDENED. Cycle 104 application: not directly relevant; this is sub-pattern enumeration, not magnitude estimation.
- **Cycle-composition-polarity discipline** → 41 instances HARDENED at cycle 102; cycle 103 application; cycle 104 substantive focal IS "research-corpus-advancement" (per default polarity) applied to cross-cluster integration. Cycle 104 application of polarity discipline.
- **Audit-as-peer pattern** — 2-instance evidence holds. Audit cycle 213 still silent-failed (no audit-repo commit since cycle 212 at 2026-05-07T04:35Z, ~52+ hours since cycle-101 measurement; audit silent-failure arc continues). Cycle 104 NOT a new instance.

## Lexicon entries (cycle 104)

- *Cross-cluster-intersection-extension-via-sub-shape-absorption* — NEW functional-class shape #22: extending established cross-cluster intersection sub-pattern enumerations to integrate newly-absorbed within-cluster sub-shapes from a multi-cycle absorption arc. Cycle 104 first instance integrating cycle 102 cluster A + H sub-shape additions.
- *Session-entry as new boundary type* — cluster A boundary-type distinct from super-step / phase / watchdog / termination-predicate / retry-exhaustion / lane-FIFO. Introduced by cluster A sub-shapes 10 + 11 (cycle 102 absorption); formalized for cross-cluster intersection at cycle 104. Intersects with both cluster B (classification-record write) and cluster C (lifecycle-op selection).
- *Quality-judgment-axis vs aggregation-window distinction* — structural 2-axis decomposition of cluster H sub-shapes. Aggregation-window axis (sub-shapes 1-4): when does feedback fire. Quality-judgment-axis (sub-shapes 5-6): what quality signal is consumed. Cluster H sub-shape catalogue is 2-axis rather than 1-axis.
- *Sub-pattern density per intersection* — count of sub-patterns within a given cross-cluster intersection. Post-cycle-104 distribution: F↔H 7 (densest), A↔B 6, A↔C 6, D↔I 5, B↔C 5, F↔I 5, E↔I 4 (sparsest). Loosely correlates with within-cluster sub-shape richness of the intersected clusters.
- *Substrate-correlation recalibration* — observation that cluster I substrate-correlation argument shifts from "v1 is uniquely positioned" to "v1 inherits cluster I primitives the same way other harness-mediated substrates do" after cycle 102's 4-substrate-type map. Subtle but material adjustment to Phase 2 weighting argument from cycle 89 audit#454 P2.
- *Compositional vs dual-cast sub-pattern ratio* — ratio of compositional (two distinct mechanisms composing at boundary) to dual-cast (one mechanism cast under both cluster labels) sub-patterns. Pre-cycle-104: 35:3. Post-cycle-104: 39:3. Deeper-system absorption tends to add compositional sub-patterns rather than dual-cast ones.

## Cycle 105+ plan

Per `ITERATION-UNTIL-APPROVAL`, cycle 105+ substantive focal options:

1. **Second-iteration sharpening on candidates A/B/C** [cycle 105+, HIGH PRIORITY] — with cycle 102 catalogue update + cycle 103 counting protocol + cycle 104 cross-cluster integration all fed in. Remaining sharpening targets: A's tool-registry-growth risk; B's coordination-overhead-magnitude (now bounded by aggregate threshold but standalone magnitude unmeasured); C's plan-authoring-discipline-conditional risk.
2. **Restored-polarity dispatching** [cycle 105+] — once second-iteration sharpening absorbs into the candidate set.
3. **Symphony deeper-read elevation** [cycle 105+ or based on cluster catalogue findings] — first-pass at cycle 98; could resolve cluster F.9 candidate status.
4. **oh-my-claudecode deeper-read elevation** [cycle 106+ or based on cluster catalogue findings] — first-pass at cycle 99; deeper read would confirm/refute the 3-system substrate-edge convergence observation.
5. **Bounded-mechanical fallback** — close absorbed dispatches if any deliver.

**Cycle 105 substantive focal default:** option (1) second-iteration sharpening on candidates. The cross-cluster integration at cycle 104 provides additional substrate for candidate-specific risk refinement: A's tool-registry-growth risk now has classifier-and-router taxonomy maintenance as a concrete cost; B's coordination-overhead can be tested against the new cross-cluster sub-patterns; C's plan-authoring-discipline can be validated against the session-entry classification as a candidate plan-input. Cross-cluster integration → candidate-side absorption is the natural progression of the absorption arc.

## Pre-commit checklist

- [x] clusters.md edits coherent with cycle 102 sub-shape catalogue annotations
- [x] 1-research.md cross-cluster intersection paragraph sync'd
- [x] Sub-pattern count totals consistent across all references (38 total post-cycle-104)
- [x] No links broken (sub-pattern numbering preserved; existing references to sub-patterns 1-5 within each intersection still valid)
- [x] All new sub-patterns explicitly tagged with "cycle 104 absorption" framing
- [x] v1 failure modes named for each new sub-pattern with concrete v1 evidence
- [x] Phase 2 implications named for each new sub-pattern
- [x] Honest qualifications retained where evidence is partial (e.g., 1-system origin caveats not needed for these 4 sub-patterns since each composes 2-system corpus evidence)
- [x] Verification table prepared for journal entry
