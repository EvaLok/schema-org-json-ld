use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};
use state_schema::{
    commit_state_json, current_utc_timestamp, read_state_value, set_value_at_pointer,
    transition_cycle_phase, write_state_value, StateJson,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const QC_REPO: &str = "EvaLok/schema-org-json-ld-qc";
const AUDIT_REPO: &str = "EvaLok/schema-org-json-ld-audit";
const DEFAULT_STALE_THRESHOLD_SECS: u64 = 7_200;
const ORCHESTRATOR_SIGNATURES: [&str; 3] = [
    "[main-orchestrator]",
    "[qc-orchestrator]",
    "[audit-orchestrator]",
];
#[derive(Parser)]
#[command(name = "cycle-start")]
struct Cli {
    /// GitHub issue number for this cycle
    #[arg(long)]
    issue: u64,

    /// Model name for the orchestrator session
    #[arg(long)]
    model: String,

    /// Path to the repository root
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Output startup brief as JSON
    #[arg(long)]
    json: bool,

    /// Recover close_out phases older than this many seconds
    #[arg(long, default_value_t = DEFAULT_STALE_THRESHOLD_SECS)]
    stale_threshold: u64,
}

#[derive(Clone, Debug, Serialize)]
struct PatchUpdate {
    path: String,
    value: Value,
}

#[derive(Clone, Debug, Serialize)]
struct PipelineStatus {
    status: String,
    detail: String,
}

#[derive(Clone, Debug, Serialize)]
struct SimpleIssue {
    number: u64,
    title: String,
}

#[derive(Clone, Debug, Serialize)]
struct SimplePr {
    number: u64,
    title: String,
    is_draft: bool,
    author: String,
}

#[derive(Clone, Debug, Serialize)]
struct EvaComment {
    issue_number: Option<u64>,
    first_line: String,
}

#[derive(Clone, Debug, Serialize)]
struct ReviewSummary {
    cycle: u64,
    score: Option<u8>,
    findings: Option<u32>,
    path: String,
}

#[derive(Clone, Debug, Serialize)]
struct InFlightSummary {
    assigned_issues: Vec<SimpleIssue>,
    open_prs: Vec<SimplePr>,
    sessions: u64,
}

#[derive(Clone, Debug, Serialize)]
struct StartupBrief {
    cycle: u64,
    issue: u64,
    receipt: String,
    eva_directives: Vec<String>,
    input_from_eva: Vec<SimpleIssue>,
    eva_comments_since_last_cycle: Vec<EvaComment>,
    review_agent: Option<ReviewSummary>,
    pipeline: PipelineStatus,
    in_flight: InFlightSummary,
    publish_gate: String,
    qc_outbound: Vec<u64>,
    audit_outbound: Vec<u64>,
    questions_for_eva: Vec<SimpleIssue>,
    warnings: Vec<String>,
}

fn should_resume(phase: Option<&str>) -> bool {
    matches!(phase, Some("work" | "close_out"))
}

fn build_resume_json(cycle: u64, phase: &str) -> Value {
    json!({
        "resume": true,
        "cycle": cycle,
        "phase": phase,
    })
}

fn detect_stale_close_out(
    phase: Option<&str>,
    phase_entered_at: Option<&str>,
    stale_threshold_secs: u64,
    now: DateTime<Utc>,
) -> Result<Option<String>, String> {
    if phase != Some("close_out") {
        return Ok(None);
    }

    let Some(entered_at) = phase_entered_at else {
        return Ok(None);
    };

    let threshold_secs = i64::try_from(stale_threshold_secs)
        .map_err(|_| format!("stale threshold is too large: {}", stale_threshold_secs))?;
    let entered_at = DateTime::parse_from_rfc3339(entered_at)
        .map_err(|error| format!("invalid close_out phase_entered_at timestamp: {}", error))?
        .with_timezone(&Utc);

    if now.signed_duration_since(entered_at).num_seconds() >= threshold_secs {
        Ok(Some(entered_at.format("%Y-%m-%dT%H:%M:%SZ").to_string()))
    } else {
        Ok(None)
    }
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let mut state = read_state_value(&cli.repo_root)?;
    let state_json = read_typed_state_json(&cli.repo_root)?;
    let previous_cycle_issue = state.pointer("/last_cycle/issue").and_then(Value::as_u64);
    let mut warnings = Vec::new();

    // Resume detection for in-progress cycles.
    let current_phase = state_json.cycle_phase.phase.as_deref();

    if let Some(entered_at) = detect_stale_close_out(
        current_phase,
        state_json.cycle_phase.phase_entered_at.as_deref(),
        cli.stale_threshold,
        Utc::now(),
    )? {
        let cycle = state_json.cycle_phase.cycle.unwrap_or(0);
        let warning = format!(
            "Stale close-out detected for cycle {} (entered at {}). Recovering...",
            cycle, entered_at
        );
        warn(&mut warnings, warning);
        recover_stale_close_out(
            &cli.repo_root,
            &mut state,
            cycle,
            previous_cycle_issue,
            &entered_at,
        )?;
    } else if should_resume(current_phase) {
        let current_phase = current_phase.unwrap_or("complete");
        let cycle = state_json.cycle_phase.cycle.unwrap_or(0);

        if cli.json {
            println!(
                "{}",
                serde_json::to_string_pretty(&build_resume_json(cycle, current_phase))
                    .map_err(|error| format!("failed to serialize resume JSON: {}", error))?
            );
        } else {
            println!("Resume: cycle {} phase {}", cycle, current_phase);
            println!("No new cycle created. Resume {} phase.", current_phase);
        }
        return Ok(());
    }

    let previous_timestamp = state
        .pointer("/last_cycle/timestamp")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_string());
    let cycle = derive_cycle_from_state(&state)?;
    let timestamp = current_utc_timestamp();
    let questions_for_eva = gather_questions_for_eva(&mut warnings);
    let open_question_numbers: Vec<u64> =
        questions_for_eva.iter().map(|issue| issue.number).collect();

    let patch = build_state_patch(
        cycle,
        cli.issue,
        previous_cycle_issue,
        &timestamp,
        &open_question_numbers,
    );
    apply_state_patch(&mut state, &patch)?;

    // Set cycle_phase for the new work phase
    transition_cycle_phase(&mut state, cycle, "work")?;

    write_state_value(&cli.repo_root, &state)?;
    let commit_message = format!(
        "state(cycle-start): begin cycle {}, issue #{} [cycle {}]",
        cycle, cli.issue, cycle
    );
    let receipt = commit_state_json(&cli.repo_root, &commit_message)?;

    if let Err(error) = post_opening_comment(cli.issue, cycle, &timestamp, &cli.model) {
        warn(&mut warnings, format!("opening comment failed: {}", error));
    }

    let input_from_eva = gather_input_from_eva(&mut warnings);
    let eva_comments = gather_eva_comments_since(&previous_timestamp, &mut warnings);
    let review_agent =
        gather_review_summary(&cli.repo_root, cycle.saturating_sub(1), &mut warnings);
    let in_flight = gather_in_flight_sessions(&mut warnings);
    let pipeline = gather_pipeline_status(&cli.repo_root, cycle, &mut warnings);
    let qc_outbound = gather_outbound_issue_numbers(QC_REPO, "qc-outbound", &mut warnings);
    let audit_outbound = gather_outbound_issue_numbers(AUDIT_REPO, "audit-outbound", &mut warnings);
    let publish_gate = summarize_publish_gate(&state);
    let eva_directives = load_eva_directives(&state_json)?;

    let brief = StartupBrief {
        cycle,
        issue: cli.issue,
        receipt,
        eva_directives,
        input_from_eva,
        eva_comments_since_last_cycle: eva_comments,
        review_agent,
        pipeline,
        in_flight,
        publish_gate,
        qc_outbound,
        audit_outbound,
        questions_for_eva,
        warnings,
    };

    if cli.json {
        let output = serde_json::to_string_pretty(&brief)
            .map_err(|error| format!("failed to serialize JSON output: {}", error))?;
        println!("{}", output);
    } else {
        println!("{}", format_human_brief(&brief));
    }

    Ok(())
}

