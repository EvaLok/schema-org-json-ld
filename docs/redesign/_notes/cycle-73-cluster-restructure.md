# Cycle 73 (2026-05-05) — Cluster catalogue restructure (deferred from cycle 72)

## Context

Orchestrator-driven restructure executing the deferral from cycle 72.
Cycles 62-72 ran the polarity-pivot research-corpus advancement arc
(eleven consecutive cycles under [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829);
cycle 73 is the twelfth). The deeper-pass implications-mining catalogue
that grew across cycles 65/70/72 synthesis cycles pushed
`docs/redesign/1-research.md` from 736 lines (post cycle 33's per-system
restructure) to 1632 lines (post cycle 72's cross-cluster intersections
synthesis) — past the cycle-33 1422-line restructure trigger.

Cycle 70's hand-off observed the file-size pressure approaching the
threshold. Cycle 72 explicitly deferred the structural restructure,
flagging Q4 in Open structural questions: "combining content addition
with structural restructure in the same cycle is risky — link breakage,
inconsistent diffs, and the editorial decisions compound." Cycle 72
recommended cycle 73 execute the deferred restructure.

This note documents the migration per the redesign-prompt's
`<evolve-the-mechanism>` mandate
(`.github/workflows/orchestrator-redesign-prompt.xml` lines 702-715:
"Document each migration so future cycles understand the history of
the mechanism itself"). Pattern parallel to cycle 33's per-system
restructure (`_notes/cycle-33-research-restructure.md`).

## What changed

### File-level diff

**New file:** [`docs/redesign/1-research/clusters.md`](../1-research/clusters.md)
(907 lines). Mechanically extracted from prior `1-research.md` lines
590-1485 with three transformations:
- Heading levels rewritten one level shallower (`## Implications-mining
  clusters` → `# Implications-mining clusters`; `### Cluster A` → `## Cluster A`;
  `#### A↔B` → `### A↔B`).
- Relative `_notes/` paths rewritten as `../_notes/` (clusters.md sits
  one directory deeper than the originals).
- Two cross-references updated: "Family-level observations above" → "Family-level
  observations in [`../1-research.md`](../1-research.md)"; "see Family E above"
  → "see Family E in [`../1-research.md`](../1-research.md)".

**Rewritten section:** the cluster section in `docs/redesign/1-research.md`
(prior lines 590-1485, ~896 lines) replaced with a brief summary block
(~65 lines) that contains:
- Section header `## Implications-mining clusters (cycles 62-72)`
  (renamed from "(cycles 62-69)" to reflect synthesis cycles 65/70/72).
- Intro paragraph naming the mining + synthesis arc and pointing to
  `1-research/clusters.md` as the load-bearing catalogue.
- Quick-reference cluster table (verbatim from prior content, kept in
  the index for navigation) with cluster A-I + theme + depth +
  implications + sub-shapes.
- Brief cluster-foregrounding summary + link to `1-research/clusters.md`
  for v1 failure-mode mapping, sub-shape catalogues, and
  cross-cluster intersection patterns.

**Edits to existing files:**
- `docs/redesign/1-research.md` Persistence-mechanism note: cycle-73
  evolution paragraph appended (parallel to cycle-33 paragraph
  appended at cycle 33).
- `docs/redesign/README.md` Layout section: `1-research.md` description
  amended to name the new `1-research/clusters.md` companion file.

### What did not change

- Cross-system observations / Family-level patterns (`1-research.md`
  lines 94-588 prior, lines 94-588 post). Verbatim. The cluster
  catalogue is a different layer and the Family-level observations
  remain in the index.
- Per-system files under `1-research/systems/`. Untouched.
- Per-cycle implication-mining files under
  `_notes/cycle-62-autogen-implications.md` through
  `_notes/cycle-72-cross-cluster-intersections.md`. Untouched.
- `1-research.md` Status, Purpose, Anchoring discipline, Per-system
  reads, Phase 1 work plan, Persistence-mechanism note (other than
  the appended cycle-73 paragraph). Verbatim move (no content
  changes).
- The redesign prompt itself. Cycle 33's restructure touched the
  prompt's Phase-1 output description; this restructure does not —
  the prompt's Phase-1 output description (`docs/redesign/1-research.md`
  plus `1-research/systems/<system>.md`) accommodates additional
  per-concern files (`1-research/clusters.md`) without prompt
  modification.

## Design decisions

### Why split the cluster catalogue to its own file rather than refactor in place

The pre-restructure file was 1632 lines / ~89KB. The cluster section
alone (lines 590-1485 prior) was ~896 lines and growing as synthesis
cycles add cross-cluster intersection patterns and Phase 2 design-input
sub-sections. Two synthesis layers coexist in the index post cycle 70
— Family-level observations (cycles 14-32 first-pass) and cluster-level
catalogue (cycles 62-69 deeper-pass) — and the cycle-33 reasoning that
"cross-system observations is the load-bearing synthesis Phase 2 reads
against; keeping it in the index keeps the load-bearing content with
navigation furniture" applies to ONE synthesis layer. With two
synthesis layers and the deeper one growing toward 1000+ lines, the
deeper layer needs its own file.

