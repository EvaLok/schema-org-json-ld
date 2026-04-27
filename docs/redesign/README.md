# Redesign workspace

This directory holds the multi-cycle redesign of the orchestrator pipeline.
The redesign-mode prompt at `.github/workflows/orchestrator-redesign-prompt.xml`
is authoritative for the work that happens here.

## Layout

- **`0-retrospective.md`** — Phase 0 working draft. Honest catalog of v1
  failure patterns. Iterates across many cycles. Required reading before
  designing v2.
- **`1-research.md`** — Phase 1 (later). External-system study (openclaw,
  PAI, others).
- **`2-candidates.md` / `2-selection.md`** — Phase 2 (later). At least
  two distinct design candidates with rationale for selection.
- **`3-experiments/`** — Phase 3 (later). Prototype, experiments,
  hypothesis log.
- **`_notes/`** — Working notes between checkpoints. Cycle-by-cycle
  observations, half-formed thoughts, dismissed alternatives. Not
  artifacts — process exhaust.

## Persistence convention (interim, will evolve)

Cycle 1 establishes minimal persistence:
- This README is the navigation index.
- `0-retrospective.md` is the only Phase 0 deliverable.
- `_notes/cycle-N-*.md` files capture per-cycle observations that feed
  into the deliverable but aren't themselves the deliverable.

The persistence mechanism itself is subject to redesign. If it gets
unwieldy, replace it. Document migrations in `_notes/`.

## Checkpoint state

| Checkpoint | Status | Eva approval |
|---|---|---|
| Post-retrospective (after Phase 0) | Iterating | — |
| Candidate-selection (after Phase 2) | Not started | — |
| Pre-cutover (before Phase 4) | Not started | — |

This table is updated as checkpoints reach review-ready state.

## Iteration log on `0-retrospective.md`

Multi-cycle iteration on the Phase 0 retrospective. Each entry: cycle
number, headline change, source-of-evidence file. Eva will review the
artifact at the post-retrospective checkpoint when she judges it ready.

