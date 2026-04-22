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
            "cycle-start-push-{name}-{}-{}",
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
            ["config", "user.name", "Cycle Start Tests"],
        );
        git(
            &self.repo_root,
            ["config", "user.email", "cycle-start-tests@example.com"],
        );

        fs::create_dir_all(self.repo_root.join("docs")).expect("docs dir should exist");
        fs::write(
            self.repo_root.join("docs/state.json"),
            r#"{
  "last_cycle": {
    "number": 40,
    "issue": 400,
    "timestamp": "2026-04-20T00:00:00Z"
  },
  "cycle_phase": {
    "cycle": 41,
    "phase": "work",
    "phase_entered_at": "2026-04-21T00:00:00Z"
  },
  "cycle_issues": [400],
  "field_inventory": {
    "fields": {
      "last_cycle": {"last_refreshed": "cycle 40"},
      "cycle_issues": {
        "cadence": "every cycle",
        "last_refreshed": "cycle 40"
      },
      "previous_cycle_issue": {
        "cadence": "every cycle",
        "last_refreshed": "cycle 40"
      }
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
fn cycle_start_pushes_resume_commit_to_remote() {
    let fixture = TempFixture::new("resume");
    fixture.init_repo();
    let remote_before = fixture.remote_head();

    let output = Command::new(binary_path("cycle-start"))
        .args([
            "--repo-root",
            fixture.repo_root.to_str().unwrap(),
            "--issue",
            "410",
            "--model",
            "gpt-5.4",
            "--json",
        ])
        .output()
        .expect("cycle-start should execute");

    assert!(
        output.status.success(),
        "cycle-start failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_ne!(fixture.remote_head(), remote_before);
    assert_eq!(
        git_stdout(&fixture.repo_root, ["log", "-1", "--pretty=%s"]),
        "state(cycle-start): resume cycle 41, issue #410 [cycle 41]"
    );
}
