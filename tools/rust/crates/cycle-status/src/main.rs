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
}

#[derive(Deserialize)]
struct LastCycle {
    timestamp: String,
}

#[derive(Serialize)]
struct Report {
    generated_at: String,
    last_cycle_timestamp: String,
    eva_input: EvaInput,
    agent_status: AgentStatus,
    qc_status: ProcessingStatus,
    audit_status: ProcessingStatus,
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

fn main() {
    let cli = Cli::parse();
    let mut errors = Vec::new();
    let state = read_state_json(&cli.repo_root.join("docs/state.json"), &mut errors);
    let last_cycle_timestamp = resolve_last_cycle_timestamp(&cli, &state, &mut errors);

    let eva_input = gather_eva_input(&last_cycle_timestamp, &mut errors);
    let agent_status = gather_agent_status(&mut errors);
    let qc_status = gather_qc_status(&state, &mut errors);
    let audit_status = gather_audit_status(&state, &mut errors);

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
        &concurrency,
    );
    let report = Report {
        generated_at: current_timestamp_utc(),
        last_cycle_timestamp,
        eva_input,
        agent_status,
        qc_status,
        audit_status,
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
                        let issue_url = json_str(item, &["issue_url"])?.to_string();
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
        "number,title,author,isDraft",
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
                        Some(OpenPr {
                            number,
                            title,
                            author,
                            is_draft,
                            copilot_work_finished: None,
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

    let copilot_issues_path =
        format!("repos/{}/issues?assignee=copilot-swe-agent[bot]&state=open", MAIN_REPO);
    match gh_json(&["api", &copilot_issues_path]) {
        Ok(value) => {
            if let Some(items) = value.as_array() {
                section.open_copilot_issues = items
                    .iter()
                    .filter(|item| item.get("pull_request").is_none())
                    .filter_map(to_simple_issue)
                    .collect();
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

fn gather_qc_status(state: &StateJson, errors: &mut Vec<String>) -> ProcessingStatus {
    let processed_set = to_set(state.qc_processed.as_ref());
    gather_processing_status(
        errors,
        "QC outbound query failed",
        &format!(
            "repos/{}/issues?labels=qc-outbound&state=open&creator=EvaLok&sort=created&direction=asc",
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
            "repos/{}/issues?labels=audit-outbound&state=open&creator=EvaLok&sort=created&direction=asc",
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

fn json_str<'a>(value: &'a Value, path: &[&str]) -> Option<&'a str> {
    let mut current = value;
    for key in path {
        current = current.get(*key)?;
    }
    current.as_str()
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
