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
            "process-merge-push-{name}-{}-{}",
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
            ["config", "user.name", "Process Merge Tests"],
        );
        git(
            &self.repo_root,
            ["config", "user.email", "process-merge-tests@example.com"],
        );

        fs::create_dir_all(self.repo_root.join("docs")).expect("docs dir should exist");
        fs::write(
            self.repo_root.join("docs/state.json"),
            r#"{
  "agent_sessions": [
    {
      "issue": 667,
      "title": "Dispatched issue 667",
      "dispatched_at": "2026-03-05T10:00:00Z",
      "model": "gpt-5.4",
      "status": "in_flight"
    },
    {
      "issue": 668,
      "title": "Already linked",
      "dispatched_at": "2026-03-05T11:00:00Z",
      "model": "gpt-5.4",
      "status": "in_flight",
      "pr": 669
    }
  ],
  "in_flight_sessions": 3,
  "last_cycle": {
    "number": 164,
    "timestamp": "2026-03-05T09:00:00Z",
    "summary": "0 dispatches, 0 merges"
  },
  "cycle_phase": {"cycle": 164},
  "copilot_metrics": {
    "closed_without_merge": 1,
    "closed_without_pr": 1,
    "dispatch_to_pr_rate": "84/85",
    "in_flight": 3,
    "merged": 80,
    "pr_merge_rate": "80/84",
    "produced_pr": 84,
    "resolved": 82,
    "total_dispatches": 85
  },
  "field_inventory": {
    "fields": {
      "copilot_metrics.in_flight": {"last_refreshed": "cycle 163"},
      "copilot_metrics.pr_merge_rate": {"last_refreshed": "cycle 163"},
      "copilot_metrics.dispatch_to_pr_rate": {"last_refreshed": "cycle 163"},
      "in_flight_sessions": {"last_refreshed": "cycle 163"}
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
fn process_merge_pushes_state_commit_to_remote() {
    let fixture = TempFixture::new("basic");
    fixture.init_repo();
    let remote_before = fixture.remote_head();

    let output = Command::new(binary_path("process-merge"))
        .args([
            "--repo-root",
            fixture.repo_root.to_str().unwrap(),
            "--prs",
            "668",
            "--issues",
            "667",
            "--merged-at",
            "2026-03-07T13:00:00Z",
        ])
        .output()
        .expect("process-merge should execute");

    assert!(
        output.status.success(),
        "process-merge failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_ne!(fixture.remote_head(), remote_before);
    assert_eq!(
        git_stdout(&fixture.repo_root, ["log", "-1", "--pretty=%s"]),
        "state(process-merge): PR #668 merged [cycle 164]"
    );
}
