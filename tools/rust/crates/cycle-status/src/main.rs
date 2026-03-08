use chrono::{DateTime, TimeDelta, Utc};
use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const QC_REPO: &str = "EvaLok/schema-org-json-ld-qc";
const AUDIT_REPO: &str = "EvaLok/schema-org-json-ld-audit";
const MAX_CONCURRENCY: usize = 2;

#[derive(Parser)]
#[command(name = "cycle-status")]
struct Cli {
    /// Path to the repository root
    #[arg(long)]
    repo_root: PathBuf,

    /// Timestamp used to filter Eva comments (ISO8601)
    #[arg(long)]
    last_cycle_timestamp: Option<String>,

    /// Output report as JSON
    #[arg(long)]
    json: bool,
}

#[derive(Default, Deserialize)]
struct StateJson {
    last_cycle: Option<LastCycle>,
    qc_processed: Option<Vec<u64>>,
    audit_processed: Option<Vec<u64>>,
    publish_gate: Option<PublishGate>,
}

#[derive(Deserialize)]
struct LastCycle {
    timestamp: String,
}

#[derive(Deserialize)]
struct PublishGate {
    status: Option<String>,
    validated_commit: Option<String>,
}

#[derive(Serialize)]
struct Report {
    generated_at: String,
    last_cycle_timestamp: String,
    eva_input: EvaInput,
    agent_status: AgentStatus,
    qc_status: ProcessingStatus,
    audit_status: ProcessingStatus,
    commit_freeze: Option<CommitFreezeStatus>,
    concurrency: Concurrency,
    action_items: Vec<String>,
    errors: Vec<String>,
}

#[derive(Default, Serialize)]
struct EvaInput {
    open_issues: Vec<SimpleIssue>,
    comments_since_last_cycle: Vec<EvaComment>,
}

#[derive(Default, Serialize)]
struct AgentStatus {
    open_prs: Vec<OpenPr>,
    open_copilot_issues: Vec<SimpleIssue>,
    stale_dispatches: Vec<StaleDispatch>,
    recently_merged: Vec<MergedPr>,
}

#[derive(Default, Serialize)]
struct ProcessingStatus {
    open_outbound: Vec<ProcessedIssue>,
    unprocessed_outbound: Vec<SimpleIssue>,
    open_inbound: Vec<SimpleIssue>,
}

#[derive(Serialize)]
struct Concurrency {
    in_flight: usize,
    max: usize,
    dispatch_available: bool,
}

#[derive(Clone, Serialize)]
struct SimpleIssue {
    number: u64,
    title: String,
}

#[derive(Serialize)]
struct EvaComment {
    issue_url: String,
    first_line: String,
}

#[derive(Serialize)]
struct OpenPr {
    number: u64,
    title: String,
    author: String,
    is_draft: bool,
    copilot_work_finished: Option<bool>,
    #[serde(skip_serializing)]
    body: Option<String>,
}

#[derive(Serialize)]
struct MergedPr {
    number: u64,
    title: String,
    merged_at: String,
}

#[derive(Serialize)]
struct ProcessedIssue {
    number: u64,
    title: String,
    processed: bool,
}

#[derive(Serialize)]
struct StaleDispatch {
    number: u64,
    title: String,
    created_at: String,
    age_hours: f64,
}

struct CopilotIssue {
    number: u64,
    title: String,
    created_at: String,
}

#[derive(Serialize)]
struct CommitFreezeStatus {
    validated_commit: String,
    diverged: bool,
    check_failed: bool,
    changed_files: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let mut errors = Vec::new();
    let state = read_state_json(&cli.repo_root.join("docs/state.json"), &mut errors);
    let last_cycle_timestamp = resolve_last_cycle_timestamp(&cli, &state, &mut errors);

    let eva_input = gather_eva_input(&last_cycle_timestamp, &mut errors);
    let agent_status = gather_agent_status(&mut errors);
    let qc_status = gather_qc_status(&state, &mut errors);
    let audit_status = gather_audit_status(&state, &mut errors);
    let commit_freeze = check_commit_freeze(&cli.repo_root, &state, &mut errors);
    let publish_gate_status = state
        .publish_gate
        .as_ref()
        .and_then(|gate| gate.status.as_deref());

    let draft_prs_by_copilot = agent_status
        .open_prs
        .iter()
        .filter(|pr| pr.author == "copilot-swe-agent[bot]" && pr.is_draft)
        .count();
    let in_flight = agent_status.open_copilot_issues.len() + draft_prs_by_copilot;
    let concurrency = Concurrency {
        in_flight,
        max: MAX_CONCURRENCY,
        dispatch_available: in_flight < MAX_CONCURRENCY,
    };

    let action_items = build_action_items(
        &eva_input,
        &agent_status,
        &qc_status,
        &audit_status,
        commit_freeze.as_ref(),
        publish_gate_status,
        &concurrency,
    );
    let report = Report {
        generated_at: current_timestamp_utc(),
        last_cycle_timestamp,
        eva_input,
        agent_status,
        qc_status,
        audit_status,
        commit_freeze,
        concurrency,
        action_items,
        errors,
    };

    if cli.json {
        match serde_json::to_string_pretty(&report) {
            Ok(json) => println!("{}", json),
            Err(e) => {
                eprintln!("Failed to serialize report JSON: {}", e);
                std::process::exit(1);
            }
        }
    } else {
        print_human_report(&report);
    }

