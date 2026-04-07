use clap::{ArgAction, Parser};
use record_dispatch::{
    apply_dispatch_patch, build_dispatch_patch, concurrency_warning_message,
    dispatch_commit_message, enforce_pipeline_gate, fixup_latest_worklog_in_flight, resolve_model,
    CommandRunner, PipelineGateError, ProcessRunner, WorklogFixupOutcome,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use state_schema::{
    commit_state_json, current_cycle_from_state, current_utc_timestamp, read_state_value,
    write_state_value,
};
use std::collections::BTreeSet;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const BASE_BRANCH: &str = "master";

#[derive(Parser, Debug)]
#[command(name = "dispatch-task")]
struct Cli {
    /// Issue title
    #[arg(long)]
    title: String,

    /// Path to file containing issue body markdown
    #[arg(long)]
    body_file: PathBuf,

    /// Model for agent_assignment
    #[arg(long)]
    model: Option<String>,

    /// Labels to apply (repeatable, default: agent-task)
    #[arg(long = "label", default_value = "agent-task")]
    labels: Vec<String>,

    /// Review finding this dispatch addresses, in CYCLE:INDEX format
    #[arg(long, action = ArgAction::Append)]
    addresses_finding: Vec<AddressedFinding>,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Print what would be created without actually dispatching
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

impl AddressedFinding {
    fn finding_ref(&self) -> String {
        format!("{}:{}", self.cycle, self.index)
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
struct IssuePayload {
    title: String,
    body: String,
    labels: Vec<String>,
    assignees: Vec<String>,
    agent_assignment: AgentAssignment,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct CreatedIssue {
    number: u64,
    html_url: String,
}

/// Trait that abstracts the GitHub API call for testability.
trait GithubApiRunner {
    fn create_issue(&self, payload: &IssuePayload) -> Result<CreatedIssue, String>;
}

struct ProcessGithubApiRunner;

impl GithubApiRunner for ProcessGithubApiRunner {
    fn create_issue(&self, payload: &IssuePayload) -> Result<CreatedIssue, String> {
        let body = serde_json::to_vec(payload)
            .map_err(|error| format!("failed to serialize issue payload: {}", error))?;
        let mut child = Command::new("gh")
            .arg("api")
            .arg(format!("repos/{MAIN_REPO}/issues"))
            .arg("--method")
            .arg("POST")
            .arg("--input")
            .arg("-")
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
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let pipeline_runner = ProcessRunner;
    let api_runner = ProcessGithubApiRunner;
    run_with_runners(cli, &pipeline_runner, &api_runner, &mut |warning| {
        eprintln!("{warning}")
    })
}

fn run_with_runners(
    cli: Cli,
    pipeline_runner: &dyn CommandRunner,
    api_runner: &dyn GithubApiRunner,
    warn: &mut dyn FnMut(&str),
) -> Result<(), String> {
    let body = read_body_file(&cli.body_file, cli.dry_run)?;
    let model = resolve_model(cli.model.as_deref(), &cli.repo_root)?;
    let payload = build_issue_payload(&cli.title, &body, &cli.labels, &model);

    if cli.dry_run {
        println!(
            "{}",
            serde_json::to_string_pretty(&payload)
                .map_err(|error| format!("failed to serialize dry-run payload: {}", error))?
        );
        println!("[dry-run] Would dispatch issue and record in docs/state.json");
        return Ok(());
    }

    // Validate pipeline gate before creating the issue.
    let pipeline_warning = match enforce_pipeline_gate(&cli.repo_root, false, pipeline_runner) {
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

    // Create the GitHub issue first; only modify state on success.
    let created_issue = api_runner.create_issue(&payload)?;

    // Record the dispatch in state.json.
    match record_dispatch_state(
        &cli.repo_root,
        created_issue.number,
        &cli.title,
        &model,
        &cli.addresses_finding,
        warn,
    ) {
        Ok(receipt) => {
            println!("Created issue #{}: {}", created_issue.number, cli.title);
            println!("Dispatch recorded in state.json (receipt: {})", receipt);
        }
        Err(error) => {
            eprintln!(
                "Warning: created issue #{} but failed to record dispatch in state.json: {}",
                created_issue.number, error
            );
            eprintln!(
                "Run: tools/record-dispatch --issue {} --title {:?} to record manually",
                created_issue.number, cli.title
            );
            return Err(format!(
                "dispatch created issue #{} but state.json update failed",
                created_issue.number
            ));
        }
    }

    Ok(())
}

/// Build, apply, and commit the dispatch state update, then push.
/// Returns the short git SHA of the state commit (receipt).
fn record_dispatch_state(
    repo_root: &Path,
    issue: u64,
    title: &str,
    model: &str,
    addresses_finding: &[AddressedFinding],
    warn: &mut dyn FnMut(&str),
) -> Result<String, String> {
    let dispatched_at = current_utc_timestamp();
    let current_cycle = current_cycle_from_state(repo_root).map_err(|error| {
        if error == "missing /cycle_phase/cycle or /last_cycle/number in state.json" {
            "missing numeric /cycle_phase/cycle or /last_cycle/number in docs/state.json"
                .to_string()
        } else {
            error
        }
    })?;

    let mut state_value = read_state_value(repo_root)?;
    let patch = build_dispatch_patch(
        &state_value,
        current_cycle,
        issue,
        title,
        model,
        &dispatched_at,
    )?;
    apply_dispatch_patch(&mut state_value, &patch)?;

    let addresses_finding = dedupe_addressed_findings(addresses_finding);
    set_session_addresses_findings(&mut state_value, issue, &addresses_finding)?;
    reconcile_review_history_dispatches(&mut state_value, &addresses_finding, warn)?;

    // NOTE: dispatch-task does NOT perform the close_out → complete phase
    // transition.  Phase transitions are exclusively the responsibility of the
    // record-dispatch binary, which runs during cycle close-out.  dispatch-task
    // runs during the work phase and must not advance the cycle state machine.

    write_state_value(repo_root, &state_value)?;

    let commit_message = dispatch_commit_message(issue, patch.current_cycle);
    let receipt = commit_state_json(repo_root, &commit_message)?;

    match fixup_latest_worklog_in_flight(repo_root, patch.in_flight)? {
        WorklogFixupOutcome::Updated(path) => {
            println!("Worklog in-flight count updated in {}", path.display());
        }
        WorklogFixupOutcome::NotFound => {
            warn("Latest worklog not found; skipping in-flight count fixup");
        }
    }

    if patch.in_flight >= 3 {
        warn(&concurrency_warning_message(patch.in_flight));
    }

    push_to_origin_master(repo_root)?;

    Ok(receipt)
}

fn dedupe_addressed_findings(addresses_finding: &[AddressedFinding]) -> Vec<AddressedFinding> {
    let mut deduped = Vec::new();
    let mut seen = BTreeSet::new();
    for finding in addresses_finding {
        if seen.insert((finding.cycle, finding.index)) {
            deduped.push(finding.clone());
        }
    }
    deduped
}

fn set_session_addresses_findings(
    state: &mut Value,
    issue: u64,
    addresses_finding: &[AddressedFinding],
) -> Result<(), String> {
    let issue_i64 = i64::try_from(issue)
        .map_err(|error| format!("issue #{issue} exceeds i64 range: {error}"))?;
    let session = state
        .pointer_mut("/agent_sessions")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| "missing array /agent_sessions in docs/state.json".to_string())?
        .iter_mut()
        .rev()
        .find(|session| session.get("issue").and_then(Value::as_i64) == Some(issue_i64))
        .ok_or_else(|| format!("agent_sessions does not contain an entry for issue #{issue}"))?;
    let session_object = session
        .as_object_mut()
        .ok_or_else(|| format!("agent_sessions entry for issue #{issue} must be an object"))?;

    match addresses_finding {
        [] => {
            session_object.remove("addresses_finding");
            session_object.remove("addresses_findings");
        }
        [finding] => {
            session_object.remove("addresses_findings");
            session_object.insert(
                "addresses_finding".to_string(),
                serde_json::json!(finding.finding_ref()),
            );
        }
        findings => {
            session_object.remove("addresses_finding");
            session_object.insert(
                "addresses_findings".to_string(),
                serde_json::json!(findings
                    .iter()
                    .map(AddressedFinding::finding_ref)
                    .collect::<Vec<_>>()),
            );
        }
    }

    Ok(())
}

fn reconcile_review_history_dispatches(
    state: &mut Value,
    addressed_findings: &[AddressedFinding],
    warn: &mut dyn FnMut(&str),
) -> Result<(), String> {
    for addressed_finding in addressed_findings {
        reconcile_review_history_dispatch(state, addressed_finding, warn)?;
    }

    Ok(())
}

fn build_issue_payload(title: &str, body: &str, labels: &[String], model: &str) -> IssuePayload {
    IssuePayload {
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

fn read_body_file(path: &Path, allow_empty: bool) -> Result<String, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    let normalized = content.trim_end_matches(['\r', '\n']);
    if !allow_empty && normalized.trim().is_empty() {
        return Err(format!("{} must not be empty", path.display()));
    }
    Ok(normalized.to_string())
}

fn push_to_origin_master(repo_root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["push", "origin", "master"])
        .output()
        .map_err(|error| format!("failed to execute git push: {}", error))?;
    if !output.status.success() {
        return Err(command_failure_message("git push origin master", &output));
    }
    Ok(())
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
        .find(|entry| entry.get("cycle").and_then(Value::as_u64) == Some(addressed_finding.cycle))
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
    use std::{
        fs,
        path::Path,
        process::Command,
        time::{SystemTime, UNIX_EPOCH},
    };

    // ---------------------------------------------------------------------------
    // Unit tests for payload construction and CLI parsing
    // ---------------------------------------------------------------------------

    #[test]
    fn payload_correctly_constructed_from_cli_args() {
        let payload = build_issue_payload(
            "Fix the thing",
            "Body text here",
            &["agent-task".to_string()],
            "gpt-5.4",
        );

        assert_eq!(payload.title, "Fix the thing");
        assert_eq!(payload.body, "Body text here");
        assert_eq!(payload.labels, vec!["agent-task"]);
        assert_eq!(payload.assignees, vec!["copilot-swe-agent[bot]"]);
        assert_eq!(payload.agent_assignment.target_repo, MAIN_REPO);
        assert_eq!(payload.agent_assignment.base_branch, BASE_BRANCH);
        assert_eq!(payload.agent_assignment.model, "gpt-5.4");
        assert_eq!(payload.agent_assignment.custom_instructions, "");
    }

    #[test]
    fn default_model_is_gpt_5_4() {
        let repo = TempRepo::new();
        repo.write_config("gpt-5.4");
        let model =
            resolve_model(None, repo.path()).expect("resolve_model should succeed with config");
        assert_eq!(model, "gpt-5.4");
    }

    #[test]
    fn default_label_is_agent_task() {
        let cli =
            Cli::try_parse_from(["dispatch-task", "--title", "T", "--body-file", "/dev/null"])
                .expect("CLI should parse without --label");
        assert_eq!(cli.labels, vec!["agent-task"]);
    }

    #[test]
    fn multiple_label_flags_produce_correct_labels_array() {
        let cli = Cli::try_parse_from([
            "dispatch-task",
            "--title",
            "T",
            "--body-file",
            "/dev/null",
            "--label",
            "agent-task",
            "--label",
            "my-extra-label",
        ])
        .expect("CLI should parse with multiple --label flags");
        assert_eq!(cli.labels, vec!["agent-task", "my-extra-label"]);
    }

    #[test]
    fn dry_run_does_not_call_gh_api_or_modify_state_json() {
        let repo = TempRepo::new();
        repo.init();

        let body_file = repo.path().join("body.md");
        fs::write(&body_file, "task description").expect("body file should be written");

        let original_state = repo.read_state();
        let api_runner = PanicApiRunner;
        let pipeline_runner = MockCommandRunner::with_exit_code(Some(0));

        run_with_runners(
            Cli {
                title: "Test task".to_string(),
                body_file,
                model: Some("gpt-5.4".to_string()),
                labels: vec!["agent-task".to_string()],
                addresses_finding: Vec::new(),
                repo_root: repo.path().to_path_buf(),
                dry_run: true,
            },
            &pipeline_runner,
            &api_runner,
            &mut |_| {},
        )
        .expect("dry-run should succeed without errors");

        assert_eq!(
            pipeline_runner.call_count(),
            0,
            "pipeline check should not be called in dry-run"
        );
        assert_eq!(
            repo.read_state(),
            original_state,
            "state.json should not be modified in dry-run"
        );
    }

    #[test]
    fn dry_run_accepts_empty_body_file() {
        let repo = TempRepo::new();
        repo.init();

        let body_file = repo.path().join("empty.md");
        fs::write(&body_file, "").expect("empty body file should be written");

        let api_runner = PanicApiRunner;
        let pipeline_runner = MockCommandRunner::with_exit_code(Some(0));

        run_with_runners(
            Cli {
                title: "Test task".to_string(),
                body_file,
                model: Some("gpt-5.4".to_string()),
                labels: vec!["agent-task".to_string()],
                addresses_finding: Vec::new(),
                repo_root: repo.path().to_path_buf(),
                dry_run: true,
            },
            &pipeline_runner,
            &api_runner,
            &mut |_| {},
        )
        .expect("dry-run should succeed with empty body");
    }

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
    fn addresses_finding_parses_valid_input() {
        let finding: AddressedFinding = "164:2".parse().expect("should parse");
        assert_eq!(finding.cycle, 164);
        assert_eq!(finding.index, 2);
    }

    #[test]
    fn addresses_finding_cli_accepts_multiple_values() {
        let cli = Cli::try_parse_from([
            "dispatch-task",
            "--title",
            "T",
            "--body-file",
            "/dev/null",
            "--addresses-finding",
            "450:1",
            "--addresses-finding",
            "450:2",
        ])
        .expect("CLI should parse repeated --addresses-finding");

        let mut state = json!({
            "agent_sessions": [{
                "issue": 901
            }]
        });

        let addresses_finding = dedupe_addressed_findings(&cli.addresses_finding);
        set_session_addresses_findings(&mut state, 901, &addresses_finding)
            .expect("session linkage should be written");

        assert_eq!(
            state.pointer("/agent_sessions/0/addresses_findings"),
            Some(&json!(["450:1", "450:2"]))
        );
        assert_eq!(state.pointer("/agent_sessions/0/addresses_finding"), None);
    }

    #[test]
    fn addresses_finding_cli_keeps_singular_field_for_one_value() {
        let cli = Cli::try_parse_from([
            "dispatch-task",
            "--title",
            "T",
            "--body-file",
            "/dev/null",
            "--addresses-finding",
            "450:1",
        ])
        .expect("CLI should parse one --addresses-finding");

        let mut state = json!({
            "agent_sessions": [{
                "issue": 901
            }]
        });

        let addresses_finding = dedupe_addressed_findings(&cli.addresses_finding);
        set_session_addresses_findings(&mut state, 901, &addresses_finding)
            .expect("session linkage should be written");

        assert_eq!(
            state.pointer("/agent_sessions/0/addresses_finding"),
            Some(&json!("450:1"))
        );
        assert_eq!(state.pointer("/agent_sessions/0/addresses_findings"), None);
    }

    #[test]
    fn reconcile_review_history_dispatches_updates_all_referenced_deferred_findings() {
        let mut state = json!({
            "review_agent": {
                "history": [{
                    "cycle": 450,
                    "deferred": 2,
                    "dispatch_created": 0,
                    "finding_count": 2,
                    "finding_dispositions": [
                        {
                            "category": "state-integrity",
                            "disposition": "deferred"
                        },
                        {
                            "category": "tooling-contract",
                            "disposition": "deferred"
                        }
                    ]
                }]
            }
        });

        let addressed_findings = vec![
            "450:1".parse().expect("first finding should parse"),
            "450:2".parse().expect("second finding should parse"),
        ];

        reconcile_review_history_dispatches(&mut state, &addressed_findings, &mut |_| {})
            .expect("review history should reconcile");

        assert_eq!(
            state.pointer("/review_agent/history/0/deferred"),
            Some(&json!(0))
        );
        assert_eq!(
            state.pointer("/review_agent/history/0/dispatch_created"),
            Some(&json!(2))
        );
        assert_eq!(
            state.pointer("/review_agent/history/0/finding_dispositions/0/disposition"),
            Some(&json!("dispatch_created"))
        );
        assert_eq!(
            state.pointer("/review_agent/history/0/finding_dispositions/1/disposition"),
            Some(&json!("dispatch_created"))
        );
    }

    #[test]
    fn addresses_finding_rejects_malformed_input() {
        assert!("164".parse::<AddressedFinding>().is_err());
        assert!("0:1".parse::<AddressedFinding>().is_err());
        assert!("1:0".parse::<AddressedFinding>().is_err());
    }

    // ---------------------------------------------------------------------------
    // Mock helpers
    // ---------------------------------------------------------------------------

    struct MockCommandRunner {
        result: Result<record_dispatch::ExecutionResult, String>,
        call_count: std::cell::Cell<usize>,
    }

    impl MockCommandRunner {
        fn with_exit_code(exit_code: Option<i32>) -> Self {
            Self {
                result: Ok(record_dispatch::ExecutionResult { exit_code }),
                call_count: std::cell::Cell::new(0),
            }
        }

        fn call_count(&self) -> usize {
            self.call_count.get()
        }
    }

    impl CommandRunner for MockCommandRunner {
        fn run_pipeline_check(
            &self,
            _repo_root: &Path,
        ) -> Result<record_dispatch::ExecutionResult, String> {
            self.call_count.set(self.call_count.get() + 1);
            self.result.clone()
        }
    }

    /// A `GithubApiRunner` that panics if called — used to assert no API call is made.
    struct PanicApiRunner;

    impl GithubApiRunner for PanicApiRunner {
        fn create_issue(&self, _payload: &IssuePayload) -> Result<CreatedIssue, String> {
            panic!("GithubApiRunner should not be called in this test");
        }
    }

    // ---------------------------------------------------------------------------
    // TempRepo helper
    // ---------------------------------------------------------------------------

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
            fs::create_dir_all(path.join("docs")).expect("docs dir should be created");
            fs::create_dir_all(path.join("tools")).expect("tools dir should be created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn init(&self) {
            self.write_default_state();
            self.write_config("gpt-5.4");
            git_success(self.path(), ["init"]);
            git_success(self.path(), ["config", "user.name", "Dispatch Task Tests"]);
            git_success(
                self.path(),
                ["config", "user.email", "dispatch-task-tests@example.com"],
            );
            git_success(self.path(), ["add", "docs/state.json"]);
            git_success(self.path(), ["commit", "-m", "initial state"]);
        }

        fn write_default_state(&self) {
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
                "last_cycle": { "number": 164 },
                "cycle_phase": {
                    "cycle": 164,
                    "phase": "work",
                    "phase_entered_at": "2026-03-07T12:00:00Z"
                },
                "field_inventory": {
                    "fields": {
                        "in_flight_sessions": { "last_refreshed": "cycle 163" }
                    }
                },
                "review_agent": { "history": [] }
            });
            fs::write(
                self.path().join("docs/state.json"),
                serde_json::to_string_pretty(&state).expect("state should serialize"),
            )
            .expect("state file should be written");
        }

        fn write_config(&self, model: &str) {
            let config = json!({ "default_model": model });
            fs::write(
                self.path().join("tools/config.json"),
                serde_json::to_string_pretty(&config).expect("config should serialize"),
            )
            .expect("config file should be written");
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
