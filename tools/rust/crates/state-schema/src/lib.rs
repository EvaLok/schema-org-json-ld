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
    pub tool_pipeline: ToolPipeline,
    pub field_inventory: FieldInventory,
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
    pub ignored: u64,
    pub finding_count: u64,
    pub complacency_score: u64,
    pub note: Option<String>,
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
/// Returns last_cycle.number from the state file.
pub fn current_cycle_from_state(repo_root: &Path) -> Result<u64, String> {
    let state = read_state_value(repo_root)?;
    state
        .pointer("/last_cycle/number")
        .and_then(Value::as_u64)
        .ok_or_else(|| "missing /last_cycle/number in state.json".to_string())
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
    pub dispatch_to_pr_rate: Option<String>,
    pub pr_merge_rate: Option<String>,
    pub in_flight: Option<i64>,
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
        commit_state_json, current_cycle_from_state, default_agent_model, read_state_value,
        set_value_at_pointer, update_freshness, write_state_value, StateJson, ToolsConfig,
    };
    use serde_json::{json, Value};
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
    fn current_cycle_from_state_reads_last_cycle_number() {
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
        assert_eq!(error, "missing /last_cycle/number in state.json");
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
