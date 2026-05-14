use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

const BIN: &str = env!("CARGO_BIN_EXE_v2-super-step-boundary");

fn run_cmd(repo: &Path, args: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(BIN);
    cmd.arg("--repo-root").arg(repo);
    for a in args {
        cmd.arg(a);
    }
    cmd.output().expect("spawn v2-super-step-boundary")
}

fn run_cmd_json(repo: &Path, args: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(BIN);
    cmd.arg("--repo-root").arg(repo);
    cmd.arg("--format").arg("json");
    for a in args {
        cmd.arg(a);
    }
    cmd.output().expect("spawn v2-super-step-boundary")
}

fn fresh_repo() -> (TempDir, PathBuf) {
    let td = TempDir::new().expect("tempdir");
    let path = td.path().to_path_buf();
    (td, path)
}

/// Write a fake channel-router state file at state/channels/<channel>.json
/// matching the writer + cycle that super-step-boundary will verify against.
fn write_channel_state(repo: &Path, channel: &str, writer: &str, cycle: u32) {
    let dir = repo.join("state").join("channels");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("{channel}.json"));
    let body = serde_json::json!({
        "channel": channel,
        "writer": writer,
        "cycle": cycle,
        "timestamp": "2026-05-14T00:00:00Z",
        "payload": { "stub": true },
    });
    std::fs::write(path, serde_json::to_string_pretty(&body).unwrap()).unwrap();
}

fn stdout_str(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn stderr_str(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stderr).to_string()
}

// ----- init -----

#[test]
fn init_creates_state_and_history_files() {
    let (_td, repo) = fresh_repo();
    let out = run_cmd(&repo, &["init"]);
    assert!(out.status.success(), "init failed: {}", stderr_str(&out));
    let state_path = repo.join("state").join("super-step.json");
    let history_path = repo.join("state").join("super-step-history.json");
    assert!(state_path.exists(), "state file not created");
    assert!(history_path.exists(), "history file not created");
}

#[test]
fn init_is_idempotent() {
    let (_td, repo) = fresh_repo();
    let out1 = run_cmd(&repo, &["init"]);
    assert!(out1.status.success());
    let out2 = run_cmd(&repo, &["init"]);
    assert!(out2.status.success(), "second init failed: {}", stderr_str(&out2));
    let stdout = stdout_str(&out2);
    assert!(
        stdout.contains("present"),
        "expected 'present' on second init, got: {stdout}"
    );
}

#[test]
fn init_json_reports_creation_status() {
    let (_td, repo) = fresh_repo();
    let out = run_cmd_json(&repo, &["init"]);
    assert!(out.status.success());
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["state_file_created"], serde_json::Value::Bool(true));
    assert_eq!(value["history_file_created"], serde_json::Value::Bool(true));
    let out2 = run_cmd_json(&repo, &["init"]);
    let value2: serde_json::Value = serde_json::from_slice(&out2.stdout).unwrap();
    assert_eq!(value2["state_file_created"], serde_json::Value::Bool(false));
    assert_eq!(value2["history_file_created"], serde_json::Value::Bool(false));
}

// ----- current (no cycle in progress) -----

#[test]
fn current_before_init_reports_not_initialized() {
    let (_td, repo) = fresh_repo();
    let out = run_cmd(&repo, &["current"]);
    assert!(!out.status.success(), "current must fail before init");
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("not initialized"),
        "expected 'not initialized' error, got: {stderr}"
    );
}

#[test]
fn current_after_init_reports_no_cycle_in_progress() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    let out = run_cmd(&repo, &["current"]);
    assert!(out.status.success());
    let stdout = stdout_str(&out);
    assert!(
        stdout.contains("no cycle in progress"),
        "expected 'no cycle in progress', got: {stdout}"
    );
}

#[test]
fn current_json_after_init_is_null() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    let out = run_cmd_json(&repo, &["current"]);
    assert!(out.status.success());
    let stdout = stdout_str(&out);
    assert_eq!(stdout.trim(), "null");
}

// ----- cycle-start -----

