use serde_json::Value;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(prefix: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "schema-org-json-ld-{prefix}-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn binary_path(name: &str) -> String {
    std::env::var(format!("CARGO_BIN_EXE_{name}"))
        .or_else(|_| std::env::var(format!("CARGO_BIN_EXE_{}", name.replace('-', "_"))))
        .unwrap_or_else(|_| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../target/debug")
                .join(name)
                .display()
                .to_string()
        })
}

fn write_file(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, content).unwrap();
}

fn install_fake_gh(bin_dir: &Path) {
    let path = bin_dir.join("gh");
    write_file(
        &path,
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf '[]'\n",
    );
    let mut permissions = fs::metadata(&path).unwrap().permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions).unwrap();
}

#[test]
fn auto_derives_cycle_from_state_json() {
    let repo_root = TempDir::new("cycle-status-auto-cycle");
    let bin_dir = repo_root.path.join("bin");
    fs::create_dir_all(&bin_dir).unwrap();
    install_fake_gh(&bin_dir);
    write_file(
        &repo_root.path.join("docs/state.json"),
        r#"{
  "last_cycle": {
    "number": 41,
    "timestamp": "2026-03-01T00:00:00Z"
  },
  "cycle_phase": {
    "cycle": 42,
    "phase": "work",
    "phase_entered_at": "2026-03-01T00:00:00Z"
  }
}
"#,
    );

    let existing_path = std::env::var("PATH").unwrap();
    let output = Command::new(binary_path("cycle-status"))
        .args(["--repo-root", repo_root.path.to_str().unwrap(), "--json"])
        .env("PATH", format!("{}:{}", bin_dir.display(), existing_path))
        .output()
        .unwrap();

    assert!(
        output.status.success(),
        "cycle-status failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report.pointer("/cycle").and_then(Value::as_u64), Some(42));
    assert_eq!(
        report
            .pointer("/last_cycle_timestamp")
            .and_then(Value::as_str),
        Some("2026-03-01T00:00:00Z")
    );
}
