//! Timeout-aware subprocess invocation primitive.
//!
//! Extracted at cycle 179 from `v2-cycle-runner/src/main.rs` (where it
//! landed cycle 177 per `v2-primitive-invoker-timeout-arc.md`) so that
//! `v2-role-driver`'s live-claude-code-spawn arc (cycle 178 design,
//! cycle 179+ implementation per `v2-role-driver-live-spawn-arc.md`)
//! can re-use the same machinery instead of duplicating it.
//!
//! The trait and its `RealInvoker` impl spawn a child process with
//! piped stdout/stderr, wait on a dedicated thread with a bounded
//! timeout, and on timeout escalate SIGTERM (then SIGKILL after a
//! 2-second internal grace) to terminate the child. Returns
//! `InvocationResult::Completed(Output)` on clean exit and
//! `InvocationResult::TimedOut { .. }` with partial-output diagnostics
//! when the budget is exhausted.
//!
//! `MockInvoker` + `CannedOutcome` are pub for use in either caller
//! crate's tests — they replace `RealInvoker` in hermetic test paths
//! without needing real subprocesses.

use serde::Serialize;
use std::io;
use std::path::Path;
use std::process::Command as ProcessCommand;
use std::time::{Duration, Instant};

/// Outcome of a single `PrimitiveInvoker::invoke` call. Distinguishes
/// "process exited" (regardless of exit code; this is the historical
/// shape) from "we killed it on timeout" (cycle 177 extension).
///
/// The non-timeout `Completed(Output)` case carries the
/// `std::process::Output` the trait returned before cycle 177 and is
/// classified by caller-side exit-code + stderr logic. `TimedOut` is
/// set when `RealInvoker` exhausted the per-step budget and
/// signal-escalated the child to termination; the caller maps it to
/// its own timeout failure mode (e.g. `FailureClass::Timeout` in
/// v2-cycle-runner, `Outcome::Timeout` in v2-role-driver) directly
/// without consulting other classification — the child's stderr at
/// kill is best-effort diagnostic data, not classification input.
#[derive(Debug)]
pub enum InvocationResult {
    /// Child exited (with any status) within the budget. The carried
    /// `Output` is unchanged from the pre-cycle-177 trait return type.
    Completed(std::process::Output),
    /// Per-step budget exhausted; `RealInvoker` escalated SIGTERM →
    /// 2s grace → SIGKILL to terminate the child. Caller maps to its
    /// own timeout failure mode. `partial_stdout` / `partial_stderr`
    /// hold whatever the child had buffered at kill (best-effort —
    /// may be empty if the child wrote nothing before the stall).
    TimedOut {
        elapsed: Duration,
        partial_stdout: Vec<u8>,
        partial_stderr: Vec<u8>,
        escalation: SignalEscalation,
    },
}

/// How `RealInvoker` terminated a child on timeout. See
/// `v2-primitive-invoker-timeout-arc.md` §4.2 for the escalation
/// policy: SIGTERM is sent first, then a 2-second internal grace
/// window, then SIGKILL if the child has not yet exited.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SignalEscalation {
    /// Child exited cleanly within the 2-second SIGTERM grace window.
    SigtermClean,
    /// Child did not exit within the grace; SIGKILL was applied and
    /// the kernel reaped the process.
    SigkillForced,
}

/// Diagnostic block attached by callers (StepTrace in cycle-runner,
/// RoleRun in role-driver) when a step was halted by per-step timeout.
/// Distinguishes the budget from the observed elapsed (operators can
/// see how far past budget the kill happened — typically
/// `elapsed_ms` ≈ `budget_ms + small_overhead`, but signal-escalation
/// adds up to ~2s when SIGKILL is required).
#[derive(Debug, Clone, Serialize)]
pub struct TimeoutDiagnostic {
    pub budget_ms: u64,
    pub elapsed_ms: u64,
    pub escalation: SignalEscalation,
}

/// Spawn a child process and wait up to `timeout` for it to exit. On
/// timeout, escalate SIGTERM → SIGKILL and return `TimedOut` with
/// whatever partial output was captured.
///
/// `io::Result` wraps OS-level failures before the child is reaped
/// (bin missing, fork failure, etc.); these are NOT timeouts.
pub trait PrimitiveInvoker {
    fn invoke(
        &self,
        bin: &Path,
        args: &[String],
        timeout: Duration,
    ) -> io::Result<InvocationResult>;
}

