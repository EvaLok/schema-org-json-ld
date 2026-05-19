# Cycle 184 — C6+C7+C9 implementation cycle 3: v2-role-driver emit-side

## Scope

Per design `_notes/v2-structured-error-envelope-arc.md` §5.2 and cycle-183
forward priority #3: wire `--error-format <text|json>` into v2-role-driver,
implement `impl DriverError { fn to_envelope() -> ErrorEnvelope }` for every
variant, branch stderr emission on the flag, and add the cycle-180
OQ-LS-AUTH visibility extension (auth-class envelope when claude-code
returns `is_error: true` + `result == "Not logged in"`).

## Variant-count verification (cycle 183 directive)

Cycle 183 flagged that design §5.2 enumerated **11 `DriverError` variants**
while the actual enum had more. Verification at cycle 184 session start:

```
$ grep -c "^    [A-Z][a-zA-Z]*" tools/rust/crates/v2-role-driver/src/main.rs
   # within `enum DriverError { ... }` block
```

**Actual pre-cycle-184 count: 14.** Design under-counted by 3:

| Missing variant            | Class assigned cycle 184 | Why design missed it |
|----------------------------|--------------------------|----------------------|
| `Io(io::Error)`            | `io`                     | Generic `?` propagation catch-all; design enumerated only path-bearing io variants. |
| `Json(String)`             | `protocol`               | Generic `?` propagation catch-all for `serde_json::Error`; design enumerated only `LiveSpawnEnvelopeParseFailure` and `InvalidSessionOutput`. |
| `NotInitialized(PathBuf)`  | `config`                 | Pre-init-error in `read_role_history`; design omitted because §5.2 focused on `Invoke` subcommand variants. |

Cycle 184 ALSO adds a new variant for the OQ-LS-AUTH extension:

| New variant                     | Class    | Rationale |
|---------------------------------|----------|-----------|
| `LiveSpawnAuthFailure { ... }`  | `auth`   | Detects claude-code `is_error: true` + `result == "Not logged in"`; upgrades from generic `Protocol` (the catch-all `LiveSpawnEnvelopeParseFailure` mapping) to typed `Auth` class. |

**Post-cycle-184 variant count: 15.** All 15 covered by `to_envelope`,
each pinned by a dedicated unit test.

Pattern `design-tacit-assumption-falsified-during-implementation`
advances to RECURRENCE-AT-4 (cycles 180 / 182 / 183 / 184). Sub-shape at
cycle 184: design-variant-table-undercounts-actual-enum. Distinct from
cycle 183's narrative-vs-table-mismatch (which was an internal §5.1
inconsistency) — cycle 184 is table-vs-source mismatch.

## Implementation summary

**1. `Cargo.toml`** (+2 lines):
- Added `v2-error-envelope = { path = "../v2-error-envelope" }` to both
  `[dependencies]` and `[dev-dependencies]`, mirroring cycle-183 router.

**2. `src/main.rs`** (+617 / −19 lines):

- Imported `v2_error_envelope::{ErrorClass, ErrorEnvelope}`.
- Added `ErrorFormat::{Text, Json}` value-enum + global `--error-format`
  flag on `Args`, default `Text`.
- Added `DriverError::LiveSpawnAuthFailure { subsystem, upstream_result }`
  variant + Display arm.
- Refactored `parse_claude_code_envelope` to return `Result<String,
  ClaudeCodeEnvelopeError>` (was `Result<String, String>`).
  `ClaudeCodeEnvelopeError` has 3 variants: `NotJson(String)`,
  `IsError { subtype, result: Option<String> }`, `MissingResult`. The
  `IsError` variant carries the `result` field so callers can detect the
  auth-shape pattern; the pre-cycle-184 parser discarded `result` in this
  case, which is what made OQ-LS-AUTH visibility unreachable.
- Added `classify_envelope_error(env_err) -> DriverError`: routes the
  is_error+result=="Not logged in" case to `LiveSpawnAuthFailure`;
  everything else falls through to the legacy
  `LiveSpawnEnvelopeParseFailure`. This keeps the routing localized in
  one function (~18 LOC) rather than inline in `cmd_invoke_live_spawn`.
- Wired `classify_envelope_error` into `cmd_invoke_live_spawn`'s
  envelope-parse error arm. The `notes` field now interpolates
  `driver_err` (its Display impl) instead of the raw envelope-error
  string, so role-history records carry the typed-error context.
