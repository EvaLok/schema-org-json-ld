use serde_json::{json, Value};
use state_schema::{default_agent_model, set_value_at_pointer};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchPatch {
    pub total_dispatches: i64,
    pub in_flight: i64,
    pub dispatch_log_latest: String,
    pub agent_session: Value,
    pub current_cycle: u64,
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
    let total_dispatches = read_required_i64(state, "/copilot_metrics/total_dispatches")?;
    let in_flight = read_required_i64(state, "/copilot_metrics/in_flight")?;
    let resolved = read_required_i64(state, "/copilot_metrics/resolved")?;

    let next_total_dispatches = total_dispatches + 1;
    let next_in_flight = in_flight + 1;

    validate_dispatch_invariant(next_total_dispatches, resolved, next_in_flight)?;

    Ok(DispatchPatch {
        total_dispatches: next_total_dispatches,
        in_flight: next_in_flight,
        dispatch_log_latest: format_dispatch_log(issue, title, current_cycle),
        agent_session: json!({
            "issue": issue,
            "title": title,
            "dispatched_at": dispatched_at,
            "model": model,
            "status": "in_flight"
        }),
        current_cycle,
    })
}

fn read_required_i64(state: &Value, pointer: &str) -> Result<i64, String> {
    state
        .pointer(pointer)
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("missing numeric {} in docs/state.json", pointer))
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

pub fn apply_dispatch_patch(state: &mut Value, patch: &DispatchPatch) -> Result<(), String> {
    let cycle_marker = format!("cycle {}", patch.current_cycle);
    set_value_at_pointer(
        state,
        "/copilot_metrics/total_dispatches",
        json!(patch.total_dispatches),
    )?;
    set_value_at_pointer(state, "/copilot_metrics/in_flight", json!(patch.in_flight))?;
    set_value_at_pointer(
        state,
        "/copilot_metrics/dispatch_log_latest",
        json!(patch.dispatch_log_latest),
    )?;
    set_value_at_pointer(
        state,
        "/field_inventory/fields/copilot_metrics.in_flight/last_refreshed",
        json!(cycle_marker.clone()),
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
                    "title": "old dispatch",
                    "dispatched_at": "2026-03-01T00:00:00Z",
                    "model": model,
                    "status": "merged",
                    "pr": 700,
                    "merged_at": "2026-03-02T00:00:00Z"
                }
            ],
            "last_cycle": { "number": 164 },
            "copilot_metrics": {
                "total_dispatches": 85,
                "in_flight": 2,
                "produced_pr": 81,
                "resolved": 83,
                "merged": 80,
                "closed_without_merge": 1,
                "dispatch_to_pr_rate": "81/85",
                "dispatch_log_latest": "#601 old dispatch (cycle 164)"
            },
            "field_inventory": {
                "fields": {
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
        assert_eq!(patch.total_dispatches, 86);
        assert_eq!(patch.in_flight, 3);
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
        assert_eq!(sessions.len(), 2);
        assert_eq!(state["copilot_metrics"]["total_dispatches"], json!(86));
        assert_eq!(state["copilot_metrics"]["in_flight"], json!(3));
        assert_eq!(
            state["copilot_metrics"]["dispatch_log_latest"],
            json!("#602 Example dispatch (cycle 164)")
        );
        assert_eq!(
            state["copilot_metrics"]["dispatch_to_pr_rate"],
            json!("81/85")
        );
        assert_eq!(
            state["field_inventory"]["fields"]["copilot_metrics.in_flight"]["last_refreshed"],
            json!("cycle 164")
        );
        assert_eq!(
            state["field_inventory"]["fields"]["copilot_metrics.dispatch_to_pr_rate"]
                ["last_refreshed"],
            json!("cycle 163")
        );
        assert_eq!(sessions[1]["issue"], json!(602));
        assert_eq!(sessions[1]["status"], json!("in_flight"));
        assert_eq!(sessions[1]["dispatched_at"], json!("2026-03-07T13:00:00Z"));
    }

    #[test]
    fn apply_dispatch_patch_does_not_require_derived_metric_fields() {
        let mut state = sample_state();
        let model = default_test_model();
        state["copilot_metrics"]
            .as_object_mut()
            .expect("copilot_metrics object")
            .remove("dispatch_to_pr_rate");
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

        assert!(state["copilot_metrics"]["dispatch_to_pr_rate"].is_null());
        assert_eq!(state["copilot_metrics"]["total_dispatches"], json!(86));
        assert_eq!(state["copilot_metrics"]["in_flight"], json!(3));
        assert_eq!(
            state["copilot_metrics"]["dispatch_log_latest"],
            json!("#602 Example dispatch (cycle 164)")
        );
    }
}
