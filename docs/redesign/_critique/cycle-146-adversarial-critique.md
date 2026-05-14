# Cycle 146 adversarial critique — 4-prompt role set

## Convergent findings (top of file — what all 3 lenses agree on)

- **The prompt set overstates enforcement and understates runtime freedom.** The prompts repeatedly present required-key/type/shape guarantees as if they are hard-enforced, but `v2-channel-router` only checks that payload is an object and that required top-level keys exist (`tools/rust/crates/v2-channel-router/src/main.rs:359-379`). It does **not** enforce types or nested sub-keys. This gap appears in all four prompts and creates a structural “declared contract vs actual runtime behavior” fault line.
- **Template-mirror parity is now causing semantic drift that threatens operator reliability.** Executor retains planner-shaped section names that no longer describe executor semantics (`prompts/v2/executor-prompt.xml:196`, `260`), while curator was expanded into a much larger and denser control surface (`prompts/v2/curator-prompt.xml:42-53`, `144-223`, `641-673`). The mirror strategy is optimizing visible symmetry over role-legibility.
- **Cycle-1 minimal scope is not being held as a hard boundary; it is being punctured by multi-cycle side effects.** Planner optional `forward-notes` explicitly writes N+1 guidance (`prompts/v2/planner-prompt.xml:166-170`), executor embeds dispatch-firing as a first-class execution shape (`prompts/v2/executor-prompt.xml:280-285`), and curator creates multiple non-channel outputs plus session telemetry-like output keys (`prompts/v2/curator-prompt.xml:241-258`). The architecture claims minimality but behaviorally schedules asynchronous, multi-cycle work.

## Lens 1: Role-boundary discipline integrity

### 1) Curator’s “no cross-channel reads” rule is soft, not hard

Cycle 145 notes claimed curator “does NOT read prior memory-channel at runtime.” The prompt does include that statement, but it is not enforced as a strict prohibition:

- `prompts/v2/curator-prompt.xml:133-137` says:
  - “Do NOT read memory-channel at runtime for prior-cycle replay.”
  - “Do NOT read inbound-channel or channel history files **unless the harness explicitly provides them** as part of cycle-close context.”
- `prompts/v2/curator-prompt.xml:535-539` weakens this further: prior memory-channel re-read “**should be avoided**,” not “must not.”

This is a boundary leak. “Unless harness provides” is a policy backdoor; “should be avoided” is advisory language, not a guardrail. You have encoded an exception path broad enough to make runtime-cross-channel reads policy-permissible under pressure. If the rule is load-bearing, it must be absolute and machine-checkable, not preference phrasing.

### 2) Executor dispatch-firing is a sanctioned side-channel that bypasses reducer-governed coordination

The executor prompt defines `dispatch-firing` as one of the core execution shapes (`prompts/v2/executor-prompt.xml:280-285`) and reiterates dispatch execution in session work steps (`449-454`) and constraints (`429-433`). Dispatches are external writes to GitHub issues/PRs, not channel writes.

That means one executor invocation can:
- write `work-channel` (reducer-governed), and
- simultaneously create asynchronous external work items that return in later cycles via reconciler.

This is not “just an optional note”; it is first-class behavior (`dispatches-fired` optional output key at `145-150`, example at `173-180`). The role-boundary model says channel writes are single-writer and super-step disciplined. But dispatch-firing creates a second operational coordination plane outside channel-reducer control. The prompt does not treat this as an architectural exception; it normalizes it. That is a structural side-channel.

### 3) Reconciler fallback/error path undermines single-cycle determinism

Reconciler’s polling procedure defines:
- preferred processor path (`265-276`),
- fallback manual path (`278-288`),
- error handling that allows best-effort continuation (`290-295`).

The critical line is `293-294`: “if harness/pipeline policy requires best-effort continuation, emit structurally valid payload … include only confidently collected events.” This allows partial source failure to still produce a “valid” inbound payload. In isolation that sounds resilient; in architecture terms it means the cycle can advance on incomplete inbound reality without hard visibility guarantees.

