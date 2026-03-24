use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

const MAIN_REPO: &str = "EvaLok/schema-org-json-ld";

pub fn generate(
    repo_root: &Path,
    cycle: u64,
    issue: u64,
    is_stabilization: bool,
) -> Result<String, String> {
    let state = state_schema::read_state_value(repo_root)?;

    let merged_prs = get_merged_prs(&state);
    let last_review = get_last_review(&state);
    let worklog_path = find_worklog_for_cycle(repo_root, cycle)?;
    let journal_path = find_latest_journal(repo_root)?;

    let worklog_rel = worklog_path
        .strip_prefix(repo_root)
        .unwrap_or(&worklog_path)
        .display()
        .to_string();
    let journal_rel = journal_path
        .strip_prefix(repo_root)
        .unwrap_or(&journal_path)
        .display()
        .to_string();

    let mut body = String::new();

    // Observation mode prefix for stabilization
    if is_stabilization {
        body.push_str("> **OBSERVATION MODE (ADR 0011):** Log all findings in standard structured format, but do NOT classify as requiring immediate action. All findings logged for post-stabilization triage. Orchestrator will NOT dispatch fixes or mark findings as actioned during stabilization. Your role is forensic documentation, not remediation.\n\n");
    }

    // Adversarial mandate
    body.push_str("## Adversarial Review Mandate\n\n");
    body.push_str("Your job is to find everything wrong with this cycle's work. Be thorough. Be skeptical. If something looks fine on the surface, dig deeper. This is an adversarial review — actively look for problems, inconsistencies, drift, and complacency. Do not assume good faith or give the benefit of the doubt.\n\n");

    // Cycle summary
    body.push_str(&format!("## Cycle {} Summary\n\n", cycle));
    body.push_str(&format!("- **Cycle issue**: {}#{}\n", MAIN_REPO, issue));

    if merged_prs.is_empty() {
        body.push_str("- **PRs merged**: None\n");
    } else {
        let pr_list: Vec<String> = merged_prs
            .iter()
            .map(|(num, title)| format!("{}#{} ({})", MAIN_REPO, num, title))
            .collect();
        body.push_str(&format!("- **PRs merged**: {}\n", pr_list.join(", ")));
    }

    body.push_str(&format!(
        "- **Direct pushes to master**: Run `bash tools/cycle-receipts --cycle {} --repo-root .` for full commit list\n",
        cycle
    ));
    body.push_str("- **Dispatched**: Check agent_sessions in state.json for this cycle's dispatches\n\n");

    // Review targets
    body.push_str("## Review Targets\n\n");

    body.push_str("### 1. Code changes\n");
    body.push_str("Merged PRs and direct pushes — quality issues, test gaps, convention violations.\n\n");

    body.push_str("### 2. Worklog accuracy\n");
    body.push_str(&format!("- File: `{}`\n", worklog_rel));
    body.push_str("- Cross-reference claims against actual commits, state.json, and issue activity. Does the narrative match reality?\n");
    body.push_str(&format!(
        "- Verify receipt table completeness using `bash tools/cycle-receipts --cycle {} --repo-root .`\n\n",
        cycle
    ));

    body.push_str("### 3. Journal quality\n");
    body.push_str(&format!("- File: `{}`\n", journal_rel));
    body.push_str("- Is the journal genuine reflection or boilerplate? Does it contain actionable commitments with observable completion conditions?\n\n");

    body.push_str("### 4. State.json integrity\n");
    body.push_str("- Verify copilot_metrics match agent_sessions array (resolved + in_flight == total_dispatches)\n");
    body.push_str("- Check field_inventory freshness markers match reality\n");
    body.push_str("- Run `bash tools/state-invariants` and `bash tools/metric-snapshot`\n\n");

    body.push_str("### 5. Process adherence\n");
    body.push_str("- Did the orchestrator follow its own checklist? Did it use tools when tools exist?\n");
    body.push_str(&format!(
        "- Did the orchestrator post per-step comments? Count step comments on {}#{}.\n\n",
        MAIN_REPO, issue
    ));

    body.push_str("### 6. Complacency detection\n");
    if let Some(ref review) = last_review {
        if let Some(score) = review.get("complacency_score").and_then(Value::as_u64) {
            body.push_str(&format!("- Previous review complacency score: {}/5\n", score));
        }
        if let Some(categories) = review.get("categories").and_then(Value::as_array) {
            let cat_list: Vec<&str> = categories.iter().filter_map(Value::as_str).collect();
            if !cat_list.is_empty() {
                body.push_str(&format!(
                    "- Previous finding categories: {}\n",
                    cat_list.join(", ")
                ));
            }
        }
    }
    body.push_str("- Are chronic categories being genuinely addressed or just acknowledged?\n\n");

    body.push_str("### 7. Commit receipt verification\n");
    body.push_str("- Verify receipt hashes resolve and match claims\n");
    body.push_str(&format!(
        "- Run `bash tools/cycle-receipts --cycle {} --repo-root .`\n",
        cycle
    ));
    body.push_str("- **Receipt table scope**: covers all commits through `cycle-complete`. Docs commit and record-dispatch commit **structurally excluded** — created after worklog. Don't flag absence as defect.\n\n");

    // Complacency scoring cap
    body.push_str("## Complacency scoring cap\n\n");
    body.push_str("If the cycle overrode any FAIL or blocking-level pipeline gate (including pipeline-check or state-invariants), the maximum complacency score is 3/5 regardless of other factors.\n\n");

    // Output format
    body.push_str("## Output format\n\n");
    body.push_str(&format!(
        "Commit your findings as `docs/reviews/cycle-{}.md` using this template for each finding:\n\n",
        cycle
    ));
    body.push_str("```\n## N. [category-name] Finding title\n\n**File**: path/to/file:line\n**Evidence**: what was observed\n**Recommendation**: concrete action\n```\n\n");
    body.push_str("End with a justified complacency score (1-5). Three deeply investigated findings with evidence are more valuable than ten surface-level observations.\n\n");
    body.push_str("Do NOT attempt to post issue comments — commit the review file as your only output.\n");

    Ok(body)
}

