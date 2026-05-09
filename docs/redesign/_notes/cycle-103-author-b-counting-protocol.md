# Cycle 103 — Author B's counting protocol specification (closing Risk 8 at specification level)

**Cycle:** redesign cycle 103 (2026-05-09)
**Source:** Risk 8 in [`docs/redesign/2-candidates/B-decomposed-multi-role.md`](../2-candidates/B-decomposed-multi-role.md) (added cycle 96 per PR #2878 absorption); deferred 7 cycles (96-102); HIGH PRIORITY in cycle 103+ plan (cycle 102 journal hand-off).
**Purpose:** Specify B's counting protocol so its hard refutation threshold can be specified before Phase 3 measurement, matching the cycle 96 absorption discipline (verified denominator + chosen unit + hard threshold pre-agreed).

## Setup

Risk 8 is the longest-standing structural-asymmetry in the candidate set:

- **Candidate A** has a hard threshold (cycle 96): if extraction < 30%, P3 is PARTIAL-FLAG; (30%, 50%) is PASS-WITH-NOTE; ≥ 50% is full PASS.
- **Candidate C** has a hard threshold (cycle 96): central-bet validated if F4 stale-detection-lag improvement ≥ 1 cycle AND F11 detection-rate improvement ≥ 3pp.
- **Candidate B** does NOT have a hard threshold — the cycle 96 absorption named the missing counting protocol as a prerequisite, and B has carried "soft validation language" (promotes / weakens) since cycle 92 sharpening landed.

This is a structural asymmetry in falsifiability, not in design quality. B *cannot be falsified the way A and C can* until the counting protocol is specified. The cycle 103 work closes that gap.

The deferral arc:

- Cycle 96 PR #2878 absorption identifies Risk 8.
- Cycle 97 _note explicitly defers counting protocol to "cycle 99-101 plan, item 6: B's counting protocol (cycle 102)".
- Cycles 98-101 prioritize PR-absorption arc (Symphony / oh-my-claudecode / PAI / oh-my-codex deep-reads) over deferred counting protocol — research-corpus advance is the polarity default per `CYCLE-COMPOSITION-POLARITY`.
- Cycle 102 prioritizes cluster catalogue rebuild (catalogue staleness ELEVATED) over deferred counting protocol — staleness elevation forces catalogue rebuild as the substantive focal.
- Cycle 103: PR-absorption arc reached natural pause cycle 101; catalogue rebuild completed cycle 102; counting protocol no longer behind any HIGH-priority deferral. Cycle 103 substantive focal under `CYCLE-COMPOSITION-POLARITY` is "v1-system retrospective extraction of lessons not yet absorbed into the redesign" or equivalent — cycle 103's choice is to clear the longest-standing deferred sharpening item (B's counting protocol), which is itself a v1-vs-redesign lessons-extraction primitive (the cycle 96 lesson about discipline-bar-too-low applied to a specific candidate's measurement plan).

## Design choices made

The protocol design responds to Risk 8's three named concerns (self-report circularity; LLM-tool-invocation conflation; specification-before-measurement) with these design choices:

### Choice 1: Unit of analysis — *structural decision-points in artifacts*, not *operational decisions in cycles*

The Risk 8 framing ("self-report by role-session AI is circular") suggested operational/per-cycle counting (UNIT-B style). But operational counting is exactly what produces the circularity. The protocol re-frames: count *structural* decision-points in the artifacts (per-role prompts + per-role checklists + cross-role coordination contracts), not operational decisions in a runtime cycle.

This is the cleanest unit because:
- Aligns with v1 baseline (50 step+substep IDs verified cycle 96 are structural counts of the prompt+checklists)
- Aligns with A's chosen-unit framework (cycle 91 _notes list step IDs as one option for A's denominator)
- Is per-cycle-invariant (the structural artifacts don't change per cycle; only firing varies)
- Is verifiable by reading the artifacts, not by observing AI behavior

The design alternative considered: operational tool-invocation counting (UNIT-C). Rejected as primary unit because Risk 8 explicitly names "counting LLM tool invocations conflates mechanical with cognitive." But operational counting is preserved as a *secondary sanity-check unit* with category-classification (mechanical-write / mechanical-read / structural-decision-prompt / cross-role-coordination), which doesn't conflate mechanical with cognitive — it explicitly distinguishes them.

### Choice 2: Method — *external enumeration with reproducibility check*, not self-report

Risk 8's self-report concern is resolved by requiring an external observer (a non-author Claude session, Eva, audit-side reviewer, or Copilot feedback-only dispatch). The protocol does not require any specific external observer; any of the four options suffices.

Reproducibility check (±10% across two independent observers) ensures the rubric is operationally well-defined. If two observers diverge by more than 10%, the rubric needs sharpening before measurement. This is the inter-rater reliability discipline — cheap (≤30 min per observer per artifact set) and load-bearing for falsification credibility.

The design alternative considered: single-external-observer with no reproducibility check. Rejected because subjective rubrics produce variable counts; without a reproducibility check, the measurement is dependent on which observer is selected.

### Choice 3: Aggregate-based threshold, not per-role threshold

The threshold is on aggregate (per-role + coordination), with per-role observed as a secondary direction-check metric. This is because:
- B's central bet is "comparable aggregate to v1, with per-role mitigation" — falsifying the central bet requires falsifying aggregate.
- Per-role thresholds (e.g., "executor must be ≤ X") would require defining v1's per-role-equivalent, which doesn't cleanly exist (v1 is single-orchestrator).
- The aggregate threshold + per-role direction-check together cover both the magnitude claim and the direction claim.

The design alternative considered: per-role hard thresholds on every role. Rejected because v1 has no per-role-equivalent baseline; per-role comparisons would be unanchored.

### Choice 4: Ratio-based threshold, not absolute count

The threshold is expressed as ratio to v1 baseline (≤ 1.0×, (1.0×, 1.5×], (1.5×, 2×], > 2×), not as absolute count (≤ 50, etc.). This is robust to unit-choice — if the rubric produces counts different from cycle 96's 50 step+substep IDs, the ratio still works as long as both candidates are measured under the same rubric.

The design alternative considered: absolute thresholds anchored at cycle 96's 50. Rejected because the rubric may produce a different v1 count (estimated 35-40 under structural-decision-point rubric per the spot-check in the protocol section). Ratio-based is more robust.

### Choice 5: Default unit-inclusion choice — *minimally-inclusive (named steps + named patterns)*

The protocol could be applied at varying inclusion levels — minimally-inclusive (named steps + named patterns only) vs maximally-inclusive (also prose-procedural decisions). The default is minimally-inclusive on the grounds that:
- B's role artifacts will also have prose-procedural decisions; rubric should treat artifacts symmetrically.
- Named steps + named patterns is closest to v1's verified baseline (cycle 96 counted step+substep IDs).
- Symmetry across A and B (A's chosen-unit framework also includes "named patterns" as one option).

The design alternative considered: maximally-inclusive default. Rejected because it would inflate v1's baseline more than B's (v1's prompt is more prose-procedural than B's role-specific prompts will be), giving B an unfair direction-validation tilt.

## Edits made

1. **`docs/redesign/2-candidates/B-decomposed-multi-role.md`** — inserted new section `### Counting protocol (cycle 103 specification, addressing Risk 8)` between `### Coordination overhead estimate` and `### Validation plan`. The section is ~150 lines covering: background, unit of analysis (with inclusion/exclusion criteria), counting method (external enumeration + reproducibility), per-role + coordination decomposition, sanity-check secondary unit (operational tool-invocation log with category classification), hard refutation thresholds (ratio-based), observational secondary metric (per-role mitigation direction-check), baseline re-derivation under the rubric (with rubric spot-check estimate of 35-40 v1 baseline under-the-rubric), re-counting cadence, and a Risk-8-specific concern-vs-answer table.

2. **`docs/redesign/2-candidates/B-decomposed-multi-role.md`** — replaced validation plan step 5's soft "promotes / weakens" language with explicit hard refutation thresholds (PASS / PASS-WITH-NOTE / PARTIAL-FLAG / REFUTED based on ratio to v1 baseline), referencing the Counting protocol section above for the methodology.

3. **`docs/redesign/2-candidates/B-decomposed-multi-role.md`** — updated Risk 8 with `CLOSED at specification level cycle 103` annotation pointing to the Counting protocol section. Notes that Risk 8 remains *open at the operational level* until Phase 3 prototype measurement actually executes the protocol.

4. **`docs/redesign/2-candidates/README.md`** — updated B's row in Load-bearing claims sharpening tracker. Sharpening cycle progression updated to "Cycle 92 → Cycle 96 → Cycle 103 (counting protocol specified, closing Risk 8 at specification level)". Direction column re-labeled `enumeration-rubric-now-specified` (previously `decision-class naming, not enumeration`). Magnitude column re-labeled `ambiguous-without-prototype-but-falsifiable (hard thresholds pre-agreed: ...)` with explicit threshold values.

5. **`docs/redesign/2-candidates/README.md`** — updated Forward-work section to flag cycle 103 closure and remove counting-protocol from second-iteration sharpening targets.

## Explicit not-modified items (with reasoning)

- **Cluster catalogue ([`docs/redesign/1-research/clusters.md`](../1-research/clusters.md))** — counting protocol is a candidate-evaluation primitive, not a research-cluster primitive. No catalogue updates owed.
- **Per-system files ([`docs/redesign/1-research/systems/*.md`](../1-research/systems/))** — counting protocol is candidate-evaluation, not per-system pattern observation. No per-system updates.
- **`docs/redesign/2-design-framework.md`** — the framework defines axes, evaluation criteria, and convergent constraints, not measurement protocols. The protocol is a candidate-specific measurement methodology and lives in the candidate file.
- **Candidate A's [validation plan](../2-candidates/A-evolved-single-orchestrator.md#validation-plan)** — A already has a hard threshold (cycle 96, ≥ 50% / 30%-50% / < 30% extraction). No update owed; B's protocol does not change A's.
- **Candidate C's [validation plan](../2-candidates/C-hybrid.md#validation-plan)** — C already has hard thresholds (cycle 96, F4 ≥ 1 cycle AND F11 ≥ 3pp). No update owed; B's protocol does not change C's.
- **Cross-cluster intersection patterns ([`clusters.md` 7 intersections](../1-research/clusters.md))** — re-evaluation of cross-cluster patterns is a separate cycle 103+ plan item; cycle 103 substantive focal is B's counting protocol, not intersections. Defer to cycle 104+.
- **`MEMORY.md` initialization (cycle 100)** — observed during cycle 103 setup that the runner-local memory directory at `/home/runner/.claude/projects/.../memory/` does NOT exist in this container; cycle 100's "MEMORY.md initialization" was ephemeral per-runner state and did not persist across cron-triggered runs. The actual cross-cycle persistence mechanism remains `docs/redesign/` + `docs/journal/` + `_notes/`. Document this observation in the cycle 103 journal (not modified in this _note, but flagged for journal entry).

## Verification (post-edit)

- Counting protocol section: ~150 lines added between `### Coordination overhead estimate` and `### Validation plan`. Single contiguous section under one `### Counting protocol` heading.
- Validation plan step 5: replaced (~3 lines deleted, ~8 lines added). Soft language gone; hard thresholds in.
- Risk 8: closure annotation added (Risk 8 entry remains visible — its existence is the audit trail). Closure language explicit about specification-level vs operational-level distinction.
- README sharpening tracker B row: 2 columns updated (sharpening cycle progression; direction; magnitude). Footnote cross-cutting per-candidate analysis preserved.
- README forward-work section: B counting protocol removed from second-iteration sharpening targets; cycle 103 closure noted with link to Counting protocol section.
- Heading anchor verification: `(#counting-protocol-cycle-103-specification-addressing-risk-8)` is the GitHub-style auto-generated anchor for the section heading "Counting protocol (cycle 103 specification, addressing Risk 8)" (lowercase, dashes for spaces, parens stripped).

## What surprised me / what I noticed

1. **The counting protocol is a methodology-discipline primitive, not a candidate-design primitive.** The protocol's structure (external observer, reproducibility check, structural unit, ratio threshold) could equally apply to A or C if their thresholds were under-specified. It's a cycle-96-discipline-bar-applied-to-B instance — and the same discipline bar applies to any candidate's measurement plan that depends on observer-counts.

2. **The 7-cycle deferral was load-bearing protection, not procrastination.** Cycles 96-102 prioritized PR absorption and catalogue rebuild over Risk 8 closure. The deferral was justified by `CYCLE-COMPOSITION-POLARITY` — research-corpus advance was the substantive focal default. Risk 8 closure was BLOCKED by absorption-arc completion, not just deferred. Cycle 103 is the first cycle where absorption arc has reached natural pause AND catalogue rebuild has completed — this is the first cycle where Risk 8 closure is both unblocked and clearly the highest-priority deferred item.

3. **The protocol resolves Risk 8 at the *specification* level, not the *operational* level.** Phase 3 prototype measurement is what actually executes the protocol; cycle 103 only specifies the methodology. Risk 8's annotation reflects this — `CLOSED at specification level cycle 103; remains open at operational level`. This dual-level closure framing is a NEW pattern worth tracking — many of B's risks are similar (named at specification level, remain open at operational level until Phase 3 prototype). The pattern is *specification-vs-operational risk-closure stratification*.

4. **The hard threshold uses 4 bands (PASS / PASS-WITH-NOTE / PARTIAL-FLAG / REFUTED), not 2 (PASS / FAIL).** This matches A's 3-band threshold (full PASS / PASS-WITH-NOTE / PARTIAL-FLAG) and slightly extends it with a REFUTED band at > 2× v1 baseline. The REFUTED band is meaningful for B because B's central decomposition bet has independent value (cluster G mitigation) even if aggregate-cost is unfavorable; REFUTED downgrades the cost claim without rejecting the design. This is a more nuanced threshold structure than C's binary (validated / not-validated).

5. **The estimate v1-baseline-under-rubric of 35-40 is significantly smaller than the cycle 96 step-ID count of 50.** Spot-check found ~70-80% of step+substep IDs are structural under the rubric; the remainder are mechanical (post-comment, commit, file-write). This means the rubric produces a more semantically meaningful baseline than the raw step-ID count. **Implication:** if Phase 3 prototype measurement reveals B aggregate ≈ 35-40 (the v1-under-rubric range), B passes the threshold; if measurement reveals B aggregate ≈ 50 (the raw v1 step-ID count), B is at PASS-WITH-NOTE. The rubric *increases B's likelihood of crossing the PASS threshold* by tightening v1's baseline — this is a *non-neutral* property of the rubric choice that should be acknowledged. Mitigation: the rubric is applied symmetrically to v1 and B, and the verified-baseline-pre-Phase-3 step in the protocol is the explicit verification.

## Sibling pattern tracking

- **Functional-class shape #21 — `risk-closure-at-specification-level`** — NOVEL at 1 instance. Pattern: a candidate-specific risk named at cycle N is closed at specification level at cycle M (N < M, deferral arc 7+ cycles); operational closure deferred to Phase 3 prototype. Distinguished from `risk-closure-at-operational-level` (where the closure depends on prototype measurement). Both halves of the closure are tracked. **NOVEL at 1 instance.**

- **Verification-discipline pattern (cycle 96 emergence)** → **6 instances HARDENED**. Cycle 103 NOT a new instance — the protocol is a *measurement-discipline specification*, not a verification of someone else's claim.

- **Generalization-level discipline (J-Q(a))** → **19 instances HARDENED**. Cycle 103 application: the protocol's choice between minimally-inclusive and maximally-inclusive rubric is a generalization-level decision (which classes of decision-points count); the choice is named explicitly with reasoning, not silently committed. Generalization-level discipline applied; not a new instance count.

- **Direction-vs-magnitude discipline** → **11 instances HARDENED**. Cycle 103 application: the protocol explicitly separates direction (per-role mitigation direction-check) from magnitude (aggregate-vs-baseline ratio); the two are observed independently and combined for verdict. The discipline is applied as protocol structure; not a new instance count.

- **Cycle-composition-polarity discipline (cycle 62 emergence)** → 41 instances HARDENED at cycle 102. Cycle 103 substantive focal *is* "v1-system retrospective extraction of lessons not yet absorbed into the redesign" applied to B's specific risk-deferral arc; the cycle 96 discipline-bar-too-low lesson is now applied to the longest-standing under-specified candidate measurement. Cycle 103 application of polarity discipline.

- **Audit-as-peer pattern** — 2-instance evidence holds. Audit cycle 213 still silent-failed (no audit-repo commit since cycle 212 at 2026-05-07T04:35Z, ~50+ hours since cycle-101 measurement; ~2 hours added since cycle 102). Cycle 103 NOT a new instance.

- **Pre-agreed-falsification-threshold discipline** (cycle 96 emergence per PR #2878 Finding 18) → cycle 103 application is the THIRD instance (A cycle 96 / C cycle 96 / B cycle 103). All three candidates now have pre-agreed thresholds; the candidate-set is *fully falsifiable for the first time*. **3-candidate completeness HARDENED at 1 instance.** This is itself a structural milestone for the candidate-selection checkpoint — Eva can evaluate all three candidates against equally falsifiable threshold structures.

## Lexicon entries (cycle 103)

- *Counting protocol* — measurement-methodology specification for B's per-role decision-count metric: structural-decision-points in role artifacts (primary unit), external-observer enumeration with reproducibility check, ratio-based threshold against v1 baseline measured under same rubric. Closes Risk 8 at the specification level.
- *Structural decision-point* — labeled prompt instruction that requires the role's session to choose between alternatives. Distinguished from mechanical instructions (post-comment, commit, file-write) which are excluded from the count. The unit of analysis for B's counting protocol.
- *Risk-closure-at-specification-level vs at-operational-level* — NOVEL functional-class shape #21 distinction. Specification-level closure means the methodology is specified; operational-level closure means the measurement has actually executed and produced a verdict. Cycle 103's B counting-protocol closure is specification-level; Phase 3 prototype is operational.
- *3-candidate falsification completeness* — state where all three candidates have pre-agreed falsification thresholds. Reached at cycle 103 (B's protocol joins A's cycle 96 and C's cycle 96 hard thresholds). Structural milestone for candidate-selection checkpoint readiness.
- *Rubric inclusion non-neutrality* — observation that the choice between minimally-inclusive and maximally-inclusive rubric is non-neutral with respect to which candidate is favored. Mitigation: apply rubric symmetrically + name the choice explicitly.

## Cycle 104+ plan

1. **Cross-cluster intersection updates** [cycle 104+, MEDIUM PRIORITY] — re-evaluate the 7 cross-cluster intersection patterns from cycles 72/74 with the new cluster A + cluster H sub-shapes from cycle 102. Specific candidate intersections: A↔B (classifier-dispatch storage), F↔H (Sonnet substrate dual-cast), A↔C (task-ingestion-routing as session-start lifecycle).
2. **Second-iteration sharpening on candidates A/B/C** [cycle 104+] — with full 4-system absorption + cluster catalogue update (cycle 102) + B's counting protocol (cycle 103) feeding into candidate risk refinement. Remaining sharpening targets: A's tool-registry-growth risk; B's coordination-overhead-magnitude (now bounded by aggregate threshold); C's plan-authoring-discipline risk.
3. **Restored-polarity dispatching** [cycle 105+] — once second-iteration sharpening absorbs into the candidate set, restore polarity dispatching for evidence-base expansion.
4. **Symphony deeper-read elevation** [cycle 105+ or based on cluster catalogue findings] — first-pass at cycle 98; could resolve cluster F.9 candidate status if Symphony's spec-first BEAM substrate has equivalent deterministic-post-processing primitive.
5. **oh-my-claudecode deeper-read elevation** [cycle 106+ or based on cluster catalogue findings] — first-pass at cycle 99; deeper read would confirm/refute the 3-system substrate-edge convergence observation.

## Pre-commit checklist

- [x] B-decomposed-multi-role.md: counting protocol section inserted (~150 lines)
- [x] B-decomposed-multi-role.md: validation plan step 5 hard-threshold replaced
- [x] B-decomposed-multi-role.md: Risk 8 closure annotation added
- [x] README.md: B sharpening tracker row updated (sharpening cycle, direction, magnitude)
- [x] README.md: forward-work section updated (cycle 103 closure noted)
- [x] _notes/cycle-103-author-b-counting-protocol.md: this file authored
- [x] Heading-anchor verified for cross-references to Counting protocol section
- [x] Sibling pattern tracking includes NOVEL functional-class shape #21
- [x] 3-candidate falsification completeness milestone noted
- [x] Cycle 104+ plan updated with cross-cluster intersection updates as next item
