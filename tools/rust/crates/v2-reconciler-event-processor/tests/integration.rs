use std::path::Path;
use std::process::Command;

use serde_json::{json, Value};
use tempfile::TempDir;

const BIN: &str = env!("CARGO_BIN_EXE_v2-reconciler-event-processor");

// ----- shared helpers -----

fn run_cmd(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new(BIN)
        .args(["--repo-root", repo.to_str().unwrap()])
        .args(args)
        .output()
        .expect("subprocess spawn")
}

fn run_cmd_json(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new(BIN)
        .args(["--repo-root", repo.to_str().unwrap(), "--format", "json"])
        .args(args)
        .output()
        .expect("subprocess spawn")
}

fn assert_success(out: &std::process::Output) {
    assert!(
        out.status.success(),
        "command failed: status={:?}\nstdout={}\nstderr={}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn assert_failure(out: &std::process::Output) {
    assert!(
        !out.status.success(),
        "expected command to fail but it succeeded.\nstdout={}\nstderr={}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn stdout_str(out: &std::process::Output) -> String {
    String::from_utf8(out.stdout.clone()).expect("stdout utf-8")
}

fn stderr_str(out: &std::process::Output) -> String {
    String::from_utf8(out.stderr.clone()).expect("stderr utf-8")
}

fn stdout_json(out: &std::process::Output) -> Value {
    serde_json::from_str(&stdout_str(out)).expect("stdout is JSON")
}

/// Initialize both the channels directory (mirroring v2-channel-router init) and
/// the reconciler directory. The channel-router init is simulated here by creating
/// the channels dir + the inbound-channel state file as an empty sentinel and the
/// inbound-channel history file as the empty-shape envelope. SCAFFOLD tests do not
/// invoke v2-channel-router directly.
fn init_all(repo: &Path) {
    // Channels dir + inbound state/history sentinels.
    let channels = repo.join("state").join("channels");
    std::fs::create_dir_all(&channels).unwrap();
    std::fs::write(channels.join("inbound-channel.json"), "null").unwrap();
    let empty_history = serde_json::json!({
        "channel": "inbound-channel",
        "entries": []
    });
    std::fs::write(
        channels.join("inbound-channel-history.json"),
        serde_json::to_string_pretty(&empty_history).unwrap(),
    )
    .unwrap();

    // Reconciler init via the binary itself.
    let out = run_cmd(repo, &["init"]);
    assert_success(&out);
}

/// Place an in-progress super-step state for the reconciler at the given cycle.
fn write_super_step_state_reconciler(repo: &Path, cycle: u32) {
    let path = repo.join("state").join("super-step.json");
    let payload = serde_json::json!({
        "cycle": cycle,
        "current_role": "reconciler",
        "cycle_started_at": "1970-01-01T00:00:00Z",
        "transitions": []
    });
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();
}

/// Place an in-progress super-step state for a non-reconciler role at the given cycle
/// (for mismatch tests).
fn write_super_step_state_role(repo: &Path, cycle: u32, role_kebab: &str) {
    let path = repo.join("state").join("super-step.json");
    let payload = serde_json::json!({
        "cycle": cycle,
        "current_role": role_kebab,
        "cycle_started_at": "1970-01-01T00:00:00Z",
        "transitions": []
    });
    std::fs::write(&path, serde_json::to_string_pretty(&payload).unwrap()).unwrap();
}

/// Write a source file for a given source kind.
fn write_source_file(path: &Path, source: &str, events: Vec<Value>) {
    let body = serde_json::json!({
        "source": source,
        "events": events,
    });
    std::fs::write(path, serde_json::to_string_pretty(&body).unwrap()).unwrap();
}

/// Build a minimal Event value.
fn event(id: &str) -> Value {
    json!({
        "id": id,
        "at": "1970-01-01T00:00:00Z",
        "raw": {}
    })
}

/// Run a poll with all-three source files. Returns the subprocess output.
fn poll_with_sources(
    repo: &Path,
    cycle: u32,
    eva: Option<&Path>,
    audit: Option<&Path>,
    dispatch: Option<&Path>,
) -> std::process::Output {
    let cycle_str = cycle.to_string();
    let mut args: Vec<String> = vec![
        "poll".into(),
        "--cycle".into(),
        cycle_str,
        "--timestamp".into(),
        "2026-05-14T07:00:00Z".into(),
    ];
    if let Some(p) = eva {
        args.push("--eva-source-file".into());
        args.push(p.to_string_lossy().into_owned());
    }
    if let Some(p) = audit {
        args.push("--audit-source-file".into());
        args.push(p.to_string_lossy().into_owned());
    }
    if let Some(p) = dispatch {
        args.push("--dispatch-source-file".into());
        args.push(p.to_string_lossy().into_owned());
    }
    let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run_cmd(repo, &str_args)
}

fn poll_with_sources_json(
    repo: &Path,
    cycle: u32,
    eva: Option<&Path>,
    audit: Option<&Path>,
    dispatch: Option<&Path>,
) -> std::process::Output {
    let cycle_str = cycle.to_string();
    let mut args: Vec<String> = vec![
        "poll".into(),
        "--cycle".into(),
        cycle_str,
        "--timestamp".into(),
        "2026-05-14T07:00:00Z".into(),
    ];
    if let Some(p) = eva {
        args.push("--eva-source-file".into());
        args.push(p.to_string_lossy().into_owned());
    }
    if let Some(p) = audit {
        args.push("--audit-source-file".into());
        args.push(p.to_string_lossy().into_owned());
    }
    if let Some(p) = dispatch {
        args.push("--dispatch-source-file".into());
        args.push(p.to_string_lossy().into_owned());
    }
    let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run_cmd_json(repo, &str_args)
}

// ----- init -----

#[test]
fn init_creates_reconciler_dir_and_state_files() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    let out = run_cmd(repo, &["init"]);
    assert_success(&out);

    let dir = repo.join("state").join("reconciler");
    assert!(dir.exists());
    assert!(dir.join("eva-cursor.json").exists());
    assert!(dir.join("audit-cursor.json").exists());
    assert!(dir.join("dispatch-cursor.json").exists());
    assert!(dir.join("poll-history.json").exists());
}

#[test]
fn init_is_idempotent() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    assert_success(&run_cmd(repo, &["init"]));
    let out2 = run_cmd(repo, &["init"]);
    assert_success(&out2);
    let s = stdout_str(&out2);
    assert!(s.contains("already-present"), "expected already-present line, got: {s}");
}

