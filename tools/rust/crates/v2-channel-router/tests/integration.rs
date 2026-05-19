use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;
use tempfile::TempDir;

/// Path to the compiled v2-channel-router binary under cargo's target tree.
fn bin() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    // Ascend out of crates/v2-channel-router to the workspace target/debug.
    p.pop(); // v2-channel-router → crates/
    p.pop(); // crates/ → tools/rust/
    p.push("target");
    p.push("debug");
    p.push("v2-channel-router");
    p
}

fn write_payload_file(dir: &Path, name: &str, body: &str) -> PathBuf {
    let p = dir.join(name);
    fs::write(&p, body).expect("write payload file");
    p
}

fn run(args: &[&str], repo_root: &Path) -> (String, String, i32) {
    let output = Command::new(bin())
        .args(args)
        .arg("--repo-root")
        .arg(repo_root)
        .output()
        .expect("run v2-channel-router");
    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    (stdout, stderr, output.status.code().unwrap_or(-1))
}

fn run_json(args: &[&str], repo_root: &Path) -> (Value, String, i32) {
    let mut a = vec!["--format", "json"];
    a.extend_from_slice(args);
    let (stdout, stderr, code) = run(&a, repo_root);
    let parsed = if stdout.trim().is_empty() {
        Value::Null
    } else {
        serde_json::from_str(&stdout).unwrap_or_else(|e| {
            panic!("could not parse stdout as JSON: {e}\nstdout: {stdout}\nstderr: {stderr}")
        })
    };
    (parsed, stderr, code)
}

#[test]
fn init_creates_empty_state_files_for_each_channel() {
    let tmp = TempDir::new().unwrap();
    let (stdout, _stderr, code) = run(&["init"], tmp.path());
    assert_eq!(code, 0, "init exited non-zero: {stdout}");

    for ch in [
        "plan-channel",
        "work-channel",
        "memory-channel",
        "inbound-channel",
    ] {
        let state_p = tmp.path().join("state/channels").join(format!("{ch}.json"));
        let hist_p = tmp
            .path()
            .join("state/channels")
            .join(format!("{ch}-history.json"));
        assert!(state_p.exists(), "missing state file {state_p:?}");
        assert!(hist_p.exists(), "missing history file {hist_p:?}");
        let state_raw = fs::read_to_string(&state_p).unwrap();
        assert_eq!(state_raw.trim(), "null");
        let hist_raw = fs::read_to_string(&hist_p).unwrap();
        let hist: Value = serde_json::from_str(&hist_raw).unwrap();
        assert_eq!(hist["channel"].as_str().unwrap(), ch);
        assert!(hist["entries"].as_array().unwrap().is_empty());
    }
}

#[test]
fn init_is_idempotent() {
    let tmp = TempDir::new().unwrap();
    let (_o1, _e1, c1) = run(&["init"], tmp.path());
    assert_eq!(c1, 0);
    let (_o2, _e2, c2) = run(&["init"], tmp.path());
    assert_eq!(c2, 0);
}

#[test]
fn init_json_output_lists_created_and_already_present() {
    let tmp = TempDir::new().unwrap();
    let (v1, _e1, c1) = run_json(&["init"], tmp.path());
    assert_eq!(c1, 0);
    assert_eq!(v1["created"].as_array().unwrap().len(), 8); // 4 channels × 2 files
    assert!(v1["already_present"].as_array().unwrap().is_empty());

    let (v2, _e2, c2) = run_json(&["init"], tmp.path());
    assert_eq!(c2, 0);
    assert!(v2["created"].as_array().unwrap().is_empty());
    assert_eq!(v2["already_present"].as_array().unwrap().len(), 8);
}

