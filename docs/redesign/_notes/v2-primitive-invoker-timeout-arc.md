# v2-cycle-runner PrimitiveInvoker timeout / cancellation arc

**Status:** design-scope (cycle 176); implementation deferred to cycle 177+.

**Provenance:** Carry-forward from `cycle-155-cycle-152-critique-absorption.md` C13 (L3.1) + X2 (no per-step timeout), both verdicted AGREE-DEFER pending a coordinated arc. Forward-priority #4 cycle 156-167, #3-#5 cycle 168-174, #3 cycle 175. C13's narrow trait was an intentional cycle 150 design choice (broad unit coverage quickly); the deferred cost is migrating that surface to a timeout-aware shape without losing the unit-test leverage.

**Scope relative to prior design:** Cycle 149's `v2-cycle-runner-design-scope` named the runner's CLI surface and step sequence; cycle 150's implementation introduced `PrimitiveInvoker` as a thin testability shim. Neither addressed hangs / per-step timeouts. This document specifies the trait evolution, the new failure-class entry, the signal-escalation policy, the default-timeout table, the MockInvoker extension for timeout simulation, and the migration cost.

## 1. Problem statement

### 1.1 What exists today

`PrimitiveInvoker` at `tools/rust/crates/v2-cycle-runner/src/main.rs:769-771`:

```rust
trait PrimitiveInvoker {
    fn invoke(&self, bin: &Path, args: &[String]) -> io::Result<std::process::Output>;
}
```

`RealInvoker` (lines 773-778) wraps `Command::new(bin).args(args).output()` — which blocks indefinitely. `MockInvoker` (test-only, lines 1879-1908) returns canned `Output` values from a queue. `invoke_with_retry_once` (lines 1077-1098) retries once on `FailureClass::Transient`.

`FailureClass` (lines 608-631) has 5 variants: Transient, RoleSessionEmpty, ChannelWriteRejected, SuperStepOutOfOrder, StateBoundExceeded. None of them denote a hang or a timeout exhaustion.

### 1.2 Why this matters

A hung primitive — `v2-channel-router` blocked on a network call, `v2-role-driver` waiting for a child Claude session that never returns, `v2-reconciler-event-processor` stuck on file-system contention — stalls the runner indefinitely. The harness wrapping the runner (`tools/cycle-runner`) has its own ~75-minute wall clock, but the orchestrator-cycle outcome is "session expired" with no per-step diagnostic. A per-step timeout converts that into a clean `FailureClass::Timeout` with named step and elapsed-ms in the trace, surfaceable via `v2-cycle-runner status` and the cycle report.

C13's broader critique also names signals and streaming. Signals (kill the hung child cleanly on timeout) are in scope below. Streaming (read stdout / stderr while the child runs) is **out of scope** for this arc — deferred to a separate design when there's evidence of value (a primitive that produces partial-progress output worth observing pre-completion). Today no primitive streams.

### 1.3 What's NOT in cycle 1 of implementation

- Streaming stdout / stderr during child execution (deferred).
- Per-step timeout configurability via CLI flags (defaults table in §3.3 only).
- Operator-visible timeout-override config file (deferred — if §3.3 defaults need per-deployment tuning, add `state/v2-cycle-runner/timeouts.toml` then).
- Backoff-with-jitter for the existing Transient retry (cycle 155 closed that explicitly — immediate retry is intentional). This arc does not revisit that decision.
- Async runtime adoption (no tokio). Std `wait_timeout` via a polling loop or `std::thread::spawn` + `Child::wait_with_output` race is sufficient.

## 2. Trait evolution

### 2.1 New trait signature

```rust
trait PrimitiveInvoker {
    fn invoke(
        &self,
        bin: &Path,
        args: &[String],
        timeout: Duration,
    ) -> io::Result<InvocationResult>;
}

pub enum InvocationResult {
    /// Process exited within timeout. Equivalent to today's `Ok(Output)`.
    Completed(std::process::Output),
    /// Timeout expired; signal escalation completed; partial output (whatever
    /// was buffered at kill) attached. Caller maps this to FailureClass::Timeout.
    TimedOut {
        elapsed: Duration,
        partial_stdout: Vec<u8>,
        partial_stderr: Vec<u8>,
        escalation: SignalEscalation,
    },
}

pub enum SignalEscalation {
    /// Child responded to SIGTERM within grace window.
    SigtermClean,
    /// Child ignored SIGTERM; SIGKILL applied after grace.
    SigkillForced,
}
```

