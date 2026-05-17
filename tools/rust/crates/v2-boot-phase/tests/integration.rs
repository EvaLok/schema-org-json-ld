use std::fs;
use std::path::Path;
use std::process::Command;

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_v2-boot-phase")
}

fn write_cycle(state_dir: &Path, n: u64, model: &str, started_at: &str) {
    let path = state_dir.join(format!("{n}.json"));
    let content = format!(
        r#"{{"cycle_number": {n}, "model": "{model}", "started_at": "{started_at}"}}"#
    );
    fs::write(path, content).unwrap();
}

/// Default-empty fixture directory under the temp repo root. Keeps the integration
/// tests offline by pointing the GH stages at an empty fixture instead of letting
/// them shell out to `gh`.
fn fixture_dir(repo_root: &Path) -> std::path::PathBuf {
    let dir = repo_root.join("gh-fixtures");
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn run(repo_root: &Path, args: &[&str]) -> (i32, String, String) {
    let dir = fixture_dir(repo_root);
    let mut cmd = Command::new(bin());
    cmd.arg("--repo-root").arg(repo_root);
    cmd.arg("--fixture-dir").arg(&dir);
    cmd.args(args);
    let out = cmd.output().unwrap();
    let code = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    (code, stdout, stderr)
}

fn run_with_fixture_dir(
    repo_root: &Path,
    fixture_dir: &Path,
    args: &[&str],
) -> (i32, String, String) {
    let mut cmd = Command::new(bin());
    cmd.arg("--repo-root").arg(repo_root);
    cmd.arg("--fixture-dir").arg(fixture_dir);
    cmd.args(args);
    let out = cmd.output().unwrap();
    let code = out.status.code().unwrap_or(-1);
    let stdout = String::from_utf8_lossy(&out.stdout).to_string();
    let stderr = String::from_utf8_lossy(&out.stderr).to_string();
    (code, stdout, stderr)
}

#[test]
fn empty_state_dir_warn_treated_as_first_cycle() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("cycles_loaded"),
        Some(&serde_json::Value::Number(0.into()))
    );
    assert!(parsed
        .get("cycle_context")
        .unwrap()
        .get("current_cycle")
        .unwrap()
        .is_null());
}

#[test]
fn missing_state_dir_warn_not_fail() {
    let tmp = tempfile::tempdir().unwrap();
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let load = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "load-cycle-history")
        .unwrap();
    assert_eq!(load.get("status").unwrap(), "warn");
}

#[test]
fn missing_state_dir_strict_exits_one() {
    let tmp = tempfile::tempdir().unwrap();
    let (code, _stdout, _err) = run(tmp.path(), &["--strict", "--format", "json"]);
    assert_eq!(code, 1);
}

#[test]
fn state_path_is_file_fails() {
    let tmp = tempfile::tempdir().unwrap();
    let state_parent = tmp.path().join("state");
    fs::create_dir_all(&state_parent).unwrap();
    fs::write(state_parent.join("cycle-history"), "not a dir").unwrap();
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 1);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let load = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "load-cycle-history")
        .unwrap();
    assert_eq!(load.get("status").unwrap(), "failed");
}

#[test]
fn single_entry_skips_gap_detection_and_advances_cursor() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 42, "claude-opus-4-7", "2026-05-11T22:30:00Z");
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("current_cycle"),
        Some(&serde_json::Value::Number(43.into()))
    );
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("previous_cycle"),
        Some(&serde_json::Value::Number(42.into()))
    );
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let gap = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "detect-gaps")
        .unwrap();
    assert_eq!(gap.get("status").unwrap(), "skipped");
}

#[test]
fn contiguous_entries_no_gaps() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    for n in 100..=103 {
        write_cycle(&state, n, "claude-opus-4-7", "2026-05-11T22:30:00Z");
    }
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("current_cycle"),
        Some(&serde_json::Value::Number(104.into()))
    );
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("cycles_loaded"),
        Some(&serde_json::Value::Number(4.into()))
    );
    let gaps = parsed
        .get("cycle_context")
        .unwrap()
        .get("gaps")
        .unwrap()
        .as_array()
        .unwrap();
    assert!(gaps.is_empty());
}

#[test]
fn gap_detected_warn_status() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    write_cycle(&state, 2, "m", "2026-05-11T22:30:00Z");
    write_cycle(&state, 5, "m", "2026-05-11T22:30:00Z");
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let gaps = parsed
        .get("cycle_context")
        .unwrap()
        .get("gaps")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(gaps.len(), 1);
    assert_eq!(
        gaps[0].get("missing_count"),
        Some(&serde_json::Value::Number(2.into()))
    );
}

