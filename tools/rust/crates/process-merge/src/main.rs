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

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MergeUpdate {
    merged: i64,
    produced_pr: i64,
    resolved: i64,
    in_flight: i64,
    total_dispatches: i64,
    pr_merge_rate: String,
    dispatch_to_pr_rate: String,
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
    let merged_at = current_utc_timestamp();
    let current_cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /last_cycle/number in state.json" {
            "missing numeric /last_cycle/number in docs/state.json".to_string()
        } else {
            error
        }
    })?;
    let update = compute_update(&state, current_cycle, &cli.prs)?;
    let patch = build_patch(&update)?;
    apply_patch(&mut state, &patch)?;
    update_agent_sessions(&mut state, &cli.prs, &issues, &merged_at)?;
    write_state_value(&cli.repo_root, &state)?;

    let commit_message = format!(
        "state(process-merge): {} merged [cycle {}]",
        format_pr_list(&cli.prs),
        update.cycle
    );
    let receipt = commit_state_json(&cli.repo_root, &commit_message)?;
    println!(
        "Merge processed: {}. Copilot metrics: {} dispatches, {} merged (receipt: {})",
        format_pr_list(&cli.prs),
        update.total_dispatches,
        update.merged,
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

fn get_metric_i64(state: &Value, field: &str) -> Result<i64, String> {
    state
        .pointer(&format!("/copilot_metrics/{}", field))
        .and_then(Value::as_i64)
        .ok_or_else(|| {
            format!(
                "missing numeric /copilot_metrics/{} in docs/state.json",
                field
            )
        })
}

fn compute_update(state: &Value, cycle: u64, prs: &[u64]) -> Result<MergeUpdate, String> {
    let merged = get_metric_i64(state, "merged")?;
    let resolved = get_metric_i64(state, "resolved")?;
    let in_flight = get_metric_i64(state, "in_flight")?;
    let produced_pr = get_metric_i64(state, "produced_pr")?;
    let total_dispatches = get_metric_i64(state, "total_dispatches")?;

    if in_flight < 0 {
        return Err(format!(
            "copilot_metrics.in_flight({}) must be non-negative",
            in_flight
        ));
    }
    if produced_pr < 0 || merged < 0 || resolved < 0 || total_dispatches < 0 {
        return Err("copilot metrics counters must be non-negative".to_string());
    }

    let merge_count = i64::try_from(prs.len()).map_err(|_| "PR count is too large".to_string())?;
    let resolved_increment = merge_count.min(in_flight);
    let next_in_flight = in_flight - resolved_increment;
    if resolved_increment < merge_count {
        eprintln!(
            "Warning: in_flight underflow prevented (requested {}, decremented {})",
            merge_count, resolved_increment
        );
    }

    let next_merged = merged + merge_count;
    // produced_pr is incremented for each merge to keep copilot_metrics self-consistent.
    // process-merge updates counters arithmetically (not from the agent_sessions ledger);
    // this increment ensures merged <= produced_pr after each merge operation.
    let next_produced_pr = produced_pr + merge_count;
    let next_resolved = resolved + resolved_increment;

    if next_resolved + next_in_flight != total_dispatches {
        return Err(format!(
            "invariant violated: resolved({}) + in_flight({}) != total_dispatches({})",
            next_resolved, next_in_flight, total_dispatches
        ));
    }

    Ok(MergeUpdate {
        merged: next_merged,
        produced_pr: next_produced_pr,
        resolved: next_resolved,
        in_flight: next_in_flight,
        total_dispatches,
        pr_merge_rate: format_percentage(next_merged, next_produced_pr),
        dispatch_to_pr_rate: format_percentage(next_produced_pr, total_dispatches),
        cycle,
    })
}

fn format_percentage(numerator: i64, denominator: i64) -> String {
    debug_assert!(numerator >= 0, "copilot metric numerators must be non-negative");
    debug_assert!(
        denominator >= 0,
        "copilot metric denominators must be non-negative"
    );

    if denominator == 0 {
        return "0.0%".to_string();
    }

    let percentage = (numerator as f64 / denominator as f64) * 100.0;
    format!("{percentage:.1}%")
}

fn build_patch(update: &MergeUpdate) -> Result<Vec<PatchUpdate>, String> {
    let cycle =
        u32::try_from(update.cycle).map_err(|_| "cycle must fit in u32 range".to_string())?;
    let marker = format!("cycle {}", cycle);

    Ok(vec![
        PatchUpdate {
            path: "/copilot_metrics/merged",
            value: json!(update.merged),
        },
        PatchUpdate {
            path: "/copilot_metrics/resolved",
            value: json!(update.resolved),
        },
        PatchUpdate {
            path: "/copilot_metrics/in_flight",
            value: json!(update.in_flight),
        },
        PatchUpdate {
            path: "/copilot_metrics/produced_pr",
            value: json!(update.produced_pr),
        },
        PatchUpdate {
            path: "/copilot_metrics/pr_merge_rate",
            value: json!(update.pr_merge_rate),
        },
        PatchUpdate {
            path: "/copilot_metrics/dispatch_to_pr_rate",
            value: json!(update.dispatch_to_pr_rate),
        },
        PatchUpdate {
            path: "/field_inventory/fields/copilot_metrics.in_flight/last_refreshed",
            value: json!(marker),
        },
        PatchUpdate {
            path: "/field_inventory/fields/copilot_metrics.pr_merge_rate/last_refreshed",
            value: json!(marker),
        },
        PatchUpdate {
            path: "/field_inventory/fields/copilot_metrics.dispatch_to_pr_rate/last_refreshed",
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
            "last_cycle": {"number": 164},
            "copilot_metrics": {
                "closed_without_merge": 1,
                "closed_without_pr": 1,
                "dispatch_to_pr_rate": "84/85",
                "in_flight": 3,
                "merged": 80,
                "pr_merge_rate": "80/84",
                "produced_pr": 84,
                "resolved": 82,
                "total_dispatches": 85
            },
            "field_inventory": {
                "fields": {
                    "copilot_metrics.in_flight": {"last_refreshed": "cycle 163"},
                    "copilot_metrics.pr_merge_rate": {"last_refreshed": "cycle 163"},
                    "copilot_metrics.dispatch_to_pr_rate": {"last_refreshed": "cycle 163"}
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
        assert!(help.contains("none"));
        assert!(help.contains("--repo-root"));
    }

    #[test]
    fn metric_calculation_single_pr_merge() {
        let state = sample_state();
        let update = compute_update(&state, 164, &[595]).expect("update should compute");
        assert_eq!(update.merged, 81);
        assert_eq!(update.produced_pr, 85);
        assert_eq!(update.resolved, 83);
        assert_eq!(update.in_flight, 2);
    }

    #[test]
    fn metric_calculation_multiple_pr_merge() {
        let state = sample_state();
        let update = compute_update(&state, 164, &[595, 597, 599]).expect("update should compute");
        assert_eq!(update.merged, 83);
        assert_eq!(update.produced_pr, 87);
        assert_eq!(update.resolved, 85);
        assert_eq!(update.in_flight, 0);
    }

    #[test]
    fn in_flight_underflow_protection_and_invariants() {
        let mut state = sample_state();
        state["copilot_metrics"]["in_flight"] = json!(1);
        state["copilot_metrics"]["resolved"] = json!(84);
        let update = compute_update(&state, 164, &[595, 597]).expect("update should compute");
        assert_eq!(update.merged, 82);
        assert_eq!(update.produced_pr, 86);
        assert_eq!(update.resolved, 85);
        assert_eq!(update.in_flight, 0);
        assert_eq!(update.resolved + update.in_flight, update.total_dispatches);
    }

    #[test]
    fn invariant_validation_detects_mismatch() {
        let mut state = sample_state();
        state["copilot_metrics"]["total_dispatches"] = json!(84);
        let error = compute_update(&state, 164, &[595]).expect_err("invariant should fail");
        assert!(error.contains("invariant violated"));
    }

    #[test]
    fn patch_updates_counters_rates_and_freshness_markers() {
        let state = sample_state();
        let update = compute_update(&state, 164, &[595]).expect("update should compute");
        let patch = build_patch(&update).expect("patch should build");
        assert_eq!(
            patch.iter().map(|update| update.path).collect::<Vec<_>>(),
            vec![
                "/copilot_metrics/merged",
                "/copilot_metrics/resolved",
                "/copilot_metrics/in_flight",
                "/copilot_metrics/produced_pr",
                "/copilot_metrics/pr_merge_rate",
                "/copilot_metrics/dispatch_to_pr_rate",
                "/field_inventory/fields/copilot_metrics.in_flight/last_refreshed",
                "/field_inventory/fields/copilot_metrics.pr_merge_rate/last_refreshed",
                "/field_inventory/fields/copilot_metrics.dispatch_to_pr_rate/last_refreshed",
            ]
        );
        assert_eq!(patch[3].value, json!(85));
        assert_eq!(patch[4].value, json!("95.3%"));
        assert_eq!(patch[5].value, json!("100.0%"));
        assert_eq!(patch[6].value, json!("cycle 164"));
        assert_eq!(patch[7].value, json!("cycle 164"));
        assert_eq!(patch[8].value, json!("cycle 164"));
    }

    #[test]
    fn produced_pr_invariant_uses_incremented_value() {
        let mut state = sample_state();
        state["copilot_metrics"]["produced_pr"] = json!(81);
        state["copilot_metrics"]["merged"] = json!(80);
        let update = compute_update(&state, 164, &[595]).expect("update should compute");
        assert_eq!(update.produced_pr, 82);
        assert_eq!(update.merged, 81);
    }

    #[test]
    fn apply_patch_updates_derived_metrics_and_refreshes_field_inventory() {
        let mut state = sample_state();
        let update = compute_update(&state, 164, &[595]).expect("update should compute");
        let patch = build_patch(&update).expect("patch should build");

        apply_patch(&mut state, &patch).expect("patch should apply");

        assert_eq!(state["copilot_metrics"]["merged"], json!(81));
        assert_eq!(state["copilot_metrics"]["resolved"], json!(83));
        assert_eq!(state["copilot_metrics"]["in_flight"], json!(2));
        assert_eq!(state["copilot_metrics"]["produced_pr"], json!(85));
        assert_eq!(state["copilot_metrics"]["pr_merge_rate"], json!("95.3%"));
        assert_eq!(
            state["copilot_metrics"]["dispatch_to_pr_rate"],
            json!("100.0%")
        );
        assert_eq!(
            state["field_inventory"]["fields"]["copilot_metrics.pr_merge_rate"]["last_refreshed"],
            json!("cycle 164")
        );
        assert_eq!(
            state["field_inventory"]["fields"]["copilot_metrics.dispatch_to_pr_rate"]
                ["last_refreshed"],
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
}
