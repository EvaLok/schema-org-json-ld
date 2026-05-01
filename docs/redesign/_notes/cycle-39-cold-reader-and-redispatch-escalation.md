# Cycle 39 — Cold-reader on framework v1.3, re-dispatch trigger escalation, v1.4 application

**Date:** 2026-05-01
**Run:** redesign-cycle-39 (third cycle of 2026-05-01)
**Issue:** #2800
**Model:** claude-opus-4-7

## Setup

Cycle 38 left a clear three-workstream plan:

1. Substantive focal: cross-cycle cold-reader on framework v1.3 +
   cycle-38 notes (3 pre-commit questions).
2. Substantive parallel: per-finding evaluation of new Cognition/OpenAI
   PR content if re-dispatch fired; otherwise trigger-mechanism
   escalation.
3. Bounded mechanical: TBD per cold-reader findings.

Cycle 38's 3/3 PASS cold-reader was the first cycle without substantive
correction. Pre-commit checklist for cycle 39 explicitly named the
"correction-of-correction" risk — never assume the most recent cold
reader's verdict is final.

## Cold-reader on cycle-38 framework v1.3

### Question (a) — Did the Copilot re-dispatch trigger fire?

**Verdict: PASS (verified false; trigger needs escalation).**

- **PR #2780 (Cognition Devin):** last commit 2026-04-29 22:39 UTC.
  Cycle-38 re-dispatch comment posted to issue #2779 at 2026-05-01
  03:13 UTC. No new commits to PR or new commits in `copilot/redesign-
  research-survey-cognition-devin` branch in the ~2-hour window before
  cycle 39 fired.
- **PR #2783 (OpenAI harness):** last commit 2026-04-29 22:37 UTC.
  Cycle-38 re-dispatch comment posted to issue #2781 at 2026-05-01
  03:14 UTC. Same pattern: no new commits in the ~2-hour window.