fn plan_payload_body(cycle: u32) -> String {
    serde_json::json!({
        "cycle": cycle,
        "timestamp": format!("2026-05-14T0{cycle}:00:00Z"),
        "payload": {
            "substantive-focal": format!("focal-{cycle}"),
            "per-role-tasks": {
                "executor": {
                    "action": format!("executor-task-{cycle}")
                },
                "curator": {
                    "action": format!("curator-task-{cycle}")
                },
                "reconciler": {
                    "action": format!("reconciler-task-{cycle}")
                }
            }
        }
    })
    .to_string()
}

fn work_payload_body(cycle: u32) -> String {
    serde_json::json!({
        "cycle": cycle,
        "timestamp": format!("2026-05-14T0{cycle}:30:00Z"),
        "payload": {
            "artifacts-written": [format!("artifact-{cycle}.md")]
        }
    })
    .to_string()
}

fn memory_payload_body(cycle: u32) -> String {
    serde_json::json!({
        "cycle": cycle,
        "timestamp": format!("2026-05-14T0{cycle}:45:00Z"),
        "payload": {
            "consolidated-insights": [format!("insight-{cycle}")],
            "anti-patterns-noted": []
        }
    })
    .to_string()
}

fn inbound_payload_body(cycle: u32) -> String {
    serde_json::json!({
        "cycle": cycle,
        "timestamp": format!("2026-05-14T0{cycle}:50:00Z"),
        "payload": {
            "eva-responses": [],
            "audit-posts": [],
            "dispatch-returns": [],
            "inbound-completeness-marker": "quiet"
        }
    })
    .to_string()
}

#[test]
fn write_plan_channel_with_planner_succeeds() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let payload = write_payload_file(tmp.path(), "plan.json", &plan_payload_body(140));
    let (v, stderr, code) = run_json(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_eq!(code, 0, "stderr: {stderr}");
    assert_eq!(v["channel"].as_str().unwrap(), "plan-channel");
    assert_eq!(v["writer"].as_str().unwrap(), "planner");
    assert_eq!(v["cycle"].as_u64().unwrap(), 140);
    assert_eq!(v["history_entries"].as_u64().unwrap(), 1);
}

#[test]
fn write_plan_channel_with_executor_rejects() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let payload = write_payload_file(tmp.path(), "plan.json", &plan_payload_body(140));
    let (_v, stderr, code) = run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "executor",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_ne!(code, 0, "expected non-zero exit");
    assert!(
        stderr.contains("reducer violation"),
        "stderr did not name reducer violation: {stderr}"
    );
    assert!(
        stderr.contains("plan-channel"),
        "stderr did not name channel: {stderr}"
    );
    assert!(
        stderr.contains("planner"),
        "stderr did not name allowed writer: {stderr}"
    );
}

#[test]
fn write_work_channel_with_planner_rejects() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let payload = write_payload_file(tmp.path(), "work.json", &work_payload_body(140));
    let (_v, stderr, code) = run(
        &[
            "write",
            "--channel",
            "work-channel",
            "--writer",
            "planner",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_ne!(code, 0);
    assert!(stderr.contains("reducer violation"));
    assert!(stderr.contains("executor"));
}

type PayloadFn = fn(u32) -> String;
type ChannelCase = (&'static str, &'static str, PayloadFn);

#[test]
fn all_four_channels_writable_with_correct_writer() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let cases: &[ChannelCase] = &[
        ("plan-channel", "planner", plan_payload_body),
        ("work-channel", "executor", work_payload_body),
        ("memory-channel", "curator", memory_payload_body),
        ("inbound-channel", "reconciler", inbound_payload_body),
    ];
    for (channel, writer, body_fn) in cases {
        let payload = write_payload_file(tmp.path(), &format!("{channel}.json"), &body_fn(140));
        let (_v, stderr, code) = run_json(
            &[
                "write",
                "--channel",
                channel,
                "--writer",
                writer,
                "--payload-file",
                payload.to_str().unwrap(),
            ],
            tmp.path(),
        );
        assert_eq!(code, 0, "{channel} write failed: stderr={stderr}");
    }
}

