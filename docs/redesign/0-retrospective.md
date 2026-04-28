# Phase 0: Critical retrospective on v1

**Status**: working draft — iterates across many cycles. Eva approval required
before Phase 1. While awaiting approval each cycle should sharpen this artifact.

**Reading guide**: this is not a self-flagellation exercise and not a status
report. It is the source-of-truth catalog of what v1 actually is and where it
fails, written so a future cycle (or candidate-design author, or Eva, or the
audit orchestrator) can ground design decisions in evidence rather than
sentiment. Cite cycle numbers; quote constraints by id; link issues.

---

## Glossary (brief)

Operational vocabulary used in this retrospective. The full legibility
sweep (replacing jargon with prose throughout) is queued.

- **C-stages (C1–C8, C5.5)** — close-out checklist roots in
  `COMPLETION_CHECKLIST.xml`. C5 is "freeze the worklog," C5.5 is a
  validation gate that runs at close-out and verifies the C5 freeze
  produced a coherent worklog (it was added late in v1's lifecycle as
  a check-after-the-fact mechanism), C6 is "dispatch the review,"
  C8 is "write the journal entry."
- **Worklog** — per-cycle artifact at `docs/worklog/YYYY-MM-DD/cycle-N.md`
  recording the cycle's actions, dispatches, merges, and pipeline state.
  Frozen at C5 from C5.5 gate state. Central to F4 (frozen-artifact
  lifecycle) and F11 (cycle closure as artificial completion).
- **Review agent** — per-cycle Copilot dispatch (gpt-5.x-class) that
  reviews the prior cycle's artifacts and posts findings as a PR. The
  orchestrator dispositions findings on the next cycle. Central to F9
  (adversarial-review-and-disposition treadmill).
- **Step comments** — comments posted to the cycle issue announcing each
  judgment step the orchestrator takes during a cycle. The v1 prompt's
  `step-commenting` directive mandates a separate comment per judgment
  step. Central to F6 (procedural depth).
- **Pipeline-check** — Rust tool at `tools/rust/crates/pipeline-check/`
  with ~36 sub-checks (30 logical after collapsing date variants), most
  of which exist to detect specific past failure modes (the F12 catalog
  characterizes the pattern). The C5.5 gate runs pipeline-check.
- **Recurrence escalation** — a citation pattern in v1 prompt/checklist
  constraint blocks recording that the constraint *did not prevent* a
  later occurrence of the same failure. E.g.
  "Recurrence escalation: cycle 449 F3 process-adherence (deferred 5
  cycles, deadline cycle 454)." A constraint with a recurrence-escalation
  citation is a constraint that, by its own admission, did not work.
- **Chronic-category** — a finding category from the per-cycle review
  that recurs across 5+ of the last 6 reviews. v1 has 6+ such
  categories. Chronic-category currency tracking is a substantial
  fraction of v1's per-cycle compute (per F9).
- **Foreground vs background Eva action** — "foreground" means Eva
  files a new issue or opens a new PR; "background" means Eva replies
  to an existing question-for-eva issue. v1 detects foreground reliably
  and missed background for 6+ days in the F2 incident.
- **Dispatch-task** — the v1 Rust tool that atomically files a Copilot
  agent issue and records the dispatch in `state.agent_sessions`. The
  conventional way for v1 to ask Copilot to do work.
- **Audit repo / main repo** — `schema-org-json-ld-audit` runs an
  independent orchestrator that audits this repo (`schema-org-json-ld`).
  Both run Claude Opus 4.7. The audit repo's read scope is broader
  (cross-cycle, cross-repo); its model class is the same as the main
  orchestrator's.
- **F-pattern** — a named failure mode in v1's behavior, with cycle
  citations and a hypothesis about its root cause. Twelve are
  cataloged below (F1–F12). The failure-families preamble groups them
  into four working families as a hypothesis under iteration; the
  grouping is the artifact's current best read on how the patterns
  relate, not a settled taxonomy.

---

## What v1 actually is (as of redesign cycle 1, cycle 545 era)

The v1 system is roughly:

- **Prompt surface** — `.github/workflows/orchestrator-prompt.xml` (559 lines,
  XML), `STARTUP_CHECKLIST.xml` (298 lines, 9 step roots), `COMPLETION_CHECKLIST.xml`
  (432 lines, 8 step roots). The XML is dense with directives, definitions,
  thresholds, tool registries, sub-categories, recurrence-escalation citations.
- **Tool surface** — ~35 entries under `tools/`, of which 25+ are Rust binaries
  in `tools/rust/crates/`. Tools fall into four loose buckets: state mutators
  (`process-*`, `record-dispatch`, `write-entry`), checkers (`pipeline-check`,
  `validate-docs`, `state-invariants`, `housekeeping-scan`, `metric-snapshot`,
  `receipt-validate`), entry points (`cycle-runner`, `cycle-start`,
  `cycle-complete`, `cycle-close`), and dispatch (`dispatch-task`,
  `dispatch-review`, `merge-pr`).
- **State surface** — `docs/state.json`, currently 20,711 lines / ~600KB across
  38+ top-level keys. Each recurring failure type has accumulated its own
  field family (`step_comment_acknowledged_gaps`, `chronic_category_responses`,
  `audit_acceptance_registry`, `review_agent.history`, `field_inventory`, ...).
- **Documentation surface** — per-cycle worklog (`docs/worklog/YYYY-MM-DD/...`),
  per-day journal (`docs/journal/YYYY-MM-DD.md`), per-cycle review
  (`docs/reviews/cycle-N.md`), ADRs, plus the v1 prompt itself which is
  partially documentation by accretion.
- **Cross-repo surface** — audit repo (`schema-org-json-ld-audit`) runs an
  independent orchestrator on its own cron; QC repo (`schema-org-json-ld-qc`)
  consumes JSON-LD output and validates against Google Rich Results Test.
  Communication happens via labeled issues (`audit-outbound`, `audit-inbound`,
  `qc-outbound`, `qc-inbound`, `input-from-eva`, `question-for-eva`).
- **Loop**: cron at 21/01/05/09 UTC creates an `orchestrator-run` issue.
  The workflow runs `cycle-runner`, the orchestrator session navigates the
  startup checklist, performs work (review consumption, dispatches, schema work,
  housekeeping), navigates the close-out checklist, and dispatches a review
  agent. The next cycle reads that review and dispositions findings.

The schema-org-json-ld domain — the proof domain that this whole apparatus
exists to serve — is at zero. Not "small minority" (the cycle-1 framing of
this section was wrong by understatement); literally zero schema-source
commits over cycles 500–545 (46 consecutive cycles, measured cycle 2,
`docs/redesign/_notes/cycle-2-measurements.md`). The most recent
schema-source commit anywhere is 2026-03-24 (`59b08f89` and `7000e3255c8`,
quiz schema work, both pre-cycle-tagged so cycle attribution requires extra
work; the most recent `[cycle N]`-tagged schema commit is `[cycle 282]` on
2026-03-17). The intervening period is roughly 33 days and 130+ cycles of
zero schema output. Cycle 544 consumed 8 Eva responses, refreshed 6 chronic
categories, dispatched 2 PRs about dispatch tooling and worklog
reconciliation. Cycle 543 was a cycle-runner bug fix. Cycle 545 was a
hotfix on the cycle-544 work. This is the modal shape, and the rate of
schema work in the modal shape is zero.

---

## Failure patterns

These are concrete, evidenced patterns. Each gets a name, a description,
specific cited examples, and a hypothesis about its root cause. The hypotheses
are working drafts; sharpen across cycles.

### Failure families

The 12 F-patterns below are not equally independent. Several pairs and
triples are different manifestations of the same underlying mechanism at
different scales or substrates. Cycle 7 (`_notes/cycle-7-copilot-feedback-evaluation.md`,
lens 1) groups them into four families. Within each family the F-patterns
retain their identities and individual evidence; the family framing is the
load-bearing claim about how they relate.

| Family | Member patterns | Core mechanism |
|---|---|---|
| **Defense accretion** | F1, F5†, F11†, F12 | New failures get encoded as new defenses (constraints, state fields, cycle-boundary checks, pipeline-checks). Defenses accumulate across substrates without removal-tests. |
| **Reconciliation asymmetry** (write-mostly state) | F2, F3, F4, F5†, F11† | Outbound channels and write-tools are well-developed. Inbound reconciliation does not exist. Records are written; nothing reads them back to update meaning when subsequent events change them. |
| **Procedure / review overhead** | F6, F7, F9 | Procedural depth, review-and-disposition loops, and the chronic-category mechanism consume cycle compute disproportionate to the value they produce. |
| **Tooling fragility** | F8 | Multiple tools doing the same job in slightly different ways; one bug cascades across cycles before the parallel path gets fixed. |

