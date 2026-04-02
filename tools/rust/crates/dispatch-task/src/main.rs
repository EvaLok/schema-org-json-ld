use clap::Parser;
use record_dispatch::{
    apply_dispatch_patch, build_dispatch_patch, dispatch_commit_message, enforce_pipeline_gate,
    fixup_latest_worklog_in_flight, resolve_model, update_review_dispatch_tracking, CommandRunner,
    PipelineGateError, ProcessRunner, WorklogFixupOutcome, PIPELINE_GATE_FAILURE_MESSAGE,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use state_schema::{
    commit_state_json, current_cycle_from_state, current_utc_timestamp, read_state_value,
    write_state_value,
};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const BASE_BRANCH: &str = "master";
const DEFAULT_MODEL: &str = "gpt-5.4";
const DEFAULT_LABEL: &str = "agent-task";

#[derive(Parser, Debug)]
#[command(name = "dispatch-task")]
struct Cli {
    /// Issue title
    #[arg(long)]
    title: String,

    /// Path to a file containing the issue body
    #[arg(long)]
    body_file: PathBuf,

    /// Model for agent_assignment
    #[arg(long, default_value = DEFAULT_MODEL)]
    model: String,

    /// Label to apply (repeatable)
    #[arg(long = "label")]
    labels: Vec<String>,

    /// Review finding this dispatch addresses, formatted as CYCLE:INDEX
    #[arg(long)]
    addresses_finding: Option<AddressedFinding>,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Print what would be created without dispatching
    #[arg(long)]
    dry_run: bool,
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

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct AgentAssignment {
    target_repo: String,
    base_branch: String,
    model: String,
    custom_instructions: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct TaskIssuePayload {
    title: String,
    body: String,
    labels: Vec<String>,
    assignees: Vec<String>,
    agent_assignment: AgentAssignment,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct CreatedIssue {
    number: u64,
}

struct DispatchRecordRequest<'a> {
    cycle: u64,
    issue: u64,
    title: &'a str,
    model: &'a str,
    addressed_finding: Option<&'a AddressedFinding>,
}

trait GitHubIssueCreator {
    fn create_issue(&self, payload: &TaskIssuePayload) -> Result<CreatedIssue, String>;
}

struct ProcessIssueCreator;

impl GitHubIssueCreator for ProcessIssueCreator {
    fn create_issue(&self, payload: &TaskIssuePayload) -> Result<CreatedIssue, String> {
        create_issue(payload)
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
    let issue_creator = ProcessIssueCreator;
    let runner = ProcessRunner;
    run_with_dependencies(cli, &runner, &issue_creator, &mut |warning| {
        eprintln!("Warning: {warning}");
    })
}

fn run_with_dependencies(
    cli: Cli,
    runner: &dyn CommandRunner,
    issue_creator: &dyn GitHubIssueCreator,
    warn: &mut dyn FnMut(&str),
) -> Result<(), String> {
    let body = read_body_file(&cli.body_file)?;
    let model = resolve_model(Some(cli.model.as_str()), &cli.repo_root)?;
    let labels = resolve_labels(&cli.labels)?;
    let current_cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /cycle_phase/cycle or /last_cycle/number in state.json" {
            "missing numeric /cycle_phase/cycle or /last_cycle/number in docs/state.json"
                .to_string()
        } else {
            error
        }
    })?;
    let payload = build_issue_payload(&cli.title, &body, &model, &labels);

    if cli.dry_run {
        println!("{}", build_dry_run_preview(&payload, current_cycle)?);
        return Ok(());
    }

    let created_issue = issue_creator.create_issue(&payload)?;
    let record_request = DispatchRecordRequest {
        cycle: current_cycle,
        issue: created_issue.number,
        title: &payload.title,
        model: &model,
        addressed_finding: cli.addresses_finding.as_ref(),
    };
    let receipt = match record_created_issue(&cli.repo_root, &record_request, runner, warn) {
        Ok(receipt) => receipt,
        Err(error) => {
            warn(&format!(
                "Issue #{} was created but dispatch bookkeeping failed. Fallback: record-dispatch --issue {} --title {:?}",
                created_issue.number, created_issue.number, payload.title
            ));
            return Err(format!(
                "created issue #{} but failed to record dispatch in docs/state.json: {}",
                created_issue.number, error
            ));
        }
    };
    push_origin_master(&cli.repo_root)?;

    println!("Created issue #{}: {}", created_issue.number, payload.title);
    println!("Dispatch recorded in state.json (receipt: {})", receipt);

    Ok(())
}

fn build_issue_payload(title: &str, body: &str, model: &str, labels: &[String]) -> TaskIssuePayload {
    TaskIssuePayload {
        title: title.to_string(),
        body: body.to_string(),
        labels: labels.to_vec(),
        assignees: vec!["copilot-swe-agent[bot]".to_string()],
        agent_assignment: AgentAssignment {
            target_repo: MAIN_REPO.to_string(),
            base_branch: BASE_BRANCH.to_string(),
            model: model.to_string(),
            custom_instructions: String::new(),
        },
    }
}

fn build_dry_run_preview(payload: &TaskIssuePayload, cycle: u64) -> Result<String, String> {
    let json = serde_json::to_string_pretty(payload)
        .map_err(|error| format!("failed to serialize dry-run payload: {}", error))?;
    Ok(format!(
        "{json}\nWould record dispatch in docs/state.json for cycle {cycle} after issue creation."
    ))
}

fn read_body_file(path: &Path) -> Result<String, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    Ok(content.trim_end_matches(['\r', '\n']).to_string())
}