#[test]
fn write_rejects_missing_required_payload_keys() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let bad_body = serde_json::json!({
        "cycle": 140,
        "timestamp": "2026-05-14T01:00:00Z",
        "payload": { "substantive-focal": "x" }  // missing per-role-tasks
    })
    .to_string();
    let payload = write_payload_file(tmp.path(), "bad.json", &bad_body);
    let (_v, stderr, code) = run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_ne!(code, 0);
    assert!(stderr.contains("invalid payload"));
    assert!(stderr.contains("per-role-tasks"));
}

#[test]
fn write_rejects_non_object_payload() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let bad_body = serde_json::json!({
        "cycle": 140,
        "timestamp": "2026-05-14T01:00:00Z",
        "payload": "not an object"
    })
    .to_string();
    let payload = write_payload_file(tmp.path(), "bad.json", &bad_body);
    let (_v, stderr, code) = run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_ne!(code, 0);
    assert!(stderr.contains("invalid payload"));
    assert!(
        stderr.contains("must be a JSON object"),
        "stderr did not name the type constraint: {stderr}"
    );
}

#[test]
fn write_rejects_missing_payload_file() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let (_v, stderr, code) = run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            tmp.path().join("does-not-exist.json").to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_ne!(code, 0);
    assert!(stderr.contains("payload file not found"));
}

#[test]
fn read_before_init_reports_not_initialized() {
    let tmp = TempDir::new().unwrap();
    let (_v, stderr, code) = run(&["read", "--channel", "plan-channel"], tmp.path());
    assert_ne!(code, 0);
    assert!(stderr.contains("not initialized"), "stderr: {stderr}");
}

#[test]
fn read_after_init_returns_empty_state() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let (v, _stderr, code) = run_json(&["read", "--channel", "plan-channel"], tmp.path());
    assert_eq!(code, 0);
    assert_eq!(v["channel"].as_str().unwrap(), "plan-channel");
    assert!(v["state"].is_null());
}

#[test]
fn read_after_write_returns_latest_state() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let payload = write_payload_file(tmp.path(), "plan.json", &plan_payload_body(140));
    run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    let (v, _stderr, code) = run_json(&["read", "--channel", "plan-channel"], tmp.path());
    assert_eq!(code, 0);
    let state = &v["state"];
    assert!(!state.is_null());
    assert_eq!(state["cycle"].as_u64().unwrap(), 140);
    assert_eq!(state["writer"].as_str().unwrap(), "planner");
    assert_eq!(
        state["payload"]["substantive-focal"].as_str().unwrap(),
        "focal-140"
    );
}

#[test]
fn read_text_output_for_empty_state_is_human_friendly() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let (stdout, _stderr, code) = run(&["read", "--channel", "plan-channel"], tmp.path());
    assert_eq!(code, 0);
    assert!(stdout.contains("empty (never written)"), "stdout: {stdout}");
}

#[test]
fn read_text_output_after_write_shows_envelope_fields() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let payload = write_payload_file(tmp.path(), "plan.json", &plan_payload_body(140));
    run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    let (stdout, _stderr, code) = run(&["read", "--channel", "plan-channel"], tmp.path());
    assert_eq!(code, 0);
    for token in [
        "channel:",
        "plan-channel",
        "writer:",
        "planner",
        "cycle:",
        "140",
        "payload:",
    ] {
        assert!(stdout.contains(token), "missing token '{token}': {stdout}");
    }
}

