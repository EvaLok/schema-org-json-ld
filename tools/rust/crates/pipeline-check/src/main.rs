use chrono::NaiveDate;
use clap::Parser;
use serde::Serialize;
use serde_json::Value;
use state_schema::{current_cycle_from_state, current_utc_timestamp, read_state_value};
use std::fs;
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

const HOUSEKEEPING_FINDINGS_KEY: &str = "items_needing_attention";
const CYCLE_STATUS_IN_FLIGHT_PATH: &str = "/concurrency/in_flight";
const CYCLE_STATUS_DIRECTIVES_PATH: &str = "/eva_input/comments_since_last_cycle";
const DERIVE_METRICS_TOOL_NAME: &str = "derive-metrics";
const DERIVE_METRICS_WRAPPER_PATH: &str = "tools/derive-metrics";
const ARTIFACT_VERIFY_STEP_NAME: &str = "artifact-verify";
const DOC_VALIDATION_STEP_NAME: &str = "doc-validation";
const STEP_COMMENTS_STEP_NAME: &str = "step-comments";
const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const STEP_COMMENT_THRESHOLD: usize = 17;
const ORCHESTRATOR_SIGNATURE: &str = "> **[main-orchestrator]**";
const MANDATORY_STEPS: [(&str, u64); 22] = [
	("0", 0),
	("0.5", 0),
	("1", 0),
	("2", 0),
	("3", 0),
	("4", 0),
	("5", 0),
	("6", 0),
	("7", 0),
	("8", 0),
	("9", 0),
	("C1", 0),
	("C2", 0),
	("C3", 0),
	("C4.1", 0),
	("C4.5", 0),
	("C5", 0),
	("C5.1", 256),
	("C5.5", 0),
	("C6", 0),
	("C7", 0),
	("C8", 0),
];
// Keep this list aligned with the orchestrator checklist steps that are expected to
// produce post-step comments. The pass threshold stays lower because some steps are
// conditional, but missing steps should still be surfaced in WARN output.
const EXPECTED_STEP_IDS: [&str; 25] = [
	"0", "0.5", "0.6", "1", "1.1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "C1",
	"C2", "C3", "C4.1", "C4.5", "C5", "C5.1", "C5.5", "C6", "C7", "C8",
];
const REVIEW_LAST_CYCLE_PATH: &str = "/review_agent/last_review_cycle";
const DERIVE_METRICS_FIELDS: [&str; 9] = [
	"total_dispatches",
	"resolved",
	"merged",
	"in_flight",
	"produced_pr",
	"closed_without_pr",
	"reviewed_awaiting_eva",
	"dispatch_to_pr_rate",
	"pr_merge_rate",
];
const DERIVE_METRICS_RATE_FIELDS: [&str; 2] = ["dispatch_to_pr_rate", "pr_merge_rate"];

#[derive(Parser)]
#[command(name = "pipeline-check")]
struct Cli {
    #[arg(long)]
    repo_root: PathBuf,

    #[arg(long)]
    cycle: Option<u64>,

    #[arg(long)]
    json: bool,
}

#[derive(Clone, Copy, Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
enum StepStatus {
	Pass,
	Warn,
	Fail,
	Error,
}

#[derive(Clone, Copy, Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
enum Severity {
	Blocking,
	Warning,
}

#[derive(Serialize)]
struct PipelineReport {
	cycle: u64,
	overall: StepStatus,
	has_blocking_findings: bool,
	timestamp: String,
	steps: Vec<StepReport>,
}

#[derive(Serialize)]
struct StepReport {
	name: &'static str,
	status: StepStatus,
	severity: Severity,
	exit_code: Option<i32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	detail: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	findings: Option<usize>,
	#[serde(skip_serializing_if = "Option::is_none")]
	summary: Option<String>,
}

struct ToolSpec {
    display_name: &'static str,
    wrapper_relative_path: &'static str,
    args: Vec<String>,
    kind: ToolKind,
}

enum ToolKind {
    MetricSnapshot,
    FieldInventory,
    HousekeepingScan,
    CycleStatus,
    StateInvariants,
	DeriveMetrics,
}

struct ExecutionResult {
    exit_code: Option<i32>,
    stdout: String,
}

trait CommandRunner {
    fn run(&self, script_path: &Path, args: &[String]) -> Result<ExecutionResult, String>;
	fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String>;
}

struct ProcessRunner;

impl CommandRunner for ProcessRunner {
    fn run(&self, script_path: &Path, args: &[String]) -> Result<ExecutionResult, String> {
        let output = Command::new("bash")
            .arg(script_path)
            .args(args)
            .output()
            .map_err(|e| format!("failed to spawn command: {}", e))?;

        Ok(ExecutionResult {
            exit_code: output.status.code(),
            stdout: String::from_utf8_lossy(&output.stdout).trim().to_string(),
        })
    }

	fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
		let output = Command::new("gh")
			.arg("api")
			.arg(format!("repos/{MAIN_REPO}/issues/{issue}/comments"))
			.arg("--paginate")
			.arg("--jq")
			.arg(".[] | .body")
			.output()
			.map_err(|error| format!("failed to execute gh api: {}", error))?;

		if !output.status.success() {
			return Err(command_failure_message("gh api", &output));
		}

		Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
	}
}

fn main() {
    let cli = Cli::parse();
    let cycle = match cli.cycle {
        Some(cycle) => cycle,
        None => match current_cycle_from_state(&cli.repo_root) {
            Ok(cycle) => cycle,
            Err(error) => {
                eprintln!("Error: {}", error);
                std::process::exit(1);
            }
        },
    };
    let runner = ProcessRunner;
    let report = run_pipeline(&cli.repo_root, cycle, &runner);
    let exit_code = pipeline_exit_code(&report.steps);

    if cli.json {
        match serde_json::to_string_pretty(&report) {
            Ok(out) => println!("{}", out),
            Err(e) => {
                eprintln!("Error: Failed to serialize pipeline report to JSON: {}", e);
                std::process::exit(2);
            }
        }
    } else {
        print_human_report(&report);
    }

    std::process::exit(exit_code);
}

fn run_pipeline(repo_root: &Path, cycle: u64, runner: &dyn CommandRunner) -> PipelineReport {
	let specs = [
		ToolSpec {
			display_name: "metric-snapshot",
			wrapper_relative_path: "tools/metric-snapshot",
			args: vec![
				"--json".to_string(),
				"--cycle".to_string(),
				cycle.to_string(),
				"--repo-root".to_string(),
				repo_root.display().to_string(),
			],
			kind: ToolKind::MetricSnapshot,
		},
		ToolSpec {
			display_name: "field-inventory",
			wrapper_relative_path: "tools/check-field-inventory-rs",
			args: vec![
				"--cycle".to_string(),
				cycle.to_string(),
				"--repo-root".to_string(),
				repo_root.display().to_string(),
			],
			kind: ToolKind::FieldInventory,
		},
		ToolSpec {
			display_name: "housekeeping-scan",
			wrapper_relative_path: "tools/housekeeping-scan",
			args: vec![
				"--json".to_string(),
				"--repo-root".to_string(),
				repo_root.display().to_string(),
			],
			kind: ToolKind::HousekeepingScan,
		},
		ToolSpec {
			display_name: "cycle-status",
			wrapper_relative_path: "tools/cycle-status",
			args: vec![
				"--json".to_string(),
				"--repo-root".to_string(),
				repo_root.display().to_string(),
			],
			kind: ToolKind::CycleStatus,
		},
		ToolSpec {
			display_name: "state-invariants",
			wrapper_relative_path: "tools/state-invariants",
			args: vec![
				"--json".to_string(),
				"--repo-root".to_string(),
				repo_root.display().to_string(),
			],
			kind: ToolKind::StateInvariants,
		},
		ToolSpec {
			display_name: DERIVE_METRICS_TOOL_NAME,
			wrapper_relative_path: DERIVE_METRICS_WRAPPER_PATH,
			args: vec![],
			kind: ToolKind::DeriveMetrics,
		},
	];

	let mut steps = Vec::new();
	steps.extend(specs.iter().map(|spec| run_step(repo_root, spec, runner)));
	steps.push(verify_artifacts(repo_root));
	let pipeline_status = pipeline_overall_status(&steps);
	steps.push(verify_doc_validation(repo_root, pipeline_status, runner));
	steps.push(verify_step_comments(repo_root, runner));
	let overall = pipeline_overall_status(&steps);
	let has_blocking_findings = steps.iter().any(|step| step.status == StepStatus::Fail);

	PipelineReport {
		cycle,
		overall,
		has_blocking_findings,
		timestamp: current_utc_timestamp(),
		steps,
	}
}

fn run_step(repo_root: &Path, spec: &ToolSpec, runner: &dyn CommandRunner) -> StepReport {
    let script_path = repo_root.join(spec.wrapper_relative_path);
    let execution = match runner.run(&script_path, &spec.args) {
        Ok(output) => output,
        Err(err) => {
            return StepReport {
                name: spec.display_name,
                status: StepStatus::Error,
                severity: severity_for_kind(&spec.kind),
                exit_code: None,
                detail: Some(format!("Tool '{}' failed: {}", spec.display_name, err)),
                findings: None,
                summary: None,
            };
        }
    };

	match spec.kind {
		ToolKind::DeriveMetrics => classify_derive_metrics_step(repo_root, spec.display_name, execution),
		_ => classify_step(spec.display_name, &spec.kind, execution),
	}
}

fn classify_step(name: &'static str, kind: &ToolKind, execution: ExecutionResult) -> StepReport {
	let severity = severity_for_kind(kind);
	let mut step = StepReport {
		name,
		status: status_from_exit_code(execution.exit_code, severity),
		severity,
		exit_code: execution.exit_code,
		detail: None,
		findings: None,
		summary: None,
	};

	match kind {
		ToolKind::FieldInventory => {
			if !execution.stdout.is_empty() {
				step.detail = Some(execution.stdout);
			}
		}
		ToolKind::MetricSnapshot => {
			if let Some(parsed) = parse_json(&execution.stdout) {
				step.detail = parsed
					.get("summary")
					.and_then(Value::as_str)
					.map(str::to_string)
					.or_else(|| {
						parsed
							.get("checks")
							.and_then(Value::as_array)
							.map(|checks| {
								let passing =
									checks.iter().filter(|check| is_check_passing(check)).count();
								format!("{}/{} checks", passing, checks.len())
							})
					});
			} else {
				step.status = StepStatus::Error;
				step.detail = Some(format!("invalid JSON output from {}", name));
			}
		}
		ToolKind::HousekeepingScan => {
			if let Some(parsed) = parse_json(&execution.stdout) {
				let findings = parsed
					.get(HOUSEKEEPING_FINDINGS_KEY)
					.and_then(Value::as_u64)
					.and_then(|v| usize::try_from(v).ok())
					.unwrap_or(0);
				step.findings = Some(findings);
				step.detail = Some(format!("{} findings", findings));
			} else {
				step.status = StepStatus::Error;
				step.detail = Some(format!("invalid JSON output from {}", name));
			}
		}
		ToolKind::StateInvariants => {
			if let Some(parsed) = parse_json(&execution.stdout) {
				let passed = parsed.get("passed").and_then(Value::as_u64).unwrap_or(0);
				let failed = parsed.get("failed").and_then(Value::as_u64).unwrap_or(0);
				step.detail = Some(format!("{}/{} invariants pass", passed, passed + failed));
			} else {
				step.status = StepStatus::Error;
				step.detail = Some(format!("invalid JSON output from {}", name));
			}
		}
		ToolKind::CycleStatus => {
			if let Some(parsed) = parse_json(&execution.stdout) {
				let in_flight = parsed
					.pointer(CYCLE_STATUS_IN_FLIGHT_PATH)
					.and_then(Value::as_u64)
					.unwrap_or(0);
				let directives = parsed
					.pointer(CYCLE_STATUS_DIRECTIVES_PATH)
					.and_then(Value::as_array)
					.map(Vec::len)
					.unwrap_or(0);
				step.summary = Some(format!(
					"{} in-flight, {} eva directives",
					in_flight, directives
				));
			} else {
				step.status = StepStatus::Error;
				step.detail = Some(format!("invalid JSON output from {}", name));
			}
		}
		ToolKind::DeriveMetrics => unreachable!("derive-metrics classification is handled separately"),
	}

	step
}

