use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Serialize, Deserialize)]
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
    pub next_metric_verification: Option<String>,
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

impl Default for StateJson {
    fn default() -> Self {
        Self {
            schema_version: None,
            schema_status: SchemaStatus::default(),
            agent_sessions: Vec::new(),
            qc_processed: Vec::new(),
            qc_requests_pending: Vec::new(),
            qc_status: BTreeMap::new(),
            blockers: Vec::new(),
            open_questions_for_eva: Vec::new(),
            eva_input_issues: EvaInputIssues::default(),
            typescript_plan: TypescriptPlan::default(),
            release: BTreeMap::new(),
            constructor_refactoring: None,
            copilot_metrics: CopilotMetrics::default(),
            last_cycle: LastCycle::default(),
            last_eva_comment_check: None,
            audit_processed: Vec::new(),
            test_count: TestCount::default(),
            next_metric_verification: None,
            total_schema_types: None,
            total_sub_types: None,
            total_schema_classes: None,
            total_enums: None,
            total_testable_types: None,
            total_standalone_testable_types: None,
            total_testable_types_note: None,
            tool_pipeline: ToolPipeline::default(),
            field_inventory: FieldInventory::default(),
            extra: BTreeMap::new(),
        }
    }
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
    pub note: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
#[serde(default, rename_all = "snake_case")]
pub struct LastCycle {
    pub issue: Option<i64>,
    pub timestamp: Option<String>,
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
        commit_state_json, read_state_value, set_value_at_pointer, update_freshness,
        write_state_value,
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

        fn init_git(&self) {
            run_git(self.path(), ["init"]);
            run_git(self.path(), ["config", "user.name", "State Schema Tests"]);
            run_git(
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

    fn run_git<I, S>(repo_root: &Path, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let output = Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .args(args)
            .output()
            .expect("git command should execute");
        assert!(
            output.status.success(),
            "git command failed: {}",
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
