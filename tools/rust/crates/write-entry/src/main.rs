mod runner;

use chrono::{DateTime, NaiveDate, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use state_schema::{
    current_cycle_from_state, read_state_value, AgentSession, DeferredFinding, ReviewHistoryEntry,
    StateJson,
};
use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

const PRIMARY_ISSUES_URL: &str = "https://github.com/EvaLok/schema-org-json-ld/issues";
const QC_ISSUES_URL: &str = "https://github.com/EvaLok/schema-org-json-ld-qc/issues";
const AUDIT_ISSUES_URL: &str = "https://github.com/EvaLok/schema-org-json-ld-audit/issues";
const PRIMARY_COMMITS_URL: &str = "https://github.com/EvaLok/schema-org-json-ld/commit";
const JOURNAL_DESCRIPTION: &str = "Reflective log for the schema-org-json-ld orchestrator.";
const NOT_PROVIDED: &str = "Not provided.";
const CYCLE_STATE_HEADING: &str = "## Cycle state";
const LEGACY_STATE_HEADING: &str = "## Pre-dispatch state";
const NEXT_STEPS_HEADING: &str = "## Next steps";
const ISSUES_PROCESSED_HEADING: &str = "### Issues processed";
const LEGACY_STATE_DISCLAIMER: &str =
    "*Snapshot before review dispatch — final counters may differ after C6.*";
const IN_FLIGHT_PREFIX: &str = "- **In-flight agent sessions**: ";
const PIPELINE_STATUS_PREFIX: &str = "- **Pipeline status**: ";
const CLOSE_OUT_GATE_FAILURES_PREFIX: &str = "- **Close-out gate failures**: ";
const PUBLISH_GATE_PREFIX: &str = "- **Publish gate**: ";
const INFRASTRUCTURE_ROOTS: [&str; 2] = ["tools", ".claude/skills"];
const INFRASTRUCTURE_FILES: [&str; 4] = [
    "STARTUP_CHECKLIST.xml",
    "COMPLETION_CHECKLIST.xml",
    "AGENTS.md",
    "AGENTS-ts.md",
];
const AGENT_SESSION_STATUS_TIMESTAMP_FIELDS: [&str; 5] = [
    "closed_at",
    "resolved_at",
    "completed_at",
    "status_changed_at",
    "updated_at",
];
const STATE_CYCLE_PHASE_COMPLETED_AT_LABEL: &str = "docs/state.json cycle_phase.completed_at";
const STATE_LAST_CYCLE_TIMESTAMP_LABEL: &str = "docs/state.json last_cycle.timestamp";

#[derive(Parser)]
#[command(name = "write-entry")]
struct Cli {
    /// Repository root path
    #[arg(long, default_value = ".", global = true)]
    repo_root: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate a worklog entry file
    Worklog(WorklogArgs),
    /// Generate or append a journal entry file
    Journal(JournalArgs),
}

#[derive(Parser)]
struct WorklogArgs {
    /// Cycle number
    #[arg(long)]
    cycle: Option<u64>,
    /// Short descriptive name for heading and filename slug
    #[arg(long)]
    title: String,
    /// Read JSON payload from file instead of stdin
    #[arg(long)]
    input_file: Option<PathBuf>,
    /// What was done during the cycle
    #[arg(long = "done")]
    done: Vec<String>,
    /// Merged PR number
    #[arg(long = "pr-merged")]
    pr_merged: Vec<u64>,
    /// Reviewed PR number
    #[arg(long = "pr-reviewed")]
    pr_reviewed: Vec<u64>,
    /// Short description of an issue processed this cycle
    #[arg(long = "issue-processed")]
    issue_processed: Vec<String>,
    /// Issue numbers processed this cycle (comma-separated)
    #[arg(long = "issues-processed", value_delimiter = ',')]
    issues_processed: Vec<String>,
    /// Auto-derive processed issues from docs/state.json activity sources
    #[arg(long = "auto-issues", default_value_t = false)]
    auto_issues: bool,
    /// Auto-derive a previous-cycle review disposition summary for the worklog
    #[arg(long = "auto-review-summary", default_value_t = false)]
    auto_review_summary: bool,
    /// Review dispatch issue number to use for auto-review-summary
    #[arg(long = "review-issue")]
    review_issue: Option<u64>,
    /// Self-modification description, optionally in FILE:DESCRIPTION form
    #[arg(long = "self-modification")]
    self_modification: Vec<String>,
    /// Auto-derive self-modifications from infrastructure changes between cycle receipts
    #[arg(long = "auto-self-modifications", default_value_t = false)]
    auto_self_modifications: bool,
    /// Next step for the following cycle
    #[arg(long = "next")]
    next: Vec<String>,
    /// Auto-derive next steps from in-flight agent sessions in docs/state.json
    #[arg(long = "auto-next", default_value_t = false)]
    auto_next: bool,
    /// Pipeline summary for the current state section
    #[arg(long)]
    pipeline: Option<String>,
    /// Close-out gate failure descriptions (comma-separated)
    #[arg(long = "prior-gate-failures", value_delimiter = ',')]
    prior_gate_failures: Vec<String>,
    /// Auto-derive close-out gate failure history from docs/state.json
    #[arg(long = "auto-gate-history", default_value_t = false)]
    auto_gate_history: bool,
    /// Auto-derive pipeline summary from tools/pipeline-check
    #[arg(long = "auto-pipeline", default_value_t = false)]
    auto_pipeline: bool,
    /// Publish gate summary for the current state section
    #[arg(long = "publish-gate")]
    publish_gate: Option<String>,
    /// Commit receipt in TOOL:SHA form
    #[arg(long = "receipt")]
    receipt: Vec<String>,
    /// Auto-derive commit receipts from tools/cycle-receipts
    #[arg(long = "auto-receipts", default_value_t = false)]
    auto_receipts: bool,
    /// Auto-derive the receipt note prefix from receipt tool categories
    #[arg(long = "auto-receipt-note", default_value_t = false)]
    auto_receipt_note: bool,
    /// Render the generated worklog to stdout without writing a file
    #[arg(long = "dry-run", default_value_t = false)]
    dry_run: bool,
}

#[derive(Parser)]
struct JournalArgs {
    /// Cycle number
    #[arg(long)]
    cycle: Option<u64>,
    /// Entry title
    #[arg(long)]
    title: String,
    /// Read JSON payload from file instead of stdin
    #[arg(long)]
    input_file: Option<PathBuf>,
    /// Journal section in HEADING::BODY form
    #[arg(long = "section")]
    section: Vec<String>,
    /// Commitment for the next cycle
    #[arg(long = "commitment")]
    commitment: Vec<String>,
    /// Follow-through status for the previous cycle commitment
    #[arg(long = "previous-commitment-status")]
    previous_commitment_status: Option<String>,
    /// Follow-through detail for the previous cycle commitment
    #[arg(long = "previous-commitment-detail")]
    previous_commitment_detail: Option<String>,
}

#[derive(Debug)]
struct WorklogWriteOutcome {
    path: PathBuf,
    replaced_existing: bool,
}

#[derive(Debug, Deserialize)]
struct WorklogInput {
    #[serde(default)]
    what_was_done: Vec<String>,
    #[serde(default)]
    deferred_findings: Vec<DeferredFinding>,
    #[serde(default)]
    self_modifications: Vec<SelfModification>,
    #[serde(default)]
    prs_merged: Vec<u64>,
    #[serde(default)]
    prs_reviewed: Vec<u64>,
    #[serde(default, deserialize_with = "deserialize_issues_processed")]
    issues_processed: Vec<String>,
    current_state: CurrentState,
    #[serde(default)]
    next_steps: Vec<String>,
    #[serde(default)]
    receipts: Vec<CommitReceipt>,
    #[serde(default)]
    receipt_note: Option<String>,
}

#[derive(Debug, Deserialize)]
struct SelfModification {
    file: String,
    description: String,
}

#[derive(Debug, Deserialize)]
struct CurrentState {
    #[serde(default)]
    in_flight_sessions: u64,
    pipeline_status: String,
    #[serde(default)]
    prior_gate_failures: Vec<String>,
    #[serde(default)]
    publish_gate: String,
    #[serde(default)]
    preliminary: bool,
}

#[derive(Debug, Deserialize)]
struct JournalInput {
    #[serde(default = "default_previous_commitment_status")]
    previous_commitment_status: String,
    #[serde(default = "default_previous_commitment_detail")]
    previous_commitment_detail: String,
    #[serde(default)]
    sections: Vec<JournalSection>,
    #[serde(default)]
    concrete_behavior_change: String,
    #[serde(default)]
    commitments: Vec<String>,
    #[serde(default)]
    open_questions: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct JournalSection {
    heading: String,
    body: String,
}

#[derive(Clone, Debug, Deserialize)]
struct CommitReceipt {
    tool: String,
    receipt: String,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    unresolved: bool,
}

#[derive(Debug, Deserialize)]
struct CycleReceiptJsonEntry {
    #[serde(alias = "step")]
    tool: String,
    #[serde(alias = "hash")]
    receipt: String,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    _aliases: Vec<String>,
}

#[cfg(test)]
#[derive(Debug)]
struct GitHistoryEntry {
    full_sha: String,
    committed_at: DateTime<Utc>,
    subject: String,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum IssueProcessedValue {
    Number(u64),
    Text(String),
}

fn main() {
    let cli = Cli::parse();
    let repo_root = cli.repo_root;
    let now = Utc::now();

    match cli.command {
        Command::Worklog(args) => {
            if args.dry_run {
                match render_worklog_output(&args, &repo_root, now) {
                    Ok(content) => println!("{}", content),
                    Err(error) => {
                        eprintln!("Error: {}", error);
                        std::process::exit(1);
                    }
                }
            } else {
                match execute_worklog_with_outcome(&args, &repo_root, now) {
                    Ok(outcome) => {
                        if outcome.replaced_existing {
                            println!(
                                "Worklog updated: {} (replaced existing)",
                                outcome.path.display()
                            );
                        } else {
                            println!("Worklog created: {}", outcome.path.display());
                        }
                    }
                    Err(error) => {
                        eprintln!("Error: {}", error);
                        std::process::exit(1);
                    }
                }
            }
        }
        Command::Journal(args) => match execute_journal(&args, &repo_root, now) {
            Ok(path) => println!("{}", path.display()),
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        },
    }
}

fn read_stdin() -> Result<String, String> {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("failed to read stdin: {}", error))?;
    if input.trim().is_empty() {
        return Err("stdin JSON payload is required".to_string());
    }
    Ok(input)
}

fn read_input_file(path: &Path) -> Result<String, String> {
    let input = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    if input.trim().is_empty() {
        return Err(format!("input file {} is empty", path.display()));
    }
    Ok(input)
}

#[cfg_attr(not(test), allow(dead_code))]
fn execute_worklog(
    args: &WorklogArgs,
    repo_root: &Path,
    now: DateTime<Utc>,
) -> Result<PathBuf, String> {
    execute_worklog_with_outcome(args, repo_root, now).map(|outcome| outcome.path)
}

fn execute_worklog_with_outcome(
    args: &WorklogArgs,
    repo_root: &Path,
    now: DateTime<Utc>,
) -> Result<WorklogWriteOutcome, String> {
    let (cycle, content) = render_worklog_output_with_cycle(args, repo_root, now)?;
    let (path, replaced_existing) =
        if let Some(existing_path) = find_worklog_for_cycle(repo_root, cycle)? {
            (existing_path, true)
        } else {
            (worklog_path(repo_root, now, cycle, &args.title), false)
        };
    write_entry_file(&path, &content)?;
    Ok(WorklogWriteOutcome {
        path,
        replaced_existing,
    })
}

fn render_worklog_output(
    args: &WorklogArgs,
    repo_root: &Path,
    now: DateTime<Utc>,
) -> Result<String, String> {
    render_worklog_output_with_cycle(args, repo_root, now).map(|(_, content)| content)
}

fn render_worklog_output_with_cycle(
    args: &WorklogArgs,
    repo_root: &Path,
    now: DateTime<Utc>,
) -> Result<(u64, String), String> {
    let cycle = resolve_cycle(args.cycle, repo_root)?;
    let mut input = resolve_worklog_input_for_cycle(args, repo_root, cycle)?;
    emit_worklog_auto_derivation_warnings(apply_worklog_auto_derivations(
        args, repo_root, cycle, &mut input,
    )?);
    emit_unresolved_receipt_warnings(&mut input.receipts, repo_root)?;
    let content = render_worklog(cycle, now, &input);
    emit_generated_markdown_sha_warnings("worklog", &content, repo_root);
    Ok((cycle, content))
}

fn execute_journal(
    args: &JournalArgs,
    repo_root: &Path,
    now: DateTime<Utc>,
) -> Result<PathBuf, String> {
    let cycle = resolve_cycle(args.cycle, repo_root)?;
    let input = resolve_journal_input(args)?;
    let status = parse_commitment_status(&input.previous_commitment_status)?;
    let path = journal_path(repo_root, now);
    let previous = lookup_previous_concrete_behavior(repo_root, now.date_naive())?;
    if previous.is_some() && matches!(status, CommitmentStatus::NoPriorCommitment) {
        return Err("previous commitment found in journal but --previous-commitment-status is 'no_prior_commitment'; specify an explicit status (followed, not_followed, not_applicable)".to_string());
    }
    if previous.is_none() && !matches!(status, CommitmentStatus::NoPriorCommitment) {
        return Err(
            "--previous-commitment-status is set but no previous commitment found in journal history"
                .to_string(),
        );
    }
    if path.exists() {
        let existing_content = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
        if existing_journal_contains_cycle_entry(&existing_content, cycle) {
            return Err(format!(
                "journal file already contains an entry for cycle {} — refusing to append duplicate",
                cycle
            ));
        }
    }
    let worklog_link = find_worklog_relative_path(repo_root, cycle)?;
    let entry = sanitize_escaped_newlines(&render_journal_entry(
        cycle,
        now,
        &args.title,
        &input,
        status,
        previous.as_deref(),
        worklog_link.as_deref(),
    ));
    reject_duplicate_journal_section_headers(&entry)?;
    emit_generated_markdown_sha_warnings("journal", &entry, repo_root);
    write_journal_file(&path, now.date_naive(), &entry)?;
    update_journal_index(repo_root, now.date_naive(), cycle)?;
    Ok(path)
}

fn resolve_cycle(cycle: Option<u64>, repo_root: &Path) -> Result<u64, String> {
    match cycle {
        Some(cycle) => Ok(cycle),
        None => current_cycle_from_state(repo_root),
    }
}

#[cfg(test)]
fn resolve_worklog_input(args: &WorklogArgs, repo_root: &Path) -> Result<WorklogInput, String> {
    let cycle = resolve_cycle(args.cycle, repo_root)?;
    resolve_worklog_input_for_cycle(args, repo_root, cycle)
}

fn resolve_worklog_input_for_cycle(
    args: &WorklogArgs,
    repo_root: &Path,
    cycle: u64,
) -> Result<WorklogInput, String> {
    validate_worklog_flag_combinations(args)?;
    if let Some(path) = &args.input_file {
        if has_inline_worklog_content(args) {
            return Err(
                "cannot combine --input-file with inline worklog content flags".to_string(),
            );
        }
        let payload = read_input_file(path)?;
        return serde_json::from_str(&payload)
            .map_err(|error| format!("invalid worklog JSON input: {}", error));
    }

    if has_inline_worklog_content(args) {
        let state = load_worklog_state(repo_root, requires_worklog_state(args))?;
        let input = WorklogInput {
            what_was_done: args.done.clone(),
            deferred_findings: Vec::new(),
            self_modifications: parse_self_modifications(&args.self_modification)?,
            prs_merged: args.pr_merged.clone(),
            prs_reviewed: args.pr_reviewed.clone(),
            issues_processed: merge_issue_processed(
                &parse_issue_processed_numbers(&args.issues_processed)?,
                &parse_issue_processed(&args.issue_processed)?,
            ),
            current_state: CurrentState {
                in_flight_sessions: state_extra_in_flight_sessions(state.as_ref())?,
                pipeline_status: resolve_pipeline_status(args, repo_root, cycle, state.as_ref())?,
                prior_gate_failures: resolve_prior_gate_failures(args, repo_root, state.as_ref())?,
                publish_gate: match &args.publish_gate {
                    Some(value) => value.clone(),
                    None => state_publish_gate_status(state.as_ref())?,
                },
                preliminary: worklog_state_is_preliminary(state.as_ref(), cycle),
            },
            next_steps: resolve_next_steps(args, state.as_ref())?,
            receipts: parse_receipts(&args.receipt)?,
            receipt_note: None,
        };
        validate_worklog_state_placeholders(&input, state.as_ref())?;
        return Ok(input);
    }

    let state = load_worklog_state(repo_root, true)?;
    let input = WorklogInput {
        what_was_done: Vec::new(),
        deferred_findings: Vec::new(),
        self_modifications: Vec::new(),
        prs_merged: Vec::new(),
        prs_reviewed: Vec::new(),
        issues_processed: Vec::new(),
        current_state: CurrentState {
            in_flight_sessions: state_extra_in_flight_sessions(state.as_ref())?,
            pipeline_status: resolve_pipeline_status(args, repo_root, cycle, state.as_ref())?,
            prior_gate_failures: resolve_prior_gate_failures(args, repo_root, state.as_ref())?,
            publish_gate: state_publish_gate_status(state.as_ref())?,
            preliminary: worklog_state_is_preliminary(state.as_ref(), cycle),
        },
        next_steps: resolve_next_steps(args, state.as_ref())?,
        receipts: Vec::new(),
        receipt_note: None,
    };
    validate_worklog_state_placeholders(&input, state.as_ref())?;
    Ok(input)
}

fn validate_worklog_flag_combinations(args: &WorklogArgs) -> Result<(), String> {
    if args.auto_next && !args.next.is_empty() {
        return Err("cannot combine --auto-next with --next".to_string());
    }
    if args.auto_pipeline && args.pipeline.is_some() {
        return Err("cannot combine --auto-pipeline with --pipeline".to_string());
    }
    if args.auto_receipts && !args.receipt.is_empty() {
        return Err("cannot combine --auto-receipts with --receipt".to_string());
    }
    if args.auto_self_modifications && !args.self_modification.is_empty() {
        return Err(
            "cannot combine --auto-self-modifications with --self-modification".to_string(),
        );
    }
    Ok(())
}

fn requires_worklog_state(args: &WorklogArgs) -> bool {
    args.auto_next || args.auto_gate_history || args.publish_gate.is_none()
}

fn load_worklog_state(repo_root: &Path, required: bool) -> Result<Option<StateJson>, String> {
    let value = match read_state_value(repo_root) {
        Ok(value) => value,
        Err(error) if required => return Err(error),
        Err(_) => return Ok(None),
    };
    serde_json::from_value(value)
        .map(Some)
        .map_err(|error| format!("failed to parse docs/state.json: {}", error))
}

fn state_publish_gate_status(state: Option<&StateJson>) -> Result<String, String> {
    let state =
        state.ok_or_else(|| "docs/state.json is required to populate publish gate".to_string())?;
    state
        .publish_gate()?
        .status
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .ok_or_else(|| "missing publish_gate.status in state.json".to_string())
}

fn state_pipeline_status(state: Option<&StateJson>) -> String {
    state
        .and_then(|state| state.tool_pipeline.status.as_deref())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| NOT_PROVIDED.to_string())
}

fn resolve_pipeline_status(
    args: &WorklogArgs,
    repo_root: &Path,
    cycle: u64,
    state: Option<&StateJson>,
) -> Result<String, String> {
    if let Some(pipeline) = &args.pipeline {
        return Ok(pipeline.clone());
    }
    if args.auto_pipeline {
        return auto_pipeline_status(repo_root);
    }
    if let Some(frozen_status) = frozen_c5_5_pipeline_status(state, cycle) {
        return Ok(frozen_status);
    }
    Ok(state_pipeline_status(state))
}

fn frozen_c5_5_pipeline_status(state: Option<&StateJson>, cycle: u64) -> Option<String> {
    let gate = state?.tool_pipeline.extra.get("c5_5_gate")?;
    if !cycle_matches(gate, cycle) {
        return None;
    }
    if gate.get("needs_reverify").and_then(Value::as_bool) == Some(true) {
        return None;
    }

    let status = gate.get("status").and_then(Value::as_str).map(str::trim)?;
    match status {
        "PASS" | "FAIL" => gate
            .get("pipeline_summary")
            .or_else(|| gate.get("summary"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .or_else(|| Some(status.to_string())),
        _ => None,
    }
}

fn worklog_state_is_preliminary(state: Option<&StateJson>, cycle: u64) -> bool {
    frozen_c5_5_pipeline_status(state, cycle).is_none()
}

fn resolve_prior_gate_failures(
    args: &WorklogArgs,
    repo_root: &Path,
    state: Option<&StateJson>,
) -> Result<Vec<String>, String> {
    let auto_failures = if args.auto_gate_history {
        let cycle = resolve_cycle(args.cycle, repo_root)?;
        resolve_auto_gate_history(state, cycle)?
    } else {
        Vec::new()
    };
    let manual_failures = args
        .prior_gate_failures
        .iter()
        .map(|failure| failure.trim().to_string())
        .filter(|failure| !failure.is_empty())
        .collect::<Vec<_>>();
    Ok(merge_prior_gate_failures(&auto_failures, &manual_failures))
}

fn resolve_auto_gate_history(state: Option<&StateJson>, cycle: u64) -> Result<Vec<String>, String> {
    let state =
        state.ok_or_else(|| "docs/state.json is required to populate gate history".to_string())?;
    let Some(initial_result) = state.tool_pipeline.extra.get("c5_5_initial_result") else {
        return Ok(Vec::new());
    };
    let Some(gate) = state.tool_pipeline.extra.get("c5_5_gate") else {
        return Ok(Vec::new());
    };

    if !cycle_matches(initial_result, cycle) {
        return Ok(Vec::new());
    }
    if !cycle_matches(gate, cycle) {
        return Ok(Vec::new());
    }
    if initial_result
        .get("result")
        .and_then(Value::as_str)
        .map(str::trim)
        != Some("FAIL")
    {
        return Ok(Vec::new());
    }
    if gate.get("status").and_then(Value::as_str).map(str::trim) != Some("PASS") {
        return Ok(Vec::new());
    }

    let summary = initial_result
        .get("summary")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|summary| !summary.is_empty())
        .ok_or_else(|| {
            "missing tool_pipeline.c5_5_initial_result.summary in state.json".to_string()
        })?;

    Ok(vec![format!("C5.5 initial FAIL: {summary}")])
}

fn cycle_matches(value: &Value, cycle: u64) -> bool {
    value.get("cycle").and_then(Value::as_u64) == Some(cycle)
}

fn merge_prior_gate_failures(auto_failures: &[String], manual_failures: &[String]) -> Vec<String> {
    let mut merged: Vec<(String, String)> = Vec::new();
    for failure in auto_failures.iter().chain(manual_failures.iter()) {
        let prefix = prior_gate_failure_prefix(failure);
        if let Some(index) = merged
            .iter()
            .position(|(existing_prefix, _)| existing_prefix == &prefix)
        {
            merged[index] = (prefix, failure.clone());
        } else {
            merged.push((prefix, failure.clone()));
        }
    }
    merged.into_iter().map(|(_, failure)| failure).collect()
}

fn prior_gate_failure_prefix(failure: &str) -> String {
    failure
        .split_once(':')
        .map(|(prefix, _)| prefix)
        .unwrap_or(failure)
        .split_whitespace()
        .next()
        .filter(|prefix| !prefix.is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| failure.trim().to_string())
}

fn resolve_next_steps(
    args: &WorklogArgs,
    state: Option<&StateJson>,
) -> Result<Vec<String>, String> {
    if !args.next.is_empty() {
        return Ok(args.next.clone());
    }
    if args.auto_next {
        return auto_next_steps(state);
    }
    Ok(Vec::new())
}

fn auto_next_steps(state: Option<&StateJson>) -> Result<Vec<String>, String> {
    let state =
        state.ok_or_else(|| "docs/state.json is required to populate next steps".to_string())?;
    let mut next_steps = Vec::new();

    for finding in active_deferred_findings(&state.deferred_findings) {
        next_steps.push(format_deferred_finding_next_step(finding));
    }

    for session in &state.agent_sessions {
        if session.status.as_deref().map(str::trim) != Some("in_flight") {
            continue;
        }
        next_steps.push(format_in_flight_next_step(session)?);
    }

    if next_steps.is_empty() {
        next_steps.push("No in-flight sessions — plan next dispatch".to_string());
    }

    Ok(next_steps)
}

fn active_deferred_findings(
    findings: &[DeferredFinding],
) -> impl Iterator<Item = &DeferredFinding> {
    findings
        .iter()
        .filter(|finding| !finding.resolved && finding.dropped_rationale.is_none())
}

fn deferred_finding_deadline_text(finding: &DeferredFinding) -> String {
    format!(
        "deferred cycle {}, deadline cycle {}",
        finding.deferred_cycle, finding.deadline_cycle
    )
}

fn format_deferred_finding_summary_item(finding: &DeferredFinding) -> String {
    format!(
        "Deferred finding remains open: {} ({})",
        finding.category,
        deferred_finding_deadline_text(finding)
    )
}

fn format_deferred_finding_next_step(finding: &DeferredFinding) -> String {
    format!(
        "Address deferred finding: {} ({}) — must be actioned, dispatched, or explicitly dropped this cycle",
        finding.category,
        deferred_finding_deadline_text(finding)
    )
}

fn format_in_flight_next_step(session: &AgentSession) -> Result<String, String> {
    let issue = session
        .issue
        .ok_or_else(|| "agent_sessions[].issue is required for in-flight sessions when using --auto-next".to_string())
        .and_then(|value| {
            u64::try_from(value).map_err(|_| {
                "agent_sessions[].issue must be a positive integer for in-flight sessions when using --auto-next"
                    .to_string()
            })
        })?;
    let title = session
        .title
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!(" ({})", value))
        .unwrap_or_default();
    Ok(format!(
        "Review and iterate on PR from #{}{} when Copilot completes",
        issue, title
    ))
}

fn auto_pipeline_status(repo_root: &Path) -> Result<String, String> {
    let output = runner::run_tool(repo_root, "pipeline-check", &["--json"])?;
    let stdout = runner::stdout_text(&output);
    if let Ok(summary) =
        parse_pipeline_check_report(&stdout).map(|report| format_pipeline_status(&report))
    {
        return Ok(summary);
    }
    let stderr = runner::stderr_text(&output);
    let detail = match (stdout.is_empty(), stderr.is_empty()) {
        (false, false) => format!("stdout: {}; stderr: {}", stdout, stderr),
        (false, true) => format!("stdout: {}", stdout),
        (true, false) => stderr,
        (true, true) => "no output".to_string(),
    };
    Err(format!(
        "pipeline-check did not produce a parseable overall summary (exit {}): {}",
        output.status.code().unwrap_or(-1),
        detail
    ))
}

#[derive(Debug, Deserialize)]
struct PipelineCheckReport {
    overall: String,
    has_blocking_findings: bool,
    #[serde(default)]
    steps: Vec<PipelineCheckStep>,
}

#[derive(Debug, Deserialize)]
struct PipelineCheckStep {
    status: String,
    #[serde(default)]
    name: Option<String>,
}

fn parse_pipeline_check_report(stdout: &str) -> Result<PipelineCheckReport, String> {
    serde_json::from_str(stdout)
        .map_err(|error| format!("failed to parse pipeline-check JSON: {}", error))
}

fn format_pipeline_status(report: &PipelineCheckReport) -> String {
    let overall = report.overall.to_ascii_uppercase();
    let mut details = Vec::new();

    let warning_count = report
        .steps
        .iter()
        .filter(|step| step.status == "warn")
        .count();
    let cascade_count = report
        .steps
        .iter()
        .filter(|step| step.status == "cascade")
        .count();
    if warning_count > 0 {
        let suffix = if warning_count == 1 {
            "warning"
        } else {
            "warnings"
        };
        details.push(format!("{} {}", warning_count, suffix));
    }

    if cascade_count > 0 {
        let suffix = if cascade_count == 1 {
            "cascade"
        } else {
            "cascades"
        };
        details.push(format!("{} {}", cascade_count, suffix));
    }

    if report.has_blocking_findings {
        let blocking_steps: Vec<&str> = report
            .steps
            .iter()
            .filter(|step| step.status == "fail")
            .filter_map(|step| step.name.as_deref())
            .collect();
        if blocking_steps.is_empty() {
            details.push("blocking findings".to_string());
        } else {
            details.push(format!(
                "{} blocking: {}",
                blocking_steps.len(),
                blocking_steps.join(", ")
            ));
        }
    }

    if details.is_empty() {
        overall
    } else {
        format!("{} ({})", overall, details.join(", "))
    }
}

fn state_extra_in_flight_sessions(state: Option<&StateJson>) -> Result<u64, String> {
    let state = state.ok_or_else(|| {
        "docs/state.json is required to populate in-flight agent sessions".to_string()
    })?;
    let in_flight = state
        .extra
        .get("in_flight_sessions")
        .and_then(Value::as_u64)
        .ok_or_else(|| "missing in_flight_sessions in state.json".to_string())?;
    Ok(in_flight)
}

fn validate_worklog_state_placeholders(
    input: &WorklogInput,
    state: Option<&StateJson>,
) -> Result<(), String> {
    let Some(state) = state else {
        return Ok(());
    };

    if input.current_state.publish_gate == NOT_PROVIDED && state_has_publish_gate_status(state) {
        return Err("publish gate cannot be 'Not provided.' when docs/state.json contains publish_gate.status".to_string());
    }

    Ok(())
}

fn state_has_publish_gate_status(state: &StateJson) -> bool {
    state
        .publish_gate()
        .ok()
        .and_then(|publish_gate| publish_gate.status)
        .as_deref()
        .map(str::trim)
        .is_some_and(|value| !value.is_empty())
}

fn has_inline_worklog_content(args: &WorklogArgs) -> bool {
    !args.done.is_empty()
        || !args.pr_merged.is_empty()
        || !args.pr_reviewed.is_empty()
        || !args.issue_processed.is_empty()
        || !args.issues_processed.is_empty()
        || !args.self_modification.is_empty()
        || !args.next.is_empty()
        || args.auto_next
        || args.pipeline.is_some()
        || args.auto_gate_history
        || args.auto_pipeline
        || args.publish_gate.is_some()
        || !args.receipt.is_empty()
}

fn deserialize_issues_processed<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let values = Vec::<IssueProcessedValue>::deserialize(deserializer)?;
    Ok(values
        .into_iter()
        .map(|value| match value {
            IssueProcessedValue::Number(number) => format!("#{}", number),
            IssueProcessedValue::Text(text) => text,
        })
        .collect())
}

