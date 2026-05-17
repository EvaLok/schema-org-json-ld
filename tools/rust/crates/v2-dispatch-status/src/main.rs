// v2-dispatch-status
//
// Probes the canonical absorption-readiness signal for a Copilot dispatch issue.
// Given an issue number, queries GitHub via GraphQL for:
//   - issue state (OPEN/CLOSED) and current assignees
//   - timeline events: AssignedEvent (was Copilot assigned?), ConnectedEvent
//     (did Copilot actually connect?), CrossReferencedEvent (linked PRs)
//   - linked PR details (head ref, state, merged, merged_at)
//   - candidate `copilot/*` branches in the repo (secondary probe, only when
//     no linked PR is found — this catches the cycle 155 false-negative case
//     where Copilot committed to a working branch but did not open a PR)
//   - count of comments authored by the Copilot bot
//
// Extracts the cycle 155 false-negative dispatch-return-detection pattern
// (dispatch-return-detection-must-include-dispatch-working-branches) into a
// reusable tool. Aligns with CORE-DESIGN-PRINCIPLE: a procedural inspection
// the orchestrator performed manually across cycle 154→155 belongs in a tool,
// not in scratch.
//
// Read-only. No mutation of state.json, no commit, no PR creation.

use clap::{Parser, Subcommand};
use serde_json::Value;
use std::process::{Command, ExitCode};

#[derive(Parser, Debug)]
#[command(
    name = "v2-dispatch-status",
    about = "Probe canonical absorption-readiness signal for a Copilot dispatch issue"
)]
struct Args {
    /// Repository (owner/name)
    #[arg(long, default_value = "EvaLok/schema-org-json-ld")]
    repo: String,

    /// Emit machine-readable JSON instead of human-readable summary
    #[arg(long, default_value_t = false)]
    json: bool,

    #[command(subcommand)]
    command: SubCmd,
}

#[derive(Subcommand, Debug)]
enum SubCmd {
    /// Probe a single dispatch issue and report its absorption-readiness signal.
    Check {
        /// Issue number to probe
        #[arg(long)]
        issue: u64,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum StatusError {
    Gh(String),
    Json(String),
    Repo(String),
    Schema(String),
}

impl std::fmt::Display for StatusError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StatusError::Gh(s) => write!(f, "gh: {s}"),
            StatusError::Json(s) => write!(f, "json: {s}"),
            StatusError::Repo(s) => write!(f, "repo: {s}"),
            StatusError::Schema(s) => write!(f, "schema: {s}"),
        }
    }
}

impl From<serde_json::Error> for StatusError {
    fn from(e: serde_json::Error) -> Self {
        StatusError::Json(e.to_string())
    }
}

/// Snapshot of one dispatch issue's GitHub state, distilled to what
/// classification needs. Built from one or two GraphQL queries.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
struct DispatchSnapshot {
    issue: u64,
    /// "OPEN" or "CLOSED" or "NOT_FOUND".
    state: String,
    title: String,
    /// Did at least one AssignedEvent name Copilot as assignee?
    copilot_ever_assigned: bool,
    /// Did at least one ConnectedEvent have Copilot as actor?
    copilot_connected: bool,
    /// Timestamp of first connect event, if any (ISO-8601).
    copilot_connected_at: Option<String>,
    /// Cross-referenced PRs from the timeline that have `copilot/*` head ref.
    linked_copilot_prs: Vec<LinkedPr>,
    /// Count of comments authored by the Copilot bot.
    copilot_comments: u32,
    /// `copilot/*` branches in the repo (secondary probe; only populated
    /// when no `linked_copilot_prs` were found AND the dispatch connected).
    candidate_branches: Vec<CopilotBranch>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
struct LinkedPr {
    number: u64,
    state: String, // "OPEN" | "CLOSED" | "MERGED"
    merged: bool,
    merged_at: Option<String>,
    head_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
struct CopilotBranch {
    /// Full ref name minus `refs/heads/` prefix (e.g., `copilot/redesign-critique-v2-cycle-runner`).
    name: String,
    /// HEAD commit SHA on the branch.
    head_sha: String,
    /// ISO-8601 committer date of HEAD commit.
    head_committed_at: String,
}

/// Computed classification of the snapshot.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
enum DispatchSignal {
    /// Issue does not exist or could not be fetched.
    NotFound,
    /// Issue exists but Copilot was never assigned.
    NotDispatched,
    /// Copilot was assigned but never connected (ADR 0016 watch window).
    DispatchedNoConnect,
    /// Copilot connected but produced no observable output (no linked PR,
    /// no `copilot/*` candidate branches, no comments). Either still working
    /// or genuinely silent.
    ConnectedNoOutput { connected_at: String },
    /// Copilot connected and there are candidate `copilot/*` branches in the
    /// repo, but no linked PR — the cycle 155 false-negative case. The
    /// orchestrator should inspect the branches to identify the dispatch's
    /// working branch and read its commit content directly.
    BranchCandidates {
        connected_at: String,
        branches: Vec<CopilotBranch>,
        copilot_comments: u32,
    },
    /// Open PR exists with `copilot/*` head referencing the issue.
    PrOpen { pr: u64, head_ref: String },
    /// Closed-but-unmerged PR exists with `copilot/*` head.
    PrClosedUnmerged { pr: u64, head_ref: String },
    /// Merged PR exists with `copilot/*` head.
    PrMerged {
        pr: u64,
        head_ref: String,
        merged_at: String,
    },
}

impl DispatchSignal {
    /// Suggested action verb for human-readable output. Heuristic; the
    /// orchestrator weighs the recommendation.
    fn recommended_action(&self) -> &'static str {
        match self {
            DispatchSignal::NotFound => "investigate-issue-number",
            DispatchSignal::NotDispatched => "check-dispatch-tooling",
            DispatchSignal::DispatchedNoConnect => "await-or-rediscover-bot",
            DispatchSignal::ConnectedNoOutput { .. } => "await",
            DispatchSignal::BranchCandidates { .. } => "absorb-from-branch",
            DispatchSignal::PrOpen { .. } => "review-pr",
            DispatchSignal::PrClosedUnmerged { .. } => "investigate-pr-closure",
            DispatchSignal::PrMerged { .. } => "housekeep-close",
        }
    }
}

/// Abstraction over the GitHub query layer. The CLI uses `GhCliClient`; tests use `MockGh`.
trait GhClient {
    fn query_dispatch(
        &self,
        owner: &str,
        name: &str,
        issue: u64,
    ) -> Result<DispatchSnapshotRaw, StatusError>;

