use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};
use state_schema::{
    commit_state_json, current_cycle_from_state, read_state_value, set_value_at_pointer,
    transition_cycle_phase, write_state_value, StateJson,
};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct CycleChanges {
    changed_paths: BTreeSet<String>,
}

#[derive(Serialize)]
struct CompletionStep {
    index: u8,
    name: &'static str,
    status: StepStatus,
    detail: String,
}

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
    let cycle_changes = match collect_cycle_changes(&cli.repo_root, cycle) {
        Ok(changes) => changes,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let now = Utc::now();
    let summary = match resolve_summary(cli.summary.as_deref(), &state, now) {
        Ok(summary) => summary,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };
    let report = match assemble_report(
        cycle,
        cli.issue,
        now,
        &state,
        &cycle_changes,
        &summary,
        &cli.reconcile,
    ) {
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

fn resolve_summary(
    provided_summary: Option<&str>,
    state: &StateJson,
    now: DateTime<Utc>,
) -> Result<String, String> {
    if let Some(summary) = provided_summary {
        return Ok(summary.to_string());
    }

    derive_cycle_summary(state, now)
}

fn read_state_json(repo_root: &Path) -> Result<StateJson, String> {
    let state_path = repo_root.join("docs/state.json");
    let state_value = read_state_value(repo_root)?;
    serde_json::from_value(state_value)
        .map_err(|error| format!("failed to parse {}: {}", state_path.display(), error))
}

fn derive_cycle_summary(state: &StateJson, now: DateTime<Utc>) -> Result<String, String> {
    let cycle_start = cycle_window_start(state)?;
    if cycle_start > now {
        return Err("cycle summary window start is in the future".to_string());
    }

    let mut dispatch_count = 0usize;
    let mut merged_prs = BTreeSet::new();

    for session in &state.agent_sessions {
        if let Some(dispatched_at) = session.dispatched_at.as_deref() {
            let dispatched_at = parse_timestamp(
                dispatched_at,
                &format!(
                    "agent_sessions issue {} dispatched_at",
                    session.issue.unwrap_or_default()
                ),
            )?;
            if timestamp_in_cycle_window(dispatched_at, cycle_start, now) {
                dispatch_count += 1;
            }
        }

        if session.status.as_deref() != Some("merged") {
            continue;
        }

        let Some(merged_at) = session.merged_at.as_deref() else {
            continue;
        };
        let merged_at = parse_timestamp(
            merged_at,
            &format!(
                "agent_sessions issue {} merged_at",
                session.issue.unwrap_or_default()
            ),
        )?;
        if !timestamp_in_cycle_window(merged_at, cycle_start, now) {
            continue;
        }

        let pr = session.pr.ok_or_else(|| {
            format!(
                "agent_sessions issue {} merged this cycle is missing pr",
                session.issue.unwrap_or_default()
            )
        })?;
        if pr <= 0 {
            return Err(format!(
                "agent_sessions issue {} has invalid pr {}",
                session.issue.unwrap_or_default(),
                pr
            ));
        }
        merged_prs.insert(pr);
    }

    if merged_prs.is_empty() {
        return Ok(format!("{dispatch_count} dispatches, 0 merges"));
    }

    let merged_list = merged_prs
        .iter()
        .map(|pr| format!("PR #{pr}"))
        .collect::<Vec<_>>()
        .join(", ");
    Ok(format!(
        "{dispatch_count} dispatches, {} merges ({merged_list})",
        merged_prs.len()
    ))
}

fn cycle_window_start(state: &StateJson) -> Result<DateTime<Utc>, String> {
    let start = state
        .cycle_phase
        .phase_entered_at
        .as_deref()
        .or(state.last_cycle.timestamp.as_deref())
        .ok_or_else(|| {
            "missing docs/state.json cycle_phase.phase_entered_at and last_cycle.timestamp"
                .to_string()
        })?;
    parse_timestamp(start, "cycle summary window start")
}

fn parse_timestamp(value: &str, field_name: &str) -> Result<DateTime<Utc>, String> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|error| format!("invalid {field_name}: {error}"))
}

fn timestamp_in_cycle_window(
    timestamp: DateTime<Utc>,
    cycle_start: DateTime<Utc>,
    now: DateTime<Utc>,
) -> bool {
    timestamp >= cycle_start && timestamp <= now
}

