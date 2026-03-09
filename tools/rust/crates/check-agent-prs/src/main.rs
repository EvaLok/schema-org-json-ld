use chrono::{DateTime, Duration, Utc};
use clap::Parser;
use serde::Serialize;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::process::Command;

const REPO: &str = "EvaLok/schema-org-json-ld";
const COPILOT_AUTHOR: &str = "copilot-swe-agent[bot]";
const STALE_THRESHOLD_HOURS: i64 = 2;

#[derive(Debug, Parser)]
#[command(name = "check-agent-prs")]
struct Cli {
    /// Output report as JSON
    #[arg(long)]
    json: bool,

    /// Path to the repository root
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Clone, Debug)]
struct PullRequestSummary {
    number: u64,
    title: String,
    is_draft: bool,
    created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TimelineEvent {
    event: CopilotEventKind,
    created_at: DateTime<Utc>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CopilotEventKind {
    WorkStarted,
    WorkFinished,
}

impl CopilotEventKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::WorkStarted => "copilot_work_started",
            Self::WorkFinished => "copilot_work_finished",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum PrStatus {
    Ready,
    Working,
    Stale,
}

#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum CiStatus {
    Success,
    Pending,
    Failure,
    None,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
struct PrReadiness {
    pr_number: u64,
    title: String,
    status: PrStatus,
    last_event: Option<String>,
    last_event_at: Option<String>,
    ci_status: Option<CiStatus>,
    is_draft: bool,
    #[serde(skip_serializing)]
    stale_reason: Option<String>,
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

    let now = Utc::now();
    let mut prs = fetch_open_copilot_prs(&cli.repo_root)?;
    prs.sort_by_key(|pr| pr.number);
    let mut reports = Vec::new();
    for pr in prs {
        let timeline = fetch_timeline(&cli.repo_root, pr.number)?;
        let mut report = classify_pr(&pr, &timeline, now);
        if report.status == PrStatus::Ready {
            report.ci_status = Some(fetch_ci_status(&cli.repo_root, pr.number)?);
        }
        reports.push(report);
    }

    if cli.json {
        let json = serde_json::to_string_pretty(&reports)
            .map_err(|error| format!("failed to serialize JSON output: {}", error))?;
        println!("{}", json);
    } else if reports.is_empty() {
        println!("No open Copilot PRs found.");
    } else {
        for report in &reports {
            println!("{}", render_human_line(report));
        }
    }

    Ok(())
}

fn fetch_open_copilot_prs(repo_root: &Path) -> Result<Vec<PullRequestSummary>, String> {
    let value = gh_json(
        repo_root,
        &[
            "pr",
            "list",
            "--repo",
            REPO,
            "--state",
            "open",
            "--author",
            COPILOT_AUTHOR,
            "--limit",
            "1000",
            "--json",
            "number,title,isDraft,createdAt",
        ],
    )?;
    parse_open_copilot_prs(value)
}

fn fetch_timeline(repo_root: &Path, pr_number: u64) -> Result<Vec<TimelineEvent>, String> {
    let endpoint = format!("repos/{}/issues/{}/timeline", REPO, pr_number);
    let value = gh_json(
        repo_root,
        &[
            "api",
            &endpoint,
            "--paginate",
            "--slurp",
            "-H",
            "Accept: application/vnd.github+json",
        ],
    )?;
    parse_timeline_events(value, pr_number)
}

fn fetch_ci_status(repo_root: &Path, pr_number: u64) -> Result<CiStatus, String> {
    let value = gh_json_with_exit_codes(
        repo_root,
        &[
            "pr",
            "checks",
            &pr_number.to_string(),
            "--repo",
            REPO,
            "--json",
            "bucket",
        ],
        &[0, 8],
    )?;
    parse_ci_status(value, pr_number)
}

fn parse_open_copilot_prs(value: Value) -> Result<Vec<PullRequestSummary>, String> {
    let prs = value
        .as_array()
        .ok_or_else(|| "unexpected response for open PR list".to_string())?;

    let mut results = Vec::new();
    for pr in prs {
        let number = pr
            .get("number")
            .and_then(Value::as_u64)
            .ok_or_else(|| "copilot PR missing number".to_string())?;
        let title = pr
            .get("title")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("copilot PR #{} missing title", number))?
            .to_string();
        let is_draft = pr
            .get("isDraft")
            .and_then(Value::as_bool)
            .ok_or_else(|| format!("copilot PR #{} missing isDraft", number))?;
        let created_at_raw = pr
            .get("createdAt")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("copilot PR #{} missing createdAt", number))?;
        let created_at = parse_time(created_at_raw)
            .ok_or_else(|| format!("copilot PR #{} has invalid createdAt", number))?;

        results.push(PullRequestSummary {
            number,
            title,
            is_draft,
            created_at,
        });
    }

    Ok(results)
}