fn parse_issue_processed(values: &[String]) -> Result<Vec<String>, String> {
    values
        .iter()
        .map(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return Err("issue-processed description cannot be empty".to_string());
            }
            Ok(trimmed.to_string())
        })
        .collect()
}

fn parse_issue_processed_numbers(values: &[String]) -> Result<Vec<String>, String> {
    let mut issues = Vec::new();

    for value in values {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return Err("issues-processed entries must be non-empty issue numbers".to_string());
        }
        let issue = trimmed.parse::<u64>().map_err(|_| {
            format!(
                "issues-processed entry '{}' is not a valid issue number",
                trimmed
            )
        })?;
        if issue == 0 {
            return Err("issues-processed entry '0' is not a valid issue number".to_string());
        }
        issues.push(format!("#{}", issue));
    }

    Ok(issues)
}

fn emit_worklog_auto_derivation_warnings(warnings: Vec<String>) {
    for warning in warnings {
        eprintln!("{}", warning);
    }
}

fn apply_worklog_auto_derivations(
    args: &WorklogArgs,
    repo_root: &Path,
    cycle: u64,
    input: &mut WorklogInput,
) -> Result<Vec<String>, String> {
    let mut warnings = Vec::new();
    let state = match load_worklog_state(repo_root, args.auto_issues || args.auto_review_summary) {
        Ok(state) => state,
        Err(error) if !args.auto_issues && !args.auto_review_summary => {
            warnings.push(format!(
                "WARNING: failed to load docs/state.json for worklog auto-derivations: {}",
                error
            ));
            None
        }
        Err(error) => return Err(error),
    };

    let cycle_receipt_through = if args.auto_receipts || args.auto_self_modifications {
        cycle_receipt_boundary_timestamp(state.as_ref())?
    } else {
        None
    };

    let cycle_receipt_entries = if args.auto_receipts || args.auto_self_modifications {
        let receipt_flag = match (args.auto_receipts, args.auto_self_modifications) {
            (true, true) => "--auto-receipts/--auto-self-modifications",
            (true, false) => "--auto-receipts",
            (false, true) => "--auto-self-modifications",
            (false, false) => {
                unreachable!("receipt auto-derivation should only run when requested")
            }
        };
        Some(
            derive_cycle_receipt_entries(repo_root, cycle, cycle_receipt_through.as_deref())
                .map_err(|error| format!("{} failed: {}", receipt_flag, error))?,
        )
    } else {
        None
    };

    if args.auto_self_modifications {
        let entries = cycle_receipt_entries.as_ref().expect(
            "BUG: cycle_receipt_entries should be Some when auto_self_modifications is true",
        );
        input.self_modifications = derive_self_modifications_from_receipts(repo_root, entries)?;
    }

    if args.auto_issues {
        let state = state.as_ref().ok_or_else(|| {
            "docs/state.json not found; --auto-issues requires docs/state.json to be present"
                .to_string()
        })?;
        let auto_issues = derive_issue_processed_entries(cycle, state, &input.what_was_done)?;
        input.issues_processed = merge_issue_processed(&auto_issues, &input.issues_processed);
    }

    if args.auto_review_summary {
        let state = state.as_ref().ok_or_else(|| {
            "docs/state.json not found; --auto-review-summary requires docs/state.json to be present"
                .to_string()
        })?;
        let review_issue = resolve_review_issue_for_summary(args, state, cycle)?;
        input
            .what_was_done
            .insert(0, derive_review_summary_line(state, review_issue)?);
    }

    if args.auto_receipts {
        let entries = cycle_receipt_entries
            .as_ref()
            .expect("BUG: cycle_receipt_entries should be Some when auto_receipts is true");
        let receipts = cycle_receipt_entries_to_receipts(entries)?;
        input.receipts = receipts;
        let state = state.as_ref().ok_or_else(|| {
            "docs/state.json not found; --auto-receipts requires docs/state.json to derive merged PRs"
                .to_string()
        })?;
        let derived_prs = derive_prs_from_cycle_receipt_entries(state, cycle)?;
        input.prs_merged = merge_numbered_refs(&input.prs_merged, &derived_prs);
        input.receipt_note = Some(
            match derive_receipt_scope_note(
                cycle,
                Some(state),
                entries,
                cycle_receipt_through.as_deref(),
            ) {
                Ok(note) => note,
                Err(error) => {
                    warnings.push(format!(
                        "WARNING: failed to derive receipt scope note for cycle {}: {}",
                        cycle, error
                    ));
                    fallback_receipt_scope_note(cycle, entries, cycle_receipt_through.as_deref())
                }
            },
        );
    }

    if args.auto_receipt_note {
        let prefix = derive_receipt_note_prefix(&input.receipts)?;
        input.receipt_note = Some(match input.receipt_note.take() {
            Some(note) if !note.trim().is_empty() => format!("{prefix}. {note}"),
            _ => prefix,
        });
    }
    Ok(warnings)
}

fn merge_numbered_refs(existing: &[u64], derived: &[u64]) -> Vec<u64> {
    let mut seen = HashSet::new();
    let mut merged = Vec::new();

    for number in existing.iter().chain(derived.iter()) {
        if seen.insert(*number) {
            merged.push(*number);
        }
    }

    merged
}

fn merge_issue_processed(existing: &[String], derived: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut merged = Vec::new();

    for item in existing.iter().chain(derived.iter()) {
        let key = issue_processed_key(item);
        if seen.insert(key) {
            merged.push(item.clone());
        }
    }

    merged
}

fn issue_processed_key(item: &str) -> String {
    if let Some(reference) = extract_named_issue_reference(item, "QC") {
        return format!("qc:{}", reference);
    }
    if let Some(reference) = extract_named_issue_reference(item, "audit")
        .or_else(|| extract_named_issue_reference(item, "Audit"))
    {
        return format!("audit:{}", reference);
    }
    let references = extract_issue_references(item);
    if references.len() == 1 {
        format!("#{}", references[0])
    } else {
        item.trim().to_ascii_lowercase()
    }
}

fn extract_named_issue_reference(item: &str, label: &str) -> Option<String> {
    let trimmed = item.trim_start();
    let suffix = trimmed.strip_prefix(label)?.strip_prefix(" #")?;
    let digits = suffix
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect::<String>();
    if digits.is_empty() {
        None
    } else {
        Some(digits)
    }
}

fn derive_issue_processed_entries(
    cycle: u64,
    state: &StateJson,
    what_was_done: &[String],
) -> Result<Vec<String>, String> {
    let start = cycle_window_start(cycle, state, "--auto-issues")?;
    let audit_issue_numbers = derive_audit_issue_numbers(state);
    let mut seen = HashSet::new();
    let mut issues = Vec::new();

    for session in &state.agent_sessions {
        if !agent_session_had_activity_since(session, start)? {
            continue;
        }
        if let Some(issue) = session.issue.and_then(|value| u64::try_from(value).ok()) {
            push_issue_processed_entry(
                &mut issues,
                &mut seen,
                format_issue_processed_entry(issue, session.title.as_deref()),
            );
        }
        if let Some(qc_report) = session
            .extra
            .get("qc_report")
            .and_then(Value::as_i64)
            .and_then(|value| u64::try_from(value).ok())
        {
            push_issue_processed_entry(
                &mut issues,
                &mut seen,
                format_named_issue_processed_entry("QC", qc_report, session.title.as_deref()),
            );
        }
        if let Some(audit_inbound) = session
            .extra
            .get("audit_inbound")
            .and_then(Value::as_i64)
            .and_then(|value| u64::try_from(value).ok())
        {
            push_issue_processed_entry(
                &mut issues,
                &mut seen,
                format_named_issue_processed_entry(
                    "audit",
                    audit_inbound,
                    session.title.as_deref(),
                ),
            );
        }
        let mut session_audit_issue_numbers = Vec::new();
        if let Some(audit_inbound) = session
            .extra
            .get("audit_inbound")
            .and_then(Value::as_i64)
            .and_then(|value| u64::try_from(value).ok())
        {
            session_audit_issue_numbers.push(audit_inbound);
        }
        if let Some(note) = session.extra.get("note").and_then(Value::as_str) {
            push_issue_processed_references(
                &mut issues,
                &mut seen,
                note,
                &audit_issue_numbers,
                &session_audit_issue_numbers,
            );
        }
    }

    for issue in state
        .eva_input_issues
        .closed_this_cycle
        .iter()
        .filter_map(|value| u64::try_from(*value).ok())
    {
        push_issue_processed_entry(
            &mut issues,
            &mut seen,
            format_issue_processed_entry(issue, Some("Eva input closed this cycle")),
        );
    }

    derive_review_history_issue_processed_entries(
        cycle,
        state,
        &audit_issue_numbers,
        &mut issues,
        &mut seen,
    );

    for item in what_was_done {
        push_issue_processed_references(&mut issues, &mut seen, item, &audit_issue_numbers, &[]);
    }

    Ok(issues)
}

fn derive_audit_issue_numbers(state: &StateJson) -> HashSet<u64> {
    let mut audit_issue_numbers = state
        .pending_audit_implementations
        .iter()
        .filter_map(|recommendation| recommendation.audit_issue)
        .collect::<HashSet<_>>();

    let Some(recommendations) = state
        .extra
        .get("audit_tracking")
        .and_then(|value| value.get("recommendations"))
        .and_then(Value::as_array)
    else {
        return audit_issue_numbers;
    };

    for recommendation in recommendations {
        if let Some(audit_issue) = recommendation
            .get("audit_issue")
            .and_then(Value::as_u64)
            .or_else(|| {
                recommendation
                    .get("audit_issue")
                    .and_then(Value::as_i64)
                    .and_then(|value| u64::try_from(value).ok())
            })
        {
            audit_issue_numbers.insert(audit_issue);
        }
    }

    audit_issue_numbers
}

fn derive_review_history_issue_processed_entries(
    cycle: u64,
    state: &StateJson,
    audit_issue_numbers: &HashSet<u64>,
    issues: &mut Vec<String>,
    seen: &mut HashSet<String>,
) {
    let Some(history) = state
        .extra
        .get("review_agent")
        .and_then(|value| value.get("history"))
        .and_then(Value::as_array)
    else {
        return;
    };

    for entry in history {
        if entry.get("cycle").and_then(Value::as_u64) != Some(cycle) {
            continue;
        }

        push_issue_processed_numeric_field(issues, seen, entry, "issue");
        push_issue_processed_numeric_field(issues, seen, entry, "review_issue");

        if let Some(note) = entry.get("note").and_then(Value::as_str) {
            push_issue_processed_references(issues, seen, note, audit_issue_numbers, &[]);
        }

        let Some(finding_dispositions) =
            entry.get("finding_dispositions").and_then(Value::as_array)
        else {
            continue;
        };
        for disposition in finding_dispositions {
            push_issue_processed_numeric_field(issues, seen, disposition, "dispatch_issue");
            push_issue_processed_numeric_field(issues, seen, disposition, "issue");
            push_issue_processed_numeric_field(issues, seen, disposition, "review_issue");

            if let Some(note) = disposition.get("note").and_then(Value::as_str) {
                push_issue_processed_references(issues, seen, note, audit_issue_numbers, &[]);
            }
        }
    }
}

fn resolve_review_issue_for_summary(
    args: &WorklogArgs,
    state: &StateJson,
    cycle: u64,
) -> Result<u64, String> {
    args.review_issue
        .or_else(|| derive_previous_cycle_review_issue(state, cycle))
        .ok_or_else(|| {
            format!(
                "unable to resolve review dispatch issue for cycle {}; pass --review-issue <number> or ensure docs/state.json records the prior [Cycle Review] agent session",
                cycle
            )
        })
}

fn derive_previous_cycle_review_issue(state: &StateJson, cycle: u64) -> Option<u64> {
    let review_cycle = cycle.checked_sub(1)?;
    let expected_title = format!("[Cycle Review] Cycle {} end-of-cycle review", review_cycle);
    for session in state.agent_sessions.iter().rev() {
        let Some(title) = session.title.as_deref() else {
            continue;
        };
        if title != expected_title {
            continue;
        }
        return session.issue.and_then(|issue| u64::try_from(issue).ok());
    }

    let dispatch_reference_sources = [
        state.dispatch_log_latest.as_deref(),
        state
            .copilot_metrics
            .as_ref()
            .and_then(|metrics| metrics.dispatch_log_latest.as_deref()),
    ];
    for reference in dispatch_reference_sources.into_iter().flatten() {
        if !reference.contains(&expected_title) {
            continue;
        }
        if let Some(issue) = extract_issue_number_from_reference(reference) {
            return Some(issue);
        }
    }

    None
}

fn extract_issue_number_from_reference(value: &str) -> Option<u64> {
    let digits: String = value
        .trim()
        .strip_prefix('#')?
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        return None;
    }
    digits.parse().ok()
}

fn derive_review_summary_line(state: &StateJson, review_issue: u64) -> Result<String, String> {
    let review_agent = state.review_agent()?;
    let entry = review_agent
        .history
        .iter()
        .filter(|entry| review_history_entry_matches_review_issue(entry, review_issue))
        .max_by_key(|entry| entry.cycle)
        .ok_or_else(|| {
            format!(
                "review_agent.history has no entry matching review_issue {}; process-review must persist review_issue on history entries",
                review_issue
            )
        })?;
    let disposition_summary = summarize_review_dispositions(entry)?;

    Ok(format!(
        "Processed cycle {} review ({} findings, complacency {}/5, {})",
        entry.cycle, entry.finding_count, entry.complacency_score, disposition_summary
    ))
}

fn review_history_entry_matches_review_issue(
    entry: &ReviewHistoryEntry,
    review_issue: u64,
) -> bool {
    entry.extra.get("review_issue").and_then(Value::as_u64) == Some(review_issue)
}

fn summarize_review_dispositions(entry: &ReviewHistoryEntry) -> Result<String, String> {
    if entry.finding_count == 0 {
        return Ok("no findings".to_string());
    }
    if entry.finding_dispositions.len() as u64 != entry.finding_count {
        return Err(format!(
            "review history entry for cycle {} has finding_count {} but {} finding_dispositions",
            entry.cycle,
            entry.finding_count,
            entry.finding_dispositions.len()
        ));
    }

    let Some(first) = entry.finding_dispositions.first() else {
        return Err(format!(
            "review history entry for cycle {} has empty finding_dispositions despite non-zero finding_count",
            entry.cycle
        ));
    };

    if entry
        .finding_dispositions
        .iter()
        .all(|disposition| disposition.disposition == first.disposition)
    {
        return Ok(format!("all {}", first.disposition));
    }

    let mut counts = Vec::<(String, usize)>::new();
    for disposition in &entry.finding_dispositions {
        if let Some((_, count)) = counts
            .iter_mut()
            .find(|(name, _)| name == &disposition.disposition)
        {
            *count += 1;
        } else {
            counts.push((disposition.disposition.clone(), 1));
        }
    }

    Ok(counts
        .into_iter()
        .map(|(disposition, count)| format_count_with_label(count, &disposition))
        .flatten()
        .collect::<Vec<_>>()
        .join(", "))
}

fn push_issue_processed_numeric_field(
    issues: &mut Vec<String>,
    seen: &mut HashSet<String>,
    value: &Value,
    field: &str,
) {
    let Some(issue) = value.get(field).and_then(Value::as_u64) else {
        return;
    };
    push_issue_processed_entry(issues, seen, format_issue_processed_entry(issue, None));
}

fn push_issue_processed_references(
    issues: &mut Vec<String>,
    seen: &mut HashSet<String>,
    text: &str,
    audit_issue_numbers: &HashSet<u64>,
    additional_audit_issue_numbers: &[u64],
) {
    let explicit_audit_references = extract_inline_named_issue_references(text, "audit")
        .into_iter()
        .chain(extract_inline_named_issue_references(text, "Audit"))
        .collect::<HashSet<_>>();

    for issue in &explicit_audit_references {
        push_issue_processed_entry(
            issues,
            seen,
            format_named_issue_processed_entry("audit", *issue, None),
        );
    }

    for issue in extract_issue_references(text) {
        let item = if explicit_audit_references.contains(&issue)
            || additional_audit_issue_numbers.contains(&issue)
            || audit_issue_numbers.contains(&issue)
        {
            format_named_issue_processed_entry("audit", issue, None)
        } else {
            format_issue_processed_entry(issue, None)
        };
        push_issue_processed_entry(issues, seen, item);
    }
}

fn extract_inline_named_issue_references(text: &str, label: &str) -> Vec<u64> {
    let normalized_label = label.to_ascii_lowercase();
    let mut references = Vec::new();
    let mut tokens = text.split_whitespace();
    let Some(mut previous) = tokens.next() else {
        return references;
    };

    for current_token in tokens {
        let normalized_previous = previous
            .trim_matches(|character: char| !character.is_ascii_alphanumeric())
            .to_ascii_lowercase();
        if normalized_previous != normalized_label {
            previous = current_token;
            continue;
        }

        let reference =
            current_token.trim_start_matches(|character: char| !matches!(character, '#'));
        let Some(reference) = reference.strip_prefix('#') else {
            previous = current_token;
            continue;
        };
        let digits = reference
            .chars()
            .take_while(|character| character.is_ascii_digit())
            .collect::<String>();
        if let Ok(issue) = digits.parse::<u64>() {
            references.push(issue);
        }

        previous = current_token;
    }

    references
}

fn push_issue_processed_entry(issues: &mut Vec<String>, seen: &mut HashSet<String>, item: String) {
    let key = issue_processed_key(&item);
    if seen.insert(key) {
        issues.push(item);
    }
}

fn cycle_window_start(
    cycle: u64,
    state: &StateJson,
    context: &str,
) -> Result<DateTime<Utc>, String> {
    let state_cycle = state
        .cycle_phase
        .cycle
        .ok_or_else(|| format!("missing docs/state.json cycle_phase.cycle for {}", context))?;
    if state_cycle != cycle {
        return Err(format!(
            "docs/state.json cycle_phase.cycle {} does not match requested cycle {} for {}",
            state_cycle, cycle, context
        ));
    }
    state
        .cycle_phase
        .phase_entered_at
        .as_deref()
        .ok_or_else(|| {
            format!(
                "missing docs/state.json cycle_phase.phase_entered_at for {}",
                context
            )
        })
        .and_then(|value| parse_timestamp(value, "docs/state.json cycle_phase.phase_entered_at"))
}

fn agent_session_had_activity_since(
    session: &AgentSession,
    cycle_start: DateTime<Utc>,
) -> Result<bool, String> {
    if let Some(timestamp) = session.dispatched_at.as_deref() {
        if parse_timestamp(timestamp, "agent_sessions[].dispatched_at")? >= cycle_start {
            return Ok(true);
        }
    }
    if let Some(timestamp) = session.merged_at.as_deref() {
        if parse_timestamp(timestamp, "agent_sessions[].merged_at")? >= cycle_start {
            return Ok(true);
        }
    }
    if let Some(timestamp) = agent_session_status_changed_at(session)? {
        if timestamp >= cycle_start {
            return Ok(true);
        }
    }

    Ok(false)
}

fn agent_session_status_changed_at(
    session: &AgentSession,
) -> Result<Option<DateTime<Utc>>, String> {
    for key in AGENT_SESSION_STATUS_TIMESTAMP_FIELDS {
        let Some(value) = session.extra.get(key) else {
            continue;
        };
        let Some(timestamp) = value.as_str() else {
            return Err(format!(
                "agent_sessions[].{} must be a string timestamp",
                key
            ));
        };
        return parse_timestamp(timestamp, &format!("agent_sessions[].{}", key)).map(Some);
    }

    Ok(None)
}

fn format_issue_processed_entry(issue: u64, title: Option<&str>) -> String {
    match title.map(str::trim).filter(|value| !value.is_empty()) {
        Some(title) => format!("#{}: {}", issue, title),
        None => format!("#{}", issue),
    }
}

fn format_named_issue_processed_entry(label: &str, issue: u64, detail: Option<&str>) -> String {
    match detail.map(str::trim).filter(|value| !value.is_empty()) {
        Some(detail) => format!("{label} #{issue}: {detail}"),
        None => format!("{label} #{issue}"),
    }
}

fn cycle_receipt_boundary_timestamp(state: Option<&StateJson>) -> Result<Option<String>, String> {
    let Some(state) = state else {
        return Ok(None);
    };

    let completed_at = state
        .cycle_phase
        .completed_at
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| (STATE_CYCLE_PHASE_COMPLETED_AT_LABEL, value))
        .or_else(|| {
            state
                .last_cycle
                .timestamp
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .map(|value| (STATE_LAST_CYCLE_TIMESTAMP_LABEL, value))
        });

    let Some((label, timestamp)) = completed_at else {
        return Ok(None);
    };
    parse_timestamp(timestamp, label)?;
    Ok(Some(timestamp.to_string()))
}

fn derive_receipt_scope_note(
    cycle: u64,
    state: Option<&StateJson>,
    entries: &[CycleReceiptJsonEntry],
    cycle_receipt_through: Option<&str>,
) -> Result<String, String> {
    let mut scope_bits = Vec::new();

    if let Some(state) = state {
        if let Some(mode) = state
            .extra
            .get("project_mode")
            .and_then(|value| value.get("mode"))
            .and_then(Value::as_str)
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            scope_bits.push(format!("mode {mode}"));
        }

        if let Some(phase) = state
            .cycle_phase
            .phase
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            let mut phase_summary = format!("phase {phase}");
            if phase == "complete" {
                if let Some(completed_at) = state
                    .cycle_phase
                    .completed_at
                    .as_deref()
                    .map(str::trim)
                    .filter(|value| !value.is_empty())
                {
                    phase_summary.push_str(&format!(" (completed at {completed_at})"));
                }
            }
            scope_bits.push(phase_summary);
        }

        if let Some(agent_activity) = summarize_agent_session_activity(cycle, state)? {
            scope_bits.push(agent_activity);
        }
    }

    if let Some(event_summary) = summarize_receipt_events(entries) {
        scope_bits.push(event_summary);
    }

    let mut scope = match cycle_receipt_through {
        Some(timestamp) => format!("cycle {cycle} commits through {timestamp} (cycle-complete)"),
        None => format!("cycle {cycle} commits (unbounded)"),
    };
    if !scope_bits.is_empty() {
        scope.push_str(" — ");
        scope.push_str(&scope_bits.join("; "));
    }

    Ok(format_receipt_scope_note(
        cycle,
        &scope,
        cycle_receipt_through,
    ))
}

fn fallback_receipt_scope_note(
    cycle: u64,
    entries: &[CycleReceiptJsonEntry],
    cycle_receipt_through: Option<&str>,
) -> String {
    let mut scope = match cycle_receipt_through {
        Some(timestamp) => format!("cycle {cycle} commits through {timestamp} (cycle-complete)"),
        None => format!("cycle {cycle} commits (unbounded)"),
    };
    if let Some(event_summary) = summarize_receipt_events(entries) {
        scope.push_str(" — ");
        scope.push_str(&event_summary);
    }
    format_receipt_scope_note(cycle, &scope, cycle_receipt_through)
}

