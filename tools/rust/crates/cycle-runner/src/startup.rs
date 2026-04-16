use crate::git;
use crate::runner;
use crate::steps;
use serde::Serialize;
use serde_json::{json, Value};
use state_schema::{current_utc_timestamp, read_state_value, write_state_value, StepCommentGap};
use std::path::Path;

const HOUSEKEEPING_STEP_ID: &str = "7";
const CONCURRENCY_STEP_ID: &str = "8";
const STEP_COMMENTS_STEP_NAME: &str = "step-comments";
const AGENT_SESSIONS_LIFECYCLE_STEP_NAME: &str = "agent-sessions-lifecycle";
const CASCADE_PREFIX: &str = "Cascade from cycle ";
const AUTO_ACKNOWLEDGED_CASCADE_REASON: &str =
    "Auto-acknowledged inherited cascade from cycle {cycle} by cycle-runner startup";

#[derive(Debug, PartialEq, Eq)]
struct StepCommentCascade {
    cycle: u64,
    issue: u64,
    missing_steps: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct SituationReport {
    pub cycle: u64,
    pub issue: u64,
    pub resumed: bool,
    pub startup_brief: Value,
    pub pipeline: Value,
    pub housekeeping: Value,
    pub cycle_status: Value,
    pub warnings: Vec<String>,
}

pub fn run(
    repo_root: &Path,
    issue: u64,
    model_override: Option<&str>,
    dry_run: bool,
) -> Result<(), String> {
    if dry_run {
        eprintln!("[dry-run] Would run startup sequence for issue #{}", issue);
        eprintln!("[dry-run] 1. cycle-start --issue {} --json", issue);
        eprintln!("[dry-run] 2. post-step --step 0 (cycle initialization)");
        eprintln!("[dry-run] 3. pipeline-check --json");
        eprintln!("[dry-run] 4. post-step --step 4 (pipeline check)");
        eprintln!("[dry-run] 5. housekeeping-scan --json");
        eprintln!(
            "[dry-run] 6. post-step --step {} (housekeeping scan)",
            HOUSEKEEPING_STEP_ID
        );
        eprintln!("[dry-run] 7. cycle-status --json");
        eprintln!(
            "[dry-run] 8. post-step --step {} (concurrency check)",
            CONCURRENCY_STEP_ID
        );
        eprintln!("[dry-run] 9. Output combined situation report as JSON");
        return Ok(());
    }

    // The orchestrator's own model identity — distinct from
    // state_schema::default_agent_model, which returns the Copilot
    // coding-agent dispatch default (gpt-5.4). Using default_agent_model
    // here was the source of the cycle 502 review Finding 3 bug: every
    // cycle-run opening comment misreported the orchestrator as gpt-5.4.
    let model = match model_override {
        Some(m) => m.to_string(),
        None => state_schema::orchestrator_model(repo_root),
    };

    let mut warnings: Vec<String> = Vec::new();

    // --- Step 1: Run cycle-start ---
    eprintln!("Running cycle-start...");
    let issue_str = issue.to_string();
    let startup_brief = runner::run_tool_json(
        repo_root,
        "cycle-start",
        &["--issue", &issue_str, "--model", &model, "--json"],
    )?;

    let cycle = startup_brief
        .get("cycle")
        .and_then(Value::as_u64)
        .ok_or_else(|| "cycle-start output missing 'cycle' field".to_string())?;

    let is_resume = startup_brief
        .get("resume")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    let step_0_body = if is_resume {
        let phase = startup_brief
            .get("phase")
            .and_then(Value::as_str)
            .unwrap_or("unknown");
        format!("Resuming cycle {} (phase: {})", cycle, phase)
    } else {
        format_startup_brief(&startup_brief)
    };
    steps::post_step(
        repo_root,
        issue,
        "0",
        "Cycle initialization",
        &step_0_body,
        false,
    )?;

    // --- Step 2: Run pipeline-check ---
    eprintln!("Running pipeline-check...");
    let pipeline = match runner::run_tool_json(repo_root, "pipeline-check", &["--json"]) {
        Ok(value) => value,
        Err(error) => {
            warnings.push(format!("pipeline-check failed: {}", error));
            Value::Null
        }
    };
    if let Err(error) = auto_acknowledge_step_comment_cascades(repo_root, &pipeline, &mut warnings)
    {
        warnings.push(format!(
            "step-comments cascade auto-acknowledgement failed: {}",
            error
        ));
    }
    if let Err(error) = auto_mark_stale_agent_sessions(repo_root, &pipeline, cycle, &mut warnings) {
        warnings.push(format!(
            "stale agent-session marker update failed: {}",
            error
        ));
    }
    let pipeline_body = format_pipeline_summary(&pipeline);
    steps::post_step(
        repo_root,
        issue,
        "4",
        "Pipeline check",
        &pipeline_body,
        false,
    )?;

    // --- Step 3: Run housekeeping-scan ---
    eprintln!("Running housekeeping-scan...");
    let housekeeping = match runner::run_tool_json(repo_root, "housekeeping-scan", &["--json"]) {
        Ok(value) => value,
        Err(error) => {
            warnings.push(format!("housekeeping-scan failed: {}", error));
            Value::Null
        }
    };
    let housekeeping_body = format_housekeeping_summary(&housekeeping);
    steps::post_step(
        repo_root,
        issue,
        HOUSEKEEPING_STEP_ID,
        "Housekeeping scan",
        &housekeeping_body,
        false,
    )?;

    // --- Step 4: Run cycle-status ---
    eprintln!("Running cycle-status...");
    let cycle_status = match runner::run_tool_json(repo_root, "cycle-status", &["--json"]) {
        Ok(value) => value,
        Err(error) => {
            warnings.push(format!("cycle-status failed: {}", error));
            Value::Null
        }
    };
    let status_body = format_status_summary(&cycle_status);
    steps::post_step(
        repo_root,
        issue,
        CONCURRENCY_STEP_ID,
        "Concurrency check",
        &status_body,
        false,
    )?;

    // --- Output situation report ---
    let report = SituationReport {
        cycle,
        issue,
        resumed: is_resume,
        startup_brief,
        pipeline,
        housekeeping,
        cycle_status,
        warnings,
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&report)
            .map_err(|error| format!("failed to serialize situation report: {}", error))?
    );

