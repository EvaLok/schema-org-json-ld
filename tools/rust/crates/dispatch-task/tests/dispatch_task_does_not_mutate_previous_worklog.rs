use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[test]
fn dispatch_task_does_not_mutate_previous_worklog() {
    let fixture = TempFixture::new();
    fixture.init_repo();

    let worklog_path = fixture
        .repo_root
        .join("docs/worklog/2099-01-01/000000-cycle-99-frozen.md");
    let before_mtime = fs::metadata(&worklog_path)
        .expect("worklog metadata should be readable")
        .modified()
        .expect("worklog mtime should be readable");
    let before_sha = sha256(&worklog_path);
    let before_content = fs::read_to_string(&worklog_path).expect("worklog should be readable");

    let output = Command::new(env!("CARGO_BIN_EXE_dispatch-task"))
        .args([
            "--title",
            "test",
            "--body-file",
            fixture
                .body_file
                .to_str()
                .expect("body file path should be utf-8"),
            "--repo-root",
            fixture
                .repo_root
                .to_str()
                .expect("repo root path should be utf-8"),
        ])
        .env("PATH", fixture.path_with_fake_gh())
        .output()
        .expect("dispatch-task should execute");

    assert!(
        output.status.success(),
        "dispatch-task failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let after_mtime = fs::metadata(&worklog_path)
        .expect("worklog metadata should still be readable")
        .modified()
        .expect("worklog mtime should still be readable");
    let after_sha = sha256(&worklog_path);
    let after_content =
        fs::read_to_string(&worklog_path).expect("worklog should still be readable");

    assert_eq!(before_mtime, after_mtime);
    assert_eq!(before_sha, after_sha);
    assert_eq!(before_content, after_content);
    assert!(after_content.contains("- **In-flight agent sessions**: 0"));
}

struct TempFixture {
    root: PathBuf,
    repo_root: PathBuf,
    remote_root: PathBuf,
    bin_root: PathBuf,
    body_file: PathBuf,
}

impl TempFixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "dispatch-task-integration-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        ));
        let repo_root = root.join("repo");
        let remote_root = root.join("remote.git");
        let bin_root = root.join("bin");
        fs::create_dir_all(&repo_root).expect("repo root should exist");
        fs::create_dir_all(&bin_root).expect("bin root should exist");
        Self {
            body_file: repo_root.join("body.md"),
            root,
            repo_root,
            remote_root,
            bin_root,
        }
    }

    fn init_repo(&self) {
        git(
            &self.root,
            ["init", "--bare", self.remote_root.to_str().unwrap()],
        );
        git(
            &self.root,
            ["init", "-b", "master", self.repo_root.to_str().unwrap()],
        );
        git(
            &self.repo_root,
            ["config", "user.name", "Dispatch Task Integration Tests"],
        );
        git(
            &self.repo_root,
            [
                "config",
                "user.email",
                "dispatch-task-integration@example.com",
            ],
        );
        git(
            &self.repo_root,
            [
                "remote",
                "add",
                "origin",
                self.remote_root.to_str().unwrap(),
            ],
        );

        fs::create_dir_all(self.repo_root.join("docs/worklog/2099-01-01"))
            .expect("worklog dir should exist");
        fs::create_dir_all(self.repo_root.join("tools")).expect("tools dir should exist");

        fs::write(
            self.repo_root.join("docs/state.json"),
            r#"{
  "agent_sessions": [
    {
      "issue": 599,
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
    "number": 100
  },
  "cycle_phase": {
    "cycle": 100,
    "phase": "work",
    "phase_entered_at": "2026-03-07T12:00:00Z"
  },
  "field_inventory": {
    "fields": {
      "in_flight_sessions": {
        "last_refreshed": "cycle 99"
      }
    }
  },
  "review_agent": {
    "history": []
  }
}
"#,
        )
        .expect("state file should be written");
        fs::write(
            self.repo_root
                .join("docs/worklog/2099-01-01/000000-cycle-99-frozen.md"),
            "# Cycle 99 — frozen\n- **In-flight agent sessions**: 0\n",
        )
        .expect("worklog should be written");
        fs::write(
            self.repo_root.join("tools/config.json"),
            r#"{"default_model":"gpt-5.4"}"#,
        )
        .expect("config should be written");
        fs::write(
            self.repo_root.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nexit 0\n",
        )
        .expect("pipeline-check script should be written");
        make_executable(&self.repo_root.join("tools/pipeline-check"));

        fs::write(&self.body_file, "dispatch body\n").expect("body file should be written");
        fs::write(
            self.bin_root.join("gh"),
            "#!/usr/bin/env bash\nset -euo pipefail\ncat >/dev/null\nprintf '%s\\n' '{\"number\":602,\"html_url\":\"https://github.com/EvaLok/schema-org-json-ld/issues/602\"}'\n",
        )
        .expect("fake gh should be written");
        make_executable(&self.bin_root.join("gh"));

        git(&self.repo_root, ["add", "."]);
        git(&self.repo_root, ["commit", "-m", "initial state"]);
        git(&self.repo_root, ["push", "-u", "origin", "master"]);
    }

    fn path_with_fake_gh(&self) -> String {
        let current_path = std::env::var("PATH").expect("PATH should exist");
        format!("{}:{}", self.bin_root.display(), current_path)
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

fn sha256(path: &Path) -> String {
    let output = Command::new("sha256sum")
        .arg(path)
        .output()
        .expect("sha256sum should execute");
    assert!(output.status.success(), "sha256sum failed");
    String::from_utf8_lossy(&output.stdout)
        .split_whitespace()
        .next()
        .expect("sha256sum output should include a digest")
        .to_string()
}

#[cfg(unix)]
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .expect("metadata should be readable")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("permissions should be updated");
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) {}
