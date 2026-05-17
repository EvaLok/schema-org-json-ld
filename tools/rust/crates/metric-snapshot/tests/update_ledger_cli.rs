use std::fs;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock must be after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("{prefix}-{suffix}"))
}

#[test]
fn update_ledger_exits_non_zero_when_state_json_is_missing() {
    let repo_root = unique_temp_dir("metric-snapshot-missing-state");
    fs::create_dir_all(&repo_root).expect("repo root should be created");

    let output = Command::new(env!("CARGO_BIN_EXE_metric-snapshot"))
        .args(["--repo-root", repo_root.to_str().unwrap(), "--update-ledger"])
        .output()
        .expect("metric-snapshot should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error reading"));
    assert!(stderr.contains("docs/state.json"));

    fs::remove_dir_all(repo_root).expect("temp dir should be removed");
}

#[test]
fn update_ledger_exits_non_zero_when_state_json_is_malformed() {
    let repo_root = unique_temp_dir("metric-snapshot-malformed-state");
    fs::create_dir_all(repo_root.join("docs")).expect("docs dir should be created");
    fs::write(repo_root.join("docs/state.json"), "{ not valid json")
        .expect("malformed fixture should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_metric-snapshot"))
        .args(["--repo-root", repo_root.to_str().unwrap(), "--update-ledger"])
        .output()
        .expect("metric-snapshot should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error parsing JSON"));
    assert!(stderr.contains("docs/state.json"));

    fs::remove_dir_all(repo_root).expect("temp dir should be removed");
}