fn recover_stale_close_out(
    repo_root: &Path,
    state: &mut Value,
    cycle: u64,
    stale_issue: Option<u64>,
    entered_at: &str,
) -> Result<(), String> {
    recover_stale_close_out_with(
        repo_root,
        state,
        cycle,
        stale_issue,
        entered_at,
        close_stale_cycle_issue,
    )
}

fn recover_stale_close_out_with<F>(
    repo_root: &Path,
    state: &mut Value,
    cycle: u64,
    stale_issue: Option<u64>,
    entered_at: &str,
    close_issue: F,
) -> Result<(), String>
where
    F: FnOnce(u64, u64, &str) -> Result<(), String>,
{
    let stale_issue = stale_issue
        .ok_or_else(|| "missing /last_cycle/issue for stale close-out recovery".to_string())?;
    close_issue(stale_issue, cycle, entered_at)?;

    transition_cycle_phase(state, cycle, "complete")?;
    write_state_value(repo_root, state)?;
    let commit_message = format!(
        "state(cycle-start): recover stale close-out for cycle {} [cycle {}]",
        cycle, cycle
    );
    commit_state_json(repo_root, &commit_message)?;
    Ok(())
}

fn close_stale_cycle_issue(issue: u64, cycle: u64, entered_at: &str) -> Result<(), String> {
    let comment = build_stale_close_out_comment(cycle, entered_at);
    let issue_arg = issue.to_string();
    let output = Command::new("gh")
        .arg("issue")
        .arg("close")
        .arg(issue_arg.as_str())
        .arg("--repo")
        .arg(MAIN_REPO)
        .arg("--comment")
        .arg(comment)
        .output()
        .map_err(|error| format!("failed to execute gh issue close: {}", error))?;

    if output.status.success() {
        return Ok(());
    }

    let stderr = String::from_utf8_lossy(&output.stderr);
    if stderr.to_ascii_lowercase().contains("already closed") {
        return Ok(());
    }

    Err(command_failure_message("gh issue close", &output))
}

fn build_stale_close_out_comment(cycle: u64, entered_at: &str) -> String {
    format!(
        "> **[main-orchestrator]** | Cycle {cycle}\n\nThe orchestrator session terminated during close-out after entering the phase at {entered_at}. Recovery auto-completed the stale close-out so the next cycle can start cleanly."
    )
}

fn read_typed_state_json(repo_root: &Path) -> Result<StateJson, String> {
    let state_path = repo_root.join("docs/state.json");
    let content = fs::read_to_string(&state_path)
        .map_err(|error| format!("failed to read {}: {}", state_path.display(), error))?;
    serde_json::from_str(&content).map_err(|error| {
        format!(
            "failed to parse {} into schema: {}",
            state_path.display(),
            error
        )
    })
}

fn load_eva_directives(state: &StateJson) -> Result<Vec<String>, String> {
    load_eva_directives_with(state, fetch_issue_title)
}

fn load_eva_directives_with<F>(state: &StateJson, mut fetch_title: F) -> Result<Vec<String>, String>
where
    F: FnMut(u64) -> Result<String, String>,
{
    state
        .eva_input_issues
        .remaining_open
        .iter()
        .map(|raw_issue_number| {
            let validated_issue_number = u64::try_from(*raw_issue_number).map_err(|_| {
                format!(
                    "docs/state.json contains invalid /eva_input_issues/remaining_open entry (negative values are not allowed): {}",
                    raw_issue_number
                )
            })?;
            let title = fetch_title(validated_issue_number)?;
            Ok(format!(
                "{}#{} ({})",
                MAIN_REPO, validated_issue_number, title
            ))
        })
        .collect()
}

fn fetch_issue_title(issue_number: u64) -> Result<String, String> {
    let issue_number_arg = issue_number.to_string();
    gh_text(&[
        "issue",
        "view",
        issue_number_arg.as_str(),
        "--repo",
        MAIN_REPO,
        "--json",
        "title",
        "--jq",
        ".title",
    ])
}