- Added `impl DriverError { fn to_envelope() -> ErrorEnvelope }`. Maps
  all 15 variants. Detail-key schema per design §4 table (path / op for
  Io, parse_error / observed_shape for Protocol, key / hint for Config,
  subsystem / upstream_result for Auth, etc.).
- Branched `main()` stderr emission: `ErrorFormat::Text` keeps
  `eprintln!("v2-role-driver: {e}")`; `ErrorFormat::Json` emits
  `e.to_envelope().to_json_line()`.

**3. Tests** (+~400 unit lines + 276 integration lines):

- 17 new unit tests in `mod tests`:
  - 1 envelope-mapping test per `DriverError` variant (15 tests)
  - `classify_envelope_error_routes_not_logged_in_to_auth_failure` —
    pins the OQ-LS-AUTH routing
  - `classify_envelope_error_routes_other_is_error_to_parse_failure` —
    pins that non-auth `is_error` cases still go to Protocol class
  - `parse_claude_code_envelope_surfaces_result_field_on_is_error` —
    pins that the parser refactor doesn't drop `result` when is_error
    is true (was the cycle-180 OQ-LS-AUTH visibility gap)
  - `envelope_serializes_to_compact_single_line_json` — §3.2 emission
    protocol invariant
  - `envelope_retry_policy_for_role_driver_classes_is_all_non_retryable`
    — including explicit Auth-class non-retryable (cycle-180
    OQ-LS-AUTH: retrying without operator action wastes budget)
  - Plus 3 updated tests for the refactored `parse_claude_code_envelope`
    error type (was String, now structured enum)

- 7 new integration tests in `tests/integration.rs`:
  - `error_format_text_default_preserves_legacy_v2_role_driver_prefix` —
    backward-compat for non-migrated callers
  - One per reachable-via-CLI error class: missing-invoke-mode,
    conflicting-invoke-modes, session-output-missing,
    not-initialized, super-step-mismatch, invalid-session-output (5
    tests covering the Config, Io, SuperStepOutOfOrder, Protocol
    classes via the CLI surface)
  - `error_format_json_emits_single_line_envelope_on_stderr_tail` —
    §3.2 single-line invariant verified at the actual CLI surface
    (not just the unit-test mock)

Live-spawn-only variants (`LiveSpawnAuthFailure`, `LiveSpawnInvocationIo`,
`LiveSpawnEnvelopeParseFailure`) are covered by unit tests using mocked
`PrimitiveInvoker`; CLI-surface integration tests would require a real
claude-code subprocess so they're scoped to unit tests.

## Verification

- `cargo build -p v2-role-driver` — GREEN (1.24s).
- `cargo test -p v2-role-driver` — GREEN at 57 unit (40 pre-cycle-184 +
  17 new envelope) + 50 integration (43 pre-cycle-184 + 7 new envelope)
  = 107 tests in v2-role-driver alone.
- `cargo clippy -p v2-role-driver --tests -- -D warnings` — clean.
- `cargo clippy -p v2-cycle-runner --tests -- -D warnings` — clean
  (caller-side dependency check; cycle-runner currently still uses
  text-format default, so no regression).
- `cargo clippy -p v2-error-envelope --tests -- -D warnings` — clean.
- `cargo test --workspace` — GREEN (no failures; pattern-by-pattern test
  counts unchanged or grew, no regression).
- Workspace clippy NOT measured this cycle. Cycle 183's `_notes`
  confirmed 5 pre-existing lints exist in v2-boot-phase + v2-wiki-search.
  No new lints introduced by cycle 184 (the per-crate clippy gates
  passed); the §7.2 overall-arc acceptance criterion handling is still
  deferred per cycle 183 _notes.

## LOC ratio

Design §8 estimated cycle 3 at ~500 raw / 600-750 honest at refined
1.2-1.5× multiplier per cycle-182/183 measurements.

Actual delta:
- `Cargo.toml`: +2
- `Cargo.lock`: +1
- `src/main.rs`: +617 / −19 = net +598 (split ~250 source / ~350 unit-test)
- `tests/integration.rs`: +276
- **Raw delta: 877 added / 19 removed.**

Multiplier:
- 877 / 500 = **1.75× raw** (above the 1.2-1.5× refined band)
- 875 honest (excluding Cargo.lock auto-update) / 675 midpoint estimate =
  **1.30× honest** (within the 1.2-1.5× refined band)

