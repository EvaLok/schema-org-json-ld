use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};
use state_schema::{current_cycle_from_state, set_value_at_pointer};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MAX_CATEGORY_LENGTH: usize = 40;

#[derive(Parser, Debug)]
#[command(name = "process-review")]
struct Cli {
    /// Path to the review file (e.g. docs/reviews/cycle-162.md)
    #[arg(long)]
    review_file: PathBuf,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Count of findings actioned this cycle
    #[arg(long, default_value_t = 0)]
    actioned: u64,

    /// Count of findings deferred this cycle
    #[arg(long, default_value_t = 0)]
    deferred: u64,

    /// Count of findings ignored this cycle
    #[arg(long, default_value_t = 0)]
    ignored: u64,

    /// Bypass validation that disposition counts match the parsed finding count
    #[arg(long, default_value_t = false)]
    skip_disposition_check: bool,

    /// Optional note for the review history entry
    #[arg(long)]
    note: Option<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct ReviewHistoryEntry {
    cycle: u64,
    finding_count: u64,
    complacency_score: u64,
    categories: Vec<String>,
    actioned: u64,
    deferred: u64,
    ignored: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedReview {
    cycle: u64,
    finding_count: u64,
    complacency_score: u64,
    categories: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PatchUpdate {
    path: String,
    value: Value,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let review_path = resolve_review_path(&cli.repo_root, &cli.review_file);
    let review_content = fs::read_to_string(&review_path)
        .map_err(|error| format!("failed to read {}: {}", review_path.display(), error))?;

    let parsed_review = parse_review(&review_path, &review_content)?;
    validate_dispositions(&cli, parsed_review.finding_count)?;
    let entry = build_history_entry(&parsed_review, &cli);

    let state_path = cli.repo_root.join("docs/state.json");
    let mut state_value = read_state_value(&state_path)?;
    let current_cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /last_cycle/number in state.json" {
            "missing numeric /last_cycle/number in docs/state.json".to_string()
        } else {
            error
        }
    })?;

    let patch = build_state_patch(&state_value, parsed_review.cycle, current_cycle, &entry)?;
    apply_patch(&mut state_value, &patch)?;
    write_state_value(&state_path, &state_value)?;

    let receipt = commit_state_json(
        &cli.repo_root,
        parsed_review.cycle,
        parsed_review.complacency_score,
        current_cycle,
    )?;

    println!(
        "Review processed: cycle {}, score {}/5, {} findings",
        parsed_review.cycle, parsed_review.complacency_score, parsed_review.finding_count
    );
    if parsed_review.categories.is_empty() {
        println!("Categories: (none parsed)");
    } else {
        println!("Categories: {}", parsed_review.categories.join(", "));
    }
    println!("History entry added (receipt: {})", receipt);

    Ok(())
}

fn validate_dispositions(cli: &Cli, finding_count: u64) -> Result<(), String> {
    if cli.skip_disposition_check {
        return Ok(());
    }

    let disposition_sum = cli
        .actioned
        .checked_add(cli.deferred)
        .and_then(|value| value.checked_add(cli.ignored))
        .ok_or_else(|| "disposition counts overflowed u64".to_string())?;

    if disposition_sum == finding_count {
        return Ok(());
    }

    let all_default = cli.actioned == 0 && cli.deferred == 0 && cli.ignored == 0;
    if all_default && finding_count > 0 {
        return Err(format!(
			"review contains {} findings, but --actioned, --deferred, and --ignored were all left at 0; provide disposition flags or pass --skip-disposition-check to bypass this validation",
			finding_count
		));
    }

    eprintln!(
        "Warning: disposition counts sum to {} but parsed finding_count is {}; proceeding anyway",
        disposition_sum, finding_count
    );
    Ok(())
}

fn resolve_review_path(repo_root: &Path, review_file: &Path) -> PathBuf {
    if review_file.is_absolute() {
        review_file.to_path_buf()
    } else {
        repo_root.join(review_file)
    }
}

