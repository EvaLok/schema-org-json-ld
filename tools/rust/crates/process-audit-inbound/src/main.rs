use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
use serde_json::Value;
use state_schema::{check_version, current_cycle_from_state, read_state_value, StateJson};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const AUDIT_REPO: &str = "EvaLok/schema-org-json-ld-audit";
const AUDIT_AUTHOR: &str = "EvaLok";
const BODY_PREVIEW_LIMIT: usize = 500;
const STALE_CYCLE_THRESHOLD: u64 = 5;

#[derive(Debug, Parser)]
#[command(name = "process-audit-inbound")]
struct Cli {
    /// Path to the repository root
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Output report as JSON
    #[arg(long)]
    json: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AuditRecommendation {
    number: u64,
    title: String,
    created_at: DateTime<Utc>,
    body: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AuditInboundIssue {
    number: u64,
    title: String,
    body: String,
    created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
struct NewRecommendation {
    number: u64,
    title: String,
    created_at: String,
    preview: String,
    status: &'static str,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
struct StaleAcceptedRecommendation {
    audit_number: u64,
    inbound_issue_number: u64,
    inbound_issue_title: String,
    accepted_cycle: u64,
    age_cycles: u64,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct Report {
    new_recommendations: Vec<NewRecommendation>,
    stale_accepted: Vec<StaleAcceptedRecommendation>,
    summary: Summary,
}

#[derive(Debug, Serialize, PartialEq, Eq)]
struct Summary {
    new_count: usize,
    stale_count: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AcceptedAuditReference {
    audit_number: u64,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    validate_repo_root(&cli.repo_root)?;

    let state = read_state(&cli.repo_root)?;
    let current_cycle = current_cycle_from_state(&cli.repo_root)?;
    let recommendations = fetch_audit_recommendations(&cli.repo_root)?;
    let inbound_issues = fetch_open_audit_inbound_issues(&cli.repo_root)?;

    let new_recommendations = filter_new_recommendations(&recommendations, &state.audit_processed)?;
    let stale_accepted =
        detect_stale_accepted(&state.audit_processed, &inbound_issues, current_cycle)?;

    let report = Report {
        summary: Summary {
            new_count: new_recommendations.len(),
            stale_count: stale_accepted.len(),
        },
        new_recommendations,
        stale_accepted,
    };

    if cli.json {
        let json = serde_json::to_string_pretty(&report)
            .map_err(|error| format!("failed to serialize JSON output: {}", error))?;
        println!("{}", json);
    } else {
        print_human_report(&report);
    }

    Ok(())
}

fn read_state(repo_root: &Path) -> Result<StateJson, String> {
    let value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(value)
        .map_err(|error| format!("failed to deserialize docs/state.json: {}", error))?;
    check_version(&state)?;
    Ok(state)
}

fn fetch_audit_recommendations(repo_root: &Path) -> Result<Vec<AuditRecommendation>, String> {
    let api_path = format!(
		"repos/{}/issues?labels=audit-outbound&state=open&creator={}&sort=created&direction=asc&per_page=100",
		AUDIT_REPO, AUDIT_AUTHOR
	);
    let value = gh_json(repo_root, &["api", &api_path])?;
    parse_outbound_recommendations(value)
}

fn fetch_open_audit_inbound_issues(repo_root: &Path) -> Result<Vec<AuditInboundIssue>, String> {
    let value = gh_json(
        repo_root,
        &[
            "issue",
            "list",
            "--repo",
            MAIN_REPO,
            "--label",
            "audit-inbound",
            "--state",
            "open",
            "--limit",
            "1000",
            "--json",
            "number,title,body,createdAt",
        ],
    )?;
    parse_inbound_issues(value)
}

fn parse_outbound_recommendations(value: Value) -> Result<Vec<AuditRecommendation>, String> {
    let issues = value
        .as_array()
        .ok_or_else(|| "unexpected response for audit recommendation query".to_string())?;

    let mut recommendations = Vec::with_capacity(issues.len());
    for issue in issues {
        let number = issue
            .get("number")
            .and_then(Value::as_u64)
            .ok_or_else(|| "audit recommendation missing number".to_string())?;
        let title = issue
            .get("title")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("audit recommendation #{} missing title", number))?
            .to_string();
        let created_at_raw = issue
            .get("created_at")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("audit recommendation #{} missing created_at", number))?;
        let created_at = parse_time(created_at_raw)
            .ok_or_else(|| format!("audit recommendation #{} has invalid created_at", number))?;
        let body = issue
            .get("body")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let author = issue
            .pointer("/user/login")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("audit recommendation #{} missing user.login", number))?;
        if author != AUDIT_AUTHOR {
            return Err(format!(
                "audit recommendation #{} has unexpected author '{}'",
                number, author
            ));
        }

        recommendations.push(AuditRecommendation {
            number,
            title,
            created_at,
            body,
        });
    }

    Ok(recommendations)
}

fn parse_inbound_issues(value: Value) -> Result<Vec<AuditInboundIssue>, String> {
    let issues = value
        .as_array()
        .ok_or_else(|| "unexpected response for audit-inbound issue query".to_string())?;

    let mut inbound_issues = Vec::with_capacity(issues.len());
    for issue in issues {
        let number = issue
            .get("number")
            .and_then(Value::as_u64)
            .ok_or_else(|| "audit-inbound issue missing number".to_string())?;
        let title = issue
            .get("title")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("audit-inbound issue #{} missing title", number))?
            .to_string();
        let body = issue
            .get("body")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let created_at_raw = issue
            .get("createdAt")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("audit-inbound issue #{} missing createdAt", number))?;
        let created_at = parse_time(created_at_raw)
            .ok_or_else(|| format!("audit-inbound issue #{} has invalid createdAt", number))?;

        inbound_issues.push(AuditInboundIssue {
            number,
            title,
            body,
            created_at,
        });
    }

    Ok(inbound_issues)
}

fn filter_new_recommendations(
    recommendations: &[AuditRecommendation],
    processed: &[i64],
) -> Result<Vec<NewRecommendation>, String> {
    let processed_set = build_processed_set(processed)?;
    Ok(recommendations
        .iter()
        .filter(|recommendation| !processed_set.contains(&recommendation.number))
        .map(|recommendation| NewRecommendation {
            number: recommendation.number,
            title: recommendation.title.clone(),
            created_at: format_timestamp(recommendation.created_at),
            preview: build_preview(&recommendation.body),
            status: "NEW",
        })
        .collect())
}

fn detect_stale_accepted(
    processed: &[i64],
    inbound_issues: &[AuditInboundIssue],
    current_cycle: u64,
) -> Result<Vec<StaleAcceptedRecommendation>, String> {
    let processed_set = build_processed_set(processed)?;
    let mut stale = Vec::new();

    for issue in inbound_issues {
        let accepted = extract_accepted_audit_references(issue);
        if accepted.is_empty() {
            continue;
        }

        let accepted_cycle = extract_cycle_number(&format!("{}\n{}", issue.title, issue.body))
            .ok_or_else(|| {
                format!(
					"audit-inbound issue #{} claims accepted recommendations but does not mention a cycle",
					issue.number
				)
            })?;

        for reference in accepted {
            if !processed_set.contains(&reference.audit_number) {
                continue;
            }
            let age_cycles = current_cycle.saturating_sub(accepted_cycle);
            if age_cycles < STALE_CYCLE_THRESHOLD {
                continue;
            }
            stale.push(StaleAcceptedRecommendation {
                audit_number: reference.audit_number,
                inbound_issue_number: issue.number,
                inbound_issue_title: issue.title.clone(),
                accepted_cycle,
                age_cycles,
            });
        }
    }

    stale.sort_by_key(|item| {
        (
            item.accepted_cycle,
            item.audit_number,
            item.inbound_issue_number,
        )
    });
    Ok(stale)
}

fn build_processed_set(processed: &[i64]) -> Result<HashSet<u64>, String> {
    processed
        .iter()
        .map(|number| {
            u64::try_from(*number)
                .map_err(|_| format!("audit_processed contains invalid value {}", number))
        })
        .collect()
}

fn extract_accepted_audit_references(issue: &AuditInboundIssue) -> Vec<AcceptedAuditReference> {
    let sections = split_audit_sections(&issue.body);
    let mut references = Vec::new();

    for (audit_number, section_text) in sections {
        if contains_word(&section_text, "accepted") {
            references.push(AcceptedAuditReference { audit_number });
        }
    }

    if references.is_empty() && contains_word(&issue.body, "accepted") {
        if let Some(audit_number) = extract_audit_number(&issue.body) {
            references.push(AcceptedAuditReference { audit_number });
        }
    }

    references.sort_by_key(|reference| reference.audit_number);
    references.dedup_by_key(|reference| reference.audit_number);
    references
}

fn split_audit_sections(body: &str) -> Vec<(u64, String)> {
    let mut sections = Vec::new();
    let mut current_number: Option<u64> = None;
    let mut current_lines: Vec<String> = Vec::new();

    for line in body.lines() {
        if let Some(number) = extract_audit_heading_number(line) {
            if let Some(previous_number) = current_number.take() {
                sections.push((previous_number, current_lines.join("\n")));
                current_lines.clear();
            }
            current_number = Some(number);
        }

        if current_number.is_some() {
            current_lines.push(line.to_string());
        }
    }

    if let Some(previous_number) = current_number {
        sections.push((previous_number, current_lines.join("\n")));
    }

    if sections.is_empty() {
        if let Some(number) = extract_audit_number(body) {
            sections.push((number, body.to_string()));
        }
    }

    sections
}

fn extract_audit_heading_number(line: &str) -> Option<u64> {
    let trimmed = line.trim();
    if !trimmed.starts_with('#') {
        return None;
    }
    extract_audit_number(trimmed)
}

fn extract_audit_number(text: &str) -> Option<u64> {
    extract_number_after_keyword(text, "audit")
        .or_else(|| extract_number_after_marker(text, "schema-org-json-ld-audit/issues/"))
}

fn extract_cycle_number(text: &str) -> Option<u64> {
    extract_number_after_keyword(text, "cycle")
}

fn extract_number_after_keyword(text: &str, keyword: &str) -> Option<u64> {
    let lower = text.to_ascii_lowercase();
    let bytes = lower.as_bytes();
    let keyword_bytes = keyword.as_bytes();
    let mut index = 0;

    while index + keyword_bytes.len() <= bytes.len() {
        if &bytes[index..index + keyword_bytes.len()] != keyword_bytes {
            index += 1;
            continue;
        }

        if index > 0 && bytes[index - 1].is_ascii_alphanumeric() {
            index += 1;
            continue;
        }

        let mut cursor = index + keyword_bytes.len();
        while cursor < bytes.len() && (bytes[cursor].is_ascii_whitespace() || bytes[cursor] == b'#')
        {
            cursor += 1;
        }

        let start = cursor;
        while cursor < bytes.len() && bytes[cursor].is_ascii_digit() {
            cursor += 1;
        }

        if cursor > start {
            return lower[start..cursor].parse::<u64>().ok();
        }

        index += 1;
    }

    None
}

fn extract_number_after_marker(text: &str, marker: &str) -> Option<u64> {
    let lower = text.to_ascii_lowercase();
    let marker = marker.to_ascii_lowercase();
    let start = lower.find(&marker)? + marker.len();
    let digits: String = lower[start..]
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse::<u64>().ok()
    }
}

