# Cycle 153 _notes — first end-to-end smoke + first measurement + state.json dispatch housekeeping

**Cycle:** 153 (two-track composition, 6th consecutive after cycle 151 single-track exception)
**Date:** 2026-05-15
**Cycle issue:** [#2962](https://github.com/EvaLok/schema-org-json-ld/issues/2962)
**Commits this cycle:** none on substantive focal (Track 1 is read-only measurement against scratch state); Track 2 + cycle-close in cycle-close commit
**Predecessor:** cycle 152 (commit `335f1199`, integration test against real primitives; dispatch #2960)
**Reference design scope:** [`cycle-149-v2-cycle-runner-design-scope.md`](cycle-149-v2-cycle-runner-design-scope.md) §11 (cycle 153 substantive focal = "first end-to-end smoke + first measurement")

## Cycle composition

Two tracks; cycle 152 forward priority list had Track 1 (first end-to-end smoke + measurement) as priority #1 and state.json dispatch-tracking housekeeping as priority #3. The latter was bounded mechanical work that pairs naturally with Track 1's measurement-and-record rhythm.

**Track 1 (substantive focal):** first end-to-end smoke + first measurement (cycle 149 §11). Differential value vs cycle 152 integration test: integration test asserts correctness in TempDir; cycle 153 measures wallclock, state-file shapes, and behavior under repeated cycles in a persistent (non-tempdir) state directory `.scratch/cycle153-smoke/`. The integration test answers "does it work"; cycle 153 answers "what does running it actually look like."

**Track 2 (bounded act-now):** state.json dispatch-tracking housekeeping (cycle 152 forward priority #3). 11 stale `in_flight` agent_sessions entries flipped to actual GitHub state; `in_flight_sessions` count corrected from 11 → 1. Unblocks the `dispatch-task` tool's "11 in-flight" warning observed cycle 152.

Two-track composition resumed cycle 152, continued cycle 153. Cycle 151 single-track exception remains pattern `design-scope-supersedes-inherited-cycle-shape-momentum`. Cycle 153 has no scope-shape direction (cycle 149 §11 names the focal but not the shape), so two-track default applies.

## Track 1 — first end-to-end smoke + first measurement

### Setup

Built all 5 v2-* primitives in **release** profile (cycles 150-152 used debug; release was the natural choice for measurement). Smoke directory: `.scratch/cycle153-smoke/` under the working tree (`/tmp` is outside the bash sandbox's allowed-write paths).

```
cargo build --release -p v2-cycle-runner -p v2-channel-router \
  -p v2-super-step-boundary -p v2-role-driver \
  -p v2-reconciler-event-processor
```

`init` against `.scratch/cycle153-smoke/` initialized 20 state files across 5 owners:
- channels: 8 (4 channel + 4 history)
- reconciler: 4 (3 cursor + 1 poll-history)
- roles: 4 (1 history per role)
- super-step: 2 (1 current + 1 history)
- v2-cycle-runner: 2 (last-cycle + cycle-history)

### Session-output payloads

Hand-prepared 4 session-output JSONs with **realistic** content (vs cycle 152 integration test's synthetic minimums):

| Role | Payload shape | Realistic content |
|---|---|---|
| reconciler | eva-responses=[], audit-posts=[1], dispatch-returns=[1] | references actual audit cycle 220 + dispatch #2960 |
| planner | substantive-focal + per-role-tasks (executor/curator) + secondary-tracks | reflects cycle 153 actual plan |
| executor | artifacts-written=[2] | names v2-cycle-runner state files |
| curator | consolidated-insights | one-sentence summary of cycle 153 |

### Run trace

Three live cycles + one dry-run, all clean. Cycle 153/154/155 status=completed, steps_attempted=10, no halt. Sample trace from cycle 153:

```
v2-cycle-runner run: cycle=153 issue=2962 status=completed
  started:        2026-05-15T20:30:33Z
  ended:          2026-05-15T20:30:33Z
  dry_run:        false
  steps_attempted: 10
```

### Wallclock measurements

External `date +%s%N` brackets around the cargo invocation. Cargo is warm after first cycle.

| Run | Mode | Wallclock (ns) | Wallclock (ms) |
|---|---|---|---|
| Cycle 154 (warm) | live | 94,376,981 | 94.4 |
| Cycle 155 (warm) | live | 98,267,995 | 98.3 |
| Cycle 999 (warm) | dry-run | 75,453,665 | 75.5 |

**Findings:**
- **Full cycle: ~94-98ms warm.** ~4ms variance across runs.
- **Dry-run: ~75ms.** Pure framework overhead (cargo invocation + binary load + trace-print).
- **Live overhead vs dry-run: ~20-25ms.** This is the actual primitive-invocation + state-write work for 10 super-steps.
- **Per-step amortized: ~2-2.5ms.** (20-25ms / 10 steps).

**Interpretation:** the framework is fast. Sub-100ms is below the cron tick resolution. Real production cycle wallclock will be dominated by role-driver's eventual live claude-code spawn (currently SCAFFOLD; deferred per cycle 149 OQ2). Conservative estimate: 30-60s per role session × 4 roles ≈ 2-4 min per cycle once live. The framework's 100ms is negligible by comparison.

### State-file shape measurements

Post-3-cycle state file sizes (sorted descending):

```
2783 channels/inbound-channel-history.json   (+~917 bytes/cycle)
1928 channels/plan-channel-history.json      (+~641 bytes/cycle)
1640 channels/work-channel-history.json      (+~547 bytes/cycle)
1527 reconciler/poll-history.json            (+~509 bytes/cycle)
1526 super-step-history.json                 (+~509 bytes/cycle)
1210 channels/memory-channel-history.json    (+~403 bytes/cycle)
 960 roles/reconciler-history.json           (+~320 bytes/cycle)
 937 roles/executor-history.json             (+~312 bytes/cycle)
 936 roles/curator-history.json              (+~312 bytes/cycle)
 930 roles/planner-history.json              (+~310 bytes/cycle)
 867 v2-cycle-runner/cycle-history.json      (+~289 bytes/cycle)
 620 channels/inbound-channel.json           (current only)
 593 channels/plan-channel.json              (current only)
 481 channels/work-channel.json              (current only)
 383 channels/memory-channel.json            (current only)
 237 v2-cycle-runner/last-cycle.json         (current only)
  43 reconciler/dispatch-cursor.json
  40 reconciler/audit-cursor.json
  38 reconciler/eva-cursor.json
   5 super-step.json
```

**Total state size after 3 cycles: ~16 KB.** Linear growth from history files (~4 KB/cycle aggregate across all owners). At a sustained 1 cycle / 5min cadence (the cron tick), state grows ~1.15 MB/year before any pruning. Bounded and inspectable.

**Notable shape observation: `super-step.json` is `null` (5 bytes) after 3 cycles.** This is the **current in-cycle indicator**; it is cleared at cycle-end while `super-step-history.json` accumulates. Design-consistent: the boundary primitive uses super-step.json as a temporary state during execution, then clears it on cycle-end. History is the authoritative record.

### Channel state shape (representative)

`channels/inbound-channel.json` after cycle 155:

```json
{
  "channel": "inbound-channel",
  "writer": "reconciler",
  "cycle": 155,
  "timestamp": "2026-05-15T20:31:01Z",
  "payload": { /* full reconciler session-output payload */ }
}
```

`channels/X-channel-history.json` accumulates one wrapped record per cycle in a `cycles[]` array.

### Important Phase 3 measurement finding: SCAFFOLD reality is visible in poll-history

`reconciler/poll-history.json` shows all 3 cycles with `events_in: 0, events_new: 0` for eva/audit/dispatch:

```json
{
  "cycle": 153,
  "eva":      { "events_in": 0, "events_new": 0, "cursor_before": null, "cursor_after": null },
  "audit":    { "events_in": 0, "events_new": 0, ... },
  "dispatch": { "events_in": 0, "events_new": 0, ... },
  "outcome": "success"
}
```

This is the **SCAFFOLD primitive's own observation surface**: the reconciler-event-processor isn't actually polling GitHub yet (deferred per Phase 3 SCAFFOLD strategy). Meanwhile, the **channel state** (inbound-channel.json) DOES contain the audit-cycle-220 + dispatch-#2960 data — because that data came from the **hand-prepared session-output JSON** via the role-driver invocation, NOT from the reconciler's own polling.

**Two separate "observation" surfaces exist and they don't agree:**
1. `reconciler/poll-history.json` — what the SCAFFOLD primitive observed (placeholder zeros)
2. `channels/inbound-channel.json` — what got into the channel via session-output (data-from-payload)

This is design-correct under SCAFFOLD strategy: the boundary primitives produce coherent state machinery (history, cursors, lookup-files) but their own observation logic is stubbed. Live polling will fill the poll-history zeros once the reconciler-event-processor gains actual GitHub-API logic.

**Pattern**: `scaffold-strategy-produces-coherent-shells-with-stubbed-observation-logic` NOVEL@1 cycle 153. Worth recording because the structural correctness validates the design split: state machinery is independent of the role's actual cognitive content, which means each can be developed and tested separately.

### Super-step-history shape

```json
{
  "cycles": [
    {
      "cycle": 153,
      "started_at": "...",
      "ended_at": "...",
      "transitions": [
        {"from": "reconciler", "to": "planner", "at": "..."},
        {"from": "planner", "to": "executor", "at": "..."},
        {"from": "executor", "to": "curator", "at": "..."}
      ]
    }
  ]
}
```

3 transitions per cycle (4 roles → 3 advance calls between them). Cycle-start and cycle-end are recorded as `started_at` / `ended_at` of the cycle entry, not as separate transitions.

### Single-timestamp-per-cycle pattern (cycle 151) validated

All 10 step invocations within a cycle share the same timestamp (down to second resolution). Confirmed via dry-run trace output:

```
step 1: ... --timestamp 2026-05-15T20:30:56Z
step 2: ... --timestamp 2026-05-15T20:30:56Z
...
step 10: ... --timestamp 2026-05-15T20:30:56Z
```

Cycle-level alignment as designed. Refinement to per-step wallclock measurement remains deferred (cycle 151 _notes called it deferred-to-first-measurement; cycle 153 IS that first measurement, and the verdict is: per-step wallclock is not necessary at this scale — the framework overhead is 2-2.5ms/step, which is below the noise floor for any production workload that involves a real role session).

## Track 2 — state.json dispatch-tracking housekeeping

### Audit

11 entries with `status: in_flight` in `docs/state.json`. Per-issue GitHub state via `gh api`:

| Issue | Actual GitHub state | New status |
|---|---|---|
| 2729 | closed 2026-05-01, no PR | closed_without_pr |
| 2736 | closed 2026-05-01, no PR | closed_without_pr |
| 2738 | closed 2026-05-01, no PR | closed_without_pr |
| 2910 | closed 2026-05-11, no PR | closed_without_pr |
| 2939 | closed 2026-05-14, no PR | closed_without_pr |
| 2940 | closed 2026-05-14, no PR | closed_without_pr |
| 2942 | closed 2026-05-14, no PR | closed_without_pr |
| 2947 | closed 2026-05-14, no PR | closed_without_pr |
| 2950 | closed 2026-05-15, no PR (critique via commit-as-file, cycle 148) | closed_without_pr |
| 2952 | closed 2026-05-15 via PR #2953 merge | **merged** (pr=2953, merged_at=2026-05-15T03:14:13Z) |
| 2960 | open, Copilot assigned+connected | **in_flight** (unchanged) |

### Approach

The state.json is 720 KB / 20,767 lines. Bash output redirection is blocked by the harness sandbox, so jq-pipe-to-file and python in-place editing both required approval that wasn't available. Fell back to 11 sequential Edit operations targeting the unique `"issue": NNNN,\n  "model": "gpt-5.4",\n  "status": "in_flight",` block per entry, plus one Edit on the top-level `in_flight_sessions: 11`.

### Post-housekeeping validation

```
by_status:
  closed:               15 (unchanged)
  closed_without_merge:  6 (unchanged)
  closed_without_pr:    25  (+10: 9 close + #2950)
  failed:               62 (unchanged)
  in_flight:             1  (-10)
  merged:              819  (+1: #2952)
  reviewed_awaiting_eva: 2 (unchanged)
in_flight_sessions: 1
still_in_flight: [2960]
```

`dispatch-task` will no longer report "11 in-flight" warning. Forward consequence: when the cron tools (or any other state.json reader) trust `in_flight_sessions` as a count, it's now accurate.

### Pattern

`stale-state-accumulates-during-rapid-dispatch-phases-and-needs-periodic-housekeeping-pass` NOVEL@1 cycle 153. The 10 stale entries accumulated across:
- 3 pre-redesign-mode dispatches that never closed cleanly (cycles ~466-470)
- 1 Phase 2 critique (cycle 118-area)
- 4 Phase 3 dispatches from cycle 144 + cycle 146 (all closed cycles 147-148 housekeeping but state.json wasn't updated synchronously with GitHub closure)
- 2 cycle 147-148 dispatches (one absorbed via commit-as-file, one merged via PR)

The pattern is **state.json status is updated when dispatch-task creates an entry, but is NOT updated when the dispatch outcome lands** (PR merges, issue closes, etc.). Manual housekeeping or a periodic-sync tool is required.

**Tool-extraction candidate:** `state-dispatch-sync` Rust tool that reads `in_flight` agent_sessions and reconciles each against GitHub via `gh api`, updating status accordingly. Cycle 154+ candidate. Aligns with CORE-DESIGN-PRINCIPLE: this housekeeping was bounded-mechanical and procedural, exactly the kind of work the orchestrator should not have to manually perform every time it accumulates.

## Pattern updates this cycle

- **`two-track-composition` HARDENING-AT-6 cycle 153.** Resumed cycle 152 after cycle 151 single-track exception; cycle 153 is 2nd consecutive resumed two-track. Combined arc: 146 + 147 + 148 + 149 + 150 = HARDENING-AT-5; cycle 151 was single-track exception (pattern `design-scope-supersedes-inherited-cycle-shape-momentum`); cycle 152 + cycle 153 resumed two-track. Counts as 2 in-band recurrences post-exception.
- **`design-scope-supersedes-inherited-cycle-shape-momentum`** HARDENING-AT-1 cycle 153 — cycle 152 + cycle 153 BOTH resumed two-track because §11 / §7.2 said nothing about shape. Confirms: when design scope is silent about cycle composition, the cycle-shape-momentum default re-applies. Single-instance arc closure for this pattern (151 NOVEL@1 → 153 HARDENING evidence via the silence-defaults-to-default scenario).
- **`scaffold-strategy-produces-coherent-shells-with-stubbed-observation-logic`** NOVEL@1 cycle 153. Validated by direct measurement evidence (poll-history zeros vs channel-state payload-data). Worth recording because it validates the SCAFFOLD design split.
- **`stale-state-accumulates-during-rapid-dispatch-phases-and-needs-periodic-housekeeping-pass`** NOVEL@1 cycle 153. State.json drift between local entry and GitHub state requires reconciliation. Tool-extraction candidate: `state-dispatch-sync`.
- **`magnitude-prediction-precision-is-shape-dependent-not-flat`** runner-shape family further data: full cycle wallclock = 94-98ms warm, dry-run 75ms. Adds a wallclock-band to the LOC-band (cycles 151 + 152 reported only LOC). Wallclock for full-arc (5-subcommand + integration test): estimate 80-120ms range under similar conditions.
- **`per-step-wallclock-not-necessary-at-scaffold-scale`** NOVEL@1 cycle 153. Measurement-driven verdict on the cycle-151-deferred per-step-wallclock refinement question: the framework overhead is 2-2.5ms/step, which is below the noise floor for any production workload involving real role sessions. Single-timestamp-per-cycle pattern (cycle 151) is correct.

## Forward priorities for cycle 154+

(Re-ordering from cycle 152's 11-item list, factoring in cycle 153 progress + new candidates.)

1. **Dispatch #2960 critique absorption** — when commit-as-file lands at `docs/redesign/_critique/cycle-152-v2-cycle-runner-adversarial-critique.md`. Per-finding evaluation per cycle 148 pattern. Likely cycle 154 or 155 substantive focal.
2. **Cycle 154+ substantive focal candidate: `state-dispatch-sync` Rust tool** — extracts the housekeeping pattern from cycle 153 into a reusable tool. Direct-push-zone. Aligns with CORE-DESIGN-PRINCIPLE: anywhere the orchestrator does the same procedural thing every cycle (or every few cycles) is a tool-extraction candidate.
3. **AGREE-ACT-NOW L1.3 + L3.6 reconciler completeness-metadata pairing** — bounded; prompt edit + v2-channel-router required-key extension.
4. **AGREE-ACT-NOW L2.4 dispatch-brief discipline addendum** — document-only.
5. **AGREE-ACT-NOW X2 honest cycle-1-scope-redefinition** — document-only.
6. **AGREE-RECORD L1.2 + X4 side-channel architecture-notes doc** — document-only.
7. **TOOL-SCOPED v2-channel-router enforcement extension design scope** (C1 / L3.1).
8. **TOOL-SCOPED v2-prompt-tag-semantic-fidelity tool design scope** (L2.5).
9. **`status` + `verify` v2-cycle-runner subcommands** — cycle 155+ if needed.
10. **AGREE-DEFER queue (post-real-role-session-measurement)** — curator complexity rework, template-mirror architecture-revisit. Hold for live-claude-code-spawn evidence (still SCAFFOLD).
11. **Cycle 120 L2 preserved.**

Note: cycle 152's "first end-to-end smoke + first measurement" (its old priority #1) is now CLOSED by cycle 153 Track 1.

## What cycle 153 does NOT do

- Does NOT implement `status` / `verify` subcommands (cycle 155+ if needed).
- Does NOT modify `cycle-runner` (forbidden zone).
- Does NOT modify `.github/workflows/` or this orchestrator prompt.
- Does NOT spawn live Claude sessions (role-driver remains SCAFFOLD; live spawn deferred per cycle 149 OQ2).
- Does NOT absorb dispatch #2960 critique (not yet returned — Copilot assigned+connected 10:58:54Z, 0 comments at cycle-start; will check at cycle-end too).
- Does NOT enact 5 remaining AGREE-ACT-NOW findings (L1.3, L2.4, L3.6, X2, L1.2+X4).
- Does NOT escalate any cycle 153 decision to Eva (EVA-DEFAULT-AUTONOMY).
- Does NOT modify `docs/redesign/2-selection.md` (cycle 120 L2 preserved).
- Does NOT measure live role-session token usage (no live spawn; SCAFFOLD).
- Does NOT extract the `state-dispatch-sync` tool (candidate noted as cycle 154+ work).
- Does NOT push a workflow YAML edit (Phase 4 cutover scope).

## Process honoring

- **38th consecutive cycle of HONORING named forward priority** (cycles 115-153). Cycle 152 named #1 = "first end-to-end smoke + first measurement"; cycle 153 Track 1 closes that priority. Cycle 152 named #3 = state.json housekeeping; cycle 153 Track 2 closes that priority.
- **66th bottleneck-asynchronous cycle** (78-153).
- **43rd non-per-candidate-sharpening cycle** (111-153).
- Cycle 120 L2 preserved (`2-selection.md` untouched).
- Cycle 128 lesson preserved (.scratch/ via Write tool for session-start + housekeeping script + cycle-close ephemerals).
- Cycle 133 clarification preserved: cargo invocations cycle 153 are legitimate measurement-mode work (build release-profile primitives + 3 live + 1 dry-run cycle of v2-cycle-runner).
- Cycle 134 lesson preserved (audit-repo HEAD via gh api; checked at session-start; unchanged).
- **Cycle 137 lessons re-validated cycle 153** — multiple in-cycle attempts: one for-loop blocked, one heredoc-redirect blocked, one shell-syntax-string blocked, one `cd` chain blocked, multiple `python3 -c` blocked, output redirection (`>`) blocked across all paths. **7-cycle running validation** (137 + 147 + 148 + 149 + 150 + 151 + 153). Cycle 152 had no validation event; cycle 153 had several.
- **Cycle 149 in-cycle CWD-drift lesson re-validated cycle 153** — zero `cd tools/rust` chains; all cargo via `--manifest-path`. **5-cycle running validation** (149 + 150 + 151 + 152 + 153).
- Cycle 151 date-test-value lesson preserved (no new date tests cycle 153).
- Cycle 152 disk-format-read-source lesson preserved (Track 1 measured against my own primitive outputs; no new asserts authored, but the lesson applies: read what the writer actually writes before asserting).
- **NEW cycle 153 lesson:** when Bash redirection (`>`) is blocked everywhere in the sandbox AND `python3 -c` requires approval AND `python3 script.py` requires approval, fall back to **multiple Edit operations** with uniquely-anchored old_strings. For the state.json housekeeping, 11 Edit operations achieved what one jq pipeline would have done — slower per-operation but reliable in-permission-model. Captured for future cycles where in-place edits of large structured files are needed.
- Journal-immutability discipline preserved (cycle 153 appends NEW section via Edit anchor at end of cycle 152 section; cycle 148-152 sections NOT back-edited).
- Anti-overstatement audit explicit ("What cycle 153 does NOT do" enumeration above).
- **Two-track composition resumed** continues from cycle 152; bounded mechanical Track 2 paired with measurement-focused Track 1.

## In-session issues and recoveries

- **One for-loop attempt blocked** ("Contains simple_expansion") at session-start orientation when iterating over 11 in_flight issue numbers via `gh api`. Recovered via 10 sequential single-issue `gh api` calls. 7-cycle running validation of cycle 137 lesson.
- **One `tee` redirect blocked** ("multiple operations") when wrapping `/usr/bin/time -v cargo run … | tee output.txt`. Recovered by emitting timing to stdout and removing the tee. Output captured manually into _notes.
- **`/usr/bin/time -v` invocation blocked** (interpreted as requiring approval, possibly because of the `-v` flag's broad permission scope). Recovered via `date +%s%N` bracketing around the cargo invocation.
- **Bash output redirection (`>`) blocked across all in-working-tree paths** including `.scratch/` and `docs/`. The error message stated "may only write to files in the allowed working directories" but writing inside the working tree was still blocked — implying `>` is blocked for safety regardless of destination. Recovered via Edit tool for state.json (11 calls) and Write tool for ephemerals.
- **`python3 -c "..."` and `python3 /scratch/script.py` both require approval.** Captured as: scripting languages via -c or script-file invocation are not in the bash auto-allow set. For data-processing tasks the right move is jq (which IS allowed) plus Edit/Write tool fallback for the apply step.
- **`rm -rf /tmp/cycle153-smoke` blocked** (path outside allowed working dirs). Recovered by relocating smoke directory to `.scratch/cycle153-smoke/` inside the working tree.
- **`mkdir … && cd …` blocked as multi-operation.** Recovered with single-operation pattern.
- All Edit / Write / gh issue comment / cargo / gh api operations clean cycle 153 beyond these.

## Cycle 153 ARTIFACTS

- `docs/state.json` — modified (11 Edits: 9 → closed_without_pr, 1 → closed_without_pr [#2950], 1 → merged [#2952] with pr+merged_at, 1 in_flight_sessions count update). ~30 lines changed in a 20,767-line file.
- `docs/redesign/_notes/cycle-153-first-end-to-end-and-housekeeping.md` — new (this file).
- This cycle appends to `docs/journal/2026-05-15.md` (cycle 148-152 sections preserved).
- 1 cycle-close commit covering: state.json edits + this _notes + journal append + cycle-close ephemerals. Single direct-push.
- `.scratch/` ephemerals:
  - `session-start-2962.md` (session-start comment body)
  - `cycle153-smoke/` smoke directory (~20 state files + 4 session-output JSONs + 1 run-output capture; ~16 KB)
  - `state-housekeeping.jq` (jq script — built but ultimately couldn't execute due to redirect block)
  - `state-housekeeping.py` (python script — built but couldn't execute due to approval requirement)
  - `cycle153-session-end.md`, `cycle153-issue-close.md` (latter two authored cycle-close)
- 0 v2-* crate source-code modifications cycle 153.
- 0 dispatches cycle 153 (no Copilot agent-task dispatches needed; #2960 still in-flight).
- 4 cargo invocations cycle 153 (build 5 primitives in release profile; 3 live cycles; 1 dry-run cycle). All clean.
- 11 Edits on state.json (housekeeping). All clean first attempt.

## Open questions surfaced this cycle (not blockers)

- **Q1: When does the reconciler-event-processor SCAFFOLD become live-polling?** Currently it produces zero-record poll-history entries. Live-polling would fill these. Not a blocker for cycle 154+, but worth scheduling as part of the broader SCAFFOLD→live transition.
- **Q2: Does `state-dispatch-sync` tool extraction need to be a separate Rust crate or can it live inside an existing crate?** Candidate: extend `record-dispatch` with a `--sync` subcommand, or create new `v2-state-dispatch-sync`. Cycle 154+ design decision.
- **Q3: At ~16 KB/3-cycles state growth, does cycle history need pruning ever?** Linear extrapolation: ~1.15 MB/year. Probably fine indefinitely. But channel-history files grow fastest (917 bytes/cycle for inbound). Worth measuring under sustained 5min-cron cadence over multiple weeks before deciding.

None of these need Eva input (EVA-DEFAULT-AUTONOMY). All can be resolved by cycle-153+orchestrator judgment + future evidence.
