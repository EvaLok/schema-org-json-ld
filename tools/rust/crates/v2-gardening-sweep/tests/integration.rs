use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn binary_path() -> PathBuf {
    let mut path = std::env::current_exe().expect("current_exe");
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.join("v2-gardening-sweep")
}

fn touch_old(path: &Path, days_ago: u64) {
    let status = Command::new("touch")
        .arg("-d")
        .arg(format!("{days_ago} days ago"))
        .arg(path)
        .status()
        .expect("touch -d");
    assert!(status.success(), "touch failed for {}", path.display());
}

fn run_with(args: &[&str]) -> (i32, String, String) {
    let output = Command::new(binary_path())
        .args(args)
        .output()
        .expect("run binary");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8(output.stdout).unwrap_or_default(),
        String::from_utf8(output.stderr).unwrap_or_default(),
    )
}

fn parse_json(stdout: &str) -> Value {
    serde_json::from_str(stdout).expect("valid JSON output")
}

#[test]
fn runs_on_empty_corpus_no_findings() {
    let dir = tempfile::tempdir().unwrap();
    let (code, stdout, _stderr) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    assert_eq!(code, 0);
    let v = parse_json(&stdout);
    assert_eq!(v["schema"], "v2-gardening-sweep/v1");
    assert_eq!(v["files_scanned"], 0);
    assert_eq!(v["stale"].as_array().unwrap().len(), 0);
    assert_eq!(v["dead_links"].as_array().unwrap().len(), 0);
}

#[test]
fn detects_stale_file_over_threshold() {
    let dir = tempfile::tempdir().unwrap();
    let old = dir.path().join("old.md");
    fs::write(&old, "# old note\n").unwrap();
    touch_old(&old, 60);
    let (code, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--stale-days",
        "30",
    ]);
    assert_eq!(code, 0);
    let v = parse_json(&stdout);
    let stale = v["stale"].as_array().unwrap();
    assert_eq!(stale.len(), 1);
    assert!(stale[0]["path"]
        .as_str()
        .unwrap()
        .ends_with("old.md"));
    let age = stale[0]["age_days"].as_u64().unwrap();
    assert!(age >= 30, "expected >= 30 days, got {age}");
}

#[test]
fn does_not_flag_recent_file_as_stale() {
    let dir = tempfile::tempdir().unwrap();
    let fresh = dir.path().join("fresh.md");
    fs::write(&fresh, "# fresh\n").unwrap();
    let (code, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--stale-days",
        "30",
    ]);
    assert_eq!(code, 0);
    let v = parse_json(&stdout);
    assert_eq!(v["stale"].as_array().unwrap().len(), 0);
    assert_eq!(v["files_scanned"], 1);
}

#[test]
fn detects_dead_markdown_link() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "see [docs](missing-target.md) for details\n",
    )
    .unwrap();
    let (code, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    assert_eq!(code, 0);
    let v = parse_json(&stdout);
    let dead = v["dead_links"].as_array().unwrap();
    assert_eq!(dead.len(), 1);
    assert_eq!(dead[0]["kind"], "markdown");
    assert_eq!(dead[0]["target_as_written"], "missing-target.md");
    assert_eq!(dead[0]["line"], 1);
}

#[test]
fn passes_alive_markdown_link() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "see [docs](b.md) for details\n",
    )
    .unwrap();
    fs::write(dir.path().join("b.md"), "# target\n").unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["dead_links"].as_array().unwrap().len(), 0);
}

#[test]
fn skips_external_links() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "see [ex](https://example.com) and [mail](mailto:a@b.c) and [ftp](ftp://x) and [file](file:///etc/passwd)\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["dead_links"].as_array().unwrap().len(), 0);
}

#[test]
fn skips_anchor_only_links() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "see [section](#section-id) for details\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["dead_links"].as_array().unwrap().len(), 0);
}

#[test]
fn strips_fragment_from_path_link() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "see [sec](b.md#somewhere)\n",
    )
    .unwrap();
    fs::write(dir.path().join("b.md"), "# target\n").unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    assert_eq!(
        v["dead_links"].as_array().unwrap().len(),
        0,
        "fragment should be stripped before resolution"
    );
}

#[test]
fn detects_dead_wiki_link() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "see [[missing-name]] for the index\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    let dead = v["dead_links"].as_array().unwrap();
    assert_eq!(dead.len(), 1);
    assert_eq!(dead[0]["kind"], "wiki");
    assert_eq!(dead[0]["target_as_written"], "missing-name");
}

