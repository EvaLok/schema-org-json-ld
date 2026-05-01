# Cycle 44 — cold-reader on v1.7 + cycle-43 work; v1.8 application

**Date:** 2026-05-01 (eighth cycle of the day)
**Cycle issue:** #2811
**Inherits from:** cycle 43 (`_notes/cycle-43-openclaw-per-finding-evaluation.md`)

## Cold-reader: 1 BORDERLINE-FAIL + 2 PASS

Three questions inherited from cycle 43's pre-commit checklist. Each
re-walked with adversarial framing.

### Q(a) BORDERLINE-FAIL — Cross-axis dep map phrasing precision

**Question:** Did v1.7's three Axis additions (Axis 2 openclaw +
global-state.ts caveat, Axis 3 layered architecture refinement, Axis 9
stuck-session watchdog) introduce internal inconsistencies or
cross-axis-propagation gaps?

**Sub-check 1: Axis 2 × Axis 3 dep map.** The dep map says "file-per-
component → memory-as-component-file; typed-channel-map →
memory-as-channel; repo-as-state → memory-as-files-in-repo."

The phrasing is **informal and doesn't reference any specific Axis 3
named position**. Walking through Axis 3's actual positions:

- **Singleton plugin slot** (openclaw) — the plugin slot's
  storage/retrieval layer can use file-per-component implementation
  (openclaw's pattern: `~/.openclaw/agents/<agentId>/` houses Markdown
  files + SQLite index used by the memory plugin).
- **Top-level architectural principle** (PAI) — supports filesystem-
  based memory at architecture level (PAI Principle 13).
- **Wiki + search** (oh-my-codex) — `.omx/wiki/` is file-per-entry
  with search index on top.
- **Context trace** — independent of state representation; in-session
  mechanism.
- **Repository-as-record** — depends on repo-as-state (Axis 2).
- **Typed channels** — pairs with typed-channel-map (Axis 2).
- **Memory derivative of state** — rejected.

The dep map's "memory-as-component-file" phrasing collapses three
distinct Axis 3 positions (singleton plugin slot WITH filesystem
implementation; top-level architectural principle with filesystem
memory; wiki+search with file-per-entry) into one informal label.
This is a **load-bearing imprecision** for Phase 2 candidate
evaluation: a candidate choosing file-per-component (Axis 2) needs
to know which Axis 3 positions are naturally available, and the
current phrasing obscures that there are three.

**Cycle-43 same-cycle Q1 already flagged this** as "minor refinement
opportunity for cycle-44+; not load-bearing for current Phase 2
input." Cycle-44 cold-reader **upgrades the verdict to load-bearing**
because:
- The "memory-as-component-file" phrasing is not a position name; it
  is an alignment description that doesn't map to Axis 3's named
  positions
- Phase 2 candidate generation needs concrete alignment guidance, not
  informal shorthand
