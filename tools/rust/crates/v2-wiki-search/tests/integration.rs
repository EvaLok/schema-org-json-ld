use serde_json::Value;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

fn binary_path() -> PathBuf {
    let mut path = std::env::current_exe().expect("current_exe");
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.join("v2-wiki-search")
}

fn setup_corpus() -> TempDir {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    fs::write(
        root.join("a.md"),
        "# scaffold-partial pattern\n\nThe scaffold-partial pattern describes a v2 design primitive.\n\nLater body content about other things.\n",
    )
    .unwrap();
    fs::write(
        root.join("b.md"),
        "# orchestration-hub measurement\n\nMeasurements of orchestration-hub shapes across cycles.\n",
    )
    .unwrap();
    fs::write(
        root.join("c.md"),
        "# unrelated\n\nNothing about the things being searched for.\n",
    )
    .unwrap();
    fs::create_dir_all(root.join("nested")).unwrap();
    fs::write(
        root.join("nested/d.md"),
        "# nested scaffold note\n\nA nested note about scaffolding.\n",
    )
    .unwrap();
    dir
}

fn run_bin(args: &[&str]) -> (i32, String, String) {
    let output = Command::new(binary_path())
        .args(args)
        .output()
        .expect("run binary");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (output.status.code().unwrap_or(-1), stdout, stderr)
}

#[test]
fn json_format_finds_scaffold_results_and_ranks_title_match_first() {
    let corpus = setup_corpus();
    let (code, stdout, stderr) = run_bin(&[
        "--corpus",
        corpus.path().to_str().unwrap(),
        "--query",
        "scaffold",
        "--format",
        "json",
    ]);
    assert_eq!(code, 0, "stderr: {stderr}");
    let v: Value = serde_json::from_str(&stdout).expect(&format!("not json: {stdout}"));
    assert_eq!(v["schema"], "v2-wiki-search/v1");
    let results = v["results"].as_array().unwrap();
    assert!(results.len() >= 2);
    let first_title = results[0]["title"].as_str().unwrap();
    assert!(
        first_title.contains("scaffold"),
        "expected scaffold-titled doc first, got: {first_title}"
    );
}

#[test]
fn json_format_includes_deferred_stages_array() {
    let corpus = setup_corpus();
    let (code, stdout, _) = run_bin(&[
        "--corpus",
        corpus.path().to_str().unwrap(),
        "--query",
        "scaffold",
    ]);
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    let deferred = v["deferred_stages"].as_array().unwrap();
    assert_eq!(deferred.len(), 4);
    let names: Vec<&str> = deferred.iter().filter_map(|d| d.as_str()).collect();
    assert!(names.iter().any(|n| n.contains("tf-idf-ranking")));
    assert!(names.iter().any(|n| n.contains("index-caching")));
    assert!(names.iter().any(|n| n.contains("frontmatter-parsing")));
}

#[test]
fn text_format_renders_results_and_deferred() {
    let corpus = setup_corpus();
    let (code, stdout, _) = run_bin(&[
        "--corpus",
        corpus.path().to_str().unwrap(),
        "--query",
        "orchestration",
        "--format",
        "text",
    ]);
    assert_eq!(code, 0);
    assert!(stdout.contains("# wiki-search: orchestration"));
    assert!(stdout.contains("## Results"));
    assert!(stdout.contains("## Deferred (scaffold-partial)"));
    assert!(stdout.contains("orchestration-hub measurement"));
}

#[test]
fn empty_query_exits_nonzero() {
    let corpus = setup_corpus();
    let (code, _, stderr) = run_bin(&[
        "--corpus",
        corpus.path().to_str().unwrap(),
        "--query",
        "   ",
    ]);
    assert_ne!(code, 0);
    assert!(stderr.contains("empty"));
}

#[test]
fn nonexistent_corpus_emits_warn_but_succeeds() {
    let (code, stdout, _) = run_bin(&[
        "--corpus",
        "/this/path/definitely/does/not/exist/abc123",
        "--query",
        "anything",
    ]);
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    let notes = v["notes"].as_array().unwrap();
    assert!(notes.iter().any(|n| n.as_str().unwrap().contains("warn:")));
    assert_eq!(v["documents_indexed"], 0);
    assert!(v["results"].as_array().unwrap().is_empty());
}

#[test]
fn strict_mode_flips_exit_when_warnings_present() {
    let (code, _, _) = run_bin(&[
        "--corpus",
        "/nonexistent/strict/path",
        "--query",
        "anything",
        "--strict",
    ]);
    assert_eq!(code, 1);
}

#[test]
fn strict_mode_returns_zero_with_clean_corpus() {
    let corpus = setup_corpus();
    let (code, _, _) = run_bin(&[
        "--corpus",
        corpus.path().to_str().unwrap(),
        "--query",
        "scaffold",
        "--strict",
    ]);
    assert_eq!(code, 0);
}

#[test]
fn top_k_caps_result_count() {
    let corpus = setup_corpus();
    let (code, stdout, _) = run_bin(&[
        "--corpus",
        corpus.path().to_str().unwrap(),
        "--query",
        "scaffold",
        "--top-k",
        "1",
    ]);
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["results"].as_array().unwrap().len(), 1);
}

