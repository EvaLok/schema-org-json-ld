use chrono::DateTime;
use clap::Parser;
use serde_json::{json, Value};
use state_schema::{
    commit_state_json, current_cycle_from_state, current_utc_timestamp, read_state_value,
    set_value_at_pointer, write_state_value,
};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
enum IssueValue {
    None,
    Number(u64),
}

#[derive(Parser, Debug)]
#[command(name = "process-merge")]
struct Cli {
    /// Comma-separated list of merged PR numbers (e.g. "595,597,599")
    #[arg(long, value_delimiter = ',', num_args = 1..)]
    prs: Vec<u64>,

    /// Required comma-separated list of issue numbers matching --prs by position; use "none" when intentionally providing no issue links
    #[arg(long, required = true, value_delimiter = ',', num_args = 1.., value_parser = parse_issue_value)]
    issues: Vec<IssueValue>,

    /// Optional RFC 3339 merge timestamp to record instead of the current time
    #[arg(long, value_parser = parse_merged_at)]
    merged_at: Option<String>,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MergeUpdate {
    in_flight: i64,
    cycle: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PatchUpdate {
    path: &'static str,
    value: Value,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    if cli.prs.is_empty() {
        return Err("at least one PR number is required via --prs".to_string());
    }
    let issues = normalize_issues(&cli.issues, cli.prs.len())?;

    let mut state = read_state_value(&cli.repo_root)?;
    let merged_at = resolve_merged_at(cli.merged_at.as_deref(), current_utc_timestamp);
    let current_cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /cycle_phase/cycle or /last_cycle/number in state.json" {
            "missing numeric /cycle_phase/cycle or /last_cycle/number in docs/state.json"
                .to_string()
        } else {
            error
        }
    })?;
    update_agent_sessions(&mut state, &cli.prs, &issues, &merged_at)?;
    let update = compute_update(&state, current_cycle)?;
    let patch = build_patch(&update)?;
    apply_patch(&mut state, &patch)?;
    write_state_value(&cli.repo_root, &state)?;

    let commit_message = format!(
        "state(process-merge): {} merged [cycle {}]",
        format_pr_list(&cli.prs),
        update.cycle
    );
    let receipt = commit_state_json(&cli.repo_root, &commit_message)?;
    println!(
        "Merge processed: {}. In-flight sessions: {} (receipt: {})",
        format_pr_list(&cli.prs),
        update.in_flight,
        receipt
    );

    Ok(())
}

fn parse_issue_value(raw: &str) -> Result<IssueValue, String> {
    if raw == "none" {
        return Ok(IssueValue::None);
    }

    raw.parse::<u64>()
        .map(IssueValue::Number)
        .map_err(|_| "issues must be numeric or the literal 'none'".to_string())
}

fn parse_merged_at(raw: &str) -> Result<String, String> {
    DateTime::parse_from_rfc3339(raw)
        .map(|_| raw.to_string())
        .map_err(|_| "--merged-at must be a valid RFC 3339 timestamp".to_string())
}

fn resolve_merged_at<F>(merged_at: Option<&str>, fallback: F) -> String
where
    F: FnOnce() -> String,
{
    merged_at.map(ToOwned::to_owned).unwrap_or_else(fallback)
}

fn normalize_issues(issue_values: &[IssueValue], pr_count: usize) -> Result<Vec<u64>, String> {
    if issue_values.len() == 1 && issue_values[0] == IssueValue::None {
        return Ok(vec![]);
    }

    if issue_values.contains(&IssueValue::None) {
        return Err("--issues none cannot be combined with numeric issue values".to_string());
    }

    if issue_values.len() != pr_count {
        return Err("--issues must have the same number of values as --prs".to_string());
    }

    issue_values
        .iter()
        .map(|issue| match issue {
            IssueValue::None => {
                Err("internal error: none issue value should have been filtered".to_string())
            }
            IssueValue::Number(value) => Ok(*value),
        })
        .collect()
}