/// Internal grace window between SIGTERM and SIGKILL. Cycle 177 picks
/// 2 seconds per design scope §4.2: child primitives in this repo are
/// short-lived I/O loops with no cleanup-on-SIGTERM logic, so 2s is
/// more than enough for graceful exit. The grace is internal to
/// `RealInvoker`; it does not change the budget visible to the caller
/// (the budget IS `timeout`; the grace is an internal escalation step).
pub const SIGTERM_GRACE: Duration = Duration::from_secs(2);

/// Production invoker: spawns `bin` with `args`, drains stdout/stderr
/// in parallel, and waits on a dedicated thread with a `recv_timeout`
/// channel. On timeout, escalates SIGTERM → SIGKILL.
pub struct RealInvoker;

impl PrimitiveInvoker for RealInvoker {
    /// Unix-only timeout-aware spawn per `v2-primitive-invoker-timeout-arc.md`
    /// §4. Pattern A (thread + mpsc): spawn child piped, hand the wait to a
    /// dedicated thread, recv-with-timeout from the main thread; on timeout
    /// send SIGTERM, wait up to `SIGTERM_GRACE`, then escalate to SIGKILL.
    fn invoke(
        &self,
        bin: &Path,
        args: &[String],
        timeout: Duration,
    ) -> io::Result<InvocationResult> {
        use std::process::Stdio;
        use std::sync::mpsc;
        use std::thread;

        let started = Instant::now();
        let mut child = ProcessCommand::new(bin)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let pid = child.id();
        // Take the stdout/stderr handles so `wait_with_output` can drain
        // them in the wait-thread. On timeout we read whatever's been
        // written so far for partial-output diagnostics.
        let stdout = child.stdout.take();
        let stderr = child.stderr.take();

        let (tx, rx) = mpsc::channel();
        let wait_thread = thread::spawn(move || {
            // Reattach the piped streams and let `wait_with_output` drain
            // them. The thread holds the only handle to `child`; sending
            // the result over the channel transfers ownership of the
            // Output (Completed path) or signals exit-after-kill (Timeout
            // path uses partial output gathered post-signal).
            let result = WaitChild { child, stdout, stderr }.wait_with_output();
            // Receiver may be gone (timeout path raced ahead); ignore send error.
            let _ = tx.send(result);
        });

        match rx.recv_timeout(timeout) {
            Ok(io_result) => {
                // Reap the thread (already terminating).
                let _ = wait_thread.join();
                let output = io_result?;
                Ok(InvocationResult::Completed(output))
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // Budget exhausted. Send SIGTERM, wait grace, then SIGKILL.
                let escalation = signal_escalate(pid, &rx);
                // Drain the channel one more time to collect partial output.
                // The wait-thread should have terminated by now (kernel
                // reaped the child); recv() blocks until it does.
                let final_io = rx.recv().ok().and_then(|r| r.ok());
                let _ = wait_thread.join();
                let elapsed = started.elapsed();
                let (partial_stdout, partial_stderr) = match final_io {
                    Some(out) => (out.stdout, out.stderr),
                    None => (Vec::new(), Vec::new()),
                };
                Ok(InvocationResult::TimedOut {
                    elapsed,
                    partial_stdout,
                    partial_stderr,
                    escalation,
                })
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                // wait-thread panicked or the child reaped before the
                // first recv could complete. Treat as IO error.
                let _ = wait_thread.join();
                Err(io::Error::other(
                    "v2-primitive-invoker: wait-thread channel disconnected before child exit",
                ))
            }
        }
    }
}

/// Bundle the `Child` with its piped stdout/stderr so the wait-thread
/// can drain them via `wait_with_output()`. Wrapped because
/// `Child::wait_with_output` takes ownership AND requires the streams
/// be attached; we have to take them off `Child` before sending the
/// pieces across threads, then call a hand-rolled drain on this end.
struct WaitChild {
    child: std::process::Child,
    stdout: Option<std::process::ChildStdout>,
    stderr: Option<std::process::ChildStderr>,
}