fn derive_receipt_note_prefix(receipts: &[CommitReceipt]) -> Result<String, String> {
    if receipts.is_empty() {
        return Err(
            "--auto-receipt-note requires at least one receipt to derive a category summary"
                .to_string(),
        );
    }

    let mut dispatches = 0usize;
    let mut merges = 0usize;
    let mut reviews = 0usize;
    let mut audits = 0usize;
    let mut eva_inputs = 0usize;

    for receipt in receipts {
        let tool = receipt.tool.trim().to_ascii_lowercase();
        if tool.starts_with("record-dispatch") {
            dispatches += 1;
        } else if tool.starts_with("process-merge") {
            merges += 1;
        } else if tool.starts_with("process-review") {
            reviews += 1;
        } else if tool.starts_with("process-audit") {
            audits += 1;
        } else if tool.starts_with("process-eva") {
            eva_inputs += 1;
        } else if tool.starts_with("cycle-start") || tool.starts_with("cycle-complete") {
            continue;
        }
    }

    let parts = [
        format_count_with_forms(dispatches, "dispatch", "dispatches"),
        format_count_with_forms(merges, "merge", "merges"),
        format_count_with_forms(reviews, "review", "reviews"),
        format_count_with_forms(audits, "audit", "audits"),
        format_count_with_forms(eva_inputs, "Eva input", "Eva inputs"),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    if parts.is_empty() {
        return Err(
            "--auto-receipt-note found no countable receipt categories in the receipt list"
                .to_string(),
        );
    }

    Ok(parts.join(", "))
}

fn cycle_receipts_scope_command(cycle: u64, cycle_receipt_through: Option<&str>) -> String {
    match cycle_receipt_through {
        Some(timestamp) => format!("cycle-receipts --cycle {cycle} --through {timestamp}"),
        None => format!("cycle-receipts --cycle {cycle}"),
    }
}

fn format_receipt_scope_note(
    cycle: u64,
    scope: &str,
    cycle_receipt_through: Option<&str>,
) -> String {
    let command = cycle_receipts_scope_command(cycle, cycle_receipt_through);
    format!("Scope: {scope}. Receipt table auto-generated by `{command}`.")
}

fn summarize_agent_session_activity(
    cycle: u64,
    state: &StateJson,
) -> Result<Option<String>, String> {
    let start = match cycle_window_start(cycle, state, "receipt scope derivation") {
        Ok(start) => start,
        Err(_) => return Ok(None),
    };
    let mut dispatches = 0usize;
    let mut merges = 0usize;
    let mut updates = 0usize;

    for session in &state.agent_sessions {
        if let Some(timestamp) = session.dispatched_at.as_deref() {
            if parse_timestamp(timestamp, "agent_sessions[].dispatched_at")? >= start {
                dispatches += 1;
            }
        }
        if let Some(timestamp) = session.merged_at.as_deref() {
            if parse_timestamp(timestamp, "agent_sessions[].merged_at")? >= start {
                merges += 1;
            }
        }
        if let Some(timestamp) = agent_session_status_changed_at(session)? {
            if timestamp >= start {
                updates += 1;
            }
        }
    }

    let summary = summarize_counts(
        "agent activity",
        [
            ("dispatch", dispatches),
            ("merge", merges),
            ("status update", updates),
        ],
    );
    Ok(summary)
}

fn summarize_receipt_events(entries: &[CycleReceiptJsonEntry]) -> Option<String> {
    let mut merges = 0usize;
    let mut dispatches = 0usize;
    let mut reviews = 0usize;

    for entry in entries {
        let tool = entry.tool.trim().to_ascii_lowercase();
        if tool == "process-merge" {
            merges += 1;
        }
        if tool == "record-dispatch" || tool == "dispatch-review" {
            dispatches += 1;
        }
        if tool == "process-review" {
            reviews += 1;
        }
    }

    summarize_counts(
        "receipt events",
        [
            ("dispatch", dispatches),
            ("merge", merges),
            ("review", reviews),
        ],
    )
}

fn summarize_counts<const N: usize>(label: &str, counts: [(&str, usize); N]) -> Option<String> {
    let parts = counts
        .into_iter()
        .filter(|(_, count)| *count > 0)
        .map(|(name, count)| format!("{} {}", count, pluralize(name, count)))
        .collect::<Vec<_>>();
    if parts.is_empty() {
        None
    } else {
        Some(format!("{label}: {}", parts.join(", ")))
    }
}

fn pluralize(noun: &str, count: usize) -> String {
    if count == 1 {
        noun.to_string()
    } else {
        format!("{noun}s")
    }
}

fn format_count_with_forms(count: usize, singular: &str, plural: &str) -> Option<String> {
    (count > 0).then(|| format!("{} {}", count, if count == 1 { singular } else { plural }))
}

fn format_count_with_label(count: usize, label: &str) -> Option<String> {
    (count > 0).then(|| format!("{count} {label}"))
}

fn extract_issue_references(item: &str) -> Vec<u64> {
    let mut issues = Vec::new();
    let bytes = item.as_bytes();
    let mut index = 0;

    while index < bytes.len() {
        if bytes[index] != b'#' {
            index += 1;
            continue;
        }

        let start = index + 1;
        let mut end = start;
        while end < bytes.len() && bytes[end].is_ascii_digit() {
            end += 1;
        }

        if end > start && !issue_reference_looks_like_pr(item, index) {
            if let Ok(issue) = item[start..end].parse::<u64>() {
                issues.push(issue);
            }
        }

        index = if end > start { end } else { index + 1 };
    }

    issues
}

fn issue_reference_looks_like_pr(item: &str, hash_index: usize) -> bool {
    let prefix = item[..hash_index].trim_end();
    let mut tokens = prefix.rsplit(|character: char| !character.is_ascii_alphabetic());
    tokens
        .find_map(|token| match token.to_ascii_lowercase().as_str() {
            "" => None,
            "pr" | "prs" => Some(true),
            "issue" | "issues" => Some(false),
            _ => None,
        })
        .unwrap_or(false)
}

fn derive_self_modifications_from_receipts(
    repo_root: &Path,
    entries: &[CycleReceiptJsonEntry],
) -> Result<Vec<SelfModification>, String> {
    let first_receipt = entries
        .first()
        .map(|entry| entry.receipt.trim())
        .filter(|receipt| !receipt.is_empty())
        .ok_or_else(|| "cycle-receipts returned no first receipt".to_string())?;
    let last_receipt = entries
        .last()
        .map(|entry| entry.receipt.trim())
        .filter(|receipt| !receipt.is_empty())
        .ok_or_else(|| "cycle-receipts returned no last receipt".to_string())?;
    let output = ProcessCommand::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(first_receipt)
        .arg(last_receipt)
        .arg("--")
        .args(INFRASTRUCTURE_ROOTS)
        .args(INFRASTRUCTURE_FILES)
        .current_dir(repo_root)
        .output()
        .map_err(|error| {
            format!(
                "failed to run git diff between {} and {} in {}: {}",
                first_receipt,
                last_receipt,
                repo_root.display(),
                error
            )
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("git diff failed: {}", stderr));
    }

    let diff = String::from_utf8(output.stdout)
        .map_err(|error| format!("failed to decode git diff output as UTF-8: {}", error))?;
    Ok(parse_infrastructure_self_modifications(&diff))
}

fn parse_infrastructure_self_modifications(diff_output: &str) -> Vec<SelfModification> {
    diff_output
        .lines()
        .map(str::trim)
        .filter(|path| !path.is_empty())
        .filter(|path| is_infrastructure_path(path))
        .map(|path| SelfModification {
            file: path.to_string(),
            description: "modified".to_string(),
        })
        .collect()
}

fn is_infrastructure_path(path: &str) -> bool {
    INFRASTRUCTURE_FILES.contains(&path)
        || INFRASTRUCTURE_ROOTS
            .iter()
            .any(|root| path == *root || path.starts_with(&format!("{root}/")))
}

#[cfg(test)]
fn find_cycle_start_commit(repo_root: &Path, cycle: u64) -> Result<String, String> {
    let commits = read_git_history(repo_root)?;
    if let Some(commit) = commits.iter().find(|commit| {
        commit.subject.starts_with("state(cycle-start):")
            && extract_cycle_tag(&commit.subject) == Some(cycle)
    }) {
        return Ok(commit.full_sha.clone());
    }

    find_first_commit_after_cycle_timestamp(repo_root, cycle, &commits)?.ok_or_else(|| {
        format!(
            "could not determine a cycle start commit for cycle {}; ensure history is available",
            cycle
        )
    })
}

#[cfg(test)]
fn read_git_history(repo_root: &Path) -> Result<Vec<GitHistoryEntry>, String> {
    let output = ProcessCommand::new("git")
        .arg("log")
        .arg("--date=iso-strict")
        .arg("--pretty=format:%H%x09%cI%x09%s")
        .arg("--reverse")
        .current_dir(repo_root)
        .output()
        .map_err(|error| {
            format!(
                "failed to read git history in {}: {}",
                repo_root.display(),
                error
            )
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("git log failed: {}", stderr));
    }

    let history = String::from_utf8(output.stdout)
        .map_err(|error| format!("failed to decode git log output as UTF-8: {}", error))?;
    history
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_git_history_line)
        .collect()
}

#[cfg(test)]
fn parse_git_history_line(line: &str) -> Result<GitHistoryEntry, String> {
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

    if full_sha.len() != 40 {
        return Err(format!("git sha must be 40 characters: {}", full_sha));
    }

    Ok(GitHistoryEntry {
        full_sha: full_sha.to_string(),
        committed_at: parse_timestamp(committed_at, "git commit timestamp")?,
        subject: subject.to_string(),
    })
}

#[cfg(test)]
fn find_first_commit_after_cycle_timestamp(
    repo_root: &Path,
    cycle: u64,
    commits: &[GitHistoryEntry],
) -> Result<Option<String>, String> {
    let current_cycle = current_cycle_from_state(repo_root)?;
    if cycle != current_cycle {
        return Ok(None);
    }

    let state = load_worklog_state(repo_root, true)?.ok_or_else(|| {
        "docs/state.json is required to resolve current cycle timestamp".to_string()
    })?;
    let timestamp = state
        .cycle_phase
        .phase_entered_at
        .as_deref()
        .ok_or_else(|| {
            "missing docs/state.json cycle_phase.phase_entered_at for current cycle; cannot derive self-modifications without a cycle-phase timestamp".to_string()
        })?;
    let cycle_start = parse_timestamp(timestamp, "docs/state.json cycle_phase.phase_entered_at")?;

    Ok(commits
        .iter()
        .find(|commit| commit.committed_at >= cycle_start)
        .map(|commit| commit.full_sha.clone()))
}

fn parse_timestamp(value: &str, label: &str) -> Result<DateTime<Utc>, String> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|error| format!("invalid {}: {}", label, error))
}

#[cfg(test)]
fn extract_cycle_tag(subject: &str) -> Option<u64> {
    let marker = "[cycle ";
    let start = subject.find(marker)?;
    let remainder = &subject[start + marker.len()..];
    let end = remainder.find(']')?;
    remainder[..end].trim().parse::<u64>().ok()
}

fn derive_cycle_receipt_entries(
    repo_root: &Path,
    cycle: u64,
    through: Option<&str>,
) -> Result<Vec<CycleReceiptJsonEntry>, String> {
    let cycle = cycle.to_string();
    let mut command = ProcessCommand::new("bash");
    command
        .arg("tools/cycle-receipts")
        .arg("--cycle")
        .arg(&cycle)
        .arg("--json");
    if let Some(timestamp) = through {
        command.arg("--through").arg(timestamp);
    }
    let output = command
        .arg("--repo-root")
        .arg(repo_root)
        .current_dir(repo_root)
        .output()
        .map_err(|error| {
            let through_fragment = through
                .map(|timestamp| format!(" --through {}", timestamp))
                .unwrap_or_default();
            format!(
                "failed to run bash tools/cycle-receipts --cycle {} --json{} --repo-root {} in {}: {}",
                cycle,
                through_fragment,
                repo_root.display(),
                repo_root.display(),
                error
            )
        })?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("cycle-receipts command failed: {}", stderr));
    }

    let json = String::from_utf8(output.stdout)
        .map_err(|error| format!("failed to decode cycle-receipts JSON as UTF-8: {}", error))?;
    let entries = parse_cycle_receipt_entries_output(&json)?;
    if entries.is_empty() {
        return Err(format!(
            "cycle-receipts returned no receipts for cycle {}",
            cycle
        ));
    }
    Ok(entries)
}

fn parse_cycle_receipt_entries_output(json: &str) -> Result<Vec<CycleReceiptJsonEntry>, String> {
    serde_json::from_str(json)
        .map_err(|error| format!("invalid cycle-receipts JSON output: {}", error))
}

fn cycle_receipt_entries_to_receipts(
    entries: &[CycleReceiptJsonEntry],
) -> Result<Vec<CommitReceipt>, String> {
    entries
        .iter()
        .map(|entry| {
            let receipt = format!("{}:{}", entry.tool.trim(), entry.receipt.trim());
            let mut parsed = parse_receipts(&[receipt])?;
            let mut parsed_receipt = parsed
                .pop()
                .ok_or_else(|| "parsed receipt unexpectedly empty".to_string())?;
            parsed_receipt.url = entry.url.clone().filter(|url| !url.trim().is_empty());
            Ok(parsed_receipt)
        })
        .collect()
}

fn derive_prs_from_cycle_receipt_entries(
    state: &StateJson,
    cycle: u64,
) -> Result<Vec<u64>, String> {
    let start = cycle_window_start(cycle, state, "receipt-backed PR derivation")?;
    let mut seen = HashSet::new();
    let mut prs = Vec::new();

    for (index, session) in state.agent_sessions.iter().enumerate() {
        let Some(merged_at) = session.merged_at.as_deref() else {
            continue;
        };
        if parse_timestamp(merged_at, "agent_sessions[].merged_at")? < start {
            continue;
        }
        let Some(pr) = session.pr else {
            continue;
        };
        let pr = u64::try_from(pr).map_err(|_| {
            format!(
                "agent_sessions[{}].pr must be a positive integer for receipt-backed PR derivation",
                index
            )
        })?;
        if pr == 0 {
            return Err(format!(
                "agent_sessions[{}].pr must be a positive integer for receipt-backed PR derivation",
                index
            ));
        }
        if seen.insert(pr) {
            prs.push(pr);
        }
    }

    Ok(prs)
}

fn parse_self_modifications(values: &[String]) -> Result<Vec<SelfModification>, String> {
    values
        .iter()
        .map(|value| {
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return Err("self-modification description cannot be empty".to_string());
            }
            if let Some((file, description)) = trimmed.split_once(':') {
                let file = file.trim();
                let description = description.trim();
                if !file.is_empty() && !description.is_empty() {
                    return Ok(SelfModification {
                        file: file.to_string(),
                        description: description.to_string(),
                    });
                }
                return Err(
                    "self-modification FILE:DESCRIPTION entries require both parts".to_string(),
                );
            }
            Ok(SelfModification {
                file: trimmed.to_string(),
                description: String::new(),
            })
        })
        .collect()
}

fn resolve_journal_input(args: &JournalArgs) -> Result<JournalInput, String> {
    if let Some(path) = &args.input_file {
        if has_inline_journal_content(args) {
            return Err(
                "cannot combine --input-file with inline journal content flags".to_string(),
            );
        }
        let payload = read_input_file(path)?;
        return serde_json::from_str(&payload)
            .map_err(|error| format!("invalid journal JSON input: {}", error));
    }

    if has_inline_journal_content(args) {
        if matches!(
            (
                args.previous_commitment_status.as_ref(),
                args.previous_commitment_detail.as_ref(),
            ),
            (Some(_), None) | (None, Some(_))
        ) {
            return Err(
                "previous-commitment override requires both --previous-commitment-status and --previous-commitment-detail".to_string(),
            );
        }

        return Ok(JournalInput {
            previous_commitment_status: args
                .previous_commitment_status
                .clone()
                .unwrap_or_else(default_previous_commitment_status),
            previous_commitment_detail: args
                .previous_commitment_detail
                .clone()
                .unwrap_or_else(default_previous_commitment_detail),
            sections: parse_sections(&args.section)?,
            concrete_behavior_change: String::new(),
            commitments: parse_commitments(&args.commitment),
            open_questions: Vec::new(),
        });
    }

    let payload = read_stdin()?;
    serde_json::from_str(&payload).map_err(|error| format!("invalid journal JSON input: {}", error))
}

fn has_inline_journal_content(args: &JournalArgs) -> bool {
    !args.section.is_empty() || !args.commitment.is_empty()
}

fn parse_receipts(values: &[String]) -> Result<Vec<CommitReceipt>, String> {
    values
        .iter()
        .map(|value| {
            let Some((tool, receipt)) = value.split_once(':') else {
                return Err(format!("invalid receipt '{}'; expected TOOL:SHA", value));
            };
            let tool = tool.trim();
            let receipt = receipt.trim();
            if tool.is_empty() || receipt.is_empty() {
                return Err(format!("invalid receipt '{}'; expected TOOL:SHA", value));
            }
            if !receipt.chars().all(|ch| ch.is_ascii_hexdigit()) {
                return Err(format!(
                    "invalid receipt '{}'; SHA must be hexadecimal",
                    value
                ));
            }
            if receipt.len() < 7 {
                return Err(format!(
                    "invalid receipt '{}'; SHA must be at least 7 hexadecimal characters",
                    value
                ));
            }
            Ok(CommitReceipt {
                tool: tool.to_string(),
                receipt: receipt.to_string(),
                url: None,
                unresolved: false,
            })
        })
        .collect()
}

fn emit_unresolved_receipt_warnings(
    receipts: &mut [CommitReceipt],
    repo_root: &Path,
) -> Result<(), String> {
    validate_receipt_shas(receipts, repo_root)
}

fn validate_receipt_shas(receipts: &mut [CommitReceipt], repo_root: &Path) -> Result<(), String> {
    for receipt in receipts {
        if git_commit_exists(repo_root, &receipt.receipt)? {
            continue;
        }

        return Err(format!(
            "unresolvable receipt SHA for {}: {}",
            receipt.tool, receipt.receipt
        ));
    }

    Ok(())
}

fn git_commit_exists(repo_root: &Path, sha: &str) -> Result<bool, String> {
    let output = std::process::Command::new("git")
        .arg("rev-parse")
        .arg("--verify")
        .arg(format!("{sha}^{{commit}}"))
        .current_dir(repo_root)
        .output()
        .map_err(|error| {
            format!(
                "failed to validate receipt SHA '{}' in {}: {}",
                sha,
                repo_root.display(),
                error
            )
        })?;

    Ok(output.status.success())
}

fn emit_generated_markdown_sha_warnings(entry_kind: &str, content: &str, repo_root: &Path) {
    match validate_generated_markdown_shas(entry_kind, content, repo_root) {
        Ok(warnings) => emit_worklog_auto_derivation_warnings(warnings),
        Err(error) => eprintln!(
            "WARNING: failed to validate generated {} commit references: {}",
            entry_kind, error
        ),
    }
}

fn validate_generated_markdown_shas(
    entry_kind: &str,
    content: &str,
    repo_root: &Path,
) -> Result<Vec<String>, String> {
    let mut warnings = Vec::new();
    for sha in find_git_sha_candidates(content) {
        if git_commit_exists(repo_root, &sha)? {
            continue;
        }
        warnings.push(format!(
            "WARNING: generated {} references unresolved commit SHA: {}",
            entry_kind, sha
        ));
    }
    Ok(warnings)
}

fn find_git_sha_candidates(content: &str) -> Vec<String> {
    let bytes = content.as_bytes();
    let mut index = 0;
    let mut seen = HashSet::new();
    let mut candidates = Vec::new();

    while index < bytes.len() {
        if !bytes[index].is_ascii_hexdigit() {
            index += 1;
            continue;
        }

        let start = index;
        while index < bytes.len() && bytes[index].is_ascii_hexdigit() {
            index += 1;
        }

        let candidate = &content[start..index];
        if (7..=40).contains(&candidate.len())
            && candidate
                .chars()
                .any(|character| matches!(character, 'a'..='f' | 'A'..='F'))
        {
            let key = candidate.to_ascii_lowercase();
            if seen.insert(key) {
                candidates.push(candidate.to_string());
            }
        }
    }

    candidates
}

fn parse_sections(values: &[String]) -> Result<Vec<JournalSection>, String> {
    values
        .iter()
        .map(|value| {
            let Some((heading, body)) = value.split_once("::") else {
                return Err(format!(
                    "invalid section '{}'; expected HEADING::BODY",
                    value
                ));
            };
            let heading = heading.trim();
            let body = body.trim();
            if heading.is_empty() || body.is_empty() {
                return Err(format!(
                    "invalid section '{}'; expected HEADING::BODY",
                    value
                ));
            }
            Ok(JournalSection {
                heading: heading.to_string(),
                body: body.to_string(),
            })
        })
        .collect()
}

fn parse_commitments(values: &[String]) -> Vec<String> {
    values
        .iter()
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .collect()
}

fn default_previous_commitment_status() -> String {
    "no_prior_commitment".to_string()
}

fn default_previous_commitment_detail() -> String {
    "No prior commitment recorded.".to_string()
}

fn worklog_path(repo_root: &Path, now: DateTime<Utc>, cycle: u64, title: &str) -> PathBuf {
    let date = now.format("%Y-%m-%d").to_string();
    let time = now.format("%H%M%S").to_string();
    let slug = slugify(strip_cycle_prefix(title));
    repo_root
        .join("docs")
        .join("worklog")
        .join(date)
        .join(format!("{}-cycle-{}-{}.md", time, cycle, slug))
}

fn journal_path(repo_root: &Path, now: DateTime<Utc>) -> PathBuf {
    repo_root
        .join("docs")
        .join("journal")
        .join(format!("{}.md", now.format("%Y-%m-%d")))
}

fn find_worklog_for_cycle(repo_root: &Path, cycle: u64) -> Result<Option<PathBuf>, String> {
    let worklog_root = repo_root.join("docs").join("worklog");
    if !worklog_root.exists() {
        return Ok(None);
    }

    let mut candidates = Vec::new();
    let date_entries = fs::read_dir(&worklog_root)
        .map_err(|error| format!("failed to read {}: {}", worklog_root.display(), error))?;
    for date_entry in date_entries {
        let date_entry = date_entry.map_err(|error| {
            format!(
                "failed to read entry in {}: {}",
                worklog_root.display(),
                error
            )
        })?;
        let date_path = date_entry.path();
        if !date_path.is_dir() {
            continue;
        }

        let file_entries = fs::read_dir(&date_path)
            .map_err(|error| format!("failed to read {}: {}", date_path.display(), error))?;
        for file_entry in file_entries {
            let file_entry = file_entry.map_err(|error| {
                format!("failed to read entry in {}: {}", date_path.display(), error)
            })?;
            let path = file_entry.path();
            if path.extension() != Some(OsStr::new("md")) {
                continue;
            }
            let content = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
            if content
                .lines()
                .next()
                .is_some_and(|line| line.starts_with(&format!("# Cycle {} — ", cycle)))
            {
                candidates.push(path);
            }
        }
    }

    candidates.sort();
    Ok(candidates.into_iter().last())
}

fn find_worklog_relative_path(repo_root: &Path, cycle: u64) -> Result<Option<String>, String> {
    find_worklog_for_cycle(repo_root, cycle)?
        .map(|path| {
            path.strip_prefix(repo_root)
                .map_err(|error| {
                    format!(
                        "failed to compute relative path for {}: {}",
                        path.display(),
                        error
                    )
                })
                .map(|relative| {
                    let normalized_path = relative.to_string_lossy().replace('\\', "/");
                    if let Some(path) = normalized_path.strip_prefix("docs/") {
                        format!("../{}", path)
                    } else {
                        normalized_path
                    }
                })
        })
        .transpose()
}

fn write_entry_file(path: &Path, content: &str) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("invalid output path {}", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("failed to create {}: {}", parent.display(), error))?;
    fs::write(path, content)
        .map_err(|error| format!("failed to write {}: {}", path.display(), error))
}

fn write_journal_file(path: &Path, date: NaiveDate, entry: &str) -> Result<bool, String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("invalid output path {}", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("failed to create {}: {}", parent.display(), error))?;

    if path.exists() {
        let mut existing = fs::read_to_string(path)
            .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
        if !existing.ends_with('\n') {
            existing.push('\n');
        }
        existing.push('\n');
        existing.push_str("---\n\n");
        existing.push_str(entry);
        fs::write(path, existing)
            .map_err(|error| format!("failed to write {}: {}", path.display(), error))?;
        Ok(false)
    } else {
        let header = format!("# Journal — {date}\n\n{JOURNAL_DESCRIPTION}\n\n---\n\n",);
        let content = format!("{header}{entry}");
        fs::write(path, content)
            .map_err(|error| format!("failed to write {}: {}", path.display(), error))?;
        Ok(true)
    }
}

fn existing_journal_contains_cycle_entry(existing_content: &str, cycle: u64) -> bool {
    existing_content.lines().any(|line| {
        journal_heading_cycle(line.trim()).is_some_and(|existing_cycle| existing_cycle == cycle)
    })
}

fn reject_duplicate_journal_section_headers(entry: &str) -> Result<(), String> {
    let mut seen_headers = HashSet::new();
    for line in entry.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("### ") && !seen_headers.insert(trimmed) {
            return Err(format!(
                "journal entry contains duplicate section header '{}' — refusing to write malformed entry",
                trimmed
            ));
        }
    }
    Ok(())
}

fn journal_heading_cycle(line: &str) -> Option<u64> {
    let heading = line.strip_prefix("## ")?;
    let (_, cycle_part) = heading.split_once(" — Cycle ")?;
    let (cycle, _) = cycle_part.split_once(':')?;
    cycle.trim().parse().ok()
}

fn update_journal_index(repo_root: &Path, date: NaiveDate, cycle: u64) -> Result<(), String> {
    let journal_index_path = repo_root.join("JOURNAL.md");
    if !journal_index_path.exists() {
        return Err(format!(
            "missing journal index file at {}",
            journal_index_path.display()
        ));
    }
    let content = fs::read_to_string(&journal_index_path)
        .map_err(|error| format!("failed to read {}: {}", journal_index_path.display(), error))?;
    let date_slug = date.format("%Y-%m-%d").to_string();
    let journal_relative_path = format!("docs/journal/{date_slug}.md");
    if content.contains(&journal_relative_path) {
        return Ok(());
    }

    let mut lines = content.lines().map(str::to_string).collect::<Vec<_>>();
    if let Some(previous_date) = lines
        .iter()
        .rev()
        .find_map(|line| open_journal_index_entry_date(line))
    {
        finalize_previous_journal_index_entry(repo_root, &mut lines, previous_date)?;
    }
    lines.push(format!(
        "- [{date_slug}]({journal_relative_path}) — Cycles {cycle}+"
    ));

    let updated = format!("{}\n", lines.join("\n"));
    fs::write(&journal_index_path, updated).map_err(|error| {
        format!(
            "failed to write {}: {}",
            journal_index_path.display(),
            error
        )
    })
}

