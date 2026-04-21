use clap::Parser;
use record_dispatch::{
    apply_dispatch_patch, build_dispatch_patch, dispatch_commit_message, enforce_pipeline_gate,
    restore_sealed_last_cycle, should_sync_last_cycle_summary, snapshot_sealed_last_cycle,
    sync_last_cycle_summary_after_dispatch, update_review_dispatch_tracking, ProcessRunner,
};
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
#[command(name = "dispatch-review")]
struct Cli {
    /// Current cycle number
    #[arg(long)]
    cycle: u64,

    /// Orchestrator run issue number for context in the review body
    #[arg(long)]
    issue: u64,

    /// Path to a file containing the review issue body
    #[arg(long)]
    body_file: PathBuf,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Print the issue JSON without creating it
    #[arg(long)]
    dry_run: bool,

    /// Record an already-created review issue number without calling gh api (testing/recovery)
    #[arg(long)]
    record_only: Option<u64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct AgentAssignment {
    target_repo: String,
    base_branch: String,
    model: String,
    custom_instructions: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct ReviewIssuePayload {
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

    let created_issue = if let Some(issue_number) = cli.record_only {
        CreatedIssue {
            number: issue_number,
            html_url: format!("https://github.com/{MAIN_REPO}/issues/{issue_number}"),
        }
    } else {
        create_issue(&payload)?
    };
    let state_result = record_created_issue(
        &cli.repo_root,
        current_cycle,
        created_issue.number,
        &payload.title,
        &model,
    );
    if let Err(error) = state_result {
        return Err(format!(
            "created review issue #{} ({}) but failed to update docs/state.json: {}",
            created_issue.number, created_issue.html_url, error
        ));
    }

    println!(
        "Created review issue #{} from orchestrator issue #{}: {}",
        created_issue.number, cli.issue, created_issue.html_url
    );
    Ok(())
}

fn build_issue_payload(cycle: u64, body: &str, model: &str) -> ReviewIssuePayload {
    ReviewIssuePayload {
        title: format!("[Cycle Review] Cycle {} end-of-cycle review", cycle),
        body: body.to_string(),
        labels: vec!["agent-task".to_string(), "cycle-review".to_string()],
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
    apply_dispatch_patch(state, &patch).map(|_| ())
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

fn resolve_cycle(cli_cycle: u64, repo_root: &Path) -> Result<u64, String> {
    let state_cycle = current_cycle_from_state(repo_root).map_err(|error| {
        if error == "missing /cycle_phase/cycle or /last_cycle/number in state.json" {
            "missing numeric /cycle_phase/cycle or /last_cycle/number in docs/state.json"
                .to_string()
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
    // Enforce pipeline gate (logs warning for review dispatches, blocks for others)
    if let Err(error) = enforce_pipeline_gate(repo_root, true, &ProcessRunner) {
        return Err(format!("pipeline gate check failed: {:?}", error));
    }

    let mut state = read_state_value(repo_root)?;
    let current_phase = state
        .pointer("/cycle_phase/phase")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let sealed_last_cycle = snapshot_sealed_last_cycle(&state, &current_phase);

    // Track consecutive review dispatches (warns at 3+)
    if let Some(warning) = update_review_dispatch_tracking(&mut state, true)? {
        eprintln!("Warning: {}", warning);
    }

    apply_dispatch_record(
        &mut state,
        cycle,
        issue,
        title,
        model,
        &current_utc_timestamp(),
    )?;
    if current_phase == "close_out" {
        transition_cycle_phase(&mut state, cycle, "complete")?;
    }
    restore_sealed_last_cycle(&mut state, sealed_last_cycle)?;
    if should_sync_last_cycle_summary(&current_phase) {
        sync_last_cycle_summary_after_dispatch(&mut state, cycle)?;
    }
    write_state_value(repo_root, &state)?;
    let commit_message = dispatch_commit_message(issue, cycle);
    commit_state_json(repo_root, &commit_message)?;
    Ok(())
}

fn create_issue(payload: &ReviewIssuePayload) -> Result<CreatedIssue, String> {
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
    use std::fs;
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    const CYCLE_495_CLOSE_OUT_FIXTURE: &str = include_str!(
        "../../record-dispatch/tests/fixtures/cycle-495-post-cycle-complete-state.json"
    );

    fn sample_state() -> Value {
        json!({
            "agent_sessions": [
                {
                    "issue": 601,
                    "title": "old dispatch",
                    "dispatched_at": "2026-03-01T00:00:00Z",
                    "model": "gpt-5.4",
                    "status": "merged",
                    "pr": 700,
                    "merged_at": "2026-03-02T00:00:00Z"
                }
            ],
            "in_flight_sessions": 0,
            "dispatch_log_latest": "#601 old dispatch (cycle 164)",
            "field_inventory": {
                "fields": {
                    "in_flight_sessions": { "last_refreshed": "cycle 163" }
                }
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
        assert!(help.contains("--record-only"));
    }

    #[test]
    fn build_issue_payload_includes_labels_assignee_and_agent_assignment() {
        let payload = build_issue_payload(200, "Review body", "gpt-5.4");

        assert_eq!(
            payload.title,
            "[Cycle Review] Cycle 200 end-of-cycle review"
        );
        assert_eq!(payload.labels, vec!["agent-task", "cycle-review"]);
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
    fn apply_dispatch_record_updates_metrics_and_appends_agent_session() {
        let mut state = sample_state();

        apply_dispatch_record(
            &mut state,
            200,
            849,
            "[Cycle Review] Cycle 200 end-of-cycle review",
            "gpt-5.4",
            "2026-03-09T02:00:00Z",
        )
        .expect("dispatch record should apply");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions should be an array");
        assert_eq!(state["in_flight_sessions"], json!(1));
        assert_eq!(
            state["dispatch_log_latest"],
            json!("#849 [Cycle Review] Cycle 200 end-of-cycle review (cycle 200)")
        );
        assert_eq!(
            state["field_inventory"]["fields"]["in_flight_sessions"]["last_refreshed"],
            json!("cycle 200")
        );
        assert_eq!(sessions.len(), 2);
        assert_eq!(sessions[1]["issue"], json!(849));
        assert_eq!(sessions[1]["status"], json!("in_flight"));
        assert_eq!(sessions[1]["model"], json!("gpt-5.4"));
    }

    #[test]
    fn record_created_issue_preserves_completed_cycle_snapshot() {
        let repo = TempRepo::new();
        repo.init_with_state_json(CYCLE_495_CLOSE_OUT_FIXTURE);
        let before = repo.read_state();
        let original_summary = before["last_cycle"]["summary"].clone();
        let original_timestamp = before["last_cycle"]["timestamp"]
            .as_str()
            .expect("fixture should include last_cycle timestamp")
            .to_string();
        let original_completed_at = before.pointer("/cycle_phase/completed_at").cloned();

        record_created_issue(
            repo.path(),
            495,
            2521,
            "[Cycle Review] Cycle 495 end-of-cycle review",
            "gpt-5.4",
        )
        .expect("record should succeed");

        let state = repo.read_state();
        assert_eq!(
            state.pointer("/cycle_phase/phase"),
            Some(&json!("complete"))
        );
        assert_eq!(
            state.pointer("/last_cycle/summary"),
            Some(&original_summary)
        );
        assert_eq!(
            state.pointer("/last_cycle/timestamp"),
            Some(&json!(original_timestamp))
        );
        assert_eq!(
            state.pointer("/cycle_phase/completed_at"),
            original_completed_at.as_ref()
        );
        assert_eq!(
            state.pointer("/dispatch_log_latest"),
            Some(&json!(
                "#2521 [Cycle Review] Cycle 495 end-of-cycle review (cycle 495)"
            ))
        );
        assert_eq!(state.pointer("/agent_sessions/2/issue"), Some(&json!(2521)));
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
                "dispatch-review-test-{}-{}",
                std::process::id(),
                unique
            ));
            fs::create_dir_all(path.join("docs")).expect("temp repo should be created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn init_with_state_json(&self, state_json: &str) {
            let state: Value = serde_json::from_str(state_json).expect("fixture should parse");
            self.init_with_state_value(&state);
        }

        fn init_with_state_value(&self, state: &Value) {
            self.write_state_value(state);

            git_success(self.path(), ["init"]);
            git_success(
                self.path(),
                ["config", "user.name", "Dispatch Review Tests"],
            );
            git_success(
                self.path(),
                ["config", "user.email", "dispatch-review-tests@example.com"],
            );
            git_success(self.path(), ["add", "docs/state.json"]);
            git_success(self.path(), ["commit", "-m", "initial state"]);
        }

        fn read_state(&self) -> Value {
            serde_json::from_str(
                &fs::read_to_string(self.path().join("docs/state.json"))
                    .expect("state file should be readable"),
            )
            .expect("state file should parse")
        }

        fn write_state_value(&self, state: &Value) {
            write_state_value(self.path(), state).expect("state should be written");
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn git_success<const N: usize>(repo_root: &Path, args: [&str; N]) {
        let output = Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .args(args)
            .output()
            .expect("git command should run");
        assert!(
            output.status.success(),
            "git {:?} failed: stdout={} stderr={}",
            args,
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