    /// Fetch `copilot/*` branch refs and their head commits. Called only when
    /// the primary probe surfaced no linked PR.
    fn query_copilot_branches(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<Vec<CopilotBranch>, StatusError>;
}

/// Raw payload from the primary GraphQL query, before secondary branch probe.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DispatchSnapshotRaw {
    issue: u64,
    state: String,
    title: String,
    copilot_ever_assigned: bool,
    copilot_connected: bool,
    copilot_connected_at: Option<String>,
    linked_copilot_prs: Vec<LinkedPr>,
    copilot_comments: u32,
}

struct GhCliClient;

const DISPATCH_QUERY: &str = r#"
query($owner: String!, $name: String!, $number: Int!) {
  repository(owner: $owner, name: $name) {
    issue(number: $number) {
      state
      title
      closedByPullRequestsReferences(first: 10, includeClosedPrs: true, userLinkedOnly: false) {
        nodes {
          number
          state
          merged
          mergedAt
          headRefName
        }
      }
      timelineItems(first: 100, itemTypes: [ASSIGNED_EVENT, CONNECTED_EVENT, CROSS_REFERENCED_EVENT]) {
        nodes {
          __typename
          ... on AssignedEvent {
            createdAt
            assignee {
              __typename
              ... on User { login }
              ... on Bot { login }
            }
          }
          ... on ConnectedEvent {
            createdAt
            actor { login }
          }
          ... on CrossReferencedEvent {
            createdAt
            source {
              __typename
              ... on PullRequest {
                number
                state
                merged
                mergedAt
                headRefName
              }
            }
          }
        }
      }
      comments(first: 100) {
        nodes {
          author { login }
        }
      }
    }
  }
}
"#;

const BRANCHES_QUERY: &str = r#"
query($owner: String!, $name: String!, $cursor: String) {
  repository(owner: $owner, name: $name) {
    refs(refPrefix: "refs/heads/copilot/", first: 100, after: $cursor) {
      nodes {
        name
        target {
          __typename
          ... on Commit {
            oid
            committedDate
          }
        }
      }
      pageInfo {
        endCursor
        hasNextPage
      }
    }
  }
}
"#;

impl GhClient for GhCliClient {
    fn query_dispatch(
        &self,
        owner: &str,
        name: &str,
        issue: u64,
    ) -> Result<DispatchSnapshotRaw, StatusError> {
        let output = Command::new("gh")
            .args([
                "api",
                "graphql",
                "-f",
                &format!("query={DISPATCH_QUERY}"),
                "-F",
                &format!("owner={owner}"),
                "-F",
                &format!("name={name}"),
                "-F",
                &format!("number={issue}"),
            ])
            .output()
            .map_err(|e| StatusError::Gh(format!("invoking gh: {e}")))?;

        let stderr = String::from_utf8_lossy(&output.stderr);
        if !output.status.success() {
            // gh exits non-zero when GraphQL returns errors. Detect the
            // "issue does not exist" case specifically so we can classify as
            // NOT_FOUND rather than propagating a generic error.
            if stderr.contains("Could not resolve to an Issue") {
                return Ok(DispatchSnapshotRaw {
                    issue,
                    state: "NOT_FOUND".to_string(),
                    title: String::new(),
                    copilot_ever_assigned: false,
                    copilot_connected: false,
                    copilot_connected_at: None,
                    linked_copilot_prs: Vec::new(),
                    copilot_comments: 0,
                });
            }
            return Err(StatusError::Gh(format!(
                "gh api graphql exit {:?}: {stderr}",
                output.status.code()
            )));
        }

        let body: Value = serde_json::from_slice(&output.stdout)?;
        parse_dispatch_response(&body, issue)
    }

    fn query_copilot_branches(
        &self,
        owner: &str,
        name: &str,
    ) -> Result<Vec<CopilotBranch>, StatusError> {
        let mut all = Vec::new();
        let mut cursor: Option<String> = None;
        loop {
            let mut args = vec![
                "api".to_string(),
                "graphql".to_string(),
                "-f".to_string(),
                format!("query={BRANCHES_QUERY}"),
                "-F".to_string(),
                format!("owner={owner}"),
                "-F".to_string(),
                format!("name={name}"),
            ];
            if let Some(c) = &cursor {
                args.push("-F".to_string());
                args.push(format!("cursor={c}"));
            }
            let output = Command::new("gh")
                .args(&args)
                .output()
                .map_err(|e| StatusError::Gh(format!("invoking gh: {e}")))?;
            if !output.status.success() {
                return Err(StatusError::Gh(format!(
                    "gh api graphql exit {:?}: {}",
                    output.status.code(),
                    String::from_utf8_lossy(&output.stderr)
                )));
            }
            let body: Value = serde_json::from_slice(&output.stdout)?;
            let (page, next) = parse_branches_response(&body)?;
            all.extend(page);
            match next {
                Some(c) => cursor = Some(c),
                None => break,
            }
        }
        Ok(all)
    }
}

/// GitHub bot login for the Copilot coding agent. The bot's *display name* is
/// "Copilot" but the GraphQL `login` field returns `copilot-swe-agent`. Match
/// both for forward-compat in case GitHub rotates the canonical login.
fn is_copilot_login(login: &str) -> bool {
    let l = login.to_ascii_lowercase();
    l == "copilot-swe-agent" || l == "copilot" || l == "github-copilot"
}

fn parse_dispatch_response(body: &Value, issue: u64) -> Result<DispatchSnapshotRaw, StatusError> {
    let issue_obj = body.pointer("/data/repository/issue").ok_or_else(|| {
        StatusError::Gh(format!(
            "issue {issue}: response missing data.repository.issue"
        ))
    })?;

    if issue_obj.is_null() {
        return Ok(DispatchSnapshotRaw {
            issue,
            state: "NOT_FOUND".to_string(),
            title: String::new(),
            copilot_ever_assigned: false,
            copilot_connected: false,
            copilot_connected_at: None,
            linked_copilot_prs: Vec::new(),
            copilot_comments: 0,
        });
    }

    let state = issue_obj
        .get("state")
        .and_then(|v| v.as_str())
        .ok_or_else(|| StatusError::Schema(format!("issue {issue}: missing state field")))?
        .to_string();

    let title = issue_obj
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let mut copilot_ever_assigned = false;
    let mut copilot_connected = false;
    let mut copilot_connected_at: Option<String> = None;
    let mut linked_copilot_prs: Vec<LinkedPr> = Vec::new();

    // Primary source for linked PRs that closed the issue (most reliable for
    // CLOSED issues — GitHub's Development panel surfaces this directly).
    if let Some(nodes) = issue_obj
        .pointer("/closedByPullRequestsReferences/nodes")
        .and_then(|v| v.as_array())
    {
        for node in nodes {
            let head_ref = node
                .get("headRefName")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if !head_ref.starts_with("copilot/") {
                continue;
            }
            let number = match node.get("number").and_then(|v| v.as_u64()) {
                Some(n) => n,
                None => continue,
            };
            let pr_state = node
                .get("state")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let merged = node
                .get("merged")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let merged_at = node
                .get("mergedAt")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());
            let pr = LinkedPr {
                number,
                state: pr_state,
                merged,
                merged_at,
                head_ref: head_ref.to_string(),
            };
            if !linked_copilot_prs.iter().any(|p| p.number == pr.number) {
                linked_copilot_prs.push(pr);
            }
        }
    }

