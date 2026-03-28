use crate::git;
use crate::review_body;
use crate::runner;
use crate::steps;
use serde_json::Value;
use std::fs;
use std::path::Path;
use std::process::Command;
use state_schema::{
    commit_state_json, current_cycle_from_state, read_state_value, transition_cycle_phase,
    write_state_value, StateJson,
};

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";
const VERIFY_REVIEW_EVENTS_TIMEOUT_SECS: u64 = 30;

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
    let (pipeline_passed, pipeline_summary) = step_c5_5(repo_root, issue)?;

    // C5.6: Stabilization counter
    step_c5_6(repo_root, issue, cycle, pipeline_passed)?;

    // C6: Review dispatch (may be skipped if Copilot is unavailable)
    let review_info = step_c6(repo_root, issue, cycle)?;

    // C6.5: Refresh worklog state after review dispatch
    step_c6_5(repo_root, issue, cycle, &worklog, &pipeline_summary)?;

    // C7: Push
    step_c7(repo_root, issue)?;

    // C8: Close issue
    step_c8(repo_root, issue, cycle, review_info.as_ref(), &pipeline_summary)?;
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
        format!(
            "FAIL: {}",
            runner::stderr_text(&worklog_output)
        )
    };

    let journal_str = journal.to_string_lossy().to_string();
    let journal_output =
        runner::run_tool(repo_root, "validate-docs", &["journal", "--file", &journal_str])?;
    let journal_ok = journal_output.status.success();
    let journal_detail = if journal_ok {
        "PASS".to_string()
    } else {
        format!(
            "FAIL: {}",
            runner::stderr_text(&journal_output)
        )
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
                    .map_err(|error| format!("failed to read entry in {}: {}", adr_dir.display(), error))
                    .and_then(|entry| {
                        let file_type = entry.file_type().map_err(|error| {
                            format!(
                                "failed to read file type for {}: {}",
                                entry.path().display(),
                                error
                            )
                        })?;
                        let path = entry.path();
                        let name = entry
                            .file_name()
                            .into_string()
                            .map_err(|_| format!("ADR path is not valid UTF-8: {}", path.display()))?;
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

fn step_c4_7_with_timeout(repo_root: &Path, issue: u64, timeout_seconds: u64) -> Result<(), String> {
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
            steps::post_step(repo_root, issue, "C4.7", "Verify review events", &body, false)?;
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

    steps::post_step(repo_root, issue, "C4.7", "Verify review events", &body, false)?;

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

fn step_c5_1(
    repo_root: &Path,
    issue: u64,
    cycle: u64,
    worklog: &Path,
) -> Result<(), String> {
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
    steps::post_step(
        repo_root,
        issue,
        "C5.1",
        "Receipt validation",
        &body,
        false,
    )?;

    Ok(())
}

fn step_c5_5(repo_root: &Path, issue: u64) -> Result<(bool, String), String> {
    eprintln!("C5.5: Final pipeline gate...");

    let output = runner::run_tool(repo_root, "pipeline-check", &["--json"])?;
    let exit_ok = output.status.success();
    let stdout = runner::stdout_text(&output);
    let stderr = runner::stderr_text(&output);
    let exit_code = output.status.code().unwrap_or(-1);

    let (passed, pipeline_summary, body) = match parse_pipeline_gate_report(&stdout) {
        Ok(report) => {
            let passed = exit_ok && report.overall == "pass" && !report.has_blocking_findings;
            let pipeline_summary = format_pipeline_summary(&report);
            let mut body = format!(
                "Pipeline: {}\n- exit_code: {}\n- overall: {}\n- has_blocking_findings: {}",
                pipeline_summary,
                exit_code,
                report.overall,
                report.has_blocking_findings
            );
            if !stdout.is_empty() {
                body.push_str(&format!("\n- raw_json: {}", stdout));
            }
            if !stderr.is_empty() {
                body.push_str(&format!("\n- stderr: {}", stderr));
            }
            (passed, pipeline_summary, body)
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
            (false, pipeline_summary, body)
        }
    };

    steps::post_step(
        repo_root,
        issue,
        "C5.5",
        "Final pipeline gate",
        &body,
        false,
    )?;

    if !passed {
        return Err(
            "Pipeline check failed at C5.5 — fix issues and re-run close-out".to_string(),
        );
    }

    Ok((true, pipeline_summary))
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
        .is_some_and(|arr| {
            arr.iter().any(|v| v.as_u64() == Some(cycle))
        });

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
    let is_stabilization = state
        .pointer("/project_mode/mode")
        .and_then(Value::as_str)
        == Some("stabilization");

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
) -> Result<(), String> {
    eprintln!("C6.5: Refreshing worklog state after review dispatch...");

    let state_value = read_state_value(repo_root)?;
    let state: StateJson = serde_json::from_value(state_value)
        .map_err(|error| format!("failed to parse docs/state.json after C6: {}", error))?;
    let in_flight = state
        .in_flight_sessions
        .ok_or_else(|| "missing in_flight_sessions in state.json".to_string())
        .and_then(|value| {
            u64::try_from(value)
                .map_err(|_| "in_flight_sessions must be non-negative in state.json".to_string())
        })?;
    let publish_gate = state
        .publish_gate()?
        .status
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "missing publish_gate.status in state.json".to_string())?
        .to_string();
    let next_steps = derive_patch_pipeline_next_steps(&state)?;
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
    ];
    for next_step in &next_steps {
        patch_args.push("--next-steps".to_string());
        patch_args.push(next_step.clone());
    }
    let patch_args_refs: Vec<&str> = patch_args.iter().map(String::as_str).collect();

    let output = runner::run_tool(
        repo_root,
        "write-entry",
        &patch_args_refs,
    )?;
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
    steps::post_step(repo_root, issue, "C6.5", "Refresh worklog state", &body, false)?;
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

fn step_c8(
    repo_root: &Path,
    issue: u64,
    cycle: u64,
    review_info: Option<&ReviewInfo>,
    pipeline_summary: &str,
) -> Result<(), String> {
    eprintln!("C8: Closing orchestrator issue...");

    let review_line = match review_info {
        Some(info) => format!("- Review: dispatched as #{} ({})", info.issue_number, info.issue_url),
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
            let title = session
                .get("title")
                .and_then(Value::as_str)
                .unwrap_or("");
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

    let url = stdout
        .rsplit(": ")
        .next()
        .unwrap_or("")
        .trim()
        .to_string();

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
            "#!/usr/bin/env bash\nset -euo pipefail\npython - \"$@\" <<'PY'\nimport sys\nfrom pathlib import Path\nargs = sys.argv[1:]\nif not args or args[0] != 'patch-pipeline':\n    raise SystemExit(f'unexpected write-entry args: {args}')\nvalues = {}\nlist_values = {}\ni = 1\nwhile i < len(args):\n    key = args[i]\n    if key == '--next-steps':\n        if i + 1 >= len(args):\n            raise SystemExit(f'missing value for {key}')\n        list_values.setdefault(key, []).append(args[i + 1])\n        i += 2\n        continue\n    if i + 1 >= len(args):\n        raise SystemExit(f'missing value for {key}')\n    values[key] = args[i + 1]\n    i += 2\nworklog = Path(values['--worklog-file'])\nlines = worklog.read_text().splitlines()\nfor index, line in enumerate(lines):\n    if line == '## Pre-dispatch state':\n        lines[index] = '## ' + values['--section-title']\n        break\nelse:\n    raise SystemExit('missing state heading')\nlines = [line for line in lines if line != '*Snapshot before review dispatch — final counters may differ after C6.*']\nreplacements = {\n    '- **In-flight agent sessions**: ': values['--in-flight'],\n    '- **Pipeline status**: ': values['--status'],\n    '- **Publish gate**: ': values['--publish-gate'],\n}\nfor prefix, value in replacements.items():\n    for index, line in enumerate(lines):\n        if line.startswith(prefix):\n            lines[index] = prefix + value\n            break\n    else:\n        raise SystemExit(f'missing line for {prefix}')\nlines = [line for line in lines if not line.startswith('- **Copilot metrics**: ')]\nif '--next-steps' in list_values:\n    for index, line in enumerate(lines):\n        if line == '## Next steps':\n            next_start = index + 1\n            break\n    else:\n        raise SystemExit('missing next steps heading')\n    next_end = len(lines)\n    for index in range(next_start, len(lines)):\n        if lines[index].startswith('## '):\n            next_end = index\n            break\n    replacement = ['']\n    for step_index, step in enumerate(list_values['--next-steps'], start=1):\n        replacement.append(f'{step_index}. {step}')\n    lines[next_start:next_end] = replacement\nworklog.write_text('\\n'.join(lines) + '\\n')\nprint(worklog)\nPY\n",
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
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"fail\",\"has_blocking_findings\":false}'\n",
        )
        .unwrap();

        let error = step_c5_5(&dir, 123).unwrap_err();
        assert_eq!(
            error,
            "Pipeline check failed at C5.5 — fix issues and re-run close-out"
        );

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("---ARG---\nC5.5\n"));
        assert!(args.contains("overall: fail"));
        assert!(args.contains("has_blocking_findings: false"));

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
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"pass\",\"has_blocking_findings\":false,\"steps\":[{\"name\":\"doc-validation\",\"status\":\"warn\"}]}'\n",
        )
        .unwrap();

        let (passed, summary) = step_c5_5(&dir, 123).unwrap();
        assert!(passed);
        assert_eq!(summary, "PASS (1 warning)");

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("---ARG---\nC5.5\n"));
        assert!(args.contains("Pipeline: PASS (1 warning)"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn step_c5_5_returns_pipeline_summary_for_blocking_warning_pass() {
        let dir = setup_temp_repo("step-c5-5-blocking-warning-pass");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"pass\",\"has_blocking_findings\":false,\"steps\":[{\"name\":\"worklog-dedup\",\"status\":\"warn\",\"severity\":\"blocking\"},{\"name\":\"doc-validation\",\"status\":\"warn\",\"severity\":\"warning\"}]}'\n",
        )
        .unwrap();

        let (passed, summary) = step_c5_5(&dir, 123).unwrap();
        assert!(passed);
        assert_eq!(summary, "PASS (1 blocking warning, 1 warning)");

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("---ARG---\nC5.5\n"));
        assert!(args.contains("Pipeline: PASS (1 blocking warning, 1 warning)"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn step_c5_5_rejects_zero_exit_when_json_reports_blocking_findings() {
        let dir = setup_temp_repo("step-c5-5-blocking");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"pass\",\"has_blocking_findings\":true}'\n",
        )
        .unwrap();

        let error = step_c5_5(&dir, 123).unwrap_err();
        assert_eq!(
            error,
            "Pipeline check failed at C5.5 — fix issues and re-run close-out"
        );

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("overall: pass"));
        assert!(args.contains("has_blocking_findings: true"));

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn step_c5_5_rejects_zero_exit_when_json_overall_is_unexpected() {
        let dir = setup_temp_repo("step-c5-5-unexpected-overall");
        let args_path = dir.join("post-step-args.txt");
        write_post_step_capture_script(&dir, &args_path);
        fs::write(
            dir.join("tools/pipeline-check"),
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\n' '{\"overall\":\"warning\",\"has_blocking_findings\":false}'\n",
        )
        .unwrap();

        let error = step_c5_5(&dir, 123).unwrap_err();
        assert_eq!(
            error,
            "Pipeline check failed at C5.5 — fix issues and re-run close-out"
        );

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("overall: warning"));
        assert!(args.contains("has_blocking_findings: false"));

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
    fn close_out_run_continues_after_c4_7_failure() {
        let (dir, remote) = setup_temp_repo_with_remote("close-out-c4-7-warning");
        fs::create_dir_all(dir.join("docs/worklog/2026-03-25")).unwrap();
        fs::create_dir_all(dir.join("docs/journal")).unwrap();
        fs::write(
            dir.join("docs/worklog/2026-03-25/122700-cycle-345-summary.md"),
            "# Cycle 345\n\n## Pre-dispatch state\n\n*Snapshot before review dispatch — final counters may differ after C6.*\n- **In-flight agent sessions**: 0\n- **Pipeline status**: PASS\n- **Publish gate**: published\n\n## Next steps\n\n1. None.\n",
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
                "#!/usr/bin/env bash\nset -euo pipefail\npython - <<'PY'\nimport json\nfrom pathlib import Path\nstate_path = Path({state_path:?})\nstate = json.loads(state_path.read_text())\nstate['dispatch_log_latest'] = '#1470 [Cycle Review] Cycle 345 end-of-cycle review (cycle 345)'\nstate['in_flight_sessions'] = 1\nstate['agent_sessions'] = [{{'issue': 1470, 'title': '[Cycle Review] Cycle 345 end-of-cycle review', 'status': 'in_flight'}}]\nstate_path.write_text(json.dumps(state, indent=2) + '\\n')\nPY\ngit -C {repo:?} add docs/state.json\ngit -C {repo:?} commit -m 'state(record-dispatch): #1470 dispatched [cycle 345]' >/dev/null\nprintf '%s\\n' 'Created review issue #1470 from orchestrator issue #123: https://github.com/EvaLok/schema-org-json-ld/issues/1470'\n",
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
        fs::write(
            &gh_path,
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1\" = \"issue\" ] && [ \"$2\" = \"close\" ]; then\n  exit 0\nfi\nprintf 'unexpected gh invocation\\n' >&2\nexit 1\n",
        )
        .unwrap();
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
            "# Cycle 345\n\n## Pre-dispatch state\n\n*Snapshot before review dispatch — final counters may differ after C6.*\n- **In-flight agent sessions**: 0\n- **Pipeline status**: PASS\n- **Publish gate**: published\n\n## Next steps\n\n1. None.\n",
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
                "#!/usr/bin/env bash\nset -euo pipefail\npython - <<'PY'\nimport json\nfrom pathlib import Path\nstate_path = Path({state_path:?})\nstate = json.loads(state_path.read_text())\nstate['dispatch_log_latest'] = '#1470 [Cycle Review] Cycle 345 end-of-cycle review (cycle 345)'\nstate['in_flight_sessions'] = 1\nstate['agent_sessions'] = [{{'issue': 1470, 'title': '[Cycle Review] Cycle 345 end-of-cycle review', 'status': 'in_flight'}}]\nstate_path.write_text(json.dumps(state, indent=2) + '\\n')\nPY\ngit -C {repo:?} add docs/state.json\ngit -C {repo:?} commit -m 'state(record-dispatch): #1470 dispatched [cycle 345]' >/dev/null\nprintf '%s\\n' 'Created review issue #1470 from orchestrator issue #123: https://github.com/EvaLok/schema-org-json-ld/issues/1470'\n",
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
        fs::write(
            &gh_path,
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1\" = \"issue\" ] && [ \"$2\" = \"close\" ]; then\n  exit 0\nfi\nprintf 'unexpected gh invocation\\n' >&2\nexit 1\n",
        )
        .unwrap();
        make_executable(&gh_path);

        with_path_prefix(&bin_dir, || run(&dir, 123, Some(345), false)).unwrap();

        let worklog = fs::read_to_string(dir.join("docs/worklog/2026-03-25/122700-cycle-345-summary.md"))
            .unwrap();
        assert!(worklog.contains("## Cycle state"));
        assert!(!worklog.contains("## Pre-dispatch state"));
        assert!(!worklog.contains("Snapshot before review dispatch"));
        assert!(worklog.contains("- **In-flight agent sessions**: 1"));
        assert!(worklog.contains("- **Pipeline status**: PASS (1 warning)"));
        assert!(!worklog.contains("phase_5_active"));
        assert!(worklog.contains("- **Publish gate**: published"));
        assert!(worklog.contains(
            "## Next steps\n\n1. Review and iterate on PR from [#1470](https://github.com/EvaLok/schema-org-json-ld/issues/1470) ([Cycle Review] Cycle 345 end-of-cycle review) when Copilot completes\n"
        ));

        let log_output = Command::new("git")
            .arg("-C")
            .arg(&dir)
            .args(["log", "--format=%s", "-3"])
            .output()
            .unwrap();
        let log = String::from_utf8_lossy(&log_output.stdout);
        assert!(log.contains("docs(worklog): refresh cycle 345 state after review dispatch [cycle 345]"));

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
        fs::write(
            &gh_path,
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1\" = \"issue\" ] && [ \"$2\" = \"close\" ]; then\n  exit 0\nfi\nprintf 'unexpected gh invocation\\n' >&2\nexit 1\n",
        )
        .unwrap();
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
        fs::write(
            &gh_path,
            "#!/usr/bin/env bash\nset -euo pipefail\nif [ \"$1\" = \"issue\" ] && [ \"$2\" = \"close\" ]; then\n  exit 0\nfi\nprintf 'unexpected gh invocation\\n' >&2\nexit 1\n",
        )
        .unwrap();
        make_executable(&gh_path);

        with_path_prefix(&bin_dir, || {
            step_c8(
                &dir,
                123,
                345,
                None,
                "PASS (0 warnings)",
            )
        })
        .unwrap();

        let args = fs::read_to_string(&args_path).unwrap();
        assert!(args.contains("Cycle 345 close-out complete."));
        assert!(args.contains("- Review: deferred (Copilot unavailable)"));
        assert!(args.contains("- Pipeline: PASS (0 warnings)"));

        let _ = fs::remove_dir_all(&dir);
    }
}
