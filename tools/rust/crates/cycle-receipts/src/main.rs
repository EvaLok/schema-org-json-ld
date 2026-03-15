use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
use state_schema::{current_cycle_from_state, StateJson};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const REPO_URL: &str = "https://github.com/EvaLok/schema-org-json-ld";
const FALLBACK_STEP: &str = "cycle-tagged";
const SPECIFIC_STATE_STEPS: [&str; 10] = [
    "cycle-start",
    "process-merge",
    "process-review",
    "process-audit",
    "process-eva",
    "cycle-complete",
    "record-dispatch",
    "commit-state-change",
    "state-fix",
    "review-format",
];

#[derive(Debug, Parser)]
#[command(name = "cycle-receipts")]
struct Cli {
    /// Cycle number to collect receipts for
    #[arg(long)]
    cycle: u64,

    /// Only include commits made strictly before this RFC3339 timestamp
    #[arg(long)]
    before: Option<String>,

    /// Output receipts as JSON
    #[arg(long)]
    json: bool,

    /// Path to the repository root
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct GitCommit {
    full_sha: String,
    short_sha: String,
    committed_at: DateTime<Utc>,
    subject: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct CycleWindow {
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
struct ReceiptEntry {
    step: String,
    receipt: String,
    commit: String,
    url: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    aliases: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct ReceiptMatch {
    full_sha: String,
    short_sha: String,
    step: String,
    commit: String,
    url: String,
}

fn main() {
    let cli = Cli::parse();
    match run(cli) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn run(cli: Cli) -> Result<String, String> {
    let before = cli
        .before
        .as_deref()
        .map(|value| parse_timestamp(value, "--before"))
        .transpose()?;
    let entries = collect_receipts(&cli.repo_root, cli.cycle, before)?;
    if cli.json {
        return serde_json::to_string_pretty(&entries)
            .map_err(|error| format!("failed to serialize JSON output: {}", error));
    }

    Ok(render_markdown(cli.cycle, &entries))
}

/// Collect receipt-bearing commits for the requested cycle, optionally capping
/// the window to commits strictly before `before`.
fn collect_receipts(
    repo_root: &Path,
    cycle: u64,
    before: Option<DateTime<Utc>>,
) -> Result<Vec<ReceiptEntry>, String> {
    let current_cycle = current_cycle_from_state(repo_root)?;
    let state = read_state_json(repo_root)?;
    let commits = read_git_commits(repo_root)?;
    let window = resolve_cycle_window(cycle, current_cycle, &state, &commits)?;

    let mut matching_commits: Vec<&GitCommit> = commits
        .iter()
        .filter(|commit| commit.committed_at >= window.start)
        .filter(|commit| window.end.is_none_or(|end| commit.committed_at < end))
        .filter(|commit| before.is_none_or(|timestamp| commit.committed_at < timestamp))
        .filter(|commit| matches_receipt_commit(&commit.subject, cycle))
        .collect();
    matching_commits.sort_by_key(|commit| commit.committed_at);

    let matches = matching_commits
        .into_iter()
        .flat_map(|commit| {
            extract_match_steps(&commit.subject, cycle)
                .into_iter()
                .map(|step| ReceiptMatch {
                    full_sha: commit.full_sha.clone(),
                    short_sha: commit.short_sha.clone(),
                    step,
                    commit: commit.subject.clone(),
                    url: format!("{}/commit/{}", REPO_URL, commit.full_sha),
                })
        })
        .collect::<Vec<_>>();

    Ok(deduplicate_receipts(matches))
}

fn matches_receipt_commit(subject: &str, cycle: u64) -> bool {
    !extract_match_steps(subject, cycle).is_empty()
}

fn extract_step(subject: &str) -> Option<String> {
    let prefix = "state(";
    if !subject.starts_with(prefix) {
        return None;
    }

    let remainder = subject.strip_prefix(prefix)?;
    let (step, suffix) = remainder.split_once("):")?;
    let step = step.trim();
    if suffix.trim().is_empty() || step.is_empty() {
        return None;
    }

    Some(step.to_string())
}

fn extract_match_steps(subject: &str, cycle: u64) -> Vec<String> {
    let mut steps = Vec::new();
    if let Some(step) = extract_step(subject) {
        steps.push(step);
    }
    if extract_cycle_tag(subject) == Some(cycle) {
        steps.push(FALLBACK_STEP.to_string());
    }
    steps
}

fn deduplicate_receipts(matches: Vec<ReceiptMatch>) -> Vec<ReceiptEntry> {
    let mut deduplicated = Vec::new();
    let mut positions = HashMap::new();

    for entry in matches {
        if let Some(position) = positions.get(&entry.full_sha).copied() {
            merge_receipt_match(&mut deduplicated[position], entry);
            continue;
        }

        positions.insert(entry.full_sha, deduplicated.len());
        deduplicated.push(ReceiptEntry {
            step: entry.step,
            receipt: entry.short_sha,
            commit: entry.commit,
            url: entry.url,
            aliases: Vec::new(),
        });
    }

    deduplicated
}

/// Merge an additional pattern match for the same commit SHA into the canonical
/// receipt entry, keeping the highest-priority step name and recording the
/// other matched labels as aliases.
fn merge_receipt_match(entry: &mut ReceiptEntry, incoming: ReceiptMatch) {
    if step_priority(&incoming.step) > step_priority(&entry.step) {
        push_unique(&mut entry.aliases, entry.step.clone());
        // Defensive cleanup for cases where the higher-priority step was already
        // recorded as an alias by an earlier lower-priority merge.
        entry.aliases.retain(|alias| alias != &incoming.step);
        entry.step = incoming.step;
        return;
    }

    if incoming.step != entry.step {
        push_unique(&mut entry.aliases, incoming.step);
    }
}

/// Return the deduplication priority for a step label.
///
/// Higher values are more specific: known workflow `state(...)` steps rank
/// above other custom `state(...)` labels, and the generic cycle-tagged match
/// ranks last.
///
/// Priority levels:
/// - 2: known workflow steps such as `process-merge`
/// - 1: custom `state(...)` labels such as `review-history`
/// - 0: generic `[cycle N]` matches rendered as `cycle-tagged`
fn step_priority(step: &str) -> u8 {
    if SPECIFIC_STATE_STEPS.contains(&step) {
        return 2;
    }
    if step == FALLBACK_STEP {
        return 0;
    }
    1
}

fn push_unique(values: &mut Vec<String>, value: String) {
    if !values.iter().any(|existing| existing == &value) {
        values.push(value);
    }
}

fn render_markdown(cycle: u64, entries: &[ReceiptEntry]) -> String {
    let mut output = format!("## Commit receipts — Cycle {}\n\n", cycle);
    let include_aliases = entries.iter().any(|entry| !entry.aliases.is_empty());
    if include_aliases {
        output.push_str("| Step | Receipt | Commit | Also |\n");
        output.push_str("|------|---------|--------|------|\n");
    } else {
        output.push_str("| Step | Receipt | Commit |\n");
        output.push_str("|------|---------|--------|\n");
    }
    for entry in entries {
        if include_aliases {
            output.push_str(&format!(
                "| {} | [`{}`]({}) | {} | {} |\n",
                escape_markdown_cell(&entry.step),
                entry.receipt,
                entry.url,
                escape_markdown_cell(&entry.commit),
                escape_markdown_cell(&entry.aliases.join(", "))
            ));
        } else {
            output.push_str(&format!(
                "| {} | [`{}`]({}) | {} |\n",
                escape_markdown_cell(&entry.step),
                entry.receipt,
                entry.url,
                escape_markdown_cell(&entry.commit)
            ));
        }
    }
    let receipt_label = if entries.len() == 1 {
        "receipt"
    } else {
        "receipts"
    };
    output.push_str(&format!(
        "\n{} {} collected.\n",
        entries.len(),
        receipt_label
    ));
    output
}

fn resolve_cycle_window(
    target_cycle: u64,
    current_cycle: u64,
    _state: &StateJson,
    commits: &[GitCommit],
) -> Result<CycleWindow, String> {
    let start = find_cycle_start_timestamp(commits, target_cycle).ok_or_else(|| {
        format!(
			"could not find cycle-start commit for cycle {}; fetch more history if this is a shallow clone",
			target_cycle
		)
    })?;
    let end = if target_cycle == current_cycle {
        None
    } else {
        find_cycle_start_timestamp(commits, target_cycle + 1)
    };
    Ok(CycleWindow { start, end })
}

fn read_state_json(repo_root: &Path) -> Result<StateJson, String> {
    let path = repo_root.join("docs/state.json");
    let content = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    serde_json::from_str::<StateJson>(&content)
        .map_err(|error| format!("failed to parse {}: {}", path.display(), error))
}

fn git_command(repo_root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute git {}: {}", args.join(" "), error))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("git {} failed: {}", args.join(" "), stderr));
    }

    String::from_utf8(output.stdout).map_err(|error| {
        format!(
            "failed to decode git {} output as UTF-8: {}",
            args.join(" "),
            error
        )
    })
}

fn read_git_commits(repo_root: &Path) -> Result<Vec<GitCommit>, String> {
    let output = git_command(
        repo_root,
        &[
            "log",
            "--date=iso-strict",
            "--pretty=format:%H%x09%cI%x09%s",
        ],
    )?;
    output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_git_commit_line)
        .collect()
}

fn parse_git_commit_line(line: &str) -> Result<GitCommit, String> {
    let mut parts = line.splitn(3, '\t');
    let full_sha = parts
        .next()
        .ok_or_else(|| format!("invalid git log line (missing sha): {}", line))?;
    let committed_at = parts
        .next()
        .ok_or_else(|| format!("invalid git log line (missing timestamp): {}", line))?;
    let subject = parts
        .next()
        .ok_or_else(|| format!("invalid git log line (missing subject): {}", line))?;
    if full_sha.len() < 7 {
        return Err(format!(
            "git sha must be at least 7 characters: {}",
            full_sha
        ));
    }

    Ok(GitCommit {
        full_sha: full_sha.to_string(),
        short_sha: full_sha[..7].to_string(),
        committed_at: parse_timestamp(committed_at, "git commit timestamp")?,
        subject: subject.to_string(),
    })
}

fn parse_timestamp(value: &str, label: &str) -> Result<DateTime<Utc>, String> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|error| format!("invalid {}: {}", label, error))
}

fn find_cycle_start_timestamp(commits: &[GitCommit], cycle: u64) -> Option<DateTime<Utc>> {
    commits.iter().find_map(|commit| {
        if extract_step(&commit.subject).as_deref() == Some("cycle-start")
            && extract_cycle_tag(&commit.subject) == Some(cycle)
        {
            return Some(commit.committed_at);
        }
        None
    })
}

fn extract_cycle_tag(subject: &str) -> Option<u64> {
    let marker = "[cycle ";
    let start = subject.find(marker)?;
    let remainder = &subject[start + marker.len()..];
    let end = remainder.find(']')?;
    remainder[..end].trim().parse::<u64>().ok()
}

fn escape_markdown_cell(value: &str) -> String {
    value.replace('|', "\\|")
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::env;
    use std::ffi::OsStr;
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn matches_known_state_prefixes() {
        for step in SPECIFIC_STATE_STEPS {
            let subject = format!("state({}): committed change", step);
            assert!(
                matches_receipt_commit(&subject, 198),
                "expected {subject} to match"
            );
            assert_eq!(extract_step(&subject), Some(step.to_string()));
        }
    }

    #[test]
    fn matches_cycle_tagged_commits_without_state_prefix() {
        assert!(matches_receipt_commit(
            "docs: update worklog [cycle 198]",
            198
        ));
        assert!(!matches_receipt_commit(
            "docs: update worklog [cycle 199]",
            198
        ));
        assert_eq!(extract_step("docs: update worklog [cycle 198]"), None);
    }

    #[test]
    fn render_markdown_includes_links_and_count() {
        let markdown = render_markdown(
            198,
            &[ReceiptEntry {
                step: "process-review".to_string(),
                receipt: "e4f5g6h".to_string(),
                commit: "state(process-review): consumed cycle 197 review".to_string(),
                url: format!("{}/commit/abcdef1234567890", REPO_URL),
                aliases: Vec::new(),
            }],
        );

        assert!(markdown.contains("## Commit receipts — Cycle 198"));
        assert!(markdown.contains("| process-review | [`e4f5g6h`](https://github.com/EvaLok/schema-org-json-ld/commit/abcdef1234567890) | state(process-review): consumed cycle 197 review |"));
        assert!(markdown.contains("1 receipt collected."));
    }

    #[test]
    fn render_markdown_includes_alias_column_when_present() {
        let markdown = render_markdown(
            198,
            &[ReceiptEntry {
                step: "process-merge".to_string(),
                receipt: "abc1234".to_string(),
                commit: "state(process-merge): merged PR #1 [cycle 198]".to_string(),
                url: format!("{}/commit/abcdef1234567890", REPO_URL),
                aliases: vec!["cycle-tagged".to_string()],
            }],
        );

        assert!(markdown.contains("| Step | Receipt | Commit | Also |"));
        assert!(markdown.contains("| process-merge | [`abc1234`](https://github.com/EvaLok/schema-org-json-ld/commit/abcdef1234567890) | state(process-merge): merged PR #1 [cycle 198] | cycle-tagged |"));
    }

    #[test]
    fn deduplication_prefers_specific_step_over_cycle_tagged() {
        let entries = deduplicate_receipts(vec![
            ReceiptMatch {
                full_sha: "abcdef1234567890".to_string(),
                short_sha: "abcdef1".to_string(),
                step: FALLBACK_STEP.to_string(),
                commit: "state(process-merge): merged PR #1 [cycle 198]".to_string(),
                url: format!("{}/commit/abcdef1234567890", REPO_URL),
            },
            ReceiptMatch {
                full_sha: "abcdef1234567890".to_string(),
                short_sha: "abcdef1".to_string(),
                step: "process-merge".to_string(),
                commit: "state(process-merge): merged PR #1 [cycle 198]".to_string(),
                url: format!("{}/commit/abcdef1234567890", REPO_URL),
            },
        ]);

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].step, "process-merge");
        assert_eq!(entries[0].aliases, vec![FALLBACK_STEP.to_string()]);
    }

    #[test]
    fn deduplication_prefers_custom_state_step_over_cycle_tagged() {
        let entries = deduplicate_receipts(vec![
            ReceiptMatch {
                full_sha: "abcdef1234567890".to_string(),
                short_sha: "abcdef1".to_string(),
                step: FALLBACK_STEP.to_string(),
                commit: "state(review-history): consumed review notes [cycle 198]".to_string(),
                url: format!("{}/commit/abcdef1234567890", REPO_URL),
            },
            ReceiptMatch {
                full_sha: "abcdef1234567890".to_string(),
                short_sha: "abcdef1".to_string(),
                step: "review-history".to_string(),
                commit: "state(review-history): consumed review notes [cycle 198]".to_string(),
                url: format!("{}/commit/abcdef1234567890", REPO_URL),
            },
        ]);

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].step, "review-history");
        assert_eq!(entries[0].aliases, vec![FALLBACK_STEP.to_string()]);
    }

    #[test]
    fn deduplication_preserves_unique_commits() {
        let entries = deduplicate_receipts(vec![
            ReceiptMatch {
                full_sha: "abcdef1234567890".to_string(),
                short_sha: "abcdef1".to_string(),
                step: "process-review".to_string(),
                commit: "state(process-review): reviewed change".to_string(),
                url: format!("{}/commit/abcdef1234567890", REPO_URL),
            },
            ReceiptMatch {
                full_sha: "fedcba0987654321".to_string(),
                short_sha: "fedcba0".to_string(),
                step: "cycle-tagged".to_string(),
                commit: "docs: update worklog [cycle 198]".to_string(),
                url: format!("{}/commit/fedcba0987654321", REPO_URL),
            },
        ]);

        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].receipt, "abcdef1");
        assert_eq!(entries[1].receipt, "fedcba0");
    }

    #[test]
    fn deduplication_handles_synthetic_triple_match() {
        // This covers the deduplication priority algorithm directly. The parser
        // cannot produce three labels from a single commit subject.
        let entries = deduplicate_receipts(vec![
            ReceiptMatch {
                full_sha: "abcdef1234567890".to_string(),
                short_sha: "abcdef1".to_string(),
                step: "cycle-tagged".to_string(),
                commit: "state(process-review): reviewed change [cycle 198]".to_string(),
                url: format!("{}/commit/abcdef1234567890", REPO_URL),
            },
            ReceiptMatch {
                full_sha: "abcdef1234567890".to_string(),
                short_sha: "abcdef1".to_string(),
                step: "review-history".to_string(),
                commit: "state(process-review): reviewed change [cycle 198]".to_string(),
                url: format!("{}/commit/abcdef1234567890", REPO_URL),
            },
            ReceiptMatch {
                full_sha: "abcdef1234567890".to_string(),
                short_sha: "abcdef1".to_string(),
                step: "process-review".to_string(),
                commit: "state(process-review): reviewed change [cycle 198]".to_string(),
                url: format!("{}/commit/abcdef1234567890", REPO_URL),
            },
        ]);

        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].step, "process-review");
        assert_eq!(
            entries[0].aliases,
            vec!["cycle-tagged".to_string(), "review-history".to_string()]
        );
    }

    #[test]
    fn json_output_includes_aliases() {
        let entries = vec![ReceiptEntry {
            step: "process-merge".to_string(),
            receipt: "abcdef1".to_string(),
            commit: "state(process-merge): merged PR #1 [cycle 198]".to_string(),
            url: format!("{}/commit/abcdef1234567890", REPO_URL),
            aliases: vec!["cycle-tagged".to_string()],
        }];

        let output = serde_json::to_value(&entries).expect("entries should serialize");

        assert_eq!(output[0]["step"], "process-merge");
        assert_eq!(output[0]["aliases"], json!(["cycle-tagged"]));
    }

    #[test]
    fn collect_receipts_uses_cycle_start_commit_for_current_cycle() {
        let repo = TempRepo::new();
        repo.init_git();
        repo.write_state(&json!({
            "last_cycle": {
                "number": 198,
                "timestamp": "2026-03-09T01:00:00Z"
            }
        }));
        repo.commit_file_at(
            "notes.txt",
            "older\n",
            "state(process-review): old receipt",
            "2026-03-09T00:30:00Z",
        );
        repo.commit_file_at(
            "notes.txt",
            "start\n",
            "state(cycle-start): begin cycle 198, issue #1 [cycle 198]",
            "2026-03-09T00:45:00Z",
        );
        repo.commit_file_at(
            "notes.txt",
            "newer\n",
            "state(process-review): current receipt",
            "2026-03-09T00:50:00Z",
        );
        repo.commit_file_at(
            "notes.txt",
            "tagged\n",
            "docs: worklog touch [cycle 198]",
            "2026-03-09T01:20:00Z",
        );

        let receipts = collect_receipts(repo.path(), 198, None).expect("receipts should collect");
        let subjects: Vec<&str> = receipts
            .iter()
            .map(|receipt| receipt.commit.as_str())
            .collect();
        assert_eq!(
            subjects,
            vec![
                "state(cycle-start): begin cycle 198, issue #1 [cycle 198]",
                "state(process-review): current receipt",
                "docs: worklog touch [cycle 198]",
            ]
        );
    }

    #[test]
    fn collect_receipts_ignores_mutated_state_timestamp_for_current_cycle() {
        let repo = TempRepo::new();
        repo.init_git();
        repo.write_state(&json!({
            "last_cycle": {
                "number": 198,
                "timestamp": "2026-03-09T01:30:00Z"
            }
        }));
        repo.commit_file_at(
            "notes.txt",
            "start\n",
            "state(cycle-start): begin cycle 198, issue #1 [cycle 198]",
            "2026-03-09T01:00:00Z",
        );
        repo.commit_file_at(
            "notes.txt",
            "review\n",
            "state(process-review): current receipt",
            "2026-03-09T01:10:00Z",
        );
        repo.commit_file_at(
            "notes.txt",
            "tagged\n",
            "docs: worklog touch [cycle 198]",
            "2026-03-09T01:20:00Z",
        );

        let receipts = collect_receipts(repo.path(), 198, None).expect("receipts should collect");
        let subjects: Vec<&str> = receipts
            .iter()
            .map(|receipt| receipt.commit.as_str())
            .collect();
        assert_eq!(
            subjects,
            vec![
                "state(cycle-start): begin cycle 198, issue #1 [cycle 198]",
                "state(process-review): current receipt",
                "docs: worklog touch [cycle 198]",
            ]
        );
    }

    #[test]
    fn collect_receipts_deduplicates_real_dual_match_commit() {
        let repo = TempRepo::new();
        repo.init_git();
        repo.write_state(&json!({
            "last_cycle": {
                "number": 198,
                "timestamp": "2026-03-09T01:30:00Z"
            }
        }));
        repo.commit_file_at(
            "notes.txt",
            "start\n",
            "state(cycle-start): begin cycle 198, issue #1 [cycle 198]",
            "2026-03-09T01:00:00Z",
        );
        repo.commit_file_at(
            "notes.txt",
            "merge\n",
            "state(process-merge): merged PR #1 [cycle 198]",
            "2026-03-09T01:10:00Z",
        );

        let receipts = collect_receipts(repo.path(), 198, None).expect("receipts should collect");

        assert_eq!(receipts.len(), 2);
        assert_eq!(receipts[0].step, "cycle-start");
        assert_eq!(receipts[0].aliases, vec![FALLBACK_STEP.to_string()]);
        assert_eq!(receipts[1].step, "process-merge");
        assert_eq!(receipts[1].commit, "state(process-merge): merged PR #1 [cycle 198]");
        assert_eq!(receipts[1].aliases, vec![FALLBACK_STEP.to_string()]);
    }

    #[test]
    fn collect_receipts_for_historical_cycle_stops_at_next_cycle_start() {
        let repo = TempRepo::new();
        repo.init_git();
        repo.write_state(&json!({
            "last_cycle": {
                "number": 199,
                "timestamp": "2026-03-09T02:00:00Z"
            }
        }));
        repo.commit_file_at(
            "history.txt",
            "cycle 198 start\n",
            "state(cycle-start): begin cycle 198, issue #1 [cycle 198]",
            "2026-03-09T01:00:00Z",
        );
        repo.commit_file_at(
            "history.txt",
            "review\n",
            "state(process-review): consumed cycle 197 review",
            "2026-03-09T01:10:00Z",
        );
        repo.commit_file_at(
            "history.txt",
            "tagged\n",
            "docs: worklog touch [cycle 198]",
            "2026-03-09T01:20:00Z",
        );
        repo.commit_file_at(
            "history.txt",
            "cycle 199 start\n",
            "state(cycle-start): begin cycle 199, issue #2 [cycle 199]",
            "2026-03-09T02:00:00Z",
        );
        repo.commit_file_at(
            "history.txt",
            "post-cycle\n",
            "state(process-review): consumed cycle 198 review",
            "2026-03-09T02:10:00Z",
        );

        let receipts = collect_receipts(repo.path(), 198, None).expect("receipts should collect");
        let subjects: Vec<&str> = receipts
            .iter()
            .map(|receipt| receipt.commit.as_str())
            .collect();
        assert_eq!(
            subjects,
            vec![
                "state(cycle-start): begin cycle 198, issue #1 [cycle 198]",
                "state(process-review): consumed cycle 197 review",
                "docs: worklog touch [cycle 198]",
            ]
        );
    }

    #[test]
    fn collect_receipts_before_caps_cycle_window() {
        let repo = TempRepo::new();
        repo.init_git();
        repo.write_state(&json!({
            "last_cycle": {
                "number": 198,
                "timestamp": "2026-03-09T01:00:00Z"
            }
        }));
        repo.commit_file_at(
            "notes.txt",
            "start\n",
            "state(cycle-start): begin cycle 198, issue #1 [cycle 198]",
            "2026-03-09T01:00:00Z",
        );
        repo.commit_file_at(
            "notes.txt",
            "older\n",
            "state(process-review): current receipt",
            "2026-03-09T01:10:00Z",
        );
        repo.commit_file_at(
            "notes.txt",
            "tagged\n",
            "docs: worklog touch [cycle 198]",
            "2026-03-09T01:20:00Z",
        );
        repo.commit_file_at(
            "notes.txt",
            "later\n",
            "state(cycle-complete): cycle 198 close out [cycle 198]",
            "2026-03-09T01:30:00Z",
        );

        let before = parse_timestamp("2026-03-09T01:20:00Z", "test timestamp")
            .expect("timestamp should parse");
        let receipts =
            collect_receipts(repo.path(), 198, Some(before)).expect("receipts should collect");
        let subjects: Vec<&str> = receipts
            .iter()
            .map(|receipt| receipt.commit.as_str())
            .collect();
        assert_eq!(
            subjects,
            vec![
                "state(cycle-start): begin cycle 198, issue #1 [cycle 198]",
                "state(process-review): current receipt",
            ]
        );
    }

    struct TempRepo {
        path: PathBuf,
    }

    impl TempRepo {
        fn new() -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos();
            let path = env::temp_dir().join(format!(
                "cycle-receipts-test-{}-{}",
                std::process::id(),
                unique
            ));
            fs::create_dir_all(path.join("docs")).expect("temp repo should be created");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn write_state(&self, state: &serde_json::Value) {
            let state_path = self.path.join("docs/state.json");
            let serialized = serde_json::to_string_pretty(state).expect("state should serialize");
            fs::write(state_path, format!("{}\n", serialized)).expect("state should be written");
        }

        fn init_git(&self) {
            assert_git_success(self.path(), ["init"]);
            assert_git_success(self.path(), ["config", "user.name", "Cycle Receipt Tests"]);
            assert_git_success(
                self.path(),
                ["config", "user.email", "cycle-receipts-tests@example.com"],
            );
        }

        fn commit_file_at(
            &self,
            relative_path: &str,
            content: &str,
            message: &str,
            timestamp: &str,
        ) {
            let file_path = self.path.join(relative_path);
            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).expect("file parent should exist");
            }
            fs::write(&file_path, content).expect("file should be written");

            let add_output = Command::new("git")
                .arg("-C")
                .arg(self.path())
                .arg("add")
                .arg(relative_path)
                .output()
                .expect("git add should execute");
            assert!(
                add_output.status.success(),
                "git add failed: {}",
                String::from_utf8_lossy(&add_output.stderr)
            );

            let commit_output = Command::new("git")
                .arg("-C")
                .arg(self.path())
                .arg("commit")
                .arg("-m")
                .arg(message)
                .env("GIT_AUTHOR_DATE", timestamp)
                .env("GIT_COMMITTER_DATE", timestamp)
                .output()
                .expect("git commit should execute");
            assert!(
                commit_output.status.success(),
                "git commit failed: {}",
                String::from_utf8_lossy(&commit_output.stderr)
            );
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn assert_git_success<I, S>(repo_root: &Path, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let output = Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .args(args)
            .output()
            .expect("git command should execute");
        assert!(
            output.status.success(),
            "git command failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}
