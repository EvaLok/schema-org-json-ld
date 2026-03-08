use chrono::Utc;
use clap::Parser;
use serde_json::{json, Value};
use state_schema::{
    commit_state_json, current_cycle_from_state, default_agent_model, read_state_value,
    set_value_at_pointer, write_state_value,
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "record-dispatch")]
struct Cli {
    /// GitHub issue number that was dispatched
    #[arg(long)]
    issue: u64,

    /// Short title/description of the dispatch
    #[arg(long)]
    title: String,

    /// Model used for the dispatch
    #[arg(long)]
    model: Option<String>,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DispatchPatch {
    total_dispatches: i64,
    in_flight: i64,
    dispatch_log_latest: String,
    agent_session: Value,
    current_cycle: u64,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let model = resolve_model(cli.model.as_deref(), &cli.repo_root)?;
    let mut state_value = read_state_value(&cli.repo_root)?;
    let dispatched_at = current_utc_timestamp();
    let current_cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /last_cycle/number in state.json" {
            "missing numeric /last_cycle/number in docs/state.json".to_string()
        } else {
            error
        }
    })?;

    let patch = build_dispatch_patch(
        &state_value,
        current_cycle,
        cli.issue,
        &cli.title,
        &model,
        &dispatched_at,
    )?;
    apply_dispatch_patch(&mut state_value, &patch)?;
    write_state_value(&cli.repo_root, &state_value)?;

    let commit_message = format!(
        "state(record-dispatch): #{} dispatched [cycle {}]",
        cli.issue, patch.current_cycle
    );
    let receipt = commit_state_json(&cli.repo_root, &commit_message)?;
    println!(
        "Dispatch recorded: #{} \"{}\" (model: {}). In-flight: {} (receipt: {})",
        cli.issue, cli.title, model, patch.in_flight, receipt
    );
    if patch.in_flight >= 3 {
        eprintln!(
            "Warning: in-flight dispatches at {} (approaching/exceeding concurrency limit of 2)",
            patch.in_flight
        );
    }

    Ok(())
}

fn resolve_model(cli_model: Option<&str>, repo_root: &std::path::Path) -> Result<String, String> {
    match cli_model {
        Some(model) if model.trim().is_empty() => Err("--model must not be empty".to_string()),
        Some(model) => Ok(model.trim().to_string()),
        None => default_agent_model(repo_root),
    }
}

fn build_dispatch_patch(
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

fn current_utc_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn read_required_i64(state: &Value, pointer: &str) -> Result<i64, String> {
    state
        .pointer(pointer)
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("missing numeric {} in docs/state.json", pointer))
}

fn validate_dispatch_invariant(
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

fn format_dispatch_log(issue: u64, title: &str, current_cycle: u64) -> String {
    format!("#{} {} (cycle {})", issue, title, current_cycle)
}

fn apply_dispatch_patch(state: &mut Value, patch: &DispatchPatch) -> Result<(), String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
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
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--issue"));
        assert!(help.contains("--title"));
        assert!(help.contains("--model"));
        assert!(help.contains("--repo-root"));
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
        assert_eq!(patch.dispatch_log_latest, "#602 Example dispatch (cycle 164)");
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
    fn concurrency_warning_threshold_is_triggered_at_three() {
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
        assert!(patch.in_flight >= 3);
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
        assert_eq!(state["copilot_metrics"]["dispatch_to_pr_rate"], json!("81/85"));
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
