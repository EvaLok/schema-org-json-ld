use clap::Parser;
use serde::Serialize;
use state_schema::{current_cycle_from_state, ReviewHistoryEntry, StateJson};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "check-commitments")]
struct Cli {
    #[arg(long)]
    cycle: Option<u64>,

    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JournalSection<'a> {
    heading: &'a str,
    body: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JournalMatch {
    path: String,
    section: JournalSectionOwned,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct JournalSectionOwned {
    heading: String,
    body: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct Escalation {
    category: String,
    consecutive_cycles: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
struct Report {
    journal: String,
    cycle: u64,
    commitments: Vec<String>,
    deferred_review_findings: Vec<Escalation>,
}

fn main() {
    let cli = Cli::parse();
    let json = cli.json;
    match run(cli) {
        Ok(report) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&report)
                        .expect("report serialization should succeed")
                );
            } else {
                print_human_report(&report);
            }
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn run(cli: Cli) -> Result<Report, String> {
    let cycle = resolve_cycle(cli.cycle, &cli.repo_root)?;
    let journal = find_journal_section(&cli.repo_root, cycle)?;
    let commitments = extract_commitments(&journal.section.body);
    let state = read_state_json(&cli.repo_root)?;
    let review_agent = state.review_agent()?;

    Ok(Report {
        journal: journal.path,
        cycle,
        commitments,
        deferred_review_findings: find_deferred_escalations(&review_agent.history),
    })
}

fn print_human_report(report: &Report) {
    println!(
        "Journal: {} (cycle {} section)",
        report.journal, report.cycle
    );
    println!();
    println!("Commitments found:");
    if report.commitments.is_empty() {
        println!("  - None");
    } else {
        for (index, commitment) in report.commitments.iter().enumerate() {
            println!("  {}. {}", index + 1, commitment);
        }
    }
    println!();
    println!("Deferred review findings (3+ cycles):");
    if report.deferred_review_findings.is_empty() {
        println!("  - None");
    } else {
        for escalation in &report.deferred_review_findings {
            println!(
                "  - {} (cycles {})",
                escalation.category,
                format_cycle_list(&escalation.consecutive_cycles)
            );
        }
    }
    println!();
    println!(
        "Result: {} commitments found — verify manually",
        report.commitments.len()
    );
}

fn resolve_cycle(cli_cycle: Option<u64>, repo_root: &Path) -> Result<u64, String> {
    match cli_cycle {
        Some(cycle) => Ok(cycle),
        None => {
            let current_cycle = current_cycle_from_state(repo_root)?;
            current_cycle.checked_sub(1).ok_or_else(|| {
                "current cycle must be at least 1 to infer previous cycle".to_string()
            })
        }
    }
}

fn read_state_json(repo_root: &Path) -> Result<StateJson, String> {
    let state_path = repo_root.join("docs/state.json");
    let content = fs::read_to_string(&state_path)
        .map_err(|error| format!("failed to read {}: {}", state_path.display(), error))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("failed to parse {}: {}", state_path.display(), error))
}

fn find_journal_section(repo_root: &Path, cycle: u64) -> Result<JournalMatch, String> {
    let journal_dir = repo_root.join("docs/journal");
    let mut entries = fs::read_dir(&journal_dir)
        .map_err(|error| format!("failed to read {}: {}", journal_dir.display(), error))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("failed to enumerate {}: {}", journal_dir.display(), error))?;
    entries.sort_by_key(|entry| entry.path());

    for entry in entries {
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) != Some("md") {
            continue;
        }

        let content = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
        if let Some(section) = extract_cycle_section(&content, cycle) {
            let relative_path = path
                .strip_prefix(repo_root)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            return Ok(JournalMatch {
                path: relative_path,
                section: JournalSectionOwned {
                    heading: section.heading.to_string(),
                    body: section.body.to_string(),
                },
            });
        }
    }

    Err(format!(
        "no docs/journal entry contains a section for cycle {}",
        cycle
    ))
}

fn extract_cycle_section<'a>(content: &'a str, cycle: u64) -> Option<JournalSection<'a>> {
    let target = format!("Cycle {}:", cycle);
    let mut heading: Option<&str> = None;
    let mut body_start = 0usize;
    let mut offset = 0usize;

    for line in content.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if let Some(matched_heading) = heading {
            if trimmed == "---" || trimmed.starts_with("## ") {
                return Some(JournalSection {
                    heading: matched_heading,
                    body: content[body_start..offset].trim(),
                });
            }
        } else if trimmed.starts_with("## ") && trimmed.contains(&target) {
            heading = Some(trimmed);
            body_start = offset + line.len();
        }
        offset += line.len();
    }

    heading.map(|matched_heading| JournalSection {
        heading: matched_heading,
        body: content[body_start..].trim(),
    })
}

