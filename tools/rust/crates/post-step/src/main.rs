use clap::{ArgGroup, Parser};
use serde_json::{json, Value};
use state_schema::current_cycle_from_state;
use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const ORCHESTRATOR_SIGNATURE: &str = "[main-orchestrator]";
const VALID_STEP_IDS: [&str; 38] = [
    "0", "0.1", "0.5", "0.6", "1", "1.1", "1.5", "2", "2.5", "3", "4", "5", "5.5", "5.6", "5.8",
    "5.9", "5.10", "5.11", "5.12", "5.13", "6", "7", "8", "9", "10", "C1", "C2", "C3", "C4.1",
    "C4.5", "C4.7", "C5.5", "C5", "C5.1", "C5.6", "C6", "C7", "C8",
];
/// Mandatory step IDs in checklist order. When posting a step, all mandatory predecessors
/// must already be present on the issue. These are the step IDs from MANDATORY_STEPS in
/// pipeline-check, without the effective-from-cycle thresholds.
/// Order matches the real close-out execution path: C5.5 runs first (pipeline gate),
/// then C5 (commit docs using pipeline summary), then C5.1 (receipt validation).
const MANDATORY_STEP_IDS: &[&str] = &[
    "0", "0.5", "0.6", "1", "1.1", "2", "3", "4", "5", "6", "7", "8", "9", "C1", "C2", "C3",
    "C4.1", "C4.5", "C5.5", "C5", "C5.1", "C6", "C7", "C8",
];

#[derive(Parser, Debug)]
#[command(name = "post-step")]
#[command(group(
	ArgGroup::new("body_source")
		.required(true)
		.args(["body", "body_file", "body_stdin"])
))]
struct Cli {
    /// Orchestrator run issue number
    #[arg(long)]
    issue: u64,

    /// Checklist step identifier (for example: 0, 0.5, 1, 5.11)
    #[arg(long)]
    step: String,

    /// Short checklist step title
    #[arg(long)]
    title: String,

    /// Step outcome body text
    #[arg(long)]
    body: Option<String>,

    /// Path to a file containing the step outcome body
    #[arg(long)]
    body_file: Option<PathBuf>,

    /// Read the step outcome body from stdin
    #[arg(long)]
    body_stdin: bool,

    /// Allow reposting a step ID even if it already exists on the issue
    #[arg(long)]
    force: bool,

    /// Skip step ID validation for non-standard step names
    #[arg(long)]
    skip_validation: bool,

    /// Skip validation that rejects likely unexpanded shell or template text in the body
    #[arg(long)]
    skip_body_validation: bool,

    /// Repository root containing docs/state.json
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    let runner = GhCommandRunner;

    match execute(&cli, &runner) {
        Ok(message) => println!("{}", message),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn execute(cli: &Cli, runner: &dyn CommentPoster) -> Result<String, String> {
    let step = validate_required_text("step", &cli.step)?;
    if cli.skip_validation {
        eprintln!(
            "Warning: skipping step ID validation for non-standard step '{}'",
            cli.step
        );
    } else {
        validate_step_id(step)?;
    }
    let title = validate_required_text("title", &cli.title)?;
    let cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /cycle_phase/cycle or /last_cycle/number in state.json" {
            "missing numeric /cycle_phase/cycle or /last_cycle/number in docs/state.json"
                .to_string()
        } else {
            error
        }
    })?;
    let body = resolve_body(cli)?;
    let comment = format_comment(cycle, step, title, &body);

    if !cli.force {
        let existing_comments = runner.existing_comments(cli.issue)?;
        if has_matching_step_comment(&existing_comments, step) {
            return Err(format!(
				"Step {step} already posted on issue #{}. Use a different step ID or --force to override.",
				cli.issue
			));
        }
        check_step_ordering(&existing_comments, step)?;
    }

    runner.post_comment(cli.issue, &comment)?;

    Ok(format!("Step {} posted to {MAIN_REPO}#{}", step, cli.issue))
}

fn resolve_body(cli: &Cli) -> Result<String, String> {
    // Clap enforces this for real CLI parsing, but direct struct construction in unit tests
    // or future internal callers can bypass parser validation, so keep a fail-closed check here.
    let source_count = usize::from(cli.body.is_some())
        + usize::from(cli.body_file.is_some())
        + usize::from(cli.body_stdin);
    if source_count != 1 {
        return Err(
            "exactly one of --body, --body-file, or --body-stdin must be provided".to_string(),
        );
    }

    let body = if let Some(body) = &cli.body {
        body.clone()
    } else if let Some(path) = &cli.body_file {
        fs::read_to_string(path)
            .map_err(|error| format!("failed to read {}: {}", path.display(), error))?
    } else {
        read_body_from_stdin()?
    };

    normalize_body_text(&body, cli.skip_body_validation)
}