#[test]
fn cycle_start_happy_path_enters_reconciler() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    let out = run_cmd(
        &repo,
        &["cycle-start", "--cycle", "141", "--timestamp", "2026-05-14T00:00:00Z"],
    );
    assert!(out.status.success(), "cycle-start failed: {}", stderr_str(&out));
    let cur = run_cmd(&repo, &["current"]);
    let stdout = stdout_str(&cur);
    assert!(
        stdout.contains("cycle 141 super-step 'reconciler'"),
        "current after cycle-start: {stdout}"
    );
}

#[test]
fn cycle_start_idempotent_on_same_cycle_first_super_step() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_cmd(
        &repo,
        &["cycle-start", "--cycle", "141", "--timestamp", "2026-05-14T00:00:00Z"],
    );
    let out = run_cmd(
        &repo,
        &["cycle-start", "--cycle", "141", "--timestamp", "2026-05-14T00:01:00Z"],
    );
    assert!(
        out.status.success(),
        "idempotent cycle-start failed: {}",
        stderr_str(&out)
    );
    let stdout = stdout_str(&out);
    assert!(
        stdout.contains("idempotent"),
        "expected idempotent marker, got: {stdout}"
    );
}

#[test]
fn cycle_start_different_cycle_in_progress_fails() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_cmd(&repo, &["cycle-start", "--cycle", "141"]);
    let out = run_cmd(&repo, &["cycle-start", "--cycle", "142"]);
    assert!(!out.status.success());
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("already in progress"),
        "expected already-in-progress error, got: {stderr}"
    );
}

#[test]
fn cycle_start_first_cycle_can_be_arbitrary_number() {
    // With no history, the very first cycle can start at any N.
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    let out = run_cmd(&repo, &["cycle-start", "--cycle", "1000"]);
    assert!(out.status.success(), "first cycle at 1000 should succeed: {}", stderr_str(&out));
}

#[test]
fn cycle_start_after_completed_cycle_must_be_sequential() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    // cycle 141 full happy path
    run_full_happy_cycle(&repo, 141);
    // Try to skip ahead to cycle 143
    let out = run_cmd(&repo, &["cycle-start", "--cycle", "143"]);
    assert!(!out.status.success(), "skip-ahead must fail");
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("must be sequential"),
        "expected sequential error, got: {stderr}"
    );
    // Sequential start succeeds
    let out = run_cmd(&repo, &["cycle-start", "--cycle", "142"]);
    assert!(out.status.success(), "sequential cycle-start failed: {}", stderr_str(&out));
}

// ----- advance happy path -----

#[test]
fn advance_full_happy_path_reaches_curator() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_cmd(&repo, &["cycle-start", "--cycle", "141"]);

    // reconciler writes inbound-channel, then advance
    write_channel_state(&repo, "inbound-channel", "reconciler", 141);
    let out = run_cmd(&repo, &["advance"]);
    assert!(out.status.success(), "reconciler→planner failed: {}", stderr_str(&out));
    assert!(stdout_str(&out).contains("'reconciler' to 'planner'"));

    // planner writes plan-channel, then advance
    write_channel_state(&repo, "plan-channel", "planner", 141);
    let out = run_cmd(&repo, &["advance"]);
    assert!(out.status.success(), "planner→executor failed: {}", stderr_str(&out));
    assert!(stdout_str(&out).contains("'planner' to 'executor'"));

    // executor writes work-channel, then advance
    write_channel_state(&repo, "work-channel", "executor", 141);
    let out = run_cmd(&repo, &["advance"]);
    assert!(out.status.success(), "executor→curator failed: {}", stderr_str(&out));
    assert!(stdout_str(&out).contains("'executor' to 'curator'"));

    // Now at curator; advance must fail
    let out = run_cmd(&repo, &["advance"]);
    assert!(!out.status.success(), "advance past curator must fail");
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("already at final super-step"),
        "expected final-super-step error, got: {stderr}"
    );
}

#[test]
fn advance_records_transitions_in_state() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_cmd(&repo, &["cycle-start", "--cycle", "141"]);
    write_channel_state(&repo, "inbound-channel", "reconciler", 141);
    run_cmd(&repo, &["advance", "--timestamp", "2026-05-14T00:01:00Z"]);
    let out = run_cmd_json(&repo, &["current"]);
    assert!(out.status.success());
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let transitions = value["transitions"].as_array().unwrap();
    assert_eq!(transitions.len(), 1);
    assert_eq!(transitions[0]["from"], "reconciler");
    assert_eq!(transitions[0]["to"], "planner");
    assert_eq!(transitions[0]["at"], "2026-05-14T00:01:00Z");
}