fn derive_cycle_from_state(state: &Value) -> Result<u64, String> {
    let last_cycle = state
        .pointer("/last_cycle/number")
        .and_then(Value::as_u64)
        .ok_or_else(|| "missing /last_cycle/number in docs/state.json".to_string())?;
    derive_cycle_number(last_cycle)
}

fn derive_cycle_number(last_cycle_number: u64) -> Result<u64, String> {
    last_cycle_number
        .checked_add(1)
        .ok_or_else(|| "last_cycle.number overflow when deriving current cycle".to_string())
}

fn build_state_patch(
    cycle: u64,
    issue: u64,
    previous_cycle_issue: Option<u64>,
    timestamp: &str,
    open_question_numbers: &[u64],
) -> Vec<PatchUpdate> {
    let mut patch = vec![
        PatchUpdate {
            path: "/last_cycle/number".to_string(),
            value: json!(cycle),
        },
        PatchUpdate {
            path: "/last_cycle/issue".to_string(),
            value: json!(issue),
        },
        PatchUpdate {
            path: "/last_cycle/timestamp".to_string(),
            value: json!(timestamp),
        },
        PatchUpdate {
            path: "/last_eva_comment_check".to_string(),
            value: json!(timestamp),
        },
        PatchUpdate {
            path: "/open_questions_for_eva".to_string(),
            value: json!(open_question_numbers),
        },
        PatchUpdate {
            path: "/field_inventory/fields/last_cycle/last_refreshed".to_string(),
            value: json!(format!("cycle {}", cycle)),
        },
        PatchUpdate {
            path: "/field_inventory/fields/last_eva_comment_check/last_refreshed".to_string(),
            value: json!(format!("cycle {}", cycle)),
        },
        PatchUpdate {
            path: "/field_inventory/fields/open_questions_for_eva/last_refreshed".to_string(),
            value: json!(format!("cycle {}", cycle)),
        },
    ];

    if let Some(previous_cycle_issue) = previous_cycle_issue {
        patch.push(PatchUpdate {
            path: "/previous_cycle_issue".to_string(),
            value: json!(previous_cycle_issue),
        });
    }

    patch.push(PatchUpdate {
        path: "/field_inventory/fields/previous_cycle_issue".to_string(),
        value: json!({
            "cadence": "every cycle",
            "last_refreshed": format!("cycle {}", cycle),
        }),
    });

    patch
}

fn apply_state_patch(state: &mut Value, patch: &[PatchUpdate]) -> Result<(), String> {
    for update in patch {
        set_value_at_pointer(state, &update.path, update.value.clone())?;
    }
    Ok(())
}

fn post_opening_comment(
    issue: u64,
    cycle: u64,
    timestamp: &str,
    model: &str,
) -> Result<(), String> {
    let body = build_opening_comment(cycle, timestamp, model, &github_run_id());

    let output = Command::new("gh")
        .arg("api")
        .arg(format!("repos/{}/issues/{}/comments", MAIN_REPO, issue))
        .arg("-X")
        .arg("POST")
        .arg("-f")
        .arg(format!("body={}", body))
        .output()
        .map_err(|error| format!("failed to execute gh api for opening comment: {}", error))?;

    if !output.status.success() {
        return Err(command_failure_message("gh api", &output));
    }

    Ok(())
}

fn build_opening_comment(cycle: u64, timestamp: &str, model: &str, run_id: &str) -> String {
    format!(
        "> **[main-orchestrator]** | Cycle {cycle}\n\n**Session start**: {timestamp} UTC\n**Model**: {model}\n**Run ID**: {run_id}\n**Cycle**: {cycle}\n\nBeginning startup checklist. Will post updates as work progresses."
    )
}

fn github_run_id() -> String {
    github_run_id_from(std::env::var("GITHUB_RUN_ID").ok().as_deref())
}

fn github_run_id_from(run_id: Option<&str>) -> String {
    run_id
        .filter(|value| !value.trim().is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| "local".to_string())
}

fn gather_pipeline_status(
    repo_root: &Path,
    cycle: u64,
    warnings: &mut Vec<String>,
) -> PipelineStatus {
    let script = repo_root.join("tools/pipeline-check");
    let output = Command::new("bash")
        .arg(script)
        .arg("--repo-root")
        .arg(repo_root)
        .arg("--cycle")
        .arg(cycle.to_string())
        .arg("--json")
        .output();

    let output = match output {
        Ok(output) => output,
        Err(error) => {
            warn(
                warnings,
                format!("pipeline-check invocation failed: {}", error),
            );
            return PipelineStatus {
                status: "unknown".to_string(),
                detail: "pipeline-check unavailable".to_string(),
            };
        }
    };

    if !output.status.success() {
        warn(
            warnings,
            format!(
                "pipeline-check failed: {}",
                command_failure_message("pipeline-check", &output)
            ),
        );
        return PipelineStatus {
            status: "unknown".to_string(),
            detail: "pipeline-check command failed".to_string(),
        };
    }

    let parsed: Value = match serde_json::from_slice(&output.stdout) {
        Ok(value) => value,
        Err(error) => {
            warn(
                warnings,
                format!("pipeline-check returned non-JSON output: {}", error),
            );
            return PipelineStatus {
                status: "unknown".to_string(),
                detail: "pipeline-check output parse failed".to_string(),
            };
        }
    };

    let status = parsed
        .pointer("/overall")
        .and_then(Value::as_str)
        .unwrap_or("unknown")
        .to_string();
    let detail = parsed
        .pointer("/steps")
        .and_then(Value::as_array)
        .map(|steps| format!("{} steps", steps.len()))
        .unwrap_or_else(|| "no step details".to_string());

    PipelineStatus { status, detail }
}

fn gather_input_from_eva(warnings: &mut Vec<String>) -> Vec<SimpleIssue> {
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
        "number,title",
    ]) {
        Ok(value) => parse_simple_issues(&value),
        Err(error) => {
            warn(warnings, format!("input-from-eva fetch failed: {}", error));
            Vec::new()
        }
    }
}

