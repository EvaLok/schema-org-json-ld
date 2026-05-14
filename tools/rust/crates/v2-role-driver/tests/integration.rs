use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::json;
use tempfile::TempDir;

const BIN: &str = env!("CARGO_BIN_EXE_v2-role-driver");

fn run_cmd(repo: &Path, args: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(BIN);
    cmd.arg("--repo-root").arg(repo);
    for a in args {
        cmd.arg(a);
    }
    cmd.output().expect("spawn v2-role-driver")
}

fn run_cmd_json(repo: &Path, args: &[&str]) -> std::process::Output {
    let mut cmd = Command::new(BIN);
    cmd.arg("--repo-root").arg(repo);
    cmd.arg("--format").arg("json");
    for a in args {
        cmd.arg(a);
    }
    cmd.output().expect("spawn v2-role-driver")
}

fn fresh_repo() -> (TempDir, PathBuf) {
    let td = TempDir::new().expect("tempdir");
    let path = td.path().to_path_buf();
    (td, path)
}

fn stdout_str(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stdout).to_string()
}

fn stderr_str(out: &std::process::Output) -> String {
    String::from_utf8_lossy(&out.stderr).to_string()
}

/// Write a super-step state file (simulating v2-super-step-boundary).
fn write_super_step_state(repo: &Path, cycle: u32, current_role: &str) {
    let dir = repo.join("state");
    std::fs::create_dir_all(&dir).unwrap();
    let body = json!({
        "cycle": cycle,
        "current_role": current_role,
        "cycle_started_at": "2026-05-14T05:00:00Z",
        "transitions": [],
    });
    std::fs::write(
        dir.join("super-step.json"),
        serde_json::to_string_pretty(&body).unwrap(),
    )
    .unwrap();
}

/// Write a populated channel state file (simulating v2-channel-router).
fn write_channel_state(repo: &Path, channel: &str, writer: &str, cycle: u32, payload: serde_json::Value) {
    let dir = repo.join("state").join("channels");
    std::fs::create_dir_all(&dir).unwrap();
    let body = json!({
        "channel": channel,
        "writer": writer,
        "cycle": cycle,
        "timestamp": "2026-05-14T05:00:00Z",
        "payload": payload,
    });
    std::fs::write(
        dir.join(format!("{channel}.json")),
        serde_json::to_string_pretty(&body).unwrap(),
    )
    .unwrap();
}

/// Write a session-output file with the full WritePayload envelope.
fn write_session_output_full(repo: &Path, name: &str, cycle: u32, timestamp: &str, payload: serde_json::Value) -> PathBuf {
    let path = repo.join(format!("{name}.json"));
    let body = json!({
        "cycle": cycle,
        "timestamp": timestamp,
        "payload": payload,
    });
    std::fs::write(&path, serde_json::to_string_pretty(&body).unwrap()).unwrap();
    path
}

fn payload_for(role: &str) -> serde_json::Value {
    match role {
        "planner" => json!({"substantive-focal": "scaffold v2-role-driver", "per-role-tasks": {"executor": "wire output channel write"}}),
        "executor" => json!({"artifacts-written": ["tools/rust/crates/v2-role-driver/src/main.rs"]}),
        "curator" => json!({"consolidated-insights": ["multi-agent topology scaffold consolidation"]}),
        "reconciler" => json!({"eva-responses": [], "audit-posts": [], "dispatch-returns": []}),
        _ => panic!("unknown role: {role}"),
    }
}

// =================================================================
// init
// =================================================================

#[test]
fn init_creates_state_roles_and_history_files() {
    let (_td, repo) = fresh_repo();
    let out = run_cmd(&repo, &["init"]);
    assert!(out.status.success(), "init failed: {}", stderr_str(&out));
    for role in ["reconciler", "planner", "executor", "curator"] {
        let path = repo.join("state").join("roles").join(format!("{role}-history.json"));
        assert!(path.exists(), "expected per-role history file for {role}");
    }
}

#[test]
fn init_is_idempotent() {
    let (_td, repo) = fresh_repo();
    assert!(run_cmd(&repo, &["init"]).status.success());
    let out2 = run_cmd(&repo, &["init"]);
    assert!(out2.status.success(), "second init failed: {}", stderr_str(&out2));
    let s = stdout_str(&out2);
    assert!(s.contains("present"), "expected 'present' on second init, got: {s}");
}