#[test]
fn init_json_splits_created_vs_already_present() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    let out1 = run_cmd_json(repo, &["init"]);
    assert_success(&out1);
    let v1 = stdout_json(&out1);
    assert_eq!(v1["created"].as_array().unwrap().len(), 4);
    assert!(v1["already_present"].as_array().unwrap().is_empty());

    let out2 = run_cmd_json(repo, &["init"]);
    assert_success(&out2);
    let v2 = stdout_json(&out2);
    assert!(v2["created"].as_array().unwrap().is_empty());
    assert_eq!(v2["already_present"].as_array().unwrap().len(), 4);
}

#[test]
fn init_cursor_files_have_null_value_sentinel() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    assert_success(&run_cmd(repo, &["init"]));
    for src in &["eva", "audit", "dispatch"] {
        let path = repo
            .join("state")
            .join("reconciler")
            .join(format!("{src}-cursor.json"));
        let raw = std::fs::read_to_string(&path).unwrap();
        let v: Value = serde_json::from_str(&raw).unwrap();
        assert_eq!(v["source"], json!(*src));
        assert!(v["value"].is_null());
    }
}

// ----- poll: happy paths -----

#[test]
fn poll_with_all_empty_sources_writes_empty_inbound_payload() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let out = poll_with_sources(repo, 1, None, None, None);
    assert_success(&out);

    // The inbound-channel state file should now be a populated envelope.
    let state_path = repo.join("state").join("channels").join("inbound-channel.json");
    let raw = std::fs::read_to_string(&state_path).unwrap();
    let v: Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v["channel"], json!("inbound-channel"));
    assert_eq!(v["writer"], json!("reconciler"));
    assert_eq!(v["cycle"], json!(1));
    assert!(v["payload"]["eva-responses"].as_array().unwrap().is_empty());
    assert!(v["payload"]["audit-posts"].as_array().unwrap().is_empty());
    assert!(v["payload"]["dispatch-returns"].as_array().unwrap().is_empty());
    assert_eq!(v["payload"]["inbound-completeness-marker"], json!("quiet"));
}

