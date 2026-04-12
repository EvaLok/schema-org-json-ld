use chrono::Utc;
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const EXCLUDED_STEPS: &[&str] = &[
    "metric-snapshot",
    "field-inventory",
    "housekeeping-scan",
    "cycle-status",
    "state-invariants",
    "chronic-category-currency",
    "artifact-verify",
    "disposition-match",
    "audit-inbound-lifecycle",
    "agent-sessions-lifecycle",
    "deferral-accumulation",
    "deferral-deadlines",
    "mass-deferral-gate",
    "dispatch-finding-reconciliation",
    "doc-validation",
    "frozen-commit-verify",
    "review-events-verified",
    "worklog-dedup",
    "worklog-immutability",
    "frozen-worklog-immutability",
    "pr-base-currency",
    "step-comments",
    "current-cycle-steps",
];

#[test]
fn deferred_resolution_merge_gate_fails_for_open_pull_request() {
    let fixture = PipelineFixture::new("open");
    fixture.write_state("PR #700");
    fixture.write_fake_gh("OPEN");

    let output = fixture.run_pipeline_check();
    assert!(
        !output.status.success(),
        "pipeline-check unexpectedly passed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let report = parse_report(&output.stdout);
    let step = find_step(&report, "deferred-resolution-merge-gate");
    assert_eq!(step.get("status").and_then(Value::as_str), Some("fail"));
    assert!(step
        .get("detail")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .contains("open PR #700"));
}

#[test]
fn deferred_resolution_merge_gate_passes_for_merged_pull_request() {
    let fixture = PipelineFixture::new("merged");
    fixture.write_state("PR #700");
    fixture.write_fake_gh("MERGED");

    let output = fixture.run_pipeline_check();
    assert!(
        output.status.success(),
        "pipeline-check failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let report = parse_report(&output.stdout);
    let step = find_step(&report, "deferred-resolution-merge-gate");
    assert_eq!(step.get("status").and_then(Value::as_str), Some("pass"));
    assert!(step
        .get("detail")
        .and_then(Value::as_str)
        .unwrap_or_default()
        .contains("verified 1 resolved deferred-finding PR/issue ref"));
}

struct PipelineFixture {
    root: PathBuf,
    repo_root: PathBuf,
    bin_root: PathBuf,
}

impl PipelineFixture {
    fn new(suffix: &str) -> Self {
        let root = std::env::temp_dir().join(format!(
            "pipeline-check-deferred-resolution-merge-gate-{suffix}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("system time should be after epoch")
                .as_nanos()
        ));
        let repo_root = root.join("repo");
        let bin_root = root.join("bin");
        fs::create_dir_all(repo_root.join("docs")).expect("docs dir should exist");
        fs::create_dir_all(&bin_root).expect("bin dir should exist");
        Self {
            root,
            repo_root,
            bin_root,
        }
    }

    fn write_state(&self, resolved_ref: &str) {
        let today = Utc::now().format("%Y-%m-%d").to_string();
        fs::write(
            self.repo_root.join("docs/state.json"),
            json!({
                "cycle_phase": {
                    "cycle": 465,
                    "phase": "close_out",
                    "phase_entered_at": "2026-04-09T04:00:00Z"
                },
                "last_cycle": {
                    "number": 465,
                    "issue": 2325
                },
                "deferred_findings": [{
                    "category": "process-adherence",
                    "deferred_cycle": 462,
                    "deadline_cycle": 465,
                    "resolved": true,
                    "resolved_ref": resolved_ref
                }]
            })
            .to_string(),
        )
        .expect("state file should be written");
        fs::create_dir_all(self.repo_root.join("docs/journal"))
            .expect("journal dir should be created");
        fs::write(
            self.repo_root
                .join("docs/journal")
                .join(format!("{today}.md")),
            format!("# Journal\n\n## {today} — Cycle 465:\n"),
        )
        .expect("journal file should be written");
    }

    fn write_fake_gh(&self, pr_state: &str) {
        let script = format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nif [[ \"$1\" == \"pr\" && \"$2\" == \"view\" && \"$3\" == \"700\" ]]; then\n  printf '%s\\n' '{}'\nelse\n  echo \"unexpected gh args: $*\" >&2\n  exit 1\nfi\n",
            pr_state
        );
        let gh_path = self.bin_root.join("gh");
        fs::write(&gh_path, script).expect("fake gh should be written");
        make_executable(&gh_path);
    }

    fn run_pipeline_check(&self) -> std::process::Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_pipeline-check"));
        command
            .arg("--repo-root")
            .arg(&self.repo_root)
            .arg("--json")
            .env("PATH", self.path_with_fake_gh());
        for step in EXCLUDED_STEPS {
            command.arg("--exclude-step").arg(step);
        }
        command.output().expect("pipeline-check should execute")
    }

    fn path_with_fake_gh(&self) -> String {
        let current_path = std::env::var("PATH").expect("PATH should exist");
        format!("{}:{}", self.bin_root.display(), current_path)
    }
}

impl Drop for PipelineFixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn parse_report(stdout: &[u8]) -> Value {
    serde_json::from_slice(stdout).expect("pipeline-check should emit valid JSON")
}

fn find_step<'a>(report: &'a Value, name: &str) -> &'a Value {
    report
        .get("steps")
        .and_then(Value::as_array)
        .and_then(|steps| {
            steps
                .iter()
                .find(|step| step.get("name").and_then(Value::as_str) == Some(name))
        })
        .expect("expected step should be present")
}

#[cfg(unix)]
fn make_executable(path: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let mut permissions = fs::metadata(path)
        .expect("file metadata should be readable")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).expect("file should be executable");
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) {}