fn assemble_report(
    cycle: u64,
    issue: u64,
    now: DateTime<Utc>,
    state: &StateJson,
    cycle_changes: &CycleChanges,
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
        cycle_changes,
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

#[allow(clippy::too_many_arguments)]
fn build_state_patch(
    cycle: u64,
    issue: u64,
    timestamp: &str,
    duration_minutes: u64,
    state: &StateJson,
    cycle_changes: &CycleChanges,
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

    updates.extend(build_freshness_updates(
        cycle,
        &updates,
        state,
        cycle_changes,
    ));
    StatePatch { updates }
}

fn collect_cycle_changes(repo_root: &Path, cycle: u64) -> Result<CycleChanges, String> {
    let start_commit = find_cycle_start_commit(repo_root, cycle)?;
    let output = run_git(
        repo_root,
        &["diff", "--name-only", start_commit.as_str(), "HEAD"],
    )?;
    let changed_paths = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect();
    Ok(CycleChanges { changed_paths })
}

fn find_cycle_start_commit(repo_root: &Path, cycle: u64) -> Result<String, String> {
    let pattern = format!(r"\[cycle {}\]", cycle);
    let output = run_git(
        repo_root,
        &[
            "log",
            "-n",
            "1",
            "--format=%H",
            "--grep",
            "^state(cycle-start):",
            "--grep",
            pattern.as_str(),
            "--all-match",
        ],
    )?;
    let commit = output.trim();
    if commit.is_empty() {
        return Err(format!(
            "could not find cycle-start commit for cycle {}; verify the cycle number is correct and that the cycle has started; fetch more history if this is a shallow clone",
            cycle
        ));
    }

    Ok(commit.to_string())
}

fn run_git(repo_root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(args)
        .output()
        .map_err(|error| format!("failed to run git {}: {}", args.join(" "), error))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!(
            "git {} failed with status {}{}",
            args.join(" "),
            output.status,
            if stderr.is_empty() {
                String::new()
            } else {
                format!(": {}", stderr)
            }
        ));
    }

    String::from_utf8(output.stdout).map_err(|error| {
        format!(
            "git {} produced non-UTF-8 output: {}",
            args.join(" "),
            error
        )
    })
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
    let mut changed_paths = apply_state_patch(&mut state_value, patch)?;
    apply_close_out_phase_transition(&mut state_value)?;
    for path in [
        "/cycle_phase/phase",
        "/cycle_phase/phase_entered_at",
        "/cycle_phase/cycle",
        "/field_inventory/fields/cycle_phase/last_refreshed",
    ] {
        if !changed_paths.iter().any(|existing| existing == path) {
            changed_paths.push(path.to_string());
        }
    }
    write_state_value(repo_root, &state_value)?;

    Ok(changed_paths)
}