The `InvocationResult` enum is the load-bearing API change. Today the trait returns `Output` only (success or non-zero exit, both expressed via `ExitStatus`). The new surface distinguishes "process exited" (regardless of exit code) from "process killed by us due to timeout."

`io::Result` is retained for OS-level errors before the child is reaped (bin missing, fork failure, etc.); these are not timeouts.

### 2.2 What this does to callers

`invoke_with_retry_once` (lines 1077-1098) becomes:

```rust
fn invoke_with_retry_once<I: PrimitiveInvoker>(
    invoker: &I,
    bin: &Path,
    args: &[String],
    name: &'static str,
    timeout: Duration,
) -> Result<InvocationResult, RunnerError> {
    // ...
}
```

The call sites (lines 921-925 in `run_cycle` and the inline invocations during the super-step sequence) consume `InvocationResult` and produce halt-decisions accordingly:

- `Completed(Output)` → existing classification path (stderr-keyword + exit-code → FailureClass).
- `TimedOut { .. }` → halt cycle with `FailureClass::Timeout`; trace includes elapsed and escalation.

### 2.3 Retry policy interaction

`InvocationResult::TimedOut` does **not** trigger automatic retry. Rationale:

- A timeout exhaustion usually indicates the timeout was wrong or the child is genuinely stuck; retrying with the same timeout often just exhausts again, doubling latency before halt.
- Halt-on-first-timeout gives operators clean signal: the trace says "step X timed out at Tms, escalation Y" rather than "X timed out twice."
- Today's Transient retry exists because some primitives' exit-classified-stderr is genuinely flaky (cycle 155 absorption). Timeouts are a different failure shape.

A future cycle MAY introduce a per-step retry-on-timeout policy if observation surfaces a class of primitives whose timeout-then-success pattern is common. Cycle 1 of implementation has no retry for Timeout.

## 3. FailureClass + classification

### 3.1 New variant

```rust
pub enum FailureClass {
    Transient,
    RoleSessionEmpty,
    ChannelWriteRejected,
    SuperStepOutOfOrder,
    StateBoundExceeded,
    /// Per-step timeout exhausted; child was signal-escalated to termination.
    /// 6th halt class per cycle 176 §3.1.
    Timeout,
}
```

`Timeout` is its own class — not a sub-case of `Transient` — for the retry-policy reason above and because the halt diagnostic shape is genuinely different (elapsed time vs stderr keyword).

### 3.2 Where Timeout enters the report

`classify_failure(exit_code, stderr)` (line 645) is unchanged — it classifies the `Completed(Output)` path. `Timeout` is set by the call site directly when it sees `InvocationResult::TimedOut`. This means `classify_failure`'s stderr-keyword brittleness (C6 + C7, scheduled separately as the structured-error-envelope arc) is **not** touched by this design.

`StepTrace` (lines 781-799) gains:

```rust
struct StepTrace {
    // ... existing fields ...
    /// Elapsed wall time for the primitive invocation, in milliseconds.
    /// `None` for dry-run-only traces. Always present for `executed: true` traces.
    elapsed_ms: Option<u64>,
    /// Timeout class diagnostic. `None` unless this step was halted by timeout.
    timeout: Option<TimeoutDiagnostic>,
}

struct TimeoutDiagnostic {
    budget_ms: u64,
    elapsed_ms: u64,
    escalation: SignalEscalation,
}
```

`elapsed_ms` is useful regardless of timeout outcome (operators want to see how close each step came to its budget; runaway-rising elapsed values predict timeouts before they happen).

### 3.3 Default timeouts (the table)

Per-step defaults in cycle 1 of implementation:

| Step kind | Default timeout | Justification |
|---|---|---|
| `v2-channel-router *` | 30s | I/O bound (TOML read + JSON validation); should be sub-second on healthy machine; 30s catches pathological FS contention. |
| `v2-super-step-boundary *` | 30s | Same family as channel-router; pure state-machine + file I/O. |
| `v2-state-audit` | 60s | Walks repo state; scales with state size; 60s comfortable through cycle ~500. |
| `v2-reconciler-event-processor Poll` | 60s | Polls inbound channel + writes session-output if quiet/complete; one fs sweep per inbound channel + minimal compute. |
| `v2-reconciler-event-processor show-history` | 30s | Read-only over local state. |
| `v2-role-driver *` | 4500s | Wraps a Claude session via the `cycle-runner` harness; the harness itself caps at ~75min wall = 4500s; this matches. |
| `v2-state-dispatch-sync` | 60s | Reads `docs/state.json` + writes `state/v2/dispatches/active.json`. |
| `v2-state-dispatch-archive` | 60s | Same shape. |
| `write-entry` | 30s | Writes one journal section. |

