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

**Hypothesis** (corrected from cycle 1's framing per audit #442): the
review agent is gpt-5.x-class (Copilot); the orchestrator is Opus 4.7. So
"same model class" is the wrong shared variable. The actual shared
variables that produce blind-spot convergence are **prompt + state
visibility + cycle context**: the review agent reads the same prompt
surface as it always does, sees the orchestrator's same-cycle state, and
operates on a one-cycle window. Findings recur because *that* combination
is constant across cycles, not because the model is. The
review-and-disposition loop generates work proportional to itself; in
steady state most cycle compute is spent on the loop's own outputs.

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

### F10. Audit's value is broader read scope, not different model perspective

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

**Hypothesis**: "external audit" framing is wrong; "co-located peer with
broader read scope" is right. The peer relationship is not the bug;
expecting model-perspective diversity from a same-class peer is the bug.

**Implication for v2**: preserve property 1 explicitly — either keep the
audit repo (with its broader read scope) or replicate the broad-read-scope
mechanism in v2's built-in review surface. Stop expecting property 2 from
same-class peers. For genuinely different perspectives, the lever is
"give the same model a different prompt and different read scope" (the
same trick that produces audit's value), not "use a different model
family."

### F11. Cycle closure as artificial completion signal

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

### F12. Late-stage defense accumulation: an unmapped catalog

v1 has accumulated a substantial set of *late-stage defenses* on top of
its original architecture, each targeting a specific failure pattern that
surfaced in production. Each defense is now load-bearing v1 behavior.
Each has an underlying problem it's defending against. v2 will need
explicit transfer-or-remove decisions on each, or risk re-introducing
problems v1 had spent significant cycle compute defending against.

**Evidence (catalog seeded from audit #442 + cycle 1 reading;
incomplete — to be filled across subsequent cycles)**:

Main side:
- `step_comment_acknowledged_gaps` field — defends against silent missed
  step comments
- `chronic_category_responses` field — defends against chronic-category
  refresh drift
- `audit_acceptance_registry` field — defends against missed audit
  acceptances
- `field_inventory` field — defends against state field freshness drift
- `review_agent.history` field — defends against review-finding
  disposition drift
- `accepted-audit-adoption` pipeline check (PR #2718, cycle 541) — blocks
  C5.5 when accepted recommendations don't have validated adoption
- `check-eva-responses` polling tool (PR #2706, cycle 539) — defends
  against F2-shape detection failures
- `c5_5_gate` for worklog freeze
- `POST_DISPATCH_RECONCILIATION_FIRST_APPLICABLE_PREVIOUS_CYCLE = 545`
  cutoff (cycle 545, commit `567ad66d`) — defends against retroactive
  validation on cycles before the rule existed

Audit side (per audit #442's self-disclosure):
- Step 10.5.1 (pipeline-step verification per #383) — defends against
  missed pipeline-step blind spots
- Step 10.5.2 (load-bearing modification verification per cycle 191
  blind spot) — defends against modification-impact misses
- Step 10.5.3 (Eva-blocker freshness verification per cycle 200 blind
  spot) — produced #439, the highest-leverage audit find of the
  Opus 4.7 period

**Hypothesis**: each defense is load-bearing now and was added because
something failed. The catalog is itself the procedural-residue of v1's
operational history. Compare to F1 (constraint accretion) and F5 (state-
shape-as-fossil-record): F12 is the same mechanism viewed at a higher
level — failure mode → defense → defense becomes load-bearing. v2 must
account for each defense: either (a) v2's architecture eliminates the
underlying problem so the defense is unnecessary, (b) v2 preserves the
defense (probably reshaped as a tool, not a constraint per F1), or (c)
v2 explicitly accepts the underlying problem and documents the trade-off.

**Implication for v2**: this catalog is a load-bearing input to Phase 2
candidate-selection. Without it, candidate designs risk silent
re-introduction of v1's failure modes. Phase 2 design candidates should
each demonstrate a transfer-or-remove decision for every catalog entry.
F12 is the *placeholder* for the catalog; the full version is multi-
cycle work. Subsequent cycles should fill in detail per entry: failure
mode addressed, current mechanism, v2 transfer-or-remove decision
rationale. This work belongs in Phase 0 — without it, Phase 2 cannot
properly evaluate candidates.

---

## Shared root: asymmetric communication and write-mostly state

F2 (Eva-response detection), F3 (multi-candidate state drift), F4
(frozen-artifact lifecycle, in its post-C5 mutation aspect), and F11
(cycle closure as artificial completion) share a common root, named
explicitly here per audit #442. F8 is *adjacent* to this root rather
than centered on it (cycle 4 adversarial re-read,
`_notes/cycle-4-adversarial-reread.md`): F8's primary failure is
parallel-implementation duplication (`cycle-runner` was fixed,
`cycle-start::gather_pipeline_status` was not), and the
"asymmetric-fix-propagation" framing applies only as a secondary
reading. F8's main implication ("fewer tools doing each job") stands
independently of the asymmetric-communication root and should not be
folded into it.

> **Asymmetric communication / write-mostly state.** Outbound channels
> have well-developed tools (`question-for-eva` filing, `audit-outbound`
> filing, `dispatch-task`, `record-dispatch`, etc.). Inbound reconciliation
> does not. Once a record is written (an Eva blocker, a multi-candidate
> acceptance, a dispatch ledger, a frozen worklog, a closed cycle), no
> tool reads it back to update its state when subsequent events change
> its meaning. v1 has tools to *create* records and tools to *summarize*
> records; it lacks tools to *reconcile* records against later events.

The unification matters because v2's design implication is one-shaped:

> **Every state field needs a write-tool AND a reconciliation-tool. Every
> channel needs a poller that produces state transitions.**

A v2 that adds polling to one channel but not another, or reconciliation
to one state field but not another, will inherit v1's failure mode in
miniature. The asymmetry is the bug, not any specific channel's missing
poller. The cycle 2 success-criterion section already named this for the
F2 case; F11 makes clear it's a system-wide property, not a single-
channel fix.

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
- **The lightweight per-cycle working-notes pattern** (cycle 3 addition,
  audit #442 observation): `docs/redesign/_notes/cycle-N-<topic>.md`
  files plus an iteration-log table in the README is producing emergent
  behavior — the iteration log forces cycle-by-cycle accountability for
  what changed, the notes-vs-deliverable distinction provides a natural
  place for half-formed thoughts, and the per-cycle file naming makes
  cross-cycle reference cheap. Audit #442 calls it "the kind of
  light-weight scaffolding that delivers value disproportionate to its
  design cost." *Caveat*: this is a two-cycle observation; the pattern
  may degrade once notes accumulate beyond ~10 files (an index helps but
  is not yet exercised at scale). v2's persistence layer should
  generalize this pattern — for cycle observations that don't yet warrant
  a journal entry, half-formed dispatch ideas, design alternatives under
  consideration — but design with the scale failure mode in mind.

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
7. **F12 catalog completion** — *Cycle 3 addition*. F12 introduces a
   late-stage-defense catalog seeded from audit #442, but the catalog
   is incomplete. Subsequent cycles should fill in: (a) every state field
   currently in `state.json` (38+ top-level), categorize as defense-vs-
   primitive; (b) every pipeline-check sub-check (~32), categorize
   similarly; (c) every recurrence-escalation citation in the
   checklists. Each entry: failure mode addressed, current mechanism,
   v2 transfer-or-remove decision. This work is multi-cycle; do not
   attempt in one cycle.
8. **F11 verification** — *Cycle 3 addition*. F11 hypothesizes that the
   cycle-issue-close boundary is the bug. The hypothesis would be
   stronger with a measurement: how many state fields are mutated by
   the post-close machinery in the next-cycle's startup, and which are
   the corresponding fields that should have been reconciled but
   weren't? Cheap measurement; defer to a future cycle.

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
  measured rate is zero (cycles 500–545). The v2 success criterion should
  be a meaningful positive rate. Audit #442 named two failure modes the
  criterion must avoid: (a) easily gamed by trivial commits — a
  one-property addition or a typo fix could satisfy a pure-count threshold
  without representing meaningful schema work; pair any count threshold
  with a quality criterion (the schema work must address an item from a
  prioritized backlog or close a QC-flagged gap, so satisfying the
  threshold requires the same prioritization process as doing real work);
  (b) schema-org has finite types (~800 in core), so any pure-count
  measure degrades as easy types get implemented and the steady-state
  assumption breaks. The long-term measure is **fraction of cycle compute
  spent on domain output vs self-management**, where "domain output"
  includes schema work, QC validation, end-user docs, and API consumer-
  side improvements (the things that make the library useful). A working
  starting target until the better measure is built: ≥1 substantive
  schema-source PR merged per ≤5 cycles, paired with the quality criterion
  above. The pre-cutover checkpoint should require an explicit, measurable
  target in v2's prompt or state representation, with a tool to compute
  and surface the running rate. The exact form of the long-term measure
  is open and worth Phase 2 candidate-design exploration — not "pick a
  number," but "design a metric that resists drift into the self-
  management attractor under any sufficiently complex workload."
- **Surface measurement, not impression** (audit #442 meta-observation,
  promoted to v2 design principle). Where v1 expressed quality as
  state-machine pass/fail (`pipeline-check`'s 32 sub-checks) and review-
  agent score (1–5), v2 should require *measurable* signals on load-
  bearing claims, with tools that surface them. The cycle 2 F7
  measurement (zero schema-source commits over 46 cycles) is what this
  looks like in practice: an impression ("small minority") replaced by a
  numeric finding from a one-query measurement. The cycle 1 retrospective
  itself softened F7 because it lacked a measurement; the discipline
  matters even for adversarial-by-design artifacts. v2 should be designed
  with this discipline as a stated invariant — anywhere a load-bearing
  claim is made about cycle quality, schema progress, system health, or
  design success, a tool should produce the measurement, and the
  measurement should be visible without having to manually run ad-hoc
  git queries. This applies to v1 reasoning patterns ("review score 4/5
  is good") that look measurable but actually conflate signal with
  artifact-quality.

---

## Iteration plan for this artifact

This document is incomplete by design. Subsequent cycles should:

- **Sharpen each F-pattern** — sample more cycles for evidence; verify
  hypotheses; reject hypotheses that don't survive scrutiny. Cycle 2 did
  this for F7 (measurement); cycle 3 did this for F9/F10 (hypothesis
  correction per audit #442).
- **Add new F-patterns** as they're found through deeper reading or
  external critique. Cycle 3 added F11 and F12 from audit #442.
- **Solicit critique** — audit critique landed unprompted via #442
  (cycle 3); a Copilot feedback-only dispatch is a candidate for cycle
  4 or 5 to get a different prompt-and-context lens. Continue iterating
  the artifact between critique rounds; do not idle awaiting the next
  one.
- **Quantify** the open questions section. Replace impression-based
  claims with measured ones. Open questions 7 (F12 catalog) and 8 (F11
  verification) were added cycle 3 and need work.
- **Demote what doesn't survive** — patterns that look weaker on
  re-examination should be cut or qualified, not protected. Cycle 3
  corrected F9's hypothesis (model-class explanation was wrong) and
  F10's value-add framing.
- **Reconcile against audit's own retrospective when it lands** (audit
  cycle 204 or sooner). Patterns appearing in both are likely robust;
  patterns in only one need explanation.

The artifact is "ready for the post-retrospective checkpoint" only when
Eva says it is.
