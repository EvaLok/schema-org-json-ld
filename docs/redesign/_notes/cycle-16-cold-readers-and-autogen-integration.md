# Cycle 16: cycle-15 cold-readers + AutoGen evaluation + Tier-1 integration

Cycle 15 (commit `e92794f8`) ran six pre-commits: three cold-readers on
cycle-14 Phase 0 edits (all PASS), an adversarial re-read of
`1-research.md` (found smuggling, applied fix), AutoGen Copilot
research-only dispatch (issue #2762, PR #2763), and Tier-2 group 5
(iteration plan move from retrospective body to README). Cycle 15 left
six pre-commits for cycle 16+: three cold-readers (Phase 0 + Phase 1),
AutoGen dispatch status check + integration if PR landed, Voyager paper
read, Tier-2 group 3.

Cycle 16 ran the three cold-readers and the AutoGen dispatch
evaluation + Tier-1 integration. Voyager and Tier-2 group 3 deferred to
cycle 17+ as cycle-15 anticipated.

The AutoGen PR landed at 17:14 UTC (~3h post-dispatch, much faster than
cycle-15's "probably cycle 16-17" estimate), making AutoGen integration
the cycle's primary work.

## Cold-reader checks on cycle-15 work

### Pre-commit 1: cold-reader on migrated iteration discipline section in README

Cycle 15 moved the iteration plan section from `0-retrospective.md`
body (lines 1253-1298) to `docs/redesign/README.md` (43-line iteration
discipline section above the existing iteration log). The
connect-across-patterns bullet was trimmed: the original verbatim quote
of cycle 7's freeze-vs-refresh framing (with pre-cycle-12 "guarantees"
wording) was dropped in favor of a brief reference: "(the
freeze-vs-refresh framing — see family preamble for current wording)".

The question: did this trimming lose substantive context for a fresh
reader? Specifically, the brief reference might under-signal what the
framing IS — a reader unfamiliar with the framing must navigate to the
family preamble to learn it.

**Verdict: PASS.** Reading the discipline section's
connect-across-patterns bullet alongside the family preamble at lines
167-194 of `0-retrospective.md`:

- The discipline section's role in the README is navigation/index for
  the iteration process, not the deliverable itself. Pointing to the
  deliverable for substantive content is appropriate.
