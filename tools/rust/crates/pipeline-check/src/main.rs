use chrono::Utc;
use clap::Parser;
use serde::Serialize;
use serde_json::Value;
use state_schema::current_cycle_from_state;
use std::path::{Path, PathBuf};
use std::process::Command;

const HOUSEKEEPING_FINDINGS_KEY: &str = "items_needing_attention";
const CYCLE_STATUS_IN_FLIGHT_PATH: &str = "/concurrency/in_flight";
const CYCLE_STATUS_DIRECTIVES_PATH: &str = "/eva_input/comments_since_last_cycle";

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
    Fail,
    Info,
    Skip,
    Error,
}

#[derive(Serialize)]
struct PipelineReport {
    cycle: u64,
    overall: StepStatus,
    timestamp: String,
    steps: Vec<StepReport>,
}

#[derive(Serialize)]
struct StepReport {
    name: &'static str,
    status: StepStatus,
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
    binary_relative_path: &'static str,
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
    let specs = vec![
        ToolSpec {
            display_name: "metric-snapshot",
            wrapper_relative_path: "tools/metric-snapshot",
            binary_relative_path: "tools/rust/target/release/metric-snapshot",
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
            binary_relative_path: "tools/rust/target/release/check-field-inventory",
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
            binary_relative_path: "tools/rust/target/release/housekeeping-scan",
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
            binary_relative_path: "tools/rust/target/release/cycle-status",
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
            binary_relative_path: "tools/rust/target/release/state-invariants",
            args: vec![
                "--json".to_string(),
                "--repo-root".to_string(),
                repo_root.display().to_string(),
            ],
            kind: ToolKind::StateInvariants,
        },
    ];

    let steps = specs
        .iter()
        .map(|spec| run_step(repo_root, spec, runner))
        .collect::<Vec<_>>();
    let has_fail_or_error = steps
        .iter()
        .any(|step| matches!(step.status, StepStatus::Fail | StepStatus::Error));
    let all_skipped = is_all_skipped(&steps);
    let overall = if has_fail_or_error || all_skipped {
        StepStatus::Fail
    } else {
        StepStatus::Pass
    };

    PipelineReport {
        cycle,
        overall,
        timestamp: Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string(),
        steps,
    }
}