#[test]
fn multiple_writes_to_same_channel_accumulate_in_history() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    for cycle in [140u32, 141, 142] {
        let payload = write_payload_file(
            tmp.path(),
            &format!("plan-{cycle}.json"),
            &plan_payload_body(cycle),
        );
        let (v, stderr, code) = run_json(
            &[
                "write",
                "--channel",
                "plan-channel",
                "--writer",
                "planner",
                "--payload-file",
                payload.to_str().unwrap(),
            ],
            tmp.path(),
        );
        assert_eq!(code, 0, "cycle {cycle} write failed: {stderr}");
        assert_eq!(v["cycle"].as_u64().unwrap(), cycle as u64);
    }
    let (v, _stderr, code) = run_json(&["history", "--channel", "plan-channel"], tmp.path());
    assert_eq!(code, 0);
    assert_eq!(v["total"].as_u64().unwrap(), 3);
    assert_eq!(v["returned"].as_u64().unwrap(), 3);
    let entries = v["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 3);
    assert_eq!(entries[0]["cycle"].as_u64().unwrap(), 140);
    assert_eq!(entries[1]["cycle"].as_u64().unwrap(), 141);
    assert_eq!(entries[2]["cycle"].as_u64().unwrap(), 142);
}

#[test]
fn history_limit_returns_newest_first_capped() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    for cycle in [140u32, 141, 142, 143, 144] {
        let payload = write_payload_file(
            tmp.path(),
            &format!("plan-{cycle}.json"),
            &plan_payload_body(cycle),
        );
        run(
            &[
                "write",
                "--channel",
                "plan-channel",
                "--writer",
                "planner",
                "--payload-file",
                payload.to_str().unwrap(),
            ],
            tmp.path(),
        );
    }
    let (v, _stderr, code) = run_json(
        &["history", "--channel", "plan-channel", "--limit", "2"],
        tmp.path(),
    );
    assert_eq!(code, 0);
    assert_eq!(v["total"].as_u64().unwrap(), 5);
    assert_eq!(v["returned"].as_u64().unwrap(), 2);
    let entries = v["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 2);
    // newest first
    assert_eq!(entries[0]["cycle"].as_u64().unwrap(), 144);
    assert_eq!(entries[1]["cycle"].as_u64().unwrap(), 143);
}

#[test]
fn history_before_init_reports_not_initialized() {
    let tmp = TempDir::new().unwrap();
    let (_v, stderr, code) = run(&["history", "--channel", "plan-channel"], tmp.path());
    assert_ne!(code, 0);
    assert!(stderr.contains("not initialized"));
}

#[test]
fn schema_all_channels_default() {
    let tmp = TempDir::new().unwrap();
    let (v, _stderr, code) = run_json(&["schema"], tmp.path());
    assert_eq!(code, 0);
    assert_eq!(v["schema_format_version"].as_u64().unwrap(), 2);
    let arr = v["channels"].as_array().unwrap();
    assert_eq!(arr.len(), 4);
    let names: Vec<&str> = arr.iter().map(|s| s["name"].as_str().unwrap()).collect();
    assert!(names.contains(&"plan-channel"));
    assert!(names.contains(&"work-channel"));
    assert!(names.contains(&"memory-channel"));
    assert!(names.contains(&"inbound-channel"));
    let plan = arr.iter().find(|s| s["name"] == "plan-channel").unwrap();
    assert_eq!(plan["allowed_writer"].as_str().unwrap(), "planner");
    assert_eq!(
        plan["state_path_template"].as_str().unwrap(),
        "state/channels/plan-channel.json"
    );
}

#[test]
fn schema_single_channel_selection() {
    let tmp = TempDir::new().unwrap();
    let (v, _stderr, code) = run_json(&["schema", "--channel", "inbound-channel"], tmp.path());
    assert_eq!(code, 0);
    let arr = v["channels"].as_array().unwrap();
    assert_eq!(arr.len(), 1);
    assert_eq!(arr[0]["name"].as_str().unwrap(), "inbound-channel");
    assert_eq!(arr[0]["allowed_writer"].as_str().unwrap(), "reconciler");
}

