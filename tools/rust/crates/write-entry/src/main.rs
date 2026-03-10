use chrono::{DateTime, NaiveDate, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Deserializer};
use state_schema::{current_cycle_from_state, read_state_value, StateJson};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

const PRIMARY_ISSUES_URL: &str = "https://github.com/EvaLok/schema-org-json-ld/issues";
const QC_ISSUES_URL: &str = "https://github.com/EvaLok/schema-org-json-ld-qc/issues";
const AUDIT_ISSUES_URL: &str = "https://github.com/EvaLok/schema-org-json-ld-audit/issues";
const PRIMARY_COMMITS_URL: &str = "https://github.com/EvaLok/schema-org-json-ld/commit";
const JOURNAL_DESCRIPTION: &str = "Reflective log for the schema-org-json-ld orchestrator.";
const NOT_PROVIDED: &str = "Not provided.";

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

#[derive(Debug, Deserialize)]
struct CommitReceipt {
    tool: String,
    receipt: String,
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
    let input = resolve_worklog_input(args, repo_root)?;
    let path = worklog_path(repo_root, now, &args.title);
    let content = render_worklog(cycle, now, &input);
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
        };
        validate_worklog_state_placeholders(&input, state.as_ref())?;
        return Ok(input);
    }

    let payload = read_stdin()?;
    serde_json::from_str(&payload).map_err(|error| format!("invalid worklog JSON input: {}", error))
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
            })
        })
        .collect()
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
                .map(|relative| relative.to_string_lossy().replace('\\', "/"))
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
    lines.push("### PRs reviewed".to_string());
    lines.push(String::new());
    lines.extend(render_numbered_refs(
        &input.prs_reviewed,
        "PR",
        PRIMARY_ISSUES_URL,
    ));
    lines.push(String::new());
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
        lines.push("| Tool | Receipt | Link |".to_string());
        lines.push("|------|---------|------|".to_string());
        for receipt in &input.receipts {
            lines.push(format!(
                "| {} | {} | [{}]({}/{}) |",
                receipt.tool,
                receipt.receipt,
                receipt.receipt,
                PRIMARY_COMMITS_URL,
                receipt.receipt
            ));
        }
    }
    lines.push(String::new());
    lines.join("\n")
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
        fs::write(repo_root.join("docs/state.json"), payload).expect("failed to write test state.json");
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
    fn parse_self_modifications_reject_empty_descriptions() {
        let error = parse_self_modifications(&["   ".to_string()]).unwrap_err();
        assert!(error.contains("self-modification description cannot be empty"));
    }

    #[test]
    fn worklog_reads_json_from_input_file() {
        let repo_root = TempRepoDir::new("worklog-input-file");
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
            Some("docs/worklog/2026-03-07/051458-cycle-two.md".to_string())
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
        let mut args = worklog_args("Inline flags");
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
            "cycle-start:abc1234".to_string(),
            "process-merge:def5678".to_string(),
        ];

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains(
            "- Merged [PR #123](https://github.com/EvaLok/schema-org-json-ld/issues/123)"
        ));
        assert!(content.contains(
            "- [PR #789](https://github.com/EvaLok/schema-org-json-ld/issues/789)"
        ));
        assert!(content.contains("- Closed EvaLok/schema-org-json-ld#924 (cycle review)"));
        assert!(content.contains("- Updated AGENTS.md"));
        assert!(!content.contains("### PRs reviewed\n\n- None."));
        assert!(!content.contains("### Issues processed\n\n- None."));
        assert!(!content.contains("## Self-modifications\n\n- None."));
        assert!(content.contains("- **Pipeline status**: PASS (6/6)"));
        assert!(content.contains("- **Copilot metrics**: 45 dispatched"));
        assert!(content.contains("- **Publish gate**: open"));
        assert!(content.contains("## Commit receipts"));
        assert!(content.contains("| cycle-start | abc1234 | [abc1234](https://github.com/EvaLok/schema-org-json-ld/commit/abc1234) |"));
        assert!(content.contains("| process-merge | def5678 | [def5678](https://github.com/EvaLok/schema-org-json-ld/commit/def5678) |"));
    }

    #[test]
    fn worklog_inline_flags_auto_populate_status_from_state() {
        let repo_root = TempRepoDir::new("worklog-auto-populate");
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
        args.done = vec!["Merged PR #123".to_string()];

        let path = execute_worklog(&args, &repo_root.path, fixed_now()).unwrap();
        let content = fs::read_to_string(path).unwrap();
        assert!(content.contains("### PRs reviewed\n\n- None."));
        assert!(content.contains("### Issues processed\n\n- None."));
        assert!(content.contains("## Self-modifications\n\n- None."));
        assert!(content.contains("- **Pipeline status**: Not provided."));
        assert!(content.contains("- **In-flight agent sessions**: 3"));
        assert!(
            content.contains(
                "- **Copilot metrics**: 45 dispatches, 42 PRs produced, 40 merged, 88.9% PR merge rate"
            )
        );
        assert!(content.contains("- **Publish gate**: published"));
    }

    #[test]
    fn worklog_inline_flags_prefer_explicit_status_over_state() {
        let repo_root = TempRepoDir::new("worklog-status-override");
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
        assert!(
            !content.contains(
                "45 dispatches, 42 PRs produced, 40 merged, 88.9% PR merge rate"
            )
        );
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
            "Worklog: [cycle 154](docs/worklog/2026-03-06/051458-from-convention-to-enforcement.md)"
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
            .contains("Worklog: [cycle 154](docs/worklog/2026-03-06/051458-cycle-reflections.md)"));
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
            "previous_commitment_status":"followed",
            "previous_commitment_detail":"Done.",
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
            "previous_commitment_status":"followed",
            "previous_commitment_detail":"Done.",
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
            "previous_commitment_status":"followed",
            "previous_commitment_detail":"Done.",
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
            "previous_commitment_status":"followed",
            "previous_commitment_detail":"Done.",
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
            "previous_commitment_status":"followed",
            "previous_commitment_detail":"Done.",
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
            "previous_commitment_status":"followed",
            "previous_commitment_detail":"Done.",
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
            "previous_commitment_status":"followed",
            "previous_commitment_detail":"Done.",
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
            "previous_commitment_status":"followed",
            "previous_commitment_detail":"Done.",
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
                assert_eq!(args.self_modification, vec!["Updated AGENTS.md".to_string()]);
            }
            Command::Journal(_) => panic!("expected worklog command"),
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
            Command::Worklog(_) => panic!("expected journal command"),
        }
    }

    #[test]
    fn worklog_derives_cycle_from_state_when_omitted() {
        let repo_root = TempRepoDir::new("worklog-derived-cycle");
        fs::create_dir_all(repo_root.path.join("docs")).unwrap();
        fs::write(
            repo_root.path.join("docs/state.json"),
            "{\n  \"last_cycle\": {\"number\": 168}\n}\n",
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
            "{\n  \"last_cycle\": {\"number\": 168}\n}\n",
        )
        .unwrap();
        let mut args = journal_args("Derived cycle");
        args.cycle = None;
        let payload = r#"{
			"previous_commitment_status":"followed",
			"previous_commitment_detail":"Done.",
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
}
