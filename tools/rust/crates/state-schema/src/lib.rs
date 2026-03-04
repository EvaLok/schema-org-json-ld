use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeMap;

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