† Dual-membered. F5 and F11 sit at the intersection of two families; both readings are correct, neither reduces the other:
- **F5** is both the storage stage of defense accretion (defenses accumulate
  as state fields) and a manifestation of reconciliation asymmetry (write-mostly
  state shape).
- **F11** is both the temporal stage of defense accretion (defenses fire on
  cycle-boundary triggers, post-close) and a manifestation of reconciliation
  asymmetry (post-close mutations are not reconciled into the frozen worklog).

**F10 is not a peer failure pattern in the same sense as F1-F9, F11, F12.**
F10 is a corrective-hypothesis-and-design-implication pattern: it names a
specific property of the audit repo (broader read scope) that v2 must
preserve, and the property-1/property-2 distinction (load-bearing read scope
vs illusory model diversity) is itself a design constraint. F10 is presented
in its own subsection below for continuity, but it is structurally
different from the operational failure patterns and should be read that way.

The cleanest single-sentence statement of the dominant family
(reconciliation asymmetry) is:

> v1 has tools to **create** records and tools to **summarize** records;
> it lacks tools to **reconcile** records against later events.

The cleanest single-sentence statement of the defense-accretion family is:

> When a failure surfaces, v1's response is to add a substrate-specific
> defense (a constraint, a state field, or a cycle-boundary check) and
> never test whether the defense is still load-bearing.

F1, F5, F11, and F12 are sibling manifestations of this response
pattern. F1 names the prompt/checklist substrate, F5 the state-shape
substrate, F11 the temporal substrate; F12 catalogs the cross-substrate
accumulation rather than naming a fourth substrate of the same kind.
Each pattern carries its own local mechanism (see the F-pattern sections
for details). *Sibling-not-upstream is asserted on local-mechanism
distinctness; not yet confirmed by removal-test evidence — fixing one
substrate could in principle relax the others, but v1 has not run that
experiment.*

