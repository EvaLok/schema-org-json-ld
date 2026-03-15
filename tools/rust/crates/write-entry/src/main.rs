use chrono::{DateTime, NaiveDate, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Deserializer};
use state_schema::{current_cycle_from_state, read_state_value, AgentSession, StateJson};
use std::collections::{HashMap, HashSet};
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
const PIPELINE_STATUS_PREFIX: &str = "- **Pipeline status**: ";
const INFRASTRUCTURE_ROOTS: [&str; 2] = ["tools", ".claude/skills"];
const INFRASTRUCTURE_FILES: [&str; 4] = [
    "STARTUP_CHECKLIST.md",
    "COMPLETION_CHECKLIST.md",
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
    /// Patch the pipeline status line in an existing worklog entry file
    PatchPipeline(PatchPipelineArgs),
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
    /// Self-modification description, optionally in FILE:DESCRIPTION form
    #[arg(long = "self-modification")]
    self_modification: Vec<String>,
    /// Next step for the following cycle
    #[arg(long = "next")]
    next: Vec<String>,
    /// Pipeline summary for the current state section
    #[arg(long)]
    pipeline: Option<String>,
    /// Copilot metrics summary for the current state section
    #[arg(long = "copilot-metrics")]
    copilot_metrics: Option<String>,
    /// Publish gate summary for the current state section
    #[arg(long = "publish-gate")]
    publish_gate: Option<String>,
    /// Number of in-flight agent sessions
    #[arg(long = "in-flight")]
    in_flight: Option<u64>,
    /// Commit receipt in TOOL:SHA form
    #[arg(long = "receipt")]
    receipt: Vec<String>,
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

#[derive(Parser)]
struct PatchPipelineArgs {
    /// Path to the worklog file to patch
    #[arg(long)]
    worklog: PathBuf,
    /// Replacement pipeline status text
    #[arg(long)]
    status: String,
}

#[derive(Debug, Deserialize)]
struct WorklogInput {
    #[serde(default)]
    what_was_done: Vec<String>,
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
    copilot_metrics: String,
    #[serde(default)]
    publish_gate: String,
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
    unresolved: bool,
}

#[derive(Debug, Deserialize)]
struct CycleReceiptJsonEntry {
    #[serde(alias = "step")]
    tool: String,
    #[serde(alias = "hash")]
    receipt: String,
    #[serde(default, alias = "message")]
    commit: String,
}

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

    let result = match cli.command {
        Command::Worklog(args) => execute_worklog(&args, &repo_root, now),
        Command::Journal(args) => execute_journal(&args, &repo_root, now),
        Command::PatchPipeline(args) => execute_patch_pipeline(&args, &repo_root),
    };

    match result {
        Ok(path) => println!("{}", path.display()),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
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

fn execute_worklog(
    args: &WorklogArgs,
    repo_root: &Path,
    now: DateTime<Utc>,
) -> Result<PathBuf, String> {
    let cycle = resolve_cycle(args.cycle, repo_root)?;
    let mut input = resolve_worklog_input(args, repo_root)?;
    emit_worklog_auto_derivation_warnings(apply_worklog_auto_derivations(
        args, repo_root, cycle, &mut input,
    )?);
    emit_unresolved_receipt_warnings(&mut input.receipts, repo_root)?;
    let path = worklog_path(repo_root, now, &args.title);
    let content = render_worklog(cycle, now, &input);
    emit_generated_markdown_sha_warnings("worklog", &content, repo_root);
    write_entry_file(&path, &content)?;
    Ok(path)
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
    let worklog_link = find_worklog_relative_path(repo_root, cycle)?;
    let entry = render_journal_entry(
        cycle,
        now,
        &args.title,
        &input,
        status,
        previous.as_deref(),
        worklog_link.as_deref(),
    );
    emit_generated_markdown_sha_warnings("journal", &entry, repo_root);
    write_journal_file(&path, now.date_naive(), &entry)?;
    update_journal_index(repo_root, now.date_naive(), cycle)?;
    Ok(path)
}

fn execute_patch_pipeline(args: &PatchPipelineArgs, repo_root: &Path) -> Result<PathBuf, String> {
    let worklog_path = resolve_repo_path(repo_root, &args.worklog);
    let content = fs::read_to_string(&worklog_path)
        .map_err(|error| format!("failed to read {}: {}", worklog_path.display(), error))?;
    let patched = patch_pipeline_status_line(&content, &args.status).ok_or_else(|| {
        format!(
            "failed to patch {}: pipeline status line not found",
            worklog_path.display()
        )
    })?;
    fs::write(&worklog_path, patched)
        .map_err(|error| format!("failed to write {}: {}", worklog_path.display(), error))?;
    Ok(worklog_path)
}

fn resolve_repo_path(repo_root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        repo_root.join(path)
    }
}

fn patch_pipeline_status_line(content: &str, status: &str) -> Option<String> {
    let start = content
        .match_indices(PIPELINE_STATUS_PREFIX)
        .find_map(|(index, _)| {
            if index == 0 || content.as_bytes().get(index - 1) == Some(&b'\n') {
                Some(index)
            } else {
                None
            }
        })?;
    let search = &content[start..];
    let line_end = search
        .find('\n')
        .map(|offset| start + offset)
        .unwrap_or(content.len());
    let suffix_start = if line_end > start && content.as_bytes().get(line_end - 1) == Some(&b'\r') {
        line_end - 1
    } else {
        line_end
    };

    Some(format!(
        "{}{}{}{}",
        &content[..start],
        PIPELINE_STATUS_PREFIX,
        status,
        &content[suffix_start..]
    ))
}

fn resolve_cycle(cycle: Option<u64>, repo_root: &Path) -> Result<u64, String> {
    match cycle {
        Some(cycle) => Ok(cycle),
        None => current_cycle_from_state(repo_root),
    }
}

fn resolve_worklog_input(args: &WorklogArgs, repo_root: &Path) -> Result<WorklogInput, String> {
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
            self_modifications: parse_self_modifications(&args.self_modification)?,
            prs_merged: args.pr_merged.clone(),
            prs_reviewed: args.pr_reviewed.clone(),
            issues_processed: parse_issue_processed(&args.issue_processed)?,
            current_state: CurrentState {
                in_flight_sessions: match args.in_flight {
                    Some(value) => value,
                    None => state_copilot_in_flight(state.as_ref())?,
                },
                pipeline_status: args
                    .pipeline
                    .clone()
                    .unwrap_or_else(|| NOT_PROVIDED.to_string()),
                copilot_metrics: match &args.copilot_metrics {
                    Some(value) => value.clone(),
                    None => format_state_copilot_metrics(state.as_ref())?,
                },
                publish_gate: match &args.publish_gate {
                    Some(value) => value.clone(),
                    None => state_publish_gate_status(state.as_ref())?,
                },
            },
            next_steps: args.next.clone(),
            receipts: parse_receipts(&args.receipt)?,
            receipt_note: None,
        };
        validate_worklog_state_placeholders(&input, state.as_ref())?;
        return Ok(input);
    }

    let state = load_worklog_state(repo_root, true)?;
    let input = WorklogInput {
        what_was_done: Vec::new(),
        self_modifications: Vec::new(),
        prs_merged: Vec::new(),
        prs_reviewed: Vec::new(),
        issues_processed: Vec::new(),
        current_state: CurrentState {
            in_flight_sessions: state_copilot_in_flight(state.as_ref())?,
            pipeline_status: state_pipeline_status(state.as_ref()),
            copilot_metrics: format_state_copilot_metrics(state.as_ref())?,
            publish_gate: state_publish_gate_status(state.as_ref())?,
        },
        next_steps: Vec::new(),
        receipts: Vec::new(),
        receipt_note: None,
    };
    validate_worklog_state_placeholders(&input, state.as_ref())?;
    Ok(input)
}

