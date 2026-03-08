use chrono::{DateTime, Duration, Utc};
use clap::Parser;
use serde::Serialize;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

const REPO: &str = "EvaLok/schema-org-json-ld";
const AGENT_ISSUE_ASSIGNEE: &str = "copilot-swe-agent[bot]";
const AGENT_PR_AUTHOR: &str = "copilot-swe-agent[bot]";

#[derive(Parser)]
#[command(name = "housekeeping-scan")]
struct Cli {
    /// Path to repository root (used to locate docs/state.json)
    #[arg(long)]
    repo_root: PathBuf,

    /// Output report as JSON
    #[arg(long)]
    json: bool,
}

#[derive(Default, Serialize)]
struct Report {
    stale_agent_issues: Vec<Finding>,
    orphan_draft_prs: Vec<Finding>,
    dead_branches: Vec<Finding>,
    stale_audit_inbound: Vec<Finding>,
    stale_qc_inbound: Vec<Finding>,
    items_needing_attention: usize,
}

#[derive(Serialize)]
struct Finding {
    identifier: String,
    age: String,
    recommended_action: String,
}

#[derive(Clone)]
struct PrState {
    branch: String,
    state: String,
}

/// Metadata for an open Copilot-authored draft PR used by both housekeeping checks.
#[derive(Clone)]
struct DraftPrInfo {
    number: u64,
    branch: String,
    created_at: DateTime<Utc>,
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = validate_repo_root(&cli.repo_root) {
        exit_with_error(e);
    }

    let now = Utc::now();
    let mut report = Report::default();
    let draft_prs = match scan_open_copilot_draft_prs() {
        Ok(value) => value,
        Err(e) => exit_with_error(e),
    };

    report.stale_agent_issues = match scan_stale_agent_issues(now, &draft_prs) {
        Ok(value) => value,
        Err(e) => exit_with_error(e),
    };
    report.orphan_draft_prs = match scan_orphan_draft_prs(now, &draft_prs) {
        Ok(value) => value,
        Err(e) => exit_with_error(e),
    };
    report.dead_branches = match scan_dead_branches(&cli.repo_root) {
        Ok(value) => value,
        Err(e) => exit_with_error(e),
    };
    report.stale_audit_inbound = match scan_stale_label_issues(now, "audit-inbound") {
        Ok(value) => value,
        Err(e) => exit_with_error(e),
    };
    report.stale_qc_inbound = match scan_stale_label_issues(now, "qc-inbound") {
        Ok(value) => value,
        Err(e) => exit_with_error(e),
    };
    report.items_needing_attention = total_findings(&report);

    if cli.json {
        match serde_json::to_string_pretty(&report) {
            Ok(json) => println!("{}", json),
            Err(e) => exit_with_error(format!("failed to serialize JSON output: {}", e)),
        }
    } else {
        print_human_report(&report);
    }

    std::process::exit(if report.items_needing_attention == 0 {
        0
    } else {
        1
    });
}

fn total_findings(report: &Report) -> usize {
    report.stale_agent_issues.len()
        + report.orphan_draft_prs.len()
        + report.dead_branches.len()
        + report.stale_audit_inbound.len()
        + report.stale_qc_inbound.len()
}

fn scan_stale_agent_issues(now: DateTime<Utc>, draft_prs: &[DraftPrInfo]) -> Result<Vec<Finding>, String> {
    let path = format!(
        "repos/{}/issues?assignee={}&state=open",
        REPO, AGENT_ISSUE_ASSIGNEE
    );
    let value = gh_json(&["api", &path, "--paginate"])?;
    let items = value
        .as_array()
        .ok_or_else(|| "unexpected response for stale agent issues query".to_string())?;
    Ok(find_stale_agent_issues(items, draft_prs, now))
}

