use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};
use state_schema::{
    commit_state_json, current_cycle_from_state, read_state_value, set_value_at_pointer,
    write_state_value,
};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

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

    /// Warn on malformed review artifacts instead of failing before state updates
    #[arg(long, default_value_t = false)]
    lenient: bool,

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

    let parsed_review = parse_review(&review_path, &review_content, cli.lenient)?;
    validate_dispositions(&cli, parsed_review.finding_count)?;
    let entry = build_history_entry(&parsed_review, &cli);

    let mut state_value = read_state_value(&cli.repo_root)?;
    let current_cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /last_cycle/number in state.json" {
            "missing numeric /last_cycle/number in docs/state.json".to_string()
        } else {
            error
        }
    })?;

    let patch = build_state_patch(&state_value, parsed_review.cycle, current_cycle, &entry)?;
    apply_patch(&mut state_value, &patch)?;
    write_state_value(&cli.repo_root, &state_value)?;

    let commit_message = format!(
        "state(process-review): cycle {} review consumed, score {}/5 [cycle {}]",
        parsed_review.cycle, parsed_review.complacency_score, current_cycle
    );
    let receipt = commit_state_json(&cli.repo_root, &commit_message)?;

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

fn parse_review(review_path: &Path, content: &str, lenient: bool) -> Result<ParsedReview, String> {
    let cycle = extract_cycle_number(review_path).ok_or_else(|| {
        format!(
            "failed to derive cycle number from {}",
            review_path.display()
        )
    })?;

    let validation_errors = validate_review_format(content);
    if !validation_errors.is_empty() {
        if lenient {
            for error in &validation_errors {
                eprintln!(
                    "Warning: {} (continuing because --lenient was passed)",
                    error
                );
            }
        } else {
            return Err(validation_errors.join("; "));
        }
    }

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

fn validate_review_format(content: &str) -> Vec<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut errors = Vec::new();
    let mut found_heading = false;
    for_each_finding_line(&lines, |index, trimmed| {
        if let Some(number) = numbered_finding_number(trimmed) {
            found_heading = true;

            match resolve_finding_category(trimmed, next_non_empty_line(&lines, index + 1)) {
                Some(raw_category) => {
                    if normalize_category(raw_category).is_none() {
                        errors.push(format!("Finding {} has an invalid category tag", number));
                    }
                }
                None => errors.push(format!("Finding {} has no category tag", number)),
            }
        }
    });

    if !found_heading {
        errors.push("review markdown must contain at least one numbered finding heading in the Findings section".to_string());
    }

    if extract_score(content).is_none() {
        errors.push(
            "review markdown must contain a complacency score section with a parsable N/5 score"
                .to_string(),
        );
    }

    errors
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
    let list_count = count_numbered_findings(content);
    if list_count > 0 {
        return Some(list_count as u64);
    }

    None
}

fn count_numbered_findings(content: &str) -> usize {
    let lines: Vec<&str> = content.lines().collect();
    let mut count = 0usize;

    for_each_finding_line(&lines, |_, trimmed| {
        if is_numbered_finding_heading(trimmed) {
            count += 1;
        }
    });

    count
}

fn is_numbered_finding_heading(line: &str) -> bool {
    numbered_finding_number(line).is_some()
}

