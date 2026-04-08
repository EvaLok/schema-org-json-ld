use crate::runner;
use std::path::Path;

pub fn post_step(
    repo_root: &Path,
    issue: u64,
    step: &str,
    title: &str,
    body: &str,
    dry_run: bool,
) -> Result<(), String> {
    if dry_run {
        eprintln!(
            "[dry-run] Would post step {} on issue #{}: {}",
            step, issue, title
        );
        return Ok(());
    }

    let issue_str = issue.to_string();
    let output = runner::run_tool(
        repo_root,
        "post-step",
        &[
            "--issue", &issue_str, "--step", step, "--title", title, "--body", body,
        ],
    )?;

    if !output.status.success() {
        let stderr = runner::stderr_text(&output);
        // Don't fail on duplicate step (already posted in a previous run)
        if stderr.contains("already posted") {
            eprintln!("Step {} already posted on issue #{}, skipping", step, issue);
            return Ok(());
        }
        return Err(format!("post-step {} failed: {}", step, stderr));
    }

    eprintln!("{}", runner::stdout_text(&output));
    Ok(())
}
