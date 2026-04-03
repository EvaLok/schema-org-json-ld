use crate::git;
use crate::review_body;
use crate::runner;
use crate::steps;
use chrono::{DateTime, FixedOffset, Utc};
use serde_json::Value;
use state_schema::{
    commit_state_json, current_cycle_from_state, read_state_value, transition_cycle_phase,
    write_state_value, AgentSession, StateJson,
};
use std::fs;
use std::path::Path;
use std::process::Command;

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const VERIFY_REVIEW_EVENTS_TIMEOUT_SECS: u64 = 30;
const PATCH_PIPELINE_ACTIVITY_TIMESTAMP_FIELDS: [&str; 5] = [
    "closed_at",
    "resolved_at",
    "completed_at",
    "status_changed_at",
    "updated_at",
];

struct ReviewInfo {
    issue_number: u64,
    issue_url: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PipelineGateReport {
    overall: String,
    has_blocking_findings: bool,
    warning_count: usize,
    blocking_warning_count: usize,
    cascade_count: usize,
    blocking_steps: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RecordedC5_5Pass {
    pipeline_summary: String,
}

pub fn run(
    repo_root: &Path,
    issue: u64,
    cycle_override: Option<u64>,
    dry_run: bool,
) -> Result<(), String> {
    let state = read_state_value(repo_root)?;
    let cycle = match cycle_override {
        Some(c) => c,
        None => current_cycle_from_state(repo_root)?,
    };

    let phase = state
        .pointer("/cycle_phase/phase")
        .and_then(Value::as_str)
        .unwrap_or("unknown");

    if dry_run {
        print_dry_run(cycle, issue);
        return Ok(());
    }

    if phase != "close_out" && phase != "complete" {
        return Err(format!(
            "Expected cycle_phase.phase to be 'close_out' or 'complete', got '{}'. \
             Run cycle-complete first.",
            phase,
        ));
    }

    let worklog = review_body::find_worklog_for_cycle(repo_root, cycle)?;
    let journal = review_body::find_latest_journal(repo_root)?;

    eprintln!("Close-out for cycle {} (issue #{})", cycle, issue);
    eprintln!("Worklog: {}", worklog.display());
    eprintln!("Journal: {}", journal.display());

    let prior_gate_failures = detect_prior_gate_failures(repo_root, issue);

    // C4.1: Validate docs — GATE
    step_c4_1(repo_root, issue, cycle, &worklog, &journal)?;

    // C4.5: ADR check
    step_c4_5(repo_root, issue)?;

    // C4.7: Verify review events (best-effort, non-blocking)
    if let Err(warn) = step_c4_7(repo_root, issue) {
        eprintln!("C4.7 warning: {}", warn);
    }

    // C5: Commit and push docs
    step_c5(repo_root, issue, cycle, &worklog)?;

    // C5.1: Receipt validation (report only)
    step_c5_1(repo_root, issue, cycle, &worklog)?;

    // C5.5: Pipeline check — GATE
    let (pipeline_passed, pipeline_summary) = step_c5_5(repo_root, issue, cycle)?;

    // C5.6: Stabilization counter
    step_c5_6(repo_root, issue, cycle, pipeline_passed)?;

    ensure_c5_5_allows_c6(repo_root, cycle)?;

    // C6: Review dispatch (may be skipped if Copilot is unavailable)
    let review_info = step_c6(repo_root, issue, cycle)?;

    // C6.5: Refresh worklog state after review dispatch
    step_c6_5(
        repo_root,
        issue,
        cycle,
        &worklog,
        &pipeline_summary,
        &prior_gate_failures,
    )?;

    // C7: Push
    step_c7(repo_root, issue)?;

    // C8: Close issue
    step_c8(
        repo_root,
        issue,
        cycle,
        review_info.as_ref(),
        &pipeline_summary,
    )?;
    complete_close_out_phase(repo_root, cycle)?;
    git::push(repo_root).map_err(|error| {
        format!(
            "{} (cycle phase was already committed locally; retry the push to publish the complete state)",
            error
        )
    })?;

    eprintln!("Close-out complete for cycle {}", cycle);
    Ok(())
}

fn detect_prior_gate_failures(repo_root: &Path, issue: u64) -> Vec<String> {
    let comments_path = format!("repos/{}/issues/{}/comments", MAIN_REPO, issue);
    let output = match Command::new("gh")
        .current_dir(repo_root)
        .args(["api", &comments_path, "--paginate", "--jq", ".[].body"])
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            eprintln!("Warning: unable to query prior gate failures from issue comments: {error}");
            return Vec::new();
        }
    };

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        eprintln!("Warning: unable to query prior gate failures from issue comments: {stderr}");
        return Vec::new();
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let comment_bodies = stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| match serde_json::from_str::<String>(line) {
            Ok(body) => body,
            Err(error) => {
                eprintln!(
                    "Warning: unable to parse gh api comment body as JSON string, using raw output: {error}"
                );
                line.trim().to_string()
            }
        })
        .collect::<Vec<_>>();

    parse_prior_gate_failures_from_comment_bodies(comment_bodies.iter().map(String::as_str))
}

fn parse_prior_gate_failures_from_comment_bodies<'a>(
    bodies: impl IntoIterator<Item = &'a str>,
) -> Vec<String> {
    let mut failures = Vec::new();
    for body in bodies {
        if let Some(failure) = parse_prior_gate_failure_from_comment_body(body) {
            if !failures.contains(&failure) {
                failures.push(failure);
            }
        }
    }
    failures
}

fn parse_prior_gate_failure_from_comment_body(body: &str) -> Option<String> {
    if body.contains("### Step C4.1") {
        return parse_c4_1_gate_failure(body).map(|reason| format!("C4.1 FAIL: {reason}"));
    }
    if body.contains("### Step C5.5") {
        return parse_c5_5_gate_failure(body).map(|reason| format!("C5.5 FAIL: {reason}"));
    }
    None
}

fn parse_c4_1_gate_failure(body: &str) -> Option<String> {
    let worklog_failure = body
        .lines()
        .find_map(|line| line.trim().strip_prefix("Worklog validation: FAIL:"))
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned);
    let journal_failure = body
        .lines()
        .find_map(|line| line.trim().strip_prefix("Journal validation: FAIL:"))
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(ToOwned::to_owned);

    match (worklog_failure, journal_failure) {
        (Some(worklog), None) => Some(worklog),
        (None, Some(journal)) => Some(journal),
        (Some(worklog), Some(journal)) => Some(format_combined_c4_1_gate_failure(&worklog, &journal)),
        (None, None) => None,
    }
}

fn format_combined_c4_1_gate_failure(worklog: &str, journal: &str) -> String {
    format!("worklog: {worklog}; journal: {journal}")
}

fn parse_c5_5_gate_failure(body: &str) -> Option<String> {
    if let Some(reason) = body
        .lines()
        .find_map(|line| line.trim().strip_prefix("- gate_failure_reason:"))
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        return Some(reason.to_string());
    }

    body.lines()
        .find_map(|line| line.trim().strip_prefix("Pipeline: FAIL"))
        .map(str::trim)
        .map(|detail| {
            detail
                .strip_prefix('(')
                .and_then(|value| value.strip_suffix(')'))
                .unwrap_or(detail)
                .trim()
                .to_string()
        })
        .filter(|detail| !detail.is_empty())
}

fn complete_close_out_phase(repo_root: &Path, cycle: u64) -> Result<(), String> {
    let mut state = read_state_value(repo_root)?;
    transition_cycle_phase(&mut state, cycle, "complete")?;
    write_state_value(repo_root, &state)?;

    let commit_message = format!(
        "state(cycle-complete-phase): cycle {} phase -> complete [cycle {}]",
        cycle, cycle
    );
    commit_state_json(repo_root, &commit_message)?;
    Ok(())
}

fn step_c4_1(
    repo_root: &Path,
    issue: u64,
    cycle: u64,
    worklog: &Path,
    journal: &Path,
) -> Result<(), String> {
    eprintln!("C4.1: Validating documentation...");

    let worklog_str = worklog.to_string_lossy().to_string();
    let cycle_str = cycle.to_string();

    let worklog_output = runner::run_tool(
        repo_root,
        "validate-docs",
        &["worklog", "--file", &worklog_str, "--cycle", &cycle_str],
    )?;
    let worklog_ok = worklog_output.status.success();
    let worklog_detail = if worklog_ok {
        "PASS".to_string()
    } else {
        format!("FAIL: {}", runner::stdout_text(&worklog_output))
    };

    let journal_str = journal.to_string_lossy().to_string();
    let journal_output = runner::run_tool(
        repo_root,
        "validate-docs",
        &["journal", "--file", &journal_str],
    )?;
    let journal_ok = journal_output.status.success();
    let journal_detail = if journal_ok {
        "PASS".to_string()
    } else {
        format!("FAIL: {}", runner::stdout_text(&journal_output))
    };

    let body = format!(
        "Worklog validation: {}\nJournal validation: {}",
        worklog_detail, journal_detail
    );
    steps::post_step(
        repo_root,
        issue,
        "C4.1",
        "Documentation validation",
        &body,
        false,
    )?;

    if !worklog_ok || !journal_ok {
        return Err(format!(
            "Documentation validation failed at C4.1 — fix issues and re-run close-out.\n\
             Worklog: {}\nJournal: {}",
            worklog_detail, journal_detail
        ));
    }

    Ok(())
}

fn step_c4_5(repo_root: &Path, issue: u64) -> Result<(), String> {
    eprintln!("C4.5: ADR check...");

    let adr_dir = repo_root.join("doc/adr");
    let mut adrs = match fs::read_dir(&adr_dir) {
        Ok(entries) => entries
            .map(|entry| {
                entry
                    .map_err(|error| {
                        format!("failed to read entry in {}: {}", adr_dir.display(), error)
                    })
                    .and_then(|entry| {
                        let file_type = entry.file_type().map_err(|error| {
                            format!(
                                "failed to read file type for {}: {}",
                                entry.path().display(),
                                error
                            )
                        })?;
                        let path = entry.path();
                        let name = entry.file_name().into_string().map_err(|_| {
                            format!("ADR path is not valid UTF-8: {}", path.display())
                        })?;
                        Ok((name, file_type.is_file()))
                    })
            })
            .collect::<Result<Vec<_>, _>>()?
            .into_iter()
            .filter_map(|(name, is_file)| is_file.then_some(name))
            .collect::<Vec<_>>(),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Vec::new(),
        Err(error) => {
            return Err(format!(
                "failed to scan {} for ADRs: {}",
                adr_dir.display(),
                error
            ));
        }
    };

    adrs.sort();

    let body = if adrs.is_empty() {
        "No ADR-worthy decisions this cycle".to_string()
    } else {
        format!("{} ADRs in doc/adr/:\n- {}", adrs.len(), adrs.join("\n- "))
    };

    steps::post_step(repo_root, issue, "C4.5", "ADR check", &body, false)
}

fn step_c4_7(repo_root: &Path, issue: u64) -> Result<(), String> {
    step_c4_7_with_timeout(repo_root, issue, VERIFY_REVIEW_EVENTS_TIMEOUT_SECS)
}

fn step_c4_7_with_timeout(
    repo_root: &Path,
    issue: u64,
    timeout_seconds: u64,
) -> Result<(), String> {
    eprintln!("C4.7: Verifying review events...");

    let repo_root_str = repo_root.to_string_lossy().to_string();
    let output = match runner::run_tool_with_timeout(
        repo_root,
        "verify-review-events",
        &["--apply", "--repo-root", &repo_root_str],
        timeout_seconds,
    ) {
        Ok(output) => output,
        Err(error) => {
            let body = format!(
                "verify-review-events warning: {}\nNon-blocking; C5.5 will still validate state freshness.",
                error
            );
            steps::post_step(
                repo_root,
                issue,
                "C4.7",
                "Verify review events",
                &body,
                false,
            )?;
            return Err(error);
        }
    };

    let stdout = runner::stdout_text(&output);
    let stderr = runner::stderr_text(&output);
    let safe_to_advance_to = parse_verify_review_events_safe_to_advance_to(&stdout).ok();

    let (body, warning) = if runner::timed_out(&output) {
        (
            format!(
                "verify-review-events warning: timed out after {} seconds\nNon-blocking; C5.5 will still validate state freshness.",
                timeout_seconds
            ),
            Some(format!(
                "verify-review-events timed out after {} seconds",
                timeout_seconds
            )),
        )
    } else if output.status.success() {
        let safe_to_advance_to = parse_verify_review_events_safe_to_advance_to(&stdout)?;
        (
            format!(
                "verify-review-events succeeded\n- safe_to_advance_to: {}\n- state updates applied before C5 commit",
                safe_to_advance_to
            ),
            None,
        )
    } else {
        let mut body = format!(
            "verify-review-events warning: exit_code {}",
            output.status.code().unwrap_or(-1)
        );
        if let Some(value) = safe_to_advance_to {
            body.push_str(&format!("\n- safe_to_advance_to: {}", value));
        }
        if !stderr.is_empty() {
            body.push_str(&format!("\n- stderr: {}", stderr));
        } else if !stdout.is_empty() {
            body.push_str(&format!("\n- stdout: {}", stdout));
        }
        body.push_str("\n- Non-blocking; C5.5 will still validate state freshness.");
        (
            body,
            Some(format!(
                "verify-review-events failed with exit code {}",
                output.status.code().unwrap_or(-1)
            )),
        )
    };

    steps::post_step(
        repo_root,
        issue,
        "C4.7",
        "Verify review events",
        &body,
        false,
    )?;

    if let Some(warning) = warning {
        return Err(warning);
    }

    Ok(())
}

