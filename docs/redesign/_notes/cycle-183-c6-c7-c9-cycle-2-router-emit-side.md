# Cycle 183 — C6+C7+C9 implementation cycle 2: v2-channel-router emit-side

**Date:** 2026-05-19
**Track:** Track 1 substantive (cycle 182+ priority #3 FULLY CLOSED at router-emit-side wiring layer)
**Arc reference:** `_notes/v2-structured-error-envelope-arc.md` §5.1 + §7.1 cycle-2 acceptance
**Prior cycle in arc:** Cycle 182 (`_notes/cycle-182-c6-c7-c9-cycle-1-shared-crate-impl.md`)

## Summary

Wired v2-channel-router as the first consumer of cycle-182's `v2-error-envelope`
shared crate. Behind a default-off `--error-format <text|json>` flag the binary
now emits a single-line `ErrorEnvelope` JSON record on stderr instead of the
legacy `v2-channel-router: <message>` text. Text mode preserves the existing
emission verbatim for backward-compat. All 6 `RouterError` variants are mapped
to envelope classes per design §5.1; 8 new unit tests + 3 new integration tests
exercise the mapping end-to-end including the design §6.2 / OQ-SEE-2
"pre-envelope warning lines tolerated" case via lenient-mode reducer-violation.

## Delta

| File | Change |
|---|---|
| `tools/rust/crates/v2-channel-router/Cargo.toml` | +2 LOC: `v2-error-envelope = { path = "../v2-error-envelope" }` in `[dependencies]` and `[dev-dependencies]` (dev for integration tests calling `parse_from_stderr_tail`) |
| `tools/rust/crates/v2-channel-router/src/main.rs` | +194 / -1: new `use` statement, new `ErrorFormat` value-enum, new `error_format` global CLI flag with docstring referencing design §3.2, new `impl RouterError { fn to_envelope(&self) -> ErrorEnvelope }`, branched `eprintln!` in `main()`, 8 new unit tests under existing `tests` mod |
| `tools/rust/crates/v2-channel-router/tests/integration.rs` | +144: 3 new integration tests `error_format_json_emits_envelope_for_not_initialized_read` / `error_format_text_still_emits_legacy_line_by_default` / `error_format_json_with_lenient_mode_pre_envelope_warning_still_parses` |
| Net | 339 raw LOC added |

## Variant mapping (as implemented vs design §5.1)

Design §5.1 listed 6 mapping rows in the table (matching actual `RouterError`)
but the cycle-181 narrative + cycle-182 _notes referred to "8-variant
`RouterError`." The actual enum has **6 variants** — the cycle-181 narrative
count was a tacit miscount; the table is authoritative. This surfaces the
`design-tacit-assumption-falsified-during-implementation` pattern.

| Variant | Class | Detail keys | Notes |
|---|---|---|---|
| `Io(io::Error)` | `io` | `kind`, `message` | Design said `details.path from io::Error context`; falsified — io::Error from `?` propagation does not consistently carry a path field. Emitted shape uses `kind` (Debug-format ErrorKind) + `message` (Display-format). Path-bearing io failures use the explicit `MissingPayloadFile` variant. |
| `Json(String)` | `protocol` | `parse_error` | Direct mapping. |
| `ReducerViolation { channel, attempted_writer, allowed_writer }` | `channel-write-rejected` | `channel`, `writer`, `allowed_writer`, `reason` | `reason="reducer-rule-mismatch"` is the lone fixed value. |
| `InvalidPayload(String)` | `protocol` | `observed_shape` | Direct mapping. |
| `NotInitialized(PathBuf)` | `config` | `key`, `expected`, `hint` | Added `hint="run \`v2-channel-router init\`"` for operator UX continuity (the legacy text mode already names the remedy). |
| `MissingPayloadFile(PathBuf)` | `io` | `path`, `op` | `op="read"` per design table. |

## CI green at boundary

- `cargo test -p v2-channel-router` GREEN at 32 integration + 27 unit = **59 tests** (delta +11 vs cycle 182 close: 8 new envelope unit tests + 3 new integration tests). The cycle-182-mentioned 32 integration result is pre-cycle-183 counting; post-cycle-183 the integration suite reports 35.
- `cargo test --workspace` GREEN at 98 test-suite OK results (no delta in suite count; cycle 183 only added tests within existing crates).
- `cargo clippy -p v2-channel-router --tests -- -D warnings` clean (after fixing 1 cycle-183-introduced `io_other_error` lint flagged by clippy and corrected before commit).
- `cargo clippy --workspace --tests -- -D warnings` NOT-CLEAN at pre-existing lints in `v2-boot-phase` (1× `if_same_then_else` at 751-755 — already documented cycle 182) AND `v2-wiki-search` (3× `useless_vec` at 1135-1136 + test integration line 65; 1× `if_same_then_else` at 213-217). The v2-wiki-search ones are NOT NEW — verified via `git show HEAD:tools/rust/crates/v2-wiki-search/src/main.rs | sed -n '1130,1140p'` and `'210,220p'`. Cycle 182's _notes claim "workspace clippy NOT-CLEAN at pre-existing `v2-boot-phase` … unrelated to cycle 182 changes" understated the pre-existing surface by 4 lints (1 boot-phase + 4 wiki-search instead of just 1 boot-phase). CI does not enforce workspace clippy so this is not a regression — but it IS a new `design-tacit-assumption-falsified-during-implementation` instance (cycle 182's assumption "workspace clippy is clean except for the one boot-phase issue" turned out to be false at cycle 183 surfacing).

