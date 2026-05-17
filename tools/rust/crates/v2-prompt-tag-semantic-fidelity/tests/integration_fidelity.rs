// Integration tests for v2-prompt-tag-semantic-fidelity.
//
// Tests the compiled binary via process::Command to exercise the full
// CLI argument-parsing + exit-code contract defined in design §8.2.
//
// Covers:
//   1. Clean fixture with valid manifest + 4 minimal prompts → exit 0 under --strict.
//   2. Unknown tag (no manifest entry, no adaptation note) → exit 1 under --strict.
//   3. Missing required tag → exit 1 under --strict.
//   4. Unknown tag WITH adaptation note → suppressed, exit 0.
//   5. JSON output shape is correct.

use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

fn binary_path() -> std::path::PathBuf {
    let mut p = std::env::current_exe().unwrap();
    p.pop(); // strip test binary name
    if p.ends_with("deps") {
        p.pop();
    }
    p.push("v2-prompt-tag-semantic-fidelity");
    p
}

/// Write a minimal role prompt XML file.
fn write_prompt(dir: &Path, role: &str, extra_tags: &str) {
    let filename = format!("{role}-prompt.xml");
    let content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<role-prompt version="v2-test" role="{role}">
<role-identity>
  <one-line>The {role} role</one-line>
  <position-in-super-step-sequence>Step 1</position-in-super-step-sequence>
  <one-cycle-scope>One cycle</one-cycle-scope>
</role-identity>
<inputs>
  <input>input-channel</input>
  <forbidden-inputs>none</forbidden-inputs>
</inputs>
<output-contract>
  <where>session output file</where>
  <format>structured json</format>
  <example>see below</example>
  <validation>must be valid json</validation>
</output-contract>
<tools>
  <tool>bash</tool>
  <forbidden-tools>none</forbidden-tools>
</tools>
<communication>
  <session-output-file>session.json</session-output-file>
</communication>
<constraints>
  <constraint>must not modify prompts</constraint>
</constraints>
<session-structure>
  <entry>read inputs</entry>
  <work>do work</work>
  <exit>write output</exit>
</session-structure>
<meta>
  <small-prompt-discipline>keep it minimal</small-prompt-discipline>
</meta>
{extra_tags}
</role-prompt>"#
    );
    fs::write(dir.join(&filename), content).unwrap();
}

/// Write a minimal manifest TOML covering the 8 required tags.
fn write_minimal_manifest(dir: &Path) {
    let content = r#"
[tags.role-identity]
intent = "Names the role and its scope"
required-in-roles = ["planner", "executor", "curator", "reconciler"]
expected-children = ["one-line", "position-in-super-step-sequence", "one-cycle-scope"]

[tags.inputs]
intent = "Enumerates the input channels the role reads"
required-in-roles = ["planner", "executor", "curator", "reconciler"]
expected-children = ["forbidden-inputs"]

[tags.output-contract]
intent = "Specifies the output payload contract"
required-in-roles = ["planner", "executor", "curator", "reconciler"]
expected-children = ["where", "format", "example", "validation"]

[tags.tools]
intent = "Lists the tools the role may use"
required-in-roles = ["planner", "executor", "curator", "reconciler"]
expected-children = ["forbidden-tools"]

[tags.communication]
intent = "Declares where output is written and issue-comment policy"
required-in-roles = ["planner", "executor", "curator", "reconciler"]
expected-children = ["session-output-file"]

[tags.constraints]
intent = "Hard rules the role must obey"
required-in-roles = ["planner", "executor", "curator", "reconciler"]

[tags.session-structure]
intent = "The procedural sequence the role follows"
required-in-roles = ["planner", "executor", "curator", "reconciler"]
expected-children = ["entry", "work", "exit"]

[tags.meta]
intent = "Prompt maintenance metadata"
required-in-roles = ["planner", "executor", "curator", "reconciler"]
expected-children = ["small-prompt-discipline"]
"#;
    fs::write(dir.join("tag-semantics.toml"), content).unwrap();
}

