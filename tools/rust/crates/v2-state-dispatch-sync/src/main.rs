// v2-state-dispatch-sync
//
// Reconciles `docs/state.json` `agent_sessions[]` entries with `status: in_flight` against
// actual GitHub state, transitioning each to `merged` (with `pr` + `merged_at`) or
// `closed_without_pr` as appropriate, and updating the top-level `in_flight_sessions` counter.
//
// Extracts the cycle 153 Track 2 manual housekeeping pattern (11 sequential Edits on state.json)
// into a reusable tool. Aligns with CORE-DESIGN-PRINCIPLE: bounded-mechanical procedural work
// belongs in a tool, not in the orchestrator's per-cycle scratch space.

use clap::{Parser, Subcommand};
use serde_json::{Map, Value};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

#[derive(Parser, Debug)]
#[command(
    name = "v2-state-dispatch-sync",
    about = "Reconcile state.json agent_sessions in_flight entries against GitHub state"
)]
struct Args {
    /// Path to state.json
    #[arg(long, default_value = "docs/state.json")]
    state_file: PathBuf,

    /// Repository (owner/name)
    #[arg(long, default_value = "EvaLok/schema-org-json-ld")]
    repo: String,

    /// Optional scope: comma-separated issue numbers to consider (default: all in_flight)
    #[arg(long, value_delimiter = ',')]
    issues: Option<Vec<u64>>,

    #[command(subcommand)]
    command: SubCmd,
}

#[derive(Subcommand, Debug)]
enum SubCmd {
    /// Print planned changes (read-only)
    Audit,
    /// Apply changes to state.json
    Sync,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum SyncError {
    Io(String),
    Json(String),
    Schema(String),
    Gh(String),
    Repo(String),
}

impl std::fmt::Display for SyncError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyncError::Io(s) => write!(f, "io: {s}"),
            SyncError::Json(s) => write!(f, "json: {s}"),
            SyncError::Schema(s) => write!(f, "schema: {s}"),
            SyncError::Gh(s) => write!(f, "gh: {s}"),
            SyncError::Repo(s) => write!(f, "repo: {s}"),
        }
    }
}

impl From<std::io::Error> for SyncError {
    fn from(e: std::io::Error) -> Self {
        SyncError::Io(e.to_string())
    }
}

impl From<serde_json::Error> for SyncError {
    fn from(e: serde_json::Error) -> Self {
        SyncError::Json(e.to_string())
    }
}

/// Live GitHub state of one issue, distilled to what status-computation needs.
#[derive(Debug, Clone, PartialEq, Eq)]
struct IssueStatus {
    /// "OPEN" or "CLOSED"
    state: String,
    /// PR that closed this issue via "Closes #N" or similar reference; None if the issue
    /// is open or closed without a closing PR linkage. If multiple PRs reference the issue,
    /// the first merged one wins (deterministic by GitHub's return order).
    closing_pr: Option<ClosingPr>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ClosingPr {
    number: u64,
    merged_at: String,
}

/// A planned status transition for one in-flight entry.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Plan {
    issue: u64,
    /// "in_flight" → one of "in_flight" (no change), "closed_without_pr", "merged"
    new_status: String,
    /// Only set when `new_status == "merged"`.
    pr: Option<u64>,
    merged_at: Option<String>,
}

/// Abstraction over the GitHub query. The CLI uses `GhCliClient`; tests use `MockGhClient`.
trait GhClient {
    fn query_issue(&self, owner: &str, name: &str, issue: u64) -> Result<IssueStatus, SyncError>;
}

struct GhCliClient;

const ISSUE_QUERY: &str = r#"
query($owner: String!, $name: String!, $number: Int!) {
  repository(owner: $owner, name: $name) {
    issue(number: $number) {
      state
      closedByPullRequestsReferences(first: 10, includeClosedPrs: true, userLinkedOnly: false) {
        nodes {
          number
          merged
          mergedAt
        }
      }
    }
  }
}
"#;

