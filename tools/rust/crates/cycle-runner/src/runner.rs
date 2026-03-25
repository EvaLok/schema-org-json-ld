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
    if !output.status.success() {
        let stderr = stderr_text(&output);
        return Err(format!(
            "{} failed (exit {}): {}",
            tool_name,
            output.status.code().unwrap_or(-1),
            stderr,
        ));
    }
    let stdout = stdout_text(&output);
    serde_json::from_str(&stdout)
        .map_err(|error| format!("failed to parse {} JSON output: {}", tool_name, error))
}

pub fn stderr_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_string()
}

pub fn stdout_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
