use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use state_schema::read_state_value;
use std::collections::BTreeSet;
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

const TEMPORAL_STATE_SNAPSHOT_FIELDS: &[(&str, &str)] = &[
    ("cycle_phase.phase", "/cycle_phase/phase"),
    ("copilot_metrics.in_flight", "/copilot_metrics/in_flight"),
    (
        "copilot_metrics.total_dispatches",
        "/copilot_metrics/total_dispatches",
    ),
    ("copilot_metrics.merged", "/copilot_metrics/merged"),
    (
        "copilot_metrics.produced_pr",
        "/copilot_metrics/produced_pr",
    ),
    ("copilot_metrics.resolved", "/copilot_metrics/resolved"),
    (
        "copilot_metrics.dispatch_to_pr_rate",
        "/copilot_metrics/dispatch_to_pr_rate",
    ),
    (
        "copilot_metrics.pr_merge_rate",
        "/copilot_metrics/pr_merge_rate",
    ),
    (
        "copilot_metrics.dispatch_log_latest",
        "/copilot_metrics/dispatch_log_latest",
    ),
];

// Currently no quality fields are monitored in the master-vs-PR snapshot diff.
// Add only fields here that should never diverge between doc dispatch and doc review.
const QUALITY_STATE_SNAPSHOT_FIELDS: &[(&str, &str)] = &[
    ("last_cycle.summary", "/last_cycle/summary"),
    ("last_cycle.timestamp", "/last_cycle/timestamp"),
    ("last_cycle.number", "/last_cycle/number"),
    ("last_cycle.issue", "/last_cycle/issue"),
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

    /// Freeze receipt completeness expectations to commits strictly before this RFC3339 timestamp
    #[arg(long)]
    dispatched_at: Option<String>,
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
    Warn,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckReport {
    pub overall: CheckStatus,
    pub results: Vec<CheckResult>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct ReceiptEntry {
    receipt: String,
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
    let dispatched_at = resolve_dispatched_at(&cli.repo_root, cli.dispatched_at.as_deref())?;

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

    // 5. state_snapshot_freshness
    results.push(check_state_snapshot_freshness(&cli.repo_root, &pr_branch));

    // 6. self_modifications_accurate
    results.push(check_self_modifications_accurate(
        &cli.repo_root,
        cli.cycle,
        worklog_content.as_deref(),
    ));

    // 7. receipt_completeness
    results.push(check_receipt_completeness(
        &cli.repo_root,
        cli.cycle,
        worklog_content.as_deref(),
        dispatched_at.as_deref(),
    ));

    // 8. receipts_valid
    results.push(check_receipts_valid(
        &cli.repo_root,
        worklog_content.as_deref(),
    ));

    // 9. journal_has_worklog_link
    let journal_content = match &journal_file {
        Some(path) => fetch_file_content(&cli.repo_root, path, &pr_branch).ok(),
        None => None,
    };
    results.push(check_journal_has_worklog_link(journal_content.as_deref()));

    // 10. journal_entry_ordering
    results.push(check_journal_entry_ordering(journal_content.as_deref()));

    // 11. title_format
    results.push(check_title_format(journal_content.as_deref()));

    // 12. no_duplicate_headers
    results.push(check_no_duplicate_headers(journal_content.as_deref()));

    // 13. worklog_consistency
    results.push(check_worklog_consistency(worklog_content.as_deref()));

    let overall = if results.iter().any(|r| r.status == CheckStatus::Fail) {
        CheckStatus::Fail
    } else {
        CheckStatus::Pass
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
        .args(["pr", "diff", &pr.to_string(), "--repo", REPO, "--name-only"])
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
    String::from_utf8(bytes)
        .map_err(|error| format!("base64 content is not valid UTF-8: {}", error))
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
        ("What was done", &["what was done", "## done", "## summary"]),
        (
            "Self-modifications",
            &["self-modifications", "self-modification"],
        ),
        ("Current state", &["current state", "## state"]),
        ("Next steps", &["next steps", "## next"]),
        ("Commit receipts", &["commit receipts", "## receipts"]),
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
                    status: CheckStatus::Warn,
                    detail: format!(
                        "Temporal divergence: worklog says {} (at doc dispatch time), state.json now says {} (state has advanced since documentation was generated).",
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

fn check_state_snapshot_freshness(repo_root: &Path, pr_branch: &str) -> CheckResult {
    let check_name = "state_snapshot_freshness".to_string();

    let master_state = match read_state_value(repo_root) {
        Ok(state) => state,
        Err(error) => {
            return CheckResult {
                check: check_name,
                status: CheckStatus::Fail,
                detail: format!("Cannot read master docs/state.json: {}", error),
            };
        }
    };

    let pr_state_content = match fetch_file_content(repo_root, "docs/state.json", pr_branch) {
        Ok(content) => content,
        Err(error) => {
            return CheckResult {
                check: check_name,
                status: CheckStatus::Fail,
                detail: format!("Cannot read PR docs/state.json: {}", error),
            };
        }
    };

    let pr_state: Value = match serde_json::from_str(&pr_state_content) {
        Ok(state) => state,
        Err(error) => {
            return CheckResult {
                check: check_name,
                status: CheckStatus::Fail,
                detail: format!("Cannot parse PR docs/state.json: {}", error),
            };
        }
    };

    evaluate_state_snapshot_freshness(&master_state, &pr_state)
}

fn evaluate_state_snapshot_freshness(master_state: &Value, pr_state: &Value) -> CheckResult {
    evaluate_state_snapshot_freshness_with_fields(
        master_state,
        pr_state,
        TEMPORAL_STATE_SNAPSHOT_FIELDS,
        QUALITY_STATE_SNAPSHOT_FIELDS,
    )
}

fn evaluate_state_snapshot_freshness_with_fields(
    master_state: &Value,
    pr_state: &Value,
    temporal_fields: &[(&str, &str)],
    quality_fields: &[(&str, &str)],
) -> CheckResult {
    let check_name = "state_snapshot_freshness".to_string();
    let temporal_divergences =
        find_state_snapshot_divergences(master_state, pr_state, temporal_fields);
    let quality_divergences =
        find_state_snapshot_divergences(master_state, pr_state, quality_fields);

    if temporal_divergences.is_empty() && quality_divergences.is_empty() {
        CheckResult {
            check: check_name,
            status: CheckStatus::Pass,
            detail: "Master and PR state snapshots match for monitored fields".to_string(),
        }
    } else if quality_divergences.is_empty() {
        CheckResult {
            check: check_name,
            status: CheckStatus::Warn,
            detail: format!(
                "Temporal divergences (expected): {}",
                temporal_divergences.join("; ")
            ),
        }
    } else if temporal_divergences.is_empty() {
        CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: format!("Quality divergences: {}", quality_divergences.join("; ")),
        }
    } else {
        CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: format!(
                "Quality divergences: {}. Temporal divergences (expected): {}",
                quality_divergences.join("; "),
                temporal_divergences.join("; ")
            ),
        }
    }
}

fn find_state_snapshot_divergences(
    master_state: &Value,
    pr_state: &Value,
    fields: &[(&str, &str)],
) -> Vec<String> {
    let mut divergences = Vec::new();

    for (label, pointer) in fields {
        let master_value = master_state.pointer(pointer);
        let pr_value = pr_state.pointer(pointer);
        if master_value != pr_value {
            divergences.push(format!(
                "{}: master={}, pr={}",
                label,
                format_state_value(master_value),
                format_state_value(pr_value)
            ));
        }
    }

    divergences
}

fn format_state_value(value: Option<&Value>) -> String {
    match value {
        Some(Value::String(text)) => format!("{:?}", text),
        Some(other) => other.to_string(),
        None => "missing".to_string(),
    }
}

pub fn extract_in_flight_from_worklog(content: &str) -> Option<i64> {
    // Search for patterns like "in-flight: 3", "in_flight: 3", "In flight: 3"
    let lower = content.to_lowercase();
    for line in lower.lines() {
        // Look for "in.flight" (any separator) followed by a number
        if let Some(pos) = line.find("in") {
            let rest = &line[pos + 2..];
            // Check for separator characters between "in" and "flight"
            let rest_trimmed = rest.trim_start_matches(['-', '_', ' ']);
            if let Some(after_flight) = rest_trimmed.strip_prefix("flight") {
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

fn fetch_cycle_receipts(
    repo_root: &Path,
    cycle: u64,
    before: Option<&str>,
) -> Result<Vec<ReceiptEntry>, String> {
    let mut command = Command::new("bash");
    command
        .current_dir(repo_root)
        .arg("tools/cycle-receipts")
        .arg("--cycle")
        .arg(cycle.to_string())
        .arg("--repo-root")
        .arg(repo_root.display().to_string())
        .arg("--json");
    if let Some(timestamp) = before {
        command.arg("--before").arg(timestamp);
    }
    let output = command.output().map_err(|error| {
        format!(
            "failed to run bash tools/cycle-receipts --cycle {} --repo-root {} --json{}: {}",
            cycle,
            repo_root.display(),
            before.map_or_else(String::new, |timestamp| format!(" --before {}", timestamp)),
            error
        )
    })?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(if stderr.is_empty() {
            format!("cycle-receipts command failed for cycle {}", cycle)
        } else {
            format!(
                "cycle-receipts command failed for cycle {}: {}",
                cycle, stderr
            )
        });
    }

    serde_json::from_slice::<Vec<ReceiptEntry>>(&output.stdout)
        .map_err(|error| format!("failed to parse cycle-receipts JSON: {}", error))
}

fn check_receipt_completeness(
    repo_root: &Path,
    cycle: u64,
    worklog_content: Option<&str>,
    dispatched_at: Option<&str>,
) -> CheckResult {
    check_receipt_completeness_with_loader(
        repo_root,
        cycle,
        worklog_content,
        dispatched_at,
        fetch_cycle_receipts,
    )
}

/// Evaluate receipt completeness using an injected receipt loader with the
/// signature `Fn(&Path, u64, Option<&str>) -> Result<Vec<ReceiptEntry>, String>`.
fn check_receipt_completeness_with_loader<F>(
    repo_root: &Path,
    cycle: u64,
    worklog_content: Option<&str>,
    dispatched_at: Option<&str>,
    receipt_loader: F,
) -> CheckResult
where
    F: Fn(&Path, u64, Option<&str>) -> Result<Vec<ReceiptEntry>, String>,
{
    let Some(content) = worklog_content else {
        return CheckResult {
            check: "receipt_completeness".to_string(),
            status: CheckStatus::Fail,
            detail: "Cannot check: worklog content not available".to_string(),
        };
    };

    let expected = match receipt_loader(repo_root, cycle, dispatched_at) {
        Ok(expected) => expected,
        Err(error) => {
            return CheckResult {
                check: "receipt_completeness".to_string(),
                status: CheckStatus::Fail,
                detail: format!("Cannot load canonical cycle receipts: {}", error),
            };
        }
    };

    evaluate_receipt_completeness(Some(content), &expected)
}

/// Resolve the dispatch timestamp by preferring the CLI override and otherwise
/// falling back to `/cycle_phase/dispatched_at` in `docs/state.json`.
fn resolve_dispatched_at(
    repo_root: &Path,
    cli_dispatched_at: Option<&str>,
) -> Result<Option<String>, String> {
    if let Some(timestamp) = cli_dispatched_at {
        return Ok(Some(timestamp.to_string()));
    }

    let state = read_state_value(repo_root)?;
    Ok(state
        .pointer("/cycle_phase/dispatched_at")
        .and_then(Value::as_str)
        .map(str::to_string))
}

fn evaluate_receipt_completeness(
    worklog_content: Option<&str>,
    expected: &[ReceiptEntry],
) -> CheckResult {
    let check_name = "receipt_completeness".to_string();

    let Some(content) = worklog_content else {
        return CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Cannot check: worklog content not available".to_string(),
        };
    };

    let present = extract_present_receipts(content);
    let missing = expected
        .iter()
        .filter_map(|entry| {
            let receipt = entry.receipt.trim();
            (!receipt.is_empty() && !present.contains(receipt)).then(|| receipt.to_string())
        })
        .collect::<Vec<_>>();

    if missing.is_empty() {
        CheckResult {
            check: check_name,
            status: CheckStatus::Pass,
            detail: format!(
                "All {} canonical receipt(s) are present in the worklog table",
                expected.len()
            ),
        }
    } else {
        CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: format!(
                "Worklog commit receipts table is missing canonical receipt(s): {}",
                missing.join(", ")
            ),
        }
    }
}

fn extract_present_receipts(content: &str) -> BTreeSet<String> {
    let Some(section) = extract_section_content(content, "commit receipts") else {
        return BTreeSet::new();
    };

    section
        .lines()
        .filter(|line| line.trim_start().starts_with('|'))
        .filter_map(|line| {
            let cells = line
                .split('|')
                .map(str::trim)
                .filter(|cell| !cell.is_empty())
                .collect::<Vec<_>>();
            if cells.is_empty() || cells.iter().all(|cell| is_table_separator_cell(cell)) {
                return None;
            }
            cells
                .get(1)
                .and_then(|cell| extract_receipt_from_cell(cell))
        })
        .collect()
}

fn extract_receipt_from_cell(cell: &str) -> Option<String> {
    if let Some(start) = cell.find('`') {
        let end = cell[start + 1..].find('`')?;
        let receipt = &cell[start + 1..start + 1 + end];
        return is_short_hex(receipt).then(|| receipt.to_string());
    }

    if let Some(bracket_end) = cell.find("](") {
        let bracket_start = cell.find('[')?;
        let receipt = cell[bracket_start + 1..bracket_end].trim_matches('`');
        return is_short_hex(receipt).then(|| receipt.to_string());
    }

    let trimmed = cell.trim_matches(|character| matches!(character, '[' | ']' | '`'));
    is_short_hex(trimmed).then(|| trimmed.to_string())
}

fn is_table_separator_cell(cell: &str) -> bool {
    let trimmed = cell.trim();
    !trimmed.is_empty()
        && trimmed
            .chars()
            .all(|character| matches!(character, '-' | ':' | ' '))
}

fn is_short_hex(value: &str) -> bool {
    value.len() >= 7 && value.chars().all(|character| character.is_ascii_hexdigit())
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
    if current.len() >= min_len
        && current.len() <= 40
        && !current.chars().all(|ch| ch.is_ascii_digit())
    {
        results.push(current);
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

fn check_journal_entry_ordering(journal_content: Option<&str>) -> CheckResult {
    let check_name = "journal_entry_ordering".to_string();

    let Some(content) = journal_content else {
        return CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Cannot check: journal content not available".to_string(),
        };
    };

    let cycle_headers = extract_journal_cycle_headers(content);
    let mut previous_cycle = None;

    for (_, cycle, line) in cycle_headers {
        if let Some(previous) = previous_cycle {
            if cycle < previous {
                return CheckResult {
                    check: check_name,
                    status: CheckStatus::Fail,
                    detail: format!(
                        "Cycle {} appears after cycle {} in journal header order: {}",
                        cycle, previous, line
                    ),
                };
            }
        }
        previous_cycle = Some(cycle);
    }

    CheckResult {
        check: check_name,
        status: CheckStatus::Pass,
        detail: "Journal cycle headers are in ascending order".to_string(),
    }
}

fn check_title_format(journal_content: Option<&str>) -> CheckResult {
    let check_name = "title_format".to_string();

    let Some(content) = journal_content else {
        return CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Cannot check: journal content not available".to_string(),
        };
    };

    let mut stuttering_headers = Vec::new();
    for (_, cycle, line) in extract_journal_cycle_headers(content) {
        let needle = format!("Cycle {}: Cycle {}:", cycle, cycle);
        if line.contains(&needle) {
            stuttering_headers.push(line);
        }
    }

    if stuttering_headers.is_empty() {
        CheckResult {
            check: check_name,
            status: CheckStatus::Pass,
            detail: "No stuttering journal titles detected".to_string(),
        }
    } else {
        CheckResult {
            check: check_name,
            status: CheckStatus::Warn,
            detail: format!(
                "Detected stuttering journal titles: {}",
                stuttering_headers.join(" | ")
            ),
        }
    }
}

fn extract_journal_cycle_headers(content: &str) -> Vec<(usize, u64, String)> {
    let mut headers = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with("## ") {
            continue;
        }

        let Some(marker_index) = trimmed.find("— Cycle ") else {
            continue;
        };
        let after_marker = &trimmed[marker_index + "— Cycle ".len()..];
        let digits: String = after_marker
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .collect();
        if digits.is_empty() {
            continue;
        }

        let cycle = match digits.parse::<u64>() {
            Ok(value) => value,
            Err(_) => continue,
        };

        let remainder = &after_marker[digits.len()..];
        if !remainder.starts_with(':') {
            continue;
        }

        headers.push((marker_index, cycle, trimmed.to_string()));
    }

    headers
}

fn check_worklog_consistency(worklog_content: Option<&str>) -> CheckResult {
    let check_name = "worklog_consistency".to_string();

    let Some(content) = worklog_content else {
        return CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Cannot check: worklog content not available".to_string(),
        };
    };

    let Some(current_state_section) = extract_section_content(content, "current state") else {
        return CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: "Cannot check: Current state section not available".to_string(),
        };
    };

    let prose_in_flight = extract_current_state_in_flight_count(&current_state_section);
    let metrics_in_flight = extract_copilot_metrics_in_flight_count(&current_state_section);

    match (prose_in_flight, metrics_in_flight) {
        (Some(prose), Some(metrics)) if prose != metrics => CheckResult {
            check: check_name,
            status: CheckStatus::Fail,
            detail: format!(
                "Current state says {}, Copilot metrics says {} in flight",
                prose, metrics
            ),
        },
        (Some(prose), Some(_)) => CheckResult {
            check: check_name,
            status: CheckStatus::Pass,
            detail: format!("Current state in-flight counts are consistent at {}", prose),
        },
        _ => CheckResult {
            check: check_name,
            status: CheckStatus::Pass,
            detail: "Could not find both in-flight counts; no inconsistency detected".to_string(),
        },
    }
}

fn extract_current_state_in_flight_count(section: &str) -> Option<i64> {
    for line in section.lines() {
        let lower = line.to_lowercase();
        if lower.contains("in-flight agent sessions") || lower.contains("in flight agent sessions")
        {
            if let Some(value) = extract_first_number(&lower) {
                return Some(value);
            }
        }
    }

    None
}

fn extract_copilot_metrics_in_flight_count(section: &str) -> Option<i64> {
    for line in section.lines() {
        let lower = line.to_lowercase();
        if !lower.contains("copilot metrics") || !lower.contains("in flight") {
            continue;
        }

        if let Some(prefix) = lower.split("in flight").next() {
            if let Some(value) = extract_last_number(prefix) {
                return Some(value);
            }
        }
    }

    None
}

fn extract_last_number(s: &str) -> Option<i64> {
    let mut end = None;
    for (i, c) in s.char_indices().rev() {
        if c.is_ascii_digit() {
            if end.is_none() {
                end = Some(i + c.len_utf8());
            }
        } else if let Some(end_idx) = end {
            let start_idx = i + c.len_utf8();
            return s[start_idx..end_idx].parse().ok();
        }
    }

    end.and_then(|end_idx| s[..end_idx].parse().ok())
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
    use std::fs;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

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
        assert!(
            help_text.contains("--dispatched-at"),
            "help should mention --dispatched-at flag"
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
    fn extract_receipt_from_cell_supports_plain_markdown_links() {
        assert_eq!(
            extract_receipt_from_cell("[abc1234](https://example.com/abc1234)"),
            Some("abc1234".to_string())
        );
    }

    #[test]
    fn in_flight_extraction_from_worklog() {
        assert_eq!(extract_in_flight_from_worklog("In-flight: 3"), Some(3));
        assert_eq!(extract_in_flight_from_worklog("in_flight: 2"), Some(2));
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
    fn receipt_completeness_passes_when_all_canonical_receipts_are_present() {
        let worklog = r#"## Commit receipts
| Step | Receipt | Commit |
|------|---------|--------|
| cycle-start | [`abc1234`](https://example.com/abc1234) | state(cycle-start): begin cycle 219 |
| process-review | [`def5678`](https://example.com/def5678) | state(process-review): consumed review |
"#;
        let expected = vec![
            ReceiptEntry {
                receipt: "abc1234".to_string(),
            },
            ReceiptEntry {
                receipt: "def5678".to_string(),
            },
        ];

        let result = evaluate_receipt_completeness(Some(worklog), &expected);

        assert_eq!(result.check, "receipt_completeness");
        assert_eq!(result.status, CheckStatus::Pass);
    }

    #[test]
    fn receipt_completeness_fails_when_worklog_is_missing_canonical_receipts() {
        let worklog = r#"## Commit receipts
| Step | Receipt | Commit |
|------|---------|--------|
| cycle-start | [`abc1234`](https://example.com/abc1234) | state(cycle-start): begin cycle 219 |
"#;
        let expected = vec![
            ReceiptEntry {
                receipt: "abc1234".to_string(),
            },
            ReceiptEntry {
                receipt: "def5678".to_string(),
            },
        ];

        let result = evaluate_receipt_completeness(Some(worklog), &expected);

        assert_eq!(result.check, "receipt_completeness");
        assert_eq!(result.status, CheckStatus::Fail);
        assert!(result.detail.contains("def5678"));
    }

    #[test]
    fn receipt_completeness_fails_when_worklog_has_no_receipt_table() {
        let expected = vec![ReceiptEntry {
            receipt: "abc1234".to_string(),
        }];

        let result =
            evaluate_receipt_completeness(Some("## Current state\nNo receipts here"), &expected);

        assert_eq!(result.check, "receipt_completeness");
        assert_eq!(result.status, CheckStatus::Fail);
        assert!(result.detail.contains("abc1234"));
    }

    #[test]
    fn receipt_completeness_uses_dispatched_at_to_freeze_expected_set() {
        let worklog = r#"## Commit receipts
| Step | Receipt | Commit |
|------|---------|--------|
| cycle-start | [`abc1234`](https://example.com/abc1234) | state(cycle-start): begin cycle 219 |
"#;

        let result = check_receipt_completeness_with_loader(
            Path::new("/tmp/repo"),
            219,
            Some(worklog),
            Some("2026-03-10T12:00:00Z"),
            |_, _, before| {
                assert_eq!(before, Some("2026-03-10T12:00:00Z"));
                Ok(vec![ReceiptEntry {
                    receipt: "abc1234".to_string(),
                }])
            },
        );

        assert_eq!(result.status, CheckStatus::Pass);
    }

    #[test]
    fn receipt_completeness_without_dispatched_at_keeps_current_behavior() {
        let worklog = r#"## Commit receipts
| Step | Receipt | Commit |
|------|---------|--------|
| cycle-start | [`abc1234`](https://example.com/abc1234) | state(cycle-start): begin cycle 219 |
"#;

        let result = check_receipt_completeness_with_loader(
            Path::new("/tmp/repo"),
            219,
            Some(worklog),
            None,
            |_, _, before| {
                assert_eq!(before, None);
                Ok(vec![
                    ReceiptEntry {
                        receipt: "abc1234".to_string(),
                    },
                    ReceiptEntry {
                        receipt: "def5678".to_string(),
                    },
                ])
            },
        );

        assert_eq!(result.status, CheckStatus::Fail);
        assert!(result.detail.contains("def5678"));
    }

    #[test]
    fn resolve_dispatched_at_reads_cycle_phase_timestamp_from_state() {
        let repo_root = create_temp_repo_root_with_state(json!({
            "cycle_phase": {
                "dispatched_at": "2026-03-10T12:00:00Z"
            }
        }));

        let dispatched_at =
            resolve_dispatched_at(&repo_root, None).expect("state lookup should succeed");

        assert_eq!(dispatched_at.as_deref(), Some("2026-03-10T12:00:00Z"));

        fs::remove_dir_all(repo_root).expect("temp repo should be removed");
    }

    #[test]
    fn resolve_dispatched_at_prefers_cli_flag_over_state() {
        let repo_root = create_temp_repo_root_with_state(json!({
            "cycle_phase": {
                "dispatched_at": "2026-03-10T12:00:00Z"
            }
        }));

        let dispatched_at = resolve_dispatched_at(&repo_root, Some("2026-03-10T11:00:00Z"))
            .expect("cli value should be accepted");

        assert_eq!(dispatched_at.as_deref(), Some("2026-03-10T11:00:00Z"));

        fs::remove_dir_all(repo_root).expect("temp repo should be removed");
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

    #[test]
    fn check_report_overall_pass_when_warnings_are_present() {
        let results = vec![
            CheckResult {
                check: "a".to_string(),
                status: CheckStatus::Pass,
                detail: "ok".to_string(),
            },
            CheckResult {
                check: "b".to_string(),
                status: CheckStatus::Warn,
                detail: "warning".to_string(),
            },
        ];
        let overall = if results.iter().any(|r| r.status == CheckStatus::Fail) {
            CheckStatus::Fail
        } else {
            CheckStatus::Pass
        };
        assert_eq!(overall, CheckStatus::Pass);
    }

    #[test]
    fn state_snapshot_divergence_check_passes_when_monitored_fields_match() {
        let master_state = json!({
            "cycle_phase": { "phase": "close_out" },
            "copilot_metrics": {
                "in_flight": 0,
                "total_dispatches": 10,
                "merged": 8
            }
        });
        let pr_state = json!({
            "cycle_phase": { "phase": "close_out" },
            "copilot_metrics": {
                "in_flight": 0,
                "total_dispatches": 10,
                "merged": 8
            }
        });

        let result = evaluate_state_snapshot_freshness(&master_state, &pr_state);

        assert_eq!(result.check, "state_snapshot_freshness");
        assert_eq!(result.status, CheckStatus::Pass);
        assert_eq!(
            result.detail,
            "Master and PR state snapshots match for monitored fields"
        );
    }

    #[test]
    fn state_snapshot_divergence_check_with_fields_passes_when_no_divergences_exist() {
        let master_state = json!({
            "last_cycle": {
                "summary": "Cycle complete",
                "number": 224
            }
        });
        let pr_state = json!({
            "last_cycle": {
                "summary": "Cycle complete",
                "number": 224
            }
        });

        let result = evaluate_state_snapshot_freshness_with_fields(
            &master_state,
            &pr_state,
            &[("last_cycle.summary", "/last_cycle/summary")],
            &[("last_cycle.number", "/last_cycle/number")],
        );

        assert_eq!(result.check, "state_snapshot_freshness");
        assert_eq!(result.status, CheckStatus::Pass);
        assert_eq!(
            result.detail,
            "Master and PR state snapshots match for monitored fields"
        );
    }

    #[test]
    fn state_snapshot_divergence_check_warns_for_temporal_divergences_only() {
        let master_state = json!({
            "cycle_phase": { "phase": "close_out" },
            "copilot_metrics": {
                "in_flight": 0,
                "total_dispatches": 11,
                "merged": 9,
                "produced_pr": 10,
                "resolved": 12,
                "dispatch_to_pr_rate": "90.9%",
                "pr_merge_rate": "90.0%"
            }
        });
        let pr_state = json!({
            "cycle_phase": { "phase": "doc_dispatched" },
            "copilot_metrics": {
                "in_flight": 2,
                "total_dispatches": 10,
                "merged": 8,
                "produced_pr": 9,
                "resolved": 11,
                "dispatch_to_pr_rate": "90.0%",
                "pr_merge_rate": "88.9%"
            }
        });

        let result = evaluate_state_snapshot_freshness(&master_state, &pr_state);

        assert_eq!(result.check, "state_snapshot_freshness");
        assert_eq!(result.status, CheckStatus::Warn);
        assert!(result.detail.contains("Temporal divergences (expected):"));
        assert!(result
            .detail
            .contains("cycle_phase.phase: master=\"close_out\", pr=\"doc_dispatched\""));
        assert!(result
            .detail
            .contains("copilot_metrics.in_flight: master=0, pr=2"));
        assert!(result
            .detail
            .contains("copilot_metrics.total_dispatches: master=11, pr=10"));
        assert!(result
            .detail
            .contains("copilot_metrics.merged: master=9, pr=8"));
        assert!(result
            .detail
            .contains("copilot_metrics.produced_pr: master=10, pr=9"));
        assert!(result
            .detail
            .contains("copilot_metrics.resolved: master=12, pr=11"));
        assert!(result
            .detail
            .contains("copilot_metrics.dispatch_to_pr_rate: master=\"90.9%\", pr=\"90.0%\""));
        assert!(result
            .detail
            .contains("copilot_metrics.pr_merge_rate: master=\"90.0%\", pr=\"88.9%\""));
    }

    #[test]
    fn state_snapshot_divergence_check_with_fields_warns_for_temporal_divergences_only() {
        let master_state = json!({
            "cycle_phase": { "phase": "close_out" }
        });
        let pr_state = json!({
            "cycle_phase": { "phase": "doc_review" }
        });

        let result = evaluate_state_snapshot_freshness_with_fields(
            &master_state,
            &pr_state,
            &[("cycle_phase.phase", "/cycle_phase/phase")],
            &[],
        );

        assert_eq!(result.status, CheckStatus::Warn);
        assert_eq!(
            result.detail,
            "Temporal divergences (expected): cycle_phase.phase: master=\"close_out\", pr=\"doc_review\""
        );
    }

    #[test]
    fn state_snapshot_divergence_check_fails_for_quality_divergences() {
        let master_state = json!({
            "last_cycle": { "cycle": 223 }
        });
        let pr_state = json!({
            "last_cycle": { "cycle": 222 }
        });

        let result = evaluate_state_snapshot_freshness_with_fields(
            &master_state,
            &pr_state,
            &[],
            &[("last_cycle.cycle", "/last_cycle/cycle")],
        );

        assert_eq!(result.status, CheckStatus::Fail);
        assert!(result
            .detail
            .contains("Quality divergences: last_cycle.cycle: master=223, pr=222"));
    }

    #[test]
    fn state_snapshot_divergence_check_fails_for_mixed_temporal_and_quality_divergences() {
        let master_state = json!({
            "cycle_phase": { "phase": "close_out" },
            "last_cycle": { "cycle": 223 }
        });
        let pr_state = json!({
            "cycle_phase": { "phase": "doc_dispatched" },
            "last_cycle": { "cycle": 222 }
        });

        let result = evaluate_state_snapshot_freshness_with_fields(
            &master_state,
            &pr_state,
            &[("cycle_phase.phase", "/cycle_phase/phase")],
            &[("last_cycle.cycle", "/last_cycle/cycle")],
        );

        assert_eq!(result.status, CheckStatus::Fail);
        assert!(result
            .detail
            .contains("Quality divergences: last_cycle.cycle: master=223, pr=222"));
        assert!(result.detail.contains(
            "Temporal divergences (expected): cycle_phase.phase: master=\"close_out\", pr=\"doc_dispatched\""
        ));
    }

    #[test]
    fn find_state_snapshot_divergences_reports_pointer_value_differences() {
        let master_state = json!({
            "last_cycle": {
                "summary": "Cycle 224 closed",
                "timestamp": "2026-03-11T10:00:00Z"
            }
        });
        let pr_state = json!({
            "last_cycle": {
                "summary": "Cycle 224 drafted"
            }
        });

        let divergences = find_state_snapshot_divergences(
            &master_state,
            &pr_state,
            &[
                ("last_cycle.summary", "/last_cycle/summary"),
                ("last_cycle.timestamp", "/last_cycle/timestamp"),
            ],
        );

        assert_eq!(
            divergences,
            vec![
                "last_cycle.summary: master=\"Cycle 224 closed\", pr=\"Cycle 224 drafted\""
                    .to_string(),
                "last_cycle.timestamp: master=\"2026-03-11T10:00:00Z\", pr=missing".to_string(),
            ]
        );
    }

    #[test]
    fn state_snapshot_monitored_field_constants_are_non_empty_and_use_json_pointers() {
        assert!(!TEMPORAL_STATE_SNAPSHOT_FIELDS.is_empty());
        assert!(!QUALITY_STATE_SNAPSHOT_FIELDS.is_empty());
        assert!(QUALITY_STATE_SNAPSHOT_FIELDS
            .iter()
            .all(|field| !TEMPORAL_STATE_SNAPSHOT_FIELDS.contains(field)));

        for fields in [
            TEMPORAL_STATE_SNAPSHOT_FIELDS,
            QUALITY_STATE_SNAPSHOT_FIELDS,
        ] {
            for (label, pointer) in fields {
                assert!(!label.is_empty());
                assert!(pointer.starts_with('/'));
            }
        }
    }

    #[test]
    fn quality_state_snapshot_fields_include_cycle_defining_fields() {
        for required_field in [
            ("last_cycle.summary", "/last_cycle/summary"),
            ("last_cycle.timestamp", "/last_cycle/timestamp"),
            ("last_cycle.number", "/last_cycle/number"),
            ("last_cycle.issue", "/last_cycle/issue"),
        ] {
            assert!(QUALITY_STATE_SNAPSHOT_FIELDS.contains(&required_field));
        }
    }

    #[test]
    fn temporal_state_snapshot_fields_include_dispatch_log_latest() {
        assert!(TEMPORAL_STATE_SNAPSHOT_FIELDS.contains(&(
            "copilot_metrics.dispatch_log_latest",
            "/copilot_metrics/dispatch_log_latest",
        )));
    }

    #[test]
    fn state_snapshot_divergence_check_warns_for_dispatch_log_latest_temporal_drift() {
        let master_state = json!({
            "copilot_metrics": { "dispatch_log_latest": "2026-03-11T18:00:00Z" }
        });
        let pr_state = json!({
            "copilot_metrics": { "dispatch_log_latest": "2026-03-11T17:00:00Z" }
        });

        let result = evaluate_state_snapshot_freshness_with_fields(
            &master_state,
            &pr_state,
            TEMPORAL_STATE_SNAPSHOT_FIELDS,
            QUALITY_STATE_SNAPSHOT_FIELDS,
        );

        assert_eq!(result.status, CheckStatus::Warn);
        assert!(result.detail.contains(
            "copilot_metrics.dispatch_log_latest: master=\"2026-03-11T18:00:00Z\", pr=\"2026-03-11T17:00:00Z\""
        ));
    }

    #[test]
    fn in_flight_check_warns_when_state_has_advanced_since_dispatch() {
        let repo_root = create_temp_repo_root_with_in_flight(2);
        let result = check_in_flight_matches(&repo_root, Some("## Current state\nIn-flight: 1"));

        assert_eq!(result.check, "in_flight_matches");
        assert_eq!(result.status, CheckStatus::Warn);
        assert_eq!(
            result.detail,
            "Temporal divergence: worklog says 1 (at doc dispatch time), state.json now says 2 (state has advanced since documentation was generated)."
        );

        fs::remove_dir_all(repo_root).expect("temp repo should be removed");
    }

    static TEST_REPO_COUNTER: AtomicU64 = AtomicU64::new(0);

    fn create_temp_repo_root_with_in_flight(in_flight: i64) -> PathBuf {
        create_temp_repo_root_with_state(json!({
            "copilot_metrics": {
                "in_flight": in_flight
            }
        }))
    }

    fn create_temp_repo_root_with_state(state: Value) -> PathBuf {
        let unique = TEST_REPO_COUNTER.fetch_add(1, Ordering::Relaxed);
        let repo_root = std::env::temp_dir().join(format!(
            "check-doc-pr-test-{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be valid")
                .as_nanos(),
            unique
        ));
        fs::create_dir_all(repo_root.join("docs")).expect("docs directory should be created");
        let serialized = serde_json::to_string(&state).expect("state should serialize");
        fs::write(repo_root.join("docs/state.json"), serialized)
            .expect("state.json should be written");
        repo_root
    }

    #[test]
    fn journal_entry_ordering_passes_for_ascending_cycles() {
        let content = "\
## 2026-03-10 — Cycle 212: Review processing\n\
\n\
Worklog: [cycle 212](docs/worklog/2026-03-10/002904-cycle-212-summary.md)\n\
\n\
## 2026-03-10 — Cycle 213: Review heavy cycle\n\
\n\
Worklog: [cycle 213](docs/worklog/2026-03-10/030816-cycle-213-summary.md)\n\
\n\
## 2026-03-10 — Cycle 214: Tool improvement dispatches\n";

        let result = check_journal_entry_ordering(Some(content));

        assert_eq!(result.check, "journal_entry_ordering");
        assert_eq!(result.status, CheckStatus::Pass);
    }

    #[test]
    fn journal_entry_ordering_fails_for_out_of_order_cycles() {
        let content = "\
## 2026-03-10 — Cycle 212: Review processing\n\
\n\
## 2026-03-10 — Cycle 222: Review fixes and tool-audit bookkeeping\n\
\n\
## 2026-03-10 — Cycle 214: Review-heavy cycle with tool improvement dispatches\n";

        let result = check_journal_entry_ordering(Some(content));

        assert_eq!(result.check, "journal_entry_ordering");
        assert_eq!(result.status, CheckStatus::Fail);
        assert!(result.detail.contains("Cycle 214 appears after cycle 222"));
    }

    #[test]
    fn title_format_warns_on_stuttering_cycle_titles() {
        let content = "\
## 2026-03-10 — Cycle 213: Cycle 213: Review-heavy cycle with structural enforcement dispatch\n\
\n\
## 2026-03-10 — Cycle 214: Clean title\n";

        let result = check_title_format(Some(content));

        assert_eq!(result.check, "title_format");
        assert_eq!(result.status, CheckStatus::Warn);
        assert!(result.detail.contains("Cycle 213: Cycle 213:"));
    }

    #[test]
    fn title_format_passes_without_stuttering_cycle_titles() {
        let content = "\
## 2026-03-10 — Cycle 213: Review-heavy cycle with structural enforcement dispatch\n\
\n\
## 2026-03-10 — Cycle 214: Clean title\n";

        let result = check_title_format(Some(content));

        assert_eq!(result.check, "title_format");
        assert_eq!(result.status, CheckStatus::Pass);
    }

    #[test]
    fn worklog_consistency_passes_when_in_flight_counts_match() {
        let content = "\
## Current state\n\
\n\
- **In-flight agent sessions**: 1\n\
- **Copilot metrics**: 292 dispatches, 287 PRs produced, 285 merged, 292 resolved, 1 in flight, 1 reviewed awaiting Eva\n";

        let result = check_worklog_consistency(Some(content));

        assert_eq!(result.check, "worklog_consistency");
        assert_eq!(result.status, CheckStatus::Pass);
    }

    #[test]
    fn worklog_consistency_fails_when_in_flight_counts_differ() {
        let content = "\
## Current state\n\
\n\
- In-flight agent sessions: 1\n\
- **Copilot metrics**: 292 dispatches, 287 PRs produced, 285 merged, 292 resolved, 0 in flight, 1 reviewed awaiting Eva\n";

        let result = check_worklog_consistency(Some(content));

        assert_eq!(result.check, "worklog_consistency");
        assert_eq!(result.status, CheckStatus::Fail);
        assert!(result
            .detail
            .contains("Current state says 1, Copilot metrics says 0"));
    }
}