fn resolve_labels(cli_labels: &[String]) -> Result<Vec<String>, String> {
    if cli_labels.is_empty() {
        return Ok(vec![DEFAULT_LABEL.to_string()]);
    }

    cli_labels
        .iter()
        .map(|label| {
            let trimmed = label.trim();
            if trimmed.is_empty() {
                Err("--label must not be empty".to_string())
            } else {
                Ok(trimmed.to_string())
            }
        })
        .collect()
}

fn record_created_issue(
    repo_root: &Path,
    request: &DispatchRecordRequest<'_>,
    runner: &dyn CommandRunner,
    warn: &mut dyn FnMut(&str),
) -> Result<String, String> {
    let pipeline_warning = match enforce_pipeline_gate(repo_root, false, runner) {
        Ok(warning) => warning,
        Err(PipelineGateError::ExecutionFailed(detail)) => {
            eprintln!("pipeline-check execution error: {detail}");
            return Err(PIPELINE_GATE_FAILURE_MESSAGE.to_string());
        }
        Err(PipelineGateError::Failed) => {
            return Err(PIPELINE_GATE_FAILURE_MESSAGE.to_string());
        }
    };
    if let Some(warning) = pipeline_warning {
        warn(warning);
    }

    let mut state_value = read_state_value(repo_root)?;
    if let Some(warning) = update_review_dispatch_tracking(&mut state_value, false)? {
        warn(&warning);
    }

    if state_value
        .pointer("/cycle_phase/phase")
        .and_then(Value::as_str)
        == Some("close_out")
    {
        warn("cycle phase is close_out; dispatch-task does not transition cycle_phase");
    }

    let dispatched_at = current_utc_timestamp();
    let patch = build_dispatch_patch(
        &state_value,
        request.cycle,
        request.issue,
        request.title,
        request.model,
        &dispatched_at,
    )?;
    apply_dispatch_patch(&mut state_value, &patch)?;
    if let Some(addressed_finding) = request.addressed_finding {
        reconcile_review_history_dispatch(&mut state_value, addressed_finding, warn)?;
    }
    write_state_value(repo_root, &state_value)?;

    let commit_message = dispatch_commit_message(request.issue, patch.current_cycle);
    let receipt = commit_state_json(repo_root, &commit_message)?;
    match fixup_latest_worklog_in_flight(repo_root, patch.in_flight)? {
        WorklogFixupOutcome::Updated(path) => {
            println!("Worklog in-flight count updated in {}", path.display());
        }
        WorklogFixupOutcome::NotFound => {
            warn("Latest worklog not found; skipping in-flight count fixup");
        }
    }

    Ok(receipt)
}