fn classify_derive_metrics_step(
	repo_root: &Path,
	name: &'static str,
	execution: ExecutionResult,
) -> StepReport {
	let severity = severity_for_kind(&ToolKind::DeriveMetrics);
	let mut step = StepReport {
		name,
		status: StepStatus::Pass,
		severity,
		exit_code: execution.exit_code,
		detail: None,
		findings: None,
		summary: None,
	};

	if execution.exit_code != Some(0) {
		step.status = StepStatus::Error;
		step.detail = Some(format!("{} exited with unexpected status {:?}", name, execution.exit_code));
		return step;
	}

	let Some(derived_metrics) = parse_json(&execution.stdout) else {
		step.status = StepStatus::Error;
		step.detail = Some(format!("invalid JSON output from {}", name));
		return step;
	};

	match collect_derive_metrics_mismatches(repo_root, &derived_metrics) {
		Ok(mismatches) if mismatches.is_empty() => {
			step.detail = Some("tracked copilot_metrics fields match".to_string());
		}
		Ok(mismatches) => {
			step.status = StepStatus::Fail;
			step.detail = Some(mismatches.join("; "));
		}
		Err(error) => {
			step.status = StepStatus::Error;
			step.detail = Some(error);
		}
	}

	step
}

fn severity_for_kind(kind: &ToolKind) -> Severity {
	match kind {
		ToolKind::MetricSnapshot
		| ToolKind::StateInvariants
		| ToolKind::CycleStatus
		| ToolKind::DeriveMetrics => {
			Severity::Blocking
		}
		ToolKind::FieldInventory | ToolKind::HousekeepingScan => {
			Severity::Warning
		}
	}
}

fn status_from_exit_code(exit_code: Option<i32>, severity: Severity) -> StepStatus {
	match exit_code {
		Some(0) => StepStatus::Pass,
		Some(1) => match severity {
			Severity::Blocking => StepStatus::Fail,
			Severity::Warning => StepStatus::Warn,
		},
		_ => StepStatus::Error,
	}
}

fn pipeline_overall_status(steps: &[StepReport]) -> StepStatus {
	if steps
		.iter()
		.any(|step| matches!(step.status, StepStatus::Fail | StepStatus::Error))
	{
		StepStatus::Fail
	} else {
		StepStatus::Pass
	}
}

fn parse_json(raw: &str) -> Option<Value> {
    serde_json::from_str(raw).ok()
}

fn command_failure_message(command: &str, output: &std::process::Output) -> String {
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

fn collect_derive_metrics_mismatches(repo_root: &Path, derived_metrics: &Value) -> Result<Vec<String>, String> {
	let state_value = read_state_value(repo_root)?;
	let current_metrics = state_value
		.pointer("/copilot_metrics")
		.and_then(Value::as_object)
		.ok_or_else(|| "missing object: /copilot_metrics".to_string())?;
	let derived_metrics = derived_metrics
		.as_object()
		.ok_or_else(|| "derive-metrics output must be a JSON object".to_string())?;

	let mut mismatches = Vec::new();
	for field in DERIVE_METRICS_FIELDS {
		if DERIVE_METRICS_RATE_FIELDS.contains(&field) {
			let expected = derived_metrics
				.get(field)
				.and_then(Value::as_str)
				.ok_or_else(|| format!("derive-metrics output missing string field '{}'", field))?;
			match current_metrics.get(field).and_then(Value::as_str) {
				Some(actual) if actual == expected => {}
				Some(actual) => mismatches.push(format!(
					"copilot_metrics.{} expected {} but found {}",
					field, expected, actual
				)),
				None => mismatches.push(format!(
					"copilot_metrics.{} is missing or not a string",
					field
				)),
			}
			continue;
		}

		let expected = derived_metrics
			.get(field)
			.and_then(Value::as_i64)
			.ok_or_else(|| format!("derive-metrics output missing integer field '{}'", field))?;
		match current_metrics.get(field).and_then(Value::as_i64) {
			Some(actual) if actual == expected => {}
			Some(actual) => mismatches.push(format!(
				"copilot_metrics.{} expected {} but found {}",
				field, expected, actual
			)),
			None => mismatches.push(format!(
				"copilot_metrics.{} is missing or not an integer",
				field
			)),
		}
	}

	Ok(mismatches)
}

fn is_check_passing(check: &Value) -> bool {
    check.get("pass").and_then(Value::as_bool).unwrap_or(false)
}

fn verify_artifacts(repo_root: &Path) -> StepReport {
	verify_artifacts_for_date(repo_root, &current_utc_timestamp()[..10])
}

fn verify_doc_validation(repo_root: &Path, pipeline_status: StepStatus, runner: &dyn CommandRunner) -> StepReport {
	verify_doc_validation_for_date(repo_root, &current_utc_timestamp()[..10], pipeline_status, runner)
}

fn verify_doc_validation_for_date(
	repo_root: &Path,
	today: &str,
	pipeline_status: StepStatus,
	runner: &dyn CommandRunner,
) -> StepReport {
	let state = match read_state_value(repo_root) {
		Ok(state) => state,
		Err(error) => {
			return StepReport {
				name: DOC_VALIDATION_STEP_NAME,
				status: StepStatus::Error,
				severity: Severity::Blocking,
				exit_code: None,
				detail: Some(error),
				findings: None,
				summary: None,
			};
		}
	};

	let phase = state.pointer("/cycle_phase/phase").and_then(Value::as_str);
	if phase != Some("close_out") {
		return StepReport {
			name: DOC_VALIDATION_STEP_NAME,
			status: StepStatus::Pass,
			severity: Severity::Blocking,
			exit_code: None,
			detail: Some("skipped: no documentation entries to validate yet".to_string()),
			findings: None,
			summary: None,
		};
	}

	let cycle = match current_cycle_from_state(repo_root) {
		Ok(cycle) => cycle,
		Err(error) => {
			return StepReport {
				name: DOC_VALIDATION_STEP_NAME,
				status: StepStatus::Error,
				severity: Severity::Blocking,
				exit_code: None,
				detail: Some(error),
				findings: None,
				summary: None,
			};
		}
	};
	let Some(worklog_path) = (match latest_worklog_entry_for_date(repo_root, today) {
		Ok(path) => path,
		Err(error) => {
			return StepReport {
				name: DOC_VALIDATION_STEP_NAME,
				status: StepStatus::Error,
				severity: Severity::Blocking,
				exit_code: None,
				detail: Some(error),
				findings: None,
				summary: None,
			};
		}
	}) else {
		return StepReport {
			name: DOC_VALIDATION_STEP_NAME,
			status: StepStatus::Pass,
			severity: Severity::Blocking,
			exit_code: None,
			detail: Some("skipped: no documentation entries to validate yet".to_string()),
			findings: None,
			summary: None,
		};
	};
	let journal_path = repo_root.join("docs/journal").join(format!("{today}.md"));
	if !journal_path.is_file() {
		return StepReport {
			name: DOC_VALIDATION_STEP_NAME,
			status: StepStatus::Pass,
			severity: Severity::Blocking,
			exit_code: None,
			detail: Some("skipped: no documentation entries to validate yet".to_string()),
			findings: None,
			summary: None,
		};
	}

	let script_path = repo_root.join("tools/validate-docs");
	let validations = [
		(
			"worklog",
			vec![
				"worklog".to_string(),
				"--file".to_string(),
				worklog_path.display().to_string(),
				"--cycle".to_string(),
				cycle.to_string(),
				"--pipeline-status".to_string(),
				step_status_label(pipeline_status).to_string(),
				"--repo-root".to_string(),
				repo_root.display().to_string(),
			],
		),
		(
			"journal",
			vec![
				"journal".to_string(),
				"--file".to_string(),
				journal_path.display().to_string(),
				"--repo-root".to_string(),
				repo_root.display().to_string(),
			],
		),
	];
	let mut failures = Vec::new();

	for (label, args) in validations {
		let execution = match runner.run(&script_path, &args) {
			Ok(execution) => execution,
			Err(error) => {
				return StepReport {
					name: DOC_VALIDATION_STEP_NAME,
					status: StepStatus::Error,
					severity: Severity::Blocking,
					exit_code: None,
					detail: Some(format!("Tool 'validate-docs' failed while validating {}: {}", label, error)),
					findings: None,
					summary: None,
				};
			}
		};

		match execution.exit_code {
			Some(0) => {}
			Some(1) => {
				if execution.stdout.is_empty() {
					failures.push(format!("{} validation failed", label));
				} else {
					failures.push(format!("{} validation failed: {}", label, execution.stdout));
				}
			}
			other => {
				return StepReport {
					name: DOC_VALIDATION_STEP_NAME,
					status: StepStatus::Error,
					severity: Severity::Blocking,
					exit_code: other,
					detail: Some(format!("{} validation exited with unexpected status {:?}", label, other)),
					findings: None,
					summary: None,
				};
			}
		}
	}

	if failures.is_empty() {
		StepReport {
			name: DOC_VALIDATION_STEP_NAME,
			status: StepStatus::Pass,
			severity: Severity::Blocking,
			exit_code: None,
			detail: Some(format!(
				"validated {} and {}",
				worklog_path.display(),
				journal_path.display()
			)),
			findings: None,
			summary: None,
		}
	} else {
		StepReport {
			name: DOC_VALIDATION_STEP_NAME,
			status: StepStatus::Fail,
			severity: Severity::Blocking,
			exit_code: Some(1),
			detail: Some(failures.join("; ")),
			findings: None,
			summary: None,
		}
	}
}

fn verify_step_comments(repo_root: &Path, runner: &dyn CommandRunner) -> StepReport {
	let state = match read_state_value(repo_root) {
		Ok(state) => state,
		Err(error) => {
			return StepReport {
				name: STEP_COMMENTS_STEP_NAME,
				status: StepStatus::Error,
				severity: Severity::Blocking,
				exit_code: None,
				detail: Some(error),
				findings: None,
				summary: None,
			};
		}
	};

	let issue = match state.pointer("/previous_cycle_issue").and_then(Value::as_u64) {
		Some(issue) => issue,
		None => {
			return StepReport {
				name: STEP_COMMENTS_STEP_NAME,
				status: StepStatus::Pass,
				severity: Severity::Blocking,
				exit_code: None,
				detail: Some(
					"skipping step comment verification: /previous_cycle_issue is not set in docs/state.json yet"
						.to_string(),
				),
				findings: None,
				summary: None,
			};
		}
	};
	let found = match fetch_step_comments_for_issue(runner, issue) {
		Ok(found) => found,
		Err(error) => {
			return StepReport {
				name: STEP_COMMENTS_STEP_NAME,
				status: StepStatus::Error,
				severity: Severity::Blocking,
				exit_code: None,
				detail: Some(error),
				findings: None,
				summary: None,
			};
		}
	};
	let cycle = match current_cycle_from_state(repo_root) {
		Ok(cycle) => cycle,
		Err(error) => {
			return StepReport {
				name: STEP_COMMENTS_STEP_NAME,
				status: StepStatus::Error,
				severity: Severity::Blocking,
				exit_code: None,
				detail: Some(error),
				findings: None,
				summary: None,
			};
		}
	};
	let issue_assessment = assess_step_comment_completeness(&found, cycle);

	StepReport {
		name: STEP_COMMENTS_STEP_NAME,
		status: issue_assessment.status,
		severity: issue_assessment.severity,
		exit_code: None,
		detail: Some(format!("issue #{}: {}", issue, issue_assessment.detail)),
		findings: Some(issue_assessment.findings),
		summary: None,
	}
}

fn fetch_step_comments_for_issue(
	runner: &dyn CommandRunner,
	issue: u64,
) -> Result<BTreeSet<&'static str>, String> {
	runner
		.fetch_issue_comment_bodies(issue)
		.map(|comment_bodies| collect_step_comment_ids(&comment_bodies))
}

