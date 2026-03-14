use serde_json::{json, Value};
use state_schema::default_agent_model;
use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
    time::SystemTime,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchPatch {
    pub total_dispatches: i64,
    pub resolved: i64,
    pub merged: i64,
    pub closed_without_pr: i64,
    pub reviewed_awaiting_eva: i64,
    pub in_flight: i64,
    pub produced_pr: i64,
    pub pr_merge_rate: String,
    pub dispatch_to_pr_rate: String,
    pub dispatch_log_latest: String,
    pub agent_session: Value,
    pub current_cycle: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
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

pub fn resolve_model(
    cli_model: Option<&str>,
    repo_root: &std::path::Path,
) -> Result<String, String> {
    match cli_model {
        Some(model) if model.trim().is_empty() => Err("--model must not be empty".to_string()),
        Some(model) => Ok(model.trim().to_string()),
        None => default_agent_model(repo_root),
    }
}

pub fn build_dispatch_patch(
    state: &Value,
    current_cycle: u64,
    issue: u64,
    title: &str,
    model: &str,
    dispatched_at: &str,
) -> Result<DispatchPatch, String> {
    let agent_session = json!({
        "issue": issue,
        "title": title,
        "dispatched_at": dispatched_at,
        "model": model,
        "status": "in_flight"
    });
    let derived = derive_metrics_with_new_session(state, &agent_session)?;
    validate_dispatch_invariant(
        derived.total_dispatches,
        derived.resolved,
        derived.in_flight,
    )?;

    Ok(DispatchPatch {
        total_dispatches: derived.total_dispatches,
        resolved: derived.resolved,
        merged: derived.merged,
        closed_without_pr: derived.closed_without_pr,
        reviewed_awaiting_eva: derived.reviewed_awaiting_eva,
        in_flight: derived.in_flight,
        produced_pr: derived.produced_pr,
        pr_merge_rate: derived.pr_merge_rate,
        dispatch_to_pr_rate: derived.dispatch_to_pr_rate,
        dispatch_log_latest: format_dispatch_log(issue, title, current_cycle),
        agent_session,
        current_cycle,
    })
}

/// Derive the next copilot_metrics values by simulating the addition of a new
/// in-flight session before mutating docs/state.json.
///
/// Returns an error if `agent_sessions` is missing or if any session status is
/// unsupported, so callers can fail closed before writing state.
fn derive_metrics_with_new_session(
    state: &Value,
    new_session: &Value,
) -> Result<DerivedMetrics, String> {
    let mut sessions = state
        .pointer("/agent_sessions")
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| "missing array /agent_sessions in docs/state.json".to_string())?;
    sessions.push(new_session.clone());
    derive_metrics_from_sessions(&sessions)
}