## Acceptance criteria (design §7.1 cycle 2)

- [x] `--error-format <text|json>` flag added to CLI (`Args.error_format`, value-enum `ErrorFormat`, default Text, global=true)
- [x] `RouterError::to_envelope(&self) -> ErrorEnvelope` impl
- [x] All 6 actual variants mapped (design table says 6 / narrative count said 8; table authoritative)
- [x] Unit tests verify envelope shape for each variant (8 tests: one per variant + 1 retry-policy invariant + 1 JSON-line-format invariant)
- [x] Integration test: invoke router with `--error-format=json` on a deliberately-erroring input (uninitialized state dir, `read` subcommand), parse envelope from stderr last line via `v2_error_envelope::parse_from_stderr_tail`, assert class=Config + key details (`channels_dir`, `hint`)
- [x] Bonus integration tests: text-mode legacy preservation (negative-assertion that envelope-parse fails on text-mode stderr), pre-envelope-warning + envelope-on-last-line co-occurrence (lenient-mode + reducer-violation)

## LOC ratio measurement

Design §8 estimated cycle 2 at **~150 source LOC + ~8 unit + 1 integration ≈ 250 raw / 290-375 honest at 1.2-1.5× refined band per cycle-182 measurement**.

Actual cycle 183: **339 raw LOC**.

- 339 / 250 = **1.36× raw**
- 339 / 332.5 (mid honest band) = **1.02× honest at midpoint** — essentially exactly the honest-band midpoint
- 339 / 375 = **0.90× honest at upper band**