#[test]
fn poll_with_events_in_all_sources_populates_inbound_payload() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let eva_path = tmp.path().join("eva-src.json");
    let audit_path = tmp.path().join("audit-src.json");
    let dispatch_path = tmp.path().join("dispatch-src.json");
    write_source_file(&eva_path, "eva", vec![event("100"), event("101")]);
    write_source_file(&audit_path, "audit", vec![event("abc123")]);
    write_source_file(&dispatch_path, "dispatch", vec![]);

    let out =
        poll_with_sources(repo, 1, Some(&eva_path), Some(&audit_path), Some(&dispatch_path));
    assert_success(&out);

    let state_path = repo.join("state").join("channels").join("inbound-channel.json");
    let v: Value =
        serde_json::from_str(&std::fs::read_to_string(state_path).unwrap()).unwrap();
    assert_eq!(v["payload"]["eva-responses"].as_array().unwrap().len(), 2);
    assert_eq!(v["payload"]["audit-posts"].as_array().unwrap().len(), 1);
    assert_eq!(v["payload"]["dispatch-returns"].as_array().unwrap().len(), 0);
    assert_eq!(
        v["payload"]["inbound-completeness-marker"],
        json!("complete")
    );
}

#[test]
fn poll_appends_inbound_channel_history() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let eva_path = tmp.path().join("eva-src.json");
    write_source_file(&eva_path, "eva", vec![event("100")]);

    let out = poll_with_sources(repo, 1, Some(&eva_path), None, None);
    assert_success(&out);

    let history_path = repo
        .join("state")
        .join("channels")
        .join("inbound-channel-history.json");
    let v: Value =
        serde_json::from_str(&std::fs::read_to_string(history_path).unwrap()).unwrap();
    assert_eq!(v["channel"], json!("inbound-channel"));
    assert_eq!(v["entries"].as_array().unwrap().len(), 1);
    assert_eq!(v["entries"][0]["writer"], json!("reconciler"));
    assert_eq!(v["entries"][0]["cycle"], json!(1));
}

#[test]
fn poll_appends_poll_history() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let out = poll_with_sources(repo, 1, None, None, None);
    assert_success(&out);

    let history_path = repo
        .join("state")
        .join("reconciler")
        .join("poll-history.json");
    let v: Value =
        serde_json::from_str(&std::fs::read_to_string(history_path).unwrap()).unwrap();
    let entries = v["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["cycle"], json!(1));
    assert_eq!(entries[0]["outcome"], json!("success"));
}

#[test]
fn poll_updates_cursors_after_successful_write() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let eva_path = tmp.path().join("eva-src.json");
    write_source_file(&eva_path, "eva", vec![event("100"), event("105"), event("103")]);

    let out = poll_with_sources(repo, 1, Some(&eva_path), None, None);
    assert_success(&out);

    let cursor_path = repo
        .join("state")
        .join("reconciler")
        .join("eva-cursor.json");
    let v: Value =
        serde_json::from_str(&std::fs::read_to_string(cursor_path).unwrap()).unwrap();
    // Lex max over {"100","105","103"} is "105".
    assert_eq!(v["value"], json!("105"));
}

#[test]
fn poll_json_output_includes_summaries_and_outcome() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let out = poll_with_sources_json(repo, 1, None, None, None);
    assert_success(&out);
    let v = stdout_json(&out);
    assert_eq!(v["cycle"], json!(1));
    assert_eq!(v["outcome"], json!("success"));
    assert_eq!(v["super_step_check_performed"], json!(true));
    assert!(v["eva"]["events_in"].is_number());
    assert!(v["audit"]["events_in"].is_number());
    assert!(v["dispatch"]["events_in"].is_number());
    assert!(v["inbound_payload"]["eva-responses"].is_array());
}

#[test]
fn poll_text_output_renders_per_source_summary_lines() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let out = poll_with_sources(repo, 1, None, None, None);
    assert_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("cycle:"));
    assert!(s.contains("outcome:"));
    assert!(s.contains("eva"));
    assert!(s.contains("audit"));
    assert!(s.contains("dispatch"));
}

