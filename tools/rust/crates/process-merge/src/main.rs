use chrono::DateTime;
use clap::Parser;
use serde_json::{json, Value};
use state_schema::{
    commit_state_json, current_cycle_from_state, current_utc_timestamp, read_state_value,
    set_value_at_pointer, write_state_value,
};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
enum IssueValue {
    None,
    Number(u64),
}

#[derive(Parser, Debug)]
#[command(name = "process-merge")]
struct Cli {
    /// Comma-separated list of merged PR numbers (e.g. "595,597,599")
    #[arg(long, value_delimiter = ',', num_args = 1..)]
    prs: Vec<u64>,

    /// Required comma-separated list of issue numbers matching --prs by position; use "none" when intentionally providing no issue links
    #[arg(long, required = true, value_delimiter = ',', num_args = 1.., value_parser = parse_issue_value)]
    issues: Vec<IssueValue>,

    /// Optional RFC 3339 merge timestamp to record instead of the current time
    #[arg(long, value_parser = parse_merged_at)]
    merged_at: Option<String>,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MergeUpdate {
    in_flight: i64,
    cycle: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PatchUpdate {
    path: &'static str,
    value: Value,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    if cli.prs.is_empty() {
        return Err("at least one PR number is required via --prs".to_string());
    }
    let issues = normalize_issues(&cli.issues, cli.prs.len())?;

    let mut state = read_state_value(&cli.repo_root)?;
    let merged_at = resolve_merged_at(cli.merged_at.as_deref(), current_utc_timestamp);
    let current_cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /cycle_phase/cycle or /last_cycle/number in state.json" {
            "missing numeric /cycle_phase/cycle or /last_cycle/number in docs/state.json"
                .to_string()
        } else {
            error
        }
    })?;
    let update = compute_update(&state, current_cycle, &cli.prs)?;
    let patch = build_patch(&update)?;
    apply_patch(&mut state, &patch)?;
    update_agent_sessions(&mut state, &cli.prs, &issues, &merged_at)?;
    sync_last_cycle_summary(&mut state, current_cycle)?;
    write_state_value(&cli.repo_root, &state)?;

    let commit_message = format!(
        "state(process-merge): {} merged [cycle {}]",
        format_pr_list(&cli.prs),
        update.cycle
    );
    let receipt = commit_state_json(&cli.repo_root, &commit_message)?;
    println!(
        "Merge processed: {}. In-flight sessions now {} (receipt: {})",
        format_pr_list(&cli.prs),
        update.in_flight,
        receipt
    );

    Ok(())
}

fn parse_issue_value(raw: &str) -> Result<IssueValue, String> {
    if raw == "none" {
        return Ok(IssueValue::None);
    }

    raw.parse::<u64>()
        .map(IssueValue::Number)
        .map_err(|_| "issues must be numeric or the literal 'none'".to_string())
}

fn parse_merged_at(raw: &str) -> Result<String, String> {
    DateTime::parse_from_rfc3339(raw)
        .map(|_| raw.to_string())
        .map_err(|_| "--merged-at must be a valid RFC 3339 timestamp".to_string())
}

fn resolve_merged_at<F>(merged_at: Option<&str>, fallback: F) -> String
where
    F: FnOnce() -> String,
{
    merged_at.map(ToOwned::to_owned).unwrap_or_else(fallback)
}

fn normalize_issues(issue_values: &[IssueValue], pr_count: usize) -> Result<Vec<u64>, String> {
    if issue_values.len() == 1 && issue_values[0] == IssueValue::None {
        return Ok(vec![]);
    }

    if issue_values.contains(&IssueValue::None) {
        return Err("--issues none cannot be combined with numeric issue values".to_string());
    }

    if issue_values.len() != pr_count {
        return Err("--issues must have the same number of values as --prs".to_string());
    }

    issue_values
        .iter()
        .map(|issue| match issue {
            IssueValue::None => {
                Err("internal error: none issue value should have been filtered".to_string())
            }
            IssueValue::Number(value) => Ok(*value),
        })
        .collect()
}

fn get_top_level_i64(state: &Value, field: &str) -> Result<i64, String> {
    state
        .pointer(&format!("/{}", field))
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("missing numeric /{} in docs/state.json", field))
}

