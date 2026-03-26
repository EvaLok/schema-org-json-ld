use std::path::{Path, PathBuf};
use std::process::{Command, Output};

fn tool_path(repo_root: &Path, tool_name: &str) -> PathBuf {
    repo_root.join("tools").join(tool_name)
}

pub fn run_tool(repo_root: &Path, tool_name: &str, args: &[&str]) -> Result<Output, String> {
    Command::new("bash")
        .arg(tool_path(repo_root, tool_name))
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute {}: {}", tool_name, error))
}

pub fn stderr_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stderr).trim().to_string()
}

pub fn stdout_text(output: &Output) -> String {
    String::from_utf8_lossy(&output.stdout).trim().to_string()
}