// ----- poll: super-step verification -----

#[test]
fn poll_fails_when_super_step_not_in_progress() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    // Do NOT write super-step state.

    let out = poll_with_sources(repo, 1, None, None, None);
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("super-step"), "stderr was: {s}");
}

#[test]
fn poll_fails_on_role_mismatch() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_role(repo, 1, "planner");

    let out = poll_with_sources(repo, 1, None, None, None);
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("super-step mismatch"));
    assert!(s.contains("planner"));
}

#[test]
fn poll_fails_on_cycle_mismatch() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 5);

    let out = poll_with_sources(repo, 1, None, None, None);
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("super-step mismatch"));
}

#[test]
fn poll_skip_super_step_check_bypasses_verification() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    // No super-step state; --skip-super-step-check should still succeed.

    let out = run_cmd(
        repo,
        &[
            "poll",
            "--cycle",
            "1",
            "--skip-super-step-check",
            "--timestamp",
            "2026-05-14T07:00:00Z",
        ],
    );
    assert_success(&out);
}

#[test]
fn poll_fails_when_super_step_state_is_null_sentinel() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    std::fs::write(
        repo.join("state").join("super-step.json"),
        "null",
    )
    .unwrap();

    let out = poll_with_sources(repo, 1, None, None, None);
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("no super-step in progress"));
}

// ----- poll: source file validation -----

#[test]
fn poll_fails_when_source_file_missing() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let bogus = tmp.path().join("does-not-exist.json");
    let out = poll_with_sources(repo, 1, Some(&bogus), None, None);
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("not found"));
    assert!(s.contains("eva"));
}

#[test]
fn poll_fails_on_non_json_source_file() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let path = tmp.path().join("bad.json");
    std::fs::write(&path, "{not json").unwrap();

    let out = poll_with_sources(repo, 1, Some(&path), None, None);
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("invalid source file"));
}

#[test]
fn poll_fails_when_source_file_declares_wrong_source() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let path = tmp.path().join("audit-mislabeled.json");
    // File declares source "audit" but is passed as eva.
    write_source_file(&path, "audit", vec![]);

    let out = poll_with_sources(repo, 1, Some(&path), None, None);
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("declares source 'audit'"));
}

#[test]
fn poll_succeeds_when_source_file_has_no_self_identification() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    // Write a source file without a `source` field — should be accepted.
    let path = tmp.path().join("no-source.json");
    let body = serde_json::json!({
        "events": [event("1")]
    });
    std::fs::write(&path, serde_json::to_string_pretty(&body).unwrap()).unwrap();

    let out = poll_with_sources(repo, 1, Some(&path), None, None);
    assert_success(&out);
}

// ----- poll: cursor filtering / advancement -----

#[test]
fn poll_filters_events_at_or_below_cursor() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    // First poll establishes cursor at "200".
    let eva_path = tmp.path().join("eva-src.json");
    write_source_file(&eva_path, "eva", vec![event("100"), event("200")]);
    let out = poll_with_sources(repo, 1, Some(&eva_path), None, None);
    assert_success(&out);

    // Verify cursor is "200".
    let cursor_path = repo
        .join("state")
        .join("reconciler")
        .join("eva-cursor.json");
    let cv: Value =
        serde_json::from_str(&std::fs::read_to_string(&cursor_path).unwrap()).unwrap();
    assert_eq!(cv["value"], json!("200"));

    // Second poll with cycle 2 and the SAME events plus a new "201". Cursor filtering
    // should keep only "201".
    write_super_step_state_reconciler(repo, 2);
    write_source_file(
        &eva_path,
        "eva",
        vec![event("100"), event("200"), event("201")],
    );
    let out = poll_with_sources_json(repo, 2, Some(&eva_path), None, None);
    assert_success(&out);
    let v = stdout_json(&out);
    assert_eq!(v["eva"]["events_in"], json!(3));
    assert_eq!(v["eva"]["events_new"], json!(1));
    assert_eq!(v["eva"]["cursor_before"], json!("200"));
    assert_eq!(v["eva"]["cursor_after"], json!("201"));
}