/// Recalculate the derived copilot_metrics block from the agent_sessions
/// ledger, failing closed on missing or unsupported session statuses.
fn derive_metrics_from_sessions(sessions: &[Value]) -> Result<DerivedMetrics, String> {
    let total_dispatches = i64::try_from(sessions.len())
        .map_err(|_| "agent_sessions length should fit within i64".to_string())?;
    let mut merged = 0_i64;
    let mut closed_without_pr = 0_i64;
    let mut reviewed_awaiting_eva = 0_i64;
    let mut in_flight = 0_i64;
    let mut produced_pr = 0_i64;

    for (index, session) in sessions.iter().enumerate() {
        if session.get("pr").and_then(Value::as_u64).is_some() {
            produced_pr += 1;
        }

        match session.get("status").and_then(Value::as_str) {
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

/// Format a ratio as a percentage string with one decimal place.
///
/// Returns `0.0%` when the denominator is zero.
fn format_percentage(numerator: i64, denominator: i64) -> String {
    if denominator == 0 {
        return "0.0%".to_string();
    }

    let percentage = (numerator as f64 / denominator as f64) * 100.0;
    format!("{:.1}%", percentage)
}

pub fn validate_dispatch_invariant(
    total_dispatches: i64,
    resolved: i64,
    in_flight: i64,
) -> Result<(), String> {
    if resolved + in_flight != total_dispatches {
        return Err(format!(
            "invariant violated: resolved({}) + in_flight({}) != total_dispatches({})",
            resolved, in_flight, total_dispatches
        ));
    }

    Ok(())
}

pub fn format_dispatch_log(issue: u64, title: &str, current_cycle: u64) -> String {
    format!("#{} {} (cycle {})", issue, title, current_cycle)
}

/// Update `field_inventory.fields[*].last_refreshed` for a metric rewritten in
/// the current cycle.
///
/// Returns an error if `field_inventory.fields` is missing or if an existing
/// entry is not an object.
fn update_field_inventory_last_refreshed(
    state: &mut Value,
    field_name: &str,
    cycle_marker: &str,
) -> Result<(), String> {
    let fields = state
        .pointer_mut("/field_inventory/fields")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "missing object /field_inventory/fields in docs/state.json".to_string())?;
    let field = fields
        .entry(field_name.to_string())
        .or_insert_with(|| json!({}));
    let object = field
        .as_object_mut()
        .ok_or_else(|| format!("field_inventory entry must be an object: {}", field_name))?;
    object.insert("last_refreshed".to_string(), json!(cycle_marker));
    Ok(())
}

/// Keep this list aligned with the terminal agent session statuses enforced by
/// `state-invariants`; anything else is treated as live for duplicate-guard
/// purposes.
const TERMINAL_AGENT_SESSION_STATUSES: [&str; 5] = [
    "merged",
    "failed",
    "closed",
    "closed_without_merge",
    "closed_without_pr",
];

fn is_terminal_status(status: &str) -> bool {
    TERMINAL_AGENT_SESSION_STATUSES.contains(&status)
}

pub fn apply_dispatch_patch(state: &mut Value, patch: &DispatchPatch) -> Result<(), String> {
    let cycle_marker = format!("cycle {}", patch.current_cycle);
    let new_issue = patch
        .agent_session
        .get("issue")
        .and_then(Value::as_u64)
        .ok_or_else(|| "agent_session missing 'issue' field".to_string())?;
    let duplicate = state
        .pointer("/agent_sessions")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing array /agent_sessions in docs/state.json".to_string())?
        .iter()
        .any(|s| {
            s.get("issue").and_then(Value::as_u64) == Some(new_issue)
                && s.get("status")
                    .and_then(Value::as_str)
                    // Missing status must fail closed as a potentially live
                    // session, so duplicate dispatches stay blocked.
                    .map(|status| !is_terminal_status(status))
                    .unwrap_or(true)
        });
    if duplicate {
        return Err(format!(
            "agent_sessions already contains an entry for issue #{}; refusing to create duplicate",
            new_issue
        ));
    }

    let metrics = state
        .pointer_mut("/copilot_metrics")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "missing object /copilot_metrics in docs/state.json".to_string())?;
    metrics.insert(
        "total_dispatches".to_string(),
        json!(patch.total_dispatches),
    );
    metrics.insert("resolved".to_string(), json!(patch.resolved));
    metrics.insert("merged".to_string(), json!(patch.merged));
    metrics.insert(
        "closed_without_pr".to_string(),
        json!(patch.closed_without_pr),
    );
    metrics.insert(
        "reviewed_awaiting_eva".to_string(),
        json!(patch.reviewed_awaiting_eva),
    );
    metrics.insert("in_flight".to_string(), json!(patch.in_flight));
    metrics.insert("produced_pr".to_string(), json!(patch.produced_pr));
    metrics.insert("pr_merge_rate".to_string(), json!(patch.pr_merge_rate));
    metrics.insert(
        "dispatch_to_pr_rate".to_string(),
        json!(patch.dispatch_to_pr_rate),
    );
    metrics.insert(
        "dispatch_log_latest".to_string(),
        json!(patch.dispatch_log_latest),
    );
    update_field_inventory_last_refreshed(state, "copilot_metrics.in_flight", &cycle_marker)?;
    update_field_inventory_last_refreshed(
        state,
        "copilot_metrics.pr_merge_rate",
        &cycle_marker,
    )?;
    update_field_inventory_last_refreshed(
        state,
        "copilot_metrics.dispatch_to_pr_rate",
        &cycle_marker,
    )?;
    state
        .pointer_mut("/agent_sessions")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| "missing array /agent_sessions in docs/state.json".to_string())?
        .push(patch.agent_session.clone());

    Ok(())
}

