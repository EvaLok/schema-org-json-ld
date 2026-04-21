use clap::Parser;
use record_dispatch::{
    apply_dispatch_patch, build_dispatch_patch, resolve_model, should_sync_last_cycle_summary,
    sync_last_cycle_summary_after_dispatch,
};
use serde::Deserialize;
use serde_json::{json, Value};
use state_schema::{
    commit_state_json, current_cycle_from_state, read_state_value, write_state_value,
};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";

#[derive(Parser, Debug)]
#[command(name = "backfill-dispatch")]
struct Cli {
    #[arg(long)]
    issue: u64,

    #[arg(long)]
    cycle: u64,

    #[arg(long)]
    reason: Option<String>,

    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct GithubIssue {
    number: u64,
    title: String,
    created_at: String,
    state: String,
    #[serde(default)]
    pull_request: Option<Value>,
}

trait GithubIssueRunner {
    fn fetch_issue(&self, issue: u64) -> Result<GithubIssue, String>;
}

struct ProcessGithubIssueRunner;

impl GithubIssueRunner for ProcessGithubIssueRunner {
    fn fetch_issue(&self, issue: u64) -> Result<GithubIssue, String> {
        let output = Command::new("gh")
            .arg("api")
            .arg(format!("repos/{MAIN_REPO}/issues/{issue}"))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|error| format!("failed to execute gh api: {}", error))?;
        if !output.status.success() {
            return Err(command_failure_message("gh api", &output));
        }
        serde_json::from_slice(&output.stdout)
            .map_err(|error| format!("failed to parse gh api issue response: {}", error))
    }
}

fn main() {
    let cli = Cli::parse();
    let runner = ProcessGithubIssueRunner;
    if let Err(error) = run_with_runner(cli, &runner) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run_with_runner(cli: Cli, runner: &dyn GithubIssueRunner) -> Result<(), String> {
    let current_cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /cycle_phase/cycle or /last_cycle/number in state.json" {
            "missing numeric /cycle_phase/cycle or /last_cycle/number in docs/state.json"
                .to_string()
        } else {
            error
        }
    })?;
    if cli.cycle != current_cycle {
        return Err(format!(
            "--cycle {} does not match docs/state.json current cycle {}; backfill-dispatch only supports the live cycle",
            cli.cycle, current_cycle
        ));
    }

    let issue = runner.fetch_issue(cli.issue)?;
    if issue.number != cli.issue {
        return Err(format!(
            "gh api returned issue #{} for requested issue #{}",
            issue.number, cli.issue
        ));
    }
    if issue.pull_request.is_some() {
        return Err(format!(
            "issue #{} is a pull request; backfill-dispatch only supports issues",
            cli.issue
        ));
    }

    let mut state_value = read_state_value(&cli.repo_root)?;
    if state_value
        .pointer("/agent_sessions")
        .and_then(Value::as_array)
        .is_some_and(|sessions| {
            sessions
                .iter()
                .any(|session| session.get("issue").and_then(Value::as_u64) == Some(cli.issue))
        })
    {
        return Err(format!(
            "docs/state.json /agent_sessions already contains issue #{}",
            cli.issue
        ));
    }

    let current_phase = state_value
        .pointer("/cycle_phase/phase")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let model = resolve_model(None, &cli.repo_root)?;
    let patch = build_dispatch_patch(
        &state_value,
        cli.cycle,
        cli.issue,
        &issue.title,
        &model,
        &issue.created_at,
    )?;
    let updated_existing = apply_dispatch_patch(&mut state_value, &patch)?;
    if updated_existing {
        return Err(format!(
            "issue #{} unexpectedly merged into an existing session; refusing ambiguous backfill",
            cli.issue
        ));
    }
    annotate_backfilled_session(&mut state_value, cli.issue, cli.reason.as_deref())?;
    if issue.state.eq_ignore_ascii_case("closed") {
        set_session_status(&mut state_value, cli.issue, "closed_without_pr")?;
        refresh_in_flight_sessions(&mut state_value)?;
    }
    if should_sync_last_cycle_summary(&current_phase) {
        sync_last_cycle_summary_after_dispatch(&mut state_value, patch.current_cycle)?;
    }

    if cli.dry_run {
        println!(
            "{}",
            serde_json::to_string_pretty(&state_value)
                .map_err(|error| format!("failed to serialize dry-run state: {}", error))?
        );
        return Ok(());
    }

    write_state_value(&cli.repo_root, &state_value)?;
    let receipt = commit_state_json(
        &cli.repo_root,
        &format!(
            "state(backfill-dispatch): #{} ledger backfill [cycle {}]",
            cli.issue, cli.cycle
        ),
    )?;
    println!(
        "Backfilled dispatch ledger for #{} (receipt: {})",
        cli.issue, receipt
    );
    Ok(())
}

fn annotate_backfilled_session(
    state: &mut Value,
    issue: u64,
    reason: Option<&str>,
) -> Result<(), String> {
    let session = state
        .pointer_mut("/agent_sessions")
        .and_then(Value::as_array_mut)
        .and_then(|sessions| {
            sessions
                .iter_mut()
                .find(|session| session.get("issue").and_then(Value::as_u64) == Some(issue))
        })
        .and_then(Value::as_object_mut)
        .ok_or_else(|| {
            format!(
                "failed to locate backfilled agent_sessions entry for #{}",
                issue
            )
        })?;
    session.insert("backfilled".to_string(), json!(true));
    session.insert("dispatch_provenance".to_string(), json!("gh-api-direct"));
    if let Some(reason) = reason.map(str::trim).filter(|reason| !reason.is_empty()) {
        session.insert("backfill_reason".to_string(), json!(reason));
    }
    Ok(())
}

