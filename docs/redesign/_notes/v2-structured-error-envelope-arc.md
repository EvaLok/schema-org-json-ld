# v2-* structured-error-envelope arc (C6 + C7 + C9 + adjacent X4)

**Status:** design-scope (cycle 181); implementation deferred to cycle 182+.

**Provenance:** Carry-forward from `cycle-155-cycle-152-critique-absorption.md` C6 (L2.1 taxonomy too coarse) + C7 (L2.2 stderr-keyword brittleness) + C9 (L2.4 exit-code ignored), all verdicted AGREE-DEFER pending a coordinated arc. Forward-priority #13 cycle 156, drifting through #12 → #10 → #9 → #8 → #6 → #5 → #6 → #4 cycle 180. Adjacent observation: cycle 155 X4 (observability is report-only) was noted as "possibly bundled with the structured-error-envelope work from C6" — partial bundling in this arc (envelope adjacent fields enable per-class rate aggregation; full observability dashboard remains separate).

**Triggering cycle:** Cycle 180 OQ-LS-3 reconciliation against the real `claude-code` binary surfaced the envelope shape `{subtype: "success", is_error: true, result: "Not logged in"}` — i.e., a primitive can complete its protocol message-flow cleanly AND report content-level error in the same envelope. This is concrete prior art for the cross-crate envelope shape this arc designs. Cycle 180 priority #4 named C6+C7+C9 as the next coordinated arc design scope; cycle 181 is that design.

**Scope relative to prior arcs:** Cycle 176's `v2-primitive-invoker-timeout-arc.md` (290 LOC scope; cycle 177 implementation) introduced `InvocationResult` + `TimeoutDiagnostic` + `SignalEscalation` — the FIRST structured-output types in the v2-* primitive layer, currently emitted by v2-primitive-invoker and consumed by v2-cycle-runner via `CycleReport` serialization. This arc generalizes that pattern: each v2-* primitive emits a structured error envelope on failure paths, consumed and classified by v2-cycle-runner with deterministic mapping (not stderr-keyword pattern-matching).

## 1. Problem statement

### 1.1 What exists today

**v2-cycle-runner classifier** (`tools/rust/crates/v2-cycle-runner/src/main.rs:696-711` per cycle 181 Explore agent measurement; drifted from the cycle 155 cited 510-525 due to cycle 177 PrimitiveInvoker refactor + cycle 179 extraction):

```rust
fn classify_failure(_exit_code: Option<i32>, stderr: &str) -> FailureClass {
    let lower = stderr.to_lowercase();
    if lower.contains("super-step") && lower.contains("out of order") {
        FailureClass::SuperStepOutOfOrder
    } else if lower.contains("rejected") && (lower.contains("required key")
        || lower.contains("schema mismatch")) {
        FailureClass::ChannelWriteRejected
    } else if lower.contains("empty") && (lower.contains("output")
        || lower.contains("payload") || lower.contains("session")) {
        FailureClass::RoleSessionEmpty
    } else {
        FailureClass::Transient
    }
}
```

The `_exit_code` parameter is **unused** (C9 confirmed). Classification is pure substring matching against `stderr.to_lowercase()` (C7 confirmed). The 6-variant `FailureClass` enum (Transient / RoleSessionEmpty / ChannelWriteRejected / SuperStepOutOfOrder / StateBoundExceeded / Timeout) is the "too coarse" target (C6 confirmed): it lacks auth, config, io, protocol, and other classes that real failures fall into.

**Per-primitive stderr emission today** (cycle 181 Explore agent measurements):

| Primitive | Lines | Stderr writes | Error type | Variants | Structured today? |
|---|---|---|---|---|---|
| v2-cycle-runner | 3681 | 1 (line 325) | `RunnerError::Display` | n/a (callee) | YES via `CycleReport` JSON |
| v2-channel-router | 1424 | 2 (851 lenient-warn, 1033 main) | `RouterError::Display` | 8 | NO |
| v2-role-driver | 2440 | 1 (line 1810) | `DriverError::Display` | 11 (incl 6 cycle-179 live-spawn) | NO |
| v2-primitive-invoker | 621 | none (library crate) | n/a | n/a | YES via `TimeoutDiagnostic`/`SignalEscalation` |

Exit-code emission today is uniformly `process::exit(1)` on any error in every primitive. No primitive distinguishes its failure classes via exit code.

