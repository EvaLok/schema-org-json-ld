use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct TempFixture {
    root: PathBuf,
    repo_root: PathBuf,
    remote_root: PathBuf,
    bin_root: PathBuf,
}

impl TempFixture {
    fn new(name: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "merge-pr-push-{name}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        ));
        let repo_root = root.join("repo");
        let remote_root = root.join("remote.git");
        let bin_root = root.join("bin");
        fs::create_dir_all(&root).expect("temp root should exist");
        fs::create_dir_all(&bin_root).expect("bin dir should exist");
        Self {
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
        git(&self.repo_root, ["config", "user.name", "Merge PR Tests"]);
        git(
            &self.repo_root,
            ["config", "user.email", "merge-pr-tests@example.com"],
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
    }
  ],
  "in_flight_sessions": 1,
  "last_cycle": {
    "number": 164,
    "timestamp": "2026-03-05T09:00:00Z",
    "summary": "0 dispatches, 0 merges"
  },
  "cycle_phase": {"cycle": 164},
  "field_inventory": {
    "fields": {
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

        git(&self.repo_root, ["checkout", "-b", "copilot/merged"]);
        fs::write(self.repo_root.join("feature.txt"), "feature\n").expect("feature file");
        git(&self.repo_root, ["add", "feature.txt"]);
        git(&self.repo_root, ["commit", "-m", "feature"]);
        git(&self.repo_root, ["push", "-u", "origin", "HEAD"]);
        git(&self.repo_root, ["checkout", "master"]);

        fs::create_dir_all(self.repo_root.join("tools")).expect("tools dir should exist");
        let wrapper = format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nexec '{}' \"$@\"\n",
            binary_path("process-merge")
        );
        fs::write(self.repo_root.join("tools/process-merge"), wrapper)
            .expect("process-merge wrapper should be written");
        make_executable(&self.repo_root.join("tools/process-merge"));

        let gh = r#"#!/usr/bin/env bash
set -euo pipefail
if [ "$1" = "pr" ] && [ "$2" = "view" ] && [ "$3" = "77" ]; then
printf '%s\n' '{"state":"MERGED","isDraft":false,"mergeable":null,"headRefName":"copilot/merged"}'
exit 0
fi
echo "unexpected gh args: $*" >&2
exit 1
"#;
        fs::write(self.bin_root.join("gh"), gh).expect("fake gh should be written");
        make_executable(&self.bin_root.join("gh"));
    }

    fn path_with_fake_bins(&self) -> String {
        let current_path = std::env::var("PATH").expect("PATH should exist");
        format!("{}:{}", self.bin_root.display(), current_path)
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

fn make_executable(path: &Path) {
    #[cfg(unix)]
    {
        let metadata = fs::metadata(path).expect("metadata should be readable");
        let mut permissions = metadata.permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).expect("permissions should be updated");
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
fn merge_pr_pushes_process_merge_commit_to_remote() {
    let fixture = TempFixture::new("already-merged");
    fixture.init_repo();
    let remote_before = fixture.remote_head();

    let output = Command::new(binary_path("merge-pr"))
        .args([
            "--pr",
            "77",
            "--issue",
            "667",
            "--repo-root",
            fixture.repo_root.to_str().unwrap(),
        ])
        .env("PATH", fixture.path_with_fake_bins())
        .output()
        .expect("merge-pr should execute");

    assert!(
        output.status.success(),
        "merge-pr failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_ne!(fixture.remote_head(), remote_before);
    assert_eq!(
        git_stdout(&fixture.repo_root, ["log", "-1", "--pretty=%s"]),
        "state(process-merge): PR #77 merged [cycle 164]"
    );

    let delete_check = Command::new("git")
        .arg("--git-dir")
        .arg(&fixture.remote_root)
        .args(["show-ref", "--verify", "refs/heads/copilot/merged"])
        .output()
        .expect("git show-ref should execute");
    assert!(
        !delete_check.status.success(),
        "remote branch should be deleted"
    );
}
