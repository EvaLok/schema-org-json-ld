use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};
use state_schema::StateJson;
use std::fs;
use std::path::{Path, PathBuf};

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
    let state = match read_state_json(&cli.repo_root) {
        Ok(state) => state,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let now = Utc::now();
    let report = assemble_report(cli.cycle, cli.issue, now, &state);

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

fn read_state_json(repo_root: &Path) -> Result<StateJson, String> {
    let state_path = repo_root.join("docs/state.json");
    let content = fs::read_to_string(&state_path)
        .map_err(|error| format!("failed to read {}: {}", state_path.display(), error))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("failed to parse {}: {}", state_path.display(), error))
}

fn assemble_report(
    cycle: u64,
    issue: u64,
    now: DateTime<Utc>,
    state: &StateJson,
) -> CompletionReport {
    let timestamp = format_timestamp(now);
    let pipeline_check = validate_pipeline_check(state, cycle);
    let state_json_patch = build_state_patch(cycle, issue, &timestamp);
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

fn build_state_patch(cycle: u64, issue: u64, timestamp: &str) -> StatePatch {
    let cycle_marker = format!("cycle {}", cycle);
    StatePatch {
        updates: vec![
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
                value: json!("TODO: Fill cycle summary."),
            },
            PatchUpdate {
                path: "/last_eva_comment_check".to_string(),
                value: json!(timestamp),
            },
            PatchUpdate {
                path: "/field_inventory/fields/last_cycle/last_refreshed".to_string(),
                value: json!(cycle_marker),
            },
            PatchUpdate {
                path: "/field_inventory/fields/last_eva_comment_check/last_refreshed".to_string(),
                value: json!(cycle_marker),
            },
        ],
    }
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
    }

    #[test]
    fn state_patch_contains_expected_updates() {
        let patch = build_state_patch(139, 464, "2026-03-05T05:06:07Z");
        assert_eq!(patch.updates.len(), 6);
        assert_eq!(patch.updates[0].path, "/last_cycle/issue");
        assert_eq!(patch.updates[0].value, json!(464));
        assert_eq!(patch.updates[1].path, "/last_cycle/timestamp");
        assert_eq!(patch.updates[1].value, json!("2026-03-05T05:06:07Z"));
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

        let report = assemble_report(139, 464, fixed_now(), &state);
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