fn parse_verify_review_events_safe_to_advance_to(stdout: &str) -> Result<u64, String> {
    if stdout.is_empty() {
        return Err("verify-review-events produced empty stdout".to_string());
    }

    if let Ok(report) = serde_json::from_str::<Value>(stdout) {
        return report
            .get("safe_to_advance_to")
            .and_then(Value::as_u64)
            .ok_or_else(|| {
                "verify-review-events JSON output is missing numeric safe_to_advance_to".to_string()
            });
    }

    for prefix in ["Safe to advance marker to ", "Marker stays at "] {
        if let Some(value) = stdout.lines().find_map(|line| {
            line.find(prefix).and_then(|index| {
                let remainder = line[index + prefix.len()..].trim_start();
                let token = remainder
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim_end_matches(|ch: char| !ch.is_ascii_digit());
                (!token.is_empty() && token.chars().all(|ch| ch.is_ascii_digit()))
                    .then(|| token.parse::<u64>().ok())
                    .flatten()
            })
        }) {
            return Ok(value);
        }
    }

    Err(format!(
        "unable to extract safe_to_advance_to from verify-review-events output: {}",
        stdout
    ))
}

fn step_c5(repo_root: &Path, issue: u64, cycle: u64, worklog: &Path) -> Result<(), String> {
    eprintln!("C5: Committing and pushing docs...");

    let message = format!(
        "docs(cycle-{}): worklog, journal, and state updates [cycle {}]",
        cycle, cycle
    );
    let sha = git::add_and_commit(repo_root, &["docs/"], &message)?;

    let body = if sha.is_empty() {
        "Docs already committed, skipping commit".to_string()
    } else {
        format!("Committed docs: {}", sha)
    };

    git::push(repo_root)?;

    let worklog_rel = worklog
        .strip_prefix(repo_root)
        .ok()
        .and_then(|path| path.to_str())
        .unwrap_or("worklog");
    let push_body = format!(
        "{}\nPushed to origin/master\nWorklog frozen at C5 commit time: {}",
        body, worklog_rel
    );
    steps::post_step(
        repo_root,
        issue,
        "C5",
        "Pre-dispatch commit and push",
        &push_body,
        false,
    )?;

    Ok(())
}

fn step_c5_1(repo_root: &Path, issue: u64, cycle: u64, worklog: &Path) -> Result<(), String> {
    eprintln!("C5.1: Validating receipts...");

    let cycle_str = cycle.to_string();
    let worklog_str = worklog.to_string_lossy().to_string();

    let output = runner::run_tool(
        repo_root,
        "receipt-validate",
        &["--cycle", &cycle_str, "--worklog", &worklog_str, "--json"],
    )?;

    let body = if output.status.success() {
        let stdout = runner::stdout_text(&output);
        // Extract key info from JSON output
        match serde_json::from_str::<Value>(&stdout) {
            Ok(report) => {
                let result = report
                    .get("result")
                    .and_then(Value::as_str)
                    .unwrap_or("unknown");
                let canonical = report
                    .get("canonical_receipts")
                    .and_then(Value::as_u64)
                    .unwrap_or(0);
                let worklog_count = report
                    .get("worklog_receipts")
                    .and_then(Value::as_u64)
                    .unwrap_or(0);
                let missing = report
                    .get("genuinely_missing")
                    .and_then(Value::as_u64)
                    .unwrap_or(0);
                format!(
                    "Receipt validation: {}\nCanonical: {}, Worklog: {}, Missing: {}",
                    result, canonical, worklog_count, missing
                )
            }
            Err(_) => stdout,
        }
    } else {
        format!(
            "Receipt validation warning: {}",
            runner::stderr_text(&output)
        )
    };

    // Receipt validation is report-only, not a gate
    steps::post_step(repo_root, issue, "C5.1", "Receipt validation", &body, false)?;

    Ok(())
}

fn step_c5_5(repo_root: &Path, issue: u64, cycle: u64) -> Result<(bool, String), String> {
    if let Some(recorded_pass) = recorded_c5_5_pass(repo_root, cycle)? {
        let body = format!(
            "Pipeline: {}\n- reused_recorded_pass: true\n- cycle: {}",
            recorded_pass.pipeline_summary, cycle
        );
        steps::post_step(
            repo_root,
            issue,
            "C5.5",
            "Final pipeline gate",
            &body,
            false,
        )?;
        return Ok((true, recorded_pass.pipeline_summary));
    }

    eprintln!("C5.5: Final pipeline gate...");

    let output = runner::run_tool(repo_root, "pipeline-check", &["--json"])?;
    let exit_ok = output.status.success();
    let stdout = runner::stdout_text(&output);
    let stderr = runner::stderr_text(&output);
    let exit_code = output.status.code().unwrap_or(-1);

    let (passed, pipeline_summary, body, initial_result) = match parse_pipeline_gate_report(&stdout) {
        Ok(report) => {
            let passed = exit_ok
                && report.overall == "pass"
                && !report.has_blocking_findings
                && report.blocking_warning_count == 0;
            let pipeline_summary = format_pipeline_summary(&report);
            let mut body = format!(
                "Pipeline: {}\n- exit_code: {}\n- overall: {}\n- has_blocking_findings: {}\n- blocking_warning_count: {}",
                pipeline_summary,
                exit_code,
                report.overall,
                report.has_blocking_findings,
                report.blocking_warning_count
            );
            if !passed {
                let gate_failure_reason = if !exit_ok {
                    "tool exit failure"
                } else if report.overall != "pass" {
                    "overall status is not pass"
                } else if report.has_blocking_findings {
                    "blocking findings"
                } else if report.blocking_warning_count > 0 {
                    "blocking warnings"
                } else {
                    "unknown"
                };
                body.push_str(&format!("\n- gate_failure_reason: {}", gate_failure_reason));
            }
            if !stdout.is_empty() {
                body.push_str(&format!("\n- raw_json: {}", stdout));
            }
            if !stderr.is_empty() {
                body.push_str(&format!("\n- stderr: {}", stderr));
            }
            let initial_result = (!passed).then(|| {
                serde_json::json!({
                    "result": "FAIL",
                    "summary": pipeline_summary,
                    "exit_code": exit_code,
                    "overall": report.overall,
                    "has_blocking_findings": report.has_blocking_findings,
                })
            });
            (passed, pipeline_summary, body, initial_result)
        }
        Err(parse_error) => {
            let pipeline_summary = "FAIL (invalid pipeline-check JSON)".to_string();
            let mut body = format!(
                "Pipeline: FAIL\n- exit_code: {}\n- json_parse_error: {}",
                exit_code, parse_error
            );
            if !stdout.is_empty() {
                body.push_str(&format!("\n- stdout: {}", stdout));
            }
            if !stderr.is_empty() {
                body.push_str(&format!("\n- stderr: {}", stderr));
            }
            (
                false,
                pipeline_summary.clone(),
                body,
                Some(serde_json::json!({
                    "result": "FAIL",
                    "summary": pipeline_summary,
                    "exit_code": exit_code,
                    "json_parse_error": parse_error,
                })),
            )
        }
    };

    if let Some(initial_result) = initial_result {
        record_initial_c5_5_failure(repo_root, cycle, initial_result)?;
    }

    steps::post_step(
        repo_root,
        issue,
        "C5.5",
        "Final pipeline gate",
        &body,
        false,
    )?;

    if !passed {
        return Err("Pipeline check failed at C5.5 — fix issues and re-run close-out".to_string());
    }

    record_c5_5_pass(repo_root, cycle, &pipeline_summary)?;

    Ok((true, pipeline_summary))
}

fn record_initial_c5_5_failure(
    repo_root: &Path,
    cycle: u64,
    mut initial_result: Value,
) -> Result<(), String> {
    let mut state = read_state_value(repo_root)?;
    let initial_result_object = initial_result
        .as_object_mut()
        .ok_or_else(|| "initial C5.5 result must be a JSON object".to_string())?;
    initial_result_object.insert("cycle".to_string(), serde_json::json!(cycle));

    if state
        .pointer("/tool_pipeline/c5_5_initial_result/cycle")
        .and_then(Value::as_u64)
        == Some(cycle)
    {
        return Ok(());
    }

    let tool_pipeline = state
        .get_mut("tool_pipeline")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "missing object /tool_pipeline in docs/state.json".to_string())?;
    tool_pipeline.insert("c5_5_initial_result".to_string(), initial_result);
    tool_pipeline.insert(
        "c5_5_gate".to_string(),
        serde_json::json!({
            "cycle": cycle,
            "status": "FAIL",
            "needs_reverify": true,
        }),
    );
    write_state_value(repo_root, &state)?;

    let commit_message = format!(
        "state(pipeline): record initial C5.5 FAIL for cycle {} [cycle {}]",
        cycle, cycle
    );
    commit_state_json(repo_root, &commit_message)?;
    Ok(())
}

fn record_c5_5_pass(repo_root: &Path, cycle: u64, pipeline_summary: &str) -> Result<(), String> {
    let mut state = read_state_value(repo_root)?;
    if let Some(existing) = state.pointer("/tool_pipeline/c5_5_gate") {
        if let Some(recorded_pass) = parse_recorded_c5_5_pass(existing, cycle) {
            if recorded_pass.pipeline_summary == pipeline_summary {
                return Ok(());
            }
        }
    }

    let tool_pipeline = state
        .get_mut("tool_pipeline")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "missing object /tool_pipeline in docs/state.json".to_string())?;
    tool_pipeline.insert(
        "c5_5_gate".to_string(),
        serde_json::json!({
            "cycle": cycle,
            "status": "PASS",
            "needs_reverify": false,
            "pipeline_summary": pipeline_summary,
        }),
    );
    write_state_value(repo_root, &state)?;

    let commit_message = format!(
        "state(pipeline): record C5.5 PASS for cycle {} [cycle {}]",
        cycle, cycle
    );
    commit_state_json(repo_root, &commit_message)?;
    Ok(())
}

fn recorded_c5_5_pass(repo_root: &Path, cycle: u64) -> Result<Option<RecordedC5_5Pass>, String> {
    let state = read_state_value(repo_root)?;
    Ok(state
        .pointer("/tool_pipeline/c5_5_gate")
        .and_then(|gate| parse_recorded_c5_5_pass(gate, cycle)))
}

fn parse_recorded_c5_5_pass(gate: &Value, cycle: u64) -> Option<RecordedC5_5Pass> {
    if gate.get("cycle").and_then(Value::as_u64) != Some(cycle) {
        return None;
    }
    if gate.get("status").and_then(Value::as_str) != Some("PASS") {
        return None;
    }
    if gate.get("needs_reverify").and_then(Value::as_bool) != Some(false) {
        return None;
    }
    let pipeline_summary = gate
        .get("pipeline_summary")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("PASS")
        .to_string();
    Some(RecordedC5_5Pass { pipeline_summary })
}

fn ensure_c5_5_allows_c6(repo_root: &Path, cycle: u64) -> Result<(), String> {
    let state = read_state_value(repo_root)?;
    let Some(gate) = state.pointer("/tool_pipeline/c5_5_gate") else {
        return Err(format!(
            "Cannot proceed to C6: no C5.5 result recorded for cycle {}. Re-run close-out to execute and pass C5.5 pipeline-check first.",
            cycle
        ));
    };
    if gate.get("cycle").and_then(Value::as_u64) != Some(cycle) {
        return Err(format!(
            "Cannot proceed to C6: C5.5 was not re-verified for cycle {}. Re-run close-out to execute and pass C5.5 pipeline-check first.",
            cycle
        ));
    }
    if gate.get("needs_reverify").and_then(Value::as_bool) == Some(true) {
        return Err(format!(
            "Cannot proceed to C6: C5.5 previously failed for cycle {} and still needs re-verification. Re-run close-out to execute and pass C5.5 pipeline-check first.",
            cycle
        ));
    }
    if gate.get("status").and_then(Value::as_str) != Some("PASS") {
        return Err(format!(
            "Cannot proceed to C6: C5.5 has not recorded a PASS for cycle {}. Re-run close-out to execute and pass C5.5 pipeline-check first.",
            cycle
        ));
    }
    Ok(())
}

fn parse_pipeline_gate_report(stdout: &str) -> Result<PipelineGateReport, String> {
    let report: Value = serde_json::from_str(stdout)
        .map_err(|error| format!("failed to parse pipeline-check JSON output: {}", error))?;
    let overall = report
        .get("overall")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing string overall in pipeline-check JSON output".to_string())?;
    let has_blocking_findings = report
        .get("has_blocking_findings")
        .and_then(Value::as_bool)
        .ok_or_else(|| {
            "missing bool has_blocking_findings in pipeline-check JSON output".to_string()
        })?;
    let steps = report
        .get("steps")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or(&[]);
    let warning_count = steps
        .iter()
        .filter(|step| step.get("status").and_then(Value::as_str) == Some("warn"))
        .count();
    let blocking_warning_count = steps
        .iter()
        .filter(|step| step.get("status").and_then(Value::as_str) == Some("warn"))
        .filter(|step| step.get("severity").and_then(Value::as_str) == Some("blocking"))
        .count();
    let cascade_count = steps
        .iter()
        .filter(|step| step.get("status").and_then(Value::as_str) == Some("cascade"))
        .count();
    let blocking_steps = steps
        .iter()
        .filter(|step| step.get("status").and_then(Value::as_str) == Some("fail"))
        .filter_map(|step| step.get("name").and_then(Value::as_str).map(str::to_string))
        .collect();

    Ok(PipelineGateReport {
        overall: overall.to_string(),
        has_blocking_findings,
        warning_count,
        blocking_warning_count,
        cascade_count,
        blocking_steps,
    })
}

