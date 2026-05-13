use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

fn bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_v2-close-phase"))
}

fn run(args: &[&str]) -> (i32, String, String) {
    let output = Command::new(bin())
        .args(args)
        .output()
        .expect("failed to execute v2-close-phase");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stdout).to_string(),
        String::from_utf8_lossy(&output.stderr).to_string(),
    )
}

fn write_fixture(dir: &Path, name: &str, content: &str) {
    fs::write(dir.join(name), content).expect("failed to write fixture");
}

fn parse_json(s: &str) -> Value {
    serde_json::from_str(s).unwrap_or_else(|e| panic!("invalid JSON: {e}\n{s}"))
}

// -----------------------------------------------------------------------------
// All-skipped baseline
// -----------------------------------------------------------------------------

#[test]
fn all_stages_skipped_exits_zero() {
    let (code, stdout, _stderr) = run(&[
        "--cycle-n",
        "136",
        "--skip-gardening",
        "--skip-history-append",
        "--skip-git-push",
        "--skip-issue-close",
    ]);
    assert_eq!(code, 0, "expected exit 0, got {code}\n{_stderr}");
    let v = parse_json(&stdout);
    assert_eq!(v["cycle_n"], 136);
    assert_eq!(v["summary"]["done"], 0);
    assert_eq!(v["summary"]["warn"], 0);
    assert_eq!(v["summary"]["failed"], 0);
    assert_eq!(v["summary"]["skipped"], 4);
    // receipt-scaffold is always Deferred at SCAFFOLD-PARTIAL
    assert_eq!(v["summary"]["deferred"], 1);
}

#[test]
fn schema_emitted() {
    let (_, stdout, _) = run(&[
        "--cycle-n",
        "1",
        "--skip-gardening",
        "--skip-history-append",
        "--skip-git-push",
        "--skip-issue-close",
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["schema"], "v2-close-phase/v1");
}

#[test]
fn strict_promotes_deferred_receipt_to_exit_1() {
    let (code, _, _) = run(&[
        "--cycle-n",
        "1",
        "--strict",
        "--skip-gardening",
        "--skip-history-append",
        "--skip-git-push",
        "--skip-issue-close",
    ]);
    // receipt-scaffold is Deferred at SCAFFOLD-PARTIAL → strict yields exit 1
    assert_eq!(code, 1);
}

// -----------------------------------------------------------------------------
// Fixture mode: all four stages fed from fixtures
// -----------------------------------------------------------------------------

fn full_fixture() -> TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    write_fixture(
        dir.path(),
        "gardening-sweep.json",
        r#"{"schema":"v2-gardening-sweep/v1","scanned_count":42,"stale":[],"dead_links":[]}"#,
    );
    write_fixture(
        dir.path(),
        "cycle-history-append.txt",
        "state/cycle-history/136.json\n",
    );
    write_fixture(
        dir.path(),
        "git-commit-sha.txt",
        "abc1234567def890\n",
    );
    write_fixture(
        dir.path(),
        "issue-comment-url.txt",
        "https://github.com/EvaLok/schema-org-json-ld/issues/2928#issuecomment-1\n",
    );
    dir
}

#[test]
fn fixture_mode_all_done() {
    let dir = full_fixture();
    let fixture_path = dir.path().to_string_lossy().to_string();
    let (code, stdout, _) = run(&[
        "--cycle-n",
        "136",
        "--fixture-dir",
        &fixture_path,
    ]);
    assert_eq!(code, 0);
    let v = parse_json(&stdout);
    assert_eq!(v["summary"]["done"], 4);
    assert_eq!(v["summary"]["deferred"], 1); // receipt-scaffold
    assert_eq!(v["summary"]["failed"], 0);
    assert_eq!(v["fixture_mode"], true);
    assert_eq!(v["dry_run"], true); // fixture_dir implies dry_run
    assert_eq!(v["receipt"]["cycle_history_path"], "state/cycle-history/136.json");
    assert_eq!(v["receipt"]["commit_sha"], "abc1234567def890");
    assert_eq!(
        v["receipt"]["issue_comment_url"],
        "https://github.com/EvaLok/schema-org-json-ld/issues/2928#issuecomment-1"
    );
    assert_eq!(v["receipt"]["gardening_findings_count"], 0);
    // DEFERRED fields are null
    assert_eq!(v["receipt"]["state_pointer_check"], Value::Null);
    assert_eq!(v["receipt"]["push_confirmation"], Value::Null);
}

