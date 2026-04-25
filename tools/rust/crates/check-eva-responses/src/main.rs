use chrono::{DateTime, SecondsFormat, Utc};
use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use state_schema::{check_version, read_state_value, StateJson};
use std::collections::{HashMap, HashSet};
use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const EVA_LOGIN: &str = "EvaLok";
const ZERO_TIME: &str = "1970-01-01T00:00:00Z";
const EXCERPT_LIMIT: usize = 200;
/// Comma-separated logins added to the default orchestrator author set.
const ORCHESTRATOR_LOGINS_ENV: &str = "CHECK_EVA_RESPONSES_ORCHESTRATOR_LOGINS";
/// Regex used against the first non-empty line to detect signed orchestrator comments.
const ORCHESTRATOR_SIGNATURE_ENV: &str = "CHECK_EVA_RESPONSES_ORCHESTRATOR_SIGNATURE";
const DEFAULT_ORCHESTRATOR_LOGINS: [&str; 3] = ["claude[bot]", "app/claude", "claude-bot"];
const DEFAULT_ORCHESTRATOR_SIGNATURE_PREFIX: &str = "> **[main-orchestrator]**";

#[derive(Debug, Parser)]
#[command(name = "check-eva-responses")]
struct Cli {
    /// Path to the repository root
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Emit JSON instead of human-readable summary
    #[arg(long)]
    json: bool,

    /// Optional RFC 3339 timestamp cutoff
    #[arg(long)]
    since: Option<String>,

