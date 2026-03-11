use clap::Parser;
use serde::Serialize;
use serde_json::Value;
use state_schema::{read_state_value, update_freshness, write_state_value, StateJson};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "refresh-field-inventory")]
struct Cli {
    /// Current cycle number
    #[arg(long)]
    cycle: u64,

    /// Show what would be updated without writing
    #[arg(long)]
    dry_run: bool,

    /// Repository root
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct RefreshFailure {
    field: String,
    reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct RefreshSummary {
    stale_fields: Vec<String>,
    refreshed_fields: Vec<String>,
    failed_fields: Vec<RefreshFailure>,
    dry_run: bool,
}

fn main() {
    let cli = Cli::parse();
    match refresh_field_inventory(&cli.repo_root, cli.cycle, cli.dry_run) {
        Ok(summary) => {
            print_summary(&summary);
            if summary.failed_fields.is_empty() {
                std::process::exit(0);
            }

            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn refresh_field_inventory(
    repo_root: &Path,
    cycle: u64,
    dry_run: bool,
) -> Result<RefreshSummary, String> {
    let cycle_u32 = u32::try_from(cycle).map_err(|_| "cycle must fit in u32 range".to_string())?;
    let mut state_value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(state_value.clone())
        .map_err(|error| format!("failed to parse docs/state.json: {}", error))?;
    let stale_fields = detect_stale_fields(&state, cycle);
    let mut refreshed_fields = Vec::new();
    let mut failed_fields = Vec::new();

    for field in &stale_fields {
        match verify_field(repo_root, &state, field) {
            Ok(()) => {
                if dry_run {
                    refreshed_fields.push(field.clone());
                    continue;
                }

                match update_freshness(&mut state_value, field, cycle_u32) {
                    Ok(()) => refreshed_fields.push(field.clone()),
                    Err(reason) => failed_fields.push(RefreshFailure {
                        field: field.clone(),
                        reason,
                    }),
                }
            }
            Err(reason) => failed_fields.push(RefreshFailure {
                field: field.clone(),
                reason,
            }),
        }
    }

    if !dry_run && !refreshed_fields.is_empty() {
        write_state_value(repo_root, &state_value)?;
    }

    Ok(RefreshSummary {
        stale_fields,
        refreshed_fields,
        failed_fields,
        dry_run,
    })
}

fn print_summary(summary: &RefreshSummary) {
    println!("Field inventory refresh summary:");
    println!("  stale fields: {}", summary.stale_fields.len());
    if summary.dry_run {
        println!("  would refresh: {}", summary.refreshed_fields.len());
    } else {
        println!("  refreshed: {}", summary.refreshed_fields.len());
    }
    println!("  failed: {}", summary.failed_fields.len());

    if !summary.stale_fields.is_empty() {
        println!("  stale field list:");
        for field in &summary.stale_fields {
            println!("    - {}", field);
        }
    }

    if !summary.refreshed_fields.is_empty() {
        if summary.dry_run {
            println!("  would refresh field list:");
        } else {
            println!("  refreshed field list:");
        }
        for field in &summary.refreshed_fields {
            println!("    - {}", field);
        }
    }

    if !summary.failed_fields.is_empty() {
        println!("  failed field list:");
        for failure in &summary.failed_fields {
            println!("    - {}: {}", failure.field, failure.reason);
        }
    }
}

fn detect_stale_fields(state: &StateJson, current_cycle: u64) -> Vec<String> {
    state
        .field_inventory
        .fields
        .iter()
        .filter_map(|(name, entry)| {
            let cadence = entry
                .get("cadence")
                .and_then(Value::as_str)
                .unwrap_or("default");
            let max_allowed_gap = cadence_threshold(cadence);
            let last_refreshed_cycle = entry
                .get("last_refreshed")
                .and_then(Value::as_str)
                .and_then(first_number);
            let gap = match last_refreshed_cycle {
                Some(value) => current_cycle.saturating_sub(value),
                None => max_allowed_gap + 1,
            };

            if gap > max_allowed_gap {
                Some(name.clone())
            } else {
                None
            }
        })
        .collect()
}

fn cadence_threshold(cadence: &str) -> u64 {
    let normalized = cadence.to_ascii_lowercase();
    if normalized.contains("every phase transition")
        || normalized.contains("every cycle")
        || normalized.contains("per cycle")
    {
        2
    } else if let Some(number) = first_number(&normalized) {
        number + 1
    } else if normalized.contains("after") {
        10
    } else {
        5
    }
}

fn first_number(value: &str) -> Option<u64> {
    let digits: String = value
        .chars()
        .skip_while(|character| !character.is_ascii_digit())
        .take_while(|character| character.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse::<u64>().ok()
    }
}

fn verify_field(repo_root: &Path, state: &StateJson, field: &str) -> Result<(), String> {
    match field {
        "total_schema_classes" => verify_i64_field(
            "total_schema_classes",
            state.total_schema_classes,
            count_files(&repo_root.join("php/src/v1/Schema"), "php")?,
        ),
        "total_enums" => verify_i64_field(
            "total_enums",
            state.total_enums,
            count_files(&repo_root.join("php/src/v1/Enum"), "php")?,
        ),
        "total_schema_types" => verify_i64_field(
            "total_schema_types",
            state.total_schema_types,
            required_i64(state.total_schema_classes, "total_schema_classes")?,
        ),
        "total_sub_types" => verify_i64_field(
            "total_sub_types",
            state.total_sub_types,
            derive_total_sub_types(state)?,
        ),
        "total_testable_types" => verify_i64_field(
            "total_testable_types",
            state.total_testable_types,
            derive_total_testable_types(state)?,
        ),
        "total_standalone_testable_types" => verify_i64_field(
            "total_standalone_testable_types",
            state.total_standalone_testable_types,
            derive_total_standalone_testable_types(state)?,
        ),
        "phpstan_level" | "schema_status.phpstan_level" => verify_string_field(
            field,
            state_phpstan_level(state)?,
            read_phpstan_level(&repo_root.join("phpstan.neon"))?,
        ),
        _ => Ok(()),
    }
}

fn verify_i64_field(field: &str, state_value: Option<i64>, actual: i64) -> Result<(), String> {
    let expected = required_i64(state_value, field)?;
    if expected == actual {
        Ok(())
    } else {
        Err(format!(
            "state value mismatch: expected {}, actual {}",
            expected, actual
        ))
    }
}

fn verify_string_field(field: &str, state_value: String, actual: String) -> Result<(), String> {
    if state_value == actual {
        Ok(())
    } else {
        Err(format!(
            "{} mismatch: expected '{}', actual '{}'",
            field, state_value, actual
        ))
    }
}

fn required_i64(value: Option<i64>, field: &str) -> Result<i64, String> {
    value.ok_or_else(|| format!("missing numeric field: {}", field))
}

fn count_files(path: &Path, extension: &str) -> Result<i64, String> {
    let entries =
        fs::read_dir(path).map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    let mut count = 0_i64;

    for entry in entries {
        let entry =
            entry.map_err(|error| format!("failed to read entry in {}: {}", path.display(), error))?;
        let entry_path = entry.path();
        if entry_path.is_file()
            && entry_path
                .extension()
                .and_then(|value| value.to_str())
                .is_some_and(|value| value == extension)
        {
            count += 1;
        }
    }

    Ok(count)
}

fn derive_total_sub_types(state: &StateJson) -> Result<i64, String> {
    let classification = state
        .schema_status
        .type_classification
        .as_ref()
        .ok_or_else(|| "missing schema_status.type_classification".to_string())?;

    Ok(required_i64(
        classification.standalone_testable,
        "schema_status.type_classification.standalone_testable",
    )? + required_i64(
        classification.building_block,
        "schema_status.type_classification.building_block",
    )? + required_i64(
        classification.building_block_only,
        "schema_status.type_classification.building_block_only",
    )? + required_i64(
        classification.enums,
        "schema_status.type_classification.enums",
    )?)
}

fn derive_total_testable_types(state: &StateJson) -> Result<i64, String> {
    Ok(required_i64(state.total_schema_classes, "total_schema_classes")?
        - required_i64(state.total_enums, "total_enums")?)
}

fn derive_total_standalone_testable_types(state: &StateJson) -> Result<i64, String> {
    Ok(derive_total_testable_types(state)?
        - required_i64(
            state.schema_status
                .type_classification
                .as_ref()
                .ok_or_else(|| "missing schema_status.type_classification".to_string())?
                .building_block_only,
            "schema_status.type_classification.building_block_only",
        )?)
}

fn state_phpstan_level(state: &StateJson) -> Result<String, String> {
    state
        .extra
        .get("phpstan_level")
        .and_then(Value::as_str)
        .map(str::to_owned)
        .or_else(|| state.schema_status.phpstan_level.clone())
        .ok_or_else(|| {
            "missing string field: phpstan_level (top-level or schema_status.phpstan_level)"
                .to_string()
        })
}

fn read_phpstan_level(path: &Path) -> Result<String, String> {
    let content =
        fs::read_to_string(path).map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("level:") {
            let value = trimmed.trim_start_matches("level:").trim();
            if !value.is_empty() {
                return Ok(value.to_string());
            }
        }
    }

    Err(format!("could not find `level:` in {}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn refreshes_stale_verified_and_refresh_only_fields_in_place() {
        let repo = create_repo_fixture("refresh-ok");
        write_state(
            repo.path(),
            json!({
                "schema_status": {
                    "type_classification": {
                        "standalone_testable": 2,
                        "standalone_parity_testable": 3,
                        "building_block": 1,
                        "building_block_only": 1,
                        "enums": 1
                    }
                },
                "total_schema_classes": 4,
                "total_enums": 1,
                "total_schema_types": 4,
                "total_sub_types": 5,
                "total_testable_types": 3,
                "total_standalone_testable_types": 2,
                "blockers": [],
                "field_inventory": {
                    "fields": {
                        "total_schema_classes": {
                            "cadence": "after schema class additions",
                            "last_refreshed": "cycle 200"
                        },
                        "blockers": {
                            "cadence": "after blocker state changes",
                            "last_refreshed": "cycle 200"
                        }
                    }
                }
            }),
        );

        let summary =
            refresh_field_inventory(repo.path(), 224, false).expect("refresh should succeed");

        assert_eq!(
            summary.stale_fields,
            vec!["blockers".to_string(), "total_schema_classes".to_string()]
        );
        assert_eq!(
            summary.refreshed_fields,
            vec!["blockers".to_string(), "total_schema_classes".to_string()]
        );
        assert!(summary.failed_fields.is_empty());

        let state = read_state(repo.path());
        assert_eq!(
            state
                .pointer("/field_inventory/fields/total_schema_classes/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 224")
        );
        assert_eq!(
            state.pointer("/field_inventory/fields/blockers/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 224")
        );
    }

    #[test]
    fn auto_verification_failure_is_reported_and_not_written() {
        let repo = create_repo_fixture("refresh-fail");
        write_state(
            repo.path(),
            json!({
                "total_enums": 4,
                "field_inventory": {
                    "fields": {
                        "total_enums": {
                            "cadence": "after enum additions",
                            "last_refreshed": "cycle 200"
                        }
                    }
                }
            }),
        );

        let summary =
            refresh_field_inventory(repo.path(), 224, false).expect("summary should be returned");

        assert_eq!(summary.stale_fields, vec!["total_enums".to_string()]);
        assert!(summary.refreshed_fields.is_empty());
        assert_eq!(summary.failed_fields.len(), 1);
        assert_eq!(summary.failed_fields[0].field, "total_enums");

        let state = read_state(repo.path());
        assert_eq!(
            state.pointer("/field_inventory/fields/total_enums/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 200")
        );
    }

    #[test]
    fn dry_run_reports_refresh_without_writing_state() {
        let repo = create_repo_fixture("refresh-dry-run");
        write_state(
            repo.path(),
            json!({
                "blockers": [],
                "field_inventory": {
                    "fields": {
                        "blockers": {
                            "cadence": "after blocker state changes",
                            "last_refreshed": "cycle 200"
                        }
                    }
                }
            }),
        );

        let summary =
            refresh_field_inventory(repo.path(), 224, true).expect("dry-run should succeed");

        assert_eq!(summary.refreshed_fields, vec!["blockers".to_string()]);
        assert!(summary.failed_fields.is_empty());

        let state = read_state(repo.path());
        assert_eq!(
            state.pointer("/field_inventory/fields/blockers/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 200")
        );
    }

    #[test]
    fn derived_testable_counts_are_verified_before_refresh() {
        let repo = create_repo_fixture("refresh-derived");
        write_state(
            repo.path(),
            json!({
                "schema_status": {
                    "type_classification": {
                        "standalone_testable": 5,
                        "standalone_parity_testable": 7,
                        "building_block": 2,
                        "building_block_only": 1,
                        "enums": 2
                    }
                },
                "total_schema_classes": 10,
                "total_enums": 2,
                "total_testable_types": 8,
                "total_standalone_testable_types": 7,
                "field_inventory": {
                    "fields": {
                        "total_testable_types": {
                            "cadence": "after schema class or enum additions",
                            "last_refreshed": "cycle 200"
                        },
                        "total_standalone_testable_types": {
                            "cadence": "after building-block-only type changes",
                            "last_refreshed": "cycle 200"
                        }
                    }
                }
            }),
        );

        let summary =
            refresh_field_inventory(repo.path(), 224, false).expect("refresh should succeed");

        assert_eq!(
            summary.refreshed_fields,
            vec![
                "total_standalone_testable_types".to_string(),
                "total_testable_types".to_string()
            ]
        );
        assert!(summary.failed_fields.is_empty());
    }

    struct TestRepo {
        path: PathBuf,
    }

    impl TestRepo {
        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn create_repo_fixture(name: &str) -> TestRepo {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock must be after unix epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("refresh-field-inventory-{name}-{unique}"));

        fs::create_dir_all(path.join("docs")).expect("docs dir should be created");
        fs::create_dir_all(path.join("php/src/v1/Schema")).expect("schema dir should be created");
        fs::create_dir_all(path.join("php/src/v1/Enum")).expect("enum dir should be created");
        fs::write(path.join("phpstan.neon"), "parameters:\n    level: 8\n")
            .expect("phpstan.neon should be written");
        fs::write(path.join("php/src/v1/Schema/Thing.php"), "<?php\n")
            .expect("schema fixture should be written");
        fs::write(path.join("php/src/v1/Schema/Product.php"), "<?php\n")
            .expect("schema fixture should be written");
        fs::write(path.join("php/src/v1/Schema/Offer.php"), "<?php\n")
            .expect("schema fixture should be written");
        fs::write(path.join("php/src/v1/Schema/Brand.php"), "<?php\n")
            .expect("schema fixture should be written");
        fs::write(path.join("php/src/v1/Enum/ItemAvailability.php"), "<?php\n")
            .expect("enum fixture should be written");

        TestRepo { path }
    }

    fn write_state(repo_root: &Path, value: Value) {
        let serialized =
            serde_json::to_string_pretty(&value).expect("state fixture should serialize");
        fs::write(repo_root.join("docs/state.json"), format!("{serialized}\n"))
            .expect("state fixture should be written");
    }

    fn read_state(repo_root: &Path) -> Value {
        let content =
            fs::read_to_string(repo_root.join("docs/state.json")).expect("state file should exist");
        serde_json::from_str(&content).expect("state fixture should parse")
    }
}
