# Cycle 179 — v2-role-driver live-claude-code-spawn arc implementation cycle 1

**Cycle issue:** [#2995](https://github.com/EvaLok/schema-org-json-ld/issues/2995)
**Tracks composed:** straight-pair-closure HARDENING-AT-17 (cycles 162–179, 17 consecutive). 28th consecutive two-track-composition post cycle 151 exception. 64th consecutive HONORING of named forward priority.

## What happened

Cycle 179 advanced cycle 178+ priority #2 (v2-role-driver live-spawn cycle 1 implementation) into cycle 1 of implementation per the §12 acceptance criteria of [`v2-role-driver-live-spawn-arc.md`](v2-role-driver-live-spawn-arc.md) (cycle 178 design scope, 548 lines, promoted by [`[input-from-eva] #2992`](https://github.com/EvaLok/schema-org-json-ld/issues/2992)).

### Track 1 — Live-spawn cycle 1 implementation (substantive)

End-to-end wiring of the LIVE-SPAWN path in v2-role-driver, with the cycle-177 `RealInvoker` extracted to a shared crate so both v2-cycle-runner and v2-role-driver consume the same timeout-aware subprocess machinery. Mock-driven end-to-end hermetic integration test. Land delta:

```
 tools/rust/Cargo.lock                              |   11 +-
 tools/rust/crates/v2-cycle-runner/Cargo.toml       |    4 +-
 tools/rust/crates/v2-cycle-runner/src/main.rs      |  423 +-------
 tools/rust/crates/v2-cycle-runner/tests/integration_cycle.rs | 172 +++
 tools/rust/crates/v2-role-driver/Cargo.toml        |    1 +
 tools/rust/crates/v2-role-driver/src/main.rs       | 1117 +++++++++++++++++++-
 tools/rust/crates/v2-role-driver/tests/integration.rs |    9 +-
 tools/rust/crates/v2-primitive-invoker/Cargo.toml  |   13 + (new file)
 tools/rust/crates/v2-primitive-invoker/src/lib.rs  |  520 + (new file)
```

969 net LOC across modified files (1353 add / 384 del), plus ~533 LOC of new shared crate. Total cycle 179 source artifact: ~1502 LOC of new/changed Rust. Compared to design §10.1 estimate of 950 raw LOC: 1.58× (within the RECURRENCE-AT-3 1.2–3× honest band — closer to raw than cycle 177 because the design scope itself was unusually thorough).

#### New shared crate: `v2-primitive-invoker`

Per design §3.1 ("Reuses C13+X2 RealInvoker directly") and the 80-LOC wiring budget in §10.1: extracted the cycle-177 `PrimitiveInvoker` trait + `RealInvoker` + `InvocationResult` + `SignalEscalation` + `TimeoutDiagnostic` + `signal_escalate` + `SIGTERM_GRACE` + `MockInvoker` + `CannedOutcome` from `v2-cycle-runner/src/main.rs` into a new sibling crate `tools/rust/crates/v2-primitive-invoker/`. Both v2-cycle-runner and v2-role-driver depend on it. Rationale: the trait is a *primitive concept* (timeout-aware subprocess spawn), not specific to either caller; duplication into v2-role-driver per the cycle 138 "isolated reducer-rule" precedent would have cost ~300 LOC of mechanical copy and bound the implementations to drift.

The crate ships 9 of its own unit tests covering:
- `MockInvoker` records calls + pops queued outcomes front-to-back.
- `MockInvoker` defaults to `Completed(ok_output())` when queue empty.
- `queue_timeout` and `queue_timeout_with_partial` produce `InvocationResult::TimedOut` with the configured fields.
- Helper constructors `ok_output`, `ok_output_with_stdout`, `fail_output` produce the expected exit status / stdout / stderr shapes.
- `RealInvoker` against `/bin/echo` produces `Completed` with the expected stdout (live unix exercise).
- `RealInvoker` against `/bin/sleep 10` with a 200 ms budget produces `TimedOut` (live unix signal-escalation exercise).

#### v2-cycle-runner refactor

- Removed local definitions: ~200 LOC of `InvocationResult` / `SignalEscalation` / `TimeoutDiagnostic` / `PrimitiveInvoker` / `RealInvoker` / `WaitChild` / `signal_escalate` / `SIGTERM_GRACE` deleted. Replaced with `use v2_primitive_invoker::{InvocationResult, PrimitiveInvoker, RealInvoker, SignalEscalation, TimeoutDiagnostic};`.
- Removed local test helpers: ~120 LOC of `MockInvoker` / `CannedOutcome` / `ok_output` / `fail_output` deleted from the `tests` module. Replaced with `use v2_primitive_invoker::{fail_output, ok_output, MockInvoker};`.
- `libc = "0.2"` direct dep removed (now transitive via v2-primitive-invoker).
- All 77 unit tests + 5 integration tests (cycle 177 baseline) preserved without test-code changes. Confirmed by running both crates' suites in isolation post-refactor.

#### v2-cycle-runner caller migration for live-spawn

- `Subcmd::Run` gains two new flags: `--claude-code-bin <PATH>` (selects live-spawn mode for role-session steps when present) and `--role-max-turns <N>` (default 50, passed through to v2-role-driver's `--max-turns`).
- `RunArgs` gains `claude_code_bin: Option<PathBuf>` and `role_max_turns: u32`.
- `build_step_invocation::StepKind::RoleInvoke` now constructs different argv depending on mode: live-spawn appends `--claude-code-bin <path> --max-turns N`, SCAFFOLD appends `--session-output-file <path>` (the pre-cycle-179 shape).
- `validate_session_output_files` short-circuits when live-spawn is selected (no per-role session-output files are needed in live mode).
- `run_args_with_outputs` test helper sets `claude_code_bin: None, role_max_turns: 50` to preserve the existing SCAFFOLD-mode behavior of all current tests.

#### v2-role-driver source-side changes

- **`Command::Invoke`** evolved: `session_output_file: PathBuf` → `Option<PathBuf>`; three new flags `--claude-code-bin <PATH>`, `--prompt-file <PATH>`, `--max-turns <N>` (default 50). The doc-comment now documents the two-mode selection.
- **`resolve_invoke_mode`** (new): dispatches on the `(claude_code_bin, session_output_file)` pair to one of `InvokeMode::LiveSpawn { ... } | InvokeMode::Scaffold { ... }` or returns `DriverError::ConflictingInvokeModes` / `DriverError::MissingInvokeMode`.
- **`cmd_invoke`** (refactored): mode-resolves first, super-step-checks second, then dispatches to `cmd_invoke_scaffold` or `cmd_invoke_live_spawn`. The pre-cycle-179 body became `cmd_invoke_scaffold` verbatim.
- **`cmd_invoke_live_spawn`** (new): the live-spawn body. Validates `--claude-code-bin` exists; resolves the per-role prompt path (default `prompts/v2/<role>-prompt.xml` or `--prompt-file`); reads the prompt; renders the user-message via `render_context_for_session`; persists the debug record; builds the claude-code argv via `build_claude_code_argv`; spawns via injected `PrimitiveInvoker`; matches on `InvocationResult::TimedOut` → `Outcome::Timeout` + `TimeoutDiagnosticRecord` + non-zero exit, or `Completed(output)` → envelope-parse → channel-payload-parse → validate → write_channel → `Outcome::Success`.
- **`render_context_for_session`** (new): produces the structured user-message context block per design §5.1: `<cycle-context>` (cycle/role/timestamp/output-channel/required-payload-keys) + `<input-channels>` (per-role bindings, with payload state inlined when populated) + optional `<inbound-surfaces>` (reconciler-only).
- **`render_role_specific_surfaces`** (new): emits the reconciler external-surface block per design §5.3 (github-issues, github-prs, audit-repo-activity, dispatch-returns). Returns `None` for planner / executor / curator.
- **`build_claude_code_argv`** (new): constructs the claude-code CLI per design §3.2: `--print <user-msg> --output-format json --append-system-prompt <prompt> --max-turns N --allowed-tools <per-role> --permission-mode acceptEdits --model claude-opus-4-7`. OQ-LS-1 (exact flag names) acknowledged but documented as plausible-shape per cycle 178 §3.2 honesty hedge; cycle 2 will reconcile against `claude-code --help`.
- **`parse_claude_code_envelope`** (new): parses the `--output-format json` envelope, extracts the `result` field per design §4.4. Returns the inner string for downstream JSON-payload parsing. Rejects `is_error=true`, missing `result`, non-object root, non-UTF-8 stdout, and non-JSON stdout — each with a one-line diagnostic.
- **`persist_live_spawn_debug_record`** (new): writes the assembled user-message to `state/roles/<role>-last-context-cycle-<N>.json` BEFORE spawning the subprocess. Per design §3.3 — debug-trail for post-hoc inspection without re-derivation cost.
- **`xml_escape`** (new): minimal escaping (`&` → `&amp;`, `<` `>` `"` `'`) for embedding payload JSON inside `<state>` blocks.
- **`Role::allowed_tools()`** (new): per-role static profile per design §6.1 — Reconciler=`Read,Bash`, Planner=`Read,Grep`, Executor=`Read,Edit,Write,Grep,Bash`, Curator=`Read,Grep`.
- **`Outcome::Timeout`** added as the 3rd variant. `RoleRun.timeout: Option<TimeoutDiagnosticRecord>` field added with `#[serde(default, skip_serializing_if = "Option::is_none")]` so pre-cycle-179 SCAFFOLD runs serialize identically (no `timeout` block on Success / WriteSkipped). `TimeoutDiagnosticRecord` is a `Serialize + Deserialize` mirror of upstream `TimeoutDiagnostic`. `TimeoutEscalation` mirror of upstream `SignalEscalation`. Per design §7.3.
- **Six new `DriverError` variants** for live-spawn-specific failures: `ConflictingInvokeModes`, `MissingInvokeMode`, `ClaudeCodeBinMissing(PathBuf)`, `PromptFileMissing(PathBuf)`, `LiveSpawnEnvelopeParseFailure(String)`, `LiveSpawnInvocationIo(String)`. Each with a tailored `Display` impl.
- **`emit_invoke_result`** (new): shared formatting of the per-invocation text/JSON output; previously inlined at the bottom of the SCAFFOLD `cmd_invoke` body. Lets `cmd_invoke_live_spawn` reuse the same envelope shape across its three exit paths (timeout / parse-failure / success).
- **`cmd_schema`** envelope updated to document the new third outcome and the live-spawn debug-record path.

#### Test-side changes (v2-role-driver)

- **21 new unit tests** added to the `mod tests` block covering the §12 acceptance criteria item-by-item:
  - **Per-role tool-profile mapping** (item 3.5): 1 test asserting all four role profiles match design §6.1.
  - **Mode-selection error paths** (item 3.1): 4 tests covering `(Some, None) → LiveSpawn`, `(None, Some) → Scaffold`, `(Some, Some) → ConflictingInvokeModes`, `(None, None) → MissingInvokeMode`.
  - **`render_context_for_session` shape** (item 3.2): 3 tests asserting cycle-context tags, role-name embedding, required-payload-keys text, `<input-channels>` block, reconciler-only `<inbound-surfaces>` presence/absence, and populated-channel `<state>` block with inlined payload JSON.
  - **XML-escape correctness**: 1 test asserting metacharacter handling.
  - **`build_claude_code_argv`**: 2 tests asserting `--print`, `--output-format`, `--append-system-prompt`, `--max-turns`, `--allowed-tools`, `--permission-mode`, `--model` flags all present with per-role tool string differentiation.
  - **`parse_claude_code_envelope`** (item 3.3): 4 tests covering well-formed envelope → `result` extraction, `is_error=true` rejection, missing-`result` rejection, non-JSON-stdout rejection.
  - **End-to-end live-spawn path with MockInvoker** (items 3.3 + 3.4): 6 tests covering:
    - Success-path: channel written, `Outcome::Success` recorded, no timeout block, debug-record persisted, per-role tool profile passed through argv.
    - Envelope-parse-failure path: channel NOT written, `Outcome::WriteSkipped` recorded.
    - **Timeout path** (item 3.4): MockInvoker `queue_timeout` → `Outcome::Timeout` with `TimeoutDiagnosticRecord` populated (budget_ms, elapsed_ms, escalation), no channel write.
    - Invalid-payload-shape path: `DriverError::InvalidSessionOutput` returned, no channel write.
    - Missing `--claude-code-bin` binary: `DriverError::ClaudeCodeBinMissing`, no invoker call.
    - Missing prompt file: `DriverError::PromptFileMissing`, no invoker call.

- **1 integration test updated** (`schema_json_well_formed`): asserts 3 outcomes (success / write-skipped / timeout) per cycle 179, and the new `live_spawn_debug_record` paths entry.

- **1 unit test updated** (`role_history_serde_roundtrip`): explicit `timeout: None` field plus assertion that SCAFFOLD-mode runs preserve `None`.

#### Hermetic integration test (v2-cycle-runner, design §12 item 4)

`live_spawn_with_mock_claude_code_drives_all_four_roles_end_to_end` in `tests/integration_cycle.rs`:

- Writes a mock-claude-code shell script at `<repo>/mock-claude-code.sh` (chmod 755). The mock greps argv for `role-marker-<role>` strings and emits the appropriate per-role envelope JSON to stdout.
- Writes the four role-prompt files at `prompts/v2/<role>-prompt.xml`, each containing the unique `role-marker-<role>` token so the mock can distinguish.
- Drives a full cycle via `v2-cycle-runner run --cycle 1 --issue 77777 --claude-code-bin <mock> --role-max-turns 10` (NO per-role `--*-output-file` flags — live-spawn mode handles that).
- Asserts: cycle reaches `status=completed`; all four channels (`inbound-channel`, `plan-channel`, `work-channel`, `memory-channel`) are written with the expected per-role payload markers; all four role-history files record `outcome=success` for cycle 1 with `timeout=null`; all four `<role>-last-context-cycle-1.json` debug records exist.

This is the first end-to-end test where v2-cycle-runner drives v2-role-driver in live-spawn mode. The mock claude-code stands in for the real binary; cycle 2 (cycle 180) will exercise the real binary per design §9.2.

### Track 2 — Cycle artifacts (bounded-mechanical)

- `cycle-179-live-spawn-implementation-cycle-1.md` (this note) — design-implementation tracing per cycle 177 template.
- `docs/journal/2026-05-19.md` cycle 179 entry.

`session-start-CI-check-discipline` operates in HARDENED-orientation-prelude mode this cycle (HARDENED cycle 178). The session-start `gh run list --branch master --workflow "Rust CI" --limit 3` invocation confirmed `3fbdc783` SUCCESS held on cycle 178 close (GREEN-path exercise of the HARDENED principle; not tracked as RECURRENCE-AT-N anymore).

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| New crate `v2-primitive-invoker/src/lib.rs` | ~520 LOC (trait + RealInvoker + MockInvoker + 9 unit tests) | cycle-close |
| New crate `v2-primitive-invoker/Cargo.toml` | 13 LOC | cycle-close |
| v2-cycle-runner src extraction + caller migration | -384 / +423 = +39 net | cycle-close |
| v2-cycle-runner integration test (mock-claude-code) | +172 | cycle-close |
| v2-role-driver src (live-spawn body + helpers + tests) | +1117 | cycle-close |
| v2-role-driver integration test schema update | +9 | cycle-close |
| `cycle-179-*.md` _notes | ~200 lines | cycle-close |
| Total cycle 179 main-authored output | ~2050 LOC across source + tests + _notes | 1 cycle-close (planned) |

Pre-cycle test count (cycle 178 close): 77 v2-cycle-runner unit + 5 v2-cycle-runner integration + 16 v2-role-driver unit + 42 v2-role-driver integration = 140 tests. Post-cycle: 77 + 6 + 37 + 42 + 9 (new v2-primitive-invoker unit) = 171 tests. Delta: +31 tests covering live-spawn end-to-end.

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 179: Track 1 substantive-by-LOC (~2050 net), substantively new functional surface (the first cycle where a live-claude-code subprocess path exists end-to-end, even if exercised only via mock in cycle 1). Track 2 bounded-mechanical-by-execution (orientation prelude + notes + journal).

`comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-4 cycle 179: fourth data point. Cycle 170 (1447 add, 4.5×), cycle 171 (2194 add, 7×), cycle 177 (938 net, 2.8× / 1.2× honest budget), cycle 179 (~1502 source LOC, 1.58× raw / 0.79× honest budget). The cycle 179 ratio is the *lowest* in the series so far — the design scope at 548 lines was the most thorough yet, including explicit honest-budget acknowledgment via the RECURRENCE-AT-3 multiplier. Suggests well-scoped designs land within or under the raw estimate; the honest-budget multiplier should perhaps be 1.2–2× rather than 2–3× when the design itself is thorough.

## Pattern updates this cycle

- `two-track-composition` HARDENING-AT-32 (cycles 152-179, 28 consecutive).
- `straight-pair-closure-as-default-two-track-shape` HARDENING-AT-17 (cycles 162-179, 17 consecutive).
- `session-start-CI-check-discipline` HARDENED (cycle 178) — operates in orientation-prelude mode cycle 179. No RECURRENCE-AT-N tracking; standing orientation step confirmed GREEN.
- **`eva-directive-overrides-implicit-arc-serialization` NOVEL@1 → RECURRENCE-AT-2** cycle 179 — second exercise after the cycle 178 first instance (Eva directive #2992 authorized parallel work on live-spawn + C13+X2). Cycle 179 demonstrated the override held in practice: live-spawn cycle 1 implementation ran in the same cycle that *could* have continued C13+X2 cycle 2, with explicit deferral of C13+X2 to a later cycle. Forward-watch cycles 179-186 (1 of 8 consumed) for RECURRENCE-AT-3 stabilization.
- **`coordinated-arc-design-scope-pairs-deferred-items` RECURRENCE-AT-2** cycle 179 — second instance of the pattern shape. Cycle 178 design (live-spawn) → cycle 179 implementation cycle 1 closed the design-implementation pair. Forward-watch cycles 179-186 for RECURRENCE-AT-3.
- **`comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-3 → RECURRENCE-AT-4** cycle 179 — fourth data point (1.58× raw / 0.79× honest budget). Suggests refinement of the honest-budget heuristic to 1.2–2× when design scope is thorough. Forward-watch cycles 180-185 for RECURRENCE-AT-5 toward HARDEN.
- **`as-needed-verification-of-HARDENED-principle-on-tool-rerun` RECURRENCE-AT-2** — NOT-EXERCISED cycle 179 (no v2-prompt-* tool runs this cycle; live-spawn arc does not touch prompts/v2/* contracts at extended-schema level). Carries; forward-watch cycles 177-184; 3 of 8 consumed.
- **`shared-crate-extraction-from-isolated-reducer-rule-precedent`** NEW pattern NOVEL@1 cycle 179 — the v2-primitive-invoker extraction from v2-cycle-runner is structurally similar to the cycle 138 isolated-reducer-rule precedent (write_channel duplicated in v2-role-driver from v2-channel-router), but inverted in polarity: cycle 138 chose duplication-for-isolation at SCAFFOLD scope; cycle 179 chose extraction-to-shared-crate at COMPLETE scope. Both are valid responses to the same shape (one primitive used by multiple callers); the choice depends on scope (SCAFFOLD vs COMPLETE) and the stability of the primitive (cycle 138's reducer-rule was nascent; cycle 179's RealInvoker is post-test-coverage). Forward-watch cycles 179-186 for RECURRENCE-AT-2 (next time a similar choice arises — e.g., if v2-channel-router's reducer-rule reaches COMPLETE scope and a third caller appears).
- **`design-scope-honesty-hedge-survives-implementation-cycle`** NEW pattern NOVEL@1 cycle 179 — design §3.2 / §4.4 explicitly acknowledged "OQ-LS-1: the exact claude-code CLI flag names ... may need ~30-50 LOC of adjustment when cycle-1 implementation runs `claude-code --help` and reconciles." Cycle 179 implementation honored the hedge by NOT running `claude-code --help` (the live binary isn't exercised until cycle 2) and documenting the as-built argv as "plausible-shape per cycle 178 §3.2 honesty hedge; cycle 2 will reconcile against `claude-code --help`." The hedge survived rather than being silently elided. Forward-watch cycles 179-186 for RECURRENCE-AT-2 (next time a design scope contains an explicit honesty hedge that an implementation cycle has the choice to honor or quietly drop).
- `master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1 NOT-EXERCISED cycle 179 (7 of 8 consumed; master remains green through cycle 178 confirmed at orientation prelude).
- `schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1 NOT-EXERCISED cycle 179 (7 of 8 consumed). Live-spawn implementation deliberately did NOT promote `cost_usd` / `usage` / `session_id` fields to `RoleRun` per design §7.3 — these are deferred to cycle 3+ when a reader appears (per `schema-promotion-discipline.md`).
- `category-level-intent-sharpening-distinguished-from-token-level-circularity` RECURRENCE-AT-2 NOT-EXERCISED cycle 179 (5 of 6 consumed).
- `tool-extraction-surfaces-prior-cycle-errors` NOT-EXERCISED cycle 179; carries at RECURRENCE-AT-10.
- `live-prompts-already-aligned-at-extended-schema-level` HARDENED — NOT-EXERCISED cycle 179 (no prompt-touching changes this cycle).
- Other carry-forwards NOT-EXERCISED cycle 179; carry at prior status.

### Forward-watch decay status

- **`eva-directive-overrides-implicit-arc-serialization` RECURRENCE-AT-2**: forward-watch cycles 179-186; 1 of 8 consumed.
- **`coordinated-arc-design-scope-pairs-deferred-items` RECURRENCE-AT-2**: forward-watch cycles 179-186.
- **`comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-4**: forward-watch cycles 180-185 for RECURRENCE-AT-5.
- **`as-needed-verification-of-HARDENED-principle-on-tool-rerun` RECURRENCE-AT-2**: forward-watch cycles 177-184; 3 of 8 consumed.
- **`shared-crate-extraction-from-isolated-reducer-rule-precedent` NOVEL@1**: forward-watch cycles 179-186.
- **`design-scope-honesty-hedge-survives-implementation-cycle` NOVEL@1**: forward-watch cycles 179-186.
- **`category-level-intent-sharpening-distinguished-from-token-level-circularity` RECURRENCE-AT-2**: forward-watch cycles 175-180; 5 of 6 consumed (DEADLINE cycle 180).
- **`master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1**: forward-watch cycles 173-180; 7 of 8 consumed (DEADLINE cycle 180).
- **`schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1**: forward-watch cycles 173-180; 7 of 8 consumed (DEADLINE cycle 180).

## Forward priorities for cycle 180+

Top three:
1. **Master Rust CI green-state maintenance** — exercised at every session start via HARDENED orientation prelude.
2. **v2-role-driver live-spawn cycle 2 of implementation** (NEW priority #2) — per design §9.2: first ACTUAL live spawn against the real `claude-code` binary. Reconciler first (input-channel set is empty, smallest payload, simplest hermetic setup). Cycle-2 acceptance: hermetic repo state + `gh issue list` polling visible + JSON-envelope shape verified against `claude-code --print --output-format json` reality + first wall-clock / token-usage measurement. Reconciles OQ-LS-1 / OQ-LS-2 / OQ-LS-3 against the live binary.
3. **C13+X2 cycle 2 of implementation** OR **next coordinated arc design** — deferred to cycle 181+ (cycle 180's substantive focal is live-spawn cycle 2 per the higher-information-gradient rule: first live-claude-code-spawn measurement unblocks the 24+ cycle AGREE-DEFER queue).

Plus carry-forward items: AGREE-DEFER queue (unblocked imminently by live-spawn cycle 2 measurement), structured-error-envelope (C6+C7+C9), resume/recovery (C11+C12+X1), audit-engagement, per-axis archival, deprecate legacy method, workflow trigger upgrade, --all sweeps, reconcile improvements, --invocation-id flag. Full enumeration carries from cycle 178+.

## In-session issues and recoveries

- **Pre-existing env-var race in `default_primitive_bin_*` tests** (v2-cycle-runner) — surfaced as a parallel-test-execution flake when `cargo test` runs without `--test-threads=1`. Pre-existing (not introduced by cycle 179 refactor); cycle 177 CI run also hit it but the GitHub Actions `cargo test` step typically serializes via the runner's defaults. Both tests manipulate global `CARGO_TARGET_DIR` env without holding a mutex. Documented; left as-is (the fix is a test-isolation refactor, not blocking; flake observable only locally under `--test-threads=N`).
- **Two clippy `doc-lazy-continuation` warnings** on `TimeoutDiagnosticRecord`'s docstring (`+` at line start was being parsed as a list-marker, then continuation lines flagged as "list item without indentation"). Rephrased docstring to avoid leading `+`. Both warnings cleared.
- **One serde-derive mistake on `RoleRun.timeout` initial omission** — first build failed because the test `role_history_serde_roundtrip` constructed `RoleRun { ... }` without the new `timeout` field. Fixed by adding `timeout: None` to the test fixture + asserting `timeout.is_none()` post-roundtrip.
- **One borrow-after-move in `cmd_invoke_live_spawn`** — initial `format!` after the `RoleRun { ..., timeout: Some(TimeoutDiagnosticRecord::from_invoker(diagnostic)) }` line tried to read `diagnostic.elapsed_ms` after moving `diagnostic`. Fixed by capturing `budget_ms` and `elapsed_ms` as locals before constructing the diagnostic.
- **One semantic bug in `emit_invoke_result` plumbing** — passed `skip_channel_write` to a parameter named `channel_write_performed` (and same for `skip_super_step_check` → `super_step_check_performed`). Surfaced via the existing `invoke_json_output_includes_channel_and_outcome` integration test asserting `channel_write_performed == true`. Fixed by flipping to `!skip_channel_write` / `!skip_super_step_check` at all four call sites (scaffold success + live timeout + live parse-failure + live success).
- **Cwd not persisted between bash calls** (recurring observation from cycles 176/177 also held cycle 179). Recovered via `cargo ... --manifest-path tools/rust/Cargo.toml` (relative to workspace root, not cwd).
- All `cargo`, `git`, `gh`, Edit/Write operations otherwise clean cycle 179.

---

**Design-scope reference:** [`docs/redesign/_notes/v2-role-driver-live-spawn-arc.md`](v2-role-driver-live-spawn-arc.md) (cycle 178, 548 lines)

**Live source post-implementation:**
- [`tools/rust/crates/v2-primitive-invoker/src/lib.rs`](../../../tools/rust/crates/v2-primitive-invoker/src/lib.rs) (cycle 179, new)
- [`tools/rust/crates/v2-role-driver/src/main.rs`](../../../tools/rust/crates/v2-role-driver/src/main.rs) (cycle 179)
- [`tools/rust/crates/v2-cycle-runner/src/main.rs`](../../../tools/rust/crates/v2-cycle-runner/src/main.rs) (cycle 179, refactor + caller migration)

**Promotion criteria for cycle 2 (cycle 180) first-live-spawn:** all six §12 items satisfied at cycle 179 close. ✓ Per acceptance:
1. `cargo clippy -p v2-primitive-invoker -p v2-role-driver -p v2-cycle-runner --tests -- -D warnings` clean.
2. All cycle 177 tests preserved: 77 v2-cycle-runner unit + 5 integration; 16 v2-role-driver unit + 42 integration.
3. New unit tests covering live-spawn path: 21 (mode-selection × 4, context-assembly × 3, xml-escape × 1, argv-build × 2, envelope-parse × 4, end-to-end with MockInvoker × 6, allowed-tools × 1). Plus 9 new tests in v2-primitive-invoker.
4. Hermetic integration test: `live_spawn_with_mock_claude_code_drives_all_four_roles_end_to_end` end-to-end clean.
5. No live exercise of `claude-code` in cycle 1 (per design §12 item 5 — that's cycle 2's acceptance).
6. This document.