fn requires_worklog_state(args: &WorklogArgs) -> bool {
    args.copilot_metrics.is_none() || args.publish_gate.is_none() || args.in_flight.is_none()
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

fn format_state_copilot_metrics(state: Option<&StateJson>) -> Result<String, String> {
    let state = state
        .ok_or_else(|| "docs/state.json is required to populate copilot metrics".to_string())?;
    let total_dispatches = state
        .copilot_metrics
        .total_dispatches
        .ok_or_else(|| "missing copilot_metrics.total_dispatches in state.json".to_string())?;
    if total_dispatches < 0 {
        return Err(
            "copilot_metrics.total_dispatches must be non-negative in state.json".to_string(),
        );
    }
    let produced_pr = state
        .copilot_metrics
        .produced_pr
        .ok_or_else(|| "missing copilot_metrics.produced_pr in state.json".to_string())?;
    if produced_pr < 0 {
        return Err("copilot_metrics.produced_pr must be non-negative in state.json".to_string());
    }
    let merged = state
        .copilot_metrics
        .merged
        .ok_or_else(|| "missing copilot_metrics.merged in state.json".to_string())?;
    if merged < 0 {
        return Err("copilot_metrics.merged must be non-negative in state.json".to_string());
    }
    let pr_merge_rate = state
        .copilot_metrics
        .pr_merge_rate
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "missing copilot_metrics.pr_merge_rate in state.json".to_string())?;

    Ok(format!(
        "{} dispatches, {} PRs produced, {} merged, {} PR merge rate",
        total_dispatches, produced_pr, merged, pr_merge_rate
    ))
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

fn state_copilot_in_flight(state: Option<&StateJson>) -> Result<u64, String> {
    let state = state.ok_or_else(|| {
        "docs/state.json is required to populate in-flight agent sessions".to_string()
    })?;
    let in_flight = state
        .copilot_metrics
        .in_flight
        .ok_or_else(|| "missing copilot_metrics.in_flight in state.json".to_string())?;
    u64::try_from(in_flight)
        .map_err(|_| "copilot_metrics.in_flight must be non-negative in state.json".to_string())
}

fn validate_worklog_state_placeholders(
    input: &WorklogInput,
    state: Option<&StateJson>,
) -> Result<(), String> {
    let Some(state) = state else {
        return Ok(());
    };

    if input.current_state.copilot_metrics == NOT_PROVIDED
        && state_has_copilot_metrics_summary(state)
    {
        return Err("copilot metrics cannot be 'Not provided.' when docs/state.json contains copilot_metrics data".to_string());
    }

    if input.current_state.publish_gate == NOT_PROVIDED && state_has_publish_gate_status(state) {
        return Err("publish gate cannot be 'Not provided.' when docs/state.json contains publish_gate.status".to_string());
    }

    Ok(())
}

fn state_has_copilot_metrics_summary(state: &StateJson) -> bool {
    state.copilot_metrics.total_dispatches.is_some()
        || state.copilot_metrics.produced_pr.is_some()
        || state.copilot_metrics.merged.is_some()
        || state
            .copilot_metrics
            .pr_merge_rate
            .as_deref()
            .map(str::trim)
            .is_some_and(|value| !value.is_empty())
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
        || !args.self_modification.is_empty()
        || !args.next.is_empty()
        || args.pipeline.is_some()
        || args.copilot_metrics.is_some()
        || args.publish_gate.is_some()
        || args.in_flight.is_some()
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

    if input.self_modifications.is_empty() {
        match derive_self_modifications(repo_root, cycle) {
            Ok(self_modifications) => input.self_modifications = self_modifications,
            Err(error) => warnings.push(format!(
                "WARNING: failed to auto-derive self-modifications for cycle {}: {}",
                cycle, error
            )),
        }
    }

    let mut auto_issues = derive_issue_processed_from_done(&input.what_was_done);
    let derived_issues_from_done = !auto_issues.is_empty();
    match derive_issue_processed_from_git_history(repo_root, cycle) {
        Ok(issues) => auto_issues = merge_issue_processed(&auto_issues, &issues),
        Err(error) if !derived_issues_from_done => warnings.push(format!(
            "WARNING: failed to auto-derive issues processed from git history for cycle {}: {}",
            cycle, error
        )),
        Err(_) => {}
    }
    match load_worklog_state(repo_root, false) {
        Ok(Some(state)) => match derive_issue_processed_from_state(repo_root, cycle, &state) {
            Ok(issues) => auto_issues = merge_issue_processed(&auto_issues, &issues),
            Err(error) if !derived_issues_from_done => warnings.push(format!(
                "WARNING: failed to auto-derive issues processed from docs/state.json for cycle {}: {}",
                cycle, error
            )),
            Err(_) => {}
        },
        Ok(None) => {}
        Err(error) if !derived_issues_from_done => warnings.push(format!(
            "WARNING: failed to read docs/state.json for issues processed auto-derivation in cycle {}: {}",
            cycle, error
        )),
        Err(_) => {}
    }
    input.issues_processed = merge_issue_processed(&input.issues_processed, &auto_issues);

    let manual_receipts = parse_receipts(&args.receipt)?;
    input.receipt_note = None;
    match derive_cycle_receipt_entries(repo_root, cycle) {
        Ok(entries) => {
            let receipts = cycle_receipt_entries_to_receipts(&entries)?;
            input.receipts = merge_receipts(receipts, &manual_receipts);
            let derived_prs = derive_prs_from_cycle_receipt_entries(&entries);
            input.prs_merged = merge_numbered_refs(&input.prs_merged, &derived_prs);
            input.receipt_note = Some(
                "Scope: all commits through cycle-complete. Docs and record-dispatch commits are structurally excluded (created post-worklog). Validated by receipt-validate at step C5.1."
                    .to_string(),
            );
        }
        Err(error) if !manual_receipts.is_empty() => {
            warnings.push(format!(
                "WARNING: failed to auto-derive receipts for cycle {}: {}; using provided manual receipts instead",
                cycle, error
            ));
            input.receipt_note = Some(
                "Automatic receipt collection via `tools/cycle-receipts` failed; using provided manual receipts instead."
                    .to_string(),
            );
            input.receipts = manual_receipts;
        }
        Err(error) => return Err(error),
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
    let references = extract_issue_references(item);
    if references.len() == 1 {
        format!("#{}", references[0])
    } else {
        item.trim().to_ascii_lowercase()
    }
}

fn derive_issue_processed_from_done(items: &[String]) -> Vec<String> {
    let mut seen = HashSet::new();
    let mut issues = Vec::new();

    for item in items {
        if !done_item_has_issue_action(item) {
            continue;
        }

        for issue in extract_issue_references(item) {
            if seen.insert(issue) {
                issues.push(format!("#{}", issue));
            }
        }
    }

    issues
}

fn derive_issue_processed_from_git_history(
    repo_root: &Path,
    cycle: u64,
) -> Result<Vec<String>, String> {
    let commits = read_git_history(repo_root)?;
    let (start, end) = cycle_history_window(repo_root, cycle, &commits)?;
    let mut seen = HashSet::new();
    let mut issues = Vec::new();

    for commit in commits.iter().filter(|commit| commit.committed_at >= start) {
        if end.is_some_and(|timestamp| commit.committed_at >= timestamp) {
            continue;
        }
        if !done_item_has_issue_action(&commit.subject) {
            continue;
        }
        for issue in extract_issue_references(&commit.subject) {
            let issue_ref = format!("#{}", issue);
            if seen.insert(issue_ref.clone()) {
                issues.push(issue_ref);
            }
        }
    }

    Ok(issues)
}

fn derive_issue_processed_from_state(
    repo_root: &Path,
    cycle: u64,
    state: &StateJson,
) -> Result<Vec<String>, String> {
    let commits = read_git_history(repo_root)?;
    let (start, end) = cycle_history_window(repo_root, cycle, &commits)?;
    let mut seen = HashSet::new();
    let mut issues = Vec::new();

    for session in &state.agent_sessions {
        if !agent_session_status_looks_processed(session) {
            continue;
        }
        let Some(issue) = session.issue.and_then(|value| u64::try_from(value).ok()) else {
            continue;
        };
        let Some(changed_at) = agent_session_status_changed_at(session) else {
            continue;
        };
        if changed_at < start || end.is_some_and(|timestamp| changed_at >= timestamp) {
            continue;
        }
        let issue_ref = format!("#{}", issue);
        if seen.insert(issue_ref.clone()) {
            issues.push(issue_ref);
        }
    }

    Ok(issues)
}

fn agent_session_status_looks_processed(session: &AgentSession) -> bool {
    session.status.as_deref().is_some_and(|status| {
        matches!(
            status.trim().to_ascii_lowercase().as_str(),
            "merged" | "closed" | "resolved" | "completed"
        )
    })
}

fn agent_session_status_changed_at(session: &AgentSession) -> Option<DateTime<Utc>> {
    session
        .merged_at
        .as_deref()
        .and_then(parse_optional_timestamp)
        .or_else(|| {
            AGENT_SESSION_STATUS_TIMESTAMP_FIELDS
                .iter()
                .find_map(|key| {
                    session
                        .extra
                        .get(*key)
                        .and_then(|value| value.as_str())
                        .and_then(parse_optional_timestamp)
                })
        })
}

fn parse_optional_timestamp(value: &str) -> Option<DateTime<Utc>> {
    parse_timestamp(value, "status change timestamp").ok()
}

fn done_item_has_issue_action(item: &str) -> bool {
    item.split(|character: char| !character.is_ascii_alphabetic())
        .filter(|token| !token.is_empty())
        .any(|token| {
            matches!(
                token.to_ascii_lowercase().as_str(),
                "closed" | "processed" | "resolved" | "merged"
            )
        })
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

fn extract_pr_references(item: &str) -> Vec<u64> {
    let mut prs = Vec::new();
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

        if end > start && issue_reference_looks_like_pr(item, index) {
            if let Ok(pr) = item[start..end].parse::<u64>() {
                prs.push(pr);
            }
        }

        index = if end > start { end } else { index + 1 };
    }

    prs
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

fn derive_self_modifications(
    repo_root: &Path,
    cycle: u64,
) -> Result<Vec<SelfModification>, String> {
    let start_commit = find_cycle_start_commit(repo_root, cycle)?;
    let output = ProcessCommand::new("git")
        .arg("diff")
        .arg("--name-only")
        .arg(format!("{start_commit}..HEAD"))
        .arg("--")
        .args(INFRASTRUCTURE_ROOTS)
        .args(INFRASTRUCTURE_FILES)
        .current_dir(repo_root)
        .output()
        .map_err(|error| {
            format!(
                "failed to run git diff for cycle {} in {}: {}",
                cycle,
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

fn cycle_history_window(
    repo_root: &Path,
    cycle: u64,
    commits: &[GitHistoryEntry],
) -> Result<(DateTime<Utc>, Option<DateTime<Utc>>), String> {
    let start = find_cycle_start_timestamp(repo_root, cycle, commits)?;
    let end = find_explicit_cycle_start_timestamp(cycle + 1, commits);
    Ok((start, end))
}

fn find_cycle_start_timestamp(
    repo_root: &Path,
    cycle: u64,
    commits: &[GitHistoryEntry],
) -> Result<DateTime<Utc>, String> {
    if let Some(timestamp) = find_explicit_cycle_start_timestamp(cycle, commits) {
        return Ok(timestamp);
    }

    let current_cycle = current_cycle_from_state(repo_root)?;
    if cycle != current_cycle {
        return Err(format!(
            "could not determine a cycle start timestamp for cycle {}; ensure history is available",
            cycle
        ));
    }

    let state = load_worklog_state(repo_root, true)?.ok_or_else(|| {
        "docs/state.json is required to resolve current cycle timestamp".to_string()
    })?;
    let timestamp = state
        .cycle_phase
        .phase_entered_at
        .as_deref()
        .ok_or_else(|| {
            "missing docs/state.json cycle_phase.phase_entered_at for current cycle".to_string()
        })?;
    parse_timestamp(timestamp, "docs/state.json cycle_phase.phase_entered_at")
}

fn find_explicit_cycle_start_timestamp(
    cycle: u64,
    commits: &[GitHistoryEntry],
) -> Option<DateTime<Utc>> {
    commits
        .iter()
        .find(|commit| {
            commit.subject.starts_with("state(cycle-start):")
                && extract_cycle_tag(&commit.subject) == Some(cycle)
        })
        .map(|commit| commit.committed_at)
}

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
) -> Result<Vec<CycleReceiptJsonEntry>, String> {
    let cycle = cycle.to_string();
    let output = ProcessCommand::new("bash")
        .arg("tools/cycle-receipts")
        .arg("--cycle")
        .arg(&cycle)
        .arg("--json")
        .current_dir(repo_root)
        .output()
        .map_err(|error| {
            format!(
                "failed to run bash tools/cycle-receipts --cycle {} --json in {}: {}",
                cycle,
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

#[cfg(test)]
fn parse_cycle_receipts_output(json: &str) -> Result<Vec<CommitReceipt>, String> {
    let entries = parse_cycle_receipt_entries_output(json)?;
    cycle_receipt_entries_to_receipts(&entries)
}

fn cycle_receipt_entries_to_receipts(
    entries: &[CycleReceiptJsonEntry],
) -> Result<Vec<CommitReceipt>, String> {
    let receipts = entries
        .iter()
        .map(|entry| format!("{}:{}", entry.tool.trim(), entry.receipt.trim()))
        .collect::<Vec<_>>();
    parse_receipts(&receipts)
}

#[cfg(test)]
fn derive_prs_from_cycle_receipts_output(json: &str) -> Result<Vec<u64>, String> {
    let entries = parse_cycle_receipt_entries_output(json)?;
    Ok(derive_prs_from_cycle_receipt_entries(&entries))
}

fn derive_prs_from_cycle_receipt_entries(entries: &[CycleReceiptJsonEntry]) -> Vec<u64> {
    let mut seen = HashSet::new();
    let mut prs = Vec::new();

    for entry in entries {
        if !entry.tool.eq_ignore_ascii_case("process-merge") {
            continue;
        }

        for pr in extract_pr_references(&entry.commit) {
            if seen.insert(pr) {
                prs.push(pr);
            }
        }
    }

    prs
}

fn merge_receipts(
    auto_receipts: Vec<CommitReceipt>,
    manual_receipts: &[CommitReceipt],
) -> Vec<CommitReceipt> {
    let manual_by_tool = manual_receipts
        .iter()
        .map(|receipt| (receipt.tool.to_ascii_lowercase(), receipt))
        .collect::<HashMap<_, _>>();
    let mut auto_tools = HashSet::new();

    let mut merged = Vec::new();
    for receipt in auto_receipts {
        let tool_key = receipt.tool.to_ascii_lowercase();
        auto_tools.insert(tool_key.clone());
        if let Some(manual) = manual_by_tool.get(&tool_key) {
            merged.push((*manual).clone());
        } else {
            merged.push(receipt);
        }
    }

    for receipt in manual_receipts {
        if !auto_tools.contains(&receipt.tool.to_ascii_lowercase()) {
            merged.push(receipt.clone());
        }
    }

    merged
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

fn worklog_path(repo_root: &Path, now: DateTime<Utc>, title: &str) -> PathBuf {
    let date = now.format("%Y-%m-%d").to_string();
    let time = now.format("%H%M%S").to_string();
    let slug = slugify(title);
    repo_root
        .join("docs")
        .join("worklog")
        .join(date)
        .join(format!("{}-{}.md", time, slug))
}

fn journal_path(repo_root: &Path, now: DateTime<Utc>) -> PathBuf {
    repo_root
        .join("docs")
        .join("journal")
        .join(format!("{}.md", now.format("%Y-%m-%d")))
}

fn find_worklog_relative_path(repo_root: &Path, cycle: u64) -> Result<Option<String>, String> {
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
    candidates
        .into_iter()
        .last()
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
        lines.push("- None.".to_string());
    } else {
        for item in &input.what_was_done {
            lines.push(format!("- {}", convert_references(item)));
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
    lines.push("### Issues processed".to_string());
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
    lines.push("## Current state".to_string());
    lines.push(String::new());
    lines.push(format!(
        "- **In-flight agent sessions**: {}",
        input.current_state.in_flight_sessions
    ));
    lines.push(format!(
        "- **Pipeline status**: {}",
        convert_references(&input.current_state.pipeline_status)
    ));
    lines.push(format!(
        "- **Copilot metrics**: {}",
        convert_references(&input.current_state.copilot_metrics)
    ));
    lines.push(format!(
        "- **Publish gate**: {}",
        convert_references(&input.current_state.publish_gate)
    ));
    lines.push(String::new());
    lines.push("## Next steps".to_string());
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
                let link_display = format!(
                    "[{}]({}/{})",
                    receipt.receipt, PRIMARY_COMMITS_URL, receipt.receipt
                );
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

fn strip_cycle_prefix(title: &str) -> &str {
    let trimmed = title.trim();
    let Some(remainder) = trimmed.strip_prefix("Cycle ") else {
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
    suffix
        .strip_prefix(':')
        .map_or(title, |rest| rest.trim_start())
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
            self_modification: Vec::new(),
            next: Vec::new(),
            pipeline: None,
            copilot_metrics: None,
            publish_gate: None,
            in_flight: None,
            receipt: Vec::new(),
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

    fn write_worklog_fixture(
        repo_root: &Path,
        now: DateTime<Utc>,
        cycle: u64,
        title: &str,
    ) -> PathBuf {
        let path = worklog_path(repo_root, now, title);
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
        fs::write(repo_root.join("docs/state.json"), payload)
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
    fn worklog_path_uses_date_time_and_slug() {
        let repo_root = PathBuf::from("/tmp/example");
        let path = worklog_path(&repo_root, fixed_now(), "From Convention to Enforcement");
        assert_eq!(
            path,
            PathBuf::from(
                "/tmp/example/docs/worklog/2026-03-06/051458-from-convention-to-enforcement.md"
            )
        );
    }

    #[test]
    fn worklog_template_keeps_required_section_order() {
        let input = WorklogInput {
            what_was_done: vec!["Fixed #42".to_string()],
            self_modifications: vec![SelfModification {
                file: "STARTUP_CHECKLIST.md".to_string(),
                description: "Updated per audit #117".to_string(),
            }],
            prs_merged: vec![537],
            prs_reviewed: vec![543],
            issues_processed: vec!["Closed #546".to_string()],
            current_state: CurrentState {
                in_flight_sessions: 2,
                pipeline_status: "5/5 phases pass".to_string(),
                copilot_metrics: "64 dispatches".to_string(),
                publish_gate: "Source diverged".to_string(),
            },
            next_steps: vec!["Review PR #543".to_string()],
            receipts: Vec::new(),
            receipt_note: None,
        };
        let rendered = render_worklog(154, fixed_now(), &input);
        let what_done = rendered.find("## What was done").unwrap();
        let self_mods = rendered.find("## Self-modifications").unwrap();
        let current = rendered.find("## Current state").unwrap();
        let next = rendered.find("## Next steps").unwrap();
        assert!(what_done < self_mods);
        assert!(self_mods < current);
        assert!(current < next);
        assert!(rendered.contains("[#42](https://github.com/EvaLok/schema-org-json-ld/issues/42)"));
        assert!(rendered.contains(
            "[audit #117](https://github.com/EvaLok/schema-org-json-ld-audit/issues/117)"
        ));
    }

    #[test]
    fn worklog_template_renders_plain_self_modification_when_description_empty() {
        let input = WorklogInput {
            what_was_done: Vec::new(),
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
                copilot_metrics: NOT_PROVIDED.to_string(),
                publish_gate: NOT_PROVIDED.to_string(),
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
    fn derive_issue_processed_from_done_extracts_issue_references() {
        let issues = derive_issue_processed_from_done(&[
            "Closed EvaLok/schema-org-json-ld#1042".to_string(),
            "Processed audit #198 and resolved #199".to_string(),
            "Merged PR #200".to_string(),
            "Checked #42".to_string(),
            "Resolved #199 again".to_string(),
        ]);

        assert_eq!(issues, vec!["#1042", "#198", "#199"]);
    }

    #[test]
    fn parse_self_modifications_reject_empty_descriptions() {
        let error = parse_self_modifications(&["   ".to_string()]).unwrap_err();
        assert!(error.contains("self-modification description cannot be empty"));
    }

    #[test]
    fn parse_infrastructure_self_modifications_filters_supported_paths() {
        let modifications = parse_infrastructure_self_modifications(
            "tools/rust/crates/write-entry/src/main.rs\nREADME.md\nSTARTUP_CHECKLIST.md\n.claude/skills/rust-tooling/SKILL.md\nAGENTS-ts.md\n",
        );

        assert_eq!(modifications.len(), 4);
        assert_eq!(
            modifications[0].file,
            "tools/rust/crates/write-entry/src/main.rs"
        );
        assert_eq!(modifications[0].description, "modified");
        assert_eq!(modifications[1].file, "STARTUP_CHECKLIST.md");
        assert_eq!(
            modifications[2].file,
            ".claude/skills/rust-tooling/SKILL.md"
        );
        assert_eq!(modifications[3].file, "AGENTS-ts.md");
    }

    #[test]
    fn parse_cycle_receipts_output_supports_current_and_legacy_json_fields() {
        let receipts = parse_cycle_receipts_output(
            r#"[
                {"step":"cycle-start","receipt":"abc1234","commit":"start"},
                {"tool":"process-merge","hash":"def5678","message":"merge"}
            ]"#,
        )
        .unwrap();

        assert_eq!(receipts.len(), 2);
        assert_eq!(receipts[0].tool, "cycle-start");
        assert_eq!(receipts[0].receipt, "abc1234");
        assert_eq!(receipts[1].tool, "process-merge");
        assert_eq!(receipts[1].receipt, "def5678");
    }

    #[test]
    fn derive_prs_from_cycle_receipts_output_uses_process_merge_entries() {
        let prs = derive_prs_from_cycle_receipts_output(
            r#"[
                {"step":"cycle-start","receipt":"abc1234","commit":"state(cycle-start): begin cycle 154 [cycle 154]"},
                {"step":"process-merge","receipt":"def5678","commit":"state(process-merge): PR #537, PR #543 merged [cycle 154]"},
                {"tool":"process-merge","hash":"fedcba9","message":"state(process-merge): PRs EvaLok/schema-org-json-ld#1199, EvaLok/schema-org-json-ld#1197 merged [cycle 251]"},
                {"step":"process-merge","receipt":"7654321","commit":"state(process-merge): PRs EvaLok/schema-org-json-ld#100 merged [cycle 50]"}
            ]"#,
        )
        .unwrap();

        assert_eq!(prs, vec![537, 543, 1199, 1197, 100]);
    }

    #[test]
    fn issue_reference_looks_like_pr_accepts_singular_and_plural_tokens() {
        for subject in [
            "state(process-merge): PR EvaLok/schema-org-json-ld#537 merged [cycle 154]",
            "state(process-merge): pr EvaLok/schema-org-json-ld#537 merged [cycle 154]",
            "state(process-merge): PRs EvaLok/schema-org-json-ld#537 merged [cycle 154]",
            "state(process-merge): prs EvaLok/schema-org-json-ld#537 merged [cycle 154]",
        ] {
            let hash_index = subject.find('#').unwrap();
            assert!(issue_reference_looks_like_pr(subject, hash_index));
        }
    }

    #[test]
    fn issue_reference_looks_like_pr_rejects_issue_tokens() {
        for subject in [
            "state(cycle-start): issue EvaLok/schema-org-json-ld#537 tracked [cycle 154]",
            "state(cycle-start): issues EvaLok/schema-org-json-ld#537, EvaLok/schema-org-json-ld#538 tracked [cycle 154]",
        ] {
            for (hash_index, character) in subject.char_indices() {
                if character == '#' {
                    assert!(!issue_reference_looks_like_pr(subject, hash_index));
                }
            }
        }
    }

    #[test]
    fn parse_cycle_receipts_output_rejects_invalid_json_shape() {
        let error = parse_cycle_receipts_output(r#"{"step":"cycle-start","receipt":"abc1234"}"#)
            .unwrap_err();

        assert!(error.contains("invalid cycle-receipts JSON output"));
    }

    #[test]
    fn parse_git_history_line_rejects_non_full_sha_values() {
        let error =
            parse_git_history_line("abc1234\t2026-03-06T01:05:00Z\tstate(cycle-start): start")
                .unwrap_err();

        assert!(error.contains("git sha must be 40 characters"));
    }

    #[test]
    fn worklog_reads_json_from_input_file() {
        let repo_root = TempRepoDir::new("worklog-input-file");
        init_git_repo(&repo_root.path);
        let receipt = create_git_commit(&repo_root.path, "notes/input-file.txt", "input\n");
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[{{"step":"manual","receipt":"{receipt}","commit":"Add notes/input-file.txt"}}]"#
            ),
        );
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154}
            }"#,
        );
        let payload_path = repo_root.path.join("worklog.json");
        fs::write(
            &payload_path,
            r#"{
                "what_was_done":["Merged PR #123"],
                "self_modifications":[],
                "prs_merged":[123],
                "prs_reviewed":[],
                "issues_processed":[546, "Closed #924 (cycle review)"],
                "current_state":{
                    "in_flight_sessions":1,
                    "pipeline_status":"PASS (6/6)",
                    "copilot_metrics":"steady",
                    "publish_gate":"clear"
                },
                "next_steps":["Review PR #124"]
            }"#,
        )
        .unwrap();
        let mut args = worklog_args("Input file");
        args.cycle = None;
        args.input_file = Some(payload_path);

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(
            content.contains("[PR #123](https://github.com/EvaLok/schema-org-json-ld/issues/123)")
        );
        assert!(content.contains("[#546](https://github.com/EvaLok/schema-org-json-ld/issues/546)"));
        assert!(content.contains(
            "Closed [#924](https://github.com/EvaLok/schema-org-json-ld/issues/924) (cycle review)"
        ));
        assert!(content.contains(
            "1. Review [PR #124](https://github.com/EvaLok/schema-org-json-ld/issues/124)"
        ));
    }

    #[test]
    fn worklog_cycle_mode_replaces_input_file_receipts_with_canonical_output() {
        let repo_root = TempRepoDir::new("worklog-input-file-cycle-receipts");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154 [cycle 154]",
        );
        let canonical_receipt = create_git_commit_with_message(
            &repo_root.path,
            "tools/rust/crates/write-entry/src/main.rs",
            "changed\n",
            "state(process-merge): canonical receipt [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154 [cycle 154]"}},
                    {{"step":"process-merge","receipt":"{canonical_receipt}","commit":"state(process-merge): canonical receipt [cycle 154]"}}
                ]"#
            ),
        );
        let payload_path = repo_root.path.join("worklog-cycle.json");
        fs::write(
            &payload_path,
            format!(
                r#"{{
                    "what_was_done":["Checked #42"],
                    "self_modifications":[],
                    "prs_merged":[],
                    "prs_reviewed":[],
                    "issues_processed":[],
                    "current_state":{{
                        "in_flight_sessions":0,
                        "pipeline_status":"PASS (6/6)",
                        "copilot_metrics":"steady",
                        "publish_gate":"open"
                    }},
                    "next_steps":[],
                    "receipts":[{{"tool":"manual","receipt":"deadbee"}}]
                }}"#
            ),
        )
        .unwrap();
        let mut args = worklog_args("Input file canonical receipts");
        args.input_file = Some(payload_path);

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains(&format!(
            "| cycle-start | {} | [{}](https://github.com/EvaLok/schema-org-json-ld/commit/{}) |",
            start_receipt, start_receipt, start_receipt
        )));
        assert!(content.contains(&format!(
            "| process-merge | {} | [{}](https://github.com/EvaLok/schema-org-json-ld/commit/{}) |",
            canonical_receipt, canonical_receipt, canonical_receipt
        )));
        assert!(!content.contains("| manual | deadbee |"));
    }

    #[test]
    fn find_worklog_relative_path_matches_cycle_and_returns_none_when_missing() {
        let repo_root = TempRepoDir::new("find-worklog");
        let first = write_worklog_fixture(&repo_root.path, fixed_now(), 154, "Cycle one");
        let second = write_worklog_fixture(
            &repo_root.path,
            fixed_now_on("2026-03-07"),
            155,
            "Cycle two",
        );

        let found = find_worklog_relative_path(&repo_root.path, 155).unwrap();
        assert_eq!(
            found,
            Some("../worklog/2026-03-07/051458-cycle-two.md".to_string())
        );
        assert!(first.exists());
        assert!(second.exists());
        assert_eq!(
            find_worklog_relative_path(&repo_root.path, 999).unwrap(),
            None
        );
    }

    #[test]
    fn worklog_inline_flags_render_receipts_table() {
        let repo_root = TempRepoDir::new("worklog-inline-flags");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154}
            }"#,
        );
        let first_receipt = create_git_commit(&repo_root.path, "first.txt", "first");
        let second_receipt = create_git_commit(&repo_root.path, "second.txt", "second");
        let mut args = worklog_args("Inline flags");
        args.cycle = None;
        args.done = vec!["Merged PR #123".to_string()];
        args.pr_merged = vec![123, 456];
        args.pr_reviewed = vec![789];
        args.issue_processed =
            vec!["Closed EvaLok/schema-org-json-ld#924 (cycle review)".to_string()];
        args.self_modification = vec!["Updated AGENTS.md".to_string()];
        args.next = vec!["Review PR #789".to_string()];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.copilot_metrics = Some("45 dispatched".to_string());
        args.publish_gate = Some("open".to_string());
        args.in_flight = Some(1);
        args.receipt = vec![
            format!("cycle-start:{first_receipt}"),
            format!("process-merge:{second_receipt}"),
        ];

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains(
            "- Merged [PR #123](https://github.com/EvaLok/schema-org-json-ld/issues/123)"
        ));
        assert!(content
            .contains("- [PR #789](https://github.com/EvaLok/schema-org-json-ld/issues/789)"));
        assert!(content.contains("- Closed EvaLok/schema-org-json-ld#924 (cycle review)"));
        assert!(content.contains("- Updated AGENTS.md"));
        assert!(!content.contains("### PRs reviewed\n\n- None."));
        assert!(!content.contains("### Issues processed\n\n- None."));
        assert!(!content.contains("## Self-modifications\n\n- None."));
        assert!(content.contains("- **Pipeline status**: PASS (6/6)"));
        assert!(content.contains("- **Copilot metrics**: 45 dispatched"));
        assert!(content.contains("- **Publish gate**: open"));
        assert!(content.contains("## Commit receipts"));
        assert!(content.contains(&format!(
            "| cycle-start | {} | [{}](https://github.com/EvaLok/schema-org-json-ld/commit/{}) |",
            first_receipt, first_receipt, first_receipt
        )));
        assert!(content.contains(&format!(
            "| process-merge | {} | [{}](https://github.com/EvaLok/schema-org-json-ld/commit/{}) |",
            second_receipt, second_receipt, second_receipt
        )));
    }

    #[test]
    fn unresolved_receipt_sha_is_rejected() {
        let repo_root = TempRepoDir::new("worklog-unresolved-receipt");
        let mut receipts = parse_receipts(&["cycle-start:deadbee".to_string()]).unwrap();

        let error = validate_receipt_shas(&mut receipts, &repo_root.path).unwrap_err();
        assert_eq!(error, "unresolvable receipt SHA for cycle-start: deadbee");
    }

    #[test]
    fn generated_markdown_sha_validation_accepts_resolvable_hashes() {
        let repo_root = TempRepoDir::new("generated-markdown-valid-sha");
        init_git_repo(&repo_root.path);
        let valid_sha = create_git_commit(&repo_root.path, "notes/valid.txt", "valid\n");

        let warnings = validate_generated_markdown_shas(
            "worklog",
            &format!("Resolved receipt `{valid_sha}` appears in the entry."),
            &repo_root.path,
        );

        assert!(warnings.unwrap().is_empty());
    }

    #[test]
    fn generated_markdown_sha_validation_warns_on_unresolved_hashes() {
        let repo_root = TempRepoDir::new("generated-markdown-invalid-sha");
        init_git_repo(&repo_root.path);
        let valid_sha = create_git_commit(&repo_root.path, "notes/valid.txt", "valid\n");

        let warnings = validate_generated_markdown_shas(
            "journal",
            &format!("Valid `{valid_sha}` and phantom `deadbee0` receipts are noted."),
            &repo_root.path,
        )
        .unwrap();

        assert_eq!(
            warnings,
            vec!["WARNING: generated journal references unresolved commit SHA: deadbee0"]
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
        assert!(content.contains("- **Pipeline status**: Not provided."));
        assert!(content.contains("- **In-flight agent sessions**: 3"));
        assert!(content.contains(
            "- **Copilot metrics**: 45 dispatches, 42 PRs produced, 40 merged, 88.9% PR merge rate"
        ));
        assert!(content.contains("- **Publish gate**: published"));
    }

    #[test]
    fn worklog_auto_derives_self_modifications_receipts_and_issues_processed() {
        let repo_root = TempRepoDir::new("worklog-auto-derives");
        init_git_repo(&repo_root.path);
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
        args.pipeline = Some("PASS (6/6)".to_string());
        args.copilot_metrics = Some("steady".to_string());
        args.publish_gate = Some("open".to_string());
        args.in_flight = Some(0);

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("### Issues processed\n\n- [#1042]("));
        assert!(!content.contains("### Issues processed\n\n- None."));
        assert!(content.contains("- **`tools/rust/crates/write-entry/src/main.rs`**: modified"));
        assert!(content.contains("- **`AGENTS.md`**: modified"));
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
    fn worklog_auto_derives_pr_sections_from_process_merge_receipts() {
        let repo_root = TempRepoDir::new("worklog-auto-derives-prs");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154}
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
        args.pipeline = Some("PASS (6/6)".to_string());
        args.copilot_metrics = Some("steady".to_string());
        args.publish_gate = Some("open".to_string());
        args.in_flight = Some(0);

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("### PRs merged"));
        assert!(
            content.contains("[PR #237](https://github.com/EvaLok/schema-org-json-ld/issues/237)")
        );
        assert!(
            content.contains("[PR #240](https://github.com/EvaLok/schema-org-json-ld/issues/240)")
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
        args.copilot_metrics = Some("steady".to_string());
        args.publish_gate = Some("open".to_string());
        args.in_flight = Some(0);

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
        args.copilot_metrics = Some("steady".to_string());
        args.publish_gate = Some("open".to_string());
        args.in_flight = Some(0);

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
        args.copilot_metrics = Some("steady".to_string());
        args.publish_gate = Some("open".to_string());
        args.in_flight = Some(0);

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
                "last_cycle": {"number": 154}
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
        args.pipeline = Some("PASS (6/6)".to_string());
        args.copilot_metrics = Some("steady".to_string());
        args.publish_gate = Some("open".to_string());
        args.in_flight = Some(0);

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let cycle = resolve_cycle(args.cycle, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, cycle, &mut input).unwrap();

        assert!(warnings.is_empty());
        assert_eq!(input.issues_processed, vec!["#1042"]);
        assert_eq!(input.self_modifications.len(), 2);
        assert!(input.self_modifications.iter().any(|item| item.file
            == "tools/rust/crates/write-entry/src/main.rs"
            && item.description == "modified"));
        assert!(input
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
    fn worklog_auto_derivation_falls_back_to_manual_receipts_when_cycle_receipts_command_fails() {
        let repo_root = TempRepoDir::new("worklog-auto-derivation-fallback");
        init_git_repo(&repo_root.path);
        let manual_receipt = create_git_commit(&repo_root.path, "notes/manual.txt", "manual\n");
        let mut args = worklog_args("Fallback");
        args.done = vec!["Closed #42".to_string()];
        args.receipt = vec![format!("manual:{manual_receipt}")];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.copilot_metrics = Some("steady".to_string());
        args.publish_gate = Some("open".to_string());
        args.in_flight = Some(0);

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("> Note: Automatic receipt collection via `tools/cycle-receipts` failed; using provided manual receipts instead."));
        assert!(content.contains(&format!(
            "| manual | {} | [{}](https://github.com/EvaLok/schema-org-json-ld/commit/{}) |",
            manual_receipt, manual_receipt, manual_receipt
        )));
    }

    #[test]
    fn worklog_cycle_receipts_must_not_be_empty() {
        let repo_root = TempRepoDir::new("worklog-empty-cycle-receipts");
        init_git_repo(&repo_root.path);
        write_cycle_receipts_script(&repo_root.path, "[]");
        let mut args = worklog_args("Empty receipts");
        args.done = vec!["Closed #42".to_string()];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.copilot_metrics = Some("steady".to_string());
        args.publish_gate = Some("open".to_string());
        args.in_flight = Some(0);

        let error = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap_err();

        assert!(error.contains("cycle-receipts returned no receipts for cycle 154"));
    }

    #[test]
    fn worklog_cycle_mode_merges_manual_receipts_with_canonical_output() {
        let repo_root = TempRepoDir::new("worklog-manual-overrides");
        init_git_repo(&repo_root.path);
        let start_receipt = create_git_commit_with_message(
            &repo_root.path,
            "notes/start.txt",
            "start\n",
            "state(cycle-start): begin cycle 154, issue #1 [cycle 154]",
        );
        let manual_only_receipt =
            create_git_commit(&repo_root.path, "notes/manual.txt", "manual\n");
        let manual_override_receipt = create_git_commit(
            &repo_root.path,
            "notes/manual-override.txt",
            "manual override\n",
        );
        let canonical_receipt = create_git_commit_with_message(
            &repo_root.path,
            "tools/rust/crates/write-entry/src/main.rs",
            "changed\n",
            "state(process-merge): canonical receipt [cycle 154]",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{start_receipt}","commit":"state(cycle-start): begin cycle 154, issue #1 [cycle 154]"}},
                    {{"step":"process-merge","receipt":"{canonical_receipt}","commit":"state(process-merge): canonical receipt [cycle 154]"}}
                ]"#
            ),
        );

        let mut args = worklog_args("Manual overrides");
        args.done = vec!["Closed EvaLok/schema-org-json-ld#1042".to_string()];
        args.issue_processed = vec!["Closed #999".to_string()];
        args.self_modification = vec!["AGENTS.md: manual override".to_string()];
        args.receipt = vec![
            format!("manual:{manual_only_receipt}"),
            format!("process-merge:{manual_override_receipt}"),
        ];
        args.pipeline = Some("PASS (6/6)".to_string());
        args.copilot_metrics = Some("steady".to_string());
        args.publish_gate = Some("open".to_string());
        args.in_flight = Some(0);

        let mut input = resolve_worklog_input(&args, &repo_root.path).unwrap();
        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();

        assert!(warnings.is_empty());
        assert_eq!(input.receipts.len(), 3);
        assert_eq!(input.receipts[0].tool, "cycle-start");
        assert_eq!(input.receipts[0].receipt, start_receipt);
        assert_eq!(input.receipts[1].tool, "process-merge");
        assert_eq!(input.receipts[1].receipt, manual_override_receipt);
        assert_eq!(input.receipts[2].tool, "manual");
        assert_eq!(input.receipts[2].receipt, manual_only_receipt);
        assert_eq!(input.issues_processed, vec!["Closed #999", "#1042"]);

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content
            .contains("- Closed [#999](https://github.com/EvaLok/schema-org-json-ld/issues/999)"));
        assert!(
            content.contains("- [#1042](https://github.com/EvaLok/schema-org-json-ld/issues/1042)")
        );
        assert!(content.contains("- **`AGENTS.md`**: manual override"));
        assert!(!content.contains(": modified"));
        assert!(content.contains(&format!(
            "| cycle-start | {} | [{}](https://github.com/EvaLok/schema-org-json-ld/commit/{}) |",
            start_receipt, start_receipt, start_receipt
        )));
        assert!(content.contains(&format!(
            "| process-merge | {} | [{}](https://github.com/EvaLok/schema-org-json-ld/commit/{}) |",
            manual_override_receipt, manual_override_receipt, manual_override_receipt
        )));
        assert!(content.contains(&format!(
            "| manual | {} | [{}](https://github.com/EvaLok/schema-org-json-ld/commit/{}) |",
            manual_only_receipt, manual_only_receipt, manual_only_receipt
        )));
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

        let mut args = worklog_args("State-derived issues");
        args.pipeline = Some("PASS (6/6)".to_string());
        args.copilot_metrics = Some("steady".to_string());
        args.publish_gate = Some("open".to_string());
        args.in_flight = Some(0);

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();

        assert!(content.contains("### Issues processed\n\n- [#42]("));
        assert!(!content.contains("[#41]("));
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

        let args = worklog_args("Cycle only");
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
    fn worklog_auto_derivation_warns_when_cycle_phase_timestamp_is_unavailable() {
        let repo_root = TempRepoDir::new("worklog-missing-cycle-phase-timestamp");
        init_git_repo(&repo_root.path);
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {
                    "number": 154
                }
            }"#,
        );
        let receipt = create_git_commit_at(
            &repo_root.path,
            "notes/first.txt",
            "first\n",
            "docs: first commit [cycle 154]",
            "2026-03-06T01:05:00Z",
        );
        write_cycle_receipts_script(
            &repo_root.path,
            &format!(
                r#"[
                    {{"step":"cycle-start","receipt":"{receipt}","commit":"docs: first commit [cycle 154]"}}
                ]"#
            ),
        );
        let args = worklog_args("Fallback warning");
        let mut input = WorklogInput {
            what_was_done: vec!["Closed #42".to_string()],
            self_modifications: Vec::new(),
            prs_merged: Vec::new(),
            prs_reviewed: Vec::new(),
            issues_processed: Vec::new(),
            current_state: CurrentState {
                in_flight_sessions: 0,
                pipeline_status: "PASS (6/6)".to_string(),
                copilot_metrics: "steady".to_string(),
                publish_gate: "open".to_string(),
            },
            next_steps: Vec::new(),
            receipts: Vec::new(),
            receipt_note: None,
        };

        let warnings =
            apply_worklog_auto_derivations(&args, &repo_root.path, 154, &mut input).unwrap();

        assert!(input.self_modifications.is_empty());
        assert_eq!(input.issues_processed, vec!["#42"]);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("failed to auto-derive self-modifications for cycle 154"));
        assert!(warnings[0].contains("cycle_phase.phase_entered_at"));
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
        args.copilot_metrics = Some("custom metrics".to_string());
        args.publish_gate = Some("pre-publish".to_string());
        args.in_flight = Some(1);

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("- **Pipeline status**: PASS (6/6)"));
        assert!(content.contains("- **In-flight agent sessions**: 1"));
        assert!(content.contains("- **Copilot metrics**: custom metrics"));
        assert!(content.contains("- **Publish gate**: pre-publish"));
        assert!(!content.contains("45 dispatches, 42 PRs produced, 40 merged, 88.9% PR merge rate"));
        assert!(!content.contains("- **Publish gate**: published"));
    }

    #[test]
    fn worklog_inline_flags_fail_closed_when_state_status_is_unavailable() {
        let repo_root = TempRepoDir::new("worklog-status-missing");
        let mut args = worklog_args("Missing status");
        args.done = vec!["Merged PR #123".to_string()];

        let error = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap_err();
        assert!(error.contains("failed to read"));
        assert!(error.contains("docs/state.json"));
    }

    #[test]
    fn worklog_inline_flags_reject_placeholder_when_state_has_real_status() {
        let repo_root = TempRepoDir::new("worklog-placeholder-rejected");
        write_state_file(
            &repo_root.path,
            r#"{
                "last_cycle": {"number": 154},
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

        let mut args = worklog_args("Placeholder rejected");
        args.done = vec!["Merged PR #123".to_string()];
        args.copilot_metrics = Some(NOT_PROVIDED.to_string());

        let error = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap_err();
        assert!(error.contains("copilot metrics"));
        assert!(error.contains(NOT_PROVIDED));
    }

    #[test]
    fn invalid_receipt_flag_is_rejected() {
        let repo_root = TempRepoDir::new("invalid-receipt");
        let mut args = worklog_args("Invalid receipt");
        args.pipeline = Some("PASS (6/6)".to_string());
        args.copilot_metrics = Some("45 dispatches".to_string());
        args.publish_gate = Some("published".to_string());
        args.in_flight = Some(0);
        args.receipt = vec!["cycle-start:not-a-sha".to_string()];

        let error = resolve_worklog_input(&args, &repo_root.path).unwrap_err();
        assert!(error.contains("invalid receipt"));
    }

    #[test]
    fn receipt_sha_length_validation_accepts_seven_and_rejects_shorter() {
        let receipts = parse_receipts(&["cycle-start:abc1234".to_string()]).unwrap();
        assert_eq!(receipts.len(), 1);

        let error = parse_receipts(&["cycle-start:abc123".to_string()]).unwrap_err();
        assert!(error.contains("at least 7 hexadecimal characters"));
    }

    #[test]
    fn journal_create_and_append_use_separator() {
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
        execute_journal(&file_args, &repo_root.path, now).unwrap();

        let path = journal_path(&repo_root.path, now);
        let content = fs::read_to_string(path).unwrap();
        assert!(content.starts_with("# Journal — 2026-03-06"));
        assert!(
            content.contains("\n---\n\n## 2026-03-06 — Cycle 154: From convention to enforcement")
        );
        assert!(content.contains(
            "Worklog: [cycle 154](../worklog/2026-03-06/051458-from-convention-to-enforcement.md)"
        ));
        assert_eq!(
            content
                .matches("\n## 2026-03-06 — Cycle 154: From convention to enforcement\n")
                .count(),
            2
        );
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
        assert!(content
            .contains("Worklog: [cycle 154](../worklog/2026-03-06/051458-cycle-reflections.md)"));
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
            Command::Journal(_) | Command::PatchPipeline(_) => panic!("expected worklog command"),
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
            Command::Journal(_) | Command::PatchPipeline(_) => panic!("expected worklog command"),
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
            "--next",
            "Review PR #124",
            "--pipeline",
            "PASS (6/6)",
            "--copilot-metrics",
            "45 dispatched",
            "--publish-gate",
            "open",
            "--in-flight",
            "1",
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
                assert!(args.self_modification.is_empty());
                assert_eq!(args.next, vec!["Review PR #124".to_string()]);
                assert_eq!(args.pipeline.as_deref(), Some("PASS (6/6)"));
                assert_eq!(args.copilot_metrics.as_deref(), Some("45 dispatched"));
                assert_eq!(args.publish_gate.as_deref(), Some("open"));
                assert_eq!(args.in_flight, Some(1));
                assert_eq!(args.receipt, vec!["cycle-start:abc1234".to_string()]);
            }
            Command::Journal(_) | Command::PatchPipeline(_) => panic!("expected worklog command"),
        }
    }

    #[test]
    fn cli_parses_new_worklog_tracking_flags() {
        let cli = Cli::try_parse_from([
            "write-entry",
            "worklog",
            "--title",
            "test",
            "--done",
            "did stuff",
            "--pr-reviewed",
            "123",
            "--issue-processed",
            "Closed EvaLok/schema-org-json-ld#924 (cycle review)",
            "--self-modification",
            "Updated AGENTS.md",
        ])
        .unwrap();

        match cli.command {
            Command::Worklog(args) => {
                assert_eq!(args.pr_reviewed, vec![123]);
                assert_eq!(
                    args.issue_processed,
                    vec!["Closed EvaLok/schema-org-json-ld#924 (cycle review)".to_string()]
                );
                assert_eq!(
                    args.self_modification,
                    vec!["Updated AGENTS.md".to_string()]
                );
            }
            Command::Journal(_) | Command::PatchPipeline(_) => panic!("expected worklog command"),
        }
    }

    #[test]
    fn cli_parses_patch_pipeline_arguments() {
        let cli = Cli::try_parse_from([
            "write-entry",
            "patch-pipeline",
            "--worklog",
            "docs/worklog/test.md",
            "--status",
            "PASS (9/9)",
        ])
        .unwrap();

        match cli.command {
            Command::PatchPipeline(args) => {
                assert_eq!(args.worklog, PathBuf::from("docs/worklog/test.md"));
                assert_eq!(args.status, "PASS (9/9)");
            }
            Command::Worklog(_) | Command::Journal(_) => panic!("expected patch-pipeline command"),
        }
    }

    #[test]
    fn cli_parses_journal_input_file_and_inline_flags() {
        let cli = Cli::try_parse_from([
            "write-entry",
            "journal",
            "--title",
            "test",
            "--input-file",
            "/tmp/journal.json",
            "--section",
            "Decision::Defer #829",
            "--commitment",
            "Dispatch #830 next cycle",
            "--previous-commitment-status",
            "followed",
            "--previous-commitment-detail",
            "Done.",
        ])
        .unwrap();

        match cli.command {
            Command::Journal(args) => {
                assert_eq!(args.input_file, Some(PathBuf::from("/tmp/journal.json")));
                assert_eq!(args.section, vec!["Decision::Defer #829".to_string()]);
                assert_eq!(
                    args.commitment,
                    vec!["Dispatch #830 next cycle".to_string()]
                );
                assert_eq!(args.previous_commitment_status.as_deref(), Some("followed"));
                assert_eq!(args.previous_commitment_detail.as_deref(), Some("Done."));
            }
            Command::Worklog(_) | Command::PatchPipeline(_) => panic!("expected journal command"),
        }
    }

    #[test]
    fn patch_pipeline_replaces_status_and_preserves_other_content() {
        let repo_root = TempRepoDir::new("patch-pipeline-success");
        let worklog_path = repo_root.path.join("docs/worklog/test.md");
        fs::create_dir_all(worklog_path.parent().unwrap()).unwrap();
        let original = "\
# Cycle 154 — 2026-03-06 05:14 UTC

## Current state

- **In-flight agent sessions**: 1
- **Pipeline status**: FAIL (1/9)
- **Copilot metrics**: stable
- **Publish gate**: open

## Next steps

1. None.
";
        fs::write(&worklog_path, original).unwrap();

        let result = execute_patch_pipeline(
            &PatchPipelineArgs {
                worklog: PathBuf::from("docs/worklog/test.md"),
                status: "PASS (9/9)".to_string(),
            },
            &repo_root.path,
        )
        .unwrap();

        assert_eq!(result, worklog_path);
        let updated = fs::read_to_string(&worklog_path).unwrap();
        assert!(updated.contains("- **Pipeline status**: PASS (9/9)"));
        assert!(updated.contains("- **In-flight agent sessions**: 1"));
        assert!(updated.contains("- **Copilot metrics**: stable"));
        assert!(updated.contains("## Next steps"));
        assert_eq!(updated.matches("- **Pipeline status**:").count(), 1);
    }

    #[test]
    fn patch_pipeline_returns_error_when_pipeline_status_line_is_missing() {
        let repo_root = TempRepoDir::new("patch-pipeline-missing-pattern");
        let worklog_path = repo_root.path.join("docs/worklog/test.md");
        fs::create_dir_all(worklog_path.parent().unwrap()).unwrap();
        fs::write(
            &worklog_path,
            "# Cycle 154\n\n## Current state\n\n- **Copilot metrics**: stable\n",
        )
        .unwrap();

        let error = execute_patch_pipeline(
            &PatchPipelineArgs {
                worklog: PathBuf::from("docs/worklog/test.md"),
                status: "PASS (9/9)".to_string(),
            },
            &repo_root.path,
        )
        .unwrap_err();

        assert_eq!(
            error,
            format!(
                "failed to patch {}: pipeline status line not found",
                worklog_path.display()
            )
        );
        let expected = "# Cycle 154\n\n## Current state\n\n- **Copilot metrics**: stable\n";
        assert_eq!(fs::read_to_string(&worklog_path).unwrap(), expected);
    }

    #[test]
    fn patch_pipeline_supports_multiline_status() {
        let repo_root = TempRepoDir::new("patch-pipeline-multiline");
        let worklog_path = repo_root.path.join("docs/worklog/test.md");
        fs::create_dir_all(worklog_path.parent().unwrap()).unwrap();
        fs::write(
            &worklog_path,
            "# Cycle 154\n\n## Current state\n\n- **Pipeline status**: FAIL (warnings pending)\n- **Publish gate**: open\n",
        )
        .unwrap();

        execute_patch_pipeline(
            &PatchPipelineArgs {
                worklog: PathBuf::from("docs/worklog/test.md"),
                status: "PASS (2 warnings:\nwarn one\nwarn two)".to_string(),
            },
            &repo_root.path,
        )
        .unwrap();

        let updated = fs::read_to_string(&worklog_path).unwrap();
        assert!(updated.contains("- **Pipeline status**: PASS (2 warnings:\nwarn one\nwarn two)"));
        assert!(updated.contains("- **Publish gate**: open"));
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