This is the **third consecutive thorough-design-scope cycle** to land within the
honest band (cycle 179 0.79× honest / cycle 182 0.99× honest at midpoint /
cycle 183 1.02× honest at midpoint). The `comprehensive-test-suite-exceeds-
design-scope-LOC-estimate` series is now 6 data points and stabilizing at
1.3-1.6× raw / 0.8-1.0× honest for thorough design scopes. The hypothesis
established cycle 179 ("more thorough design scope → tighter LOC ratio at
implementation") has now three confirming cycles in a row. The hypothesis from
the much earlier cycles 170-171 (4.5-7× raw) increasingly looks like the
under-thoroughness case, not the typical case.

## Pattern updates

### `design-tacit-assumption-falsified-during-implementation` (NOVEL@1 cycle 180; RECURRENCE-AT-2 cycle 182; **RECURRENCE-AT-3 cycle 183**)

Cycle 183 surfaced TWO new tacit-assumption-falsified instances:

1. **Variant count: design narrative said 8 variants, actual is 6.** The §5.1
   table was correct at 6 rows; the cycle-181 commit-message narrative and
   cycle-182 _notes carried over the "8" count. Cycle 183 implementation made
   this visible because mapping to envelope requires enumerating actual
   variants.

2. **Pre-existing workspace clippy state was richer than cycle-182 captured.**
   Cycle 182 _notes claimed "workspace clippy NOT-CLEAN at pre-existing
   `v2-boot-phase/src/main.rs:751-755`" — implying that was the ONLY
   pre-existing lint. Cycle 183 discovered 4 additional pre-existing lints in
   v2-wiki-search (3 `useless_vec` + 1 `if_same_then_else`). Net: 5 pre-existing
   workspace clippy lints, not 1.

Both confirm the pattern: tacit assumptions in design or earlier-cycle notes
get surfaced during the next implementation cycle that needs to act on them.
Forward-watch continues for cycles 183-189 (5 of 6 consumed; deadline cycle 188
to consume one more or pattern decays). The variant-count instance is the
"design-side" sub-shape; the clippy instance is the "earlier-cycle-notes-side"
sub-shape — both qualify under the umbrella pattern.

### `comprehensive-test-suite-exceeds-design-scope-LOC-estimate` (RECURRENCE-AT-5 cycle 182 → **RECURRENCE-AT-6 cycle 183**)

Six data points now: cycle 170 4.5× raw / cycle 171 7× raw / cycle 177 2.8× raw
/ cycle 179 1.58× raw 0.79× honest / cycle 182 1.38× raw 0.99× honest / cycle
183 1.36× raw 1.02× honest. Cycles 179+182+183 all land at 1.3-1.6× raw with
honest-budget ratio at 0.8-1.0×. Pattern is stabilizing; the multiplier
question (1.2-1.5× refined band cycle 182 floated) is borne out at 3 data
points. Refined hypothesis: **thorough design scopes (≥350 lines, explicit
honest-budget acknowledgment, per-cycle acceptance criteria) land at 1.3-1.6×
raw / 0.8-1.0× honest at midpoint**.

### `coordinated-arc-design-scope-pairs-deferred-items` (RECURRENCE-AT-5 cycle 182 → **RECURRENCE-AT-6 cycle 183**)

Sixth instance — cycle 181 design → cycle 182 implementation cycle 1 → cycle
183 implementation cycle 2. The pair-vs-followup distinction (cycle 180 raised,
cycle 182 ripe at 3 pair + 1 followup) now sits at 3 pair-closures + 2 followup
cycles within the C6+C7+C9 arc. Naming-refinement-to-split-into-two-patterns
still deferred; will revisit at RECURRENCE-AT-7+ if the followup-count grows
faster than pair-count.

### `shared-crate-extraction-from-isolated-reducer-rule-precedent` (RECURRENCE-AT-2 cycle 182 → NOT-EXERCISED cycle 183)

Cycle 183 consumed v2-error-envelope as a downstream consumer rather than
creating a new shared crate. 3 of 6 consumed; DEADLINE cycle 186. Cycle 184
(role-driver emit-side) is the second consumer; cycle 185 (cycle-runner caller)
will be the third — three callers reinforce the cycle-182 polarity-axis
positioning but don't add a new "extraction" event.

### `session-start-CI-check-discipline` HARDENED → orientation-prelude mode cycle 183

Standing step #2 confirmed `b06c83f8` SUCCESS held on cycle 182 close (all
required checks: Node 20/22/24, PHP 8.1-8.5, Check and Test, Static Analysis,
Code Style). No RECURRENCE-AT-N tracking.

### `gitignore-extension-from-orchestrator-tempfile-sandbox-friction` (NOT-EXERCISED 4 of 6 consumed cycle 182 → **EXERCISED-AND-RESOLVED-VIA-GITIGNORE-EXTENSION cycle 183** — the prescribed resolution path)

Cycle 183 hit the friction directly twice:

1. **Write to /tmp blocked.** `Write` tool refused
   `/tmp/session-start-c183.md` per sandbox restriction. Workaround: workdir
   `.tmp-c183/session-start.md` for the gh-comment body.
2. **rm of workdir scratchdir ALSO blocked.** Attempted `rm -rf .tmp-c183/`
   and `rm .tmp-c183/session-start.md` were both blocked by Claude Code's
   permission system (compound `rm` operations require approval, and the
   underlying sandbox treats workdir rm with same caution as /tmp writes).

Resolution path 1 (pre-commit cleanup) was therefore blocked, forcing
resolution path 2 (gitignore extension) — exactly what the pattern's name
predicts. Added `.tmp-c*/` to `.gitignore` so `.tmp-c183/` becomes invisible
to git without needing to delete it. The pattern's prescribed resolution is
**vindicated** — first-instance reasoning that "cleanup is simpler than
gitignore extension" was wrong because cleanup is also sandboxed. Future
cycles using `.tmp-c<N>/` scratchdirs are now covered automatically.

