use clap::Parser;
use record_dispatch::{apply_dispatch_patch, build_dispatch_patch};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use state_schema::{
    commit_state_json, current_cycle_from_state, current_utc_timestamp, read_state_value,
    transition_cycle_phase, write_state_value,
};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const BASE_BRANCH: &str = "master";

#[derive(Parser, Debug)]
#[command(name = "dispatch-docs")]
struct Cli {
    /// Current cycle number
    #[arg(long)]
    cycle: u64,

    /// Orchestrator run issue number for context in the docs body
    #[arg(long)]
    issue: u64,

    /// Path to a file containing the docs issue body
    #[arg(long)]
    body_file: PathBuf,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Print the issue JSON without creating it
    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct AgentAssignment {
    target_repo: String,
    base_branch: String,
    model: String,
    custom_instructions: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct DocsIssuePayload {
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

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let current_cycle = resolve_cycle(cli.cycle, &cli.repo_root)?;
    let body = read_body_file(&cli.body_file)?;
    let body = inject_cycle_receipts(&cli.repo_root, current_cycle, &body, load_cycle_receipts);
    let model = state_schema::default_agent_model(&cli.repo_root)?;
    let payload = build_issue_payload(current_cycle, &body, &model);

    if cli.dry_run {
        println!(
            "{}",
            serde_json::to_string_pretty(&payload)
                .map_err(|error| format!("failed to serialize dry-run payload: {}", error))?
        );
        return Ok(());
    }

    let created_issue = create_issue(&payload)?;
    let state_result = record_created_issue(
        &cli.repo_root,
        current_cycle,
        created_issue.number,
        &payload.title,
        &model,
    );
    if let Err(error) = state_result {
        return Err(format!(
            "created docs issue #{} ({}) but failed to update docs/state.json: {}",
            created_issue.number, created_issue.html_url, error
        ));
    }

    println!(
        "Created docs issue #{} from orchestrator issue #{}: {}",
        created_issue.number, cli.issue, created_issue.html_url
    );
    Ok(())
}

fn build_issue_payload(cycle: u64, body: &str, model: &str) -> DocsIssuePayload {
    DocsIssuePayload {
        title: format!("[Cycle Docs] Cycle {} worklog and journal", cycle),
        body: body.to_string(),
        labels: vec!["agent-task".to_string(), "cycle-docs".to_string()],
        assignees: vec!["copilot-swe-agent[bot]".to_string()],
        agent_assignment: AgentAssignment {
            target_repo: MAIN_REPO.to_string(),
            base_branch: BASE_BRANCH.to_string(),
            model: model.to_string(),
            custom_instructions: String::new(),
        },
    }
}

fn apply_dispatch_record(
    state: &mut Value,
    cycle: u64,
    issue: u64,
    title: &str,
    model: &str,
    dispatched_at: &str,
) -> Result<(), String> {
    let patch = build_dispatch_patch(state, cycle, issue, title, model, dispatched_at)?;
    apply_dispatch_patch(state, &patch)
}

fn apply_cycle_phase_update(state: &mut Value, cycle: u64, issue: u64) -> Result<(), String> {
    // Use shared transition function for phase + freshness
    transition_cycle_phase(state, cycle, "doc_dispatched")?;

    // Set doc-specific fields on top of the phase transition
    let cycle_phase = state
        .pointer_mut("/cycle_phase")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "missing object /cycle_phase in docs/state.json".to_string())?;

    cycle_phase.insert("doc_issue".to_string(), serde_json::json!(issue as i64));
    cycle_phase.insert("doc_pr".to_string(), Value::Null);
    cycle_phase.insert("review_iteration".to_string(), serde_json::json!(0));
    cycle_phase.insert("review_max".to_string(), serde_json::json!(3));

    Ok(())
}

fn read_body_file(path: &Path) -> Result<String, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    let normalized = content.trim_end_matches(['\r', '\n']);
    if normalized.trim().is_empty() {
        return Err(format!("{} must not be empty", path.display()));
    }

    Ok(normalized.to_string())
}

fn inject_cycle_receipts<F>(repo_root: &Path, cycle: u64, body: &str, loader: F) -> String
where
    F: Fn(&Path, u64) -> Result<String, String>,
{
    match loader(repo_root, cycle) {
        Ok(receipts_output) => format!(
            "{body}\n\n## Cycle receipts (auto-generated)\n\n{}",
            receipts_output.trim_end_matches(['\r', '\n'])
        ),
        Err(error) => {
            eprintln!(
                "Warning: failed to generate cycle receipts for cycle {}: {}",
                cycle, error
            );
            body.to_string()
        }
    }
}

fn load_cycle_receipts(repo_root: &Path, cycle: u64) -> Result<String, String> {
    let output = Command::new("bash")
        .arg("tools/cycle-receipts")
        .arg("--cycle")
        .arg(cycle.to_string())
        .arg("--repo-root")
        .arg(repo_root.display().to_string())
        .current_dir(repo_root)
        .output()
        .map_err(|error| {
            format!(
                "failed to run bash tools/cycle-receipts --cycle {} --repo-root {}: {}",
                cycle,
                repo_root.display(),
                error
            )
        })?;
    if !output.status.success() {
        return Err(command_failure_message(
            "bash tools/cycle-receipts",
            &output,
        ));
    }

    String::from_utf8(output.stdout)
        .map_err(|error| format!("failed to decode cycle-receipts output as UTF-8: {}", error))
}