fn format_pipeline_summary(report: &PipelineGateReport) -> String {
    let overall = report.overall.to_ascii_uppercase();
    let mut details = Vec::new();

    if report.blocking_warning_count > 0 {
        let suffix = if report.blocking_warning_count == 1 {
            "blocking warning"
        } else {
            "blocking warnings"
        };
        details.push(format!("{} {}", report.blocking_warning_count, suffix));
    }

    let non_blocking_warning_count = report
        .warning_count
        .saturating_sub(report.blocking_warning_count);
    if non_blocking_warning_count > 0 {
        let suffix = if non_blocking_warning_count == 1 {
            "warning"
        } else {
            "warnings"
        };
        details.push(format!("{} {}", non_blocking_warning_count, suffix));
    }

    if report.cascade_count > 0 {
        let suffix = if report.cascade_count == 1 {
            "cascade"
        } else {
            "cascades"
        };
        details.push(format!("{} {}", report.cascade_count, suffix));
    }

    if report.has_blocking_findings {
        if report.blocking_steps.is_empty() {
            details.push("blocking findings".to_string());
        } else {
            details.push(format!(
                "{} blocking: {}",
                report.blocking_steps.len(),
                report.blocking_steps.join(", ")
            ));
        }
    }

    if details.is_empty() {
        overall
    } else {
        format!("{} ({})", overall, details.join(", "))
    }
}

fn step_c5_6(
    repo_root: &Path,
    issue: u64,
    cycle: u64,
    pipeline_passed: bool,
) -> Result<(), String> {
    let state = state_schema::read_state_value(repo_root)?;

    let mode = state
        .pointer("/project_mode/mode")
        .and_then(Value::as_str)
        .unwrap_or("normal");

    if mode != "stabilization" {
        steps::post_step(
            repo_root,
            issue,
            "C5.6",
            "Stabilization counter",
            "Not in stabilization mode, skipping",
            false,
        )?;
        return Ok(());
    }

    eprintln!("C5.6: Updating stabilization counter...");

    // Check if already updated for this cycle
    let already_updated = state
        .pointer("/project_mode/consecutive_clean_cycles")
        .and_then(Value::as_array)
        .is_some_and(|arr| arr.iter().any(|v| v.as_u64() == Some(cycle)));

    if already_updated {
        let counter = state
            .pointer("/project_mode/clean_cycle_counter")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        let target = state
            .pointer("/project_mode/burn_in_target")
            .and_then(Value::as_u64)
            .unwrap_or(12);
        let body = format!(
            "Already updated for cycle {} (counter: {}/{})",
            cycle, counter, target
        );
        steps::post_step(
            repo_root,
            issue,
            "C5.6",
            "Stabilization counter",
            &body,
            false,
        )?;
        return Ok(());
    }

    let had_dispatches = had_tool_dispatches_this_cycle(&state, cycle);
    let is_clean = pipeline_passed && !had_dispatches;

    let mut state = state;
    let target = state
        .pointer("/project_mode/burn_in_target")
        .and_then(Value::as_u64)
        .unwrap_or(12);

    let (body, commit_msg) = if is_clean {
        let counter = state
            .pointer("/project_mode/clean_cycle_counter")
            .and_then(Value::as_u64)
            .unwrap_or(0);
        let new_counter = counter + 1;

        state["project_mode"]["clean_cycle_counter"] = serde_json::json!(new_counter);
        if let Some(arr) = state
            .pointer_mut("/project_mode/consecutive_clean_cycles")
            .and_then(Value::as_array_mut)
        {
            arr.push(serde_json::json!(cycle));
        }

        let body = format!(
            "Clean cycle {}/{} (pipeline passed, zero tool dispatches)",
            new_counter, target
        );
        let msg = format!(
            "state(stabilization): clean cycle {}/{} — cycle {} [cycle {}]",
            new_counter, target, cycle, cycle
        );

        if new_counter >= target {
            eprintln!(
                "Stabilization target reached! ({}/{}) — recommend transition out of stabilization",
                new_counter, target
            );
        }

        (body, msg)
    } else {
        let reason = if !pipeline_passed {
            "pipeline failed"
        } else {
            "tool dispatches detected"
        };

        state["project_mode"]["clean_cycle_counter"] = serde_json::json!(0);
        state["project_mode"]["consecutive_clean_cycles"] = serde_json::json!([]);

        let body = format!("Counter reset to 0 ({})", reason);
        let msg = format!(
            "state(stabilization): counter reset — {} [cycle {}]",
            reason, cycle
        );
        (body, msg)
    };

    state_schema::write_state_value(repo_root, &state)?;
    state_schema::commit_state_json(repo_root, &commit_msg)?;

    steps::post_step(
        repo_root,
        issue,
        "C5.6",
        "Stabilization counter",
        &body,
        false,
    )?;

    Ok(())
}

/// Check whether the Copilot agent is likely available by examining recent
/// agent session outcomes.  Returns `Ok(true)` when available, `Ok(false)`
/// when the last `WINDOW` sessions all ended in a failure status.
fn is_copilot_available(state: &Value) -> bool {
    const WINDOW: usize = 3;

    let sessions = match state.get("agent_sessions").and_then(Value::as_array) {
        Some(s) if !s.is_empty() => s,
        _ => return true, // no history → assume available
    };

    // Sort by dispatched_at descending (most recent first)
    let mut sorted: Vec<&Value> = sessions.iter().collect();
    sorted.sort_by(|a, b| {
        let ts_a = a.get("dispatched_at").and_then(Value::as_str).unwrap_or("");
        let ts_b = b.get("dispatched_at").and_then(Value::as_str).unwrap_or("");
        ts_b.cmp(ts_a)
    });

    let recent: Vec<&str> = sorted
        .iter()
        .take(WINDOW)
        .filter_map(|s| s.get("status").and_then(Value::as_str))
        .collect();

    if recent.len() < WINDOW {
        return true; // not enough history to judge
    }

    let failure_statuses = ["failed", "closed_without_pr"];
    !recent.iter().all(|s| failure_statuses.contains(s))
}

fn step_c6(repo_root: &Path, issue: u64, cycle: u64) -> Result<Option<ReviewInfo>, String> {
    eprintln!("C6: Dispatching review agent...");

    // Check if review already dispatched for this cycle (idempotency)
    let state = state_schema::read_state_value(repo_root)?;
    if let Some(existing) = find_existing_review_dispatch(&state, cycle) {
        let body = format!(
            "Review already dispatched as #{} (skipping duplicate)",
            existing.issue_number
        );
        steps::post_step(repo_root, issue, "C6", "Review dispatch", &body, false)?;
        return Ok(Some(existing));
    }

    // Copilot availability gate (per audit recommendation #329)
    if !is_copilot_available(&state) {
        let body = "Review deferred — Copilot agent unavailable \
                    (last 3 dispatches all failed). \
                    When Copilot resumes, the next cycle's review should cover the full gap period.";
        steps::post_step(repo_root, issue, "C6", "Review dispatch", body, false)?;
        eprintln!("C6: Review skipped — Copilot unavailable");
        return Ok(None);
    }

    // Check stabilization mode
    let is_stabilization =
        state.pointer("/project_mode/mode").and_then(Value::as_str) == Some("stabilization");

    // Generate review body
    let body_content = review_body::generate(repo_root, cycle, issue, is_stabilization)?;
    let body_path = review_body::write_to_file(repo_root, cycle, &body_content)?;
    eprintln!("Review body written to {}", body_path.display());

    // Call dispatch-review
    let cycle_str = cycle.to_string();
    let issue_str = issue.to_string();
    let body_path_str = body_path.to_string_lossy().to_string();

    let output = runner::run_tool(
        repo_root,
        "dispatch-review",
        &[
            "--cycle",
            &cycle_str,
            "--issue",
            &issue_str,
            "--body-file",
            &body_path_str,
        ],
    )?;

    if !output.status.success() {
        let stderr = runner::stderr_text(&output);
        return Err(format!("dispatch-review failed at C6: {}", stderr));
    }

    let stdout = runner::stdout_text(&output);
    let review_info = parse_dispatch_output(&stdout)?;

    let step_body = if is_stabilization {
        format!(
            "Review dispatched as #{} (observation mode — ADR 0011)",
            review_info.issue_number
        )
    } else {
        format!("Review dispatched as #{}", review_info.issue_number)
    };
    steps::post_step(repo_root, issue, "C6", "Review dispatch", &step_body, false)?;

    Ok(Some(review_info))
}

fn step_c7(repo_root: &Path, issue: u64) -> Result<(), String> {
    eprintln!("C7: Pushing dispatch state...");

    git::push(repo_root)?;

    steps::post_step(
        repo_root,
        issue,
        "C7",
        "Dispatch state push",
        "Pushed to origin/master",
        false,
    )?;

    Ok(())
}

fn step_c6_5(
    repo_root: &Path,
    issue: u64,
    cycle: u64,
    worklog: &Path,
    pipeline_summary: &str,
    prior_gate_failures: &[String],
) -> Result<(), String> {
    eprintln!("C6.5: Refreshing worklog state after review dispatch...");

    let state_value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(state_value)
        .map_err(|error| format!("failed to parse docs/state.json after C6: {}", error))?;
    let in_flight = state
        .extra
        .get("in_flight_sessions")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| "missing in_flight_sessions in state.json".to_string())?;
    let publish_gate = state
        .publish_gate()?
        .status
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "missing publish_gate.status in state.json".to_string())?
        .to_string();
    let next_steps = derive_patch_pipeline_next_steps(&state)?;
    let issues_processed = derive_patch_pipeline_issues_processed(&state, cycle)?;
    let worklog_str = worklog.to_string_lossy().to_string();
    let in_flight_str = in_flight.to_string();
    let mut patch_args = vec![
        "patch-pipeline".to_string(),
        "--worklog-file".to_string(),
        worklog_str.clone(),
        "--status".to_string(),
        pipeline_summary.to_string(),
        "--in-flight".to_string(),
        in_flight_str,
        "--publish-gate".to_string(),
        publish_gate.clone(),
        "--section-title".to_string(),
        "Cycle state".to_string(),
        "--issues-processed".to_string(),
        issues_processed,
    ];
    if !prior_gate_failures.is_empty() {
        patch_args.push("--prior-gate-failures".to_string());
        patch_args.push(prior_gate_failures.join(","));
    }
    for next_step in &next_steps {
        patch_args.push("--next-steps".to_string());
        patch_args.push(next_step.clone());
    }
    let patch_args_refs: Vec<&str> = patch_args.iter().map(String::as_str).collect();

    let output = runner::run_tool(repo_root, "write-entry", &patch_args_refs)?;
    if !output.status.success() {
        return Err(format!(
            "write-entry patch-pipeline failed at C6.5: {}",
            runner::stderr_text(&output)
        ));
    }

    let worklog_rel = worklog
        .strip_prefix(repo_root)
        .ok()
        .and_then(|path| path.to_str())
        .unwrap_or("worklog");
    let sha = git::add_and_commit(
        repo_root,
        &[worklog_rel],
        &format!(
            "docs(worklog): refresh cycle {} state after review dispatch [cycle {}]",
            cycle, cycle
        ),
    )?;
    let body = if sha.is_empty() {
        format!("Worklog state already current: {}", worklog_rel)
    } else {
        format!("Patched worklog state after C6: {} ({})", worklog_rel, sha)
    };
    steps::post_step(
        repo_root,
        issue,
        "C6.5",
        "Refresh worklog state",
        &body,
        false,
    )?;
    Ok(())
}

fn derive_patch_pipeline_next_steps(state: &StateJson) -> Result<Vec<String>, String> {
    let mut next_steps = Vec::new();

    for session in &state.agent_sessions {
        if session.status.as_deref().map(str::trim) != Some("in_flight") {
            continue;
        }
        let issue = session
            .issue
            .ok_or_else(|| {
                "agent_sessions[].issue is required for in-flight sessions during C6.5 refresh"
                    .to_string()
            })
            .and_then(|value| {
                u64::try_from(value).map_err(|_| {
                    "agent_sessions[].issue must be a positive integer for in-flight sessions during C6.5 refresh"
                        .to_string()
                })
            })?;
        let title = session
            .title
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(|value| format!(" ({})", value))
            .unwrap_or_default();
        next_steps.push(format!(
            "Review and iterate on PR from [#{}](https://github.com/{}/issues/{}){} when Copilot completes",
            issue, MAIN_REPO, issue, title
        ));
    }

    if next_steps.is_empty() {
        next_steps.push("No in-flight sessions — plan next dispatch".to_string());
    }

    Ok(next_steps)
}