pub fn dispatch_commit_message(issue: u64, current_cycle: u64) -> String {
    format!(
        "state(record-dispatch): #{} dispatched [cycle {}]",
        issue, current_cycle
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorklogFixupOutcome {
    Updated(PathBuf),
    NotFound,
}

const IN_FLIGHT_WORKLOG_PREFIX: &str = "- **In-flight agent sessions**: ";

pub fn fixup_latest_worklog_in_flight(
    repo_root: &Path,
    in_flight: i64,
) -> Result<WorklogFixupOutcome, String> {
    let Some(worklog_path) = find_latest_worklog_file(repo_root)? else {
        return Ok(WorklogFixupOutcome::NotFound);
    };

    let content = fs::read_to_string(&worklog_path)
        .map_err(|error| format!("failed to read {}: {}", worklog_path.display(), error))?;
    let updated = replace_in_flight_line(&content, in_flight).ok_or_else(|| {
        format!(
            "missing '{}' line in {}",
            IN_FLIGHT_WORKLOG_PREFIX.trim_end(),
            worklog_path.display()
        )
    })?;
    fs::write(&worklog_path, updated)
        .map_err(|error| format!("failed to write {}: {}", worklog_path.display(), error))?;

    Ok(WorklogFixupOutcome::Updated(worklog_path))
}

fn find_latest_worklog_file(repo_root: &Path) -> Result<Option<PathBuf>, String> {
    let worklog_root = repo_root.join("docs/worklog");
    if !worklog_root.exists() {
        return Ok(None);
    }
    let metadata = fs::metadata(&worklog_root)
        .map_err(|error| format!("failed to read {}: {}", worklog_root.display(), error))?;
    if !metadata.is_dir() {
        return Err(format!(
            "expected {} to be a directory",
            worklog_root.display()
        ));
    }

    let mut latest: Option<(SystemTime, PathBuf)> = None;
    let mut pending = vec![worklog_root];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("failed to read {}: {}", directory.display(), error))?
        {
            let entry = entry.map_err(|error| {
                format!("failed to read entry in {}: {}", directory.display(), error)
            })?;
            let path = entry.path();
            let file_type = entry.file_type().map_err(|error| {
                format!("failed to read file type for {}: {}", path.display(), error)
            })?;
            if file_type.is_dir() {
                pending.push(path);
                continue;
            }
            if !file_type.is_file() || path.extension() != Some(OsStr::new("md")) {
                continue;
            }

            let modified = entry
                .metadata()
                .map_err(|error| format!("failed to read {}: {}", path.display(), error))?
                .modified()
                .map_err(|error| {
                    format!("failed to read modification time for {}: {}", path.display(), error)
                })?;
            let should_replace = latest
                .as_ref()
                .is_none_or(|(current_modified, current_path)| {
                    modified > *current_modified
                        || (modified == *current_modified && path > *current_path)
                });
            if should_replace {
                latest = Some((modified, path));
            }
        }
    }

    Ok(latest.map(|(_, path)| path))
}

