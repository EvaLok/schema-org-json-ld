use chrono::{DateTime, Utc};
use clap::Parser;
use serde::Serialize;
use serde_json::{json, Value};
use state_schema::{
    check_version, commit_state_json, read_state_value, set_value_at_pointer, update_freshness,
    write_state_value, AgentSession, StateJson,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const REPO: &str = "EvaLok/schema-org-json-ld";

#[derive(Parser, Debug)]
#[command(name = "verify-review-events")]
struct Cli {
    /// Path to repository root
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Update docs/state.json with the verified cycle marker
    #[arg(long)]
    apply: bool,

    /// Commit docs/state.json after applying the updated marker
    #[arg(long, requires = "apply")]
    commit: bool,

    /// Output report as JSON
    #[arg(long)]
    json: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
#[serde(rename_all = "snake_case")]
enum PrClassification {
    Code,
    Docs,
    Tool,
}

impl PrClassification {
    fn label(self) -> &'static str {
        match self {
            Self::Code => "code",
            Self::Docs => "docs-only",
            Self::Tool => "tooling",
        }
    }

    fn expected_reviews_label(self) -> &'static str {
        match self {
            Self::Code => "required",
            Self::Docs | Self::Tool => "none",
        }
    }

    fn expects_reviews(self) -> bool {
        matches!(self, Self::Code)
    }
}

#[derive(Debug, Clone, Serialize)]
struct PullRequestVerification {
    number: u64,
    title: String,
    cycle: u64,
    classification: PrClassification,
    review_count: usize,
    reviewers: Vec<String>,
    expected_reviews: bool,
    verified: bool,
}

#[derive(Debug, Serialize)]
struct VerificationReport {
    verified_through_cycle: u64,
    current_cycle: u64,
    checked_cycles: Vec<u64>,
    pull_requests: Vec<PullRequestVerification>,
    safe_to_advance_to: u64,
    all_prs_verified: bool,
    applied: bool,
    committed: bool,
    commit_sha: Option<String>,
}

#[derive(Debug, Clone)]
struct CycleWindow {
    cycle: u64,
    start: DateTime<Utc>,
    end: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone)]
struct GitCommit {
    committed_at: DateTime<Utc>,
    subject: String,
}

#[derive(Debug)]
struct SessionPrCandidate {
    number: u64,
    cycle: u64,
    details: PullRequestDetails,
}

#[derive(Debug, Serialize)]
struct PullRequestDetails {
    title: String,
    author_login: String,
    merged_at: Option<String>,
}

#[derive(Debug)]
struct ReviewData {
    count: usize,
    reviewers: Vec<String>,
}

