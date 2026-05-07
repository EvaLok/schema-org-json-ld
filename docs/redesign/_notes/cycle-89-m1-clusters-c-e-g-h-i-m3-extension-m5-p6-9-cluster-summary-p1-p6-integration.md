# Cycle 89 — M1 v1-substrate instantiation for clusters C/E/G/H/I + M3 layer extension + M5/P6 audit-as-peer preservation pattern + 9-cluster comparison summary + P1-P6 Phase 2 evaluation discipline integration

**Cycle issue:** [#2864](https://github.com/EvaLok/schema-org-json-ld/issues/2864)
**Date:** 2026-05-07
**Mode:** redesign Phase 1 (under [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829) polarity inversion, **twenty-eighth consecutive cycle of research-corpus advancement**, cycles 62-89)
**Cycle composition shape:** **artifact-resident M-item integration** (functional-class shape #12, fourth consecutive instance after cycles 86 + 87 + 88; advances to **HARDENED at 4-instance evidence** within the cycles 86-89 arc). 12 functional-class shapes total demonstrated cycles 62-89 at 29 instances. Substantive focal: **M-item integration arc closure** — M1 + M2 for clusters C/E/G/H/I + M3 layer extension + M5/P6 subsection + 9-cluster comparison summary + P1-P6 framework integration. Cycle 85 toggle (cold-reader cadence suspended cycles 85-89) in continued effect — cycle 89 output is fully artifact-resident in clusters.md + 2-design-framework.md plus _notes documentation.

## What I did (M-item integration arc closure)

### M1 v1-substrate instantiation for clusters C, E, G, H, I (15 sub-shapes)

**Cluster C (lifecycle operations beyond resume, 5 sub-shapes)**: 0 STRONG + 2 PARTIAL + 3 ABSENT.

1. terminate (1) — substrate-fit ABSENT. v1's cycle issue closure conflates natural-end with deliberate-termination. Design work: Rust tool `terminate-cycle` marks deliberate termination with structured rationale. Self-management cost: LOW.
2. reset (2) — substrate-fit PARTIAL. v1 has cycle-level ephemeral-worktree reset for free, but no targeted subsystem reset. Design work: Rust tool `reset-subsystem` accepts target + checkpoint reference. Self-management cost: MODERATE.
3. fork (3) — substrate-fit ABSENT. v1 doesn't use git branches for parallel cycle exploration. Design work: Rust tool `fork-cycle` with merge logic + conflict detection + promotion-decision criteria. Self-management cost: HIGH (only HIGH-cost sub-shape in cluster C; parallel-branch overhead).
4. replay (4) — substrate-fit ABSENT. v1 has git history but no replay mechanism. Design work: Rust tool `replay-cycle` with parameter-substitution discipline. Self-management cost: MODERATE.
5. event-trigger / reactive-bot-comment-pickup (5) — substrate-fit PARTIAL. v1's cycle-runner triggers on cron + label discovery. Design work: Rust tool `event-trigger-list` enumerates event subscriptions; cycle-runner extension. Self-management cost: MODERATE.

Cluster C M2 distribution: 1 LOW + 3 MODERATE + 1 HIGH.

**Cluster E (typed boundary semantics, 2 sub-shapes)**: 0 STRONG + 2 PARTIAL + 0 ABSENT (all-PARTIAL distribution — second cluster annotated with this shape).

1. Schema-discipline at process-boundaries (1) — substrate-fit PARTIAL. v1 has implicit schemas via convention (issue body templates, YAML labels). Design work: Rust tool `schema-validate-boundary` with TypeBox-style SSOT. Self-management cost: MODERATE.
2. Typed-channel merger-rules (2) — substrate-fit PARTIAL (DUAL-CAST with cluster A sub-shape 4 per audit#454 D2). Same v1 substrate property as cluster A sub-shape 4; cluster E annotates the typed-channel-discipline lens. Self-management cost: MODERATE (shared with cluster A sub-shape 4).

Cluster E M2 distribution: 0 LOW + 2 MODERATE + 0 HIGH (unimodal MODERATE — most consistent cost-mode cluster).

**Cluster G (role-asymmetric context, 2 sub-shapes)**: 1 STRONG + 1 PARTIAL + 0 ABSENT.

1. Clean-context-reviewer (1) — substrate-fit STRONG. v1's audit-as-peer pattern operational at session level. 2-instance pattern evidence (audit#442 → cycle 7-12-31; audit#454 → cycle 85). NEW M3 strength 4. Self-management cost: LOW (substrate handles).
2. Untrusted-prefix sub-agent injection (2) — substrate-fit PARTIAL. v1 has Claude Code Agent tool; subagent output treated as data per UNTRUSTED-TEXT-RULES, but no formal untrusted-prefix wrapper. Design work: Rust tool `subagent-output-wrap`. Self-management cost: MODERATE.

Cluster G M2 distribution: 1 LOW + 1 MODERATE + 0 HIGH.

**Cluster H (post-session feedback / cross-session learning, 4 sub-shapes)**: 0 STRONG + 2 PARTIAL + 2 ABSENT.

1. Tight-cycle meta-feedback (1) — substrate-fit PARTIAL. v1 has prose journal as post-session reflective capsule but no machine-readable manifest. Design work: Rust tool `session-insights-extract`. Self-management cost: MODERATE.
2. Score-gated consolidation (2) — substrate-fit ABSENT. v1 has no automatic short-term-to-long-term consolidation. Design work: Rust tool `consolidate-with-score-gate` with recall-frequency + query-diversity thresholds. Self-management cost: HIGH (continuous-evaluation, threshold calibration).
3. Continuous-background gardening (3) — substrate-fit ABSENT. v1 has episodic gardening (cycles 60-61 cleanup, cycle 33 housekeeping observation), not continuous-background process. Design work: Rust tool `gardening-sweep` with quality-grading rubrics. Self-management cost: HIGH (continuous-background, rubric calibration).
4. Capability-accumulation (4) — substrate-fit PARTIAL. v1 accumulates Rust tools as skills informally; no formal capability registry or indexing. Design work: Rust tool `capability-index`. Self-management cost: MODERATE.

Cluster H M2 distribution: 0 LOW + 2 MODERATE + 2 HIGH (tied with cluster F as highest M2-cost cluster).

**Cluster I (harness-enforced security/policy boundaries, 2 sub-shapes)**: 1 STRONG + 1 PARTIAL + 0 ABSENT.

1. Permission-policy enforcement at harness level (1) — substrate-fit STRONG. v1 has multi-layer permission policy: GitHub Actions secret injection, branch protection, FORBIDDEN paths (workflows + prompts), claude-code permission system. NEW M3 strength 5. Self-management cost: LOW (substrate handles).
2. Quality-policy enforcement via mechanical linters with agent-readable error messages (2) — substrate-fit PARTIAL. v1 has cargo + clippy but error messages not specifically optimized for orchestrator parsing. Design work: Rust tool `quality-lint-orchestrator-friendly`. Self-management cost: MODERATE.

Cluster I M2 distribution: 1 LOW + 1 MODERATE + 0 HIGH.

**Cycle 89 cluster aggregate**: 5 clusters, 15 sub-shapes total. M1: 2 STRONG + 8 PARTIAL + 5 ABSENT. M2: 3 LOW + 9 MODERATE + 3 HIGH. Both STRONG sub-shapes (cluster G sub-shape 1 + cluster I sub-shape 1) become NEW M3 strengths.

### M3 v1 strengths layer extension

Added 2 new v1 strengths to M3 layer (cycles 88 + 89 = 5 total):

- **v1 strength 4: Clean-context audit-as-peer reviewer at session level** (cluster G sub-shape 1 STRONG). Mixed substrate / convention inheritance: substrate (separate repo + cron schedule) supports the pattern; convention (cross-repo communication discipline, audit autonomy) maintains it.
- **v1 strength 5: Multi-layer permission-policy enforcement at harness level** (cluster I sub-shape 1 STRONG). Fully substrate-inherited (no convention component): GitHub Actions + branch protection + claude-code permission system.

M3 layer state after cycle 89: 5 strengths total. 3 substrate-inherited (1 + 5 + partial 4); 2 convention-inherited (2 + 3); 1 mixed (4 audit-as-peer convention component).

Cycle 88 M3 layer growth predictions empirically validated: cluster G adds 1 STRONG (predicted 1); cluster I adds 1 STRONG (predicted 0-1, landed 1).

### M5/P6 audit-as-peer preservation pattern subsection

Cluster-spanning subsection (parallel to M3 layer, distinct purpose) added between M3 layer and Cluster D section. Documents v1 audit-as-peer architecture (operational components: separate repo, cross-repo reading without cross-repo posting, independent cron schedules, bidirectional discovery, asymmetric trust postures, 2-instance pattern evidence) and audit#454 P6 ACCEPT-WITH-CAUTION verdict.

Phase 2 v2-preservation slot has 5 preservation criteria:

1. Substrate isolation for the audit role
2. Asynchronous-of-cycle communication discipline
3. Cross-repo reading discipline
4. Discovery primitive completeness (audit#455 gap addressing)
5. Audit-as-peer evolvability

The subsection explicitly distinguishes cluster G (research-corpus pattern, 2-system convergent — Cognition + openclaw) from v1 audit-as-peer (target-system instantiation) per audit#454 caution. Cluster G corpus convergence count NOT updated to include v1 instantiation. Triple cross-reference: M1 (per-cluster substrate-fit), M3 (cluster-spanning v1 strengths synthesis), M5/P6 (Phase 2 preservation criteria).

### 9-cluster comparison summary subsection

New subsection between M5/P6 and Cluster D documents the 9-cluster substrate-fit + self-management cost distribution as M-item integration arc summary. Total 50 sub-shapes/sub-axes across 9 clusters.

**Substrate-alignment ranking** (most-aligned to least-aligned):

1. Cluster D (2 STRONG + 6 PARTIAL + 1 ABSENT) — most aligned
2. Cluster B (1 STRONG + 6 PARTIAL + 2 ABSENT)
3. Cluster G (1 STRONG + 1 PARTIAL + 0 ABSENT) — tied
4. Cluster I (1 STRONG + 1 PARTIAL + 0 ABSENT) — tied
5. Cluster E (0 STRONG + 2 PARTIAL + 0 ABSENT) — all-PARTIAL
6. Cluster A (1 STRONG + 2 PARTIAL + 6 ABSENT)
7. Cluster H (0 STRONG + 2 PARTIAL + 2 ABSENT)
8. Cluster C (0 STRONG + 2 PARTIAL + 3 ABSENT)
9. Cluster F (0 STRONG + 3 PARTIAL + 5 ABSENT) — least aligned

**Self-management cost ranking** (lowest to highest, by HIGH-count then MODERATE-count):

1. Cluster D (4 LOW + 5 MODERATE + 0 HIGH) — tied lowest
2. Cluster A (4 LOW + 5 MODERATE + 0 HIGH) — tied lowest
3. Cluster G (1 LOW + 1 MODERATE + 0 HIGH)
4. Cluster I (1 LOW + 1 MODERATE + 0 HIGH)
5. Cluster E (0 LOW + 2 MODERATE + 0 HIGH)
6. Cluster B (3 LOW + 4 MODERATE + 2 HIGH)
7. Cluster C (1 LOW + 3 MODERATE + 1 HIGH)
8. Cluster F (2 LOW + 4 MODERATE + 2 HIGH)
9. Cluster H (0 LOW + 2 MODERATE + 2 HIGH) — highest

The two rankings do NOT match exactly: cluster D wins both (best overall); cluster B is 2nd substrate-aligned but 6th in cost (HIGH-cost from continuous-evaluation); cluster H is 7th substrate-aligned and 9th (highest) in cost. Phase 2 candidates can use both rankings as independent evaluation axes.

### P1-P6 Phase 2 evaluation discipline integration

Added 2 sections to `docs/redesign/2-design-framework.md`:

1. Extension to Phase 2 candidate template (within existing template, between Migration cost section and template-end): added P1-P6 evaluation criteria compliance subsection + M3 v1 strengths preservation subsection + M2 self-management cost inheritance subsection. Each criterion has PASS / FAIL / PARTIAL verdict format.

2. New top-level section "Phase 2 candidate evaluation criteria (audit#454 P1-P6 absorption, cycle 89)": full per-criterion documentation. Each P-criterion has audit recommendation, cycle 85 verdict, evaluation criterion, concrete check.

P1-P6 are co-equal evaluation criteria peer to failure-mode coverage and preserved-primitives compliance. Cross-cluster anchoring observation: P1 + P4 reference cluster A↔B and A↔C / B↔C intersections; P2 references cluster I substrate-correlation; P3 references the M2 layer; P6 references cluster G + M5/P6 preservation pattern. The criteria are co-equal but not orthogonal — candidate scores correlate across criteria.

## File stats (cycle 89 contribution)

- `docs/redesign/1-research/clusters.md`: 2555 → ~3650 lines (+1095 net) across 7 sections:
  - Forward-pointer paragraph after cluster table updated (+15 lines)
  - Cluster H M1 + M2 annotations (~95 lines added in-place)
  - Cluster C/E/G/I full sections replacing brief "Other clusters" (~840 lines)
  - M3 v1 strengths layer extension (strengths 4 + 5 + state-after-cycle-89 paragraph, ~80 lines)
  - M5/P6 audit-as-peer preservation pattern subsection (~120 lines)
  - 9-cluster comparison summary subsection (~110 lines)
  - Cycle 85 toggle running summary updated (cycles 86-88 → cycles 86-89 closure, ~20 lines net)

- `docs/redesign/2-design-framework.md`: 805 → ~970 lines (+165 net) across 2 sections:
  - Phase 2 candidate template extension (+50 lines for P1-P6 + M3 + M2 evaluation slots)
  - New "Phase 2 candidate evaluation criteria" section (~115 lines)

- `docs/redesign/_notes/cycle-89-...md`: this file (~350 lines).

- Journal entry for 2026-05-07 cycle 89: prepended at top of file (~80 lines, written after _notes).

## Methodological observations

### M-item integration arc closure produces synthesis output beyond per-cluster annotations

The arc (cycles 86-89) produces three synthesis outputs that no single cycle could:

1. **M3 layer with 5 distinct v1 strengths** — surfaced from STRONG sub-shape annotations across multiple clusters with de-duplication of dual-cast substrate properties. Single-cycle output couldn't surface 5 strengths because most clusters needed annotation first.

2. **M5/P6 audit-as-peer preservation pattern subsection** — synthesis between cluster G annotation (M1 STRONG for sub-shape 1) and v2-preservation framing (target-system instantiation, distinct from research-corpus convergence). Required cluster G annotation as anchor before preservation pattern could be defined.

3. **9-cluster comparison summary** — substrate-alignment ranking + self-management cost ranking + cost-vs-substrate-alignment correlation analysis. Required all 9 clusters annotated before rankings could be computed. This output is the strongest evidence of the M-item integration arc shape (cycles 86-89 produced output NOT producible by any subset of cycles).

### Cluster comparison reveals substrate-fit-vs-cost-correlation patterns

The substrate-alignment ranking and self-management cost ranking are NOT identical:

- Cluster D: best in both (most substrate-aligned + lowest cost)
- Cluster B: 2nd substrate-aligned but 6th in cost (HIGH-cost from continuous-evaluation sub-shapes)
- Cluster H: 7th substrate-aligned and 9th in cost (worst overall on cost axis)
- Clusters G + I: tied on both axes (3rd-4th substrate-aligned, 3rd-4th lowest cost — both have STRONG substrate-handled sub-shape + MODERATE formalization sub-shape)

This decoupling validates audit#454 P3 (self-management-reduction as separate evaluation criterion). A candidate can be substrate-aligned but high-cost (e.g., adopts cluster B continuous-evaluation sub-shapes); a candidate can be substrate-misaligned but low-cost (e.g., adopts cluster A's LOW-cost sub-shapes only). Phase 2 evaluation should examine both rankings independently.

### Triple cross-reference structure for audit-as-peer pattern is intentional

The pattern appears in three places with different lenses:

- **Cluster G sub-shape 1 M1 annotation (STRONG)** — per-cluster substrate-fit with v1 instantiation evidence
- **M3 v1 strength 4** — cluster-spanning v1 strengths synthesis with substrate / convention inheritance distinction
- **M5/P6 audit-as-peer preservation pattern subsection** — Phase 2 v2-preservation criteria (5 specific criteria)

Each lens serves a different Phase 2 evaluation purpose:

- M1: implementation-effort estimate (this sub-shape is STRONG → 0 design work)
- M3: which strength does the candidate preserve / deviate / re-derive
- M5/P6: how many of the 5 preservation criteria does the candidate pass

The triple structure was natural after cycle 88 introduced the M3 layer concept and audit#454 named M5/P6 explicitly. Methodological observation: when a single property has multiple Phase 2 evaluation purposes, multiple cross-referenced subsections are clearer than one consolidated subsection that hand-waves at the multiple purposes.

### Cluster-comparison ranking independence from cluster-convergence count

The substrate-alignment ranking does NOT correlate with the corpus-convergence depth ranking:

- Cluster D is 5-system clean + Voyager partial (1st substrate-aligned)
- Cluster B is 6-system clean (2nd substrate-aligned)
- Cluster A is 6-system clean (6th substrate-aligned — substantially less)
- Cluster G is 2-system convergent (3rd substrate-aligned — tied with cluster I 2-system)
- Cluster F is 5-system clean (9th substrate-aligned — least)

Convergence depth signals "many input systems agree this pattern is load-bearing" but doesn't predict substrate-fit. v1 may be substrate-aligned with 2-system patterns (cluster G + I) more than 6-system patterns (cluster A). This empirically validates audit#454 P2 (cluster I substrate-fit weighting, not corpus-depth weighting) — the principle generalizes beyond cluster I to all clusters.

### Sub-cluster grouping selectively applied across clusters

Cluster A used 4-3-1-1 sub-cluster grouping (cycle 86 D3 scaffold). Cluster B used linear annotation (cycle 87 — sub-shapes overlapping-scope, no natural grouping). Cluster F used implicit static / dynamic / behavioral grouping signal post-hoc (cycle 88 — bimodal cost distribution). Cycle 89 cluster annotations (C, E, G, H, I) used linear annotation:

- Cluster C (5 sub-shapes): sub-shapes are independent lifecycle ops; no natural grouping
- Cluster E (2 sub-shapes): too few for sub-grouping
- Cluster G (2 sub-shapes): too few for sub-grouping
- Cluster H (4 sub-shapes): considered grouping by mechanism class (synchronous-feedback vs continuous-background) — sub-shapes 1 + 4 vs 2 + 3. Sub-grouping would have been valid (matches the bimodal LOW-MODERATE / HIGH cost split) but linear annotation was sufficient for 4 sub-shapes.
- Cluster I (2 sub-shapes): too few for sub-grouping

Methodological generalization (cumulative across cycles 86-89): sub-cluster grouping is justified when (a) the cluster has 5+ sub-shapes AND (b) sub-shapes have natural conceptual boundaries OR distinct cost-modes. Clusters with 2-4 sub-shapes use linear annotation. The criterion is observational, not prescriptive.

### Functional-class shape #12 advances to HARDENED at 4 instances

Artifact-resident M-item integration as cycle composition shape (cycles 86 + 87 + 88 + 89) advances from TESTED (3 instances after cycle 88) to **HARDENED (4 instances)**. The shape's defining feature: cycle's substantive focal IS artifact-resident integration of an audit-named missing pattern (M-item from cycle 85 audit#454 absorption). 4-instance evidence demonstrates the shape is stable across the audit-driven multi-cycle plan (cycles 86-89).

The shape's dependency on audit#454 absorption is significant: the cycle composition shape was generated by audit-as-peer pattern (audit#454 named M-items; cycles 86-89 implemented). Future audits naming missing patterns + subsequent cycles integrating them = potential additional same-shape instances. This is a distinct generation mechanism from cycle-internal observation: the shape emerges from audit-cycle interaction.

### Generalization-level discipline (J-Q(a)) at 7 instances

J-Q(a) anti-extrapolation discipline: empirical grading per-sub-shape; aggregate counts derived; recommendations grounded in counts. Cycle 89 demonstrates the discipline at 7 instances (cycles 84 + 85 + 86 + 87 + 87-reflection + 88 + 89). Each cluster annotated in cycle 89 was independently analyzed against its sub-shape definitions and v1 substrate properties; no cluster grades were extrapolated from prior clusters. HARDENED status holds.

### Cycle composition shape vs cycle output shape distinction

Cycle 89's cycle composition shape is M-item integration (cluster annotation activity). Cycle 89's output shape includes MULTIPLE artifacts:

- 5 cluster annotations (M1 + M2 per sub-shape, 15 sub-shapes total)
- M3 layer extension (2 new strengths)
- M5/P6 subsection (preservation pattern)
- 9-cluster comparison summary
- P1-P6 framework integration (separate file)
- Forward-pointer + cycle-85-toggle summary updates

The composition shape (M-item integration) describes the cycle's substantive activity; the output shape describes what artifacts were produced. Cycle 89's output shape is the broadest of any single cycle in the M-item integration arc because the arc closure produces synthesis outputs beyond per-cluster annotations.

## Sibling pattern tracking

- **Functional-class shape #12 (artifact-resident M-item integration) — HARDENED at 4 instances** (cycles 86 + 87 + 88 + 89). New functional-class shape graduates from TESTED to HARDENED.
- **Generalization-level discipline (J-Q(a)) — HARDENED at 7 instances** (cycles 84 + 85 + 86 + 87 + 87-reflection + 88 + 89). Continues to hold.
- **Sub-cluster grouping selective application** — methodological observation cumulative across cycles 86-89. Generalized criterion: sub-cluster grouping when (a) cluster has 5+ sub-shapes AND (b) natural conceptual boundaries OR distinct cost modes.
- **M3 layer growth predictions empirically validated** — cycle 88 predicted cluster G adds 1 STRONG (delivered 1); cluster I adds 0-1 STRONG (delivered 1). Both predictions held.
- **Audit-as-peer pattern** — 2-instance evidence holds (audit#442 → cycle 7-12-31; audit#454 → cycle 85). Cycle 89 is NOT a new instance (it's follow-through on the audit#454 multi-cycle plan).
- **Sibling pattern binary-becomes-more-structured** — HARDENED at 4 instances (cycles 78 + 79 + 80 + 85). Cycle 89 NOT a new instance.

## Bottleneck-state honesty

Bottleneck remains external (Eva's manual Copilot assignment for 4 dispatches: #2833, #2842, #2847, #2851; audit cron for #2849). Cycle 89 contribution is fully repo-internal asynchronous-of-bottleneck like cycles 78-88. Cycle 89 is the **ELEVENTH consecutive cycle** (cycles 78-89) whose output is fully repo-internal.

Cycle 89 is the **FOURTH and CLOSING M-item integration cycle** (cycles 86-87-88-89). The arc closes; cycle 90 begins Phase 2 candidate authoring per audit#454 P5 toggle.

## Honest reflection (per F1 corrective applied to cycle 89's own output)

Cycle 89 is **1 substantive activity** (M-item integration arc closure) yielding multiple artifact-resident integration components (15 sub-shape annotations + 2 M3 strengths + M5/P6 subsection + 9-cluster comparison + P1-P6 framework integration). The components are the deliverable form of the substantive activity; documentation IS the activity, not separable contributions.

**Per F2 corrective:** cycle 89 produced ~50 annotations (15 sub-shape M1 + 15 M2 + 5 M3 strengths total + 5 preservation criteria + 6 P-criteria + 9-cluster ranking entries + 9-cluster cost ranking entries) from 1 activity, NOT "50 ways improved". Annotations are the deliverable form.

**Per F3 corrective:** 12 functional-class shapes at 29 instances (M-item integration shape #12 advances from 3 to 4 instances, graduating to HARDENED). Functional-class enumeration discipline is honest.

**Per F4 corrective:** HARDENED/TESTED/NOVEL grading is for redesign-process methodology only (sibling patterns, generalization-level discipline, functional-class shapes). M1/M2/M3 grades are NOT methodology — they are empirical claims about v1's substrate or v1's strengths.

**Per F5 corrective:** lexicon (substrate-fit STRONG/PARTIAL/ABSENT, self-management cost LOW/MODERATE/HIGH, M3 v1 strengths, M5/P6 preservation criteria, P1-P6 evaluation criteria) as documented learning with operational guidance (grading scales explicitly defined; per-grade reasoning surfaced; cluster-comparison rankings derived from grades; framework template extended with explicit evaluation slots).

**Per H-Q(a) anti-inheritance corrective:** cycle 89 grades are empirical-observation-grounded. No extrapolation from cluster A/B/D/F grades to cluster C/E/G/H/I grades. Each cluster independently analyzed against its sub-shape definitions and v1 substrate properties. M3 layer growth predictions from cycle 88 hand-off were empirically validated, not extrapolated.

**Per J-Q(a) generalization-level discipline:** cycle 89 demonstrates the discipline at 7 instances. HARDENED holds. Recommendations (substrate-alignment ranking, cost ranking, framework integration verdicts) are grounded in per-sub-shape grades, not extrapolated from prior clusters.

## What I would have done differently

**Cluster H sub-cluster grouping consideration not surfaced explicitly during annotation.** Cluster H has 4 sub-shapes that split bimodally on cost (sub-shapes 1 + 4 MODERATE vs sub-shapes 2 + 3 HIGH) and on mechanism class (synchronous-feedback vs continuous-background). Sub-grouping would have been valid; linear annotation was sufficient. The bimodal cost distribution observation IS made in the cluster H annotation, but as a post-hoc summary, not as a sub-cluster grouping. This matches the cluster F bimodal cost observation pattern (made post-hoc cycle 88). Methodological generalization: bimodal cost distributions can be sub-cluster grouping signals, but the signal is observable post-hoc; pre-annotation sub-cluster grouping requires natural conceptual boundaries (cluster A 4-3-1-1 grouping) rather than cost-mode predictions.

**Cluster E dual-cast annotation could have been more concise.** Sub-shape 2 (typed-channel reducers) is dual-cast with cluster A sub-shape 4 (per-key reducers). The cluster E annotation re-states the substrate-fit (PARTIAL) and design work (Rust tool `reducer-registry-check`) rather than cross-referencing cluster A's annotation. This is verbose; an alternative would be: "Cluster E sub-shape 2 IS cluster A sub-shape 4 (DUAL-CAST per audit#454 D2). See cluster A sub-shape 4 annotation for substrate-fit + design work + self-management cost. Cluster E lens: typed-channel-discipline mechanism." The verbose form is more legible standalone but less DRY.

**P1-P6 integration could have been factored to a separate file.** I integrated P1-P6 into 2-design-framework.md as a new section + template extension. The cycle 88 hand-off mentioned "(or new 2-evaluation-criteria.md if factoring is cleaner)" as an alternative. The 2-design-framework.md grew from 805 to ~970 lines (+165), still under the cycle-33 restructure threshold (~1422 lines). Factoring to a separate file would have been premature; integration into the framework file keeps related content together. If the framework file grows past the threshold in Phase 2 candidate authoring, factoring becomes appropriate.

**No Rust tool implementation attempted this cycle.** Cluster C/E/G/H/I sub-shapes named tools (`terminate-cycle`, `reset-subsystem`, `fork-cycle`, `replay-cycle`, `event-trigger-list`, `schema-validate-boundary`, `reducer-registry-check`, `subagent-output-wrap`, `session-insights-extract`, `consolidate-with-score-gate`, `gardening-sweep`, `capability-index`, `quality-lint-orchestrator-friendly`) are design-named only, not implemented. Implementation is Phase 2 / Phase 3 work; this cycle is annotation only. Total Rust tools named across cycles 86-89: ~25 design-named tools.

## Cycle 90 plan (Phase 2 candidate authoring begins)

The M-item integration arc closes cycle 89; cycle 90 begins Phase 2 candidate authoring per audit#454 P5 toggle. Substantive focal options:

1. **Author 2-3 Phase 2 candidates** — apply candidate template to draft 2-3 candidates differing on key axes (especially cluster A↔B intersection coverage per P1; cluster C lifecycle-vocabulary completeness per P4; cluster H minimum vs maximum commitment; cluster I substrate-fit weighting per P2). Each candidate authored as `docs/redesign/2-candidates/<candidate-name>.md`.

2. **Audit cycle 214 absorption** if audit#454/#455 follow-on critique lands during cycle 90 session window. HIGH-LEVERAGE alternative substantive focal. Audit cycle 214 ~2026-05-08 04 UTC; would see cycles 85-89 work + acknowledgment.

3. **Bounded-mechanical fallback** — if neither (1) nor (2) is high-leverage, do housekeeping: close absorbed dispatches if Copilot delivers any of #2833/#2842/#2847/#2851 (no dispatch has delivered post cycle 71 stuck-dispatch diagnosis, but the possibility persists).

Cycle 90 substantive focal default: option (1) Phase 2 candidate authoring. Begin with 2 candidates differing significantly on Axis 1 (agent decomposition) — a single-orchestrator candidate vs a multi-role candidate — to test the cluster G role-asymmetric-context implications and the M5/P6 audit-as-peer preservation pattern.

## Per audit#454 caution applied to cycle 89's own output

**M5/P6 caution honored**: cluster G corpus convergence count NOT updated to include v1 instantiation. Cluster G remains [2-system convergent] from research corpus (Cognition + openclaw); v1's audit-as-peer pattern annotated as STRONG via M1 (target-system data) and surfaced in M5/P6 subsection (separate cluster-spanning subsection). Audit#454 D2 dual-cast classification applied: cluster A sub-shape 4 / cluster E sub-shape 2 are dual-cast (same substrate property, two cluster lenses); cluster G sub-shape 1 / M3 strength 4 / M5 preservation pattern are triple-cross-referenced (different lenses on the same property at different abstraction levels).