fn resolve_cycle(cli_cycle: u64, repo_root: &Path) -> Result<u64, String> {
    let state_cycle = current_cycle_from_state(repo_root).map_err(|error| {
        if error == "missing /last_cycle/number in state.json" {
            "missing numeric /last_cycle/number in docs/state.json".to_string()
        } else {
            error
        }
    })?;

    if cli_cycle != state_cycle {
        return Err(format!(
            "--cycle {} does not match docs/state.json current cycle {}",
            cli_cycle, state_cycle
        ));
    }

    Ok(state_cycle)
}

fn record_created_issue(
    repo_root: &Path,
    cycle: u64,
    issue: u64,
    title: &str,
    model: &str,
) -> Result<(), String> {
    let mut state = read_state_value(repo_root)?;
    let timestamp = current_utc_timestamp();
    apply_dispatch_record(&mut state, cycle, issue, title, model, &timestamp)?;
    apply_cycle_phase_update(&mut state, cycle, issue)?;
    write_state_value(repo_root, &state)?;
    let commit_message = format!(
        "state(dispatch-docs): #{} dispatched [cycle {}]",
        issue, cycle
    );
    commit_state_json(repo_root, &commit_message)?;
    Ok(())
}

fn create_issue(payload: &DocsIssuePayload) -> Result<CreatedIssue, String> {
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

    fn successful_receipt_loader(_: &Path, cycle: u64) -> Result<String, String> {
        Ok(format!(
            "## Commit receipts — Cycle {}\n\n| Step | Receipt |",
            cycle
        ))
    }

    fn failing_receipt_loader(_: &Path, _: u64) -> Result<String, String> {
        Err("cycle-receipts unavailable".to_string())
    }

    fn sample_state() -> Value {
        json!({
            "agent_sessions": [
                {
                    "issue": 601,
                    "title": "old dispatch",
                    "dispatched_at": "2026-03-01T00:00:00Z",
                    "model": "gpt-5.4",
                    "status": "merged"
                }
            ],
            "copilot_metrics": {
                "total_dispatches": 85,
                "in_flight": 2,
                "resolved": 83,
                "dispatch_log_latest": "#601 old dispatch (cycle 164)"
            },
            "field_inventory": {
                "fields": {
                    "copilot_metrics.in_flight": { "last_refreshed": "cycle 163" }
                }
            },
            "cycle_phase": {
                "cycle": 218,
                "phase": "work",
                "doc_issue": null,
                "doc_pr": null,
                "review_iteration": 0,
                "review_max": 3,
                "phase_entered_at": "2026-03-09T00:00:00Z"
            }
        })
    }

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--cycle"));
        assert!(help.contains("--issue"));
        assert!(help.contains("--body-file"));
        assert!(help.contains("--repo-root"));
        assert!(help.contains("--dry-run"));
    }

    #[test]
    fn build_issue_payload_uses_cycle_docs_labels() {
        let payload = build_issue_payload(219, "Docs body", "gpt-5.4");

        assert_eq!(payload.title, "[Cycle Docs] Cycle 219 worklog and journal");
        assert_eq!(payload.labels, vec!["agent-task", "cycle-docs"]);
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
    fn apply_dispatch_and_phase_update_sets_all_cycle_phase_fields() {
        let mut state = sample_state();
        let timestamp = "2026-03-10T12:00:00Z";

        apply_dispatch_record(
            &mut state,
            219,
            980,
            "[Cycle Docs] Cycle 219 worklog and journal",
            "gpt-5.4",
            timestamp,
        )
        .expect("dispatch record should apply");

        apply_cycle_phase_update(&mut state, 219, 980).expect("cycle phase update should apply");

        // Verify dispatch record was applied
        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions should be an array");
        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[1]["issue"], json!(980));
        assert_eq!(sessions[1]["status"], json!("in_flight"));

        // Verify all cycle_phase fields
        assert_eq!(state["cycle_phase"]["phase"], json!("doc_dispatched"));
        assert_eq!(state["cycle_phase"]["doc_issue"], json!(980));
        assert_eq!(state["cycle_phase"]["doc_pr"], json!(null));
        assert_eq!(state["cycle_phase"]["review_iteration"], json!(0));
        assert_eq!(state["cycle_phase"]["review_max"], json!(3));
        assert_eq!(state["cycle_phase"]["cycle"], json!(219));
        // phase_entered_at is set by transition_cycle_phase (not the hardcoded timestamp)
        assert!(state["cycle_phase"]["phase_entered_at"].is_string());

        // Verify freshness was bumped
        assert_eq!(
            state
                .pointer("/field_inventory/fields/cycle_phase/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 219")
        );
    }

    #[test]
    fn inject_cycle_receipts_appends_generated_section() {
        let body = inject_cycle_receipts(
            Path::new("/tmp/repo"),
            219,
            "Existing docs body",
            successful_receipt_loader,
        );

        assert_eq!(
            body,
            "Existing docs body\n\n## Cycle receipts (auto-generated)\n\n## Commit receipts — Cycle 219\n\n| Step | Receipt |"
        );
    }

    #[test]
    fn inject_cycle_receipts_falls_back_when_receipt_generation_fails() {
        let body = inject_cycle_receipts(
            Path::new("/tmp/repo"),
            219,
            "Existing docs body",
            failing_receipt_loader,
        );

        assert_eq!(body, "Existing docs body");
    }
}
