use clap::Parser;
use serde::Serialize;
use serde_json::Value;
use state_schema::StateJson;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "state-invariants")]
struct Cli {
    /// Path to repository root
    #[arg(long)]
    repo_root: PathBuf,

    /// Output results as JSON
    #[arg(long)]
    json: bool,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum CheckStatus {
    Pass,
    Fail,
    Warn,
}

#[derive(Serialize)]
struct CheckResult {
    name: &'static str,
    status: CheckStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

#[derive(Serialize)]
struct Report {
    checks: Vec<CheckResult>,
    passed: usize,
    failed: usize,
}

fn main() {
    let cli = Cli::parse();

    let state = match read_state_json(&cli.repo_root) {
        Ok(state) => state,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let report = run_checks(&state);

    if cli.json {
        match serde_json::to_string_pretty(&report) {
            Ok(json) => println!("{}", json),
            Err(error) => {
                eprintln!("Error: failed to serialize JSON report: {}", error);
                std::process::exit(1);
            }
        }
    } else {
        print_human_report(&report);
    }

    std::process::exit(if report.failed > 0 { 1 } else { 0 });
}

fn read_state_json(repo_root: &Path) -> Result<StateJson, String> {
    let state_path = repo_root.join("docs/state.json");
    let content = fs::read_to_string(&state_path)
        .map_err(|error| format!("failed to read {}: {}", state_path.display(), error))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("failed to parse {}: {}", state_path.display(), error))
}

fn run_checks(state: &StateJson) -> Report {
    let checks = vec![
        check_review_agent_pointer(state),
        check_copilot_metrics_math(state),
        check_blockers_narrative(state),
        check_publish_gate_consistency(state),
        check_last_cycle_consistency(state),
    ];

    let passed = checks
        .iter()
        .filter(|check| check.status == CheckStatus::Pass)
        .count();
    let failed = checks
        .iter()
        .filter(|check| check.status == CheckStatus::Fail)
        .count();

    Report {
        checks,
        passed,
        failed,
    }
}

fn check_review_agent_pointer(state: &StateJson) -> CheckResult {
    let review_agent = match state.extra.get("review_agent") {
        Some(value) => value,
        None => return warn("review_agent_pointer", "missing field: review_agent"),
    };

    let history = match review_agent.get("history").and_then(Value::as_array) {
        Some(history) => history,
        None => {
            return warn(
                "review_agent_pointer",
                "missing field: review_agent.history",
            )
        }
    };

    if history.is_empty() {
        return pass("review_agent_pointer");
    }

    let mut max_cycle: Option<i64> = None;
    for entry in history {
        let cycle = match entry.get("cycle").and_then(Value::as_i64) {
            Some(cycle) => cycle,
            None => {
                return warn(
                    "review_agent_pointer",
                    "missing field: review_agent.history[].cycle",
                )
            }
        };
        max_cycle = Some(max_cycle.map_or(cycle, |current| current.max(cycle)));
    }

    let last_review_cycle = match review_agent
        .get("last_review_cycle")
        .and_then(Value::as_i64)
    {
        Some(value) => value,
        None => {
            return warn(
                "review_agent_pointer",
                "missing field: review_agent.last_review_cycle",
            )
        }
    };

    if let Some(max_cycle) = max_cycle {
        if last_review_cycle < max_cycle {
            return fail(
                "review_agent_pointer",
                format!(
                    "last_review_cycle({}) < max history cycle({})",
                    last_review_cycle, max_cycle
                ),
            );
        }
    }

    pass("review_agent_pointer")
}

fn check_copilot_metrics_math(state: &StateJson) -> CheckResult {
    let total_dispatches = match get_metric_i64(state, "total_dispatches") {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_math",
                "missing field: copilot_metrics.total_dispatches",
            )
        }
    };
    let resolved = match get_metric_i64(state, "resolved") {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_math",
                "missing field: copilot_metrics.resolved",
            )
        }
    };
    let in_flight = match state.copilot_metrics.in_flight {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_math",
                "missing field: copilot_metrics.in_flight",
            )
        }
    };
    let produced_pr = match get_metric_i64(state, "produced_pr") {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_math",
                "missing field: copilot_metrics.produced_pr",
            )
        }
    };
    let merged = match get_metric_i64(state, "merged") {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_math",
                "missing field: copilot_metrics.merged",
            )
        }
    };
    let closed_without_merge = match get_metric_i64(state, "closed_without_merge") {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_math",
                "missing field: copilot_metrics.closed_without_merge",
            )
        }
    };
    let dispatch_to_pr_rate = match state.copilot_metrics.dispatch_to_pr_rate.as_ref() {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_math",
                "missing field: copilot_metrics.dispatch_to_pr_rate",
            )
        }
    };
    let pr_merge_rate = match state.copilot_metrics.pr_merge_rate.as_ref() {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_math",
                "missing field: copilot_metrics.pr_merge_rate",
            )
        }
    };

    let mut failures = Vec::new();
    for (name, value) in [
        ("total_dispatches", total_dispatches),
        ("resolved", resolved),
        ("in_flight", in_flight),
        ("produced_pr", produced_pr),
        ("merged", merged),
        ("closed_without_merge", closed_without_merge),
    ] {
        if value < 0 {
            failures.push(format!("{}({}) must be non-negative", name, value));
        }
    }

    if produced_pr != merged + closed_without_merge {
        failures.push(format!(
            "produced_pr({}) != merged({}) + closed({})",
            produced_pr, merged, closed_without_merge
        ));
    }

    if resolved + in_flight != total_dispatches {
        failures.push(format!(
            "resolved({}) + in_flight({}) != total_dispatches({})",
            resolved, in_flight, total_dispatches
        ));
    }

    match parse_rate(dispatch_to_pr_rate) {
        Some((n, m)) if n == produced_pr && m == resolved => {}
        Some((n, m)) => failures.push(format!(
            "dispatch_to_pr_rate({}/{}) != produced_pr({})/resolved({})",
            n, m, produced_pr, resolved
        )),
        None => failures.push(format!(
            "dispatch_to_pr_rate has invalid format: {}",
            dispatch_to_pr_rate
        )),
    }

    match parse_rate(pr_merge_rate) {
        Some((n, m)) if n == merged && m == produced_pr => {}
        Some((n, m)) => failures.push(format!(
            "pr_merge_rate({}/{}) != merged({})/produced_pr({})",
            n, m, merged, produced_pr
        )),
        None => failures.push(format!(
            "pr_merge_rate has invalid format: {}",
            pr_merge_rate
        )),
    }

    if failures.is_empty() {
        pass("copilot_metrics_math")
    } else {
        fail("copilot_metrics_math", failures.join("; "))
    }
}