Pattern RESOLVED-VIA-GITIGNORE-EXTENSION; advances to 5 of 6 consumed; DEADLINE
cycle 184. If cycle 184 needs a different shape of scratchdir (e.g., a name
that doesn't match `.tmp-c*/`), pattern may fire again; otherwise pattern
HARDENED at "always extend .gitignore for new workdir-scratch shapes."

### `eva-directive-overrides-implicit-arc-serialization` NOT-EXERCISED cycle 183 (4 of 6 consumed; DEADLINE cycle 185)

No Eva directive landed cycle 183. C6+C7+C9 arc proceeds under cycle 181 named
priority chain.

### `as-needed-verification-of-HARDENED-principle-on-tool-rerun` NOT-EXERCISED cycle 183 (5 of 6 consumed; DEADLINE cycle 184)

No prompt-touching changes in cycle 183.

### `cargo-spawned-subprocess-bypasses-tool-permission-intercept` NOT-EXERCISED cycle 183 (3 of 6 consumed; DEADLINE cycle 187)

No subprocess-spawning friction in cycle 183 (binary built + run via cargo
integration test which uses `Command::new` directly per the existing
`bin()` helper).

### `design-scope-honesty-hedge-survives-implementation-cycle` (NOT-EXERCISED PRECONDITION-SET cycles 181-182 → **NOT-EXERCISED cycle 183**)

Cycle 181 design contained 8 OQ-SEE-* hedges. Cycle 183 implementation honored
two of them tacitly:

- **OQ-SEE-1** (envelope byte-length): cycle 183 measured envelope-line length
  on the `ReducerViolation` variant (the largest expected) — JSON line ~~270
  bytes well under any reasonable concern. Not formally verified; no
  byte-length assertion in tests. Honoring is "did not bloat" not "did
  measure-and-prove."
- **OQ-SEE-2** (pre-envelope-warning + tail-parse): exercised explicitly by
  the new integration test
  `error_format_json_with_lenient_mode_pre_envelope_warning_still_parses`.
  This is direct hedge-honoring — design said cycle-2 integration test should
  combine pre-envelope warnings + envelope final line; cycle 183 wrote
  exactly that test. **OQ-SEE-2 closed by cycle 183 test.**

Other OQ-SEE-* items target cycles 184-185 (role-driver / cycle-runner side).
RECURRENCE-AT-3 forward-watch continues 4 of 8 cycles remaining; DEADLINE
cycle 188.

### `deferred-arc-cycles-from-original-defer-to-design-scope` NOT-EXERCISED cycle 183 (2 of 6 consumed; DEADLINE cycle 188)

No new deferred-arc reached design scope in cycle 183. Forward-watch continues
for C11+C12+X1 resume-arc (predicted design cycle 196-205 per cycle-181
20-25-cycle aging).

## Forward priorities for cycle 184

1. **(Standing) orientation-prelude HARDENED step** — confirm master CI green
   on cycle-183 close commit at session start.
2. **AWAIT #2997** — Eva response on OQ-LS-AUTH child-spawn credential
   propagation. 5-cycle BETWEEN-CHECKPOINTS band expires cycle 185-186 (no
   response by cycle 186 → orchestrator may propose a default and proceed,
   though the EVA-DEFAULT-AUTONOMY categorization says infra-secret-add is
   Eva-only so the default behavior is "continue to AWAIT indefinitely" not
   "self-resolve at 5-cycle deadline" for secret-add specifically).
3. **C6+C7+C9 implementation cycle 3 (NEW priority #3)** — v2-role-driver
   emit-side per design §5.2: add `--error-format <text|json>` flag at the
   `Invoke` subcommand parse site, implement `impl DriverError { fn
   to_envelope(&self) -> ErrorEnvelope }` mapping all 11 variants per §5.2
   table (including the 6 cycle-179 live-spawn-introduced variants), branch
   `emit_invoke_result`-equivalent text emission on flag. Specifically include
   the cycle-180 OQ-LS-AUTH visibility extension: when
   `parse_claude_code_envelope` surfaces `is_error: true` with `result="Not
   logged in"` (or similar auth-failure shape), emit envelope with
   `class=auth, details.subsystem="claude-code-oauth",
   details.upstream_result="Not logged in"`. Estimate **~250 source LOC + ~15
   unit + 2 integration ≈ 500 raw LOC / 600-750 honest at 1.2-1.5× refined
   band per cycle-183 measurement**. Track 1 substantive.
4. **(Track 2 bounded-mechanical)** — cycle-184 _notes + journal entry.

Note: the design §5.2 mapping table for `DriverError` lists 11 variants. Cycle
183's design-tacit-assumption-falsified-during-implementation discovery
recommends verifying the actual variant count before mapping (the §5.2 table
may have the same kind of narrative-vs-table mismatch as §5.1 did).
