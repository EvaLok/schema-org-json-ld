use serde_json::Value;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};

const CYCLE_493_CLOSE_OUT_FIXTURE: &str =
    include_str!("../../record-dispatch/tests/fixtures/cycle-493-post-cycle-complete-state.json");
const CYCLE_495_CLOSE_OUT_FIXTURE: &str =
    include_str!("../../record-dispatch/tests/fixtures/cycle-495-post-cycle-complete-state.json");

#[test]
fn dispatch_review_replays_cycle_493_close_out_flow_in_record_only_mode() {
    let repo = TempRepo::new();
    repo.init_with_state(CYCLE_493_CLOSE_OUT_FIXTURE);
    let body_path = repo.write_review_body("Cycle 493 review body");

    let before = repo.read_state();
    let original_timestamp = before["last_cycle"]["timestamp"]
        .as_str()
        .expect("fixture should include last_cycle timestamp")
        .to_string();
    let original_completed_at = before.pointer("/cycle_phase/completed_at").cloned();

    let output = Command::new(env!("CARGO_BIN_EXE_dispatch-review"))
        .args([
            "--cycle",
            "493",
            "--issue",
            "2493",
            "--body-file",
            body_path.to_str().expect("body path should be valid UTF-8"),
            "--repo-root",
            repo.path()
                .to_str()
                .expect("repo path should be valid UTF-8"),
            "--record-only",
            "2511",
        ])
        .output()
        .expect("dispatch-review should execute");
    assert!(
        output.status.success(),
        "dispatch-review failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let after = repo.read_state();
    assert_eq!(
        after.pointer("/cycle_phase/phase"),
        Some(&serde_json::json!("complete"))
    );
    assert_eq!(
        after.pointer("/last_cycle/summary"),
        Some(&serde_json::json!(
            "1 dispatch, 3 merges (PR #2505, PR #2507, PR #2509)"
        ))
    );
    assert_ne!(
        after.pointer("/last_cycle/timestamp"),
        Some(&serde_json::json!(original_timestamp))
    );
    assert_eq!(
        after.pointer("/cycle_phase/completed_at"),
        original_completed_at.as_ref()
    );
    assert_eq!(
        after.pointer("/dispatch_log_latest"),
        Some(&serde_json::json!(
            "#2511 [Cycle Review] Cycle 493 end-of-cycle review (cycle 493)"
        ))
    );

    let head_subject = git_output(repo.path(), ["log", "-1", "--pretty=%s"]);
    assert_eq!(
        head_subject.trim(),
        "state(record-dispatch): #2511 dispatched [cycle 493]"
    );
}

#[test]
fn dispatch_review_replays_cycle_495_close_out_flow_in_record_only_mode() {
    let repo = TempRepo::new();
    repo.init_with_state(CYCLE_495_CLOSE_OUT_FIXTURE);
    let body_path = repo.write_review_body("Cycle 495 review body");

    let before = repo.read_state();
    let original_timestamp = before["last_cycle"]["timestamp"]
        .as_str()
        .expect("fixture should include last_cycle timestamp")
        .to_string();
    let original_completed_at = before.pointer("/cycle_phase/completed_at").cloned();

    let output = Command::new(env!("CARGO_BIN_EXE_dispatch-review"))
        .args([
            "--cycle",
            "495",
            "--issue",
            "2495",
            "--body-file",
            body_path.to_str().expect("body path should be valid UTF-8"),
            "--repo-root",
            repo.path()
                .to_str()
                .expect("repo path should be valid UTF-8"),
            "--record-only",
            "2521",
        ])
        .output()
        .expect("dispatch-review should execute");
    assert!(
        output.status.success(),
        "dispatch-review failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let after = repo.read_state();
    assert_eq!(
        after.pointer("/cycle_phase/phase"),
        Some(&serde_json::json!("complete"))
    );
    assert_eq!(
        after.pointer("/last_cycle/summary"),
        Some(&serde_json::json!("1 dispatch, 0 merges"))
    );
    assert_ne!(
        after.pointer("/last_cycle/timestamp"),
        Some(&serde_json::json!(original_timestamp))
    );
    assert_eq!(
        after.pointer("/cycle_phase/completed_at"),
        original_completed_at.as_ref()
    );
    assert_eq!(
        after.pointer("/dispatch_log_latest"),
        Some(&serde_json::json!(
            "#2521 [Cycle Review] Cycle 495 end-of-cycle review (cycle 495)"
        ))
    );

    let head_subject = git_output(repo.path(), ["log", "-1", "--pretty=%s"]);
    assert_eq!(
        head_subject.trim(),
        "state(record-dispatch): #2521 dispatched [cycle 495]"
    );
}

struct TempRepo {
    path: PathBuf,
    remote_path: PathBuf,
}

impl TempRepo {
    fn new() -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "dispatch-review-real-flow-test-{}-{}",
            std::process::id(),
            unique
        ));
        let remote_path = std::env::temp_dir().join(format!(
            "dispatch-review-real-flow-remote-{}-{}",
            std::process::id(),
            unique
        ));
        fs::create_dir_all(path.join("docs")).expect("temp repo should be created");
        fs::create_dir_all(path.join("tools")).expect("temp repo tools dir should be created");
        Self { path, remote_path }
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn init_with_state(&self, state_json: &str) {
        fs::write(self.path().join("docs/state.json"), state_json)
            .expect("fixture state should be written");
        fs::write(
            self.path().join("tools/config.json"),
            r#"{"default_model":"gpt-5.4"}"#,
        )
        .expect("tools config should be written");
        git_success(self.path(), ["init"]);
        git_success(
            self.path(),
            ["config", "user.name", "Dispatch Review Integration Tests"],
        );
        git_success(
            self.path(),
            [
                "config",
                "user.email",
                "dispatch-review-integration-tests@example.com",
            ],
        );
        git_success(self.path(), ["add", "docs/state.json"]);
        git_success(self.path(), ["commit", "-m", "initial state"]);
        git_success(self.path(), ["init", "--bare", self.remote_path.to_str().unwrap()]);
        git_success(
            &self.remote_path,
            ["symbolic-ref", "HEAD", "refs/heads/master"],
        );
        git_success(
            self.path(),
            ["remote", "add", "origin", self.remote_path.to_str().unwrap()],
        );
        git_success(self.path(), ["push", "-u", "origin", "HEAD:master"]);
    }

    fn write_review_body(&self, content: &str) -> PathBuf {
        let path = self.path().join("review-body.md");
        fs::write(&path, content).expect("review body should be written");
        path
    }

    fn read_state(&self) -> Value {
        serde_json::from_str(
            &fs::read_to_string(self.path().join("docs/state.json"))
                .expect("state file should be readable"),
        )
        .expect("state file should parse")
    }
}

impl Drop for TempRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
        let _ = fs::remove_dir_all(&self.remote_path);
    }
}

fn git_success<I, S>(repo_root: &Path, args: I)
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let rendered_args: Vec<String> = args
        .into_iter()
        .map(|argument| argument.as_ref().to_string_lossy().into_owned())
        .collect();
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(&rendered_args)
        .output()
        .expect("git command should execute");
    assert!(
        output.status.success(),
        "git command failed (git -C {} {}): {}",
        repo_root.display(),
        rendered_args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_output<I, S>(repo_root: &Path, args: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let rendered_args: Vec<String> = args
        .into_iter()
        .map(|argument| argument.as_ref().to_string_lossy().into_owned())
        .collect();
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(&rendered_args)
        .output()
        .expect("git command should execute");
    assert!(
        output.status.success(),
        "git command failed (git -C {} {}): {}",
        repo_root.display(),
        rendered_args.join(" "),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("git output should be UTF-8")
}