fn derive_patch_pipeline_issues_processed(state: &StateJson, cycle: u64) -> Result<String, String> {
    let cycle_start = patch_pipeline_cycle_start(state, cycle)?;
    let mut entries = Vec::new();

    for session in &state.agent_sessions {
        if !patch_pipeline_session_active_this_cycle(session, cycle_start)? {
            continue;
        }
        let issue = session
            .issue
            .ok_or_else(|| {
                "agent_sessions[].issue is required for active sessions during C6.5 issues refresh"
                    .to_string()
            })
            .and_then(|value| {
                u64::try_from(value).map_err(|_| {
                    "agent_sessions[].issue must be a positive integer for active sessions during C6.5 issues refresh"
                        .to_string()
                })
            })?;
        let title = session
            .title
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                format!(
                    "agent_sessions[issue={}].title is required for active sessions during C6.5 issues refresh",
                    issue
                )
            })?;
        let status = session
            .status
            .as_deref()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                format!(
                    "agent_sessions[issue={}].status is required for active sessions during C6.5 issues refresh",
                    issue
                )
            })?;
        validate_patch_pipeline_issue_field(title, "title", issue)?;
        validate_patch_pipeline_issue_field(status, "status", issue)?;
        entries.push(format!("{issue};{title};{status}"));
    }

    Ok(entries.join("|"))
}

fn patch_pipeline_cycle_start(state: &StateJson, cycle: u64) -> Result<DateTime<Utc>, String> {
    let state_cycle = state.cycle_phase.cycle.ok_or_else(|| {
        "missing docs/state.json cycle_phase.cycle for C6.5 issues refresh".to_string()
    })?;
    if state_cycle != cycle {
        return Err(format!(
            "docs/state.json cycle_phase.cycle {} does not match close-out cycle {} for C6.5 issues refresh",
            state_cycle, cycle
        ));
    }
    let phase_entered_at = state
        .cycle_phase
        .phase_entered_at
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            "missing docs/state.json cycle_phase.phase_entered_at for C6.5 issues refresh"
                .to_string()
        })?;
    parse_patch_pipeline_timestamp(
        phase_entered_at,
        "docs/state.json cycle_phase.phase_entered_at",
    )
}

fn patch_pipeline_session_active_this_cycle(
    session: &AgentSession,
    cycle_start: DateTime<Utc>,
) -> Result<bool, String> {
    if let Some(timestamp) = session.dispatched_at.as_deref() {
        if parse_patch_pipeline_timestamp(timestamp, "agent_sessions[].dispatched_at")?
            >= cycle_start
        {
            return Ok(true);
        }
    }
    if let Some(timestamp) = session.merged_at.as_deref() {
        if parse_patch_pipeline_timestamp(timestamp, "agent_sessions[].merged_at")? >= cycle_start {
            return Ok(true);
        }
    }
    for field in PATCH_PIPELINE_ACTIVITY_TIMESTAMP_FIELDS {
        let Some(value) = session.extra.get(field) else {
            continue;
        };
        let Some(timestamp) = value.as_str() else {
            return Err(format!(
                "agent_sessions[].{} must be a string timestamp",
                field
            ));
        };
        if parse_patch_pipeline_timestamp(timestamp, &format!("agent_sessions[].{}", field))?
            >= cycle_start
        {
            return Ok(true);
        }
    }

    Ok(false)
}

fn parse_patch_pipeline_timestamp(value: &str, label: &str) -> Result<DateTime<Utc>, String> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp: DateTime<FixedOffset>| timestamp.with_timezone(&Utc))
        .map_err(|error| format!("invalid {} '{}': {}", label, value, error))
}

fn validate_patch_pipeline_issue_field(value: &str, field: &str, issue: u64) -> Result<(), String> {
    if value.contains('|') || value.contains(';') {
        return Err(format!(
            "agent_sessions[issue={}].{} must not contain '|' or ';' for C6.5 issues refresh",
            issue, field
        ));
    }
    Ok(())
}

fn step_c8(
    repo_root: &Path,
    issue: u64,
    cycle: u64,
    review_info: Option<&ReviewInfo>,
    pipeline_summary: &str,
) -> Result<(), String> {
    eprintln!("C8: Closing orchestrator issue...");

    let review_line = match review_info {
        Some(info) => format!(
            "- Review: dispatched as #{} ({})",
            info.issue_number, info.issue_url
        ),
        None => "- Review: deferred (Copilot unavailable)".to_string(),
    };
    let closing_body = format!(
        "Cycle {} close-out complete.\n\n\
         {}\n\
         - Pipeline: {}\n\
         - All close-out steps completed by cycle-runner",
        cycle, review_line, pipeline_summary,
    );

    steps::post_step(
        repo_root,
        issue,
        "C8",
        "Cycle close-out",
        &closing_body,
        false,
    )?;

    // Close the issue
    let issue_str = issue.to_string();
    let output = Command::new("gh")
        .args(["issue", "close", &issue_str, "-R", MAIN_REPO])
        .output()
        .map_err(|error| format!("failed to execute gh issue close: {}", error))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        // Already closed is fine
        if !stderr.contains("already closed") {
            return Err(format!("gh issue close failed: {}", stderr));
        }
    }

    eprintln!("Issue #{} closed", issue);
    Ok(())
}

fn had_tool_dispatches_this_cycle(state: &Value, _cycle: u64) -> bool {
    let last_cycle_ts = state
        .pointer("/last_cycle/timestamp")
        .and_then(Value::as_str)
        .unwrap_or("");

    let sessions = match state.get("agent_sessions").and_then(Value::as_array) {
        Some(s) => s,
        None => return false,
    };

    for session in sessions {
        let dispatched_at = session
            .get("dispatched_at")
            .and_then(Value::as_str)
            .unwrap_or("");
        // Only check sessions dispatched after the previous cycle ended
        if !last_cycle_ts.is_empty() && dispatched_at > last_cycle_ts {
            let title = session.get("title").and_then(Value::as_str).unwrap_or("");
            // Exclude review dispatches (they're mandatory, not "tool" dispatches)
            if !title.starts_with("[Cycle Review]") {
                return true;
            }
        }
    }

    false
}

fn find_existing_review_dispatch(state: &Value, cycle: u64) -> Option<ReviewInfo> {
    let expected_title = format!("[Cycle Review] Cycle {} end-of-cycle review", cycle);

    let sessions = state.get("agent_sessions").and_then(Value::as_array)?;
    for session in sessions {
        let title = session.get("title").and_then(Value::as_str).unwrap_or("");
        if title == expected_title {
            let issue_number = session.get("issue").and_then(Value::as_u64)?;
            return Some(ReviewInfo {
                issue_number,
                issue_url: format!("https://github.com/{}/issues/{}", MAIN_REPO, issue_number),
            });
        }
    }
    None
}

fn parse_dispatch_output(stdout: &str) -> Result<ReviewInfo, String> {
    // Format: "Created review issue #NNN from orchestrator issue #NNN: URL"
    let issue_num = stdout
        .strip_prefix("Created review issue #")
        .and_then(|s| s.split_whitespace().next())
        .and_then(|s| s.parse::<u64>().ok())
        .ok_or_else(|| {
            format!(
                "failed to parse review issue number from dispatch-review output: {}",
                stdout
            )
        })?;

    let url = stdout.rsplit(": ").next().unwrap_or("").trim().to_string();

    Ok(ReviewInfo {
        issue_number: issue_num,
        issue_url: if url.starts_with("https://") {
            url
        } else {
            format!("https://github.com/{}/issues/{}", MAIN_REPO, issue_num)
        },
    })
}

fn print_dry_run(cycle: u64, issue: u64) {
    for line in close_out_dry_run_lines(cycle, issue) {
        eprintln!("{}", line);
    }
}

