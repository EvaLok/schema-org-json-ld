# Cycle 34 (2026-04-30) — Cold-reader on cycle-33 restructure + first HOUSEKEEPING-section sweep

## Setup

Cold-start session. Cron fired 2026-04-30 17:33 UTC (issue #2795). The
prior cycle (cycle 33, commit `46cf1820`) was Eva-driven outside the
orchestrator flow — Eva opened a session 2026-05-01 (UTC+8) and split
`docs/redesign/1-research.md` into index + per-system files in
`docs/redesign/1-research/systems/`. The cycle-33 notes file
(`_notes/cycle-33-research-restructure.md`) named four bounded-mechanical
cold-reader questions for the next orchestrator cycle, and four
follow-ups beyond that. Cycle 34's substantive work is the cold-reader
verification; the new HOUSEKEEPING section in the prompt (commit
`4af239a8`) shaped the second thread.

## Cold-reader on the cycle-33 restructure: 4/4 PASS

### Question (a) — verbatim-move check

**PASS.** The cross-system observations section (494 lines in the
prior file at lines 817-1310; 496 lines in the current file at lines
94-589) preserves prose verbatim except for the two documented
intentional updates: "(see Voyager section above)" → "(see
[`systems/voyager.md`](1-research/systems/voyager.md))" at the
mechanical-enforcement bullet (current line 311) and the
iteration-ceilings bullet (current line 335). Verification:
- Family headers identical (5 families, same names, same pattern
  counts: C/5, E/4, A/3, B/3, D/1).
- 16 maturity badges identical between cycle-32 commit `960bb007`
  and current head, in the same sequence and exact text (verified
  via `grep -E -o '\*\*.\[[^]]+\]'`).
- No other "see X section above/below" references in the current
  file beyond the link table (line 83) and a per-system file
  reference at line 611. The two intentional Voyager updates are
  the only intra-text edits.

The +2-line section size (494 → 496) is consistent with the two
markdown-link replacements, which lengthened the affected sentences
just enough to wrap onto an additional line each.

### Question (b) — per-system files standalone-readable

**PASS.** All 8 per-system files
(`openclaw.md`, `pai.md`, `autogen.md`, `voyager.md`, `langgraph.md`,
`cognition-devin.md`, `openai-harness.md`, `oh-my-codex.md`) read
sensibly as standalone documents. None contains stranded "see X
above" or "see X below" references back to the cross-system
observations content. Each file follows the same structural pattern:

1. Title with system identifier (and source repo / scale / language
   where applicable).
2. Back-link to Phase 1 index (`[← back to Phase 1 index](../../1-research.md)`).
3. Sources section (or a status section for stubs).
4. Architecture / patterns content.
5. Anchoring caveats (named with their own discounting argument).
6. "Patterns observed in {system}" bullet list at the end with
   "(relevance evaluation deferred to cross-system synthesis, gated
   on multi-system reading)" caveat.

Spot-read confirmed: the openclaw and Voyager files (the smallest
and largest deeper-read files at 70 and 213 lines respectively)
both stand on their own without requiring the reader to first
have read cross-system observations.

### Question (c) — stub files honest about being stubs