fn run_step(repo_root: &Path, spec: &ToolSpec, runner: &dyn CommandRunner) -> StepReport {
    let binary_path = repo_root.join(spec.binary_relative_path);
    if !binary_path.exists() {
        return StepReport {
            name: spec.display_name,
            status: StepStatus::Skip,
            exit_code: None,
            detail: Some(format!("binary not found at {}", binary_path.display())),
            findings: None,
            summary: None,
        };
    }

    let script_path = repo_root.join(spec.wrapper_relative_path);
    let execution = match runner.run(&script_path, &spec.args) {
        Ok(output) => output,
        Err(err) => {
            return StepReport {
                name: spec.display_name,
                status: StepStatus::Error,
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
    let mut step = StepReport {
        name,
        status: StepStatus::Pass,
        exit_code: execution.exit_code,
        detail: None,
        findings: None,
        summary: None,
    };

    match kind {
        ToolKind::FieldInventory => {
            step.status = match execution.exit_code {
                Some(0) => StepStatus::Pass,
                Some(1) => StepStatus::Fail,
                _ => StepStatus::Error,
            };
            if !execution.stdout.is_empty() {
                step.detail = Some(execution.stdout);
            }
        }
        ToolKind::MetricSnapshot => {
            if let Some(parsed) = parse_json(&execution.stdout) {
                step.status = match execution.exit_code {
                    Some(0) => StepStatus::Pass,
                    Some(1) => StepStatus::Fail,
                    _ => StepStatus::Error,
                };
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
                step.status = match execution.exit_code {
                    Some(0) => StepStatus::Pass,
                    Some(1) => StepStatus::Fail,
                    _ => StepStatus::Error,
                };
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
                step.status = match execution.exit_code {
                    Some(0) => StepStatus::Pass,
                    Some(1) => StepStatus::Fail,
                    _ => StepStatus::Error,
                };
                let passed = parsed
                    .get("passed")
                    .and_then(Value::as_u64)
                    .unwrap_or(0);
                let failed = parsed
                    .get("failed")
                    .and_then(Value::as_u64)
                    .unwrap_or(0);
                step.detail = Some(format!("{}/{} invariants pass", passed, passed + failed));
            } else {
                step.status = StepStatus::Error;
                step.detail = Some(format!("invalid JSON output from {}", name));
            }
        }
        ToolKind::CycleStatus => {
            if let Some(parsed) = parse_json(&execution.stdout) {
                step.status = match execution.exit_code {
                    Some(0) => StepStatus::Info,
                    Some(1) => StepStatus::Fail,
                    _ => StepStatus::Error,
                };
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

fn parse_json(raw: &str) -> Option<Value> {
    serde_json::from_str(raw).ok()
}

fn is_all_skipped(steps: &[StepReport]) -> bool {
    !steps.is_empty()
        && steps
            .iter()
            .all(|step| matches!(step.status, StepStatus::Skip))
}

fn is_check_passing(check: &Value) -> bool {
    check.get("pass").and_then(Value::as_bool).unwrap_or(false)
}

fn pipeline_exit_code(steps: &[StepReport]) -> i32 {
    if steps.iter().any(|step| step.status == StepStatus::Error) {
        2
    } else if steps.iter().any(|step| step.status == StepStatus::Fail) || is_all_skipped(steps) {
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
                "  {}. {:<19} {:<4}",
                index + 1,
                format!("{}:", step.name),
                step_status_label(step.status)
            );
        } else {
            println!(
                "  {}. {:<19} {:<4} ({})",
                index + 1,
                format!("{}:", step.name),
                step_status_label(step.status),
                summary
            );
        }
    }

    println!();
    println!("Overall: {}", step_status_label(report.overall));
    if is_all_skipped(&report.steps) {
        println!("Reason: no tools could run (all steps skipped)");
    }
}

fn step_status_label(status: StepStatus) -> &'static str {
    match status {
        StepStatus::Pass => "PASS",
        StepStatus::Fail => "FAIL",
        StepStatus::Info => "INFO",
        StepStatus::Skip => "SKIP",
        StepStatus::Error => "ERROR",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::collections::HashMap;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn repo_root() -> PathBuf {
        PathBuf::from("/repo")
    }

    #[test]
    fn cycle_status_is_info_when_command_succeeds() {
        let execution = ExecutionResult {
            exit_code: Some(0),
            stdout: json!({
                "concurrency": { "in_flight": 1 },
                "eva_input": { "comments_since_last_cycle": [{"x":1}, {"x":2}] }
            })
            .to_string(),
        };
        let step = classify_step("cycle-status", &ToolKind::CycleStatus, execution);
        assert_eq!(step.status, StepStatus::Info);
        assert_eq!(
            step.summary.as_deref(),
            Some("1 in-flight, 2 eva directives")
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
            overall: StepStatus::Pass,
            timestamp: "2026-01-01T00:00:00Z".to_string(),
            steps: vec![
                StepReport {
                    name: "metric-snapshot",
                    status: StepStatus::Pass,
                    exit_code: Some(0),
                    detail: None,
                    findings: None,
                    summary: None,
                },
                StepReport {
                    name: "field-inventory",
                    status: StepStatus::Fail,
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
    fn all_skipped_steps_return_failure_exit_code() {
        let steps = vec![
            StepReport {
                name: "metric-snapshot",
                status: StepStatus::Skip,
                exit_code: None,
                detail: None,
                findings: None,
                summary: None,
            },
            StepReport {
                name: "cycle-status",
                status: StepStatus::Skip,
                exit_code: None,
                detail: None,
                findings: None,
                summary: None,
            },
        ];
        assert_eq!(pipeline_exit_code(&steps), 1);
    }

    #[test]
    fn run_step_skips_when_binary_is_missing() {
        struct NoopRunner;
        impl CommandRunner for NoopRunner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("runner should not execute for missing binaries");
            }
        }

        let spec = ToolSpec {
            display_name: "metric-snapshot",
            wrapper_relative_path: "tools/metric-snapshot",
            binary_relative_path: "tools/rust/target/release/metric-snapshot",
            args: vec![],
            kind: ToolKind::MetricSnapshot,
        };
        let step = run_step(&repo_root(), &spec, &NoopRunner);
        assert_eq!(step.status, StepStatus::Skip);
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
                    "housekeeping-scan" | "cycle-status" | "state-invariants" => {
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
        fs::create_dir_all(root.join("tools/rust/target/release")).unwrap();
        fs::write(root.join("tools/rust/target/release/metric-snapshot"), "").unwrap();
        fs::write(
            root.join("tools/rust/target/release/check-field-inventory"),
            "",
        )
        .unwrap();
        fs::write(root.join("tools/rust/target/release/housekeeping-scan"), "").unwrap();
        fs::write(root.join("tools/rust/target/release/cycle-status"), "").unwrap();
        fs::write(root.join("tools/rust/target/release/state-invariants"), "").unwrap();

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
            ]),
        };

        let report = run_pipeline(&root, 135, &runner);
        assert_eq!(report.overall, StepStatus::Pass);
        assert_eq!(report.steps.len(), 5);
        assert_eq!(report.steps[0].status, StepStatus::Pass);
        assert_eq!(report.steps[1].status, StepStatus::Pass);
        assert_eq!(report.steps[2].status, StepStatus::Pass);
        assert_eq!(report.steps[3].status, StepStatus::Info);
        assert_eq!(
            report.steps[3].summary.as_deref(),
            Some("1 in-flight, 2 eva directives")
        );
        assert_eq!(report.steps[4].status, StepStatus::Pass);
        assert_eq!(
            report.steps[4].detail.as_deref(),
            Some("5/5 invariants pass")
        );
    }

    #[test]
    fn cli_accepts_missing_cycle_argument() {
        let cli = Cli::try_parse_from(["pipeline-check", "--repo-root", "."]).unwrap();
        assert_eq!(cli.repo_root, PathBuf::from("."));
        assert_eq!(cli.cycle, None);
    }

    #[test]
    fn run_pipeline_fails_when_all_steps_are_skipped() {
        struct NoopRunner;

        impl CommandRunner for NoopRunner {
            fn run(
                &self,
                _script_path: &Path,
                _args: &[String],
            ) -> Result<ExecutionResult, String> {
                panic!("runner should not execute when all binaries are missing");
            }
        }

        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!("pipeline-check-all-skipped-{}", run_id));
        fs::create_dir_all(&root).unwrap();

        let report = run_pipeline(&root, 140, &NoopRunner);
        assert_eq!(report.overall, StepStatus::Fail);
        assert_eq!(report.steps.len(), 5);
        assert!(report
            .steps
            .iter()
            .all(|step| matches!(step.status, StepStatus::Skip)));
    }
}
