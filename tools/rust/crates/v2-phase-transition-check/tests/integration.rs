use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop();
    path.pop();
    path.push("target");
    path.push(if cfg!(debug_assertions) {
        "debug"
    } else {
        "release"
    });
    path.push("v2-phase-transition-check");
    path
}

fn run_with_args(repo_root: &Path, args: &[&str]) -> std::process::Output {
    Command::new(binary_path())
        .arg("--repo-root")
        .arg(repo_root)
        .args(args)
        .output()
        .expect("failed to invoke v2-phase-transition-check")
}

fn make_entry(state_dir: &Path, cycle: u64, body: &str) {
    fs::create_dir_all(state_dir).unwrap();
    fs::write(state_dir.join(format!("{}.json", cycle)), body).unwrap();
}

fn entry_with_required(cycle: u64) -> String {
    format!(
        r#"{{
  "cycle_number": {cycle},
  "model": "claude-opus-4-7",
  "started_at": "2026-05-11T20:31:00Z"
}}"#
    )
}

#[test]
fn missing_state_dir_is_invocation_error() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_with_args(dir.path(), &[]);
    assert!(!out.status.success());
    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("state directory does not exist"));
}

#[test]
fn empty_state_dir_passes() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    fs::create_dir_all(&state_dir).unwrap();
    let out = run_with_args(dir.path(), &[]);
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Entries scanned: 0"));
}

#[test]
fn passes_on_clean_two_entry_set() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    make_entry(&state_dir, 1, &entry_with_required(1));
    make_entry(&state_dir, 2, &entry_with_required(2));

    let out = run_with_args(dir.path(), &[]);
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Entries scanned: 2"));
    assert!(stdout.contains("[PASS] filename-field-consistency"));
    assert!(stdout.contains("[PASS] monotonicity"));
    assert!(stdout.contains("[PASS] no-gaps"));
    assert!(stdout.contains("[PASS] required-fields"));
    assert!(stdout.contains("[PASS] rfc3339-started-at"));
}

#[test]
fn detects_filename_field_mismatch() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    // Filename says 5, payload says 7
    make_entry(
        &state_dir,
        5,
        r#"{"cycle_number": 7, "model": "x", "started_at": "2026-01-01T00:00:00Z"}"#,
    );
    let out = run_with_args(dir.path(), &[]);
    assert_eq!(out.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[FAIL] filename-field-consistency"));
}

#[test]
fn detects_gaps_as_warning() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    make_entry(&state_dir, 1, &entry_with_required(1));
    make_entry(&state_dir, 3, &entry_with_required(3));
    make_entry(&state_dir, 4, &entry_with_required(4));

    let out = run_with_args(dir.path(), &[]);
    // No --strict: warnings do not fail
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[WARN] no-gaps"));
    assert!(stdout.contains("missing cycle 2"));
}

#[test]
fn strict_promotes_warnings_to_fail() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    make_entry(&state_dir, 1, &entry_with_required(1));
    make_entry(&state_dir, 3, &entry_with_required(3));

    let out = run_with_args(dir.path(), &["--strict"]);
    assert_eq!(out.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[WARN] no-gaps"));
}

#[test]
fn detects_missing_required_fields() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    make_entry(&state_dir, 1, r#"{"cycle_number": 1}"#);
    let out = run_with_args(dir.path(), &[]);
    assert_eq!(out.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[FAIL] required-fields"));
    assert!(stdout.contains("missing required field 'model'"));
    assert!(stdout.contains("missing required field 'started_at'"));
}

#[test]
fn detects_malformed_timestamp_as_warning() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    make_entry(
        &state_dir,
        1,
        r#"{"cycle_number": 1, "model": "x", "started_at": "not-a-date"}"#,
    );
    let out = run_with_args(dir.path(), &[]);
    // Required-fields passes (started_at is non-empty); rfc3339 warns
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[WARN] rfc3339-started-at"));
}

#[test]
fn json_output_is_valid_json() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    make_entry(&state_dir, 1, &entry_with_required(1));
    make_entry(&state_dir, 2, &entry_with_required(2));

    let out = run_with_args(dir.path(), &["--format", "json"]);
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8(out.stdout).expect("utf8");
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid json");
    assert_eq!(parsed["entries_scanned"], 2);
    assert_eq!(parsed["range"]["min"], 1);
    assert_eq!(parsed["range"]["max"], 2);
    let summary = &parsed["summary"];
    assert!(summary["pass"].as_u64().unwrap() >= 4);
}

#[test]
fn invalid_json_payload_is_error() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    make_entry(&state_dir, 1, "{not json");

    let out = run_with_args(dir.path(), &[]);
    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("json parse error"));
}

#[test]
fn root_must_be_object() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    make_entry(&state_dir, 1, "[1, 2, 3]");

    let out = run_with_args(dir.path(), &[]);
    assert_eq!(out.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("root is array"));
}

#[test]
fn ignores_non_numeric_filenames() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    fs::create_dir_all(&state_dir).unwrap();
    fs::write(state_dir.join("README.json"), r#"{"note": "skip me"}"#).unwrap();
    make_entry(&state_dir, 1, &entry_with_required(1));

    let out = run_with_args(dir.path(), &[]);
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Entries scanned: 1"));
}

#[test]
fn ignores_non_json_files() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    fs::create_dir_all(&state_dir).unwrap();
    fs::write(state_dir.join("1.txt"), "not json").unwrap();
    make_entry(&state_dir, 1, &entry_with_required(1));

    let out = run_with_args(dir.path(), &[]);
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Entries scanned: 1"));
}

#[test]
fn from_cycle_filter() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    make_entry(&state_dir, 1, &entry_with_required(1));
    make_entry(&state_dir, 5, &entry_with_required(5));
    make_entry(&state_dir, 10, &entry_with_required(10));

    let out = run_with_args(dir.path(), &["--from-cycle", "5"]);
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Entries scanned: 2"));
    assert!(stdout.contains("Range: [5..10]"));
}

#[test]
fn to_cycle_filter() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    make_entry(&state_dir, 1, &entry_with_required(1));
    make_entry(&state_dir, 5, &entry_with_required(5));
    make_entry(&state_dir, 10, &entry_with_required(10));

    let out = run_with_args(dir.path(), &["--to-cycle", "5"]);
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Entries scanned: 2"));
    assert!(stdout.contains("Range: [1..5]"));
}

#[test]
fn phase_field_detected_when_present() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    make_entry(
        &state_dir,
        1,
        r#"{"cycle_number": 1, "model": "x", "started_at": "2026-01-01T00:00:00Z", "phase": "boot"}"#,
    );
    let out = run_with_args(dir.path(), &[]);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[PASS] phase-field"));
}

#[test]
fn phase_field_invalid_value_fails() {
    let dir = tempfile::tempdir().unwrap();
    let state_dir = dir.path().join("state").join("cycle-history");
    make_entry(
        &state_dir,
        1,
        r#"{"cycle_number": 1, "model": "x", "started_at": "2026-01-01T00:00:00Z", "phase": "middle"}"#,
    );
    let out = run_with_args(dir.path(), &[]);
    assert_eq!(out.status.code(), Some(1));
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("[FAIL] phase-field"));
}

#[test]
fn custom_state_dir_arg() {
    let dir = tempfile::tempdir().unwrap();
    let custom_dir = dir.path().join("custom");
    make_entry(&custom_dir, 1, &entry_with_required(1));

    let out = run_with_args(dir.path(), &["--state-dir", "custom"]);
    assert!(
        out.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("Entries scanned: 1"));
}