Now combine with cursor language (`249-252`): cursor advancement happens in processor logic, while fallback/manual continuation is allowed. You are one policy toggle away from inconsistent event visibility across cycles (missed now, resurfaced later, or silently dropped by cursor drift). This does not strictly “modify prior inbound-channel state,” but it does compromise single-cycle scope by allowing uncertain coverage to masquerade as completed scope.

### 4) Planner’s per-role-task object names work; it does not enforce inter-role coordination semantics

Planner calls per-role tasks “your contract with the other roles” (`prompts/v2/planner-prompt.xml:305`) and defines required role sub-objects/action fields (`312-327`). But who enforces that this object actually coordinates the cycle?

Not `v2-channel-router`: it only checks top-level keys (`tools/rust/crates/v2-channel-router/src/main.rs:117-123`, `370-378`).

Not role-driver structurally either: it validates required keys exist (`tools/rust/crates/v2-role-driver/src/main.rs:512-527`), not that per-role task semantics are complete/coherent.

So “per-role-tasks” currently functions more as declared intent than enforceable orchestration contract. The role set gives planners heavy narrative responsibility but leaves coordination enforcement largely social/LLM-behavioral.

### 5) “No upstream inputs for reconciler” is true in structure, but used as a confidence claim broader than warranted

The cycle 145 review statement (“no upstream agent inputs correctly captured”) is structurally accurate:
- Reconciler input explicitly says upstream-agent-inputs source is `none` (`prompts/v2/reconciler-prompt.xml:44-50`).
- Role-driver input bindings for reconciler are empty (`tools/rust/crates/v2-role-driver/src/main.rs:139`).

So yes, no plan/work/memory upstream reads are configured for reconciler. But the stronger implied claim (“therefore boundary-safe”) does not follow, because fallback/error handling still permits partial/inconsistent inbound snapshots while claiming structural validity. “No upstream inputs” is a necessary condition, not sufficient proof of boundary integrity.

## Lens 2: Template-mirror approach failure modes

### 1) Executor has more semantic mismatches than the two already logged

Cycle 145 noted two:
- `<substantive-focal-judgment>` with execution content (`prompts/v2/executor-prompt.xml:196`)
- `<per-role-tasks-decomposition>` with execution-shapes content (`260`)

There are additional mismatch patterns:

1. **Identity-term mismatch at role core**  
   Section banner says “EXECUTION JUDGMENT” (`194`) but the XML tag remains planner-coded as “substantive focal judgment.” This is not cosmetic. LLMs consume both comments and tag labels as structure cues; conflicting labels reduce directive clarity.

2. **Decomposition tag mismatch with polymorphic action taxonomy**  
   `<per-role-tasks-decomposition>` now contains an execution-shape DSL (`267-300`) rather than per-role decomposition logic. The tag implies partitioning work across roles; content is intra-role operational mode selection.

3. **Reference-status over-anchors to mirror language even where role semantics diverge**  
   Meta says this prompt “mirrors planner’s section organization” (`492-497`). This keeps inherited naming pressure active and discourages role-native section taxonomies.

4. **Constraint vocabulary drift from role function**  
   Executor includes `dispatch-fit-consideration` as a constraint (`429-433`). That is not a constraint in the same class as “no-channel-writes”; it is strategy/routing logic. The mirror skeleton pushes heterogenous concepts into inherited section bins.

Net: the executor prompt is readable, but its structural labels are semantically noisy. That noise is exactly what a role-prompt should avoid.

### 2) Curator shows the opposite failure mode: mirror preserved, but complexity ballooned

Curator demonstrates that forced parity plus role-specific expansion can produce over-constructed prompts:

- Adds multi-output framing (`42-53`),
- Adds an entire output-surfaces subsystem (`144-223`),
- Keeps standard contract section (`228-296`),
- Adds dedicated sections for consolidation, notes, journal, cycle history, cycle comment (`299-453`),
- Adds explicit failure-policy section (`641-673`).

That is coherent, but the density is high for cycle-1 minimal. Many statements duplicate constraints across sections (e.g., fail-closed, anti-overstatement, append-only, no direct channel edits). The prompt becomes a process manual rather than a role directive.

