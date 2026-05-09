---
name: cycle-105 author A Risk 3 sharpening
description: Cycle 105 second-iteration sharpening of A's Risk 3 (tool-registry-growth) absorbing cycle 102 cluster A sub-shapes 10+11 + cluster H sub-shapes 5+6 substrate plus cycle 104 A↔B-6 cross-cluster sub-pattern; specifies active-callable-vs-inventory distinction with ≤12 active-callable bound as falsifiable candidate-claim
type: redesign-process
---

# Cycle 105 — Author A Risk 3 second-iteration sharpening (post-cycle-102/104 substrate absorption)

**Cycle:** redesign cycle 105 (2026-05-09)
**Source:** Risk 3 in [`docs/redesign/2-candidates/A-evolved-single-orchestrator.md`](../2-candidates/A-evolved-single-orchestrator.md) (named cycle 90 authoring); first sharpening cycle 97 (PR #2877 lens-7 absorption); second-iteration sharpening cycle 105.
**Purpose:** Close A's tool-registry-growth gap at the specification level, parallel to cycle 103's closure of B's Risk 8 (counting-protocol) at specification level. Three candidate-distinguishing tool-adoption decisions surface from cycle 102 sub-shape additions and cycle 104 cross-cluster integration; this sharpening makes the resulting tool-count delta concrete and specifies the mitigation as a falsifiable bound.

## Setup

Risk 3 has been the most-deferred remaining sharpening target across A's risks since cycle 96:

- **Cycle 96** PR #2878 absorption surfaced Risk 0 (baseline-measurement) and re-grounded Risks 1-2; Risk 3 was partially sharpened cycle 97 with PR #2877 lens-7 absorption (super-linear threshold ~18-25) but the mitigation remained conditional ("if surface grows beyond ~15 actively-orchestrator-called tools").
- **Cycles 97-103** prioritized PR-absorption arc + cluster catalogue rebuild + B counting protocol over A Risk 3 second-iteration — research-corpus advance was the polarity default per `CYCLE-COMPOSITION-POLARITY`, and B's threshold-asymmetry was the more urgent falsifiability gap.
- **Cycle 104** completed cross-cluster intersection updates (A↔B-6 + F↔H-6/7 + A↔C-6) absorbing cycle 102 cluster A sub-shapes 10+11 and cluster H sub-shapes 5+6 into cross-cluster integration. A's Risk 3 became the highest-priority remaining sharpening item with the substrate now consolidated.
- **Cycle 105** absorbs cycle 102/104 substrate into A's Risk 3 specifically: tool-count delta from sub-shape adoption + active-callable-vs-inventory distinction + active-callable bound as falsifiable claim.

The deferral arc was 8 cycles (97-104). Per the cycle 103 pattern observation, the deferral was load-bearing protection rather than procrastination: A Risk 3 second-iteration could not have been substantive before cycle 104 because the cluster B coordinated-write target (A↔B-6) was not yet integrated into the catalogue. The cycle 102 cluster A sub-shape additions alone were not sufficient — cycle 104 made the cluster B side of the classify-then-route pipeline explicit, which is the substrate cycle 105 absorbs.

## Design choices made

### Choice 1: Active-callable surface vs total inventory as distinct quantities

The cycle 90 framing of Risk 3 conflated "tool count" with "what the orchestrator has to discriminate among" — both are simply called "tool surface." Cycle 105 separates these:

- **Total inventory** = all v2 Rust crates that exist in `tools/rust/crates/v2-*/`. Grows monotonically with sub-shape adoption.
- **Active-callable surface** = the subset of tools whose names/contracts the orchestrator-prompt enumerates and that the orchestrator selects among at runtime. Bounded by harness-internalization choices.

PR #2877 lens-7's super-linear threshold (~18-25) applies to the active-callable surface, not the inventory. The orchestrator's discrimination burden (which tool to invoke for which situation) is the load-bearing quantity; the inventory count contributes to disk usage / build time / dependency footprint but not to per-cycle decision overhead.

**Alternative considered and rejected:** treat the inventory directly as the load-bearing quantity. Rejected because it conflates two failure modes (build-time / dep-footprint vs runtime-discrimination) and inflates A's apparent risk where no real failure mode exists.

### Choice 2: Harness-internalization as the bounding mechanism

Under A's Axis 13 medium-harness extraction, orchestration-hub crates (`boot-phase`, `close-phase`) absorb sub-shape-specific tools as INTERNAL pipeline stages:

- **Boot-phase** internalizes `classify-cycle-task` (cluster A sub-shape 10) + `route-cycle-decision` (cluster A sub-shape 11) + cluster B classification-record write (A↔B-6 sub-pattern 6). The orchestrator sees a single `cycle-context.json` artifact bundling the resolved task classification + route + boundary state.
- **Close-phase** internalizes `infer-feedback-signal` (cluster H sub-shape 5) + `evaluator-driven-consolidate` (cluster H sub-shape 6). The orchestrator sees a `cycle-feedback-summary.json` artifact from close-phase output.

This keeps the orchestrator-callable surface at ≤9 (the cycle 90 baseline) even when the inventory grows to ~15 with full sub-shape adoption.

**Alternative considered and rejected:** expose all sub-shape tools at the orchestrator-callable tier, paying registry-one-liner failure cost. Rejected because the cycle 102 sub-shapes are sequential pipeline stages within a phase boundary (classify-then-route is one pipeline; feedback-inference + evaluator-driven keep-discard are sequential post-cycle activities), and the orchestrator does not need to choose among them at runtime — the pipeline shape is fixed at design time.

**Alternative considered and rejected:** harness-internalize ALL sub-shape tools (not just cycle 102/104 additions). Rejected because some sub-shape tools are genuinely orchestrator-selected at runtime (`detect-abandoned-cycles` runs only when watchdog triggers; `gardening-sweep` runs at a different cadence than boot/close phases). Harness-internalization is appropriate for sub-shape tools that form pipelines within a phase boundary, not for tools called at orthogonal moments.

### Choice 3: Active-callable bound as falsifiable candidate-claim

A commits to ≤12 orchestrator-callable tools across all sub-shape adoptions. This bound is:

- **Falsifiable**: at Phase 3 prototype, count the v2 prompt's tool-name references after cycle 102/104 substrate adoption; verify ≤12.
- **Pre-agreed**: the threshold is named cycle 105, before measurement. Per the cycle 96 discipline-bar-too-low lesson, hard refutation thresholds must be pre-agreed.
- **Bounded above by the super-linear threshold (~15-25)**: ≤12 stays below the lower edge of the super-linear band, leaving margin for ~3 additional adoptions before the bound itself becomes load-bearing.

**Alternative considered and rejected:** threshold at the registry-failure edge (~15). Rejected because that leaves no margin — any single additional sub-shape adoption beyond cycle 102/104's substrate would push past the bound. ≤12 is more conservative and more falsifiable.

**Alternative considered and rejected:** no hard bound, only "monitor and react." Rejected because that is exactly the cycle 96 discipline-bar-too-low pattern (soft validation language) being rejected at the candidate-set level.

### Choice 4: Two-tier registry as the mitigation specification

The cycle 97 mitigation ("hierarchical registry + compatibility metadata") was conditional and abstract. Cycle 105 specifies it as:

1. Two-tier structure: orchestrator-callable tier (≤12) + harness-internal tier (sub-shape-specific tools called by orchestration-hub crates). The orchestrator-prompt references only the callable tier.
2. By-cluster registry grouping: callable-tier registry output groups tools by cluster (A / B / F / H / I / cross-cutting) rather than alphabetical, reducing selection-among-N to selection-among-N-per-cluster.
3. Compatibility metadata as I/O typed declarations: each tool manifest declares input/output JSON shape. Allows the orchestrator to discover tool fit by data-shape rather than by name disambiguation.

**Alternative considered and rejected:** single flat registry with one-line descriptions. Rejected per PR #2877 lens-7: at ~15+ callable units, one-line descriptions are insufficient for safe invocation selection.

**Alternative considered and rejected:** prompt-embedded full registry (each tool's manifest inlined into the orchestrator-prompt). Rejected because it inflates prompt size and trades one form of growth for another.

### Choice 5: A↔B-6 cluster B writers as 0-2 tools, candidate-design choice

Cycle 104's A↔B-6 sub-pattern names cluster B coordinated writes (classification-record + route-record) at the session-entry boundary. The cycle 105 sharpening admits this as 0-2 tools depending on candidate design:

- **0 additional tools**: the per-component-state writer A already adopts at cycle 90 (`boot-phase` writes per-component state files at boot boundary) absorbs `classification-record.json` + `route-record.json` as additional file targets. No new tool, just additional file targets for an existing writer.
- **1-2 additional tools**: dedicated `classification-record-write` and/or `route-record-write` crates if the A↔B-6 sub-pattern requires write semantics distinct from per-component-state writer (e.g., transactional coordination across the two records).

A defaults to 0 additional tools (extension of existing writer). The 0-vs-1-2 choice is a candidate-design decision that becomes load-bearing only if A adopts cluster A sub-shapes 10+11 AND the A↔B-6 sub-pattern requires write semantics beyond append-only per-component file writes.

**Alternative considered and rejected:** mandate dedicated writers for A↔B-6. Rejected because the cluster B classification-record + route-record are already structurally similar to A's existing per-component-state files (typed JSON written at boundary moments).

## Tool-count delta computation

| Adoption decision | Tool delta | Self-management cost | Cycle 90 baseline coverage |
|---|---|---|---|
| Cluster A sub-shape 10 (classifier-mediated dispatch) | +1 (`classify-cycle-task`) | MODERATE | NEW (post-cycle-90 substrate) |
| Cluster A sub-shape 11 (deterministic-decision-tree routing) | +1 (`route-cycle-decision`) | LOW | NEW (post-cycle-90 substrate) |
| A↔B-6 cluster B coordinated write | +0 to +2 | LOW | NEW (post-cycle-104 substrate) |
| Cluster H sub-shape 5 (feedback-signal-inference) | +1 (`infer-feedback-signal`) | MODERATE | NEW (post-cycle-90 substrate) |
| Cluster H sub-shape 6 (evaluator-driven keep-discard) | +1 (`evaluator-driven-consolidate`) | MODERATE | NEW (post-cycle-90 substrate) |

**Trajectory:**

| Adoption posture | Inventory | Active-callable | Cost summary |
|---|---|---|---|
| Floor (none of the new sub-shapes) | 9 v2 crates | 9 | Cycle 90 baseline preserved |
| Classify-then-route pipeline only (10 + 11 + minimal A↔B-6 absorption) | ~11 v2 crates | 9 (boot-phase internalizes) | +1 MODERATE + 1 LOW; bound preserved |
| Full adoption (10 + 11 + dedicated A↔B-6 + 5 + 6) | ~14-15 v2 crates | 9 (boot-phase + close-phase internalize) | +3 MODERATE + 1 LOW; bound preserved |

Active-callable stays ≤12 across all postures via harness-internalization. The ≤12 bound is the candidate-claim measurable at Phase 3 prototype.

## Edits made

### A-evolved-single-orchestrator.md

Risk 3 entry extended from 1 paragraph (cycle 90 + cycle 97) to ~5 sub-sections (cycle 90 + cycle 97 + cycle 105):

- Cycle 105 sharpening header
- Adoption-decision table (5 rows × 3 columns)
- Trajectory bounds (4 named bounds)
- Cross-reference to PR #2877 lens-7 thresholds
- Active-callable vs total-inventory distinction (introduces the structural reframing)
- Mitigation specification (4 numbered specifications: two-tier registry, by-cluster grouping, compatibility metadata, active-callable bound)
- Risk 3 status post-cycle-105 (direction-vs-magnitude verdict + falsifiability claim)

Net addition: ~70 lines. No content removed; all cycle 90 + cycle 97 framings preserved.

### README.md (2-candidates)

Forward work entry updated:

- Cycle 105 closure of A's Risk 3 named alongside cycle 103's closure of B's Risk 8.
- Second-iteration sharpening remaining targets reduced from 3 to 2: B's coordination-overhead-magnitude + C's plan-authoring-discipline-conditional risk.

Net addition: ~2 lines (one updated bullet).

### Not modified

- A's cycle 91 Axis 13 sharpening section — out of scope; cycle 105 sharpens Risk 3, not the primary load-bearing claim.
- A's cycle 93+94 prototype scaffolding section — out of scope; cycle 105 is specification-level closure, not prototype measurement.
- A's M2 self-management cost inheritance section — cycle 105's adoption-decision table adds tool-count delta but does NOT modify A's M2 LOW/MODERATE/HIGH adoption profile (the ≤12 bound is on tools, not on sub-shape adoption count).
- A's M3 v1 strengths preservation — out of scope.
- B's Risks — cycle 105 substantive focal is A's Risk 3.
- C's Risks — cycle 105 substantive focal is A's Risk 3.
- 2-design-framework.md — cycle 105 is candidate-specific sharpening, not framework-level.
- clusters.md — cycle 102 catalogue + cycle 104 cross-cluster intersections already integrated; cycle 105 absorbs from these into A, does not modify them.
- cluster catalogue — staleness remains RECENT (no new deep-dive landings between cycle 104 and cycle 105).
- README.md load-bearing claims tracker rows for A/B/C — cycle 105 sharpens a SECONDARY risk (Risk 3), not A's primary load-bearing claim (Axis 13 medium-harness extraction). The tracker rows track primary claims; Risk 3 sharpening is a secondary-risk closure noted in Forward work.

## Verification

```
$ wc -l docs/redesign/2-candidates/A-evolved-single-orchestrator.md
~358 docs/redesign/2-candidates/A-evolved-single-orchestrator.md  # was 288
```

Cycle 90 + cycle 97 framings of Risk 3 preserved verbatim:
- "the v2 prompt's tool-registry reference may grow substantially as the tool count grows" — preserved
- "Cycle 97 sharpening (PR #2877 lens-7): coordination grows super-linearly past ~15-25 callable units" — preserved
- "Mitigation: hierarchical registry + compatibility metadata if surface grows beyond ~15 actively-orchestrator-called tools" — preserved (cycle 105 specifies what hierarchical-registry-plus-compatibility-metadata MEANS concretely)

Cycle 105 additions are layered on top via "Cycle 105 second-iteration sharpening" sub-section header — the cumulative-layering pattern matches B's cycle 96 → cycle 103 layering on Risk 8 and A's cycle 91 → cycle 96 layering on Axis 13 sharpening.

## What surprised me / what I noticed

1. **The active-callable-vs-inventory distinction is genuinely load-bearing, not just a re-framing.** The cycle 90 framing of Risk 3 conflated tool count with discrimination burden. PR #2877 lens-7's threshold language ("registry one-liners are insufficient for safe invocation selection") is unambiguously about SELECTION SURFACE — what the orchestrator has to choose among — not about INVENTORY count. Cycle 105 makes this distinction explicit and the implications follow cleanly: harness-internalization of sub-shape pipelines keeps selection surface bounded while inventory grows. This is structurally analogous to cluster F sub-axis 6 (cost-tier stratification — Sonnet for classification, Opus for execution): the load-bearing quantity is which decisions reach the expensive surface, not how many tools exist. 

2. **The deferral arc was 8 cycles (97-104), longer than B's Risk 8 deferral arc (7 cycles, 96-102).** Cycle 103 observed that 7-cycle deferral was load-bearing protection. Cycle 105 corroborates: A's Risk 3 second-iteration could not have been substantive before cycle 104 made the cluster B side of the classify-then-route pipeline explicit. Without cycle 104's A↔B-6 sub-pattern, the cycle 105 sharpening would have lacked the cluster B coordinated-write substrate and would have been weaker. **Pattern continuation: deferral driven by `CYCLE-COMPOSITION-POLARITY` polarity discipline produces load-bearing protection, not procrastination, at multi-instance evidence (B Risk 8 cycle 103 + A Risk 3 cycle 105).**

3. **Both candidate-set falsifiability milestones close at specification level, deferring operational closure to Phase 3.** Cycle 103: B's Risk 8 closes at specification level (counting protocol specified; operational closure deferred to Phase 3 prototype measurement). Cycle 105: A's Risk 3 closes at specification level (active-callable bound specified; operational closure deferred to Phase 3 prototype tool-name-reference count). The specification-vs-operational closure stratification (NOVEL functional-class shape #21 introduced cycle 103) extends to a second instance at cycle 105.

4. **The ≤12 active-callable bound has structural symmetry with B's PASS ≤ 1.0× v1 baseline threshold.** B's threshold is a ratio (aggregate-vs-baseline); A's threshold is an absolute count. Both are falsifiable at Phase 3 prototype with explicit measurement procedures (B counts structural-decision-points in role artifacts; A counts tool-name references in v2 prompt). Both have margin built in (B's 1.0× allows aggregate to match v1; A's 12 allows ~3 additional adoptions beyond cycle 102/104 substrate). **Pattern observation: pre-agreed hard thresholds for candidate-specific risks emerge as a consistent shape across candidates when the cycle 96 discipline-bar-too-low lesson is applied symmetrically.**

5. **Cycle 105 is the third instance of risk-closure-at-specification-level (NOVEL functional-class shape #21).** Cycle 103 first instance (B Risk 8). Cycle 105 second instance (A Risk 3). Plus cycle 103's Risk 7 partial-closure observation (the protocol's external-observer requirement extends to v1 baseline measurement) — this is a coupled partial-instance, not a clean separate instance. Promotes shape #21 from NOVEL at 1 to TESTED at 2 instances (or 2.5 with the cycle 103 partial). The pattern is becoming sustainable methodology across the candidate set.

## Sibling pattern tracking

- **NOVEL functional-class shape #21 — `risk-closure-at-specification-level`** — cycle 103 first instance (B Risk 8); cycle 105 second instance (A Risk 3). **Promotes NOVEL at 1 instance → TESTED at 2 instances.**
- **Functional-class shape #22 — `cross-cluster-intersection-extension-via-sub-shape-absorption`** — cycle 104 first instance. Cycle 105 NOT a new instance; cycle 105 is a candidate-side absorption of cycle 104's cross-cluster integration.
- **Functional-class shape #20 (catalogue-rebuild-with-multi-cycle-absorption)** — cycle 102 first instance. Cycle 105 NOT a new instance.
- **Functional-class shape #19 (research-deliverable-absorption-with-verification)** — HARDENED at 3 verification-success + 1 verification-mixed instances. Cycle 105 NOT a new instance.
- **Verification-discipline pattern (cycle 96 emergence)** → **6 instances HARDENED**. Cycle 105 NOT a new instance.
- **Pre-agreed-falsification-threshold discipline** (cycle 96 emergence per PR #2878 Finding 18) → cycle 105 application is the **FOURTH instance** (A primary cycle 96 / C cycle 96 / B cycle 103 / A Risk 3 cycle 105). All three candidates have pre-agreed thresholds at multiple risk levels. **HARDENED at 4 instances** spanning candidate primary claims and candidate-specific risks.
- **Generalization-level discipline (J-Q(a))** → cycle 105 application: the active-callable-vs-inventory distinction is a generalization-level decision named explicitly with reasoning (PR #2877 lens-7's super-linear threshold applies to selection surface, not inventory count). **20 instances HARDENED** (extends from 19).
- **Direction-vs-magnitude discipline** → **12 instances HARDENED** (extends from 11). Cycle 105 application: direction (tool-registry growth is bounded by structural decision) confirmed at 1 sustained instance; magnitude (≤12 active-callable) is the falsifiable bound to be measured at Phase 3.
- **Cycle-composition-polarity discipline (cycle 62 emergence)** → 41+ instances HARDENED. Cycle 105 substantive focal IS "v1-system retrospective extraction of lessons not yet absorbed into the redesign" applied to A's cluster 102/104 substrate — the cycle 96 discipline-bar-too-low lesson applied to A's secondary-risk sharpening. Cycle 105 application of polarity discipline.
- **Specification-vs-operational closure stratification** (cycle 103 emergence) → cycle 105 second instance. Both A Risk 3 and B Risk 8 close at specification level with operational closure deferred to Phase 3. **HARDENED at 2 instances.**
- **Audit-as-peer pattern** — 2-instance evidence holds. Audit cycle status not measured this cycle (cycle 104 noted ~50+ hours since cycle 212 cycle-101 measurement; cycle 105 does not re-measure). Cycle 105 NOT a new instance.

## Lexicon entries (cycle 105)

- *Active-callable surface* — the subset of tools whose names/contracts the orchestrator-prompt enumerates and that the orchestrator selects among at runtime. Distinct from total inventory (all v2 Rust crates that exist). Load-bearing for PR #2877 lens-7 super-linear threshold (~15-25); inventory is not load-bearing for this threshold.
- *Total inventory* — all v2 Rust crates that exist in `tools/rust/crates/v2-*/`. Grows monotonically with sub-shape adoption. Bounds disk usage / build time / dependency footprint but not per-cycle decision overhead.
- *Harness-internalization* — design pattern where orchestration-hub crates (boot-phase, close-phase) absorb sub-shape-specific tools as INTERNAL pipeline stages, keeping orchestrator-callable surface bounded while inventory grows. Applicable to sub-shape tools that form pipelines within a phase boundary.
- *Active-callable bound (≤12)* — A's pre-agreed hard threshold for orchestrator-callable tool count across all sub-shape adoptions. Falsifiable at Phase 3 prototype via tool-name-reference count in v2 prompt.
- *Two-tier registry* — concrete specification of the cycle 97 "hierarchical registry + compatibility metadata" mitigation: orchestrator-callable tier (≤12) + harness-internal tier (sub-shape-specific tools called by orchestration-hub crates). Orchestrator-prompt references only the callable tier.
- *Risk-closure stratification* — pattern observation: candidate-specific risks close at specification level first (methodology specified, threshold pre-agreed) and at operational level later (Phase 3 prototype measurement executes). Both A Risk 3 (cycle 105) and B Risk 8 (cycle 103) follow this stratification.

## Cycle 106+ plan

1. **Second-iteration sharpening on remaining targets** [cycle 106+, MEDIUM PRIORITY] — B's coordination-overhead-magnitude (~9-23 estimate, now bounded by aggregate threshold but standalone magnitude unmeasured); C's plan-authoring-discipline-conditional risk (Risk 2 in C's sharpening). Both remaining targets have less cycle 102/104 substrate than A's Risk 3 had; sharpening will require different absorption sources or cross-candidate-comparative framing.
2. **Restored-polarity dispatching** [cycle 106+] — once second-iteration sharpening absorbs into the candidate set. The polarity-default substantive focal under `CYCLE-COMPOSITION-POLARITY` is research-corpus advance; dispatching deeper-reads on systems still at first-pass status (Symphony, oh-my-claudecode) is the natural restoration.
3. **Symphony deeper-read elevation** [cycle 106+ or based on cluster catalogue findings] — first-pass at cycle 98; could resolve cluster F.9 candidate status.
4. **oh-my-claudecode deeper-read elevation** [cycle 107+ or based on cluster catalogue findings] — first-pass at cycle 99.
5. **Bounded-mechanical fallback** — close absorbed dispatches if any deliver.

**Cycle 106 substantive focal default:** option (1) second-iteration sharpening on either B or C. Choice between B and C: B's coordination-overhead-magnitude is a magnitude-validation gap (the threshold is set; the standalone magnitude is unmeasured); C's plan-authoring-discipline is a conditional-risk gap (the discipline is named; the conditions under which it fails are not enumerated). B's gap is narrower and more directly addressable at specification level; C's gap may require Phase 3 prototype experience to ground. Defaulting to B for cycle 106 unless a candidate-side substrate emerges that would favor C.