#[test]
fn gap_strict_mode_exits_one() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    write_cycle(&state, 3, "m", "2026-05-11T22:30:00Z");
    let (code, _stdout, _err) = run(tmp.path(), &["--strict", "--format", "json"]);
    assert_eq!(code, 1);
}

#[test]
fn malformed_entry_warn_not_fail() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    // malformed: missing cycle_number
    fs::write(
        state.join("2.json"),
        r#"{"model": "m", "started_at": "2026-05-11T22:30:00Z"}"#,
    )
    .unwrap();
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let load = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "load-cycle-history")
        .unwrap();
    assert_eq!(load.get("status").unwrap(), "warn");
    let details = load.get("details").unwrap().as_str().unwrap();
    assert!(details.contains("malformed"));
}

#[test]
fn non_numeric_filenames_ignored() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    fs::write(state.join("README.json"), "not-an-entry").unwrap();
    fs::write(state.join("notes.md"), "ignored").unwrap();
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("cycles_loaded"),
        Some(&serde_json::Value::Number(1.into()))
    );
}

#[test]
fn previous_cycle_summary_populated() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 122, "claude-opus-4-7", "2026-05-11T20:31:00Z");
    let (_code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let summary = parsed
        .get("cycle_context")
        .unwrap()
        .get("previous_cycle_summary")
        .unwrap();
    assert_eq!(
        summary.get("cycle_number"),
        Some(&serde_json::Value::Number(122.into()))
    );
    assert_eq!(
        summary.get("model"),
        Some(&serde_json::Value::String("claude-opus-4-7".into()))
    );
    assert_eq!(
        summary.get("started_at"),
        Some(&serde_json::Value::String("2026-05-11T20:31:00Z".into()))
    );
}

#[test]
fn standing_directives_and_gardening_done_with_empty_fixtures() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    let (_code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let directives = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "check-standing-directives")
        .unwrap();
    assert_eq!(directives.get("status").unwrap(), "done");
    let gardening = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "identify-gardening-candidates")
        .unwrap();
    assert_eq!(gardening.get("status").unwrap(), "done");
    let standing_in_ctx = parsed
        .get("cycle_context")
        .unwrap()
        .get("standing_directives")
        .unwrap();
    assert_eq!(
        standing_in_ctx.get("implemented"),
        Some(&serde_json::Value::Bool(true))
    );
    assert!(standing_in_ctx
        .get("items")
        .unwrap()
        .as_array()
        .unwrap()
        .is_empty());
    let gardening_in_ctx = parsed
        .get("cycle_context")
        .unwrap()
        .get("gardening_candidates")
        .unwrap();
    assert_eq!(
        gardening_in_ctx.get("implemented"),
        Some(&serde_json::Value::Bool(true))
    );
}

#[test]
fn standing_directives_filters_to_eva_authored_only() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    let fixture_dir = tmp.path().join("gh-fixtures");
    fs::create_dir_all(&fixture_dir).unwrap();
    fs::write(
        fixture_dir.join("input-from-eva.json"),
        r#"[
            {"number": 1, "title": "Real directive",
             "author": {"login": "EvaLok"},
             "createdAt": "2026-01-01T00:00:00Z",
             "labels": [{"name": "input-from-eva"}]},
            {"number": 2, "title": "Spoof",
             "author": {"login": "attacker"},
             "createdAt": "2026-05-01T00:00:00Z",
             "labels": [{"name": "input-from-eva"}]}
        ]"#,
    )
    .unwrap();
    let (code, stdout, _err) =
        run_with_fixture_dir(tmp.path(), &fixture_dir, &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let directives = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "check-standing-directives")
        .unwrap();
    // 1 spoof rejected -> Warn
    assert_eq!(directives.get("status").unwrap(), "warn");
    let items = parsed
        .get("cycle_context")
        .unwrap()
        .get("standing_directives")
        .unwrap()
        .get("items")
        .unwrap()
        .as_array()
        .unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].get("author_login").unwrap(), "EvaLok");
    assert_eq!(items[0].get("number").unwrap(), 1);
}