    if let Some(nodes) = issue_obj
        .pointer("/timelineItems/nodes")
        .and_then(|v| v.as_array())
    {
        for node in nodes {
            let typename = node
                .get("__typename")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            match typename {
                "AssignedEvent" => {
                    let login = node
                        .pointer("/assignee/login")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if is_copilot_login(login) {
                        copilot_ever_assigned = true;
                    }
                }
                "ConnectedEvent" => {
                    let login = node
                        .pointer("/actor/login")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if is_copilot_login(login) {
                        if !copilot_connected {
                            // Record the first connect timestamp.
                            copilot_connected_at = node
                                .get("createdAt")
                                .and_then(|v| v.as_str())
                                .map(|s| s.to_string());
                        }
                        copilot_connected = true;
                    }
                }
                "CrossReferencedEvent" => {
                    let source_typename = node
                        .pointer("/source/__typename")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if source_typename != "PullRequest" {
                        continue;
                    }
                    let head_ref = node
                        .pointer("/source/headRefName")
                        .and_then(|v| v.as_str())
                        .unwrap_or("");
                    if !head_ref.starts_with("copilot/") {
                        continue;
                    }
                    let number = match node.pointer("/source/number").and_then(|v| v.as_u64()) {
                        Some(n) => n,
                        None => continue,
                    };
                    let pr_state = node
                        .pointer("/source/state")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let merged = node
                        .pointer("/source/merged")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    let merged_at = node
                        .pointer("/source/mergedAt")
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    let pr = LinkedPr {
                        number,
                        state: pr_state,
                        merged,
                        merged_at,
                        head_ref: head_ref.to_string(),
                    };
                    if !linked_copilot_prs.iter().any(|p| p.number == pr.number) {
                        linked_copilot_prs.push(pr);
                    }
                }
                _ => {}
            }
        }
    }

    let mut copilot_comments = 0u32;
    if let Some(nodes) = issue_obj
        .pointer("/comments/nodes")
        .and_then(|v| v.as_array())
    {
        for node in nodes {
            let login = node
                .pointer("/author/login")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if is_copilot_login(login) {
                copilot_comments += 1;
            }
        }
    }

    Ok(DispatchSnapshotRaw {
        issue,
        state,
        title,
        copilot_ever_assigned,
        copilot_connected,
        copilot_connected_at,
        linked_copilot_prs,
        copilot_comments,
    })
}

/// Returns (page, next_cursor).
fn parse_branches_response(
    body: &Value,
) -> Result<(Vec<CopilotBranch>, Option<String>), StatusError> {
    let refs = body.pointer("/data/repository/refs").ok_or_else(|| {
        StatusError::Gh("branches response missing data.repository.refs".to_string())
    })?;

    let mut out = Vec::new();
    if let Some(nodes) = refs.get("nodes").and_then(|v| v.as_array()) {
        for node in nodes {
            let short_name = match node.get("name").and_then(|v| v.as_str()) {
                Some(s) => s,
                None => continue,
            };
            // Query uses refPrefix "refs/heads/copilot/", so name is the suffix.
            // Prepend "copilot/" for the full short ref name.
            let full = format!("copilot/{short_name}");
            let target_type = node
                .pointer("/target/__typename")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if target_type != "Commit" {
                continue;
            }
            let oid = match node.pointer("/target/oid").and_then(|v| v.as_str()) {
                Some(s) => s.to_string(),
                None => continue,
            };
            let committed_at = node
                .pointer("/target/committedDate")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            out.push(CopilotBranch {
                name: full,
                head_sha: oid,
                head_committed_at: committed_at,
            });
        }
    }

    let next = refs
        .pointer("/pageInfo/hasNextPage")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let cursor = if next {
        refs.pointer("/pageInfo/endCursor")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    } else {
        None
    };

    Ok((out, cursor))
}

/// Build the full snapshot. Calls the primary probe; if no linked PR was
/// found and the dispatch connected, follows up with the branches probe and
/// filters candidates by "committedDate >= connected_at" (avoiding ancient
/// branches unrelated to this dispatch).
fn build_snapshot(
    client: &dyn GhClient,
    owner: &str,
    name: &str,
    issue: u64,
) -> Result<DispatchSnapshot, StatusError> {
    let raw = client.query_dispatch(owner, name, issue)?;
    let candidate_branches = if raw.linked_copilot_prs.is_empty() && raw.copilot_connected {
        let all = client.query_copilot_branches(owner, name)?;
        let cutoff = raw.copilot_connected_at.clone().unwrap_or_default();
        filter_recent_branches(all, &cutoff)
    } else {
        Vec::new()
    };
    Ok(DispatchSnapshot {
        issue: raw.issue,
        state: raw.state,
        title: raw.title,
        copilot_ever_assigned: raw.copilot_ever_assigned,
        copilot_connected: raw.copilot_connected,
        copilot_connected_at: raw.copilot_connected_at,
        linked_copilot_prs: raw.linked_copilot_prs,
        copilot_comments: raw.copilot_comments,
        candidate_branches,
    })
}

/// Filter copilot/* branches to those committed AT OR AFTER `cutoff` (the
/// dispatch's connect timestamp). Branches with empty `head_committed_at`
/// are conservatively kept. If `cutoff` is empty, all branches are kept.
fn filter_recent_branches(branches: Vec<CopilotBranch>, cutoff: &str) -> Vec<CopilotBranch> {
    if cutoff.is_empty() {
        return branches;
    }
    branches
        .into_iter()
        .filter(|b| b.head_committed_at.is_empty() || b.head_committed_at.as_str() >= cutoff)
        .collect()
}

fn classify(snapshot: &DispatchSnapshot) -> DispatchSignal {
    if snapshot.state == "NOT_FOUND" {
        return DispatchSignal::NotFound;
    }

    // Prefer linked PR signals (they are precise — head ref is `copilot/*`
    // and the cross-reference is GitHub-canonical).
    if let Some(pr) = snapshot.linked_copilot_prs.iter().find(|p| p.merged) {
        return DispatchSignal::PrMerged {
            pr: pr.number,
            head_ref: pr.head_ref.clone(),
            merged_at: pr.merged_at.clone().unwrap_or_default(),
        };
    }
    if let Some(pr) = snapshot
        .linked_copilot_prs
        .iter()
        .find(|p| p.state == "OPEN")
    {
        return DispatchSignal::PrOpen {
            pr: pr.number,
            head_ref: pr.head_ref.clone(),
        };
    }
    if let Some(pr) = snapshot
        .linked_copilot_prs
        .iter()
        .find(|p| p.state == "CLOSED" && !p.merged)
    {
        return DispatchSignal::PrClosedUnmerged {
            pr: pr.number,
            head_ref: pr.head_ref.clone(),
        };
    }

    if !snapshot.copilot_ever_assigned {
        return DispatchSignal::NotDispatched;
    }
    if !snapshot.copilot_connected {
        return DispatchSignal::DispatchedNoConnect;
    }

    let connected_at = snapshot.copilot_connected_at.clone().unwrap_or_default();
    if snapshot.candidate_branches.is_empty() {
        return DispatchSignal::ConnectedNoOutput { connected_at };
    }
    DispatchSignal::BranchCandidates {
        connected_at,
        branches: snapshot.candidate_branches.clone(),
        copilot_comments: snapshot.copilot_comments,
    }
}

fn parse_repo(repo: &str) -> Result<(String, String), StatusError> {
    let parts: Vec<&str> = repo.splitn(2, '/').collect();
    if parts.len() != 2 || parts[0].is_empty() || parts[1].is_empty() {
        return Err(StatusError::Repo(format!(
            "expected owner/name, got {repo:?}"
        )));
    }
    Ok((parts[0].to_string(), parts[1].to_string()))
}