#[test]
fn poll_with_zero_new_events_leaves_cursor_unchanged() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let eva_path = tmp.path().join("eva-src.json");
    write_source_file(&eva_path, "eva", vec![event("100")]);
    assert_success(&poll_with_sources(repo, 1, Some(&eva_path), None, None));

    // Same events second poll.
    write_super_step_state_reconciler(repo, 2);
    let out = poll_with_sources_json(repo, 2, Some(&eva_path), None, None);
    assert_success(&out);
    let v = stdout_json(&out);
    assert_eq!(v["eva"]["events_new"], json!(0));
    assert_eq!(v["eva"]["cursor_before"], json!("100"));
    assert_eq!(v["eva"]["cursor_after"], json!("100"));
}

#[test]
fn poll_handles_unsorted_events_using_lex_max() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let eva_path = tmp.path().join("eva-src.json");
    write_source_file(
        &eva_path,
        "eva",
        vec![event("aaa"), event("zzz"), event("mmm")],
    );

    let out = poll_with_sources_json(repo, 1, Some(&eva_path), None, None);
    assert_success(&out);
    let v = stdout_json(&out);
    assert_eq!(v["eva"]["cursor_after"], json!("zzz"));
}

// ----- poll: skip-channel-write -----

#[test]
fn poll_skip_channel_write_records_write_skipped_outcome() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let out = run_cmd(
        repo,
        &[
            "poll",
            "--cycle",
            "1",
            "--skip-channel-write",
            "--timestamp",
            "2026-05-14T07:00:00Z",
        ],
    );
    assert_success(&out);

    let history_path = repo
        .join("state")
        .join("reconciler")
        .join("poll-history.json");
    let v: Value =
        serde_json::from_str(&std::fs::read_to_string(history_path).unwrap()).unwrap();
    assert_eq!(v["entries"][0]["outcome"], json!("write-skipped"));
}

#[test]
fn poll_skip_channel_write_leaves_inbound_state_at_sentinel() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let out = run_cmd(
        repo,
        &[
            "poll",
            "--cycle",
            "1",
            "--skip-channel-write",
            "--timestamp",
            "2026-05-14T07:00:00Z",
        ],
    );
    assert_success(&out);

    let state_path = repo
        .join("state")
        .join("channels")
        .join("inbound-channel.json");
    let raw = std::fs::read_to_string(&state_path).unwrap();
    // Should be the null sentinel from init_all, NOT a written envelope.
    assert_eq!(raw.trim(), "null");
}

#[test]
fn poll_skip_channel_write_does_not_advance_cursors() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let eva_path = tmp.path().join("eva-src.json");
    write_source_file(&eva_path, "eva", vec![event("100")]);

    let out = run_cmd(
        repo,
        &[
            "poll",
            "--cycle",
            "1",
            "--eva-source-file",
            eva_path.to_str().unwrap(),
            "--skip-channel-write",
            "--timestamp",
            "2026-05-14T07:00:00Z",
        ],
    );
    assert_success(&out);

    // Cursor should still be null.
    let cursor_path = repo
        .join("state")
        .join("reconciler")
        .join("eva-cursor.json");
    let cv: Value =
        serde_json::from_str(&std::fs::read_to_string(cursor_path).unwrap()).unwrap();
    assert!(cv["value"].is_null());
}

// ----- poll: channel-router not initialized -----

#[test]
fn poll_fails_when_channels_dir_absent() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    // Reconciler init only; do NOT call init_all (no channels dir).
    assert_success(&run_cmd(repo, &["init"]));
    write_super_step_state_reconciler(repo, 1);

    let out = run_cmd(
        repo,
        &[
            "poll",
            "--cycle",
            "1",
            "--timestamp",
            "2026-05-14T07:00:00Z",
        ],
    );
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("channels state not initialized"));
}

// ----- poll: reconciler-dir not initialized -----

#[test]
fn poll_fails_when_reconciler_dir_absent() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    // No init at all.
    let out = run_cmd(
        repo,
        &[
            "poll",
            "--cycle",
            "1",
            "--skip-super-step-check",
            "--timestamp",
            "2026-05-14T07:00:00Z",
        ],
    );
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("not initialized"));
}

// ----- poll: timestamp default -----

