use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};
use state_schema::set_value_at_pointer;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    let entry = build_history_entry(&parsed_review, &cli);

    let state_path = cli.repo_root.join("docs/state.json");
    let mut state_value = read_state_value(&state_path)?;
    let current_cycle = read_current_cycle(&state_value)?;

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
    for line in content.lines() {
        if !line.contains("/5") {
            continue;
        }

        if let Some((score, _)) = find_number_before_token(line, "/5") {
            return Some(score);
        }
    }

    None
}

fn extract_finding_count(content: &str) -> Option<u64> {
    // Priority 1: Look for explicit "## Number of findings" heading, take number from next line
    let lines: Vec<&str> = content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
        let lower = line.to_ascii_lowercase();
        if lower.trim().starts_with("## number of finding") {
            // Check the next non-empty line for the count
            for next_line in &lines[i + 1..] {
                let trimmed = next_line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if let Some(number) = first_number_in_text(trimmed) {
                    return Some(number);
                }
                break; // first non-empty line had no number, stop looking
            }
        }
    }

    // Priority 2: Look for "N findings" pattern (number immediately before "findings")
    for line in content.lines() {
        let lower = line.to_ascii_lowercase();
        if let Some(idx) = lower.find("finding") {
            // Look for a number immediately before "finding" (with optional whitespace)
            let before = &line[..idx];
            let trimmed_before = before.trim_end();
            if let Some(number) = last_number_in_text(trimmed_before) {
                return Some(number);
            }
        }
    }

    // Priority 3: Count finding headings
    let finding_heading_count = content
        .lines()
        .filter(|line| {
            let trimmed = line.trim();
            let lower = trimmed.to_ascii_lowercase();
            lower.starts_with("**finding ") || lower.starts_with("### finding ")
        })
        .count();
    if finding_heading_count > 0 {
        return Some(finding_heading_count as u64);
    }

    // Priority 4: Count numbered list items in ## Findings section
    let list_count = count_numbered_findings_in_findings_section(content);
    if list_count > 0 {
        return Some(list_count as u64);
    }

    None
}

fn count_numbered_findings_in_findings_section(content: &str) -> usize {
    let mut in_findings = false;
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

        if is_numbered_list_item(trimmed) {
            count += 1;
        }
    }

    count
}

fn is_numbered_list_item(line: &str) -> bool {
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

    matches!(chars.next(), Some('.'))
}

fn extract_categories(content: &str) -> Vec<String> {
    let mut categories = BTreeSet::new();

    for line in content.lines() {
        let trimmed = line.trim();
        let lower = trimmed.to_ascii_lowercase();

        if let Some(index) = lower.find("category:") {
            let raw = trimmed[(index + "category:".len())..].trim();
            for candidate in split_category_candidates(raw) {
                if let Some(normalized) = normalize_category(candidate) {
                    categories.insert(normalized);
                }
            }
            continue;
        }

        if lower.starts_with("### ") || lower.starts_with("#### ") {
            let heading = trimmed.trim_start_matches('#').trim();
            let heading_lower = heading.to_ascii_lowercase();
            if heading_lower.starts_with("finding ") {
                continue;
            }
            if let Some(normalized) = normalize_category(heading) {
                categories.insert(normalized);
            }
        }

        if let Some(title) = extract_bold_finding_title(trimmed) {
            if let Some(normalized) = normalize_category(title) {
                categories.insert(normalized);
            }
        }
    }

    categories.into_iter().collect()
}

fn extract_bold_finding_title(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if !is_numbered_list_item(trimmed) {
        return None;
    }

    let start = trimmed.find("**")?;
    let rest = &trimmed[(start + 2)..];
    let end = rest.find("**")?;
    Some(rest[..end].trim())
}

fn split_category_candidates(raw: &str) -> Vec<&str> {
    let mut head = raw;
    for delimiter in [")", "—", " - ", ":"] {
        if let Some(index) = head.find(delimiter) {
            head = &head[..index];
        }
    }

    head.split(&[',', '/', ';'][..]).collect()
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
    if trimmed.is_empty() {
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

fn last_number_in_text(text: &str) -> Option<u64> {
    let mut last_number: Option<u64> = None;
    let mut current = String::new();
    for ch in text.chars() {
        if ch.is_ascii_digit() {
            current.push(ch);
        } else if !current.is_empty() {
            last_number = current.parse::<u64>().ok();
            current.clear();
        }
    }
    if !current.is_empty() {
        last_number = current.parse::<u64>().ok();
    }
    last_number
}

fn first_number_in_text(text: &str) -> Option<u64> {
    let mut number = String::new();
    for ch in text.chars() {
        if ch.is_ascii_digit() {
            number.push(ch);
        } else if !number.is_empty() {
            return number.parse::<u64>().ok();
        }
    }

    if number.is_empty() {
        None
    } else {
        number.parse::<u64>().ok()
    }
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

fn read_current_cycle(state: &Value) -> Result<u64, String> {
    state
        .pointer("/last_cycle/number")
        .and_then(Value::as_u64)
        .ok_or_else(|| "missing numeric /last_cycle/number in docs/state.json".to_string())
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
   (Category: Data Integrity)
2. **Field freshness gaps continue in one path.**
   Category: Process Integrity
3. **Arithmetic checks now pass for metrics.**

## Recommendations

1. Automate more state updates.

## Complacency score

**Complacency Score: 2/5**

7 findings recorded in this review.
"#;

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
        assert!(help.contains("--note"));
    }

    #[test]
    fn score_extraction_finds_first_match() {
        let markdown = "noise\nScore: 3/5\n**Complacency Score: 2/5**\n";
        assert_eq!(extract_score(markdown), Some(3));
    }

    #[test]
    fn finding_count_extraction_reads_findings_line() {
        assert_eq!(extract_finding_count(SAMPLE_REVIEW), Some(7));
    }

    #[test]
    fn finding_count_prefers_number_of_findings_heading() {
        // This is the format that caused the bug: "## Number of findings" heading
        // with the count on the next line, and a later line mentioning "cycle-162" + "findings"
        let markdown = r#"# Cycle 163 Review

## Complacency score

**3/5**

## Number of findings

**5**

## Findings

1. **Category:** state-consistency
   **Description:** The cycle-162 review history ingestion is accurate and reconciles with `docs/reviews/cycle-162.md` (7 findings, score 2/5).
2. **Category:** state-freshness
3. **Category:** review-accounting
4. **Category:** release-governance
5. **Category:** process-traceability
"#;
        assert_eq!(extract_finding_count(markdown), Some(5));
    }

    #[test]
    fn finding_count_extraction_falls_back_to_numbered_list() {
        let markdown = r#"## Findings
1. One
2. Two
## Recommendations
1. R
"#;
        assert_eq!(extract_finding_count(markdown), Some(2));
    }

    #[test]
    fn category_extraction_normalizes_values() {
        let categories = extract_categories(SAMPLE_REVIEW);
        assert_eq!(
            categories,
            vec![
                "arithmetic-checks-now-pass-for-metrics".to_string(),
                "data-integrity".to_string(),
                "field-freshness-gaps-continue-in-one-path".to_string(),
                "process-integrity".to_string(),
                "state-consistency-drift-remains-visible".to_string(),
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
        assert_eq!(parsed.finding_count, 7);
    }
}
