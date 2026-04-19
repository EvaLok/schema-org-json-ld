use clap::Parser;
use record_dispatch::{
    apply_dispatch_patch, build_dispatch_patch, concurrency_warning_message,
    dispatch_commit_message, enforce_pipeline_gate, resolve_model, restore_sealed_last_cycle,
    snapshot_sealed_last_cycle, sync_last_cycle_summary_after_dispatch,
    update_review_dispatch_tracking, CommandRunner, PipelineGateError, ProcessRunner,
};
use state_schema::{
    current_cycle_from_state, current_utc_timestamp, read_state_value, transition_cycle_phase,
    write_state_value,
};
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::UNIX_EPOCH,
};

const POST_DISPATCH_DELTA_HEADING: &str = "## Post-dispatch delta";

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
        Err(PipelineGateError::ReviewDispatchBlocked(message)) => {
            return Err(message);
        }
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
    let current_phase = state_value
        .pointer("/cycle_phase/phase")
        .and_then(|value| value.as_str())
        .unwrap_or("unknown")
        .to_string();
    let sealed_last_cycle = snapshot_sealed_last_cycle(&state_value, &current_phase);
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
    let duplicate_session_error = match apply_dispatch_patch(&mut state_value, &patch) {
        Ok(_) => false,
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
    let phase_transitioned = if current_phase == "close_out" {
        transition_cycle_phase(&mut state_value, current_cycle, "complete")?;
        true
    } else {
        false
    };
    restore_sealed_last_cycle(&mut state_value, sealed_last_cycle)?;
    sync_last_cycle_summary_after_dispatch(&mut state_value, patch.current_cycle)?;
    write_state_value(&cli.repo_root, &state_value)?;
    let worklog_path =
        sync_post_dispatch_worklog(&cli.repo_root, &state_value, patch.current_cycle)?;

    let commit_message = dispatch_commit_message(cli.issue, patch.current_cycle);
    let receipt =
        commit_dispatch_artifacts(&cli.repo_root, &commit_message, worklog_path.as_deref())?;
    if duplicate_session_error {
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

fn sync_post_dispatch_worklog(
    repo_root: &Path,
    state: &serde_json::Value,
    cycle: u64,
) -> Result<Option<PathBuf>, String> {
    let Some(worklog_path) = resolve_post_dispatch_worklog_path(repo_root, state, cycle)? else {
        return Ok(None);
    };
    let content = fs::read_to_string(&worklog_path)
        .map_err(|error| format!("failed to read {}: {}", worklog_path.display(), error))?;
    let in_flight_sessions = state
        .pointer("/in_flight_sessions")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "missing numeric /in_flight_sessions in docs/state.json".to_string())?;
    let last_cycle_summary = state
        .pointer("/last_cycle/summary")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| "missing string /last_cycle/summary in docs/state.json".to_string())?;
    let dispatch_count = dispatch_count_clause(last_cycle_summary);
    let updated = render_post_dispatch_delta(
        &content,
        in_flight_sessions,
        dispatch_count,
        last_cycle_summary,
    );
    if updated != content {
        fs::write(&worklog_path, updated)
            .map_err(|error| format!("failed to write {}: {}", worklog_path.display(), error))?;
    }
    Ok(Some(worklog_path))
}

fn resolve_post_dispatch_worklog_path(
    repo_root: &Path,
    state: &serde_json::Value,
    cycle: u64,
) -> Result<Option<PathBuf>, String> {
    if let Some(worklog_path) = find_worklog_for_cycle(repo_root, cycle)? {
        return Ok(Some(worklog_path));
    }

    let Some(previous_cycle) = state
        .pointer("/last_cycle/number")
        .and_then(serde_json::Value::as_u64)
        .filter(|previous_cycle| *previous_cycle != cycle)
    else {
        return Ok(None);
    };

    let Some(worklog_path) = find_worklog_for_cycle(repo_root, previous_cycle)? else {
        return Ok(None);
    };

    eprintln!(
        "record-dispatch: no cycle {} worklog found; syncing cycle {} worklog instead ({})",
        cycle,
        previous_cycle,
        worklog_path.display()
    );
    Ok(Some(worklog_path))
}

