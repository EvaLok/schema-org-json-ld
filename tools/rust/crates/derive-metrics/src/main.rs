use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};
use state_schema::{read_state_value, write_state_value, StateJson};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(name = "derive-metrics")]
struct Cli {
    /// Path to repository root
    #[arg(long)]
    repo_root: PathBuf,

    /// Check current copilot_metrics values against derived values
    #[arg(long, conflicts_with = "apply")]
    check: bool,

    /// Update docs/state.json copilot_metrics values with derived values
    #[arg(long, conflicts_with = "check")]
    apply: bool,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct DerivedMetrics {
    total_dispatches: i64,
    resolved: i64,
    merged: i64,
    closed_without_pr: i64,
    reviewed_awaiting_eva: i64,
    in_flight: i64,
    produced_pr: i64,
    pr_merge_rate: String,
    dispatch_to_pr_rate: String,
}

fn main() {
    let cli = Cli::parse();

    let state = match read_state_json(&cli.repo_root) {
        Ok(state) => state,
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    };

    let derived = match derive_metrics(&state) {
        Ok(metrics) => metrics,
        Err(error) => {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    };

    let mut exit_code = 0;

    if cli.check {
        let state_value = match read_state_value(&cli.repo_root) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Error: {}", error);
                process::exit(1);
            }
        };

        match collect_mismatches(&derived, &state_value) {
            Ok(mismatches) if !mismatches.is_empty() => {
                eprintln!("copilot_metrics mismatches detected:");
                for mismatch in mismatches {
                    eprintln!("- {}", mismatch);
                }
                exit_code = 1;
            }
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error: {}", error);
                process::exit(1);
            }
        }
    }

    if cli.apply {
        if let Err(error) = apply_derived_metrics_to_repo(&cli.repo_root, &derived) {
            eprintln!("Error: {}", error);
            process::exit(1);
        }
    }

    match serde_json::to_string_pretty(&derived) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            eprintln!("Error: failed to serialize derived metrics: {}", error);
            process::exit(1);
        }
    }

    process::exit(exit_code);
}

fn read_state_json(repo_root: &Path) -> Result<StateJson, String> {
    let state_path = repo_root.join("docs/state.json");
    let content = fs::read_to_string(&state_path)
        .map_err(|error| format!("failed to read {}: {}", state_path.display(), error))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("failed to parse {}: {}", state_path.display(), error))
}

fn derive_metrics(state: &StateJson) -> Result<DerivedMetrics, String> {
    let total_dispatches = i64::try_from(state.agent_sessions.len())
        .map_err(|_| "agent_sessions length should fit within i64".to_string())?;
    let mut merged = 0_i64;
    let mut closed_without_pr = 0_i64;
    let mut reviewed_awaiting_eva = 0_i64;
    let mut in_flight = 0_i64;
    let mut produced_pr = 0_i64;

    for (index, session) in state.agent_sessions.iter().enumerate() {
        if session.pr.is_some() {
            produced_pr += 1;
        }

        match session.status.as_deref() {
            Some("merged") => merged += 1,
            Some("closed_without_pr") | Some("failed") => closed_without_pr += 1,
            Some("reviewed_awaiting_eva") => reviewed_awaiting_eva += 1,
            Some("in_flight") | Some("dispatched") => in_flight += 1,
            Some("closed_without_merge") | Some("closed") => {}
            Some(status) => {
                return Err(format!(
                    "agent_sessions[{}].status has unsupported value '{}'",
                    index, status
                ))
            }
            None => return Err(format!("agent_sessions[{}].status is missing", index)),
        }
    }

    // `reviewed_awaiting_eva` and every other non in-flight status are treated as resolved for
    // copilot_metrics, matching the existing state ledger semantics.
    let resolved = total_dispatches - in_flight;

    Ok(DerivedMetrics {
        total_dispatches,
        resolved,
        merged,
        closed_without_pr,
        reviewed_awaiting_eva,
        in_flight,
        produced_pr,
        pr_merge_rate: format_percentage(merged, produced_pr),
        dispatch_to_pr_rate: format_percentage(produced_pr, total_dispatches),
    })
}

fn format_percentage(numerator: i64, denominator: i64) -> String {
    if denominator <= 0 {
        return "0.0%".to_string();
    }

    let percentage = (numerator as f64 / denominator as f64) * 100.0;
    format!("{percentage:.1}%")
}