fn read_body_from_stdin() -> Result<String, String> {
    let mut content = String::new();
    io::stdin()
        .read_to_string(&mut content)
        .map_err(|error| format!("failed to read stdin: {}", error))?;
    Ok(content)
}

fn validate_body_text(body: &str) -> Result<(), String> {
    if body.contains("$(") {
        return Err(body_validation_error(
            "unexpanded shell command substitution `$(`",
        ));
    }

    if body.contains("${") {
        return Err(body_validation_error("unexpanded shell variable `${`"));
    }

    if let Some(pattern) = find_placeholder_pattern(body) {
        return Err(body_validation_error(pattern));
    }

    Ok(())
}

fn body_validation_error(pattern: &str) -> String {
    format!(
        "body contains {pattern}; use --skip-body-validation only when literal text is intentional"
    )
}

fn find_placeholder_pattern(body: &str) -> Option<&'static str> {
    let normalized = body.to_ascii_lowercase();
    for (pattern, message) in [
        ("{{", "unexpanded template marker `{{`"),
        ("}}", "unexpanded template marker `}}`"),
        ("<placeholder", "placeholder marker `<placeholder`"),
        ("[placeholder", "placeholder marker `[placeholder`"),
        ("<fill in", "placeholder marker `<fill in`"),
        ("[fill in", "placeholder marker `[fill in`"),
        ("[insert", "placeholder marker `[insert`"),
        (
            "replace_me",
            "placeholder marker `replace_me` (matched case-insensitively)",
        ),
    ] {
        if normalized.contains(pattern) {
            return Some(message);
        }
    }

    None
}

fn validate_required_text<'a>(field_name: &str, value: &'a str) -> Result<&'a str, String> {
    if value.trim().is_empty() {
        return Err(format!("{field_name} must not be empty"));
    }

    Ok(value)
}

fn validate_step_id(step: &str) -> Result<(), String> {
    if let Some((from_step, to_step)) = range_step_ids(step) {
        return Err(format!(
			"Invalid step ID '{step}': step IDs must be posted individually. Use separate post-step calls for steps {from_step} and {to_step}."
		));
    }

    if VALID_STEP_IDS.contains(&step) {
        return Ok(());
    }

    Err(format!(
        "Invalid step ID '{step}': expected one of {}",
        VALID_STEP_IDS.join(", ")
    ))
}

fn range_step_ids(step: &str) -> Option<(&str, &str)> {
    for (index, character) in step.char_indices() {
        if character != '-' {
            continue;
        }

        let prefix = &step[..index];
        let suffix = &step[index + 1..];
        let previous = prefix.chars().next_back()?;
        let next = suffix.chars().next()?;
        if previous.is_ascii_digit() && next.is_ascii_digit() {
            return Some((step_token_suffix(prefix), step_token_prefix(suffix)));
        }
    }

    None
}

fn step_token_suffix(segment: &str) -> &str {
    let start = segment
        .rfind(|character: char| !character.is_ascii_alphanumeric() && character != '.')
        .map_or(0, |index| index + 1);
    &segment[start..]
}

fn step_token_prefix(segment: &str) -> &str {
    let end = segment
        .find(|character: char| !character.is_ascii_alphanumeric() && character != '.')
        .unwrap_or(segment.len());
    &segment[..end]
}

fn normalize_body_text(body: &str, skip_body_validation: bool) -> Result<String, String> {
    let normalized = body.trim_end_matches(['\r', '\n']);
    validate_required_text("body", normalized)?;
    if !skip_body_validation {
        validate_body_text(normalized)?;
    }
    Ok(normalized.to_string())
}

fn format_comment(cycle: u64, step: &str, title: &str, body: &str) -> String {
    format!("> **{ORCHESTRATOR_SIGNATURE}** | Cycle {cycle} | Step {step}\n\n### {title}\n\n{body}")
}

fn has_matching_step_comment(existing_comments: &[String], step: &str) -> bool {
    let expected_suffix = format!("| Step {step}");

    existing_comments
        .iter()
        .filter_map(|body| body.lines().next())
        .any(|line| {
            line.starts_with(&format!("> **{ORCHESTRATOR_SIGNATURE}** | Cycle "))
                && line.ends_with(&expected_suffix)
        })
}

/// Returns an error if posting `step` would create a checklist ordering gap: a mandatory
/// predecessor step has not yet been posted on the issue. Bypass with --force.
fn check_step_ordering(existing_comments: &[String], step: &str) -> Result<(), String> {
    let step_index = match VALID_STEP_IDS.iter().position(|&s| s == step) {
        Some(idx) => idx,
        None => return Ok(()), // non-standard step; skip ordering check
    };

    // Collect mandatory steps that come before `step` in checklist order and are absent.
    let missing: Vec<&str> = VALID_STEP_IDS[..step_index]
        .iter()
        .copied()
        .filter(|&s| MANDATORY_STEP_IDS.contains(&s))
        .filter(|&s| !has_matching_step_comment(existing_comments, s))
        .collect();

    if missing.is_empty() {
        return Ok(());
    }

    Err(format!(
        "Cannot post step {step}: mandatory predecessor step(s) not yet posted: [{}]. \
         Post those steps first, or use --force to override.",
        missing.join(", ")
    ))
}

