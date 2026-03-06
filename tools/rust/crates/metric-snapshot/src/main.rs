use clap::Parser;
use serde_json::{json, Value};
use state_schema::{
    check_version, set_value_at_pointer, update_freshness, StateJson, TypescriptStats,
    SCHEMA_VERSION,
};
use std::collections::{BTreeMap, BTreeSet, HashMap};
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

    /// Apply auto-fixable state.json mismatches and update freshness markers
    #[arg(long)]
    fix: bool,
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
    let state_path = cli.repo_root.join("docs/state.json");

    let mut state = read_state_file(&state_path);
    let mut checks = build_checks(&cli.repo_root, &state);

    if cli.fix {
        let cycle = cli.cycle.unwrap_or_else(|| {
            eprintln!("Error: --fix requires --cycle so freshness markers can be updated");
            process::exit(2);
        });
        match apply_fixes(&state_path, &checks, cycle) {
            Ok(updated) if updated > 0 => {
                state = read_state_file(&state_path);
                checks = build_checks(&cli.repo_root, &state);
            }
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error applying fixes: {}", error);
                process::exit(2);
            }
        }
    }

    let staleness = cli
        .cycle
        .map(|current_cycle| build_staleness_report(&state, current_cycle));

    emit_output(&cli, &checks, staleness);
}

fn build_checks(repo_root: &Path, state: &StateJson) -> Vec<CheckResult> {
    let ts_stats = get_typescript_stats(state);
    let php_schema_count = count_files(&repo_root.join("php/src/v1/Schema"), "php");
    let php_enum_count = count_files(&repo_root.join("php/src/v1/Enum"), "php");
    let ts_schema_count = count_files(&repo_root.join("ts/src/schema"), "ts");
    let ts_enum_count = count_files(&repo_root.join("ts/src/enum"), "ts");
    let ts_core_count = count_files(&repo_root.join("ts/src"), "ts");
    let ts_total_count = ts_schema_count + ts_enum_count + ts_core_count;

    let expected_ts_schema = get_i64_from_map(&ts_stats, "schema_types");
    let expected_php_enums = get_i64_from_option(state.total_enums, "total_enums");
    let expected_ts_enums = get_i64_from_map(&ts_stats, "enums");
    let expected_ts_core = get_i64_from_map(&ts_stats, "core_modules");
    let expected_ts_total = get_i64_from_map(&ts_stats, "total_modules");
    let expected_phpstan_level = get_phpstan_level_from_state(state);
    let expected_php_test_count = get_i64_from_option(state.test_count.php, "test_count.php");
    let expected_ts_test_count = get_i64_from_option(state.test_count.ts, "test_count.ts");
    let expected_total_test_count = get_i64_from_option(state.test_count.total, "test_count.total");
    let actual_phpstan_level = read_phpstan_level(&repo_root.join("phpstan.neon"));
    let actual_php_test_count = count_php_test_methods(&repo_root.join("php/test/unit"));
    let actual_ts_test_count = count_ts_test_methods(&repo_root.join("ts/test"));
    let actual_total_test_count = actual_php_test_count + actual_ts_test_count;
    let schema_version_check_result = check_version(state);
    let actual_schema_version = json!(state.schema_version);

    vec![
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
            ts_total_count == expected_ts_total,
            None,
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
    ]
}

#[derive(Clone)]
/// Auto-fix plan entry for a failed metric check.
///
/// `pointer` is the concrete JSON pointer path to mutate in `state.json`.
/// `freshness_field` is the corresponding `field_inventory.fields.<name>` key
/// whose `last_refreshed` marker should be updated when this pointer changes.
struct FixUpdate {
    pointer: &'static str,
    value: Value,
    freshness_field: &'static str,
}

fn apply_fixes(state_path: &Path, checks: &[CheckResult], cycle: i64) -> Result<usize, String> {
    let cycle = u32::try_from(cycle).map_err(|_| "cycle must fit in u32 range".to_string())?;
    let mut state_value = read_state_value(state_path)?;
    let updates = collect_fix_updates(checks);
    if updates.is_empty() {
        return Ok(0);
    }

    let mut changed_fields = BTreeSet::new();
    let mut changed_count = 0_usize;

    for update in updates {
        if set_value_at_pointer(&mut state_value, update.pointer, update.value)? {
            changed_fields.insert(update.freshness_field);
            changed_count += 1;
        }
    }

    for field_name in changed_fields {
        update_freshness(&mut state_value, field_name, cycle)?;
    }

    if changed_count > 0 {
        let serialized = serde_json::to_string_pretty(&state_value)
            .map_err(|error| format!("failed to serialize state.json: {}", error))?;
        fs::write(state_path, format!("{}\n", serialized))
            .map_err(|error| format!("failed to write {}: {}", state_path.display(), error))?;
    }

    Ok(changed_count)
}

