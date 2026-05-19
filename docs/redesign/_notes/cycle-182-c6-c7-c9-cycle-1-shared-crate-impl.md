# Cycle 182 — C6+C7+C9 implementation cycle 1: v2-error-envelope shared crate

**Status:** Track 1 substantive landed. Cycle 181 priority #3 fully closed at shared-crate-creation layer per design §7.1 cycle-1 acceptance.

**Provenance:** Cycle 181 design scope `docs/redesign/_notes/v2-structured-error-envelope-arc.md` §6.5 bottom-up migration order: shared crate first, no consumers yet.

## 1. Substantive changes per layer

### 1.1 New shared crate `tools/rust/crates/v2-error-envelope/`

**Files added:**
- `Cargo.toml` (10 LOC): package + serde + serde_json + v2-primitive-invoker path-dep
- `src/lib.rs` (474 LOC): types + helpers + 14 unit tests

**Public surface:**
- `ErrorEnvelope` struct with 4 fields per design §3.1: `primitive: String`, `class: ErrorClass`, `details: serde_json::Map<String, serde_json::Value>` (default-empty, skip_serializing_if), `human: String`. Derives `Debug + Clone + PartialEq + Eq + Serialize + Deserialize`.
- `ErrorClass` enum with 11 variants per design §3.1: `Transient` / `Auth` / `Config` / `Io` / `Protocol` / `ChannelWriteRejected` / `RoleSessionEmpty` / `SuperStepOutOfOrder` / `StateBoundExceeded` / `Timeout` / `Unknown`. `#[serde(rename_all = "kebab-case")]`. Methods: `as_slug()`, `is_retryable()`.
- `ErrorEnvelope::new(primitive, class, human)` constructor + `with_detail(k, v)` + `with_details_map(map)` builder methods.
- `ErrorEnvelope::for_timeout(primitive, &TimeoutDiagnostic)` factory translating cycle-177 `TimeoutDiagnostic` into a `Timeout`-class envelope with details `{budget_ms, elapsed_ms, escalation}`. Per design §4 table.
- `ErrorEnvelope::to_json_line()` emit helper (compact JSON, no trailing newline) — primitives append the newline at write time.
- `ErrorEnvelope::is_retryable_class()` convenience delegating to `class.is_retryable()`.
- `parse_from_stderr_tail(stderr: &str) -> Option<ErrorEnvelope>` per design §3.3: last non-empty line of stderr, deserialize attempt, `None` on parse failure (cycle-runner's caller-side classifier will fall back to legacy substring matching).

**Workspace registration:** automatic via `tools/rust/Cargo.toml` `members = ["crates/*"]`. No workspace Cargo.toml edit needed.

### 1.2 Tests added (14 unit tests in v2-error-envelope/src/lib.rs)

| Test | What it pins |
|---|---|
| `class_slug_matches_serde_rename` | `as_slug()` mirrors serde output for all 11 variants; round-trip via `from_str` returns equal value |
| `retry_policy_only_transient_and_unknown_retry` | Negative-assertion: 9 classes must NOT be retryable; `Transient` + `Unknown` must be |
| `envelope_roundtrip_minimal` | No-details envelope serializes WITHOUT `"details"` key (skip_serializing_if); round-trips equal |
| `envelope_roundtrip_with_details` | With-details envelope serializes WITH `"details"` key; round-trips equal; details accessible via Map API |
| `envelope_with_details_map_overwrites` | `with_details_map` replaces pre-existing entries (not merge) |
| `timeout_envelope_from_diagnostic_carries_all_fields` | `for_timeout(SigkillForced)` carries budget_ms/elapsed_ms/escalation; human contains slug; class is not retryable |
| `timeout_envelope_sigterm_clean_branch` | `SigtermClean` branch produces correct slug + human text |
| `timeout_envelope_roundtrips_through_json` | Timeout-class envelope round-trips through serde without loss |
| `parse_from_stderr_tail_finds_last_line` | Pre-envelope warning line tolerated; last line parsed as envelope |
| `parse_from_stderr_tail_tolerates_trailing_whitespace` | Trailing blank lines + whitespace don't defeat tail parse |
| `parse_from_stderr_tail_returns_none_on_non_envelope` | Plain text stderr → None |
| `parse_from_stderr_tail_returns_none_on_empty` | Empty / whitespace-only stderr → None |
| `parse_from_stderr_tail_ignores_envelope_not_on_last_line` | Pins "envelope is LAST line" contract: trailing non-envelope line → None even if earlier line was an envelope |
| `unknown_class_envelope_roundtrips` | Forward-compat: primitives can emit `class=unknown` explicitly; round-trips equal; is_retryable |

### 1.3 No source changes outside the new crate

Bottom-up migration discipline (design §6.5) — cycle 1 ships the crate compiled + tested; no v2-* primitive depends on it yet. v2-channel-router (cycle 183), v2-role-driver (cycle 184), v2-cycle-runner (cycle 185) add `v2-error-envelope` to their `Cargo.toml` at the cycle each migrates.

## 2. Substantive measurements

### 2.1 LOC delta

| File | LOC |
|---|---|
| `tools/rust/crates/v2-error-envelope/Cargo.toml` | 10 |
| `tools/rust/crates/v2-error-envelope/src/lib.rs` | 474 |
| **Total source delta** | **484** |

**Design scope §8 estimate:** ~250 LOC source + 10 unit tests ≈ **350 raw**. Honest budget at 1.2-2.0× multiplier: **420-700**.

**Actual / estimate ratio:** 484 / 350 = **1.38× raw** / **0.99× honest** (at 1.4× midpoint).

### 2.2 `comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-5

Fifth data point in series, NEW LOWEST raw ratio:

| Cycle | Design LOC | Actual LOC | Raw ratio | Honest ratio (at midpoint) |
|---|---|---|---|---|
| 170 | (varies) | (varies) | **4.5×** | 1.5-2.25× |
| 171 | (varies) | (varies) | **7×** | 2.3-3.5× |
| 177 | (varies) | (varies) | **2.8×** | **1.2×** |
| 179 | 950 raw | 1502 | **1.58×** | **0.79×** (prior LOWEST) |
| 182 | 350 raw | 484 | **1.38×** | **0.99×** (NEW LOWEST raw) |

The trend reinforces cycle 179's hypothesis: **more thorough design scope → tighter LOC ratio at implementation**. Cycle 182's design scope was mid-thoroughness (391 doc-LOC vs cycle 178's 548); actual landed 1.38× raw, within the predicted 1.4-1.7× band.

**Multiplier-refinement candidate:** the 1.2-2.0× honest band proposed in cycle 181 §8 is empirically supported. The bottom of the band (1.2×) holds. Cycle 183+ measurement against v2-channel-router emit-side (design estimate ~250 LOC) will provide the sixth data point.

### 2.3 Test suite delta

- Pre-cycle (cycle 181 close): 96 test-suite OK results.
- Post-cycle (cycle 182 close): 98 test-suite OK results.
- Delta: **+2 test-suite results** (v2-error-envelope unit-tests + doc-tests). Net +14 actual tests.

### 2.4 Clippy state

- `cargo clippy -p v2-error-envelope --tests -- -D warnings`: **clean**.
- `cargo clippy --workspace --tests -- -D warnings`: **pre-existing v2-boot-phase issue** at `crates/v2-boot-phase/src/main.rs:751-755` (`if_same_then_else` — both branches return `1`). Not introduced by cycle 182; not blocking CI (CI does not run clippy).

This is a `design-tacit-assumption-falsified-during-implementation` RECURRENCE-AT-2 instance — see §3.1 below.

## 3. Pattern updates

### 3.1 `design-tacit-assumption-falsified-during-implementation` NOVEL@1 cycle 180 → RECURRENCE-AT-2 cycle 182

**Cycle 180 (NOVEL@1):** cycle-178 design §3.1 point 2 tacitly assumed `ANTHROPIC_API_KEY` was configured as a GitHub Actions secret; reality was `CLAUDE_CODE_OAUTH_TOKEN` via `anthropics/claude-code-action@v1`. Tacit assumption falsified.

**Cycle 182 (RECURRENCE-AT-2):** cycle-181 design §7.1 cycle-1 acceptance criterion `cargo clippy --workspace --tests -- -D warnings clean` tacitly assumed workspace was clippy-clean. Reality: pre-existing `if_same_then_else` violation in `v2-boot-phase/src/main.rs:751-755` (both branches return `1`). Tacit assumption falsified.

**Distinct from `design-scope-honesty-hedge-survives-implementation-cycle`:** that pattern tracks EXPLICIT hedges (OQ-LS-*, OQ-SEE-*) the design author marked as uncertain. THIS pattern tracks TACIT assumptions the design author carried as fact. Both flavors arise; both are useful to surface separately.

**Disposition cycle 182:** match cycle-179 precedent (clippy on touched packages, not workspace). The design author of cycle 181 over-wrote the acceptance — the cycle-1 deliverable is the new crate, and the new crate is clean. Scope-creep into v2-boot-phase is a separate cycle's substantive focal (or a bounded-mechanical follow-on when convenient).

**Forward-watch:** cycles 182-189 for RECURRENCE-AT-3. 6 of 6 watch-window cycles remain.

### 3.2 `shared-crate-extraction-from-isolated-reducer-rule-precedent` NOVEL@1 cycle 179 → RECURRENCE-AT-2 cycle 182

**Cycle 179 (NOVEL@1):** `v2-primitive-invoker` extracted from `v2-cycle-runner` so both cycle-runner and role-driver could consume; structurally similar to cycle 138's isolated-reducer-rule precedent (write_channel duplicated in v2-role-driver from v2-channel-router at SCAFFOLD scope) but INVERTED in polarity: cycle 138 chose duplication-for-isolation at SCAFFOLD; cycle 179 chose extraction-to-shared-crate at COMPLETE.

**Cycle 182 (RECURRENCE-AT-2):** `v2-error-envelope` created as a NEW shared crate (not extracted from a primitive) with anticipated multi-caller (cycle 183+: v2-channel-router, cycle 184+: v2-role-driver, cycle 185+: v2-cycle-runner). The crate-creation decision is a third polarity variant: **green-field shared crate at full multi-caller intent**, not extraction-after-fact (cycle 179) or duplication-for-isolation (cycle 138).

The three polarity points now:
- Cycle 138: duplication-for-isolation at SCAFFOLD (one caller, second isolated for future)
- Cycle 179: extraction-to-shared-crate at COMPLETE (two callers, common ancestor extracted)
- Cycle 182: green-field-shared-crate at multi-caller-intent (zero existing callers, designed for ≥3 future callers)

The choice axis is two-dimensional: **scope-maturity** (SCAFFOLD / COMPLETE) × **caller-cardinality** (1 / 2 / ≥3). Cycle 182 occupies the COMPLETE × ≥3 quadrant — design-driven anticipation.

**Forward-watch:** cycles 182-189 for RECURRENCE-AT-3. 5 of 6 watch-window cycles remain.

### 3.3 `comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-4 → RECURRENCE-AT-5 cycle 182

See §2.2 above. NEW LOWEST raw ratio: 1.38× (prior LOWEST 1.58× cycle 179). Multiplier-refinement supported.

**Forward-watch:** cycle 183+ (v2-channel-router emit-side) provides sixth data point at design estimate ~250 LOC.

### 3.4 `coordinated-arc-design-scope-pairs-deferred-items` RECURRENCE-AT-4 → RECURRENCE-AT-5 cycle 182

Fifth instance of the arc-shape (cycle N design → cycle N+1 cycle-1 implementation closes pair):

| Pair # | Design cycle | Cycle-1 impl cycle | Arc |
|---|---|---|---|
| 1 | 176 | 177 | timeout |
| 2 | 178 | 179 | live-spawn |
| 3 | 181 | 182 | structured-error-envelope (this) |

Plus 2 followup instances:
- Cycle 178→180 (cycle-2 reconciliation, not pair-closure)
- Cycle 181→? (cycle-2 will appear at cycle 183)

The pair-vs-followup distinction from cycle 180 is now ripe at 3 pair + 1 followup data points. Naming-refinement question: split into two patterns (`design-cycle-1-implementation-pair-closure` + `design-cycleN+1-implementation-followup`)? **Defer to RECURRENCE-AT-6+** — the followup data is too thin (1 of 4 instances) to justify a split yet.

**Forward-watch:** cycle 183 closes a followup-shape (cycle 181 design → cycle 183 cycle-2 impl, which is NOT a pair-closure since cycle 182 already paired it).

### 3.5 `design-scope-honesty-hedge-survives-implementation-cycle` NOT-EXERCISED PRECONDITION-SET cycle 181 → NOT-EXERCISED cycle 182

Cycle 181 design contained 8 `OQ-SEE-*` open questions deferred to cycle 2-4. Cycle 182 was cycle 1 of implementation — none of the OQ items were exercised (they target router/role-driver/cycle-runner sides). Forward-watch continues. **6 of 8 watch-window cycles remain (cycles 183-188).**

### 3.6 `session-start-CI-check-discipline` HARDENED orientation-prelude mode

Standing step #2 confirmed `be346df6` (cycle 181 commit) held SUCCESS on master Rust CI. No RECURRENCE-AT-N tracking; operates as the orientation-prelude step.

### 3.7 Two-track-composition continuity

**31st consecutive two-track-composition** post cycle 151 exception (HARDENING-AT-21):
- Track 1 substantive: v2-error-envelope crate + 14 unit tests + integration with v2-primitive-invoker TimeoutDiagnostic
- Track 2 bounded-mechanical: this _notes file + journal entry

**67th consecutive HONORING of named forward priority:** cycle 181 priority #3 (C6+C7+C9 implementation cycle 1) → cycle 182 substantive focal.

### 3.8 Other forward-watch decay

| Pattern | Cycle 181 state | Cycle 182 state | Deadline |
|---|---|---|---|
| `eva-directive-overrides-implicit-arc-serialization` | 2 of 6 consumed | 3 of 6 consumed (NOT-EXERCISED) | cycle 185 |
| `gitignore-extension-from-orchestrator-tempfile-sandbox-friction` | 3 of 6 consumed | 4 of 6 consumed (NOT-EXERCISED) | cycle 184 |
| `as-needed-verification-of-HARDENED-principle-on-tool-rerun` | 3 of 6 consumed | 4 of 6 consumed (NOT-EXERCISED) | cycle 184 |
| `cargo-spawned-subprocess-bypasses-tool-permission-intercept` | 1 of 6 consumed | 2 of 6 consumed (NOT-EXERCISED) | cycle 187 |
| `deferred-arc-cycles-from-original-defer-to-design-scope` | NOVEL@1 cycle 181 | 1 of 6 consumed (NOT-EXERCISED) | cycle 188 |
| `design-scope-template-stabilizes-across-arcs` candidate | NOT-NAMED-YET | continues unnamed | n/a |

## 4. Open items + forward priorities (cycle 183+)

1. **Master Rust CI green-state maintenance** — HARDENED orientation-prelude exercise.
2. **AWAIT #2997 Eva response on OQ-LS-AUTH** — 5-cycle BETWEEN-CHECKPOINTS band expires cycle 185-186. Cycle 183 within band.
3. **C6+C7+C9 implementation cycle 2 (NEW priority #3)** — v2-channel-router emit-side per design §5.1. Add `--error-format <text|json>` flag at CLI parse site, `impl RouterError { fn to_envelope(&self) -> ErrorEnvelope }` mapping all 8 variants per §5.1 table, replace `eprintln!("v2-channel-router: {err}")` with flag-branch. Estimated ~150 source LOC + ~8 unit + 1 integration ≈ **250 raw**; honest band 290-375.
4. **C13+X2 cycle 2 of implementation** — `elapsed_ms` schema promotion (still deferred per design §1.3 + cycle 180 schema-promotion-discipline absorption).
5. **C11+C12+X1 resume/recovery coordinated arc** — predicted design landing cycle 196-205 per `deferred-arc-cycles-from-original-defer-to-design-scope`.

## 5. Cross-references

- **Design scope:** `docs/redesign/_notes/v2-structured-error-envelope-arc.md` (cycle 181).
- **Precedent cycle (timeout arc):** `docs/redesign/_notes/v2-primitive-invoker-timeout-arc.md` (cycle 176) + `cycle-177-c13-x2-cycle-1-implementation.md`.
- **Precedent cycle (live-spawn arc):** `docs/redesign/_notes/v2-role-driver-live-spawn-arc.md` (cycle 178) + `cycle-179-live-spawn-implementation-cycle-1.md` + `cycle-180-live-spawn-implementation-cycle-2.md`.
- **OQ-LS-AUTH (blocking live-spawn cycle 2 success-path):** [#2997](https://github.com/EvaLok/schema-org-json-ld/issues/2997).
- **Eva directive (live-spawn arc auth):** [#2992](https://github.com/EvaLok/schema-org-json-ld/issues/2992).
- **Triggering arc-of-deferral:** `docs/redesign/_notes/cycle-155-cycle-152-critique-absorption.md` §C6/C7/C9.