fn finalize_previous_journal_index_entry(
    repo_root: &Path,
    lines: &mut [String],
    previous_date: NaiveDate,
) -> Result<(), String> {
    let Some(last_index) = lines.iter().rposition(|line| !line.trim().is_empty()) else {
        return Ok(());
    };

    let previous_date_slug = previous_date.format("%Y-%m-%d").to_string();
    let previous_relative_path = format!("docs/journal/{previous_date_slug}.md");
    if !lines[last_index].contains(&previous_relative_path) || !lines[last_index].ends_with('+') {
        return Ok(());
    }

    let previous_journal_path = repo_root
        .join("docs")
        .join("journal")
        .join(format!("{previous_date_slug}.md"));
    let highest_cycle = highest_cycle_in_journal_file(&previous_journal_path)?;
    lines[last_index] = replace_open_cycle_range(&lines[last_index], highest_cycle)?;
    Ok(())
}

fn open_journal_index_entry_date(line: &str) -> Option<NaiveDate> {
    let (date_part, rest) = line.strip_prefix("- [")?.split_once("](")?;
    let (path_part, cycles_part) = rest.split_once(") — Cycles ")?;
    let cycle_start = cycles_part.strip_suffix('+')?.parse::<u64>().ok()?;
    if cycle_start == 0 || path_part != format!("docs/journal/{date_part}.md") {
        return None;
    }
    NaiveDate::parse_from_str(date_part, "%Y-%m-%d").ok()
}

fn highest_cycle_in_journal_file(path: &Path) -> Result<u64, String> {
    let content = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;

    content
        .lines()
        .filter_map(journal_header_cycle_number)
        .max()
        .ok_or_else(|| format!("failed to find cycle header in {}", path.display()))
}

fn journal_header_cycle_number(line: &str) -> Option<u64> {
    if !line.trim_start().starts_with("## ") {
        return None;
    }

    let cycle_start = line.find("Cycle ")? + "Cycle ".len();
    let digits = line[cycle_start..]
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect::<String>();
    if digits.is_empty() {
        return None;
    }

    digits.parse().ok()
}

fn replace_open_cycle_range(line: &str, highest_cycle: u64) -> Result<String, String> {
    let marker = " — Cycles ";
    let marker_index = line
        .find(marker)
        .ok_or_else(|| format!("invalid journal index entry: {}", line))?;
    let prefix = &line[..marker_index + marker.len()];
    let open_range = line[marker_index + marker.len()..]
        .strip_suffix('+')
        .ok_or_else(|| format!("journal index entry is not open-ended: {}", line))?;
    let start_cycle = open_range
        .parse::<u64>()
        .map_err(|error| format!("invalid cycle range start in '{}': {}", line, error))?;
    if highest_cycle < start_cycle {
        return Err(format!(
            "highest cycle {} is lower than range start {} in '{}'",
            highest_cycle, start_cycle, line
        ));
    }

    Ok(format!("{prefix}{start_cycle}–{highest_cycle}"))
}

fn render_worklog(cycle: u64, now: DateTime<Utc>, input: &WorklogInput) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "# Cycle {} — {} {} UTC",
        cycle,
        now.format("%Y-%m-%d"),
        now.format("%H:%M")
    ));
    lines.push(String::new());
    lines.push("## What was done".to_string());
    lines.push(String::new());
    if input.what_was_done.is_empty() {
        if input.deferred_findings.is_empty() {
            lines.push("- None.".to_string());
        } else {
            for finding in active_deferred_findings(&input.deferred_findings) {
                lines.push(format!(
                    "- {}",
                    convert_references(&format_deferred_finding_summary_item(finding))
                ));
            }
        }
    } else {
        for item in &input.what_was_done {
            lines.push(format!("- {}", convert_references(item)));
        }
        for finding in active_deferred_findings(&input.deferred_findings) {
            lines.push(format!(
                "- {}",
                convert_references(&format_deferred_finding_summary_item(finding))
            ));
        }
    }
    lines.push(String::new());
    lines.push("### PRs merged".to_string());
    lines.push(String::new());
    lines.extend(render_numbered_refs(
        &input.prs_merged,
        "PR",
        PRIMARY_ISSUES_URL,
    ));
    lines.push(String::new());
    if !input.prs_reviewed.is_empty() {
        lines.push("### PRs reviewed".to_string());
        lines.push(String::new());
        lines.extend(render_numbered_refs(
            &input.prs_reviewed,
            "PR",
            PRIMARY_ISSUES_URL,
        ));
        lines.push(String::new());
    }
    lines.push(ISSUES_PROCESSED_HEADING.to_string());
    lines.push(String::new());
    lines.extend(render_bullet_list(&input.issues_processed));
    lines.push(String::new());
    lines.push("## Self-modifications".to_string());
    lines.push(String::new());
    if input.self_modifications.is_empty() {
        lines.push("- None.".to_string());
    } else {
        for item in &input.self_modifications {
            if item.description.trim().is_empty() {
                lines.push(format!("- {}", convert_references(&item.file)));
            } else {
                lines.push(format!(
                    "- **`{}`**: {}",
                    item.file,
                    convert_references(&item.description)
                ));
            }
        }
    }
    lines.push(String::new());
    lines.push(
        if input.current_state.preliminary {
            LEGACY_STATE_HEADING
        } else {
            CYCLE_STATE_HEADING
        }
        .to_string(),
    );
    lines.push(String::new());
    if input.current_state.preliminary {
        lines.push(LEGACY_STATE_DISCLAIMER.to_string());
        lines.push(String::new());
    }
    lines.push(format!(
        "{}{}",
        IN_FLIGHT_PREFIX, input.current_state.in_flight_sessions
    ));
    lines.push(format!(
        "{}{}",
        PIPELINE_STATUS_PREFIX,
        convert_references(&render_pipeline_status_summary(&input.current_state))
    ));
    for failure in &input.current_state.prior_gate_failures {
        lines.push(format!(
            "{}{}",
            CLOSE_OUT_GATE_FAILURES_PREFIX,
            convert_references(failure)
        ));
    }
    lines.push(format!(
        "{}{}",
        PUBLISH_GATE_PREFIX,
        convert_references(&input.current_state.publish_gate)
    ));
    lines.push(String::new());
    lines.push(NEXT_STEPS_HEADING.to_string());
    lines.push(String::new());
    if input.next_steps.is_empty() {
        lines.push("1. None.".to_string());
    } else {
        for (index, step) in input.next_steps.iter().enumerate() {
            lines.push(format!("{}. {}", index + 1, convert_references(step)));
        }
    }
    if !input.receipts.is_empty() {
        lines.push(String::new());
        lines.push("## Commit receipts".to_string());
        lines.push(String::new());
        if let Some(note) = &input.receipt_note {
            lines.push(format!("> Note: {}", note));
            lines.push(String::new());
        }
        lines.push("| Tool | Receipt | Link |".to_string());
        lines.push("|------|---------|------|".to_string());
        for receipt in &input.receipts {
            if receipt.unresolved {
                let receipt_display = format_receipt_display(receipt);
                lines.push(format!(
                    "| {} | {} | {} |",
                    receipt.tool, receipt_display, receipt_display
                ));
            } else {
                let receipt_display = format_receipt_display(receipt);
                let link_target = receipt
                    .url
                    .as_deref()
                    .filter(|url| !url.trim().is_empty())
                    .map(ToOwned::to_owned)
                    .unwrap_or_else(|| format!("{}/{}", PRIMARY_COMMITS_URL, receipt.receipt));
                let link_display = format!("[{}]({})", receipt.receipt, link_target);
                lines.push(format!(
                    "| {} | {} | {} |",
                    receipt.tool, receipt_display, link_display
                ));
            }
        }
    }
    lines.push(String::new());
    lines.join("\n")
}

fn render_pipeline_status_summary(current_state: &CurrentState) -> String {
    let pipeline_status = current_state.pipeline_status.trim();
    if current_state.prior_gate_failures.is_empty() || !pipeline_status.starts_with("PASS") {
        return current_state.pipeline_status.clone();
    }

    let mut details = current_state
        .prior_gate_failures
        .iter()
        .map(|failure| summarize_prior_gate_failure(failure))
        .collect::<Vec<_>>();
    details.push("resolved by re-running close-out after fixes".to_string());

    format!("FAIL→PASS ({})", details.join("; "))
}

fn summarize_prior_gate_failure(failure: &str) -> String {
    if let Some(reason) = failure.strip_prefix("C4.1 FAIL:").map(str::trim) {
        return format!("C4.1 initially failed: {}", reason);
    }
    if let Some(reason) = failure.strip_prefix("C5.5 FAIL:").map(str::trim) {
        return format!("C5.5 initially failed: {}", reason);
    }
    if let Some(reason) = failure.strip_prefix("C5.5 initial FAIL:").map(str::trim) {
        return format!("C5.5 initially failed: {}", reason);
    }
    format!("blocking gate initially failed: {}", failure.trim())
}

fn format_receipt_display(receipt: &CommitReceipt) -> String {
    if receipt.unresolved {
        format!("{} [UNRESOLVED]", receipt.receipt)
    } else {
        receipt.receipt.clone()
    }
}

fn render_numbered_refs(numbers: &[u64], kind: &str, issues_url: &str) -> Vec<String> {
    if numbers.is_empty() {
        return vec!["- None.".to_string()];
    }

    numbers
        .iter()
        .map(|number| match kind {
            "PR" => format!("- [PR #{}]({}/{})", number, issues_url, number),
            "issue" => format!("- [#{}]({}/{})", number, issues_url, number),
            _ => format!("- [{} #{}]({}/{})", kind, number, issues_url, number),
        })
        .collect()
}

fn render_bullet_list(items: &[String]) -> Vec<String> {
    if items.is_empty() {
        return vec!["- None.".to_string()];
    }

    items
        .iter()
        .map(|item| format!("- {}", convert_references(item)))
        .collect()
}

#[derive(Clone, Copy)]
enum CommitmentStatus {
    Followed,
    NotFollowed,
    NotApplicable,
    NoPriorCommitment,
}

fn parse_commitment_status(value: &str) -> Result<CommitmentStatus, String> {
    match value {
		"followed" => Ok(CommitmentStatus::Followed),
		"not_followed" => Ok(CommitmentStatus::NotFollowed),
		"not_applicable" => Ok(CommitmentStatus::NotApplicable),
		"no_prior_commitment" => Ok(CommitmentStatus::NoPriorCommitment),
		_ => Err(format!(
			"invalid previous_commitment_status '{}'; expected one of: followed, not_followed, not_applicable, no_prior_commitment",
			value
		)),
	}
}

fn render_journal_entry(
    cycle: u64,
    now: DateTime<Utc>,
    title: &str,
    input: &JournalInput,
    status: CommitmentStatus,
    previous_commitment: Option<&str>,
    worklog_relative_path: Option<&str>,
) -> String {
    let title = strip_cycle_prefix(title);
    let mut lines = Vec::new();
    lines.push(format!(
        "## {} — Cycle {}: {}",
        now.format("%Y-%m-%d"),
        cycle,
        title
    ));
    lines.push(String::new());
    if let Some(path) = worklog_relative_path {
        lines.push(format!("Worklog: [cycle {}]({})", cycle, path));
        lines.push(String::new());
    }
    lines.push("### Context".to_string());
    lines.push(String::new());
    lines.push(format!(
        "Cycle {} focused on {}.",
        cycle,
        convert_references(title)
    ));
    lines.push(String::new());
    lines.push("### Previous commitment follow-through".to_string());
    lines.push(String::new());
    if let Some(previous) = previous_commitment {
        lines.push(format!(
            "> Previous commitment: {}",
            convert_references(previous)
        ));
        lines.push(String::new());
    }
    lines.push(format!(
        "{} {}",
        commitment_status_label(status),
        convert_references(&input.previous_commitment_detail)
    ));
    lines.push(String::new());
    for section in &input.sections {
        lines.push(format!("### {}", convert_references(&section.heading)));
        lines.push(String::new());
        lines.push(convert_references(&section.body));
        lines.push(String::new());
    }
    lines.push("### Concrete commitments for next cycle".to_string());
    lines.push(String::new());
    let commitments = journal_commitments(input);
    if commitments.is_empty() {
        lines.push("1. None.".to_string());
    } else {
        for (index, commitment) in commitments.iter().enumerate() {
            lines.push(format!("{}. {}", index + 1, convert_references(commitment)));
        }
    }
    lines.push(String::new());
    lines.push("### Open questions".to_string());
    lines.push(String::new());
    if input.open_questions.is_empty() {
        lines.push("- None.".to_string());
    } else {
        for question in &input.open_questions {
            lines.push(format!("- {}", convert_references(question)));
        }
    }
    lines.push(String::new());
    lines.join("\n")
}

fn sanitize_escaped_newlines(text: &str) -> String {
    text.replace("\\n", "\n")
}

fn strip_cycle_prefix(title: &str) -> &str {
    let trimmed = title.trim();
    let Some(remainder) = trimmed
        .strip_prefix("Cycle ")
        .or_else(|| trimmed.strip_prefix("cycle "))
    else {
        return title;
    };
    let digits_length = remainder
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .count();
    if digits_length == 0 {
        return title;
    }
    let suffix = &remainder[digits_length..];
    if let Some(rest) = suffix
        .strip_prefix(':')
        .or_else(|| suffix.strip_prefix('-'))
    {
        return rest.trim_start();
    }
    if suffix.is_empty() || suffix.chars().next().is_some_and(char::is_whitespace) {
        return suffix.trim_start();
    }
    title
}

fn journal_commitments(input: &JournalInput) -> Vec<&str> {
    if !input.commitments.is_empty() {
        return input
            .commitments
            .iter()
            .map(String::as_str)
            .filter(|value| !value.trim().is_empty())
            .collect();
    }

    if input.concrete_behavior_change.trim().is_empty() {
        Vec::new()
    } else {
        vec![input.concrete_behavior_change.as_str()]
    }
}

fn commitment_status_label(status: CommitmentStatus) -> &'static str {
    match status {
        CommitmentStatus::Followed => "**Followed.**",
        CommitmentStatus::NotFollowed => "**Not followed.**",
        CommitmentStatus::NotApplicable => "**Not applicable.**",
        CommitmentStatus::NoPriorCommitment => "**No prior commitment.**",
    }
}

fn lookup_previous_concrete_behavior(
    repo_root: &Path,
    today: NaiveDate,
) -> Result<Option<String>, String> {
    let journal_dir = repo_root.join("docs").join("journal");
    if !journal_dir.exists() {
        return Ok(None);
    }

    let mut dated_files = Vec::<(NaiveDate, PathBuf)>::new();
    let entries = fs::read_dir(&journal_dir)
        .map_err(|error| format!("failed to read {}: {}", journal_dir.display(), error))?;
    for entry in entries {
        let entry = entry.map_err(|error| {
            format!(
                "failed to read entry in {}: {}",
                journal_dir.display(),
                error
            )
        })?;
        let path = entry.path();
        if path.extension() != Some(OsStr::new("md")) {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(OsStr::to_str) else {
            continue;
        };
        let Ok(date) = NaiveDate::parse_from_str(stem, "%Y-%m-%d") else {
            continue;
        };
        if date <= today {
            dated_files.push((date, path));
        }
    }

    dated_files.sort_by(|a, b| a.0.cmp(&b.0));
    for (_, path) in dated_files.into_iter().rev() {
        let content = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
        if let Some(section) = extract_last_concrete_behavior(&content) {
            return Ok(Some(section));
        }
    }
    Ok(None)
}

fn extract_last_concrete_behavior(content: &str) -> Option<String> {
    let mut line_starts = vec![0usize];
    for (idx, ch) in content.char_indices() {
        if ch == '\n' {
            line_starts.push(idx + 1);
        }
    }

    let mut latest: Option<String> = None;
    for (line_index, start) in line_starts.iter().enumerate() {
        let line = line_text(content, *start);
        let trimmed_line = line.trim();
        if trimmed_line != "### Concrete behavior change this cycle"
            && trimmed_line != "### Concrete commitments for next cycle"
        {
            continue;
        }
        let mut end = content.len();
        for next_start in line_starts.iter().skip(line_index + 1) {
            let next_line = line_text(content, *next_start);
            let trimmed = next_line.trim();
            if trimmed.starts_with("### ") || trimmed == "---" {
                end = *next_start;
                break;
            }
        }
        let block_start = line_end_index(content, *start);
        let section = content[block_start..end].trim();
        if !section.is_empty() {
            latest = Some(section.to_string());
        }
    }
    latest
}

fn line_text(content: &str, start: usize) -> &str {
    let rest = &content[start..];
    match rest.find('\n') {
        Some(index) => &rest[..index],
        None => rest,
    }
}

fn line_end_index(content: &str, start: usize) -> usize {
    let rest = &content[start..];
    match rest.find('\n') {
        Some(index) => start + index + 1,
        None => content.len(),
    }
}

fn slugify(title: &str) -> String {
    let mut output = String::new();
    let mut in_hyphen = false;
    for ch in title.chars() {
        let mapped = ch.to_ascii_lowercase();
        if mapped.is_ascii_alphanumeric() {
            output.push(mapped);
            in_hyphen = false;
        } else if !in_hyphen {
            output.push('-');
            in_hyphen = true;
        }
    }
    let slug = output.trim_matches('-').to_string();
    if slug.is_empty() {
        "entry".to_string()
    } else {
        slug
    }
}

fn convert_references(text: &str) -> String {
    let link_spans = markdown_link_spans(text);
    let mut output = String::new();
    let mut cursor = 0usize;
    for (start, end) in link_spans {
        if cursor < start {
            output.push_str(&convert_segment(&text[cursor..start]));
        }
        output.push_str(&text[start..end]);
        cursor = end;
    }
    if cursor < text.len() {
        output.push_str(&convert_segment(&text[cursor..]));
    }
    output
}

fn markdown_link_spans(text: &str) -> Vec<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut spans = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] != b'[' {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < bytes.len() && bytes[j] != b']' {
            j += 1;
        }
        if j + 1 >= bytes.len() || bytes[j + 1] != b'(' {
            i += 1;
            continue;
        }
        let mut k = j + 2;
        while k < bytes.len() && bytes[k] != b')' {
            k += 1;
        }
        if k >= bytes.len() {
            i += 1;
            continue;
        }
        spans.push((i, k + 1));
        i = k + 1;
    }
    spans
}

fn convert_segment(segment: &str) -> String {
    let chars: Vec<char> = segment.chars().collect();
    let mut output = String::new();
    let mut i = 0usize;

    while i < chars.len() {
        if let Some((replacement, next)) =
            match_named_reference(&chars, i, "PR", PRIMARY_ISSUES_URL)
        {
            output.push_str(&replacement);
            i = next;
            continue;
        }
        if let Some((replacement, next)) = match_named_reference(&chars, i, "QC", QC_ISSUES_URL) {
            output.push_str(&replacement);
            i = next;
            continue;
        }
        if let Some((replacement, next)) =
            match_named_reference(&chars, i, "audit", AUDIT_ISSUES_URL)
        {
            output.push_str(&replacement);
            i = next;
            continue;
        }
        if let Some((replacement, next)) =
            match_named_reference(&chars, i, "Audit", AUDIT_ISSUES_URL)
        {
            output.push_str(&replacement);
            i = next;
            continue;
        }
        if chars[i] == '#' {
            let prev = i.checked_sub(1).and_then(|idx| chars.get(idx)).copied();
            if prev != Some('[') && !is_embedded_reference_prefix(prev) {
                let (digits, end) = parse_digits(&chars, i + 1);
                if !digits.is_empty() {
                    let next_char = chars.get(end).copied();
                    if next_char != Some(']') {
                        output
                            .push_str(&format!("[#{}]({}/{})", digits, PRIMARY_ISSUES_URL, digits));
                        i = end;
                        continue;
                    }
                }
            }
        }
        output.push(chars[i]);
        i += 1;
    }

    output
}

fn is_embedded_reference_prefix(ch: Option<char>) -> bool {
    matches!(ch, Some(value) if value.is_ascii_alphanumeric() || matches!(value, '/' | '-' | '_'))
}

fn match_named_reference(
    chars: &[char],
    start: usize,
    label: &str,
    base_url: &str,
) -> Option<(String, usize)> {
    let mut idx = start;
    for expected in label.chars() {
        if chars.get(idx).copied()? != expected {
            return None;
        }
        idx += 1;
    }
    if chars.get(idx).copied()? != ' ' || chars.get(idx + 1).copied()? != '#' {
        return None;
    }
    let (digits, end) = parse_digits(chars, idx + 2);
    if digits.is_empty() {
        return None;
    }
    let replacement = format!("[{} #{}]({}/{})", label, digits, base_url, digits);
    Some((replacement, end))
}