fn read_state_value(path: &Path) -> Result<Value, String> {
    let content = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    serde_json::from_str::<Value>(&content)
        .map_err(|error| format!("failed to parse {}: {}", path.display(), error))
}

fn collect_fix_updates(checks: &[CheckResult]) -> Vec<FixUpdate> {
    let mut deduped: BTreeMap<&'static str, FixUpdate> = BTreeMap::new();

    for check in checks {
        if check.pass {
            continue;
        }
        let Some((pointer, freshness_field)) = fix_target_for_check(check.name) else {
            continue;
        };

        deduped.insert(
            pointer,
            FixUpdate {
                pointer,
                value: check.actual.clone(),
                freshness_field,
            },
        );
    }

    deduped.into_values().collect()
}

fn fix_target_for_check(check_name: &str) -> Option<(&'static str, &'static str)> {
    match check_name {
        "php_schema_classes" => Some(("/total_schema_classes", "total_schema_classes")),
        "php_enum_classes" => Some(("/total_enums", "total_enums")),
        "ts_schema_types" => Some((
            "/schema_status/typescript_stats/schema_types",
            "typescript_stats",
        )),
        "ts_enum_types" => Some(("/schema_status/typescript_stats/enums", "typescript_stats")),
        "ts_core_modules" => Some((
            "/schema_status/typescript_stats/core_modules",
            "typescript_stats",
        )),
        "ts_total_modules" => Some((
            "/schema_status/typescript_stats/total_modules",
            "typescript_stats",
        )),
        "test_count_php" => Some(("/test_count/php", "test_count")),
        "test_count_ts" => Some(("/test_count/ts", "test_count")),
        "test_count_total" => Some(("/test_count/total", "test_count")),
        "phpstan_level" => Some(("/schema_status/phpstan_level", "phpstan_level")),
        _ => None,
    }
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
    count_test_cases_in_dir(path, is_php_file, count_php_tests_in_content)
}

fn count_ts_test_methods(path: &Path) -> i64 {
    count_test_cases_in_dir(path, is_ts_test_file, count_ts_tests_in_content)
}

fn count_test_cases_in_dir(
    path: &Path,
    file_matcher: fn(&Path) -> bool,
    content_counter: fn(&str) -> i64,
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
            count += count_test_cases_in_dir(&entry_path, file_matcher, content_counter);
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

        count += content_counter(&content);
    }

    count
}

fn count_php_tests_in_content(content: &str) -> i64 {
    let provider_case_counts = parse_php_data_provider_case_counts(content);
    let lines: Vec<&str> = content.lines().collect();
    let mut total = 0_i64;

    for (index, line) in lines.iter().enumerate() {
        if !is_php_test_method_line(line) {
            continue;
        }

        if let Some(provider_name) = php_data_provider_for_test_method(&lines, index) {
            if let Some(provider_cases) = provider_case_counts.get(&provider_name) {
                total += *provider_cases;
                continue;
            }
        }

        total += 1;
    }

    total
}

fn parse_php_data_provider_case_counts(content: &str) -> HashMap<String, i64> {
    let lines: Vec<&str> = content.lines().collect();
    let mut counts = HashMap::new();

    for (index, line) in lines.iter().enumerate() {
        let Some(function_name) = php_function_name(line) else {
            continue;
        };
        let Some(case_count) = php_data_provider_case_count(&lines, index + 1) else {
            continue;
        };

        counts.insert(function_name, case_count);
    }

    counts
}

fn php_function_name(line: &str) -> Option<String> {
    let mut tokens = line.trim_start().split_whitespace();
    while let Some(token) = tokens.next() {
        if token != "function" {
            continue;
        }

        let name = tokens
            .next()
            .map(|candidate| candidate.trim_start_matches('&'))
            .and_then(|candidate| candidate.split('(').next())
            .unwrap_or_default();
        if name.is_empty() {
            return None;
        }

        return Some(name.to_string());
    }

    None
}

