use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const SCHEMA_VERSION: u32 = 1;
pub const TOOLS_CONFIG_RELATIVE_PATH: &str = "tools/config.json";

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(default, rename_all = "snake_case")]
pub struct StateJson {
    pub schema_version: Option<u32>,
    pub schema_status: SchemaStatus,
    pub agent_sessions: Vec<AgentSession>,
    pub qc_processed: Vec<i64>,
    pub qc_requests_pending: Vec<Value>,
    pub qc_status: BTreeMap<String, Value>,
    pub blockers: Vec<Value>,
    pub open_questions_for_eva: Vec<Value>,
    pub eva_input_issues: EvaInputIssues,
    pub typescript_plan: TypescriptPlan,
    pub release: BTreeMap<String, Release>,
    pub constructor_refactoring: Option<ConstructorRefactoring>,
    pub copilot_metrics: CopilotMetrics,
    pub last_cycle: LastCycle,
    pub last_eva_comment_check: Option<String>,
    pub audit_processed: Vec<i64>,
    pub test_count: TestCount,
    pub total_schema_types: Option<i64>,
    pub total_sub_types: Option<i64>,
    pub total_schema_classes: Option<i64>,
    pub total_enums: Option<i64>,
    pub total_testable_types: Option<i64>,
    pub total_standalone_testable_types: Option<i64>,
    pub total_testable_types_note: Option<String>,
    pub review_dispatch_consecutive: Option<u64>,
    pub tool_pipeline: ToolPipeline,
    pub field_inventory: FieldInventory,
    pub cycle_phase: CyclePhase,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

impl StateJson {
    pub fn review_agent(&self) -> Result<ReviewAgent, String> {
        let value = self
            .extra
            .get("review_agent")
            .cloned()
            .ok_or_else(|| "missing field: review_agent".to_string())?;
        serde_json::from_value(value)
            .map_err(|error| format!("failed to parse review_agent from state.json: {}", error))
    }