| Cycle | Headline change | Source |
|---|---|---|
| Redesign cycle 1 (2026-04-26) | Initial draft. 10 failure patterns F1–F10, working hypotheses, open-questions section. | `docs/journal/2026-04-26.md` |
| Redesign cycle 2 (2026-04-27) | F7 sharpened with measurement: 0 schema-source commits across cycles 500–545; updated success criteria with measurable threshold; sharpened "what appears to be working" claims with caveats from F2/F8 evidence. | `_notes/cycle-2-measurements.md`, `docs/journal/2026-04-27.md` |
| Redesign cycle 3 (2026-04-27) | Integrated audit-repo #442's structural critique. F9 hypothesis corrected (model-class explanation was wrong: both audit and main are Opus 4.7; real diversity comes from prompt+state+context). F10 rewritten with property-1 (broader read scope, real) vs property-2 (different model perspective, illusory) distinction. Added F11 (cycle closure as artificial completion) and F12 (late-stage defense accumulation catalog hint). Added shared-root preamble unifying F2/F3/F4/F8/F11 as asymmetric/write-mostly. Added foreground-Eva caveat, persistence-mechanism preservation, schema-work threshold rewrite (gameable + finite-types concerns), measurement-not-impression v2 design principle, F1 audit-side parallel evidence. Open questions 5 partially resolved; OQ7 (F12 catalog) and OQ8 (F11 verification) added. Retrospective grew 526 → 799 lines. | `_notes/cycle-3-audit-442-integration.md`, audit-repo issue #442, `docs/journal/2026-04-27.md` |
| Redesign cycle 4 (2026-04-27) | F1 measurement (constraint-vs-tool ratio): 12 of 13 (92%) close-out constraint additions are constraint-only with no paired tool fix; 13 of 13 (100%) cite recurrence-escalation, meaning each constraint was insufficient and the failure pattern recurred; one constraint (`rerun-step-comment-refresh`) self-documents as "Behavioral fix in lieu of tool dispatch" — single-case existence proof of F1 mechanism in v1's own text. Adversarial re-read of audit-derived additions: 8 of 9 hold up unchanged; F8's inclusion in the shared-root preamble qualified (F8's primary root is parallel-implementation duplication, not asymmetric communication; preamble now marks F8 as adjacent rather than centered on the asymmetric-communication root). Cycle 3's 100%-acceptance rate flag examined and resolved: integrations were substantively defensible, only one framing nuance needed. Retrospective grew 799 → 829 lines. | `_notes/cycle-4-adversarial-reread.md`, `_notes/cycle-4-f1-measurement.md`, `docs/journal/2026-04-27.md` |
| Redesign cycle 5 (2026-04-27) | F12 catalog sub-(a): all 42 top-level state.json keys categorized — 19 D / 13 P / 10 M (45%/31%/24%); 62%+ defense-character; 4× state-surface reduction estimate for v2. F5 strengthened with the catalog measurement and added to shared-root preamble at full level (not adjacent like F8) — F5 is the field-level instance of write-mostly state. F11 verification (OQ8 resolved): 4.3 post-close state mutations per cycle averaged across cycles 543/544/545; 2 of 3 cycles had new dispatches recorded post-`cycle-complete` (#2733, #2738); 5 distinct fields routinely mutated post-close (4 of 5 are F12-cataloged defenses); 0 of 5 reconciled in frozen worklog. F12 hypothesis updated with cross-substrate framing (F1 prompt-layer / F5 state-shape-layer / F11 temporal-layer / F12 cross-substrate meta-pattern); the "Phase 2 cannot proceed without complete catalog" claim relaxed to allow Phase 2 against a partial catalog with TBC rationale. Iteration plan updated with "connect across patterns" entry. Copilot feedback-only dispatch deferred again (3rd time) with explicit cycle-6 commitment to either build the mechanism or document a manual procedure. Retrospective grew 829 → 958 lines. | `_notes/cycle-5-state-categorization.md`, `_notes/cycle-5-f11-post-close-measurement.md`, `docs/journal/2026-04-27.md` |
| Redesign cycle 6 (2026-04-27) | (1) Copilot feedback-only dispatch executed (#2748, gpt-5.4) — fourth defer would have demoted the item; cycle-5 commitment honored. Dispatch procedure documented for future re-runs. (2) Adversarial re-read of cycle 5's F1+F5+F11+F12 mechanical connection: count overlap (4-of-5 D-cataloged) is consistent with the 62-69% defense-character base rate (~37% probability under random sampling) and is **not load-bearing on its own**; the mechanical claim rests on the named tool→field→trigger chain — `verify-review-events` writing `review_events_verified_through_cycle` 9-11m post-close in 3/3 cycles, `metric-snapshot` writing `field_inventory.fields.*` 0-3m post-close in 2/3 cycles, `pipeline-check`'s C5.5 write being post-close by construction, `record-dispatch` writing `agent_sessions` post-close in 2/3 cycles. F11 section, shared-root preamble, and F12 hypothesis paragraph rewritten to lead with mechanism and acknowledge the base-rate concern explicitly. (3) F12 catalog sub-(b): 36 pipeline-check functions (30 logical checks after collapsing 6 date-variant pairs) categorized as 22 D / 5 P / 3 M = 83% defense-character — meaningfully more lopsided than state.json's 62-69%. Cluster analysis: post-dispatch reconciliation cluster (5), chronic-category cluster (3), worklog-immutability cluster (3), step-comment cluster (3) — defenses cluster around F-patterns the retrospective already names. Estimated v2 pipeline-check size: 5-7 of 30 logical checks (4-6× reduction), comparable to cycle 5's 4× state-surface reduction. Retrospective grew 958 → 1034 lines. | `_notes/cycle-6-feedback-dispatch.md`, `_notes/cycle-6-mechanism-sharpening.md`, `_notes/cycle-6-pipeline-check-categorization.md`, `docs/journal/2026-04-27.md` |