#[test]
fn passes_alive_wiki_link_by_stem() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "see [[existing-name]] for the index\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("existing-name.md"),
        "# target\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["dead_links"].as_array().unwrap().len(), 0);
}

#[test]
fn skips_inline_code_spans() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "`[fake](should-not-resolve.md)` plus [real](other-real.md)\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    let dead = v["dead_links"].as_array().unwrap();
    assert_eq!(dead.len(), 1);
    assert_eq!(dead[0]["target_as_written"], "other-real.md");
}

#[test]
fn skips_fenced_code_blocks() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "before\n```\n[fake](missing-in-fence.md)\n```\nafter [real](after-real.md)\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    let dead = v["dead_links"].as_array().unwrap();
    assert_eq!(dead.len(), 1);
    assert_eq!(dead[0]["target_as_written"], "after-real.md");
}

#[test]
fn strict_mode_returns_one_with_findings() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "[broken](missing-strict.md)\n",
    )
    .unwrap();
    let (code, _, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--strict",
    ]);
    assert_eq!(code, 1, "strict mode should exit 1 when findings present");
}

#[test]
fn strict_mode_returns_zero_when_clean() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.md"), "# clean note\n").unwrap();
    let (code, _, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--strict",
    ]);
    assert_eq!(code, 0);
}

#[test]
fn multiple_corpus_roots_aggregate_findings() {
    let dir1 = tempfile::tempdir().unwrap();
    let dir2 = tempfile::tempdir().unwrap();
    fs::write(
        dir1.path().join("a.md"),
        "[bad](missing-dir1.md)\n",
    )
    .unwrap();
    fs::write(
        dir2.path().join("b.md"),
        "[bad](missing-dir2.md)\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir1.path().to_str().unwrap(),
        "--corpus",
        dir2.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["files_scanned"], 2);
    assert_eq!(v["dead_links"].as_array().unwrap().len(), 2);
    assert_eq!(v["corpus"].as_array().unwrap().len(), 2);
}

#[test]
fn text_output_format_is_human_readable() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "[bad](missing-text.md)\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--format",
        "text",
    ]);
    assert!(stdout.contains("v2-gardening-sweep"));
    assert!(stdout.contains("Dead links"));
    assert!(stdout.contains("missing-text.md"));
}

#[test]
fn no_stale_disables_stale_detection() {
    let dir = tempfile::tempdir().unwrap();
    let old = dir.path().join("old.md");
    fs::write(&old, "# old\n").unwrap();
    touch_old(&old, 60);
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--no-stale",
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["stale_detection_enabled"], false);
    assert_eq!(v["stale"].as_array().unwrap().len(), 0);
}

#[test]
fn no_dead_links_disables_dead_link_detection() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "[broken](missing.md)\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--no-dead-links",
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["dead_link_detection_enabled"], false);
    assert_eq!(v["dead_links"].as_array().unwrap().len(), 0);
}

#[test]
fn both_detectors_disabled_errors() {
    let dir = tempfile::tempdir().unwrap();
    let (code, _, stderr) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--no-stale",
        "--no-dead-links",
    ]);
    assert_ne!(code, 0);
    assert!(stderr.contains("nothing to do"));
}

#[test]
fn recursive_walk_finds_nested_md() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("nested/deeper")).unwrap();
    fs::write(dir.path().join("top.md"), "# top\n").unwrap();
    fs::write(dir.path().join("nested/mid.md"), "# mid\n").unwrap();
    fs::write(
        dir.path().join("nested/deeper/leaf.md"),
        "# leaf\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["files_scanned"], 3);
}

#[test]
fn ignores_non_markdown_extensions() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.md"), "# md\n").unwrap();
    fs::write(dir.path().join("b.txt"), "# txt\n").unwrap();
    fs::write(dir.path().join("c.json"), "{}\n").unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["files_scanned"], 1);
}

#[test]
fn custom_extension_filter() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.md"), "# md\n").unwrap();
    fs::write(dir.path().join("b.rst"), "= rst =\n").unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--extension",
        "rst",
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["files_scanned"], 1);
}

#[test]
fn output_file_writes_json_to_disk() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("a.md"), "# clean\n").unwrap();
    let out_path = dir.path().join("report.json");
    let (code, _, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--output",
        out_path.to_str().unwrap(),
    ]);
    assert_eq!(code, 0);
    let content = fs::read_to_string(&out_path).unwrap();
    let v: Value = serde_json::from_str(&content).unwrap();
    assert_eq!(v["schema"], "v2-gardening-sweep/v1");
}

