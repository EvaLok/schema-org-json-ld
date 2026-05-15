# [redesign-critique] cycle 152 v2-cycle-runner adversarial critique

Artifact critiqued: `tools/rust/crates/v2-cycle-runner/` at HEAD (`335f1199` context)
Mode: feedback-only, negative findings only.

## Lens 1 — Control-flow model and super-step sequencing

### C1 (L1.1) Sequence is hard-coded policy, not a schedulable plan
- **Issue:** `super_step_sequence()` bakes a fixed 10-element array into code, so orchestration policy is compile-time static rather than data-driven runtime policy.
- **Cites:** `tools/rust/crates/v2-cycle-runner/src/main.rs:471-484` (`super_step_sequence`).
- **Counterfactual:** Externalize sequence + constraints as a schema/state artifact (or primitive-provided plan) with validation.
- **Counterargument:** Hard-coding prevents accidental drift and keeps early-phase scope bounded.

### C2 (L1.2) Step granularity is inconsistent and obscures phase boundaries
- **Issue:** The model mixes workflow phases (`reconciler-pre-poll`) and transition mechanics (`super-step-advance-*`) in one linear list, conflating semantic work with control plumbing.
- **Cites:** `src/main.rs:473-482`, `StepKind` at `454-462`.
- **Counterfactual:** Model phase-level states (reconcile/plan/execute/curate) separately from transition actions; emit both in trace.
- **Counterargument:** Single list is simple and easy to test.

### C3 (L1.3) Parallelizable work is structurally impossible
- **Issue:** Reconciler poll sources (eva/audit/dispatch) are all fed through one monolithic poll step; no room for independent fetch deadlines/retries.
- **Cites:** `build_step_invocation` poll arg assembly `706-726`, `RunArgs` source file fields `445-447`.
- **Counterfactual:** Split polling into per-source substeps or concurrent primitive calls with per-source error handling.
- **Counterargument:** Serial poll keeps deterministic ordering and simpler failure reasoning.

### C4 (L1.4) Ordering invariants are asserted in tests but not encoded as contracts
- **Issue:** Tests enforce exact step names/order, but production has no machine-checkable contract layer (only code + tests).
- **Cites:** tests `super_step_sequence_has_ten_steps_in_exact_order` `1090-1110`; `run_schema` emits prose/json snapshots `343-405`.
- **Counterfactual:** Add explicit transition table and validate execution against table at runtime.
- **Counterargument:** Compile-time array plus tests may be enough during scaffold phase.

### C5 (L1.5) Dry-run and live-run share trace shape but represent different semantics
- **Issue:** Dry-run traces include step args/bin paths like execution traces, but no marker for "not actually invoked" per step; downstream may over-trust trace equivalence.
- **Cites:** `run_cycle` dry-run branch `624-635`, `CycleReport` trace model `538-560`.
- **Counterfactual:** Add per-step `executed: bool`/`result` fields.
- **Counterargument:** Top-level `dry_run` flag exists.

## Lens 2 — Failure taxonomy, retry policy, and halt durability

### C6 (L2.1) Failure taxonomy is too coarse for meaningful remediation
- **Issue:** Everything not matched by a few substrings becomes `Transient`; this collapses permanent config/user errors into retryable class.
- **Cites:** `FailureClass` `486-497`, `classify_failure` default `523`.
- **Counterfactual:** Add explicit classes (auth/config/io/timeout/protocol) and map via structured primitive exit payloads.
- **Counterargument:** Four classes reduce cognitive overhead initially.

### C7 (L2.2) Stderr keyword matching is brittle and vendor-string coupled
- **Issue:** Classification depends on ad-hoc phrase presence (`"rejected"`, `"required key"`, etc.), so wording changes in primitives silently change halt semantics.
- **Cites:** `classify_failure` `510-525`.
- **Counterfactual:** Standardize machine-readable error envelope (JSON stderr/stdout or exit-code contract per primitive).
- **Counterargument:** Fast path while primitive APIs are still evolving.

### C8 (L2.3) Retry-once has no backoff/jitter despite comment promising it
- **Issue:** `FailureClass::Transient` docs claim "brief backoff" but `invoke_with_retry_once` immediately retries with zero delay.
- **Cites:** enum doc `489`, `invoke_with_retry_once` `783-803`.
- **Counterfactual:** Add bounded delay + jitter + timeout budget carryover.
- **Counterargument:** Immediate retry reduces cycle latency.