- openclaw concretely demonstrates that file-per-component pairs with
  singleton-plugin-slot (not with a "memory-as-component-file" position
  that doesn't exist as named)

**v1.8 fix:** rewrite the Axis 2 × Axis 3 dep map entry to name the
specific Axis 3 positions that align with each Axis 2 position. The
file-per-component row should explicitly list the three aligned Axis 3
positions (singleton plugin slot with filesystem storage; top-level
architectural principle with filesystem memory; wiki+search with file-
per-entry).

**Sub-check 2: F8 mapping rationale.** F8 (abandonment cascades) maps
to "Axis 9, CORE-DESIGN-PRINCIPLE | Bounded loops + single-implementation
discipline (no parallel implementations)." The v1.7 Axis 9 row added
the **stuck-session watchdog** (`diagnostics.stuckSessionWarnMs`) as a
"more interesting primitive than the bare timeout." The F8 mapping
rationale doesn't reference this primitive.

The watchdog is a **detection-and-recovery** primitive, structurally
different from bounded loops (which are **prevention** primitives).
Both contribute to F8 mitigation. The mapping rationale could mention
both:
- Bounded loops prevent runaway loops (no upper bound → unbounded
  retry)
- Stuck-session watchdog detects stalled sessions and releases lanes
  (timeout-fired retry mechanism for unbounded-runtime positions)

**v1.8 fix (minor):** F8 mapping rationale could include "or
stuck-session watchdog for runtime-ceiling positions (openclaw's
`diagnostics.stuckSessionWarnMs` instance)" alongside "Bounded loops."

**Sub-check 3: Axis 3 layered-implementation consistency.** The v1.7
Axis 3 update clarified that openclaw's "singleton plugin slot" is the
storage/retrieval LAYER, with the full memory architecture layered on
top (Markdown + SQLite + active-memory sub-agent + dreaming).

Is "one mechanism active" still the right framing if the implementation
is layered? Yes — the framing is at the interface level. One memory
plugin is active in the slot; the plugin's internal implementation can
be layered. The four internal layers compose into ONE memory plugin's
implementation. **Sub-check PASSES — no inconsistency.**

**Q(a) summary:** BORDERLINE-FAIL (load-bearing dep map fix) + minor
refinement (F8 mapping rationale) + PASS (Axis 3 layered consistency).

### Q(b) PASS — Per-finding evaluation calibration

**Question:** Was the cycle-43 per-finding evaluation correctly
calibrated, or did I over-accept findings? Re-read OC9 (session
lifecycle as transferable freshness mechanism) and OC13 (delegate
architecture tier escalation).

**OC9 re-read:** Verdict is "ACCEPT WITH QUALIFICATION." The
qualification text:

> "The 'session' pattern is interesting for v2 design but doesn't
> transfer cleanly. openclaw sessions are human-conversation threads;
> the redesign orchestrator operates on a cron schedule with no
> per-session human dialog. The lifecycle primitives (daily reset,
> idle reset, fresh-per-cron) ARE potentially transferable as state
> freshness mechanisms — for example, 'reset state freshness daily'
> could be a useful primitive for v2 candidates that maintain state
> across cycles."

The qualification text is **appropriately hedged**: "doesn't transfer
cleanly," "potentially transferable," "could be a useful primitive."
The integration target is "no framework change yet; flag for Phase 2
candidate generation."

But cycle-44 cold-reader observation: **"fresh-per-cron" is what the
redesign already has implicitly** — each cron-fired cycle starts with
a fresh context, no in-memory state from prior cycle. The persistence
is in files, but the in-context-state is fresh per cycle. So the
"fresh-per-cron" lifecycle primitive is the SAME shape as what already
exists, not a new primitive to adopt.

The "daily reset" specifically — for a cron-driven orchestrator, this
would mean **daily reset of accumulated working notes** while
preserving authoritative artifacts. This is similar to `MEMORY.md`
rotation patterns. It IS a different primitive than "fresh-per-cron"
but is currently **underspecified** — what state resets, what persists.

**Verdict on OC9 qualification: PASS (appropriately hedged, integration
target appropriately deferred).** Minor wording-improvement opportunity:
acknowledge that "fresh-per-cron" already exists implicitly in
cron-driven systems, and the genuinely new primitive is "daily reset of
accumulated working notes" which requires more concrete specification.
Not load-bearing for current Phase 2 input.

**OC13 re-read:** Verdict is plain "ACCEPT" (cycle-43 pre-commit
checklist incorrectly labeled this as "ACCEPT WITH QUALIFICATION" — a
minor categorization error in the checklist, not in the actual
evaluation). Source claim: "Tier-1/tier-2/tier-3 capability escalation
(read-only → send-on-behalf → autonomous with standing orders) maps to
tool permission escalation in the redesign."

Cycle-44 observation: **the pattern is ALREADY present in the redesign
prompt's COPILOT-DISPATCHES section** as research-only / feedback-only /
implementation tiers. openclaw's tier-1/2/3 is explicitly about
delegating to operators OTHER than the human owner (delegation
pattern), but the structural shape (three tiers of escalating
permissions/capability) maps to the redesign's three Copilot dispatch
tiers (research-only / feedback-only / implementation).

So OC13's pattern transfers cleanly to existing infrastructure and
**validates** the existing tier discipline. Cycle-43's verdict ACCEPT
is appropriate; the integration target ("v2 candidate template:
consider permission-tier discipline; Phase 2 candidate generation may
surface as a sub-axis or fold into Constraint 3") is appropriately
forward-looking. **Verdict on OC13: PASS.**

**Q(b) summary:** PASS — both qualifications appropriately scoped, no
over-acceptance. Minor wording improvements possible but not load-
bearing.

### Q(c) PASS — Axis 12 cost-framing balance

**Question:** Did the cycle-43 Q(c) cost-framing balance go far enough,
or is there latent prejudice? Specific check: write a one-paragraph
rationale FOR each of the four Axis 12 positions and see if any is
harder to defend than the others.

**Position 1 — "No reconciliation":** A v2 candidate could argue that
explicit reconciliation primitives are over-engineering for the
redesign's actual workload. Most channels have low message rates (Eva
responds occasionally; audit posts every few cycles; PR merges by Eva).
For these, the orchestrator could BUILD reconciliation logic per-cycle
as needed. The simplicity is "the orchestrator reads what it needs each
cycle, no structural primitive." HOWEVER: F2 fundamentally requires
SOME inbound mechanism (Eva-response detection); F4 fundamentally
requires reconciliation for refresh-timing decisions. Without
structural reconciliation, the orchestrator must perform reconciliation
ad-hoc each cycle, which is what v1 does and exhibits F2/F4 directly.
**Defense fails on F2/F4.** Anti-pattern label is correct.

**Position 2 — "Active polling":** A v2 candidate could argue this gives
uniform mechanism (one pattern per channel) — easy to understand,
debug, and reason about. The per-channel implementation overhead is
justified by consistency benefit. Each channel reader has a clear
interface: "given current state, what changed in this channel?"
Predictable schedule (cron-fired). For low-frequency channels, polling
is wasteful but the cost is bounded. **Defense holds.**

**Position 3 — "Event-driven":** A v2 candidate could argue that
reactive handling has lowest LATENCY and shared infrastructure (one
webhook configuration handles all channels via event filters, vs N
readers for active polling). Costs: webhook/Actions setup is an
infrastructure decision, but for a public GitHub repo this is
already-paid infrastructure (GitHub Actions exist for the workflow).
For three channels (Eva, audit, dispatch outputs), event-driven could
be 1 webhook with 3 event filters vs 3 readers for active polling.
**Defense holds.**

**Position 4 — "Hybrid":** A v2 candidate could argue that different
channels have different natural frequencies. Eva responds occasionally
(low-frequency); audit posts on cron (low-frequency); dispatch outputs
arrive on PR-merge events (medium-frequency). Matching the choice to
channel frequency minimizes wasted work. Cost: design complexity for
deciding per-channel-class boundaries; partial uniformity (some
channels follow polling pattern, others event-driven). **Defense
holds.**

**Comparison observation:** Active polling, Event-driven, and Hybrid
are **all defensible**. No-reconciliation fails on F2/F4. The
cost-framing balance is **adequate for non-prejudice**; each defensible
position has a stated cost; the rejected position has F-pattern-
referenced rationale.

**Minor wording-symmetry observation:** Active polling annotation says
"Uniform mechanism (one pattern per channel); per-channel implementation
overhead." Event-driven annotation says "requires inbound trigger
infrastructure (webhook, GitHub Actions on event)." The framing is
asymmetric — "Uniform mechanism" sounds like a benefit; "requires X"
sounds like a cost-only framing. A more symmetric framing would
acknowledge event-driven's "shared infrastructure" benefit alongside
the setup-requirement cost.

**Q(c) summary:** PASS — cost-framing balance is adequate; minor
wording-symmetry opportunity for v1.8 (event-driven annotation could
add "shared infrastructure" benefit framing alongside "requires X" cost
framing).

## v1.8 application — three changes

Applied to `docs/redesign/2-design-framework.md`:

### Change 1 (Q(a) BORDERLINE-FAIL fix) — Axis 2 × Axis 3 dep map phrasing

**Old:** "State representation shapes natural memory primitive —
file-per-component → memory-as-component-file; typed-channel-map →
memory-as-channel; repo-as-state → memory-as-files-in-repo."

**New:** "State representation shapes which Axis 3 positions are
natural — file-per-component aligns with filesystem-based memory
(singleton plugin slot WITH filesystem storage as in openclaw's
`~/.openclaw/agents/<agentId>/`; top-level architectural principle with
filesystem memory as in PAI; wiki+search with file-per-entry as in
oh-my-codex's `.omx/wiki/`); typed-channel-map aligns with typed
channels with checkpointer (LangGraph); repo-as-state aligns with
repository-as-record (OpenAI harness)."

Naming the specific Axis 3 positions that align with each Axis 2
position resolves the load-bearing imprecision flagged by cycle-44
Q(a) cold-reader and cycle-43 same-cycle Q1.

### Change 2 (Q(a) minor refinement) — F8 mapping rationale

**Old:** "F8 (abandonment cascades) | Tooling fragility | Axis 9,
CORE-DESIGN-PRINCIPLE | Bounded loops + single-implementation discipline
(no parallel implementations)"

**New:** "F8 (abandonment cascades) | Tooling fragility | Axis 9,
CORE-DESIGN-PRINCIPLE | Bounded loops (prevention) or stuck-session
watchdog for runtime-ceiling positions (detection-and-recovery,
openclaw's `diagnostics.stuckSessionWarnMs` instance) + single-
implementation discipline (no parallel implementations)"

Adds the cycle-43 v1.7-introduced stuck-session watchdog as a F8-
relevant Axis 9 primitive alongside bounded loops. Both are valid F8
mitigations; mentioning both in the mapping rationale clarifies that
runtime-ceiling positions have a recovery mechanism.

### Change 3 (Q(c) minor wording-symmetry) — Axis 12 event-driven annotation

**Old:** "Event-driven: state changes reactively when external events
arrive | Reactive handling; requires inbound trigger infrastructure
(webhook, GitHub Actions on event); openclaw's Gateway is an instance
— channels maintain persistent upstream connections, agent runs are
per-event discrete turns"

**New:** "Event-driven: state changes reactively when external events
arrive | Reactive handling; shared inbound infrastructure (one webhook
or GitHub Actions trigger handles all subscribed channels; for a
public-repo orchestrator the infrastructure is already-paid); openclaw's
Gateway is an instance — channels maintain persistent upstream
connections, agent runs are per-event discrete turns"

Adds "shared inbound infrastructure" as a positive framing alongside
the setup cost. The "already-paid" parenthetical makes explicit that
for the redesign's specific context (public GitHub repo orchestrator),
the infrastructure cost is largely sunk.

### v1.8 status update

Iteration history table gets a v1.8 row covering all three changes.
Status header bumps v1.7 → v1.8.

## Bounded mechanical (1): paired-PR housekeeping

Three draft PRs paired with cycle-43-closed v1-era issues remain open:

- **PR #2730** (READY) — paired with closed issue #2729 (Replace raw
  `gh api` task dispatch template with `tools/dispatch-task`). Issue
  closed cycle 43 per "v1 prompt fix on frozen production prompt"
  pattern.
- **PR #2737** (DRAFT) — paired with closed issue #2736 (Add
  `write-entry journal --auto-blockers` for live Standing Eva blocker
  derivation). Issue closed cycle 43 per "v1 tool fix on frozen
  production tool" pattern.
- **PR #2739** (DRAFT) — paired with closed issue #2738 (Cycle 545
  end-of-cycle review). Issue closed cycle 43 per "v1 superseded by
  redesign mode" pattern.

