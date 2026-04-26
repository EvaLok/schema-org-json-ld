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

fn prepare_journal_repo() -> TempDir {
    let repo_root = TempDir::new("write-entry-met-validation");
    write_file(
        &repo_root.path.join("docs/state.json"),
        r#"{
  "last_cycle": {"number": 153},
  "cycle_phase": {"cycle": 154},
  "agent_sessions": [],
  "in_flight_sessions": 0
}
"#,
    );
    write_file(
        &repo_root.path.join("JOURNAL.md"),
        "# Journal\n\nJournal entries have been split into per-date files in [`docs/journal/`](docs/journal/).\n",
    );
    write_file(
        &repo_root.path.join("docs/journal/2026-03-05.md"),
        concat!(
            "# Journal — 2026-03-05\n\n",
            "Reflective log for the schema-org-json-ld orchestrator.\n\n",
            "---\n\n",
            "## 2026-03-05 — Cycle 153: Prior title\n\n",
            "### Concrete commitments for next cycle\n\n",
            "1. Verify close-out gate widening.\n",
        ),
    );
    write_file(
        &repo_root
            .path
            .join("docs/worklog/2026-03-06/051458-cycle-154-smoke.md"),
        "# Cycle 154 — Smoke\n\nBody.\n",
    );
    repo_root
}

fn run_write_entry(repo_root: &Path, args: &[&str]) -> std::process::Output {
    Command::new(binary_path("write-entry"))
        .args(["--repo-root", repo_root.to_str().unwrap()])
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn cli_rejects_met_future_tense_detail() {
    let repo_root = prepare_journal_repo();

    let output = run_write_entry(
        &repo_root.path,
        &[
            "journal",
            "--title",
            "Future tense rejection",
            "--cycle",
            "154",
            "--previous-commitment-status",
            "Met",
            "--previous-commitment-detail",
            "The check will pass once dispatched",
        ],
    );

    assert!(!output.status.success(), "command should fail");
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(
        stderr.contains("forbidden phrase 'will'"),
        "stderr was: {stderr}"
    );
}

#[test]
fn cli_accepts_met_past_tense_detail() {
    let repo_root = prepare_journal_repo();

    let output = run_write_entry(
        &repo_root.path,
        &[
            "journal",
            "--title",
            "Past tense acceptance",
            "--cycle",
            "154",
            "--previous-commitment-status",
            "Met",
            "--previous-commitment-detail",
            "Ran tools/state-invariants at receipt abc1234; check 8 emitted WARN not FAIL as required",
        ],
    );

    assert!(
        output.status.success(),
        "write-entry failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let output_path = PathBuf::from(stdout.trim());
    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains(
        "- **Met** — Ran tools/state-invariants at receipt abc1234; check 8 emitted WARN not FAIL as required"
    ));
}

#[test]
fn cli_allows_partial_future_tense_detail() {
    let repo_root = prepare_journal_repo();

    let output = run_write_entry(
        &repo_root.path,
        &[
            "journal",
            "--title",
            "Partial future tense",
            "--cycle",
            "154",
            "--previous-commitment-status",
            "Partial",
            "--previous-commitment-detail",
            "The check will pass once dispatched",
        ],
    );

    assert!(
        output.status.success(),
        "write-entry failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    let output_path = PathBuf::from(stdout.trim());
    let content = fs::read_to_string(output_path).unwrap();
    assert!(content.contains("- **Partial** — The check will pass once dispatched"));
}
