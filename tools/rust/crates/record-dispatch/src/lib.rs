use serde_json::{json, Value};
use state_schema::{current_utc_timestamp, default_agent_model};
use std::{fs, path::Path, process::Command};

pub const PIPELINE_GATE_FAILURE_MESSAGE: &str =
    "Cannot dispatch: pipeline-check failed. Fix failures before dispatching.";
pub const REVIEW_DISPATCH_WARNING: &str =
    "Pipeline gate bypassed for review dispatch (--review-dispatch)";
const PIPELINE_CHECK_ARGS: [&str; 9] = [
    "tools/pipeline-check",
    "--exclude-step",
    "step-comments",
    "--exclude-step",
    "current-cycle-steps",
    "--exclude-step",
    "deferral-accumulation",
    "--exclude-step",
    "chronic-category-currency",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineGateError {
    Failed,
    ExecutionFailed(String),
    ReviewDispatchBlocked(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionResult {
    pub exit_code: Option<i32>,
}

pub trait CommandRunner {
    fn run_pipeline_check(&self, repo_root: &Path) -> Result<ExecutionResult, String>;
}

pub struct ProcessRunner;

impl CommandRunner for ProcessRunner {
    fn run_pipeline_check(&self, repo_root: &Path) -> Result<ExecutionResult, String> {
        let status = Command::new("bash")
            .args(PIPELINE_CHECK_ARGS)
            .current_dir(repo_root)
            .status()
            .map_err(|error| format!("failed to execute pipeline-check: {}", error))?;

        Ok(ExecutionResult {
            exit_code: status.code(),
        })
    }
}

pub fn enforce_pipeline_gate(
    repo_root: &Path,
    review_dispatch: bool,
    runner: &dyn CommandRunner,
) -> Result<Option<&'static str>, PipelineGateError> {
    if review_dispatch {
        enforce_review_dispatch_gate(repo_root)?;
        return Ok(Some(REVIEW_DISPATCH_WARNING));
    }

    let execution = runner
        .run_pipeline_check(repo_root)
        .map_err(PipelineGateError::ExecutionFailed)?;

    match execution.exit_code {
        Some(0) => Ok(None),
        _ => Err(PipelineGateError::Failed),
    }
}

fn enforce_review_dispatch_gate(repo_root: &Path) -> Result<(), PipelineGateError> {
    let state = read_state_json(repo_root).map_err(PipelineGateError::ReviewDispatchBlocked)?;
    let gate = state.pointer("/tool_pipeline/c5_5_gate").ok_or_else(|| {
        PipelineGateError::ReviewDispatchBlocked(
            "Cannot dispatch review: no C5.5 gate result found in state.json".to_string(),
        )
    })?;
    let status = gate.get("status").and_then(Value::as_str).ok_or_else(|| {
        PipelineGateError::ReviewDispatchBlocked(
            "Cannot dispatch review: invalid C5.5 gate status in state.json".to_string(),
        )
    })?;
    let cycle = gate.get("cycle").and_then(Value::as_u64).ok_or_else(|| {
        PipelineGateError::ReviewDispatchBlocked(
            "Cannot dispatch review: invalid C5.5 gate cycle in state.json".to_string(),
        )
    })?;
    if status != "PASS" {
        return Err(PipelineGateError::ReviewDispatchBlocked(format!(
            "Cannot dispatch review: C5.5 gate status is {} (cycle {})",
            status, cycle
        )));
    }
    let current_cycle = state
        .pointer("/cycle_phase/cycle")
        .and_then(Value::as_u64)
        .or_else(|| state.pointer("/last_cycle/number").and_then(Value::as_u64))
        .ok_or_else(|| {
            PipelineGateError::ReviewDispatchBlocked(
                "Cannot dispatch review: missing /cycle_phase/cycle or /last_cycle/number in state.json"
                    .to_string(),
            )
        })?;
    if cycle != current_cycle {
        return Err(PipelineGateError::ReviewDispatchBlocked(format!(
            "Cannot dispatch review: C5.5 gate is stale (gate cycle {}, current cycle {})",
            cycle, current_cycle
        )));
    }

    let needs_reverify = gate
        .get("needs_reverify")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            PipelineGateError::ReviewDispatchBlocked(
                "Cannot dispatch review: invalid C5.5 gate needs_reverify in state.json"
                    .to_string(),
            )
        })?;
    if needs_reverify {
        return Err(PipelineGateError::ReviewDispatchBlocked(
            "Cannot dispatch review: C5.5 gate needs re-verification".to_string(),
        ));
    }

    Ok(())
}