All three closed with linking comments naming the cycle-43 issue
closure as the absorption event and the corresponding `_notes/`
files documenting the v1-era rationale being superseded by redesign
mode.

After cycle-44 sweep: 11 → 11 open issues (no issue closures) + 3 → 0
open PRs (3 closures).

## Bounded mechanical (2): cross-axis-impact-check tool design draft

Drafted at `tools/v2/_drafts/cross-axis-impact-check-design.md`. The
tool is a mechanical structural-consistency lint for the redesign
Phase 2 framework file. Motivation: the cross-axis-update-propagation
failure mode has fired three times in five cycles (42, 43 same-cycle
flag, 44 cross-cycle escalation); a mechanical lint tool would catch
propagation gaps faster and free orchestrator compute for substantive
Phase 2 candidate work.

The draft covers:
- Purpose, scope (in/out for v0; possible v1+ extensions)
- Inputs/outputs spec (CLI flags, exit codes, output format)
- 3-phase architecture sketch (parser, analyzer, reporter)
- Test surface (unit + integration; integration test against v1.4
  framework state should detect cycle-42's Cognition-update propagation
  gap)
- 4 open design questions (when does the tool run, freeform prose
  handling, persistence interaction, minimum viable cycle for v0)
- Cycle 45+ next steps (iterate design vs prototype parser; ordering
  vs `redispatch` tool)

