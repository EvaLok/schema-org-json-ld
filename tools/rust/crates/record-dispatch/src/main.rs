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

    /// Review finding this dispatch addresses, formatted as CYCLE:INDEX
    #[arg(long)]
    addresses_finding: Option<AddressedFinding>,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct AddressedFinding {
    cycle: u64,
    index: u64,
}

impl std::str::FromStr for AddressedFinding {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (cycle, index) = value.split_once(':').ok_or_else(|| {
            "--addresses-finding must use CYCLE:INDEX format (for example 316:2)".to_string()
        })?;
        let cycle = cycle
            .parse::<u64>()
            .map_err(|_| "--addresses-finding cycle must be a positive integer".to_string())?;
        let index = index
            .parse::<u64>()
            .map_err(|_| "--addresses-finding index must be a positive integer".to_string())?;
        if cycle == 0 {
            return Err("--addresses-finding cycle must be greater than zero".to_string());
        }
        if index == 0 {
            return Err("--addresses-finding index must be greater than zero".to_string());
        }

        Ok(Self { cycle, index })
    }
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
    if let Some(addressed_finding) = cli.addresses_finding.as_ref() {
        reconcile_review_history_dispatch(&mut state_value, addressed_finding, warn)?;
    }
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

fn reconcile_review_history_dispatch(
    state: &mut serde_json::Value,
    addressed_finding: &AddressedFinding,
    warn: &mut dyn FnMut(&str),
) -> Result<(), String> {
    let finding_zero_based_index = (addressed_finding.index - 1) as usize;
    let history = state
        .pointer_mut("/review_agent/history")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "missing array /review_agent/history in docs/state.json".to_string())?;

    let entry = history
        .iter_mut()
        .find(|entry| {
            entry.get("cycle").and_then(serde_json::Value::as_u64) == Some(addressed_finding.cycle)
        })
        .ok_or_else(|| {
            format!(
                "review history entry for cycle {} was not found in docs/state.json",
                addressed_finding.cycle
            )
        })?;

    let finding_count = entry
        .get("finding_count")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| {
            format!(
                "review history entry for cycle {} is missing a numeric finding_count",
                addressed_finding.cycle
            )
        })?;
    if addressed_finding.index > finding_count {
        return Err(format!(
            "--addresses-finding {}:{} is out of range; cycle {} has {} finding(s)",
            addressed_finding.cycle,
            addressed_finding.index,
            addressed_finding.cycle,
            finding_count
        ));
    }

    let finding_disposition_path = format!("/finding_dispositions/{}", finding_zero_based_index);
    let finding_disposition = entry
        .pointer(&finding_disposition_path)
        .ok_or_else(|| {
            format!(
                "review history entry for cycle {} is missing finding_dispositions[{}] for finding {}",
                addressed_finding.cycle,
                finding_zero_based_index,
                addressed_finding.index
            )
        })?;
    let current_disposition = finding_disposition
        .get("disposition")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            format!(
                "review history entry for cycle {} finding {} is missing a string disposition",
                addressed_finding.cycle,
                addressed_finding.index
            )
        })?;
    if current_disposition != "deferred" {
        warn(&format!(
            "review history entry for cycle {} finding {} has disposition {:?}; expected \"deferred\", leaving review history unchanged",
            addressed_finding.cycle, addressed_finding.index, current_disposition
        ));
        return Ok(());
    }

    let deferred = entry
        .get("deferred")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| {
            format!(
                "review history entry for cycle {} is missing a numeric deferred count",
                addressed_finding.cycle
            )
        })?;
    if deferred == 0 {
        return Err(format!(
            "review history entry for cycle {} has no deferred findings left to mark as dispatch_created",
            addressed_finding.cycle
        ));
    }

    let dispatch_created = entry
        .get("dispatch_created")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);

    let entry_object = entry.as_object_mut().ok_or_else(|| {
        format!(
            "review history entry for cycle {} must be an object",
            addressed_finding.cycle
        )
    })?;
    let finding_dispositions = entry_object
        .get_mut("finding_dispositions")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| {
            format!(
                "review history entry for cycle {} is missing an array finding_dispositions",
                addressed_finding.cycle
            )
        })?;
    let finding_disposition = finding_dispositions
        .get_mut(finding_zero_based_index)
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| {
            format!(
                "review history entry for cycle {} finding {} (finding_dispositions[{}]) must be an object",
                addressed_finding.cycle,
                addressed_finding.index,
                finding_zero_based_index
            )
        })?;
    finding_disposition.insert(
        "disposition".to_string(),
        serde_json::json!("dispatch_created"),
    );
    entry_object.insert("deferred".to_string(), serde_json::json!(deferred - 1));
    entry_object.insert(
        "dispatch_created".to_string(),
        serde_json::json!(dispatch_created + 1),
    );

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
        assert!(help.contains("--addresses-finding"));
        assert!(!help.contains("--skip-pipeline-gate"));
        assert!(help.contains("--repo-root"));
    }

    #[test]
    fn cli_rejects_malformed_addresses_finding_flag() {
        let error = Cli::try_parse_from([
            "record-dispatch",
            "--issue",
            "602",
            "--title",
            "Example dispatch",
            "--review-dispatch",
            "--addresses-finding",
            "164",
        ])
        .expect_err("malformed finding reference should fail");

        let rendered = error.to_string();
        assert!(rendered.contains("--addresses-finding"));
        assert!(rendered.contains("CYCLE:INDEX format"));
    }

    #[test]
    fn run_reconciles_review_history_when_addresses_finding_is_provided() {
        let repo = TempRepo::new();
        repo.init();
        repo.set_review_history_entry_with_dispositions(
            164,
            &["deferred", "deferred", "actioned"],
        );

        run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_dispatch: true,
            addresses_finding: Some("164:2".parse().expect("finding ref should parse")),
            repo_root: repo.path().to_path_buf(),
        })
        .expect("dispatch should reconcile review history");

        let state = repo.read_state();
        assert_eq!(
            state.pointer("/review_agent/history/0/deferred"),
            Some(&serde_json::json!(1))
        );
        assert_eq!(
            state.pointer("/review_agent/history/0/dispatch_created"),
            Some(&serde_json::json!(1))
        );
        assert_eq!(
            state.pointer("/review_agent/history/0/finding_dispositions/1/disposition"),
            Some(&serde_json::json!("dispatch_created"))
        );
    }

    #[test]
    fn run_warns_when_target_finding_is_not_deferred() {
        let repo = TempRepo::new();
        repo.init();
        repo.set_review_history_entry_with_dispositions(
            164,
            &["deferred", "actioned", "deferred"],
        );
        let mut warnings = Vec::new();
        let runner = MockRunner::with_error("runner should not be called when bypassing");

        run_with_runner(
            Cli {
                issue: 602,
                title: "Example dispatch".to_string(),
                model: Some("gpt-5.4".to_string()),
                review_dispatch: true,
                addresses_finding: Some("164:2".parse().expect("finding ref should parse")),
                repo_root: repo.path().to_path_buf(),
            },
            &runner,
            &mut |warning| warnings.push(warning.to_string()),
        )
        .expect("non-deferred finding should warn without failing");

        assert_eq!(runner.call_count(), 0);
        assert!(
            warnings.contains(&record_dispatch::REVIEW_DISPATCH_WARNING.to_string()),
            "expected standard review-dispatch warning"
        );
        assert!(
            warnings.iter().any(|warning| warning.contains(
                "review history entry for cycle 164 finding 2 has disposition \"actioned\""
            )),
            "expected warning about non-deferred finding disposition, got {warnings:?}"
        );

        let state = repo.read_state();
        assert_eq!(
            state.pointer("/review_agent/history/0/deferred"),
            Some(&serde_json::json!(2))
        );
        assert_eq!(
            state.pointer("/review_agent/history/0/dispatch_created"),
            Some(&serde_json::json!(0))
        );
        assert_eq!(
            state.pointer("/review_agent/history/0/finding_dispositions/1/disposition"),
            Some(&serde_json::json!("actioned"))
        );
    }

    #[test]
    fn run_leaves_review_history_unchanged_when_addresses_finding_is_omitted() {
        let repo = TempRepo::new();
        repo.init();
        repo.set_review_history_entry(164, 3, 2, 0);
        let before = repo.read_state();

        run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_dispatch: true,
            addresses_finding: None,
            repo_root: repo.path().to_path_buf(),
        })
        .expect("dispatch without finding ref should still succeed");

        let after = repo.read_state();
        assert_eq!(
            after.pointer("/review_agent/history"),
            before.pointer("/review_agent/history")
        );
    }

    #[test]
    fn run_fails_when_addresses_finding_cycle_is_missing() {
        let repo = TempRepo::new();
        repo.init();
        repo.set_review_history_entry(164, 3, 2, 0);

        let error = run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_dispatch: true,
            addresses_finding: Some("999:1".parse().expect("finding ref should parse")),
            repo_root: repo.path().to_path_buf(),
        })
        .expect_err("missing review cycle should fail");

        assert_eq!(
            error,
            "review history entry for cycle 999 was not found in docs/state.json"
        );
    }

    #[test]
    fn run_fails_when_addresses_finding_index_is_out_of_range() {
        let repo = TempRepo::new();
        repo.init();
        repo.set_review_history_entry(164, 3, 2, 0);

        let error = run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_dispatch: true,
            addresses_finding: Some("164:4".parse().expect("finding ref should parse")),
            repo_root: repo.path().to_path_buf(),
        })
        .expect_err("out-of-range finding index should fail");

        assert_eq!(
            error,
            "--addresses-finding 164:4 is out of range; cycle 164 has 3 finding(s)"
        );
    }

    #[test]
    fn concurrency_warning_threshold_is_triggered_at_three() {
        let patch = record_dispatch::build_dispatch_patch(
            &serde_json::json!({
                "agent_sessions": [
                    { "status": "in_flight" },
                    { "status": "dispatched" }
                ],
                "dispatch_log_latest": "#601 old dispatch (cycle 164)",
                "field_inventory": {
                    "fields": {
                        "in_flight_sessions": { "last_refreshed": "cycle 163" }
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
            addresses_finding: None,
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
            addresses_finding: None,
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
            addresses_finding: None,
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
            addresses_finding: None,
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
                addresses_finding: None,
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
                addresses_finding: None,
                repo_root: repo.path().to_path_buf(),
            },
            &MockRunner::with_exit_code(Some(0)),
            &mut |warning| warnings.push(warning.to_string()),
        )
        .expect("passing pipeline gate should allow dispatch");

        assert!(warnings.is_empty());
        let state = repo.read_state();
        assert_eq!(
            state.pointer("/in_flight_sessions"),
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
                addresses_finding: None,
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
            state.pointer("/in_flight_sessions"),
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
                addresses_finding: None,
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
                addresses_finding: None,
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
                addresses_finding: None,
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
                "dispatch_log_latest": "#601 Closed change (cycle 164)",
                "in_flight_sessions": 0,
                "field_inventory": {
                    "fields": {
                        "cycle_phase": {
                            "last_refreshed": "cycle 163"
                        },
                        "in_flight_sessions": {
                            "last_refreshed": "cycle 163"
                        }
                    }
                },
                "review_agent": {
                    "history": []
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

        fn set_review_history_entry(
            &self,
            cycle: u64,
            finding_count: u64,
            deferred: u64,
            dispatch_created: u64,
        ) {
            let mut state = self.read_state();
            state["review_agent"] = serde_json::json!({
                "history": [
                    {
                        "cycle": cycle,
                        "finding_count": finding_count,
                        "complacency_score": 0,
                        "categories": ["correctness"],
                        "actioned": 0,
                        "deferred": deferred,
                        "dispatch_created": dispatch_created,
                        "actioned_failed": 0,
                        "verified_resolved": 0,
                        "ignored": 0
                    }
                ]
            });
            fs::write(
                self.path().join("docs/state.json"),
                serde_json::to_string_pretty(&state).expect("state should serialize"),
            )
            .expect("state file should be updated");
        }

        fn set_review_history_entry_with_dispositions(&self, cycle: u64, dispositions: &[&str]) {
            let mut actioned = 0_u64;
            let mut deferred = 0_u64;
            let mut dispatch_created = 0_u64;
            let mut actioned_failed = 0_u64;
            let mut verified_resolved = 0_u64;
            let mut ignored = 0_u64;

            let finding_dispositions: Vec<serde_json::Value> = dispositions
                .iter()
                .map(|disposition| {
                    match *disposition {
                        "actioned" => actioned += 1,
                        "deferred" => deferred += 1,
                        "dispatch_created" => dispatch_created += 1,
                        "actioned_failed" => actioned_failed += 1,
                        "verified_resolved" => verified_resolved += 1,
                        "ignored" => ignored += 1,
                        other => panic!("unsupported test disposition {other}"),
                    }

                    serde_json::json!({
                        "category": "correctness",
                        "disposition": disposition,
                    })
                })
                .collect();

            let mut state = self.read_state();
            state["review_agent"] = serde_json::json!({
                "history": [
                    {
                        "cycle": cycle,
                        "finding_count": dispositions.len(),
                        "complacency_score": 0,
                        "categories": ["correctness"],
                        "actioned": actioned,
                        "deferred": deferred,
                        "dispatch_created": dispatch_created,
                        "actioned_failed": actioned_failed,
                        "verified_resolved": verified_resolved,
                        "ignored": ignored,
                        "finding_dispositions": finding_dispositions,
                    }
                ]
            });
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
                    "# Cycle 164 — 2026-03-10 14:25 UTC\n\n## Pre-dispatch state\n\n*Snapshot before review dispatch — final counters may differ after C6.*\n- **In-flight agent sessions**: {}\n- **Pipeline status**: PASS (8/8)\n",
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
