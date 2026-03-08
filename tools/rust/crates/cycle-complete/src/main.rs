use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};
use state_schema::{
    commit_state_json, current_cycle_from_state, read_state_value, set_value_at_pointer,
    write_state_value, StateJson,
};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

#[derive(Debug, Parser)]
#[command(name = "cycle-complete")]
struct Cli {
    /// Path to the repository root
    #[arg(long)]
    repo_root: PathBuf,

    /// Current cycle number
    #[arg(long)]
    cycle: Option<u64>,

    /// Current cycle issue number
    #[arg(long)]
    issue: u64,

    /// Output report as JSON
    #[arg(long)]
    json: bool,

    /// Apply computed state patch updates to docs/state.json
    #[arg(long)]
    apply: bool,

    /// Reconcile an in-flight agent session as ISSUE:PR:STATUS
    #[arg(long = "reconcile", value_name = "ISSUE:PR:STATUS", value_parser = parse_reconcile_arg)]
    reconcile: Vec<ReconcileArg>,

    /// Cycle summary text for /last_cycle/summary
    #[arg(long)]
    summary: Option<String>,

    /// Commit docs/state.json after applying changes
    #[arg(long)]
    commit: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ReconcileStatus {
    Merged,
}

impl ReconcileStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Merged => "merged",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ReconcileArg {
    issue: i64,
    pr: i64,
    status: ReconcileStatus,
}

#[derive(Clone, Copy, Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
enum StepStatus {
    Pass,
    Warn,
    Ready,
    Pending,
}

#[derive(Serialize)]
struct CompletionReport {
    cycle: u64,
    issue: u64,
    timestamp: String,
    session_duration_minutes: u64,
    pipeline_check: PipelineCheckStatus,
    state_json_patch: StatePatch,
    agent_session_reconciliation: AgentSessionReconciliationReport,
    review_agent_body: String,
    completion_steps: Vec<CompletionStep>,
}

#[derive(Serialize)]
struct PipelineCheckStatus {
    status: StepStatus,
    detail: String,
}

#[derive(Serialize)]
struct StatePatch {
    updates: Vec<PatchUpdate>,
}

#[derive(Serialize)]
struct PatchUpdate {
    path: String,
    value: Value,
}

#[derive(Serialize)]
struct AgentSessionReconciliationReport {
    requested: usize,
    reconciled: Vec<ReconciledAgentSession>,
}

#[derive(Serialize)]
struct ReconciledAgentSession {
    issue: i64,
    pr: i64,
    status: String,
}

struct AgentSessionReconciliationPlan {
    patch_value: Option<Value>,
    report: AgentSessionReconciliationReport,
}

#[derive(Serialize)]
struct CompletionStep {
    index: u8,
    name: &'static str,
    status: StepStatus,
    detail: String,
}

const EVENT_DRIVEN_AUTO_REFRESH_FIELDS: &[&str] = &[
    "test_count",
    "typescript_stats",
    "schema_status.in_progress",
    "schema_status.planned_next",
    "review_agent.chronic_category_responses",
    "publish_gate",
    "review_agent",
    "pre_python_clean_cycles",
    "eva_input_issues.closed_this_cycle",
    "eva_input_issues.remaining_open",
    "typescript_plan.status",
    "copilot_metrics.dispatch_to_pr_rate",
    "copilot_metrics.in_flight",
    "copilot_metrics.pr_merge_rate",
];

fn parse_reconcile_arg(value: &str) -> Result<ReconcileArg, String> {
    let mut parts = value.split(':');
    let issue = parts
        .next()
        .ok_or_else(|| "expected ISSUE:PR:STATUS".to_string())?;
    let pr = parts
        .next()
        .ok_or_else(|| "expected ISSUE:PR:STATUS".to_string())?;
    let status = parts
        .next()
        .ok_or_else(|| "expected ISSUE:PR:STATUS".to_string())?;

    if parts.next().is_some() {
        return Err("expected ISSUE:PR:STATUS".to_string());
    }

    let issue = issue
        .parse::<i64>()
        .map_err(|_| format!("invalid issue number '{}'", issue))?;
    if issue <= 0 {
        return Err(format!("invalid issue number '{}'", issue));
    }

    let pr = pr
        .parse::<i64>()
        .map_err(|_| format!("invalid PR number '{}'", pr))?;
    if pr <= 0 {
        return Err(format!("invalid PR number '{}'", pr));
    }

    let status = match status {
        "merged" => ReconcileStatus::Merged,
        other => {
            return Err(format!(
                "unsupported reconciliation status '{}' (expected merged)",
                other
            ))
        }
    };

    Ok(ReconcileArg { issue, pr, status })
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = validate_cli_flags(&cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }

    let cycle = match cli.cycle {
        Some(cycle) => cycle,
        None => match current_cycle_from_state(&cli.repo_root) {
            Ok(cycle) => cycle,
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        },
    };

    let state = match read_state_json(&cli.repo_root) {
        Ok(state) => state,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let now = Utc::now();
    let summary = cli
        .summary
        .as_deref()
        .unwrap_or("TODO: Fill cycle summary.");
    let report = match assemble_report(cycle, cli.issue, now, &state, summary, &cli.reconcile) {
        Ok(report) => report,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    if cli.apply {
        match apply_cycle_patch(&cli.repo_root, &report.state_json_patch) {
            Ok(changed_paths) => {
                println!(
                    "Session duration: {} minutes",
                    report.session_duration_minutes
                );
                print_agent_session_reconciliation(&report.agent_session_reconciliation);
                print_patch_apply_summary(&changed_paths);
                if cli.commit {
                    let commit_message =
                        format!("state(cycle-complete): {} [cycle {}]", summary, cycle);
                    match commit_state_json(&cli.repo_root, &commit_message) {
                        Ok(sha) => println!("Committed: {}", sha),
                        Err(error) => {
                            eprintln!("Error: {}", error);
                            std::process::exit(1);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        }
        return;
    }

    if cli.json {
        match serde_json::to_string_pretty(&report) {
            Ok(output) => println!("{}", output),
            Err(error) => {
                eprintln!("Error: failed to serialize report: {}", error);
                std::process::exit(2);
            }
        }
    } else {
        print_human_report(&report);
    }
}

fn validate_cli_flags(cli: &Cli) -> Result<(), String> {
    if cli.commit && !cli.apply {
        return Err("--commit requires --apply".to_string());
    }

    Ok(())
}

fn read_state_json(repo_root: &Path) -> Result<StateJson, String> {
    let state_path = repo_root.join("docs/state.json");
    let state_value = read_state_value(repo_root)?;
    serde_json::from_value(state_value)
        .map_err(|error| format!("failed to parse {}: {}", state_path.display(), error))
}

fn assemble_report(
    cycle: u64,
    issue: u64,
    now: DateTime<Utc>,
    state: &StateJson,
    summary: &str,
    reconcile: &[ReconcileArg],
) -> Result<CompletionReport, String> {
    let timestamp = format_timestamp(now);
    let session_duration_minutes = compute_session_duration_minutes(state, now)?;
    let pipeline_check = validate_pipeline_check(state, cycle);
    let agent_session_reconciliation = build_agent_session_reconciliation(state, reconcile)?;
    let state_json_patch = build_state_patch(
        cycle,
        issue,
        &timestamp,
        session_duration_minutes,
        state,
        summary,
        &agent_session_reconciliation,
    );
    let review_agent_body = build_review_agent_body(cycle, issue, now);
    let completion_steps = build_completion_steps(&pipeline_check, &state_json_patch);

    Ok(CompletionReport {
        cycle,
        issue,
        timestamp,
        session_duration_minutes,
        pipeline_check,
        state_json_patch,
        agent_session_reconciliation: agent_session_reconciliation.report,
        review_agent_body,
        completion_steps,
    })
}

fn format_timestamp(now: DateTime<Utc>) -> String {
    now.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn compute_session_duration_minutes(state: &StateJson, now: DateTime<Utc>) -> Result<u64, String> {
    let start_timestamp = state
        .last_cycle
        .timestamp
        .as_deref()
        .ok_or_else(|| "missing docs/state.json last_cycle.timestamp".to_string())?;
    let start = DateTime::parse_from_rfc3339(start_timestamp)
        .map_err(|error| format!("invalid docs/state.json last_cycle.timestamp: {}", error))?
        .with_timezone(&Utc);

    if start > now {
        return Err("docs/state.json last_cycle.timestamp is in the future".to_string());
    }

    let elapsed_seconds = now.signed_duration_since(start).num_seconds();
    let rounded_minutes = ((elapsed_seconds + 30) / 60) as u64;
    Ok(rounded_minutes)
}

fn validate_pipeline_check(state: &StateJson, cycle: u64) -> PipelineCheckStatus {
    let last_clean_cycle = state
        .extra
        .get("pipeline_reliability")
        .and_then(|value| value.get("last_clean_cycle"))
        .and_then(Value::as_u64);
    let clean_cycle_verified = last_clean_cycle.is_some_and(|value| value >= cycle);

    if clean_cycle_verified {
        PipelineCheckStatus {
            status: StepStatus::Pass,
            detail: "verified this cycle".to_string(),
        }
    } else {
        PipelineCheckStatus {
            status: StepStatus::Warn,
            detail: "not verified this cycle".to_string(),
        }
    }
}

fn build_state_patch(
    cycle: u64,
    issue: u64,
    timestamp: &str,
    duration_minutes: u64,
    state: &StateJson,
    summary: &str,
    agent_session_reconciliation: &AgentSessionReconciliationPlan,
) -> StatePatch {
    let mut updates = vec![
        PatchUpdate {
            path: "/last_cycle/number".to_string(),
            value: json!(cycle),
        },
        PatchUpdate {
            path: "/last_cycle/issue".to_string(),
            value: json!(issue),
        },
        PatchUpdate {
            path: "/last_cycle/timestamp".to_string(),
            value: json!(timestamp),
        },
        PatchUpdate {
            path: "/last_cycle/duration_minutes".to_string(),
            value: json!(duration_minutes),
        },
        PatchUpdate {
            path: "/last_cycle/summary".to_string(),
            value: json!(summary),
        },
        PatchUpdate {
            path: "/last_eva_comment_check".to_string(),
            value: json!(timestamp),
        },
    ];

    if let Some(agent_sessions) = agent_session_reconciliation.patch_value.as_ref() {
        updates.push(PatchUpdate {
            path: "/agent_sessions".to_string(),
            value: agent_sessions.clone(),
        });
    }

    updates.extend(build_freshness_updates(cycle, &updates, state));
    StatePatch { updates }
}

fn build_agent_session_reconciliation(
    state: &StateJson,
    reconcile: &[ReconcileArg],
) -> Result<AgentSessionReconciliationPlan, String> {
    if reconcile.is_empty() {
        return Ok(AgentSessionReconciliationPlan {
            patch_value: None,
            report: AgentSessionReconciliationReport {
                requested: 0,
                reconciled: Vec::new(),
            },
        });
    }

    let mut sessions = serde_json::to_value(&state.agent_sessions)
        .map_err(|error| format!("failed to serialize agent_sessions: {}", error))?;
    prune_nulls(&mut sessions);
    let session_entries = sessions
        .as_array_mut()
        .ok_or_else(|| "agent_sessions must serialize to an array".to_string())?;
    let mut reconciled = Vec::new();

    for item in reconcile {
        let Some(session_value) = session_entries
            .iter_mut()
            .find(|entry| entry.get("issue").and_then(Value::as_i64) == Some(item.issue))
        else {
            continue;
        };

        if session_value.get("status").and_then(Value::as_str) != Some("in_flight") {
            continue;
        }

        let session = session_value
            .as_object_mut()
            .ok_or_else(|| "agent_sessions entries must be objects".to_string())?;
        session.insert("status".to_string(), json!(item.status.as_str()));
        session.insert("pr".to_string(), json!(item.pr));
        reconciled.push(ReconciledAgentSession {
            issue: item.issue,
            pr: item.pr,
            status: item.status.as_str().to_string(),
        });
    }

    Ok(AgentSessionReconciliationPlan {
        patch_value: if reconciled.is_empty() {
            None
        } else {
            Some(sessions)
        },
        report: AgentSessionReconciliationReport {
            requested: reconcile.len(),
            reconciled,
        },
    })
}

fn prune_nulls(value: &mut Value) {
    match value {
        Value::Array(values) => {
            for item in values {
                prune_nulls(item);
            }
        }
        Value::Object(map) => {
            map.retain(|_, entry| !entry.is_null());
            for entry in map.values_mut() {
                prune_nulls(entry);
            }
        }
        _ => {}
    }
}

fn apply_state_patch(state_value: &mut Value, patch: &StatePatch) -> Result<Vec<String>, String> {
    let mut changed_paths = Vec::new();
    for update in &patch.updates {
        if set_value_at_pointer(state_value, &update.path, update.value.clone())? {
            changed_paths.push(update.path.clone());
        }
    }

    Ok(changed_paths)
}

fn apply_cycle_patch(repo_root: &Path, patch: &StatePatch) -> Result<Vec<String>, String> {
    let mut state_value = read_state_value(repo_root)?;
    let changed_paths = apply_state_patch(&mut state_value, patch)?;
    write_state_value(repo_root, &state_value)?;

    Ok(changed_paths)
}

fn print_patch_apply_summary(changed_paths: &[String]) {
    println!("Applied {} state update(s).", changed_paths.len());
    if changed_paths.is_empty() {
        return;
    }

    println!("Updated paths:");
    for path in changed_paths {
        println!("- {}", path);
    }
}

fn print_agent_session_reconciliation(report: &AgentSessionReconciliationReport) {
    println!(
        "Agent session reconciliation: {} requested, {} reconciled",
        report.requested,
        report.reconciled.len()
    );
    for reconciled in &report.reconciled {
        println!(
            "- issue #{} -> PR #{} ({})",
            reconciled.issue, reconciled.pr, reconciled.status
        );
    }
}

fn build_freshness_updates(
    cycle: u64,
    updates: &[PatchUpdate],
    state: &StateJson,
) -> Vec<PatchUpdate> {
    let tracked_fields: BTreeSet<&str> = state
        .field_inventory
        .fields
        .keys()
        .map(String::as_str)
        .collect();
    let mut refreshed_fields = BTreeSet::new();

    for update in updates {
        if update.path.starts_with("/field_inventory/") {
            continue;
        }
        if let Some(field_name) = inventory_field_for_patch_path(&update.path, &tracked_fields) {
            refreshed_fields.insert(field_name.to_string());
        }
    }

    refreshed_fields.extend(build_event_driven_auto_refresh_fields(
        cycle,
        state,
        &tracked_fields,
        &refreshed_fields,
    ));

    let cycle_marker = format!("cycle {}", cycle);
    refreshed_fields
        .into_iter()
        .map(|field_name| PatchUpdate {
            path: format!("/field_inventory/fields/{}/last_refreshed", field_name),
            value: json!(cycle_marker),
        })
        .collect()
}

fn build_event_driven_auto_refresh_fields(
    cycle: u64,
    state: &StateJson,
    tracked_fields: &BTreeSet<&str>,
    refreshed_fields: &BTreeSet<String>,
) -> Vec<String> {
    EVENT_DRIVEN_AUTO_REFRESH_FIELDS
        .iter()
        .filter(|field_name| tracked_fields.contains(**field_name))
        .filter(|field_name| !refreshed_fields.contains(**field_name))
        .filter(|field_name| {
            state
                .field_inventory
                .fields
                .get(**field_name)
                .is_some_and(|entry| field_inventory_entry_needs_refresh(entry, cycle))
        })
        .map(|field_name| (*field_name).to_string())
        .collect()
}

fn field_inventory_entry_needs_refresh(entry: &Value, cycle: u64) -> bool {
    match entry
        .get("last_refreshed")
        .and_then(Value::as_str)
        .and_then(extract_cycle_number)
    {
        Some(last_refreshed_cycle) => last_refreshed_cycle < cycle,
        None => true,
    }
}

fn extract_cycle_number(value: &str) -> Option<u64> {
    let digits: String = value
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();

    if digits.is_empty() {
        None
    } else {
        digits.parse::<u64>().ok()
    }
}

/// Matches a JSON pointer patch path to the best tracked field inventory key.
///
/// The matcher tries progressively shorter dotted prefixes from the full pointer path.
/// Example: `/last_cycle/issue` tries `last_cycle.issue`, then `last_cycle`.
fn inventory_field_for_patch_path(path: &str, tracked_fields: &BTreeSet<&str>) -> Option<String> {
    let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    if segments.is_empty() || segments[0].is_empty() {
        return None;
    }

    for index in (1..=segments.len()).rev() {
        let candidate = segments[..index].join(".");
        if tracked_fields.contains(candidate.as_str()) {
            return Some(candidate);
        }
    }

    None
}

fn build_review_agent_body(cycle: u64, issue: u64, now: DateTime<Utc>) -> String {
    let date = now.format("%Y-%m-%d");
    let time = now.format("%H%M%S");
    format!(
        "## End-of-Cycle Review — Cycle {cycle}

You are a review agent dispatched at the end of orchestrator cycle {cycle} (issue [#{issue}](https://github.com/EvaLok/schema-org-json-ld/issues/{issue})).

Your job is to review the cycle's work and provide honest, critical feedback. Commit your findings as a file at `docs/reviews/cycle-{cycle}.md`. Copilot coding agents CANNOT post issue comments — your only output mode is committing files in a PR.

### What to review

1. **Recent commits on master** since the last cycle — check for:
   - Code quality issues
   - Stale or inaccurate documentation
   - Infrastructure drift (AGENTS.md, skills, checklists out of sync with practice)
   - Test coverage gaps

2. **Worklog entry** at `docs/worklog/{date}/{time}-{{name}}.md` — check for:
   - Accuracy and completeness
   - Whether \"next steps\" are actionable
   - Whether self-modifications are properly documented

3. **Journal entry** at `docs/journal/{date}.md` — check for:
   - Genuine reflection vs formulaic/boilerplate entries
   - Complacency indicators (repeating the same observations without acting on them)
   - Missing lessons from challenges encountered

4. **State.json** at `docs/state.json` — check for:
   - Stale metrics (compare file counts against actual `ls` output)
   - Field inventory cadence violations
   - Inconsistencies between state.json and reality

5. **Complacency audit** — honestly assess:
   - Is the orchestrator genuinely improving, or going through motions?
   - Are there repeated patterns that should have been automated by now?
   - Is the journal adding value or just filling space?
   - Are worklog \"next steps\" actually being followed through?

### Output format

Commit a file at `docs/reviews/cycle-{cycle}.md` containing:
- **Findings**: Numbered list of specific observations (with file paths and line numbers where relevant)
- **Recommendations**: Concrete actions for the next cycle
- **Complacency score**: 1-5 scale (1 = actively improving, 5 = going through motions)
- **Priority items**: Top 3 things the next cycle should address

Each finding MUST include a `Category: <kebab-case-name>` line immediately after the finding title. Example:

1. **Finding title here**
   Category: descriptive-kebab-case-name
   Description of the finding...

Categories must be short kebab-case identifiers (max 40 characters). Do NOT omit the Category line.

**IMPORTANT**: Do NOT attempt to post a comment on this issue. Your only output is the committed review file in your PR.
"
    )
}

fn build_completion_steps(
    pipeline_check: &PipelineCheckStatus,
    state_patch: &StatePatch,
) -> Vec<CompletionStep> {
    vec![
        CompletionStep {
            index: 1,
            name: "pipeline-check",
            status: pipeline_check.status,
            detail: pipeline_check.detail.clone(),
        },
        CompletionStep {
            index: 2,
            name: "state-json-patch",
            status: StepStatus::Ready,
            detail: format!("{} fields to update", state_patch.updates.len()),
        },
        CompletionStep {
            index: 3,
            name: "review-agent-body",
            status: StepStatus::Ready,
            detail: "generated".to_string(),
        },
        CompletionStep {
            index: 4,
            name: "worklog",
            status: StepStatus::Pending,
            detail: "manual step".to_string(),
        },
        CompletionStep {
            index: 5,
            name: "journal",
            status: StepStatus::Pending,
            detail: "manual step".to_string(),
        },
        CompletionStep {
            index: 6,
            name: "commit-push",
            status: StepStatus::Pending,
            detail: "manual step".to_string(),
        },
        CompletionStep {
            index: 7,
            name: "close-issue",
            status: StepStatus::Pending,
            detail: "manual step".to_string(),
        },
    ]
}

fn print_human_report(report: &CompletionReport) {
    println!("Cycle Completion — Cycle {}", report.cycle);
    println!(
        "Session duration: {} minutes",
        report.session_duration_minutes
    );
    println!();
    for step in &report.completion_steps {
        println!(
            "  {}. {:<19} {:<7} ({})",
            step.index,
            format!("{}:", step.name),
            status_label(step.status),
            step.detail
        );
    }
    println!();
    println!("State JSON Patch:");
    match serde_json::to_string_pretty(&report.state_json_patch) {
        Ok(json) => println!("{}", json),
        Err(error) => {
            eprintln!("Error: failed to format state patch JSON: {}", error);
            std::process::exit(2);
        }
    }
    println!();
    print_agent_session_reconciliation(&report.agent_session_reconciliation);
    println!();
    println!("Review Agent Issue Body:");
    println!("{}", report.review_agent_body);
}

fn status_label(status: StepStatus) -> &'static str {
    match status {
        StepStatus::Pass => "PASS",
        StepStatus::Warn => "WARN",
        StepStatus::Ready => "READY",
        StepStatus::Pending => "PENDING",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use std::collections::BTreeMap;

    fn fixed_now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-03-05T05:06:07Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    fn no_reconciliation() -> AgentSessionReconciliationPlan {
        AgentSessionReconciliationPlan {
            patch_value: None,
            report: AgentSessionReconciliationReport {
                requested: 0,
                reconciled: Vec::new(),
            },
        }
    }

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("Usage:"));
        assert!(help.contains("--repo-root"));
        assert!(help.contains("--cycle"));
        assert!(help.contains("--issue"));
        assert!(help.contains("--json"));
        assert!(help.contains("--apply"));
        assert!(help.contains("--reconcile"));
        assert!(help.contains("--summary"));
        assert!(help.contains("--commit"));
    }

    #[test]
    fn state_patch_contains_expected_updates() {
        let mut state = StateJson::default();
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_cycle.duration_minutes".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_eva_comment_check".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        let patch = build_state_patch(
            139,
            464,
            "2026-03-05T05:06:07Z",
            47,
            &state,
            "summary",
            &no_reconciliation(),
        );
        assert_eq!(patch.updates.len(), 9);
        assert_eq!(patch.updates[0].path, "/last_cycle/number");
        assert_eq!(patch.updates[0].value, json!(139));
        assert_eq!(patch.updates[1].path, "/last_cycle/issue");
        assert_eq!(patch.updates[1].value, json!(464));
        assert_eq!(patch.updates[2].path, "/last_cycle/timestamp");
        assert_eq!(patch.updates[2].value, json!("2026-03-05T05:06:07Z"));
        assert_eq!(patch.updates[3].path, "/last_cycle/duration_minutes");
        assert_eq!(patch.updates[3].value, json!(47));
        assert_eq!(patch.updates[4].path, "/last_cycle/summary");
        assert_eq!(patch.updates[4].value, json!("summary"));
        assert_eq!(
            patch.updates[6].path,
            "/field_inventory/fields/last_cycle/last_refreshed"
        );
        assert_eq!(patch.updates[6].value, json!("cycle 139"));
        assert_eq!(
            patch.updates[7].path,
            "/field_inventory/fields/last_cycle.duration_minutes/last_refreshed"
        );
        assert_eq!(patch.updates[7].value, json!("cycle 139"));
        assert_eq!(
            patch.updates[8].path,
            "/field_inventory/fields/last_eva_comment_check/last_refreshed"
        );
        assert_eq!(patch.updates[8].value, json!("cycle 139"));
    }

    #[test]
    fn state_patch_generates_freshness_for_modified_fields() {
        let mut state = StateJson::default();
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_cycle.duration_minutes".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_eva_comment_check".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );

        let patch = build_state_patch(
            153,
            700,
            "2026-03-06T00:00:00Z",
            47,
            &state,
            "summary",
            &no_reconciliation(),
        );
        let freshness_paths: Vec<&str> = patch
            .updates
            .iter()
            .map(|update| update.path.as_str())
            .filter(|path| path.starts_with("/field_inventory/fields/"))
            .collect();

        assert!(freshness_paths.contains(&"/field_inventory/fields/last_cycle/last_refreshed"));
        assert!(freshness_paths
            .contains(&"/field_inventory/fields/last_cycle.duration_minutes/last_refreshed"));
        assert!(freshness_paths
            .contains(&"/field_inventory/fields/last_eva_comment_check/last_refreshed"));
    }

    #[test]
    fn state_patch_deduplicates_freshness_for_repeated_field_updates() {
        let mut state = StateJson::default();
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );

        let updates = vec![
            PatchUpdate {
                path: "/last_cycle/issue".to_string(),
                value: json!(1),
            },
            PatchUpdate {
                path: "/last_cycle/timestamp".to_string(),
                value: json!("2026-03-06T00:00:00Z"),
            },
            PatchUpdate {
                path: "/last_cycle/duration_minutes".to_string(),
                value: json!(47),
            },
            PatchUpdate {
                path: "/last_cycle/summary".to_string(),
                value: json!("summary"),
            },
        ];

        let freshness_updates = build_freshness_updates(153, &updates, &state);
        assert_eq!(freshness_updates.len(), 1);
        assert_eq!(
            freshness_updates[0].path,
            "/field_inventory/fields/last_cycle/last_refreshed"
        );
    }

    #[test]
    fn state_patch_auto_refreshes_stale_event_driven_fields() {
        let mut state = StateJson::default();
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_cycle.duration_minutes".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_eva_comment_check".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );

        for field_name in EVENT_DRIVEN_AUTO_REFRESH_FIELDS {
            state.field_inventory.fields.insert(
                (*field_name).to_string(),
                json!({"last_refreshed": "cycle 120"}),
            );
        }

        let patch = build_state_patch(
            153,
            700,
            "2026-03-06T00:00:00Z",
            47,
            &state,
            "summary",
            &no_reconciliation(),
        );
        let freshness_paths: Vec<&str> = patch
            .updates
            .iter()
            .map(|update| update.path.as_str())
            .collect();

        for field_name in EVENT_DRIVEN_AUTO_REFRESH_FIELDS {
            let expected_path = format!("/field_inventory/fields/{}/last_refreshed", field_name);
            assert!(
                freshness_paths.contains(&expected_path.as_str()),
                "missing auto-refresh for {}",
                field_name
            );
        }
    }

    #[test]
    fn state_patch_skips_current_or_missing_event_driven_fields() {
        let mut state = StateJson::default();
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_cycle.duration_minutes".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_eva_comment_check".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "publish_gate".to_string(),
            json!({"last_refreshed": "cycle 153"}),
        );
        state.field_inventory.fields.insert(
            "review_agent".to_string(),
            json!({"last_refreshed": "cycle 154"}),
        );

        let patch = build_state_patch(
            153,
            700,
            "2026-03-06T00:00:00Z",
            47,
            &state,
            "summary",
            &no_reconciliation(),
        );
        let freshness_paths: Vec<&str> = patch
            .updates
            .iter()
            .map(|update| update.path.as_str())
            .collect();

        assert!(!freshness_paths.contains(&"/field_inventory/fields/publish_gate/last_refreshed"));
        assert!(!freshness_paths.contains(&"/field_inventory/fields/review_agent/last_refreshed"));
        assert!(!freshness_paths
            .contains(&"/field_inventory/fields/pre_python_clean_cycles/last_refreshed"));
    }

    #[test]
    fn event_driven_auto_refresh_fields_includes_required_field_names() {
        // Semantic test: asserts that specific fields are in EVENT_DRIVEN_AUTO_REFRESH_FIELDS.
        // This catches omissions that the iteration-based tests cannot detect,
        // since those tests only verify "whatever is in the constant gets refreshed."
        let required_fields = [
            "test_count",
            "typescript_stats",
            "schema_status.in_progress",
            "schema_status.planned_next",
            "publish_gate",
            "review_agent",
            "pre_python_clean_cycles",
            "eva_input_issues.closed_this_cycle",
            "eva_input_issues.remaining_open",
            "typescript_plan.status",
            "copilot_metrics.dispatch_to_pr_rate",
            "copilot_metrics.in_flight",
            "copilot_metrics.pr_merge_rate",
        ];
        for field in &required_fields {
            assert!(
                EVENT_DRIVEN_AUTO_REFRESH_FIELDS.contains(field),
                "EVENT_DRIVEN_AUTO_REFRESH_FIELDS is missing required field: {}",
                field
            );
        }
    }

    #[test]
    fn summary_flag_overrides_placeholder_text_in_patch() {
        let state = StateJson::default();
        let patch = build_state_patch(
            153,
            700,
            "2026-03-06T00:00:00Z",
            47,
            &state,
            "custom summary",
            &no_reconciliation(),
        );
        assert_eq!(patch.updates[4].path, "/last_cycle/summary");
        assert_eq!(patch.updates[4].value, json!("custom summary"));
    }

    #[test]
    fn apply_state_patch_applies_all_updates_and_freshness_markers() {
        let mut state = StateJson::default();
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_cycle.duration_minutes".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_eva_comment_check".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "publish_gate".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "review_agent".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );

        let patch = build_state_patch(
            153,
            700,
            "2026-03-06T00:00:00Z",
            47,
            &state,
            "custom summary",
            &no_reconciliation(),
        );
        let mut raw_state = json!({
            "last_cycle": {
                "number": 120,
                "issue": 100,
                "timestamp": "2026-02-01T00:00:00Z",
                "duration_minutes": 15,
                "summary": "old summary"
            },
            "last_eva_comment_check": "2026-02-01T00:00:00Z",
            "publish_gate": {
                "ready": true
            },
            "review_agent": {
                "total_reviews_processed": 12
            },
            "field_inventory": {
                "fields": {
                    "last_cycle": {"last_refreshed": "cycle 120"},
                    "last_cycle.duration_minutes": {"last_refreshed": "cycle 120"},
                    "last_eva_comment_check": {"last_refreshed": "cycle 120"},
                    "publish_gate": {"last_refreshed": "cycle 120"},
                    "review_agent": {"last_refreshed": "cycle 120"}
                }
            }
        });

        let changed_paths =
            apply_state_patch(&mut raw_state, &patch).expect("state patch should apply cleanly");
        assert_eq!(changed_paths.len(), 11);
        assert_eq!(
            raw_state
                .pointer("/last_cycle/number")
                .and_then(Value::as_u64),
            Some(153)
        );
        assert_eq!(
            raw_state
                .pointer("/last_cycle/issue")
                .and_then(Value::as_u64),
            Some(700)
        );
        assert_eq!(
            raw_state
                .pointer("/last_cycle/duration_minutes")
                .and_then(Value::as_u64),
            Some(47)
        );
        assert_eq!(
            raw_state
                .pointer("/last_cycle/summary")
                .and_then(Value::as_str),
            Some("custom summary")
        );
        assert_eq!(
            raw_state
                .pointer("/field_inventory/fields/last_cycle/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 153")
        );
        assert_eq!(
            raw_state
                .pointer("/field_inventory/fields/last_cycle.duration_minutes/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 153")
        );
        assert_eq!(
            raw_state
                .pointer("/field_inventory/fields/last_eva_comment_check/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 153")
        );
        assert_eq!(
            raw_state
                .pointer("/field_inventory/fields/publish_gate/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 153")
        );
        assert_eq!(
            raw_state
                .pointer("/field_inventory/fields/review_agent/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 153")
        );
    }

    #[test]
    fn commit_without_apply_produces_error() {
        let cli = Cli::parse_from([
            "cycle-complete",
            "--repo-root",
            ".",
            "--cycle",
            "162",
            "--issue",
            "585",
            "--commit",
        ]);
        let error = validate_cli_flags(&cli).expect_err("commit without apply must fail");
        assert_eq!(error, "--commit requires --apply");
    }

    #[test]
    fn apply_flag_is_recognized_by_parser() {
        let parsed = Cli::try_parse_from([
            "cycle-complete",
            "--repo-root",
            ".",
            "--cycle",
            "162",
            "--issue",
            "585",
            "--apply",
        ]);
        assert!(parsed.is_ok());
    }

    #[test]
    fn reconcile_flag_updates_in_flight_agent_sessions() {
        let mut state = StateJson::default();
        state.last_cycle.timestamp = Some("2026-03-05T04:19:37Z".to_string());
        state.agent_sessions = serde_json::from_value(json!([
            {
                "issue": 751,
                "status": "in_flight",
                "title": "example"
            }
        ]))
        .unwrap();
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_cycle.duration_minutes".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_eva_comment_check".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "agent_sessions".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );

        let report = assemble_report(
            139,
            464,
            fixed_now(),
            &state,
            "summary",
            &[ReconcileArg {
                issue: 751,
                pr: 752,
                status: ReconcileStatus::Merged,
            }],
        )
        .unwrap();

        assert_eq!(report.agent_session_reconciliation.reconciled.len(), 1);
        assert_eq!(report.agent_session_reconciliation.reconciled[0].issue, 751);
        assert_eq!(report.agent_session_reconciliation.reconciled[0].pr, 752);
        assert_eq!(
            report
                .state_json_patch
                .updates
                .iter()
                .find(|update| update.path == "/agent_sessions")
                .map(|update| update.value.clone()),
            Some(json!([
                {
                    "issue": 751,
                    "status": "merged",
                    "title": "example",
                    "pr": 752
                }
            ]))
        );
    }

    #[test]
    fn reconcile_defaults_to_noop_when_no_args_are_provided() {
        let mut state = StateJson::default();
        state.last_cycle.timestamp = Some("2026-03-05T04:19:37Z".to_string());
        state.agent_sessions = serde_json::from_value(json!([
            {
                "issue": 751,
                "status": "in_flight",
                "title": "example"
            }
        ]))
        .unwrap();

        let report = assemble_report(139, 464, fixed_now(), &state, "summary", &[]).unwrap();

        assert_eq!(report.agent_session_reconciliation.reconciled.len(), 0);
        assert!(report
            .state_json_patch
            .updates
            .iter()
            .all(|update| update.path != "/agent_sessions"));
    }

    #[test]
    fn reconcile_parser_rejects_invalid_issue_numbers() {
        let parsed = Cli::try_parse_from([
            "cycle-complete",
            "--repo-root",
            ".",
            "--issue",
            "585",
            "--reconcile",
            "not-a-number:752:merged",
        ]);
        let error = parsed.expect_err("invalid issue number should fail");
        let rendered = error.to_string();
        assert!(rendered.contains("--reconcile"));
        assert!(rendered.contains("issue"));
    }

    #[test]
    fn cli_accepts_missing_cycle_argument() {
        let cli =
            Cli::try_parse_from(["cycle-complete", "--repo-root", ".", "--issue", "585"]).unwrap();
        assert_eq!(cli.repo_root, PathBuf::from("."));
        assert_eq!(cli.cycle, None);
        assert_eq!(cli.issue, 585);
    }

    #[test]
    fn review_agent_body_fills_cycle_issue_and_paths() {
        let body = build_review_agent_body(139, 464, fixed_now());
        assert!(body.contains("Cycle 139"));
        assert!(body.contains("[#464](https://github.com/EvaLok/schema-org-json-ld/issues/464)"));
        assert!(body.contains("docs/worklog/2026-03-05/050607-{name}.md"));
        assert!(body.contains("docs/journal/2026-03-05.md"));
        assert!(!body.contains("{N}"));
        assert!(!body.contains("{issue}"));
        assert!(!body.contains("{date}"));
        assert!(!body.contains("{time}"));
    }

    #[test]
    fn review_agent_body_enforces_file_based_delivery_policy() {
        let body = build_review_agent_body(139, 464, fixed_now());
        assert!(body.contains("docs/reviews/cycle-"));
        assert!(body.contains("Commit your findings"));
        assert!(body.contains("Each finding MUST include a `Category: <kebab-case-name>` line"));
        assert!(body.contains("Category: descriptive-kebab-case-name"));
        assert!(body.contains("Do NOT omit the Category line."));
        assert!(!body.contains("Post your findings as a comment"));
    }

    #[test]
    fn pipeline_validation_warns_when_not_verified() {
        let state = StateJson::default();
        let status = validate_pipeline_check(&state, 139);
        assert_eq!(status.status, StepStatus::Warn);
        assert_eq!(status.detail, "not verified this cycle");
    }

    #[test]
    fn json_report_serializes_to_valid_json() {
        let mut state = StateJson::default();
        let mut extra = BTreeMap::new();
        extra.insert(
            "pipeline_reliability".to_string(),
            json!({"last_clean_cycle": 139}),
        );
        state.extra = extra;
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.last_cycle.timestamp = Some("2026-03-05T04:19:37Z".to_string());
        state.field_inventory.fields.insert(
            "last_cycle.duration_minutes".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_eva_comment_check".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );

        let report = assemble_report(139, 464, fixed_now(), &state, "summary", &[]).unwrap();
        let output = serde_json::to_string_pretty(&report).unwrap();
        let parsed: Value = serde_json::from_str(&output).unwrap();

        assert_eq!(parsed.get("cycle"), Some(&json!(139)));
        assert_eq!(parsed.get("issue"), Some(&json!(464)));
        assert_eq!(parsed.get("session_duration_minutes"), Some(&json!(47)));
        assert_eq!(
            parsed
                .pointer("/pipeline_check/status")
                .and_then(Value::as_str),
            Some("pass")
        );
        assert_eq!(
            parsed
                .pointer("/completion_steps/1/detail")
                .and_then(Value::as_str),
            Some("9 fields to update")
        );
    }

    #[test]
    fn compute_session_duration_minutes_rounds_to_nearest_minute() {
        let mut state = StateJson::default();
        state.last_cycle.timestamp = Some("2026-03-05T04:19:37Z".to_string());

        let minutes = compute_session_duration_minutes(&state, fixed_now()).unwrap();

        assert_eq!(minutes, 47);
    }

    #[test]
    fn assemble_report_includes_session_duration_in_patch_and_output() {
        let mut state = StateJson::default();
        state.last_cycle.timestamp = Some("2026-03-05T04:19:37Z".to_string());
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_cycle.duration_minutes".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_eva_comment_check".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );

        let report = assemble_report(139, 464, fixed_now(), &state, "summary", &[]).unwrap();

        assert_eq!(report.session_duration_minutes, 47);
        assert_eq!(
            report.state_json_patch.updates[3].path,
            "/last_cycle/duration_minutes"
        );
        assert_eq!(report.state_json_patch.updates[3].value, json!(47));
    }
}