fn read_state_json(repo_root: &Path) -> Result<Value, String> {
    let path = repo_root.join("docs/state.json");
    let contents = fs::read_to_string(&path).map_err(|error| {
        format!(
            "Cannot dispatch review: failed to read {}: {}",
            path.display(),
            error
        )
    })?;
    serde_json::from_str(&contents).map_err(|error| {
        format!(
            "Cannot dispatch review: failed to parse {}: {}",
            path.display(),
            error
        )
    })
}

pub fn update_review_dispatch_tracking(
    state: &mut Value,
    review_dispatch: bool,
) -> Result<Option<String>, String> {
    let previous = read_review_dispatch_consecutive(state)?;
    let next = if review_dispatch {
        previous
            .checked_add(1)
            .ok_or_else(|| "review_dispatch_consecutive overflowed u64".to_string())?
    } else {
        0
    };

    let state_object = state
        .as_object_mut()
        .ok_or_else(|| "docs/state.json root must be an object".to_string())?;
    state_object.insert("review_dispatch_consecutive".to_string(), json!(next));

    if review_dispatch && next >= 3 {
        return Ok(Some(review_dispatch_consecutive_warning(next)));
    }

    Ok(None)
}

fn read_review_dispatch_consecutive(state: &Value) -> Result<u64, String> {
    match state.get("review_dispatch_consecutive") {
        None => Ok(0),
        Some(Value::Number(value)) => value.as_u64().ok_or_else(|| {
            "docs/state.json field review_dispatch_consecutive must be a non-negative integer"
                .to_string()
        }),
        Some(_) => Err(
            "docs/state.json field review_dispatch_consecutive must be a non-negative integer"
                .to_string(),
        ),
    }
}

pub fn review_dispatch_consecutive_warning(count: u64) -> String {
    format!(
        "review-dispatch bypass used {} consecutive cycles — investigate underlying pipeline failure.",
        count
    )
}

pub fn concurrency_warning_message(in_flight: i64) -> String {
    format!(
        "Warning: in-flight dispatches at {} (approaching/exceeding concurrency limit of 2)",
        in_flight
    )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DispatchPatch {
    pub in_flight: i64,
    pub dispatch_log_latest: String,
    pub agent_session: Value,
    pub current_cycle: u64,
}

#[derive(Clone)]
pub struct SealedLastCycleSnapshot {
    summary: Option<serde_json::Value>,
    timestamp: Option<serde_json::Value>,
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
    let in_flight = derive_in_flight_with_new_session(state, &agent_session)?;

    Ok(DispatchPatch {
        in_flight,
        dispatch_log_latest: format_dispatch_log(issue, title, current_cycle),
        agent_session,
        current_cycle,
    })
}

fn derive_in_flight_with_new_session(state: &Value, new_session: &Value) -> Result<i64, String> {
    let mut sessions = state
        .pointer("/agent_sessions")
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| "missing array /agent_sessions in docs/state.json".to_string())?;
    sessions.push(new_session.clone());
    count_in_flight_sessions(&sessions)
}