fn render_human(snapshot: &DispatchSnapshot, signal: &DispatchSignal) -> String {
    let mut out = String::new();
    out.push_str(&format!("issue #{} — {}\n", snapshot.issue, snapshot.title));
    out.push_str(&format!("state: {}\n", snapshot.state));
    out.push_str(&format!(
        "copilot: assigned={} connected={}{}\n",
        snapshot.copilot_ever_assigned,
        snapshot.copilot_connected,
        snapshot
            .copilot_connected_at
            .as_deref()
            .map(|s| format!(" at={s}"))
            .unwrap_or_default()
    ));
    out.push_str(&format!(
        "copilot comments: {}\n",
        snapshot.copilot_comments
    ));
    if !snapshot.linked_copilot_prs.is_empty() {
        out.push_str("linked copilot/* PRs:\n");
        for pr in &snapshot.linked_copilot_prs {
            out.push_str(&format!(
                "  #{} state={} merged={} head={}\n",
                pr.number, pr.state, pr.merged, pr.head_ref
            ));
        }
    }
    if !snapshot.candidate_branches.is_empty() {
        out.push_str("candidate copilot/* branches (no linked PR; filtered by connect-time):\n");
        for b in &snapshot.candidate_branches {
            out.push_str(&format!(
                "  {} sha={} committed_at={}\n",
                b.name, b.head_sha, b.head_committed_at
            ));
        }
    }
    out.push_str(&format!("\nsignal: {}\n", signal_kind(signal)));
    out.push_str(&format!(
        "recommended action: {}\n",
        signal.recommended_action()
    ));
    out
}

fn signal_kind(signal: &DispatchSignal) -> &'static str {
    match signal {
        DispatchSignal::NotFound => "not-found",
        DispatchSignal::NotDispatched => "not-dispatched",
        DispatchSignal::DispatchedNoConnect => "dispatched-no-connect",
        DispatchSignal::ConnectedNoOutput { .. } => "connected-no-output",
        DispatchSignal::BranchCandidates { .. } => "branch-candidates",
        DispatchSignal::PrOpen { .. } => "pr-open",
        DispatchSignal::PrClosedUnmerged { .. } => "pr-closed-unmerged",
        DispatchSignal::PrMerged { .. } => "pr-merged",
    }
}

fn run<W: std::io::Write>(
    args: Args,
    client: &dyn GhClient,
    out: &mut W,
) -> Result<i32, StatusError> {
    let (owner, name) = parse_repo(&args.repo)?;
    match &args.command {
        SubCmd::Check { issue } => {
            let snapshot = build_snapshot(client, &owner, &name, *issue)?;
            let signal = classify(&snapshot);
            if args.json {
                let body = serde_json::json!({
                    "snapshot": snapshot,
                    "signal": &signal,
                    "signal_kind": signal_kind(&signal),
                    "recommended_action": signal.recommended_action(),
                });
                writeln!(out, "{}", serde_json::to_string_pretty(&body)?)
                    .map_err(|e| StatusError::Gh(format!("write: {e}")))?;
            } else {
                let text = render_human(&snapshot, &signal);
                write!(out, "{text}").map_err(|e| StatusError::Gh(format!("write: {e}")))?;
            }
            Ok(0)
        }
    }
}

fn main() -> ExitCode {
    let args = Args::parse();
    let client = GhCliClient;
    let mut stdout = std::io::stdout();
    match run(args, &client, &mut stdout) {
        Ok(code) => ExitCode::from(code as u8),
        Err(e) => {
            eprintln!("v2-dispatch-status: {e}");
            ExitCode::from(1)
        }
    }
}

