---
cycle: 144
date: 2026-05-14
focus: Absorption of input-from-eva #2937 ("Better leverage Copilot dispatch for development work")
forward-priority-honored: cycle 143 #2 (4 minimal role prompts at prompts/v2/<role>-prompt.xml); cycle 143 #1 (audit cycle 219) NOT AVAILABLE
substrate-cycles: 34 (cycles 111-143 → cycle 144 begins role-prompt + dispatch arc)
bottleneck-async-cycles: 56 (cycles 78-143)
---

# Cycle 144 — Directive #2937 absorption

## What the directive says

Eva filed [`input-from-eva #2937`](https://github.com/EvaLok/schema-org-json-ld/issues/2937) at 2026-05-14 07:34 UTC (~1.3h before cycle 144 session-start at 08:53 UTC). The directive is: **"better leverage Copilot dispatch for development work where parallelism or external-view helps."**

Key claims from the directive (paraphrased; full text in the issue):

- Phase 1 made strong use of Copilot dispatch for primary-source research deeper-reads (PR #2804/#2809/#2805/#2875/#2874/#2873/#2876/#2877).
- Phase 2's single dispatch (#2911 adversarial critique) was load-bearing for the Q7 resolution; cycle 138 absorption acknowledged it as correct where main's analysis was structurally biased.
- **Phase 3 has produced zero Copilot dispatch across cycles 138-143** despite 4 crate SCAFFOLDs (channel-router cycle 140 / super-step-boundary cycle 141 / role-driver cycle 142 / reconciler-event-processor cycle 143) being built end-to-end by main.
- The dispatch mechanism is verified working (cycle 213 #2879-2882) and remains operational.
- The pattern "main builds tightly-coupled Rust crates directly" is defensible for any single crate, but it is not the only option, and the current cycle composition has not been considering dispatch as a live alternative.

## Five structural opportunities Eva named

1. **Parallel SCAFFOLD work.** Cycle 143's natural focal was `v2-reconciler-event-processor` SCAFFOLD. If main has other substrate-producing work that doesn't depend on reconciler completion (audit absorption, role-prompt authoring, prompt-contract-check design, gardening, _notes housekeeping), dispatching the SCAFFOLD to Copilot and doing the other work in main is a structurally reasonable composition.
2. **Adversarial critique on Phase 3 prototype designs.** #2911 dispatched after cycle 118 caught real bias; the same critique mechanism on Phase 3's role-prompt design or the 4-primitive integration surface (before cycle 144+ commits to specific role-prompt structures) would surface bias before it propagates.
3. **Role-prompt authoring (cycle 144).** Authoring 4 distinct per-role prompts (planner / executor / curator / reconciler) is naturally parallelizable across 4 dispatched sessions, each with its own role-specific brief. *"Main can author one as reference and dispatch the other 3 with a contract specifying input/output channel schemas and the reducer-rule discipline they must produce against. Or dispatch all 4 and review."*
4. **prompt-contract-check work (cycle 145+).** This is the 5th open-questioned crate; designing per-role contract checks is structurally analogous to the cycle 71/75/77 deliverables that dispatched well during Phase 1.
5. **Research carry-overs.** Phase 1 deeper-read deliverables that were named but never dispatched (or were dispatched at first-pass and never returned to) remain candidates if Phase 3 prototype work surfaces design questions the existing research base doesn't answer.

## Four prohibitions Eva named

- **Don't dispatch reflexively.** Some work genuinely benefits from main's continuous context — closely-coupled refactors, cross-crate integration, anything where dispatch overhead exceeds the work being dispatched.
- **Don't dispatch work that requires preserved-zone edits** (`.github/workflows/`, `2-selection.md`, the orchestrator prompt itself) — those are PR-required-with-Eva-merges paths, not Copilot-dispatch paths.
- **Don't dispatch when the spec for the dispatched work isn't already specified enough that the result is reviewable.**
- **Don't lose the substrate-production discipline.** The Phase 3 crates main has built are real value; the directive is to widen the toolset, not to switch tools.

## Concrete ask

*"At each cycle's forward-priorities step, include 'Copilot dispatch fit?' as one of the considerations. When fit is present, dispatch and continue substrate work in main in parallel. The two-track composition (main + 1-N dispatched sessions per cycle) is a first-class option, not an exception."*

## Cycle 144 acceptance disposition

**ACCEPTED**, with cycle 144 as the directive-acknowledging cycle and the operational test of the directive at directive-named opportunity #3 (role-prompt authoring).

**Why cycle 144 is the right cycle to test:**

- Directive landed 1.3h before cycle 144 session-start (07:34 UTC vs 08:53 UTC); the directive specifies cycle 144 work directly (item #3 names "cycle 144" by number).
- Cycle 143's named forward priority #2 was "4 minimal role prompts at `prompts/v2/<role>-prompt.xml`" — natural focal per cycle 139 ordering proposal. This is exactly the work item #3 names as canonical dispatch opportunity.
- The minimal end-to-end ordering proposal at cycle 139 lines 164-172 placed role-prompt authoring at cycle 144 specifically; the directive aligns with the existing ordering rather than disrupting it.
- Phase 3 has produced 4 SCAFFOLDs (cycles 140-143) all built in main. Cycle 144 inverting that composition for one cycle of dispatch is the smallest possible test of the directive's two-track claim.

**Why item #3 ("one as reference, dispatch the other 3") over item #2 (adversarial critique) or item #4 (prompt-contract-check) for cycle 144:**

- Item #3 is what the directive specifically names as cycle 144 fit. The other items are either pre-cycle-144 (item #2 is cycle 144- alternative; item #4 is cycle 145+).
- Item #2 (adversarial critique) is most useful AFTER the artifacts exist for it to critique; cycle 144 produces the first artifact (planner-prompt.xml), and cycle 145+ can critique the full set.
- Item #4 (prompt-contract-check) is named cycle 145+ work (5th open-questioned crate; design work for cycle 144+ after minimal end-to-end is running, per cycle 139 forward priority #5).

**Two-track plan for cycle 144:**

1. **Track A (main):** author `prompts/v2/planner-prompt.xml` as reference prompt. Planner is the architectural keystone of B (substantive-focal selection drives the multi-agent system; planner's per-role-tasks decomposition is what makes the other 3 roles do useful work). Authoring the keystone in main means the hardest design thinking happens with full main context, and gives Copilot dispatches a concrete reference to model against.
2. **Track B (Copilot, 3 parallel dispatches):** dispatch reconciler-prompt + executor-prompt + curator-prompt to Copilot with planner-prompt.xml as reference + per-role brief naming input/output channel schemas + reducer-rule discipline.

**Why planner is the reference role (not reconciler, which is structurally simplest):**

- Planner's substantive-focal decision is the highest-stakes design surface; getting it right in main anchors choices for the other 3 roles.
- Reconciler is simpler but its job (poll cron-internal sources + write inbound-channel) is highly procedural; the dispatched brief can be tight even without a reference because the work is structurally close to v2-reconciler-event-processor's CLI surface (cycle 143 SCAFFOLD).
- The other 3 roles (reconciler / executor / curator) all read from previous super-step outputs and write to a single output channel; their structural shape is similar to each other and easier to parallelize once the planner reference establishes the prompt-file format.
- B body Axis 13 commitment: "Per role, the prompt is **small** (a reference + role-specific judgment-call decisions); the bulk of procedure lives in the harness." The 4 prompts share a common structural template; authoring the keystone first then dispatching the structural-template instances is consistent.

## How the directive integrates with existing forward-priority discipline

The directive **does not invalidate** cycle 143's named forward priority #2 (4 role prompts at cycle 144). It **augments** the discipline at item #3 by adding "Copilot dispatch fit?" as a cycle composition consideration. Cycle 144 honors cycle 143 priority #2 AND tests the directive's dispatch-fit consideration at the same time — these are compatible, not competing.

**29th consecutive HONORING cycle** of named forward priority (cycles 115-144 — cycle 144 honors cycle 143 priority #2 with directive #2937 absorption integrated). **34 cycles of substrate** at cycle 144 entry (cycles 111-143). **56th consecutive bottleneck-asynchronous cycle** (cycles 78-143).

## How dispatches are tracked for cycle 145+ review

The 3 dispatched issues will appear in `docs/state.json` per the standard `tools/dispatch-task` pattern. Each will produce a Copilot-authored draft PR over the next few hours. Cycle 145 (or whenever main next runs after the dispatches return) will:

1. Read the 3 returned draft PRs.
2. Compare each against the planner-prompt.xml reference for format + structural consistency.
3. Compare each against its per-role brief for spec compliance.
4. Land the prompts via direct push to `prompts/v2/` (zone allows direct push per redesign prompt SECTION 2).
5. Close the dispatch issues with a forward-link comment per the housekeeping discipline.

If a dispatch returns a prompt that doesn't model against the reference cleanly, the path is:
- Comment on the dispatch PR with the specific divergence.
- Either: edit the PR ourselves (since direct push is allowed in `prompts/v2/`) and land it; OR re-dispatch with a tightened brief; OR write the prompt in main if the dispatch isn't producing usable output.
- Cycle 145+ adjudicates per artifact.

## Anti-overstatement audit

- Cycle 144 does NOT abandon the substrate-production discipline. Main still authors planner-prompt.xml end-to-end this cycle; the directive's prohibition #4 (don't lose the substrate-production discipline) is honored.
- Cycle 144 does NOT dispatch all 4 prompts. Eva's directive offered "one as reference, dispatch the other 3" OR "dispatch all 4 and review"; cycle 144 chooses the former for the reasons named above (reference anchors structural template; main does the hardest design thinking).
- Cycle 144 does NOT dispatch on item #2 (adversarial critique). That's deferred — most useful after all 4 prompts exist.
- Cycle 144 does NOT dispatch on item #4 (prompt-contract-check). That's cycle 145+ work per cycle 139 forward priority #5.
- Cycle 144 does NOT modify the directive #2937 issue beyond an acknowledgment comment (the issue stays open as standing context, like other `input-from-eva` directives that name ongoing constraints — see housekeeping `what-not-to-close`).
- Cycle 144 does NOT establish a new HARDENED pattern from this single directive-resolution. The directive integrates with cycle 143's already-named forward priority; this is not a directive that overrides substrate direction (cycle 138 NOVEL@1 #1 stays at NOVEL@1; cycle 144 is not a second instance because cycle 144 honors the directive AS part of cycle 143's forward priority, not as an override of it).
- Cycle 144 does NOT predict whether the 3 dispatches will return usable prompts. The directive is at first-test-of-pattern scope; cycle 145+ adjudicates per artifact.
- Cycle 144 does NOT make claims about cycle 145+ dispatch cadence. The directive's "two-track composition is a first-class option, not an exception" framing is preserved as forward consideration; each future cycle's composition is its own judgment call.

## Forward note for cycle 145+

At cycle 145 entry (or whenever main next runs), the forward-priorities consideration set is:

1. **Review 3 returned Copilot dispatches** (executor / curator / reconciler prompts). Land cleanly-modeled ones; iterate or re-author the rest.
2. **Audit cycle 219 absorption** (carry-over from cycle 144 priority #1 NOT AVAILABLE; cycle 219 may land any time).
3. **Adversarial critique dispatch on the 4-prompt set** (item #2 from directive #2937; now ripe with all 4 prompts in hand).
4. **`prompt-contract-check` design** (item #4 from directive #2937; cycle 139 forward priority #5).
5. **Cycle-runner harness rewrite** (cycle 139 forward priority #2; PR-required forbidden zone; Eva merges).
6. **First end-to-end run + first measurement** (cycle 139 forward priority #2 last item; cycle 146+).

Directive #2937 also says **"at each cycle's forward-priorities step, include 'Copilot dispatch fit?' as one of the considerations"** — cycle 145+ session-start comments should include this consideration when naming forward priorities.

---

**Author note:** This is the first cycle the orchestrator has accepted a directive that pushes against the current cycle composition discipline. Cycle 138 directive #2930 (Q7 resolution + Candidate B selection) was a *substrate-direction override* — it changed which candidate Phase 3 would build. Directive #2937 is a *cycle-composition augmentation* — it doesn't change WHAT to build, but adds a structural consideration (Copilot dispatch fit) to HOW cycles are composed. Both are legitimate directive types; the prompt section EVA-DEFAULT-AUTONOMY says orchestrator defaults to resolving issues itself except when genuinely Eva-only; cycle composition is Eva's call when she chooses to direct it, and this is one such case.
