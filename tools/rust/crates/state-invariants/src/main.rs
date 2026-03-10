use clap::Parser;
use serde::Serialize;
use serde_json::Value;
use state_schema::StateJson;
use std::collections::{HashMap, HashSet};
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
        check_review_history_accounting(state),
        check_review_history_categories(state),
        check_copilot_metrics_rates(state),
        check_blockers_narrative(state),
        check_publish_gate_consistency(state),
        check_last_cycle_consistency(state),
        check_future_cycle_freshness(state),
        check_chronic_categories(state),
        check_agent_sessions_reconciliation(state),
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
        if last_review_cycle != max_cycle {
            return fail(
                "review_agent_pointer",
                format!(
                    "last_review_cycle({}) != max history cycle({}); pointer must match latest history entry",
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
    let mut failures = Vec::new();
    for (name, value) in [
        ("total_dispatches", total_dispatches),
        ("resolved", resolved),
        ("in_flight", in_flight),
        ("produced_pr", produced_pr),
        ("merged", merged),
    ] {
        if value < 0 {
            failures.push(format!("{}({}) must be non-negative", name, value));
        }
    }

    if produced_pr < merged {
        failures.push(format!(
            "produced_pr({}) must be >= merged({})",
            produced_pr, merged
        ));
    }

    if resolved + in_flight != total_dispatches {
        failures.push(format!(
            "resolved({}) + in_flight({}) != total_dispatches({})",
            resolved, in_flight, total_dispatches
        ));
    }

    if failures.is_empty() {
        pass("copilot_metrics_math")
    } else {
        fail("copilot_metrics_math", failures.join("; "))
    }
}

fn check_review_history_accounting(state: &StateJson) -> CheckResult {
    let history = match get_review_history(state) {
        Some(value) => value,
        None => {
            return warn(
                "review_history_accounting",
                "missing field: review_agent.history",
            )
        }
    };

    let mut failures = Vec::new();
    for (index, entry) in history.iter().enumerate() {
        let finding_count = match entry.get("finding_count").and_then(Value::as_i64) {
            Some(value) => value,
            None => {
                return warn(
                    "review_history_accounting",
                    format!(
                        "missing field: review_agent.history[{}].finding_count",
                        index
                    ),
                )
            }
        };
        let actioned = match entry.get("actioned").and_then(Value::as_i64) {
            Some(value) => value,
            None => {
                return warn(
                    "review_history_accounting",
                    format!("missing field: review_agent.history[{}].actioned", index),
                )
            }
        };
        let deferred = match entry.get("deferred").and_then(Value::as_i64) {
            Some(value) => value,
            None => {
                return warn(
                    "review_history_accounting",
                    format!("missing field: review_agent.history[{}].deferred", index),
                )
            }
        };
        let ignored = match entry.get("ignored").and_then(Value::as_i64) {
            Some(value) => value,
            None => {
                return warn(
                    "review_history_accounting",
                    format!("missing field: review_agent.history[{}].ignored", index),
                )
            }
        };
        let complacency_score = match entry.get("complacency_score").and_then(Value::as_i64) {
            Some(value) => value,
            None => {
                return warn(
                    "review_history_accounting",
                    format!(
                        "missing field: review_agent.history[{}].complacency_score",
                        index
                    ),
                )
            }
        };

        if finding_count <= 0 {
            failures.push(format!(
                "review_agent.history[{}].finding_count({}) must be > 0",
                index, finding_count
            ));
        }

        let accounted = actioned + deferred + ignored;
        if accounted != finding_count {
            failures.push(format!(
                "review_agent.history[{}] actioned({}) + deferred({}) + ignored({}) != finding_count({})",
                index, actioned, deferred, ignored, finding_count
            ));
        }

        if !(1..=5).contains(&complacency_score) {
            failures.push(format!(
                "review_agent.history[{}].complacency_score({}) must be between 1 and 5",
                index, complacency_score
            ));
        }
    }

    if failures.is_empty() {
        pass("review_history_accounting")
    } else {
        fail("review_history_accounting", failures.join("; "))
    }
}

fn check_review_history_categories(state: &StateJson) -> CheckResult {
    let history = match get_review_history(state) {
        Some(value) => value,
        None => {
            return warn(
                "review_history_categories",
                "missing field: review_agent.history",
            )
        }
    };

    let mut failures = Vec::new();
    for (entry_index, entry) in history.iter().enumerate() {
        let categories = match entry.get("categories").and_then(Value::as_array) {
            Some(value) => value,
            None => {
                failures.push(format!(
                    "review_agent.history[{}].categories must be a non-empty array",
                    entry_index
                ));
                continue;
            }
        };

        if categories.is_empty() {
            failures.push(format!(
                "review_agent.history[{}].categories must be a non-empty array",
                entry_index
            ));
            continue;
        }

        let mut seen = HashSet::new();
        for (category_index, category) in categories.iter().enumerate() {
            let category_text = match category.as_str() {
                Some(value) => value.trim(),
                None => {
                    failures.push(format!(
                        "review_agent.history[{}].categories[{}] must be a non-empty string",
                        entry_index, category_index
                    ));
                    continue;
                }
            };

            if category_text.is_empty() {
                failures.push(format!(
                    "review_agent.history[{}].categories[{}] must be a non-empty string",
                    entry_index, category_index
                ));
                continue;
            }

            if !seen.insert(category_text.to_string()) {
                failures.push(format!(
                    "review_agent.history[{}].categories has duplicate value '{}'",
                    entry_index, category_text
                ));
            }
        }
    }

    if failures.is_empty() {
        pass("review_history_categories")
    } else {
        fail("review_history_categories", failures.join("; "))
    }
}

fn check_copilot_metrics_rates(state: &StateJson) -> CheckResult {
    let resolved = match get_metric_i64(state, "resolved") {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_rates",
                "missing field: copilot_metrics.resolved",
            )
        }
    };
    let produced_pr = match get_metric_i64(state, "produced_pr") {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_rates",
                "missing field: copilot_metrics.produced_pr",
            )
        }
    };
    let merged = match get_metric_i64(state, "merged") {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_rates",
                "missing field: copilot_metrics.merged",
            )
        }
    };
    let dispatch_to_pr_rate = match state.copilot_metrics.dispatch_to_pr_rate.as_ref() {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_rates",
                "missing field: copilot_metrics.dispatch_to_pr_rate",
            )
        }
    };
    let pr_merge_rate = match state.copilot_metrics.pr_merge_rate.as_ref() {
        Some(value) => value,
        None => {
            return warn(
                "copilot_metrics_rates",
                "missing field: copilot_metrics.pr_merge_rate",
            )
        }
    };

    let mut failures = Vec::new();

    validate_rate(
        "dispatch_to_pr_rate",
        dispatch_to_pr_rate,
        produced_pr,
        resolved,
        "produced_pr",
        "resolved",
        &mut failures,
    );

    validate_rate(
        "pr_merge_rate",
        pr_merge_rate,
        merged,
        produced_pr,
        "merged",
        "produced_pr",
        &mut failures,
    );

    if failures.is_empty() {
        pass("copilot_metrics_rates")
    } else {
        fail("copilot_metrics_rates", failures.join("; "))
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

fn check_future_cycle_freshness(state: &StateJson) -> CheckResult {
    let current_cycle_number = match state.last_cycle.extra.get("number").and_then(Value::as_i64) {
        Some(value) => value,
        None => return warn("future_cycle_freshness", "missing field: last_cycle.number"),
    };

    let mut failures = Vec::new();
    for (field_name, field_value) in &state.field_inventory.fields {
        let Some(last_refreshed) = field_value.get("last_refreshed").and_then(Value::as_str) else {
            continue;
        };

        let cycle = match parse_cycle_marker(last_refreshed) {
            Some(value) => value,
            None => {
                failures.push(format!(
                    "field_inventory.fields.{}.last_refreshed has invalid format '{}'",
                    field_name, last_refreshed
                ));
                continue;
            }
        };

        if cycle > current_cycle_number {
            failures.push(format!(
                "field_inventory.fields.{}.last_refreshed({}) exceeds current cycle ({})",
                field_name, cycle, current_cycle_number
            ));
        }
    }

    if failures.is_empty() {
        pass("future_cycle_freshness")
    } else {
        fail("future_cycle_freshness", failures.join("; "))
    }
}

fn get_metric_i64(state: &StateJson, key: &str) -> Option<i64> {
    match key {
        "total_dispatches" => state.copilot_metrics.total_dispatches,
        "produced_pr" => state.copilot_metrics.produced_pr,
        "merged" => state.copilot_metrics.merged,
        "in_flight" => state.copilot_metrics.in_flight,
        _ => state.copilot_metrics.extra.get(key).and_then(Value::as_i64),
    }
}

fn get_review_history(state: &StateJson) -> Option<&Vec<Value>> {
    state
        .extra
        .get("review_agent")
        .and_then(|value| value.get("history"))
        .and_then(Value::as_array)
}

enum RateFormat {
    Ratio(i64, i64),
    Percentage(f64),
}

fn validate_rate(
    rate_name: &str,
    rate_value: &str,
    expected_numerator: i64,
    expected_denominator: i64,
    numerator_label: &str,
    denominator_label: &str,
    failures: &mut Vec<String>,
) {
    match parse_rate(rate_value) {
        Some(RateFormat::Ratio(n, m)) if n == expected_numerator && m == expected_denominator => {}
        Some(RateFormat::Ratio(n, m)) => failures.push(format!(
            "{rate_name}({n}/{m}) != {numerator_label}({expected_numerator})/{denominator_label}({expected_denominator})"
        )),
        Some(RateFormat::Percentage(value)) if (0.0..=100.0).contains(&value) => {}
        Some(RateFormat::Percentage(value)) => {
            failures.push(format!("{rate_name} percentage out of range: {value}%"))
        }
        None => failures.push(format!("{rate_name} has invalid format: {rate_value}")),
    }
}

fn parse_rate(value: &str) -> Option<RateFormat> {
    if let Some(percentage) = parse_percentage(value) {
        return Some(RateFormat::Percentage(percentage));
    }

    parse_ratio(value).map(|(numerator, denominator)| RateFormat::Ratio(numerator, denominator))
}

fn parse_percentage(value: &str) -> Option<f64> {
    let percentage = value.trim().strip_suffix('%')?.trim().parse::<f64>().ok()?;
    percentage.is_finite().then_some(percentage)
}

fn parse_ratio(value: &str) -> Option<(i64, i64)> {
    let mut parts = value.split('/');
    let n = parts.next()?.trim().parse::<i64>().ok()?;
    let m = parts.next()?.trim().parse::<i64>().ok()?;
    if parts.next().is_some() {
        return None;
    }
    Some((n, m))
}

fn parse_cycle_marker(value: &str) -> Option<i64> {
    value.strip_prefix("cycle ")?.parse::<i64>().ok()
}

fn check_chronic_categories(state: &StateJson) -> CheckResult {
    let review_agent = match state.extra.get("review_agent") {
        Some(value) => value,
        None => return warn("chronic_categories", "missing field: review_agent"),
    };

    let history = match review_agent.get("history").and_then(Value::as_array) {
        Some(history) => history,
        None => return warn("chronic_categories", "missing field: review_agent.history"),
    };

    // Take last 6 entries
    let window_size = 6;
    let window: Vec<&Value> = if history.len() > window_size {
        history[history.len() - window_size..].iter().collect()
    } else {
        history.iter().collect()
    };

    if window.len() < window_size {
        return pass("chronic_categories");
    }

    // Count category occurrences
    let mut counts: HashMap<String, usize> = HashMap::new();
    for entry in &window {
        if let Some(categories) = entry.get("categories").and_then(Value::as_array) {
            for cat in categories {
                if let Some(name) = cat.as_str() {
                    *counts.entry(name.to_string()).or_insert(0) += 1;
                }
            }
        }
    }

    // Find chronic categories (5+ in 6)
    let chronic: Vec<(&String, &usize)> = counts.iter().filter(|(_, count)| **count >= 5).collect();

    if chronic.is_empty() {
        return pass("chronic_categories");
    }

    // Check if chronic categories have corresponding response entries
    let responses = review_agent
        .get("chronic_category_responses")
        .and_then(|v| v.get("entries"))
        .and_then(Value::as_array);

    let response_categories: HashSet<String> = match responses {
        Some(entries) => entries
            .iter()
            .filter_map(|e| e.get("category").and_then(Value::as_str).map(String::from))
            .collect(),
        None => HashSet::new(),
    };

    let mut untracked: Vec<String> = Vec::new();
    for (cat, count) in &chronic {
        if !response_categories.contains(cat.as_str()) {
            untracked.push(format!("{} ({}x)", cat, count));
        }
    }

    if untracked.is_empty() {
        pass("chronic_categories")
    } else {
        fail(
            "chronic_categories",
            format!(
                "chronic categories without response entries: {}",
                untracked.join(", ")
            ),
        )
    }
}

fn check_agent_sessions_reconciliation(state: &StateJson) -> CheckResult {
    let total_dispatches = match get_metric_i64(state, "total_dispatches") {
        Some(value) => value,
        None => {
            return warn(
                "agent_sessions_reconciliation",
                "missing field: copilot_metrics.total_dispatches",
            )
        }
    };
    let merged = match get_metric_i64(state, "merged") {
        Some(value) => value,
        None => {
            return warn(
                "agent_sessions_reconciliation",
                "missing field: copilot_metrics.merged",
            )
        }
    };
    let in_flight = match state.copilot_metrics.in_flight {
        Some(value) => value,
        None => {
            return warn(
                "agent_sessions_reconciliation",
                "missing field: copilot_metrics.in_flight",
            )
        }
    };
    let resolved = match get_metric_i64(state, "resolved") {
        Some(value) => value,
        None => {
            return warn(
                "agent_sessions_reconciliation",
                "missing field: copilot_metrics.resolved",
            )
        }
    };
    let closed_without_merge = match get_metric_i64(state, "closed_without_merge") {
        Some(value) => value,
        None => {
            return warn(
                "agent_sessions_reconciliation",
                "missing field: copilot_metrics.closed_without_merge",
            )
        }
    };
    let closed_without_pr = match get_metric_i64(state, "closed_without_pr") {
        Some(value) => value,
        None => {
            return warn(
                "agent_sessions_reconciliation",
                "missing field: copilot_metrics.closed_without_pr",
            )
        }
    };
    let produced_pr = match get_metric_i64(state, "produced_pr") {
        Some(value) => value,
        None => {
            return warn(
                "agent_sessions_reconciliation",
                "missing field: copilot_metrics.produced_pr",
            )
        }
    };

    let mut merged_expected = 0;
    let mut in_flight_expected = 0;
    let mut closed_without_merge_expected = 0;
    let mut closed_without_pr_expected = 0;
    let mut produced_pr_expected = 0;
    let mut invalid_statuses = Vec::new();
    let mut in_flight_issues: HashMap<i64, usize> = HashMap::new();
    let mut terminal_issues = HashSet::new();

    for (index, session) in state.agent_sessions.iter().enumerate() {
        if session.pr.is_some() {
            produced_pr_expected += 1;
        }

        // Match derive-metrics semantics, including the legacy status aliases still present in
        // docs/state.json during the migration to canonical status names.
        match session.status.as_deref() {
            Some("merged") => {
                merged_expected += 1;
                if let Some(issue) = session.issue {
                    terminal_issues.insert(issue);
                }
            }
            Some("in_flight") | Some("dispatched") => {
                in_flight_expected += 1;
                if let Some(issue) = session.issue {
                    *in_flight_issues.entry(issue).or_insert(0) += 1;
                }
            }
            Some("closed_without_merge") | Some("closed") => {
                closed_without_merge_expected += 1;
                if let Some(issue) = session.issue {
                    terminal_issues.insert(issue);
                }
            }
            Some("closed_without_pr") | Some("failed") => {
                closed_without_pr_expected += 1;
                if let Some(issue) = session.issue {
                    terminal_issues.insert(issue);
                }
            }
            Some("reviewed_awaiting_eva") => {}
            Some(status) => invalid_statuses.push(format!(
                "agent_sessions[{}].status has unsupported value '{}'",
                index, status
            )),
            None => invalid_statuses.push(format!("agent_sessions[{}].status is missing", index)),
        }
    }

    if !invalid_statuses.is_empty() {
        return fail("agent_sessions_reconciliation", invalid_statuses.join("; "));
    }

    let total_dispatches_expected = i64::try_from(state.agent_sessions.len())
        .expect("agent_sessions length should fit within i64");
    let resolved_expected = total_dispatches_expected - in_flight_expected;

    let mut failures = Vec::new();
    let mut duplicate_in_flight_issues: Vec<i64> = in_flight_issues
        .iter()
        .filter_map(|(issue, count)| (*count > 1).then_some(*issue))
        .collect();
    duplicate_in_flight_issues.sort_unstable();
    if !duplicate_in_flight_issues.is_empty() {
        failures.push(format!(
            "duplicate in_flight sessions for issue(s): {}",
            duplicate_in_flight_issues
                .iter()
                .map(i64::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    let mut stale_terminal_in_flight_issues: Vec<i64> = in_flight_issues
        .keys()
        .copied()
        .filter(|issue| terminal_issues.contains(issue))
        .collect();
    stale_terminal_in_flight_issues.sort_unstable();
    if !stale_terminal_in_flight_issues.is_empty() {
        failures.push(format!(
            "issue(s) have both terminal and in_flight sessions: {}",
            stale_terminal_in_flight_issues
                .iter()
                .map(i64::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    for (field, expected, actual) in [
        (
            "total_dispatches",
            total_dispatches_expected,
            total_dispatches,
        ),
        ("merged", merged_expected, merged),
        ("in_flight", in_flight_expected, in_flight),
        ("resolved", resolved_expected, resolved),
        (
            "closed_without_merge",
            closed_without_merge_expected,
            closed_without_merge,
        ),
        (
            "closed_without_pr",
            closed_without_pr_expected,
            closed_without_pr,
        ),
        ("produced_pr", produced_pr_expected, produced_pr),
    ] {
        if actual != expected {
            failures.push(format!(
                "{} expected {} from agent_sessions but actual {}",
                field, expected, actual
            ));
        }
    }

    if failures.is_empty() {
        pass("agent_sessions_reconciliation")
    } else {
        fail("agent_sessions_reconciliation", failures.join("; "))
    }
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
        ("review_history_accounting", "review history accounting"),
        ("review_history_categories", "review history categories"),
        ("copilot_metrics_rates", "copilot_metrics rates"),
        ("blockers_narrative", "blockers narrative"),
        ("publish_gate_consistency", "publish_gate consistency"),
        ("last_cycle_consistency", "last_cycle consistency"),
        ("future_cycle_freshness", "future cycle freshness"),
        ("chronic_categories", "chronic categories"),
        (
            "agent_sessions_reconciliation",
            "agent_sessions reconciliation",
        ),
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
            "agent_sessions": [
                {
                    "issue": 101,
                    "status": "merged",
                    "pr": 201
                },
                {
                    "issue": 102,
                    "status": "closed",
                    "pr": 202
                },
                {
                    "issue": 103,
                    "status": "failed"
                }
            ],
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
                "closed_without_merge": 1,
                "closed_without_pr": 1
            },
            "last_cycle": {
                "number": 10
            },
            "audit_processed": [],
            "test_count": {},
            "tool_pipeline": {},
            "field_inventory": {
                "fields": {
                    "copilot_metrics": {
                        "last_refreshed": "cycle 10"
                    }
                }
            },
            "review_agent": {
                "last_review_cycle": 10,
                "history": [
                    {
                        "cycle": 9,
                        "finding_count": 3,
                        "actioned": 1,
                        "deferred": 1,
                        "ignored": 1,
                        "complacency_score": 3,
                        "categories": ["state-consistency", "tooling"]
                    },
                    {
                        "cycle": 10,
                        "finding_count": 2,
                        "actioned": 1,
                        "deferred": 1,
                        "ignored": 0,
                        "complacency_score": 2,
                        "categories": ["coverage"]
                    }
                ]
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
    fn review_pointer_ahead_of_history_fails() {
        let mut value = minimal_valid_state();
        // Set pointer to 11 but max history entry is cycle 10
        value["review_agent"]["last_review_cycle"] = json!(11);

        let state = state_from_json(value);
        let check = check_review_agent_pointer(&state);
        assert_eq!(check.status, CheckStatus::Fail);
        assert!(check
            .details
            .as_deref()
            .unwrap_or_default()
            .contains("must match latest history entry"));
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
        value["copilot_metrics"]["produced_pr"] = json!(0);

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
    fn copilot_math_allows_reviewed_awaiting_eva_sessions_with_prs() {
        let mut value = minimal_valid_state();
        value["agent_sessions"][2]["status"] = json!("reviewed_awaiting_eva");
        value["agent_sessions"][2]["pr"] = json!(203);
        value["copilot_metrics"]["produced_pr"] = json!(3);
        value["copilot_metrics"]["closed_without_pr"] = json!(0);
        value["copilot_metrics"]["reviewed_awaiting_eva"] = json!(1);

        let state = state_from_json(value);
        let check = check_copilot_metrics_math(&state);
        assert_eq!(check.status, CheckStatus::Pass);
    }

    #[test]
    fn review_history_accounting_passes_for_valid_entries() {
        let state = state_from_json(minimal_valid_state());
        let check = check_review_history_accounting(&state);
        assert_eq!(check.status, CheckStatus::Pass);
    }

    #[test]
    fn review_history_accounting_detects_totals_mismatch() {
        let mut value = minimal_valid_state();
        value["review_agent"]["history"][0]["actioned"] = json!(0);

        let state = state_from_json(value);
        let check = check_review_history_accounting(&state);
        assert_eq!(check.status, CheckStatus::Fail);
        assert!(check
            .details
            .as_deref()
            .unwrap_or_default()
            .contains("actioned"));
    }

    #[test]
    fn review_history_categories_detects_duplicates() {
        let mut value = minimal_valid_state();
        value["review_agent"]["history"][0]["categories"] =
            json!(["state-consistency", "state-consistency"]);

        let state = state_from_json(value);
        let check = check_review_history_categories(&state);
        assert_eq!(check.status, CheckStatus::Fail);
        assert!(check
            .details
            .as_deref()
            .unwrap_or_default()
            .contains("duplicate"));
    }

    #[test]
    fn copilot_metrics_rates_passes_when_rates_match() {
        let state = state_from_json(minimal_valid_state());
        let check = check_copilot_metrics_rates(&state);
        assert_eq!(check.status, CheckStatus::Pass);
    }

    #[test]
    fn copilot_metrics_rates_detects_mismatch() {
        let mut value = minimal_valid_state();
        value["copilot_metrics"]["dispatch_to_pr_rate"] = json!("3/3");

        let state = state_from_json(value);
        let check = check_copilot_metrics_rates(&state);
        assert_eq!(check.status, CheckStatus::Fail);
        assert!(check
            .details
            .as_deref()
            .unwrap_or_default()
            .contains("dispatch_to_pr_rate"));
    }

    #[test]
    fn copilot_metrics_rates_accept_percentage_format() {
        let mut value = minimal_valid_state();
        value["copilot_metrics"]["dispatch_to_pr_rate"] = json!("66.7%");
        value["copilot_metrics"]["pr_merge_rate"] = json!("50.0%");

        let state = state_from_json(value);
        let check = check_copilot_metrics_rates(&state);
        assert_eq!(check.status, CheckStatus::Pass);
    }

    #[test]
    fn get_metric_i64_prefers_promoted_fields_before_extra() {
        let mut value = minimal_valid_state();
        value["copilot_metrics"]["extra_total_dispatches"] = json!(99);
        value["copilot_metrics"]["total_dispatches"] = json!(3);
        value["copilot_metrics"]["merged"] = json!(1);

        let state = state_from_json(value);

        assert_eq!(get_metric_i64(&state, "total_dispatches"), Some(3));
        assert_eq!(get_metric_i64(&state, "merged"), Some(1));
        assert_eq!(get_metric_i64(&state, "extra_total_dispatches"), Some(99));
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

    #[test]
    fn future_cycle_freshness_fails_when_field_is_ahead_of_last_cycle() {
        let mut value = minimal_valid_state();
        value["field_inventory"]["fields"]["audit_processed"]["last_refreshed"] = json!("cycle 11");

        let state = state_from_json(value);
        let check = check_future_cycle_freshness(&state);
        assert_eq!(check.status, CheckStatus::Fail);
        assert!(check
            .details
            .as_deref()
            .unwrap_or_default()
            .contains("audit_processed"));
    }

    #[test]
    fn future_cycle_freshness_passes_when_all_fields_are_at_or_before_last_cycle() {
        let mut value = minimal_valid_state();
        value["field_inventory"]["fields"]["audit_processed"]["last_refreshed"] = json!("cycle 9");

        let state = state_from_json(value);
        let check = check_future_cycle_freshness(&state);
        assert_eq!(check.status, CheckStatus::Pass);
    }

    #[test]
    fn parse_cycle_marker_accepts_cycle_prefix_with_integer() {
        assert_eq!(parse_cycle_marker("cycle 10"), Some(10));
    }

    #[test]
    fn parse_cycle_marker_rejects_invalid_formats() {
        assert_eq!(parse_cycle_marker("10"), None);
        assert_eq!(parse_cycle_marker("cycle"), None);
        assert_eq!(parse_cycle_marker("cycle abc"), None);
    }

    #[test]
    fn chronic_categories_passes_when_no_chronic() {
        let state = state_from_json(minimal_valid_state());
        let check = check_chronic_categories(&state);
        // Only 2 history entries, so window < 6, should pass
        assert_eq!(check.status, CheckStatus::Pass);
    }

    #[test]
    fn chronic_categories_detects_untracked_chronic() {
        let mut value = minimal_valid_state();
        // Build 6 history entries with the same category
        let mut history = Vec::new();
        for i in 1..=6 {
            history.push(json!({
                "cycle": i,
                "finding_count": 2,
                "actioned": 1,
                "deferred": 0,
                "ignored": 1,
                "complacency_score": 3,
                "categories": ["state-consistency", "journal-quality"]
            }));
        }
        value["review_agent"]["history"] = json!(history);
        value["review_agent"]["last_review_cycle"] = json!(6);
        value["review_agent"]["chronic_category_responses"] = json!({
            "entries": []
        });

        let state = state_from_json(value);
        let check = check_chronic_categories(&state);
        assert_eq!(check.status, CheckStatus::Fail);
        assert!(check
            .details
            .as_deref()
            .unwrap_or_default()
            .contains("state-consistency"));
    }

    #[test]
    fn chronic_categories_passes_when_tracked() {
        let mut value = minimal_valid_state();
        let mut history = Vec::new();
        for i in 1..=6 {
            history.push(json!({
                "cycle": i,
                "finding_count": 2,
                "actioned": 1,
                "deferred": 0,
                "ignored": 1,
                "complacency_score": 3,
                "categories": ["state-consistency"]
            }));
        }
        value["review_agent"]["history"] = json!(history);
        value["review_agent"]["last_review_cycle"] = json!(6);
        value["review_agent"]["chronic_category_responses"] = json!({
            "entries": [{"category": "state-consistency", "root_cause": "test", "chosen_path": "fix"}]
        });

        let state = state_from_json(value);
        let check = check_chronic_categories(&state);
        assert_eq!(check.status, CheckStatus::Pass);
    }

    #[test]
    fn agent_sessions_reconciliation_passes_for_matching_summary() {
        let state = state_from_json(minimal_valid_state());
        let check = check_agent_sessions_reconciliation(&state);
        assert_eq!(check.status, CheckStatus::Pass);
    }

    #[test]
    fn agent_sessions_reconciliation_detects_mismatch() {
        let mut value = minimal_valid_state();
        value["copilot_metrics"]["merged"] = json!(0);
        value["copilot_metrics"]["total_dispatches"] = json!(4);

        let state = state_from_json(value);
        let check = check_agent_sessions_reconciliation(&state);
        assert_eq!(check.status, CheckStatus::Fail);

        let details = check.details.as_deref().unwrap_or_default();
        assert!(details.contains("total_dispatches"));
        assert!(details.contains("expected 3"));
        assert!(details.contains("actual 4"));
        assert!(details.contains("merged"));
        assert!(details.contains("expected 1"));
        assert!(details.contains("actual 0"));
    }

    #[test]
    fn agent_sessions_reconciliation_counts_only_non_null_prs() {
        let mut value = minimal_valid_state();
        value["agent_sessions"][1]["pr"] = Value::Null;
        value["copilot_metrics"]["produced_pr"] = json!(1);

        let state = state_from_json(value);
        let check = check_agent_sessions_reconciliation(&state);
        assert_eq!(check.status, CheckStatus::Pass);
    }

    #[test]
    fn agent_sessions_reconciliation_detects_duplicate_in_flight_issues() {
        let mut value = minimal_valid_state();
        value["agent_sessions"][0]["issue"] = json!(999);
        value["agent_sessions"][0]["status"] = json!("in_flight");
        value["agent_sessions"][1]["issue"] = json!(999);
        value["agent_sessions"][1]["status"] = json!("in_flight");
        value["copilot_metrics"]["merged"] = json!(0);
        value["copilot_metrics"]["in_flight"] = json!(2);
        value["copilot_metrics"]["resolved"] = json!(1);
        value["copilot_metrics"]["closed_without_merge"] = json!(0);
        value["copilot_metrics"]["closed_without_pr"] = json!(1);

        let state = state_from_json(value);
        let check = check_agent_sessions_reconciliation(&state);
        assert_eq!(check.status, CheckStatus::Fail);

        let details = check.details.as_deref().unwrap_or_default();
        assert!(details.contains("duplicate in_flight sessions"));
        assert!(details.contains("999"));
    }

    #[test]
    fn agent_sessions_reconciliation_detects_terminal_and_in_flight_overlap() {
        let mut value = minimal_valid_state();
        value["agent_sessions"][0]["issue"] = json!(777);
        value["agent_sessions"][1]["issue"] = json!(777);
        value["agent_sessions"][1]["status"] = json!("in_flight");
        value["copilot_metrics"]["in_flight"] = json!(1);
        value["copilot_metrics"]["resolved"] = json!(2);
        value["copilot_metrics"]["closed_without_merge"] = json!(0);

        let state = state_from_json(value);
        let check = check_agent_sessions_reconciliation(&state);
        assert_eq!(check.status, CheckStatus::Fail);

        let details = check.details.as_deref().unwrap_or_default();
        assert!(details.contains("both terminal and in_flight sessions"));
        assert!(details.contains("777"));
    }

    #[test]
    fn run_checks_includes_agent_sessions_reconciliation_as_eleventh_check() {
        let state = state_from_json(minimal_valid_state());
        let report = run_checks(&state);

        assert_eq!(report.checks.len(), 11);
        assert_eq!(
            report.checks.last().map(|check| check.name),
            Some("agent_sessions_reconciliation")
        );
    }

    #[test]
    fn agent_sessions_reconciliation_allows_reviewed_awaiting_eva_without_counting_it_in_flight() {
        let mut value = minimal_valid_state();
        value["agent_sessions"][2]["status"] = json!("reviewed_awaiting_eva");
        value["agent_sessions"][2]["pr"] = json!(203);
        value["copilot_metrics"]["in_flight"] = json!(0);
        value["copilot_metrics"]["resolved"] = json!(3);
        value["copilot_metrics"]["closed_without_pr"] = json!(0);
        value["copilot_metrics"]["produced_pr"] = json!(3);

        let state = state_from_json(value);
        let check = check_agent_sessions_reconciliation(&state);
        assert_eq!(check.status, CheckStatus::Pass);
    }
}