Numbers are first-pass; cycle-1 of implementation ships these as constants; tuning is post-observation.

## 4. RealInvoker implementation

### 4.1 Approach (no async runtime)

`std::process::Command` does not provide a built-in wait-with-timeout. Two viable patterns:

**Pattern A: thread-and-channel.** Spawn child via `Command::spawn()`. Spawn a thread that calls `child.wait_with_output()` and sends `Result` over a channel. Main thread `recv_timeout(timeout)` on the channel. If timeout fires, signal child (§4.2) and recv again.

**Pattern B: polling loop.** Spawn child via `Command::spawn()`. Use `child.try_wait()` in a loop with `thread::sleep(poll_interval)`. When elapsed >= timeout, signal child.

**Decision:** Pattern A. Pattern B's poll interval is a tuning knob with no good default (10ms wastes CPU; 1s adds latency to per-step accounting). Pattern A's thread cost is one OS thread per invocation, which is bounded by the synchronous nature of the runner (one primitive at a time) and amortized over the multi-second-minimum primitive duration.

### 4.2 Signal escalation

When timeout fires, RealInvoker:

1. Send SIGTERM to child PID. Record `started_at_kill`.
2. Wait up to `SIGTERM_GRACE_MS = 2_000` for child exit (via the same wait-thread channel).
3. If child exited: return `TimedOut { escalation: SigtermClean, .. }` with whatever stdout/stderr was captured.
4. If grace exhausted: send SIGKILL. Wait for reap (cannot be ignored). Return `TimedOut { escalation: SigkillForced, .. }`.

`SIGTERM_GRACE_MS = 2_000` is a starting point. Child primitives in this repo are short-lived I/O loops with no cleanup-on-SIGTERM logic; 2s is more than enough for graceful exit. The grace window is internal to RealInvoker; it does not change the budget visible to the caller (the budget IS the timeout passed in; the grace is an internal escalation step).

### 4.3 Partial output capture

When the child is killed, RealInvoker still has stdout/stderr buffer handles. Read what's been written so far (via `child.stdout.take().read_to_end()` after signal). Attach to `TimedOut.partial_stdout` / `partial_stderr`. This is best-effort — if the child wrote nothing before the hang, partial output is empty.

Partial output is for diagnostic purposes (let the operator see what the primitive said before it stalled); it is **not** used for classification.

### 4.4 OS portability

Initial implementation: Unix only (libc signals via `nix` crate or raw `libc::kill`). Windows portability is out of scope (the orchestrator runs on Linux runners exclusively). Document this as a cycle-1 constraint, not a permanent decision.

## 5. MockInvoker extension

Tests need to simulate timeouts deterministically. Extend `MockInvoker`'s canned queue with a sum type:

```rust
enum CannedOutcome {
    Completed(Output),
    Timeout {
        elapsed_ms: u64,
        escalation: SignalEscalation,
        partial_stdout: Vec<u8>,
        partial_stderr: Vec<u8>,
    },
}

impl MockInvoker {
    fn queue_ok(&self, o: Output) {
        self.canned.borrow_mut().push(CannedOutcome::Completed(o));
    }
    fn queue_timeout(&self, elapsed_ms: u64, escalation: SignalEscalation) {
        self.canned.borrow_mut().push(CannedOutcome::Timeout {
            elapsed_ms,
            escalation,
            partial_stdout: Vec::new(),
            partial_stderr: Vec::new(),
        });
    }
}
```

Existing test invocations of `queue(Output)` migrate to `queue_ok(Output)` (mechanical rename, ~9 call sites per cycle 176 grep `MockInvoker::new()` occurrences in tests, all queue-via-default plus a small number with explicit queue).

## 6. Migration cost honest accounting

### 6.1 Source-side