#[test]
fn init_json_output_splits_created_and_already_present() {
    let (_td, repo) = fresh_repo();
    let out1 = run_cmd_json(&repo, &["init"]);
    assert!(out1.status.success());
    let v1: serde_json::Value = serde_json::from_str(&stdout_str(&out1)).expect("init json");
    assert_eq!(v1["created"].as_array().unwrap().len(), 4);
    assert_eq!(v1["already_present"].as_array().unwrap().len(), 0);

    let out2 = run_cmd_json(&repo, &["init"]);
    let v2: serde_json::Value = serde_json::from_str(&stdout_str(&out2)).expect("init json 2");
    assert_eq!(v2["created"].as_array().unwrap().len(), 0);
    assert_eq!(v2["already_present"].as_array().unwrap().len(), 4);
}

// =================================================================
// invoke — happy paths
// =================================================================

fn happy_invoke(repo: &Path, role: &str, cycle: u32) -> std::process::Output {
    run_cmd(repo, &["init"]);
    write_super_step_state(repo, cycle, role);
    let out_path = write_session_output_full(
        repo,
        &format!("{role}-out"),
        cycle,
        "2026-05-14T05:10:00Z",
        payload_for(role),
    );
    let path_str = out_path.to_string_lossy().to_string();
    run_cmd(repo, &[
        "invoke",
        "--role", role,
        "--cycle", &cycle.to_string(),
        "--session-output-file", &path_str,
        "--timestamp", "2026-05-14T05:11:00Z",
    ])
}

#[test]
fn invoke_planner_happy_path() {
    let (_td, repo) = fresh_repo();
    let out = happy_invoke(&repo, "planner", 1);
    assert!(out.status.success(), "invoke failed: {}", stderr_str(&out));
    let s = stdout_str(&out);
    assert!(s.contains("outcome=success"));
    assert!(s.contains("role=planner"));
}

#[test]
fn invoke_each_role_succeeds() {
    for role in ["reconciler", "planner", "executor", "curator"] {
        let (_td, repo) = fresh_repo();
        let out = happy_invoke(&repo, role, 7);
        assert!(out.status.success(), "invoke {role} failed: {}", stderr_str(&out));
    }
}

#[test]
fn invoke_writes_channel_state_envelope() {
    let (_td, repo) = fresh_repo();
    let _out = happy_invoke(&repo, "planner", 3);
    let chan_path = repo.join("state").join("channels").join("plan-channel.json");
    let body: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&chan_path).unwrap()).unwrap();
    assert_eq!(body["channel"], "plan-channel");
    assert_eq!(body["writer"], "planner");
    assert_eq!(body["cycle"], 3);
    assert_eq!(body["timestamp"], "2026-05-14T05:11:00Z");
    assert!(body["payload"]["substantive-focal"].is_string());
}

#[test]
fn invoke_appends_channel_history() {
    let (_td, repo) = fresh_repo();
    let _out = happy_invoke(&repo, "executor", 5);
    let h_path = repo
        .join("state")
        .join("channels")
        .join("work-channel-history.json");
    let body: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&h_path).unwrap()).unwrap();
    assert_eq!(body["channel"], "work-channel");
    let entries = body["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["writer"], "executor");
    assert_eq!(entries[0]["cycle"], 5);
}

#[test]
fn invoke_appends_to_per_role_history() {
    let (_td, repo) = fresh_repo();
    let _ = happy_invoke(&repo, "curator", 9);
    let path = repo
        .join("state")
        .join("roles")
        .join("curator-history.json");
    let body: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    let runs = body["runs"].as_array().unwrap();
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0]["cycle"], 9);
    assert_eq!(runs[0]["role"], "curator");
    assert_eq!(runs[0]["outcome"], "success");
}