fn count_in_flight_sessions(sessions: &[Value]) -> Result<i64, String> {
    let mut in_flight = 0_i64;

    for (index, session) in sessions.iter().enumerate() {
        match session.get("status").and_then(Value::as_str) {
            Some("in_flight") | Some("dispatched") => in_flight += 1,
            Some("merged")
            | Some("closed_without_pr")
            | Some("failed")
            | Some("reviewed_awaiting_eva")
            | Some("closed_without_merge")
            | Some("closed") => {}
            Some(status) => {
                return Err(format!(
                    "agent_sessions[{}].status has unsupported value '{}'",
                    index, status
                ))
            }
            None => return Err(format!("agent_sessions[{}].status is missing", index)),
        }
    }

    Ok(in_flight)
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

pub fn apply_dispatch_patch(state: &mut Value, patch: &DispatchPatch) -> Result<bool, String> {
    let cycle_marker = format!("cycle {}", patch.current_cycle);
    let new_issue = patch
        .agent_session
        .get("issue")
        .and_then(Value::as_u64)
        .ok_or_else(|| "agent_session missing 'issue' field".to_string())?;
    let sessions = state
        .pointer_mut("/agent_sessions")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| "missing array /agent_sessions in docs/state.json".to_string())?;
    let mut updated_existing = false;
    let live_duplicate_index = sessions.iter().position(|session| {
        if session.get("issue").and_then(Value::as_u64) != Some(new_issue) {
            return false;
        }
        session
            .get("status")
            .and_then(Value::as_str)
            .map(|status| !is_terminal_status(status))
            .unwrap_or(true)
    });
    if let Some(existing_index) = live_duplicate_index {
        let existing_session = sessions
            .get_mut(existing_index)
            .ok_or_else(|| "agent_sessions entry index out of range".to_string())?;
        if existing_session
            .get("status")
            .and_then(Value::as_str)
            .is_none()
        {
            return Err(format!(
                "agent_sessions already contains an entry for issue #{}; refusing to create duplicate",
                new_issue
            ));
        }
        merge_duplicate_dispatch_session(existing_session, &patch.agent_session, new_issue)?;
        updated_existing = true;
    } else {
        sessions.push(patch.agent_session.clone());
    }
    state
        .as_object_mut()
        .ok_or_else(|| "docs/state.json root must be an object".to_string())?
        .insert(
            "dispatch_log_latest".to_string(),
            json!(patch.dispatch_log_latest),
        );
    state
        .as_object_mut()
        .ok_or_else(|| "docs/state.json root must be an object".to_string())?
        .insert("in_flight_sessions".to_string(), json!(patch.in_flight));
    update_field_inventory_last_refreshed(state, "in_flight_sessions", &cycle_marker)?;
    Ok(updated_existing)
}

pub fn snapshot_sealed_last_cycle(
    state: &serde_json::Value,
    phase: &str,
) -> Option<SealedLastCycleSnapshot> {
    if phase != "close_out" && phase != "complete" {
        return None;
    }
    let last_cycle = state.pointer("/last_cycle")?.as_object()?;
    Some(SealedLastCycleSnapshot {
        summary: last_cycle.get("summary").cloned(),
        timestamp: last_cycle.get("timestamp").cloned(),
    })
}

pub fn restore_sealed_last_cycle(
    state: &mut serde_json::Value,
    snapshot: Option<SealedLastCycleSnapshot>,
) -> Result<(), String> {
    let Some(snapshot) = snapshot else {
        return Ok(());
    };
    let last_cycle = state
        .pointer_mut("/last_cycle")
        .and_then(serde_json::Value::as_object_mut)
        .ok_or_else(|| "missing object /last_cycle in docs/state.json".to_string())?;
    match snapshot.summary {
        Some(summary) => {
            last_cycle.insert("summary".to_string(), summary);
        }
        None => {
            last_cycle.remove("summary");
        }
    }
    match snapshot.timestamp {
        Some(timestamp) => {
            last_cycle.insert("timestamp".to_string(), timestamp);
        }
        None => {
            last_cycle.remove("timestamp");
        }
    }
    Ok(())
}

fn merge_duplicate_dispatch_session(
    existing_session: &mut Value,
    incoming_session: &Value,
    issue: u64,
) -> Result<(), String> {
    let existing = existing_session
        .as_object_mut()
        .ok_or_else(|| "agent_sessions entry must be an object".to_string())?;
    let incoming = incoming_session
        .as_object()
        .ok_or_else(|| "agent_session patch must be an object".to_string())?;

    merge_optional_session_field(existing, incoming, "model");
    merge_optional_session_field(existing, incoming, "title");
    merge_addresses_finding(existing, incoming, issue)?;

    Ok(())
}

fn merge_optional_session_field(
    existing: &mut serde_json::Map<String, Value>,
    incoming: &serde_json::Map<String, Value>,
    field: &str,
) {
    let Some(incoming_value) = incoming.get(field) else {
        return;
    };
    if !session_value_present(incoming_value) {
        return;
    }
    let should_copy = existing
        .get(field)
        .map(|value| !session_value_present(value))
        .unwrap_or(true);
    if should_copy {
        existing.insert(field.to_string(), incoming_value.clone());
    }
}

