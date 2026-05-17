# Cycle 164 — Track 1: v2-state-dispatch-archive Copilot dispatch (first Track-1 dispatch application of #2937) + Track 2: v2-channel-router enforcement extension design scope

**Authoring cycle:** 164 (2026-05-17).
**Mode:** redesign (phase-3 prototype + dispatch).
**Forward priorities source:** [`cycle-163-state-dispatch-archive-design-scope-and-super-step-out-of-order-live-test.md`](cycle-163-state-dispatch-archive-design-scope-and-super-step-out-of-order-live-test.md#forward-priorities-for-cycle-164).
**Predecessors:** cycle 163 [`v2-state-dispatch-archive.md`](v2-state-dispatch-archive.md) design scope (Track 1 dispatch target); cycle 148 [`cycle-148-two-track-absorption-and-landing.md`](cycle-148-two-track-absorption-and-landing.md) C1/L3.1 absorption verdicts (Track 2 origin).

## Cycle shape

Two-track composition continues. Cycle 162 was straight-pair NOVEL@1; cycle 163 was straight-pair RECURRENCE-AT-2 (both tracks in main). **Cycle 164 is straight-pair RECURRENCE-AT-3 with a SUBSTRATE VARIANT: Track 1 is a Copilot dispatch, Track 2 is main-side design scope.** The two-track HARDENING extends to **HARDENING-AT-17** (13 consecutive post cycle 151 single-track exception).

Track 1 closes a forward priority via dispatch rather than local implementation — the FIRST Track-1 dispatch application of Eva directive [#2937](https://github.com/EvaLok/schema-org-json-ld/issues/2937) since the directive landed 2026-05-14. The directive named "parallel SCAFFOLD work with already-scoped design" as a structural opportunity; v2-state-dispatch-archive implementation (with cycle 163's 260-line design scope and v2-state-audit 1260 LOC precedent crate) is the strongest dispatch fit in the current priority list.

Track 2 closes a long-standing C1/L3.1 design-scope priority that had been carrying forward since cycle 148 (4 cycle-numbered carry-forwards: 161 #5 → 162 #5 → 163 #4).

## Track 1 — v2-state-dispatch-archive Copilot dispatch

**Closes cycle 163 forward priority #1** (was cycle 162+ priority #2 from `v2-state-dispatch-archive.md` §8 priority #1; ~1000 LOC implementation per cycle 163 design scope).

Dispatched via `tools/dispatch-task` (single command form, --skip-pipeline-gate per redesign mode, --label agent-task --label implementation):

```
tools/dispatch-task \
  --title "[redesign-impl] v2-state-dispatch-archive crate (cycle 164 per cycle 163 design scope)" \
  --body-file .scratch/cycle164-dispatch-body.md \
  --label agent-task \
  --label implementation \
  --skip-pipeline-gate
```

Result: issue [#2974](https://github.com/EvaLok/schema-org-json-ld/issues/2974) created; `docs/state.json` agent_sessions[] receipt recorded via the dispatch-task subprocess (commit `0032c141`, pushed to master immediately per cycle 524 atomic-commit-and-push discipline); Copilot assigned 2026-05-17T03:09:31Z and `connected` event fired 2026-05-17T03:09:41Z (10s post-assignment, within expected ~10-15s window per cycle 92 verification).

**Dispatch issue body composition** (`.scratch/cycle164-dispatch-body.md`, ~280 lines):
- Section 1 (Context): names directive #2937 item #1 (parallel SCAFFOLD work), the Phase 3 axis 6 state-bound-exceeded situation, and the cycle 161 §4.2 Option B selection that produced the design scope.
- Section 2 (Reading list, ordered): the 260-line design scope as authoritative spec; v2-state-audit `src/main.rs` as precedent shape (1260 LOC, single-file pattern); the Cargo.toml dependency pattern; the workspace Cargo.toml (no change needed); cycle 158 retention-policy §4 Axis 6 for threshold context; cycle 161 policy-enforcement scope for sequencing context.
- Section 3 (What to build): crate layout, Cargo.toml template (with `sha2` added for §6.2 hash check; `tempfile` as dev-dep for integration tests), CLI per §5.1, status whitelist constant per §3.1, age-computation flow per §3.2/3.3, archive-file schema per §4, atomicity ordering per §6.1 (step 4 BEFORE step 6), hash-check per §6.2, lock-file per §6.3, exit codes per §5.2, JSON output schema per §5.3.
- Section 4 (Tests): inline unit tests (~25-40 tests covering status enum, age computation, CSV parsing, age-days-0 behavior, max-entries bound, filename formatting, hash determinism) + 3 integration tests minimum (happy-path, dry-run, hash-mismatch deterministic-via-unit-test).
- Section 5 (Specific behaviors): 7 invariants quoted from the design scope (§3.1 unknown-status-ineligible, §3.3 missing-both-timestamps-ineligible, §4.1 invocation-date filename, §4.3 same-day append, §4.4 archive_version field, §6.1 ordering invariance, §6.2 hash-check mandatory).
- Section 6 (Anti-patterns to avoid): 6 explicit anti-patterns from §9 of the design scope.
- Section 7 (What's already done): scaffolding context — workspace already includes `crates/*`; v1 record-dispatch frozen; v2-state-audit untouchable; docs/state-archive/ does not exist (tool creates); 930 agent_sessions[] entries (922 archivable by status).
- Section 8 (Verify before PR): cargo build/test/clippy + dry-run smoke against real state.json (predicting ~7-week-old merged entries dominate so most archive).
- Section 9 (Direct-push zones): notes the crate is NOT in a redesign direct-push zone — PR required for review, similar to cycle 147 PR #2953 absorption pattern.

**Expected return:** PR opened by Copilot within hours/days. Main-orchestrator's absorption cycle (cycle 165+) will run the same per-finding verdict pattern as cycle 148 PR #2953 absorption and cycle 155 PR #2961 absorption — review against scope, run cargo build/test/clippy on the PR, identify any deviations from scope, accept/carveout/disagree per finding, land if acceptable.

## Track 2 — v2-channel-router enforcement extension design scope

**Closes cycle 163 forward priority #4** (was cycle 162 priority #5, was cycle 161 priority #5; originally C1 + L3.1 AGREE-RECORD + TOOL-SCOPED from cycle 148 absorption — 16 cycles carry-forward from origin to closure).

Authored as new canonical file [`docs/redesign/_notes/v2-channel-router-enforcement-extension.md`](v2-channel-router-enforcement-extension.md) (new, 279 lines). Single direct-push commit [`b92cd258`](https://github.com/EvaLok/schema-org-json-ld/commit/b92cd258).

Architecture decisions captured:

1. **§2 tool-boundary discipline** — extension to existing `v2-channel-router` + `v2-prompt-contract-check`, not a new crate. Channel-router gains a richer schema API; contract-check extends the comparison.

2. **§3 schema source-of-truth** — Option B (`Channel::payload_schema() -> &'static PayloadSchema` new method) recommended over Option A (in-place change to `required_payload_keys()` return type), Option C (external JSON files), Option D (prompt-XML-derived). Option B preserves backwards-compat via thin wrapper, allows incremental migration, no startup cost.

3. **§4 in-house type vocabulary** — `PayloadType` enum (String/Integer/Number/Boolean/Array/Object) + `PayloadKey { name, ty, sub_keys }` struct with `&'static` discipline (no allocation, no startup parse). One-level nesting only this scope (§4.3 — matches today's prompts; defer deeper recursion to a future cycle if drift surfaces). Optional-key bucket via sibling `optional_payload_keys()` method (§4.4). Null-is-not-missing strict discipline (§4.5).

4. **§4.6 rejection of JSON Schema dialect adoption** — bounded surface (4 channels, ~10 keys total) does not justify the maintenance + dependency footprint. Mechanical migration path noted if surface grows (revisit at 10+ channels).

5. **§5 dual enforcement points** — write-time via extended `validate_payload` (router-side, runtime) + static via extended `v2-prompt-contract-check` comparison (operator-time / CI). Both consume the same `payload_schema()` source-of-truth.

6. **§5.3 `schema --format json` extension** — bump `schema_format_version` to `2`; v2-prompt-contract-check's parser handles format-v2. Backward-compat reading allowed; format-v1 emission deprecated.

7. **§5.4 `--mode strict|lenient` rollout flag** — lenient cycle-1 surfaces real type mismatches via stderr log without rejecting; strict mode default after honesty-pass on prompts has landed. Phased rollout per §6.4.

8. **§6 migration plan** — 3 cycle-165+ priorities (router extension + contract-check extension + prompts honesty-pass for cycle 148 L3.2 cargo-culted overclaims) + verify pass (cycle 165+ priority #4) + eventual deprecation of `required_payload_keys()` (cycle 166+ priority #5).

9. **§7 ordering discipline** — router-side lands FIRST (contract-check needs schema-format-v2); prompts honesty-pass AFTER both tools (pass reads the extended schema to know what's now enforceable).

10. **§9 anti-patterns rejected** — runtime coupling with static check, JSON Schema dialect for bounded surface, element-type enforcement in arrays (deferred), strict-from-day-one rollout (real cycle traffic may contain silent type mismatches), deep-nesting recursion beyond one level (YAGNI).

11. **§10 11 explicit non-doings** — including not building the extension this cycle, not modifying prompts, not adopting JSON Schema, not enforcing array element-types, not deprecating `required_payload_keys()` yet.

### Track 2 forward priorities produced (§8)

5 forward priorities:
1. v2-channel-router enforcement extension implementation (~250-400 LOC router-side).
2. v2-prompt-contract-check extension implementation (~100-200 LOC contract-check-side; sequenceable after #1 since it needs schema-format-v2).
3. Honesty-pass on v2 role prompts (cycle 148 L3.2 cargo-culted overclaims; per-case fix-prompt-OR-upgrade-router decision).
4. `v2-prompt-contract-check --strict` re-run post-extension (verify 4/4 prompts contract-aligned at extended comparison level).
5. Deprecate `Channel::required_payload_keys()` (post-migration; cycle 166+).

§8 priorities #1-2 are themselves strong Copilot-dispatch candidates per directive #2937 (this scope is now the pre-scope; v2-channel-router is the precedent crate). Naming this for cycle 165+ forward-priority consideration.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 dispatch issue body `.scratch/cycle164-dispatch-body.md` | ~280 lines (transient) | (passed to dispatch-task) |
| Track 1 dispatch receipt commit | (auto) | `0032c141` |
| Track 2 design scope `v2-channel-router-enforcement-extension.md` | 279 lines (new) | `b92cd258` |
| `cycle-164-*.md` (_notes) | this file (~265 lines) | cycle-close |
| Total cycle 164 textual output | ~825 lines | 2 substantive + 1 cycle-close |

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 164 datapoints:
- **Track 2 design-scope-textual:** 279 lines (target was 200-260; +19 above upper bound, ~7.3% overshoot). Slightly above cycle 163 v2-state-dispatch-archive.md (260 lines, ~4% overshoot) and cycle 158 v2-state-retention-policy.md (244). Drivers: §3 enumerated 4 source-of-truth options with rejection rationale; §4 enumerated type-system details (5 sub-sections); §6 migration-plan covered 4 phases of rollout. The size shape is consistent: 260-280 for scope-with-options-enumeration design-scope-textual.
- **Track 1 dispatch issue body:** ~280 lines — first measurement for "dispatch-issue-body" shape under directive #2937 Track-1 application. Precedent #2952 (cycle 147 v2-prompt-contract-check dispatch) was similarly sized; this confirms the shape is reproducible for the implementation-dispatch class.

## Pattern updates this cycle

- **`directive-2937-track-1-dispatch-fit-application`** NOVEL@1 cycle 164. First cycle where Track 1's substantive focal was a Copilot dispatch rather than a main-side implementation/design. Surfaces a new composition variant: dispatch-as-track-1-with-main-side-track-2. Recurrence test: any cycle where a forward priority is dispatched as Track 1 rather than implemented in main, where the directive #2937 fit-criteria are met (already-scoped design, precedent crate, reviewable-against-scope, parallelism with main-side substrate work).

- **`straight-pair-with-dispatch-variant`** NOVEL@1 cycle 164. Cycle 163 was straight-pair (both tracks in main); cycle 164 is straight-pair WITH Track 1 dispatched. The variant is a sub-shape of straight-pair (two unrelated priorities, no coupling) with the dispatch axis as the new dimension. Recurrence test: any straight-pair where one track is dispatch and one is main-side.

- **`straight-pair-closure-as-default-two-track-shape` HARDENING-AT-3** cycle 164 (162, 163, 164). Cycle 162 NOVEL@1, cycle 163 RECURRENCE-AT-2, cycle 164 HARDENING-AT-3. Now load-bearing as the modal two-track composition shape across the recent arc.

- **`two-track-composition` HARDENING-AT-17** cycle 164. 13 consecutive post cycle 151 exception (152-164).

- **`carry-forward-from-cycle-N-absorption-verdict`** RECURRENCE-AT-2 cycle 164. Cycle 162 Track 2 closed L2.4 from cycle 148 absorption (dispatch-brief-discipline addendum); cycle 164 Track 2 closes C1/L3.1 from cycle 148 absorption (channel-router enforcement extension). Pattern: long-running TOOL-SCOPED absorption findings produce design-scope-textual closures cycles later. Recurrence test: any cycle that closes a cycle-148 absorption finding via design-scope authoring.

- **`tool-extraction-surfaces-prior-cycle-errors` NOT-EXERCISED** cycle 164. Both tracks build forward substrate, do not extract a tool from a procedure. Carries forward at RECURRENCE-AT-6 from cycle 162.

- **`agree-act-now-bounded-fix-via-tracksecond-pattern` NOT-EXERCISED** cycle 164. Neither track closes AGREE-ACT-NOW residue. Carries forward at RECURRENCE-AT-7 from cycle 162.

- **`pre-flight-vs-mid-cycle-halt-distinction` NOT-EXERCISED** cycle 164. Neither track exercises a halt scenario. Carries forward at RECURRENCE-AT-2 from cycle 163.

- **`fail-open-on-tool-internal-anomalies-but-fail-closed-on-policy-violations` NOT-EXERCISED** cycle 164. Carries forward at NOVEL@1 from cycle 162.

- **`boundary-error-text-classifier-mismatch-found-via-test-implementation` NOT-EXERCISED** cycle 164. Carries forward at NOVEL@1 from cycle 163.

- **`agree-defer-priority-becomes-live-after-blocker-clears` NOT-EXERCISED** cycle 164. Carries forward at NOVEL@1 from cycle 163.

## Process honoring

- **49th consecutive cycle of HONORING named forward priority** (cycles 115-164). Cycle 163 named #1 (state-dispatch-archive implementation, Track 1 via dispatch) + #4 (channel-router enforcement extension design scope, Track 2); cycle 164 closes both.
- **77th bottleneck-asynchronous cycle** (78-164).
- **54th non-per-candidate-sharpening cycle** (111-164).
- Cycle 120 L2 preserved (`2-selection.md` untouched cycle 164).
- Cycle 128 lesson preserved (.scratch/ via Write tool — 3 files this cycle: session-start, dispatch-body, track2-commit-msg).
- **Cycle 133 lesson application:** 1 cargo invocation this cycle (build only). Neither track modified Rust source — Track 1 is dispatch (Copilot will write code in a PR), Track 2 is design-doc only. Per cycle 133 "no gratuitous cargo on non-Rust cycles," 1 conservative cargo build for workspace health is the discipline; 3 (build + test + clippy) would have been gratuitous. **Deviation from cycle 163 which ran all 3 — but cycle 163 modified Rust source via Track 2 +2 LOC + 1 integration test, so 3 cargo invocations was warranted there.** Cycle 164 is the first cycle in the recent arc where 0-or-1 cargo invocations is correct per discipline; recorded as `non-rust-cycle-cargo-discipline` clarification.
- Cycle 134 lesson preserved (audit-repo HEAD via gh api at session-start; unchanged at `8285b7d3` cycle 164).
- **Cycle 137 lessons re-validated cycle 164 18-cycle-running** (137 + 147-164). All `gh` operations single-purpose-bash-invocation form.
- **Cycle 149 in-cycle CWD-drift lesson re-validated cycle 164** — single cargo invocation via `--manifest-path tools/rust/Cargo.toml`.
- Cycle 151 date-test-value lesson preserved (no new date tests).
- Cycle 152 disk-format-read-source lesson preserved.
- Cycle 153 multi-Edit-fallback lesson preserved (single Edit on commit msg).
- **Cycle 154 `gh api graphql` subprocess pattern preserved** — Track 1 dispatch via `tools/dispatch-task` (which calls the graphql replaceActorsForAssignable mutation internally per the documented dispatch-method); cycle 164 is the FIRST cycle since cycle 154 to exercise the pattern via a Track-1 dispatch (cycles 155-163 had 0 dispatches per Eva directive #2937 observation). Pattern operational; Copilot connected within 10s per cycle 92 verification.
- Cycle 155 dispatch-return-detection lesson preserved (the dispatched #2974 will return as a PR; absorption cycle TBD).
- Cycle 156 anti-overstatement audit enumeration discipline preserved (cycle 164 _notes has explicit "What cycle 164 does NOT do" with 13 items below).
- Cycle 157 audit-HEAD-check at session-start preserved.
- Cycle 158 / 159 / 160 / 161 / 162 / 163 closure pattern progression — straight-pair HARDENING-AT-3.
- Cycle 159 test-pattern-mirror lesson preserved (Track 2 design scope mirrors cycle 163 v2-state-dispatch-archive.md structure: 11-section layout, predecessor-citations, in-scope/out-of-scope split, options-enumeration with rejection rationale, forward-priorities-produced, anti-patterns rejected, explicit non-doings).
- Cycle 160 schema-duplication observation preserved (Track 2 §4 chose in-house vocabulary over JSON Schema dialect — single source-of-truth at `Channel::payload_schema()`).
- Cycle 161 paired-closure pattern preserved (cycle 164 is straight-pair-with-dispatch-variant, structurally different from cycle 161 paired-closure).
- Cycle 162 state-audit session-start wiring preserved (not exercised cycle 164; session-start v2-state-audit invocation runs per cycle 162 design).
- Cycle 163 implementation-discovery via testing lesson preserved as PATTERN (NOVEL@1 carrying forward); not exercised cycle 164.
- Journal-immutability discipline preserved (cycle 164 APPENDS to `docs/journal/2026-05-17.md` which cycle 163 created; cycle 163's section NOT back-edited).
- **Two-track composition continued** — cycle 164 is 13th consecutive post cycle 151 exception (HARDENING-AT-17).
- **SECTION 6b list housekeeping NOT invoked cycle 164** — 6 currently-open issues (cycle issue + 4 standing input-from-eva + dispatch issue #2974); 0 open PRs (PR from #2974 not yet returned).

## In-session issues and recoveries

- **Zero in-session test-failure recoveries cycle 164.** No Rust code modified.
- All `git add` / `git commit` / `git push` operations clean cycle 164.
- All Edit / Write operations clean cycle 164.
- All `gh api` / `gh issue list` / `gh issue comment` / `tools/dispatch-task` operations clean cycle 164.
- The pre-cycle-issue listing showed `agent_sessions[]` at 930; the post-dispatch state shows 931 (the new dispatch entry recorded by `tools/dispatch-task`). Within expected behavior.

## Cycle 164 ARTIFACTS

- `docs/redesign/_notes/v2-channel-router-enforcement-extension.md` — new (Track 2, 279 lines, commit `b92cd258`).
- `docs/state.json` — modified by `tools/dispatch-task` subprocess (Track 1, +1 agent_sessions entry for #2974, commit `0032c141`).
- `docs/redesign/_notes/cycle-164-state-dispatch-archive-dispatch-and-channel-router-enforcement-extension.md` — new (this file, ~265 lines, cycle-close).
- `docs/journal/2026-05-17.md` — appended (cycle 164 section; cycle 163 section preserved).
- `.scratch/cycle164-*.{md,txt}` — ephemerals (session-start, dispatch-body, track2-commit-msg, session-end, issue-close).
- 2 substantive commits: `0032c141` (Track 1 dispatch receipt) + `b92cd258` (Track 2 design scope).
- 1 cycle-close commit (this _notes + journal section + ephemerals).
- 0 issues closed cycle 164 (no candidates; cycle issue itself closes per existing convention; dispatched #2974 is operationally-live).
- **1 Copilot dispatch cycle 164** — issue [#2974](https://github.com/EvaLok/schema-org-json-ld/issues/2974) for v2-state-dispatch-archive implementation (first Track-1 dispatch since cycle 137-ish; first under directive #2937 explicit Track-1 framing). Copilot connected `2026-05-17T03:09:41Z`.
- 1 cargo invocation cycle 164 (build; non-Rust cycle so test + clippy gratuitous).
- ~7 GitHub API operations cycle 164 (audit HEAD, input-from-eva list, open issues + PR list, session-start comment, dispatch-task issue creation + assignment + state.json update, dispatch-event verification).
- 1 Edit on `.scratch/cycle164-track2-commit.txt` (size correction).
- 0 Edits on Rust source.
- 0 Edits on existing v2 prompts or workflow YAML.
- 0 Edits on this orchestrator prompt.
- 1 Write of `docs/redesign/_notes/v2-channel-router-enforcement-extension.md` (Track 2 new file).
- 1 Write of dispatch issue body (transient, .scratch/).
- 0 Edits on `2-selection.md` (cycle 120 L2 preserved).

## What cycle 164 does NOT do

1. Does NOT build v2-state-dispatch-archive in main (Track 1 dispatched to Copilot as #2974; absorption cycle 165+).
2. Does NOT build the v2-channel-router enforcement extension this cycle (Track 2 design scope only; cycle 165+ priorities #1-2 in Track 2 §8).
3. Does NOT modify any v2 role prompt (cycle 148 L3.2 honesty-pass is cycle 165+ priority #3 in Track 2 §8; gated on extension landing).
4. Does NOT modify v1 record-dispatch, v1 prompts, or any frozen-zone surface.
5. Does NOT run a backlog archival against state.json (gated on Track 1 PR returning + absorbing + landing in main).
6. Does NOT patch v2-state-retention-policy.md §4 Axis 6 thresholds (cycle 165+ priority #3 in cycle 163 §8 inheritance).
7. Does NOT modify v2-cycle-runner, v2-state-audit, v2-state-dispatch-sync, v2-channel-router, v2-prompt-contract-check, or any other existing v2 crate (Track 2 is design only; cycle 165+ implementation will).
8. Does NOT advance Phase 2 candidate selection (`2-selection.md` untouched cycle 164; per cycle 120 L2).
9. Does NOT extend the halt-class catalog (cycle 162 extended to 5 entries; cycle 164 does not exercise any halt class).
10. Does NOT engage audit-repo cross-repo critique (audit HEAD unchanged at `8285b7d3` since 2026-05-16; no audit-engagement single-track variant exercised cycle 164).
11. Does NOT modify cycle 163's _notes or journal section (journal-immutability discipline preserved).
12. Does NOT mark or close [#2937](https://github.com/EvaLok/schema-org-json-ld/issues/2937) — the directive is standing (continues to apply to cycle 165+); cycle 164 honors it via Track 1 application, does not retire it.
13. Does NOT run cargo test or clippy this cycle (cycle 133 gratuitous-cargo discipline on non-Rust cycle; build was a conservative workspace-health check).

## Forward priorities for cycle 165+

Inheriting from cycle 163's renumbered list, minus #1 + #4 (closed cycle 164), plus new sub-priorities from Track 2 §8.

1. **Absorb Track 1 dispatch PR for v2-state-dispatch-archive** (NEW cycle 165+): when [#2974](https://github.com/EvaLok/schema-org-json-ld/issues/2974) Copilot dispatch returns as a PR, run the cycle 148 / cycle 155 per-finding absorption pattern: cargo build/test/clippy on the PR; review against the cycle 163 design scope; per-finding accept / carveout / disagree verdicts; land if acceptable. Magnitude: variable depending on PR size + deviation count; estimate 1 cycle for absorption + landing if 0-3 deviations.
2. **Backlog archival run** (was cycle 163+ priority #2): one-time invocation of v2-state-dispatch-archive against the live 931-entry agent_sessions[] backlog. Gated on priority #1 landing.
3. **v2-state-retention-policy.md §4 Axis 6 recalibration patch** (was cycle 163+ priority #3): apply cycle 161 §4.1 live+total split thresholds. Gated on priorities #1-2.
4. **v2-channel-router enforcement extension implementation** (NEW from Track 2 §8 #1; Copilot-dispatch candidate per directive #2937, parallel to Track 2 main-side substrate). ~250-400 LOC router-side.
5. **v2-prompt-contract-check extension implementation** (NEW from Track 2 §8 #2; sequenced after #4). ~100-200 LOC contract-check-side.
6. **TOOL-SCOPED `v2-prompt-tag-semantic-fidelity` tool design scope** (was cycle 163 priority #5). Carries forward.
7. **`status` + `verify` v2-cycle-runner subcommands** (was cycle 163 priority #6). Carries forward.
8. **AGREE-DEFER queue (post-real-role-session-measurement)** (was cycle 163 priority #7). Hold for live-claude-code-spawn evidence (still SCAFFOLD).
9. **Coordinated retry/timeout/cancellation arc** — C13 + X2 (was cycle 163 priority #8).
10. **Coordinated structured-error-envelope arc** — C6 + C7 + C9 (was cycle 163 priority #9).
11. **Coordinated resume/recovery arc** — C11 + C12 + X1 (was cycle 163 priority #10).
12. **Audit-engagement substantive-focal single-track variant** (was cycle 163 priority #11). Gate = audit HEAD changes.
13. **Per-axis archival mechanism design scope** (was cycle 163 priority #12). Gate-bounded.
14. **Honesty-pass on v2 role prompts** (NEW from Track 2 §8 #3; cycle 148 L3.2 cargo-culted overclaims). Gated on priorities #4-5 landing.
15. **`v2-prompt-contract-check --strict` re-run post-extension** (NEW from Track 2 §8 #4). Gated on priorities #4-5 + #14.
16. **Deprecate `Channel::required_payload_keys()` legacy method** (NEW from Track 2 §8 #5; cycle 166+ migration). Gated on priorities #4-5 + #14-15.

**Cycle 164 forward priorities CLOSED:**
- Cycle 163 priority #1 (v2-state-dispatch-archive implementation, Track 1 via dispatch).
- Cycle 163 priority #4 (v2-channel-router enforcement extension design scope, Track 2).

**Cycle 164 new sub-priorities:** 1 net new from Track 1 (dispatch-PR absorption, priority #1 above) + 5 net new from Track 2 §8 (priorities #4-5, #14-16). Priority #4 (channel-router extension implementation) is the strongest Copilot-dispatch candidate in the new priority list per directive #2937 — same fit-criteria as cycle 164 Track 1 (pre-scoped design, precedent crate, parallelizable with Track 2 substrate). Cycle 165+ should consider it for Track-1 dispatch.