#[test]
fn write_persists_state_file_on_disk_as_expected_shape() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let payload = write_payload_file(tmp.path(), "plan.json", &plan_payload_body(140));
    run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    let state_path = tmp.path().join("state/channels/plan-channel.json");
    let raw = fs::read_to_string(&state_path).unwrap();
    let v: Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v["channel"].as_str().unwrap(), "plan-channel");
    assert_eq!(v["writer"].as_str().unwrap(), "planner");
    assert_eq!(v["cycle"].as_u64().unwrap(), 140);
    assert!(v["payload"].is_object());
}

#[test]
fn write_appends_history_file_on_disk_with_entry_envelope() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let payload = write_payload_file(tmp.path(), "plan.json", &plan_payload_body(140));
    run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    let hist_path = tmp.path().join("state/channels/plan-channel-history.json");
    let raw = fs::read_to_string(&hist_path).unwrap();
    let v: Value = serde_json::from_str(&raw).unwrap();
    assert_eq!(v["channel"].as_str().unwrap(), "plan-channel");
    let entries = v["entries"].as_array().unwrap();
    assert_eq!(entries.len(), 1);
    assert_eq!(entries[0]["cycle"].as_u64().unwrap(), 140);
    assert_eq!(entries[0]["writer"].as_str().unwrap(), "planner");
    assert!(entries[0]["payload"].is_object());
}

#[test]
fn distinct_channels_have_independent_state_and_history() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());

    let p1 = write_payload_file(tmp.path(), "plan.json", &plan_payload_body(140));
    run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            p1.to_str().unwrap(),
        ],
        tmp.path(),
    );
    let p2 = write_payload_file(tmp.path(), "work.json", &work_payload_body(140));
    run(
        &[
            "write",
            "--channel",
            "work-channel",
            "--writer",
            "executor",
            "--payload-file",
            p2.to_str().unwrap(),
        ],
        tmp.path(),
    );

    let (plan_state, _e, _c) = run_json(&["read", "--channel", "plan-channel"], tmp.path());
    let (work_state, _e, _c) = run_json(&["read", "--channel", "work-channel"], tmp.path());
    let (mem_state, _e, _c) = run_json(&["read", "--channel", "memory-channel"], tmp.path());

    assert_eq!(
        plan_state["state"]["payload"]["substantive-focal"]
            .as_str()
            .unwrap(),
        "focal-140"
    );
    assert_eq!(
        work_state["state"]["payload"]["artifacts-written"][0]
            .as_str()
            .unwrap(),
        "artifact-140.md"
    );
    assert!(
        mem_state["state"].is_null(),
        "memory-channel should be untouched"
    );
}

#[test]
fn reducer_violation_does_not_mutate_state_or_history() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());

    // Seed plan-channel with a valid write.
    let good = write_payload_file(tmp.path(), "plan.json", &plan_payload_body(140));
    run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            good.to_str().unwrap(),
        ],
        tmp.path(),
    );

    // Attempt a wrong-writer write at cycle 141.
    let bad = write_payload_file(tmp.path(), "bad.json", &plan_payload_body(141));
    let (_v, _stderr, code) = run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "curator",
            "--payload-file",
            bad.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_ne!(code, 0);

    // State should still reflect cycle 140; history should still have 1 entry.
    let (state, _e, _c) = run_json(&["read", "--channel", "plan-channel"], tmp.path());
    assert_eq!(state["state"]["cycle"].as_u64().unwrap(), 140);

    let (hist, _e, _c) = run_json(&["history", "--channel", "plan-channel"], tmp.path());
    assert_eq!(hist["total"].as_u64().unwrap(), 1);
}

#[test]
fn write_with_corrupt_payload_file_fails_cleanly() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let payload = write_payload_file(tmp.path(), "bad.json", "not valid json");
    let (_v, stderr, code) = run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_ne!(code, 0);
    assert!(
        stderr.contains("json error") || stderr.contains("decoding"),
        "stderr: {stderr}"
    );
}