fn merge_addresses_finding(
    existing: &mut serde_json::Map<String, Value>,
    incoming: &serde_json::Map<String, Value>,
    issue: u64,
) -> Result<(), String> {
    let Some(incoming_value) = incoming.get("addresses_finding") else {
        return Ok(());
    };
    let incoming_ref = incoming_value
        .as_str()
        .map(str::trim)
        .filter(|value| !value.is_empty());
    let Some(incoming_ref) = incoming_ref else {
        return Ok(());
    };
    let existing_ref = existing
        .get("addresses_finding")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty());
    if let Some(existing_ref) = existing_ref {
        if existing_ref != incoming_ref {
            return Err(format!(
                "agent_sessions issue #{} has conflicting addresses_finding values: '{}' vs '{}'",
                issue, existing_ref, incoming_ref
            ));
        }
        return Ok(());
    }
    existing.insert(
        "addresses_finding".to_string(),
        Value::String(incoming_ref.to_string()),
    );
    Ok(())
}

fn session_value_present(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::String(text) => !text.trim().is_empty(),
        Value::Array(values) => !values.is_empty(),
        Value::Object(entries) => !entries.is_empty(),
        _ => true,
    }
}

pub fn sync_last_cycle_summary_after_dispatch(
    state: &mut Value,
    current_cycle: u64,
) -> Result<(), String> {
    let Some(last_cycle) = state
        .pointer_mut("/last_cycle")
        .and_then(Value::as_object_mut)
    else {
        return Ok(());
    };
    if last_cycle.get("number").and_then(Value::as_u64) != Some(current_cycle) {
        return Ok(());
    }
    let Some(summary) = last_cycle.get("summary").and_then(Value::as_str) else {
        return Ok(());
    };
    let Some(updated_summary) = increment_last_cycle_dispatch_count(summary) else {
        return Ok(());
    };
    last_cycle.insert("summary".to_string(), json!(updated_summary));
    last_cycle.insert("timestamp".to_string(), json!(current_utc_timestamp()));
    Ok(())
}

fn increment_last_cycle_dispatch_count(summary: &str) -> Option<String> {
    // Leave custom or unparseable summary formats untouched rather than guessing
    // how to rewrite them during dispatch recording.
    let (dispatches, remainder) = summary
        .split_once(" dispatches, ")
        .or_else(|| summary.split_once(" dispatch, "))?;
    let dispatches = dispatches.trim().parse::<u64>().ok()?;
    let updated_dispatches = dispatches + 1;
    let dispatch_label = if updated_dispatches == 1 {
        "dispatch"
    } else {
        "dispatches"
    };
    Some(format!(
        "{updated_dispatches} {dispatch_label}, {remainder}"
    ))
}

pub fn dispatch_commit_message(issue: u64, current_cycle: u64) -> String {
    format!(
        "state(record-dispatch): #{} dispatched [cycle {}]",
        issue, current_cycle
    )
}

pub fn push_to_origin_master(repo_root: &Path) -> Result<(), String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(["push", "origin", "master"])
        .output()
        .map_err(|error| format!("failed to execute git push: {}", error))?;
    if !output.status.success() {
        return Err(command_failure_message("git push origin master", &output));
    }
    Ok(())
}

