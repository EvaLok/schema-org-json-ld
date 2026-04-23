use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn exits_non_zero_off_master_with_guard_message() {
    let repo_root = temp_repo_root("post-step-branch-guard");
    set_head_branch(&repo_root, "feature/test-branch");
    fs::write(
        repo_root.join("docs/state.json"),
        "{\"last_cycle\":{\"number\":198}}\n",
    )
    .expect("state.json should be written");

    let output = Command::new(env!("CARGO_BIN_EXE_post-step"))
        .args([
            "--issue",
            "834",
            "--step",
            "0",
            "--title",
            "Check for input-from-eva issues",
            "--body",
            "Found 2 open issues.",
            "--repo-root",
        ])
        .arg(&repo_root)
        .output()
        .expect("post-step should execute");

    assert!(
        !output.status.success(),
        "expected non-zero exit, stdout={}, stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains(
        "post-step refuses to run off master: HEAD=feature/test-branch; pass --force-branch to override"
    ));
}

fn temp_repo_root(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("{prefix}-{unique}"));
    fs::create_dir_all(path.join("docs")).expect("docs directory should be created");
    assert_git_success(&path, ["init"]);
    set_head_branch(&path, "master");
    path
}

fn set_head_branch(repo_root: &Path, branch: &str) {
    let ref_name = format!("refs/heads/{branch}");
    assert_git_success(repo_root, ["symbolic-ref", "HEAD", &ref_name]);
}

fn assert_git_success<I, S>(repo_root: &Path, args: I)
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