This is a design draft, NOT an implementation. Cycle 45+ may iterate
the design before any building happens.

The choice of cross-axis-impact-check over redispatch:
- cross-axis-impact-check addresses HIGH-FREQUENCY orchestrator work
  (every framework iteration cycle)
- redispatch addresses LOWER-FREQUENCY work (every external dispatch,
  empirically every 1-3 cycles)
- Higher-frequency value rationale dominates; redispatch can defer
  to cycle 45+ with its own design draft

## Same-cycle review (5 questions)

Five questions on cycle-44 work + v1.8 framework:

### Q1 — v1.8 internal consistency

Did v1.8's Axis 2 × Axis 3 dep map rewrite introduce any internal
inconsistencies elsewhere?

Re-walked: the new phrasing names specific Axis 3 positions (singleton
plugin slot with filesystem storage; top-level architectural principle
with filesystem memory; wiki+search with file-per-entry; typed channels
with checkpointer; repository-as-record). Cross-referenced against
Axis 3's actual position rows: all five named positions are present in
the Axis 3 table. The mapping is internally consistent.

**Verdict: PASS.**

### Q2 — F8 mapping rationale composition

Does the F8 mapping rationale's new "Bounded loops (prevention) or
stuck-session watchdog for runtime-ceiling positions (detection-and-
recovery)" framing imply that bounded loops and stuck-session watchdog
are alternatives, when they could be complementary?