fn find_stale_agent_issues(items: &[Value], draft_prs: &[DraftPrInfo], now: DateTime<Utc>) -> Vec<Finding> {
    items
        .iter()
        .filter(|issue| issue.get("pull_request").is_none())
        .filter_map(|issue| {
            let number = issue.get("number")?.as_u64()?;
            let created_at = parse_time(issue.get("created_at")?.as_str()?)?;
            let age = now.signed_duration_since(created_at);
            if age <= Duration::hours(2) {
                return None;
            }
            if has_linked_draft_pr(number, draft_prs) {
                return None;
            }
            Some(Finding {
                identifier: format!("#{}", number),
                age: format_duration(age),
                recommended_action: "Close issue or create/update linked PR to reflect active work"
                    .to_string(),
            })
        })
        .collect()
}

fn scan_open_copilot_draft_prs() -> Result<Vec<DraftPrInfo>, String> {
    let value = gh_json(&[
        "pr",
        "list",
        "--repo",
        REPO,
        "--state",
        "open",
        "--json",
        "number,title,isDraft,author,createdAt,headRefName",
    ])?;
    let prs = value
        .as_array()
        .ok_or_else(|| "unexpected response for open PR list".to_string())?;
    parse_open_copilot_draft_prs(prs)
}

/// Parse open Copilot draft PR metadata and fail closed on missing critical fields.
fn parse_open_copilot_draft_prs(prs: &[Value]) -> Result<Vec<DraftPrInfo>, String> {
    let mut draft_prs = Vec::new();

    for pr in prs.iter().filter(|pr| is_copilot_draft_pr(pr)) {
        let number = pr
            .get("number")
            .and_then(Value::as_u64)
            .ok_or_else(|| "copilot draft PR missing number".to_string())?;
        let branch = pr
            .get("headRefName")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("copilot draft PR #{} missing headRefName", number))?
            .to_string();
        let created_at_raw = pr
            .get("createdAt")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("copilot draft PR #{} missing createdAt", number))?;
        let created_at = parse_time(created_at_raw)
            .ok_or_else(|| format!("copilot draft PR #{} has invalid createdAt", number))?;

        draft_prs.push(DraftPrInfo {
            number,
            branch,
            created_at,
        });
    }

    Ok(draft_prs)
}

fn scan_orphan_draft_prs(now: DateTime<Utc>, draft_prs: &[DraftPrInfo]) -> Result<Vec<Finding>, String> {
    find_orphan_draft_prs(draft_prs, now)
}

fn find_orphan_draft_prs(draft_prs: &[DraftPrInfo], now: DateTime<Utc>) -> Result<Vec<Finding>, String> {
    let mut findings = Vec::new();

    for pr in draft_prs {
        let number = pr.number;
        let created_at = pr.created_at;
        let age = now.signed_duration_since(created_at);

        let timeline_path = format!("repos/{}/issues/{}/timeline", REPO, number);
        let timeline = gh_json(&[
            "api",
            &timeline_path,
            "--paginate",
            "-H",
            "Accept: application/vnd.github+json",
        ])?;
        if has_copilot_work_finished(&timeline) {
            findings.push(Finding {
                identifier: format!("#{}", number),
                age: format_duration(age),
                recommended_action: "Mark PR ready for review or close if work is complete"
                    .to_string(),
            });
        }
    }

    Ok(findings)
}

fn is_copilot_draft_pr(pr: &Value) -> bool {
    pr.get("isDraft").and_then(Value::as_bool).unwrap_or(false)
        && pr.pointer("/author/login").and_then(Value::as_str) == Some(AGENT_PR_AUTHOR)
}