fn gather_questions_for_eva(warnings: &mut Vec<String>) -> Vec<SimpleIssue> {
    match gh_json(&[
        "issue",
        "list",
        "--repo",
        MAIN_REPO,
        "--label",
        "question-for-eva",
        "--state",
        "open",
        "--json",
        "number,title",
    ]) {
        Ok(value) => parse_simple_issues(&value),
        Err(error) => {
            warn(
                warnings,
                format!("question-for-eva fetch failed: {}", error),
            );
            Vec::new()
        }
    }
}

fn gather_eva_comments_since(since: &str, warnings: &mut Vec<String>) -> Vec<EvaComment> {
    let normalized_since = match normalize_since_timestamp(since) {
        Some(value) => value,
        None => {
            warn(
                warnings,
                format!(
                    "invalid last cycle timestamp '{}', falling back to epoch",
                    since
                ),
            );
            "1970-01-01T00:00:00Z".to_string()
        }
    };

    let endpoint = format!(
        "repos/{}/issues/comments?sort=created&direction=desc&since={}&per_page=30",
        MAIN_REPO, normalized_since
    );

    match gh_json(&["api", &endpoint]) {
        Ok(value) => parse_eva_comments(&value),
        Err(error) => {
            warn(warnings, format!("eva comment scan failed: {}", error));
            Vec::new()
        }
    }
}

fn normalize_since_timestamp(timestamp: &str) -> Option<String> {
    let parsed = DateTime::parse_from_rfc3339(timestamp).ok()?;
    Some(
        parsed
            .with_timezone(&Utc)
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string(),
    )
}

fn gather_review_summary(
    repo_root: &Path,
    review_cycle: u64,
    warnings: &mut Vec<String>,
) -> Option<ReviewSummary> {
    if review_cycle == 0 {
        return None;
    }

    let path = repo_root.join(format!("docs/reviews/cycle-{}.md", review_cycle));
    if !path.exists() {
        return None;
    }

    match fs::read_to_string(&path) {
        Ok(content) => Some(parse_review_summary_from_content(
            review_cycle,
            &path,
            &content,
        )),
        Err(error) => {
            warn(
                warnings,
                format!(
                    "failed reading review summary {}: {}",
                    path.display(),
                    error
                ),
            );
            None
        }
    }
}

fn gather_in_flight_sessions(warnings: &mut Vec<String>) -> InFlightSummary {
    let assigned_issues = match gh_json(&[
        "issue",
        "list",
        "--repo",
        MAIN_REPO,
        "--assignee",
        "copilot-swe-agent[bot]",
        "--state",
        "open",
        "--json",
        "number,title",
    ]) {
        Ok(value) => parse_simple_issues(&value),
        Err(error) => {
            warn(warnings, format!("in-flight issue scan failed: {}", error));
            Vec::new()
        }
    };

    let open_prs = match gh_json(&[
        "pr",
        "list",
        "--repo",
        MAIN_REPO,
        "--state",
        "open",
        "--json",
        "number,title,isDraft,author",
    ]) {
        Ok(value) => parse_copilot_prs(&value),
        Err(error) => {
            warn(warnings, format!("in-flight PR scan failed: {}", error));
            Vec::new()
        }
    };

    let sessions = assigned_issues.len() as u64 + open_prs.len() as u64;

    InFlightSummary {
        assigned_issues,
        open_prs,
        sessions,
    }
}

fn gather_outbound_issue_numbers(repo: &str, label: &str, warnings: &mut Vec<String>) -> Vec<u64> {
    let endpoint = format!(
        "repos/{}/issues?labels={}&state=open&creator=EvaLok&sort=created&direction=asc",
        repo, label
    );

    match gh_json(&["api", &endpoint]) {
        Ok(value) => parse_issue_numbers(&value),
        Err(error) => {
            warn(
                warnings,
                format!("{} outbound scan failed: {}", label, error),
            );
            Vec::new()
        }
    }
}

