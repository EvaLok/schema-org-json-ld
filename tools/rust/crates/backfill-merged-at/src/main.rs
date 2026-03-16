use chrono::DateTime;
use clap::Parser;
use serde::Deserialize;
use serde_json::Value;
use state_schema::{check_version, read_state_value, write_state_value, AgentSession, StateJson};
use std::path::{Path, PathBuf};
use std::process::Command;

const REPO: &str = "EvaLok/schema-org-json-ld";

#[derive(Debug, Parser)]
#[command(name = "backfill-merged-at")]
struct Cli {
    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SessionToBackfill {
    index: usize,
    issue: Option<i64>,
    pr: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ExecutionResult {
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
}

trait GhRunner {
    fn gh(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String>;
}

struct ProcessRunner;

impl GhRunner for ProcessRunner {
    fn gh(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
        let output = Command::new("gh")
            .current_dir(repo_root)
            .args(args)
            .output()
            .map_err(|error| format!("failed to execute gh {}: {}", args.join(" "), error))?;

        Ok(ExecutionResult {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct PullRequestResponse {
    #[serde(default)]
    merged_at: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    let runner = ProcessRunner;

    match execute(&cli, &runner) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn execute(cli: &Cli, runner: &dyn GhRunner) -> Result<String, String> {
    let mut state = read_state_json(&cli.repo_root)?;
    let targets = sessions_missing_merged_at(&state.agent_sessions)?;

    if targets.is_empty() {
        return Ok("Nothing to do: no merged agent_sessions are missing merged_at.".to_string());
    }

    let mut updated = 0usize;
    let mut output_lines = Vec::with_capacity(targets.len() + 1);
    for target in targets {
        let merged_at = fetch_merged_at(&cli.repo_root, runner, target.pr)?;
        state.agent_sessions[target.index].merged_at = Some(merged_at.clone());
        output_lines.push(format!(
            "Updated {}PR #{} merged_at={}",
            target
                .issue
                .map(|issue| format!("issue #{issue} / "))
                .unwrap_or_default(),
            target.pr,
            merged_at
        ));
        updated += 1;
    }

    write_agent_sessions(&cli.repo_root, &state.agent_sessions)?;
    output_lines.push(format!(
        "Backfilled merged_at for {updated} merged agent_sessions in docs/state.json."
    ));
    Ok(output_lines.join("\n"))
}

fn read_state_json(repo_root: &Path) -> Result<StateJson, String> {
    let state_path = repo_root.join("docs/state.json");
    let state_value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(state_value)
        .map_err(|error| format!("failed to parse {}: {}", state_path.display(), error))?;
    check_version(&state)?;
    Ok(state)
}

fn sessions_missing_merged_at(sessions: &[AgentSession]) -> Result<Vec<SessionToBackfill>, String> {
    let mut missing = Vec::new();

    for (index, session) in sessions.iter().enumerate() {
        if session.status.as_deref() != Some("merged") {
            continue;
        }

        if session
            .merged_at
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
        {
            continue;
        }

        let pr = session.pr.ok_or_else(|| {
            format!(
                "{} has status merged but is missing pr",
                session_label(session.issue)
            )
        })?;
        if pr <= 0 {
            return Err(format!(
                "{} has invalid pr {}",
                session_label(session.issue),
                pr
            ));
        }

        missing.push(SessionToBackfill {
            index,
            issue: session.issue,
            pr: u64::try_from(pr)
                .map_err(|_| format!("{} has invalid pr {}", session_label(session.issue), pr))?,
        });
    }

    Ok(missing)
}

fn fetch_merged_at(repo_root: &Path, runner: &dyn GhRunner, pr: u64) -> Result<String, String> {
    let args = vec!["api".to_string(), format!("repos/{REPO}/pulls/{pr}")];
    let output = runner.gh(repo_root, &args)?;
    let exit_code = output.exit_code.unwrap_or(-1);
    if exit_code != 0 {
        let stderr = output.stderr.trim();
        return Err(format!(
            "`gh {}` failed with status {}: {}",
            args.join(" "),
            exit_code,
            if stderr.is_empty() {
                "<no stderr>"
            } else {
                stderr
            }
        ));
    }

    let response: PullRequestResponse = serde_json::from_str(&output.stdout).map_err(|error| {
        format!(
            "failed to parse JSON output from `gh {}`: {}",
            args.join(" "),
            error
        )
    })?;
    let merged_at = response
        .merged_at
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| format!("PR #{} is missing merged_at in GitHub API response", pr))?;

    DateTime::parse_from_rfc3339(merged_at)
        .map_err(|_| format!("PR #{} returned invalid merged_at {}", pr, merged_at))?;
    Ok(merged_at.to_string())
}

fn write_agent_sessions(repo_root: &Path, sessions: &[AgentSession]) -> Result<(), String> {
    let mut state_value = read_state_value(repo_root)?;
    let sessions_value = serde_json::to_value(sessions)
        .map_err(|error| format!("failed to serialize agent_sessions: {}", error))?;
    let serialized_sessions = sessions_value
        .as_array()
        .cloned()
        .ok_or_else(|| "serialized agent_sessions must be an array".to_string())?;
    let state_sessions = state_value
        .pointer_mut("/agent_sessions")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| "missing array /agent_sessions in docs/state.json".to_string())?;
    *state_sessions = serialized_sessions;
    write_state_value(repo_root, &state_value)
}

fn session_label(issue: Option<i64>) -> String {
    issue
        .map(|value| format!("issue #{value}"))
        .unwrap_or_else(|| "agent_session <unknown issue>".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[derive(Debug, Default)]
    struct MockRunner {
        responses: BTreeMap<u64, Result<String, String>>,
        calls: RefCell<Vec<u64>>,
    }

    impl GhRunner for MockRunner {
        fn gh(&self, _repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
            let pr = args
                .last()
                .and_then(|value| value.rsplit('/').next())
                .and_then(|value| value.parse::<u64>().ok())
                .ok_or_else(|| "missing pr".to_string())?;
            self.calls.borrow_mut().push(pr);

            let response = self
                .responses
                .get(&pr)
                .cloned()
                .ok_or_else(|| format!("no mock response for PR #{pr}"))?;
            match response {
                Ok(merged_at) => Ok(ExecutionResult {
                    exit_code: Some(0),
                    stdout: json!({
                        "merged_at": merged_at,
                    })
                    .to_string(),
                    stderr: String::new(),
                }),
                Err(error) => Ok(ExecutionResult {
                    exit_code: Some(1),
                    stdout: String::new(),
                    stderr: error,
                }),
            }
        }
    }

    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(prefix: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "schema-org-json-ld-{prefix}-{}-{unique}",
                std::process::id()
            ));
            fs::create_dir_all(&path).unwrap();
            Self { path }
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn identifies_sessions_missing_merged_at() {
        let sessions = vec![
            session(Some("merged"), Some(101), None),
            session(Some("merged"), Some(102), Some("2026-03-01T00:00:00Z")),
            session(Some("failed"), Some(103), None),
        ];

        let missing = sessions_missing_merged_at(&sessions).unwrap();

        assert_eq!(
            missing,
            vec![SessionToBackfill {
                index: 0,
                issue: None,
                pr: 101,
            }]
        );
    }

    #[test]
    fn existing_merged_at_sessions_are_not_modified() {
        let repo = TempDir::new("backfill-merged-at-existing");
        write_state(
            &repo.path,
            &StateJson {
                schema_version: Some(1),
                agent_sessions: vec![
                    session(Some("merged"), Some(101), None),
                    session(Some("merged"), Some(102), Some("2026-03-01T00:00:00Z")),
                ],
                ..StateJson::default()
            },
        );
        let cli = Cli {
            repo_root: repo.path.clone(),
        };
        let runner = MockRunner {
            responses: BTreeMap::from([(101, Ok("2026-03-02T00:00:00Z".to_string()))]),
            calls: RefCell::new(Vec::new()),
        };

        let output = execute(&cli, &runner).unwrap();

        assert!(output.contains("Updated PR #101 merged_at=2026-03-02T00:00:00Z"));
        assert_eq!(*runner.calls.borrow(), vec![101]);

        let state = read_state(&repo.path);
        assert_eq!(
            state.agent_sessions[0].merged_at.as_deref(),
            Some("2026-03-02T00:00:00Z")
        );
        assert_eq!(
            state.agent_sessions[1].merged_at.as_deref(),
            Some("2026-03-01T00:00:00Z")
        );
    }

    #[test]
    fn no_sessions_need_backfill_reports_nothing_to_do() {
        let repo = TempDir::new("backfill-merged-at-noop");
        write_state(
            &repo.path,
            &StateJson {
                schema_version: Some(1),
                agent_sessions: vec![session(
                    Some("merged"),
                    Some(101),
                    Some("2026-03-01T00:00:00Z"),
                )],
                ..StateJson::default()
            },
        );
        let cli = Cli {
            repo_root: repo.path.clone(),
        };
        let runner = MockRunner::default();
        let before = fs::read_to_string(repo.path.join("docs/state.json")).unwrap();

        let output = execute(&cli, &runner).unwrap();

        assert_eq!(
            output,
            "Nothing to do: no merged agent_sessions are missing merged_at."
        );
        assert!(runner.calls.borrow().is_empty());
        let after = fs::read_to_string(repo.path.join("docs/state.json")).unwrap();
        assert_eq!(before, after);
    }

    #[test]
    fn merged_session_without_pr_fails_closed() {
        let sessions = vec![session(Some("merged"), None, None)];

        let error = sessions_missing_merged_at(&sessions).unwrap_err();

        assert!(error.contains("missing pr"));
    }

    #[test]
    fn gh_failure_bubbles_up() {
        let repo = TempDir::new("backfill-merged-at-gh-error");
        write_state(
            &repo.path,
            &StateJson {
                schema_version: Some(1),
                agent_sessions: vec![session(Some("merged"), Some(101), None)],
                ..StateJson::default()
            },
        );
        let cli = Cli {
            repo_root: repo.path.clone(),
        };
        let runner = MockRunner {
            responses: BTreeMap::from([(101, Err("not found".to_string()))]),
            calls: RefCell::new(Vec::new()),
        };

        let error = execute(&cli, &runner).unwrap_err();

        assert!(error.contains("not found"));
    }

    #[test]
    fn missing_merged_at_in_gh_response_fails_closed() {
        let runner = MockRunner {
            responses: BTreeMap::from([(101, Ok(String::new()))]),
            calls: RefCell::new(Vec::new()),
        };

        let error = fetch_merged_at(Path::new("."), &runner, 101).unwrap_err();

        assert!(error.contains("missing merged_at"));
    }

    fn session(status: Option<&str>, pr: Option<i64>, merged_at: Option<&str>) -> AgentSession {
        AgentSession {
            issue: None,
            title: None,
            dispatched_at: None,
            model: None,
            status: status.map(ToOwned::to_owned),
            pr,
            merged_at: merged_at.map(ToOwned::to_owned),
            extra: Default::default(),
        }
    }

    fn write_state(repo_root: &Path, state: &StateJson) {
        let state_path = repo_root.join("docs/state.json");
        fs::create_dir_all(state_path.parent().unwrap()).unwrap();
        fs::write(
            state_path,
            format!("{}\n", serde_json::to_string_pretty(state).unwrap()),
        )
        .unwrap();
    }

    fn read_state(repo_root: &Path) -> StateJson {
        let content = fs::read_to_string(repo_root.join("docs/state.json")).unwrap();
        serde_json::from_str(&content).unwrap()
    }
}
