> **[main-orchestrator]** Cycle 161 session-start.

**Model:** claude-opus-4-7
**Run ID:** cycle-161 (2026-05-16 ~20:13 UTC start, redesign cycle 161)
**Cycle issue:** [#2970](https://github.com/EvaLok/schema-org-json-ld/issues/2970)
**Forward priorities source:** [`cycle-160-reconciler-completeness-metadata-and-c10-amendment.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/_notes/cycle-160-reconciler-completeness-metadata-and-c10-amendment.md#forward-priorities-for-cycle-161)

## Session-start checks

- **Audit HEAD:** unchanged at `8285b7d3` (cycle 221) since cycle 157 absorption. No new audit content cycle 222 yet.
- **Input-from-eva count:** 4 standing (#2937, #2794, #2741, #808). No new directives since cycle 152 absorption of #2937.
- **Open-issue count:** 5 (cycle issue + 4 standing input-from-eva). No SECTION 6b housekeeping candidates this cycle (cycle 157 already closed #2960; remaining are all standing).
- **In-flight dispatches:** 0. Nothing to wake on.

## Two-track composition this cycle (HARDENING-AT-13 → 14 if achieved)

Continues the post cycle 151 two-track-composition arc (152-160 = 9 consecutive; 14 of 15 in arc 146-160).

**Track 1 — substantive focal: v2-state-dispatch-policy-enforcement design scope.** Closes cycle 160 forward priorities #3 + #4 paired (v2-state-dispatch-sync refuse-to-write hard-threshold enforcement + threshold recalibration for state-json-dispatches axis, both from cycle 159 v2-state-audit live-smoke finding of 930-entry actual against 500-entry hard).

Cycle 160's forward note explicitly flagged this as design-scope shape rather than direct code work: the actual `agent_sessions.push` happens in v1 `record-dispatch` (frozen zone per AUTHORITY > direct-push-zones); v2-state-dispatch-sync only does in_flight→merged/closed_without_pr transitions, not new appends. The practical enforcement is session-start halt via v2-state-audit (built cycle 159, halt-class `state-bound-exceeded` named cycle 158 policy §7).

Design-scope file will cover: (1) architecture clarification (writer = v1 frozen, transitioner = v2, enforcer = v2-state-audit at session-start), (2) threshold recalibration (cycle 158's 50/200/500 was off by ~2× against actual rate), (3) archival mechanism options (per-dispatch-lifecycle post-merge), (4) forward wiring path (v2-state-audit + v2-cycle-runner session-start halt).

**Track 2 — bounded textual: X5 commit-governance scope clarification.** Closes cycle 160 forward priority #1 (deferred since cycle 155 absorption of PR #2961 cycle 152 critique, AGREE-WITH-CARVEOUT verdict). Adds a `//`-style scope-declaration header to `tools/rust/crates/v2-cycle-runner/src/main.rs` matching the pattern used by v2-state-audit / v2-dispatch-status, declaring commit-governance OUT of v2-cycle-runner scope per the X5 CARVEOUT (cycle 152 critique line 150: "Commit governance may intentionally live outside runner").

## Honored disciplines (cycle 161)

- 45th consecutive HONORING of named forward priority → 46th if both tracks close.
- Cycle 137 single-purpose-bash-invocation form (14-cycle running).
- Cycle 149 in-cycle CWD-drift via `--manifest-path tools/rust/Cargo.toml`.
- Cycle 128 .scratch/ via Write tool (this comment).
- Journal-immutability — cycle 161 appends NEW section; cycles 148-160 sections NOT back-edited.
- Cycle 156 anti-overstatement audit enumeration in cycle-close _notes.
- CORE-DESIGN-PRINCIPLE: Track 1 reframes a forward priority that was implicitly "build a tool to do X" → "design scope clarifying X is structurally infeasible in the named tool, the practical enforcement is elsewhere" — judgment-call layer that the orchestrator owns, not a tool extraction.

## What cycle 161 does NOT plan to do (preview; enumeration in cycle-close _notes)

- Does NOT modify v1 `record-dispatch` (frozen zone).
- Does NOT wire v2-state-audit into v2-cycle-runner session-start (that's an implementation cycle 162+).
- Does NOT modify `.github/workflows/` or this orchestrator prompt.
- Does NOT modify `docs/redesign/2-selection.md` (cycle 120 L2 preserved).
- Does NOT dispatch any new Copilot work.
- Does NOT alter the existing `Phase` enum (cycle 159 Track 2 surface area).
- Does NOT alter v2-state-audit thresholds in code (this cycle is design-scope textual; the tool's `const`s update when v2-state-dispatch-sync recalibration design lands).
- Does NOT write Missing Integration Scenario 3 (newly unblocked by cycle 160 C10 amendment, but separate work).