fn collect_mismatches(
    derived: &DerivedMetrics,
    state_value: &Value,
) -> Result<Vec<String>, String> {
    let metrics = state_value
        .pointer("/copilot_metrics")
        .and_then(Value::as_object)
        .ok_or_else(|| "missing object: /copilot_metrics".to_string())?;

    let mut mismatches = Vec::new();

    compare_i64(
        metrics,
        "total_dispatches",
        derived.total_dispatches,
        &mut mismatches,
    );
    compare_i64(metrics, "resolved", derived.resolved, &mut mismatches);
    compare_i64(metrics, "merged", derived.merged, &mut mismatches);
    compare_i64(
        metrics,
        "closed_without_pr",
        derived.closed_without_pr,
        &mut mismatches,
    );
    compare_i64(
        metrics,
        "reviewed_awaiting_eva",
        derived.reviewed_awaiting_eva,
        &mut mismatches,
    );
    compare_i64(metrics, "in_flight", derived.in_flight, &mut mismatches);
    compare_i64(metrics, "produced_pr", derived.produced_pr, &mut mismatches);
    compare_rate(
        metrics,
        "pr_merge_rate",
        &derived.pr_merge_rate,
        derived.merged,
        derived.produced_pr,
        &mut mismatches,
    );
    compare_rate(
        metrics,
        "dispatch_to_pr_rate",
        &derived.dispatch_to_pr_rate,
        derived.produced_pr,
        derived.total_dispatches,
        &mut mismatches,
    );

    Ok(mismatches)
}

fn compare_i64(
    metrics: &serde_json::Map<String, Value>,
    field: &str,
    expected: i64,
    mismatches: &mut Vec<String>,
) {
    match metrics.get(field).and_then(Value::as_i64) {
        Some(actual) if actual == expected => {}
        Some(actual) => mismatches.push(format!(
            "copilot_metrics.{} expected {} but found {}",
            field, expected, actual
        )),
        None => mismatches.push(format!(
            "copilot_metrics.{} is missing or not an integer",
            field
        )),
    }
}

/// Accept both the new percentage display and the legacy `n/d` ratio format so `--check` can
/// validate the current hand-maintained state block during the migration.
fn compare_rate(
    metrics: &serde_json::Map<String, Value>,
    field: &str,
    expected_display: &str,
    expected_numerator: i64,
    expected_denominator: i64,
    mismatches: &mut Vec<String>,
) {
    let Some(actual) = metrics.get(field).and_then(Value::as_str) else {
        mismatches.push(format!(
            "copilot_metrics.{} is missing or not a string",
            field
        ));
        return;
    };

    if actual == expected_display {
        return;
    }

    if parse_ratio(actual) == Some((expected_numerator, expected_denominator)) {
        return;
    }

    mismatches.push(format!(
        "copilot_metrics.{} expected {} (or {}/{}) but found {}",
        field, expected_display, expected_numerator, expected_denominator, actual
    ));
}

fn parse_ratio(value: &str) -> Option<(i64, i64)> {
    let (numerator, denominator) = value.split_once('/')?;
    let numerator = numerator.trim().parse().ok()?;
    let denominator = denominator.trim().parse().ok()?;
    Some((numerator, denominator))
}

fn apply_derived_metrics_to_repo(repo_root: &Path, derived: &DerivedMetrics) -> Result<(), String> {
    let mut state_value = read_state_value(repo_root)?;
    apply_derived_metrics_value(&mut state_value, derived)?;
    write_state_value(repo_root, &state_value)
}