Re-read: the "or" suggests choice; the actual semantics are
"prevention OR detection-and-recovery, both valid for different Axis 9
positions." A v2 candidate could choose loop-count ceilings (Bounded
loops, prevention) OR runtime ceiling (stuck-session watchdog,
detection-and-recovery) OR both (Axis 9's "Both" position). The "or"
phrasing matches the Axis 9 position structure.

**Verdict: BORDERLINE-PASS** — wording is slightly ambiguous but
matches the underlying Axis 9 structure. Could be clarified in v1.9+
if Phase 2 candidates surface confusion.

### Q3 — Axis 12 event-driven "already-paid" framing accuracy

Is the "for a public-repo orchestrator the infrastructure is
already-paid" parenthetical accurate, or does it under-state the
configuration cost?

Re-check: GitHub Actions ARE already-paid (no per-event charge for
public repos under reasonable usage). The CONFIGURATION cost (writing
workflow YAML to react to events) is real but bounded — a single
workflow can dispatch handlers for multiple event types. The
"infrastructure" framing is correct (the Actions platform is paid);
the "configuration" cost (writing handlers) is one-time setup work,
not infrastructure cost.

The parenthetical could be more precise: "the Actions infrastructure
is already-paid; per-event handler configuration is a bounded one-time
cost." But this adds wordiness; the current framing is acceptable.

**Verdict: PASS** — accurate enough; configuration cost framing could
be sharper but not load-bearing.

### Q4 — Cycle-43 OC13 categorization correction

Should cycle-44 explicitly correct cycle-43's pre-commit checklist
mis-categorization of OC13 (labeled "ACCEPT WITH QUALIFICATION" in the
pre-commit checklist text but actually "ACCEPT" in the per-finding
evaluation file)?

The mis-categorization is in cycle-43's notes file (the pre-commit
checklist text), not in the framework. It's a narrative-record error,
not a framework error. Cycle-44's notes file documents the correction
explicitly via this Q(b) re-read; that's adequate. No framework or
notes-file edit needed beyond cycle-44's own documentation.

