use clap::{ArgGroup, Parser};
use serde_json::json;
use state_schema::current_cycle_from_state;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Output, Stdio};

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const ORCHESTRATOR_SIGNATURE: &str = "[main-orchestrator]";
const VALID_STEP_IDS: [&str; 35] = [
	"0", "0.5", "0.6", "1", "1.1", "1.5", "2", "2.5", "3", "4", "5", "5.5", "5.6", "5.8",
	"5.9", "5.10", "5.11", "5.12", "5.13", "6", "7", "8", "9", "10", "C1", "C2", "C3",
	"C4.1", "C4.5", "C5", "C5.1", "C5.5", "C6", "C7", "C8",
];

#[derive(Parser, Debug)]
#[command(name = "post-step")]
#[command(group(
	ArgGroup::new("body_source")
		.required(true)
		.args(["body", "body_file"])
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

	/// Skip step ID validation for non-standard step names
	#[arg(long)]
	skip_validation: bool,

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

	runner.post_comment(cli.issue, &comment)?;

	Ok(format!("Step {} posted to {MAIN_REPO}#{}", step, cli.issue))
}

fn resolve_body(cli: &Cli) -> Result<String, String> {
	// Clap enforces this for real CLI parsing, but direct struct construction in unit tests
	// or future internal callers can bypass parser validation, so keep a fail-closed check here.
	match (&cli.body, &cli.body_file) {
		(Some(_), Some(_)) => Err("exactly one of --body or --body-file must be provided".to_string()),
		(None, None) => Err("exactly one of --body or --body-file must be provided".to_string()),
		(Some(body), None) => normalize_body_text(body),
		(None, Some(path)) => {
			let content = fs::read_to_string(path)
				.map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
			normalize_body_text(&content)
		}
	}
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

fn normalize_body_text(body: &str) -> Result<String, String> {
	let normalized = body.trim_end_matches(['\r', '\n']);
	validate_required_text("body", normalized)?;
	Ok(normalized.to_string())
}

fn format_comment(cycle: u64, step: &str, title: &str, body: &str) -> String {
	format!(
		"> **{ORCHESTRATOR_SIGNATURE}** | Cycle {cycle} | Step {step}\n\n### {title}\n\n{body}"
	)
}

trait CommentPoster {
	fn post_comment(&self, issue: u64, body: &str) -> Result<(), String>;
}

struct GhCommandRunner;

impl CommentPoster for GhCommandRunner {
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
		error: Option<String>,
	}

	impl RecordingPoster {
		fn success() -> Self {
			Self {
				body: std::sync::Mutex::new(Vec::new()),
				error: None,
			}
		}

		fn failing(error: &str) -> Self {
			Self {
				body: std::sync::Mutex::new(Vec::new()),
				error: Some(error.to_string()),
			}
		}

		fn posted_bodies(&self) -> Vec<String> {
			self.body.lock().unwrap().clone()
		}
	}

	impl CommentPoster for RecordingPoster {
		fn post_comment(&self, _issue: u64, body: &str) -> Result<(), String> {
			if let Some(error) = &self.error {
				return Err(error.clone());
			}

			self.body.lock().unwrap().push(body.to_string());
			Ok(())
		}
	}

	#[test]
	fn format_comment_includes_signature_cycle_step_title_and_body() {
		let comment = format_comment(198, "1", "Check for input-from-eva issues", "Found 2 open issues.");

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
			skip_validation: false,
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
			skip_validation: false,
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
			skip_validation: false,
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
			skip_validation: false,
			repo_root: repo_root.clone(),
		};
		let poster = RecordingPoster::failing("gh api failed with status 1: rate limited");

		let error = execute(&cli, &poster).expect_err("execute should fail");

		assert_eq!(error, "gh api failed with status 1: rate limited");
	}

	#[test]
	fn cli_requires_exactly_one_body_source() {
		let missing = Cli::try_parse_from(["post-step", "--issue", "834", "--step", "1", "--title", "Test"]);
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
	}

	#[test]
	fn valid_step_ids_are_accepted() {
		for step in ["0", "0.5", "1", "C1", "C4.5", "10"] {
			assert!(validate_step_id(step).is_ok(), "expected {step} to be valid");
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
	fn skip_validation_allows_non_standard_step_ids() {
		let repo_root = temp_repo_root("post-step-skip-validation");
		write_state_json(&repo_root, r#"{"last_cycle":{"number":198}}"#);
		let cli = Cli {
			issue: 834,
			step: "11".to_string(),
			title: "Non-standard step".to_string(),
			body: Some("Posted intentionally.".to_string()),
			body_file: None,
			skip_validation: true,
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
			skip_validation: false,
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
		assert!(help.contains("--skip-validation"));
		assert!(help.contains("--repo-root"));
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
}