fn parse_digits(chars: &[char], start: usize) -> (String, usize) {
    let mut idx = start;
    let mut digits = String::new();
    while let Some(ch) = chars.get(idx) {
        if ch.is_ascii_digit() {
            digits.push(*ch);
            idx += 1;
        } else {
            break;
        }
    }
    (digits, idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::process::Command as ProcessCommand;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct TempRepoDir {
        path: PathBuf,
    }

    impl TempRepoDir {
        fn new(prefix: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "write-entry-{}-{}-{}-{}",
                prefix,
                std::process::id(),
                nanos,
                run_id
            ));
            fs::create_dir_all(&path).unwrap();
            fs::create_dir_all(path.join("docs")).unwrap();
            fs::write(
                path.join("docs/state.json"),
                r#"{
  "last_cycle": {"number": 154},
  "cycle_phase": {"cycle": 154},
  "agent_sessions": [],
  "in_flight_sessions": 0
}
"#,
            )
            .unwrap();
            Self { path }
        }
    }

    impl Drop for TempRepoDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn write_root_journal_index(repo_root: &Path, body: &str) {
        fs::write(
            repo_root.join("JOURNAL.md"),
            format!(
                "# Journal\n\nJournal entries have been split into per-date files in [`docs/journal/`](docs/journal/).\n\n{}",
                body
            ),
        )
        .unwrap();
    }

    fn fixed_now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-03-06T05:14:58Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    fn fixed_now_on(date: &str) -> DateTime<Utc> {
        DateTime::parse_from_rfc3339(&format!("{date}T05:14:58Z"))
            .unwrap()
            .with_timezone(&Utc)
    }

    fn worklog_args(title: &str) -> WorklogArgs {
        WorklogArgs {
            cycle: Some(154),
            title: title.to_string(),
            input_file: None,
            done: Vec::new(),
            pr_merged: Vec::new(),
            pr_reviewed: Vec::new(),
            issue_processed: Vec::new(),
            issues_processed: Vec::new(),
            auto_issues: false,
            auto_review_summary: false,
            review_issue: None,
            self_modification: Vec::new(),
            auto_self_modifications: false,
            next: Vec::new(),
            auto_next: false,
            pipeline: None,
            prior_gate_failures: Vec::new(),
            auto_gate_history: false,
            auto_pipeline: false,
            publish_gate: None,
            receipt: Vec::new(),
            auto_receipts: false,
            auto_receipt_note: false,
            dry_run: false,
        }
    }

    fn journal_args(title: &str) -> JournalArgs {
        JournalArgs {
            cycle: Some(154),
            title: title.to_string(),
            input_file: None,
            section: Vec::new(),
            commitment: Vec::new(),
            previous_commitment_status: None,
            previous_commitment_detail: None,
        }
    }

    fn run_git(repo_root: &Path, args: &[&str]) -> String {
        let output = ProcessCommand::new("git")
            .args(args)
            .current_dir(repo_root)
            .env("GIT_AUTHOR_NAME", "Test User")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "Test User")
            .env("GIT_COMMITTER_EMAIL", "test@example.com")
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).unwrap().trim().to_string()
    }

    fn init_git_repo(repo_root: &Path) {
        run_git(repo_root, &["init"]);
        run_git(repo_root, &["branch", "-m", "master"]);
    }

    fn create_git_commit(repo_root: &Path, file_name: &str, content: &str) -> String {
        create_git_commit_with_message(repo_root, file_name, content, &format!("Add {}", file_name))
    }

    fn create_git_commit_with_message(
        repo_root: &Path,
        file_name: &str,
        content: &str,
        message: &str,
    ) -> String {
        if let Some(parent) = repo_root.join(file_name).parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(repo_root.join(file_name), content).unwrap();
        run_git(repo_root, &["add", file_name]);
        run_git(repo_root, &["commit", "-m", message]);
        run_git(repo_root, &["rev-parse", "--short=7", "HEAD"])
    }

    fn create_git_commit_at(
        repo_root: &Path,
        file_name: &str,
        content: &str,
        message: &str,
        timestamp: &str,
    ) -> String {
        if let Some(parent) = repo_root.join(file_name).parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(repo_root.join(file_name), content).unwrap();
        run_git(repo_root, &["add", file_name]);
        let output = ProcessCommand::new("git")
            .args(["commit", "-m", message])
            .current_dir(repo_root)
            .env("GIT_AUTHOR_NAME", "Test User")
            .env("GIT_AUTHOR_EMAIL", "test@example.com")
            .env("GIT_COMMITTER_NAME", "Test User")
            .env("GIT_COMMITTER_EMAIL", "test@example.com")
            .env("GIT_AUTHOR_DATE", timestamp)
            .env("GIT_COMMITTER_DATE", timestamp)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git commit -m {:?} failed: {}",
            message,
            String::from_utf8_lossy(&output.stderr)
        );
        run_git(repo_root, &["rev-parse", "--short=7", "HEAD"])
    }

    fn write_cycle_receipts_script(repo_root: &Path, json: &str) {
        let script_path = repo_root.join("tools").join("cycle-receipts");
        fs::create_dir_all(script_path.parent().unwrap()).unwrap();
        fs::write(
            script_path,
            format!("#!/usr/bin/env bash\nset -euo pipefail\ncat <<'JSON'\n{json}\nJSON\n"),
        )
        .unwrap();
    }

    fn write_cycle_receipts_script_with_args_log(repo_root: &Path, json: &str, args_log: &Path) {
        let script_path = repo_root.join("tools").join("cycle-receipts");
        fs::create_dir_all(script_path.parent().unwrap()).unwrap();
        fs::write(
            script_path,
            format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$@\" > '{}'\ncat <<'JSON'\n{json}\nJSON\n",
                args_log.display()
            ),
        )
        .unwrap();
    }

    fn write_pipeline_check_script(repo_root: &Path, body: &str) {
        let script_path = repo_root.join("tools").join("pipeline-check");
        fs::create_dir_all(script_path.parent().unwrap()).unwrap();
        fs::write(
            script_path,
            format!("#!/usr/bin/env bash\nset -euo pipefail\n{body}\n"),
        )
        .unwrap();
    }

    fn write_worklog_fixture(
        repo_root: &Path,
        now: DateTime<Utc>,
        cycle: u64,
        title: &str,
    ) -> PathBuf {
        let path = worklog_path(repo_root, now, cycle, title);
        write_entry_file(
            &path,
            &format!(
                "# Cycle {} — {} {} UTC\n",
                cycle,
                now.format("%Y-%m-%d"),
                now.format("%H:%M")
            ),
        )
        .unwrap();
        path
    }

    fn write_input_file(repo_root: &Path, name: &str, payload: &str) -> PathBuf {
        let path = repo_root.join(name);
        fs::write(&path, payload).unwrap();
        path
    }

    fn write_state_file(repo_root: &Path, payload: &str) {
        fs::create_dir_all(repo_root.join("docs")).expect("failed to create docs directory");
        let mut value: Value = serde_json::from_str(payload).expect("test state JSON should parse");
        if value.get("in_flight_sessions").is_none() {
            let in_flight = value
                .get("agent_sessions")
                .and_then(Value::as_array)
                .map(|sessions| {
                    sessions
                        .iter()
                        .filter(|session| {
                            session.get("status").and_then(Value::as_str) == Some("in_flight")
                        })
                        .count() as u64
                })
                .unwrap_or(0);
            value["in_flight_sessions"] = json!(in_flight);
        }
        fs::write(
            repo_root.join("docs/state.json"),
            serde_json::to_string_pretty(&value).expect("test state JSON should serialize"),
        )
        .expect("failed to write test state.json");
    }

    #[test]
    fn converts_issue_references_and_preserves_existing_links() {
        let input = "Refs: #42, PR #10, QC #11, audit #12, finding EvaLok/schema-org-json-ld#1, [#13](https://github.com/EvaLok/schema-org-json-ld/issues/13)";
        let output = convert_references(input);
        assert!(output.contains("[#42](https://github.com/EvaLok/schema-org-json-ld/issues/42)"));
        assert!(output.contains("[PR #10](https://github.com/EvaLok/schema-org-json-ld/issues/10)"));
        assert!(
            output.contains("[QC #11](https://github.com/EvaLok/schema-org-json-ld-qc/issues/11)")
        );
        assert!(output
            .contains("[audit #12](https://github.com/EvaLok/schema-org-json-ld-audit/issues/12)"));
        assert!(convert_references("Audit #14")
            .contains("[Audit #14](https://github.com/EvaLok/schema-org-json-ld-audit/issues/14)"));
        assert!(output.contains("finding EvaLok/schema-org-json-ld#1"));
        assert!(!output.contains("finding EvaLok/schema-org-json-ld[#1]"));
        assert!(output.contains("[#13](https://github.com/EvaLok/schema-org-json-ld/issues/13)"));
        assert_eq!(
            output
                .matches("[#13](https://github.com/EvaLok/schema-org-json-ld/issues/13)")
                .count(),
            1
        );
    }

    #[test]
    fn worklog_path_uses_date_time_cycle_and_slug() {
        let repo_root = PathBuf::from("/tmp/example");
        let path = worklog_path(
            &repo_root,
            fixed_now(),
            403,
            "From Convention to Enforcement",
        );
        assert_eq!(
            path,
            PathBuf::from(
                "/tmp/example/docs/worklog/2026-03-06/051458-cycle-403-from-convention-to-enforcement.md"
            )
        );
    }

    #[test]
    fn worklog_path_strips_redundant_cycle_prefix_from_slug() {
        let repo_root = PathBuf::from("/tmp/example");
        let uppercase = worklog_path(
            &repo_root,
            fixed_now(),
            403,
            "Cycle 403: Three merges review tool audit",
        );
        let lowercase = worklog_path(
            &repo_root,
            fixed_now(),
            403,
            "cycle 403 three merges review tool audit",
        );

        assert_eq!(
            uppercase,
            PathBuf::from(
                "/tmp/example/docs/worklog/2026-03-06/051458-cycle-403-three-merges-review-tool-audit.md"
            )
        );
        assert_eq!(uppercase, lowercase);
        assert!(!uppercase.to_string_lossy().contains("cycle-403-cycle-403"));
    }

    #[test]
    fn worklog_template_keeps_required_section_order() {
        let input = WorklogInput {
            what_was_done: vec!["Fixed #42".to_string()],
            deferred_findings: Vec::new(),
            self_modifications: vec![SelfModification {
                file: "STARTUP_CHECKLIST.xml".to_string(),
                description: "Updated per audit #117".to_string(),
            }],
            prs_merged: vec![537],
            prs_reviewed: vec![543],
            issues_processed: vec!["Closed #546".to_string()],
            current_state: CurrentState {
                in_flight_sessions: 2,
                pipeline_status: "5/5 phases pass".to_string(),
                prior_gate_failures: Vec::new(),
                publish_gate: "Source diverged".to_string(),
                preliminary: false,
            },
            next_steps: vec!["Review PR #543".to_string()],
            receipts: Vec::new(),
            receipt_note: None,
        };
        let rendered = render_worklog(154, fixed_now(), &input);
        let what_done = rendered.find("## What was done").unwrap();
        let self_mods = rendered.find("## Self-modifications").unwrap();
        let current = rendered.find("## Cycle state").unwrap();
        let next = rendered.find("## Next steps").unwrap();
        assert!(what_done < self_mods);
        assert!(self_mods < current);
        assert!(current < next);
        assert!(!rendered
            .contains("*Snapshot before review dispatch — final counters may differ after C6.*"));
        assert!(rendered.contains("[#42](https://github.com/EvaLok/schema-org-json-ld/issues/42)"));
        assert!(rendered.contains(
            "[audit #117](https://github.com/EvaLok/schema-org-json-ld-audit/issues/117)"
        ));
    }

    #[test]
    fn worklog_template_renders_plain_self_modification_when_description_empty() {
        let input = WorklogInput {
            what_was_done: Vec::new(),
            deferred_findings: Vec::new(),
            self_modifications: vec![SelfModification {
                file: "Updated AGENTS.md".to_string(),
                description: String::new(),
            }],
            prs_merged: Vec::new(),
            prs_reviewed: Vec::new(),
            issues_processed: Vec::new(),
            current_state: CurrentState {
                in_flight_sessions: 0,
                pipeline_status: NOT_PROVIDED.to_string(),
                prior_gate_failures: Vec::new(),
                publish_gate: NOT_PROVIDED.to_string(),
                preliminary: false,
            },
            next_steps: Vec::new(),
            receipts: Vec::new(),
            receipt_note: None,
        };

        let rendered = render_worklog(154, fixed_now(), &input);
        assert!(rendered.contains("\n## Self-modifications\n\n- Updated AGENTS.md\n"));
        assert!(!rendered.contains("**`Updated AGENTS.md`**:"));
    }

    #[test]
    fn worklog_deferred_finding_summary_and_next_steps_share_deadline_text() {
        let finding = DeferredFinding {
            category: "worklog-accuracy".to_string(),
            deferred_cycle: 456,
            deadline_cycle: 461,
            resolved: false,
            resolved_ref: None,
            dropped_rationale: None,
        };
        let summary_item = format_deferred_finding_summary_item(&finding);
        let next_step = format_deferred_finding_next_step(&finding);
        let deadline_text = deferred_finding_deadline_text(&finding);
        let input = WorklogInput {
            what_was_done: Vec::new(),
            deferred_findings: vec![finding],
            self_modifications: Vec::new(),
            prs_merged: Vec::new(),
            prs_reviewed: Vec::new(),
            issues_processed: Vec::new(),
            current_state: CurrentState {
                in_flight_sessions: 0,
                pipeline_status: "PASS (3 warnings)".to_string(),
                prior_gate_failures: Vec::new(),
                publish_gate: "clear".to_string(),
                preliminary: false,
            },
            next_steps: vec![next_step.clone()],
            receipts: Vec::new(),
            receipt_note: None,
        };

        let rendered = render_worklog(154, fixed_now(), &input);

        assert!(rendered.contains(&summary_item));
        assert!(rendered.contains(&next_step));
        assert_eq!(summary_item.matches(&deadline_text).count(), 1);
        assert_eq!(next_step.matches(&deadline_text).count(), 1);
    }

    #[test]
    fn worklog_template_omits_close_out_gate_failures_when_none_provided() {
        let input = WorklogInput {
            what_was_done: Vec::new(),
            deferred_findings: Vec::new(),
            self_modifications: Vec::new(),
            prs_merged: Vec::new(),
            prs_reviewed: Vec::new(),
            issues_processed: Vec::new(),
            current_state: CurrentState {
                in_flight_sessions: 0,
                pipeline_status: "PASS (3 warnings)".to_string(),
                prior_gate_failures: Vec::new(),
                publish_gate: "clear".to_string(),
                preliminary: false,
            },
            next_steps: Vec::new(),
            receipts: Vec::new(),
            receipt_note: None,
        };

        let rendered = render_worklog(154, fixed_now(), &input);

        assert!(!rendered.contains(CLOSE_OUT_GATE_FAILURES_PREFIX));
    }

    #[test]
    fn worklog_template_renders_close_out_gate_failures_after_pipeline_status_lines() {
        let input = WorklogInput {
            what_was_done: Vec::new(),
            deferred_findings: Vec::new(),
            self_modifications: Vec::new(),
            prs_merged: Vec::new(),
            prs_reviewed: Vec::new(),
            issues_processed: Vec::new(),
            current_state: CurrentState {
                in_flight_sessions: 0,
                pipeline_status: "PASS (3 warnings)".to_string(),
                prior_gate_failures: vec!["C4.1 FAIL: pipeline status mismatch".to_string()],
                publish_gate: "clear".to_string(),
                preliminary: false,
            },
            next_steps: Vec::new(),
            receipts: Vec::new(),
            receipt_note: None,
        };

        let rendered = render_worklog(154, fixed_now(), &input);
        let pipeline = rendered
            .find("- **Pipeline status**: FAIL→PASS (C4.1 initially failed: pipeline status mismatch; resolved by re-running close-out after fixes)")
            .unwrap();
        let gate_failure = rendered
            .find("- **Close-out gate failures**: C4.1 FAIL: pipeline status mismatch")
            .unwrap();
        let publish_gate = rendered.find("- **Publish gate**: clear").unwrap();

        assert!(pipeline < gate_failure);
        assert!(gate_failure < publish_gate);
    }

    #[test]
    fn execute_worklog_renders_multiple_close_out_gate_failures() {
        let repo_root = TempRepoDir::new("worklog-prior-gate-failures");
        let mut args = worklog_args("Gate failures");
        args.done = vec!["Regenerated worklog after gate failures".to_string()];
        args.pipeline = Some("PASS (3 warnings)".to_string());
        args.prior_gate_failures = vec![
            "C4.1 FAIL: mismatch".to_string(),
            "C5.5 FAIL: doc-validation".to_string(),
        ];
        args.publish_gate = Some("clear".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("- **Close-out gate failures**: C4.1 FAIL: mismatch"));
        assert!(content.contains("- **Close-out gate failures**: C5.5 FAIL: doc-validation"));
    }

    #[test]
    fn auto_gate_history_reads_c5_5_initial_failure() {
        let repo_root = TempRepoDir::new("worklog-auto-gate-history");
        write_state_file(
            &repo_root.path,
            r#"{
  "tool_pipeline": {
    "c5_5_initial_result": {
      "cycle": 154,
      "result": "FAIL",
      "summary": "PASS (1 blocking warning, 2 warnings)"
    },
    "c5_5_gate": {
      "cycle": 154,
      "status": "PASS"
    }
  }
}"#,
        );

        let mut args = worklog_args("Auto gate history");
        args.auto_gate_history = true;
        args.pipeline = Some("PASS (2 warnings)".to_string());
        args.publish_gate = Some("clear".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains(
            "- **Close-out gate failures**: C5.5 initial FAIL: PASS (1 blocking warning, 2 warnings)"
        ));
    }

    #[test]
    fn auto_gate_history_merges_with_cli_input() {
        let repo_root = TempRepoDir::new("worklog-auto-gate-history-merge");
        write_state_file(
            &repo_root.path,
            r#"{
  "tool_pipeline": {
    "c5_5_initial_result": {
      "cycle": 154,
      "result": "FAIL",
      "summary": "PASS (1 blocking warning, 2 warnings)"
    },
    "c5_5_gate": {
      "cycle": 154,
      "status": "PASS"
    }
  }
}"#,
        );

        let mut args = worklog_args("Merged gate history");
        args.auto_gate_history = true;
        args.prior_gate_failures = vec![
            "C4.1 FAIL: mismatch".to_string(),
            "C5.5 FAIL: manual detail".to_string(),
        ];
        args.pipeline = Some("PASS (2 warnings)".to_string());
        args.publish_gate = Some("clear".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("- **Close-out gate failures**: C4.1 FAIL: mismatch"));
        assert!(content.contains("- **Close-out gate failures**: C5.5 FAIL: manual detail"));
        assert!(!content.contains("C5.5 initial FAIL: PASS (1 blocking warning, 2 warnings)"));
    }

    #[test]
    fn auto_gate_history_empty_when_no_failures() {
        let repo_root = TempRepoDir::new("worklog-auto-gate-history-empty");
        write_state_file(
            &repo_root.path,
            r#"{
  "tool_pipeline": {
    "c5_5_gate": {
      "cycle": 154,
      "status": "PASS"
    }
  }
}"#,
        );

        let mut args = worklog_args("No gate history");
        args.auto_gate_history = true;
        args.pipeline = Some("PASS (2 warnings)".to_string());
        args.publish_gate = Some("clear".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(!content.contains(CLOSE_OUT_GATE_FAILURES_PREFIX));
    }

    #[test]
    fn auto_gate_history_ignores_stale_cycle() {
        let repo_root = TempRepoDir::new("worklog-auto-gate-history-stale-cycle");
        write_state_file(
            &repo_root.path,
            r#"{
  "tool_pipeline": {
    "c5_5_initial_result": {
      "cycle": 153,
      "result": "FAIL",
      "summary": "PASS (1 blocking warning, 2 warnings)"
    },
    "c5_5_gate": {
      "cycle": 154,
      "status": "PASS"
    }
  }
}"#,
        );

        let mut args = worklog_args("Stale gate history");
        args.auto_gate_history = true;
        args.pipeline = Some("PASS (2 warnings)".to_string());
        args.publish_gate = Some("clear".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(!content.contains(CLOSE_OUT_GATE_FAILURES_PREFIX));
    }

    #[test]
    fn parse_self_modifications_supports_structured_entries() {
        let modifications =
            parse_self_modifications(&["AGENTS.md: Updated guidance".to_string()]).unwrap();

        assert_eq!(modifications.len(), 1);
        assert_eq!(modifications[0].file, "AGENTS.md");
        assert_eq!(modifications[0].description, "Updated guidance");
    }

    #[test]
    fn parse_self_modifications_rejects_incomplete_structured_entries() {
        let error = parse_self_modifications(&["AGENTS.md:".to_string()]).unwrap_err();
        assert!(error.contains("FILE:DESCRIPTION"));
    }

    #[test]
    fn parse_issue_processed_rejects_empty_descriptions() {
        let error = parse_issue_processed(&["   ".to_string()]).unwrap_err();
        assert!(error.contains("issue-processed description cannot be empty"));
    }

    #[test]
    fn parse_issue_processed_numbers_validates_split_input() {
        assert_eq!(
            parse_issue_processed_numbers(&["42".to_string(), "77".to_string(), "105".to_string()])
                .unwrap(),
            vec!["#42", "#77", "#105"]
        );

        let error = parse_issue_processed_numbers(&["nope".to_string()]).unwrap_err();
        assert_eq!(
            error,
            "issues-processed entry 'nope' is not a valid issue number"
        );

        let zero_error = parse_issue_processed_numbers(&["0".to_string()]).unwrap_err();
        assert_eq!(
            zero_error,
            "issues-processed entry '0' is not a valid issue number"
        );
    }

    #[test]
    fn worklog_inline_flags_auto_populate_status_from_state() {
        let repo_root = TempRepoDir::new("worklog-auto-populate");
        init_git_repo(&repo_root.path);
        let receipt = create_git_commit(&repo_root.path, "notes/auto-populate.txt", "auto\n");
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[{{"step":"manual","receipt":"{receipt}","commit":"Add notes/auto-populate.txt"}}]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "in_flight_sessions": 3,
                "copilot_metrics": {
                    "total_dispatches": 45,
                    "produced_pr": 42,
                    "merged": 40,
                    "pr_merge_rate": "88.9%",
                    "in_flight": 3
                },
                "publish_gate": {
                    "status": "published"
                }
            }"#,
        );

        let mut args = worklog_args("Auto populate");
        args.cycle = None;
        args.done = vec!["Merged PR #123".to_string()];

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(!content.contains("### PRs reviewed"));
        assert!(content.contains("### Issues processed\n\n- None."));
        assert!(content.contains("## Self-modifications\n\n- None."));
        assert!(content.contains("## Pre-dispatch state"));
        assert!(content.contains(LEGACY_STATE_DISCLAIMER));
        assert!(content.contains("- **Pipeline status**: Not provided."));
        assert!(content.contains("- **In-flight agent sessions**: 3"));
        assert!(!content.contains("- **Copilot metrics**:"));
        assert!(content.contains("- **Publish gate**: published"));
        assert!(!content.contains("post-dispatch"));
    }

    #[test]
    fn worklog_uses_canonical_in_flight_sessions_from_state() {
        let repo_root = TempRepoDir::new("worklog-canonical-in-flight");
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "cycle": 154
                },
                "in_flight_sessions": 3,
                "tool_pipeline": {
                    "status": "PASS (6/6)"
                },
                "publish_gate": {
                    "status": "open"
                },
                "agent_sessions": [
                    {
                        "issue": 101,
                        "title": "First active dispatch",
                        "status": "in_flight"
                    },
                    {
                        "issue": 102,
                        "title": "Second active dispatch",
                        "status": "in_flight"
                    },
                    {
                        "issue": 103,
                        "title": "Third active dispatch",
                        "status": "in_flight"
                    }
                ]
            }"#,
        );

        let mut args = worklog_args("Canonical in flight");
        args.done = vec!["Closed review loop".to_string()];

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("- **In-flight agent sessions**: 3"));
    }

    #[test]
    fn worklog_inline_flags_use_frozen_c5_5_state_when_present() {
        let repo_root = TempRepoDir::new("worklog-c5-5-frozen-state");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "in_flight_sessions": 0,
                "publish_gate": {
                    "status": "published"
                },
                "tool_pipeline": {
                    "status": "phase_5_active",
                    "c5_5_gate": {
                        "cycle": 154,
                        "status": "PASS",
                        "needs_reverify": false,
                        "pipeline_summary": "PASS (6/6)"
                    }
                }
            }"#,
        );

        let mut args = worklog_args("Frozen state");
        args.cycle = Some(154);

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("## Cycle state"));
        assert!(!content.contains(LEGACY_STATE_DISCLAIMER));
        assert!(content.contains("- **Pipeline status**: PASS (6/6)"));
        assert!(!content.contains("post-dispatch"));
    }

    #[test]
    fn worklog_auto_derives_self_modifications_receipts_and_issues_processed() {
        let repo_root = TempRepoDir::new("worklog-auto-derives");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "cycle": 154,
                    "phase_entered_at": "2026-03-06T01:00:00Z"
                },
                "agent_sessions": []
            }"#,
        );
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154, issue #1 [cycle 154]",
        );
        let merge_receipt = create_git_commit_with_message(
            &repo_root.path,
            "tools/rust/crates/write-entry/src/main.rs",
            "changed\n",
            "state(process-merge): update worklog [cycle 154]",
        );
        create_git_commit_with_message(
            &repo_root.path,
            "AGENTS.md",
            "agent guidance\n",
            "docs: update agents [cycle 154]",
        );
        create_git_commit_with_message(
            &repo_root.path,
            "README.md",
            "readme\n",
            "docs: update readme [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154, issue #1 [cycle 154]"}},
                    {{"step":"process-merge","receipt":"{merge_receipt}","commit":"state(process-merge): update worklog [cycle 154]"}}
                ]"#
            ),
        );

        let mut args = worklog_args("Auto derive sections");
        args.done = vec![
            "Closed EvaLok/schema-org-json-ld#1042".to_string(),
            "Merged PR #200".to_string(),
        ];
        args.auto_self_modifications = true;
        args.auto_receipts = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("### Issues processed\n\n- None."));
        assert!(content.contains("- **`tools/rust/crates/write-entry/src/main.rs`**: modified"));
        assert!(!content.contains("- **`AGENTS.md`**: modified"));
        assert!(!content.contains("README.md"));
        assert!(content.contains("## Commit receipts"));
        assert!(content.contains(&format!(
            "| cycle-start | {} | [{}](https://github.com/EvaLok/schema-org-json-ld/commit/{}) |",
            start_receipt, start_receipt, start_receipt
        )));
        assert!(content.contains(&format!(
            "| process-merge | {} | [{}](https://github.com/EvaLok/schema-org-json-ld/commit/{}) |",
            merge_receipt, merge_receipt, merge_receipt
        )));
    }

    #[test]
    fn worklog_auto_derives_pr_sections_from_cycle_bounded_agent_sessions() {
        let repo_root = TempRepoDir::new("worklog-auto-derives-prs");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "cycle": 154,
                    "phase_entered_at": "2026-03-06T01:00:00Z"
                },
                "agent_sessions": [
                    {
                        "issue": 1041,
                        "pr": 237,
                        "merged_at": "2026-03-06T00:59:59Z",
                        "status": "merged"
                    },
                    {
                        "issue": 1042,
                        "pr": 240,
                        "merged_at": "2026-03-06T01:00:01Z",
                        "status": "merged"
                    }
                ]
            }"#,
        );
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154, issue #1 [cycle 154]",
        );
        let merge_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/merge.txt",
            "merged\n",
            "state(process-merge): PR #237, PR #240 merged [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154, issue #1 [cycle 154]"}},
                    {{"step":"process-merge","receipt":"{merge_receipt}","commit":"state(process-merge): PR #237, PR #240 merged [cycle 154]"}}
                ]"#
            ),
        );

        let mut args = worklog_args("Auto derive PR sections");
        args.cycle = None;
        args.done = vec!["Closed EvaLok/schema-org-json-ld#1042".to_string()];
        args.auto_receipts = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("### PRs merged"));
        assert!(
            content.contains("[PR #240](https://github.com/EvaLok/schema-org-json-ld/issues/240)")
        );
        assert!(
            !content.contains("[PR #237](https://github.com/EvaLok/schema-org-json-ld/issues/237)")
        );
        assert!(!content.contains("### PRs reviewed"));
        assert!(!content.contains("### PRs merged\n\n- None."));
    }

    #[test]
    fn worklog_prs_merged_flag_does_not_auto_render_reviewed_prs() {
        let repo_root = TempRepoDir::new("worklog-prs-merged-without-reviewed");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154, issue #1 [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154, issue #1 [cycle 154]"}}
                ]"#
            ),
        );

        let mut args = worklog_args("Merged only");
        args.pr_merged = vec![1226];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("### PRs merged"));
        assert!(content
            .contains("[PR #1226](https://github.com/EvaLok/schema-org-json-ld/issues/1226)"));
        assert!(!content.contains("### PRs reviewed"));
    }

    #[test]
    fn worklog_prs_reviewed_flag_renders_reviewed_prs() {
        let repo_root = TempRepoDir::new("worklog-prs-reviewed-explicit");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154, issue #1 [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154, issue #1 [cycle 154]"}}
                ]"#
            ),
        );

        let mut args = worklog_args("Reviewed only");
        args.pr_reviewed = vec![1226];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("### PRs reviewed"));
        assert!(content
            .contains("[PR #1226](https://github.com/EvaLok/schema-org-json-ld/issues/1226)"));
    }

    #[test]
    fn worklog_prs_merged_and_reviewed_flags_render_both_sections() {
        let repo_root = TempRepoDir::new("worklog-prs-merged-and-reviewed");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154, issue #1 [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154, issue #1 [cycle 154]"}}
                ]"#
            ),
        );

        let mut args = worklog_args("Merged and reviewed");
        args.pr_merged = vec![1226];
        args.pr_reviewed = vec![1226];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("### PRs merged"));
        assert!(content.contains("### PRs reviewed"));
        assert!(content
            .contains("[PR #1226](https://github.com/EvaLok/schema-org-json-ld/issues/1226)"));
    }

    #[test]
    fn worklog_auto_derives_sections_when_cycle_is_resolved_from_state() {
        let repo_root = TempRepoDir::new("worklog-auto-derives-resolved-cycle");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "cycle": 154,
                    "phase_entered_at": "2026-03-06T01:00:00Z"
                },
                "agent_sessions": []
            }"#,
        );
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154, issue #1 [cycle 154]",
        );
        let merge_receipt = create_git_commit_with_message(
            &repo_root.path,
            "tools/rust/crates/write-entry/src/main.rs",
            "changed\n",
            "state(process-merge): update worklog [cycle 154]",
        );
        create_git_commit_with_message(
            &repo_root.path,
            "AGENTS.md",
            "agent guidance\n",
            "docs: update agents [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154, issue #1 [cycle 154]"}},
                    {{"step":"process-merge","receipt":"{merge_receipt}","commit":"state(process-merge): update worklog [cycle 154]"}}
                ]"#
            ),
        );

        let mut args = worklog_args("Resolved cycle auto derive");
        args.cycle = None;
        args.done = vec!["Closed EvaLok/schema-org-json-ld#1042".to_string()];
        args.auto_self_modifications = true;
        args.auto_receipts = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let cycle = resolve_cycle(args.cycle, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, cycle, &mut input).unwrap();

        assert!(warnings.is_empty());
        assert!(input.issues_processed.is_empty());
        assert_eq!(input.self_modifications.len(), 1);
        assert!(input.self_modifications.iter().any(|item| item.file
            == "tools/rust/crates/write-entry/src/main.rs"
            && item.description == "modified"));
        assert!(!input
            .self_modifications
            .iter()
            .any(|item| item.file == "AGENTS.md" && item.description == "modified"));
        assert_eq!(input.receipts.len(), 2);
        assert_eq!(input.receipts[0].tool, "cycle-start");
        assert_eq!(input.receipts[0].receipt, start_receipt);
        assert_eq!(input.receipts[1].tool, "process-merge");
        assert_eq!(input.receipts[1].receipt, merge_receipt);
    }

    #[test]
    fn worklog_auto_derives_receipt_scope_note_from_state_and_events() {
        let repo_root = TempRepoDir::new("worklog-auto-derives-scope-note");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "project_mode": {"mode": "stabilization"},
                "cycle_phase": {
                    "phase": "close_out",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [
                    {
                        "issue": 42,
                        "status": "queued",
                        "dispatched_at": "2026-03-06T01:05:00Z",
                        "title": "Dispatched this cycle"
                    },
                    {
                        "issue": 43,
                        "status": "merged",
                        "merged_at": "2026-03-06T02:00:00Z",
                        "title": "Merged this cycle"
                    },
                    {
                        "issue": 44,
                        "status": "reviewed_awaiting_eva",
                        "status_changed_at": "2026-03-06T03:00:00Z",
                        "title": "Reviewed this cycle"
                    }
                ]
            }"#,
        );
        let cycle_start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
        );
        let dispatch_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/dispatch.txt",
            "dispatch\n",
            "state(record-dispatch): #42 dispatched [cycle 154]",
        );
        let merge_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/merge.txt",
            "merge\n",
            "state(process-merge): PR #88 merged [cycle 154]",
        );
        let review_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/review.txt",
            "review\n",
            "state(verify-review-events): review events verified [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{cycle_start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}},
                    {{"step":"record-dispatch","receipt":"{dispatch_receipt}","commit":"state(record-dispatch): #42 dispatched [cycle 154]"}},
                    {{"step":"process-merge","receipt":"{merge_receipt}","commit":"state(process-merge): PR #88 merged [cycle 154]"}},
                    {{"step":"verify-review-events","receipt":"{review_receipt}","commit":"state(verify-review-events): review events verified [cycle 154]"}}
                ]"#
            ),
        );

        let mut args = worklog_args("Derived scope note");
        args.cycle = None;
        args.done = vec!["Closed #42".to_string()];
        args.auto_receipts = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let cycle = resolve_cycle(args.cycle, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, cycle, &mut input).unwrap();

        assert!(warnings.is_empty());
        let note = input.receipt_note.expect("scope note should be derived");
        assert!(note.contains("Scope: cycle 154 commits (unbounded)"));
        assert!(note.contains("mode stabilization"));
        assert!(note.contains("phase close_out"));
        assert!(note.contains("agent activity: 1 dispatch, 1 merge, 1 status update"));
        assert!(note.contains("receipt events: 1 dispatch, 1 merge"));
        assert!(note.contains("Receipt table auto-generated by `cycle-receipts --cycle 154`."));
        assert!(
            !note.contains("Receipt table covers commits through cycle-complete (C5.1 snapshot).")
        );
        assert!(!note.contains("Post-C5.1 commits"));
        assert!(!note.contains("Validated by receipt-validate at step C5.1."));
    }

    #[test]
    fn fallback_receipt_scope_note_uses_factual_auto_generated_message() {
        let entries = vec![CycleReceiptJsonEntry {
            tool: "record-dispatch".to_string(),
            receipt: "abcdef1".to_string(),
            url: Some(
                "https://github.com/EvaLok/schema-org-json-ld/commit/abcdef1234567890".to_string(),
            ),
            _aliases: Vec::new(),
        }];

        let note = fallback_receipt_scope_note(154, &entries, None);

        assert!(note.contains("Scope: cycle 154 commits (unbounded)"));
        assert!(note.contains("receipt events: 1 dispatch"));
        assert!(note.contains("Receipt table auto-generated by `cycle-receipts --cycle 154`."));
        assert!(
            !note.contains("Receipt table covers commits through cycle-complete (C5.1 snapshot).")
        );
        assert!(!note.contains("Post-C5.1 commits"));
    }

    #[test]
    fn worklog_auto_receipts_fail_closed_when_cycle_receipts_command_fails() {
        let repo_root = TempRepoDir::new("worklog-auto-receipts-fail-closed");
        init_git_repo(&repo_root.path);
        let mut args = worklog_args("Fallback");
        args.done = vec!["Closed #42".to_string()];
        args.auto_receipts = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let error = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap_err();

        assert!(error.contains("cycle-receipts command failed"));
    }

    #[test]
    fn worklog_cycle_receipts_must_not_be_empty() {
        let repo_root = TempRepoDir::new("worklog-empty-cycle-receipts");
        init_git_repo(&repo_root.path);
        write_cycle_receipts_script(&repo_root.path, "[]");
        let mut args = worklog_args("Empty receipts");
        args.done = vec!["Closed #42".to_string()];
        args.auto_receipts = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let error = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap_err();

        assert!(error.contains("cycle-receipts returned no receipts for cycle 154"));
    }

    #[test]
    fn worklog_auto_self_modifications_renders_none_when_diff_is_empty() {
        let repo_root = TempRepoDir::new("worklog-auto-self-modifications-empty");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154, issue #1 [cycle 154]",
        );
        let end_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/end.txt",
            "end\n",
            "state(process-merge): canonical receipt [cycle 154]",
        );
        create_git_commit_with_message(
            &repo_root.path,
            "AGENTS.md",
            "late infra change\n",
            "docs: late infrastructure edit [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154, issue #1 [cycle 154]"}},
                    {{"step":"process-merge","receipt":"{end_receipt}","commit":"state(process-merge): canonical receipt [cycle 154]"}}
                ]"#
            ),
        );

        let mut args = worklog_args("Auto self-modifications empty");
        args.done = vec!["Closed EvaLok/schema-org-json-ld#1042".to_string()];
        args.auto_self_modifications = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();

        assert!(warnings.is_empty());
        assert!(input.self_modifications.is_empty());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("## Self-modifications\n\n- None."));
    }

    #[test]
    fn worklog_auto_derives_issues_processed_from_state_agent_sessions() {
        let repo_root = TempRepoDir::new("worklog-state-issues");
        init_git_repo(&repo_root.path);
        create_git_commit_at(
            &repo_root.path,
            "notes/prior.txt",
            "prior\n",
            "docs: prior cycle [cycle 153]",
            "2026-03-05T23:59:00Z",
        );
        let start_receipt = create_git_commit_at(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
            "2026-03-06T01:00:00Z",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}}
                ]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [
                    {
                        "issue": 42,
                        "pr": 100,
                        "status": "merged",
                        "merged_at": "2026-03-06T02:00:00Z",
                        "title": "Merged this cycle"
                    },
                    {
                        "issue": 41,
                        "pr": 99,
                        "status": "merged",
                        "merged_at": "2026-03-05T22:00:00Z",
                        "title": "Merged prior cycle"
                    }
                ]
            }"#,
        );

        let cli = Cli::try_parse_from([
            "write-entry",
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "worklog",
            "--title",
            "State-derived issues",
            "--auto-issues",
            "--pipeline",
            "PASS (6/6)",
            "--publish-gate",
            "open",
        ])
        .unwrap();
        let args = match cli.command {
            Command::Worklog(args) => args,
            Command::Journal(_) => panic!("expected worklog command"),
        };

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("### Issues processed\n\n- [#42]("));
        assert!(content.contains(": Merged this cycle"));
        assert!(!content.contains("[#41]("));
    }

    #[test]
    fn worklog_auto_issues_derives_multiple_sessions_from_state() {
        let repo_root = TempRepoDir::new("worklog-auto-issues-multiple");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_at(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
            "2026-03-06T01:00:00Z",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}}
                ]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [
                    {
                        "issue": 42,
                        "pr": 100,
                        "status": "queued",
                        "dispatched_at": "2026-03-06T01:05:00Z",
                        "title": "Dispatched this cycle"
                    },
                    {
                        "issue": 43,
                        "pr": 101,
                        "status": "merged",
                        "merged_at": "2026-03-06T02:00:00Z",
                        "title": "Merged this cycle"
                    },
                    {
                        "issue": 44,
                        "pr": 102,
                        "status": "in_progress",
                        "status_changed_at": "2026-03-06T03:00:00Z",
                        "title": "Status changed this cycle"
                    },
                    {
                        "issue": 45,
                        "pr": 103,
                        "status": "queued",
                        "dispatched_at": "2026-03-05T23:00:00Z",
                        "title": "Dispatched prior cycle"
                    }
                ]
            }"#,
        );

        let cli = Cli::try_parse_from([
            "write-entry",
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "worklog",
            "--title",
            "Auto issues multiple",
            "--auto-issues",
            "--pipeline",
            "PASS (6/6)",
            "--publish-gate",
            "open",
        ])
        .unwrap();
        let args = match cli.command {
            Command::Worklog(args) => args,
            Command::Journal(_) => panic!("expected worklog command"),
        };

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("- [#42](https://github.com/EvaLok/schema-org-json-ld/issues/42): Dispatched this cycle"));
        assert!(content.contains(
            "- [#43](https://github.com/EvaLok/schema-org-json-ld/issues/43): Merged this cycle"
        ));
        assert!(content.contains("- [#44](https://github.com/EvaLok/schema-org-json-ld/issues/44): Status changed this cycle"));
        assert!(!content.contains("[#45]("));
        assert!(!content.contains("[PR #100]("));
        assert!(!content.contains("[PR #101]("));
        assert!(!content.contains("[PR #102]("));
    }

    #[test]
    fn worklog_auto_issues_renders_none_when_no_sessions_match() {
        let repo_root = TempRepoDir::new("worklog-auto-issues-none");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_at(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
            "2026-03-06T01:00:00Z",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}}
                ]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [
                    {
                        "issue": 45,
                        "status": "queued",
                        "dispatched_at": "2026-03-05T23:00:00Z",
                        "title": "Prior cycle"
                    }
                ]
            }"#,
        );

        let cli = Cli::try_parse_from([
            "write-entry",
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "worklog",
            "--title",
            "Auto issues none",
            "--auto-issues",
            "--pipeline",
            "PASS (6/6)",
            "--publish-gate",
            "open",
        ])
        .unwrap();
        let args = match cli.command {
            Command::Worklog(args) => args,
            Command::Journal(_) => panic!("expected worklog command"),
        };

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("### Issues processed\n\n- None."));
    }

    #[test]
    fn worklog_auto_issues_appends_manual_entries_after_auto_entries() {
        let repo_root = TempRepoDir::new("worklog-auto-issues-manual");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_at(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
            "2026-03-06T01:00:00Z",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}}
                ]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [
                    {
                        "issue": 42,
                        "status": "merged",
                        "merged_at": "2026-03-06T02:00:00Z",
                        "title": "Merged this cycle"
                    }
                ]
            }"#,
        );

        let cli = Cli::try_parse_from([
            "write-entry",
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "worklog",
            "--title",
            "Auto issues manual",
            "--auto-issues",
            "--issue-processed",
            "Processed review #77",
            "--pipeline",
            "PASS (6/6)",
            "--publish-gate",
            "open",
        ])
        .unwrap();
        let args = match cli.command {
            Command::Worklog(args) => args,
            Command::Journal(_) => panic!("expected worklog command"),
        };

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();

        assert!(warnings.is_empty());
        assert_eq!(
            input.issues_processed,
            vec!["#42: Merged this cycle", "Processed review #77"]
        );

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        let auto_index = content
            .find("- [#42](https://github.com/EvaLok/schema-org-json-ld/issues/42): Merged this cycle")
            .unwrap();
        let manual_index = content
            .find(
                "- Processed review [#77](https://github.com/EvaLok/schema-org-json-ld/issues/77)",
            )
            .unwrap();
        assert!(auto_index < manual_index);
    }

    #[test]
    fn worklog_auto_issues_merges_multiple_state_sources_with_manual_entries() {
        let repo_root = TempRepoDir::new("worklog-auto-issues-state-sources");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_at(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
            "2026-03-06T01:00:00Z",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}}
                ]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [
                    {
                        "issue": 42,
                        "status": "merged",
                        "merged_at": "2026-03-06T02:00:00Z",
                        "qc_report": 160,
                        "title": "Merged this cycle"
                    },
                    {
                        "issue": 43,
                        "status": "merged",
                        "merged_at": "2026-03-06T03:00:00Z",
                        "audit_inbound": 315,
                        "title": "Audit linked this cycle"
                    }
                ],
                "eva_input_issues": {
                    "closed_this_cycle": [99]
                },
                "qc_processed": [160, 171],
                "audit_processed": [315, 400]
            }"#,
        );

        let cli = Cli::try_parse_from([
            "write-entry",
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "worklog",
            "--title",
            "Auto issues state sources",
            "--auto-issues",
            "--issue-processed",
            "Processed review #77",
            "--pipeline",
            "PASS (6/6)",
            "--publish-gate",
            "open",
        ])
        .unwrap();
        let args = match cli.command {
            Command::Worklog(args) => args,
            Command::Journal(_) => panic!("expected worklog command"),
        };

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();

        assert!(warnings.is_empty());
        assert_eq!(
            input.issues_processed,
            vec![
                "#42: Merged this cycle",
                "QC #160: Merged this cycle",
                "#43: Audit linked this cycle",
                "audit #315: Audit linked this cycle",
                "#99: Eva input closed this cycle",
                "Processed review #77",
            ]
        );

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("- [QC #160](https://github.com/EvaLok/schema-org-json-ld-qc/issues/160): Merged this cycle"));
        assert!(content.contains("- [audit #315](https://github.com/EvaLok/schema-org-json-ld-audit/issues/315): Audit linked this cycle"));
        assert!(content.contains("- [#99](https://github.com/EvaLok/schema-org-json-ld/issues/99): Eva input closed this cycle"));
        assert!(!content.contains("[QC #171]"));
        assert!(!content.contains("[audit #400]"));
        let auto_index = content
            .find("- [audit #315](https://github.com/EvaLok/schema-org-json-ld-audit/issues/315): Audit linked this cycle")
            .unwrap();
        let manual_index = content
            .find(
                "- Processed review [#77](https://github.com/EvaLok/schema-org-json-ld/issues/77)",
            )
            .unwrap();
        assert!(auto_index < manual_index);
        assert_eq!(content.matches("[QC #160]").count(), 1);
        assert_eq!(content.matches("[audit #315]").count(), 1);
    }

    #[test]
    fn worklog_auto_issues_prefixes_audit_tracking_references_from_session_notes() {
        let repo_root = TempRepoDir::new("worklog-auto-issues-audit-tracking-notes");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_at(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
            "2026-03-06T01:00:00Z",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}}
                ]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [
                    {
                        "issue": 42,
                        "status": "merged",
                        "merged_at": "2026-03-06T02:00:00Z",
                        "note": "Validated fix against #357 before merge",
                        "title": "Audit follow-up merged"
                    }
                ],
                "audit_tracking": {
                    "recommendations": [
                        {
                            "issue": 42,
                            "audit_issue": 357,
                            "description": "Verify the audit evidence link"
                        }
                    ]
                }
            }"#,
        );

        let mut args = worklog_args("Auto issues audit tracking notes");
        args.auto_issues = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();

        assert!(warnings.is_empty());
        assert_eq!(
            input.issues_processed,
            vec!["#42: Audit follow-up merged", "audit #357"]
        );

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains(
            "- [#42](https://github.com/EvaLok/schema-org-json-ld/issues/42): Audit follow-up merged"
        ));
        assert!(content.contains(
            "- [audit #357](https://github.com/EvaLok/schema-org-json-ld-audit/issues/357)"
        ));
    }

    #[test]
    fn worklog_auto_issues_keeps_regular_main_repo_references_bare() {
        let repo_root = TempRepoDir::new("worklog-auto-issues-main-repo-bare");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_at(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
            "2026-03-06T01:00:00Z",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}}
                ]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [],
                "audit_tracking": {
                    "recommendations": [
                        {
                            "issue": 42,
                            "audit_issue": 357,
                            "description": "Verify the audit evidence link"
                        }
                    ]
                }
            }"#,
        );

        let mut args = worklog_args("Auto issues main repo references");
        args.auto_issues = true;
        args.done = vec!["Closed follow-up issue #42 after validation".to_string()];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();

        assert!(warnings.is_empty());
        assert_eq!(input.issues_processed, vec!["#42"]);

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("- [#42](https://github.com/EvaLok/schema-org-json-ld/issues/42)"));
        assert!(!content.contains("[audit #42]"));
    }

    #[test]
    fn worklog_auto_issues_derives_issue_references_from_what_was_done() {
        let repo_root = TempRepoDir::new("worklog-auto-issues-what-was-done");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_at(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
            "2026-03-06T01:00:00Z",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}}
                ]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": []
            }"#,
        );

        let mut args = worklog_args("Auto issues what was done");
        args.auto_issues = true;
        args.done = vec![
            "Processed cycle 153 review and closed EvaLok/schema-org-json-ld#1803".to_string(),
            "Merged PR #200".to_string(),
        ];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();

        assert!(warnings.is_empty());
        assert_eq!(input.issues_processed, vec!["#1803"]);

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains(
            "### Issues processed\n\n- [#1803](https://github.com/EvaLok/schema-org-json-ld/issues/1803)"
        ));
        assert!(!content.contains("[#200]("));
    }

    #[test]
    fn worklog_auto_issues_derives_current_cycle_review_history_issues() {
        let repo_root = TempRepoDir::new("worklog-auto-issues-review-history");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_at(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
            "2026-03-06T01:00:00Z",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}}
                ]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [],
                "review_agent": {
                    "history": [
                        {
                            "cycle": 154,
                            "note": "Processed review #77 and prepared follow-up dispatch",
                            "finding_dispositions": [
                                {
                                    "category": "worklog-accuracy",
                                    "disposition": "dispatch_created",
                                    "dispatch_issue": 88
                                }
                            ]
                        },
                        {
                            "cycle": 153,
                            "note": "Prior cycle review #66",
                            "finding_dispositions": []
                        }
                    ]
                }
            }"#,
        );

        let mut args = worklog_args("Auto issues review history");
        args.auto_issues = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();

        assert!(warnings.is_empty());
        assert_eq!(input.issues_processed, vec!["#77", "#88"]);

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("- [#77](https://github.com/EvaLok/schema-org-json-ld/issues/77)"));
        assert!(content.contains("- [#88](https://github.com/EvaLok/schema-org-json-ld/issues/88)"));
        assert!(!content.contains("[#66]("));
    }

    #[test]
    fn worklog_auto_review_summary_reports_mixed_dispositions() {
        let repo_root = TempRepoDir::new("worklog-auto-review-summary-mixed");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [
                    {
                        "issue": 77,
                        "title": "[Cycle Review] Cycle 153 end-of-cycle review",
                        "status": "merged"
                    }
                ],
                "review_agent": {
                    "history": [
                        {
                            "cycle": 153,
                            "review_issue": 77,
                            "finding_count": 3,
                            "complacency_score": 2,
                            "finding_dispositions": [
                                {"category": "worklog-accuracy", "disposition": "deferred"},
                                {"category": "receipt-coverage", "disposition": "dispatch_created"},
                                {"category": "follow-up", "disposition": "deferred"}
                            ]
                        }
                    ]
                }
            }"#,
        );

        let mut args = worklog_args("Auto review summary mixed");
        args.auto_review_summary = true;
        args.done = vec!["Manual done item".to_string()];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();

        assert!(warnings.is_empty());
        assert_eq!(
            input.what_was_done,
            vec![
                "Processed cycle 153 review (3 findings, complacency 2/5, 2 deferred, 1 dispatch_created)"
                    .to_string(),
                "Manual done item".to_string()
            ]
        );
    }

    #[test]
    fn worklog_auto_review_summary_reports_all_same_dispositions() {
        let repo_root = TempRepoDir::new("worklog-auto-review-summary-same");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [
                    {
                        "issue": 78,
                        "title": "[Cycle Review] Cycle 153 end-of-cycle review",
                        "status": "merged"
                    }
                ],
                "review_agent": {
                    "history": [
                        {
                            "cycle": 152,
                            "review_issue": 78,
                            "finding_count": 2,
                            "complacency_score": 4,
                            "finding_dispositions": [
                                {"category": "worklog-accuracy", "disposition": "deferred"},
                                {"category": "receipt-coverage", "disposition": "deferred"}
                            ]
                        }
                    ]
                }
            }"#,
        );

        let mut args = worklog_args("Auto review summary same");
        args.auto_review_summary = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();

        assert!(warnings.is_empty());
        assert_eq!(
            input.what_was_done,
            vec!["Processed cycle 152 review (2 findings, complacency 4/5, all deferred)"]
        );
    }

    #[test]
    fn worklog_auto_review_summary_fails_when_review_issue_cannot_be_resolved() {
        let repo_root = TempRepoDir::new("worklog-auto-review-summary-missing-review-issue");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [],
                "review_agent": {
                    "history": []
                }
            }"#,
        );

        let mut args = worklog_args("Auto review summary missing issue");
        args.auto_review_summary = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let error = apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input)
            .expect_err("auto review summary should fail without a resolvable review issue");
        assert!(
            error.contains("pass --review-issue <number>"),
            "unexpected error: {error}"
        );
    }

    #[test]
    fn worklog_explicit_issue_numbers_flag_renders_linked_issue_entries() {
        let repo_root = TempRepoDir::new("worklog-explicit-issue-numbers");
        init_git_repo(&repo_root.path);

        let cli = Cli::try_parse_from([
            "write-entry",
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "worklog",
            "--cycle",
            "154",
            "--title",
            "Explicit issue numbers",
            "--issues-processed",
            "42, 77",
            "--pipeline",
            "PASS (6/6)",
            "--publish-gate",
            "open",
        ])
        .unwrap();
        let args = match cli.command {
            Command::Worklog(args) => args,
            Command::Journal(_) => panic!("expected worklog command"),
        };

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        assert_eq!(input.issues_processed, vec!["#42", "#77"]);

        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();
        assert!(warnings.is_empty());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(
            content.contains(
                "### Issues processed\n\n- [#42](https://github.com/EvaLok/schema-org-json-ld/issues/42)"
            )
        );
        assert!(content.contains("- [#77](https://github.com/EvaLok/schema-org-json-ld/issues/77)"));
        assert!(!content.contains("### Issues processed\n\n- None."));
    }

    #[test]
    fn worklog_cycle_only_arguments_render_auto_populated_sections() {
        let repo_root = TempRepoDir::new("worklog-cycle-only");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_at(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
            "2026-03-06T01:00:00Z",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}}
                ]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "in_flight_sessions": 3,
                "copilot_metrics": {
                    "total_dispatches": 45,
                    "produced_pr": 42,
                    "merged": 40,
                    "pr_merge_rate": "88.9%",
                    "in_flight": 3
                },
                "tool_pipeline": {
                    "status": "PASS (6/6)"
                },
                "publish_gate": {
                    "status": "published"
                },
                "cycle_phase": {
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "cycle": 154
                },
                "agent_sessions": [
                    {
                        "issue": 42,
                        "pr": 100,
                        "status": "merged",
                        "merged_at": "2026-03-06T02:00:00Z",
                        "title": "Merged this cycle"
                    }
                ]
            }"#,
        );

        let cli = Cli::try_parse_from([
            "write-entry",
            "--repo-root",
            repo_root.path.to_str().unwrap(),
            "worklog",
            "--title",
            "Cycle only",
            "--auto-issues",
            "--auto-receipts",
        ])
        .unwrap();
        let args = match cli.command {
            Command::Worklog(args) => args,
            Command::Journal(_) => panic!("expected worklog command"),
        };
        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("### Issues processed\n\n- [#42]("));
        assert!(content.contains("| cycle-start |"));
        assert!(content.contains("- **In-flight agent sessions**: 3"));
        assert!(content.contains("- **Pipeline status**: PASS (6/6)"));
        assert!(content.contains("- **Publish gate**: published"));
    }

    #[test]
    fn find_cycle_start_commit_prefers_cycle_phase_timestamp() {
        let repo_root = TempRepoDir::new("worklog-cycle-start-fallback");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {
                    "number": 154,
                    "timestamp": "2026-03-06T01:00:00Z"
                },
                "cycle_phase": {
                    "cycle": 154,
                    "phase": "work",
                    "phase_entered_at": "2026-03-06T01:10:00Z"
                }
            }"#,
        );
        create_git_commit_at(
            &repo_root.path,
            "notes/first.txt",
            "first\n",
            "docs: first commit [cycle 154]",
            "2026-03-06T01:05:00Z",
        );
        let second_commit = create_git_commit_at(
            &repo_root.path,
            "notes/second.txt",
            "second\n",
            "docs: second commit [cycle 154]",
            "2026-03-06T01:15:00Z",
        );

        let start_commit = find_cycle_start_commit(&repo_root.path, 154).unwrap();
        assert!(start_commit.starts_with(&second_commit));
    }

    #[test]
    fn render_journal_entry_strips_cycle_prefix_from_title_and_context() {
        let rendered = render_journal_entry(
            226,
            fixed_now_on("2026-03-11"),
            "Cycle 226: Breaking the worklog-accuracy pattern",
            &JournalInput {
                previous_commitment_status: "no_prior_commitment".to_string(),
                previous_commitment_detail: "No prior commitment recorded.".to_string(),
                sections: Vec::new(),
                concrete_behavior_change: String::new(),
                commitments: Vec::new(),
                open_questions: Vec::new(),
            },
            CommitmentStatus::NoPriorCommitment,
            None,
            Some("../worklog/2026-03-11/123451-cycle-226-summary.md"),
        );

        assert!(
            rendered.contains("## 2026-03-11 — Cycle 226: Breaking the worklog-accuracy pattern")
        );
        assert!(!rendered.contains("Cycle 226: Cycle 226:"));
        assert!(rendered.contains("Cycle 226 focused on Breaking the worklog-accuracy pattern."));
        assert!(!rendered.contains("focused on Cycle 226:"));
    }

    #[test]
    fn worklog_auto_receipts_reject_manual_receipts_during_execution() {
        let repo_root = TempRepoDir::new("worklog-auto-receipts-conflict");
        init_git_repo(&repo_root.path);
        let mut args = worklog_args("Conflict warning");
        args.done = vec!["Closed #42".to_string()];
        args.auto_receipts = true;
        args.receipt = vec!["cycle-start:abc1234".to_string()];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let error = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap_err();

        assert!(error.contains("--auto-receipts"));
        assert!(error.contains("--receipt"));
    }

    #[test]
    fn worklog_inline_flags_prefer_explicit_status_over_state() {
        let repo_root = TempRepoDir::new("worklog-status-override");
        init_git_repo(&repo_root.path);
        let receipt = create_git_commit(&repo_root.path, "notes/status-override.txt", "override\n");
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[{{"step":"manual","receipt":"{receipt}","commit":"Add notes/status-override.txt"}}]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "in_flight_sessions": 1,
                "copilot_metrics": {
                    "total_dispatches": 45,
                    "produced_pr": 42,
                    "merged": 40,
                    "pr_merge_rate": "88.9%",
                    "in_flight": 3
                },
                "publish_gate": {
                    "status": "published"
                }
            }"#,
        );

        let mut args = worklog_args("Override");
        args.cycle = None;
        args.done = vec!["Merged PR #123".to_string()];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("pre-publish".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("- **Pipeline status**: PASS (6/6)"));
        assert!(content.contains("- **In-flight agent sessions**: 1"));
        assert!(content.contains("- **Publish gate**: pre-publish"));
        assert!(!content.contains("- **Copilot metrics**:"));
        assert!(!content.contains("- **Publish gate**: published"));
    }

    #[test]
    fn auto_pipeline_status_uses_pass_prefix_for_warning_only_json_report() {
        let repo_root = TempRepoDir::new("worklog-auto-pipeline-pass");
        write_pipeline_check_script(
            &repo_root.path,
            "printf '%s\\n' '{\"overall\":\"pass\",\"has_blocking_findings\":false,\"steps\":[{\"status\":\"warn\"}]}'",
        );

        let status = auto_pipeline_status(&repo_root.path).unwrap();
        assert!(status.starts_with("PASS"));
        assert_eq!(status, "PASS (1 warning)");
    }

    #[test]
    fn auto_pipeline_status_uses_fail_prefix_for_blocking_json_report() {
        let repo_root = TempRepoDir::new("worklog-auto-pipeline-fail");
        write_pipeline_check_script(
            &repo_root.path,
            "printf '%s\\n' '{\"overall\":\"fail\",\"has_blocking_findings\":true,\"steps\":[{\"name\":\"field-inventory\",\"status\":\"fail\"}]}'\nexit 1",
        );

        let status = auto_pipeline_status(&repo_root.path).unwrap();
        assert!(status.starts_with("FAIL"));
        assert_eq!(status, "FAIL (1 blocking: field-inventory)");
    }

    #[test]
    fn format_pipeline_status_keeps_pass_prefix_for_non_blocking_warning_report() {
        let report = PipelineCheckReport {
            overall: "pass".to_string(),
            has_blocking_findings: false,
            steps: vec![
                PipelineCheckStep {
                    status: "warn".to_string(),
                    name: Some("field-inventory".to_string()),
                },
                PipelineCheckStep {
                    status: "warn".to_string(),
                    name: Some("step-comments".to_string()),
                },
            ],
        };

        let status = format_pipeline_status(&report);
        assert!(status.starts_with("PASS"));
        assert_eq!(status, "PASS (2 warnings)");
    }

    #[test]
    fn format_pipeline_status_keeps_fail_prefix_for_blocking_findings_report() {
        let report = PipelineCheckReport {
            overall: "fail".to_string(),
            has_blocking_findings: true,
            steps: vec![PipelineCheckStep {
                status: "fail".to_string(),
                name: Some("field-inventory".to_string()),
            }],
        };

        let status = format_pipeline_status(&report);
        assert!(status.starts_with("FAIL"));
        assert_eq!(status, "FAIL (1 blocking: field-inventory)");
    }

    #[test]
    fn format_pipeline_status_keeps_pass_prefix_for_mixed_warning_severities() {
        let report = PipelineCheckReport {
            overall: "pass".to_string(),
            has_blocking_findings: false,
            steps: vec![
                PipelineCheckStep {
                    status: "pass".to_string(),
                    name: Some("validate-docs".to_string()),
                },
                PipelineCheckStep {
                    status: "warn".to_string(),
                    name: Some("field-inventory".to_string()),
                },
                PipelineCheckStep {
                    status: "warn".to_string(),
                    name: Some("step-comments".to_string()),
                },
            ],
        };

        let status = format_pipeline_status(&report);
        assert!(status.starts_with("PASS"));
        assert_eq!(status, "PASS (2 warnings)");
    }

    #[test]
    fn format_pipeline_status_prefix_matches_report_overall() {
        let reports = [
            PipelineCheckReport {
                overall: "pass".to_string(),
                has_blocking_findings: false,
                steps: vec![],
            },
            PipelineCheckReport {
                overall: "fail".to_string(),
                has_blocking_findings: true,
                steps: vec![PipelineCheckStep {
                    status: "fail".to_string(),
                    name: Some("field-inventory".to_string()),
                }],
            },
            PipelineCheckReport {
                overall: "warn".to_string(),
                has_blocking_findings: false,
                steps: vec![PipelineCheckStep {
                    status: "warn".to_string(),
                    name: Some("step-comments".to_string()),
                }],
            },
        ];

        for report in reports {
            let status = format_pipeline_status(&report);
            assert!(
                status.starts_with(&report.overall.to_ascii_uppercase()),
                "expected {status:?} to start with {:?}",
                report.overall.to_ascii_uppercase()
            );
        }
    }

    #[test]
    fn worklog_inline_flags_fail_closed_when_state_status_is_unavailable() {
        let repo_root = TempRepoDir::new("worklog-status-missing");
        fs::remove_file(repo_root.path.join("docs/state.json")).unwrap();
        let mut args = worklog_args("Missing status");
        args.done = vec!["Merged PR #123".to_string()];

        let error = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap_err();
        assert!(error.contains("failed to read"));
        assert!(error.contains("docs/state.json"));
    }

    #[test]
    fn worklog_auto_pipeline_fails_closed_when_pipeline_check_fails() {
        let repo_root = TempRepoDir::new("worklog-auto-pipeline-failure");
        write_pipeline_check_script(&repo_root.path, "echo 'pipeline blocked' >&2\nexit 1");

        let mut args = worklog_args("Auto pipeline failure");
        args.done = vec!["Merged PR #123".to_string()];
        args.auto_pipeline = true;
        args.publish_gate = Some("open".to_string());

        let error = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap_err();
        assert!(error.contains("pipeline-check did not produce a parseable overall summary"));
        assert!(error.contains("pipeline blocked"));
    }

    #[test]
    fn worklog_auto_pipeline_overrides_state_when_it_is_the_only_inline_flag() {
        let repo_root = TempRepoDir::new("worklog-auto-pipeline-only-inline");
        write_pipeline_check_script(
            &repo_root.path,
            "printf '%s\\n' '{\"overall\":\"pass\",\"has_blocking_findings\":false,\"steps\":[{\"status\":\"warn\"},{\"status\":\"warn\"},{\"status\":\"warn\"}]}'",
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "tool_pipeline": {"status": "FAIL (state value should not be used)"},
                "in_flight_sessions": 3,
                "copilot_metrics": {
                    "total_dispatches": 45,
                    "produced_pr": 42,
                    "merged": 40,
                    "pr_merge_rate": "88.9%",
                    "in_flight": 3
                },
                "publish_gate": {
                    "status": "published"
                }
            }"#,
        );

        let mut args = worklog_args("Auto pipeline only");
        args.cycle = None;
        args.auto_pipeline = true;

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("- **Pipeline status**: PASS (3 warnings)"));
        assert!(!content.contains("FAIL (state value should not be used)"));
        assert!(content.contains("- **In-flight agent sessions**: 3"));
    }

    #[test]
    fn worklog_auto_next_generates_next_steps_from_state() {
        let repo_root = TempRepoDir::new("worklog-auto-next");
        write_state_file(
            &repo_root.path,
            r#"{
                "cycle_phase": {
                    "cycle": 154
                },
                "in_flight_sessions": 1,
                "tool_pipeline": {
                    "status": "PASS (6/6)"
                },
                "publish_gate": {
                    "status": "open"
                },
                "copilot_metrics": {
                    "total_dispatches": 12,
                    "produced_pr": 11,
                    "merged": 10,
                    "pr_merge_rate": "90.9%",
                    "in_flight": 1
                },
                "deferred_findings": [
                    {
                        "category": "process-adherence",
                        "deferred_cycle": 149,
                        "deadline_cycle": 154,
                        "resolved": false
                    },
                    {
                        "category": "journal-quality",
                        "deferred_cycle": 150,
                        "deadline_cycle": 155,
                        "resolved": false
                    }
                ],
                "agent_sessions": [
                    {
                        "issue": 1825,
                        "title": "Add --auto-pipeline flag to write-entry",
                        "status": "in_flight"
                    },
                    {
                        "issue": 1826,
                        "title": "Already merged",
                        "status": "merged"
                    }
                ]
            }"#,
        );

        let mut args = worklog_args("Auto next");
        args.auto_next = true;

        let input = resolve_worklog_input(&args, &repo_root.path).unwrap();

        assert_eq!(
            input.next_steps,
            vec![
                "Address deferred finding: process-adherence (deferred cycle 149, deadline cycle 154) — must be actioned, dispatched, or explicitly dropped this cycle"
                    .to_string(),
                "Address deferred finding: journal-quality (deferred cycle 150, deadline cycle 155) — must be actioned, dispatched, or explicitly dropped this cycle"
                    .to_string(),
                "Review and iterate on PR from #1825 (Add --auto-pipeline flag to write-entry) when Copilot completes"
                    .to_string()
            ]
        );
    }

    #[test]
    fn worklog_auto_next_empty_when_no_sessions() {
        let repo_root = TempRepoDir::new("worklog-auto-next-empty");
        write_state_file(
            &repo_root.path,
            r#"{
                "cycle_phase": {
                    "cycle": 154
                },
                "in_flight_sessions": 0,
                "tool_pipeline": {
                    "status": "PASS (6/6)"
                },
                "publish_gate": {
                    "status": "open"
                },
                "copilot_metrics": {
                    "total_dispatches": 12,
                    "produced_pr": 11,
                    "merged": 10,
                    "pr_merge_rate": "90.9%",
                    "in_flight": 0
                },
                "agent_sessions": []
            }"#,
        );

        let mut args = worklog_args("Auto next empty");
        args.auto_next = true;

        let input = resolve_worklog_input(&args, &repo_root.path).unwrap();

        assert_eq!(
            input.next_steps,
            vec!["No in-flight sessions — plan next dispatch".to_string()]
        );
    }

    #[test]
    fn worklog_auto_next_excludes_dropped_deferred_findings() {
        let repo_root = TempRepoDir::new("worklog-auto-next-dropped-finding");
        write_state_file(
            &repo_root.path,
            r#"{
                "cycle_phase": {
                    "cycle": 154
                },
                "in_flight_sessions": 0,
                "tool_pipeline": {
                    "status": "PASS (6/6)"
                },
                "publish_gate": {
                    "status": "open"
                },
                "copilot_metrics": {
                    "total_dispatches": 12,
                    "produced_pr": 11,
                    "merged": 10,
                    "pr_merge_rate": "90.9%",
                    "in_flight": 0
                },
                "deferred_findings": [
                    {
                        "category": "process-adherence",
                        "deferred_cycle": 149,
                        "deadline_cycle": 154,
                        "resolved": false,
                        "dropped_rationale": "superseded by broader fix"
                    }
                ],
                "agent_sessions": []
            }"#,
        );

        let mut args = worklog_args("Auto next dropped finding");
        args.auto_next = true;

        let input = resolve_worklog_input(&args, &repo_root.path).unwrap();

        assert_eq!(
            input.next_steps,
            vec!["No in-flight sessions — plan next dispatch".to_string()]
        );
    }

    #[test]
    fn worklog_dry_run_renders_markdown_without_writing_file() {
        let repo_root = TempRepoDir::new("worklog-dry-run");
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "cycle": 154
                },
                "in_flight_sessions": 0,
                "tool_pipeline": {
                    "status": "PASS (6/6)"
                },
                "publish_gate": {
                    "status": "open"
                },
                "copilot_metrics": {
                    "total_dispatches": 12,
                    "produced_pr": 11,
                    "merged": 10,
                    "pr_merge_rate": "90.9%",
                    "in_flight": 0
                },
                "deferred_findings": [
                    {
                        "category": "journal-quality",
                        "deferred_cycle": 150,
                        "deadline_cycle": 155,
                        "resolved": false
                    }
                ],
                "agent_sessions": []
            }"#,
        );

        let mut args = worklog_args("Dry run");
        args.dry_run = true;
        args.auto_next = true;

        let content = render_worklog_output(&args, &repo_root.path, fixed_now()).unwrap();

        assert!(content.contains("Address deferred finding: journal-quality"));
        assert!(!repo_root.path.join("docs/worklog").exists());
    }

    #[test]
    fn auto_next_rejects_manual_next_flag() {
        let repo_root = TempRepoDir::new("auto-next-mutually-exclusive");
        let mut args = worklog_args("Auto next conflict");
        args.auto_next = true;
        args.next = vec!["Manual follow-up".to_string()];

        let error = resolve_worklog_input(&args, &repo_root.path).unwrap_err();

        assert_eq!(error, "cannot combine --auto-next with --next");
    }

    #[test]
    fn worklog_inline_flags_allow_missing_copilot_metrics_summary() {
        let repo_root = TempRepoDir::new("worklog-missing-copilot-metrics-allowed");
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "in_flight_sessions": 3,
                "copilot_metrics": {
                    "total_dispatches": 45,
                    "merged": 40,
                    "pr_merge_rate": "88.9%",
                    "in_flight": 3
                },
                "publish_gate": {
                    "status": "published"
                }
            }"#,
        );

        let mut args = worklog_args("Placeholder allowed");
        args.done = vec!["Merged PR #123".to_string()];

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(!content.contains("- **Copilot metrics**:"));
        assert!(content.contains("- **Publish gate**: published"));
    }

    #[test]
    fn invalid_receipt_flag_is_rejected() {
        let repo_root = TempRepoDir::new("invalid-receipt");
        let mut args = worklog_args("Invalid receipt");
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("published".to_string());
        args.receipt = vec!["cycle-start:not-a-sha".to_string()];

        let error = resolve_worklog_input(&args, &repo_root.path).unwrap_err();
        assert!(error.contains("invalid receipt"));
    }

    #[test]
    fn auto_receipts_reject_manual_receipt_flags() {
        let repo_root = TempRepoDir::new("auto-receipts-mutually-exclusive");
        let mut args = worklog_args("Auto receipts conflict");
        args.done = vec!["Closed #42".to_string()];
        args.auto_receipts = true;
        args.receipt = vec!["cycle-start:abc1234".to_string()];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let error = resolve_worklog_input(&args, &repo_root.path).unwrap_err();
        assert!(error.contains("--auto-receipts"));
        assert!(error.contains("--receipt"));
    }

    #[test]
    fn auto_self_modifications_reject_manual_self_modification_flags() {
        let repo_root = TempRepoDir::new("auto-self-modifications-mutually-exclusive");
        let mut args = worklog_args("Auto self-modifications conflict");
        args.done = vec!["Closed #42".to_string()];
        args.auto_self_modifications = true;
        args.self_modification = vec!["AGENTS.md: manual override".to_string()];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let error = resolve_worklog_input(&args, &repo_root.path).unwrap_err();
        assert!(error.contains("--auto-self-modifications"));
        assert!(error.contains("--self-modification"));
    }

    #[test]
    fn auto_pipeline_rejects_manual_pipeline_flag() {
        let repo_root = TempRepoDir::new("auto-pipeline-mutually-exclusive");
        let mut args = worklog_args("Auto pipeline conflict");
        args.done = vec!["Closed #42".to_string()];
        args.auto_pipeline = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let error = resolve_worklog_input(&args, &repo_root.path).unwrap_err();
        assert!(error.contains("--auto-pipeline"));
        assert!(error.contains("--pipeline"));
    }

    #[test]
    fn worklog_auto_receipts_use_cycle_receipts_urls() {
        let repo_root = TempRepoDir::new("worklog-auto-receipts-urls");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "cycle": 154,
                    "phase_entered_at": "2026-03-06T01:00:00Z"
                },
                "agent_sessions": []
            }"#,
        );
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]","url":"https://example.test/commit/{start_receipt}","aliases":["cycle-tagged"]}}
                ]"#
            ),
        );

        let mut args = worklog_args("Auto receipts url");
        args.done = vec!["Closed #42".to_string()];
        args.auto_receipts = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains(&format!(
            "| cycle-start | {} | [{}](https://example.test/commit/{}) |",
            start_receipt, start_receipt, start_receipt
        )));
    }

    #[test]
    fn worklog_auto_receipts_pass_through_timestamp_when_cycle_complete_exists() {
        let repo_root = TempRepoDir::new("worklog-auto-receipts-before");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "complete",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "completed_at": "2026-03-06T04:00:00Z",
                    "cycle": 154
                }
            }"#,
        );
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
        );
        let args_log = repo_root.path.join("cycle-receipts-args.txt");
        write_cycle_receipts_script_with_args_log(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}}
                ]"#
            ),
            &args_log,
        );

        let mut args = worklog_args("Auto receipts boundary");
        args.cycle = Some(154);
        args.done = vec!["Closed #42".to_string()];
        args.auto_receipts = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let argv = fs::read_to_string(args_log).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(argv.contains("--through"));
        assert!(argv.contains("2026-03-06T04:00:00Z"));
        assert!(content
            .contains("Scope: cycle 154 commits through 2026-03-06T04:00:00Z (cycle-complete)"));
        assert!(content.contains(
            "Receipt table auto-generated by `cycle-receipts --cycle 154 --through 2026-03-06T04:00:00Z`."
        ));
    }

    #[test]
    fn worklog_auto_receipt_note_includes_all_non_excluded_categories() {
        let repo_root = TempRepoDir::new("worklog-auto-receipt-note");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
                "cycle_phase": {
                    "phase": "complete",
                    "phase_entered_at": "2026-03-06T01:00:00Z",
                    "completed_at": "2026-03-06T04:55:00Z",
                    "cycle": 154
                },
                "agent_sessions": []
            }"#,
        );
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
        );
        let dispatch_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/dispatch.txt",
            "dispatch\n",
            "state(record-dispatch): #42 dispatched [cycle 154]",
        );
        let merge_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/merge.txt",
            "merge\n",
            "state(process-merge): PR #88 merged [cycle 154]",
        );
        let review_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/review.txt",
            "review\n",
            "state(process-review): review handled [cycle 154]",
        );
        let audit_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/audit.txt",
            "audit\n",
            "state(process-audit): audit handled [cycle 154]",
        );
        let eva_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/eva.txt",
            "eva\n",
            "state(process-eva): eva input handled [cycle 154]",
        );
        let complete_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/complete.txt",
            "complete\n",
            "state(cycle-complete): close cycle 154 [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}},
                    {{"step":"record-dispatch","receipt":"{dispatch_receipt}","commit":"state(record-dispatch): #42 dispatched [cycle 154]"}},
                    {{"step":"process-merge","receipt":"{merge_receipt}","commit":"state(process-merge): PR #88 merged [cycle 154]"}},
                    {{"step":"process-review","receipt":"{review_receipt}","commit":"state(process-review): review handled [cycle 154]"}},
                    {{"step":"process-audit","receipt":"{audit_receipt}","commit":"state(process-audit): audit handled [cycle 154]"}},
                    {{"step":"process-eva","receipt":"{eva_receipt}","commit":"state(process-eva): eva input handled [cycle 154]"}},
                    {{"step":"cycle-complete","receipt":"{complete_receipt}","commit":"state(cycle-complete): close cycle 154 [cycle 154]"}}
                ]"#
            ),
        );

        let mut args = worklog_args("Auto receipt note");
        args.auto_receipts = true;
        args.auto_receipt_note = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();

        assert!(warnings.is_empty());
        let note = input.receipt_note.expect("receipt note should be derived");
        assert!(note.starts_with(
            "1 dispatch, 1 merge, 1 review, 1 audit, 1 Eva input. Scope: cycle 154 commits through 2026-03-06T04:55:00Z (cycle-complete)"
        ));
        assert!(!note.contains("cycle-start): begin"));
        assert!(!note.contains("cycle-complete): close"));
    }

    #[test]
    fn worklog_auto_self_modifications_ignores_changes_after_last_receipt() {
        let repo_root = TempRepoDir::new("worklog-auto-self-modifications-range");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
        );
        create_git_commit_with_message(
            &repo_root.path,
            "AGENTS.md",
            "agents\n",
            "docs: update agents [cycle 154]",
        );
        let last_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/merge.txt",
            "merge\n",
            "state(process-merge): merged PR #88 [cycle 154]",
        );
        create_git_commit_with_message(
            &repo_root.path,
            "tools/rust/crates/write-entry/src/main.rs",
            "late change\n",
            "docs: post-receipt infrastructure change [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]","url":"https://github.com/EvaLok/schema-org-json-ld/commit/{start_receipt}","aliases":[]}},
                    {{"step":"process-merge","receipt":"{last_receipt}","commit":"state(process-merge): merged PR #88 [cycle 154]","url":"https://github.com/EvaLok/schema-org-json-ld/commit/{last_receipt}","aliases":[]}}
                ]"#
            ),
        );

        let mut args = worklog_args("Auto self-modification range");
        args.done = vec!["Closed #42".to_string()];
        args.auto_self_modifications = true;
        args.pipeline = Some("PASS (6/6)".to_string());
        args.publish_gate = Some("open".to_string());

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("## Self-modifications\n\n- **`AGENTS.md`**: modified"));
        assert!(!content.contains("tools/rust/crates/write-entry/src/main.rs"));
    }

    #[test]
    fn receipt_sha_length_validation_accepts_seven_and_rejects_shorter() {
        let receipts = parse_receipts(&["cycle-start:abc1234".to_string()]).unwrap();
        assert_eq!(receipts.len(), 1);

        let error = parse_receipts(&["cycle-start:abc123".to_string()]).unwrap_err();
        assert!(error.contains("at least 7 hexadecimal characters"));
    }

    #[test]
    fn journal_rejects_duplicate_cycle_entries_for_same_day() {
        let repo_root = TempRepoDir::new("append");
        let now = fixed_now();
        write_root_journal_index(&repo_root.path, "");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        fs::write(
            journal_dir.join("2026-03-05.md"),
            r#"# Journal — 2026-03-05

Reflective log for the schema-org-json-ld orchestrator.

---

## 2026-03-05 — Cycle 153: Prior title

### Concrete commitments for next cycle

1. Dispatch #546 immediately after acceptance.
"#,
        )
        .unwrap();
        write_worklog_fixture(&repo_root.path, now, 154, "From convention to enforcement");
        let payload = r#"{
			"previous_commitment_status":"followed",
			"previous_commitment_detail":"Ran cargo test after PR #543.",
			"sections":[{"heading":"Observation — Enforcement","body":"Audit #117 was right."}],
			"concrete_behavior_change":"Dispatch #546 immediately after acceptance.",
			"open_questions":[]
		}"#;

        let mut file_args = journal_args("From convention to enforcement");
        file_args.input_file = Some(write_input_file(&repo_root.path, "journal.json", payload));

        execute_journal(&file_args, &repo_root.path, now).unwrap();
        let error = execute_journal(&file_args, &repo_root.path, now).unwrap_err();

        let path = journal_path(&repo_root.path, now);
        let content = fs::read_to_string(path).unwrap();
        assert!(content.starts_with("# Journal — 2026-03-06"));
        assert!(
            content.contains("\n---\n\n## 2026-03-06 — Cycle 154: From convention to enforcement")
        );
        assert!(content.contains(
            "Worklog: [cycle 154](../worklog/2026-03-06/051458-cycle-154-from-convention-to-enforcement.md)"
        ));
        assert_eq!(
            content
                .matches("Cycle 154: From convention to enforcement")
                .count(),
            1
        );
        assert_eq!(
            error,
            "journal file already contains an entry for cycle 154 — refusing to append duplicate"
        );
    }

    #[test]
    fn journal_sanitizes_escaped_newlines_before_writing() {
        let repo_root = TempRepoDir::new("sanitize-newlines");
        write_root_journal_index(&repo_root.path, "");
        write_worklog_fixture(&repo_root.path, fixed_now(), 154, "Sanitized newlines");
        let payload = r#"{
			"previous_commitment_status":"no_prior_commitment",
			"previous_commitment_detail":"First line.\nSecond line.",
			"sections":[{"heading":"Observation","body":"Alpha.\nBeta."}],
			"concrete_behavior_change":"Commit once.\nVerify twice.",
			"open_questions":["Should this stay?\nYes."]
		}"#;

        let mut args = journal_args("Sanitized newlines");
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-sanitize-newlines.json",
            payload,
        ));

        let path = execute_journal(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(!content.contains("\\n"));
        assert!(content.contains("**No prior commitment.** First line."));
        assert!(content.contains("Second line."));
        assert!(content.contains("### Observation\n\nAlpha.\nBeta."));
        assert!(content.contains("1. Commit once.\nVerify twice."));
        assert!(content.contains("- Should this stay?\nYes."));
    }

    #[test]
    fn journal_rejects_duplicate_section_headers_within_entry() {
        let repo_root = TempRepoDir::new("duplicate-section-headers");
        write_root_journal_index(&repo_root.path, "");
        write_worklog_fixture(
            &repo_root.path,
            fixed_now(),
            154,
            "Duplicate section headings",
        );
        let payload = r#"{
			"previous_commitment_status":"no_prior_commitment",
			"previous_commitment_detail":"No prior commitment recorded.",
			"sections":[
				{"heading":"Observation","body":"First."},
				{"heading":"Observation","body":"Second."}
			],
			"concrete_behavior_change":"Keep going.",
			"open_questions":[]
		}"#;

        let mut args = journal_args("Duplicate section headings");
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-duplicate-headings.json",
            payload,
        ));

        let error = execute_journal(&args, &repo_root.path, fixed_now()).unwrap_err();
        assert_eq!(
            error,
            "journal entry contains duplicate section header '### Observation' — refusing to write malformed entry"
        );
        assert!(!journal_path(&repo_root.path, fixed_now()).exists());
    }

    #[test]
    fn duplicate_cycle_detection_matches_exact_cycle_number() {
        let existing = concat!(
            "# Journal — 2026-03-06\n\n",
            "Reflective log for the schema-org-json-ld orchestrator.\n\n",
            "---\n\n",
            "## 2026-03-06 — Cycle 154: Existing entry\n"
        );

        assert!(existing_journal_contains_cycle_entry(existing, 154));
        assert!(!existing_journal_contains_cycle_entry(existing, 15));
        assert!(!existing_journal_contains_cycle_entry(existing, 1));
    }

    #[test]
    fn journal_inline_flags_render_worklog_link_and_commitments() {
        let repo_root = TempRepoDir::new("journal-inline-flags");
        write_root_journal_index(&repo_root.path, "");
        write_worklog_fixture(&repo_root.path, fixed_now(), 154, "Cycle reflections");
        let mut args = journal_args("Cycle reflections");
        args.section = vec!["Decisions::Chose to defer #829".to_string()];
        args.commitment = vec!["Will dispatch #830 next cycle".to_string()];

        let path = execute_journal(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains(
            "Worklog: [cycle 154](../worklog/2026-03-06/051458-cycle-154-cycle-reflections.md)"
        ));
        assert!(content.contains("### Decisions"));
        assert!(content.contains(
            "Chose to defer [#829](https://github.com/EvaLok/schema-org-json-ld/issues/829)"
        ));
        assert!(content.contains("### Concrete commitments for next cycle"));
        assert!(content.contains(
            "1. Will dispatch [#830](https://github.com/EvaLok/schema-org-json-ld/issues/830) next cycle"
        ));
    }

    #[test]
    fn journal_inline_flags_support_previous_commitment_override_and_default() {
        let repo_root = TempRepoDir::new("journal-inline-previous-commitment");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        write_root_journal_index(
            &repo_root.path,
            "- [2026-03-05](docs/journal/2026-03-05.md) — Cycles 153+\n",
        );
        write_worklog_fixture(&repo_root.path, fixed_now(), 154, "Inline override");
        fs::write(
            journal_dir.join("2026-03-05.md"),
            r#"# Journal — 2026-03-05

Reflective log for the schema-org-json-ld orchestrator.

---

## 2026-03-05 — Cycle 153: Prior title

### Concrete commitments for next cycle

1. Dispatch #546 in the same cycle.
"#,
        )
        .unwrap();

        let mut explicit_args = journal_args("Inline override");
        explicit_args.section = vec!["Decisions::Closed the loop.".to_string()];
        explicit_args.commitment = vec!["Keep momentum.".to_string()];
        explicit_args.previous_commitment_status = Some("followed".to_string());
        explicit_args.previous_commitment_detail = Some("Done.".to_string());

        let explicit_path = execute_journal(&explicit_args, &repo_root.path, fixed_now()).unwrap();
        let explicit_content = fs::read_to_string(explicit_path).unwrap();
        assert!(explicit_content.contains(
            "> Previous commitment: 1. Dispatch [#546](https://github.com/EvaLok/schema-org-json-ld/issues/546) in the same cycle."
        ));
        assert!(explicit_content.contains("**Followed.** Done."));
        assert!(
            !explicit_content.contains("**No prior commitment.** No prior commitment recorded.")
        );

        let mut default_args = journal_args("Inline default");
        default_args.section = vec!["Notes::Keep notes minimal.".to_string()];
        let default_input = resolve_journal_input(&default_args).unwrap();
        assert_eq!(
            default_input.previous_commitment_status,
            "no_prior_commitment"
        );
        assert_eq!(
            default_input.previous_commitment_detail,
            "No prior commitment recorded."
        );
    }

    #[test]
    fn journal_inline_flags_reject_previous_commitment_status_without_detail() {
        let mut args = journal_args("Partial override");
        args.section = vec!["Decisions::Closed the loop.".to_string()];
        args.previous_commitment_status = Some("followed".to_string());

        let error = resolve_journal_input(&args).unwrap_err();
        assert_eq!(
            error,
            "previous-commitment override requires both --previous-commitment-status and --previous-commitment-detail"
        );
    }

    #[test]
    fn journal_inline_flags_reject_previous_commitment_detail_without_status() {
        let mut args = journal_args("Partial override");
        args.section = vec!["Decisions::Closed the loop.".to_string()];
        args.previous_commitment_detail = Some("Done.".to_string());

        let error = resolve_journal_input(&args).unwrap_err();
        assert_eq!(
            error,
            "previous-commitment override requires both --previous-commitment-status and --previous-commitment-detail"
        );
    }

    #[test]
    fn journal_json_fallback_renders_concrete_behavior_under_commitments_heading() {
        let repo_root = TempRepoDir::new("journal-json-fallback");
        write_root_journal_index(&repo_root.path, "");
        write_worklog_fixture(&repo_root.path, fixed_now(), 154, "JSON fallback");
        let payload = r#"{
            "previous_commitment_status":"no_prior_commitment",
            "previous_commitment_detail":"No prior commitment recorded.",
            "sections":[],
            "concrete_behavior_change":"Keep going.",
            "open_questions":[]
        }"#;
        let mut args = journal_args("JSON fallback");
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-fallback.json",
            payload,
        ));

        let path = execute_journal(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("### Concrete commitments for next cycle"));
        assert!(content.contains("1. Keep going."));
    }

    #[test]
    fn invalid_section_flag_is_rejected() {
        let mut args = journal_args("Invalid section");
        args.section = vec!["Missing delimiter".to_string()];

        let error = resolve_journal_input(&args).unwrap_err();
        assert!(error.contains("invalid section"));
    }

    #[test]
    fn invalid_section_flag_rejects_empty_heading_and_body() {
        let empty_heading = parse_sections(&["  ::Body".to_string()]).unwrap_err();
        assert!(empty_heading.contains("invalid section"));

        let empty_body = parse_sections(&["Heading::   ".to_string()]).unwrap_err();
        assert!(empty_body.contains("invalid section"));
    }

    #[test]
    fn new_journal_date_updates_index_and_finalizes_previous_range() {
        let repo_root = TempRepoDir::new("journal-index-new-date");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        write_root_journal_index(
            &repo_root.path,
            "- [2026-03-05](docs/journal/2026-03-05.md) — Cycles 151+\n",
        );
        fs::write(
            journal_dir.join("2026-03-05.md"),
            concat!(
                "# Journal — 2026-03-05\n\n",
                "Reflective log for the schema-org-json-ld orchestrator.\n\n",
                "---\n\n",
                "## 2026-03-05 — Cycle 151: First\n\n",
                "## 2026-03-05 — Cycle 153: Last\n"
            ),
        )
        .unwrap();

        let payload = r#"{
            "previous_commitment_status":"no_prior_commitment",
            "previous_commitment_detail":"No prior commitment recorded.",
            "sections":[],
            "concrete_behavior_change":"Keep going.",
            "open_questions":[]
        }"#;
        let mut file_args = journal_args("New date");
        file_args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-new-date.json",
            payload,
        ));

        execute_journal(&file_args, &repo_root.path, fixed_now()).unwrap();

        let journal_index = fs::read_to_string(repo_root.path.join("JOURNAL.md")).unwrap();
        assert!(
            journal_index.contains("- [2026-03-05](docs/journal/2026-03-05.md) — Cycles 151–153")
        );
        assert!(journal_index.contains("- [2026-03-06](docs/journal/2026-03-06.md) — Cycles 154+"));
    }

    #[test]
    fn new_journal_date_after_gap_finalizes_latest_open_range() {
        let repo_root = TempRepoDir::new("journal-index-gap-day");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        write_root_journal_index(
            &repo_root.path,
            "- [2026-03-05](docs/journal/2026-03-05.md) — Cycles 151+\n",
        );
        fs::write(
            journal_dir.join("2026-03-05.md"),
            concat!(
                "# Journal — 2026-03-05\n\n",
                "Reflective log for the schema-org-json-ld orchestrator.\n\n",
                "---\n\n",
                "## 2026-03-05 — Cycle 151: First\n\n",
                "## 2026-03-05 — Cycle 153: Last\n"
            ),
        )
        .unwrap();

        let mut args = journal_args("Gap day");
        let payload = r#"{
            "previous_commitment_status":"no_prior_commitment",
            "previous_commitment_detail":"No prior commitment recorded.",
            "sections":[],
            "concrete_behavior_change":"Keep going.",
            "open_questions":[]
        }"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-gap-day.json",
            payload,
        ));

        execute_journal(&args, &repo_root.path, fixed_now_on("2026-03-07")).unwrap();

        let journal_index = fs::read_to_string(repo_root.path.join("JOURNAL.md")).unwrap();
        assert!(
            journal_index.contains("- [2026-03-05](docs/journal/2026-03-05.md) — Cycles 151–153")
        );
        assert!(journal_index.contains("- [2026-03-07](docs/journal/2026-03-07.md) — Cycles 154+"));
    }

    #[test]
    fn new_journal_date_after_multiple_day_gap_finalizes_latest_open_range() {
        let repo_root = TempRepoDir::new("journal-index-multi-day-gap");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        write_root_journal_index(
            &repo_root.path,
            "- [2026-03-03](docs/journal/2026-03-03.md) — Cycles 151+\n",
        );
        fs::write(
            journal_dir.join("2026-03-03.md"),
            concat!(
                "# Journal — 2026-03-03\n\n",
                "Reflective log for the schema-org-json-ld orchestrator.\n\n",
                "---\n\n",
                "## 2026-03-03 — Cycle 151: First\n\n",
                "## 2026-03-03 — Cycle 160: Last\n"
            ),
        )
        .unwrap();

        let mut args = journal_args("Multi day gap");
        args.cycle = Some(161);
        let payload = r#"{
            "previous_commitment_status":"no_prior_commitment",
            "previous_commitment_detail":"No prior commitment recorded.",
            "sections":[],
            "concrete_behavior_change":"Keep going.",
            "open_questions":[]
        }"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-multi-gap.json",
            payload,
        ));

        execute_journal(&args, &repo_root.path, fixed_now_on("2026-03-07")).unwrap();

        let journal_index = fs::read_to_string(repo_root.path.join("JOURNAL.md")).unwrap();
        assert!(
            journal_index.contains("- [2026-03-03](docs/journal/2026-03-03.md) — Cycles 151–160")
        );
        assert!(journal_index.contains("- [2026-03-07](docs/journal/2026-03-07.md) — Cycles 161+"));
    }

    #[test]
    fn first_journal_date_appends_new_index_entry_without_crashing() {
        let repo_root = TempRepoDir::new("journal-index-first-date");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        write_root_journal_index(&repo_root.path, "");

        let mut args = journal_args("First date");
        let payload = r#"{
            "previous_commitment_status":"no_prior_commitment",
            "previous_commitment_detail":"No prior commitment recorded.",
            "sections":[],
            "concrete_behavior_change":"Keep going.",
            "open_questions":[]
        }"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-first-date.json",
            payload,
        ));

        execute_journal(&args, &repo_root.path, fixed_now()).unwrap();

        let journal_index = fs::read_to_string(repo_root.path.join("JOURNAL.md")).unwrap();
        assert!(journal_index.contains("- [2026-03-06](docs/journal/2026-03-06.md) — Cycles 154+"));
    }

    #[test]
    fn new_journal_date_leaves_closed_previous_index_entry_unchanged() {
        let repo_root = TempRepoDir::new("journal-index-closed-previous-entry");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        let initial_index = "- [2026-03-05](docs/journal/2026-03-05.md) — Cycles 151–160\n";
        write_root_journal_index(&repo_root.path, initial_index);

        let mut args = journal_args("Closed previous");
        args.cycle = Some(161);
        let payload = r#"{
            "previous_commitment_status":"no_prior_commitment",
            "previous_commitment_detail":"No prior commitment recorded.",
            "sections":[],
            "concrete_behavior_change":"Keep going.",
            "open_questions":[]
        }"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-closed-previous.json",
            payload,
        ));

        execute_journal(&args, &repo_root.path, fixed_now_on("2026-03-07")).unwrap();

        let journal_index = fs::read_to_string(repo_root.path.join("JOURNAL.md")).unwrap();
        assert!(
            journal_index.contains("- [2026-03-05](docs/journal/2026-03-05.md) — Cycles 151–160")
        );
        assert!(journal_index.contains("- [2026-03-07](docs/journal/2026-03-07.md) — Cycles 161+"));
    }

    #[test]
    fn appending_to_existing_journal_date_does_not_modify_index() {
        let repo_root = TempRepoDir::new("journal-index-existing-date");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        let initial_index = concat!(
            "- [2026-03-05](docs/journal/2026-03-05.md) — Cycles 151–153\n",
            "- [2026-03-06](docs/journal/2026-03-06.md) — Cycles 154+\n"
        );
        write_root_journal_index(&repo_root.path, initial_index);
        fs::write(
            journal_dir.join("2026-03-06.md"),
            concat!(
                "# Journal — 2026-03-06\n\n",
                "Reflective log for the schema-org-json-ld orchestrator.\n\n",
                "---\n\n",
                "## 2026-03-06 — Cycle 154: Existing\n"
            ),
        )
        .unwrap();

        let mut args = journal_args("Append");
        args.cycle = Some(155);
        let payload = r#"{
            "previous_commitment_status":"no_prior_commitment",
            "previous_commitment_detail":"No prior commitment recorded.",
            "sections":[],
            "concrete_behavior_change":"Keep going.",
            "open_questions":[]
        }"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-append.json",
            payload,
        ));

        execute_journal(&args, &repo_root.path, fixed_now()).unwrap();

        let journal_index = fs::read_to_string(repo_root.path.join("JOURNAL.md")).unwrap();
        assert_eq!(
            journal_index,
            format!(
                "# Journal\n\nJournal entries have been split into per-date files in [`docs/journal/`](docs/journal/).\n\n{}",
                initial_index
            )
        );
    }

    #[test]
    fn appending_to_existing_journal_date_adds_missing_index_entry() {
        let repo_root = TempRepoDir::new("journal-index-missing-entry");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        // Index only has 2026-03-05 — missing 2026-03-06 entry
        let initial_index = "- [2026-03-05](docs/journal/2026-03-05.md) — Cycles 151–153\n";
        write_root_journal_index(&repo_root.path, initial_index);
        // But the 2026-03-06 journal file already exists on disk
        fs::write(
            journal_dir.join("2026-03-06.md"),
            concat!(
                "# Journal — 2026-03-06\n\n",
                "Reflective log for the schema-org-json-ld orchestrator.\n\n",
                "---\n\n",
                "## 2026-03-06 — Cycle 154: Existing\n"
            ),
        )
        .unwrap();

        let mut args = journal_args("Append to missing index");
        args.cycle = Some(155);
        let payload = r#"{
            "previous_commitment_status":"no_prior_commitment",
            "previous_commitment_detail":"No prior commitment recorded.",
            "sections":[],
            "concrete_behavior_change":"Keep going.",
            "open_questions":[]
        }"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-missing-index.json",
            payload,
        ));

        execute_journal(&args, &repo_root.path, fixed_now()).unwrap();

        let journal_index = fs::read_to_string(repo_root.path.join("JOURNAL.md")).unwrap();
        // The missing date should now appear in the index
        assert!(
            journal_index.contains("- [2026-03-06](docs/journal/2026-03-06.md) — Cycles 155+"),
            "expected missing date to be added to index, got: {journal_index}"
        );
    }

    #[test]
    fn journal_includes_previous_commitment_quote_from_last_entry() {
        let repo_root = TempRepoDir::new("previous");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        write_root_journal_index(
            &repo_root.path,
            "- [2026-03-05](docs/journal/2026-03-05.md) — Cycles 153+\n",
        );
        let existing = r#"# Journal — 2026-03-05

Reflective log for the schema-org-json-ld orchestrator.

---

## 2026-03-05 — Cycle 153: Prior title

### Concrete behavior change this cycle

When accepting recommendations, dispatch #546 in the same cycle.
"#;
        fs::write(journal_dir.join("2026-03-05.md"), existing).unwrap();

        let mut args = journal_args("New title");
        let payload = r#"{
            "previous_commitment_status":"followed",
            "previous_commitment_detail":"Done.",
			"sections":[],
			"concrete_behavior_change":"Keep going.",
			"open_questions":[]
		}"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-previous.json",
            payload,
        ));
        execute_journal(&args, &repo_root.path, fixed_now()).unwrap();

        let content = fs::read_to_string(journal_path(&repo_root.path, fixed_now())).unwrap();
        assert!(content.contains("> Previous commitment: When accepting recommendations, dispatch [#546](https://github.com/EvaLok/schema-org-json-ld/issues/546) in the same cycle."));
    }

    #[test]
    fn journal_extracts_previous_commitment_from_new_heading_format() {
        let repo_root = TempRepoDir::new("previous-new-heading");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        write_root_journal_index(
            &repo_root.path,
            "- [2026-03-05](docs/journal/2026-03-05.md) — Cycles 153+\n",
        );
        write_worklog_fixture(&repo_root.path, fixed_now(), 154, "New heading");
        let existing = r#"# Journal — 2026-03-05

Reflective log for the schema-org-json-ld orchestrator.

---

## 2026-03-05 — Cycle 153: Prior title

### Concrete commitments for next cycle

1. Dispatch #546 in the same cycle.
"#;
        fs::write(journal_dir.join("2026-03-05.md"), existing).unwrap();

        let payload = r#"{
			"previous_commitment_status":"followed",
			"previous_commitment_detail":"Done.",
			"sections":[],
			"concrete_behavior_change":"Keep going.",
			"open_questions":[]
		}"#;
        let mut args = journal_args("New heading");
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-new-heading.json",
            payload,
        ));
        execute_journal(&args, &repo_root.path, fixed_now()).unwrap();

        let content = fs::read_to_string(journal_path(&repo_root.path, fixed_now())).unwrap();
        assert!(content.contains(
            "> Previous commitment: 1. Dispatch [#546](https://github.com/EvaLok/schema-org-json-ld/issues/546) in the same cycle."
        ));
    }

    #[test]
    fn invalid_previous_commitment_status_is_rejected() {
        let repo_root = TempRepoDir::new("status");
        let mut args = journal_args("Invalid status");
        let payload = r#"{
			"previous_commitment_status":"unknown",
			"previous_commitment_detail":"Done.",
			"sections":[],
			"concrete_behavior_change":"Keep going.",
			"open_questions":[]
		}"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-invalid-status.json",
            payload,
        ));
        let error = execute_journal(&args, &repo_root.path, fixed_now()).unwrap_err();
        assert!(error.contains("invalid previous_commitment_status"));
    }

    #[test]
    fn journal_rejects_default_no_prior_status_when_previous_commitment_exists() {
        let repo_root = TempRepoDir::new("previous-contradiction");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        write_root_journal_index(
            &repo_root.path,
            "- [2026-03-05](docs/journal/2026-03-05.md) — Cycles 153+\n",
        );
        fs::write(
            journal_dir.join("2026-03-05.md"),
            r#"# Journal — 2026-03-05

Reflective log for the schema-org-json-ld orchestrator.

---

## 2026-03-05 — Cycle 153: Prior title

### Concrete commitments for next cycle

1. Dispatch #546 in the same cycle.
"#,
        )
        .unwrap();

        let mut args = journal_args("New title");
        let payload = r#"{
			"sections":[],
			"concrete_behavior_change":"Keep going.",
			"open_questions":[]
		}"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-contradiction.json",
            payload,
        ));

        let error = execute_journal(&args, &repo_root.path, fixed_now()).unwrap_err();
        assert_eq!(
            error,
            "previous commitment found in journal but --previous-commitment-status is 'no_prior_commitment'; specify an explicit status (followed, not_followed, not_applicable)"
        );
    }

    #[test]
    fn journal_rejects_explicit_status_when_no_previous_commitment_exists() {
        let repo_root = TempRepoDir::new("no-previous-history");
        write_root_journal_index(&repo_root.path, "");

        let mut args = journal_args("New title");
        let payload = r#"{
			"previous_commitment_status":"followed",
			"previous_commitment_detail":"Done.",
			"sections":[],
			"concrete_behavior_change":"Keep going.",
			"open_questions":[]
		}"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-no-history.json",
            payload,
        ));

        let error = execute_journal(&args, &repo_root.path, fixed_now()).unwrap_err();
        assert_eq!(
            error,
            "--previous-commitment-status is set but no previous commitment found in journal history"
        );
    }

    #[test]
    fn cli_accepts_repo_root_before_subcommand() {
        let cli = Cli::try_parse_from([
            "write-entry",
            "--repo-root",
            "/tmp/example",
            "worklog",
            "--cycle",
            "1",
            "--title",
            "test",
        ])
        .unwrap();
        assert_eq!(cli.repo_root, PathBuf::from("/tmp/example"));
        match cli.command {
            Command::Worklog(args) => {
                assert_eq!(args.cycle, Some(1));
                assert_eq!(args.title, "test");
                assert!(args.input_file.is_none());
            }
            Command::Journal(_) => panic!("expected worklog command"),
        }
    }

    #[test]
    fn cli_uses_default_repo_root_when_omitted() {
        let cli = Cli::try_parse_from(["write-entry", "worklog", "--title", "test"]).unwrap();
        assert_eq!(cli.repo_root, PathBuf::from("."));
        match cli.command {
            Command::Worklog(args) => {
                assert_eq!(args.cycle, None);
                assert!(args.done.is_empty());
                assert!(args.receipt.is_empty());
            }
            Command::Journal(_) => panic!("expected worklog command"),
        }
    }

    #[test]
    fn cli_parses_worklog_input_file_and_inline_flags() {
        let cli = Cli::try_parse_from([
            "write-entry",
            "worklog",
            "--title",
            "test",
            "--input-file",
            "/tmp/worklog.json",
            "--done",
            "Merged PR #123",
            "--pr-merged",
            "123",
            "--issues-processed",
            "42,43",
            "--next",
            "Review PR #124",
            "--pipeline",
            "PASS (6/6)",
            "--publish-gate",
            "open",
            "--receipt",
            "cycle-start:abc1234",
        ])
        .unwrap();

        match cli.command {
            Command::Worklog(args) => {
                assert_eq!(args.input_file, Some(PathBuf::from("/tmp/worklog.json")));
                assert_eq!(args.done, vec!["Merged PR #123".to_string()]);
                assert_eq!(args.pr_merged, vec![123]);
                assert!(args.pr_reviewed.is_empty());
                assert!(args.issue_processed.is_empty());
                assert_eq!(
                    args.issues_processed,
                    vec!["42".to_string(), "43".to_string()]
                );
                assert!(args.self_modification.is_empty());
                assert_eq!(args.next, vec!["Review PR #124".to_string()]);
                assert_eq!(args.pipeline.as_deref(), Some("PASS (6/6)"));
                assert_eq!(args.receipt, vec!["cycle-start:abc1234".to_string()]);
            }
            Command::Journal(_) => panic!("expected worklog command"),
        }
    }

    #[test]
    fn cli_parses_new_worklog_tracking_flags() {
        let cli = Cli::try_parse_from([
            "write-entry",
            "worklog",
            "--title",
            "test",
            "--issues-processed",
            "924,925",
            "--auto-issues",
            "--auto-review-summary",
            "--review-issue",
            "777",
            "--auto-next",
            "--auto-pipeline",
            "--auto-gate-history",
            "--auto-self-modifications",
            "--auto-receipts",
            "--auto-receipt-note",
            "--dry-run",
            "--done",
            "did stuff",
            "--pr-reviewed",
            "123",
            "--issue-processed",
            "Closed EvaLok/schema-org-json-ld#924 (cycle review)",
            "--self-modification",
            "Updated AGENTS.md",
            "--prior-gate-failures",
            "C4.1 FAIL: mismatch,C5.5 FAIL: doc-validation",
        ])
        .unwrap();

        match cli.command {
            Command::Worklog(args) => {
                assert_eq!(
                    args.issues_processed,
                    vec!["924".to_string(), "925".to_string()]
                );
                assert!(args.auto_issues);
                assert!(args.auto_review_summary);
                assert_eq!(args.review_issue, Some(777));
                assert!(args.auto_next);
                assert!(args.auto_pipeline);
                assert!(args.auto_gate_history);
                assert!(args.auto_self_modifications);
                assert!(args.auto_receipts);
                assert!(args.auto_receipt_note);
                assert!(args.dry_run);
                assert_eq!(args.pr_reviewed, vec![123]);
                assert_eq!(
                    args.issue_processed,
                    vec!["Closed EvaLok/schema-org-json-ld#924 (cycle review)".to_string()]
                );
                assert_eq!(
                    args.self_modification,
                    vec!["Updated AGENTS.md".to_string()]
                );
                assert_eq!(
                    args.prior_gate_failures,
                    vec![
                        "C4.1 FAIL: mismatch".to_string(),
                        "C5.5 FAIL: doc-validation".to_string()
                    ]
                );
            }
            Command::Journal(_) => panic!("expected worklog command"),
        }
    }

    #[test]
    fn cli_rejects_removed_in_flight_flag() {
        let error = match Cli::try_parse_from([
            "write-entry",
            "worklog",
            "--title",
            "test",
            "--in-flight",
            "1",
        ]) {
            Ok(_) => panic!("--in-flight should no longer parse"),
            Err(error) => error,
        };

        assert!(error.to_string().contains("--in-flight"));
    }

    #[test]
    fn worklog_derives_cycle_from_state_when_omitted() {
        let repo_root = TempRepoDir::new("worklog-derived-cycle");
        init_git_repo(&repo_root.path);
        let receipt = create_git_commit(&repo_root.path, "notes/derived-cycle.txt", "derived\n");
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[{{"step":"manual","receipt":"{receipt}","commit":"Add notes/derived-cycle.txt"}}]"#
            ),
        );
        fs::create_dir_all(repo_root.path.join("docs")).unwrap();
        fs::write(
            repo_root.path.join("docs/state.json"),
            "{\n  \"last_cycle\": {\"number\": 167},\n  \"cycle_phase\": {\"cycle\": 168}\n}\n",
        )
        .unwrap();
        let mut args = worklog_args("Derived cycle");
        args.cycle = None;
        let payload = r#"{
			"what_was_done":["Checked #42"],
			"self_modifications":[],
			"prs_merged":[],
			"prs_reviewed":[],
			"issues_processed":[],
			"current_state":{
				"in_flight_sessions":0,
				"pipeline_status":"pass",
				"copilot_metrics":"steady",
				"publish_gate":"clear"
			},
			"next_steps":[]
		}"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "worklog-derived-cycle.json",
            payload,
        ));

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("# Cycle 168 — 2026-03-06 05:14 UTC"));
    }

    #[test]
    fn journal_derives_cycle_from_state_when_omitted() {
        let repo_root = TempRepoDir::new("journal-derived-cycle");
        fs::create_dir_all(repo_root.path.join("docs")).unwrap();
        write_root_journal_index(&repo_root.path, "");
        write_worklog_fixture(&repo_root.path, fixed_now(), 168, "Derived cycle");
        fs::write(
            repo_root.path.join("docs/state.json"),
            "{\n  \"last_cycle\": {\"number\": 167},\n  \"cycle_phase\": {\"cycle\": 168}\n}\n",
        )
        .unwrap();
        let mut args = journal_args("Derived cycle");
        args.cycle = None;
        let payload = r#"{
			"previous_commitment_status":"no_prior_commitment",
			"previous_commitment_detail":"No prior commitment recorded.",
			"sections":[],
			"concrete_behavior_change":"Keep going.",
			"open_questions":[]
		}"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-derived-cycle.json",
            payload,
        ));

        let path = execute_journal(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("## 2026-03-06 — Cycle 168: Derived cycle"));
    }

    #[test]
    fn journal_preserves_cycle_phase_state_from_disk() {
        let repo_root = TempRepoDir::new("journal-preserves-cycle-phase");
        fs::create_dir_all(repo_root.path.join("docs")).unwrap();
        write_root_journal_index(&repo_root.path, "");
        write_worklog_fixture(&repo_root.path, fixed_now(), 168, "Cycle phase preserved");
        let original_state = "{\n  \"last_cycle\": {\"number\": 168},\n  \"cycle_phase\": {\n    \"cycle\": 168,\n    \"phase\": \"complete\",\n    \"phase_entered_at\": \"2026-03-06T05:00:00Z\",\n    \"completed_at\": \"2026-03-06T05:10:00Z\"\n  }\n}\n";
        fs::write(repo_root.path.join("docs/state.json"), original_state).unwrap();
        let mut args = journal_args("Cycle phase preserved");
        args.cycle = None;
        let payload = r#"{
			"previous_commitment_status":"no_prior_commitment",
			"previous_commitment_detail":"No prior commitment recorded.",
			"sections":[],
			"concrete_behavior_change":"Keep going.",
			"open_questions":[]
		}"#;
        args.input_file = Some(write_input_file(
            &repo_root.path,
            "journal-preserves-cycle-phase.json",
            payload,
        ));

        execute_journal(&args, &repo_root.path, fixed_now()).unwrap();

        let state_after = fs::read_to_string(repo_root.path.join("docs/state.json")).unwrap();
        assert_eq!(state_after, original_state);
    }
}
