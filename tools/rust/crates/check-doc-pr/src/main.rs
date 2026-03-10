use clap::Parser;
use serde::{Deserialize, Serialize};
use state_schema::read_state_value;
use std::path::{Path, PathBuf};
use std::process::Command;

const REPO: &str = "EvaLok/schema-org-json-ld";

const INFRA_PATHS: &[&str] = &[
    "tools/",
    "STARTUP_CHECKLIST.md",
    "COMPLETION_CHECKLIST.md",
    "AGENTS.md",
    ".claude/skills/",
];

#[derive(Debug, Parser)]
#[command(name = "check-doc-pr")]
struct Cli {
    /// Pull request number
    #[arg(long)]
    pr: u64,

    /// Cycle number
    #[arg(long)]
    cycle: u64,

    /// Path to the repository root
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckResult {
    pub check: String,
    pub status: CheckStatus,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CheckStatus {
    Pass,
    Fail,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckReport {
    pub overall: CheckStatus,
    pub results: Vec<CheckResult>,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(2);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    validate_repo_root(&cli.repo_root)?;

    let pr_files = fetch_pr_files(&cli.repo_root, cli.pr)?;
    let pr_branch = fetch_pr_branch(&cli.repo_root, cli.pr)?;

    let mut results = Vec::new();

    // 1. worklog_exists
    let worklog_file = find_worklog_file(&pr_files, cli.cycle);
    results.push(check_worklog_exists(&worklog_file));

    // 2. journal_exists
    let journal_file = find_journal_file(&pr_files);
    results.push(check_journal_exists(&journal_file));

    // 3. worklog_sections (needs worklog content)
    let worklog_content = match &worklog_file {
        Some(path) => fetch_file_content(&cli.repo_root, path, &pr_branch).ok(),
        None => None,
    };
    results.push(check_worklog_sections(worklog_content.as_deref()));

    // 4. in_flight_matches
    results.push(check_in_flight_matches(
        &cli.repo_root,
        worklog_content.as_deref(),
    ));

    // 5. self_modifications_accurate
    results.push(check_self_modifications_accurate(
        &cli.repo_root,
        cli.cycle,
        worklog_content.as_deref(),
    ));

    // 6. receipts_valid
    results.push(check_receipts_valid(
        &cli.repo_root,
        worklog_content.as_deref(),
    ));

    // 7. journal_has_worklog_link
    let journal_content = match &journal_file {
        Some(path) => fetch_file_content(&cli.repo_root, path, &pr_branch).ok(),
        None => None,
    };
    results.push(check_journal_has_worklog_link(journal_content.as_deref()));

    // 8. no_duplicate_headers
    results.push(check_no_duplicate_headers(journal_content.as_deref()));

    let overall = if results.iter().all(|r| r.status == CheckStatus::Pass) {
        CheckStatus::Pass
    } else {
        CheckStatus::Fail
    };

    let report = CheckReport { overall, results };
    let json = serde_json::to_string_pretty(&report)
        .map_err(|error| format!("failed to serialize report: {}", error))?;
    println!("{}", json);

    if report.overall == CheckStatus::Fail {
        std::process::exit(1);
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// PR data fetching
// ---------------------------------------------------------------------------

fn fetch_pr_files(repo_root: &Path, pr: u64) -> Result<Vec<String>, String> {
    let output = Command::new("gh")
        .current_dir(repo_root)
        .args([
            "pr",
            "diff",
            &pr.to_string(),
            "--repo",
            REPO,
            "--name-only",
        ])
        .output()
        .map_err(|error| format!("failed to run gh pr diff: {}", error))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("gh pr diff failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect())
}

fn fetch_pr_branch(repo_root: &Path, pr: u64) -> Result<String, String> {
    let output = Command::new("gh")
        .current_dir(repo_root)
        .args([
            "pr",
            "view",
            &pr.to_string(),
            "--repo",
            REPO,
            "--json",
            "headRefName",
            "--jq",
            ".headRefName",
        ])
        .output()
        .map_err(|error| format!("failed to run gh pr view: {}", error))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("gh pr view failed: {}", stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn fetch_file_content(repo_root: &Path, path: &str, branch: &str) -> Result<String, String> {
    let endpoint = format!("repos/{}/contents/{}?ref={}", REPO, path, branch);
    let output = Command::new("gh")
        .current_dir(repo_root)
        .args(["api", &endpoint, "--jq", ".content"])
        .output()
        .map_err(|error| format!("failed to run gh api for {}: {}", path, error))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("gh api contents failed for {}: {}", path, stderr));
    }

    let encoded = String::from_utf8_lossy(&output.stdout).trim().to_string();
    decode_base64(&encoded)
}

fn decode_base64(input: &str) -> Result<String, String> {
    // GitHub returns base64 with newlines; strip them before decoding.
    let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    let bytes = base64_decode(&cleaned)?;
    String::from_utf8(bytes).map_err(|error| format!("base64 content is not valid UTF-8: {}", error))
}

/// Minimal base64 decoder (standard alphabet, no padding required).
fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    fn val(c: u8) -> Result<u8, String> {
        match c {
            b'A'..=b'Z' => Ok(c - b'A'),
            b'a'..=b'z' => Ok(c - b'a' + 26),
            b'0'..=b'9' => Ok(c - b'0' + 52),
            b'+' => Ok(62),
            b'/' => Ok(63),
            b'=' => Ok(0),
            _ => Err(format!("invalid base64 character: {}", c as char)),
        }
    }

    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() * 3 / 4);
    let chunks = bytes.chunks(4);
    for chunk in chunks {
        if chunk.len() < 2 {
            break;
        }
        let a = val(chunk[0])?;
        let b = val(chunk[1])?;
        out.push((a << 2) | (b >> 4));
        if chunk.len() > 2 && chunk[2] != b'=' {
            let c = val(chunk[2])?;
            out.push((b << 4) | (c >> 2));
            if chunk.len() > 3 && chunk[3] != b'=' {
                let d = val(chunk[3])?;
                out.push((c << 6) | d);
            }
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// File pattern matching
// ---------------------------------------------------------------------------

fn find_worklog_file(files: &[String], cycle: u64) -> Option<String> {
    let cycle_pattern = format!("cycle-{}", cycle);
    files
        .iter()
        .find(|f| f.starts_with("docs/worklog/") && f.to_lowercase().contains(&cycle_pattern))
        .cloned()
}

fn find_journal_file(files: &[String]) -> Option<String> {
    files
        .iter()
        .find(|f| f.starts_with("docs/journal/") && f.ends_with(".md"))
        .cloned()
}

// ---------------------------------------------------------------------------
// Individual checks
// ---------------------------------------------------------------------------

fn check_worklog_exists(worklog_file: &Option<String>) -> CheckResult {
    match worklog_file {
        Some(path) => CheckResult {
            check: "worklog_exists".to_string(),
            status: CheckStatus::Pass,
            detail: format!("Found {}", path),
        },
        None => CheckResult {
            check: "worklog_exists".to_string(),
            status: CheckStatus::Fail,
            detail: "No worklog file matching docs/worklog/{{date}}/*cycle-N* found in PR"
                .to_string(),
        },
    }
}

fn check_journal_exists(journal_file: &Option<String>) -> CheckResult {
    match journal_file {
        Some(path) => CheckResult {
            check: "journal_exists".to_string(),
            status: CheckStatus::Pass,
            detail: format!("Found {}", path),
        },
        None => CheckResult {
            check: "journal_exists".to_string(),
            status: CheckStatus::Fail,
            detail: "No journal file matching docs/journal/{{date}}.md found in PR".to_string(),
        },
    }
}

pub fn has_required_sections(content: &str) -> Vec<&'static str> {
    let lower = content.to_lowercase();

    let section_groups: &[(&str, &[&str])] = &[
        (
            "What was done",
            &["what was done", "## done", "## summary"],
        ),
        ("Self-modifications", &["self-modifications", "self-modification"]),
        ("Current state", &["current state", "## state"]),
        ("Next steps", &["next steps", "## next"]),
        (
            "Commit receipts",
            &["commit receipts", "## receipts"],
        ),
    ];

    let mut missing = Vec::new();
    for (name, patterns) in section_groups {
        if !patterns.iter().any(|p| lower.contains(p)) {
            missing.push(*name);
        }
    }
    missing
}

fn check_worklog_sections(content: Option<&str>) -> CheckResult {
    let Some(content) = content else {
        return CheckResult {
            check: "worklog_sections".to_string(),
            status: CheckStatus::Fail,
            detail: "Cannot check sections: worklog content not available".to_string(),
        };
    };

    let missing = has_required_sections(content);
    if missing.is_empty() {
        CheckResult {
            check: "worklog_sections".to_string(),
            status: CheckStatus::Pass,
            detail: "All required sections present".to_string(),
        }
    } else {
        CheckResult {
            check: "worklog_sections".to_string(),
            status: CheckStatus::Fail,
            detail: format!("Missing sections: {}", missing.join(", ")),
        }
    }
}

fn check_in_flight_matches(repo_root: &Path, worklog_content: Option<&str>) -> CheckResult {
    let check_name = "in_flight_matches".to_string();

    let state_in_flight = match read_in_flight_from_state(repo_root) {
        Ok(value) => value,
        Err(error) => {
            return CheckResult {
                check: check_name,
                status: CheckStatus::Fail,
                detail: format!("Cannot read state.json: {}", error),
            };
        }
    };

    let Some(content) = worklog_content else {
        return CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Cannot check in-flight: worklog content not available".to_string(),
        };
    };

    match extract_in_flight_from_worklog(content) {
        Some(worklog_value) => {
            if worklog_value == state_in_flight {
                CheckResult {
                    check: check_name,
                    status: CheckStatus::Pass,
                    detail: format!("In-flight count matches: {}", state_in_flight),
                }
            } else {
                CheckResult {
                    check: check_name,
                    status: CheckStatus::Fail,
                    detail: format!(
                        "Worklog says {}, state.json says {}",
                        worklog_value, state_in_flight
                    ),
                }
            }
        }
        None => CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Could not find in-flight count in worklog".to_string(),
        },
    }
}

fn read_in_flight_from_state(repo_root: &Path) -> Result<i64, String> {
    let state = read_state_value(repo_root)?;
    state
        .pointer("/copilot_metrics/in_flight")
        .and_then(|v| v.as_i64())
        .ok_or_else(|| "missing /copilot_metrics/in_flight in state.json".to_string())
}

pub fn extract_in_flight_from_worklog(content: &str) -> Option<i64> {
    // Search for patterns like "in-flight: 3", "in_flight: 3", "In flight: 3"
    let lower = content.to_lowercase();
    for line in lower.lines() {
        // Look for "in.flight" (any separator) followed by a number
        if let Some(pos) = line.find("in") {
            let rest = &line[pos + 2..];
            // Check for separator characters between "in" and "flight"
            let rest_trimmed = rest.trim_start_matches(|c: char| c == '-' || c == '_' || c == ' ');
            if rest_trimmed.starts_with("flight") {
                let after_flight = &rest_trimmed[6..];
                // Find the first digit sequence after "flight"
                if let Some(num) = extract_first_number(after_flight) {
                    return Some(num);
                }
            }
        }
    }
    None
}

fn extract_first_number(s: &str) -> Option<i64> {
    let mut start = None;
    for (i, c) in s.char_indices() {
        if c.is_ascii_digit() {
            if start.is_none() {
                start = Some(i);
            }
        } else if start.is_some() {
            return s[start.unwrap()..i].parse().ok();
        }
    }
    if let Some(start_idx) = start {
        return s[start_idx..].parse().ok();
    }
    None
}

fn check_self_modifications_accurate(
    repo_root: &Path,
    cycle: u64,
    worklog_content: Option<&str>,
) -> CheckResult {
    let check_name = "self_modifications_accurate".to_string();

    let Some(content) = worklog_content else {
        return CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Cannot check: worklog content not available".to_string(),
        };
    };

    let infra_changed = match detect_infrastructure_changes(repo_root, cycle) {
        Ok(changed) => changed,
        Err(error) => {
            return CheckResult {
                check: check_name,
                status: CheckStatus::Fail,
                detail: format!("Cannot detect infrastructure changes: {}", error),
            };
        }
    };

    if !infra_changed {
        return CheckResult {
            check: check_name,
            status: CheckStatus::Pass,
            detail: "No infrastructure changes detected in cycle commits".to_string(),
        };
    }

    // Infrastructure changed; check that "Self-modifications" section doesn't just say "None"
    let self_mod_content = extract_section_content(content, "self-modification");
    match self_mod_content {
        Some(section) => {
            let trimmed = section.trim().to_lowercase();
            if trimmed == "none" || trimmed == "n/a" || trimmed == "none." || trimmed == "n/a." {
                CheckResult {
                    check: check_name,
                    status: CheckStatus::Fail,
                    detail: "Infrastructure files changed but Self-modifications says None"
                        .to_string(),
                }
            } else {
                CheckResult {
                    check: check_name,
                    status: CheckStatus::Pass,
                    detail: "Self-modifications section documents infrastructure changes"
                        .to_string(),
                }
            }
        }
        None => CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Infrastructure files changed but no Self-modifications section found"
                .to_string(),
        },
    }
}

fn detect_infrastructure_changes(repo_root: &Path, cycle: u64) -> Result<bool, String> {
    // Find commits tagged with this cycle in their message
    let cycle_tag = format!("cycle {}", cycle);
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(["log", "--format=%H", "--all", "--grep", &cycle_tag])
        .output()
        .map_err(|error| format!("failed to run git log: {}", error))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("git log failed: {}", stderr));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let commits: Vec<&str> = stdout.lines().filter(|l| !l.is_empty()).collect();

    if commits.is_empty() {
        return Ok(false);
    }

    for commit in commits {
        let diff_output = Command::new("git")
            .current_dir(repo_root)
            .args(["diff-tree", "--no-commit-id", "-r", "--name-only", commit])
            .output()
            .map_err(|error| format!("failed to run git diff-tree: {}", error))?;

        if !diff_output.status.success() {
            continue;
        }

        let files = String::from_utf8_lossy(&diff_output.stdout);
        for file in files.lines() {
            if INFRA_PATHS
                .iter()
                .any(|prefix| file.starts_with(prefix) || file == prefix.trim_end_matches('/'))
            {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn extract_section_content(content: &str, section_name_lower: &str) -> Option<String> {
    let mut in_section = false;
    let mut section_lines = Vec::new();

    for line in content.lines() {
        if line.starts_with('#') {
            if in_section {
                break;
            }
            if line.to_lowercase().contains(section_name_lower) {
                in_section = true;
                continue;
            }
        } else if in_section {
            section_lines.push(line);
        }
    }

    if in_section {
        Some(section_lines.join("\n"))
    } else {
        None
    }
}

pub fn extract_receipt_hashes(content: &str) -> Vec<String> {
    let mut hashes = Vec::new();
    let lower = content.to_lowercase();

    for line in lower.lines() {
        if line.contains("receipt") || line.contains("commit") || line.contains("sha") {
            // Extract 7+ char hex strings from this line
            for hash in extract_hex_strings(line, 7) {
                hashes.push(hash);
            }
        }
    }

    // Also scan for standalone 7-char hex strings in a "receipts" section
    if let Some(section) = extract_section_content(content, "receipt") {
        for line in section.lines() {
            for hash in extract_hex_strings(line, 7) {
                if !hashes.contains(&hash) {
                    hashes.push(hash);
                }
            }
        }
    }

    hashes
}

fn extract_hex_strings(line: &str, min_len: usize) -> Vec<String> {
    let mut results = Vec::new();
    let mut current = String::new();

    for c in line.chars() {
        if c.is_ascii_hexdigit() {
            current.push(c);
        } else {
            if current.len() >= min_len && current.len() <= 40 {
                // Skip if it's all digits (likely a number, not a hash)
                if !current.chars().all(|ch| ch.is_ascii_digit()) {
                    results.push(current.clone());
                }
            }
            current.clear();
        }
    }
    // Check trailing
    if current.len() >= min_len && current.len() <= 40 {
        if !current.chars().all(|ch| ch.is_ascii_digit()) {
            results.push(current);
        }
    }

    results
}

fn check_receipts_valid(repo_root: &Path, worklog_content: Option<&str>) -> CheckResult {
    let check_name = "receipts_valid".to_string();

    let Some(content) = worklog_content else {
        return CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Cannot check: worklog content not available".to_string(),
        };
    };

    let hashes = extract_receipt_hashes(content);
    if hashes.is_empty() {
        return CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "No receipt hashes found in worklog".to_string(),
        };
    }

    let mut invalid = Vec::new();
    for hash in &hashes {
        if !verify_commit_hash(repo_root, hash) {
            invalid.push(hash.clone());
        }
    }

    if invalid.is_empty() {
        CheckResult {
            check: check_name,
            status: CheckStatus::Pass,
            detail: format!(
                "All {} receipt hashes verified: {}",
                hashes.len(),
                hashes.join(", ")
            ),
        }
    } else {
        CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: format!("Invalid receipt hashes: {}", invalid.join(", ")),
        }
    }
}

fn verify_commit_hash(repo_root: &Path, hash: &str) -> bool {
    let ref_spec = format!("{}^{{commit}}", hash);
    let output = Command::new("git")
        .current_dir(repo_root)
        .args(["rev-parse", "--verify", &ref_spec])
        .output();

    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

fn check_journal_has_worklog_link(journal_content: Option<&str>) -> CheckResult {
    let check_name = "journal_has_worklog_link".to_string();

    let Some(content) = journal_content else {
        return CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Cannot check: journal content not available".to_string(),
        };
    };

    let lower = content.to_lowercase();
    if lower.contains("worklog") {
        CheckResult {
            check: check_name,
            status: CheckStatus::Pass,
            detail: "Journal contains worklog reference".to_string(),
        }
    } else {
        CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Journal does not contain a worklog link or reference".to_string(),
        }
    }
}

pub fn find_duplicate_headers(content: &str) -> Vec<String> {
    let mut seen = Vec::new();
    let mut duplicates = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            let header = trimmed.to_lowercase();
            if seen.contains(&header) {
                if !duplicates.contains(&header) {
                    duplicates.push(header);
                }
            } else {
                seen.push(header);
            }
        }
    }

    duplicates
}

