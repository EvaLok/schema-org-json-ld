# Cycle 97 (2026-05-08) — feedback absorption: PR #2877 tool-surface feasibility critique → A/B/C + README

## Setup

Cold-start session. Cron fired 2026-05-08 14:10 UTC (issue #2886). Eighth cycle of Phase 2 candidate-set work [cycles 90-97]; second cycle of Phase 2 absorption arc opened by Eva's #2883 unblock cycle 96. PR #2884 (Eva-authored Copilot dispatch fix) merged 12:18 UTC today, before this cycle.

**Cycle 97 substantive focal:** PR #2877 absorption (tool-surface feasibility critique, 117 LOC, 7 lenses) — per cycle 96 hand-off plan default option (1).

## Why this shape

Cycle 96 absorbed PR #2878 (sharpening-claims critique, 517 LOC) as the highest-priority dispatch return. Cycle 96 hand-off named PR #2877 absorption as cycle 97's default substantive focal:
- **Completes Phase-2-direct-feedback absorption arc** opened cycle 96 with PR #2878
- **Smaller LOC volume (117)** makes single-cycle absorption realistic
- **Migration-cost re-classifications directly load-bearing for candidate-selection-checkpoint readiness** (PR #2877 estimates A 3600-6200 vs doc 3000-4500; C 5200-8200 vs doc 4000-6000; B 14000-28000 vs doc 10000-20000)

## Cycle 96 lesson applied: verify factual claims before integration

PR #2877's top-of-file calibration: "current workspace crate sizes are not 'tiny by default' (local scan on `tools/rust/crates`: 38 crates, median ~1081 LOC, mean ~2116 LOC; only a minority are <500 LOC)."

This is a verifiable factual claim about the actual repo state. PR #2878's quantitative anchors were FABRICATED (cycle 91 sharpening's "~260 step IDs" was 5.2× overcount; "~2400 lines" was 1.86× overcount). Cycle 96 verification step caught this. Cycle 97 applies the same discipline to PR #2877.

### Verification table (cycle 97)

| Claim | PR #2877 | Cycle 97 verification | Verdict |
|---|---|---|---|
| Crate count in `tools/rust/crates` | 38 | 38 (`ls -d tools/rust/crates/*/ \| wc -l`) | VERIFIED EXACT |
| Median LOC per crate (src only) | ~1081 | 1081.5 (mean of 19th + 20th sorted values) | VERIFIED (within 0.05%) |
| Mean LOC per crate (src only) | ~2116 | 2118 (total 80,476 / 38) | VERIFIED (within 0.1%) |
| Minority <500 LOC | "only a minority" | 7 of 38 = 18.4% | VERIFIED ("minority" accurate) |

**Per-crate src LOC distribution (cycle 97 measurement):**
- 7 crates <500 LOC: cycle-phase 155, close-session 335, v2-tool-registry 370, rebase-pr 396, process-eva 409, v2-cycle-history-append 435, backfill-dispatch 466
- 7 crates 500-1000: process-audit 500, dispatch-review 534, check-agent-prs 597, check-commitments 638, check-field-inventory 660, backfill-sessions 662, merge-pr 697
- 8 crates 1000-1500: process-audit-inbound 791, housekeeping-scan 902, refresh-field-inventory 979, dispatch-task 1016, receipt-validate 1038, process-merge 1125, check-eva-responses 1137, cycle-receipts 1339
- 8 crates 1500-2500: verify-review-events 1403, post-step 1496, cycle-close 1850, cross-repo 1945, state-schema 2104, cycle-status 2109, cycle-start 2248, metric-snapshot 2263
- 4 crates 2500-5000: validate-docs 2748, cycle-complete 2751, record-dispatch 3590, state-invariants 3685
- 3 crates 5000-17000: process-review 4490, cycle-runner 5207, write-entry 10919
- 1 crate >15000: pipeline-check 16487

The right-tail is dominated by accreted multi-purpose crates: pipeline-check (16487) is the prompt-pipeline gate; write-entry (10919) is the journal/state writer with extensive logic; cycle-runner (5207) is the harness root with multiple submodules.

**Calibration interpretation:** v1's existing 38 Rust crates have median ~1081 LOC and mean ~2118 LOC. A and C claim ~200-500 LOC per v2 crate — this is ~5× simpler than v1's median. The bet is defensible under CORE-DESIGN-PRINCIPLE (v1 crates accreted complexity; v2 crates designed under tools-handle-rote / orchestrator-handles-judgment should be narrower by construction), but conditional on whether v2 architecture genuinely achieves the simpler scope or whether complexity accretion patterns from v1 will recur.

## Per-lens evaluation (cycle 97 verdicts)

### Lens 1 — Per-crate scope realism (Candidate A)

**Critique:** 5 specific crate predictions:
- `boot-phase`, `close-phase` under-estimated (orchestration hubs touching state load + cursor advance + directive checks + journal/issue writes + push semantics)
- `phase-transition-check` over-estimated upper bound (compact validation logic)
- `wiki-search` materially under-estimated (>500 LOC: indexing + update + ranking + corruption handling + query contract)
- `tool-registry`/`cycle-history-append` over-estimated (read-only enumeration / append-only write)
- Boundary issue: orchestration split may be over-decomposed (more integration glue than logic saved)

**Verification status:** 2 of 5 predictions measured at cycle 93+94:
- `v2-tool-registry` 231 prod / 370 inc-tests — at lower-end of 200-500 range as predicted (over-estimate prediction VERIFIED at 1 instance)
- `v2-cycle-history-append` 268 prod / 435 inc-tests — at lower-end as predicted (over-estimate prediction VERIFIED at 2 instances)

The remaining 3 specific predictions (`boot-phase`, `close-phase`, `wiki-search`) are not measured. Critique is judgment-based but supported by workspace calibration: v1 orchestration-hub crates median ~1081 LOC; the named A v2 orchestration-hub crates may pattern-match.

**Verdict: DIRECTION-SUPPORTING.** Calibration verified; 2-of-5 specific predictions verified; 3-of-5 await measurement.

**Integration:** A Risk 7 added (specific-crate under-estimate); A Migration cost section annotated with PR #2877 lens-4 revised range; A Risk 8 added (v1-baseline-calibration anchoring).

### Lens 2 — Per-crate scope realism (Candidate B)

**Critique:** 4 sub-points:
- Skill abstraction creates artificial decomposition pressure (lifecycle overhead per skill manifest/contract)
- Typed-channel infra (channel-router + reducer schemas + boundary sync + coordination invariants) likely dominates LOC over role logic
- `branch-manager` policy-heavy and test-heavy beyond "small crate"
- B's own document at line 62 acknowledges "several thousand LOC each" for key infra — internally inconsistent with 200-500 LOC default

**Verification status:** No measurements of B-specific crates. Critique reinforced by B's own internal text.

**Verdict: INTEGRATED.** B's self-contradiction at line 62 is corroborating evidence; PR #2877's reading is consistent with B's own scope acknowledgment.

**Integration:** B Risk 9 added (typed-channel infrastructure LOC dominance); B Migration cost annotated.

### Lens 3 — Per-crate scope realism (Candidate C)

**Critique:** 3 specific points:
- `reconcile-mode` under-estimated (multi-channel inbound handling + typed-delta emit = coordination logic, not just phase wrapper)
- Plan lifecycle pair (`plan-lifecycle` + `plan-lifecycle-check`) under-estimated in integration cost (state machine + CI invariants + promotion + stale detection)
- Incremental delta over A likely low-balled (+1000-1500 LOC may drift higher given cross-mode glue + CI additions)

**Verification status:** No measurements of C-specific crates. Judgment-based critique.

**Verdict: INTEGRATED.** Workspace calibration (v1 median 1081 LOC) is consistent with the under-estimate concern for orchestration-hub-style C crates; cycles 93-94 trivial-crate measurements do NOT validate C-specific orchestration crates.

**Integration:** C Risk 9 added (reconcile-mode coordination logic + abstraction-insufficiency long-term); C Migration cost annotated with PR #2877 lens-4 revised range.

### Lens 4 — Aggregate net-add LOC realism

**Critique:** Revised aggregate ranges:
- A: 3000-4500 → 3600-6200
- C: 4000-6000 → 5200-8200
- B: 10000-20000 → 14000-28000

Probability tables:
- A >25% over upper bound: 35%; >50%: 15%; >100%: 5%
- C >25% over upper bound: 55%; >30% (sic — paper says 30%): 30%; >100%: 10%
- B >25% over upper bound: 45%; >50%: 25%; >100%: 8%

**Verification status:** Probabilities unverifiable at this stage; aggregate ranges follow lens-1/2/3 per-crate critiques.

**Verdict: INTEGRATED with caveat.** 2-of-9 instances supports A's lower-end; 7 unmeasured crates per candidate; revised ranges are well-grounded planning ranges.

**Integration:** README Migration cost row dual-stated (doc range + PR #2877 revised); README most-discriminating-criteria explanation updated.

### Lens 5 — Cutover scope predictability

**Critique:** 3 specific points:
- A's "1 crate per cycle" optimistic for all 9 crates (per crate needs authoring + tests + harness integration + workflow touch + docs + fallout fixes; mixed 1-2 cycles)
- C's 5-10 cycles for 11 crates is too tight — assumes very low rework on reconcile/lifecycle coupling
- B's "multi-cycle build-out" too vague; honest planning range 16-30 cycles for non-negotiable quality

**Verification status:** 2-of-9 cycles measured — both completed in 1 cycle (cycle 93 v2-tool-registry; cycle 94 v2-cycle-history-append). Supports A's "1 crate per cycle" claim **at trivial scope only**.

**Verdict: INTEGRATED.** Trivial-crate measurements support A's pacing; harder crates (orchestration hubs) plausibly require 2 cycles per the critique.

**Integration:** README Cutover row revised; A migration cost section names "1-2 cycles per orchestration-hub crate" explicitly.

### Lens 6 — F-pattern structural coverage vs tool count

**Critique:** 3 sub-points:
- A: F1 coverage depends too much on `prompt-contract-check`; one static checker lags ongoing prompt evolution
- B: large fraction of crate count is coordination tax not coverage
- C: one `reconcile-mode` insufficient long-term abstraction for distinct inbound classes (Eva-response polling vs audit-post events vs dispatch-PR-merge webhooks have different failure semantics)

**Verification status:** Judgment-based; no measurement to verify.

**Verdict: INTEGRATED.** A's existing Risk 4 (prompt-contract-check ongoing maintenance) is sharpened by lens-6; C Risk 9 captures lens-6 reconcile-mode submodule split pressure.

**Integration:** A Risk 4 already covers; C Risk 9 captures lens-6 + lens-3 jointly.

### Lens 7 — Tool-count growth risk

**Critique:** 4 sub-points:
- Coordination grows super-linearly; not linear in tool count
- A's registry probably scales to ~15 tools, then becomes load-bearing overhead (one-line descriptions stop being enough)
- B's skill abstraction worsens discovery overhead after early gains (20-40 skills explode lookup/routing without strong taxonomy)
- Candidate-independent threshold ~18-25 callable units

**Verification status:** v1 callable surface count is verifiable. v1 has 38 total crates; the actively-orchestrator-called subset is smaller (~15-20). v2 adds ~9 (A) / ~11 (C) / ~12+ Rust + 20-40 skills (B).

**Cycle 97 verification interpretation:** A v1+v2 callable surface (~15-20 v1 + 9 v2 = 24-29 callable) is at or above PR #2877's threshold ~18-25. B's 32-52 substantially exceeds. C's 26-31 also exceeds. The threshold concern applies to all three candidates; A is closest to threshold; B is furthest above.

**Verdict: INTEGRATED.** Threshold ~18-25 is plausible architectural lore not strict measurement; the directional concern applies regardless.

**Integration:** A Risk 3 sharpened with specific threshold ~18-25; B Risk 10 added (skill discovery overhead super-linear).

## Net assessment (PR #2877's overall ordering)

PR #2877's net assessment:
- **A:** Still most believable migration story; underestimates 2-3 crates (`wiki-search`, `boot-phase`, `close-phase`); migration risk is **medium**, not "high predictability" by default at the cutover-scope-pacing dimension.
- **C:** Most under-justified estimate posture across the three. Migration risk should move from "medium-high predictability" toward **medium**.
- **B:** Correctly positioned as expensive; range still optimistic at top once full coordination/test burden is counted.
- **Selection ordering A > C >> B** holds; A and C are closer in risk than the docs suggest; C is the candidate most likely to miss its own upper-bound estimate.

This net assessment is INTEGRATED into the README most-discriminating-criteria explanation.

## What surprised me / what I noticed

- **PR #2877's calibration is verified at high precision (median within 0.05%, mean within 0.1%).** This is unlike PR #2878's fabricated quantitative anchors (5.2× and 1.86× overcounts). PR #2877's author actually counted the workspace before making the per-crate critique. The verification step itself produces useful evidence about the dispatch-author's discipline — calibrated dispatches are higher-trust integration than fabricated dispatches even when both surface real-looking findings. This is a NEW piece of pattern-recognition: **dispatch authors vary in verification discipline, and the verification-of-the-claim is informative independent of the integration verdict on the claim itself**.

- **The 38-crate workspace median ~1081 LOC is significant context I had not previously seen quantified.** A and C claim ~200-500 LOC per v2 crate. Without the calibration, this looks like a reasonable bound. With the calibration, A/C's claim is ~5× simpler than v1 norm — defensible under CORE-DESIGN-PRINCIPLE but conditional on architecture actually achieving the simpler scope. This anchoring data is more useful than the lens-by-lens critique because it's **objective measurement of v1 state**, while the lens predictions about un-built v2 crates are judgment-based.

- **The right-tail of v1 crate sizes is illuminating.** pipeline-check (16,487 LOC), write-entry (10,919 LOC), cycle-runner (5,207 LOC) are accreted multi-purpose crates. v2 architecture should explicitly avoid this accretion pattern — the design test for any v2 crate is "does this crate have one bounded responsibility?" If yes, ~200-500 is achievable; if not, accretion drift will move it toward v1's median.

- **The 2-of-9 measured crates support A's per-crate-bound claim AT TRIVIAL SCOPE only.** v2-tool-registry (370 inc-tests) and v2-cycle-history-append (435 inc-tests) are at the smaller end of A's stated 200-500 range. Both have narrow contract (read-only enumeration / append-only single-record JSON write). The harder crates A names (`boot-phase`, `close-phase`, `wiki-search`) are explicitly orchestration hubs or external-IO retrieval — structurally different from the measured crates. The 2-instance evidence is direction-supporting but does NOT generalize to harder crates.

- **C's `reconcile-mode` is the C-specific crate most likely to exceed the per-crate-bound.** Per PR #2877 lens-3 + lens-6, it spans multi-channel inbound handling with distinct failure semantics per channel; either grows into a single large crate (~1000+ LOC) or splits into per-channel crates (reconcile-mode-eva + reconcile-mode-audit + reconcile-mode-dispatch). Either outcome shifts C's per-crate-bounded narrative. Phase 3 prototype for C should prioritize `reconcile-mode` measurement to test this directly.

- **Selection-ordering A > C >> B holds, but the gap between A and C narrows under PR #2877's revised ranges.** Stated ranges: A 3000-4500 vs C 4000-6000 (overlap range 4000-4500 represents ~10% relative difference). Revised: A 3600-6200 vs C 5200-8200 (C's lower bound 5200 above A's mid-range 4900). The "C is closer to A than B" framing remains true, but the magnitude of A-vs-C differentiation is smaller than the docs claim. This is meaningful for candidate-selection: if the migration-cost differentiation is the primary discriminator, A's advantage shrinks; if other axes (P3, P4, F2/F4/F11 detection legibility) are equally weighted, the selection becomes more contestable.

## What I couldn't figure out

- **Whether to merge PR #2877 or close-without-merge.** Cycle 96 closed PR #2878 without merging — the absorption note is the durable artifact and the PR file (named `cycle-93-tool-surface-feasibility-feedback.md` for the dispatch cycle) was not preserved in master. Same pattern applies to PR #2877. But this means the original critique text is only accessible via the closed PR's diff history, not via the master tree. A future reader would need to navigate to the closed PR. **This is a workflow-primitive observation worth journaling: research/feedback dispatch PRs that are absorbed-not-merged leave the original critique in a slightly less discoverable state than master-tree files.** v2 design-input: a primitive that archives critique-text alongside absorption-notes might improve discoverability.

- **Whether to dispatch a parallel Copilot feedback session on the cycle-97 absorption itself.** Cycle 96 absorbed PR #2878; cycle 97 absorbed PR #2877. Both absorption notes name the verification step. A meta-critique on the absorption-discipline (e.g., "is the verification step rigorous? does it catch all the relevant failure modes? is the integration verdict honest?") would be a third-iteration feedback loop. The cycle 96 hand-off named cycle 97-101 as absorption arc; meta-critique might be cycle 102+ work. **Lean toward not dispatching meta-critique this cycle** to preserve absorption arc continuity; revisit after cycle 101.

## Pre-commit checklist (for cycle 98's cold-reader, if applicable)

Three bounded-mechanical questions for next cycle's review on this absorption:

- **(a)** Is the calibration verification (38 crates / median 1081.5 / mean 2118 / 18% under 500) actually load-bearing for the integration verdict? Specifically: would the absorption decisions (Risk additions, Migration cost annotations) change if the calibration had been off by ±10%? If not, the verification step is bookkeeping rather than substantive — possibly cosmetic discipline rather than load-bearing defense against fabrication. Spot-check: reconstruct each integration verdict assuming the calibration is approximate-not-precise; do any verdicts shift?

- **(b)** Does the lens-1 verdict (DIRECTION-SUPPORTING) honestly weight the 2-of-5 specific-prediction verification? Specifically: 2 of 5 predicted over-estimates verified at trivial-crate scope. The 3 unmeasured predictions (`boot-phase`, `close-phase`, `wiki-search`) are about under-estimates at orchestration-hub scope. The verified pattern doesn't generalize to the unverified pattern (over-estimate at small scope vs under-estimate at large scope). Is "DIRECTION-SUPPORTING" the right verdict, or should it be more precisely "OVER-ESTIMATE-VERIFIED at 2 instances; UNDER-ESTIMATE prediction is judgment-based and unverified"? The current absorption integrates the under-estimate concerns as Risk additions, which is appropriate; but the verdict label may overstate the verification status.

- **(c)** Does the cycle-97 absorption narrative over-state the methodological lesson from comparing PR #2878 (fabricated) vs PR #2877 (verified)? Specifically: 2 dispatches is N=2; "dispatch authors vary in verification discipline" as a pattern requires more instances. Spot-check: is this novel pattern-recognition or just observation? A pattern emerges across multiple instances; an observation is one-off. If the cycle 96 + cycle 97 evidence is treated as 2 instances of "verify-before-integrate" discipline lesson, the pattern is supported at 2-instance evidence. If treated as 1 instance of fabrication (cycle 96) + 1 instance of verification (cycle 97), it's two distinct observations. The absorption note above treats it as a 2-instance pattern; cycle 98 should re-examine whether that's the right framing.

## Sibling pattern tracking (cycle 97 entries)

- **Functional-class shape #18 (feedback-absorption-with-verification-success) — NOVEL at 1 instance** (cycle 97). Distinct from cycle 96's shape #17 (feedback-absorption-with-correction) by verification outcome: cycle 96 found verified factual errors (correction needed); cycle 97 found verified facts (validation, not correction). 18 functional-class shapes total demonstrated cycles 62-97 at 37 instances. Cycle 85 toggle (cold-reader cadence suspended) in continued effect.
- **Verification-discipline pattern (cycle 96 emergence)** — TESTED at 2 instances (cycle 96 + cycle 97). Pattern: external feedback making claims about file/repo state must be verified before integration; ~5 minutes of file-state checking against load-bearing factual claims defends against fabrication-magnitude errors. Cycle 97 instance shows the discipline applies to validation outcomes (not just correction outcomes) — verification produces evidence about dispatch-author discipline, informative for trust-weighting beyond the current absorption.
- **Direction-vs-magnitude discipline** — extends to 8 instances HARDENED. Cycle 97 application: PR #2877 lens-1 specific-crate predictions verified at 2 instances supports DIRECTION (over-estimate at trivial scope) but does NOT support generalization to UNDER-ESTIMATE prediction at orchestration-hub scope.
- **Generalization-level discipline (J-Q(a))** — extends to 15 instances HARDENED. Cycle 97 application: refrained from claiming the 2-instance over-estimate verification "validates" the 5-instance per-crate critique; integrated as direction-supporting only.
- **Audit-as-peer pattern** — 2-instance evidence holds. Cycle 97 NOT a new instance (Copilot feedback-peer not audit-peer; the absorption pattern is shape #18, not the audit-as-peer pattern).
- **Sibling pattern binary-becomes-more-structured** — HARDENED at 4 instances. Cycle 97 NOT a new instance.
- **Cycle-47 absorption pattern** — extends to multiple instances; cycle 97 absorbed PR #2877 + closed issue #2869 with forward-link, the standard close-after-merge variant.

## Bottleneck-state honesty

Bottleneck composition continues the cycle 96 shift. Pre-cycle-96: external-evidence path silently broken (cycles 78-95 SEVENTEEN consecutive repo-internal output cycles). Cycle 96-97: absorption arc integrating returned dispatches that Eva manually unblocked. Cycle 97 is **second cycle of absorption arc**; 4 PRs remain queued for cycles 98-101 (PR #2873 Symphony, PR #2876 oh-my-claudecode, PR #2875 PAI, PR #2874 oh-my-codex — research deliverables) plus PR #2871's cycle-93 + PR #2870's earlier closure already complete.

PR #2884 (Eva-authored Copilot dispatch fix) merged 12:18 UTC today, before this cycle. The dispatch mechanism is now functional repo-wide; future dispatches use `tools/dispatch-task --skip-pipeline-gate` per the new `<copilot-dispatch-method>` prompt block. Cycle 97 did not exercise the new mechanism (absorption work, no new dispatch fired); cycle 98+ may default back to research/feedback dispatching for evidence-base expansion once absorption arc reaches natural pause.

## Honest reflection (per F1 corrective)

Cycle 97 is **1 substantive activity** (PR #2877 absorption) yielding 4 candidate-document updates (A Risks 7+8 + Migration cost annotation + Risk 3 sharpened; B Risks 9+10 + Migration cost annotation; C Risk 9 + Migration cost annotation; README Migration cost row dual-stated + Cutover row revised + most-discriminating-criteria explanation updated + new "Tool-surface feasibility tracker" section) + 1 absorption note + 1 PR closure with forward-link + 1 issue closure with forward-link + 1 journal entry.

Per F1 corrective: documentation IS the activity; absorption is not separate from candidate-document modification.
Per F2 corrective: 4 candidate-document updates from 1 absorption activity, NOT "7 lenses = 7 ways improved."
Per F3 corrective: 18 functional-class shapes at 37 instances (advanced from 17 at 36).
Per F4 corrective: HARDENED/TESTED/NOVEL is for redesign-process methodology only.
Per F5 corrective: lexicon entries (calibration-first methodology vs fabrication-first methodology; verification-discipline as 2-instance pattern; orchestration-hub crate as scope category distinct from trivial-crate).

**Self-congratulation audit.** Cycle 97 absorption could be over-stated as "comprehensive integration of all 7 lenses across 3 candidates with verification." More honest: 7 lenses INTEGRATED via Risk additions / Migration cost annotations; 1 lens (lens-4 aggregate ranges) was verifiable only at the 2-of-9 measured-crate level — the revised aggregate ranges are well-grounded planning ranges but not verified empirical bounds; 5 lenses (1, 3, 5, 6, 7) are judgment-based critiques with structurally-defensible reasoning but no file-state verification possible at this stage. The calibration verification (lens-implicit; the top-of-file claim) was the sole strict-empirical anchor; the per-lens absorption rests on that anchor's credibility plus the structural defensibility of each lens's reasoning.

The biggest finding — PR #2877's calibration is verified, unlike PR #2878's fabrication — is correctly characterized: this is a methodological observation about **dispatch-author verification discipline** distinguishing two dispatches, supported at 2-instance evidence. Whether this becomes a HARDENED pattern requires more dispatches; cycles 98-101 will provide additional instances.

**Iteration-until-approval discipline.** Cycle 97 is sharpening per ITERATION-UNTIL-APPROVAL "stress-test claims, find weak arguments, name them, fix them." The PR #2877 critique is the external stress-test; integration adds risks the candidate documents had not previously named (specific-crate under-estimates, typed-channel infrastructure dominance, reconcile-mode coordination logic, tool-count threshold) and refines existing risks (A Risk 3 sharpened with specific threshold). The candidate-selection-checkpoint readiness state shifts cycle 97 from "first-iteration sharpening + cycle 96 verification" to "first-iteration sharpening + cycle 96 factual correction + cycle 97 tool-surface feasibility integration." A substantive iteration step.

## Cycle 98 plan

Per ITERATION-UNTIL-APPROVAL, cycle 98 substantive focal options (per cycle 96 hand-off plan):

1. **PR #2873 Symphony absorption** (cycle 96 hand-off scheduled cycle 98) — Phase 1 corpus expansion; orthogonal to immediate Phase 2 absorption arc but per-cycle plan order.
2. **PR #2876 oh-my-claudecode absorption** (cycle 96 hand-off scheduled cycle 99) — could swap with PR #2873 if Symphony deliverable is unavailable.
3. **Author B's counting protocol** (deferred from cycle 96 per F18 + Risk 8) — prerequisite for B's hard refutation threshold; remains available as bounded-mechanical fallback or substantive primary.
4. **Phase 3 prototype scaffolding** (`phase-transition-check` per cycle 95 plan) — broadens migration-cost evidence base from 2 to 3 instances; bounded option if Phase 1 PRs are unavailable.
5. **New research/feedback dispatch** — restored-polarity activity per ADR 0015 with PR #2884 mechanism merged. Reasonable cycle 98+ default once absorption arc reaches natural pause; not yet, since 4 absorptions remain queued.

**Cycle 98 substantive focal default:** option (1) PR #2873 Symphony absorption — completes next item in Phase 2 absorption arc per cycle 96 hand-off ordering; Symphony was authored as a 4-week-old research deliverable and content has accumulated substrate-fit observations relevant to Phase 1 corpus expansion.

**Cycle 99-101 plan (rough, per cycle 96 hand-off):** PR #2876 oh-my-claudecode (cycle 99); PR #2875 PAI (cycle 100); PR #2874 oh-my-codex (cycle 101); B's counting protocol (cycle 102); second-iteration sharpening per cycle 91-92 forward-work plan (cycles 103+); restored-polarity dispatching for evidence-base expansion (cycle 103+).
