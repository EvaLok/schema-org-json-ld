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
    write_state_value,
};

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";

struct ReviewInfo {
    issue_number: u64,
    issue_url: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct PipelineGateReport {
    overall: String,
    has_blocking_findings: bool,
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

    // C5: Commit and push docs
    step_c5(repo_root, issue, cycle, &worklog)?;

    // C5.1: Receipt validation (report only)
    step_c5_1(repo_root, issue, cycle, &worklog)?;

    // C5.5: Pipeline check — GATE
    let pipeline_passed = step_c5_5(repo_root, issue)?;

    // C5.6: Stabilization counter
    step_c5_6(repo_root, issue, cycle, pipeline_passed)?;

    // C6: Review dispatch
    let review_info = step_c6(repo_root, issue, cycle)?;

    // C7: Push
    step_c7(repo_root, issue)?;

    // C8: Close issue
    step_c8(repo_root, issue, cycle, &review_info)?;
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

fn step_c5_5(repo_root: &Path, issue: u64) -> Result<bool, String> {
    eprintln!("C5.5: Final pipeline gate...");

    let output = runner::run_tool(repo_root, "pipeline-check", &["--json"])?;
    let exit_ok = output.status.success();
    let stdout = runner::stdout_text(&output);
    let stderr = runner::stderr_text(&output);
    let exit_code = output.status.code().unwrap_or(-1);

    let (passed, body) = match parse_pipeline_gate_report(&stdout) {
        Ok(report) => {
            let passed =
                exit_ok && report.overall.eq_ignore_ascii_case("pass") && !report.has_blocking_findings;
            let mut body = format!(
                "Pipeline: {}\n- exit_code: {}\n- overall: {}\n- has_blocking_findings: {}",
                if passed { "PASS" } else { "FAIL" },
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
            (passed, body)
        }
        Err(parse_error) => {
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
            (false, body)
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

    Ok(true)
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

    Ok(PipelineGateReport {
        overall: overall.to_string(),
        has_blocking_findings,
    })
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

fn step_c6(repo_root: &Path, issue: u64, cycle: u64) -> Result<ReviewInfo, String> {
    eprintln!("C6: Dispatching review agent...");

    // Check if review already dispatched for this cycle (idempotency)
    let state = state_schema::read_state_value(repo_root)?;
    if let Some(existing) = find_existing_review_dispatch(&state, cycle) {
        let body = format!(
            "Review already dispatched as #{} (skipping duplicate)",
            existing.issue_number
        );
        steps::post_step(repo_root, issue, "C6", "Review dispatch", &body, false)?;
        return Ok(existing);
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

    Ok(review_info)
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

fn step_c8(
    repo_root: &Path,
    issue: u64,
    cycle: u64,
    review_info: &ReviewInfo,
) -> Result<(), String> {
    eprintln!("C8: Closing orchestrator issue...");

    let closing_body = format!(
        "Cycle {} close-out complete.\n\n\
         - Review: dispatched as #{} ({})\n\
         - Pipeline: PASS\n\
         - All close-out steps completed by cycle-runner",
        cycle, review_info.issue_number, review_info.issue_url,
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
        "[dry-run] C5:   git add docs/ && git commit && git push (worklog frozen at this point)"
            .to_string(),
        "[dry-run] C5.1: receipt-validate (report only)".to_string(),
        "[dry-run] C5.5: pipeline-check (GATE)".to_string(),
        "[dry-run] C5.6: stabilization counter update (if applicable)".to_string(),
        "[dry-run] C6:   generate review body + dispatch-review".to_string(),
        "[dry-run] C7:   git push".to_string(),
        format!("[dry-run] C8:   close issue #{}", issue),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use std::process::Command;
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
            .arg("init")
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
        fs::write(
            dir.join("tools/post-step"),
            format!(
                "#!/usr/bin/env bash\nset -euo pipefail\n{{\nfor arg in \"$@\"; do\nprintf -- '---ARG---\\n%s\\n' \"$arg\"\ndone\n}} > \"{}\"\n",
                output_path.display()
            ),
        )
        .unwrap();
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
    fn close_out_dry_run_sequence_goes_directly_from_c6_to_c7() {
        let lines = close_out_dry_run_lines(345, 123);
        assert!(lines.iter().all(|line| !line.contains("C6.5")));

        let c6 = lines
            .iter()
            .position(|line| line.contains("[dry-run] C6:"))
            .expect("C6 dry-run line should exist");
        let c7 = lines
            .iter()
            .position(|line| line.contains("[dry-run] C7:"))
            .expect("C7 dry-run line should exist");

        assert_eq!(c7, c6 + 1);
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
}
