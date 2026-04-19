use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const CYCLE_493_CLOSE_OUT_FIXTURE: &str =
    include_str!("fixtures/cycle-493-post-cycle-complete-state.json");
const CYCLE_495_CLOSE_OUT_FIXTURE: &str =
    include_str!("fixtures/cycle-495-post-cycle-complete-state.json");

#[test]
fn record_dispatch_replays_cycle_493_close_out_flow() {
    let repo = TempRepo::new();
    repo.init_with_state(CYCLE_493_CLOSE_OUT_FIXTURE);

    let before = repo.read_state();
    let original_timestamp = before["last_cycle"]["timestamp"]
        .as_str()
        .expect("fixture should include last_cycle timestamp")
        .to_string();

    let output = Command::new(env!("CARGO_BIN_EXE_record-dispatch"))
        .args([
            "--repo-root",
            repo.path()
                .to_str()
                .expect("repo path should be valid UTF-8"),
            "--issue",
            "2511",
            "--title",
            "[Cycle Review] Cycle 493 end-of-cycle review",
            "--review-dispatch",
            "--model",
            "gpt-5.4",
        ])
        .output()
        .expect("record-dispatch should execute");
    assert!(
        output.status.success(),
        "record-dispatch failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let after = repo.read_state();
    assert_eq!(
        after.pointer("/cycle_phase/phase"),
        Some(&serde_json::json!("complete"))
    );
    assert_eq!(
        after.pointer("/last_cycle/summary"),
        Some(&serde_json::json!(
            "1 dispatch, 3 merges (PR #2505, PR #2507, PR #2509)"
        ))
    );
    assert_ne!(
        after
            .pointer("/last_cycle/timestamp")
            .and_then(Value::as_str),
        Some(original_timestamp.as_str())
    );
    assert_eq!(
        after.pointer("/dispatch_log_latest"),
        Some(&serde_json::json!(
            "#2511 [Cycle Review] Cycle 493 end-of-cycle review (cycle 493)"
        ))
    );

    let head_subject = git_output(repo.path(), ["log", "-1", "--pretty=%s"]);
    assert_eq!(
        head_subject.trim(),
        "state(record-dispatch): #2511 dispatched [cycle 493]"
    );
}

#[test]
fn record_dispatch_replays_cycle_495_close_out_flow() {
    let repo = TempRepo::new();
    repo.init_with_state(CYCLE_495_CLOSE_OUT_FIXTURE);

    let before = repo.read_state();
    let original_timestamp = before["last_cycle"]["timestamp"]
        .as_str()
        .expect("fixture should include last_cycle timestamp")
        .to_string();

    let output = Command::new(env!("CARGO_BIN_EXE_record-dispatch"))
        .args([
            "--repo-root",
            repo.path()
                .to_str()
                .expect("repo path should be valid UTF-8"),
            "--issue",
            "2521",
            "--title",
            "[Cycle Review] Cycle 495 end-of-cycle review",
            "--review-dispatch",
            "--model",
            "gpt-5.4",
        ])
        .output()
        .expect("record-dispatch should execute");
    assert!(
        output.status.success(),
        "record-dispatch failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let after = repo.read_state();
    assert_eq!(
        after.pointer("/cycle_phase/phase"),
        Some(&serde_json::json!("complete"))
    );
    assert_eq!(
        after.pointer("/last_cycle/summary"),
        Some(&serde_json::json!("1 dispatch, 0 merges"))
    );
    assert_ne!(
        after
            .pointer("/last_cycle/timestamp")
            .and_then(Value::as_str),
        Some(original_timestamp.as_str())
    );
    assert_eq!(
        after.pointer("/dispatch_log_latest"),
        Some(&serde_json::json!(
            "#2521 [Cycle Review] Cycle 495 end-of-cycle review (cycle 495)"
        ))
    );

    let head_subject = git_output(repo.path(), ["log", "-1", "--pretty=%s"]);
    assert_eq!(
        head_subject.trim(),
        "state(record-dispatch): #2521 dispatched [cycle 495]"
    );
}

#[test]
fn record_dispatch_updates_previous_cycle_worklog_when_current_cycle_worklog_is_missing() {
    let repo = TempRepo::new();
    repo.init_with_state(
        r#"{
  "agent_sessions": [],
  "in_flight_sessions": 0,
  "last_cycle": {
    "number": 513,
    "summary": "0 dispatches, 0 merges"
  },
  "cycle_phase": {
    "cycle": 514,
    "phase": "work",
    "phase_entered_at": "2026-04-18T10:00:00Z"
  },
  "review_agent": {
    "history": []
  },
  "field_inventory": {
    "fields": {
      "copilot_metrics.in_flight": {"last_refreshed": "cycle 513"},
      "copilot_metrics.dispatch_to_pr_rate": {"last_refreshed": "cycle 513"},
      "copilot_metrics.pr_merge_rate": {"last_refreshed": "cycle 513"}
    }
  },
  "copilot_metrics": {
    "total_dispatches": 0,
    "resolved": 0,
    "merged": 0,
    "closed_without_pr": 0,
    "reviewed_awaiting_eva": 0,
    "in_flight": 0,
    "produced_pr": 0,
    "pr_merge_rate": "0.0%",
    "dispatch_to_pr_rate": "0.0%",
    "dispatch_log_latest": ""
  },
  "tool_pipeline": {
    "c5_5_gate": {
      "cycle": 514,
      "status": "PASS",
      "needs_reverify": false
    }
  }
}"#,
    );
    repo.write_worklog(
        "2026-04-18",
        "094529-cycle-513-summary.md",
        "# Cycle 513 — 2026-04-18 09:45 UTC\n\n## What was done\n\n- No new dispatches.\n",
    );
    git_success(repo.path(), ["add", "docs/worklog"]);
    git_success(repo.path(), ["commit", "-m", "initial worklog"]);

    let output = Command::new(env!("CARGO_BIN_EXE_record-dispatch"))
        .args([
            "--repo-root",
            repo.path()
                .to_str()
                .expect("repo path should be valid UTF-8"),
            "--issue",
            "2586",
            "--title",
            "Cycle review dispatch",
            "--review-dispatch",
            "--model",
            "gpt-5.4",
        ])
        .output()
        .expect("record-dispatch should execute");
    assert!(
        output.status.success(),
        "record-dispatch failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let worklog = fs::read_to_string(
        repo.path()
            .join("docs/worklog/2026-04-18/094529-cycle-513-summary.md"),
    )
    .expect("worklog should be readable");
    assert!(worklog.contains("## Post-dispatch delta"));
    assert!(worklog.contains("- **In-flight agent sessions**: 1"));
    assert!(worklog.contains("- **Dispatch count**: 0 dispatches"));
    assert!(worklog.contains("- **Last-cycle summary**: 0 dispatches, 0 merges"));

    let changed_files = git_output(repo.path(), ["show", "--name-only", "--format=", "HEAD"]);
    assert!(changed_files.contains("docs/state.json"));
    assert!(changed_files.contains("docs/worklog/2026-04-18/094529-cycle-513-summary.md"));
}

struct TempRepo {
    path: PathBuf,
}

impl TempRepo {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "record-dispatch-real-flow-test-{}-{}",
            std::process::id(),
            unique
        ));
        fs::create_dir_all(path.join("docs")).expect("temp repo should be created");
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn init_with_state(&self, state_json: &str) {
        fs::write(self.path().join("docs/state.json"), state_json)
            .expect("fixture state should be written");
        git_success(self.path(), ["init"]);
        git_success(
            self.path(),
            ["config", "user.name", "Record Dispatch Integration Tests"],
        );
        git_success(
            self.path(),
            [
                "config",
                "user.email",
                "record-dispatch-integration-tests@example.com",
            ],
        );
        git_success(self.path(), ["add", "docs/state.json"]);
        git_success(self.path(), ["commit", "-m", "initial state"]);
    }

    fn read_state(&self) -> Value {
        serde_json::from_str(
            &fs::read_to_string(self.path().join("docs/state.json"))
                .expect("state file should be readable"),
        )
        .expect("state file should parse")
    }

    fn write_worklog(&self, date: &str, file_name: &str, content: &str) {
        let path = self.path().join("docs/worklog").join(date).join(file_name);
        fs::create_dir_all(path.parent().expect("worklog parent should exist"))
            .expect("worklog directory should be created");
        fs::write(path, content).expect("worklog should be written");
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn git_success<I, S>(repo_root: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let rendered_args: Vec<String> = args
        .into_iter()
        .map(|argument| argument.as_ref().to_string_lossy().into_owned())
        .collect();
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(&rendered_args)
        .output()
        .expect("git command should execute");
    assert!(
        output.status.success(),
        "git command failed (git -C {} {}): {}",
        repo_root.display(),
        rendered_args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_output<I, S>(repo_root: &Path, args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let rendered_args: Vec<String> = args
        .into_iter()
        .map(|argument| argument.as_ref().to_string_lossy().into_owned())
        .collect();
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(&rendered_args)
        .output()
        .expect("git command should execute");
    assert!(
        output.status.success(),
        "git command failed (git -C {} {}): {}",
        repo_root.display(),
        rendered_args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("git output should be UTF-8")
}
