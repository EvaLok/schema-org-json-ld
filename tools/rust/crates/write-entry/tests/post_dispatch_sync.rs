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

fn workspace_manifest_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../Cargo.toml")
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
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(&path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }
}

fn run_write_entry(repo_root: &Path, args: &[&str]) -> PathBuf {
    let output = Command::new(binary_path("write-entry"))
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "write-entry failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).unwrap();
    PathBuf::from(
        stdout
            .trim()
            .strip_prefix("Worklog created: ")
            .unwrap_or(stdout.trim()),
    )
    .strip_prefix(repo_root)
    .map(|relative| repo_root.join(relative))
    .unwrap_or_else(|_| PathBuf::from(stdout.trim()))
}

#[test]
fn narrative_merged_prs_are_added_to_prs_merged_block_when_auto_receipts_find_none() {
    let repo_root = TempDir::new("write-entry-narrative-pr-sync");
    let receipt = init_git_repo(&repo_root.path);
    install_cycle_receipts_script(&repo_root.path, &receipt);
    write_file(
        &repo_root.path.join("docs/state.json"),
        r#"{
  "last_cycle": {"number": 98},
  "in_flight_sessions": 0,
  "cycle_phase": {
    "cycle": 99,
    "phase": "close_out",
    "phase_entered_at": "2026-03-01T00:00:00Z"
  }
}
"#,
    );

    let worklog_path = run_write_entry(
        &repo_root.path,
        &[
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "worklog",
            "--title",
            "test",
            "--done",
            "Merged cycle 98 review PR #88.",
            "--auto-receipts",
            "--pipeline",
            "PASS",
            "--publish-gate",
            "published",
        ],
    );

    let content = fs::read_to_string(worklog_path).unwrap();
    assert!(content.contains(
        "Merged cycle 98 review [PR #88](https://github.com/EvaLok/schema-org-json-ld/issues/88)."
    ));
    assert!(content.contains("### PRs merged"));
    assert!(content.contains("[PR #88](https://github.com/EvaLok/schema-org-json-ld/issues/88)"));
    assert!(!content.contains("### PRs merged\n\n- None."));
}

#[test]
fn record_dispatch_appends_post_dispatch_delta_matching_final_state() {
    let repo_root = TempDir::new("write-entry-post-dispatch-delta");
    init_git_repo(&repo_root.path);
    write_file(
        &repo_root.path.join("docs/state.json"),
        r##"{
  "agent_sessions": [
    {
      "issue": 600,
      "title": "Merged change",
      "dispatched_at": "2026-03-01T00:00:00Z",
      "model": "gpt-5.4",
      "status": "merged",
      "pr": 700,
      "merged_at": "2026-03-02T00:00:00Z"
    }
  ],
  "in_flight_sessions": 0,
  "last_cycle": {
    "number": 164,
    "timestamp": "2026-03-07T12:00:00Z",
    "summary": "0 dispatches, 1 merges (PR #700)"
  },
  "cycle_phase": {
    "cycle": 164,
    "phase": "close_out",
    "phase_entered_at": "2026-03-07T12:00:00Z"
  },
  "review_agent": {
    "history": []
  },
  "field_inventory": {
    "fields": {
      "cycle_phase": {"last_refreshed": "cycle 163"},
      "copilot_metrics.in_flight": {"last_refreshed": "cycle 163"},
      "copilot_metrics.dispatch_to_pr_rate": {"last_refreshed": "cycle 163"},
      "copilot_metrics.pr_merge_rate": {"last_refreshed": "cycle 163"}
    }
  },
  "copilot_metrics": {
    "total_dispatches": 1,
    "resolved": 1,
    "merged": 1,
    "closed_without_pr": 0,
    "reviewed_awaiting_eva": 0,
    "in_flight": 0,
    "produced_pr": 1,
    "pr_merge_rate": "100.0%",
    "dispatch_to_pr_rate": "100.0%",
    "dispatch_log_latest": "#600 Merged change (cycle 164)"
  },
  "tool_pipeline": {
    "c5_5_gate": {
      "cycle": 164,
      "status": "PASS",
      "needs_reverify": false
    }
  }
}
"##,
    );

    let worklog_path = run_write_entry(
        &repo_root.path,
        &[
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "worklog",
            "--title",
            "closeout",
            "--cycle",
            "164",
            "--done",
            "No new dispatches.",
            "--pipeline",
            "PASS",
            "--publish-gate",
            "published",
        ],
    );

    let record_dispatch = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "-p",
            "record-dispatch",
            "--manifest-path",
            workspace_manifest_path().to_str().unwrap(),
            "--",
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "--issue",
            "602",
            "--title",
            "Cycle review dispatch",
            "--review-dispatch",
            "--model",
            "gpt-5.4",
        ])
        .output()
        .unwrap();
    assert!(
        record_dispatch.status.success(),
        "record-dispatch failed: {}",
        String::from_utf8_lossy(&record_dispatch.stderr)
    );
    let record_dispatch_stdout = String::from_utf8(record_dispatch.stdout).unwrap();
    assert!(record_dispatch_stdout.contains("Dispatch recorded"));

    let state: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(repo_root.path.join("docs/state.json")).unwrap())
            .unwrap();
    let content = fs::read_to_string(worklog_path).unwrap();
    let in_flight = state["in_flight_sessions"].as_u64().unwrap();
    let summary = state["last_cycle"]["summary"].as_str().unwrap();
    let dispatch_count = summary.split(',').next().unwrap().trim();

    assert!(content.contains("## Post-dispatch delta"));
    assert!(content.contains(&format!("- **In-flight agent sessions**: {in_flight}")));
    assert!(content.contains(&format!("- **Dispatch count**: {dispatch_count}")));
    assert!(content.contains(&format!("- **Last-cycle summary**: {summary}")));
}

