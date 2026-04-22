use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct TempFixture {
    root: PathBuf,
    repo_root: PathBuf,
    remote_root: PathBuf,
}

impl TempFixture {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "record-dispatch-push-{name}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        ));
        let repo_root = root.join("repo");
        let remote_root = root.join("remote.git");
        fs::create_dir_all(&root).expect("temp root should exist");
        Self {
            root,
            repo_root,
            remote_root,
        }
    }

    fn init_repo(&self) {
        git(
            &self.root,
            ["init", "--bare", self.remote_root.to_str().unwrap()],
        );
        git(
            &self.remote_root,
            ["symbolic-ref", "HEAD", "refs/heads/master"],
        );
        git(
            &self.root,
            [
                "clone",
                self.remote_root.to_str().unwrap(),
                self.repo_root.to_str().unwrap(),
            ],
        );
        git(
            &self.repo_root,
            ["config", "user.name", "Record Dispatch Tests"],
        );
        git(
            &self.repo_root,
            ["config", "user.email", "record-dispatch-tests@example.com"],
        );

        fs::create_dir_all(self.repo_root.join("docs")).expect("docs dir should exist");
        fs::write(
            self.repo_root.join("docs/state.json"),
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
}
"#,
        )
        .expect("state should be written");
        git(&self.repo_root, ["add", "docs/state.json"]);
        git(&self.repo_root, ["commit", "-m", "initial state"]);
        git(&self.repo_root, ["push", "-u", "origin", "HEAD:master"]);
    }

    fn remote_head(&self) -> String {
        git_stdout(&self.remote_root, ["rev-parse", "HEAD"])
    }
}

impl Drop for TempFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn git<I, S>(cwd: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let rendered_args: Vec<String> = args
        .into_iter()
        .map(|value| value.as_ref().to_string_lossy().into_owned())
        .collect();
    let output = Command::new("git")
        .current_dir(cwd)
        .args(&rendered_args)
        .output()
        .expect("git should execute");
    assert!(
        output.status.success(),
        "git {} failed in {}: {}",
        rendered_args.join(" "),
        cwd.display(),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_stdout<I, S>(cwd: &Path, args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let rendered_args: Vec<String> = args
        .into_iter()
        .map(|value| value.as_ref().to_string_lossy().into_owned())
        .collect();
    let output = Command::new("git")
        .current_dir(cwd)
        .args(&rendered_args)
        .output()
        .expect("git should execute");
    assert!(output.status.success());
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[test]
fn record_dispatch_pushes_state_commit_to_remote() {
    let fixture = TempFixture::new("review-dispatch");
    fixture.init_repo();
    let remote_before = fixture.remote_head();

    let output = Command::new(env!("CARGO_BIN_EXE_record-dispatch"))
        .args([
            "--repo-root",
            fixture.repo_root.to_str().unwrap(),
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
        "record-dispatch failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_ne!(fixture.remote_head(), remote_before);
    assert_eq!(
        git_stdout(&fixture.repo_root, ["log", "-1", "--pretty=%s"]),
        "state(record-dispatch): #2586 dispatched [cycle 514]"
    );
}
