use clap::Parser;
use serde::Deserialize;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Parser)]
#[command(name = "rebase-pr")]
struct Cli {
    /// Pull request number to rebase
    #[arg(long)]
    pr: u64,

    /// Repository root
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Print planned actions without modifying git state
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ExecutionResult {
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
}

trait CommandRunner {
    fn gh(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String>;
    fn git(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String>;
}

struct ProcessRunner;

impl CommandRunner for ProcessRunner {
    fn gh(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
        run_command("gh", repo_root, args)
    }

    fn git(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
        run_command("git", repo_root, args)
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PullRequestHead {
    head_ref_name: String,
    head_ref_oid: String,
}

struct LocalBranchGuard<'a> {
    runner: &'a dyn CommandRunner,
    repo_root: PathBuf,
    local_branch: String,
    should_return_to_previous_branch: bool,
    active: bool,
}

impl<'a> LocalBranchGuard<'a> {
    fn new(runner: &'a dyn CommandRunner, repo_root: &Path, local_branch: String) -> Self {
        Self {
            runner,
            repo_root: repo_root.to_path_buf(),
            local_branch,
            should_return_to_previous_branch: false,
            active: true,
        }
    }

    fn mark_checked_out(&mut self) {
        self.should_return_to_previous_branch = true;
    }

    fn cleanup(&mut self) -> Result<(), String> {
        if !self.active {
            return Ok(());
        }

        let mut errors = Vec::new();
        if self.should_return_to_previous_branch {
            let args = vec!["checkout".to_string(), "-".to_string()];
            match self.runner.git(&self.repo_root, &args) {
                Ok(output) if output.exit_code == Some(0) => {}
                Ok(output) => errors.push(command_failure_message("git checkout -", &output)),
                Err(error) => errors.push(error),
            }
        }

        let delete_command = format!("git branch -D {}", self.local_branch);
        let args = vec![
            "branch".to_string(),
            "-D".to_string(),
            self.local_branch.clone(),
        ];
        match self.runner.git(&self.repo_root, &args) {
            Ok(output) if output.exit_code == Some(0) => {}
            Ok(output) => errors.push(command_failure_message(&delete_command, &output)),
            Err(error) => errors.push(error),
        }

        self.active = false;
        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors.join("; "))
        }
    }
}

impl Drop for LocalBranchGuard<'_> {
    fn drop(&mut self) {
        let _ = self.cleanup();
    }
}

fn main() {
    let cli = Cli::parse();
    let runner = ProcessRunner;
    match run(cli, &runner) {
        Ok(message) => println!("{message}"),
        Err(error) => {
            eprintln!("Error: {error}");
            std::process::exit(1);
        }
    }
}

fn run(cli: Cli, runner: &dyn CommandRunner) -> Result<String, String> {
    let repo_root = std::fs::canonicalize(&cli.repo_root).map_err(|error| {
        format!(
            "failed to resolve repo root {}: {error}",
            cli.repo_root.display()
        )
    })?;
    let pr_head = fetch_pr_head(cli.pr, &repo_root, runner)?;
    ensure_safe_head_ref(&pr_head.head_ref_name)?;

    if cli.dry_run {
        return Ok(render_dry_run(cli.pr, &repo_root, &pr_head));
    }

    let local_branch = format!("pr-{}", cli.pr);
    ensure_success(
        &format!("git fetch origin pull/{}/head:{}", cli.pr, local_branch),
        &runner.git(
            &repo_root,
            &[
                "fetch".to_string(),
                "origin".to_string(),
                format!("pull/{}/head:{}", cli.pr, local_branch),
            ],
        )?,
    )?;
    let mut branch_guard = LocalBranchGuard::new(runner, &repo_root, local_branch.clone());

    ensure_success(
        "git fetch origin master",
        &runner.git(
            &repo_root,
            &[
                "fetch".to_string(),
                "origin".to_string(),
                "master".to_string(),
            ],
        )?,
    )?;

    if is_ancestor(&repo_root, runner, "origin/master", &local_branch)? {
        branch_guard.cleanup()?;
        return Ok("Already up to date with origin/master, no rebase needed".to_string());
    }

    ensure_success(
        &format!("git checkout {}", local_branch),
        &runner.git(&repo_root, &["checkout".to_string(), local_branch.clone()])?,
    )?;
    branch_guard.mark_checked_out();

    let rebase_output = runner.git(
        &repo_root,
        &["rebase".to_string(), "origin/master".to_string()],
    )?;
    if rebase_output.exit_code != Some(0) {
        let conflicting_files = conflicted_files(&repo_root, runner)?;
        let abort_output =
            runner.git(&repo_root, &["rebase".to_string(), "--abort".to_string()])?;
        let cleanup_result = branch_guard.cleanup();

        let mut errors = Vec::new();
        if !conflicting_files.is_empty() {
            errors.push(format!(
                "git rebase origin/master failed due to conflicts in: {}",
                conflicting_files.join(", ")
            ));
        } else {
            errors.push(command_failure_message(
                "git rebase origin/master",
                &rebase_output,
            ));
        }
        if abort_output.exit_code != Some(0) {
            errors.push(command_failure_message("git rebase --abort", &abort_output));
        }
        if let Err(error) = cleanup_result {
            errors.push(format!("cleanup failed: {error}"));
        }
        return Err(errors.join("; "));
    }

    let new_sha = rev_parse_head(&repo_root, runner)?;
    ensure_success(
        &format!(
            "git push --force-with-lease origin {}:{}",
            local_branch, pr_head.head_ref_name
        ),
        &runner.git(
            &repo_root,
            &[
                "push".to_string(),
                "--force-with-lease".to_string(),
                "origin".to_string(),
                format!("{local_branch}:{}", pr_head.head_ref_name),
            ],
        )?,
    )?;

    branch_guard.cleanup()?;
    Ok(format!(
        "Rebased PR #{} onto origin/master at {}",
        cli.pr, new_sha
    ))
}

fn fetch_pr_head(
    pr: u64,
    repo_root: &Path,
    runner: &dyn CommandRunner,
) -> Result<PullRequestHead, String> {
    let args = vec![
        "pr".to_string(),
        "view".to_string(),
        pr.to_string(),
        "--json".to_string(),
        "headRefName,headRefOid".to_string(),
    ];
    let output = runner.gh(repo_root, &args)?;
    ensure_success(
        &format!("gh pr view {pr} --json headRefName,headRefOid"),
        &output,
    )?;
    serde_json::from_str(output.stdout.trim())
        .map_err(|error| format!("failed to parse gh pr view output for PR #{pr}: {error}"))
}

fn ensure_safe_head_ref(head_ref_name: &str) -> Result<(), String> {
    if matches!(head_ref_name, "master" | "main") || head_ref_name.starts_with("release/") {
        return Err(format!(
            "refusing to rebase protected PR head ref {head_ref_name}"
        ));
    }
    Ok(())
}

fn render_dry_run(pr: u64, repo_root: &Path, pr_head: &PullRequestHead) -> String {
    let local_branch = format!("pr-{pr}");
    [
        format!(
            "Would inspect PR #{} head ref {} at {}",
            pr, pr_head.head_ref_name, pr_head.head_ref_oid
        ),
        format!(
            "Would run in {}: git fetch origin pull/{}/head:{}",
            repo_root.display(),
            pr,
            local_branch
        ),
        format!(
            "Would run in {}: git fetch origin master",
            repo_root.display()
        ),
        format!(
            "Would check whether origin/master is already an ancestor of {}",
            local_branch
        ),
        format!(
            "Would run in {}: git checkout {} && git rebase origin/master",
            repo_root.display(),
            local_branch
        ),
        format!(
            "Would run in {}: git push --force-with-lease origin {}:{}",
            repo_root.display(),
            local_branch,
            pr_head.head_ref_name
        ),
    ]
    .join("\n")
}

fn rev_parse_head(repo_root: &Path, runner: &dyn CommandRunner) -> Result<String, String> {
    let output = runner.git(repo_root, &["rev-parse".to_string(), "HEAD".to_string()])?;
    ensure_success("git rev-parse HEAD", &output)?;
    Ok(output.stdout.trim().to_string())
}

fn conflicted_files(repo_root: &Path, runner: &dyn CommandRunner) -> Result<Vec<String>, String> {
    let output = runner.git(
        repo_root,
        &[
            "diff".to_string(),
            "--name-only".to_string(),
            "--diff-filter=U".to_string(),
        ],
    )?;
    ensure_success("git diff --name-only --diff-filter=U", &output)?;
    Ok(output
        .stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned)
        .collect())
}