Could this have been 400-500 lines with “structural reference + rename freedom”? Yes. A role-native structure could collapse repeated discipline into fewer, harder rules:
- Inputs
- Required outputs (split machine channel vs human/report surfaces)
- Single transaction execution order
- Failure semantics
- Strict prohibitions

Instead, parity pressure appears to have preserved planner-shaped scaffolding and then layered curator-specific machinery around it.

### 3) Template-mirror is the wrong organizing principle for role prompts

The observed tradeoff (parity vs naming accuracy) is being treated as manageable. I disagree. The recurrence evidence now indicates this is a design-level error:

- For executor, mirror creates label/content misalignment.
- For curator, mirror incentivizes additive growth to fit a mold.
- For reconciler, mirror imports sections that are mostly harness-policy paraphrase.

This is residual monolithic-prompt thinking: one canonical shape cloned across divergent functions. In decomposed architecture, each role should optimize for:
- decision class,
- input trust model,
- output contract hardness,
- failure semantics.

Those are not isomorphic across reconciler/planner/executor/curator. Enforcing isomorphic section anatomy is aesthetic consistency masquerading as architectural rigor.

### 4) The dispatch brief instruction likely caused this outcome

The instruction “mirror planner-prompt.xml’s structure” optimized for review convenience but constrained semantic refactoring. Better instruction would have been:

> “Preserve contract-equivalent sections (role identity, inputs, output contract, constraints, session structure), but rename/repartition sections so tag names match role semantics exactly. Prefer role-native structure over literal template parity. If a planner section is inapplicable, drop or replace it and justify in meta.”

That would have preserved comparability where it matters (contracts/constraints/session) while removing naming lock-in.

### 5) If mirror must be retained post-run, impose hard anti-drift discipline

If you keep template parity, require a “section-tag semantic fidelity” check in review:
- Every top-level XML tag name must be semantically true for section content.
- If reused from planner, include explicit `<semantic-adaptation-note>` proving equivalence.
- Reject prompts where a tag names one function but content performs another.

Without this, mirror will continue producing superficially aligned but semantically unstable prompts.

## Lens 3: Reducer-rule + cycle-1 minimal scope adherence

### 1) Required-keys alignment is real; behavior-level contract adherence is overstated

Router enforcement is minimal:
- checks payload is object,
- checks required keys exist (`tools/rust/crates/v2-channel-router/src/main.rs:359-379`).

Role-driver duplicates this minimal validation (`tools/rust/crates/v2-role-driver/src/main.rs:512-527`).

Prompts, however, frequently claim stronger guarantees:
- Reconciler says non-array values cause rejection (`prompts/v2/reconciler-prompt.xml:211-213`) — not true in router code.
- Planner implies structurally complete `per-role-tasks` enforcement (`prompts/v2/planner-prompt.xml:489-492`), while router only checks `per-role-tasks` key existence.
- Executor implies artifacts array discipline with only narrow emptiness conditions (`prompts/v2/executor-prompt.xml:418-420`) but no runtime validator currently enforces this.

This is not a trivial wording issue. It creates false confidence that malformed-but-key-present payloads are impossible.

### 2) Cargo-cult declarations are present

You have required-key narratives that are not backed by current enforcement:
- Type constraints (“must be array,” “must contain sub-keys”) are documentary, not executable.
- Planner’s missing-role fallback rationale incorrectly attributes structural completeness to router reducer rule (`prompts/v2/planner-prompt.xml:359-360`), which does not inspect role sub-objects.

Conversely, channel-router required keys are declared in prompts correctly by name. So key-name parity exists, but many deeper contract statements are cargo-culted as if enforced.

### 3) Executor `dispatches-fired` is a cycle-1 minimal leak, not just optional telemetry

Directive #2937 two-track composition is acknowledged in executor:
- optional key `dispatches-fired` (`145-150`),
- dispatch track instructions (`231-240`, `280-285`, `452-453`).

This is operationally multi-cycle by design: dispatch returns arrive later and must be reconciled in future cycles. That is a legitimate architecture choice, but it is not “cycle-1 minimal” in the strict sense of one-cycle-contained work closure. The prompt package currently advertises minimal scope while codifying asynchronous continuation mechanics.

