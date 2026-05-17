//! Integration tests: v2-cycle-runner against real v2-* primitive binaries.
//!
//! Per cycle 149 design scope §7.2:
//!   1. Dry-run test: builds the 4 primitives, runs `v2-cycle-runner run --dry-run`,
//!      asserts 10-step trace + no state mutation.
//!   2. Live test: hand-prepares 4 session-output JSONs with correct payload shapes,
//!      runs `v2-cycle-runner run` (no --dry-run), asserts state files present +
//!      super-step-history has one entry clean-settled + no halt marker.
//!
//! These complement the in-crate unit tests (which use MockInvoker) by exercising
//! the actual primitive-to-primitive coordination across the cycle.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::Once;

use tempfile::TempDir;

const PRIMITIVES: &[&str] = &[
    "v2-channel-router",
    "v2-super-step-boundary",
    "v2-role-driver",
    "v2-reconciler-event-processor",
    // Cycle 162: v2-state-audit is invoked as a pre-flight session-start
    // check by v2-cycle-runner. Empty tempdir state surfaces report Ok
    // (audit tolerates missing paths), so this addition does not change
    // the integration expectations — but the binary MUST be built so
    // v2-cycle-runner can find it.
    "v2-state-audit",
];

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("CARGO_MANIFEST_DIR has parent crates/")
        .parent()
        .expect("crates/ has parent tools/rust/")
        .to_path_buf()
}

fn workspace_target() -> PathBuf {
    if let Ok(t) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(t)
    } else {
        workspace_root().join("target")
    }
}

fn primitive_bin_path(name: &str) -> PathBuf {
    workspace_target().join("debug").join(name)
}

fn cycle_runner_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_v2-cycle-runner"))
}

static BUILD_ONCE: Once = Once::new();

fn build_primitives_once() {
    BUILD_ONCE.call_once(|| {
        let manifest = workspace_root().join("Cargo.toml");
        let mut args: Vec<String> = vec![
            "build".into(),
            "--manifest-path".into(),
            manifest.to_string_lossy().into_owned(),
        ];
        for p in PRIMITIVES {
            args.push("-p".into());
            args.push((*p).into());
        }
        let status = Command::new(std::env::var("CARGO").unwrap_or_else(|_| "cargo".into()))
            .args(&args)
            .status()
            .expect("failed to launch cargo build for primitives");
        assert!(status.success(), "cargo build of primitives failed");

        for p in PRIMITIVES {
            let bin = primitive_bin_path(p);
            assert!(
                bin.exists(),
                "primitive binary not present after build: {}",
                bin.display()
            );
        }
    });
}

fn primitive_bin_args(repo_root: &Path) -> Vec<String> {
    let bin_to_string = |n: &str| primitive_bin_path(n).to_string_lossy().into_owned();
    vec![
        "--repo-root".into(),
        repo_root.to_string_lossy().into_owned(),
        "--channel-router-bin".into(),
        bin_to_string("v2-channel-router"),
        "--super-step-boundary-bin".into(),
        bin_to_string("v2-super-step-boundary"),
        "--role-driver-bin".into(),
        bin_to_string("v2-role-driver"),
        "--reconciler-event-processor-bin".into(),
        bin_to_string("v2-reconciler-event-processor"),
        "--state-audit-bin".into(),
        bin_to_string("v2-state-audit"),
    ]
}

fn run_cycle_runner(args: &[String]) -> Output {
    Command::new(cycle_runner_bin())
        .args(args)
        .output()
        .expect("failed to launch v2-cycle-runner")
}

fn init_state_in(repo_root: &Path) {
    let mut args = primitive_bin_args(repo_root);
    args.push("init".into());
    let output = run_cycle_runner(&args);
    assert!(
        output.status.success(),
        "init failed: stderr={}, stdout={}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );
}

fn collect_files(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    if !root.exists() {
        return Ok(out);
    }
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}

fn write_json(path: &Path, value: &serde_json::Value) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dir");
    }
    fs::write(
        path,
        serde_json::to_string_pretty(value).expect("serialize"),
    )
    .expect("write json");
}

fn read_json(path: &Path) -> serde_json::Value {
    let s =
        fs::read_to_string(path).unwrap_or_else(|e| panic!("read {} failed: {e}", path.display()));
    serde_json::from_str(&s).unwrap_or_else(|e| panic!("parse {} failed: {e}", path.display()))
}

