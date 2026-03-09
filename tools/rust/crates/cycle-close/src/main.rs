use chrono::{DateTime, Utc};
use clap::Parser;
use serde_json::json;
use state_schema::{current_cycle_from_state, read_state_value, StateJson};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const ORCHESTRATOR_SIGNATURE: &str = "[main-orchestrator]";
const RECEIPTS_TOOL_PATH: &str = "tools/cycle-receipts";

#[derive(Debug, Parser)]
#[command(name = "cycle-close")]
struct Cli {
    /// Path to the repository root
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Orchestrator run issue number
    #[arg(long)]
    issue: u64,

    /// Short summary of cycle accomplishments
    #[arg(long)]
    summary: Option<String>,

    /// Priorities for the next cycle
    #[arg(long)]
    priorities: Option<String>,

    /// Show planned actions without executing them
    #[arg(long)]
    dry_run: bool,

    /// Commit but do not push
    #[arg(long)]
    skip_push: bool,

    /// Do not close the issue
    #[arg(long)]
    skip_close: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ExecutionResult {
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
}

trait CommandRunner {
    fn git(&self, _repo_root: &Path, _args: &[String]) -> Result<ExecutionResult, String>;
    fn bash(&self, _repo_root: &Path, _args: &[String]) -> Result<ExecutionResult, String>;
    fn gh(&self, _args: &[String], _input: Option<Vec<u8>>) -> Result<ExecutionResult, String>;
}

struct ProcessRunner;

impl CommandRunner for ProcessRunner {
    fn git(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
        let output = Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .args(args)
            .output()
            .map_err(|error| format!("failed to execute git: {}", error))?;
        Ok(ExecutionResult {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    fn bash(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
        let output = Command::new("bash")
            .current_dir(repo_root)
            .arg(RECEIPTS_TOOL_PATH)
            .args(args)
            .output()
            .map_err(|error| format!("failed to execute bash {}: {}", RECEIPTS_TOOL_PATH, error))?;
        Ok(ExecutionResult {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    fn gh(&self, args: &[String], input: Option<Vec<u8>>) -> Result<ExecutionResult, String> {
        if let Some(input) = input {
            let mut child = Command::new("gh")
                .args(args)
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .map_err(|error| format!("failed to execute gh api: {}", error))?;

            {
                use std::io::Write;

                let stdin = child
                    .stdin
                    .as_mut()
                    .ok_or_else(|| "failed to open stdin for gh api".to_string())?;
                stdin
                    .write_all(&input)
                    .map_err(|error| format!("failed to write gh api payload: {}", error))?;
            }

            let output = child
                .wait_with_output()
                .map_err(|error| format!("failed to wait for gh api: {}", error))?;
            return Ok(ExecutionResult {
                exit_code: output.status.code(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }

        let output = Command::new("gh")
            .args(args)
            .output()
            .map_err(|error| format!("failed to execute gh api: {}", error))?;
        Ok(ExecutionResult {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

fn main() {
    let cli = Cli::parse();
    let runner = ProcessRunner;

    match execute(&cli, &runner) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn execute(cli: &Cli, runner: &dyn CommandRunner) -> Result<String, String> {
    execute_at(cli, runner, Utc::now())
}

fn execute_at(cli: &Cli, runner: &dyn CommandRunner, now: DateTime<Utc>) -> Result<String, String> {
    let cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /last_cycle/number in state.json" {
            "missing numeric /last_cycle/number in docs/state.json".to_string()
        } else {
            error
        }
    })?;
    let state_value = read_state_value(&cli.repo_root)?;
    let state: StateJson = serde_json::from_value(state_value).map_err(|error| {
        format!(
            "failed to parse {}: {}",
            cli.repo_root.join("docs/state.json").display(),
            error
        )
    })?;
    let pipeline_status = state
        .last_cycle
        .summary
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| {
            cli.summary
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
        })
        .unwrap_or("N/A")
        .to_string();
    let worklog_summary = current_cycle_worklog_summary(&cli.repo_root, cycle, now)?;
    let summary = resolve_summary(
        cli.summary.as_deref(),
        worklog_summary.as_deref(),
        state.last_cycle.summary.as_deref(),
    );
    let review_issue = extract_review_issue_number(&state);
    let receipts = load_commit_receipts(&cli.repo_root, cycle, runner)?;
    let comment = format_closing_comment(
        cycle,
        &pipeline_status,
        review_issue,
        &summary,
        cli.priorities.as_deref(),
        &receipts,
    );
    let commit_message = format!(
        "docs(worklog,journal): cycle {} entries [cycle {}]",
        cycle, cycle
    );

    if cli.dry_run {
        let mut lines = vec![format!(
            "Would stage and commit cycle artifacts with message: {}",
            commit_message
        )];
        if cli.skip_push {
            lines.push("Would skip push due to --skip-push".to_string());
        } else {
            lines.push("Would run: git push origin master".to_string());
        }
        lines.push(format!(
            "Would post closing summary comment to issue #{}",
            cli.issue
        ));
        if cli.skip_close {
            lines.push("Would skip closing issue due to --skip-close".to_string());
        } else {
            lines.push(format!("Would close issue #{}", cli.issue));
        }
        lines.push(String::new());
        lines.push(comment);
        return Ok(lines.join("\n"));
    }

    let mut lines = Vec::new();
    let commit_result =
        commit_cycle_artifacts(&cli.repo_root, cycle, now, &commit_message, runner)?;
    match commit_result {
        CommitOutcome::Committed { sha } => {
            lines.push(format!("Committed cycle artifacts: {}", sha));
        }
        CommitOutcome::NothingToCommit => {
            lines.push("No cycle artifact changes to commit".to_string());
        }
    }

    if cli.skip_push {
        lines.push("Skipped push due to --skip-push".to_string());
    } else {
        push_origin_master(&cli.repo_root, runner)?;
        lines.push("Pushed origin master".to_string());
    }

    post_closing_comment(cli.issue, &comment, runner)?;
    lines.push(format!("Posted closing summary comment to #{}", cli.issue));

    if cli.skip_close {
        lines.push("Skipped issue close due to --skip-close".to_string());
    } else {
        close_issue(cli.issue, runner)?;
        lines.push(format!("Closed issue #{}", cli.issue));
    }

    Ok(lines.join("\n"))
}

fn format_closing_comment(
    cycle: u64,
    pipeline_status: &str,
    review_issue: Option<u64>,
    summary: &str,
    priorities: Option<&str>,
    receipts: &[ReceiptEntry],
) -> String {
    let mut lines = vec![
        format!("> **{ORCHESTRATOR_SIGNATURE}** | Cycle {cycle}"),
        String::new(),
        format!("Pipeline status: {}", pipeline_status.trim()),
    ];

    if let Some(issue) = review_issue {
        lines.push(format!("Review agent issue: #{}", issue));
    }

    lines.push(String::new());
    lines.push("## Accomplishments".to_string());
    lines.push(String::new());
    lines.extend(
        summary_items(summary)
            .into_iter()
            .map(|item| format!("- {}", item)),
    );

    if let Some(priorities) = priorities.map(str::trim).filter(|value| !value.is_empty()) {
        lines.push(String::new());
        lines.push("## Next cycle priorities".to_string());
        lines.push(String::new());
        lines.extend(
            summary_items(priorities)
                .into_iter()
                .map(|item| format!("- {}", item)),
        );
    }

    if !receipts.is_empty() {
        lines.push(String::new());
        lines.push("## Commit receipts".to_string());
        lines.push(String::new());
        lines.push("| Tool | Receipt | Link |".to_string());
        lines.push("|------|---------|------|".to_string());
        lines.extend(receipts.iter().map(|entry| {
            format!(
                "| {} | {} | [{}]({}) |",
                escape_markdown_cell(&entry.tool),
                escape_markdown_cell(&entry.receipt),
                entry.receipt,
                entry.url
            )
        }));
    }
    lines.join("\n")
}

#[derive(Debug, PartialEq, Eq)]
enum CommitOutcome {
    Committed { sha: String },
    NothingToCommit,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ReceiptEntry {
    tool: String,
    receipt: String,
    url: String,
}

fn resolve_summary(
    cli_summary: Option<&str>,
    worklog_summary: Option<&str>,
    state_summary: Option<&str>,
) -> String {
    cli_summary
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .or_else(|| {
            worklog_summary
                .map(str::trim)
                .filter(|value| !value.is_empty())
        })
        .or_else(|| {
            state_summary
                .map(str::trim)
                .filter(|value| !value.is_empty())
        })
        .unwrap_or("Cycle close completed.")
        .to_string()
}

fn summary_items(summary: &str) -> Vec<String> {
    let line_items: Vec<String> = summary
        .lines()
        .map(str::trim)
        .map(|line| {
            line.strip_prefix("- ")
                .or_else(|| line.strip_prefix("* "))
                .unwrap_or(line)
                .trim()
                .to_string()
        })
        .filter(|line| !line.is_empty())
        .collect();
    if line_items.len() > 1 {
        return line_items;
    }

    let semicolon_items: Vec<String> = summary
        .split(';')
        .map(str::trim)
        .filter(|item| !item.is_empty())
        .map(ToOwned::to_owned)
        .collect();
    if !semicolon_items.is_empty() {
        return semicolon_items;
    }

    vec![summary.trim().to_string()]
}

fn extract_review_issue_number(state: &StateJson) -> Option<u64> {
    let latest = state.copilot_metrics.dispatch_log_latest.as_deref()?;
    extract_issue_number_from_reference(latest)
}

fn extract_issue_number_from_reference(value: &str) -> Option<u64> {
    let digits: String = value
        .trim()
        .strip_prefix('#')?
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse().ok()
}

fn commit_cycle_artifacts(
    repo_root: &Path,
    cycle: u64,
    now: DateTime<Utc>,
    commit_message: &str,
    runner: &dyn CommandRunner,
) -> Result<CommitOutcome, String> {
    let artifact_paths = cycle_artifact_paths(repo_root, cycle, now, runner)?;
    if artifact_paths.is_empty() {
        return Ok(CommitOutcome::NothingToCommit);
    }
    for artifact_path in &artifact_paths {
        ensure_success(
            "git add cycle artifact",
            runner.git(
                repo_root,
                &["add".to_string(), "--".to_string(), artifact_path.clone()],
            )?,
        )?;
    }

    let mut diff_args = vec![
        "diff".to_string(),
        "--cached".to_string(),
        "--quiet".to_string(),
        "--".to_string(),
    ];
    diff_args.extend(artifact_paths.iter().cloned());
    let diff_output = runner.git(repo_root, &diff_args)?;
    match diff_output.exit_code {
        Some(0) => return Ok(CommitOutcome::NothingToCommit),
        Some(1) => {}
        _ => {
            return Err(command_failure_message(
                "git diff --cached --quiet",
                &diff_output,
            ))
        }
    }

    let mut commit_args = vec![
        "commit".to_string(),
        "-m".to_string(),
        commit_message.to_string(),
        "--".to_string(),
    ];
    commit_args.extend(artifact_paths.iter().cloned());
    ensure_success("git commit", runner.git(repo_root, &commit_args)?)?;

    let rev_parse_output = runner.git(
        repo_root,
        &[
            "rev-parse".to_string(),
            "--short=7".to_string(),
            "HEAD".to_string(),
        ],
    )?;
    ensure_success("git rev-parse --short=7 HEAD", rev_parse_output.clone())?;

    Ok(CommitOutcome::Committed {
        sha: rev_parse_output.stdout.trim().to_string(),
    })
}

fn cycle_artifact_paths(
    repo_root: &Path,
    cycle: u64,
    now: DateTime<Utc>,
    runner: &dyn CommandRunner,
) -> Result<Vec<String>, String> {
    let mut paths = Vec::new();
    if let Some(worklog_path) = find_current_cycle_worklog_relative_path(repo_root, cycle, now)? {
        paths.push(worklog_path);
    }
    if let Some(journal_path) = existing_file_relative_path(
        repo_root,
        &format!("docs/journal/{}.md", now.format("%Y-%m-%d")),
    )? {
        paths.push(journal_path);
    }
    paths.push(required_file_relative_path(repo_root, "docs/state.json")?);
    if journal_index_is_modified(repo_root, runner)? {
        paths.push("JOURNAL.md".to_string());
    }
    if let Some(review_path) =
        existing_file_relative_path(repo_root, &format!("docs/reviews/cycle-{}.md", cycle))?
    {
        paths.push(review_path);
    }
    Ok(paths)
}

fn current_cycle_worklog_summary(
    repo_root: &Path,
    cycle: u64,
    now: DateTime<Utc>,
) -> Result<Option<String>, String> {
    let Some(relative_path) = find_current_cycle_worklog_relative_path(repo_root, cycle, now)?
    else {
        return Ok(None);
    };
    let path = repo_root.join(&relative_path);
    let content = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    Ok(extract_markdown_section(&content, "What was done"))
}

fn find_current_cycle_worklog_relative_path(
    repo_root: &Path,
    cycle: u64,
    now: DateTime<Utc>,
) -> Result<Option<String>, String> {
    let worklog_root = repo_root
        .join("docs")
        .join("worklog")
        .join(now.format("%Y-%m-%d").to_string());
    if !worklog_root.exists() {
        return Ok(None);
    }
    let metadata = fs::metadata(&worklog_root)
        .map_err(|error| format!("failed to read {}: {}", worklog_root.display(), error))?;
    if !metadata.is_dir() {
        return Err(format!(
            "expected {} to be a directory",
            worklog_root.display()
        ));
    }

    let mut candidates = Vec::new();
    for entry in fs::read_dir(&worklog_root)
        .map_err(|error| format!("failed to read {}: {}", worklog_root.display(), error))?
    {
        let entry = entry.map_err(|error| {
            format!(
                "failed to read entry in {}: {}",
                worklog_root.display(),
                error
            )
        })?;
        let path = entry.path();
        if path.extension() != Some(OsStr::new("md")) {
            continue;
        }
        let file_metadata = fs::metadata(&path)
            .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
        if !file_metadata.is_file() {
            return Err(format!("expected {} to be a file", path.display()));
        }
        let content = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
        if content
            .lines()
            .next()
            .is_some_and(|line| matches_cycle_heading(line, cycle))
        {
            candidates.push(path);
        }
    }

    candidates.sort();
    candidates
        .into_iter()
        .last()
        .map(|path| {
            path.strip_prefix(repo_root)
                .map_err(|error| {
                    format!(
                        "failed to compute relative path for {}: {}",
                        path.display(),
                        error
                    )
                })
                .map(|relative| relative.to_string_lossy().replace('\\', "/"))
        })
        .transpose()
}

fn matches_cycle_heading(line: &str, cycle: u64) -> bool {
    let prefix = format!("# Cycle {}", cycle);
    let Some(remainder) = line.trim_start().strip_prefix(&prefix) else {
        return false;
    };
    remainder.is_empty()
        || remainder
            .chars()
            .next()
            .is_some_and(|character| character.is_whitespace() || matches!(character, '—' | '-'))
}

fn extract_markdown_section(content: &str, heading: &str) -> Option<String> {
    let target_heading = format!("## {heading}");
    let mut in_section = false;
    let mut lines = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim_end();
        if in_section {
            if trimmed.starts_with("## ") {
                break;
            }
            lines.push(trimmed);
        } else if trimmed == target_heading {
            in_section = true;
        }
    }

    let section = lines.join("\n").trim().to_string();
    if section.is_empty() {
        None
    } else {
        Some(section)
    }
}

fn required_file_relative_path(repo_root: &Path, relative_path: &str) -> Result<String, String> {
    let path = repo_root.join(relative_path);
    let metadata = fs::metadata(&path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    if !metadata.is_file() {
        return Err(format!("expected {} to be a file", path.display()));
    }
    Ok(relative_path.to_string())
}

fn existing_file_relative_path(
    repo_root: &Path,
    relative_path: &str,
) -> Result<Option<String>, String> {
    let path = repo_root.join(relative_path);
    let Ok(metadata) = fs::metadata(&path) else {
        return Ok(None);
    };
    if !metadata.is_file() {
        return Err(format!("expected {} to be a file", path.display()));
    }
    Ok(Some(relative_path.to_string()))
}

fn journal_index_is_modified(repo_root: &Path, runner: &dyn CommandRunner) -> Result<bool, String> {
    let output = runner.git(
        repo_root,
        &[
            "status".to_string(),
            "--short".to_string(),
            "--".to_string(),
            "JOURNAL.md".to_string(),
        ],
    )?;
    ensure_success("git status --short -- JOURNAL.md", output.clone())?;
    Ok(!output.stdout.trim().is_empty())
}

fn load_commit_receipts(
    repo_root: &Path,
    cycle: u64,
    runner: &dyn CommandRunner,
) -> Result<Vec<ReceiptEntry>, String> {
    let output = runner.bash(
        repo_root,
        &[
            "--cycle".to_string(),
            cycle.to_string(),
            "--json".to_string(),
        ],
    )?;
    ensure_success("bash tools/cycle-receipts", output.clone())?;
    let value: serde_json::Value = serde_json::from_str(&output.stdout).map_err(|error| {
        format!(
            "failed to parse cycle-receipts JSON output for cycle {}: {}",
            cycle, error
        )
    })?;
    let array = value.as_array().ok_or_else(|| {
        format!(
            "cycle-receipts output for cycle {} was not a JSON array",
            cycle
        )
    })?;
    array
        .iter()
        .map(|entry| {
            Ok(ReceiptEntry {
                tool: json_string_field(entry, "step")?,
                receipt: json_string_field(entry, "receipt")?,
                url: json_string_field(entry, "url")?,
            })
        })
        .collect()
}

fn json_string_field(value: &serde_json::Value, field: &str) -> Result<String, String> {
    value
        .get(field)
        .and_then(|inner| inner.as_str())
        .map(ToOwned::to_owned)
        .ok_or_else(|| {
            format!(
                "missing string field `{}` in cycle-receipts output: {}",
                field, value
            )
        })
}

fn escape_markdown_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

fn push_origin_master(repo_root: &Path, runner: &dyn CommandRunner) -> Result<(), String> {
    let push_args = [
        "push".to_string(),
        "origin".to_string(),
        "master".to_string(),
    ];
    let push_output = runner.git(repo_root, &push_args)?;
    if matches!(push_output.exit_code, Some(0)) {
        return Ok(());
    }

    if !contains_fetch_first(&push_output) {
        return Err(command_failure_message(
            "git push origin master",
            &push_output,
        ));
    }

    ensure_success(
        "git pull --rebase origin master",
        runner.git(
            repo_root,
            &[
                "pull".to_string(),
                "--rebase".to_string(),
                "origin".to_string(),
                "master".to_string(),
            ],
        )?,
    )?;

    let retry_output = runner.git(repo_root, &push_args)?;
    ensure_success("git push origin master", retry_output)
}

fn contains_fetch_first(output: &ExecutionResult) -> bool {
    let combined = format!("{}\n{}", output.stdout, output.stderr).to_ascii_lowercase();
    combined.contains("fetch first")
}

fn post_closing_comment(
    issue: u64,
    comment: &str,
    runner: &dyn CommandRunner,
) -> Result<(), String> {
    let payload = serde_json::to_vec(&json!({ "body": comment }))
        .map_err(|error| format!("failed to serialize comment payload: {}", error))?;
    ensure_success(
        "gh api",
        runner.gh(
            &[
                "api".to_string(),
                format!("repos/{MAIN_REPO}/issues/{issue}/comments"),
                "--method".to_string(),
                "POST".to_string(),
                "--input".to_string(),
                "-".to_string(),
            ],
            Some(payload),
        )?,
    )
}

fn close_issue(issue: u64, runner: &dyn CommandRunner) -> Result<(), String> {
    ensure_success(
        "gh api",
        runner.gh(
            &[
                "api".to_string(),
                format!("repos/{MAIN_REPO}/issues/{issue}"),
                "-X".to_string(),
                "PATCH".to_string(),
                "-f".to_string(),
                "state=closed".to_string(),
            ],
            None,
        )?,
    )
}

fn ensure_success(command: &str, output: ExecutionResult) -> Result<(), String> {
    if matches!(output.exit_code, Some(0)) {
        Ok(())
    } else {
        Err(command_failure_message(command, &output))
    }
}

fn command_failure_message(command: &str, output: &ExecutionResult) -> String {
    let code = output.exit_code.map_or_else(
        || "terminated by signal".to_string(),
        |value| value.to_string(),
    );
    let stderr = output.stderr.trim();

    if stderr.is_empty() {
        format!("{command} failed with status {code}")
    } else {
        format!("{command} failed with status {code}: {stderr}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use serde_json::json;
    use state_schema::write_state_value;
    use std::collections::VecDeque;
    use std::fs;
    use std::path::Path;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Mutex;

    #[derive(Default)]
    struct MockRunner {
        git_results: Mutex<VecDeque<Result<ExecutionResult, String>>>,
        bash_results: Mutex<VecDeque<Result<ExecutionResult, String>>>,
        gh_results: Mutex<VecDeque<Result<ExecutionResult, String>>>,
        git_calls: Mutex<Vec<Vec<String>>>,
        bash_calls: Mutex<Vec<Vec<String>>>,
        gh_calls: Mutex<Vec<(Vec<String>, Option<Vec<u8>>)>>,
    }

    impl MockRunner {
        fn with_results(
            git_results: Vec<Result<ExecutionResult, String>>,
            bash_results: Vec<Result<ExecutionResult, String>>,
            gh_results: Vec<Result<ExecutionResult, String>>,
        ) -> Self {
            Self {
                git_results: Mutex::new(VecDeque::from(git_results)),
                bash_results: Mutex::new(VecDeque::from(bash_results)),
                gh_results: Mutex::new(VecDeque::from(gh_results)),
                ..Self::default()
            }
        }

        fn git_calls(&self) -> Vec<Vec<String>> {
            self.git_calls.lock().unwrap().clone()
        }

        fn gh_calls(&self) -> Vec<(Vec<String>, Option<Vec<u8>>)> {
            self.gh_calls.lock().unwrap().clone()
        }

        fn bash_calls(&self) -> Vec<Vec<String>> {
            self.bash_calls.lock().unwrap().clone()
        }
    }

    impl CommandRunner for MockRunner {
        fn git(&self, _repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
            self.git_calls.lock().unwrap().push(args.to_vec());
            self.git_results
                .lock()
                .unwrap()
                .pop_front()
                .unwrap_or_else(|| panic!("unexpected git call: {:?}", args))
        }

        fn bash(&self, _repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
            self.bash_calls.lock().unwrap().push(args.to_vec());
            self.bash_results
                .lock()
                .unwrap()
                .pop_front()
                .unwrap_or_else(|| panic!("unexpected bash call: {:?}", args))
        }

        fn gh(&self, args: &[String], input: Option<Vec<u8>>) -> Result<ExecutionResult, String> {
            self.gh_calls.lock().unwrap().push((args.to_vec(), input));
            self.gh_results
                .lock()
                .unwrap()
                .pop_front()
                .unwrap_or_else(|| panic!("unexpected gh call: {:?}", args))
        }
    }

    struct TempRepo {
        path: PathBuf,
        remote_path: PathBuf,
    }

    impl TempRepo {
        fn new() -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
            let root = std::env::temp_dir().join(format!("cycle-close-test-{}", run_id));
            let remote = std::env::temp_dir().join(format!("cycle-close-remote-{}", run_id));
            fs::create_dir_all(root.join("docs/journal")).unwrap();
            fs::create_dir_all(root.join("docs/reviews")).unwrap();
            fs::create_dir_all(&remote).unwrap();

            Self {
                path: root,
                remote_path: remote,
            }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn init(&self, state: serde_json::Value) {
            write_state_value(&self.path, &state).unwrap();
            fs::write(self.path.join("JOURNAL.md"), "# Journal\n").unwrap();
            git_ok(self.path(), ["init", "--initial-branch=master"]);
            git_ok(self.path(), ["config", "user.name", "Cycle Close Tests"]);
            git_ok(
                self.path(),
                ["config", "user.email", "cycle-close-tests@example.com"],
            );
            git_ok(self.path(), ["add", "."]);
            git_ok(self.path(), ["commit", "-m", "Initial state"]);
            git_ok(
                self.path(),
                ["init", "--bare", self.remote_path.to_str().unwrap()],
            );
            git_ok(
                self.path(),
                [
                    "remote",
                    "add",
                    "origin",
                    self.remote_path.to_str().unwrap(),
                ],
            );
            git_ok(self.path(), ["push", "-u", "origin", "master"]);
        }

        fn write_cycle_artifacts(&self, cycle: u64, now: DateTime<Utc>) {
            let worklog_dir = self
                .path
                .join("docs/worklog")
                .join(now.format("%Y-%m-%d").to_string());
            fs::create_dir_all(&worklog_dir).unwrap();
            fs::write(
                worklog_dir.join(format!("{}-cycle-{}-summary.md", now.format("%H%M%S"), cycle)),
                format!(
                    "# Cycle {} — {} UTC\n\n## What was done\n\n- Captured worklog accomplishments\n- Updated cycle-close contract\n\n## Next steps\n\n1. Review the next dispatched PR.\n",
                    cycle,
                    now.format("%Y-%m-%d %H:%M")
                ),
            )
            .unwrap();
            fs::write(
                self.path
                    .join(format!("docs/journal/{}.md", now.format("%Y-%m-%d"))),
                "Journal entry\n",
            )
            .unwrap();
            fs::write(
                self.path.join(format!("docs/reviews/cycle-{}.md", cycle)),
                "Review entry\n",
            )
            .unwrap();
            fs::write(
                self.path.join("JOURNAL.md"),
                "# Journal\n\nAppended entry\n",
            )
            .unwrap();
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
            let _ = fs::remove_dir_all(&self.remote_path);
        }
    }

    struct HybridRunner {
        bash_calls: Mutex<Vec<Vec<String>>>,
        gh_calls: Mutex<Vec<(Vec<String>, Option<Vec<u8>>)>>,
    }

    impl HybridRunner {
        fn new() -> Self {
            Self {
                bash_calls: Mutex::new(Vec::new()),
                gh_calls: Mutex::new(Vec::new()),
            }
        }

        fn gh_calls(&self) -> Vec<(Vec<String>, Option<Vec<u8>>)> {
            self.gh_calls.lock().unwrap().clone()
        }

        fn bash_calls(&self) -> Vec<Vec<String>> {
            self.bash_calls.lock().unwrap().clone()
        }
    }

    impl CommandRunner for HybridRunner {
        fn git(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
            let output = std::process::Command::new("git")
                .arg("-C")
                .arg(repo_root)
                .args(args)
                .output()
                .map_err(|error| format!("failed to execute git: {}", error))?;
            Ok(ExecutionResult {
                exit_code: output.status.code(),
                stdout: String::from_utf8_lossy(&output.stdout).to_string(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            })
        }

        fn bash(&self, _repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
            self.bash_calls.lock().unwrap().push(args.to_vec());
            Ok(success_output(&sample_receipts_json()))
        }

        fn gh(&self, args: &[String], input: Option<Vec<u8>>) -> Result<ExecutionResult, String> {
            self.gh_calls.lock().unwrap().push((args.to_vec(), input));
            Ok(ExecutionResult {
                exit_code: Some(0),
                stdout: "{}".to_string(),
                stderr: String::new(),
            })
        }
    }

    fn git_ok<I, S>(repo_root: &Path, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let rendered: Vec<String> = args
            .into_iter()
            .map(|value| value.as_ref().to_string_lossy().into_owned())
            .collect();
        let output = std::process::Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .args(&rendered)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git command failed: git -C {} {}: {}",
            repo_root.display(),
            rendered.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn sample_state() -> serde_json::Value {
        json!({
            "last_cycle": {
                "number": 202,
                "summary": "Pipeline check: PASS; review findings recorded"
            },
            "copilot_metrics": {
                "dispatch_log_latest": "#873 Review findings follow-up (cycle 202)"
            }
        })
    }

    fn fixed_now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-03-09T08:06:02Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    fn sample_receipts_json() -> String {
        serde_json::to_string(&json!([
            {
                "step": "cycle-start",
                "receipt": "abc1234",
                "commit": "state(cycle-start): begin cycle 202 [cycle 202]",
                "url": "https://github.com/EvaLok/schema-org-json-ld/commit/abc1234"
            }
        ]))
        .unwrap()
    }

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--issue"));
        assert!(help.contains("--summary"));
        assert!(help.contains("--priorities"));
        assert!(help.contains("--dry-run"));
        assert!(help.contains("--skip-push"));
        assert!(help.contains("--skip-close"));
        assert!(help.contains("--repo-root"));
    }

    #[test]
    fn dry_run_reports_planned_actions_without_running_commands() {
        let repo = TempRepo::new();
        repo.init(sample_state());
        repo.write_cycle_artifacts(202, fixed_now());
        let cli = Cli {
            repo_root: repo.path().to_path_buf(),
            issue: 871,
            summary: Some("Validated pipeline and captured review follow-up".to_string()),
            priorities: None,
            dry_run: true,
            skip_push: false,
            skip_close: false,
        };
        let runner = MockRunner::with_results(
            vec![],
            vec![Ok(success_output(&sample_receipts_json()))],
            vec![],
        );

        let output = execute_at(&cli, &runner, fixed_now()).expect("dry-run should succeed");

        assert!(output.contains("Would stage and commit cycle artifacts"));
        assert!(output.contains("docs(worklog,journal): cycle 202 entries [cycle 202]"));
        assert!(output.contains("> **[main-orchestrator]** | Cycle 202"));
        assert!(runner.git_calls().is_empty());
        assert!(runner.gh_calls().is_empty());
        assert_eq!(
            runner.bash_calls(),
            vec![vec![
                "--cycle".to_string(),
                "202".to_string(),
                "--json".to_string()
            ]]
        );
    }

    #[test]
    fn worklog_summary_is_used_when_cli_summary_is_omitted() {
        let repo = TempRepo::new();
        repo.init(sample_state());
        repo.write_cycle_artifacts(202, fixed_now());
        let cli = Cli {
            repo_root: repo.path().to_path_buf(),
            issue: 871,
            summary: None,
            priorities: None,
            dry_run: true,
            skip_push: false,
            skip_close: false,
        };
        let runner = MockRunner::with_results(vec![], vec![Ok(success_output("[]"))], vec![]);

        let output = execute_at(&cli, &runner, fixed_now()).expect("dry-run should succeed");

        assert!(output.contains("- Captured worklog accomplishments"));
        assert!(output.contains("- Updated cycle-close contract"));
        assert!(!output.contains("- Pipeline check: PASS"));
    }

    #[test]
    fn commit_stages_only_current_cycle_artifacts() {
        let repo = TempRepo::new();
        repo.init(sample_state());
        repo.write_cycle_artifacts(202, fixed_now());
        let older_dir = repo.path().join("docs/worklog/2026-03-08");
        fs::create_dir_all(&older_dir).unwrap();
        fs::write(
            older_dir.join("225043-hundred-ninety-seventh-orchestrator-cycle.md"),
            "# Cycle 197 — 2026-03-08 22:50 UTC\n\n## What was done\n\n- Older entry.\n",
        )
        .unwrap();

        let cli = Cli {
            repo_root: repo.path().to_path_buf(),
            issue: 871,
            summary: None,
            priorities: None,
            dry_run: false,
            skip_push: true,
            skip_close: true,
        };
        let runner = MockRunner::with_results(
            vec![
                Ok(success_output(" M JOURNAL.md\n")),
                Ok(success_output("")),
                Ok(success_output("")),
                Ok(success_output("")),
                Ok(success_output("")),
                Ok(success_output("")),
                Ok(exit_output(1, "", "")),
                Ok(success_output("")),
                Ok(success_output("abc1234\n")),
            ],
            vec![Ok(success_output("[]"))],
            vec![Ok(success_output("{}"))],
        );

        execute_at(&cli, &runner, fixed_now()).expect("execution should succeed");

        let expected_worklog = format!(
            "docs/worklog/{}/{}-cycle-202-summary.md",
            fixed_now().format("%Y-%m-%d"),
            fixed_now().format("%H%M%S")
        );
        let expected_journal = format!("docs/journal/{}.md", fixed_now().format("%Y-%m-%d"));
        let git_calls = runner.git_calls();
        assert_eq!(
            git_calls[1].as_slice(),
            ["add", "--", expected_worklog.as_str()]
        );
        assert_eq!(
            git_calls[2].as_slice(),
            ["add", "--", expected_journal.as_str()]
        );
        assert_eq!(git_calls[3].as_slice(), ["add", "--", "docs/state.json"]);
        assert_eq!(git_calls[4].as_slice(), ["add", "--", "JOURNAL.md"]);
        assert_eq!(
            git_calls[5].as_slice(),
            ["add", "--", "docs/reviews/cycle-202.md"]
        );
        assert!(!git_calls.iter().flatten().any(|arg| arg == "docs/worklog"));
        assert!(!git_calls.iter().flatten().any(|arg| arg == "docs/journal"));
        assert!(!git_calls.iter().flatten().any(|arg| arg == "docs/reviews"));
    }

    #[test]
    fn nothing_to_commit_is_not_an_error() {
        let repo = TempRepo::new();
        repo.init(sample_state());
        let cli = Cli {
            repo_root: repo.path().to_path_buf(),
            issue: 871,
            summary: None,
            priorities: None,
            dry_run: false,
            skip_push: false,
            skip_close: false,
        };
        let runner = MockRunner::with_results(
            vec![
                Ok(success_output("")),
                Ok(exit_output(0, "", "")),
                Ok(success_output("")),
                Ok(success_output("")),
            ],
            vec![Ok(success_output("[]"))],
            vec![Ok(success_output("{}")), Ok(success_output("{}"))],
        );

        let output = execute_at(&cli, &runner, fixed_now()).expect("execution should succeed");

        assert!(output.contains("No cycle artifact changes to commit"));
    }

    #[test]
    fn push_retries_once_after_fetch_first_failure() {
        let repo = TempRepo::new();
        repo.init(sample_state());
        repo.write_cycle_artifacts(202, fixed_now());
        let cli = Cli {
            repo_root: repo.path().to_path_buf(),
            issue: 871,
            summary: None,
            priorities: None,
            dry_run: false,
            skip_push: false,
            skip_close: false,
        };
        let runner = MockRunner::with_results(
            vec![
                Ok(success_output(" M JOURNAL.md\n")),
                Ok(success_output("")),
                Ok(success_output("")),
                Ok(success_output("")),
                Ok(success_output("")),
                Ok(success_output("")),
                Ok(exit_output(1, "", "")),
                Ok(success_output("")),
                Ok(success_output("abc1234\n")),
                Ok(exit_output(1, "", "error: failed to push some refs\nhint: Updates were rejected because the remote contains work that you do not have locally. fetch first\n")),
                Ok(success_output("")),
                Ok(success_output("")),
            ],
            vec![Ok(success_output("[]"))],
            vec![Ok(success_output("{}")), Ok(success_output("{}"))],
        );

        execute_at(&cli, &runner, fixed_now()).expect("execution should succeed");

        let git_calls = runner.git_calls();
        assert!(git_calls
            .iter()
            .any(|call| call.as_slice() == ["push", "origin", "master"]));
        assert!(git_calls
            .iter()
            .any(|call| call.as_slice() == ["pull", "--rebase", "origin", "master"]));
    }

    #[test]
    fn end_to_end_commits_pushes_comments_and_is_idempotent() {
        let repo = TempRepo::new();
        repo.init(sample_state());
        repo.write_cycle_artifacts(202, fixed_now());
        let cli = Cli {
            repo_root: repo.path().to_path_buf(),
            issue: 871,
            summary: Some("Validated pipeline and captured review follow-up".to_string()),
            priorities: Some("Review PR #878; Fix write-entry journal bug".to_string()),
            dry_run: false,
            skip_push: false,
            skip_close: false,
        };
        let runner = HybridRunner::new();

        let first_output =
            execute_at(&cli, &runner, fixed_now()).expect("first run should succeed");
        assert!(first_output.contains("Committed cycle artifacts"));

        let log_output = std::process::Command::new("git")
            .arg("-C")
            .arg(repo.path())
            .args(["log", "-1", "--pretty=%B"])
            .output()
            .unwrap();
        assert!(log_output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&log_output.stdout).trim(),
            "docs(worklog,journal): cycle 202 entries [cycle 202]"
        );

        let local_head = std::process::Command::new("git")
            .arg("-C")
            .arg(repo.path())
            .args(["rev-parse", "HEAD"])
            .output()
            .unwrap();
        let remote_head = std::process::Command::new("git")
            .arg("-C")
            .arg(repo.path())
            .args(["ls-remote", "origin", "refs/heads/master"])
            .output()
            .unwrap();
        let local_sha = String::from_utf8_lossy(&local_head.stdout)
            .trim()
            .to_string();
        let remote_sha = String::from_utf8_lossy(&remote_head.stdout)
            .split_whitespace()
            .next()
            .unwrap()
            .to_string();
        assert_eq!(local_sha, remote_sha);

        let gh_calls = runner.gh_calls();
        assert_eq!(gh_calls.len(), 2);
        let comment_body = gh_calls[0]
            .1
            .as_ref()
            .and_then(|payload| serde_json::from_slice::<serde_json::Value>(payload).ok())
            .and_then(|json| {
                json.get("body")
                    .and_then(|value| value.as_str())
                    .map(ToOwned::to_owned)
            })
            .unwrap();
        assert!(comment_body.starts_with("> **[main-orchestrator]** | Cycle 202"));
        assert!(comment_body.contains("Review agent issue: #873"));
        assert!(comment_body.contains("## Next cycle priorities"));
        assert!(comment_body.contains("## Commit receipts"));
        assert!(comment_body.contains("| cycle-start | abc1234 | [abc1234]("));
        assert_eq!(runner.bash_calls().len(), 1);

        let second_output =
            execute_at(&cli, &runner, fixed_now()).expect("second run should also succeed");
        assert!(second_output.contains("No cycle artifact changes to commit"));
        assert_eq!(runner.bash_calls().len(), 2);
    }

    #[test]
    fn format_closing_comment_includes_required_sections() {
        let comment = format_closing_comment(
            202,
            "Pipeline check: PASS; review findings recorded",
            Some(873),
            "Validated pipeline and captured review follow-up",
            Some("Review PR #878; Fix write-entry journal bug"),
            &[ReceiptEntry {
                tool: "cycle-start".to_string(),
                receipt: "abc1234".to_string(),
                url: "https://github.com/EvaLok/schema-org-json-ld/commit/abc1234".to_string(),
            }],
        );

        assert!(comment.starts_with("> **[main-orchestrator]** | Cycle 202"));
        assert!(comment.contains("Pipeline status: Pipeline check: PASS; review findings recorded"));
        assert!(comment.contains("Review agent issue: #873"));
        assert!(comment.contains("- Validated pipeline and captured review follow-up"));
        assert!(comment.contains("## Next cycle priorities"));
        assert!(comment.contains("## Commit receipts"));
        assert!(comment.contains("| cycle-start | abc1234 | [abc1234]("));
    }

    fn success_output(stdout: &str) -> ExecutionResult {
        ExecutionResult {
            exit_code: Some(0),
            stdout: stdout.to_string(),
            stderr: String::new(),
        }
    }

    fn exit_output(code: i32, stdout: &str, stderr: &str) -> ExecutionResult {
        ExecutionResult {
            exit_code: Some(code),
            stdout: stdout.to_string(),
            stderr: stderr.to_string(),
        }
    }
}
