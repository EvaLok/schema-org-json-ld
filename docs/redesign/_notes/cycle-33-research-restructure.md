# Cycle 33 (2026-05-01) — Research-file restructure (Eva-driven mechanism evolution)

## Context

Eva-driven restructure performed outside the normal orchestrator
cycle flow. Cycle 32 (commit `960bb007`, 2026-04-30 23:58) had just
landed the Tier-2 cross-system observations restructure based on
Copilot PR #2791 feedback. Eva opened a session 2026-05-01 asking
for a restructure of the Phase 1 research file because the single
`1-research.md` file (1422 lines / 78KB at cycle 32) had become
unwieldy: per-system writeups buried the cross-system synthesis Phase
2 will read against, and the file was on a growth trajectory (3
pending dispatches plus 2 Eva-input research targets) toward 2500+
lines.

This note documents the migration per the redesign-prompt's
`<evolve-the-mechanism>` mandate (`.github/workflows/orchestrator-redesign-prompt.xml`
lines 702-715: "Document each migration so future cycles understand
the history of the mechanism itself").

## What changed

### File-level diff

**New directory:** `docs/redesign/1-research/systems/`

**New per-system files (mechanical extraction from the prior
`1-research.md`):**
- `1-research/systems/openclaw.md` (from prior lines 65-132)
- `1-research/systems/pai.md` (from prior lines 134-215)
- `1-research/systems/autogen.md` (from prior lines 217-387)
- `1-research/systems/voyager.md` (from prior lines 389-599)
- `1-research/systems/langgraph.md` (from prior lines 601-815)

**New stub files (assembled from cross-system observations
citations + cycle-26 dispatch context, marking the asymmetry with
deeper-read systems explicitly):**
- `1-research/systems/cognition-devin.md`
- `1-research/systems/openai-harness.md`
- `1-research/systems/oh-my-codex.md`

**Rewritten file:** `docs/redesign/1-research.md` is now the index.
Section ordering: Status / Layout / Purpose & scope / Anchoring
discipline / Per-system reads (link table) / **Cross-system
observations** (moved from line 817 to roughly line 100) / Phase 1
work plan / Persistence-mechanism note (with cycle-33 evolution
paragraph appended). Cross-system observations content is verbatim
from the prior file; only two intra-file references updated to point
at per-system files instead of "see Voyager section above."

**Edits to existing files:**
- `.github/workflows/orchestrator-redesign-prompt.xml` line 746
  area: phase-1 output description amended to name the
  index + per-system-files layout.
- `docs/redesign/README.md` Layout section: 1-research.md description
  updated to name the new layout.

### What did not change

- All Phase 0 deliverables (`0-retrospective.md`, ~1276 lines).
- All `_notes/cycle-N-*.md` files (they remain process exhaust;
  cycle-22 single-system observations file is referenced from the
  new index but unchanged).
- Cross-system observations prose. Verbatim move with two minimal
  updates to swap "see Voyager section above" → "see
  `systems/voyager.md`."
- Anchoring discipline, work plan, and persistence-mechanism note
  prose. Verbatim move (the persistence note has one paragraph
  appended at the end documenting this evolution).
- `_notes/README.md` index. The 30-file scale concern noted there is
  separate from this restructure; not touched.

## Design decisions

### Why split per-system into a subdirectory rather than refactor in place

The pre-restructure file was ~1422 lines; per-system writeups (lines
65-816) were ~750 lines and growing as new systems get read. Phase
2 candidate generation reads primarily against cross-system
observations, not the per-system writeups, so burying the synthesis
below the writeups was load-bearing-readable in the wrong direction.
Split into index + subdirectory makes:
- Cross-system observations the first substantive section in the
  index (after Status / Purpose / Anchoring / link table).
- Per-system files independent. New system reads land into a new
  file in `systems/` rather than expanding the index.
- The asymmetry between deeper-read and stub-quality systems
  visible: stub files exist and are explicitly marked, rather than
  the asymmetry being hidden by stub-systems being citation-only in
  cross-system prose.

### Why keep cross-system observations in the index file rather than its own file

Cross-system observations is the load-bearing synthesis Phase 2
candidate generation will read against. It's also the section that
external feedback (Copilot dispatch #2791, etc.) targets directly.
Splitting it into its own file would add a hop without clarifying
the navigation. Keeping it in the index keeps the load-bearing
content with the navigation furniture.

### Why give Cognition / OpenAI harness / oh-my-codex stub files instead of leaving citations inline

Three options were considered:
1. **Stub files explicitly marked as such (chosen).** Surfaces the
   asymmetry. Each stub names the originating PR (#2780/#2783/#2784)
   and the still-open follow-on issue (#2779/#2781/#2782) for the
   deeper read. Pattern citations from cross-system observations
   resolve to the per-system view.
2. Leave citations inline in cross-system observations only (no
   per-system file). Rejected: hides the asymmetry; the systems-
   index link table would have entries for systems with no file
   behind them, or be inconsistent.
3. Wait for the deeper reads to land, then create files at
   deep-dive depth. Rejected: defers a structural improvement
   indefinitely on the contingency of dispatches that have been
   open since cycle 26.

The chosen option matches the redesign-prompt's discipline: name
asymmetries explicitly rather than smooth them over.

### Cycle numbering

This work is labeled "cycle 33" because it's chronologically the
next thing after cycle 32 (2026-04-30 23:58) and consistent with
the redesign mode's cycle-numbered structure. It is NOT an
orchestrator-driven cycle — Eva drove it directly in a Claude Code
session. The orchestrator's next autonomous fire should pick up at
cycle 34 (or the orchestrator will figure its own numbering from
state.json + journal grep — this note just records the convention
used here).

If the orchestrator independently labels its next cycle as 33
ignoring this note, that's a small bookkeeping conflict to
reconcile, not a structural problem. The substantive change (file
layout) is what matters; the cycle-number label is incidental.

## Open follow-ups for the next orchestrator cycle

These should be picked up when the orchestrator next fires:

1. **Cold-reader on the restructure.** Per the cycle-N-pre-commits-
   cycle-N+1-checks discipline, four bounded-mechanical questions for
   the next cycle's cold-reader pass:
   - **(a)** Cross-system observations content is verbatim move from
     prior `1-research.md` lines 817-1310 — verify by diff that no
     prose was lost in the move (only the two "see Voyager section
     above" → "see `systems/voyager.md`" updates at prior lines 1034
     and 1056 are intentional changes).
   - **(b)** Per-system files are independent of cross-system
     observations: each per-system file should read sensibly as
     standalone with no "see ... above" or "see ... below" stranded
     references back to cross-system content. Verify by reading each
     file standalone.
   - **(c)** Stub files for Cognition / OpenAI harness / oh-my-codex
     are honest about being stubs: each names the originating
     dispatch PR, the still-open follow-on issue, and is clearly
     marked as **Status: stub**. No stub claims deeper-read fidelity
     than it has.
   - **(d)** Index link table covers all 8 systems studied at depth
     plus the still-pending systems mentioned in the work plan;
     stub-marker badge consistent with each system's actual state.

2. **Eva to-decide: close-#2789-or-#2790.** The two redesign-feedback
   issues #2789 and #2790 (titled identically, created 56 seconds
   apart on 2026-04-30) appear to be a duplicate dispatch. Eva
   surfaced the question; not auto-closed by this restructure. Ask
   Eva or check the issue bodies + which one Copilot's PR #2791
   actually attached to before closing the duplicate.

3. **`_notes/` scale-migration is a separate decision.** The
   `_notes/README.md` flags ~10 files as the degradation threshold;
   the directory now holds 32 files (cycles 2-32). Topic-bucketing
   into subdirectories (`_notes/measurements/`, `_notes/integrations/`,
   `_notes/dismissed/`) was discussed and explicitly deferred. If
   the orchestrator finds notes-grepping painful, that's the
   migration to do next.

4. **Deeper reads on stub-marked systems.** Issues #2779
   (Cognition Devin) / #2781 (OpenAI harness) / #2782 (oh-my-codex)
   are still open from cycle 26. When those land, the corresponding
   stub file should grow to deep-dive depth (Voyager / AutoGen /
   LangGraph file structure as the depth target).

## Authority

The redesign-prompt's `<evolve-the-mechanism>` block (lines 702-715)
explicitly authorizes mechanism evolution: "If the initial mechanism
starts failing — too unwieldy, missing important data shapes,
friction outweighing value — replace it." The prior monolithic
shape was unwieldy (the cross-system synthesis Phase 2 reads against
was structurally buried) and on a growth trajectory (~2500+ lines
projected once pending Phase 1 reads land). The split addresses
both. Migration documented per the same block's "Document each
migration so future cycles understand the history of the mechanism
itself" mandate.
