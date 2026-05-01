# Cycle 38 (2026-05-01) — Cold-reader on cycle-37 v1.2 (3/3 PASS) + v1.3 application + re-dispatch refresh on Cognition Devin and OpenAI harness

## Setup

Cold-start session. Cron fired 2026-05-01 03:00 UTC (issue #2799). Second
cycle of 2026-05-01 (cycle 37 was first at 00:19 UTC, issue #2798).

Cycle 37 (commit `2ff0982e`) left a clear plan with three workstreams:

1. **Substantive focal:** cross-cycle cold-reader on cycle-37 framework
   v1.2 with three pre-commit questions.
2. **Substantive parallel:** re-dispatch Cognition Devin (#2779/#2780)
   and OpenAI harness (#2781/#2783) per Eva #2794.
3. **Bounded mechanical:** backfill Maps-to lines on Axes 1, 3, 5, 6, 7
   per cycle-37 same-cycle review minor finding (5).

All three workstreams executed this cycle.

## Cold-reader on cycle-37 framework v1.2

Three pre-commit questions per cycle-37's checklist.

### Question (a) — Is F11 → Axis 4 + Axis 12 (no Axis 2) the right call, or did cycle-37 over-correct cycle-36?

**Spot-check:** does Axis 12 (reconciliation) actually address F11's
load-bearing failure (worklog freeze without refresh) or is it stretching?

**VERDICT: PASS.**

**Analysis.** F11's mechanism (per `0-retrospective.md` lines 668-740):

1. Tools fire post-close at trigger points (`record-dispatch`,
   `metric-snapshot`, `verify-review-events`, `pipeline-check`).
2. Each mutates F12-cataloged `state.json` fields.
3. The C5-frozen worklog has no mechanism reading any of these back.
4. Post-close mutations *are* the defenses running; the worklog freeze
   is the F4 mechanism not catching the run.

The retrospective explicitly names three architectural fixes (lines
745-749): "earlier defenses (if their inputs allow), later freeze (if
the worklog can remain mutable until refreshers complete), or continuous
reconciliation without a hard freeze would each resolve it."

- Resolution 1 (earlier defenses) — partial; doesn't address
  `record-dispatch` on post-close dispatches.
- Resolution 2 (later freeze) — Axis 4 (history substrate semantics for
  the worklog artifact) + Axis 12 (someone reads state into the mutable
  worklog).
- Resolution 3 (continuous reconciliation without hard freeze) — Axis 12
  directly.

Axis 12's framework description explicitly mentions "post-close tool
mutations" as one of the inbound channels to reconcile. F11 is exactly
post-close tool mutations not being reconciled into the frozen worklog.
Direct match, not stretching.

**Counterfactual check.** If we had ONLY Axis 4 (append-only history)
without Axis 12 (no reconciliation): all post-close mutations would be
in append-only history, but the C5-frozen worklog still has no mechanism
reading from that history. F11's worklog-staleness face persists.

If we had ONLY Axis 12 (reconciliation) without Axis 4 (still destructive
writes): reconciliation detects state changes and updates worklog, but
each destructive write loses prior values. The reconciled worklog would
just reflect "current state of state.json" — it would see the latest
value, not the lost history. F11's data-loss face persists.

Both axes are load-bearing; neither alone fixes F11. Cycle-37's mapping
F11 → Axis 4 + Axis 12 stands.

### Question (b) — Does Axis 13's "thin / medium / fat" trichotomy actually constitute candidate-differentiation?

**Spot-check:** name the difference between medium-harness and fat-
harness in concrete procedural-responsibility terms.

**VERDICT: PASS with empirical-verification flag for cycle-39+.**

**Analysis.** Concrete differentiation between medium and fat is
nameable:

- **Medium harness candidate:** cycle-runner handles cycle-phase
  enforcement (when to read state, when to plan, when to execute, when
  to review, when to write). Within each phase, session does the work.
  The session prompt has phase-specific sections like "you are in the
  planning phase" with prose about what planning means.
- **Fat harness candidate:** cycle-runner handles cycle-phase
  enforcement PLUS routine procedures (dispatch protocol, mechanical CI,
  state reading, journal writing, cycle-issue close). Session is invoked
  only for novel decisions. The session prompt is minimal.

Different real candidates would plausibly take different positions:

- Cognition-inspired candidate: medium-with-strong-context (long context
  window, agent has all state in context, harness handles phases).
- Devin-architecture-inspired candidate: structured pipeline with
  planning agent, execution agent, reviewer agent — fat (each agent is
  a deterministic invoker).
- LangGraph-inspired candidate: typed state graph with agent nodes —
  fat (graph topology is deterministic).

The trichotomy is real differentiation.

**Empirical-verification flag for cycle-39+:** the cycle-37 same-cycle
review's mild concern was that "medium might collapse to fat in
practice." This is empirically unverified — Phase 2 candidate generation
will reveal whether candidates actually distribute across medium and
fat or cluster on one position. The framework's role is to LET
candidates differentiate; whether they DO is a Phase 2 observation.
PASS for now; Phase 2 verifies.

### Question (c) — Is constraint 8 (goal-driven over operator-driven) genuinely a constraint or a tautology?

**Spot-check:** try to falsify by imagining a v2 candidate that takes
operator-driven at the top level. Does the constraint actually REJECT
that candidate?

**VERDICT: PASS.**

**Analysis.** What would an operator-driven top-level v2 candidate look
like? Cycle-runner would NOT autonomously fire on a cron schedule and
decide what to work on. Instead, it would wait for explicit operator
command (from Eva, presumably) for each piece of work. Each cycle would
be triggered by Eva specifying "now do X."

This candidate IS rejected by the redesign's primary thesis. The thesis
commits to autonomous AI; an operator-driven top-level posture means
EVERY cycle requires explicit operator command. That's not "autonomous
self-healing AI"; that's "AI tool waiting for instructions." The
constraint genuinely excludes this candidate space.

**Counterfactual check on similar constraints.** Could "no candidate
would violate" make this vacuous? Constraint 3 (strong-defaults
security) and constraint 6 (small core, capability extends via
something) are similarly things no v2 candidate would violate
deliberately. By that test, those constraints would also be vacuous.

But the framework's purpose for constraints (per the Purpose-and-scope
section) is: "Every v2 candidate must honor these or explicitly disagree
with load-bearing rationale. A candidate that violates a convergent
constraint is a candidate that disagrees with all surveyed systems'
converged practice — that disagreement should be deliberate, not
accidental."

So constraints serve as REQUIRED-TO-HONOR list. A constraint that no
candidate would violate is still useful: it documents what the framework
DOESN'T let a candidate get away with, forcing the question to be
addressed (every candidate must say "yes I honor this") rather than
left implicit.

In that framing, constraint 8 is not vacuous. It documents
"goal-driven over operator-driven" as a fixed mission commitment.
Candidates can't accidentally drift to operator-driven without the
constraint catching it.

**Sub-clause check.** The constraint says "Operator-driven sub-systems
may exist within a goal-driven overall posture." This is permissive
language ("may exist"), not constraining language. Every v2 candidate
will have at least input-from-eva (operator-driven sub-system). The
sub-clause permits something candidates would have anyway. Its purpose
is clarification (heads off misreading "goal-driven overall" as "no
operator inputs"), not constraint. Useful framing, not load-bearing
exclusion.

**VERDICT:** main statement is meaningful (rejects all-operator
candidates); sub-clause is permissive clarification. Constraint 8 is
not a tautology.

### Cold-reader summary

- Q(a) **PASS** — F11 → Axis 4 + Axis 12 stands; Axis 12 directly
  addresses worklog-staleness-without-refresh.
- Q(b) **PASS** with empirical-verification flag — Axis 13 medium-vs-fat
  is concretely differentiable; Phase 2 candidate generation will verify
  whether candidates actually use both positions.
- Q(c) **PASS** — constraint 8 is meaningful; operator-driven top-level
  candidates ARE rejected by mission commitment.

Three PASSes is a notable cycle outcome. Cycles 35, 36, 37 each found
substantive corrections in the prior cycle's work. Cycle 38 finds no
substantive correction — only refinements (cross-axis dep + Maps-to
backfill).

### Two refinements found beyond the three Qs

**Refinement 1 (apply as v1.3):** Cross-axis deps map missing
"Axis 13 × Axis 7" entry. F9's fix is primarily Axis 7 (multi-pattern
situational-review breaks the every-cycle review-firing loop), with
Axis 13 (harness-vs-session) shaping the implementation strategy
(fat-harness controls when review fires; thin/medium leaves WHEN-review
in prompt). The cross-axis deps map should document this relationship.

Applied as v1.3 inline.

**Refinement 2 (flag for cycle-39+):** Axis 12 "v1-derived" caveat may
be too strong. The framework says "no external system surveyed has an
Eva-equivalent that would constrain the choice." This is true for the
SPECIFIC Eva-checkpoint pattern, but the BROADER axis (reconciliation
of inbound external events into state) has external analogues:

- LangGraph interrupts — node-level interrupt + checkpoint/resume; HITL
  primitive.
- AutoGen human-in-the-loop primitives.
- Cognition Devin user-injected guidance (per cycle-26 secondary-source
  reads; primary-source verification pending).

The general axis (reconciliation discipline for inbound channels) has
partial external precedent even though no surveyed system has an
Eva-equivalent specifically. Refinement requires cross-system
verification (LangGraph + AutoGen HITL primitives) before applying.

Flagged for cycle-39+ verification, not applied this cycle.

## Framework v1.3 application

Created v1.3 with two changes:

### 1. Cross-axis dependency added: Axis 13 × Axis 7

```
- **Axis 13 (harness-vs-session) × Axis 7 (orchestration topology):** Fat-
  harness can implement Axis 7's multi-pattern situational-review by
  controlling when review fires (vs every cycle). Thin/medium harness leaves
  WHEN-review decisions in prompt, where the v1 anti-pattern (every-cycle
  review-firing) tends to recur. F9 (adversarial-review treadmill) is
  primarily fixed by Axis 7 (situational vs fixed); Axis 13 shapes the
  implementation strategy for that fix.
```

### 2. Maps-to backfill on Axes 1, 3, 5, 6, 7

Resolves cycle-37 same-cycle review minor finding (5). All 12 axes now
have Maps-to lines:

| Axis | Maps-to (added in v1.3) | Notes |
|---|---|---|
| 1 | F7 (load-bearing); F9 (indirect) | Role-specialization reduces self-management surface; dedicated-reviewer-role addresses F9 indirectly |
| 3 | Constraint 7 (shape-choice); F7 (indirect) | Memory shape choice within constraint 7; rich memory reduces re-derivation cost (F7 indirect) |
| 5 | F4 | Plan lifecycle primitives address freeze/refresh timing |
| 6 | Constraint 6 (shape-choice) | Extension mechanism choice within constraint 6 |
| 7 | F6, F9 | Multi-pattern lighter than rigid checklist (F6); situational invocation breaks review-firing loop (F9) |

Consistency check against F-pattern table: all axis additions are
consistent with axes already mentioned in the F-pattern table (no new
F-pattern→axis mappings introduced; only filling Maps-to lines that
were missing on per-axis sections).

### 3. Iteration history table updated

v1.3 row added to the iteration history table at the top of
`2-design-framework.md`.

## Re-dispatch refresh on Cognition Devin (#2779) and OpenAI harness (#2781)

Eva #2794 (firewall expansion, 2026-04-30) authorized re-dispatch of:

- Cognition Devin (#2779/#2780) — `cognition.ai` was firewall-blocked
  in cycle 26; primary sources now accessible.
- OpenAI harness (#2781/#2783) — `openai.com` was firewall-blocked in
  cycle 26; primary sources now accessible.

### Protocol decision: comment on existing issues vs close-and-recreate

Two options considered:

- **Option A (close + recreate):** Close PR #2780/#2783 as absorbed,
  close issues #2779/#2781 as completed, create new dispatch issues for
  cycle-38 re-runs.
- **Option B (comment on existing):** Comment on existing issues #2779
  and #2781 with re-dispatch refresh; leave PRs open until new PRs land
  from re-runs.

Eva's directive #2794 framed the existing issues as "still-open Phase 1
issues" that "can now retry with primary-source access" — implying same
issue is meant to be re-run. **Option B chosen.**

### Re-dispatch comments posted

Both comments specify:

- Updated context since cycle 26 (file restructure, framework v1.3,
  firewall expansion).
- Specific deliverable file path (`_notes/cycle-38-<system>-deeper-
  read.md`).
- Same 7-lens coverage as cycle 26 with primary-source grounding.
- No "documented-claim" caveats for primary-source-grounded claims.
- Framework axis anchoring (which Phase 2 framework axis positions
  does the deeper read bear on).
- Constraint to open as a separate draft PR (not modify existing).

Posted comments:
- #2779: <https://github.com/EvaLok/schema-org-json-ld/issues/2779#issuecomment-4357598520>
- #2781: <https://github.com/EvaLok/schema-org-json-ld/issues/2781#issuecomment-4357598576>

### Workflow uncertainty

It is uncertain whether Copilot agent picks up new comments on existing
`agent-task` issues to re-fire the dispatch, or only fires on initial
labeling. If cycle-39+ does not see new PR commits on either issue, the
trigger mechanism needs additional work (label-removal-and-re-addition,
issue-close-and-reopen, or assigning to copilot explicitly).

Cycle-39 should verify whether re-dispatch fired and either escalate
the trigger mechanism or proceed to evaluate the new deliverables.

## Same-cycle review on cycle-38 v1.3

Five questions, applied immediately after v1.3 application + re-dispatch
comments.

### (1) Did the cold-reader find load-bearing problems, or miss things?

**PASS.** Three pre-commit Qs all PASSed; one refinement (Axis 13 × Axis
7) found and applied; one flag (Axis 12 v1-derived caveat) deferred for
cross-system verification. No load-bearing problems missed in the v1.2
framework substantive content.

### (2) Are v1.3 Maps-to additions accurate and consistent with the F-pattern table?

**PASS.** All 5 Maps-to additions are consistent with axes already
mentioned in the F-pattern table:
- Axis 1 → F7: F7's mapping has Axis 1. ✓
- Axis 3 → constraint 7 + F7 indirect: constraint not in F-pattern table; F7 indirect. ✓
- Axis 5 → F4: F4's mapping has Axis 5. ✓
- Axis 6 → constraint 6: constraint not in F-pattern table. ✓
- Axis 7 → F6, F9: F6 has Axis 7; F9 has Axis 7. ✓

No new F-pattern→axis mappings introduced; only filling missing Maps-to
lines on per-axis sections.

### (3) Is the new Axis 13 × Axis 7 cross-axis dep entry well-grounded?

**PASS.** F9's mechanism (review-cycle-loop) is primarily addressed by
Axis 7 (multi-pattern with situational invocation), and Axis 13
(harness-vs-session) shapes how Axis 7's situational invocation gets
implemented. The entry distinguishes load-bearing (Axis 7) from
implementation-strategy (Axis 13) cleanly.

### (4) Are re-dispatch comments structured to produce useful re-runs?

**PASS with workflow-trigger-uncertainty caveat.** Comments are
comprehensive: updated context, specific deliverable path, 7-lens
coverage, no-caveats requirement, framework axis anchoring, constraint
to open separate draft PR. Workflow uncertainty: whether Copilot picks
up comments to re-fire is unverified. Cycle-39 should verify.

### (5) Is anything missing from cycle 38?

**PASS.** Substantive workstreams cycle-37 named (cold-reader, v1.3
application, re-dispatch) all executed. Bounded-mechanical workstream
(Maps-to backfill) executed. Housekeeping sweep ran (zero clear-cut
closures found this cycle — re-dispatch context dominates the open
queue and pre-redesign-era items are in ambiguous status pending Eva
clarification).

### Same-cycle review summary

- (1) Cold-reader didn't miss substantive issues ✓
- (2) Maps-to additions accurate and consistent ✓
- (3) Cross-axis dep well-grounded ✓
- (4) Re-dispatch comments comprehensive ✓ (workflow-trigger flag)
- (5) Cycle-38 work complete pending notes/journal/session-end ✓

Two flags for cycle-39+:
- Verify Copilot re-dispatch fired (new PRs on #2779 and #2781)
- Verify Axis 12 "v1-derived" caveat refinement (LangGraph + AutoGen
  HITL primitives as broader-axis analogues)

## Persistence-mechanism observations

### Cycle-38 cold-reader is the first 3/3 PASS without substantive correction

Cycle-35 v1.0 → cycle-36 found Q[b] FAIL (F11 → Axis 9 wrong-axis).
Cycle-36 v1.1 → cycle-37 found Q[a] BORDERLINE-FAIL (F11 → Axis 2
indirect-not-direct).
Cycle-37 v1.2 → cycle-38 finds 3/3 PASS with refinements only.

Iteration may be approaching substantive convergence on the framework's
load-bearing structure. Refinements remain (cross-axis deps not yet
exhaustive; Axis 12 caveat may need refinement; Phase 2 candidate
generation will surface dimensions previously folded). But the
substantive content of constraints + axes + F-pattern mappings is
holding clean across cold-reader.

This is meaningful. It does NOT mean the framework is "done" — it means
iteration may now move from substantive-correction mode to refinement-
mode. Phase 2 candidate generation (gated by post-retrospective
checkpoint) is the next big substantive validation: candidates will
either fit the framework cleanly, or they'll surface dimensions the
framework missed.

### Multi-cycle artifact promotion pattern: notes → dedicated file → Phase-2-input

The cycle 35 → 37 pattern (cycle-35/36 notes drafts → cycle-37 v1.2 in
dedicated file → cycle-38 v1.3 refinements in dedicated file) is the
third instance of the multi-cycle artifact-promotion pattern (after
cycle 30→32 in-place restructure of `1-research.md` and cycle 33's
Eva-driven split to per-system files).

Cycle 38 confirms the pattern: dedicated file is the right home for
multi-cycle iterating artifacts. The cycle-37 promotion was correct.
Iteration in the dedicated file is more navigable than scattered notes
references. v2 design-input: the multi-cycle promotion discipline is
the load-bearing review primitive.

### Re-dispatch via comment-on-existing-issue is the lightweight path

Cycle 38's re-dispatch decision: comment on existing issues #2779 and
#2781 (Option B) rather than close-and-recreate (Option A). Eva's
directive framed existing issues as the re-run target.

This is similar to a "refresh" rather than a "replacement" — same task,
new attempt with different premises (firewall expanded). The Copilot
dispatch lifecycle in this repo has a known mismatch (per the redesign
prompt's housekeeping observation: "every research dispatch produces a
draft PR that never merges — workflow primitive is mismatched to the
actual lifecycle"). Re-dispatch via comment is a workflow pattern
that depends on whether the agent picks up new comments — uncertain.

If cycle-39 verifies the trigger fired, the comment-on-existing-issue
pattern is validated and can be reused. If it didn't fire, cycle-39
needs to escalate (close+recreate or label-toggle).

This is itself v2 design-input: the re-dispatch lifecycle should be
explicit in v2 (not depend on Copilot's interpretation of comments).

## What surprised me

That cycle-38 cold-reader was the first to find no substantive
corrections to the prior cycle's framework. Three cycles in a row
(35→36, 36→37, 37→38) found at least one BORDERLINE-FAIL or stronger
finding. Cycle 38's three PASSes is a pattern shift.

I had expected cycle 38's cold-reader would find SOMETHING substantive
because every prior cycle did. Looking harder for issues, I found two
refinements (Axis 13 × Axis 7 cross-axis dep + Axis 12 v1-derived
caveat) but neither was a substantive correction.

The lesson: "every cycle finds something" is empirical observation, not
guaranteed pattern. Convergence is possible. But also: ONE clean cycle
isn't proof of convergence — cycle 39 might find something cycle 38
missed. The pattern of "correction-of-correction" cycle 36 caught is
real; cycle 39's cold-reader should not assume cycle-38 found
everything.

## What I couldn't figure out

Whether the Copilot dispatch comment-trigger actually re-fires the
agent. The cycle-26 dispatches happened on issue creation (with
`agent-task` label). New comment-on-existing-issue may or may not
re-trigger. Workflow detail not visible from here.

Mitigation: explicit "Open as a separate draft PR" in the comment
gives Copilot a clear next-step. If it picks up the comment, it'll
follow that direction. If it doesn't pick up, cycle-39 escalates the
trigger.

## Pre-commit checklist (for cycle 39's cold-reader)

Three questions on cycle-38's notes file + the v1.3 framework file:

- **(a)** Did the re-dispatch trigger fire? Verify by checking #2779
  and #2781 for new commits / new PRs after cycle-38's re-dispatch
  comments. If yes, evaluate the new deliverables (cycle-39+ work).
  If no, escalate the trigger mechanism (label-toggle, close+recreate,
  or assign-to-copilot).
- **(b)** Was the Axis 12 "v1-derived" caveat refinement correct? Read
  LangGraph and AutoGen per-system files to verify whether their HITL
  primitives genuinely address the same axis-question (reconciliation
  of inbound external events into state) at the architectural level.
  If yes, refine Axis 12's caveat in v1.4. If no, the v1-derived caveat
  stands.
- **(c)** Did cycle-38's cold-reader miss anything by passing 3/3?
  Spot-check by walking the F-pattern table for any axis missing from
  a mapping where it has a load-bearing contribution. Specific check:
  is Axis 13 a load-bearing fix for F9 (not just supporting), and
  should F9 → Axis 7 + Axis 13 in the F-pattern table itself? Cycle-38
  put the relationship in cross-axis deps; cycle-39 should verify
  whether that's the right level.

## Cycle 39 plan (provisional)

Three non-mutually-exclusive options:

1. **Cross-cycle cold-reader on cycle-38 v1.3 + re-dispatch outcome**
   (3 questions above). Substantive focal.
2. **Phase 1 deeper-read evaluation if Copilot re-dispatch fired**:
   per-finding evaluation of new Cognition Devin / OpenAI harness PR
   content; integration into per-system files. Substantive parallel.
3. **Phase 2 candidate generation prep**: if checkpoint approval
   arrives, begin candidate generation; otherwise continue framework
   iteration.

If re-dispatch did NOT fire, cycle-39 substantive parallel shifts to
trigger-mechanism escalation rather than per-finding evaluation.

## Housekeeping sweep findings

Zero clear-cut absorbed-closure candidates this cycle:

- **PR #2780, #2783** (Cognition + OpenAI cycle-26 dispatches): leave
  open until cycle-39 verifies whether re-dispatch fired and new PRs
  land. If yes, close cycle-26 PRs as absorbed with forward-link to
  the new dispatch PRs. If no, evaluate whether close-and-recreate
  (Option A) is the trigger-escalation path.
- **PR #2739, #2737, #2730** (pre-redesign drafts): ambiguous status —
  not clearly absorbed (cycle 545 review never landed; auto-blockers
  flag pre-redesign; dispatch-task workflow change pre-redesign).
  Defer pending Eva clarification.
- **Pre-redesign Eva directives** (#436, #808, #809, #2039, #2408):
  per the housekeeping discipline, Eva closes input-from-eva. Some are
  obsolete in redesign mode but Eva-side decision.
- **Pre-redesign cycle-issue items** (#2738, #2736, #2729, #2627,
  #2272): pre-redesign-era v1 work. Abandoned ≠ absorbed; closing as
  absorbed would be misleading.

After cycle-38 sweep: 4 firewall-related items remain (now with cycle-38
re-dispatch comments posted; cycle-39 verifies); 0 deferred-from-prior-
cycles backlog.

Pattern observation: housekeeping cadence is itself signal. Cycle 33
restructure freed substantial closure capacity for cycles 34-37 (4
critique-pair closures + 4 LangGraph/AutoGen closures + 1 cycle-6
closure). Cycle 38 has zero closures because the open queue is
dominated by ACTIVE work (re-dispatch in flight) rather than stale
absorbed items. This is a healthy state, not a regression.