fn apply_derived_metrics_value(
    state_value: &mut Value,
    derived: &DerivedMetrics,
) -> Result<(), String> {
    let metrics = state_value
        .pointer_mut("/copilot_metrics")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "missing object: /copilot_metrics".to_string())?;

    metrics.insert(
        "total_dispatches".to_string(),
        json!(derived.total_dispatches),
    );
    metrics.insert("resolved".to_string(), json!(derived.resolved));
    metrics.insert("merged".to_string(), json!(derived.merged));
    metrics.insert(
        "closed_without_pr".to_string(),
        json!(derived.closed_without_pr),
    );
    metrics.insert(
        "reviewed_awaiting_eva".to_string(),
        json!(derived.reviewed_awaiting_eva),
    );
    metrics.insert("in_flight".to_string(), json!(derived.in_flight));
    metrics.insert("produced_pr".to_string(), json!(derived.produced_pr));
    metrics.insert("pr_merge_rate".to_string(), json!(derived.pr_merge_rate));
    metrics.insert(
        "dispatch_to_pr_rate".to_string(),
        json!(derived.dispatch_to_pr_rate),
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        apply_derived_metrics_value, collect_mismatches, derive_metrics, format_percentage,
    };
    use serde_json::json;
    use state_schema::StateJson;

    #[test]
    fn derive_metrics_counts_legacy_and_canonical_statuses() {
        let state: StateJson = serde_json::from_value(json!({
            "agent_sessions": [
                { "status": "merged", "pr": 10 },
                { "status": "closed", "pr": 11 },
                { "status": "closed_without_merge", "pr": 12 },
                { "status": "failed", "pr": null },
                { "status": "closed_without_pr", "pr": null },
                { "status": "reviewed_awaiting_eva", "pr": null },
                { "status": "in_flight", "pr": null },
                { "status": "dispatched", "pr": null }
            ]
        }))
        .expect("state fixture should parse");

        let derived = derive_metrics(&state).expect("derivation should succeed");

        assert_eq!(derived.total_dispatches, 8);
        assert_eq!(derived.resolved, 6);
        assert_eq!(derived.merged, 1);
        assert_eq!(derived.closed_without_pr, 2);
        assert_eq!(derived.reviewed_awaiting_eva, 1);
        assert_eq!(derived.in_flight, 2);
        assert_eq!(derived.produced_pr, 3);
        assert_eq!(derived.pr_merge_rate, "33.3%");
        assert_eq!(derived.dispatch_to_pr_rate, "37.5%");
    }

    #[test]
    fn derive_metrics_fails_closed_for_unknown_status() {
        let state: StateJson = serde_json::from_value(json!({
            "agent_sessions": [
                { "status": "mystery_status", "pr": null }
            ]
        }))
        .expect("state fixture should parse");

        let error = derive_metrics(&state).expect_err("unknown statuses must fail");
        assert!(error.contains("unsupported value"));
        assert!(error.contains("mystery_status"));
    }

    #[test]
    fn collect_mismatches_accepts_legacy_ratio_rate_strings() {
        let state: StateJson = serde_json::from_value(json!({
            "agent_sessions": [
                { "status": "merged", "pr": 10 },
                { "status": "closed", "pr": 11 },
                { "status": "failed", "pr": null }
            ]
        }))
        .expect("state fixture should parse");
        let derived = derive_metrics(&state).expect("derivation should succeed");

        let mismatches = collect_mismatches(
            &derived,
            &json!({
                "copilot_metrics": {
                    "total_dispatches": 3,
                    "resolved": 3,
                    "merged": 1,
                    "closed_without_pr": 1,
                    "reviewed_awaiting_eva": 0,
                    "in_flight": 0,
                    "produced_pr": 2,
                    "pr_merge_rate": "1/2",
                    "dispatch_to_pr_rate": "2/3"
                }
            }),
        )
        .expect("mismatch collection should succeed");

        assert!(mismatches.is_empty());
    }

    #[test]
    fn collect_mismatches_reports_missing_or_wrong_fields() {
        let state: StateJson = serde_json::from_value(json!({
            "agent_sessions": [
                { "status": "merged", "pr": 10 },
                { "status": "reviewed_awaiting_eva", "pr": null }
            ]
        }))
        .expect("state fixture should parse");
        let derived = derive_metrics(&state).expect("derivation should succeed");

        let mismatches = collect_mismatches(
            &derived,
            &json!({
                "copilot_metrics": {
                    "total_dispatches": 2,
                    "resolved": 2,
                    "merged": 1,
                    "closed_without_pr": 0,
                    "in_flight": 0,
                    "produced_pr": 1,
                    "pr_merge_rate": "100.0%",
                    "dispatch_to_pr_rate": "50.0%"
                }
            }),
        )
        .expect("mismatch collection should succeed");

        assert_eq!(mismatches.len(), 1);
        assert!(mismatches[0].contains("reviewed_awaiting_eva"));
    }

    #[test]
    fn apply_derived_metrics_updates_target_fields_and_preserves_other_keys() {
        let state: StateJson = serde_json::from_value(json!({
            "agent_sessions": [
                { "status": "merged", "pr": 10 },
                { "status": "closed_without_pr", "pr": null }
            ]
        }))
        .expect("state fixture should parse");
        let derived = derive_metrics(&state).expect("derivation should succeed");

        let mut value = json!({
            "copilot_metrics": {
                "closed_without_merge": 99,
                "dispatch_log_latest": "keep",
                "dispatch_to_pr_rate": "old",
                "in_flight": 9,
                "merged": 0,
                "produced_pr": 0,
                "pr_merge_rate": "old",
                "resolved": 0,
                "total_dispatches": 0
            }
        });

        apply_derived_metrics_value(&mut value, &derived).expect("apply should succeed");

        assert_eq!(value["copilot_metrics"]["total_dispatches"], json!(2));
        assert_eq!(value["copilot_metrics"]["resolved"], json!(2));
        assert_eq!(value["copilot_metrics"]["merged"], json!(1));
        assert_eq!(value["copilot_metrics"]["closed_without_pr"], json!(1));
        assert_eq!(value["copilot_metrics"]["reviewed_awaiting_eva"], json!(0));
        assert_eq!(value["copilot_metrics"]["in_flight"], json!(0));
        assert_eq!(value["copilot_metrics"]["produced_pr"], json!(1));
        assert_eq!(value["copilot_metrics"]["pr_merge_rate"], json!("100.0%"));
        assert_eq!(
            value["copilot_metrics"]["dispatch_to_pr_rate"],
            json!("50.0%")
        );
        assert_eq!(value["copilot_metrics"]["closed_without_merge"], json!(99));
        assert_eq!(
            value["copilot_metrics"]["dispatch_log_latest"],
            json!("keep")
        );
    }

    #[test]
    fn format_percentage_handles_zero_denominator() {
        assert_eq!(format_percentage(0, 0), "0.0%");
    }
}
