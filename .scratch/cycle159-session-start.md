**Cycle 159 session-start** — Opus 4.7, redesign cycle 159, two-track composition continued (HARDENING-AT-12 if it lands).

**Inputs verified:**
- Audit HEAD: `8285b7d3` (unchanged; same as cycle 157/158 absorption; no new audit content)
- Open input-from-eva: 4 (#2937, #2794, #2741, #808) — all standing directives, no new since cycle 152
- Open issues: 5 (cycle issue + 4 standing input-from-eva); no closure candidates this cycle
- Cycle 158 closed both its named priorities (#1 v2-state-retention-policy + #2 v2-critique-task-class-taxonomy via symmetric closure)

**Cycle 159 plan (two-track):**
- **Track 1 (substantive focal):** `v2-state-audit` Rust tool — cycle 158 forward priority #6 (NEW from cycle 158 Track 1 §5). Walks `state/` + `docs/state.json`, classifies per-axis size/count against retention-policy thresholds, emits JSON breakdown. CORE-DESIGN-PRINCIPLE-aligned tool extraction. ~600-900 LOC + tests.
- **Track 2 (bounded ACT-NOW):** C2 (L1.2) phase marker in StepTrace — cycle 158 forward priority #1 (inherited from cycle 157 #5). Adds derived `phase` to `StepTrace` so trace consumers can distinguish semantic work from transition mechanics. Pairs with cycle 156's C5 `executed: bool`.

This continues `two-track-composition` (HARDENING-AT-12) and would be the 44th consecutive HONORING of named forward priority.

**Out of scope cycle 159:**
- v1 `tools/cycle-runner/` (forbidden zone)
- `.github/workflows/` (forbidden zone)
- This orchestrator prompt (forbidden zone)
- Per-axis archival mechanisms (cycle 158 forward priority #15 — defer until first axis approaches threshold under live execution)
- C10 amendment / X5 clarification (carry forward; Track 2 chose C2 as natural pair with cycle 156 C5)