fn has_copilot_work_finished(value: &Value) -> bool {
    value
        .as_array()
        .map(|events| {
            events.iter().any(|entry| {
                entry
                    .get("event")
                    .and_then(Value::as_str)
                    .map(|event| event == "copilot_work_finished")
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

fn has_linked_draft_pr(issue_number: u64, draft_prs: &[DraftPrInfo]) -> bool {
    draft_prs
        .iter()
        .any(|pr| branch_contains_issue_number(&pr.branch, issue_number))
}

fn branch_contains_issue_number(branch: &str, issue_number: u64) -> bool {
    branch
        .split(|c: char| !c.is_ascii_digit())
        .filter(|token| !token.is_empty())
        .filter_map(|token| token.parse::<u64>().ok())
        .any(|token| token == issue_number)
}

fn scan_dead_branches(repo_root: &Path) -> Result<Vec<Finding>, String> {
    let branches_output = Command::new("git")
        .args(["branch", "-r", "--list", "origin/*"])
        .current_dir(repo_root)
        .output()
        .map_err(|e| format!("failed to execute git branch query: {}", e))?;
    if !branches_output.status.success() {
        return Err(format!(
            "git branch query failed: {}",
            String::from_utf8_lossy(&branches_output.stderr).trim()
        ));
    }
    let remote_branches = parse_remote_branches(&String::from_utf8_lossy(&branches_output.stdout));

    let pr_state_value = gh_json(&[
        "api",
        &format!("repos/{}/pulls?state=all&per_page=100", REPO),
        "--paginate",
    ])?;
    let pr_states = parse_pr_states(&pr_state_value);
    Ok(find_dead_branches(&remote_branches, &pr_states))
}

fn parse_remote_branches(raw: &str) -> Vec<String> {
    raw.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.strip_prefix("origin/"))
        .filter(|name| *name != "HEAD -> origin/master")
        .filter(|name| *name != "master")
        .map(str::to_string)
        .collect()
}

fn parse_pr_states(value: &Value) -> Vec<PrState> {
    value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|item| {
                    Some(PrState {
                        branch: item.pointer("/head/ref")?.as_str()?.to_string(),
                        state: item.get("state")?.as_str()?.to_string(),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn find_dead_branches(remote_branches: &[String], pr_states: &[PrState]) -> Vec<Finding> {
    remote_branches
        .iter()
        .filter_map(|branch| {
            let has_open = pr_states
                .iter()
                .any(|pr| pr.branch == *branch && pr.state == "open");
            let has_closed_or_merged = pr_states
                .iter()
                .any(|pr| pr.branch == *branch && pr.state != "open");
            if has_open || !has_closed_or_merged {
                return None;
            }
            Some(Finding {
                identifier: branch.clone(),
                age: "n/a".to_string(),
                recommended_action: "Delete remote branch".to_string(),
            })
        })
        .collect()
}

fn scan_stale_label_issues(now: DateTime<Utc>, label: &str) -> Result<Vec<Finding>, String> {
    let value = gh_json(&[
        "issue",
        "list",
        "--repo",
        REPO,
        "--label",
        label,
        "--state",
        "open",
        "--json",
        "number,title,createdAt",
    ])?;
    let items = value
        .as_array()
        .ok_or_else(|| format!("unexpected response for {} issue list", label))?;
    Ok(find_open_label_issues(items, now, label))
}

fn find_open_label_issues(items: &[Value], now: DateTime<Utc>, label: &str) -> Vec<Finding> {
    items
        .iter()
        .filter_map(|item| {
            let number = item.get("number")?.as_u64()?;
            let created_at = item
                .get("createdAt")
                .and_then(Value::as_str)
                .and_then(parse_time)?;
            let age = now.signed_duration_since(created_at);
            Some(Finding {
                identifier: format!("#{}", number),
                age: format_duration(age),
                recommended_action: format!("Close {} issue after processing", label),
            })
        })
        .collect()
}

fn parse_time(value: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(value)
        .ok()
        .map(|dt| dt.with_timezone(&Utc))
}

fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.num_seconds().max(0);
    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    if hours > 0 {
        format!("{}h{}m", hours, minutes)
    } else {
        format!("{}m", minutes)
    }
}

fn print_human_report(report: &Report) {
    println!("Housekeeping Scan");
    println!();
    println!(
        "  Stale agent issues:     {}{}",
        report.stale_agent_issues.len(),
        format_identifiers(&report.stale_agent_issues)
    );
    println!(
        "  Orphan draft PRs:       {}{}",
        report.orphan_draft_prs.len(),
        format_identifiers(&report.orphan_draft_prs)
    );
    println!(
        "  Dead branches:          {}{}",
        report.dead_branches.len(),
        format_identifiers(&report.dead_branches)
    );
    println!(
        "  Stale audit-inbound:    {}{}",
        report.stale_audit_inbound.len(),
        format_identifiers(&report.stale_audit_inbound)
    );
    println!(
        "  Stale qc-inbound:       {}{}",
        report.stale_qc_inbound.len(),
        format_identifiers(&report.stale_qc_inbound)
    );
    println!();
    println!(
        "  Items needing attention: {}",
        report.items_needing_attention
    );
}

fn format_identifiers(findings: &[Finding]) -> String {
    if findings.is_empty() {
        return String::new();
    }

    let values = findings
        .iter()
        .map(|f| f.identifier.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    format!(" ({})", values)
}

fn gh_json(args: &[&str]) -> Result<Value, String> {
    let output = Command::new("gh")
        .args(args)
        .output()
        .map_err(|e| format!("failed to execute gh {}: {}", args.join(" "), e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!(
            "`gh {}` failed with status {}: {}",
            args.join(" "),
            output
                .status
                .code()
                .map(|code| code.to_string())
                .unwrap_or_else(|| "terminated by signal".to_string()),
            if stderr.is_empty() {
                "<no stderr>".to_string()
            } else {
                stderr
            }
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(|e| {
        format!(
            "failed to parse JSON output from `gh {}`: {}",
            args.join(" "),
            e
        )
    })
}

fn validate_repo_root(repo_root: &Path) -> Result<(), String> {
    let state_path = repo_root.join("docs/state.json");
    if !state_path.exists() {
        return Err(format!(
            "--repo-root must point to the repository root containing {}",
            state_path.display()
        ));
    }
    Ok(())
}

fn exit_with_error(message: String) -> ! {
    eprintln!("Error: {}", message);
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn stale_agent_issues_only_flags_over_two_hours() {
        let now = parse_time("2026-03-04T12:00:00Z").unwrap();
        let issues = vec![
            json!({"number": 1, "created_at": "2026-03-04T09:59:00Z"}),
            json!({"number": 2, "created_at": "2026-03-04T11:30:00Z"}),
            json!({"number": 3, "created_at": "2026-03-04T09:00:00Z", "pull_request": {}}),
        ];
        let findings = find_stale_agent_issues(&issues, &[], now);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].identifier, "#1");
    }

    #[test]
    fn stale_agent_issue_excluded_when_matching_draft_pr_branch_exists() {
        let now = parse_time("2026-03-04T12:00:00Z").unwrap();
        let issues = vec![json!({"number": 746, "created_at": "2026-03-04T09:00:00Z"})];
        let draft_prs = vec![DraftPrInfo {
            number: 10,
            branch: "copilot/add-severity-tiers-746".to_string(),
            created_at: parse_time("2026-03-04T10:00:00Z").unwrap(),
        }];

        let findings = find_stale_agent_issues(&issues, &draft_prs, now);

        assert!(findings.is_empty());
    }

    #[test]
    fn stale_agent_issue_still_flagged_without_draft_pr() {
        let now = parse_time("2026-03-04T12:00:00Z").unwrap();
        let issues = vec![json!({"number": 1, "created_at": "2026-03-04T09:00:00Z"})];

        let findings = find_stale_agent_issues(&issues, &[], now);

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].identifier, "#1");
    }

    #[test]
    fn stale_agent_issue_still_flagged_with_unrelated_draft_pr() {
        let now = parse_time("2026-03-04T12:00:00Z").unwrap();
        let issues = vec![json!({"number": 746, "created_at": "2026-03-04T09:00:00Z"})];
        let draft_prs = vec![DraftPrInfo {
            number: 10,
            branch: "copilot/add-severity-tiers-745".to_string(),
            created_at: parse_time("2026-03-04T10:00:00Z").unwrap(),
        }];

        let findings = find_stale_agent_issues(&issues, &draft_prs, now);

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].identifier, "#746");
    }

    #[test]
    fn stale_agent_issue_requires_whole_number_branch_token_match() {
        let now = parse_time("2026-03-04T12:00:00Z").unwrap();
        let issues = vec![json!({"number": 746, "created_at": "2026-03-04T09:00:00Z"})];
        let draft_prs = vec![DraftPrInfo {
            number: 10,
            branch: "copilot/add-severity-tiers-7460".to_string(),
            created_at: parse_time("2026-03-04T10:00:00Z").unwrap(),
        }];

        let findings = find_stale_agent_issues(&issues, &draft_prs, now);

        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].identifier, "#746");
    }

    #[test]
    fn multiple_stale_issues_only_exclude_matching_linked_prs() {
        let now = parse_time("2026-03-04T12:00:00Z").unwrap();
        let issues = vec![
            json!({"number": 746, "created_at": "2026-03-04T09:00:00Z"}),
            json!({"number": 747, "created_at": "2026-03-04T09:00:00Z"}),
            json!({"number": 748, "created_at": "2026-03-04T09:00:00Z"}),
        ];
        let draft_prs = vec![
            DraftPrInfo {
                number: 10,
                branch: "copilot/add-severity-tiers-746".to_string(),
                created_at: parse_time("2026-03-04T10:00:00Z").unwrap(),
            },
            DraftPrInfo {
                number: 11,
                branch: "copilot/fix-issue-900".to_string(),
                created_at: parse_time("2026-03-04T10:30:00Z").unwrap(),
            },
        ];

        let findings = find_stale_agent_issues(&issues, &draft_prs, now);

        assert_eq!(findings.len(), 2);
        assert_eq!(
            findings.iter().map(|finding| finding.identifier.as_str()).collect::<Vec<_>>(),
            vec!["#747", "#748"]
        );
    }

    #[test]
    fn parse_open_copilot_draft_prs_requires_head_ref_name() {
        let prs = vec![json!({
            "number": 10,
            "isDraft": true,
            "author": { "login": "copilot-swe-agent[bot]" },
            "createdAt": "2026-03-04T10:00:00Z"
        })];

        let error = parse_open_copilot_draft_prs(&prs)
            .err()
            .expect("missing headRefName should fail");

        assert_eq!(error, "copilot draft PR #10 missing headRefName");
    }

    #[test]
    fn orphan_detection_requires_copilot_finished_event() {
        let timeline = json!([
            {"event": "commented"},
            {"event": "copilot_work_finished"}
        ]);
        assert!(has_copilot_work_finished(&timeline));

        let no_match = json!([
            {"event": "commented"},
            {"event": "ready_for_review"}
        ]);
        assert!(!has_copilot_work_finished(&no_match));
    }

    #[test]
    fn copilot_draft_pr_match_uses_bot_login() {
        let copilot_draft = json!({
            "isDraft": true,
            "author": { "login": "copilot-swe-agent[bot]" }
        });
        let wrong_author = json!({
            "isDraft": true,
            "author": { "login": "app/copilot-swe-agent" }
        });
        assert!(is_copilot_draft_pr(&copilot_draft));
        assert!(!is_copilot_draft_pr(&wrong_author));
    }

    #[test]
    fn dead_branches_excludes_master_and_open_pr_branches() {
        let branches = vec![
            "feature/closed".to_string(),
            "feature/open".to_string(),
            "feature/no-pr".to_string(),
        ];
        let prs = vec![
            PrState {
                branch: "feature/closed".to_string(),
                state: "closed".to_string(),
            },
            PrState {
                branch: "feature/open".to_string(),
                state: "open".to_string(),
            },
        ];
        let findings = find_dead_branches(&branches, &prs);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].identifier, "feature/closed");
    }

    #[test]
    fn parse_remote_branches_ignores_master_and_head_pointer() {
        let raw = "  origin/HEAD -> origin/master\n  origin/master\n  origin/feature/a\n";
        let parsed = parse_remote_branches(raw);
        assert_eq!(parsed, vec!["feature/a".to_string()]);
    }
}