    std::process::exit(report_exit_code(&report, publish_gate_status));
}

fn current_timestamp_utc() -> String {
    const FALLBACK: &str = "1970-01-01T00:00:00Z";
    let output = Command::new("date")
        .args(["-u", "+%Y-%m-%dT%H:%M:%SZ"])
        .output();
    match output {
        Ok(result) if result.status.success() => {
            let timestamp = String::from_utf8_lossy(&result.stdout).trim().to_string();
            if timestamp.is_empty() {
                FALLBACK.to_string()
            } else {
                timestamp
            }
        }
        _ => FALLBACK.to_string(),
    }
}

fn read_state_json(path: &Path, errors: &mut Vec<String>) -> StateJson {
    let content = match fs::read_to_string(path) {
        Ok(content) => content,
        Err(e) => {
            errors.push(format!("Failed to read {}: {}", path.display(), e));
            return StateJson::default();
        }
    };

    match serde_json::from_str::<StateJson>(&content) {
        Ok(state) => state,
        Err(e) => {
            errors.push(format!("Failed to parse {}: {}", path.display(), e));
            StateJson::default()
        }
    }
}

fn resolve_last_cycle_timestamp(cli: &Cli, state: &StateJson, errors: &mut Vec<String>) -> String {
    if let Some(cli_timestamp) = &cli.last_cycle_timestamp {
        return cli_timestamp.clone();
    }
    if let Some(state_timestamp) = state.last_cycle.as_ref().map(|c| c.timestamp.clone()) {
        return state_timestamp;
    }
    let fallback = "1970-01-01T00:00:00Z".to_string();
    errors.push(format!(
        "Missing --last-cycle-timestamp and docs/state.json last_cycle.timestamp; defaulting to {}",
        fallback
    ));
    fallback
}

fn gather_eva_input(last_cycle_timestamp: &str, errors: &mut Vec<String>) -> EvaInput {
    let mut section = EvaInput::default();

    match gh_json(&[
        "issue",
        "list",
        "--repo",
        MAIN_REPO,
        "--label",
        "input-from-eva",
        "--state",
        "open",
        "--json",
        "number,title,author,createdAt",
    ]) {
        Ok(value) => {
            if let Some(items) = value.as_array() {
                section.open_issues = items
                    .iter()
                    .filter(|item| json_str(item, &["author", "login"]) == Some("EvaLok"))
                    .filter_map(to_simple_issue)
                    .collect();
            }
        }
        Err(e) => errors.push(format!("Eva input issues query failed: {}", e)),
    }

    let comment_path = format!(
        "repos/{}/issues/comments?sort=created&direction=desc&since={}&per_page=50",
        MAIN_REPO, last_cycle_timestamp
    );
    match gh_json(&["api", &comment_path]) {
        Ok(value) => {
            if let Some(items) = value.as_array() {
                section.comments_since_last_cycle = items
                    .iter()
                    .filter(|item| json_str(item, &["user", "login"]) == Some("EvaLok"))
                    .filter_map(|item| {
                        let body = json_str(item, &["body"])?;
                        let ignore = [
                            "[main-orchestrator]",
                            "[qc-orchestrator]",
                            "[audit-orchestrator]",
                        ]
                        .iter()
                        .any(|tag| body.contains(tag));
                        if ignore {
                            return None;
                        }
                        let issue_url = api_issue_url_to_web_url(json_str(item, &["issue_url"])?);
                        let first_line = body.lines().next().unwrap_or("").trim().to_string();
                        Some(EvaComment {
                            issue_url,
                            first_line,
                        })
                    })
                    .collect();
            }
        }
        Err(e) => errors.push(format!("Eva comments query failed: {}", e)),
    }

    section
}