7th data point for `comprehensive-test-suite-exceeds-design-scope-LOC-estimate`:

| Cycle | Raw mult | Honest mult | Notes                                |
|-------|----------|-------------|--------------------------------------|
| 170   | 4.5×     | —           | thin design                          |
| 171   | 7×       | —           | thin design                          |
| 177   | 2.8×     | —           | medium design                        |
| 179   | 1.58×    | 0.79×       | thorough design                      |
| 182   | 1.38×    | 0.99×       | thorough design (shared-crate cycle) |
| 183   | 1.36×    | 1.02×       | thorough design (router emit-side)   |
| 184   | 1.75×    | 1.30×       | thorough design (role-driver, 15 variants vs 6 router) |

Cycle 184 sits slightly above the cycle 182/183 cluster on both axes.
The likely driver is variant count (15 vs 6) scaling test code more than
proportionally — each variant gets a dedicated unit test of ~15-25 lines.
Pattern stabilizes at 1.3-1.7× raw / 0.8-1.3× honest for thorough scopes,
strengthening the cycle-179 hypothesis "thorough scope → tighter ratio
than thin scope (which was 2.8-7×)".

## Pattern updates

- **`design-tacit-assumption-falsified-during-implementation`**
  RECURRENCE-AT-3 → **RECURRENCE-AT-4** cycle 184 (variant-count
  undercount: 11 design vs 14 pre-cycle-184 actual vs 15 post-cycle-184).
  Sub-shape: design-table-undercounts-actual-enum (distinct from cycle
  183's narrative-vs-table mismatch sub-shape). The pattern's
  prescribed mitigation — "verify variant count before mapping" — was
  honored: I counted via grep at session start before writing the
  mapping. Without that step, my mapping would have silently missed
  `Io`/`Json`/`NotInitialized`, leaving 3 variants unmapped → either
  compile-fail or default-unreachable arm → cycle 5+ surfacing.
  Verification at design-vs-source level should become a
  HARDENING-AT-5 candidate at next recurrence.

- **`comprehensive-test-suite-exceeds-design-scope-LOC-estimate`**
  RECURRENCE-AT-6 → **RECURRENCE-AT-7** cycle 184 (1.75× raw / 1.30×
  honest at midpoint). Table updated; cluster shape "thorough → 1.3-1.7×
  raw / 0.8-1.3× honest" stable across 4 data points (179, 182, 183,
  184). Pattern not ready for HARDENING; needs ~3 more thorough-scope
  data points to confirm cluster stability.

