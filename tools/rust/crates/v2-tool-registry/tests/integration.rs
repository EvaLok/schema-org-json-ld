use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    // Tests run from the crate's manifest directory; the workspace root is two
    // levels up (crates/<name>/.. == crates/.. == tools/rust/).
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR set");
    PathBuf::from(manifest_dir)
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

fn binary_path() -> PathBuf {
    // Cargo places the binary at <workspace>/target/<profile>/<name> when run via cargo test
    // in release mode, or at target/debug otherwise. Use env vars cargo provides.
    let exe = env!("CARGO_BIN_EXE_v2-tool-registry");
    PathBuf::from(exe)
}

fn make_repo_root_with_crates(temp: &tempfile::TempDir, crates: &[(&str, &str)]) {
    let crates_dir = temp.path().join("tools/rust/crates");
    fs::create_dir_all(&crates_dir).unwrap();
    for (name, description) in crates {
        let crate_dir = crates_dir.join(name);
        fs::create_dir_all(&crate_dir).unwrap();
        let cargo_toml = format!(
            "[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\ndescription = \"{}\"\n",
            name, description
        );
        fs::write(crate_dir.join("Cargo.toml"), cargo_toml).unwrap();
    }
}

#[test]
fn enumerates_existing_workspace_crates_in_markdown() {
    let bin = binary_path();
    let workspace = workspace_root();
    let repo_root = workspace.parent().unwrap().parent().unwrap();
    let output = Command::new(&bin)
        .arg("--repo-root")
        .arg(repo_root)
        .output()
        .expect("v2-tool-registry runs");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8_lossy(&output.stdout);
    // The workspace contains many crates; one of the most stable is cycle-runner.
    assert!(
        stdout.contains("cycle-runner"),
        "expected cycle-runner in output:\n{stdout}"
    );
    // The tool excludes itself.
    assert!(
        !stdout.contains("v2-tool-registry"),
        "self should be excluded:\n{stdout}"
    );
}

#[test]
fn json_output_is_valid_array_of_entries() {
    let temp = tempfile::tempdir().unwrap();
    make_repo_root_with_crates(
        &temp,
        &[("alpha", "first tool"), ("v2-beta", "second tool")],
    );
    let output = Command::new(binary_path())
        .arg("--repo-root")
        .arg(temp.path())
        .arg("--format")
        .arg("json")
        .output()
        .expect("runs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let parsed: serde_json::Value = serde_json::from_str(&stdout).expect("valid JSON");
    let array = parsed.as_array().expect("array output");
    assert_eq!(array.len(), 2);
    let names: Vec<&str> = array
        .iter()
        .map(|e| e["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, vec!["alpha", "v2-beta"]);
    let v2_flags: Vec<bool> = array.iter().map(|e| e["is_v2"].as_bool().unwrap()).collect();
    assert_eq!(v2_flags, vec![false, true]);
}

#[test]
fn v2_only_filter_excludes_legacy() {
    let temp = tempfile::tempdir().unwrap();
    make_repo_root_with_crates(
        &temp,
        &[("legacy-tool", "old"), ("v2-new-tool", "new")],
    );
    let output = Command::new(binary_path())
        .arg("--repo-root")
        .arg(temp.path())
        .arg("--v2-only")
        .arg("--format")
        .arg("names")
        .output()
        .expect("runs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines, vec!["v2-new-tool"]);
}

#[test]
fn filter_substring_match() {
    let temp = tempfile::tempdir().unwrap();
    make_repo_root_with_crates(
        &temp,
        &[
            ("foo-bar", "foo"),
            ("baz-qux", "baz"),
            ("foo-baz", "both"),
        ],
    );
    let output = Command::new(binary_path())
        .arg("--repo-root")
        .arg(temp.path())
        .arg("--filter")
        .arg("foo")
        .arg("--format")
        .arg("names")
        .output()
        .expect("runs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines: Vec<&str> = stdout.lines().collect();
    lines.sort();
    assert_eq!(lines, vec!["foo-bar", "foo-baz"]);
}

#[test]
fn missing_crates_dir_is_clear_error() {
    let temp = tempfile::tempdir().unwrap();
    // No tools/rust/crates created
    let output = Command::new(binary_path())
        .arg("--repo-root")
        .arg(temp.path())
        .output()
        .expect("runs");
    assert!(!output.status.success(), "should fail when dir missing");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("crates directory not found"),
        "expected helpful error, got: {stderr}"
    );
}

#[test]
fn skips_directories_without_cargo_toml() {
    let temp = tempfile::tempdir().unwrap();
    let crates_dir = temp.path().join("tools/rust/crates");
    fs::create_dir_all(&crates_dir).unwrap();
    fs::create_dir_all(crates_dir.join("not-a-crate")).unwrap();
    fs::create_dir_all(crates_dir.join("real-crate")).unwrap();
    fs::write(
        crates_dir.join("real-crate/Cargo.toml"),
        "[package]\nname = \"real-crate\"\ndescription = \"real\"\n",
    )
    .unwrap();
    let output = Command::new(binary_path())
        .arg("--repo-root")
        .arg(temp.path())
        .arg("--format")
        .arg("names")
        .output()
        .expect("runs");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();
    assert_eq!(lines, vec!["real-crate"]);
}
