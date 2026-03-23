use crate::runner;
use crate::steps;
use serde::Serialize;
use serde_json::Value;
use std::path::Path;

const HOUSEKEEPING_STEP_ID: &str = "7";
const CONCURRENCY_STEP_ID: &str = "8";

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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

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
}
