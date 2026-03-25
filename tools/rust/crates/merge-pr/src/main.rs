use clap::Parser;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const PROCESS_MERGE_BINARY: &str = "tools/rust/target/release/process-merge";

#[derive(Debug, Parser)]
#[command(name = "merge-pr")]
struct Cli {
    /// Pull request number to merge
    #[arg(long)]
    pr: u64,

    /// Originating issue number to pass to process-merge
    #[arg(long)]
    issue: u64,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Show planned actions without executing them
    #[arg(long)]
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
    fn process_merge(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String>;
}

struct ProcessRunner;

impl CommandRunner for ProcessRunner {
    fn gh(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
        run_command("gh", repo_root, args)
    }

    fn git(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
        run_command("git", repo_root, args)
    }

    fn process_merge(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
        let binary = repo_root.join(PROCESS_MERGE_BINARY);
        let output = Command::new(&binary)
            .current_dir(repo_root)
            .args(args)
            .output()
            .map_err(|error| {
                format!(
                    "failed to execute {}: {}",
                    binary.display(),
                    error
                )
            })?;
        Ok(ExecutionResult {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum PullRequestState {
    Open,
    Closed,
    Merged,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum MergeableState {
    Mergeable,
    Conflicting,
    Unknown,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct PullRequestView {
    state: PullRequestState,
    is_draft: bool,
    mergeable: Option<MergeableState>,
    head_ref_name: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let runner = ProcessRunner;

    match run(cli, &runner) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn run(cli: Cli, runner: &dyn CommandRunner) -> Result<String, String> {
    if cli.dry_run {
        return execute(&cli, runner);
    }

    let repo_root = canonicalize_repo_root(&cli.repo_root)?;
    execute(
        &Cli {
            repo_root,
            ..cli
        },
        runner,
    )
}

fn execute(cli: &Cli, runner: &dyn CommandRunner) -> Result<String, String> {
    if cli.dry_run {
        return Ok(render_dry_run(cli));
    }

    let pr = fetch_pr_view(cli.pr, &cli.repo_root, runner)?;
    let branch_name = extract_branch_name(&pr)?;

    let mut lines = Vec::new();

    if pr.state == PullRequestState::Merged {
        lines.push(format!(
            "PR #{} is already merged; skipping readiness and merge steps",
            cli.pr
        ));
    } else {
        ensure_mergeable(cli.pr, &pr)?;
        if pr.is_draft {
            ensure_success(
                &format!("gh pr ready {} --repo {}", cli.pr, MAIN_REPO),
                &runner.gh(
                    &cli.repo_root,
                    &[
                        "pr".to_string(),
                        "ready".to_string(),
                        cli.pr.to_string(),
                        "--repo".to_string(),
                        MAIN_REPO.to_string(),
                    ],
                )?,
            )?;
            lines.push(format!("Marked PR #{} as ready for review", cli.pr));
        }

        ensure_success(
            &format!("gh pr merge {} --squash --repo {}", cli.pr, MAIN_REPO),
            &runner.gh(
                &cli.repo_root,
                &[
                    "pr".to_string(),
                    "merge".to_string(),
                    cli.pr.to_string(),
                    "--squash".to_string(),
                    "--repo".to_string(),
                    MAIN_REPO.to_string(),
                ],
            )?,
        )?;
        lines.push(format!("Merged PR #{} with squash merge", cli.pr));

        let merged_state = fetch_pr_state(cli.pr, &cli.repo_root, runner)?;
        if merged_state != PullRequestState::Merged {
            return Err(format!(
                "PR #{} state after merge is {:?}, expected MERGED",
                cli.pr, merged_state
            ));
        }
        lines.push(format!("Verified PR #{} is now merged", cli.pr));
    }

    ensure_success(
        "git pull --rebase origin master",
        &runner.git(
            &cli.repo_root,
            &[
                "pull".to_string(),
                "--rebase".to_string(),
                "origin".to_string(),
                "master".to_string(),
            ],
        )?,
    )?;
    lines.push("Pulled latest origin/master with rebase".to_string());

    let process_merge_output = runner.process_merge(
        &cli.repo_root,
        &[
            "--prs".to_string(),
            cli.pr.to_string(),
            "--issues".to_string(),
            cli.issue.to_string(),
            "--repo-root".to_string(),
            cli.repo_root.display().to_string(),
        ],
    )?;
    ensure_success("process-merge", &process_merge_output)?;
    let receipt = extract_receipt_hash(&process_merge_output.stdout)?;
    lines.push(format!(
        "Processed merge state for PR #{} and issue #{} (receipt: {})",
        cli.pr, cli.issue, receipt
    ));

    let push_output = runner.git(
        &cli.repo_root,
        &[
            "push".to_string(),
            "origin".to_string(),
            "master".to_string(),
        ],
    )?;
    ensure_success("git push origin master", &push_output)?;
    lines.push("Pushed origin/master state updates".to_string());

    let delete_branch_command = format!("git push origin --delete {}", branch_name);
    let deleted_branch_message = format!("Deleted remote branch {}", branch_name);
    let skipped_branch_message = format!(
        "Remote branch {} was already deleted; skipping branch deletion",
        branch_name
    );
    let delete_branch_output = runner.git(
        &cli.repo_root,
        &[
            "push".to_string(),
            "origin".to_string(),
            "--delete".to_string(),
            branch_name,
        ],
    )?;
    if matches!(delete_branch_output.exit_code, Some(0)) {
        lines.push(deleted_branch_message);
    } else if is_missing_remote_branch(&delete_branch_output) {
        lines.push(skipped_branch_message);
    } else {
        return Err(command_failure_message(&delete_branch_command, &delete_branch_output));
    }

    lines.push(format!("Merge workflow complete for PR #{} (receipt: {})", cli.pr, receipt));
    Ok(lines.join("\n"))
}

fn run_command(program: &str, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
    let output = Command::new(program)
        .current_dir(repo_root)
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute {} {}: {}", program, args.join(" "), error))?;
    Ok(ExecutionResult {
        exit_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
    })
}

fn canonicalize_repo_root(repo_root: &Path) -> Result<PathBuf, String> {
    fs::canonicalize(repo_root)
        .map_err(|error| format!("failed to resolve repo root {}: {}", repo_root.display(), error))
}

fn fetch_pr_view(
    pr_number: u64,
    repo_root: &Path,
    runner: &dyn CommandRunner,
) -> Result<PullRequestView, String> {
    let output = runner.gh(
        repo_root,
        &[
            "pr".to_string(),
            "view".to_string(),
            pr_number.to_string(),
            "--repo".to_string(),
            MAIN_REPO.to_string(),
            "--json".to_string(),
            "state,isDraft,mergeable,headRefName".to_string(),
        ],
    )?;
    ensure_success(&format!("gh pr view {} --repo {}", pr_number, MAIN_REPO), &output)?;
    parse_pr_view(&output.stdout)
}

fn fetch_pr_state(
    pr_number: u64,
    repo_root: &Path,
    runner: &dyn CommandRunner,
) -> Result<PullRequestState, String> {
    Ok(fetch_pr_view(pr_number, repo_root, runner)?.state)
}

fn parse_pr_view(raw: &str) -> Result<PullRequestView, String> {
    serde_json::from_str(raw).map_err(|error| format!("failed to parse gh pr view output: {}", error))
}

fn ensure_mergeable(pr_number: u64, pr: &PullRequestView) -> Result<(), String> {
    if pr.state != PullRequestState::Open {
        return Err(format!(
            "PR #{} is {:?}, expected OPEN or MERGED",
            pr_number, pr.state
        ));
    }

    if pr.mergeable != Some(MergeableState::Mergeable) {
        return Err(format!(
            "PR #{} is not mergeable (mergeable={:?})",
            pr_number, pr.mergeable
        ));
    }

    Ok(())
}

fn extract_branch_name(pr: &PullRequestView) -> Result<String, String> {
    let branch_name = pr
        .head_ref_name
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "missing non-empty headRefName in gh pr view output".to_string())?;
    Ok(branch_name.to_string())
}

fn render_dry_run(cli: &Cli) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "Would run: gh pr view {} --repo {} --json state,isDraft,mergeable,headRefName",
        cli.pr, MAIN_REPO
    ));
    lines.push(format!(
        "Would mark PR #{} as ready if gh pr view reports it is a draft",
        cli.pr
    ));
    lines.push(format!(
        "Would run: gh pr merge {} --squash --repo {}",
        cli.pr, MAIN_REPO
    ));
    lines.push(format!(
        "Would verify PR #{} is merged, or skip merge if it is already merged",
        cli.pr
    ));
    lines.push("Would run: git pull --rebase origin master".to_string());
    lines.push(format!(
        "Would run: {} --prs {} --issues {} --repo-root {}",
        cli.repo_root.join(PROCESS_MERGE_BINARY).display(),
        cli.pr,
        cli.issue,
        cli.repo_root.display()
    ));
    lines.push("Would run: git push origin master".to_string());
    lines.push(format!(
        "Would query PR #{} for headRefName and run: git push origin --delete <headRefName>",
        cli.pr
    ));
    lines.push("Would print a merge summary including the process-merge receipt hash".to_string());
    lines.join("\n")
}

fn extract_receipt_hash(stdout: &str) -> Result<String, String> {
    let marker = "(receipt: ";
    let start = stdout
        .rfind(marker)
        .ok_or_else(|| format!("process-merge output did not include a receipt hash: {}", stdout.trim()))?
        + marker.len();
    let remaining = &stdout[start..];
    let end = remaining.find(')').ok_or_else(|| {
        format!(
            "process-merge output included an unterminated receipt hash: {}",
            stdout.trim()
        )
    })?;
    let receipt = remaining[..end].trim();
    if receipt.is_empty() {
        return Err("process-merge output included an empty receipt hash".to_string());
    }
    Ok(receipt.to_string())
}

fn is_missing_remote_branch(output: &ExecutionResult) -> bool {
    let combined = format!("{}\n{}", output.stdout, output.stderr).to_ascii_lowercase();
    combined.contains("remote ref does not exist")
        || combined.contains("remote ref doesn't exist")
        || combined.contains("couldn't find remote ref")
}

fn ensure_success(command: &str, output: &ExecutionResult) -> Result<(), String> {
    if matches!(output.exit_code, Some(0)) {
        Ok(())
    } else {
        Err(command_failure_message(command, output))
    }
}

fn command_failure_message(command: &str, output: &ExecutionResult) -> String {
    let code = output
        .exit_code
        .map_or_else(|| "terminated by signal".to_string(), |value| value.to_string());
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
    use std::collections::VecDeque;
    use std::sync::Mutex;

    #[derive(Default)]
    struct MockRunner {
        gh_results: Mutex<VecDeque<Result<ExecutionResult, String>>>,
        git_results: Mutex<VecDeque<Result<ExecutionResult, String>>>,
        process_merge_results: Mutex<VecDeque<Result<ExecutionResult, String>>>,
        gh_calls: Mutex<Vec<Vec<String>>>,
        git_calls: Mutex<Vec<Vec<String>>>,
        process_merge_calls: Mutex<Vec<Vec<String>>>,
    }

    impl MockRunner {
        fn with_results(
            gh_results: Vec<Result<ExecutionResult, String>>,
            git_results: Vec<Result<ExecutionResult, String>>,
            process_merge_results: Vec<Result<ExecutionResult, String>>,
        ) -> Self {
            Self {
                gh_results: Mutex::new(VecDeque::from(gh_results)),
                git_results: Mutex::new(VecDeque::from(git_results)),
                process_merge_results: Mutex::new(VecDeque::from(process_merge_results)),
                ..Self::default()
            }
        }

        fn gh_calls(&self) -> Vec<Vec<String>> {
            self.gh_calls.lock().expect("gh_calls poisoned").clone()
        }

        fn git_calls(&self) -> Vec<Vec<String>> {
            self.git_calls.lock().expect("git_calls poisoned").clone()
        }

        fn process_merge_calls(&self) -> Vec<Vec<String>> {
            self.process_merge_calls
                .lock()
                .expect("process_merge_calls poisoned")
                .clone()
        }
    }

    impl CommandRunner for MockRunner {
        fn gh(&self, _repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
            self.gh_calls
                .lock()
                .expect("gh_calls poisoned")
                .push(args.to_vec());
            self.gh_results
                .lock()
                .expect("gh_results poisoned")
                .pop_front()
                .unwrap_or_else(|| Err("missing mocked gh result".to_string()))
        }

        fn git(&self, _repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
            self.git_calls
                .lock()
                .expect("git_calls poisoned")
                .push(args.to_vec());
            self.git_results
                .lock()
                .expect("git_results poisoned")
                .pop_front()
                .unwrap_or_else(|| Err("missing mocked git result".to_string()))
        }

        fn process_merge(
            &self,
            _repo_root: &Path,
            args: &[String],
        ) -> Result<ExecutionResult, String> {
            self.process_merge_calls
                .lock()
                .expect("process_merge_calls poisoned")
                .push(args.to_vec());
            self.process_merge_results
                .lock()
                .expect("process_merge_results poisoned")
                .pop_front()
                .unwrap_or_else(|| Err("missing mocked process-merge result".to_string()))
        }
    }

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).expect("help should render");
        let help = String::from_utf8(output).expect("help should be utf-8");

        assert!(help.contains("--pr"));
        assert!(help.contains("--issue"));
        assert!(help.contains("--repo-root"));
        assert!(help.contains("--dry-run"));
    }

    #[test]
    fn parses_pr_view_json() {
        let parsed = parse_pr_view(
            r#"{
                "state": "OPEN",
                "isDraft": true,
                "mergeable": "MERGEABLE",
                "headRefName": "copilot/add-merge-pr-tool"
            }"#,
        )
        .expect("json should parse");

        assert_eq!(parsed.state, PullRequestState::Open);
        assert!(parsed.is_draft);
        assert_eq!(parsed.mergeable, Some(MergeableState::Mergeable));
        assert_eq!(
            parsed.head_ref_name.as_deref(),
            Some("copilot/add-merge-pr-tool")
        );
    }

