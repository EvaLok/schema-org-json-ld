use clap::Parser;
use serde::Deserialize;
use serde_json::Value;
use state_schema::{read_state_value, write_state_value, AgentSession, StateJson};
use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const REPO: &str = "EvaLok/schema-org-json-ld";

#[derive(Parser, Debug)]
#[command(name = "backfill-sessions")]
struct Cli {
    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Print planned changes without writing docs/state.json
    #[arg(long)]
    dry_run: bool,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct IssueRecord {
    number: u64,
    title: String,
    created_at: String,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    pull_request: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct PullRequestRecord {
    number: u64,
    title: String,
    state: String,
    #[serde(default)]
    body: Option<String>,
    #[serde(default)]
    merged_at: Option<String>,
    head: PullRequestHead,
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
struct PullRequestHead {
    #[serde(rename = "ref")]
    ref_name: String,
}

#[derive(Debug)]
struct BackfillPlan {
    already_tracked: usize,
    new_sessions: Vec<AgentSession>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum MatchStrength {
    Branch = 1,
    BodyCloses = 2,
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

    let state = read_state_json(&cli.repo_root)?;
    let issues = fetch_closed_agent_task_issues()?;
    let prs = fetch_pull_requests()?;
    let plan = build_backfill_plan(&state.agent_sessions, &issues, &prs)?;
    let new_count = plan.new_sessions.len();
    let already_tracked = plan.already_tracked;
    let total_closed_issues = plan.already_tracked + plan.new_sessions.len();

    if cli.dry_run {
        for session in &plan.new_sessions {
            println!("Would add {}", describe_session(session));
        }
        println!(
            "Dry run: {} new sessions would be added, {} already tracked, {} total closed agent-task issues.",
            plan.new_sessions.len(),
            plan.already_tracked,
            total_closed_issues
        );
        return Ok(());
    }

    let mut updated_sessions = state.agent_sessions;
    updated_sessions.extend(plan.new_sessions);
    updated_sessions.sort_by_key(|session| session.issue.unwrap_or(i64::MAX));

    let mut state_value = read_state_value(&cli.repo_root)?;
    let sessions_value = serde_json::to_value(&updated_sessions)
        .map_err(|error| format!("failed to serialize agent_sessions: {}", error))?;
    let sessions = state_value
        .pointer_mut("/agent_sessions")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| "missing array /agent_sessions in docs/state.json".to_string())?;
    *sessions = sessions_value
        .as_array()
        .cloned()
        .ok_or_else(|| "serialized agent_sessions must be an array".to_string())?;
    write_state_value(&cli.repo_root, &state_value)?;

    println!(
        "Backfill complete: {} new sessions added, {} already tracked, {} total closed agent-task issues.",
        new_count,
        already_tracked,
        total_closed_issues
    );

    Ok(())
}

fn build_backfill_plan(
    existing_sessions: &[AgentSession],
    issues: &[IssueRecord],
    prs: &[PullRequestRecord],
) -> Result<BackfillPlan, String> {
    let tracked: HashSet<u64> = existing_sessions
        .iter()
        .filter_map(|session| session.issue)
        .filter_map(|issue| u64::try_from(issue).ok())
        .collect();

    let already_tracked = issues
        .iter()
        .filter(|issue| tracked.contains(&issue.number))
        .count();
    let mut new_sessions = issues
        .iter()
        .filter(|issue| issue.pull_request.is_none())
        .filter(|issue| !tracked.contains(&issue.number))
        .map(|issue| build_agent_session(issue, prs))
        .collect::<Result<Vec<_>, _>>()?;
    new_sessions.sort_by_key(|session| session.issue.unwrap_or(i64::MAX));

    Ok(BackfillPlan {
        already_tracked,
        new_sessions,
    })
}

fn build_agent_session(
    issue: &IssueRecord,
    prs: &[PullRequestRecord],
) -> Result<AgentSession, String> {
    let issue_number =
        i64::try_from(issue.number).map_err(|_| format!("issue #{} does not fit in i64", issue.number))?;

    let linked_pr = find_linked_pr(issue, prs);
    let (status, pr, merged_at) = match linked_pr {
        Some(pr) if pr.merged_at.is_some() => (
            "merged".to_string(),
            Some(
                i64::try_from(pr.number)
                    .map_err(|_| format!("PR #{} does not fit in i64", pr.number))?,
            ),
            pr.merged_at.clone(),
        ),
        Some(pr) if pr.state == "closed" => (
            "closed".to_string(),
            Some(
                i64::try_from(pr.number)
                    .map_err(|_| format!("PR #{} does not fit in i64", pr.number))?,
            ),
            None,
        ),
        _ => ("failed".to_string(), None, None),
    };

    Ok(AgentSession {
        issue: Some(issue_number),
        title: Some(issue.title.clone()),
        dispatched_at: Some(issue.created_at.clone()),
        model: Some("unknown".to_string()),
        status: Some(status),
        pr,
        merged_at,
        extra: BTreeMap::new(),
    })
}

fn find_linked_pr<'a>(
    issue: &IssueRecord,
    prs: &'a [PullRequestRecord],
) -> Option<&'a PullRequestRecord> {
    prs.iter()
        .filter_map(|pr| {
            let match_strength = if pr
                .body
                .as_deref()
                .is_some_and(|body| pr_body_closes_issue(body, issue.number))
            {
                Some(MatchStrength::BodyCloses)
            } else if branch_matches_issue(&pr.head.ref_name, &issue.title) {
                Some(MatchStrength::Branch)
            } else {
                None
            }?;

            let status_rank = if pr.merged_at.is_some() {
                2_u8
            } else if pr.state == "closed" {
                1_u8
            } else {
                0_u8
            };

            Some((match_strength, status_rank, proximity_to_issue(issue.number, pr.number), pr))
        })
        .max_by(|left, right| {
            left.0
                .cmp(&right.0)
                .then(left.1.cmp(&right.1))
                .then(right.2.cmp(&left.2))
        })
        .map(|(_, _, _, pr)| pr)
}

fn proximity_to_issue(issue_number: u64, pr_number: u64) -> u64 {
    pr_number.abs_diff(issue_number)
}

fn pr_body_closes_issue(body: &str, issue_number: u64) -> bool {
    let lower = body.to_lowercase();
    let refs = [
        format!("#{}", issue_number),
        format!("evalok/schema-org-json-ld#{}", issue_number),
        format!("/issues/{}", issue_number),
    ];

    [
        "close ", "closes ", "closed ", "fix ", "fixes ", "fixed ", "resolve ", "resolves ",
        "resolved ",
    ]
    .iter()
    .any(|keyword| {
        let mut search = lower.as_str();
        while let Some(position) = search.find(keyword) {
            let rest = &search[position + keyword.len()..];
            if refs.iter().any(|value| contains_tag_at_word_boundary(rest, value)) {
                return true;
            }
            search = rest;
        }
        false
    })
}

fn branch_matches_issue(branch: &str, issue_title: &str) -> bool {
    let branch_slug = slugify(branch.rsplit('/').next().unwrap_or(branch));
    title_branch_candidates(issue_title).into_iter().any(|candidate| {
        candidate.len() >= 8
            && (branch_slug.contains(&candidate) || candidate.contains(&branch_slug))
    })
}

fn title_branch_candidates(title: &str) -> Vec<String> {
    let full = slugify(title);
    let mut candidates = vec![full.clone()];

    if let Some(stripped) = strip_phase_prefix(&full) {
        candidates.push(stripped);
    }

    if let Some(trimmed) = trim_stop_word_prefix(&full) {
        candidates.push(trimmed);
    }

    candidates.sort();
    candidates.dedup();
    candidates
}

fn strip_phase_prefix(slug: &str) -> Option<String> {
    let mut parts = slug.split('-');
    let first = parts.next()?;
    let second = parts.next()?;
    if first == "phase" && second.chars().any(|ch| ch.is_ascii_digit()) {
        let remainder: Vec<&str> = parts.collect();
        if remainder.is_empty() {
            None
        } else {
            Some(remainder.join("-"))
        }
    } else {
        None
    }
}

fn trim_stop_word_prefix(slug: &str) -> Option<String> {
    let stop_words = [
        "add", "build", "create", "extend", "fix", "implement", "keep", "port", "refactor",
        "standardize", "update",
    ];
    for word in stop_words {
        let prefix = format!("{}-", word);
        if let Some(stripped) = slug.strip_prefix(&prefix) {
            return Some(stripped.to_string());
        }
    }
    None
}

fn slugify(text: &str) -> String {
    let mut slug = String::new();
    let mut last_was_dash = false;
    for character in text.chars() {
        if character.is_ascii_alphanumeric() {
            slug.push(character.to_ascii_lowercase());
            last_was_dash = false;
        } else if !last_was_dash && !slug.is_empty() {
            slug.push('-');
            last_was_dash = true;
        }
    }
    while slug.ends_with('-') {
        slug.pop();
    }
    slug
}

fn contains_tag_at_word_boundary(text: &str, tag: &str) -> bool {
    let mut search = text;
    while let Some(position) = search.find(tag) {
        let after = position + tag.len();
        let next_character = search[after..].chars().next();
        if next_character.is_none_or(|character| !character.is_ascii_digit()) {
            return true;
        }
        search = &search[after..];
    }
    false
}

fn flatten_paginated_items(value: Value) -> Result<Vec<Value>, String> {
    match value {
        Value::Array(items) => {
            if items.iter().all(Value::is_object) {
                return Ok(items);
            }

            if items.iter().all(Value::is_array) {
                let flattened = items
                    .into_iter()
                    .flat_map(|page| page.as_array().cloned().unwrap_or_default())
                    .collect();
                return Ok(flattened);
            }

            Err("unexpected paginated response shape".to_string())
        }
        _ => Err("expected array response from gh api".to_string()),
    }
}

fn fetch_closed_agent_task_issues() -> Result<Vec<IssueRecord>, String> {
    let endpoint = format!("repos/{}/issues?labels=agent-task&state=closed&per_page=100", REPO);
    let value = gh_json(&["api", &endpoint, "--paginate", "--slurp"])?;
    let items = flatten_paginated_items(value)?;

    items.into_iter()
        .map(|item| {
            serde_json::from_value::<IssueRecord>(item)
                .map_err(|error| format!("failed to parse issue response: {}", error))
        })
        .filter(|result| match result {
            Ok(issue) => issue.pull_request.is_none(),
            Err(_) => true,
        })
        .collect()
}

fn fetch_pull_requests() -> Result<Vec<PullRequestRecord>, String> {
    let endpoint = format!("repos/{}/pulls?state=all&per_page=100", REPO);
    let value = gh_json(&["api", &endpoint, "--paginate", "--slurp"])?;
    let items = flatten_paginated_items(value)?;

    items.into_iter()
        .map(|item| {
            serde_json::from_value::<PullRequestRecord>(item)
                .map_err(|error| format!("failed to parse pull request response: {}", error))
        })
        .collect()
}

fn gh_json(args: &[&str]) -> Result<Value, String> {
    let output = Command::new("gh")
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute gh {}: {}", args.join(" "), error))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!(
            "`gh {}` failed with status {}: {}",
            args.join(" "),
            output
                .status
                .code()
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

fn validate_repo_root(repo_root: &Path) -> Result<(), String> {
    let state_path = state_path(repo_root);
    if !state_path.exists() {
        return Err(format!(
            "--repo-root must point to the repository root containing {}",
            state_path.display()
        ));
    }
    Ok(())
}

fn read_state_json(repo_root: &Path) -> Result<StateJson, String> {
    let path = state_path(repo_root);
    let content =
        fs::read_to_string(&path).map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    serde_json::from_str::<StateJson>(&content)
        .map_err(|error| format!("failed to parse {}: {}", path.display(), error))
}

fn state_path(repo_root: &Path) -> PathBuf {
    repo_root.join("docs/state.json")
}

fn describe_session(session: &AgentSession) -> String {
    let issue = session
        .issue
        .map(|value| format!("#{}", value))
        .unwrap_or_else(|| "<unknown issue>".to_string());
    let status = session.status.as_deref().unwrap_or("unknown");
    let title = session.title.as_deref().unwrap_or("<untitled>");

    match (session.pr, session.merged_at.as_deref()) {
        (Some(pr), Some(merged_at)) => {
            format!("{} [{}] {} -> PR #{} merged at {}", issue, status, title, pr, merged_at)
        }
        (Some(pr), None) => format!("{} [{}] {} -> PR #{}", issue, status, title, pr),
        (None, _) => format!("{} [{}] {}", issue, status, title),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn flatten_paginated_items_accepts_slurped_pages() {
        let pages = json!([
            [
                {"number": 1, "title": "one", "created_at": "2026-03-01T00:00:00Z"}
            ],
            [
                {"number": 2, "title": "two", "created_at": "2026-03-02T00:00:00Z"}
            ]
        ]);

        let items = flatten_paginated_items(pages).expect("pages should flatten");
        assert_eq!(items.len(), 2);
        assert_eq!(items[0]["number"], json!(1));
        assert_eq!(items[1]["number"], json!(2));
    }

    #[test]
    fn pr_body_closes_issue_matches_issue_references() {
        assert!(pr_body_closes_issue("This change closes #680.", 680));
        assert!(pr_body_closes_issue(
            "Fixes EvaLok/schema-org-json-ld#303 as requested.",
            303
        ));
        assert!(!pr_body_closes_issue("References #680 but does not close it.", 680));
    }

    #[test]
    fn branch_matches_issue_uses_title_slug_candidates() {
        assert!(branch_matches_issue(
            "copilot/build-agent-sessions-backfill-tool",
            "Build agent_sessions backfill tool for historical ledger",
        ));
        assert!(branch_matches_issue(
            "copilot/add-npm-publish-workflow",
            "Phase 4b: npm publish workflow",
        ));
        assert!(!branch_matches_issue(
            "copilot/fix-unrelated-thing",
            "Build agent_sessions backfill tool for historical ledger",
        ));
    }

    #[test]
    fn build_backfill_plan_deduplicates_and_reconstructs_statuses() {
        let existing_sessions = vec![AgentSession {
            issue: Some(667),
            title: Some("already tracked".to_string()),
            dispatched_at: Some("2026-03-07T12:14:00Z".to_string()),
            model: Some("gpt-5.3-codex".to_string()),
            status: Some("merged".to_string()),
            pr: Some(668),
            merged_at: Some("2026-03-07T13:53:21Z".to_string()),
            extra: BTreeMap::new(),
        }];
        let issues = vec![
            IssueRecord {
                number: 667,
                title: "Standardize cycle number access in 3 Rust tools".to_string(),
                created_at: "2026-03-07T12:14:00Z".to_string(),
                body: None,
                pull_request: None,
            },
            IssueRecord {
                number: 674,
                title: "Extend record-dispatch and process-merge to maintain agent_sessions array"
                    .to_string(),
                created_at: "2026-03-07T13:53:44Z".to_string(),
                body: None,
                pull_request: None,
            },
            IssueRecord {
                number: 303,
                title: "Phase 4b: npm publish workflow".to_string(),
                created_at: "2026-03-01T13:47:00Z".to_string(),
                body: None,
                pull_request: None,
            },
            IssueRecord {
                number: 999,
                title: "No PR ever landed".to_string(),
                created_at: "2026-03-07T00:00:00Z".to_string(),
                body: None,
                pull_request: None,
            },
        ];
        let prs = vec![
            PullRequestRecord {
                number: 675,
                title: "Keep `agent_sessions` in sync across dispatch and merge tooling".to_string(),
                state: "closed".to_string(),
                body: Some("Fixes EvaLok/schema-org-json-ld#674".to_string()),
                merged_at: Some("2026-03-07T15:12:15Z".to_string()),
                head: PullRequestHead {
                    ref_name: "copilot/extend-agent-sessions-array".to_string(),
                },
            },
            PullRequestRecord {
                number: 305,
                title: "Add release-driven npm publish workflow".to_string(),
                state: "closed".to_string(),
                body: Some("Workflow implementation without merge.".to_string()),
                merged_at: None,
                head: PullRequestHead {
                    ref_name: "copilot/add-npm-publish-workflow".to_string(),
                },
            },
        ];

        let plan = build_backfill_plan(&existing_sessions, &issues, &prs).expect("plan should build");
        assert_eq!(plan.already_tracked, 1);
        assert_eq!(plan.new_sessions.len(), 3);

        let issues: Vec<i64> = plan
            .new_sessions
            .iter()
            .map(|session| session.issue.expect("issue should be set"))
            .collect();
        assert_eq!(issues, vec![303, 674, 999]);

        let workflow = &plan.new_sessions[0];
        assert_eq!(workflow.pr, Some(305));
        assert_eq!(workflow.status.as_deref(), Some("closed"));
        assert_eq!(workflow.model.as_deref(), Some("unknown"));

        let merge = &plan.new_sessions[1];
        assert_eq!(merge.pr, Some(675));
        assert_eq!(merge.status.as_deref(), Some("merged"));
        assert_eq!(merge.merged_at.as_deref(), Some("2026-03-07T15:12:15Z"));

        let failed = &plan.new_sessions[2];
        assert_eq!(failed.pr, None);
        assert_eq!(failed.status.as_deref(), Some("failed"));
    }
}
