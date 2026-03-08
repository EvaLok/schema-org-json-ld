use chrono::Utc;
use clap::Parser;
use serde::Serialize;
use serde_json::Value;
use state_schema::{current_cycle_from_state, read_state_value};
use std::path::{Path, PathBuf};
use std::process::Command;

const HOUSEKEEPING_FINDINGS_KEY: &str = "items_needing_attention";
const CYCLE_STATUS_IN_FLIGHT_PATH: &str = "/concurrency/in_flight";
const CYCLE_STATUS_DIRECTIVES_PATH: &str = "/eva_input/comments_since_last_cycle";
const DERIVE_METRICS_TOOL_NAME: &str = "derive-metrics";
const DERIVE_METRICS_WRAPPER_PATH: &str = "tools/derive-metrics";
const DERIVE_METRICS_FIELDS: [&str; 7] = [
    "total_dispatches",
    "resolved",
    "merged",
    "in_flight",
    "produced_pr",
    "closed_without_pr",
    "reviewed_awaiting_eva",
];

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

    let steps = specs
        .iter()
        .map(|spec| run_step(repo_root, spec, runner))
        .collect::<Vec<_>>();
    let overall = pipeline_overall_status(&steps);
    let has_blocking_findings = steps.iter().any(|step| step.status == StepStatus::Fail);

    PipelineReport {
        cycle,
        overall,
        has_blocking_findings,
        timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
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
        ToolKind::DeriveMetrics => {
            classify_derive_metrics_step(repo_root, spec.display_name, execution)
        }
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
        ToolKind::DeriveMetrics => {
            unreachable!("derive-metrics classification is handled separately")
        }
    }

    step
}

fn classify_derive_metrics_step(
    repo_root: &Path,
    name: &'static str,
    execution: ExecutionResult,
) -> StepReport {
    let mut step = StepReport {
        name,
        status: StepStatus::Pass,
        severity: Severity::Warning,
        exit_code: execution.exit_code,
        detail: None,
        findings: None,
        summary: None,
    };

    if execution.exit_code != Some(0) {
        step.status = StepStatus::Error;
        step.detail = Some(format!(
            "{} exited with unexpected status {:?}",
            name, execution.exit_code
        ));
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
            step.status = StepStatus::Warn;
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
        ToolKind::MetricSnapshot | ToolKind::StateInvariants | ToolKind::CycleStatus => {
            Severity::Blocking
        }
        ToolKind::FieldInventory | ToolKind::HousekeepingScan | ToolKind::DeriveMetrics => {
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

fn collect_derive_metrics_mismatches(
    repo_root: &Path,
    derived_metrics: &Value,
) -> Result<Vec<String>, String> {
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
        let suffix = if warning_count == 1 {
            "warning"
        } else {
            "warnings"
        };
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

    fn repo_root() -> PathBuf {
        PathBuf::from("/repo")
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
                    "reviewed_awaiting_eva": 1
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
                        "reviewed_awaiting_eva": 1
                    })
                    .to_string(),
                })
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
        assert_eq!(
            step.detail.as_deref(),
            Some("tracked copilot_metrics fields match")
        );
    }

    #[test]
    fn derive_metrics_is_warn_when_tracked_fields_diverge() {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("pipeline-check-derive-warn-{}", run_id));
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
                    "reviewed_awaiting_eva": 0
                }
            })
            .to_string(),
        )
        .unwrap();

        struct DeriveMetricsRunner;

        impl CommandRunner for DeriveMetricsRunner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                Ok(ExecutionResult {
                    exit_code: Some(0),
                    stdout: json!({
                        "total_dispatches": 5,
                        "resolved": 3,
                        "merged": 1,
                        "in_flight": 2,
                        "produced_pr": 2,
                        "closed_without_pr": 1,
                        "reviewed_awaiting_eva": 0
                    })
                    .to_string(),
                })
            }
        }

        let spec = ToolSpec {
            display_name: DERIVE_METRICS_TOOL_NAME,
            wrapper_relative_path: DERIVE_METRICS_WRAPPER_PATH,
            args: vec![],
            kind: ToolKind::DeriveMetrics,
        };
        let step = run_step(&root, &spec, &DeriveMetricsRunner);
        assert_eq!(
            severity_for_kind(&ToolKind::DeriveMetrics),
            Severity::Warning
        );
        assert_eq!(step.status, StepStatus::Warn);
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
        }

        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("pipeline-check-test-{}", run_id));
        fs::create_dir_all(root.join("docs")).unwrap();
        fs::write(
            root.join("docs/state.json"),
            json!({
                "copilot_metrics": {
                    "total_dispatches": 3,
                    "resolved": 2,
                    "merged": 1,
                    "in_flight": 1,
                    "produced_pr": 2,
                    "closed_without_pr": 0,
                    "reviewed_awaiting_eva": 1
                }
            })
            .to_string(),
        )
        .unwrap();

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
                            "reviewed_awaiting_eva": 1
                        })
                        .to_string(),
                    },
                ),
            ]),
        };

        let report = run_pipeline(&root, 135, &runner);
        assert_eq!(report.overall, StepStatus::Pass);
        assert_eq!(report.steps.len(), 6);
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
            fn run(&self, script_path: &Path, _args: &[String]) -> Result<ExecutionResult, String> {
                Err(format!("failed to invoke {}", script_path.display()))
            }
        }

        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("pipeline-check-fail-all-errors-{}", run_id));
        fs::create_dir_all(&root).unwrap();

        let report = run_pipeline(&root, 140, &ErrorRunner);
        assert_eq!(report.overall, StepStatus::Fail);
        assert_eq!(report.steps.len(), 6);
        assert!(report
            .steps
            .iter()
            .all(|step| matches!(step.status, StepStatus::Error)));
    }
}