fn extract_commitments(section: &str) -> Vec<String> {
    let subsections = parse_subsections(section);
    let mut commitments = Vec::new();
    let mut seen = BTreeSet::new();

    if let Some(concrete_section) = subsections.iter().find(|subsection| {
        subsection
            .title
            .is_some_and(|title| title.eq_ignore_ascii_case("Concrete commitments for next cycle"))
    }) {
        for line in &concrete_section.lines {
            if let Some(item) = extract_list_item(line) {
                push_commitment(&mut commitments, &mut seen, item);
            }
        }
    }

    for subsection in &subsections {
        if subsection
            .title
            .is_some_and(|title| title.eq_ignore_ascii_case("Previous commitment follow-through"))
        {
            continue;
        }

        if subsection
            .title
            .is_some_and(|title| title.eq_ignore_ascii_case("Concrete commitments for next cycle"))
        {
            continue;
        }

        for line in &subsection.lines {
            if contains_commitment_phrase(line) {
                push_commitment(&mut commitments, &mut seen, line);
            }
        }
    }

    commitments
}

fn parse_subsections(section: &str) -> Vec<Subsection<'_>> {
    let mut subsections = Vec::new();
    let mut current = Subsection {
        title: None,
        lines: Vec::new(),
    };

    for line in section.lines() {
        let trimmed = line.trim();
        if let Some(title) = trimmed.strip_prefix("### ") {
            subsections.push(current);
            current = Subsection {
                title: Some(title.trim()),
                lines: Vec::new(),
            };
        } else {
            current.lines.push(trimmed);
        }
    }

    subsections.push(current);
    subsections
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Subsection<'a> {
    title: Option<&'a str>,
    lines: Vec<&'a str>,
}

fn extract_list_item(line: &str) -> Option<&str> {
    let trimmed = line.trim();
    if let Some(item) = trimmed.strip_prefix("- ") {
        return Some(item.trim());
    }
    if let Some(item) = trimmed.strip_prefix("* ") {
        return Some(item.trim());
    }

    let digits = trimmed
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .count();
    if digits == 0 {
        return None;
    }

    let remainder = &trimmed[digits..];
    let remainder = remainder
        .strip_prefix('.')
        .or_else(|| remainder.strip_prefix(')'))?;
    Some(remainder.trim())
}

fn contains_commitment_phrase(line: &str) -> bool {
    let cleaned = clean_commitment_text(line);
    if cleaned.is_empty() {
        return false;
    }

    let lowered = format!(" {} ", cleaned.to_ascii_lowercase());
    lowered.contains(" will ")
        || lowered.contains(" must ")
        || lowered.contains(" plan to ")
        || lowered.contains(" needs to ")
        || lowered.contains(" commit to ")
        || contains_should_commitment(&cleaned)
}

fn contains_should_commitment(cleaned: &str) -> bool {
    let lowered = cleaned.to_ascii_lowercase();
    let subjects = ["i", "we", "this", "that", "it", "they", "you"];

    lowered.starts_with("should ")
        || subjects.iter().any(|subject| {
            lowered.starts_with(&format!("{subject} should "))
                || lowered.starts_with(&format!("{subject} should be "))
        })
}

fn push_commitment(commitments: &mut Vec<String>, seen: &mut BTreeSet<String>, text: &str) {
    let cleaned = clean_commitment_text(text);
    if cleaned.is_empty() {
        return;
    }

    let key = cleaned.to_ascii_lowercase();
    if seen.insert(key) {
        commitments.push(cleaned);
    }
}

fn clean_commitment_text(text: &str) -> String {
    normalize_whitespace(&strip_markdown(strip_list_prefix(text.trim())))
}

fn strip_list_prefix(text: &str) -> &str {
    extract_list_item(text).unwrap_or(text).trim()
}

fn strip_markdown(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();

    while let Some(character) = chars.next() {
        if character == '[' {
            let mut label = String::new();
            let mut found_close_bracket = false;
            for next in chars.by_ref() {
                if next == ']' {
                    found_close_bracket = true;
                    break;
                }
                label.push(next);
            }

            if found_close_bracket && chars.peek() == Some(&'(') {
                chars.next();
                let mut depth = 1usize;
                for next in chars.by_ref() {
                    match next {
                        '(' => depth += 1,
                        ')' => {
                            depth -= 1;
                            if depth == 0 {
                                break;
                            }
                        }
                        _ => {}
                    }
                }
                result.push_str(&label);
                continue;
            }

            result.push('[');
            result.push_str(&label);
            continue;
        }

        if matches!(character, '*' | '_' | '`') {
            continue;
        }

        result.push(character);
    }

    result
}

fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn find_deferred_escalations(history: &[ReviewHistoryEntry]) -> Vec<Escalation> {
    let mut cycles_by_category: BTreeMap<String, Vec<u64>> = BTreeMap::new();

    for entry in history {
        for category in deferred_categories(entry) {
            cycles_by_category
                .entry(category)
                .or_default()
                .push(entry.cycle);
        }
    }

    let mut escalations = Vec::new();
    for (category, mut cycles) in cycles_by_category {
        cycles.sort_unstable();
        cycles.dedup();

        let mut streak = Vec::new();
        for cycle in cycles {
            if streak.last().is_none_or(|previous| *previous + 1 == cycle) {
                streak.push(cycle);
            } else {
                if streak.len() >= 3 {
                    escalations.push(Escalation {
                        category: category.clone(),
                        consecutive_cycles: streak.clone(),
                    });
                }
                streak.clear();
                streak.push(cycle);
            }
        }

        if streak.len() >= 3 {
            escalations.push(Escalation {
                category,
                consecutive_cycles: streak,
            });
        }
    }

    escalations
}

