# Cycle 63 — oh-my-codex deeper-read dispatch

**Date:** 2026-05-04
**Substantive focal activity:** option 1 from input-from-eva [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829) (deeper-read dispatch on a stub-status Phase 1 system).
**Cycle-62 provisional read carried forward:** dispatch oh-my-codex deeper-read per [#2782](https://github.com/EvaLok/schema-org-json-ld/issues/2782) — picked up this cycle as new dispatch [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) (since #2782 itself is the closed cycle-26 Copilot dispatch).

## What this document is

This is the cycle-63 process document — what was dispatched, why this scope and not another, what decision was made about complementary activity, and what cycle 64+ should pick up. It is NOT the deliverable itself (that lands at `docs/redesign/_notes/cycle-63-oh-my-codex-deeper-read.md` when the dispatched Copilot session returns).

## Cycle composition under #2829's polarity inversion

Cycle 62 was the FIRST cycle in the v1.X sequence (cycles 35-61) to do research-corpus advancement as substantive focal under #2829's polarity inversion. Cycle 63 is the SECOND such cycle. The sequence is now:

- Cycle 62: option 4 (implications mining on AutoGen) — produced 432-line `_notes/cycle-62-autogen-implications.md`
- Cycle 63: option 1 (deeper-read dispatch on oh-my-codex) — produced new dispatch [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833); deliverable awaits Copilot session return

Cold-reader cadence on `2-design-framework.md` is bounded-mechanical fallback only when no substantive option is viable. Today both #2829 substantive options were viable and option 1 was specifically named by Eva ("oh-my-codex per #2782 — never picked up"). Cold-reader is not in play this cycle.

## What the dispatch targets

The cycle-26 PR #2784 deliverable (closed-without-merge per absorption convention; lives on never-merged branch `copilot/redesign-research-phase-1-survey` at commit `f291ec05`) is a 299-line high-level survey covering 22 named patterns. The cycle-26 author **explicitly flagged three files as not-read-in-full**:

- `src/hooks/keyword-detector.ts` (44KB) — central integration point between user input and workflow mode activation
- `src/config/generator.ts` (43KB) — single assembly point for what the runtime looks like to the model
- `src/autoresearch/runtime.ts` (45KB) — closest oh-my-codex has to a fully autonomous loop

These three files are exactly the most relevant to the v2 redesign's CORE-DESIGN-PRINCIPLE (tools and deterministic processes handle repetitive procedural work; orchestrator handles novel/judgment). Code-level reading of them should reveal exactly HOW oh-my-codex's code-vs-prompts split is structured. The dispatch also targets the state/hooks/MCP-server modules and the Rust sparkshell crate, plus code-level confirm/refine of cycle-26's 22 named patterns.

Dispatch issue body lives at [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833); it follows the deeper-read template established by cycle-42's openclaw dispatch [#2808](https://github.com/EvaLok/schema-org-json-ld/issues/2808) — Why-cycle-N-and-not-earlier preamble, What's-new-since cross-references to current redesign artifacts, lens-numbered structure for Copilot output mirroring, anti-smuggling discipline pre-loaded.

## Why a Copilot dispatch and not orchestrator-direct

The per-system stub `oh-my-codex.md` flagged the deeper read as "orchestrator-direct" (line 7 of the original stub, before this cycle's edit). Eva's #2829 directive used the wording "deeper-read dispatch" as option 1, which the orchestrator interprets as "Copilot dispatch with deeper-read instructions." Either reading is defensible.

The Copilot-dispatch choice was made on these grounds:

- **Time efficiency.** A 75-minute orchestrator session can read perhaps 5-15 files in oh-my-codex with citation-grade attention; a Copilot dispatch can read substantially more in parallel because the orchestrator session can spend its time on dispatch construction + complementary work rather than on reading.
- **Context windowing.** The three flagged files alone are 132KB; reading them in full plus their callers/callees plus state/hooks/MCP-server modules plus the Rust crate exceeds what an orchestrator session can hold in working context with quality.
- **Citation quality.** Copilot dispatches return file:line citations for each claim; orchestrator-direct reads are more vulnerable to summary-shape tradeoffs.
- **Precedent.** Cycle-42 openclaw deeper read used Copilot dispatch ([#2808](https://github.com/EvaLok/schema-org-json-ld/issues/2808) → PR [#2809](https://github.com/EvaLok/schema-org-json-ld/pull/2809), 893 lines); cycle-41 cognition deeper read used Copilot dispatch ([#2802](https://github.com/EvaLok/schema-org-json-ld/issues/2802) → PR [#2804](https://github.com/EvaLok/schema-org-json-ld/pull/2804), 795 lines); cycle-41 OpenAI harness deeper read used Copilot dispatch ([#2803](https://github.com/EvaLok/schema-org-json-ld/issues/2803) → PR [#2805](https://github.com/EvaLok/schema-org-json-ld/pull/2805), 780 lines). Cycle-63 oh-my-codex follows the same pattern — three deeper-read landings in close-and-recreate sequence at cycles 41-43, plus this fourth at cycle 63.

The literal "orchestrator-direct" phrasing in the original stub was made obsolete by the cycle-41/43 close-and-recreate primitive proving that Copilot dispatches reliably produce primary-source-grounded deliverables when the firewall allowlist is current (Eva [#2794](https://github.com/EvaLok/schema-org-json-ld/issues/2794)). The cycle-63 dispatch updates the per-system stub accordingly.

## Cycle composition decision: dispatch-only substantive focal

Cycle 62's recommendation: "If the dispatch can be constructed in <10 minutes of session time, both can fit; if not, prioritize the dispatch as the substantive focal and tag cycle 63 explicitly as 'dispatch-only substantive focal' with cycle 64 picking up the complementary write activity."

Dispatch construction took ~30 minutes (orientation reads, gap analysis on PR #2784, drafting against #2808 template, opening the issue). This exceeds the 10-minute budget; cycle 63 is therefore tagged **dispatch-only substantive focal**. The complementary write activity that was on the table — LangGraph implications mining as parallel to cycle-62's AutoGen — defers to cycle 64 (or later if cycle 64 picks up something else from the #2829 substantive option list).

## Stale-reference housekeeping (bounded-mechanical add-on)

Cycle 63's gap analysis surfaced four stale references in `1-research.md` summary tables:

- **Line 80 (openclaw):** still says "First-pass: README + VISION.md" but `systems/openclaw.md` says "Status: deeper read landed (cycle 43)" with PR [#2809](https://github.com/EvaLok/schema-org-json-ld/pull/2809) at 893 lines.
- **Line 85 (Cognition Devin):** still says "Stub — deeper read pending [#2779](https://github.com/EvaLok/schema-org-json-ld/issues/2779)" but `systems/cognition-devin.md` says "Status: deeper read landed (cycle 41)" with PR [#2804](https://github.com/EvaLok/schema-org-json-ld/pull/2804) at 795 lines.
- **Line 86 (OpenAI harness):** still says "Stub — deeper read pending [#2781](https://github.com/EvaLok/schema-org-json-ld/issues/2781)" but `systems/openai-harness.md` says "Status: deeper read landed (cycle 41)" with PR [#2805](https://github.com/EvaLok/schema-org-json-ld/pull/2805) at 780 lines.
- **Line 87 (oh-my-codex):** said "Stub — deeper read pending [#2782](https://github.com/EvaLok/schema-org-json-ld/issues/2782)" — fixed this cycle to point at cycle-63 dispatch [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833).

A second table at lines 607-618 has the same stale references for Cognition (line 612), OpenAI harness (line 613), and oh-my-codex (line 614) — though that table is labeled "Further systems to study" with "Order not yet committed," so it may have been authored as a planning-time table not intended to be updated as deeper reads landed.

**This cycle's housekeeping action.** Updated lines 87 and 614 (the two oh-my-codex references) to point at the cycle-63 dispatch [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) as the in-flight successor to the closed cycle-26 dispatch [#2782](https://github.com/EvaLok/schema-org-json-ld/issues/2782). Also updated `systems/oh-my-codex.md` to (a) reflect the cycle-63 deeper read in flight, (b) note the cycle-26 PR #2784 deliverable lives on never-merged branch (a stale citation-target the original stub didn't flag), and (c) update the To-be-completed section to reference [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) and the expected deliverable path.

**Deferred housekeeping for cycle 64+.** The other three stale references (lines 80, 85, 86 in the first table; lines 612, 613 in the second table — five total edits across openclaw, Cognition, OpenAI harness) were left untouched in cycle 63. Scope discipline: cycle-63 is dispatch-only substantive focal; broader stale-reference cleanup is a housekeeping sweep that fits cycle 64+ bounded-mechanical capacity. The pattern itself (table-vs-per-system-file synchronization not maintained as deeper reads land) is a v1-style finding worth a journal note: cold-reader cadence operates on `2-design-framework.md`, not on `1-research.md`, so per-system-file updates that imply summary-table updates have been silently bypassed for ~20 cycles. v2 design-input candidate: "every artifact-mutation operation that propagates to peer artifacts should be either (a) tool-enforced or (b) checklist-acknowledged." This parallels cycle-59's row-ordering finding (implicit conventions inherited across cycles drift undetected) and cycle-61's hypothesis-counting drift finding (loose counting introduced at cycle-58 inherited without re-validation). Three independent findings now point at the same v2 design need.

## Why oh-my-codex matters specifically for v2 redesign

The cycle-26 survey already identified oh-my-codex as the strongest cross-system example of CORE-DESIGN-PRINCIPLE-in-action (substantial deterministic TypeScript code for procedural work; LLM-driven Codex tool loop handles judgment). The deeper read should reveal:

- **Axis 2 anchoring.** The current framework's Axis 2 (Code-vs-prompts substrate) cites oh-my-codex's 44KB keyword detector + 43KB generator; the deeper read should provide code-level evidence for what those files actually do, allowing the framework to anchor more precisely than "44KB suggests substantial logic."
- **Axis 6 anchoring.** Extension shape — oh-my-codex's "skills + role prompts + MCP servers + sparkshell" is a multi-surface extension model; deeper reading should distinguish which extension surface handles what kinds of capability and how they compose.
- **Axis 8 anchoring.** State scope — oh-my-codex's per-mode + per-session + root state with explicit reconciliation is uniquely complex compared to other surveyed systems; deeper reading should clarify the algorithmic structure of reconciliation and what failure modes it handles.
- **Axis 13 anchoring.** Anti-pattern integration — oh-my-codex has anti-patterns at multiple layers (CONTRIBUTING.md `<Bad>` examples, prompt-file `<Bad>` examples, hard-deprecation tags); deeper reading should surface the structural pattern of how anti-patterns flow through the system.

Phase 2 candidate authors will draw differently from oh-my-codex's deterministic-vs-LLM separation pattern depending on whether they have code-level depth or pattern-name depth. Cycle 63's dispatch closes that gap.

## What the deliverable should look like

Target: `docs/redesign/_notes/cycle-63-oh-my-codex-deeper-read.md`, 600-1200 lines, 9 lens sections (3 sections deep on the operationally-largest files; 3 sections on state/hooks/MCP; 1 on Rust sparkshell; 1 confirm/refine of cycle-26 patterns 1-22; 1 on NEW patterns visible only at code level; plus anchoring caveats). Dispatch instructions explicitly forbid v2-relevance smuggling in the patterns sections — relevance evaluation deferred to multi-system synthesis or to per-finding evaluation cycle that follows the deliverable landing.

When the deliverable lands, expected workflow per the cycle-41 cognition / cycle-43 openclaw absorption pattern:

1. **Per-finding evaluation cycle (cycle 64 if dispatch returns by then; cycle 65+ otherwise).** Walk each new finding; verify primary-source citation; classify as confirm/refine/contest of cycle-26 patterns OR as NEW pattern; integrate into `1-research.md` cross-system observations.
2. **Per-system file expansion.** `systems/oh-my-codex.md` grows from 147-line stub to deep-dive depth, mirroring `systems/openclaw.md` (cycle-43 landing pattern) and `systems/cognition-devin.md` / `systems/openai-harness.md` (cycle-41 landing pattern).
3. **Phase 2 framework update.** Axis 2 / Axis 6 / Axis 8 / Axis 13 positions for oh-my-codex updated with code-level evidence; framework version bump (v1.22 → v1.23 or higher depending on findings magnitude).

## What I couldn't figure out this cycle

**The right scope for "broad enough to surface NEW patterns vs. focused enough to deliver in one PR."** The dispatch asks for 600-1200 lines covering 9 lens sections. Cycle-26 was 299 lines on 7 lenses. Cycle-42 openclaw was 893 lines on 7 lenses. The 600-1200-line target tries to be deeper than cycle-26 without exceeding the cycle-42 magnitude. Whether that lands in the right place depends on what the three flagged files actually contain — if `keyword-detector.ts` is 44KB of repetitive lookup tables, the lens-1 deep dive may be short; if it's 44KB of layered control logic, the lens-1 deep dive may need 400 lines on its own. Copilot will scope to fit; the magnitude target is guidance.

**Whether the cycle-26 pattern set (22 named patterns) is actually the right baseline for confirm/refine/extend.** Cycle-26 grouped patterns under high-level lenses; the deeper read may surface that the patterns as named conflate distinct sub-patterns (e.g., "behavioral prompt-contract regression tests" might be 3 distinct test patterns). The dispatch instructs Copilot to use cycle-26 numbering for cross-reference but allows refining the pattern statements. Whether this produces a cleanly mappable verdict per pattern or a substantially-renumbered new pattern set will depend on what the code reveals.

**Whether the v2 framework's Axis 13 (Anti-pattern integration) actually has a coherent oh-my-codex position to anchor or whether oh-my-codex's anti-patterns are too scattered to anchor.** The cycle-26 survey identified anti-patterns at multiple layers but didn't map them as a structural pattern — they're a list of locations. The deeper read should clarify whether anti-patterns are a first-class surface (a registry, a contract, a documented invariant set) or whether they're emergent from accumulated `<Bad>` examples without a unifying structure. The framework axis position depends on the answer.

## Provisional read for cycle 64

**Default:** continue research-corpus advancement under #2829 polarity. Concrete options (all from #2829's option list):

- **Option 4 (implications mining), system: LangGraph.** Parallel to cycle-62's AutoGen implications. LangGraph has unique singular-voice patterns: Pregel super-step model, pending-writes-for-failed-super-steps, typed-channels-with-per-key-reducers. ~30-40 minutes for a quality writeup matching cycle-62's structure. **Highest-priority candidate for cycle 64.**
- **Option 4 (implications mining), system: openclaw.** Cycle-42's deeper read (893 lines) hasn't been implications-mined as a singular voice. Multi-channel Gateway, plugin-promotion-bar, sandbox allowlist/denylist patterns are uniquely openclaw. Lower priority than LangGraph because cycle 62 already focused on a Phase 1 deep-dive system; LangGraph is the natural next pair.
- **Option 2 (cross-system synthesis), pair: AutoGen + LangGraph.** Both have cycle-62 / cycle-64-pending implications writeups; pairing them as a synthesis ("what differs about super-step vs message-passing actor models for v2") would extract the cross-system signal. Best done after both implications writeups exist; defer to cycle 65+.
- **Option 5 (audit framework against fresh evidence).** Pick a v1 incident (e.g., the cycle 524 corruption that motivated git-safety primitive) and run it through the current framework v1.22; identify silent or wrong axes. Useful but lower urgency than the implications writeups, which feed Phase 2 candidate generation more directly.
- **Bounded-mechanical housekeeping:** complete the stale-reference cleanup on `1-research.md` summary tables (5 remaining edits across openclaw, Cognition, OpenAI harness). Bounded; can fit alongside any of the substantive options above.

**Recommended:** cycle 64 = LangGraph implications mining (option 4) + bounded-mechanical stale-reference cleanup (housekeeping). Mirrors cycle-62 + cycle-63 pattern of substantive-focal-plus-bounded-mechanical-add-on, sustaining #2829 polarity discipline at quality.

## What this cycle achieved

- Surfaced and dispatched the long-pending oh-my-codex code-level deeper read; #2833 is in flight with a 9-lens, 600-1200-line target.
- Surfaced a broader stale-reference pattern in `1-research.md` summary tables (4 stale references; 1 fixed this cycle, 3 deferred to cycle 64+ as bounded housekeeping).
- Produced this process document tracking decisions, rationale, and provisional read for cycle 64+.
- Validated cycle-62's dispatch-only-substantive-focal recommendation (dispatch construction took ~30 minutes; complementary write activity correctly deferred).
- Sustained #2829 polarity discipline at the SECOND consecutive cycle of research-corpus advancement.

## What this cycle didn't achieve

- Did not produce a new implications writeup (deferred to cycle 64+).
- Did not complete broader stale-reference cleanup on `1-research.md` (deferred to cycle 64+ housekeeping).
- Did not advance Phase 2 candidate generation directly (Phase 2 still gates on post-retrospective checkpoint per #2829's "What this does not change" section).

## Iteration-until-approval reflection

Cycle 63 advances research-corpus depth at exactly the surface area #2829 named ("deeper read pending #2782 — never picked up"); it operates on the substantive-focal lever rather than the framework-polish lever; it surfaces a finding (1-research.md table-vs-per-system-file desynchronization across 4 systems) that cold-reader on `2-design-framework.md` could not have caught because cold-reader doesn't operate on `1-research.md`. This is genuine iteration-until-approval activity at higher value-density than the cycles 50-60 framework-polish work, validating Eva's polarity diagnosis a SECOND consecutive cycle.

The dispatch deliverable will return in a future cycle; cycle 63's contribution is the dispatch construction quality (sharper than cycle-26's because it targets specific gaps cycle-26 explicitly flagged) plus the stale-reference housekeeping byproduct. Future readers (Eva at post-retrospective checkpoint, Phase 2 candidate authors) will see the cycle-63 dispatch as the closing of the asymmetry between oh-my-codex (stub) and other surveyed systems (deep-dive depth) — symmetric pattern with cycle-43 (openclaw) and cycle-41 (cognition + OpenAI harness).