fn php_data_provider_case_count(lines: &[&str], start_index: usize) -> Option<i64> {
    let mut in_return_array = false;
    let mut count = 0_i64;

    for line in lines.iter().skip(start_index) {
        let trimmed = line.trim();

        if !in_return_array {
            if trimmed.contains(" function ") || trimmed.starts_with("function ") {
                return None;
            }
            if trimmed.contains("return [") {
                in_return_array = true;
            }
            continue;
        }

        if trimmed == "];" || trimmed.ends_with("];") {
            return Some(count);
        }

        if trimmed.contains("=>") {
            count += 1;
        }
    }

    None
}

fn php_data_provider_for_test_method(lines: &[&str], method_line_index: usize) -> Option<String> {
    if method_line_index == 0 {
        return None;
    }

    for line in lines[..method_line_index].iter().rev() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        if let Some(provider_name) = parse_php_data_provider_annotation(trimmed) {
            return Some(provider_name);
        }

        if trimmed.starts_with('*')
            || trimmed.starts_with("/**")
            || trimmed.starts_with("/*")
            || trimmed.starts_with("*/")
        {
            continue;
        }

        break;
    }

    None
}

fn parse_php_data_provider_annotation(line: &str) -> Option<String> {
    let mut tokens = line.split_whitespace();
    while let Some(token) = tokens.next() {
        if token != "@dataProvider" {
            continue;
        }

        return tokens
            .next()
            .map(|provider| provider.trim().trim_matches('*').to_string())
            .filter(|provider| !provider.is_empty());
    }

    None
}

fn count_ts_tests_in_content(content: &str) -> i64 {
    let direct_tests = content
        .lines()
        .filter(|line| is_ts_test_method_line(line))
        .count() as i64;
    let each_array_case_counts = parse_ts_array_case_counts(content);
    let each_tests = count_ts_each_tests(content, &each_array_case_counts);

    direct_tests + each_tests
}

fn parse_ts_array_case_counts(content: &str) -> HashMap<String, i64> {
    let mut counts = HashMap::new();
    let mut start = 0_usize;

    while let Some(relative_const_index) = content[start..].find("const ") {
        let const_index = start + relative_const_index;
        let mut cursor = const_index + "const ".len();
        cursor = skip_ascii_whitespace(content, cursor);
        let Some((name, after_name)) = parse_identifier_at(content, cursor) else {
            start = const_index + "const ".len();
            continue;
        };

        cursor = skip_ascii_whitespace(content, after_name);
        if !content[cursor..].starts_with('=') {
            start = const_index + "const ".len();
            continue;
        }
        cursor += 1;
        cursor = skip_ascii_whitespace(content, cursor);
        if !content[cursor..].starts_with('[') {
            start = const_index + "const ".len();
            continue;
        }

        if let Some((count, end_index)) = count_top_level_array_items(content, cursor) {
            counts.insert(name.to_string(), count);
            start = end_index + 1;
            continue;
        }

        start = const_index + "const ".len();
    }

    counts
}

fn count_ts_each_tests(content: &str, array_case_counts: &HashMap<String, i64>) -> i64 {
    let mut count = 0_i64;
    let mut start = 0_usize;

    while start < content.len() {
        let it_pos = content[start..].find("it.each");
        let test_pos = content[start..].find("test.each");
        let next_match = match (it_pos, test_pos) {
            (Some(i), Some(t)) => Some(i.min(t)),
            (Some(i), None) => Some(i),
            (None, Some(t)) => Some(t),
            (None, None) => None,
        };
        let Some(relative_match) = next_match else {
            break;
        };

        let match_index = start + relative_match;
        let mut cursor = match_index
            + if content[match_index..].starts_with("it.each") {
                "it.each".len()
            } else {
                "test.each".len()
            };
        cursor = skip_ascii_whitespace(content, cursor);
        if !content[cursor..].starts_with('(') {
            start = match_index + 1;
            continue;
        }

        let Some(close_index) = find_matching_delimiter(content, cursor, '(', ')') else {
            start = match_index + 1;
            continue;
        };

        let argument = content[cursor + 1..close_index].trim();
        let case_count = if argument.starts_with('[') {
            count_top_level_array_items(argument, 0)
                .map(|(items, _)| items)
                .unwrap_or(1)
        } else if let Some((identifier, _)) = parse_identifier_at(argument, 0) {
            array_case_counts.get(identifier).copied().unwrap_or(1)
        } else {
            1
        };
        count += case_count;
        start = close_index + 1;
    }

    count
}

