# Cycle 151 _notes — v2-cycle-runner `run` subcommand implementation

**Cycle:** 151 (single-track substantive focal)
**Date:** 2026-05-15
**Cycle issue:** [#2958](https://github.com/EvaLok/schema-org-json-ld/issues/2958)
**Commit:** `6e13a3a7`
**Predecessor:** cycle 150 implementation entry (commits `1a9becde` skeleton + `8ccefb0f` executor 4-fix)
**Reference design scope:** [`cycle-149-v2-cycle-runner-design-scope.md`](cycle-149-v2-cycle-runner-design-scope.md) §3, §9, §11

## Single-track shape — design-scope authoritativeness

Cycle 149 design scope §11 named cycle 151 as **"Run subcommand implementation + first unit tests + dry-run subcommand wiring. Single direct-push commit."** Cycle 150 followed two-track-composition pattern HARDENING-AT-5 (5 consecutive two-track cycles 146-150).

Cycle 151 honors the scope's explicit shape — single-track, single commit. This is a substantive-vs-pattern resolution: when prior-cycle design scope predetermines cycle shape, scope adherence supersedes inherited cycle-composition momentum.

New pattern observation: **`design-scope-supersedes-inherited-cycle-shape-momentum`** NOVEL@1 cycle 151. Watch for recurrence in future scope-predetermined cycles.

## What got built

Single file modified: `tools/rust/crates/v2-cycle-runner/src/main.rs`.
+1029 LOC / −4 LOC.

### Production code (~530 LOC addition)

1. **`Run` subcommand** added to `Subcmd` enum with 11 args:
   - Required: `--cycle <N>`, `--issue <M>`
   - Mode: `--dry-run` (bool flag)
   - Debug: `--halt-after-role <role>` (optional Role enum)
   - Source files (optional): `--eva-source-file`, `--audit-source-file`, `--dispatch-source-file`
   - Session-output files (required unless --dry-run): `--reconciler-output-file`, `--planner-output-file`, `--executor-output-file`, `--curator-output-file`

2. **Local `Role` enum** with kebab-case clap + serde rename matching v2-role-driver. `as_kebab()` helper for clean arg-string building.

3. **`super_step_sequence() -> [Step; 10]`** — declarative 10-step sequence as data, with index + name + StepKind. StepKind variants: `SuperStepCycleStart`, `ReconcilerPoll`, `RoleInvoke(Role)`, `SuperStepAdvance`, `SuperStepCycleEnd`.

4. **`FailureClass` enum** per cycle 149 design scope §3.3:
   - `Transient` → retry-once
   - `RoleSessionEmpty` → halt
   - `ChannelWriteRejected` → halt
   - `SuperStepOutOfOrder` → hard error (cycle abort)

5. **`classify_failure(exit_code, stderr) -> FailureClass`** — pattern-matches lower-cased stderr for distinctive keywords. Order of checks matters: out-of-order → rejected → empty → default-transient.

6. **`PrimitiveInvoker` trait** with method `invoke(&self, bin: &Path, args: &[String]) -> io::Result<Output>`. Production impl `RealInvoker` (ProcessCommand). Tests use `MockInvoker` (canned-output queue + invocation-recording via RefCell). Trait-based abstraction enables unit-level run-loop coverage without exec.

7. **`iso8601_now()` + `format_unix_secs_iso(unix_secs)`** built on `std::time::SystemTime` + Howard Hinnant civil-from-days algorithm (public domain). Avoids `chrono` dependency. Three tests verify against epoch, Y2K, leap-day 2024-02-29.

8. **`run_cycle()`** main entrypoint:
   - Validates session-output files for live mode
   - Iterates `super_step_sequence()`
   - For each step: builds invocation via `build_step_invocation()`, optionally dry-run-traces, otherwise invokes via `invoke_with_retry_once()`
   - Handles `--halt-after-role` short-circuit (after a role-session, before its advance)
   - On failure: classifies, writes halt marker, returns appropriate `RunnerError`
   - On success: writes completed state + appends history

9. **`write_runner_state()`** — single-writer-per-state-file discipline per cycle 149 design scope §5. Writes `state/v2-cycle-runner/last-cycle.json` (overwrite-current) + appends entry to `state/v2-cycle-runner/cycle-history.json` (preserves prior entries).

10. **`RunnerError` extensions:** added `MissingSessionOutput { role }`, `CycleHalted { step, class, stderr }`, `SuperStepOutOfOrder { step, stderr }` variants with structured Display impls.

### Tests added (~495 LOC, 17 new tests, 28 total in crate)

Categories listed in commit message. Notable test coverage:

- **`super_step_sequence_has_ten_steps_in_exact_order`** — declarative sequence is a data structure, so this is a pure-data assertion. Tests step.index + step.name simultaneously.
- **`super_step_sequence_role_invokes_are_in_reconciler_planner_executor_curator_order`** — extracts roles from StepKind::RoleInvoke variants; asserts ordering matches contract-check writer-map.
- **`classify_failure_*`** — four tests covering the 4 FailureClass variants with realistic stderr samples.
- **`format_unix_secs_iso_*`** — three datums: epoch zero, Y2K, leap-day 2024-02-29 (Hinnant algorithm correctness check).
- **`build_step_invocation_*`** — four tests covering distinct StepKind variants and verifying flag/arg presence (timestamp / cycle / role / session-output-file / source-file forwarding / advance not taking cycle).
- **`validate_session_output_files_*`** — boundary tests for the per-role validation.
- **`dry_run_does_not_invoke_primitives_or_mutate_state`** — invoke_count==0 + state dir not created.
- **`live_run_invokes_ten_steps_and_writes_state_on_success`** — full happy path; verifies state shape + history entry shape.
- **`live_run_halts_when_role_session_returns_empty_output`** — halt-class plumbing all the way to halt-state JSON.
- **`live_run_hard_errors_on_super_step_out_of_order`** — verifies SuperStepOutOfOrder returns a distinct error variant + state shows status "out-of-order".
- **`live_run_retries_transient_failure_once`** — counts invoke_count == 11 (10 logical + 1 retry).
- **`live_run_halt_after_role_short_circuits_before_advance`** — halt-after-planner stops at step 5 (planner-session), not step 6 (advance-2).
- **`live_run_missing_session_output_file_errors_before_invoking`** — fails fast, zero primitive invocations.
- **`mock_invoker_records_args_in_order`** — verifies MockInvoker recording invariant for test-tool robustness.

## In-cycle issues + recoveries

1. **First-attempt cargo build clean.** No surprises after substantial edit.
2. **One test failure first run:** `format_unix_secs_iso_known_date` — my hand-computed unix timestamp for 2026-05-15T08:58:33Z was wrong by 338,400 seconds (~3.9 days). Replaced with two more-robust test values: Y2K (946684800) and leap-day-2024 (1709210096), both independently computable and stable across time. **Lesson:** when testing date-formatting functions, prefer well-known landmark timestamps (epoch, Y2K, named leap days) over recent-event-derived values that require ad-hoc arithmetic.
3. **No clippy warnings first run** after substantial code addition. The trait-based abstraction did not introduce any clippy noise.
4. **CWD-drift lesson cycle 149 re-validated:** all cargo invocations used `--manifest-path tools/rust/Cargo.toml` from repo root. Zero `cd tools/rust` chains. 3-cycle running validation (149 → 150 → 151).
5. **`v2-prompt-contract-check` re-verified clean 4/4** post-edit (cycle 133 clarification: cargo work on Rust-PR cycles is legitimate; this is post-edit contract-check verification, not gratuitous cargo).
6. **One smoke-run argument noise:** running v2-prompt-contract-check with default binary path failed because cargo target dir is `tools/rust/target/debug/...` not `./target/debug/...` from repo root. Passed `--channel-router-bin <absolute>` explicitly. Not a code issue — a documentation gap. Future runner-deployment doc should note workspace-target-dir vs invocation-cwd discrepancy.

## Substantive measurements (cycle 151)

### LOC

| Component | Cycle 150 | Cycle 151 delta | Cycle 151 post |
|---|---|---|---|
| `Cargo.toml` | 13 | 0 | 13 |
| `src/main.rs` prod | 310 | +530 | ~840 |
| `src/main.rs` tests | 73 | +495 | ~568 |
| `src/main.rs` total | 383 | +1029 (1025 net after -4) | 1408 |
| **Crate total** | 396 | +1029 | 1421 |

### Runner-shape LOC family — first datapoint

Cycle 150 declined to predict a band. Cycle 151 establishes first datapoint:
- **Minimal-3-subcommand point (init+schema+run): 1421 LOC.**
- Cycle 149 design scope §11 sketched cycles 150-153+ implementation arc; if `status` + `verify` each add ~150-300 LOC, family ceiling estimate: **~1700-2200 LOC at complete-arc (5-subcommand) point.**
- Future status+verify cycles can refine this band; recurrence test for `magnitude-prediction-precision-is-shape-dependent-not-flat`.

### Test count

- Cycle 150: 5 unit tests
- Cycle 151 addition: +23 new tests
- Cycle 151 total: 28 tests in v2-cycle-runner crate

### Dry-run smoke

10/10 steps traced with correct primitive + correct args. Timestamp formatted consistently across all 10 steps (single timestamp per cycle, captured at run start). Zero state-dir mutation under `--dry-run` (verified `ls .scratch/cycle151-smoke` empty post-run).

## Pattern updates

- **`design-scope-supersedes-inherited-cycle-shape-momentum`** NOVEL@1 cycle 151. Honored single-track despite 5 consecutive two-track cycles (HARDENING-AT-5). Lesson: when prior-cycle design scope predetermines cycle composition, that scope is authoritative over pattern-maintenance pressure.
- **`design-scope-vs-implementation-naming-divergence`** HARDENING-AT-1 cycle 151 (cycle 150 NOVEL@1 → cycle 151 in-band recurrence). Implementation uses cycle-start/advance/cycle-end (NOT design scope's begin/advance/settle). Cycle 150 lesson "when prose and implementation diverge, implementation wins" load-bears cycle 151.
- **`proactive-document-before-implementation`** HARDENING-AT-2 cycle 151. 3-cycle arc (149 design-scope → 150 entry → 151 main implementation). Design-scope §11 explicit cycle-by-cycle forward agenda load-bore cycle 151's shape AND scope of work.
- **`magnitude-prediction-precision-is-shape-dependent-not-flat`** band-establishment cycle for runner-shape family at minimal-3-subcommand point: 1421 LOC.
- **`testable-via-trait-abstraction-trumps-integration-test-scaffolding`** NOVEL@1 cycle 151. The `PrimitiveInvoker` trait + `MockInvoker` pattern enabled unit-level coverage of the run loop (including failure paths) without external exec. Cycle 149 design scope §7.2 lists integration-test against real primitives as cycle 152 work; the trait-abstraction approach makes that integration test additive evidence, not the only way to verify run-loop correctness. Pattern worth recurring when test scaffolding cost outweighs trait-abstraction cost.
- **`single-timestamp-per-cycle`** NOVEL@1 cycle 151. The runner generates one ISO-8601 timestamp at run-start and passes it to all 10 primitive invocations. This makes super-step transitions semantically aligned (all 10 steps stamped as "happening during cycle N") rather than fine-grained per-step wallclock measurements. If wallclock-per-step is needed later (e.g., for measurement collection at cycle ~153), a refinement is to capture per-step start/end timestamps internally while passing one cycle-timestamp to primitives. Defer to first measurement evidence.

## Forward priorities for cycle 152+

The cycle 149 design scope §11 forward arc:
- **Cycle 152:** Integration test against real primitives (built-by-test-setup, hermetic). First end-to-end dry-run against synthetic state. Per design scope §7.2.
- **Cycle 153:** First non-dry-run end-to-end smoke against temp state. **First measurement opportunity for v2 architecture** per design scope §11 + cycle 148 forward priority #2.

Concrete forward priorities (re-ordering cycle 150's 10-item list):

1. **Cycle 152 substantive focal: integration test against real primitives** (cycle 149 §7.2). Build 4 primitives in test-setup, exercise `run --dry-run` end-to-end, then `run` without dry-run against temp state with hand-prepared session-output files (4 trivial-payload JSONs). Verify all state files present + well-formed; halt markers absent on clean exit.
2. **Cycle 153 substantive focal: first end-to-end smoke + first measurement.** Profile per-role token usage (when live spawn lands), wall-clock per step (already capturable now via per-step trace timestamps if added), state-file size delta per step, halt-rate, channel-write-reject-rate, side-channel-leak-rate.
3. **AGREE-ACT-NOW L1.3 + L3.6 reconciler completeness-metadata pairing** (prompt edit + router required-key extension) — bounded; queue for cycle 152+ pairing with substantive focal.
4. **AGREE-ACT-NOW L2.4 dispatch-brief discipline addendum** — document-only; bounded.
5. **AGREE-ACT-NOW X2 honest cycle-1-scope-redefinition** — document-only.
6. **AGREE-RECORD L1.2 + X4 side-channel architecture-notes doc** — document-only.
7. **TOOL-SCOPED v2-channel-router enforcement extension design scope** (C1 / L3.1) — design scope first.
8. **TOOL-SCOPED v2-prompt-tag-semantic-fidelity tool design scope** (L2.5) — design scope first.
9. **`status` + `verify` v2-cycle-runner subcommands** — cycle 154+ if needed; not blocking for first-measurement milestone.
10. **AGREE-DEFER queue** (post-first-measurement) — curator complexity rework, template-mirror architecture-revisit. Hold for cycle 153+ runtime evidence.
11. **Cycle 120 L2 preserved.**

## Audit cycle 220 acknowledgment

Audit HEAD advanced from `72cda153` (cycle 148 blocker) to `333a745d` (audit cycle 220, landed 2026-05-15 04:38 UTC, prior to cycle 150 start). Audit cycle 220 content spans cycles 134-148 of main and is largely observational confirmation of Phase 3 progress: Q7 RESOLVED B selected → multi-agent role-specialization Phase 3 substrate → 4 primitive SCAFFOLDs landed cycles 140-143 → 4 role-prompts complete cycles 144-145 → adversarial-critique pattern exercised cycles 146-148 → PR #2951 24-finding ledger absorbed cycle 148 → PR #2953 v2-prompt-contract-check landed.

Distinctive audit observations:
- **`audit-as-Q7-resolution-input-channel`** HARDENED@5 (audit chain A1-A6 retrospective → cycle 217 cluster framework critique → cycle 218 #465 M1 sharpening converged on substrate-replacement as Eva's correct architectural response).
- **`substrate-replacement-shifts-A4-framing`** NOVEL meta-observation: A4 silent-fail rate 8/17 ~47% under single-orchestrator substrate; Phase 3 multi-agent topology may amplify/mitigate/transform; cycle 221+ to observe.
- **`Copilot-as-adversarial-critique-parallel-pattern`** emergent observation: main exercised cycle 146-148 via PR #2951; parallels audit's V2 cross-repo audit-engagement format at intra-repo scope.

**No critique demanding main change.** Acknowledgment-only — content informs cycle 151+ context without requiring artifact updates.

## What cycle 151 does NOT do

- Does NOT implement `status` / `verify` subcommands (cycle 152+ if needed).
- Does NOT modify cycle-runner (forbidden zone preserved during Phase 3).
- Does NOT modify `.github/workflows/` or this orchestrator prompt.
- Does NOT spawn live Claude sessions (v2-role-driver remains SCAFFOLD; live spawn deferred per cycle 149 design scope OQ2).
- Does NOT add integration test against real primitives (cycle 152 per design scope §11).
- Does NOT enact 5 remaining AGREE-ACT-NOW findings (L1.3+L3.6, L2.4, X2, L1.2+X4 architecture-notes).
- Does NOT escalate any decision to Eva (EVA-DEFAULT-AUTONOMY).
- Does NOT modify `docs/redesign/2-selection.md` (cycle 120 L2 preserved).
- Does NOT capture wallclock-per-step or token usage measurements (cycle 153+).
- Does NOT add a separate `bin/` build, install script, or workflow YAML edit (cycle 4 cutover scope).

## Process honoring (cycle 151)

- **36th consecutive cycle of HONORING named forward priority** (cycles 115-151).
- **64th bottleneck-asynchronous cycle** (78-151).
- **41st non-per-candidate-sharpening cycle** (111-151).
- Cycle 120 L2 preserved (2-selection.md untouched).
- Cycle 128 lesson preserved (.scratch/ via Write tool for session-start + commit-message + cycle-close ephemerals).
- Cycle 133 clarification preserved: cargo invocations cycle 151 are legitimate (Rust-PR work — build / test / clippy + post-edit contract-check verification + dry-run smoke).
- Cycle 134 lesson preserved (audit-repo HEAD via gh api).
- **Cycle 137 lessons re-validated cycle 151** — one for-loop attempt at session-start blocked ("Contains simple_expansion"); recovered via individual per-primitive grep invocations. 6-cycle running validation (137 + 147 + 148 + 149 + 150 + 151).
- **Cycle 149 in-cycle CWD-drift lesson re-validated cycle 151** — zero `cd tools/rust` chains; all cargo work via `--manifest-path tools/rust/Cargo.toml`. 3-cycle running validation.
- **NEW cycle 151 in-cycle lesson:** date-formatting tests should use landmark timestamps (epoch, Y2K, named leap-days), not recent-event-derived values requiring ad-hoc arithmetic. The 338,400-second error was a textbook example of why test data should be computationally verifiable, not just plausible.
- Journal-immutability discipline preserved (cycle 151 appends NEW section via Edit anchor at end of cycle 150 section; cycle 148/149/150 sections NOT back-edited).
- Anti-overstatement audit explicit ("What cycle 151 does NOT do" enumeration above).
- **Single-track substantive focal** honors design-scope authoritativeness even at the cost of breaking two-track-composition HARDENING-AT-5 momentum. The pattern observation `design-scope-supersedes-inherited-cycle-shape-momentum` captures the principle.

## Cycle 151 ARTIFACTS

- `tools/rust/crates/v2-cycle-runner/src/main.rs` — modified (+1029 / -4; 383 → 1408 LOC).
- Single direct-push commit `6e13a3a7`.
- This _notes file.
- Journal section (appended via Edit; cycle 148/149/150 sections preserved).
- `.scratch/` ephemerals: `session-start-2958.md`, `cycle151-commit.txt`, `cycle151-smoke/` (empty post-dry-run), `cycle151-session-end.md`, `cycle151-issue-close.md` (latter two cycle-close).
- 1 commit + 1 push cycle 151 (so far) — cycle-close adds 1 commit + 1 push for journal + _notes + ephemerals.
- 0 v2-* crate modifications beyond v2-cycle-runner (single substantive focal honored).
- 0 dispatches cycle 151 (no Copilot agent-task dispatches needed).
- 4 cargo invocations cycle 151: build v2-cycle-runner; test v2-cycle-runner; clippy v2-cycle-runner; run v2-cycle-runner with --dry-run smoke. All clean after the 1 in-session iteration (timestamp test value fix). Plus contract-check re-verification with explicit binary path (1 additional cargo invocation; clean).