fn numbered_finding_number(line: &str) -> Option<u64> {
    let mut chars = strip_hash_heading_prefix(line).chars().peekable();
    let mut digits = String::new();
    while let Some(ch) = chars.peek() {
        if ch.is_ascii_digit() {
            digits.push(*ch);
            chars.next();
        } else {
            break;
        }
    }

    if digits.is_empty() {
        return None;
    }

    if !matches!(chars.next(), Some('.')) {
        return None;
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

    if !saw_whitespace {
        return None;
    }

    let remainder: String = chars.collect();
    let remainder = remainder.trim_start();

    if !(remainder.starts_with("**") || remainder.starts_with('[')) {
        return None;
    }

    digits.parse::<u64>().ok()
}

fn extract_categories(content: &str) -> Vec<String> {
    let lines: Vec<&str> = content.lines().collect();
    let mut categories = BTreeSet::new();
    for_each_finding_line(&lines, |index, trimmed| {
        if is_numbered_finding_heading(trimmed) {
            if let Some(raw) =
                resolve_finding_category(trimmed, next_non_empty_line(&lines, index + 1))
            {
                if let Some(normalized) = normalize_category(raw) {
                    categories.insert(normalized);
                }
            }
        }
    });

    categories.into_iter().collect()
}

fn for_each_finding_line(lines: &[&str], mut visitor: impl FnMut(usize, &str)) {
    match findings_line_bounds(lines) {
        Some((start, end)) => {
            let mut in_code_block = false;
            for (offset, line) in lines[start..end].iter().enumerate() {
                let trimmed = line.trim();
                if trimmed.starts_with("```") {
                    in_code_block = !in_code_block;
                    continue;
                }

                if in_code_block {
                    continue;
                }

                visitor(start + offset, trimmed);
            }
        }
        None => {
            let mut in_code_block = false;
            for (index, line) in lines.iter().enumerate() {
                let trimmed = line.trim();
                if trimmed.starts_with("```") {
                    in_code_block = !in_code_block;
                    continue;
                }

                if in_code_block {
                    continue;
                }

                visitor(index, trimmed);
            }
        }
    }
}

fn findings_line_bounds(lines: &[&str]) -> Option<(usize, usize)> {
    let start = lines
        .iter()
        .position(|line| line.trim().to_ascii_lowercase().starts_with("## findings"))?;

    for (offset, line) in lines[start + 1..].iter().enumerate() {
        let trimmed = line.trim();
        let lower = trimmed.to_ascii_lowercase();
        if lower.starts_with("## ")
            && !lower.starts_with("## findings")
            && !matches_numbered_finding_with_hash_prefix(trimmed)
        {
            return Some((start + 1, start + 1 + offset));
        }
    }

    Some((start + 1, lines.len()))
}

fn extract_inline_category(line: &str) -> Option<&str> {
    let remainder = strip_hash_heading_prefix(line);
    let (_, remainder) = remainder
        .split_once("**[")
        .or_else(|| remainder.split_once('['))?;
    let (category, _) = remainder.split_once(']')?;
    if category.is_empty() {
        None
    } else {
        Some(category)
    }
}

fn extract_category_line(line: &str) -> Option<&str> {
    let raw = line.trim_start().strip_prefix("Category:")?.trim();
    if raw.is_empty() {
        None
    } else {
        Some(raw)
    }
}

fn next_non_empty_line<'a>(lines: &[&'a str], start_index: usize) -> Option<&'a str> {
    if start_index >= lines.len() {
        return None;
    }

    lines[start_index..]
        .iter()
        .copied()
        .find(|line| !line.trim().is_empty())
}

fn resolve_finding_category<'a>(
    heading_line: &'a str,
    next_non_empty_line: Option<&'a str>,
) -> Option<&'a str> {
    extract_inline_category(heading_line)
        .or_else(|| next_non_empty_line.and_then(extract_category_line))
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

fn strip_hash_heading_prefix(line: &str) -> &str {
    match line.strip_prefix("## ") {
        Some(remainder)
            if remainder
                .chars()
                .next()
                .is_some_and(|ch| ch.is_ascii_digit()) =>
        {
            remainder
        }
        _ => line,
    }
}