impl GhClient for GhCliClient {
    fn query_issue(&self, owner: &str, name: &str, issue: u64) -> Result<IssueStatus, SyncError> {
        let output = Command::new("gh")
            .args([
                "api",
                "graphql",
                "-f",
                &format!("query={ISSUE_QUERY}"),
                "-F",
                &format!("owner={owner}"),
                "-F",
                &format!("name={name}"),
                "-F",
                &format!("number={issue}"),
            ])
            .output()
            .map_err(|e| SyncError::Gh(format!("invoking gh: {e}")))?;

        if !output.status.success() {
            return Err(SyncError::Gh(format!(
                "gh api graphql exit {:?}: {}",
                output.status.code(),
                String::from_utf8_lossy(&output.stderr)
            )));
        }

        let body: Value = serde_json::from_slice(&output.stdout)?;
        parse_issue_response(&body, issue)
    }
}

fn parse_issue_response(body: &Value, issue: u64) -> Result<IssueStatus, SyncError> {
    let issue_obj = body
        .pointer("/data/repository/issue")
        .ok_or_else(|| SyncError::Gh(format!("issue {issue}: response missing data.repository.issue")))?;

    if issue_obj.is_null() {
        return Err(SyncError::Gh(format!("issue {issue}: not found")));
    }

    let state = issue_obj
        .get("state")
        .and_then(|v| v.as_str())
        .ok_or_else(|| SyncError::Gh(format!("issue {issue}: missing state field")))?
        .to_string();

    let mut closing_pr = None;
    if let Some(nodes) = issue_obj
        .pointer("/closedByPullRequestsReferences/nodes")
        .and_then(|v| v.as_array())
    {
        for node in nodes {
            let merged = node.get("merged").and_then(|v| v.as_bool()).unwrap_or(false);
            if !merged {
                continue;
            }
            let number = match node.get("number").and_then(|v| v.as_u64()) {
                Some(n) => n,
                None => continue,
            };
            let merged_at = match node.get("mergedAt").and_then(|v| v.as_str()) {
                Some(s) => s.to_string(),
                None => continue,
            };
            closing_pr = Some(ClosingPr { number, merged_at });
            break; // first merged PR wins
        }
    }

    Ok(IssueStatus { state, closing_pr })
}

fn read_state(path: &Path) -> Result<Value, SyncError> {
    let bytes = std::fs::read(path)?;
    let value: Value = serde_json::from_slice(&bytes)?;
    if !value.is_object() {
        return Err(SyncError::Schema(format!(
            "{}: top-level value is not an object",
            path.display()
        )));
    }
    Ok(value)
}

fn write_state(path: &Path, value: &Value) -> Result<(), SyncError> {
    let mut serialized = serde_json::to_string_pretty(value)?;
    serialized.push('\n');
    std::fs::write(path, serialized)?;
    Ok(())
}

/// Return the list of `agent_sessions[]` entries with `status == "in_flight"`, in original order.
/// Each entry's `issue` field is required (u64); entries missing or malformed are skipped with a warning.
fn find_in_flight(state: &Value) -> Result<Vec<u64>, SyncError> {
    let sessions = state
        .get("agent_sessions")
        .and_then(|v| v.as_array())
        .ok_or_else(|| SyncError::Schema("state.json missing agent_sessions array".to_string()))?;

    let mut out = Vec::new();
    for entry in sessions {
        let status = match entry.get("status").and_then(|v| v.as_str()) {
            Some(s) => s,
            None => continue,
        };
        if status != "in_flight" {
            continue;
        }
        let issue = match entry.get("issue").and_then(|v| v.as_u64()) {
            Some(n) => n,
            None => continue,
        };
        out.push(issue);
    }
    Ok(out)
}