- **`coordinated-arc-design-scope-pairs-deferred-items`**
  RECURRENCE-AT-6 → **RECURRENCE-AT-7** cycle 184 (cycle 4 of arc still
  deferred; cycle-runner caller-side migration named in cycle-184
  forward priority #3).

- **`design-scope-honesty-hedge-survives-implementation-cycle`**
  Cycle 184 EXERCISES OQ-SEE-3 partially: design §6.3 said "details is
  `Map<String, Value>` rather than typed enum-per-class so primitives
  can extend without coordinating release across consumers." Cycle 184's
  cycle-180 OQ-LS-AUTH extension is exactly this scenario: role-driver
  added new `subsystem` / `upstream_result` keys to the Auth class
  details without touching v2-error-envelope's `ErrorClass::Auth`
  definition — the hedge's "primitives can extend" promise is honored
  in practice. RECURRENCE-AT-2 for this hedge (cycle 183 honored
  OQ-SEE-2; cycle 184 honors OQ-SEE-3). Other OQ-SEE-* items remain for
  cycles 185+.

- **`session-start-CI-check-discipline`** HARDENED operates in
  orientation-prelude mode cycle 184 (CI green on `2264d4ba`, run
  `26088294563` confirmed at session start).

- **`eva-directive-overrides-implicit-arc-serialization`** NOT-EXERCISED
  cycle 184 (3 of 6 consumed; DEADLINE cycle 185).

- **`gitignore-extension-from-orchestrator-tempfile-sandbox-friction`**
  NOT-EXERCISED cycle 184 — no tempfile scratch this cycle (RESOLVED-VIA-
  GITIGNORE-EXTENSION at cycle 183; 6 of 6 consumed; DEADLINE CONSUMED).
  Pattern can be archived to "settled" status if no recurrence by
  cycle 188.

- **`as-needed-verification-of-HARDENED-principle-on-tool-rerun`**
  NOT-EXERCISED cycle 184 (4 of 6 consumed; DEADLINE cycle 185).

- **`design-tacit-assumption-falsified-during-implementation`** (see
  above; RECURRENCE-AT-4).

- **`cargo-spawned-subprocess-bypasses-tool-permission-intercept`**
  NOT-EXERCISED cycle 184 (2 of 6 consumed; DEADLINE cycle 187).

- **`shared-crate-extraction-from-isolated-reducer-rule-precedent`**
  NOT-EXERCISED cycle 184 (4 of 6 consumed; DEADLINE cycle 186; cycle
  184 was a consumer of the shared crate via path-dep, not an extractor
  — pattern explicitly tracks extraction events, not consumption).

## Open items / observations

- **Variant-count discrepancy demands a design-correction note in
  `_notes/v2-structured-error-envelope-arc.md` §5.2.** Either: (a) edit
  the arc design to reflect 15 variants with the missing-then-added
  rows, (b) add a "post-cycle-184 amendment" footnote, or (c) leave as-
  is since cycle 184 _notes documents the gap. Defer disposition to
  cycle 185 — option (c) costs less rework but reads worse for someone
  picking up the arc cold; option (a) overwrites historical design
  context. Probably (b) is the right balance; will decide cycle 185.

- **`LiveSpawnAuthFailure` is currently a literal-string match on `result
  == "Not logged in"`.** This is fragile if claude-code's auth-error
  result text ever changes (e.g., "Not authenticated", "Session
  expired"). The design §5.2 wording was specific to "Not logged in"
  as the cycle-180 observed string. Possible follow-on: extend
  `classify_envelope_error` to recognize a small set of auth-shape
  result strings, or to consult `subtype == "auth-error"` as the
  primary auth signal (cycle-180 envelope had `subtype: "auth-error"`
  in addition to result). Defer to cycle 5+ of arc; not in cycle 4
  caller-side scope.

- **No regression in v2-cycle-runner.** cycle-runner currently spawns
  v2-role-driver WITHOUT the new `--error-format` flag, so it defaults
  to `text` and existing behavior is preserved. Cycle 4 (caller-side
  migration) is where cycle-runner will switch to `--error-format=json`.

- **§7.2 overall-arc acceptance criterion** still unresolved. Cycle 183
  _notes raised this; cycle 184 does not change the disposition. Three
  options (extend ci-rust.yml, narrow §7.2 to per-crate, opportunistic
  cleanup) remain on the table. Defer to cycle 185 close.

## Forward priorities (cycle 185+)

1. **Master Rust CI green-state maintenance** — HARDENED orientation-
   prelude exercise.
2. **AWAIT #2997 (OQ-LS-AUTH)** — Eva's secret-add path still no
   response; 5-cycle BETWEEN-CHECKPOINTS band expires cycle 185-186 so
   cycle 185 is the natural close. If still no response at cycle 185
   session start, cycle 185 will need to decide whether to (a) extend
   the band per a new 5-cycle window, (b) close the question as no-
   longer-blocking (since cycle 184 added the visibility-extension
   that makes auth failures observable even without the secret), or
   (c) escalate via a follow-up question.
3. **C6+C7+C9 implementation cycle 4 (NEW priority #3)** — v2-cycle-
   runner caller-side migration per design §5.3. Three sub-changes:
   (a) update primitive-invocation sites to pass `--error-format=json`,
   (b) rewrite `classify_failure` to envelope-parse-first + legacy-
   substring fallback, (c) extend `FailureClass` with Auth/Config/Io/
   Protocol/Unknown variants + halt-decision wiring. Naturally absorbs
   the C13+X2 `elapsed_ms` schema promotion (when v2-cycle-runner reads
   the Timeout envelope details). Estimate ~300 raw LOC / 360-450
   honest at refined 1.2-1.5× multiplier per cycle 182/183/184 cluster.
4. **C11+C12+X1 resume/recovery coordinated arc** — predicted design
   landing cycle 196-205 per `deferred-arc-cycles-from-original-defer-
   to-design-scope` pattern.
5. **Workspace clippy CI extension** — defer to RECURRENCE-AT-3+ when
   §7.2 acceptance becomes actionable per cycle 183 _notes.
6. **Design-scope amendment for §5.2 variant count** — decide
   disposition cycle 185 (option a/b/c above).