fn check_blockers_narrative(state: &StateJson) -> CheckResult {
    let blockers0 = match state.blockers.first() {
        Some(value) => value,
        None => return warn("blockers_narrative", "missing field: blockers[0]"),
    };

    let checkpoint = match blockers0
        .get("pre_publish_checkpoint")
        .and_then(Value::as_str)
    {
        Some(value) => value,
        None => {
            return warn(
                "blockers_narrative",
                "missing field: blockers[0].pre_publish_checkpoint",
            )
        }
    };

    if !checkpoint.contains("ALL GATES SATISFIED") {
        return pass("blockers_narrative");
    }

    let remaining_actions = match blockers0.get("remaining_actions").and_then(Value::as_array) {
        Some(value) => value,
        None => {
            return warn(
                "blockers_narrative",
                "missing field: blockers[0].remaining_actions",
            )
        }
    };

    let has_pending = remaining_actions.iter().any(|item| {
        item.as_str()
            .map(|text| text.to_ascii_uppercase().contains("PENDING"))
            .unwrap_or(false)
    });

    if has_pending {
        fail(
            "blockers_narrative",
            "pre_publish_checkpoint says ALL GATES SATISFIED but remaining_actions contains PENDING"
                .to_string(),
        )
    } else {
        pass("blockers_narrative")
    }
}