fn compute_update(state: &Value, cycle: u64, prs: &[u64]) -> Result<MergeUpdate, String> {
    let in_flight = get_top_level_i64(state, "in_flight_sessions")?;

    if in_flight < 0 {
        return Err(format!(
            "in_flight_sessions({}) must be non-negative",
            in_flight
        ));
    }

    let merge_count = i64::try_from(prs.len()).map_err(|_| "PR count is too large".to_string())?;
    let decremented = merge_count.min(in_flight);
    let next_in_flight = in_flight - decremented;
    if decremented < merge_count {
        eprintln!(
            "Warning: in_flight underflow prevented (requested {}, decremented {})",
            merge_count, decremented
        );
    }

    Ok(MergeUpdate {
        in_flight: next_in_flight,
        cycle,
    })
}

fn build_patch(update: &MergeUpdate) -> Result<Vec<PatchUpdate>, String> {
    let cycle =
        u32::try_from(update.cycle).map_err(|_| "cycle must fit in u32 range".to_string())?;
    let marker = format!("cycle {}", cycle);

    Ok(vec![
        PatchUpdate {
            path: "/in_flight_sessions",
            value: json!(update.in_flight),
        },
        PatchUpdate {
            path: "/field_inventory/fields/in_flight_sessions/last_refreshed",
            value: json!(marker),
        },
    ])
}

fn apply_patch(state: &mut Value, updates: &[PatchUpdate]) -> Result<(), String> {
    for update in updates {
        set_value_at_pointer(state, update.path, update.value.clone())?;
    }

    Ok(())
}

fn update_agent_sessions(
    state: &mut Value,
    prs: &[u64],
    issues: &[u64],
    merged_at: &str,
) -> Result<(), String> {
    let Some(sessions) = state
        .pointer_mut("/agent_sessions")
        .and_then(Value::as_array_mut)
    else {
        return Err("missing array /agent_sessions in docs/state.json".to_string());
    };

    for (index, pr) in prs.iter().enumerate() {
        let issue = issues.get(index).copied();
        let matching_indexes = find_matching_agent_session_indexes(sessions, *pr, issue);
        let matched_index = if !matching_indexes.is_empty() {
            collapse_agent_session_duplicates(sessions, &matching_indexes)?
        } else if let Some(fallback_index) = find_single_unlinked_in_flight_session(sessions) {
            fallback_index
        } else {
            usize::MAX
        };

        if matched_index != usize::MAX {
            let Some(object) = sessions
                .get_mut(matched_index)
                .and_then(Value::as_object_mut)
            else {
                return Err("agent_sessions entry must be an object".to_string());
            };
            object.insert("status".to_string(), json!("merged"));
            object.insert("merged_at".to_string(), json!(merged_at));
            object.insert("pr".to_string(), json!(pr));
        } else {
            match issue {
                Some(issue_number) => eprintln!(
                    "Warning: no agent_sessions entry found for PR #{} (issue #{})",
                    pr, issue_number
                ),
                None => eprintln!(
                    "Warning: no agent_sessions entry found for PR #{} (no associated issue)",
                    pr
                ),
            }

            let mut backfill = serde_json::Map::new();
            backfill.insert("pr".to_string(), json!(pr));
            if let Some(issue_number) = issue {
                backfill.insert("issue".to_string(), json!(issue_number));
            }
            backfill.insert("status".to_string(), json!("merged"));
            backfill.insert("merged_at".to_string(), json!(merged_at));
            backfill.insert(
                "title".to_string(),
                json!(format!("Backfilled: PR #{}", pr)),
            );
            backfill.insert("backfilled".to_string(), json!(true));
            sessions.push(json!(backfill));
            eprintln!("Backfilled agent_sessions entry for orphan PR #{}", pr);
        }
    }

    Ok(())
}