**PASS.** The three stub files
(`cognition-devin.md`, `openai-harness.md`, `oh-my-codex.md`) all
follow an identical convention:
- Bold marker `**Status: stub.**` near the top.
- Origination named: PR number for the originating Copilot research
  dispatch (#2780 / #2783 / #2784 respectively).
- Pending deeper read named: open issue number (#2779 / #2781 /
  #2782 respectively).
- "To-be-completed" closing section reaffirming the deeper-read
  follow-up.

No stub claims deeper-read fidelity than it has. The Cognition stub
explicitly carries the cycle-26 source-access caveat
(`cognition.ai` was firewall-blocked from the dispatch environment;
substance came via secondary sources) — this is more conservative
honesty than the other two stubs need to maintain.

### Question (d) — index link table coverage and stub-marker consistency

**PASS.** Link table at `1-research.md` lines 78-87 covers all 8
systems with working markdown links to per-system files. The
"Status / depth" column honestly reflects depth-disparity rather
than smoothing it:
- openclaw / PAI: "First-pass: README + VISION.md" / "First-pass:
  README"
- AutoGen / LangGraph: "Deep-dive" (full Copilot dispatch
  deliverables)
- Voyager: "Code-level: agents + prompts" (orchestrator-direct
  code-read)
- Cognition Devin / OpenAI harness / oh-my-codex: "**Stub** —
  deeper read pending #NNNN" (bold marker, identical phrasing
  across all three stubs)

The bold "**Stub**" marker visually distinguishes the Copilot-
dispatch-only-with-pending-deeper-read systems from the orchestrator-
direct-but-still-first-pass systems (openclaw / PAI). This is
appropriate asymmetry — the stub category is structurally distinct
from "first-pass orchestrator read" because the deeper read is
explicitly tracked as an open issue with the same dispatch-origin
metadata.

### Cold-reader verdict summary

4/4 PASS. The cycle-33 restructure preserved all properties
the cycle-33 notes file's follow-up #1 named. Eva-driven structural
work on a load-bearing artifact passed all four bounded-mechanical
verification questions on the next orchestrator cycle.

## Housekeeping: first HOUSEKEEPING-section sweep

The redesign-prompt's SECTION 6b (added commit `4af239a8`) names
issue-tracker / draft-PR hygiene as periodic bounded-mechanical work
with a 2-4-closures-per-cycle cadence. This is the first cycle
applying that section. Prior to this cycle: 25 open issues over
2 months; 11 open PRs of which 10 are drafts.

### Closures executed (3 items, the cycle-30 dispatch cluster)

Cycle-33 follow-up #2 explicitly flagged the duplicate
redesign-feedback issues #2789 and #2790 (created 7 seconds apart on
2026-04-30 by EvaLok with identical title and body) and named the
resolution procedure: "check the issue bodies + which one Copilot's
PR #2791 actually attached to before closing the duplicate." The
check confirmed PR #2791's `closingIssuesReferences` lists #2790,
making #2789 the orphan duplicate.

Closures with linking comments naming the absorbed-content location
per the HOUSEKEEPING `<closure-discipline>`:

1. **#2789** — closed as duplicate. Comment names #2790 as the
   live issue and points at the cycle-31 verdict file +
   cycle-32 in-place restructure as the absorbed-content
   destination.
2. **#2790** — closed as absorbed. Comment names PR #2791
   as the artifact, the cycle-31 per-finding-evaluation
   (47/7/6 verdicts), and the cycle-32 in-place restructure
   commit (`960bb007`).
3. **PR #2791** — closed as absorbed (feedback-only dispatch,
   never intended to merge per dispatch labeling). Comment names
   the seven cycle-31 structural decisions and where each
   landed in the cycle-32 in-place restructure.

### Closures considered but deferred to subsequent cycles

The cycle-33 notes observation that "10 of 11 open PRs are drafts
that were never intended to merge" suggests a larger sweep is
warranted. I considered the full set:

| Item | Status | Why defer |
|---|---|---|
| PR #2784 (oh-my-codex research, cycle 26) | Absorbed: cross-system + stub file | Not flagged by cycle-33; sweeping all 6 cycle-26-or-earlier PRs in one cycle exceeds the 2-4-closures cadence guideline. |
| PR #2783 (OpenAI harness research, cycle 26) | Absorbed: cross-system + stub file | Same as above. |
| PR #2780 (Cognition Devin research, cycle 26) | Absorbed: cross-system + stub file | Same as above. |
| PR #2768 (LangGraph research, cycle 18) | Absorbed: full per-system file | Same as above. |
| PR #2763 (AutoGen research, cycle 15) | Absorbed: full per-system file | Same as above. |
| PR #2756 (Phase 0 critique 2nd dispatch, cycle 11) | Absorbed in cycle 12 verdict | Same as above. |
| PR #2749 (Phase 0 critique 1st dispatch, cycle 6) | Absorbed in cycle 7 verdict | Same as above. |
| #2767 (LangGraph dispatch issue, cycle 18) | Absorbed via PR #2768 | Pair with PR #2768. |
| #2762 (AutoGen dispatch issue, cycle 15) | Absorbed via PR #2763 | Pair with PR #2763. |
| #2755 (Phase 0 critique 2 issue, cycle 11) | Absorbed via PR #2756 | Pair with PR #2756. |
| #2748 (Phase 0 critique 1 issue, cycle 6) | Absorbed via PR #2749 | Pair with PR #2749. |