fn check_no_duplicate_headers(journal_content: Option<&str>) -> CheckResult {
    let check_name = "no_duplicate_headers".to_string();

    let Some(content) = journal_content else {
        return CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Cannot check: journal content not available".to_string(),
        };
    };

    let duplicates = find_duplicate_headers(content);
    if duplicates.is_empty() {
        CheckResult {
            check: check_name,
            status: CheckStatus::Pass,
            detail: "No duplicate section headers found".to_string(),
        }
    } else {
        CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: format!("Duplicate headers: {}", duplicates.join(", ")),
        }
    }
}

fn validate_repo_root(repo_root: &Path) -> Result<(), String> {
    if !repo_root.exists() {
        return Err(format!(
            "--repo-root must point to an existing path, got {}",
            repo_root.display()
        ));
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn help_contains_expected_flags() {
        let result = Cli::try_parse_from(["check-doc-pr", "--help"]);
        // clap returns Err on --help (it's a special exit), so check the error message
        let error = result.unwrap_err();
        let help_text = error.to_string();
        assert!(help_text.contains("--pr"), "help should mention --pr flag");
        assert!(
            help_text.contains("--cycle"),
            "help should mention --cycle flag"
        );
        assert!(
            help_text.contains("--repo-root"),
            "help should mention --repo-root flag"
        );
    }

    #[test]
    fn check_result_serializes_to_expected_json() {
        let report = CheckReport {
            overall: CheckStatus::Fail,
            results: vec![
                CheckResult {
                    check: "worklog_exists".to_string(),
                    status: CheckStatus::Pass,
                    detail: "Found docs/worklog/2026-03-10/cycle-218.md".to_string(),
                },
                CheckResult {
                    check: "in_flight_matches".to_string(),
                    status: CheckStatus::Fail,
                    detail: "Worklog says 3, state.json says 2".to_string(),
                },
            ],
        };

        let serialized = serde_json::to_value(&report).expect("report should serialize");

        assert_eq!(serialized["overall"], json!("fail"));
        assert_eq!(serialized["results"][0]["check"], json!("worklog_exists"));
        assert_eq!(serialized["results"][0]["status"], json!("pass"));
        assert_eq!(
            serialized["results"][0]["detail"],
            json!("Found docs/worklog/2026-03-10/cycle-218.md")
        );
        assert_eq!(
            serialized["results"][1]["check"],
            json!("in_flight_matches")
        );
        assert_eq!(serialized["results"][1]["status"], json!("fail"));
        assert_eq!(
            serialized["results"][1]["detail"],
            json!("Worklog says 3, state.json says 2")
        );
    }

    #[test]
    fn detect_missing_worklog_sections() {
        let content = "## What was done\nSome work\n## Next steps\nDo more";
        let missing = has_required_sections(content);
        assert!(
            missing.contains(&"Self-modifications"),
            "should detect missing Self-modifications"
        );
        assert!(
            missing.contains(&"Current state"),
            "should detect missing Current state"
        );
        assert!(
            missing.contains(&"Commit receipts"),
            "should detect missing Commit receipts"
        );
        assert!(
            !missing.contains(&"What was done"),
            "should not flag present section"
        );
        assert!(
            !missing.contains(&"Next steps"),
            "should not flag present section"
        );
    }

    #[test]
    fn detect_missing_worklog_sections_with_alternate_names() {
        let content =
            "## Summary\nWork\n## Self-modifications\nNone\n## State\nOk\n## Next\nMore\n## Receipts\nabc1234";
        let missing = has_required_sections(content);
        assert!(
            missing.is_empty(),
            "all alternate section names should be recognized, but missing: {:?}",
            missing
        );
    }

    #[test]
    fn detect_duplicate_headers() {
        let content = "## Overview\nSome text\n## Cycle 218\nDetails\n## Overview\nDuplicate";
        let duplicates = find_duplicate_headers(content);
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0], "## overview");
    }

    #[test]
    fn no_duplicates_when_headers_are_unique() {
        let content = "## Overview\nSome text\n## Details\nMore text\n## Summary\nEnd";
        let duplicates = find_duplicate_headers(content);
        assert!(
            duplicates.is_empty(),
            "unique headers should produce no duplicates"
        );
    }

    #[test]
    fn receipt_hash_pattern_extraction() {
        let content = r#"
## Commit receipts
- state commit receipt: abc1234
- docs commit receipt: def5678
- merge receipt: 9a8b7c6
"#;
        let hashes = extract_receipt_hashes(content);
        assert!(
            hashes.contains(&"abc1234".to_string()),
            "should extract abc1234, got {:?}",
            hashes
        );
        assert!(
            hashes.contains(&"def5678".to_string()),
            "should extract def5678, got {:?}",
            hashes
        );
        assert!(
            hashes.contains(&"9a8b7c6".to_string()),
            "should extract 9a8b7c6, got {:?}",
            hashes
        );
    }

    #[test]
    fn receipt_extraction_ignores_pure_numeric_strings() {
        let content = "## Receipts\n- commit receipt: abc1234\n- count: 1234567\n";
        let hashes = extract_receipt_hashes(content);
        assert!(
            hashes.contains(&"abc1234".to_string()),
            "should extract hex hash"
        );
        assert!(
            !hashes.contains(&"1234567".to_string()),
            "should not extract pure numeric string"
        );
    }

    #[test]
    fn in_flight_extraction_from_worklog() {
        assert_eq!(
            extract_in_flight_from_worklog("In-flight: 3"),
            Some(3)
        );
        assert_eq!(
            extract_in_flight_from_worklog("in_flight: 2"),
            Some(2)
        );
        assert_eq!(
            extract_in_flight_from_worklog("In flight count: 5"),
            Some(5)
        );
        assert_eq!(
            extract_in_flight_from_worklog("No relevant info here"),
            None
        );
    }

    #[test]
    fn find_worklog_file_matches_cycle_pattern() {
        let files = vec![
            "docs/worklog/2026-03-10/cycle-218-worklog.md".to_string(),
            "docs/journal/2026-03-10.md".to_string(),
        ];
        assert_eq!(
            find_worklog_file(&files, 218),
            Some("docs/worklog/2026-03-10/cycle-218-worklog.md".to_string())
        );
        assert_eq!(find_worklog_file(&files, 999), None);
    }

    #[test]
    fn find_journal_file_matches_pattern() {
        let files = vec![
            "docs/worklog/2026-03-10/cycle-218-worklog.md".to_string(),
            "docs/journal/2026-03-10.md".to_string(),
        ];
        assert_eq!(
            find_journal_file(&files),
            Some("docs/journal/2026-03-10.md".to_string())
        );
    }

    #[test]
    fn extract_section_content_returns_section_body() {
        let content =
            "## Self-modifications\nUpdated tools/foo\nAdded bar\n## Next steps\nDo things";
        let section = extract_section_content(content, "self-modification");
        assert!(section.is_some());
        let body = section.unwrap();
        assert!(body.contains("Updated tools/foo"));
        assert!(body.contains("Added bar"));
        assert!(!body.contains("Do things"));
    }

    #[test]
    fn base64_decode_round_trip() {
        let decoded = decode_base64("SGVsbG8gV29ybGQ=").expect("should decode");
        assert_eq!(decoded, "Hello World");
    }

    #[test]
    fn base64_decode_with_newlines() {
        let decoded = decode_base64("SGVs\nbG8g\nV29y\nbGQ=\n").expect("should decode");
        assert_eq!(decoded, "Hello World");
    }

    #[test]
    fn check_report_overall_pass_when_all_pass() {
        let results = vec![
            CheckResult {
                check: "a".to_string(),
                status: CheckStatus::Pass,
                detail: "ok".to_string(),
            },
            CheckResult {
                check: "b".to_string(),
                status: CheckStatus::Pass,
                detail: "ok".to_string(),
            },
        ];
        let overall = if results.iter().all(|r| r.status == CheckStatus::Pass) {
            CheckStatus::Pass
        } else {
            CheckStatus::Fail
        };
        assert_eq!(overall, CheckStatus::Pass);
    }

    #[test]
    fn check_report_overall_fail_when_any_fail() {
        let results = vec![
            CheckResult {
                check: "a".to_string(),
                status: CheckStatus::Pass,
                detail: "ok".to_string(),
            },
            CheckResult {
                check: "b".to_string(),
                status: CheckStatus::Fail,
                detail: "bad".to_string(),
            },
        ];
        let overall = if results.iter().all(|r| r.status == CheckStatus::Pass) {
            CheckStatus::Pass
        } else {
            CheckStatus::Fail
        };
        assert_eq!(overall, CheckStatus::Fail);
    }
}