fn command_failure_message(command: &str, output: &std::process::Output) -> String {
    let code = output.status.code().map_or_else(
        || "terminated by signal".to_string(),
        |value| value.to_string(),
    );
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    if stderr.is_empty() {
        format!("{command} failed with status {code}")
    } else {
        format!("{command} failed with status {code}: {stderr}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;
    use std::{
        env,
        ffi::OsString,
        fs,
        path::PathBuf,
        sync::{Mutex, OnceLock},
        time::{SystemTime, UNIX_EPOCH},
    };

    fn env_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    struct PathGuard(Option<OsString>);

    impl Drop for PathGuard {
        fn drop(&mut self) {
            if let Some(path) = self.0.take() {
                env::set_var("PATH", path);
            } else {
                env::remove_var("PATH");
            }
        }
    }

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
            "last_cycle": {
                "number": 164,
                "timestamp": "2026-03-07T12:00:00Z",
                "summary": "0 dispatches, 1 merges (PR #700)"
            },
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
    fn count_in_flight_sessions_fails_closed_for_missing_status() {
        let sessions = vec![json!({"issue": 601})];

        let error =
            count_in_flight_sessions(&sessions).expect_err("missing status should fail closed");

        assert!(error.contains("status is missing"));
    }

    #[test]
    fn count_in_flight_sessions_counts_multiple_live_sessions() {
        let sessions = vec![
            json!({"issue": 601, "status": "in_flight"}),
            json!({"issue": 602, "status": "dispatched"}),
            json!({"issue": 603, "status": "merged"}),
        ];

        let count = count_in_flight_sessions(&sessions).expect("count should succeed");

        assert_eq!(count, 2);
    }

    #[test]
    fn count_in_flight_sessions_ignores_terminal_statuses() {
        let sessions = vec![
            json!({"issue": 601, "status": "merged"}),
            json!({"issue": 602, "status": "closed_without_merge"}),
            json!({"issue": 603, "status": "closed_without_pr"}),
            json!({"issue": 604, "status": "reviewed_awaiting_eva"}),
        ];

        let count = count_in_flight_sessions(&sessions).expect("count should succeed");

        assert_eq!(count, 0);
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
    fn update_review_dispatch_tracking_increments_missing_counter() {
        let mut state = sample_state();

        let warning = update_review_dispatch_tracking(&mut state, true)
            .expect("tracking should update cleanly");

        assert_eq!(warning, None);
        assert_eq!(state["review_dispatch_consecutive"], json!(1));
    }

    #[test]
    fn update_review_dispatch_tracking_resets_counter_without_review_dispatch() {
        let mut state = sample_state();
        state["review_dispatch_consecutive"] = json!(2);

        let warning = update_review_dispatch_tracking(&mut state, false)
            .expect("tracking should reset cleanly");

        assert_eq!(warning, None);
        assert_eq!(state["review_dispatch_consecutive"], json!(0));
    }

    #[test]
    fn update_review_dispatch_tracking_warns_on_third_consecutive_bypass() {
        let mut state = sample_state();
        state["review_dispatch_consecutive"] = json!(2);

        let warning = update_review_dispatch_tracking(&mut state, true)
            .expect("tracking should update cleanly");

        assert_eq!(state["review_dispatch_consecutive"], json!(3));
        assert_eq!(warning, Some(review_dispatch_consecutive_warning(3)));
    }

    #[test]
    fn update_review_dispatch_tracking_rejects_invalid_counter_type() {
        let mut state = sample_state();
        state["review_dispatch_consecutive"] = json!("oops");

        let error = update_review_dispatch_tracking(&mut state, true)
            .expect_err("invalid counter type must fail closed");

        assert_eq!(
            error,
            "docs/state.json field review_dispatch_consecutive must be a non-negative integer"
        );
    }

    #[cfg(unix)]
    #[test]
    fn pipeline_check_excludes_step_audit_phases() {
        let _env_guard = env_lock().lock().expect("env lock should not be poisoned");
        let repo_root = temp_repo_root("record-dispatch-pipeline-check");
        let bin_dir = repo_root.join("bin");
        fs::create_dir_all(&bin_dir).expect("bin dir should be created");
        let captured_args = repo_root.join("captured-args.txt");
        let fake_bash = bin_dir.join("bash");
        fs::write(
            &fake_bash,
            format!(
                "#!/bin/bash\nset -euo pipefail\nprintf '%s\\n' \"$@\" > '{}'\n",
                captured_args.display()
            ),
        )
        .expect("fake bash should be written");
        fs::set_permissions(&fake_bash, fs::Permissions::from_mode(0o755))
            .expect("fake bash should be executable");

        let original_path = env::var_os("PATH");
        let mut combined_entries = vec![bin_dir.clone()];
        if let Some(path) = original_path.as_ref() {
            combined_entries.extend(env::split_paths(path));
        }
        let combined_path = env::join_paths(combined_entries).expect("PATH should join");
        let _path_guard = PathGuard(original_path);
        env::set_var("PATH", &combined_path);

        let result = ProcessRunner
            .run_pipeline_check(&repo_root)
            .expect("pipeline-check should execute");

        assert_eq!(result.exit_code, Some(0));
        let recorded_args =
            fs::read_to_string(&captured_args).expect("captured args should be readable");
        assert_eq!(
            recorded_args.lines().collect::<Vec<_>>(),
            PIPELINE_CHECK_ARGS
        );
    }

    #[test]
    fn enforce_pipeline_gate_blocks_stale_review_dispatch_gate() {
        let repo_root = temp_repo_root("record-dispatch-stale-review-gate");
        let mut state = sample_state();
        state["last_cycle"]["number"] = json!(470);
        state["cycle_phase"] = json!({
            "cycle": 470,
            "phase": "complete",
            "phase_entered_at": "2026-03-07T12:00:00Z"
        });
        state["tool_pipeline"] = json!({
            "c5_5_gate": {
                "cycle": 469,
                "status": "PASS",
                "needs_reverify": false
            }
        });
        fs::write(
            repo_root.join("docs/state.json"),
            serde_json::to_string_pretty(&state).expect("state should serialize"),
        )
        .expect("state file should be written");

        let error = enforce_pipeline_gate(&repo_root, true, &ProcessRunner)
            .expect_err("stale review gate should fail closed");

        assert_eq!(
            error,
            PipelineGateError::ReviewDispatchBlocked(
                "Cannot dispatch review: C5.5 gate is stale (gate cycle 469, current cycle 470)"
                    .to_string()
            )
        );
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

        let updated_existing =
            apply_dispatch_patch(&mut state, &patch).expect("patch should apply");

        assert!(!updated_existing);
        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions.len(), 3);
        assert_eq!(state["in_flight_sessions"], json!(1));
        assert_eq!(
            state["dispatch_log_latest"],
            json!("#603 Example dispatch (cycle 164)")
        );
        assert_eq!(
            state["field_inventory"]["fields"]["in_flight_sessions"]["last_refreshed"],
            json!("cycle 164")
        );
        assert_eq!(sessions[2]["issue"], json!(603));
        assert_eq!(sessions[2]["status"], json!("in_flight"));
        assert_eq!(sessions[2]["dispatched_at"], json!("2026-03-07T13:00:00Z"));
    }

    #[test]
    fn apply_dispatch_patch_syncs_in_flight_sessions() {
        let mut state = sample_state();
        state["in_flight_sessions"] = json!(0);
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

        let updated_existing =
            apply_dispatch_patch(&mut state, &patch).expect("patch should apply");

        assert!(!updated_existing);
        assert_eq!(state["in_flight_sessions"], json!(patch.in_flight));
        assert_eq!(
            state["field_inventory"]["fields"]["in_flight_sessions"]["last_refreshed"],
            json!("cycle 164")
        );
    }

    #[test]
    fn sync_last_cycle_summary_after_dispatch_increments_last_cycle_summary_dispatches() {
        let mut state = sample_state();
        let original_timestamp = state["last_cycle"]["timestamp"]
            .as_str()
            .expect("sample state should include last_cycle timestamp")
            .to_string();
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

        let updated_existing =
            apply_dispatch_patch(&mut state, &patch).expect("patch should apply");
        assert!(!updated_existing);
        sync_last_cycle_summary_after_dispatch(&mut state, patch.current_cycle)
            .expect("summary sync should succeed");

        assert_eq!(
            state["last_cycle"]["summary"],
            json!("1 dispatch, 1 merges (PR #700)")
        );
        let updated_timestamp = state["last_cycle"]["timestamp"]
            .as_str()
            .expect("dispatch patch should refresh last_cycle timestamp");
        assert_ne!(updated_timestamp, original_timestamp);
    }

    #[test]
    fn sync_last_cycle_summary_after_dispatch_preserves_unparseable_last_cycle_summary() {
        let mut state = sample_state();
        state["last_cycle"]["summary"] = json!("custom summary");
        let original_last_cycle = state["last_cycle"].clone();
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

        let updated_existing =
            apply_dispatch_patch(&mut state, &patch).expect("patch should apply");
        assert!(!updated_existing);
        sync_last_cycle_summary_after_dispatch(&mut state, patch.current_cycle)
            .expect("summary sync should succeed");

        assert_eq!(state["last_cycle"], original_last_cycle);
    }

    #[test]
    fn sync_last_cycle_summary_after_dispatch_leaves_other_cycle_unchanged() {
        let mut state = sample_state();
        state["last_cycle"]["number"] = json!(163);
        let original_last_cycle = state["last_cycle"].clone();
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

        let updated_existing =
            apply_dispatch_patch(&mut state, &patch).expect("patch should apply");
        assert!(!updated_existing);
        sync_last_cycle_summary_after_dispatch(&mut state, patch.current_cycle)
            .expect("summary sync should succeed");

        assert_eq!(state["last_cycle"], original_last_cycle);
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

        let updated_existing = apply_dispatch_patch(&mut state, &patch)
            .expect("duplicate in-flight issue should be updated in place");

        assert!(updated_existing);
        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        let matching = sessions
            .iter()
            .filter(|session| session.get("issue").and_then(Value::as_u64) == Some(601))
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 2);
        assert_eq!(matching[1]["title"], json!("Live duplicate"));
        assert_eq!(
            state["last_cycle"]["summary"],
            json!("0 dispatches, 1 merges (PR #700)")
        );
    }

    #[test]
    fn apply_dispatch_patch_updates_existing_issue_with_addresses_finding() {
        let mut state = sample_state();
        let model = default_test_model();
        state["agent_sessions"]
            .as_array_mut()
            .expect("agent_sessions array")
            .push(json!({
                "issue": 2301,
                "title": "Post-step recovery",
                "dispatched_at": "2026-03-07T13:00:00Z",
                "model": model.clone(),
                "status": "in_flight"
            }));
        let patch = DispatchPatch {
            in_flight: 1,
            dispatch_log_latest: "#2301 Post-step recovery (cycle 164)".to_string(),
            agent_session: json!({
                "issue": 2301,
                "title": "Post-step recovery",
                "dispatched_at": "2026-03-07T13:00:00Z",
                "model": model,
                "status": "in_flight",
                "addresses_finding": "459:2"
            }),
            current_cycle: 164,
        };

        let updated_existing = apply_dispatch_patch(&mut state, &patch)
            .expect("duplicate recovery should update in place");

        assert!(updated_existing);
        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        let matching = sessions
            .iter()
            .filter(|session| session.get("issue").and_then(Value::as_u64) == Some(2301))
            .collect::<Vec<_>>();
        assert_eq!(matching.len(), 1);
        assert_eq!(matching[0]["issue"], json!(2301));
        assert_eq!(matching[0]["addresses_finding"], json!("459:2"));
        assert_eq!(
            state["last_cycle"]["summary"],
            json!("0 dispatches, 1 merges (PR #700)")
        );
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

        let updated_existing =
            apply_dispatch_patch(&mut state, &patch).expect("terminal duplicate should be allowed");

        assert!(!updated_existing);
        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions.len(), 3);
        assert_eq!(sessions[2]["issue"], json!(601));
        assert_eq!(sessions[2]["status"], json!("in_flight"));
        assert_eq!(state["in_flight_sessions"], json!(1));
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

        let updated_existing = apply_dispatch_patch(&mut state, &patch)
            .expect("duplicate live issue should update the existing row");

        assert!(updated_existing);
        assert_eq!(state["agent_sessions"], original["agent_sessions"]);
        assert_eq!(
            state["last_cycle"]["summary"],
            original["last_cycle"]["summary"]
        );
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
            in_flight: 1,
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
    fn apply_dispatch_patch_sets_top_level_dispatch_log_latest_without_copilot_metrics() {
        let mut state = sample_state();
        let model = default_test_model();
        state
            .as_object_mut()
            .expect("state object")
            .remove("copilot_metrics");
        let patch = build_dispatch_patch(
            &state,
            164,
            603,
            "Example dispatch",
            &model,
            "2026-03-07T13:00:00Z",
        )
        .expect("patch should build");

        let updated_existing =
            apply_dispatch_patch(&mut state, &patch).expect("patch should apply");

        assert!(!updated_existing);
        assert_eq!(
            state["dispatch_log_latest"],
            json!("#603 Example dispatch (cycle 164)")
        );
        assert_eq!(state["in_flight_sessions"], json!(1));
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