#[test]
fn skips_frontmatter_links() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "---\ntitle: foo\nrelated: [bad](missing-in-frontmatter.md)\n---\n# Body\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    assert_eq!(
        v["dead_links"].as_array().unwrap().len(),
        0,
        "links inside frontmatter should be skipped"
    );
}

#[test]
fn skips_multiline_html_comments() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "before\n<!--\n[bad](missing-html.md)\n-->\nafter [real](missing-after.md)\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    let dead = v["dead_links"].as_array().unwrap();
    assert_eq!(dead.len(), 1);
    assert_eq!(dead[0]["target_as_written"], "missing-after.md");
}

#[test]
fn skips_indented_code_blocks() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "para\n    [fake](missing-indent.md)\nback [real](missing-back.md)\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    let dead = v["dead_links"].as_array().unwrap();
    assert_eq!(dead.len(), 1);
    assert_eq!(dead[0]["target_as_written"], "missing-back.md");
}

#[test]
fn resolves_reference_style_link() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "see [docs][ref1] for details\n\n[ref1]: target.md\n",
    )
    .unwrap();
    fs::write(dir.path().join("target.md"), "# target\n").unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    assert_eq!(
        v["dead_links"].as_array().unwrap().len(),
        0,
        "reference-style link should resolve to existing target"
    );
}

#[test]
fn dead_reference_style_link_when_undefined() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "see [docs][nope] for details\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    let dead = v["dead_links"].as_array().unwrap();
    assert_eq!(dead.len(), 1);
    assert_eq!(dead[0]["kind"], "markdown_ref");
    assert!(dead[0]["target_as_written"]
        .as_str()
        .unwrap()
        .contains("undefined-ref"));
}

#[test]
fn exclude_pattern_filters_files() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("archive")).unwrap();
    fs::write(
        dir.path().join("archive/old.md"),
        "[bad](missing-archive.md)\n",
    )
    .unwrap();
    fs::write(
        dir.path().join("current.md"),
        "[bad](missing-current.md)\n",
    )
    .unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--exclude",
        "archive/*.md",
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["files_scanned"], 1);
    assert_eq!(v["files_excluded"], 1);
    let dead = v["dead_links"].as_array().unwrap();
    assert_eq!(dead.len(), 1);
    assert_eq!(dead[0]["target_as_written"], "missing-current.md");
}

#[test]
fn exclude_pattern_with_double_star() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("deep/sub/archive")).unwrap();
    fs::write(
        dir.path().join("deep/sub/archive/old.md"),
        "[bad](missing.md)\n",
    )
    .unwrap();
    fs::write(dir.path().join("current.md"), "# clean\n").unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--exclude",
        "**/archive/*.md",
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["files_scanned"], 1);
    assert_eq!(v["files_excluded"], 1);
}

#[test]
fn auto_fix_suggestion_for_near_match() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "[link](sibling-typo.md)\n",
    )
    .unwrap();
    fs::write(dir.path().join("sibling-type.md"), "# target\n").unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    let dead = v["dead_links"].as_array().unwrap();
    assert_eq!(dead.len(), 1);
    let suggested = dead[0]["suggested_target"].as_str();
    assert!(suggested.is_some(), "expected a suggestion for near-match");
    assert!(suggested.unwrap().contains("sibling-type"));
}

#[test]
fn no_suggest_fixes_disables_suggestions() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "[link](sibling-typo.md)\n",
    )
    .unwrap();
    fs::write(dir.path().join("sibling-type.md"), "# target\n").unwrap();
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--no-suggest-fixes",
    ]);
    let v = parse_json(&stdout);
    let dead = v["dead_links"].as_array().unwrap();
    assert_eq!(dead.len(), 1);
    assert!(
        dead[0]["suggested_target"].is_null(),
        "suggested_target should be omitted when --no-suggest-fixes given"
    );
}

#[test]
fn state_file_first_run_no_hash_unchanged_annotation() {
    let dir = tempfile::tempdir().unwrap();
    let old = dir.path().join("old.md");
    fs::write(&old, "# old\n").unwrap();
    touch_old(&old, 60);
    let state = dir.path().join("state.json");
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--state-file",
        state.to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    let stale = v["stale"].as_array().unwrap();
    assert_eq!(stale.len(), 1);
    // First run: no prior hash, so hash_unchanged should be null/absent
    assert!(stale[0].get("hash_unchanged").is_none() || stale[0]["hash_unchanged"].is_null());
    // State file written
    assert!(state.exists());
}

