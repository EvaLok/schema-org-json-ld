use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};
use state_schema::{set_value_at_pointer, StateJson};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser)]
#[command(name = "cycle-complete")]
struct Cli {
    /// Path to the repository root
    #[arg(long)]
    repo_root: PathBuf,

    /// Current cycle number
    #[arg(long)]
    cycle: u64,

    /// Current cycle issue number
    #[arg(long)]
    issue: u64,

    /// Output report as JSON
    #[arg(long)]
    json: bool,

    /// Apply computed state patch updates to docs/state.json
    #[arg(long)]
    apply: bool,

    /// Cycle summary text for /last_cycle/summary
    #[arg(long)]
    summary: Option<String>,

    /// Commit docs/state.json after applying changes
    #[arg(long)]
    commit: bool,
}

#[derive(Clone, Copy, Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
enum StepStatus {
    Pass,
    Warn,
    Ready,
    Pending,
}

#[derive(Serialize)]
struct CompletionReport {
    cycle: u64,
    issue: u64,
    timestamp: String,
    pipeline_check: PipelineCheckStatus,
    state_json_patch: StatePatch,
    review_agent_body: String,
    completion_steps: Vec<CompletionStep>,
}

#[derive(Serialize)]
struct PipelineCheckStatus {
    status: StepStatus,
    detail: String,
}

#[derive(Serialize)]
struct StatePatch {
    updates: Vec<PatchUpdate>,
}

#[derive(Serialize)]
struct PatchUpdate {
    path: String,
    value: Value,
}

#[derive(Serialize)]
struct CompletionStep {
    index: u8,
    name: &'static str,
    status: StepStatus,
    detail: String,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = validate_cli_flags(&cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }

    let state = match read_state_json(&cli.repo_root) {
        Ok(state) => state,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let now = Utc::now();
    let summary = cli
        .summary
        .as_deref()
        .unwrap_or("TODO: Fill cycle summary.");
    let report = assemble_report(cli.cycle, cli.issue, now, &state, summary);

    if cli.apply {
        match apply_cycle_patch(&cli.repo_root, &report.state_json_patch) {
            Ok(changed_paths) => {
                print_patch_apply_summary(&changed_paths);
                if cli.commit {
                    match commit_state_json(&cli.repo_root, summary, cli.cycle) {
                        Ok(sha) => println!("Committed: {}", sha),
                        Err(error) => {
                            eprintln!("Error: {}", error);
                            std::process::exit(1);
                        }
                    }
                }
            }
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        }
        return;
    }

    if cli.json {
        match serde_json::to_string_pretty(&report) {
            Ok(output) => println!("{}", output),
            Err(error) => {
                eprintln!("Error: failed to serialize report: {}", error);
                std::process::exit(2);
            }
        }
    } else {
        print_human_report(&report);
    }
}

fn validate_cli_flags(cli: &Cli) -> Result<(), String> {
    if cli.commit && !cli.apply {
        return Err("--commit requires --apply".to_string());
    }

    Ok(())
}

fn read_state_json(repo_root: &Path) -> Result<StateJson, String> {
    let state_path = repo_root.join("docs/state.json");
    let content = fs::read_to_string(&state_path)
        .map_err(|error| format!("failed to read {}: {}", state_path.display(), error))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("failed to parse {}: {}", state_path.display(), error))
}

fn read_state_value(path: &Path) -> Result<Value, String> {
    let content = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    serde_json::from_str::<Value>(&content)
        .map_err(|error| format!("failed to parse {}: {}", path.display(), error))
}

fn assemble_report(
    cycle: u64,
    issue: u64,
    now: DateTime<Utc>,
    state: &StateJson,
    summary: &str,
) -> CompletionReport {
    let timestamp = format_timestamp(now);
    let pipeline_check = validate_pipeline_check(state, cycle);
    let state_json_patch = build_state_patch(cycle, issue, &timestamp, state, summary);
    let review_agent_body = build_review_agent_body(cycle, issue, now);
    let completion_steps = build_completion_steps(&pipeline_check, &state_json_patch);

    CompletionReport {
        cycle,
        issue,
        timestamp,
        pipeline_check,
        state_json_patch,
        review_agent_body,
        completion_steps,
    }
}