    Ok(())
}

fn format_startup_brief(brief: &Value) -> String {
    let mut parts = Vec::new();

    if let Some(cycle) = brief.get("cycle").and_then(Value::as_u64) {
        parts.push(format!("Cycle {} initialized", cycle));
    }
    if let Some(receipt) = brief.get("receipt").and_then(Value::as_str) {
        parts.push(format!("Receipt: {}", receipt));
    }
    if let Some(directives) = brief.get("eva_directives").and_then(Value::as_array) {
        if !directives.is_empty() {
            parts.push(format!("Eva directives: {}", directives.len()));
        }
    }
    if let Some(inputs) = brief.get("input_from_eva").and_then(Value::as_array) {
        if !inputs.is_empty() {
            parts.push(format!("Input-from-eva issues: {}", inputs.len()));
        }
    }
    if let Some(sessions) = brief.pointer("/in_flight/sessions").and_then(Value::as_u64) {
        parts.push(format!("In-flight sessions: {}", sessions));
    }
    if let Some(warnings) = brief.get("warnings").and_then(Value::as_array) {
        for w in warnings {
            if let Some(s) = w.as_str() {
                parts.push(format!("Warning: {}", s));
            }
        }
    }

    if parts.is_empty() {
        "Cycle initialized".to_string()
    } else {
        parts.join("\n")
    }
}

fn format_pipeline_summary(pipeline: &Value) -> String {
    if pipeline.is_null() {
        return "Pipeline check failed (see warnings)".to_string();
    }

    let mut parts = Vec::new();
    if let Some(overall) = pipeline.get("overall").and_then(Value::as_str) {
        parts.push(format!("Overall: {}", overall));
    }
    if let Some(true) = pipeline
        .get("has_blocking_findings")
        .and_then(Value::as_bool)
    {
        parts.push("BLOCKING findings detected".to_string());
    }
    if let Some(steps) = pipeline.get("steps").and_then(Value::as_array) {
        for step in steps {
            let name = step.get("name").and_then(Value::as_str).unwrap_or("?");
            let status = step.get("status").and_then(Value::as_str).unwrap_or("?");
            parts.push(format!("  {}: {}", name, status));
        }
    }

    if parts.is_empty() {
        "Pipeline check complete".to_string()
    } else {
        parts.join("\n")
    }
}

