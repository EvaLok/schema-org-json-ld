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