fn reconcile_review_history_dispatch(
    state: &mut Value,
    addressed_finding: &AddressedFinding,
    warn: &mut dyn FnMut(&str),
) -> Result<(), String> {
    let finding_zero_based_index = (addressed_finding.index - 1) as usize;
    let history = state
        .pointer_mut("/review_agent/history")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| "missing array /review_agent/history in docs/state.json".to_string())?;

    let entry = history
        .iter_mut()
        .find(|entry| {
            entry.get("cycle").and_then(Value::as_u64) == Some(addressed_finding.cycle)
        })
        .ok_or_else(|| {
            format!(
                "review history entry for cycle {} was not found in docs/state.json",
                addressed_finding.cycle
            )
        })?;

    let finding_count = entry
        .get("finding_count")
        .and_then(Value::as_u64)
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
        .and_then(Value::as_str)
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
        .and_then(Value::as_u64)
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
        .and_then(Value::as_u64)
        .unwrap_or(0);

    let entry_object = entry.as_object_mut().ok_or_else(|| {
        format!(
            "review history entry for cycle {} must be an object",
            addressed_finding.cycle
        )
    })?;
    let finding_dispositions = entry_object
        .get_mut("finding_dispositions")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| {
            format!(
                "review history entry for cycle {} is missing an array finding_dispositions",
                addressed_finding.cycle
            )
        })?;
    let finding_disposition = finding_dispositions
        .get_mut(finding_zero_based_index)
        .and_then(Value::as_object_mut)
        .ok_or_else(|| {
            format!(
                "review history entry for cycle {} finding {} (finding_dispositions[{}]) must be an object",
                addressed_finding.cycle,
                addressed_finding.index,
                finding_zero_based_index
            )
        })?;
    finding_disposition.insert("disposition".to_string(), serde_json::json!("dispatch_created"));
    entry_object.insert("deferred".to_string(), serde_json::json!(deferred - 1));
    entry_object.insert(
        "dispatch_created".to_string(),
        serde_json::json!(dispatch_created + 1),
    );

    Ok(())
}

fn create_issue(payload: &TaskIssuePayload) -> Result<CreatedIssue, String> {
    let body = serde_json::to_vec(payload)
        .map_err(|error| format!("failed to serialize issue payload: {}", error))?;
    let mut child = Command::new("gh")
        .args([
            "api",
            "/repos/EvaLok/schema-org-json-ld/issues",
            "--method",
            "POST",
            "--input",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("failed to execute gh api: {}", error))?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| "failed to open stdin for gh api".to_string())?;
        stdin
            .write_all(&body)
            .map_err(|error| format!("failed to write gh api payload: {}", error))?;
    }

    let output = child
        .wait_with_output()
        .map_err(|error| format!("failed to wait for gh api: {}", error))?;
    if !output.status.success() {
        return Err(command_failure_message("gh api", &output));
    }

    serde_json::from_slice::<CreatedIssue>(&output.stdout)
        .map_err(|error| format!("failed to parse gh api response as issue JSON: {}", error))
}

fn push_origin_master(repo_root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .args(["push", "origin", "master"])
        .current_dir(repo_root)
        .output()
        .map_err(|error| format!("failed to execute git push: {}", error))?;
    if !output.status.success() {
        return Err(command_failure_message("git push origin master", &output));
    }

    Ok(())
}