fn missing_expected_step_ids(found: &BTreeSet<&'static str>) -> Vec<&'static str> {
	EXPECTED_STEP_IDS
		.iter()
		.copied()
		.filter(|step| !found.contains(step))
		.collect()
}

fn ordered_found_step_ids(found: &BTreeSet<&'static str>) -> Vec<&'static str> {
	EXPECTED_STEP_IDS
		.iter()
		.copied()
		.filter(|step| found.contains(step))
		.collect()
}

fn format_step_id_list(step_ids: &[&str]) -> String {
	if step_ids.is_empty() {
		"none".to_string()
	} else {
		step_ids.join(", ")
	}
}

fn is_mandatory_step_for_cycle(step: &str, cycle: u64) -> bool {
	MANDATORY_STEPS
		.iter()
		.any(|(mandatory_step, effective_from_cycle)| *mandatory_step == step && *effective_from_cycle <= cycle)
}

fn assess_step_comment_completeness(
	found: &BTreeSet<&'static str>,
	cycle: u64,
) -> StepCommentAssessment {
	let found_ids = ordered_found_step_ids(found);
	let missing = missing_expected_step_ids(found);
	let (mandatory_missing, optional_missing): (Vec<_>, Vec<_>) = missing
		.into_iter()
		.partition(|step| is_mandatory_step_for_cycle(step, cycle));
	let detail = format!(
		"found {} unique step comments [{}]; missing mandatory [{}]; missing optional [{}]",
		found.len(),
		format_step_id_list(&found_ids),
		format_step_id_list(&mandatory_missing),
		format_step_id_list(&optional_missing)
	);

	if found.len() < STEP_COMMENT_THRESHOLD {
		return StepCommentAssessment {
			status: StepStatus::Fail,
			severity: Severity::Blocking,
			detail: format!(
				"{}; below backstop threshold {}",
				detail, STEP_COMMENT_THRESHOLD
			),
			findings: found.len(),
		};
	}

	if !mandatory_missing.is_empty() {
		return StepCommentAssessment {
			status: StepStatus::Fail,
			severity: Severity::Blocking,
			detail,
			findings: found.len(),
		};
	}

	if !optional_missing.is_empty() {
		return StepCommentAssessment {
			status: StepStatus::Warn,
			severity: Severity::Warning,
			detail,
			findings: found.len(),
		};
	}

	StepCommentAssessment {
		status: StepStatus::Pass,
		severity: Severity::Blocking,
		detail,
		findings: found.len(),
	}
}

/// Completeness assessment for a collected set of step comments.
///
/// PASS means all expected steps were found, WARN means only optional steps are
/// missing, and FAIL means either a mandatory step is missing or the threshold
/// backstop was not met.
struct StepCommentAssessment {
	status: StepStatus,
	severity: Severity,
	detail: String,
	findings: usize,
}

/// Collect recognized orchestrator step identifiers from issue comment bodies.
///
/// Returned step IDs are references to the static `EXPECTED_STEP_IDS` list rather than
/// slices of the input text. Unrecognized step tokens are ignored.
fn collect_step_comment_ids(comment_bodies: &str) -> BTreeSet<&'static str> {
	comment_bodies
		.lines()
		.filter_map(detect_step_comment_id)
		.collect()
}

fn detect_step_comment_id(line: &str) -> Option<&'static str> {
	detect_any_step_comment_token(line).and_then(|candidate| {
		EXPECTED_STEP_IDS
			.iter()
			.copied()
			.find(|step| *step == candidate)
	})
}

fn detect_any_step_comment_token(line: &str) -> Option<&str> {
	let trimmed = line.trim();
	if trimmed.starts_with(ORCHESTRATOR_SIGNATURE) {
		extract_step_token_after_marker(trimmed, "Step ")
	} else if trimmed.starts_with("## Step ") {
		extract_step_token_after_marker(trimmed, "## Step ")
	} else {
		None
	}
}

fn extract_step_token_after_marker<'a>(line: &'a str, marker: &str) -> Option<&'a str> {
	// Callers only invoke this after confirming the line begins with the relevant marker.
	let start = line.find(marker)? + marker.len();
	let candidate = line[start..]
		.split(|ch: char| ch == '|' || ch.is_whitespace())
		.next()
		.unwrap_or_default()
		.trim_end_matches(':');
	if candidate.is_empty() {
		None
	} else {
		Some(candidate)
	}
}

fn verify_artifacts_for_date(repo_root: &Path, today: &str) -> StepReport {
	let mut status = StepStatus::Pass;
	let mut details = Vec::with_capacity(3);

	for result in [
		verify_journal_freshness(repo_root, today),
		verify_worklog_exists(repo_root, today),
		verify_review_artifact_exists(repo_root),
	] {
		match result {
			Ok((check_status, detail)) => {
				if check_status == StepStatus::Warn {
					status = StepStatus::Warn;
				}
				details.push(detail);
			}
			Err(error) => {
				status = StepStatus::Error;
				details.push(error);
			}
		}
	}

	StepReport {
		name: ARTIFACT_VERIFY_STEP_NAME,
		status,
		severity: Severity::Warning,
		exit_code: None,
		detail: Some(details.join("; ")),
		findings: None,
		summary: None,
	}
}

fn verify_journal_freshness(repo_root: &Path, today: &str) -> Result<(StepStatus, String), String> {
	let journal_dir = repo_root.join("docs/journal");
	if !journal_dir.is_dir() {
		return Ok((
			StepStatus::Warn,
			format!("docs/journal/ directory is missing at {}", journal_dir.display()),
		));
	}

	let Some(latest) = latest_journal_file_date(&journal_dir)? else {
		return Ok((
			StepStatus::Warn,
			format!(
				"docs/journal/ has no dated journal files in YYYY-MM-DD.md format at {}",
				journal_dir.display()
			),
		));
	};
	let latest_date = parse_iso_date(&latest)?;
	let today_date = parse_iso_date(today)?;
	let days_ago = today_date.signed_duration_since(latest_date).num_days();

	if days_ago > 1 {
		Ok((
			StepStatus::Warn,
			format!("Journal last entry is from {}, {} days ago", latest, days_ago),
		))
	} else {
		Ok((
			StepStatus::Pass,
			format!("Journal current (last entry {})", latest),
		))
	}
}

fn verify_worklog_exists(repo_root: &Path, today: &str) -> Result<(StepStatus, String), String> {
	let worklog_dir = repo_root.join("docs/worklog").join(today);
	if !worklog_dir.is_dir() {
		return Ok((
			StepStatus::Warn,
			format!("No worklog entry found for today ({})", today),
		));
	}

	let mut entries = fs::read_dir(&worklog_dir)
		.map_err(|error| format!("failed to read {}: {}", worklog_dir.display(), error))?;
	let has_file = entries.any(|entry| {
		entry
			.ok()
			.and_then(|entry| entry.file_type().ok())
			.is_some_and(|file_type| file_type.is_file())
	});

	if has_file {
		Ok((
			StepStatus::Pass,
			format!("Worklog entry found for today ({})", today),
		))
	} else {
		Ok((
			StepStatus::Warn,
			format!("No worklog entry found for today ({})", today),
		))
	}
}

fn latest_worklog_entry_for_date(repo_root: &Path, today: &str) -> Result<Option<PathBuf>, String> {
	let worklog_dir = repo_root.join("docs/worklog").join(today);
	if !worklog_dir.is_dir() {
		return Ok(None);
	}

	let entries = fs::read_dir(&worklog_dir)
		.map_err(|error| format!("failed to read {}: {}", worklog_dir.display(), error))?;
	let mut latest = None;

	for entry in entries {
		let entry = entry
			.map_err(|error| format!("failed to read {}: {}", worklog_dir.display(), error))?;
		if !entry
			.file_type()
			.map_err(|error| format!("failed to inspect {}: {}", entry.path().display(), error))?
			.is_file()
		{
			continue;
		}

		let file_name = entry.file_name();
		let Some(file_name) = file_name.to_str() else {
			continue;
		};
		if !is_worklog_entry_filename(file_name) {
			continue;
		}

		if latest
			.as_ref()
			.is_none_or(|(current_file_name, _): &(String, PathBuf)| file_name > current_file_name)
		{
			latest = Some((file_name.to_string(), entry.path()));
		}
	}

	Ok(latest.map(|(_, path)| path))
}

fn is_worklog_entry_filename(file_name: &str) -> bool {
	file_name.ends_with(".md")
		&& file_name.len() > 10
		&& file_name.as_bytes()[..6].iter().all(u8::is_ascii_digit)
		&& file_name.as_bytes()[6] == b'-'
}

fn verify_review_artifact_exists(repo_root: &Path) -> Result<(StepStatus, String), String> {
	let state = read_state_value(repo_root)?;
	let cycle = state
		.pointer(REVIEW_LAST_CYCLE_PATH)
		.and_then(Value::as_u64)
		.ok_or_else(|| format!("missing integer: {}", REVIEW_LAST_CYCLE_PATH))?;
	let review_path = repo_root.join(format!("docs/reviews/cycle-{}.md", cycle));

	if review_path.is_file() {
		Ok((
			StepStatus::Pass,
			format!("Review artifact present for cycle {}", cycle),
		))
	} else {
		Ok((
			StepStatus::Warn,
			format!("Review artifact missing for cycle {}", cycle),
		))
	}
}