fn gather_agent_status(errors: &mut Vec<String>) -> AgentStatus {
    let mut section = AgentStatus::default();

    match gh_json(&[
        "pr",
        "list",
        "--repo",
        MAIN_REPO,
        "--state",
        "open",
        "--json",
        "number,title,author,isDraft,body",
    ]) {
        Ok(value) => {
            if let Some(items) = value.as_array() {
                section.open_prs = items
                    .iter()
                    .filter_map(|item| {
                        let number = item.get("number")?.as_u64()?;
                        let title = item.get("title")?.as_str()?.to_string();
                        let author = json_str(item, &["author", "login"])?.to_string();
                        let is_draft = item.get("isDraft")?.as_bool()?;
                        let body = item
                            .get("body")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string());
                        Some(OpenPr {
                            number,
                            title,
                            author,
                            is_draft,
                            copilot_work_finished: None,
                            body,
                        })
                    })
                    .collect();

                for pr in &mut section.open_prs {
                    if pr.author == "copilot-swe-agent[bot]" {
                        let timeline_path =
                            format!("repos/{}/issues/{}/timeline", MAIN_REPO, pr.number);
                        match gh_json(&["api", &timeline_path, "--paginate"]) {
                            Ok(timeline) => {
                                let finished = timeline.as_array().is_some_and(|events| {
                                    events.iter().any(|event| {
                                        event
                                            .get("event")
                                            .and_then(|v| v.as_str())
                                            .is_some_and(|name| name == "copilot_work_finished")
                                    })
                                });
                                pr.copilot_work_finished = Some(finished);
                            }
                            Err(e) => errors.push(format!(
                                "Copilot timeline query failed for PR #{}: {}",
                                pr.number, e
                            )),
                        }
                    }
                }
            }
        }
        Err(e) => errors.push(format!("Open PR query failed: {}", e)),
    }

    let copilot_issues_path = format!(
        "repos/{}/issues?assignee=copilot-swe-agent%5Bbot%5D&state=open",
        MAIN_REPO
    );
    match gh_json(&["api", &copilot_issues_path]) {
        Ok(value) => {
            if let Some(items) = value.as_array() {
                let copilot_issues = items
                    .iter()
                    .filter(|item| item.get("pull_request").is_none())
                    .filter_map(to_copilot_issue)
                    .collect::<Vec<_>>();
                section.open_copilot_issues = copilot_issues
                    .iter()
                    .map(|issue| SimpleIssue {
                        number: issue.number,
                        title: issue.title.clone(),
                    })
                    .collect();
                section.stale_dispatches = collect_stale_dispatches(
                    &copilot_issues,
                    &section.open_prs,
                    Utc::now(),
                    errors,
                );
            }
        }
        Err(e) => errors.push(format!("Open Copilot issues query failed: {}", e)),
    }

    match gh_json(&[
        "pr",
        "list",
        "--repo",
        MAIN_REPO,
        "--state",
        "merged",
        "--limit",
        "5",
        "--json",
        "number,title,mergedAt",
    ]) {
        Ok(value) => {
            if let Some(items) = value.as_array() {
                section.recently_merged = items
                    .iter()
                    .filter_map(|item| {
                        let number = item.get("number")?.as_u64()?;
                        let title = item.get("title")?.as_str()?.to_string();
                        let merged_at = item.get("mergedAt")?.as_str()?.to_string();
                        Some(MergedPr {
                            number,
                            title,
                            merged_at,
                        })
                    })
                    .collect();
            }
        }
        Err(e) => errors.push(format!("Recently merged PR query failed: {}", e)),
    }

    section
}

fn collect_stale_dispatches(
    copilot_issues: &[CopilotIssue],
    open_prs: &[OpenPr],
    now: DateTime<Utc>,
    errors: &mut Vec<String>,
) -> Vec<StaleDispatch> {
    copilot_issues
        .iter()
        .filter_map(|issue| {
            let created_at = match DateTime::parse_from_rfc3339(&issue.created_at) {
                Ok(value) => value.with_timezone(&Utc),
                Err(e) => {
                    errors.push(format!(
                        "Failed to parse created_at for issue #{}: {}",
                        issue.number, e
                    ));
                    return None;
                }
            };
            let age = now.signed_duration_since(created_at);
            let has_matching_pr = open_prs.iter().any(|pr| {
                if pr.author != "copilot-swe-agent[bot]" {
                    return false;
                }
                let issue_tag = format!("#{}", issue.number);
                // Word-boundary title match: `#N` must not be followed by a digit
                // (avoids #1 matching inside #101).
                let title_matches =
                    contains_issue_tag_at_word_boundary(&pr.title, &issue_tag);
                // Body match: Fixes/Closes/Resolves #N (case-insensitive)
                let body_matches = pr.body.as_deref().is_some_and(|body| {
                    let lower = body.to_lowercase();
                    ["fixes ", "closes ", "resolves "].iter().any(|kw| {
                        let mut search = lower.as_str();
                        while let Some(pos) = search.find(kw) {
                            let rest = &search[pos + kw.len()..];
                            if contains_issue_tag_at_word_boundary(rest, &issue_tag) {
                                return true;
                            }
                            search = &search[pos + kw.len()..];
                        }
                        false
                    })
                });
                title_matches || body_matches
            });
            if age > TimeDelta::try_hours(2).unwrap() && !has_matching_pr {
                Some(StaleDispatch {
                    number: issue.number,
                    title: issue.title.clone(),
                    created_at: issue.created_at.clone(),
                    age_hours: age.num_minutes() as f64 / 60.0,
                })
            } else {
                None
            }
        })
        .collect()
}

/// Returns true if `text` contains `tag` at a position where the character
/// immediately following the tag is not a digit — i.e. `#N` is not a prefix
/// of a larger number like `#101`.
fn contains_issue_tag_at_word_boundary(text: &str, tag: &str) -> bool {
    let mut search = text;
    while let Some(pos) = search.find(tag) {
        let after = pos + tag.len();
        let at_word_boundary = search
            .as_bytes()
            .get(after)
            .is_none_or(|c| !c.is_ascii_digit());
        if at_word_boundary {
            return true;
        }
        search = &search[pos + 1..];
    }
    false
}

fn is_valid_commit_sha(sha: &str) -> bool {
    let len = sha.len();
    (4..=40).contains(&len) && sha.chars().all(|c| c.is_ascii_hexdigit())
}

fn is_pre_publish_gate_status(status: Option<&str>) -> bool {
    matches!(status, Some("awaiting_validation" | "validated"))
}

fn report_exit_code(report: &Report, publish_gate_status: Option<&str>) -> i32 {
    if report
        .commit_freeze
        .as_ref()
        .is_some_and(|status| status.check_failed || status.diverged)
        && is_pre_publish_gate_status(publish_gate_status)
    {
        1
    } else {
        0
    }
}

