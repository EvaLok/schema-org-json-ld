# Cycle 66 — Cognition Devin implications for the v2 redesign

**Date:** 2026-05-04
**Substantive focal activity:** option 4 from input-from-eva [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829) (implications mining on a system already read at depth) — third instance of the implications-mining cadence, parallel to cycle-62 (AutoGen) and cycle-64 (LangGraph).
**Pivot from cycle-65's provisional read:** cycle-65 named per-finding-evaluation as cycle-66's highest-priority candidate contingent on cycle-63 dispatch [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) returning. Confirmed at session-start: dispatch is OPEN, 0 comments. Implications-mining option 4 continuation is the cycle-66 substantive focal. Cycle-65 also flagged cluster D conversion as a specific hypothesis to test — Cognition Devin foregrounds documentation honesty via its June 2025 → April 2026 published walkback, so Cognition is the right pick for testing that hypothesis (rather than openclaw, where the anti-pattern catalog is the singular-voice surface but the documentation-honesty signal is less central).

## What this document is, and is not

This is a focused, Cognition-Devin-specific implications writeup — what
the cycle-41 deeper-read evidence (PR
[#2804](https://github.com/EvaLok/schema-org-json-ld/pull/2804))
suggests for the v2 redesign that has NOT been written down in
`1-research.md` cross-system observations or in `2-design-framework.md`
axes. The cross-system synthesis cites Cognition alongside other
systems under shared patterns; this document inverts the lens — it
asks what Cognition tells us *as a singular voice* (or a
pair/triple-with-AutoGen-and-LangGraph voice where the pattern is
foregrounded by multiple).

It is NOT a Phase 2 candidate. It is implications-as-input. Phase 2
candidates still gate on the post-retrospective checkpoint and Eva
approval.

It is NOT a re-summary of `systems/cognition-devin.md` — that file is
the navigation summary; the deep-dive evidence is the cycle-38 deeper
read (`_notes/cycle-38-cognition-devin-deeper-read.md`, 795 lines)
plus the cycle-41 per-finding evaluation. This file identifies what
those patterns *imply* for our redesign that the framework has not
absorbed.

It pairs with `_notes/cycle-62-autogen-implications.md`,
`_notes/cycle-64-langgraph-implications.md`, and the cross-system
synthesis at `_notes/cycle-65-cross-implications-synthesis.md`. Where
the three implications documents converge, that's a stronger
cross-system data point (three-system pattern with deep-dive evidence
on three sides). Cycle-65's six-cluster decomposition over 16
implications becomes a six-cluster decomposition over 24 implications
once cycle-66's contributions land. Cluster reshape consequences are
in the "Cross-system convergence" section below; the implications
themselves come first.

## Anchoring frame inherited from systems/cognition-devin.md

Per the per-system file's anchoring caveats list, Cognition-to-v2-
redesign transfer is discounted by:

- **Hosted commercial product vs public-repo autonomous orchestrator.**
  microVM infrastructure, MCP marketplace, per-session identity chaining
  do not transfer directly. Patterns that transfer: context engineering,
  context rot awareness, write-single-threaded invariant,
  clean-context-for-reviewer, VM isolation concept (without VMs).
- **User-issued tasks vs autonomous cron.** Devin's task boundary is
  explicit (user starts session); ours is cron-state-inferred. Human
  takeover as primary recovery primitive does not transfer.
- **Closed source.** All Devin internals are documented-claim; no
  primary-source code access. Confidence calibrated accordingly.
- **Author bias.** Walden Yan's posts argue for the architectural
  choices Cognition made. Walkback credibility is real but does not
  extend to product-launch posts (Managed Devins announcement) or
  enterprise-case metrics.
- **Devin-builds-Devin context.** 659 Devin-authored PRs in a recent
  week internally. Throughput-of-engineering scale conditions which
  patterns assume scale and which transfer to single-team work.

Implications below carry positive transferability arguments where the
discount-list is silent. Where a discount applies, it's named inline.
Per the cycle-18 anchoring-caveats-symmetric discipline, discounts and
transfers are both made explicit.

## Implications

### I-C1. Public position evolution / walkback as first-class artifact

**Cognition-specific evidence.** Walden Yan's April 2026 post
"Multi-Agents: What's Actually Working" opens explicitly:

> "10 months ago, I wrote Don't Build Multi-Agents, arguing that most
> people shouldn't try to build multi-agent systems. [...] A lot has
> changed since then."

The walkback is not retroactive editing of the June 2025 post (it
remains published). It is not a quiet pivot. It is an explicit named
acknowledgement that prior framing was overstated, with new evidence
foregrounded. The June 2025 → April 2026 trajectory is itself a
documented sequence: prior thesis remains visible alongside the
revision, the revision names what survived (write-single-threaded
invariant) and what was overstated (the derivation to single-agent
linear execution).

**Implication for v2.** v2 will carry forward design choices made
during Phase 0 retrospective and Phase 2 candidate selection that
later cycles may discover were wrong. v1 has no mechanism for
acknowledging "the system used to assume X; we now think Y; here's
why" — corrections happen in commit messages, journal entries, or
implicit through prompt edits. None of these are first-class
artifacts that future readers (Eva at checkpoints, Phase 2 candidate
authors, post-cutover orchestrator instances) can navigate as
position-evolution.

**v2 design candidate input.** Add a `POSITIONS.md` (or similar) to
the v2 artifact tree that lists named system theses and their
evolution: original thesis, current revision, what changed and when.
Distinct from `MIGRATIONS.md` (cycle-62 I-1, which targets API/feature
removal between versions) — `POSITIONS.md` targets thesis evolution
within a version. Cognition's pattern: keep prior thesis visible
(don't retroactively edit), name the walkback explicitly, separate
the durable invariant from the overstated derivation.

**Discount.** Cognition's walkback is between two blog posts on a
public-facing engineering blog. Ours would be between revisions of
the orchestrator's own self-documentation. The substrate-difference
is real but the *discipline* (acknowledge revision explicitly,
preserve the prior framing as historical context, name what survived
versus what was overstated) is what transfers.

**Cross-reference.** Strongly converges with AutoGen I-4 (publishing
non-guarantees) and AutoGen I-7 (aspirational vs implemented as
explicit documented distinction). Together with I-C2 below, these
form the documentation-honesty cluster D — which cycle-65 hypothesized
would convert from partial-convergent to strong-convergent if a third
system foregrounded documentation honesty. Cognition foregrounds it
explicitly. **Cluster D conversion confirmed.** See "Cross-system
convergence" section.

---

### I-C2. Durable invariants vs derived prescriptions — separate published layers

**Cognition-specific evidence.** The June 2025 → April 2026 walkback
explicitly factors the original position into two layers:

- **Durable invariant** (April 2026 calls "what survived"):
  *writes stay single-threaded.* This was load-bearing in June 2025
  and remains load-bearing in April 2026. The April 2026 productions
  (Devin Review, Managed Devins) all preserve it.
- **Overstated derivation** (April 2026 calls "where I overshot"):
  *therefore, single-threaded linear execution.* This was a derivation
  from the invariant, not an axiom itself. The April 2026 patterns
  (clean-context reviewer, parallel children with single-threaded
  coordinator) violate the derivation while preserving the invariant.

The structural separation matters: when the derivation is revised, the
invariant doesn't have to be. The two layers are now visible to
readers in a way they were not in June 2025 — which is what makes the
April 2026 walkback intelligible.

**Implication for v2.** v1's prompt mixes durable design-axioms
(e.g., "self-modification of THIS PROMPT FILE requires a workflow-
change PR" — load-bearing security invariant) with derived
prescriptions (e.g., the cycle structure with startup checklist and
completion checklist — one current application of the invariant
"orchestrator should not bypass review"). When the redesign revises
derivations (small prompt, tools handle routine), the invariants do
not need to change — but readers cannot tell which is which without
deep-reading. This is precisely the failure mode Cognition's June
2025 derivation suffered from.

**v2 design candidate input.** Stratify the v2 prompt into INVARIANTS
and DERIVATIONS sections (or an analogous separation). INVARIANTS are
load-bearing axioms whose revision requires re-justifying the entire
design. DERIVATIONS are current applications of those axioms whose
revision is expected and bounded. When v3 deprecates parts of v2 (per
cycle-62 I-1), the INVARIANTS section is mostly preserved; the
DERIVATIONS section is where most edits land. Combined with I-C1's
`POSITIONS.md`, the system can revise its applied design without
destabilizing its load-bearing axioms.

**Discount.** Cognition's two-layer separation emerged retrospectively
through the June 2025 → April 2026 walkback, not as an upfront design
discipline. Retrofitting the discipline (have the layers clear from
day one) is a stronger claim than Cognition has demonstrated — the
discipline itself may be hard to apply prospectively. Phase 2 should
weigh this honestly: it's possible v2 cannot cleanly separate
invariants from derivations until v3's walkback forces the question.

**Cross-reference.** Tightly intersects with I-C1 (walkback
mechanism). Also intersects with AutoGen I-4 (non-guarantees) — both
push toward making the *layers* of system claims explicit. Strong
augmentation of cluster D.

---

### I-C3. Multi-layer memory architecture — distinct mechanism per (scope, lifetime) pair

**Cognition-specific evidence.** Cognition documents 5+ memory
mechanism types in production, each addressing a distinct
(scope, lifetime) pair:

| Mechanism | Scope | Lifetime | Format |
|---|---|---|---|
| Session context | Per-session | In-session | Rolling window, fine-tuned compressor |
| VM snapshot | Per-session | Across-async-gap | Hypervisor-level full state |
| Cross-session notes | Per-Devin-instance | Across recurring runs | Devin's own notes file |
| Knowledge API | Org-level | Persistent | Structured CRUD entries |
| Playbooks | Task-class | Persistent | Outcome + steps + postconditions |
| DeepWiki | Codebase | Per-repo | Auto-indexed wiki |
| Session Insights | Cross-session | Post-session | Suggested prompt deltas |

The architecture is *not* one primitive (e.g., "shared memory") with
multiple use cases — it is multiple distinct mechanisms each chosen
for a specific (scope, lifetime) coordinate. Context rot (citing
Chroma research) is named explicitly as a structural degradation mode
at long context lengths — primary motivation for separating short
context from long context with different mechanisms.

**Implication for v2.** v1 has the journal as cross-cycle persistence
substrate, plus the memory-directory bootstrap that cycle-62
discovered to be ephemeral within-session. The cross-cycle persistence
shape is monotonic-append-only-prose at one (scope=cycle,
lifetime=permanent) coordinate. AutoGen I-6 (plan/progress split) and
LangGraph I-L5 (short/long memory split) cover 2 axes of stratification
each; Cognition's 7-mechanism architecture is the upper-bound
stratification. The redesign's persistence problem
(`<persistence>` section in this prompt) is more solvable when the
solution is "multiple distinct mechanisms per coordinate" rather than
"one mechanism that handles all coordinates."

**v2 design candidate input.** Stratify the v2 cross-cycle persistence
into named mechanisms each addressing a specific (scope, lifetime)
coordinate. Concrete proposal grid:

| Coordinate | v2 mechanism candidate |
|---|---|
| Per-cycle, in-session | journal entry being drafted (a single file under construction during the session) |
| Per-cycle, post-session | committed journal entry |
| Per-cycle-arc (e.g., last 5 cycles), summary | rolling summary file regenerated each cycle (per-cycle-arc compressor) |
| Cross-cycle, design-decision | ADRs under `docs/redesign/adr/` — append-only, indexed |
| Cross-cycle, methodological | `_notes/methodology/` — extracted methodology observations |
| Per-task-class, prompt template | task-class playbook (cycle-66 I-C6 below) |
| Codebase, retrieval | `docs/redesign/index.md` cross-reference — read at session start |
| Post-session, feedback for next session | "for next session" capsule (cycle-66 I-C7 below) |

Different from `2-design-framework.md` Axis 4 (state representation),
which asks *what shape* the state takes; this implication asks *how
many distinct mechanisms are warranted*.

**Discount.** Cognition's mechanisms exist at hosted-product scale
with paying users; ours run on a public repo with one (or few)
operators. Some mechanisms (DeepWiki auto-indexing of a 365k-star
repo) do not transfer at our scale. The *discipline* (don't collapse
multiple coordinates into one mechanism if collapsing destroys
information) does transfer.

**Cross-reference.** Strongly augments cluster B (cross-cycle artifact
organization). Cluster B was 3 implications (1 strong-convergent
cycle-62 I-6 ↔ I-L5 + 1 AutoGen-singular I-2); I-C3 makes it 3-system
strong-convergent at 4 implications. Tightly intersects with
I-C6 (Playbooks) and I-C7 (Session Insights) — three of the seven
mechanisms in the table above are themselves implications.

---

### I-C4. Role-asymmetric context semantics — clean-context reviewer pattern

**Cognition-specific evidence.** The June 2025 principle "share
context, share full agent traces, not just individual messages"
applies to the *primary* execution role. The April 2026 finding for
the *reviewer* role inverts it:

> "The reviewer works *better* with NO shared context with the coder."

Devin Review is structurally a separate Devin instance with its own
fresh context, reading the PR diff but not the coder's session trace.
This avoids context rot (the reviewer reasons fresh about correctness
from the artifact, not from the trajectory that produced the
artifact). Reported: 2 bugs/PR average, 58% severe — direct evidence
that the role-asymmetric default outperforms uniform context-sharing.

**Implication for v2.** v1 treats all dispatched activity (Copilot
research, Copilot feedback, audit-repo critique) as having the same
context-availability semantics: the dispatch sees the artifact it's
asked about, plus whatever context the dispatch instructions provide.
But the *role* of the dispatch should determine context defaults:

- **Implementation/research dispatches** benefit from full context
  (what we know, what we tried, what failed) — like Cognition's
  primary role.
- **Critique/review dispatches** benefit from clean context (the
  artifact alone, with no prior framing) — like Cognition's reviewer
  role. Audit-repo cross-repo-reading already operates this way (it
  reads our repo without shared session memory) — Cognition's pattern
  validates that this is a *feature* of audit-repo, not a limitation.

**v2 design candidate input.** Articulate per-dispatch-type context
semantics. Concretely: when dispatching a Copilot feedback session
on (say) a retrospective draft, the dispatch instructions should say
"read this artifact fresh — do NOT read the prior feedback dispatches
or my journal explaining why I think it's good." The clean-context
default for critique roles should be load-bearing, not incidental.
When dispatching research, the opposite — provide as much context as
fits.

**Discount.** Cognition's reviewer is a Devin instance, with its own
shell + IDE + browser. Our reviewers are Copilot dispatches and
audit-repo orchestrator runs. The substrate-difference doesn't
obstruct the discipline (clean-context default for critique role).
What does require attention: in our setup the dispatch *instructions*
control context — careless instructions that summarize prior framing
defeat the clean-context property. v2 should make the discipline
mechanically supported (e.g., a `dispatch-feedback` tool that
constructs clean-context dispatch boilerplate by default).

**Cross-reference.** Singular-voice as of cycle 66 — neither AutoGen
nor LangGraph foregrounds role-asymmetric context as a load-bearing
discipline. AutoGen's roles are user-defined; LangGraph's role-vs-
state is about super-step phasing not about context isolation. New
cluster G ("role-asymmetric context semantics") with one implication
as singleton; openclaw's delegate architecture may or may not
foreground this — to be checked when openclaw's implications are
mined.

---

### I-C5. Reactive failure recovery via external triggers (event-driven wakeup)

**Cognition-specific evidence.** Devin's failure recovery is reactive,
not pre-planned-retry: write → CI/lint/review fires → bot comment
arrives → Devin picks up the comment → fix → CI runs → iterate.
External bot comments are the *trigger* for the next step; Devin does
not poll, retry on a schedule, or apply pre-planned exception
handlers. Combined with hypervisor-snapshot infrastructure, the
runtime cost during the wait is zero (compute idle until event
arrives). Documented April 2026 in "Closing the Agent Loop: Devin
Autofixes Review Comments."

**Implication for v2.** v1's failure recovery is mostly cycle-bounded
and pre-planned (the orchestrator catches errors within a cycle and
journals them; cross-cycle recovery happens because the next cron
fire reads the prior journal and decides what to do). External
events (PR review comments, audit-repo posts, dispatched-Copilot PR
descriptions, Eva's input-from-eva issues) are read at session start
but not as wakeup triggers — the cron schedule is the wakeup, not the
event.

**v2 design candidate input.** v2 could augment the cron-only wakeup
with event-driven wakeup for a small set of high-priority events:

- New audit-repo post on a load-bearing topic — wake immediately
  rather than wait for the next scheduled cycle
- Eva's input-from-eva issue — wake immediately
- Dispatched Copilot returned a deliverable — wake immediately to
  begin per-finding evaluation

The mechanism could be a webhook-driven workflow trigger paired with
the existing cron, with the prompt declaring which events qualify for
immediate wakeup. Concretely: a `dispatch-state.toml` or analogous
ledger tracking active dispatches; a webhook on dispatch-PR-merge
fires a workflow that creates an orchestrator-run issue tagged
"event-triggered: dispatch-returned" so the next session sees the
event in its session-start.

**Discount.** Cognition's external triggers are CI bots and human
review on a hosted product. Ours are GitHub Actions webhooks (already
available). The substrate is similar; the friction-difference is
"how many event sources qualify for immediate wakeup" — too many and
the orchestrator runs constantly; too few and the cron-only default
absorbs valuable events too slowly. Phase 2 should pick a small,
high-signal set initially.

**Cross-reference.** Augments cluster C (lifecycle operations) — adds
an "event-triggered wakeup" lifecycle operation peer to AutoGen I-8
(Reset/Resume) and LangGraph I-L4 (time travel as fork). Partial
convergence with AutoGen I-3 (termination predicates as
graceful-vs-immediate distinction) — both about explicit lifecycle
state transitions rather than implicit-by-time. Cluster C grows from
4 implications to 5.

---

### I-C6. Task-class prompt templates (Playbooks) — system-prompt-shape, not memory

**Cognition-specific evidence.** Devin Playbooks are task-class
templates with structured fields:

- **Outcome** — what success looks like
- **Steps** — typical sequence
- **Postconditions** — what must hold at completion
- **Advice** — heuristics
- **Forbidden actions** — what not to do

A Playbook is invoked when starting a task in a class (e.g., "fix a
failing CI build" Playbook). The Playbook becomes part of the
session's effective system prompt for that task class. Different from
session memory (which is per-session) and Knowledge API (which is
org-scoped CRUD entries) — Playbooks operate at the *task-class*
level.

**Implication for v2.** Cycle-65 surfaced "explicit cycle-composition-
tag declaration at session start" as a v2 design-input candidate (five
distinct cycle composition shapes already observed: mining,
dispatch-construction, synthesis, per-finding-evaluation, framework-
iteration). Playbooks are the natural mechanism: each cycle composition
shape has a Playbook with that shape's outcome, typical steps,
postconditions, advice, and forbidden actions. A "synthesis cycle"
Playbook would have different forbidden actions ("do NOT mine new
implications during synthesis") than a "mining cycle" Playbook ("do
NOT elevate to cross-system observations during mining").

**v2 design candidate input.** Per-cycle-composition Playbooks as a
v2 prompt-suite stratification primitive. Concretely:

- `prompts/v2/playbooks/mining.md` — implications mining shape
- `prompts/v2/playbooks/synthesis.md` — synthesis cycle shape
- `prompts/v2/playbooks/dispatch-construction.md` — dispatch
  construction shape
- `prompts/v2/playbooks/per-finding-evaluation.md` — absorption shape
- `prompts/v2/playbooks/framework-iteration.md` — cold-reader fallback
  shape (probably the smallest Playbook; this is the bounded-mechanical
  default)
- `prompts/v2/playbooks/housekeeping.md` — bounded mechanical
  housekeeping shape

The orchestrator's main prompt declares cycle-composition tags as
top-level vocabulary; the Playbook for the chosen tag is invoked at
session start. Cycle-composition tag for cycle 66 would be `mining`;
for cycle 65 it was `synthesis`; for cycle 63 it was
`dispatch-construction`.

**Discount.** Cognition's Playbooks operate within a hosted product
that lets users author Playbooks for their own task classes. Ours are
internal artifacts authored by the orchestrator (or by Eva via PR).
The substrate-difference doesn't obstruct the discipline; the
authorship model is different — Cognition's Playbooks are user-
authored at scale, ours are author-by-design.

**Cross-reference.** Strongly augments cluster F (tool-suite and
prompt-suite stratification). Cluster F was singleton AutoGen I-2;
I-C6 makes it 2-implication 2-system convergent. The pattern
generalizes I-2's opinion-gradient layering from tools to prompts
— both AutoGen and Cognition independently arrive at "stratify your
artifacts by structural role" as a discipline. Also intersects with
cluster B (cross-cycle artifact organization) since Playbooks ARE
one of the cluster-B mechanisms in I-C3's seven-mechanism table.

---

### I-C7. Self-improvement loop — Session Insights as post-session feedback for next session

**Cognition-specific evidence.** Session Insights is a post-session
analysis mechanism: after a Devin session completes, an analyzer
reads the session log and produces "improved prompt suggestions"
that the user can review and apply for the next session. Documented
in "How Cognition Uses Devin to Build Devin" (February 2026). The
loop is human-mediated (user reviews the suggestions) but
mechanically-surfaced (the suggestions appear automatically; the
human doesn't have to manually mine them). This is distinct from
Knowledge API CRUD (which is an organizational artifact) and from
Playbooks (which are task-class templates). Session Insights are
*per-session-arc improvements to the prompt itself.*

**Implication for v2.** This is highly load-bearing for the redesign
mission. The system prompt for this redesign explicitly states (in
`<critical-context-read-first>`):

> "Without an explicit mechanism you will re-derive the same insights
> every cycle and the design will not converge."

v1's solution is the journal — but the journal is post-session
substrate, not surfaced. The cold-start orchestrator reads the
journal and *derives* "what would help next session" from it. If the
deriving is implicit and per-cycle, drift is inevitable (cycle-59
row-ordering and cycle-61 hypothesis-counting findings both surfaced
that implicit conventions inherited across cycles drift undetected).
Session Insights is the missing primitive: a tool that runs at
session-end, reads the journal entry just written, and produces a
"for-next-session" capsule explicitly.

**v2 design candidate input.** A `surface-next-session-input` tool
(Rust binary per `<constraints>`) that runs at session-end as part of
the standard cycle-completion sequence. Inputs: the just-written
journal entry, the just-closed cycle issue, the dispatch-state ledger.
Outputs: a `docs/redesign/next-session-input.md` (or analogous) file,
overwritten each cycle, that the next cold-start orchestrator reads
*first* before deriving anything from longer history. The capsule
contains: outstanding dispatches, in-flight checkpoint artifacts,
methodological flags surfaced this cycle, the provisional read for
next cycle. The current "Provisional read for cycle N+1" prose at
the end of each journal entry is the manual ancestor of this
mechanism.

A v2 enhancement: in addition to the human-readable capsule, a
machine-readable manifest (TOML or JSON) lists structured items the
next session can act on without re-deriving (e.g.,
`open_dispatches: [#2833]`, `polarity: substantive-focal-default`,
`fallback_shape: cold-reader`). The next orchestrator reads both;
the manifest avoids re-deriving easily-deriveable things; the prose
preserves nuance.

**Discount.** Cognition's Session Insights is human-in-the-loop; the
user reviews suggestions before applying. v2 needs to decide: are
"for-next-session" suggestions auto-applied (the next orchestrator
reads and acts on them without explicit approval), or held for
review? Auto-applied is faster; review-required is safer. The
redesign mission ("self-healing self-improving system") favors
auto-applied for low-stakes items (provisional read for next cycle)
and review-required for high-stakes items (proposed prompt edits).
Phase 2 should articulate this split.

**Cross-reference.** Singular-voice as of cycle 66 — neither AutoGen
nor LangGraph has a structural Session Insights analogue. New cluster
H ("post-session feedback as a structural primitive") with one
implication as singleton; openclaw and oh-my-codex may or may not
have analogues — to be checked when their implications are mined.
Highly load-bearing despite singular-voice status given the redesign
mission.

---

### I-C8. Map-reduce-and-manage as the practical multi-agent shape

**Cognition-specific evidence.** April 2026 explicitly names the
practical multi-agent shape:

> "Arbitrary networks of agents negotiating with each other is mostly
> a distraction. The practical shape is map-reduce-and-manage."

Map-reduce-and-manage is the Managed Devins shape: a coordinator
scopes tasks, parallel children execute in isolated environments,
the coordinator synthesizes results. Single-threaded writes are
preserved because tasks are scoped to be independent (no two children
write to the same target). The coordinator IS the manager — there is
no separate "manager-of-managers" tier; the architecture is
deliberately flat.

**Implication for v2.** v1 already operates in a map-reduce-and-manage
shape de facto: the orchestrator dispatches Copilot research /
feedback / implementation tasks (parallel "map" workers); Copilot
outputs as PRs or comments; the orchestrator integrates per-finding
(the "reduce" step at cycle-7 / cycle-12 / cycle-31 / cycle-43 /
cycle-41 absorption pattern); the orchestrator writes single-threaded
to repo. The shape is unnamed, which means future iteration may
accidentally violate it (e.g., dispatching two Copilot sessions to
edit the same file in parallel; auto-merging without per-finding
review). Naming the shape protects it.

**v2 design candidate input.** Articulate map-reduce-and-manage as a
v2 architectural axiom alongside write-single-threaded:

- **Map step.** Dispatched workers (Copilot research dispatches,
  Copilot feedback dispatches, Copilot implementation dispatches,
  audit-repo critique requests) operate in isolated context with
  explicit scope.
- **Reduce step.** Orchestrator integrates worker outputs through
  per-finding evaluation (the cycle-7 / cycle-12 / etc. pattern).
- **Manage step.** Orchestrator scopes the next batch of dispatched
  work based on integrated state.

The axiom forbids: dispatching workers that write to the same target
in parallel; auto-merging worker outputs without per-finding review;
dispatch chains where worker A's output is consumed by worker B
without orchestrator-mediated integration in between.

**Discount.** Cognition's map-reduce-and-manage runs at the scale of
659 PRs/week internally, with VM-isolated children and internal MCP
for coordinator-children communication. Ours runs at single-digit
dispatches per week with GitHub Actions isolation and PR-based
integration. The shape transfers; the throughput conditions which
optimizations matter (Cognition needs internal MCP at their scale; we
don't).

**Cross-reference.** Strongly augments cluster A (cycle-internal
boundaries with state-write semantics) by elevating the *dispatch
boundary* to peer-status with the super-step boundary (LangGraph
I-L1) and termination boundary (AutoGen I-3). Cluster A had 5
implications (1 strong-convergent + 3 LangGraph-singular + 1
AutoGen-singular); I-C8 makes it 6 with a third-system contribution.
The dispatch-as-super-step framing reframes super-step semantics from
within-process (LangGraph) to across-process (Cognition) — same
discipline, different substrate.

---

### I-C9. Under-delegation as a named failure mode peer to over-delegation

**Cognition-specific evidence.** April 2026 documents a new named
failure mode in the manager-of-managers context: "Overly prescriptive
managers backfire." Managers that micromanage children — over-
specifying the task, prescribing too many steps, refusing to delegate
genuine decisions — produce worse outcomes than under-managed
delegations. The implication: under-delegation is as dangerous as
over-delegation; the failure modes are dual.

**Implication for v2.** The redesign mission explicitly inverts v1's
failure mode of *over-prescription*: v1's prompt has 12+ XML sections
+ STARTUP_CHECKLIST + COMPLETION_CHECKLIST; the redesign target is
"prompt is small, tools handle routine." But Cognition's finding
warns that swinging too far toward "small prompt, delegate everything"
introduces the dual failure mode. v2 should publish under-delegation
as a documented failure mode peer to over-delegation, so future
iterations don't swing to either extreme.

**v2 design candidate input.** The retrospective at
`docs/redesign/0-retrospective.md` already names over-prescription
patterns as F-patterns. Phase 0's completion should add
under-prescription patterns as forward-anti-patterns to monitor.
Concrete v2 anti-patterns to declare:

- **Under-prescribed dispatch instructions** — vague "review this and
  comment" without scope, lens, or success criteria. Cognition's
  framing: managers must scope tasks crisply even while delegating
  genuine decisions within the scope.
- **Under-specified Playbooks** — task-class templates with no
  forbidden actions section. Cognition's Playbook structure includes
  "forbidden actions" as a load-bearing field; v2 Playbooks should
  too.
- **Under-bounded autonomy ranges** — `<eva-default-autonomy>`
  declares "default to RESOLVING ISSUES YOURSELF" but lacks
  affirmative scope (what classes of issues qualify; when to escalate
  even within the autonomy range). Phase 2 should articulate the
  affirmative scope explicitly.

**Discount.** Cognition's "overly prescriptive manager" is observed
in the Managed Devins coordinator context — coordinator-of-children
scope. v1's analogue is the orchestrator-of-Copilot-dispatches scope
(orchestrator IS the coordinator; Copilot dispatches are the
children). The substrate matches more cleanly than for some other
implications. The failure mode probably transfers without strong
discount.

**Cross-reference.** Augments cluster D (documentation honesty) by
extending the discipline to *forward-anti-patterns* (failure modes the
v2 system declares it must monitor for, not just F-patterns of v1
that are documented retrospectively). Cluster D becomes 5 implications
across 3 systems: AutoGen I-4 + I-7 (existing), LangGraph section 2.8
(promoted in cycle-65 Finding 4), Cognition I-C1 + I-C2 + I-C9 (new).
This is the cycle-65-hypothesized cluster-D conversion confirmed:
**partial-convergent → strong-convergent at three-system depth.**

---

## Cross-system convergence with cycle-62 (AutoGen) and cycle-64 (LangGraph)

This section updates cycle-65's six-cluster decomposition with
cycle-66 contributions. Cycle-65 had 16 implications across 6 clusters
with a specific hypothesis to test (cluster D conversion). Cycle-66
adds 9 implications, bringing the total to 25 implications across the
same 6 clusters plus 2 new clusters (G and H).

### Updated cluster table

| Cluster | Theme | Cycle-65 state | Cycle-66 additions | Cycle-66 state |
|---|---|---|---|---|
| A | Cycle-internal boundaries with state-write semantics | 5 impls (1 strong-conv + 3 LG + 1 AG via I-2) | I-C8 (map-reduce-and-manage; dispatch boundary as super-step peer) | 6 impls, **3-system signal** at cluster level (AG, LG, Cognition all foreground), strong |
| B | Cross-cycle artifact organization | 3 impls (1 strong-conv + 1 AG-singular) | I-C3 (multi-layer memory architecture; 7 mechanisms) | 4 impls, **3-system strong-convergent**, very high signal |
| C | Lifecycle operations beyond resume | 4 impls (1 strong-conv + 1 AG + 1 LG via I-L8) | I-C5 (event-triggered wakeup) | 5 impls, **3-system strong-convergent**, high signal |
| D | Documentation honesty | 3 impls (2 AG + 1 LG-implicit) — **partial-convergent** | I-C1 (walkback) + I-C2 (invariants/derivations) + I-C9 (under-delegation as named failure) | 6 impls, **3-system strong-convergent**, **CONVERSION CONFIRMED** |
| E | Typed boundary semantics | 2 impls (AG I-5 + LG I-L2) — partial-convergent | none | 2 impls, partial-convergent |
| F | Tool-suite and prompt-suite stratification | 1 impl (AG I-2) — singleton | I-C6 (Playbooks as prompt-suite stratification) | 2 impls, **2-system convergent** (AutoGen + Cognition; LangGraph silent) |
| G | Role-asymmetric context semantics | (did not exist as cluster) | I-C4 (clean-context reviewer) | 1 impl, **NEW cluster, singleton** |
| H | Post-session feedback as structural primitive | (did not exist as cluster) | I-C7 (Session Insights → for-next-session capsule) | 1 impl, **NEW cluster, singleton** |

**Cluster D conversion confirmed.** Cycle-65's hypothesis was: "adding
a third system revises the cluster table and may convert cluster D
from partial-convergent to strong-convergent if Cognition or openclaw
foregrounds documentation honesty." Cognition's June 2025 → April 2026
walkback (I-C1) is a textbook documentation-honesty exercise. The
walkback explicitly factors into invariants vs derivations (I-C2),
and explicitly names a new failure mode (I-C9). Three Cognition
implications all land in cluster D. Combined with AutoGen's 2 and
LangGraph's implicit 1, cluster D is now 6-implication 3-system
strong-convergent — converted as hypothesized.

**Two new singleton clusters (G and H).** Cycle-66 introduces patterns
that neither cycle-62 nor cycle-64 surfaced. Both are singular-voice
as of cycle 66; openclaw / Voyager / OpenAI-harness / oh-my-codex
implications mining (in subsequent cycles) may add to either cluster.
Cluster G (role-asymmetric context) is a candidate for openclaw
augmentation (delegate architecture may foreground asymmetric context
between delegate and coordinator). Cluster H (post-session feedback)
is a candidate for openclaw or oh-my-codex augmentation (both have
documented self-improvement mechanisms in their respective surveys,
though not yet mined).

### Three-system convergence summary

| Convergent depth | Implications |
|---|---|
| **3-system strong-convergent (cluster-level)** | Clusters B, C, D — each has Cognition contribution paired with AutoGen + LangGraph contributions on the same architectural theme |
| **3-system signal at cluster level (with cluster A's caveat)** | Cluster A — Cognition I-C8 reframes super-step semantics (LangGraph) and termination predicates (AutoGen) at the cross-process boundary; signal is at cluster level not pair-level |
| **2-system convergent** | Cluster F (AutoGen I-2 + Cognition I-C6 — both stratification disciplines, silently absent in LangGraph) |
| **2-system partial-convergent** | Cluster E (AutoGen I-5 + LangGraph I-L2 — typed boundary semantics; Cognition silent) |
| **Singular-voice (singleton clusters)** | Cluster G (Cognition I-C4 only); Cluster H (Cognition I-C7 only) |

The 3-system depth across clusters B, C, D (and arguably A) is a
substantively stronger signal than cycle-65's 2-system clusters. Phase
2 candidate authors gain higher-confidence design-input on those
clusters because three independent systems with different substrates
(AutoGen Python library, LangGraph Python/TypeScript library,
Cognition closed hosted product) all foreground patterns that map to
the same cluster.

The 2-cluster diversity hedge (cycle-65's `[2-system strict + diversity
hedge]` maturity badge) for clusters B, C, D upgrades to
**3-system clean** at cluster level. The substrate diversity is now:
Python library (AutoGen) + Python/TypeScript library (LangGraph) +
closed hosted product (Cognition) — three different substrate
positions on the library/library/product axis. The pure-library
hedge no longer applies.

### Within-system intersections (Cognition Devin)

A within-system intersection matrix for Cognition completes the
parallel to cycle-65's matrices for AutoGen and LangGraph:

| | I-C1 | I-C2 | I-C3 | I-C4 | I-C5 | I-C6 | I-C7 | I-C8 | I-C9 |
|---|---|---|---|---|---|---|---|---|---|
| **I-C1** walkback | — | strong | none | weak | none | none | none | none | strong |
| **I-C2** invariants/derivations | strong | — | none | none | none | weak | none | weak | strong |
| **I-C3** multi-layer memory | none | none | — | weak | weak | strong | strong | weak | none |
| **I-C4** clean-context reviewer | weak | none | weak | — | none | weak | none | strong | weak |
| **I-C5** event-triggered wakeup | none | none | weak | none | — | none | weak | strong | none |
| **I-C6** Playbooks | none | weak | strong | weak | none | — | weak | weak | strong |
| **I-C7** Session Insights | none | none | strong | none | weak | weak | — | none | weak |
| **I-C8** map-reduce-and-manage | none | weak | weak | strong | strong | weak | none | — | weak |
| **I-C9** under-delegation | strong | strong | none | weak | none | strong | weak | weak | — |

Strong intersections (within Cognition):

- **I-C1 ↔ I-C2 ↔ I-C9: documentation-honesty triangle.** Walkback
  mechanism (I-C1) + invariant/derivation separation (I-C2) +
  named-failure-modes (I-C9). All three are documentation-honesty
  patterns at different time horizons (I-C1 about prior position,
  I-C2 about layered current position, I-C9 about anticipating future
  failures). This is the Cognition-side of cluster D, structurally
  parallel to the AutoGen-side I-1 ↔ I-4 ↔ I-7 documentation-discipline
  triangle (with one positional shift: AutoGen has de-prescription as
  the "across-versions" anchor; Cognition has walkback as the
  "within-version revisited" anchor).
- **I-C3 ↔ I-C6 ↔ I-C7: artifact-stratification triangle.** Multi-
  layer memory (I-C3) + Playbooks as task-class templates (I-C6) +
  Session Insights as feedback (I-C7). Three of the seven mechanisms
  in I-C3's table ARE Playbooks and Session Insights themselves;
  the triangle is partially self-referential. This is the
  Cognition-side of clusters B + F + H (a triangle that spans three
  cycle-65/66 clusters because Cognition's stratification discipline
  cuts across content-type, structural-role, and post-session-feedback
  axes simultaneously).
- **I-C4 ↔ I-C8: dispatch-coordination pair.** Clean-context reviewer
  (I-C4) + map-reduce-and-manage (I-C8). Both about how
  parallel-execution is structured: clean-context for critique,
  scoped-children for parallel work. Together they form the
  multi-Devin coordination discipline. New cluster G's I-C4 has its
  strongest intersection with cluster A's I-C8 — analogous to cycle-65
  finding that orphans tend to have cross-cluster intersections.

Orphans (no strong intersection within Cognition):

- I-C5 (event-triggered wakeup) — within Cognition, intersects only
  weakly. Its strong intersection is *cross-system* with cluster C
  (lifecycle operations) — pairing with AutoGen I-3 (termination
  predicates) and AutoGen I-8 (Reset/Resume) and LangGraph I-L4 (time
  travel as fork) at the lifecycle-discipline level.

The pattern from cycle-65 (orphans tend to have strong cross-system
intersections) reproduces in Cognition: I-C5's strongest intersection
is cross-system with cluster C peers, exactly as cycle-65's pattern
predicted.

### Updated three-pair convergence list (for future elevation)

Cycle-65 drafted elevation forms for 3 strong-convergent pairs in
1-research.md Family format. Cycle-66 enables drafting for additional
3-system clusters:

**3-system clean elevations now available:**

- Cluster B (cross-cycle artifact organization with content-type
  ×temporal-scope×opinion-level stratification) — AutoGen I-6 +
  LangGraph I-L5 + Cognition I-C3
- Cluster C (lifecycle operations beyond resume — including
  event-triggered wakeup) — AutoGen I-8 + LangGraph I-L4 + Cognition
  I-C5; AutoGen I-1 + LangGraph I-L8 sit nearby
- Cluster D (documentation-honesty discipline including walkback,
  invariant/derivation layering, and named-failure-modes) — AutoGen
  I-4 + I-7 + LangGraph section 2.8 + Cognition I-C1 + I-C2 + I-C9

**3-system signal at cluster level (sub-pair drafting still 2-system):**

- Cluster A — the within-process super-step semantics (LangGraph) and
  the across-process dispatch boundary (Cognition) and the termination
  predicates (AutoGen) are different sub-patterns at the cluster
  level. Elevation drafting at sub-pair level remains 2-system; at
  cluster level it's 3-system.

**2-system clean (unchanged from cycle-65):**

- Cluster F (tool-suite and prompt-suite stratification) — AutoGen
  I-2 + Cognition I-C6

**Actual elevation is still deferred** per cycle-64/65's recommendation
to a future cross-system synthesis cycle. Cycle 66 produces
implications-mining material; the elevation cycle (option 5 from
#2829) will draw from cycle-66 along with cycle-62 / cycle-64 / cycle-65
synthesis. The cluster-D conversion + cluster-B / cluster-C 3-system
clean upgrades give the elevation cycle copy-edit-able starting
points for stronger Family-format observations than cycle-65's drafts
could produce.

## What this informs

Phase 2 candidate authors gain Cognition-Devin-specific design-input
not in `1-research.md` cross-system synthesis or in
`2-design-framework.md` axes:

- **`POSITIONS.md` artifact** as a forward-compatible thesis-evolution
  mechanism (I-C1)
- **Invariants/derivations stratification** as a v2 prompt structure
  primitive (I-C2)
- **Multi-mechanism memory architecture** as the persistence-shape
  discipline — distinct mechanism per (scope, lifetime) coordinate
  (I-C3)
- **Per-dispatch-type context semantics** with clean-context as
  critique-role default (I-C4)
- **Event-triggered wakeup augmenting cron** as a lifecycle primitive
  (I-C5)
- **Per-cycle-composition Playbooks** as a v2 prompt-suite
  stratification mechanism, naturally pairing with cycle-65's
  cycle-composition-tag-at-session-start recommendation (I-C6)
- **Post-session feedback capsule** as a load-bearing primitive for
  the redesign's self-healing-self-improving mission (I-C7) — highly
  load-bearing despite singular-voice
- **Map-reduce-and-manage** as a v2 architectural axiom forbidding
  parallel-writer dispatches and chained dispatches without orchestrator
  integration (I-C8)
- **Under-delegation as named failure mode** alongside over-delegation
  (I-C9)

Cluster D conversion confirmed at three-system depth — documentation
honesty (walkback + invariants/derivations + non-guarantees +
aspirational-vs-implemented + named-failure-modes) is a strong-
convergent v2 design discipline.

Two new singleton clusters (G role-asymmetric context, H post-session
feedback) await augmentation from openclaw / oh-my-codex / Voyager /
OpenAI-harness implications mining in subsequent cycles.

## What remains open

- **Cluster G and H augmentation depth.** Singular-voice clusters tend
  to either grow into multi-system clusters or remain singular. Two
  more implications-mining cycles (e.g., openclaw + oh-my-codex when
  #2833 returns) will resolve which way they go.
- **Cross-implications-cluster intersections.** Cycle-66 Cognition
  matrix shows cluster G's I-C4 has its strongest intersection with
  cluster A's I-C8 (cross-cluster). Cycle-65 found similar
  cross-cluster intersections (orphans tend to have cross-system
  strong intersections). A future synthesis cycle could draw a
  cross-cluster intersection map across all 25 implications and
  surface cross-cluster mesh structure that the 8-cluster
  decomposition obscures.
- **Whether cluster H's I-C7 (Session Insights → for-next-session
  capsule) should be elevated to first-priority Phase 2 input given
  the redesign mission.** Cluster H has one implication and is
  singular-voice as of cycle 66, but the implication is structurally
  load-bearing for the redesign's self-improving thesis. Phase 2
  candidate authors should weight implications by mission-relevance
  not just by cross-system depth — singular-voice + high mission
  relevance is potentially more valuable than 3-system convergent +
  low mission relevance. v2 design-input candidate: explicit
  weighting discipline for implications-mining outputs.
- **Whether the v2 prompt's INVARIANTS section requires a security
  carve-out for self-modification gates** (current production prompt
  carves out `<self-modification-gates>` from `input-from-eva`
  override). I-C2 implies INVARIANTS section as a load-bearing
  separation; the security carve-out is one example of a current
  invariant; Phase 2 should articulate which other invariants belong
  in the same layer.
- **Whether cluster F (tool-suite stratification) extends to cluster
  H (post-session feedback) in the v2 tool suite.** The
  surface-next-session-input tool (I-C7) is itself a tool whose
  opinion-level matters: heavy-opinion (ALWAYS auto-applies provisional
  read for next cycle) vs light-opinion (writes capsule, next session
  decides). Phase 2 should articulate.

## Methodological observations after three implications-mining cycles
plus one synthesis

**The implications-mining cadence is now demonstrably viable at THREE
instances** (cycles 62, 64, 66), confirming cycle-64's "cadence is now
demonstrably viable as cycle-composition shape at second instance"
observation at the next-instance-count. Cycle-62 produced ~430 lines,
cycle-64 ~620 lines, cycle-66 likely ~700 lines (TBD at writing). All
three fit within single-cycle session budget (~75 minute compute).
Cycle-66 took longer than cycle-62/64 not because of system-density
but because the cluster-conversion check + within-system matrix added
synthesis-layer work *during* the mining cycle. Future implications-
mining cycles can choose: pure mining (~430 lines, faster) vs
mining-with-synthesis-update (~700 lines, slower). Either is valid;
the latter accelerates the next pure-synthesis cycle's work.

**Three-system depth changes the cluster-elevation calculus.**
Cycle-65 noted clusters with 2-system support carried a `[2-system
strict + diversity hedge]` maturity badge (both AutoGen and LangGraph
are Python/TypeScript/.NET in-process libraries — limited substrate
diversity). Cycle-66's Cognition (closed hosted product) breaks the
substrate symmetry — clusters B, C, D, and arguably A become
3-system clean. The diversity-hedge concern dissolves at three-system
depth with substrate variation; what was previously hedged as
"library-pattern-only" now has product-substrate confirmation.

**The hypothesis-driven cycle structure works.** Cycle-65 explicitly
hypothesized that adding a third system "may convert cluster D from
partial-convergent to strong-convergent if Cognition or openclaw
foregrounds documentation honesty." Cycle-66 confirmed the hypothesis
with 3 Cognition implications all in cluster D. Hypothesis-driven
cycle structure (predict outcome → test → confirm/refute) accelerates
research-corpus advancement compared to undirected mining. v2 design-
input candidate: cycle composition tags should support a
"hypothesis" subtype declaring what observation would falsify the
predicted outcome.

**Singular-voice clusters are stable until a contradicting system is
mined.** Cluster F was AutoGen-singleton; cycle-66 Cognition I-C6
makes it 2-system. Cluster G (I-C4) and cluster H (I-C7) are now
Cognition-singleton; future cycles will resolve. The implication: a
singular-voice cluster at N-system depth has expected value of
becoming multi-system at (N+1)-system depth roughly proportional to
the *next system's substrate similarity* to the singular-voice
system. Cognition is the most substrate-distant system in the corpus
(closed product vs libraries); subsequent systems (openclaw,
oh-my-codex, Voyager) are closer to AutoGen/LangGraph substrate-wise.
Cluster G/H 3-system extension is therefore lower-probability than
cluster D extension was before this cycle.

**Within-system intersection matrices reveal a consistent orphan
pattern.** Cycle-65 found AutoGen I-6 and LangGraph I-L5 are
within-system orphans whose strongest intersection is cross-system
(the cluster B convergent pair). Cycle-66 finds Cognition I-C5 is
also a within-system orphan whose strongest intersection is
cross-system (cluster C). The pattern is: implications without strong
same-system neighbors are *predominantly* the ones that anchor
cross-system clusters. This is structurally informative — it suggests
the within-system intersection matrix is a useful discovery tool for
elevation candidates (look for the orphans). v2 design-input
candidate: a tool that computes within-system intersection matrices
mechanically once an implications doc is written.

## Cycle accounting

**Polarity-pivot continuation discipline.** Cycle 66 is the FIFTH
consecutive cycle of research-corpus advancement under #2829's
polarity inversion (cycle 62 = AutoGen mining; cycle 63 = oh-my-codex
deeper-read dispatch construction; cycle 64 = LangGraph mining;
cycle 65 = cross-implications synthesis; cycle 66 = Cognition Devin
mining + cluster-conversion check + within-system matrix).
Five-cycle robustness on the polarity-pivot pattern with three
distinct cycle composition shapes (mining, dispatch-construction,
synthesis) all viable as substantive focal under #2829. Cycle-66
mixes mining + synthesis-update — either cycle composition shape on
its own would have been valid; combining them at the same cycle is a
shape variant (call it `mining-with-synthesis-update`).

Cold-reader was not in play this cycle (substantive option 4
continuation fully viable; option 1 already dispatched at cycle 63;
option 2 already executed at cycle 65; option 3 still untouched).

**Provisional read for cycle 67.**
1. **If cycle-63 dispatch [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) deliverable returns by cycle 67:**
   per-finding evaluation absorption cycle (parallel to cycle-41
   cognition / cycle-43 openclaw absorption pattern). Highest priority
   — the dispatch was costly to construct and the deliverable's value
   compounds when integrated rapidly.
2. **Otherwise, implications mining on a fourth deep-dive system
   (option 4 continuation):** openclaw is the highest-value next
   target — anti-pattern catalog framing, delegate architecture
   (potentially augmenting cluster G), security-model / sandboxing
   foregrounding (potentially a new cluster I). Plus openclaw is
   already deeper-read (cycle-43) so no dispatch needed. Voyager and
   OpenAI-harness are also viable but openclaw has the densest
   implications surface based on the per-system file size (605 lines
   vs 217 LangGraph / 211 Cognition / 173 AutoGen).
3. **Otherwise, v1-system retrospective mining (option 3 from #2829):**
   implications-to-F-pattern mapping for Phase 2 candidate evaluation
   against retrospective evidence. Useful but lower priority than
   continuing the implications-mining cadence while it has unique
   systems left to mine.

**The implications-mining cadence still sustains for at least three
more cycles before exhausting unique-deep-dive systems** (openclaw,
Voyager, OpenAI-harness, oh-my-codex once cycle-63 returns = 4
systems remain to mine; we've done 3). After that, cross-system
implications synthesis (option 2) becomes the natural transition
activity — likely a multi-cycle synthesis arc given 32+ implications
across 7+ systems.

**Iteration-until-approval honest reflection.** Cycle 66's mining
adds 9 implications and confirms cycle-65's cluster D conversion
hypothesis at three-system depth. The within-system matrix for
Cognition + the updated cluster table for the full three-system
corpus are synthesis-layer work that future synthesis cycles inherit
as completed rather than re-deriving. Cycle 66 is genuine
iteration-until-approval activity at quality comparable to cycles
62/64 mining cycles, with a more demanding cycle composition shape
(`mining-with-synthesis-update`) that adds upper-cost / upper-value
to the cycle. Future readers (Eva at post-retrospective checkpoint,
Phase 2 candidate authors) will see a stronger cross-system signal
(3-system clean for clusters B, C, D vs cycle-65's 2-system
hedged-clean) plus two new singleton clusters that openclaw mining
may augment. This is the FIFTH consecutive cycle producing additive
research-corpus output under #2829, validating the polarity-pivot
diagnosis at five-cycle robustness.