#[test]
fn gardening_candidates_surfaces_issues_and_draft_prs() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    let fixture_dir = tmp.path().join("gh-fixtures");
    fs::create_dir_all(&fixture_dir).unwrap();
    fs::write(
        fixture_dir.join("open-issues.json"),
        r#"[
            {"number": 2879, "title": "Eva dispatch test",
             "labels": [{"name": "agent-task"}],
             "createdAt": "2026-04-01T00:00:00Z"},
            {"number": 2916, "title": "Cycle issue",
             "labels": [{"name": "orchestrator-run"}],
             "createdAt": "2026-05-12T00:21:24Z"}
        ]"#,
    )
    .unwrap();
    fs::write(
        fixture_dir.join("open-prs.json"),
        r#"[
            {"number": 100, "title": "draft pr", "isDraft": true,
             "createdAt": "2026-04-01T00:00:00Z", "labels": []},
            {"number": 101, "title": "ready pr", "isDraft": false,
             "createdAt": "2026-05-01T00:00:00Z", "labels": []}
        ]"#,
    )
    .unwrap();
    let (code, stdout, _err) =
        run_with_fixture_dir(tmp.path(), &fixture_dir, &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let items = parsed
        .get("cycle_context")
        .unwrap()
        .get("gardening_candidates")
        .unwrap()
        .get("items")
        .unwrap()
        .as_array()
        .unwrap();
    // 1 stale-issue (2879) + 1 open-draft-pr (100); orchestrator-run (2916) and ready PR (101) excluded
    assert_eq!(items.len(), 2);
    let numbers: Vec<u64> = items
        .iter()
        .map(|i| i.get("number").unwrap().as_u64().unwrap())
        .collect();
    assert!(numbers.contains(&2879));
    assert!(numbers.contains(&100));
    assert!(!numbers.contains(&2916));
    assert!(!numbers.contains(&101));
}

#[test]
fn text_format_renders_human_readable() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 10, "m", "2026-05-11T22:30:00Z");
    let (code, stdout, _err) = run(tmp.path(), &["--format", "text"]);
    assert_eq!(code, 0);
    assert!(stdout.contains("v2-boot-phase report"));
    assert!(stdout.contains("Stages:"));
    assert!(stdout.contains("current_cycle:  11"));
    assert!(stdout.contains("DONE"));
    assert!(stdout.contains("standing_directives"));
    assert!(stdout.contains("gardening_candidates"));
}

#[test]
fn output_to_file_writes_target() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    let target = tmp.path().join("out").join("context.json");
    let (code, stdout, _err) = run(
        tmp.path(),
        &["--format", "json", "--output", target.to_str().unwrap()],
    );
    assert_eq!(code, 0);
    assert!(stdout.is_empty());
    let content = fs::read_to_string(&target).unwrap();
    let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("current_cycle"),
        Some(&serde_json::Value::Number(2.into()))
    );
}

#[test]
fn custom_state_dir_arg() {
    let tmp = tempfile::tempdir().unwrap();
    let alt_state = tmp.path().join("alt").join("history");
    fs::create_dir_all(&alt_state).unwrap();
    write_cycle(&alt_state, 99, "m", "2026-05-11T22:30:00Z");
    let (code, stdout, _err) = run(
        tmp.path(),
        &["--state-dir", "alt/history", "--format", "json"],
    );
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(
        parsed.get("cycle_context").unwrap().get("current_cycle"),
        Some(&serde_json::Value::Number(100.into()))
    );
}

#[test]
fn invalid_json_payload_counted_as_malformed() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    fs::write(state.join("2.json"), "{ this is not json").unwrap();
    let (code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    assert_eq!(code, 0);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let stages = parsed.get("stages").unwrap().as_array().unwrap();
    let load = stages
        .iter()
        .find(|s| s.get("name").unwrap() == "load-cycle-history")
        .unwrap();
    assert_eq!(load.get("status").unwrap(), "warn");
}

#[test]
fn summary_counts_stages_by_status() {
    let tmp = tempfile::tempdir().unwrap();
    let state = tmp.path().join("state").join("cycle-history");
    fs::create_dir_all(&state).unwrap();
    write_cycle(&state, 1, "m", "2026-05-11T22:30:00Z");
    write_cycle(&state, 2, "m", "2026-05-11T22:30:00Z");
    let (_code, stdout, _err) = run(tmp.path(), &["--format", "json"]);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let sm = parsed.get("summary").unwrap();
    // load=done compute=done detect=done directives=done(empty fixture) gardening=done(empty fixture) assemble=done -> 6 done
    assert_eq!(
        sm.get("done"),
        Some(&serde_json::Value::Number(6.into()))
    );
    assert_eq!(
        sm.get("deferred"),
        Some(&serde_json::Value::Number(0.into()))
    );
}