// =====================================================================
// Tests
// =====================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Test double: returns canned responses keyed by issue (for dispatch)
    /// and by call-count (for branches).
    struct MockGh {
        dispatch: HashMap<u64, DispatchSnapshotRaw>,
        branches: Vec<CopilotBranch>,
    }

    impl GhClient for MockGh {
        fn query_dispatch(
            &self,
            _owner: &str,
            _name: &str,
            issue: u64,
        ) -> Result<DispatchSnapshotRaw, StatusError> {
            self.dispatch.get(&issue).cloned().ok_or_else(|| {
                StatusError::Gh(format!("mock: no canned response for issue {issue}"))
            })
        }
        fn query_copilot_branches(
            &self,
            _owner: &str,
            _name: &str,
        ) -> Result<Vec<CopilotBranch>, StatusError> {
            Ok(self.branches.clone())
        }
    }

    fn empty_raw(issue: u64, state: &str) -> DispatchSnapshotRaw {
        DispatchSnapshotRaw {
            issue,
            state: state.to_string(),
            title: format!("dispatch #{issue}"),
            copilot_ever_assigned: false,
            copilot_connected: false,
            copilot_connected_at: None,
            linked_copilot_prs: Vec::new(),
            copilot_comments: 0,
        }
    }

    #[test]
    fn parse_repo_splits_owner_and_name() {
        let (o, n) = parse_repo("EvaLok/schema-org-json-ld").unwrap();
        assert_eq!(o, "EvaLok");
        assert_eq!(n, "schema-org-json-ld");
    }

    #[test]
    fn parse_repo_rejects_missing_slash() {
        assert!(parse_repo("EvaLok").is_err());
    }

    #[test]
    fn parse_repo_rejects_empty_owner() {
        assert!(parse_repo("/schema-org-json-ld").is_err());
    }

    #[test]
    fn parse_repo_rejects_empty_name() {
        assert!(parse_repo("EvaLok/").is_err());
    }

    #[test]
    fn parse_dispatch_response_not_found() {
        let body = serde_json::json!({"data": {"repository": {"issue": null}}});
        let raw = parse_dispatch_response(&body, 99999).unwrap();
        assert_eq!(raw.state, "NOT_FOUND");
        assert_eq!(raw.title, "");
        assert!(!raw.copilot_ever_assigned);
        assert!(!raw.copilot_connected);
        assert_eq!(raw.copilot_comments, 0);
    }

    #[test]
    fn parse_dispatch_response_open_no_events() {
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "OPEN",
            "title": "stub",
            "timelineItems": {"nodes": []},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 100).unwrap();
        assert_eq!(raw.state, "OPEN");
        assert!(!raw.copilot_ever_assigned);
        assert!(!raw.copilot_connected);
        assert!(raw.linked_copilot_prs.is_empty());
    }

    #[test]
    fn parse_dispatch_response_assigned_only() {
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "OPEN",
            "title": "[redesign-critique] X",
            "timelineItems": {"nodes": [
                {"__typename": "AssignedEvent", "createdAt": "2026-05-15T10:58:00Z",
                 "assignee": {"__typename": "Bot", "login": "Copilot"}}
            ]},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 100).unwrap();
        assert!(raw.copilot_ever_assigned);
        assert!(!raw.copilot_connected);
    }

    #[test]
    fn parse_dispatch_response_assigned_to_real_copilot_swe_agent_login() {
        // The bot's display name is "Copilot" but the GraphQL `login` field
        // returns `copilot-swe-agent`. Live smoke against issue #2960
        // (cycle 155 case) revealed this — the prior matcher missed it.
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "OPEN",
            "title": "[redesign-critique] cycle 152 v2-cycle-runner adversarial critique",
            "timelineItems": {"nodes": [
                {"__typename": "AssignedEvent", "createdAt": "2026-05-15T10:58:54Z",
                 "assignee": {"__typename": "Bot", "login": "copilot-swe-agent"}},
                {"__typename": "ConnectedEvent", "createdAt": "2026-05-15T10:59:04Z",
                 "actor": {"__typename": "Bot", "login": "copilot-swe-agent"}}
            ]},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 2960).unwrap();
        assert!(raw.copilot_ever_assigned);
        assert!(raw.copilot_connected);
        assert_eq!(
            raw.copilot_connected_at.as_deref(),
            Some("2026-05-15T10:59:04Z")
        );
    }

    #[test]
    fn is_copilot_login_matches_known_logins() {
        assert!(is_copilot_login("copilot-swe-agent"));
        assert!(is_copilot_login("Copilot"));
        assert!(is_copilot_login("copilot"));
        assert!(is_copilot_login("github-copilot"));
        // Case-insensitive
        assert!(is_copilot_login("COPILOT-SWE-AGENT"));
        // Negatives
        assert!(!is_copilot_login("EvaLok"));
        assert!(!is_copilot_login("dependabot"));
        assert!(!is_copilot_login("copilot-helper"));
    }

    #[test]
    fn parse_dispatch_response_assigned_to_human_not_copilot() {
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "OPEN",
            "title": "manual issue",
            "timelineItems": {"nodes": [
                {"__typename": "AssignedEvent", "createdAt": "2026-05-15T10:58:00Z",
                 "assignee": {"__typename": "User", "login": "EvaLok"}}
            ]},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 100).unwrap();
        assert!(!raw.copilot_ever_assigned);
        assert!(!raw.copilot_connected);
    }

    #[test]
    fn parse_dispatch_response_connected() {
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "OPEN",
            "title": "stub",
            "timelineItems": {"nodes": [
                {"__typename": "AssignedEvent", "createdAt": "2026-05-15T10:58:00Z",
                 "assignee": {"__typename": "Bot", "login": "Copilot"}},
                {"__typename": "ConnectedEvent", "createdAt": "2026-05-15T10:59:04Z",
                 "actor": {"login": "Copilot"}}
            ]},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 100).unwrap();
        assert!(raw.copilot_ever_assigned);
        assert!(raw.copilot_connected);
        assert_eq!(
            raw.copilot_connected_at.as_deref(),
            Some("2026-05-15T10:59:04Z")
        );
    }

    #[test]
    fn parse_dispatch_response_records_first_connect_time_only() {
        // Multiple Connected events: keep the first one. (Reconnects shouldn't
        // override the canonical "when did the bot first connect" timestamp.)
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "OPEN",
            "title": "stub",
            "timelineItems": {"nodes": [
                {"__typename": "ConnectedEvent", "createdAt": "2026-05-15T10:59:04Z",
                 "actor": {"login": "Copilot"}},
                {"__typename": "ConnectedEvent", "createdAt": "2026-05-15T11:30:00Z",
                 "actor": {"login": "Copilot"}}
            ]},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 100).unwrap();
        assert_eq!(
            raw.copilot_connected_at.as_deref(),
            Some("2026-05-15T10:59:04Z")
        );
    }

    #[test]
    fn parse_dispatch_response_closed_by_pr_is_picked_up() {
        // `closedByPullRequestsReferences` is the canonical GraphQL field for
        // "PRs that closed this issue". Live smoke against #2952 revealed
        // that CrossReferencedEvent alone doesn't surface this — and that
        // draft PRs ARE included in this field, which catches a class of
        // false-negatives the timeline-only path misses.
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "CLOSED",
            "title": "stub",
            "closedByPullRequestsReferences": {"nodes": [
                {"number": 2953, "state": "MERGED", "merged": true,
                 "mergedAt": "2026-05-14T22:42:00Z",
                 "headRefName": "copilot/redesign-impl-v2-prompt-contract-check"}
            ]},
            "timelineItems": {"nodes": []},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 2952).unwrap();
        assert_eq!(raw.linked_copilot_prs.len(), 1);
        assert_eq!(raw.linked_copilot_prs[0].number, 2953);
        assert!(raw.linked_copilot_prs[0].merged);
        assert_eq!(raw.linked_copilot_prs[0].state, "MERGED");
    }

    #[test]
    fn parse_dispatch_response_draft_pr_via_closed_by_field() {
        // Live smoke against issue #2960 revealed that PR #2961 was open as
        // DRAFT since dispatch time, but cycle 154's `gh pr list` (without
        // --state all) missed it, and cycle 155's branch-direct inspection
        // missed it too. The closedByPullRequestsReferences field surfaces
        // draft PRs — guard against regressing on this discovery.
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "OPEN",
            "title": "[redesign-critique] cycle 152 v2-cycle-runner adversarial critique",
            "closedByPullRequestsReferences": {"nodes": [
                {"number": 2961, "state": "OPEN", "merged": false, "mergedAt": null,
                 "headRefName": "copilot/redesign-critique-v2-cycle-runner"}
            ]},
            "timelineItems": {"nodes": []},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 2960).unwrap();
        assert_eq!(raw.linked_copilot_prs.len(), 1);
        assert_eq!(raw.linked_copilot_prs[0].number, 2961);
        assert!(!raw.linked_copilot_prs[0].merged);
        assert_eq!(raw.linked_copilot_prs[0].state, "OPEN");
    }

    #[test]
    fn parse_dispatch_response_closed_by_field_filters_non_copilot_branches() {
        // Even in closedByPullRequestsReferences, only `copilot/*` head refs
        // should be kept — a manually-authored PR closing the issue is not
        // a Copilot dispatch outcome.
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "CLOSED",
            "title": "stub",
            "closedByPullRequestsReferences": {"nodes": [
                {"number": 999, "state": "MERGED", "merged": true,
                 "mergedAt": "x", "headRefName": "feature/manual-pr"}
            ]},
            "timelineItems": {"nodes": []},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 100).unwrap();
        assert!(raw.linked_copilot_prs.is_empty());
    }

    #[test]
    fn parse_dispatch_response_merges_closed_by_and_cross_referenced_dedup() {
        // If the same PR appears in BOTH closedByPullRequestsReferences and
        // CrossReferencedEvent, dedup by PR number — same as within-source
        // dedup. closedByPullRequestsReferences wins (parsed first).
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "OPEN",
            "title": "stub",
            "closedByPullRequestsReferences": {"nodes": [
                {"number": 2961, "state": "OPEN", "merged": false, "mergedAt": null,
                 "headRefName": "copilot/x"}
            ]},
            "timelineItems": {"nodes": [
                {"__typename": "CrossReferencedEvent",
                 "source": {"__typename": "PullRequest", "number": 2961,
                            "state": "OPEN", "merged": false, "mergedAt": null,
                            "headRefName": "copilot/x"}}
            ]},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 100).unwrap();
        assert_eq!(raw.linked_copilot_prs.len(), 1);
    }

    #[test]
    fn parse_dispatch_response_linked_pr_copilot_branch() {
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "OPEN",
            "title": "stub",
            "timelineItems": {"nodes": [
                {"__typename": "CrossReferencedEvent", "createdAt": "2026-05-15T11:00:00Z",
                 "source": {"__typename": "PullRequest", "number": 2961,
                            "state": "OPEN", "merged": false, "mergedAt": null,
                            "headRefName": "copilot/redesign-critique-foo"}}
            ]},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 100).unwrap();
        assert_eq!(raw.linked_copilot_prs.len(), 1);
        assert_eq!(raw.linked_copilot_prs[0].number, 2961);
        assert_eq!(raw.linked_copilot_prs[0].state, "OPEN");
        assert!(!raw.linked_copilot_prs[0].merged);
    }

    #[test]
    fn parse_dispatch_response_ignores_non_copilot_branch_prs() {
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "OPEN",
            "title": "stub",
            "timelineItems": {"nodes": [
                {"__typename": "CrossReferencedEvent",
                 "source": {"__typename": "PullRequest", "number": 999,
                            "state": "OPEN", "merged": false, "mergedAt": null,
                            "headRefName": "feature/some-feature"}}
            ]},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 100).unwrap();
        assert!(raw.linked_copilot_prs.is_empty());
    }

    #[test]
    fn parse_dispatch_response_dedup_repeated_pr_references() {
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "OPEN",
            "title": "stub",
            "timelineItems": {"nodes": [
                {"__typename": "CrossReferencedEvent",
                 "source": {"__typename": "PullRequest", "number": 2961,
                            "state": "OPEN", "merged": false, "mergedAt": null,
                            "headRefName": "copilot/foo"}},
                {"__typename": "CrossReferencedEvent",
                 "source": {"__typename": "PullRequest", "number": 2961,
                            "state": "OPEN", "merged": false, "mergedAt": null,
                            "headRefName": "copilot/foo"}}
            ]},
            "comments": {"nodes": []}
        }}}});
        let raw = parse_dispatch_response(&body, 100).unwrap();
        assert_eq!(raw.linked_copilot_prs.len(), 1);
    }

    #[test]
    fn parse_dispatch_response_counts_copilot_comments_only() {
        let body = serde_json::json!({"data": {"repository": {"issue": {
            "state": "OPEN",
            "title": "stub",
            "timelineItems": {"nodes": []},
            "comments": {"nodes": [
                {"author": {"login": "Copilot"}},
                {"author": {"login": "EvaLok"}},
                {"author": {"login": "Copilot"}}
            ]}
        }}}});
        let raw = parse_dispatch_response(&body, 100).unwrap();
        assert_eq!(raw.copilot_comments, 2);
    }

    #[test]
    fn parse_branches_response_single_page() {
        let body = serde_json::json!({"data": {"repository": {"refs": {
            "nodes": [
                {"name": "redesign-critique-v2-cycle-runner",
                 "target": {"__typename": "Commit",
                            "oid": "fbc9ddbe1234",
                            "committedDate": "2026-05-15T11:02:57Z"}},
                {"name": "other-branch",
                 "target": {"__typename": "Commit",
                            "oid": "abc123",
                            "committedDate": "2026-05-01T00:00:00Z"}}
            ],
            "pageInfo": {"endCursor": "x", "hasNextPage": false}
        }}}});
        let (page, next) = parse_branches_response(&body).unwrap();
        assert_eq!(page.len(), 2);
        assert_eq!(page[0].name, "copilot/redesign-critique-v2-cycle-runner");
        assert_eq!(page[0].head_sha, "fbc9ddbe1234");
        assert_eq!(page[0].head_committed_at, "2026-05-15T11:02:57Z");
        assert!(next.is_none());
    }

    #[test]
    fn parse_branches_response_paginated() {
        let body = serde_json::json!({"data": {"repository": {"refs": {
            "nodes": [
                {"name": "a",
                 "target": {"__typename": "Commit", "oid": "1", "committedDate": "2026-01-01T00:00:00Z"}}
            ],
            "pageInfo": {"endCursor": "cur123", "hasNextPage": true}
        }}}});
        let (page, next) = parse_branches_response(&body).unwrap();
        assert_eq!(page.len(), 1);
        assert_eq!(next.as_deref(), Some("cur123"));
    }

    #[test]
    fn filter_recent_branches_keeps_at_or_after_cutoff() {
        let cutoff = "2026-05-15T11:00:00Z";
        let bs = vec![
            CopilotBranch {
                name: "copilot/old".into(),
                head_sha: "1".into(),
                head_committed_at: "2026-04-01T00:00:00Z".into(),
            },
            CopilotBranch {
                name: "copilot/new".into(),
                head_sha: "2".into(),
                head_committed_at: "2026-05-15T11:02:57Z".into(),
            },
            CopilotBranch {
                name: "copilot/exact".into(),
                head_sha: "3".into(),
                head_committed_at: "2026-05-15T11:00:00Z".into(),
            },
        ];
        let filtered = filter_recent_branches(bs, cutoff);
        let names: Vec<_> = filtered.iter().map(|b| b.name.as_str()).collect();
        assert_eq!(names, vec!["copilot/new", "copilot/exact"]);
    }

    #[test]
    fn filter_recent_branches_keeps_empty_committed_at() {
        let cutoff = "2026-05-15T11:00:00Z";
        let bs = vec![CopilotBranch {
            name: "copilot/unknown".into(),
            head_sha: "1".into(),
            head_committed_at: String::new(),
        }];
        let filtered = filter_recent_branches(bs, cutoff);
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn filter_recent_branches_empty_cutoff_keeps_all() {
        let bs = vec![CopilotBranch {
            name: "copilot/old".into(),
            head_sha: "1".into(),
            head_committed_at: "2020-01-01T00:00:00Z".into(),
        }];
        let filtered = filter_recent_branches(bs, "");
        assert_eq!(filtered.len(), 1);
    }

    #[test]
    fn classify_not_found() {
        let s = DispatchSnapshot {
            issue: 99999,
            state: "NOT_FOUND".into(),
            title: String::new(),
            copilot_ever_assigned: false,
            copilot_connected: false,
            copilot_connected_at: None,
            linked_copilot_prs: Vec::new(),
            copilot_comments: 0,
            candidate_branches: Vec::new(),
        };
        assert_eq!(classify(&s), DispatchSignal::NotFound);
    }

    #[test]
    fn classify_not_dispatched() {
        let s = DispatchSnapshot {
            issue: 100,
            state: "OPEN".into(),
            title: "manual".into(),
            copilot_ever_assigned: false,
            copilot_connected: false,
            copilot_connected_at: None,
            linked_copilot_prs: Vec::new(),
            copilot_comments: 0,
            candidate_branches: Vec::new(),
        };
        assert_eq!(classify(&s), DispatchSignal::NotDispatched);
    }

    #[test]
    fn classify_dispatched_no_connect() {
        let s = DispatchSnapshot {
            issue: 100,
            state: "OPEN".into(),
            title: "stub".into(),
            copilot_ever_assigned: true,
            copilot_connected: false,
            copilot_connected_at: None,
            linked_copilot_prs: Vec::new(),
            copilot_comments: 0,
            candidate_branches: Vec::new(),
        };
        assert_eq!(classify(&s), DispatchSignal::DispatchedNoConnect);
    }

    #[test]
    fn classify_connected_no_output() {
        let s = DispatchSnapshot {
            issue: 100,
            state: "OPEN".into(),
            title: "stub".into(),
            copilot_ever_assigned: true,
            copilot_connected: true,
            copilot_connected_at: Some("2026-05-15T10:59:04Z".into()),
            linked_copilot_prs: Vec::new(),
            copilot_comments: 0,
            candidate_branches: Vec::new(),
        };
        match classify(&s) {
            DispatchSignal::ConnectedNoOutput { connected_at } => {
                assert_eq!(connected_at, "2026-05-15T10:59:04Z");
            }
            other => panic!("expected ConnectedNoOutput, got {other:?}"),
        }
    }

    #[test]
    fn classify_branch_candidates_cycle_155_case() {
        // Cycle 155 false-negative reconstruction: Copilot committed to its
        // working branch but did not open a PR. Cycle 154's inspection missed
        // this; the tool catches it by reporting branch candidates.
        let s = DispatchSnapshot {
            issue: 2960,
            state: "OPEN".into(),
            title: "[redesign-critique] cycle 152 v2-cycle-runner adversarial critique".into(),
            copilot_ever_assigned: true,
            copilot_connected: true,
            copilot_connected_at: Some("2026-05-15T10:59:04Z".into()),
            linked_copilot_prs: Vec::new(),
            copilot_comments: 0,
            candidate_branches: vec![CopilotBranch {
                name: "copilot/redesign-critique-v2-cycle-runner".into(),
                head_sha: "fbc9ddbe".into(),
                head_committed_at: "2026-05-15T11:02:57Z".into(),
            }],
        };
        match classify(&s) {
            DispatchSignal::BranchCandidates { branches, .. } => {
                assert_eq!(branches.len(), 1);
                assert_eq!(
                    branches[0].name,
                    "copilot/redesign-critique-v2-cycle-runner"
                );
            }
            other => panic!("expected BranchCandidates, got {other:?}"),
        }
    }

    #[test]
    fn classify_pr_open_takes_precedence_over_branches() {
        // If a linked PR is open, the PR signal trumps any branch-list signal.
        let s = DispatchSnapshot {
            issue: 100,
            state: "OPEN".into(),
            title: "stub".into(),
            copilot_ever_assigned: true,
            copilot_connected: true,
            copilot_connected_at: Some("2026-05-15T10:59:04Z".into()),
            linked_copilot_prs: vec![LinkedPr {
                number: 2961,
                state: "OPEN".into(),
                merged: false,
                merged_at: None,
                head_ref: "copilot/x".into(),
            }],
            copilot_comments: 0,
            candidate_branches: vec![CopilotBranch {
                name: "copilot/x".into(),
                head_sha: "deadbeef".into(),
                head_committed_at: "2026-05-15T11:02:57Z".into(),
            }],
        };
        match classify(&s) {
            DispatchSignal::PrOpen { pr, head_ref } => {
                assert_eq!(pr, 2961);
                assert_eq!(head_ref, "copilot/x");
            }
            other => panic!("expected PrOpen, got {other:?}"),
        }
    }

    #[test]
    fn classify_pr_merged() {
        let s = DispatchSnapshot {
            issue: 100,
            state: "CLOSED".into(),
            title: "stub".into(),
            copilot_ever_assigned: true,
            copilot_connected: true,
            copilot_connected_at: Some("2026-05-15T10:59:04Z".into()),
            linked_copilot_prs: vec![LinkedPr {
                number: 2961,
                state: "MERGED".into(),
                merged: true,
                merged_at: Some("2026-05-15T12:00:00Z".into()),
                head_ref: "copilot/x".into(),
            }],
            copilot_comments: 0,
            candidate_branches: Vec::new(),
        };
        match classify(&s) {
            DispatchSignal::PrMerged {
                pr,
                head_ref,
                merged_at,
            } => {
                assert_eq!(pr, 2961);
                assert_eq!(head_ref, "copilot/x");
                assert_eq!(merged_at, "2026-05-15T12:00:00Z");
            }
            other => panic!("expected PrMerged, got {other:?}"),
        }
    }

    #[test]
    fn classify_pr_merged_takes_precedence_over_open() {
        // Iteration order finds merged first because the find() filter for
        // `merged` runs before the OPEN filter — even if the linked list has
        // an OPEN PR before a merged one, merged wins.
        let s = DispatchSnapshot {
            issue: 100,
            state: "CLOSED".into(),
            title: "stub".into(),
            copilot_ever_assigned: true,
            copilot_connected: true,
            copilot_connected_at: Some("2026-05-15T10:59:04Z".into()),
            linked_copilot_prs: vec![
                LinkedPr {
                    number: 100,
                    state: "OPEN".into(),
                    merged: false,
                    merged_at: None,
                    head_ref: "copilot/stale-pr".into(),
                },
                LinkedPr {
                    number: 200,
                    state: "MERGED".into(),
                    merged: true,
                    merged_at: Some("2026-05-15T12:00:00Z".into()),
                    head_ref: "copilot/real-pr".into(),
                },
            ],
            copilot_comments: 0,
            candidate_branches: Vec::new(),
        };
        match classify(&s) {
            DispatchSignal::PrMerged { pr, .. } => assert_eq!(pr, 200),
            other => panic!("expected PrMerged, got {other:?}"),
        }
    }

    #[test]
    fn classify_pr_closed_unmerged() {
        let s = DispatchSnapshot {
            issue: 100,
            state: "OPEN".into(),
            title: "stub".into(),
            copilot_ever_assigned: true,
            copilot_connected: true,
            copilot_connected_at: Some("2026-05-15T10:59:04Z".into()),
            linked_copilot_prs: vec![LinkedPr {
                number: 2961,
                state: "CLOSED".into(),
                merged: false,
                merged_at: None,
                head_ref: "copilot/x".into(),
            }],
            copilot_comments: 0,
            candidate_branches: Vec::new(),
        };
        match classify(&s) {
            DispatchSignal::PrClosedUnmerged { pr, head_ref } => {
                assert_eq!(pr, 2961);
                assert_eq!(head_ref, "copilot/x");
            }
            other => panic!("expected PrClosedUnmerged, got {other:?}"),
        }
    }

    #[test]
    fn recommended_action_for_each_signal_kind() {
        // Confirm every variant of DispatchSignal has a non-empty
        // recommended_action — guards against missing arms if the enum grows.
        let signals = vec![
            DispatchSignal::NotFound,
            DispatchSignal::NotDispatched,
            DispatchSignal::DispatchedNoConnect,
            DispatchSignal::ConnectedNoOutput {
                connected_at: "x".into(),
            },
            DispatchSignal::BranchCandidates {
                connected_at: "x".into(),
                branches: Vec::new(),
                copilot_comments: 0,
            },
            DispatchSignal::PrOpen {
                pr: 1,
                head_ref: "copilot/x".into(),
            },
            DispatchSignal::PrClosedUnmerged {
                pr: 1,
                head_ref: "copilot/x".into(),
            },
            DispatchSignal::PrMerged {
                pr: 1,
                head_ref: "copilot/x".into(),
                merged_at: "x".into(),
            },
        ];
        for s in &signals {
            let a = s.recommended_action();
            assert!(!a.is_empty(), "missing recommended_action for {s:?}");
        }
    }

    #[test]
    fn build_snapshot_skips_branch_probe_when_pr_linked() {
        let mut dispatch = HashMap::new();
        let raw = DispatchSnapshotRaw {
            issue: 100,
            state: "OPEN".into(),
            title: "stub".into(),
            copilot_ever_assigned: true,
            copilot_connected: true,
            copilot_connected_at: Some("2026-05-15T10:59:04Z".into()),
            linked_copilot_prs: vec![LinkedPr {
                number: 2961,
                state: "OPEN".into(),
                merged: false,
                merged_at: None,
                head_ref: "copilot/x".into(),
            }],
            copilot_comments: 0,
        };
        dispatch.insert(100, raw);
        // Branches probe would return junk if called — but it should NOT be called.
        let client = MockGh {
            dispatch,
            branches: vec![CopilotBranch {
                name: "copilot/should-not-appear".into(),
                head_sha: "x".into(),
                head_committed_at: "2026-05-15T11:02:57Z".into(),
            }],
        };
        let s = build_snapshot(&client, "EvaLok", "schema-org-json-ld", 100).unwrap();
        assert!(
            s.candidate_branches.is_empty(),
            "branch probe should be skipped when PR is linked"
        );
    }

    #[test]
    fn build_snapshot_skips_branch_probe_when_not_connected() {
        let mut dispatch = HashMap::new();
        dispatch.insert(100, empty_raw(100, "OPEN"));
        let client = MockGh {
            dispatch,
            branches: vec![CopilotBranch {
                name: "copilot/should-not-appear".into(),
                head_sha: "x".into(),
                head_committed_at: "x".into(),
            }],
        };
        let s = build_snapshot(&client, "o", "n", 100).unwrap();
        assert!(s.candidate_branches.is_empty());
    }

    #[test]
    fn build_snapshot_runs_branch_probe_when_connected_no_pr() {
        let mut dispatch = HashMap::new();
        dispatch.insert(
            100,
            DispatchSnapshotRaw {
                issue: 100,
                state: "OPEN".into(),
                title: "stub".into(),
                copilot_ever_assigned: true,
                copilot_connected: true,
                copilot_connected_at: Some("2026-05-15T10:59:04Z".into()),
                linked_copilot_prs: Vec::new(),
                copilot_comments: 0,
            },
        );
        let client = MockGh {
            dispatch,
            branches: vec![
                CopilotBranch {
                    name: "copilot/old".into(),
                    head_sha: "1".into(),
                    head_committed_at: "2026-01-01T00:00:00Z".into(),
                },
                CopilotBranch {
                    name: "copilot/new".into(),
                    head_sha: "2".into(),
                    head_committed_at: "2026-05-15T11:02:57Z".into(),
                },
            ],
        };
        let s = build_snapshot(&client, "o", "n", 100).unwrap();
        // Only the post-connect branch should appear.
        assert_eq!(s.candidate_branches.len(), 1);
        assert_eq!(s.candidate_branches[0].name, "copilot/new");
    }

    #[test]
    fn run_writes_human_output_with_signal_and_action() {
        let mut dispatch = HashMap::new();
        dispatch.insert(
            100,
            DispatchSnapshotRaw {
                issue: 100,
                state: "OPEN".into(),
                title: "stub".into(),
                copilot_ever_assigned: true,
                copilot_connected: true,
                copilot_connected_at: Some("2026-05-15T10:59:04Z".into()),
                linked_copilot_prs: Vec::new(),
                copilot_comments: 0,
            },
        );
        let client = MockGh {
            dispatch,
            branches: vec![CopilotBranch {
                name: "copilot/new".into(),
                head_sha: "ab12".into(),
                head_committed_at: "2026-05-15T11:02:57Z".into(),
            }],
        };
        let args = Args {
            repo: "EvaLok/schema-org-json-ld".into(),
            json: false,
            command: SubCmd::Check { issue: 100 },
        };
        let mut buf = Vec::new();
        let code = run(args, &client, &mut buf).unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert_eq!(code, 0);
        assert!(
            text.contains("signal: branch-candidates"),
            "text was: {text}"
        );
        assert!(
            text.contains("recommended action: absorb-from-branch"),
            "text was: {text}"
        );
        assert!(text.contains("copilot/new"));
    }

    #[test]
    fn run_writes_json_output_with_keys() {
        let mut dispatch = HashMap::new();
        dispatch.insert(100, empty_raw(100, "OPEN"));
        let client = MockGh {
            dispatch,
            branches: Vec::new(),
        };
        let args = Args {
            repo: "EvaLok/schema-org-json-ld".into(),
            json: true,
            command: SubCmd::Check { issue: 100 },
        };
        let mut buf = Vec::new();
        let code = run(args, &client, &mut buf).unwrap();
        let text = String::from_utf8(buf).unwrap();
        assert_eq!(code, 0);
        let parsed: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(parsed["signal_kind"].as_str(), Some("not-dispatched"));
        assert_eq!(
            parsed["recommended_action"].as_str(),
            Some("check-dispatch-tooling")
        );
        assert!(parsed["snapshot"].is_object());
    }

    #[test]
    fn run_propagates_not_found() {
        let mut dispatch = HashMap::new();
        dispatch.insert(99999, empty_raw(99999, "NOT_FOUND"));
        let client = MockGh {
            dispatch,
            branches: Vec::new(),
        };
        let args = Args {
            repo: "EvaLok/schema-org-json-ld".into(),
            json: true,
            command: SubCmd::Check { issue: 99999 },
        };
        let mut buf = Vec::new();
        run(args, &client, &mut buf).unwrap();
        let text = String::from_utf8(buf).unwrap();
        let parsed: Value = serde_json::from_str(&text).unwrap();
        assert_eq!(parsed["signal_kind"].as_str(), Some("not-found"));
    }

    #[test]
    fn signal_kind_strings_are_stable() {
        // Lock in the kebab-case strings — these become part of the tool's
        // observable contract (state.json, journal entries, scripts).
        assert_eq!(signal_kind(&DispatchSignal::NotFound), "not-found");
        assert_eq!(
            signal_kind(&DispatchSignal::NotDispatched),
            "not-dispatched"
        );
        assert_eq!(
            signal_kind(&DispatchSignal::DispatchedNoConnect),
            "dispatched-no-connect"
        );
        assert_eq!(
            signal_kind(&DispatchSignal::ConnectedNoOutput {
                connected_at: "x".into()
            }),
            "connected-no-output"
        );
        assert_eq!(
            signal_kind(&DispatchSignal::BranchCandidates {
                connected_at: "x".into(),
                branches: Vec::new(),
                copilot_comments: 0
            }),
            "branch-candidates"
        );
        assert_eq!(
            signal_kind(&DispatchSignal::PrOpen {
                pr: 1,
                head_ref: "x".into()
            }),
            "pr-open"
        );
        assert_eq!(
            signal_kind(&DispatchSignal::PrClosedUnmerged {
                pr: 1,
                head_ref: "x".into()
            }),
            "pr-closed-unmerged"
        );
        assert_eq!(
            signal_kind(&DispatchSignal::PrMerged {
                pr: 1,
                head_ref: "x".into(),
                merged_at: "x".into()
            }),
            "pr-merged"
        );
    }
}