Cadence rationale: cycle 34 has done 4 substantive cold-reader
checks plus the cycle-30 cluster of 3 closures. Per HOUSEKEEPING
`<cadence>`: "A cycle that's heavy on substantive work doesn't
need to sweep." The substantive cold-reader work + the explicitly-
flagged cycle-30 cluster fits the bounded-mechanical capacity.
The deferred closures are not urgent — they're noise reduction,
not blocked work.

Plan: subsequent cycles' housekeeping passes pick up 2-4 of the
deferred closures each, with the same linking-comment discipline.
Three or four cycles of housekeeping at the 2-4 cadence will
converge to a clean state.

### Items left explicitly open per HOUSEKEEPING `<what-not-to-close>`

- All `input-from-eva` issues that name ongoing constraints:
  #2794 (firewall allowlist), #2775/#2774 (research targets —
  active substrate), #2759 (Phase 1 authorized — phase-scoped
  ongoing), #2741 (redesign mode active — phase-scoped ongoing),
  #2408 / #2039 / #809 / #808 / #699 / #436 (older directives;
  many predating redesign mode but some still load-bearing).
- Research-only follow-up issues for stub systems: #2779
  (Cognition Devin), #2781 (OpenAI harness), #2782 (oh-my-codex)
  — deeper orchestrator-direct reads still pending; corresponding
  stub files reference these issue numbers as the tracking points.
- Pre-redesign workflow / bug / cycle-review issues: #2738, #2736,
  #2729, #2627, #2272 — uncertain absorption status without deeper
  investigation; HOUSEKEEPING says "anything where 'absorbed' is
  uncertain. When in doubt, leave it open."
- Pre-redesign PRs (not draft, or with unclear absorption status):
  #2730 (workflow-change dispatch-task — non-draft, may need Eva
  decision), #2739 (cycle 545 adversarial review), #2737
  (write-entry --auto-blockers).

## Observations for the design-input meta-loop

The HOUSEKEEPING `<meta-observation>` block names "patterns observed
during housekeeping are themselves design-input for v2 candidate
generation." This first sweep surfaced a few:

### Eva-driven structural change happened cleanly without orchestrator pre-coordination

Cycle 33 happened outside the orchestrator flow — Eva opened a
Claude Code session and made a substantial structural change
(`1-research.md` split into 8 per-system files + index). The
cycle-33 notes file documented the change with cold-reader
questions for the next orchestrator cycle. The cycle-34
orchestrator picked it up and verified the migration cleanly.

This is a useful pattern observation: **two orchestrators (Eva-as-
operator session + autonomous orchestrator) can coordinate via
notes files + named follow-up questions, without explicit
synchronization protocols.** The cycle-33 notes file functioned
as a hand-off contract; cycle-34 honored it. Worth noting for v2:
the human-in-the-loop case for occasional substantial structural
changes works without bespoke machinery, IF the persistence
mechanism (notes files + journal) carries enough information
across the boundary.

### Duplicate-issue-from-UI-hiccup is a real failure mode the dispatch tooling could prevent

#2789 / #2790 came from Eva's UI hiccup — the same dispatch fired
twice in 7 seconds. The Copilot agent only engaged the second
issue, leaving the first as an orphan that needed manual housekeeping
later. A v2 dispatch-task tool could:
- Detect "title + body identical to a recent issue" and warn /
  dedupe before creating a second issue.
- OR record an idempotency key per dispatch attempt so retries
  don't create new issues.

This is bounded-mechanical and small-scope; not load-bearing for v2
candidate generation but worth a one-line entry in the tool-suite
considerations for the eventual v2 prompt. Captured here; will
re-surface if the same pattern recurs.

### "Many drafts that never merge" is structurally invisible to GitHub