**Verdict: PASS** — adequately documented in cycle-44 notes via Q(b)
re-read.

### Q5 — Cycle 45 pre-commit checklist scope

Does cycle-44's cycle-45 pre-commit checklist appropriately scope
forward to next-cycle work?

Cycle 45 inherits:
- (a) v1.8 internal consistency check on the three new pieces (dep map
  rewrite, F8 rationale, Axis 12 event-driven annotation)
- (b) Did v1.8 over-correct in the dep map rewrite by being too
  prescriptive about "specific named positions" — could the rewrite
  unintentionally constrain Phase 2 candidate creativity?
- (c) Bounded-mechanical TBD: Rust tool design draft (cross-axis-
  impact-check OR redispatch) deferred from cycle-44; deeper sweep on
  remaining v1-era input-from-eva items (#2039, #699/#808/#809);
  potential `src/global-state.ts` Copilot research-only dispatch
  (cycle-43 deferred, low-priority)

Three questions covering different lenses (consistency, prescriptive-
ness, future-work scope). Each question is concrete enough to be
falsifiable.

**Verdict: PASS.**

## What surprised me

**The Axis 2 × Axis 3 dep map imprecision was load-bearing rather
than minor.** Cycle-43 same-cycle Q1 had flagged it as "minor
refinement opportunity for cycle-44+; not load-bearing for current
Phase 2 input." Cycle-44 cold-reader walked through Axis 3's actual
named positions and found the dep map's "memory-as-component-file"
phrasing doesn't reference any specific Axis 3 position. It's
informal shorthand that obscures THREE distinct Axis 3 positions
that align with file-per-component (singleton plugin slot with
filesystem storage; top-level architectural principle with filesystem
memory; wiki+search with file-per-entry). For Phase 2 candidate
generation, this distinction matters — a candidate choosing
file-per-component needs to know its Axis 3 alignment options.

The cross-cycle review pattern (cycle 44 reviewing cycle 43's
"minor" flag) caught what same-cycle review missed.

**OC13 was already addressed in v1's prompt structure.** Cycle-43
had categorized OC13 as a Phase 2 candidate consideration. Cycle-44
re-read recognized that the redesign prompt's COPILOT-DISPATCHES
section already implements the three-tier permission pattern
(research-only / feedback-only / implementation). So OC13
**validates existing prompt infrastructure** rather than naming a
gap. The cycle-43 verdict was correct (ACCEPT) but understated the
finding's load-bearing role — it's not just "consider for v2," it's
"existing v1 pattern is correct and should be preserved."

**The Axis 12 cost-framing wording-symmetry was real but minor.**
Going in, I expected Q(c) to find either no prejudice or substantial
prejudice. Found a real but minor wording opportunity: event-driven's
annotation framed "requires X" while active polling's framed
"per-channel implementation overhead" — both costs but with different
valence. The v1.8 fix adds "shared inbound infrastructure" as a
positive framing alongside the setup cost.

## What I couldn't figure out

Whether to upgrade OC13 from "v2 consideration" to "v1 pattern to
preserve" in the cycle-43 per-finding evaluation file or just document
the upgrade in cycle-44 notes. Lean toward documenting in cycle-44
notes only, because the cycle-43 evaluation IS the per-finding-
evaluation artifact and editing it now would obscure the iteration
history. Cross-cycle observations live in subsequent cycle notes
files. **Decision:** document in cycle-44 only; do not edit cycle-43
evaluation file.

Also: whether the cycle-43→cycle-44 "minor flag → load-bearing
finding" pattern indicates a calibration issue with same-cycle Q1
review (consistently under-reading severity?), or just normal
cross-cycle convergence (multi-cycle review surfaces what same-cycle
misses). Lean toward "normal cross-cycle convergence" because:
- The pattern fired correctly — same-cycle flagged the issue, cross-
  cycle escalated to fix
- The under-reading is informational, not blocking — same-cycle
  caught the issue; cross-cycle gave it appropriate weight
- Calibration issues would manifest as same-cycle MISSING issues
  entirely, not flagging-without-prioritization

If the pattern recurs (cycle-45 cross-cycle escalates a cycle-44
"minor" flag), might warrant deeper calibration review. For now:
process working as designed.

## Pre-commit checklist for cycle 45's cold-reader

Three questions:

- **(a)** v1.8 internal consistency check: re-walk the Axis 2 × Axis 3
  dep map rewrite. The new phrasing names "filesystem-based memory"
  positions but doesn't address what happens when a v2 candidate wants
  file-per-component (Axis 2) WITHOUT filesystem-based memory (e.g.,
  in-memory typed channels above filesystem state). Does v1.8 force a
  spurious alignment, or does the existing "natural support, not
  exclusive" framing in the surrounding text adequately cover this?
- **(b)** v1.8 prescriptiveness check: by naming specific Axis 3
  positions in the dep map, does v1.8 unintentionally constrain Phase
  2 candidate creativity? Stress test: imagine a v2 candidate that
  takes file-per-component (Axis 2) + context-trace memory (Axis 3) —
  does the v1.8 dep map rule this out as "unnatural" when it's
  actually defensible?
- **(c)** F8 mapping rationale "or" phrasing: did v1.8's "Bounded
  loops (prevention) or stuck-session watchdog (detection-and-
  recovery)" framing accidentally imply these are alternatives when
  they could be complementary? Same-cycle Q2 flagged this as
  BORDERLINE-PASS; cycle-45 cross-cycle review can verify or
  escalate.

## Cycle 45 plan (provisional)

1. **Substantive focal:** cross-cycle cold-reader on v1.8 + cycle-44
   work (3 Qs above). Verify v1.8 changes don't over-prescribe Phase
   2 candidate space.
2. **Substantive parallel:** TBD per cold-reader. Possibilities:
   - v1.9 corrections if cold-reader finds substantive issues
   - Iterate cross-axis-impact-check design (cycle-44 draft at
     `tools/v2/_drafts/cross-axis-impact-check-design.md` has 4 open
     design questions Q1-Q4 for cycle 45+ to address)
   - `redispatch` tool design draft (the second of the two tools
     listed in cycle 43 — cycle 44 chose cross-axis-impact-check based
     on higher-frequency value rationale; redispatch can be drafted in
     cycle 45 as the second-priority tool)
   - Continued housekeeping sweep on remaining v1-era input-from-eva
     items (#2039, #699/#808/#809) — conservative deferral remains
     correct unless cycle-45 finds genuine absorption signal
3. **Bounded mechanical:** if substantive load is light, prototype the
   cross-axis-impact-check parser in a single Rust file (no full crate
   yet) to validate the parser-design-questions before committing to
   the full architecture.

## What this cycle achieved

Cycle 44 is the **third "cold-reader → v1.X application" cycle** in
the redesign sequence (after cycles 38 and 42 which were v1.3 and v1.6
applications). The substantive output:

- 3 cold-reader questions answered (1 BORDERLINE-FAIL + 2 PASS)
- 1 load-bearing framework fix (Axis 2 × Axis 3 dep map phrasing
  precision)
- 2 minor framework fixes (F8 mapping rationale stuck-session
  watchdog mention; Axis 12 event-driven annotation wording-symmetry)
- v1.7 → v1.8 framework version bump
- 3 PR closures (#2730, #2737, #2739) — paired with cycle-43 issue
  closures
- 1 v2 tool design draft (cross-axis-impact-check) committed to
  `tools/v2/_drafts/`
- 1 cycle-44 same-cycle review (5 questions, 4 PASS + 1 BORDERLINE-PASS)

The most interesting cross-cycle observation: **same-cycle "minor
refinement opportunity" flags can escalate to load-bearing findings
on cross-cycle review.** This is the third instance of the pattern
(cycle 36 escalated cycle 35; cycle 37 escalated cycle 36; cycle 44
escalates cycle 43). The convergence pattern is asymptotic — each
cycle's correction is itself a claim that subsequent cross-cycle
review can refine.