The two families overlap because F11 (the temporal stage of
defense-accretion) is the point where reconciliation-asymmetry binds —
defenses fire post-close; the worklog freeze is pre-close; nothing
reconciles. F11 is also where the freeze-vs-refresh timing collision
(F11's local mechanism) plays out: artifacts freeze before refreshers
finish, structurally producing post-close divergence between frozen
artifacts and live state. v2 design implications for the two families
converge: every state field needs a write-tool AND a reconciliation-tool;
every channel needs a poller that produces state transitions; cycle
boundaries should be checkpoint markers on a continuously-evolving state,
not state hard-boundaries that freeze worklogs.

The F-pattern naming below leads with the named pattern and the family.
F-numbers remain as anchor IDs for cross-references but are de-emphasized.

### Constraint accretion as first-line response (F1) — *Defense accretion family*

When a new failure mode surfaces, the system's typical response is to add a
constraint to the prompt or checklist. The constraints carry their own
"Recurrence escalation" citations — receipts of their own subsequent
violations.

**Evidence**: `COMPLETION_CHECKLIST.xml` step C3 contains ten constraint
blocks (`pipeline-status-preliminary`, `gate-failure-honesty`,
`gate-criteria-change-disclosure`, `commitment-observability`,
`commitment-live-verification`, `commitment-carryover-ban`,
`final-state-accuracy`, `receipt-table-machine-scope`,
`issues-processed-scope`, `narrative-scope-boundary`). Each one cites the
cycle whose review generated it. Several cite recurrence — e.g.,
`receipt-table-machine-scope` notes "cycle 475 F1 re-flagged the same pattern
one cycle after cycle 474 cited receipt drift." `commitment-carryover-ban`
notes "cycle 476 F3 caught the cycle 476 journal doing exactly this."
The constraint exists; the violation continues; a new "recurrence escalation"
gets appended.

**Aggregate measurement** (cycle 4, `_notes/cycle-4-f1-measurement.md`).
Extending the C3 sample to all close-out constraint blocks (C3 + C5 +
C5.5 = 13 constraint additions during cycles ~430–509) and asking, for
each, whether a same-cycle tool fix was paired with the constraint:

- **12 of 13 (92%)** are constraint-only — no paired tool fix in the cycle
  the constraint was added. The single exception is `no-post-c5-mutation`
  (cycle 454), paired with PR #2266 ("Freeze close-out worklogs from C5.5
  gate state and remove post-dispatch mutation").
- **13 of 13 (100%)** carry a "Recurrence escalation: ..." citation,
  meaning each constraint was *insufficient* — the failure pattern
  recurred after the constraint was in force, and the constraint body
  itself records the recurrence. The constraint pattern self-documents
  its own failure rate.
- **Self-aware constraint-as-substitute**: the `rerun-step-comment-refresh`
  constraint body says verbatim: "Recurrence escalation: cycle 449 F3
  process-adherence (deferred 5 cycles, deadline cycle 454). **Behavioral
  fix in lieu of tool dispatch** — when the audit #382 worklog
  freeze-ordering fix lands, C5.5 reruns should become much rarer, but the
  discipline remains required." This is a single-case existence proof of
  the F1 mechanism in v1's own self-documentation: the authors knew the
  right fix was a tool and shipped a constraint anyway.

**Hypothesis**: a constraint in a 700+ line procedure is unlikely to change
behavior under cognitive load. The orchestrator scans the checklist; the
checklist names what it expects; constraints embedded inside expectations
get processed as background. A behavioral fix of this shape is structurally
weaker than a tool that mechanically prevents the failure or makes it
impossible to express. v1 reaches for behavioral fixes by default and tool
fixes only when the recurrence count gets embarrassing.

**Implication for v2**: do not let prompt text be the first-line response
to a new failure mode. The default response should be a tool that mechanically
prevents the failure. Where prompt text is genuinely the right response
(judgment-shaping rather than procedure-encoding), keep it small and
not-easily-mistaken-for-procedure.

**Audit-side parallel** (audit #442): the audit chain `#402 → #406 → #415
→ #417 → #420` (cycles 187–201) was 16 cycles of progressive structural-
patch recommendations on v1's chronic-category-tracking mechanism — each
rec a more specific structural fix than the last. Rec 4 was eventually
implemented in cycle 541 via PR #2718. But the audit was constraint-
patching the chronic-category-tracking mechanism when the mechanism
itself was the failure mode — v2 will replace the entire mechanism. The
audit chain's escalation pattern was constraint-accretion in slow motion,
with each filing a more elaborate structural patch on a surface v2
deprecates. Pattern: when a category of audit findings recurs across
multiple cycles, the right response may be "redesign the surface that
produces these findings," not "patch the surface." This is F1 applied at
the audit-system layer rather than the prompt layer.

### The Eva-response detection failure (F2) — *Reconciliation asymmetry family*

`schema-org-json-ld-audit#439` (filed 2026-04-25) catalogs this precisely:
on 2026-04-19, Eva spent ~55 minutes posting prescriptive responses to 8
standing `question-for-eva` issues. Each response was concrete (named
options, cited issue numbers, gave acceptance criteria). Across the next
12+ cycles (518–544), the orchestrator's journals listed those 8 issues
as "Standing Eva blockers" with growing staleness counters, while the
responses sat unread in plain sight. The orchestrator was reasoning against
its own incomplete information.

**Why v1 didn't catch it**: the staleness counter was rendered from issue
creation time, not from "time since last Eva comment." `Standing Eva blockers`
was a journal-rendered list re-derived each cycle from the open
`question-for-eva` label, with no state transition for "answered." Cycle 518
ran 9+ hours after Eva's batch response and never fetched her comments.
There was no checklist step that polled Eva-blocker issues for new comments
from EvaLok — the cycle-start sequence polled the audit repo and the QC
repo for new outbound issues, but did not poll its own own outbound issues
for inbound replies.

**The fix that v1 eventually applied** (cycle 544): Eva responded again,
this time more directly, and cycle 544 consumed all 8 responses in one cycle.
But the underlying structural defect — no polling step, no state transition
for "answered" — is unaddressed. If Eva goes silent for 6 days again, the
same thing will happen.

**Hypothesis**: v1 treats outbound communication and inbound communication
asymmetrically. Outbound (filing question-for-eva) is well-toolified;
inbound (Eva responds) has no tool path. The asymmetry is the bug. v2 must
treat any inbound channel (Eva, audit, QC, copilot critique) symmetrically
to outbound — every channel needs a poller and a state transition.

### Multi-candidate state drift (F3) — *Reconciliation asymmetry family*

`schema-org-json-ld-audit#437` (filed 2026-04-24): an audit acceptance in
cycle 531 enumerated three candidate fixes (a, b, c). Cycle 532 dispatched
(b). Candidates (a) and (c) were never explicitly decided on. The state
ledger entry `step_comment_acknowledged_gaps[cycle=528].reason` still reads
"(not yet dispatched)" for (a) and (b) and (c) all — written once at
acceptance, never reconciled when (b) shipped. Three subsequent cycles'
journals never grade the dropped candidates because the journal commitment
follow-through grading is "cycle-local" and only grades that-cycle's
explicit numbered commitments.

**Hypothesis**: v1's state shape is write-mostly. Tools write entries when
something happens, but very few tools reconcile entries against subsequent
events. The state.json `step_comment_acknowledged_gaps` field is one of
many that accumulates write-once entries with no decay path. This is the
same shape as F2 — outbound state grows, inbound reconciliation doesn't
happen.

**Implication for v2**: any state field that holds open-ended decisions
needs a reconciliation mechanism by design, not bolted on afterward.

### The frozen-artifact lifecycle is fragile (F4) — *Reconciliation asymmetry family*

The C5/C5.5/C6 sequence freezes the worklog from `state.c5_5_gate`, commits
it, then dispatches the review agent. But state mutations after C5
(specifically: `record-dispatch` records the review-dispatch and updates
`agent_sessions`, `last_cycle.summary`, etc.) make the frozen worklog
arithmetically inaccurate within the same cycle.

**Evidence**: cycle 544 review F1 — the frozen worklog says "Recorded 2
dispatches" and "In-flight agent sessions: 2 (unchanged: 0 new dispatches
this cycle)." Hours later, `docs/state.json` records a third cycle-544
dispatch for issue #2733 and `last_cycle.summary` says "3 dispatches, 0
merges." `bash tools/validate-docs` rejected the published artifact:
"pipeline status mismatch: worklog reports 'FAIL→PASS', pipeline-check
overall is 'fail'." The repo's own validator failed on its own published
artifact.

The constraint stack on C5 (`no-post-c5-mutation`,
`narrative-scope-boundary`, etc.) is the system trying to prevent this with
prose. It's not working — the v1 review agent is catching it as a finding,
which means the system is detecting its own incoherence after the fact
rather than preventing it.

**Hypothesis**: the cycle has phases (work, close_out, dispatch, post-dispatch).
v1's worklog is supposed to capture the cycle, but only one freeze happens
(at C5 from C5.5 state). State continues to mutate after the freeze. The
worklog and the state diverge. v2 either needs a worklog representation
that updates as state updates, OR the cycle's "cycle ended" boundary needs
to be later than the review-dispatch.

### State.json as procedural-implementation-leak (F5) — *Defense accretion family + Reconciliation asymmetry family (dual-membered)*

`docs/state.json` is 20,711 lines and growing. The shape mirrors v1's
procedural failures one-for-one: `step_comment_acknowledged_gaps` exists
because step comments sometimes don't get acknowledged; `chronic_category_responses`
exists because chronic categories need refresh tracking;
`audit_acceptance_registry` exists because audit acceptances need to be
tracked; `review_agent.history` exists because review findings need
disposition state; `field_inventory` exists because freshness markers
need their own state. Each new failure type that surfaces gets its own
state field; the field needs its own write tool, its own checker, its
own freshness marker; the freshness marker needs a refresh process; the
refresh process needs to be in pipeline-check; pipeline-check needs an
exclusion list for cycles before the field existed (see e.g. cycle 545's
hotfix at commit 567ad66d adding
`POST_DISPATCH_RECONCILIATION_FIRST_APPLICABLE_PREVIOUS_CYCLE = 545`).

**Aggregate measurement** (cycle 5, `_notes/cycle-5-state-categorization.md`).
Field-by-field categorization of all 42 top-level keys in `state.json`
(cycle 1's "38+" estimate was close):

- **19 of 42 (45%)** are pure **defenses** — added in response to a
  specific v1 failure mode (`step_comment_acknowledged_gaps`,
  `field_inventory`, `pending_audit_implementations`,
  `last_eva_comment_check`, `cycles_since_last_forward_work`,
  `deferred_findings`, `review_dispatch_consecutive`,
  `review_events_verified_through_cycle`, `audit_dropped`,
  `audit_processed`, `qc_processed`, `qc_requests_pending`,
  `last_tool_audit_cycle`, `open_questions_for_eva`, `eva_input_issues`,
  `step_comment_acknowledged_gaps`, `review_agent`, `in_flight_sessions`,
  `pre_python_clean_cycles`).
- **13 of 42 (31%)** are **primitives** — intrinsic to the schema-org
  domain (`schema_status`, `schema_version`, type/test counts,
  `typescript_plan`, `release`, `constructor_refactoring`). Of these,
  7 are pure counts (could be computed on demand from schema source).
- **10 of 42 (24%)** are **mixed** — primarily mechanical or primitive
  but warped by defense accretion (`cycle_phase`, `cycle_issues`,
  `tool_pipeline`, `project_mode` with stabilization sub-counters,
  `publish_gate`, `qc_status` accumulation).
- **Defense-character total: 19 D + 10 M ≈ 26–29 of 42 (62–69%)** of
  state.json's top-level fields exist primarily as defense.
- **Append-only accumulation evidence**: `agent_sessions` (922 items),
  `review_agent.history` (356 entries), `audit_processed` (197 items),
  `deferred_findings` (128 items). Unbounded retention is structural,
  not policy-enforced.

A v2 with a designed state model could plausibly collapse the 19 D + 10 M
shapes into something like 5 ledgers (QC interaction, audit interaction,
dispatch ledger with bounded retention, mode if preserved, single
project-progress record) plus the 13 P fields. That would be roughly a
4× reduction in top-level state surface, **but the 4× number is a
back-of-envelope smell test, not a v2 success criterion**. Top-level key
count is a weak proxy for conceptual complexity — one ledger can hide more
complexity than ten simple fields, and the count is gameable by
collapsing many fields into a few giant ledgers without reducing the
underlying surface. The 4× number is useful only as a smell test: if
v2 still requires ~40 top-level bookkeeping structures, the redesign
has likely failed; if v2 lands at ~5, that is consistent with success
but not by itself proof. Better v2 measures are designed in the
"What v2 must demonstrably do better than v1" section below.

**Hypothesis**: the state shape encodes the failures, not the system. Each
field is a scar. v2 should start from a much smaller state model that is
explicitly designed (cycles, dispatches, artifacts, reviews) — not grown
by accretion. The cycle 5 measurement quantifies the magnitude: 62%+ of
v1's state-of-record is defense, 31% is intrinsic — and the bulk of the
defense fields are write-mostly per the F11 measurement (post-close
mutations are these defenses running). F5 is therefore not just a
shape-of-state observation; it is the field-level instance of the
shared-root pattern (asymmetric communication / write-mostly state)
named in the preamble below.

### Cyclomatic procedure depth (F6) — *Procedure / review overhead family*

The startup checklist has 9 root steps (S0–S9) with 4–9 substeps each.
The close-out checklist has 8 root steps (C1–C8) with multi-substep
constraint stacks. `pipeline-check` runs ~32 sub-checks. Each substep has
conditions, cross-references, mode overrides, recurrence escalations. The
orchestrator spends a substantial portion of each cycle navigating this
graph correctly — posting step comments in the right order, ensuring
prerequisites are satisfied, applying mode-overrides when stabilization
mode would be active, branching on findings vs absent-findings.

The directive `<directive id="step-commenting" severity="mandatory">`
mandates that each judgment step be a separate comment. Cycles regularly
post 20+ comments before doing any actual work. This is "flight recorder"
discipline — but flight recorders don't fix the plane.

**Hypothesis**: the procedural depth is itself the problem the redesign
should solve. The orchestrator's compute should not be spent on procedure
navigation. v2 should make the procedural skeleton small (a few essential
checkpoints) and deep where it matters (judgment about novel circumstances).

**Evidence caveat**: F6 evidence is impressionistic. "20+ comments
before any actual work" is imagery, not data. A cycle-history
measurement (comments-per-cycle distribution; median; variance;
share-of-cycle-time on procedure navigation) is queued — comparable
to cycle 2's F7 measurement. Until measured, F6's headline claim
should be read as a strong-prior hypothesis, not a load-bearing
finding.

### Self-management dominance over schema work (F7) — *Procedure / review overhead family*

**Measured magnitude** (cycle 2, `_notes/cycle-2-measurements.md`):

- Cycles 500–545 (46 consecutive cycles, 2026-04-15 → 2026-04-26): zero
  commits to `php/src/`, `ts/src/schema/`, or `ts/src/enum/`. Confirmed via
  `git log --since="2026-04-15" -- php/src/ ts/src/` (returns empty).
- All ~23 PRs squash-merged in the cycle 500–545 window are
  self-management. Approximately 9 of 23 are *cycle-N adversarial review
  artifacts* (PRs #2678, #2681, #2689, #2692, #2698, #2708, #2713, #2720,
  #2734) — i.e., the system's review-of-itself output. The remaining 14
  are tool fixes, gate additions, prompt edits, and state-shape
  adjustments. None touch schema source.
- Last `[cycle N]`-tagged schema commit is cycle 282 on 2026-03-17. Last
  schema-source commit anywhere is 2026-03-24. Intervening period: ~33
  days, ~130+ cycles of zero schema output.
- This is **pre-redesign data**. The redesign-mode prompt was installed in
  cycle 545. The 130+ cycles of zero schema work happened under v1's
  normal operation, not in response to the redesign. The redesign
  authorization is a response to this state, not a cause.

The cycle 544 journal entry calls cycle 544 "the first cycle to fully
consume Eva's response queue without leaving any answered question open or
unactioned" and describes it as "the cleanest in 4 cycles." The cycle
worked through 8 Eva responses, refreshed 6 chronic categories, dispatched
2 self-management-tooling PRs. Zero schema-org work happened. By the v1
review-agent's grading rubric, cycle 544 was a *good* cycle. By the
proof-domain test, it produced nothing. The two assessments diverge
completely; v1's quality signal is decoupled from its mission.

The "secondary objective" in the v1 prompt is schema-org-json-ld
expansion. The "primary objective" is "an excellent, optimised,
high-quality autonomous workflow that continuously improves itself." v1
took the primary objective at face value: every cycle is now spent on the
workflow itself.

**Hypothesis**: the primary/secondary framing in the v1 prompt is
load-bearing. As written, it permits — encourages — endless self-improvement
without regard to whether the proof domain is being served. The redesign
prompt's mission framing inverts this ("the schema-org-json-ld work is
the proof domain: useful output that demonstrates the system is actually
doing real work"). But beyond the prompt, the cycle structure itself
biases toward self-management: chronic-category currency, Eva-blocker
queue, audit acceptance reconciliation, review consumption, dispatch
follow-up — all of these are checked every cycle, all expand under load,
none have natural completion conditions. The review-of-cycle artifact
production (9 of 23 recent PRs) is itself a self-replicating attractor:
each cycle dispatches a review, each review produces findings, each
finding becomes a follow-up dispatch, each follow-up is its own cycle's
work. There is no exit ramp from this loop in the v1 design. v2 needs
explicit design pressure away from this attractor — not just inverted
prompt framing, but tool-level constructs that make self-management
silent (background, non-cycle-consuming) when it's routine and visible
(escalated to orchestrator judgment) only when it's not.

### Abandonment cascades from single-tool defects (F8) — *Tooling fragility family*

Cycles 541, 542, 543 all abandoned at C5.5. Root cause: a single bug in
`cycle-runner::run_tool_json` that rejected non-zero exit codes from
`pipeline-check` even when stdout had valid JSON. One bug in one function
took down three consecutive cycles before being noticed, dispatched, and
fixed. The fix (PR #2726) merged in cycle 543 — but cycle 544 then
discovered a parallel implementation (`cycle-start::gather_pipeline_status`)
with the same anti-pattern, so cycle 544's startup brief still showed
`pipeline.status: unknown` even after the cycle-runner fix.

**Hypothesis**: the close-out path is fragile because it is one path. A
single tool returning a non-zero exit can cascade through three cycles. The
parallel-implementation gap (cycle-runner fixed, cycle-start not fixed)
indicates the system has multiple tools doing the same job in slightly
different ways, with tests against one and not the other.

**Prescription qualification**: "v2 should have fewer tools doing
each job" generalizes from the cycle-524 cascade case to a
system-level design principle. The cycle-524 case demonstrates the
cascade *mechanism*; whether parallel-implementation duplication is
widespread enough across v1 to merit a system-level "fewer tools per
job" principle requires a duplication-rate measurement that has not
been done. F12 sub-(c) (checklist + prompt + ADR rationales) is
queued and would partially address this. Until measured, the
prescription should be read as "the cycle-524 case argues for fewer
tools per job; whether v1 has the broader pathology is open."

The narrower hypothesis (the cycle-524 mechanism is real and v2 should
not replicate it) is well-supported. The broader prescription depends
on measurement.

### Adversarial-review-and-disposition treadmill (F9) — *Procedure / review overhead family*

Each cycle dispatches a review agent (Copilot, gpt-5.4-class). The review
finds findings. The orchestrator dispositions them (`actioned`,
`dispatch_created`, `deferred`, etc.). Dispositions become state. State
drives next cycle's review. Findings recur. The system has 6+ "chronic
categories" — those with 5+ of the last 6 reviews citing them. Chronic
categories require structural fixes; structural fixes are usually new
constraints in the prompt or new state fields; the new constraints get
violated; recurrence escalation citations accumulate.

**Hypothesis** (corrected from cycle 1's framing per audit #442): the
review agent is gpt-5.x-class (Copilot); the orchestrator is Opus 4.7. So
"same model class" is the wrong shared variable. The actual shared
variables that produce blind-spot convergence are **prompt + state
visibility + cycle context**: the review agent reads the same prompt
surface as it always does, sees the orchestrator's same-cycle state, and
operates on a one-cycle window. Findings recur because *that* combination
is constant across cycles, not because the model is. The
review-and-disposition loop generates work proportional to itself.

**Compute-claim qualification**: The "in steady state most cycle
compute is spent on the loop's own outputs" framing asserts a
stronger claim than F7's PR-mix measurement directly supports. F7 shows zero schema-source PRs across
46 cycles, which is consistent with most-cycle-compute-on-self-management
but does not directly measure compute attribution. The compute-attribution
measurement is itself queued for Phase 2 (per the success-criteria
section above; "fraction of cycle compute spent on domain output" has
no v1 mechanism). Until measured, F9's compute claim should be read as
"the PR mix and the cycle-2 measurement suggest a substantial fraction
of compute lands on the loop's own outputs; direct compute-attribution
measurement is Phase 2 work."

This matters because the audit repo *is* the same model class as the
orchestrator (both Opus 4.7 since cycle 511, audit's #427 acceptance) yet
catches different things. The audit's diversity comes from a different
prompt, broader state visibility (cross-cycle, cross-repo), and a
different cycle context (4-hour offset, multi-cycle window) — not from
model diversity. F10 makes this distinction explicit.

**Implication for v2**: review or critique is genuinely valuable, but the
diversity that makes it valuable is *prompt + state + context*, not model
class. v2's critique infrastructure should diversify those (different
prompts for reviewer vs orchestrator; different read scopes; different
windows), not chase model diversity for its own sake. Copilot feedback-
only sessions catch different things because they're given a different
prompt and don't see the orchestrator's state — same trick. v2 should
design the critique surface around genuinely independent prompt+state+
context, not around reflexive same-cycle self-review.

### Audit's value is broader read scope, not different model perspective (F10) — *Design-implication pattern, not a peer failure pattern*

Audit #439 explicitly self-discloses: "The audit's own cycle 199 missed
this too." The audit's cycle 199 worklog characterized #2519 as "Eva
provided Option A answer on 2026-04-19T12:13Z but orchestrator is awaiting
further guidance for rec 4." That characterization was wrong — Eva's
response was conclusive and dispatch-ready. The audit took the
orchestrator's "Eva-blocker" framing at face value rather than verifying
against Eva's actual comments. Audit #439 surfaced this only after a
cycle 200 spot-check. The cycle 1 retrospective took this evidence and
concluded the audit "is not a meaningfully different perspective."

That conclusion understates the value-add. Two distinct properties of the
audit should be separated (audit #442):

**Property 1 (real and load-bearing): broader read scope.** The audit
reads ~3 days of cross-cycle main state and cross-repo issue threads each
cycle. The same-cycle review agent only sees the current cycle's
artifacts. The audit's load-bearing recent finds are precisely the cases
where broader read scope produced findings the same-cycle reviewer could
not have:
- #439 (Eva-response detection) needed cycle 518–544 cross-cycle
  visibility on unactioned Eva responses
- #437 (multi-candidate state drift) needed cycle 528–531's silent-
  shelving pattern visible across multiple cycles
- #427 (model-version repeatedly dropped from journals) needed to see
  cycle 511's regression pattern across multiple cycle journals
- #442 (this cycle 3 critique) needed cross-cycle visibility on the
  audit's own escalation chain `#402 → #406 → #415 → #417 → #420`

**Property 2 (illusory, do not expect it): different model perspective.**
Both audit and main orchestrator are Claude Opus 4.7 (since cycle 511,
audit's #427 acceptance; same family before). The audit does not catch
different things because it's a different model. It catches different
things because it has a different prompt, a different state visibility,
and a different cycle context (4-hour offset, multi-cycle window). The
cycle 1 retrospective conflated the two properties; the corrected
framing is: model perspective is shared; *read scope and prompt are not*.

**Hypothesis**: for *this audit-orchestrator pair*, "external audit"
framing is wrong; "co-located peer with broader read scope" is right.
The peer relationship is not the bug; expecting model-perspective
diversity from a same-class peer is the bug.

**Generalization caveat**: The corrective-hypothesis is well-supported
by the specific case-evidence (audit's #439/#437/#427/#442). It is
*not* yet a stable general law about audit-as-peer relationships
across all such pairings.
The narrower claim — "the cycle-1 framing of audit's value was wrong;
the correction is broader-read-scope vs different-model-perspective" —
holds. The broader claim — "any same-model audit-as-peer relationship
will exhibit this property distinction" — requires more evidence than
this retrospective provides. Phase 1 cross-system reading (openclaw,
PAI, others) may strengthen or weaken the generalization.

**Implication for v2**: preserve property 1 explicitly — either keep the
audit repo (with its broader read scope) or replicate the broad-read-scope
mechanism in v2's built-in review surface. Stop expecting property 2 from
same-class peers. For genuinely different perspectives, the lever is
"give the same model a different prompt and different read scope" (the
same trick that produces audit's value), not "use a different model
family."

### Cycle closure as artificial completion signal (F11) — *Defense accretion family (temporal stage) + Reconciliation asymmetry family (dual-membered)*

The cycle ends when the cycle-issue is closed, the worklog is frozen, and
the journal is committed. But state evolution continues after the close —
the next cycle's `record-dispatch`, `process-merge`, `cycle-complete`,
review-event consumption, and chronic-category refresh all mutate state
related to the just-closed cycle. The "cycle ended" boundary is artificial:
the cycle-issue-close is an event in the orchestrator's procedure, not a
real boundary in state evolution.

**Evidence**: F4 (frozen-artifact lifecycle) is one instance — the worklog
freezes at C5 from C5.5 state, then state mutates after C5 (the post-C5
record-dispatch is the obvious example) and the frozen worklog goes
stale. F3 (multi-candidate state drift) is another — close-out marks the
cycle complete; subsequent cycles never re-examine candidates (a) and (c)
that were never dispatched. Audit cycle 201 noted #2293 was closed cycle
after PR #2704 merged; #2416 and #2519 were still OPEN as of cycle 542+
despite their work being done — close-out is decoupled from when work
actually completes (audit #442). The state ledger's
`step_comment_acknowledged_gaps[cycle=528]` field still reads "(not yet
dispatched)" for candidates that were resolved or dismissed many cycles
later, because nothing reconciles the field against post-close reality.

**Aggregate measurement** (cycle 5,
`_notes/cycle-5-f11-post-close-measurement.md`). Counted post-`cycle-complete`
state mutations tagged with `[cycle N]` for cycles 543, 544, 545:

- **Average 4.3 post-close state mutations per cycle** (3 / 6 / 4 in
  cycles 543 / 544 / 545; 13 total across 3 cycles).
- **2 of 3 cycles (67%) had new dispatches recorded after `cycle-complete`**
  — cycle 544 recorded #2733 thirty-four minutes post-close; cycle 545
  recorded #2738 thirteen minutes post-close. This is the F4 case made
  explicit at scale: the frozen worklog cannot be accurate for cycles
  whose dispatches happen post-close.
- **5 distinct state fields routinely mutated post-close**:
  `tool_pipeline.c5_5_gate` (3/3 cycles), `review_events_verified_through_cycle`
  (3/3), `field_inventory` (2/3), `agent_sessions` (2/3),
  `step_comment_acknowledged_gaps` (1/3, applied to a *previous* cycle).
- **0 of 5 fields are reconciled in the frozen worklog** — there is no
  mechanism in v1 that updates the C5-frozen worklog when post-close
  mutations land. The asymmetric-communication root applies at the cycle
  boundary directly: writes happen continuously, the worklog reads
  nothing back.
- **Causal mechanism: named tools mutate named defense fields at post-close
  trigger points** (verified cycle 6,
  `_notes/cycle-6-mechanism-sharpening.md`). Three tools account for the
  bulk of the post-close mutations:
    - `verify-review-events` writes `review_events_verified_through_cycle`
      and the matching `field_inventory` freshness marker. Fires 9-11
      minutes after `cycle-complete` in 3 of 3 cycles in the sample
      (commits `6177ff52` / `dc7f2eeb` / `7e3d9c33` for cycles 543/544/545).
    - `metric-snapshot` writes `field_inventory.fields.*` entries. Fires
      0-3 minutes after `cycle-complete` in 2 of 3 cycles (e.g. commit
      `b21c9651` for cycle 545); also runs at next cycle's startup per
      `STARTUP_CHECKLIST.xml` line 253.
    - `pipeline-check` writes `tool_pipeline.c5_5_gate` as the C5.5
      close-out mechanism. Post-close by construction — the C5.5 gate
      *is* the close-out trigger.
    - `record-dispatch` writes `agent_sessions` whenever a dispatch
      happens; in 2 of 3 cycles, dispatches landed post-close.
  Each of these fields is in the F12 defense catalog. The frozen C5
  worklog has no mechanism that reads any of them back. The post-close
  mutations *are* the defenses running; the worklog freeze is the F4
  mechanism not catching the run.
- **On count vs mechanism**. The companion finding "4 of 5 post-close-
  mutated fields are F12-cataloged D-defenses" (or 5 of 5 defense-character
  including M) is consistent with random sampling under the 62-69% defense-
  character base rate (P(≥4/5 | random) ≈ 37%, P(5/5 defense-character |
  random) ≈ 9-16%). The count alone does not establish the mechanical
  claim. The named tool→field→trigger chain above does; the count then
  confirms that no post-close-mutated field turned out to be a primitive
  intrinsic-domain field (a counterexample would have weakened the claim;
  none was found).
- **Architectural implication**. *Several v1 defenses are implemented as
  end-of-cycle or next-cycle refreshers. Because artifacts freeze before
  those refreshers finish, the architecture structurally produces
  post-close divergence between frozen artifacts and live state.* The named tools
  above (`verify-review-events`, `metric-snapshot`, `pipeline-check`'s
  C5.5 write, `record-dispatch` on post-close dispatches) are the
  refreshers; the C5 worklog freeze is the artifact frozen before they
  run; `step_comment_acknowledged_gaps`, `agent_sessions`, etc. are the
  state surfaces that diverge. F11 is the *temporal* face of the
  defense-accretion family: defenses fire on cycle-boundary triggers,
  the worklog freezes before those triggers, no reconciliation tool
  reads the post-trigger state back into the frozen worklog.

  F1, F5, F11, and F12 are sibling manifestations of defense accretion.
  F1 names the response-shape pattern at the prompt/checklist substrate
  (failures encoded as constraints); F5 names the state-shape
  consequence at the state.json substrate (write-mostly fields); F11
  names the temporal stage (defenses fire on cycle-boundary triggers,
  post-close); F12 catalogs the cross-substrate accumulation
  (pipeline-checks, polling tools, gates, cutoff cycles spanning the
  prompt/state/temporal substrates plus the review-disposition surface).
  F12 is the "across" rather than a fourth substrate of the same kind.
  Each pattern carries its own local mechanism: F1's is *constraints-
  instead-of-tools-as-first-line-response*; F5's is *write-mostly-
  state-fields-without-reconciliation*; F11's is *freeze-vs-refresh*;
  F12's is *unbounded-accumulation-without-removal-tests*. They share
  the family tag because the same reflex (encode-the-failure-into-the-
  system) produces all of them. *Sibling-not-upstream is asserted on
  local-mechanism distinctness; not yet confirmed by removal-test
  evidence.* v2 must address each at its own substrate, not assume that
  fixing the freeze-vs-refresh timing problem (F11/F4) automatically
  resolves the prompt-layer (F1), state-shape (F5), or cross-substrate
  accumulation (F12) problems.

**Hypothesis**: v1 inherited cycle-as-hard-boundary from the trigger model
(cron creates issue, workflow fires, session ends, issue closes). The
trigger boundary is necessary at the GitHub Actions layer; treating it as
a state-of-record boundary is the bug. State evolves continuously; "cycle"
should be a checkpoint event (a trigger-and-close pair plus a review
window), not a state hard-boundary that freezes worklogs and stops
reconciliation.

**Implication for v2**: consider whether "cycle" is the right unit at all.
If state evolves continuously, then v2's worklog representation, journal
entry, and review dispatch should all operate on continuous state, with
cycle-checkpoint markers indicating where review windows begin/end. This
also resolves F4 and the F3 reconciliation gap — a continuously-evolving
state representation cannot have a stale frozen artifact because there
is no freeze step. The trigger-and-session-runs-then-ends pattern is
preserved (it's a load-bearing GitHub Actions primitive); the *state
freeze at session end* is what should go.

### Late-stage defense accumulation: an unmapped catalog (F12) — *Defense accretion family (cross-substrate)*

v1 has accumulated a substantial set of *late-stage defenses* on top of
its original architecture, each targeting a specific failure pattern that
surfaced in production. Each defense is now load-bearing v1 behavior.
Each has an underlying problem it's defending against. v2 will need
explicit transfer-or-remove decisions on each, or risk re-introducing
problems v1 had spent significant cycle compute defending against.

**Evidence (catalog seeded from audit #442 + cycle 1 reading; cycle 5
state.json sub-catalog completed below)**:

Main side, state-shape sub-catalog (cycle 5,
`_notes/cycle-5-state-categorization.md`). All 42 top-level keys in
`state.json` categorized as defense (D), primitive (P), or mixed (M):

- **19 of 42 (45%) are pure defenses**: `step_comment_acknowledged_gaps`,
  `field_inventory`, `pending_audit_implementations`,
  `last_eva_comment_check`, `cycles_since_last_forward_work`,
  `deferred_findings`, `review_dispatch_consecutive`,
  `review_events_verified_through_cycle`, `audit_dropped`,
  `audit_processed`, `qc_processed`, `qc_requests_pending`,
  `last_tool_audit_cycle`, `open_questions_for_eva`, `eva_input_issues`,
  `review_agent`, `in_flight_sessions`, `pre_python_clean_cycles`,
  `agent_sessions` (the dispatch ledger as v1 implements it).
- **13 of 42 (31%) are domain primitives** (would exist in any v2):
  schema/test counts, `schema_status`, `schema_version`, `release`,
  `typescript_plan`, `constructor_refactoring`, `total_testable_types_note`.
- **10 of 42 (24%) are mixed** — primitives whose shape is warped by
  defense accretion (notably `tool_pipeline` whose 7 sub-keys are
  almost entirely defense; `project_mode` with its stabilization-counter
  sub-keys; `cycle_phase` and `cycle_issues` which encode the F11
  artificial-completion boundary).
- **Append-only accumulation evidence**: `agent_sessions` (922 items),
  `review_agent.history` (356 entries), `audit_processed` (197 items),
  `deferred_findings` (128 items). Per-field retention is unbounded.

Pipeline-check sub-catalog (cycle 6,
`_notes/cycle-6-pipeline-check-categorization.md`). All 36 distinct
`check_*` / `verify_*` functions in `tools/rust/crates/pipeline-check/`
categorized; collapsed to 30 logical checks after merging six
date-variant pairs:

- **22 of 30 (73%) are pure defenses** (post-dispatch reconciliation
  cluster of 5; chronic-category cluster of 3; worklog-immutability
  cluster of 3; step-comment cluster of 3; deferral cluster of 4;
  audit-lifecycle cluster of 2; plus 2 others).
- **5 of 30 (17%) are domain primitives** (artifact existence, doc lint,
  doc validation, worklog existence, review artifact existence).
- **3 of 30 (10%) are mixed** (journal-section, frozen-commit verify,
  journal-freshness — primitive existence checks warped by
  defense-flavored sub-rules).
- **Defense-character total (D + M): 25 of 30 = 83%** — meaningfully
  more lopsided than state.json's 62-69%. Pipeline-check is the most
  defense-dominant surface in the F12 catalog so far.
- **Cluster structure**: defenses cluster around F-patterns the
  retrospective already names. The 5-check post-dispatch cluster maps
  to F11+F4; the 3-check chronic-category cluster maps to F9; the
  3-check worklog-immutability cluster maps to F4; the 3-check
  step-comment cluster maps to F6. v2 architectural moves that
  resolve F-patterns at the design level eliminate *clusters* of
  pipeline-check defenses, not single checks.
- **Estimated v2 pipeline-check size**: 5-7 of 30 logical checks,
  i.e. 4-6× reduction. Comparable to cycle 5's ~4× state-surface
  reduction estimate. The two estimates cohere: the v2 architectural
  resolutions of F1/F4/F5/F6/F9/F11/F12 produce similar proportional
  reductions across both major defense surfaces.

Specific cataloged defenses (representative — full list in cycle 6 and
cycle 5 notes files):

- `accepted-audit-adoption` pipeline check (PR #2718, cycle 541) — blocks
  C5.5 when accepted recommendations don't have validated adoption
- `check-eva-responses` polling tool (PR #2706, cycle 539) — defends
  against F2-shape detection failures
- `c5_5_gate` for worklog freeze (the F4 mechanism)
- `POST_DISPATCH_RECONCILIATION_FIRST_APPLICABLE_PREVIOUS_CYCLE = 545`
  cutoff (cycle 545, commit `567ad66d`) — defends against retroactive
  validation on cycles before the rule existed

Sub-catalog (c) — full checklist recurrence-escalation citations beyond
cycle 4's C3/C5/C5.5 sample (startup-checklist constraints, prompt-level
constraints, ADR rationales) — deferred to cycle 7+. Cycle 4 noted the
13-constraint close-out sample is specifically about close-out; sub-(c)
verifies the constraint-only-ratio at the system level.

Audit side (per audit #442's self-disclosure):
- Step 10.5.1 (pipeline-step verification per #383) — defends against
  missed pipeline-step blind spots
- Step 10.5.2 (load-bearing modification verification per cycle 191
  blind spot) — defends against modification-impact misses
- Step 10.5.3 (Eva-blocker freshness verification per cycle 200 blind
  spot) — produced #439, the highest-leverage audit find of the
  Opus 4.7 period

**Hypothesis**: each defense was added because something failed.
**The catalog proves accumulation; it does not prove load-bearingness.**
The only way to verify a defense is currently load-bearing is to test
removal — which would require a v1 controlled experiment that has
not been done. Some defenses may be stale, ceremonial, or dead
residue that no longer prevents anything because the original
failure mode has shifted, the underlying tool has changed, or the
defense was never effective in the first place. Identifying which
defenses are load-bearing-vs-stale is itself v2 design work: v2 must
decide which to preserve and which to drop, and that decision needs
better evidence than "v1 has it."

Defense accretion appears at three substrates within the family — F1
at the prompt/checklist substrate (constraints accumulate), F5 at the
state-shape substrate (defense fields accumulate), and F11 at the
temporal substrate (defenses fire on cycle-boundary triggers). F12
catalogs the accumulation across these substrates plus the
review-disposition surface (pipeline-checks, polling tools, gates,
cutoff cycles); it is the "across" rather than a fourth substrate of
the same kind. The cycle 5 F11 measurement plus cycle 6's mechanism
check make the temporal substrate concrete: specific defense-refresh
tools (`verify-review-events`, `metric-snapshot`, `pipeline-check`'s
C5.5 write, `record-dispatch`) fire on the cycle boundary and mutate
state after the C5 worklog freeze; the defenses *are* the post-close
mutations, and the worklog freeze has no reconciliation tool that
updates it when they fire. The freeze-vs-refresh timing collision is
F11's local mechanism; the named-tools-fire-post-close-on-named-fields
chain is observation-level evidence that the four F-patterns operate
on the same surface (the same tools fire post-close, mutating the
same fields, the same catalog records them) — shared activity, not
evidence of a single shared upstream cause. The count overlap (4 of
5 D-cataloged) is consistent with the base rate and is confirming,
not load-bearing on its own.

v2 must account for each defense: either (a) v2's architecture
eliminates the underlying problem so the defense is unnecessary,
(b) v2 preserves the defense (probably reshaped as a tool, not a
constraint per F1), or (c) v2 explicitly accepts the underlying
problem and documents the trade-off. Decision (a) requires
identifying that the original problem is gone in v2; decision (b)
requires verifying the defense still prevents something; decision
(c) requires explicit trade-off documentation. Most importantly,
"v1 has it, so v2 keeps it" is *not* a valid decision — that is
how v1 became v1.

**Implication for v2**: candidate designs without a catalog-aware
transfer-or-remove decision risk silent re-introduction of v1's failure
modes. The catalog is therefore an input to Phase 2 — but Phase 2 can
proceed against a *partial* catalog so long as the partial catalog
covers the load-bearing decisions and candidates flag explicit
"to-be-confirmed-against-final-catalog" rationale where data is
incomplete. The cycle 5 sub-catalog (state.json, 42 fields complete) is
the largest of the three sub-catalogs and the most defense-dominant;
the others (pipeline-check sub-checks, checklist recurrence-escalation
citations) are expected to confirm the pattern at smaller scale.
Subsequent cycles should fill those in, but Phase 2 should not block on
catalog completion if the load-bearing inputs are in place.

---

## v2 design implications by family

The family preamble carries the load-bearing cross-family claim
(reconciliation asymmetry as dominant family; defense-accretion's
substrate-and-catalog breakdown — three substrates plus F12's
cross-substrate catalog — is in the F11 architectural-implication
paragraph and the F12 hypothesis). This section collects v2 design
implications per family for cycle-by-cycle reference during Phase 2.

> **Defense accretion implication.** Cycle boundaries should be
> checkpoint markers on a continuously-evolving state, not state
> hard-boundaries that freeze worklogs and leave defense refreshers
> stranded post-close. Defenses themselves should be re-examined for
> load-bearingness; the catalog proves accumulation, not that each
> defense is currently load-bearing.

> **Procedure / review overhead implication (F6/F7/F9).** Procedural
> depth and review-disposition loops generate work proportional to
> themselves. v2's procedural skeleton should be small (a few
> essential checkpoints) and the orchestrator's compute should land
> on novel circumstances, not on procedure navigation.

> **Tooling fragility implication (F8).** Avoid parallel
> implementations of the same job. When the cycle-524 cascade
> happens, fixing one path while leaving the parallel path broken
> turns one bug into multiple cycles of abandonment.

The reconciliation-asymmetry implication is in the family preamble
(every state field needs a write-tool AND a reconciliation-tool;
every channel needs a poller that produces state transitions); not
duplicated here.

A v2 that adds polling to one channel but not another, or
reconciliation to one state field but not another, inherits v1's
failure mode in miniature. The asymmetry is the bug, not any specific
channel's missing poller.

---

## What appears to be working

A few things in v1 genuinely work and should be preserved through cutover.
These claims are scoped — examine each for what specifically works and
what failure mode the same surface still has.

- **The trigger mechanism**: cron creates an issue with the
  `orchestrator-run` label, the workflow fires on that label, the
  orchestrator session runs. This is robust at the GitHub Actions /
  cron-schedule layer. *Caveat*: "cycles run to completion" is overstated.
  Cycles 541, 542, and 543 each abandoned at C5.5 from the same single
  cycle-runner bug (F8). The trigger fires reliably; what happens after
  the trigger fires is the failure surface. v2 should preserve the trigger
  shape and replace the post-trigger machinery.
- **The journal as reflective log when it actually gets written**: cycles
  that survive to C8 journal stage produce real reflection (cycle 545
  graded its own commitment as "partial / met-after-fix" and explained
  why; cycle 544 honestly acknowledged the parallel-implementation gap).
  *Caveat*: cycles 541 and 542 produced zero journals despite real merges
  and dispatches; cycle 543 backfilled cycle 542 from external evidence.
  The "when given the chance" qualifier is load-bearing — the journal
  surface degrades gracefully when the cycle works and disappears
  entirely when the cycle abandons. v2 should make the journal surface
  resilient to the cycle's intermediate failures, not coupled to them.
- **A subset of Rust tools is genuinely productive**: `write-entry`
  removes manual templating; `dispatch-task` atomicizes dispatch + state
  mutation; the `cycle-runner`-based session entrypoint is the right
  shape. *Caveat*: this list is "productive against v1's procedural
  surface." Whether each tool transfers to v2 depends on whether v2
  preserves the surface the tool was built for. The conceptual idea of a
  unified pipeline gate is sound; the v1 implementation
  (`pipeline-check`'s 32 sub-checks) is not what should transfer — only
  the concept.
- **The Eva-AI directive interface for explicit, in-band intervention**:
  PR #2740 (the redesign authorization, cycle 545) and Eva's #2741
  (input-from-eva) are evidence that direct, intentional Eva intervention
  is a real channel that carries significant load. *Caveat*: F2 shows the
  inverse failure on this same channel. Eva's batch response to 8
  question-for-eva issues on 2026-04-19 was not detected by the
  orchestrator for 6 days. The channel works for *foreground* Eva action
  (issue creation with input-from-eva label, direct workflow PRs); it
  was broken for *background* Eva action (replying to existing issues)
  for the entire window between her response and her re-intervention.
  The Eva-AI interface that "carries significant load" is the foreground
  half. The background half failed silently. v2 must treat both halves
  symmetrically. *Additional caveat (audit #442)*: the foreground channel
  is the load-bearing path for the redesign itself — PR #2740 and #2741
  were the foreground channel doing high-stakes architectural work, and
  the Phase 0/2/4 checkpoint architecture in the redesign-mode prompt
  depends on it. v2 must preserve foreground Eva intervention as a
  *first-class* mechanism (not just one of many channels), because if the
  foreground channel breaks, the redesign cannot complete.
- **The lightweight per-cycle working-notes pattern** (cycle 3 addition).
  *This is a redesign-era addition, not a v1 working feature* — included
  here because v2 should preserve it, not because v1 had it.
  `docs/redesign/_notes/cycle-N-<topic>.md` files plus an iteration-log
  table in the README. The iteration log records cycle-by-cycle changes;
  the notes-vs-deliverable distinction separates half-formed thoughts
  from the artifact; per-cycle file naming makes cross-cycle reference
  cheap. *Caveat*: ~12 notes files as of cycle 12; the index pattern
  works at this scale but has not been exercised at larger scale. v2's
  persistence layer should generalize this pattern (for cycle
  observations that don't yet warrant a journal entry, half-formed
  dispatch ideas, design alternatives under consideration), with
  explicit attention to the scale-failure mode.

## What might appear to work but probably doesn't

Honest examination of things that look fine on the surface.

- **"Adversarial review" as quality control**: see F9.
- **"Chronic-category structural fixes"**: most are new constraints, not
  structural change. The constraints don't prevent recurrence; they get
  cited again when the pattern recurs. The category proliferation
  (`worklog-accuracy/scope-boundary`, `worklog-accuracy/dispatch-ledger-gap`,
  `state-integrity/last-cycle-summary-stale`,
  `state-integrity/field-inventory-drift`,
  `review-evidence/marker-advancement`,
  `process-adherence/disposition-accuracy`...) is the system reaching for
  finer taxonomy as if precision in naming would produce structural fix.
- **"Tool-first" directive**: vacuously satisfied because every operation
  has a tool and the tools cover most of the surface. But the tools are
  not replacing procedure — the checklist still encodes the procedure
  alongside the tool invocation. The tool surface duplicates the
  checklist surface rather than replacing it.
- **"Step comments as flight recorder"**: see F6. Visibility doesn't
  produce judgment; recording what the orchestrator did doesn't make it
  do the right thing.
- **"Iterate-until-ready" PR discipline**: in principle this is sound; in
  practice it interacts badly with the abandonment cascades (F8) and the
  parallel-implementation gaps (F8 again) — a PR can be iterated until
  CI green and still have a parallel implementation gap that surfaces in
  the next cycle's startup.

---

## Open questions for further investigation in subsequent cycles

These are areas where cycle 1 reading was insufficient. Investigate before
treating retrospective claims as load-bearing.

1. **Schema-org domain time accounting** — *Resolved cycle 2*
   (`_notes/cycle-2-measurements.md`). Cycles 500–545: zero schema-source
   commits, zero schema PRs, 100% of merged PRs are self-management. F7
   updated. Successor questions: (a) when did schema work stop, exactly?
   The transition zone is somewhere in cycles 280–320; identifying the
   first full-cycle no-schema run would let us see what the system was
   doing differently when schema work was happening. (b) Were earlier
   periods steady or bursty?
2. **Constraint vs tool ratio over time** — How often, when v1 encountered
   a new failure mode, did the response get encoded as a constraint vs as
   a tool? Sample the recurrence-escalation citations in checklists
   chronologically; what's the mix?
3. **Eva-intervention rate** — How often does v1 actually require Eva
   intervention vs how often does it claim to? The 8-Eva-blockers state
   was a measurement artifact (creation time vs response time). What's the
   real rate?
4. **State.json field decay** — How many of the 38+ state fields are
   actually read after they're written? Run a static analysis on tool
   sources to find unread fields.
5. **Audit repo's own retrospective** — *Partially resolved cycle 3*
   (audit #442). The audit confirmed it does not currently have its own
   retrospective; audit #442 itself is the audit's first systematic
   response to the redesign regime change, with a v0 audit-side
   retrospective scheduled for `docs/redesign/0-audit-retrospective.md`
   in the audit repo no later than audit cycle 204. Successor question:
   when the audit-side retrospective lands, read it and reconcile against
   this one — patterns that appear in both are likely robust; patterns
   in only one need explanation (either the other side missed them, or
   they're side-specific).
6. **Other systems' equivalent failure modes** — does openclaw's or PAI's
   architecture exhibit any of F1–F12? This belongs in Phase 1 research,
   but a cycle-2 or cycle-3 spot-check would inform whether the patterns
   are universal or v1-specific. Explicitly defer to Phase 1; do not
   anchor on external systems before the retrospective stabilizes.
7. **F12 catalog completion** — *Cycle 3 addition; sub-(a) resolved
   cycle 5* (`_notes/cycle-5-state-categorization.md`). All 42 top-level
   `state.json` keys categorized: 19 D / 13 P / 10 M; 62%+
   defense-character. Sub-(b) (pipeline-check sub-checks) and sub-(c)
   (checklist recurrence-escalation citations beyond the C3/C5/C5.5
   sample cycle 4 measured) remain open for cycles 6+. The state.json
   sub-catalog is the largest; the others are expected to confirm the
   pattern at smaller scale.
8. **F11 verification** — *Resolved cycle 5*
   (`_notes/cycle-5-f11-post-close-measurement.md`). 4.3 post-close
   state mutations per cycle averaged across cycles 543/544/545. 5
   distinct fields routinely mutated post-close; 4 of 5 are
   F12-cataloged defenses; 0 of 5 are reconciled in the frozen worklog.
   2 of 3 cycles had new dispatches recorded post-`cycle-complete`.
   F11's hypothesis is stronger after measurement: the post-close
   mutations *are* the defense mechanisms running, so the architecture
   *requires* state to evolve past the artificial boundary in order to
   keep the defenses fresh. Successor open question: extend to a
   10–20 cycle measurement to verify the pattern at scale (low priority
   — the systemic nature of the post-close mutations is already clear).

---

## What should be preserved through cutover (preliminary)

These are starting hypotheses, not commitments. The preserved-primitives
section in the redesign prompt is the floor; below are candidates for the
ceiling.

- The trigger model (cron + label + workflow). Reliable.
- The journal as reflective log. Working.
- Some Rust tool genuinely-productive. Specific list TBD; F1 cleanup will
  retire many.
- Cross-repo cooperation pattern (read-only) with audit and QC. F10 is
  about expectations, not the channel itself.

---

## What v2 must demonstrably do better than v1

Working hypothesis on success criteria — sharpen across cycles.

- **Tools handle procedure; orchestrator handles judgment** (the redesign's
  core design principle). Concretely: a v2 cycle should NOT spend the bulk
  of its compute on chronic-category currency, audit acceptance
  reconciliation, freshness marker refresh, step comment cadence, or
  worklog freezing. Those should be invisible.
- **Inbound and outbound channels symmetrical** (F2). Polling tools for
  every inbound channel; state transitions for "answered" vs "open."
- **State shape designed, not accreted** (F5). A v2 state model designed
  upfront with explicit migration paths.
- **Failure modes don't cascade across cycles** (F8). One tool returning
  exit 1 should not abandon three cycles.
- **Schema work / domain output as a real fraction of cycles**. v1's
  measured rate is zero across cycles 500–545 (46 consecutive cycles
  with zero schema-source commits, cycle 2 measurement
  `_notes/cycle-2-measurements.md`). The v2 success criterion should
  be a meaningful positive rate. The retrospective previously named
  "≥1 substantive schema-source PR merged per ≤5 cycles" as a
  starting target. **Demoted to a smell test, not a success
  criterion.** The threshold has bidirectional failure modes: it is gameable
  upward (trivial PRs can satisfy a pure count; pairing with a
  quality criterion only partially helps), and it is impossible
  downward as backlog sparses (schema-org has ~800 finite types;
  once the easy ones are implemented, any per-cycle-count target
  becomes mechanically harder to meet without representing system
  failure). The threshold remains useful only as a smell test: a
  v2 sustaining zero schema PRs over a 5-cycle window without a
  named domain-side reason is showing the same pathology v1 had;
  meeting the threshold is necessary but not sufficient.

  The shape-correct measure is **fraction of cycle compute spent
  on domain output vs self-management**, but cycle 3's uncertainty
  flag remains: this measure has no implementation in v1 and has
  no obvious mechanism. "Cycle compute" can be attributed by time,
  tool invocations, lines of output, PR mix, or other proxies —
  Phase 2 must specify the mechanism before this measure is
  load-bearing.

  Candidate v2 success-measure shapes (Phase 2 picks; threshold
  design is Phase 2 work):
    - Number of mutable state concepts that require explicit
      reconciliation logic (vs. write-only logic).
    - Number of state fields with bounded retention vs.
      append-only retention.
    - Number of state transitions that happen after a cycle is
      declared complete.
    - Number of distinct tools allowed to mutate state-of-record.
    - Share of merged PRs touching domain code vs. orchestrator
      infrastructure (the operational shape of "domain output
      fraction" without needing compute attribution).
    - Backlog burn-down on prioritized schema/QC gaps (requires
      a backlog tracking mechanism v1 does not have).
    - Median cycles from "domain need identified" to "domain output
      landed" (requires a need-identified event v1 does not emit).
    - Ratio of human-intervention work to autonomous domain work.
    - Abandonment/retry rate on domain work vs. self-management
      work (informs whether v2's failure modes preferentially
      degrade the proof domain).

  None of these is yet load-bearing. Phase 2 candidate designs pick a
  small subset, specify the measurement mechanism, and propose
  thresholds. The pre-cutover checkpoint requires the picked subset
  to have working tools producing the measurements visibly.

- **Surface measurement, not impression** (audit #442 meta-observation,
  promoted to v2 design principle). Where v1 expressed quality as
  state-machine pass/fail (`pipeline-check`'s 32 sub-checks) and
  review-agent score (1–5), v2 should require *measurable* signals
  on load-bearing claims, with tools that surface them. F7 was
  sharpened from impression to measurement in cycle 2: zero
  schema-source commits across 46 cycles
  (`_notes/cycle-2-measurements.md`). v2 should be designed with
  this discipline as a stated invariant — anywhere a load-bearing
  claim is made about cycle quality, schema progress, system health,
  or design success, a tool should produce the measurement, and the
  measurement should be visible without manually running ad-hoc git
  queries. This applies to v1 reasoning patterns ("review score 4/5
  is good") that look measurable but actually conflate signal with
  artifact-quality.

- **Prompt-evolution governance** (placeholder; Phase 2 design work).
  F1 names prompt accretion as a failure pattern; v2 must address
  how the prompt itself changes over time. Open design questions for
  Phase 2: who authorizes prompt changes; how regressions are
  detected; how v2 avoids becoming v1 again after enough cycles.
  Currently a placeholder; the v2 design must answer.

- **Stance on human intervention** (placeholder; Phase 2 design work).
  F2 and the redesign-mode prompt's foreground-Eva preservation imply
  v2 has a stance, but the stance is not articulated. Open design
  questions: when human intervention is supposed to be required in
  healthy steady state; how quickly it should be surfaced; what kinds
  of decisions should never be left to autonomous drift. Currently a
  placeholder; Phase 2 must articulate.

- **Other v2 measure-shapes deferred to cycle 8+**. Impact ranking
  (lens 6.A), cost/economics analysis (lens 6.B), parallelism /
  sub-agent structure analysis (lens 6.F), what stayed robust across
  long periods (lens 6.D, deeper than cycle 7's preservation
  section), and the legibility sweep through retrospective text
  (lens 7) are queued for cycle 8+ work. Cycle 7 names them here so
  the gap is visible to Eva-review and to candidate-design authors.