// ----- advance verification failures -----

#[test]
fn advance_fails_when_channel_state_absent() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_cmd(&repo, &["cycle-start", "--cycle", "141"]);
    // No inbound-channel write at all
    let out = run_cmd(&repo, &["advance"]);
    assert!(!out.status.success());
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("does not exist") || stderr.contains("not initialized"),
        "expected missing-channel error, got: {stderr}"
    );
}

#[test]
fn advance_fails_when_channel_writer_wrong() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_cmd(&repo, &["cycle-start", "--cycle", "141"]);
    // Wrong writer (planner can't write inbound-channel; even if file exists, super-step-boundary
    // sees the reconciler super-step is current and expects writer == reconciler).
    write_channel_state(&repo, "inbound-channel", "planner", 141);
    let out = run_cmd(&repo, &["advance"]);
    assert!(!out.status.success());
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("channel writer is 'planner'"),
        "expected wrong-writer error, got: {stderr}"
    );
}

#[test]
fn advance_fails_when_channel_cycle_wrong() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_cmd(&repo, &["cycle-start", "--cycle", "141"]);
    // Writer correct, but cycle is from a different cycle (stale state).
    write_channel_state(&repo, "inbound-channel", "reconciler", 140);
    let out = run_cmd(&repo, &["advance"]);
    assert!(!out.status.success());
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("channel cycle is 140"),
        "expected wrong-cycle error, got: {stderr}"
    );
}

#[test]
fn advance_with_skip_verify_works_without_channel_write() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_cmd(&repo, &["cycle-start", "--cycle", "141"]);
    let out = run_cmd(&repo, &["advance", "--skip-verify"]);
    assert!(out.status.success(), "skip-verify advance failed: {}", stderr_str(&out));
    let stdout = stdout_str(&out);
    assert!(stdout.contains("verification skipped"));
}

#[test]
fn advance_fails_when_no_cycle_in_progress() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    let out = run_cmd(&repo, &["advance"]);
    assert!(!out.status.success());
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("no cycle is in progress"),
        "expected no-cycle error, got: {stderr}"
    );
}

// ----- cycle-end -----

#[test]
fn cycle_end_happy_path() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_full_happy_cycle(&repo, 141);
    // After cycle-end, state must be empty-sentinel and history must contain the record.
    let cur = run_cmd(&repo, &["current"]);
    assert!(cur.status.success());
    let stdout = stdout_str(&cur);
    assert!(
        stdout.contains("no cycle in progress"),
        "after cycle-end current should be no-cycle, got: {stdout}"
    );
    let history = run_cmd_json(&repo, &["history"]);
    let value: serde_json::Value = serde_json::from_slice(&history.stdout).unwrap();
    let cycles = value["cycles"].as_array().unwrap();
    assert_eq!(cycles.len(), 1);
    assert_eq!(cycles[0]["cycle"], 141);
    let transitions = cycles[0]["transitions"].as_array().unwrap();
    assert_eq!(transitions.len(), 3);
}

#[test]
fn cycle_end_fails_when_not_at_curator() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_cmd(&repo, &["cycle-start", "--cycle", "141"]);
    // Only advance to planner
    write_channel_state(&repo, "inbound-channel", "reconciler", 141);
    run_cmd(&repo, &["advance"]);
    let out = run_cmd(&repo, &["cycle-end", "--cycle", "141"]);
    assert!(!out.status.success());
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("current super-step is 'planner'"),
        "expected not-at-curator error, got: {stderr}"
    );
}

#[test]
fn cycle_end_fails_with_wrong_cycle_number() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_full_happy_cycle_to_curator(&repo, 141);
    let out = run_cmd(&repo, &["cycle-end", "--cycle", "999"]);
    assert!(!out.status.success());
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("cycle 141 is in progress"),
        "expected cycle-mismatch error, got: {stderr}"
    );
}