fn find_matching_agent_session_indexes(
    sessions: &[Value],
    pr: u64,
    issue: Option<u64>,
) -> Vec<usize> {
    let pr_matches = sessions
        .iter()
        .enumerate()
        .filter_map(|(index, session)| {
            (session.get("pr").and_then(Value::as_u64) == Some(pr)).then_some(index)
        })
        .collect::<Vec<_>>();
    if !pr_matches.is_empty() {
        return pr_matches;
    }

    issue
        .map(|issue_number| {
            sessions
                .iter()
                .enumerate()
                .filter_map(|(index, session)| {
                    (session.get("issue").and_then(Value::as_u64) == Some(issue_number))
                        .then_some(index)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn find_single_unlinked_in_flight_session(sessions: &[Value]) -> Option<usize> {
    let live_session_indexes = sessions
        .iter()
        .enumerate()
        .filter_map(|(index, session)| {
            session
                .get("status")
                .and_then(Value::as_str)
                .filter(|status| matches!(*status, "in_flight" | "dispatched"))
                .map(|_| index)
        })
        .collect::<Vec<_>>();
    if live_session_indexes.len() != 1 {
        return None;
    }

    let candidates = sessions
        .iter()
        .enumerate()
        .filter_map(|(index, session)| {
            let status = session.get("status").and_then(Value::as_str)?;
            let is_live = matches!(status, "in_flight" | "dispatched");
            let has_pr = session.get("pr").is_some_and(|value| !value.is_null());
            let is_backfilled = session
                .get("backfilled")
                .and_then(Value::as_bool)
                .unwrap_or(false);
            (is_live && !has_pr && !is_backfilled).then_some(index)
        })
        .collect::<Vec<_>>();
    (candidates.len() == 1).then_some(candidates[0])
}

fn collapse_agent_session_duplicates(
    sessions: &mut Vec<Value>,
    indexes: &[usize],
) -> Result<usize, String> {
    let Some(mut canonical_index) = earliest_dispatched_session_index(sessions, indexes)? else {
        return Err("agent_sessions duplicate collapse requires at least one entry".to_string());
    };
    let mut duplicate_indexes = indexes
        .iter()
        .copied()
        .filter(|index| *index != canonical_index)
        .collect::<Vec<_>>();
    duplicate_indexes.sort_unstable_by(|left, right| right.cmp(left));

    for duplicate_index in duplicate_indexes {
        let duplicate = sessions.remove(duplicate_index);
        if duplicate_index < canonical_index {
            canonical_index -= 1;
        }
        let Some(canonical) = sessions
            .get_mut(canonical_index)
            .and_then(Value::as_object_mut)
        else {
            return Err("agent_sessions entry must be an object".to_string());
        };
        let Some(duplicate_object) = duplicate.as_object() else {
            return Err("agent_sessions entry must be an object".to_string());
        };
        merge_agent_session_fields(canonical, duplicate_object);
    }

    Ok(canonical_index)
}

fn earliest_dispatched_session_index(
    sessions: &[Value],
    indexes: &[usize],
) -> Result<Option<usize>, String> {
    let mut canonical: Option<(usize, Option<DateTime<chrono::Utc>>)> = None;
    for index in indexes {
        let session = sessions
            .get(*index)
            .ok_or_else(|| "agent_sessions entry index out of range".to_string())?;
        let dispatched_at = session
            .get("dispatched_at")
            .and_then(Value::as_str)
            .map(|value| parse_timestamp(value, "agent_sessions[].dispatched_at"))
            .transpose()?;
        canonical = match canonical {
            None => Some((*index, dispatched_at)),
            Some((best_index, best_timestamp)) => {
                if dispatched_timestamp_is_earlier(dispatched_at, best_timestamp) {
                    Some((*index, dispatched_at))
                } else {
                    Some((best_index, best_timestamp))
                }
            }
        };
    }

    Ok(canonical.map(|(index, _)| index))
}

fn dispatched_timestamp_is_earlier(
    candidate: Option<DateTime<chrono::Utc>>,
    current: Option<DateTime<chrono::Utc>>,
) -> bool {
    match (candidate, current) {
        (Some(candidate), Some(current)) => candidate < current,
        (Some(_), None) => true,
        _ => false,
    }
}

fn merge_agent_session_fields(
    canonical: &mut serde_json::Map<String, Value>,
    duplicate: &serde_json::Map<String, Value>,
) {
    for (key, value) in duplicate {
        if key == "backfilled" || !value_is_present(value) {
            continue;
        }
        let should_copy = canonical
            .get(key)
            .map(|existing| !value_is_present(existing))
            .unwrap_or(true);
        if should_copy {
            canonical.insert(key.clone(), value.clone());
        }
    }
}

fn value_is_present(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::String(text) => !text.trim().is_empty(),
        Value::Array(items) => !items.is_empty(),
        Value::Object(entries) => !entries.is_empty(),
        _ => true,
    }
}

fn sync_last_cycle_summary(state: &mut Value, current_cycle: u64) -> Result<(), String> {
    let Some(last_cycle_number) = state.pointer("/last_cycle/number").and_then(Value::as_u64)
    else {
        return Ok(());
    };
    if last_cycle_number != current_cycle {
        return Ok(());
    }

    let last_cycle_timestamp = state
        .pointer("/last_cycle/timestamp")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            "missing docs/state.json last_cycle.timestamp for last_cycle.summary sync".to_string()
        })?;
    let cycle_start =
        parse_timestamp(last_cycle_timestamp, "docs/state.json last_cycle.timestamp")?;
    let sessions = state
        .pointer("/agent_sessions")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing array /agent_sessions in docs/state.json".to_string())?;
    let mut dispatches = 0usize;
    let mut merges = 0usize;
    for session in sessions {
        if let Some(dispatched_at) = session.get("dispatched_at").and_then(Value::as_str) {
            if parse_timestamp(dispatched_at, "agent_sessions[].dispatched_at")? >= cycle_start {
                dispatches += 1;
            }
        }
        if let Some(merged_at) = session.get("merged_at").and_then(Value::as_str) {
            if parse_timestamp(merged_at, "agent_sessions[].merged_at")? >= cycle_start {
                merges += 1;
            }
        }
    }

    let last_cycle = state
        .pointer_mut("/last_cycle")
        .and_then(Value::as_object_mut)
        .ok_or_else(|| "missing object /last_cycle in docs/state.json".to_string())?;
    last_cycle.insert(
        "summary".to_string(),
        json!(format!("{dispatches} dispatches, {merges} merges")),
    );
    Ok(())
}

fn parse_timestamp(value: &str, label: &str) -> Result<chrono::DateTime<chrono::Utc>, String> {
    DateTime::parse_from_rfc3339(value)
        .map(|timestamp| timestamp.with_timezone(&chrono::Utc))
        .map_err(|error| format!("invalid {}: {}", label, error))
}

fn format_pr_list(prs: &[u64]) -> String {
    let formatted: Vec<String> = prs.iter().map(|pr| format!("#{}", pr)).collect();
    if formatted.len() == 1 {
        format!("PR {}", formatted[0])
    } else {
        format!("PRs {}", formatted.join(", "))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::error::ErrorKind;
    use clap::CommandFactory;
    use state_schema::default_agent_model;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use std::sync::atomic::{AtomicU64, Ordering};

    fn repo_root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../../..")
    }

    fn default_test_model() -> String {
        default_agent_model(&repo_root()).expect("default model should load from config")
    }

    fn sample_state() -> Value {
        let model = default_test_model();
        json!({
            "agent_sessions": [
                {
                    "issue": 667,
                    "title": "Dispatched issue 667",
                    "dispatched_at": "2026-03-05T10:00:00Z",
                    "model": model.clone(),
                    "status": "in_flight"
                },
                {
                    "issue": 668,
                    "title": "Already linked",
                    "dispatched_at": "2026-03-05T11:00:00Z",
                    "model": model,
                    "status": "in_flight",
                    "pr": 669
                }
            ],
            "in_flight_sessions": 3,
            "last_cycle": {
                "number": 164,
                "timestamp": "2026-03-05T09:00:00Z",
                "summary": "0 dispatches, 0 merges"
            },
            "cycle_phase": {
                "cycle": 164
            },
            "copilot_metrics": {
                "closed_without_merge": 1,
                "closed_without_pr": 1,
                "dispatch_to_pr_rate": "84/85",
                "in_flight": 3,
                "merged": 80,
                "pr_merge_rate": "80/84",
                "produced_pr": 84,
                "resolved": 82,
                "total_dispatches": 85
            },
            "field_inventory": {
                "fields": {
                    "copilot_metrics.in_flight": {"last_refreshed": "cycle 163"},
                    "copilot_metrics.pr_merge_rate": {"last_refreshed": "cycle 163"},
                    "copilot_metrics.dispatch_to_pr_rate": {"last_refreshed": "cycle 163"},
                    "in_flight_sessions": {"last_refreshed": "cycle 163"}
                }
            }
        })
    }

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--prs"));
        assert!(help.contains("--issues"));
        assert!(help.contains("--merged-at"));
        assert!(help.contains("none"));
        assert!(help.contains("--repo-root"));
    }

    #[test]
    fn metric_calculation_single_pr_merge() {
        let state = sample_state();
        let update = compute_update(&state, 164, &[595]).expect("update should compute");
        assert_eq!(update.in_flight, 2);
    }

    #[test]
    fn metric_calculation_multiple_pr_merge() {
        let state = sample_state();
        let update = compute_update(&state, 164, &[595, 597, 599]).expect("update should compute");
        assert_eq!(update.in_flight, 0);
    }

    #[test]
    fn in_flight_underflow_protection_and_invariants() {
        let mut state = sample_state();
        state["in_flight_sessions"] = json!(1);
        let update = compute_update(&state, 164, &[595, 597]).expect("update should compute");
        assert_eq!(update.in_flight, 0);
    }

    #[test]
    fn invariant_validation_detects_mismatch() {
        let mut state = sample_state();
        state["in_flight_sessions"] = json!(-1);
        let error = compute_update(&state, 164, &[595]).expect_err("negative in_flight must fail");
        assert!(error.contains("in_flight_sessions"));
    }

    #[test]
    fn patch_updates_counters_rates_and_freshness_markers() {
        let state = sample_state();
        let update = compute_update(&state, 164, &[595]).expect("update should compute");
        let patch = build_patch(&update).expect("patch should build");
        assert_eq!(
            patch.iter().map(|update| update.path).collect::<Vec<_>>(),
            vec![
                "/in_flight_sessions",
                "/field_inventory/fields/in_flight_sessions/last_refreshed",
            ]
        );
        assert_eq!(patch[0].value, json!(2));
        assert_eq!(patch[1].value, json!("cycle 164"));
    }

    #[test]
    fn compute_update_uses_top_level_in_flight_sessions() {
        let mut state = sample_state();
        state["in_flight_sessions"] = json!(1);
        let update = compute_update(&state, 164, &[595]).expect("update should compute");
        assert_eq!(update.in_flight, 0);
    }

    #[test]
    fn apply_patch_updates_derived_metrics_and_refreshes_field_inventory() {
        let mut state = sample_state();
        let update = compute_update(&state, 164, &[595]).expect("update should compute");
        let patch = build_patch(&update).expect("patch should build");

        apply_patch(&mut state, &patch).expect("patch should apply");

        assert_eq!(state["in_flight_sessions"], json!(2));
        assert_eq!(
            state["field_inventory"]["fields"]["in_flight_sessions"]["last_refreshed"],
            json!("cycle 164")
        );
    }

    #[test]
    fn format_pr_list_handles_single_and_multiple_values() {
        assert_eq!(format_pr_list(&[595]), "PR #595");
        assert_eq!(format_pr_list(&[595, 597]), "PRs #595, #597");
    }

    #[test]
    fn update_agent_sessions_matches_issue_mapping_and_sets_merge_fields() {
        let mut state = sample_state();

        update_agent_sessions(&mut state, &[668], &[667], "2026-03-07T13:00:00Z")
            .expect("agent sessions should update");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions[0]["issue"], json!(667));
        assert_eq!(sessions[0]["status"], json!("merged"));
        assert_eq!(sessions[0]["pr"], json!(668));
        assert_eq!(sessions[0]["merged_at"], json!("2026-03-07T13:00:00Z"));
    }

    #[test]
    fn update_agent_sessions_matches_existing_pr_without_issue_mapping() {
        let mut state = sample_state();

        update_agent_sessions(&mut state, &[669], &[999], "2026-03-07T13:00:00Z")
            .expect("agent sessions should update");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions[1]["status"], json!("merged"));
        assert_eq!(sessions[1]["pr"], json!(669));
        assert_eq!(sessions[1]["merged_at"], json!("2026-03-07T13:00:00Z"));
    }

    #[test]
    fn update_agent_sessions_backfills_when_mapping_is_missing() {
        let mut state = sample_state();

        update_agent_sessions(&mut state, &[700], &[777], "2026-03-07T13:00:00Z")
            .expect("missing session should not fail");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions.len(), 3);

        let session = sessions[2].as_object().expect("backfilled session object");
        assert_eq!(session.get("status"), Some(&json!("merged")));
        assert_eq!(session.get("pr"), Some(&json!(700)));
        assert_eq!(session.get("issue"), Some(&json!(777)));
        assert_eq!(session.get("backfilled"), Some(&json!(true)));
        assert_eq!(
            session.get("merged_at"),
            Some(&json!("2026-03-07T13:00:00Z"))
        );
    }

    fn temp_repo_path(name: &str) -> PathBuf {
        static COUNTER: AtomicU64 = AtomicU64::new(0);
        let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("process-merge-{name}-{run_id}"))
    }

    fn init_git_repo(repo_root: &PathBuf) {
        if repo_root.exists() {
            fs::remove_dir_all(repo_root).expect("remove old temp repo");
        }
        fs::create_dir_all(repo_root).expect("repo root directory");
        let remote_root = repo_root.with_extension("remote.git");
        if remote_root.exists() {
            fs::remove_dir_all(&remote_root).expect("remove old remote repo");
        }
        let status = Command::new("git")
            .arg("init")
            .arg(repo_root)
            .status()
            .expect("git init");
        assert!(status.success());
        let status = Command::new("git")
            .args([
                "-C",
                repo_root.to_str().unwrap(),
                "config",
                "user.name",
                "Test User",
            ])
            .status()
            .expect("git config user.name");
        assert!(status.success());
        let status = Command::new("git")
            .args([
                "-C",
                repo_root.to_str().unwrap(),
                "config",
                "user.email",
                "test@example.com",
            ])
            .status()
            .expect("git config user.email");
        assert!(status.success());
        let status = Command::new("git")
            .arg("init")
            .arg("--bare")
            .arg(&remote_root)
            .status()
            .expect("git init --bare");
        assert!(status.success());
        let status = Command::new("git")
            .current_dir(&remote_root)
            .args(["symbolic-ref", "HEAD", "refs/heads/master"])
            .status()
            .expect("git symbolic-ref");
        assert!(status.success());
        let status = Command::new("git")
            .args([
                "-C",
                repo_root.to_str().unwrap(),
                "remote",
                "add",
                "origin",
                remote_root.to_str().unwrap(),
            ])
            .status()
            .expect("git remote add");
        assert!(status.success());
    }

    fn write_repo_state(repo_root: &Path, state: Value) {
        fs::create_dir_all(repo_root.join("docs")).expect("docs dir");
        fs::write(
            repo_root.join("docs/state.json"),
            format!("{}\n", serde_json::to_string_pretty(&state).unwrap()),
        )
        .expect("write state.json");
    }

    fn read_repo_state(repo_root: &Path) -> Value {
        serde_json::from_str(
            &fs::read_to_string(repo_root.join("docs/state.json")).expect("read state.json"),
        )
        .expect("parse state.json")
    }

    #[test]
    fn run_updates_last_cycle_summary_for_current_cycle_merge() {
        let repo_root = temp_repo_path("summary-current-cycle");
        init_git_repo(&repo_root);
        let model = default_test_model();
        write_repo_state(
            &repo_root,
            json!({
                "agent_sessions": [
                    {
                        "issue": 667,
                        "title": "Dispatched issue 667",
                        "model": model,
                        "status": "in_flight"
                    }
                ],
                "in_flight_sessions": 1,
                "last_cycle": {
                    "number": 164,
                    "timestamp": "2026-03-05T09:00:00Z",
                    "summary": "0 dispatches, 0 merges"
                },
                "cycle_phase": {
                    "cycle": 164
                },
                "field_inventory": {
                    "fields": {
                        "in_flight_sessions": {
                            "last_refreshed": "cycle 163"
                        }
                    }
                }
            }),
        );

        run(Cli {
            prs: vec![700],
            issues: vec![IssueValue::Number(667)],
            merged_at: Some("2026-03-05T12:00:00Z".to_string()),
            repo_root: repo_root.clone(),
        })
        .expect("process-merge should succeed");

        let state = read_repo_state(&repo_root);
        assert_eq!(
            state["last_cycle"]["summary"],
            json!("0 dispatches, 1 merges")
        );
    }

    #[test]
    fn run_counts_existing_current_cycle_dispatches_in_last_cycle_summary() {
        let repo_root = temp_repo_path("summary-with-dispatch");
        init_git_repo(&repo_root);
        let model = default_test_model();
        write_repo_state(
            &repo_root,
            json!({
                "agent_sessions": [
                    {
                        "issue": 667,
                        "title": "Implement feature",
                        "dispatched_at": "2026-03-05T10:00:00Z",
                        "model": model.clone(),
                        "status": "in_flight"
                    },
                    {
                        "issue": 668,
                        "title": "[Cycle Review] review session",
                        "dispatched_at": "2026-03-05T10:05:00Z",
                        "model": model,
                        "status": "reviewed_awaiting_eva",
                        "review_dispatch": true
                    }
                ],
                "in_flight_sessions": 1,
                "last_cycle": {
                    "number": 164,
                    "timestamp": "2026-03-05T09:00:00Z",
                    "summary": "0 dispatches, 0 merges"
                },
                "cycle_phase": {
                    "cycle": 164
                },
                "field_inventory": {
                    "fields": {
                        "in_flight_sessions": {
                            "last_refreshed": "cycle 163"
                        }
                    }
                }
            }),
        );

        run(Cli {
            prs: vec![700],
            issues: vec![IssueValue::Number(667)],
            merged_at: Some("2026-03-05T12:00:00Z".to_string()),
            repo_root: repo_root.clone(),
        })
        .expect("process-merge should succeed");

        let state = read_repo_state(&repo_root);
        assert_eq!(
            state["last_cycle"]["summary"],
            json!("2 dispatches, 1 merges")
        );
    }

    #[test]
    fn run_preserves_review_dispatch_count_recorded_by_record_dispatch() {
        let repo_root = temp_repo_path("summary-review-dispatch-preserved");
        init_git_repo(&repo_root);
        let model = default_test_model();
        write_repo_state(
            &repo_root,
            json!({
                "agent_sessions": [
                    {
                        "issue": 667,
                        "title": "[Cycle Review] review session",
                        "dispatched_at": "2026-03-05T10:05:00Z",
                        "model": model,
                        "status": "in_flight",
                        "review_dispatch": true
                    }
                ],
                "in_flight_sessions": 1,
                "last_cycle": {
                    "number": 164,
                    "timestamp": "2026-03-05T09:00:00Z",
                    "summary": "1 dispatch, 0 merges"
                },
                "cycle_phase": {
                    "cycle": 164
                },
                "field_inventory": {
                    "fields": {
                        "in_flight_sessions": {
                            "last_refreshed": "cycle 163"
                        }
                    }
                }
            }),
        );

        run(Cli {
            prs: vec![700],
            issues: vec![IssueValue::Number(667)],
            merged_at: Some("2026-03-05T12:00:00Z".to_string()),
            repo_root: repo_root.clone(),
        })
        .expect("process-merge should succeed");

        let state = read_repo_state(&repo_root);
        assert_eq!(
            state["last_cycle"]["summary"],
            json!("1 dispatches, 1 merges")
        );
    }

    #[test]
    fn run_leaves_last_cycle_summary_unchanged_when_last_cycle_number_differs() {
        let repo_root = temp_repo_path("summary-mismatched-cycle");
        init_git_repo(&repo_root);
        let model = default_test_model();
        write_repo_state(
            &repo_root,
            json!({
                "agent_sessions": [
                    {
                        "issue": 667,
                        "title": "Implement feature",
                        "dispatched_at": "2026-03-05T10:00:00Z",
                        "model": model,
                        "status": "in_flight"
                    }
                ],
                "in_flight_sessions": 1,
                "last_cycle": {
                    "number": 163,
                    "timestamp": "2026-03-05T09:00:00Z",
                    "summary": "keep existing summary"
                },
                "cycle_phase": {
                    "cycle": 164
                },
                "field_inventory": {
                    "fields": {
                        "in_flight_sessions": {
                            "last_refreshed": "cycle 163"
                        }
                    }
                }
            }),
        );

        run(Cli {
            prs: vec![700],
            issues: vec![IssueValue::Number(667)],
            merged_at: Some("2026-03-05T12:00:00Z".to_string()),
            repo_root: repo_root.clone(),
        })
        .expect("process-merge should succeed");

        let state = read_repo_state(&repo_root);
        assert_eq!(
            state["last_cycle"]["summary"],
            json!("keep existing summary")
        );
    }

    #[test]
    fn update_agent_sessions_backfills_pr_without_issue_mapping() {
        let mut state = sample_state();

        update_agent_sessions(&mut state, &[701], &[], "2026-03-07T13:00:00Z")
            .expect("missing session should backfill without issue");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions.len(), 3);

        let session = sessions[2].as_object().expect("backfilled session object");
        assert_eq!(session.get("status"), Some(&json!("merged")));
        assert_eq!(session.get("pr"), Some(&json!(701)));
        assert_eq!(session.get("issue"), None);
        assert_eq!(session.get("backfilled"), Some(&json!(true)));
        assert_eq!(
            session.get("merged_at"),
            Some(&json!("2026-03-07T13:00:00Z"))
        );
    }

    #[test]
    fn update_agent_sessions_reuses_existing_in_flight_row_when_issue_mapping_is_wrong() {
        let model = default_test_model();
        let mut state = json!({
            "agent_sessions": [
                {
                    "issue": 2298,
                    "title": "Real originating session",
                    "dispatched_at": "2026-04-08T08:00:00Z",
                    "model": model,
                    "status": "in_flight"
                }
            ]
        });

        update_agent_sessions(&mut state, &[2299], &[2300], "2026-04-08T09:00:00Z")
            .expect("wrong issue mapping should still update the existing session");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0]["issue"], json!(2298));
        assert_eq!(sessions[0]["pr"], json!(2299));
        assert_eq!(sessions[0]["status"], json!("merged"));
        assert_eq!(sessions[0]["merged_at"], json!("2026-04-08T09:00:00Z"));
    }

    #[test]
    fn cli_requires_issues_flag() {
        let error = Cli::try_parse_from(["process-merge", "--prs", "595"])
            .expect_err("--issues should be required");

        assert_eq!(error.kind(), ErrorKind::MissingRequiredArgument);
        assert!(error.to_string().contains("--issues"));
    }

    #[test]
    fn cli_accepts_none_as_empty_issue_list_and_updates_pr_only_session() {
        let cli = Cli::try_parse_from(["process-merge", "--prs", "669", "--issues", "none"])
            .expect("--issues none should parse");
        let issues = normalize_issues(&cli.issues, cli.prs.len()).expect("none should normalize");
        assert!(issues.is_empty());

        let mut state = sample_state();
        update_agent_sessions(&mut state, &cli.prs, &issues, "2026-03-07T13:00:00Z")
            .expect("empty issue mapping should still allow PR matches");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        let session = sessions[1].as_object().expect("session object");
        assert_eq!(session.get("status"), Some(&json!("merged")));
        assert_eq!(session.get("pr"), Some(&json!(669)));
        assert_eq!(
            session.get("merged_at"),
            Some(&json!("2026-03-07T13:00:00Z"))
        );
    }

    #[test]
    fn cli_accepts_numeric_issue_values() {
        let cli = Cli::try_parse_from(["process-merge", "--prs", "668", "--issues", "667"])
            .expect("numeric issues should parse");
        let issues =
            normalize_issues(&cli.issues, cli.prs.len()).expect("numeric issues should normalize");
        assert_eq!(issues, vec![667]);

        let mut state = sample_state();
        update_agent_sessions(&mut state, &cli.prs, &issues, "2026-03-07T13:00:00Z")
            .expect("numeric issue mapping should update agent sessions");

        let sessions = state["agent_sessions"]
            .as_array()
            .expect("agent_sessions array");
        assert_eq!(sessions[0]["issue"], json!(667));
        assert_eq!(sessions[0]["status"], json!("merged"));
        assert_eq!(sessions[0]["pr"], json!(668));
        assert_eq!(sessions[0]["merged_at"], json!("2026-03-07T13:00:00Z"));
    }

    #[test]
    fn cli_accepts_explicit_merged_at_timestamp() {
        let cli = Cli::try_parse_from([
            "process-merge",
            "--prs",
            "668",
            "--issues",
            "667",
            "--merged-at",
            "2026-03-07T12:34:56Z",
        ])
        .expect("merged-at should parse");

        assert_eq!(cli.merged_at.as_deref(), Some("2026-03-07T12:34:56Z"));
        assert_eq!(
            resolve_merged_at(cli.merged_at.as_deref(), || "fallback".to_string()),
            "2026-03-07T12:34:56Z"
        );
    }

    #[test]
    fn cli_rejects_invalid_merged_at_timestamp() {
        let error = Cli::try_parse_from([
            "process-merge",
            "--prs",
            "668",
            "--issues",
            "667",
            "--merged-at",
            "not-a-timestamp",
        ])
        .expect_err("invalid merged-at should be rejected");

        assert_eq!(error.kind(), ErrorKind::ValueValidation);
        assert!(error.to_string().contains("--merged-at"));
    }

    #[test]
    fn resolve_merged_at_uses_fallback_when_flag_omitted() {
        assert_eq!(
            resolve_merged_at(None, || "2026-03-08T01:02:03Z".to_string()),
            "2026-03-08T01:02:03Z"
        );
    }
}
