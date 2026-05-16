redesign(phase-3): cycle 160 Track 1 — reconciler completeness-metadata pairing (L1.3 + L3.6 AGREE-ACT-NOW, cycle 158 forward priority #4 / cycle 159 priority #3 CLOSED)

Adds `inbound-completeness-marker` as a 4th required key on the inbound-channel payload contract, with values `complete` / `partial` / `quiet`. Closes the L1.3 / L3.6 failure mode from PR #2951 critique (cycle 148 absorption) where the reconciler's best-effort fallback path emitted structurally valid payload with arrays-present-but-incomplete and no signal to downstream consumers about whether the absence-of-events reflected polling success or polling failure.

Two-tool pairing per the L1.3 finding: prompt edit + router-side required-keys extension.

Surface area:

- `prompts/v2/reconciler-prompt.xml`: new `<required-key name="inbound-completeness-marker">` with semantics for the three values; example payloads updated (complete / quiet / new partial); validation section enumerates the four keys; error-handling section requires emitting `partial` when source poll fails (not silent best-effort); quiet-cycle-handling section requires emitting `quiet` (and forbids using `quiet` to hide poll failures); polling-procedure step (3) updated to "4 required keys".

- `tools/rust/crates/v2-channel-router/src/main.rs`: marker added to `Channel::InboundChannel::required_payload_keys`; new unit test `validate_payload_rejects_inbound_missing_completeness_marker`. The router enforces presence only (existing minimal-scope contract); value-shape enforcement lives in the writer (processor + reconciler).

- `tools/rust/crates/v2-channel-router/tests/integration.rs`: `inbound_payload_body` helper updated.

- `tools/rust/crates/v2-reconciler-event-processor/src/main.rs`: `validate_inbound_payload` extended to require the marker key, verify it's a string, and verify the value is in the three-element enum (`COMPLETENESS_MARKER_VALUES` const); new `derive_processor_marker` helper that returns `quiet` if all three event arrays are empty and `complete` otherwise (the processor aborts on source-poll failure, so it never emits `partial`); `cmd_poll` payload-building uses the derived marker; `cmd_schema`'s `inbound_required_keys` lists the new key; 5 new unit tests covering marker validation (missing, non-string, invalid value, all-valid-values, derivation logic).

- `tools/rust/crates/v2-reconciler-event-processor/tests/integration.rs`: assertions for marker in poll outputs; `schema_json_is_well_formed` updated to assert the 4-key inventory by name (stronger than length-only check).

- `tools/rust/crates/v2-role-driver/src/main.rs`: hardcoded inbound-channel required-keys copy (SCAFFOLD duplication acknowledged in the source comment) extended to match; corresponding unit test updated.

- `tools/rust/crates/v2-role-driver/tests/integration.rs`: reconciler payload helper updated.

- `tools/rust/crates/v2-cycle-runner/tests/integration_cycle.rs`: reconciler session-output helper updated; both the dry-run and live-run end-to-end tests exercise the new 4-key contract against the real primitive binaries.

Tests / lint:
- `cargo build -p v2-channel-router / v2-reconciler-event-processor / v2-role-driver / v2-cycle-runner` clean.
- `cargo test -p v2-channel-router`: 26 unit + 4 integration passed (was 23 + 4 before; +3 new tests).
- `cargo test -p v2-reconciler-event-processor`: 52 tests passed (was 47; +5 new).
- `cargo test -p v2-role-driver`: 42 tests passed.
- `cargo test -p v2-cycle-runner`: 2 integration tests passed (`live_run_against_real_primitives_writes_completed_state_with_no_halt_marker` exercises the full payload chain end-to-end with real v2-channel-router + v2-reconciler-event-processor binaries).
- `cargo test -p v2-prompt-contract-check`: `strict_check_passes_against_live_prompts` passes — confirms reconciler-prompt.xml's `<required-key>` declarations match the router's runtime-emitted schema exactly.
- `cargo clippy --all-targets -- -D warnings` clean for v2-channel-router, v2-reconciler-event-processor, v2-role-driver.

Cycle 160 forward priorities CLOSED: #3 (reconciler completeness-metadata pairing).