#[test]
fn invoke_two_cycles_same_role_accumulates() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);

    write_super_step_state(&repo, 1, "planner");
    let p1 = write_session_output_full(&repo, "p1", 1, "t1", payload_for("planner"));
    let o1 = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p1.to_string_lossy()]);
    assert!(o1.status.success());

    write_super_step_state(&repo, 2, "planner");
    let p2 = write_session_output_full(&repo, "p2", 2, "t2", payload_for("planner"));
    let o2 = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "2",
        "--session-output-file", &p2.to_string_lossy()]);
    assert!(o2.status.success(), "second invoke failed: {}", stderr_str(&o2));

    let path = repo.join("state").join("roles").join("planner-history.json");
    let body: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    let runs = body["runs"].as_array().unwrap();
    assert_eq!(runs.len(), 2);
    assert_eq!(runs[0]["cycle"], 1);
    assert_eq!(runs[1]["cycle"], 2);

    let h_path = repo.join("state").join("channels").join("plan-channel-history.json");
    let h_body: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&h_path).unwrap()).unwrap();
    let entries = h_body["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 2);
}

#[test]
fn invoke_json_output_includes_channel_and_outcome() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 1, "executor");
    let p = write_session_output_full(&repo, "ex", 1, "t", payload_for("executor"));
    let out = run_cmd_json(&repo, &["invoke", "--role", "executor", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy()]);
    assert!(out.status.success(), "{}", stderr_str(&out));
    let v: serde_json::Value = serde_json::from_str(&stdout_str(&out)).unwrap();
    assert_eq!(v["role"], "executor");
    assert_eq!(v["cycle"], 1);
    assert_eq!(v["outcome"], "success");
    assert_eq!(v["channel"], "work-channel");
    assert_eq!(v["channel_write_performed"], true);
    assert_eq!(v["super_step_check_performed"], true);
}

// =================================================================
// invoke — super-step verification
// =================================================================

#[test]
fn invoke_fails_when_super_step_not_in_progress() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    let p = write_session_output_full(&repo, "p", 1, "t", payload_for("planner"));
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy()]);
    assert!(!out.status.success());
    assert!(stderr_str(&out).contains("no super-step is in progress"));
}

#[test]
fn invoke_fails_when_super_step_role_mismatch() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 1, "executor"); // current role is executor
    let p = write_session_output_full(&repo, "p", 1, "t", payload_for("planner"));
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy()]);
    assert!(!out.status.success());
    let s = stderr_str(&out);
    assert!(s.contains("super-step mismatch"), "got: {s}");
    assert!(s.contains("planner"));
    assert!(s.contains("executor"));
}

#[test]
fn invoke_fails_when_super_step_cycle_mismatch() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 5, "planner");
    let p = write_session_output_full(&repo, "p", 1, "t", payload_for("planner"));
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy()]);
    assert!(!out.status.success());
    let s = stderr_str(&out);
    assert!(s.contains("super-step mismatch"), "got: {s}");
    assert!(s.contains("cycle 5") || s.contains("for cycle 5"));
}

#[test]
fn invoke_skip_super_step_check_bypasses_verification() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    // intentionally no super-step state file
    let p = write_session_output_full(&repo, "p", 1, "t", payload_for("planner"));
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy(),
        "--skip-super-step-check"]);
    assert!(out.status.success(), "{}", stderr_str(&out));
}

#[test]
fn invoke_fails_when_super_step_state_is_empty_sentinel() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    // Write the empty sentinel (matches what v2-super-step-boundary writes for "no cycle in progress")
    let dir = repo.join("state");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("super-step.json"), "null\n").unwrap();

    let p = write_session_output_full(&repo, "p", 1, "t", payload_for("planner"));
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy()]);
    assert!(!out.status.success());
    assert!(stderr_str(&out).contains("no super-step is in progress"));
}

// =================================================================
// invoke — session-output validation
// =================================================================

#[test]
fn invoke_fails_when_session_output_file_missing() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 1, "planner");
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", "/does/not/exist.json"]);
    assert!(!out.status.success());
    assert!(stderr_str(&out).contains("session-output file not found"));
}

#[test]
fn invoke_fails_when_session_output_not_json() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 1, "planner");
    let p = repo.join("bad.json");
    std::fs::write(&p, "definitely not json").unwrap();
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy()]);
    assert!(!out.status.success());
    assert!(stderr_str(&out).contains("invalid session output"));
}

#[test]
fn invoke_fails_when_payload_missing_required_keys() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 1, "planner");
    // missing per-role-tasks
    let p = write_session_output_full(&repo, "p", 1, "t", json!({"substantive-focal": "x"}));
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy()]);
    assert!(!out.status.success());
    let s = stderr_str(&out);
    assert!(s.contains("per-role-tasks"), "got: {s}");
}