fn contains_word(text: &str, needle: &str) -> bool {
    text.to_ascii_lowercase()
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|word| word == needle)
}

fn build_preview(body: &str) -> String {
    let collapsed = body.split_whitespace().collect::<Vec<_>>().join(" ");
    let preview: String = collapsed.chars().take(BODY_PREVIEW_LIMIT).collect();
    if collapsed.chars().count() > BODY_PREVIEW_LIMIT {
        format!("{}...", preview)
    } else {
        preview
    }
}

fn print_human_report(report: &Report) {
    println!("Audit recommendations:");
    if report.new_recommendations.is_empty() {
        println!("  - None");
    } else {
        for recommendation in &report.new_recommendations {
            println!(
                "  {}: audit#{} — {}",
                recommendation.status, recommendation.number, recommendation.title
            );
            println!("    Created: {}", recommendation.created_at);
            println!("    Preview: {}", recommendation.preview);
        }
    }

    println!();
    println!("Stale accepted ({}+ cycles):", STALE_CYCLE_THRESHOLD);
    if report.stale_accepted.is_empty() {
        println!("  - None");
    } else {
        for stale in &report.stale_accepted {
            println!(
                "  - audit#{} via issue #{} — accepted in cycle {} ({} cycles old): {}",
                stale.audit_number,
                stale.inbound_issue_number,
                stale.accepted_cycle,
                stale.age_cycles,
                stale.inbound_issue_title
            );
        }
    }

    println!();
    println!(
        "Summary: {} new, {} stale",
        report.summary.new_count, report.summary.stale_count
    );
}