impl WaitChild {
    fn wait_with_output(mut self) -> io::Result<std::process::Output> {
        use std::io::Read;
        // Drain stdout/stderr in parallel threads to avoid the deadlock
        // where the child blocks on writing to a full pipe while we
        // block on wait().
        let stdout_thread = self.stdout.take().map(|mut s| {
            std::thread::spawn(move || {
                let mut buf = Vec::new();
                let _ = s.read_to_end(&mut buf);
                buf
            })
        });
        let stderr_thread = self.stderr.take().map(|mut s| {
            std::thread::spawn(move || {
                let mut buf = Vec::new();
                let _ = s.read_to_end(&mut buf);
                buf
            })
        });
        let status = self.child.wait()?;
        let stdout = stdout_thread
            .and_then(|t| t.join().ok())
            .unwrap_or_default();
        let stderr = stderr_thread
            .and_then(|t| t.join().ok())
            .unwrap_or_default();
        Ok(std::process::Output { status, stdout, stderr })
    }
}

/// Send SIGTERM to `pid`, wait up to `SIGTERM_GRACE` for the child to
/// exit (signaled via `rx`), then SIGKILL if still alive. Unix-only.
/// Returns the escalation level reached.
#[cfg(unix)]
fn signal_escalate(
    pid: u32,
    rx: &std::sync::mpsc::Receiver<io::Result<std::process::Output>>,
) -> SignalEscalation {
    // Safety: `libc::kill` is a thin FFI wrapper; passing a valid pid
    // and a defined signal constant. EPERM/ESRCH on a reaped child is
    // harmless (the wait-thread will report exit via `rx` regardless).
    unsafe {
        libc::kill(pid as libc::pid_t, libc::SIGTERM);
    }
    match rx.recv_timeout(SIGTERM_GRACE) {
        Ok(_) => SignalEscalation::SigtermClean,
        Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {
            unsafe {
                libc::kill(pid as libc::pid_t, libc::SIGKILL);
            }
            SignalEscalation::SigkillForced
        }
        Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
            // wait-thread already terminated (rare race); treat as clean.
            SignalEscalation::SigtermClean
        }
    }
}

#[cfg(not(unix))]
fn signal_escalate(
    _pid: u32,
    _rx: &std::sync::mpsc::Receiver<io::Result<std::process::Output>>,
) -> SignalEscalation {
    // Non-Unix platforms are out of scope for cycle 1 of implementation
    // per `v2-primitive-invoker-timeout-arc.md` §4.4. The cycle-runner
    // and role-driver are deployed only on Linux GitHub Actions runners.
    SignalEscalation::SigkillForced
}

// =====================================================================
// Test helpers — pub so either caller crate's tests can use them.
// =====================================================================

/// Test-only canned outcome queued on `MockInvoker`. `Completed(Output)`
/// matches the pre-cycle-177 `queue(Output)` API (renamed to `queue_ok`
/// for clarity post-extension). `Timeout` simulates the timeout path
/// without needing a real subprocess.
pub enum CannedOutcome {
    Completed(std::process::Output),
    Timeout {
        elapsed_ms: u64,
        escalation: SignalEscalation,
        partial_stdout: Vec<u8>,
        partial_stderr: Vec<u8>,
    },
}

/// Hermetic stand-in for `RealInvoker` used by tests in both
/// v2-cycle-runner and v2-role-driver. Queue canned outcomes via
/// `queue_ok` / `queue_timeout`; the mock pops front-to-back. If the
/// queue is empty, returns `Completed(ok_output())` (a clean
/// zero-exit `Output` with empty stdout/stderr).
#[derive(Default)]
pub struct MockInvoker {
    calls: std::cell::RefCell<Vec<(std::path::PathBuf, Vec<String>, Duration)>>,
    canned: std::cell::RefCell<Vec<CannedOutcome>>, // popped front-to-back
}

