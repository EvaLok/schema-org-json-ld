# Redesign cycle notes — index

Per-cycle working notes for the redesign. The deliverables live in the
parent directory (`0-retrospective.md`, later `1-research.md`,
`2-candidates.md`, etc.); these files capture working-out, measurement
runs, dismissed alternatives, and half-formed thoughts that don't belong
in the artifacts themselves.

The pattern is `cycle-N-<topic>.md` per the parent README's persistence
convention. Cycles may have multiple notes files; topic suffix
disambiguates. The point of the notes is process exhaust — the artifact
gets only the load-bearing conclusion, the notes capture the working-out
that produced it.

## Index

| File | Cycle | Topic |
|---|---|---|
| [`cycle-2-measurements.md`](cycle-2-measurements.md) | redesign cycle 2 (2026-04-27) | F7 schema-vs-self-management measurement: cycles 500–545 produced zero schema-source commits |
| [`cycle-3-audit-442-integration.md`](cycle-3-audit-442-integration.md) | redesign cycle 3 (2026-04-27) | Integration of audit-repo issue #442's structural critique into `0-retrospective.md`: F9/F10 hypothesis corrections, F11/F12 additions, shared-root preamble, schema-work threshold rewrite, measurement-not-impression principle |
| [`cycle-4-adversarial-reread.md`](cycle-4-adversarial-reread.md) | redesign cycle 4 (2026-04-27) | Adversarial re-read of cycle 3's audit-derived additions: 8 of 9 hold up; F8's role in the shared-root preamble qualified as adjacent, not centered |
| [`cycle-4-f1-measurement.md`](cycle-4-f1-measurement.md) | redesign cycle 4 (2026-04-27) | F1 measurement (constraint-vs-tool ratio in close-out constraint blocks cycles ~430–509): 12/13 constraint-only (no paired tool fix), 13/13 cite recurrence-escalation, one self-documents as "behavioral fix in lieu of tool dispatch" |
| [`cycle-5-state-categorization.md`](cycle-5-state-categorization.md) | redesign cycle 5 (2026-04-27) | F12 catalog sub-(a): all 42 top-level state.json keys categorized — 19 D / 13 P / 10 M; 62%+ defense-character; preliminary v2 transfer-or-eliminate decision per field; estimate of ~4× state-surface reduction in v2 |
| [`cycle-5-f11-post-close-measurement.md`](cycle-5-f11-post-close-measurement.md) | redesign cycle 5 (2026-04-27) | F11 verification: 4.3 post-close state mutations per cycle averaged across cycles 543/544/545; 2 of 3 cycles had new dispatches recorded post-`cycle-complete`; 5 distinct fields routinely mutated post-close, 4 of 5 are F12-cataloged defenses; 0 of 5 reconciled in frozen worklog |

## Conventions

- Filename `cycle-N-<topic>.md`. If a cycle has multiple distinct topics, use multiple files (`cycle-5-f1-measurement.md`, `cycle-5-f12-state-fields.md`) rather than packing everything into one.
- Each notes file states cycle, source(s), purpose at the top.
- Dismissed-but-evaluated material lives here, not in the parent artifact. The artifact records the conclusion; the notes record what was considered and why.
- The index above is updated whenever a new notes file is created. Future cycles can grep `_notes/` to find prior measurements without re-running queries.

## Scale failure mode (flagged in cycle 3 retrospective)

This pattern is two cycles old at time of writing (2026-04-27). Audit #442 observes it's working but at very small scale. Likely degradation point: ~10 files, when scrolling the index becomes inefficient and topic-tagging is more useful than chronological listing. Migration option (when needed): replace flat `_notes/` with topic subdirectories (`_notes/measurements/`, `_notes/integrations/`, `_notes/dismissed/`), or build a tool to render the index with topic groupings. Defer until pain is real; document migration in this README when it happens.