fn latest_journal_file_date(journal_dir: &Path) -> Result<Option<String>, String> {
	let entries = fs::read_dir(journal_dir)
		.map_err(|error| format!("failed to read {}: {}", journal_dir.display(), error))?;
	let mut latest = None;

	for entry in entries {
		let entry = entry
			.map_err(|error| format!("failed to read {}: {}", journal_dir.display(), error))?;
		if !entry
			.file_type()
			.map_err(|error| format!("failed to inspect {}: {}", entry.path().display(), error))?
			.is_file()
		{
			continue;
		}

		let file_name = entry.file_name();
		let Some(file_name) = file_name.to_str() else {
			continue;
		};
		let Some(candidate) = file_name.strip_suffix(".md") else {
			continue;
		};
		if !is_iso_date(candidate) {
			continue;
		}
		let candidate_date = match parse_iso_date(candidate) {
			Ok(candidate_date) => candidate_date,
			Err(error) => {
				eprintln!(
					"Skipping malformed journal filename {}: {}",
					entry.path().display(),
					error
				);
				continue;
			}
		};

		if latest
			.as_ref()
			.is_none_or(|(current_date, _)| candidate_date > *current_date)
		{
			latest = Some((candidate_date, candidate.to_string()));
		}
	}

	Ok(latest.map(|(_, candidate)| candidate))
}

fn is_iso_date(value: &str) -> bool {
	value.len() == 10
		&& value
			.chars()
			.enumerate()
			.all(|(index, ch)| match index {
				4 | 7 => ch == '-',
				_ => ch.is_ascii_digit(),
			})
}

fn parse_iso_date(value: &str) -> Result<NaiveDate, String> {
	NaiveDate::parse_from_str(value, "%Y-%m-%d")
		.map_err(|error| format!("invalid date '{}': {}", value, error))
}

fn pipeline_exit_code(steps: &[StepReport]) -> i32 {
	if steps.iter().any(|step| step.status == StepStatus::Error) {
		2
	} else if steps.iter().any(|step| step.status == StepStatus::Fail) {
		1
	} else {
		0
	}
}

fn print_human_report(report: &PipelineReport) {
	println!("Pipeline Check — Cycle {}", report.cycle);
	println!();

	for (index, step) in report.steps.iter().enumerate() {
		let summary = match step.name {
			"metric-snapshot" => step.detail.as_deref().unwrap_or(""),
			"housekeeping-scan" => step.detail.as_deref().unwrap_or(""),
			"cycle-status" => step.summary.as_deref().unwrap_or(""),
			_ => step.detail.as_deref().unwrap_or(""),
		};
		if summary.is_empty() {
			println!(
				"  {}. {:<19} {:<5}",
				index + 1,
				format!("{}:", step.name),
				step_status_label(step.status)
			);
		} else {
			println!(
				"  {}. {:<19} {:<5} ({})",
				index + 1,
				format!("{}:", step.name),
				step_status_label(step.status),
				summary
			);
		}
	}

	let warning_count = report
		.steps
		.iter()
		.filter(|step| step.status == StepStatus::Warn)
		.count();

	println!();
	if warning_count == 0 {
		println!("Overall: {}", step_status_label(report.overall));
	} else {
		let suffix = if warning_count == 1 { "warning" } else { "warnings" };
		println!(
			"Overall: {} ({} {})",
			step_status_label(report.overall),
			warning_count,
			suffix
		);
	}
}