fn check_commit_freeze(
    repo_root: &Path,
    state: &StateJson,
    errors: &mut Vec<String>,
) -> Option<CommitFreezeStatus> {
    let validated_commit = state
        .publish_gate
        .as_ref()
        .and_then(|gate| gate.validated_commit.clone())?;

    // Validate the commit SHA format before using it in a git command.
    if !is_valid_commit_sha(&validated_commit) {
        errors.push(format!(
            "Commit freeze check failed: validated_commit {:?} is not a valid commit SHA (expected 4–40 hex characters)",
            validated_commit
        ));
        return Some(CommitFreezeStatus {
            validated_commit,
            diverged: true,
            check_failed: true,
            changed_files: Vec::new(),
        });
    }

    // Verify the commit is reachable in this repository.
    let cat_file = Command::new("git")
        .current_dir(repo_root)
        .args(["cat-file", "-t", &validated_commit])
        .output();
    match cat_file {
        Ok(output) if output.status.success() => {
            let obj_type = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if obj_type != "commit" {
                errors.push(format!(
                    "Commit freeze check failed: validated commit {} is not a commit object (got {:?})",
                    validated_commit, obj_type
                ));
                return Some(CommitFreezeStatus {
                    validated_commit,
                    diverged: true,
                    check_failed: true,
                    changed_files: Vec::new(),
                });
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            errors.push(format!(
                "Commit freeze check failed: validated commit {} is not reachable in this repository: {}",
                validated_commit,
                if stderr.is_empty() { "<no stderr>".to_string() } else { stderr }
            ));
            return Some(CommitFreezeStatus {
                validated_commit,
                diverged: true,
                check_failed: true,
                changed_files: Vec::new(),
            });
        }
        Err(e) => {
            errors.push(format!(
                "Commit freeze check failed (unable to execute git cat-file): {}",
                e
            ));
            return Some(CommitFreezeStatus {
                validated_commit,
                diverged: true,
                check_failed: true,
                changed_files: Vec::new(),
            });
        }
    }

    let range = format!("{}..HEAD", validated_commit);
    let output = match Command::new("git")
        .current_dir(repo_root)
        .args([
            "diff",
            "--name-only",
            &range,
            "--",
            "php/src/",
            "php/test/",
            "ts/src/",
            "ts/test/",
            "package.json",
            "tsconfig.json",
            "scripts/verify-build.mjs",
        ])
        .output()
    {
        Ok(output) => output,
        Err(e) => {
            errors.push(format!(
                "Commit freeze check failed (unable to execute git diff): {}",
                e
            ));
            return Some(CommitFreezeStatus {
                validated_commit,
                diverged: true,
                check_failed: true,
                changed_files: Vec::new(),
            });
        }
    };
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        errors.push(format!(
            "Commit freeze check failed (`git diff --name-only {} -- ...`): {}",
            range,
            if stderr.is_empty() {
                "<no stderr>".to_string()
            } else {
                stderr
            }
        ));
        return Some(CommitFreezeStatus {
            validated_commit,
            diverged: true,
            check_failed: true,
            changed_files: Vec::new(),
        });
    }

    let changed_files = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.to_string())
        .collect::<Vec<_>>();
    let diverged = !changed_files.is_empty();
    Some(CommitFreezeStatus {
        validated_commit,
        diverged,
        check_failed: false,
        changed_files,
    })
}

fn gather_qc_status(state: &StateJson, errors: &mut Vec<String>) -> ProcessingStatus {
    let processed_set = to_set(state.qc_processed.as_ref());
    gather_processing_status(
        errors,
        "QC outbound query failed",
        &format!(
            "repos/{}/issues?labels=qc-outbound&state=open&creator=EvaLok&sort=created&direction=asc&per_page=100",
            QC_REPO
        ),
        "QC inbound query failed",
        &[
            "issue",
            "list",
            "--repo",
            MAIN_REPO,
            "--label",
            "qc-inbound",
            "--state",
            "open",
            "--json",
            "number,title",
        ],
        &processed_set,
    )
}

fn gather_audit_status(state: &StateJson, errors: &mut Vec<String>) -> ProcessingStatus {
    let processed_set = to_set(state.audit_processed.as_ref());
    gather_processing_status(
        errors,
        "Audit outbound query failed",
        &format!(
            "repos/{}/issues?labels=audit-outbound&state=open&creator=EvaLok&sort=created&direction=asc&per_page=100",
            AUDIT_REPO
        ),
        "Audit inbound query failed",
        &[
            "issue",
            "list",
            "--repo",
            MAIN_REPO,
            "--label",
            "audit-inbound",
            "--state",
            "open",
            "--json",
            "number,title",
        ],
        &processed_set,
    )
}

fn gather_processing_status(
    errors: &mut Vec<String>,
    outbound_error_label: &str,
    outbound_api_path: &str,
    inbound_error_label: &str,
    inbound_cmd: &[&str],
    processed_set: &HashSet<u64>,
) -> ProcessingStatus {
    let mut section = ProcessingStatus::default();

    match gh_json(&["api", outbound_api_path]) {
        Ok(value) => {
            if let Some(items) = value.as_array() {
                section.open_outbound = items
                    .iter()
                    .filter_map(|item| {
                        let issue = to_simple_issue(item)?;
                        let processed = processed_set.contains(&issue.number);
                        Some(ProcessedIssue {
                            number: issue.number,
                            title: issue.title,
                            processed,
                        })
                    })
                    .collect();
                section.unprocessed_outbound = section
                    .open_outbound
                    .iter()
                    .filter(|item| !item.processed)
                    .map(|item| SimpleIssue {
                        number: item.number,
                        title: item.title.clone(),
                    })
                    .collect();
            }
        }
        Err(e) => errors.push(format!("{}: {}", outbound_error_label, e)),
    }

    match gh_json(inbound_cmd) {
        Ok(value) => {
            if let Some(items) = value.as_array() {
                section.open_inbound = items.iter().filter_map(to_simple_issue).collect();
            }
        }
        Err(e) => errors.push(format!("{}: {}", inbound_error_label, e)),
    }

    section
}