fn main() {
    let cli = Cli::parse();
    let json_output = cli.json;
    match run(cli) {
        Ok((report, success)) => {
            if let Err(error) = print_report(&report, json_output) {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
            std::process::exit(if success { 0 } else { 1 });
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn run(cli: Cli) -> Result<(VerificationReport, bool), String> {
    validate_repo_root(&cli.repo_root)?;

    let state = read_state(&cli.repo_root)?;
    let current_cycle = last_cycle_number(&state)?;
    let verified_through_cycle = review_events_verified_through_cycle(&state)?;
    if verified_through_cycle > current_cycle {
        return Err(format!(
            "review_events_verified_through_cycle({}) exceeds last_cycle.number({})",
            verified_through_cycle, current_cycle
        ));
    }

    let checked_cycles = cycle_range(verified_through_cycle, current_cycle)?;
    let mut pull_requests = collect_pull_requests(
        &cli.repo_root,
        &state.agent_sessions,
        &checked_cycles,
        current_cycle,
    )?;
    pull_requests.sort_by_key(|pr| (pr.cycle, pr.number));
    let dispatch_cycles = collect_dispatch_cycles(&state.agent_sessions, &checked_cycles);

    let safe_to_advance_to = compute_safe_advance(
        verified_through_cycle,
        current_cycle,
        &checked_cycles,
        &pull_requests,
        &dispatch_cycles,
    );
    let all_prs_verified = safe_to_advance_to == current_cycle;

    let mut commit_sha = None;
    let mut applied = false;
    if cli.apply && safe_to_advance_to > verified_through_cycle {
        apply_verified_marker(&cli.repo_root, safe_to_advance_to)?;
        applied = true;
        if cli.commit {
            let message = verification_commit_message(safe_to_advance_to, current_cycle);
            commit_sha = Some(commit_state_json(&cli.repo_root, &message)?);
        }
    }

    Ok((
        VerificationReport {
            verified_through_cycle,
            current_cycle,
            checked_cycles,
            pull_requests,
            safe_to_advance_to,
            all_prs_verified,
            applied,
            committed: commit_sha.is_some(),
            commit_sha,
        },
        all_prs_verified,
    ))
}

fn validate_repo_root(repo_root: &Path) -> Result<(), String> {
    let state_path = repo_root.join("docs/state.json");
    if !state_path.exists() {
        return Err(format!(
            "--repo-root must point to the repository root containing {}",
            state_path.display()
        ));
    }

    Ok(())
}

fn read_state(repo_root: &Path) -> Result<StateJson, String> {
    let content = fs::read_to_string(repo_root.join("docs/state.json"))
        .map_err(|error| format!("failed to read docs/state.json: {}", error))?;
    let state: StateJson = serde_json::from_str(&content)
        .map_err(|error| format!("failed to parse docs/state.json: {}", error))?;
    check_version(&state)?;
    Ok(state)
}

fn last_cycle_number(state: &StateJson) -> Result<u64, String> {
    state
        .last_cycle
        .extra
        .get("number")
        .and_then(Value::as_u64)
        .ok_or_else(|| "missing numeric field: last_cycle.number".to_string())
}

fn review_events_verified_through_cycle(state: &StateJson) -> Result<u64, String> {
    state
        .extra
        .get("review_events_verified_through_cycle")
        .and_then(Value::as_u64)
        .ok_or_else(|| "missing numeric field: review_events_verified_through_cycle".to_string())
}

fn cycle_range(verified_through_cycle: u64, current_cycle: u64) -> Result<Vec<u64>, String> {
    if verified_through_cycle > current_cycle {
        return Err(format!(
            "verified cycle {} exceeds current cycle {}",
            verified_through_cycle, current_cycle
        ));
    }

    Ok(((verified_through_cycle + 1)..=current_cycle).collect())
}

fn collect_pull_requests(
    repo_root: &Path,
    agent_sessions: &[AgentSession],
    checked_cycles: &[u64],
    current_cycle: u64,
) -> Result<Vec<PullRequestVerification>, String> {
    if checked_cycles.is_empty() {
        return Ok(Vec::new());
    }

    let cycle_windows = build_cycle_windows(repo_root, checked_cycles, current_cycle)?;
    let mut candidates = Vec::new();
    let mut seen_prs = BTreeSet::new();

    for session in agent_sessions {
        if !is_merged_status(session.status.as_deref()) {
            continue;
        }

        let Some(number) = session_pr_number(session) else {
            continue;
        };

        if !seen_prs.insert(number) {
            continue;
        }

        let details = fetch_pull_request_details(repo_root, number)?;
        let Some(cycle) =
            infer_session_cycle(session, &cycle_windows, details.merged_at.as_deref())?
        else {
            continue;
        };
        if !checked_cycles.contains(&cycle) {
            continue;
        }
        candidates.push(SessionPrCandidate {
            number,
            cycle,
            details,
        });
    }

    let mut pull_requests = Vec::new();
    for candidate in candidates {
        let files = fetch_pull_request_files(repo_root, candidate.number)?;
        let classification = classify_pr_paths(&files);
        let review_data = fetch_pull_request_reviews(
            repo_root,
            candidate.number,
            &candidate.details.author_login,
            candidate.details.merged_at.as_deref(),
        )?;
        let verified = !classification.expects_reviews() || review_data.count > 0;

        pull_requests.push(PullRequestVerification {
            number: candidate.number,
            title: candidate.details.title,
            cycle: candidate.cycle,
            classification,
            review_count: review_data.count,
            reviewers: review_data.reviewers,
            expected_reviews: classification.expects_reviews(),
            verified,
        });
    }

    Ok(pull_requests)
}

fn collect_dispatch_cycles(
    agent_sessions: &[AgentSession],
    checked_cycles: &[u64],
) -> BTreeSet<u64> {
    let checked_cycle_set: BTreeSet<u64> = checked_cycles.iter().copied().collect();
    agent_sessions
        .iter()
        .filter_map(|session| session.extra.get("cycle").and_then(Value::as_u64))
        .filter(|cycle| checked_cycle_set.contains(cycle))
        .collect()
}

fn is_merged_status(status: Option<&str>) -> bool {
    matches!(status, Some("merged") | Some("reviewed_merged"))
}

fn build_cycle_windows(
    repo_root: &Path,
    checked_cycles: &[u64],
    current_cycle: u64,
) -> Result<Vec<CycleWindow>, String> {
    let commits = read_git_commits(repo_root)?;
    checked_cycles
        .iter()
        .copied()
        .map(|cycle| resolve_cycle_window(cycle, current_cycle, &commits))
        .collect()
}

fn resolve_cycle_window(
    target_cycle: u64,
    current_cycle: u64,
    commits: &[GitCommit],
) -> Result<CycleWindow, String> {
    let start = find_cycle_start_timestamp(commits, target_cycle).ok_or_else(|| {
        format!(
            "no cycle-start commit found for cycle {}. If this is a shallow clone, fetch full history with: git fetch --unshallow origin",
            target_cycle
        )
    })?;
    let end = if target_cycle == current_cycle {
        None
    } else {
        find_cycle_start_timestamp(commits, target_cycle + 1)
    };

    Ok(CycleWindow {
        cycle: target_cycle,
        start,
        end,
    })
}

fn read_git_commits(repo_root: &Path) -> Result<Vec<GitCommit>, String> {
    let output = git_command(
        repo_root,
        &[
            "log",
            "--date=iso-strict",
            "--pretty=format:%H%x09%cI%x09%s",
        ],
    )?;
    output
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(parse_git_commit_line)
        .collect()
}

fn git_command(repo_root: &Path, args: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute git {}: {}", args.join(" "), error))?;
    if !output.status.success() {
        return Err(command_failure_message(
            &format!("git {}", args.join(" ")),
            &output,
        ));
    }

    String::from_utf8(output.stdout).map_err(|error| {
        format!(
            "failed to decode git {} output as UTF-8: {}",
            args.join(" "),
            error
        )
    })
}

fn parse_git_commit_line(line: &str) -> Result<GitCommit, String> {
    let mut parts = line.splitn(3, '\t');
    let _sha = parts
        .next()
        .ok_or_else(|| format!("invalid git log line (missing sha): {}", line))?;
    let committed_at = parts
        .next()
        .ok_or_else(|| format!("invalid git log line (missing timestamp): {}", line))?;
    let subject = parts
        .next()
        .ok_or_else(|| format!("invalid git log line (missing subject): {}", line))?;

    Ok(GitCommit {
        committed_at: parse_timestamp(committed_at, "git commit timestamp")?,
        subject: subject.to_string(),
    })
}

fn infer_session_cycle(
    session: &AgentSession,
    cycle_windows: &[CycleWindow],
    pull_request_merged_at: Option<&str>,
) -> Result<Option<u64>, String> {
    if let Some(cycle) = session.extra.get("cycle").and_then(Value::as_u64) {
        return Ok(Some(cycle));
    }

    let merged_at_raw = session_merged_at(session)
        .or(pull_request_merged_at)
        .ok_or_else(|| {
            merged_session_label(
                session,
                "cannot determine cycle because merged_at is missing in both the agent session and PR API response",
            )
        })?;
    let merged_at = parse_timestamp(merged_at_raw, "agent_sessions[].merged_at")?;

    Ok(cycle_windows
        .iter()
        .find(|window| timestamp_in_window(merged_at, window))
        .map(|window| window.cycle))
}

fn merged_session_label(session: &AgentSession, suffix: &str) -> String {
    match (session_pr_number(session), session.title.as_deref()) {
        (Some(pr), Some(title)) => {
            format!("merged agent session for PR #{} ({}) {}", pr, title, suffix)
        }
        (Some(pr), None) => format!("merged agent session for PR #{} {}", pr, suffix),
        (None, Some(title)) => format!("merged agent session '{}' {}", title, suffix),
        (None, None) => format!("merged agent session {}", suffix),
    }
}

fn session_pr_number(session: &AgentSession) -> Option<u64> {
    session
        .pr
        .and_then(|value| u64::try_from(value).ok())
        .or_else(|| session.extra.get("merged_pr").and_then(Value::as_u64))
}

fn session_merged_at(session: &AgentSession) -> Option<&str> {
    session
        .merged_at
        .as_deref()
        .or_else(|| session.extra.get("pr_merged_at").and_then(Value::as_str))
}

fn timestamp_in_window(timestamp: DateTime<Utc>, window: &CycleWindow) -> bool {
    if timestamp < window.start {
        return false;
    }

    match window.end {
        Some(end) => timestamp < end,
        None => true,
    }
}

fn find_cycle_start_timestamp(commits: &[GitCommit], cycle: u64) -> Option<DateTime<Utc>> {
    commits.iter().find_map(|commit| {
        if extract_step(&commit.subject).as_deref() == Some("cycle-start")
            && extract_cycle_tag(&commit.subject) == Some(cycle)
        {
            return Some(commit.committed_at);
        }
        None
    })
}

fn extract_step(subject: &str) -> Option<String> {
    let prefix = "state(";
    let suffix = "):";
    let remainder = subject.strip_prefix(prefix)?;
    let end = remainder.find(suffix)?;
    Some(remainder[..end].to_string())
}

fn extract_cycle_tag(subject: &str) -> Option<u64> {
    let marker = "[cycle ";
    let start = subject.find(marker)?;
    let remainder = &subject[start + marker.len()..];
    let end = remainder.find(']')?;
    remainder[..end].trim().parse::<u64>().ok()
}

fn parse_timestamp(value: &str, label: &str) -> Result<DateTime<Utc>, String> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&Utc))
        .map_err(|error| format!("invalid {}: {}", label, error))
}

fn fetch_pull_request_details(
    repo_root: &Path,
    pr_number: u64,
) -> Result<PullRequestDetails, String> {
    let path = format!("repos/{}/pulls/{}", REPO, pr_number);
    let value = gh_json(repo_root, &["api".to_string(), path])?;
    let title = value
        .get("title")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("PR #{} response missing title", pr_number))?;
    let author_login = value
        .pointer("/user/login")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("PR #{} response missing user.login", pr_number))?;

    Ok(PullRequestDetails {
        title: title.to_string(),
        author_login: author_login.to_string(),
        merged_at: value
            .get("merged_at")
            .and_then(Value::as_str)
            .map(|value| value.to_string()),
    })
}

