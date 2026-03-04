use clap::Parser;
use serde_json::{json, Value};
use state_schema::{check_version, StateJson, TypescriptStats, SCHEMA_VERSION};
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

    /// Current cycle number for field staleness detection (advisory only)
    #[arg(long)]
    cycle: Option<i64>,
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

    let state = read_state_file(&cli.repo_root.join("docs/state.json"));
    let ts_stats = get_typescript_stats(&state);

    let php_schema_count = count_files(&cli.repo_root.join("php/src/v1/Schema"), "php");
    let php_enum_count = count_files(&cli.repo_root.join("php/src/v1/Enum"), "php");
    let ts_schema_count = count_files(&cli.repo_root.join("ts/src/schema"), "ts");
    let ts_enum_count = count_files(&cli.repo_root.join("ts/src/enum"), "ts");
    let ts_core_count = count_files(&cli.repo_root.join("ts/src"), "ts");
    let ts_total_count = ts_schema_count + ts_enum_count + ts_core_count;

    let expected_ts_schema = get_i64_from_map(&ts_stats, "schema_types");
    let expected_php_enums = get_i64_from_option(state.total_enums, "total_enums");
    let expected_ts_enums = get_i64_from_map(&ts_stats, "enums");
    let expected_ts_core = get_i64_from_map(&ts_stats, "core_modules");
    let expected_ts_total = get_i64_from_map(&ts_stats, "total_modules");
    let expected_phpstan_level = get_phpstan_level_from_state(&state);
    let expected_total_schema_classes =
        get_i64_from_option(state.total_schema_classes, "total_schema_classes");
    let expected_php_test_count = get_i64_from_option(state.test_count.php, "test_count.php");
    let expected_ts_test_count = get_i64_from_option(state.test_count.ts, "test_count.ts");
    let expected_total_test_count = get_i64_from_option(state.test_count.total, "test_count.total");
    let actual_phpstan_level = read_phpstan_level(&cli.repo_root.join("phpstan.neon"));
    let actual_php_test_count = count_php_test_methods(&cli.repo_root.join("php/test/unit"));
    let actual_ts_test_count = count_ts_test_methods(&cli.repo_root.join("ts/test"));
    let actual_total_test_count = actual_php_test_count + actual_ts_test_count;
    let schema_version_check_result = check_version(&state);
    let actual_schema_version = json!(state.schema_version);
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
        check(
            "php_schema_classes",
            "PHP schema classes",
            php_schema_count,
            expected_ts_schema,
        ),
        check(
            "php_enum_classes",
            "PHP enum classes",
            php_enum_count,
            expected_php_enums,
        ),
        check(
            "ts_schema_types",
            "TS schema types",
            ts_schema_count,
            expected_ts_schema,
        ),
        check(
            "ts_enum_types",
            "TS enum types",
            ts_enum_count,
            expected_ts_enums,
        ),
        check(
            "ts_core_modules",
            "TS core modules",
            ts_core_count,
            expected_ts_core,
        ),
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
        check(
            "test_count_php",
            "PHP test count",
            actual_php_test_count,
            expected_php_test_count,
        ),
        check(
            "test_count_ts",
            "TS test count",
            actual_ts_test_count,
            expected_ts_test_count,
        ),
        check(
            "test_count_total",
            "Total test count",
            actual_total_test_count,
            expected_total_test_count,
        ),
        string_check(
            "phpstan_level",
            "PHPStan level",
            actual_phpstan_level,
            expected_phpstan_level,
        ),
        value_check_with_pass(
            "state_schema_version",
            "State schema version",
            actual_schema_version,
            json!(SCHEMA_VERSION),
            schema_version_check_result.is_ok(),
            schema_version_check_result.err(),
        ),
    ];

    let staleness = cli
        .cycle
        .map(|current_cycle| build_staleness_report(&state, current_cycle));

    emit_output(&cli, &checks, staleness);
}

