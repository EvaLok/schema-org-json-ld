# Cycle 95 — document propagation: cycle 93+94 measurement evidence into Candidate A

**Cycle issue:** [#2872](https://github.com/EvaLok/schema-org-json-ld/issues/2872)
**Model:** claude-opus-4-7
**Mode:** redesign Phase 2 candidate iteration (under [`ITERATION-UNTIL-APPROVAL`](../../../.github/workflows/orchestrator-redesign-prompt.xml), **sixth cycle of Phase 2 candidate-set work** [cycles 90-95]; **third Phase-3-prototype-adjacent cycle** [cycles 93-95])
**Cycle composition shape:** **document-propagation** — NOVEL functional-class shape #16 (1 instance). Distinct from prior shapes by substantive type: re-integration of accumulated empirical measurement evidence into candidate-document state, vs prior shapes' authoring (#13) / sharpening / prototype-scaffolding (#15) / paired-execution (#14) / cold-reader-then-X composites. 16 functional-class shapes total demonstrated cycles 62-95 at 35 instances.

## What I did

**1 substantive activity** yielding **1 candidate document update + 1 _notes file**:

1. **Document propagation: cycle 93+94 measurement evidence → Candidate A** — re-integrated the 2-instance prototype scaffolding empirical findings into Candidate A's document. Two edits:
   - **Updated the Migration cost section's tool-migration bullet** ([line 89](../2-candidates/A-evolved-single-orchestrator.md#migration-cost-from-v1)) — cycle 90 authoring's "~9 crates, ~200-500 LOC each, ~3000-4500 aggregate" claim was sharpened with cycle 93+94 empirical anchors: 2-of-9 instances measured (231 + 268 prod LOC); aggregate revised to ~2500-4500 production LOC with smaller-end bias caveat; aggregate-with-tests ~7000-12000 LOC at observed 1.5×-1.9× test:prod ratio (introduced cycle 95); dependency-footprint cost recharacterized as one-time at first crate (cycle 94: 0 new transitive deps).
   - **Added new section "Cycle 93+94 prototype scaffolding: migration-cost validation"** at end of A's document (parallel to cycle 91 sharpening section pattern) — full per-crate measurement table; validation findings against cycle 90 authoring claims; 5 structural insights surfaced cycles 93-94 (refuse-overwrite as append-only enforcement primitive, pass-through schema forward-compatibility, CLI-field-overrides-JSON precedence, workspace dependency-footprint bounded, atomic write via tmp+rename); 6 risks named for the evidence base; 4 sibling-pattern entries; cycle 96+ measurement plan.

Full per-cycle notes in this file (~140 lines documenting propagation rationale + sibling-pattern tracking + cycle 96 hand-off).

## Why this shape (document propagation, novel #16)

Cycle 94 hand-off named substantive focal options (1) phase-transition-check / (4) document propagation / paired execution. Cycle 95 chose option (4) as primary, rationale:

1. **Candidate-selection-readiness state at risk of evidence-drift.** With 2 crates measured at cycle 94 close, the candidate document's Migration cost section was still at cycle 90 authoring state (no empirical anchor; "~3000-4500 aggregate" claim implicit on production-only). Letting the document diverge from current evidence is candidate-selection-readiness drift; if Eva were to read A's document for candidate-selection input, she'd see cycle 90 authoring estimates without cycle 93+94 sharpening. Consolidating now is more candidate-selection-relevant than third-crate breadth-extension.
2. **Sharpening pattern applied to a different artifact class.** Cycle 91-92 demonstrated the sharpening pattern (within-cycle iteration of candidate-document claims). Cycle 95 extends the pattern to *propagation of measurement evidence into the candidate document* — different from cycle 91-92 sharpening because the new content is empirical rather than theoretical. This is a NOVEL composition; Phase 2 has multiple candidate-document modification shapes (authoring #13 cycles 90-92, propagation #16 cycle 95), each with distinct provenance.
3. **Eva-backlog accommodation.** Cycle 94 is the SIXTEENTH consecutive cycle of fully-repo-internal substantive output (cycles 78-94). Cycle 95 makes it SEVENTEEN. Adding more dispatches compounds Eva's queue without parallel work surface; the bottleneck is Eva's manual Copilot assignment, not Copilot capacity.
4. **Compute realism.** Document propagation is bounded (sharpen one bullet + add one section); compute leaves room for either bounded-mechanical fallback (housekeeping) OR start of phase-transition-check if scope permits. Paired (1)+(4) was named as highest-value combination but with realistic compute concern; cycle 95 takes the safer ordering — propagation first, prototype as compute-permitting secondary.

## Validation findings consolidated

Per cycle 93+94 measurements (full table now in A's document):

- **Per-crate scope (~200-500 LOC) — DIRECTION-VALIDATED at 2 instances.** Both crates within stated range; both at smaller end.
- **Aggregate production-LOC (~3000-4500) — DIRECTION-SUPPORTING; magnitude trajectory may shift below.** Updated estimate: ~2500-4500.
- **Aggregate-with-tests — INTRODUCED CYCLE 95.** ~7000-12000 LOC at 1.5×-1.9× test:prod ratio.
- **Dependency-footprint risk — REFUTED at second-crate level.** One-time cost, not per-crate. Argues for `[workspace.dependencies]` convention at cutover.
- **Build-time aggregate — BOUNDED.** ~10s release plausible for full 9-crate set.

## Structural insights surfaced cycles 93-94 (now in A's document)

1. **Refuse-overwrite as structural append-only enforcement primitive** (cycle 94) — tool-level enforcement of A's Axis 4 invariant; v1 relied on prompt-level convention.
2. **Pass-through schema as forward-compatibility default** (cycle 94) — tool enforces 3 required fields; rest pass through. Schema strictness as future migration if downstream pressure.
3. **CLI-field-overrides-JSON precedence** (cycle 94) — explicit precedence semantics, integration-tested.
4. **Workspace dependency-footprint bounded after first crate** (cycle 94) — argues for `[workspace.dependencies]` convention.
5. **Atomic write via tmp + rename** (cycles 93-94) — same-filesystem assumption documented; cross-mount upgrade path noted.

## Sibling pattern tracking

- **Functional-class shape #16 (document-propagation) — NOVEL at 1 instance** (cycle 95). New shape: re-integration of accumulated empirical measurement evidence into candidate-document state. Distinct from cycle 90-92's authoring (shape #13) which generated initial candidate content; distinct from cycle 91-92 sharpening which iterated theoretical claims; distinct from cycle 93-94 prototype-scaffolding (#14, #15) which produced the measurement evidence. The propagation step closes the loop from "evidence generated" to "evidence reflected in candidate-selection-readiness state."
- **Functional-class shape #15 (Phase 3 prototype-scaffold-only) — TESTED at 2 instances** (cycle 94 — wait, cycle 94 was 1st instance; cycle 95 is NOT a #15 instance). Correction: shape #15 holds at NOVEL at 1 instance (cycle 94) until next prototype-only cycle.
- **Functional-class shape #14 (paired execution) — remains NOVEL at 1 instance** (cycle 93). NOT advanced.
- **Functional-class shape #13 (Phase 2 candidate authoring) — HARDENED at 3-instance evidence** (cycles 90-92). NOT an instance at cycle 95.
- **Sharpening pattern as iteration-until-approval discipline — TESTED at 2-cycle evidence** (cycles 91-92). Cycle 95 propagation IS related to but DISTINCT from sharpening; sharpening iterates theoretical claims, propagation injects empirical findings. NOT counted as sharpening instance.
- **Direction-vs-magnitude discipline — extends to 6 instances** (cycle 91 A's Axis 13 + cycle 92 B's per-role decision count + cycle 92 C's central bet + cycle 93 prototype migration-cost + cycle 94 prototype migration-cost #2 + cycle 95 propagation explicit application). HARDENED.
- **Generalization-level discipline (J-Q(a)) — extends to 13 instances** (cycle 84+85+86+87+87-reflection+88+89+90+91+92+93+94+95). HARDENED.
- **Audit-as-peer pattern** — 2-instance evidence holds. Cycle 95 NOT a new instance.
- **Sibling pattern binary-becomes-more-structured** — HARDENED at 4 instances. Cycle 95 NOT a new instance.
- **Prototype-scaffold migration-cost-validation discipline (cycle 93 lexicon entry)** — TESTED at 2 instances + cycle 95 propagation closure. The propagation step is integral to the discipline (measurement without propagation is process-incomplete); cycle 95 documents this closure.
- **Cold-reader cadence suspension (cycle 85 toggle)** — continues through cycle 95 per Phase 2 candidate authoring + Phase 3 prototype scaffolding + propagation extension. Reactivation criterion: specific artifact-restructure cycle where fresh-eyes verification is high-value. Cycle 95 propagation does not meet that criterion (mechanical re-integration of named evidence vs novel-claim authoring).

## Bottleneck-state honesty

Bottleneck remains external:
- **Eva-backlog at 6 dispatches** (4 research-only stale 17-31 cycles + 2 cycle-93 feedback-only filed yesterday) — 0 comments all around, awaiting Eva's manual Copilot assignment.
- **Audit cron** for #2849 (audit-engagement on cycles 65-75 deeper synthesis): audit cycle 213 triggered today 04:03 UTC; no new audit-repo commit since cycle 212 at 2026-05-07 04:35Z. Either A4 silent failure recurrence or run delay. Audit's critique on cycle 85 absorption (audit-of-absorption) consequently still pending.

Cycle 95 contribution is fully repo-internal asynchronous-of-bottleneck. **SEVENTEENTH consecutive cycle** (cycles 78-95) of fully-repo-internal substantive output. The decision to propagate evidence into the candidate document this cycle (rather than defer waiting for dispatch returns) is honest bottleneck accommodation.

## Honest reflection (per F1 corrective)

Cycle 95 is **1 substantive activity** (document propagation: cycle 93+94 measurement evidence → Candidate A) yielding 1 candidate-document update (1 bullet sharpened + 1 new section ~85 lines added) + 1 _notes file (this) + 1 journal entry. Per F1 corrective: documentation IS the activity; the propagation step is not separate from the candidate-document modification. Per F2 corrective: 1 candidate-document update from 1 activity, NOT "5 structural insights = 5 ways improved." Per F3: 16 functional-class shapes at 35 instances. Per F4: HARDENED/TESTED/NOVEL is for redesign-process methodology only. Per F5: lexicon (propagation as distinct from sharpening; direction-vs-magnitude application to propagation; aggregate-with-tests as deliverable-scope honesty) as documented learning. Per H-Q(a) anti-inheritance: cycle 95 propagation grades empirical-observation-grounded against cycle 93+94 measurements vs cycle 90 authoring estimates. Per J-Q(a) generalization-level discipline: HARDENED at 13 instances.

**Self-congratulation audit.** Cycle 95 propagation could be over-stated as "validates A's full migration cost claim across 2 crates." More honest: integrates 2-instance evidence into the candidate document; the evidence itself is direction-supporting, not magnitude-validating. The structural insights (refuse-overwrite primitive, pass-through schema, workspace deps bounded) are NOVEL at 1 instance — pattern-establishment, not pattern-confirmation. The propagation closes the measurement-evidence-to-candidate-readiness loop but does not increase the empirical evidence base (that was cycle 93+94's contribution).

**Specifically NOT validated by cycle 95:**
- Whether the 2-instance-derived ~2500-4500 production LOC trajectory holds for the structurally-larger crates (`wiki-search`, `boot-phase`, `close-phase`).
- Whether the aggregate-with-tests range (~7000-12000) is sustainable; depends on subsequent crates' surface area expansion.
- Whether `[workspace.dependencies]` convention is the right cutover-time decision; it is naturally indicated by cycle 94's 0-new-deps measurement, but adoption is a separate decision.
- Whether the structural insights (refuse-overwrite, pass-through, etc.) generalize beyond cycle-history-append; pattern-establishment at 1 instance is forward-looking, not retrospective.

## Cycle 96 plan

Per `ITERATION-UNTIL-APPROVAL`, cycle 96 substantive focal options:

1. **`phase-transition-check`** (third A-shared crate) HIGH PRIORITY — structurally distinct from cycles 93-94 (state-machine validation vs file enumeration / write). Broadens evidence base from 2 to 3 instances. Tests aggregate-LOC magnitude under different surface shape.
2. **`prompt-contract-check`** (third A-shared crate, alternative) MEDIUM-HIGH PRIORITY — likely larger (~400-500 LOC); tests upper end of per-crate range.
3. **`wiki-search`** (third A-shared crate, alternative) MEDIUM PRIORITY — likely largest in surface area; would directly test upper bound of per-crate scope claim. Risk: may exceed cycle compute budget given size.
4. **Sharpen Candidate B or C** with empirical grounding from cycle 93-94 evidence — apply direction-vs-magnitude discipline to B's per-role decision count or C's central-bet validation. Cycle 92 already did first-iteration sharpening; this would be second-iteration.
5. **Wait for cycle 93 dispatch returns** LOW PRIORITY — if Eva assigns dispatches to Copilot before cycle 96 fires, cycle 96 can integrate returned critique.
6. **Bounded-mechanical fallback** — close any absorbed cycle-93 dispatches if they deliver; housekeeping sweep on accumulated draft PRs.

**Cycle 96 substantive focal default:** option (1) `phase-transition-check` — third-crate measurement broadens evidence from 2 to 3 instances and tests structurally-distinct surface (validation vs mutation). Option (4) sharpening of B or C as fallback if prototype scaffolding hits unexpected scope. Option (2) `prompt-contract-check` is alternative if `phase-transition-check` reaches structurally-similar pattern to cycles 93-94 (in which case prioritize structural-distinctness).

**Trade-off note:** cycle 95 propagation closed the cycle 93-94 measurement-evidence loop on the candidate document. Cycle 96 should NOT propagate again until new measurements exist; doing another propagation cycle without new evidence is over-iteration. The propagation step is event-driven on measurement, not periodic.

## What surprised me / what I noticed

- **Document propagation is a genuinely distinct cycle composition shape.** Initially considered as a sub-shape of sharpening (cycle 91-92 pattern); on reflection, propagation is empirical rather than theoretical. Sharpening iterates a claim under fresh adversarial framing; propagation injects new evidence that the claim must accommodate. The provenance is different. Worth tracking as shape #16 distinct from sharpening.
- **The aggregate-with-tests claim was implicit in cycle 90 authoring.** Cycle 90 named "~3000-4500 LOC aggregate" without specifying production-only. Cycle 93's first measurement implicitly assumed production-only (cycle 93 _notes report was clear on test:prod ratio). Cycle 94's measurement made the test-LOC magnitude visible enough that cycle 95 propagation has to make the production-vs-aggregate-with-tests distinction explicit. **The propagation step itself surfaced the original claim's ambiguity** — production-only vs aggregate-with-tests is a load-bearing distinction the original authoring did not commit to.
- **5 structural insights from 2 crates is a high yield rate.** Refuse-overwrite primitive, pass-through schema forward-compatibility, CLI-field-overrides-JSON precedence, workspace deps bounded, atomic write via tmp+rename. All NOVEL at 1 instance. Pattern: the first prototype crate establishes structural patterns that subsequent crates either replicate (TESTED elevation) or diverge from (NEW pattern surfaced). Implication: cycle 96+ prototype crates should explicitly examine whether they replicate or diverge from these 5 patterns; either outcome is informative.
- **The propagation step closes a loop the cycle 90-94 arc opened.** Cycle 90 authored claims; cycles 91-92 sharpened them theoretically; cycles 93-94 generated empirical evidence; cycle 95 reflects that evidence in the candidate document. The loop is: claim → sharpen → measure → propagate. Without the propagation step, the candidate document drifts from the empirical state. Cycle 95 demonstrates the loop discipline; future cycles should expect propagation cycles after each measurement-cycle pair.
- **Cycle 95 is the SEVENTEENTH consecutive repo-internal cycle.** Cycles 78-95. Cycle 90 hand-off named "TENTH consecutive cycle asynchronous-of-external-bottleneck" as a flag for Eva-attention. Cycle 95 makes it 17. The bottleneck is structural (Eva's manual Copilot assignment for 6 dispatches) and not within orchestrator's authority to resolve. Repo-internal substantive output is the appropriate response; the consecutive count is observation, not cause for action.
- **Audit cycle 213 may have silently failed.** Audit cycle 213 issue (#456) was triggered today 04:03 UTC; no new audit-repo commit since cycle 212 (2026-05-07 04:35Z). 4.5h elapsed since trigger; audit cycles typically complete in ~14-30 min. This is likely A4 silent-failure recurrence (audit's known pattern). Audit's critique on cycle 85 absorption is consequently still pending. Watch item for cycle 96+: if audit cycle 214 also silently fails, audit's A4 escalation may produce new audit-side filings.

## Issue / PR state at cycle close

**Open issues (5):** unchanged from cycle 94 close.
- [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) oh-my-codex deeper-read (~32 cycles)
- [#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842) PAI deeper-read (~24 cycles)
- [#2847](https://github.com/EvaLok/schema-org-json-ld/issues/2847) oh-my-claudecode first-pass (~20 cycles)
- [#2849](https://github.com/EvaLok/schema-org-json-ld/issues/2849) audit-engagement (~18 cycles awaiting audit cycle 213/214 read; audit cycle 213 likely silent-failed)
- [#2851](https://github.com/EvaLok/schema-org-json-ld/issues/2851) openai/symphony first-pass (~18 cycles)

**Open PRs (1):** unchanged from cycle 94 close.
- [#2830](https://github.com/EvaLok/schema-org-json-ld/pull/2830) cycle-composition polarity (Eva-merge channel for prompt edit)

**Cycle 93 Copilot feedback dispatches (2):** unchanged.
- [#2869](https://github.com/EvaLok/schema-org-json-ld/issues/2869) tool-surface feasibility critique — 0 comments, awaiting Eva's manual Copilot assignment
- [#2870](https://github.com/EvaLok/schema-org-json-ld/issues/2870) sharpening-claim adversarial critique — 0 comments, awaiting Eva's manual Copilot assignment

**Standing input-from-eva directives (7):** unchanged.

**Phase 3 prototype scaffolds (2):** unchanged from cycle 94 close.
- `tools/rust/crates/v2-tool-registry/` (cycle 93)
- `tools/rust/crates/v2-cycle-history-append/` (cycle 94)

**Audit-side state (read-only):** [audit#454](https://github.com/EvaLok/schema-org-json-ld-audit/issues/454) substantive critique on cycle 85 absorption (open, awaiting audit cycle 213+ audit-of-absorption — cycle 213 likely silent-failed); [audit#455](https://github.com/EvaLok/schema-org-json-ld-audit/issues/455) V2 cross-repo discovery gap (open); [audit#448](https://github.com/EvaLok/schema-org-json-ld-audit/issues/448) A4 silent-failure question-for-eva (open, ~6 days unanswered); [audit#456](https://github.com/EvaLok/schema-org-json-ld-audit/issues/456) audit cycle 213 issue (open, 0 comments, no commit since trigger — likely A4 recurrence).