fn fetch_pull_request_files(repo_root: &Path, pr_number: u64) -> Result<Vec<String>, String> {
    let path = format!("repos/{}/pulls/{}/files", REPO, pr_number);
    let value = gh_json(
        repo_root,
        &[
            "api".to_string(),
            path,
            "--paginate".to_string(),
            "--slurp".to_string(),
        ],
    )?;
    let entries = flatten_paginated_array(value, &format!("PR #{} files", pr_number))?;
    entries
        .iter()
        .map(|entry| {
            entry
                .get("filename")
                .and_then(Value::as_str)
                .map(|value| value.to_string())
                .ok_or_else(|| format!("PR #{} file entry missing filename", pr_number))
        })
        .collect()
}

fn fetch_pull_request_reviews(
    repo_root: &Path,
    pr_number: u64,
    pr_author: &str,
    pr_merged_at: Option<&str>,
) -> Result<ReviewData, String> {
    let path = format!("repos/{}/pulls/{}/reviews", REPO, pr_number);
    let value = gh_json(
        repo_root,
        &[
            "api".to_string(),
            path,
            "--paginate".to_string(),
            "--slurp".to_string(),
        ],
    )?;
    let entries = flatten_paginated_array(value, &format!("PR #{} reviews", pr_number))?;
    review_data_from_entries(
        &entries,
        pr_author,
        pr_merged_at.ok_or_else(|| format!("PR #{} response missing merged_at", pr_number))?,
    )
}

