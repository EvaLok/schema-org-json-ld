# Cycle 113 — Second central-bet stress-test (equivalent-outcomes detection-vs-processing) + runtime allowlist friction reconfirmed

**Cycle issue:** [#2902](https://github.com/EvaLok/schema-org-json-ld/issues/2902)
**Date:** 2026-05-10
**Mode:** redesign Phase 2 candidate iteration, **twenty-fifth cycle of Phase 2 candidate-set work** (cycles 90-113), under [`ITERATION-UNTIL-APPROVAL`](../../../.github/workflows/orchestrator-redesign-prompt.xml).

## Setup and the cycle 112 → 113 forward-going-commitment trajectory

Cycle 112 (2026-05-10) closed with two cycle-113+ priorities: (1) retry the BLOCKED Copilot adversarial feedback dispatch on the post-cycle-112 selection draft, and (2) continue central-bet stress-testing on remaining load-bearing claims. Cycle 110 had named cycle 111+ rotation commitment to A-side or B-side risk closure; cycles 111 and 112 both departed from that. **Cycle 113 also departed (third consecutive cycle of departure).** The forward-going-commitment-honored discipline (cycle 110 NOVEL@1 across 4-cycle chain 107→108, 108→109, 109→110, 110→111-forward-named) is now broken at three consecutive forward-going-commitment links (cycle 111, cycle 112, cycle 113). **The discipline is now CONCLUSIVELY REFUTED at strict-discipline level**; it survives only as soft "honor when no higher-priority work surfaces" — which is closer to no-discipline than to a discipline.

The cycle 113 departure was reasoned: continuing the cycle 112 stress-test pattern (which surfaced a real motivated-reasoning finding) AND retrying the cycle 112 dispatch were higher-priority than per-candidate risk closure. Rotation-commitment is now substantively dead; per-candidate sharpening is no longer the substantive default for Phase 2 cycles since cycle 110.

## Cycle 113 contributions, in order

### 1. Cycle 112 BLOCKED dispatch retry — runtime allowlist gap reconfirmed

The cycle 112 adversarial feedback dispatch body file at [`cycle-112-dispatch-1-body.md`](./cycle-112-dispatch-1-body.md) was authored, the action was authorized by the redesign prompt's `COPILOT-DISPATCHES`, and the syntax is documented in `COPILOT-DISPATCH-METHOD`. Cycle 113 retried the dispatch using `tools/dispatch-task --skip-pipeline-gate ...` per the documented method.

**Result:** runtime returned "This command requires approval" — same block as cycle 112. The block is structural, not transient. The runtime permission allowlist in [`.github/workflows/orchestrator.yml`](../../../.github/workflows/orchestrator.yml) lines 66-87 includes:

```
"Bash(gh *)", "Bash(git *)", "Bash(jq *)", "Bash(composer *)", "Bash(bash *)",
"Bash(php *)", "Bash(cargo *)", "Bash(mkdir *)", "Bash(ls *)", "Bash(date *)",
"Bash(wc *)", "Bash(sort *)", "Bash(cat *)", "Bash(head *)", "Bash(tail *)",
```

`tools/dispatch-task` and `tools/dispatch-review` are not in the list. The two-line fix is unambiguous; cycle 113 attempted the standard forbidden-zone fix path (open PR you merge):

1. Created branch `cycle-113-allow-dispatch-task-runtime` with the diff.
2. Committed locally.
3. Attempted `git push -u origin cycle-113-allow-dispatch-task-runtime` — **rejected** by GitHub:

   > ! [remote rejected]   cycle-113-allow-dispatch-task-runtime -> cycle-113-allow-dispatch-task-runtime (refusing to allow a Personal Access Token to create or update workflow `.github/workflows/orchestrator.yml` without `workflow` scope)

`ORCHESTRATOR_PAT` does not have `workflow` scope. **The standard forbidden-zone PR path is unavailable for this fix** because the orchestrator cannot push the PR that would resolve the allowlist gap.

**Double structural friction observed:**
- Level 1 (runtime allowlist gap) — blocks `tools/dispatch-task` execution.
- Level 2 (PAT `workflow` scope gap) — blocks the orchestrator-side PR fix for level 1.

**Resolution:** filed [question-for-eva #2903](https://github.com/EvaLok/schema-org-json-ld/issues/2903) with three options Eva can choose (Option A: manual commit, recommended; Option B: add `workflow` scope to PAT; Option C: decline both). The 5-cycle autonomy default is named: if no Eva response within 5 cycles, proceed under Option C (continue without dispatch substrate; rely on audit-as-peer cross-repo critique). Local branch was cleaned up; the proposed diff is preserved in [`cycle-113-q4eva-runtime-allowlist.md`](./cycle-113-q4eva-runtime-allowlist.md).

This is a textbook `EVA-DEFAULT-AUTONOMY` category 1 (infrastructure access / provisioning — PAT scope IS infrastructure) AND category 3 (CI configuration — workflow allowlist) overlapping question. The escalation to question-for-eva is appropriate per the redesign prompt's filter-before-filing language.

### 2. Second central-bet stress-test — equivalent-reconciliation-outcomes claim (substantive cycle 113 contribution)

Cycle 112 framed the A-vs-C reconciliation difference as "legibility only," anchoring on the claim "A and C have equivalent reconciliation OUTCOMES." Cycle 113 stress-tested this claim by re-reading [`A-evolved-single-orchestrator.md`](../2-candidates/A-evolved-single-orchestrator.md) lines 36-37+45 and [`C-hybrid.md`](../2-candidates/C-hybrid.md) lines 20+22+44-46.

**Finding:** the "equivalent outcomes" claim is correct on DETECTION but glosses over PROCESSING. Three dimensions cycle 112 named as "legibility only" are actually structurally distinct:

1. **Per-mode runtime budget allocation.** C explicitly declares per-mode runtime budgets at Axis 9 (boot ≤5min / reconcile ≤10min / work ≤50min / close ≤10min). Reconcile-mode has dedicated runtime independent of other boot work. A has no reconciliation-specific budget — reconciliation work shares boot-phase budget with state-load + cursor-advance + standing-directive check + gardening-sweep-pre-cycle. Under burst load (multiple Eva responses + audit posts queued same cycle), A's reconciliation may be truncated or work spills into work-phase; C's reconcile-mode is bounded but dedicated. **This is not legibility — it is runtime allocation.**

2. **Per-channel structured handlers.** C's Axis 12 explicitly names *"Per-event handlers per channel; reconciler-mode emits typed-deltas to per-component state files"*. A's Axis 12 describes *"input-from-eva pull at session-start with cursor advance; question-for-eva-response check via existing `check-eva-responses` (or replacement)"* — procedurally less structured. Adding a new inbound channel (e.g., a third-party orchestrator with a new label, a webhook-driven event source) to A's boot-phase polling requires a code change that interleaves with existing boot work; C's reconcile-mode adds a per-channel handler with a uniform shape. **This affects extensibility AND consistency-of-handling across channels.**

3. **Typed-delta output discipline.** C's reconcile-mode emits typed-deltas to per-component state files — a single auditable output stream per cycle. A's reconciliation outputs scatter across boot-phase activities (cursor advances, in-place state updates, journal appends). For audit-as-peer's ability to read reconciliation state across cycles, C's typed-deltas are more readable than A's scattered boot-phase outputs. **This is partly legibility but also partly auditability — they are not the same.**

**Honest qualification:** detection latency genuinely is equivalent (1 cycle, set by cron cadence). Processing characteristics differ. The cycle 112 reframing of the A-vs-C trade-off as "named-phase legibility worth +~1000-2600 LOC" should now read "named-phase legibility + runtime budget allocation + per-channel extensibility + audit-readability worth +~1000-2600 LOC."

**Does this flip the recommendation?** No, not on this finding alone. Criteria 1+3 still favor A. The processing-structure differences are real but bounded. But the case for A continues to weaken across cycles 111→112→113 — and the cycle 113 work explicitly names the threshold for an honest flip:

> **If a fourth round of stress-testing (cycle 114+) surfaces a fourth load-bearing under-weighted dimension, the recommendation should be honestly flipped to C.** The pattern of progressive sharpening identifying A-favoring claims as motivated reasoning suggests the cycle 111-derived recommendation may not survive deeper iteration.

This is a substantive commitment. Cycle 114+ stress-tests will either find no further under-weighted dimensions (recommendation holds at A; the case is "weakened but not flipped") or surface a fourth (recommendation flips to C with the cumulative four-finding rationale).

### 3. Edits made cycle 113

1. **[`docs/redesign/2-selection.md`](../2-selection.md)** — substantial update:
   - Status header: cycle 113 attribution added; "central-bet stress-test on equivalent-outcomes claim absorbed."
   - Authoring cycle line: cycle 113 stress-test acknowledged.
   - Section "Why this matters less than it first appears": rewritten as "(with cycle 113 qualification)"; added ~30 lines naming the three under-weighted dimensions; reframed the equivalence claim as "DETECTION outcomes equivalent / PROCESSING characteristics differ."
   - Section "Reframed selection question": added cycle-113 framing alongside cycle-112 framing; preserved both for traceability.
   - Section "Why the recommendation stays as A": renamed to "(post-cycle-112, post-cycle-113)"; revised all 4 points; added new "Honest cumulative observation" paragraph naming the four-finding flip threshold.
   - Outstanding work item #3: updated to reflect cycle 113 absorption; added cycle 114+ remaining stress-test targets.
   - Open question #6 for Eva: rewritten with cycle 113 sharpening (three-dimension trade-off vs cycle 112's two-dimension framing).
   - "What A gives up" item #1: added "Post-cycle-113 sharpening" paragraph naming the boot-phase-bottleneck risk on three dimensions.
   - Side-by-side comparison table: added new row "A-vs-C reconciliation distinction (cycle 112 → cycle 113 evolution)" with per-candidate framing of the cycle-112-then-cycle-113 trajectory.
   - Iteration log: added cycle 113 row covering the dispatch retry + the substantive stress-test finding + the workflow-change PR friction + the question-for-eva filing.

2. **[`docs/redesign/2-candidates/README.md`](../2-candidates/README.md)** — status paragraph extended with cycle 113 stress-test summary, the runtime-allowlist friction observation, and the question-for-eva #2903 reference.

3. **[`docs/redesign/_notes/cycle-113-q4eva-runtime-allowlist.md`](./cycle-113-q4eva-runtime-allowlist.md)** — question-for-eva body, including the 2-line proposed diff, the three resolution options, and the 5-cycle autonomy default.

4. **[question-for-eva #2903](https://github.com/EvaLok/schema-org-json-ld/issues/2903)** — filed with the body above.

5. **[`docs/redesign/_notes/cycle-113-second-stress-test-and-runtime-friction.md`](./cycle-113-second-stress-test-and-runtime-friction.md)** — this file.

## Verification (compressed)

| Item | Pre-cycle-113 | Post-cycle-113 |
|---|---|---|
| `docs/redesign/2-selection.md` line count | ~340 | ~360 (+20 lines net of edits + new "Honest cumulative observation" paragraph + cycle 113 iteration log row + side-by-side table row) |
| Sections in 2-selection.md | 13 | 13 (no new sections; existing sections sharpened) |
| Open questions for Eva | 6 | 6 (Q6 sharpened with cycle 113 three-dimension framing) |
| Side-by-side comparison table rows | 12 | 13 (new "A-vs-C reconciliation distinction" row) |
| Outstanding work items | 6 | 6 (item #3 sharpened with cycle 113 absorption + cycle 114+ remaining targets) |
| Iteration log rows | 2 (cycles 111, 112) | 3 (cycles 111, 112, 113) |
| Recommendation | A (tentative; central-bet rationale contested rather than confidently defended) | A (tentative; central-bet rationale weakening across 3 cycles; explicit four-finding flip threshold named) |
| Adversarial Copilot dispatch on selection draft | attempted cycle 112; BLOCKED by runtime; deferred to cycle 113+ | retried cycle 113; BLOCKED again; runtime allowlist gap confirmed structural; question-for-eva #2903 filed; deferred to cycle 114+ post-Eva-resolution |
| Forward-going-commitment-honored discipline | REFUTED-at-strict-discipline-level (cycles 111+112 both departed) | **CONCLUSIVELY REFUTED** (cycles 111+112+113 all departed; three consecutive forward-going-commitment links broken) |
| `discretionary-departure-from-forward-going-commitment` candidate-pattern | TESTED@2 (cycle 111 + cycle 112) | **HARDENED-at-3** (cycle 111 selection-draft authoring + cycle 112 central-bet stress-test + cycle 113 second stress-test — three substrate-content-distinct departures) |
| `central-bet-stress-test-finds-mis-grounding` candidate-pattern | NOVEL@1 (cycle 112 first instance) | **TESTED@2** (cycle 112 mis-grounded against retrospective text + cycle 113 under-weighted dimensions in cycle 112 reframing — substrate-content-distinct: cycle 112 found mis-grounding against retrospective; cycle 113 found mis-grounding within cycle 112's own reframing) |
| Functional-class shape framework | 24 shapes / 54 instances (paused per cycle 111) | 24 shapes / 54 instances (paused; cycle 113 produced no shape-tracking promotions per the cycle 111 framework-paused observation) |
| `runtime-permission-delta-vs-prompt-authorization` lexicon entry | named cycle 112 (one observation) | **structurally confirmed cycle 113** (second observation; PR-fix path also blocked by PAT scope; double-friction) |

## What surprised me / what I noticed

1. **The cycle 112 reframing was itself motivated reasoning, just at a smaller scale than cycle 111's.** Cycle 111 selected pattern-level dominance (which favored A) without acknowledging family-level dominance (which favors C). Cycle 112 corrected this BUT then framed the A-vs-C difference as "legibility only" — a softer characterization that preserved A's recommendation despite the family-level finding favoring C. Cycle 113 surfaces that "legibility only" was under-weighting on three dimensions (runtime budget, per-channel extensibility, audit-readability). **The pattern is: each round of stress-testing finds the prior round's reframing was itself softening A-against-C tension to keep the recommendation at A.** This is an instance of what `ITERATION-UNTIL-APPROVAL` calls "examine for self-congratulation" — and it is recurring across cycles, not bounded to a single instance.

2. **The "legibility only" framing was a defensive softening of a finding that should have been characterized as more structurally distinct from the start.** Reading C's Axis 9 line 20 ("per-mode runtime budget (boot-mode ≤ 5min; reconcile-mode ≤ 10min; work-mode ≤ 50min; close-mode ≤ 10min)"), it is structurally distinct from A on more than legibility — and this was visible in C's document text cycle 112 already cited. The cycle 112 reframing did not engage with the per-mode runtime budget claim; it reduced the difference to "named harness phase vs interleaved with boot." **This is the kind of detail that is easy to miss when the reframing-author has a recommendation already in mind.**

3. **The runtime-allowlist friction is doubled, not single.** Cycle 112 observed the runtime allowlist gap; cycle 113 confirmed it AND surfaced the PAT-scope gap that prevents the orchestrator-side PR fix. **The forbidden-zone fix path (open PR Eva merges) does not work for this category of fix.** The fix path that works is "Eva commits the change directly with her account scope." This is operationally important for v2 design: the redesign prompt's `direct-push-zones` and `SELF-MODIFICATION-GATES` envision the orchestrator opening PRs to forbidden zones; the actual mechanism requires PAT scope the orchestrator does not have. v2 either needs (a) `workflow` scope for the orchestrator (broader privilege), (b) a non-PR escalation pattern Eva accepts (which this question-for-eva instantiates), or (c) Eva-only commits to those zones (which is what option A in #2903 implicitly proposes).

4. **The forward-going-commitment-honored discipline is now CONCLUSIVELY REFUTED across three consecutive cycles.** Cycle 110 named cycle 111 rotation; cycle 111 departed (selection-draft authoring); cycle 112 departed (central-bet stress-test); cycle 113 departed (second central-bet stress-test). **The discipline survives only as a soft "honor when no higher-priority work surfaces"** — but cycles 111-113 demonstrate that higher-priority work surfaces consistently when the cycle starts. The discipline is therefore not a binding commitment but rather an aspiration. **Implication for v2 design:** the redesign prompt's `BETWEEN-CHECKPOINTS` 5-cycle autonomy default IS a discipline that worked (Eva has not had to override the orchestrator's defaults during the gap on #2903 yet, but the default is named ahead of time so Eva CAN). v2 should preserve named-thresholds-with-named-responses-decided-in-advance (audit's "commitment-thread discipline" per audit cycle 215) and demote internal "rotation commitments" to soft heuristics.

5. **The cycle 113 dispatch retry surfaced double-structural friction that cycle 112 did not name.** Cycle 112 _notes #4 named the runtime-allowlist gap. Cycle 113's substantive contribution to the same friction observation was: the standard forbidden-zone fix path (PR) is also blocked. **This means there is no orchestrator-resolvable path for this category of fix.** It is a structural escalation point. Naming it as a structural escalation point — rather than retrying the same dispatch path indefinitely — was the cycle 113 contribution to the friction question.

6. **The four-finding flip threshold is a structurally important commitment.** The cycle 113 selection draft now names: "if a fourth round of stress-testing (cycle 114+) surfaces a fourth load-bearing under-weighted dimension, the recommendation should be honestly flipped to C." This converts the progressive-weakening pattern from a vague concern into a falsifiable commitment. Cycle 114+ stress-tests now have a defined success/failure criterion: if no new under-weighted dimension surfaces across N cycles, the case for A is "stable but weakened" and the recommendation holds; if a fourth dimension surfaces, the recommendation flips. This is the kind of pre-committed threshold the cycles 103-110 risk-closure work exemplified at per-candidate level; cycle 113 brings the same discipline to the cross-candidate selection.

7. **The bottleneck-asynchronous posture continues uninterrupted (cycle 113 = twenty-fifth consecutive cycle).** The dispatch friction does not change this — cycle 113 produced substantive forward-progress repo-internally (selection draft sharpening + question-for-eva filing) even with the dispatch blocked. **The interpretation:** orchestrator-internal stress-testing is producing real findings cycle-over-cycle (cycle 112 mis-grounding finding; cycle 113 under-weighted-dimensions finding); external feedback (audit cycle 216, dispatch return) would accelerate but is not strictly required for further progressive sharpening. Cycle 114+ can continue stress-testing remaining load-bearing claims even if #2903 remains unresolved.

## Sibling pattern tracking (informal — formal tracking remains paused per cycle 111)

| Pattern | Pre-cycle-113 status | Post-cycle-113 observation |
|---|---|---|
| `discretionary-departure-from-forward-going-commitment` | TESTED@2 (cycle 111 + cycle 112) | **HARDENED-at-3** (cycle 113 third departure with substrate-content-distinct: second central-bet stress-test) |
| `forward-going-commitment-honored` | REFUTED-at-strict-discipline-level (cycles 111+112) | **CONCLUSIVELY REFUTED** (cycle 113 third consecutive departure; discipline is dead at strict level; survives only as soft heuristic) |
| `central-bet-stress-test-finds-mis-grounding` (cycle 112 NEW candidate-pattern) | NOVEL@1 (cycle 112 first instance) | **TESTED@2** (cycle 113 second instance: under-weighted dimensions in cycle 112's reframing — substrate-content-distinct) |
| `runtime-permission-delta-vs-prompt-authorization` (cycle 112 lexicon entry) | observed once (cycle 112) | **structurally confirmed twice with PR-fix-path-blocked observation** (cycle 113) |
| `progressive-rationale-weakening-without-flip` (cycle 113 NEW candidate-pattern) | not named | **NOVEL@1** (cycle 113 first instance: cycle 111→112→113 trajectory of progressive A-rationale weakening with recommendation holding; applicability test is whether cycle 114+ surfaces a fourth under-weighted dimension that triggers the named flip threshold) |
| `pre-committed-flip-threshold` (cycle 113 NEW candidate-pattern) | not named | **NOVEL@1** (cycle 113 first instance: the four-finding-triggers-flip commitment in 2-selection.md "Why the recommendation stays as A"; applicability test is whether cycle 114+ honors the threshold or reasons-around it; if reasoned-around, the threshold reveals as protective fiction; if honored, it is a load-bearing discipline) |
| Risk-closure-at-specification-level (shape #21) | HARDENED-at-7 (cycle 110) | unchanged (cycle 113 not a per-candidate risk closure) |
| `central-bet-stress-test` activity (lexicon entry from cycle 112) | TESTED@2 (cycle 112 + initial cycle 113) | **HARDENED@3** (cycle 113 second cycle's substantive output is a stress-test finding) |

## Lexicon entries

- **detection-vs-processing distinction:** the cycle 113 framing of the A-vs-C reconciliation difference. Detection (latency, cursor-polling mechanism) is genuinely equivalent between A and C. Processing (runtime budget allocation, per-channel structured handlers, typed-delta output discipline) differs structurally. Cycle 112's "equivalent outcomes" claim collapses both layers; cycle 113 separates them.
- **per-mode runtime budget (C):** C's Axis 9 explicitly declares boot ≤5min / reconcile ≤10min / work ≤50min / close ≤10min — dedicated per-phase runtime independent of other phase work. A has no equivalent.
- **per-channel structured handlers (C):** C's Axis 12 names "Per-event handlers per channel; reconciler-mode emits typed-deltas to per-component state files." A's Axis 12 describes a procedurally-loose pull pattern.
- **typed-delta output discipline (C):** C's reconcile-mode emits typed-deltas to per-component state files — single auditable output stream. A's reconciliation outputs scatter across boot-phase activities.
- **four-finding flip threshold:** the cycle 113 commitment that if a fourth round of stress-testing surfaces a fourth load-bearing under-weighted dimension favoring C over A, the recommendation should honestly flip to C. Converts progressive-weakening from concern into falsifiable commitment.
- **double-structural friction:** when both a primary fix and the meta-fix-path for the primary fix are structurally unavailable. Cycle 113 instance: runtime allowlist gap blocks dispatch (level 1); PAT-scope gap blocks the PR fix for level 1 (level 2). Implication: orchestrator-resolvable path does not exist; escalation to Eva is the only resolution.
- **progressive-rationale-weakening-without-flip:** the pattern across cycles 111→112→113 where each round of stress-testing weakens the recommendation's rationale but does not flip the recommendation. Each cycle preserves the recommendation by reframing the prior cycle's finding as "less load-bearing than first appeared." Cycle 113 names this pattern explicitly and commits to a flip threshold.
- **pre-committed flip threshold:** a pre-stated falsifiable criterion for when a recommendation should be honestly reversed. Distinct from "I will revisit the recommendation if X happens" (vague) by naming a measurable trigger ("a fourth under-weighted dimension surfaces"). Borrowed from cycles 103-110 risk-closure-at-specification-level discipline at the per-candidate level; cycle 113 applies it at the cross-candidate selection level.
- **the discipline is dead, but the pattern is hardened:** cycle 113's three-cycle confirmation of `discretionary-departure-from-forward-going-commitment` (HARDENED-at-3) alongside the conclusive refutation of `forward-going-commitment-honored`. Same dynamic as cycle 112's "the discipline is breaking, but the pattern is forming" but at HARDENED scale. The discipline did not survive contact with priority-shifting work substrate; the departure-pattern is now the normal rhythm.

## Bottleneck-state honesty

Bottleneck remains external: no audit critique landings since #454; no Copilot dispatch returns (and now the path to landing one is structurally unavailable until #2903 resolves); no Eva input arrivals since standing directives. **Cycle 113 produced two parallel forward-progress streams from a single cycle:** (1) substantive selection-draft sharpening (the equivalent-outcomes detection-vs-processing finding); (2) operational-friction escalation (the question-for-eva for runtime allowlist + PAT scope).

Both streams are repo-internal in their immediate output but produce external dependencies for cycle 114+: stream (1) produces a body-of-work that Eva and audit can read for the candidate-selection checkpoint; stream (2) produces a question-for-eva whose resolution determines whether dispatch substrate becomes available for cycle 114+. **Cycle 113 is the twenty-fifth consecutive cycle (cycles 78-113) whose output is fully repo-internal; it is the third cycle since cycle 102 not focused on per-candidate sharpening (cycles 111, 112, 113).**

Audit cycle 215 (2026-05-10 04:21-04:41 UTC) closed without substantive Phase 2 critique landing. Audit cycle 216 expected at next audit cron (~04:00 UTC 2026-05-11, ~17.5h from cycle 113's start at ~10:24 UTC). Cycle 113 cannot wait for it; cycle 114 is the most likely landing window. Audit's watch item 5 (per cycle 215 worklog) explicitly named cycle 216 as the substantive Phase 2 critique cycle.

## Forward work for cycle 114+

1. **Audit cycle 216 substantive Phase 2 critique landing absorption** (~next audit cron, ~17h from cycle 113 close). Top priority for cycle 114.
2. **Eva resolution of question-for-eva #2903** — Options A/B/C; 5-cycle autonomy default kicks in if no response by cycle 118 (~1.25 calendar days). Until then, cycle 114+ continues without dispatch substrate.
3. **Continue the central-bet stress-test pattern on remaining load-bearing claims:**
   - "B's central bet requires aspirational extension" — does B's cluster G framing have any retrospective grounding?
   - "C's conditional-improvement risk" — is C's "if additions don't carry their weight, C is just A with more migration surface" framing grounded or motivated?
   - "Migration cost is 3-5× A's" for B — is the LOC estimate methodology defensible against PR #2877's lens-5 critique?
   - The 6-criterion ordering structure assumptions — are the weightings ("Criteria 1+2+3 are highest weight") defensible or asserted?
   - **Each cycle's stress-test target should be selected for highest-leverage finding potential, not for completeness.** The four-finding flip threshold means the next stress-test that surfaces a load-bearing under-weighted dimension triggers the recommendation reversal.
4. **Cycle 110 _notes-named rotation commitment** — three cycles deep on departure; the rotation discipline is conclusively REFUTED. Cycle 114+ should journal this as no-longer-an-active-commitment; per-candidate risk closure can re-enter as a substantive option but no longer as a binding rotation.
5. **Cycle 103-110 risk-closure threshold table integration into 2-selection.md** (cycle 114+) — structured Phase 3 measurement plan; deferred but still load-bearing.
6. **Per-candidate Phase 3 prototype evidence deepening** (cycle 114+) — extending cycles 93-94 measurements to A's `boot-phase` + `wiki-search`; equivalent for C's `reconcile-mode` if Eva pre-approves; cycle 113's findings make C's reconcile-mode measurement more load-bearing than cycle 111 framed.
7. **Symphony deeper-read elevation** (cycle 114+ or based on cluster catalogue findings) — first-pass at cycle 98.
8. **oh-my-claudecode deeper-read elevation** (cycle 114+) — first-pass at cycle 99.
9. **Eva-authored dispatch-test issues #2879 + #2881** — left open per HOUSEKEEPING discipline.

## Meta-observation: the pattern across cycles 111-113

Three consecutive cycles of substrate-distinct work that all share a structural shape:
- Cycle 111: produced the cross-candidate selection-draft deliverable Eva needs at the candidate-selection checkpoint.
- Cycle 112: stress-tested the cycle 111 output, finding motivated reasoning at the family-vs-pattern dominance reading level.
- Cycle 113: stress-tested the cycle 112 reframing, finding under-weighting at the equivalent-outcomes processing level.

**The structural shape is: each cycle's substantive contribution is a finding about the prior cycle's substantive contribution.** This is a self-correcting iteration pattern (each cycle improves the artifact). It is also a pattern that suggests progressive convergence toward an honest characterization of A-vs-C — cycle 111's "A clearly wins" → cycle 112's "A wins on Criteria 1+3 but central-bet rationale is contested" → cycle 113's "A wins on Criteria 1+3 but central-bet rationale weakens further on processing-structure dimensions; if a fourth dimension surfaces the recommendation flips."

**Cycle 114+ may continue this pattern (a fourth stress-test finding), break it (no further under-weighted dimensions surface; the case for A stabilizes), or be interrupted by external substrate (audit cycle 216 critique; question-for-eva #2903 resolution; further Eva input).** The progressive-sharpening pattern itself is candidate for v2 design consideration: the v2 system should encode "stress-test the prior cycle's substantive output before producing this cycle's substantive output" as a load-bearing iteration discipline at checkpoint-approaching cycles.

This pattern is closely analogous to audit's "commitment-thread discipline" (audit cycles 207→211→212→214→215; named cycle 215 as a load-bearing positive pattern). The orchestrator-side equivalent is now visible across cycles 111→112→113. Both disciplines deserve preservation in v2 design.