fn setup_clean_fixture() -> TempDir {
    let dir = TempDir::new().unwrap();
    let prompts_dir = dir.path().join("prompts/v2");
    fs::create_dir_all(&prompts_dir).unwrap();

    for role in &["planner", "executor", "curator", "reconciler"] {
        write_prompt(&prompts_dir, role, "");
    }
    write_minimal_manifest(&prompts_dir);
    dir
}

// --------------------------------------------------------------------------
// Test 1: clean fixture → exit 0 under --strict
// --------------------------------------------------------------------------

#[test]
fn clean_fixture_strict_exits_zero() {
    let dir = setup_clean_fixture();
    let prompts_dir = dir.path().join("prompts/v2");

    let status = Command::new(binary_path())
        .args([
            "check",
            "--prompts-dir",
            prompts_dir.to_str().unwrap(),
            "--manifest",
            prompts_dir.join("tag-semantics.toml").to_str().unwrap(),
            "--strict",
        ])
        .status()
        .expect("failed to run binary");

    assert!(status.success(), "expected exit 0 on clean fixture, got {status:?}");
}

// --------------------------------------------------------------------------
// Test 2: unknown tag without adaptation note → exit 1 under --strict
// --------------------------------------------------------------------------

#[test]
fn unknown_tag_strict_exits_one() {
    let dir = setup_clean_fixture();
    let prompts_dir = dir.path().join("prompts/v2");

    // Inject an unknown tag into planner prompt (no adaptation note).
    let planner_path = prompts_dir.join("planner-prompt.xml");
    let original = fs::read_to_string(&planner_path).unwrap();
    let mutated = original.replace(
        "</role-prompt>",
        "<completely-unknown-tag>content</completely-unknown-tag>\n</role-prompt>",
    );
    fs::write(&planner_path, mutated).unwrap();

    let status = Command::new(binary_path())
        .args([
            "check",
            "--prompts-dir",
            prompts_dir.to_str().unwrap(),
            "--manifest",
            prompts_dir.join("tag-semantics.toml").to_str().unwrap(),
            "--strict",
        ])
        .status()
        .expect("failed to run binary");

    assert_eq!(
        status.code(),
        Some(1),
        "expected exit 1 for unknown tag, got {status:?}"
    );
}

// --------------------------------------------------------------------------
// Test 3: missing required tag → exit 1 under --strict
// --------------------------------------------------------------------------

#[test]
fn missing_required_tag_strict_exits_one() {
    let dir = setup_clean_fixture();
    let prompts_dir = dir.path().join("prompts/v2");

    // Remove the <constraints> block from planner prompt.
    let planner_path = prompts_dir.join("planner-prompt.xml");
    let original = fs::read_to_string(&planner_path).unwrap();
    // Remove the constraints section.
    let start = original.find("<constraints>").unwrap();
    let end = original.find("</constraints>").unwrap() + "</constraints>".len();
    let mutated = format!("{}{}", &original[..start], &original[end..]);
    fs::write(&planner_path, mutated).unwrap();

    let status = Command::new(binary_path())
        .args([
            "check",
            "--prompts-dir",
            prompts_dir.to_str().unwrap(),
            "--manifest",
            prompts_dir.join("tag-semantics.toml").to_str().unwrap(),
            "--strict",
        ])
        .status()
        .expect("failed to run binary");

    assert_eq!(
        status.code(),
        Some(1),
        "expected exit 1 for missing required tag, got {status:?}"
    );
}

// --------------------------------------------------------------------------
// Test 4: unknown tag WITH adaptation note → suppressed, exit 0
// --------------------------------------------------------------------------

