use clap::Parser;
use serde_json::Value;
use state_schema::{SchemaStatus, StateJson, TypescriptPlan};
use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;
use std::process;

/// Field inventory completeness check for state.json.
///
/// Per audit #87: programmatic comparison of state.json fields vs field_inventory entries.
/// Replaces the jq-based check-field-inventory.jq that couldn't run in the orchestrator sandbox.
#[derive(Parser)]
#[command(name = "check-field-inventory")]
struct Cli {
    /// Path to the repository root
    #[arg(long)]
    repo_root: PathBuf,
}

/// Top-level keys excluded from tracking (append-only or static).
const EXCLUDED_TOP_LEVEL: &[&str] = &[
    "schema_version",
    "agent_sessions",
    "release",
    "field_inventory",
    "constructor_refactoring",
];

/// schema_status sub-keys excluded from tracking (append-only or static).
const EXCLUDED_SCHEMA_STATUS: &[&str] = &[
    "implemented",
    "quality_fixes",
    "enums_implemented",
    "enum_namespace",
    "directory_layout",
];

/// typescript_plan sub-keys excluded from tracking (static/historical).
const EXCLUDED_TYPESCRIPT_PLAN: &[&str] = &[
    "eva_decisions",
    "preparatory_artifacts",
    "audit_enhancements",
    "phases",
    "plan_version",
    "approved_at",
    "issue",
    "qc_coordination_issue",
    "qc_validation_strategy",
];

fn main() {
    let cli = Cli::parse();

    let state = read_state_file(&cli.repo_root.join("docs/state.json"));

    let inventoried = get_inventoried_fields(&state);
    let mut gaps: BTreeSet<String> = BTreeSet::new();

    // Mutable top-level fields (excluding append-only and static)
    for key in state_top_level_keys(&state) {
        if EXCLUDED_TOP_LEVEL.contains(&key.as_str()) {
            continue;
        }
        if !is_inventoried(&key, &inventoried) {
            gaps.insert(key);
        }
    }

    // Mutable schema_status sub-fields
    // Some sub-fields share names with top-level keys (e.g., phpstan_level,
    // type_classification, typescript_stats). The inventory may track them
    // under either the bare name or the schema_status.* prefix.
    for key in schema_status_keys(&state) {
        if EXCLUDED_SCHEMA_STATUS.contains(&key.as_str()) {
            continue;
        }
        let prefixed = format!("schema_status.{}", key);
        if !inventoried.contains(&prefixed) && !inventoried.contains(key.as_str()) {
            gaps.insert(prefixed);
        }
    }

    // Mutable typescript_plan sub-fields
    for key in typescript_plan_keys(&state) {
        if EXCLUDED_TYPESCRIPT_PLAN.contains(&key.as_str()) {
            continue;
        }
        let path = format!("typescript_plan.{}", key);
        if !inventoried.contains(&path) {
            gaps.insert(path);
        }
    }

    // Mutable eva_input_issues sub-fields
    let eva_check = "eva_input_issues.closed_this_cycle".to_string();
    if !inventoried.contains(&eva_check) {
        gaps.insert(eva_check);
    }

    if gaps.is_empty() {
        println!(
            "PASS: All mutable fields have field_inventory entries ({} tracked)",
            inventoried.len()
        );
    } else {
        println!(
            "GAPS FOUND: {} mutable field(s) without inventory entries:",
            gaps.len()
        );
        for gap in &gaps {
            println!("  - {}", gap);
        }
        println!();
        println!("Currently inventoried: {} fields", inventoried.len());
        process::exit(1);
    }
}

/// Extract the set of field paths from .field_inventory.fields keys.
fn read_state_file(path: &PathBuf) -> StateJson {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading {}: {}", path.display(), e);
            process::exit(1);
        }
    };

    match serde_json::from_str::<StateJson>(&content) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing JSON ({}): {}", path.display(), e);
            process::exit(1);
        }
    }
}

fn get_inventoried_fields(state: &StateJson) -> BTreeSet<String> {
    state.field_inventory.fields.keys().cloned().collect()
}

/// Check if a top-level key is inventoried (exact match or has sub-field entries).
fn is_inventoried(key: &str, inventoried: &BTreeSet<String>) -> bool {
    if inventoried.contains(key) {
        return true;
    }
    let prefix = format!("{}.", key);
    inventoried.iter().any(|f| f.starts_with(&prefix))
}

fn state_top_level_keys(state: &StateJson) -> BTreeSet<String> {
    let mut keys = object_keys_from_serialized(StateJson::default());
    keys.extend(state.extra.keys().cloned());
    keys
}

fn schema_status_keys(state: &StateJson) -> BTreeSet<String> {
    let mut keys = object_keys_from_serialized(SchemaStatus::default());
    keys.extend(state.schema_status.extra.keys().cloned());
    keys
}

fn typescript_plan_keys(state: &StateJson) -> BTreeSet<String> {
    let mut keys = object_keys_from_serialized(TypescriptPlan::default());
    keys.extend(state.typescript_plan.extra.keys().cloned());
    keys
}

fn object_keys_from_serialized<T: serde::Serialize>(value: T) -> BTreeSet<String> {
    let serialized = match serde_json::to_value(value) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error serializing state schema defaults: {}", e);
            process::exit(1);
        }
    };
    object_keys_from_value(serialized)
}

fn object_keys_from_value(value: Value) -> BTreeSet<String> {
    value
        .as_object()
        .map(|obj| obj.keys().cloned().collect())
        .unwrap_or_default()
}
