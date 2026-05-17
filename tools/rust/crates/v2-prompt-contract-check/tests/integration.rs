use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

const EXPECTED_PROMPTS: &[&str] = &[
    "planner-prompt.xml",
    "reconciler-prompt.xml",
    "executor-prompt.xml",
    "curator-prompt.xml",
];

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
        stdout.contains(&format!(
            "v2-prompt-contract-check: {0}/{0} prompts contract-aligned with v2-channel-router",
            EXPECTED_PROMPTS.len()
        )),
        "unexpected stdout:\n{}",
        stdout
    );
    assert!(stdout.contains("planner-prompt.xml ↔ plan-channel (writer: Planner)"));
    assert!(stdout.contains("reconciler-prompt.xml ↔ inbound-channel (writer: Reconciler)"));
    assert!(stdout.contains("executor-prompt.xml ↔ work-channel (writer: Executor)"));
    assert!(stdout.contains("curator-prompt.xml ↔ memory-channel (writer: Curator)"));
}

#[test]
fn end_to_end_comparison_with_synthetic_schema_v2_and_synthetic_prompts() {
    let tmp = TempDir::new().unwrap();
    let prompts_dir = tmp.path().join("prompts");
    std::fs::create_dir_all(&prompts_dir).unwrap();

    let planner_prompt = r#"
<role-prompt>
  <output-contract>
    <format>
      <required-key name="substantive-focal" type="object">
        <sub-key name="executor" type="string" />
        <sub-key name="extra" type="string" />
      </required-key>
    </format>
  </output-contract>
</role-prompt>
"#;
    std::fs::write(prompts_dir.join("planner-prompt.xml"), planner_prompt).unwrap();
    std::fs::write(
        prompts_dir.join("reconciler-prompt.xml"),
        "<role-prompt><output-contract><format><required-key name=\"x\" type=\"string\"/></format></output-contract></role-prompt>",
    )
    .unwrap();
    std::fs::write(
        prompts_dir.join("executor-prompt.xml"),
        "<role-prompt><output-contract><format><required-key name=\"x\" type=\"string\"/></format></output-contract></role-prompt>",
    )
    .unwrap();
    std::fs::write(
        prompts_dir.join("curator-prompt.xml"),
        "<role-prompt><output-contract><format><required-key name=\"x\" type=\"string\"/></format></output-contract></role-prompt>",
    )
    .unwrap();

    let schema_json = r#"{
  "schema_format_version": 2,
  "channels": [
    {
      "name": "plan-channel",
      "allowed_writer": "planner",
      "payload_schema": {
        "required": [
          {
            "name": "substantive-focal",
            "type": "string",
            "sub_keys": [
              { "name": "executor", "type": "object", "sub_keys": [] },
              { "name": "curator", "type": "string", "sub_keys": [] }
            ]
          }
        ],
        "optional": []
      }
    },
    {"name":"inbound-channel","allowed_writer":"reconciler","payload_schema":{"required":[{"name":"x","type":"string","sub_keys":[]}],"optional":[]}},
    {"name":"work-channel","allowed_writer":"executor","payload_schema":{"required":[{"name":"x","type":"string","sub_keys":[]}],"optional":[]}},
    {"name":"memory-channel","allowed_writer":"curator","payload_schema":{"required":[{"name":"x","type":"string","sub_keys":[]}],"optional":[]}}
  ]
}"#;
    let fake_router = tmp.path().join("fake-router.sh");
    let script = format!(
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1\" == \"schema\" && \"$2\" == \"--format\" && \"$3\" == \"json\" ]]; then\ncat <<'JSON'\n{schema_json}\nJSON\nelse\nexit 1\nfi\n"
    );
    std::fs::write(&fake_router, script).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&fake_router).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&fake_router, perms).unwrap();
    }

    let output = Command::new(check_bin())
        .arg("--strict")
        .arg("--format")
        .arg("json")
        .arg("--prompts-dir")
        .arg(&prompts_dir)
        .arg("--channel-router-bin")
        .arg(&fake_router)
        .current_dir(tools_rust_root())
        .output()
        .expect("run synthetic v2-prompt-contract-check");

    assert!(!output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("type_mismatch"), "{stdout}");
    assert!(stdout.contains("sub_key_missing_in_prompt"), "{stdout}");
    assert!(stdout.contains("sub_key_missing_in_router"), "{stdout}");
    assert!(stdout.contains("sub_key_type_mismatch"), "{stdout}");
}

#[test]
fn rejects_schema_format_version_1_with_exit_2() {
    let tmp = TempDir::new().unwrap();
    let prompts_dir = tmp.path().join("prompts");
    std::fs::create_dir_all(&prompts_dir).unwrap();
    for prompt in EXPECTED_PROMPTS {
        std::fs::write(
            prompts_dir.join(prompt),
            "<role-prompt><output-contract><format><required-key name=\"x\" type=\"string\"/></format></output-contract></role-prompt>",
        )
        .unwrap();
    }

    let schema_json = r#"{
  "schema_format_version": 1,
  "channels": []
}"#;
    let fake_router = tmp.path().join("fake-router-v1.sh");
    let script = format!(
        "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1\" == \"schema\" && \"$2\" == \"--format\" && \"$3\" == \"json\" ]]; then\ncat <<'JSON'\n{schema_json}\nJSON\nelse\nexit 1\nfi\n"
    );
    std::fs::write(&fake_router, script).unwrap();
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&fake_router).unwrap().permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&fake_router, perms).unwrap();
    }

    let output = Command::new(check_bin())
        .arg("--format")
        .arg("json")
        .arg("--prompts-dir")
        .arg(&prompts_dir)
        .arg("--channel-router-bin")
        .arg(&fake_router)
        .current_dir(tools_rust_root())
        .output()
        .expect("run v2-prompt-contract-check with schema v1");

    assert_eq!(output.status.code(), Some(2));
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("schema-format-version mismatch"),
        "{stderr}"
    );
}