fn read_state_file(path: &Path) -> StateJson {
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

fn get_typescript_stats(state: &StateJson) -> TypescriptStats {
    if let Some(stats) = state
        .extra
        .get("typescript_stats")
        .and_then(|v| serde_json::from_value::<TypescriptStats>(v.clone()).ok())
    {
        return stats;
    }

    if let Some(stats) = state.schema_status.typescript_stats.clone() {
        return stats;
    }

    eprintln!("Missing object: typescript_stats (top-level or schema_status.typescript_stats)");
    process::exit(1);
}

fn get_i64_from_map(obj: &TypescriptStats, key: &str) -> i64 {
    let value = match key {
        "schema_types" => obj.schema_types,
        "enums" => obj.enums,
        "core_modules" => obj.core_modules,
        "total_modules" => obj.total_modules,
        _ => None,
    };

    match value {
        Some(v) => v,
        None => {
            eprintln!("Missing or non-integer field: {}", key);
            process::exit(1);
        }
    }
}

fn get_i64_from_option(value: Option<i64>, key: &str) -> i64 {
    match value {
        Some(v) => v,
        None => {
            eprintln!("Missing or non-integer field: {}", key);
            process::exit(1);
        }
    }
}

fn get_phpstan_level_from_state(state: &StateJson) -> String {
    let level = state
        .extra
        .get("phpstan_level")
        .and_then(|v| v.as_str())
        .map(str::to_owned)
        .or_else(|| state.schema_status.phpstan_level.clone());

    match level {
        Some(value) => value,
        None => {
            eprintln!(
                "Missing string field: phpstan_level (top-level or schema_status.phpstan_level)"
            );
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

fn count_php_test_methods(path: &Path) -> i64 {
    count_matching_lines_in_dir(path, is_php_file, is_php_test_method_line)
}

fn count_ts_test_methods(path: &Path) -> i64 {
    count_matching_lines_in_dir(path, is_ts_test_file, is_ts_test_method_line)
}

fn count_matching_lines_in_dir(
    path: &Path,
    file_matcher: fn(&Path) -> bool,
    line_matcher: fn(&str) -> bool,
) -> i64 {
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
        if entry_path.is_dir() {
            count += count_matching_lines_in_dir(&entry_path, file_matcher, line_matcher);
            continue;
        }

        if !entry_path.is_file() || !file_matcher(&entry_path) {
            continue;
        }

        let content = match fs::read_to_string(&entry_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading {}: {}", entry_path.display(), e);
                process::exit(1);
            }
        };

        count += content.lines().filter(|line| line_matcher(line)).count() as i64;
    }

    count
}

fn is_php_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext == "php")
}

fn is_ts_test_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext == "ts")
        && path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| name.ends_with(".test.ts"))
}

fn is_php_test_method_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    let mut tokens = trimmed.split_whitespace();
    if tokens.next() != Some("public") || tokens.next() != Some("function") {
        return false;
    }

    let function_token = match tokens.next() {
        Some(token) => token,
        None => return false,
    };
    let function_name = function_token.split('(').next().unwrap_or(function_token);

    function_name
        .strip_prefix("test")
        .and_then(|remaining| remaining.chars().next())
        .is_some_and(|ch| ch.is_ascii_alphanumeric() || ch == '_')
}

fn is_ts_test_method_line(line: &str) -> bool {
    let trimmed = line.trim_start();
    ["it", "test"].iter().any(|keyword| {
        trimmed
            .strip_prefix(keyword)
            .is_some_and(|remaining| remaining.trim_start().starts_with('('))
    })
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
    value_check_with_pass(name, label, json!(actual), json!(expected), pass, note)
}