/// For each in-flight issue, query GitHub and compute the planned transition.
/// `scope` (if provided) restricts the planning to that subset.
fn compute_plan(
    state: &Value,
    client: &dyn GhClient,
    owner: &str,
    name: &str,
    scope: Option<&[u64]>,
) -> Result<Vec<Plan>, SyncError> {
    let in_flight = find_in_flight(state)?;
    let scope_set: Option<std::collections::HashSet<u64>> = scope.map(|s| s.iter().copied().collect());

    let mut plans = Vec::new();
    for issue in in_flight {
        if let Some(scope) = &scope_set {
            if !scope.contains(&issue) {
                continue;
            }
        }
        let status = client.query_issue(owner, name, issue)?;
        let plan = match status.state.as_str() {
            "OPEN" => Plan {
                issue,
                new_status: "in_flight".to_string(),
                pr: None,
                merged_at: None,
            },
            "CLOSED" => match status.closing_pr {
                Some(pr) => Plan {
                    issue,
                    new_status: "merged".to_string(),
                    pr: Some(pr.number),
                    merged_at: Some(pr.merged_at),
                },
                None => Plan {
                    issue,
                    new_status: "closed_without_pr".to_string(),
                    pr: None,
                    merged_at: None,
                },
            },
            other => {
                return Err(SyncError::Gh(format!(
                    "issue {issue}: unexpected state {other:?} (expected OPEN or CLOSED)"
                )));
            }
        };
        plans.push(plan);
    }
    Ok(plans)
}

/// Apply plans to `state` in place. Returns the count of changes that produced an actual mutation
/// (i.e. excludes plans where `new_status == "in_flight"` which is a no-op).
fn apply_plan(state: &mut Value, plans: &[Plan]) -> Result<usize, SyncError> {
    let plan_map: HashMap<u64, &Plan> = plans
        .iter()
        .filter(|p| p.new_status != "in_flight")
        .map(|p| (p.issue, p))
        .collect();

    if plan_map.is_empty() {
        // Still recompute in_flight_sessions to stay self-consistent.
        let count = recount_in_flight(state)?;
        update_in_flight_sessions(state, count)?;
        return Ok(0);
    }

    let sessions = state
        .get_mut("agent_sessions")
        .and_then(|v| v.as_array_mut())
        .ok_or_else(|| SyncError::Schema("state.json missing agent_sessions array".to_string()))?;

    let mut mutated = 0usize;
    for entry in sessions.iter_mut() {
        let issue = match entry.get("issue").and_then(|v| v.as_u64()) {
            Some(n) => n,
            None => continue,
        };
        let plan = match plan_map.get(&issue) {
            Some(p) => *p,
            None => continue,
        };
        let obj = entry
            .as_object_mut()
            .ok_or_else(|| SyncError::Schema(format!("agent_sessions entry for #{issue} is not an object")))?;
        mutate_entry(obj, plan)?;
        mutated += 1;
    }

    let count = recount_in_flight(state)?;
    update_in_flight_sessions(state, count)?;
    Ok(mutated)
}

fn mutate_entry(obj: &mut Map<String, Value>, plan: &Plan) -> Result<(), SyncError> {
    match plan.new_status.as_str() {
        "merged" => {
            obj.insert("status".to_string(), Value::String("merged".to_string()));
            if let Some(pr) = plan.pr {
                obj.insert("pr".to_string(), Value::Number(pr.into()));
            }
            if let Some(ts) = &plan.merged_at {
                obj.insert("merged_at".to_string(), Value::String(ts.clone()));
            }
        }
        "closed_without_pr" => {
            obj.insert(
                "status".to_string(),
                Value::String("closed_without_pr".to_string()),
            );
            // Defensive: if a prior bad write left pr / merged_at, scrub them.
            obj.remove("pr");
            obj.remove("merged_at");
        }
        other => {
            return Err(SyncError::Schema(format!(
                "mutate_entry: unsupported new_status {other:?}"
            )));
        }
    }
    Ok(())
}

fn recount_in_flight(state: &Value) -> Result<u64, SyncError> {
    let sessions = state
        .get("agent_sessions")
        .and_then(|v| v.as_array())
        .ok_or_else(|| SyncError::Schema("state.json missing agent_sessions array".to_string()))?;
    let count = sessions
        .iter()
        .filter(|e| e.get("status").and_then(|v| v.as_str()) == Some("in_flight"))
        .count() as u64;
    Ok(count)
}

