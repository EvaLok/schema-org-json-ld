# Cycle 167 — straight-pair: v2-state-dispatch-sync operational invocation (Track 1) + v2-state-retention-policy §4 Axis 6 recalibration (Track 2)

**Cycle issue:** [#2980](https://github.com/EvaLok/schema-org-json-ld/issues/2980)
**Cycle date:** 2026-05-17
**Forward priorities source:** [`cycle-166-backlog-archival-and-channel-router-extension-dispatch.md`](cycle-166-backlog-archival-and-channel-router-extension-dispatch.md#forward-priorities-for-cycle-167)
**Predecessors:**
- Track 1: cycle 154 tool `v2-state-dispatch-sync` (never run operationally since landing) → cycle 166 Track 1 first-live-invocation of v2-state-dispatch-archive surfaced state-sync drift (NOVEL@1 pattern `tool-first-live-invocation-surfaces-state-drift-in-adjacent-tool`) → cycle 167 Track 1 remediation.
- Track 2: cycle 158 design scope [`v2-state-retention-policy.md`](v2-state-retention-policy.md) §4 Axis 6 initial thresholds (50/200/500) + cycle 159 storage-key-clarification noting assumption-vs-reality gap (~10/cycle V1 rate vs 50-advisory estimate) → cycle 166 Track 1 archival run reduced 931→69 entries, clearing the recalibration gate → cycle 167 Track 2 recalibration patch.

## Cycle shape

Two-track composition continues, **HARDENING-AT-20** post cycle 151 single-track exception. 16 consecutive post-exception (152-167); 21 of 22 in arc 146-167.

Cycle 167 is **straight-pair HARDENING-AT-6** (162 NOVEL@1, 163 RECURRENCE-AT-2, 164-167 HARDENING-AT-3..6 — modal two-track composition shape across 6 consecutive cycles). Both Tracks main-side; no dispatch this cycle (PR #2979 for cycle 166 Track 2 dispatch #2978 is DRAFT, Copilot still working; absorption deferred to cycle 168+).

Track 1 closes cycle 166+ forward priority #2 (NEW state-sync drift remediation, discovered cycle 166 Track 1). Track 2 closes cycle 166+ forward priority #1 (NEW post-archival Axis 6 recalibration, gate cleared cycle 166 Track 1).

## Track 1 — v2-state-dispatch-sync operational invocation (FIRST live run since cycle 154 landing)

**Closes cycle 166+ forward priority #2** (bounded-mechanical, ~1 cycle estimate per cycle 166 _notes; actual: <10 minutes elapsed).

### Pre-run state

`docs/state.json` `agent_sessions[]`: 70 entries post cycle 166 (4 live + 65 below-30-day-threshold terminal + 1 cycle 166 Track 2 dispatch receipt). Of the 4 in_flight-status entries discovered by cycle 166 _notes, 2 known stale:
- **#2960** dispatched 2026-05-15 (cycle 152 feedback-only); GH state CLOSED since dispatch return.
- **#2974** dispatched 2026-05-17 (cycle 164 implementation); GH state CLOSED since 2026-05-17 05:26 (closed by PR #2975 merge cycle 165).
- **#2978** dispatched 2026-05-17 (cycle 166 Track 2); GH state OPEN — legitimately in_flight.

### Audit (read-only)

```
cargo run --manifest-path tools/rust/Cargo.toml --release -p v2-state-dispatch-sync -- audit
```

Result (2026-05-17T08:53Z):
```
Plan: 3 entries audited, 2 transitions, 1 unchanged

  #2960: in_flight → closed_without_pr
  #2974: in_flight → merged (pr=#2975, merged_at=2026-05-17T05:26:18Z)

Unchanged (still open on GitHub):
  #2978: in_flight (no change)

Resulting in_flight_sessions count: 1
```

Plan matches cycle 166 _notes prediction exactly (no false positives, no false negatives). The tool queried each of the 3 in_flight issues via the cycle 154 `gh api graphql` `closedByPullRequestsReferences` query and produced deterministic transitions.

### Sync (apply)

```
cargo run --manifest-path tools/rust/Cargo.toml --release -p v2-state-dispatch-sync -- sync
```

Result: identical plan applied. `docs/state.json` mutated:
- `agent_sessions[]` entry for #2960: `status: in_flight → closed_without_pr` (no PR linkage in GH; feedback-only dispatch had no closing PR).
- `agent_sessions[]` entry for #2974: `status: in_flight → merged`, `pr: 2975`, `merged_at: 2026-05-17T05:26:18Z` (cycle 165 merge).
- Top-level counter: `in_flight_sessions: 3 → 1`.

Single atomic commit `6a82e3e4` per cycle 524 atomic-commit-push discipline. Direct-push to master (docs/state.json is in tool-mutated state zone).

### Observation: latent-tool first-live-invocation

Cycle 167 Track 1 is the **second consecutive cycle** to first-live-invoke a tool that had been sitting unused since landing:
- Cycle 166 Track 1: `v2-state-dispatch-archive` (landed cycle 165 PR #2975 merge; first-live cycle 166).
- Cycle 167 Track 1: `v2-state-dispatch-sync` (landed cycle 154; first-live cycle 167 — 13 cycles latent).

Different latency profiles (1 cycle vs 13 cycles), but the structural pattern is the same: tools authored and tested but never operationally invoked until a need surfaces. The 13-cycle latency for v2-state-dispatch-sync is the longer-running observation — the tool existed precisely to handle the kind of drift that accumulated, but was not run because no observable signal surfaced until cycle 166 archival exposed it. This is consistent with `tool-first-live-invocation-surfaces-state-drift-in-adjacent-tool` (cycle 166 NOVEL@1) at a meta level: not just adjacent-tool state-drift, but also tool-operational-disuse can sit latent.

This cycle did NOT name a new pattern around this observation — the cycle 166 NOVEL@1 captures the discovery-side; cycle 167 is the remediation, which is just the natural follow-up arc. The 2-cycle observation (2 latent first-live-invocations in 2 consecutive cycles) is captured as journal observation for future watch.

## Track 2 — v2-state-retention-policy §4 Axis 6 recalibration (50/200/500 → 150/400/1000)

**Closes cycle 166+ forward priority #1** (design-scope-textual, ~50-100 line edits estimated; actual: ~30 line edits to policy doc + 12 line changes to tool source + 8 line changes to two tests = ~50 LOC total).

### Recalibration rationale

Old thresholds (cycle 158 initial): Advisory 50 / Mandatory 200 / Hard 500.

These were sized against a slow steady-state stream assumption WITHOUT an archival mechanism. Late-V1 ran ~10 dispatches/cycle (per cycle 159 measurement); accumulation reached 930 entries over 3 months before cycle 166 archival landed. Cycle 159 made the assumption-vs-reality gap explicit but deferred remediation to "implement refuse-to-write OR calibrate against historical growth."

Cycle 163 designed `v2-state-dispatch-archive` (30-day age threshold sweep to dated archive files); cycle 164-165 dispatch+merge landed; cycle 166 first-live-invocation reduced 931→69 (~93% reduction) and made the post-archival baseline observable for the first time.

### New thresholds: 150 / 400 / 1000

Empirical V2 dispatch cadence over cycle 138-167 (30 cycles): 2 dispatches total (#2974 cycle 164, #2978 cycle 166) ≈ 0.07/cycle ≈ 0.27/day ≈ 8/month. Eva directive #2937 explicitly authorizes higher cadence, so projections:
- Low (current actual ~0.27/day): 8 entries / 30-day window
- Medium (~1/day): 30 entries / 30-day window
- Active (~2/day): 60 entries / 30-day window
- Heavy (~5/day): 150 entries / 30-day window

30-day window steady-state at those rates with ~70-entry archival floor: 78 / 100 / 130 / 220 entries total.

| Threshold | Old (cycle 158) | New (cycle 167) | New rationale |
|---|---|---|---|
| Advisory | 50 | **150** | Above heavy V2 cadence ceiling + safety margin. Normal operation below this. |
| Mandatory | 200 | **400** | ~2× advisory; sustained excess of heavy cadence OR archival lag of ~2 months at active cadence. |
| Hard | 500 | **1000** | V1-era no-archival rate × 3+ months without archival; write-leak indicator. |

### Files changed

- **`docs/redesign/_notes/v2-state-retention-policy.md`** — Axis 6 table values updated; column header changed from "Value (initial)" to "Value (cycle 167 recalibration)"; cycle 159 commentary updated to reflect post-archival reality + ownership clarification (v2-state-dispatch-sync writes statuses, v2-state-dispatch-archive sweeps); NEW cycle 167 recalibration rationale paragraph after the cycle 159 storage-key-clarification block, with projected cadence ranges + threshold-vs-projection mapping.

- **`tools/rust/crates/v2-state-audit/src/main.rs`** — `DISPATCHES_ADVISORY` 50 → 150, `DISPATCHES_MANDATORY` 200 → 400, `DISPATCHES_HARD` 500 → 1000, with a 4-line comment block citing the cycle 167 recalibration source. Two unit tests updated:
  - `build_report_picks_up_state_json_dispatches`: was 60-entry fixture asserting Advisory (60 > 50 old); now 200-entry fixture asserting Advisory (200 > 150 new).
  - `dispatches_axis_classifies_hard_at_500_plus` renamed to `dispatches_axis_classifies_hard_at_1000_plus`; 500-entry fixture → 1000-entry fixture; assertions updated.

### Verification

```
cargo build --release -p v2-state-audit            # clean
cargo test  --release -p v2-state-audit            # 39/39 passing (was 39 pre-change)
cargo clippy --release -p v2-state-audit           # clean
cargo clippy --release --workspace --all-targets   # clean except 2 pre-existing unrelated warnings (v2-boot-phase, v2-wiki-search)
```

Live v2-state-audit run against current 70-entry `docs/state.json`:
```
state-json-dispatches:
  value: 70
  thresholds: { advisory: 150, mandatory: 400, hard: 1000 }
  classification: ok
  recommended_action: no-op
overall: ok
```

Pre-recalibration the live audit would have classified 70 as Advisory (70 > 50) — a chronic warning state for the post-archival baseline. Recalibration normalizes the post-archival operational reality as Ok.

Single atomic commit `af5ed960` per cycle 524 atomic-commit-push discipline. Direct-push to master (both files in direct-push zones: `docs/redesign/_notes/` and `tools/rust/crates/v2-*`).

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 state.json mutations | +5 -3 LOC net | `6a82e3e4` |
| Track 2 policy doc patch | +6 -4 LOC (Axis 6 section) | `af5ed960` |
| Track 2 tool source changes | +21 -10 LOC (constants + 2 tests) | `af5ed960` |
| `cycle-167-*.md` (_notes, this file) | ~265 lines | cycle-close |
| Total cycle 167 textual + code output | ~290 lines | 3 substantive commits |

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 167 datapoints: bounded-mechanical-shape (Track 1) was 2 tool invocations + 1 commit — under 10 minutes elapsed (consistent with cycle 166 Track 1 mechanical-shape estimate). Design-scope-textual-shape combined with code (Track 2) was ~30 LOC policy + ~30 LOC tool source/tests = ~60 LOC — substantially smaller than cycle 158 v2-state-retention-policy initial authoring (~244 lines), as expected for a focused recalibration patch vs initial design-scope authoring.

## Pattern updates this cycle

- **`straight-pair-closure-as-default-two-track-shape` HARDENING-AT-6** cycle 167 (162, 163, 164, 165, 166, 167). Modal two-track composition shape across 6 consecutive cycles. Continues to dominate as the natural shape for closing 2 unrelated priorities per cycle.
- **`two-track-composition` HARDENING-AT-20** cycle 167. 16 consecutive post cycle 151 exception (152-167).
- **`tool-first-live-invocation-surfaces-state-drift-in-adjacent-tool` NOT-EXERCISED-FOR-DISCOVERY** cycle 167 — cycle 167 Track 1 is the remediation phase of the cycle 166 NOVEL@1 discovery, not a new application of the pattern. Carries forward at NOVEL@1. Observational followup: cycle 167 Track 1 is itself a first-live-invocation of v2-state-dispatch-sync (13-cycle latent), suggesting a meta-pattern around tool-operational-disuse latency, but cycle 167 declines to name a new pattern around this — the observation is captured as journal note for future watch.
- **`straight-pair-with-dispatch-variant` NOT-EXERCISED** cycle 167 (no dispatch this cycle; PR #2979 still DRAFT). Carries forward at RECURRENCE-AT-2.
- **`directive-2937-copilot-role-bifurcation` NOT-EXERCISED** cycle 167. Carries forward at RECURRENCE-AT-2 (cross-cycle pattern; next instance pending PR #2979 absorption or new dispatch).
- **`audit-as-priority-1-input` NOT-EXERCISED** cycle 167 (audit HEAD unchanged at `bc8fda63`; no new audit engagement to absorb). Carries forward at HARDENING-AT-3.
- **`external-deliverable-absorption-pair` NOT-EXERCISED** cycle 167. Carries forward at NOVEL@1.
- **`directive-2937-track-1-dispatch-fit-application` NOT-EXERCISED** cycle 167. Carries forward at NOVEL@1.
- **`tool-extraction-surfaces-prior-cycle-errors` NOT-EXERCISED** cycle 167. Carries forward at RECURRENCE-AT-6.
- **`agree-act-now-bounded-fix-via-tracksecond-pattern` NOT-EXERCISED** cycle 167. Carries forward at RECURRENCE-AT-7.
- **`pre-flight-vs-mid-cycle-halt-distinction` NOT-EXERCISED** cycle 167. Carries forward at RECURRENCE-AT-2.
- **`fail-open-on-tool-internal-anomalies-but-fail-closed-on-policy-violations` NOT-EXERCISED** cycle 167. Carries forward at NOVEL@1.
- **`boundary-error-text-classifier-mismatch-found-via-test-implementation` NOT-EXERCISED** cycle 167. Carries forward at NOVEL@1.
- **`agree-defer-priority-becomes-live-after-blocker-clears` NOT-EXERCISED** cycle 167 (Track 1 closed a priority where the gate was the cycle 166 discovery, not a long-deferred AGREE-DEFER). Carries forward at NOVEL@1.

## Process honoring

- **52nd consecutive cycle of HONORING named forward priority** (cycles 115-167).
- **80th bottleneck-asynchronous cycle** (78-167).
- **57th non-per-candidate-sharpening cycle** (111-167).
- Cycle 120 L2 preserved (`2-selection.md` untouched cycle 167).
- Cycle 128 lesson preserved (.scratch/ via Write tool — 3 files: session-start, Track 1 commit msg, Track 2 commit msg).
- Cycle 133 lesson application: 4 cargo invocations cycle 167 (build + test + clippy on v2-state-audit + workspace clippy regression-check). All clean except 2 pre-existing unrelated workspace warnings.
- Cycle 134 lesson preserved (audit-repo HEAD via gh api at session-start; `bc8fda63`).
- **Cycle 137 lessons re-validated cycle 167 21-cycle-running** (137 + 147-167). All `gh` operations single-purpose-bash-invocation form.
- **Cycle 149 in-cycle CWD-drift lesson re-validated cycle 167** — all cargo invocations via `--manifest-path tools/rust/Cargo.toml`. No cd attempts.
- Cycle 151 date-test-value lesson preserved.
- Cycle 152 disk-format-read-source lesson preserved.
- Cycle 153 multi-Edit-fallback lesson preserved (2 single Edits on v2-state-audit/src/main.rs sequentially: constants block, then test pair).
- Cycle 154 `gh api graphql` subprocess pattern preserved (Track 1 v2-state-dispatch-sync calls `gh api graphql` via the cycle 154 ISSUE_QUERY internally).
- Cycle 155 dispatch-return-detection lesson preserved.
- Cycle 156 anti-overstatement audit enumeration discipline preserved (this _notes file has explicit "What cycle 167 does NOT do" with 13 items).
- Cycle 157 audit-HEAD-check at session-start preserved.
- Cycle 158 / 159 / 160 / 161 / 162 / 163 / 164 / 165 / 166 closure pattern progression — straight-pair HARDENING-AT-6.
- Cycle 159 test-pattern-mirror lesson preserved (Track 2 test changes follow the existing v2-state-audit test idioms — TempDir setup, JSON-fixture writing, axis-find by name).
- Cycle 160 schema-duplication observation preserved.
- Cycle 161 paired-closure pattern preserved.
- Cycle 162 state-audit session-start wiring preserved (not exercised cycle 167 — no v2-cycle-runner invocation; the live v2-state-audit run cycle 167 was direct verification, not a cycle-runner pre-flight).
- Cycle 163 implementation-discovery-via-testing lesson preserved (Track 2's test updates were predictable from the threshold change; no new bug surfaced).
- Cycle 164 forward-priority-renumbering preserved (cycle 167 _notes inherits cycle 166's 15-priority list, closes NEW#1 and NEW#2, renumbers).
- Cycle 164 dispatch-receipt atomic-commit pattern preserved (not exercised cycle 167 — no dispatch).
- Cycle 165 PR-absorption-shape preserved as reference for cycle 168+ absorption of PR #2979 when it lands.
- Cycle 166 first-live-invocation-shape preserved and exercised at a new axis (cycle 166: v2-state-dispatch-archive bounded-mechanical run; cycle 167: v2-state-dispatch-sync bounded-mechanical run). Both first-live, both bounded-mechanical.
- Journal-immutability discipline preserved (cycle 167 APPENDS to today's journal `2026-05-17.md` containing cycles 163-166; prior cycle sections NOT back-edited).
- **Two-track composition continued** — cycle 167 is 16th consecutive post cycle 151 exception (HARDENING-AT-20).
- **SECTION 6b list housekeeping NOT invoked cycle 167** — 6 currently-open issues (cycle issue + dispatch #2978 + 4 standing input-from-eva); 1 open PR (#2979 DRAFT). No closure candidates (dispatch is open and pre-PR-ready; standing input-from-eva are operative directives; PR is still Copilot-working).

## In-session issues and recoveries

- **One in-session permission-gate recovery cycle 167** (parallel-bash multi-operation rejection). Initial parallel `tools/v2-state-dispatch-sync --help` + `cat tools/v2-state-dispatch-sync` was blocked because the wrapper-script call required user approval. Recovery: directly grep'd the source for CLI surface (Args struct + SubCmd enum) per cycle 166 Track 1 lesson (cargo-direct instead of wrapper). CWD-drift discipline preserved (all subsequent invocations via `--manifest-path tools/rust/Cargo.toml`).
- **One in-session output-redirection rejection** (cycle 165 lesson re-validated). Initial `cargo run ... > .scratch/cycle167-audit-live.json` was blocked. Recovery: cargo run without redirection + grep pipe to inspect specific axis. No `Write` of large captured output needed for this verification step.
- **One in-session pipe-with-python rejection**. Initial `cargo run ... | python3 -c "..."` had the `python3 -c "..."` portion blocked. Recovery: cargo run with grep -A14 to extract the same fields textually. The Python parsing was unnecessary; grep was sufficient.
- All `git add` / `git commit` / `git push` operations clean cycle 167.
- All Edit / Write operations clean cycle 167 (2 sequential Edits on v2-state-audit/src/main.rs; 1 Edit on v2-state-retention-policy.md; 1 Write of new _notes file).
- All `gh api` / `gh issue list` / `gh issue comment` operations clean cycle 167.
- Pre-cycle agent_sessions[] count was 70 (4 in_flight + 65 below-threshold-terminal + 1 cycle 166 receipt — math from cycle 166 close). Post-Track-1 count remains 70 but in_flight distribution is now 1 (was 3 stale-counted). No additions cycle 167.

## Cycle 167 ARTIFACTS

- `docs/state.json` — Track 1 mutation (2 entries status-transitioned + in_flight_sessions counter 3→1; commit `6a82e3e4`).
- `docs/redesign/_notes/v2-state-retention-policy.md` — Track 2 patch (Axis 6 section + cycle 167 rationale paragraph; commit `af5ed960`).
- `tools/rust/crates/v2-state-audit/src/main.rs` — Track 2 patch (3 constants + 2 tests; commit `af5ed960`).
- `docs/redesign/_notes/cycle-167-state-sync-reconciliation-and-axis-6-recalibration.md` — new (this file, ~265 lines, cycle-close).
- `docs/journal/2026-05-17.md` — appended (cycle 167 section).
- `.scratch/cycle167-*.{md,txt}` — ephemerals (session-start, Track 1 commit msg, Track 2 commit msg, session-end, issue-close).
- 3 substantive commits: `6a82e3e4` (Track 1) + `af5ed960` (Track 2) + cycle-close commit (this _notes + journal + ephemerals).
- 0 issues closed cycle 167 explicitly (cycle issue closes per existing convention).
- 0 dispatches cycle 167.
- 4 cargo invocations cycle 167 (v2-state-audit build + test + clippy + workspace clippy regression-check; plus 2 v2-state-dispatch-sync invocations which reuse the build). All clean except 2 pre-existing unrelated workspace warnings.
- ~10 GitHub API operations cycle 167 (audit HEAD, input-from-eva list, open issues + PR list, PR #2979 view, session-start comment, session-end comment, cycle issue close; plus v2-state-dispatch-sync's internal graphql queries for #2960, #2974, #2978).
- 2 Edits on `tools/rust/crates/v2-state-audit/src/main.rs` (Track 2 constants + tests).
- 1 Edit on `docs/redesign/_notes/v2-state-retention-policy.md` (Track 2 Axis 6 section).
- 1 Write of new file `docs/redesign/_notes/cycle-167-state-sync-reconciliation-and-axis-6-recalibration.md` (this file).
- 0 Edits on legacy `tools/cycle-runner/`.
- 0 Edits on `.github/workflows/`.
- 0 Edits on this orchestrator prompt.

## What cycle 167 does NOT do

1. Does NOT absorb PR #2979 (cycle 166 Track 2 v2-channel-router enforcement extension dispatch) — still DRAFT. Cycle 168+ priority #1 (renumbered from cycle 166+ #3 after Track 1+Track 2 closures).
2. Does NOT implement refuse-to-write in writers — the cycle 158 forward work item (writer-side hard-threshold enforcement) remains forward. Cycle 162 wired runner pre-flight halt-on-hard; writer-side refuse remains design-only.
3. Does NOT migrate dispatches key from `agent_sessions` → `dispatches` — cycle 159 noted this as future v2 migration; cycle 167 honored the existing key per "tool + policy revise together when that happens" framing.
4. Does NOT design `v2-prompt-tag-semantic-fidelity` tool — cycle 168+ priority #2 (renumbered from cycle 166+ #4).
5. Does NOT add `status` / `verify` v2-cycle-runner subcommands — cycle 168+ priority #3 (was cycle 166+ #5).
6. Does NOT progress AGREE-DEFER queue — cycle 168+ priority #4 (was #6).
7. Does NOT advance coordinated retry/timeout/cancellation arc — cycle 168+ priority #5 (was #7).
8. Does NOT advance coordinated structured-error-envelope arc — cycle 168+ priority #6 (was #8).
9. Does NOT advance coordinated resume/recovery arc — cycle 168+ priority #7 (was #9).
10. Does NOT do single-track audit-engagement substantive-focal — cycle 168+ priority #8 (was #10); audit HEAD unchanged at `bc8fda63`.
11. Does NOT design per-axis archival mechanism — cycle 168+ priority #9 (was #11).
12. Does NOT do honesty-pass on v2 role prompts — cycle 168+ priority #10 (was #12); gated on PR #2979 landing.
13. Does NOT modify v1 frozen tools or `.github/workflows/` or this orchestrator prompt.

## Forward priorities for cycle 168+

Inheriting from cycle 166's renumbered list, minus #1 + #2 (closed cycle 167). 0 NEW priorities added (no new discoveries this cycle).

1. **PR #2979 absorption** (cycle 166 Track 2 dispatch; was cycle 166+ #3). Per cycle 165 PR #2975 absorption precedent shape — per-scope-reference verdict ledger against the cycle 164 design scope. Estimated 1 cycle if PR lands cleanly. Cycle 168-169 candidate depending on Copilot turnaround.
2. **TOOL-SCOPED `v2-prompt-tag-semantic-fidelity` tool design scope** (was cycle 166+ #4).
3. **`status` + `verify` v2-cycle-runner subcommands** (was cycle 166+ #5).
4. **AGREE-DEFER queue (post-real-role-session-measurement)** (was cycle 166+ #6).
5. **Coordinated retry/timeout/cancellation arc** — C13 + X2 (was cycle 166+ #7).
6. **Coordinated structured-error-envelope arc** — C6 + C7 + C9 (was cycle 166+ #8).
7. **Coordinated resume/recovery arc** — C11 + C12 + X1 (was cycle 166+ #9).
8. **Audit-engagement substantive-focal single-track variant** (was cycle 166+ #10; gate = audit HEAD changes from `bc8fda63`).
9. **Per-axis archival mechanism design scope** (was cycle 166+ #11).
10. **Honesty-pass on v2 role prompts** (was cycle 166+ #12; gated on PR #2979 landing).
11. **`v2-prompt-contract-check --strict` re-run post-extension** (was cycle 166+ #13; gated on PR #2979 landing + honesty-pass).
12. **Deprecate `Channel::required_payload_keys()` legacy method** (was cycle 166+ #14; gated on PR #2979 landing + honesty-pass + --strict re-run).
13. **`v2-state-dispatch-archive --invocation-id` flag** (was cycle 166+ #15; LOW priority; defer until v2-cycle-runner archival-wiring).

**Cycle 167 forward priorities CLOSED:**
- Cycle 166+ priority #1 (Axis 6 recalibration) — closed Track 2.
- Cycle 166+ priority #2 (v2-state-dispatch-sync stale-entry reconciliation) — closed Track 1.

**Cycle 167 new sub-priorities (net of closures):** 0 NEW.

**Observational forward-watch item (not yet a priority):** the 2-cycle pattern of first-live-invocation of latent tools (cycle 166: v2-state-dispatch-archive at 1-cycle latency; cycle 167: v2-state-dispatch-sync at 13-cycle latency) suggests a quick sweep audit of the v2 tool inventory may surface other tools authored-but-never-operationally-invoked. If a third such instance appears cycle 168+, name a pattern and consider a forward-priority audit. Until then, journal observation only.
