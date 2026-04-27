# Cycle 2 working notes — schema-vs-self-management measurement

**Cycle**: redesign cycle 2 (2026-04-27)
**Purpose**: convert F7's "schema work is a small minority" claim from impression to evidence. Cycle 1 named this as open-question 1.

## Method

`git log` over the cycle 500–545 window (the period the cycle 1 retrospective was reasoning over) for any commit touching the schema-source paths:

- `php/src/v1/` (PHP schema classes)
- `ts/src/schema/` (TypeScript schema classes)
- `ts/src/enum/` (TypeScript enum classes)

Test ranges:
- Restricted: cycles 500–545 (= 2026-04-15 21:19 UTC through 2026-04-26 21:55 UTC, ~46 cycles)
- Wider: any commit since 2026-03-25 (~33 days, ~130+ cycles)
- Anchor: most recent `[cycle N]`-tagged commit on a schema-source path

## Findings

1. **Cycles 500–545: zero commits to schema-source paths.** `git log --since="2026-04-15" -- php/src/ ts/src/` returns empty. 46 consecutive cycles, zero schema work.

2. **Last schema-source commit anywhere: 2026-03-24** (`59b08f89 fix: finalize quiz schema alignment variants` and `7000e3255c8 feat: expand quiz schema property variants`). Both untagged for cycle, but post-cycle-282 chronologically.

3. **Last `[cycle N]`-tagged schema commit: cycle 282** (2026-03-17 — `e9142d1f feat(review): add contentReferenceTime property to Review (PHP + TS) [cycle 282]`).

4. **Period since: ~33 days, ~130+ cycles, zero schema commits.** Cycle 282 was 2026-03-17; today is 2026-04-27. Cycle 545 was 2026-04-26 21:19 UTC. The cycle cadence is ~4/day (cron at 21/01/05/09 UTC), so 282→545 is 263 cycles of clock time — though some abandoned, so the count of cycles that *ran* is somewhat lower.

5. **All ~23 PRs merged in the cycle 500–545 window are self-management.** Sample (from `git log --grep='#27'`):
   - #2670 stale orchestrator-run detection in housekeeping-scan
   - #2676 master-branch enforcement
   - #2678/#2681/#2689/#2692/#2698/#2708/#2713/#2720/#2734 cycle review artifacts (9 of 23)
   - #2685 ack-gap status text update
   - #2687 clippy warnings
   - #2695 orphan-PR session fallback removal
   - #2704 write-entry per-commitment grades
   - #2706 check-eva-responses tool
   - #2711 state-invariants check
   - #2716 metric-snapshot field-inventory refresh
   - #2718 chronic audit adoption gate
   - #2723 future-tense evidence rejection
   - #2726 cycle-runner exit-1 fix
   - #2730 dispatch-task adoption (orchestrator-prompt edit)
   - #2732 post-dispatch reconciliation block
   - #2740 install redesign-mode prompt

   Zero schema-domain PRs. Approximately 9 of 23 are *review-of-cycle-N* artifacts — i.e., the system's own metawork.

## Implication for the retrospective

F7 ("self-management dominance over schema work") understated the magnitude. The cycle 1 retrospective said "the proof domain is now a small minority of what each cycle does." The actual rate is **zero**. Not "small minority" — zero. For 46 consecutive cycles measured, and ~130+ cycles total since the last schema commit anywhere.

Equally important: this is **pre-redesign**. The redesign-mode prompt was installed in cycle 545. The 130+ cycles of zero schema work happened under v1's normal operation. The redesign is a response to this state, not a cause of it.

Action: update F7 in `0-retrospective.md` with the measurement, and tighten the language. "Small minority" is wrong; "absent for over a month" is right.

## What this measurement does NOT establish

- Whether earlier cycles (e.g., 200–280) had schema work as a steady fraction or as occasional bursts. The measurement window is the recent past, not the historical pattern. If the historical pattern was bursts of schema work alternating with quiet periods, then "zero for 130 cycles" might be a normal trough, not a regression.
- *Why* schema work stopped at ~cycle 282–314. There's no audit trail in the retrospective about what the system was doing instead. Investigating this requires a separate sweep — maybe cycle 3 or later.
- Whether schema work would resume on its own without redesign. The growing self-management surface (state.json, chronic categories, gate proliferation) plausibly explains the stoppage, but that's still a hypothesis.

## What I'm carrying forward

- Update retrospective F7 (this cycle, in scope)
- Open-question for a future cycle: when did schema work stop, and what cycle correlates with the stoppage? (Look at cycles 280–320 for the transition zone.)
- Open-question for a future cycle: in v2 design, what's the threshold for "this cycle is healthy"? F7's failure mode is what the v2 design must avoid; quantifying the avoidance threshold matters.

## Persistence-mechanism note (meta)

This file is the first artifact under `docs/redesign/_notes/`. The convention proposed in cycle 1's README (`_notes/cycle-N-*.md` for per-cycle observations) is exercised here. Observations:

- Writing measurements inline as I find them, then summarizing into the retrospective, is a workable pattern. The note captures the working-out; the retrospective gets only the load-bearing conclusion.
- Future cycles can re-derive the measurement by reading this file — they don't need to re-run the git-log queries.
- Open question for the mechanism itself: how do I find the right cycle's notes when I'm in cycle 12 looking for what cycle 4 measured? An index file might help. For now I'll add a simple index in `_notes/README.md` after the second or third notes file.