### C9 (L2.4) Exit-code input is ignored, losing high-signal classification data
- **Issue:** `classify_failure(_exit_code, stderr)` discards exit codes; all semantic power is shoved into text parsing.
- **Cites:** signature `510`; call sites `641`, `797`.
- **Counterfactual:** Reserve exit-code ranges per primitive and classify primarily by code.
- **Counterargument:** Existing primitives may not yet expose stable code taxonomy.

### C10 (L2.5) Super-step-out-of-order behavior contradicts design-scope intent
- **Issue:** Design scope says out-of-order should abort without state mutation, but current `halt_cycle` still writes runner state/history before returning hard error.
- **Cites:** design scope table `docs/redesign/_notes/cycle-149-v2-cycle-runner-design-scope.md:99-105`; `halt_cycle` writes state unconditionally `824-843`, `write_runner_state` `846-888`.
- **Counterfactual:** Either stop writing state for this class, or update design contract to explicit "record hard-error checkpoint" semantics.
- **Counterargument:** Persisting report aids postmortem.

### C11 (L2.6) Halt marker schema is underpowered for recovery/replay
- **Issue:** Stored fields omit primitive exit code, stderr fingerprint, retry count, and last successful step digest, limiting deterministic resume logic.
- **Cites:** `last-cycle.json` payload `850-860`; history entry `871-881`.
- **Counterfactual:** Add structured diagnostics and resumability metadata.
- **Counterargument:** Minimal state ownership is a deliberate simplification.

### C12 (L2.7) No crash consistency across mid-cycle failures
- **Issue:** If process dies between step side effects and `write_runner_state`, runner-local state can claim stale progress while primitive states advanced.
- **Cites:** `run_cycle` invokes primitives in-loop `637-656`, writes runner state only at halt/success boundaries `681-683`, `837`.
- **Counterfactual:** Persist append-only per-step journal after each step, then commit final summary.
- **Counterargument:** Primitive-owned state still records most truth.

## Lens 3 — Abstractions, test strategy, and scaffolding debt

### C13 (L3.1) `PrimitiveInvoker` abstraction hides real subprocess failure modes
- **Issue:** Trait models only `output()` success/failure, not hangs, partial output streaming, signal termination nuance, or timeout cancellation.
- **Cites:** trait `527-529`; `RealInvoker` `531-535`; unit tests rely on canned outputs `1003-1034`.
- **Counterfactual:** Introduce timeout-aware invocation contract and integration tests for hung/broken processes.
- **Counterargument:** Current trait unlocked broad run-loop unit coverage quickly.

### C14 (L3.2) Mock-heavy unit tests overfit implementation details
- **Issue:** Many assertions target exact arg strings/order; they will fail on harmless refactors while missing behavioral invariants like idempotent recovery.
- **Cites:** tests `1193-1251`, `1389-1407`.
- **Counterfactual:** Shift more tests toward state-transition invariants and failure contracts.
- **Counterargument:** Exact args are critical because primitives are CLI-boundary contracts.

### C15 (L3.3) Integration tests are happy-path only and miss halting contracts
- **Issue:** No integration scenario validates `RoleSessionEmpty`, `ChannelWriteRejected`, or `SuperStepOutOfOrder` behavior against real binaries.
- **Cites:** only two integration tests `tests/integration_cycle.rs:153-224, 226-422`.
- **Counterfactual:** Add fixture-driven failing session outputs and out-of-order setup to validate real halt markers and exit behavior.
- **Counterargument:** Unit tests already cover these branches with `MockInvoker`.

### C16 (L3.4) Integration harness is workspace-layout fragile
- **Issue:** Tests derive workspace root via parent-parent of `CARGO_MANIFEST_DIR` and run cargo build in-test; this assumes mono-workspace topology and toolchain presence.
- **Cites:** `workspace_root` `28-35`; `build_primitives_once` `55-81`.
- **Counterfactual:** Use prebuilt artifact env vars or dedicated test fixture binaries, and isolate build from runtime tests.
- **Counterargument:** Built-by-test setup increases hermetic reproducibility on CI.

### C17 (L3.5) `Once`-guarded primitive build leaks global mutable test state
- **Issue:** `BUILD_ONCE` makes integration tests order-dependent on first build outcome and unsuitable for parallel isolation.
- **Cites:** `static BUILD_ONCE` `53`; `build_primitives_once` `55-82`.
- **Counterfactual:** Build per-test temp target dir or explicit shared fixture setup stage with failure diagnostics.
- **Counterargument:** `Once` reduces duplicate compile cost.