fn step_status_label(status: StepStatus) -> &'static str {
	match status {
		StepStatus::Pass => "PASS",
		StepStatus::Warn => "WARN",
		StepStatus::Fail => "FAIL",
		StepStatus::Error => "ERROR",
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;
    use std::fs;
    use std::sync::atomic::AtomicBool;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::Mutex;

    fn repo_root() -> PathBuf {
        PathBuf::from("/repo")
    }

	/// Build mock orchestrator step comment bodies for tests from a list of step IDs.
	fn step_comment_bodies(cycle: u64, step_ids: &[&str]) -> String {
		step_ids
			.iter()
			.map(|step| format!("> **[main-orchestrator]** | Cycle {cycle} | Step {step}\n"))
			.collect()
	}

	#[test]
	fn warning_steps_get_warn_status_not_fail() {
		let execution = ExecutionResult {
			exit_code: Some(1),
			stdout: "WARNING: metadata refresh pending".to_string(),
		};
		let step = classify_step("field-inventory", &ToolKind::FieldInventory, execution);
		assert_eq!(step.status, StepStatus::Warn);
		assert_ne!(step.status, StepStatus::Fail);
		assert_eq!(step.detail.as_deref(), Some("WARNING: metadata refresh pending"));
	}

	#[test]
	fn housekeeping_scan_is_warn_when_findings_are_reported() {
		let execution = ExecutionResult {
			exit_code: Some(1),
			stdout: json!({
				"items_needing_attention": 1
			})
			.to_string(),
		};
		let step = classify_step("housekeeping-scan", &ToolKind::HousekeepingScan, execution);
		assert_eq!(severity_for_kind(&ToolKind::HousekeepingScan), Severity::Warning);
		assert_eq!(step.status, StepStatus::Warn);
		assert_eq!(step.findings, Some(1));
		assert_eq!(step.detail.as_deref(), Some("1 findings"));
	}

	#[test]
	fn cycle_status_is_pass_when_command_succeeds() {
		let execution = ExecutionResult {
			exit_code: Some(0),
			stdout: json!({
				"concurrency": { "in_flight": 1 },
				"eva_input": { "comments_since_last_cycle": [{"x":1}, {"x":2}] }
			})
			.to_string(),
		};
		let step = classify_step("cycle-status", &ToolKind::CycleStatus, execution);
		assert_eq!(step.status, StepStatus::Pass);
		assert_eq!(
			step.summary.as_deref(),
			Some("1 in-flight, 2 eva directives")
		);
	}

	#[test]
	fn cycle_status_is_fail_when_commit_freeze_check_fails() {
		let execution = ExecutionResult {
			exit_code: Some(1),
			stdout: json!({
				"concurrency": { "in_flight": 0 },
				"eva_input": { "comments_since_last_cycle": [] }
			})
			.to_string(),
		};
		let step = classify_step("cycle-status", &ToolKind::CycleStatus, execution);
		assert_eq!(severity_for_kind(&ToolKind::CycleStatus), Severity::Blocking);
		assert_eq!(step.status, StepStatus::Fail);
		assert_eq!(step.exit_code, Some(1));
		assert_eq!(step.summary.as_deref(), Some("0 in-flight, 0 eva directives"));
	}

	#[test]
	fn derive_metrics_is_pass_when_tracked_fields_match() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir().join(format!("pipeline-check-derive-match-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"copilot_metrics": {
					"total_dispatches": 4,
					"resolved": 3,
					"merged": 1,
					"in_flight": 1,
					"produced_pr": 2,
					"closed_without_pr": 1,
					"reviewed_awaiting_eva": 1,
					"dispatch_to_pr_rate": "50.0%",
					"pr_merge_rate": "50.0%"
				}
			})
			.to_string(),
		)
		.unwrap();

		struct DeriveMetricsRunner;

		impl CommandRunner for DeriveMetricsRunner {
			fn run(&self, script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				assert_eq!(
					script_path.file_name().and_then(|name| name.to_str()),
					Some(DERIVE_METRICS_TOOL_NAME)
				);
				Ok(ExecutionResult {
					exit_code: Some(0),
					stdout: json!({
						"total_dispatches": 4,
						"resolved": 3,
						"merged": 1,
						"in_flight": 1,
						"produced_pr": 2,
						"closed_without_pr": 1,
						"reviewed_awaiting_eva": 1,
						"dispatch_to_pr_rate": "50.0%",
						"pr_merge_rate": "50.0%"
					})
					.to_string(),
				})
			}

			fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
				Err("issue comments are not used in derive-metrics test".to_string())
			}
		}

		let spec = ToolSpec {
			display_name: DERIVE_METRICS_TOOL_NAME,
			wrapper_relative_path: DERIVE_METRICS_WRAPPER_PATH,
			args: vec![],
			kind: ToolKind::DeriveMetrics,
		};
		let step = run_step(&root, &spec, &DeriveMetricsRunner);
		assert_eq!(step.status, StepStatus::Pass);
		assert_eq!(step.detail.as_deref(), Some("tracked copilot_metrics fields match"));
	}

	#[test]
	fn derive_metrics_is_fail_when_tracked_fields_diverge() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir().join(format!("pipeline-check-derive-fail-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"copilot_metrics": {
					"total_dispatches": 4,
					"resolved": 2,
					"merged": 1,
					"in_flight": 2,
					"produced_pr": 1,
					"closed_without_pr": 0,
					"reviewed_awaiting_eva": 0,
					"dispatch_to_pr_rate": "25.0%",
					"pr_merge_rate": "100.0%"
				}
			})
			.to_string(),
		)
		.unwrap();

		struct DeriveMetricsRunner;

		impl CommandRunner for DeriveMetricsRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				Ok(ExecutionResult {
					exit_code: Some(0),
					stdout: json!({
						"total_dispatches": 5,
						"resolved": 3,
						"merged": 1,
						"in_flight": 2,
						"produced_pr": 2,
						"closed_without_pr": 1,
						"reviewed_awaiting_eva": 0,
						"dispatch_to_pr_rate": "40.0%",
						"pr_merge_rate": "50.0%"
					})
					.to_string(),
				})
			}

			fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
				Err("issue comments are not used in derive-metrics test".to_string())
			}
		}

		let spec = ToolSpec {
			display_name: DERIVE_METRICS_TOOL_NAME,
			wrapper_relative_path: DERIVE_METRICS_WRAPPER_PATH,
			args: vec![],
			kind: ToolKind::DeriveMetrics,
		};
		let step = run_step(&root, &spec, &DeriveMetricsRunner);
		assert_eq!(severity_for_kind(&ToolKind::DeriveMetrics), Severity::Blocking);
		assert_eq!(step.status, StepStatus::Fail);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("total_dispatches"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("produced_pr"));
	}

	#[test]
	fn derive_metrics_is_fail_when_rate_fields_diverge() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir().join(format!("pipeline-check-derive-rate-fail-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"copilot_metrics": {
					"total_dispatches": 4,
					"resolved": 3,
					"merged": 1,
					"in_flight": 1,
					"produced_pr": 2,
					"closed_without_pr": 1,
					"reviewed_awaiting_eva": 1,
					"dispatch_to_pr_rate": "2/4",
					"pr_merge_rate": "50.0%"
				}
			})
			.to_string(),
		)
		.unwrap();

		struct DeriveMetricsRunner;

		impl CommandRunner for DeriveMetricsRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				Ok(ExecutionResult {
					exit_code: Some(0),
					stdout: json!({
						"total_dispatches": 4,
						"resolved": 3,
						"merged": 1,
						"in_flight": 1,
						"produced_pr": 2,
						"closed_without_pr": 1,
						"reviewed_awaiting_eva": 1,
						"dispatch_to_pr_rate": "50.0%",
						"pr_merge_rate": "50.0%"
					})
					.to_string(),
				})
			}

			fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
				Err("issue comments are not used in derive-metrics test".to_string())
			}
		}

		let spec = ToolSpec {
			display_name: DERIVE_METRICS_TOOL_NAME,
			wrapper_relative_path: DERIVE_METRICS_WRAPPER_PATH,
			args: vec![],
			kind: ToolKind::DeriveMetrics,
		};
		let step = run_step(&root, &spec, &DeriveMetricsRunner);
		assert_eq!(step.status, StepStatus::Fail);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("dispatch_to_pr_rate"));
		assert!(!step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("pr_merge_rate"));
	}

	#[test]
	fn derive_metrics_is_fail_when_pr_merge_rate_diverges() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root =
			std::env::temp_dir().join(format!("pipeline-check-derive-pr-merge-rate-fail-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"copilot_metrics": {
					"total_dispatches": 4,
					"resolved": 3,
					"merged": 1,
					"in_flight": 1,
					"produced_pr": 2,
					"closed_without_pr": 1,
					"reviewed_awaiting_eva": 1,
					"dispatch_to_pr_rate": "50.0%",
					"pr_merge_rate": "1/2"
				}
			})
			.to_string(),
		)
		.unwrap();

		struct DeriveMetricsRunner;

		impl CommandRunner for DeriveMetricsRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				Ok(ExecutionResult {
					exit_code: Some(0),
					stdout: json!({
						"total_dispatches": 4,
						"resolved": 3,
						"merged": 1,
						"in_flight": 1,
						"produced_pr": 2,
						"closed_without_pr": 1,
						"reviewed_awaiting_eva": 1,
						"dispatch_to_pr_rate": "50.0%",
						"pr_merge_rate": "50.0%"
					})
					.to_string(),
				})
			}

			fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
				Err("issue comments are not used in derive-metrics test".to_string())
			}
		}

		let spec = ToolSpec {
			display_name: DERIVE_METRICS_TOOL_NAME,
			wrapper_relative_path: DERIVE_METRICS_WRAPPER_PATH,
			args: vec![],
			kind: ToolKind::DeriveMetrics,
		};
		let step = run_step(&root, &spec, &DeriveMetricsRunner);
		assert_eq!(step.status, StepStatus::Fail);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("pr_merge_rate"));
		assert!(!step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("dispatch_to_pr_rate"));
	}

    #[test]
    fn metric_snapshot_detail_prefers_summary() {
        let execution = ExecutionResult {
            exit_code: Some(0),
            stdout: json!({"summary":"13/13 checks","checks":[{"pass":true}]}).to_string(),
        };
        let step = classify_step("metric-snapshot", &ToolKind::MetricSnapshot, execution);
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.detail.as_deref(), Some("13/13 checks"));
    }

	#[test]
	fn fail_and_error_affect_overall_and_exit_code() {
		let report = PipelineReport {
			cycle: 10,
			overall: StepStatus::Fail,
			timestamp: "2026-01-01T00:00:00Z".to_string(),
			has_blocking_findings: true,
			steps: vec![
				StepReport {
					name: "metric-snapshot",
					status: StepStatus::Pass,
					severity: Severity::Blocking,
					exit_code: Some(0),
					detail: None,
					findings: None,
					summary: None,
				},
				StepReport {
					name: "state-invariants",
					status: StepStatus::Fail,
					severity: Severity::Blocking,
					exit_code: Some(1),
					detail: None,
					findings: None,
					summary: None,
				},
            ],
        };
        assert_eq!(pipeline_exit_code(&report.steps), 1);

        let mut steps = report.steps;
        steps[1].status = StepStatus::Error;
        assert_eq!(pipeline_exit_code(&steps), 2);
    }

    #[test]
	fn fail_steps_return_failure_exit_code() {
		let steps = vec![
			StepReport {
				name: "metric-snapshot",
				status: StepStatus::Pass,
				severity: Severity::Blocking,
				exit_code: Some(0),
				detail: None,
				findings: None,
				summary: None,
			},
			StepReport {
				name: "cycle-status",
				status: StepStatus::Fail,
				severity: Severity::Blocking,
				exit_code: Some(1),
				detail: None,
				findings: None,
				summary: None,
			},
		];
		assert_eq!(pipeline_exit_code(&steps), 1);
	}

	#[test]
	fn pipeline_with_only_warn_steps_is_overall_pass_and_has_no_blocking_findings() {
		let report = PipelineReport {
			cycle: 10,
			overall: StepStatus::Pass,
			timestamp: "2026-01-01T00:00:00Z".to_string(),
			has_blocking_findings: false,
			steps: vec![
				StepReport {
					name: "field-inventory",
					status: StepStatus::Warn,
					severity: Severity::Warning,
					exit_code: Some(1),
					detail: Some("metadata refresh pending".to_string()),
					findings: None,
					summary: None,
				},
				StepReport {
					name: "housekeeping-scan",
					status: StepStatus::Warn,
					severity: Severity::Warning,
					exit_code: Some(1),
					detail: Some("1 findings".to_string()),
					findings: Some(1),
					summary: None,
				},
			],
		};

		assert_eq!(report.overall, StepStatus::Pass);
		assert!(!report.has_blocking_findings);
		assert_eq!(pipeline_exit_code(&report.steps), 0);
	}

	#[test]
	fn pipeline_with_one_fail_step_has_overall_fail_and_blocking_findings() {
		let report = PipelineReport {
			cycle: 10,
			overall: StepStatus::Fail,
			timestamp: "2026-01-01T00:00:00Z".to_string(),
			has_blocking_findings: true,
			steps: vec![StepReport {
				name: "metric-snapshot",
				status: StepStatus::Fail,
				severity: Severity::Blocking,
				exit_code: Some(1),
				detail: Some("12/13 checks".to_string()),
				findings: None,
				summary: None,
			}],
		};

		assert_eq!(report.overall, StepStatus::Fail);
		assert!(report.has_blocking_findings);
		assert_eq!(pipeline_exit_code(&report.steps), 1);
	}

	#[test]
	fn mixed_warn_and_fail_steps_still_have_blocking_findings() {
		let report = PipelineReport {
			cycle: 10,
			overall: StepStatus::Fail,
			timestamp: "2026-01-01T00:00:00Z".to_string(),
			has_blocking_findings: true,
			steps: vec![
				StepReport {
					name: "field-inventory",
					status: StepStatus::Warn,
					severity: Severity::Warning,
					exit_code: Some(1),
					detail: Some("metadata refresh pending".to_string()),
					findings: None,
					summary: None,
				},
				StepReport {
					name: "state-invariants",
					status: StepStatus::Fail,
					severity: Severity::Blocking,
					exit_code: Some(1),
					detail: Some("4/5 invariants pass".to_string()),
					findings: None,
					summary: None,
				},
			],
		};

		assert_eq!(report.overall, StepStatus::Fail);
		assert!(report.has_blocking_findings);
		assert_eq!(pipeline_exit_code(&report.steps), 1);
	}

	#[test]
	fn all_pass_steps_have_overall_pass_and_no_blocking_findings() {
		let report = PipelineReport {
			cycle: 10,
			overall: StepStatus::Pass,
			timestamp: "2026-01-01T00:00:00Z".to_string(),
			has_blocking_findings: false,
			steps: vec![
				StepReport {
					name: "metric-snapshot",
					status: StepStatus::Pass,
					severity: Severity::Blocking,
					exit_code: Some(0),
					detail: Some("13/13 checks".to_string()),
					findings: None,
					summary: None,
				},
				StepReport {
					name: "field-inventory",
					status: StepStatus::Pass,
					severity: Severity::Warning,
					exit_code: Some(0),
					detail: Some("PASS: all fields covered".to_string()),
					findings: None,
					summary: None,
				},
			],
		};

		assert_eq!(report.overall, StepStatus::Pass);
		assert!(!report.has_blocking_findings);
		assert_eq!(pipeline_exit_code(&report.steps), 0);
	}

	#[test]
	fn pipeline_exit_code_maps_pass_warn_fail_and_error() {
		let pass_steps = vec![StepReport {
			name: "metric-snapshot",
			status: StepStatus::Pass,
			severity: Severity::Blocking,
			exit_code: Some(0),
			detail: None,
			findings: None,
			summary: None,
		}];
		let warn_steps = vec![StepReport {
			name: "field-inventory",
			status: StepStatus::Warn,
			severity: Severity::Warning,
			exit_code: Some(1),
			detail: None,
			findings: None,
			summary: None,
		}];
		let fail_steps = vec![StepReport {
			name: "state-invariants",
			status: StepStatus::Fail,
			severity: Severity::Blocking,
			exit_code: Some(1),
			detail: None,
			findings: None,
			summary: None,
		}];
		let error_steps = vec![StepReport {
			name: "housekeeping-scan",
			status: StepStatus::Error,
			severity: Severity::Warning,
			exit_code: None,
			detail: None,
			findings: None,
			summary: None,
		}];

		assert_eq!(pipeline_exit_code(&pass_steps), 0);
		assert_eq!(pipeline_exit_code(&warn_steps), 0);
		assert_eq!(pipeline_exit_code(&fail_steps), 1);
		assert_eq!(pipeline_exit_code(&error_steps), 2);
	}

	#[test]
	fn json_output_includes_step_severity_and_overall_blocking_flag() {
		let report = PipelineReport {
			cycle: 10,
			overall: StepStatus::Pass,
			has_blocking_findings: false,
			timestamp: "2026-01-01T00:00:00Z".to_string(),
			steps: vec![StepReport {
				name: "field-inventory",
				status: StepStatus::Warn,
				severity: Severity::Warning,
				exit_code: Some(1),
				detail: Some("metadata refresh pending".to_string()),
				findings: None,
				summary: None,
			}],
		};

		let value = serde_json::to_value(&report).unwrap();
		assert_eq!(value.get("has_blocking_findings").and_then(Value::as_bool), Some(false));
		assert_eq!(
			value
				.get("steps")
				.and_then(Value::as_array)
				.and_then(|steps| steps.first())
				.and_then(|step| step.get("severity"))
				.and_then(Value::as_str),
			Some("warning")
		);
	}

    #[test]
    fn run_step_reports_error_when_wrapper_fails() {
        struct FailingRunner<'a> {
            called: &'a AtomicBool,
        }

        impl CommandRunner for FailingRunner<'_> {
            fn run(
                &self,
                script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                self.called.store(true, Ordering::Relaxed);
                assert_eq!(script_path, Path::new("/repo/tools/metric-snapshot"));
                Err("wrapper exited with status 101".to_string())
            }

			fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
				Err("issue comments are not used in wrapper failure test".to_string())
			}
        }

        let called = AtomicBool::new(false);
        let spec = ToolSpec {
            display_name: "metric-snapshot",
            wrapper_relative_path: "tools/metric-snapshot",
            args: vec![],
            kind: ToolKind::MetricSnapshot,
        };
        let step = run_step(&repo_root(), &spec, &FailingRunner { called: &called });
        assert!(called.load(Ordering::Relaxed));
        assert_eq!(step.status, StepStatus::Error);
        assert_eq!(
            step.detail.as_deref(),
            Some("Tool 'metric-snapshot' failed: wrapper exited with status 101")
        );
    }

    #[test]
    fn run_pipeline_aggregates_tool_results_with_mock_runner() {
        struct MockRunner {
            outputs: HashMap<String, ExecutionResult>,
            expected_cycle: u64,
        }

        impl CommandRunner for MockRunner {
            fn run(&self, script_path: &Path, args: &[String]) -> Result<ExecutionResult, String> {
                let key = script_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default()
                    .to_string();
				let has_cycle_arg = args
					.windows(2)
					.any(|window| window[0] == "--cycle" && window[1] == self.expected_cycle.to_string());
				match key.as_str() {
					"metric-snapshot" | "check-field-inventory-rs" => assert!(has_cycle_arg),
					"validate-docs" => {
						let mode = args.first().map(String::as_str).unwrap_or_default();
						assert!(matches!(mode, "worklog" | "journal"));
						if mode == "worklog" {
							assert!(has_cycle_arg);
						} else {
							assert!(!has_cycle_arg);
						}
						return Ok(ExecutionResult {
							exit_code: Some(0),
							stdout: String::new(),
						});
					}
					"housekeeping-scan" | "cycle-status" | "state-invariants" | "derive-metrics" => {
						assert!(!has_cycle_arg)
					}
					_ => panic!("unexpected tool invocation: {}", key),
				}
                self.outputs
                    .get(&key)
                    .map(|result| ExecutionResult {
                        exit_code: result.exit_code,
                        stdout: result.stdout.clone(),
                    })
                    .ok_or_else(|| format!("missing mock output for {}", key))
			}

			fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
				assert_eq!(issue, 834, "aggregation test should read last_cycle.issue from state");
				Ok(concat!(
					"> **[main-orchestrator]** | Cycle 135 | Step 0\n",
					"> **[main-orchestrator]** | Cycle 135 | Step 0.5\n",
					"> **[main-orchestrator]** | Cycle 135 | Step 0.6\n",
					"> **[main-orchestrator]** | Cycle 135 | Step 1\n",
					"> **[main-orchestrator]** | Cycle 135 | Step 1.1\n",
					"> **[main-orchestrator]** | Cycle 135 | Step 2\n",
					"> **[main-orchestrator]** | Cycle 135 | Step 3\n",
					"> **[main-orchestrator]** | Cycle 135 | Step 4\n",
					"> **[main-orchestrator]** | Cycle 135 | Step 5\n",
					"> **[main-orchestrator]** | Cycle 135 | Step 6\n"
				)
				.to_string())
			}
        }

		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("pipeline-check-test-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"last_cycle": {
					"number": 135,
					"issue": 834
				},
				"cycle_phase": {
					"phase": "close_out"
				},
				"copilot_metrics": {
					"total_dispatches": 3,
					"resolved": 2,
					"merged": 1,
					"in_flight": 1,
					"produced_pr": 2,
					"closed_without_pr": 0,
					"reviewed_awaiting_eva": 1,
					"dispatch_to_pr_rate": "66.7%",
					"pr_merge_rate": "50.0%"
				},
				"review_agent": {
					"last_review_cycle": 135
				}
			})
			.to_string(),
		)
		.unwrap();
		let today = &current_utc_timestamp()[..10];
		fs::create_dir_all(root.join("docs/journal")).unwrap();
		fs::write(root.join("docs/journal").join(format!("{}.md", today)), "# Journal\n").unwrap();
		fs::create_dir_all(root.join("docs/worklog").join(today)).unwrap();
		fs::write(root.join("docs/worklog").join(today).join("entry.md"), "worklog").unwrap();
		fs::create_dir_all(root.join("docs/reviews")).unwrap();
		fs::write(root.join("docs/reviews/cycle-135.md"), "review").unwrap();

        let runner = MockRunner {
            expected_cycle: 135,
            outputs: HashMap::from([
                (
                    "metric-snapshot".to_string(),
                    ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"summary":"13/13 checks","checks":[]}).to_string(),
                    },
                ),
                (
                    "check-field-inventory-rs".to_string(),
                    ExecutionResult {
                        exit_code: Some(0),
                        stdout: "PASS: all fields covered".to_string(),
                    },
                ),
                (
                    "housekeeping-scan".to_string(),
                    ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"items_needing_attention":0}).to_string(),
                    },
                ),
                (
                    "state-invariants".to_string(),
                    ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"passed":5,"failed":0}).to_string(),
                    },
                ),
                (
                    "cycle-status".to_string(),
                    ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({
                            "concurrency": {"in_flight": 1},
                            "eva_input": {"comments_since_last_cycle": [{"x": 1}, {"x": 2}]}
                        })
                        .to_string(),
                    },
                ),
				(
					"derive-metrics".to_string(),
					ExecutionResult {
						exit_code: Some(0),
						stdout: json!({
							"total_dispatches": 3,
							"resolved": 2,
						"merged": 1,
						"in_flight": 1,
						"produced_pr": 2,
						"closed_without_pr": 0,
						"reviewed_awaiting_eva": 1,
						"dispatch_to_pr_rate": "66.7%",
						"pr_merge_rate": "50.0%"
					})
						.to_string(),
					},
				),
            ]),
        };

        let report = run_pipeline(&root, 135, &runner);
        assert_eq!(report.overall, StepStatus::Pass);
		assert_eq!(report.steps.len(), 9);
		assert_eq!(report.steps[0].status, StepStatus::Pass);
		assert_eq!(report.steps[1].status, StepStatus::Pass);
		assert_eq!(report.steps[2].status, StepStatus::Pass);
		assert_eq!(report.steps[3].status, StepStatus::Pass);
		assert_eq!(
			report.steps[3].summary.as_deref(),
			Some("1 in-flight, 2 eva directives")
        );
        assert_eq!(report.steps[4].status, StepStatus::Pass);
        assert_eq!(
            report.steps[4].detail.as_deref(),
            Some("5/5 invariants pass")
        );
		assert_eq!(report.steps[5].status, StepStatus::Pass);
		assert_eq!(
			report.steps[5].detail.as_deref(),
			Some("tracked copilot_metrics fields match")
		);
		assert_eq!(report.steps[6].name, "artifact-verify");
		assert_eq!(report.steps[6].status, StepStatus::Pass);
		assert_eq!(report.steps[7].name, "doc-validation");
		assert_eq!(report.steps[7].status, StepStatus::Pass);
		assert_eq!(report.steps[8].name, "step-comments");
		assert_eq!(report.steps[8].status, StepStatus::Pass);
	}

    #[test]
    fn cli_accepts_missing_cycle_argument() {
        let cli = Cli::try_parse_from(["pipeline-check", "--repo-root", "."]).unwrap();
        assert_eq!(cli.repo_root, PathBuf::from("."));
        assert_eq!(cli.cycle, None);
    }

    #[test]
    fn run_pipeline_fails_when_all_steps_error() {
        struct ErrorRunner;

        impl CommandRunner for ErrorRunner {
            fn run(
                &self,
                script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                Err(format!("failed to invoke {}", script_path.display()))
            }

			fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
				Err("failed to fetch issue comments".to_string())
			}
        }

        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("pipeline-check-fail-all-errors-{}", run_id));
        fs::create_dir_all(&root).unwrap();

        let report = run_pipeline(&root, 140, &ErrorRunner);
        assert_eq!(report.overall, StepStatus::Fail);
		assert_eq!(report.steps.len(), 9);
		assert!(report
			.steps
			.iter()
			.all(|step| matches!(step.status, StepStatus::Error)));
	}

	#[test]
	fn doc_validation_passes_when_close_out_docs_are_valid() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir().join(format!("pipeline-check-doc-validation-pass-{}", run_id));
		fs::create_dir_all(root.join("docs/journal")).unwrap();
		fs::create_dir_all(root.join("docs/worklog/2026-03-12")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"last_cycle": {"number": 239},
				"cycle_phase": {"phase": "close_out"}
			})
			.to_string(),
		)
		.unwrap();
		fs::write(
			root.join("docs/worklog/2026-03-12/010203-cycle-239-summary.md"),
			"older worklog",
		)
		.unwrap();
		fs::write(
			root.join("docs/worklog/2026-03-12/020304-cycle-239-summary.md"),
			"latest worklog",
		)
		.unwrap();
		fs::write(root.join("docs/journal/2026-03-12.md"), "# Journal\n").unwrap();

		struct ValidateDocsRunner {
			calls: Mutex<Vec<Vec<String>>>,
		}

		impl CommandRunner for ValidateDocsRunner {
			fn run(&self, script_path: &Path, args: &[String]) -> Result<ExecutionResult, String> {
				assert_eq!(script_path.file_name().and_then(|name| name.to_str()), Some("validate-docs"));
				self.calls.lock().unwrap().push(args.to_vec());
				Ok(ExecutionResult {
					exit_code: Some(0),
					stdout: String::new(),
				})
			}

			fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
				panic!("issue comments are not used in doc validation test");
			}
		}

		let runner = ValidateDocsRunner {
			calls: Mutex::new(Vec::new()),
		};

		let step = verify_doc_validation_for_date(&root, "2026-03-12", StepStatus::Pass, &runner);
		assert_eq!(step.name, "doc-validation");
		assert_eq!(step.status, StepStatus::Pass);
		assert_eq!(step.severity, Severity::Blocking);

		let calls = runner.calls.lock().unwrap();
		assert_eq!(calls.len(), 2);
		assert_eq!(calls[0][0], "worklog");
		assert_eq!(calls[0][1], "--file");
		assert_eq!(calls[0][2], root.join("docs/worklog/2026-03-12/020304-cycle-239-summary.md").display().to_string());
		assert_eq!(calls[0][3], "--cycle");
		assert_eq!(calls[0][4], "239");
		assert_eq!(calls[0][5], "--pipeline-status");
		assert_eq!(calls[0][6], "PASS");
		assert_eq!(calls[0][7], "--repo-root");
		assert_eq!(calls[0][8], root.display().to_string());
		assert_eq!(calls[1][0], "journal");
		assert_eq!(calls[1][1], "--file");
		assert_eq!(calls[1][2], root.join("docs/journal/2026-03-12.md").display().to_string());
	}

	#[test]
	fn doc_validation_skips_when_not_in_close_out_phase() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir().join(format!("pipeline-check-doc-validation-skip-phase-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"last_cycle": {"number": 239},
				"cycle_phase": {"phase": "work"}
			})
			.to_string(),
		)
		.unwrap();

		struct NoRunRunner;

		impl CommandRunner for NoRunRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				panic!("validate-docs should not run outside close-out");
			}

			fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
				panic!("issue comments are not used in doc validation test");
			}
		}

		let step = verify_doc_validation_for_date(&root, "2026-03-12", StepStatus::Pass, &NoRunRunner);
		assert_eq!(step.status, StepStatus::Pass);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("skipped"));
	}

	#[test]
	fn doc_validation_fails_when_validate_docs_reports_errors() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir().join(format!("pipeline-check-doc-validation-fail-{}", run_id));
		fs::create_dir_all(root.join("docs/journal")).unwrap();
		fs::create_dir_all(root.join("docs/worklog/2026-03-12")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"last_cycle": {"number": 239},
				"cycle_phase": {"phase": "close_out"}
			})
			.to_string(),
		)
		.unwrap();
		fs::write(
			root.join("docs/worklog/2026-03-12/020304-cycle-239-summary.md"),
			"latest worklog",
		)
		.unwrap();
		fs::write(root.join("docs/journal/2026-03-12.md"), "# Journal\n").unwrap();

		struct FailingValidateDocsRunner;

		impl CommandRunner for FailingValidateDocsRunner {
			fn run(&self, _script_path: &Path, args: &[String]) -> Result<ExecutionResult, String> {
				let command = args.first().map(String::as_str).unwrap_or_default();
				if command == "worklog" {
					return Ok(ExecutionResult {
						exit_code: Some(1),
						stdout: "missing receipts".to_string(),
					});
				}

				Ok(ExecutionResult {
					exit_code: Some(0),
					stdout: String::new(),
				})
			}

			fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
				panic!("issue comments are not used in doc validation test");
			}
		}

		let step =
			verify_doc_validation_for_date(&root, "2026-03-12", StepStatus::Fail, &FailingValidateDocsRunner);
		assert_eq!(step.status, StepStatus::Fail);
		assert_eq!(step.severity, Severity::Blocking);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("worklog"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("missing receipts"));
	}

	#[test]
	fn doc_validation_handles_missing_worklog_or_journal_gracefully() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir().join(format!("pipeline-check-doc-validation-missing-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"last_cycle": {"number": 239},
				"cycle_phase": {"phase": "close_out"}
			})
			.to_string(),
		)
		.unwrap();

		struct NoRunRunner;

		impl CommandRunner for NoRunRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				panic!("validate-docs should not run when docs are missing");
			}

			fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
				panic!("issue comments are not used in doc validation test");
			}
		}

		let step = verify_doc_validation_for_date(&root, "2026-03-12", StepStatus::Pass, &NoRunRunner);
		assert_eq!(step.status, StepStatus::Pass);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("skipped"));
	}

	#[test]
	fn latest_journal_file_date_returns_most_recent_filename() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let journal_dir =
			std::env::temp_dir().join(format!("pipeline-check-journal-files-{}", run_id));
		fs::create_dir_all(&journal_dir).unwrap();
		fs::write(journal_dir.join("2026-03-05.md"), "# Journal\n").unwrap();
		fs::write(journal_dir.join("notes.md"), "# Notes\n").unwrap();
		fs::write(journal_dir.join("2026-03-08.md"), "# Journal\n").unwrap();

		assert_eq!(
			latest_journal_file_date(&journal_dir).unwrap().as_deref(),
			Some("2026-03-08")
		);
	}

	#[test]
	fn latest_journal_file_date_returns_none_for_empty_directory() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let journal_dir =
			std::env::temp_dir().join(format!("pipeline-check-empty-journal-files-{}", run_id));
		fs::create_dir_all(&journal_dir).unwrap();

		assert_eq!(latest_journal_file_date(&journal_dir).unwrap(), None);
	}

	#[test]
	fn latest_journal_file_date_returns_none_for_non_dated_filenames() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let journal_dir = std::env::temp_dir()
			.join(format!("pipeline-check-non-dated-journal-files-{}", run_id));
		fs::create_dir_all(&journal_dir).unwrap();
		fs::write(journal_dir.join("notes.md"), "# Notes\n").unwrap();
		fs::write(journal_dir.join("readme.txt"), "notes").unwrap();

		assert_eq!(latest_journal_file_date(&journal_dir).unwrap(), None);
	}

	#[test]
	fn latest_journal_file_date_skips_malformed_date_filenames() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let journal_dir = std::env::temp_dir()
			.join(format!("pipeline-check-malformed-journal-files-{}", run_id));
		fs::create_dir_all(&journal_dir).unwrap();
		fs::write(journal_dir.join("2026-13-40.md"), "# Invalid\n").unwrap();
		fs::write(journal_dir.join("2026-00-01.md"), "# Invalid\n").unwrap();
		fs::write(journal_dir.join("2026-03-08.md"), "# Valid\n").unwrap();

		assert_eq!(
			latest_journal_file_date(&journal_dir).unwrap().as_deref(),
			Some("2026-03-08")
		);
	}

	#[test]
	fn artifact_verification_warns_for_stale_or_missing_artifacts() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir().join(format!("pipeline-check-artifacts-warn-{}", run_id));
		fs::create_dir_all(root.join("docs/journal")).unwrap();
		fs::write(root.join("docs/journal/2026-03-06.md"), "# Journal\n").unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"review_agent": {
					"last_review_cycle": 208
				}
			})
			.to_string(),
		)
		.unwrap();

		let step = verify_artifacts_for_date(&root, "2026-03-09");
		assert_eq!(step.status, StepStatus::Warn);
		assert_eq!(step.name, "artifact-verify");
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("Journal last entry is from 2026-03-06, 3 days ago"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("No worklog entry found for today (2026-03-09)"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("Review artifact missing for cycle 208"));
	}

	#[test]
	fn step_comment_detection_supports_header_and_heading_formats() {
		let bodies = concat!(
			"> **[main-orchestrator]** | Cycle 212 | Step 0\n\n### Start\n\nBody\n",
			"noise\n",
			"## Step 0.5: Check workflow runs\n",
			"## Step 1.1: Extra validation\n",
			"## Step C1: Pipeline early check\n",
			"> **[main-orchestrator]** | Cycle 212 | Step 10\n\n### Finish\n",
			"> **[main-orchestrator]** | Cycle 212 | Step 10\n\n### Duplicate\n"
		);

		let detected = collect_step_comment_ids(bodies);

		assert_eq!(detected.len(), 5);
		assert!(detected.contains("0"));
		assert!(detected.contains("0.5"));
		assert!(detected.contains("1.1"));
		assert!(detected.contains("C1"));
		assert!(detected.contains("10"));
	}

	#[test]
	fn missing_expected_step_ids_returns_missing_steps_in_expected_order() {
		let found = ["0", "0.5", "1", "2", "6", "7", "9", "10"]
			.into_iter()
			.collect::<BTreeSet<_>>();

		assert_eq!(
			missing_expected_step_ids(&found),
			vec![
				"0.6", "1.1", "3", "4", "5", "8", "C1", "C2", "C3", "C4.1", "C4.5", "C5",
				"C5.1", "C5.5", "C6", "C7", "C8",
			]
		);
	}

	#[test]
	fn step_comment_verification_fails_when_fewer_than_threshold_steps_are_found_on_previous_cycle_issue(
	) {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root =
			std::env::temp_dir().join(format!("pipeline-check-step-comments-fail-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"previous_cycle_issue": 834,
				"last_cycle": {
					"issue": 999,
					"number": 257
				}
			})
			.to_string(),
		)
		.unwrap();

		struct StepCommentRunner;

		impl CommandRunner for StepCommentRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				panic!("tool wrapper execution not expected in step comment verification test");
			}

			fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
				assert_eq!(issue, 834);
				Ok(concat!(
					"> **[main-orchestrator]** | Cycle 212 | Step 0\n",
					"> **[main-orchestrator]** | Cycle 212 | Step 0.5\n",
					"> **[main-orchestrator]** | Cycle 212 | Step 0.6\n",
					"> **[main-orchestrator]** | Cycle 212 | Step 1\n",
					"> **[main-orchestrator]** | Cycle 212 | Step 2\n",
					"> **[main-orchestrator]** | Cycle 212 | Step 3\n",
					"> **[main-orchestrator]** | Cycle 212 | Step 4\n",
					"> **[main-orchestrator]** | Cycle 212 | Step 5\n",
					"> **[main-orchestrator]** | Cycle 212 | Step 6\n"
				)
				.to_string())
			}
		}

		let step = verify_step_comments(&root, &StepCommentRunner);
		assert_eq!(step.name, "step-comments");
		assert_eq!(step.status, StepStatus::Fail);
		assert_eq!(step.severity, Severity::Blocking);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("found 9 unique step comments"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("below backstop threshold 17"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("missing mandatory [7, 8, 9, C1, C2, C3, C4.1, C4.5, C5, C5.1, C5.5, C6, C7, C8]"));
	}

	#[test]
	fn step_comment_verification_fails_when_only_two_mandatory_steps_are_present() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir()
			.join(format!("pipeline-check-step-comments-unrecognized-tokens-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"previous_cycle_issue": 996,
				"last_cycle": {
					"number": 257
				},
				"cycle_phase": {
					"phase": "close_out"
				}
			})
			.to_string(),
		)
		.unwrap();

		struct StepCommentRunner;

		impl CommandRunner for StepCommentRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				panic!("tool wrapper execution not expected in step comment verification test");
			}

			fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
				assert_eq!(issue, 996);
				Ok(concat!(
					"> **[main-orchestrator]** | Cycle 221 | Step 0\n",
					"> **[main-orchestrator]** | Cycle 221 | Step 5\n",
					"> **[main-orchestrator]** | Cycle 221 | Step Opening\n",
					"> **[main-orchestrator]** | Cycle 221 | Step 10.B\n",
					"> **[main-orchestrator]** | Cycle 221 | Step 10.C\n",
					"> **[main-orchestrator]** | Cycle 221 | Step Close\n"
				)
				.to_string())
			}
		}

		let step = verify_step_comments(&root, &StepCommentRunner);
		assert_eq!(step.status, StepStatus::Fail);
		assert_eq!(step.severity, Severity::Blocking);
		assert_eq!(step.findings, Some(2));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("issue #996: found 2 unique step comments [0, 5]"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("missing mandatory [0.5, 1, 2, 3, 4, 6, 7, 8, 9, C1, C2, C3, C4.1, C4.5, C5, C5.1, C5.5, C6, C7, C8]"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("below backstop threshold 17"));
	}

	#[test]
	fn step_comment_verification_passes_when_previous_cycle_issue_is_missing() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root =
			std::env::temp_dir().join(format!("pipeline-check-step-comments-missing-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(root.join("docs/state.json"), json!({"last_cycle": {"issue": 834}}).to_string())
			.unwrap();

		struct StepCommentRunner;

		impl CommandRunner for StepCommentRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				panic!("tool wrapper execution not expected in step comment verification test");
			}

			fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
				panic!("gh api should not run when previous_cycle_issue is missing");
			}
		}

		let step = verify_step_comments(&root, &StepCommentRunner);
		assert_eq!(step.status, StepStatus::Pass);
		assert_eq!(step.severity, Severity::Blocking);
		assert_eq!(step.detail.as_deref(), Some("skipping step comment verification: /previous_cycle_issue is not set in docs/state.json yet"));
	}

	#[test]
	fn step_comment_verification_errors_when_gh_api_fails() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir()
			.join(format!("pipeline-check-step-comments-gh-failure-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"previous_cycle_issue": 836
			})
			.to_string(),
		)
		.unwrap();

		struct StepCommentRunner;

		impl CommandRunner for StepCommentRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				panic!("tool wrapper execution not expected in step comment verification test");
			}

			fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
				assert_eq!(issue, 836);
				Err("gh api failed with status 1: rate limited".to_string())
			}
		}

		let step = verify_step_comments(&root, &StepCommentRunner);
		assert_eq!(step.status, StepStatus::Error);
		assert_eq!(step.severity, Severity::Blocking);
		assert_eq!(
			step.detail.as_deref(),
			Some("gh api failed with status 1: rate limited")
		);
	}

	#[test]
	fn step_comment_verification_does_not_require_c5_1_before_cycle_256() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir()
			.join(format!("pipeline-check-step-comments-pre-c51-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"previous_cycle_issue": 839,
				"last_cycle": {
					"number": 254
				}
			})
			.to_string(),
		)
		.unwrap();

		struct StepCommentRunner;

		impl CommandRunner for StepCommentRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				panic!("tool wrapper execution not expected in step comment verification test");
			}

			fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
				assert_eq!(issue, 839);
				let steps = EXPECTED_STEP_IDS
					.iter()
					.copied()
					.filter(|step| *step != "C5.1")
					.collect::<Vec<_>>();
				Ok(step_comment_bodies(254, &steps))
			}
		}

		let step = verify_step_comments(&root, &StepCommentRunner);
		assert_eq!(step.status, StepStatus::Warn);
		assert_eq!(step.severity, Severity::Warning);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("missing mandatory [none]"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("missing optional [C5.1]"));
	}

	#[test]
	fn step_comment_verification_requires_c5_1_from_cycle_256_onward() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir()
			.join(format!("pipeline-check-step-comments-post-c51-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"previous_cycle_issue": 840,
				"last_cycle": {
					"number": 257
				}
			})
			.to_string(),
		)
		.unwrap();

		struct StepCommentRunner;

		impl CommandRunner for StepCommentRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				panic!("tool wrapper execution not expected in step comment verification test");
			}

			fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
				assert_eq!(issue, 840);
				let steps = EXPECTED_STEP_IDS
					.iter()
					.copied()
					.filter(|step| *step != "C5.1")
					.collect::<Vec<_>>();
				Ok(step_comment_bodies(257, &steps))
			}
		}

		let step = verify_step_comments(&root, &StepCommentRunner);
		assert_eq!(step.status, StepStatus::Fail);
		assert_eq!(step.severity, Severity::Blocking);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("missing mandatory [C5.1]"));
	}

	#[test]
	fn step_comment_verification_still_requires_cycle_zero_mandatory_steps() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir()
			.join(format!("pipeline-check-step-comments-always-mandatory-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"previous_cycle_issue": 841,
				"last_cycle": {
					"number": 254
				}
			})
			.to_string(),
		)
		.unwrap();

		struct StepCommentRunner;

		impl CommandRunner for StepCommentRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				panic!("tool wrapper execution not expected in step comment verification test");
			}

			fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
				assert_eq!(issue, 841);
				let steps = EXPECTED_STEP_IDS
					.iter()
					.copied()
					.filter(|step| *step != "C1")
					.collect::<Vec<_>>();
				Ok(step_comment_bodies(254, &steps))
			}
		}

		let step = verify_step_comments(&root, &StepCommentRunner);
		assert_eq!(step.status, StepStatus::Fail);
		assert_eq!(step.severity, Severity::Blocking);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("missing mandatory [C1]"));
	}

	#[test]
	fn step_comment_verification_fails_when_mandatory_step_is_missing_even_above_threshold() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root =
			std::env::temp_dir().join(format!("pipeline-check-step-comments-mandatory-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"previous_cycle_issue": 835,
				"last_cycle": {
					"number": 257
				}
			})
			.to_string(),
		)
		.unwrap();

		struct StepCommentRunner;

		impl CommandRunner for StepCommentRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				panic!("tool wrapper execution not expected in step comment verification test");
			}

			fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
				assert_eq!(issue, 835);
				let steps = EXPECTED_STEP_IDS
					.iter()
					.copied()
					.filter(|step| *step != "C1" && *step != "C8")
					.collect::<Vec<_>>();
				Ok(step_comment_bodies(212, &steps))
			}
		}

		let step = verify_step_comments(&root, &StepCommentRunner);
		assert_eq!(step.status, StepStatus::Fail);
		assert_eq!(step.severity, Severity::Blocking);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("found 23 unique step comments"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("missing mandatory [C1, C8]"));
	}

	#[test]
	fn step_comment_verification_fails_when_mandatory_closeout_step_c4_5_is_missing() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root =
			std::env::temp_dir().join(format!("pipeline-check-step-comments-c45-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"previous_cycle_issue": 837,
				"last_cycle": {
					"number": 257
				}
			})
			.to_string(),
		)
		.unwrap();

		struct StepCommentRunner;

		impl CommandRunner for StepCommentRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				panic!("tool wrapper execution not expected in step comment verification test");
			}

			fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
				assert_eq!(issue, 837);
				let steps = EXPECTED_STEP_IDS
					.iter()
					.copied()
					.filter(|step| *step != "C4.5")
					.collect::<Vec<_>>();
				Ok(step_comment_bodies(212, &steps))
			}
		}

		let step = verify_step_comments(&root, &StepCommentRunner);
		assert_eq!(step.status, StepStatus::Fail);
		assert_eq!(step.severity, Severity::Blocking);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("found 24 unique step comments"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("missing mandatory [C4.5]"));
	}

	#[test]
	fn step_comment_verification_passes_with_all_mandatory_including_closeout() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root =
			std::env::temp_dir().join(format!("pipeline-check-step-comments-pass-{}", run_id));
		fs::create_dir_all(root.join("docs")).unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"previous_cycle_issue": 838,
				"last_cycle": {
					"number": 257
				}
			})
			.to_string(),
		)
		.unwrap();

		struct StepCommentRunner;

		impl CommandRunner for StepCommentRunner {
			fn run(&self, _script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
				panic!("tool wrapper execution not expected in step comment verification test");
			}

			fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
				assert_eq!(issue, 838);
				Ok(step_comment_bodies(212, &EXPECTED_STEP_IDS))
			}
		}

		let step = verify_step_comments(&root, &StepCommentRunner);
		assert_eq!(step.status, StepStatus::Pass);
		assert_eq!(step.severity, Severity::Blocking);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("found 25 unique step comments"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("missing mandatory [none]"));
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("missing optional [none]"));
	}

	#[test]
	fn artifact_verification_passes_when_journal_exists_for_today() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir()
			.join(format!("pipeline-check-artifacts-current-journal-{}", run_id));
		fs::create_dir_all(root.join("docs/journal")).unwrap();
		fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();
		fs::create_dir_all(root.join("docs/reviews")).unwrap();
		fs::write(root.join("docs/journal/2026-03-09.md"), "# Journal\n").unwrap();
		fs::write(root.join("docs/worklog/2026-03-09/entry.md"), "worklog").unwrap();
		fs::write(root.join("docs/reviews/cycle-208.md"), "review").unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"review_agent": {
					"last_review_cycle": 208
				}
			})
			.to_string(),
		)
		.unwrap();

		let step = verify_artifacts_for_date(&root, "2026-03-09");
		assert_eq!(step.status, StepStatus::Pass);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("Journal current (last entry 2026-03-09)"));
	}

	#[test]
	fn artifact_verification_warns_when_journal_directory_is_missing() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir()
			.join(format!("pipeline-check-artifacts-missing-journal-dir-{}", run_id));
		fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();
		fs::create_dir_all(root.join("docs/reviews")).unwrap();
		fs::write(root.join("docs/worklog/2026-03-09/entry.md"), "worklog").unwrap();
		fs::write(root.join("docs/reviews/cycle-208.md"), "review").unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"review_agent": {
					"last_review_cycle": 208
				}
			})
			.to_string(),
		)
		.unwrap();

		let step = verify_artifacts_for_date(&root, "2026-03-09");
		assert_eq!(step.status, StepStatus::Warn);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("docs/journal/ directory is missing"));
	}

	#[test]
	fn artifact_verification_warns_when_journal_directory_has_no_valid_dated_files() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root =
			std::env::temp_dir().join(format!("pipeline-check-artifacts-empty-journal-{}", run_id));
		fs::create_dir_all(root.join("docs/journal")).unwrap();
		fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();
		fs::create_dir_all(root.join("docs/reviews")).unwrap();
		fs::write(root.join("docs/worklog/2026-03-09/entry.md"), "worklog").unwrap();
		fs::write(root.join("docs/reviews/cycle-208.md"), "review").unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"review_agent": {
					"last_review_cycle": 208
				}
			})
			.to_string(),
		)
		.unwrap();

		let step = verify_artifacts_for_date(&root, "2026-03-09");
		assert_eq!(step.status, StepStatus::Warn);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("docs/journal/ has no dated journal files in YYYY-MM-DD.md format"));
	}

	#[test]
	fn artifact_verification_warns_when_journal_directory_has_only_non_dated_files() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir()
			.join(format!("pipeline-check-artifacts-non-dated-journal-{}", run_id));
		fs::create_dir_all(root.join("docs/journal")).unwrap();
		fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();
		fs::create_dir_all(root.join("docs/reviews")).unwrap();
		fs::write(root.join("docs/journal/notes.md"), "# Notes\n").unwrap();
		fs::write(root.join("docs/worklog/2026-03-09/entry.md"), "worklog").unwrap();
		fs::write(root.join("docs/reviews/cycle-208.md"), "review").unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"review_agent": {
					"last_review_cycle": 208
				}
			})
			.to_string(),
		)
		.unwrap();

		let step = verify_artifacts_for_date(&root, "2026-03-09");
		assert_eq!(step.status, StepStatus::Warn);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("docs/journal/ has no dated journal files in YYYY-MM-DD.md format"));
	}

	#[test]
	fn artifact_verification_uses_newest_valid_journal_when_invalid_files_are_present() {
		static COUNTER: AtomicU64 = AtomicU64::new(0);
		let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
		let root = std::env::temp_dir()
			.join(format!("pipeline-check-artifacts-mixed-journal-files-{}", run_id));
		fs::create_dir_all(root.join("docs/journal")).unwrap();
		fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();
		fs::create_dir_all(root.join("docs/reviews")).unwrap();
		fs::write(root.join("docs/journal/2026-13-40.md"), "# Invalid\n").unwrap();
		fs::write(root.join("docs/journal/2026-00-01.md"), "# Invalid\n").unwrap();
		fs::write(root.join("docs/journal/2026-03-08.md"), "# Valid\n").unwrap();
		fs::write(root.join("docs/journal/2026-03-09.md"), "# Valid\n").unwrap();
		fs::write(root.join("docs/worklog/2026-03-09/entry.md"), "worklog").unwrap();
		fs::write(root.join("docs/reviews/cycle-208.md"), "review").unwrap();
		fs::write(
			root.join("docs/state.json"),
			json!({
				"review_agent": {
					"last_review_cycle": 208
				}
			})
			.to_string(),
		)
		.unwrap();

		let step = verify_artifacts_for_date(&root, "2026-03-09");
		assert_eq!(step.status, StepStatus::Pass);
		assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("Journal current (last entry 2026-03-09)"));
	}
}
