use chrono::NaiveDate;
use clap::{builder::PossibleValuesParser, Parser};
use regex::Regex;
use serde::Serialize;
use serde_json::Value;
use state_schema::{
    current_cycle_from_state, current_utc_timestamp, read_state_value, StateJson, StepCommentGap,
};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::LazyLock;

const HOUSEKEEPING_FINDINGS_KEY: &str = "items_needing_attention";
const CYCLE_STATUS_IN_FLIGHT_PATH: &str = "/concurrency/in_flight";
const CYCLE_STATUS_DIRECTIVES_PATH: &str = "/eva_input/comments_since_last_cycle";
const ARTIFACT_VERIFY_STEP_NAME: &str = "artifact-verify";
const DISPOSITION_MATCH_STEP_NAME: &str = "disposition-match";
const DEFERRAL_ACCUMULATION_STEP_NAME: &str = "deferral-accumulation";
const DEFERRAL_DEADLINES_STEP_NAME: &str = "deferral-deadlines";
const MASS_DEFERRAL_GATE_STEP_NAME: &str = "mass-deferral-gate";
const DISPATCH_FINDING_RECONCILIATION_STEP_NAME: &str = "dispatch-finding-reconciliation";
const DOC_VALIDATION_STEP_NAME: &str = "doc-validation";
const WORKLOG_DEDUP_STEP_NAME: &str = "worklog-dedup";
const STEP_COMMENTS_STEP_NAME: &str = "step-comments";
const CURRENT_CYCLE_STEPS_STEP_NAME: &str = "current-cycle-steps";
const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const STEP_NAMES: [&str; 15] = [
    "metric-snapshot",
    "field-inventory",
    "housekeeping-scan",
    "cycle-status",
    "state-invariants",
    ARTIFACT_VERIFY_STEP_NAME,
    DISPOSITION_MATCH_STEP_NAME,
    DEFERRAL_ACCUMULATION_STEP_NAME,
    DEFERRAL_DEADLINES_STEP_NAME,
    MASS_DEFERRAL_GATE_STEP_NAME,
    DISPATCH_FINDING_RECONCILIATION_STEP_NAME,
    DOC_VALIDATION_STEP_NAME,
    WORKLOG_DEDUP_STEP_NAME,
    STEP_COMMENTS_STEP_NAME,
    CURRENT_CYCLE_STEPS_STEP_NAME,
];
// Steps that have not been posted yet when pipeline-check runs at C5.5.
// These are excluded from the current-cycle mandatory step check.
const POST_GATE_STEP_IDS: &[&str] = &["C5.5", "C5.6", "C6", "C6.5", "C7", "C8"];
const STEP_COMMENT_THRESHOLD: usize = 17;
const ORCHESTRATOR_SIGNATURE: &str = "> **[main-orchestrator]**";
const MANDATORY_STEPS: &[(&str, u64)] = &[
    ("0", 0),
    ("0.5", 0),
    ("0.6", 0),
    ("1", 0),
    ("1.1", 0),
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
// conditional, but mandatory gaps must still fail while optional gaps warn.
const EXPECTED_STEP_IDS: [&str; 27] = [
    "0", "0.1", "0.5", "0.6", "1", "1.1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "C1", "C2",
    "C3", "C4.1", "C4.5", "C5", "C5.1", "C5.5", "C5.6", "C6", "C7", "C8",
];
const MIN_CURRENT_CYCLE_FOR_FALLBACK_WARNING: u64 = 1;
const LAST_CYCLE_NUMBER_PATH: &str = "/last_cycle/number";
const REVIEW_LAST_CYCLE_PATH: &str = "/review_agent/last_review_cycle";
const BLOCKERS_PATH: &str = "/blockers";
const DEFERRAL_ACCUMULATION_THRESHOLD: usize = 3;
static REVIEW_FINDING_HEADER_REGEX: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?m)^## \d+\.").expect("review finding regex should compile"));

#[derive(Parser)]
#[command(name = "pipeline-check")]
struct Cli {
    #[arg(long)]
    repo_root: PathBuf,

    #[arg(long)]
    cycle: Option<u64>,

    #[arg(long)]
    json: bool,

    #[arg(
        long = "exclude-step",
        help = "Step name(s) to exclude from the pipeline run",
        value_parser = PossibleValuesParser::new(STEP_NAMES)
    )]
    exclude_steps: Vec<String>,
}

#[derive(Clone, Copy, Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
enum StepStatus {
    Pass,
    Warn,
    Cascade,
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
    let report =
        run_pipeline_with_excluded_steps(&cli.repo_root, cycle, &cli.exclude_steps, &runner);
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

#[cfg(test)]
fn run_pipeline(repo_root: &Path, cycle: u64, runner: &dyn CommandRunner) -> PipelineReport {
    run_pipeline_with_excluded_steps(repo_root, cycle, &[], runner)
}

fn run_pipeline_with_excluded_steps(
    repo_root: &Path,
    cycle: u64,
    exclude_steps: &[String],
    runner: &dyn CommandRunner,
) -> PipelineReport {
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
    ];

    let mut steps = Vec::new();
    steps.extend(
        specs
            .iter()
            .filter(|spec| !is_excluded_step(spec.display_name, exclude_steps))
            .map(|spec| run_step(repo_root, spec, runner)),
    );
    if !is_excluded_step(ARTIFACT_VERIFY_STEP_NAME, exclude_steps) {
        steps.push(verify_artifacts(repo_root));
    }
    if !is_excluded_step(DISPOSITION_MATCH_STEP_NAME, exclude_steps) {
        steps.push(verify_disposition_match(repo_root));
    }
    if !is_excluded_step(DEFERRAL_ACCUMULATION_STEP_NAME, exclude_steps) {
        steps.push(verify_deferral_accumulation(repo_root));
    }
    if !is_excluded_step(DEFERRAL_DEADLINES_STEP_NAME, exclude_steps) {
        steps.push(verify_deferral_deadlines(repo_root));
    }
    if !is_excluded_step(MASS_DEFERRAL_GATE_STEP_NAME, exclude_steps) {
        steps.push(verify_mass_deferral_gate(repo_root));
    }
    if !is_excluded_step(DISPATCH_FINDING_RECONCILIATION_STEP_NAME, exclude_steps) {
        steps.push(verify_dispatch_finding_reconciliation(repo_root));
    }
    let pipeline_status = pipeline_overall_status(&steps);
    if !is_excluded_step(DOC_VALIDATION_STEP_NAME, exclude_steps) {
        steps.push(verify_doc_validation(repo_root, pipeline_status, runner));
    }
    if !is_excluded_step(WORKLOG_DEDUP_STEP_NAME, exclude_steps) {
        steps.push(verify_worklog_dedup(repo_root));
    }
    if !is_excluded_step(STEP_COMMENTS_STEP_NAME, exclude_steps) {
        steps.push(verify_step_comments(repo_root, cycle, runner));
    }
    if !is_excluded_step(CURRENT_CYCLE_STEPS_STEP_NAME, exclude_steps) {
        steps.push(verify_current_cycle_step_comments(repo_root, cycle, runner));
    }
    // Doc validation runs before step-comments so it can pass the pre-step-comments
    // pipeline status through to validate-docs. Reclassify afterward, once the real
    // step-comments result is known, but before computing the final overall status.
    reclassify_doc_validation_cascade(&mut steps);
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

fn is_excluded_step(name: &str, exclude_steps: &[String]) -> bool {
    exclude_steps.iter().any(|excluded| excluded == name)
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

    classify_step(spec.display_name, &spec.kind, execution)
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
                                let passing = checks
                                    .iter()
                                    .filter(|check| is_check_passing(check))
                                    .count();
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
                let warned = parsed
                    .get("warned")
                    .and_then(Value::as_u64)
                    .or_else(|| {
                        parsed
                            .get("checks")
                            .and_then(Value::as_array)
                            .map(|checks| {
                                checks
                                    .iter()
                                    .filter(|check| {
                                        check
                                            .get("status")
                                            .and_then(Value::as_str)
                                            .map(|status| status == "warn")
                                            .unwrap_or(false)
                                    })
                                    .count() as u64
                            })
                    })
                    .unwrap_or(0);
                let total = passed + failed + warned;
                let warning_suffix = if warned == 0 {
                    String::new()
                } else if warned == 1 {
                    ", 1 warn".to_string()
                } else {
                    format!(", {} warns", warned)
                };
                step.detail = Some(format!(
                    "{}/{} invariants pass{}",
                    passed, total, warning_suffix
                ));
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
    }

    step
}