fn review_data_from_entries(
    entries: &[Value],
    pr_author: &str,
    pr_merged_at: &str,
) -> Result<ReviewData, String> {
    let merged_at = parse_timestamp(pr_merged_at, "pull request merged_at")?;
    let mut reviewers = BTreeSet::new();
    let mut count = 0;

    for (index, entry) in entries.iter().enumerate() {
        let state = entry
            .get("state")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("review entry {} missing state", index + 1))?;
        if state != "APPROVED" {
            continue;
        }

        let reviewer = entry
            .pointer("/user/login")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("review entry {} missing user.login", index + 1))?;
        if reviewer == pr_author {
            continue;
        }

        let submitted_at_raw = entry
            .get("submitted_at")
            .and_then(Value::as_str)
            .ok_or_else(|| format!("review entry {} missing submitted_at", index + 1))?;
        let submitted_at = parse_timestamp(submitted_at_raw, "review submitted_at")?;
        if submitted_at > merged_at {
            continue;
        }

        count += 1;
        reviewers.insert(reviewer.to_string());
    }

    Ok(ReviewData {
        count,
        reviewers: reviewers.into_iter().collect(),
    })
}

fn gh_json(repo_root: &Path, args: &[String]) -> Result<Value, String> {
    let output = Command::new("gh")
        .current_dir(repo_root)
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute gh {}: {}", args.join(" "), error))?;
    if !output.status.success() {
        return Err(command_failure_message(
            &format!("gh {}", args.join(" ")),
            &output,
        ));
    }

    serde_json::from_slice(&output.stdout).map_err(|error| {
        format!(
            "failed to parse gh {} output as JSON: {}",
            args.join(" "),
            error
        )
    })
}