fn parse_timeline_events(value: Value, pr_number: u64) -> Result<Vec<TimelineEvent>, String> {
    let entries = flatten_paginated_items(value)?;
    let mut events = Vec::new();
    for entry in entries {
        let Some(event_name) = entry.get("event").and_then(Value::as_str) else {
            continue;
        };
        let event = match event_name {
            "copilot_work_started" => CopilotEventKind::WorkStarted,
            "copilot_work_finished" => CopilotEventKind::WorkFinished,
            _ => continue,
        };
        let created_at_raw = entry
            .get("created_at")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                format!(
                    "timeline event {} for PR #{} missing created_at",
                    event_name, pr_number
                )
            })?;
        let created_at = parse_time(created_at_raw).ok_or_else(|| {
            format!(
                "timeline event {} for PR #{} has invalid created_at",
                event_name, pr_number
            )
        })?;
        events.push(TimelineEvent { event, created_at });
    }
    Ok(events)
}

fn parse_ci_status(value: Value, pr_number: u64) -> Result<CiStatus, String> {
    let checks = value
        .as_array()
        .ok_or_else(|| format!("unexpected response for PR #{} checks", pr_number))?;
    if checks.is_empty() {
        return Ok(CiStatus::None);
    }

    let mut has_pass = false;
    let mut has_pending = false;
    let mut has_failure = false;

    for check in checks {
        let bucket = check
            .get("bucket")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("PR #{} check entry missing bucket", pr_number))?;
        match bucket {
            "pass" => has_pass = true,
            "pending" => has_pending = true,
            "fail" | "cancel" => has_failure = true,
            "skipping" => {}
            other => {
                return Err(format!(
                    "PR #{} check entry has unsupported bucket '{}'",
                    pr_number, other
                ))
            }
        }
    }

    if has_failure {
        Ok(CiStatus::Failure)
    } else if has_pending {
        Ok(CiStatus::Pending)
    } else if has_pass {
        Ok(CiStatus::Success)
    } else {
        Ok(CiStatus::None)
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
        _ => Err("expected array response from gh api".to_string()),
    }
}

fn classify_pr(
    pr: &PullRequestSummary,
    timeline: &[TimelineEvent],
    now: DateTime<Utc>,
) -> PrReadiness {
    let last_event = timeline.iter().max_by_key(|event| event.created_at);
    let age = non_negative_duration(now.signed_duration_since(pr.created_at));

    let (status, stale_reason) = match last_event.map(|event| event.event) {
        Some(CopilotEventKind::WorkFinished) => (PrStatus::Ready, None),
        Some(CopilotEventKind::WorkStarted) if age > Duration::hours(STALE_THRESHOLD_HOURS) => (
            PrStatus::Stale,
            Some(format!(
                "dispatched {} ago with no copilot_work_finished",
                format_age(age)
            )),
        ),
        Some(CopilotEventKind::WorkStarted) => (PrStatus::Working, None),
        None => (
            PrStatus::Stale,
            Some(format!(
                "dispatched {} ago with no copilot events",
                format_age(age)
            )),
        ),
    };

    PrReadiness {
        pr_number: pr.number,
        title: pr.title.clone(),
        status,
        last_event: last_event.map(|event| event.event.as_str().to_string()),
        last_event_at: last_event.map(|event| format_timestamp(event.created_at)),
        ci_status: None,
        is_draft: pr.is_draft,
        stale_reason,
    }
}

fn render_human_line(report: &PrReadiness) -> String {
    let event_summary = match (&report.last_event, &report.last_event_at) {
        (Some(event), Some(timestamp)) => format!("{} ({})", event, timestamp),
        _ => "no copilot events".to_string(),
    };
    let status_summary = match report.status {
        PrStatus::Ready => format!(
            "READY for review{}",
            report
                .ci_status
                .map(|status| format!(" (CI: {})", ci_status_label(status)))
                .unwrap_or_default()
        ),
        PrStatus::Working => "STILL WORKING".to_string(),
        PrStatus::Stale => format!(
            "STALE ({})",
            report
                .stale_reason
                .as_deref()
                .unwrap_or("dispatch needs attention")
        ),
    };
    format!(
        "PR {}#{}: {} — {}",
        REPO, report.pr_number, event_summary, status_summary
    )
}

fn format_timestamp(timestamp: DateTime<Utc>) -> String {
    timestamp.format("%Y-%m-%dT%H:%M:%SZ").to_string()
}

fn format_age(age: Duration) -> String {
    let hours = age.num_hours();
    if hours > 0 {
        format!("{}h", hours)
    } else {
        let minutes = age.num_minutes();
        format!("{}m", minutes.max(0))
    }
}

fn gh_json(repo_root: &Path, args: &[&str]) -> Result<Value, String> {
    gh_json_with_exit_codes(repo_root, args, &[0])
}