- A fresh reader who follows the pointer reads the family preamble at
  lines 183-193: "F11 (the temporal stage of defense-accretion) is the
  point where reconciliation-asymmetry binds — defenses fire post-close;
  the worklog freeze is pre-close; nothing reconciles. F11 is also where
  the freeze-vs-refresh timing collision (F11's local mechanism) plays
  out: artifacts freeze before refreshers finish, structurally producing
  post-close divergence between frozen artifacts and live state." This
  IS the freeze-vs-refresh framing in current wording. The discipline
  section + family preamble together carry the full claim.
- Duplicating the framing in the discipline section would risk
  staleness: cycle 12 demoted "guarantees" to "structurally produces" in
  the family preamble; the iteration log's old "guarantees" quote was a
  stale historical artifact. The discipline section pointing to the
  family preamble for current wording avoids this drift.

**Optional cycle-17+ flag**: the reference "see family preamble for
current wording" doesn't say WHICH FILE the family preamble is in. The
section title "Iteration discipline for `0-retrospective.md`" sets the
context, so a careful reader infers the file. Could be tightened to
"see family preamble in `0-retrospective.md` for current wording" —
slight clarity improvement, not load-bearing.

### Pre-commit 2: cold-reader on retrospective new ending at line 1251

Cycle 15 left the retrospective ending at line 1251 with the
deferred-measures bullet ("Cycle 7 names them here so the gap is
visible to Eva-review and to candidate-design authors"). Question:
does this read as a deliberate structural close, or as a cut-off?

Read the final ~70 lines (1180-1251) as a fresh reader. The
section is "What v2 must demonstrably do better than v1" (line 1147).
The closing bullet is "Other v2 measure-shapes deferred to cycle 8+"
which lists more deferred items and ends with the cycle-7-attribution
sentence.

**Verdict: PASS.** The ending is deliberate, not cut-off:

- The bullet completes structurally — it's a complete bullet listing
  deferred items. Not mid-sentence, not mid-thought.
- A "deferred-measures" close is thematically appropriate for an
  iterative artifact: "what's not yet measured" is itself a form of
  closure. The retrospective is incomplete by design, per the README's
  iteration discipline section ("ready for the post-retrospective
  checkpoint only when Eva says it is").
- The README sets reader expectations: this is an iterating
  artifact, not a "wrap-up" deliverable.

The ending is slightly weak as a "deliverable close" (no overall
summary, no transition pointer to Phase 1 / Phase 2). But weakness ≠
cut-off. The structural completion is real.

**Optional cycle-17+ flag**: a single closing pointer paragraph could
help (e.g., "This retrospective continues iterating per the discipline
in `README.md`. Phase 1 cross-system reading proceeds in parallel
(`1-research.md`); Phase 2 candidate design awaits the
post-retrospective checkpoint."). Not load-bearing — but a reader
transitioning from this artifact to next-steps would benefit. Risk:
adding a "closing" sentence might inadvertently signal "done" when the
artifact is iterating. Defer with this caveat noted.

### Pre-commit 3: cold-reader on smuggling-fix in `1-research.md`

Cycle 15 found smuggling in the per-system "Provisional patterns to
track" sections (v2-relevance framings like "as a guardrail mechanism
in the deliverable itself"), renamed sections to "Patterns observed in
[system] (relevance evaluation deferred to cross-system synthesis,
gated on multi-system reading)" with v2-relevance framings stripped.

Cycle-15 caveat: some original framings carried descriptive content
about what the pattern DOES (e.g., "as a guardrail mechanism" describes
the function of the pattern), not just what it IS. Verify the
pure-observation framings retain enough descriptive content.

Read both per-system pattern lists in current state.

**openclaw patterns** (lines 126-132):
- "Small-core architecture with plugin-extensible capability" — pure
  observation. PASS.
- "Anti-pattern list ('What We Will Not Merge') as a deliverable
  artifact" — borderline. "as a deliverable artifact" is structural
  description (the list IS a thing in the artifact), not v2-relevance
  smuggling, but reads as slightly structural-evaluative.
- "Memory as a singleton plugin slot (one active mechanism, replaceable,
  not layered)" — pure observation with descriptive parenthetical. PASS.
- "Strong-defaults-with-operator-knobs security posture" — pattern name
  + qualifier. PASS.

**PAI patterns** (lines 206-215):
- "Principle-list published as part of the deliverable (16 named
  principles included in PAI's README)" — pure observation. PASS.
- "Decision hierarchy 'Goal → Code → CLI → Prompts → Agents' for where
  capability lives" — describes the pattern's function ("for where
  capability lives") without v2-relevance framing. PASS.
- "Closed feedback loop (Observe→Think→Plan→Execute→Verify→Learn→Improve)
  with explicit Learn → Improve closure" — describes the pattern. PASS.
- "Explicit 'I don't know' as a sanctioned response (PAI's Principle 16,
  'Permission to Fail')" — describes the pattern with citation. PASS.

**Verdict: PASS** with one borderline note.

7 of 8 pattern bullets are clean pure observations. One openclaw
bullet (item 2) reads slightly structural-evaluative ("as a deliverable
artifact"). The smuggling severity is minor compared to the pre-cycle-15
"as a guardrail mechanism in the deliverable itself" — the new framing
describes WHERE the pattern lives (in the deliverable), not what it
DOES for v2.

**Loss check**: did the trimming lose substantive content? The original
"as a guardrail mechanism" carried real descriptive content about the
pattern's function (preventing certain kinds of work from being merged).
The new "as a deliverable artifact" doesn't capture this functional
description. But: the function ("guardrail") is described elsewhere at
lines 89-95 ("Roadmap guardrails (VISION.md 'What We Will Not Merge')"
section header explicitly names the pattern's role). A reader who reads
the full document gets the function-claim from the descriptive section.
The trimming is appropriate — those framings were doing dual duty as
evaluation and description, and the description appears elsewhere.

**Optional cycle-17+ flag**: openclaw item 2 could be tightened to
"Anti-pattern list ('What We Will Not Merge') published in the project's
VISION document" — pure description of where the artifact lives, no
"as a [evaluative-category]" framing. Minor improvement.

## Pre-commit 4: AutoGen dispatch evaluation + Tier-1 integration

PR [#2763](https://github.com/EvaLok/schema-org-json-ld/pull/2763)
landed at 2026-04-28T17:14:57Z (~3h post-dispatch — much faster than
cycle-15's "probably cycle 16-17" estimate). Single new file
`docs/redesign/_notes/cycle-15-autogen-research.md`, 697 lines, 0
deletions. Branch protection blocks merge (same as PRs #2749, #2756);
PR remains open as evidence base.

### Per-section evaluation

The deliverable structure follows the seven dispatch lenses with an
additional "Patterns observed in AutoGen" section at the end (the
observation-shaped catalog cycle 15 explicitly requested).

**Lens 1 (architecture)**: Five named layers (Core / AgentChat /
Extensions / Studio / Bench) with quoted README citations for each
layer's responsibility. Concrete primitives named (AssistantAgent,
RoundRobinGroupChat, etc.). Migration history (v0.2 ConversableAgent /
GroupChat → v0.4 from-the-ground-up rewrite) covered with quoted
migration-guide citations.

**Lens 2 (state)**: Component-local state save/load described with
concrete TeamState dictionary structure example. Dictionary-based
serialization explicit. Reset/resume distinction named with quoted
teams.ipynb citation. Model-context abstraction (Unbounded /
Buffered / TokenLimited) covered.

**Lens 3 (orchestration)**: Multiple patterns first-class — Core
unopinionated, AgentChat opinionated. Nine documented orchestration
shapes including Magentic-One lead-orchestrator with Task Ledger /
Progress Ledger. The "patterns emerge from message protocols" framing
quoted from application-stack.md.

**Lens 4 (failure)**: Termination conditions + cancellation distinction
covered. The "what AutoGen does NOT centrally guarantee" list is
particularly useful (semantic correctness, deadlock diagnosis beyond
timeouts, global reconciliation, automatic retry, durable
crash-recovery without app-level persist).

**Lens 5 (tools)**: Schema-driven tool calling described. Trust
boundaries explicit (Docker vs local executor, MCP warnings,
Magentic-One prompt-injection caution). Agent-as-tool composition with
parallel-execution forbidden for stateful agent tools.

**Lens 6 (anti-patterns)**: Ten explicit non-goals enumerated, all
with quoted citations. Maintenance-mode + successor-framework
recommendation (notable factual observation). Sequential chat removed
in v0.4 as "too opinionated" — concrete de-prescription.

**Lens 7 (anchoring caveats)**: Twelve specific caveats. Each is
concretely grounded (not abstract "may not generalize"). The four
anchoring differences cycle-15 explicitly named in the dispatch body
(library vs orchestrator, application-prompt vs cron, human-in-the-loop
vs minimal, Python vs Rust) are addressed; additional caveats added
(developer-owned state persistence, maintenance mode, short vs
long-horizon tasks, etc.).

**Patterns observed section**: 38 bullets, all pure observations. No
v2-relevance framings. The cycle-15 anti-smuggling discipline pre-loaded
in the dispatch body was honored.

### Anti-smuggling discipline check (the critical cycle-15 carry-forward)

Cycle 15's adversarial re-read of `1-research.md` found that even with
explicit anchoring discipline documented, smuggling could occur in the
framings of pattern names. The cycle-15 dispatch body explicitly
pre-loaded this discipline (four named anchoring differences + request
to add specifics). Did the AutoGen deliverable honor it?

**Yes.** Sample bullets from the Patterns observed section:
- "Layered architecture: Core API for actor-style messaging/runtime,
  AgentChat for opinionated high-level agents/teams, Extensions for
  model clients/tools, Studio/Bench as developer tools." — describes
  what AutoGen has; no "as a v2-relevant pattern" smuggling.
- "Magentic-One docs explicitly warn about risky web actions, prompt
  injection, cookie agreements, and sensitive data exposure." —
  describes documentation; no v2-import framing.
- "Built-in sequential chat removed from AgentChat v0.4 as too
  opinionated; suggested alternatives are basic Python glue or Core
  workflows." — describes design history; no v2-evaluation.

All 38 bullets follow this discipline.

### Tier-1 integration

Single navigation summary added to `docs/redesign/1-research.md`
(parallel to openclaw and PAI sections), citing PR #2763 for the
deep-dive. Structure:

- Project status (maintenance mode + successor recommendation)
- Layered architecture (5 layers named)
- Multiple orchestration patterns first-class (9 documented)
- State as serialized component dictionaries
- Termination as composable callables
- What AutoGen does not centrally guarantee
- Trust boundaries
- Anti-patterns in v0.4 migration guide
- Anchoring caveats on AutoGen (7 specific)
- Patterns observed in AutoGen (16 bullets, pure observation —
  selected from the deep-dive's 38 by importance, not by
  v2-relevance)

The Patterns observed list in the navigation summary (16 bullets)
selects from the deep-dive's 38 bullets. Selection criteria: which
bullets carry the most architectural information, not which align
with v2 hopes. **Selection bias risk** — cycle-17 cold-reader queued.

Phase 1 work plan updated with `Status` column and chronological cycle
plan (cycles 14, 15, 16 each named). AutoGen marked "Cycle 15
dispatched (PR #2763); cycle 16 integrated above." Voyager moved to
cycle 17+ as the next priority.

Net `1-research.md` change: 313 → 474 lines (+161 net).

### Tier-2 deferrals

The following Tier-2 work is deferred to cycle 17+:

1. **Cross-system observations rewrite incorporating AutoGen.** The
   current section (lines 240-260 of `1-research.md`) lists shared
   patterns and differences for openclaw and PAI only. AutoGen's
   layered architecture, multiple-orchestration-patterns-first-class,
   actor-model framing, ledger vocabulary, etc. should be incorporated.
   With only 3 systems read, statistical thinness limits load-bearing
   claims; better to read more systems first. Deferred reasoning: the
   discipline says "multi-system reading should establish what's
   idiosyncratic vs cross-validates as generalizable" — 3 systems is
   the floor, not the target.
2. **Phase-2-input section drafting.** Per `1-research.md`'s discipline,
   "not to be drafted until at least 3-4 systems have been read."
   3 done; defer drafting until at least Voyager (cycle 17+) is read.

### Cycle 17+ pre-commits

1. **Cold-reader on the AutoGen navigation summary in `1-research.md`.**
   Does it undersell or misrepresent the deep-dive? The summary is ~140
   lines; the deep-dive is 697 lines. Compression ratio ~5×. Read both
   side-by-side and verify nothing load-bearing was lost or
   misrepresented in the compression.
2. **Cold-reader on the AutoGen "Patterns observed" 16-bullet list vs
   the deep-dive's 38-bullet flat list.** Did selection bias creep in
   during summarization? Specifically: which 22 of the deep-dive's 38
   bullets were dropped, and was that selection driven by v2-relevance
   anchoring (a smuggling failure) or by genuine architectural
   importance?
3. **Voyager paper read (orchestrator-direct, paper is short).** Cycle
   14's cycle plan #2 priority. Deferred from cycles 15 and 16. Cycle
   17 should pick up unless integration of AutoGen surfaces a higher
   priority. Voyager's absence from `1-research.md` is the largest
   single gap relative to the prompt's named-system list.
4. **Tier-2 group 3 (freeze-vs-refresh framing alternative — deepest
   cycle-11 finding still open).** If cycle 17 capacity permits beyond
   Voyager, group 3 is the strongest Tier-2 item. Carries forward from
   cycles 13, 14, 15, 16 (4 cycles deferred).
5. **Decision on next external-system dispatch.** Per `1-research.md`
   Cycle 17+ priority list: LangGraph (state-management focus),
   Cognition Devin writeups, or Semantic Kernel. LangGraph is the
   strongest pick (state-management is a v2 design concern that
   AutoGen handled lightly).

## Long-deferred items roll-call (carried forward, no action this cycle)

1. Journal-entry self-congratulation sweep (now 10 cycles deferred from
   cycle 7+)
2. F6/F8/F9 measurements (cycle 7+)
3. Refactor-for-length F-section sweep (cycle 8+; Tier-2 group 8)
4. Persistence-mechanism deferred-list consolidation (cycle 13 flag)
5. Tier-2 group 2 (review/disposition substrate, partial integration in
   cycle 13)
6. Tier-2 group 3 (freeze-vs-refresh framing alternative — deepest
   cycle-11 finding still open) — moved to cycle 17+ pre-commit list
   as item 4
7. Tier-2 group 4 (nine measures rework)
8. Tier-2 group 6 (preserved-through-cutover disposition)
9. Tier-2 group 7 (resolved open questions collapse)
10. Tier-2 group 9 (F8 singleton-family acknowledgment — small edit,
    cycle 13 noted promotion candidate)

Cycle 16 added 0 items to long-deferred list (the AutoGen integration
generated cycle-17+ pre-commits, not long-defers).

## What surprised me

**(1) The dispatch produced an unusually high-quality deliverable
respecting the anti-smuggling discipline.** Cycle-15's pre-loading
discipline in the dispatch body (four named anchoring differences +
explicit request to add specifics) clearly worked. The deliverable's
"Patterns observed in AutoGen" section is 38 bullets of pure
observation — none with v2-relevance framings, none with "as a
[redesign-relevant thing]" smuggling. The cycle-15 lesson about even
explicit discipline failing if it doesn't propagate to framings has now
a positive proof-point: when the discipline is pre-loaded into the
dispatch body, it propagates.

**(2) Branch protection makes the merge-vs-leave-open decision
automatic.** I started planning to merge PR #2763 to bring the research
file into master, then realized branch protection blocks the merge.
This matches the cycle 6/11 convention: dispatch PRs stay open as
evidence base, content gets summarized into the deliverable. Cycle 15's
notes had said "different evaluation discipline applies" without naming
the merge-or-not specifically; the convention emerges from
infrastructure, not just discipline.

**(3) The deliverable size (697 lines for one system) is much larger
than openclaw (60 lines) and PAI (130 lines).** This is partly because
Copilot deep-dive vs orchestrator-direct first-pass is a different
depth. The asymmetry makes cross-system observations harder — the
AutoGen evidence base is much richer than openclaw/PAI's, which biases
any cross-system pattern-validation toward AutoGen-supported patterns.
Cycle 17+ should consider whether to deepen openclaw and PAI to closer
parity (orchestrator-direct deeper reads) before cross-system synthesis.

## What I'm still uncertain about

Whether the AutoGen navigation summary in `1-research.md` (16-bullet
Patterns observed list) is the right size. The deep-dive has 38 bullets;
the summary has 16. Selection criteria: "carry the most architectural
information." But what if the bullets I dropped contained important
information? Cycle-17 cold-reader (pre-commit 2) will examine.

Whether the openclaw item 2 borderline ("as a deliverable artifact") is
worth the explicit cleanup or whether deferring it is the right call.
The smuggling severity is minor; the cleanup is bounded; cycle 17 has
capacity. Could be done in cycle 17 if it doesn't displace higher-value
work.

Whether the "What AutoGen does not centrally guarantee" framing in the
navigation summary section is observation-shaped or implicitly
evaluative. The framing names what AutoGen LACKS as a structural
observation about AutoGen, but a careful reader could interpret "what
AutoGen does not centrally guarantee" as a v2-design-implication
("things v2 should consider providing"). Re-read at cold-reader 1
will check this. Defensible reading: it's a structural observation
about AutoGen's documented design choices, parallel to "AutoGen does
not assert one canonical orchestration shape" — a description of
AutoGen's framework posture, not a v2 design signal.

Whether the Cycle 17+ priority of "Voyager next" is right given AutoGen's
state-management treatment was light, suggesting LangGraph (state-management
focus) might be more directly relevant. Cycle-15 ranked Voyager #2 for
direct relevance to v2's self-improvement claim. Both have merit;
defer the choice to cycle 17.

## Persistence-mechanism observations

The cycle-N-pre-commits-cycle-N+1-checks chain is now eleven cycles
deep (cycle 7 → 8 → 9 → 10 → 11 → 12 → 13 → 14 → 15 → 16 → 17
pre-committed). No breakdown. The chain has now produced value at
cycle-N-1 (immediate adjacency, the dominant case), cycle-N-2 (cycle 14
caught a cycle-11 issue), and cycle-N-3+ scopes. Multi-cycle scope is a
real benefit.

This cycle exhibited a different persistence-mechanism observation: the
**dispatch-body-pre-loading-of-discipline** pattern. Cycle 15 learned
the smuggling failure mode mid-cycle, then pre-loaded the discipline
into the cycle-16-evaluated dispatch body. The dispatch's deliverable
honored the discipline. This is a single-cycle propagation of a
discipline-discovery into a discipline-adherence — faster than the
typical multi-cycle chain.

Speculation: this works because the dispatch is a fresh AI session
reading the dispatch body cold; embedding discipline in the dispatch
body is structurally similar to embedding it in the system prompt for a
new session. The orchestrator's own multi-cycle chain works on
re-reading at cycle boundaries; the dispatch-body chain works on
fresh-session-prompt-reading. Both are forms of cold-start propagation.
**v2 design implication candidate**: this might generalize to
"discipline embedded in dispatch templates" as a propagation mechanism
distinct from "discipline embedded in orchestrator prompt" or
"discipline embedded in tools." Defer to Phase 2 rather than committing
now.

The Phase-0 / Phase-1 split is now stable across three cycles (14, 15,
16). The combined cycle scope continues to work — six pre-commits +
dispatch + Tier-1 integration in cycle 16 fits within the ~75-min
compute envelope. The cycle-7 over-extension lesson holds — Tier-1
applied this cycle, Tier-2 deferred to cycle 17+.

## Reconciling cycle scopes

Cycle 14: 3 cold-readers + Phase 1 initiation (`1-research.md` draft +
2 system reads).

Cycle 15: 3 cold-readers + 1 adversarial re-read with edit + 1 dispatch
+ 1 Tier-2 group execution.

Cycle 16: 3 cold-readers + AutoGen dispatch evaluation + Tier-1
AutoGen integration (~140 lines added to `1-research.md`, README
iteration log entry, this notes file).

All three cycles fit the ~75-min envelope. Cycle 16 is moderately
loaded but bounded — the Tier-2 deferrals (cross-system observations
rewrite, Phase-2-input drafting) prevent over-extension. Pattern
holding: Phase 0 (cold-readers + small edits) + Phase 1 (dispatch
evaluation + integration or new system reading) is the cycle shape that
fits the compute envelope sustainably.