fn deferred_categories(entry: &ReviewHistoryEntry) -> Vec<String> {
    if entry.deferred == 0 {
        return Vec::new();
    }

    let explicit = explicit_deferred_categories(entry);
    if !explicit.is_empty() {
        return explicit;
    }

    entry.categories.clone()
}

fn explicit_deferred_categories(entry: &ReviewHistoryEntry) -> Vec<String> {
    let Some(note) = entry.note.as_deref() else {
        return Vec::new();
    };

    let lowered = note.to_ascii_lowercase();
    if !lowered.contains("deferred") {
        return Vec::new();
    }

    entry
        .categories
        .iter()
        .filter(|category| lowered.contains(&category.to_ascii_lowercase()))
        .cloned()
        .collect()
}

fn format_cycle_list(cycles: &[u64]) -> String {
    cycles
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_JOURNAL: &str = r#"# Journal — 2026-03-09

---

## 2026-03-09 — Cycle 199: Accountability reckoning and tool dispatch

### Previous commitment follow-through

1. **"Review PR from #835"** — **Followed**

### Concrete commitments for next cycle

1. Review PRs from #843 and #844 when Copilot finishes
2. **Fix cycle-close-drift**: Rewrite COMPLETION_CHECKLIST steps 5-7 to dispatch review before final state commit
3. Dispatch #825 and #826 if concurrency allows

---

## 2026-03-09 — Cycle 200: Milestone cycle — tool delivery and debt resolution

### Observation

I will verify the next merge carefully.

### Concrete commitments for next cycle

1. Review PRs from #850 and #851 when Copilot finishes
2. Dispatch #827 and #828 if concurrency allows
3. Fix cycle-receipts timing bug

### Notes

This should stay focused.
"#;

    #[test]
    fn extract_cycle_section_returns_target_cycle_body() {
        let section =
            extract_cycle_section(SAMPLE_JOURNAL, 200).expect("cycle section should exist");
        assert!(section.heading.contains("Cycle 200"));
        assert!(section.body.contains("Fix cycle-receipts timing bug"));
        assert!(!section.body.contains("Cycle 199"));
    }

    #[test]
    fn extract_commitments_prefers_concrete_commitments_and_skips_previous_follow_through_quotes() {
        let section =
            extract_cycle_section(SAMPLE_JOURNAL, 200).expect("cycle section should exist");
        assert_eq!(
            extract_commitments(section.body),
            vec![
                "Review PRs from #850 and #851 when Copilot finishes".to_string(),
                "Dispatch #827 and #828 if concurrency allows".to_string(),
                "Fix cycle-receipts timing bug".to_string(),
                "I will verify the next merge carefully.".to_string(),
                "This should stay focused.".to_string(),
            ]
        );
    }

    #[test]
    fn find_deferred_escalations_flags_three_cycle_streaks() {
        let history = vec![
            state_schema::ReviewHistoryEntry {
                cycle: 197,
                categories: vec![
                    "cycle-close-drift".to_string(),
                    "disposition-accuracy".to_string(),
                ],
                actioned: 1,
                deferred: 1,
                ignored: 0,
                finding_count: 2,
                complacency_score: 5,
                note: Some("Deferred: cycle-close-drift.".to_string()),
                extra: Default::default(),
            },
            state_schema::ReviewHistoryEntry {
                cycle: 198,
                categories: vec![
                    "clean-cycle-accounting".to_string(),
                    "cycle-close-drift".to_string(),
                ],
                actioned: 1,
                deferred: 1,
                ignored: 0,
                finding_count: 2,
                complacency_score: 5,
                note: Some("Deferred 1 (cycle-close-drift).".to_string()),
                extra: Default::default(),
            },
            state_schema::ReviewHistoryEntry {
                cycle: 199,
                categories: vec![
                    "cycle-close-drift".to_string(),
                    "receipt-integrity".to_string(),
                ],
                actioned: 1,
                deferred: 1,
                ignored: 0,
                finding_count: 2,
                complacency_score: 4,
                note: Some("Deferred 1 (cycle-close-drift awaiting structural fix).".to_string()),
                extra: Default::default(),
            },
            state_schema::ReviewHistoryEntry {
                cycle: 200,
                categories: vec![
                    "cycle-close-drift".to_string(),
                    "receipt-integrity".to_string(),
                ],
                actioned: 2,
                deferred: 0,
                ignored: 0,
                finding_count: 2,
                complacency_score: 4,
                note: None,
                extra: Default::default(),
            },
        ];

        assert_eq!(
            find_deferred_escalations(&history),
            vec![Escalation {
                category: "cycle-close-drift".to_string(),
                consecutive_cycles: vec![197, 198, 199],
            }]
        );
    }
}