impl MockInvoker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Queue an `Output` (clean process exit, any status).
    pub fn queue_ok(&self, o: std::process::Output) {
        self.canned.borrow_mut().push(CannedOutcome::Completed(o));
    }

    /// Queue a timeout simulation: the mock will return
    /// `InvocationResult::TimedOut` with these fields on the next
    /// `invoke` call.
    pub fn queue_timeout(&self, elapsed_ms: u64, escalation: SignalEscalation) {
        self.canned.borrow_mut().push(CannedOutcome::Timeout {
            elapsed_ms,
            escalation,
            partial_stdout: Vec::new(),
            partial_stderr: Vec::new(),
        });
    }

    /// Queue a timeout simulation with explicit partial output bytes
    /// (for tests that need to assert on partial-output capture).
    pub fn queue_timeout_with_partial(
        &self,
        elapsed_ms: u64,
        escalation: SignalEscalation,
        partial_stdout: Vec<u8>,
        partial_stderr: Vec<u8>,
    ) {
        self.canned.borrow_mut().push(CannedOutcome::Timeout {
            elapsed_ms,
            escalation,
            partial_stdout,
            partial_stderr,
        });
    }

    pub fn calls(&self) -> Vec<(std::path::PathBuf, Vec<String>)> {
        self.calls
            .borrow()
            .iter()
            .map(|(p, a, _)| (p.clone(), a.clone()))
            .collect()
    }

    pub fn invocation_timeouts(&self) -> Vec<Duration> {
        self.calls.borrow().iter().map(|(_, _, t)| *t).collect()
    }

    pub fn invoke_count(&self) -> usize {
        self.calls.borrow().len()
    }
}

impl PrimitiveInvoker for MockInvoker {
    fn invoke(
        &self,
        bin: &Path,
        args: &[String],
        timeout: Duration,
    ) -> io::Result<InvocationResult> {
        self.calls
            .borrow_mut()
            .push((bin.to_path_buf(), args.to_vec(), timeout));
        let mut canned = self.canned.borrow_mut();
        if canned.is_empty() {
            Ok(InvocationResult::Completed(ok_output()))
        } else {
            let popped = canned.remove(0);
            Ok(match popped {
                CannedOutcome::Completed(o) => InvocationResult::Completed(o),
                CannedOutcome::Timeout {
                    elapsed_ms,
                    escalation,
                    partial_stdout,
                    partial_stderr,
                } => InvocationResult::TimedOut {
                    elapsed: Duration::from_millis(elapsed_ms),
                    partial_stdout,
                    partial_stderr,
                    escalation,
                },
            })
        }
    }
}

/// Construct a clean zero-exit `Output` with empty stdout/stderr.
/// Useful for tests that need a default `Completed` outcome.
pub fn ok_output() -> std::process::Output {
    use std::process::{ExitStatus, Output};
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }
    #[cfg(not(unix))]
    {
        // Non-unix path is not exercised by the orchestrator runtime
        // (deployed only on Linux); this branch keeps the crate
        // cross-compilable for editor/tooling.
        Output {
            status: ExitStatus::default(),
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }
}

/// Construct an `Output` carrying a specific stdout payload and a
/// zero exit status. Useful for tests that need the mock to return
/// canned bytes.
pub fn ok_output_with_stdout(stdout: impl Into<Vec<u8>>) -> std::process::Output {
    use std::process::{ExitStatus, Output};
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        Output {
            status: ExitStatus::from_raw(0),
            stdout: stdout.into(),
            stderr: Vec::new(),
        }
    }
    #[cfg(not(unix))]
    {
        Output {
            status: ExitStatus::default(),
            stdout: stdout.into(),
            stderr: Vec::new(),
        }
    }
}

