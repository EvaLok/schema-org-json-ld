use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.pop(); // v2-prompt-contract-check -> crates
    p.pop(); // crates -> rust
    p.pop(); // rust -> tools
    p.pop(); // tools -> repo root
    p
}

fn tools_rust_root() -> PathBuf {
    repo_root().join("tools/rust")
}

fn check_bin() -> PathBuf {
    tools_rust_root()
        .join("target")
        .join("debug")
        .join("v2-prompt-contract-check")
}

fn channel_router_bin() -> PathBuf {
    tools_rust_root()
        .join("target")
        .join("debug")
        .join("v2-channel-router")
}

#[test]
fn strict_check_passes_against_live_prompts() {
    let build = Command::new("cargo")
        .current_dir(tools_rust_root())
        .args([
            "build",
            "-p",
            "v2-channel-router",
            "-p",
            "v2-prompt-contract-check",
        ])
        .output()
        .expect("build v2 binaries");
    assert!(
        build.status.success(),
        "cargo build failed\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let prompts_dir = repo_root().join("prompts/v2");
    let output = Command::new(check_bin())
        .arg("--strict")
        .arg("--format")
        .arg("text")
        .arg("--prompts-dir")
        .arg(&prompts_dir)
        .arg("--channel-router-bin")
        .arg(channel_router_bin())
        .current_dir(tools_rust_root())
        .output()
        .expect("run v2-prompt-contract-check");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    assert!(
        output.status.success(),
        "strict contract check failed\nstdout:\n{}\nstderr:\n{}",
        stdout,
        stderr
    );
    assert!(
        stdout.contains("v2-prompt-contract-check: 4/4 prompts contract-aligned with v2-channel-router"),
        "unexpected stdout:\n{}",
        stdout
    );
    assert!(stdout.contains("planner-prompt.xml ↔ plan-channel (writer: Planner)"));
    assert!(stdout.contains("reconciler-prompt.xml ↔ inbound-channel (writer: Reconciler)"));
    assert!(stdout.contains("executor-prompt.xml ↔ work-channel (writer: Executor)"));
    assert!(stdout.contains("curator-prompt.xml ↔ memory-channel (writer: Curator)"));
}
