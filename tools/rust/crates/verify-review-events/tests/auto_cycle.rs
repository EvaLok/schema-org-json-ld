use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(prefix: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "schema-org-json-ld-{prefix}-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn binary_path(name: &str) -> String {
    std::env::var(format!("CARGO_BIN_EXE_{name}"))
        .or_else(|_| std::env::var(format!("CARGO_BIN_EXE_{}", name.replace('-', "_"))))
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/debug")
                .join(name)
                .display()
                .to_string()
        })
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

fn run_git(repo_root: &Path, args: &[&str]) {
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(args)
        .env("GIT_AUTHOR_NAME", "Test User")
        .env("GIT_AUTHOR_EMAIL", "test@example.com")
        .env("GIT_COMMITTER_NAME", "Test User")
        .env("GIT_COMMITTER_EMAIL", "test@example.com")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn init_git_repo(repo_root: &Path) {
    run_git(repo_root, &["init"]);
    run_git(repo_root, &["config", "user.name", "Test User"]);
    run_git(repo_root, &["config", "user.email", "test@example.com"]);
    write_file(&repo_root.join("notes/start.txt"), "cycle start\n");
    run_git(repo_root, &["add", "."]);
    run_git(
        repo_root,
        &[
            "commit",
            "-m",
            "state(cycle-start): begin cycle 4, issue #1 [cycle 4]",
        ],
    );
}

fn commit_file_at(repo_root: &Path, path: &str, content: &str, message: &str, timestamp: &str) {
    write_file(&repo_root.join(path), content);
    run_git(repo_root, &["add", "."]);
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(["commit", "-m", message])
        .env("GIT_AUTHOR_NAME", "Test User")
        .env("GIT_AUTHOR_EMAIL", "test@example.com")
        .env("GIT_COMMITTER_NAME", "Test User")
        .env("GIT_COMMITTER_EMAIL", "test@example.com")
        .env("GIT_AUTHOR_DATE", timestamp)
        .env("GIT_COMMITTER_DATE", timestamp)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git commit failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn binary_apply_advances_when_no_prs_need_verification() {
    let repo_root = TempDir::new("verify-review-events-binary");
    init_git_repo(&repo_root.path);
    commit_file_at(
        &repo_root.path,
        "notes/cycle5.txt",
        "cycle 5\n",
        "state(cycle-start): begin cycle 5, issue #2 [cycle 5]",
        "2026-03-01T00:05:00Z",
    );
    write_file(
        &repo_root.path.join("docs/state.json"),
        r#"{
  "schema_version": 1,
  "last_cycle": {
    "number": 5
  },
  "cycle_phase": {
    "cycle": 5,
    "phase": "work",
    "phase_entered_at": "2026-03-01T00:05:00Z"
  },
  "field_inventory": {
    "fields": {
      "review_events_verified_through_cycle": {
        "last_refreshed": "cycle 4"
      }
    }
  },
  "agent_sessions": [],
  "copilot_metrics": {},
  "review_events_verified_through_cycle": 4
}
"#,
    );

    let output = Command::new(binary_path("verify-review-events"))
        .args([
            "--repo-root",
            repo_root
                .path
                .to_str()
                .expect("repo_root path should be valid UTF-8"),
            "--apply",
            "--json",
        ])
        .output()
        .expect("verify-review-events binary should execute");

    assert!(
        output.status.success(),
        "verify-review-events failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        report.get("safe_to_advance_to").and_then(Value::as_u64),
        Some(5)
    );
    assert_eq!(report.get("applied").and_then(Value::as_bool), Some(true));

    let state: Value =
        serde_json::from_str(&fs::read_to_string(repo_root.path.join("docs/state.json")).unwrap())
            .unwrap();
    assert_eq!(
        state
            .get("review_events_verified_through_cycle")
            .and_then(Value::as_u64),
        Some(5)
    );
}