fn count_top_level_array_items(content: &str, open_index: usize) -> Option<(i64, usize)> {
    if !content[open_index..].starts_with('[') {
        return None;
    }
    let close_index = find_matching_delimiter(content, open_index, '[', ']')?;
    let body = &content[open_index + 1..close_index];

    let mut square_depth = 0_i32;
    let mut paren_depth = 0_i32;
    let mut brace_depth = 0_i32;
    let mut in_string: Option<char> = None;
    let mut escaped = false;
    let mut has_token = false;
    let mut count = 0_i64;

    for ch in body.chars() {
        if let Some(delimiter) = in_string {
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == delimiter {
                in_string = None;
            }
            has_token = true;
            continue;
        }

        match ch {
            '"' | '\'' | '`' => {
                in_string = Some(ch);
                has_token = true;
            }
            '[' => {
                square_depth += 1;
                has_token = true;
            }
            ']' => {
                square_depth -= 1;
                has_token = true;
            }
            '(' => {
                paren_depth += 1;
                has_token = true;
            }
            ')' => {
                paren_depth -= 1;
                has_token = true;
            }
            '{' => {
                brace_depth += 1;
                has_token = true;
            }
            '}' => {
                brace_depth -= 1;
                has_token = true;
            }
            ',' if square_depth == 0 && paren_depth == 0 && brace_depth == 0 => {
                if has_token {
                    count += 1;
                    has_token = false;
                }
            }
            _ if !ch.is_whitespace() => {
                has_token = true;
            }
            _ => {}
        }
    }

    if has_token {
        count += 1;
    }

    Some((count, close_index))
}

fn find_matching_delimiter(
    content: &str,
    open_index: usize,
    open_delimiter: char,
    close_delimiter: char,
) -> Option<usize> {
    let mut depth = 0_i32;
    let mut in_string: Option<char> = None;
    let mut escaped = false;

    for (relative_index, ch) in content[open_index..].char_indices() {
        if let Some(delimiter) = in_string {
            if escaped {
                escaped = false;
                continue;
            }
            if ch == '\\' {
                escaped = true;
                continue;
            }
            if ch == delimiter {
                in_string = None;
            }
            continue;
        }

        if ch == '"' || ch == '\'' || ch == '`' {
            in_string = Some(ch);
            continue;
        }

        if ch == open_delimiter {
            depth += 1;
            continue;
        }
        if ch == close_delimiter {
            depth -= 1;
            if depth == 0 {
                return Some(open_index + relative_index);
            }
        }
    }

    None
}

fn parse_identifier_at(content: &str, start: usize) -> Option<(&str, usize)> {
    if start >= content.len() {
        return None;
    }

    let mut chars = content[start..].char_indices();
    let (_, first_char) = chars.next()?;
    if !(first_char == '_' || first_char == '$' || first_char.is_ascii_alphabetic()) {
        return None;
    }

    let mut end = start + first_char.len_utf8();
    for (offset, ch) in chars {
        if ch == '_' || ch == '$' || ch.is_ascii_alphanumeric() {
            end = start + offset + ch.len_utf8();
            continue;
        }
        break;
    }

    Some((&content[start..end], end))
}