fn format_housekeeping_summary(housekeeping: &Value) -> String {
    if housekeeping.is_null() {
        return "Housekeeping scan failed (see warnings)".to_string();
    }

    let count = housekeeping
        .get("items_needing_attention")
        .and_then(Value::as_u64)
        .unwrap_or(0);

    if count == 0 {
        "No housekeeping items found".to_string()
    } else {
        let mut parts = vec![format!("{} items need attention:", count)];
        for key in &[
            "stale_agent_issues",
            "orphan_draft_prs",
            "dead_branches",
            "stale_audit_inbound",
            "stale_qc_inbound",
        ] {
            if let Some(arr) = housekeeping.get(*key).and_then(Value::as_array) {
                if !arr.is_empty() {
                    parts.push(format!("  {}: {}", key, arr.len()));
                }
            }
        }
        parts.join("\n")
    }
}

fn format_status_summary(status: &Value) -> String {
    if status.is_null() {
        return "Cycle status check failed (see warnings)".to_string();
    }

    let mut parts = Vec::new();
    if let Some(total) = status
        .pointer("/concurrency/total_in_flight")
        .and_then(Value::as_u64)
    {
        parts.push(format!("In-flight: {}", total));
    }
    if let Some(actions) = status.get("action_items").and_then(Value::as_array) {
        if !actions.is_empty() {
            parts.push(format!("Action items: {}", actions.len()));
        }
    }
    if let Some(errors) = status.get("errors").and_then(Value::as_array) {
        if !errors.is_empty() {
            parts.push(format!("Errors: {}", errors.len()));
        }
    }

    if parts.is_empty() {
        "Cycle status check complete".to_string()
    } else {
        parts.join("\n")
    }
}

fn auto_acknowledge_step_comment_cascades(
    repo_root: &Path,
    pipeline: &Value,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    let Some(steps) = pipeline.get("steps").and_then(Value::as_array) else {
        return Ok(());
    };

    let mut cascades = Vec::new();
    for step in steps {
        if step.get("name").and_then(Value::as_str) != Some(STEP_COMMENTS_STEP_NAME) {
            continue;
        }
        if step.get("status").and_then(Value::as_str) != Some("Fail") {
            continue;
        }
        let Some(detail) = step.get("detail").and_then(Value::as_str) else {
            continue;
        };
        if let Some(cascade) = parse_step_comment_cascade(detail)? {
            cascades.push(cascade);
        }
    }

    if cascades.is_empty() {
        return Ok(());
    }

    let mut state = read_state_value(repo_root)?;
    let mut gaps: Vec<StepCommentGap> = state
        .get("step_comment_acknowledged_gaps")
        .cloned()
        .map(serde_json::from_value)
        .transpose()
        .map_err(|error| format!("failed to parse step_comment_acknowledged_gaps: {}", error))?
        .unwrap_or_default();
    let acknowledged_at = current_utc_timestamp();
    let mut changed = false;

    for cascade in cascades {
        if gaps
            .iter()
            .any(|existing| matches_acknowledged_gap(existing, &cascade))
        {
            continue;
        }

        gaps.push(StepCommentGap {
            cycle: cascade.cycle,
            issue: cascade.issue,
            missing_steps: cascade.missing_steps.clone(),
            acknowledged_at: acknowledged_at.clone(),
            reason: AUTO_ACKNOWLEDGED_CASCADE_REASON.replace("{cycle}", &cascade.cycle.to_string()),
        });
        warnings.push(format!(
            "Auto-acknowledged inherited step-comments cascade from cycle {} for issue #{}: {}",
            cascade.cycle,
            cascade.issue,
            cascade.missing_steps.join(", ")
        ));
        changed = true;
    }

    if changed {
        state["step_comment_acknowledged_gaps"] = serde_json::to_value(&gaps)
            .map_err(|error| format!("failed to serialize acknowledged gap records: {}", error))?;
        write_state_value(repo_root, &state)?;
    }

    Ok(())
}

