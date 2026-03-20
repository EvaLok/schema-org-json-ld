use clap::Parser;
use record_dispatch::{
    apply_dispatch_patch, build_dispatch_patch, concurrency_warning_message,
    dispatch_commit_message, enforce_pipeline_gate, resolve_model, update_review_dispatch_tracking,
    CommandRunner, PipelineGateError, ProcessRunner,
};
use state_schema::{
    commit_state_json, current_cycle_from_state, current_utc_timestamp, read_state_value,
    transition_cycle_phase, write_state_value,
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "record-dispatch")]
struct Cli {
    /// GitHub issue number that was dispatched
    #[arg(long)]
    issue: u64,

    /// Short title/description of the dispatch
    #[arg(long)]
    title: String,

    /// Model used for the dispatch
    #[arg(long)]
    model: Option<String>,

    /// Bypass pipeline gate for review agent dispatch only. Logged in receipts.
    #[arg(long)]
    review_dispatch: bool,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let runner = ProcessRunner;
    run_with_runner(cli, &runner, &mut |warning| eprintln!("{warning}"))
}

fn run_with_runner(
    cli: Cli,
    runner: &dyn CommandRunner,
    warn: &mut dyn FnMut(&str),
) -> Result<(), String> {
    let pipeline_warning = match enforce_pipeline_gate(&cli.repo_root, cli.review_dispatch, runner)
    {
        Ok(warning) => warning,
        Err(PipelineGateError::ExecutionFailed(detail)) => {
            eprintln!("pipeline-check execution error: {detail}");
            return Err(record_dispatch::PIPELINE_GATE_FAILURE_MESSAGE.to_string());
        }
        Err(PipelineGateError::Failed) => {
            return Err(record_dispatch::PIPELINE_GATE_FAILURE_MESSAGE.to_string());
        }
    };

    if let Some(warning) = pipeline_warning {
        warn(warning);
    }

    let model = resolve_model(cli.model.as_deref(), &cli.repo_root)?;
    let mut state_value = read_state_value(&cli.repo_root)?;
    let review_dispatch_warning =
        update_review_dispatch_tracking(&mut state_value, cli.review_dispatch)?;
    if let Some(warning) = review_dispatch_warning.as_deref() {
        warn(warning);
    }
    let dispatched_at = current_utc_timestamp();
    let current_cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /cycle_phase/cycle or /last_cycle/number in state.json" {
            "missing numeric /cycle_phase/cycle or /last_cycle/number in docs/state.json"
                .to_string()
        } else {
            error
        }
    })?;

    let patch = build_dispatch_patch(
        &state_value,
        current_cycle,
        cli.issue,
        &cli.title,
        &model,
        &dispatched_at,
    )?;
    let already_recorded = match apply_dispatch_patch(&mut state_value, &patch) {
        Ok(()) => false,
        Err(error) if error.contains("already contains an entry for issue") => {
            eprintln!(
                "Note: session for #{} already recorded (likely by dispatch-review); skipping append, applying phase transition only",
                cli.issue
            );
            true
        }
        Err(error) => return Err(error),
    };
    let current_phase = state_value
        .pointer("/cycle_phase/phase")
        .and_then(|value| value.as_str())
        .unwrap_or("unknown")
        .to_string();
    let phase_transitioned = if current_phase == "close_out" {
        transition_cycle_phase(&mut state_value, current_cycle, "complete")?;
        true
    } else {
        false
    };
    write_state_value(&cli.repo_root, &state_value)?;

    let commit_message = dispatch_commit_message(cli.issue, patch.current_cycle);
    let receipt = commit_state_json(&cli.repo_root, &commit_message)?;
    if already_recorded {
        if phase_transitioned {
            println!(
                "Phase transitioned to complete (session already recorded for #{}). (receipt: {})",
                cli.issue, receipt
            );
        } else {
            println!(
                "Dispatch recorded (phase unchanged: {}). Session already recorded for #{}. (receipt: {})",
                current_phase, cli.issue, receipt
            );
        }
    } else if phase_transitioned {
        println!(
            "Dispatch recorded: #{} \"{}\" (model: {}). In-flight: {} (receipt: {})",
            cli.issue, cli.title, model, patch.in_flight, receipt
        );
    } else {
        println!(
            "Dispatch recorded (phase unchanged: {}). #{} \"{}\" (model: {}). In-flight: {} (receipt: {})",
            current_phase, cli.issue, cli.title, model, patch.in_flight, receipt
        );
    }
    if patch.in_flight >= 3 {
        warn(&concurrency_warning_message(patch.in_flight));
    }

    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use std::{
        fs,
        path::Path,
        process::Command,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--issue"));
        assert!(help.contains("--title"));
        assert!(help.contains("--model"));
        assert!(help.contains("--review-dispatch"));
        assert!(!help.contains("--skip-pipeline-gate"));
        assert!(help.contains("--repo-root"));
    }

    #[test]
    fn concurrency_warning_threshold_is_triggered_at_three() {
        let patch = record_dispatch::build_dispatch_patch(
            &serde_json::json!({
                "agent_sessions": [
                    { "status": "in_flight" },
                    { "status": "dispatched" }
                ],
                "copilot_metrics": {
                    "total_dispatches": 2,
                    "in_flight": 2,
                    "resolved": 0,
                    "dispatch_log_latest": "#601 old dispatch (cycle 164)"
                },
                "field_inventory": {
                    "fields": {
                        "copilot_metrics.in_flight": { "last_refreshed": "cycle 163" },
                        "copilot_metrics.dispatch_to_pr_rate": { "last_refreshed": "cycle 163" },
                        "copilot_metrics.pr_merge_rate": { "last_refreshed": "cycle 163" }
                    }
                }
            }),
            164,
            602,
            "Example dispatch",
            "gpt-5.4",
            "2026-03-07T13:00:00Z",
        )
        .expect("patch should build");
        assert!(patch.in_flight >= 3);
    }

    #[test]
    fn run_leaves_worklog_unchanged_and_commits_only_state_json() {
        let repo = TempRepo::new();
        repo.init();
        let older_worklog = repo.write_worklog("2026-03-09", "235959-older.md", 0);
        std::thread::sleep(Duration::from_millis(20));
        let latest_worklog = repo.write_worklog("2026-03-10", "142511-current.md", 0);

        run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_dispatch: true,
            repo_root: repo.path().to_path_buf(),
        })
        .expect("dispatch should succeed");

        let latest_content =
            fs::read_to_string(&latest_worklog).expect("latest worklog should be readable");
        assert!(latest_content.contains("- **In-flight agent sessions**: 0"));
        let older_content =
            fs::read_to_string(&older_worklog).expect("older worklog should be readable");
        assert!(older_content.contains("- **In-flight agent sessions**: 0"));

        let output = Command::new("git")
            .arg("-C")
            .arg(repo.path())
            .args(["show", "--name-only", "--pretty=format:%B", "HEAD"])
            .output()
            .expect("git show should execute");
        assert!(output.status.success());
        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(stdout.contains("state(record-dispatch): #602 dispatched [cycle 164]"));
        assert!(stdout.contains("docs/state.json"));
        assert!(!stdout.contains("docs/worklog/2026-03-10/142511-current.md"));

        let state: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(repo.path().join("docs/state.json"))
                .expect("state file should be readable"),
        )
        .expect("state file should parse");
        assert_eq!(
            state.pointer("/cycle_phase/phase"),
            Some(&serde_json::json!("complete"))
        );
        assert_eq!(
            state.pointer("/cycle_phase/cycle"),
            Some(&serde_json::json!(164))
        );
        assert_ne!(
            state
                .pointer("/cycle_phase/phase_entered_at")
                .and_then(serde_json::Value::as_str),
            Some("2026-03-07T12:00:00Z")
        );
        assert_eq!(
            state
                .pointer("/field_inventory/fields/cycle_phase/last_refreshed")
                .and_then(serde_json::Value::as_str),
            Some("cycle 164")
        );
    }

    #[test]
    fn run_transitions_close_out_phase_to_complete() {
        let repo = TempRepo::new();
        repo.init_with_phase("close_out");

        run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_dispatch: true,
            repo_root: repo.path().to_path_buf(),
        })
        .expect("dispatch should succeed");

        let state = repo.read_state();
        assert_eq!(
            state.pointer("/cycle_phase/phase"),
            Some(&serde_json::json!("complete"))
        );
        assert_ne!(
            state
                .pointer("/cycle_phase/phase_entered_at")
                .and_then(serde_json::Value::as_str),
            Some("2026-03-07T12:00:00Z")
        );
        assert_eq!(
            state
                .pointer("/field_inventory/fields/cycle_phase/last_refreshed")
                .and_then(serde_json::Value::as_str),
            Some("cycle 164")
        );
    }

    #[test]
    fn run_keeps_work_phase_unchanged_for_mid_cycle_dispatch() {
        let repo = TempRepo::new();
        repo.init_with_phase("work");

        run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_dispatch: true,
            repo_root: repo.path().to_path_buf(),
        })
        .expect("dispatch should succeed");

        let state = repo.read_state();
        assert_eq!(
            state.pointer("/cycle_phase/phase"),
            Some(&serde_json::json!("work"))
        );
        assert_eq!(
            state
                .pointer("/cycle_phase/phase_entered_at")
                .and_then(serde_json::Value::as_str),
            Some("2026-03-07T12:00:00Z")
        );
        assert_eq!(
            state
                .pointer("/field_inventory/fields/cycle_phase/last_refreshed")
                .and_then(serde_json::Value::as_str),
            Some("cycle 163")
        );
    }

    #[test]
    fn run_succeeds_when_worklog_is_missing() {
        let repo = TempRepo::new();
        repo.init();

        run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_dispatch: true,
            repo_root: repo.path().to_path_buf(),
        })
        .expect("missing worklog should only warn");

        let output = Command::new("git")
            .arg("-C")
            .arg(repo.path())
            .args(["log", "-1", "--pretty=%B"])
            .output()
            .expect("git log should execute");
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout).trim(),
            "state(record-dispatch): #602 dispatched [cycle 164]"
        );
    }

    #[test]
    fn run_fails_when_pipeline_check_fails() {
        let repo = TempRepo::new();
        repo.init();
        let before = fs::read_to_string(repo.path().join("docs/state.json"))
            .expect("state file should be readable before run");
        let mut warnings = Vec::new();

        let error = run_with_runner(
            Cli {
                issue: 602,
                title: "Example dispatch".to_string(),
                model: Some("gpt-5.4".to_string()),
                review_dispatch: false,
                repo_root: repo.path().to_path_buf(),
            },
            &MockRunner::with_exit_code(Some(1)),
            &mut |warning| warnings.push(warning.to_string()),
        )
        .expect_err("pipeline failure should stop dispatch");

        assert_eq!(
            error,
            record_dispatch::PIPELINE_GATE_FAILURE_MESSAGE.to_string()
        );
        assert!(warnings.is_empty());
        let after = fs::read_to_string(repo.path().join("docs/state.json"))
            .expect("state file should be readable after failed run");
        assert_eq!(after, before);
    }

    #[test]
    fn run_proceeds_when_pipeline_check_passes() {
        let repo = TempRepo::new();
        repo.init();
        let mut warnings = Vec::new();

        run_with_runner(
            Cli {
                issue: 602,
                title: "Example dispatch".to_string(),
                model: Some("gpt-5.4".to_string()),
                review_dispatch: false,
                repo_root: repo.path().to_path_buf(),
            },
            &MockRunner::with_exit_code(Some(0)),
            &mut |warning| warnings.push(warning.to_string()),
        )
        .expect("passing pipeline gate should allow dispatch");

        assert!(warnings.is_empty());
        let state = repo.read_state();
        assert_eq!(
            state.pointer("/copilot_metrics/in_flight"),
            Some(&serde_json::json!(1))
        );
        assert_eq!(
            state.pointer("/agent_sessions/2/issue"),
            Some(&serde_json::json!(602))
        );
    }

    #[test]
    fn review_dispatch_bypasses_check_with_warning() {
        let repo = TempRepo::new();
        repo.init();
        let mut warnings = Vec::new();
        let runner = MockRunner::with_error("runner should not be called when skipping");

        run_with_runner(
            Cli {
                issue: 602,
                title: "Example dispatch".to_string(),
                model: Some("gpt-5.4".to_string()),
                review_dispatch: true,
                repo_root: repo.path().to_path_buf(),
            },
            &runner,
            &mut |warning| warnings.push(warning.to_string()),
        )
        .expect("review-dispatch flag should bypass pipeline gate");

        assert_eq!(
            warnings,
            vec![record_dispatch::REVIEW_DISPATCH_WARNING.to_string()]
        );
        assert_eq!(runner.call_count(), 0);
        let state = repo.read_state();
        assert_eq!(
            state.pointer("/copilot_metrics/in_flight"),
            Some(&serde_json::json!(1))
        );
        assert_eq!(
            state.pointer("/review_dispatch_consecutive"),
            Some(&serde_json::json!(1))
        );
    }

    #[test]
    fn run_fails_closed_when_pipeline_check_cannot_execute() {
        let repo = TempRepo::new();
        repo.init();
        let mut warnings = Vec::new();

        let error = run_with_runner(
            Cli {
                issue: 602,
                title: "Example dispatch".to_string(),
                model: Some("gpt-5.4".to_string()),
                review_dispatch: false,
                repo_root: repo.path().to_path_buf(),
            },
            &MockRunner::with_error("missing bash"),
            &mut |warning| warnings.push(warning.to_string()),
        )
        .expect_err("command execution failure should stop dispatch");

        assert_eq!(
            error,
            record_dispatch::PIPELINE_GATE_FAILURE_MESSAGE.to_string()
        );
        assert!(warnings.is_empty());
    }

    #[test]
    fn review_dispatch_warns_after_three_consecutive_bypasses() {
        let repo = TempRepo::new();
        repo.init();
        repo.set_review_dispatch_consecutive(2);
        let mut warnings = Vec::new();
        let runner = MockRunner::with_error("runner should not be called when bypassing");

        run_with_runner(
            Cli {
                issue: 602,
                title: "Example dispatch".to_string(),
                model: Some("gpt-5.4".to_string()),
                review_dispatch: true,
                repo_root: repo.path().to_path_buf(),
            },
            &runner,
            &mut |warning| warnings.push(warning.to_string()),
        )
        .expect("review dispatch should succeed");

        assert_eq!(runner.call_count(), 0);
        assert!(warnings.contains(&record_dispatch::REVIEW_DISPATCH_WARNING.to_string()));
        assert!(warnings.contains(&record_dispatch::review_dispatch_consecutive_warning(3)));
        let state = repo.read_state();
        assert_eq!(
            state.pointer("/review_dispatch_consecutive"),
            Some(&serde_json::json!(3))
        );
    }

    #[test]
    fn non_review_dispatch_resets_consecutive_bypass_counter() {
        let repo = TempRepo::new();
        repo.init();
        repo.set_review_dispatch_consecutive(2);
        let mut warnings = Vec::new();
        let runner = MockRunner::with_exit_code(Some(0));

        run_with_runner(
            Cli {
                issue: 602,
                title: "Example dispatch".to_string(),
                model: Some("gpt-5.4".to_string()),
                review_dispatch: false,
                repo_root: repo.path().to_path_buf(),
            },
            &runner,
            &mut |warning| warnings.push(warning.to_string()),
        )
        .expect("non-review dispatch should succeed");

        assert_eq!(runner.call_count(), 1);
        assert!(warnings.is_empty());
        let state = repo.read_state();
        assert_eq!(
            state.pointer("/review_dispatch_consecutive"),
            Some(&serde_json::json!(0))
        );
    }

    struct MockRunner {
        result: Result<record_dispatch::ExecutionResult, String>,
        call_count: std::cell::Cell<usize>,
    }

    impl MockRunner {
        fn with_exit_code(exit_code: Option<i32>) -> Self {
            Self {
                result: Ok(record_dispatch::ExecutionResult { exit_code }),
                call_count: std::cell::Cell::new(0),
            }
        }

        fn with_error(message: &str) -> Self {
            Self {
                result: Err(message.to_string()),
                call_count: std::cell::Cell::new(0),
            }
        }

        fn call_count(&self) -> usize {
            self.call_count.get()
        }
    }

    impl CommandRunner for MockRunner {
        fn run_pipeline_check(
            &self,
            _repo_root: &Path,
        ) -> Result<record_dispatch::ExecutionResult, String> {
            self.call_count.set(self.call_count.get() + 1);
            self.result.clone()
        }
    }

    struct TempRepo {
        path: PathBuf,
    }

    impl TempRepo {
        fn new() -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "record-dispatch-main-test-{}-{}",
                std::process::id(),
                unique
            ));
            fs::create_dir_all(path.join("docs")).expect("temp repo should be created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn init(&self) {
            self.init_with_phase("close_out");
        }

        fn init_with_phase(&self, phase: &str) {
            self.write_state(phase);
            git_success(self.path(), ["init"]);
            git_success(
                self.path(),
                ["config", "user.name", "Record Dispatch Tests"],
            );
            git_success(
                self.path(),
                ["config", "user.email", "record-dispatch-tests@example.com"],
            );
            git_success(self.path(), ["add", "docs/state.json"]);
            git_success(self.path(), ["commit", "-m", "initial state"]);
        }

        fn write_state(&self, phase: &str) {
            let state = serde_json::json!({
                "agent_sessions": [
                    {
                        "issue": 600,
                        "title": "Merged change",
                        "dispatched_at": "2026-03-01T00:00:00Z",
                        "model": "gpt-5.4",
                        "status": "merged",
                        "pr": 700,
                        "merged_at": "2026-03-02T00:00:00Z"
                    },
                    {
                        "issue": 601,
                        "title": "Closed change",
                        "dispatched_at": "2026-03-03T00:00:00Z",
                        "model": "gpt-5.4",
                        "status": "closed_without_pr"
                    }
                ],
                "last_cycle": {
                    "number": 164
                },
                "cycle_phase": {
                    "cycle": 164,
                    "phase": phase,
                    "phase_entered_at": "2026-03-07T12:00:00Z"
                },
                "copilot_metrics": {
                    "total_dispatches": 2,
                    "resolved": 2,
                    "merged": 1,
                    "closed_without_pr": 1,
                    "reviewed_awaiting_eva": 0,
                    "in_flight": 0,
                    "produced_pr": 1,
                    "pr_merge_rate": "100.0%",
                    "dispatch_to_pr_rate": "50.0%",
                    "dispatch_log_latest": "#601 Closed change (cycle 164)"
                },
                "field_inventory": {
                    "fields": {
                        "copilot_metrics.in_flight": {
                            "last_refreshed": "cycle 163"
                        },
                        "cycle_phase": {
                            "last_refreshed": "cycle 163"
                        },
                        "copilot_metrics.pr_merge_rate": {
                            "last_refreshed": "cycle 163"
                        },
                        "copilot_metrics.dispatch_to_pr_rate": {
                            "last_refreshed": "cycle 163"
                        }
                    }
                }
            });
            fs::write(
                self.path().join("docs/state.json"),
                serde_json::to_string_pretty(&state).expect("state file should serialize"),
            )
            .expect("state file should be written");
        }

        fn read_state(&self) -> serde_json::Value {
            serde_json::from_str(
                &fs::read_to_string(self.path().join("docs/state.json"))
                    .expect("state file should be readable"),
            )
            .expect("state file should parse")
        }

        fn set_review_dispatch_consecutive(&self, count: u64) {
            let mut state = self.read_state();
            state["review_dispatch_consecutive"] = serde_json::json!(count);
            fs::write(
                self.path().join("docs/state.json"),
                serde_json::to_string_pretty(&state).expect("state should serialize"),
            )
            .expect("state file should be updated");
        }

        fn write_worklog(&self, date: &str, name: &str, in_flight: i64) -> PathBuf {
            let dir = self.path().join("docs/worklog").join(date);
            fs::create_dir_all(&dir).expect("worklog dir should be created");
            let path = dir.join(name);
            fs::write(
                &path,
                format!(
                    "# Cycle 164 — 2026-03-10 14:25 UTC\n\n## Current state\n\n- **In-flight agent sessions**: {}\n- **Pipeline status**: PASS (8/8)\n",
                    in_flight
                ),
            )
            .expect("worklog should be written");
            let relative_path = path
                .strip_prefix(self.path())
                .expect("worklog should be under repo root")
                .to_string_lossy()
                .into_owned();
            git_success(self.path(), ["add", "--", relative_path.as_str()]);
            let commit_message = format!("add worklog {}", name);
            git_success(self.path(), ["commit", "-m", commit_message.as_str()]);
            path
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn git_success<I, S>(repo_root: &Path, args: I)
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
}