#[test]
fn state_file_second_run_marks_unchanged() {
    let dir = tempfile::tempdir().unwrap();
    let old = dir.path().join("old.md");
    fs::write(&old, "# old\n").unwrap();
    touch_old(&old, 60);
    let state = dir.path().join("state.json");

    // First run: writes state.
    let (_, _, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--state-file",
        state.to_str().unwrap(),
    ]);

    // Re-touch as old (same content) so it's still stale.
    touch_old(&old, 60);

    // Second run: content unchanged from state.
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--state-file",
        state.to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    let stale = v["stale"].as_array().unwrap();
    assert_eq!(stale.len(), 1);
    assert_eq!(stale[0]["hash_unchanged"], true);
}

#[test]
fn state_file_marks_changed_when_content_differs() {
    let dir = tempfile::tempdir().unwrap();
    let old = dir.path().join("old.md");
    fs::write(&old, "# original\n").unwrap();
    touch_old(&old, 60);
    let state = dir.path().join("state.json");

    let (_, _, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--state-file",
        state.to_str().unwrap(),
    ]);

    // Rewrite with different content, keep old mtime.
    fs::write(&old, "# changed\n").unwrap();
    touch_old(&old, 60);

    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--state-file",
        state.to_str().unwrap(),
    ]);
    let v = parse_json(&stdout);
    let stale = v["stale"].as_array().unwrap();
    assert_eq!(stale.len(), 1);
    assert_eq!(stale[0]["hash_unchanged"], false);
}

#[test]
fn detector_composition_dead_links_only_on_stale() {
    let dir = tempfile::tempdir().unwrap();
    let stale = dir.path().join("stale.md");
    let fresh = dir.path().join("fresh.md");
    fs::write(&stale, "[bad](missing-stale.md)\n").unwrap();
    fs::write(&fresh, "[bad](missing-fresh.md)\n").unwrap();
    touch_old(&stale, 60);
    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--stale-days",
        "30",
        "--dead-links-only-on-stale-files",
    ]);
    let v = parse_json(&stdout);
    let dead = v["dead_links"].as_array().unwrap();
    assert_eq!(
        dead.len(),
        1,
        "only the stale file's dead link should be reported"
    );
    assert_eq!(dead[0]["target_as_written"], "missing-stale.md");
    assert_eq!(v["composition"], "dead-links-only-on-stale-files");
}

#[test]
fn config_file_provides_defaults() {
    let dir = tempfile::tempdir().unwrap();
    let corpus_dir = dir.path().join("docs");
    fs::create_dir_all(&corpus_dir).unwrap();
    let old = corpus_dir.join("old.md");
    fs::write(&old, "# old\n").unwrap();
    touch_old(&old, 60);

    let config_path = dir.path().join("config.json");
    let config_body = format!(
        r#"{{"corpus": ["{}"], "stale_days": 10}}"#,
        corpus_dir.display()
    );
    fs::write(&config_path, config_body).unwrap();

    let (code, stdout, _) = run_with(&[
        "--config",
        config_path.to_str().unwrap(),
    ]);
    assert_eq!(code, 0);
    let v = parse_json(&stdout);
    assert_eq!(v["stale_threshold_days"], 10);
    assert_eq!(v["stale"].as_array().unwrap().len(), 1);
}

#[test]
fn config_excludes_union_with_cli() {
    let dir = tempfile::tempdir().unwrap();
    fs::create_dir_all(dir.path().join("archive")).unwrap();
    fs::create_dir_all(dir.path().join("draft")).unwrap();
    fs::write(dir.path().join("archive/old.md"), "# old\n").unwrap();
    fs::write(dir.path().join("draft/wip.md"), "# wip\n").unwrap();
    fs::write(dir.path().join("current.md"), "# current\n").unwrap();

    let config_path = dir.path().join("config.json");
    let config_body = r#"{"exclude": ["archive/*.md"]}"#;
    fs::write(&config_path, config_body).unwrap();

    let (_, stdout, _) = run_with(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--config",
        config_path.to_str().unwrap(),
        "--exclude",
        "draft/*.md",
    ]);
    let v = parse_json(&stdout);
    assert_eq!(v["files_scanned"], 1, "only current.md should remain");
    assert_eq!(v["files_excluded"], 2);
}

#[test]
fn no_corpus_specified_errors() {
    let (code, _, stderr) = run_with(&["--stale-days", "30"]);
    assert_ne!(code, 0);
    assert!(stderr.contains("no corpus specified"));
}