fn command_failure_message(command: &str, output: &Output) -> String {
    let code = output.status.code().map_or_else(
        || "terminated by signal".to_string(),
        |value| value.to_string(),
    );
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();

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
    use std::cell::Cell;
    use std::{
        fs,
        path::Path,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--title"));
        assert!(help.contains("--body-file"));
        assert!(help.contains("--model"));
        assert!(help.contains("--label"));
        assert!(help.contains("--addresses-finding"));
        assert!(help.contains("--repo-root"));
        assert!(help.contains("--dry-run"));
    }

    #[test]
    fn build_issue_payload_uses_cli_values() {
        let payload = build_issue_payload(
            "Example title",
            "Task body",
            "gpt-5.4",
            &["agent-task".to_string(), "backend".to_string()],
        );

        assert_eq!(payload.title, "Example title");
        assert_eq!(payload.body, "Task body");
        assert_eq!(payload.labels, vec!["agent-task", "backend"]);
        assert_eq!(payload.assignees, vec!["copilot-swe-agent[bot]"]);
        assert_eq!(
            payload.agent_assignment,
            AgentAssignment {
                target_repo: MAIN_REPO.to_string(),
                base_branch: BASE_BRANCH.to_string(),
                model: "gpt-5.4".to_string(),
                custom_instructions: String::new(),
            }
        );
    }

    #[test]
    fn cli_defaults_model_to_gpt_5_4() {
        let cli = Cli::parse_from(["dispatch-task", "--title", "Example", "--body-file", "/dev/null"]);

        assert_eq!(cli.model, "gpt-5.4");
    }

    #[test]
    fn default_label_is_agent_task() {
        let cli = Cli::parse_from(["dispatch-task", "--title", "Example", "--body-file", "/dev/null"]);

        assert_eq!(
            resolve_labels(&cli.labels).expect("default labels should resolve"),
            vec!["agent-task".to_string()]
        );
    }

    #[test]
    fn repeated_label_flags_are_preserved() {
        let cli = Cli::parse_from([
            "dispatch-task",
            "--title",
            "Example",
            "--body-file",
            "/dev/null",
            "--label",
            "agent-task",
            "--label",
            "backend",
            "--label",
            "priority-high",
        ]);

        assert_eq!(
            resolve_labels(&cli.labels).expect("labels should resolve"),
            vec![
                "agent-task".to_string(),
                "backend".to_string(),
                "priority-high".to_string()
            ]
        );
    }

    #[test]
    fn dry_run_does_not_call_gh_api_or_modify_state_json() {
        let repo = TempRepo::new();
        repo.write_state("work");
        let body_file = repo.write_body_file("Dispatch body");
        let state_before = repo.read_state();
        let runner = MockRunner::with_error("pipeline runner should not be called");
        let issue_creator = MockIssueCreator::new(Ok(CreatedIssue { number: 9001 }));
        let mut warnings = Vec::new();

        run_with_dependencies(
            Cli {
                title: "Example dispatch".to_string(),
                body_file,
                model: DEFAULT_MODEL.to_string(),
                labels: Vec::new(),
                addresses_finding: None,
                repo_root: repo.path().to_path_buf(),
                dry_run: true,
            },
            &runner,
            &issue_creator,
            &mut |warning| warnings.push(warning.to_string()),
        )
        .expect("dry-run should succeed");

        assert_eq!(runner.call_count(), 0);
        assert_eq!(issue_creator.call_count(), 0);
        assert!(warnings.is_empty());
        assert_eq!(repo.read_state(), state_before);
    }

    #[test]
    fn record_created_issue_keeps_close_out_phase_unchanged() {
        let repo = TempRepo::new();
        repo.init("close_out");
        let runner = MockRunner::with_exit_code(Some(0));
        let mut warnings = Vec::new();

        record_created_issue(
            repo.path(),
            &DispatchRecordRequest {
                cycle: 164,
                issue: 602,
                title: "Example dispatch",
                model: "gpt-5.4",
                addressed_finding: None,
            },
            &runner,
            &mut |warning| warnings.push(warning.to_string()),
        )
        .expect("recording should succeed");

        let state = repo.read_state();
        assert_eq!(state.pointer("/cycle_phase/phase"), Some(&json!("close_out")));
        assert_eq!(runner.call_count(), 1);
        assert!(warnings.iter().any(|warning| {
            warning == "cycle phase is close_out; dispatch-task does not transition cycle_phase"
        }));
    }

    struct MockRunner {
        result: Result<record_dispatch::ExecutionResult, String>,
        call_count: Cell<usize>,
    }

    impl MockRunner {
        fn with_exit_code(exit_code: Option<i32>) -> Self {
            Self {
                result: Ok(record_dispatch::ExecutionResult { exit_code }),
                call_count: Cell::new(0),
            }
        }

        fn with_error(message: &str) -> Self {
            Self {
                result: Err(message.to_string()),
                call_count: Cell::new(0),
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

    struct MockIssueCreator {
        result: Result<CreatedIssue, String>,
        call_count: Cell<usize>,
    }

    impl MockIssueCreator {
        fn new(result: Result<CreatedIssue, String>) -> Self {
            Self {
                result,
                call_count: Cell::new(0),
            }
        }

        fn call_count(&self) -> usize {
            self.call_count.get()
        }
    }

    impl GitHubIssueCreator for MockIssueCreator {
        fn create_issue(&self, _payload: &TaskIssuePayload) -> Result<CreatedIssue, String> {
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
                "dispatch-task-test-{}-{}",
                std::process::id(),
                unique
            ));
            fs::create_dir_all(path.join("docs")).expect("temp repo should be created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn init(&self, phase: &str) {
            self.write_state(phase);
            git_success(self.path(), ["init"]);
            git_success(self.path(), ["config", "user.name", "Dispatch Task Tests"]);
            git_success(
                self.path(),
                ["config", "user.email", "dispatch-task-tests@example.com"],
            );
            git_success(self.path(), ["add", "docs/state.json"]);
            git_success(self.path(), ["commit", "-m", "initial state"]);
        }

        fn write_state(&self, phase: &str) {
            let state = json!({
                "agent_sessions": [
                    {
                        "issue": 600,
                        "title": "Merged change",
                        "dispatched_at": "2026-03-01T00:00:00Z",
                        "model": "gpt-5.4",
                        "status": "merged",
                        "pr": 700,
                        "merged_at": "2026-03-02T00:00:00Z"
                    }
                ],
                "in_flight_sessions": 0,
                "dispatch_log_latest": "#600 Merged change (cycle 164)",
                "last_cycle": {
                    "number": 164
                },
                "cycle_phase": {
                    "cycle": 164,
                    "phase": phase,
                    "phase_entered_at": "2026-03-07T12:00:00Z"
                },
                "copilot_metrics": {
                    "total_dispatches": 1,
                    "resolved": 1,
                    "merged": 1,
                    "closed_without_pr": 0,
                    "reviewed_awaiting_eva": 0,
                    "in_flight": 0,
                    "produced_pr": 1,
                    "pr_merge_rate": "100.0%",
                    "dispatch_to_pr_rate": "100.0%",
                    "dispatch_log_latest": "#600 Merged change (cycle 164)"
                },
                "field_inventory": {
                    "fields": {
                        "in_flight_sessions": { "last_refreshed": "cycle 163" },
                        "copilot_metrics.in_flight": { "last_refreshed": "cycle 163" },
                        "copilot_metrics.pr_merge_rate": { "last_refreshed": "cycle 163" },
                        "copilot_metrics.dispatch_to_pr_rate": { "last_refreshed": "cycle 163" },
                        "cycle_phase": { "last_refreshed": "cycle 163" }
                    }
                },
                "review_agent": {
                    "history": []
                }
            });
            fs::write(
                self.path().join("docs/state.json"),
                serde_json::to_string_pretty(&state).expect("state should serialize"),
            )
            .expect("state should be written");
        }

        fn write_body_file(&self, content: &str) -> PathBuf {
            let path = self.path().join("body.md");
            fs::write(&path, content).expect("body file should be written");
            path
        }

        fn read_state(&self) -> Value {
            serde_json::from_str(
                &fs::read_to_string(self.path().join("docs/state.json"))
                    .expect("state file should be readable"),
            )
            .expect("state file should parse")
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn git_success<const N: usize>(repo: &Path, args: [&str; N]) {
        let output = Command::new("git")
            .args(args)
            .current_dir(repo)
            .output()
            .expect("git command should run");
        if !output.status.success() {
            panic!(
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
}