    pub fn publish_gate(&self) -> Result<PublishGate, String> {
        let value = self
            .extra
            .get("publish_gate")
            .cloned()
            .ok_or_else(|| "missing field: publish_gate".to_string())?;
        serde_json::from_value(value)
            .map_err(|error| format!("failed to parse publish_gate from state.json: {}", error))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(default, rename_all = "snake_case")]
pub struct ReviewAgent {
    pub description: Option<String>,
    pub history: Vec<ReviewHistoryEntry>,
    pub chronic_category_responses: Option<Value>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(default, rename_all = "snake_case")]
pub struct ReviewHistoryEntry {
    pub cycle: u64,
    pub categories: Vec<String>,
    pub actioned: u64,
    pub deferred: u64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub dispatch_created: u64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub actioned_failed: u64,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub verified_resolved: u64,
    pub ignored: u64,
    pub finding_count: u64,
    pub complacency_score: u64,
    pub note: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

fn is_zero(value: &u64) -> bool {
    *value == 0
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
#[serde(default, rename_all = "snake_case")]
pub struct PublishGate {
    pub status: Option<String>,
    pub qc_ack: Option<String>,
    pub validated_commit: Option<String>,
    pub source_diverged: Option<bool>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

pub fn check_version(state: &StateJson) -> Result<(), String> {
    match state.schema_version {
        Some(version) if version == SCHEMA_VERSION => Ok(()),
        Some(version) => Err(format!(
            "State schema version mismatch: expected {}, got {}",
            SCHEMA_VERSION, version
        )),
        None => Err(format!(
            "State schema version missing: expected {}",
            SCHEMA_VERSION
        )),
    }
}

pub fn current_utc_timestamp() -> String {
    Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

pub fn read_state_value(repo_root: &Path) -> Result<Value, String> {
    let state_path = state_json_path(repo_root);
    let content = fs::read_to_string(&state_path)
        .map_err(|error| format!("failed to read {}: {}", state_path.display(), error))?;
    serde_json::from_str::<Value>(&content)
        .map_err(|error| format!("failed to parse {}: {}", state_path.display(), error))
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ToolsConfig {
    pub default_model: String,
}

pub fn read_tools_config(repo_root: &Path) -> Result<ToolsConfig, String> {
    let config_path = tools_config_path(repo_root);
    let content = fs::read_to_string(&config_path)
        .map_err(|error| format!("failed to read {}: {}", config_path.display(), error))?;
    serde_json::from_str::<ToolsConfig>(&content)
        .map_err(|error| format!("failed to parse {}: {}", config_path.display(), error))
}

pub fn default_agent_model(repo_root: &Path) -> Result<String, String> {
    let config = read_tools_config(repo_root)?;
    let model = config.default_model.trim();
    if model.is_empty() {
        return Err(format!(
            "{} must define a non-empty default_model",
            tools_config_path(repo_root).display()
        ));
    }

    Ok(model.to_string())
}

/// Read the current cycle number from state.json.
/// Prefers the active cycle at /cycle_phase/cycle and falls back to
/// /last_cycle/number for older state snapshots.
pub fn current_cycle_from_state(repo_root: &Path) -> Result<u64, String> {
    let state = read_state_value(repo_root)?;
    state
        .pointer("/cycle_phase/cycle")
        .and_then(Value::as_u64)
        .or_else(|| state.pointer("/last_cycle/number").and_then(Value::as_u64))
        .ok_or_else(|| "missing /cycle_phase/cycle or /last_cycle/number in state.json".to_string())
}

pub fn write_state_value(repo_root: &Path, state: &Value) -> Result<(), String> {
    let state_path = state_json_path(repo_root);
    let serialized = serde_json::to_string_pretty(state)
        .map_err(|error| format!("failed to serialize state.json: {}", error))?;
    fs::write(&state_path, format!("{}\n", serialized))
        .map_err(|error| format!("failed to write {}: {}", state_path.display(), error))
}

pub fn commit_state_json(repo_root: &Path, message: &str) -> Result<String, String> {
    let add_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("add")
        .arg("docs/state.json")
        .output()
        .map_err(|error| format!("failed to execute git add: {}", error))?;
    if !add_output.status.success() {
        let stderr = String::from_utf8_lossy(&add_output.stderr)
            .trim()
            .to_string();
        return Err(format!("git add docs/state.json failed: {}", stderr));
    }

    let commit_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("commit")
        .arg("-m")
        .arg(message)
        .output()
        .map_err(|error| format!("failed to execute git commit: {}", error))?;
    if !commit_output.status.success() {
        let stderr = String::from_utf8_lossy(&commit_output.stderr)
            .trim()
            .to_string();
        return Err(format!("git commit failed: {}", stderr));
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("rev-parse")
        .arg("--short=7")
        .arg("HEAD")
        .output()
        .map_err(|error| format!("failed to execute git rev-parse: {}", error))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("git rev-parse --short=7 HEAD failed: {}", stderr));
    }

    let sha = String::from_utf8(output.stdout)
        .map_err(|error| format!("failed to decode git rev-parse output as UTF-8: {}", error))?;
    Ok(sha.trim().to_string())
}

pub fn update_freshness(state: &mut Value, field_name: &str, cycle: u32) -> Result<(), String> {
    let fields = state
        .pointer_mut("/field_inventory/fields")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "missing object: field_inventory.fields".to_string())?;

    let field_entry = fields
        .get_mut(field_name)
        .ok_or_else(|| format!("field_inventory entry not found: {}", field_name))?;

    let field_obj = field_entry
        .as_object_mut()
        .ok_or_else(|| format!("field_inventory entry must be an object: {}", field_name))?;

    field_obj.insert(
        "last_refreshed".to_string(),
        Value::String(format!("cycle {}", cycle)),
    );

    Ok(())
}

/// Valid cycle phase values for the state machine.
pub const VALID_PHASES: &[&str] = &["work", "close_out", "complete"];

/// Transition `cycle_phase` to a new phase, updating `phase_entered_at` and
/// bumping `field_inventory.fields.cycle_phase.last_refreshed`.
///
/// Returns an error if the target phase is not in `VALID_PHASES` or if the
/// required JSON structure is missing.
pub fn transition_cycle_phase(
    state: &mut Value,
    cycle: u64,
    new_phase: &str,
) -> Result<(), String> {
    if !VALID_PHASES.contains(&new_phase) {
        return Err(format!(
            "invalid cycle phase '{}': must be one of {:?}",
            new_phase, VALID_PHASES
        ));
    }

    let timestamp = current_utc_timestamp();

    let cycle_phase = state
        .pointer_mut("/cycle_phase")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "missing object /cycle_phase in docs/state.json".to_string())?;

    cycle_phase.insert("phase".to_string(), Value::String(new_phase.to_string()));
    cycle_phase.insert(
        "phase_entered_at".to_string(),
        Value::String(timestamp.clone()),
    );
    cycle_phase.insert("cycle".to_string(), serde_json::json!(cycle));
    if new_phase == "complete" {
        cycle_phase.insert("completed_at".to_string(), Value::String(timestamp));
    } else {
        cycle_phase.remove("completed_at");
    }

    // Bump field_inventory freshness
    let cycle_marker = format!("cycle {}", cycle);
    let fields = state
        .pointer_mut("/field_inventory/fields")
        .and_then(Value::as_object_mut);

    if let Some(fields) = fields {
        let entry = fields.entry("cycle_phase".to_string()).or_insert_with(|| {
            serde_json::json!({
                "cadence": "every phase transition",
                "note": "Tracks cycle, phase, phase_entered_at, and completed_at."
            })
        });
        if let Some(obj) = entry.as_object_mut() {
            obj.insert("last_refreshed".to_string(), Value::String(cycle_marker));
            obj.entry("note".to_string()).or_insert_with(|| {
                Value::String(
                    "Tracks cycle, phase, phase_entered_at, and completed_at.".to_string(),
                )
            });
        }
    }

    Ok(())
}

pub fn set_value_at_pointer(root: &mut Value, pointer: &str, value: Value) -> Result<bool, String> {
    let segments: Vec<String> = pointer
        .split('/')
        .skip(1)
        .map(|segment| segment.replace("~1", "/").replace("~0", "~"))
        .collect();

    if segments.is_empty() {
        return Err("json pointer must not be empty".to_string());
    }

    let mut cursor = root;
    for segment in &segments[..segments.len() - 1] {
        cursor = cursor
            .as_object_mut()
            .and_then(|object| object.get_mut(segment))
            .ok_or_else(|| {
                format!(
                    "missing object path segment for pointer {}: {}",
                    pointer, segment
                )
            })?;
    }

    let terminal = segments
        .last()
        .expect("segments is guaranteed to be non-empty");
    let object = cursor
        .as_object_mut()
        .ok_or_else(|| format!("target parent is not an object for pointer {}", pointer))?;
    let existing = object.get(terminal).ok_or_else(|| {
        format!(
            "missing target path segment for pointer {}: {}",
            pointer, terminal
        )
    })?;

    if existing == &value {
        return Ok(false);
    }

    object.insert(terminal.clone(), value);
    Ok(true)
}

fn state_json_path(repo_root: &Path) -> PathBuf {
    repo_root.join("docs/state.json")
}

fn tools_config_path(repo_root: &Path) -> PathBuf {
    repo_root.join(TOOLS_CONFIG_RELATIVE_PATH)
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct SchemaStatus {
    pub implemented: Vec<Value>,
    pub quality_fixes: Vec<Value>,
    pub enums_implemented: Vec<String>,
    pub enum_namespace: Option<String>,
    pub in_progress: Vec<Value>,
    pub planned_next: Vec<Value>,
    pub google_rich_results_types: Option<Value>,
    pub remaining_audit_findings: Option<Value>,
    pub property_gap_audit: Option<Value>,
    pub type_classification: Option<TypeClassification>,
    pub phpstan_level: Option<String>,
    pub phpstan_max_assessment: Option<Value>,
    pub directory_layout: Option<Value>,
    pub typescript_stats: Option<TypescriptStats>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct AgentSession {
    pub issue: Option<i64>,
    pub title: Option<String>,
    pub dispatched_at: Option<String>,
    pub model: Option<String>,
    pub status: Option<String>,
    pub pr: Option<i64>,
    pub merged_at: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct TypescriptPlan {
    pub status: Option<String>,
    pub issue: Option<i64>,
    pub qc_coordination_issue: Option<i64>,
    pub plan_version: Option<i64>,
    pub approved_at: Option<String>,
    pub qc_validation_strategy: Option<String>,
    pub eva_decisions: Option<Value>,
    pub preparatory_artifacts: Option<Value>,
    pub audit_enhancements: Option<Value>,
    pub phases: Vec<Value>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct CopilotMetrics {
    pub total_dispatches: Option<i64>,
    pub produced_pr: Option<i64>,
    pub merged: Option<i64>,
    pub dispatch_to_pr_rate: Option<String>,
    pub pr_merge_rate: Option<String>,
    pub in_flight: Option<i64>,
    pub dispatch_log_latest: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct LastCycle {
    pub issue: Option<i64>,
    pub timestamp: Option<String>,
    pub duration_minutes: Option<u64>,
    pub summary: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct TestCount {
    pub php: Option<i64>,
    pub ts: Option<i64>,
    pub total: Option<i64>,
    pub last_verified: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct TypeClassification {
    pub standalone_testable: Option<i64>,
    pub standalone_parity_testable: Option<i64>,
    pub building_block_only: Option<i64>,
    pub building_block: Option<i64>,
    pub enums: Option<i64>,
    pub note: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct ToolPipeline {
    pub eva_directive: Option<i64>,
    pub status: Option<String>,
    pub blocks_publish: Option<bool>,
    pub phases: Vec<Value>,
    pub publish_gate: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct FieldInventory {
    pub description: Option<String>,
    pub fields: BTreeMap<String, Value>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct CyclePhase {
    pub cycle: Option<u64>,
    pub phase: Option<String>,
    pub phase_entered_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct Release {
    pub release_date: Option<String>,
    pub php_version: Option<String>,
    pub ts_version: Option<String>,
    pub notes: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct ConstructorRefactoring {
    pub eva_directive: Option<i64>,
    pub status: Option<String>,
    pub total_classes_converted: Option<i64>,
    pub missed_class_fixed: Option<Value>,
    pub batches: Vec<Value>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct EvaInputIssues {
    pub closed_this_cycle: Vec<i64>,
    pub closed_prior_cycles: Vec<i64>,
    pub remaining_open: Vec<i64>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default, rename_all = "snake_case")]
pub struct TypescriptStats {
    pub schema_types: Option<i64>,
    pub enums: Option<i64>,
    pub core_modules: Option<i64>,
    pub total_modules: Option<i64>,
    pub port_complete: Option<bool>,
    pub port_duration: Option<String>,
    pub prs_merged: Option<i64>,
    pub agent_sessions: Option<i64>,
    pub zero_revision_rounds: Option<bool>,
    pub audit_37_fix: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct QcStatus {
    #[serde(flatten)]
    pub entries: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct Blockers {
    #[serde(flatten)]
    pub entries: BTreeMap<String, Value>,
}

#[cfg(test)]
mod tests {
    use super::{
        commit_state_json, current_cycle_from_state, current_utc_timestamp, default_agent_model,
        read_state_value, set_value_at_pointer, transition_cycle_phase, update_freshness,
        write_state_value, ReviewHistoryEntry, StateJson, ToolsConfig, VALID_PHASES,
    };
    use chrono::DateTime;
    use serde_json::{json, Value};
    use std::collections::BTreeMap;
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TempRepo {
        path: PathBuf,
    }

    impl TempRepo {
        fn new() -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos();
            let path = env::temp_dir().join(format!(
                "state-schema-test-{}-{}",
                std::process::id(),
                unique
            ));
            fs::create_dir_all(path.join("docs")).expect("temp repo should be created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn write_state(&self, state: &Value) {
            write_state_value(&self.path, state).expect("state should be written");
        }

        fn read_state_file(&self) -> String {
            fs::read_to_string(self.path.join("docs/state.json")).expect("state.json should exist")
        }

        fn write_tools_config(&self, content: &str) {
            let config_path = self.path.join("tools/config.json");
            fs::create_dir_all(config_path.parent().expect("config parent should exist"))
                .expect("tools dir should be created");
            fs::write(config_path, content).expect("tools config should be written");
        }

        fn init_git(&self) {
            assert_git_success(self.path(), ["init"]);
            assert_git_success(self.path(), ["config", "user.name", "State Schema Tests"]);
            assert_git_success(
                self.path(),
                ["config", "user.email", "state-schema-tests@example.com"],
            );
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn assert_git_success<I, S>(repo_root: &Path, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let rendered_args: Vec<String> = args
            .into_iter()
            .map(|argument| argument.as_ref().to_string_lossy().into_owned())
            .collect();
        let output = Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .args(&rendered_args)
            .output()
            .expect("git command should execute");
        assert!(
            output.status.success(),
            "git command failed (git -C {} {}): {}",
            repo_root.display(),
            rendered_args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn update_freshness_updates_existing_field() {
        let mut state = json!({
            "field_inventory": {
                "fields": {
                    "copilot_metrics": {
                        "cadence": "every cycle",
                        "last_refreshed": "cycle 120"
                    }
                }
            }
        });

        update_freshness(&mut state, "copilot_metrics", 153).expect("update should succeed");
        assert_eq!(
            state
                .pointer("/field_inventory/fields/copilot_metrics/last_refreshed")
                .and_then(|value| value.as_str()),
            Some("cycle 153")
        );
    }

    #[test]
    fn review_history_entry_deserializes_without_new_fields() {
        let entry: ReviewHistoryEntry = serde_json::from_value(json!({
            "cycle": 162,
            "categories": ["data-integrity"],
            "actioned": 1,
            "deferred": 1,
            "ignored": 5,
            "finding_count": 7,
            "complacency_score": 2
        }))
        .expect("legacy history entry should deserialize");

        assert_eq!(entry.dispatch_created, 0);
        assert_eq!(entry.actioned_failed, 0);
        assert_eq!(entry.verified_resolved, 0);
    }

    #[test]
    fn review_history_entry_serialization_omits_zero_new_fields() {
        let entry = ReviewHistoryEntry {
            cycle: 162,
            categories: vec!["data-integrity".to_string()],
            actioned: 1,
            deferred: 1,
            dispatch_created: 0,
            actioned_failed: 0,
            verified_resolved: 0,
            ignored: 5,
            finding_count: 7,
            complacency_score: 2,
            note: None,
            extra: BTreeMap::new(),
        };

        let value = serde_json::to_value(&entry).expect("history entry should serialize");
        let object = value
            .as_object()
            .expect("history entry should be an object");
        assert!(!object.contains_key("dispatch_created"));
        assert!(!object.contains_key("actioned_failed"));
        assert!(!object.contains_key("verified_resolved"));
    }

    #[test]
    fn review_history_entry_serialization_includes_non_zero_new_fields() {
        let entry = ReviewHistoryEntry {
            cycle: 162,
            categories: vec!["data-integrity".to_string()],
            actioned: 1,
            deferred: 1,
            dispatch_created: 2,
            actioned_failed: 1,
            verified_resolved: 1,
            ignored: 2,
            finding_count: 7,
            complacency_score: 2,
            note: None,
            extra: BTreeMap::new(),
        };

        let value = serde_json::to_value(&entry).expect("history entry should serialize");
        let object = value
            .as_object()
            .expect("history entry should be an object");
        assert_eq!(object.get("dispatch_created"), Some(&json!(2)));
        assert_eq!(object.get("actioned_failed"), Some(&json!(1)));
        assert_eq!(object.get("verified_resolved"), Some(&json!(1)));
    }

    #[test]
    fn default_agent_model_reads_tools_config() {
        let repo = TempRepo::new();
        let config = ToolsConfig {
            default_model: "gpt-5.4".to_string(),
        };
        repo.write_tools_config(&serde_json::to_string(&config).expect("config should serialize"));

        let model = default_agent_model(repo.path()).expect("default model should load");
        assert_eq!(model, "gpt-5.4");
    }

    #[test]
    fn default_agent_model_rejects_empty_config_value() {
        let repo = TempRepo::new();
        repo.write_tools_config(r#"{"default_model":"   "}"#);

        let error = default_agent_model(repo.path()).expect_err("empty default should fail");
        assert!(error.contains("must define a non-empty default_model"));
    }

    #[test]
    fn update_freshness_returns_error_for_missing_field() {
        let mut state = json!({
            "field_inventory": {
                "fields": {}
            }
        });

        let error = update_freshness(&mut state, "total_enums", 153).expect_err("must fail");
        assert!(error.contains("field_inventory entry not found"));
    }

    #[test]
    fn update_freshness_uses_cycle_marker_format() {
        let mut state = json!({
            "field_inventory": {
                "fields": {
                    "test_count": {
                        "cadence": "every merge",
                        "last_refreshed": "cycle 120"
                    }
                }
            }
        });

        update_freshness(&mut state, "test_count", 153).expect("update should succeed");
        assert_eq!(
            state
                .pointer("/field_inventory/fields/test_count/last_refreshed")
                .and_then(|value| value.as_str()),
            Some("cycle 153")
        );
    }

    #[test]
    fn update_freshness_supports_dotted_field_names() {
        let mut state = json!({
            "field_inventory": {
                "fields": {
                    "schema_status.typescript_stats": {
                        "cadence": "every cycle",
                        "last_refreshed": "cycle 120"
                    }
                }
            }
        });

        update_freshness(&mut state, "schema_status.typescript_stats", 153)
            .expect("update should succeed");
        assert_eq!(
            state
                .pointer("/field_inventory/fields/schema_status.typescript_stats/last_refreshed")
                .and_then(|value| value.as_str()),
            Some("cycle 153")
        );
    }

    #[test]
    fn set_value_at_pointer_updates_existing_nested_path() {
        let mut state = json!({
            "test_count": {
                "php": 425
            }
        });

        let changed = set_value_at_pointer(&mut state, "/test_count/php", json!(430))
            .expect("path should exist");
        assert!(changed);
        assert_eq!(
            state
                .pointer("/test_count/php")
                .and_then(|value| value.as_i64()),
            Some(430)
        );
    }

    #[test]
    fn set_value_at_pointer_requires_existing_path() {
        let mut state = json!({
            "test_count": {
                "php": 425
            }
        });

        let error = set_value_at_pointer(&mut state, "/test_count/missing", json!(1))
            .expect_err("missing path should fail");
        assert!(error.contains("missing target path segment"));
    }

    #[test]
    fn set_value_at_pointer_returns_false_when_value_is_unchanged() {
        let mut state = json!({
            "test_count": {
                "php": 425
            }
        });

        let changed = set_value_at_pointer(&mut state, "/test_count/php", json!(425))
            .expect("path should exist");
        assert!(!changed);
    }

    #[test]
    fn set_value_at_pointer_supports_tilde_escapes() {
        let mut state = json!({
            "paths": {
                "with~tilde/and/slash": "before"
            }
        });

        let changed =
            set_value_at_pointer(&mut state, "/paths/with~0tilde~1and~1slash", json!("after"))
                .expect("escaped path should resolve");
        assert!(changed);
        assert_eq!(
            state
                .pointer("/paths/with~0tilde~1and~1slash")
                .and_then(|value| value.as_str()),
            Some("after")
        );
    }

    #[test]
    fn read_write_state_value_round_trip_preserves_json_and_trailing_newline() {
        let repo = TempRepo::new();
        let state = json!({
            "last_cycle": {"number": 166},
            "copilot_metrics": {"in_flight": 2}
        });

        repo.write_state(&state);

        let content = repo.read_state_file();
        assert!(content.ends_with('\n'));
        let loaded = read_state_value(repo.path()).expect("state should round-trip");
        assert_eq!(loaded, state);
    }

    #[test]
    fn read_state_value_errors_when_file_is_missing() {
        let repo = TempRepo::new();

        let error = read_state_value(repo.path()).expect_err("missing state file must fail");
        assert!(error.contains("failed to read"));
        assert!(error.contains("docs/state.json"));
    }

    #[test]
    fn current_cycle_from_state_prefers_cycle_phase_cycle() {
        let repo = TempRepo::new();
        repo.write_state(&json!({
            "last_cycle": {"number": 166},
            "cycle_phase": {"cycle": 167}
        }));

        let cycle = current_cycle_from_state(repo.path()).expect("cycle should load from state");
        assert_eq!(cycle, 167);
    }

    #[test]
    fn current_cycle_from_state_falls_back_to_last_cycle_number() {
        let repo = TempRepo::new();
        repo.write_state(&json!({"last_cycle": {"number": 166}}));

        let cycle = current_cycle_from_state(repo.path()).expect("cycle should load from state");
        assert_eq!(cycle, 166);
    }

    #[test]
    fn current_cycle_from_state_requires_last_cycle_number() {
        let repo = TempRepo::new();
        repo.write_state(&json!({"last_cycle": {}}));

        let error = current_cycle_from_state(repo.path()).expect_err("missing cycle must fail");
        assert_eq!(
            error,
            "missing /cycle_phase/cycle or /last_cycle/number in state.json"
        );
    }

    #[test]
    fn last_cycle_deserializes_duration_minutes_when_present() {
        let state: StateJson = serde_json::from_value(json!({
            "last_cycle": {
                "timestamp": "2026-03-07T21:05:16Z",
                "duration_minutes": 47
            }
        }))
        .expect("state should deserialize");

        assert_eq!(state.last_cycle.duration_minutes, Some(47));
    }

    #[test]
    fn publish_gate_deserializes_from_state_extra() {
        let state: StateJson = serde_json::from_value(json!({
            "publish_gate": {
                "status": "published",
                "qc_ack": "EvaLok/schema-org-json-ld-qc#225",
                "validated_commit": "ea8ffff",
                "source_diverged": false
            }
        }))
        .expect("state should deserialize");

        let publish_gate = state
            .publish_gate()
            .expect("publish gate should deserialize");
        assert_eq!(publish_gate.status.as_deref(), Some("published"));
        assert_eq!(
            publish_gate.qc_ack.as_deref(),
            Some("EvaLok/schema-org-json-ld-qc#225")
        );
        assert_eq!(publish_gate.validated_commit.as_deref(), Some("ea8ffff"));
        assert_eq!(publish_gate.source_diverged, Some(false));
    }

    #[test]
    fn cycle_phase_defaults_to_empty() {
        let state: StateJson = serde_json::from_value(json!({})).expect("state should deserialize");
        assert!(state.cycle_phase.cycle.is_none());
        assert!(state.cycle_phase.phase.is_none());
        assert!(state.cycle_phase.phase_entered_at.is_none());
        assert!(state.cycle_phase.completed_at.is_none());
    }

    #[test]
    fn cycle_phase_deserializes_supported_fields() {
        let state: StateJson = serde_json::from_value(json!({
            "cycle_phase": {
                "cycle": 219,
                "phase": "close_out",
                "phase_entered_at": "2026-03-10T15:00:00Z"
            }
        }))
        .expect("state should deserialize");

        assert_eq!(state.cycle_phase.cycle, Some(219));
        assert_eq!(state.cycle_phase.phase.as_deref(), Some("close_out"));
        assert_eq!(
            state.cycle_phase.phase_entered_at.as_deref(),
            Some("2026-03-10T15:00:00Z")
        );
        assert!(state.cycle_phase.completed_at.is_none());
    }

    #[test]
    fn transition_cycle_phase_updates_phase_and_freshness() {
        let mut state = json!({
            "cycle_phase": {
                "cycle": 219,
                "phase": "work",
                "phase_entered_at": "2026-03-10T12:00:00Z"
            },
            "field_inventory": {
                "fields": {
                    "cycle_phase": {
                        "cadence": "every phase transition",
                        "last_refreshed": "cycle 218"
                    }
                }
            }
        });

        transition_cycle_phase(&mut state, 219, "close_out").expect("transition should succeed");

        assert_eq!(
            state.pointer("/cycle_phase/phase"),
            Some(&json!("close_out"))
        );
        assert_eq!(state.pointer("/cycle_phase/cycle"), Some(&json!(219)));
        // phase_entered_at should be updated (not the old value)
        assert_ne!(
            state
                .pointer("/cycle_phase/phase_entered_at")
                .and_then(Value::as_str),
            Some("2026-03-10T12:00:00Z")
        );
        assert!(state.pointer("/cycle_phase/completed_at").is_none());
        // freshness should be bumped
        assert_eq!(
            state
                .pointer("/field_inventory/fields/cycle_phase/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 219")
        );
        assert_eq!(
            state
                .pointer("/field_inventory/fields/cycle_phase/note")
                .and_then(Value::as_str),
            Some("Tracks cycle, phase, phase_entered_at, and completed_at.")
        );
    }

    #[test]
    fn transition_cycle_phase_to_complete_succeeds() {
        let mut state = json!({
            "cycle_phase": {
                "cycle": 219,
                "phase": "close_out",
                "phase_entered_at": "2026-03-10T13:00:00Z"
            },
            "field_inventory": {
                "fields": {
                    "cycle_phase": {
                        "cadence": "every phase transition",
                        "last_refreshed": "cycle 218"
                    }
                }
            }
        });

        transition_cycle_phase(&mut state, 219, "complete").expect("transition should succeed");
        assert_eq!(
            state.pointer("/cycle_phase/phase"),
            Some(&json!("complete"))
        );
        let completed_at = state
            .pointer("/cycle_phase/completed_at")
            .and_then(Value::as_str)
            .expect("completed_at should be set when phase becomes complete");
        let phase_entered_at = state
            .pointer("/cycle_phase/phase_entered_at")
            .and_then(Value::as_str)
            .expect("phase_entered_at should be set");
        assert_eq!(completed_at, phase_entered_at);
        assert_eq!(
            state
                .pointer("/field_inventory/fields/cycle_phase/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 219")
        );
    }

    #[test]
    fn transition_cycle_phase_clears_completed_at_outside_complete() {
        let mut state = json!({
            "cycle_phase": {
                "cycle": 219,
                "phase": "complete",
                "phase_entered_at": "2026-03-10T13:00:00Z",
                "completed_at": "2026-03-10T13:05:00Z"
            },
            "field_inventory": {
                "fields": {
                    "cycle_phase": {
                        "cadence": "every phase transition",
                        "last_refreshed": "cycle 218"
                    }
                }
            }
        });

        transition_cycle_phase(&mut state, 219, "work").expect("transition should succeed");

        assert_eq!(state.pointer("/cycle_phase/phase"), Some(&json!("work")));
        assert!(state.pointer("/cycle_phase/completed_at").is_none());
    }

    #[test]
    fn transition_cycle_phase_rejects_invalid_phase() {
        let mut state = json!({
            "cycle_phase": {
                "cycle": 219,
                "phase": "work"
            },
            "field_inventory": { "fields": {} }
        });

        let error = transition_cycle_phase(&mut state, 219, "bogus")
            .expect_err("invalid phase should fail");
        assert!(error.contains("invalid cycle phase"));
        assert!(error.contains("bogus"));
    }

    #[test]
    fn transition_cycle_phase_creates_freshness_entry_when_missing() {
        let mut state = json!({
            "cycle_phase": {
                "cycle": 219,
                "phase": "work"
            },
            "field_inventory": { "fields": {} }
        });

        transition_cycle_phase(&mut state, 219, "close_out")
            .expect("transition should succeed even without pre-existing freshness entry");
        assert_eq!(
            state
                .pointer("/field_inventory/fields/cycle_phase/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 219")
        );
    }

    #[test]
    fn valid_phases_contains_all_state_machine_values() {
        let expected = vec!["work", "close_out", "complete"];
        assert_eq!(VALID_PHASES, expected.as_slice());
    }

    #[test]
    fn cycle_phase_round_trips_through_serialization() {
        let input = json!({
            "cycle_phase": {
                "cycle": 220,
                "phase": "work",
                "phase_entered_at": "2026-03-10T16:00:00Z",
                "completed_at": "2026-03-10T16:30:00Z"
            }
        });
        let state: StateJson = serde_json::from_value(input).expect("state should deserialize");
        let serialized = serde_json::to_value(&state).expect("state should serialize");
        assert_eq!(serialized.pointer("/cycle_phase/cycle"), Some(&json!(220)));
        assert_eq!(
            serialized.pointer("/cycle_phase/phase"),
            Some(&json!("work"))
        );
        assert_eq!(
            serialized.pointer("/cycle_phase/phase_entered_at"),
            Some(&json!("2026-03-10T16:00:00Z"))
        );
        assert_eq!(
            serialized.pointer("/cycle_phase/completed_at"),
            Some(&json!("2026-03-10T16:30:00Z"))
        );
    }

    #[test]
    fn copilot_metrics_deserializes_dispatch_log_latest() {
        let state: StateJson = serde_json::from_value(json!({
            "copilot_metrics": {
                "dispatch_log_latest": "#873 Review findings follow-up (cycle 202)"
            }
        }))
        .expect("state should deserialize");

        assert_eq!(
            state.copilot_metrics.dispatch_log_latest.as_deref(),
            Some("#873 Review findings follow-up (cycle 202)")
        );
    }

    #[test]
    fn copilot_metrics_deserializes_summary_fields() {
        let state: StateJson = serde_json::from_value(json!({
            "copilot_metrics": {
                "total_dispatches": 45,
                "merged": 40,
                "pr_merge_rate": "88.9%",
                "in_flight": 3
            }
        }))
        .expect("state should deserialize");

        assert_eq!(state.copilot_metrics.total_dispatches, Some(45));
        assert_eq!(state.copilot_metrics.merged, Some(40));
        assert_eq!(
            state.copilot_metrics.pr_merge_rate.as_deref(),
            Some("88.9%")
        );
        assert_eq!(state.copilot_metrics.in_flight, Some(3));
    }

    #[test]
    fn current_utc_timestamp_returns_rfc3339_value() {
        let timestamp = current_utc_timestamp();
        assert!(DateTime::parse_from_rfc3339(&timestamp).is_ok());
        assert!(timestamp.ends_with('Z'));
    }

    #[test]
    fn commit_state_json_commits_state_file_and_returns_short_sha() {
        let repo = TempRepo::new();
        repo.init_git();
        repo.write_state(&json!({"last_cycle": {"number": 166}}));

        let sha =
            commit_state_json(repo.path(), "state(test): update").expect("commit should succeed");
        assert_eq!(sha.len(), 7);
        assert!(sha.chars().all(|character| character.is_ascii_hexdigit()));

        let output = Command::new("git")
            .arg("-C")
            .arg(repo.path())
            .args(["log", "-1", "--pretty=%B"])
            .output()
            .expect("git log should execute");
        assert!(output.status.success());
        assert_eq!(
            String::from_utf8_lossy(&output.stdout).trim(),
            "state(test): update"
        );
    }
}