### 1.2 Why this matters

The substring-matching classifier breaks in three ways:

1. **Re-wording silently mis-classifies.** A future Display impl that changes `"channel write rejected"` → `"reducer violation"` is now `FailureClass::Transient` (the catch-all), which triggers immediate retry — exactly wrong for what is in fact a permanent contract violation.

2. **Exit-code information is discarded.** A primitive that exits with status 42 (e.g., panic on a Rust assert!) and status 1 (normal error path) are indistinguishable to the classifier. Cycle-155 X1 / X2-adjacent concerns about retry-policy correctness can't be addressed without distinct error semantics from primitives.

3. **The class taxonomy can't grow without new substring rules.** Adding `Auth` (cycle-180 claude-code auth failure case lives here logically), `Config` (malformed CLI args or env), `Protocol` (envelope-parse failure, sub-protocol mismatch), `Io` (file-not-found, permission denied), or `External` (network, third-party API) requires either more brittle substring rules OR — the cleaner path — a structured envelope from each primitive.

The cycle-180 finding makes the urgency concrete: v2-role-driver's cycle-179 `parse_claude_code_envelope` reads claude-code's JSON envelope on stdout (richer than cycle-178 design enumerated — includes `is_error`, `subtype`, `result`, `duration_ms`, `total_cost_usd`, `usage`, etc.), but on its OWN error path emits text via `DriverError::Display`. The primitive can READ structured envelopes but can't EMIT them. C6+C7+C9 closes that asymmetry.

### 1.3 What's NOT in cycle 1 of implementation

- **Backward-compat removal.** Text-on-stderr remains the default emission for human readability of logs. JSON-on-stderr is an opt-in via `--error-format json` flag (§3 below). Removing text-on-stderr is a separate breaking-change cycle, post-arc.
- **Exit-code ranges with semantics.** Each primitive's exit-code becomes a SINGLE distinguishing signal (0=success / 1=generic / 2=class-specific-but-uniform) — exit-code-as-class-encoding is deferred to a follow-on cycle if cycle-2-3 measurement shows the JSON-only classifier is unreliable.
- **Schema-promotion of envelope fields beyond minimum.** Cycle-180 schema-promotion-discipline (HARDENED via cycle 178 `schema-promotion-requires-reader-co-edit-via-named-field`): keep envelope MINIMAL until a reader needs each field. Cycle-1 envelope is `{primitive, class, details, human}`. `invocation_id`, `timestamp`, `elapsed_ms`, `process_pid` get added when v2-cycle-runner classifier or v2-state-audit reads them — not preemptively.
- **Cross-language polyglot envelope.** Polyglot work is deferred (cycle ~200+ when non-Rust schema work begins). The envelope is JSON-on-stderr, so cross-language compatibility is preserved by accident; no extra work to enable it.
- **Streaming envelopes (mid-execution progress).** Each primitive emits the envelope ONCE on exit (success or failure). Mid-execution streaming was already deferred in cycle 176 timeout-arc §1.3; this arc honors the same scope.
- **X4 observability dashboard.** The envelope's `class` field enables per-class rate aggregation downstream, but actually building the aggregator + dashboard is a separate cycle.
- **Replacing `RouterError` / `DriverError` enums.** The envelope is an emission shape; the internal error enums remain. Mapping internal-error → envelope-class lives in a new `impl Error::to_envelope(&self) -> ErrorEnvelope` method per crate.

## 2. Design framework alignment

Per `docs/redesign/2-design-framework.md` axis-3 (state primitives) and axis-5 (failure-mode handling), this arc:

- **Strengthens axis-5** by replacing brittle substring matching with structured envelope reads — the failure-classification surface becomes machine-verifiable and amenable to negative-assertion tests.
- **Extends axis-3** by introducing a new shared type set (`ErrorEnvelope`) consumed by all v2-* primitives, mirroring the cycle 177 `TimeoutDiagnostic` precedent.
- **Honors axis-6 (one-writer-per-channel discipline)**: the envelope is a sibling artifact to the primitive's success-path stdout/state-write, NOT a substitute. No primitive's reducer-rule responsibilities change.
- **Honors CORE-DESIGN-PRINCIPLE**: the prompt-level "what failure class happened?" question moves entirely to deterministic tool code; the orchestrator never has to read stderr and pattern-match — that's v2-cycle-runner's job, and the envelope makes it deterministic rather than fragile.