fn update_in_flight_sessions(state: &mut Value, count: u64) -> Result<(), SyncError> {
    let obj = state
        .as_object_mut()
        .ok_or_else(|| SyncError::Schema("state.json top-level is not an object".to_string()))?;
    obj.insert("in_flight_sessions".to_string(), Value::Number(count.into()));
    Ok(())
}

fn print_plan(plans: &[Plan], in_flight_after: u64) {
    if plans.is_empty() {
        println!("No in_flight agent_sessions entries found.");
        return;
    }
    let unchanged: Vec<&Plan> = plans.iter().filter(|p| p.new_status == "in_flight").collect();
    let changed: Vec<&Plan> = plans.iter().filter(|p| p.new_status != "in_flight").collect();

    println!("Plan: {} entries audited, {} transitions, {} unchanged", plans.len(), changed.len(), unchanged.len());
    println!();
    for p in &changed {
        match p.new_status.as_str() {
            "merged" => {
                let pr = p.pr.map(|n| n.to_string()).unwrap_or_else(|| "?".to_string());
                let ts = p.merged_at.clone().unwrap_or_else(|| "?".to_string());
                println!("  #{}: in_flight → merged (pr=#{pr}, merged_at={ts})", p.issue);
            }
            "closed_without_pr" => {
                println!("  #{}: in_flight → closed_without_pr", p.issue);
            }
            other => {
                println!("  #{}: in_flight → {other} (unexpected)", p.issue);
            }
        }
    }
    if !unchanged.is_empty() {
        println!();
        println!("Unchanged (still open on GitHub):");
        for p in &unchanged {
            println!("  #{}: in_flight (no change)", p.issue);
        }
    }
    println!();
    println!("Resulting in_flight_sessions count: {in_flight_after}");
}

fn parse_repo(repo: &str) -> Result<(String, String), SyncError> {
    let mut parts = repo.splitn(2, '/');
    let owner = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| SyncError::Repo(format!("repo {repo:?} missing owner")))?;
    let name = parts
        .next()
        .filter(|s| !s.is_empty())
        .ok_or_else(|| SyncError::Repo(format!("repo {repo:?} missing name")))?;
    Ok((owner.to_string(), name.to_string()))
}

fn run(args: Args, client: &dyn GhClient) -> Result<i32, SyncError> {
    let (owner, name) = parse_repo(&args.repo)?;
    let mut state = read_state(&args.state_file)?;
    let scope = args.issues.as_deref();
    let plans = compute_plan(&state, client, &owner, &name, scope)?;

    match args.command {
        SubCmd::Audit => {
            // Compute hypothetical in_flight_sessions count without mutating state on disk.
            let mut preview = state.clone();
            apply_plan(&mut preview, &plans)?;
            let count = preview
                .get("in_flight_sessions")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            print_plan(&plans, count);
            Ok(0)
        }
        SubCmd::Sync => {
            let mutated = apply_plan(&mut state, &plans)?;
            write_state(&args.state_file, &state)?;
            let count = state
                .get("in_flight_sessions")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);
            print_plan(&plans, count);
            println!();
            println!("Wrote {} ({mutated} entries mutated).", args.state_file.display());
            Ok(0)
        }
    }
}