fn is_ancestor(
    repo_root: &Path,
    runner: &dyn CommandRunner,
    ancestor: &str,
    descendant: &str,
) -> Result<bool, String> {
    let output = runner.git(
        repo_root,
        &[
            "merge-base".to_string(),
            "--is-ancestor".to_string(),
            ancestor.to_string(),
            descendant.to_string(),
        ],
    )?;
    match output.exit_code {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => Err(command_failure_message(
            &format!("git merge-base --is-ancestor {ancestor} {descendant}"),
            &output,
        )),
    }
}

fn run_command(
    program: &str,
    repo_root: &Path,
    args: &[String],
) -> Result<ExecutionResult, String> {
    let output = Command::new(program)
        .current_dir(repo_root)
        .args(args)
        .output()
        .map_err(|error| {
            format!(
                "failed to execute {} {}: {}",
                program,
                args.join(" "),
                error
            )
        })?;

    Ok(ExecutionResult {
        exit_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
    })
}

fn ensure_success(command: &str, result: &ExecutionResult) -> Result<(), String> {
    if result.exit_code == Some(0) {
        Ok(())
    } else {
        Err(command_failure_message(command, result))
    }
}

fn command_failure_message(command: &str, result: &ExecutionResult) -> String {
    let stderr = result.stderr.trim();
    let stdout = result.stdout.trim();
    let detail = if !stderr.is_empty() {
        stderr.to_string()
    } else if !stdout.is_empty() {
        stdout.to_string()
    } else {
        "command produced no output".to_string()
    };
    format!("{command} failed: {detail}")
}