fn replace_in_flight_line(content: &str, in_flight: i64) -> Option<String> {
    let replacement = format!("{IN_FLIGHT_WORKLOG_PREFIX}{in_flight}");
    let trailing_newlines = &content[content.trim_end_matches('\n').len()..];
    let mut replaced = false;
    let mut updated_lines = Vec::new();
    for line in content.lines() {
        if !replaced && line.starts_with(IN_FLIGHT_WORKLOG_PREFIX) {
            updated_lines.push(replacement.clone());
            replaced = true;
        } else {
            updated_lines.push(line.to_string());
        }
    }
    if !replaced {
        return None;
    }

    let mut updated = updated_lines.join("\n");
    updated.push_str(trailing_newlines);
    Some(updated)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

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
                    "issue": 601,
                    "title": "Merged change",
                    "dispatched_at": "2026-03-01T00:00:00Z",
                    "model": model.clone(),
                    "status": "merged",
                    "pr": 700,
                    "merged_at": "2026-03-02T00:00:00Z"
                },
                {
                    "issue": 602,
                    "title": "Closed change",
                    "dispatched_at": "2026-03-03T00:00:00Z",
                    "model": model,
                    "status": "closed_without_pr"
                }
            ],
            "last_cycle": { "number": 164 },
            "copilot_metrics": {
                "total_dispatches": 2,
                "in_flight": 0,
                "produced_pr": 1,
                "resolved": 2,
                "merged": 1,
                "closed_without_merge": 1,
                "closed_without_pr": 1,
                "reviewed_awaiting_eva": 0,
                "dispatch_to_pr_rate": "50.0%",
                "pr_merge_rate": "100.0%",
                "dispatch_log_latest": "#602 Closed change (cycle 164)"
            },
            "field_inventory": {
                "fields": {
                    "copilot_metrics.pr_merge_rate": { "last_refreshed": "cycle 163" },
                    "copilot_metrics.dispatch_to_pr_rate": { "last_refreshed": "cycle 163" },
                    "copilot_metrics.in_flight": { "last_refreshed": "cycle 163" }
                }
            }
        })
    }

    #[test]
    fn metric_calculation_after_dispatch_is_correct() {
        let model = default_test_model();
        let patch = build_dispatch_patch(
            &sample_state(),
            164,
            602,
            "Example dispatch",
            &model,
            "2026-03-07T13:00:00Z",
        )
        .expect("patch should build");
        assert_eq!(patch.total_dispatches, 3);
        assert_eq!(patch.in_flight, 1);
        assert_eq!(
            patch.dispatch_log_latest,
            "#602 Example dispatch (cycle 164)"
        );
    }

    #[test]
    fn dispatch_log_formatting_is_correct() {
        assert_eq!(
            format_dispatch_log(602, "Example dispatch", 164),
            "#602 Example dispatch (cycle 164)"
        );
    }

    #[test]
    fn invariant_validation_fails_when_totals_do_not_match() {
        let error = validate_dispatch_invariant(86, 82, 3).expect_err("invariant should fail");
        assert!(error.contains("invariant violated"));
    }

    #[test]
    fn resolve_model_uses_shared_default_when_flag_is_omitted() {
        let model = resolve_model(None, &repo_root()).expect("default model should resolve");
        assert_eq!(model, default_test_model());
    }

    #[test]
    fn resolve_model_prefers_cli_override() {
        let model =
            resolve_model(Some("custom-model"), &repo_root()).expect("override should resolve");
        assert_eq!(model, "custom-model");
    }

    #[test]
    fn resolve_model_rejects_empty_cli_override() {
        let error = resolve_model(Some("   "), &repo_root()).expect_err("empty override must fail");
        assert_eq!(error, "--model must not be empty");
    }

    #[test]
    fn dispatch_patch_includes_agent_session_entry() {
        let model = default_test_model();
        let patch = build_dispatch_patch(
            &sample_state(),
            164,
            602,
            "Example dispatch",
            &model,
            "2026-03-07T13:00:00Z",
        )
        .expect("patch should build");

        assert_eq!(
            patch.agent_session,
            json!({
                "issue": 602,
                "title": "Example dispatch",
                "dispatched_at": "2026-03-07T13:00:00Z",
                "model": model,
                "status": "in_flight"
            })
        );
    }

    #[test]
    fn apply_dispatch_patch_appends_agent_session() {
        let mut state = sample_state();
        let model = default_test_model();
        let patch = build_dispatch_patch(
            &state,
            164,
            603,
            "Example dispatch",
            &model,
            "2026-03-07T13:00:00Z",
        )
        .expect("patch should build");

        apply_dispatch_patch(&mut state, &patch).expect("patch should apply");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions.len(), 3);
        assert_eq!(state["copilot_metrics"]["total_dispatches"], json!(3));
        assert_eq!(state["copilot_metrics"]["resolved"], json!(2));
        assert_eq!(state["copilot_metrics"]["merged"], json!(1));
        assert_eq!(state["copilot_metrics"]["closed_without_pr"], json!(1));
        assert_eq!(state["copilot_metrics"]["reviewed_awaiting_eva"], json!(0));
        assert_eq!(state["copilot_metrics"]["produced_pr"], json!(1));
        assert_eq!(state["copilot_metrics"]["in_flight"], json!(1));
        assert_eq!(state["copilot_metrics"]["pr_merge_rate"], json!("100.0%"));
        assert_eq!(
            state["copilot_metrics"]["dispatch_log_latest"],
            json!("#603 Example dispatch (cycle 164)")
        );
        assert_eq!(
            state["copilot_metrics"]["dispatch_to_pr_rate"],
            json!("33.3%")
        );
        assert_eq!(
            state["field_inventory"]["fields"]["copilot_metrics.in_flight"]["last_refreshed"],
            json!("cycle 164")
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
        assert_eq!(sessions[2]["issue"], json!(603));
        assert_eq!(sessions[2]["status"], json!("in_flight"));
        assert_eq!(sessions[2]["dispatched_at"], json!("2026-03-07T13:00:00Z"));
    }

    #[test]
    fn apply_dispatch_patch_rejects_duplicate_issue() {
        let mut state = sample_state();
        let model = default_test_model();
        state["agent_sessions"]
            .as_array_mut()
            .expect("agent_sessions array")
            .push(json!({
                "issue": 601,
                "title": "Live duplicate",
                "dispatched_at": "2026-03-06T00:00:00Z",
                "model": model.clone(),
                "status": "in_flight"
            }));
        let patch = build_dispatch_patch(
            &state,
            164,
            601,
            "Duplicate dispatch",
            &model,
            "2026-03-07T13:00:00Z",
        )
        .expect("patch should build");

        let error = apply_dispatch_patch(&mut state, &patch)
            .expect_err("duplicate issue should be rejected");
        assert!(error.contains("already contains an entry for issue #601"));
    }

    #[test]
    fn apply_dispatch_patch_allows_duplicate_issue_after_terminal_session() {
        let mut state = sample_state();
        let model = default_test_model();
        let patch = build_dispatch_patch(
            &state,
            164,
            601,
            "Redispatched after merge",
            &model,
            "2026-03-07T13:00:00Z",
        )
        .expect("patch should build");

        apply_dispatch_patch(&mut state, &patch).expect("terminal duplicate should be allowed");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions.len(), 3);
        assert_eq!(sessions[2]["issue"], json!(601));
        assert_eq!(sessions[2]["status"], json!("in_flight"));
        assert_eq!(state["copilot_metrics"]["total_dispatches"], json!(3));
        assert_eq!(state["copilot_metrics"]["in_flight"], json!(1));
    }

    #[test]
    fn apply_dispatch_patch_leaves_state_unchanged_for_duplicate_in_flight_issue() {
        let mut state = sample_state();
        let model = default_test_model();
        state["agent_sessions"]
            .as_array_mut()
            .expect("agent_sessions array")
            .push(json!({
                "issue": 602,
                "title": "Still live",
                "dispatched_at": "2026-03-06T00:00:00Z",
                "model": model.clone(),
                "status": "in_flight"
            }));
        let original = state.clone();
        let patch = build_dispatch_patch(
            &state,
            164,
            602,
            "Duplicate live dispatch",
            &model,
            "2026-03-07T13:00:00Z",
        )
        .expect("patch should build");

        let error = apply_dispatch_patch(&mut state, &patch)
            .expect_err("duplicate live issue should be rejected");

        assert!(error.contains("already contains an entry for issue #602"));
        assert_eq!(state, original);
    }

    #[test]
    fn apply_dispatch_patch_rejects_duplicate_issue_when_status_is_missing() {
        let mut state = sample_state();
        let model = default_test_model();
        state["agent_sessions"]
            .as_array_mut()
            .expect("agent_sessions array")
            .push(json!({
                "issue": 603,
                "title": "Missing status duplicate",
                "dispatched_at": "2026-03-06T00:00:00Z",
                "model": model.clone()
            }));
        // `build_dispatch_patch` already fails closed on missing statuses; build
        // the patch directly so this test can exercise `apply_dispatch_patch`'s
        // duplicate guard for malformed existing session rows.
        let patch = DispatchPatch {
            total_dispatches: 4,
            resolved: 2,
            merged: 1,
            closed_without_pr: 1,
            reviewed_awaiting_eva: 0,
            in_flight: 1,
            produced_pr: 1,
            pr_merge_rate: "100.0%".to_string(),
            dispatch_to_pr_rate: "25.0%".to_string(),
            dispatch_log_latest: "#603 Duplicate live dispatch (cycle 164)".to_string(),
            agent_session: json!({
                "issue": 603,
                "title": "Duplicate live dispatch",
                "dispatched_at": "2026-03-07T13:00:00Z",
                "model": model,
                "status": "in_flight"
            }),
            current_cycle: 164,
        };

        let error = apply_dispatch_patch(&mut state, &patch)
            .expect_err("missing status duplicate should fail closed");

        assert!(error.contains("already contains an entry for issue #603"));
    }

    #[test]
    fn apply_dispatch_patch_recomputes_missing_derived_metric_fields() {
        let mut state = sample_state();
        let model = default_test_model();
        state["copilot_metrics"]
            .as_object_mut()
            .expect("copilot_metrics object")
            .remove("dispatch_to_pr_rate");
        state["copilot_metrics"]
            .as_object_mut()
            .expect("copilot_metrics object")
            .remove("pr_merge_rate");
        let patch = build_dispatch_patch(
            &state,
            164,
            603,
            "Example dispatch",
            &model,
            "2026-03-07T13:00:00Z",
        )
        .expect("patch should build");

        apply_dispatch_patch(&mut state, &patch).expect("patch should apply");

        assert_eq!(
            state["copilot_metrics"]["dispatch_to_pr_rate"],
            json!("33.3%")
        );
        assert_eq!(state["copilot_metrics"]["pr_merge_rate"], json!("100.0%"));
        assert_eq!(state["copilot_metrics"]["total_dispatches"], json!(3));
        assert_eq!(state["copilot_metrics"]["in_flight"], json!(1));
        assert_eq!(
            state["copilot_metrics"]["dispatch_log_latest"],
            json!("#603 Example dispatch (cycle 164)")
        );
    }

    #[test]
    fn build_dispatch_patch_fails_closed_for_unknown_agent_session_status() {
        let mut state = sample_state();
        let model = default_test_model();
        state["agent_sessions"][1]["status"] = json!("mystery_status");

        let error = build_dispatch_patch(
            &state,
            164,
            602,
            "Example dispatch",
            &model,
            "2026-03-07T13:00:00Z",
        )
        .expect_err("unknown statuses must fail closed");

        assert!(error.contains("unsupported value"));
        assert!(error.contains("mystery_status"));
    }

    #[test]
    fn fixup_latest_worklog_updates_in_flight_line() {
        let repo_root = temp_repo_root("record-dispatch-worklog");
        let worklog_dir = repo_root.join("docs/worklog/2026-03-10");
        fs::create_dir_all(&worklog_dir).expect("worklog dir should exist");
        let worklog_path = worklog_dir.join("142511-cycle.md");
        fs::write(
            &worklog_path,
            concat!(
                "## Current state\n\n",
                "- **In-flight agent sessions**: 0\n",
                "- **Pipeline status**: PASS (8/8)\n"
            ),
        )
        .expect("worklog should be written");

        let outcome =
            fixup_latest_worklog_in_flight(&repo_root, 1).expect("worklog fixup should succeed");

        assert_eq!(outcome, WorklogFixupOutcome::Updated(worklog_path.clone()));
        let updated = fs::read_to_string(&worklog_path).expect("worklog should be readable");
        assert!(updated.contains("- **In-flight agent sessions**: 1"));
        assert!(!updated.contains("- **In-flight agent sessions**: 0"));
    }

    #[test]
    fn fixup_latest_worklog_returns_not_found_when_worklog_is_missing() {
        let repo_root = temp_repo_root("record-dispatch-no-worklog");
        let outcome =
            fixup_latest_worklog_in_flight(&repo_root, 1).expect("missing worklog is not fatal");
        assert_eq!(outcome, WorklogFixupOutcome::NotFound);
    }

    fn temp_repo_root(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{unique}"));
        fs::create_dir_all(path.join("docs")).expect("temp repo root should be created");
        path
    }
}