fn set_session_status(state: &mut Value, issue: u64, status: &str) -> Result<(), String> {
    let session = state
        .pointer_mut("/agent_sessions")
        .and_then(Value::as_array_mut)
        .and_then(|sessions| {
            sessions
                .iter_mut()
                .find(|session| session.get("issue").and_then(Value::as_u64) == Some(issue))
        })
        .and_then(Value::as_object_mut)
        .ok_or_else(|| format!("failed to locate agent_sessions entry for #{}", issue))?;
    session.insert("status".to_string(), json!(status));
    Ok(())
}

fn refresh_in_flight_sessions(state: &mut Value) -> Result<(), String> {
    let sessions = state
        .pointer("/agent_sessions")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing array /agent_sessions in docs/state.json".to_string())?;
    let in_flight = sessions.iter().filter(|session| {
        matches!(
            session.get("status").and_then(Value::as_str),
            Some("in_flight") | Some("dispatched")
        )
    });
    let count = i64::try_from(in_flight.count())
        .map_err(|_| "in-flight session count does not fit in i64".to_string())?;
    state
        .as_object_mut()
        .ok_or_else(|| "docs/state.json root must be an object".to_string())?
        .insert("in_flight_sessions".to_string(), json!(count));
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
    use serde_json::json;
    use state_schema::StateJson;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Clone)]
    struct MockGithubIssueRunner {
        result: Result<GithubIssue, String>,
    }

    impl GithubIssueRunner for MockGithubIssueRunner {
        fn fetch_issue(&self, _issue: u64) -> Result<GithubIssue, String> {
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
                "backfill-dispatch-test-{}-{}",
                std::process::id(),
                unique
            ));
            fs::create_dir_all(path.join("docs")).unwrap();
            fs::create_dir_all(path.join("tools")).unwrap();
            Self { path }
        }

        fn init(&self) {
            fs::write(
                self.path.join("docs/state.json"),
                json!({
                    "agent_sessions": [],
                    "in_flight_sessions": 0,
                    "last_cycle": {
                        "number": 523,
                        "summary": "0 dispatches, 1 merges"
                    },
                    "cycle_phase": {
                        "cycle": 523,
                        "phase": "work",
                        "phase_entered_at": "2026-04-21T01:00:00Z"
                    },
                    "field_inventory": {
                        "fields": {
                            "in_flight_sessions": {
                                "last_refreshed": "cycle 522"
                            }
                        }
                    },
                    "review_agent": {
                        "history": []
                    },
                    "tool_pipeline": {}
                })
                .to_string(),
            )
            .unwrap();
            fs::write(
                self.path.join("tools/config.json"),
                r#"{"default_model":"gpt-5.4"}"#,
            )
            .unwrap();
            git_success(self.path(), ["init"]);
            git_success(
                self.path(),
                ["config", "user.name", "Backfill Dispatch Tests"],
            );
            git_success(
                self.path(),
                [
                    "config",
                    "user.email",
                    "backfill-dispatch-tests@example.com",
                ],
            );
            git_success(self.path(), ["add", "docs/state.json", "tools/config.json"]);
            git_success(self.path(), ["commit", "-m", "initial state"]);
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn backfill_dispatch_writes_compliant_agent_session_entry() {
        let repo = TempRepo::new();
        repo.init();
        let runner = MockGithubIssueRunner {
            result: Ok(GithubIssue {
                number: 2631,
                title: "Fix close-out ordering".to_string(),
                created_at: "2026-04-21T02:10:00Z".to_string(),
                state: "open".to_string(),
                pull_request: None,
            }),
        };

        run_with_runner(
            Cli {
                issue: 2631,
                cycle: 523,
                reason: Some("manual gh-api-direct fallback".to_string()),
                repo_root: repo.path().to_path_buf(),
                dry_run: false,
            },
            &runner,
        )
        .expect("backfill should succeed");

        let content = fs::read_to_string(repo.path().join("docs/state.json")).unwrap();
        let state: StateJson = serde_json::from_str(&content).expect("backfilled state must parse");
        let session = state
            .agent_sessions
            .iter()
            .find(|session| session.issue == Some(2631))
            .expect("backfilled session should exist");
        assert_eq!(session.title.as_deref(), Some("Fix close-out ordering"));
        assert_eq!(
            session.dispatched_at.as_deref(),
            Some("2026-04-21T02:10:00Z")
        );
        assert_eq!(session.model.as_deref(), Some("gpt-5.4"));
        assert_eq!(session.status.as_deref(), Some("in_flight"));
        assert_eq!(
            session.extra.get("dispatch_provenance"),
            Some(&json!("gh-api-direct"))
        );
        assert_eq!(session.extra.get("backfilled"), Some(&json!(true)));
        assert_eq!(
            session.extra.get("backfill_reason"),
            Some(&json!("manual gh-api-direct fallback"))
        );
    }

    #[test]
    fn backfill_dispatch_rejects_cycle_mismatch() {
        let repo = TempRepo::new();
        repo.init();
        let runner = MockGithubIssueRunner {
            result: Ok(GithubIssue {
                number: 2631,
                title: "Fix close-out ordering".to_string(),
                created_at: "2026-04-21T02:10:00Z".to_string(),
                state: "open".to_string(),
                pull_request: None,
            }),
        };

        let error = run_with_runner(
            Cli {
                issue: 2631,
                cycle: 522,
                reason: None,
                repo_root: repo.path().to_path_buf(),
                dry_run: false,
            },
            &runner,
        )
        .expect_err("cycle mismatch should fail");

        assert!(error.contains("does not match docs/state.json current cycle"));
    }

    fn git_success<I, S>(repo_root: &Path, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let output = Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .args(args)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