fn parse_review(review_path: &Path, content: &str) -> Result<ParsedReview, String> {
    let cycle = extract_cycle_number(review_path).ok_or_else(|| {
        format!(
            "failed to derive cycle number from {}",
            review_path.display()
        )
    })?;

    let complacency_score = match extract_score(content) {
        Some(value) => value,
        None => {
            eprintln!("Warning: unable to parse complacency score from review markdown");
            return Err("complacency score is required".to_string());
        }
    };

    let finding_count = match extract_finding_count(content) {
        Some(value) => value,
        None => {
            eprintln!("Warning: unable to parse finding count from review markdown");
            return Err("finding count is required".to_string());
        }
    };

    let categories = extract_categories(content);
    if categories.is_empty() {
        eprintln!("Warning: no finding categories were parsed from review markdown");
    }

    Ok(ParsedReview {
        cycle,
        finding_count,
        complacency_score,
        categories,
    })
}

fn extract_cycle_number(path: &Path) -> Option<u64> {
    let name = path.file_name()?.to_str()?.to_ascii_lowercase();
    if !name.ends_with(".md") {
        return None;
    }

    let stem = name.trim_end_matches(".md");
    if let Some(number) = stem.strip_prefix("cycle-") {
        return number.parse::<u64>().ok();
    }

    None
}

fn extract_score(content: &str) -> Option<u64> {
    let mut in_complacency = false;

    for line in content.lines() {
        let trimmed = line.trim();
        let lower = trimmed.to_ascii_lowercase();
        let is_complacency_heading = lower.starts_with("## complacency");
        let is_level_two_heading = lower.starts_with("## ");

        if is_complacency_heading {
            in_complacency = true;
            continue;
        }

        if in_complacency && is_level_two_heading {
            break;
        }

        if !in_complacency || !trimmed.contains("/5") {
            continue;
        }

        if let Some((score, _)) = find_number_before_token(trimmed, "/5") {
            return Some(score);
        }
    }

    None
}

fn extract_finding_count(content: &str) -> Option<u64> {
    let list_count = count_numbered_findings_in_findings_section(content);
    if list_count > 0 {
        return Some(list_count as u64);
    }

    None
}

fn count_numbered_findings_in_findings_section(content: &str) -> usize {
    let mut in_findings = false;
    let mut in_code_block = false;
    let mut count = 0usize;

    for line in content.lines() {
        let trimmed = line.trim();
        let lower = trimmed.to_ascii_lowercase();

        if lower.starts_with("## findings") {
            in_findings = true;
            continue;
        }

        if in_findings && lower.starts_with("## ") && !lower.starts_with("## findings") {
            break;
        }

        if !in_findings {
            continue;
        }

        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }

        if in_code_block {
            continue;
        }

        if is_numbered_finding_heading(trimmed) {
            count += 1;
        }
    }

    count
}

fn is_numbered_finding_heading(line: &str) -> bool {
    let mut chars = line.chars().peekable();
    let mut saw_digit = false;
    while let Some(ch) = chars.peek() {
        if ch.is_ascii_digit() {
            saw_digit = true;
            chars.next();
        } else {
            break;
        }
    }

    if !saw_digit {
        return false;
    }

    if !matches!(chars.next(), Some('.')) {
        return false;
    }

    let mut saw_whitespace = false;
    while let Some(ch) = chars.peek() {
        if ch.is_whitespace() {
            saw_whitespace = true;
            chars.next();
        } else {
            break;
        }
    }

    saw_whitespace && chars.next() == Some('*') && chars.next() == Some('*')
}