fn flatten_paginated_array(value: Value, label: &str) -> Result<Vec<Value>, String> {
    let pages = value
        .as_array()
        .ok_or_else(|| format!("unexpected {} response: expected array", label))?;
    if pages.iter().all(Value::is_array) {
        let mut flattened = Vec::new();
        for (index, page) in pages.iter().enumerate() {
            let page_entries = page.as_array().ok_or_else(|| {
                format!("unexpected {} page {}: expected array", label, index + 1)
            })?;
            flattened.extend(page_entries.iter().cloned());
        }
        Ok(flattened)
    } else {
        Ok(pages.to_vec())
    }
}

fn classify_pr_paths(paths: &[String]) -> PrClassification {
    if paths.iter().any(|path| is_code_path(path)) {
        return PrClassification::Code;
    }

    if paths.iter().all(|path| is_docs_path(path)) {
        return PrClassification::Docs;
    }

    PrClassification::Tool
}

fn is_code_path(path: &str) -> bool {
    path.starts_with("php/src/") || path.starts_with("ts/src/")
}

fn is_docs_path(path: &str) -> bool {
    path == "README.md"
        || path.starts_with("docs/")
        || path.starts_with("reviews/")
        || path.ends_with(".md")
        || path.ends_with(".markdown")
        || path.ends_with(".mdx")
}

