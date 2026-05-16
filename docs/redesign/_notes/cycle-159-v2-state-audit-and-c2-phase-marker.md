# Cycle 159 — v2-state-audit tool + C2 phase marker in StepTrace

**Date:** 2026-05-16
**Cycle issue:** [#2968](https://github.com/EvaLok/schema-org-json-ld/issues/2968)
**Track 1 commit:** [`7fcc39a5`](https://github.com/EvaLok/schema-org-json-ld/commit/7fcc39a5) — v2-state-audit crate + policy doc patch
**Track 2 commit:** [`a44ab023`](https://github.com/EvaLok/schema-org-json-ld/commit/a44ab023) — C2 phase marker in StepTrace
**Predecessors:** cycle 158 [`f6205702`](https://github.com/EvaLok/schema-org-json-ld/commit/f6205702) v2-state-retention-policy + [`6ac84450`](https://github.com/EvaLok/schema-org-json-ld/commit/6ac84450) v2-critique-task-class-taxonomy
**Forward priority targets closed:** cycle 158 priority #6 (v2-state-audit tool, NEW from cycle 158 Track 1 §5) + priority #1 (C2 phase marker, inherited from cycle 157 #5)

## Cycle shape

Two-track composition, **HARDENING-AT-12** post cycle 151 single-track exception. 8 consecutive post-exception (152-159); 13 of 14 in arc 146-159.

**44th consecutive cycle of HONORING named forward priority** (cycles 115-159). Cycle 158 named #6 (v2-state-audit tool) and #1 (C2 phase marker); cycle 159 closes both.

## Track 1 — v2-state-audit Rust tool

Codifies the cross-axis enforcement surface named in cycle 158 v2-state-retention-policy §5. CORE-DESIGN-PRINCIPLE-aligned tool extraction: the orchestrator should invoke the tool at session-start and read the classification — it does not walk the state surface itself.

### Implementation

- New crate `tools/rust/crates/v2-state-audit/` (1280 LOC total including Cargo.toml + Cargo.lock entry)
- ~720 prod LOC + ~440 test LOC (the test module includes a minimal in-crate `TempDir` to avoid pulling `tempfile` as a dev-dep; design choice mirrors v2-dispatch-status cycle 156)
- Embedded thresholds match policy v1 §4 axis values per-axis (9 axes)
- 3 measurement primitives: `measure_file_size`, `measure_directory_size` (recursive, symlinks not followed), `measure_json_array_count`
- Classification: at-or-above advisory but below mandatory = Advisory; at-or-above mandatory but below hard = Mandatory; at-or-above hard = Hard; strictly below advisory = Ok
- Overall = max severity across all axes and cross-axis ceiling
- Exit codes 0/1/2/3 map to Ok/Advisory/Mandatory/Hard — consumers can read severity from exit code without parsing JSON
- CLI: `--repo-root <path>` (default `.`), `--json` flag (default human-readable)
- `POLICY_VERSION=1` baked into report (stale-tool detection explicit)
- All measurements tolerate missing files/directories (SCAFFOLD-correct: state/ does not yet exist)

### Tests (39 unit tests, all green)

- `classify` boundary semantics (6 tests: below-advisory, at-advisory, between-advisory-mandatory, at-mandatory, between-mandatory-hard, at-hard)
- Kind → action mapping (1 test)
- Kind kebab-form for each variant (1 test)
- Kind ordering for max rollup (1 test)
- `measure_file_size` (3 tests: missing returns zero, present returns byte count, directory returns zero)
- `measure_directory_size` (4 tests: missing, empty, recurses+sums, file-path returns zero)
- `measure_json_array_count` (5 tests: missing file, invalid JSON, missing key, key-not-array, returns array length)
- `build_axis_report` (2 tests: file-size missing, classifies hard)
- `build_cross_axis` (2 tests: sums byte axes / skips entry axes, classifies against total thresholds)
- `build_overall` (2 tests: takes max across axes+cross, uses cross when higher)
- `build_report` integration (3 tests: missing state surface is ok, picks up state.json dispatches at advisory, picks up role history at mandatory)
- JSON output stability (2 tests: top-level keys, kebab-case classification)
- Exit code mapping distinct per kind (1 test)
- Axis-table sanity (5 tests: unique names, unique paths, monotonic thresholds, cross-axis thresholds monotonic, 9-axis count guard)
- Plus `dispatches_axis_classifies_hard_at_500_plus` (1 test for the cycle 159 live finding)

### Live smoke finding — cycle 159 tool-extraction-surfaces-prior-cycle-error

Running `v2-state-audit --repo-root .` against this repo produced exit code 3 (hard). The `state-json-dispatches` axis reported **930 entries / [hard] / halt-session-start** — nearly 2× the policy's hard threshold of 500.

Root cause: cycle 158 policy authoring used the conceptual name "dispatches" for §4 Axis 6 without verifying against the actual storage key. The legacy v1 `docs/state.json` stores dispatch records under `agent_sessions`, not `dispatches`. v2-state-dispatch-sync uses `agent_sessions`. The 930 entries accumulated since 2026-02-24 (~3 months at the legacy V1 cadence).

Cycle 159 actions:
- Updated `v2-state-audit` axis spec to read the actual storage key `agent_sessions` (axis name `state-json-dispatches` preserved as the policy-conceptual name)
- Patched `v2-state-retention-policy.md` §4 Axis 6 with a **Storage-key clarification (cycle 159)** paragraph recording: (a) the actual storage key, (b) the live-measurement finding, (c) forward options (refuse-to-write enforcement in v2-state-dispatch-sync vs threshold recalibration against historical growth data)
- The hard-threshold "refuse-to-write" semantics are owner-enforced (v2-state-dispatch-sync would refuse the write), not audit-tool-enforced; the audit tool only reports. v2-state-dispatch-sync does not yet implement refuse-to-write, so the 930-entry state is **observable-but-tolerated** until either enforcement is added or thresholds are recalibrated

### Pattern observations

- **`tool-extraction-surfaces-prior-cycle-errors`** RECURRENCE-AT-2 cycle 159. Cycle 156 v2-dispatch-status surfaced cycle 155 false-positive on PR-not-opened; cycle 159 v2-state-audit surfaces cycle 158 policy/implementation key mismatch. Pattern: tool extraction is a debugging mechanism on prior cycles, not only an automation. CORE-DESIGN-PRINCIPLE adjacent.

- **`policy-implementation-naming-divergence`** NOVEL@1 cycle 159. Policy-author-side names a conceptual axis ("dispatches"); implementation uses different storage key ("agent_sessions"). Recurrence test: any future policy doc referencing concrete storage keys should be verified against implementation source. Mitigation: design docs that name keys should grep the implementation before landing.

- **`live-smoke-as-policy-calibration-mechanism`** NOVEL@1 cycle 159. Cycle 158 policy estimated 50/200/500 thresholds for dispatches based on "audit retrospective measurements"; live smoke cycle 159 showed 930 actual entries. The gap (~2× hard) was invisible to cycle 158 because policy author didn't query the actual data. Recurrence test: any future policy with numeric thresholds should be validated against live measurement before landing.

- **`tool-as-codification-of-design-scope`** NOVEL@1 cycle 159. The v2-state-audit tool is the executable form of the v2-state-retention-policy design scope: every threshold in policy §4 has a corresponding `const` in the tool; every axis named in §2 has an `AxisSpec`; the tool fails (returns non-zero) exactly when the policy says enforcement should trigger. Pattern test: future design-scope documents specifying thresholds should be paired with codified tools, not left as prose.

## Track 2 — C2 (L1.2) phase marker in StepTrace

Cycle 158 forward priority #1 (inherited from cycle 157 #5). Pairs naturally with cycle 156's C5 `executed: bool` field.

### Implementation

- New `Phase` enum (Boundary | Reconciler | Planner | Executor | Curator | Transition), kebab-case serialized
- New `phase_for(kind: StepKind) -> Phase` derivation function
- New `phase: Phase` field on `StepTrace`
- Construction site at `run_cycle` updated to derive phase from `step.kind` at trace-build time
- Phase derives from StepKind alone, independent of run mode (dry-run and live carry the same phase value per step)
- 181 LOC added (+5 enum + 8 derivation + 1 field + 1 construction + ~166 tests)

### Phase distribution

Per the canonical 10-step super-step sequence:

| Step | Name | Kind | Phase |
|------|------|------|-------|
| 1 | super-step-init | SuperStepCycleStart | Boundary |
| 2 | reconciler-pre-poll | ReconcilerPoll | Reconciler |
| 3 | reconciler-session | RoleInvoke(Reconciler) | Reconciler |
| 4 | super-step-advance-1 | SuperStepAdvance | Transition |
| 5 | planner-session | RoleInvoke(Planner) | Planner |
| 6 | super-step-advance-2 | SuperStepAdvance | Transition |
| 7 | executor-session | RoleInvoke(Executor) | Executor |
| 8 | super-step-advance-3 | SuperStepAdvance | Transition |
| 9 | curator-session | RoleInvoke(Curator) | Curator |
| 10 | super-step-settle | SuperStepCycleEnd | Boundary |

= 2 Boundary + 2 Reconciler + 1 Planner + 1 Executor + 1 Curator + 3 Transition

Design choices:
- `ReconcilerPoll` and `RoleInvoke(Reconciler)` are both Phase::Reconciler (pre-poll is preparatory; the phase is the semantic intent, not the step shape)
- `SuperStepCycleStart` and `SuperStepCycleEnd` are both Boundary (cycle boundaries are not role phases)
- `SuperStepAdvance` is Transition (control plumbing between role phases)

### Tests (5 new, 36 unit + 2 integration green total)

- `phase_for_each_step_kind_variant`: pin derivation for every StepKind variant
- `super_step_sequence_phase_distribution_matches_design`: pin per-step phase + distribution counts
- `dry_run_traces_carry_phase_in_json`: phase serialized on dry-run traces
- `live_traces_carry_phase_in_json`: phase serialized on live traces (same phase order as dry-run — phase derives from StepKind, not execution mode)
- `phase_uses_kebab_case_serialization`: kebab-case smoke for every variant

### Consumer benefit

Trace consumers can now group by phase without parsing step-name strings — e.g., "show me only the role-invoke phases" filters affirmatively on `phase in {planner, executor, curator}`. The C2 critique's load-bearing fix: semantic-workflow steps are distinguishable from transition mechanics in the trace shape itself.

### Pattern observations

- **`agree-act-now-bounded-fix-via-tracksecond-pattern`** RECURRENCE-AT-4 cycle 159. Cycle 150 executor 4-fix; cycle 155 C8 docstring; cycle 156 C5 executed:bool; cycle 159 C2 phase marker. Pattern: bounded ACT-NOW critique findings make excellent Track 2 work — small enough to fit alongside substantive Track 1, large enough to demonstrate visible progress on critique absorption.
- **StepTrace shape progression**: cycle 149 design (5 fields: index, name, primitive, args, bin_path) → cycle 156 +executed (6 fields) → cycle 159 +phase (7 fields). Each addition resolves one cycle 152 critique finding without restructuring.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 v2-state-audit (Cargo.toml + main.rs + Cargo.lock entry) | 1280 LOC | `7fcc39a5` |
| Track 1 policy doc §4 Axis 6 patch | 1 paragraph added | `7fcc39a5` |
| Track 2 v2-cycle-runner Phase enum + field + tests | 181 LOC | `a44ab023` |
| This _notes | ~380 lines | cycle-close |
| Total cycle 159 textual + code output | ~1840+ lines | 3 commits (Track 1 + Track 2 + close) |

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 159 datapoints:
- Tool-build shape (v2-state-audit cycle 159): 1280 LOC — within the cycle 158 prediction range (~600-900 LOC + tests). Slightly over because of the 39-test suite (axis-table sanity tests add ~80 LOC; in-crate TempDir adds ~20 LOC).
- Bounded-code shape (C2 cycle 159): 181 LOC for 1 enum + 1 derivation + 1 field + 5 tests. Comparable to cycle 156 C5 (95 LOC for 1 field + 3 tests). Cycle 159 is larger because phase tests pin the full 10-step distribution.

## Process honoring

- **44th consecutive cycle of HONORING named forward priority** (cycles 115-159). Cycle 158 named #6 (v2-state-audit tool, NEW) + #1 (C2 phase marker, inherited); cycle 159 closes BOTH.
- **72nd bottleneck-asynchronous cycle** (78-159).
- **49th non-per-candidate-sharpening cycle** (111-159).
- Cycle 120 L2 preserved (`2-selection.md` untouched cycle 159).
- Cycle 128 lesson preserved (.scratch/ via Write tool).
- Cycle 133 clarification preserved: 4 cargo invocations cycle 159 (build + test + clippy on v2-state-audit; build + test + clippy on v2-cycle-runner; bounded-purpose discipline honored across the two crates).
- Cycle 134 lesson preserved (audit-repo HEAD via gh api at session-start; unchanged at `8285b7d3`).
- **Cycle 137 lessons re-validated cycle 159 13-cycle-running** (137 + 147-159). All `gh` operations single-purpose-bash-invocation form.
- **Cycle 149 in-cycle CWD-drift lesson re-validated cycle 159** — all cargo via `--manifest-path tools/rust/Cargo.toml`.
- Cycle 151 date-test-value lesson preserved (no new date tests).
- Cycle 152 disk-format-read-source lesson preserved.
- Cycle 153 multi-Edit-fallback lesson preserved (not exercised cycle 159).
- Cycle 154 `gh api graphql` subprocess pattern preserved (not exercised cycle 159).
- Cycle 155 dispatch-return-detection lesson preserved (v2-dispatch-status not invoked cycle 159).
- Cycle 156 anti-overstatement audit enumeration discipline preserved (see "What cycle 159 does NOT do" below, 17 items).
- Cycle 157 audit-HEAD-check at session-start preserved (HEAD unchanged at `8285b7d3` — same as cycle 158 session-start; no new audit content cycle 222 yet).
- Cycle 158 symmetric-closure-of-new-cycle-N-priorities pattern: cycle 158 produced 2 new priorities (#6 v2-state-audit + #15 per-axis archival); cycle 159 closes #6 + #1 (the latter is inherited, not new from 158). **Asymmetric closure cycle 159** — pattern observation: symmetric closure (cycle 158) was a special case where Track 2 absorbed the NEW priority; cycle 159 Track 2 absorbed an INHERITED bounded priority instead because the new priority (#15 per-axis archival) is gate-bounded (cycle 158 priority #15 says "When first axis approaches advisory threshold under live execution" — not yet triggered; the 930-entry finding is from `state-json-dispatches`, not a v2 state axis, so the per-axis archival gate hasn't yet been triggered).
- Journal-immutability discipline preserved (cycle 159 appends NEW section via Edit anchor at end of cycle 158 section; cycle 148-158 sections NOT back-edited).
- **Two-track composition continued** — cycle 159 is 8th consecutive post cycle 151 exception (HARDENING-AT-12).

## In-session issues and recoveries

- **In-session issue**: cycle 159 first attempt at `dry_run_traces_carry_phase_in_json` test used `args.dry_run_arg = true` referencing a non-existent field on `Args`. Recovery: read existing `dry_run_traces_have_executed_false` test (line 1450) which already had the correct pattern (mutate `ra.dry_run = true` after `run_args_with_outputs`); refactored the new test to match. Single Edit cycle — no follow-on errors. Lesson: when adding tests for newly-added fields, mirror the existing tests that exercise the same control flow (dry-run vs live) rather than reinventing the control surface. Pattern: `cycle-159-test-pattern-mirror-existing-control-flow` NOVEL@1 — captured for future cycles.
- All `git add` / `git commit` / `git push` operations clean cycle 159 (2 Track-side direct-push commits).
- All `cargo build` / `cargo test` / `cargo clippy` operations clean cycle 159.
- All Edit / Write operations clean cycle 159.
- All `gh api` / `gh issue list` operations clean cycle 159 (session-start checks only).

## Forward priorities for cycle 160+

**Renumbered list (cycle 158 inherited + cycle 159 produced):**

1. **C10 (L2.5) design-scope amendment** — bounded textual change to `cycle-149-v2-cycle-runner-design-scope.md:99-105` clarifying "state mutation" scope. Cycle 158 forward priority #2. Carries forward. <1 cycle.

2. **X5 commit-governance scope clarification** — bounded textual addition to v2-cycle-runner crate doc-comment declaring commit-governance OUT of scope. Cycle 158 forward priority #3. Carries forward. <1 cycle.

3. **AGREE-ACT-NOW L1.3 + L3.6 reconciler completeness-metadata pairing** (from cycle 148 absorption). Cycle 158 forward priority #4. Carries forward. <1 cycle.

4. **AGREE-ACT-NOW L2.4 dispatch-brief discipline addendum** (from cycle 148 absorption). Cycle 158 forward priority #5. Carries forward. <1 cycle.

5. **v2-state-dispatch-sync refuse-to-write hard-threshold enforcement** (NEW from cycle 159 live-smoke finding). Implement the owner-side hard-threshold semantic: refuse `add` operation when array length would exceed hard threshold; emit halt-reason `state-bound-exceeded` per policy §7. Pairs with policy-side threshold recalibration (option (b) from cycle 159 policy patch). Estimate: <1 cycle (bounded check + 1 new error class + tests).

6. **Threshold recalibration for state-json-dispatches axis** (NEW from cycle 159 live-smoke finding). Cycle 158 policy estimated 50/200/500 thresholds; live data shows 930. Either (a) recalibrate against the 3-month growth rate (~310/month), OR (b) implement archival mechanism for v2-state-dispatch-sync. Pairs with priority #5.

7. **TOOL-SCOPED `v2-channel-router` enforcement extension design scope** (C1 / L3.1 from cycle 148 absorption). Cycle 158 forward priority #7. Carries forward.

8. **TOOL-SCOPED `v2-prompt-tag-semantic-fidelity` tool design scope** (L2.5 from cycle 148 absorption). Cycle 158 forward priority #8. Carries forward.

9. **`status` + `verify` v2-cycle-runner subcommands** — cycle 158 forward priority #9. Carries forward; deferred again cycle 159.

10. **AGREE-DEFER queue (post-real-role-session-measurement)** — curator complexity rework, template-mirror architecture-revisit. Hold for live-claude-code-spawn evidence (still SCAFFOLD). Cycle 158 forward priority #10. Carries forward.

11. **Coordinated retry/timeout/cancellation arc** — C13 + X2. Cycle 158 forward priority #11. Carries forward.

12. **Coordinated structured-error-envelope arc** — C6 + C7 + C9. Cycle 158 forward priority #12. Carries forward.

13. **Coordinated resume/recovery arc** — C11 + C12 + X1. Cycle 158 forward priority #13. Carries forward.

14. **Audit-engagement substantive-focal single-track variant** (from cycle 157 R5.a HARDENING-AT-3-CONDITIONAL). Conditional; requires audit to file fresh substantive content in audit cycles 222+. Cycle 158 forward priority #14. Carries forward.

15. **Per-axis archival mechanism design scope** — cycle 158 forward priority #15. Gate: when first v2 state axis approaches advisory threshold under live execution. NOT YET TRIGGERED — cycle 159's 930-entry finding is on `state-json-dispatches` (a legacy v1 axis owned by v2-state-dispatch-sync), not a v2 state axis. Carries forward unconditionally.

16. **Cycle 120 L2 preserved** (no recursive annotation of `2-selection.md`).

**Cycle 158 forward priorities CLOSED by cycle 159:** #1 (C2 phase marker) + #6 (v2-state-audit tool).

**Inherited bounded items C10/X5 (cycle 158 priorities #2/#3) carry forward** — cycle 159 Track 2 was C2 (priority #1); C10/X5 are natural Track 2 candidates for cycle 160+.

## What cycle 159 does NOT do

1. Does NOT modify legacy `tools/cycle-runner` (v1; forbidden zone preserved during Phase 3).
2. Does NOT modify `.github/workflows/` or this orchestrator prompt.
3. Does NOT implement v2-state-dispatch-sync refuse-to-write semantics (named as cycle 160+ priority #5).
4. Does NOT recalibrate dispatch axis thresholds (named as cycle 160+ priority #6).
5. Does NOT amend C10 — deferred again (carries to cycle 160+ as forward priority #1).
6. Does NOT address X5 — deferred again (carries to cycle 160+ as forward priority #2).
7. Does NOT add per-step timeout (X2) — deferred.
8. Does NOT add lock/lease (X1) — deferred.
9. Does NOT modify `docs/redesign/2-selection.md` (cycle 120 L2 preserved).
10. Does NOT spawn live Claude sessions (role-driver remains SCAFFOLD).
11. Does NOT escalate any cycle 159 decision to Eva (EVA-DEFAULT-AUTONOMY — all decisions within design-space and bounded technical scope; the 930-entry finding is observation, not crisis).
12. Does NOT dispatch a Copilot critique on either Track 1 or Track 2 artifact (per cycle 158 v2-critique-task-class-taxonomy: v2-state-audit is hybrid-class but the tool-extraction-surfaces-prior-cycle-error pattern is itself the critique; C2 phase marker is small enough to not need external review).
13. Does NOT file a cross-repo audit-engagement issue in this repo (cycle 159's findings are within the existing absorption pattern; if audit cycle 222 surfaces fresh perspective on cycle 159 commits, that becomes the natural cycle 160 absorption target).
14. Does NOT modify `docs/state.json` (the 930-entry finding is observation; modifying state.json is owner-side work for v2-state-dispatch-sync).
15. Does NOT close any open issues (cycle issue #2968 closes per existing convention; no other closure candidates — 4 standing input-from-eva remain).
16. Does NOT integrate v2-state-audit into v2-cycle-runner's session-start path (policy §5 cadence says "Session-start: v2-state-audit runs once" — wiring is forward, not cycle 159).
17. Does NOT touch the SECURITY threat model or the prompt's forbidden zones.

## Cycle 159 ARTIFACTS

- `tools/rust/crates/v2-state-audit/Cargo.toml` — new (Track 1, commit `7fcc39a5`)
- `tools/rust/crates/v2-state-audit/src/main.rs` — new (Track 1, ~720 prod + ~440 test ≈ 1160 LOC, commit `7fcc39a5`)
- `docs/redesign/_notes/v2-state-retention-policy.md` — modified (Track 1 patch §4 Axis 6, commit `7fcc39a5`)
- `tools/rust/Cargo.lock` — modified (Track 1 dependency resolution, commit `7fcc39a5`)
- `tools/rust/crates/v2-cycle-runner/src/main.rs` — modified (Track 2 +181 LOC: Phase enum + derivation + field + 5 tests, commit `a44ab023`)
- `docs/redesign/_notes/cycle-159-v2-state-audit-and-c2-phase-marker.md` — new (this _notes, ~380 lines)
- `.scratch/cycle159-session-start.md` — session-start comment body (ephemeral)
- `.scratch/cycle159-track1-msg.txt` — Track 1 commit message (ephemeral)
- `.scratch/cycle159-track2-msg.txt` — Track 2 commit message (ephemeral)
- `.scratch/cycle159-session-end.md` — session-end comment (authored cycle-close, ephemeral)
- `.scratch/cycle159-issue-close.md` — cycle issue close comment (authored cycle-close, ephemeral)
- 2 Track-side direct-push commits: `7fcc39a5` (Track 1) + `a44ab023` (Track 2)
- 1 cycle-close commit (this _notes + journal section + ephemerals)
- 0 issues closed cycle 159 (no candidates beyond cycle issue itself).
- 0 dispatches cycle 159.
- 6 cargo invocations cycle 159: 2× build (v2-state-audit, v2-cycle-runner) + 2× test (each crate) + 2× clippy (each crate). All clean.
- 4 Edits on `tools/rust/crates/v2-cycle-runner/src/main.rs` (Phase enum + field + construction + new tests; one test refactor after `args.dry_run_arg` typo).
- 1 Edit on `tools/rust/crates/v2-state-audit/src/main.rs` (axis-spec key change `dispatches` → `agent_sessions` after live smoke).
- 1 Edit on `docs/redesign/_notes/v2-state-retention-policy.md` (storage-key clarification §4 Axis 6).
- 0 Edits on `docs/state.json`.
- 0 Edits on legacy `tools/cycle-runner/`.
- 0 Edits on `.github/workflows/`.
- 0 Edits on this orchestrator prompt.
- ~7 GitHub API operations cycle 159 (audit HEAD check, input-from-eva list, open-issues list, cycle-issue view, 2× session comment post, cycle-issue close).