fn check_publish_gate_consistency(state: &StateJson) -> CheckResult {
    let publish_gate = match state.extra.get("publish_gate") {
        Some(value) => value,
        None => return warn("publish_gate_consistency", "missing field: publish_gate"),
    };

    let source_diverged = match publish_gate.get("source_diverged").and_then(Value::as_bool) {
        Some(value) => value,
        None => {
            return warn(
                "publish_gate_consistency",
                "missing field: publish_gate.source_diverged",
            )
        }
    };

    if !source_diverged {
        return pass("publish_gate_consistency");
    }

    let checkpoint = match state
        .blockers
        .first()
        .and_then(|value| value.get("pre_publish_checkpoint"))
        .and_then(Value::as_str)
    {
        Some(value) => value,
        None => {
            return warn(
                "publish_gate_consistency",
                "missing field: blockers[0].pre_publish_checkpoint",
            )
        }
    };

    if checkpoint.contains("ALL GATES SATISFIED") {
        fail(
            "publish_gate_consistency",
            "publish_gate.source_diverged is true but blockers pre_publish_checkpoint says ALL GATES SATISFIED"
                .to_string(),
        )
    } else {
        pass("publish_gate_consistency")
    }
}

fn check_last_cycle_consistency(state: &StateJson) -> CheckResult {
    let last_cycle_number = match state.last_cycle.extra.get("number").and_then(Value::as_i64) {
        Some(value) => value,
        None => return warn("last_cycle_consistency", "missing field: last_cycle.number"),
    };

    if last_cycle_number <= 0 {
        return fail(
            "last_cycle_consistency",
            format!(
                "last_cycle.number({}) is not a positive integer",
                last_cycle_number
            ),
        );
    }

    let Some(history) = state
        .extra
        .get("review_agent")
        .and_then(|value| value.get("history"))
        .and_then(Value::as_array)
    else {
        return pass("last_cycle_consistency");
    };

    if history.is_empty() {
        return pass("last_cycle_consistency");
    }

    let mut max_cycle: Option<i64> = None;
    for entry in history {
        let cycle = match entry.get("cycle").and_then(Value::as_i64) {
            Some(value) => value,
            None => {
                return warn(
                    "last_cycle_consistency",
                    "missing field: review_agent.history[].cycle",
                )
            }
        };
        max_cycle = Some(max_cycle.map_or(cycle, |current| current.max(cycle)));
    }

    if let Some(max_cycle) = max_cycle {
        if max_cycle > last_cycle_number {
            return fail(
                "last_cycle_consistency",
                format!(
                    "max review_agent.history cycle({}) > last_cycle.number({})",
                    max_cycle, last_cycle_number
                ),
            );
        }
    }

    pass("last_cycle_consistency")
}

fn get_metric_i64(state: &StateJson, key: &str) -> Option<i64> {
    state.copilot_metrics.extra.get(key).and_then(Value::as_i64)
}

fn parse_rate(value: &str) -> Option<(i64, i64)> {
    let mut parts = value.split('/');
    let n = parts.next()?.parse::<i64>().ok()?;
    let m = parts.next()?.parse::<i64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((n, m))
}

fn pass(name: &'static str) -> CheckResult {
    CheckResult {
        name,
        status: CheckStatus::Pass,
        details: None,
    }
}

fn warn(name: &'static str, details: impl Into<String>) -> CheckResult {
    CheckResult {
        name,
        status: CheckStatus::Warn,
        details: Some(details.into()),
    }
}

fn fail(name: &'static str, details: impl Into<String>) -> CheckResult {
    CheckResult {
        name,
        status: CheckStatus::Fail,
        details: Some(details.into()),
    }
}

fn print_human_report(report: &Report) {
    println!("State Invariants Check");
    println!();

    let labels = [
        ("review_agent_pointer", "review_agent pointer"),
        ("copilot_metrics_math", "copilot_metrics math"),
        ("blockers_narrative", "blockers narrative"),
        ("publish_gate_consistency", "publish_gate consistency"),
        ("last_cycle_consistency", "last_cycle consistency"),
    ];

    for (index, (name, label)) in labels.iter().enumerate() {
        if let Some(check) = report.checks.iter().find(|check| check.name == *name) {
            let status = status_label(check.status);
            if let Some(details) = check.details.as_deref() {
                println!(
                    "  {}. {:<29} {} ({})",
                    index + 1,
                    format!("{}:", label),
                    status,
                    details
                );
            } else {
                println!("  {}. {:<29} {}", index + 1, format!("{}:", label), status);
            }
        }
    }

    println!();
    println!("  Passed: {}/{}", report.passed, report.checks.len());
    println!("  Failed: {}", report.failed);
}