/// After pipeline-check runs, scan the `agent-sessions-lifecycle` step for
/// drift reports (issues closed on GitHub but still `in_flight` in state) and
/// write `last_seen_stale_at_cycle = current_cycle` on each matching session
/// row. Without this writer the pipeline-check escalation from WARN (first
/// detection) to FAIL (stale across cycles) never fires — the detective-only
/// gap flagged by cycle-503 F3 review (state-integrity chronic category).
///
/// Idempotent: sessions whose marker is already at `current_cycle` (or later)
/// are skipped, so a same-cycle re-run produces no new commit.
fn auto_mark_stale_agent_sessions(
    repo_root: &Path,
    pipeline: &Value,
    current_cycle: u64,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    let mut state = read_state_value(repo_root)?;
    let marked = mark_stale_agent_sessions_in_state(&mut state, pipeline, current_cycle)?;
    if marked.is_empty() {
        return Ok(());
    }

    write_state_value(repo_root, &state)?;
    let summary = marked
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(", ");
    let message = format!(
        "state(stale-session-mark): marked agent session(s) #{} stale at cycle {} [cycle {}]",
        summary, current_cycle, current_cycle
    );
    git::add_and_commit(repo_root, &["docs/state.json"], &message)?;
    git::push(repo_root)?;
    warnings.push(format!(
        "Marked {} stale in_flight agent session(s) with last_seen_stale_at_cycle = {}: #{}",
        marked.len(),
        current_cycle,
        summary
    ));
    Ok(())
}

/// Pure state-mutation core of [`auto_mark_stale_agent_sessions`]. Returns the
/// list of issue numbers whose session row was updated so the caller can emit
/// a commit / log message. Extracted from the IO wrapper so the mutation logic
/// can be unit tested without a real git repo.
fn mark_stale_agent_sessions_in_state(
    state: &mut Value,
    pipeline: &Value,
    current_cycle: u64,
) -> Result<Vec<u64>, String> {
    let Some(steps) = pipeline.get("steps").and_then(Value::as_array) else {
        return Ok(Vec::new());
    };

    let mut stale_issues: Vec<u64> = Vec::new();
    for step in steps {
        if step.get("name").and_then(Value::as_str) != Some(AGENT_SESSIONS_LIFECYCLE_STEP_NAME) {
            continue;
        }
        let Some(detail) = step.get("detail").and_then(Value::as_str) else {
            continue;
        };
        for issue in parse_stale_session_issue_numbers(detail) {
            if !stale_issues.contains(&issue) {
                stale_issues.push(issue);
            }
        }
    }

    if stale_issues.is_empty() {
        return Ok(Vec::new());
    }

    let Some(sessions) = state
        .pointer_mut("/agent_sessions")
        .and_then(Value::as_array_mut)
    else {
        return Ok(Vec::new());
    };

    let mut marked = Vec::new();
    for session in sessions.iter_mut() {
        let Some(issue) = session.get("issue").and_then(Value::as_u64) else {
            continue;
        };
        if !stale_issues.contains(&issue) {
            continue;
        }
        if session.get("status").and_then(Value::as_str) != Some("in_flight") {
            continue;
        }
        let existing_cycle = session
            .get("last_seen_stale_at_cycle")
            .and_then(Value::as_u64);
        if existing_cycle.is_some_and(|seen| seen >= current_cycle) {
            continue;
        }
        session
            .as_object_mut()
            .ok_or_else(|| format!("agent session for issue #{issue} must be an object"))?
            .insert(
                "last_seen_stale_at_cycle".to_string(),
                json!(current_cycle),
            );
        marked.push(issue);
    }

    Ok(marked)
}

/// Extract GitHub issue numbers referenced in an agent-sessions-lifecycle WARN
/// or FAIL detail string. The authoritative format produced by
/// pipeline-check's `agent_sessions_lifecycle_assessment` is:
///   "agent session issue #N \"title\" is closed on GitHub but still marked in_flight"
/// Multiple sessions may be joined with "; ". We take every "#N" occurrence
/// and let the caller cross-reference against state.json, so changes to the
/// exact wording downstream do not silently break the writer.
fn parse_stale_session_issue_numbers(detail: &str) -> Vec<u64> {
    let mut issues = Vec::new();
    let mut remainder = detail;
    while let Some(index) = remainder.find('#') {
        let after = &remainder[index + 1..];
        let digits: String = after.chars().take_while(|ch| ch.is_ascii_digit()).collect();
        if !digits.is_empty() {
            if let Ok(issue) = digits.parse::<u64>() {
                if !issues.contains(&issue) {
                    issues.push(issue);
                }
            }
        }
        remainder = &after[digits.len()..];
    }
    issues
}

