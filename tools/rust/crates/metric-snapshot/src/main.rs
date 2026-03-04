use clap::Parser;
use serde_json::{json, Map, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process;

#[derive(Parser)]
#[command(name = "metric-snapshot")]
struct Cli {
    /// Path to the repository root
    #[arg(long)]
    repo_root: PathBuf,

    /// Output results as JSON
    #[arg(long)]
    json: bool,
}

struct CheckResult {
    name: &'static str,
    label: &'static str,
    actual: Value,
    expected: Value,
    pass: bool,
    note: Option<String>,
}

fn main() {
    let cli = Cli::parse();

    let state = read_json_file(&cli.repo_root.join("docs/state.json"));
    let ts_stats = get_typescript_stats(&state);

    let php_schema_count = count_files(&cli.repo_root.join("php/src/v1/Schema"), "php");
    let php_enum_count = count_files(&cli.repo_root.join("php/src/v1/Enum"), "php");
    let ts_schema_count = count_files(&cli.repo_root.join("ts/src/schema"), "ts");
    let ts_enum_count = count_files(&cli.repo_root.join("ts/src/enum"), "ts");
    let ts_core_count = count_files(&cli.repo_root.join("ts/src"), "ts");
    let ts_total_count = ts_schema_count + ts_enum_count + ts_core_count;

    let expected_ts_schema = get_i64_from_map(&ts_stats, "schema_types");
    let expected_php_enums = get_i64_from_state(&state, "total_enums");
    let expected_ts_enums = get_i64_from_map(&ts_stats, "enums");
    let expected_ts_core = get_i64_from_map(&ts_stats, "core_modules");
    let expected_ts_total = get_i64_from_map(&ts_stats, "total_modules");
    let expected_phpstan_level = get_phpstan_level_from_state(&state);
    let expected_total_schema_classes = get_i64_from_state(&state, "total_schema_classes");
    let actual_phpstan_level = read_phpstan_level(&cli.repo_root.join("phpstan.neon"));
    let ts_total_check_pass =
        ts_total_count == expected_ts_total && ts_total_count == expected_total_schema_classes;
    let ts_total_note = if ts_total_count != expected_total_schema_classes {
        Some(format!(
            "total_schema_classes in state.json is {}",
            expected_total_schema_classes
        ))
    } else {
        None
    };

    let checks = vec![
        check("php_schema_classes", "PHP schema classes", php_schema_count, expected_ts_schema),
        check("php_enum_classes", "PHP enum classes", php_enum_count, expected_php_enums),
        check("ts_schema_types", "TS schema types", ts_schema_count, expected_ts_schema),
        check("ts_enum_types", "TS enum types", ts_enum_count, expected_ts_enums),
        check("ts_core_modules", "TS core modules", ts_core_count, expected_ts_core),
        check_with_pass(
            "ts_total_modules",
            "TS total modules",
            ts_total_count,
            expected_ts_total,
            ts_total_check_pass,
            ts_total_note,
        ),
        parity_check(
            "php_ts_schema_parity",
            "PHP/TS schema parity",
            php_schema_count,
            ts_schema_count,
        ),
        parity_check(
            "php_ts_enum_parity",
            "PHP/TS enum parity",
            php_enum_count,
            ts_enum_count,
        ),
        string_check(
            "phpstan_level",
            "PHPStan level",
            actual_phpstan_level,
            expected_phpstan_level,
        ),
    ];

    emit_output(&cli, &checks);
}

fn read_json_file(path: &Path) -> Value {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading {}: {}", path.display(), e);
            process::exit(1);
        }
    };

    match serde_json::from_str(&content) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error parsing JSON ({}): {}", path.display(), e);
            process::exit(1);
        }
    }
}

fn get_typescript_stats(state: &Value) -> Map<String, Value> {
    let stats = state
        .get("typescript_stats")
        .or_else(|| state.get("schema_status").and_then(|s| s.get("typescript_stats")))
        .and_then(|v| v.as_object());

    match stats {
        Some(map) => map.clone(),
        None => {
            eprintln!("Missing object: typescript_stats (top-level or schema_status.typescript_stats)");
            process::exit(1);
        }
    }
}

fn get_i64_from_map(obj: &Map<String, Value>, key: &str) -> i64 {
    match obj.get(key).and_then(|v| v.as_i64()) {
        Some(v) => v,
        None => {
            eprintln!("Missing or non-integer field: {}", key);
            process::exit(1);
        }
    }
}