## 3. Envelope shape

### 3.1 The `ErrorEnvelope` type

Lives in a new shared crate `tools/rust/crates/v2-error-envelope/`:

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ErrorEnvelope {
    /// Which v2-* primitive emitted this envelope. Kebab-case crate name
    /// (e.g., "v2-channel-router", "v2-role-driver").
    pub primitive: String,

    /// Failure class. Kebab-case taxonomy slot. See §4 for the full taxonomy.
    pub class: ErrorClass,

    /// Free-form per-class diagnostic details. Each class documents its own
    /// expected key set in §4. The Map is serde_json::Value so different
    /// classes can carry different shapes without a giant union enum.
    #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
    pub details: serde_json::Map<String, serde_json::Value>,

    /// Human-readable single-line message. The text that would appear on
    /// stderr in --error-format=text mode. Used by v2-cycle-runner when
    /// rendering CycleReport in text mode + when JSON parse fails (fallback).
    pub human: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorClass {
    /// Transient; retry safe. Network blip, fs contention, etc.
    Transient,
    /// Authentication / authorization. Cycle-180 OQ-LS-AUTH shape.
    Auth,
    /// Configuration. Missing env var, malformed CLI arg, bad config file.
    Config,
    /// I/O. File-not-found, permission-denied, disk-full.
    Io,
    /// Protocol. Envelope-parse-failure, sub-protocol mismatch, schema-mismatch.
    Protocol,
    /// Reducer-rule violation; existing class.
    ChannelWriteRejected,
    /// Empty-output-but-success; existing class.
    RoleSessionEmpty,
    /// Super-step sequence violation; existing class.
    SuperStepOutOfOrder,
    /// State bound exceeded; existing class.
    StateBoundExceeded,
    /// Timeout; cycle-177 precedent.
    Timeout,
    /// Catch-all for unknown / unclassified.
    Unknown,
}
```

### 3.2 Emission protocol

Each v2-* primitive supports an `--error-format <text|json>` flag (default `text` for backward-compat). When `--error-format=json` is set:

- On **success** paths: the primitive's normal success behavior (stdout / state-write / etc.) is unchanged. Stderr is empty.
- On **error** paths: instead of `eprintln!("v2-foo: {err}")`, the primitive emits a SINGLE LINE of compact JSON serialization of `ErrorEnvelope` to stderr, then exits with status 1 (or class-specific code in a future cycle).

v2-cycle-runner invokes every primitive with `--error-format=json` from cycle 1+. Text mode is preserved for direct human invocation (debugging, ad-hoc CLI use).

**JSON-on-stderr (not stdout) rationale:**
- Stdout is reserved for primitives' success-path data (e.g., v2-channel-router's --json-output mode, v2-state-audit's report). Mixing error envelopes into stdout muddles success/failure separation.
- Stderr is conventionally the "metadata" channel; envelopes belong there.
- Cycle 180 OQ-LS-3 measured claude-code's envelope on stdout — that's claude-code's choice; our primitives differ because our success-paths already use stdout.

### 3.3 Parser side (v2-cycle-runner)

v2-cycle-runner's `classify_failure` becomes:

```rust
fn classify_failure(
    exit_code: Option<i32>,
    stderr: &str,
) -> (FailureClass, Option<ErrorEnvelope>) {
    // Try to parse the LAST non-empty line of stderr as an ErrorEnvelope.
    if let Some(line) = stderr.lines().rev().find(|l| !l.trim().is_empty()) {
        if let Ok(envelope) = serde_json::from_str::<ErrorEnvelope>(line) {
            let class = envelope.class.into();
            return (class, Some(envelope));
        }
    }
    // Fallback: legacy substring matching against full stderr text. Logged
    // as a degraded-classification observation in CycleReport.
    let class = classify_failure_legacy(exit_code, stderr);
    (class, None)
}
```

The dual-mode parser preserves correctness even if a primitive misbehaves (panics with raw Rust backtrace on stderr — not a valid envelope). The `Option<ErrorEnvelope>` return flows into `CycleReport.traces[N].envelope` so operators see both the structural envelope (when present) and the raw stderr (always).

## 4. Per-class taxonomy + details schema

Each `ErrorClass` documents its expected `details` keys. v2-cycle-runner does NOT enforce the shape (forward-compat) — primitives are free to add keys, and the cycle report passes through whatever's there.

| Class | Expected `details` keys | Emitting primitives | Retry-policy |
|---|---|---|---|
| `transient` | (none required) | any | retry-once |
| `auth` | `subsystem` (e.g., `"claude-code-oauth"`) | role-driver (cycle-180 OQ-LS-AUTH), future | NO retry |
| `config` | `key` (config name), `expected` (optional) | any | NO retry |
| `io` | `path`, `op` (read/write/stat), `errno` (optional) | any | NO retry |
| `protocol` | `expected_shape`, `observed_shape`, `parse_error` | role-driver (envelope-parse-failure → here), channel-router (payload-schema-mismatch → here) | NO retry |
| `channel-write-rejected` | `channel`, `writer`, `reason` | channel-router | NO retry |
| `role-session-empty` | `role`, `cycle` | role-driver, cycle-runner internal | NO retry |
| `super-step-out-of-order` | `current_step`, `attempted_step` | super-step-boundary | NO retry |
| `state-bound-exceeded` | `dimension`, `observed`, `bound` | state-audit | NO retry |
| `timeout` | mirrors cycle-177 `TimeoutDiagnostic` fields | invoker-mediated (any primitive) | NO retry |
| `unknown` | (free-form, primitive-specific) | any (fallback) | retry-once (mirror Transient) |

**Existing `FailureClass` enum (in v2-cycle-runner) absorbs new classes**: extend with Auth / Config / Io / Protocol / Unknown variants. Existing 6 variants preserved.

## 5. Per-primitive impact analysis

### 5.1 v2-channel-router (cycle 2 of implementation)

Smallest emit-side delta. `RouterError` enum's 8 variants map to envelope classes as:

| `RouterError` variant | Envelope class | Notes |
|---|---|---|
| `Io(io::Error)` | `io` | `details.path` from io::Error context |
| `Json(String)` | `protocol` | `details.parse_error` |
| `InvalidPayload(String)` | `protocol` | `details.observed_shape` |
| `ReducerViolation { channel, attempted_writer, allowed_writer }` | `channel-write-rejected` | direct mapping |
| `NotInitialized(PathBuf)` | `config` | `details.path` |
| `MissingPayloadFile(PathBuf)` | `io` | `details.path, op=read` |

Add `--error-format` flag handling at CLI parse site. Add `impl RouterError { fn to_envelope(&self) -> ErrorEnvelope }`. Replace `eprintln!("v2-channel-router: {err}")` with branch on flag.

Estimated delta: ~150 source LOC + ~8 unit tests + 1 integration test exercising the JSON-mode emission end-to-end (router fails → cycle-runner classifies via envelope).

### 5.2 v2-role-driver (cycle 3 of implementation)

Highest emit-side complexity. 11 `DriverError` variants — 5 pre-cycle-179, 6 cycle-179 live-spawn. Mapping:

| `DriverError` variant | Envelope class | Notes |
|---|---|---|
| `SuperStepMismatch` | `super-step-out-of-order` | preserves existing class |
| `SuperStepNotInProgress` | `super-step-out-of-order` | sub-variant via `details.reason` |
| `CycleMismatch` | `super-step-out-of-order` | sub-variant via `details.reason` |
| `SessionOutputMissing` | `io` | scaffold-mode; deprecated post-arc |
| `InvalidSessionOutput` | `protocol` | scaffold-mode |
| `ConflictingInvokeModes` | `config` | CLI-flag conflict |
| `MissingInvokeMode` | `config` | CLI-flag missing |
| `ClaudeCodeBinMissing` | `io` | `details.path, op=stat` |
| `PromptFileMissing` | `io` | `details.path, op=read` |
| `LiveSpawnEnvelopeParseFailure(String)` | `protocol` | claude-code's envelope unparseable |
| `LiveSpawnInvocationIo(String)` | `io` | subprocess spawn-time OS error |

**Notable**: cycle-180 surfaced that role-driver can encounter `is_error: true` in claude-code's envelope (e.g., "Not logged in"). Today this becomes `Outcome::WriteSkipped` in `RoleRun`. The arc adds: when role-driver detects `is_error: true` in claude-code's envelope, role-driver emits an envelope with class derived from claude-code's `subtype` / `result` content. The Not-logged-in case maps to `class=auth, details.subsystem="claude-code-oauth", details.upstream_result="Not logged in"`. This is concrete progress on cycle-180 OQ-LS-AUTH visibility — even if Eva's secret-add is the resolution mechanism, the orchestrator-visible classification is no longer "WriteSkipped with diagnostic" but `auth` class.

Estimated delta: ~250 source LOC + ~15 unit tests + 2 integration tests (one for envelope-emit on classic DriverError, one for live-spawn-claude-code-is_error → envelope-emit path).

### 5.3 v2-cycle-runner (cycle 4 of implementation — caller-side)

The CLASSIFIER and INVOKER side. Three changes:

1. **Invoke children with `--error-format=json`.** Update every primitive-invocation site to pass the flag.
2. **Rewrite `classify_failure`.** Replace substring-only path with envelope-parse-first + legacy-substring-fallback (§3.3 above).
3. **Extend `CycleReport`** to include `envelope: Option<ErrorEnvelope>` on each `StepTrace` (alongside existing `timeout: Option<TimeoutDiagnostic>` from cycle 177). Schema-promotion-discipline applies: the field is added because the reader (downstream operator looking at the cycle report) is the cycle-runner emitting CycleReport — the reader EXISTS via the JSON-format report, so this promotion is honored.
4. **Extend `FailureClass`** with Auth / Config / Io / Protocol / Unknown variants. Update halt-decision logic: most new classes are non-retryable. `Auth` specifically (in light of cycle-180 OQ-LS-AUTH discovery) is non-retryable and produces a distinct halt-marker enabling operator escalation (potentially via a future `question-for-eva` auto-file path — separate cycle).

Estimated delta: ~300 source LOC + ~20 unit tests + 3 integration tests (envelope-roundtrip end-to-end with mock primitive, fallback-to-legacy-classification when envelope unparseable, mixed-mode where some children emit envelope and others don't).

### 5.4 v2-primitive-invoker (no direct change)

Already emits structured types (`TimeoutDiagnostic`, `SignalEscalation`). The arc consumes these into the `Timeout` class's `details` schema; no new types needed here. v2-primitive-invoker is dependency-clean.

### 5.5 Other v2-* primitives (deferred or partial)

The 4 v2-* primitives directly invoked by v2-cycle-runner are channel-router, role-driver, super-step-boundary, and (via cycle 179) primitive-invoker-mediated calls. Beyond these, v2-state-audit, v2-prompt-tag-semantic-fidelity, v2-tool-registry, v2-cycle-history-append, v2-dispatch-status, v2-reconciler-event-processor, v2-state-dispatch-archive, v2-state-dispatch-sync, v2-gardening-sweep, v2-phase-transition-check, v2-boot-phase, v2-close-phase, v2-wiki-search, v2-prompt-contract-check are NOT in v2-cycle-runner's super-step invocation path — they're separate orchestrator-or-tool-invoked binaries.

**Migration discipline**: cycle 1-4 of implementation cover the 3 actively-classified primitives (channel-router, role-driver, super-step-boundary if it has its own error paths) + cycle-runner caller-side. Other v2-* primitives migrate to envelope emission OPPORTUNISTICALLY when next touched — not as a sweep. Each carries its own ~150-200 LOC budget when migrated.

## 6. Cross-cutting design questions

### 6.1 Why JSON-on-stderr instead of exit-code ranges

Cycle-155 wording offered two paths: (a) JSON-on-stderr with class field, (b) stable exit-code ranges per primitive. This design picks (a) primarily and treats (b) as deferred:

- **(a) JSON has richer details + arbitrary key extension.** A `protocol` failure with `expected_shape` and `observed_shape` keys carries diagnostic info that no exit-code byte can.
- **(a) Avoids exit-code-namespace collisions.** Each primitive's exit-code semantics today is "0=success, 1=anything else." Reserving ranges across primitives is a coordination tax with no immediate payoff.
- **(b) defense-in-depth is worth doing later** if cycle-2-4 measurement shows envelope-parse-failure rates non-trivial (i.e., panic-stderr eating the envelope, race conditions truncating the JSON line). Each primitive can reserve `1=generic, 2-9=class-specific` AT THAT POINT — backward-compat-preserving extension.

### 6.2 Why per-line-tail JSON parse, not whole-stderr parse

Stderr may contain pre-envelope warning lines (lenient-mode validation in v2-channel-router lines 851), debug eprintln output during development, etc. The envelope is conventionally the LAST line of stderr. Parsing tail-first preserves the "envelope is the last word" convention and avoids re-parsing prefixes.

### 6.3 Why `details` is `serde_json::Map<String, Value>` rather than a typed enum-per-class

A typed-per-class details enum (`enum ErrorDetails { Auth(AuthDetails), Config(ConfigDetails), ... }`) would catch type errors at compile time but couples the shared crate to every primitive's specific error shape — versioning becomes painful (adding a key requires bumping the shared crate). The free-form Map preserves forward-compat: any primitive can add a new details key without coordinating a release across all consumers.

The cost is run-time validation (cycle-runner consumes details opaquely, mostly for display). v2-cycle-runner DOES type-check the `class` enum (compile-time exhaustive match) — that's where the safety lives.

### 6.4 Why the `human` field rather than always rendering from `details`

The Display impl of each primitive's internal error type today produces well-tested human-readable text. Reusing that as `human` field preserves operator-readable stderr (in text mode) AND gives JSON-mode a fallback when the cycle-runner can't render `details` (e.g., unrecognized class in a forward-compat scenario). It's deliberate duplication of information for forward-compat resilience.

### 6.5 Why cycle 4 (cycle-runner caller-side) is LAST, not first

If cycle-runner expects envelopes from primitives that don't yet emit them, every test breaks immediately. If primitives emit envelopes that nothing consumes, the emit side is dead code visible in stderr only. **Bottom-up migration** (envelope shared crate → router → role-driver → cycle-runner consumer) keeps each cycle's commit independently passing CI:

- Cycle 1: new crate compiles + tests; nobody depends on it yet.
- Cycle 2: router emits envelope in JSON-mode (flag opt-in); nobody invokes with the flag yet; CI green.
- Cycle 3: role-driver emits envelope similarly; cycle-runner still invokes without flag; CI green.
- Cycle 4: cycle-runner adds flag + classifier rewrite. NOW the end-to-end path is live. CI green requires all 3 prior cycles' work landed.

## 7. Acceptance criteria

### 7.1 Per-cycle acceptance

**Cycle 1 (this cycle's design + new shared crate):**
- [x] design scope authored (this file)
- [ ] `tools/rust/crates/v2-error-envelope/` crate created (cycle 182+)
- [ ] `ErrorEnvelope` + `ErrorClass` types + Serialize/Deserialize round-trip tests + integration with v2-primitive-invoker `TimeoutDiagnostic` (cycle 182+)
- [ ] `cargo clippy --workspace --tests -- -D warnings` clean

**Cycle 2 (v2-channel-router emit-side):**
- [ ] `--error-format <text|json>` flag added to CLI
- [ ] `RouterError::to_envelope(&self) -> ErrorEnvelope` impl
- [ ] All 8 variants mapped per §5.1 table
- [ ] Unit tests verify envelope shape for each variant
- [ ] Integration test: invoke router with `--error-format=json` on a deliberately-erroring input, parse envelope from stderr last line, assert class + key details

**Cycle 3 (v2-role-driver emit-side):**
- [ ] same flag + impl pattern as cycle 2
- [ ] All 11 variants mapped per §5.2 table
- [ ] cycle-180 OQ-LS-AUTH case specifically: when `parse_claude_code_envelope` returns `is_error=true` with `result="Not logged in"` (or similar auth-failure shape), role-driver emits envelope with `class=auth`
- [ ] Unit tests for each variant + the live-spawn-auth case

**Cycle 4 (v2-cycle-runner caller-side):**
- [ ] All super-step primitive invocations pass `--error-format=json`
- [ ] `classify_failure` rewritten per §3.3 (envelope-parse-first + legacy-fallback)
- [ ] `FailureClass` extended with Auth / Config / Io / Protocol / Unknown
- [ ] `CycleReport.traces[N].envelope: Option<ErrorEnvelope>` added with schema-promotion-discipline justification (reader = cycle-runner JSON report output)
- [ ] Halt-decision logic extended: Auth / Config / Io / Protocol / Channel* / RoleSession* / SuperStep* / StateBound / Timeout all non-retryable; Transient + Unknown retryable-once
- [ ] Negative-assertion tests: stderr keyword `"out of order"` no longer drives classification (envelope class is authoritative)
- [ ] End-to-end integration test: child primitive emits envelope → cycle-runner classifies via envelope, not legacy substring

### 7.2 Overall arc acceptance (post cycle 4)

- [ ] `tests/integration_cycle.rs` exercises envelope-roundtrip end-to-end with at least 2 distinct primitives and at least 3 distinct classes
- [ ] `cargo clippy --workspace --tests -- -D warnings` clean across the full 4-cycle commit history
- [ ] `cargo test --workspace` green at every cycle boundary (no commit leaves the tree red)
- [ ] No stderr-substring-match remains in `classify_failure` (the legacy fallback is preserved but only fires when envelope parse fails)
- [ ] cycle-180 OQ-LS-AUTH case (which is the cycle-181-named motivating failure) produces `class=auth` in CycleReport when v2-role-driver encounters claude-code "Not logged in"

## 8. Honest LOC + cycle estimate

Per cycle 179's `comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-4 data (1.58× raw / 0.79× honest), the multiplier for thorough-design-scopes is 1.2-2.0× honest. This design scope is moderately-thorough (~400 lines, comparable to cycle-176 timeout-arc at 290 LOC scope + cycle-177 implementation at ~835 LOC actual; vs cycle 178 live-spawn at 548 LOC scope + cycle-179 implementation at 1502 LOC actual).

Raw estimate per §5:
- Cycle 1 (shared crate + types): ~250 LOC source + 10 tests ≈ 350 raw
- Cycle 2 (router): ~150 LOC source + 8 unit + 1 integration ≈ 250 raw
- Cycle 3 (role-driver): ~250 LOC source + 15 unit + 2 integration ≈ 500 raw
- Cycle 4 (cycle-runner): ~300 LOC source + 20 unit + 3 integration ≈ 600 raw

Raw total: ~1700 LOC. Honest budget at 1.2-2.0×: 2000-3400 LOC. Comparable to cycle-178 live-spawn arc (548 design / 1502 actual). Implementation cycle estimate: 4 cycles minimum, 5-6 if reconciliation pass (cycle-180 pattern) is required for any cycle.

**Forward-watch**: this arc accumulating to 5+ cycles would be a fifth `comprehensive-test-suite-exceeds-design-scope-LOC-estimate` data point — important for the multiplier-refinement question. Cycle 180 noted: when design scope is THOROUGH (548 lines, explicit honest-budget acknowledgment, 10 enumerated OQ items deferred with rationale), implementation lands within or under raw. THIS design scope is mid-thorough (~400 lines, 8 OQ-SEE items, explicit non-scope §1.3 list); the prediction is 1.4-1.7× raw, i.e., ~2400-2900 LOC actual. Will measure cycle 182-185+.

## 9. Open questions (OQ-SEE-*)

Items requiring measurement against real primitive behavior before cycle 4 caller-side migration.

- **OQ-SEE-1**: Does `serde_json::Map<String, Value>` serialize compactly enough? Cycle 2 measurement: emit envelope for the largest `RouterError` variant (probably `ReducerViolation` with channel name + writer + allowed-writer), measure byte length. If >1KB on single-line, consider compact-key naming. Likely fine; flag for verification.

- **OQ-SEE-2**: When stderr already contains pre-envelope warning lines (router lenient-mode line 851), does the last-line parse correctly find the envelope? Cycle 2 integration test should deliberately combine pre-envelope warnings + envelope final line, assert parse picks the envelope.

- **OQ-SEE-3**: How does cycle-179's `parse_claude_code_envelope` fit into the new envelope-emission shape? Two options: (a) keep `parse_claude_code_envelope` returning `Err(String)` and let the role-driver's outer error path map to envelope class; (b) refactor `parse_claude_code_envelope` to return `Result<_, ParseEnvelopeError>` where ParseEnvelopeError has structured fields. Cycle 3 design moment; defer to that cycle.

- **OQ-SEE-4**: Should `class=auth` envelope details include the upstream-system's own envelope embedded? I.e., for cycle-180 case, should `details.upstream_envelope` carry the full claude-code JSON envelope? Pro: full diagnostic chain; Con: size + sensitive fields (claude-code envelope includes `session_id`, `uuid`, etc.). Default: include only `upstream_result` (the human-error string), defer full-envelope embedding.

- **OQ-SEE-5**: Do we need a `version: u32` field on `ErrorEnvelope` for future schema evolution? Pro: explicit versioning. Con: most-likely-never-used; schema-promotion-discipline says defer. Default: NO version field at cycle 1; add when first incompatible change is contemplated.

- **OQ-SEE-6**: How does the `Unknown` class interact with retry policy? `Unknown` is the "envelope was parseable but class isn't recognized by this version of cycle-runner" case (forward-compat). Default: retry-once (mirror Transient) — operator sees the unknown class in CycleReport and decides. Cycle 4 acceptance criteria validate this.

- **OQ-SEE-7**: Should the legacy substring-fallback in `classify_failure` log a degraded-classification warning to CycleReport? Pro: visibility into envelope-emission-coverage. Con: noise during the multi-cycle migration when most invocations are non-envelope. Default: yes, log once-per-cycle (not per-step) as a `CycleReport.degraded_classifications: u32` counter.

- **OQ-SEE-8**: Does the `--error-format` flag belong as a long-option per-primitive, or as a v2-cycle-runner-wide env var (e.g., `V2_ERROR_FORMAT=json`)? Long-option is explicit; env var is propagation-cheap. Default: long-option (per-primitive explicit). Cycle 2 implementation may revisit if invocation-site verbosity becomes painful.

## 10. Implementation cycle plan (proposed)

| Cycle | Substantive focal | Estimated LOC | Track-2 commit |
|---|---|---|---|
| 181 | Design scope (this file) | ~400 doc LOC | _notes + journal |
| 182+ | C6+C7+C9 cycle 1: v2-error-envelope crate + types + tests | ~350 raw | _notes + journal |
| 183+ | C6+C7+C9 cycle 2: v2-channel-router emit-side + integration test | ~250 raw | _notes + journal |
| 184+ | C6+C7+C9 cycle 3: v2-role-driver emit-side incl cycle-180 OQ-LS-AUTH visibility | ~500 raw | _notes + journal |
| 185+ | C6+C7+C9 cycle 4: v2-cycle-runner caller-side + classifier rewrite + FailureClass extension | ~600 raw | _notes + journal |
| 186+ | (potential reconciliation cycle if needed; mirrors cycle-180 pattern) | ~100 raw | _notes + journal |

**Implicit-arc-serialization**: per `eva-directive-overrides-implicit-arc-serialization` pattern (cycle 178), this arc runs serially unless an Eva directive opens a parallel track. Cycle 180 close named #2997 AUTH-resolution as the unblock signal for cycle 2 of live-spawn arc; that arc and THIS arc do NOT conflict (different code paths) — but the implicit serialization convention says one substantive arc at a time. If #2997 resolves during this arc, cycle 181+ work pauses for live-spawn cycle 2 success-path acceptance (per cycle-180 priority #3), then resumes here.

## 11. Forward priorities post-arc

When this arc closes (cycle 185-186+):

- **X4 observability** is now partially unblocked: CycleReport carries `class` field per failure → per-class rate aggregation is a viable next arc.
- **C11+C12+X1 resume/recovery coordinated arc** moves up in priority: structured envelopes give resume-logic deterministic class-routing.
- **Cross-language polyglot primitive support**: JSON envelopes are language-agnostic. When polyglot work begins, no envelope-shape work needed.
- **Exit-code-range defense-in-depth**: if cycle 4 acceptance shows envelope-parse-failure rate >5%, add reserved exit-code ranges per primitive. Else defer indefinitely.

## 12. Provenance + cross-references

- **Original critique source**: `docs/redesign/_critique/cycle-152-v2-cycle-runner-adversarial-critique.md` (PR-less Copilot dispatch #2960, cycle 152, landed by cycle 155 direct-push `3607915f`).
- **Absorption cycle**: `docs/redesign/_notes/cycle-155-cycle-152-critique-absorption.md` §C6/C7/C9.
- **Carry-forward trail**: cycles 156→159→160→161→162→163→164→165→166→167→168→169→170→171→172→173→174→175→176→177→178→179→180 → THIS DESIGN cycle 181.
- **Adjacent arcs**: cycle 176's `v2-primitive-invoker-timeout-arc.md` (precedent: structured types for primitive failure-shape), cycle 178's `v2-role-driver-live-spawn-arc.md` (precedent: JSON envelope from external subprocess).
- **Triggering finding**: cycle 180's OQ-LS-3 reconciliation of claude-code JSON envelope shape (cycle-180 _notes §3).
- **Affected primitives**: v2-cycle-runner, v2-channel-router, v2-role-driver. v2-primitive-invoker dependency-clean; other v2-* primitives migrate opportunistically.