fn render_post_dispatch_delta(
    content: &str,
    in_flight_sessions: u64,
    dispatch_count: &str,
    last_cycle_summary: &str,
) -> String {
    let base = if let Some(index) = content.find(&format!("\n{POST_DISPATCH_DELTA_HEADING}\n")) {
        &content[..index]
    } else {
        content
    }
    .trim_end_matches('\n');
    format!(
        "{base}\n\n{POST_DISPATCH_DELTA_HEADING}\n\n- **In-flight agent sessions**: {in_flight_sessions}\n- **Dispatch count**: {dispatch_count}\n- **Last-cycle summary**: {last_cycle_summary}\n"
    )
}

// `last_cycle.summary` is maintained by record-dispatch/process-merge in the
// standard "<dispatches>, <merges>" form; the post-dispatch delta only needs the
// first clause for display. If the summary is custom text, fall back to the full
// string rather than inventing a dispatch count.
fn dispatch_count_clause(summary: &str) -> &str {
    summary
        .split(',')
        .next()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(summary)
}

/// Find the most recent worklog file for `cycle` by scanning `docs/worklog/*/*.md`
/// and matching files whose first line starts with `# Cycle {cycle} — `.
fn find_worklog_for_cycle(repo_root: &Path, cycle: u64) -> Result<Option<PathBuf>, String> {
    let worklog_root = repo_root.join("docs").join("worklog");
    if !worklog_root.exists() {
        return Ok(None);
    }

    let mut candidates = Vec::new();
    for date_entry in fs::read_dir(&worklog_root)
        .map_err(|error| format!("failed to read {}: {}", worklog_root.display(), error))?
    {
        let date_entry = date_entry.map_err(|error| {
            format!(
                "failed to read entry in {}: {}",
                worklog_root.display(),
                error
            )
        })?;
        let date_path = date_entry.path();
        if !date_path.is_dir() {
            continue;
        }
        for file_entry in fs::read_dir(&date_path)
            .map_err(|error| format!("failed to read {}: {}", date_path.display(), error))?
        {
            let file_entry = file_entry.map_err(|error| {
                format!("failed to read entry in {}: {}", date_path.display(), error)
            })?;
            let path = file_entry.path();
            if path.extension() != Some(OsStr::new("md")) {
                continue;
            }
            let content = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
            if content
                .lines()
                .next()
                .is_some_and(|line| line.starts_with(&format!("# Cycle {} — ", cycle)))
            {
                let modified = fs::metadata(&path)
                    .map_err(|error| format!("failed to stat {}: {}", path.display(), error))?
                    .modified()
                    .map_err(|error| {
                        format!(
                            "failed to read modified time for {}: {}",
                            path.display(),
                            error
                        )
                    })?
                    .duration_since(UNIX_EPOCH)
                    .map_err(|error| {
                        format!(
                            "failed to normalize modified time for {}: {}",
                            path.display(),
                            error
                        )
                    })?;
                candidates.push((modified, path));
            }
        }
    }

    candidates.sort_by(|(left_modified, left_path), (right_modified, right_path)| {
        left_modified
            .cmp(right_modified)
            .then_with(|| left_path.cmp(right_path))
    });
    Ok(candidates.into_iter().last().map(|(_, path)| path))
}