fn extract_categories(content: &str) -> Vec<String> {
    let mut categories = BTreeSet::new();
    let mut in_findings = false;
    let mut in_code_block = false;
    let mut awaiting_category = false;

    for line in content.lines() {
        let trimmed = line.trim();
        let lower = trimmed.to_ascii_lowercase();

        if lower.starts_with("## findings") {
            in_findings = true;
            in_code_block = false;
            awaiting_category = false;
            continue;
        }

        if in_findings && lower.starts_with("## ") && !lower.starts_with("## findings") {
            break;
        }

        if !in_findings {
            continue;
        }

        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            awaiting_category = false;
            continue;
        }

        if in_code_block {
            continue;
        }

        if is_numbered_finding_heading(trimmed) {
            if let Some(raw) = extract_inline_category(trimmed) {
                if let Some(normalized) = normalize_category(raw) {
                    categories.insert(normalized);
                }
                awaiting_category = false;
            } else {
                awaiting_category = true;
            }
            continue;
        }

        if awaiting_category && trimmed.is_empty() {
            continue;
        }

        if awaiting_category {
            if let Some(raw) = extract_category_line(line) {
                if let Some(normalized) = normalize_category(raw) {
                    categories.insert(normalized);
                }
            }
            awaiting_category = false;
        }
    }

    categories.into_iter().collect()
}

fn extract_inline_category(line: &str) -> Option<&str> {
    let start = line.find("**[")? + 3;
    let remainder = &line[start..];
    let end = remainder.find(']')?;
    if end == 0 {
        None
    } else {
        Some(&remainder[..end])
    }
}

fn extract_category_line(line: &str) -> Option<&str> {
    let trimmed_start = line.trim_start();
    if trimmed_start.len() == line.len() {
        return None;
    }

    let raw = trimmed_start.strip_prefix("Category:")?.trim();
    if raw.is_empty() {
        None
    } else {
        Some(raw)
    }
}

fn normalize_category(category: &str) -> Option<String> {
    let mut normalized = String::new();
    let mut last_dash = false;

    for ch in category.chars() {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch.to_ascii_lowercase());
            last_dash = false;
        } else if !last_dash {
            normalized.push('-');
            last_dash = true;
        }
    }

    let trimmed = normalized.trim_matches('-').to_string();
    if trimmed.is_empty() || trimmed.len() > MAX_CATEGORY_LENGTH {
        None
    } else {
        Some(trimmed)
    }
}

fn find_number_before_token(text: &str, token: &str) -> Option<(u64, usize)> {
    let token_index = text.find(token)?;
    let mut end = token_index;

    while end > 0 && text.as_bytes()[end - 1].is_ascii_whitespace() {
        end -= 1;
    }

    let mut start = end;
    while start > 0 && text.as_bytes()[start - 1].is_ascii_digit() {
        start -= 1;
    }

    if start == end {
        return None;
    }

    let value = text[start..end].parse::<u64>().ok()?;
    Some((value, start))
}

fn build_history_entry(parsed_review: &ParsedReview, cli: &Cli) -> ReviewHistoryEntry {
    ReviewHistoryEntry {
        cycle: parsed_review.cycle,
        finding_count: parsed_review.finding_count,
        complacency_score: parsed_review.complacency_score,
        categories: parsed_review.categories.clone(),
        actioned: cli.actioned,
        deferred: cli.deferred,
        ignored: cli.ignored,
        note: cli.note.clone(),
    }
}

fn read_state_value(path: &Path) -> Result<Value, String> {
    let content = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    serde_json::from_str::<Value>(&content)
        .map_err(|error| format!("failed to parse {}: {}", path.display(), error))
}

fn write_state_value(path: &Path, value: &Value) -> Result<(), String> {
    let serialized = serde_json::to_string_pretty(value)
        .map_err(|error| format!("failed to serialize state.json: {}", error))?;
    fs::write(path, format!("{}\n", serialized))
        .map_err(|error| format!("failed to write {}: {}", path.display(), error))
}

fn build_state_patch(
    state: &Value,
    review_cycle: u64,
    current_cycle: u64,
    entry: &ReviewHistoryEntry,
) -> Result<Vec<PatchUpdate>, String> {
    let history = state
        .pointer("/review_agent/history")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing /review_agent/history array in docs/state.json".to_string())?;

    let mut next_history = history.clone();
    let entry_value = serde_json::to_value(entry)
        .map_err(|error| format!("failed to serialize review history entry: {}", error))?;
    next_history.push(entry_value);

    Ok(vec![
        PatchUpdate {
            path: "/review_agent/last_review_cycle".to_string(),
            value: json!(review_cycle),
        },
        PatchUpdate {
            path: "/review_agent/history".to_string(),
            value: Value::Array(next_history),
        },
        PatchUpdate {
            path: "/field_inventory/fields/review_agent/last_refreshed".to_string(),
            value: json!(format!("cycle {}", current_cycle)),
        },
    ])
}