    #[test]
    fn extract_branch_name_returns_trimmed_name() {
        let pr = PullRequestView {
            state: PullRequestState::Merged,
            is_draft: false,
            mergeable: None,
            head_ref_name: Some(" feature/cleanup ".to_string()),
        };

        assert_eq!(
            extract_branch_name(&pr).expect("branch should be extracted"),
            "feature/cleanup"
        );
    }

    #[test]
    fn extract_branch_name_rejects_missing_or_blank_name() {
        let missing = PullRequestView {
            state: PullRequestState::Merged,
            is_draft: false,
            mergeable: None,
            head_ref_name: None,
        };
        let blank = PullRequestView {
            state: PullRequestState::Merged,
            is_draft: false,
            mergeable: None,
            head_ref_name: Some("   ".to_string()),
        };

        assert!(extract_branch_name(&missing).is_err());
        assert!(extract_branch_name(&blank).is_err());
    }

    #[test]
    fn extracts_receipt_hash_from_process_merge_output() {
        let receipt = extract_receipt_hash(
            "Merge processed: #42. Copilot metrics: 12 dispatches, 5 merged (receipt: abc1234)",
        )
        .expect("receipt should parse");

        assert_eq!(receipt, "abc1234");
    }

    #[test]
    fn dry_run_reports_steps_without_running_follow_up_commands() {
        let repo_root = std::env::temp_dir().join("merge-pr-dry-run");
        fs::create_dir_all(&repo_root).expect("temp repo root should exist");
        let cli = Cli {
            pr: 1234,
            issue: 5678,
            repo_root: repo_root.clone(),
            dry_run: true,
        };
        let runner = MockRunner::default();

        let output = execute(&cli, &runner).expect("dry-run should succeed");

        assert!(output.contains("Would run: gh pr merge 1234 --squash --repo EvaLok/schema-org-json-ld"));
        assert!(output.contains("Would run: git pull --rebase origin master"));
        assert!(output.contains("Would query PR #1234 for headRefName and run: git push origin --delete <headRefName>"));
        assert!(runner.gh_calls().is_empty());
        assert!(runner.git_calls().is_empty());
        assert!(runner.process_merge_calls().is_empty());
    }

