use chrono::{Duration, Utc};
use std::fs;
#[cfg(unix)]
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

fn make_executable(path: &Path) {
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }
}

fn run_write_entry_with_fake_gh(repo_root: &Path, gh_response: &str, cycle: u64) -> String {
    fs::create_dir_all(repo_root.join("docs")).unwrap();
    write_file(
        &repo_root.join("JOURNAL.md"),
        "# Journal\n\nJournal entries have been split into per-date files in [`docs/journal/`](docs/journal/).\n\n",
    );

    let bin_dir = repo_root.join("bin");
    fs::create_dir_all(&bin_dir).unwrap();
    let gh_response_path = repo_root.join("gh-response.json");
    write_file(&gh_response_path, gh_response);
    let gh_path = bin_dir.join("gh");
    write_file(
        &gh_path,
        r#"#!/usr/bin/env bash
set -euo pipefail
if [ "${1:-}" != "api" ]; then
  echo "unexpected gh invocation: $*" >&2
  exit 1
fi
cat "$GH_RESPONSE_FILE"
"#,
    );
    make_executable(&gh_path);

    let path = format!(
        "{}:{}",
        bin_dir.display(),
        std::env::var("PATH").unwrap_or_default()
    );
    let output = Command::new(binary_path("write-entry"))
        .env("PATH", path)
        .env("GH_RESPONSE_FILE", &gh_response_path)
        .args([
            "--repo-root",
            repo_root.to_str().unwrap(),
            "journal",
            "--cycle",
            &cycle.to_string(),
            "--title",
            "Auto blockers journal",
            "--auto-blockers",
            "--previous-commitment-status",
            "no_prior_commitment",
            "--previous-commitment-detail",
            "No prior commitment recorded.",
        ])
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "write-entry failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let output_path = PathBuf::from(String::from_utf8(output.stdout).unwrap().trim());
    fs::read_to_string(output_path).unwrap()
}

#[test]
fn auto_blockers_renders_live_question_for_eva_issues_in_journal_format() {
    let repo_root = TempDir::new("write-entry-auto-blockers-populated");
    let now = Utc::now();
    let gh_response = format!(
        r#"[
  {{
    "number": 2696,
    "title": "[question-for-eva] Replace raw gh api dispatch template in orchestrator-prompt.xml with dispatch-task (root-cause fix for cycle 536 F1)",
    "created_at": "{}",
    "user": {{"login": "EvaLok"}}
  }},
  {{
    "number": 2638,
    "title": "[question-for-eva] cycle-start commits without pushing — cycle 524 corrupted mid-close, F4 violation reproduced",
    "created_at": "{}",
    "user": {{"login": "EvaLok"}}
  }},
  {{
    "number": 9999,
    "title": "Untrusted question should not render",
    "created_at": "{}",
    "user": {{"login": "someone-else"}}
  }}
]"#,
        (now - Duration::hours(43) - Duration::seconds(1)).to_rfc3339(),
        (now - Duration::hours(212) - Duration::seconds(1)).to_rfc3339(),
        (now - Duration::hours(10)).to_rfc3339(),
    );

    let content = run_write_entry_with_fake_gh(&repo_root.path, &gh_response, 543);

    assert!(content.contains("### Standing Eva blockers"));
    assert!(content.contains("[#2696](https://github.com/EvaLok/schema-org-json-ld/issues/2696) — [question-for-eva] Replace raw gh api dispatch template in orchestrator-prompt.xml with dispatch-task (root-cause fix for cycle 536 F1) (44h stale)"));
    assert!(content.contains("[#2638](https://github.com/EvaLok/schema-org-json-ld/issues/2638) — [question-for-eva] cycle-start commits without pushing — cycle 524 corrupted mid-close, F4 violation reproduced (213h stale)"));
    assert!(!content.contains("#9999"));
}

#[test]
fn auto_blockers_renders_none_when_no_live_question_for_eva_issues_exist() {
    let repo_root = TempDir::new("write-entry-auto-blockers-empty");
    let content = run_write_entry_with_fake_gh(&repo_root.path, "[]", 544);

    assert!(content.contains("### Standing Eva blockers\n\n- None.\n"));
}

#[test]
fn journal_help_mentions_auto_blockers_flag() {
    let output = Command::new(binary_path("write-entry"))
        .args(["journal", "--help"])
        .output()
        .unwrap();
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("--auto-blockers"));
}