fn main() -> ExitCode {
    let args = Args::parse();
    let client = GhCliClient;
    match run(args, &client) {
        Ok(code) => {
            if code == 0 {
                ExitCode::SUCCESS
            } else {
                ExitCode::from(code as u8)
            }
        }
        Err(e) => {
            eprintln!("v2-state-dispatch-sync: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test double: returns canned responses keyed by issue number.
    struct MockGh {
        responses: HashMap<u64, IssueStatus>,
    }

    impl GhClient for MockGh {
        fn query_issue(&self, _owner: &str, _name: &str, issue: u64) -> Result<IssueStatus, SyncError> {
            self.responses
                .get(&issue)
                .cloned()
                .ok_or_else(|| SyncError::Gh(format!("mock: no canned response for issue {issue}")))
        }
    }

    fn state_with_sessions(entries: Vec<Value>) -> Value {
        serde_json::json!({
            "in_flight_sessions": entries.iter().filter(|e| e["status"] == "in_flight").count(),
            "agent_sessions": entries,
            "other_field": "preserved",
        })
    }

    fn session(issue: u64, status: &str) -> Value {
        serde_json::json!({
            "dispatched_at": "2026-05-01T00:00:00Z",
            "issue": issue,
            "model": "gpt-5.4",
            "status": status,
            "title": format!("dispatch #{issue}")
        })
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
    fn find_in_flight_returns_issue_numbers_in_order() {
        let state = state_with_sessions(vec![
            session(2950, "closed_without_pr"),
            session(2960, "in_flight"),
            session(2961, "in_flight"),
            session(2962, "merged"),
        ]);
        let found = find_in_flight(&state).unwrap();
        assert_eq!(found, vec![2960, 2961]);
    }

    #[test]
    fn find_in_flight_errors_when_agent_sessions_missing() {
        let state = serde_json::json!({"in_flight_sessions": 0});
        assert!(find_in_flight(&state).is_err());
    }

    #[test]
    fn find_in_flight_skips_entries_without_issue_number() {
        let state = serde_json::json!({
            "agent_sessions": [
                {"status": "in_flight"},
                {"status": "in_flight", "issue": 2960},
                {"status": "in_flight", "issue": "not-a-number"},
            ]
        });
        let found = find_in_flight(&state).unwrap();
        assert_eq!(found, vec![2960]);
    }

    #[test]
    fn parse_issue_response_open_issue() {
        let body = serde_json::json!({
            "data": {"repository": {"issue": {
                "state": "OPEN",
                "closedByPullRequestsReferences": {"nodes": []}
            }}}
        });
        let s = parse_issue_response(&body, 2960).unwrap();
        assert_eq!(s.state, "OPEN");
        assert!(s.closing_pr.is_none());
    }

    #[test]
    fn parse_issue_response_closed_without_pr() {
        let body = serde_json::json!({
            "data": {"repository": {"issue": {
                "state": "CLOSED",
                "closedByPullRequestsReferences": {"nodes": []}
            }}}
        });
        let s = parse_issue_response(&body, 2950).unwrap();
        assert_eq!(s.state, "CLOSED");
        assert!(s.closing_pr.is_none());
    }

    #[test]
    fn parse_issue_response_closed_via_merged_pr() {
        let body = serde_json::json!({
            "data": {"repository": {"issue": {
                "state": "CLOSED",
                "closedByPullRequestsReferences": {"nodes": [
                    {"number": 2953, "merged": true, "mergedAt": "2026-05-15T03:14:13Z"}
                ]}
            }}}
        });
        let s = parse_issue_response(&body, 2952).unwrap();
        assert_eq!(s.state, "CLOSED");
        let pr = s.closing_pr.expect("expected closing PR");
        assert_eq!(pr.number, 2953);
        assert_eq!(pr.merged_at, "2026-05-15T03:14:13Z");
    }

    #[test]
    fn parse_issue_response_skips_unmerged_prs() {
        let body = serde_json::json!({
            "data": {"repository": {"issue": {
                "state": "CLOSED",
                "closedByPullRequestsReferences": {"nodes": [
                    {"number": 2998, "merged": false, "mergedAt": null},
                    {"number": 2999, "merged": true, "mergedAt": "2026-05-15T10:00:00Z"}
                ]}
            }}}
        });
        let s = parse_issue_response(&body, 2997).unwrap();
        let pr = s.closing_pr.expect("expected merged PR to win");
        assert_eq!(pr.number, 2999);
    }

    #[test]
    fn parse_issue_response_errors_on_missing_issue() {
        let body = serde_json::json!({"data": {"repository": {"issue": null}}});
        assert!(parse_issue_response(&body, 9999).is_err());
    }

    #[test]
    fn parse_issue_response_errors_on_missing_data() {
        let body = serde_json::json!({"errors": [{"message": "Not found"}]});
        assert!(parse_issue_response(&body, 9999).is_err());
    }

    #[test]
    fn compute_plan_open_issue_no_change() {
        let state = state_with_sessions(vec![session(2960, "in_flight")]);
        let mock = MockGh {
            responses: HashMap::from([(
                2960,
                IssueStatus {
                    state: "OPEN".to_string(),
                    closing_pr: None,
                },
            )]),
        };
        let plans = compute_plan(&state, &mock, "EvaLok", "schema-org-json-ld", None).unwrap();
        assert_eq!(plans.len(), 1);
        assert_eq!(plans[0].issue, 2960);
        assert_eq!(plans[0].new_status, "in_flight");
        assert!(plans[0].pr.is_none());
    }

    #[test]
    fn compute_plan_closed_without_pr() {
        let state = state_with_sessions(vec![session(2950, "in_flight")]);
        let mock = MockGh {
            responses: HashMap::from([(
                2950,
                IssueStatus {
                    state: "CLOSED".to_string(),
                    closing_pr: None,
                },
            )]),
        };
        let plans = compute_plan(&state, &mock, "EvaLok", "schema-org-json-ld", None).unwrap();
        assert_eq!(plans[0].new_status, "closed_without_pr");
    }

    #[test]
    fn compute_plan_merged() {
        let state = state_with_sessions(vec![session(2952, "in_flight")]);
        let mock = MockGh {
            responses: HashMap::from([(
                2952,
                IssueStatus {
                    state: "CLOSED".to_string(),
                    closing_pr: Some(ClosingPr {
                        number: 2953,
                        merged_at: "2026-05-15T03:14:13Z".to_string(),
                    }),
                },
            )]),
        };
        let plans = compute_plan(&state, &mock, "EvaLok", "schema-org-json-ld", None).unwrap();
        assert_eq!(plans[0].new_status, "merged");
        assert_eq!(plans[0].pr, Some(2953));
        assert_eq!(plans[0].merged_at.as_deref(), Some("2026-05-15T03:14:13Z"));
    }

    #[test]
    fn compute_plan_scope_filter() {
        let state = state_with_sessions(vec![
            session(2950, "in_flight"),
            session(2952, "in_flight"),
            session(2960, "in_flight"),
        ]);
        let mock = MockGh {
            responses: HashMap::from([
                (
                    2950,
                    IssueStatus {
                        state: "CLOSED".to_string(),
                        closing_pr: None,
                    },
                ),
                (
                    2952,
                    IssueStatus {
                        state: "CLOSED".to_string(),
                        closing_pr: None,
                    },
                ),
            ]),
        };
        let plans = compute_plan(
            &state,
            &mock,
            "EvaLok",
            "schema-org-json-ld",
            Some(&[2950, 2952]),
        )
        .unwrap();
        assert_eq!(plans.len(), 2);
        assert!(plans.iter().all(|p| p.issue != 2960));
    }

    #[test]
    fn apply_plan_merges_and_updates_count() {
        let mut state = state_with_sessions(vec![
            session(2952, "in_flight"),
            session(2960, "in_flight"),
        ]);
        let plans = vec![
            Plan {
                issue: 2952,
                new_status: "merged".to_string(),
                pr: Some(2953),
                merged_at: Some("2026-05-15T03:14:13Z".to_string()),
            },
            Plan {
                issue: 2960,
                new_status: "in_flight".to_string(),
                pr: None,
                merged_at: None,
            },
        ];
        let mutated = apply_plan(&mut state, &plans).unwrap();
        assert_eq!(mutated, 1);
        assert_eq!(state["in_flight_sessions"], 1);
        let entry_2952 = &state["agent_sessions"][0];
        assert_eq!(entry_2952["status"], "merged");
        assert_eq!(entry_2952["pr"], 2953);
        assert_eq!(entry_2952["merged_at"], "2026-05-15T03:14:13Z");
        let entry_2960 = &state["agent_sessions"][1];
        assert_eq!(entry_2960["status"], "in_flight");
    }

    #[test]
    fn apply_plan_closes_without_pr_and_scrubs_stale_fields() {
        let mut state = serde_json::json!({
            "in_flight_sessions": 1,
            "agent_sessions": [
                {
                    "dispatched_at": "2026-05-01T00:00:00Z",
                    "issue": 2950,
                    "model": "gpt-5.4",
                    "status": "in_flight",
                    "title": "stale",
                    "pr": 9999,
                    "merged_at": "should-be-scrubbed"
                }
            ]
        });
        let plans = vec![Plan {
            issue: 2950,
            new_status: "closed_without_pr".to_string(),
            pr: None,
            merged_at: None,
        }];
        apply_plan(&mut state, &plans).unwrap();
        assert_eq!(state["in_flight_sessions"], 0);
        let entry = &state["agent_sessions"][0];
        assert_eq!(entry["status"], "closed_without_pr");
        assert!(entry.get("pr").is_none());
        assert!(entry.get("merged_at").is_none());
    }

    #[test]
    fn apply_plan_preserves_unrelated_entries_and_fields() {
        let mut state = serde_json::json!({
            "in_flight_sessions": 2,
            "agent_sessions": [
                {"issue": 100, "status": "merged", "pr": 101},
                {"issue": 200, "status": "in_flight", "title": "to-close"},
                {"issue": 300, "status": "in_flight", "title": "still-open"},
            ],
            "other_top_level": "preserved",
        });
        let plans = vec![
            Plan {
                issue: 200,
                new_status: "closed_without_pr".to_string(),
                pr: None,
                merged_at: None,
            },
            Plan {
                issue: 300,
                new_status: "in_flight".to_string(),
                pr: None,
                merged_at: None,
            },
        ];
        apply_plan(&mut state, &plans).unwrap();
        assert_eq!(state["in_flight_sessions"], 1);
        assert_eq!(state["other_top_level"], "preserved");
        // Unrelated merged entry unchanged
        assert_eq!(state["agent_sessions"][0]["status"], "merged");
        assert_eq!(state["agent_sessions"][0]["pr"], 101);
        // Closed entry mutated
        assert_eq!(state["agent_sessions"][1]["status"], "closed_without_pr");
        assert_eq!(state["agent_sessions"][1]["title"], "to-close");
        // Still-open entry untouched
        assert_eq!(state["agent_sessions"][2]["status"], "in_flight");
    }

    #[test]
    fn apply_plan_with_empty_plans_still_normalizes_count() {
        let mut state = serde_json::json!({
            "in_flight_sessions": 999,
            "agent_sessions": [
                {"issue": 100, "status": "merged"},
                {"issue": 200, "status": "in_flight"},
            ]
        });
        let mutated = apply_plan(&mut state, &[]).unwrap();
        assert_eq!(mutated, 0);
        assert_eq!(state["in_flight_sessions"], 1);
    }

    #[test]
    fn read_write_roundtrip_preserves_structure() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state.json");
        let original = serde_json::json!({
            "in_flight_sessions": 1,
            "agent_sessions": [session(2960, "in_flight")],
            "other": "field"
        });
        write_state(&path, &original).unwrap();
        let read_back = read_state(&path).unwrap();
        assert_eq!(read_back, original);
    }

    #[test]
    fn read_state_errors_on_non_object_top_level() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("state.json");
        std::fs::write(&path, "[]").unwrap();
        assert!(read_state(&path).is_err());
    }

    #[test]
    fn run_audit_prints_plan_without_mutating_state_file() {
        let dir = tempfile::tempdir().unwrap();
        let state_path = dir.path().join("state.json");
        let state = state_with_sessions(vec![
            session(2950, "in_flight"),
            session(2952, "in_flight"),
            session(2960, "in_flight"),
        ]);
        write_state(&state_path, &state).unwrap();

        let mock = MockGh {
            responses: HashMap::from([
                (
                    2950,
                    IssueStatus {
                        state: "CLOSED".to_string(),
                        closing_pr: None,
                    },
                ),
                (
                    2952,
                    IssueStatus {
                        state: "CLOSED".to_string(),
                        closing_pr: Some(ClosingPr {
                            number: 2953,
                            merged_at: "2026-05-15T03:14:13Z".to_string(),
                        }),
                    },
                ),
                (
                    2960,
                    IssueStatus {
                        state: "OPEN".to_string(),
                        closing_pr: None,
                    },
                ),
            ]),
        };

        let args = Args {
            state_file: state_path.clone(),
            repo: "EvaLok/schema-org-json-ld".to_string(),
            issues: None,
            command: SubCmd::Audit,
        };
        let code = run(args, &mock).unwrap();
        assert_eq!(code, 0);

        // State file must be unchanged on disk.
        let after = read_state(&state_path).unwrap();
        assert_eq!(after, state);
    }

    #[test]
    fn run_sync_writes_updates_to_state_file() {
        let dir = tempfile::tempdir().unwrap();
        let state_path = dir.path().join("state.json");
        let state = state_with_sessions(vec![
            session(2950, "in_flight"),
            session(2952, "in_flight"),
            session(2960, "in_flight"),
        ]);
        write_state(&state_path, &state).unwrap();

        let mock = MockGh {
            responses: HashMap::from([
                (
                    2950,
                    IssueStatus {
                        state: "CLOSED".to_string(),
                        closing_pr: None,
                    },
                ),
                (
                    2952,
                    IssueStatus {
                        state: "CLOSED".to_string(),
                        closing_pr: Some(ClosingPr {
                            number: 2953,
                            merged_at: "2026-05-15T03:14:13Z".to_string(),
                        }),
                    },
                ),
                (
                    2960,
                    IssueStatus {
                        state: "OPEN".to_string(),
                        closing_pr: None,
                    },
                ),
            ]),
        };

        let args = Args {
            state_file: state_path.clone(),
            repo: "EvaLok/schema-org-json-ld".to_string(),
            issues: None,
            command: SubCmd::Sync,
        };
        let code = run(args, &mock).unwrap();
        assert_eq!(code, 0);

        let after = read_state(&state_path).unwrap();
        assert_eq!(after["in_flight_sessions"], 1);
        // 2950 closed_without_pr
        assert_eq!(after["agent_sessions"][0]["status"], "closed_without_pr");
        // 2952 merged
        assert_eq!(after["agent_sessions"][1]["status"], "merged");
        assert_eq!(after["agent_sessions"][1]["pr"], 2953);
        assert_eq!(
            after["agent_sessions"][1]["merged_at"],
            "2026-05-15T03:14:13Z"
        );
        // 2960 still in_flight
        assert_eq!(after["agent_sessions"][2]["status"], "in_flight");
    }

    #[test]
    fn run_sync_with_scope_only_touches_scoped_issues() {
        let dir = tempfile::tempdir().unwrap();
        let state_path = dir.path().join("state.json");
        let state = state_with_sessions(vec![
            session(2950, "in_flight"),
            session(2952, "in_flight"),
        ]);
        write_state(&state_path, &state).unwrap();

        let mock = MockGh {
            responses: HashMap::from([(
                2950,
                IssueStatus {
                    state: "CLOSED".to_string(),
                    closing_pr: None,
                },
            )]),
        };

        let args = Args {
            state_file: state_path.clone(),
            repo: "EvaLok/schema-org-json-ld".to_string(),
            issues: Some(vec![2950]),
            command: SubCmd::Sync,
        };
        let code = run(args, &mock).unwrap();
        assert_eq!(code, 0);

        let after = read_state(&state_path).unwrap();
        assert_eq!(after["agent_sessions"][0]["status"], "closed_without_pr");
        // 2952 should still be in_flight (not in scope)
        assert_eq!(after["agent_sessions"][1]["status"], "in_flight");
        assert_eq!(after["in_flight_sessions"], 1);
    }
}