fn apply_close_out_phase_transition(state_value: &mut Value) -> Result<(), String> {
    let cycle = state_value
        .pointer("/cycle_phase/cycle")
        .and_then(Value::as_u64)
        .ok_or_else(|| "missing numeric /cycle_phase/cycle in docs/state.json".to_string())?;
    transition_cycle_phase(state_value, cycle, "close_out")
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
    cycle_changes: &CycleChanges,
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

    refreshed_fields.extend(build_auto_refresh_fields(
        cycle,
        state,
        &tracked_fields,
        &refreshed_fields,
        cycle_changes,
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

fn build_auto_refresh_fields(
    cycle: u64,
    state: &StateJson,
    tracked_fields: &BTreeSet<&str>,
    refreshed_fields: &BTreeSet<String>,
    cycle_changes: &CycleChanges,
) -> Vec<String> {
    tracked_fields
        .iter()
        .filter(|field_name| !refreshed_fields.contains(**field_name))
        .filter_map(|field_name| {
            state
                .field_inventory
                .fields
                .get(*field_name)
                .filter(|entry| {
                    field_inventory_entry_due_for_auto_refresh(
                        field_name,
                        entry,
                        cycle,
                        cycle_changes,
                    )
                })
                .map(|_| (*field_name).to_string())
        })
        .collect()
}

fn field_inventory_entry_due_for_auto_refresh(
    field_name: &str,
    entry: &Value,
    cycle: u64,
    cycle_changes: &CycleChanges,
) -> bool {
    let cadence = entry
        .get("cadence")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_ascii_lowercase();

    if cadence.contains("every cycle") {
        return field_inventory_entry_needs_refresh(entry, cycle);
    }

    if let Some(interval) = cadence_cycle_interval(&cadence) {
        return field_inventory_entry_is_due(entry, cycle, interval);
    }

    if field_name.starts_with("copilot_metrics.") {
        return field_inventory_entry_needs_refresh(entry, cycle);
    }

    if field_name == "test_count" || cadence.contains("php or ts tests") {
        return cycle_changes.tests_changed() && field_inventory_entry_needs_refresh(entry, cycle);
    }

    if field_name == "typescript_stats" || cadence.contains("ts files") {
        return cycle_changes.ts_source_changed()
            && field_inventory_entry_needs_refresh(entry, cycle);
    }

    if field_name == "typescript_plan.status" || cadence.contains("plan phase transitions") {
        return cycle_changes.ts_files_changed()
            && field_inventory_entry_needs_refresh(entry, cycle);
    }

    if cadence.contains("dispatch or merge") || cadence.contains("pr merges") {
        return cycle_changes.any_changes() && field_inventory_entry_needs_refresh(entry, cycle);
    }

    if cadence.contains("after planning or completing types") {
        return cycle_changes.schema_source_changed()
            && field_inventory_entry_needs_refresh(entry, cycle);
    }

    false
}

fn cadence_cycle_interval(cadence: &str) -> Option<u64> {
    if cadence.contains("every phase transition") || cadence.contains("every cycle") {
        return None;
    }

    if !cadence.contains("every") {
        return None;
    }
    extract_cycle_number(cadence)
}

fn field_inventory_entry_is_due(entry: &Value, cycle: u64, interval: u64) -> bool {
    match entry
        .get("last_refreshed")
        .and_then(Value::as_str)
        .and_then(extract_cycle_number)
    {
        Some(last_refreshed_cycle) => cycle.saturating_sub(last_refreshed_cycle) >= interval,
        None => true,
    }
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

impl CycleChanges {
    fn any_changes(&self) -> bool {
        !self.changed_paths.is_empty()
    }

    fn tests_changed(&self) -> bool {
        self.changed_paths
            .iter()
            .any(|path| path.starts_with("php/test/") || path.starts_with("ts/test/"))
    }

    fn ts_files_changed(&self) -> bool {
        self.changed_paths
            .iter()
            .any(|path| path.starts_with("ts/"))
    }

    fn ts_source_changed(&self) -> bool {
        self.changed_paths
            .iter()
            .any(|path| path.starts_with("ts/src/"))
    }

    fn schema_source_changed(&self) -> bool {
        self.changed_paths
            .iter()
            .any(|path| path.starts_with("php/src/") || path.starts_with("ts/src/"))
    }
}

fn build_review_agent_body(cycle: u64, issue: u64, now: DateTime<Utc>) -> String {
    let date = now.format("%Y-%m-%d");
    let time = now.format("%H%M%S");
    format!(
        "## End-of-Cycle Review — Cycle {cycle}

Your job is to find everything wrong with cycle {cycle}'s work. Be thorough. Be skeptical. If something looks fine on the surface, dig deeper. This is an adversarial review — actively look for problems, inconsistencies, drift, and complacency. Do not assume good faith or give the benefit of the doubt.

Your primary obligation is to find problems. Assume the orchestrator is trying to present its work favorably. Verify claims independently.

**Orchestrator issue**: [#{issue}](https://github.com/EvaLok/schema-org-json-ld/issues/{issue})

Commit your findings as `docs/reviews/cycle-{cycle}.md`. Do NOT attempt to post issue comments.

### Review targets (ALL 8 required)

1. **Code changes** — review all PRs merged this cycle for:
   - Correctness and code quality
   - Test coverage gaps
   - Infrastructure drift (AGENTS.md, skills, checklists out of sync with practice)

2. **Worklog accuracy** at `docs/worklog/{date}/{time}-{{name}}.md` — check for:
   - Cross-reference claims against actual commits (`git log`), state.json, and issue activity
   - Whether the narrative matches reality — are any claims unsupported by evidence?
   - Whether self-modifications are properly documented

3. **Journal quality** at `docs/journal/{date}.md` — check for:
   - Genuine reflection vs formulaic/boilerplate entries
   - Complacency indicators (repeating the same observations without acting on them)
   - Actionable commitments with observable completion conditions

4. **State.json integrity** at `docs/state.json` — check for:
   - Run `bash tools/metric-snapshot` and verify metrics are current
   - Field inventory cadence violations
   - Inconsistencies between state.json and reality

5. **Commit receipt verification** — for each receipt in the worklog:
   - Verify each commit receipt SHA against `git show <sha> --stat`
   - Confirm the committed changes match the worklog claims
   - Check that `bash tools/cycle-receipts --cycle {cycle}` output matches the worklog receipt table
   - **Receipt table scope**: the worklog receipt table covers all commits through `cycle-complete`. The docs commit (`docs(cycle-N): ...`) and record-dispatch commit (`state(record-dispatch): ...`) are **structurally excluded** — they are created after the worklog is written and cannot appear in their own table. This is an inherent temporal constraint, not a defect. Do NOT flag their absence as a worklog-accuracy issue. Instead, verify that all OTHER cycle receipts are present and correct.

6. **Infrastructure consistency** — check that:
   - AGENTS.md, skills, and checklists are consistent with actual practice
   - Tools match their documented behavior
   - No stale references to removed or renamed features

7. **Process adherence** — verify the orchestrator followed:
   - Its own startup checklist (each step posted as a separate comment)
   - The standard completion checklist
   - Tool-first mandate (tools used when tools exist)

8. **Complacency detection** — honestly assess:
   - Is the orchestrator genuinely improving, or going through motions?
   - Are there repeated patterns that should have been automated by now?
   - Are findings being \"noted\" but not fixed? Are deferred items accumulating?
   - Are worklog \"next steps\" actually being followed through?

### Output format

Commit a file at `docs/reviews/cycle-{cycle}.md`. Each finding must follow this exact format:

```
## N. [category-name] Finding title

**File**: path/to/file:line
**Evidence**: what was observed
**Recommendation**: concrete action
```

Categories must be short kebab-case identifiers (max 40 characters).

Include a **Complacency score** section at the end (1-5 scale with evidence-based justification).

Encourage depth over breadth. Three deeply investigated findings with evidence are more valuable than ten surface-level observations.

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

    fn no_cycle_changes() -> CycleChanges {
        CycleChanges::default()
    }

    fn cycle_changes(paths: &[&str]) -> CycleChanges {
        CycleChanges {
            changed_paths: paths.iter().map(|path| (*path).to_string()).collect(),
        }
    }

    fn state_with_agent_sessions(agent_sessions: Value) -> StateJson {
        let mut state = StateJson::default();
        state.cycle_phase.phase_entered_at = Some("2026-03-05T04:00:00Z".to_string());
        state.last_cycle.timestamp = Some("2026-03-05T03:00:00Z".to_string());
        state.agent_sessions = serde_json::from_value(agent_sessions).unwrap();
        state
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
            &no_cycle_changes(),
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
            &no_cycle_changes(),
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

        let freshness_updates = build_freshness_updates(153, &updates, &state, &no_cycle_changes());
        assert_eq!(freshness_updates.len(), 1);
        assert_eq!(
            freshness_updates[0].path,
            "/field_inventory/fields/last_cycle/last_refreshed"
        );
    }

    #[test]
    fn state_patch_refreshes_file_driven_fields_when_relevant_changes_exist() {
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
            "test_count".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "typescript_stats".to_string(),
            json!({"last_refreshed": "cycle 120", "cadence": "every merge that adds/removes TS files"}),
        );
        state.field_inventory.fields.insert(
            "typescript_plan.status".to_string(),
            json!({"last_refreshed": "cycle 120", "cadence": "after plan phase transitions"}),
        );

        let patch = build_state_patch(
            153,
            700,
            "2026-03-06T00:00:00Z",
            47,
            &state,
            &cycle_changes(&[
                "php/test/unit/ProductTest.php",
                "ts/src/schema/Product.ts",
                "ts/test/schema/Product.test.ts",
            ]),
            "summary",
            &no_reconciliation(),
        );
        let freshness_paths: Vec<&str> = patch
            .updates
            .iter()
            .map(|update| update.path.as_str())
            .collect();

        assert!(freshness_paths.contains(&"/field_inventory/fields/test_count/last_refreshed"));
        assert!(
            freshness_paths.contains(&"/field_inventory/fields/typescript_stats/last_refreshed")
        );
        assert!(freshness_paths
            .contains(&"/field_inventory/fields/typescript_plan.status/last_refreshed"));
    }

    #[test]
    fn state_patch_skips_file_driven_fields_without_relevant_changes() {
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
            "test_count".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "typescript_stats".to_string(),
            json!({"last_refreshed": "cycle 120", "cadence": "every merge that adds/removes TS files"}),
        );
        state.field_inventory.fields.insert(
            "typescript_plan.status".to_string(),
            json!({"last_refreshed": "cycle 120", "cadence": "after plan phase transitions"}),
        );
        state.field_inventory.fields.insert(
            "copilot_metrics.in_flight".to_string(),
            json!({"last_refreshed": "cycle 120", "cadence": "every dispatch or merge"}),
        );

        let patch = build_state_patch(
            153,
            700,
            "2026-03-06T00:00:00Z",
            47,
            &state,
            &no_cycle_changes(),
            "summary",
            &no_reconciliation(),
        );
        let freshness_paths: Vec<&str> = patch
            .updates
            .iter()
            .map(|update| update.path.as_str())
            .collect();

        assert!(!freshness_paths.contains(&"/field_inventory/fields/test_count/last_refreshed"));
        assert!(
            !freshness_paths.contains(&"/field_inventory/fields/typescript_stats/last_refreshed")
        );
        assert!(!freshness_paths
            .contains(&"/field_inventory/fields/typescript_plan.status/last_refreshed"));
        assert!(freshness_paths
            .contains(&"/field_inventory/fields/copilot_metrics.in_flight/last_refreshed"));
    }

    #[test]
    fn periodic_cadence_refreshes_only_when_due() {
        let entry = json!({
            "cadence": "every 10 cycles",
            "last_refreshed": "cycle 144"
        });

        assert!(!field_inventory_entry_due_for_auto_refresh(
            "last_tool_audit_cycle",
            &entry,
            153,
            &no_cycle_changes(),
        ));
        assert!(field_inventory_entry_due_for_auto_refresh(
            "last_tool_audit_cycle",
            &entry,
            154,
            &no_cycle_changes(),
        ));
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
            &no_cycle_changes(),
            "custom summary",
            &no_reconciliation(),
        );
        assert_eq!(patch.updates[4].path, "/last_cycle/summary");
        assert_eq!(patch.updates[4].value, json!("custom summary"));
    }

    #[test]
    fn resolve_summary_auto_derives_dispatches_with_zero_merges() {
        let state = state_with_agent_sessions(json!([
            {
                "issue": 1,
                "status": "in_flight",
                "dispatched_at": "2026-03-05T04:15:00Z"
            },
            {
                "issue": 2,
                "status": "in_flight",
                "dispatched_at": "2026-03-05T04:30:00Z"
            },
            {
                "issue": 3,
                "status": "in_flight",
                "dispatched_at": "2026-03-05T03:45:00Z"
            }
        ]));

        let summary = resolve_summary(None, &state, fixed_now()).unwrap();

        assert_eq!(summary, "2 dispatches, 0 merges");
    }

    #[test]
    fn resolve_summary_auto_derives_multiple_merges_and_prs() {
        let state = state_with_agent_sessions(json!([
            {
                "issue": 1,
                "status": "merged",
                "pr": 44,
                "dispatched_at": "2026-03-05T04:01:00Z",
                "merged_at": "2026-03-05T04:20:00Z"
            },
            {
                "issue": 2,
                "status": "merged",
                "pr": 42,
                "dispatched_at": "2026-03-05T04:30:00Z",
                "merged_at": "2026-03-05T05:00:00Z"
            },
            {
                "issue": 3,
                "status": "merged",
                "pr": 50,
                "dispatched_at": "2026-03-05T03:30:00Z",
                "merged_at": "2026-03-05T04:45:00Z"
            },
            {
                "issue": 4,
                "status": "merged",
                "pr": 60,
                "dispatched_at": "2026-03-05T04:35:00Z",
                "merged_at": "2026-03-05T05:10:00Z"
            }
        ]));

        let summary = resolve_summary(None, &state, fixed_now()).unwrap();

        assert_eq!(summary, "3 dispatches, 3 merges (PR #42, PR #44, PR #50)");
    }

    #[test]
    fn resolve_summary_prefers_manual_override() {
        let summary =
            resolve_summary(Some("manual summary"), &StateJson::default(), fixed_now()).unwrap();

        assert_eq!(summary, "manual summary");
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
            json!({"cadence": "every cycle when set (divergence check)", "last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "review_agent".to_string(),
            json!({"cadence": "every cycle (updated when consuming review findings)", "last_refreshed": "cycle 120"}),
        );

        let patch = build_state_patch(
            153,
            700,
            "2026-03-06T00:00:00Z",
            47,
            &state,
            &no_cycle_changes(),
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
                    "publish_gate": {"cadence": "every cycle when set (divergence check)", "last_refreshed": "cycle 120"},
                    "review_agent": {"cadence": "every cycle (updated when consuming review findings)", "last_refreshed": "cycle 120"}
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
    fn close_out_transition_updates_cycle_phase_after_state_patch() {
        let patch = StatePatch {
            updates: vec![PatchUpdate {
                path: "/last_cycle/summary".to_string(),
                value: json!("custom summary"),
            }],
        };
        let mut raw_state = json!({
            "last_cycle": {
                "summary": "old summary"
            },
            "cycle_phase": {
                "cycle": 153,
                "phase": "work",
                "phase_entered_at": "2026-03-05T00:00:00Z"
            },
            "field_inventory": {
                "fields": {
                    "cycle_phase": {"last_refreshed": "cycle 152"}
                }
            }
        });

        apply_state_patch(&mut raw_state, &patch).expect("state patch should apply cleanly");
        apply_close_out_phase_transition(&mut raw_state)
            .expect("close out transition should apply cleanly");

        assert_eq!(
            raw_state.pointer("/cycle_phase/phase"),
            Some(&json!("close_out"))
        );
        assert_eq!(raw_state.pointer("/cycle_phase/cycle"), Some(&json!(153)));
        assert_ne!(
            raw_state
                .pointer("/cycle_phase/phase_entered_at")
                .and_then(Value::as_str),
            Some("2026-03-05T00:00:00Z")
        );
        assert_eq!(
            raw_state
                .pointer("/field_inventory/fields/cycle_phase/last_refreshed")
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
            &no_cycle_changes(),
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

        let report = assemble_report(
            139,
            464,
            fixed_now(),
            &state,
            &no_cycle_changes(),
            "summary",
            &[],
        )
        .unwrap();

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
        assert!(body.contains("[category-name] Finding title"));
        assert!(body.contains("Categories must be short kebab-case identifiers"));
        assert!(body.contains("Do NOT attempt to post a comment"));
        assert!(!body.contains("Post your findings as a comment"));
    }

    #[test]
    fn review_agent_body_includes_all_eight_targets() {
        let body = build_review_agent_body(222, 999, fixed_now());
        assert!(body.contains("adversarial review"));
        assert!(body.contains("Verify claims independently"));
        assert!(body.contains("Code changes"));
        assert!(body.contains("Worklog accuracy"));
        assert!(body.contains("Journal quality"));
        assert!(body.contains("State.json integrity"));
        assert!(body.contains("Commit receipt verification"));
        assert!(body.contains("Infrastructure consistency"));
        assert!(body.contains("Process adherence"));
        assert!(body.contains("Complacency detection"));
        assert!(body.contains("cycle-receipts"));
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

        let report = assemble_report(
            139,
            464,
            fixed_now(),
            &state,
            &no_cycle_changes(),
            "summary",
            &[],
        )
        .unwrap();
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

        let report = assemble_report(
            139,
            464,
            fixed_now(),
            &state,
            &no_cycle_changes(),
            "summary",
            &[],
        )
        .unwrap();

        assert_eq!(report.session_duration_minutes, 47);
        assert_eq!(
            report.state_json_patch.updates[3].path,
            "/last_cycle/duration_minutes"
        );
        assert_eq!(report.state_json_patch.updates[3].value, json!(47));
    }
}
