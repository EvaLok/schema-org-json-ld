use std::path::{Path, PathBuf};
use std::process::{Command, Output};

pub fn tool_path(repo_root: &Path, tool_name: &str) -> PathBuf {
    repo_root.join("tools").join(tool_name)
}

pub fn run_tool(repo_root: &Path, tool_name: &str, args: &[&str]) -> Result<Output, String> {
    Command::new("bash")
        .arg(tool_path(repo_root, tool_name))
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute {}: {}", tool_name, error))
}

pub fn run_tool_with_timeout(
    repo_root: &Path,
    tool_name: &str,
    args: &[&str],
    timeout_seconds: u64,
) -> Result<Output, String> {
    Command::new("timeout")
        .arg(timeout_seconds.to_string())
        .arg("bash")
        .arg(tool_path(repo_root, tool_name))
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute {} with timeout: {}", tool_name, error))
}

pub fn timed_out(output: &Output) -> bool {
    output.status.code() == Some(124)
}

pub fn run_tool_json(
    repo_root: &Path,
    tool_name: &str,
    args: &[&str],
) -> Result<serde_json::Value, String> {
    let output = run_tool(repo_root, tool_name, args)?;
    if timed_out(&output) {
        let stderr = stderr_text(&output);
        return Err(if stderr.is_empty() {
            format!(
                "{} timed out (exit {})",
                tool_name,
                output.status.code().unwrap_or(-1),
            )
        } else {
            format!(
                "{} timed out (exit {}): {}",
                tool_name,
                output.status.code().unwrap_or(-1),
                stderr,
            )
        });
    }

    let stdout = stdout_text(&output);
    match serde_json::from_str(&stdout) {
        Ok(value) => Ok(value),
        Err(_error) if !output.status.success() => {
            let stderr = stderr_text(&output);
            Err(format!(
                "{} failed (exit {}): {}",
                tool_name,
                output.status.code().unwrap_or(-1),
                stderr,
            ))
        }
        Err(error) => Err(format!(
            "failed to parse {} JSON output: {}",
            tool_name, error
        )),
    }
}

pub fn stderr_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_string()
}

pub fn stdout_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn temp_repo_root(prefix: &str) -> std::path::PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "cycle-runner-runner-{}-{}-{}",
            prefix,
            std::process::id(),
            run_id
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join("tools")).expect("failed to create tools directory");
        root
    }

    fn write_tool(root: &Path, tool_name: &str, script: &str) {
        let path = root.join("tools").join(tool_name);
        fs::write(&path, script).expect("failed to write tool script");
        let mut permissions = fs::metadata(&path)
            .expect("tool script metadata should exist")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&path, permissions).expect("failed to mark tool script executable");
    }

    #[test]
    fn run_tool_json_returns_parsed_json_on_exit_zero() {
        let root = temp_repo_root("exit-zero");
        write_tool(
            &root,
            "test-tool",
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' '{\"overall\":\"pass\"}'\n",
        );

        let result = run_tool_json(&root, "test-tool", &[]).expect("JSON should be returned");

        assert_eq!(result, json!({"overall": "pass"}));
    }

    #[test]
    fn run_tool_json_returns_parsed_json_on_exit_one_with_valid_stdout() {
        let root = temp_repo_root("exit-one-valid-json");
        write_tool(
            &root,
            "test-tool",
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' '{\"overall\":\"fail\"}'\nexit 1\n",
        );

        let result = run_tool_json(&root, "test-tool", &[]).expect("JSON should be returned");

        assert_eq!(result, json!({"overall": "fail"}));
    }

    #[test]
    fn run_tool_json_returns_err_on_exit_one_with_invalid_stdout() {
        let root = temp_repo_root("exit-one-invalid-json");
        write_tool(
            &root,
            "test-tool",
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' 'not-json'\nprintf '%s\\n' 'simulated failure' >&2\nexit 1\n",
        );

        let error = run_tool_json(&root, "test-tool", &[]).expect_err("error should be returned");

        assert!(error.contains("exit 1"));
        assert!(error.contains("simulated failure"));
    }

    #[test]
    fn run_tool_json_returns_err_on_timeout_even_with_valid_stdout() {
        let root = temp_repo_root("timeout");
        write_tool(
            &root,
            "test-tool",
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' '{\"overall\":\"fail\"}'\nprintf '%s\\n' 'simulated timeout' >&2\nexit 124\n",
        );

        let error = run_tool_json(&root, "test-tool", &[]).expect_err("timeout should be returned");

        assert!(error.contains("timed out"));
        assert!(error.contains("124"));
    }
}