fn format_timestamp(now: DateTime<Utc>) -> String {
    now.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn validate_pipeline_check(state: &StateJson, cycle: u64) -> PipelineCheckStatus {
    let last_clean_cycle = state
        .extra
        .get("pipeline_reliability")
        .and_then(|value| value.get("last_clean_cycle"))
        .and_then(Value::as_u64);
    let metric_verified = state
        .next_metric_verification
        .as_deref()
        .is_some_and(|value| text_mentions_cycle(value, cycle));
    let clean_cycle_verified = last_clean_cycle.is_some_and(|value| value >= cycle);

    if clean_cycle_verified || metric_verified {
        PipelineCheckStatus {
            status: StepStatus::Pass,
            detail: "verified this cycle".to_string(),
        }
    } else {
        PipelineCheckStatus {
            status: StepStatus::Warn,
            detail: "not verified this cycle".to_string(),
        }
    }
}

fn text_mentions_cycle(text: &str, cycle: u64) -> bool {
    text.split(|ch: char| !ch.is_ascii_digit())
        .filter(|token| !token.is_empty())
        .filter_map(|token| token.parse::<u64>().ok())
        .any(|number| number == cycle)
}

fn build_state_patch(
    cycle: u64,
    issue: u64,
    timestamp: &str,
    state: &StateJson,
    summary: &str,
) -> StatePatch {
    let mut updates = vec![
        PatchUpdate {
            path: "/last_cycle/issue".to_string(),
            value: json!(issue),
        },
        PatchUpdate {
            path: "/last_cycle/timestamp".to_string(),
            value: json!(timestamp),
        },
        PatchUpdate {
            path: "/last_cycle/summary".to_string(),
            value: json!(summary),
        },
        PatchUpdate {
            path: "/last_eva_comment_check".to_string(),
            value: json!(timestamp),
        },
    ];

    updates.extend(build_freshness_updates(cycle, &updates, state));
    StatePatch { updates }
}

fn apply_state_patch(state_value: &mut Value, patch: &StatePatch) -> Result<Vec<String>, String> {
    let mut changed_paths = Vec::new();
    for update in &patch.updates {
        if set_value_at_pointer(state_value, &update.path, update.value.clone())? {
            changed_paths.push(update.path.clone());
        }
    }

    Ok(changed_paths)
}

fn apply_cycle_patch(repo_root: &Path, patch: &StatePatch) -> Result<Vec<String>, String> {
    let state_path = repo_root.join("docs/state.json");
    let mut state_value = read_state_value(&state_path)?;
    let changed_paths = apply_state_patch(&mut state_value, patch)?;
    let serialized = serde_json::to_string_pretty(&state_value)
        .map_err(|error| format!("failed to serialize state.json: {}", error))?;
    fs::write(&state_path, format!("{}\n", serialized))
        .map_err(|error| format!("failed to write {}: {}", state_path.display(), error))?;

    Ok(changed_paths)
}

fn print_patch_apply_summary(changed_paths: &[String]) {
    println!("Applied {} state update(s).", changed_paths.len());
    if changed_paths.is_empty() {
        return;
    }

    println!("Updated paths:");
    for path in changed_paths {
        println!("- {}", path);
    }
}

fn commit_state_json(repo_root: &Path, summary: &str, cycle: u64) -> Result<String, String> {
    let add_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("add")
        .arg("docs/state.json")
        .output()
        .map_err(|error| format!("failed to execute git add: {}", error))?;
    if !add_output.status.success() {
        let stderr = String::from_utf8_lossy(&add_output.stderr)
            .trim()
            .to_string();
        return Err(format!("git add docs/state.json failed: {}", stderr));
    }

    let commit_message = format!("state(cycle-complete): {} [cycle {}]", summary, cycle);
    let commit_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .output()
        .map_err(|error| format!("failed to execute git commit: {}", error))?;
    if !commit_output.status.success() {
        let stderr = String::from_utf8_lossy(&commit_output.stderr)
            .trim()
            .to_string();
        return Err(format!("git commit failed: {}", stderr));
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("rev-parse")
        .arg("--short=7")
        .arg("HEAD")
        .output()
        .map_err(|error| format!("failed to execute git rev-parse: {}", error))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("git rev-parse --short=7 HEAD failed: {}", stderr));
    }

    let sha = String::from_utf8(output.stdout)
        .map_err(|error| format!("failed to decode git rev-parse output as UTF-8: {}", error))?;
    Ok(sha.trim().to_string())
}

