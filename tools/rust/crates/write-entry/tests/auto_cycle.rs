use chrono::Utc;
use std::fs;
use std::os::unix::fs::PermissionsExt;
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

fn run_git(repo_root: &Path, args: &[&str]) -> String {
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
    String::from_utf8(output.stdout).unwrap().trim().to_string()
}

fn init_git_repo(repo_root: &Path) -> String {
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
            "state(cycle-start): begin cycle 99 [cycle 99]",
        ],
    );
    run_git(repo_root, &["rev-parse", "--short=7", "HEAD"])
}

fn install_cycle_receipts_script(repo_root: &Path, receipt: &str) {
    let script = format!(
        "#!/usr/bin/env bash\nset -euo pipefail\ncat <<'JSON'\n[{{\"step\":\"cycle-start\",\"receipt\":\"{receipt}\",\"commit\":\"state(cycle-start): begin cycle 99 [cycle 99]\"}}]\nJSON\n"
    );
    let path = repo_root.join("tools/cycle-receipts");
    write_file(&path, &script);
    let mut permissions = fs::metadata(&path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

#[test]
fn auto_derives_cycle_from_state_json() {
    let repo_root = TempDir::new("write-entry-auto-cycle");
    let receipt = init_git_repo(&repo_root.path);
    install_cycle_receipts_script(&repo_root.path, &receipt);
    write_file(
        &repo_root.path.join("docs/state.json"),
        r#"{
  "last_cycle": {"number": 98},
  "cycle_phase": {
    "cycle": 99,
    "phase": "work",
    "phase_entered_at": "2026-03-01T00:00:00Z"
  }
}
"#,
    );

    let output = Command::new(binary_path("write-entry"))
        .args([
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "worklog",
            "--title",
            "test",
            "--done",
            "test done",
            "--pipeline",
            "PASS",
            "--in-flight",
            "0",
            "--copilot-metrics",
            "test",
            "--publish-gate",
            "test",
        ])
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "write-entry failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output_path = PathBuf::from(String::from_utf8(output.stdout).unwrap().trim());
    let content = fs::read_to_string(&output_path).unwrap();
    let expected_date_dir = repo_root
        .path
        .join("docs/worklog")
        .join(Utc::now().format("%Y-%m-%d").to_string());

    assert!(content.contains("Cycle 99"));
    assert_eq!(output_path.parent(), Some(expected_date_dir.as_path()));
    assert!(output_path.is_file());
}