#[test]
fn poll_omitting_timestamp_uses_sentinel() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let out = run_cmd(repo, &["poll", "--cycle", "1"]);
    assert_success(&out);

    let history_path = repo
        .join("state")
        .join("reconciler")
        .join("poll-history.json");
    let v: Value =
        serde_json::from_str(&std::fs::read_to_string(history_path).unwrap()).unwrap();
    assert_eq!(v["entries"][0]["at"], json!("1970-01-01T00:00:00Z"));
}

// ----- two-cycle accumulation -----

#[test]
fn two_consecutive_cycles_accumulate_inbound_history() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);

    write_super_step_state_reconciler(repo, 1);
    assert_success(&poll_with_sources(repo, 1, None, None, None));

    write_super_step_state_reconciler(repo, 2);
    assert_success(&poll_with_sources(repo, 2, None, None, None));

    let history_path = repo
        .join("state")
        .join("channels")
        .join("inbound-channel-history.json");
    let v: Value =
        serde_json::from_str(&std::fs::read_to_string(history_path).unwrap()).unwrap();
    assert_eq!(v["entries"].as_array().unwrap().len(), 2);
    assert_eq!(v["entries"][0]["cycle"], json!(1));
    assert_eq!(v["entries"][1]["cycle"], json!(2));
}

#[test]
fn two_consecutive_cycles_accumulate_poll_history() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);

    write_super_step_state_reconciler(repo, 1);
    assert_success(&poll_with_sources(repo, 1, None, None, None));

    write_super_step_state_reconciler(repo, 2);
    assert_success(&poll_with_sources(repo, 2, None, None, None));

    let history_path = repo
        .join("state")
        .join("reconciler")
        .join("poll-history.json");
    let v: Value =
        serde_json::from_str(&std::fs::read_to_string(history_path).unwrap()).unwrap();
    assert_eq!(v["entries"].as_array().unwrap().len(), 2);
}

// ----- cursors subcommand -----

#[test]
fn cursors_after_init_are_all_null() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    assert_success(&run_cmd(repo, &["init"]));

    let out = run_cmd_json(repo, &["cursors"]);
    assert_success(&out);
    let v = stdout_json(&out);
    let cursors = v["cursors"].as_array().unwrap();
    assert_eq!(cursors.len(), 3);
    for c in cursors {
        assert!(c["value"].is_null());
    }
}

#[test]
fn cursors_text_output_lists_three_sources() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    assert_success(&run_cmd(repo, &["init"]));

    let out = run_cmd(repo, &["cursors"]);
    assert_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("eva"));
    assert!(s.contains("audit"));
    assert!(s.contains("dispatch"));
    assert!(s.contains("<unset>"));
}

#[test]
fn cursors_after_poll_reflect_advanced_values() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let eva_path = tmp.path().join("eva-src.json");
    let audit_path = tmp.path().join("audit-src.json");
    write_source_file(&eva_path, "eva", vec![event("100")]);
    write_source_file(&audit_path, "audit", vec![event("abc")]);
    assert_success(&poll_with_sources(repo, 1, Some(&eva_path), Some(&audit_path), None));

    let out = run_cmd_json(repo, &["cursors"]);
    assert_success(&out);
    let v = stdout_json(&out);
    let cursors = v["cursors"].as_array().unwrap();
    let mut by_source = std::collections::HashMap::new();
    for c in cursors {
        by_source.insert(c["source"].as_str().unwrap().to_string(), c["value"].clone());
    }
    assert_eq!(by_source["eva"], json!("100"));
    assert_eq!(by_source["audit"], json!("abc"));
    assert!(by_source["dispatch"].is_null());
}

#[test]
fn cursors_before_init_errors() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    let out = run_cmd(repo, &["cursors"]);
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("not initialized"));
}

// ----- history subcommand -----

#[test]
fn history_after_init_is_empty() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    assert_success(&run_cmd(repo, &["init"]));

    let out = run_cmd(repo, &["history"]);
    assert_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("(no poll history)"));
}

#[test]
fn history_limit_returns_newest_first() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);

    for cycle in 1..=3 {
        write_super_step_state_reconciler(repo, cycle);
        assert_success(&poll_with_sources(repo, cycle, None, None, None));
    }

    let out = run_cmd_json(repo, &["history", "--limit", "2"]);
    assert_success(&out);
    let v = stdout_json(&out);
    let entries = v["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0]["cycle"], json!(3));
    assert_eq!(entries[1]["cycle"], json!(2));
}