fn severity_for_kind(kind: &ToolKind) -> Severity {
    match kind {
        ToolKind::MetricSnapshot | ToolKind::StateInvariants | ToolKind::CycleStatus => {
            Severity::Blocking
        }
        ToolKind::FieldInventory | ToolKind::HousekeepingScan => Severity::Warning,
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

fn reclassify_doc_validation_cascade(steps: &mut [StepReport]) {
    let Some(doc_validation_index) = steps
        .iter()
        .position(|step| step.name == DOC_VALIDATION_STEP_NAME)
    else {
        return;
    };
    let Some(step_comments_index) = steps
        .iter()
        .position(|step| step.name == STEP_COMMENTS_STEP_NAME)
    else {
        return;
    };
    let step_comments_status = steps[step_comments_index].status;
    if !matches!(
        step_comments_status,
        StepStatus::Fail | StepStatus::Error | StepStatus::Warn
    ) {
        return;
    }

    let doc_validation = &mut steps[doc_validation_index];
    if doc_validation.status != StepStatus::Fail {
        return;
    }
    let Some(detail) = doc_validation.detail.as_deref() else {
        return;
    };
    if !is_step_comments_pipeline_cascade(detail) {
        return;
    }

    doc_validation.status = StepStatus::Cascade;
}

fn is_step_comments_pipeline_cascade(detail: &str) -> bool {
    // Doc validation joins multiple failures with "; ". Treat doc validation as a
    // cascade only when every sub-failure is either the step-comments-induced
    // pipeline-status mismatch or a known environment/infrastructure error that can
    // accompany it, such as shallow-clone history gaps while resolving receipts.
    detail.split("; ").all(|part| {
        is_pipeline_status_mismatch_failure(part)
            || is_known_doc_validation_environment_failure(part)
    })
}

fn is_pipeline_status_mismatch_failure(detail_part: &str) -> bool {
    strip_doc_validation_failure_prefix(detail_part).starts_with("pipeline status mismatch:")
}

fn is_known_doc_validation_environment_failure(detail_part: &str) -> bool {
    strip_doc_validation_failure_prefix(detail_part).contains("shallow clone")
}

fn strip_doc_validation_failure_prefix(detail_part: &str) -> &str {
    detail_part
        .strip_prefix("worklog validation failed: ")
        .or_else(|| detail_part.strip_prefix("journal validation failed: "))
        .unwrap_or(detail_part)
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

fn is_check_passing(check: &Value) -> bool {
    check.get("pass").and_then(Value::as_bool).unwrap_or(false)
}

fn verify_artifacts(repo_root: &Path) -> StepReport {
    verify_artifacts_for_date(repo_root, &current_utc_timestamp()[..10])
}

fn verify_doc_validation(
    repo_root: &Path,
    pipeline_status: StepStatus,
    runner: &dyn CommandRunner,
) -> StepReport {
    verify_doc_validation_for_date(
        repo_root,
        &current_utc_timestamp()[..10],
        pipeline_status,
        runner,
    )
}

fn verify_worklog_dedup(repo_root: &Path) -> StepReport {
    verify_worklog_dedup_for_date(repo_root, &current_utc_timestamp()[..10])
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
                    detail: Some(format!(
                        "Tool 'validate-docs' failed while validating {}: {}",
                        label, error
                    )),
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
                    detail: Some(format!(
                        "{} validation exited with unexpected status {:?}",
                        label, other
                    )),
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

fn verify_step_comments(repo_root: &Path, cycle: u64, runner: &dyn CommandRunner) -> StepReport {
    let previous_cycle = match cycle.checked_sub(1) {
        Some(previous_cycle) => previous_cycle,
        None => {
            return StepReport {
                name: STEP_COMMENTS_STEP_NAME,
                status: StepStatus::Pass,
                severity: Severity::Blocking,
                exit_code: None,
                detail: Some(
                    "skipping step comment verification: cycle 0 has no previous cycle".to_string(),
                ),
                findings: None,
                summary: None,
            };
        }
    };
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

    let issue = match state
        .pointer("/previous_cycle_issue")
        .and_then(Value::as_u64)
    {
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
    let found = match fetch_step_comments_for_issue(runner, issue, previous_cycle) {
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
    let acknowledged = match acknowledged_step_comment_ids(&state, previous_cycle) {
        Ok(acknowledged) => acknowledged,
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
    let effective_acknowledged = acknowledged
        .difference(&found)
        .copied()
        .collect::<BTreeSet<_>>();
    let mut merged_found = found.clone();
    merged_found.extend(effective_acknowledged.iter().copied());
    let issue_assessment = assess_step_comment_completeness_with_acknowledged(
        &merged_found,
        previous_cycle,
        StepCommentCheckScope::PreviousCycle,
        &found,
        &effective_acknowledged,
    );

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

/// Check the current cycle's issue for pre-gate mandatory steps.
/// Steps posted after C5.5 (C5.5, C5.6, C6, C7, C8) are excluded because
/// they haven't been posted yet when pipeline-check runs.
fn verify_current_cycle_step_comments(
    repo_root: &Path,
    cycle: u64,
    runner: &dyn CommandRunner,
) -> StepReport {
    let state = match read_state_value(repo_root) {
        Ok(state) => state,
        Err(error) => {
            return StepReport {
                name: CURRENT_CYCLE_STEPS_STEP_NAME,
                status: StepStatus::Error,
                severity: Severity::Blocking,
                exit_code: None,
                detail: Some(error),
                findings: None,
                summary: None,
            };
        }
    };

    let issue = match state.pointer("/last_cycle/issue").and_then(Value::as_u64) {
        Some(issue) => issue,
        None => {
            return StepReport {
				name: CURRENT_CYCLE_STEPS_STEP_NAME,
				status: StepStatus::Pass,
				severity: Severity::Blocking,
				exit_code: None,
				detail: Some(
					"skipping current-cycle step verification: /last_cycle/issue is not set in docs/state.json"
						.to_string(),
				),
				findings: None,
				summary: None,
			};
        }
    };

    let current_cycle_issues = match discover_current_cycle_issues(&state, issue) {
        Ok(issues) => issues,
        Err(error) => {
            return StepReport {
                name: CURRENT_CYCLE_STEPS_STEP_NAME,
                status: StepStatus::Error,
                severity: Severity::Blocking,
                exit_code: None,
                detail: Some(error),
                findings: None,
                summary: None,
            };
        }
    };
    let mut found = BTreeSet::new();
    for issue in &current_cycle_issues.issues {
        let issue_found = match fetch_step_comments_for_issue(runner, *issue, cycle) {
            Ok(found) => found,
            Err(error) => {
                return StepReport {
                    name: CURRENT_CYCLE_STEPS_STEP_NAME,
                    status: StepStatus::Error,
                    severity: Severity::Blocking,
                    exit_code: None,
                    detail: Some(error),
                    findings: None,
                    summary: None,
                };
            }
        };
        found.extend(issue_found);
    }
    let issue_detail = current_cycle_issues.detail;

    // Check only pre-gate mandatory steps (exclude post-gate steps that haven't been posted yet)
    let pre_gate_mandatory_missing: Vec<&str> = MANDATORY_STEPS
        .iter()
        .copied()
        .filter(|(step, effective_from_cycle)| {
            *effective_from_cycle <= cycle
                && !POST_GATE_STEP_IDS.contains(step)
                && !found.contains(step)
        })
        .map(|(step, _)| step)
        .collect();

    let found_ids = ordered_found_step_ids(&found);

    if pre_gate_mandatory_missing.is_empty() {
        StepReport {
            name: CURRENT_CYCLE_STEPS_STEP_NAME,
            status: StepStatus::Pass,
            severity: Severity::Blocking,
            exit_code: None,
            detail: Some(format!(
                "{}: {} pre-gate mandatory steps present [{}]",
                issue_detail,
                found_ids.len(),
                format_step_id_list(&found_ids)
            )),
            findings: Some(found_ids.len()),
            summary: None,
        }
    } else {
        StepReport {
            name: CURRENT_CYCLE_STEPS_STEP_NAME,
            status: StepStatus::Fail,
            severity: Severity::Blocking,
            exit_code: None,
            detail: Some(format!(
                "{}: missing pre-gate mandatory steps [{}]; found [{}]",
                issue_detail,
                format_step_id_list(&pre_gate_mandatory_missing),
                format_step_id_list(&found_ids)
            )),
            findings: Some(found_ids.len()),
            summary: None,
        }
    }
}

struct CurrentCycleIssues {
    issues: Vec<u64>,
    detail: String,
}

fn discover_current_cycle_issues(state: &Value, issue: u64) -> Result<CurrentCycleIssues, String> {
    if let Some(cycle_issues_value) = state.pointer("/cycle_issues") {
        if !cycle_issues_value.is_null() {
            let cycle_issues: Vec<u64> = serde_json::from_value(cycle_issues_value.clone())
                .map_err(|error| {
                    format!(
                        "failed to parse /cycle_issues in docs/state.json: {}",
                        error
                    )
                })?;
            let issues = dedup_issue_numbers(cycle_issues);
            if issues.is_empty() {
                return Err("/cycle_issues in docs/state.json must not be empty".to_string());
            }

            let detail = if issues.len() == 1 {
                format!("issue #{}", issues[0])
            } else {
                format!("issues {}", format_repo_issue_list(&issues))
            };
            return Ok(CurrentCycleIssues { issues, detail });
        }
    }

    let previous_cycle_issue = state
        .pointer("/previous_cycle_issue")
        .and_then(Value::as_u64)
        .filter(|previous_issue| *previous_issue != issue);
    let detail = if let Some(previous_issue) = previous_cycle_issue {
        format!(
            "issue {}#{} + {}#{}",
            MAIN_REPO, issue, MAIN_REPO, previous_issue
        )
    } else {
        format!("issue #{}", issue)
    };
    let issues = if let Some(previous_issue) = previous_cycle_issue {
        vec![issue, previous_issue]
    } else {
        vec![issue]
    };
    Ok(CurrentCycleIssues { issues, detail })
}

fn dedup_issue_numbers(issues: Vec<u64>) -> Vec<u64> {
    let mut deduped = Vec::new();
    for issue in issues {
        if !deduped.contains(&issue) {
            deduped.push(issue);
        }
    }
    deduped
}

fn format_repo_issue_list(issues: &[u64]) -> String {
    issues
        .iter()
        .map(|issue| format!("{}#{}", MAIN_REPO, issue))
        .collect::<Vec<_>>()
        .join(" + ")
}

fn fetch_step_comments_for_issue(
    runner: &dyn CommandRunner,
    issue: u64,
    cycle: u64,
) -> Result<BTreeSet<&'static str>, String> {
    runner
        .fetch_issue_comment_bodies(issue)
        .map(|comment_bodies| collect_step_comment_ids(&comment_bodies, cycle))
}

fn acknowledged_step_comment_ids(
    state: &Value,
    cycle: u64,
) -> Result<BTreeSet<&'static str>, String> {
    let Some(gaps_value) = state.pointer("/step_comment_acknowledged_gaps") else {
        return Ok(BTreeSet::new());
    };
    if gaps_value.is_null() {
        return Ok(BTreeSet::new());
    }
    let gaps: Vec<StepCommentGap> =
        serde_json::from_value(gaps_value.clone()).map_err(|error| {
            format!(
                "failed to parse /step_comment_acknowledged_gaps in docs/state.json: {}",
                error
            )
        })?;
    let mut acknowledged = BTreeSet::new();
    for gap in gaps.into_iter().filter(|gap| gap.cycle == cycle) {
        for missing_step in gap.missing_steps {
            let step_id = EXPECTED_STEP_IDS
                .iter()
                .copied()
                .find(|candidate| *candidate == missing_step)
                .ok_or_else(|| {
                    format!(
						"invalid /step_comment_acknowledged_gaps entry for cycle {} issue {}: unknown step id {}",
						gap.cycle, gap.issue, missing_step
					)
                })?;
            acknowledged.insert(step_id);
        }
    }
    Ok(acknowledged)
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
        .copied()
        .any(|(mandatory_step, effective_from_cycle)| {
            mandatory_step == step && effective_from_cycle <= cycle
        })
}

#[cfg(test)]
fn assess_step_comment_completeness(
    found: &BTreeSet<&'static str>,
    cycle: u64,
    scope: StepCommentCheckScope,
) -> StepCommentAssessment {
    assess_step_comment_completeness_with_acknowledged(found, cycle, scope, found, &BTreeSet::new())
}

fn assess_step_comment_completeness_with_acknowledged(
    found: &BTreeSet<&'static str>,
    cycle: u64,
    scope: StepCommentCheckScope,
    actual_found: &BTreeSet<&'static str>,
    acknowledged: &BTreeSet<&'static str>,
) -> StepCommentAssessment {
    let found_ids = ordered_found_step_ids(found);
    let missing = missing_expected_step_ids(found);
    let (mandatory_missing, optional_missing): (Vec<_>, Vec<_>) = missing
        .into_iter()
        .partition(|step| is_mandatory_step_for_cycle(step, cycle));
    let detail = format_step_comment_detail(
        found,
        &found_ids,
        &mandatory_missing,
        &optional_missing,
        actual_found,
        acknowledged,
    );

    if found.len() < STEP_COMMENT_THRESHOLD {
        // The backstop only blocks for the current cycle. For the previous cycle,
        // cycle-aware filtering may legitimately reduce the per-cycle count below
        // the threshold (e.g., resumed cycles where steps span multiple cycle numbers).
        // Individual missing mandatory steps are still caught by the cascade logic below.
        let backstop_severity = match scope {
            StepCommentCheckScope::CurrentCycle => Severity::Blocking,
            StepCommentCheckScope::PreviousCycle => Severity::Warning,
        };
        let backstop_status = match scope {
            StepCommentCheckScope::CurrentCycle => StepStatus::Fail,
            StepCommentCheckScope::PreviousCycle => StepStatus::Warn,
        };
        return StepCommentAssessment {
            status: backstop_status,
            severity: backstop_severity,
            detail: format!(
                "{}; below backstop threshold {}",
                detail, STEP_COMMENT_THRESHOLD
            ),
            findings: found.len(),
        };
    }

    if !mandatory_missing.is_empty() {
        // Mandatory step failures are always blocking, regardless of scope.
        // A missing mandatory step in the previous cycle means that cycle was
        // not compliant — the current cycle's pipeline must reflect that.
        // (audit #281, #284 — cascade logic must not downgrade mandatory failures)
        return StepCommentAssessment {
            status: StepStatus::Fail,
            severity: Severity::Blocking,
            detail: if scope == StepCommentCheckScope::PreviousCycle {
                format!(
                    "{}; {}",
                    format_step_comment_cascade_message(cycle, &mandatory_missing),
                    detail
                )
            } else {
                detail
            },
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

fn format_step_comment_detail(
    found: &BTreeSet<&'static str>,
    found_ids: &[&'static str],
    mandatory_missing: &[&'static str],
    optional_missing: &[&'static str],
    actual_found: &BTreeSet<&'static str>,
    acknowledged: &BTreeSet<&'static str>,
) -> String {
    if acknowledged.is_empty() {
        return format!(
            "found {} unique step comments [{}]; missing mandatory [{}]; missing optional [{}]",
            found.len(),
            format_step_id_list(found_ids),
            format_step_id_list(mandatory_missing),
            format_step_id_list(optional_missing)
        );
    }

    let actual_found_ids = ordered_found_step_ids(actual_found);
    let acknowledged_ids = ordered_found_step_ids(acknowledged);
    format!(
		"found {} unique step comments ({} acknowledged) [{}]; actually found [{}]; {} step(s) acknowledged via gap record [{}]; missing mandatory [{}]; missing optional [{}]",
		found.len(),
		acknowledged_ids.len(),
		format_step_id_list(found_ids),
		format_step_id_list(&actual_found_ids),
		acknowledged_ids.len(),
		format_step_id_list(&acknowledged_ids),
		format_step_id_list(mandatory_missing),
		format_step_id_list(optional_missing)
	)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum StepCommentCheckScope {
    #[cfg_attr(not(test), allow(dead_code))]
    CurrentCycle,
    PreviousCycle,
}

fn format_step_comment_cascade_message(cycle: u64, missing_steps: &[&str]) -> String {
    let label = if missing_steps.len() == 1 {
        "step"
    } else {
        "steps"
    };
    let verb = if missing_steps.len() == 1 {
        "was"
    } else {
        "were"
    };
    format!(
        "Cascade from cycle {}: {} {} {} missing (already penalized)",
        cycle,
        label,
        format_step_id_list(missing_steps),
        verb
    )
}

/// Completeness assessment for a collected set of step comments.
///
/// PASS means all expected steps were found, WARN means only optional steps are
/// missing, and FAIL means either a current-cycle mandatory step is missing or
/// the threshold backstop was not met. Previous-cycle mandatory misses are
/// downgraded to WARN so the next cycle records the cascade without double
/// penalizing the original omission.
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
fn collect_step_comment_ids(comment_bodies: &str, cycle: u64) -> BTreeSet<&'static str> {
    comment_bodies
        .lines()
        .filter_map(|line| detect_step_comment_id(line, cycle))
        .collect()
}

fn detect_step_comment_id(line: &str, cycle: u64) -> Option<&'static str> {
    detect_any_step_comment_token(line, cycle).and_then(|candidate| {
        EXPECTED_STEP_IDS
            .iter()
            .copied()
            .find(|step| *step == candidate)
    })
}

fn detect_any_step_comment_token(line: &str, cycle: u64) -> Option<&str> {
    let trimmed = line.trim();
    if trimmed.starts_with(ORCHESTRATOR_SIGNATURE) {
        if !orchestrator_step_comment_matches_cycle(trimmed, cycle) {
            return None;
        }
        extract_step_token_after_marker(trimmed, "Step ")
    } else if trimmed.starts_with("## Step ") {
        extract_step_token_after_marker(trimmed, "## Step ")
    } else {
        None
    }
}

fn orchestrator_step_comment_matches_cycle(line: &str, expected_cycle: u64) -> bool {
    match extract_cycle_marker(line) {
        Some(found_cycle) => found_cycle == expected_cycle,
        None => !line.contains("Cycle "),
    }
}

fn extract_cycle_marker(line: &str) -> Option<u64> {
    let signature_index = line.find(ORCHESTRATOR_SIGNATURE)?;
    let cycle_fragment = line
        .get(signature_index + ORCHESTRATOR_SIGNATURE.len()..)?
        .split_once("Cycle ")?
        .1;
    let digits: String = cycle_fragment
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        None
    } else {
        digits.parse().ok()
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
            format!(
                "docs/journal/ directory is missing at {}",
                journal_dir.display()
            ),
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
            format!(
                "Journal last entry is from {}, {} days ago",
                latest, days_ago
            ),
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

fn verify_worklog_dedup_for_date(repo_root: &Path, today: &str) -> StepReport {
    match worklog_dedup_status_for_date(repo_root, today) {
        Ok((status, detail)) => StepReport {
            name: WORKLOG_DEDUP_STEP_NAME,
            status,
            severity: Severity::Blocking,
            exit_code: None,
            detail: Some(detail),
            findings: None,
            summary: None,
        },
        Err(error) => StepReport {
            name: WORKLOG_DEDUP_STEP_NAME,
            status: StepStatus::Error,
            severity: Severity::Blocking,
            exit_code: None,
            detail: Some(error),
            findings: None,
            summary: None,
        },
    }
}

fn worklog_dedup_status_for_date(repo_root: &Path, today: &str) -> Result<(StepStatus, String), String> {
    let worklog_dir = repo_root.join("docs/worklog").join(today);
    if !worklog_dir.is_dir() {
        return Ok((StepStatus::Pass, "No duplicate worklog files found".to_string()));
    }

    let entries = fs::read_dir(&worklog_dir)
        .map_err(|error| format!("failed to read {}: {}", worklog_dir.display(), error))?;
    let mut cycles_to_files: BTreeMap<u64, Vec<String>> = BTreeMap::new();
    let mut legacy_files = Vec::new();

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
        if !file_name.ends_with(".md") {
            continue;
        }

        match extract_worklog_cycle_from_filename(file_name) {
            Some(cycle) => cycles_to_files
                .entry(cycle)
                .or_default()
                .push(file_name.to_string()),
            None => legacy_files.push(file_name.to_string()),
        }
    }

    let duplicate_details = cycles_to_files
        .into_iter()
        .filter_map(|(cycle, mut files)| {
            if files.len() < 2 {
                return None;
            }
            files.sort_unstable();
            Some(format!(
                "Duplicate worklog files for cycle {}: {}",
                cycle,
                files.join(", ")
            ))
        })
        .collect::<Vec<_>>();

    legacy_files.sort_unstable();
    let legacy_detail = if legacy_files.is_empty() {
        None
    } else {
        Some(format!(
            "worklog files missing cycle-NNN pattern: {}",
            legacy_files.join(", ")
        ))
    };

    if !duplicate_details.is_empty() {
        let mut detail = duplicate_details.join("; ");
        if let Some(legacy_detail) = legacy_detail {
            detail.push_str("; ");
            detail.push_str(&legacy_detail);
        }
        return Ok((StepStatus::Fail, detail));
    }

    if let Some(legacy_detail) = legacy_detail {
        return Ok((
            StepStatus::Warn,
            format!("No duplicate worklog files found; {}", legacy_detail),
        ));
    }

    Ok((StepStatus::Pass, "No duplicate worklog files found".to_string()))
}

fn is_worklog_entry_filename(file_name: &str) -> bool {
    file_name.ends_with(".md")
        && file_name.len() > 10
        && file_name.as_bytes()[..6].iter().all(u8::is_ascii_digit)
        && file_name.as_bytes()[6] == b'-'
}

fn extract_worklog_cycle_from_filename(file_name: &str) -> Option<u64> {
    static WORKLOG_CYCLE_FILENAME_REGEX: LazyLock<Regex> = LazyLock::new(|| {
        Regex::new(r"(?:^|-)cycle-(\d+)(?:-|\.md$)")
            .expect("worklog cycle filename regex should compile")
    });

    WORKLOG_CYCLE_FILENAME_REGEX
        .captures(file_name)
        .and_then(|captures| captures.get(1))
        .and_then(|capture| capture.as_str().parse::<u64>().ok())
}

fn verify_disposition_match(repo_root: &Path) -> StepReport {
    match disposition_match_status(repo_root) {
        Ok((status, detail)) => StepReport {
            name: DISPOSITION_MATCH_STEP_NAME,
            status,
            severity: Severity::Warning,
            exit_code: None,
            detail: Some(detail),
            findings: None,
            summary: None,
        },
        Err(error) => StepReport {
            name: DISPOSITION_MATCH_STEP_NAME,
            status: StepStatus::Error,
            severity: Severity::Warning,
            exit_code: None,
            detail: Some(error),
            findings: None,
            summary: None,
        },
    }
}

fn verify_deferral_accumulation(repo_root: &Path) -> StepReport {
    match deferral_accumulation_assessment(repo_root) {
        Ok(assessment) => StepReport {
            name: DEFERRAL_ACCUMULATION_STEP_NAME,
            status: assessment.status,
            severity: assessment.severity,
            exit_code: None,
            detail: Some(assessment.detail),
            findings: None,
            summary: None,
        },
        Err(error) => StepReport {
            name: DEFERRAL_ACCUMULATION_STEP_NAME,
            status: StepStatus::Error,
            severity: Severity::Blocking,
            exit_code: None,
            detail: Some(error),
            findings: None,
            summary: None,
        },
    }
}

fn verify_deferral_deadlines(repo_root: &Path) -> StepReport {
    match deferral_deadlines_status(repo_root) {
        Ok((status, detail)) => StepReport {
            name: DEFERRAL_DEADLINES_STEP_NAME,
            status,
            severity: Severity::Blocking,
            exit_code: None,
            detail: Some(detail),
            findings: None,
            summary: None,
        },
        Err(error) => StepReport {
            name: DEFERRAL_DEADLINES_STEP_NAME,
            status: StepStatus::Error,
            severity: Severity::Blocking,
            exit_code: None,
            detail: Some(error),
            findings: None,
            summary: None,
        },
    }
}

fn verify_mass_deferral_gate(repo_root: &Path) -> StepReport {
    match mass_deferral_gate_assessment(repo_root) {
        Ok(assessment) => StepReport {
            name: MASS_DEFERRAL_GATE_STEP_NAME,
            status: assessment.status,
            severity: assessment.severity,
            exit_code: None,
            detail: Some(assessment.detail),
            findings: None,
            summary: None,
        },
        Err(error) => StepReport {
            name: MASS_DEFERRAL_GATE_STEP_NAME,
            status: StepStatus::Error,
            severity: Severity::Blocking,
            exit_code: None,
            detail: Some(error),
            findings: None,
            summary: None,
        },
    }
}

fn verify_dispatch_finding_reconciliation(repo_root: &Path) -> StepReport {
    match dispatch_finding_reconciliation_status(repo_root) {
        Ok((status, detail)) => StepReport {
            name: DISPATCH_FINDING_RECONCILIATION_STEP_NAME,
            status,
            severity: Severity::Warning,
            exit_code: None,
            detail: Some(detail),
            findings: None,
            summary: None,
        },
        Err(error) => StepReport {
            name: DISPATCH_FINDING_RECONCILIATION_STEP_NAME,
            status: StepStatus::Error,
            severity: Severity::Warning,
            exit_code: None,
            detail: Some(error),
            findings: None,
            summary: None,
        },
    }
}

fn disposition_match_status(repo_root: &Path) -> Result<(StepStatus, String), String> {
    let state_value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(state_value)
        .map_err(|error| format!("failed to parse state.json: {}", error))?;
    let review_agent = state.review_agent()?;
    let Some(history_entry) = review_agent.history.last() else {
        return Ok((StepStatus::Pass, "no review history".to_string()));
    };

    let review_relative_path = format!("docs/reviews/cycle-{}.md", history_entry.cycle);
    let review_path = repo_root.join(&review_relative_path);
    if !review_path.is_file() {
        return Ok((
            StepStatus::Warn,
            format!("review file not found: {}", review_relative_path),
        ));
    }

    let review_content = fs::read_to_string(&review_path)
        .map_err(|error| format!("failed to read {}: {}", review_path.display(), error))?;
    let review_finding_count = count_review_findings(&review_content)?;
    let disposition_sum = checked_disposition_sum(history_entry)?;

    let mut details = vec![format!(
        "{} findings in review file for cycle {}",
        review_finding_count, history_entry.cycle
    )];

    if history_entry.finding_count != review_finding_count {
        details.push(format!(
            "history finding_count {} does not match review file {}",
            history_entry.finding_count, review_finding_count
        ));
        if disposition_sum == history_entry.finding_count {
            details.push(format!(
                "disposition sum {} matches history finding_count",
                disposition_sum
            ));
        } else {
            details.push(format!(
                "disposition sum {} does not match finding_count {}",
                disposition_sum, history_entry.finding_count
            ));
        }
        return Ok((StepStatus::Fail, details.join("; ")));
    }

    if disposition_sum != history_entry.finding_count {
        details.push(format!(
            "disposition sum {} does not match finding_count {}",
            disposition_sum, history_entry.finding_count
        ));
        return Ok((StepStatus::Warn, details.join("; ")));
    }

    details.push("history finding_count and dispositions match review file".to_string());
    Ok((StepStatus::Pass, details.join("; ")))
}

struct StepAssessment {
    status: StepStatus,
    severity: Severity,
    detail: String,
}

fn deferral_accumulation_assessment(repo_root: &Path) -> Result<StepAssessment, String> {
    let state_value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(state_value)
        .map_err(|error| format!("failed to parse state.json: {}", error))?;
    let review_agent = state.review_agent()?;

    if review_agent.history.is_empty() {
        return Ok(StepAssessment {
            status: StepStatus::Pass,
            severity: Severity::Warning,
            detail: "no review history".to_string(),
        });
    }

    let accumulations = find_deferral_accumulations(&review_agent.history);
    if accumulations.is_empty() {
        if review_agent
            .history
            .iter()
            .all(|entry| entry.finding_dispositions.is_empty())
        {
            return Ok(StepAssessment {
                status: StepStatus::Pass,
                severity: Severity::Warning,
                detail: "per-finding disposition data not yet available in review history"
                    .to_string(),
            });
        }

        return Ok(StepAssessment {
            status: StepStatus::Pass,
            severity: Severity::Warning,
            detail: format!(
                "no categories deferred {}+ consecutive cycles",
                DEFERRAL_ACCUMULATION_THRESHOLD
            ),
        });
    }

    let latest_cycle = review_agent
        .history
        .iter()
        .filter(|entry| !entry.finding_dispositions.is_empty())
        .map(|entry| entry.cycle)
        .max()
        .ok_or_else(|| "review history is missing per-finding disposition cycles".to_string())?;
    let details = accumulations
        .iter()
        .map(|accumulation| {
            format!(
                "category '{}' deferred in cycles {}",
                accumulation.category,
                format_cycle_list(&accumulation.cycles)
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    let has_recent_accumulation = accumulations
        .iter()
        .any(|accumulation| accumulation.cycles.last() == Some(&latest_cycle));

    Ok(if has_recent_accumulation {
        StepAssessment {
            status: StepStatus::Fail,
            severity: Severity::Blocking,
            detail: details,
        }
    } else {
        StepAssessment {
            status: StepStatus::Warn,
            severity: Severity::Warning,
            detail: details,
        }
    })
}

fn deferral_deadlines_status(repo_root: &Path) -> Result<(StepStatus, String), String> {
    let current_cycle = current_cycle_from_state(repo_root)?;
    let state_value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(state_value)
        .map_err(|error| format!("failed to parse state.json: {}", error))?;

    let active_findings = state
        .deferred_findings
        .iter()
        .filter(|finding| !finding.resolved && finding.dropped_rationale.is_none())
        .collect::<Vec<_>>();
    if active_findings.is_empty() {
        return Ok((StepStatus::Pass, "no active deferred findings are due".to_string()));
    }

    let overdue = active_findings
        .iter()
        .filter(|finding| current_cycle > finding.deadline_cycle)
        .map(|finding| {
            format!(
                "category '{}' is {} cycles overdue (deferred cycle {}, deadline cycle {}, current cycle {})",
                finding.category,
                current_cycle - finding.deadline_cycle,
                finding.deferred_cycle,
                finding.deadline_cycle,
                current_cycle
            )
        })
        .collect::<Vec<_>>();
    if !overdue.is_empty() {
        return Ok((StepStatus::Fail, overdue.join("; ")));
    }

    let due_this_cycle = active_findings
        .iter()
        .filter(|finding| current_cycle == finding.deadline_cycle)
        .map(|finding| {
            format!(
                "category '{}' is due this cycle (deferred cycle {}, deadline cycle {})",
                finding.category, finding.deferred_cycle, finding.deadline_cycle
            )
        })
        .collect::<Vec<_>>();
    if !due_this_cycle.is_empty() {
        return Ok((StepStatus::Warn, due_this_cycle.join("; ")));
    }

    Ok((StepStatus::Pass, "no active deferred findings are due".to_string()))
}

fn mass_deferral_gate_assessment(repo_root: &Path) -> Result<StepAssessment, String> {
    let state_value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(state_value)
        .map_err(|error| format!("failed to parse state.json: {}", error))?;
    let review_agent = state.review_agent()?;
    let Some(history_entry) = review_agent.history.last() else {
        return Ok(StepAssessment {
            status: StepStatus::Pass,
            severity: Severity::Warning,
            detail: "no review history".to_string(),
        });
    };
    if history_entry.finding_count == 0 {
        return Ok(StepAssessment {
            status: StepStatus::Pass,
            severity: Severity::Warning,
            detail: format!("review cycle {} has no findings", history_entry.cycle),
        });
    }

    let detail = format!(
        "review cycle {} deferred {} of {} findings ({:.1}%)",
        history_entry.cycle,
        history_entry.deferred,
        history_entry.finding_count,
        (history_entry.deferred as f64 / history_entry.finding_count as f64) * 100.0
    );
    if history_entry.deferred == history_entry.finding_count {
        return Ok(StepAssessment {
            status: StepStatus::Fail,
            severity: Severity::Blocking,
            detail,
        });
    }
    if meets_mass_deferral_warning_threshold(
        history_entry.deferred,
        history_entry.finding_count,
    ) {
        return Ok(StepAssessment {
            status: StepStatus::Warn,
            severity: Severity::Warning,
            detail,
        });
    }

    Ok(StepAssessment {
        status: StepStatus::Pass,
        severity: Severity::Warning,
        detail,
    })
}

fn meets_mass_deferral_warning_threshold(deferred: u64, finding_count: u64) -> bool {
    deferred.saturating_mul(4) >= finding_count.saturating_mul(3)
}

fn dispatch_finding_reconciliation_status(repo_root: &Path) -> Result<(StepStatus, String), String> {
    let current_cycle = current_cycle_from_state(repo_root)?;
    let Some(previous_cycle) = current_cycle.checked_sub(1) else {
        return Ok((StepStatus::Pass, "cycle 0 has no previous review cycle".to_string()));
    };

    let state_value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(state_value)
        .map_err(|error| format!("failed to parse state.json: {}", error))?;
    let review_agent = state.review_agent()?;
    let Some(history_entry) = review_agent
        .history
        .iter()
        .find(|entry| entry.cycle == previous_cycle)
    else {
        return Ok((
            StepStatus::Pass,
            format!("no review history entry for cycle {}", previous_cycle),
        ));
    };

    if !history_entry
        .finding_dispositions
        .iter()
        .any(|disposition| disposition.disposition == "dispatch_created")
    {
        return Ok((
            StepStatus::Pass,
            format!(
                "review cycle {} has no dispatch_created finding dispositions",
                previous_cycle
            ),
        ));
    }

    let last_cycle_timestamp = state
        .last_cycle
        .timestamp
        .as_deref()
        .ok_or_else(|| "missing /last_cycle/timestamp in state.json".to_string())?;
    let candidate_dispatches = state
        .agent_sessions
        .iter()
        .filter(|session| {
            session
                .dispatched_at
                .as_deref()
                .is_some_and(|dispatched_at| dispatched_at > last_cycle_timestamp)
        })
        .filter(|session| !is_review_dispatch_session(session))
        .filter(|session| !session_has_addresses_finding(session))
        .map(format_dispatch_candidate)
        .collect::<Vec<_>>();

    if candidate_dispatches.is_empty() {
        return Ok((
            StepStatus::Pass,
            format!(
                "review cycle {} has dispatch_created findings and all current-cycle non-review dispatches set addresses_finding",
                previous_cycle
            ),
        ));
    }

    Ok((
        StepStatus::Warn,
        format!(
            "review cycle {} has dispatch_created findings; current-cycle dispatches may need --addresses-finding: {}",
            previous_cycle,
            candidate_dispatches.join(", ")
        ),
    ))
}

fn is_review_dispatch_session(session: &state_schema::AgentSession) -> bool {
    // Review dispatches may be identified either by an explicit flag written by
    // record-dispatch/dispatch-review or by the standardized review title used
    // by cycle-runner when locating existing review issues in state.json.
    session
        .extra
        .get("review_dispatch")
        .and_then(Value::as_bool)
        .unwrap_or(false)
        || session
            .title
            .as_deref()
            .is_some_and(|title| title.contains("[Cycle Review]"))
}

fn session_has_addresses_finding(session: &state_schema::AgentSession) -> bool {
    // AgentSession flattens unknown state.json keys into `extra`, so a
    // top-level JSON field like `addresses_finding` is accessed here.
    session
        .extra
        .get("addresses_finding")
        .is_some_and(|value| !value.is_null())
}

fn format_dispatch_candidate(session: &state_schema::AgentSession) -> String {
    match (session.issue, session.title.as_deref()) {
        (Some(issue), Some(title)) => format!("#{} \"{}\"", issue, title),
        (Some(issue), None) => format!("#{}", issue),
        (None, Some(title)) => format!("\"{}\"", title),
        (None, None) => "dispatch with missing issue/title".to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DeferralAccumulation {
    category: String,
    cycles: Vec<u64>,
}

fn find_deferral_accumulations(
    history: &[state_schema::ReviewHistoryEntry],
) -> Vec<DeferralAccumulation> {
    let mut cycles_by_category: BTreeMap<String, Vec<u64>> = BTreeMap::new();

    for entry in history
        .iter()
        .filter(|entry| !entry.finding_dispositions.is_empty())
    {
        let deferred_categories = entry
            .finding_dispositions
            .iter()
            .filter(|disposition| disposition.disposition == "deferred")
            .map(|disposition| disposition.category.clone())
            .collect::<BTreeSet<_>>();
        for category in deferred_categories {
            cycles_by_category
                .entry(category)
                .or_default()
                .push(entry.cycle);
        }
    }

    let mut accumulations = Vec::new();
    for (category, mut cycles) in cycles_by_category {
        cycles.sort_unstable();
        cycles.dedup();

        let mut streak = Vec::new();
        for cycle in cycles {
            if streak.last().is_none_or(|previous| *previous + 1 == cycle) {
                streak.push(cycle);
                continue;
            }

            if streak.len() >= DEFERRAL_ACCUMULATION_THRESHOLD {
                accumulations.push(DeferralAccumulation {
                    category: category.clone(),
                    cycles: streak.clone(),
                });
            }

            streak.clear();
            streak.push(cycle);
        }

        if streak.len() >= DEFERRAL_ACCUMULATION_THRESHOLD {
            accumulations.push(DeferralAccumulation {
                category,
                cycles: streak,
            });
        }
    }

    accumulations
}

fn format_cycle_list(cycles: &[u64]) -> String {
    cycles
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

fn count_review_findings(review_content: &str) -> Result<u64, String> {
    let count = REVIEW_FINDING_HEADER_REGEX.find_iter(review_content).count();
    u64::try_from(count).map_err(|error| format!("review finding count overflow: {}", error))
}

fn checked_disposition_sum(
    history_entry: &state_schema::ReviewHistoryEntry,
) -> Result<u64, String> {
    history_entry
        .actioned
        .checked_add(history_entry.deferred)
        .and_then(|sum| sum.checked_add(history_entry.dispatch_created))
        .and_then(|sum| sum.checked_add(history_entry.actioned_failed))
        .and_then(|sum| sum.checked_add(history_entry.verified_resolved))
        .and_then(|sum| sum.checked_add(history_entry.ignored))
        .ok_or_else(|| {
            format!(
                "disposition sum overflow for review cycle {}",
                history_entry.cycle
            )
        })
}

fn verify_review_artifact_exists(repo_root: &Path) -> Result<(StepStatus, String), String> {
    let state = read_state_value(repo_root)?;
    let cycle = state
        .pointer(REVIEW_LAST_CYCLE_PATH)
        .and_then(Value::as_u64)
        .ok_or_else(|| format!("missing integer: {}", REVIEW_LAST_CYCLE_PATH))?;
    let review_path = repo_root.join(format!("docs/reviews/cycle-{}.md", cycle));
    let review_exists = review_path.is_file();
    let mut status = if review_exists {
        StepStatus::Pass
    } else {
        StepStatus::Warn
    };
    let mut details = vec![if review_exists {
        format!("Review artifact present for cycle {}", cycle)
    } else {
        format!("Review artifact missing for cycle {}", cycle)
    }];

    if let Some(detail) = review_artifact_fallback_warning(repo_root, &state)? {
        status = StepStatus::Warn;
        details.push(detail);
    }

    Ok((status, details.join("; ")))
}

fn review_artifact_fallback_warning(
    repo_root: &Path,
    state: &Value,
) -> Result<Option<String>, String> {
    let Some(current_cycle) = state
        .pointer(LAST_CYCLE_NUMBER_PATH)
        .and_then(Value::as_u64)
    else {
        return Ok(None);
    };
    if current_cycle <= MIN_CURRENT_CYCLE_FOR_FALLBACK_WARNING {
        return Ok(None);
    }
    if !copilot_review_fallback_needed(state) {
        return Ok(None);
    }

    let min_acceptable_review_cycle = current_cycle.saturating_sub(1);
    let latest_review_cycle = latest_review_artifact_cycle(&repo_root.join("docs/reviews"))?;
    if latest_review_cycle.is_some_and(|cycle| cycle >= min_acceptable_review_cycle) {
        return Ok(None);
    }

    Ok(Some(
        "No review artifact for current cycle — C6.1 self-review fallback may be needed"
            .to_string(),
    ))
}

fn copilot_review_fallback_needed(state: &Value) -> bool {
    state
        .pointer("/in_flight_sessions")
        .and_then(Value::as_u64)
        .is_some_and(|in_flight| in_flight == 0)
        && state
            .pointer(BLOCKERS_PATH)
            .and_then(Value::as_array)
            .is_some_and(|blockers| blockers.iter().any(blocker_mentions_copilot))
}

fn blocker_mentions_copilot(blocker: &Value) -> bool {
    match blocker {
        Value::String(text) => text.to_ascii_lowercase().contains("copilot"),
        Value::Array(items) => items.iter().any(blocker_mentions_copilot),
        Value::Object(entries) => entries.values().any(blocker_mentions_copilot),
        _ => false,
    }
}

fn latest_review_artifact_cycle(review_dir: &Path) -> Result<Option<u64>, String> {
    if !review_dir.is_dir() {
        return Ok(None);
    }

    let entries = fs::read_dir(review_dir)
        .map_err(|error| format!("failed to read {}: {}", review_dir.display(), error))?;
    let mut latest: Option<u64> = None;

    for entry in entries {
        let entry =
            entry.map_err(|error| format!("failed to read {}: {}", review_dir.display(), error))?;
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
        let Some(cycle) = review_artifact_cycle_from_file_name(file_name) else {
            continue;
        };
        latest = Some(match latest {
            Some(existing) => existing.max(cycle),
            None => cycle,
        });
    }

    Ok(latest)
}

fn review_artifact_cycle_from_file_name(file_name: &str) -> Option<u64> {
    file_name
        .strip_prefix("cycle-")
        .and_then(|suffix| suffix.strip_suffix(".md"))
        .and_then(|value| value.parse::<u64>().ok())
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
        && value.chars().enumerate().all(|(index, ch)| match index {
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
    print!("{}", render_human_report(report));
}

fn render_human_report(report: &PipelineReport) -> String {
    let mut output = String::new();
    output.push_str(&format!("Pipeline Check — Cycle {}\n\n", report.cycle));

    for (index, step) in report.steps.iter().enumerate() {
        let summary = match step.name {
            "metric-snapshot" => step.detail.as_deref().unwrap_or(""),
            "housekeeping-scan" => step.detail.as_deref().unwrap_or(""),
            "cycle-status" => step.summary.as_deref().unwrap_or(""),
            _ => step.detail.as_deref().unwrap_or(""),
        };
        if summary.is_empty() {
            output.push_str(&format!(
                "  {}. {:<19} {:<5}\n",
                index + 1,
                format!("{}:", step.name),
                step_status_label(step.status)
            ));
        } else {
            output.push_str(&format!(
                "  {}. {:<19} {:<5} ({})\n",
                index + 1,
                format!("{}:", step.name),
                step_status_label(step.status),
                summary
            ));
        }
    }

    let warning_count = report
        .steps
        .iter()
        .filter(|step| step.status == StepStatus::Warn)
        .count();
    let cascade_count = report
        .steps
        .iter()
        .filter(|step| step.status == StepStatus::Cascade)
        .count();

    output.push('\n');
    let mut suffixes = Vec::new();
    if warning_count > 0 {
        let suffix = if warning_count == 1 {
            "warning"
        } else {
            "warnings"
        };
        suffixes.push(format!("{} {}", warning_count, suffix));
    }
    if cascade_count > 0 {
        let suffix = if cascade_count == 1 {
            "cascade"
        } else {
            "cascades"
        };
        suffixes.push(format!("{} {}", cascade_count, suffix));
    }
    if suffixes.is_empty() {
        output.push_str(&format!("Overall: {}\n", step_status_label(report.overall)));
    } else {
        output.push_str(&format!(
            "Overall: {} ({})\n",
            step_status_label(report.overall),
            suffixes.join(", ")
        ));
    }

    output
}

fn step_status_label(status: StepStatus) -> &'static str {
    match status {
        StepStatus::Pass => "PASS",
        StepStatus::Warn => "WARN",
        StepStatus::Cascade => "CASCADE",
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
        assert_eq!(
            step.detail.as_deref(),
            Some("WARNING: metadata refresh pending")
        );
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
        assert_eq!(
            severity_for_kind(&ToolKind::HousekeepingScan),
            Severity::Warning
        );
        assert_eq!(step.status, StepStatus::Warn);
        assert_eq!(step.findings, Some(1));
        assert_eq!(step.detail.as_deref(), Some("1 findings"));
    }

    #[test]
    fn state_invariants_detail_includes_warns_in_total_when_present() {
        let mut checks = vec![json!({"status": "pass"}); 14];
        checks.push(json!({"status": "warn"}));
        let execution = ExecutionResult {
            exit_code: Some(0),
            stdout: json!({
                "passed": 14,
                "failed": 0,
                "checks": checks
            })
            .to_string(),
        };
        let step = classify_step("state-invariants", &ToolKind::StateInvariants, execution);
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(
            step.detail.as_deref(),
            Some("14/15 invariants pass, 1 warn")
        );
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
        assert_eq!(
            severity_for_kind(&ToolKind::CycleStatus),
            Severity::Blocking
        );
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.exit_code, Some(1));
        assert_eq!(
            step.summary.as_deref(),
            Some("0 in-flight, 0 eva directives")
        );
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
    fn pipeline_with_only_cascade_steps_is_overall_pass_and_has_no_blocking_findings() {
        let report = PipelineReport {
            cycle: 10,
            overall: StepStatus::Pass,
            timestamp: "2026-01-01T00:00:00Z".to_string(),
            has_blocking_findings: false,
            steps: vec![StepReport {
                name: "doc-validation",
                status: StepStatus::Cascade,
                severity: Severity::Blocking,
                exit_code: Some(1),
                detail: Some("worklog validation failed: pipeline status mismatch".to_string()),
                findings: None,
                summary: None,
            }],
        };

        assert_eq!(pipeline_overall_status(&report.steps), StepStatus::Pass);
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
        assert_eq!(
            value.get("has_blocking_findings").and_then(Value::as_bool),
            Some(false)
        );
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
    fn render_human_report_includes_cascade_status_and_count() {
        let report = PipelineReport {
            cycle: 10,
            overall: StepStatus::Pass,
            has_blocking_findings: false,
            timestamp: "2026-01-01T00:00:00Z".to_string(),
            steps: vec![
                StepReport {
                    name: "doc-validation",
                    status: StepStatus::Cascade,
                    severity: Severity::Blocking,
                    exit_code: Some(1),
                    detail: Some("worklog validation failed: pipeline status mismatch".to_string()),
                    findings: None,
                    summary: None,
                },
                StepReport {
                    name: "field-inventory",
                    status: StepStatus::Warn,
                    severity: Severity::Warning,
                    exit_code: Some(1),
                    detail: Some("metadata refresh pending".to_string()),
                    findings: None,
                    summary: None,
                },
            ],
        };

        let rendered = render_human_report(&report);
        assert!(rendered.contains("doc-validation:     CASCADE"));
        assert!(rendered.contains("Overall: PASS (1 warning, 1 cascade)"));
    }

    #[test]
    fn run_step_reports_error_when_wrapper_fails() {
        struct FailingRunner<'a> {
            called: &'a AtomicBool,
        }

        impl CommandRunner for FailingRunner<'_> {
            fn run(&self, script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
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
                let has_cycle_arg = args.windows(2).any(|window| {
                    window[0] == "--cycle" && window[1] == self.expected_cycle.to_string()
                });
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
                    "housekeeping-scan" | "cycle-status" | "state-invariants"
                    | "derive-metrics" => {
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
                assert_eq!(
                    issue, 834,
                    "aggregation test should read last_cycle.issue from state"
                );
                let mut bodies = step_comment_bodies(134, &EXPECTED_STEP_IDS);
                bodies.push_str(&step_comment_bodies(135, &EXPECTED_STEP_IDS));
                Ok(bodies)
            }
        }

        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("pipeline-check-test-{}", run_id));
        let _ = fs::remove_dir_all(&root);
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
        fs::write(
            root.join("docs/journal").join(format!("{}.md", today)),
            "# Journal\n",
        )
        .unwrap();
        fs::create_dir_all(root.join("docs/worklog").join(today)).unwrap();
        fs::write(
            root.join("docs/worklog")
                .join(today)
                .join("020304-cycle-135-summary.md"),
            "worklog",
        )
        .unwrap();
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
        assert_eq!(report.steps.len(), 15);
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
        assert_eq!(report.steps[5].name, "artifact-verify");
        assert_eq!(report.steps[5].status, StepStatus::Pass);
        assert_eq!(report.steps[6].name, "disposition-match");
        assert_eq!(report.steps[6].status, StepStatus::Pass);
        assert_eq!(report.steps[7].name, "deferral-accumulation");
        assert_eq!(report.steps[7].status, StepStatus::Pass);
        assert_eq!(report.steps[8].name, "deferral-deadlines");
        assert_eq!(report.steps[8].status, StepStatus::Pass);
        assert_eq!(report.steps[9].name, "mass-deferral-gate");
        assert_eq!(report.steps[9].status, StepStatus::Pass);
        assert_eq!(report.steps[10].name, "dispatch-finding-reconciliation");
        assert_eq!(report.steps[10].status, StepStatus::Pass);
        assert_eq!(report.steps[11].name, "doc-validation");
        assert_eq!(report.steps[11].status, StepStatus::Pass);
        assert_eq!(report.steps[12].name, "worklog-dedup");
        assert_eq!(report.steps[12].status, StepStatus::Pass);
        assert_eq!(report.steps[13].name, "step-comments");
        assert_eq!(report.steps[13].status, StepStatus::Pass);
        assert_eq!(report.steps[14].name, "current-cycle-steps");
        assert_eq!(report.steps[14].status, StepStatus::Pass);
    }

    #[test]
    fn cli_accepts_missing_cycle_argument() {
        let cli = Cli::try_parse_from(["pipeline-check", "--repo-root", "."]).unwrap();
        assert_eq!(cli.repo_root, PathBuf::from("."));
        assert_eq!(cli.cycle, None);
        assert!(cli.exclude_steps.is_empty());
    }

    #[test]
    fn cli_accepts_repeated_exclude_step_argument() {
        let cli = Cli::try_parse_from([
            "pipeline-check",
            "--repo-root",
            ".",
            "--exclude-step",
            "doc-validation",
            "--exclude-step",
            "worklog-dedup",
        ])
        .unwrap();
        assert_eq!(
            cli.exclude_steps,
            vec!["doc-validation".to_string(), "worklog-dedup".to_string()]
        );
    }

    #[test]
    fn cli_rejects_unknown_exclude_step_argument() {
        let result = Cli::try_parse_from([
            "pipeline-check",
            "--repo-root",
            ".",
            "--exclude-step",
            "not-a-real-step",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn run_pipeline_fails_when_wrapper_steps_error_despite_passing_local_checks() {
        struct ErrorRunner;

        impl CommandRunner for ErrorRunner {
            fn run(&self, script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
                Err(format!("failed to invoke {}", script_path.display()))
            }

            fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
                Err("failed to fetch issue comments".to_string())
            }
        }

        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("pipeline-check-fail-all-errors-{}", run_id));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).unwrap();

        let report = run_pipeline(&root, 140, &ErrorRunner);
        assert_eq!(report.overall, StepStatus::Fail);
        assert_eq!(report.steps.len(), 15);
        assert!(report.steps[..5]
            .iter()
            .all(|step| matches!(step.status, StepStatus::Error)));
        assert!(report.steps[13..]
            .iter()
            .all(|step| matches!(step.status, StepStatus::Error | StepStatus::Warn)));
        assert!(report
            .steps
            .iter()
            .any(|step| matches!(step.status, StepStatus::Error)));
    }

    #[test]
    fn doc_validation_passes_when_close_out_docs_are_valid() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-doc-validation-pass-{}", run_id));
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
                assert_eq!(
                    script_path.file_name().and_then(|name| name.to_str()),
                    Some("validate-docs")
                );
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
        assert_eq!(
            calls[0][2],
            root.join("docs/worklog/2026-03-12/020304-cycle-239-summary.md")
                .display()
                .to_string()
        );
        assert_eq!(calls[0][3], "--cycle");
        assert_eq!(calls[0][4], "239");
        assert_eq!(calls[0][5], "--pipeline-status");
        assert_eq!(calls[0][6], "PASS");
        assert_eq!(calls[0][7], "--repo-root");
        assert_eq!(calls[0][8], root.display().to_string());
        assert_eq!(calls[1][0], "journal");
        assert_eq!(calls[1][1], "--file");
        assert_eq!(
            calls[1][2],
            root.join("docs/journal/2026-03-12.md")
                .display()
                .to_string()
        );
    }

    #[test]
    fn doc_validation_skips_when_not_in_close_out_phase() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-doc-validation-skip-phase-{}",
            run_id
        ));
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
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("validate-docs should not run outside close-out");
            }

            fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
                panic!("issue comments are not used in doc validation test");
            }
        }

        let step =
            verify_doc_validation_for_date(&root, "2026-03-12", StepStatus::Pass, &NoRunRunner);
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
        let root =
            std::env::temp_dir().join(format!("pipeline-check-doc-validation-fail-{}", run_id));
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

        let step = verify_doc_validation_for_date(
            &root,
            "2026-03-12",
            StepStatus::Fail,
            &FailingValidateDocsRunner,
        );
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
    fn step_comments_pipeline_cascade_matches_single_pipeline_status_mismatch() {
        assert!(is_step_comments_pipeline_cascade(
			"worklog validation failed: pipeline status mismatch: worklog reports 'PASS', pipeline-check overall is 'fail'",
		));
    }

    #[test]
    fn step_comments_pipeline_cascade_matches_pipeline_mismatch_with_shallow_clone_error() {
        assert!(is_step_comments_pipeline_cascade(
			"worklog validation failed: shallow clone cannot find cycle-complete commit abc1234; pipeline status mismatch: worklog reports 'PASS', pipeline-check overall is 'fail'",
		));
    }

    #[test]
    fn step_comments_pipeline_cascade_rejects_pipeline_mismatch_with_genuine_doc_failure() {
        assert!(!is_step_comments_pipeline_cascade(
			"missing receipts; worklog validation failed: pipeline status mismatch: worklog reports 'PASS', pipeline-check overall is 'fail'",
		));
    }

    #[test]
    fn step_comments_pipeline_cascade_rejects_single_genuine_failure() {
        assert!(!is_step_comments_pipeline_cascade("missing receipts"));
    }

    #[test]
    fn run_pipeline_marks_doc_validation_as_cascade_when_only_step_comments_fail() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-doc-validation-cascade-{}", run_id));
        let today = &current_utc_timestamp()[..10];
        fs::create_dir_all(root.join("docs/journal")).unwrap();
        fs::create_dir_all(root.join("docs/worklog").join(today)).unwrap();
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 834,
                "last_cycle": {"number": 257},
                "cycle_phase": {"phase": "close_out"},
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
                    "last_review_cycle": 257
                }
            })
            .to_string(),
        )
        .unwrap();
        fs::write(
            root.join("docs/worklog")
                .join(today)
                .join("020304-cycle-257-summary.md"),
            "latest worklog",
        )
        .unwrap();
        fs::write(
            root.join("docs/journal").join(format!("{today}.md")),
            "# Journal\n",
        )
        .unwrap();
        fs::write(root.join("docs/reviews/cycle-257.md"), "review").unwrap();

        struct CascadeRunner;

        impl CommandRunner for CascadeRunner {
            fn run(&self, script_path: &Path, args: &[String]) -> Result<ExecutionResult, String> {
                let key = script_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default();
                match key {
                    "metric-snapshot" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"summary":"13/13 checks","checks":[]}).to_string(),
                    }),
                    "check-field-inventory-rs" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: "PASS: all fields covered".to_string(),
                    }),
                    "housekeeping-scan" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"items_needing_attention":0}).to_string(),
                    }),
                    "cycle-status" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({
                            "concurrency": {"in_flight": 1},
                            "eva_input": {"comments_since_last_cycle": [{"x": 1}]}
                        })
                        .to_string(),
                    }),
                    "state-invariants" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"passed":5,"failed":0}).to_string(),
                    }),
                    "derive-metrics" => Ok(ExecutionResult {
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
                    }),
                    "validate-docs" => {
                        let mode = args.first().map(String::as_str).unwrap_or_default();
                        if mode == "worklog" {
                            Ok(ExecutionResult {
								exit_code: Some(1),
								stdout: "pipeline status mismatch: worklog reports 'PASS', pipeline-check overall is 'fail'".to_string(),
							})
                        } else {
                            Ok(ExecutionResult {
                                exit_code: Some(0),
                                stdout: String::new(),
                            })
                        }
                    }
                    other => panic!("unexpected tool invocation: {}", other),
                }
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 834);
                Ok(step_comment_bodies(
                    256,
                    &["0", "0.5", "0.6", "1", "2", "3", "4", "5", "6"],
                ))
            }
        }

        let report = run_pipeline(&root, 257, &CascadeRunner);
        assert_eq!(report.steps[11].name, "doc-validation");
        assert_eq!(report.steps[11].status, StepStatus::Cascade);
        assert_eq!(report.steps[12].name, "worklog-dedup");
        assert_eq!(report.steps[12].status, StepStatus::Pass);
        assert_eq!(report.steps[13].name, "step-comments");
        // Previous-cycle backstop is downgraded to Warn — no blocking failures remain
        assert_eq!(report.steps[13].status, StepStatus::Warn);
        assert_eq!(report.overall, StepStatus::Pass);
        assert!(!report.has_blocking_findings);
    }

    #[test]
    fn run_pipeline_marks_doc_validation_as_cascade_for_multi_cause_step_comments_failures() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-doc-validation-multi-cause-{}",
            run_id
        ));
        let today = &current_utc_timestamp()[..10];
        fs::create_dir_all(root.join("docs/journal")).unwrap();
        fs::create_dir_all(root.join("docs/worklog").join(today)).unwrap();
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 834,
                "last_cycle": {"number": 257},
                "cycle_phase": {"phase": "close_out"},
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
                    "last_review_cycle": 257
                }
            })
            .to_string(),
        )
        .unwrap();
        fs::write(
            root.join("docs/worklog")
                .join(today)
                .join("020304-cycle-257-summary.md"),
            "latest worklog",
        )
        .unwrap();
        fs::write(
            root.join("docs/journal").join(format!("{today}.md")),
            "# Journal\n",
        )
        .unwrap();
        fs::write(root.join("docs/reviews/cycle-257.md"), "review").unwrap();

        struct MultiCauseCascadeRunner;

        impl CommandRunner for MultiCauseCascadeRunner {
            fn run(&self, script_path: &Path, args: &[String]) -> Result<ExecutionResult, String> {
                let key = script_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default();
                match key {
                    "metric-snapshot" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"summary":"13/13 checks","checks":[]}).to_string(),
                    }),
                    "check-field-inventory-rs" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: "PASS: all fields covered".to_string(),
                    }),
                    "housekeeping-scan" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"items_needing_attention":0}).to_string(),
                    }),
                    "cycle-status" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({
                            "concurrency": {"in_flight": 1},
                            "eva_input": {"comments_since_last_cycle": [{"x": 1}]}
                        })
                        .to_string(),
                    }),
                    "state-invariants" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"passed":5,"failed":0}).to_string(),
                    }),
                    "derive-metrics" => Ok(ExecutionResult {
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
                    }),
                    "validate-docs" => {
                        let mode = args.first().map(String::as_str).unwrap_or_default();
                        if mode == "worklog" {
                            Ok(ExecutionResult {
								exit_code: Some(1),
								stdout: "shallow clone cannot find cycle-complete commit abc1234; pipeline status mismatch: worklog reports 'PASS', pipeline-check overall is 'fail'".to_string(),
							})
                        } else {
                            Ok(ExecutionResult {
                                exit_code: Some(0),
                                stdout: String::new(),
                            })
                        }
                    }
                    other => panic!("unexpected tool invocation: {}", other),
                }
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 834);
                Ok(step_comment_bodies(
                    256,
                    &["0", "0.5", "0.6", "1", "2", "3", "4", "5", "6"],
                ))
            }
        }

        let report = run_pipeline(&root, 257, &MultiCauseCascadeRunner);
        assert_eq!(report.steps[11].name, "doc-validation");
        assert_eq!(report.steps[11].status, StepStatus::Cascade);
        assert_eq!(report.steps[12].name, "worklog-dedup");
        assert_eq!(report.steps[12].status, StepStatus::Pass);
        assert_eq!(report.steps[13].name, "step-comments");
        // Previous-cycle backstop is downgraded to Warn — no blocking failures remain
        assert_eq!(report.steps[13].status, StepStatus::Warn);
        assert_eq!(report.overall, StepStatus::Pass);
        assert!(!report.has_blocking_findings);
    }

    #[test]
    fn run_pipeline_respects_cycle_override_for_step_comments() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        const OVERRIDE_CYCLE: u64 = 255;
        const CURRENT_CYCLE: u64 = 258;
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-step-comments-cycle-override-{}",
            run_id
        ));
        let today = &current_utc_timestamp()[..10];
        fs::create_dir_all(root.join("docs/journal")).unwrap();
        fs::create_dir_all(root.join("docs/worklog").join(today)).unwrap();
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 842,
                "last_cycle": {"number": CURRENT_CYCLE},
                "cycle_phase": {"phase": "close_out"},
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
                    "last_review_cycle": CURRENT_CYCLE
                }
            })
            .to_string(),
        )
        .unwrap();
        fs::write(
            root.join("docs/worklog")
                .join(today)
                .join(format!("020304-cycle-{}-summary.md", CURRENT_CYCLE)),
            "latest worklog",
        )
        .unwrap();
        fs::write(
            root.join("docs/journal").join(format!("{today}.md")),
            "# Journal\n",
        )
        .unwrap();
        fs::write(
            root.join(format!("docs/reviews/cycle-{}.md", CURRENT_CYCLE)),
            "review",
        )
        .unwrap();

        struct OverrideRunner;

        impl CommandRunner for OverrideRunner {
            fn run(&self, script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
                let key = script_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default();
                match key {
                    "metric-snapshot" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"summary":"13/13 checks","checks":[]}).to_string(),
                    }),
                    "check-field-inventory-rs" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: "PASS: all fields covered".to_string(),
                    }),
                    "housekeeping-scan" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"items_needing_attention":0}).to_string(),
                    }),
                    "cycle-status" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({
                            "concurrency": {"in_flight": 1},
                            "eva_input": {"comments_since_last_cycle": [{"x": 1}]}
                        })
                        .to_string(),
                    }),
                    "state-invariants" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"passed":5,"failed":0}).to_string(),
                    }),
                    "derive-metrics" => Ok(ExecutionResult {
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
                    }),
                    "validate-docs" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: String::new(),
                    }),
                    other => panic!("unexpected tool invocation: {}", other),
                }
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 842);
                let steps = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "C5.1")
                    .collect::<Vec<_>>();
                Ok(step_comment_bodies(OVERRIDE_CYCLE - 1, &steps))
            }
        }

        let report = run_pipeline(&root, OVERRIDE_CYCLE, &OverrideRunner);
        assert_eq!(report.steps[13].name, "step-comments");
        assert_eq!(report.steps[13].status, StepStatus::Warn);
        assert_eq!(report.steps[13].severity, Severity::Warning);
        assert!(report.steps[13]
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("missing mandatory [none]"));
        assert!(report
            .steps
            .iter()
            .any(|step| {
                step.detail
                    .as_deref()
                    .unwrap_or_default()
                    .contains("missing optional [C5.1]")
            }));
        assert_eq!(report.overall, StepStatus::Pass);
        assert!(!report.has_blocking_findings);
    }

    #[test]
    fn run_pipeline_keeps_doc_validation_fail_for_independent_failures() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-doc-validation-independent-{}",
            run_id
        ));
        let today = &current_utc_timestamp()[..10];
        fs::create_dir_all(root.join("docs/journal")).unwrap();
        fs::create_dir_all(root.join("docs/worklog").join(today)).unwrap();
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 834,
                "last_cycle": {"number": 257},
                "cycle_phase": {"phase": "close_out"},
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
                    "last_review_cycle": 257
                }
            })
            .to_string(),
        )
        .unwrap();
        fs::write(
            root.join("docs/worklog")
                .join(today)
                .join("020304-cycle-257-summary.md"),
            "latest worklog",
        )
        .unwrap();
        fs::write(
            root.join("docs/journal").join(format!("{today}.md")),
            "# Journal\n",
        )
        .unwrap();
        fs::write(root.join("docs/reviews/cycle-257.md"), "review").unwrap();

        struct IndependentFailureRunner;

        impl CommandRunner for IndependentFailureRunner {
            fn run(&self, script_path: &Path, args: &[String]) -> Result<ExecutionResult, String> {
                let key = script_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default();
                match key {
                    "metric-snapshot" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"summary":"13/13 checks","checks":[]}).to_string(),
                    }),
                    "check-field-inventory-rs" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: "PASS: all fields covered".to_string(),
                    }),
                    "housekeeping-scan" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"items_needing_attention":0}).to_string(),
                    }),
                    "cycle-status" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({
                            "concurrency": {"in_flight": 1},
                            "eva_input": {"comments_since_last_cycle": [{"x": 1}]}
                        })
                        .to_string(),
                    }),
                    "state-invariants" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"passed":5,"failed":0}).to_string(),
                    }),
                    "derive-metrics" => Ok(ExecutionResult {
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
                    }),
                    "validate-docs" => {
                        let mode = args.first().map(String::as_str).unwrap_or_default();
                        if mode == "worklog" {
                            Ok(ExecutionResult {
                                exit_code: Some(1),
                                stdout: "missing receipts".to_string(),
                            })
                        } else {
                            Ok(ExecutionResult {
                                exit_code: Some(0),
                                stdout: String::new(),
                            })
                        }
                    }
                    other => panic!("unexpected tool invocation: {}", other),
                }
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 834);
                Ok(step_comment_bodies(
                    256,
                    &["0", "0.5", "0.6", "1", "2", "3", "4", "5", "6"],
                ))
            }
        }

        let report = run_pipeline(&root, 257, &IndependentFailureRunner);
        assert_eq!(report.steps[11].name, "doc-validation");
        assert_eq!(report.steps[11].status, StepStatus::Fail);
        assert_eq!(report.steps[12].name, "worklog-dedup");
        assert_eq!(report.steps[12].status, StepStatus::Pass);
        // Previous-cycle backstop is downgraded to Warn
        assert_eq!(report.steps[13].status, StepStatus::Warn);
        assert_eq!(report.overall, StepStatus::Fail);
        assert!(report.has_blocking_findings);
    }

    #[test]
    fn run_pipeline_omits_excluded_steps_from_output() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-exclude-doc-validation-{}", run_id));
        let today = &current_utc_timestamp()[..10];
        fs::create_dir_all(root.join("docs/journal")).unwrap();
        fs::create_dir_all(root.join("docs/worklog").join(today)).unwrap();
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 834,
                "last_cycle": {"number": 257, "issue": 834},
                "cycle_phase": {"phase": "close_out"},
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
                    "last_review_cycle": 257
                }
            })
            .to_string(),
        )
        .unwrap();
        fs::write(
            root.join("docs/worklog")
                .join(today)
                .join("020304-cycle-257-summary.md"),
            "latest worklog",
        )
        .unwrap();
        fs::write(
            root.join("docs/journal").join(format!("{today}.md")),
            "# Journal\n",
        )
        .unwrap();
        fs::write(root.join("docs/reviews/cycle-257.md"), "review").unwrap();

        struct ExcludeDocValidationRunner;

        impl CommandRunner for ExcludeDocValidationRunner {
            fn run(&self, script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
                let key = script_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default();
                match key {
                    "metric-snapshot" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"summary":"13/13 checks","checks":[]}).to_string(),
                    }),
                    "check-field-inventory-rs" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: "PASS: all fields covered".to_string(),
                    }),
                    "housekeeping-scan" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"items_needing_attention":0}).to_string(),
                    }),
                    "cycle-status" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({
                            "concurrency": {"in_flight": 1},
                            "eva_input": {"comments_since_last_cycle": [{"x": 1}]}
                        })
                        .to_string(),
                    }),
                    "state-invariants" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"passed":5,"failed":0}).to_string(),
                    }),
                    "derive-metrics" => Ok(ExecutionResult {
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
                    }),
                    "validate-docs" => panic!("doc-validation should be excluded"),
                    other => panic!("unexpected tool invocation: {}", other),
                }
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 834);
                let mut bodies = step_comment_bodies(256, &EXPECTED_STEP_IDS);
                bodies.push_str(&step_comment_bodies(257, &EXPECTED_STEP_IDS));
                Ok(bodies)
            }
        }

        let report = run_pipeline_with_excluded_steps(
            &root,
            257,
            &["doc-validation".to_string()],
            &ExcludeDocValidationRunner,
        );
        assert_eq!(report.overall, StepStatus::Pass);
        assert_eq!(report.steps.len(), 14);
        assert!(!report
            .steps
            .iter()
            .any(|step| step.name == "doc-validation"));
        assert!(report.steps.iter().any(|step| step.name == "worklog-dedup"));
        assert!(report.steps.iter().any(|step| step.name == "step-comments"));
        assert!(report
            .steps
            .iter()
            .any(|step| step.name == "current-cycle-steps"));
    }

    #[test]
    fn run_pipeline_ignores_unknown_excluded_step_names() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("pipeline-check-exclude-unknown-{}", run_id));
        let today = &current_utc_timestamp()[..10];
        fs::create_dir_all(root.join("docs/journal")).unwrap();
        fs::create_dir_all(root.join("docs/worklog").join(today)).unwrap();
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 834,
                "last_cycle": {"number": 257, "issue": 834},
                "cycle_phase": {"phase": "close_out"},
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
                    "last_review_cycle": 257
                }
            })
            .to_string(),
        )
        .unwrap();
        fs::write(
            root.join("docs/worklog")
                .join(today)
                .join("020304-cycle-257-summary.md"),
            "latest worklog",
        )
        .unwrap();
        fs::write(
            root.join("docs/journal").join(format!("{today}.md")),
            "# Journal\n",
        )
        .unwrap();
        fs::write(root.join("docs/reviews/cycle-257.md"), "review").unwrap();

        struct UnknownExcludeRunner;

        impl CommandRunner for UnknownExcludeRunner {
            fn run(&self, script_path: &Path, args: &[String]) -> Result<ExecutionResult, String> {
                let key = script_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default();
                match key {
                    "metric-snapshot" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"summary":"13/13 checks","checks":[]}).to_string(),
                    }),
                    "check-field-inventory-rs" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: "PASS: all fields covered".to_string(),
                    }),
                    "housekeeping-scan" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"items_needing_attention":0}).to_string(),
                    }),
                    "cycle-status" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({
                            "concurrency": {"in_flight": 1},
                            "eva_input": {"comments_since_last_cycle": [{"x": 1}]}
                        })
                        .to_string(),
                    }),
                    "state-invariants" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"passed":5,"failed":0}).to_string(),
                    }),
                    "derive-metrics" => Ok(ExecutionResult {
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
                    }),
                    "validate-docs" => {
                        let mode = args.first().map(String::as_str).unwrap_or_default();
                        assert!(matches!(mode, "worklog" | "journal"));
                        Ok(ExecutionResult {
                            exit_code: Some(0),
                            stdout: String::new(),
                        })
                    }
                    other => panic!("unexpected tool invocation: {}", other),
                }
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 834);
                let mut bodies = step_comment_bodies(256, &EXPECTED_STEP_IDS);
                bodies.push_str(&step_comment_bodies(257, &EXPECTED_STEP_IDS));
                Ok(bodies)
            }
        }

        let report = run_pipeline_with_excluded_steps(
            &root,
            257,
            &["not-a-real-step".to_string()],
            &UnknownExcludeRunner,
        );
        assert_eq!(report.overall, StepStatus::Pass);
        assert_eq!(report.steps.len(), 15);
        assert!(report
            .steps
            .iter()
            .any(|step| step.name == "doc-validation"));
        assert!(report
            .steps
            .iter()
            .any(|step| step.name == "worklog-dedup"));
    }

    #[test]
    fn run_pipeline_omits_worklog_dedup_when_excluded() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-exclude-worklog-dedup-{}",
            run_id
        ));
        let today = &current_utc_timestamp()[..10];
        fs::create_dir_all(root.join("docs/journal")).unwrap();
        fs::create_dir_all(root.join("docs/worklog").join(today)).unwrap();
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 834,
                "last_cycle": {"number": 257, "issue": 834},
                "cycle_phase": {"phase": "close_out"},
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
                    "last_review_cycle": 257
                }
            })
            .to_string(),
        )
        .unwrap();
        fs::write(
            root.join("docs/worklog")
                .join(today)
                .join("020304-cycle-257-summary.md"),
            "latest worklog",
        )
        .unwrap();
        fs::write(
            root.join("docs/worklog")
                .join(today)
                .join("020305-cycle-257-followup.md"),
            "duplicate worklog",
        )
        .unwrap();
        fs::write(
            root.join("docs/journal").join(format!("{today}.md")),
            "# Journal\n",
        )
        .unwrap();
        fs::write(root.join("docs/reviews/cycle-257.md"), "review").unwrap();

        struct ExcludeWorklogDedupRunner;

        impl CommandRunner for ExcludeWorklogDedupRunner {
            fn run(&self, script_path: &Path, args: &[String]) -> Result<ExecutionResult, String> {
                let key = script_path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or_default();
                match key {
                    "metric-snapshot" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"summary":"13/13 checks","checks":[]}).to_string(),
                    }),
                    "check-field-inventory-rs" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: "PASS: all fields covered".to_string(),
                    }),
                    "housekeeping-scan" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"items_needing_attention":0}).to_string(),
                    }),
                    "cycle-status" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({
                            "concurrency": {"in_flight": 1},
                            "eva_input": {"comments_since_last_cycle": [{"x": 1}]}
                        })
                        .to_string(),
                    }),
                    "state-invariants" => Ok(ExecutionResult {
                        exit_code: Some(0),
                        stdout: json!({"passed":5,"failed":0}).to_string(),
                    }),
                    "derive-metrics" => Ok(ExecutionResult {
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
                    }),
                    "validate-docs" => {
                        let mode = args.first().map(String::as_str).unwrap_or_default();
                        assert!(matches!(mode, "worklog" | "journal"));
                        Ok(ExecutionResult {
                            exit_code: Some(0),
                            stdout: String::new(),
                        })
                    }
                    other => panic!("unexpected tool invocation: {}", other),
                }
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 834);
                let mut bodies = step_comment_bodies(256, &EXPECTED_STEP_IDS);
                bodies.push_str(&step_comment_bodies(257, &EXPECTED_STEP_IDS));
                Ok(bodies)
            }
        }

        let report = run_pipeline_with_excluded_steps(
            &root,
            257,
            &["worklog-dedup".to_string()],
            &ExcludeWorklogDedupRunner,
        );

        assert_eq!(report.overall, StepStatus::Pass);
        assert_eq!(report.steps.len(), 14);
        assert!(!report
            .steps
            .iter()
            .any(|step| step.name == "worklog-dedup"));
    }

    #[test]
    fn doc_validation_handles_missing_worklog_or_journal_gracefully() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-doc-validation-missing-{}", run_id));
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
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("validate-docs should not run when docs are missing");
            }

            fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
                panic!("issue comments are not used in doc validation test");
            }
        }

        let step =
            verify_doc_validation_for_date(&root, "2026-03-12", StepStatus::Pass, &NoRunRunner);
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
        let journal_dir =
            std::env::temp_dir().join(format!("pipeline-check-non-dated-journal-files-{}", run_id));
        fs::create_dir_all(&journal_dir).unwrap();
        fs::write(journal_dir.join("notes.md"), "# Notes\n").unwrap();
        fs::write(journal_dir.join("readme.txt"), "notes").unwrap();

        assert_eq!(latest_journal_file_date(&journal_dir).unwrap(), None);
    }

    #[test]
    fn latest_journal_file_date_skips_malformed_date_filenames() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let journal_dir =
            std::env::temp_dir().join(format!("pipeline-check-malformed-journal-files-{}", run_id));
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
    fn worklog_dedup_passes_when_today_has_no_worklog_files() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-worklog-dedup-empty-{}", run_id));
        fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();

        let step = verify_worklog_dedup_for_date(&root, "2026-03-09");

        assert_eq!(step.name, "worklog-dedup");
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.severity, Severity::Blocking);
        assert_eq!(
            step.detail.as_deref(),
            Some("No duplicate worklog files found")
        );
    }

    #[test]
    fn worklog_dedup_passes_with_one_file_per_cycle() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-worklog-dedup-unique-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();
        fs::write(
            root.join("docs/worklog/2026-03-09/120000-cycle-210-summary.md"),
            "# Worklog\n",
        )
        .unwrap();
        fs::write(
            root.join("docs/worklog/2026-03-09/120100-cycle-211-summary.md"),
            "# Worklog\n",
        )
        .unwrap();
        fs::create_dir_all(root.join("docs/worklog/2026-03-08")).unwrap();
        fs::write(
            root.join("docs/worklog/2026-03-08/120000-cycle-210-legacy.md"),
            "# Older worklog\n",
        )
        .unwrap();

        let step = verify_worklog_dedup_for_date(&root, "2026-03-09");

        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(
            step.detail.as_deref(),
            Some("No duplicate worklog files found")
        );
    }

    #[test]
    fn worklog_dedup_fails_when_cycle_has_multiple_files() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-worklog-dedup-duplicate-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();
        fs::write(
            root.join("docs/worklog/2026-03-09/120000-cycle-354-summary.md"),
            "# Worklog A\n",
        )
        .unwrap();
        fs::write(
            root.join("docs/worklog/2026-03-09/120100-cycle-354-followup.md"),
            "# Worklog B\n",
        )
        .unwrap();

        let step = verify_worklog_dedup_for_date(&root, "2026-03-09");

        assert_eq!(step.status, StepStatus::Fail);
        let detail = step.detail.as_deref().unwrap_or_default();
        assert!(detail.contains("Duplicate worklog files for cycle 354"));
        assert!(detail.contains("120000-cycle-354-summary.md"));
        assert!(detail.contains("120100-cycle-354-followup.md"));
    }

    #[test]
    fn worklog_dedup_warns_when_file_is_missing_cycle_marker() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-worklog-dedup-legacy-name-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();
        fs::write(root.join("docs/worklog/2026-03-09/entry.md"), "# Worklog\n").unwrap();

        let step = verify_worklog_dedup_for_date(&root, "2026-03-09");

        assert_eq!(step.status, StepStatus::Warn);
        let detail = step.detail.as_deref().unwrap_or_default();
        assert!(detail.contains("missing cycle-NNN pattern"));
        assert!(detail.contains("entry.md"));
    }

    #[test]
    fn worklog_dedup_warns_without_blocking_when_only_legacy_name_is_extra() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-worklog-dedup-mixed-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();
        fs::write(
            root.join("docs/worklog/2026-03-09/120000-cycle-354-summary.md"),
            "# Worklog\n",
        )
        .unwrap();
        fs::write(root.join("docs/worklog/2026-03-09/notes.md"), "# Notes\n").unwrap();

        let step = verify_worklog_dedup_for_date(&root, "2026-03-09");

        assert_eq!(step.status, StepStatus::Warn);
        let detail = step.detail.as_deref().unwrap_or_default();
        assert!(detail.contains("No duplicate worklog files found"));
        assert!(detail.contains("notes.md"));
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
    fn artifact_verification_does_not_warn_when_review_artifact_exists_for_current_cycle() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-artifacts-current-{}", run_id));
        fs::create_dir_all(root.join("docs/journal")).unwrap();
        fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(root.join("docs/journal/2026-03-09.md"), "# Journal\n").unwrap();
        fs::write(
            root.join("docs/worklog/2026-03-09/120000-cycle-210-summary.md"),
            "# Worklog\n",
        )
        .unwrap();
        fs::write(root.join("docs/reviews/cycle-210.md"), "# Review\n").unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "last_cycle": {"number": 210},
                "review_agent": {"last_review_cycle": 210},
                "in_flight_sessions": 0,
                "blockers": [{"summary": "Copilot outage continues"}]
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
            .contains("Review artifact present for cycle 210"));
        assert!(!step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("C6.1 self-review fallback"));
    }

    #[test]
    fn artifact_verification_does_not_warn_for_stale_review_when_copilot_is_active() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("pipeline-check-artifacts-active-{}", run_id));
        fs::create_dir_all(root.join("docs/journal")).unwrap();
        fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(root.join("docs/journal/2026-03-09.md"), "# Journal\n").unwrap();
        fs::write(
            root.join("docs/worklog/2026-03-09/120000-cycle-210-summary.md"),
            "# Worklog\n",
        )
        .unwrap();
        fs::write(root.join("docs/reviews/cycle-208.md"), "# Review\n").unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "last_cycle": {"number": 210},
                "review_agent": {"last_review_cycle": 208},
                "in_flight_sessions": 2,
                "blockers": [{"summary": "Copilot outage continues"}]
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
            .contains("Review artifact present for cycle 208"));
        assert!(!step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("C6.1 self-review fallback"));
    }

    #[test]
    fn artifact_verification_warns_when_recent_review_artifacts_are_missing_during_copilot_outage()
    {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("pipeline-check-artifacts-outage-{}", run_id));
        fs::create_dir_all(root.join("docs/journal")).unwrap();
        fs::create_dir_all(root.join("docs/worklog/2026-03-09")).unwrap();
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(root.join("docs/journal/2026-03-09.md"), "# Journal\n").unwrap();
        fs::write(
            root.join("docs/worklog/2026-03-09/120000-cycle-210-summary.md"),
            "# Worklog\n",
        )
        .unwrap();
        fs::write(root.join("docs/reviews/cycle-208.md"), "# Review\n").unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "last_cycle": {"number": 210},
                "review_agent": {"last_review_cycle": 208},
                "in_flight_sessions": 0,
                "blockers": [{"summary": "Copilot outage continues"}]
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_artifacts_for_date(&root, "2026-03-09");

        assert_eq!(step.status, StepStatus::Warn);
        assert!(step.detail.as_deref().unwrap_or_default().contains(
            "No review artifact for current cycle — C6.1 self-review fallback may be needed"
        ));
    }

    #[test]
    fn disposition_match_passes_when_review_history_and_review_file_match() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-disposition-match-pass-{}", run_id));
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(
            root.join("docs/reviews/cycle-210.md"),
            concat!(
                "# Cycle 210 Review\n\n",
                "## 1. [validation] First finding\n\n",
                "Details\n\n",
                "## 2. [testing] Second finding\n\n",
                "Details\n\n",
                "## Complacency score\n\n",
                "1/5\n"
            ),
        )
        .unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": [{
                        "cycle": 210,
                        "categories": ["validation", "testing"],
                        "actioned": 1,
                        "deferred": 0,
                        "dispatch_created": 1,
                        "ignored": 0,
                        "finding_count": 2,
                        "complacency_score": 1
                    }]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_disposition_match(&root);

        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.name, "disposition-match");
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("2 findings"));
    }

    #[test]
    fn disposition_match_warns_when_disposition_sum_does_not_match_finding_count() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-disposition-match-warn-{}", run_id));
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(
            root.join("docs/reviews/cycle-211.md"),
            concat!(
                "# Cycle 211 Review\n\n",
                "## 1. [validation] First finding\n\n",
                "## 2. [testing] Second finding\n"
            ),
        )
        .unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": [{
                        "cycle": 211,
                        "categories": ["validation", "testing"],
                        "actioned": 1,
                        "deferred": 0,
                        "dispatch_created": 0,
                        "ignored": 0,
                        "finding_count": 2,
                        "complacency_score": 1
                    }]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_disposition_match(&root);

        assert_eq!(step.status, StepStatus::Warn);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("disposition sum 1 does not match finding_count 2"));
    }

    #[test]
    fn disposition_match_fails_when_history_finding_count_differs_from_review_file() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-disposition-match-fail-{}", run_id));
        fs::create_dir_all(root.join("docs/reviews")).unwrap();
        fs::write(
            root.join("docs/reviews/cycle-212.md"),
            concat!(
                "# Cycle 212 Review\n\n",
                "## 1. [validation] First finding\n\n",
                "## 2. [testing] Second finding\n\n",
                "## 3. [docs] Third finding\n"
            ),
        )
        .unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": [{
                        "cycle": 212,
                        "categories": ["validation", "testing", "docs"],
                        "actioned": 1,
                        "deferred": 1,
                        "dispatch_created": 0,
                        "ignored": 0,
                        "finding_count": 2,
                        "complacency_score": 1
                    }]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_disposition_match(&root);

        assert_eq!(step.status, StepStatus::Fail);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("history finding_count 2 does not match review file 3"));
    }

    #[test]
    fn disposition_match_warns_when_review_file_is_missing() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-disposition-match-missing-file-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": [{
                        "cycle": 213,
                        "categories": ["validation"],
                        "actioned": 1,
                        "deferred": 0,
                        "dispatch_created": 0,
                        "ignored": 0,
                        "finding_count": 1,
                        "complacency_score": 1
                    }]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_disposition_match(&root);

        assert_eq!(step.status, StepStatus::Warn);
        assert_eq!(
            step.detail.as_deref(),
            Some("review file not found: docs/reviews/cycle-213.md")
        );
    }

    #[test]
    fn disposition_match_passes_when_review_history_is_empty() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-disposition-match-empty-history-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": []
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_disposition_match(&root);

        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.detail.as_deref(), Some("no review history"));
    }

    #[test]
    fn deferral_accumulation_passes_when_review_history_is_empty() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-deferral-accumulation-empty-history-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": []
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_deferral_accumulation(&root);

        assert_eq!(step.name, "deferral-accumulation");
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.detail.as_deref(), Some("no review history"));
    }

    #[test]
    fn deferral_accumulation_passes_when_per_finding_data_is_unavailable() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-deferral-accumulation-no-finding-data-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": [{
                        "cycle": 210,
                        "categories": ["journal-quality"],
                        "actioned": 0,
                        "deferred": 1,
                        "ignored": 0,
                        "finding_count": 1,
                        "complacency_score": 1
                    }]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_deferral_accumulation(&root);

        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(
            step.detail.as_deref(),
            Some("per-finding disposition data not yet available in review history")
        );
    }

    #[test]
    fn deferral_accumulation_passes_for_two_consecutive_deferred_cycles() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-deferral-accumulation-two-cycles-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": [
                        {
                            "cycle": 348,
                            "categories": ["journal-quality"],
                            "actioned": 0,
                            "deferred": 1,
                            "ignored": 0,
                            "finding_count": 1,
                            "complacency_score": 2,
                            "finding_dispositions": [{
                                "category": "journal-quality",
                                "disposition": "deferred"
                            }]
                        },
                        {
                            "cycle": 349,
                            "categories": ["journal-quality"],
                            "actioned": 0,
                            "deferred": 1,
                            "ignored": 0,
                            "finding_count": 1,
                            "complacency_score": 2,
                            "finding_dispositions": [{
                                "category": "journal-quality",
                                "disposition": "deferred"
                            }]
                        }
                    ]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_deferral_accumulation(&root);

        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(
            step.detail.as_deref(),
            Some("no categories deferred 3+ consecutive cycles")
        );
    }

    #[test]
    fn deferral_accumulation_warns_for_three_consecutive_deferred_cycles() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-deferral-accumulation-three-cycles-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": [
                        {
                            "cycle": 348,
                            "categories": ["journal-quality"],
                            "actioned": 0,
                            "deferred": 1,
                            "ignored": 0,
                            "finding_count": 1,
                            "complacency_score": 2,
                            "finding_dispositions": [{
                                "category": "journal-quality",
                                "disposition": "deferred"
                            }]
                        },
                        {
                            "cycle": 349,
                            "categories": ["journal-quality"],
                            "actioned": 0,
                            "deferred": 1,
                            "ignored": 0,
                            "finding_count": 1,
                            "complacency_score": 2,
                            "finding_dispositions": [{
                                "category": "journal-quality",
                                "disposition": "deferred"
                            }]
                        },
                        {
                            "cycle": 350,
                            "categories": ["journal-quality"],
                            "actioned": 0,
                            "deferred": 1,
                            "ignored": 0,
                            "finding_count": 1,
                            "complacency_score": 2,
                            "finding_dispositions": [{
                                "category": "journal-quality",
                                "disposition": "deferred"
                            }]
                        }
                    ]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_deferral_accumulation(&root);

        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        assert_eq!(
            step.detail.as_deref(),
            Some("category 'journal-quality' deferred in cycles 348, 349, 350")
        );
    }

    #[test]
    fn deferral_accumulation_warns_for_historical_three_cycle_streak() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-deferral-accumulation-historical-three-cycles-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": [
                        {
                            "cycle": 348,
                            "categories": ["journal-quality"],
                            "actioned": 0,
                            "deferred": 1,
                            "ignored": 0,
                            "finding_count": 1,
                            "complacency_score": 2,
                            "finding_dispositions": [{
                                "category": "journal-quality",
                                "disposition": "deferred"
                            }]
                        },
                        {
                            "cycle": 349,
                            "categories": ["journal-quality"],
                            "actioned": 0,
                            "deferred": 1,
                            "ignored": 0,
                            "finding_count": 1,
                            "complacency_score": 2,
                            "finding_dispositions": [{
                                "category": "journal-quality",
                                "disposition": "deferred"
                            }]
                        },
                        {
                            "cycle": 350,
                            "categories": ["journal-quality"],
                            "actioned": 0,
                            "deferred": 1,
                            "ignored": 0,
                            "finding_count": 1,
                            "complacency_score": 2,
                            "finding_dispositions": [{
                                "category": "journal-quality",
                                "disposition": "deferred"
                            }]
                        },
                        {
                            "cycle": 351,
                            "categories": ["journal-quality"],
                            "actioned": 1,
                            "deferred": 0,
                            "ignored": 0,
                            "finding_count": 1,
                            "complacency_score": 2,
                            "finding_dispositions": [{
                                "category": "journal-quality",
                                "disposition": "actioned"
                            }]
                        }
                    ]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_deferral_accumulation(&root);

        assert_eq!(step.status, StepStatus::Warn);
        assert_eq!(step.severity, Severity::Warning);
        assert_eq!(
            step.detail.as_deref(),
            Some("category 'journal-quality' deferred in cycles 348, 349, 350")
        );
    }

    #[test]
    fn deferral_accumulation_fails_only_for_categories_meeting_the_recent_threshold() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-deferral-accumulation-mixed-categories-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": [
                        {
                            "cycle": 348,
                            "categories": ["journal-quality", "receipt-integrity"],
                            "actioned": 0,
                            "deferred": 2,
                            "ignored": 0,
                            "finding_count": 2,
                            "complacency_score": 2,
                            "finding_dispositions": [
                                {"category": "journal-quality", "disposition": "deferred"},
                                {"category": "receipt-integrity", "disposition": "deferred"}
                            ]
                        },
                        {
                            "cycle": 349,
                            "categories": ["journal-quality", "receipt-integrity"],
                            "actioned": 1,
                            "deferred": 1,
                            "ignored": 0,
                            "finding_count": 2,
                            "complacency_score": 2,
                            "finding_dispositions": [
                                {"category": "journal-quality", "disposition": "deferred"},
                                {"category": "receipt-integrity", "disposition": "actioned"}
                            ]
                        },
                        {
                            "cycle": 350,
                            "categories": ["journal-quality"],
                            "actioned": 0,
                            "deferred": 1,
                            "ignored": 0,
                            "finding_count": 1,
                            "complacency_score": 2,
                            "finding_dispositions": [
                                {"category": "journal-quality", "disposition": "deferred"}
                            ]
                        }
                    ]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_deferral_accumulation(&root);

        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("journal-quality"));
        assert!(!step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("receipt-integrity"));
    }

    #[test]
    fn deferral_deadlines_fail_when_finding_is_overdue() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-deferral-deadlines-overdue-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "last_cycle": {"number": 401},
                "deferred_findings": [{
                    "category": "review-accounting",
                    "deferred_cycle": 394,
                    "deadline_cycle": 399,
                    "resolved": false
                }]
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_deferral_deadlines(&root);

        assert_eq!(step.name, "deferral-deadlines");
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        let detail = step.detail.as_deref().unwrap_or_default();
        assert!(detail.contains("review-accounting"));
        assert!(detail.contains("overdue"));
        assert!(detail.contains("deadline cycle 399"));
    }

    #[test]
    fn deferral_deadlines_warn_when_finding_is_due_this_cycle() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-deferral-deadlines-due-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "last_cycle": {"number": 399},
                "deferred_findings": [{
                    "category": "review-accounting",
                    "deferred_cycle": 394,
                    "deadline_cycle": 399,
                    "resolved": false
                }]
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_deferral_deadlines(&root);

        assert_eq!(step.status, StepStatus::Warn);
        assert_eq!(step.severity, Severity::Blocking);
        let detail = step.detail.as_deref().unwrap_or_default();
        assert!(detail.contains("review-accounting"));
        assert!(detail.contains("due this cycle"));
    }

    #[test]
    fn deferral_deadlines_ignore_resolved_findings() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-deferral-deadlines-resolved-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "last_cycle": {"number": 401},
                "deferred_findings": [{
                    "category": "review-accounting",
                    "deferred_cycle": 394,
                    "deadline_cycle": 399,
                    "resolved": true,
                    "resolved_ref": "docs/reviews/cycle-395.md"
                }]
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_deferral_deadlines(&root);

        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.detail.as_deref(), Some("no active deferred findings are due"));
    }

    #[test]
    fn mass_deferral_gate_warns_at_seventy_five_percent() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-mass-deferral-gate-warn-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": [{
                        "cycle": 394,
                        "categories": ["review-accounting"],
                        "actioned": 1,
                        "deferred": 3,
                        "ignored": 0,
                        "finding_count": 4,
                        "complacency_score": 2
                    }]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_mass_deferral_gate(&root);

        assert_eq!(step.name, "mass-deferral-gate");
        assert_eq!(step.status, StepStatus::Warn);
        assert_eq!(step.severity, Severity::Warning);
        let detail = step.detail.as_deref().unwrap_or_default();
        assert!(detail.contains("3 of 4"));
        assert!(detail.contains("75.0%"));
    }

    #[test]
    fn mass_deferral_gate_fails_at_one_hundred_percent() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-mass-deferral-gate-fail-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "review_agent": {
                    "history": [{
                        "cycle": 394,
                        "categories": ["review-accounting"],
                        "actioned": 0,
                        "deferred": 4,
                        "ignored": 0,
                        "finding_count": 4,
                        "complacency_score": 2
                    }]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_mass_deferral_gate(&root);

        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        let detail = step.detail.as_deref().unwrap_or_default();
        assert!(detail.contains("4 of 4"));
        assert!(detail.contains("100.0%"));
    }

    #[test]
    fn dispatch_finding_reconciliation_passes_when_review_has_no_dispatch_created_findings() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-dispatch-finding-reconciliation-no-findings-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "last_cycle": {
                    "number": 350,
                    "timestamp": "2026-03-10T00:00:00Z"
                },
                "cycle_phase": {
                    "cycle": 351
                },
                "agent_sessions": [{
                    "issue": 901,
                    "title": "Follow-up dispatch",
                    "dispatched_at": "2026-03-10T12:00:00Z",
                    "status": "in_flight"
                }],
                "review_agent": {
                    "history": [{
                        "cycle": 350,
                        "categories": ["testing"],
                        "actioned": 1,
                        "deferred": 0,
                        "ignored": 0,
                        "finding_count": 1,
                        "complacency_score": 1,
                        "finding_dispositions": [{
                            "category": "testing",
                            "disposition": "actioned"
                        }]
                    }]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_dispatch_finding_reconciliation(&root);

        assert_eq!(step.name, "dispatch-finding-reconciliation");
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(
            step.detail.as_deref(),
            Some("review cycle 350 has no dispatch_created finding dispositions")
        );
    }

    #[test]
    fn dispatch_finding_reconciliation_passes_when_all_relevant_dispatches_are_flagged() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-dispatch-finding-reconciliation-all-flagged-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "last_cycle": {
                    "number": 350,
                    "timestamp": "2026-03-10T00:00:00Z"
                },
                "cycle_phase": {
                    "cycle": 351
                },
                "agent_sessions": [
                    {
                        "issue": 902,
                        "title": "Address review finding",
                        "dispatched_at": "2026-03-10T12:00:00Z",
                        "status": "in_flight",
                        "addresses_finding": "350:1"
                    },
                    {
                        "issue": 800,
                        "title": "Older dispatch",
                        "dispatched_at": "2026-03-09T12:00:00Z",
                        "status": "merged"
                    }
                ],
                "review_agent": {
                    "history": [{
                        "cycle": 350,
                        "categories": ["testing"],
                        "actioned": 0,
                        "deferred": 0,
                        "dispatch_created": 1,
                        "ignored": 0,
                        "finding_count": 1,
                        "complacency_score": 1,
                        "finding_dispositions": [{
                            "category": "testing",
                            "disposition": "dispatch_created"
                        }]
                    }]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_dispatch_finding_reconciliation(&root);

        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(
            step.detail.as_deref(),
            Some(
                "review cycle 350 has dispatch_created findings and all current-cycle non-review dispatches set addresses_finding"
            )
        );
    }

    #[test]
    fn dispatch_finding_reconciliation_warns_when_current_cycle_dispatches_are_unflagged() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-dispatch-finding-reconciliation-warn-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "last_cycle": {
                    "number": 350,
                    "timestamp": "2026-03-10T00:00:00Z"
                },
                "cycle_phase": {
                    "cycle": 351
                },
                "agent_sessions": [
                    {
                        "issue": 903,
                        "title": "Unflagged finding fix",
                        "dispatched_at": "2026-03-10T12:00:00Z",
                        "status": "in_flight",
                        "addresses_finding": null
                    },
                    {
                        "issue": 904,
                        "title": "Flagged finding fix",
                        "dispatched_at": "2026-03-10T12:30:00Z",
                        "status": "in_flight",
                        "addresses_finding": "350:2"
                    }
                ],
                "review_agent": {
                    "history": [{
                        "cycle": 350,
                        "categories": ["testing"],
                        "actioned": 0,
                        "deferred": 0,
                        "dispatch_created": 1,
                        "ignored": 0,
                        "finding_count": 1,
                        "complacency_score": 1,
                        "finding_dispositions": [{
                            "category": "testing",
                            "disposition": "dispatch_created"
                        }]
                    }]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_dispatch_finding_reconciliation(&root);

        assert_eq!(step.status, StepStatus::Warn);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("#903 \"Unflagged finding fix\""));
        assert!(!step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("#904 \"Flagged finding fix\""));
    }

    #[test]
    fn dispatch_finding_reconciliation_excludes_review_dispatches() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-dispatch-finding-reconciliation-review-dispatch-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "last_cycle": {
                    "number": 350,
                    "timestamp": "2026-03-10T00:00:00Z"
                },
                "cycle_phase": {
                    "cycle": 351
                },
                "agent_sessions": [
                    {
                        "issue": 905,
                        "title": "[Cycle Review] Cycle 350 end-of-cycle review",
                        "dispatched_at": "2026-03-10T12:00:00Z",
                        "status": "in_flight",
                        "review_dispatch": true,
                        "addresses_finding": null
                    },
                    {
                        "issue": 906,
                        "title": "Non-review finding fix",
                        "dispatched_at": "2026-03-10T12:30:00Z",
                        "status": "in_flight",
                        "addresses_finding": "350:1"
                    }
                ],
                "review_agent": {
                    "history": [{
                        "cycle": 350,
                        "categories": ["testing"],
                        "actioned": 0,
                        "deferred": 0,
                        "dispatch_created": 1,
                        "ignored": 0,
                        "finding_count": 1,
                        "complacency_score": 1,
                        "finding_dispositions": [{
                            "category": "testing",
                            "disposition": "dispatch_created"
                        }]
                    }]
                }
            })
            .to_string(),
        )
        .unwrap();

        let step = verify_dispatch_finding_reconciliation(&root);

        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(
            step.detail.as_deref(),
            Some(
                "review cycle 350 has dispatch_created findings and all current-cycle non-review dispatches set addresses_finding"
            )
        );
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

        let detected = collect_step_comment_ids(bodies, 212);

        assert_eq!(detected.len(), 5);
        assert!(detected.contains("0"));
        assert!(detected.contains("0.5"));
        assert!(detected.contains("1.1"));
        assert!(detected.contains("C1"));
        assert!(detected.contains("10"));
    }

    #[test]
    fn step_comment_detection_ignores_orchestrator_comments_from_other_cycles() {
        let bodies = concat!(
            "> **[main-orchestrator]** | Cycle 315 | Step 0\n",
            "> **[main-orchestrator]** | Cycle 316 | Step 0.5\n",
            "> **[main-orchestrator]** | Cycle 316 | Step 1\n",
            "> **[main-orchestrator]** | Cycle 317 | Step 2\n"
        );

        let detected = collect_step_comment_ids(bodies, 316);

        assert_eq!(detected.len(), 2);
        assert!(detected.contains("0.5"));
        assert!(detected.contains("1"));
        assert!(!detected.contains("0"));
        assert!(!detected.contains("2"));
    }

    #[test]
    fn step_comment_detection_keeps_old_heading_format_without_cycle_marker() {
        let bodies = concat!(
            "## Step 0: Start cycle\n",
            "## Step 0.5: Validate state\n",
            "> **[main-orchestrator]** | Cycle 315 | Step 1\n"
        );

        let detected = collect_step_comment_ids(bodies, 316);

        assert_eq!(detected.len(), 2);
        assert!(detected.contains("0"));
        assert!(detected.contains("0.5"));
        assert!(!detected.contains("1"));
    }

    #[test]
    fn missing_expected_step_ids_returns_missing_steps_in_expected_order() {
        let found = ["0", "0.5", "1", "2", "6", "7", "9", "10"]
            .into_iter()
            .collect::<BTreeSet<_>>();

        assert_eq!(
            missing_expected_step_ids(&found),
            vec![
                "0.1", "0.6", "1.1", "3", "4", "5", "8", "C1", "C2", "C3", "C4.1", "C4.5", "C5",
                "C5.1", "C5.5", "C5.6", "C6", "C7", "C8",
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
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 834);
                Ok(concat!(
                    "> **[main-orchestrator]** | Cycle 256 | Step 0\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step 0.5\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step 0.6\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step 1\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step 2\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step 3\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step 4\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step 5\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step 6\n"
                )
                .to_string())
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
        assert_eq!(step.name, "step-comments");
        // Previous-cycle backstop is downgraded to Warn/Warning (cycle-aware filter
        // can legitimately reduce per-cycle counts below the backstop threshold)
        assert_eq!(step.status, StepStatus::Warn);
        assert_eq!(step.severity, Severity::Warning);
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
        assert!(step.detail.as_deref().unwrap_or_default().contains(
            "missing mandatory [1.1, 7, 8, 9, C1, C2, C3, C4.1, C4.5, C5, C5.1, C5.5, C6, C7, C8]"
        ));
    }

    #[test]
    fn step_comment_verification_fails_when_only_two_mandatory_steps_are_present() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-step-comments-unrecognized-tokens-{}",
            run_id
        ));
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
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 996);
                Ok(concat!(
                    "> **[main-orchestrator]** | Cycle 256 | Step 0\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step 5\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step Opening\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step 10.B\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step 10.C\n",
                    "> **[main-orchestrator]** | Cycle 256 | Step Close\n"
                )
                .to_string())
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
        // Previous-cycle backstop is downgraded to Warn/Warning
        assert_eq!(step.status, StepStatus::Warn);
        assert_eq!(step.severity, Severity::Warning);
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
			.contains("missing mandatory [0.5, 0.6, 1, 1.1, 2, 3, 4, 6, 7, 8, 9, C1, C2, C3, C4.1, C4.5, C5, C5.1, C5.5, C6, C7, C8]"));
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
        fs::write(
            root.join("docs/state.json"),
            json!({"last_cycle": {"issue": 834}}).to_string(),
        )
        .unwrap();

        struct StepCommentRunner;

        impl CommandRunner for StepCommentRunner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
                panic!("gh api should not run when previous_cycle_issue is missing");
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.severity, Severity::Blocking);
        assert_eq!(step.detail.as_deref(), Some("skipping step comment verification: /previous_cycle_issue is not set in docs/state.json yet"));
    }

    #[test]
    fn step_comment_verification_skips_for_cycle_zero() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-step-comments-cycle-zero-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({"previous_cycle_issue": 834}).to_string(),
        )
        .unwrap();

        struct StepCommentRunner;

        impl CommandRunner for StepCommentRunner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
                panic!("gh api should not run when cycle 0 has no previous cycle");
            }
        }

        let step = verify_step_comments(&root, 0, &StepCommentRunner);
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.severity, Severity::Blocking);
        assert_eq!(
            step.detail.as_deref(),
            Some("skipping step comment verification: cycle 0 has no previous cycle")
        );
    }

    #[test]
    fn step_comment_verification_errors_when_gh_api_fails() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-step-comments-gh-failure-{}",
            run_id
        ));
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
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 836);
                Err("gh api failed with status 1: rate limited".to_string())
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
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
        let root =
            std::env::temp_dir().join(format!("pipeline-check-step-comments-pre-c51-{}", run_id));
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
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 839);
                let steps = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "C5.1")
                    .collect::<Vec<_>>();
                Ok(step_comment_bodies(253, &steps))
            }
        }

        let step = verify_step_comments(&root, 254, &StepCommentRunner);
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
    fn step_comment_verification_warns_for_previous_cycle_c5_1_gap_from_cycle_256_onward() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-step-comments-post-c51-{}", run_id));
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
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 840);
                let steps = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "C5.1")
                    .collect::<Vec<_>>();
                Ok(step_comment_bodies(256, &steps))
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("Cascade from cycle 256: step C5.1 was missing (already penalized)"));
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("missing mandatory [C5.1]"));
    }

    #[test]
    fn step_comment_verification_warns_for_previous_cycle_zero_mandatory_gap() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-step-comments-always-mandatory-{}",
            run_id
        ));
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
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 841);
                let steps = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "C1")
                    .collect::<Vec<_>>();
                Ok(step_comment_bodies(253, &steps))
            }
        }

        let step = verify_step_comments(&root, 254, &StepCommentRunner);
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("Cascade from cycle 253: step C1 was missing (already penalized)"));
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("missing mandatory [C1]"));
    }

    #[test]
    fn step_comment_verification_warns_when_previous_cycle_mandatory_step_is_missing_above_threshold(
    ) {
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
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 835);
                let steps = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "C1" && *step != "C8")
                    .collect::<Vec<_>>();
                Ok(step_comment_bodies(256, &steps))
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("Cascade from cycle 256: steps C1, C8 were missing (already penalized)"));
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("found 25 unique step comments"));
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("missing mandatory [C1, C8]"));
    }

    #[test]
    fn step_comment_verification_warns_when_previous_cycle_phase_one_mandatory_steps_are_missing() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-step-comments-phase-one-mandatory-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 843,
                "last_cycle": {
                    "number": 257
                }
            })
            .to_string(),
        )
        .unwrap();

        struct StepCommentRunner;

        impl CommandRunner for StepCommentRunner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 843);
                let steps = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "0.6" && *step != "1.1")
                    .collect::<Vec<_>>();
                Ok(step_comment_bodies(256, &steps))
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("Cascade from cycle 256: steps 0.6, 1.1 were missing (already penalized)"));
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("missing mandatory [0.6, 1.1]"));
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("missing optional [none]"));
    }

    #[test]
    fn step_comment_verification_warns_when_only_optional_steps_are_missing() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-step-comments-optional-{}", run_id));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 844,
                "last_cycle": {
                    "number": 257
                }
            })
            .to_string(),
        )
        .unwrap();

        struct StepCommentRunner;

        impl CommandRunner for StepCommentRunner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 844);
                let steps = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "0.1" && *step != "10" && *step != "C5.6")
                    .collect::<Vec<_>>();
                Ok(step_comment_bodies(256, &steps))
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
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
            .contains("missing optional [0.1, 10, C5.6]"));
    }

    #[test]
    fn step_comment_verification_warns_when_previous_cycle_closeout_step_c4_5_is_missing() {
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
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 837);
                let steps = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "C4.5")
                    .collect::<Vec<_>>();
                Ok(step_comment_bodies(256, &steps))
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("Cascade from cycle 256: step C4.5 was missing (already penalized)"));
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("found 26 unique step comments"));
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
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 838);
                Ok(step_comment_bodies(256, &EXPECTED_STEP_IDS))
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("found 27 unique step comments"));
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
    fn step_comment_verification_acknowledges_matching_previous_cycle_gap() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-step-comments-acknowledged-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 846,
                "last_cycle": {
                    "number": 257
                },
                "step_comment_acknowledged_gaps": [
                    {
                        "cycle": 256,
                        "issue": 846,
                        "missing_steps": ["C4.5"],
                        "acknowledged_at": "2026-03-20T14:00:00Z",
                        "reason": "inherited close-out timeout"
                    }
                ]
            })
            .to_string(),
        )
        .unwrap();

        struct StepCommentRunner;

        impl CommandRunner for StepCommentRunner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 846);
                let steps = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "C4.5")
                    .collect::<Vec<_>>();
                Ok(step_comment_bodies(256, &steps))
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.severity, Severity::Blocking);
        assert_eq!(step.findings, Some(27));
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("found 27 unique step comments (1 acknowledged)"));
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("1 step(s) acknowledged via gap record [C4.5]"));
    }

    #[test]
    fn step_comment_verification_ignores_non_matching_acknowledged_gap_cycles() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-step-comments-acknowledged-ignore-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 847,
                "last_cycle": {
                    "number": 257
                },
                "step_comment_acknowledged_gaps": [
                    {
                        "cycle": 255,
                        "issue": 847,
                        "missing_steps": ["C4.5"],
                        "acknowledged_at": "2026-03-20T14:00:00Z",
                        "reason": "wrong cycle"
                    }
                ]
            })
            .to_string(),
        )
        .unwrap();

        struct StepCommentRunner;

        impl CommandRunner for StepCommentRunner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 847);
                let steps = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "C4.5")
                    .collect::<Vec<_>>();
                Ok(step_comment_bodies(256, &steps))
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("missing mandatory [C4.5]"));
        assert!(!step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("acknowledged via gap record"));
    }

    #[test]
    fn step_comment_verification_empty_acknowledged_gap_list_keeps_existing_behavior() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-step-comments-acknowledged-empty-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 848,
                "last_cycle": {
                    "number": 257
                },
                "step_comment_acknowledged_gaps": []
            })
            .to_string(),
        )
        .unwrap();

        struct StepCommentRunner;

        impl CommandRunner for StepCommentRunner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 848);
                let steps = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "C4.5")
                    .collect::<Vec<_>>();
                Ok(step_comment_bodies(256, &steps))
            }
        }

        let step = verify_step_comments(&root, 257, &StepCommentRunner);
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("missing mandatory [C4.5]"));
        assert!(!step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("acknowledged via gap record"));
    }

    #[test]
    fn step_comment_verification_filters_previous_cycle_comments_by_cycle_number() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-step-comments-cycle-filter-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 845,
                "last_cycle": {
                    "number": 316
                }
            })
            .to_string(),
        )
        .unwrap();

        struct StepCommentRunner;

        impl CommandRunner for StepCommentRunner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected in step comment verification test");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 845);
                let previous_cycle_steps = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "0")
                    .collect::<Vec<_>>();
                let mut bodies = step_comment_bodies(315, &previous_cycle_steps);
                bodies.push_str("> **[main-orchestrator]** | Cycle 316 | Step 0\n");
                Ok(bodies)
            }
        }

        let step = verify_step_comments(&root, 316, &StepCommentRunner);
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("Cascade from cycle 315: step 0 was missing (already penalized)"));
    }

    #[test]
    fn assess_step_comment_completeness_warns_for_previous_cycle_mandatory_gap() {
        let found = EXPECTED_STEP_IDS
            .iter()
            .copied()
            .filter(|step| *step != "1.1")
            .collect::<BTreeSet<_>>();

        let assessment =
            assess_step_comment_completeness(&found, 257, StepCommentCheckScope::PreviousCycle);

        assert_eq!(assessment.status, StepStatus::Fail);
        assert_eq!(assessment.severity, Severity::Blocking);
        assert!(assessment
            .detail
            .contains("Cascade from cycle 257: step 1.1 was missing (already penalized)"));
        assert!(assessment.detail.contains("missing mandatory [1.1]"));
    }

    #[test]
    fn assess_step_comment_completeness_fails_for_current_cycle_mandatory_gap() {
        let found = EXPECTED_STEP_IDS
            .iter()
            .copied()
            .filter(|step| *step != "1.1")
            .collect::<BTreeSet<_>>();

        let assessment =
            assess_step_comment_completeness(&found, 257, StepCommentCheckScope::CurrentCycle);

        assert_eq!(assessment.status, StepStatus::Fail);
        assert_eq!(assessment.severity, Severity::Blocking);
        assert!(!assessment.detail.contains("already penalized"));
        assert!(assessment.detail.contains("missing mandatory [1.1]"));
    }

    #[test]
    fn step_1_1_is_mandatory_from_cycle_zero() {
        assert!(is_mandatory_step_for_cycle("1.1", 0));
        assert!(is_mandatory_step_for_cycle("1.1", 257));
    }

    #[test]
    fn startup_step_constants_keep_manual_and_automated_steps_distinct() {
        assert!(MANDATORY_STEPS.contains(&("5", 0)));
        assert!(MANDATORY_STEPS.contains(&("6", 0)));
        assert!(MANDATORY_STEPS.contains(&("7", 0)));
        assert!(MANDATORY_STEPS.contains(&("8", 0)));

        assert!(EXPECTED_STEP_IDS.contains(&"5"));
        assert!(EXPECTED_STEP_IDS.contains(&"6"));
        assert!(EXPECTED_STEP_IDS.contains(&"7"));
        assert!(EXPECTED_STEP_IDS.contains(&"8"));
    }

    #[test]
    fn current_cycle_steps_passes_when_all_pre_gate_mandatory_steps_present() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-current-cycle-pass-{}", run_id));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "cycle_issues": [950],
                "previous_cycle_issue": 950,
                "last_cycle": {
                    "number": 301,
                    "issue": 950
                }
            })
            .to_string(),
        )
        .unwrap();

        struct Runner;

        impl CommandRunner for Runner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 950);
                // All expected steps including post-gate ones
                let steps: Vec<&str> = EXPECTED_STEP_IDS.to_vec();
                Ok(step_comment_bodies(301, &steps))
            }
        }

        let step = verify_current_cycle_step_comments(&root, 301, &Runner);
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("issue #950"));
    }

    #[test]
    fn current_cycle_steps_uses_cycle_issues_for_two_issue_cycle() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-current-cycle-two-issues-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "cycle_issues": [1551, 1554],
                "previous_cycle_issue": 1551,
                "last_cycle": {
                    "number": 321,
                    "issue": 1554
                }
            })
            .to_string(),
        )
        .unwrap();

        struct Runner;

        impl CommandRunner for Runner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                match issue {
                    1554 => {
                        let steps: Vec<&str> = EXPECTED_STEP_IDS
                            .iter()
                            .copied()
                            .filter(|step| *step != "3")
                            .collect();
                        Ok(step_comment_bodies(321, &steps))
                    }
                    1551 => Ok(step_comment_bodies(321, &["3"])),
                    _ => panic!("unexpected issue {issue}"),
                }
            }
        }

        let step = verify_current_cycle_step_comments(&root, 321, &Runner);
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.severity, Severity::Blocking);
        assert_eq!(step.findings, Some(EXPECTED_STEP_IDS.len()));
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("issues EvaLok/schema-org-json-ld#1551 + EvaLok/schema-org-json-ld#1554"));
    }

    #[test]
    fn current_cycle_steps_fails_when_pre_gate_mandatory_step_missing() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-current-cycle-fail-{}", run_id));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 951,
                "last_cycle": {
                    "number": 301,
                    "issue": 951
                }
            })
            .to_string(),
        )
        .unwrap();

        struct Runner;

        impl CommandRunner for Runner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 951);
                // Missing steps 1.1 and 3 (the exact cycle 299 scenario)
                let steps: Vec<&str> = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "1.1" && *step != "3")
                    .collect();
                Ok(step_comment_bodies(301, &steps))
            }
        }

        let step = verify_current_cycle_step_comments(&root, 301, &Runner);
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        let detail = step.detail.as_deref().unwrap_or_default();
        assert!(
            detail.contains("missing pre-gate mandatory steps [1.1, 3]"),
            "expected missing steps 1.1 and 3, got: {}",
            detail
        );
    }

    #[test]
    fn current_cycle_step_verification_ignores_acknowledged_gap_records() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-current-cycle-acknowledged-ignore-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 952,
                "last_cycle": {
                    "number": 301,
                    "issue": 952
                },
                "step_comment_acknowledged_gaps": [
                    {
                        "cycle": 301,
                        "issue": 952,
                        "missing_steps": ["3"],
                        "acknowledged_at": "2026-03-20T14:00:00Z",
                        "reason": "should not affect current cycle checks"
                    }
                ]
            })
            .to_string(),
        )
        .unwrap();

        struct Runner;

        impl CommandRunner for Runner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 952);
                let steps: Vec<&str> = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| *step != "3")
                    .collect();
                Ok(step_comment_bodies(301, &steps))
            }
        }

        let step = verify_current_cycle_step_comments(&root, 301, &Runner);
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("missing pre-gate mandatory steps [3]"));
    }

    #[test]
    fn current_cycle_steps_merge_resumed_cycle_comments_from_both_issues() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-current-cycle-resumed-{}", run_id));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 1551,
                "last_cycle": {
                    "number": 321,
                    "issue": 1554
                }
            })
            .to_string(),
        )
        .unwrap();

        struct Runner;

        impl CommandRunner for Runner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                match issue {
                    1554 => {
                        let steps: Vec<&str> = EXPECTED_STEP_IDS
                            .iter()
                            .copied()
                            .filter(|step| *step != "3")
                            .collect();
                        Ok(step_comment_bodies(321, &steps))
                    }
                    1551 => Ok(step_comment_bodies(321, &["3"])),
                    _ => panic!("unexpected issue {issue}"),
                }
            }
        }

        let step = verify_current_cycle_step_comments(&root, 321, &Runner);
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.severity, Severity::Blocking);
        assert_eq!(step.findings, Some(EXPECTED_STEP_IDS.len()));
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("issue EvaLok/schema-org-json-ld#1554 + EvaLok/schema-org-json-ld#1551"));
    }

    #[test]
    fn current_cycle_steps_merge_comments_from_all_cycle_issues() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-current-cycle-three-issues-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "cycle_issues": [2001, 2002, 2003],
                "previous_cycle_issue": 2002,
                "last_cycle": {
                    "number": 330,
                    "issue": 2003
                }
            })
            .to_string(),
        )
        .unwrap();

        struct Runner;

        impl CommandRunner for Runner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                match issue {
                    2001 => Ok(step_comment_bodies(
                        330,
                        &["0", "0.5", "0.6", "1", "1.1", "2"],
                    )),
                    2002 => Ok(step_comment_bodies(
                        330,
                        &["3", "4", "5", "6", "7", "8", "9", "C1", "C2", "C3"],
                    )),
                    2003 => Ok(step_comment_bodies(330, &["C4.1", "C4.5", "C5", "C5.1"])),
                    _ => panic!("unexpected issue {issue}"),
                }
            }
        }

        let step = verify_current_cycle_step_comments(&root, 330, &Runner);
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
			.detail
			.as_deref()
			.unwrap_or_default()
			.contains("issues EvaLok/schema-org-json-ld#2001 + EvaLok/schema-org-json-ld#2002 + EvaLok/schema-org-json-ld#2003"));
    }

    #[test]
    fn current_cycle_steps_rejects_mandatory_steps_from_other_cycles() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-current-cycle-wrong-cycle-{}",
            run_id
        ));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 953,
                "last_cycle": {
                    "number": 316,
                    "issue": 953
                }
            })
            .to_string(),
        )
        .unwrap();

        struct Runner;

        impl CommandRunner for Runner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 953);
                Ok(concat!(
                    "> **[main-orchestrator]** | Cycle 315 | Step 0\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step 0.5\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step 0.6\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step 1\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step 1.1\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step 2\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step 3\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step 4\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step 5\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step 6\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step 7\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step 8\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step 9\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step C1\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step C2\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step C3\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step C4.1\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step C4.5\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step C5\n",
                    "> **[main-orchestrator]** | Cycle 316 | Step C5.1\n"
                )
                .to_string())
            }
        }

        let step = verify_current_cycle_step_comments(&root, 316, &Runner);
        assert_eq!(step.status, StepStatus::Fail);
        assert_eq!(step.severity, Severity::Blocking);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("missing pre-gate mandatory steps [0]"));
    }

    #[test]
    fn current_cycle_steps_ignores_missing_post_gate_steps() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-current-cycle-post-gate-{}", run_id));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 952,
                "last_cycle": {
                    "number": 301,
                    "issue": 952
                }
            })
            .to_string(),
        )
        .unwrap();

        struct Runner;

        impl CommandRunner for Runner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected");
            }

            fn fetch_issue_comment_bodies(&self, issue: u64) -> Result<String, String> {
                assert_eq!(issue, 952);
                // All pre-gate steps present, but post-gate steps (C5.5, C5.6, C6, C7, C8) missing
                let steps: Vec<&str> = EXPECTED_STEP_IDS
                    .iter()
                    .copied()
                    .filter(|step| !POST_GATE_STEP_IDS.contains(step))
                    .collect();
                Ok(step_comment_bodies(301, &steps))
            }
        }

        let step = verify_current_cycle_step_comments(&root, 301, &Runner);
        assert_eq!(step.status, StepStatus::Pass);
        assert_eq!(step.severity, Severity::Blocking);
    }

    #[test]
    fn current_cycle_steps_skips_when_last_cycle_issue_missing() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root =
            std::env::temp_dir().join(format!("pipeline-check-current-cycle-skip-{}", run_id));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "previous_cycle_issue": 900,
                "last_cycle": {
                    "number": 301
                }
            })
            .to_string(),
        )
        .unwrap();

        struct Runner;

        impl CommandRunner for Runner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("tool wrapper execution not expected");
            }

            fn fetch_issue_comment_bodies(&self, _issue: u64) -> Result<String, String> {
                panic!("gh api should not run when last_cycle.issue is missing");
            }
        }

        let step = verify_current_cycle_step_comments(&root, 301, &Runner);
        assert_eq!(step.status, StepStatus::Pass);
        assert!(step
            .detail
            .as_deref()
            .unwrap_or_default()
            .contains("skipping current-cycle step verification"));
    }

    #[test]
    fn artifact_verification_passes_when_journal_exists_for_today() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-artifacts-current-journal-{}",
            run_id
        ));
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
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-artifacts-missing-journal-dir-{}",
            run_id
        ));
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
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-artifacts-non-dated-journal-{}",
            run_id
        ));
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
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-artifacts-mixed-journal-files-{}",
            run_id
        ));
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
