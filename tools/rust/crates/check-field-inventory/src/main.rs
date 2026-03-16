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
    /// Current cycle number for cadence-based staleness checks
    #[arg(long)]
    cycle: Option<u64>,
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

    let stale = cli
        .cycle
        .map(|current_cycle| detect_stale_fields(&state, current_cycle))
        .unwrap_or_default();

    if gaps.is_empty() && stale.is_empty() {
        println!(
            "PASS: All mutable fields have field_inventory entries ({} tracked)",
            inventoried.len()
        );
    } else {
        if !gaps.is_empty() {
            println!(
                "GAPS FOUND: {} mutable field(s) without inventory entries:",
                gaps.len()
            );
            for gap in &gaps {
                println!("  - {}", gap);
            }
        }
        if !stale.is_empty() {
            if !gaps.is_empty() {
                println!();
            }
            println!(
                "STALE FIELD INVENTORY: {} field(s) exceed cadence thresholds:",
                stale.len()
            );
            for field in &stale {
                println!(
                    "  - STALE: {} (cadence: \"{}\", tier: {}, last_refreshed: cycle {}, gap: {} cycles, max allowed: {})",
                    field.name,
                    field.cadence,
                    field.tier,
                    field.last_refreshed_cycle,
                    field.gap,
                    field.max_allowed_gap
                );
            }
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

#[derive(Debug)]
struct StaleField {
    name: String,
    cadence: String,
    tier: &'static str,
    last_refreshed_cycle: u64,
    gap: u64,
    max_allowed_gap: u64,
}

fn detect_stale_fields(state: &StateJson, current_cycle: u64) -> Vec<StaleField> {
    state
        .field_inventory
        .fields
        .iter()
        .filter_map(|(name, entry)| {
            let cadence = entry
                .get("cadence")
                .and_then(Value::as_str)
                .unwrap_or("default")
                .to_string();
            let (tier, max_allowed_gap) = cadence_threshold(&cadence);
            let last_refreshed_cycle = entry
                .get("last_refreshed")
                .and_then(Value::as_str)
                .and_then(first_number);
            let gap = match last_refreshed_cycle {
                Some(cycle) => current_cycle.saturating_sub(cycle),
                None => max_allowed_gap + 1,
            };
            if gap > max_allowed_gap {
                Some(StaleField {
                    name: name.clone(),
                    cadence,
                    tier,
                    last_refreshed_cycle: last_refreshed_cycle.unwrap_or(0),
                    gap,
                    max_allowed_gap,
                })
            } else {
                None
            }
        })
        .collect()
}

fn cadence_threshold(cadence: &str) -> (&'static str, u64) {
    let normalized = cadence.to_ascii_lowercase();
    if normalized.contains("every phase transition") {
        ("per-phase-transition", 2)
    } else if normalized.contains("every cycle") || normalized.contains("per cycle") {
        ("per-cycle", 2)
    } else if let Some(number) = first_number(&normalized) {
        ("periodic", number + 1)
    } else if normalized.contains("after") {
        ("after-change", 10)
    } else {
        ("default", 5)
    }
}

/// Returns the first contiguous numeric segment in a string, if present.
fn first_number(value: &str) -> Option<u64> {
    let digits: String = value
        .chars()
        .skip_while(|c| !c.is_ascii_digit())
        .take_while(|c| c.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse::<u64>().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn cadence_threshold_classifies_per_phase_transition() {
        assert_eq!(
            cadence_threshold("every phase transition"),
            ("per-phase-transition", 2)
        );
    }

    #[test]
    fn cadence_threshold_classifies_per_cycle() {
        assert_eq!(cadence_threshold("every cycle"), ("per-cycle", 2));
        assert_eq!(cadence_threshold("per cycle"), ("per-cycle", 2));
    }

    #[test]
    fn cadence_threshold_classifies_periodic() {
        assert_eq!(cadence_threshold("every 5 cycles"), ("periodic", 6));
    }

    #[test]
    fn cadence_threshold_classifies_after_change() {
        assert_eq!(cadence_threshold("after changes"), ("after-change", 10));
        assert_eq!(
            cadence_threshold("after schema class additions"),
            ("after-change", 10)
        );
    }

    #[test]
    fn cadence_threshold_uses_default_for_unknown() {
        assert_eq!(cadence_threshold("on demand"), ("default", 5));
    }

    #[test]
    fn first_number_extracts_numeric_value() {
        assert_eq!(first_number("cycle 158"), Some(158));
        assert_eq!(first_number("every 12 cycles"), Some(12));
        assert_eq!(first_number("no digits"), None);
    }

    #[test]
    fn detect_stale_fields_respects_tier_thresholds() {
        let mut state = StateJson::default();
        state.field_inventory.fields.insert(
            "after-ok".to_string(),
            json!({"cadence": "after changes", "last_refreshed": "cycle 148"}),
        );
        state.field_inventory.fields.insert(
            "after-stale".to_string(),
            json!({"cadence": "after changes", "last_refreshed": "cycle 147"}),
        );
        state.field_inventory.fields.insert(
            "per-cycle-ok".to_string(),
            json!({"cadence": "every cycle", "last_refreshed": "cycle 156"}),
        );
        state.field_inventory.fields.insert(
            "per-cycle-stale".to_string(),
            json!({"cadence": "every cycle", "last_refreshed": "cycle 155"}),
        );
        state.field_inventory.fields.insert(
            "periodic-ok".to_string(),
            json!({"cadence": "every 5 cycles", "last_refreshed": "cycle 152"}),
        );
        state.field_inventory.fields.insert(
            "periodic-stale".to_string(),
            json!({"cadence": "every 5 cycles", "last_refreshed": "cycle 151"}),
        );
        state.field_inventory.fields.insert(
            "phase-ok".to_string(),
            json!({"cadence": "every phase transition", "last_refreshed": "cycle 157"}),
        );
        state.field_inventory.fields.insert(
            "phase-stale".to_string(),
            json!({"cadence": "every phase transition", "last_refreshed": "cycle 155"}),
        );

        let stale = detect_stale_fields(&state, 158);
        let stale_names = stale
            .iter()
            .map(|field| field.name.as_str())
            .collect::<Vec<_>>();

        assert_eq!(
            stale_names,
            vec![
                "after-stale",
                "per-cycle-stale",
                "periodic-stale",
                "phase-stale"
            ]
        );
    }

    #[test]
    fn detect_stale_fields_marks_missing_last_refreshed_as_stale() {
        let mut state = StateJson::default();
        state.field_inventory.fields.insert(
            "missing-last".to_string(),
            json!({"cadence": "after changes"}),
        );

        let stale = detect_stale_fields(&state, 158);
        assert_eq!(stale.len(), 1);
        assert_eq!(stale[0].name, "missing-last");
        assert_eq!(stale[0].tier, "after-change");
        assert_eq!(stale[0].gap, 11);
    }
}