    /// Include closed question-for-eva issues
    #[arg(long)]
    include_closed: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ExecutionResult {
    exit_code: Option<i32>,
    stdout: String,
    stderr: String,
}

trait CommandRunner {
    fn gh(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String>;
}

struct ProcessRunner;

impl CommandRunner for ProcessRunner {
    fn gh(&self, repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
        let output = Command::new("gh")
            .current_dir(repo_root)
            .args(args)
            .output()
            .map_err(|error| format!("failed to execute gh {}: {}", args.join(" "), error))?;
        Ok(ExecutionResult {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).to_string(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct QuestionForEvaIssue {
    number: u64,
    title: String,
    state: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct IssueComment {
    author_login: String,
    created_at: DateTime<Utc>,
    url: String,
    body: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct NewEvaResponse {
    issue: u64,
    title: String,
    eva_comment_at: String,
    eva_comment_url: String,
    eva_excerpt: String,
    orchestrator_last_comment_at: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct Report {
    since: String,
    checked_issues: usize,
    new_responses: Vec<NewEvaResponse>,
}

#[derive(Debug)]
struct OrchestratorDetector {
    logins: HashSet<String>,
    signature: Regex,
}

fn main() {
    let cli = Cli::parse();
    let runner = ProcessRunner;
    let orchestrator_detector = match orchestrator_detector_from_env() {
        Ok(detector) => detector,
        Err(error) => {
            eprintln!("check-eva-responses error: {}", error);
            std::process::exit(1);
        }
    };

    match execute(&cli, &runner, &orchestrator_detector) {
        Ok(output) => println!("{}", output),
        Err(error) => {
            eprintln!("check-eva-responses error: {}", error);
            std::process::exit(1);
        }
    }
}

fn execute(
    cli: &Cli,
    runner: &dyn CommandRunner,
    orchestrator_detector: &OrchestratorDetector,
) -> Result<String, String> {
    let since = resolve_since(cli)?;
    let report = collect_report(
        &cli.repo_root,
        runner,
        since,
        cli.include_closed,
        orchestrator_detector,
    )?;

    if cli.json {
        serde_json::to_string_pretty(&report)
            .map_err(|error| format!("failed to serialize report JSON: {}", error))
    } else {
        Ok(render_human_report(&report))
    }
}

fn resolve_since(cli: &Cli) -> Result<DateTime<Utc>, String> {
    if let Some(since) = &cli.since {
        return parse_timestamp(since)
            .map_err(|error| format!("invalid --since timestamp {:?}: {}", since, error));
    }

    let raw = if let Some(since) = read_state(&cli.repo_root)?.last_eva_comment_check {
        since
    } else {
        ZERO_TIME.to_string()
    };

    parse_timestamp(&raw).map_err(|error| {
        format!(
            "invalid docs/state.json last_eva_comment_check {:?}: {}",
            raw, error
        )
    })
}

fn read_state(repo_root: &Path) -> Result<StateJson, String> {
    let value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(value)
        .map_err(|error| format!("failed to deserialize docs/state.json: {}", error))?;
    check_version(&state)?;
    Ok(state)
}

fn collect_report(
    repo_root: &Path,
    runner: &dyn CommandRunner,
    since: DateTime<Utc>,
    include_closed: bool,
    orchestrator_detector: &OrchestratorDetector,
) -> Result<Report, String> {
    let issues = fetch_question_for_eva_issues(repo_root, runner, include_closed)?;
    let mut issue_comments = HashMap::new();

    for issue in &issues {
        let comments = fetch_issue_comments(repo_root, runner, issue.number)?;
        issue_comments.insert(issue.number, comments);
    }

    Ok(Report {
        since: format_timestamp(since),
        checked_issues: issues
            .iter()
            .filter(|issue| include_closed || !is_closed_issue(issue))
            .count(),
        new_responses: classify_responses(
            &issues,
            &issue_comments,
            since,
            include_closed,
            orchestrator_detector,
        ),
    })
}

fn fetch_question_for_eva_issues(
    repo_root: &Path,
    runner: &dyn CommandRunner,
    include_closed: bool,
) -> Result<Vec<QuestionForEvaIssue>, String> {
    let state = if include_closed { "all" } else { "open" };
    let value = gh_json(
        repo_root,
        runner,
        &[
            "issue".to_string(),
            "list".to_string(),
            "--repo".to_string(),
            MAIN_REPO.to_string(),
            "--label".to_string(),
            "question-for-eva".to_string(),
            "--state".to_string(),
            state.to_string(),
            "--limit".to_string(),
            "500".to_string(),
            "--json".to_string(),
            "number,title,state".to_string(),
        ],
    )?;

    let Some(items) = value.as_array() else {
        return Err("unexpected issue list response format".to_string());
    };

    items.iter().map(parse_question_for_eva_issue).collect()
}

fn parse_question_for_eva_issue(value: &Value) -> Result<QuestionForEvaIssue, String> {
    let number = value
        .get("number")
        .and_then(Value::as_u64)
        .ok_or_else(|| "issue entry missing numeric number".to_string())?;
    let title = value
        .get("title")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("issue #{} missing title", number))?
        .to_string();
    let state = value
        .get("state")
        .and_then(Value::as_str)
        .ok_or_else(|| format!("issue #{} missing state", number))?
        .to_string();

    Ok(QuestionForEvaIssue {
        number,
        title,
        state,
    })
}

fn fetch_issue_comments(
    repo_root: &Path,
    runner: &dyn CommandRunner,
    issue_number: u64,
) -> Result<Vec<IssueComment>, String> {
    let path = format!(
        "repos/{}/issues/{}/comments?sort=created&direction=asc&per_page=100",
        MAIN_REPO, issue_number
    );
    let value = gh_json(
        repo_root,
        runner,
        &["api".to_string(), path, "--paginate".to_string()],
    )?;
    let Some(items) = value.as_array() else {
        return Err(format!(
            "unexpected comments response format for issue #{}",
            issue_number
        ));
    };

    items
        .iter()
        .map(|item| parse_issue_comment(issue_number, item))
        .collect()
}

fn parse_issue_comment(issue_number: u64, value: &Value) -> Result<IssueComment, String> {
    let author_login = json_str(value, &["user", "login"])
        .ok_or_else(|| format!("issue #{} comment missing user.login", issue_number))?
        .to_string();
    let created_at_raw = json_str(value, &["created_at"])
        .ok_or_else(|| format!("issue #{} comment missing created_at", issue_number))?;
    let created_at = parse_timestamp(created_at_raw).map_err(|error| {
        format!(
            "issue #{} comment has invalid created_at {:?}: {}",
            issue_number, created_at_raw, error
        )
    })?;
    let url = json_str(value, &["html_url"])
        .ok_or_else(|| format!("issue #{} comment missing html_url", issue_number))?
        .to_string();
    let body = value
        .get("body")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_string();

    Ok(IssueComment {
        author_login,
        created_at,
        url,
        body,
    })
}

fn classify_responses(
    issues: &[QuestionForEvaIssue],
    issue_comments: &HashMap<u64, Vec<IssueComment>>,
    since: DateTime<Utc>,
    include_closed: bool,
    orchestrator_detector: &OrchestratorDetector,
) -> Vec<NewEvaResponse> {
    let mut responses = Vec::new();

    for issue in issues {
        if !include_closed && is_closed_issue(issue) {
            continue;
        }

        let Some(comments) = issue_comments.get(&issue.number) else {
            continue;
        };
        let Some(eva_comment) = latest_comment(comments, |comment| {
            comment_is_eva(comment, orchestrator_detector)
        }) else {
            continue;
        };
        let orchestrator_comment = latest_comment(comments, |comment| {
            comment_is_orchestrator(comment, orchestrator_detector)
        });

        if eva_comment.created_at <= since {
            continue;
        }
        if orchestrator_comment
            .as_ref()
            // Equal timestamps are treated as already actioned; only strictly newer Eva comments
            // are surfaced as new responses.
            .is_some_and(|comment| eva_comment.created_at <= comment.created_at)
        {
            continue;
        }

        responses.push(NewEvaResponse {
            issue: issue.number,
            title: issue.title.clone(),
            eva_comment_at: format_timestamp(eva_comment.created_at),
            eva_comment_url: eva_comment.url.clone(),
            eva_excerpt: excerpt(&eva_comment.body),
            orchestrator_last_comment_at: orchestrator_comment
                .as_ref()
                .map(|comment| format_timestamp(comment.created_at)),
        });
    }

    responses.sort_by_key(|response| response.issue);
    responses
}

fn latest_comment<F>(comments: &[IssueComment], predicate: F) -> Option<&IssueComment>
where
    F: Fn(&IssueComment) -> bool,
{
    comments
        .iter()
        .filter(|comment| predicate(comment))
        .max_by_key(|comment| comment.created_at)
}

fn comment_is_eva(comment: &IssueComment, orchestrator_detector: &OrchestratorDetector) -> bool {
    comment.author_login == EVA_LOGIN && !comment_is_orchestrator(comment, orchestrator_detector)
}

fn comment_is_orchestrator(
    comment: &IssueComment,
    orchestrator_detector: &OrchestratorDetector,
) -> bool {
    orchestrator_detector.logins.contains(&comment.author_login)
        || comment_signature_matches(comment, &orchestrator_detector.signature)
}

fn comment_signature_matches(comment: &IssueComment, signature: &Regex) -> bool {
    non_empty_lines(&comment.body).any(|line| signature.is_match(line))
}

fn non_empty_lines(body: &str) -> impl Iterator<Item = &str> {
    body.lines().filter(|line| !line.trim().is_empty())
}

fn is_closed_issue(issue: &QuestionForEvaIssue) -> bool {
    issue.state.eq_ignore_ascii_case("closed")
}

fn excerpt(body: &str) -> String {
    let mut excerpt = String::new();
    let mut used_chars = 0;

    for word in body.split_whitespace() {
        let word_len = word.chars().count();
        let separator_len = usize::from(!excerpt.is_empty());
        if used_chars + separator_len >= EXCERPT_LIMIT {
            break;
        }
        if separator_len == 1 {
            excerpt.push(' ');
            used_chars += 1;
        }

        let remaining = EXCERPT_LIMIT - used_chars;
        if word_len <= remaining {
            excerpt.push_str(word);
            used_chars += word_len;
            continue;
        }

        excerpt.extend(word.chars().take(remaining));
        break;
    }

    excerpt
}

fn render_human_report(report: &Report) -> String {
    let mut lines = vec![format!(
        "check-eva-responses — {} new Eva response(s) since {}",
        report.new_responses.len(),
        report.since
    )];

    if report.new_responses.is_empty() {
        return lines.join("\n");
    }

    for response in &report.new_responses {
        lines.push(String::new());
        lines.push(format!("{MAIN_REPO}#{} {}", response.issue, response.title));
        lines.push(format!("  Eva responded: {}", response.eva_comment_at));
        lines.push(format!("  Excerpt: {}", response.eva_excerpt));
        lines.push(format!("  URL: {}", response.eva_comment_url));
    }

    lines.join("\n")
}

fn gh_json(repo_root: &Path, runner: &dyn CommandRunner, args: &[String]) -> Result<Value, String> {
    let output = runner.gh(repo_root, args)?;
    if output.exit_code != Some(0) {
        let stderr = output.stderr.trim();
        return Err(format!(
            "`gh {}` failed with status {}: {}",
            args.join(" "),
            output.exit_code.map_or_else(
                || "terminated by signal".to_string(),
                |code| code.to_string()
            ),
            if stderr.is_empty() {
                "<no stderr>".to_string()
            } else {
                stderr.to_string()
            }
        ));
    }

    serde_json::from_str(&output.stdout).map_err(|error| {
        format!(
            "failed to parse JSON output from `gh {}`: {}",
            args.join(" "),
            error
        )
    })
}

fn orchestrator_detector_from_env() -> Result<OrchestratorDetector, String> {
    build_orchestrator_detector(
        env::var(ORCHESTRATOR_LOGINS_ENV).ok().as_deref(),
        env::var(ORCHESTRATOR_SIGNATURE_ENV).ok().as_deref(),
    )
}

fn build_orchestrator_logins(extra_logins: Option<&str>) -> Result<HashSet<String>, String> {
    let mut logins = DEFAULT_ORCHESTRATOR_LOGINS
        .iter()
        .map(|login| (*login).to_string())
        .collect::<HashSet<_>>();

    let Some(extra_logins) = extra_logins else {
        return Ok(logins);
    };

    for segment in extra_logins.split(',') {
        let login = segment.trim();
        if login.is_empty() {
            return Err(format!(
                "{} must not contain empty login values",
                ORCHESTRATOR_LOGINS_ENV
            ));
        }
        logins.insert(login.to_string());
    }

    Ok(logins)
}

fn build_orchestrator_signature(signature: Option<&str>) -> Result<Regex, String> {
    let pattern = signature
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(default_orchestrator_signature_pattern);

    Regex::new(&pattern).map_err(|error| {
        format!(
            "invalid {} regex {:?}: {}",
            ORCHESTRATOR_SIGNATURE_ENV, pattern, error
        )
    })
}

fn default_orchestrator_signature_pattern() -> String {
    format!(
        r"^\s*{}(?:\s*\|.*)?$",
        regex::escape(DEFAULT_ORCHESTRATOR_SIGNATURE_PREFIX)
    )
}

fn build_orchestrator_detector(
    extra_logins: Option<&str>,
    signature: Option<&str>,
) -> Result<OrchestratorDetector, String> {
    Ok(OrchestratorDetector {
        logins: build_orchestrator_logins(extra_logins)?,
        signature: build_orchestrator_signature(signature)?,
    })
}

fn parse_timestamp(value: &str) -> Result<DateTime<Utc>, chrono::ParseError> {
    DateTime::parse_from_rfc3339(value).map(|timestamp| timestamp.with_timezone(&Utc))
}

fn format_timestamp(value: DateTime<Utc>) -> String {
    value.to_rfc3339_opts(SecondsFormat::Secs, true)
}

fn json_str<'a>(value: &'a Value, path: &[&str]) -> Option<&'a str> {
    let mut current = value;
    for segment in path {
        current = current.get(*segment)?;
    }
    current.as_str()
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use serde_json::json;
    use std::collections::VecDeque;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Mutex;

    #[derive(Default)]
    struct MockRunner {
        gh_results: Mutex<VecDeque<Result<ExecutionResult, String>>>,
        gh_calls: Mutex<Vec<Vec<String>>>,
    }

    impl MockRunner {
        fn with_gh_results(gh_results: Vec<Result<ExecutionResult, String>>) -> Self {
            Self {
                gh_results: Mutex::new(VecDeque::from(gh_results)),
                ..Self::default()
            }
        }

        fn gh_calls(&self) -> Vec<Vec<String>> {
            self.gh_calls.lock().expect("gh calls lock").clone()
        }
    }

    impl CommandRunner for MockRunner {
        fn gh(&self, _repo_root: &Path, args: &[String]) -> Result<ExecutionResult, String> {
            self.gh_calls
                .lock()
                .expect("gh calls lock")
                .push(args.to_vec());
            self.gh_results
                .lock()
                .expect("gh results lock")
                .pop_front()
                .unwrap_or_else(|| Err(format!("unexpected gh call: {:?}", args)))
        }
    }

    struct TempRepo {
        path: PathBuf,
    }

    impl TempRepo {
        fn new(state: &Value) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = env::temp_dir().join(format!("check-eva-responses-test-{}", run_id));
            fs::create_dir_all(path.join("docs")).expect("create temp repo docs");
            fs::write(
                path.join("docs/state.json"),
                serde_json::to_string_pretty(state).expect("serialize state"),
            )
            .expect("write temp state");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).expect("write help");
        let help = String::from_utf8(output).expect("utf8 help");
        assert!(help.contains("--repo-root"));
        assert!(help.contains("--json"));
        assert!(help.contains("--since"));
        assert!(help.contains("--include-closed"));
    }

    #[test]
    fn issue_with_only_eva_comments_is_classified_as_new_response() {
        let responses = classify_responses(
            &[issue(2293, "Question", "OPEN")],
            &HashMap::from([(
                2293,
                vec![comment(
                    EVA_LOGIN,
                    "2026-04-19T11:21:46Z",
                    "https://example.test/1",
                    "Eva answered with details",
                )],
            )]),
            parse_timestamp("2026-04-01T00:00:00Z").expect("parse timestamp"),
            false,
            &default_orchestrator_detector(),
        );

        assert_eq!(responses.len(), 1);
        assert_eq!(responses[0].issue, 2293);
        assert_eq!(responses[0].orchestrator_last_comment_at, None);
    }

    #[test]
    fn eva_orchestrator_eva_sequence_is_classified_as_new_response() {
        let responses = classify_responses(
            &[issue(2402, "Question", "OPEN")],
            &HashMap::from([(
                2402,
                vec![
                    comment(
                        EVA_LOGIN,
                        "2026-04-18T09:00:00Z",
                        "https://example.test/old-eva",
                        "Initial reply",
                    ),
                    comment(
                        "claude[bot]",
                        "2026-04-18T10:00:00Z",
                        "https://example.test/orch",
                        "Thanks",
                    ),
                    comment(
                        EVA_LOGIN,
                        "2026-04-19T12:00:00Z",
                        "https://example.test/new-eva",
                        "Follow-up answer",
                    ),
                ],
            )]),
            parse_timestamp("2026-04-18T00:00:00Z").expect("parse timestamp"),
            false,
            &default_orchestrator_detector(),
        );

        assert_eq!(responses.len(), 1);
        assert_eq!(
            responses[0].orchestrator_last_comment_at.as_deref(),
            Some("2026-04-18T10:00:00Z")
        );
    }

    #[test]
    fn orchestrator_after_eva_is_not_a_new_response() {
        let responses = classify_responses(
            &[issue(2403, "Question", "OPEN")],
            &HashMap::from([(
                2403,
                vec![
                    comment(
                        EVA_LOGIN,
                        "2026-04-18T09:00:00Z",
                        "https://example.test/eva",
                        "Answer",
                    ),
                    comment(
                        "claude[bot]",
                        "2026-04-18T10:00:00Z",
                        "https://example.test/orch",
                        "Acknowledged",
                    ),
                ],
            )]),
            parse_timestamp("2026-04-01T00:00:00Z").expect("parse timestamp"),
            false,
            &default_orchestrator_detector(),
        );

        assert!(responses.is_empty());
    }

    #[test]
    fn issue_with_only_orchestrator_comments_is_not_classified() {
        let responses = classify_responses(
            &[issue(2405, "Question", "OPEN")],
            &HashMap::from([(
                2405,
                vec![comment(
                    "claude[bot]",
                    "2026-04-18T10:00:00Z",
                    "https://example.test/orch",
                    "Awaiting reply",
                )],
            )]),
            parse_timestamp("2026-04-01T00:00:00Z").expect("parse timestamp"),
            false,
            &default_orchestrator_detector(),
        );

        assert!(responses.is_empty());
    }

    #[test]
    fn issue_with_no_comments_is_not_classified() {
        let responses = classify_responses(
            &[issue(2416, "Question", "OPEN")],
            &HashMap::from([(2416, Vec::new())]),
            parse_timestamp("2026-04-01T00:00:00Z").expect("parse timestamp"),
            false,
            &default_orchestrator_detector(),
        );

        assert!(responses.is_empty());
    }

    #[test]
    fn since_cutoff_is_respected() {
        let responses = classify_responses(
            &[issue(2519, "Question", "OPEN")],
            &HashMap::from([(
                2519,
                vec![comment(
                    EVA_LOGIN,
                    "2026-04-18T09:00:00Z",
                    "https://example.test/eva",
                    "Old answer",
                )],
            )]),
            parse_timestamp("2026-04-19T00:00:00Z").expect("parse timestamp"),
            false,
            &default_orchestrator_detector(),
        );

        assert!(responses.is_empty());
    }

    #[test]
    fn mixed_open_and_closed_issues_respect_include_closed() {
        let issues = vec![
            issue(2542, "Open question", "OPEN"),
            issue(2574, "Closed question", "CLOSED"),
        ];
        let comments = HashMap::from([
            (
                2542,
                vec![comment(
                    EVA_LOGIN,
                    "2026-04-19T10:00:00Z",
                    "https://example.test/open",
                    "Open reply",
                )],
            ),
            (
                2574,
                vec![comment(
                    EVA_LOGIN,
                    "2026-04-19T11:00:00Z",
                    "https://example.test/closed",
                    "Closed reply",
                )],
            ),
        ]);
        let since = parse_timestamp("2026-04-01T00:00:00Z").expect("parse timestamp");

        let open_only = classify_responses(
            &issues,
            &comments,
            since,
            false,
            &default_orchestrator_detector(),
        );
        let with_closed = classify_responses(
            &issues,
            &comments,
            since,
            true,
            &default_orchestrator_detector(),
        );

        assert_eq!(
            open_only.iter().map(|item| item.issue).collect::<Vec<_>>(),
            vec![2542]
        );
        assert_eq!(
            with_closed
                .iter()
                .map(|item| item.issue)
                .collect::<Vec<_>>(),
            vec![2542, 2574]
        );
    }

    #[test]
    fn execute_uses_state_last_eva_comment_check_when_since_absent() {
        let repo = TempRepo::new(&sample_state("2026-04-25T05:43:02Z"));
        let runner = MockRunner::with_gh_results(vec![
            ok_json(json!([
                {
                    "number": 2293,
                    "title": "Question",
                    "state": "OPEN"
                }
            ])),
            ok_json(json!([
                {
                    "user": { "login": EVA_LOGIN },
                    "created_at": "2026-04-25T05:50:00Z",
                    "html_url": "https://example.test/comment",
                    "body": "Eva answer"
                }
            ])),
        ]);
        let cli = Cli {
            repo_root: repo.path().to_path_buf(),
            json: true,
            since: None,
            include_closed: false,
        };

        let output =
            execute(&cli, &runner, &default_orchestrator_detector()).expect("execute should work");
        let report: Report = serde_json::from_str(&output).expect("report should parse");

        assert_eq!(report.since, "2026-04-25T05:43:02Z");
        assert_eq!(report.checked_issues, 1);
        assert_eq!(report.new_responses.len(), 1);
        assert_eq!(
            runner.gh_calls()[0],
            vec![
                "issue".to_string(),
                "list".to_string(),
                "--repo".to_string(),
                MAIN_REPO.to_string(),
                "--label".to_string(),
                "question-for-eva".to_string(),
                "--state".to_string(),
                "open".to_string(),
                "--limit".to_string(),
                "500".to_string(),
                "--json".to_string(),
                "number,title,state".to_string(),
            ]
        );
    }

    #[test]
    fn human_output_matches_expected_shape() {
        let report = Report {
            since: "2026-04-25T05:43:02Z".to_string(),
            checked_issues: 1,
            new_responses: vec![NewEvaResponse {
                issue: 2293,
                title: "Question title".to_string(),
                eva_comment_at: "2026-04-19T11:21:46Z".to_string(),
                eva_comment_url: "https://example.test/comment".to_string(),
                eva_excerpt: "Answer excerpt".to_string(),
                orchestrator_last_comment_at: Some("2026-04-12T00:00:00Z".to_string()),
            }],
        };

        let output = render_human_report(&report);

        assert!(output
            .contains("check-eva-responses — 1 new Eva response(s) since 2026-04-25T05:43:02Z"));
        assert!(output.contains("EvaLok/schema-org-json-ld#2293 Question title"));
        assert!(output.contains("Eva responded: 2026-04-19T11:21:46Z"));
        assert!(output.contains("Excerpt: Answer excerpt"));
        assert!(output.contains("URL: https://example.test/comment"));
    }

    #[test]
    fn excerpt_truncates_to_two_hundred_characters() {
        let body = "a".repeat(EXCERPT_LIMIT + 10);
        let excerpt = excerpt(&body);

        assert_eq!(excerpt.len(), EXCERPT_LIMIT);
    }

    #[test]
    fn extra_orchestrator_logins_are_pluggable() {
        let detector =
            build_orchestrator_detector(Some("copilot-swe-agent[bot], github-actions[bot]"), None)
                .expect("detector should parse");

        assert!(detector.logins.contains("copilot-swe-agent[bot]"));
        assert!(detector.logins.contains("github-actions[bot]"));
        assert!(detector.logins.contains("claude[bot]"));
    }

    #[test]
    fn empty_extra_orchestrator_login_fails_closed() {
        let error = build_orchestrator_logins(Some("copilot-swe-agent[bot], "))
            .expect_err("empty login should fail");

        assert!(error.contains(ORCHESTRATOR_LOGINS_ENV));
    }

    #[test]
    fn signed_evalok_comment_is_treated_as_orchestrator() {
        let detector = default_orchestrator_detector();
        let comment = comment(
            EVA_LOGIN,
            "2026-04-25T06:04:23Z",
            "https://example.test/orch",
            "\n  > **[main-orchestrator]** | Cycle 538\n\nAck",
        );

        assert!(comment_is_orchestrator(&comment, &detector));
        assert!(!comment_is_eva(&comment, &detector));
    }

    #[test]
    fn signature_on_later_non_empty_line_is_still_treated_as_orchestrator() {
        let detector = default_orchestrator_detector();
        let comment = comment(
            EVA_LOGIN,
            "2026-04-22T21:41:15Z",
            "https://example.test/orch-late-signature",
            "Dispatched as #2657.\n\n> **[main-orchestrator]** | Cycle 529",
        );

        assert!(comment_is_orchestrator(&comment, &detector));
        assert!(!comment_is_eva(&comment, &detector));
    }

    #[test]
    fn unsigned_evalok_comment_is_treated_as_eva() {
        let detector = default_orchestrator_detector();
        let comment = comment(
            EVA_LOGIN,
            "2026-04-19T11:21:46Z",
            "https://example.test/eva",
            "Actual Eva response",
        );

        assert!(!comment_is_orchestrator(&comment, &detector));
        assert!(comment_is_eva(&comment, &detector));
    }

    #[test]
    fn signed_evalok_then_unsigned_evalok_yields_new_eva_response() {
        let responses = classify_responses(
            &[issue(2402, "Question", "OPEN")],
            &HashMap::from([(
                2402,
                vec![
                    comment(
                        EVA_LOGIN,
                        "2026-04-18T10:00:00Z",
                        "https://example.test/orch",
                        "> **[main-orchestrator]** | Cycle 537\n\nAck",
                    ),
                    comment(
                        EVA_LOGIN,
                        "2026-04-19T12:00:00Z",
                        "https://example.test/eva",
                        "Unsigned Eva follow-up",
                    ),
                ],
            )]),
            parse_timestamp("2026-04-18T00:00:00Z").expect("parse timestamp"),
            false,
            &default_orchestrator_detector(),
        );

        assert_eq!(responses.len(), 1);
        assert_eq!(
            responses[0].orchestrator_last_comment_at.as_deref(),
            Some("2026-04-18T10:00:00Z")
        );
    }

    #[test]
    fn unsigned_evalok_comments_preserve_legacy_eva_behavior() {
        let responses = classify_responses(
            &[issue(2403, "Question", "OPEN")],
            &HashMap::from([(
                2403,
                vec![
                    comment(
                        EVA_LOGIN,
                        "2026-04-18T10:00:00Z",
                        "https://example.test/eva-1",
                        "Legacy Eva comment",
                    ),
                    comment(
                        EVA_LOGIN,
                        "2026-04-19T10:00:00Z",
                        "https://example.test/eva-2",
                        "Later legacy Eva comment",
                    ),
                ],
            )]),
            parse_timestamp("2026-04-18T00:00:00Z").expect("parse timestamp"),
            false,
            &default_orchestrator_detector(),
        );

        assert_eq!(responses.len(), 1);
        assert_eq!(responses[0].orchestrator_last_comment_at, None);
        assert_eq!(responses[0].eva_comment_at, "2026-04-19T10:00:00Z");
    }

    #[test]
    fn custom_signature_regex_is_supported() {
        let detector = build_orchestrator_detector(None, Some(r"^\s*CUSTOM-SIGNATURE(?:\b|$)"))
            .expect("detector should parse");
        let comment = comment(
            EVA_LOGIN,
            "2026-04-19T12:00:00Z",
            "https://example.test/custom",
            "  CUSTOM-SIGNATURE from custom deployment",
        );

        assert!(comment_is_orchestrator(&comment, &detector));
    }

    #[test]
    fn invalid_custom_signature_regex_fails_closed() {
        let error =
            build_orchestrator_detector(None, Some("(")).expect_err("invalid regex should fail");

        assert!(error.contains(ORCHESTRATOR_SIGNATURE_ENV));
    }

    fn issue(number: u64, title: &str, state: &str) -> QuestionForEvaIssue {
        QuestionForEvaIssue {
            number,
            title: title.to_string(),
            state: state.to_string(),
        }
    }

    fn comment(author_login: &str, created_at: &str, url: &str, body: &str) -> IssueComment {
        IssueComment {
            author_login: author_login.to_string(),
            created_at: parse_timestamp(created_at).expect("parse comment timestamp"),
            url: url.to_string(),
            body: body.to_string(),
        }
    }

    fn sample_state(last_eva_comment_check: &str) -> Value {
        json!({
            "schema_version": 1,
            "last_cycle": {
                "number": 538
            },
            "last_eva_comment_check": last_eva_comment_check
        })
    }

    fn ok_json(value: Value) -> Result<ExecutionResult, String> {
        Ok(ExecutionResult {
            exit_code: Some(0),
            stdout: serde_json::to_string(&value).expect("serialize json"),
            stderr: String::new(),
        })
    }

    fn default_orchestrator_detector() -> OrchestratorDetector {
        build_orchestrator_detector(None, None).expect("default detector")
    }

    fn build_orchestrator_detector(
        extra_logins: Option<&str>,
        signature: Option<&str>,
    ) -> Result<OrchestratorDetector, String> {
        Ok(OrchestratorDetector {
            logins: build_orchestrator_logins(extra_logins)?,
            signature: build_orchestrator_signature(signature)?,
        })
    }
}