#[test]
fn dry_run_against_real_primitives_traces_ten_steps_without_state_mutation() {
    build_primitives_once();

    let temp = TempDir::new().expect("create temp dir");
    let repo_root = temp.path();

    init_state_in(repo_root);

    let state_dir = repo_root.join("state");
    let before = collect_files(&state_dir).expect("collect before");
    assert!(
        !before.is_empty(),
        "expected init to create some state files, got none"
    );
    let snapshot: Vec<(PathBuf, Vec<u8>)> = before
        .iter()
        .map(|p| (p.clone(), fs::read(p).expect("read pre-snapshot")))
        .collect();

    let mut args = primitive_bin_args(repo_root);
    args.extend([
        "run".into(),
        "--cycle".into(),
        "1".into(),
        "--issue".into(),
        "99999".into(),
        "--dry-run".into(),
    ]);

    let output = run_cycle_runner(&args);
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(
        output.status.success(),
        "dry-run failed: status={:?}, stderr={}, stdout={}",
        output.status,
        stderr,
        stdout
    );

    let step_lines: Vec<&str> = stdout
        .lines()
        .filter(|l| l.trim_start().starts_with("step "))
        .collect();
    assert_eq!(
        step_lines.len(),
        10,
        "expected 10 step trace lines under --dry-run, got {}:\n{}",
        step_lines.len(),
        stdout
    );

    for (path, expected) in &snapshot {
        let actual = fs::read(path).expect("read post-snapshot");
        assert_eq!(
            expected,
            &actual,
            "state file mutated during dry-run: {}",
            path.display()
        );
    }

    let after = collect_files(&state_dir).expect("collect after");
    assert_eq!(
        before.len(),
        after.len(),
        "state file count changed during dry-run: before={} after={}",
        before.len(),
        after.len()
    );
}