Eleven open PRs, ten of them drafts that were never intended to
merge per the redesign workflow's feedback-only / research-only
patterns. GitHub's open-PR list has no native concept of
"draft-but-feedback-only" vs "draft-and-WIP." The closure-as-
absorbed pattern is the workaround, but it's a workaround. A v2
design considering issue-tracker integration could:
- Track absorbed-vs-draft state in a dedicated label or status
  field.
- OR build a tool that, given a draft-PR list, classifies each
  by dispatch-type and absorption status.

The current absent-tooling means housekeeping IS work-the-
orchestrator-does-by-judgment, which is exactly the kind of
procedural work the redesign aims to push into tools per
CORE-DESIGN-PRINCIPLE. Captured here for v2 consideration.

### `_notes/` directory at 33 files exceeds the README's degradation threshold

`docs/redesign/_notes/README.md` flags ~10 files as the threshold.
The directory now holds 33 files (cycles 2-34, with multiple files
per cycle in some cases). Cycle-33's follow-up #3 deferred this
migration explicitly. This is a real persistence-mechanism scale
problem — the kind of thing the prompt's
`<evolve-the-mechanism>` block authorizes addressing when the
mechanism starts failing.

Symptom check: have I had to grep across `_notes/` files this
cycle to find prior context? Yes — searching for "cycle-31"
content during housekeeping required reading `_notes/cycle-31-
copilot-feedback-evaluation.md` directly because the bare
filename wasn't enough to confirm content. But it wasn't painful
this cycle; the file naming convention worked. Watch for the
pattern: when grep-across-notes becomes painful in any single
cycle, that's the trigger to migrate.

## Persistence-mechanism observation

The cycle-33 Eva-driven structural change adds a new pattern to
the persistence mechanism's history: **substantial structural
migrations can come from outside the orchestrator flow as long as
they document themselves with cold-reader questions for the next
orchestrator cycle.** The migration succeeded without losing any
load-bearing properties, verified by cycle-34's 4/4 PASS
cold-reader. This is the second cross-orchestrator coordination
event in the redesign (the first being audit-repo cross-reads,
which are also orchestrator-to-orchestrator without
synchronization).

Pattern: cross-orchestrator coordination via notes files +
follow-up questions in named subsections is reliable as a
hand-off mechanism, in both directions (Eva → orchestrator;
orchestrator → audit-orchestrator).

## Cycle-35 plan suggestion (provisional)

Two non-mutually-exclusive options:

1. **Phase 2 candidate-generation preparation** (substantive,
   architecturally-load-bearing). The cycle-32 plan suggested
   this. The Phase 1 deliverable (now in `1-research.md` index +
   per-system files) is structurally stable; cross-system
   observations have been re-validated. Phase 2 candidate
   generation requires the post-retrospective checkpoint
   approval (which is yet to land), but preparation work
   (re-reading the restructured cross-system observations
   against audit's A-pattern mapping; identifying the design
   axes Phase 2 candidates must span) is unblocked.

2. **Continue housekeeping** (bounded mechanical). Pick up
   2-4 of the deferred PRs/issues from the cycle-26-and-earlier
   set. Lower-priority than option 1 but progressive cleanup
   maintains the discipline.

Lean toward option 1 since the cold-reader verification has
freed the Phase 1 deliverable from active iteration, and Phase 2
preparation is unblocked design work that the cycle budget
should be spent on. Option 2 can run as bounded mechanical in
parallel with option 1's substantive focal.

## Pre-commit checklist (for cycle 35's cold-reader)

Three bounded-mechanical questions for the next cycle's
cold-reader on this cycle's notes file:

- **(a)** Are the three closures' linking comments accurate? Do
  they name file paths and commit SHAs that resolve to the
  intended absorbed-content destinations? (Spot-check one
  closure comment by following its file/SHA references.)
- **(b)** Is the deferral rationale for the 11 considered-but-
  deferred items principled (cadence-respecting) or
  procrastinating (avoiding work)? The cadence rationale cites
  HOUSEKEEPING's "heavy substantive work" caveat — does that
  apply legitimately to cycle 34?
- **(c)** Does the design-input meta-loop section identify
  observations that are genuinely load-bearing for v2
  consideration, or is it filler? Spot-check one observation
  (the duplicate-issue UI hiccup) for substance.