fn compute_safe_advance(
    verified_through_cycle: u64,
    current_cycle: u64,
    checked_cycles: &[u64],
    pull_requests: &[PullRequestVerification],
    dispatch_cycles: &BTreeSet<u64>,
) -> u64 {
    if checked_cycles.is_empty() {
        return verified_through_cycle.min(current_cycle);
    }

    let prs_by_cycle = group_pull_requests_by_cycle(pull_requests);
    let mut safe = verified_through_cycle;

    for cycle in checked_cycles {
        let verified = prs_by_cycle
            .get(cycle)
            .map(|prs| prs.iter().all(|pr| pr.verified))
            .unwrap_or_else(|| !dispatch_cycles.contains(cycle));
        if !verified {
            break;
        }
        safe = *cycle;
    }

    safe
}

fn group_pull_requests_by_cycle(
    pull_requests: &[PullRequestVerification],
) -> BTreeMap<u64, Vec<&PullRequestVerification>> {
    let mut grouped: BTreeMap<u64, Vec<&PullRequestVerification>> = BTreeMap::new();
    for pull_request in pull_requests {
        grouped
            .entry(pull_request.cycle)
            .or_default()
            .push(pull_request);
    }
    grouped
}

fn apply_verified_marker(repo_root: &Path, verified_cycle: u64) -> Result<(), String> {
    let mut state = read_state_value(repo_root)?;
    let changed = set_value_at_pointer(
        &mut state,
        "/review_events_verified_through_cycle",
        json!(verified_cycle),
    )?;
    if !changed {
        return Ok(());
    }

    let cycle_u32 = u32::try_from(verified_cycle).map_err(|_| {
        format!(
            "cycle number too large: {} (maximum supported value is {})",
            verified_cycle,
            u32::MAX,
        )
    })?;
    update_freshness(
        &mut state,
        "review_events_verified_through_cycle",
        cycle_u32,
    )?;
    write_state_value(repo_root, &state)?;
    Ok(())
}

fn verification_commit_message(verified_cycle: u64, current_cycle: u64) -> String {
    format!(
        "state(verify-review-events): verified review events through cycle {} [cycle {}]",
        verified_cycle, current_cycle
    )
}

fn print_report(report: &VerificationReport, json_output: bool) -> Result<(), String> {
    if json_output {
        let rendered = serde_json::to_string_pretty(report)
            .map_err(|error| format!("failed to serialize JSON report: {}", error))?;
        println!("{}", rendered);
        return Ok(());
    }

    println!("Review Events Verification\n");
    println!(
        "  Verified through cycle: {} (current marker)",
        report.verified_through_cycle
    );
    if report.checked_cycles.is_empty() {
        println!("  No cycles to check.\n");
    } else {
        println!(
            "  Checking cycles {}...\n",
            format_cycle_range(&report.checked_cycles)
        );
    }

    for pr in &report.pull_requests {
        let reviewers = if pr.reviewers.is_empty() {
            "none".to_string()
        } else {
            pr.reviewers.join(", ")
        };
        println!(
            "  PR {REPO}#{} (cycle {}): {} -- {} reviews (expected: {}) -- {} [reviewers: {}]",
            pr.number,
            pr.cycle,
            pr.classification.label(),
            pr.review_count,
            pr.classification.expected_reviews_label(),
            pr.title,
            reviewers
        );
    }

    if !report.pull_requests.is_empty() {
        println!();
    }

    if report.safe_to_advance_to == report.current_cycle {
        println!(
            "  Result: All {} PRs verified. Safe to advance marker to {}.",
            report.pull_requests.len(),
            report.safe_to_advance_to
        );
    } else {
        let failed_cycle = first_unverified_cycle(report).unwrap_or(report.safe_to_advance_to);
        println!(
            "  Result: Verification failed for cycle {}. Marker stays at {}.",
            failed_cycle, report.safe_to_advance_to
        );
    }

    if report.applied {
        println!(
            "  Applied: updated docs/state.json to review_events_verified_through_cycle = {}.",
            report.safe_to_advance_to
        );
    }
    if let Some(commit_sha) = &report.commit_sha {
        println!("  Committed: {}.", commit_sha);
    }

    Ok(())
}