#[test]
fn live_run_against_real_primitives_writes_completed_state_with_no_halt_marker() {
    build_primitives_once();

    let temp = TempDir::new().expect("create temp dir");
    let repo_root = temp.path();

    init_state_in(repo_root);

    // Hand-prepare session-output files with payload shapes matching channel-router's
    // required_keys per channel:
    //   inbound-channel  (reconciler) -> eva-responses, audit-posts, dispatch-returns,
    //                                    inbound-completeness-marker
    //   plan-channel     (planner)    -> substantive-focal, per-role-tasks
    //   work-channel     (executor)   -> artifacts-written
    //   memory-channel   (curator)    -> consolidated-insights
    let session_dir = repo_root.join("session-outputs");
    let reconciler_out = session_dir.join("reconciler.json");
    let planner_out = session_dir.join("planner.json");
    let executor_out = session_dir.join("executor.json");
    let curator_out = session_dir.join("curator.json");

    write_json(
        &reconciler_out,
        &serde_json::json!({
            "eva-responses": [],
            "audit-posts": [],
            "dispatch-returns": [],
            "inbound-completeness-marker": "quiet"
        }),
    );
    write_json(
        &planner_out,
        &serde_json::json!({
            "substantive-focal": "integration-test-cycle-1",
            "per-role-tasks": {"executor": "noop"}
        }),
    );
    write_json(&executor_out, &serde_json::json!({"artifacts-written": []}));
    write_json(
        &curator_out,
        &serde_json::json!({"consolidated-insights": "integration test cycle completed"}),
    );

    let mut args = primitive_bin_args(repo_root);
    args.extend([
        "run".into(),
        "--cycle".into(),
        "1".into(),
        "--issue".into(),
        "99999".into(),
        "--reconciler-output-file".into(),
        reconciler_out.to_string_lossy().into_owned(),
        "--planner-output-file".into(),
        planner_out.to_string_lossy().into_owned(),
        "--executor-output-file".into(),
        executor_out.to_string_lossy().into_owned(),
        "--curator-output-file".into(),
        curator_out.to_string_lossy().into_owned(),
    ]);

    let output = run_cycle_runner(&args);
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    assert!(
        output.status.success(),
        "live run failed: status={:?}, stderr={}, stdout={}",
        output.status,
        stderr,
        stdout
    );

    let runner_last = repo_root.join("state/v2-cycle-runner/last-cycle.json");
    assert!(
        runner_last.exists(),
        "expected v2-cycle-runner/last-cycle.json after live run"
    );
    let last = read_json(&runner_last);
    assert_eq!(
        last.get("status").and_then(|v| v.as_str()),
        Some("completed"),
        "expected status=completed in last-cycle, got {:?}\nfull: {}",
        last.get("status"),
        last
    );
    assert_eq!(
        last.get("cycle").and_then(|v| v.as_u64()),
        Some(1),
        "expected cycle=1 in last-cycle, got {:?}",
        last.get("cycle")
    );
    assert_eq!(
        last.get("issue").and_then(|v| v.as_u64()),
        Some(99999),
        "expected issue=99999 in last-cycle, got {:?}",
        last.get("issue")
    );
    let halt_step = last.get("halt_step");
    assert!(
        halt_step.map(|v| v.is_null()).unwrap_or(true),
        "expected halt_step absent or null on clean exit, got {:?}",
        halt_step
    );
    let halt_reason = last.get("halt_reason");
    assert!(
        halt_reason.map(|v| v.is_null()).unwrap_or(true),
        "expected halt_reason absent or null on clean exit, got {:?}",
        halt_reason
    );
    let halted_after_role = last.get("halted_after_role");
    assert!(
        halted_after_role.map(|v| v.is_null()).unwrap_or(true),
        "expected halted_after_role absent or null on clean exit, got {:?}",
        halted_after_role
    );
    let steps_attempted = last.get("steps_attempted").and_then(|v| v.as_u64());
    assert_eq!(
        steps_attempted,
        Some(10),
        "expected steps_attempted=10 on clean exit, got {:?}",
        steps_attempted
    );

    let runner_hist = repo_root.join("state/v2-cycle-runner/cycle-history.json");
    assert!(
        runner_hist.exists(),
        "expected v2-cycle-runner/cycle-history.json after live run"
    );
    let hist = read_json(&runner_hist);
    let entries = hist
        .get("cycles")
        .and_then(|v| v.as_array())
        .expect("cycle-history.cycles is an array");
    assert_eq!(
        entries.len(),
        1,
        "expected 1 entry in cycle-history.cycles, got {}: {}",
        entries.len(),
        hist
    );
    assert_eq!(
        entries[0].get("status").and_then(|v| v.as_str()),
        Some("completed"),
        "expected first history entry status=completed, got {:?}",
        entries[0].get("status")
    );

    let super_step_hist = repo_root.join("state/super-step-history.json");
    assert!(
        super_step_hist.exists(),
        "expected super-step-history.json after live run"
    );
    let ss_hist = read_json(&super_step_hist);
    let ss_entries = ss_hist
        .get("cycles")
        .and_then(|v| v.as_array())
        .expect("super-step-history.cycles is an array");
    assert_eq!(
        ss_entries.len(),
        1,
        "expected 1 entry in super-step-history.cycles, got {}: {}",
        ss_entries.len(),
        ss_hist
    );
    assert_eq!(
        ss_entries[0].get("cycle").and_then(|v| v.as_u64()),
        Some(1),
        "expected super-step-history first entry cycle=1"
    );

    // Verify all 4 channel-state files are present and well-formed (single entry per
    // channel since this is the first cycle).
    for channel in &[
        "inbound-channel",
        "plan-channel",
        "work-channel",
        "memory-channel",
    ] {
        let state_file = repo_root
            .join("state/channels")
            .join(format!("{channel}.json"));
        assert!(
            state_file.exists(),
            "expected channel state file present: {}",
            state_file.display()
        );
        let state = read_json(&state_file);
        assert_eq!(
            state.get("cycle").and_then(|v| v.as_u64()),
            Some(1),
            "expected channel {} state cycle=1, got {:?}",
            channel,
            state.get("cycle")
        );
    }
}