#[test]
fn invoke_fails_when_payload_is_not_object() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 1, "planner");
    let p = repo.join("arr.json");
    std::fs::write(&p, r#"{"payload": [1, 2, 3]}"#).unwrap();
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy()]);
    assert!(!out.status.success());
    let s = stderr_str(&out);
    assert!(s.contains("array") || s.contains("must be a JSON object"));
}

#[test]
fn invoke_fails_when_cycle_payload_mismatch() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 1, "planner");
    // payload declares cycle 99 but --cycle is 1
    let p = write_session_output_full(&repo, "p", 99, "t", payload_for("planner"));
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy()]);
    assert!(!out.status.success());
    assert!(stderr_str(&out).contains("cycle mismatch"));
}

#[test]
fn invoke_accepts_payload_only_shape() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 1, "planner");
    let p = repo.join("payload-only.json");
    std::fs::write(
        &p,
        serde_json::to_string_pretty(&json!({"payload": payload_for("planner")})).unwrap(),
    )
    .unwrap();
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy(),
        "--timestamp", "ts-cli"]);
    assert!(out.status.success(), "{}", stderr_str(&out));
    let chan: serde_json::Value = serde_json::from_slice(
        &std::fs::read(repo.join("state").join("channels").join("plan-channel.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(chan["timestamp"], "ts-cli");
}

#[test]
fn invoke_accepts_bare_payload_shape() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 1, "curator");
    let p = repo.join("bare.json");
    std::fs::write(
        &p,
        serde_json::to_string_pretty(&payload_for("curator")).unwrap(),
    )
    .unwrap();
    let out = run_cmd(&repo, &["invoke", "--role", "curator", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy()]);
    assert!(out.status.success(), "{}", stderr_str(&out));
}

// =================================================================
// invoke — skip flags
// =================================================================

#[test]
fn invoke_skip_channel_write_records_skipped_outcome() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 1, "planner");
    let p = write_session_output_full(&repo, "p", 1, "t", payload_for("planner"));
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy(),
        "--skip-channel-write"]);
    assert!(out.status.success(), "{}", stderr_str(&out));
    assert!(stdout_str(&out).contains("outcome=write-skipped"));

    // channel state file should NOT have been created
    let chan = repo.join("state").join("channels").join("plan-channel.json");
    assert!(!chan.exists(), "channel state file should not exist when --skip-channel-write");

    // but per-role history should still record the run
    let hist = repo.join("state").join("roles").join("planner-history.json");
    let body: serde_json::Value = serde_json::from_slice(&std::fs::read(hist).unwrap()).unwrap();
    let runs = body["runs"].as_array().unwrap();
    assert_eq!(runs.len(), 1);
    assert_eq!(runs[0]["outcome"], "write-skipped");
}

#[test]
fn invoke_does_not_write_history_when_validation_fails() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 1, "planner");
    // missing required keys
    let p = write_session_output_full(&repo, "p", 1, "t", json!({"substantive-focal": "only this"}));
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy()]);
    assert!(!out.status.success());

    let hist = repo.join("state").join("roles").join("planner-history.json");
    let body: serde_json::Value = serde_json::from_slice(&std::fs::read(hist).unwrap()).unwrap();
    let runs = body["runs"].as_array().unwrap();
    assert!(runs.is_empty(), "no run should be appended when validation fails");
}

// =================================================================
// context
// =================================================================

#[test]
fn context_for_reconciler_has_no_inputs() {
    let (_td, repo) = fresh_repo();
    let out = run_cmd(&repo, &["context", "--role", "reconciler", "--cycle", "1"]);
    assert!(out.status.success(), "{}", stderr_str(&out));
    let s = stdout_str(&out);
    assert!(s.contains("(none — role polls external surfaces)"));
}

#[test]
fn context_for_planner_lists_memory_and_inbound_channels() {
    let (_td, repo) = fresh_repo();
    let out = run_cmd(&repo, &["context", "--role", "planner", "--cycle", "5"]);
    assert!(out.status.success(), "{}", stderr_str(&out));
    let s = stdout_str(&out);
    assert!(s.contains("memory-channel"));
    assert!(s.contains("inbound-channel"));
}