fn status_label(status: CheckStatus) -> &'static str {
    match status {
        CheckStatus::Pass => "PASS",
        CheckStatus::Fail => "FAIL",
        CheckStatus::Warn => "WARN",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn state_from_json(value: Value) -> StateJson {
        serde_json::from_value(value).expect("state should deserialize")
    }

    fn minimal_valid_state() -> Value {
        json!({
            "schema_version": 1,
            "schema_status": {},
            "agent_sessions": [],
            "qc_processed": [],
            "qc_requests_pending": [],
            "qc_status": {},
            "blockers": [{
                "pre_publish_checkpoint": "ALL GATES SATISFIED",
                "remaining_actions": ["done"]
            }],
            "open_questions_for_eva": [],
            "eva_input_issues": {},
            "typescript_plan": {},
            "release": {},
            "copilot_metrics": {
                "in_flight": 0,
                "dispatch_to_pr_rate": "2/3",
                "pr_merge_rate": "1/2",
                "total_dispatches": 3,
                "resolved": 3,
                "produced_pr": 2,
                "merged": 1,
                "closed_without_merge": 1
            },
            "last_cycle": {
                "number": 10
            },
            "audit_processed": [],
            "test_count": {},
            "tool_pipeline": {},
            "field_inventory": {},
            "review_agent": {
                "last_review_cycle": 10,
                "history": [{"cycle": 9}, {"cycle": 10}]
            },
            "publish_gate": {
                "source_diverged": false
            }
        })
    }

    #[test]
    fn malformed_json_fails_closed() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let repo_root = std::env::temp_dir().join(format!("state-invariants-malformed-{}", run_id));
        fs::create_dir_all(repo_root.join("docs")).unwrap();
        fs::write(repo_root.join("docs/state.json"), "not valid json").unwrap();

        let error = read_state_json(&repo_root).expect_err("malformed json should error");
        assert!(error.contains("failed to parse"));
    }

    #[test]
    fn missing_field_returns_warn_not_fail() {
        let mut value = minimal_valid_state();
        value
            .as_object_mut()
            .and_then(|obj| obj.remove("review_agent"))
            .expect("review_agent should exist");

        let state = state_from_json(value);
        let check = check_review_agent_pointer(&state);
        assert_eq!(check.status, CheckStatus::Warn);
        assert!(check
            .details
            .as_deref()
            .unwrap_or_default()
            .contains("missing field"));
    }

    #[test]
    fn boundary_last_cycle_must_be_positive() {
        let mut value = minimal_valid_state();
        value["last_cycle"]["number"] = json!(0);

        let state = state_from_json(value);
        let check = check_last_cycle_consistency(&state);
        assert_eq!(check.status, CheckStatus::Fail);
        assert!(check
            .details
            .as_deref()
            .unwrap_or_default()
            .contains("positive integer"));
    }

    #[test]
    fn copilot_math_detects_mismatch() {
        let mut value = minimal_valid_state();
        value["copilot_metrics"]["produced_pr"] = json!(5);

        let state = state_from_json(value);
        let check = check_copilot_metrics_math(&state);
        assert_eq!(check.status, CheckStatus::Fail);
        assert!(check
            .details
            .as_deref()
            .unwrap_or_default()
            .contains("produced_pr"));
    }

    #[test]
    fn blockers_pending_conflict_fails() {
        let mut value = minimal_valid_state();
        value["blockers"][0]["remaining_actions"] = json!(["PENDING: do later"]);

        let state = state_from_json(value);
        let check = check_blockers_narrative(&state);
        assert_eq!(check.status, CheckStatus::Fail);
    }

    #[test]
    fn publish_gate_diverged_conflict_fails() {
        let mut value = minimal_valid_state();
        value["publish_gate"]["source_diverged"] = json!(true);

        let state = state_from_json(value);
        let check = check_publish_gate_consistency(&state);
        assert_eq!(check.status, CheckStatus::Fail);
    }
}