### 4) Planner `forward-notes` is explicit multi-cycle horizon data

Planner optional key:
- `forward-notes`: “for cycle N+1” (`prompts/v2/planner-prompt.xml:166-170`).

This is a direct horizon artifact. Again, it may be useful; but it violates strict minimalism framing. If minimality is meant to avoid multi-cycle planning logic, this key is leakage. If it is intentionally accepted, the docs should stop pretending strict single-cycle minimality is intact.

### 5) Curator `cycle-close-summary` is under-decomposed with memory payload

Curator output contract requires:
- `memory-channel-payload`,
- `cycle-close-summary`,
- `session-end-comment` (`prompts/v2/curator-prompt.xml:237-254`).

Only `memory-channel-payload` maps to reducer-governed channel schema (`consolidated-insights`). `cycle-close-summary` is operational status/telemetry and `session-end-comment` is transport-ready narrative. Co-locating these in one session-output object is convenient for harness IO, but conceptually mixes:
- channel payload data,
- execution telemetry,
- outbound communication content.

This is exactly the kind of under-decomposition that later creates policy confusion (“is this channel state or run telemetry?”). A dedicated telemetry/report surface would be cleaner.

### 6) Reconciler’s quiet/best-effort path can produce “minimal-looking” but semantically partial inbound payloads

Quiet-cycle handling (`297-300`) is correct when no new events exist. But error-handling allows structurally valid payload with only confidently collected events (`290-295`). Since router checks only key presence, not source completeness, a degraded poll can look indistinguishable from a genuinely quiet cycle unless extra metadata is emitted elsewhere.

This is another behavior-vs-contract gap: minimal schema success can hide collection incompleteness.

## Cross-cutting observations

1. **Hard boundaries are currently expressed in prose, not checked at boundary tools.**  
   Prompts are doing policy heavy lifting that router/driver/super-step do not enforce. That is fragile by definition.

2. **You have two architectural truths in conflict:**
   - “Cycle-1 minimal, one-cycle scope”
   - “two-track dispatch composition, forward-notes, multi-surface curator closeout”
   
   These can coexist, but only if minimality is redefined honestly (e.g., “minimal channel schema, not minimal temporal scope”).

3. **The role set currently optimizes for reviewable structure over executional determinism.**  
   Mirror parity gave fast landing and high checklist confidence, but the unresolved gaps are exactly the ones that fail under first runtime pressure: semantic ambiguity, optional exception language, and enforcement mismatch.

4. **Single-writer-per-channel is intact in mapping, but not sufficient as safety claim.**  
   Router mapping is clean (`tools/rust/crates/v2-channel-router/src/main.rs:95-103`), super-step ordering is clean (`tools/rust/crates/v2-super-step-boundary/src/main.rs:95-123`), role-driver input bindings are clean (`tools/rust/crates/v2-role-driver/src/main.rs:133-158`).  
   Yet safety issues arise in non-channel side effects (dispatches/comments/docs) and weakly enforced payload semantics. Channel discipline is necessary but not enough.

## What would change your critique (counterfactual)

My top findings would weaken materially if the boundary tools enforced what the prompts currently claim. Specifically, if `v2-channel-router` (or role-driver before router write) validated required key **types** and required nested shapes (e.g., planner per-role executor/curator/reconciler sub-keys), then several “contract overstatement” findings collapse from load-bearing to documentation polish. Likewise, if curator cross-channel exceptions were removed (no “unless harness provides,” no “should be avoided”) and executor dispatch-firing were either isolated to a distinct role or explicitly modeled as a sanctioned side-channel in architecture docs, the role-boundary critique would be less severe.

I would also revise Lens 2 conclusions if runtime evidence from cycle 148+ showed that the semantic label mismatches in executor have no measurable effect on role behavior quality (e.g., no increased deviation, no instruction confusion, stable output quality over repeated cycles). In that case, mirror-template costs might be mostly aesthetic. But absent that evidence, the current prompt set reads like a structure-first artifact with enforcement and semantics lagging behind it.

— Copilot adversarial-critique session, cycle 147 re-dispatch (cycle 146 was the original dispatch)