fn matches_numbered_finding_with_hash_prefix(line: &str) -> bool {
    matches!(
        line.strip_prefix("## "),
        Some(remainder) if remainder
            .chars()
            .next()
            .is_some_and(|ch| ch.is_ascii_digit())
            && numbered_finding_number(remainder).is_some()
    )
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

    const CYCLE_196_REVIEW: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../docs/reviews/cycle-196.md"
    ));
    const CYCLE_197_REVIEW: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../docs/reviews/cycle-197.md"
    ));
    const CYCLE_205_REVIEW: &str = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../../docs/reviews/cycle-205.md"
    ));

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
        assert!(help.contains("--lenient"));
        assert!(help.contains("--note"));
    }

    #[test]
    fn cli_accepts_lenient_flag() {
        assert!(Cli::try_parse_from([
            "process-review",
            "--review-file",
            "docs/reviews/cycle-162.md",
            "--lenient",
        ])
        .is_ok());
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
    fn numbered_finding_number_supports_hash_prefixed_findings() {
        assert_eq!(
            numbered_finding_number("## 1. [worklog-accuracy] Finding title"),
            Some(1)
        );
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
    fn category_extraction_reads_hash_prefixed_heading_category() {
        let markdown = r#"## Findings

## 1. [worklog-accuracy] First finding

## 2. [tooling-contract] Second finding
"#;

        assert_eq!(
            extract_categories(markdown),
            vec![
                "tooling-contract".to_string(),
                "worklog-accuracy".to_string(),
            ]
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
    fn category_extraction_parses_cycle_196_artifact_with_inline_categories() {
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
    fn finding_count_supports_mixed_hash_prefixed_findings() {
        let markdown = r#"## Findings

1. **[metrics-ownership] Legacy finding**

## 2. [worklog-accuracy] Hash-prefixed finding

## 3. [tooling-contract] Another hash-prefixed finding

## Recommendations

1. Leave the parser alone.
"#;

        assert_eq!(count_numbered_findings(markdown), 3);
    }

    #[test]
    fn finding_count_falls_back_without_findings_header() {
        let markdown = r#"# Cycle 213 Review

## 1. [review-evidence] First finding

## 2. [tooling-contract] Second finding

## Complacency score

**4/5**
"#;

        assert_eq!(extract_finding_count(markdown), Some(2));
        assert_eq!(extract_score(markdown), Some(4));
    }

    #[test]
    fn category_extraction_falls_back_without_findings_header() {
        let markdown = r#"# Cycle 213 Review

## 1. [review-evidence] First finding

## 2. [tooling-contract] Second finding

## Complacency score

**4/5**
"#;

        assert_eq!(
            extract_categories(markdown),
            vec![
                "review-evidence".to_string(),
                "tooling-contract".to_string(),
            ]
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
        let parsed = parse_review(path, SAMPLE_REVIEW, true).expect("parse should succeed");
        assert_eq!(parsed.cycle, 162);
        assert_eq!(parsed.complacency_score, 2);
        assert_eq!(parsed.finding_count, 3);
    }

    #[test]
    fn parse_review_accepts_category_line_tags_by_default() {
        let path = Path::new("docs/reviews/cycle-162.md");
        let markdown = r#"## Findings

1. **Finding title**
Category: Review Accounting

## Complacency score

**2/5**
"#;
        let parsed = parse_review(path, markdown, false).expect("parse should succeed");
        assert_eq!(parsed.categories, vec!["review-accounting".to_string()]);
    }

    #[test]
    fn parse_review_accepts_findings_without_findings_header() {
        let path = Path::new("docs/reviews/cycle-213.md");
        let markdown = r#"# Cycle 213 Review

## 1. [review-evidence] First finding

## 2. [tooling-contract] Second finding

## Complacency score

**4/5**
"#;

        let parsed = parse_review(path, markdown, false).expect("parse should succeed");
        assert_eq!(parsed.cycle, 213);
        assert_eq!(parsed.finding_count, 2);
        assert_eq!(parsed.complacency_score, 4);
        assert_eq!(
            parsed.categories,
            vec![
                "review-evidence".to_string(),
                "tooling-contract".to_string(),
            ]
        );
    }

    #[test]
    fn parse_review_rejects_missing_category_by_default() {
        let path = Path::new("docs/reviews/cycle-162.md");
        let error = parse_review(path, SAMPLE_REVIEW, false).expect_err("parse should fail");
        assert!(error.contains("Finding 3 has no category tag"));
    }

    #[test]
    fn parse_review_accepts_cycle_197_artifact() {
        let path = Path::new("docs/reviews/cycle-197.md");
        let parsed = parse_review(path, CYCLE_197_REVIEW, false).expect("parse should succeed");
        assert_eq!(parsed.cycle, 197);
        assert_eq!(parsed.complacency_score, 5);
        assert_eq!(parsed.finding_count, 4);
    }

    #[test]
    fn parse_review_accepts_cycle_205_artifact() {
        let path = Path::new("docs/reviews/cycle-205.md");
        let parsed = parse_review(path, CYCLE_205_REVIEW, false).expect("parse should succeed");
        assert_eq!(parsed.cycle, 205);
        assert_eq!(parsed.complacency_score, 4);
        assert_eq!(parsed.finding_count, 4);
        assert_eq!(
            parsed.categories,
            vec![
                "complacency-audit".to_string(),
                "state-integrity".to_string(),
                "tooling-contract".to_string(),
                "worklog-accuracy".to_string(),
            ]
        );
    }

    #[test]
    fn parse_review_lenient_mode_accepts_legacy_category_lines() {
        let path = Path::new("docs/reviews/cycle-162.md");
        let parsed = parse_review(path, SAMPLE_REVIEW, true).expect("parse should succeed");
        assert_eq!(
            parsed.categories,
            vec![
                "data-integrity".to_string(),
                "process-integrity".to_string()
            ]
        );
    }

    #[test]
    fn review_format_validation_rejects_missing_finding_headings() {
        let markdown = r#"## Findings

No numbered findings here.

## Complacency score

**2/5**
"#;

        assert_eq!(
            validate_review_format(markdown),
            vec![
                "review markdown must contain at least one numbered finding heading in the Findings section"
                    .to_string(),
            ]
        );
    }

    #[test]
    fn review_format_validation_rejects_missing_complacency_score_section() {
        let markdown = r#"## Findings

1. **[tooling-contract] Finding title**
"#;

        assert_eq!(
            validate_review_format(markdown),
            vec![
                "review markdown must contain a complacency score section with a parsable N/5 score"
                    .to_string(),
            ]
        );
    }

    #[test]
    fn review_format_validation_accepts_hash_prefixed_findings() {
        let markdown = r#"## Findings

## 1. [worklog-accuracy] Finding title

## Complacency score

4/5 — actual score
"#;

        assert!(validate_review_format(markdown).is_empty());
    }

    #[test]
    fn review_format_validation_accepts_category_lines() {
        let markdown = r#"## Findings

1. **Finding title**
Category: Review Accounting

## Complacency score

4/5 — actual score
"#;

        assert!(validate_review_format(markdown).is_empty());
    }

    #[test]
    fn review_format_validation_accepts_findings_without_findings_header() {
        let markdown = r#"# Cycle 213 Review

## 1. [review-evidence] Finding title

## Complacency score

4/5 — actual score
"#;

        assert!(validate_review_format(markdown).is_empty());
    }
}
