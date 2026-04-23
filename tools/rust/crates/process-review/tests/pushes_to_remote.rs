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
            "process-review-push-{name}-{}-{}",
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
            ["config", "user.name", "Process Review Tests"],
        );
        git(
            &self.repo_root,
            ["config", "user.email", "process-review-tests@example.com"],
        );

        fs::create_dir_all(self.repo_root.join("docs/reviews")).expect("review dir should exist");
        fs::write(
            self.repo_root.join("docs/state.json"),
            r#"{
  "last_cycle": {"number": 500},
  "cycle_phase": {"cycle": 500},
  "review_agent": {
    "last_review_cycle": 499,
    "history": []
  },
  "field_inventory": {
    "fields": {
      "review_agent": {"last_refreshed": "cycle 499"}
    }
  }
}
"#,
        )
        .expect("state should be written");
        fs::write(
            self.repo_root.join("docs/reviews/cycle-500.md"),
            r#"# Cycle 500 Review

## Findings

1. **[review-accounting] Review accounting finding**

## Complacency score

2/5
"#,
        )
        .expect("review should be written");
        git(
            &self.repo_root,
            ["add", "docs/state.json", "docs/reviews/cycle-500.md"],
        );
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
fn process_review_pushes_state_commit_to_remote() {
    let fixture = TempFixture::new("basic");
    fixture.init_repo();
    let remote_before = fixture.remote_head();

    let output = Command::new(binary_path("process-review"))
        .args([
            "--repo-root",
            fixture.repo_root.to_str().unwrap(),
            "--review-file",
            "docs/reviews/cycle-500.md",
            "--review-issue",
            "900",
            "--actioned",
            "1",
        ])
        .output()
        .expect("process-review should execute");

    assert!(
        output.status.success(),
        "process-review failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_ne!(fixture.remote_head(), remote_before);
    assert!(git_stdout(&fixture.repo_root, ["log", "-1", "--pretty=%s"])
        .starts_with("state(process-review):"));
}