fn format_cycle_range(checked_cycles: &[u64]) -> String {
    match (checked_cycles.first(), checked_cycles.last()) {
        (Some(start), Some(end)) if start == end => start.to_string(),
        (Some(start), Some(end)) => format!("{}-{}", start, end),
        _ => "none".to_string(),
    }
}

fn first_unverified_cycle(report: &VerificationReport) -> Option<u64> {
    report
        .checked_cycles
        .iter()
        .copied()
        .find(|cycle| *cycle > report.safe_to_advance_to)
}

fn command_failure_message(command: &str, output: &Output) -> String {
    let code = output.status.code().map_or_else(
        || "terminated by signal".to_string(),
        |value| value.to_string(),
    );
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();

    if stderr.is_empty() {
        format!("{command} failed with status {code}")
    } else {
        format!("{command} failed with status {code}: {stderr}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TempDir {
        path: PathBuf,
    }

    impl TempDir {
        fn new(prefix: &str) -> Self {
            let unique = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "schema-org-json-ld-{prefix}-{}-{unique}",
                std::process::id()
            ));
            fs::create_dir_all(&path).unwrap();
            Self { path }
        }
    }

    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn write_file(path: &Path, content: &str) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, content).unwrap();
    }

    fn sample_pr(
        number: u64,
        cycle: u64,
        classification: PrClassification,
        review_count: usize,
    ) -> PullRequestVerification {
        PullRequestVerification {
            number,
            title: format!("PR {}", number),
            cycle,
            classification,
            review_count,
            reviewers: Vec::new(),
            expected_reviews: classification.expects_reviews(),
            verified: !classification.expects_reviews() || review_count > 0,
        }
    }

    fn sample_review(state: &str, reviewer: &str, submitted_at: &str) -> Value {
        json!({
            "state": state,
            "user": {
                "login": reviewer,
            },
            "submitted_at": submitted_at,
        })
    }

    fn sample_session(cycle: u64) -> AgentSession {
        let mut session = AgentSession::default();
        session.extra.insert("cycle".to_string(), json!(cycle));
        session
    }

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--repo-root"));
        assert!(help.contains("--apply"));
        assert!(help.contains("--commit"));
        assert!(help.contains("--json"));
    }

    #[test]
    fn classify_pr_paths_marks_code_when_php_or_ts_source_changes() {
        let classification = classify_pr_paths(&[
            "php/src/v1/Schema/Product.php".to_string(),
            "docs/worklog/entry.md".to_string(),
        ]);
        assert_eq!(classification, PrClassification::Code);
    }

    #[test]
    fn classify_pr_paths_marks_docs_for_docs_and_state_only() {
        let classification = classify_pr_paths(&[
            "docs/state.json".to_string(),
            "docs/worklog/2026-03-15/entry.md".to_string(),
            "README.md".to_string(),
        ]);
        assert_eq!(classification, PrClassification::Docs);
    }

    #[test]
    fn classify_pr_paths_marks_tool_for_tooling_and_infrastructure_changes() {
        let classification = classify_pr_paths(&[
            "tools/rust/crates/process-review/src/main.rs".to_string(),
            ".github/workflows/main.yml".to_string(),
        ]);
        assert_eq!(classification, PrClassification::Tool);
    }

    #[test]
    fn cycle_range_uses_verified_marker_and_current_cycle() {
        assert_eq!(cycle_range(265, 267).unwrap(), vec![266, 267]);
        assert!(cycle_range(268, 267).is_err());
    }

    #[test]
    fn review_data_filters_to_approved_reviews_only() {
        let comment_only = review_data_from_entries(
            &[sample_review(
                "COMMENTED",
                "reviewer",
                "2026-03-15T00:30:00Z",
            )],
            "author",
            "2026-03-15T01:00:00Z",
        )
        .unwrap();
        assert_eq!(comment_only.count, 0);
        assert!(comment_only.reviewers.is_empty());

        let approved = review_data_from_entries(
            &[sample_review(
                "APPROVED",
                "reviewer",
                "2026-03-15T00:30:00Z",
            )],
            "author",
            "2026-03-15T01:00:00Z",
        )
        .unwrap();
        assert_eq!(approved.count, 1);
        assert_eq!(approved.reviewers, vec!["reviewer".to_string()]);
    }

    #[test]
    fn review_data_rejects_self_reviews() {
        let review_data = review_data_from_entries(
            &[sample_review("APPROVED", "author", "2026-03-15T00:30:00Z")],
            "author",
            "2026-03-15T01:00:00Z",
        )
        .unwrap();
        assert_eq!(review_data.count, 0);
        assert!(review_data.reviewers.is_empty());
    }

    #[test]
    fn review_data_rejects_post_merge_reviews() {
        let review_data = review_data_from_entries(
            &[sample_review(
                "APPROVED",
                "reviewer",
                "2026-03-15T01:30:00Z",
            )],
            "author",
            "2026-03-15T01:00:00Z",
        )
        .unwrap();
        assert_eq!(review_data.count, 0);
        assert!(review_data.reviewers.is_empty());
    }

    #[test]
    fn compute_safe_advance_handles_docs_only_and_stops_on_unreviewed_code_prs() {
        let checked_cycles = vec![266, 267];
        let pull_requests = vec![
            sample_pr(1284, 266, PrClassification::Docs, 0),
            sample_pr(1288, 267, PrClassification::Code, 0),
        ];

        assert_eq!(
            compute_safe_advance(265, 267, &checked_cycles, &pull_requests, &BTreeSet::new()),
            266
        );
    }

    #[test]
    fn compute_safe_advance_advances_when_no_prs_are_in_range() {
        assert_eq!(
            compute_safe_advance(265, 267, &[266, 267], &[], &BTreeSet::new()),
            267
        );
    }

    #[test]
    fn compute_safe_advance_fails_closed_when_cycle_has_dispatches_but_no_prs() {
        let checked_cycles = vec![266, 267];
        let pull_requests = vec![sample_pr(1284, 266, PrClassification::Docs, 0)];
        let dispatch_cycles = collect_dispatch_cycles(&[sample_session(267)], &checked_cycles);

        assert_eq!(
            compute_safe_advance(265, 267, &checked_cycles, &pull_requests, &dispatch_cycles),
            266
        );
    }

    #[test]
    fn compute_safe_advance_advances_for_legitimate_no_pr_cycles() {
        let checked_cycles = vec![266, 267];

        assert_eq!(
            compute_safe_advance(265, 267, &checked_cycles, &[], &BTreeSet::new()),
            267
        );
    }

    #[test]
    fn apply_verified_marker_updates_marker_and_field_inventory() {
        let repo_root = TempDir::new("verify-review-events-apply");
        write_file(
            &repo_root.path.join("docs/state.json"),
            r#"{
  "schema_version": 1,
  "last_cycle": {
    "number": 267
  },
  "cycle_phase": {
    "cycle": 267,
    "phase": "work",
    "phase_entered_at": "2026-03-15T00:00:00Z"
  },
  "field_inventory": {
    "fields": {
      "review_events_verified_through_cycle": {
        "last_refreshed": "cycle 266"
      }
    }
  },
  "agent_sessions": [],
  "copilot_metrics": {},
  "review_events_verified_through_cycle": 266
}
"#,
        );

        apply_verified_marker(&repo_root.path, 267).unwrap();

        let updated: Value = serde_json::from_str(
            &fs::read_to_string(repo_root.path.join("docs/state.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(
            updated
                .get("review_events_verified_through_cycle")
                .and_then(Value::as_u64),
            Some(267)
        );
        assert_eq!(
            updated
                .pointer(
                    "/field_inventory/fields/review_events_verified_through_cycle/last_refreshed"
                )
                .and_then(Value::as_str),
            Some("cycle 267")
        );
    }
}
