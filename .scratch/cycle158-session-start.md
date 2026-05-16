> **[main-orchestrator]** cycle 158 session-start

**Model:** `claude-opus-4-7`
**Cycle:** 158 (redesign mode, Phase 3)
**Predecessor commit:** `15ce9e0b` (cycle 157)
**Date:** 2026-05-16

## Session-start checks

- **Audit HEAD:** `8285b7d3` (unchanged from cycle 157 absorption; no new audit content this cycle).
- **Input-from-eva:** 4 open issues (#2937, #2794, #2741, #808) — all standing directives already absorbed; no new input.
- **Open issues:** 5 total (cycle issue #2967 + 4 standing input-from-eva). No new SECTION 6b closure candidates this cycle.

## Composition

Two-track composition continues — HARDENING-AT-10 candidate becomes HARDENING-AT-11 if both tracks land. 7 consecutive post cycle 151 single-track exception (152-158); 11 of 12 in arc 146-157 + cycle 158 makes 12 of 13.

### Track 1 — `v2-state-retention-policy` design scope draft

Cycle 157 forward priority #1, produced by audit cycle 221 R2 absorption (A1 axis-coverage instance against Step 13.1 defense itself). Target file: `docs/redesign/_notes/v2-state-retention-policy.md`. Per-axis retention thresholds (advisory/mandatory/hard) mirroring audit's Step 13.1 model. Candidate v2 state axes:

- channel-state payload retention (cycles or absolute bytes)
- step-trace retention (currently per-cycle in `StepTrace`)
- per-role output retention (role-output files; cleanup cadence)
- state.json dispatches array (append-only via `v2-state-dispatch-sync`)

Estimate: substantive design doc, ~300-500 lines.

### Track 2 — `v2-critique-task-class-taxonomy` design scope draft

Cycle 157 forward priority #2, produced by audit cycle 221 R5.b absorption (Copilot-as-adversarial-critique-parallel-pattern + audit's four distinctive properties). Target file: `docs/redesign/_notes/v2-critique-task-class-taxonomy.md`. Formalizes audit-class vs Copilot-class vs hybrid-class assignment criteria. The taxonomy was sketched in cycle 157 _notes lines 145-166 — Track 2 extracts and formalizes into a standalone doc.

Estimate: bounded extraction, ~150-250 lines.

Both new cycle 157-produced priorities close in cycle 158 — symmetric closure pattern (cycle 157 produced 2 new priorities, cycle 158 closes both).

## Cycle 158 will NOT

- Modify legacy `tools/cycle-runner` (v1; forbidden zone preserved during Phase 3).
- Modify `.github/workflows/` or this orchestrator prompt.
- Implement C2 / amend C10 / address X5 / 22 DEFER findings (cycle 157 priorities #3, #4, #5; carry forward).
- Spawn live Claude sessions (role-driver remains SCAFFOLD).
- Modify `docs/redesign/2-selection.md` (cycle 120 L2 preserved).
- Open or close issues beyond cycle issue itself (no SECTION 6b candidates this cycle).
- Dispatch any Copilot or audit-request work.

## Process honoring (anticipated)

- **43rd consecutive cycle of HONORING named forward priority** (cycles 115-158).
- 71st bottleneck-asynchronous cycle (78-158).
- 48th non-per-candidate-sharpening cycle (111-158).
- Two-track composition continued — 7 consecutive post cycle 151 exception (HARDENING-AT-11 if both tracks land).

Session-end will report on actual artifacts and any in-session findings.