fn format_timestamp(timestamp: DateTime<Utc>) -> String {
    timestamp.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn parse_time(raw: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
}

fn gh_json(repo_root: &Path, args: &[&str]) -> Result<Value, String> {
    let output = Command::new("gh")
        .current_dir(repo_root)
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute gh {}: {}", args.join(" "), error))?;

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

    serde_json::from_slice(&output.stdout).map_err(|error| {
        format!(
            "failed to parse JSON output from `gh {}`: {}",
            args.join(" "),
            error
        )
    })
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use serde_json::json;

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--repo-root"));
        assert!(help.contains("--json"));
    }

    #[test]
    fn filters_out_processed_recommendations() {
        let recommendations = vec![
            audit_recommendation(
                160,
                "post-step tool merged but not used",
                "2026-03-09T03:14:56Z",
                "The post-step tool should be used every cycle.",
            ),
            audit_recommendation(
                161,
                "new audit item",
                "2026-03-09T03:20:00Z",
                "Body for a new item",
            ),
        ];

        let report =
            filter_new_recommendations(&recommendations, &[160]).expect("filter should succeed");

        assert_eq!(
            report,
            vec![NewRecommendation {
                number: 161,
                title: "new audit item".to_string(),
                created_at: "2026-03-09T03:20:00Z".to_string(),
                preview: "Body for a new item".to_string(),
                status: "NEW",
            }]
        );
    }

    #[test]
    fn stale_detection_only_flags_open_accepted_items_past_threshold() {
        let inbound_issues = vec![
			audit_inbound_issue(
				900,
				"[Audit-ACK] Accepted: add validation",
				"2026-03-09T01:00:00Z",
				"> **[main-orchestrator]** | Cycle 194\n\nResponding to https://github.com/EvaLok/schema-org-json-ld-audit/issues/160\n\n## Accepted\n\nWill dispatch a fix.",
			),
			audit_inbound_issue(
				901,
				"[Audit-ACK] Deferred",
				"2026-03-09T01:00:00Z",
				"> **[main-orchestrator]** | Cycle 194\n\nResponding to https://github.com/EvaLok/schema-org-json-ld-audit/issues/161\n\n## Deferred\n\nNot this cycle.",
			),
			audit_inbound_issue(
				902,
				"[Audit-ACK] Accepted recent",
				"2026-03-09T01:00:00Z",
				"> **[main-orchestrator]** | Cycle 198\n\nResponding to https://github.com/EvaLok/schema-org-json-ld-audit/issues/162\n\n## Accepted\n\nStill recent.",
			),
		];

        let stale =
            detect_stale_accepted(&[160, 161, 162], &inbound_issues, 200).expect("stale scan");

        assert_eq!(
            stale,
            vec![StaleAcceptedRecommendation {
                audit_number: 160,
                inbound_issue_number: 900,
                inbound_issue_title: "[Audit-ACK] Accepted: add validation".to_string(),
                accepted_cycle: 194,
                age_cycles: 6,
            }]
        );
    }

    #[test]
    fn stale_detection_handles_multi_section_ack_issue() {
        let inbound_issues = vec![audit_inbound_issue(
			246,
			"[AUDIT-ACK] Implemented idle cycle detection, tools cleanup, validator docs, cron question",
			"2026-02-28T12:09:53Z",
			"> **[main-orchestrator]** | Cycle 198\n\n## Audit #2: Idle cycle detection\n\n**Accepted and implemented.** Added step 2.5.\n\n## Audit #3: tools/ directory cleanup\n\n**Accepted — removed entirely.** Removed old scripts.\n\n## Audit #5: Cron frequency reduction\n\n**Deferred to Eva.** Created #245.",
		)];

        let stale = detect_stale_accepted(&[2, 3, 5], &inbound_issues, 200).expect("stale scan");

        assert!(stale.is_empty());
        assert_eq!(
            extract_accepted_audit_references(&inbound_issues[0]),
            vec![
                AcceptedAuditReference { audit_number: 2 },
                AcceptedAuditReference { audit_number: 3 },
            ]
        );
    }

    #[test]
    fn stale_detection_errors_when_accepted_issue_has_no_cycle_marker() {
        let inbound_issues = vec![audit_inbound_issue(
			903,
			"[Audit-ACK] Accepted without cycle",
			"2026-03-09T01:00:00Z",
			"Responding to https://github.com/EvaLok/schema-org-json-ld-audit/issues/170\n\n## Accepted\n\nWill fix later.",
		)];

        let error = detect_stale_accepted(&[170], &inbound_issues, 200)
            .expect_err("missing cycle should fail closed");

        assert!(error.contains("does not mention a cycle"));
    }

    #[test]
    fn outbound_parser_rejects_unexpected_author() {
        let value = json!([
            {
                "number": 160,
                "title": "Bad author",
                "created_at": "2026-03-09T03:14:56Z",
                "body": "Body",
                "user": { "login": "someone-else" }
            }
        ]);

        let error = parse_outbound_recommendations(value).expect_err("author mismatch should fail");

        assert!(error.contains("unexpected author"));
    }

    fn audit_recommendation(
        number: u64,
        title: &str,
        created_at: &str,
        body: &str,
    ) -> AuditRecommendation {
        AuditRecommendation {
            number,
            title: title.to_string(),
            created_at: parse_utc(created_at),
            body: body.to_string(),
        }
    }

    fn audit_inbound_issue(
        number: u64,
        title: &str,
        created_at: &str,
        body: &str,
    ) -> AuditInboundIssue {
        AuditInboundIssue {
            number,
            title: title.to_string(),
            body: body.to_string(),
            created_at: parse_utc(created_at),
        }
    }

    fn parse_utc(raw: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(raw)
            .expect("timestamp should parse")
            .with_timezone(&Utc)
    }
}