/// Cycle 162: pre-flight state-audit on session-start halts the cycle when
/// the live state surface breaches a `Hard` threshold. This test populates
/// `docs/state.json` with 600 synthetic agent_sessions entries (above
/// the cycle 158 policy DISPATCHES_HARD = 500), runs the runner, and
/// asserts:
///   - non-zero exit
///   - last-cycle.json records halt_reason=state-bound-exceeded
///     and halt_step=state-audit-on-start
///   - no super-step state mutation occurs (no super-step-history entries)
#[test]
fn live_run_halts_with_state_bound_exceeded_when_audit_reports_hard() {
    build_primitives_once();

    let temp = TempDir::new().expect("create temp dir");
    let repo_root = temp.path();

    init_state_in(repo_root);

    // Populate docs/state.json with enough agent_sessions entries to
    // breach DISPATCHES_HARD (500). Each entry is a minimal placeholder
    // — only count matters for the audit's array-length probe.
    let docs_dir = repo_root.join("docs");
    fs::create_dir_all(&docs_dir).expect("create docs/ dir");
    let mut entries: Vec<serde_json::Value> = Vec::with_capacity(600);
    for i in 0..600 {
        entries.push(serde_json::json!({
            "issue_number": 100_000 + i,
            "status": "merged",
        }));
    }
    let state_body = serde_json::json!({ "agent_sessions": entries });
    fs::write(
        docs_dir.join("state.json"),
        serde_json::to_vec(&state_body).expect("serialize state.json"),
    )
    .expect("write state.json");

    // Prepare session-output files anyway (validate_session_output_files
    // runs before the audit pre-flight). Their contents don't matter
    // here — execution halts at the audit step before any role-driver
    // invocation.
    let session_dir = repo_root.join("session-outputs");
    let reconciler_out = session_dir.join("reconciler.json");
    let planner_out = session_dir.join("planner.json");
    let executor_out = session_dir.join("executor.json");
    let curator_out = session_dir.join("curator.json");
    for p in [&reconciler_out, &planner_out, &executor_out, &curator_out] {
        write_json(p, &serde_json::json!({}));
    }

    let mut args = primitive_bin_args(repo_root);
    args.extend([
        "run".into(),
        "--cycle".into(),
        "1".into(),
        "--issue".into(),
        "99999".into(),
        "--reconciler-output-file".into(),
        reconciler_out.to_string_lossy().into_owned(),
        "--planner-output-file".into(),
        planner_out.to_string_lossy().into_owned(),
        "--executor-output-file".into(),
        executor_out.to_string_lossy().into_owned(),
        "--curator-output-file".into(),
        curator_out.to_string_lossy().into_owned(),
    ]);

    let output = run_cycle_runner(&args);
    assert!(
        !output.status.success(),
        "expected non-zero exit when audit halts at state-bound-exceeded; \
         stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let runner_last = repo_root.join("state/v2-cycle-runner/last-cycle.json");
    assert!(
        runner_last.exists(),
        "expected v2-cycle-runner/last-cycle.json after halt-at-audit"
    );
    let last = read_json(&runner_last);
    assert_eq!(
        last.get("status").and_then(|v| v.as_str()),
        Some("halted"),
        "expected status=halted, got {:?}",
        last.get("status"),
    );
    assert_eq!(
        last.get("halt_step").and_then(|v| v.as_str()),
        Some("state-audit-on-start"),
        "expected halt_step=state-audit-on-start, got {:?}",
        last.get("halt_step"),
    );
    assert_eq!(
        last.get("halt_reason").and_then(|v| v.as_str()),
        Some("state-bound-exceeded"),
        "expected halt_reason=state-bound-exceeded, got {:?}",
        last.get("halt_reason"),
    );
    assert_eq!(
        last.get("steps_attempted").and_then(|v| v.as_u64()),
        Some(0),
        "expected steps_attempted=0 when audit halts pre-flight, got {:?}",
        last.get("steps_attempted"),
    );

    // No super-step state mutation: super-step-history must be empty (the
    // post-init shape is `{ "cycles": [] }`).
    let super_step_hist = repo_root.join("state/super-step-history.json");
    if super_step_hist.exists() {
        let hist = read_json(&super_step_hist);
        let entries = hist
            .get("cycles")
            .and_then(|v| v.as_array())
            .expect("super-step-history.cycles is an array");
        assert_eq!(
            entries.len(),
            0,
            "expected no super-step-history entries after audit halt, got {}",
            entries.len(),
        );
    }
}

/// Cycle 163 (Missing Integration Scenario 3, unblocked by cycle 160 C10
/// amendment): super-step out-of-order halt. Pre-populates
/// `state/super-step-history.json` with a synthetic cycle-5 completion
/// record, leaving `state/super-step.json` as the post-init empty sentinel.
/// Runs `v2-cycle-runner run --cycle 1`; v2-super-step-boundary's
/// cycle-start step refuses with `OutOfOrderCycleStart` (cycles must be
/// sequential: last completed 5, requested 1, expected 6). Cycle 163's
/// boundary message-prefix change makes this stderr classify as
/// `FailureClass::SuperStepOutOfOrder` in v2-cycle-runner. Asserts the
/// dual cycle 160 C10 invariants:
///
///   1. **State-mutation-PRESENT for runner-local observability state**:
///      `last-cycle.json` exists with `status=out-of-order`,
///      `halt_step=super-step-init`, `halt_class=super-step-out-of-order`,
///      `steps_attempted=1`.
///   2. **State-mutation-ABSENT for super-step state machine state**:
///      `super-step-history.json` is byte-identical to its pre-run
///      state (still only the synthetic cycle-5 entry);
///      `super-step.json` is byte-identical (still the empty sentinel).
#[test]
fn live_run_halts_super_step_out_of_order_when_history_diverges() {
    build_primitives_once();

    let temp = TempDir::new().expect("create temp dir");
    let repo_root = temp.path();

    init_state_in(repo_root);

    // Pre-populate super-step-history.json with a synthetic completed
    // cycle-5 entry. This makes any subsequent cycle-start at cycle != 6
    // trigger OutOfOrderCycleStart.
    let history_path = repo_root.join("state/super-step-history.json");
    let synthetic_history = serde_json::json!({
        "cycles": [
            {
                "cycle": 5,
                "started_at": "1970-01-01T00:00:00Z",
                "ended_at": "1970-01-01T00:00:00Z",
                "transitions": []
            }
        ]
    });
    write_json(&history_path, &synthetic_history);

    // Snapshot the super-step state files for state-mutation-absent
    // assertion (cycle 160 C10 amendment).
    let super_step_path = repo_root.join("state/super-step.json");
    let history_before = fs::read(&history_path).expect("read history pre-run");
    let super_step_before = fs::read(&super_step_path).expect("read super-step pre-run");

    // Prepare session-output files (validate_session_output_files runs
    // before the cycle-start step; contents don't matter because execution
    // halts at step 1).
    let session_dir = repo_root.join("session-outputs");
    let reconciler_out = session_dir.join("reconciler.json");
    let planner_out = session_dir.join("planner.json");
    let executor_out = session_dir.join("executor.json");
    let curator_out = session_dir.join("curator.json");
    for p in [&reconciler_out, &planner_out, &executor_out, &curator_out] {
        write_json(p, &serde_json::json!({}));
    }

    let mut args = primitive_bin_args(repo_root);
    args.extend([
        "run".into(),
        "--cycle".into(),
        "1".into(),
        "--issue".into(),
        "99999".into(),
        "--reconciler-output-file".into(),
        reconciler_out.to_string_lossy().into_owned(),
        "--planner-output-file".into(),
        planner_out.to_string_lossy().into_owned(),
        "--executor-output-file".into(),
        executor_out.to_string_lossy().into_owned(),
        "--curator-output-file".into(),
        curator_out.to_string_lossy().into_owned(),
    ]);

    let output = run_cycle_runner(&args);
    assert!(
        !output.status.success(),
        "expected non-zero exit when boundary returns out-of-order; \
         stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    // C10 invariant #1: runner-local observability state IS written.
    let runner_last = repo_root.join("state/v2-cycle-runner/last-cycle.json");
    assert!(
        runner_last.exists(),
        "expected v2-cycle-runner/last-cycle.json after super-step out-of-order halt"
    );
    let last = read_json(&runner_last);
    assert_eq!(
        last.get("status").and_then(|v| v.as_str()),
        Some("out-of-order"),
        "expected status=out-of-order, got {:?}\nfull last-cycle: {}",
        last.get("status"),
        last
    );
    assert_eq!(
        last.get("halt_step").and_then(|v| v.as_str()),
        Some("super-step-init"),
        "expected halt_step=super-step-init, got {:?}",
        last.get("halt_step"),
    );
    assert_eq!(
        last.get("halt_reason").and_then(|v| v.as_str()),
        Some("super-step-out-of-order"),
        "expected halt_reason=super-step-out-of-order, got {:?}",
        last.get("halt_reason"),
    );
    assert_eq!(
        last.get("steps_attempted").and_then(|v| v.as_u64()),
        Some(1),
        "expected steps_attempted=1 (cycle-start was the step that failed), got {:?}",
        last.get("steps_attempted"),
    );

    // C10 invariant #2: super-step state machine state is NOT mutated.
    let history_after = fs::read(&history_path).expect("read history post-run");
    assert_eq!(
        history_before, history_after,
        "expected super-step-history.json to be byte-identical after out-of-order halt; \
         pre-run had synthetic cycle-5 only, post-run differs — boundary mutated \
         super-step history despite refusing the cycle-start"
    );
    let super_step_after = fs::read(&super_step_path).expect("read super-step post-run");
    assert_eq!(
        super_step_before, super_step_after,
        "expected super-step.json to be byte-identical after out-of-order halt; \
         boundary mutated current super-step state despite refusing the cycle-start"
    );
}