#[test]
fn schema_text_output_lists_each_channel_block() {
    let tmp = TempDir::new().unwrap();
    let (stdout, _stderr, code) = run(&["schema"], tmp.path());
    assert_eq!(code, 0);
    assert!(stdout.contains("schema_format_version: 2"));
    for ch in [
        "plan-channel",
        "work-channel",
        "memory-channel",
        "inbound-channel",
    ] {
        assert!(stdout.contains(ch), "stdout missing '{ch}': {stdout}");
    }
    for role in ["planner", "executor", "curator", "reconciler"] {
        assert!(
            stdout.contains(role),
            "stdout missing role '{role}': {stdout}"
        );
    }
}

#[test]
fn validate_payload_strict_mode_default_rejects_type_mismatch() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let bad_body = serde_json::json!({
        "cycle": 140,
        "timestamp": "2026-05-14T01:00:00Z",
        "payload": {
            "substantive-focal": "x",
            "per-role-tasks": "wrong"
        }
    })
    .to_string();
    let payload = write_payload_file(tmp.path(), "bad.json", &bad_body);
    let (_stdout, stderr, code) = run(
        &[
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_ne!(code, 0);
    assert!(stderr.contains("payload key 'per-role-tasks': expected type object, observed string"));
}

#[test]
fn validate_payload_lenient_mode_logs_and_accepts() {
    let tmp = TempDir::new().unwrap();
    run(&["init"], tmp.path());
    let bad_body = serde_json::json!({
        "cycle": 140,
        "timestamp": "2026-05-14T01:00:00Z",
        "payload": {
            "substantive-focal": "x",
            "per-role-tasks": "wrong"
        }
    })
    .to_string();
    let payload = write_payload_file(tmp.path(), "bad.json", &bad_body);
    let (_stdout, stderr, code) = run(
        &[
            "--mode",
            "lenient",
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "planner",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_eq!(code, 0, "stderr: {stderr}");
    assert!(stderr.contains("[router-lenient]"));
    assert!(stderr.contains("payload key 'per-role-tasks'"));
}

#[test]
fn schema_subcommand_emits_format_version_2() {
    let tmp = TempDir::new().unwrap();
    let (v, _stderr, code) = run_json(&["schema"], tmp.path());
    assert_eq!(code, 0);
    assert_eq!(v["schema_format_version"].as_u64().unwrap(), 2);
}

// ----- Cycle 183: envelope emit-side end-to-end (design §7.1 cycle 2) ------

#[test]
fn error_format_json_emits_envelope_for_not_initialized_read() {
    // Acceptance criterion: invoke router with --error-format=json on a
    // deliberately-erroring input (here: read against a non-initialized
    // state dir), parse the envelope from stderr's last line, assert class +
    // key details land.
    let tmp = TempDir::new().unwrap();
    let (_stdout, stderr, code) = run(
        &[
            "--error-format",
            "json",
            "read",
            "--channel",
            "plan-channel",
        ],
        tmp.path(),
    );
    assert_ne!(code, 0, "must exit non-zero on uninitialized read");

    let envelope = v2_error_envelope::parse_from_stderr_tail(&stderr)
        .unwrap_or_else(|| panic!("expected envelope on stderr; got: {stderr}"));
    assert_eq!(envelope.primitive, "v2-channel-router");
    assert_eq!(envelope.class, v2_error_envelope::ErrorClass::Config);
    assert_eq!(
        envelope.details.get("key").and_then(|v| v.as_str()),
        Some("channels_dir"),
    );
    assert_eq!(
        envelope.details.get("hint").and_then(|v| v.as_str()),
        Some("run `v2-channel-router init`"),
    );
    // human field is the legacy Display text — preserved so text mode and
    // JSON mode carry the same operator-visible message.
    assert!(
        envelope.human.contains("channel state directory not initialized"),
        "human field should mirror Display impl: {}",
        envelope.human
    );
}

#[test]
fn error_format_text_still_emits_legacy_line_by_default() {
    // Backward-compat: omitting --error-format must preserve the legacy
    // `v2-channel-router: <message>` stderr line. The cycle 4 caller-side
    // migration is what flips invocations to json; this cycle's emit-side
    // change must not silently break operators reading text stderr today.
    let tmp = TempDir::new().unwrap();
    let (_stdout, stderr, code) = run(
        &["read", "--channel", "plan-channel"],
        tmp.path(),
    );
    assert_ne!(code, 0);
    assert!(
        stderr.starts_with("v2-channel-router: "),
        "text-mode stderr must keep legacy prefix: {stderr:?}",
    );
    assert!(
        v2_error_envelope::parse_from_stderr_tail(&stderr).is_none(),
        "text-mode stderr must NOT parse as an envelope: {stderr:?}",
    );
}

#[test]
fn error_format_json_with_lenient_mode_pre_envelope_warning_still_parses() {
    // Pin design §6.2 / OQ-SEE-2: when stderr already contains pre-envelope
    // warning lines (lenient-mode validation warnings), the envelope on the
    // last non-empty line of stderr must still be recoverable via
    // parse_from_stderr_tail.
    //
    // We trigger this by:
    //   1. init
    //   2. write a plan-channel payload that is mostly valid but missing a
    //      sub-key (`per-role-tasks` deep schema mismatch) — lenient-mode
    //      logs a `[router-lenient]` warning then succeeds.
    //   3. read with --error-format=json, --writer Executor, payload-file
    //      that triggers a strict-mode validation failure. Since we want the
    //      lenient-warning + envelope co-occurrence, we exercise via the
    //      reducer-violation path which both validates AND rejects writers
    //      that don't own the channel — and use lenient mode to attach a
    //      warning preamble.
    //
    // Simpler shape: trigger a reducer violation in lenient mode. That
    // emits the lenient warning then surfaces the ReducerViolation as a
    // hard error (lenient is for payload validation, not for ownership).
    let tmp = TempDir::new().unwrap();
    let (_o, _e, c) = run(&["init"], tmp.path());
    assert_eq!(c, 0);

    // Write a well-formed plan-channel payload but declare writer=executor
    // (only planner is allowed). Use lenient mode to introduce a
    // pre-envelope warning line into stderr.
    let body = serde_json::json!({
        "cycle": 1,
        "timestamp": "2026-05-19T09:00:00Z",
        "payload": {
            "substantive-focal": "exercise pre-envelope warning",
            "per-role-tasks": {
                "executor": {"action": "x"},
                "curator": {"action": "x"},
                "reconciler": {"action": "x"}
            },
            "unexpected-extra-key": "lenient warns on this"
        }
    })
    .to_string();
    let payload = write_payload_file(tmp.path(), "violation.json", &body);
    let (_stdout, stderr, code) = run(
        &[
            "--error-format",
            "json",
            "--mode",
            "lenient",
            "write",
            "--channel",
            "plan-channel",
            "--writer",
            "executor",
            "--payload-file",
            payload.to_str().unwrap(),
        ],
        tmp.path(),
    );
    assert_ne!(code, 0, "reducer-violation must exit non-zero");

    // Envelope must be recoverable as the last non-empty line.
    let envelope = v2_error_envelope::parse_from_stderr_tail(&stderr)
        .unwrap_or_else(|| panic!("expected envelope on stderr; got: {stderr}"));
    assert_eq!(envelope.class, v2_error_envelope::ErrorClass::ChannelWriteRejected);
    assert_eq!(
        envelope.details.get("channel").and_then(|v| v.as_str()),
        Some("plan-channel"),
    );
    assert_eq!(
        envelope.details.get("writer").and_then(|v| v.as_str()),
        Some("executor"),
    );
    assert_eq!(
        envelope.details.get("allowed_writer").and_then(|v| v.as_str()),
        Some("planner"),
    );
}