trait CommentPoster {
    fn existing_comments(&self, issue: u64) -> Result<Vec<String>, String>;
    fn post_comment(&self, issue: u64, body: &str) -> Result<(), String>;
}

struct GhCommandRunner;

fn parse_paginated_json(raw: &str) -> Result<Vec<Value>, String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }

    let mut comments = Vec::new();
    for (index, page) in serde_json::Deserializer::from_str(trimmed)
        .into_iter::<Value>()
        .enumerate()
    {
        let value = page.map_err(|error| {
            format!(
                "failed to parse gh api paginated output page {}: {}",
                index + 1,
                error
            )
        })?;
        let array = value.as_array().ok_or_else(|| {
            format!(
                "failed to parse gh api paginated output page {}: expected JSON array",
                index + 1
            )
        })?;
        comments.extend(array.iter().cloned());
    }

    Ok(comments)
}

impl CommentPoster for GhCommandRunner {
    fn existing_comments(&self, issue: u64) -> Result<Vec<String>, String> {
        let output = Command::new("gh")
            .arg("api")
            .arg(format!("repos/{MAIN_REPO}/issues/{issue}/comments"))
            .arg("--paginate")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .map_err(|error| format!("failed to execute gh api: {}", error))?;

        if !output.status.success() {
            return Err(command_failure_message("gh api", &output));
        }

        let raw = String::from_utf8_lossy(&output.stdout);
        let comments = parse_paginated_json(&raw)?;

        Ok(comments
            .iter()
            .filter_map(|comment| {
                comment
                    .get("body")
                    .and_then(|v| v.as_str())
                    .map(String::from)
            })
            .collect())
    }

    fn post_comment(&self, issue: u64, body: &str) -> Result<(), String> {
        let payload = serde_json::to_vec(&json!({ "body": body }))
            .map_err(|error| format!("failed to serialize comment payload: {}", error))?;
        let mut child = Command::new("gh")
            .arg("api")
            .arg(format!("repos/{MAIN_REPO}/issues/{issue}/comments"))
            .arg("--method")
            .arg("POST")
            .arg("--input")
            .arg("-")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|error| format!("failed to execute gh api: {}", error))?;

        {
            let stdin = child
                .stdin
                .as_mut()
                .ok_or_else(|| "failed to open stdin for gh api".to_string())?;
            stdin
                .write_all(&payload)
                .map_err(|error| format!("failed to write gh api payload: {}", error))?;
        }

        let output = child
            .wait_with_output()
            .map_err(|error| format!("failed to wait for gh api: {}", error))?;

        if !output.status.success() {
            return Err(command_failure_message("gh api", &output));
        }

