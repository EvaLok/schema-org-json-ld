use std::fs;
use std::path::Path;
use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_v2-boot-phase")
}

fn write_cycle(state_dir: &Path, n: u64, model: &str, started_at: &str) {
    let path = state_dir.join(format!("{n}.json"));
    let content = format!(
        r#"{{"cycle_number": {n}, "model": "{model}", "started_at": "{started_at}"}}"#
    );
    fs::write(path, content).unwrap();
}

fn run(repo_root: &Path, args: &[&str]) -> (i32, String, String) {
    let mut cmd = Command::new(bin());
    cmd.arg("--repo-root").arg(repo_root);
    cmd.args(args);
    let out = cmd.output().unwrap();
    let code = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    (code, stdout, stderr)
}

#[test]
fn empty_state_dir_warn_treated_as_first_cycle() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("cycles_loaded"),
        Some(&serde_json::Value::Number(0.into()))
    );
    assert!(parsed
        .get("cycle_context")
        .unwrap()
        .get("current_cycle")
        .unwrap()
        .is_null());
}

#[test]
fn missing_state_dir_warn_not_fail() {
    let tmp = tempfile::tempdir().unwrap();
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let load = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "load-cycle-history")
        .unwrap();
    assert_eq!(load.get("status").unwrap(), "warn");
}

#[test]
fn missing_state_dir_strict_exits_one() {
    let tmp = tempfile::tempdir().unwrap();
    let (code, _stdout, _err) = run(tmp.path(), &["--strict", "--format", "json"]);
    assert_eq!(code, 1);
}

#[test]
fn state_path_is_file_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let state_parent = tmp.path().join("state");
    fs::create_dir_all(&state_parent).unwrap();
    fs::write(state_parent.join("cycle-history"), "not a dir").unwrap();
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 1);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let load = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "load-cycle-history")
        .unwrap();
    assert_eq!(load.get("status").unwrap(), "failed");
}

#[test]
fn single_entry_skips_gap_detection_and_advances_cursor() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 42, "claude-opus-4-7", "2026-05-11T22:30:00Z");
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("current_cycle"),
        Some(&serde_json::Value::Number(43.into()))
    );
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("previous_cycle"),
        Some(&serde_json::Value::Number(42.into()))
    );
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let gap = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "detect-gaps")
        .unwrap();
    assert_eq!(gap.get("status").unwrap(), "skipped");
}

#[test]
fn contiguous_entries_no_gaps() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    for n in 100..=103 {
        write_cycle(&state, n, "claude-opus-4-7", "2026-05-11T22:30:00Z");
    }
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("current_cycle"),
        Some(&serde_json::Value::Number(104.into()))
    );
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("cycles_loaded"),
        Some(&serde_json::Value::Number(4.into()))
    );
    let gaps = parsed
        .get("cycle_context")
        .unwrap()
        .get("gaps")
        .unwrap()
        .as_array()
        .unwrap();
    assert!(gaps.is_empty());
}

#[test]
fn gap_detected_warn_status() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    write_cycle(&state, 2, "m", "2026-05-11T22:30:00Z");
    write_cycle(&state, 5, "m", "2026-05-11T22:30:00Z");
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let gaps = parsed
        .get("cycle_context")
        .unwrap()
        .get("gaps")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(gaps.len(), 1);
    assert_eq!(
        gaps[0].get("missing_count"),
        Some(&serde_json::Value::Number(2.into()))
    );
}

#[test]
fn gap_strict_mode_exits_one() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    write_cycle(&state, 3, "m", "2026-05-11T22:30:00Z");
    let (code, _stdout, _err) = run(tmp.path(), &["--strict", "--format", "json"]);
    assert_eq!(code, 1);
}

#[test]
fn malformed_entry_warn_not_fail() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    // malformed: missing cycle_number
    fs::write(
        state.join("2.json"),
        r#"{"model": "m", "started_at": "2026-05-11T22:30:00Z"}"#,
    )
    .unwrap();
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let load = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "load-cycle-history")
        .unwrap();
    assert_eq!(load.get("status").unwrap(), "warn");
    let details = load.get("details").unwrap().as_str().unwrap();
    assert!(details.contains("malformed"));
}

