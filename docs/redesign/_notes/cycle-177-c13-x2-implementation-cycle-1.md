# Cycle 177 — C13 + X2 implementation cycle 1 (PrimitiveInvoker timeout / cancellation arc) + --strict re-run

**Cycle issue:** [#2993](https://github.com/EvaLok/schema-org-json-ld/issues/2993)
**Tracks composed:** straight-pair-closure HARDENING-AT-16 (cycles 162–177, 16 consecutive). 26th consecutive two-track-composition post cycle 151 exception. 62nd consecutive HONORING of named forward priority.

## What happened

Cycle 177 advanced cycle 176+ priority #3 (C13 + X2 IMPLEMENTATION) into cycle 1 of implementation per the §8 acceptance criteria of [`v2-primitive-invoker-timeout-arc.md`](v2-primitive-invoker-timeout-arc.md) (cycle 176 design scope, 290 lines).

### Track 1 — C13 + X2 implementation (substantive)

End-to-end implementation of the timeout-aware `PrimitiveInvoker` trait + `RealInvoker` + `MockInvoker` extension + caller migration + new tests, all in a single coherent change. Land delta:

```
tools/rust/Cargo.lock                         |    1 +
tools/rust/crates/v2-cycle-runner/Cargo.toml  |    3 +
tools/rust/crates/v2-cycle-runner/src/main.rs | 1021 +++++++++++++++++++--
3 files changed, 938 insertions(+), 87 deletions(-)
```

#### Source-side changes

- **`PrimitiveInvoker` trait signature** evolved from `fn invoke(&self, bin: &Path, args: &[String]) -> io::Result<Output>` to `fn invoke(&self, bin: &Path, args: &[String], timeout: Duration) -> io::Result<InvocationResult>`. The new `InvocationResult` enum carries `Completed(Output)` (pre-cycle-177 shape) or `TimedOut { elapsed, partial_stdout, partial_stderr, escalation }`.
- **`RealInvoker`** implemented per design scope §4 Pattern A (thread + `mpsc::recv_timeout`). Spawns child with piped stdout/stderr; hand-rolled `WaitChild` drains the streams from a dedicated thread to avoid pipe-full deadlock; main thread `recv_timeout`s the channel; on timeout, calls `signal_escalate(pid, &rx)` which sends SIGTERM via `libc::kill`, waits the 2-second `SIGTERM_GRACE`, then SIGKILL if no exit. Partial output collected from a final `rx.recv()` post-signal.
- **`signal_escalate`** is `cfg(unix)` per design scope §4.4. Non-Unix fallback returns `SigkillForced` immediately (out-of-scope; the cycle-runner is Linux-only on GitHub Actions runners).
- **`SignalEscalation`** enum: `SigtermClean` (child responded to SIGTERM within grace) | `SigkillForced` (grace exhausted; SIGKILL applied). Both kebab-serialize per `Serialize` derive.
- **`InvocationResult`** enum carries `Completed(Output)` or `TimedOut { elapsed: Duration, partial_stdout: Vec<u8>, partial_stderr: Vec<u8>, escalation }`. Not Serialize (the `Output` inside `Completed` does not implement Serialize); the timeout-relevant fields surface via `StepTrace.timeout` instead.
- **`FailureClass::Timeout`** added as the 6th halt class. `as_kebab() -> "timeout"`. Set by the call site when it observes `InvocationResult::TimedOut`, NOT by `classify_failure` (which uses stderr keywords; timeouts produce no completion-class stderr).
- **`TimeoutDiagnostic`** struct: `{ budget_ms, elapsed_ms, escalation }`. Populates `StepTrace.timeout` only on timeout halts.
- **`default_step_timeout(StepKind) -> Duration`** maps StepKind to design scope §3.3 budgets: 30s for super-step-boundary calls, 60s for ReconcilerPoll, 4500s for RoleInvoke (matches cycle-runner harness wall clock).
- **`STATE_AUDIT_TIMEOUT = Duration::from_secs(60)`** constant for the cycle 162 state-audit pre-flight.
- **`SIGTERM_GRACE = Duration::from_secs(2)`** internal escalation step (design scope §4.2).

#### Caller migration

- **`invoke_state_audit_on_start`** now passes `STATE_AUDIT_TIMEOUT` and switches on `InvocationResult`. On `TimedOut`, returns a synthetic `StateAuditOutcome` with `severity: StateAuditSeverity::Timeout` (new variant), `exit_code: -1`, and `stderr` documenting budget / elapsed / escalation / partial_stderr. The cycle report's `state_audit` field captures the picture even though `StateTrace` is per-step (state-audit halts before the super-step sequence).
- **`StateAuditSeverity::Timeout`** added. `requires_halt()` extended to include `Timeout`. `halt_failure_class()` new method maps `Hard → StateBoundExceeded`, `Timeout → Timeout`, others `None`.
- **`halt_cycle_at_state_audit`** derives the `FailureClass` from `outcome.severity.halt_failure_class()` (with a defensive fallback to `StateBoundExceeded` to retain the cycle-162 mapping if a future variant breaks the caller-invariant). Distinct stderr text per class so post-hoc inspection sees a Timeout halt vs a Hard-severity halt without re-parsing the audit JSON.
- **`invoke_with_retry_once`** now takes `timeout: Duration` and returns `InvocationResult`. On `TimedOut`, returns immediately without retry (design scope §2.3 policy: no automatic retry on timeout). On `Completed`, retains the existing exit-code + stderr-keyword classification path.
- **`run_cycle`** step-invocation loop: measures `elapsed` via `Instant::now()` around `invoke_with_retry_once`, populates `trace.elapsed_ms = Some(elapsed.as_millis() as u64)`. Switches on the returned `InvocationResult`: `TimedOut` populates `trace.timeout = Some(TimeoutDiagnostic { ... })` and halts with `FailureClass::Timeout`; `Completed` retains the existing success / non-zero-exit classification path.
- **`StepTrace`** extended with `elapsed_ms: Option<u64>` (always `Some` for `executed: true` traces; `None` for dry-run) and `timeout: Option<TimeoutDiagnostic>` (only on Timeout halts). Both `#[serde(skip_serializing_if = "Option::is_none")]` so completed traces don't carry an unused `timeout` block in the cycle report.

#### Cargo dependency

`libc = "0.2"` added as a `cfg(unix)` target-specific dependency. Used exclusively for `libc::kill(pid, libc::SIGTERM)` and `libc::kill(pid, libc::SIGKILL)`. No new direct deps elsewhere. Cargo.lock regenerated (+1 line); `--locked` build clean.

#### Test-side changes

- **`MockInvoker`** rewritten: `canned: Vec<Output>` → `canned: Vec<CannedOutcome>` sum type. `queue(o)` API renamed to `queue_ok(o)` at 19 call sites (mechanical replace_all). New `queue_timeout(elapsed_ms, escalation)` method for queuing simulated timeouts. `calls` field now carries `(PathBuf, Vec<String>, Duration)` to enable timeout-assertion tests; `invocation_timeouts()` accessor surfaces the per-call timeouts.
- **14 new tests** added at the end of `mod tests`:
  - `failure_class_timeout_kebab_serialization` — kebab + serde round-trip.
  - `signal_escalation_kebab_serialization` — kebab variant round-trip.
  - `default_step_timeout_returns_designed_values_per_step_kind` — §3.3 table assertion.
  - `state_audit_timeout_constant_is_60_seconds` — `STATE_AUDIT_TIMEOUT` value.
  - `state_audit_severity_timeout_kebab_and_halt` — `Timeout` severity halts; `halt_failure_class()` mapping; `Hard` invariant preserved.
  - `mock_invoker_queue_timeout_produces_timed_out_result` — basic mock plumbing for the new outcome type.
  - `mock_invoker_records_timeout_per_call` — `invocation_timeouts()` accessor.
  - `invoke_with_retry_once_does_not_retry_on_timeout` — retry-policy assertion (no retry on Timeout).
  - `live_run_halts_with_timeout_class_when_step_times_out` — end-to-end halt path via mocked timeout at step 1.
  - `live_run_timeout_trace_carries_diagnostic_in_full_report` — full `StepTrace.timeout` block surface: budget_ms, elapsed_ms, escalation.
  - `live_run_completed_trace_carries_elapsed_ms` — `elapsed_ms` present on all live traces; `timeout` block absent on non-timeout traces.
  - `state_audit_timeout_halts_with_timeout_failure_class` — state-audit timeout path end-to-end; cycle report shows `halt_step=state-audit-on-start`, `halt_reason=timeout`, `state_audit.severity=timeout`.
  - `real_invoker_against_sleep_produces_timed_out_with_sigterm_clean` — **§8 acceptance criterion #4**: `/bin/sleep 5` with 500ms budget → `TimedOut` with `SigtermClean` escalation; partial output empty; outer wall clock < 5s (proves we actually killed the child rather than letting it complete).
  - `real_invoker_completes_normally_when_within_budget` — inverse: `/bin/true` with 5s budget → `Completed` with exit-0 status.

#### Acceptance criteria status per design scope §8

1. ✅ `cargo test -p v2-cycle-runner` passes all 77 unit tests + 5 integration tests post-mechanical-rename plus all 14 new tests in §6.2.
2. ✅ `cargo test --test integration_cycle` passes the 5 integration tests (`live_run_against_real_primitives_writes_completed_state_with_no_halt_marker`, `live_run_halts_super_step_out_of_order_when_history_diverges`, `live_run_halts_with_state_bound_exceeded_when_audit_reports_hard`, `dry_run_against_real_primitives_traces_ten_steps_without_state_mutation`, `status_and_verify_after_live_cycle_report_clean_then_dirty_after_corruption`). The §3.3 budgets are not exercised by these tests (they complete in milliseconds), but the trait change is exercised against real binaries.
3. ⏸️ Live `v2-cycle-runner run` against the prompts/v2/ stack — not exercised cycle 177. Out of unit-test scope; would require a real-role-driver session (Claude-API-billed wall-time test). Deferred to next cycle's real-role-session window if one occurs.
4. ✅ `real_invoker_against_sleep_produces_timed_out_with_sigterm_clean` confirms `/bin/sleep 5` with 500ms budget produces `TimedOut` with `SigtermClean` escalation; outer wall clock asserted < 5s (proving we killed the child rather than waiting for it). `live_run_timeout_trace_carries_diagnostic_in_full_report` confirms `halt_class=timeout` with `trace.timeout` block fully populated (`budget_ms`, `elapsed_ms`, `escalation`).
5. ✅ This document. §7 open questions resolved during implementation:
   - **Should `RealInvoker` allow stdin redirection?** No primitive needed it; not added. Trait shape leaves room without breaking.
   - **Should timeouts be operator-overridable via CLI?** No; §3.3 defaults shipped as constants per design scope.
   - **Should `elapsed_ms` be persisted in cycle-history.json?** Cycle 177 does NOT extend the persisted trace shape (still `{index, name, executed, phase}` per cycle 169 `write_runner_state`). Surface-only in the JSON cycle-report and `last-cycle.json`-equivalent paths. Schema-promotion to history is deferred per `schema-promotion-discipline.md` — no readers exist yet; promotion remains clean when a reader appears.
   - **Should the wait-thread channel use `crossbeam` or `std::sync::mpsc`?** `std::sync::mpsc::Receiver::recv_timeout` was sufficient. No new dep beyond `libc`.

#### Magnitude prediction precision (RECURRENCE-AT-3 for `comprehensive-test-suite-exceeds-design-scope-LOC-estimate`)

Design scope §6.3 estimate: ~330 LOC source+tests. Honest budget incorporating cycle 176's RECURRENCE-AT-2 pattern: 600–800 LOC.

Actual: 938 net LOC delta (1021 add / 87 del). Pattern advances to **RECURRENCE-AT-3** — three data points (cycles 170, 171, 177) all exceeding pre-implementation LOC estimates by 2-3× when comprehensive tests are included. Cycle 177 is 17–56% over the honest budget; ~285% over the raw design-scope estimate; consistent shape with cycle 170 (1447 add) and cycle 171 (2194 add), albeit for a smaller-surface change (single-crate trait evolution vs new-crate tool launch).

The single design-scope vs single PR comparison is now stable. Future arcs (structured-error-envelope, resume/recovery) should budget 2-3× design-scope-LOC-estimate when planning cycle counts.

### Track 2 — `--strict` re-run post-extension (bounded-mechanical)

Two tool invocations post-implementation, exercising `as-needed-verification-of-HARDENED-principle-on-tool-rerun` NOVEL@1 (cycle 176) → **RECURRENCE-AT-2**:

```
$ cargo run --bin v2-prompt-contract-check -- --strict
v2-prompt-contract-check: 4/4 prompts contract-aligned with v2-channel-router
  ✓ planner-prompt.xml ↔ plan-channel (writer: Planner)
  ✓ reconciler-prompt.xml ↔ inbound-channel (writer: Reconciler)
  ✓ executor-prompt.xml ↔ work-channel (writer: Executor)
  ✓ curator-prompt.xml ↔ memory-channel (writer: Curator)
Deferred in cycle-1 minimal scope: check-3 / check-4 (input-channel checks)

$ cargo run --bin v2-prompt-tag-semantic-fidelity -- check --strict
v2-prompt-tag-semantic-fidelity — 4 prompts scanned
[PASS] Tier 1: no errors
[PASS] Tier 2: no warnings
Summary: tier1_errors=0 tier1_passes=47 tier2_warnings=0 adaptation_notes=0
exit_code: 0
```

Identical green state to cycle 176 / 175 close. The HARDENED `live-prompts-already-aligned-at-extended-schema-level` principle continues to hold at the tool layer — cycle 177's runtime-layer change (v2-cycle-runner trait evolution) did not affect role prompts, tag-semantics manifest, or channel-router contracts. Track 2 produced 0 file changes, 0 standalone commits.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 source delta `tools/rust/crates/v2-cycle-runner/src/main.rs` | 938 net LOC (1021 add / 87 del) | cycle-close |
| Track 1 Cargo.toml dep addition | 3 add (libc target-specific) | cycle-close |
| Track 1 Cargo.lock regeneration | 1 add | cycle-close |
| Track 2 verification artifacts | 0 file changes | cycle-close |
| `cycle-177-*.md` _notes | ~280 lines | cycle-close |
| Total cycle 177 main-authored output | ~1218 LOC across source + tests + _notes | 1 cycle-close (planned) |

Pre-cycle test count: 63 unit + 5 integration = 68 tests. Post-cycle: 77 unit + 5 integration = 82 tests. Delta: +14 unit tests covering the timeout arc end-to-end.

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 177: Track 1 substantive-by-LOC (largest single-cycle source delta in arc 162-177 — 938 net LOC vs the prior maximum of cycle 170 PR #2979 at 1447 add but absorbed Copilot output). Track 2 bounded-mechanical-by-execution (2 tool invocations, identical exit-0 to cycle 176).

## Pattern updates this cycle

- `two-track-composition` HARDENING-AT-30 (cycles 152-177, 26 consecutive).
- `straight-pair-closure-as-default-two-track-shape` HARDENING-AT-16 (cycles 162-177, 16 consecutive).
- **`session-start-CI-check-discipline` RECURRENCE-AT-5 → RECURRENCE-AT-6** cycle 177 — session-start `gh run list --branch master --workflow "Rust CI" --limit 3` confirmed `4b2e2e4e` SUCCESS held on cycle 176 close. Six consecutive cycles of exercise (172-177). **NEW HARDEN candidate**: per convention, 5+ consecutive exercises with no missed cycle is the HARDEN threshold. Cycle 178 SOLIDIFY analysis required before formally HARDENING the pattern. Operational consequence if HARDENED: session-start CI check becomes part of the standing orientation discipline, not a forward-watched novel pattern.
- **`as-needed-verification-of-HARDENED-principle-on-tool-rerun` NOVEL@1 → RECURRENCE-AT-2** cycle 177 — second exercise (cycle 176 first; cycle 177 second after substantive runtime-layer change confirmed no regression). Forward-watch cycles 177-183 (1 of 8 consumed) for RECURRENCE-AT-3.
- **`coordinated-arc-design-scope-pairs-deferred-items` NOVEL@1 (cycle 176) → DESIGN-IMPLEMENTATION-CYCLE-COMPLETED** cycle 177 — first instance of the pattern shape (design + implementation) fully closing. Forward-watch cycles 178-183 for RECURRENCE-AT-2 (next coordinated arc design: C6+C7+C9 structured-error-envelope or C11+C12+X1 resume/recovery).
- **`comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-2 → RECURRENCE-AT-3** cycle 177 — third data point: cycle 170 (1447 add, 4.5× the design estimate), cycle 171 (2194 add, 7× estimate), cycle 177 (938 net LOC, 2.8× estimate, 1.2× honest budget). Three points support the 2-3× heuristic. Forward-watch cycles 178-183 for RECURRENCE-AT-4 stabilization toward HARDEN.
- `category-level-intent-sharpening-distinguished-from-token-level-circularity` RECURRENCE-AT-2 NOT-EXERCISED cycle 177 (3 of 6 consumed; carries; forward-watch cycles 175-180).
- `master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1 NOT-EXERCISED cycle 177 (5 of 8 consumed; master remains green; pattern continues at NOVEL@1).
- `schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1 NOT-EXERCISED cycle 177 (5 of 8 consumed). The §7 open question on `elapsed_ms` persistence was answered in line with this discipline: persistence NOT extended in cycle 177 because no reader exists yet; promotion deferred until a reader appears.
- `tool-extraction-surfaces-prior-cycle-errors` NOT-EXERCISED cycle 177; carries at RECURRENCE-AT-10.
- `live-prompts-already-aligned-at-extended-schema-level` HARDENED — operational mode exercised cycle 177 via Track 2 re-run; principle holds across runtime-layer changes (the new substantive coverage area for this principle).
- Other carry-forwards (straight-pair-with-dispatch, partial-investigation-misses-second-workflow, design-scope-internal-contradiction-resolved-by-implementation, atomic-dual-crate-PR-stronger-than-design-ordering-requirement, implementation-discovery-as-design-doc-revision-trigger) NOT-EXERCISED cycle 177; carry at prior status.

### Forward-watch decay status

- **`session-start-CI-check-discipline` RECURRENCE-AT-6**: cycle 178 SOLIDIFY analysis for potential HARDEN.
- **`as-needed-verification-of-HARDENED-principle-on-tool-rerun` RECURRENCE-AT-2**: forward-watch cycles 177-183; 1 of 8 consumed.
- **`coordinated-arc-design-scope-pairs-deferred-items` NOVEL@1**: shape-complete cycle 177; forward-watch cycles 178-183 for RECURRENCE-AT-2 (next coordinated arc).
- **`comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-3**: forward-watch cycles 178-183 for RECURRENCE-AT-4.
- **`category-level-intent-sharpening-distinguished-from-token-level-circularity` RECURRENCE-AT-2**: forward-watch cycles 175-180; 3 of 6 consumed.
- **`master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1**: forward-watch cycles 173-180; 5 of 8 consumed.
- **`schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1**: forward-watch cycles 173-180; 5 of 8 consumed.

## Forward priorities for cycle 178+

Top three:
1. **`session-start-CI-check-discipline` SOLIDIFY-or-HARDEN analysis** (NEW) — cycle 178 either lands HARDEN status (6+ consecutive exercises with no miss) or surfaces a structural reason against HARDENING. Continue session-start `gh run list --branch master --workflow "Rust CI" --limit 3` invocation.
2. **Master Rust CI green-state maintenance** (continued from cycle 176+ #1) — exercised at every session start.
3. **C13 + X2 cycle 2 of implementation** OR **next coordinated arc design-scope** — cycle 177 closed cycle 1 of timeout-arc implementation. Cycle 2 could: (a) wire `elapsed_ms` into the persisted cycle-history.json schema (only if a reader appears that needs it — currently no reader does); (b) extend timeout coverage to other primitives outside cycle-runner's scope (write-entry, v2-channel-router) per the broader §3.3 table; (c) live-cycle integration test that exercises actual budgets (would need real-role-session window). Alternative: open the next coordinated arc (C6+C7+C9 structured-error-envelope per design scope §9 pointer, or C11+C12+X1 resume/recovery).

Plus 10 carry-forward items: AGREE-DEFER queue, structured-error-envelope, resume/recovery, audit-engagement, per-axis archival, deprecate legacy method, workflow trigger upgrade, --all sweeps, reconcile improvements, --invocation-id flag. Full enumeration carries from cycle 176+.

## In-session issues and recoveries

- **`gh issue comment --body "..."` inline message with `#` headers** triggered "newline followed by # inside a quoted argument" sandbox warning. Recovered via tempfile write to `docs/redesign/_notes/.tmp-cycle-177-session-start.md` + `gh issue comment --body-file ...`. Pattern: avoid embedded `#` markdown in inline `--body` strings; use `--body-file` for any multi-line comment with markdown headers.
- **Cwd not persisted between bash calls** (similar to cycle 176). Recovered via `cargo run --manifest-path /home/runner/work/.../tools/rust/Cargo.toml --bin ...` (explicit absolute manifest path).
- **`cp` and `rm` blocked by multi-operation sandbox rule** when trying to copy tempfile into workspace and clean up afterward. Recovered via `Write` directly into the workspace-relative path (`docs/redesign/_notes/.tmp-cycle-177-session-start.md`).
- **`v2-prompt-tag-semantic-fidelity` defaults are cwd-relative**; my invocation from outside the repo root needed explicit `--prompts-dir` + `--manifest` flags. Recovered via `--help` discovery + flag specification.
- **Two clippy warnings on first run**: `io::Error::new(io::ErrorKind::Other, ...)` → suggest `io::Error::other(...)`, and `unwrap_or_else(|| const_value)` → suggest `unwrap_or(const_value)`. Both straightforward; applied via Edit. Pre-existing clippy warning in `v2-boot-phase` is **NOT** from cycle 177 (CI does not run clippy at all; latent for some time).
- All `cargo`, `git`, `gh`, Edit/Write operations otherwise clean cycle 177.

---

**Design-scope reference:** [`docs/redesign/_notes/v2-primitive-invoker-timeout-arc.md`](v2-primitive-invoker-timeout-arc.md) (cycle 176)

**Live source post-implementation:** [`tools/rust/crates/v2-cycle-runner/src/main.rs`](../../../tools/rust/crates/v2-cycle-runner/src/main.rs) (cycle 177)