#[test]
fn cycle_end_fails_when_curator_did_not_write() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_cmd(&repo, &["cycle-start", "--cycle", "141"]);
    // Use skip-verify to reach curator without writing channels
    run_cmd(&repo, &["advance", "--skip-verify"]);
    run_cmd(&repo, &["advance", "--skip-verify"]);
    run_cmd(&repo, &["advance", "--skip-verify"]);
    // No memory-channel write. cycle-end must fail at the verification step.
    let out = run_cmd(&repo, &["cycle-end", "--cycle", "141"]);
    assert!(!out.status.success());
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("channel-output verification failed"),
        "expected verification-failed error, got: {stderr}"
    );
}

#[test]
fn cycle_end_fails_when_no_cycle_in_progress() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    let out = run_cmd(&repo, &["cycle-end", "--cycle", "141"]);
    assert!(!out.status.success());
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("no cycle is in progress"),
        "expected no-cycle error, got: {stderr}"
    );
}

// ----- history -----

#[test]
fn history_with_no_completed_cycles_text() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    let out = run_cmd(&repo, &["history"]);
    assert!(out.status.success());
    let stdout = stdout_str(&out);
    assert!(
        stdout.contains("no completed cycles"),
        "expected no-completed-cycles, got: {stdout}"
    );
}

#[test]
fn history_with_limit_returns_newest_first() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_full_happy_cycle(&repo, 141);
    run_full_happy_cycle(&repo, 142);
    run_full_happy_cycle(&repo, 143);
    let out = run_cmd_json(&repo, &["history", "--limit", "2"]);
    assert!(out.status.success(), "history --limit failed: {}", stderr_str(&out));
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let cycles = value["cycles"].as_array().unwrap();
    assert_eq!(cycles.len(), 2);
    // Newest first: 143 then 142
    assert_eq!(cycles[0]["cycle"], 143);
    assert_eq!(cycles[1]["cycle"], 142);
    assert_eq!(value["total_history_length"], 3);
}

// ----- schema -----

#[test]
fn schema_text_lists_4_roles_4_channels_and_rules() {
    let out = Command::new(BIN).arg("schema").output().unwrap();
    assert!(out.status.success(), "schema failed: {}", stderr_str(&out));
    let stdout = stdout_str(&out);
    for role in ["reconciler", "planner", "executor", "curator"] {
        assert!(stdout.contains(role), "expected role '{role}' in schema text output");
    }
    for ch in ["inbound-channel", "plan-channel", "work-channel", "memory-channel"] {
        assert!(stdout.contains(ch), "expected channel '{ch}' in schema text output");
    }
    assert!(stdout.contains("ordering"));
    assert!(stdout.contains("transition rules"));
}

#[test]
fn schema_json_is_well_formed() {
    let out = Command::new(BIN)
        .arg("--format")
        .arg("json")
        .arg("schema")
        .output()
        .unwrap();
    assert!(out.status.success(), "schema --format json failed: {}", stderr_str(&out));
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let ordering = value["ordering"].as_array().unwrap();
    assert_eq!(ordering.len(), 4);
    assert_eq!(ordering[0], "reconciler");
    assert_eq!(ordering[3], "curator");
    let role_map = value["role_to_channel"].as_object().unwrap();
    assert_eq!(role_map["reconciler"], "inbound-channel");
    assert_eq!(role_map["curator"], "memory-channel");
    let rules = &value["transition_rules"];
    assert_eq!(rules["cycle_start_enters"], "reconciler");
    assert_eq!(rules["cycle_end_requires"], "curator");
    assert_eq!(rules["advance_verifies_channel_output"], true);
}

// ----- multi-cycle sequence -----

#[test]
fn two_consecutive_cycles_run_end_to_end() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_full_happy_cycle(&repo, 141);
    run_full_happy_cycle(&repo, 142);
    let out = run_cmd_json(&repo, &["history"]);
    assert!(out.status.success());
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    let cycles = value["cycles"].as_array().unwrap();
    assert_eq!(cycles.len(), 2);
    assert_eq!(cycles[0]["cycle"], 141);
    assert_eq!(cycles[1]["cycle"], 142);
}

// ----- corrupt-state handling -----