#[test]
fn non_numeric_filenames_ignored() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    fs::write(state.join("README.json"), "not-an-entry").unwrap();
    fs::write(state.join("notes.md"), "ignored").unwrap();
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("cycles_loaded"),
        Some(&serde_json::Value::Number(1.into()))
    );
}

#[test]
fn previous_cycle_summary_populated() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 122, "claude-opus-4-7", "2026-05-11T20:31:00Z");
    let (_code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let summary = parsed
        .get("cycle_context")
        .unwrap()
        .get("previous_cycle_summary")
        .unwrap();
    assert_eq!(
        summary.get("cycle_number"),
        Some(&serde_json::Value::Number(122.into()))
    );
    assert_eq!(
        summary.get("model"),
        Some(&serde_json::Value::String("claude-opus-4-7".into()))
    );
    assert_eq!(
        summary.get("started_at"),
        Some(&serde_json::Value::String("2026-05-11T20:31:00Z".into()))
    );
}

#[test]
fn standing_directives_and_gardening_marked_deferred() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    let (_code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let directives = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "check-standing-directives")
        .unwrap();
    assert_eq!(directives.get("status").unwrap(), "deferred");
    let gardening = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "identify-gardening-candidates")
        .unwrap();
    assert_eq!(gardening.get("status").unwrap(), "deferred");
    let standing_in_ctx = parsed
        .get("cycle_context")
        .unwrap()
        .get("standing_directives")
        .unwrap();
    assert_eq!(
        standing_in_ctx.get("implemented"),
        Some(&serde_json::Value::Bool(false))
    );
    let gardening_in_ctx = parsed
        .get("cycle_context")
        .unwrap()
        .get("gardening_candidates")
        .unwrap();
    assert_eq!(
        gardening_in_ctx.get("implemented"),
        Some(&serde_json::Value::Bool(false))
    );
}

#[test]
fn text_format_renders_human_readable() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 10, "m", "2026-05-11T22:30:00Z");
    let (code, stdout, _err) = run(tmp.path(), &["--format", "text"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("v2-boot-phase report"));
    assert!(stdout.contains("Stages:"));
    assert!(stdout.contains("current_cycle:  11"));
    assert!(stdout.contains("DEFERRED"));
}

#[test]
fn output_to_file_writes_target() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    let target = tmp.path().join("out").join("context.json");
    let (code, stdout, _err) = run(
        tmp.path(),
        &["--format", "json", "--output", target.to_str().unwrap()],
    );
    assert_eq!(code, 0);
    assert!(stdout.is_empty());
    let content = fs::read_to_string(&target).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("current_cycle"),
        Some(&serde_json::Value::Number(2.into()))
    );
}

#[test]
fn custom_state_dir_arg() {
    let tmp = tempfile::tempdir().unwrap();
    let alt_state = tmp.path().join("alt").join("history");
    fs::create_dir_all(&alt_state).unwrap();
    write_cycle(&alt_state, 99, "m", "2026-05-11T22:30:00Z");
    let (code, stdout, _err) = run(
        tmp.path(),
        &["--state-dir", "alt/history", "--format", "json"],
    );
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("current_cycle"),
        Some(&serde_json::Value::Number(100.into()))
    );
}

#[test]
fn invalid_json_payload_counted_as_malformed() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    fs::write(state.join("2.json"), "{ this is not json").unwrap();
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let load = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "load-cycle-history")
        .unwrap();
    assert_eq!(load.get("status").unwrap(), "warn");
}

#[test]
fn summary_counts_stages_by_status() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    write_cycle(&state, 2, "m", "2026-05-11T22:30:00Z");
    let (_code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let sm = parsed.get("summary").unwrap();
    // load=done compute=done detect=done directives=deferred gardening=deferred assemble=done -> 4 done, 0 warn, 2 deferred
    assert_eq!(
        sm.get("done"),
        Some(&serde_json::Value::Number(4.into()))
    );
    assert_eq!(
        sm.get("deferred"),
        Some(&serde_json::Value::Number(2.into()))
    );
}