fn parse_simple_issues(value: &Value) -> Vec<SimpleIssue> {
    value
        .as_array()
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    Some(SimpleIssue {
                        number: entry.get("number")?.as_u64()?,
                        title: entry.get("title")?.as_str()?.to_string(),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_copilot_prs(value: &Value) -> Vec<SimplePr> {
    value
        .as_array()
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    let author = entry
                        .get("author")
                        .and_then(|author| author.get("login"))
                        .and_then(Value::as_str)
                        .unwrap_or("unknown")
                        .to_string();

                    if author != "copilot-swe-agent[bot]" {
                        return None;
                    }

                    Some(SimplePr {
                        number: entry.get("number")?.as_u64()?,
                        title: entry.get("title")?.as_str()?.to_string(),
                        is_draft: entry
                            .get("isDraft")
                            .and_then(Value::as_bool)
                            .unwrap_or(false),
                        author,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_eva_comments(value: &Value) -> Vec<EvaComment> {
    value
        .as_array()
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| {
                    let login = entry
                        .get("user")
                        .and_then(|user| user.get("login"))
                        .and_then(Value::as_str)?;
                    if login != "EvaLok" {
                        return None;
                    }

                    let body = entry.get("body")?.as_str()?.trim();
                    if body.is_empty() {
                        return None;
                    }

                    if ORCHESTRATOR_SIGNATURES
                        .iter()
                        .any(|signature| body.contains(signature))
                    {
                        return None;
                    }

                    let first_line = body.lines().next().unwrap_or_default().trim().to_string();
                    if first_line.is_empty() {
                        return None;
                    }

                    let issue_number = entry
                        .get("issue_url")
                        .and_then(Value::as_str)
                        .and_then(parse_issue_number_from_api_url);

                    Some(EvaComment {
                        issue_number,
                        first_line,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_issue_numbers(value: &Value) -> Vec<u64> {
    value
        .as_array()
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| entry.get("number").and_then(Value::as_u64))
                .collect()
        })
        .unwrap_or_default()
}

fn parse_issue_number_from_api_url(url: &str) -> Option<u64> {
    url.rsplit('/').next()?.parse::<u64>().ok()
}

fn parse_review_summary_from_content(cycle: u64, path: &Path, content: &str) -> ReviewSummary {
    let first_lines: Vec<&str> = content.lines().take(20).collect();

    let mut score = None;
    let mut findings = None;
    let mut in_findings_section = false;
    let mut numbered_findings = 0_u32;

    for line in &first_lines {
        let trimmed = line.trim();
        let lower = trimmed.to_ascii_lowercase();

        if lower.starts_with("## findings") {
            in_findings_section = true;
            continue;
        }

        if lower.starts_with("## ") && !lower.starts_with("## findings") {
            in_findings_section = false;
        }

        if score.is_none() && lower.contains("complacency score") {
            score = extract_first_u32(trimmed).and_then(|value| u8::try_from(value).ok());
        }

        if findings.is_none() && (lower.contains("finding count") || lower.contains("findings:")) {
            findings = extract_first_u32(trimmed);
        }

        if in_findings_section && starts_with_numbered_item(trimmed) {
            numbered_findings += 1;
        }
    }

    if findings.is_none() && numbered_findings > 0 {
        findings = Some(numbered_findings);
    }

    ReviewSummary {
        cycle,
        score,
        findings,
        path: path.display().to_string(),
    }
}

fn extract_first_u32(line: &str) -> Option<u32> {
    line.split(|ch: char| !ch.is_ascii_digit())
        .find(|token| !token.is_empty())
        .and_then(|token| token.parse::<u32>().ok())
}

fn starts_with_numbered_item(line: &str) -> bool {
    let mut chars = line.chars().peekable();
    let mut saw_digit = false;
    while let Some(ch) = chars.peek() {
        if ch.is_ascii_digit() {
            saw_digit = true;
            chars.next();
        } else {
            break;
        }
    }

    if !saw_digit {
        return false;
    }

    matches!(chars.next(), Some('.'))
}

fn summarize_publish_gate(state: &Value) -> String {
    if let Some(summary) = state
        .pointer("/publish_gate/summary")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
    {
        return summary;
    }

    if let Some(status) = state
        .pointer("/publish_gate/status")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
    {
        let reason = state
            .pointer("/publish_gate/reason")
            .and_then(Value::as_str)
            .unwrap_or("no reason");
        return format!("{} ({})", status, reason);
    }

    if let Some(tool_pipeline) = state
        .pointer("/tool_pipeline/publish_gate")
        .and_then(Value::as_str)
        .map(ToOwned::to_owned)
    {
        return tool_pipeline;
    }

    "unknown".to_string()
}

fn format_human_brief(brief: &StartupBrief) -> String {
    let comments_preview = if brief.eva_comments_since_last_cycle.is_empty() {
        "none".to_string()
    } else {
        brief
            .eva_comments_since_last_cycle
            .iter()
            .take(2)
            .map(|comment| match comment.issue_number {
                Some(number) => format!("#{} {}", number, comment.first_line),
                None => comment.first_line.clone(),
            })
            .collect::<Vec<_>>()
            .join("; ")
    };

    let review_line = match &brief.review_agent {
        Some(review) => format!(
            "{}/5, {} findings — {}",
            review
                .score
                .map(|score| score.to_string())
                .unwrap_or_else(|| "?".to_string()),
            review
                .findings
                .map(|count| count.to_string())
                .unwrap_or_else(|| "?".to_string()),
            review.path
        ),
        None => "none".to_string(),
    };

    let qc_line = format_issue_list(&brief.qc_outbound);
    let audit_line = format_issue_list(&brief.audit_outbound);
    let question_line = format_simple_issue_numbers(&brief.questions_for_eva);
    let input_from_eva_line = format_simple_issue_numbers(&brief.input_from_eva);

    let mut lines = vec![
        format!("Cycle {} started (receipt: {})", brief.cycle, brief.receipt),
        String::new(),
        format!("Input from Eva: {}", input_from_eva_line),
        if brief.eva_directives.is_empty() {
            "Eva directives: none".to_string()
        } else {
            format!("Eva directives: {}", brief.eva_directives.join(", "))
        },
        format!(
            "Eva comments since last cycle: {} ({})",
            brief.eva_comments_since_last_cycle.len(),
            comments_preview
        ),
        format!(
            "Review agent (cycle {}): {}",
            brief.cycle.saturating_sub(1),
            review_line
        ),
        format!(
            "Pipeline: {} ({})",
            brief.pipeline.status, brief.pipeline.detail
        ),
        format!("In-flight: {} sessions", brief.in_flight.sessions),
        format!("Publish gate: {}", brief.publish_gate),
        format!("QC outbound: {}", qc_line),
        format!("Audit outbound: {}", audit_line),
        format!("Questions for Eva: {}", question_line),
    ];

    if !brief.warnings.is_empty() {
        lines.push(String::new());
        lines.push("Warnings:".to_string());
        for warning in &brief.warnings {
            lines.push(format!("- {}", warning));
        }
    }

    lines.join("\n")
}

fn format_issue_list(numbers: &[u64]) -> String {
    if numbers.is_empty() {
        return "none".to_string();
    }

    numbers
        .iter()
        .map(|number| format!("#{}", number))
        .collect::<Vec<_>>()
        .join(", ")
}

fn format_simple_issue_numbers(issues: &[SimpleIssue]) -> String {
    if issues.is_empty() {
        return "none".to_string();
    }

    issues
        .iter()
        .map(|issue| format!("#{}", issue.number))
        .collect::<Vec<_>>()
        .join(", ")
}

fn warn(warnings: &mut Vec<String>, message: String) {
    eprintln!("Warning: {}", message);
    warnings.push(message);
}

fn gh_json(args: &[&str]) -> Result<Value, String> {
    let stdout = gh_output(args)?;
    serde_json::from_slice(&stdout).map_err(|error| {
        format!(
            "failed to parse JSON output from `gh {}`: {}",
            args.join(" "),
            error
        )
    })
}

fn gh_text(args: &[&str]) -> Result<String, String> {
    let stdout = gh_output(args)?;
    let text = String::from_utf8(stdout)
        .map_err(|error| format!("gh {} returned non-UTF-8 output: {}", args.join(" "), error))?;
    Ok(text.trim().to_string())
}

fn gh_output(args: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new("gh")
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute gh {}: {}", args.join(" "), error))?;

    if !output.status.success() {
        return Err(command_failure_message(
            &format!("gh {}", args.join(" ")),
            &output,
        ));
    }

    Ok(output.stdout)
}

fn command_failure_message(command: &str, output: &std::process::Output) -> String {
    let code = output.status.code().map_or_else(
        || "terminated by signal".to_string(),
        |value| value.to_string(),
    );
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();

    if stderr.is_empty() {
        format!("{} failed with status {}", command, code)
    } else {
        format!("{} failed with status {}: {}", command, code, stderr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use std::cell::Cell;
    use std::path::PathBuf;
    use std::process::Command;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TempGitRepo {
        path: PathBuf,
    }

    impl TempGitRepo {
        fn new() -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("failed to get duration since UNIX epoch")
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "cycle-start-stale-close-out-{}-{}",
                std::process::id(),
                unique
            ));
            fs::create_dir_all(path.join("docs"))
                .expect("failed to create temp repo docs directory");
            fs::write(
                path.join("docs/state.json"),
                format!("{}\n", minimal_state_json()),
            )
            .expect("failed to write state.json");

            run_git(&path, &["init"]);
            run_git(&path, &["config", "user.name", "Cycle Start Tests"]);
            run_git(
                &path,
                &["config", "user.email", "cycle-start-tests@example.com"],
            );
            run_git(&path, &["add", "docs/state.json"]);
            run_git(&path, &["commit", "-m", "initial state"]);

            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempGitRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn minimal_state_json() -> String {
        json!({
            "last_cycle": {
                "number": 242,
                "issue": 991,
                "timestamp": "2026-03-13T02:00:00Z"
            },
            "cycle_phase": {
                "phase": "close_out",
                "phase_entered_at": "2026-03-13T04:00:00Z",
                "cycle": 242
            },
            "field_inventory": {
                "fields": {
                    "cycle_phase": {
                        "cadence": "every phase transition",
                        "last_refreshed": "cycle 242"
                    }
                }
            }
        })
        .to_string()
    }

    fn run_git(repo_root: &Path, args: &[&str]) -> String {
        let output = Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .args(args)
            .output()
            .expect("failed to execute git command");
        if !output.status.success() {
            panic!(
                "git {:?} failed: {}",
                args,
                String::from_utf8_lossy(&output.stderr).trim()
            );
        }

        String::from_utf8(output.stdout)
            .expect("git stdout is not valid UTF-8")
            .trim()
            .to_string()
    }

    fn read_phase(repo_root: &Path) -> String {
        read_state_value(repo_root)
            .expect("failed to read state.json")
            .pointer("/cycle_phase/phase")
            .and_then(Value::as_str)
            .expect("cycle_phase/phase field missing from state.json")
            .to_string()
    }

    fn git_commit_count(repo_root: &Path) -> usize {
        run_git(repo_root, &["rev-list", "--count", "HEAD"])
            .parse::<usize>()
            .expect("failed to parse git commit count as integer")
    }

    fn git_head_subject(repo_root: &Path) -> String {
        run_git(repo_root, &["log", "-1", "--pretty=%s"])
    }

    #[test]
    fn cycle_number_is_last_plus_one() {
        let derived = derive_cycle_number(162).expect("cycle should derive");
        assert_eq!(derived, 163);
    }

    #[test]
    fn state_patch_contains_required_pointer_paths() {
        let patch = build_state_patch(163, 592, Some(591), "2026-03-06T18:00:00Z", &[600, 601]);
        let paths: Vec<&str> = patch.iter().map(|update| update.path.as_str()).collect();

        assert_eq!(
            paths,
            vec![
                "/last_cycle/number",
                "/last_cycle/issue",
                "/last_cycle/timestamp",
                "/last_eva_comment_check",
                "/open_questions_for_eva",
                "/field_inventory/fields/last_cycle/last_refreshed",
                "/field_inventory/fields/last_eva_comment_check/last_refreshed",
                "/field_inventory/fields/open_questions_for_eva/last_refreshed",
                "/previous_cycle_issue",
                "/field_inventory/fields/previous_cycle_issue",
            ]
        );
        assert_eq!(patch[0].value, json!(163));
        assert_eq!(patch[1].value, json!(592));
        assert_eq!(patch[4].value, json!([600, 601]));
        assert_eq!(patch[5].value, json!("cycle 163"));
        assert_eq!(patch[7].value, json!("cycle 163"));
        assert_eq!(patch[8].value, json!(591));
        assert_eq!(
            patch[9].value,
            json!({
                "cadence": "every cycle",
                "last_refreshed": "cycle 163",
            })
        );
    }

    #[test]
    fn state_patch_uses_empty_array_when_no_open_questions_exist() {
        let patch = build_state_patch(163, 592, None, "2026-03-06T18:00:00Z", &[]);

        let open_questions = patch
            .iter()
            .find(|update| update.path == "/open_questions_for_eva")
            .expect("open_questions_for_eva patch should exist");

        assert_eq!(open_questions.value, json!([]));
    }

    #[test]
    fn state_patch_omits_previous_cycle_issue_when_unavailable() {
        let patch = build_state_patch(163, 592, None, "2026-03-06T18:00:00Z", &[600, 601]);
        let paths: Vec<&str> = patch.iter().map(|update| update.path.as_str()).collect();

        assert!(!paths.contains(&"/previous_cycle_issue"));
        assert!(paths.contains(&"/field_inventory/fields/previous_cycle_issue"));
    }

    #[test]
    fn opening_comment_format_includes_session_metadata() {
        let body = build_opening_comment(182, "2026-03-05T23:12:00Z", "Claude Opus 4.6", "123456");

        assert_eq!(
            body,
            "> **[main-orchestrator]** | Cycle 182\n\n**Session start**: 2026-03-05T23:12:00Z UTC\n**Model**: Claude Opus 4.6\n**Run ID**: 123456\n**Cycle**: 182\n\nBeginning startup checklist. Will post updates as work progresses."
        );
    }

    #[test]
    fn github_run_id_falls_back_to_local_when_unset() {
        assert_eq!(github_run_id_from(None), "local");
    }

    #[test]
    fn github_run_id_uses_environment_value_when_present() {
        assert_eq!(github_run_id_from(Some("123456")), "123456");
    }

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--issue"));
        assert!(help.contains("--model"));
        assert!(help.contains("--repo-root"));
        assert!(help.contains("--json"));
        assert!(help.contains("--stale-threshold"));
    }

    #[test]
    fn human_brief_format_includes_required_lines() {
        let brief = StartupBrief {
            cycle: 163,
            issue: 592,
            receipt: "abc1234".to_string(),
            eva_directives: vec!["EvaLok/schema-org-json-ld#11 (Directive A)".to_string()],
            input_from_eva: vec![],
            eva_comments_since_last_cycle: vec![EvaComment {
                issue_number: Some(591),
                first_line: "Please prioritize this tool.".to_string(),
            }],
            review_agent: Some(ReviewSummary {
                cycle: 162,
                score: Some(2),
                findings: Some(3),
                path: "docs/reviews/cycle-162.md".to_string(),
            }),
            pipeline: PipelineStatus {
                status: "pass".to_string(),
                detail: "5 steps".to_string(),
            },
            in_flight: InFlightSummary {
                assigned_issues: vec![],
                open_prs: vec![],
                sessions: 1,
            },
            publish_gate: "open".to_string(),
            qc_outbound: vec![496],
            audit_outbound: vec![],
            questions_for_eva: vec![SimpleIssue {
                number: 600,
                title: "Need decision".to_string(),
            }],
            warnings: vec![],
        };

        let output = format_human_brief(&brief);
        assert!(output.contains("Cycle 163 started (receipt: abc1234)"));
        assert!(output.contains("Input from Eva: none"));
        assert!(output.contains("Eva directives: EvaLok/schema-org-json-ld#11 (Directive A)"));
        assert!(
            output.contains("Eva comments since last cycle: 1 (#591 Please prioritize this tool.)")
        );
        assert!(output
            .contains("Review agent (cycle 162): 2/5, 3 findings — docs/reviews/cycle-162.md"));
        assert!(output.contains("Pipeline: pass (5 steps)"));
        assert!(output.contains("QC outbound: #496"));
        assert!(output.contains("Questions for Eva: #600"));
    }

    #[test]
    fn json_output_mode_serializes_brief() {
        let brief = StartupBrief {
            cycle: 163,
            issue: 592,
            receipt: "abc1234".to_string(),
            eva_directives: vec!["EvaLok/schema-org-json-ld#11 (Directive A)".to_string()],
            input_from_eva: vec![SimpleIssue {
                number: 593,
                title: "Input".to_string(),
            }],
            eva_comments_since_last_cycle: vec![],
            review_agent: None,
            pipeline: PipelineStatus {
                status: "fail".to_string(),
                detail: "5 steps".to_string(),
            },
            in_flight: InFlightSummary {
                assigned_issues: vec![],
                open_prs: vec![],
                sessions: 0,
            },
            publish_gate: "closed".to_string(),
            qc_outbound: vec![],
            audit_outbound: vec![],
            questions_for_eva: vec![],
            warnings: vec!["pipeline-check command failed".to_string()],
        };

        let output =
            serde_json::to_string_pretty(&brief).expect("json serialization should succeed");
        let parsed: Value = serde_json::from_str(&output).expect("json should parse");

        assert_eq!(parsed.get("cycle"), Some(&json!(163)));
        assert_eq!(parsed.get("issue"), Some(&json!(592)));
        assert_eq!(parsed.pointer("/pipeline/status"), Some(&json!("fail")));
        assert_eq!(
            parsed.pointer("/warnings/0"),
            Some(&json!("pipeline-check command failed"))
        );
    }

    #[test]
    fn load_eva_directives_uses_remaining_open_issue_titles() {
        let mut state = StateJson::default();
        state.eva_input_issues.remaining_open = vec![11, 12];

        let directives = load_eva_directives_with(&state, |issue_number| match issue_number {
            11 => Ok("Directive A".to_string()),
            12 => Ok("Directive B".to_string()),
            other => Err(format!("unexpected issue {}", other)),
        })
        .expect("eva directives should load");

        assert_eq!(
            directives,
            vec![
                "EvaLok/schema-org-json-ld#11 (Directive A)".to_string(),
                "EvaLok/schema-org-json-ld#12 (Directive B)".to_string(),
            ]
        );
    }

    #[test]
    fn load_eva_directives_rejects_negative_issue_numbers() {
        let mut state = StateJson::default();
        state.eva_input_issues.remaining_open = vec![-1];

        let error = load_eva_directives_with(&state, |_| Ok("unused".to_string()))
            .expect_err("negative directive numbers must fail");

        assert_eq!(
            error,
            "docs/state.json contains invalid /eva_input_issues/remaining_open entry (negative values are not allowed): -1"
        );
    }

    #[test]
    fn human_brief_format_shows_input_from_eva_issues() {
        let brief = StartupBrief {
            cycle: 198,
            issue: 820,
            receipt: "def5678".to_string(),
            eva_directives: vec![],
            input_from_eva: vec![
                SimpleIssue {
                    number: 808,
                    title: "Pause language ports".to_string(),
                },
                SimpleIssue {
                    number: 809,
                    title: "Iterate on Copilot PRs".to_string(),
                },
            ],
            eva_comments_since_last_cycle: vec![],
            review_agent: None,
            pipeline: PipelineStatus {
                status: "pass".to_string(),
                detail: "5 steps".to_string(),
            },
            in_flight: InFlightSummary {
                assigned_issues: vec![],
                open_prs: vec![],
                sessions: 0,
            },
            publish_gate: "open".to_string(),
            qc_outbound: vec![],
            audit_outbound: vec![],
            questions_for_eva: vec![],
            warnings: vec![],
        };

        let output = format_human_brief(&brief);
        assert!(output.contains("Input from Eva: #808, #809"));
    }

    #[test]
    fn human_brief_format_shows_none_when_no_eva_directives_exist() {
        let brief = StartupBrief {
            cycle: 163,
            issue: 592,
            receipt: "abc1234".to_string(),
            eva_directives: vec![],
            input_from_eva: vec![],
            eva_comments_since_last_cycle: vec![],
            review_agent: None,
            pipeline: PipelineStatus {
                status: "pass".to_string(),
                detail: "5 steps".to_string(),
            },
            in_flight: InFlightSummary {
                assigned_issues: vec![],
                open_prs: vec![],
                sessions: 0,
            },
            publish_gate: "open".to_string(),
            qc_outbound: vec![],
            audit_outbound: vec![],
            questions_for_eva: vec![],
            warnings: vec![],
        };

        let output = format_human_brief(&brief);
        assert!(output.contains("Eva directives: none"));
    }

    #[test]
    fn resume_detects_work_phase() {
        assert!(should_resume(Some("work")));
    }

    #[test]
    fn resume_detects_close_out_phase() {
        assert!(should_resume(Some("close_out")));
    }

    #[test]
    fn normal_start_when_phase_is_complete() {
        assert!(!should_resume(Some("complete")));
    }

    #[test]
    fn normal_start_when_phase_is_absent() {
        assert!(!should_resume(None));
    }

    #[test]
    fn resume_json_includes_resume_flag_and_core_fields() {
        let parsed = build_resume_json(219, "close_out");

        assert_eq!(parsed.get("resume"), Some(&json!(true)));
        assert_eq!(parsed.get("cycle"), Some(&json!(219)));
        assert_eq!(parsed.get("phase"), Some(&json!("close_out")));
        assert_eq!(parsed.as_object().map(|obj| obj.len()), Some(3));
    }

    #[test]
    fn stale_close_out_detected_when_threshold_is_met() {
        let now = DateTime::parse_from_rfc3339("2026-03-13T06:00:00Z")
            .expect("timestamp should parse")
            .with_timezone(&Utc);

        let detected = detect_stale_close_out(
            Some("close_out"),
            Some("2026-03-13T04:00:00Z"),
            DEFAULT_STALE_THRESHOLD_SECS,
            now,
        )
        .expect("stale detection should succeed");

        assert_eq!(detected.as_deref(), Some("2026-03-13T04:00:00Z"));
    }

    #[test]
    fn stale_close_out_not_detected_before_threshold() {
        let now = DateTime::parse_from_rfc3339("2026-03-13T05:59:59Z")
            .expect("timestamp should parse")
            .with_timezone(&Utc);

        let detected = detect_stale_close_out(
            Some("close_out"),
            Some("2026-03-13T04:00:00Z"),
            DEFAULT_STALE_THRESHOLD_SECS,
            now,
        )
        .expect("stale detection should succeed");

        assert!(detected.is_none());
    }

    #[test]
    fn stale_close_out_comment_is_signed_and_explains_recovery() {
        let comment = build_stale_close_out_comment(241, "2026-03-13T01:15:00Z");

        assert!(comment.starts_with("> **[main-orchestrator]** | Cycle 241"));
        assert!(comment.contains("terminated during close-out"));
        assert!(comment.contains("2026-03-13T01:15:00Z"));
        assert!(comment.contains("Recovery auto-completed the stale close-out"));
    }

    #[test]
    fn stale_close_out_recovery_closes_issue_before_committing_state() {
        let repo = TempGitRepo::new();
        let mut state = read_state_value(repo.path()).expect("state should load");

        recover_stale_close_out_with(
            repo.path(),
            &mut state,
            242,
            Some(991),
            "2026-03-13T04:00:00Z",
            |issue, cycle, entered_at| {
                assert_eq!(issue, 991);
                assert_eq!(cycle, 242);
                assert_eq!(entered_at, "2026-03-13T04:00:00Z");
                assert_eq!(read_phase(repo.path()), "close_out");
                assert_eq!(git_commit_count(repo.path()), 1);
                Ok(())
            },
        )
        .expect("recovery should succeed");

        assert_eq!(read_phase(repo.path()), "complete");
        assert_eq!(git_commit_count(repo.path()), 2);
        assert_eq!(
            git_head_subject(repo.path()),
            "state(cycle-start): recover stale close-out for cycle 242 [cycle 242]"
        );
    }

    #[test]
    fn stale_close_out_recovery_retries_when_issue_closure_fails() {
        let repo = TempGitRepo::new();
        let attempts = Cell::new(0);
        let mut state = read_state_value(repo.path()).expect("state should load");

        let error = recover_stale_close_out_with(
            repo.path(),
            &mut state,
            242,
            Some(991),
            "2026-03-13T04:00:00Z",
            |_, _, _| {
                attempts.set(attempts.get() + 1);
                Err("simulated close failure".to_string())
            },
        )
        .expect_err("recovery should fail when issue closure fails");

        assert_eq!(error, "simulated close failure");
        assert_eq!(attempts.get(), 1);
        assert_eq!(read_phase(repo.path()), "close_out");
        assert_eq!(git_commit_count(repo.path()), 1);

        let mut retry_state = read_state_value(repo.path()).expect("state should reload");
        recover_stale_close_out_with(
            repo.path(),
            &mut retry_state,
            242,
            Some(991),
            "2026-03-13T04:00:00Z",
            |_, _, _| {
                attempts.set(attempts.get() + 1);
                Ok(())
            },
        )
        .expect("recovery retry should succeed");

        assert_eq!(attempts.get(), 2);
        assert_eq!(read_phase(repo.path()), "complete");
        assert_eq!(git_commit_count(repo.path()), 2);
    }
}