fn build_freshness_updates(
    cycle: u64,
    updates: &[PatchUpdate],
    state: &StateJson,
) -> Vec<PatchUpdate> {
    let tracked_fields: BTreeSet<&str> = state
        .field_inventory
        .fields
        .keys()
        .map(String::as_str)
        .collect();
    let mut refreshed_fields = BTreeSet::new();

    for update in updates {
        if update.path.starts_with("/field_inventory/") {
            continue;
        }
        if let Some(field_name) = inventory_field_for_patch_path(&update.path, &tracked_fields) {
            refreshed_fields.insert(field_name.to_string());
        }
    }

    let cycle_marker = format!("cycle {}", cycle);
    refreshed_fields
        .into_iter()
        .map(|field_name| PatchUpdate {
            path: format!("/field_inventory/fields/{}/last_refreshed", field_name),
            value: json!(cycle_marker),
        })
        .collect()
}

/// Matches a JSON pointer patch path to the best tracked field inventory key.
///
/// The matcher tries progressively shorter dotted prefixes from the full pointer path.
/// Example: `/last_cycle/issue` tries `last_cycle.issue`, then `last_cycle`.
fn inventory_field_for_patch_path(path: &str, tracked_fields: &BTreeSet<&str>) -> Option<String> {
    let segments: Vec<&str> = path.trim_start_matches('/').split('/').collect();
    if segments.is_empty() || segments[0].is_empty() {
        return None;
    }

    for index in (1..=segments.len()).rev() {
        let candidate = segments[..index].join(".");
        if tracked_fields.contains(candidate.as_str()) {
            return Some(candidate);
        }
    }

    None
}