fn get_i64_from_state(state: &Value, key: &str) -> i64 {
    match state.get(key).and_then(|v| v.as_i64()) {
        Some(v) => v,
        None => {
            eprintln!("Missing or non-integer field: {}", key);
            process::exit(1);
        }
    }
}

fn get_phpstan_level_from_state(state: &Value) -> String {
    let level = state
        .get("phpstan_level")
        .or_else(|| state.get("schema_status").and_then(|s| s.get("phpstan_level")))
        .and_then(|v| v.as_str());

    match level {
        Some(value) => value.to_string(),
        None => {
            eprintln!("Missing string field: phpstan_level (top-level or schema_status.phpstan_level)");
            process::exit(1);
        }
    }
}

fn read_phpstan_level(path: &Path) -> String {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading {}: {}", path.display(), e);
            process::exit(1);
        }
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("level:") {
            let value = trimmed.trim_start_matches("level:").trim();
            if !value.is_empty() {
                return value.to_string();
            }
        }
    }

    eprintln!("Could not find `level:` in {}", path.display());
    process::exit(1);
}

fn count_files(path: &Path, extension: &str) -> i64 {
    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Error reading {}: {}", path.display(), e);
            process::exit(1);
        }
    };

    let mut count = 0_i64;
    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Error reading entry in {}: {}", path.display(), e);
                process::exit(1);
            }
        };
        let entry_path = entry.path();
        if entry_path.is_file()
            && entry_path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext == extension)
        {
            count += 1;
        }
    }
    count
}

fn check(name: &'static str, label: &'static str, actual: i64, expected: i64) -> CheckResult {
    check_with_pass(name, label, actual, expected, actual == expected, None)
}

fn check_with_pass(
    name: &'static str,
    label: &'static str,
    actual: i64,
    expected: i64,
    pass: bool,
    note: Option<String>,
) -> CheckResult {
    CheckResult {
        name,
        label,
        actual: json!(actual),
        expected: json!(expected),
        pass,
        note,
    }
}

fn parity_check(name: &'static str, label: &'static str, left: i64, right: i64) -> CheckResult {
    CheckResult {
        name,
        label,
        actual: json!(left),
        expected: json!(right),
        pass: left == right,
        note: None,
    }
}

fn string_check(name: &'static str, label: &'static str, actual: String, expected: String) -> CheckResult {
    let pass = actual == expected;
    CheckResult {
        name,
        label,
        actual: json!(actual),
        expected: json!(expected),
        pass,
        note: None,
    }
}

fn emit_output(cli: &Cli, checks: &[CheckResult]) {
    let failed_count = checks.iter().filter(|c| !c.pass).count();
    let pass = failed_count == 0;
    let summary = if pass {
        format!("All {} checks passed", checks.len())
    } else {
        format!("{} of {} checks failed", failed_count, checks.len())
    };

    if cli.json {
        let checks_json: Vec<Value> = checks
            .iter()
            .map(|c| {
                json!({
                    "name": c.name,
                    "actual": c.actual,
                    "expected": c.expected,
                    "pass": c.pass,
                    "note": c.note,
                })
            })
            .collect();

        let output = json!({
            "pass": pass,
            "checks": checks_json,
            "summary": summary,
        });
        println!("{}", output);
    } else {
        println!("Metric Snapshot — repo root: {}", cli.repo_root.display());
        println!();

        for check in checks {
            let marker = if check.pass { "✓" } else { "✗ MISMATCH" };
            if check.name.starts_with("php_ts_") {
                println!(
                    "  {:<22} {:>3} = {:<3} {}{}",
                    format!("{}:", check.label),
                    check
                        .actual
                        .as_i64()
                        .expect("parity check actual value must be integer"),
                    check
                        .expected
                        .as_i64()
                        .expect("parity check expected value must be integer"),
                    marker,
                    check
                        .note
                        .as_ref()
                        .map(|n| format!(" ({})", n))
                        .unwrap_or_default()
                );
            } else {
                println!(
                    "  {:<22} {:>3} (state.json: {}) {}{}",
                    format!("{}:", check.label),
                    value_to_display(&check.actual),
                    value_to_display(&check.expected),
                    marker,
                    check
                        .note
                        .as_ref()
                        .map(|n| format!(" ({})", n))
                        .unwrap_or_default()
                );
            }
        }
        println!();
        if pass {
            println!("PASS: {}", summary);
        } else {
            println!("FAIL: {}", summary);
        }
    }

    if !pass {
        process::exit(1);
    }
}

fn value_to_display(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        _ => value.to_string(),
    }
}
