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

fn repo_root_from_manifest() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../../")
}

fn workspace_manifest_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml")
}

fn copy_file(from: &Path, to: &Path) {
    if let Some(parent) = to.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::copy(from, to).unwrap();
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
    let remote_root = repo_root.with_extension("remote.git");
    run_git(repo_root, &["init"]);
    run_git(repo_root, &["config", "user.name", "Test User"]);
    run_git(repo_root, &["config", "user.email", "test@example.com"]);
    run_git(
        repo_root,
        &["init", "--bare", remote_root.to_str().unwrap()],
    );
    run_git(&remote_root, &["symbolic-ref", "HEAD", "refs/heads/master"]);
    run_git(
        repo_root,
        &["remote", "add", "origin", remote_root.to_str().unwrap()],
    );
    run_git(repo_root, &["add", "."]);
    run_git(
        repo_root,
        &["commit", "-m", "test: seed real docs state fixture"],
    );
    run_git(repo_root, &["push", "-u", "origin", "HEAD:master"]);
}

#[test]
fn auto_review_summary_works_with_real_state_shape_after_process_review_persists_review_issue() {
    let source_repo = repo_root_from_manifest();
    let repo_root = TempDir::new("write-entry-real-state-auto-review-summary");

    copy_file(
        &source_repo.join("docs/state.json"),
        &repo_root.path.join("docs/state.json"),
    );
    copy_file(
        &source_repo.join("docs/reviews/cycle-473.md"),
        &repo_root.path.join("docs/reviews/cycle-473.md"),
    );
    init_git_repo(&repo_root.path);

    let process_review = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "-p",
            "process-review",
            "--manifest-path",
            workspace_manifest_path().to_str().unwrap(),
            "--",
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "--review-file",
            "docs/reviews/cycle-473.md",
            "--review-issue",
            "2393",
            "--actioned",
            "1",
            "--deferred",
            "1",
            "--dispatch-created",
            "1",
            "--disposition",
            "code-change-quality:dispatch_created",
            "--disposition",
            "journal-quality:deferred",
            "--disposition",
            "state-integrity:actioned",
        ])
        .output()
        .unwrap();
    assert!(
        process_review.status.success(),
        "process-review failed: {}",
        String::from_utf8_lossy(&process_review.stderr)
    );

    let updated_state: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(repo_root.path.join("docs/state.json")).unwrap())
            .unwrap();
    let cycle_473_entry = updated_state["review_agent"]["history"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["cycle"].as_u64() == Some(473))
        .expect("cycle 473 history entry should exist");
    assert_eq!(cycle_473_entry["review_issue"].as_u64(), Some(2393));

    let write_entry = Command::new(binary_path("write-entry"))
        .args([
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "worklog",
            "--title",
            "real state review summary",
            "--cycle",
            "474",
            "--auto-review-summary",
            "--dry-run",
            "--pipeline",
            "PASS",
            "--publish-gate",
            "published",
        ])
        .output()
        .unwrap();
    assert!(
        write_entry.status.success(),
        "write-entry failed: {}",
        String::from_utf8_lossy(&write_entry.stderr)
    );

    let stdout = String::from_utf8(write_entry.stdout).unwrap();
    assert!(stdout.contains("Processed cycle 473 review (3 findings, complacency 3/5,"));
    assert!(stdout.contains("1 dispatch_created"));
    assert!(stdout.contains("1 deferred"));
    assert!(stdout.contains("1 actioned"));
}