#[test]
fn unknown_tag_with_adaptation_note_exits_zero() {
    let dir = setup_clean_fixture();
    let prompts_dir = dir.path().join("prompts/v2");

    // Inject unknown tag WITH semantic-adaptation-note into planner prompt.
    let planner_path = prompts_dir.join("planner-prompt.xml");
    let original = fs::read_to_string(&planner_path).unwrap();
    let mutated = original.replace(
        "</role-prompt>",
        "<custom-adapted-tag>\n  <semantic-adaptation-note>Adapts the planner for special context.</semantic-adaptation-note>\n  <content>body</content>\n</custom-adapted-tag>\n</role-prompt>",
    );
    fs::write(&planner_path, mutated).unwrap();

    let status = Command::new(binary_path())
        .args([
            "check",
            "--prompts-dir",
            prompts_dir.to_str().unwrap(),
            "--manifest",
            prompts_dir.join("tag-semantics.toml").to_str().unwrap(),
            "--strict",
        ])
        .status()
        .expect("failed to run binary");

    assert!(
        status.success(),
        "expected exit 0 when unknown tag has adaptation note, got {status:?}"
    );
}

// --------------------------------------------------------------------------
// Test 5: JSON output has correct schema_version and shape
// --------------------------------------------------------------------------

#[test]
fn json_output_has_correct_shape() {
    let dir = setup_clean_fixture();
    let prompts_dir = dir.path().join("prompts/v2");

    let output = Command::new(binary_path())
        .args([
            "check",
            "--prompts-dir",
            prompts_dir.to_str().unwrap(),
            "--manifest",
            prompts_dir.join("tag-semantics.toml").to_str().unwrap(),
            "--format",
            "json",
        ])
        .output()
        .expect("failed to run binary");

    assert!(output.status.success(), "expected exit 0");

    let stdout = String::from_utf8(output.stdout).unwrap();
    let report: serde_json::Value = serde_json::from_str(&stdout)
        .expect("stdout should be valid JSON");

    assert_eq!(report["schema_version"], "v1");
    assert!(report["prompts_scanned"].is_array());
    assert!(report["tier1_findings"].is_array());
    assert!(report["tier2_findings"].is_array());
    assert!(report["summary"].is_object());
    assert_eq!(report["exit_code"], 0);
    assert!(report["summary"]["tier1_errors"].is_number());
    assert!(report["summary"]["tier2_warnings"].is_number());
    assert!(report["summary"]["adaptation_notes_seen"].is_number());
}

// --------------------------------------------------------------------------
// Test 6: missing expected child → exit 1 under --strict
// --------------------------------------------------------------------------

#[test]
fn missing_expected_child_strict_exits_one() {
    let dir = setup_clean_fixture();
    let prompts_dir = dir.path().join("prompts/v2");

    // Remove the <forbidden-inputs> element from planner prompt.
    let planner_path = prompts_dir.join("planner-prompt.xml");
    let original = fs::read_to_string(&planner_path).unwrap();
    let mutated = original.replace("  <forbidden-inputs>none</forbidden-inputs>\n", "");
    fs::write(&planner_path, mutated).unwrap();

    let status = Command::new(binary_path())
        .args([
            "check",
            "--prompts-dir",
            prompts_dir.to_str().unwrap(),
            "--manifest",
            prompts_dir.join("tag-semantics.toml").to_str().unwrap(),
            "--strict",
        ])
        .status()
        .expect("failed to run binary");

    assert_eq!(
        status.code(),
        Some(1),
        "expected exit 1 for missing expected child, got {status:?}"
    );
}

// --------------------------------------------------------------------------
// Test 7: schema subcommand exits 0 and prints TOML template
// --------------------------------------------------------------------------

#[test]
fn schema_subcommand_exits_zero() {
    let output = Command::new(binary_path())
        .args(["schema"])
        .output()
        .expect("failed to run binary");

    assert!(output.status.success(), "expected exit 0");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(
        stdout.contains("[tags.example-tag]"),
        "should print TOML template"
    );
}

// --------------------------------------------------------------------------
// Test 8: list-tags subcommand
// --------------------------------------------------------------------------

#[test]
fn list_tags_subcommand_exits_zero() {
    let dir = setup_clean_fixture();
    let prompts_dir = dir.path().join("prompts/v2");

    let output = Command::new(binary_path())
        .args(["list-tags", "--prompts-dir", prompts_dir.to_str().unwrap()])
        .output()
        .expect("failed to run binary");

    assert!(output.status.success(), "expected exit 0");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("role-identity"), "should list role-identity tag");
    assert!(stdout.contains("constraints"), "should list constraints tag");
}