#[test]
fn fixture_gardening_with_findings_warns() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_fixture(
        dir.path(),
        "gardening-sweep.json",
        r#"{"schema":"v2-gardening-sweep/v1","stale":[{"path":"a.md"}],"dead_links":[{"path":"b.md","ref":"c.md"}]}"#,
    );
    write_fixture(
        dir.path(),
        "cycle-history-append.txt",
        "state/cycle-history/136.json",
    );
    write_fixture(dir.path(), "git-commit-sha.txt", "deadbeefcafe");
    write_fixture(dir.path(), "issue-comment-url.txt", "https://example/url");
    let p = dir.path().to_string_lossy().to_string();
    let (code, stdout, _) = run(&["--cycle-n", "1", "--fixture-dir", &p]);
    assert_eq!(code, 0);
    let v = parse_json(&stdout);
    let stages = v["stages"].as_array().unwrap();
    let gs = stages.iter().find(|s| s["name"] == "gardening-sweep").unwrap();
    assert_eq!(gs["status"], "warn");
    assert!(gs["details"].as_str().unwrap().contains("2 findings"));
    assert_eq!(v["receipt"]["gardening_findings_count"], 2);
}

#[test]
fn fixture_gardening_findings_with_strict_exits_1() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_fixture(
        dir.path(),
        "gardening-sweep.json",
        r#"{"stale":[],"dead_links":[{"path":"x"}]}"#,
    );
    write_fixture(
        dir.path(),
        "cycle-history-append.txt",
        "state/cycle-history/1.json",
    );
    write_fixture(dir.path(), "git-commit-sha.txt", "feed");
    write_fixture(dir.path(), "issue-comment-url.txt", "u");
    let p = dir.path().to_string_lossy().to_string();
    let (code, _, _) = run(&["--cycle-n", "1", "--strict", "--fixture-dir", &p]);
    // Warn (gardening) + Deferred (receipt) → strict yields exit 1
    assert_eq!(code, 1);
}