#[test]
fn corrupt_state_file_gives_clean_json_error() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    let state_path = repo.join("state").join("super-step.json");
    std::fs::write(&state_path, "{ this is not valid json").unwrap();
    let out = run_cmd(&repo, &["current"]);
    assert!(!out.status.success());
    let stderr = stderr_str(&out);
    assert!(
        stderr.contains("json error"),
        "expected json-error message, got: {stderr}"
    );
}

// ----- on-disk shape -----

#[test]
fn state_file_has_expected_shape_after_advance() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_cmd(&repo, &["cycle-start", "--cycle", "141", "--timestamp", "2026-05-14T00:00:00Z"]);
    write_channel_state(&repo, "inbound-channel", "reconciler", 141);
    run_cmd(&repo, &["advance", "--timestamp", "2026-05-14T00:01:00Z"]);

    let raw = std::fs::read_to_string(repo.join("state").join("super-step.json")).unwrap();
    let value: serde_json::Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(value["cycle"], 141);
    assert_eq!(value["current_role"], "planner");
    assert_eq!(value["cycle_started_at"], "2026-05-14T00:00:00Z");
    let transitions = value["transitions"].as_array().unwrap();
    assert_eq!(transitions.len(), 1);
    assert_eq!(transitions[0]["from"], "reconciler");
    assert_eq!(transitions[0]["to"], "planner");
}

#[test]
fn history_file_grows_on_each_cycle_end() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_full_happy_cycle(&repo, 141);
    let h1 = std::fs::read_to_string(repo.join("state").join("super-step-history.json")).unwrap();
    let v1: serde_json::Value = serde_json::from_str(&h1).unwrap();
    assert_eq!(v1["cycles"].as_array().unwrap().len(), 1);
    run_full_happy_cycle(&repo, 142);
    let h2 = std::fs::read_to_string(repo.join("state").join("super-step-history.json")).unwrap();
    let v2: serde_json::Value = serde_json::from_str(&h2).unwrap();
    assert_eq!(v2["cycles"].as_array().unwrap().len(), 2);
}

// ----- post-cycle-end no-cycle invariant -----

#[test]
fn cycle_end_leaves_state_in_empty_sentinel() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    run_full_happy_cycle(&repo, 141);
    let raw = std::fs::read_to_string(repo.join("state").join("super-step.json")).unwrap();
    assert_eq!(raw.trim(), "null");
}

// ----- helpers -----

/// Run a complete happy-path cycle (cycle-start through cycle-end) at cycle N.
fn run_full_happy_cycle(repo: &Path, cycle: u32) {
    let cycle_str = cycle.to_string();
    let out = run_cmd(repo, &["cycle-start", "--cycle", &cycle_str]);
    assert!(out.status.success(), "cycle-start {cycle} failed: {}", stderr_str(&out));
    let writers = [
        ("inbound-channel", "reconciler"),
        ("plan-channel", "planner"),
        ("work-channel", "executor"),
        ("memory-channel", "curator"),
    ];
    for (channel, writer) in writers {
        write_channel_state(repo, channel, writer, cycle);
    }
    for _ in 0..3 {
        let out = run_cmd(repo, &["advance"]);
        assert!(out.status.success(), "advance at cycle {cycle} failed: {}", stderr_str(&out));
    }
    let out = run_cmd(repo, &["cycle-end", "--cycle", &cycle_str]);
    assert!(out.status.success(), "cycle-end {cycle} failed: {}", stderr_str(&out));
}

/// Run cycle-start through three advances (reaches curator super-step) but does
/// NOT call cycle-end. Used by tests that need to assert cycle-end's preconditions.
fn run_full_happy_cycle_to_curator(repo: &Path, cycle: u32) {
    let cycle_str = cycle.to_string();
    run_cmd(repo, &["cycle-start", "--cycle", &cycle_str]);
    let writers = [
        ("inbound-channel", "reconciler"),
        ("plan-channel", "planner"),
        ("work-channel", "executor"),
        ("memory-channel", "curator"),
    ];
    for (channel, writer) in writers {
        write_channel_state(repo, channel, writer, cycle);
    }
    for _ in 0..3 {
        run_cmd(repo, &["advance"]);
    }
}
