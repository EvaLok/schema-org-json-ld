use clap::Parser;
use record_dispatch::push_to_origin_master;
use serde_json::{json, Value};
use state_schema::{
    commit_state_json, current_cycle_from_state, current_utc_timestamp, read_state_value,
    write_state_value,
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Parser, Debug)]
#[command(name = "close-session")]
struct Cli {
    /// GitHub issue number whose agent_sessions row should be closed
    #[arg(long)]
    issue: u64,

    /// Free-text reason stored as closed_reason
    #[arg(long)]
    reason: String,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Print a unified diff without writing docs/state.json
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let reason = cli.reason.trim().to_string();
    if reason.is_empty() {
        return Err("--reason must not be empty".to_string());
    }

    let mut state = read_state_value(&cli.repo_root)?;
    let before = serialize_state(&state)?;
    let closed_at = current_utc_timestamp();
    let in_flight = close_session_in_state(&mut state, cli.issue, &reason, &closed_at)?;
    let after = serialize_state(&state)?;

    if cli.dry_run {
        print!("{}", render_unified_diff(&before, &after)?);
        return Ok(());
    }

    write_state_value(&cli.repo_root, &state)?;
    let cycle = current_cycle_from_state(&cli.repo_root)?;
    let commit_message = format!(
        "state(close-session): #{} closed ({reason}) [cycle {cycle}]",
        cli.issue
    );
    let receipt = commit_state_json(&cli.repo_root, &commit_message)?;
    push_to_origin_master(&cli.repo_root)?;
    println!(
        "Closed agent_sessions row for #{}. in_flight: {} (receipt: {})",
        cli.issue, in_flight, receipt
    );
    Ok(())
}

fn close_session_in_state(
    state: &mut Value,
    issue: u64,
    reason: &str,
    closed_at: &str,
) -> Result<u64, String> {
    let issue_i64 = i64::try_from(issue)
        .map_err(|error| format!("issue #{issue} exceeds i64 range: {error}"))?;
    let in_flight = {
        let sessions = state
            .pointer_mut("/agent_sessions")
            .and_then(Value::as_array_mut)
            .ok_or_else(|| "missing array /agent_sessions in docs/state.json".to_string())?;
        let session = sessions
            .iter_mut()
            .find(|session| session.get("issue").and_then(Value::as_i64) == Some(issue_i64))
            .ok_or_else(|| format!("no agent_sessions row for issue #{issue}"))?;
        let status = session
            .get("status")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                format!("agent_sessions row for issue #{issue} is not in_flight (status: missing)")
            })?;
        if status != "in_flight" {
            return Err(format!(
                "agent_sessions row for issue #{issue} is not in_flight (status: {status})"
            ));
        }

        let session_object = session
            .as_object_mut()
            .ok_or_else(|| format!("agent_sessions row for issue #{issue} must be an object"))?;
        session_object.insert("status".to_string(), json!("closed"));
        session_object.insert("closed_at".to_string(), json!(closed_at));
        session_object.insert("closed_reason".to_string(), json!(reason));

        sessions
            .iter()
            .filter(|session| session.get("status").and_then(Value::as_str) == Some("in_flight"))
            .count() as u64
    };

    state
        .as_object_mut()
        .ok_or_else(|| "docs/state.json root must be an object".to_string())?
        .insert("in_flight_sessions".to_string(), json!(in_flight));

    Ok(in_flight)
}

fn serialize_state(state: &Value) -> Result<String, String> {
    let serialized = serde_json::to_string_pretty(state)
        .map_err(|error| format!("failed to serialize state.json: {error}"))?;
    Ok(format!("{serialized}\n"))
}

fn render_unified_diff(before: &str, after: &str) -> Result<String, String> {
    let temp_root = std::env::temp_dir().join(format!(
        "close-session-diff-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos()
    ));
    fs::create_dir_all(&temp_root).map_err(|error| {
        format!(
            "failed to create temp diff dir {}: {error}",
            temp_root.display()
        )
    })?;
    let before_path = temp_root.join("before.json");
    let after_path = temp_root.join("after.json");
    let result = render_unified_diff_files(&before_path, &after_path, before, after);
    let _ = fs::remove_dir_all(&temp_root);
    result
}