fn value_check_with_pass(
    name: &'static str,
    label: &'static str,
    actual: Value,
    expected: Value,
    pass: bool,
    note: Option<String>,
) -> CheckResult {
    CheckResult {
        name,
        label,
        actual,
        expected,
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

fn string_check(
    name: &'static str,
    label: &'static str,
    actual: String,
    expected: String,
) -> CheckResult {
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

fn build_staleness_report(state: &StateJson, current_cycle: i64) -> Value {
    let stale_fields: Vec<Value> = state
        .field_inventory
        .fields
        .iter()
        .filter_map(|(field_name, metadata)| {
            let metadata_obj = metadata.as_object()?;
            let cadence = metadata_obj.get("cadence")?.as_str()?;
            let last_refreshed = metadata_obj.get("last_refreshed")?.as_str()?;
            let threshold = staleness_threshold(cadence);
            let last_cycle = parse_cycle_number(last_refreshed)?;
            let cycles_behind = current_cycle.saturating_sub(last_cycle);
            if cycles_behind > threshold {
                Some(json!({
                    "field": field_name,
                    "cadence": cadence,
                    "last_refreshed": last_refreshed,
                    "last_cycle": last_cycle,
                    "cycles_behind": cycles_behind,
                    "threshold": threshold,
                }))
            } else {
                None
            }
        })
        .collect();

    json!({
        "current_cycle": current_cycle,
        "stale_fields": stale_fields,
    })
}

fn staleness_threshold(cadence: &str) -> i64 {
    if cadence.trim().eq_ignore_ascii_case("every cycle") {
        2
    } else {
        10
    }
}

fn parse_cycle_number(value: &str) -> Option<i64> {
    value
        .split_whitespace()
        .find_map(|token| token.parse::<i64>().ok())
}

fn emit_output(cli: &Cli, checks: &[CheckResult], staleness: Option<Value>) {
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
            "staleness": staleness,
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

        if let Some(staleness_report) = &staleness {
            let current_cycle = staleness_report
                .get("current_cycle")
                .and_then(Value::as_i64)
                .expect("staleness current_cycle must be integer");
            let stale_fields = staleness_report
                .get("stale_fields")
                .and_then(Value::as_array)
                .expect("staleness stale_fields must be array");

            println!();
            println!("Staleness report (cycle {}):", current_cycle);
            if stale_fields.is_empty() {
                println!("  No stale fields detected.");
            } else {
                for stale in stale_fields {
                    println!(
                        "  - {}: {} ({} cycles behind; threshold {})",
                        stale
                            .get("field")
                            .and_then(Value::as_str)
                            .expect("stale field name must be string"),
                        stale
                            .get("last_refreshed")
                            .and_then(Value::as_str)
                            .expect("stale last_refreshed must be string"),
                        stale
                            .get("cycles_behind")
                            .and_then(Value::as_i64)
                            .expect("stale cycles_behind must be integer"),
                        stale
                            .get("threshold")
                            .and_then(Value::as_i64)
                            .expect("stale threshold must be integer")
                    );
                }
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

#[cfg(test)]
mod tests {
    use super::{
        is_php_test_method_line, is_ts_test_method_line, parse_cycle_number, staleness_threshold,
    };

    #[test]
    fn php_test_method_matching_works() {
        assert!(is_php_test_method_line(
            "public function testMinimalOutput(): void {"
        ));
        assert!(is_php_test_method_line(
            "\tpublic\tfunction\ttest_with_underscore() {"
        ));
        assert!(!is_php_test_method_line(
            "private function testMinimalOutput(): void {"
        ));
        assert!(!is_php_test_method_line(
            "public function helperMethod(): void {"
        ));
        assert!(!is_php_test_method_line("public function test(): void {"));
    }

    #[test]
    fn ts_test_method_matching_works() {
        assert!(is_ts_test_method_line("it(\"works\", () => {})"));
        assert!(is_ts_test_method_line("  test (\"works\", () => {})"));
        assert!(!is_ts_test_method_line("it.each([1,2])(...)"));
        assert!(!is_ts_test_method_line("expect(true).toBe(true)"));
    }

    #[test]
    fn cycle_parsing_and_thresholds_work() {
        assert_eq!(Some(128), parse_cycle_number("cycle 128"));
        assert_eq!(Some(131), parse_cycle_number("refreshed at cycle 131"));
        assert_eq!(None, parse_cycle_number("n/a"));
        assert_eq!(2, staleness_threshold("every cycle"));
        assert_eq!(
            10,
            staleness_threshold("every merge that adds/removes tests")
        );
    }
}