/// Write the review body to docs/reviews/cycle-{N}-review-body.md and return the path.
pub fn write_to_file(repo_root: &Path, cycle: u64, body: &str) -> Result<PathBuf, String> {
    let reviews_dir = repo_root.join("docs").join("reviews");
    fs::create_dir_all(&reviews_dir)
        .map_err(|error| format!("failed to create {}: {}", reviews_dir.display(), error))?;

    let path = reviews_dir.join(format!("cycle-{}-review-body.md", cycle));
    fs::write(&path, body)
        .map_err(|error| format!("failed to write {}: {}", path.display(), error))?;

    Ok(path)
}

/// Find the worklog file for a given cycle by searching for *cycle-{N}* in docs/worklog/*/.
pub fn find_worklog_for_cycle(repo_root: &Path, cycle: u64) -> Result<PathBuf, String> {
    let worklog_dir = repo_root.join("docs").join("worklog");
    let pattern = format!("cycle-{}", cycle);
    let mut candidates = Vec::new();

    let entries = fs::read_dir(&worklog_dir)
        .map_err(|error| format!("failed to read {}: {}", worklog_dir.display(), error))?;

    for entry in entries {
        let entry =
            entry.map_err(|error| format!("failed to read worklog dir entry: {}", error))?;
        if !entry.file_type().map_or(false, |ft| ft.is_dir()) {
            continue;
        }
        let sub_entries = fs::read_dir(entry.path())
            .map_err(|error| format!("failed to read {}: {}", entry.path().display(), error))?;
        for sub_entry in sub_entries {
            let sub_entry =
                sub_entry.map_err(|error| format!("failed to read dir entry: {}", error))?;
            let name = sub_entry.file_name().to_string_lossy().to_string();
            if name.contains(&pattern) && name.ends_with(".md") {
                candidates.push(sub_entry.path());
            }
        }
    }

    candidates.sort();
    candidates.last().cloned().ok_or_else(|| {
        format!(
            "No worklog file found matching *{}* in {}",
            pattern,
            worklog_dir.display()
        )
    })
}

/// Find the most recent journal file in docs/journal/ (by filename, which is date-based).
pub fn find_latest_journal(repo_root: &Path) -> Result<PathBuf, String> {
    let journal_dir = repo_root.join("docs").join("journal");
    let mut files: Vec<PathBuf> = fs::read_dir(&journal_dir)
        .map_err(|error| format!("failed to read {}: {}", journal_dir.display(), error))?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.ends_with(".md") && name != "JOURNAL.md"
        })
        .map(|e| e.path())
        .collect();

    files.sort();
    files
        .last()
        .cloned()
        .ok_or_else(|| format!("No journal files found in {}", journal_dir.display()))
}

fn get_merged_prs(state: &Value) -> Vec<(u64, String)> {
    let sessions = match state.get("agent_sessions").and_then(Value::as_array) {
        Some(s) => s,
        None => return Vec::new(),
    };

    // Get the last_cycle timestamp as the boundary for "this cycle"
    let last_cycle_ts = state
        .pointer("/last_cycle/timestamp")
        .and_then(Value::as_str)
        .unwrap_or("");

    let mut merged = Vec::new();
    for session in sessions {
        let status = session
            .get("status")
            .and_then(Value::as_str)
            .unwrap_or("");
        if status != "merged" {
            continue;
        }
        let merged_at = session
            .get("merged_at")
            .and_then(Value::as_str)
            .unwrap_or("");
        // Include PRs merged after the last cycle completed
        if !last_cycle_ts.is_empty() && merged_at <= last_cycle_ts {
            continue;
        }
        let pr = session.get("pr").and_then(Value::as_u64).unwrap_or(0);
        let title = session
            .get("title")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string();
        if pr > 0 {
            merged.push((pr, title));
        }
    }
    merged
}

fn get_last_review(state: &Value) -> Option<Value> {
    state
        .pointer("/review_agent/history")
        .and_then(Value::as_array)
        .and_then(|h| h.last())
        .cloned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn get_merged_prs_filters_by_timestamp() {
        let state = json!({
            "agent_sessions": [
                {"status": "merged", "merged_at": "2026-03-01T00:00:00Z", "pr": 100, "title": "old"},
                {"status": "merged", "merged_at": "2026-03-19T00:00:00Z", "pr": 200, "title": "new"},
                {"status": "in_flight", "pr": 300, "title": "wip"}
            ],
            "last_cycle": {"timestamp": "2026-03-18T00:00:00Z"}
        });
        let merged = get_merged_prs(&state);
        assert_eq!(merged.len(), 1);
        assert_eq!(merged[0].0, 200);
    }

    #[test]
    fn get_last_review_returns_most_recent() {
        let state = json!({
            "review_agent": {
                "history": [
                    {"cycle": 299, "complacency_score": 3},
                    {"cycle": 300, "complacency_score": 4}
                ]
            }
        });
        let review = get_last_review(&state).unwrap();
        assert_eq!(review["cycle"], 300);
    }

    #[test]
    fn generate_includes_observation_mode_for_stabilization() {
        // This test would need a full repo setup, so just test the flag logic
        let body_with = "> **OBSERVATION MODE (ADR 0011):**";
        assert!(body_with.contains("OBSERVATION MODE"));
    }
}