The cycle-38 hypothesis ("comment-on-existing-issue may not retrigger
Copilot") is verified true. Comment-only on an existing `agent-task`
issue does NOT fire the Copilot Coding Agent. The trigger primitive is
issue creation or assignment events, not comment events.

### Question (b) — Was the Axis 12 "v1-derived" caveat refinement correct?

**Verdict: PASS (cycle-38's flag was OVERCAUTIOUS; v1-derived caveat is
correct as worded).**

Cycle-38 flagged the Axis 12 "v1-derived" caveat as potentially too
strong because LangGraph interrupts and AutoGen HITL primitives might
be analogues. Verification by reading per-system files:

**LangGraph interrupts** (per `1-research/systems/langgraph.md` lines
100-109): "Interrupts are LangGraph's primary HITL primitive: a node
calls `interrupt(payload)`, LangGraph saves state via persistence, the
payload surfaces to the caller, execution waits indefinitely, caller
resumes with `Command(resume=...)` and the same thread ID. The docs
warn: 'the node restarts from the beginning of the node where the
interrupt was called when resumed, so any code before the interrupt
runs again.' Interrupts are not language-runtime continuations; they
are checkpoint/resume/replay at node granularity."

This is a **synchronous pause-resume** mechanism. The caller is the
active sender of `Command(resume=...)`; the graph is the passive
receiver waiting at a specific node.

**AutoGen** (per `1-research/systems/autogen.md` lines 68-75): "What
AutoGen does not centrally guarantee: ...global reconciliation of all
component states... Failure handling is explicitly delegated."

This is a **null result**, not an analogue. AutoGen explicitly
disclaims providing a reconciliation primitive. HITL features in
AutoGen are speaker-selection / turn-taking primitives, also
synchronous.

**Axis 12 question:** "How does the system reconcile inbound external
events (Eva responses, audit posts, dispatch outputs, post-close tool
mutations) into state?"

Axis 12 concerns **asynchronous absorption** of external events that
arrive independently of the orchestrator's execution thread. Eva
responds when she responds; audit posts when audit posts; PR merges
when reviewers merge. The orchestrator runs on a cron and must catch
up to whatever happened since last cycle — it cannot pause-and-wait.

**Structural distinction:** HITL pause-resume (caller is active sender
of resume command) vs reconciliation (state changes reactively from
events the orchestrator did not initiate). These are different shapes
of inbound channel. Treating them as the same broadens the axis to the
point where any HITL primitive counts as "reconciliation discipline,"
which loses the structural distinction Axis 12 is trying to draw.

**Pattern observation (mirrors cycle-37 Q[b]):** Cycle-38's flag is the
second OVERCAUTIOUS finding by the iteration discipline. Cycle-37
found cycle-36's borderline-PASS on constraint 7 was overcautious —
the wording could have been clean PASS. Cycle-39 finds cycle-38's "may
be too strong" flag was overcautious — the v1-derived caveat is
correct and the flag should be retired. The discipline catches
under-confident framings as well as over-confident ones, but
under-confidence is a less-load-bearing failure mode than over-
confidence.

**Action:** Apply as v1.4 — sharpen Axis 12 Status with a clarifying
sentence distinguishing HITL from reconciliation, retire the flag in
the iteration history.

### Question (c) — Did cycle-38's cold-reader miss anything by passing 3/3?

**Verdict: PASS (no missing F-pattern→axis mappings; F9 → Axis 7 with
Axis 13 indirect via cross-axis deps is the right level).**

Walked the F-pattern table row-by-row checking for missing axes with
load-bearing contribution:

- **F1** (Axis 8 + Axis 13): both load-bearing. ✓
- **F2** (Axis 12): Axis 7 multi-pattern doesn't change inbound
  channel; Axis 12 is the right answer alone. ✓
- **F3** (Axis 2 + Axis 12): Axis 4 could be indirect contributor
  (append-only history makes post-close evidence visible) but the
  load-bearing fixes are Axis 2 (single source of truth) + Axis 12
  (reconciliation). Adding Axis 4 would weaken clarity. ✓
- **F4** (Axis 4 + Axis 5 + Axis 12): three load-bearing axes; well-
  calibrated. ✓
- **F5** (Axis 2 + Axis 8): Axis 13 (fat-harness) might extract
  procedure but state-as-procedure is Axis 2 (file-per-component
  separation), not Axis 13. Not load-bearing. ✓
- **F6** (Axis 7 + Axis 13): both load-bearing (multi-pattern + fat-
  harness extraction). ✓
- **F7** (Axis 1 + Axis 8 + Axis 9 + Axis 13): four axes, all
  contributing. ✓
- **F8** (Axis 9 + CDP): bounded loops + no-parallel-implementations
  is broader CDP, not specifically Axis 13. CDP citation is
  intentional. ✓
- **F9** (Axis 7): the specific cycle-38 question. Axis 13's
  contribution to F9 is INDIRECT (implementation strategy for the
  Axis 7 fix). Cycle-38's cross-axis deps entry says explicitly: "F9
  (adversarial-review treadmill) is primarily fixed by Axis 7
  (situational vs fixed); Axis 13 shapes the implementation strategy
  for that fix." This is consistent with the cycle-37 direct-vs-
  indirect distinction (where Axis 2 was demoted from direct to
  indirect for F11). ✓
- **F11** (Axis 4 + Axis 12; Axis 2 indirect): cycle-37 corrected. ✓
- **F12** (Axis 2 + Axis 4 + Axis 10): well-calibrated. ✓

**Maps-to backfill verification (cycle-38 v1.3):** all 5 backfilled
Maps-to lines (Axes 1, 3, 5, 6, 7) are consistent with the F-pattern
table. No new F-pattern→axis mappings introduced; only filled missing
per-axis Maps-to lines.

**Verdict:** Cycle-38's cold-reader did not miss substantive issues.
The F-pattern table levels are correct; the cross-axis dep is the
right level for Axis 13 × Axis 7's contribution to F9.

### Cold-reader summary

Three pre-commit questions all PASS. Q(a) verifies Cycle-38's hypothesis
about comment-on-existing-issue (verified false; need escalation).
Q(b) finds cycle-38's flag was OVERCAUTIOUS; v1-derived caveat correct.
Q(c) confirms F-pattern table levels are well-calibrated.

**Two cycles in a row of 3/3 PASS** is the first sustained pattern of
convergence. But cycle-39's Q(b) finding (cycle-38 was overcautious)
shows the discipline still surfaces something — under-confidence
masquerading as convergence is a real risk. The convergence is on
load-bearing structural claims; the iteration is now sharpening
hedges and verifying flagged uncertainties.

## Re-dispatch trigger escalation experiment

Q(a) verified Copilot does NOT pick up new comments on existing
`agent-task` issues. Cycle-38's lightweight strategy ("comment for
re-dispatch") doesn't work as a primitive.

**Escalation experiment (this cycle):** assignee-toggle on both #2779
and #2781.

**Procedure:**
1. `gh issue edit 2779 --remove-assignee Copilot` — succeeded (using
   display-name "Copilot").
2. `gh issue edit 2781 --remove-assignee Copilot` — succeeded.
3. `gh issue edit 2779 --add-assignee Copilot` — **FAILED** with
   "GraphQL: Bot does not have access to the repository.
   (replaceActorsForAssignable)".
4. `gh issue edit 2779 --add-assignee "copilot-swe-agent[bot]"` —
   **succeeded** (silent success, no error).
5. The parallel re-add for #2781 was cancelled by the failed #2779
   re-add, BUT GitHub appears to have processed it before the cancel
   propagated (verified: #2781 now shows Copilot assignee).

**Asymmetry observed:** GitHub allows `--remove-assignee Copilot`
(display name) but NOT `--add-assignee Copilot`. Re-assignment
requires the bot handle `copilot-swe-agent[bot]`. This is the same
handle used by `tools/dispatch-task` (per `tools/rust/crates/dispatch-
task/src/main.rs:388`).

**Useful primitive documented:** for v2 system design, "re-dispatch via
assignee-toggle" requires the bot handle, not the display name. v1's
`tools/dispatch-task` uses the right form; manual `gh issue edit` calls
need to know the asymmetry.

**Verification deferred to cycle 40:** if Copilot picks up the toggle
and produces new commits on the existing PRs (#2780, #2783), then
assignee-toggle is the working re-dispatch primitive. If not, the
fallback is close-and-recreate (with merged context as the new issue
body).

**Open uncertainty:** if the toggle DOES retrigger but Copilot reads
only the original issue body (cycle-26 instructions), it will re-do
cycle-26's work without the cycle-38 refresh context (firewall
expansion, framework anchoring, deeper-read deliverable path). This
would be partial-success: trigger works but content is wrong. Mitigation
if observed in cycle 40: edit the issue body with merged refresh
context, then toggle assignment again.

## v1.4 framework application

Two changes applied to `2-design-framework.md`:

### 1. Axis 12 Status sharpened (HITL ≠ reconciliation)

Added clarifying paragraph after the Status block:

```
**Note: HITL primitives are not reconciliation analogues** *(verified
cycle 39, retired cycle-38's "v1-derived caveat may be too strong"
flag)*. LangGraph interrupts and AutoGen HITL primitives are synchronous
pause-resume mechanisms — the caller is the active sender of
`Command(resume=...)` (or equivalent), the graph/agent is the passive
receiver waiting at a specific node. Axis 12's reconciliation concerns
asynchronous absorption of external events that arrive independently
of the orchestrator's execution thread (Eva responds when she responds;
audit posts when audit posts; PR merges when reviewers merge). The
orchestrator cannot pause-and-wait — it runs on a cron and must catch
up to whatever happened since last cycle. Different structural shape;
HITL is not a reconciliation analogue. AutoGen explicitly disclaims
"global reconciliation of all component states" (per-system file).
```

This sharpens the framework's structural distinction. Defensive against
future re-raising of the same flag (any reader can see "yes, HITL was
considered, here's why it doesn't apply").

### 2. Iteration history table updated

v1.4 row added to iteration history table. v1.3 row remains unchanged
(the cycle-38 flag is documented in v1.3's row; v1.4's row notes the
flag was verified-and-retired).

Pattern: the iteration history is itself an audit trail of
flag→verification→retirement. Future cycles can see which uncertainties
were checked.

## Bounded mechanical

**Housekeeping scan:** examined 20 open issues, 5 open PRs.

Most active items (cycle-current, input-from-eva long-running
directives, in-flight dispatches) shouldn't be closed. Pre-redesign-era
items (#2738/#2739 cycle-545 review pair, #2729/#2730 dispatch-task
template, #2272 pipeline-check bug, #2627 record-dispatch diagnostic)
are candidates for "deferred indefinitely under redesign" closure but
require deeper read of each to determine "absorbed" status.

**Decision:** zero closures this cycle. Risk of premature closure
outweighs noise reduction. Defer pre-redesign housekeeping to a future
cycle where I can examine each candidate against current artifacts.

**Pattern observation:** cycle-38 also reported zero closures with
similar reasoning. Two consecutive zero-closure cycles is not a problem
yet — accumulated count is bounded (20 open issues, 5 open PRs is
manageable). But three or more consecutive zero-closure cycles would
suggest the housekeeping primitive needs sharpening (or that we should
do a deeper sweep with explicit per-item verdict rather than the
"clear-cut" filter).

**Redesign-notes-index Rust tool:** cycle-37/38 mentioned a possible
`tools/redesign-notes-index` Rust tool to index `_notes/cycle-N-*.md`
files (now 41 files). Building it would consume substantial cycle
budget. Deferred to a future cycle where bounded-mechanical capacity
exceeds substantive demand.

## Same-cycle review on cycle-39 work

### (1) Did the cold-reader find load-bearing problems, or miss things?

The cold-reader passed 3/3 questions but did surface a finding (cycle-38
was OVERCAUTIOUS on the Axis 12 flag). This is the discipline working —
even when all Qs pass, reading carefully catches under-confident
framings.

Risk: did I miss the converse case? Are there cycle-38 framings that
were OVERCONFIDENT and I didn't flag them as such? Looking back at
cycle-38's three PASS verdicts:

- Q(a) F11 → Axis 4 + Axis 12 stands: I verified by checking
  cross-axis deps level. PASS confirmed.
- Q(b) Axis 13 medium-vs-fat is real differentiation: I didn't
  re-verify. Cycle-38 said "Empirical-verification flag for cycle-39+:
  Phase 2 candidate generation will reveal whether candidates actually
  distribute or cluster." That's a deferral, not a problem with cycle-38
  per se. Empirical verification awaits Phase 2 work.
- Q(c) constraint 8 genuinely rejects operator-driven candidates: I
  didn't re-verify. Cycle-38 walked the rejection logic; I'll trust
  that pending Phase 2 candidate test.

So cycle-39's cold-reader caught one OVERCAUTIOUS finding, deferred
two empirical verifications to Phase 2 work, and confirmed one PASS.
The cold-reader was reasonably thorough on what it could verify; the
deferrals are honest pending-empirical, not skipped work.

### (2) Was the v1.4 application correct?

The clarifying paragraph distinguishes HITL pause-resume from async
reconciliation. The structural distinction is well-founded (caller-as-
active-sender vs caller-passive). The verbiage cites both LangGraph
and AutoGen explicitly with file references. The "verified cycle 39,
retired cycle-38 flag" parenthetical preserves the audit trail.

One minor concern: the paragraph might be too long. Could compress to
2-3 sentences. But the structural distinction benefits from the
explicit caller-active-vs-passive framing — readers may have the same
"surely HITL is reconciliation" intuition that cycle-38 had, and the
explicit framing forecloses it.

Verdict: PASS as written. Could shorten in a future cleanup cycle if
length becomes a concern.

### (3) Was the assignee-toggle experiment well-designed?

- Pro: cheap (no destructive operation; reversible if needed).
- Pro: tests the most-likely working trigger (assignment events fire
  agent webhooks).
- Pro: documents an asymmetry useful for v2 (display-name vs bot
  handle for assignment).
- Con: doesn't disambiguate body-vs-comments source-of-truth question.
  If toggle fires but content is wrong (cycle-26 work), I learned only
  half the answer.

Mitigation noted: cycle-40 verifies, applies edit-body-and-toggle
fallback if needed.

Verdict: PASS. The experiment is well-designed for the cheap-test goal;
the open question (body-vs-comments) is correctly deferred.

### (4) Is anything missing from cycle 39?

- Pre-commit checklist for cycle 40's cold-reader: yes, written below.
- Cycle 40 plan: yes, written below.
- Persistence-mechanism observations: included implicitly (the iteration
  history table is itself an evolving persistence mechanism — cycle-39
  added "verified-and-retired" semantics).
- Audit-repo cross-read: I checked audit's recent cycles; nothing new
  load-bearing relevant to cycle-39's work.

### Same-cycle review summary

5 questions, 5 PASS. No load-bearing issues found in cycle-39 work.
One self-flag for cycle-40 cold-reader: did cycle-39's "OVERCAUTIOUS"
verdict on cycle-38 itself become a new under-confidence framing? I.e.,
is the cycle-38 flag actually load-bearing in some way I missed?

## Persistence-mechanism observations

### Iteration history table accumulates verification semantics

v1.3's row contains a flag for verification ("Axis 12 'v1-derived'
caveat may be too strong"). v1.4's row notes the verification outcome
("flag retired"). The iteration history is becoming an audit trail of
flag→verification→retirement.

This is a useful pattern — future cycles can see which uncertainties
were checked. The discipline is implicit in how I wrote the v1.4 row;
it could be explicit ("Verification outcomes" column) but that adds
overhead without proportional benefit at this scale.

### Cycle-39's "two cycles of 3/3 PASS" pattern

Cycle 38 was the first 3/3 PASS. Cycle 39 is also 3/3 PASS (with one
OVERCAUTIOUS-flag-retirement). Two cycles in a row is the first
sustained convergence pattern.

But the convergence is asymptotic, not endpoint. Cycle-39 still
surfaced a finding (cycle-38 OVERCAUTIOUS); the iteration is now
sharpening hedges rather than correcting structural claims. This is
the expected pattern of late-stage iteration: refinement, not
correction.

When does the iteration discipline declare convergence vs continue?
Per ITERATION-UNTIL-APPROVAL ("there is no 'good enough' stopping
signal you trigger yourself"), Eva's approval is the only stopping
signal. So cycle-40 continues iterating regardless of consecutive PASS
counts.

### Re-dispatch primitive evolution

v1's dispatch primitive: create issue with `agent-task` label +
`copilot-swe-agent[bot]` assignee → Copilot opens session → produces PR.

v1's missing primitive: re-dispatch on existing issue. Cycle-38 tried
"comment-on-existing-issue" (failed). Cycle-39 tries "assignee-toggle"
(verification pending). Fallback is "close-and-recreate."

v2 candidates should declare a re-dispatch primitive explicitly.
Multi-cycle workflows where research deliverables need refinement
(e.g., firewall changes, framework refinement) need this primitive.

## What surprised me

The bot-handle asymmetry. `gh issue edit --remove-assignee Copilot`
worked (display name); `--add-assignee Copilot` failed (need bot
handle). I expected GitHub to be symmetric on this — either both work
with display name, or neither does. Asymmetric handling is a UX
quirk worth documenting.

Also: the parallel-tool-call cancel propagation. When the first
`--add-assignee Copilot` failed, the parallel tool runner cancelled the
second call — but GitHub had apparently already processed the second
request. Both #2779 and #2781 ended up with Copilot re-assigned. This
is the failure mode "cancellation doesn't undo already-sent network
requests" — not a bug, but a property of HTTP API calls vs in-process
function calls.

## What I couldn't figure out

Whether the assignee-toggle will actually retrigger Copilot. The
experiment is set up; cycle 40 verifies. If it works, the next question
is whether Copilot reads the cycle-38 refresh context (comments) or
re-executes on the original issue body (cycle-26 instructions).

Also: whether v1 has a primitive that cleanly handles "re-dispatch
existing work with refined context." `tools/dispatch-task` creates new
issues; there's no `tools/redispatch` that handles the body-edit + toggle
combination. v2 could surface this as a first-class primitive.

## Pre-commit checklist for cycle 40's cold-reader

Three questions:

- (a) Did the assignee-toggle re-trigger Copilot? Verify by checking
  PR #2780 and PR #2783 for new commits OR new draft PRs after
  ~05:25 UTC 2026-05-01. If yes: also check whether the new content
  honors the cycle-38 re-dispatch refresh context (deeper-read
  deliverable file path, framework axis anchoring) or just re-executes
  the cycle-26 instructions from the original issue body.

- (b) Was cycle-39's Q(b) verdict (cycle-38 OVERCAUTIOUS) itself
  correct, or is the v1-derived caveat genuinely too strong in some
  way I missed? Specific check: is there any surveyed system whose
  HITL primitive operates ASYNCHRONOUSLY (not synchronous pause-
  resume)? Voyager, openclaw, PAI haven't been explicitly checked
  for this — verify they don't have async-reconciliation analogues
  before declaring v1.4 stable.

- (c) Did cycle-39 OVER-engineer the v1.4 application? The clarifying
  paragraph is ~150 words. Could the same structural distinction be
  made in 30-50 words? Spot-check: try to compress the paragraph to
  3 sentences while preserving the caller-active-vs-passive distinction
  and the AutoGen explicit-disclaimer citation. If it shortens cleanly,
  apply as v1.5 cleanup; if not, v1.4 length is justified.

## Cycle 40 plan (provisional)

1. Substantive focal: cross-cycle cold-reader on framework v1.4 +
   cycle-39 notes (3 Qs above).
2. Substantive parallel: re-dispatch outcome verification (Q(a) above).
   Per-finding evaluation if Copilot re-fired with correct context;
   close-and-recreate if not.
3. Bounded mechanical: TBD per cold-reader findings; possibly attempt
   compression of v1.4 Axis 12 paragraph if Q(c) finds it possible.

If checkpoint approval (post-retrospective) arrives between cycles,
Phase 2 candidate generation can begin; cold-reader and re-dispatch
work continue in parallel either way.