### C18 (L3.6) Session-output-file scaffolding creates migration cliff
- **Issue:** Runner API currently assumes pre-materialized JSON files per role; switching to live role spawn later likely requires a command-surface break, not incremental extension.
- **Cites:** Run args `89-101`; validation `774-780`; role invoke arg assembly `728-741`.
- **Counterfactual:** Introduce role-input provider abstraction now (`file|stdin|live`) to avoid hard file-channel lock-in.
- **Counterargument:** Current role-driver is explicitly scaffold-only.

## Cross-cutting findings (missing controls / governance / operability)

### X1 Concurrency control is missing (double-run race)
- **Issue:** No lock/lease prevents two `run` invocations on same repo_root from interleaving state writes.
- **Cites:** `run_cycle` has no lock acquisition path `597-685`; state writes in `write_runner_state` are plain fs writes `846-888`.
- **Counterfactual:** Add lockfile with PID+ttl or git-based lease before step 1.
- **Counterargument:** Scheduler may currently serialize runs externally.

### X2 No per-step timeout/resource budgets
- **Issue:** Primitive subprocess calls can block indefinitely (`Command::output()`), stalling entire cycle.
- **Cites:** `RealInvoker::invoke` `533-535`.
- **Counterfactual:** Add per-primitive timeout with terminate + classified failure.
- **Counterargument:** Simpler control flow while cycle durations are still being characterized.

### X3 Repo-root/state-layout coupling is rigid
- **Issue:** Paths are hard-coded to `repo_root/state/...`, limiting portability to alternate repo layouts or subrepo operation.
- **Cites:** schema ownership output `360-367`; `write_runner_state` path building `847`; integration assertions on fixed layout `374-421`.
- **Counterfactual:** Make state root configurable or discoverable from manifest.
- **Counterargument:** Strong convention reduces configuration surface.

### X4 Observability is report-only, not operations-grade
- **Issue:** Outputs are final summary text/json; no structured per-step logs/metrics/tracing for latency, retries, or primitive stderr taxonomy drift.
- **Cites:** `emit_cycle_report` `890-913`; `StepTrace` lacks timing/result fields `538-545`.
- **Counterfactual:** Emit per-step structured events + metric counters (retry count, class rates, durations).
- **Counterargument:** Minimal logging keeps noise low during scaffold stage.

### X5 Commit-shape discipline is not enforced in artifact
- **Issue:** The crate records cycle/issue but does not verify single-direct-push or push cleanliness assumptions named in redesign notes.
- **Cites:** run args/report fields `440-452`, `548-560`; no git checks in run loop.
- **Counterfactual:** Add optional "governance checks" mode (verify clean/pushed state boundaries) or explicitly move discipline ownership to another tool.
- **Counterargument:** Commit governance may intentionally live outside runner.

### X6 Reducer-rule reliance is implicit, not asserted
- **Issue:** Runner depends on channel-router/reducer discipline but treats writes as opaque; no runner-side sanity check that role→channel mapping remained one-writer.
- **Cites:** runner just invokes primitives and records traces `614-648`; no post-step ownership validation.
- **Counterfactual:** Add lightweight post-step invariants (expected channel mutated by role) before advancing.
- **Counterargument:** Single source of truth should remain channel-router, not duplicated here.

## Integration-test failure scenarios that should exist (but currently do not)

1. **Channel-write-rejected live integration**: provide planner/executor payload missing required key and assert `status=halted`, `halt_reason=channel-write-rejected`, non-zero exit.
2. **Role-session-empty live integration**: provide empty/invalid session output for one role and assert halt marker + no subsequent advance.
3. **Super-step-out-of-order live integration**: pre-seed super-step state to wrong phase before `run` and assert hard error class behavior plus explicit state contract.
4. **Crash/restart simulation**: terminate runner between steps and assert restart behavior is deterministic (either resume policy or fail-safe abort with explicit diagnostic).
5. **Concurrent invocation test**: launch two runners against same temp repo and assert one is rejected by lock/lease (currently impossible because no lock exists).

## Bottom line

The crate is coherent as scaffold orchestration, but it currently encodes policy as hard-coded linear control flow, classifies failures with fragile string heuristics, and lacks the durability/concurrency/timeout controls needed for trustworthy production-cycle execution. The main structural risk is not any single bug; it is that the current architecture cannot make strong claims about replay safety, race safety, or taxonomy-correct recovery under real process faults.
