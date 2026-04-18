## 1. [worklog-accuracy/pr-merge-accounting] The worklog contradicts itself about whether PR #2576 was merged this cycle

**File**: docs/worklog/2026-04-18/015333-cycle-511-review-actioned-1-of-3-field-inventory-refreshed-state-integrity-deferred-drop.md:6
**Evidence**: The narrative says `Merged cycle 510 review PR #2576` (`:6`), but the dedicated `PRs merged` section immediately below says `None.` (`:9-11`). GitHub metadata for PR #2576 shows it was actually merged at `2026-04-18T01:50:38Z`, which is inside cycle 511, and the cycle issue's own Step C2 comment also says `cycle 510 review PR #2576 was the only merge this cycle`.
**Recommendation**: Keep the narrative and the `PRs merged` block on the same timeline. If review PR merges count for cycle reporting, list `#2576` in the table; if they are intentionally excluded, remove the merge claim from the prose and say the exclusion explicitly.

## 2. [worklog-accuracy/post-dispatch-state] The published cycle summary still freezes a pre-dispatch snapshot and goes stale before the cycle actually closes

**File**: docs/worklog/2026-04-18/015333-cycle-511-review-actioned-1-of-3-field-inventory-refreshed-state-integrity-deferred-drop.md:7
**Evidence**: The published worklog says `No new dispatches.` (`:7`) and reports `In-flight agent sessions: 1` (`:24`). But the same cycle's `state(record-dispatch)` commit `ee1ad88` lands four seconds after the docs commit, adds review issue `#2578` to `agent_sessions` (`docs/state.json:9653-9658`), increments `in_flight_sessions` to `2`, and rewrites `last_cycle.summary` to `1 dispatch, 0 merges` (`docs/state.json:10958-10964`). Cycle 510's review explicitly flagged this exact stale post-dispatch pattern, and cycle 511 deferred that finding instead of preventing it from recurring in its own published artifacts.
**Recommendation**: Either dispatch the review before freezing the worklog/journal, or regenerate the published cycle-state lines after `record-dispatch`. If the worklog must stay frozen at `cycle-complete`, add an explicit post-dispatch delta so the permanent artifact cannot contradict final committed state.

## 3. [journal-quality/commitment-grading] The journal still flattens a revised commitment to plain “Followed”

**File**: docs/journal/2026-04-18.md:19
**Evidence**: The cycle 510 review finding said walked-back commitments should be marked as revised, not plain `Followed`. But the permanent cycle 511 journal entry still starts with `**Followed.**` and only then explains the real grade as `followed-and-revised` (`:19`). The same journal later admits the structural problem remains: `the rendered journal flattens it to plain markdown that the review agent reads as single-label` (`:33`). That means this chronic category was acknowledged in prose, but not genuinely corrected in the durable artifact.
**Recommendation**: Make the rendered journal label match the real grade. Prefer a first-class `followed-and-revised`/partial status in the journal generator; until that exists, do not lead the entry with `Followed.` when the evidence in the same paragraph says the outcome was revised.

## Complacency score

**3/5** — Cycle 511 did real work on the field-inventory finding and used the available tools/process comments diligently, so this is not a collapse. But two chronic review categories immediately recurred in the final artifacts, and the worklog added a new internal contradiction about whether PR #2576 merged. That is more than a cosmetic miss: it shows repeated known accuracy problems being narrated rather than retired.
