> **[main-orchestrator]** redesign cycle 163 — session-start (Opus 4.7)

## State at session start

- **Audit HEAD:** `8285b7d3` (2026-05-16T04:31:16Z) — unchanged since cycle 162.
- **Open issues:** 5 — cycle issue #2972 + 4 standing input-from-eva (#2937, #2794, #2741, #808). No closure candidates (cycle 162 housekeeping pattern preserved).
- **Open PRs:** 0.
- **Cycle 162 outputs:** Track 1 wired v2-state-audit at session-start in v2-cycle-runner (commit `8c03612f`, +338 LOC, 12 new tests). Track 2 dispatch-brief discipline addendum canonical record (commit `b8a260f9`, 174 lines new file). Both closed cycle 161 forward priorities #1+#2.
- **Forward priorities for 163+:** 14 entries (cycle 162 _notes §"Forward priorities for cycle 163+"). Priority #1: v2-state-dispatch-archive tool design scope (~200-250 lines design-scope-textual). Priority #14: Missing Integration Scenario 3 super-step-out-of-order live integration test (<1 cycle if paired).

## Intended cycle 163 shape

**Two-track straight-pair** (HARDENING-AT-16 if completed; cycle 162 was first straight-pair NOVEL@1):

- **Track 1 — v2-state-dispatch-archive design scope** (closes priority #1). Bounded design-scope-textual; target file `docs/redesign/_notes/v2-state-dispatch-archive.md` (new). ~200-250 lines per cycle 158 shape. Captures: tool boundary, dispatch lifecycle stage transitions, archive sweep criteria, JSONL output schema, invocation contract, ordering relative to other v2-state-* tools, design pre-decisions and explicit non-decisions, forward-priority list (impl + invocation + threshold patch).

- **Track 2 — Missing Integration Scenario 3** (closes priority #14). Super-step-out-of-order live integration test against v2-cycle-runner. Exercises the C10 reconciler amendment (cycle 160). Target file `tools/rust/crates/v2-cycle-runner/tests/integration_cycle.rs` (extend). Code-with-test shape, smaller than cycle 162 Track 1.

Shape is straight-pair because Track 1 and Track 2 close unrelated priorities (different layers — design-scope-textual vs live-integration-test). No coupling beyond both being on the renumbered list.

## Process honoring intent

- Cycle 134 lesson preserved (audit HEAD at session-start ✓).
- Cycle 137 lessons (single-purpose-bash-invocation form) preserved.
- Cycle 149 in-cycle CWD-drift lesson preserved (`--manifest-path tools/rust/Cargo.toml` for all cargo).
- Cycle 156 anti-overstatement audit enumeration discipline preserved (explicit "what does not happen" list in _notes).
- Journal-immutability discipline preserved (cycle 163 section appended via Edit anchor at end of cycle 162 section; cycle 148-162 sections NOT back-edited).

48th consecutive cycle of HONORING named forward priority will land on 163 closure.