fn commit_dispatch_artifacts(
    repo_root: &Path,
    message: &str,
    worklog_path: Option<&Path>,
) -> Result<String, String> {
    let mut add_command = Command::new("git");
    add_command
        .arg("-C")
        .arg(repo_root)
        .arg("add")
        .arg("docs/state.json");
    if let Some(worklog_path) = worklog_path {
        let relative_path = worklog_path.strip_prefix(repo_root).map_err(|error| {
            format!(
                "failed to compute relative worklog path for {}: {}",
                worklog_path.display(),
                error
            )
        })?;
        add_command.arg(relative_path);
    }
    let add_output = add_command
        .output()
        .map_err(|error| format!("failed to execute git add: {}", error))?;
    if !add_output.status.success() {
        let stderr = String::from_utf8_lossy(&add_output.stderr)
            .trim()
            .to_string();
        return Err(format!("git add dispatch artifacts failed: {}", stderr));
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
        return Err(format!("git commit failed: {}", stderr));
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["rev-parse", "--short=7", "HEAD"])
        .output()
        .map_err(|error| format!("failed to execute git rev-parse: {}", error))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("git rev-parse --short=7 HEAD failed: {}", stderr));
    }

    let sha = String::from_utf8(output.stdout)
        .map_err(|error| format!("failed to decode git rev-parse output as UTF-8: {}", error))?;
    Ok(sha.trim().to_string())
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
    let finding_disposition = entry.pointer(&finding_disposition_path).ok_or_else(|| {
        format!(
            "review history entry for cycle {} is missing finding_dispositions[{}] for finding {}",
            addressed_finding.cycle, finding_zero_based_index, addressed_finding.index
        )
    })?;
    let current_disposition = finding_disposition
        .get("disposition")
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| {
            format!(
                "review history entry for cycle {} finding {} is missing a string disposition",
                addressed_finding.cycle, addressed_finding.index
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
        time::{SystemTime, UNIX_EPOCH},
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
        repo.set_review_history_entry_with_dispositions(164, &["deferred", "deferred", "actioned"]);

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
        repo.set_review_history_entry_with_dispositions(164, &["deferred", "actioned", "deferred"]);
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
    fn run_commits_only_state_json() {
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
        .expect("dispatch should succeed");

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
        let status = Command::new("git")
            .arg("-C")
            .arg(repo.path())
            .args(["status", "--short"])
            .output()
            .expect("git status should execute");
        assert!(status.status.success());
        let status_stdout = String::from_utf8_lossy(&status.stdout);
        assert!(
            status_stdout.trim().is_empty(),
            "unexpected git status output: {status_stdout}"
        );

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
    fn run_preserves_close_out_snapshot_without_clobbering_dispatch_count() {
        let repo = TempRepo::new();
        repo.init_with_phase("close_out");
        let mut initial_state = repo.read_state();
        initial_state["last_cycle"]["summary"] = serde_json::json!(
            "0 dispatches, 2 merges (PR EvaLok/schema-org-json-ld#100, PR EvaLok/schema-org-json-ld#200)"
        );
        initial_state["last_cycle"]["timestamp"] = serde_json::json!("2026-04-09T09:52:44Z");
        repo.write_state_value(&initial_state);

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
            state.pointer("/last_cycle/summary"),
            Some(&serde_json::json!(
                "1 dispatch, 2 merges (PR EvaLok/schema-org-json-ld#100, PR EvaLok/schema-org-json-ld#200)"
            ))
        );
        let updated_timestamp = state
            .pointer("/last_cycle/timestamp")
            .and_then(serde_json::Value::as_str)
            .expect("dispatch should refresh last_cycle timestamp");
        assert_ne!(updated_timestamp, "2026-04-09T09:52:44Z");
        assert!(
            updated_timestamp > "2026-04-09T09:52:44Z",
            "timestamp should advance after re-sync"
        );
        assert!(
            updated_timestamp.contains('T') && updated_timestamp.ends_with('Z'),
            "refreshed timestamp should keep RFC 3339 UTC shape"
        );
    }

    #[test]
    fn sync_runs_when_session_prerecorded_and_phase_transitions() {
        // dispatch-review can pre-record a live session before record-dispatch runs.
        // Even when record-dispatch merges that existing session instead of appending
        // a new one, it must still re-sync last_cycle.summary after restoring the
        // sealed close_out snapshot.
        let repo = TempRepo::new();
        repo.init_with_phase("close_out");
        let mut initial_state = repo.read_state();
        initial_state["last_cycle"]["summary"] =
            serde_json::json!("0 dispatches, 1 merges (PR #700)");
        initial_state["last_cycle"]["timestamp"] = serde_json::json!("2026-03-07T12:00:00Z");
        // Pre-record a session for issue 602 with status "in_flight", simulating
        // what dispatch-review does when it creates the session before record-dispatch.
        initial_state["agent_sessions"]
            .as_array_mut()
            .expect("agent_sessions should be an array")
            .push(serde_json::json!({
                "issue": 602,
                "title": "Pre-recorded by dispatch-review",
                "dispatched_at": "2026-03-07T11:00:00Z",
                "model": "gpt-5.4",
                "status": "in_flight"
            }));
        repo.write_state_value(&initial_state);

        run(Cli {
            issue: 602,
            title: "Cycle review dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_dispatch: true,
            addresses_finding: None,
            repo_root: repo.path().to_path_buf(),
        })
        .expect("dispatch should succeed even when session was pre-recorded");

        let state = repo.read_state();
        assert_eq!(
            state.pointer("/last_cycle/summary"),
            Some(&serde_json::json!("1 dispatch, 1 merges (PR #700)"))
        );
        assert_eq!(
            state.pointer("/cycle_phase/phase"),
            Some(&serde_json::json!("complete"))
        );
    }

    #[test]
    fn sync_runs_when_session_prerecorded_without_phase_transition() {
        // The summary sync is safe to run unconditionally after restoring the sealed
        // snapshot, including when a live session was pre-recorded and the phase does
        // not change.
        let repo = TempRepo::new();
        repo.init_with_phase("work");
        let mut initial_state = repo.read_state();
        initial_state["last_cycle"]["summary"] =
            serde_json::json!("0 dispatches, 1 merges (PR #700)");
        initial_state["last_cycle"]["timestamp"] = serde_json::json!("2026-03-07T12:00:00Z");
        initial_state["agent_sessions"]
            .as_array_mut()
            .expect("agent_sessions should be an array")
            .push(serde_json::json!({
                "issue": 602,
                "title": "Pre-recorded session",
                "dispatched_at": "2026-03-07T11:00:00Z",
                "model": "gpt-5.4",
                "status": "in_flight"
            }));
        repo.write_state_value(&initial_state);
        let runner = MockRunner::with_exit_code(Some(0));

        run_with_runner(
            Cli {
                issue: 602,
                title: "Work phase re-dispatch".to_string(),
                model: Some("gpt-5.4".to_string()),
                review_dispatch: false,
                addresses_finding: None,
                repo_root: repo.path().to_path_buf(),
            },
            &runner,
            &mut |_| {},
        )
        .expect("work-phase re-dispatch should succeed");

        let state = repo.read_state();
        assert_eq!(
            state.pointer("/last_cycle/summary"),
            Some(&serde_json::json!("1 dispatch, 1 merges (PR #700)"))
        );
        assert_eq!(
            state.pointer("/cycle_phase/phase"),
            Some(&serde_json::json!("work"))
        );
    }

    #[test]
    fn sync_post_dispatch_worklog_falls_back_to_previous_cycle_worklog_when_current_cycle_missing() {
        let repo = TempRepo::new();
        repo.init_with_phase("work");
        let mut state = repo.read_state();
        state["last_cycle"] = serde_json::json!({
            "number": 513,
            "summary": "1 dispatch, 0 merges"
        });
        state["cycle_phase"]["cycle"] = serde_json::json!(514);
        state["in_flight_sessions"] = serde_json::json!(1);
        repo.write_state_value(&state);

        let worklog_path = repo
            .path()
            .join("docs/worklog/2026-04-18/094529-cycle-513-summary.md");
        fs::create_dir_all(worklog_path.parent().expect("worklog parent should exist")).unwrap();
        fs::write(
            &worklog_path,
            "# Cycle 513 — 2026-04-18 09:45 UTC\n\n## What was done\n\n- No new dispatches.\n",
        )
        .unwrap();

        let synced = sync_post_dispatch_worklog(repo.path(), &state, 514)
            .expect("post-dispatch sync should succeed")
            .expect("previous cycle worklog should be selected");

        assert_eq!(synced, worklog_path);
        let content = fs::read_to_string(&synced).unwrap();
        assert!(content.contains("## Post-dispatch delta"));
        assert!(content.contains("- **In-flight agent sessions**: 1"));
        assert!(content.contains("- **Dispatch count**: 1 dispatch"));
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
        assert_eq!(
            state.pointer("/in_flight_sessions"),
            Some(&serde_json::json!(1))
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
            state.pointer("/dispatch_log_latest"),
            Some(&serde_json::json!("#602 Example dispatch (cycle 164)"))
        );
        assert_eq!(
            state.pointer("/agent_sessions/2/issue"),
            Some(&serde_json::json!(602))
        );
    }

    #[test]
    fn review_dispatch_blocked_when_c5_5_gate_not_pass() {
        let repo = TempRepo::new();
        repo.init();
        repo.set_c5_5_gate("FAIL", false, 164);
        let before = repo.read_state();
        let mut warnings = Vec::new();
        let runner = MockRunner::with_error("runner should not be called for review dispatch");

        let error = run_with_runner(
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
        .expect_err("failing C5.5 gate should block review dispatch");

        assert_eq!(
            error,
            "Cannot dispatch review: C5.5 gate status is FAIL (cycle 164)"
        );
        assert!(warnings.is_empty());
        assert_eq!(runner.call_count(), 0);
        assert_eq!(repo.read_state(), before);
    }

    #[test]
    fn review_dispatch_blocked_when_c5_5_gate_needs_reverify() {
        let repo = TempRepo::new();
        repo.init();
        repo.set_c5_5_gate("PASS", true, 164);
        let before = repo.read_state();
        let mut warnings = Vec::new();
        let runner = MockRunner::with_error("runner should not be called for review dispatch");

        let error = run_with_runner(
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
        .expect_err("needs_reverify should block review dispatch");

        assert_eq!(
            error,
            "Cannot dispatch review: C5.5 gate needs re-verification"
        );
        assert!(warnings.is_empty());
        assert_eq!(runner.call_count(), 0);
        assert_eq!(repo.read_state(), before);
    }

    #[test]
    fn run_fails_when_c5_5_gate_is_from_previous_cycle() {
        let repo = TempRepo::new();
        repo.init();
        let mut state = repo.read_state();
        state["last_cycle"]["number"] = serde_json::json!(470);
        state["cycle_phase"]["cycle"] = serde_json::json!(470);
        repo.write_state_value(&state);
        repo.set_c5_5_gate("PASS", false, 469);
        let before = repo.read_state();
        let mut warnings = Vec::new();
        let runner = MockRunner::with_error("runner should not be called for review dispatch");

        let error = run_with_runner(
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
        .expect_err("stale C5.5 gate should block review dispatch");

        assert_eq!(
            error,
            "Cannot dispatch review: C5.5 gate is stale (gate cycle 469, current cycle 470)"
        );
        assert!(warnings.is_empty());
        assert_eq!(runner.call_count(), 0);
        assert_eq!(repo.read_state(), before);
    }

    #[test]
    fn run_succeeds_when_c5_5_gate_matches_current_cycle() {
        let repo = TempRepo::new();
        repo.init();
        repo.set_c5_5_gate("PASS", false, 164);
        let mut warnings = Vec::new();
        let runner = MockRunner::with_error("runner should not be called for review dispatch");

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
        .expect("passing C5.5 gate should allow review dispatch");

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
            state.pointer("/dispatch_log_latest"),
            Some(&serde_json::json!("#602 Example dispatch (cycle 164)"))
        );
        assert_eq!(
            state.pointer("/review_dispatch_consecutive"),
            Some(&serde_json::json!(1))
        );
    }

    #[test]
    fn review_dispatch_blocked_when_c5_5_gate_missing() {
        let repo = TempRepo::new();
        repo.init();
        repo.remove_c5_5_gate();
        let before = repo.read_state();
        let mut warnings = Vec::new();
        let runner = MockRunner::with_error("runner should not be called for review dispatch");

        let error = run_with_runner(
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
        .expect_err("missing C5.5 gate should block review dispatch");

        assert_eq!(
            error,
            "Cannot dispatch review: no C5.5 gate result found in state.json"
        );
        assert!(warnings.is_empty());
        assert_eq!(runner.call_count(), 0);
        assert_eq!(repo.read_state(), before);
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
    fn review_dispatch_fails_closed_when_state_json_is_invalid() {
        let repo = TempRepo::new();
        repo.init();
        fs::write(repo.path().join("docs/state.json"), "{invalid json")
            .expect("invalid state should be written");
        let mut warnings = Vec::new();
        let runner = MockRunner::with_error("runner should not be called for review dispatch");

        let error = run_with_runner(
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
        .expect_err("invalid state.json should fail closed");

        assert!(
            error.contains("Cannot dispatch review: failed to parse"),
            "unexpected error: {error}"
        );
        assert!(warnings.is_empty());
        assert_eq!(runner.call_count(), 0);
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
                },
                "review_agent": {
                    "history": []
                },
                "tool_pipeline": {
                    "c5_5_gate": {
                        "cycle": 164,
                        "status": "PASS",
                        "needs_reverify": false
                    }
                }
            });
            self.write_state_value(&state);
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
            self.write_state_value(&state);
        }

        fn set_c5_5_gate(&self, status: &str, needs_reverify: bool, cycle: u64) {
            let mut state = self.read_state();
            state["tool_pipeline"]["c5_5_gate"] = serde_json::json!({
                "cycle": cycle,
                "status": status,
                "needs_reverify": needs_reverify
            });
            self.write_state_value(&state);
        }

        fn remove_c5_5_gate(&self) {
            let mut state = self.read_state();
            if let Some(tool_pipeline) = state
                .get_mut("tool_pipeline")
                .and_then(serde_json::Value::as_object_mut)
            {
                tool_pipeline.remove("c5_5_gate");
            }
            self.write_state_value(&state);
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
            self.write_state_value(&state);
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
            self.write_state_value(&state);
        }

        fn write_state_value(&self, state: &serde_json::Value) {
            fs::write(
                self.path().join("docs/state.json"),
                serde_json::to_string_pretty(state).expect("state should serialize"),
            )
            .expect("state file should be updated");
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
