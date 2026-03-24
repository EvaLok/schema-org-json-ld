use crate::runner;
use crate::steps;
use serde::Serialize;
use serde_json::Value;
use state_schema::{current_utc_timestamp, read_state_value, write_state_value, StepCommentGap};
use std::path::Path;

const HOUSEKEEPING_STEP_ID: &str = "7";
const CONCURRENCY_STEP_ID: &str = "8";
const STEP_COMMENTS_STEP_NAME: &str = "step-comments";
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

    let model = match model_override {
        Some(m) => m.to_string(),
        None => state_schema::default_agent_model(repo_root)?,
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
        fs::create_dir_all(root.join("docs")).unwrap();
        root
    }

    fn write_state(root: &std::path::Path, state: serde_json::Value) {
        fs::write(root.join("docs/state.json"), format!("{}\n", state)).unwrap();
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
}