fn build_review_agent_body(cycle: u64, issue: u64, now: DateTime<Utc>) -> String {
    let date = now.format("%Y-%m-%d");
    let time = now.format("%H%M%S");
    format!(
        "## End-of-Cycle Review — Cycle {cycle}

You are a review agent dispatched at the end of orchestrator cycle {cycle} (issue [#{issue}](https://github.com/EvaLok/schema-org-json-ld/issues/{issue})).

Your job is to review the cycle's work and provide honest, critical feedback. Commit your findings as a file at `docs/reviews/cycle-{cycle}.md`. Copilot coding agents CANNOT post issue comments — your only output mode is committing files in a PR.

### What to review

1. **Recent commits on master** since the last cycle — check for:
   - Code quality issues
   - Stale or inaccurate documentation
   - Infrastructure drift (AGENTS.md, skills, checklists out of sync with practice)
   - Test coverage gaps

2. **Worklog entry** at `docs/worklog/{date}/{time}-{{name}}.md` — check for:
   - Accuracy and completeness
   - Whether \"next steps\" are actionable
   - Whether self-modifications are properly documented

3. **Journal entry** at `docs/journal/{date}.md` — check for:
   - Genuine reflection vs formulaic/boilerplate entries
   - Complacency indicators (repeating the same observations without acting on them)
   - Missing lessons from challenges encountered

4. **State.json** at `docs/state.json` — check for:
   - Stale metrics (compare file counts against actual `ls` output)
   - Field inventory cadence violations
   - Inconsistencies between state.json and reality

5. **Complacency audit** — honestly assess:
   - Is the orchestrator genuinely improving, or going through motions?
   - Are there repeated patterns that should have been automated by now?
   - Is the journal adding value or just filling space?
   - Are worklog \"next steps\" actually being followed through?

### Output format

Commit a file at `docs/reviews/cycle-{cycle}.md` containing:
- **Findings**: Numbered list of specific observations (with file paths and line numbers where relevant)
- **Recommendations**: Concrete actions for the next cycle
- **Complacency score**: 1-5 scale (1 = actively improving, 5 = going through motions)
- **Priority items**: Top 3 things the next cycle should address

**IMPORTANT**: Do NOT attempt to post a comment on this issue. Your only output is the committed review file in your PR.
"
    )
}

fn build_completion_steps(
    pipeline_check: &PipelineCheckStatus,
    state_patch: &StatePatch,
) -> Vec<CompletionStep> {
    vec![
        CompletionStep {
            index: 1,
            name: "pipeline-check",
            status: pipeline_check.status,
            detail: pipeline_check.detail.clone(),
        },
        CompletionStep {
            index: 2,
            name: "state-json-patch",
            status: StepStatus::Ready,
            detail: format!("{} fields to update", state_patch.updates.len()),
        },
        CompletionStep {
            index: 3,
            name: "review-agent-body",
            status: StepStatus::Ready,
            detail: "generated".to_string(),
        },
        CompletionStep {
            index: 4,
            name: "worklog",
            status: StepStatus::Pending,
            detail: "manual step".to_string(),
        },
        CompletionStep {
            index: 5,
            name: "journal",
            status: StepStatus::Pending,
            detail: "manual step".to_string(),
        },
        CompletionStep {
            index: 6,
            name: "commit-push",
            status: StepStatus::Pending,
            detail: "manual step".to_string(),
        },
        CompletionStep {
            index: 7,
            name: "close-issue",
            status: StepStatus::Pending,
            detail: "manual step".to_string(),
        },
    ]
}

fn print_human_report(report: &CompletionReport) {
    println!("Cycle Completion — Cycle {}", report.cycle);
    println!();
    for step in &report.completion_steps {
        println!(
            "  {}. {:<19} {:<7} ({})",
            step.index,
            format!("{}:", step.name),
            status_label(step.status),
            step.detail
        );
    }
    println!();
    println!("State JSON Patch:");
    match serde_json::to_string_pretty(&report.state_json_patch) {
        Ok(json) => println!("{}", json),
        Err(error) => {
            eprintln!("Error: failed to format state patch JSON: {}", error);
            std::process::exit(2);
        }
    }
    println!();
    println!("Review Agent Issue Body:");
    println!("{}", report.review_agent_body);
}

fn status_label(status: StepStatus) -> &'static str {
    match status {
        StepStatus::Pass => "PASS",
        StepStatus::Warn => "WARN",
        StepStatus::Ready => "READY",
        StepStatus::Pending => "PENDING",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use std::collections::BTreeMap;

    fn fixed_now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-03-05T05:06:07Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("Usage:"));
        assert!(help.contains("--repo-root"));
        assert!(help.contains("--cycle"));
        assert!(help.contains("--issue"));
        assert!(help.contains("--json"));
        assert!(help.contains("--apply"));
        assert!(help.contains("--summary"));
        assert!(help.contains("--commit"));
    }

    #[test]
    fn state_patch_contains_expected_updates() {
        let mut state = StateJson::default();
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_eva_comment_check".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        let patch = build_state_patch(139, 464, "2026-03-05T05:06:07Z", &state, "summary");
        assert_eq!(patch.updates.len(), 6);
        assert_eq!(patch.updates[0].path, "/last_cycle/issue");
        assert_eq!(patch.updates[0].value, json!(464));
        assert_eq!(patch.updates[1].path, "/last_cycle/timestamp");
        assert_eq!(patch.updates[1].value, json!("2026-03-05T05:06:07Z"));
        assert_eq!(patch.updates[2].path, "/last_cycle/summary");
        assert_eq!(patch.updates[2].value, json!("summary"));
        assert_eq!(
            patch.updates[4].path,
            "/field_inventory/fields/last_cycle/last_refreshed"
        );
        assert_eq!(patch.updates[4].value, json!("cycle 139"));
        assert_eq!(
            patch.updates[5].path,
            "/field_inventory/fields/last_eva_comment_check/last_refreshed"
        );
        assert_eq!(patch.updates[5].value, json!("cycle 139"));
    }

    #[test]
    fn state_patch_generates_freshness_for_modified_fields() {
        let mut state = StateJson::default();
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_eva_comment_check".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );

        let patch = build_state_patch(153, 700, "2026-03-06T00:00:00Z", &state, "summary");
        let freshness_paths: Vec<&str> = patch
            .updates
            .iter()
            .map(|update| update.path.as_str())
            .filter(|path| path.starts_with("/field_inventory/fields/"))
            .collect();

        assert!(freshness_paths.contains(&"/field_inventory/fields/last_cycle/last_refreshed"));
        assert!(freshness_paths
            .contains(&"/field_inventory/fields/last_eva_comment_check/last_refreshed"));
    }

    #[test]
    fn state_patch_deduplicates_freshness_for_repeated_field_updates() {
        let mut state = StateJson::default();
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );

        let updates = vec![
            PatchUpdate {
                path: "/last_cycle/issue".to_string(),
                value: json!(1),
            },
            PatchUpdate {
                path: "/last_cycle/timestamp".to_string(),
                value: json!("2026-03-06T00:00:00Z"),
            },
            PatchUpdate {
                path: "/last_cycle/summary".to_string(),
                value: json!("summary"),
            },
        ];

        let freshness_updates = build_freshness_updates(153, &updates, &state);
        assert_eq!(freshness_updates.len(), 1);
        assert_eq!(
            freshness_updates[0].path,
            "/field_inventory/fields/last_cycle/last_refreshed"
        );
    }

    #[test]
    fn summary_flag_overrides_placeholder_text_in_patch() {
        let state = StateJson::default();
        let patch = build_state_patch(153, 700, "2026-03-06T00:00:00Z", &state, "custom summary");
        assert_eq!(patch.updates[2].path, "/last_cycle/summary");
        assert_eq!(patch.updates[2].value, json!("custom summary"));
    }

    #[test]
    fn apply_state_patch_applies_all_updates_and_freshness_markers() {
        let mut state = StateJson::default();
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_eva_comment_check".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );

        let patch = build_state_patch(153, 700, "2026-03-06T00:00:00Z", &state, "custom summary");
        let mut raw_state = json!({
            "last_cycle": {
                "issue": 100,
                "timestamp": "2026-02-01T00:00:00Z",
                "summary": "old summary"
            },
            "last_eva_comment_check": "2026-02-01T00:00:00Z",
            "field_inventory": {
                "fields": {
                    "last_cycle": {"last_refreshed": "cycle 120"},
                    "last_eva_comment_check": {"last_refreshed": "cycle 120"}
                }
            }
        });

        let changed_paths =
            apply_state_patch(&mut raw_state, &patch).expect("state patch should apply cleanly");
        assert_eq!(changed_paths.len(), 6);
        assert_eq!(
            raw_state
                .pointer("/last_cycle/issue")
                .and_then(Value::as_u64),
            Some(700)
        );
        assert_eq!(
            raw_state
                .pointer("/last_cycle/summary")
                .and_then(Value::as_str),
            Some("custom summary")
        );
        assert_eq!(
            raw_state
                .pointer("/field_inventory/fields/last_cycle/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 153")
        );
        assert_eq!(
            raw_state
                .pointer("/field_inventory/fields/last_eva_comment_check/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 153")
        );
    }

    #[test]
    fn commit_without_apply_produces_error() {
        let cli = Cli::parse_from([
            "cycle-complete",
            "--repo-root",
            ".",
            "--cycle",
            "162",
            "--issue",
            "585",
            "--commit",
        ]);
        let error = validate_cli_flags(&cli).expect_err("commit without apply must fail");
        assert_eq!(error, "--commit requires --apply");
    }

    #[test]
    fn apply_flag_is_recognized_by_parser() {
        let parsed = Cli::try_parse_from([
            "cycle-complete",
            "--repo-root",
            ".",
            "--cycle",
            "162",
            "--issue",
            "585",
            "--apply",
        ]);
        assert!(parsed.is_ok());
    }

    #[test]
    fn review_agent_body_fills_cycle_issue_and_paths() {
        let body = build_review_agent_body(139, 464, fixed_now());
        assert!(body.contains("Cycle 139"));
        assert!(body.contains("[#464](https://github.com/EvaLok/schema-org-json-ld/issues/464)"));
        assert!(body.contains("docs/worklog/2026-03-05/050607-{name}.md"));
        assert!(body.contains("docs/journal/2026-03-05.md"));
        assert!(!body.contains("{N}"));
        assert!(!body.contains("{issue}"));
        assert!(!body.contains("{date}"));
        assert!(!body.contains("{time}"));
    }

    #[test]
    fn review_agent_body_enforces_file_based_delivery_policy() {
        let body = build_review_agent_body(139, 464, fixed_now());
        assert!(body.contains("docs/reviews/cycle-"));
        assert!(body.contains("Commit your findings"));
        assert!(!body.contains("Post your findings as a comment"));
    }

    #[test]
    fn pipeline_validation_warns_when_not_verified() {
        let state = StateJson::default();
        let status = validate_pipeline_check(&state, 139);
        assert_eq!(status.status, StepStatus::Warn);
        assert_eq!(status.detail, "not verified this cycle");
    }

    #[test]
    fn json_report_serializes_to_valid_json() {
        let mut state = StateJson::default();
        state.next_metric_verification = Some("cycle 139".to_string());
        state.extra = BTreeMap::new();
        state.field_inventory.fields.insert(
            "last_cycle".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );
        state.field_inventory.fields.insert(
            "last_eva_comment_check".to_string(),
            json!({"last_refreshed": "cycle 120"}),
        );

        let report = assemble_report(139, 464, fixed_now(), &state, "summary");
        let output = serde_json::to_string_pretty(&report).unwrap();
        let parsed: Value = serde_json::from_str(&output).unwrap();

        assert_eq!(parsed.get("cycle"), Some(&json!(139)));
        assert_eq!(parsed.get("issue"), Some(&json!(464)));
        assert_eq!(
            parsed
                .pointer("/pipeline_check/status")
                .and_then(Value::as_str),
            Some("pass")
        );
        assert_eq!(
            parsed
                .pointer("/completion_steps/1/detail")
                .and_then(Value::as_str),
            Some("6 fields to update")
        );
    }
}