#[test]
fn context_reports_prompt_not_found_when_absent() {
    let (_td, repo) = fresh_repo();
    let out = run_cmd(&repo, &["context", "--role", "planner", "--cycle", "1"]);
    assert!(out.status.success());
    let s = stdout_str(&out);
    assert!(s.contains("PROMPT-NOT-FOUND"), "got: {s}");
}

#[test]
fn context_reads_prompt_file_when_present() {
    let (_td, repo) = fresh_repo();
    // Default location:
    let dir = repo.join("prompts").join("v2");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("planner-prompt.xml"), "<planner-prompt/>").unwrap();
    let out = run_cmd(&repo, &["context", "--role", "planner", "--cycle", "1"]);
    assert!(out.status.success());
    let s = stdout_str(&out);
    assert!(s.contains("prompt present: yes"));
    assert!(!s.contains("PROMPT-NOT-FOUND"));
}

#[test]
fn context_prompt_file_override_works() {
    let (_td, repo) = fresh_repo();
    let custom = repo.join("custom-prompt.xml");
    std::fs::write(&custom, "<custom/>").unwrap();
    let out = run_cmd(&repo, &["context", "--role", "executor", "--cycle", "1",
        "--prompt-file", &custom.to_string_lossy()]);
    assert!(out.status.success());
    let s = stdout_str(&out);
    assert!(s.contains("prompt present: yes"));
    assert!(s.contains("custom-prompt.xml"));
}

#[test]
fn context_reports_populated_channel_state() {
    let (_td, repo) = fresh_repo();
    write_channel_state(&repo, "plan-channel", "planner", 7, payload_for("planner"));
    let out = run_cmd(&repo, &["context", "--role", "executor", "--cycle", "7"]);
    assert!(out.status.success(), "{}", stderr_str(&out));
    let s = stdout_str(&out);
    assert!(s.contains("plan-channel"));
    assert!(s.contains("populated"));
    assert!(s.contains("by planner"));
}

#[test]
fn context_json_output_well_formed() {
    let (_td, repo) = fresh_repo();
    write_channel_state(&repo, "work-channel", "executor", 3, payload_for("executor"));
    let out = run_cmd_json(&repo, &["context", "--role", "curator", "--cycle", "3"]);
    assert!(out.status.success());
    let v: serde_json::Value = serde_json::from_str(&stdout_str(&out)).expect("json output");
    assert_eq!(v["role"], "curator");
    assert_eq!(v["cycle"], 3);
    let inputs = v["inputs"].as_array().unwrap();
    assert_eq!(inputs.len(), 1);
    assert_eq!(inputs[0]["channel"], "work-channel");
}

// =================================================================
// inputs
// =================================================================

#[test]
fn inputs_each_role_lists_correct_bindings() {
    for (role, expected_in, expected_out) in [
        ("planner", vec!["memory-channel", "inbound-channel"], "plan-channel"),
        ("executor", vec!["plan-channel"], "work-channel"),
        ("curator", vec!["work-channel"], "memory-channel"),
        ("reconciler", vec![], "inbound-channel"),
    ] {
        let (_td, repo) = fresh_repo();
        let out = run_cmd(&repo, &["inputs", "--role", role]);
        assert!(out.status.success(), "inputs --role {role} failed: {}", stderr_str(&out));
        let s = stdout_str(&out);
        for ch in expected_in {
            assert!(s.contains(ch), "expected {ch} in output for {role}, got: {s}");
        }
        assert!(s.contains(expected_out));
    }
}

#[test]
fn inputs_with_cycle_reads_channel_state() {
    let (_td, repo) = fresh_repo();
    write_channel_state(&repo, "plan-channel", "planner", 1, payload_for("planner"));
    let out = run_cmd(&repo, &["inputs", "--role", "executor", "--cycle", "1"]);
    assert!(out.status.success(), "{}", stderr_str(&out));
    let s = stdout_str(&out);
    assert!(s.contains("state=populated"));
}

#[test]
fn inputs_without_cycle_omits_state() {
    let (_td, repo) = fresh_repo();
    let out = run_cmd(&repo, &["inputs", "--role", "executor"]);
    assert!(out.status.success());
    let s = stdout_str(&out);
    // Without --cycle we don't read state at all
    assert!(!s.contains("state="));
}

