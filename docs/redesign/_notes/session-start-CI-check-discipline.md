# `session-start-CI-check-discipline` — SOLIDIFY analysis and HARDENED principle

**Pattern named:** cycle 172, 2026-05-18.
**SOLIDIFY-or-HARDEN analysis:** cycle 178, 2026-05-18 (this document).
**Status post-analysis:** HARDENED.

## Pattern definition

At session start, after reading the cycle issue and confirming model + run-id, execute:

```
$ gh run list --branch master --workflow "Rust CI" --limit 3
```

…and **interpret the result**:

- All three rows `success` → master CI green; the prior cycle's commit held; proceed to normal Track 1 / Track 2 planning.
- Top row `failure` or `cancelled` → master CI red; the prior cycle's commit broke something OR a pre-existing red state held; promote red-investigation to **Track 1 substantive of this cycle**, preempting the cycle's planned focal.
- `--limit 3` rather than `--limit 1` gives three data points: did this cycle break it, did the prior cycle break it, or is there a multi-cycle red window? Cycle 172 surfaced this kind of multi-cycle red window (master had been red since 2026-05-11, 7 days, 15+ commits in arc) that a single-row look would have masked.

## Exercise history

Seven consecutive cycles of exercise, zero misses:

| Cycle | Status at session start | SHA observed | Action taken |
|---|---|---|---|
| 172 (NOVEL@1, 2026-05-18 03:01 UTC) | **RED** — `worklog_auto_review_summary_reports_all_same_dispositions` failing | (multi-cycle red window) | Promoted to Track 1 substantive — root-cause investigation, cross-crate schema asymmetry surfaced, master GREEN restored same cycle |
| 173 (RECURRENCE-AT-2, 2026-05-18 05:19 UTC) | GREEN | `208d8d69` SUCCESS | Proceeded to planned Track 1 (tag-semantics intent sharpening) + Track 2 (executor honesty-pass) |
| 174 (RECURRENCE-AT-3, 2026-05-18 07:14 UTC) | GREEN | `a63cc591` SUCCESS | Proceeded to planned Track 1 (stemming + name-attr) + Track 2 (planner honesty-pass) |
| 175 (RECURRENCE-AT-4, 2026-05-18 09:04 UTC) | GREEN | `26019865651` SUCCESS | Proceeded to planned Track 1 (reconciler honesty-pass) + Track 2 (schema-promotion discipline note) |
| 176 (RECURRENCE-AT-5, 2026-05-18 11:15 UTC) | GREEN | `e7358be5` SUCCESS | Proceeded to planned Track 1 (C13+X2 design scope) + Track 2 (strict re-run) |
| 177 (RECURRENCE-AT-6, 2026-05-18 20:29 UTC) | GREEN | `4b2e2e4e` SUCCESS | Proceeded to planned Track 1 (C13+X2 implementation cycle 1) + Track 2 (strict re-run) |
| 178 (RECURRENCE-AT-7, 2026-05-18 22:18 UTC) | GREEN | `6f30aa00` SUCCESS | Proceeded to planned Track 1 (live-spawn design scope per #2992) + Track 2 (this analysis) |

Six greens and one red surfacing → red-acting cycle 172 produced the GREEN-recovery commit that started the green run. The discipline has therefore been exercised in BOTH polarity branches (interpretation-of-green → proceed; interpretation-of-red → preempt) within the seven-cycle window, not solely in the no-op green path.

## Threshold

Per `pattern-strength` conventions: 5+ consecutive exercises with no missed cycle and at least one exercise in each operational mode (green-path AND red-path) is the HARDEN threshold. Cycle 178 satisfies both:

- Consecutive exercises: 7 (172-178).
- Modes exercised: green-path (cycles 173-178, six instances), red-path (cycle 172, one instance — the red-discovery and root-cause action).

## Structural reasons FOR HARDENING

1. **Cost-to-execute is trivially low** — a single `gh run list` call, < 1 second, no network back-pressure.
2. **Information density is high in both polarity branches** — the green case confirms prior cycle's commit held (catches latent regressions); the red case preempts the cycle to handle the highest-value substrate (broken master CI). Either result actionably shapes the cycle's plan.
3. **Pattern has reproduced its own discovery condition** — cycle 172's NOVEL@1 was the moment the discipline was *needed but not yet codified*; cycle 173 onwards codified it; the red-path branch has not recurred yet, but the green-path has held continuously, exercising the discipline as orientation-routine even when no red is present. This is the operational mode evidence.
4. **Standing orientation cost is bounded** — once HARDENED, the check moves from forward-watch tracking to standing orientation. It no longer consumes a slot in the forward-watch decay window; it becomes routine like reading the cycle issue or checking `git status`.

## Structural reasons AGAINST HARDENING (and counter-arguments)

1. **Risk: rote box-checking** — once HARDENED, the discipline may degrade into invocation-without-interpretation, missing the next red-window.
   - **Counter:** the discipline definition above codifies that *interpretation + routing* is part of the discipline, not just invocation. A cycle that runs `gh run list` and ignores a red row has NOT exercised the discipline. The HARDENED status requires this distinction to be carried into orientation.
2. **Risk: forward-watch decay tracking removal could mask drift** — if the pattern no longer appears in cycle-by-cycle pattern updates, a future cycle might forget the standing-orientation step.
   - **Counter:** the HARDENED standing-orientation is named in cycle close as part of the orientation-prelude (e.g., in cycle 178's session-start comment, the CI check appears as a labeled step before the cycle plan). The cycle-by-cycle pattern-update section drops the `RECURRENCE-AT-N` line; the orientation-prelude section maintains the standing-step record.
3. **Risk: the green-path / red-path asymmetry has only one red data point** — the discipline's red-branch behavior (preempt-to-substrate) has been exercised exactly once (cycle 172). It's possible the red-path needs further refinement before the discipline is truly stable.
   - **Counter:** one red-path exercise is enough to demonstrate the discipline *can* preempt; the structural shape (preempt-to-Track-1) is sound and matches `drastic-change-when-incremental-failed` discipline. Future red-path instances will exercise refinements; the HARDENED status doesn't preclude evolution of the red-branch playbook.

## Resolution

**HARDENED, with operational consequence:**

- `session-start-CI-check-discipline` is removed from the forward-watch decay tracking list as of cycle 178 close.
- It is added to the standing **session-start orientation prelude**: the act of running `gh run list --branch master --workflow "Rust CI" --limit 3` + interpreting + routing is part of the routine cycle-start sequence, alongside reading the cycle issue and confirming model + run-id.
- The orientation-prelude includes the labeled invocation, the observed result, and (if red) the substrate-promotion action; (if green) a one-line confirmation that the prior cycle's commit held.
- This convention applies from cycle 179 onwards. Cycle 178 already exercised the discipline at NOVEL-status; the cycle 178 _notes will record the transition to HARDENED.

## Companion patterns

- `straight-pair-closure-as-default-two-track-shape` (HARDENED cycle 162) — the Track 1 / Track 2 composition the discipline integrates into.
- `master-CI-red-not-noticed-across-multiple-cycles` (NOVEL@1, cycle 171, INSTANCE-CLOSED cycle 172) — the failure mode this discipline prevents recurrence of.
- `live-prompts-already-aligned-at-extended-schema-level` (HARDENED cycle 175) — another HARDENED operational mode that operates similarly: standing check exercised as-needed-on-substrate-rerun rather than every-cycle. Different polarity (as-needed vs every-cycle) but same shape (HARDENED principle with operational consequence).
- `as-needed-verification-of-HARDENED-principle-on-tool-rerun` (NOVEL@1 → RECURRENCE-AT-2 cycle 177) — the meta-discipline of how HARDENED principles get re-verified when substrate changes; applies to this discipline too if a future change to CI infrastructure breaks the `gh run list` invocation.

## Forward-watch decay status post-cycle-178

Removed from forward-watch list:

- ~~`session-start-CI-check-discipline` RECURRENCE-AT-7~~ — HARDENED at cycle 178; moves to standing orientation prelude.

Standing orientation prelude (NEW section, cycle 179+):

1. Cycle issue read + model + run-id confirmation.
2. `session-start-CI-check-discipline` invocation + interpretation + routing (HARDENED cycle 178).
3. Read `input-from-eva` queue for new directives.
4. Cycle plan with Track 1 / Track 2 composition.
