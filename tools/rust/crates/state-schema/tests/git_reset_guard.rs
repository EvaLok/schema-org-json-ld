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
            "git-reset-guard-{name}-{}-{}",
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
        git(&self.repo_root, ["config", "user.name", "Guard Tests"]);
        git(
            &self.repo_root,
            ["config", "user.email", "guard-tests@example.com"],
        );
        fs::create_dir_all(self.repo_root.join("docs")).expect("docs dir should exist");
        fs::write(
            self.repo_root.join("docs/state.json"),
            "{\n  \"cycle\": 1\n}\n",
        )
        .expect("state should be written");
        git(&self.repo_root, ["add", "docs/state.json"]);
        git(&self.repo_root, ["commit", "-m", "initial state"]);
        git(&self.repo_root, ["push", "-u", "origin", "HEAD:master"]);
    }
}

impl Drop for TempFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../..")
        .canonicalize()
        .expect("repo root should resolve")
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

#[test]
fn refuses_hard_reset_when_unpushed_commits_exist() {
    let fixture = TempFixture::new("refuse");
    fixture.init_repo();
    fs::write(
        fixture.repo_root.join("docs/state.json"),
        "{\n  \"cycle\": 2\n}\n",
    )
    .expect("state should update");
    git(&fixture.repo_root, ["add", "docs/state.json"]);
    git(&fixture.repo_root, ["commit", "-m", "local only"]);

    let output = Command::new("bash")
        .arg(repo_root().join("tools/git-reset-guard"))
        .args(["--hard", "origin/master"])
        .current_dir(&fixture.repo_root)
        .output()
        .expect("git-reset-guard should execute");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("local only"));
    assert!(stderr.contains("git-reset-guard: refusing --hard; unpushed commits present"));
}

#[test]
fn allows_hard_reset_when_branch_is_clean() {
    let fixture = TempFixture::new("clean");
    fixture.init_repo();

    let output = Command::new("bash")
        .arg(repo_root().join("tools/git-reset-guard"))
        .args(["--hard", "origin/master"])
        .current_dir(&fixture.repo_root)
        .output()
        .expect("git-reset-guard should execute");

    assert!(
        output.status.success(),
        "git-reset-guard failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}
