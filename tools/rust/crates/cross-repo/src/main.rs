use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use state_schema::{
    check_version, current_cycle_from_state, read_state_value, PublishGate, StateJson,
};
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use std::process::Command;

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const QC_REPO: &str = "EvaLok/schema-org-json-ld-qc";
const AUDIT_REPO: &str = "EvaLok/schema-org-json-ld-audit";
const TRUSTED_AUTHOR: &str = "EvaLok";
const BODY_PREVIEW_LIMIT: usize = 200;
const STALE_CYCLE_THRESHOLD: u64 = 5;
const NO_STDERR_PLACEHOLDER: &str = "<no stderr>";
const PACKAGE_AFFECTING_PATHS: &[&str] = &[
    "php/src/",
    "php/test/",
    "ts/src/",
    "ts/test/",
    "package.json",
    "tsconfig.json",
    "scripts/verify-build.mjs",
];

#[derive(Debug, Parser)]
#[command(name = "cross-repo")]
struct Cli {
    /// Path to the repository root
    #[arg(long, global = true, default_value = ".")]
    repo_root: PathBuf,

    /// Output report as JSON
    #[arg(long, global = true)]
    json: bool,

    #[command(subcommand)]
    command: CrossRepoCommand,
}

#[derive(Debug, Subcommand)]
enum CrossRepoCommand {
    /// List new QC reports from the QC repo
    ProcessQc,
    /// Detect new QC acknowledgements for pending QC requests
    CheckQcAck,
    /// List new audit recommendations and stale accepted items
    ProcessAudit,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ExecutionResult {
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
}

trait CommandRunner {
    fn git(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String>;
    fn gh(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String>;
}

struct ProcessRunner;

impl CommandRunner for ProcessRunner {
    fn git(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
        let output = Command::new("git")
            .current_dir(repo_root)
            .args(args)
            .output()
            .map_err(|error| format!("failed to execute git {}: {}", args.join(" "), error))?;
        Ok(ExecutionResult {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    fn gh(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
        let output = Command::new("gh")
            .current_dir(repo_root)
            .args(args)
            .output()
            .map_err(|error| format!("failed to execute gh {}: {}", args.join(" "), error))?;
        Ok(ExecutionResult {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct SourceIssue {
    number: u64,
    title: String,
    created_at: String,
    body: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct QcAckIssue {
    number: u64,
    title: String,
    body: String,
    closed_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AuditInboundIssue {
    number: u64,
    title: String,
    body: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ParsedIssues<T> {
    trusted: Vec<T>,
    skipped_untrusted: usize,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct PendingIssue {
    number: u64,
    title: String,
    created_at: String,
    preview: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct StaleAcceptedRecommendation {
    audit_number: u64,
    inbound_issue_number: u64,
    inbound_issue_title: String,
    accepted_cycle: u64,
    age_cycles: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct DivergenceStatus {
    source_diverged: bool,
    check_failed: bool,
    changed_files: Vec<String>,
    error: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct QcAckMatch {
    ack_number: u64,
    title: String,
    closed_at: String,
    source_request_number: u64,
    qc_ack_reference: String,
    validated_commit: Option<String>,
    preview: String,
    divergence: DivergenceStatus,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct ProcessQcReport {
    reports: Vec<PendingIssue>,
    summary: ProcessingSummary,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct ProcessAuditReport {
    recommendations: Vec<PendingIssue>,
    stale_accepted: Vec<StaleAcceptedRecommendation>,
    summary: AuditSummary,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct CheckQcAckReport {
    acknowledgements: Vec<QcAckMatch>,
    summary: AckSummary,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct ProcessingSummary {
    new_count: usize,
    skipped_untrusted_count: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct AuditSummary {
    new_count: usize,
    stale_count: usize,
    skipped_untrusted_outbound_count: usize,
    skipped_untrusted_inbound_count: usize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct AckSummary {
    new_count: usize,
    skipped_untrusted_count: usize,
    pending_request_count: usize,
}

fn main() {
    let cli = Cli::parse();
    let runner = ProcessRunner;

    match execute(&cli, &runner) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn execute(cli: &Cli, runner: &dyn CommandRunner) -> Result<String, String> {
    validate_repo_root(&cli.repo_root)?;
    let state = read_state(&cli.repo_root)?;

    match cli.command {
        CrossRepoCommand::ProcessQc => {
            let report = process_qc(&cli.repo_root, &state, runner)?;
            render_output(&report, cli.json, print_qc_report)
        }
        CrossRepoCommand::CheckQcAck => {
            let report = check_qc_ack(&cli.repo_root, &state, runner)?;
            render_output(&report, cli.json, print_qc_ack_report)
        }
        CrossRepoCommand::ProcessAudit => {
            let report = process_audit(&cli.repo_root, &state, runner)?;
            render_output(&report, cli.json, print_audit_report)
        }
    }
}

fn render_output<T>(
    report: &T,
    json_output: bool,
    formatter: fn(&T) -> String,
) -> Result<String, String>
where
    T: Serialize,
{
    if json_output {
        serde_json::to_string_pretty(report)
            .map_err(|error| format!("failed to serialize JSON output: {}", error))
    } else {
        Ok(formatter(report))
    }
}

fn process_qc(
    repo_root: &Path,
    state: &StateJson,
    runner: &dyn CommandRunner,
) -> Result<ProcessQcReport, String> {
    let issues = fetch_open_outbound_issues(repo_root, runner, QC_REPO, "qc-outbound")?;
    let reports = filter_new_issues(&issues.trusted, &state.qc_processed, "qc_processed")?;
    Ok(ProcessQcReport {
        summary: ProcessingSummary {
            new_count: reports.len(),
            skipped_untrusted_count: issues.skipped_untrusted,
        },
        reports,
    })
}

fn check_qc_ack(
    repo_root: &Path,
    state: &StateJson,
    runner: &dyn CommandRunner,
) -> Result<CheckQcAckReport, String> {
    let issues = fetch_closed_qc_ack_issues(repo_root, runner)?;
    let publish_gate = state
        .publish_gate()
        .unwrap_or_else(|_| PublishGate::default());
    let pending_requests = build_pending_request_set(&state.qc_requests_pending)?;
    let mut acknowledgements = Vec::new();

    for issue in issues.trusted {
        let source_request_number = match extract_main_issue_number(&issue.body) {
            Some(number) => number,
            None => continue,
        };
        if !pending_requests.contains(&source_request_number) {
            continue;
        }

        let qc_ack_reference = format!("{}#{}", QC_REPO, issue.number);
        if publish_gate.qc_ack.as_deref() == Some(qc_ack_reference.as_str()) {
            continue;
        }

        let validated_commit =
            extract_commit_sha(&issue.title).or_else(|| extract_commit_sha(&issue.body));
        let divergence =
            check_package_source_divergence(repo_root, runner, validated_commit.as_deref());

        acknowledgements.push(QcAckMatch {
            ack_number: issue.number,
            title: issue.title,
            closed_at: issue.closed_at,
            source_request_number,
            qc_ack_reference,
            validated_commit,
            preview: build_preview(&issue.body),
            divergence,
        });
    }

    acknowledgements.sort_by_key(|issue| issue.ack_number);
    Ok(CheckQcAckReport {
        summary: AckSummary {
            new_count: acknowledgements.len(),
            skipped_untrusted_count: issues.skipped_untrusted,
            pending_request_count: pending_requests.len(),
        },
        acknowledgements,
    })
}

fn process_audit(
    repo_root: &Path,
    state: &StateJson,
    runner: &dyn CommandRunner,
) -> Result<ProcessAuditReport, String> {
    let current_cycle = current_cycle_from_state(repo_root).map_err(|error| {
        if error == "missing /last_cycle/number in state.json" {
            "missing numeric /last_cycle/number in docs/state.json".to_string()
        } else {
            error
        }
    })?;
    let recommendations =
        fetch_open_outbound_issues(repo_root, runner, AUDIT_REPO, "audit-outbound")?;
    let inbound_issues = fetch_open_audit_inbound_issues(repo_root, runner)?;
    let new_recommendations = filter_new_issues(
        &recommendations.trusted,
        &state.audit_processed,
        "audit_processed",
    )?;
    let stale_accepted = detect_stale_accepted(
        &state.audit_processed,
        &inbound_issues.trusted,
        current_cycle,
    )?;

    Ok(ProcessAuditReport {
        summary: AuditSummary {
            new_count: new_recommendations.len(),
            stale_count: stale_accepted.len(),
            skipped_untrusted_outbound_count: recommendations.skipped_untrusted,
            skipped_untrusted_inbound_count: inbound_issues.skipped_untrusted,
        },
        recommendations: new_recommendations,
        stale_accepted,
    })
}

fn read_state(repo_root: &Path) -> Result<StateJson, String> {
    let value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(value)
        .map_err(|error| format!("failed to deserialize docs/state.json: {}", error))?;
    check_version(&state)?;
    Ok(state)
}

fn fetch_open_outbound_issues(
    repo_root: &Path,
    runner: &dyn CommandRunner,
    repo: &str,
    label: &str,
) -> Result<ParsedIssues<SourceIssue>, String> {
    let api_path = format!(
        "repos/{}/issues?labels={}&state=open&sort=created&direction=asc&per_page=100",
        repo, label
    );
    let value = gh_json(
        repo_root,
        runner,
        &[
            "api".to_string(),
            api_path,
            "--paginate".to_string(),
            "--slurp".to_string(),
        ],
    )?;
    parse_outbound_issues(value, label)
}

fn fetch_closed_qc_ack_issues(
    repo_root: &Path,
    runner: &dyn CommandRunner,
) -> Result<ParsedIssues<QcAckIssue>, String> {
    let api_path = format!(
        "repos/{}/issues?labels=qc-inbound&state=closed&sort=updated&direction=desc&per_page=20",
        QC_REPO
    );
    let value = gh_json(
        repo_root,
        runner,
        &[
            "api".to_string(),
            api_path,
            "--paginate".to_string(),
            "--slurp".to_string(),
        ],
    )?;
    parse_closed_qc_ack_issues(value)
}

fn fetch_open_audit_inbound_issues(
    repo_root: &Path,
    runner: &dyn CommandRunner,
) -> Result<ParsedIssues<AuditInboundIssue>, String> {
    let api_path = format!(
        "repos/{}/issues?labels=audit-inbound&state=open&sort=created&direction=asc&per_page=100",
        MAIN_REPO
    );
    let value = gh_json(
        repo_root,
        runner,
        &[
            "api".to_string(),
            api_path,
            "--paginate".to_string(),
            "--slurp".to_string(),
        ],
    )?;
    parse_audit_inbound_issues(value)
}

fn parse_outbound_issues(value: Value, label: &str) -> Result<ParsedIssues<SourceIssue>, String> {
    let issues = flatten_paginated_items(value)?;
    let mut trusted = Vec::new();
    let mut skipped_untrusted = 0;

    for issue in issues {
        let number = issue
            .get("number")
            .and_then(Value::as_u64)
            .ok_or_else(|| format!("{} issue missing number", label))?;
        let title = issue
            .get("title")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{} issue #{} missing title", label, number))?
            .to_string();
        let created_at = issue
            .get("created_at")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{} issue #{} missing created_at", label, number))?
            .to_string();
        let body = issue
            .get("body")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let author = issue
            .pointer("/user/login")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("{} issue #{} missing user.login", label, number))?;
        if author != TRUSTED_AUTHOR {
            skipped_untrusted += 1;
            continue;
        }

        trusted.push(SourceIssue {
            number,
            title,
            created_at,
            body,
        });
    }

    Ok(ParsedIssues {
        trusted,
        skipped_untrusted,
    })
}

fn parse_closed_qc_ack_issues(value: Value) -> Result<ParsedIssues<QcAckIssue>, String> {
    let issues = flatten_paginated_items(value)?;
    let mut trusted = Vec::new();
    let mut skipped_untrusted = 0;

    for issue in issues {
        let number = issue
            .get("number")
            .and_then(Value::as_u64)
            .ok_or_else(|| "qc-inbound issue missing number".to_string())?;
        let title = issue
            .get("title")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("qc-inbound issue #{} missing title", number))?
            .to_string();
        let body = issue
            .get("body")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let closed_at = issue
            .get("closed_at")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("qc-inbound issue #{} missing closed_at", number))?
            .to_string();
        let author = issue
            .pointer("/user/login")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("qc-inbound issue #{} missing user.login", number))?;
        if author != TRUSTED_AUTHOR {
            skipped_untrusted += 1;
            continue;
        }

        trusted.push(QcAckIssue {
            number,
            title,
            body,
            closed_at,
        });
    }

    Ok(ParsedIssues {
        trusted,
        skipped_untrusted,
    })
}

fn parse_audit_inbound_issues(value: Value) -> Result<ParsedIssues<AuditInboundIssue>, String> {
    let issues = flatten_paginated_items(value)?;
    let mut trusted = Vec::new();
    let mut skipped_untrusted = 0;

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
        let author = issue
            .pointer("/user/login")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("audit-inbound issue #{} missing user.login", number))?;
        if author != TRUSTED_AUTHOR {
            skipped_untrusted += 1;
            continue;
        }

        trusted.push(AuditInboundIssue {
            number,
            title,
            body,
        });
    }

    Ok(ParsedIssues {
        trusted,
        skipped_untrusted,
    })
}

fn filter_new_issues(
    issues: &[SourceIssue],
    processed: &[i64],
    field_name: &str,
) -> Result<Vec<PendingIssue>, String> {
    let processed_set = build_processed_set(processed, field_name)?;
    Ok(issues
        .iter()
        .filter(|issue| !processed_set.contains(&issue.number))
        .map(|issue| PendingIssue {
            number: issue.number,
            title: issue.title.clone(),
            created_at: issue.created_at.clone(),
            preview: build_preview(&issue.body),
        })
        .collect())
}

fn detect_stale_accepted(
    processed: &[i64],
    inbound_issues: &[AuditInboundIssue],
    current_cycle: u64,
) -> Result<Vec<StaleAcceptedRecommendation>, String> {
    let processed_set = build_processed_set(processed, "audit_processed")?;
    let mut stale = Vec::new();

    for issue in inbound_issues {
        let accepted_references = extract_accepted_audit_references(issue);
        if accepted_references.is_empty() {
            continue;
        }

        let accepted_cycle = extract_cycle_number(&issue.title)
            .or_else(|| extract_cycle_number(&issue.body))
            .ok_or_else(|| {
                format!(
                    "audit-inbound issue #{} claims accepted recommendations but does not mention a cycle number in title or body",
                    issue.number
                )
            })?;

        for audit_number in accepted_references {
            if !processed_set.contains(&audit_number) {
                continue;
            }

            let age_cycles = current_cycle.saturating_sub(accepted_cycle);
            if age_cycles < STALE_CYCLE_THRESHOLD {
                continue;
            }

            stale.push(StaleAcceptedRecommendation {
                audit_number,
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

fn extract_accepted_audit_references(issue: &AuditInboundIssue) -> Vec<u64> {
    let sections = split_audit_sections(&issue.body);
    let mut references = Vec::new();

    for (audit_number, section_text) in sections {
        if contains_word(&section_text, "accepted") {
            references.push(audit_number);
        }
    }

    if references.is_empty() && contains_word(&issue.body, "accepted") {
        if let Some(audit_number) = extract_audit_number(&issue.body) {
            references.push(audit_number);
        }
    }

    references.sort_unstable();
    references.dedup();
    references
}

fn split_audit_sections(body: &str) -> Vec<(u64, String)> {
    let mut sections = Vec::new();
    let mut current_number: Option<u64> = None;
    let mut current_lines = Vec::new();

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
    extract_number_after_keyword(text, "audit", true)
        .or_else(|| extract_number_after_marker(text, "schema-org-json-ld-audit/issues/"))
}

fn extract_cycle_number(text: &str) -> Option<u64> {
    extract_number_after_keyword(text, "cycle", false)
}

fn extract_main_issue_number(text: &str) -> Option<u64> {
    extract_number_after_marker(
        text,
        &format!("https://github.com/{}/issues/", MAIN_REPO).to_ascii_lowercase(),
    )
    .or_else(|| extract_number_after_marker(text, "schema-org-json-ld/issues/"))
}

fn extract_commit_sha(text: &str) -> Option<String> {
    extract_hex_after_keyword(text, "commit")
}

fn extract_number_after_keyword(text: &str, keyword: &str, allow_hash_prefix: bool) -> Option<u64> {
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
        while cursor < bytes.len()
            && (bytes[cursor].is_ascii_whitespace()
                || bytes[cursor] == b':'
                || bytes[cursor] == b'`'
                || (allow_hash_prefix && bytes[cursor] == b'#'))
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

fn extract_hex_after_keyword(text: &str, keyword: &str) -> Option<String> {
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
        while cursor < bytes.len()
            && (bytes[cursor].is_ascii_whitespace()
                || bytes[cursor] == b':'
                || bytes[cursor] == b'`'
                || bytes[cursor] == b'#')
        {
            cursor += 1;
        }

        let start = cursor;
        while cursor < bytes.len() && bytes[cursor].is_ascii_hexdigit() {
            cursor += 1;
        }

        if cursor > start {
            let candidate = &lower[start..cursor];
            if is_valid_commit_sha(candidate) {
                return Some(candidate.to_string());
            }
        }

        index += 1;
    }

    None
}

fn extract_number_after_marker(text: &str, marker: &str) -> Option<u64> {
    let lower = text.to_ascii_lowercase();
    let start = lower.find(marker)? + marker.len();
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
    let needle = needle.to_ascii_lowercase();
    text.to_ascii_lowercase()
        .split(|character: char| !character.is_ascii_alphanumeric())
        .any(|word| word == needle)
}

fn build_processed_set(values: &[i64], field_name: &str) -> Result<HashSet<u64>, String> {
    values
        .iter()
        .map(|number| {
            u64::try_from(*number)
                .map_err(|_| format!("{} contains invalid value {}", field_name, number))
        })
        .collect()
}

fn build_pending_request_set(values: &[Value]) -> Result<HashSet<u64>, String> {
    values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            if let Some(number) = value.as_u64() {
                return Ok(number);
            }
            if let Some(number) = value.get("number").and_then(Value::as_u64) {
                return Ok(number);
            }
            if let Some(number) = value.as_str().and_then(|raw| raw.parse::<u64>().ok()) {
                return Ok(number);
            }
            Err(format!(
                "qc_requests_pending[{}] has unsupported shape: {}",
                index, value
            ))
        })
        .collect()
}

fn check_package_source_divergence(
    repo_root: &Path,
    runner: &dyn CommandRunner,
    validated_commit: Option<&str>,
) -> DivergenceStatus {
    let validated_commit = match validated_commit {
        Some(value) => value,
        None => {
            return DivergenceStatus {
                source_diverged: true,
                check_failed: true,
                changed_files: Vec::new(),
                error: Some("validated commit not found in QC acknowledgement".to_string()),
            };
        }
    };

    if !is_valid_commit_sha(validated_commit) {
        return DivergenceStatus {
            source_diverged: true,
            check_failed: true,
            changed_files: Vec::new(),
            error: Some(format!(
                "validated commit {:?} is not a valid commit SHA (expected 4-40 hex characters)",
                validated_commit
            )),
        };
    }

    let cat_file_args = vec![
        "cat-file".to_string(),
        "-t".to_string(),
        validated_commit.to_string(),
    ];
    let cat_file = match runner.git(repo_root, &cat_file_args) {
        Ok(output) => output,
        Err(error) => {
            return DivergenceStatus {
                source_diverged: true,
                check_failed: true,
                changed_files: Vec::new(),
                error: Some(format!(
                    "unable to execute git cat-file for {}: {}",
                    validated_commit, error
                )),
            };
        }
    };
    if cat_file.exit_code != Some(0) {
        return DivergenceStatus {
            source_diverged: true,
            check_failed: true,
            changed_files: Vec::new(),
            error: Some(format!(
                "validated commit {} is not reachable in this repository: {}",
                validated_commit,
                trimmed_or_default(&cat_file.stderr)
            )),
        };
    }
    if cat_file.stdout.trim() != "commit" {
        return DivergenceStatus {
            source_diverged: true,
            check_failed: true,
            changed_files: Vec::new(),
            error: Some(format!(
                "validated commit {} is not a commit object",
                validated_commit
            )),
        };
    }

    let mut diff_args = vec![
        "diff".to_string(),
        "--name-only".to_string(),
        format!("{}..HEAD", validated_commit),
        "--".to_string(),
    ];
    diff_args.extend(PACKAGE_AFFECTING_PATHS.iter().map(|path| path.to_string()));
    let diff_output = match runner.git(repo_root, &diff_args) {
        Ok(output) => output,
        Err(error) => {
            return DivergenceStatus {
                source_diverged: true,
                check_failed: true,
                changed_files: Vec::new(),
                error: Some(format!(
                    "unable to execute git diff for {}..HEAD: {}",
                    validated_commit, error
                )),
            };
        }
    };
    if diff_output.exit_code != Some(0) {
        return DivergenceStatus {
            source_diverged: true,
            check_failed: true,
            changed_files: Vec::new(),
            error: Some(format!(
                "git diff --name-only {}..HEAD failed: {}",
                validated_commit,
                trimmed_or_default(&diff_output.stderr)
            )),
        };
    }

    let changed_files = diff_output
        .stdout
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<_>>();

    DivergenceStatus {
        source_diverged: !changed_files.is_empty(),
        check_failed: false,
        changed_files,
        error: None,
    }
}

fn is_valid_commit_sha(value: &str) -> bool {
    let len = value.len();
    (4..=40).contains(&len) && value.chars().all(|character| character.is_ascii_hexdigit())
}

fn gh_json(repo_root: &Path, runner: &dyn CommandRunner, args: &[String]) -> Result<Value, String> {
    let output = runner.gh(repo_root, args)?;
    if output.exit_code != Some(0) {
        return Err(format!(
            "`gh {}` failed with status {}: {}",
            args.join(" "),
            output
                .exit_code
                .map(|code| code.to_string())
                .unwrap_or_else(|| "terminated by signal".to_string()),
            trimmed_or_default(&output.stderr)
        ));
    }

    serde_json::from_str(&output.stdout).map_err(|error| {
        format!(
            "failed to parse JSON output from `gh {}`: {}",
            args.join(" "),
            error
        )
    })
}

fn trimmed_or_default(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        NO_STDERR_PLACEHOLDER.to_string()
    } else {
        trimmed.to_string()
    }
}

fn flatten_paginated_items(value: Value) -> Result<Vec<Value>, String> {
    match value {
        Value::Array(items) => {
            if items.iter().all(Value::is_object) {
                return Ok(items);
            }
            if items.iter().all(Value::is_array) {
                return Ok(items
                    .into_iter()
                    .flat_map(|page| page.as_array().cloned().unwrap_or_default())
                    .collect());
            }
            Err("unexpected paginated response shape".to_string())
        }
        _ => Err("expected array response from GitHub CLI".to_string()),
    }
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

fn print_qc_report(report: &ProcessQcReport) -> String {
    let mut lines = vec!["New QC reports:".to_string()];
    if report.reports.is_empty() {
        lines.push("  - None".to_string());
    } else {
        for issue in &report.reports {
            lines.push(format!("  - qc#{} — {}", issue.number, issue.title));
            lines.push(format!("    Created: {}", issue.created_at));
            lines.push(format!("    Preview: {}", issue.preview));
        }
    }
    lines.push(String::new());
    lines.push(format!(
        "Summary: {} new, {} skipped due to untrusted author",
        report.summary.new_count, report.summary.skipped_untrusted_count
    ));
    lines.join("\n")
}

fn print_qc_ack_report(report: &CheckQcAckReport) -> String {
    let mut lines = vec!["New QC acknowledgements:".to_string()];
    if report.acknowledgements.is_empty() {
        lines.push("  - None".to_string());
    } else {
        for ack in &report.acknowledgements {
            lines.push(format!(
                "  - QC-ACK #{} for request #{} — {}",
                ack.ack_number, ack.source_request_number, ack.title
            ));
            lines.push(format!("    Closed: {}", ack.closed_at));
            lines.push(format!("    Reference: {}", ack.qc_ack_reference));
            lines.push(format!(
                "    Validated commit: {}",
                ack.validated_commit.as_deref().unwrap_or("<missing>")
            ));
            if ack.divergence.check_failed {
                lines.push(format!(
                    "    Source divergence: check failed ({})",
                    ack.divergence.error.as_deref().unwrap_or("unknown error")
                ));
            } else if ack.divergence.source_diverged {
                lines.push("    Source divergence: yes".to_string());
                for file in &ack.divergence.changed_files {
                    lines.push(format!("      - {}", file));
                }
            } else {
                lines.push("    Source divergence: no".to_string());
            }
            lines.push(format!("    Preview: {}", ack.preview));
        }
    }
    lines.push(String::new());
    lines.push(format!(
        "Summary: {} new, {} pending QC requests, {} skipped due to untrusted author",
        report.summary.new_count,
        report.summary.pending_request_count,
        report.summary.skipped_untrusted_count
    ));
    lines.join("\n")
}

fn print_audit_report(report: &ProcessAuditReport) -> String {
    let mut lines = vec!["New audit recommendations:".to_string()];
    if report.recommendations.is_empty() {
        lines.push("  - None".to_string());
    } else {
        for issue in &report.recommendations {
            lines.push(format!("  - audit#{} — {}", issue.number, issue.title));
            lines.push(format!("    Created: {}", issue.created_at));
            lines.push(format!("    Preview: {}", issue.preview));
        }
    }

    lines.push(String::new());
    lines.push(format!(
        "Stale accepted recommendations ({}+ cycles):",
        STALE_CYCLE_THRESHOLD
    ));
    if report.stale_accepted.is_empty() {
        lines.push("  - None".to_string());
    } else {
        for stale in &report.stale_accepted {
            lines.push(format!(
                "  - audit#{} via issue #{} — accepted in cycle {} ({} cycles old): {}",
                stale.audit_number,
                stale.inbound_issue_number,
                stale.accepted_cycle,
                stale.age_cycles,
                stale.inbound_issue_title
            ));
        }
    }

    lines.push(String::new());
    lines.push(format!(
        "Summary: {} new, {} stale, {} outbound skipped, {} inbound skipped",
        report.summary.new_count,
        report.summary.stale_count,
        report.summary.skipped_untrusted_outbound_count,
        report.summary.skipped_untrusted_inbound_count
    ));
    lines.join("\n")
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
    use state_schema::write_state_value;
    use std::collections::VecDeque;
    use std::env;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Mutex;

    #[derive(Default)]
    struct MockRunner {
        git_results: Mutex<VecDeque<Result<ExecutionResult, String>>>,
        gh_results: Mutex<VecDeque<Result<ExecutionResult, String>>>,
        git_calls: Mutex<Vec<Vec<String>>>,
        gh_calls: Mutex<Vec<Vec<String>>>,
    }

    impl MockRunner {
        fn with_results(
            git_results: Vec<Result<ExecutionResult, String>>,
            gh_results: Vec<Result<ExecutionResult, String>>,
        ) -> Self {
            Self {
                git_results: Mutex::new(VecDeque::from(git_results)),
                gh_results: Mutex::new(VecDeque::from(gh_results)),
                ..Self::default()
            }
        }

        fn git_calls(&self) -> Vec<Vec<String>> {
            self.git_calls.lock().unwrap().clone()
        }

        fn gh_calls(&self) -> Vec<Vec<String>> {
            self.gh_calls.lock().unwrap().clone()
        }
    }

    impl CommandRunner for MockRunner {
        fn git(&self, _repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
            self.git_calls.lock().unwrap().push(args.to_vec());
            self.git_results
                .lock()
                .unwrap()
                .pop_front()
                .unwrap_or_else(|| Err(format!("unexpected git call: {:?}", args)))
        }

        fn gh(&self, _repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
            self.gh_calls.lock().unwrap().push(args.to_vec());
            self.gh_results
                .lock()
                .unwrap()
                .pop_front()
                .unwrap_or_else(|| Err(format!("unexpected gh call: {:?}", args)))
        }
    }

    struct TempRepo {
        path: PathBuf,
    }

    impl TempRepo {
        fn new(state: &Value) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = env::temp_dir().join(format!("cross-repo-test-{}", run_id));
            fs::create_dir_all(path.join("docs")).expect("temp repo should be created");
            write_state_value(&path, state).expect("state should be written");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn help_contains_expected_flags_and_subcommands() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--repo-root"));
        assert!(help.contains("--json"));
        assert!(help.contains("process-qc"));
        assert!(help.contains("check-qc-ack"));
        assert!(help.contains("process-audit"));
    }

    #[test]
    fn process_qc_json_filters_processed_and_skips_untrusted_authors() {
        let repo = TempRepo::new(&sample_state());
        let runner = MockRunner::with_results(
            vec![],
            vec![ok_json(json!([[
                qc_issue(
                    101,
                    "already processed",
                    "2026-03-09T01:00:00Z",
                    "ignored",
                    TRUSTED_AUTHOR
                ),
                qc_issue(
                    102,
                    "new trusted",
                    "2026-03-09T02:00:00Z",
                    "A newly discovered QC report.",
                    TRUSTED_AUTHOR
                ),
                qc_issue(
                    103,
                    "new untrusted",
                    "2026-03-09T03:00:00Z",
                    "ignore me",
                    "someone-else"
                )
            ]]))],
        );
        let cli = Cli {
            repo_root: repo.path().to_path_buf(),
            json: true,
            command: CrossRepoCommand::ProcessQc,
        };

        let output = execute(&cli, &runner).expect("process-qc should succeed");
        let report: ProcessQcReport = serde_json::from_str(&output).expect("report should parse");

        assert_eq!(
            report.reports,
            vec![PendingIssue {
                number: 102,
                title: "new trusted".to_string(),
                created_at: "2026-03-09T02:00:00Z".to_string(),
                preview: "A newly discovered QC report.".to_string(),
            }]
        );
        assert_eq!(
            report.summary,
            ProcessingSummary {
                new_count: 1,
                skipped_untrusted_count: 1,
            }
        );
        assert_eq!(runner.gh_calls().len(), 1);
    }

    #[test]
    fn process_qc_human_output_reports_empty_results() {
        let repo = TempRepo::new(&sample_state());
        let runner = MockRunner::with_results(vec![], vec![ok_json(json!([]))]);
        let cli = Cli {
            repo_root: repo.path().to_path_buf(),
            json: false,
            command: CrossRepoCommand::ProcessQc,
        };

        let output = execute(&cli, &runner).expect("process-qc should succeed");

        assert!(output.contains("New QC reports:"));
        assert!(output.contains("  - None"));
        assert!(output.contains("Summary: 0 new, 0 skipped due to untrusted author"));
    }

    #[test]
    fn process_audit_json_detects_stale_accepted_items() {
        let repo = TempRepo::new(&sample_state());
        let runner = MockRunner::with_results(
			vec![],
			vec![
				ok_json(json!([[
					audit_issue(301, "already processed", "2026-03-09T01:00:00Z", "ignored", TRUSTED_AUTHOR),
					audit_issue(303, "new trusted", "2026-03-09T02:00:00Z", "Audit recommendation body", TRUSTED_AUTHOR),
					audit_issue(304, "new untrusted", "2026-03-09T03:00:00Z", "ignore me", "someone-else")
				]])),
				ok_json(json!([[
					audit_inbound_issue_json(
						900,
						"[Audit-ACK] Accepted improvement",
						"> **[main-orchestrator]** | Cycle 194\n\nResponding to https://github.com/EvaLok/schema-org-json-ld-audit/issues/301\n\n## Accepted\n\nWill dispatch a fix.",
						TRUSTED_AUTHOR
					),
					audit_inbound_issue_json(
						901,
						"[Audit-ACK] Accepted but untrusted",
						"> **[main-orchestrator]** | Cycle 190\n\nResponding to https://github.com/EvaLok/schema-org-json-ld-audit/issues/302\n\n## Accepted\n\nIgnore.",
						"someone-else"
					)
				]])),
			],
		);
        let cli = Cli {
            repo_root: repo.path().to_path_buf(),
            json: true,
            command: CrossRepoCommand::ProcessAudit,
        };

        let output = execute(&cli, &runner).expect("process-audit should succeed");
        let report: ProcessAuditReport =
            serde_json::from_str(&output).expect("report should parse");

        assert_eq!(report.recommendations.len(), 1);
        assert_eq!(report.recommendations[0].number, 303);
        assert_eq!(report.stale_accepted.len(), 1);
        assert_eq!(report.stale_accepted[0].audit_number, 301);
        assert_eq!(
            report.summary,
            AuditSummary {
                new_count: 1,
                stale_count: 1,
                skipped_untrusted_outbound_count: 1,
                skipped_untrusted_inbound_count: 1,
            }
        );
    }

    #[test]
    fn stale_detection_errors_when_cycle_marker_is_missing() {
        let error = detect_stale_accepted(
			&[301],
			&[AuditInboundIssue {
				number: 900,
				title: "[Audit-ACK] Accepted".to_string(),
				body: "Responding to https://github.com/EvaLok/schema-org-json-ld-audit/issues/301\n\n## Accepted".to_string(),
			}],
			200,
		)
		.expect_err("missing cycle should fail closed");

        assert!(error.contains("does not mention a cycle"));
    }

    #[test]
    fn check_qc_ack_json_matches_pending_requests_and_checks_divergence() {
        let repo = TempRepo::new(&sample_state());
        let runner = MockRunner::with_results(
			vec![ok_stdout("commit\n"), ok_stdout("package.json\n")],
			vec![ok_json(json!([[
				qc_ack_issue(
					225,
					"[QC-ACK] Re-validate v1.0.1 commit ea8ffff for publish",
					"## QC-ACK\n\nResponding to https://github.com/EvaLok/schema-org-json-ld/issues/535",
					"2026-03-06T04:39:42Z",
					TRUSTED_AUTHOR
				),
				qc_ack_issue(
					226,
					"[QC-ACK] Re-validate v1.0.2 commit abcd1234 for publish",
					"## QC-ACK\n\nResponding to https://github.com/EvaLok/schema-org-json-ld/issues/535\n\nValidated commit abcd1234.",
					"2026-03-08T04:39:42Z",
					TRUSTED_AUTHOR
				),
				qc_ack_issue(
					227,
					"[QC-ACK] Re-validate untrusted",
					"Responding to https://github.com/EvaLok/schema-org-json-ld/issues/535",
					"2026-03-08T05:39:42Z",
					"someone-else"
				)
			]]))],
		);
        let cli = Cli {
            repo_root: repo.path().to_path_buf(),
            json: true,
            command: CrossRepoCommand::CheckQcAck,
        };

        let output = execute(&cli, &runner).expect("check-qc-ack should succeed");
        let report: CheckQcAckReport = serde_json::from_str(&output).expect("report should parse");

        assert_eq!(report.acknowledgements.len(), 1);
        let ack = &report.acknowledgements[0];
        assert_eq!(ack.ack_number, 226);
        assert_eq!(ack.source_request_number, 535);
        assert_eq!(ack.validated_commit.as_deref(), Some("abcd1234"));
        assert!(ack.divergence.source_diverged);
        assert!(!ack.divergence.check_failed);
        assert_eq!(
            ack.divergence.changed_files,
            vec!["package.json".to_string()]
        );
        assert_eq!(
            report.summary,
            AckSummary {
                new_count: 1,
                skipped_untrusted_count: 1,
                pending_request_count: 1,
            }
        );
        assert_eq!(runner.git_calls().len(), 2);
    }

    #[test]
    fn qc_ack_missing_commit_fails_closed_without_git_calls() {
        let divergence =
            check_package_source_divergence(Path::new("."), &MockRunner::default(), None);

        assert!(divergence.source_diverged);
        assert!(divergence.check_failed);
        assert!(divergence.changed_files.is_empty());
        assert_eq!(
            divergence.error.as_deref(),
            Some("validated commit not found in QC acknowledgement")
        );
    }

    #[test]
    fn closed_qc_ack_parser_allows_missing_body() {
        let parsed = parse_closed_qc_ack_issues(json!([[
            {
                "number": 27,
                "title": "Legacy QC issue",
                "body": null,
                "closed_at": "2026-03-01T00:00:00Z",
                "user": { "login": TRUSTED_AUTHOR }
            }
        ]]))
        .expect("missing body should default to empty text");

        assert_eq!(parsed.trusted.len(), 1);
        assert_eq!(parsed.trusted[0].body, "");
    }

    #[test]
    fn pending_request_parser_rejects_invalid_shape() {
        let error = build_pending_request_set(&[json!({"unexpected": true})])
            .expect_err("invalid pending request should fail closed");

        assert!(error.contains("unsupported shape"));
    }

    fn sample_state() -> Value {
        json!({
            "schema_version": 1,
            "last_cycle": { "number": 200 },
            "qc_processed": [101],
            "audit_processed": [301, 302],
            "qc_requests_pending": [535],
            "publish_gate": {
                "qc_ack": "EvaLok/schema-org-json-ld-qc#225",
                "validated_commit": "ea8ffff",
                "source_diverged": false
            }
        })
    }

    fn ok_json(value: Value) -> Result<ExecutionResult, String> {
        Ok(ExecutionResult {
            exit_code: Some(0),
            stdout: serde_json::to_string(&value).unwrap(),
            stderr: String::new(),
        })
    }

    fn ok_stdout(stdout: &str) -> Result<ExecutionResult, String> {
        Ok(ExecutionResult {
            exit_code: Some(0),
            stdout: stdout.to_string(),
            stderr: String::new(),
        })
    }

    fn qc_issue(number: u64, title: &str, created_at: &str, body: &str, author: &str) -> Value {
        json!({
            "number": number,
            "title": title,
            "created_at": created_at,
            "body": body,
            "user": { "login": author }
        })
    }

    fn audit_issue(number: u64, title: &str, created_at: &str, body: &str, author: &str) -> Value {
        qc_issue(number, title, created_at, body, author)
    }

    fn audit_inbound_issue_json(number: u64, title: &str, body: &str, author: &str) -> Value {
        json!({
            "number": number,
            "title": title,
            "body": body,
            "user": { "login": author }
        })
    }

    fn qc_ack_issue(number: u64, title: &str, body: &str, closed_at: &str, author: &str) -> Value {
        json!({
            "number": number,
            "title": title,
            "body": body,
            "closed_at": closed_at,
            "user": { "login": author }
        })
    }
}