fn build_action_items(
    eva_input: &EvaInput,
    agent_status: &AgentStatus,
    qc_status: &ProcessingStatus,
    audit_status: &ProcessingStatus,
    commit_freeze: Option<&CommitFreezeStatus>,
    publish_gate_status: Option<&str>,
    concurrency: &Concurrency,
) -> Vec<String> {
    let mut items = Vec::new();

    if !eva_input.open_issues.is_empty() {
        items.push(format!(
            "{} open input-from-eva issue{} requires attention",
            eva_input.open_issues.len(),
            if eva_input.open_issues.len() == 1 {
                ""
            } else {
                "s"
            }
        ));
    }
    if !qc_status.unprocessed_outbound.is_empty() {
        items.push(format!(
            "{} new QC outbound issue{} awaiting processing",
            qc_status.unprocessed_outbound.len(),
            if qc_status.unprocessed_outbound.len() == 1 {
                ""
            } else {
                "s"
            }
        ));
    }
    if !audit_status.unprocessed_outbound.is_empty() {
        items.push(format!(
            "{} new audit outbound issue{} awaiting processing",
            audit_status.unprocessed_outbound.len(),
            if audit_status.unprocessed_outbound.len() == 1 {
                ""
            } else {
                "s"
            }
        ));
    }
    let ready_prs = agent_status
        .open_prs
        .iter()
        .filter(|pr| {
            pr.author == "copilot-swe-agent[bot]"
                && !pr.is_draft
                && pr.copilot_work_finished == Some(true)
        })
        .count();
    if ready_prs > 0 {
        items.push(format!(
            "{} Copilot PR(s) marked as work-finished and ready for review",
            ready_prs
        ));
    }
    if !agent_status.stale_dispatches.is_empty() {
        items.push(format!(
            "{} stale Copilot dispatch{} older than 2h without PR",
            agent_status.stale_dispatches.len(),
            if agent_status.stale_dispatches.len() == 1 {
                ""
            } else {
                "es"
            }
        ));
    }
    if commit_freeze.is_some_and(|status| status.diverged) {
        if commit_freeze.is_some_and(|status| status.check_failed) {
            if is_pre_publish_gate_status(publish_gate_status) {
                items.push(
                    "Commit freeze check failed — could not verify QC-validated commit integrity"
                        .to_string(),
                );
            } else {
                items.push(
                    "Commit freeze check failed outside pre-publish gate — awareness only"
                        .to_string(),
                );
            }
        } else if is_pre_publish_gate_status(publish_gate_status) {
            items.push(
                "Source files changed since QC-validated commit — re-validation required"
                    .to_string(),
            );
        } else {
            items.push(
                "Source files changed since QC-validated commit outside pre-publish gate — awareness only"
                    .to_string(),
            );
        }
    }
    if !concurrency.dispatch_available {
        items.push(format!(
            "Dispatch slots are full ({} / {})",
            concurrency.in_flight, concurrency.max
        ));
    }

    items
}

