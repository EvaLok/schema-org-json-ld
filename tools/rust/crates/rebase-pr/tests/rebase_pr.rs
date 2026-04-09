use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct TempFixture {
    root: PathBuf,
    repo_root: PathBuf,
    remote_root: PathBuf,
    bin_root: PathBuf,
    git_log: PathBuf,
    pr_number: u64,
}

impl TempFixture {
    fn new(name: &str, pr_number: u64) -> Self {
        let root = std::env::temp_dir().join(format!(
            "rebase-pr-{name}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        ));
        let repo_root = root.join("repo");
        let remote_root = root.join("remote.git");
        let bin_root = root.join("bin");
        let git_log = root.join("git.log");
        fs::create_dir_all(&repo_root).expect("repo root should exist");
        fs::create_dir_all(&bin_root).expect("bin root should exist");
        Self {
            root,
            repo_root,
            remote_root,
            bin_root,
            git_log,
            pr_number,
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
        git(&self.repo_root, ["config", "user.name", "Rebase PR Tests"]);
        git(
            &self.repo_root,
            ["config", "user.email", "rebase-pr-tests@example.com"],
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

        fs::write(self.repo_root.join("README.md"), "initial\n").expect("README should exist");
        git(&self.repo_root, ["add", "README.md"]);
        git(&self.repo_root, ["commit", "-m", "initial"]);
        git(&self.repo_root, ["push", "-u", "origin", "master"]);
    }

    fn create_stale_pr_branch(&self, head_ref_name: &str) -> (String, String) {
        git(&self.repo_root, ["checkout", "-b", head_ref_name]);
        fs::write(self.repo_root.join("feature.txt"), "feature\n").expect("feature file");
        git(&self.repo_root, ["add", "feature.txt"]);
        git(&self.repo_root, ["commit", "-m", "feature"]);
        let original_head = git_stdout(&self.repo_root, ["rev-parse", "HEAD"]);
        git(&self.repo_root, ["push", "-u", "origin", head_ref_name]);
        self.update_pull_ref(&original_head);

        git(&self.repo_root, ["checkout", "master"]);
        fs::write(self.repo_root.join("README.md"), "initial\nmaster change\n")
            .expect("README update");
        git(&self.repo_root, ["add", "README.md"]);
        git(&self.repo_root, ["commit", "-m", "master change"]);
        let master_tip = git_stdout(&self.repo_root, ["rev-parse", "HEAD"]);
        git(&self.repo_root, ["push", "origin", "master"]);
        (original_head, master_tip)
    }

    fn create_up_to_date_pr_branch(&self, head_ref_name: &str) -> (String, String) {
        fs::write(self.repo_root.join("README.md"), "initial\nmaster change\n")
            .expect("README update");
        git(&self.repo_root, ["add", "README.md"]);
        git(&self.repo_root, ["commit", "-m", "master change"]);
        let master_tip = git_stdout(&self.repo_root, ["rev-parse", "HEAD"]);
        git(&self.repo_root, ["push", "origin", "master"]);

        git(&self.repo_root, ["checkout", "-b", head_ref_name]);
        fs::write(self.repo_root.join("feature.txt"), "feature\n").expect("feature file");
        git(&self.repo_root, ["add", "feature.txt"]);
        git(&self.repo_root, ["commit", "-m", "feature"]);
        let feature_head = git_stdout(&self.repo_root, ["rev-parse", "HEAD"]);
        git(&self.repo_root, ["push", "-u", "origin", head_ref_name]);
        self.update_pull_ref(&feature_head);
        git(&self.repo_root, ["checkout", "master"]);

        (feature_head, master_tip)
    }

    fn update_pull_ref(&self, sha: &str) {
        git_with_git_dir(
            &self.remote_root,
            [
                "update-ref",
                &format!("refs/pull/{}/head", self.pr_number),
                sha,
            ],
        );
    }

    fn install_fake_tools(&self, head_ref_name: &str, head_sha: &str) {
        let fake_gh = format!(
            r#"#!/usr/bin/env bash
set -euo pipefail
if [ "$1" = "pr" ] && [ "$2" = "view" ] && [ "$3" = "{pr_number}" ] && [ "$4" = "--json" ] && [ "$5" = "headRefName,headRefOid" ]; then
	printf '%s\n' '{{"headRefName":"{head_ref_name}","headRefOid":"{head_sha}"}}'
	exit 0
fi
echo "unexpected gh args: $*" >&2
exit 1
"#,
            pr_number = self.pr_number,
            head_ref_name = head_ref_name,
            head_sha = head_sha
        );
        fs::write(self.bin_root.join("gh"), fake_gh).expect("fake gh should be written");
        make_executable(&self.bin_root.join("gh"));

        let real_git = real_git_path();
        let fake_git = format!(
            r#"#!/usr/bin/env bash
set -euo pipefail
printf '%s\n' "$*" >> '{git_log}'
exec '{real_git}' "$@"
"#,
            git_log = self.git_log.display(),
            real_git = real_git
        );
        fs::write(self.bin_root.join("git"), fake_git).expect("fake git should be written");
        make_executable(&self.bin_root.join("git"));
    }

    fn path_with_fake_bins(&self) -> String {
        let current_path = std::env::var("PATH").expect("PATH should exist");
        format!("{}:{}", self.bin_root.display(), current_path)
    }
}

impl Drop for TempFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
fn rebases_stale_pr_onto_master_and_prints_new_head_sha() {
    let fixture = TempFixture::new("happy-path", 77);
    let head_ref_name = "copilot/pr-77";
    fixture.init_repo();
    let (original_head, master_tip) = fixture.create_stale_pr_branch(head_ref_name);
    fixture.install_fake_tools(head_ref_name, &original_head);

    let output = Command::new(binary_path("rebase-pr"))
        .args([
            "--pr",
            "77",
            "--repo-root",
            fixture.repo_root.to_str().unwrap(),
        ])
        .env("PATH", fixture.path_with_fake_bins())
        .output()
        .expect("rebase-pr should execute");

    assert!(
        output.status.success(),
        "rebase-pr failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let rebased_sha = stdout
        .strip_prefix("Rebased PR #77 onto origin/master at ")
        .expect("success message should include rebased sha")
        .to_string();

    let remote_head = git_stdout(
        &fixture.repo_root,
        ["rev-parse", &format!("origin/{head_ref_name}")],
    );
    let merge_base = git_stdout(
        &fixture.repo_root,
        [
            "merge-base",
            "origin/master",
            &format!("origin/{head_ref_name}"),
        ],
    );
    let current_branch = git_stdout(&fixture.repo_root, ["rev-parse", "--abbrev-ref", "HEAD"]);
    let local_pr_branch = git_stdout(&fixture.repo_root, ["branch", "--list", "pr-77"]);
    let git_log = fs::read_to_string(&fixture.git_log).expect("git log should be readable");

    assert_eq!(rebased_sha, remote_head);
    assert_eq!(merge_base, master_tip);
    assert_eq!(current_branch, "master");
    assert!(
        local_pr_branch.trim().is_empty(),
        "local pr branch should be deleted"
    );
    assert_ne!(rebased_sha, original_head);
    assert!(
        git_log.contains("push --force-with-lease origin pr-77:copilot/pr-77"),
        "expected force-with-lease push in log:\n{git_log}"
    );
}

#[test]
fn short_circuits_when_pr_is_already_up_to_date() {
    let fixture = TempFixture::new("already-up-to-date", 88);
    let head_ref_name = "copilot/pr-88";
    fixture.init_repo();
    let (feature_head, master_tip) = fixture.create_up_to_date_pr_branch(head_ref_name);
    fixture.install_fake_tools(head_ref_name, &feature_head);

    let output = Command::new(binary_path("rebase-pr"))
        .args([
            "--pr",
            "88",
            "--repo-root",
            fixture.repo_root.to_str().unwrap(),
        ])
        .env("PATH", fixture.path_with_fake_bins())
        .output()
        .expect("rebase-pr should execute");

    assert!(
        output.status.success(),
        "rebase-pr failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let remote_head = git_stdout(
        &fixture.repo_root,
        ["rev-parse", &format!("origin/{head_ref_name}")],
    );
    let merge_base = git_stdout(
        &fixture.repo_root,
        [
            "merge-base",
            "origin/master",
            &format!("origin/{head_ref_name}"),
        ],
    );
    let current_branch = git_stdout(&fixture.repo_root, ["rev-parse", "--abbrev-ref", "HEAD"]);
    let local_pr_branch = git_stdout(&fixture.repo_root, ["branch", "--list", "pr-88"]);
    let git_log = fs::read_to_string(&fixture.git_log).expect("git log should be readable");

    assert_eq!(
        stdout,
        "Already up to date with origin/master, no rebase needed"
    );
    assert_eq!(remote_head, feature_head);
    assert_eq!(merge_base, master_tip);
    assert_eq!(current_branch, "master");
    assert!(
        local_pr_branch.trim().is_empty(),
        "local pr branch should be deleted"
    );
    assert!(
        !git_log.contains("push --force-with-lease"),
        "did not expect a force push in log:\n{git_log}"
    );
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
    let output = Command::new("git")
        .current_dir(cwd)
        .args(args)
        .output()
        .expect("git should execute");
    assert!(
        output.status.success(),
        "git failed in {}: {}",
        cwd.display(),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_stdout<I, S>(cwd: &Path, args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let output = Command::new("git")
        .current_dir(cwd)
        .args(args)
        .output()
        .expect("git should execute");
    assert!(
        output.status.success(),
        "git stdout command failed in {}: {}",
        cwd.display(),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

fn git_with_git_dir<I, S>(git_dir: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let output = Command::new("git")
        .arg(format!("--git-dir={}", git_dir.display()))
        .args(args)
        .output()
        .expect("git should execute");
    assert!(
        output.status.success(),
        "git --git-dir failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn real_git_path() -> String {
    let output = Command::new("sh")
        .args(["-c", "command -v git"])
        .output()
        .expect("sh should execute");
    assert!(output.status.success(), "should locate git");
    String::from_utf8_lossy(&output.stdout).trim().to_string()
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
