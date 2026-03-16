use clap::Parser;
use record_dispatch::{
    apply_dispatch_patch, build_dispatch_patch, dispatch_commit_message, resolve_model,
};
use state_schema::{
    commit_state_json, current_cycle_from_state, current_utc_timestamp, read_state_value,
    transition_cycle_phase, write_state_value, ReviewHistoryEntry,
};
use std::{collections::BTreeSet, path::PathBuf};

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

    /// Review finding number(s) to reclassify from deferred to dispatch_created
    #[arg(long = "review-finding", value_name = "NUMBER")]
    review_findings: Vec<u64>,

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
    let model = resolve_model(cli.model.as_deref(), &cli.repo_root)?;
    let mut state_value = read_state_value(&cli.repo_root)?;
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
    reconcile_review_findings(&mut state_value, current_cycle, &cli.review_findings)?;
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
        eprintln!(
            "Warning: in-flight dispatches at {} (approaching/exceeding concurrency limit of 2)",
            patch.in_flight
        );
    }

    Ok(())
}

fn reconcile_review_findings(
    state_value: &mut serde_json::Value,
    current_cycle: u64,
    review_findings: &[u64],
) -> Result<(), String> {
    if review_findings.is_empty() {
        return Ok(());
    }

    let mut unique_findings = BTreeSet::new();
    let mut duplicate_findings = BTreeSet::new();
    for finding in review_findings {
        if !unique_findings.insert(*finding) {
            duplicate_findings.insert(*finding);
        }
    }
    if !duplicate_findings.is_empty() {
        let duplicates = duplicate_findings
            .iter()
            .map(u64::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        return Err(format!(
            "duplicate --review-finding value(s) are not allowed: {}",
            duplicates
        ));
    }

    let history = state_value
        .pointer_mut("/review_agent/history")
        .and_then(serde_json::Value::as_array_mut)
        .ok_or_else(|| "missing array: review_agent.history".to_string())?;
    let latest_entry_value = history
        .last_mut()
        .ok_or_else(|| "review_agent.history is empty".to_string())?;
    let mut latest_entry: ReviewHistoryEntry =
        serde_json::from_value(latest_entry_value.clone()).map_err(|error| {
            format!("failed to parse latest review_agent.history entry: {}", error)
        })?;

    if latest_entry.cycle != current_cycle {
        return Err(format!(
            "latest review_agent.history entry is for cycle {}, expected current cycle {}",
            latest_entry.cycle, current_cycle
        ));
    }

    for finding in &unique_findings {
        if *finding == 0 || *finding > latest_entry.finding_count {
            return Err(format!(
                "review finding {} is out of range for cycle {} (expected 1..={})",
                finding, current_cycle, latest_entry.finding_count
            ));
        }
    }

    let reclassified_count = u64::try_from(unique_findings.len())
        .map_err(|_| "review finding count too large to store as u64".to_string())?;
    if latest_entry.deferred < reclassified_count {
        return Err(format!(
            "cannot reclassify {} review finding(s): deferred count {} would go below 0",
            reclassified_count, latest_entry.deferred
        ));
    }

    latest_entry.deferred -= reclassified_count;
    latest_entry.dispatch_created = latest_entry
        .dispatch_created
        .checked_add(reclassified_count)
        .ok_or_else(|| "dispatch_created overflowed u64".to_string())?;

    let disposition_sum = review_disposition_sum(&latest_entry)?;
    if disposition_sum != latest_entry.finding_count {
        // The issue contract requires warning instead of failing so dispatch
        // recording can proceed even if earlier review bookkeeping was inconsistent.
        eprintln!(
            "Warning: latest review history entry dispositions sum to {} but finding_count is {}; continuing",
            disposition_sum, latest_entry.finding_count
        );
    }

    *latest_entry_value = serde_json::to_value(&latest_entry)
        .map_err(|error| format!("failed to serialize latest review history entry: {}", error))?;
    Ok(())
}

fn review_disposition_sum(entry: &ReviewHistoryEntry) -> Result<u64, String> {
    entry
        .actioned
        .checked_add(entry.deferred)
        .and_then(|value| value.checked_add(entry.dispatch_created))
        .and_then(|value| value.checked_add(entry.actioned_failed))
        .and_then(|value| value.checked_add(entry.verified_resolved))
        .and_then(|value| value.checked_add(entry.ignored))
        .ok_or_else(|| "review history disposition counts overflowed u64".to_string())
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
        assert!(help.contains("--review-finding"));
        assert!(help.contains("--repo-root"));
    }

    #[test]
    fn cli_parses_multiple_review_finding_flags() {
        let cli = Cli::parse_from([
            "record-dispatch",
            "--issue",
            "602",
            "--title",
            "Example dispatch",
            "--review-finding",
            "1",
            "--review-finding",
            "3",
        ]);

        assert_eq!(cli.review_findings, vec![1, 3]);
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
            review_findings: Vec::new(),
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
            review_findings: Vec::new(),
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
            review_findings: Vec::new(),
            repo_root: repo.path().to_path_buf(),
        })
        .expect("dispatch should succeed");

        let state = repo.read_state();
        assert_eq!(state.pointer("/cycle_phase/phase"), Some(&serde_json::json!("work")));
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
            review_findings: Vec::new(),
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
    fn run_reclassifies_single_review_finding() {
        let repo = TempRepo::new();
        repo.init_with_review_history("close_out", vec![sample_review_history_entry(164, 0, 2, 0, 0, 0, 0, 2)]);

        run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_findings: vec![1],
            repo_root: repo.path().to_path_buf(),
        })
        .expect("dispatch should reconcile review dispositions");

        let state = repo.read_state();
        assert_eq!(
            state.pointer("/review_agent/history/0/deferred"),
            Some(&serde_json::json!(1))
        );
        assert_eq!(
            state.pointer("/review_agent/history/0/dispatch_created"),
            Some(&serde_json::json!(1))
        );
    }

    #[test]
    fn run_reclassifies_multiple_review_findings() {
        let repo = TempRepo::new();
        repo.init_with_review_history("close_out", vec![sample_review_history_entry(164, 1, 2, 0, 0, 0, 0, 3)]);

        run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_findings: vec![1, 3],
            repo_root: repo.path().to_path_buf(),
        })
        .expect("dispatch should reconcile multiple findings");

        let state = repo.read_state();
        assert_eq!(
            state.pointer("/review_agent/history/0/deferred"),
            Some(&serde_json::json!(0))
        );
        assert_eq!(
            state.pointer("/review_agent/history/0/dispatch_created"),
            Some(&serde_json::json!(2))
        );
    }

    #[test]
    fn run_rejects_out_of_range_review_finding() {
        let repo = TempRepo::new();
        repo.init_with_review_history("close_out", vec![sample_review_history_entry(164, 0, 2, 0, 0, 0, 0, 2)]);

        let error = run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_findings: vec![3],
            repo_root: repo.path().to_path_buf(),
        })
        .expect_err("out-of-range finding should fail");

        assert!(error.contains("out of range"));
    }

    #[test]
    fn run_rejects_review_finding_when_deferred_would_underflow() {
        let repo = TempRepo::new();
        repo.init_with_review_history("close_out", vec![sample_review_history_entry(164, 2, 0, 0, 0, 0, 0, 2)]);

        let error = run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_findings: vec![1],
            repo_root: repo.path().to_path_buf(),
        })
        .expect_err("underflow should fail");

        assert!(error.contains("would go below 0"));
    }

    #[test]
    fn run_leaves_review_history_unchanged_when_flag_is_absent() {
        let repo = TempRepo::new();
        repo.init_with_review_history("close_out", vec![sample_review_history_entry(164, 0, 2, 0, 0, 0, 0, 2)]);

        run(Cli {
            issue: 602,
            title: "Example dispatch".to_string(),
            model: Some("gpt-5.4".to_string()),
            review_findings: Vec::new(),
            repo_root: repo.path().to_path_buf(),
        })
        .expect("dispatch should succeed without review findings");

        let state = repo.read_state();
        assert_eq!(
            state.pointer("/review_agent/history/0/deferred"),
            Some(&serde_json::json!(2))
        );
        assert_eq!(
            state.pointer("/review_agent/history/0/dispatch_created"),
            Some(&serde_json::json!(0))
        );
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

        fn init_with_review_history(&self, phase: &str, history: Vec<serde_json::Value>) {
            self.write_state_with_review_history(phase, history);
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

        fn write_state_with_review_history(&self, phase: &str, history: Vec<serde_json::Value>) {
            let mut state = self.base_state(phase);
            state["review_agent"] = serde_json::json!({
                "history": history
            });
            fs::write(
                self.path().join("docs/state.json"),
                serde_json::to_string_pretty(&state).expect("state file should serialize"),
            )
            .expect("state file should be written");
        }

        fn base_state(&self, phase: &str) -> serde_json::Value {
            serde_json::json!({
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
            })
        }

        fn read_state(&self) -> serde_json::Value {
            serde_json::from_str(
                &fs::read_to_string(self.path().join("docs/state.json"))
                    .expect("state file should be readable"),
            )
            .expect("state file should parse")
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

    fn sample_review_history_entry(
        cycle: u64,
        actioned: u64,
        deferred: u64,
        dispatch_created: u64,
        actioned_failed: u64,
        verified_resolved: u64,
        ignored: u64,
        finding_count: u64,
    ) -> serde_json::Value {
        serde_json::json!({
            "cycle": cycle,
            "categories": ["dispatch-reconciliation"],
            "actioned": actioned,
            "deferred": deferred,
            "dispatch_created": dispatch_created,
            "actioned_failed": actioned_failed,
            "verified_resolved": verified_resolved,
            "ignored": ignored,
            "finding_count": finding_count,
            "complacency_score": 4
        })
    }
}