fn print_human_report(report: &Report) {
    println!("=== Cycle Status Report ===");
    println!("Generated: {}", report.generated_at);
    println!("Last cycle: {}", report.last_cycle_timestamp);
    println!();

    println!("--- Eva Input ---");
    println!(
        "Open input-from-eva issues: {}",
        report.eva_input.open_issues.len()
    );
    for issue in &report.eva_input.open_issues {
        println!("  {}#{} {}", MAIN_REPO, issue.number, issue.title);
    }
    println!(
        "Eva comments since last cycle: {}",
        report.eva_input.comments_since_last_cycle.len()
    );
    for comment in &report.eva_input.comments_since_last_cycle {
        println!("  {} {}", comment.issue_url, comment.first_line);
    }
    println!();

    println!("--- Agent Status ---");
    println!("Open PRs: {}", report.agent_status.open_prs.len());
    for pr in &report.agent_status.open_prs {
        if let Some(done) = pr.copilot_work_finished {
            println!(
                "  {}#{} {} [{}; copilot {}]",
                MAIN_REPO,
                pr.number,
                pr.title,
                if pr.is_draft { "draft" } else { "ready" },
                if done { "finished" } else { "working" }
            );
        } else {
            println!(
                "  {}#{} {} [{}]",
                MAIN_REPO,
                pr.number,
                pr.title,
                if pr.is_draft { "draft" } else { "ready" }
            );
        }
    }
    println!(
        "Open Copilot issues: {}",
        report.agent_status.open_copilot_issues.len()
    );
    for issue in &report.agent_status.open_copilot_issues {
        println!("  {}#{} {}", MAIN_REPO, issue.number, issue.title);
    }
    let merged_list = report
        .agent_status
        .recently_merged
        .iter()
        .map(|pr| format!("{}#{}", MAIN_REPO, pr.number))
        .collect::<Vec<_>>()
        .join(", ");
    println!(
        "Recently merged: {}",
        if merged_list.is_empty() {
            "none".to_string()
        } else {
            merged_list
        }
    );
    println!(
        "Stale dispatches (>2h, no PR): {}",
        report.agent_status.stale_dispatches.len()
    );
    for dispatch in &report.agent_status.stale_dispatches {
        println!(
            "  {}#{} {} (created {}; age {:.1}h)",
            MAIN_REPO, dispatch.number, dispatch.title, dispatch.created_at, dispatch.age_hours
        );
    }
    println!();

    println!("--- QC Status ---");
    println!(
        "Open qc-outbound (QC repo): {}{}",
        report.qc_status.open_outbound.len(),
        if report.qc_status.open_outbound.is_empty() {
            "".to_string()
        } else if report.qc_status.unprocessed_outbound.is_empty() {
            " (all processed)".to_string()
        } else {
            format!(
                " ({} unprocessed)",
                report.qc_status.unprocessed_outbound.len()
            )
        }
    );
    for issue in &report.qc_status.open_outbound {
        println!(
            "  {}#{} {} ({})",
            QC_REPO,
            issue.number,
            issue.title,
            if issue.processed { "processed" } else { "new" }
        );
    }
    println!(
        "Open qc-inbound (this repo): {}",
        report.qc_status.open_inbound.len()
    );
    for issue in &report.qc_status.open_inbound {
        println!("  {}#{} {}", MAIN_REPO, issue.number, issue.title);
    }
    println!();

    println!("--- Audit Status ---");
    println!(
        "Open audit-outbound (audit repo): {}{}",
        report.audit_status.open_outbound.len(),
        if report.audit_status.open_outbound.is_empty() {
            "".to_string()
        } else if report.audit_status.unprocessed_outbound.is_empty() {
            " (all processed)".to_string()
        } else {
            format!(
                " ({} unprocessed)",
                report.audit_status.unprocessed_outbound.len()
            )
        }
    );
    for issue in &report.audit_status.open_outbound {
        println!(
            "  {}#{} {} ({})",
            AUDIT_REPO,
            issue.number,
            issue.title,
            if issue.processed { "processed" } else { "new" }
        );
    }
    println!(
        "Open audit-inbound (this repo): {}",
        report.audit_status.open_inbound.len()
    );
    for issue in &report.audit_status.open_inbound {
        println!("  {}#{} {}", MAIN_REPO, issue.number, issue.title);
    }
    println!();

    println!("--- Concurrency ---");
    println!(
        "In-flight agent sessions: {} / {}",
        report.concurrency.in_flight, report.concurrency.max
    );
    println!(
        "Dispatch slots available: {}",
        if report.concurrency.dispatch_available {
            "YES"
        } else {
            "NO"
        }
    );
    println!();

    println!("--- Commit Freeze ---");
    match report.commit_freeze.as_ref() {
        Some(status) => {
            println!("Validated commit: {}", status.validated_commit);
            println!(
                "Source freeze: {}",
                if status.check_failed {
                    "CHECK FAILED"
                } else if status.diverged {
                    "DIVERGED"
                } else {
                    "intact"
                }
            );
            if status.changed_files.is_empty() {
                println!("Changed files: none");
            } else {
                println!("Changed files:");
                for file in &status.changed_files {
                    println!("  {}", file);
                }
            }
        }
        None => println!("Validated commit: not set"),
    }
    println!();

    println!("--- Action Items ---");
    if report.action_items.is_empty() {
        println!("None");
    } else {
        for item in &report.action_items {
            println!("[!] {}", item);
        }
    }
    if !report.errors.is_empty() {
        println!();
        println!("--- Errors ---");
        for error in &report.errors {
            println!("[!] {}", error);
        }
    }
}

fn to_set(values: Option<&Vec<u64>>) -> HashSet<u64> {
    match values {
        Some(numbers) => numbers.iter().copied().collect(),
        None => HashSet::new(),
    }
}

fn to_simple_issue(value: &Value) -> Option<SimpleIssue> {
    Some(SimpleIssue {
        number: value.get("number")?.as_u64()?,
        title: value.get("title")?.as_str()?.to_string(),
    })
}

fn to_copilot_issue(value: &Value) -> Option<CopilotIssue> {
    Some(CopilotIssue {
        number: value.get("number")?.as_u64()?,
        title: value.get("title")?.as_str()?.to_string(),
        created_at: value.get("created_at")?.as_str()?.to_string(),
    })
}

fn json_str<'a>(value: &'a Value, path: &[&str]) -> Option<&'a str> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_str()
}