        Ok(())
    }
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
    use std::fs;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct RecordingPoster {
        body: std::sync::Mutex<Vec<String>>,
        existing_comments: Vec<String>,
        fetch_error: Option<String>,
        post_error: Option<String>,
    }

    impl RecordingPoster {
        fn success() -> Self {
            Self {
                body: std::sync::Mutex::new(Vec::new()),
                existing_comments: Vec::new(),
                fetch_error: None,
                post_error: None,
            }
        }

        fn with_existing_comments(existing_comments: &[&str]) -> Self {
            Self {
                body: std::sync::Mutex::new(Vec::new()),
                existing_comments: existing_comments
                    .iter()
                    .map(|body| body.to_string())
                    .collect(),
                fetch_error: None,
                post_error: None,
            }
        }

        fn failing(error: &str) -> Self {
            Self {
                body: std::sync::Mutex::new(Vec::new()),
                existing_comments: Vec::new(),
                fetch_error: None,
                post_error: Some(error.to_string()),
            }
        }

        fn fetch_failing(error: &str) -> Self {
            Self {
                body: std::sync::Mutex::new(Vec::new()),
                existing_comments: Vec::new(),
                fetch_error: Some(error.to_string()),
                post_error: None,
            }
        }

        fn posted_bodies(&self) -> Vec<String> {
            self.body.lock().unwrap().clone()
        }
    }

    impl CommentPoster for RecordingPoster {
        fn existing_comments(&self, _issue: u64) -> Result<Vec<String>, String> {
            if let Some(error) = &self.fetch_error {
                return Err(error.clone());
            }

            Ok(self.existing_comments.clone())
        }

        fn post_comment(&self, _issue: u64, body: &str) -> Result<(), String> {
            if let Some(error) = &self.post_error {
                return Err(error.clone());
            }

            self.body.lock().unwrap().push(body.to_string());
            Ok(())
        }
    }

    #[test]
    fn format_comment_includes_signature_cycle_step_title_and_body() {
        let comment = format_comment(
            198,
            "1",
            "Check for input-from-eva issues",
            "Found 2 open issues.",
        );

        assert_eq!(
			comment,
			"> **[main-orchestrator]** | Cycle 198 | Step 1\n\n### Check for input-from-eva issues\n\nFound 2 open issues."
		);
    }

    #[test]
    fn execute_posts_formatted_comment_and_returns_confirmation() {
        let repo_root = temp_repo_root("post-step-success");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "1".to_string(),
            title: "Check for input-from-eva issues".to_string(),
            body: Some("Found 2 open issues.".to_string()),
            body_file: None,
            body_stdin: false,
            force: false,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::success();

        let result = execute(&cli, &poster).expect("execute should succeed");

        assert_eq!(result, "Step 1 posted to EvaLok/schema-org-json-ld#834");
        assert_eq!(
			poster.posted_bodies(),
			vec![
				"> **[main-orchestrator]** | Cycle 198 | Step 1\n\n### Check for input-from-eva issues\n\nFound 2 open issues."
					.to_string()
			]
		);
    }

    #[test]
    fn execute_reads_body_from_file() {
        let repo_root = temp_repo_root("post-step-body-file");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let body_path = repo_root.join("body.md");
        fs::write(&body_path, "Line one.\n\nLine two.\n").unwrap();
        let cli = Cli {
            issue: 834,
            step: "5.11".to_string(),
            title: "Summarize completion checks".to_string(),
            body: None,
            body_file: Some(body_path),
            body_stdin: false,
            force: false,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::success();

        execute(&cli, &poster).expect("execute should succeed");

        assert_eq!(
			poster.posted_bodies(),
			vec![
				"> **[main-orchestrator]** | Cycle 198 | Step 5.11\n\n### Summarize completion checks\n\nLine one.\n\nLine two."
					.to_string()
			]
		);
    }

    #[test]
    fn execute_fails_when_cycle_number_is_missing() {
        let repo_root = temp_repo_root("post-step-missing-cycle");
        write_state_json(&repo_root, r#"{"last_cycle":{}}"#);
        let cli = Cli {
            issue: 834,
            step: "1".to_string(),
            title: "Check for input-from-eva issues".to_string(),
            body: Some("Found 2 open issues.".to_string()),
            body_file: None,
            body_stdin: false,
            force: false,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::success();

        let error = execute(&cli, &poster).expect_err("execute should fail");

        assert_eq!(
            error,
            "missing numeric /cycle_phase/cycle or /last_cycle/number in docs/state.json"
        );
    }

    #[test]
    fn execute_fails_closed_when_comment_posting_fails() {
        let repo_root = temp_repo_root("post-step-command-failure");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "1".to_string(),
            title: "Check for input-from-eva issues".to_string(),
            body: Some("Found 2 open issues.".to_string()),
            body_file: None,
            body_stdin: false,
            force: false,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::failing("gh api failed with status 1: rate limited");

        let error = execute(&cli, &poster).expect_err("execute should fail");

        assert_eq!(error, "gh api failed with status 1: rate limited");
    }

    #[test]
    fn cli_requires_exactly_one_body_source() {
        let missing = Cli::try_parse_from([
            "post-step",
            "--issue",
            "834",
            "--step",
            "1",
            "--title",
            "Test",
        ]);
        assert!(missing.is_err());

        let both = Cli::try_parse_from([
            "post-step",
            "--issue",
            "834",
            "--step",
            "1",
            "--title",
            "Test",
            "--body",
            "text",
            "--body-file",
            "/tmp/body.md",
        ]);
        assert!(both.is_err());

        let body_stdin = Cli::try_parse_from([
            "post-step",
            "--issue",
            "834",
            "--step",
            "1",
            "--title",
            "Test",
            "--body-stdin",
        ]);
        assert!(body_stdin.is_ok());
    }

    #[test]
    fn valid_step_ids_are_accepted() {
        for step in ["0", "0.5", "1", "C1", "C4.5", "10"] {
            assert!(
                validate_step_id(step).is_ok(),
                "expected {step} to be valid"
            );
        }
    }

    #[test]
    fn invalid_step_ids_are_rejected() {
        for step in ["11", "foo", "step1"] {
            let error = validate_step_id(step).expect_err("step should be rejected");
            assert_eq!(
                error,
                format!(
                    "Invalid step ID '{step}': expected one of {}",
                    VALID_STEP_IDS.join(", ")
                )
            );
        }
    }

    #[test]
    fn range_step_ids_are_rejected_with_specific_error() {
        for (step, from_step, to_step) in [
            ("4-5", "4", "5"),
            ("6-8", "6", "8"),
            ("1-3", "1", "3"),
            ("1.1-1.5", "1.1", "1.5"),
            ("step4-5", "step4", "5"),
        ] {
            let error = validate_step_id(step).expect_err("range should be rejected");
            assert_eq!(
				error,
				format!(
					"Invalid step ID '{step}': step IDs must be posted individually. Use separate post-step calls for steps {from_step} and {to_step}."
				)
			);
        }
    }

    #[test]
    fn range_token_helpers_extract_expected_tokens() {
        assert_eq!(step_token_suffix("step4"), "step4");
        assert_eq!(step_token_suffix("prefix step4.1"), "step4.1");
        assert_eq!(step_token_prefix("5"), "5");
        assert_eq!(step_token_prefix("5.13 suffix"), "5.13");
    }

    #[test]
    fn body_validation_rejects_command_substitution() {
        let error = normalize_body_text("result: $(date)", false).expect_err("body should fail");

        assert_eq!(
            error,
            "body contains unexpanded shell command substitution `$(`; use --skip-body-validation only when literal text is intentional"
        );
    }

    #[test]
    fn body_validation_rejects_shell_variable_expansion() {
        let error = normalize_body_text("result: ${USER}", false).expect_err("body should fail");

        assert_eq!(
            error,
            "body contains unexpanded shell variable `${`; use --skip-body-validation only when literal text is intentional"
        );
    }

    #[test]
    fn body_validation_allows_backtick_wrapped_commands() {
        let body = normalize_body_text("Captured output from `date`.", false)
            .expect("backtick-wrapped inline code should be allowed");

        assert_eq!(body, "Captured output from `date`.");
    }

    #[test]
    fn body_validation_accepts_normal_markdown_text() {
        let body = normalize_body_text("- Completed validation.\n- No placeholders remain.", false)
            .expect("body should pass");

        assert_eq!(body, "- Completed validation.\n- No placeholders remain.");
    }

    #[test]
    fn skip_body_validation_allows_literal_shell_text() {
        let repo_root = temp_repo_root("post-step-skip-body-validation");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "1".to_string(),
            title: "Literal shell example".to_string(),
            body: Some("Example output: $(date)".to_string()),
            body_file: None,
            body_stdin: false,
            force: false,
            skip_validation: false,
            skip_body_validation: true,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::success();

        let result = execute(&cli, &poster).expect("skip flag should allow literal body text");

        assert_eq!(result, "Step 1 posted to EvaLok/schema-org-json-ld#834");
        assert_eq!(poster.posted_bodies().len(), 1);
    }

    #[test]
    fn skip_validation_allows_non_standard_step_ids() {
        let repo_root = temp_repo_root("post-step-skip-validation");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "11".to_string(),
            title: "Non-standard step".to_string(),
            body: Some("Posted intentionally.".to_string()),
            body_file: None,
            body_stdin: false,
            force: false,
            skip_validation: true,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::success();

        let result = execute(&cli, &poster).expect("skip validation should allow the step");

        assert_eq!(result, "Step 11 posted to EvaLok/schema-org-json-ld#834");
    }

    #[test]
    fn execute_rejects_whitespace_only_step_ids() {
        let repo_root = temp_repo_root("post-step-empty-step");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "   ".to_string(),
            title: "Whitespace step".to_string(),
            body: Some("Body.".to_string()),
            body_file: None,
            body_stdin: false,
            force: false,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::success();

        let error = execute(&cli, &poster).expect_err("whitespace step should fail");

        assert_eq!(error, "step must not be empty");
    }

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();

        assert!(help.contains("--issue"));
        assert!(help.contains("--step"));
        assert!(help.contains("--title"));
        assert!(help.contains("--body"));
        assert!(help.contains("--body-file"));
        assert!(help.contains("--body-stdin"));
        assert!(help.contains("--force"));
        assert!(help.contains("--skip-validation"));
        assert!(help.contains("--skip-body-validation"));
        assert!(help.contains("--repo-root"));
    }

    #[test]
    fn execute_allows_posting_when_no_existing_comments_exist() {
        let repo_root = temp_repo_root("post-step-no-existing-comments");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "1".to_string(),
            title: "Check for input-from-eva issues".to_string(),
            body: Some("Found 2 open issues.".to_string()),
            body_file: None,
            body_stdin: false,
            force: false,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::with_existing_comments(&[]);

        let result = execute(&cli, &poster).expect("execute should succeed");

        assert_eq!(result, "Step 1 posted to EvaLok/schema-org-json-ld#834");
        assert_eq!(poster.posted_bodies().len(), 1);
    }

    #[test]
    fn execute_allows_posting_when_different_step_exists() {
        let repo_root = temp_repo_root("post-step-different-step");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "1".to_string(),
            title: "Check for input-from-eva issues".to_string(),
            body: Some("Found 2 open issues.".to_string()),
            body_file: None,
            body_stdin: false,
            force: false,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::with_existing_comments(&[
			"> **[main-orchestrator]** | Cycle 197 | Step 5.11\n\n### Summarize completion checks\n\nDone.",
		]);

        let result = execute(&cli, &poster).expect("execute should succeed");

        assert_eq!(result, "Step 1 posted to EvaLok/schema-org-json-ld#834");
        assert_eq!(poster.posted_bodies().len(), 1);
    }

    #[test]
    fn execute_rejects_duplicate_step_ids() {
        let repo_root = temp_repo_root("post-step-duplicate-step");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "1".to_string(),
            title: "Check for input-from-eva issues".to_string(),
            body: Some("Found 2 open issues.".to_string()),
            body_file: None,
            body_stdin: false,
            force: false,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::with_existing_comments(&[
			"> **[main-orchestrator]** | Cycle 197 | Step 1\n\n### Earlier update\n\nAlready posted.",
		]);

        let error = execute(&cli, &poster).expect_err("duplicate step should fail");

        assert_eq!(
            error,
            "Step 1 already posted on issue #834. Use a different step ID or --force to override."
        );
        assert!(poster.posted_bodies().is_empty());
    }

    #[test]
    fn execute_allows_duplicate_step_ids_with_force() {
        let repo_root = temp_repo_root("post-step-force-duplicate");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "1".to_string(),
            title: "Check for input-from-eva issues".to_string(),
            body: Some("Found 2 open issues.".to_string()),
            body_file: None,
            body_stdin: false,
            force: true,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::with_existing_comments(&[
			"> **[main-orchestrator]** | Cycle 197 | Step 1\n\n### Earlier update\n\nAlready posted.",
		]);

        let result = execute(&cli, &poster).expect("force should bypass duplicate detection");

        assert_eq!(result, "Step 1 posted to EvaLok/schema-org-json-ld#834");
        assert_eq!(poster.posted_bodies().len(), 1);
    }

    #[test]
    fn execute_rejects_duplicate_step_ids_when_one_of_multiple_comments_matches() {
        let repo_root = temp_repo_root("post-step-one-of-many-duplicate");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "1".to_string(),
            title: "Check for input-from-eva issues".to_string(),
            body: Some("Found 2 open issues.".to_string()),
            body_file: None,
            body_stdin: false,
            force: false,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::with_existing_comments(&[
			"General discussion comment",
			"> **[main-orchestrator]** | Cycle 197 | Step 5.11\n\n### Different step\n\nDone.",
			"> **[main-orchestrator]** | Cycle 198 | Step 1\n\n### Matching step\n\nAlready posted.",
		]);

        let error = execute(&cli, &poster).expect_err("matching duplicate should fail");

        assert_eq!(
            error,
            "Step 1 already posted on issue #834. Use a different step ID or --force to override."
        );
        assert!(poster.posted_bodies().is_empty());
    }

    #[test]
    fn has_matching_step_comment_matches_special_character_steps_across_cycles() {
        assert!(has_matching_step_comment(
			&[
				"> **[main-orchestrator]** | Cycle 197 | Step 5.11\n\n### Earlier update\n\nAlready posted."
					.to_string(),
			],
			"5.11"
		));
    }

    #[test]
    fn has_matching_step_comment_rejects_partial_step_matches() {
        assert!(!has_matching_step_comment(
			&[
				"> **[main-orchestrator]** | Cycle 198 | Step 10\n\n### Different step\n\nAlready posted."
					.to_string(),
			],
			"1"
		));
    }

    #[test]
    fn has_matching_step_comment_ignores_quoted_headers() {
        assert!(!has_matching_step_comment(
            &[concat!(
				"Reviewer note\n\n",
				"Quoting a previous update:\n",
				"> **[main-orchestrator]** | Cycle 198 | Step 1\n\n### Matching step\n\nAlready posted."
			)
            .to_string()],
            "1"
        ));
    }

    #[test]
    fn parse_paginated_json_parses_single_page_output() {
        let comments =
            parse_paginated_json(r#"[{"body":"hello"}]"#).expect("single page should parse");

        assert_eq!(comment_bodies(&comments), vec!["hello"]);
    }

    #[test]
    fn parse_paginated_json_parses_newline_separated_pages() {
        let comments = parse_paginated_json("[{\"body\":\"a\"}]\n[{\"body\":\"b\"}]")
            .expect("newline-separated pages should parse");

        assert_eq!(comment_bodies(&comments), vec!["a", "b"]);
    }

    #[test]
    fn parse_paginated_json_parses_concatenated_pages_without_whitespace() {
        let comments = parse_paginated_json(r#"[{"body":"a"}][{"body":"b"}]"#)
            .expect("concatenated pages should parse");

        assert_eq!(comment_bodies(&comments), vec!["a", "b"]);
    }

    #[test]
    fn parse_paginated_json_returns_empty_vec_for_empty_output() {
        let comments = parse_paginated_json("").expect("empty output should be accepted");

        assert!(comments.is_empty());
    }

    #[test]
    fn parse_paginated_json_errors_on_malformed_output() {
        let error = parse_paginated_json("not json").expect_err("malformed output should fail");

        assert!(error.contains("failed to parse gh api paginated output page 1"));
    }

    #[test]
    fn execute_fails_closed_when_existing_comment_lookup_fails() {
        let repo_root = temp_repo_root("post-step-existing-comment-lookup-error");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "1".to_string(),
            title: "Check for input-from-eva issues".to_string(),
            body: Some("Found 2 open issues.".to_string()),
            body_file: None,
            body_stdin: false,
            force: false,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::fetch_failing("gh api failed with status 1: rate limited");

        let error = execute(&cli, &poster).expect_err("lookup failure should fail");

        assert_eq!(error, "gh api failed with status 1: rate limited");
        assert!(poster.posted_bodies().is_empty());
    }

    fn temp_repo_root(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("{prefix}-{unique}"));
        fs::create_dir_all(path.join("docs")).unwrap();
        path
    }

    fn write_state_json(repo_root: &Path, content: &str) {
        fs::write(repo_root.join("docs/state.json"), content).unwrap();
    }

    fn comment_bodies(comments: &[Value]) -> Vec<&str> {
        comments
            .iter()
            .filter_map(|comment| comment.get("body").and_then(Value::as_str))
            .collect()
    }

    // --- Ordering enforcement tests ---

    #[test]
    fn check_step_ordering_allows_step_0_with_no_existing_comments() {
        // Step 0 has no mandatory predecessors, so it's always allowed.
        assert!(check_step_ordering(&[], "0").is_ok());
    }

    #[test]
    fn check_step_ordering_allows_step_1_when_predecessor_0_is_present() {
        let existing = vec![
            "> **[main-orchestrator]** | Cycle 198 | Step 0\n\n### Init\n\nDone.".to_string(),
            "> **[main-orchestrator]** | Cycle 198 | Step 0.5\n\n### Input scan\n\nDone."
                .to_string(),
            "> **[main-orchestrator]** | Cycle 198 | Step 0.6\n\n### Scan\n\nDone.".to_string(),
        ];
        assert!(check_step_ordering(&existing, "1").is_ok());
    }

    #[test]
    fn check_step_ordering_rejects_step_4_when_predecessors_missing() {
        // Posting step 4 when 0.5, 0.6, 1, 1.1, 2, 3 haven't been posted yet.
        let existing =
            vec!["> **[main-orchestrator]** | Cycle 198 | Step 0\n\n### Init\n\nDone.".to_string()];
        let err = check_step_ordering(&existing, "4").expect_err("should fail ordering check");
        assert!(err.contains("Cannot post step 4"));
        assert!(err.contains("0.5"));
        assert!(err.contains("3"));
    }

    #[test]
    fn check_step_ordering_rejects_step_7_when_step_5_and_6_missing() {
        // Posting step 7 when 5 and 6 haven't been posted.
        let mandatory_up_to_4 = ["0", "0.5", "0.6", "1", "1.1", "2", "3", "4"];
        let existing: Vec<String> = mandatory_up_to_4
            .iter()
            .map(|s| {
                format!("> **[main-orchestrator]** | Cycle 198 | Step {s}\n\n### Title\n\nDone.")
            })
            .collect();
        let err = check_step_ordering(&existing, "7").expect_err("should fail ordering check");
        assert!(err.contains("Cannot post step 7"));
        assert!(err.contains("5"));
        assert!(err.contains("6"));
    }

    #[test]
    fn check_step_ordering_allows_step_4_when_all_predecessors_present() {
        let mandatory_predecessors = ["0", "0.5", "0.6", "1", "1.1", "2", "3"];
        let existing: Vec<String> = mandatory_predecessors
            .iter()
            .map(|s| {
                format!("> **[main-orchestrator]** | Cycle 198 | Step {s}\n\n### Title\n\nDone.")
            })
            .collect();
        assert!(check_step_ordering(&existing, "4").is_ok());
    }

    #[test]
    fn execute_rejects_out_of_order_step() {
        let repo_root = temp_repo_root("post-step-ordering-reject");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "4".to_string(),
            title: "Pipeline check".to_string(),
            body: Some("Pipeline passed.".to_string()),
            body_file: None,
            body_stdin: false,
            force: false,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        // Only step 0 posted; mandatory predecessors 0.5, 0.6, 1, 1.1, 2, 3 are missing.
        let poster = RecordingPoster::with_existing_comments(&[
            "> **[main-orchestrator]** | Cycle 198 | Step 0\n\n### Init\n\nDone.",
        ]);

        let error = execute(&cli, &poster).expect_err("out-of-order post should fail");

        assert!(error.contains("Cannot post step 4"));
        assert!(poster.posted_bodies().is_empty());
    }

    #[test]
    fn execute_allows_out_of_order_step_with_force() {
        let repo_root = temp_repo_root("post-step-ordering-force");
        write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
        let cli = Cli {
            issue: 834,
            step: "4".to_string(),
            title: "Pipeline check".to_string(),
            body: Some("Pipeline passed.".to_string()),
            body_file: None,
            body_stdin: false,
            force: true,
            skip_validation: false,
            skip_body_validation: false,
            repo_root: repo_root.clone(),
        };
        let poster = RecordingPoster::with_existing_comments(&[
            "> **[main-orchestrator]** | Cycle 198 | Step 0\n\n### Init\n\nDone.",
        ]);

        let result = execute(&cli, &poster).expect("--force should bypass ordering check");

        assert_eq!(result, "Step 4 posted to EvaLok/schema-org-json-ld#834");
        assert_eq!(poster.posted_bodies().len(), 1);
    }

    #[test]
    fn check_step_ordering_skips_non_standard_step_ids() {
        // Non-standard steps (not in VALID_STEP_IDS) bypass the ordering check.
        assert!(check_step_ordering(&[], "custom-step").is_ok());
    }

    // --- C5.5 → C5 → C5.1 close-out ordering regression tests ---
    // Validates the canonical sequence matches the real close-out execution path:
    // C5.5 (pipeline gate) runs first, then C5 (commit docs), then C5.1 (receipt validation).

    fn mandatory_comments_up_to(steps: &[&str]) -> Vec<String> {
        steps
            .iter()
            .map(|s| {
                format!("> **[main-orchestrator]** | Cycle 530 | Step {s}\n\n### Title\n\nDone.")
            })
            .collect()
    }

    #[test]
    fn check_step_ordering_allows_c5_5_after_c4_1_and_c4_5() {
        // C5.5 is the first close-out step in the canonical order; it only requires
        // C4.1 and C4.5 (and the pre-close-out mandatory steps) to be present.
        let existing = mandatory_comments_up_to(&[
            "0", "0.5", "0.6", "1", "1.1", "2", "3", "4", "5", "6", "7", "8", "9", "C1", "C2",
            "C3", "C4.1", "C4.5",
        ]);
        assert!(
            check_step_ordering(&existing, "C5.5").is_ok(),
            "C5.5 should be allowed when C4.1 and C4.5 are present"
        );
    }

    #[test]
    fn check_step_ordering_allows_c5_after_c5_5() {
        // C5 must be posted after C5.5 in the new canonical order.
        let existing = mandatory_comments_up_to(&[
            "0", "0.5", "0.6", "1", "1.1", "2", "3", "4", "5", "6", "7", "8", "9", "C1", "C2",
            "C3", "C4.1", "C4.5", "C5.5",
        ]);
        assert!(
            check_step_ordering(&existing, "C5").is_ok(),
            "C5 should be allowed when C5.5 is already posted"
        );
    }

    #[test]
    fn check_step_ordering_allows_c5_1_after_c5_5_and_c5() {
        // C5.1 requires both C5.5 and C5 to already be posted.
        let existing = mandatory_comments_up_to(&[
            "0", "0.5", "0.6", "1", "1.1", "2", "3", "4", "5", "6", "7", "8", "9", "C1", "C2",
            "C3", "C4.1", "C4.5", "C5.5", "C5",
        ]);
        assert!(
            check_step_ordering(&existing, "C5.1").is_ok(),
            "C5.1 should be allowed when both C5.5 and C5 are posted"
        );
    }

    #[test]
    fn check_step_ordering_rejects_c5_before_c5_5() {
        // Posting C5 before C5.5 must be rejected — the pipeline gate (C5.5)
        // must run before the docs commit (C5).
        let existing = mandatory_comments_up_to(&[
            "0", "0.5", "0.6", "1", "1.1", "2", "3", "4", "5", "6", "7", "8", "9", "C1", "C2",
            "C3", "C4.1", "C4.5",
        ]);
        let err = check_step_ordering(&existing, "C5")
            .expect_err("C5 without C5.5 should fail ordering check");
        assert!(
            err.contains("Cannot post step C5"),
            "error should name the blocked step"
        );
        assert!(
            err.contains("C5.5"),
            "error should list C5.5 as the missing mandatory predecessor"
        );
    }

    #[test]
    fn check_step_ordering_rejects_c5_1_before_c5_5() {
        // Posting C5.1 before C5.5 must also be rejected.
        let existing = mandatory_comments_up_to(&[
            "0", "0.5", "0.6", "1", "1.1", "2", "3", "4", "5", "6", "7", "8", "9", "C1", "C2",
            "C3", "C4.1", "C4.5",
        ]);
        let err = check_step_ordering(&existing, "C5.1")
            .expect_err("C5.1 without C5.5 should fail ordering check");
        assert!(err.contains("Cannot post step C5.1"));
        assert!(err.contains("C5.5"));
    }
}