#[test]
fn inputs_json_well_formed() {
    let (_td, repo) = fresh_repo();
    let out = run_cmd_json(&repo, &["inputs", "--role", "planner"]);
    assert!(out.status.success());
    let v: serde_json::Value = serde_json::from_str(&stdout_str(&out)).unwrap();
    assert_eq!(v["role"], "planner");
    assert_eq!(v["output_channel"], "plan-channel");
    let inputs = v["inputs"].as_array().unwrap();
    assert_eq!(inputs.len(), 2);
}

// =================================================================
// history
// =================================================================

#[test]
fn history_before_init_errors() {
    let (_td, repo) = fresh_repo();
    let out = run_cmd(&repo, &["history", "--role", "planner"]);
    assert!(!out.status.success());
    assert!(stderr_str(&out).contains("not initialized"));
}

#[test]
fn history_after_init_is_empty() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    let out = run_cmd(&repo, &["history", "--role", "planner"]);
    assert!(out.status.success(), "{}", stderr_str(&out));
    assert!(stdout_str(&out).contains("(no runs)"));
}

#[test]
fn history_limit_newest_first() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    for cycle in [1u32, 2, 3] {
        write_super_step_state(&repo, cycle, "planner");
        let p = write_session_output_full(
            &repo,
            &format!("p{cycle}"),
            cycle,
            &format!("ts-{cycle}"),
            payload_for("planner"),
        );
        let o = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", &cycle.to_string(),
            "--session-output-file", &p.to_string_lossy()]);
        assert!(o.status.success(), "{}", stderr_str(&o));
    }

    let out = run_cmd_json(&repo, &["history", "--role", "planner", "--limit", "2"]);
    assert!(out.status.success());
    let v: serde_json::Value = serde_json::from_str(&stdout_str(&out)).unwrap();
    let runs = v["runs"].as_array().unwrap();
    assert_eq!(runs.len(), 2);
    // newest-first
    assert_eq!(runs[0]["cycle"], 3);
    assert_eq!(runs[1]["cycle"], 2);
}

#[test]
fn history_corrupt_file_clean_error() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    let path = repo.join("state").join("roles").join("planner-history.json");
    std::fs::write(&path, "{not: valid json").unwrap();
    let out = run_cmd(&repo, &["history", "--role", "planner"]);
    assert!(!out.status.success());
    assert!(stderr_str(&out).contains("json error"));
}

// =================================================================
// schema
// =================================================================

#[test]
fn schema_text_lists_all_roles() {
    let (_td, repo) = fresh_repo();
    let out = run_cmd(&repo, &["schema"]);
    assert!(out.status.success());
    let s = stdout_str(&out);
    for role in ["reconciler", "planner", "executor", "curator"] {
        assert!(s.contains(role), "missing role {role} from schema: {s}");
    }
    for channel in ["plan-channel", "work-channel", "memory-channel", "inbound-channel"] {
        assert!(s.contains(channel), "missing channel {channel}: {s}");
    }
    assert!(s.contains("success"));
    assert!(s.contains("write-skipped"));
}

#[test]
fn schema_json_well_formed() {
    let (_td, repo) = fresh_repo();
    let out = run_cmd_json(&repo, &["schema"]);
    assert!(out.status.success());
    let v: serde_json::Value = serde_json::from_str(&stdout_str(&out)).expect("schema json");
    let roles = v["roles"].as_array().unwrap();
    assert_eq!(roles.len(), 4);
    let outcomes = v["run_record"]["outcomes"].as_array().unwrap();
    assert_eq!(outcomes.len(), 2);
    assert!(v["paths"]["role_history"].is_string());
}

// =================================================================
// session-output --timestamp precedence
// =================================================================

#[test]
fn invoke_cli_timestamp_overrides_payload_timestamp() {
    let (_td, repo) = fresh_repo();
    run_cmd(&repo, &["init"]);
    write_super_step_state(&repo, 1, "planner");
    let p = write_session_output_full(&repo, "p", 1, "payload-ts", payload_for("planner"));
    let out = run_cmd(&repo, &["invoke", "--role", "planner", "--cycle", "1",
        "--session-output-file", &p.to_string_lossy(),
        "--timestamp", "cli-ts"]);
    assert!(out.status.success(), "{}", stderr_str(&out));
    let chan: serde_json::Value = serde_json::from_slice(
        &std::fs::read(repo.join("state").join("channels").join("plan-channel.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(chan["timestamp"], "cli-ts");
}
