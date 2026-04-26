# Phase 0: Critical retrospective on v1

**Status**: working draft — iterates across many cycles. Eva approval required
before Phase 1. While awaiting approval each cycle should sharpen this artifact.

**Reading guide**: this is not a self-flagellation exercise and not a status
report. It is the source-of-truth catalog of what v1 actually is and where it
fails, written so a future cycle (or candidate-design author, or Eva, or the
audit orchestrator) can ground design decisions in evidence rather than
sentiment. Cite cycle numbers; quote constraints by id; link issues.

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
exists to serve — is now a small minority of what each cycle does. Cycle 544
(2026-04-26, the cycle directly before the redesign-mode PR landed) consumed
8 Eva responses, refreshed 6 chronic categories, dispatched 2 PRs about
dispatch tooling and worklog reconciliation. No schema work touched the cycle.
Cycle 543 was a cycle-runner bug fix. Cycle 545 was a hotfix on the cycle-544
work. This is the modal shape.

---

## Failure patterns

These are concrete, evidenced patterns. Each gets a name, a description,
specific cited examples, and a hypothesis about its root cause. The hypotheses
are working drafts; sharpen across cycles.

### F1. Constraint accretion as first-line response

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

### F2. The Eva-response detection failure

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

### F3. Multi-candidate state drift

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

### F4. The frozen-artifact lifecycle is fragile

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

### F5. State.json as procedural-implementation-leak

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

**Hypothesis**: the state shape encodes the failures, not the system. Each
field is a scar. v2 should start from a much smaller state model that is
explicitly designed (cycles, dispatches, artifacts, reviews) — not grown
by accretion.

### F6. Cyclomatic procedure depth

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

### F7. Self-management dominance over schema work

The cycle 544 journal entry calls cycle 544 "the first cycle to fully
consume Eva's response queue without leaving any answered question open or
unactioned." It is described as "the cleanest in 4 cycles." The cycle
worked through 8 Eva responses, refreshed 6 chronic categories, dispatched
2 self-management-tooling PRs. Zero schema-org work happened. The
preceding cycles 541–543 abandoned at C5.5 from a single cycle-runner bug
and produced no journal entries. Cycle 545 dispatched a self-management
fix and hotfixed a self-management PR.

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
none have natural completion conditions. v2 needs design pressure away
from this attractor.

### F8. Abandonment cascades from single-tool defects

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
different ways, with tests against one and not the other. v2 should have
fewer tools doing each job, with explicit failure handling that does not
cascade across cycles.

### F9. Adversarial-review-and-disposition treadmill

Each cycle dispatches a review agent (Copilot, gpt-5.4-class). The review
finds findings. The orchestrator dispositions them (`actioned`,
`dispatch_created`, `deferred`, etc.). Dispositions become state. State
drives next cycle's review. Findings recur. The system has 6+ "chronic
categories" — those with 5+ of the last 6 reviews citing them. Chronic
categories require structural fixes; structural fixes are usually new
constraints in the prompt or new state fields; the new constraints get
violated; recurrence escalation citations accumulate.

**Hypothesis**: the review agent is the same model class as the
orchestrator. Its critique converges on familiar patterns. Fresh blind
spots aren't caught — the audit repo (a different orchestrator session
with broader read scope) catches things the review agent misses (e.g., the
Eva-response detection failure and the multi-candidate state drift were
both audit-repo finds, not review-agent finds). The review-and-disposition
loop generates work proportional to itself; in a steady state most cycle
compute is spent on the loop's own outputs.

**Implication for v2**: review or critique is genuinely valuable, but
same-model self-review may not be. The audit-repo critic catches more.
Copilot feedback (a different model family) might catch different things.
v2 should design the critique surface around getting genuinely independent
perspectives, not around reflexive self-review.

### F10. The audit repo is not as independent as it appears

Audit #439 explicitly self-discloses: "The audit's own cycle 199 missed
this too." The audit's cycle 199 worklog characterized #2519 as "Eva
provided Option A answer on 2026-04-19T12:13Z but orchestrator is awaiting
further guidance for rec 4." That characterization was wrong — Eva's
response was conclusive and dispatch-ready. The audit took the
orchestrator's "Eva-blocker" framing at face value rather than verifying
against Eva's actual comments. Audit #439 surfaced this only after a
cycle 200 spot-check.

The audit is genuinely useful (439, 437, 435 are all real, non-trivial
finds). But it is not a meaningfully different perspective; it is a
co-located peer that shares many failure modes with the main orchestrator.
v2 should treat audit feedback as a useful but not adversarial signal —
useful peer review, not external audit.

---

## What appears to be working

A few things in v1 genuinely work and should be preserved through cutover.

- **The trigger model**: cron creates an issue, workflow fires on the
  issue label, the orchestrator session runs to completion or timeout.
  This is robust. Cycles run reliably.
- **The journal as reflective log**: when the cycle survives to write a
  journal, the journal contains genuine reflection (cycle 545 graded its
  own commitment as "partial / met-after-fix" and explained why; cycle 544
  honestly acknowledged the parallel-implementation gap). The reflective
  habit produces value when given the chance.
- **Some Rust tools genuinely productive**: `write-entry` removes
  meaningful manual work; `dispatch-task` atomicizes dispatch + state
  mutation; `pipeline-check`'s structural drift is a v1 problem, but
  conceptually a unified pipeline gate is sound.
- **Eva-AI interface**: when Eva intervenes, it lands. The redesign-mode
  authorization itself (PR #2740, cycle 545) is evidence that the
  human-AI directive interface can carry significant load.

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

1. **Schema-org domain time accounting** — Cycle-1 reading of recent
   journals suggests schema work has been a small minority recently. Quantify:
   over cycles 500–545, what fraction of cycles had any schema commit
   (PHP or TS) vs were entirely self-management? Do not rely on impression.
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
5. **Audit repo's own retrospective** — does the audit orchestrator have
   its own equivalent of this retrospective? If so, read it. If not,
   propose one (cross-repo posting, async).
6. **Other systems' equivalent failure modes** — does openclaw's or PAI's
   architecture exhibit any of F1–F10? This belongs in Phase 1 research,
   but a cycle-2 spot-check would inform whether the patterns are universal
   or v1-specific.

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
- **Schema work as a real fraction of cycles**. v2 should produce schema
  commits in most cycles, not as the exception. Quantify the target.

---

## Iteration plan for this artifact

This document is incomplete by design. Subsequent cycles should:

- **Sharpen each F-pattern** — sample more cycles for evidence; verify
  hypotheses; reject hypotheses that don't survive scrutiny.
- **Add new F-patterns** as they're found through deeper reading.
- **Solicit critique** — file a request in this repo for the audit
  orchestrator to critique this retrospective; dispatch Copilot
  feedback-only sessions for outside-AI perspective; let multiple lenses
  converge or diverge.
- **Quantify** the open questions section. Replace impression-based claims
  with measured ones.
- **Demote what doesn't survive** — patterns that look weaker on
  re-examination should be cut or qualified, not protected.

The artifact is "ready for the post-retrospective checkpoint" only when
Eva says it is.