fn gh_json_with_exit_codes(
    repo_root: &Path,
    args: &[&str],
    accepted_exit_codes: &[i32],
) -> Result<Value, String> {
    let output = Command::new("gh")
        .current_dir(repo_root)
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute gh {}: {}", args.join(" "), error))?;

    let exit_code = output.status.code();
    let accepted = exit_code
        .map(|code| accepted_exit_codes.contains(&code))
        .unwrap_or(false);
    if !accepted {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!(
            "`gh {}` failed with status {}: {}",
            args.join(" "),
            exit_code
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

fn parse_time(raw: &str) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(raw)
        .ok()
        .map(|timestamp| timestamp.with_timezone(&Utc))
}

fn non_negative_duration(duration: Duration) -> Duration {
    if duration < Duration::zero() {
        Duration::zero()
    } else {
        duration
    }
}

fn ci_status_label(status: CiStatus) -> &'static str {
    match status {
        CiStatus::Success => "SUCCESS",
        CiStatus::Pending => "PENDING",
        CiStatus::Failure => "FAILURE",
        CiStatus::None => "NONE",
    }
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
    use serde_json::json;

    #[test]
    fn classify_ready_when_last_event_is_finished() {
        let pr = sample_pr("2026-03-08T20:00:00Z");
        let timeline = vec![
            timeline_event(CopilotEventKind::WorkStarted, "2026-03-08T20:05:00Z"),
            timeline_event(CopilotEventKind::WorkFinished, "2026-03-08T20:30:00Z"),
        ];

        let report = classify_pr(&pr, &timeline, parse_utc("2026-03-08T21:00:00Z"));

        assert_eq!(report.status, PrStatus::Ready);
        assert_eq!(report.last_event.as_deref(), Some("copilot_work_finished"));
        assert_eq!(report.stale_reason, None);
    }

    #[test]
    fn classify_working_when_started_recently_without_finish() {
        let pr = sample_pr("2026-03-08T20:00:00Z");
        let timeline = vec![timeline_event(
            CopilotEventKind::WorkStarted,
            "2026-03-08T20:10:00Z",
        )];

        let report = classify_pr(&pr, &timeline, parse_utc("2026-03-08T21:30:00Z"));

        assert_eq!(report.status, PrStatus::Working);
        assert_eq!(report.last_event.as_deref(), Some("copilot_work_started"));
        assert_eq!(report.stale_reason, None);
    }

    #[test]
    fn classify_stale_when_started_without_finish_past_threshold() {
        let pr = sample_pr("2026-03-08T20:00:00Z");
        let timeline = vec![timeline_event(
            CopilotEventKind::WorkStarted,
            "2026-03-08T20:10:00Z",
        )];

        let report = classify_pr(&pr, &timeline, parse_utc("2026-03-08T22:30:01Z"));

        assert_eq!(report.status, PrStatus::Stale);
        assert_eq!(report.last_event.as_deref(), Some("copilot_work_started"));
        assert_eq!(
            report.stale_reason.as_deref(),
            Some("dispatched 2h ago with no copilot_work_finished")
        );
    }

    #[test]
    fn classify_stale_when_no_copilot_events_exist() {
        let report = classify_pr(
            &sample_pr("2026-03-08T20:00:00Z"),
            &[],
            parse_utc("2026-03-08T20:30:00Z"),
        );

        assert_eq!(report.status, PrStatus::Stale);
        assert_eq!(report.last_event, None);
        assert_eq!(report.last_event_at, None);
        assert_eq!(
            report.stale_reason.as_deref(),
            Some("dispatched 30m ago with no copilot events")
        );
    }

    #[test]
    fn parse_ci_status_reports_success_when_all_buckets_pass() {
        let status = parse_ci_status(
            json!([
                {"bucket": "pass"},
                {"bucket": "pass"}
            ]),
            813,
        )
        .expect("ci status should parse");

        assert_eq!(status, CiStatus::Success);
    }

    #[test]
    fn parse_ci_status_reports_pending_when_any_check_is_pending() {
        let status = parse_ci_status(
            json!([
                {"bucket": "pass"},
                {"bucket": "pending"}
            ]),
            813,
        )
        .expect("ci status should parse");

        assert_eq!(status, CiStatus::Pending);
    }

    #[test]
    fn parse_ci_status_reports_failure_when_any_check_fails() {
        let status = parse_ci_status(
            json!([
                {"bucket": "pass"},
                {"bucket": "fail"}
            ]),
            813,
        )
        .expect("ci status should parse");

        assert_eq!(status, CiStatus::Failure);
    }

    fn sample_pr(created_at: &str) -> PullRequestSummary {
        PullRequestSummary {
            number: 813,
            title: "Add post-step tool".to_string(),
            is_draft: false,
            created_at: parse_utc(created_at),
        }
    }

    fn timeline_event(event: CopilotEventKind, created_at: &str) -> TimelineEvent {
        TimelineEvent {
            event,
            created_at: parse_utc(created_at),
        }
    }

    fn parse_utc(raw: &str) -> DateTime<Utc> {
        parse_time(raw).expect("timestamp should parse")
    }
}
