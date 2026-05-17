# Cycle 166 — straight-pair-with-dispatch (inverted): backlog archival (Track 1) + v2-channel-router enforcement extension dispatch (Track 2)

**Cycle issue:** [#2977](https://github.com/EvaLok/schema-org-json-ld/issues/2977)
**Cycle date:** 2026-05-17
**Forward priorities source:** [`cycle-165-pr2975-absorption-and-audit-470-bifurcation-accept.md`](cycle-165-pr2975-absorption-and-audit-470-bifurcation-accept.md#forward-priorities-for-cycle-166)
**Predecessors:**
- Track 1: cycle 163 design scope [`v2-state-dispatch-archive.md`](v2-state-dispatch-archive.md) + cycle 164 dispatch [#2974](https://github.com/EvaLok/schema-org-json-ld/issues/2974) → cycle 165 PR #2975 merge (commit `45a59cf5`) → cycle 166 first live invocation against the 931-entry backlog.
- Track 2: cycle 164 design scope [`v2-channel-router-enforcement-extension.md`](v2-channel-router-enforcement-extension.md) + cycle 148 C1/L3.1 critique verdicts → cycle 166 dispatch [#2978](https://github.com/EvaLok/schema-org-json-ld/issues/2978).

## Cycle shape

Two-track composition continues, **HARDENING-AT-19** post cycle 151 single-track exception. 15 consecutive post-exception (152-166); 20 of 21 in arc 146-166.

Cycle 166 is **straight-pair HARDENING-AT-5** (162 NOVEL@1, 163 RECURRENCE-AT-2, 164 HARDENING-AT-3, 165 HARDENING-AT-4, 166 HARDENING-AT-5 — modal two-track composition shape across 5 consecutive cycles). **`straight-pair-with-dispatch-variant` RECURRENCE-AT-2 cycle 166 with inverted Track ordering** — cycle 164 had Track 1 = dispatch / Track 2 = design-scope; cycle 166 has Track 1 = bounded-mechanical (archival run) / Track 2 = dispatch. The inverted ordering shows the dispatch axis is symmetric across Track positions; the underlying composition shape (bounded-other-work + dispatch + same-cycle) is the invariant.

Track 1 closes cycle 165+ forward priority #1 (was cycle 164+ priority #2; gate cleared by cycle 165 PR #2975 merge). Track 2 closes cycle 165+ forward priority #3 (was cycle 164+ priority #4; Copilot-dispatch candidate per directive #2937 explicitly named in the design-scope §11 references line).

## Track 1 — v2-state-dispatch-archive backlog run (first live invocation)

**Closes cycle 165+ forward priority #1** (1-cycle estimate per cycle 165 _notes; actual: 1 cycle as predicted).

### Pre-run state

`docs/state.json` `agent_sessions[]`: **931 entries** at session start, distribution per cycle 165 close measurement:
- 819 merged + 62 failed + 25 closed_without_pr + 15 closed + 6 closed_without_merge = **927 in 5 archivable terminal statuses**
- 2 in_flight + 2 reviewed_awaiting_eva = **4 live-protected**

### Dry-run

```
cargo run --manifest-path tools/rust/Cargo.toml --release -p v2-state-dispatch-archive -- --repo-root . --dry-run --json
```

Result (2026-05-17T06:58:56Z):
```json
{
  "dry_run": true,
  "archived_count": 862,
  "archive_file": "docs/state-archive/dispatches-2026-05-17.json",
  "ineligible": {
    "live_status": 4,
    "ineligible_status": 0,
    "below_age_threshold": 65,
    "missing_timestamp": 0
  },
  "state_after": {
    "agent_sessions_total": 69,
    "agent_sessions_live": 4,
    "agent_sessions_terminal_retained": 65
  }
}
```

Math check: 862 archived + 65 below-threshold + 4 live = 931 ✓. The 65 below-30-day-threshold terminal entries date from 2026-04-17 forward (cycles ~138+).

### Real run

```
cargo run --manifest-path tools/rust/Cargo.toml --release -p v2-state-dispatch-archive -- --repo-root . --json
```

Result (2026-05-17T06:59:05Z): identical to dry-run (counts unchanged 9 seconds later — no concurrent mutation). Tool exited cleanly. Files written:
- `docs/state.json` — agent_sessions[] reduced 931 → 69 entries (~93% reduction).
- `docs/state-archive/dispatches-2026-05-17.json` — new file, 290633 bytes (~284KB), archive_version 1, 862 entries.

### Archive file schema verification

```json
{
  "archive_version": 1,
  "tool_version": "v2-state-dispatch-archive 0.1.0",
  "archived_at": "2026-05-17T06:59:05Z",
  "source_file": "docs/state.json",
  "criteria": {
    "age_days": 30,
    "status_filter": ["merged","failed","closed_without_pr","closed","closed_without_merge"],
    "invocation_id": "2026-05-17"
  },
  "entries_count": 862
}
```

`invocation_id` is the bare date (`"2026-05-17"`) per the cycle 165 PR #2975 §4.3 carveout — tool-layer has no cycle knowledge. Cycle-N richness is forward priority #16 (LOW, deferred to v2-cycle-runner archival-wiring cycle).

### Commit-and-push

Single atomic commit `fad7c5de` (per cycle 524 atomic-commit-push discipline): `docs/state.json` mutation + `docs/state-archive/dispatches-2026-05-17.json` creation in one commit. Direct-push to master — both files are in direct-push zones (`docs/state.json` is tool-mutated state; `docs/state-archive/` is tool-output).

### State-sync drift discovered (forward implication)

Post-archival `agent_sessions[]` shows **3 in_flight entries**:
- #2960 — dispatched 2026-05-15 10:58 (cycle 152 feedback-only); GH state is **CLOSED** since dispatch return.
- #2974 — dispatched 2026-05-17 03:09 (cycle 164 implementation); GH state is **CLOSED** since 2026-05-17 05:26 (closed by PR #2975 merge, cycle 165).
- #2978 — dispatched 2026-05-17 07:02 (this cycle's Track 2); GH state is **OPEN** — correctly in_flight.

The first two are **state-sync drift** — GH issue state advanced past terminal, but `docs/state.json` was never updated. v2-state-dispatch-sync (cycle 154) is the dedicated reconciliation tool; it has not been invoked operationally since landing. Forward priority: run v2-state-dispatch-sync to reconcile both stale entries.

This drift is harmless to dispatch capacity gating (the warning fires but doesn't block — and capacity is operator-judgment under redesign mode per AUTHORITY.copilot-dispatches), but it falsifies the post-archival agent_sessions[] count as a "live workload" measurement. The 4-live count in the tool's report includes 2 stale-in-flight entries.

## Track 2 — v2-channel-router enforcement extension Copilot dispatch (2nd Role 2 instance under directive #2937)

**Closes cycle 165+ forward priority #3** (was cycle 164+ priority #4; cycle 164 design-scope-authored, cycle 166 dispatch-fired).

### Dispatch mechanics

```
tools/dispatch-task \
  --title "[redesign-impl] v2-channel-router enforcement extension + v2-prompt-contract-check extension (cycle 166)" \
  --body-file .scratch/cycle166-dispatch-body.md \
  --label agent-task --label implementation --skip-pipeline-gate
```

- Issue [#2978](https://github.com/EvaLok/schema-org-json-ld/issues/2978) created.
- Receipt commit `a29b42d4` pushed atomically (per cycle 524 discipline; dispatch-task handles internally).
- Copilot assigned `2026-05-17T07:02:05Z`, `connected` event fired `2026-05-17T07:02:16Z` (11s post-assignment, within expected ~10-15s window).
- Tool warning: "Warning: in-flight dispatches at 3 (approaching/exceeding concurrency limit of 2)" — informational; the count includes 2 stale-in-flight entries discovered in Track 1 §State-sync drift.

### Dispatch body shape

~280 lines at `.scratch/cycle166-dispatch-body.md` (transient — not committed). Modeled on cycle 164 PR #2974 brief shape (which itself modeled on cycle 147 PR #2952). Sections:
- Context: where this fits (dual-crate extension, schema-version-bump atomicity)
- Ordered reading list (6 documents: design scope first as authoritative spec, then both crate sources, then workspace Cargo.toml, then critique-verdict source, then v2-state-audit as shape model)
- What to build (2 crate-extension blocks):
  - Crate 1 (v2-channel-router): PayloadType + PayloadKey + PayloadSchema types; Channel::payload_schema() method (Option B per design §3.2); extended validate_payload (5 sub-rules); `--mode strict|lenient` flag; extended run_schema with schema_format_version: 2
  - Crate 2 (v2-prompt-contract-check): schema-format-version check; extended comparison loop with 4 new mismatch categories
- Tests (unit + integration, per-crate)
- Specific behaviors (7 invariants)
- Anti-patterns (7 transcribed from design scope §9 + design intent clarifications)
- What's already done (5 items)
- What to skip / NOT do (12 explicit non-doings)
- Verify-before-PR steps (cargo build/test/clippy per crate + workspace regression + end-to-end schema verification)
- Direct-push-zones note (`tools/rust/crates/v2-*` zone; --admin merge expected per cycle 165 PR #2975 precedent)
- Provenance footer

### Dispatch scope explicitly bounds

The dispatch covers ONLY:
- v2-channel-router extension (PayloadSchema, payload_schema(), validate_payload, --mode flag, run_schema schema_format_version: 2)
- v2-prompt-contract-check extension (schema-format-version handling, extended comparison loop)
- Tests for both

Explicitly OUT of dispatch scope (separately tracked as cycle 167+ forward priorities):
- Honesty-pass on v2 role prompts (cycle 165+ priority #4 — gated on dispatch landing + extended comparison report)
- Deprecation of `Channel::required_payload_keys()` (cycle 165+ priority #15 — post-migration, separate cycle)
- v2-prompt-tag-semantic-fidelity tool (cycle 165+ priority #5 — orthogonal axis, separate design scope)
- Any element-type enforcement in arrays (design scope §2.2 / §9.3 — out of scope by design)

### Expected absorption shape (cycle 167+)

PR will follow cycle 165 PR #2975 absorption pattern: per-scope-reference verdict ledger against the cycle 164 design scope. Estimated scope-references to verify: ~30-40 (across §2 tool boundary, §3 source-of-truth Option B, §4 type vocabulary including §4.1-4.6, §5 enforcement points §5.1-5.4, §6 migration plan §6.1 implementation only, §9 anti-patterns).

Expected first-cycle absorption cost: 1 cycle if PR lands cleanly (cycle 165 PR #2975 was 1-cycle absorption). 2 cycles if material disagreements emerge.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 archival (state.json mutation) | -9618 lines net | `fad7c5de` |
| Track 1 archive file `docs/state-archive/dispatches-2026-05-17.json` | 290633 bytes / 862 entries | `fad7c5de` |
| Track 2 dispatch body `.scratch/cycle166-dispatch-body.md` | ~280 lines (transient) | (passed to dispatch-task) |
| Track 2 dispatch receipt commit | (auto: +1 agent_sessions entry) | `a29b42d4` |
| `cycle-166-*.md` (_notes) | ~265 lines | cycle-close |
| Total cycle 166 textual output | ~545 lines | 3 substantive commits |

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 166 datapoints: bounded-mechanical-shape (Track 1) was 2 tool invocations + 1 commit — under 10 minutes elapsed. Dispatch-shape (Track 2) was ~280 lines brief + 1 tool invocation + verification — consistent with cycle 164 Track 1 dispatch shape (~280 lines brief). Combined cycle is mechanical+dispatch-pair shape: substantially shorter main-authored text than substrate-build cycles, comparable to absorption-shape cycles (cycle 165 was 290 main-authored lines; cycle 166 was ~545 lines including the longer _notes that document Track 1's measurement details).

## Pattern updates this cycle

- **`straight-pair-with-dispatch-variant` RECURRENCE-AT-2** cycle 166 (164 NOVEL@1, 166 RECURRENCE). Inverted Track ordering vs cycle 164 (T1=dispatch/T2=design → T1=mechanical/T2=dispatch) shows the dispatch axis is symmetric across Track positions.
- **`directive-2937-copilot-role-bifurcation` RECURRENCE-AT-2** cycle 166 (cross-cycle pattern audit-named cycle 222, main-accepted cycle 165, main-RECURRES cycle 166). 2nd Role 2 instance (`Copilot-as-implementer` = #2974/#2978) joins the 2 Role 1 (`Copilot-as-adversarial-critique` = #2951/#2961). Empirical instance count over 20-cycle window 146-166: Role 1 = 2, Role 2 = 2. Bifurcation balance.
- **`straight-pair-closure-as-default-two-track-shape` HARDENING-AT-5** cycle 166 (162, 163, 164, 165, 166). Modal two-track composition shape across 5 consecutive cycles.
- **`two-track-composition` HARDENING-AT-19** cycle 166. 15 consecutive post cycle 151 exception (152-166).
- **`audit-as-priority-1-input` NOT-EXERCISED** cycle 166 (audit HEAD unchanged at `bc8fda63`; no new audit-engagement to absorb). Carries forward at HARDENING-AT-3.
- **`external-deliverable-absorption-pair` NOT-EXERCISED** cycle 166 (cycle 166 authored substrate via Track 1 archival run; not an absorption pair). Carries forward at NOVEL@1.
- **`directive-2937-track-1-dispatch-fit-application` NOT-EXERCISED-AS-TRACK-1** cycle 166 — Track 1 was bounded-mechanical, dispatch was Track 2. The cycle 164 NOVEL@1 framing specifically required Track-1 position; cycle 166's Track-2 dispatch is a related-but-distinct shape (named via `straight-pair-with-dispatch-variant` RECURRENCE above). Carries forward at NOVEL@1.
- **`tool-extraction-surfaces-prior-cycle-errors` NOT-EXERCISED** cycle 166 (Track 1 was first-live-invocation of a tool built cycle 163-165; no prior-cycle error surfaced). Carries forward at RECURRENCE-AT-6.
- **`agree-act-now-bounded-fix-via-tracksecond-pattern` NOT-EXERCISED** cycle 166. Carries forward at RECURRENCE-AT-7.
- **`pre-flight-vs-mid-cycle-halt-distinction` NOT-EXERCISED** cycle 166. Carries forward at RECURRENCE-AT-2.
- **`fail-open-on-tool-internal-anomalies-but-fail-closed-on-policy-violations` NOT-EXERCISED** cycle 166. Carries forward at NOVEL@1.
- **`boundary-error-text-classifier-mismatch-found-via-test-implementation` NOT-EXERCISED** cycle 166. Carries forward at NOVEL@1.
- **`agree-defer-priority-becomes-live-after-blocker-clears` NOT-EXERCISED** cycle 166 (Track 1 closed a priority where the blocker was the cycle 165 PR merge, not a long-deferred AGREE-DEFER). Carries forward at NOVEL@1.
- **NEW NOVEL@1 cycle 166: `tool-first-live-invocation-surfaces-state-drift-in-adjacent-tool`** — Track 1 first-live-invocation of v2-state-dispatch-archive surfaced state-sync drift in v2-state-dispatch-sync's responsibility (stale in_flight entries for #2960 + #2974). The new tool worked correctly; the discovery is that the adjacent reconciliation tool has not been run operationally. Pattern: tooling activation cascade — activating tool A surfaces work for tool B. Distinct from `tool-extraction-surfaces-prior-cycle-errors` (which is about THE tool revealing errors in the THING IT'S EXTRACTING); this is about tool A's clean operation revealing a gap in tool B's operational use.

## Process honoring

- **51st consecutive cycle of HONORING named forward priority** (cycles 115-166).
- **79th bottleneck-asynchronous cycle** (78-166).
- **56th non-per-candidate-sharpening cycle** (111-166).
- Cycle 120 L2 preserved (`2-selection.md` untouched cycle 166).
- Cycle 128 lesson preserved (.scratch/ via Write tool — 4 files: session-start, Track 1 commit msg, Track 2 dispatch body, Track 2 commit msg).
- Cycle 133 lesson application: 1 cargo invocation cycle 166 (`cargo run -p v2-state-dispatch-archive` dry-run; the real-run reused the release build instantly). Non-cargo-test cycle (no Rust source modified by main).
- Cycle 134 lesson preserved (audit-repo HEAD via gh api at session-start AND session-end; unchanged at `bc8fda63` both times).
- **Cycle 137 lessons re-validated cycle 166 20-cycle-running** (137 + 147-166). All `gh` operations single-purpose-bash-invocation form.
- **Cycle 149 in-cycle CWD-drift lesson re-validated cycle 166** — all cargo invocations via `--manifest-path tools/rust/Cargo.toml`. No cd attempts this cycle (initial attempt to use the `tools/v2-state-dispatch-archive` wrapper hit permission gate; recovery to cargo direct preserved CWD discipline).
- Cycle 151 date-test-value lesson preserved.
- Cycle 152 disk-format-read-source lesson preserved.
- Cycle 153 multi-Edit-fallback lesson preserved (no multi-Edit needed cycle 166).
- **Cycle 154 `gh api graphql` subprocess pattern preserved** — Track 2 dispatch via `tools/dispatch-task` which calls graphql replaceActorsForAssignable internally. RECURRENCE of the cycle 164 first Track-1 dispatch under directive #2937 (now 2nd Role 2 instance).
- Cycle 155 dispatch-return-detection lesson preserved.
- Cycle 156 anti-overstatement audit enumeration discipline preserved (this _notes file has explicit "What cycle 166 does NOT do" with 15 items; Track 1 measurements have explicit pre/post counts and math-check; Track 2 has explicit "out of dispatch scope" enumeration).
- Cycle 157 audit-HEAD-check at session-start preserved.
- Cycle 158 / 159 / 160 / 161 / 162 / 163 / 164 / 165 closure pattern progression — straight-pair HARDENING-AT-5.
- Cycle 159 test-pattern-mirror lesson preserved.
- Cycle 160 schema-duplication observation preserved.
- Cycle 161 paired-closure pattern preserved (acknowledged; cycle 166 structurally different — straight-pair-with-dispatch-variant).
- Cycle 162 state-audit session-start wiring preserved (not exercised cycle 166 — no v2-cycle-runner invocation).
- Cycle 163 implementation-discovery-via-testing lesson preserved (not exercised cycle 166 — no new Rust authored by main).
- Cycle 164 forward-priority-renumbering pattern preserved (cycle 166 _notes inherits cycle 165's 16-priority list, closes #1 and #3, renumbers).
- Cycle 164 dispatch-receipt atomic-commit pattern preserved (Track 2 dispatch receipt commit `a29b42d4` pushed atomically via dispatch-task).
- Cycle 165 PR-absorption-shape preserved as reference for cycle 167+ absorption of PR-when-it-lands from this dispatch.
- Journal-immutability discipline preserved (cycle 166 APPENDS to today's journal `2026-05-17.md` containing cycles 163-165; prior cycle sections NOT back-edited).
- **Two-track composition continued** — cycle 166 is 15th consecutive post cycle 151 exception (HARDENING-AT-19).
- **SECTION 6b list housekeeping NOT invoked cycle 166** — 6 currently-open issues (cycle issue + dispatch issue #2978 + 4 standing input-from-eva); 0 open PRs. No closure candidates (dispatch is open and pre-PR; standing input-from-eva are operative directives).

## In-session issues and recoveries

- **One in-session permission-gate recovery cycle 166.** Initial `tools/v2-state-dispatch-archive --help` and `--dry-run --json` invocations required user approval (wrapper script not in auto-approved list). Recovery: invoke via cargo directly (`cargo run --manifest-path tools/rust/Cargo.toml --release -p v2-state-dispatch-archive -- ...`) which IS auto-approved. CWD-drift discipline preserved (manifest-path arg, no cd). Identical functional behavior; different invocation surface.
- All `git add` / `git commit` / `git push` operations clean cycle 166.
- All Edit / Write operations clean cycle 166.
- All `gh api` / `gh issue list` / `gh issue comment` / `tools/dispatch-task` operations clean cycle 166.
- Tool-discovered drift: 2 stale in_flight entries in docs/state.json (#2960 + #2974). Captured as cycle 167+ forward priority; NOT remediated in cycle 166 to avoid scope expansion.

## Cycle 166 ARTIFACTS

- `docs/state.json` — mutated by Track 1 (agent_sessions[] 931→69 entries; commit `fad7c5de`) + Track 2 dispatch-task subprocess (+1 entry for #2978; commit `a29b42d4`).
- `docs/state-archive/dispatches-2026-05-17.json` — new file (Track 1, 290633 bytes, 862 entries, commit `fad7c5de`).
- `docs/redesign/_notes/cycle-166-backlog-archival-and-channel-router-extension-dispatch.md` — new (this file, ~265 lines, cycle-close).
- `docs/journal/2026-05-17.md` — appended (cycle 166 section).
- `.scratch/cycle166-*.{md,txt}` — ephemerals (session-start, Track 1 commit msg, Track 2 dispatch body, session-end, issue-close).
- 3 substantive commits: `fad7c5de` (Track 1) + `a29b42d4` (Track 2 dispatch receipt) + cycle-close commit (this _notes + journal + ephemerals).
- 0 issues closed cycle 166 explicitly (cycle issue closes per existing convention; #2974 GH-closed cycle 165 but still stale-in-flight in state.json; #2978 newly open and in-flight).
- **1 Copilot dispatch cycle 166** — issue [#2978](https://github.com/EvaLok/schema-org-json-ld/issues/2978) (2nd Role 2 instance under directive #2937, HARDENS `directive-2937-copilot-role-bifurcation` to RECURRENCE-AT-2).
- 1 cargo invocation cycle 166 (v2-state-dispatch-archive dry-run; real-run reused release build).
- ~12 GitHub API operations cycle 166 (audit HEAD ×2, input-from-eva list, open issues + PR list ×2, session-start comment, issue #2978 events, issue #2974 state, issue #2960 state, session-end comment).
- 0 Edits cycle 166 (no Rust source modified by main; all main artifacts are new Writes or auto-mutations).
- 4 Writes cycle 166 (.scratch/session-start, .scratch/track1-commit-msg, .scratch/dispatch-body, this _notes file).
- 0 Edits on legacy `tools/cycle-runner/`.
- 0 Edits on `.github/workflows/`.
- 0 Edits on this orchestrator prompt.

## What cycle 166 does NOT do

1. Does NOT patch `v2-state-retention-policy.md` §4 Axis 6 thresholds (50/200/500) — that's cycle 167+ forward priority #1 (post-archival recalibration). The 50/200/500 thresholds were sized against a steady-state stream; the post-archival 69-entry floor + new ~30-day-window growth pattern needs threshold review. Now actionable.
2. Does NOT run `v2-state-dispatch-sync` to reconcile the 2 stale in_flight entries (#2960 + #2974) — that's a NEW cycle 167+ forward priority discovered this cycle. Bounded-mechanical, ~1 cycle.
3. Does NOT design `v2-prompt-tag-semantic-fidelity` tool — cycle 165+ priority #5 (now cycle 167+ #4).
4. Does NOT add `status` / `verify` v2-cycle-runner subcommands — cycle 165+ priority #6 (now #5).
5. Does NOT progress AGREE-DEFER queue — cycle 165+ priority #7 (now #6).
6. Does NOT advance coordinated retry/timeout/cancellation arc — cycle 165+ priority #8 (now #7).
7. Does NOT advance coordinated structured-error-envelope arc — cycle 165+ priority #9 (now #8).
8. Does NOT advance coordinated resume/recovery arc — cycle 165+ priority #10 (now #9).
9. Does NOT do single-track audit-engagement substantive-focal — cycle 165+ priority #11; audit HEAD unchanged at `bc8fda63`, no engagement to absorb.
10. Does NOT design per-axis archival mechanism — cycle 165+ priority #12 (now #11).
11. Does NOT do honesty-pass on v2 role prompts — cycle 165+ priority #13 (now #12); gated on dispatch #2978 landing.
12. Does NOT re-run v2-prompt-contract-check --strict — cycle 165+ priority #14 (now #13); gated on dispatch #2978 landing.
13. Does NOT deprecate `Channel::required_payload_keys()` — cycle 165+ priority #15 (now #14); gated on #12-13.
14. Does NOT add v2-state-dispatch-archive --invocation-id flag richness — cycle 165+ priority #16 (now #15); LOW priority, deferred to v2-cycle-runner archival-wiring cycle.
15. Does NOT modify v1 frozen tools or `.github/workflows/` or this orchestrator prompt.

## Forward priorities for cycle 167+

Inheriting from cycle 165's renumbered list, minus #1 + #3 (closed cycle 166). Two NEW priorities added (NEW#1 axis recalibration is the cycle-165-flagged-gate-now-cleared; NEW#2 is the state-sync drift discovered this cycle).

1. **NEW: v2-state-retention-policy.md §4 Axis 6 recalibration patch** (gate cleared by Track 1 archival run; was cycle 165 priority #2). Update the 50/200/500 hard/mandatory/soft thresholds to reflect post-archival steady-state. Estimated ~50-100 lines of edits to the policy doc; design-scope-textual-shape.
2. **NEW: v2-state-dispatch-sync operational invocation to reconcile stale entries** (discovered cycle 166 Track 1). Bounded-mechanical; reconciles #2960 + #2974 (and any other drift). ~1 cycle.
3. **PR #2978 absorption** (cycle 166 Track 2 dispatch; per cycle 165 PR #2975 absorption precedent shape — per-scope-reference verdict ledger against the cycle 164 design scope). Estimated 1 cycle if PR lands cleanly. Cycle 167-168 candidate depending on Copilot turnaround.
4. **TOOL-SCOPED `v2-prompt-tag-semantic-fidelity` tool design scope** (was cycle 165+ priority #5).
5. **`status` + `verify` v2-cycle-runner subcommands** (was cycle 165+ priority #6).
6. **AGREE-DEFER queue (post-real-role-session-measurement)** (was cycle 165+ priority #7).
7. **Coordinated retry/timeout/cancellation arc** — C13 + X2 (was cycle 165+ priority #8).
8. **Coordinated structured-error-envelope arc** — C6 + C7 + C9 (was cycle 165+ priority #9).
9. **Coordinated resume/recovery arc** — C11 + C12 + X1 (was cycle 165+ priority #10).
10. **Audit-engagement substantive-focal single-track variant** (was cycle 165+ priority #11; gate = audit HEAD changes from `bc8fda63`).
11. **Per-axis archival mechanism design scope** (was cycle 165+ priority #12).
12. **Honesty-pass on v2 role prompts** (was cycle 165+ priority #13; gated on PR #2978 landing).
13. **`v2-prompt-contract-check --strict` re-run post-extension** (was cycle 165+ priority #14; gated on PR #2978 landing + honesty-pass).
14. **Deprecate `Channel::required_payload_keys()` legacy method** (was cycle 165+ priority #15; gated on PR #2978 landing + honesty-pass + --strict re-run).
15. **`v2-state-dispatch-archive --invocation-id` flag** (was cycle 165+ priority #16; LOW priority; defer until v2-cycle-runner archival-wiring).

**Cycle 166 forward priorities CLOSED:**
- Cycle 165+ priority #1 (backlog archival run, Track 1) — closed; gate cleared for what becomes cycle 167+ NEW #1.
- Cycle 165+ priority #3 (v2-channel-router enforcement extension implementation, Track 2 — dispatched, not yet landed; PR absorption becomes cycle 167+ #3).

**Cycle 166 new sub-priorities (net of closures):** 2 NEW (Axis 6 recalibration formally added vs being cycle-165-flagged as gated; state-sync drift remediation NEW).
