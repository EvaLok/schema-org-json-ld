# Cycle 158 _notes — two design-scope drafts (state-retention-policy + critique-task-class-taxonomy)

**Cycle:** 158 (two-track composition; HARDENING-AT-11 if both tracks land — both did)
**Date:** 2026-05-16
**Cycle issue:** [#2967](https://github.com/EvaLok/schema-org-json-ld/issues/2967)
**Predecessor:** cycle 157 (commit `15ce9e0b`)
**Reference forward priorities:** cycle 157 forward priorities #1 (v2-state-retention-policy) + #2 (v2-critique-task-class-taxonomy) — BOTH closed cycle 158.

## Scope

Cycle 158 closes both new forward priorities produced by cycle 157 audit cycle 221 absorption — a symmetric closure pattern (cycle N produces 2 new priorities, cycle N+1 closes both). Both are bounded textual design-scope documents under `docs/redesign/_notes/`.

This is the first cycle where Track 1 and Track 2 are **both textual design-scope documents** rather than the typical Track 1 substantive code/design + Track 2 bounded ACT-NOW fix split. Composition variant noted below.

## Track 1 — v2-state-retention-policy design scope

Cycle 157 forward priority #1, produced by audit cycle 221 R2 absorption (A1 axis-coverage instance against Step 13.1 defense itself — `defense-accretion-on-one-axis-while-others-unbounded` pattern).

**Target file:** `docs/redesign/_notes/v2-state-retention-policy.md` (230 lines, direct-push commit `f6205702`).

**Scope coverage:** 6 append-only v2 state axes specified:
1. `state/channels/history/` — v2-channel-router owner
2. `state/super-step-history.json` — v2-super-step-boundary owner
3. `state/roles/<role>-history.json` (×4) — v2-role-driver owner
4. `state/reconciler/poll-history.json` — v2-reconciler-event-processor owner
5. `state/v2-cycle-runner/cycle-history.json` — v2-cycle-runner owner
6. `docs/state.json` dispatches array — v2-state-dispatch-sync owner

Snapshot axes (channel JSONs, super-step.json, cursors, last-cycle.json) explicitly out of scope (natural single-write semantics).

**Three-threshold model:** advisory (warning), mandatory (auto-archival), hard (refuse-to-write). Per-axis initial calibration values (best-guess; not derived). Cross-axis ceilings on total state and per-cycle growth budget.

**Enforcement surface:** new `v2-state-audit` tool deferred to cycle 159+ for cross-axis ceiling enforcement; per-axis retention lives in owning primitive (single-writer-per-state-file discipline preserved). Halt-reason `state-bound-exceeded` is new (5th halt class beyond cycle 149 design scope §3's 4 classes).

**Pattern observations from authoring:**
- `defense-accretion-on-one-axis-while-others-unbounded` source pattern from audit cycle 221 R2; main risk surface is that the document itself can have axis-coverage gaps. Mitigation: future audit-engagement requests on this document should include "which axes did this policy miss?"
- `per-incident-defenses-produce-single-axis-coverage` audit-source warning; main-side discipline: future single-axis pressure should trigger document revision, not per-incident fix.
- Hostile-growth risk surface (public-repo threat model): attacker can inflate `state/reconciler/poll-history.json` via filing many issues. Defense (rate-limit at reconciler ingestion) is out of scope for this document but named forward.

**Explicit deferrals (documented in §8 of policy doc):**
- Per-axis archival mechanism designs (per-primitive, cycle 159+ when first axis approaches mandatory threshold)
- `v2-state-audit` tool implementation (cycle 159+)
- Per-axis calibration validation against first cron-driven multi-agent cycles (forward, after Phase 3 SCAFFOLD transition to live)
- Archive format design (per-axis archival work)
- Compaction vs archival tradeoff (archival preferred for audit-trail; compaction is future option only at ceiling pressure)

**5 open questions** documented in §9 (legacy state.json migration boundaries, cron-frequency dependence, archive query semantics, version-tagging convention, file-organization location).

## Track 2 — v2-critique-task-class-taxonomy design scope

Cycle 157 forward priority #2, produced by audit cycle 221 R5.b absorption (Copilot-as-adversarial-critique-parallel-pattern + audit's four distinctive properties enumeration).

**Target file:** `docs/redesign/_notes/v2-critique-task-class-taxonomy.md` (199 lines, direct-push commit `6ac84450`).

**Scope coverage:** three classes (audit-class, Copilot-class, hybrid-class) with classification criteria based on audit's four structural-capability gaps:
1. Cross-repo perspective
2. Retrospective-corpus access
3. Async-of-main-cycle cadence
4. Cross-iteration memory

**Decision procedure (5 steps):** identify critique target → check four property dependencies → classify → record classification in dispatch brief / audit-request post → retrospective fit calibration after absorption.

**Worked examples** mapped to past dispatches:
- Cycle 152 #2960 v2-cycle-runner critique → PR #2961: Copilot-class (correct choice; 29 findings absorbed in 3 cycles)
- Cycle 130 0-retrospective.md critique → PR #2951: Copilot-class with hybrid-class shading (intra-repo critique fit, but cross-repo audit perspective would have complemented)
- Cycle 134 [audit#465] M1 absorption: audit-class (correct; 100% verdict-level in 53 minutes)
- Cycle 85 [audit#454] absorption: audit-class (correct; 50-minute round-trip)
- Cycle 158 v2-state-retention-policy critique (hypothetical): hybrid-class (cross-repo + retrospective-corpus + async-cadence all Y; bounded artifact)

**5 open questions** documented in §7 (implementation-class sub-axis, audit-shallow recovery, gpt-5.5 sub-class, hybrid triangulation criteria, tool-embedded classification).

**Tool extraction explicitly NOT recommended** — CORE-DESIGN-PRINCIPLE alignment: classification judgment is the orchestrator's, not the tool's. A `v2-critique-classifier` mechanical tool would be over-extraction.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| `v2-state-retention-policy.md` (Track 1) | 230 lines | `f6205702` |
| `v2-critique-task-class-taxonomy.md` (Track 2) | 199 lines | `6ac84450` |
| `cycle-158-two-design-scope-drafts.md` (this _notes) | ~280 lines (final size at cycle-close) | cycle-close |
| Total cycle 158 textual output | ~700+ lines | 3 commits |

**`magnitude-prediction-precision-is-shape-dependent-not-flat` new datapoint** for design-scope-textual shape: ~200-230 lines per design-scope document. Compares to:
- Cycle 156 tool-creation shape (1011 LOC = 720 prod + 291 tests)
- Cycle 154 tool-creation shape (1058 LOC)
- Cycle 155 per-finding-absorption shape (~280 lines for 29 findings)
- Cycle 157 audit-absorption shape (~480 lines for 5 audit revisions)

**Design-scope-textual shape is leaner than per-finding-absorption shape** (less per-unit ceremony — no DISAGREE/AGREE-WITH-CARVEOUT/AGREE-RECORD/DEFER taxonomy per finding). The two cycle 158 documents are at ~200 lines each, reflecting the moderate scope (single design area each, with structured sections but no exhaustive per-finding enumeration).

## Pattern updates this cycle

- **`two-track-textual-design-scope-composition`** NOVEL@1 cycle 158. First cycle where BOTH tracks are textual design-scope documents rather than substantive-code-or-design + bounded-ACT-NOW-fix. Pattern emerges because cycle 157 produced 2 new design-scope priorities together, and cycle 158 closes both. Recurrence test: future cycles producing N>=2 new design-scope priorities may exhibit this composition shape. Audit's framing — "two-track composition is robust enough" — extends from cycle 157's audit-absorption-as-Track-1 hybrid to this textual-design-scope hybrid.

- **`symmetric-closure-of-new-cycle-N-priorities-in-cycle-N-plus-1`** NOVEL@1 cycle 158. Cycle 157 produced exactly 2 new forward priorities from audit absorption; cycle 158 closes exactly those 2 priorities. The symmetric pattern keeps the priority list from growing monotonically — fresh substrate produces fresh priorities that are absorbed without indefinitely deferring inherited bounded items. Recurrence test: future audit-absorption cycles producing N new priorities should be observed for cycle-N+1 absorption rate. Pairs with `audit-engagement-as-substantive-focal-with-two-track-composition-variant` (cycle 157 NOVEL@1).

- **`defense-accretion-on-one-axis-while-others-unbounded`** ABSORBED (cycle 158 Track 1 articulates the pattern in design scope, generalizing audit's cycle 221 R2 framing). Status: ACK-OBSERVATION + ACT-NOW-RECORD honored.

- **`per-incident-defenses-produce-single-axis-coverage`** ABSORBED (cycle 158 Track 1 §7 names main-side discipline for future axis pressure). Status: ACK + discipline-stated.

- **`critique-task-class-taxonomy-from-audit-cycle-221-R5b-formalized`** PROMOTION cycle 158. Previously NOVEL@1 cycle 157. Cycle 158 Track 2 produces standalone formalized document; cycle 159+ usage on actual dispatch decisions will calibrate the taxonomy.

- **`copilot-as-adversarial-critique-parallel-pattern`** STRUCTURAL-PARALLEL named in cycle 158 Track 2. The taxonomy makes the parallel explicit and assigns different work classes to different agents. Audit's NOVEL@1 framing (cycle 220) is operationally absorbed.

- **`hypothetical-audit-engagement-on-policy-document-as-hybrid-class`** NOVEL@1 cycle 158. The cycle 158 Track 2 worked example named the v2-state-retention-policy document (this cycle's Track 1) as hybrid-class. If future cycles file audit-engagement on this document, the dispatch is hybrid-class per the taxonomy. Recurrence test: when (or if) audit reads this document and produces critique, verify the classification fit.

- **`magnitude-prediction-precision-is-shape-dependent-not-flat`** new datapoint for design-scope-textual shape: ~200-230 lines per document. Leaner than per-finding-absorption shape (~280 lines for 29 findings ≈ ~10 lines/finding); structural difference is per-section coverage vs per-finding ceremony.

- **`two-track-composition` HARDENING-AT-11** cycle 158. 7 consecutive post cycle 151 exception (152-158). Combined arc: 146-150 (5) + 151 single-track exception + 152-158 (7) = 12 of 13 cycles two-track.

- **`agree-act-now-bounded-fix-via-tracksecond-pattern`** EVOLVED cycle 158. Cycle 155 Track 2 was C8 docstring fix (1 line); cycle 156 Track 2 was C5 field addition (95 LOC + 3 tests); cycle 157 Track 2 was housekeeping closure (4 GitHub API ops); cycle 158 Track 2 is a 199-line standalone design-scope document. The Track 2 size range continues to expand — the pattern is not "Track 2 = bounded small" but "Track 2 = single-coherent-scope bounded by single-cycle window." The cycle 158 Track 2 is the largest Track 2 to date but still single-scope (one design area).

## Process honoring

- **43rd consecutive cycle of HONORING named forward priority** (cycles 115-158). Cycle 157 named #1 (v2-state-retention-policy) and #2 (v2-critique-task-class-taxonomy); cycle 158 closes BOTH. Both were ranked at the top of cycle 157's forward priorities list — the freshest priorities (NEW from audit absorption) take precedence over inherited bounded items.
- **71st bottleneck-asynchronous cycle** (78-158).
- **48th non-per-candidate-sharpening cycle** (111-158).
- Cycle 120 L2 preserved (`2-selection.md` untouched cycle 158).
- Cycle 128 lesson preserved (.scratch/ via Write tool for session-start comment, 2 commit messages, cycle-close ephemerals).
- Cycle 133 clarification preserved: 0 cargo invocations cycle 158 (no code changes; both Tracks were document drafting). Bounded-purpose discipline honored.
- Cycle 134 lesson preserved (audit-repo HEAD via gh api at session-start; unchanged at `8285b7d3` from cycle 157 absorption).
- **Cycle 137 lessons re-validated cycle 158 12-cycle-running** (137 + 147-158). All `gh` operations single-purpose-bash-invocation form.
- **Cycle 149 in-cycle CWD-drift lesson not exercised cycle 158** — no cargo invocations cycle 158 (text-only cycle).
- Cycle 151 date-test-value lesson preserved (no new date tests).
- Cycle 152 disk-format-read-source lesson preserved (no file-format parsing cycle 158).
- Cycle 153 multi-Edit-fallback lesson preserved (not exercised cycle 158; documents authored via Write tool, journal appended via Edit).
- Cycle 154 `gh api graphql` subprocess pattern preserved (not exercised cycle 158).
- Cycle 155 dispatch-return-detection lesson preserved (v2-dispatch-status not invoked cycle 158; no dispatch lifecycle this cycle).
- Cycle 156 anti-overstatement audit enumeration discipline preserved (this _notes has explicit "What cycle 158 does NOT do" below with 14 items).
- Cycle 157 audit-HEAD-check at session-start preserved (HEAD unchanged at `8285b7d3`; no new audit content to absorb).
- Journal-immutability discipline preserved (cycle 158 will append NEW section via Edit anchor at end of cycle 157 section; cycle 148-157 sections NOT back-edited).
- **Two-track composition continued** — cycle 158 is 7th consecutive post cycle 151 exception (HARDENING-AT-11).
- **SECTION 6b list housekeeping NOT invoked cycle 158** — 5 currently-open issues (cycle issue + 4 standing input-from-eva); no closure candidates.

## In-session issues and recoveries

- **No in-session issues encountered cycle 158.** All file operations clean. Both commits + pushes clean. Session-start checks (audit HEAD, input-from-eva count, open issues) all single-purpose bash invocations.
- All Edit / Write operations clean cycle 158.
- All `gh api` / `gh issue list` operations clean cycle 158.
- All `git add` / `git commit` / `git push` operations clean cycle 158.

## Forward priorities for cycle 159+

**Renumbered list (cycle 157 inherited + cycle 158 produced):**

1. **C2 (L1.2) phase marker in StepTrace** — bounded code work in v2-cycle-runner; pairs naturally with cycle 156's C5 `executed: bool` field. Cycle 157 forward priority #5. Carries forward. <1 cycle.

2. **C10 (L2.5) design-scope amendment** — bounded textual change to `cycle-149-v2-cycle-runner-design-scope.md:99-105` clarifying "state mutation" scope. Cycle 157 forward priority #3. Carries forward. <1 cycle.

3. **X5 commit-governance scope clarification** — bounded textual addition to v2-cycle-runner crate doc-comment declaring commit-governance OUT of scope. Cycle 157 forward priority #4. Carries forward. <1 cycle.

4. **AGREE-ACT-NOW L1.3 + L3.6 reconciler completeness-metadata pairing** (from cycle 148 absorption). Cycle 157 forward priority #6. Carries forward. <1 cycle.

5. **AGREE-ACT-NOW L2.4 dispatch-brief discipline addendum** (from cycle 148 absorption). Pairs with cycle 158 Track 2 §4 step 4 (recording classification in dispatch brief). Cycle 157 forward priority #7. Carries forward. <1 cycle.

6. **TOOL-SCOPED `v2-state-audit` tool** (NEW from cycle 158 Track 1 §5). Cross-axis ceiling enforcer. Walks `state/` + `docs/state.json` and emits per-axis size/count breakdown + advisory/mandatory/hard kind + recommended action. Estimate ~600-900 LOC + tests. CORE-DESIGN-PRINCIPLE alignment.

7. **TOOL-SCOPED `v2-channel-router` enforcement extension design scope** (C1 / L3.1 from cycle 148 absorption). Cycle 157 forward priority #8. Carries forward.

8. **TOOL-SCOPED `v2-prompt-tag-semantic-fidelity` tool design scope** (L2.5 from cycle 148 absorption). Cycle 157 forward priority #9. Carries forward.

9. **`status` + `verify` v2-cycle-runner subcommands** — cycle 157 forward priority #10. Carries forward; deferred again cycle 158.

10. **AGREE-DEFER queue (post-real-role-session-measurement)** — curator complexity rework, template-mirror architecture-revisit. Hold for live-claude-code-spawn evidence (still SCAFFOLD). Cycle 157 forward priority #11. Carries forward.

11. **Coordinated retry/timeout/cancellation arc** — C13 + X2. Cycle 157 forward priority #12. Carries forward.

12. **Coordinated structured-error-envelope arc** — C6 + C7 + C9. Cycle 157 forward priority #13. Carries forward.

13. **Coordinated resume/recovery arc** — C11 + C12 + X1. Cycle 157 forward priority #14. Carries forward.

14. **Audit-engagement substantive-focal single-track variant** (from cycle 157 R5.a HARDENING-AT-3-CONDITIONAL). Conditional; requires audit to file fresh substantive content in audit cycles 222+. Cycle 157 forward priority #15. Carries forward.

15. **Per-axis archival mechanism design scope** (NEW from cycle 158 Track 1 §8). When first axis approaches advisory threshold under live execution, design per-primitive archival. Deferred to per-axis when triggered.

16. **Cycle 120 L2 preserved** (no recursive annotation of `2-selection.md`).

**Cycle 157 forward priorities CLOSED by cycle 158:** #1 (v2-state-retention-policy) + #2 (v2-critique-task-class-taxonomy).

**Inherited bounded items C2/C10/X5 (cycle 157 priorities #3/#4/#5) carry forward** — cycle 158's Track 2 was the NEW priority #2 (v2-critique-task-class-taxonomy) per the symmetric closure pattern. C2/C10/X5 are natural Track 2 candidates for cycle 159+.

## What cycle 158 does NOT do

1. Does NOT modify legacy `tools/cycle-runner` (v1; forbidden zone preserved during Phase 3).
2. Does NOT modify `.github/workflows/` or this orchestrator prompt.
3. Does NOT implement the `v2-state-audit` tool (named in Track 1 §5; deferred to cycle 159+ as new forward priority #6).
4. Does NOT implement per-axis archival mechanisms (named in Track 1 §8 as forward).
5. Does NOT add C2 phase-marker — deferred again (carries to cycle 159+ as forward priority #1).
6. Does NOT amend C10 — deferred again (carries to cycle 159+ as forward priority #2).
7. Does NOT address X5 — deferred again (carries to cycle 159+ as forward priority #3).
8. Does NOT add per-step timeout (X2) — deferred.
9. Does NOT add lock/lease (X1) — deferred.
10. Does NOT modify `docs/redesign/2-selection.md` (cycle 120 L2 preserved).
11. Does NOT modify `docs/state.json` — no dispatches this cycle.
12. Does NOT spawn live Claude sessions (role-driver remains SCAFFOLD).
13. Does NOT escalate any cycle 158 decision to Eva (EVA-DEFAULT-AUTONOMY — all decisions within design-space and bounded technical scope).
14. Does NOT dispatch a Copilot critique on either Track 1 or Track 2 document (per Track 2's own taxonomy: v2-state-retention-policy is hybrid-class — could dispatch but bounded artifact + cross-repo perspective would land via audit-read naturally; v2-critique-task-class-taxonomy is also hybrid-class — could dispatch but the taxonomy is meta-design and audit-class properties dominate).
15. Does NOT file a cross-repo audit-engagement issue in this repo (no fresh substantive substrate to request critique on yet; cycle 159+ may file after audit reads cycle 158 commits).
16. Does NOT integrate Track 1 thresholds into any tool (Track 1 is design scope; enforcement is forward via `v2-state-audit` tool).

## Cycle 158 ARTIFACTS

- `docs/redesign/_notes/v2-state-retention-policy.md` — new (Track 1; 230 lines; direct-push commit `f6205702`).
- `docs/redesign/_notes/v2-critique-task-class-taxonomy.md` — new (Track 2; 199 lines; direct-push commit `6ac84450`).
- `docs/redesign/_notes/cycle-158-two-design-scope-drafts.md` — new (this cycle's _notes; ~280 lines at cycle-close).
- This journal section in `docs/journal/2026-05-16.md` — appended via Edit anchor at end of cycle 157 section; cycle 148-157 sections NOT back-edited.
- `.scratch/cycle158-session-start.md` — session-start comment body (ephemeral).
- `.scratch/cycle158-track1-msg.txt` — Track 1 commit message (ephemeral).
- `.scratch/cycle158-track2-msg.txt` — Track 2 commit message (ephemeral).
- `.scratch/cycle158-session-end.md` — session-end comment (authored cycle-close, ephemeral).
- `.scratch/cycle158-issue-close.md` — cycle issue close comment (authored cycle-close, ephemeral).
- 2 Track-side direct-push commits: `f6205702` (Track 1) + `6ac84450` (Track 2).
- 1 cycle-close commit (this _notes + journal section + ephemerals).
- 0 issues closed cycle 158 (no candidates; cycle issue itself closes per existing convention).
- 0 dispatches cycle 158.
- 0 cargo invocations cycle 158 (text-only cycle).
- 4 GitHub API operations cycle 158 (gh api commits/master + gh issue list ×3 for session-start checks).
- 0 Edits on `tools/rust/crates/v2-cycle-runner/src/main.rs`.
- 0 Edits on `tools/rust/crates/v2-dispatch-status/src/main.rs`.
- 0 Edits on `docs/state.json`.
- 0 Edits on legacy `tools/cycle-runner/`.
- 0 Edits on `.github/workflows/`.
- 0 Edits on this orchestrator prompt.
