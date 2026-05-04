# Cycle 62 — AutoGen implications for the v2 redesign

**Date:** 2026-05-04
**Substantive focal activity:** option 4 from input-from-eva [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829) (implications mining on a system already read at depth).
**Pivot from cycle-61's provisional read:** cycle-61 recommended applying the Maps-to ↔ position-table-system-list lens (sixth NEW parent lens first-application under strict count). That recommendation is bounded-mechanical fallback under the new polarity; cycle 62 instead advances research-corpus implications.

## What this document is, and is not

This is a focused, AutoGen-specific implications writeup — what AutoGen's
deep-dive evidence (PR [#2763](https://github.com/EvaLok/schema-org-json-ld/pull/2763))
suggests for the v2 redesign that has NOT been written down in
`1-research.md` cross-system observations or in `2-design-framework.md`
axes. The cross-system synthesis cites AutoGen alongside other systems
under shared patterns; this document inverts the lens — it asks what
AutoGen tells us *as a singular voice* (or a pair-with-LangGraph voice
where the pattern is foregrounded by both).

It is NOT a Phase 2 candidate. It is implications-as-input. Phase 2
candidates still gate on the post-retrospective checkpoint and Eva
approval.

It is NOT a re-summary of `systems/autogen.md` — that file is the
navigation summary; the deep-dive evidence is PR #2763. This file
identifies what those patterns *imply* for our redesign that the
framework has not absorbed.

## Anchoring frame inherited from systems/autogen.md

Per the per-system file's anchoring caveats list, AutoGen-to-v2-redesign
transfer is discounted by:

- Library/framework vs concrete autonomous orchestrator
- Application task boundary (`agent.run(task=...)`) vs cron-state-inferred boundary
- Human-in-the-loop default vs `EVA-DEFAULT-AUTONOMY`
- Python/.NET runtime vs Rust-on-GitHub-Actions
- Developer-owned state persistence vs repo-resident cross-cycle ledger
- Maintenance-mode status (forward-looking framework claims discounted)
- Short-to-medium tasks vs hundreds-of-cycles institutional-memory work

Implications below carry positive transferability arguments where the
discount-list is silent. Where a discount applies, it's named inline.

## Implications

### I-1. De-prescription between major versions is a first-class workflow, not a one-time event

**AutoGen-specific evidence.** v0.4 deliberately removed built-in
sequential chat as "too opinionated and not flexible enough." v0.4
also documents `ConversableAgent.register_reply` as discouraged and
old user-proxy tool-routing as problematic. The migration guide is
shipped *as part of the v0.4 release artifact*, not as a post-hoc
blog post. Anti-patterns are named alongside replacements.

**Implication for v2.** Our redesign tends to think of cutover as "we
replace v1 with v2 and v1 is done." AutoGen's pattern says: build
v2 with the expectation that v3 will explicitly remove parts of v2
later, and bake the *removal-with-rationale* mechanism into the
artifact. Cutover is one event in an indefinite-length de-prescription
sequence, not a terminus.

**v2 design candidate input.** The cutover artifact (per
`<artifact-composition>`) includes a migration runbook from v1 → v2.
That runbook should establish a *forward-compatible removal
convention* — when v3 deprecates parts of v2, the convention is
already in place. Concretely: a `MIGRATIONS.md` (or similar) under
`prompts/v2/` that v3 will append to, listing what was removed, why,
and what replaced it. v2 starts that file with v1 → v2 entries.

**Discount.** AutoGen's de-prescription happens between *library
versions* released independently; ours happens between
*orchestrator-prompt versions* on the same repo. The mechanism
(documentation artifact) transfers; the cadence (planned-release vs
on-demand) differs — for us, the mechanism may need
event-triggering rather than version-tagging.

**Open question.** Does v2 itself need an explicit version number
visible to the orchestrator? Or is "the prompt's git SHA at runtime"
sufficient? AutoGen has explicit semantic versions (v0.2 / v0.4);
v1's prompt has cycle-tracked iteration but no semver. Worth deciding
in Phase 2.

---

### I-2. Layered architecture with intentional opinion-gradient

**AutoGen-specific evidence.** Five named layers — Core (unopinionated
event-driven runtime), AgentChat (opinionated presets), Extensions
(model clients, tools, code execution), AutoGen Studio (developer GUI,
explicitly "not meant to be a production-ready app"), AutoGen Bench
(benchmarking). The opinion-gradient is *load-bearing*: Core says "It
is not tied to any specific agent abstraction or multi-agent pattern;"
AgentChat provides "intuitive defaults, such as Agents with preset
behaviors and Teams with predefined multi-agent design patterns." The
five layers are not just module separation — they are deliberate
opinion-gradient layering.

**Implication for v2.** v1's prompt + checklists + tools mix opinion
levels indiscriminately. Some prompt sections prescribe specific
sequences (high-opinion); some tools enforce mechanical invariants
(also high-opinion); some tools are unopinionated primitives (e.g.,
write-entry just appends). The mix is implicit. v2 could make the
opinion-gradient explicit — e.g., a Core/Tools/Workflow layering
where Core is unopinionated primitives (file-write, issue-read,
git-commit), Tools layer adds opinion (commit-with-push as one
operation enforcing the cycle-524 invariant), and Workflow layer is
the orchestrator prompt that composes them.

**v2 design candidate input.** Stratify the v2 tool suite by opinion
level. Document per-tool the invariants it enforces (i.e., "this tool
embeds the opinion that X"). Make the boundary between "thing that
just does I/O" and "thing that enforces a workflow rule" visible in
the directory layout or naming convention.

**Discount.** AutoGen's layers exist because library users compose
them differently per application. Our v2 has a single composition
(the orchestrator prompt). The layering may not need user-facing
exposure but the *design discipline* of opinion-gradient still applies
internally.

**Cross-reference.** Connects to `2-design-framework.md` Axis 1
(Authority allocation: prompt-vs-tool split) — but adds a within-tools
sub-axis the framework does not yet split on (opinion-level within
the tool suite).

---

### I-3. Termination as first-class composable callable, with graceful vs immediate distinction

**AutoGen-specific evidence.** Termination conditions are objects
evaluated against recent messages/events, combinable with AND (`&`)
and OR (`|`); types include maximum-messages, text-mention,
token-usage, timeout, source-match, external-termination. External
*graceful* stop is distinct from immediate cancellation: graceful lets
the current agent's turn finish before team stop ("keeping the team's
state consistent"); immediate is exception-based abort.

**Implication for v2.** v1 cycles run for a fixed compute budget
(~75 minutes). Termination is implicit (cron-runner kills the
process). There is no first-class concept of "graceful termination"
vs "immediate termination" — the orchestrator just stops mid-turn
when the budget hits. Cycle 524's corruption class (issue #2638,
preserved as `<git-safety>` primitive) is precisely the failure mode
that "graceful termination keeping state consistent" prevents.

**v2 design candidate input.** Termination predicates as first-class
composable objects:
- `BudgetExpired` (compute budget hit) — graceful (finish current
  tool call, then commit + push pending state, then exit)
- `EvaApprovalReceived` (input-from-eva on a pending checkpoint) —
  graceful (acknowledge, persist context, exit)
- `AbortCriterion` (one of `<abort-criteria>` triggers) — graceful
  with abort-summary
- `CorruptionDetected` (state-of-record consistency check fails) —
  immediate (no graceful state-write, escalate to Eva)

The graceful-vs-immediate distinction maps onto trust posture:
graceful trusts the system enough to write state; immediate distrusts
the state and requires human review.

**Discount.** AutoGen's termination is for in-process agent teams
where termination is a runtime decision. Our v2 termination is at the
process boundary (cron-runner exit). The mechanism transfers as
*conceptual structure* — the predicate composability and graceful-vs-
immediate distinction — even if the implementation is a Rust trait or
enum rather than a Python AND/OR overload.

**Cross-reference.** Connects to `2-design-framework.md` Axis 2
(Cycle boundary primitive) and to `<git-safety>` preserved-primitive
(unpushed-commit prohibition is the corruption-detected immediate-
termination case, conceptually).

---

### I-4. Publishing what the system does NOT centrally guarantee

**AutoGen-specific evidence.** PR #2763's deep-dive lens 4 lists
explicitly: AutoGen does not centrally guarantee semantic correctness
of final answers; deadlock diagnosis beyond timeouts/cancellation;
global reconciliation of all component states; automatic retry policy
for malformed model output; durable recovery from process crash
without app-level persist/reload. Failure handling is explicitly
delegated: "define a protocol, stream/observe it, terminate it,
cancel if needed, persist state if the app needs resume."

**Implication for v2.** v1 has *implicit* non-guarantees that have
surfaced as F-patterns in `0-retrospective.md` (chronic-category-
currency loop, eva-blocker queue, abandonment cascade, gate
proliferation). These are post-hoc descriptions of failures, not
explicitly-published non-guarantees. The asymmetry matters: AutoGen
publishes non-guarantees so users plan around them; v1 silently fails
and surfaces non-guarantees as retrospective findings.

**v2 design candidate input.** v2's prompt or top-level documentation
should include an explicit "What v2 does not guarantee" section.
Candidate non-guarantees worth naming:
- Semantic correctness of orchestrator decisions (Eva is the
  semantic-correctness oracle; v2 cannot self-verify)
- Convergence on a checkpoint within a fixed cycle budget (per
  `<iteration-until-approval>`, approval is the only stopping
  signal — no internal threshold)
- Recovery from corruption mid-cycle (per `<git-safety>` and
  cycle-524's lesson — graceful termination can fail; immediate
  termination requires human reset)
- Deduplication across the dispatch fleet (Copilot dispatches and
  audit-repo cross-references can produce overlapping work; v2
  doesn't centrally reconcile)
- Cross-cycle hypothesis tracking persistence (the persistence
  mechanism is the orchestrator's design problem per `<persistence>`,
  and may itself drift across cycles)

**Discount.** AutoGen's non-guarantees are documented for library
users to plan around. Our v2's non-guarantees would be documented
for Eva (and for any future cold-start orchestrator instance reading
its own prompt). The audience is different but the discipline is
the same.

**Cross-reference.** Connects to `0-retrospective.md` F-patterns —
several F-patterns (F1 chronic-category-currency loop, F8
abandonment cascade) are non-guarantees in disguise. Naming them
positively as non-guarantees is a different framing than treating
them as failure modes; both framings are valid, but the positive
framing is what AutoGen's pattern adds.

---

### I-5. Uniform error-shape on tool results (`is_error: true`)

**AutoGen-specific evidence.** Tool errors are reported as result
objects with `is_error: true` rather than thrown exceptions, keeping
error shape uniform with success shape. The host receives a
structured result either way.

**Implication for v2.** v1's Rust tools have heterogeneous output
formats: some emit JSON to stdout, some print human-readable text,
some error via stderr + non-zero exit, some via stdout messages
mixed with exit codes. The orchestrator-prompt has had to encode
per-tool result-parsing knowledge in the checklists. This is one of
the cleanest cases where `<core-design-principle>` is violated:
procedural tool-result parsing in the prompt that should be a tool's
own concern.

**v2 design candidate input.** Standard v2 tool-result shape
(JSON):

```json
{
  "ok": true|false,
  "data": {...} | null,
  "error": {"code": "...", "message": "...", "context": {...}} | null,
  "warnings": [{"code": "...", "message": "..."}],
  "tool": "tool-name",
  "version": "0.1.0"
}
```

Both success and error use the same envelope. The orchestrator-prompt
no longer needs per-tool result-parsing logic — it has one
result-handling convention.

**Discount.** AutoGen's `is_error: true` is in-process; ours is
across-process (Rust binary stdout consumed by orchestrator). The
convention transfers (uniform shape with explicit error flag) but
the transport is different. JSON-to-stdout with one envelope is the
Rust-tool-equivalent.

**Cross-reference.** Connects to `2-design-framework.md` Axis 1
(prompt-vs-tool split) and to `<core-design-principle>` test-for-
violation (per-tool result parsing in the prompt is procedural work
that should live in tools or in a uniform convention).

---

### I-6. Plan-vs-progress artifact split (Magentic-One Task Ledger / Progress Ledger)

**AutoGen-specific evidence.** `MagenticOneGroupChat` is documented
as a lead-orchestrator + specialized workers team with two named
ledgers: a **Task Ledger** (planning artifact: what to do, prioritized
list of facts and educated guesses) and a **Progress Ledger** (tracking
artifact: what's been done, what's blocked). The two ledgers are
distinct artifacts, not one merged log.

**Implication for v2.** v1 mixes planning state and progress state
across multiple primitives — cycle-issue-comments are progress;
journal entries are mixed (some planning forward, some progress
recording); `_notes/` are mostly progress with occasional forward-
planning. The mix means a future cold-start orchestrator must
reconstruct "what was planned" vs "what happened" by inference rather
than by reading two distinct artifacts.

**v2 design candidate input.** Split planning artifact from progress
artifact at the v2 design level. Candidate shapes:
- `docs/plan/active.md` — what's currently committed-to (the Task
  Ledger analogue), updated when commitments change
- `docs/journal/YYYY-MM-DD.md` — what happened (the Progress Ledger
  analogue), append-only per-cycle

The split has a load-bearing purpose: a cold-start orchestrator
reads `active.md` to know what to do next; reads the journal to know
what's been done. Today this distinction is collapsed into a single
journal that's both forward-looking and backward-recording.

**Discount.** Magentic-One's ledgers are in-process state for a
single team running on a single task. v2's analogues would be
cross-cycle state for an institutional-memory orchestrator. The
shape is similar (two artifacts, distinct purposes); the semantics
are different (ours is durable across cycles, theirs is per-task).

**Cross-reference.** Connects to `<persistence>` directive — the
plan/progress split is one candidate persistence mechanism shape.
The current journal-as-everything pattern is *one* mechanism; the
split-artifact pattern is *another*. Phase 2 candidate generation
can compare them.

---

### I-7. Aspirational vs implemented as explicit documented distinction

**AutoGen-specific evidence.** AutoGen lifecycle docs explicitly
mark some features as "not implemented yet" — agent paging in/out
is named as aspirational rather than shipped. The distinction is
deliberate: documentation is honest about what is design intent vs
what is shippable.

**Implication for v2.** v1's documentation has had drift — some
prompt directives describe behavior that no tool actually enforces
(or the enforcing tool was removed/changed without prompt update).
The asymmetry between "what the prompt says" and "what the tools
actually do" is silently maintained. v2 should mechanically
distinguish aspirational from implemented at the artifact level.

**v2 design candidate input.** Mechanical check: every directive in
the v2 prompt that names a specific tool, file path, or behavior is
backed by either (a) an existing tool/file/behavior, or (b) an
explicit `<aspirational>` tag on the directive. A CI check (or a
v2 tool — `prompt-coverage-check`) walks the prompt at build time
and verifies every named entity exists, with aspirational sections
allow-listed. This catches prompt-drift the moment the prompt says
something that's no longer true.

**Discount.** AutoGen's aspirational features are documented as
"not implemented yet" without enforcement (the docs could go stale).
v2 with mechanical enforcement is a stronger version of the same
discipline — borrowing the principle (be honest about aspirational)
and adding the rigor (CI-checked).

**Cross-reference.** Connects to F-pattern in `0-retrospective.md`
about checklist-prompt drift (where checklists referenced procedures
that didn't match prompt expectations). Mechanical enforcement is
the v2 mitigation for that F-pattern.

---

### I-8. Reset vs Resume as distinct first-class operations

**AutoGen-specific evidence.** Team `reset` (clear state, start
fresh) and team `resume` (continue from current state) are documented
as distinct operations with distinct call sites. The
component-local-state architecture makes this clean: resetting one
component (an agent) is well-defined; resetting the whole team is
a composition of component resets.

**Implication for v2.** v1 has no concept of "reset." Every cycle
implicitly resumes from repo state. There is no operator-invoked or
self-invoked "start fresh, ignore prior state" capability. This is
a gap: when prior cycle state is corrupted (per cycle 524), there
is no clean reset path beyond Eva manually editing files.

**v2 design candidate input.** First-class reset operation at the
v2 cycle level. Could be:
- An `input-from-eva` directive that triggers a reset (Eva-invoked)
- A `<corruption-detected>` immediate-termination outcome (self-
  invoked)
- A specific file (e.g., `docs/redesign/RESET.md`) that the next
  cycle reads and treats as "start fresh from here"

The reset operation is bounded: it does not delete history; it
declares "this point is the new baseline" and the orchestrator
ignores prior cycle commitments downstream of that point.

**Discount.** AutoGen's reset is in-process state clearing; ours is
distributed-state reset across multiple files. The conceptual
operation transfers; the implementation is different (probably a
git tag plus a sentinel file rather than a memory clear).

**Cross-reference.** Connects to `<abort-criteria>` (when to stop
and escalate). Reset is the post-abort recovery operation that
`<abort-criteria>` does not currently specify.

---

## What this informs

These eight implications add to the v2 design-input pool. None of them
are *commitments* — Phase 2 candidates will choose which to adopt,
which to reject, which to combine. The implications enrich the design
space rather than narrow it. Specifically:

- **I-1, I-7, I-8** add operations to the v2 lifecycle vocabulary
  (de-prescription, aspirational-vs-implemented, reset)
- **I-2, I-3, I-5** add architectural primitives (opinion-gradient
  layering, termination predicates, uniform tool-result envelope)
- **I-4, I-6** add explicit-discipline structures (published non-
  guarantees, plan/progress split)

## What remains open

- Some implications connect to existing framework axes; some don't.
  Whether the framework needs new axes to absorb (e.g., a "lifecycle
  operations" axis covering reset/resume/de-prescription) is a Phase
  2 question.
- Several implications carry discount-list caveats that need positive
  transferability arguments at Phase 2 candidate selection. The
  arguments above are first-pass; a Phase 2 candidate adopting (say)
  I-3 should re-argue transferability against the current substrate.
- This document handles AutoGen alone. LangGraph (the other deep-dive
  Eva named as un-mined) deserves a parallel implications document in
  a future cycle. Cross-system synthesis pulling from both implications
  documents would be a separate option-2 substantive activity.
- The eight implications above intersect each other in ways not yet
  mapped. Example: I-3 (termination predicates) and I-8 (reset) both
  touch lifecycle; I-5 (uniform tool-result) and I-2 (opinion-gradient)
  both touch tool-suite design. A future implications-mining cycle
  could write the cross-reference matrix.

## Cycle accounting note

This is the FIRST cycle in the v1.X sequence (cycles 35-61) to do
research-corpus advancement as the substantive focal. Prior 27 cycles
all placed cold-reader cadence in that slot per the (now superseded)
cycle-35 framing. Eva's [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829)
inverted the polarity. Cycle-62's pivot to implications mining is the
direct response to that inversion.
