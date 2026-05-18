// Integration test: real prompt files + real manifest.
//
// Runs the compiled binary against the actual prompts/v2/ directory and
// the actual prompts/v2/tag-semantics.toml manifest at HEAD.
//
// INVARIANT (design §8.3): must exit 0 under --strict.
// This is the load-bearing assertion that the initial manifest baseline
// aligns with the current state of prompts/v2/*-prompt.xml.
//
// If this test fails, either:
//   a) The manifest has drifted from the prompts, or
//   b) A prompt was modified in a way that violates the manifest.
// Fix by updating the manifest or adding <semantic-adaptation-note> blocks.

use std::path::PathBuf;
use std::process::Command;

fn binary_path() -> PathBuf {
    let mut p = std::env::current_exe().unwrap();
    p.pop(); // strip test binary name
    if p.ends_with("deps") {
        p.pop();
    }
    p.push("v2-prompt-tag-semantic-fidelity");
    p
}

/// Locate the repository root by walking up from the test binary location.
/// Falls back to CARGO_MANIFEST_DIR (set during test execution).
fn repo_root() -> PathBuf {
    // CARGO_MANIFEST_DIR points to crates/v2-prompt-tag-semantic-fidelity/
    // which is 3 levels below the workspace root (tools/rust/).
    // From workspace root, go up one more to get repo root.
    if let Ok(manifest_dir) = std::env::var("CARGO_MANIFEST_DIR") {
        let crate_dir = PathBuf::from(manifest_dir);
        // crate_dir = tools/rust/crates/v2-prompt-tag-semantic-fidelity
        // go up 3 levels: crates/, rust/, tools/ → repo root
        if let Some(root) = crate_dir.ancestors().nth(3) {
            let candidate = root.to_path_buf();
            if candidate.join("prompts/v2").exists() {
                return candidate;
            }
        }
    }
    // Fallback: walk up from current dir.
    let mut dir = std::env::current_dir().unwrap_or_default();
    for _ in 0..10 {
        if dir.join("prompts/v2").exists() {
            return dir;
        }
        if !dir.pop() {
            break;
        }
    }
    panic!("could not locate repo root containing prompts/v2/");
}

// --------------------------------------------------------------------------
// The load-bearing test: real prompts + real manifest, --strict → exit 0
// --------------------------------------------------------------------------

#[test]
fn real_prompts_strict_exits_zero() {
    let root = repo_root();
    let prompts_dir = root.join("prompts/v2");
    let manifest = prompts_dir.join("tag-semantics.toml");

    assert!(
        prompts_dir.exists(),
        "prompts/v2/ directory not found at {}",
        prompts_dir.display()
    );
    assert!(
        manifest.exists(),
        "tag-semantics.toml not found at {}",
        manifest.display()
    );

    let output = Command::new(binary_path())
        .args([
            "check",
            "--prompts-dir",
            prompts_dir.to_str().unwrap(),
            "--manifest",
            manifest.to_str().unwrap(),
            "--strict",
            "--format",
            "json",
        ])
        .output()
        .expect("failed to run v2-prompt-tag-semantic-fidelity binary");

    // Print findings for diagnosis if the test fails.
    if !output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("=== stdout ===\n{stdout}");
        eprintln!("=== stderr ===\n{stderr}");

        // Try to parse and print tier1 findings specifically.
        if let Ok(report) = serde_json::from_str::<serde_json::Value>(&stdout) {
            if let Some(findings) = report["tier1_findings"].as_array() {
                eprintln!("\nTier 1 findings ({}):", findings.len());
                for f in findings {
                    eprintln!("  {:?}", f);
                }
            }
        }
    }

    assert!(
        output.status.success(),
        "real-prompt regression test failed: tool exited with code {:?}. \
         This means the initial manifest baseline does not align with the current \
         prompts/v2/ state. See stdout/stderr above for details.",
        output.status.code()
    );
}

// --------------------------------------------------------------------------
// Informational: tier 2 warnings on real prompts (advisory, always exit 0)
// --------------------------------------------------------------------------

#[test]
fn real_prompts_tier2_always_exit_zero() {
    let root = repo_root();
    let prompts_dir = root.join("prompts/v2");
    let manifest = prompts_dir.join("tag-semantics.toml");

    if !prompts_dir.exists() || !manifest.exists() {
        // Skip if not found (e.g., in isolated build environment).
        return;
    }

    let status = Command::new(binary_path())
        .args([
            "check",
            "--prompts-dir",
            prompts_dir.to_str().unwrap(),
            "--manifest",
            manifest.to_str().unwrap(),
            // No --strict: tier2 warnings alone must not cause non-zero exit.
        ])
        .status()
        .expect("failed to run binary");

    assert!(
        status.success(),
        "non-strict mode should always exit 0; got {status:?}"
    );
}

// --------------------------------------------------------------------------
// Informational: list-tags against real prompts
// --------------------------------------------------------------------------

#[test]
fn real_prompts_list_tags_exits_zero() {
    let root = repo_root();
    let prompts_dir = root.join("prompts/v2");

    if !prompts_dir.exists() {
        return;
    }

    let output = Command::new(binary_path())
        .args(["list-tags", "--prompts-dir", prompts_dir.to_str().unwrap()])
        .output()
        .expect("failed to run binary");

    assert!(output.status.success(), "list-tags should exit 0");
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Sanity: known tags should appear.
    assert!(stdout.contains("role-identity"));
    assert!(stdout.contains("session-structure"));
}
