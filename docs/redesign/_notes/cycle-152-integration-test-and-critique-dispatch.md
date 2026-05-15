# Cycle 152 _notes — v2-cycle-runner integration test against real primitives + adversarial-critique dispatch

**Cycle:** 152 (two-track composition resumed)
**Date:** 2026-05-15
**Cycle issue:** [#2959](https://github.com/EvaLok/schema-org-json-ld/issues/2959)
**Commits this cycle:** `335f1199` (Track 1)
**Track 2 dispatch:** [#2960](https://github.com/EvaLok/schema-org-json-ld/issues/2960) (feedback-only critique on v2-cycle-runner)
**Predecessor:** cycle 151 single-track `run` subcommand (commit `6e13a3a7`)
**Reference design scope:** [`cycle-149-v2-cycle-runner-design-scope.md`](cycle-149-v2-cycle-runner-design-scope.md) §7.2, §11

## Two-track composition shape

Cycle 151 was single-track per the cycle 149 design scope §11 explicit shape directive ("Run subcommand implementation … Single direct-push commit"). Cycle 152's design-scope wording for §7.2 (integration test against real primitives) does NOT specify cycle shape — leaving composition open. Cycle 152 chose two-track composition for two reasons:

1. **Substantive focal (integration test) is bounded.** ~422 LOC of test code + cargo test verification. Doesn't fully consume a 75-min session.
2. **Input-from-eva #2937 directive (2026-05-14) — explicitly considers dispatch-fit each cycle.** Phase 3 cycles 149-151 had zero dispatches. Cycle 152 surfaces a clear dispatch opportunity: adversarial critique on the v2-cycle-runner crate (1421 LOC + 30 tests now landed cycles 150-152).

Resumes the `two-track-composition` pattern broken cycle 151. Pattern observation: `design-scope-authoritativeness-prevails-when-explicit-falls-back-to-cycle-composition-discretion-when-implicit` — cycle 151 had explicit scope direction (single-track), cycle 152 had only scope direction on the substantive focal (integration test) so composition reverted to two-track default.

## What got built (Track 1)

Single new file: `tools/rust/crates/v2-cycle-runner/tests/integration_cycle.rs`.
+422 LOC, single direct-push commit `335f1199`.

### Two integration tests (per cycle 149 design scope §7.2)

1. **`dry_run_against_real_primitives_traces_ten_steps_without_state_mutation`** — builds the 4 v2-* primitive binaries via `cargo build -p` (idempotent, gated by `std::sync::Once` for test-process-level once-only), runs `v2-cycle-runner init` against a TempDir-rooted repo, snapshots state-dir contents, runs `run --cycle 1 --issue 99999 --dry-run`, asserts:
   - exit success
   - 10 "step N" lines in trace output
   - all state files byte-identical pre/post (no mutation)

2. **`live_run_against_real_primitives_writes_completed_state_with_no_halt_marker`** — same setup, hand-prepares 4 session-output JSONs with payload shapes matching channel-router required-keys per channel:
   - inbound-channel (reconciler) → `{eva-responses: [], audit-posts: [], dispatch-returns: []}`
   - plan-channel (planner) → `{substantive-focal: "...", per-role-tasks: {...}}`
   - work-channel (executor) → `{artifacts-written: []}`
   - memory-channel (curator) → `{consolidated-insights: "..."}`

   Runs `run --cycle 1 --issue 99999` with 4 `--*-output-file` flags (no `--dry-run`), asserts:
   - `state/v2-cycle-runner/last-cycle.json` has `status=completed`, `cycle=1`, `issue=99999`, `steps_attempted=10`, `halt_step` / `halt_reason` / `halted_after_role` all null
   - `state/v2-cycle-runner/cycle-history.json` has 1 `cycles[]` entry with `status=completed`
   - `state/super-step-history.json` has 1 `cycles[]` entry with `cycle=1`
   - all 4 `state/channels/<channel>.json` files present with `cycle=1`

### Helper machinery in tests/integration_cycle.rs

- `workspace_root()` / `workspace_target()` / `primitive_bin_path()` — derive workspace paths from `env!("CARGO_MANIFEST_DIR")`
- `cycle_runner_bin()` — uses `env!("CARGO_BIN_EXE_v2-cycle-runner")` (cargo provides this for integration tests in the bearing crate)
- `build_primitives_once()` — std::sync::Once-gated cargo build of 4 primitives + presence assertion
- `primitive_bin_args(repo_root)` — builds the 5-flag tuple (--repo-root + 4 --*-bin)
- `collect_files(root)` — stdlib-only recursive file walk (no walkdir dep)
- `write_json` / `read_json` — minimal serde_json wrappers

### Significance: first end-to-end real-primitive cycle execution

The in-crate unit tests (cycle 151's 28 tests) use `MockInvoker`. Cycle 152's integration tests exercise the actual primitive-to-primitive coordination across the cycle. The full 10-step sequence runs:

```
super-step-boundary cycle-start --cycle 1
reconciler-event-processor poll --cycle 1
role-driver invoke --role reconciler (writes inbound-channel)
super-step-boundary advance (verifies inbound-channel)
role-driver invoke --role planner (writes plan-channel)
super-step-boundary advance (verifies plan-channel)
role-driver invoke --role executor (writes work-channel)
super-step-boundary advance (verifies work-channel)
role-driver invoke --role curator (writes memory-channel)
super-step-boundary cycle-end --cycle 1
```

All advance verifications pass against real channel-state files written by real role-driver invocations using real channel-router payload validation. **Super-step-boundary's strict-verify path (no `--skip-verify`) is exercised end-to-end.**

Per cycle 149 design scope §11, this work was scheduled for cycle 152; **first measurement (cycle 153 focal)** follows naturally — and is actually one cycle ahead of its original §11 schedule since the integration test demonstrates that the architecture works end-to-end, not just at the dry-run layer.

## In-cycle issues + recoveries (Track 1)

1. **First test attempt failed on disk-format assumption.** I initially asserted `last.get("dry_run").as_bool() == Some(false)` and `cycle-history.json.as_array()` — but the actual write_runner_state hand-builds JSON via `serde_json::json!` macro (not derived from CycleReport) and writes:
   - `last-cycle.json`: fields `cycle`, `issue`, `status`, `started_at`, `ended_at`, `halt_reason`, `halt_step`, `halted_after_role`, `steps_attempted` (NO `dry_run` field; uses `halt_reason` not `halt_class`).
   - `cycle-history.json`: shape `{"cycles": [...]}`, NOT bare array.
   - `super-step-history.json` (written by super-step-boundary): same `{"cycles": [...]}` shape.

   Recovered by reading the actual write paths in source + adjusting asserts.

   **Lesson:** when writing integration tests against a system you didn't fully author, don't assume serde-derived field names — read the actual write paths in source first. The `CycleReport` struct has a `dry_run` field; the hand-built JSON written to disk does not.

2. **CWD-drift / `cd tools/rust` chains avoided cycle 152** — all cargo invocations used `--manifest-path tools/rust/Cargo.toml` from repo root. 4-cycle running validation (cycle 149 + 150 + 151 + 152).

3. **No clippy warnings on either pass.** Despite ~422 LOC of new test code with a custom recursive walker, JSON helpers, and trait-style helpers, `clippy --all-targets -D warnings` was clean.

4. **No shell-syntax / for-loop blocks cycle 152** — single-purpose bash invocations throughout. Cycle 137 lesson preserved without re-validation event this cycle.

## What got dispatched (Track 2)

Issue **[#2960](https://github.com/EvaLok/schema-org-json-ld/issues/2960)** — adversarial critique dispatch on v2-cycle-runner crate.

**Brief shape:**
- Frames task as adversarial critique (anti-flattery explicit; "looks good findings are zero-signal — skip them entirely")
- Identifies artifact: v2-cycle-runner at HEAD `335f1199`, including the cycle 152 integration tests
- Names 10 critique-surface areas (sequence shape; FailureClass taxonomy; PrimitiveInvoker trait; iso8601 algorithm; halt semantics; session-output mechanism; CLI ergonomics; test mix; missing concerns; integration test design itself) plus cross-cutting questions on repo-root layout, commit-shape discipline, reducer-rule reliance, failure-path test coverage.
- Specifies delivery as a commit-as-file at `docs/redesign/_critique/cycle-152-v2-cycle-runner-adversarial-critique.md` (per cycle 146 lesson about Copilot comment-write permission gap).
- Specifies PR title format for absorption ease.

**Dispatch verification:**
- `tools/dispatch-task` ran clean with `--skip-pipeline-gate` (redesign-mode required) and `--label agent-task --label feedback-only` (matches `COPILOT-DISPATCHES` taxonomy).
- Issue #2960 created.
- Copilot agent `assigned` and `connected` events within ~10 seconds (verified via gh api).
- Receipt commit `b9994e4` (dispatch-task auto-records in state.json).

**Warning observed:** `dispatch-task` reported "in-flight dispatches at 11 (approaching/exceeding concurrency limit of 2)". This is a stale state.json signal — redesign-mode is not subject to the 2-slot cap (redesign prompt SECTION 2 `COPILOT-DISPATCHES`: "Capacity limits are your judgment call during this phase — there is no enforced 2-slot cap"). The 11-figure includes long-stale dispatches from Phase 1 / Phase 2 that were never closed in state.json. Cycle 152 does NOT clean these up; that's a housekeeping pass for a future cycle (likely paired with `tools/housekeeping-scan` or similar). Captured as a finding for forward priorities.

**Async absorption:** the critique commit (when it lands as PR-from-Copilot) will be readable as a file; absorption is cycle 153 or 154 focal per the cycle 146-148 pattern (24 findings per-finding evaluated, AGREE-ACT-NOW/DEFER/RECORD/WITH-CARVEOUT/DISAGREE verdicts). Main orchestrator does NOT wait on Eva approval — the PR can stay draft; the file is the absorption artifact.

## Substantive measurements (cycle 152)

### LOC

| Component | Cycle 151 | Cycle 152 delta | Cycle 152 post |
|---|---|---|---|
| `src/main.rs` | 1408 | 0 | 1408 |
| `tests/integration_cycle.rs` | n/a (file didn't exist) | +422 (new) | 422 |
| `Cargo.toml` | 13 | 0 | 13 |
| **Crate total** | 1421 | +422 | 1843 |

### Runner-shape LOC family — second datapoint

Cycle 151 first datapoint: 1421 LOC at minimal-3-subcommand point (no integration tests).
Cycle 152 datapoint: 1843 LOC at minimal-3-subcommand point + integration tests.

Test infrastructure adds ~30% to crate size. Cycle 149 design scope §10 declined to predict; cycle 151 sketched ~1700-2200 LOC complete-arc-with-status+verify. With 2 more subcommands (~150-300 LOC each per cycle 151 estimate) plus their integration test extensions, complete-arc ceiling adjusts to **~2300-3000 LOC at 5-subcommand point** with integration tests. Future status/verify cycles can refine; recurrence test for `magnitude-prediction-precision-is-shape-dependent-not-flat`.

### Test count

- Cycle 151 total: 28 unit tests (no integration tests)
- Cycle 152 addition: +2 integration tests
- Cycle 152 total: **30 tests** (28 unit + 2 integration)

### End-to-end run measurement (smoke)

Live integration test successfully:
- Exercises 10/10 super-step transitions
- Writes 4 channel-state files
- Writes super-step.json + super-step-history.json
- Writes v2-cycle-runner/last-cycle.json + cycle-history.json
- Writes 4 role-history files
- Writes reconciler/poll-history.json
- Total state files post-run: ~20 (matching the cycle 150 init-only count plus updates)

This is **not** yet a token / wallclock measurement (cycle 153 focal). It IS the structural evidence that end-to-end the architecture writes the expected state shape from a clean init.

## Pattern updates

- **`two-track-composition` resumed after single-track cycle 151 break.** Pattern resumes at HARDENING-AT-6 (cycles 146, 147, 148, 149, 150, 152 — six cycles within a 7-cycle window; cycle 151 is the explicit single-track exception). Pattern observation: `pattern-momentum-can-survive-single-out-of-band-cycle-when-the-out-of-band-cycle-has-explicit-justification`. NOVEL@1 cycle 152.
- **`integration-test-against-real-primitives-completes-cycle-149-§7.2`** NOVEL@1 cycle 152. The design scope §7.2 explicit two-test spec is now landed exactly as named. Watch for design-scope-§-X-completion-cycle as a tracked pattern when other §-named scopes complete.
- **`disk-format-assumption-bites-first-test-attempt`** NOVEL@1 cycle 152. Even when authoring tests against my own recently-written code, the difference between `CycleReport` struct (serde) and `write_runner_state` hand-built JSON (different field names) caught me. Lesson: read the write path in source before authoring asserts; never assume derived field names.
- **`copilot-dispatch-on-phase-3-prototype-after-three-cycles-of-no-dispatch`** NOVEL@1 cycle 152. Honors input-from-eva #2937 directive after cycles 149-151 had no dispatches.
- **`feedback-only-dispatch-on-substantial-rust-crate`** NOVEL@1 cycle 152. Cycle 146 dispatched feedback-only on 4 role-prompt XMLs (~2000 lines total). Cycle 152 dispatches feedback-only on a single ~1800-LOC Rust crate. Different artifact shape, same critique mechanism.
- **`magnitude-prediction-precision-is-shape-dependent-not-flat`** band-extension for runner-shape family at minimal-3-subcommand + integration tests point: 1843 LOC.
- **`pre-pipeline-gate-stale-dispatch-count-warning`** NOVEL@1 cycle 152. `tools/dispatch-task` reported in-flight dispatches at 11 (the cap is 2 in production mode); this is stale state.json data from Phase 1 / Phase 2 dispatches that were never closed in state.json. Forward-priority housekeeping item.

## Forward priorities for cycle 153+

(Re-ordering of cycle 151's 11-item list; full detail to come from cycle 153 _notes.)

1. **Cycle 153 substantive focal: first end-to-end smoke + first measurement** (cycle 149 design scope §11 + cycle 148 forward priority #2). Now genuinely unblocked — cycle 152's integration test demonstrates the architecture works end-to-end at the dry-run AND live levels. Measurement targets: per-role wall-clock per step (capture-able via runner trace timestamps if a `--measure` flag is added), state-file size delta per step, halt-rate baseline (cycle 152 0%), channel-write-reject-rate baseline (cycle 152 0%), side-channel-leak-rate baseline (cycle 152 0%). Token usage NOT capturable until v2-role-driver gets live-spawn.

2. **Cycle 153 or 154: absorb dispatch #2960 critique** when the file lands at `docs/redesign/_critique/cycle-152-v2-cycle-runner-adversarial-critique.md`. Follow cycle 148 absorption pattern (per-finding evaluation with verdict classification).

3. **AGREE-ACT-NOW L1.3 + L3.6 reconciler completeness-metadata pairing** — bounded; queue for cycle 153+ pairing with substantive focal.

4. **State.json dispatch-tracking housekeeping** — clean up stale dispatch entries (Phase 1 / Phase 2 dispatches that were never closed in state.json; current count "11" is misleading). Forward priority captured cycle 152.

5. **AGREE-ACT-NOW L2.4 dispatch-brief discipline addendum** — document-only.
6. **AGREE-ACT-NOW X2 honest cycle-1-scope-redefinition** — document-only.
7. **AGREE-RECORD L1.2 + X4 side-channel architecture-notes doc** — document-only.
8. **TOOL-SCOPED v2-channel-router enforcement extension design scope** (C1 / L3.1) — design scope first.
9. **TOOL-SCOPED v2-prompt-tag-semantic-fidelity tool design scope** (L2.5) — design scope first.
10. **`status` + `verify` v2-cycle-runner subcommands** — cycle 154+ if needed; cycle 153 first-measurement is the higher-priority deliverable.
11. **Cycle 120 L2 preserved.**

## What cycle 152 does NOT do

- Does NOT implement `status` / `verify` subcommands (cycle 154+ if needed).
- Does NOT modify `cycle-runner` (forbidden zone preserved during Phase 3).
- Does NOT modify `.github/workflows/` or this orchestrator prompt.
- Does NOT spawn live Claude sessions (v2-role-driver remains SCAFFOLD per cycle 149 design scope OQ2).
- Does NOT capture token usage or wallclock-per-step measurements (cycle 153 focal).
- Does NOT enact 5 remaining AGREE-ACT-NOW findings (L1.3+L3.6, L2.4, X2, L1.2+X4 architecture-notes).
- Does NOT clean up state.json stale dispatch entries (forward-priority captured; cycle 153+).
- Does NOT escalate any decision to Eva (EVA-DEFAULT-AUTONOMY: design-scope-driven cycle, dispatch-fit named in #2937).
- Does NOT modify `docs/redesign/2-selection.md` (cycle 120 L2 preserved).
- Does NOT absorb the dispatch #2960 critique (it hasn't returned yet; cycle 153 or 154 focal).

## Process honoring (cycle 152)

- **37th consecutive cycle of HONORING named forward priority** (cycles 115-152).
- **65th bottleneck-asynchronous cycle** (78-152).
- **42nd non-per-candidate-sharpening cycle** (111-152).
- Cycle 120 L2 preserved (2-selection.md untouched).
- Cycle 128 lesson preserved (.scratch/ via Write tool for session-start + commit-message + dispatch-body + cycle-close ephemerals).
- Cycle 133 clarification preserved: cargo invocations cycle 152 are legitimate (build primitives + test v2-cycle-runner + clippy v2-cycle-runner; not gratuitous cargo).
- Cycle 134 lesson preserved (audit-repo HEAD via gh api — not exercised cycle 152 since audit HEAD unchanged since cycle 151 absorption).
- Cycle 137 lessons preserved cycle 152 (no for-loop attempts; no shell-syntax-string blocks; no heredoc redirects). 7-cycle running validation pause (137 validated at 147+148+149+150+151; cycle 152 didn't exercise the risk).
- Cycle 149 in-cycle CWD-drift lesson preserved cycle 152 (zero `cd tools/rust` chains; all cargo via `--manifest-path`). 4-cycle running validation (149 → 150 → 151 → 152).
- Cycle 151 in-cycle date-test-value lesson preserved cycle 152 (no new date tests this cycle; lesson unexercised but preserved).
- **NEW cycle 152 in-cycle lesson:** when writing integration tests against your own code, do NOT assume serde-derived field names; read the actual write path in source first. The 5-minute fix-loop was the cost of NOT doing this. Lesson captured.
- Journal-immutability discipline preserved (cycle 152 appends NEW section via Edit anchor at end of cycle 151 section; cycle 148/149/150/151 sections NOT back-edited).
- Anti-overstatement audit explicit ("What cycle 152 does NOT do" enumeration above).
- **Two-track composition resumed** after the cycle 151 single-track exception. Honored input-from-eva #2937 directive by dispatching adversarial critique as Track 2.

## Cycle 152 ARTIFACTS

- `tools/rust/crates/v2-cycle-runner/tests/integration_cycle.rs` — new (422 LOC, 2 integration tests).
- `docs/redesign/_notes/cycle-152-integration-test-and-critique-dispatch.md` — this file.
- Journal section (appended via Edit; cycle 148-151 sections preserved).
- Track 1 single direct-push commit `335f1199` (the integration test + a state.json delta from dispatch-record).
- Track 2 dispatch issue `#2960` (Copilot adversarial critique, async return).
- `.scratch/` ephemerals: `session-start-2959.md`, `cycle152-track1-commit.txt`, `cycle152-dispatch-body.md`, `cycle152-session-end.md` (authored cycle-close), `cycle152-issue-close.md` (authored cycle-close).
- 0 v2-* crate source-code modifications cycle 152 (Track 1 is test-only addition; Track 2 is dispatch-only).
- 1 dispatch cycle 152 (#2960).
- 4 cargo invocations cycle 152: build 4 primitives + v2-cycle-runner; test --test integration_cycle (initial — caught field-name bugs); test --test integration_cycle (after fix — green); test -p v2-cycle-runner (full 30-test suite); clippy -p v2-cycle-runner --all-targets -D warnings. All clean after the 1 in-cycle iteration on test asserts.