    #[test]
    fn already_merged_pr_skips_merge_steps_and_handles_missing_branch_delete() {
        let repo_root = std::env::temp_dir().join("merge-pr-already-merged");
        fs::create_dir_all(&repo_root).expect("temp repo root should exist");
        let cli = Cli {
            pr: 77,
            issue: 88,
            repo_root: repo_root.clone(),
            dry_run: false,
        };
        let runner = MockRunner::with_results(
            vec![Ok(ExecutionResult {
                exit_code: Some(0),
                stdout: r#"{"state":"MERGED","isDraft":false,"mergeable":null,"headRefName":"copilot/merged"}"#.to_string(),
                stderr: String::new(),
            })],
            vec![
                Ok(ExecutionResult {
                    exit_code: Some(0),
                    stdout: String::new(),
                    stderr: String::new(),
                }),
                Ok(ExecutionResult {
                    exit_code: Some(0),
                    stdout: String::new(),
                    stderr: String::new(),
                }),
                Ok(ExecutionResult {
                    exit_code: Some(1),
                    stdout: String::new(),
                    stderr: "error: unable to delete 'copilot/merged': remote ref does not exist".to_string(),
                }),
            ],
            vec![Ok(ExecutionResult {
                exit_code: Some(0),
                stdout: "Merge processed: #77. Copilot metrics: 9 dispatches, 4 merged (receipt: deadbeef)".to_string(),
                stderr: String::new(),
            })],
        );

        let output = execute(&cli, &runner).expect("already merged flow should succeed");

        assert!(output.contains("already merged; skipping readiness and merge steps"));
        assert!(output.contains("receipt: deadbeef"));
        assert_eq!(runner.gh_calls().len(), 1);
        assert_eq!(runner.git_calls().len(), 3);
        assert_eq!(runner.process_merge_calls().len(), 1);
    }
}