/// Convert GitHub API issue URLs to web issue URLs for human-friendly output.
/// Example: `https://api.github.com/repos/owner/repo/issues/123` -> `https://github.com/owner/repo/issues/123`.
fn api_issue_url_to_web_url(api_url: &str) -> String {
    api_url.replace("https://api.github.com/repos/", "https://github.com/")
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
            output.status.code().map_or_else(
                || "terminated by signal".to_string(),
                |code| code.to_string()
            ),
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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeDelta, Utc};

    #[test]
    fn collects_stale_dispatches_without_matching_copilot_pr() {
        let now = Utc::now();
        let issues = vec![CopilotIssue {
            number: 101,
            title: "Fix stale dispatch".to_string(),
            created_at: (now - TimeDelta::try_hours(3).unwrap()).to_rfc3339(),
        }];
        let open_prs = vec![OpenPr {
            number: 10,
            title: "Unrelated PR #999".to_string(),
            author: "copilot-swe-agent[bot]".to_string(),
            is_draft: true,
            copilot_work_finished: None,
            body: None,
        }];
        let mut errors = Vec::new();

        let stale = collect_stale_dispatches(&issues, &open_prs, now, &mut errors);

        assert!(errors.is_empty());
        assert_eq!(stale.len(), 1);
        assert_eq!(stale[0].number, 101);
        assert!((stale[0].age_hours - 3.0).abs() < 0.01);
    }

    #[test]
    fn build_action_items_includes_commit_freeze_divergence_warning() {
        let eva_input = EvaInput::default();
        let agent_status = AgentStatus::default();
        let qc_status = ProcessingStatus::default();
        let audit_status = ProcessingStatus::default();
        let concurrency = Concurrency {
            in_flight: 0,
            max: 2,
            dispatch_available: true,
        };
        let commit_freeze = Some(CommitFreezeStatus {
            validated_commit: "abc1234".to_string(),
            diverged: true,
            check_failed: false,
            changed_files: vec!["php/src/v1/Schema/Product.php".to_string()],
        });

        let action_items = build_action_items(
            &eva_input,
            &agent_status,
            &qc_status,
            &audit_status,
            commit_freeze.as_ref(),
            Some("published"),
            &concurrency,
        );

        assert!(action_items.iter().any(|item| {
            item == "Source files changed since QC-validated commit outside pre-publish gate — awareness only"
        }));
    }

    #[test]
    fn issue_younger_than_2h_is_not_stale() {
        let now = Utc::now();
        let issues = vec![CopilotIssue {
            number: 42,
            title: "New issue".to_string(),
            created_at: (now - TimeDelta::try_hours(1).unwrap()).to_rfc3339(),
        }];
        let open_prs: Vec<OpenPr> = vec![];
        let mut errors = Vec::new();

        let stale = collect_stale_dispatches(&issues, &open_prs, now, &mut errors);

        assert!(errors.is_empty());
        assert!(stale.is_empty(), "Issue younger than 2h should not be stale");
    }

    #[test]
    fn issue_with_matching_copilot_pr_is_not_stale() {
        let now = Utc::now();
        let issues = vec![CopilotIssue {
            number: 55,
            title: "Issue with PR".to_string(),
            created_at: (now - TimeDelta::try_hours(5).unwrap()).to_rfc3339(),
        }];
        let open_prs = vec![OpenPr {
            number: 20,
            title: "Implement feature #55".to_string(),
            author: "copilot-swe-agent[bot]".to_string(),
            is_draft: true,
            copilot_work_finished: None,
            body: None,
        }];
        let mut errors = Vec::new();

        let stale = collect_stale_dispatches(&issues, &open_prs, now, &mut errors);

        assert!(errors.is_empty());
        assert!(
            stale.is_empty(),
            "Issue with a matching Copilot PR should not be stale"
        );
    }

    #[test]
    fn issue_with_matching_pr_in_body_closes_is_not_stale() {
        let now = Utc::now();
        let issues = vec![CopilotIssue {
            number: 77,
            title: "Issue linked in body".to_string(),
            created_at: (now - TimeDelta::try_hours(4).unwrap()).to_rfc3339(),
        }];
        let open_prs = vec![OpenPr {
            number: 30,
            title: "Some unrelated title".to_string(),
            author: "copilot-swe-agent[bot]".to_string(),
            is_draft: false,
            copilot_work_finished: Some(true),
            body: Some("This PR closes #77 as requested.".to_string()),
        }];
        let mut errors = Vec::new();

        let stale = collect_stale_dispatches(&issues, &open_prs, now, &mut errors);

        assert!(errors.is_empty());
        assert!(
            stale.is_empty(),
            "Issue linked with 'closes #N' in PR body should not be stale"
        );
    }

    #[test]
    fn false_positive_title_match_does_not_suppress_stale() {
        // Issue #1 should NOT be suppressed by a PR mentioning #101
        let now = Utc::now();
        let issues = vec![CopilotIssue {
            number: 1,
            title: "Issue one".to_string(),
            created_at: (now - TimeDelta::try_hours(3).unwrap()).to_rfc3339(),
        }];
        let open_prs = vec![OpenPr {
            number: 40,
            title: "Fix issue #101".to_string(),
            author: "copilot-swe-agent[bot]".to_string(),
            is_draft: true,
            copilot_work_finished: None,
            body: None,
        }];
        let mut errors = Vec::new();

        let stale = collect_stale_dispatches(&issues, &open_prs, now, &mut errors);

        assert!(errors.is_empty());
        assert_eq!(
            stale.len(),
            1,
            "PR mentioning #101 should not suppress issue #1 stale detection"
        );
    }

    #[test]
    fn diverged_false_does_not_produce_commit_freeze_action_item() {
        let eva_input = EvaInput::default();
        let agent_status = AgentStatus::default();
        let qc_status = ProcessingStatus::default();
        let audit_status = ProcessingStatus::default();
        let concurrency = Concurrency {
            in_flight: 0,
            max: 2,
            dispatch_available: true,
        };
        let commit_freeze = Some(CommitFreezeStatus {
            validated_commit: "abc1234".to_string(),
            diverged: false,
            check_failed: false,
            changed_files: vec![],
        });

        let action_items = build_action_items(
            &eva_input,
            &agent_status,
            &qc_status,
            &audit_status,
            commit_freeze.as_ref(),
            Some("validated"),
            &concurrency,
        );

        assert!(
            !action_items
                .iter()
                .any(|item| item.contains("re-validation required")),
            "diverged=false should not produce a commit freeze action item"
        );
        assert!(
            !action_items
                .iter()
                .any(|item| item.contains("check failed")),
            "diverged=false/check_failed=false should not produce a check failed action item"
        );
    }

    #[test]
    fn invalid_sha_fails_validation() {
        assert!(!is_valid_commit_sha(""));
        assert!(!is_valid_commit_sha("xyz"));
        assert!(!is_valid_commit_sha("abc")); // too short (< 4)
        assert!(!is_valid_commit_sha("not-a-sha!"));
        assert!(!is_valid_commit_sha(&"a".repeat(41))); // too long
        assert!(!is_valid_commit_sha("deadbeef; rm -rf /"));
    }

    #[test]
    fn valid_sha_passes_validation() {
        assert!(is_valid_commit_sha("abcd"));
        assert!(is_valid_commit_sha("abc1234"));
        assert!(is_valid_commit_sha("deadbeef"));
        assert!(is_valid_commit_sha(&"a".repeat(40)));
        assert!(is_valid_commit_sha("0123456789abcdef"));
    }

    #[test]
    fn check_failed_produces_check_failed_action_item() {
        let eva_input = EvaInput::default();
        let agent_status = AgentStatus::default();
        let qc_status = ProcessingStatus::default();
        let audit_status = ProcessingStatus::default();
        let concurrency = Concurrency {
            in_flight: 0,
            max: 2,
            dispatch_available: true,
        };
        let commit_freeze = Some(CommitFreezeStatus {
            validated_commit: "abc1234".to_string(),
            diverged: true,
            check_failed: true,
            changed_files: vec![],
        });

        let action_items = build_action_items(
            &eva_input,
            &agent_status,
            &qc_status,
            &audit_status,
            commit_freeze.as_ref(),
            Some("validated"),
            &concurrency,
        );

        assert!(
            action_items
                .iter()
                .any(|item| item.contains("Commit freeze check failed")),
            "check_failed=true should produce a 'check failed' action item"
        );
        assert!(
            !action_items
                .iter()
                .any(|item| item.contains("re-validation required")),
            "check_failed=true should not produce the divergence message"
        );
    }

    fn sample_report(commit_freeze: Option<CommitFreezeStatus>, action_items: Vec<String>) -> Report {
        Report {
            generated_at: "2026-03-08T00:00:00Z".to_string(),
            last_cycle_timestamp: "2026-03-08T00:00:00Z".to_string(),
            eva_input: EvaInput::default(),
            agent_status: AgentStatus::default(),
            qc_status: ProcessingStatus::default(),
            audit_status: ProcessingStatus::default(),
            commit_freeze,
            concurrency: Concurrency {
                in_flight: 0,
                max: MAX_CONCURRENCY,
                dispatch_available: true,
            },
            action_items,
            errors: Vec::new(),
        }
    }

    #[test]
    fn report_without_commit_freeze_issues_exits_zero() {
        let report = sample_report(
            Some(CommitFreezeStatus {
                validated_commit: "abc1234".to_string(),
                diverged: false,
                check_failed: false,
                changed_files: vec![],
            }),
            Vec::new(),
        );

        assert_eq!(report_exit_code(&report, Some("validated")), 0);
    }

    #[test]
    fn report_with_check_failed_commit_freeze_exits_one() {
        let report = sample_report(
            Some(CommitFreezeStatus {
                validated_commit: "abc1234".to_string(),
                diverged: true,
                check_failed: true,
                changed_files: vec![],
            }),
            vec!["Commit freeze check failed".to_string()],
        );

        assert_eq!(report_exit_code(&report, Some("validated")), 1);
    }

    #[test]
    fn report_with_diverged_commit_freeze_exits_one() {
        let report = sample_report(
            Some(CommitFreezeStatus {
                validated_commit: "abc1234".to_string(),
                diverged: true,
                check_failed: false,
                changed_files: vec!["ts/src/index.ts".to_string()],
            }),
            vec!["Source files changed since QC-validated commit".to_string()],
        );

        assert_eq!(report_exit_code(&report, Some("validated")), 1);
    }

    #[test]
    fn report_with_diverged_commit_freeze_post_publish_exits_zero() {
        let report = sample_report(
            Some(CommitFreezeStatus {
                validated_commit: "abc1234".to_string(),
                diverged: true,
                check_failed: false,
                changed_files: vec!["package.json".to_string()],
            }),
            vec![
                "Source files changed since QC-validated commit outside pre-publish gate — awareness only"
                    .to_string(),
            ],
        );

        assert_eq!(report_exit_code(&report, Some("published")), 0);
    }

    #[test]
    fn report_with_other_action_items_but_clean_commit_freeze_exits_zero() {
        let report = sample_report(
            Some(CommitFreezeStatus {
                validated_commit: "abc1234".to_string(),
                diverged: false,
                check_failed: false,
                changed_files: vec![],
            }),
            vec![
                "Dispatch slots are full (2 / 2)".to_string(),
                "1 open input-from-eva issue requires attention".to_string(),
            ],
        );

        assert_eq!(report_exit_code(&report, Some("validated")), 0);
    }
}