The split criterion is the same as cycle 33's: the cycle-33 1422-line
threshold remains the active threshold; the file crossed it post cycle
72 (1632 lines). The split form mirrors cycle 33: index keeps the
load-bearing summary + quick-reference table + link, deeper content
moves to its own file.

### Why keep the cluster table in the index rather than only in clusters.md

The cluster table is high-density quick reference — 9 rows naming
cluster theme, depth, implications, sub-shapes/sub-axes. It serves as
a navigation entry point for readers who need to know what clusters
exist before deciding whether to follow the link to clusters.md. This
mirrors cycle 33's per-system table which sits in the index as a
navigation entry point to the per-system files.

A reader with the index alone can answer "what are the cluster
themes?" via the index table; a reader needing v1-failure-mode
mapping or cross-cluster intersection patterns follows the link to
clusters.md. The duplication (the table appears in both index and
clusters.md) is acceptable because the index version is a quick
reference and the clusters.md version is the in-context table that
makes that file self-contained.

### Why cluster-by-cluster sub-files were not chosen

Three options were considered:
1. **Single `clusters.md` file (chosen).** Mirrors cycle-33's
   per-system-files split structurally (one file per layer-of-concern
   below the index). Cluster catalogue is coherent — clusters share
   the v1-failure-mode mapping, the cross-cluster intersection
   layer, and the open structural questions; splitting into
   per-cluster files would fragment that coherence and force
   cross-file references for the intersection layer. Single
   `clusters.md` keeps the catalogue legible end-to-end.
2. Per-cluster files (`clusters/A.md`, `clusters/B.md`, ..., one per
   cluster). Rejected: 9 small files duplicate the
   v1-failure-mode-mapping intro and force the cross-cluster
   intersection layer to live somewhere awkward (its own file,
   making it equally hard to find). The intersection layer is part
   of what makes the catalogue load-bearing — splitting clusters
   from intersections compromises the integration.
3. Wait for further synthesis to push the cluster catalogue to a
   self-evident split point. Rejected: cycle 72 already produced
   the cross-cluster intersection synthesis (~310 lines added) and
   deferred restructure on principled risk grounds (combining
   content + restructure in one cycle compounds editorial decisions).
   Cycle 73's job is the deferred restructure; further deferral
   serves no design goal beyond procrastination.

### Cycle numbering

This work is cycle 73 — orchestrator-driven, twelfth consecutive
cycle of polarity-pivot research-corpus advancement under
[#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829).
Distinguished from cycle 33 by being orchestrator-driven (Eva drove
cycle 33 directly in a Claude Code session); cycle 73 follows the
cycle-72 hand-off recommendation and runs in the standard
orchestrator cron.

## Open follow-ups for the next orchestrator cycle

These should be picked up when the orchestrator next fires:

1. **Cold-reader on the restructure.** Per the cycle-N-pre-commits-
   cycle-N+1-checks discipline, three bounded-mechanical questions for
   cycle 74's cold-reader pass:
   - **(a)** Cluster section content is verbatim move from prior
     `1-research.md` lines 590-1485 to `1-research/clusters.md`
     (with heading-level adjustment + path adjustment + two
     cross-reference adjustments). Verify by diff that no prose was
     lost in the move (only the documented adjustments are intentional
     changes).
   - **(b)** Index summary section in `1-research.md` accurately
     summarizes what clusters.md contains. Verify by reading the
     index summary standalone — does it tell a reader what they'd
     find at clusters.md?
   - **(c)** Cross-references survive the split. Verify by grep:
     "see Family E", "Open structural questions", "_notes/" paths in
     clusters.md should resolve correctly.

2. **Dispatch state.** [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833)
   oh-my-codex deeper-read still pending Eva's manual Copilot
   assignment per cycle 71's diagnosis comment;
   [#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842)
   PAI deeper-read also pending. If Eva's manual assignment lands
   between cycle 73 and cycle 74, dispatches will start processing.
   Per-finding evaluation absorption (highest-priority cycle
   composition shape per cycle-72 hand-off) activates when either
   dispatch returns.

3. **Continued synthesis arc.** With cluster catalogue extracted to
   its own file, future synthesis cycles can grow `clusters.md`
   without re-triggering the index file-size threshold. Cycle 72's
   four flagged additional intersections (A↔C, B↔C, F↔I, E↔I) remain
   available for deeper synthesis. Re-mining existing systems at
   deeper depth is also available if no dispatch returns by cycle 75+.

## Authority

The redesign-prompt's `<evolve-the-mechanism>` block (lines 702-715)
explicitly authorizes mechanism evolution: "If the initial mechanism
starts failing — too unwieldy, missing important data shapes,
friction outweighing value — replace it." The pre-restructure shape
was unwieldy (1632 lines past the 1422-line trigger; two synthesis
layers crowded into the index). The split addresses both.

The split form also matches the redesign-prompt's `<persistence>`
discipline (lines 686-696): "Treat the mechanism as part of the
system being designed, not as scaffolding that's settled once
picked." The persistence mechanism evolved from monolithic
`1-research.md` (pre cycle 33) → index + per-system files (cycle 33)
→ index + per-system files + per-concern catalogue (cycle 73). Each
evolution responded to specific friction; each is documented.