fn render_unified_diff_files(
    before_path: &Path,
    after_path: &Path,
    before: &str,
    after: &str,
) -> Result<String, String> {
    fs::write(before_path, before)
        .map_err(|error| format!("failed to write {}: {error}", before_path.display()))?;
    fs::write(after_path, after)
        .map_err(|error| format!("failed to write {}: {error}", after_path.display()))?;
    let output = Command::new("git")
        .args(["diff", "--no-index", "--no-color", "--"])
        .arg(before_path)
        .arg(after_path)
        .output()
        .map_err(|error| format!("failed to execute git diff --no-index: {error}"))?;

    match output.status.code() {
        Some(0) | Some(1) => Ok(String::from_utf8_lossy(&output.stdout).into_owned()),
        _ => {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(format!("git diff --no-index failed: {stderr}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn close_session_updates_row_and_recomputes_in_flight() {
        let mut state = json!({
            "agent_sessions": [
                {
                    "issue": 41,
                    "status": "in_flight",
                    "title": "Still active"
                },
                {
                    "issue": 42,
                    "status": "in_flight",
                    "title": "Close me",
                    "model": "gpt-5.4"
                },
                {
                    "issue": 43,
                    "status": "merged"
                }
            ],
            "in_flight_sessions": 2
        });

        let in_flight =
            close_session_in_state(&mut state, 42, "merged elsewhere", "2026-04-09T00:00:00Z")
                .expect("session should close");

        assert_eq!(in_flight, 1);
        assert_eq!(state.pointer("/in_flight_sessions"), Some(&json!(1)));
        assert_eq!(
            state.pointer("/agent_sessions/1/status"),
            Some(&json!("closed"))
        );
        assert_eq!(
            state.pointer("/agent_sessions/1/closed_at"),
            Some(&json!("2026-04-09T00:00:00Z"))
        );
        assert_eq!(
            state.pointer("/agent_sessions/1/closed_reason"),
            Some(&json!("merged elsewhere"))
        );
        assert_eq!(
            state.pointer("/agent_sessions/1/model"),
            Some(&json!("gpt-5.4"))
        );
    }

    #[test]
    fn close_session_rejects_missing_issue() {
        let mut state = json!({
            "agent_sessions": [
                {
                    "issue": 41,
                    "status": "in_flight"
                }
            ]
        });

        let error = close_session_in_state(&mut state, 99, "test", "2026-04-09T00:00:00Z")
            .expect_err("missing session should fail");

        assert_eq!(error, "no agent_sessions row for issue #99");
    }

    #[test]
    fn close_session_rejects_non_in_flight_status() {
        let mut state = json!({
            "agent_sessions": [
                {
                    "issue": 41,
                    "status": "merged"
                }
            ]
        });

        let error = close_session_in_state(&mut state, 41, "test", "2026-04-09T00:00:00Z")
            .expect_err("merged session should fail");

        assert_eq!(
            error,
            "agent_sessions row for issue #41 is not in_flight (status: merged)"
        );
    }

    #[test]
    fn dry_run_prints_unified_diff_without_writing() {
        let repo = TempRepo::new();
        let output = run(Cli {
            issue: 42,
            reason: "stale session".to_string(),
            repo_root: repo.path.clone(),
            dry_run: true,
        });

        assert!(output.is_ok(), "dry-run should succeed: {output:?}");

        let state = fs::read_to_string(repo.path.join("docs/state.json"))
            .expect("state file should remain readable");
        assert!(state.contains("\"in_flight_sessions\": 1"));
        assert!(!state.contains("closed_reason"));
    }

    #[test]
    fn help_mentions_issue_and_reason() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--issue"));
        assert!(help.contains("--reason"));
    }

    struct TempRepo {
        path: PathBuf,
    }

    impl TempRepo {
        fn new() -> Self {
            let path = std::env::temp_dir().join(format!(
                "close-session-test-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("system time should be after epoch")
                    .as_nanos()
            ));
            fs::create_dir_all(path.join("docs")).expect("docs dir should exist");
            fs::write(
                path.join("docs/state.json"),
                r#"{
  "agent_sessions": [
    {
      "issue": 42,
      "status": "in_flight",
      "title": "Close me"
    }
  ],
  "in_flight_sessions": 1
}
"#,
            )
            .expect("state file should be written");
            Self { path }
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }
}