/// Construct an `Output` carrying a non-zero exit code and a stderr
/// payload (stdout empty). Useful for tests that need the mock to
/// emit a failure outcome.
pub fn fail_output(code: i32, stderr: &str) -> std::process::Output {
    use std::process::{ExitStatus, Output};
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        // ExitStatus::from_raw on unix encodes (signal | (exit_code << 8)).
        let raw = (code & 0xff) << 8;
        Output {
            status: ExitStatus::from_raw(raw),
            stdout: Vec::new(),
            stderr: stderr.as_bytes().to_vec(),
        }
    }
    #[cfg(not(unix))]
    {
        // Best-effort on non-unix (the orchestrator never runs here).
        let _ = code;
        Output {
            status: ExitStatus::default(),
            stdout: Vec::new(),
            stderr: stderr.as_bytes().to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::time::Duration;

    #[test]
    fn mock_records_calls() {
        let mock = MockInvoker::new();
        let bin = PathBuf::from("/usr/bin/true");
        let args = vec!["--flag".to_string()];
        let _ = mock.invoke(&bin, &args, Duration::from_secs(1)).unwrap();
        let calls = mock.calls();
        assert_eq!(calls.len(), 1);
        assert_eq!(calls[0].0, bin);
        assert_eq!(calls[0].1, args);
    }

    #[test]
    fn mock_defaults_to_ok_when_queue_empty() {
        let mock = MockInvoker::new();
        let result = mock
            .invoke(Path::new("/bin/x"), &[], Duration::from_secs(1))
            .unwrap();
        match result {
            InvocationResult::Completed(o) => {
                assert!(o.status.success());
                assert!(o.stdout.is_empty());
            }
            other => panic!("expected Completed, got {other:?}"),
        }
    }

    #[test]
    fn mock_pops_queued_outcomes_in_order() {
        let mock = MockInvoker::new();
        mock.queue_ok(ok_output_with_stdout(b"first".to_vec()));
        mock.queue_ok(ok_output_with_stdout(b"second".to_vec()));

        let r1 = mock
            .invoke(Path::new("/bin/x"), &[], Duration::from_secs(1))
            .unwrap();
        let r2 = mock
            .invoke(Path::new("/bin/x"), &[], Duration::from_secs(1))
            .unwrap();
        match (r1, r2) {
            (InvocationResult::Completed(a), InvocationResult::Completed(b)) => {
                assert_eq!(a.stdout, b"first");
                assert_eq!(b.stdout, b"second");
            }
            other => panic!("unexpected outcome pair: {other:?}"),
        }
    }

    #[test]
    fn mock_returns_timed_out_for_queued_timeout() {
        let mock = MockInvoker::new();
        mock.queue_timeout(1234, SignalEscalation::SigkillForced);
        let result = mock
            .invoke(Path::new("/bin/x"), &[], Duration::from_secs(10))
            .unwrap();
        match result {
            InvocationResult::TimedOut {
                elapsed, escalation, ..
            } => {
                assert_eq!(elapsed, Duration::from_millis(1234));
                assert_eq!(escalation, SignalEscalation::SigkillForced);
            }
            other => panic!("expected TimedOut, got {other:?}"),
        }
    }

    #[test]
    fn mock_returns_partial_output_for_queued_timeout_with_partial() {
        let mock = MockInvoker::new();
        mock.queue_timeout_with_partial(
            500,
            SignalEscalation::SigtermClean,
            b"partial stdout".to_vec(),
            b"partial stderr".to_vec(),
        );
        let result = mock
            .invoke(Path::new("/bin/x"), &[], Duration::from_secs(10))
            .unwrap();
        match result {
            InvocationResult::TimedOut {
                partial_stdout,
                partial_stderr,
                ..
            } => {
                assert_eq!(partial_stdout, b"partial stdout");
                assert_eq!(partial_stderr, b"partial stderr");
            }
            other => panic!("expected TimedOut, got {other:?}"),
        }
    }

    #[test]
    fn ok_output_constructs_success_status() {
        let o = ok_output();
        assert!(o.status.success());
    }

    #[test]
    fn fail_output_carries_stderr_and_nonzero_exit() {
        let o = fail_output(2, "something broke");
        assert!(!o.status.success());
        assert_eq!(o.status.code(), Some(2));
        assert_eq!(o.stderr, b"something broke");
    }

    #[cfg(unix)]
    #[test]
    fn real_invoker_runs_short_command_within_budget() {
        let invoker = RealInvoker;
        let result = invoker
            .invoke(Path::new("/bin/echo"), &["hi".to_string()], Duration::from_secs(5))
            .unwrap();
        match result {
            InvocationResult::Completed(o) => {
                assert!(o.status.success());
                assert!(String::from_utf8_lossy(&o.stdout).contains("hi"));
            }
            other => panic!("expected Completed, got {other:?}"),
        }
    }

    #[cfg(unix)]
    #[test]
    fn real_invoker_times_out_on_long_sleep() {
        let invoker = RealInvoker;
        let result = invoker
            .invoke(
                Path::new("/bin/sleep"),
                &["10".to_string()],
                Duration::from_millis(200),
            )
            .unwrap();
        match result {
            InvocationResult::TimedOut { escalation, .. } => {
                // SIGTERM-clean or SIGKILL-forced both acceptable; sleep
                // handles SIGTERM by exiting.
                assert!(matches!(
                    escalation,
                    SignalEscalation::SigtermClean | SignalEscalation::SigkillForced
                ));
            }
            other => panic!("expected TimedOut, got {other:?}"),
        }
    }
}