fn parse_step_comment_cascade(detail: &str) -> Result<Option<StepCommentCascade>, String> {
    if !detail.contains(CASCADE_PREFIX) || !detail.contains("(already penalized)") {
        return Ok(None);
    }

    let issue = parse_cascade_issue_number(detail)?;
    let cascade_index = detail
        .find(CASCADE_PREFIX)
        .ok_or_else(|| "missing cascade marker".to_string())?;
    let after_prefix = &detail[cascade_index + CASCADE_PREFIX.len()..];
    let cycle_text: String = after_prefix
        .chars()
        .take_while(|ch| ch.is_ascii_digit())
        .collect();
    if cycle_text.is_empty() {
        return Err(format!(
            "failed to parse cascade cycle number from step-comments detail: {}",
            detail
        ));
    }
    let cycle = cycle_text
        .parse::<u64>()
        .map_err(|error| format!("invalid cascade cycle '{}': {}", cycle_text, error))?;
    let remainder = &after_prefix[cycle_text.len()..];
    let (steps_text, end_marker) = if let Some(after_steps) = remainder.strip_prefix(": step ") {
        (after_steps, " was missing")
    } else if let Some(after_steps) = remainder.strip_prefix(": steps ") {
        (after_steps, " were missing")
    } else {
        return Err(format!(
            "failed to parse missing step ids from step-comments detail: {}",
            detail
        ));
    };
    let missing_end = steps_text.find(end_marker).ok_or_else(|| {
        format!(
            "failed to locate missing-step terminator in step-comments detail: {}",
            detail
        )
    })?;
    let missing_steps = steps_text[..missing_end]
        .split(',')
        .map(str::trim)
        .filter(|step| !step.is_empty())
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    if missing_steps.is_empty() {
        return Err(format!(
            "step-comments cascade detail listed no missing steps: {}",
            detail
        ));
    }

    Ok(Some(StepCommentCascade {
        cycle,
        issue,
        missing_steps,
    }))
}

fn parse_cascade_issue_number(detail: &str) -> Result<u64, String> {
    let prefix = detail
        .split(CASCADE_PREFIX)
        .next()
        .unwrap_or(detail)
        .trim_end();
    if let Some(hash_index) = prefix.rfind('#') {
        let digits = prefix[hash_index + 1..]
            .chars()
            .take_while(|ch| ch.is_ascii_digit())
            .collect::<String>();
        if !digits.is_empty() {
            return digits
                .parse::<u64>()
                .map_err(|error| format!("invalid issue number '{}': {}", digits, error));
        }
    }

    let issue_index = prefix.rfind("issue ").ok_or_else(|| {
        format!(
            "failed to locate issue number in step-comments detail: {}",
            detail
        )
    })?;
    let digits = prefix[issue_index + "issue ".len()..]
        .chars()
        .skip_while(|ch| !ch.is_ascii_digit())
        .take_while(|ch| ch.is_ascii_digit())
        .collect::<String>();
    if digits.is_empty() {
        return Err(format!(
            "failed to parse issue number from step-comments detail: {}",
            detail
        ));
    }
    digits
        .parse::<u64>()
        .map_err(|error| format!("invalid issue number '{}': {}", digits, error))
}

fn matches_acknowledged_gap(existing: &StepCommentGap, cascade: &StepCommentCascade) -> bool {
    existing.cycle == cascade.cycle
        && existing.issue == cascade.issue
        && normalized_step_ids(&existing.missing_steps)
            == normalized_step_ids(&cascade.missing_steps)
}