- `PrimitiveInvoker` trait: 1 signature change.
- `RealInvoker::invoke`: ~80 lines new (spawn + thread + signal escalation).
- `invoke_with_retry_once`: ~5 line change (add timeout parameter, branch on InvocationResult).
- All 10 call sites in `run_cycle` and surrounding helpers: add timeout parameter pulled from the §3.3 defaults table (1 small lookup function).
- `StepTrace`: add `elapsed_ms`, `timeout` fields. Persist (cycle 162 already does selective persist; `write_runner_state` extends).
- `FailureClass::Timeout` variant + `as_kebab` arm: 2 lines.
- `halt_cycle`: 0 changes (already takes `FailureClass`; just one more variant).

### 6.2 Test-side

- `MockInvoker`: extend canned queue per §5. ~30 lines.
- Existing tests using `MockInvoker::new()` + `queue(Output)`: mechanical rename to `queue_ok`. Grep `tools/rust/crates/v2-cycle-runner` shows MockInvoker references at lines 1879, 1884, 1899, 1900 plus call sites; ~9 tests touch the queue API. Rename is one sed pass + verify cargo test.
- New tests:
  - `RealInvoker::invoke` with `sleep 5` and timeout=1s → expect TimedOut with escalation=SigtermClean (sleep is SIGTERM-respecting).
  - `RealInvoker::invoke` with a primitive that traps SIGTERM and loops → expect TimedOut with escalation=SigkillForced. Use a small Rust test-fixture binary.
  - `invoke_with_retry_once` with MockInvoker queuing TimedOut → expect no retry, return TimedOut unchanged.
  - `halt_cycle` with `FailureClass::Timeout` → expect cycle report status=halted, halt_class=timeout, trace.timeout populated.
  - 2-3 integration tests in `tests/integration_cycle.rs` covering one timeout-during-planner + one timeout-during-curator.
- Total: ~150 LOC new tests, ~9 mechanical renames.

### 6.3 Cycle 1 implementation LOC estimate

- Source: ~150 LOC.
- Tests: ~180 LOC.
- Total: ~330 LOC.
- Cycle 168/169 pattern (status/verify): 250-line design scope landed cycle 168, ~600 LOC implementation cycle 169. This arc is half that — single subcommand-equivalent surface change.

`comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-2 (cycle 170+171) applies — actual implementation may produce 2-3× the test-side estimate above. Budget cycle 1 of implementation for ~600-800 LOC total without raising concern.

## 7. Open questions deferred to implementation cycle

- **Should `RealInvoker` allow stdin redirection?** Today no primitive consumes stdin. If a future primitive needs it, extend trait then.
- **Should timeouts be operator-overridable via CLI?** Defer until §3.3 defaults prove inadequate in practice.
- **Should `elapsed_ms` be persisted in cycle-history.json?** Yes — operators want trend data on per-step elapsed. Schema migration handled per `schema-promotion-discipline.md` (typed field, reader co-edit if any consumers exist; currently none do, so promotion is clean).
- **Should the wait-thread channel use `crossbeam` or `std::sync::mpsc`?** `std::sync::mpsc::Receiver::recv_timeout` exists and is sufficient. No new dep.

## 8. Acceptance criteria for the implementation cycle

The implementation cycle (177+) is complete when:

1. `cargo test -p v2-cycle-runner` passes all existing tests post-mechanical-rename plus all new tests in §6.2.
2. `cargo test -p v2-cycle-runner --test integration_cycle` passes the new timeout-during-role integration tests.
3. A live `v2-cycle-runner run` against the prompts/v2/ stack completes a normal cycle within all §3.3 budgets (sanity check that defaults are not pathologically tight).
4. A targeted live-or-synthetic test confirms a primitive that exceeds its budget produces `halt_class=timeout` with `trace.timeout` populated and `partial_stdout` reflecting whatever the child wrote.
5. The cycle-close _notes documents any §7 question resolved by implementation, per `implementation-discovery-as-design-doc-revision-trigger` NOVEL@1 discipline.

## 9. What this arc closes

Closes C13 (L3.1) PrimitiveInvoker hides hangs/signals + X2 (no per-step timeout) per cycle 155 verdict pairing. Does **not** close C13's streaming critique — that is deferred per §1.2 (no streaming primitive exists yet to motivate the work).

`coordinated-arc-design-scope-pairs-deferred-items` may NOVEL@1 if this arc-style design-then-implement composition recurs (the cycle 175+ forward-priorities list has two more arcs queued — structured-error-envelope C6+C7+C9 and resume/recovery C11+C12+X1).
