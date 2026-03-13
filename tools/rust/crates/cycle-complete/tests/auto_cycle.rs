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
            "state(cycle-start): begin cycle 42 [cycle 42]",
        ],
    );
}

#[test]
fn auto_derives_cycle_from_state_json() {
    let repo_root = TempDir::new("cycle-complete-auto-cycle");
    init_git_repo(&repo_root.path);
    write_file(
        &repo_root.path.join("docs/state.json"),
        r#"{
  "last_cycle": {
    "number": 41,
    "timestamp": "2026-03-01T00:00:00Z"
  },
  "cycle_phase": {
    "cycle": 42,
    "phase": "work",
    "phase_entered_at": "2026-03-01T00:00:00Z"
  }
}
"#,
    );

    let output = Command::new(binary_path("cycle-complete"))
        .args([
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "--issue",
            "123",
            "--json",
        ])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "cycle-complete failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report.pointer("/cycle").and_then(Value::as_u64), Some(42));
}