#[test]
fn history_before_init_errors() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    let out = run_cmd(repo, &["history"]);
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("not initialized"));
}

#[test]
fn history_with_corrupt_file_produces_clean_error() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);

    // Corrupt the poll-history file.
    let p = repo
        .join("state")
        .join("reconciler")
        .join("poll-history.json");
    std::fs::write(&p, "{not valid").unwrap();

    let out = run_cmd(repo, &["history"]);
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("decoding") || s.contains("json"));
}

// ----- schema subcommand -----

#[test]
fn schema_text_lists_three_sources_and_inbound_binding() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    let out = run_cmd(repo, &["schema"]);
    assert_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("inbound-channel"));
    assert!(s.contains("reconciler"));
    assert!(s.contains("eva-responses"));
    assert!(s.contains("audit-posts"));
    assert!(s.contains("dispatch-returns"));
    assert!(s.contains("inbound-completeness-marker"));
    assert!(s.contains("eva"));
    assert!(s.contains("audit"));
    assert!(s.contains("dispatch"));
    assert!(s.contains("success"));
    assert!(s.contains("write-skipped"));
}

#[test]
fn schema_json_is_well_formed() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    let out = run_cmd_json(repo, &["schema"]);
    assert_success(&out);
    let v = stdout_json(&out);
    assert_eq!(v["inbound_channel"], json!("inbound-channel"));
    assert_eq!(v["inbound_writer"], json!("reconciler"));
    let req = v["inbound_required_keys"].as_array().unwrap();
    assert_eq!(req.len(), 4);
    let req_strs: Vec<&str> = req.iter().map(|s| s.as_str().unwrap()).collect();
    assert!(req_strs.contains(&"eva-responses"));
    assert!(req_strs.contains(&"audit-posts"));
    assert!(req_strs.contains(&"dispatch-returns"));
    assert!(req_strs.contains(&"inbound-completeness-marker"));
    let sources = v["sources"].as_array().unwrap();
    assert_eq!(sources.len(), 3);
}

// ----- inbound subcommand (read-only convenience) -----

#[test]
fn inbound_with_no_state_reports_absence() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    let out = run_cmd(repo, &["inbound"]);
    assert_success(&out);
    let s = stdout_str(&out);
    assert!(s.contains("no state"));
}

#[test]
fn inbound_after_poll_reads_state_for_current_cycle() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);
    assert_success(&poll_with_sources(repo, 1, None, None, None));

    let out = run_cmd_json(repo, &["inbound", "--cycle", "1"]);
    assert_success(&out);
    let v = stdout_json(&out);
    assert_eq!(v["matched"], json!(true));
    assert_eq!(v["state"]["cycle"], json!(1));
    assert_eq!(v["state"]["writer"], json!("reconciler"));
}

#[test]
fn inbound_with_wrong_cycle_returns_unmatched() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);
    assert_success(&poll_with_sources(repo, 1, None, None, None));

    let out = run_cmd_json(repo, &["inbound", "--cycle", "999"]);
    assert_success(&out);
    let v = stdout_json(&out);
    assert_eq!(v["matched"], json!(false));
    assert!(v["state"].is_null());
}

#[test]
fn inbound_without_cycle_filter_returns_latest() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);
    assert_success(&poll_with_sources(repo, 1, None, None, None));

    let out = run_cmd_json(repo, &["inbound"]);
    assert_success(&out);
    let v = stdout_json(&out);
    assert_eq!(v["matched"], json!(true));
    assert_eq!(v["state"]["cycle"], json!(1));
}

// ----- cursor file corruption -----

#[test]
fn poll_fails_cleanly_with_corrupt_cursor_file() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let cursor_path = repo
        .join("state")
        .join("reconciler")
        .join("eva-cursor.json");
    std::fs::write(&cursor_path, "{not valid").unwrap();

    let out = poll_with_sources(repo, 1, None, None, None);
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("cursor file") || s.contains("could not be parsed"));
}