fn compute_update(state: &Value, cycle: u64) -> Result<MergeUpdate, String> {
    let sessions = state
        .pointer("/agent_sessions")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing array /agent_sessions in docs/state.json".to_string())?;
    let mut in_flight = 0_i64;
    for (index, session) in sessions.iter().enumerate() {
        match session.get("status").and_then(Value::as_str) {
            Some("in_flight") | Some("dispatched") => in_flight += 1,
            Some("merged")
            | Some("closed_without_merge")
            | Some("closed")
            | Some("closed_without_pr")
            | Some("failed")
            | Some("reviewed_awaiting_eva") => {}
            Some(status) => {
                return Err(format!(
                    "agent_sessions[{}].status has unsupported value '{}'",
                    index, status
                ))
            }
            None => return Err(format!("agent_sessions[{}].status is missing", index)),
        }
    }

    Ok(MergeUpdate { in_flight, cycle })
}

fn build_patch(update: &MergeUpdate) -> Result<Vec<PatchUpdate>, String> {
    let cycle =
        u32::try_from(update.cycle).map_err(|_| "cycle must fit in u32 range".to_string())?;
    let marker = format!("cycle {}", cycle);

    Ok(vec![
        PatchUpdate {
            path: "/in_flight_sessions",
            value: json!(update.in_flight),
        },
        PatchUpdate {
            path: "/field_inventory/fields/in_flight_sessions/last_refreshed",
            value: json!(marker),
        },
    ])
}

fn apply_patch(state: &mut Value, updates: &[PatchUpdate]) -> Result<(), String> {
    for update in updates {
        set_value_at_pointer(state, update.path, update.value.clone())?;
    }

    Ok(())
}

fn update_agent_sessions(
    state: &mut Value,
    prs: &[u64],
    issues: &[u64],
    merged_at: &str,
) -> Result<(), String> {
    let Some(sessions) = state
        .pointer_mut("/agent_sessions")
        .and_then(Value::as_array_mut)
    else {
        return Err("missing array /agent_sessions in docs/state.json".to_string());
    };

    for (index, pr) in prs.iter().enumerate() {
        let issue = issues.get(index).copied();
        let mut matched = false;
        for session in sessions.iter_mut() {
            let Some(object) = session.as_object_mut() else {
                continue;
            };

            let existing_pr = object.get("pr").and_then(Value::as_u64);
            let existing_issue = object.get("issue").and_then(Value::as_u64);

            if existing_pr == Some(*pr)
                || issue.is_some_and(|issue_number| existing_issue == Some(issue_number))
            {
                object.insert("status".to_string(), json!("merged"));
                object.insert("merged_at".to_string(), json!(merged_at));
                object.insert("pr".to_string(), json!(pr));
                matched = true;
                break;
            }
        }

        if !matched {
            match issue {
                Some(issue_number) => eprintln!(
                    "Warning: no agent_sessions entry found for PR #{} (issue #{})",
                    pr, issue_number
                ),
                None => eprintln!(
                    "Warning: no agent_sessions entry found for PR #{} (no associated issue)",
                    pr
                ),
            }
        }
    }

    Ok(())
}

