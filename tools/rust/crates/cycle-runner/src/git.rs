use std::path::Path;
use std::process::Command;

pub fn add_and_commit(repo_root: &Path, paths: &[&str], message: &str) -> Result<String, String> {
    if !has_changes(repo_root, paths)? {
        eprintln!("No changes to commit, skipping");
        return Ok(String::new());
    }

    let mut add_cmd = Command::new("git");
    add_cmd.arg("-C").arg(repo_root).arg("add");
    for path in paths {
        add_cmd.arg(path);
    }
    let add_output = add_cmd
        .output()
        .map_err(|error| format!("failed to execute git add: {}", error))?;
    if !add_output.status.success() {
        let stderr = String::from_utf8_lossy(&add_output.stderr)
            .trim()
            .to_string();
        return Err(format!("git add failed: {}", stderr));
    }

    let commit_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()
        .map_err(|error| format!("failed to execute git commit: {}", error))?;
    if !commit_output.status.success() {
        let stderr = String::from_utf8_lossy(&commit_output.stderr)
            .trim()
            .to_string();
        if stderr.contains("nothing to commit") {
            eprintln!("Nothing to commit, skipping");
            return Ok(String::new());
        }
        return Err(format!("git commit failed: {}", stderr));
    }

    let sha_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("rev-parse")
        .arg("--short=7")
        .arg("HEAD")
        .output()
        .map_err(|error| format!("failed to execute git rev-parse: {}", error))?;

    Ok(String::from_utf8_lossy(&sha_output.stdout)
        .trim()
        .to_string())
}

pub fn push(repo_root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .map_err(|error| format!("failed to execute git push: {}", error))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        if stderr.contains("Everything up-to-date") {
            eprintln!("Already up to date, nothing to push");
            return Ok(());
        }
        return Err(format!("git push failed: {}", stderr));
    }
    Ok(())
}

fn has_changes(repo_root: &Path, paths: &[&str]) -> Result<bool, String> {
    let mut cmd = Command::new("git");
    cmd.arg("-C")
        .arg(repo_root)
        .arg("status")
        .arg("--porcelain");
    for path in paths {
        cmd.arg(path);
    }
    let output = cmd
        .output()
        .map_err(|error| format!("failed to execute git status: {}", error))?;
    Ok(!String::from_utf8_lossy(&output.stdout).trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn setup_temp_repo(name: &str) -> std::path::PathBuf {
        let dir = std::env::temp_dir().join(format!("cycle-runner-git-test-{}", name));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .arg("init")
            .output()
            .unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["config", "user.email", "test@test.com"])
            .output()
            .unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["config", "user.name", "Test"])
            .output()
            .unwrap();
        dir
    }

    #[test]
    fn has_changes_detects_new_files() {
        let dir = setup_temp_repo("has-changes");
        fs::write(dir.join("test.txt"), "hello").unwrap();
        assert!(has_changes(&dir, &["."]).unwrap());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn has_changes_reports_false_on_clean_repo() {
        let dir = setup_temp_repo("clean");
        assert!(!has_changes(&dir, &["."]).unwrap());
        let _ = fs::remove_dir_all(&dir);
    }
}