fn normalized_step_ids(step_ids: &[String]) -> Vec<String> {
    let mut normalized = step_ids.to_vec();
    normalized.sort();
    normalized.dedup();
    normalized
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::fs;
    use std::sync::atomic::{AtomicU64, Ordering};

    #[test]
    fn startup_step_ids_match_checklist_sections() {
        assert_eq!(HOUSEKEEPING_STEP_ID, "7");
        assert_eq!(CONCURRENCY_STEP_ID, "8");
    }

    #[test]
    fn format_startup_brief_with_full_data() {
        let brief = json!({
            "cycle": 301,
            "receipt": "abc1234",
            "eva_directives": ["#100"],
            "input_from_eva": [{"number": 200, "title": "test"}],
            "in_flight": {"sessions": 1},
            "warnings": ["stale dispatch detected"]
        });
        let result = format_startup_brief(&brief);
        assert!(result.contains("Cycle 301 initialized"));
        assert!(result.contains("Receipt: abc1234"));
        assert!(result.contains("Eva directives: 1"));
        assert!(result.contains("Input-from-eva issues: 1"));
        assert!(result.contains("In-flight sessions: 1"));
        assert!(result.contains("Warning: stale dispatch detected"));
    }

    #[test]
    fn format_pipeline_summary_with_null() {
        assert_eq!(
            format_pipeline_summary(&Value::Null),
            "Pipeline check failed (see warnings)"
        );
    }

    #[test]
    fn format_pipeline_summary_with_data() {
        let pipeline = json!({
            "overall": "Pass",
            "has_blocking_findings": false,
            "steps": [
                {"name": "metrics", "status": "Pass"},
                {"name": "invariants", "status": "Pass"}
            ]
        });
        let result = format_pipeline_summary(&pipeline);
        assert!(result.contains("Overall: Pass"));
        assert!(result.contains("metrics: Pass"));
    }

    #[test]
    fn format_housekeeping_summary_zero_items() {
        let report = json!({"items_needing_attention": 0});
        assert_eq!(
            format_housekeeping_summary(&report),
            "No housekeeping items found"
        );
    }

    #[test]
    fn format_status_summary_with_concurrency() {
        let status = json!({
            "concurrency": {"total_in_flight": 2},
            "action_items": ["review PR"],
            "errors": []
        });
        let result = format_status_summary(&status);
        assert!(result.contains("In-flight: 2"));
        assert!(result.contains("Action items: 1"));
    }

    fn temp_repo_root(prefix: &str) -> std::path::PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "cycle-runner-startup-{}-{}-{}",
            prefix,
            std::process::id(),
            run_id
        ));
        fs::create_dir_all(root.join("docs"))
            .expect("failed to create docs directory for startup test");
        root
    }

    fn write_state(root: &std::path::Path, state: serde_json::Value) {
        fs::write(root.join("docs/state.json"), format!("{}\n", state))
            .expect("failed to write startup test state to docs/state.json");
    }

    #[test]
    fn parse_step_comment_cascade_parses_multiple_missing_steps() {
        let cascade = parse_step_comment_cascade(
            "issue EvaLok/schema-org-json-ld#1680: Cascade from cycle 345: steps 0, 1.1, 4, 7, 8, C4.1 were missing (already penalized); found 21 comments",
        )
        .expect("parse should succeed")
        .expect("cascade should be detected");

        assert_eq!(cascade.cycle, 345);
        assert_eq!(cascade.issue, 1680);
        assert_eq!(
            cascade.missing_steps,
            vec!["0", "1.1", "4", "7", "8", "C4.1"]
        );
    }

    #[test]
    fn auto_acknowledge_step_comment_cascade_updates_state_and_warns() {
        let root = temp_repo_root("acknowledge");
        write_state(
            &root,
            json!({
                "last_cycle": {"number": 346}
            }),
        );
        let pipeline = json!({
            "steps": [
                {
                    "name": "step-comments",
                    "status": "Fail",
                    "detail": "issue EvaLok/schema-org-json-ld#1680: Cascade from cycle 345: steps 0, 1.1, 4, 7, 8, C4.1 were missing (already penalized); found 21 comments"
                }
            ]
        });
        let mut warnings = Vec::new();

        auto_acknowledge_step_comment_cascades(&root, &pipeline, &mut warnings)
            .expect("auto-acknowledgement should succeed");

        let state = state_schema::read_state_value(&root).expect("state should be readable");
        let gaps = state
            .get("step_comment_acknowledged_gaps")
            .and_then(serde_json::Value::as_array)
            .expect("gap array should exist");
        assert_eq!(gaps.len(), 1);
        assert_eq!(
            gaps[0].get("cycle").and_then(serde_json::Value::as_u64),
            Some(345)
        );
        assert_eq!(
            gaps[0].get("issue").and_then(serde_json::Value::as_u64),
            Some(1680)
        );
        assert_eq!(
            gaps[0].get("missing_steps").cloned(),
            Some(json!(["0", "1.1", "4", "7", "8", "C4.1"]))
        );
        assert_eq!(
            gaps[0].get("reason").and_then(serde_json::Value::as_str),
            Some("Auto-acknowledged inherited cascade from cycle 345 by cycle-runner startup")
        );
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0]
            .contains("Auto-acknowledged inherited step-comments cascade from cycle 345"));
        assert!(warnings[0].contains("issue #1680"));
    }

    #[test]
    fn auto_acknowledge_step_comment_cascade_leaves_state_unchanged_without_cascade() {
        let root = temp_repo_root("no-cascade");
        write_state(
            &root,
            json!({
                "last_cycle": {"number": 346}
            }),
        );
        let pipeline = json!({
            "steps": [
                {
                    "name": "step-comments",
                    "status": "Fail",
                    "detail": "issue EvaLok/schema-org-json-ld#1680: missing mandatory [0]"
                }
            ]
        });
        let mut warnings = Vec::new();

        auto_acknowledge_step_comment_cascades(&root, &pipeline, &mut warnings)
            .expect("non-cascade failures should be ignored");

        let state = state_schema::read_state_value(&root).expect("state should be readable");
        assert!(state.get("step_comment_acknowledged_gaps").is_none());
        assert!(warnings.is_empty());
    }

    #[test]
    fn auto_acknowledge_step_comment_cascade_is_idempotent_for_existing_gap() {
        let root = temp_repo_root("idempotent");
        write_state(
            &root,
            json!({
                "last_cycle": {"number": 346},
                "step_comment_acknowledged_gaps": [
                    {
                        "cycle": 345,
                        "issue": 1680,
                        "missing_steps": ["0", "1.1", "4", "7", "8", "C4.1"],
                        "acknowledged_at": "2026-03-24T10:30:00Z",
                        "reason": "Auto-acknowledged inherited cascade from cycle 345 by cycle-runner startup"
                    }
                ]
            }),
        );
        let pipeline = json!({
            "steps": [
                {
                    "name": "step-comments",
                    "status": "Fail",
                    "detail": "issue EvaLok/schema-org-json-ld#1680: Cascade from cycle 345: steps 0, 1.1, 4, 7, 8, C4.1 were missing (already penalized)"
                }
            ]
        });
        let mut warnings = Vec::new();

        auto_acknowledge_step_comment_cascades(&root, &pipeline, &mut warnings)
            .expect("existing gaps should be ignored");

        let state = state_schema::read_state_value(&root).expect("state should be readable");
        let gaps = state
            .get("step_comment_acknowledged_gaps")
            .and_then(serde_json::Value::as_array)
            .expect("gap array should exist");
        assert_eq!(gaps.len(), 1);
        assert!(warnings.is_empty());
    }

    #[test]
    fn parse_stale_session_issue_numbers_extracts_single_issue() {
        let detail = "agent session issue #2317 \"Structural fix dispatch\" is closed on GitHub but still marked in_flight";
        assert_eq!(parse_stale_session_issue_numbers(detail), vec![2317]);
    }

    #[test]
    fn parse_stale_session_issue_numbers_extracts_multiple_issues_semicolon_joined() {
        let detail = "agent session issue #2317 \"one\" is closed on GitHub but still marked in_flight; agent session issue #2549 \"two\" is closed on GitHub but still marked in_flight";
        assert_eq!(
            parse_stale_session_issue_numbers(detail),
            vec![2317, 2549]
        );
    }

    #[test]
    fn parse_stale_session_issue_numbers_deduplicates() {
        let detail = "issue #2317 ... issue #2317 (already stale in cycle 462)";
        assert_eq!(parse_stale_session_issue_numbers(detail), vec![2317]);
    }

    #[test]
    fn parse_stale_session_issue_numbers_ignores_non_numeric_hashes() {
        let detail = "# header ## subheading nothing here";
        assert!(parse_stale_session_issue_numbers(detail).is_empty());
    }

    #[test]
    fn mark_stale_agent_sessions_sets_marker_on_first_detection() {
        let mut state = json!({
            "agent_sessions": [
                {
                    "issue": 2549,
                    "title": "stale session",
                    "status": "in_flight"
                }
            ]
        });
        let pipeline = json!({
            "steps": [
                {
                    "name": "agent-sessions-lifecycle",
                    "status": "Warn",
                    "detail": "agent session issue #2549 \"stale session\" is closed on GitHub but still marked in_flight"
                }
            ]
        });

        let marked = mark_stale_agent_sessions_in_state(&mut state, &pipeline, 505)
            .expect("mark should succeed");

        assert_eq!(marked, vec![2549]);
        let session = state
            .pointer("/agent_sessions/0")
            .expect("session row should exist");
        assert_eq!(
            session
                .get("last_seen_stale_at_cycle")
                .and_then(Value::as_u64),
            Some(505)
        );
    }

    #[test]
    fn mark_stale_agent_sessions_is_idempotent_within_same_cycle() {
        let mut state = json!({
            "agent_sessions": [
                {
                    "issue": 2549,
                    "status": "in_flight",
                    "last_seen_stale_at_cycle": 505
                }
            ]
        });
        let pipeline = json!({
            "steps": [
                {
                    "name": "agent-sessions-lifecycle",
                    "status": "Warn",
                    "detail": "agent session issue #2549 \"x\" is closed on GitHub but still marked in_flight"
                }
            ]
        });

        let marked = mark_stale_agent_sessions_in_state(&mut state, &pipeline, 505)
            .expect("mark should succeed");

        assert!(marked.is_empty());
    }

    #[test]
    fn mark_stale_agent_sessions_refreshes_older_marker() {
        let mut state = json!({
            "agent_sessions": [
                {
                    "issue": 2549,
                    "status": "in_flight",
                    "last_seen_stale_at_cycle": 503
                }
            ]
        });
        let pipeline = json!({
            "steps": [
                {
                    "name": "agent-sessions-lifecycle",
                    "status": "Fail",
                    "detail": "agent session issue #2549 \"x\" is closed on GitHub but still marked in_flight (already stale in cycle 503)"
                }
            ]
        });

        let marked = mark_stale_agent_sessions_in_state(&mut state, &pipeline, 505)
            .expect("mark should succeed");

        // Existing older marker means the FAIL path has already been taken;
        // we still refresh the cycle so subsequent cycles see it as current.
        assert_eq!(marked, vec![2549]);
        assert_eq!(
            state
                .pointer("/agent_sessions/0/last_seen_stale_at_cycle")
                .and_then(Value::as_u64),
            Some(505)
        );
    }

    #[test]
    fn mark_stale_agent_sessions_skips_closed_session_rows() {
        let mut state = json!({
            "agent_sessions": [
                {
                    "issue": 2549,
                    "status": "closed"
                }
            ]
        });
        let pipeline = json!({
            "steps": [
                {
                    "name": "agent-sessions-lifecycle",
                    "status": "Warn",
                    "detail": "agent session issue #2549 \"x\" is closed on GitHub but still marked in_flight"
                }
            ]
        });

        let marked = mark_stale_agent_sessions_in_state(&mut state, &pipeline, 505)
            .expect("mark should succeed");

        // If the session row is already closed, the check is stale and we
        // should not re-mark it. Downstream state has moved on.
        assert!(marked.is_empty());
    }

    #[test]
    fn mark_stale_agent_sessions_noop_without_pipeline_step() {
        let mut state = json!({
            "agent_sessions": [
                {"issue": 2549, "status": "in_flight"}
            ]
        });
        let pipeline = json!({
            "steps": [
                {"name": "other-step", "status": "Pass", "detail": "anything"}
            ]
        });

        let marked = mark_stale_agent_sessions_in_state(&mut state, &pipeline, 505)
            .expect("mark should succeed");

        assert!(marked.is_empty());
        assert!(state
            .pointer("/agent_sessions/0")
            .and_then(Value::as_object)
            .and_then(|obj| obj.get("last_seen_stale_at_cycle"))
            .is_none());
    }
}
