use serde_json::{json, Value};
use state_schema::default_agent_model;

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

pub fn apply_dispatch_patch(state: &mut Value, patch: &DispatchPatch) -> Result<(), String> {
    let cycle_marker = format!("cycle {}", patch.current_cycle);
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

#[cfg(test)]
mod tests {
    use super::*;
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
            602,
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
            json!("#602 Example dispatch (cycle 164)")
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
        assert_eq!(sessions[2]["issue"], json!(602));
        assert_eq!(sessions[2]["status"], json!("in_flight"));
        assert_eq!(sessions[2]["dispatched_at"], json!("2026-03-07T13:00:00Z"));
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
            602,
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
            json!("#602 Example dispatch (cycle 164)")
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
}
