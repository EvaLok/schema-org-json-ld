use std::fs;
use std::path::PathBuf;
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
    path.push("v2-cycle-history-append");
    path
}

fn run_with_args(repo_root: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(binary_path())
        .args(args)
        .arg("--repo-root")
        .arg(repo_root)
        .output()
        .expect("failed to invoke v2-cycle-history-append")
}

#[test]
fn writes_entry_with_field_args() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_with_args(
        dir.path(),
        &[
            "--cycle-n",
            "94",
            "--field",
            "model=claude-opus-4-7",
            "--field",
            "started_at=2026-05-08T06:31:00Z",
        ],
    );
    assert!(
        out.status.success(),
        "expected success, stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );

    let target = dir.path().join("state/cycle-history/94.json");
    assert!(target.exists(), "target file not written: {:?}", target);

    let contents = fs::read_to_string(&target).unwrap();
    let value: serde_json::Value = serde_json::from_str(&contents).unwrap();
    assert_eq!(value["cycle_number"], serde_json::Value::from(94));
    assert_eq!(
        value["model"],
        serde_json::Value::String("claude-opus-4-7".to_string())
    );
}

#[test]
fn refuses_overwrite_by_default() {
    let dir = tempfile::tempdir().unwrap();
    let args = [
        "--cycle-n",
        "94",
        "--field",
        "model=x",
        "--field",
        "started_at=t",
    ];
    let first = run_with_args(dir.path(), &args);
    assert!(first.status.success(), "first write should succeed");

    let second = run_with_args(dir.path(), &args);
    assert!(
        !second.status.success(),
        "second write should fail (refuse-overwrite)"
    );
    let stderr = String::from_utf8_lossy(&second.stderr);
    assert!(
        stderr.contains("refusing to overwrite"),
        "stderr = {stderr}"
    );
}

#[test]
fn allow_overwrite_replaces_entry() {
    let dir = tempfile::tempdir().unwrap();
    let first = run_with_args(
        dir.path(),
        &[
            "--cycle-n",
            "94",
            "--field",
            "model=first",
            "--field",
            "started_at=t",
        ],
    );
    assert!(first.status.success());

    let second = run_with_args(
        dir.path(),
        &[
            "--cycle-n",
            "94",
            "--allow-overwrite",
            "--field",
            "model=second",
            "--field",
            "started_at=t",
        ],
    );
    assert!(
        second.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&second.stderr)
    );

    let target = dir.path().join("state/cycle-history/94.json");
    let contents = fs::read_to_string(&target).unwrap();
    let value: serde_json::Value = serde_json::from_str(&contents).unwrap();
    assert_eq!(
        value["model"],
        serde_json::Value::String("second".to_string())
    );
}

#[test]
fn dry_run_does_not_write() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_with_args(
        dir.path(),
        &[
            "--cycle-n",
            "94",
            "--dry-run",
            "--field",
            "model=x",
            "--field",
            "started_at=t",
        ],
    );
    assert!(out.status.success());
    let target = dir.path().join("state/cycle-history/94.json");
    assert!(!target.exists(), "dry-run should not write");

    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(stdout.contains("94.json"), "stdout = {stdout}");
    assert!(stdout.contains("\"cycle_number\""), "stdout = {stdout}");
}

#[test]
fn rejects_missing_required_field() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_with_args(dir.path(), &["--cycle-n", "94", "--field", "model=x"]);
    assert!(!out.status.success(), "should fail on missing started_at");
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("started_at"), "stderr = {stderr}");
}

#[test]
fn from_json_loads_payload() {
    let dir = tempfile::tempdir().unwrap();
    let payload_path = dir.path().join("payload.json");
    fs::write(
        &payload_path,
        r#"{"cycle_number": 94, "model": "test-model", "started_at": "2026-05-08T06:31:00Z", "extra": "passthrough"}"#,
    )
    .unwrap();

    let out = run_with_args(
        dir.path(),
        &[
            "--cycle-n",
            "94",
            "--from-json",
            payload_path.to_str().unwrap(),
        ],
    );
    assert!(
        out.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );

    let target = dir.path().join("state/cycle-history/94.json");
    let contents = fs::read_to_string(&target).unwrap();
    let value: serde_json::Value = serde_json::from_str(&contents).unwrap();
    assert_eq!(
        value["extra"],
        serde_json::Value::String("passthrough".to_string()),
        "unknown fields must pass through unchanged"
    );
}

#[test]
fn cli_field_overrides_json_payload() {
    let dir = tempfile::tempdir().unwrap();
    let payload_path = dir.path().join("payload.json");
    fs::write(
        &payload_path,
        r#"{"cycle_number": 94, "model": "from-json", "started_at": "t"}"#,
    )
    .unwrap();

    let out = run_with_args(
        dir.path(),
        &[
            "--cycle-n",
            "94",
            "--from-json",
            payload_path.to_str().unwrap(),
            "--field",
            "model=from-cli",
        ],
    );
    assert!(
        out.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );

    let target = dir.path().join("state/cycle-history/94.json");
    let contents = fs::read_to_string(&target).unwrap();
    let value: serde_json::Value = serde_json::from_str(&contents).unwrap();
    assert_eq!(
        value["model"],
        serde_json::Value::String("from-cli".to_string()),
        "--field should override --from-json values"
    );
}

#[test]
fn rejects_payload_cycle_mismatch() {
    let dir = tempfile::tempdir().unwrap();
    let payload_path = dir.path().join("payload.json");
    fs::write(
        &payload_path,
        r#"{"cycle_number": 93, "model": "x", "started_at": "t"}"#,
    )
    .unwrap();

    let out = run_with_args(
        dir.path(),
        &[
            "--cycle-n",
            "94",
            "--from-json",
            payload_path.to_str().unwrap(),
        ],
    );
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(stderr.contains("does not match"), "stderr = {stderr}");
}

#[test]
fn writes_to_custom_state_dir() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_with_args(
        dir.path(),
        &[
            "--cycle-n",
            "94",
            "--state-dir",
            "alt/cycles",
            "--field",
            "model=x",
            "--field",
            "started_at=t",
        ],
    );
    assert!(
        out.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(dir.path().join("alt/cycles/94.json").exists());
}

#[test]
fn output_quiet_suppresses_stdout() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_with_args(
        dir.path(),
        &[
            "--cycle-n",
            "94",
            "--output",
            "quiet",
            "--field",
            "model=x",
            "--field",
            "started_at=t",
        ],
    );
    assert!(out.status.success());
    assert!(
        out.stdout.is_empty(),
        "stdout should be empty under --output quiet"
    );
}

#[test]
fn output_json_prints_full_content() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_with_args(
        dir.path(),
        &[
            "--cycle-n",
            "94",
            "--output",
            "json",
            "--field",
            "model=test",
            "--field",
            "started_at=t",
        ],
    );
    assert!(out.status.success());
    let stdout = String::from_utf8_lossy(&out.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        parsed["model"],
        serde_json::Value::String("test".to_string())
    );
}

#[test]
fn rejects_invalid_field_format() {
    let dir = tempfile::tempdir().unwrap();
    let out = run_with_args(
        dir.path(),
        &[
            "--cycle-n",
            "94",
            "--field",
            "no-equals-sign",
            "--field",
            "model=x",
            "--field",
            "started_at=t",
        ],
    );
    assert!(!out.status.success());
    let stderr = String::from_utf8_lossy(&out.stderr);
    assert!(
        stderr.contains("KEY=VALUE") || stderr.contains("invalid"),
        "stderr = {stderr}"
    );
}