fn apply_patch(state: &mut Value, updates: &[PatchUpdate]) -> Result<(), String> {
    for update in updates {
        set_value_at_pointer(state, &update.path, update.value.clone())?;
    }

    Ok(())
}

fn commit_state_json(
    repo_root: &Path,
    review_cycle: u64,
    score: u64,
    current_cycle: u64,
) -> Result<String, String> {
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

    let commit_message = format!(
        "state(process-review): cycle {} review consumed, score {}/5 [cycle {}]",
        review_cycle, score, current_cycle
    );
    let commit_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    const SAMPLE_REVIEW: &str = r#"# Cycle 162 Review

## Findings

1. **State consistency drift remains visible.**
   Category: Data Integrity
2. **Field freshness gaps continue in one path.**
   Category: Process Integrity
3. **Arithmetic checks now pass for metrics.**

## Recommendations

1. Automate more state updates.

## Complacency score

**Complacency Score: 2/5**
"#;

    const CYCLE_170_REVIEW: &str = r#"# Cycle 170 Review

## Findings

1. **`review_agent` changed this cycle, but its freshness marker still says it did not.**  
   Category: review-agent-freshness-drift  
   The worklog says cycle 170 manually corrected the review history entry after `process-review` produced bad category output (`docs/worklog/2026-03-07/045200-hundred-seventieth-orchestrator-cycle.md:23-29`), and the current history entry for cycle 169 reflects that correction (`docs/state.json:1329-1344`).

2. **The cycle fixed the rate bug in code, but only after two more rounds of manual state surgery.**  
   Category: reactive-manual-repair  
   The journal is candid that this was the third straight cycle with a manual `dispatch_to_pr_rate` repair (`docs/journal/2026-03-07.md:65-68`), and the worklog records two manual corrections in the same cycle before PR #637 merged (`docs/worklog/2026-03-07/045200-hundred-seventieth-orchestrator-cycle.md:23-29`).

3. **The publish-gate status was refreshed as current-cycle state, but its divergence evidence still points at cycle 169.**  
   Category: publish-gate-evidence-reuse  
   The worklog's current-state section says the publish gate is "FULLY CLEARED" and that there is "No source divergence" (`docs/worklog/2026-03-07/045200-hundred-seventieth-orchestrator-cycle.md:37-41`).

## Recommendations

1. Whenever `docs/state.json` is edited manually, update the matching `field_inventory` freshness entry in the same change or route the change through a tool that does it automatically.

## Complacency score

3/5 — this cycle made real improvements and the journal is genuinely self-critical rather than formulaic, but it still normalized avoidable manual state repair and let some evidence/freshness bookkeeping lag behind the actual work.
"#;

    const CYCLE_196_REVIEW: &str =
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../../../docs/reviews/cycle-196.md"));

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--review-file"));
        assert!(help.contains("--repo-root"));
        assert!(help.contains("--actioned"));
        assert!(help.contains("--deferred"));
        assert!(help.contains("--ignored"));
        assert!(help.contains("--skip-disposition-check"));
        assert!(help.contains("--note"));
    }

    #[test]
    fn score_extraction_reads_complacency_section() {
        let markdown = r#"## Recommendations

1. Keep the previous 5/5 note out of the score parser.

## Complacency

2/5 — actual score
"#;
        assert_eq!(extract_score(markdown), Some(2));
    }

    #[test]
    fn score_extraction_ignores_recommendation_scores_before_complacency_section() {
        let markdown = r#"## Recommendations

3. Stop recording pipeline status as a bare "5/5" unless the evidence is current.

## Complacency score

4/5 — cycle 183 did real work and still left follow-up behind.
"#;
        assert_eq!(extract_score(markdown), Some(4));
    }

    #[test]
    fn finding_count_counts_only_numbered_bold_findings() {
        assert_eq!(extract_finding_count(SAMPLE_REVIEW), Some(3));
    }

    #[test]
    fn finding_count_uses_cycle_170_review_format() {
        assert_eq!(extract_finding_count(CYCLE_170_REVIEW), Some(3));
        assert_eq!(extract_score(CYCLE_170_REVIEW), Some(3));
    }

    #[test]
    fn finding_count_requires_bold_numbered_findings() {
        let markdown = r#"## Findings
1. **One**
2. **Two**
## Recommendations
1. R
"#;
        assert_eq!(extract_finding_count(markdown), Some(2));
    }

    #[test]
    fn category_extraction_normalizes_values() {
        let categories = extract_categories(CYCLE_170_REVIEW);
        assert_eq!(
            categories,
            vec![
                "publish-gate-evidence-reuse".to_string(),
                "reactive-manual-repair".to_string(),
                "review-agent-freshness-drift".to_string(),
            ]
        );
    }

    #[test]
    fn category_extraction_ignores_titles_without_category_lines() {
        let markdown = r#"## Findings

1. **Cycle 123 closes with a false all-green narrative: the repository currently fails 2 of the 9 state invariants.**

## Complacency score

**2/5**
"#;

        assert!(extract_categories(markdown).is_empty());
    }

    #[test]
    fn category_extraction_reads_inline_heading_category() {
        let markdown = r#"## Findings

1. **[metrics-ownership] Finding title**
"#;

        assert_eq!(
            extract_categories(markdown),
            vec!["metrics-ownership".to_string()]
        );
    }

    #[test]
    fn category_extraction_supports_mixed_inline_and_category_line_formats() {
        let markdown = r#"## Findings

1. **[metrics-ownership] Inline finding**

2. **Separate category finding**
   Category: Review Accounting
"#;

        assert_eq!(
            extract_categories(markdown),
            vec![
                "metrics-ownership".to_string(),
                "review-accounting".to_string(),
            ]
        );
    }

    #[test]
    fn category_extraction_collects_multiple_inline_categories() {
        let markdown = r#"## Findings

1. **[metrics-ownership] First finding**

2. **[tooling-contract] Second finding**

3. **[review-accounting] Third finding**
"#;

        assert_eq!(
            extract_categories(markdown),
            vec![
                "metrics-ownership".to_string(),
                "review-accounting".to_string(),
                "tooling-contract".to_string(),
            ]
        );
    }

    #[test]
    fn category_extraction_uses_first_inline_bracket_pair_only() {
        let markdown = r#"## Findings

1. **[category-name] Title with [other] brackets**
"#;

        assert_eq!(
            extract_categories(markdown),
            vec!["category-name".to_string()]
        );
    }

    #[test]
    fn category_extraction_parses_cycle_196_inline_categories() {
        assert_eq!(
            extract_categories(CYCLE_196_REVIEW),
            vec![
                "metrics-ownership".to_string(),
                "process-adherence".to_string(),
                "review-accounting".to_string(),
                "tooling-contract".to_string(),
            ]
        );
    }

    #[test]
    fn category_extraction_deduplicates_short_categories() {
        let markdown = r#"## Findings

1. **State consistency remains visible across multiple validation surfaces.**
   Category: Data Integrity
2. **Data integrity remains the priority across all repository state accounting checks.**
   Category: Data Integrity
3. **Process integrity needs follow-up across several long-running workflow maintenance paths.**
   Category: Process Integrity
"#;

        assert_eq!(
            extract_categories(markdown),
            vec![
                "data-integrity".to_string(),
                "process-integrity".to_string(),
            ]
        );
    }

    #[test]
    fn parsing_ignores_code_blocks_and_paths() {
        let markdown = r#"## Findings

1. **Real finding**
   Category: First Category
   Description with a real path: `docs/reviews/cycle-170.md`

```text
1. **Not a real finding**
   Category: bogus-category
/home/runner/work/schema-org-json-ld/schema-org-json-ld/tools/rust/crates/process-review/src/main.rs
```

2. **Second real finding**
   Category: Second Category

## Recommendations

1. Leave the code block alone.
"#;

        assert_eq!(extract_finding_count(markdown), Some(2));
        assert_eq!(
            extract_categories(markdown),
            vec!["first-category".to_string(), "second-category".to_string()]
        );
    }

    #[test]
    fn history_entry_construction_uses_cli_overrides() {
        let parsed = ParsedReview {
            cycle: 162,
            finding_count: 7,
            complacency_score: 2,
            categories: vec!["data-integrity".to_string()],
        };
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--actioned",
            "1",
            "--deferred",
            "1",
            "--ignored",
            "5",
            "--note",
            "triaged",
        ]);

        let entry = build_history_entry(&parsed, &cli);
        assert_eq!(entry.cycle, 162);
        assert_eq!(entry.finding_count, 7);
        assert_eq!(entry.complacency_score, 2);
        assert_eq!(entry.actioned, 1);
        assert_eq!(entry.deferred, 1);
        assert_eq!(entry.ignored, 5);
        assert_eq!(entry.note.as_deref(), Some("triaged"));
    }

    #[test]
    fn disposition_validation_rejects_default_counts_when_findings_exist() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
        ]);

        let error = validate_dispositions(&cli, 3).expect_err("validation should fail");
        assert!(error.contains("review contains 3 findings"));
        assert!(error.contains("--skip-disposition-check"));
    }

    #[test]
    fn disposition_validation_passes_when_counts_sum_correctly() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--actioned",
            "2",
            "--deferred",
            "1",
        ]);

        assert_eq!(validate_dispositions(&cli, 3), Ok(()));
    }

    #[test]
    fn disposition_validation_skip_flag_bypasses_check() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--skip-disposition-check",
        ]);

        assert_eq!(validate_dispositions(&cli, 3), Ok(()));
    }

    #[test]
    fn disposition_validation_allows_zero_findings_with_zero_counts() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
        ]);

        assert_eq!(validate_dispositions(&cli, 0), Ok(()));
    }

    #[test]
    fn disposition_validation_warns_but_allows_non_default_mismatch() {
        let cli = Cli::parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--actioned",
            "1",
        ]);

        assert_eq!(validate_dispositions(&cli, 3), Ok(()));
    }

    #[test]
    fn state_patch_generation_updates_expected_paths() {
        let state = json!({
            "last_cycle": {"number": 163},
            "review_agent": {
                "last_review_cycle": 162,
                "history": [
                    {"cycle": 162, "finding_count": 7, "complacency_score": 2, "categories": ["a"], "actioned": 1, "deferred": 1, "ignored": 5}
                ]
            },
            "field_inventory": {
                "fields": {
                    "review_agent": {"last_refreshed": "cycle 162"}
                }
            }
        });

        let entry = ReviewHistoryEntry {
            cycle: 163,
            finding_count: 3,
            complacency_score: 1,
            categories: vec!["state-consistency".to_string()],
            actioned: 1,
            deferred: 1,
            ignored: 1,
            note: None,
        };

        let patch = build_state_patch(&state, 163, 163, &entry).expect("patch should build");
        assert_eq!(patch.len(), 3);
        assert_eq!(patch[0].path, "/review_agent/last_review_cycle");
        assert_eq!(patch[1].path, "/review_agent/history");
        assert_eq!(
            patch[2].path,
            "/field_inventory/fields/review_agent/last_refreshed"
        );

        let history = patch[1]
            .value
            .as_array()
            .expect("history value should be array");
        assert_eq!(history.len(), 2);
        assert_eq!(
            history.last().and_then(|value| value.get("cycle")),
            Some(&json!(163))
        );
    }

    #[test]
    fn parse_review_extracts_cycle_from_filename() {
        let path = Path::new("docs/reviews/cycle-162.md");
        let parsed = parse_review(path, SAMPLE_REVIEW).expect("parse should succeed");
        assert_eq!(parsed.cycle, 162);
        assert_eq!(parsed.complacency_score, 2);
        assert_eq!(parsed.finding_count, 3);
    }
}