fn format_pr_list(prs: &[u64]) -> String {
    let formatted: Vec<String> = prs.iter().map(|pr| format!("#{}", pr)).collect();
    if formatted.len() == 1 {
        format!("PR {}", formatted[0])
    } else {
        format!("PRs {}", formatted.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;
    use clap::CommandFactory;
    use state_schema::default_agent_model;
    use std::path::PathBuf;

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..")
    }

    fn default_test_model() -> String {
        default_agent_model(&repo_root()).expect("default model should load from config")
    }

    fn sample_state() -> Value {
        let model = default_test_model();
        json!({
            "agent_sessions": [
                {
                    "issue": 667,
                    "title": "Dispatched issue 667",
                    "dispatched_at": "2026-03-05T10:00:00Z",
                    "model": model.clone(),
                    "status": "in_flight"
                },
                {
                    "issue": 668,
                    "title": "Already linked",
                    "dispatched_at": "2026-03-05T11:00:00Z",
                    "model": model,
                    "status": "in_flight",
                    "pr": 669
                }
            ],
            "in_flight_sessions": 3,
            "last_cycle": {"number": 164},
            "field_inventory": {
                "fields": {
                    "in_flight_sessions": {"last_refreshed": "cycle 163"}
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
        assert!(help.contains("--prs"));
        assert!(help.contains("--issues"));
        assert!(help.contains("--merged-at"));
        assert!(help.contains("none"));
        assert!(help.contains("--repo-root"));
    }

    #[test]
    fn update_calculation_counts_in_flight_sessions_after_merge_updates() {
        let mut state = sample_state();
        update_agent_sessions(&mut state, &[595], &[667], "2026-03-07T13:00:00Z")
            .expect("agent sessions should update");
        let update = compute_update(&state, 164).expect("update should compute");
        assert_eq!(update.in_flight, 1);
    }

    #[test]
    fn update_calculation_handles_multiple_merged_sessions() {
        let mut state = sample_state();
        update_agent_sessions(&mut state, &[595, 669], &[667, 668], "2026-03-07T13:00:00Z")
            .expect("agent sessions should update");
        let update = compute_update(&state, 164).expect("update should compute");
        assert_eq!(update.in_flight, 0);
    }

    #[test]
    fn update_calculation_fails_closed_for_unknown_status() {
        let mut state = sample_state();
        state["agent_sessions"][0]["status"] = json!("mystery_status");
        let error = compute_update(&state, 164).expect_err("unknown status should fail");
        assert!(error.contains("unsupported value"));
    }

    #[test]
    fn patch_updates_in_flight_and_freshness_marker() {
        let update = MergeUpdate {
            in_flight: 2,
            cycle: 164,
        };
        let patch = build_patch(&update).expect("patch should build");
        assert_eq!(
            patch.iter().map(|update| update.path).collect::<Vec<_>>(),
            vec![
                "/in_flight_sessions",
                "/field_inventory/fields/in_flight_sessions/last_refreshed",
            ]
        );
        assert_eq!(patch[0].value, json!(2));
        assert_eq!(patch[1].value, json!("cycle 164"));
    }

    #[test]
    fn apply_patch_updates_in_flight_and_refreshes_field_inventory() {
        let mut state = sample_state();
        let update = MergeUpdate {
            in_flight: 2,
            cycle: 164,
        };
        let patch = build_patch(&update).expect("patch should build");

        apply_patch(&mut state, &patch).expect("patch should apply");

        assert_eq!(state["in_flight_sessions"], json!(2));
        assert_eq!(
            state["field_inventory"]["fields"]["in_flight_sessions"]["last_refreshed"],
            json!("cycle 164")
        );
    }

    #[test]
    fn format_pr_list_handles_single_and_multiple_values() {
        assert_eq!(format_pr_list(&[595]), "PR #595");
        assert_eq!(format_pr_list(&[595, 597]), "PRs #595, #597");
    }

    #[test]
    fn update_agent_sessions_matches_issue_mapping_and_sets_merge_fields() {
        let mut state = sample_state();

        update_agent_sessions(&mut state, &[668], &[667], "2026-03-07T13:00:00Z")
            .expect("agent sessions should update");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions[0]["issue"], json!(667));
        assert_eq!(sessions[0]["status"], json!("merged"));
        assert_eq!(sessions[0]["pr"], json!(668));
        assert_eq!(sessions[0]["merged_at"], json!("2026-03-07T13:00:00Z"));
    }

    #[test]
    fn update_agent_sessions_matches_existing_pr_without_issue_mapping() {
        let mut state = sample_state();

        update_agent_sessions(&mut state, &[669], &[999], "2026-03-07T13:00:00Z")
            .expect("agent sessions should update");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions[1]["status"], json!("merged"));
        assert_eq!(sessions[1]["pr"], json!(669));
        assert_eq!(sessions[1]["merged_at"], json!("2026-03-07T13:00:00Z"));
    }

    #[test]
    fn update_agent_sessions_warns_but_does_not_fail_when_mapping_is_missing() {
        let mut state = sample_state();
        let before = state["agent_sessions"].clone();

        update_agent_sessions(&mut state, &[700], &[777], "2026-03-07T13:00:00Z")
            .expect("missing session should not fail");

        assert_eq!(state["agent_sessions"], before);
    }

    #[test]
    fn cli_requires_issues_flag() {
        let error = Cli::try_parse_from(["process-merge", "--prs", "595"])
            .expect_err("--issues should be required");

        assert_eq!(error.kind(), ErrorKind::MissingRequiredArgument);
        assert!(error.to_string().contains("--issues"));
    }

    #[test]
    fn cli_accepts_none_as_empty_issue_list_and_updates_pr_only_session() {
        let cli = Cli::try_parse_from(["process-merge", "--prs", "669", "--issues", "none"])
            .expect("--issues none should parse");
        let issues = normalize_issues(&cli.issues, cli.prs.len()).expect("none should normalize");
        assert!(issues.is_empty());

        let mut state = sample_state();
        update_agent_sessions(&mut state, &cli.prs, &issues, "2026-03-07T13:00:00Z")
            .expect("empty issue mapping should still allow PR matches");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        let session = sessions[1].as_object().expect("session object");
        assert_eq!(session.get("status"), Some(&json!("merged")));
        assert_eq!(session.get("pr"), Some(&json!(669)));
        assert_eq!(
            session.get("merged_at"),
            Some(&json!("2026-03-07T13:00:00Z"))
        );
    }

    #[test]
    fn cli_accepts_numeric_issue_values() {
        let cli = Cli::try_parse_from(["process-merge", "--prs", "668", "--issues", "667"])
            .expect("numeric issues should parse");
        let issues =
            normalize_issues(&cli.issues, cli.prs.len()).expect("numeric issues should normalize");
        assert_eq!(issues, vec![667]);

        let mut state = sample_state();
        update_agent_sessions(&mut state, &cli.prs, &issues, "2026-03-07T13:00:00Z")
            .expect("numeric issue mapping should update agent sessions");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions[0]["issue"], json!(667));
        assert_eq!(sessions[0]["status"], json!("merged"));
        assert_eq!(sessions[0]["pr"], json!(668));
        assert_eq!(sessions[0]["merged_at"], json!("2026-03-07T13:00:00Z"));
    }

    #[test]
    fn cli_accepts_explicit_merged_at_timestamp() {
        let cli = Cli::try_parse_from([
            "process-merge",
            "--prs",
            "668",
            "--issues",
            "667",
            "--merged-at",
            "2026-03-07T12:34:56Z",
        ])
        .expect("merged-at should parse");

        assert_eq!(cli.merged_at.as_deref(), Some("2026-03-07T12:34:56Z"));
        assert_eq!(
            resolve_merged_at(cli.merged_at.as_deref(), || "fallback".to_string()),
            "2026-03-07T12:34:56Z"
        );
    }

    #[test]
    fn cli_rejects_invalid_merged_at_timestamp() {
        let error = Cli::try_parse_from([
            "process-merge",
            "--prs",
            "668",
            "--issues",
            "667",
            "--merged-at",
            "not-a-timestamp",
        ])
        .expect_err("invalid merged-at should be rejected");

        assert_eq!(error.kind(), ErrorKind::ValueValidation);
        assert!(error.to_string().contains("--merged-at"));
    }

    #[test]
    fn resolve_merged_at_uses_fallback_when_flag_omitted() {
        assert_eq!(
            resolve_merged_at(None, || "2026-03-08T01:02:03Z".to_string()),
            "2026-03-08T01:02:03Z"
        );
    }
}