#[test]
fn worklog_generation_includes_post_dispatch_delta_for_current_cycle_dispatches() {
    let repo_root = TempDir::new("write-entry-closeout-post-dispatch-delta");
    init_git_repo(&repo_root.path);
    write_file(
        &repo_root.path.join("docs/state.json"),
        r##"{
  "agent_sessions": [
    {
      "issue": 701,
      "title": "Current cycle dispatch",
      "dispatched_at": "2026-03-07T12:15:00Z",
      "model": "gpt-5.4",
      "status": "in_flight"
    },
    {
      "issue": 699,
      "title": "Previous cycle dispatch",
      "dispatched_at": "2026-03-07T11:45:00Z",
      "model": "gpt-5.4",
      "status": "merged",
      "merged_at": "2026-03-07T11:55:00Z"
    }
  ],
  "in_flight_sessions": 1,
  "last_cycle": {
    "number": 164,
    "timestamp": "2026-03-07T12:20:00Z",
    "summary": "1 dispatch, 0 merges"
  },
  "cycle_phase": {
    "cycle": 164,
    "phase": "close_out",
    "phase_entered_at": "2026-03-07T12:00:00Z"
  },
  "review_agent": {
    "history": []
  },
  "field_inventory": {
    "fields": {
      "cycle_phase": {"last_refreshed": "cycle 163"},
      "copilot_metrics.in_flight": {"last_refreshed": "cycle 163"},
      "copilot_metrics.dispatch_to_pr_rate": {"last_refreshed": "cycle 163"},
      "copilot_metrics.pr_merge_rate": {"last_refreshed": "cycle 163"}
    }
  },
  "tool_pipeline": {
    "c5_5_gate": {
      "cycle": 164,
      "status": "PASS",
      "needs_reverify": false
    }
  },
  "publish_gate": {
    "status": "published"
  }
}
"##,
    );

    let worklog_path = run_write_entry(
        &repo_root.path,
        &[
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "worklog",
            "--title",
            "closeout",
            "--cycle",
            "164",
            "--done",
            "Closed out the cycle.",
            "--pipeline",
            "PASS",
            "--publish-gate",
            "published",
        ],
    );

    let content = fs::read_to_string(worklog_path).unwrap();
    assert!(content.contains("## Post-dispatch delta"));
    assert!(content.contains("- **In-flight agent sessions**: 1"));
    assert!(content.contains("- **Dispatch count**: 1 dispatch"));
    assert!(content.contains("- **Last-cycle summary**: 1 dispatch, 0 merges"));
}