fn skip_ascii_whitespace(content: &str, mut index: usize) -> usize {
    while index < content.len() {
        let mut chars = content[index..].chars();
        let Some(ch) = chars.next() else {
            break;
        };
        if !ch.is_ascii_whitespace() {
            break;
        }
        index += ch.len_utf8();
    }

    index
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
    let function_name = function_token
        .split('(')
        .next()
        .expect("split always returns at least one segment");

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
        check, collect_fix_updates, count_php_tests_in_content, count_ts_tests_in_content,
        get_i64_from_map, get_i64_from_option, get_typescript_stats, is_php_test_method_line,
        is_ts_test_method_line, parse_cycle_number, read_state_file, set_value_at_pointer,
        staleness_threshold, CheckResult,
    };
    use serde_json::json;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

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
        assert!(!is_php_test_method_line("public function test() {"));
    }

    #[test]
    fn ts_test_method_matching_works() {
        assert!(is_ts_test_method_line("it(\"works\", () => {})"));
        assert!(is_ts_test_method_line("  test (\"works\", () => {})"));
        assert!(!is_ts_test_method_line("it.each([1,2])(...)"));
        assert!(!is_ts_test_method_line("it.skip(\"works\", () => {})"));
        assert!(!is_ts_test_method_line("test.only(\"works\", () => {})"));
        assert!(!is_ts_test_method_line(
            "it.concurrent(\"works\", () => {})"
        ));
        assert!(!is_ts_test_method_line("expect(true).toBe(true)"));
    }

    #[test]
    fn php_data_provider_cases_are_counted() {
        let content = r#"
final class ExampleTest extends TestCase {
    public static function mediaUrlCases(): array {
        return [
            'first' => [null, null],
            'second' => ['a', null],
            'third' => [null, 'b'],
        ];
    }

    /**
     * @dataProvider mediaUrlCases
     */
    public function testMediaCases(?string $a, ?string $b): void {}

    public function testDirectCase(): void {}
}
"#;

        assert_eq!(4, count_php_tests_in_content(content));
    }

    #[test]
    fn ts_each_cases_are_counted() {
        let content = r#"
const mediaUrlCases = [
    ['a', null],
    ['b', null],
    ['c', null],
] as const;

it.each(mediaUrlCases)('works: %s', (_name, _image) => {});
it('direct test', () => {});
"#;

        assert_eq!(4, count_ts_tests_in_content(content));
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

    #[test]
    fn total_schema_classes_and_ts_total_modules_are_checked_independently() {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock must be after unix epoch")
            .as_nanos();
        let temp_dir = std::env::temp_dir().join(format!("metric-snapshot-{suffix}"));
        fs::create_dir_all(&temp_dir).expect("temp dir should be created");

        let state_path = temp_dir.join("state.json");
        fs::write(
            &state_path,
            r#"{
  "total_schema_classes": 89,
  "typescript_stats": {
    "schema_types": 89,
    "enums": 0,
    "core_modules": 15,
    "total_modules": 104
  }
}"#,
        )
        .expect("state fixture should be written");

        let state = read_state_file(&state_path);
        let ts_stats = get_typescript_stats(&state);
        let expected_php_schema =
            get_i64_from_option(state.total_schema_classes, "total_schema_classes");
        let expected_ts_total = get_i64_from_map(&ts_stats, "total_modules");

        let php_schema_check = check(
            "php_schema_classes",
            "PHP schema classes",
            89,
            expected_php_schema,
        );
        let ts_total_check = check(
            "ts_total_modules",
            "TS total modules",
            104,
            expected_ts_total,
        );

        assert_eq!(89, expected_php_schema);
        assert_eq!(104, expected_ts_total);
        assert_ne!(expected_php_schema, expected_ts_total);
        assert!(php_schema_check.pass);
        assert!(ts_total_check.pass);

        fs::remove_dir_all(temp_dir).expect("temp dir should be removed");
    }

    #[test]
    fn collect_fix_updates_maps_and_deduplicates_fixable_checks() {
        let checks = vec![
            CheckResult {
                name: "test_count_php",
                label: "PHP test count",
                actual: json!(440),
                expected: json!(425),
                pass: false,
                note: None,
            },
            CheckResult {
                name: "test_count_php",
                label: "PHP test count",
                actual: json!(441),
                expected: json!(425),
                pass: false,
                note: None,
            },
            CheckResult {
                name: "php_ts_schema_parity",
                label: "PHP/TS schema parity",
                actual: json!(89),
                expected: json!(88),
                pass: false,
                note: None,
            },
        ];

        let updates = collect_fix_updates(&checks);
        assert_eq!(updates.len(), 1);
        assert_eq!(updates[0].pointer, "/test_count/php");
        assert_eq!(updates[0].freshness_field, "test_count");
        assert_eq!(updates[0].value, json!(441));
    }

    #[test]
    fn set_value_at_pointer_requires_existing_path() {
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

        let error = set_value_at_pointer(&mut state, "/test_count/missing", json!(1))
            .expect_err("missing path should fail");
        assert!(error.contains("missing target path segment"));
    }
}
