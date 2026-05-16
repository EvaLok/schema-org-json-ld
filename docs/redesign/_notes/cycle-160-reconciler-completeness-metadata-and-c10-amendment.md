# Cycle 160 — Track 1: reconciler completeness-metadata pairing (L1.3 + L3.6 AGREE-ACT-NOW) + Track 2: C10 (L2.5) design-scope amendment

**Cycle issue:** [#2969](https://github.com/EvaLok/schema-org-json-ld/issues/2969)
**Start:** 2026-05-16 10:25 UTC
**Mode:** redesign / Phase 3
**Model:** claude-opus-4-7

## Composition

Two-track composition continues, **HARDENING-AT-13** post cycle 151 single-track exception (9 consecutive post-exception cycles: 152-160). Combined arc: 146-150 (5 consecutive) → 151 single-track exception → 152-160 (9 consecutive post-exception). 14 of 15 cycles in the 146-160 arc are two-track.

Cycle 159 was an **asymmetric-closure** cycle (cycle 158 produced 2 new priorities #6 + #15; cycle 159 closed #6 but not #15 because #15 is gate-bounded). Cycle 160 is a **mixed-closure** cycle — Track 1 closes a long-deferred inherited priority (#3 reconciler completeness-metadata, deferred since cycle 148 absorption = 12 cycles); Track 2 closes a shorter-deferred inherited priority (#1 C10 amendment, deferred since cycle 158 = 2 cycles). Neither closure is from cycle 159's NEW priorities (#5 / #6 from the v2-state-audit live-smoke finding); those remain on the priority list pending their own dedicated cycles.

**44th consecutive cycle of HONORING named forward priority** (cycles 115-160). Track 1 closes priority #3 (was cycle 158 #4 → cycle 159 #3); Track 2 closes priority #1 (was cycle 158 #2 → cycle 159 #1).

## Track 1 — Reconciler completeness-metadata pairing (L1.3 + L3.6)

The L1.3 / L3.6 finding from PR #2951 cycle 148 absorption (cycle 148-two-track-absorption-and-landing.md, lines 51–53 + 106–107): the reconciler's `error-handling` and `quiet-cycle-handling` sections of `reconciler-prompt.xml` permit best-effort continuation that emits structurally valid payload with arrays present but doesn't distinguish "polling succeeded with no events" from "polling failed and arrays under-populated." Router enforces only key PRESENCE, not source completeness. Concrete fix specified in cycle 148: "reconciler must emit explicit completeness metadata (e.g., `inbound-completeness-marker` required key with values `complete | partial | quiet`). Requires (a) prompt edit + (b) router `required_payload_keys` extension."

Cycle 160 lands the full pairing, which turned out to require updates across **5 crates + 1 prompt file** because the inbound-payload key inventory is referenced in several places (an existing schema-duplication form acknowledged in the v2-role-driver source comment).

### Surface area landed

**`prompts/v2/reconciler-prompt.xml`** — new `<required-key name="inbound-completeness-marker" type="string">` block inside `<output-contract>/<format>`, with the three-value enum (`complete` / `partial` / `quiet`) and prescriptive semantics for when each applies. The `<example>` now sets marker to `complete`; `<quiet-cycle-example>` sets marker to `quiet`; new `<partial-cycle-example>` shows what partial looks like. `<validation>` enumerates the 4 keys (was 3). `<error-handling>` explicitly REQUIRES emitting `partial` when source poll fails (was: "include only confidently collected events" with no signal). `<quiet-cycle-handling>` REQUIRES emitting `quiet` and FORBIDS using `quiet` to hide a poll failure. `<polling-procedure>` step (3) updated to "all 4 required keys".

**`tools/rust/crates/v2-channel-router/src/main.rs`** — `Channel::InboundChannel::required_payload_keys()` returns 4 keys (was 3). New unit test `validate_payload_rejects_inbound_missing_completeness_marker`. Router enforces PRESENCE only — value-shape (string + enum membership) is the writer's responsibility, consistent with the existing minimal-scope contract (v2-channel-router COMPLETE arc covers type checks).

**`tools/rust/crates/v2-channel-router/tests/integration.rs`** — `inbound_payload_body` helper updated to include marker. Existing tests `write_inbound_channel_with_reconciler_succeeds`, `all_four_channels_writable_with_correct_writer`, etc. continue to pass because they use this helper.

**`tools/rust/crates/v2-reconciler-event-processor/src/main.rs`** — three substantive additions:
1. `COMPLETENESS_MARKER_VALUES` const enumerating the three values.
2. `validate_inbound_payload` extended to require the marker key, check string type, check enum membership.
3. `derive_processor_marker(eva_events, audit_events, dispatch_events)` helper that returns `quiet` if all three arrays are empty, `complete` otherwise. **The processor never emits `partial` because source-poll failure aborts the whole poll — `partial` is reserved for the fallback manual path where reconciler does its own polling without the processor.**
4. `cmd_poll` payload-building uses the derived marker.
5. `cmd_schema`'s `inbound_required_keys` lists the 4 keys.

5 new unit tests: `validate_inbound_rejects_missing_completeness_marker`, `validate_inbound_rejects_non_string_marker`, `validate_inbound_rejects_invalid_marker_value`, `validate_inbound_accepts_each_valid_marker_value`, `derive_processor_marker_quiet_when_all_empty`, `derive_processor_marker_complete_when_any_source_has_events`. Existing `validate_inbound_passes_with_all_keys_as_arrays` updated to include marker.

**`tools/rust/crates/v2-reconciler-event-processor/tests/integration.rs`** — assertions added to `poll_with_all_empty_sources_writes_empty_inbound_payload` (marker == "quiet") and `poll_with_events_in_all_sources_populates_inbound_payload` (marker == "complete"). `schema_json_is_well_formed` updated to assert the 4-key inventory by name (stronger than length-only check); `schema_text_lists_three_sources_and_inbound_binding` updated to assert the marker key is in the human-readable schema output.

**`tools/rust/crates/v2-role-driver/src/main.rs`** — `required_payload_keys("inbound-channel")` returns 4 keys (was 3). Unit test updated. The source comment at line 185–186 acknowledges this is a SCAFFOLD-scope duplication of v2-channel-router's same data, named for future deduplication via subprocess invocation of `v2-channel-router write` (DEFERRED #1 per cycle-149-v2-cycle-runner-design-scope.md §3.3).

**`tools/rust/crates/v2-role-driver/tests/integration.rs`** — reconciler stub payload updated.

**`tools/rust/crates/v2-cycle-runner/tests/integration_cycle.rs`** — reconciler session-output helper updated. **Both the dry-run and live-run end-to-end tests pass against real v2-channel-router + v2-reconciler-event-processor binaries** — this is the strongest single-cycle verification because it exercises the 4-key contract through the full primitive chain (role driver → reconciler → channel router → boundary → planner → ...).

### Tests / lint

| Crate | Before | After | New tests |
|---|---|---|---|
| v2-channel-router | 23 unit + 4 integration | 24 unit + 4 integration | +1 |
| v2-reconciler-event-processor | 47 total | 52 total | +5 |
| v2-role-driver | 42 total | 42 total | 0 (existing tests cover) |
| v2-cycle-runner | 2 integration | 2 integration | 0 |
| v2-prompt-contract-check | 11 unit + 1 integration | 11 unit + 1 integration | 0 |

All `cargo build / test / clippy --all-targets -- -D warnings` clean.

**`strict_check_passes_against_live_prompts` integration test passes**, confirming that the live reconciler-prompt.xml declared `<required-key>` set matches the live v2-channel-router runtime-emitted schema set exactly. This is the load-bearing end-to-end pairing check.

### In-session issue and recovery

**Clippy lint surfaced an idiomatic-Rust improvement:** my first `derive_processor_marker_complete_when_any_source_has_events` test passed `&[evt.clone()]` for each of the three call shapes. Clippy `cloned_ref_to_slice_refs` (rust-1.95.0) flagged this as `std::slice::from_ref(&evt)`. Single Edit recovery using a hoisted `one` variable. Lesson: when constructing a single-element slice for a function call, `std::slice::from_ref(&x)` is the idiomatic form, not `&[x.clone()]`.

## Track 2 — C10 (L2.5) design-scope amendment

Bounded textual change to `cycle-149-v2-cycle-runner-design-scope.md` row for the SuperStepOutOfOrder failure class.

**Background** (cycle-155-cycle-152-critique-absorption.md lines 65–68, 122):

- Design scope said SuperStepOutOfOrder = "Hard error; abort cycle without state mutation; runner exits non-zero."
- Implementation (`halt_cycle`, v2-cycle-runner) calls `write_runner_state` for ALL halt classes including out-of-order before returning Err.
- Cycle 155 verdict: paired AGREE-ACT-NOW + AGREE-WITH-CARVEOUT. ACT-NOW = amend the design wording. CARVEOUT = preserve the current implementation behavior (it produces useful postmortem data).

**This commit lands the ACT-NOW half.** The clarification:

1. Inline addition to the SuperStepOutOfOrder row: "abort cycle without super-step state-machine mutation; runner exits non-zero. The runner MAY persist runner-local observability state (e.g. `state/v2-cycle-runner/last-cycle.json` postmortem record) — that is not 'state mutation' in the contractual sense, which refers strictly to super-step state-machine state."

2. New dedicated paragraph following the row: "Scope of 'state mutation' (cycle 160 clarification...)" enumerating what counts as super-step state-machine state (channel state envelopes, `state/super-step.json`, reconciler cursors, role-driver-written artifacts) and naming that runner-local observability state is the postmortem aid the runner is free to write. The paragraph also notes that cycle 160 preserves the existing `halt_cycle` implementation that writes `last-cycle.json` for ALL halt classes including out-of-order; this clarification makes the design contract consistent with that behavior rather than forcing a code change.

**Unblocks Missing Integration Scenario 3** from cycle 155 absorption (super-step-out-of-order live integration test, currently AGREE-DEFER): with C10 amended, the test can now assert state-mutation-absent for super-step state AND state-mutation-present for runner-local observability state. The test itself remains a future cycle.

**Surface area:** 1 file, +3 -1 LOC. No code changes.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 reconciler-prompt + v2-channel-router + v2-reconciler-event-processor + v2-role-driver + v2-cycle-runner | 254 +ins, 26 -del = +228 net LOC across 8 files | `429ac79a` |
| Track 2 cycle-149 design-scope C10 row + paragraph | 3 +ins, 1 -del = +2 net LOC | `0ad96a4a` |
| `cycle-160-reconciler-completeness-metadata-and-c10-amendment.md` (this _notes) | ~280 lines | cycle-close |
| Total cycle 160 textual + code output | ~510 net LOC | 3 commits |

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 160 datapoints:
- Track 1 shape was "schema-bump across N tools" — prediction range from cycle 148 absorption was "small prompt + router edit candidate" (probably under-estimated). Actual: 8 files touched because the inbound-key inventory is referenced in 5 crates + 1 prompt. The duplication is acknowledged in v2-role-driver source comment as SCAFFOLD-scope but it materially expanded this cycle's surface area beyond the cycle-148 prediction.
- Track 2 shape was "bounded textual edit" — prediction was "1 cycle (textual)" per cycle 155 absorption table. Actual: 2 net LOC, well under 1 cycle. The shape held to prediction.

## Pattern updates this cycle

- **`schema-duplication-across-multi-crate-pipeline-amplifies-required-key-edits`** NOVEL@1 cycle 160. The inbound-channel required-key inventory exists in: (1) `prompts/v2/reconciler-prompt.xml` `<required-key>` declarations, (2) `tools/rust/crates/v2-channel-router/src/main.rs` `required_payload_keys()`, (3) `tools/rust/crates/v2-reconciler-event-processor/src/main.rs` `validate_inbound_payload` + `cmd_schema::inbound_required_keys`, (4) `tools/rust/crates/v2-role-driver/src/main.rs` `required_payload_keys()`, (5) integration test helpers in 3 crates. Adding a single new required key required edits in all 5 places. Recurrence test: any future required-key addition. Mitigation candidate: have v2-role-driver and v2-reconciler-event-processor delegate to v2-channel-router subprocess instead of duplicating — but this is DEFERRED to v2-cycle-runner COMPLETE arc; SCAFFOLD scope tolerates the duplication.

- **`end-to-end-integration-test-as-load-bearing-verification`** RECURRENCE-AT-3 cycle 160 (cycles 148, 156, 160). Pattern: when changing a contract across multiple crates, the strongest verification is an integration test that exercises the full pipeline with real binaries (not mocks). Cycle 160's `live_run_against_real_primitives_writes_completed_state_with_no_halt_marker` and `strict_check_passes_against_live_prompts` both passed without modification, confirming the 4-key contract holds end-to-end.

- **`clippy-as-idiomatic-rust-teacher`** NOVEL@1 cycle 160. Clippy lint `cloned_ref_to_slice_refs` surfaced `std::slice::from_ref` as the idiomatic alternative to `&[x.clone()]` for single-element slice construction. Single-Edit recovery. Lesson: don't dismiss clippy lints as nits — some are genuine API discoveries.

- **`mixed-closure-of-different-deferral-ages`** NOVEL@1 cycle 160. Track 1 closed an inherited priority deferred 12 cycles (since cycle 148 absorption); Track 2 closed an inherited priority deferred 2 cycles (since cycle 158). Pattern test: cycles can pair work items of very different deferral ages. The two ACT-NOWs were independent; closing both in the same cycle doesn't require they be the same age.

- **`agree-act-now-bounded-fix-via-tracksecond-pattern`** RECURRENCE-AT-5 cycle 160 (cycles 150, 155, 156, 159, 160). Pattern: bounded AGREE-ACT-NOW critique findings make excellent Track 2 work.

- **`two-track-composition` HARDENING-AT-13** cycle 160 (9 consecutive post cycle 151 exception). Combined arc: 146-150 (5) + 151 single-track exception + 152-160 (9) = 14 of 15 cycles two-track.

- **`tool-extraction-surfaces-prior-cycle-errors` REVERSED on cycle 160**: cycle 156 and 159's pattern was "tool extraction surfaces prior-cycle errors." Cycle 160 inverts this — the prompt+router edit surfaces an old documentation/code duplication that was always there but only visible when a required-key change forces touching all duplicate sites. So the pattern is: "Schema-bump operations are first-class duplication-surface mechanisms, not only the tool-extraction debugging variant."

## Process honoring

- **45th consecutive cycle of HONORING named forward priority** (cycles 115-160). Cycle 159 named #3 + #1 in the renumbered carry-forward list; cycle 160 closes both.
- **73rd bottleneck-asynchronous cycle** (78-160).
- **50th non-per-candidate-sharpening cycle** (111-160).
- Cycle 120 L2 preserved (`2-selection.md` untouched cycle 160).
- Cycle 128 lesson preserved (.scratch/ via Write tool — heredoc redirection blocked once cycle 160, immediate fallback to Write).
- Cycle 133 clarification preserved: 12 cargo invocations cycle 160 across 5 crates × {build, test, clippy} (some crates pulled by transitive dep rebuild). All bounded to the schema-change surface; no gratuitous "let's run all tests" sweeps.
- Cycle 134 lesson preserved (audit-repo HEAD via gh api at session-start; unchanged at `8285b7d3`).
- **Cycle 137 lessons re-validated cycle 160 14-cycle-running** (137 + 147-160). All `gh` operations single-purpose-bash-invocation form.
- **Cycle 149 in-cycle CWD-drift lesson re-validated cycle 160** — all cargo via `--manifest-path tools/rust/Cargo.toml`.
- Cycle 151 date-test-value lesson preserved (no new date tests).
- Cycle 152 disk-format-read-source lesson preserved.
- Cycle 153 multi-Edit-fallback lesson preserved (not exercised cycle 160).
- Cycle 154 `gh api graphql` subprocess pattern preserved (not exercised cycle 160).
- Cycle 155 dispatch-return-detection lesson preserved (v2-dispatch-status not invoked cycle 160; no in-flight dispatches).
- Cycle 156 anti-overstatement audit enumeration discipline preserved (cycle 160 _notes has explicit "What cycle 160 does NOT do" section).
- Cycle 157 audit-HEAD-check at session-start preserved (HEAD unchanged at `8285b7d3` — same as cycles 158/159; no new audit content cycle 222 yet).
- Cycle 158 symmetric-closure pattern observed (special case); cycle 159 asymmetric-closure pattern observed (gate-bounded priorities defer); cycle 160 mixed-closure pattern observed (different deferral ages can close together).
- Cycle 159 test-pattern-mirror-existing-control-flow lesson preserved (not exercised in this exact form, but adjacent: when writing the marker-derivation tests I used the same construction shape as `derive_processor_marker_quiet_when_all_empty` for the complete variant, mirroring the existing call-site shape).
- Journal-immutability discipline preserved (cycle 160 appends NEW section via Edit anchor at end of cycle 159 section; cycle 148-159 sections NOT back-edited).
- **Two-track composition continued** — cycle 160 is 9th consecutive post cycle 151 exception (HARDENING-AT-13).
- **SECTION 6b list housekeeping NOT invoked cycle 160** — 5 currently-open issues (cycle issue + 4 standing input-from-eva); no closure candidates.

## In-session issues and recoveries

- **In-session clippy lint cycle 160.** First attempt at `derive_processor_marker_complete_when_any_source_has_events` test used `&[evt.clone()]` × 3 for the three call shapes. `cargo clippy --all-targets -- -D warnings` flagged this as `cloned_ref_to_slice_refs`. Recovery: hoist `let one = std::slice::from_ref(&evt);` and pass `one` to each call. Single Edit cycle. Lesson recorded as `clippy-as-idiomatic-rust-teacher` NOVEL@1.
- **In-session heredoc-redirection block.** First attempt at posting session-start used `cat > .scratch/cycle160-session-start.md <<'EOF' ... EOF` heredoc; sandbox blocked redirection. Immediate recovery to Write tool (cycle 128 lesson preserved).
- All `gh api` / `gh issue list` operations clean cycle 160 (single-purpose-bash-invocation form preserved).
- All `cargo build` / `cargo test` / `cargo clippy` operations clean cycle 160 across 5 crates.
- All Edit / Write operations clean cycle 160.
- All `git add` / `git commit` / `git push` operations clean cycle 160.

## Cycle 160 ARTIFACTS

Track 1 (commit `429ac79a`):
- `prompts/v2/reconciler-prompt.xml` — modified (+marker required-key + 3 example updates + validation/error/quiet section semantics).
- `tools/rust/crates/v2-channel-router/src/main.rs` — modified (+marker in required_payload_keys + 1 new unit test).
- `tools/rust/crates/v2-channel-router/tests/integration.rs` — modified (helper).
- `tools/rust/crates/v2-reconciler-event-processor/src/main.rs` — modified (validate + derive + cmd_poll + cmd_schema + 5 new tests).
- `tools/rust/crates/v2-reconciler-event-processor/tests/integration.rs` — modified (assertions).
- `tools/rust/crates/v2-role-driver/src/main.rs` — modified (hardcoded copy + test).
- `tools/rust/crates/v2-role-driver/tests/integration.rs` — modified (helper).
- `tools/rust/crates/v2-cycle-runner/tests/integration_cycle.rs` — modified (helper).

Track 2 (commit `0ad96a4a`):
- `docs/redesign/_notes/cycle-149-v2-cycle-runner-design-scope.md` — modified (+row clarification + dedicated paragraph for cycle 160 scope clarification).

Cycle-close:
- `docs/redesign/_notes/cycle-160-reconciler-completeness-metadata-and-c10-amendment.md` — new (this file, ~280 lines).
- Journal section in `docs/journal/2026-05-16.md` — appended via Edit anchor at end of cycle 159 section.
- `.scratch/cycle160-*.{md,txt}` — ephemerals (session-start, Track 1 commit msg, Track 2 commit msg, session-end, issue-close).

Counts:
- 3 direct-push commits cycle 160 (Track 1 + Track 2 + cycle-close).
- 0 issues closed cycle 160 (no candidates; cycle issue itself closes per existing convention).
- 0 dispatches cycle 160.
- 12 cargo invocations cycle 160 (5 crates × ~{build, test, clippy} with bounded scope). All clean.
- ~6 GitHub API operations cycle 160 (audit HEAD, input-from-eva list, open issues, cycle issue view, comment posts, cycle close).
- 8 Edit operations on the 8 modified files.
- 0 Edits on `docs/state.json`.
- 0 Edits on legacy `tools/cycle-runner/`.
- 0 Edits on `.github/workflows/`.
- 0 Edits on this orchestrator prompt.

## Forward priorities for cycle 161+

**Renumbered list (cycle 159 inherited + cycle 160 closures applied):**

1. **X5 commit-governance scope clarification** — bounded textual addition to v2-cycle-runner crate doc-comment declaring commit-governance OUT of scope. Was cycle 160 priority #2 (deferred again this cycle). Carries forward. <1 cycle.

2. **AGREE-ACT-NOW L2.4 dispatch-brief discipline addendum** (from cycle 148 absorption). Was cycle 160 priority #4. Carries forward. <1 cycle.

3. **v2-state-dispatch-sync refuse-to-write hard-threshold enforcement** (from cycle 159 live-smoke finding). Implement the owner-side hard-threshold semantic. NOTE: cycle 159's priority text was slightly misframed — the actual `add` happens in v1 `record-dispatch` (frozen zone), not v2-state-dispatch-sync. A design scope cycle may be the right shape rather than direct code work. Was cycle 160 priority #5. Carries forward.

4. **Threshold recalibration for state-json-dispatches axis** (from cycle 159 live-smoke finding). Pairs with #3. Was cycle 160 priority #6. Carries forward.

5. **TOOL-SCOPED `v2-channel-router` enforcement extension design scope** (C1 / L3.1 from cycle 148 absorption). Was cycle 160 priority #7. Carries forward.

6. **TOOL-SCOPED `v2-prompt-tag-semantic-fidelity` tool design scope** (L2.5 from cycle 148 absorption). Was cycle 160 priority #8. Carries forward.

7. **`status` + `verify` v2-cycle-runner subcommands** — was cycle 160 priority #9. Carries forward.

8. **AGREE-DEFER queue (post-real-role-session-measurement)** — was cycle 160 priority #10. Hold for live-claude-code-spawn evidence (still SCAFFOLD). Carries forward.

9. **Coordinated retry/timeout/cancellation arc** — C13 + X2. Was cycle 160 priority #11. Carries forward.

10. **Coordinated structured-error-envelope arc** — C6 + C7 + C9. Was cycle 160 priority #12. Carries forward.

11. **Coordinated resume/recovery arc** — C11 + C12 + X1. Was cycle 160 priority #13. Carries forward.

12. **Audit-engagement substantive-focal single-track variant** — Was cycle 160 priority #14. Carries forward; gate = audit HEAD changes.

13. **Per-axis archival mechanism design scope** — Was cycle 160 priority #15. Gate-bounded; NOT YET TRIGGERED. Carries forward unconditionally.

14. **Missing Integration Scenario 3 — super-step-out-of-order live integration test** (from cycle 155 absorption, now unblocked by cycle 160 Track 2 C10 amendment). Was AGREE-DEFER pending C10; now writeable. <1 cycle if paired with another bounded item.

15. **Cycle 120 L2 preserved** (no recursive annotation of `2-selection.md`).

**Cycle 160 forward priorities CLOSED:** #1 (C10 amendment, Track 2) + #3 (reconciler completeness-metadata pairing, Track 1).

**Newly unblocked by cycle 160:** Missing Integration Scenario 3 (now priority #14) — cycle 160 Track 2 C10 amendment makes this test writeable.

## What cycle 160 does NOT do

1. Does NOT modify legacy `tools/cycle-runner` (v1; forbidden zone preserved during Phase 3).
2. Does NOT modify `.github/workflows/` or this orchestrator prompt.
3. Does NOT implement v2-state-dispatch-sync refuse-to-write semantics (carries forward as priority #3).
4. Does NOT recalibrate dispatch axis thresholds (carries forward as priority #4).
5. Does NOT address X5 — deferred again (carries to cycle 161+ as forward priority #1).
6. Does NOT write the Missing Integration Scenario 3 super-step-out-of-order live integration test (newly unblocked but a future cycle's work).
7. Does NOT extend `validate_payload` in v2-channel-router to enforce marker value-shape (string + enum) — the router contract is presence-only by design; writers enforce shape.
8. Does NOT deduplicate the inbound-key inventory across v2-role-driver + v2-reconciler-event-processor + v2-channel-router (DEFERRED per cycle 149 §3.3 to v2-cycle-runner COMPLETE arc).
9. Does NOT add per-step timeout (X2) — deferred.
10. Does NOT add lock/lease (X1) — deferred.
11. Does NOT modify `docs/redesign/2-selection.md` (cycle 120 L2 preserved).
12. Does NOT close any issue beyond the cycle issue itself.
13. Does NOT dispatch any new Copilot work (no in-flight dispatches; nothing to wake on).
14. Does NOT alter the `Phase` enum or any cycle 159 Track 2 surface area.
15. Does NOT alter v2-state-audit or v2-state-retention-policy (cycles 158/159 surface area preserved).
16. Does NOT change the v2-channel-router schema output format (only the data values).
17. Does NOT touch the v2-prompt-contract-check internal `sample_schema` (the live integration test uses the real binary, not the sample).