#[test]
fn poll_fails_cleanly_when_cursor_declares_wrong_source() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    // Replace eva-cursor with one declaring source = "audit".
    let cursor_path = repo
        .join("state")
        .join("reconciler")
        .join("eva-cursor.json");
    let bad = serde_json::json!({"source": "audit", "value": null});
    std::fs::write(&cursor_path, serde_json::to_string(&bad).unwrap()).unwrap();

    let out = poll_with_sources(repo, 1, None, None, None);
    assert_failure(&out);
    let s = stderr_str(&out);
    assert!(s.contains("declares source"));
}

// ----- on-disk state file shape -----

#[test]
fn inbound_state_envelope_matches_v2_channel_router_shape() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    assert_success(&poll_with_sources(repo, 1, None, None, None));

    let p = repo
        .join("state")
        .join("channels")
        .join("inbound-channel.json");
    let v: Value = serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap();
    // Expected fields: channel, writer, cycle, timestamp, payload.
    for k in &["channel", "writer", "cycle", "timestamp", "payload"] {
        assert!(v.get(k).is_some(), "missing field '{k}' in envelope: {v}");
    }
}

#[test]
fn poll_history_records_summary_correctly() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let eva_path = tmp.path().join("eva-src.json");
    write_source_file(&eva_path, "eva", vec![event("100"), event("200")]);

    assert_success(&poll_with_sources(repo, 1, Some(&eva_path), None, None));

    let history_path = repo
        .join("state")
        .join("reconciler")
        .join("poll-history.json");
    let v: Value =
        serde_json::from_str(&std::fs::read_to_string(history_path).unwrap()).unwrap();
    let entry = &v["entries"][0];
    assert_eq!(entry["eva"]["events_in"], json!(2));
    assert_eq!(entry["eva"]["events_new"], json!(2));
    assert!(entry["eva"]["cursor_before"].is_null());
    assert_eq!(entry["eva"]["cursor_after"], json!("200"));
    assert_eq!(entry["audit"]["events_in"], json!(0));
    assert_eq!(entry["dispatch"]["events_in"], json!(0));
}

// ----- raw event payload pass-through -----

#[test]
fn events_with_raw_payload_pass_through_into_inbound() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let eva_path = tmp.path().join("eva-src.json");
    let evt = json!({
        "id": "100",
        "at": "2026-05-14T07:00:00Z",
        "raw": {
            "author": "EvaLok",
            "title": "test issue"
        }
    });
    write_source_file(&eva_path, "eva", vec![evt]);

    assert_success(&poll_with_sources(repo, 1, Some(&eva_path), None, None));

    let p = repo
        .join("state")
        .join("channels")
        .join("inbound-channel.json");
    let v: Value = serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap();
    let eva_arr = v["payload"]["eva-responses"].as_array().unwrap();
    assert_eq!(eva_arr.len(), 1);
    assert_eq!(eva_arr[0]["raw"]["author"], json!("EvaLok"));
}

#[test]
fn events_missing_raw_default_to_null() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);
    write_super_step_state_reconciler(repo, 1);

    let eva_path = tmp.path().join("eva-src.json");
    let evt = json!({
        "id": "100",
        "at": "2026-05-14T07:00:00Z"
        // no `raw` field
    });
    write_source_file(&eva_path, "eva", vec![evt]);

    assert_success(&poll_with_sources(repo, 1, Some(&eva_path), None, None));

    let p = repo
        .join("state")
        .join("channels")
        .join("inbound-channel.json");
    let v: Value = serde_json::from_str(&std::fs::read_to_string(p).unwrap()).unwrap();
    let eva_arr = v["payload"]["eva-responses"].as_array().unwrap();
    assert!(eva_arr[0]["raw"].is_null());
}

// ----- super-step state with extra fields (forward-compat) -----

#[test]
fn poll_tolerates_extra_fields_in_super_step_state() {
    let tmp = TempDir::new().unwrap();
    let repo = tmp.path();
    init_all(repo);

    // Write a super-step state with extra fields not in SuperStepStateLite.
    let p = repo.join("state").join("super-step.json");
    let payload = serde_json::json!({
        "cycle": 1,
        "current_role": "reconciler",
        "future_field_we_dont_know": "value",
        "another": [1, 2, 3]
    });
    std::fs::write(&p, serde_json::to_string_pretty(&payload).unwrap()).unwrap();

    let out = poll_with_sources(repo, 1, None, None, None);
    assert_success(&out);
}