#[test]
fn fixture_missing_gardening_fails() {
    let dir = tempfile::tempdir().expect("tempdir");
    // Only history fixture present
    write_fixture(
        dir.path(),
        "cycle-history-append.txt",
        "state/cycle-history/1.json",
    );
    let p = dir.path().to_string_lossy().to_string();
    let (code, stdout, _) = run(&[
        "--cycle-n",
        "1",
        "--fixture-dir",
        &p,
        "--skip-git-push",
        "--skip-issue-close",
    ]);
    // gardening-sweep fixture missing → Failed → exit 2
    assert_eq!(code, 2);
    let v = parse_json(&stdout);
    let gs = v["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "gardening-sweep")
        .unwrap();
    assert_eq!(gs["status"], "failed");
}

#[test]
fn fixture_malformed_gardening_json_fails() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_fixture(dir.path(), "gardening-sweep.json", "{not json");
    write_fixture(
        dir.path(),
        "cycle-history-append.txt",
        "state/cycle-history/1.json",
    );
    write_fixture(dir.path(), "git-commit-sha.txt", "x");
    write_fixture(dir.path(), "issue-comment-url.txt", "x");
    let p = dir.path().to_string_lossy().to_string();
    let (code, stdout, _) = run(&["--cycle-n", "1", "--fixture-dir", &p]);
    assert_eq!(code, 2);
    let v = parse_json(&stdout);
    let gs = v["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "gardening-sweep")
        .unwrap();
    assert_eq!(gs["status"], "failed");
    assert!(gs["details"].as_str().unwrap().contains("parse error"));
}

// -----------------------------------------------------------------------------
// Dry-run mode (no fixture)
// -----------------------------------------------------------------------------

#[test]
fn dry_run_all_stages_yields_skipped_or_warn() {
    let (code, stdout, _) = run(&[
        "--cycle-n",
        "136",
        "--dry-run",
        "--gardening-corpus",
        "docs/redesign/_notes",
        "--history-payload",
        "/tmp/nonexistent.json",
        "--journal-path",
        "docs/journal/2026-05-13.md",
        "--issue-number",
        "2928",
        "--close-comment-body",
        "/tmp/nonexistent-body.md",
    ]);
    assert_eq!(code, 0);
    let v = parse_json(&stdout);
    let summary = &v["summary"];
    // 4 stages → all Skipped (dry-run); receipt is Deferred
    assert_eq!(summary["skipped"], 4);
    assert_eq!(summary["deferred"], 1);
    assert_eq!(summary["failed"], 0);
    assert_eq!(v["dry_run"], true);
    assert_eq!(v["fixture_mode"], false);
}

#[test]
fn dry_run_no_corpus_warns() {
    let (_, stdout, _) = run(&[
        "--cycle-n",
        "1",
        "--dry-run",
        "--skip-history-append",
        "--skip-git-push",
        "--skip-issue-close",
    ]);
    let v = parse_json(&stdout);
    let gs = v["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "gardening-sweep")
        .unwrap();
    // dry-run + no corpus → still Warn (corpus precondition checked before dry-run gate)
    assert_eq!(gs["status"], "warn");
    assert!(gs["details"].as_str().unwrap().contains("no --gardening-corpus"));
}

#[test]
fn dry_run_no_history_payload_warns() {
    let (_, stdout, _) = run(&[
        "--cycle-n",
        "1",
        "--dry-run",
        "--skip-gardening",
        "--skip-git-push",
        "--skip-issue-close",
    ]);
    let v = parse_json(&stdout);
    let ch = v["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "cycle-history-append")
        .unwrap();
    assert_eq!(ch["status"], "warn");
    assert!(ch["details"].as_str().unwrap().contains("--history-payload"));
}

#[test]
fn dry_run_no_journal_path_warns() {
    let (_, stdout, _) = run(&[
        "--cycle-n",
        "1",
        "--dry-run",
        "--skip-gardening",
        "--skip-history-append",
        "--skip-issue-close",
    ]);
    let v = parse_json(&stdout);
    let gp = v["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "journal-commit-push")
        .unwrap();
    assert_eq!(gp["status"], "warn");
}

#[test]
fn dry_run_no_issue_number_warns() {
    let (_, stdout, _) = run(&[
        "--cycle-n",
        "1",
        "--dry-run",
        "--skip-gardening",
        "--skip-history-append",
        "--skip-git-push",
    ]);
    let v = parse_json(&stdout);
    let ic = v["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "issue-close")
        .unwrap();
    assert_eq!(ic["status"], "warn");
    assert!(ic["details"].as_str().unwrap().contains("--issue-number"));
}

#[test]
fn dry_run_no_close_comment_body_warns() {
    let (_, stdout, _) = run(&[
        "--cycle-n",
        "1",
        "--dry-run",
        "--issue-number",
        "9",
        "--skip-gardening",
        "--skip-history-append",
        "--skip-git-push",
    ]);
    let v = parse_json(&stdout);
    let ic = v["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "issue-close")
        .unwrap();
    assert_eq!(ic["status"], "warn");
    assert!(ic["details"].as_str().unwrap().contains("--close-comment-body"));
}

// -----------------------------------------------------------------------------
// Output formats
// -----------------------------------------------------------------------------

#[test]
fn text_format_renders_summary_line() {
    let (_, stdout, _) = run(&[
        "--cycle-n",
        "42",
        "--format",
        "text",
        "--skip-gardening",
        "--skip-history-append",
        "--skip-git-push",
        "--skip-issue-close",
    ]);
    assert!(stdout.contains("v2-close-phase cycle 42"));
    assert!(stdout.contains("done=0"));
    assert!(stdout.contains("skipped=4"));
    assert!(stdout.contains("[DEFER] receipt-scaffold"));
}

#[test]
fn text_format_includes_status_labels() {
    let dir = full_fixture();
    let p = dir.path().to_string_lossy().to_string();
    let (_, stdout, _) = run(&[
        "--cycle-n",
        "136",
        "--format",
        "text",
        "--fixture-dir",
        &p,
    ]);
    assert!(stdout.contains("[DONE ] gardening-sweep"));
    assert!(stdout.contains("[DONE ] cycle-history-append"));
    assert!(stdout.contains("[DONE ] journal-commit-push"));
    assert!(stdout.contains("[DONE ] issue-close"));
    assert!(stdout.contains("[DEFER] receipt-scaffold"));
}

#[test]
fn output_to_file_writes_serialized_report() {
    let dir = tempfile::tempdir().expect("tempdir");
    let out_path = dir.path().join("report.json");
    let (_, stdout, _) = run(&[
        "--cycle-n",
        "9",
        "--skip-gardening",
        "--skip-history-append",
        "--skip-git-push",
        "--skip-issue-close",
        "--output",
        &out_path.to_string_lossy(),
    ]);
    // Empty stdout when --output != "-"
    assert!(stdout.is_empty(), "stdout: {stdout}");
    let contents = fs::read_to_string(&out_path).unwrap();
    let v: Value = serde_json::from_str(&contents).unwrap();
    assert_eq!(v["cycle_n"], 9);
}

// -----------------------------------------------------------------------------
// Receipt scaffold
// -----------------------------------------------------------------------------

#[test]
fn receipt_scaffold_always_deferred() {
    let dir = full_fixture();
    let p = dir.path().to_string_lossy().to_string();
    let (_, stdout, _) = run(&["--cycle-n", "1", "--fixture-dir", &p]);
    let v = parse_json(&stdout);
    let rs = v["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "receipt-scaffold")
        .unwrap();
    assert_eq!(rs["status"], "deferred");
    assert!(rs["details"].as_str().unwrap().contains("4/4 artifacts captured"));
    assert!(rs["details"].as_str().unwrap().contains("DEFERRED"));
}

#[test]
fn receipt_scaffold_partial_capture() {
    // Only gardening fixture; other stages skipped
    let dir = tempfile::tempdir().expect("tempdir");
    write_fixture(
        dir.path(),
        "gardening-sweep.json",
        r#"{"stale":[],"dead_links":[]}"#,
    );
    let p = dir.path().to_string_lossy().to_string();
    let (_, stdout, _) = run(&[
        "--cycle-n",
        "1",
        "--fixture-dir",
        &p,
        "--skip-history-append",
        "--skip-git-push",
        "--skip-issue-close",
    ]);
    let v = parse_json(&stdout);
    let rs = v["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "receipt-scaffold")
        .unwrap();
    assert!(rs["details"].as_str().unwrap().contains("1/4 artifacts captured"));
}

#[test]
fn receipt_scaffold_zero_capture_when_all_skipped() {
    let (_, stdout, _) = run(&[
        "--cycle-n",
        "1",
        "--skip-gardening",
        "--skip-history-append",
        "--skip-git-push",
        "--skip-issue-close",
    ]);
    let v = parse_json(&stdout);
    let rs = v["stages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "receipt-scaffold")
        .unwrap();
    assert!(rs["details"].as_str().unwrap().contains("0/4 artifacts captured"));
}

// -----------------------------------------------------------------------------
// Validation / error paths
// -----------------------------------------------------------------------------

#[test]
fn missing_required_cycle_n_fails() {
    let (code, _, stderr) = run(&[]);
    assert_ne!(code, 0);
    assert!(stderr.contains("--cycle-n") || stderr.contains("cycle-n"));
}

#[test]
fn stage_order_is_stable() {
    let (_, stdout, _) = run(&[
        "--cycle-n",
        "1",
        "--skip-gardening",
        "--skip-history-append",
        "--skip-git-push",
        "--skip-issue-close",
    ]);
    let v = parse_json(&stdout);
    let names: Vec<&str> = v["stages"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["name"].as_str().unwrap())
        .collect();
    assert_eq!(
        names,
        vec![
            "gardening-sweep",
            "cycle-history-append",
            "journal-commit-push",
            "issue-close",
            "receipt-scaffold",
        ]
    );
}

#[test]
fn dry_run_when_fixture_dir_set() {
    let dir = full_fixture();
    let p = dir.path().to_string_lossy().to_string();
    let (_, stdout, _) = run(&["--cycle-n", "1", "--fixture-dir", &p]);
    let v = parse_json(&stdout);
    // fixture_dir implies dry_run
    assert_eq!(v["dry_run"], true);
    assert_eq!(v["fixture_mode"], true);
}

#[test]
fn exit_code_failed_outranks_warn() {
    let dir = tempfile::tempdir().expect("tempdir");
    // Gardening with findings (Warn); missing history fixture (Failed)
    write_fixture(
        dir.path(),
        "gardening-sweep.json",
        r#"{"stale":[{}],"dead_links":[{}]}"#,
    );
    // No cycle-history-append.txt → Failed
    write_fixture(dir.path(), "git-commit-sha.txt", "x");
    write_fixture(dir.path(), "issue-comment-url.txt", "y");
    let p = dir.path().to_string_lossy().to_string();
    let (code, _, _) = run(&["--cycle-n", "1", "--fixture-dir", &p]);
    // Failed → exit 2 regardless of strict
    assert_eq!(code, 2);
}