fn close_out_dry_run_lines(cycle: u64, issue: u64) -> Vec<String> {
    vec![
        format!(
            "[dry-run] Would run close-out sequence for cycle {} (issue #{})",
            cycle, issue
        ),
        "[dry-run] C4.1: validate-docs worklog + journal (GATE)".to_string(),
        "[dry-run] C4.5: scan doc/adr/ and post ADR check step".to_string(),
        "[dry-run] C4.7: verify-review-events --apply (best-effort, non-blocking)".to_string(),
        "[dry-run] C5:   git add docs/ && git commit && git push (worklog frozen at this point)"
            .to_string(),
        "[dry-run] C5.1: receipt-validate (report only)".to_string(),
        "[dry-run] C5.5: pipeline-check (GATE)".to_string(),
        "[dry-run] C5.6: stabilization counter update (if applicable)".to_string(),
        "[dry-run] C6:   generate review body + dispatch-review".to_string(),
        "[dry-run] C6.5: refresh worklog state from updated docs/state.json".to_string(),
        "[dry-run] C7:   git push".to_string(),
        format!("[dry-run] C8:   close issue #{}", issue),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::ffi::OsString;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::process::Command;
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    #[test]
    fn parse_dispatch_output_extracts_issue_number() {
        let stdout = "Created review issue #1470 from orchestrator issue #1459: https://github.com/EvaLok/schema-org-json-ld/issues/1470";
        let info = parse_dispatch_output(stdout).unwrap();
        assert_eq!(info.issue_number, 1470);
        assert!(info.issue_url.contains("1470"));
    }

    #[test]
    fn had_tool_dispatches_detects_non_review_dispatch() {
        let state = json!({
            "agent_sessions": [
                {
                    "dispatched_at": "2026-03-19T10:00:00Z",
                    "title": "Implement FooBar",
                    "status": "in_flight"
                }
            ],
            "last_cycle": {"timestamp": "2026-03-19T08:00:00Z"}
        });
        assert!(had_tool_dispatches_this_cycle(&state, 301));
    }

    #[test]
    fn had_tool_dispatches_ignores_review_dispatch() {
        let state = json!({
            "agent_sessions": [
                {
                    "dispatched_at": "2026-03-19T10:00:00Z",
                    "title": "[Cycle Review] Cycle 301 end-of-cycle review",
                    "status": "in_flight"
                }
            ],
            "last_cycle": {"timestamp": "2026-03-19T08:00:00Z"}
        });
        assert!(!had_tool_dispatches_this_cycle(&state, 301));
    }

    #[test]
    fn had_tool_dispatches_ignores_old_sessions() {
        let state = json!({
            "agent_sessions": [
                {
                    "dispatched_at": "2026-03-18T10:00:00Z",
                    "title": "Implement FooBar",
                    "status": "merged"
                }
            ],
            "last_cycle": {"timestamp": "2026-03-19T08:00:00Z"}
        });
        assert!(!had_tool_dispatches_this_cycle(&state, 301));
    }

    #[test]
    fn find_existing_review_dispatch_matches_cycle() {
        let state = json!({
            "agent_sessions": [
                {"issue": 1470, "title": "[Cycle Review] Cycle 301 end-of-cycle review", "status": "in_flight"},
                {"issue": 1400, "title": "[Cycle Review] Cycle 300 end-of-cycle review", "status": "merged"}
            ]
        });
        let result = find_existing_review_dispatch(&state, 301);
        assert!(result.is_some());
        assert_eq!(result.unwrap().issue_number, 1470);

        let result_300 = find_existing_review_dispatch(&state, 300);
        assert!(result_300.is_some());
        assert_eq!(result_300.unwrap().issue_number, 1400);

        assert!(find_existing_review_dispatch(&state, 999).is_none());
    }

    #[test]
    fn derive_patch_pipeline_issues_processed_uses_active_cycle_sessions() {
        let state: StateJson = serde_json::from_value(json!({
            "cycle_phase": {
                "cycle": 345,
                "phase_entered_at": "2026-03-25T00:00:00Z"
            },
            "agent_sessions": [
                {
                    "issue": 1200,
                    "title": "Dispatched this cycle",
                    "status": "merged",
                    "dispatched_at": "2026-03-25T01:00:00Z"
                },
                {
                    "issue": 1201,
                    "title": "Closed this cycle",
                    "status": "closed_without_pr",
                    "dispatched_at": "2026-03-24T01:00:00Z",
                    "closed_at": "2026-03-25T02:00:00Z"
                },
                {
                    "issue": 1199,
                    "title": "Old session",
                    "status": "merged",
                    "dispatched_at": "2026-03-24T01:00:00Z"
                }
            ]
        }))
        .unwrap();

        let issues = derive_patch_pipeline_issues_processed(&state, 345).unwrap();
        assert_eq!(
            issues,
            "1200;Dispatched this cycle;merged|1201;Closed this cycle;closed_without_pr"
        );
    }

    fn setup_temp_repo(name: &str) -> std::path::PathBuf {
        let dir = unique_temp_dir(&format!("cycle-runner-close-out-{}", name));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("docs")).unwrap();
        fs::create_dir_all(dir.join("tools")).unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["init", "-b", "master"])
            .output()
            .unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["config", "user.email", "test@test.com"])
            .output()
            .unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["config", "user.name", "Test"])
            .output()
            .unwrap();
        dir
    }

    fn write_post_step_capture_script(dir: &std::path::Path, output_path: &std::path::Path) {
        let output_path = shell_single_quote(output_path);
        fs::write(
            dir.join("tools/post-step"),
            format!(
                "#!/usr/bin/env bash\nset -euo pipefail\n{{\nfor arg in \"$@\"; do\nprintf -- '---ARG---\\n%s\\n' \"$arg\"\ndone\n}} > {}\n",
                output_path
            ),
        )
        .unwrap();
    }

    fn write_post_step_append_capture_script(dir: &std::path::Path, output_path: &std::path::Path) {
        let output_path = shell_single_quote(output_path);
        fs::write(
            dir.join("tools/post-step"),
            format!(
                "#!/usr/bin/env bash\nset -euo pipefail\n{{\nfor arg in \"$@\"; do\nprintf -- '---ARG---\\n%s\\n' \"$arg\"\ndone\n}} >> {}\n",
                output_path
            ),
        )
        .unwrap();
    }

    fn write_write_entry_patch_script(dir: &std::path::Path) {
        fs::write(
            dir.join("tools/write-entry"),
            r#"#!/usr/bin/env bash
set -euo pipefail
python - "$@" <<'PY'
import sys
from pathlib import Path

ISSUES_URL = 'https://github.com/EvaLok/schema-org-json-ld/issues'

args = sys.argv[1:]
if not args or args[0] != 'patch-pipeline':
    raise SystemExit(f'unexpected write-entry args: {args}')
values = {}
list_values = {}
i = 1
while i < len(args):
    key = args[i]
    if key == '--next-steps':
        if i + 1 >= len(args):
            raise SystemExit(f'missing value for {key}')
        list_values.setdefault(key, []).append(args[i + 1])
        i += 2
        continue
    if i + 1 >= len(args):
        raise SystemExit(f'missing value for {key}')
    values[key] = args[i + 1]
    i += 2
worklog = Path(values['--worklog-file'])
lines = worklog.read_text().splitlines()
for index, line in enumerate(lines):
    if line == '## Pre-dispatch state':
        lines[index] = '## ' + values['--section-title']
        break
else:
    raise SystemExit('missing state heading')
lines = [line for line in lines if line != '*Snapshot before review dispatch — final counters may differ after C6.*']
lines = [line for line in lines if not line.startswith('- **Copilot metrics**: ')]

def patch_or_addendum(prefix, value, addendum_prefix):
    for index, line in enumerate(lines):
        if not line.startswith(prefix):
            continue
        current = line[len(prefix):].strip()
        if not current or current == 'Not provided.':
            lines[index] = prefix + value
            return
        if current == value.strip():
            return
        for addendum_index, addendum_line in enumerate(lines):
            if addendum_line.startswith(addendum_prefix):
                if addendum_line[len(addendum_prefix):].strip() != value.strip():
                    lines[addendum_index] = addendum_prefix + value
                return
        lines.insert(index + 1, addendum_prefix + value)
        return
    raise SystemExit(f'missing line for {prefix}')

def section_bounds(heading):
    for index, line in enumerate(lines):
        if line == heading:
            start = index + 1
            end = len(lines)
            for next_index in range(start, len(lines)):
                if lines[next_index].startswith('## ') or lines[next_index].startswith('### '):
                    end = next_index
                    break
            return index, start, end
    raise SystemExit(f'missing section {heading}')

def section_entries(start, end):
    return [line.strip() for line in lines[start:end] if line.strip()]

def section_has_placeholder(start, end):
    entries = section_entries(start, end)
    if not entries:
        return True
    if len(entries) == 1 and entries[0] in {'Not provided.', '1. Not provided.', '- None.', 'None.'}:
        return True
    return False

def render_numbered_steps(items):
    rendered = ['']
    for step_index, step in enumerate(items, start=1):
        rendered.append(f'{step_index}. {step}')
    return rendered

def render_issues(raw):
    if not raw.strip():
        return ['- None.']
    rendered = []
    for entry in raw.split('|'):
        parts = entry.split(';')
        if len(parts) != 3:
            raise SystemExit(f'invalid issues processed entry: {entry}')
        issue, title, status = [part.strip() for part in parts]
        rendered.append(f'- [#{issue}]({ISSUES_URL}/{issue}): {title} ({status})')
    return rendered

def patch_next_steps(items):
    _, start, end = section_bounds('## Next steps')
    replacement = render_numbered_steps(items)
    if section_has_placeholder(start, end):
        lines[start:end] = replacement
        return
    heading = '## Next steps (post-dispatch)'
    try:
        _, post_start, post_end = section_bounds(heading)
        lines[post_start:post_end] = replacement
    except SystemExit:
        insertion = []
        if end == 0 or lines[end - 1] != '':
            insertion.append('')
        insertion.append(heading)
        insertion.extend(replacement)
        lines[end:end] = insertion

def patch_issues_processed(raw):
    _, start, end = section_bounds('### Issues processed')
    replacement = ['']
    replacement.extend(render_issues(raw))
    if end < len(lines):
        replacement.append('')
    current = section_entries(start, end)
    expected = [line for line in replacement if line]
    if section_has_placeholder(start, end):
        lines[start:end] = replacement
        return
    if current == expected:
        return
    heading = '### Issues processed (post-dispatch)'
    try:
        _, post_start, post_end = section_bounds(heading)
        if section_entries(post_start, post_end) == expected:
            return
        lines[post_start:post_end] = replacement
    except SystemExit:
        insertion = []
        if end == 0 or lines[end - 1] != '':
            insertion.append('')
        insertion.append(heading)
        insertion.extend(replacement)
        if end < len(lines):
            insertion.append('')
        lines[end:end] = insertion

def patch_prior_gate_failures(raw):
    failures = [item.strip() for item in raw.split(',') if item.strip()]
    state_heading = '## ' + values['--section-title']
    _, start, end = section_bounds(state_heading)
    filtered = [line for line in lines[start:end] if not line.startswith('- **Close-out gate failures**: ')]
    lines[start:end] = filtered
    _, start, end = section_bounds(state_heading)
    insert_at = end
    for index in range(start, end):
        if lines[index].startswith('- **Pipeline status'):
            insert_at = index + 1
    rendered = [f'- **Close-out gate failures**: {failure}' for failure in failures]
    lines[insert_at:insert_at] = rendered

patch_or_addendum('- **In-flight agent sessions**: ', values['--in-flight'], '- **In-flight agent sessions (post-dispatch)**: ')
patch_or_addendum('- **Pipeline status**: ', values['--status'], '- **Pipeline status (post-dispatch)**: ')
if '--prior-gate-failures' in values:
    patch_prior_gate_failures(values['--prior-gate-failures'])
patch_or_addendum('- **Publish gate**: ', values['--publish-gate'], '- **Publish gate (post-dispatch)**: ')
patch_issues_processed(values['--issues-processed'])
if '--next-steps' in list_values:
    patch_next_steps(list_values['--next-steps'])
worklog.write_text('\n'.join(lines) + '\n')
print(worklog)
PY
"#,
        )
        .unwrap();
    }

    fn write_gh_script(
        path: &std::path::Path,
        issue: u64,
        comment_bodies: &[&str],
        support_issue_close: bool,
    ) {
        let comments_json = shell_single_quote_str(&serde_json::to_string(comment_bodies).unwrap());
        let close_branch = if support_issue_close {
            "if [ \"$1\" = \"issue\" ] && [ \"$2\" = \"close\" ]; then\n  exit 0\nfi\n"
        } else {
            ""
        };
        fs::write(
            path,
            format!(
                "#!/usr/bin/env bash\nset -euo pipefail\nexport COMMENTS_JSON={comments_json}\n{close_branch}if [ \"$1\" = \"api\" ] && [ \"$2\" = \"repos/EvaLok/schema-org-json-ld/issues/{issue}/comments\" ]; then\n  python - <<'PY'\nimport json\nimport os\nfor body in json.loads(os.environ['COMMENTS_JSON']):\n    print(json.dumps(body))\nPY\n  exit 0\nfi\nprintf 'unexpected gh invocation\\n' >&2\nexit 1\n",
            ),
        )
        .unwrap();
    }

    fn setup_temp_repo_with_remote(name: &str) -> (std::path::PathBuf, std::path::PathBuf) {
        let dir = setup_temp_repo(name);
        let remote = unique_temp_dir(&format!("cycle-runner-close-out-remote-{}", name));
        let _ = fs::remove_dir_all(&remote);
        Command::new("git")
            .arg("init")
            .arg("--bare")
            .arg(&remote)
            .output()
            .unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["remote", "add", "origin"])
            .arg(&remote)
            .output()
            .unwrap();
        (dir, remote)
    }

    fn path_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    fn make_executable(path: &std::path::Path) {
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }

    fn shell_single_quote(path: &std::path::Path) -> String {
        format!("'{}'", path.to_string_lossy().replace('\'', "'\"'\"'"))
    }

    fn shell_single_quote_str(value: &str) -> String {
        format!("'{}'", value.replace('\'', "'\"'\"'"))
    }

    fn with_path_prefix<T>(prefix: &std::path::Path, f: impl FnOnce() -> T) -> T {
        let _guard = path_lock().lock().unwrap();
        let old_path = std::env::var_os("PATH").unwrap_or_default();
        let mut new_path = OsString::from(prefix.as_os_str());
        new_path.push(":");
        new_path.push(&old_path);
        std::env::set_var("PATH", &new_path);
        let result = f();
        std::env::set_var("PATH", old_path);
        result
    }

    fn unique_temp_dir(prefix: &str) -> std::path::PathBuf {
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        std::env::temp_dir().join(format!("{}-{}", prefix, suffix))
    }

    fn write_minimal_close_out_state(dir: &std::path::Path, cycle: u64) {
        fs::write(
            dir.join("docs/state.json"),
            serde_json::to_string_pretty(&json!({
                "cycle_phase": {
                    "cycle": cycle,
                    "phase": "close_out",
                    "phase_entered_at": "2026-03-25T00:00:00Z"
                },
                "last_cycle": {
                    "number": cycle,
                    "timestamp": "2026-03-24T00:00:00Z"
                },
                "field_inventory": {
                    "fields": {
                        "cycle_phase": {
                            "last_refreshed": format!("cycle {}", cycle.saturating_sub(1))
                        }
                    }
                },
                "tool_pipeline": {
                    "status": "phase_5_active"
                }
            }))
            .unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn complete_close_out_phase_transitions_state_and_commits_expected_message() {
        let dir = setup_temp_repo("complete-phase");
        let state_path = dir.join("docs/state.json");
        fs::write(
            &state_path,
            serde_json::to_string_pretty(&json!({
                "cycle_phase": {
                    "cycle": 345,
                    "phase": "close_out",
                    "phase_entered_at": "2026-03-24T00:00:00Z"
                },
                "field_inventory": {
                    "fields": {
                        "cycle_phase": {
                            "last_refreshed": "cycle 344"
                        }
                    }
                }
            }))
            .unwrap(),
        )
        .unwrap();

        complete_close_out_phase(&dir, 345).unwrap();

        let state = state_schema::read_state_value(&dir).unwrap();
        assert_eq!(
            state.pointer("/cycle_phase/phase"),
            Some(&json!("complete"))
        );

        let log_output = Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["log", "-1", "--pretty=%s"])
            .output()
            .unwrap();
        assert!(log_output.status.success());
        assert_eq!(
            String::from_utf8(log_output.stdout).unwrap().trim(),
            "state(cycle-complete-phase): cycle 345 phase -> complete [cycle 345]"
        );

        fs::remove_dir_all(&dir).unwrap();
    }

    #[test]
    fn step_c5_5_rejects_zero_exit_when_json_overall_is_fail() {
        let dir = setup_temp_repo("step-c5-5-overall-fail");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        write_minimal_close_out_state(&dir, 345);
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"fail\",\"has_blocking_findings\":false}'\n",
        )
        .unwrap();

        let error = step_c5_5(&dir, 123, 345).unwrap_err();
        assert_eq!(
            error,
            "Pipeline check failed at C5.5 — fix issues and re-run close-out"
        );

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("---ARG---\nC5.5\n"));
        assert!(args.contains("overall: fail"));
        assert!(args.contains("has_blocking_findings: false"));

        let state = state_schema::read_state_value(&dir).unwrap();
        assert_eq!(
            state.pointer("/tool_pipeline/c5_5_initial_result"),
            Some(&json!({
                "cycle": 345,
                "result": "FAIL",
                "summary": "FAIL",
                "exit_code": 0,
                "overall": "fail",
                "has_blocking_findings": false
            }))
        );

        let log_output = Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["log", "-1", "--pretty=%s"])
            .output()
            .unwrap();
        assert!(log_output.status.success());
        assert_eq!(
            String::from_utf8(log_output.stdout).unwrap().trim(),
            "state(pipeline): record initial C5.5 FAIL for cycle 345 [cycle 345]"
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_pipeline_gate_report_counts_blocking_warnings_separately() {
        let report = parse_pipeline_gate_report(
            r#"{
                "overall":"pass",
                "has_blocking_findings":false,
                "steps":[
                    {"name":"worklog-dedup","status":"warn","severity":"blocking"},
                    {"name":"doc-validation","status":"warn","severity":"warning"},
                    {"name":"review-sync","status":"cascade"}
                ]
            }"#,
        )
        .unwrap();

        assert_eq!(
            report,
            PipelineGateReport {
                overall: "pass".to_string(),
                has_blocking_findings: false,
                warning_count: 2,
                blocking_warning_count: 1,
                cascade_count: 1,
                blocking_steps: Vec::new(),
            }
        );
    }

    #[test]
    fn format_pipeline_summary_distinguishes_blocking_warnings() {
        let summary = format_pipeline_summary(&PipelineGateReport {
            overall: "pass".to_string(),
            has_blocking_findings: false,
            warning_count: 3,
            blocking_warning_count: 1,
            cascade_count: 0,
            blocking_steps: Vec::new(),
        });

        assert_eq!(summary, "PASS (1 blocking warning, 2 warnings)");
    }

    #[test]
    fn step_c5_5_returns_pipeline_summary_for_warning_pass() {
        let dir = setup_temp_repo("step-c5-5-warning-pass");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        write_minimal_close_out_state(&dir, 345);
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"pass\",\"has_blocking_findings\":false,\"steps\":[{\"name\":\"doc-validation\",\"status\":\"warn\"}]}'\n",
        )
        .unwrap();

        let (passed, summary) = step_c5_5(&dir, 123, 345).unwrap();
        assert!(passed);
        assert_eq!(summary, "PASS (1 warning)");

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("---ARG---\nC5.5\n"));
        assert!(args.contains("Pipeline: PASS (1 warning)"));

        let state = state_schema::read_state_value(&dir).unwrap();
        assert_eq!(state.pointer("/tool_pipeline/c5_5_initial_result"), None);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn step_c5_5_rejects_zero_exit_when_json_reports_blocking_warnings() {
        let dir = setup_temp_repo("step-c5-5-blocking-warning-fail");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        write_minimal_close_out_state(&dir, 345);
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"pass\",\"has_blocking_findings\":false,\"steps\":[{\"name\":\"worklog-dedup\",\"status\":\"warn\",\"severity\":\"blocking\"},{\"name\":\"doc-validation\",\"status\":\"warn\",\"severity\":\"warning\"}]}'\n",
        )
        .unwrap();

        let error = step_c5_5(&dir, 123, 345).unwrap_err();
        assert_eq!(
            error,
            "Pipeline check failed at C5.5 — fix issues and re-run close-out"
        );

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("---ARG---\nC5.5\n"));
        assert!(args.contains("Pipeline: PASS (1 blocking warning, 1 warning)"));
        assert!(args.contains("has_blocking_findings: false"));
        assert!(args.contains("blocking_warning_count: 1"));
        assert!(args.contains("gate_failure_reason: blocking warnings"));

        let state = state_schema::read_state_value(&dir).unwrap();
        assert_eq!(
            state.pointer("/tool_pipeline/c5_5_initial_result"),
            Some(&json!({
                "cycle": 345,
                "result": "FAIL",
                "summary": "PASS (1 blocking warning, 1 warning)",
                "exit_code": 0,
                "overall": "pass",
                "has_blocking_findings": false
            }))
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn step_c5_5_rejects_zero_exit_when_json_reports_blocking_findings() {
        let dir = setup_temp_repo("step-c5-5-blocking");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        write_minimal_close_out_state(&dir, 345);
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"pass\",\"has_blocking_findings\":true}'\n",
        )
        .unwrap();

        let error = step_c5_5(&dir, 123, 345).unwrap_err();
        assert_eq!(
            error,
            "Pipeline check failed at C5.5 — fix issues and re-run close-out"
        );

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("overall: pass"));
        assert!(args.contains("has_blocking_findings: true"));
        assert!(args.contains("blocking_warning_count: 0"));
        assert!(args.contains("gate_failure_reason: blocking findings"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn step_c5_5_passes_when_blocking_warning_count_is_zero() {
        let dir = setup_temp_repo("step-c5-5-no-blocking-warnings");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        write_minimal_close_out_state(&dir, 345);
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"pass\",\"has_blocking_findings\":false,\"steps\":[{\"name\":\"doc-validation\",\"status\":\"warn\",\"severity\":\"warning\"},{\"name\":\"review-sync\",\"status\":\"cascade\"}]}'\n",
        )
        .unwrap();

        let (passed, summary) = step_c5_5(&dir, 123, 345).unwrap();
        assert!(passed);
        assert_eq!(summary, "PASS (1 warning, 1 cascade)");

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("---ARG---\nC5.5\n"));
        assert!(args.contains("Pipeline: PASS (1 warning, 1 cascade)"));
        assert!(args.contains("blocking_warning_count: 0"));
        assert!(!args.contains("gate_failure_reason:"));

        let state = state_schema::read_state_value(&dir).unwrap();
        assert_eq!(state.pointer("/tool_pipeline/c5_5_initial_result"), None);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn step_c5_5_rejects_zero_exit_when_json_overall_is_unexpected() {
        let dir = setup_temp_repo("step-c5-5-unexpected-overall");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        write_minimal_close_out_state(&dir, 345);
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"warning\",\"has_blocking_findings\":false}'\n",
        )
        .unwrap();

        let error = step_c5_5(&dir, 123, 345).unwrap_err();
        assert_eq!(
            error,
            "Pipeline check failed at C5.5 — fix issues and re-run close-out"
        );

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("overall: warning"));
        assert!(args.contains("has_blocking_findings: false"));
        assert!(args.contains("blocking_warning_count: 0"));
        assert!(args.contains("gate_failure_reason: overall status is not pass"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn c5_5_fail_blocks_progression_to_c6() {
        let dir = setup_temp_repo("step-c5-5-blocks-c6");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        write_minimal_close_out_state(&dir, 345);
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"fail\",\"has_blocking_findings\":false}'\n",
        )
        .unwrap();

        let error = step_c5_5(&dir, 123, 345).unwrap_err();
        assert_eq!(
            error,
            "Pipeline check failed at C5.5 — fix issues and re-run close-out"
        );

        let gate_error = ensure_c5_5_allows_c6(&dir, 345).unwrap_err();
        assert!(gate_error.contains("needs re-verification"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn c5_5_pass_after_previous_fail_allows_progression_to_c6() {
        let dir = setup_temp_repo("step-c5-5-pass-after-fail");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        write_minimal_close_out_state(&dir, 345);
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"fail\",\"has_blocking_findings\":false}'\n",
        )
        .unwrap();

        step_c5_5(&dir, 123, 345).unwrap_err();

        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"pass\",\"has_blocking_findings\":false}'\n",
        )
        .unwrap();

        let (passed, summary) = step_c5_5(&dir, 123, 345).unwrap();
        assert!(passed);
        assert_eq!(summary, "PASS");
        ensure_c5_5_allows_c6(&dir, 345).unwrap();

        let state = state_schema::read_state_value(&dir).unwrap();
        assert_eq!(
            state.pointer("/tool_pipeline/c5_5_gate"),
            Some(&json!({
                "cycle": 345,
                "status": "PASS",
                "needs_reverify": false,
                "pipeline_summary": "PASS"
            }))
        );
        assert_eq!(
            state.pointer("/tool_pipeline/c5_5_initial_result"),
            Some(&json!({
                "cycle": 345,
                "result": "FAIL",
                "summary": "FAIL",
                "exit_code": 0,
                "overall": "fail",
                "has_blocking_findings": false
            }))
        );

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn c5_5_pass_is_reused_without_rerunning_pipeline_check() {
        let dir = setup_temp_repo("step-c5-5-pass-reuse");
        let args_path = dir.join("post-step-args.txt");
        let invocation_count = dir.join("pipeline-check-count.txt");
        write_post_step_capture_script(&dir, &args_path);
        write_minimal_close_out_state(&dir, 345);
        fs::write(
            dir.join("tools/pipeline-check"),
            format!(
                "#!/usr/bin/env bash\nset -euo pipefail\ncount_file={count_file:?}\ncount=0\nif [ -f \"$count_file\" ]; then\n  count=$(cat \"$count_file\")\nfi\ncount=$((count + 1))\nprintf '%s' \"$count\" > \"$count_file\"\nprintf '%s\n' '{{\"overall\":\"pass\",\"has_blocking_findings\":false}}'\n",
                count_file = invocation_count
            ),
        )
        .unwrap();

        let (passed, summary) = step_c5_5(&dir, 123, 345).unwrap();
        assert!(passed);
        assert_eq!(summary, "PASS");

        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\necho 'pipeline-check should not rerun after PASS' >&2\nexit 1\n",
        )
        .unwrap();

        let (passed, summary) = step_c5_5(&dir, 123, 345).unwrap();
        assert!(passed);
        assert_eq!(summary, "PASS");
        assert_eq!(
            fs::read_to_string(&invocation_count).unwrap(),
            "1",
            "pipeline-check should only run once after PASS is recorded"
        );

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("reused_recorded_pass: true"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn close_out_dry_run_includes_step_c6_5_between_c6_and_c7() {
        let lines = close_out_dry_run_lines(345, 123);

        let c6 = lines
            .iter()
            .position(|line| line.contains("[dry-run] C6:"))
            .expect("C6 dry-run line should exist");
        let c6_5 = lines
            .iter()
            .position(|line| line.contains("[dry-run] C6.5:"))
            .expect("C6.5 dry-run line should exist");
        let c7 = lines
            .iter()
            .position(|line| line.contains("[dry-run] C7:"))
            .expect("C7 dry-run line should exist");

        assert_eq!(c6_5, c6 + 1);
        assert_eq!(c7, c6_5 + 1);
    }

    #[test]
    fn close_out_dry_run_includes_c4_7_between_c4_5_and_c5() {
        let lines = close_out_dry_run_lines(345, 123);

        let c4_5 = lines
            .iter()
            .position(|line| line.contains("[dry-run] C4.5:"))
            .expect("C4.5 dry-run line should exist");
        let c4_7 = lines
            .iter()
            .position(|line| line.contains("[dry-run] C4.7:"))
            .expect("C4.7 dry-run line should exist");
        let c5 = lines
            .iter()
            .position(|line| line.contains("[dry-run] C5:"))
            .expect("C5 dry-run line should exist");

        assert_eq!(c4_7, c4_5 + 1);
        assert_eq!(c5, c4_7 + 1);
    }

    #[test]
    fn step_c4_1_reports_validate_docs_stdout_failures() {
        let dir = setup_temp_repo("step-c4-1-stdout-failure");
        let worklog = dir.join("docs/worklog.md");
        let journal = dir.join("docs/journal.md");
        fs::write(&worklog, "# Worklog\n").unwrap();
        fs::write(&journal, "# Journal\n").unwrap();

        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        fs::write(
            dir.join("tools/validate-docs"),
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1\" = \"worklog\" ]; then\n  printf 'missing receipt summary'\n  exit 1\nfi\nif [ \"$1\" = \"journal\" ]; then\n  printf 'missing commitments section'\n  exit 1\nfi\nprintf 'unexpected validate-docs args\\n' >&2\nexit 1\n",
        )
        .unwrap();

        let error = step_c4_1(&dir, 123, 345, &worklog, &journal).unwrap_err();
        assert!(error.contains("Worklog: FAIL: missing receipt summary"));
        assert!(error.contains("Journal: FAIL: missing commitments section"));

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("---ARG---\nC4.1\n"));
        assert!(args.contains("Worklog validation: FAIL: missing receipt summary"));
        assert!(args.contains("Journal validation: FAIL: missing commitments section"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn step_c4_7_posts_safe_to_advance_to_on_success() {
        let dir = setup_temp_repo("step-c4-7-success");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        fs::write(
            dir.join("tools/verify-review-events"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'Verification report\\n  Result: All 2 PRs verified. Safe to advance marker to 345.\\n'\n",
        )
        .unwrap();

        step_c4_7_with_timeout(&dir, 123, 1).unwrap();

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("---ARG---\nC4.7\n"));
        assert!(args.contains("---ARG---\nVerify review events\n"));
        assert!(args.contains("safe_to_advance_to: 345"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn step_c4_7_posts_safe_to_advance_to_from_json_output() {
        let dir = setup_temp_repo("step-c4-7-json-success");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        fs::write(
            dir.join("tools/verify-review-events"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' '{\"safe_to_advance_to\":344}'\n",
        )
        .unwrap();

        step_c4_7_with_timeout(&dir, 123, 1).unwrap();

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("safe_to_advance_to: 344"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn parse_verify_review_events_safe_to_advance_to_accepts_marker_stays_prefix() {
        let value = parse_verify_review_events_safe_to_advance_to(
            "Verification report\n  Result: Verification failed for cycle 345. Marker stays at 344.\n",
        )
        .unwrap();

        assert_eq!(value, 344);
    }

    #[test]
    fn step_c4_7_timeout_posts_warning_and_returns_err() {
        let dir = setup_temp_repo("step-c4-7-timeout");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        fs::write(
            dir.join("tools/verify-review-events"),
            "#!/usr/bin/env bash\nset -euo pipefail\nsleep 2\n",
        )
        .unwrap();

        let error = step_c4_7_with_timeout(&dir, 123, 1).unwrap_err();
        assert!(error.contains("timed out"));

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("---ARG---\nC4.7\n"));
        assert!(args.contains("timed out after 1 seconds"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn detect_prior_gate_failures_parses_c4_1_fail_step_comments() {
        let dir = setup_temp_repo("detect-prior-gate-failures-c4-1");
        let bin_dir = dir.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let gh_path = bin_dir.join("gh");
        write_gh_script(
            &gh_path,
            123,
            &["> **[main-orchestrator]** | Cycle 345\n\n### Step C4.1 — Documentation validation\n\nWorklog validation: FAIL: mismatch in receipts\nJournal validation: PASS"],
            false,
        );
        make_executable(&gh_path);

        let failures = with_path_prefix(&bin_dir, || detect_prior_gate_failures(&dir, 123));

        assert_eq!(failures, vec!["C4.1 FAIL: mismatch in receipts".to_string()]);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn detect_prior_gate_failures_parses_c5_5_fail_step_comments() {
        let dir = setup_temp_repo("detect-prior-gate-failures-c5-5");
        let bin_dir = dir.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let gh_path = bin_dir.join("gh");
        write_gh_script(
            &gh_path,
            123,
            &["> **[main-orchestrator]** | Cycle 345\n\n### Step C5.5 — Final pipeline gate\n\nPipeline: FAIL (1 blocking finding)\n- gate_failure_reason: doc-validation stale"],
            false,
        );
        make_executable(&gh_path);

        let failures = with_path_prefix(&bin_dir, || detect_prior_gate_failures(&dir, 123));

        assert_eq!(failures, vec!["C5.5 FAIL: doc-validation stale".to_string()]);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn detect_prior_gate_failures_returns_empty_vec_when_no_failures_exist() {
        let dir = setup_temp_repo("detect-prior-gate-failures-empty");
        let bin_dir = dir.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let gh_path = bin_dir.join("gh");
        write_gh_script(
            &gh_path,
            123,
            &["> **[main-orchestrator]** | Cycle 345\n\n### Step C4.1 — Documentation validation\n\nWorklog validation: PASS\nJournal validation: PASS"],
            false,
        );
        make_executable(&gh_path);

        let failures = with_path_prefix(&bin_dir, || detect_prior_gate_failures(&dir, 123));

        assert!(failures.is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn step_c6_5_passes_prior_gate_failures_to_write_entry() {
        let dir = setup_temp_repo("step-c6-5-prior-gate-failures");
        fs::create_dir_all(dir.join("docs/worklog/2026-03-25")).unwrap();
        fs::write(
            dir.join("docs/worklog/2026-03-25/122700-cycle-345-summary.md"),
            "# Cycle 345\n\n### Issues processed\n\n- None.\n\n## Cycle state\n\n- **In-flight agent sessions**: 0\n- **Pipeline status**: FAIL (1 blocking finding)\n- **Publish gate**: published\n\n## Next steps\n\n1. None.\n",
        )
        .unwrap();
        fs::write(
            dir.join("docs/state.json"),
            serde_json::to_string_pretty(&json!({
                "cycle_phase": {
                    "cycle": 345,
                    "phase": "close_out",
                    "phase_entered_at": "2026-03-25T00:00:00Z"
                },
                "publish_gate": { "status": "blocked pending review" },
                "in_flight_sessions": 1,
                "agent_sessions": [
                    {
                        "issue": 1470,
                        "title": "[Cycle Review] Cycle 345 end-of-cycle review",
                        "status": "in_flight",
                        "dispatched_at": "2026-03-25T03:00:00Z"
                    }
                ]
            }))
            .unwrap(),
        )
        .unwrap();
        let worklog = dir.join("docs/worklog/2026-03-25/122700-cycle-345-summary.md");
        let args_path = dir.join("write-entry-args.txt");
        let args_path_quoted = shell_single_quote(&args_path);
        fs::write(
            dir.join("tools/write-entry"),
            format!(
                "#!/usr/bin/env bash\nset -euo pipefail\n{{\nfor arg in \"$@\"; do\nprintf '%s\\n' \"$arg\"\ndone\n}} > {args_path_quoted}\npython - \"$@\" <<'PY'\nimport sys\nfrom pathlib import Path\nargs = sys.argv[1:]\nfor index, arg in enumerate(args):\n    if arg == '--worklog-file' and index + 1 < len(args):\n        worklog = Path(args[index + 1])\n        worklog.write_text(worklog.read_text() + '\\n<!-- patched -->\\n')\n        break\nprint(worklog)\nPY\n",
            ),
        )
        .unwrap();
        write_post_step_capture_script(&dir, &dir.join("post-step-args.txt"));
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["add", "."])
            .output()
            .unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["commit", "-m", "initial test state"])
            .output()
            .unwrap();

        step_c6_5(
            &dir,
            123,
            345,
            &worklog,
            "PASS (1 warning)",
            &["C4.1 FAIL: mismatch".to_string(), "C5.5 FAIL: doc-validation".to_string()],
        )
        .unwrap();

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("--prior-gate-failures"));
        assert!(args.contains("C4.1 FAIL: mismatch,C5.5 FAIL: doc-validation"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn close_out_run_continues_after_c4_7_failure() {
        let (dir, remote) = setup_temp_repo_with_remote("close-out-c4-7-warning");
        fs::create_dir_all(dir.join("docs/worklog/2026-03-25")).unwrap();
        fs::create_dir_all(dir.join("docs/journal")).unwrap();
        fs::write(
            dir.join("docs/worklog/2026-03-25/122700-cycle-345-summary.md"),
            "# Cycle 345\n\n### Issues processed\n\n- None.\n\n## Pre-dispatch state\n\n*Snapshot before review dispatch — final counters may differ after C6.*\n- **In-flight agent sessions**: 0\n- **Pipeline status**: PASS\n- **Publish gate**: published\n\n## Next steps\n\n1. None.\n",
        )
        .unwrap();
        fs::write(dir.join("docs/journal/2026-03-25.md"), "# Journal\n").unwrap();
        fs::write(
            dir.join("docs/state.json"),
            serde_json::to_string_pretty(&json!({
                "cycle_phase": {
                    "cycle": 345,
                    "phase": "close_out",
                    "phase_entered_at": "2026-03-25T00:00:00Z"
                },
                "last_cycle": {
                    "number": 345,
                    "timestamp": "2026-03-24T00:00:00Z"
                },
                "field_inventory": {
                    "fields": {
                        "cycle_phase": {
                            "last_refreshed": "cycle 344"
                        }
                    }
                },
                "tool_pipeline": {
                    "status": "phase_5_active"
                },
                "publish_gate": {
                    "status": "published"
                },
                "in_flight_sessions": 0,
                "copilot_metrics": {
                    "total_dispatches": 1,
                    "produced_pr": 1,
                    "merged": 1,
                    "pr_merge_rate": "100.0%",
                    "in_flight": 0
                },
                "agent_sessions": []
            }))
            .unwrap(),
        )
        .unwrap();
        let args_path = dir.join("post-step-args.txt");
        write_post_step_append_capture_script(&dir, &args_path);
        write_write_entry_patch_script(&dir);
        fs::write(
            dir.join("tools/validate-docs"),
            "#!/usr/bin/env bash\nset -euo pipefail\n",
        )
        .unwrap();
        fs::write(
            dir.join("tools/verify-review-events"),
            "#!/usr/bin/env bash\nset -euo pipefail\necho 'simulated verify-review-events failure' >&2\nexit 1\n",
        )
        .unwrap();
        fs::write(
            dir.join("tools/receipt-validate"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"result\":\"pass\",\"canonical_receipts\":1,\"worklog_receipts\":1,\"genuinely_missing\":0}'\n",
        )
        .unwrap();
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"pass\",\"has_blocking_findings\":false,\"steps\":[{\"name\":\"doc-validation\",\"status\":\"warn\"}]}'\n",
        )
        .unwrap();
        fs::write(
            dir.join("tools/dispatch-review"),
            format!(
                "#!/usr/bin/env bash\nset -euo pipefail\npython - <<'PY'\nimport json\nfrom pathlib import Path\nstate_path = Path({state_path:?})\nstate = json.loads(state_path.read_text())\nstate['in_flight_sessions'] = 1\nstate['publish_gate']['status'] = 'blocked pending review'\nstate['dispatch_log_latest'] = '#1470 [Cycle Review] Cycle 345 end-of-cycle review (cycle 345)'\nstate['agent_sessions'] = [{{'issue': 1470, 'title': '[Cycle Review] Cycle 345 end-of-cycle review', 'status': 'in_flight', 'dispatched_at': '2026-03-25T03:00:00Z'}}]\nstate_path.write_text(json.dumps(state, indent=2) + '\\n')\nPY\ngit -C {repo:?} add docs/state.json\ngit -C {repo:?} commit -m 'state(record-dispatch): #1470 dispatched [cycle 345]' >/dev/null\nprintf '%s\\n' 'Created review issue #1470 from orchestrator issue #123: https://github.com/EvaLok/schema-org-json-ld/issues/1470'\n",
                state_path = dir.join("docs/state.json"),
                repo = dir,
            ),
        )
        .unwrap();

        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["add", "."])
            .output()
            .unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["commit", "-m", "initial test state"])
            .output()
            .unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["push", "-u", "origin", "master"])
            .output()
            .unwrap();

        let bin_dir = dir.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let gh_path = bin_dir.join("gh");
        write_gh_script(
            &gh_path,
            123,
            &["> **[main-orchestrator]** | Cycle 345\n\n### Step C4.1 — Documentation validation\n\nWorklog validation: FAIL: mismatch in receipts\nJournal validation: PASS"],
            true,
        );
        make_executable(&gh_path);

        with_path_prefix(&bin_dir, || run(&dir, 123, Some(345), false)).unwrap();

        let args = fs::read_to_string(&args_path).unwrap();
        let c4_5 = args.find("---ARG---\nC4.5\n").unwrap();
        let c4_7 = args.find("---ARG---\nC4.7\n").unwrap();
        let c5 = args.find("---ARG---\nC5\n").unwrap();
        let c6_5 = args.find("---ARG---\nC6.5\n").unwrap();
        assert!(c4_5 < c4_7);
        assert!(c4_7 < c5);
        assert!(c5 < c6_5);
        assert!(args.contains("simulated verify-review-events failure"));
        assert!(args.contains("---ARG---\nC5.5\n"));
        assert!(args.contains("Cycle 345 close-out complete."));
        assert!(args.contains("- Pipeline: PASS (1 warning)"));

        let state = state_schema::read_state_value(&dir).unwrap();
        assert_eq!(
            state.pointer("/cycle_phase/phase"),
            Some(&json!("complete"))
        );

        let _ = fs::remove_dir_all(&dir);
        let _ = fs::remove_dir_all(&remote);
    }

    #[test]
    fn close_out_run_patches_worklog_state_after_review_dispatch() {
        let (dir, remote) = setup_temp_repo_with_remote("close-out-worklog-state-patch");
        fs::create_dir_all(dir.join("docs/worklog/2026-03-25")).unwrap();
        fs::create_dir_all(dir.join("docs/journal")).unwrap();
        fs::write(
            dir.join("docs/worklog/2026-03-25/122700-cycle-345-summary.md"),
            "# Cycle 345\n\n### Issues processed\n\n- None.\n\n## Pre-dispatch state\n\n*Snapshot before review dispatch — final counters may differ after C6.*\n- **In-flight agent sessions**: 0\n- **Pipeline status**: PASS\n- **Publish gate**: published\n\n## Next steps\n\n1. None.\n",
        )
        .unwrap();
        fs::write(dir.join("docs/journal/2026-03-25.md"), "# Journal\n").unwrap();
        fs::write(
            dir.join("docs/state.json"),
            serde_json::to_string_pretty(&json!({
                "cycle_phase": {
                    "cycle": 345,
                    "phase": "close_out",
                    "phase_entered_at": "2026-03-25T00:00:00Z"
                },
                "last_cycle": {
                    "number": 345,
                    "timestamp": "2026-03-24T00:00:00Z"
                },
                "field_inventory": {
                    "fields": {
                        "cycle_phase": {
                            "last_refreshed": "cycle 344"
                        }
                    }
                },
                "tool_pipeline": {
                    "status": "phase_5_active"
                },
                "publish_gate": {
                    "status": "published"
                },
                "in_flight_sessions": 0,
                "copilot_metrics": {
                    "total_dispatches": 1,
                    "produced_pr": 1,
                    "merged": 1,
                    "pr_merge_rate": "100.0%",
                    "in_flight": 0
                },
                "agent_sessions": []
            }))
            .unwrap(),
        )
        .unwrap();
        let args_path = dir.join("post-step-args.txt");
        write_post_step_append_capture_script(&dir, &args_path);
        write_write_entry_patch_script(&dir);
        fs::write(
            dir.join("tools/validate-docs"),
            "#!/usr/bin/env bash\nset -euo pipefail\n",
        )
        .unwrap();
        fs::write(
            dir.join("tools/verify-review-events"),
            "#!/usr/bin/env bash\nset -euo pipefail\n",
        )
        .unwrap();
        fs::write(
            dir.join("tools/receipt-validate"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"result\":\"pass\",\"canonical_receipts\":1,\"worklog_receipts\":1,\"genuinely_missing\":0}'\n",
        )
        .unwrap();
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"pass\",\"has_blocking_findings\":false,\"steps\":[{\"name\":\"doc-validation\",\"status\":\"warn\"}]}'\n",
        )
        .unwrap();
        fs::write(
            dir.join("tools/dispatch-review"),
            format!(
                "#!/usr/bin/env bash\nset -euo pipefail\npython - <<'PY'\nimport json\nfrom pathlib import Path\nstate_path = Path({state_path:?})\nstate = json.loads(state_path.read_text())\nstate['in_flight_sessions'] = 1\nstate['dispatch_log_latest'] = '#1470 [Cycle Review] Cycle 345 end-of-cycle review (cycle 345)'\nstate['agent_sessions'] = [{{'issue': 1470, 'title': '[Cycle Review] Cycle 345 end-of-cycle review', 'status': 'in_flight', 'dispatched_at': '2026-03-25T03:00:00Z'}}]\nstate_path.write_text(json.dumps(state, indent=2) + '\\n')\nPY\ngit -C {repo:?} add docs/state.json\ngit -C {repo:?} commit -m 'state(record-dispatch): #1470 dispatched [cycle 345]' >/dev/null\nprintf '%s\\n' 'Created review issue #1470 from orchestrator issue #123: https://github.com/EvaLok/schema-org-json-ld/issues/1470'\n",
                state_path = dir.join("docs/state.json"),
                repo = dir,
            ),
        )
        .unwrap();

        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["add", "."])
            .output()
            .unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["commit", "-m", "initial test state"])
            .output()
            .unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["push", "-u", "origin", "master"])
            .output()
            .unwrap();

        let bin_dir = dir.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let gh_path = bin_dir.join("gh");
        write_gh_script(
            &gh_path,
            123,
            &["> **[main-orchestrator]** | Cycle 345\n\n### Step C4.1 — Documentation validation\n\nWorklog validation: FAIL: mismatch in receipts\nJournal validation: PASS"],
            true,
        );
        make_executable(&gh_path);

        with_path_prefix(&bin_dir, || run(&dir, 123, Some(345), false)).unwrap();

        let worklog =
            fs::read_to_string(dir.join("docs/worklog/2026-03-25/122700-cycle-345-summary.md"))
                .unwrap();
        assert!(worklog.contains("## Cycle state"));
        assert!(!worklog.contains("## Pre-dispatch state"));
        assert!(!worklog.contains("Snapshot before review dispatch"));
        assert!(worklog.contains(
            "### Issues processed\n\n- [#1470](https://github.com/EvaLok/schema-org-json-ld/issues/1470): [Cycle Review] Cycle 345 end-of-cycle review (in_flight)\n\n## Cycle state"
        ));
        assert!(worklog.contains("- **In-flight agent sessions**: 0"));
        assert!(worklog.contains("- **In-flight agent sessions (post-dispatch)**: 1"));
        assert!(worklog.contains("- **Pipeline status**: PASS"));
        assert!(worklog.contains("- **Pipeline status (post-dispatch)**: PASS (1 warning)"));
        assert!(worklog.contains(
            "- **Close-out gate failures**: C4.1 FAIL: mismatch in receipts"
        ));
        assert!(!worklog.contains("phase_5_active"));
        assert!(!worklog.contains("- **Copilot metrics**:"));
        assert!(worklog.contains("- **Publish gate**: published"));
        assert!(worklog.contains(
            "## Next steps\n\n1. None.\n\n## Next steps (post-dispatch)\n\n1. Review and iterate on PR from [#1470](https://github.com/EvaLok/schema-org-json-ld/issues/1470) ([Cycle Review] Cycle 345 end-of-cycle review) when Copilot completes\n"
        ));

        let log_output = Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["log", "--format=%s", "-3"])
            .output()
            .unwrap();
        let log = String::from_utf8_lossy(&log_output.stdout);
        assert!(log
            .contains("docs(worklog): refresh cycle 345 state after review dispatch [cycle 345]"));

        let _ = fs::remove_dir_all(&dir);
        let _ = fs::remove_dir_all(&remote);
    }

    #[test]
    fn close_out_run_preserves_existing_failed_pipeline_status_and_adds_post_dispatch_addendum() {
        let (dir, remote) = setup_temp_repo_with_remote("close-out-worklog-status-immutable");
        fs::create_dir_all(dir.join("docs/worklog/2026-03-25")).unwrap();
        fs::create_dir_all(dir.join("docs/journal")).unwrap();
        fs::write(
            dir.join("docs/worklog/2026-03-25/122700-cycle-345-summary.md"),
            "# Cycle 345\n\n### Issues processed\n\n- None.\n\n## Pre-dispatch state\n\n*Snapshot before review dispatch — final counters may differ after C6.*\n- **In-flight agent sessions**: 0\n- **Pipeline status**: FAIL (1 blocking finding)\n- **Publish gate**: published\n\n## Next steps\n\n1. Plan next dispatch\n",
        )
        .unwrap();
        fs::write(dir.join("docs/journal/2026-03-25.md"), "# Journal\n").unwrap();
        fs::write(
            dir.join("docs/state.json"),
            serde_json::to_string_pretty(&json!({
                "cycle_phase": {
                    "cycle": 345,
                    "phase": "close_out",
                    "phase_entered_at": "2026-03-25T00:00:00Z"
                },
                "last_cycle": {
                    "number": 345,
                    "timestamp": "2026-03-24T00:00:00Z"
                },
                "field_inventory": {
                    "fields": {
                        "cycle_phase": {
                            "last_refreshed": "cycle 344"
                        }
                    }
                },
                "tool_pipeline": {
                    "status": "phase_5_active"
                },
                "publish_gate": {
                    "status": "published"
                },
                "in_flight_sessions": 0,
                "copilot_metrics": {
                    "total_dispatches": 1,
                    "produced_pr": 1,
                    "merged": 1,
                    "pr_merge_rate": "100.0%",
                    "in_flight": 0
                },
                "agent_sessions": []
            }))
            .unwrap(),
        )
        .unwrap();
        let args_path = dir.join("post-step-args.txt");
        write_post_step_append_capture_script(&dir, &args_path);
        write_write_entry_patch_script(&dir);
        fs::write(
            dir.join("tools/validate-docs"),
            "#!/usr/bin/env bash\nset -euo pipefail\n",
        )
        .unwrap();
        fs::write(
            dir.join("tools/verify-review-events"),
            "#!/usr/bin/env bash\nset -euo pipefail\n",
        )
        .unwrap();
        fs::write(
            dir.join("tools/receipt-validate"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"result\":\"pass\",\"canonical_receipts\":1,\"worklog_receipts\":1,\"genuinely_missing\":0}'\n",
        )
        .unwrap();
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"pass\",\"has_blocking_findings\":false,\"steps\":[{\"name\":\"doc-validation\",\"status\":\"warn\"}]}'\n",
        )
        .unwrap();
        fs::write(
            dir.join("tools/dispatch-review"),
            format!(
                "#!/usr/bin/env bash\nset -euo pipefail\npython - <<'PY'\nimport json\nfrom pathlib import Path\nstate_path = Path({state_path:?})\nstate = json.loads(state_path.read_text())\nstate['in_flight_sessions'] = 1\nstate['publish_gate']['status'] = 'blocked pending review'\nstate['dispatch_log_latest'] = '#1470 [Cycle Review] Cycle 345 end-of-cycle review (cycle 345)'\nstate['agent_sessions'] = [{{'issue': 1470, 'title': '[Cycle Review] Cycle 345 end-of-cycle review', 'status': 'in_flight', 'dispatched_at': '2026-03-25T03:00:00Z'}}]\nstate_path.write_text(json.dumps(state, indent=2) + '\\n')\nPY\ngit -C {repo:?} add docs/state.json\ngit -C {repo:?} commit -m 'state(record-dispatch): #1470 dispatched [cycle 345]' >/dev/null\nprintf '%s\\n' 'Created review issue #1470 from orchestrator issue #123: https://github.com/EvaLok/schema-org-json-ld/issues/1470'\n",
                state_path = dir.join("docs/state.json"),
                repo = dir,
            ),
        )
        .unwrap();

        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["add", "."])
            .output()
            .unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["commit", "-m", "initial test state"])
            .output()
            .unwrap();
        Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["push", "-u", "origin", "master"])
            .output()
            .unwrap();

        let bin_dir = dir.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let gh_path = bin_dir.join("gh");
        write_gh_script(&gh_path, 123, &[], true);
        make_executable(&gh_path);

        with_path_prefix(&bin_dir, || run(&dir, 123, Some(345), false)).unwrap();

        let worklog =
            fs::read_to_string(dir.join("docs/worklog/2026-03-25/122700-cycle-345-summary.md"))
                .unwrap();
        assert!(worklog.contains(
            "### Issues processed\n\n- [#1470](https://github.com/EvaLok/schema-org-json-ld/issues/1470): [Cycle Review] Cycle 345 end-of-cycle review (in_flight)\n\n## Cycle state"
        ));
        assert!(worklog.contains("- **In-flight agent sessions**: 0"));
        assert!(worklog.contains("- **In-flight agent sessions (post-dispatch)**: 1"));
        assert!(worklog.contains("- **Pipeline status**: FAIL (1 blocking finding)"));
        assert!(worklog.contains("- **Pipeline status (post-dispatch)**: PASS (1 warning)"));
        assert!(worklog.contains("- **Publish gate**: published"));
        assert!(worklog.contains("- **Publish gate (post-dispatch)**: blocked pending review"));
        assert!(worklog.contains("## Next steps\n\n1. Plan next dispatch\n\n## Next steps (post-dispatch)\n\n1. Review and iterate on PR from [#1470](https://github.com/EvaLok/schema-org-json-ld/issues/1470) ([Cycle Review] Cycle 345 end-of-cycle review) when Copilot completes\n"));
        assert_eq!(
            worklog.matches("- **In-flight agent sessions**:").count(),
            1
        );
        assert_eq!(
            worklog
                .matches("- **In-flight agent sessions (post-dispatch)**:")
                .count(),
            1
        );
        assert_eq!(worklog.matches("- **Publish gate**:").count(), 1);
        assert_eq!(
            worklog
                .matches("- **Publish gate (post-dispatch)**:")
                .count(),
            1
        );
        assert_eq!(worklog.matches("- **Pipeline status**:").count(), 1);
        assert_eq!(
            worklog
                .matches("- **Pipeline status (post-dispatch)**:")
                .count(),
            1
        );

        let _ = fs::remove_dir_all(&dir);
        let _ = fs::remove_dir_all(&remote);
    }

    #[test]
    fn step_c4_5_posts_adr_check_with_existing_adrs() {
        let dir = setup_temp_repo("step-c4-5");
        fs::create_dir_all(dir.join("doc/adr")).unwrap();
        fs::write(dir.join("doc/adr/0001-example.md"), "# ADR 1\n").unwrap();
        fs::write(dir.join("doc/adr/0002-example.md"), "# ADR 2\n").unwrap();

        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);

        step_c4_5(&dir, 123).unwrap();

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("---ARG---\n--issue\n"));
        assert!(args.contains("---ARG---\n123\n"));
        assert!(args.contains("---ARG---\n--step\n"));
        assert!(args.contains("---ARG---\nC4.5\n"));
        assert!(args.contains("---ARG---\n--title\n"));
        assert!(args.contains("---ARG---\nADR check\n"));
        assert!(args.contains("2 ADRs in doc/adr/"));
        assert!(args.contains("0001-example.md"));
        assert!(args.contains("0002-example.md"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn step_c8_posts_pipeline_summary() {
        let dir = setup_temp_repo("step-c8-pipeline-fail");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);

        let bin_dir = dir.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let gh_path = bin_dir.join("gh");
        write_gh_script(&gh_path, 123, &[], true);
        make_executable(&gh_path);

        let review_info = ReviewInfo {
            issue_number: 1470,
            issue_url: "https://github.com/EvaLok/schema-org-json-ld/issues/1470".to_string(),
        };

        with_path_prefix(&bin_dir, || {
            step_c8(
                &dir,
                123,
                345,
                Some(&review_info),
                "FAIL (1 blocking: doc-validation)",
            )
        })
        .unwrap();

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("---ARG---\nC8\n"));
        assert!(args.contains("Cycle 345 close-out complete."));
        assert!(args.contains("- Review: dispatched as #1470 (https://github.com/EvaLok/schema-org-json-ld/issues/1470)"));
        assert!(args.contains("- Pipeline: FAIL (1 blocking: doc-validation)"));
        assert!(args.contains("- All close-out steps completed by cycle-runner"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn copilot_available_when_no_sessions() {
        let state = json!({"agent_sessions": []});
        assert!(is_copilot_available(&state));
    }

    #[test]
    fn copilot_available_when_recent_success() {
        let state = json!({
            "agent_sessions": [
                {"dispatched_at": "2026-03-27T01:00:00Z", "status": "merged"},
                {"dispatched_at": "2026-03-27T02:00:00Z", "status": "failed"},
                {"dispatched_at": "2026-03-27T03:00:00Z", "status": "failed"}
            ]
        });
        assert!(is_copilot_available(&state));
    }

    #[test]
    fn copilot_unavailable_when_all_recent_failed() {
        let state = json!({
            "agent_sessions": [
                {"dispatched_at": "2026-03-27T01:00:00Z", "status": "merged"},
                {"dispatched_at": "2026-03-27T02:00:00Z", "status": "failed"},
                {"dispatched_at": "2026-03-27T03:00:00Z", "status": "failed"},
                {"dispatched_at": "2026-03-27T04:00:00Z", "status": "closed_without_pr"}
            ]
        });
        assert!(!is_copilot_available(&state));
    }

    #[test]
    fn copilot_available_with_mixed_failure_statuses() {
        let state = json!({
            "agent_sessions": [
                {"dispatched_at": "2026-03-27T02:00:00Z", "status": "failed"},
                {"dispatched_at": "2026-03-27T03:00:00Z", "status": "in_flight"},
                {"dispatched_at": "2026-03-27T04:00:00Z", "status": "failed"}
            ]
        });
        assert!(is_copilot_available(&state));
    }

    #[test]
    fn step_c8_handles_deferred_review() {
        let dir = setup_temp_repo("step-c8-deferred");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);

        let bin_dir = dir.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let gh_path = bin_dir.join("gh");
        write_gh_script(&gh_path, 123, &[], true);
        make_executable(&gh_path);

        with_path_prefix(&bin_dir, || {
            step_c8(&dir, 123, 345, None, "PASS (0 warnings)")
        })
        .unwrap();

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("Cycle 345 close-out complete."));
        assert!(args.contains("- Review: deferred (Copilot unavailable)"));
        assert!(args.contains("- Pipeline: PASS (0 warnings)"));

        let _ = fs::remove_dir_all(&dir);
    }
}