#[test]
fn output_file_writes_json_to_disk() {
    let corpus = setup_corpus();
    let dir = tempfile::tempdir().unwrap();
    let out = dir.path().join("result.json");
    let (code, _, _) = run_bin(&[
        "--corpus",
        corpus.path().to_str().unwrap(),
        "--query",
        "scaffold",
        "--output",
        out.to_str().unwrap(),
    ]);
    assert_eq!(code, 0);
    let raw = fs::read_to_string(&out).unwrap();
    let v: Value = serde_json::from_str(&raw).unwrap();
    assert!(!v["results"].as_array().unwrap().is_empty());
}

#[test]
fn multiple_corpus_roots_merge() {
    let corpus_a = tempfile::tempdir().unwrap();
    let corpus_b = tempfile::tempdir().unwrap();
    fs::write(corpus_a.path().join("a.md"), "# alpha foo\n\nalpha\n").unwrap();
    fs::write(corpus_b.path().join("b.md"), "# beta foo\n\nbeta\n").unwrap();
    let (code, stdout, _) = run_bin(&[
        "--corpus",
        corpus_a.path().to_str().unwrap(),
        "--corpus",
        corpus_b.path().to_str().unwrap(),
        "--query",
        "foo",
    ]);
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["documents_indexed"], 2);
    assert_eq!(v["results"].as_array().unwrap().len(), 2);
}

#[test]
fn multi_term_query_accumulates_score() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    fs::write(
        root.join("two.md"),
        "# scaffold pattern\n\nBoth terms here.\n",
    )
    .unwrap();
    fs::write(root.join("one.md"), "# scaffold\n\nOnly one term.\n").unwrap();
    let (code, stdout, _) = run_bin(&[
        "--corpus",
        root.to_str().unwrap(),
        "--query",
        "scaffold pattern",
    ]);
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    let results = v["results"].as_array().unwrap();
    assert_eq!(results.len(), 2);
    assert!(results[0]["path"].as_str().unwrap().ends_with("two.md"));
    let matched: Vec<&str> = results[0]["matched_terms"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|t| t.as_str())
        .collect();
    assert!(matched.contains(&"scaffold"));
    assert!(matched.contains(&"pattern"));
}

#[test]
fn include_body_toggles_body_field() {
    let corpus = setup_corpus();
    let (_, with_body, _) = run_bin(&[
        "--corpus",
        corpus.path().to_str().unwrap(),
        "--query",
        "scaffold",
        "--include-body",
    ]);
    let (_, without_body, _) = run_bin(&[
        "--corpus",
        corpus.path().to_str().unwrap(),
        "--query",
        "scaffold",
    ]);
    let with: Value = serde_json::from_str(&with_body).unwrap();
    let without: Value = serde_json::from_str(&without_body).unwrap();
    assert!(with["results"][0].get("body").is_some());
    assert!(without["results"][0].get("body").is_none());
}

#[test]
fn min_score_filters_weak_results() {
    let dir = tempfile::tempdir().unwrap();
    let root = dir.path();
    fs::write(
        root.join("strong.md"),
        "# scaffold scaffold scaffold\n\nlots of scaffold scaffold mentions\n",
    )
    .unwrap();
    fs::write(
        root.join("weak.md"),
        "# Other title\n\nMentions scaffold once.\n",
    )
    .unwrap();
    let (code, stdout, _) = run_bin(&[
        "--corpus",
        root.to_str().unwrap(),
        "--query",
        "scaffold",
        "--min-score",
        "5.0",
    ]);
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    let results = v["results"].as_array().unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0]["path"].as_str().unwrap().ends_with("strong.md"));
}

#[test]
fn corpus_with_no_md_files_returns_empty_results() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(dir.path().join("ignored.txt"), "not markdown\n").unwrap();
    let (code, stdout, _) = run_bin(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--query",
        "anything",
    ]);
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["documents_indexed"], 0);
    assert!(v["results"].as_array().unwrap().is_empty());
}

#[test]
fn empty_corpus_returns_empty_results() {
    let dir = tempfile::tempdir().unwrap();
    let (code, stdout, _) = run_bin(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--query",
        "anything",
    ]);
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["documents_indexed"], 0);
    assert!(v["results"].as_array().unwrap().is_empty());
}

#[test]
fn case_insensitive_query_matches_mixed_case_content() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "# Scaffold-Partial Pattern\n\nMixedCase Body Content.\n",
    )
    .unwrap();
    let (code, stdout, _) = run_bin(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--query",
        "SCAFFOLD-PARTIAL",
    ]);
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(v["results"].as_array().unwrap().len(), 1);
}

#[test]
fn punctuation_in_query_is_tokenized_clean() {
    let dir = tempfile::tempdir().unwrap();
    fs::write(
        dir.path().join("a.md"),
        "# orchestration-hub\n\nthe hub.\n",
    )
    .unwrap();
    let (code, stdout, _) = run_bin(&[
        "--corpus",
        dir.path().to_str().unwrap(),
        "--query",
        "orchestration-hub!",
    ]);
    assert_eq!(code, 0);
    let v: Value = serde_json::from_str(&stdout).unwrap();
    let terms = v["query_terms"].as_array().unwrap();
    assert!(terms.iter().any(|t| t.as_str().unwrap() == "orchestration-hub"));
}
