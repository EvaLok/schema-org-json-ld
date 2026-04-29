# Cycle 26: Three Phase 1 dispatches (Cognition Devin + OpenAI harness-engineering + oh-my-codex) + fresh-eye cold-reader on cycle-25 elevations

Cycle 25 (commit `e089a33f`) elevated 3 single-system observations to 2-system convergence in `1-research.md` (memory as first-class architectural concept; small fixed team with explicit role-separation; per-agent model selection as architectural primitive), tightened the section-opening rules, codified the discipline-lightening rule, and started the audit retrospective interleave-read. Cycle 26 took the focal task (Cognition Devin), interleave (harness-engineering), and dispatch (oh-my-codex) per the cycle-25 plan — but pivoted both originally-orchestrator-direct reads to Copilot dispatches because of an environmental constraint, then ran the bounded-mechanical fresh-eye cold-reader on the cycle-25 elevations. Cycle-27+ pre-commits at the bottom.

## Item 1 (planned focal): Cognition Devin orchestrator-direct read — PIVOTED to dispatch

The cycle-25 plan named Cognition Devin orchestrator-direct read as the cycle-26 focal. The intended mechanism was direct fetch of cognition.ai/blog and related Cognition writeups from this orchestrator session. Both `WebFetch` and `curl` are gated in the redesign-mode session; the orchestrator-direct mechanism is therefore unavailable in this environment.

**Decision:** pivot to Copilot research-only dispatch. The destination (Phase 1 read of Cognition Devin) is unchanged; only the mechanism shifted from orchestrator-direct to Copilot-dispatch. Standard cycle-15 procedure (research-only label; 7 lenses; anti-smuggling discipline pre-loaded; deliverable as single new file under `_notes/`). Defensible per `EVA-DEFAULT-AUTONOMY` (resolve environmental issues without filing question-for-eva when a viable alternative exists).

**Trade-off acknowledged.** Orchestrator-direct read affords tighter integration with the orchestrator's first-pass framing and immediate cross-system synthesis; Copilot dispatch decouples reading from synthesis and adds 1-3 cycles of integration latency. The decoupling is non-trivial: cycle 17's Voyager orchestrator-direct read produced ~190 lines of integrated `1-research.md` content within the same cycle, while cycle 18's LangGraph dispatch took cycles 18-20 (dispatch + lands + Tier-1 integration). For a focal read with high architectural relevance like Cognition Devin, the lost-tightness is real. Pivot was forced by environment, not preferred.

Dispatch: issue [#2779](https://github.com/EvaLok/schema-org-json-ld/issues/2779) (Copilot WIP PR auto-created at #2780).

## Item 2 (planned interleave): OpenAI harness-engineering writeup orchestrator-direct read — PIVOTED to dispatch

Same constraint as item 1. The harness-engineering writeup is at openai.com/index/harness-engineering/, not directly fetchable in this orchestrator session. Same pivot rationale.

The harness-engineering read is structurally distinct from the Cognition Devin read: Devin is a hosted product with multiple writeups across Cognition's blog (blog-shape AND product-page-shape); harness-engineering is a single OpenAI writeup with possible related Codex / agents-API material. The dispatch lenses are calibrated differently — harness-engineering centers on what OpenAI argues a "harness" is and how it should be structured; Cognition centers on philosophical positions and architectural anti-patterns.

Dispatch: issue [#2781](https://github.com/EvaLok/schema-org-json-ld/issues/2781).

## Item 3 (planned dispatch): oh-my-codex Copilot research-only dispatch

This was already a planned dispatch (Eva #2774). Standard cycle-15 procedure. Code+docs read of `Yeachan-Heo/oh-my-codex`, calibrated for a small/medium personal project (likely a wrapper around OpenAI's Codex CLI tool).

Dispatch: issue [#2782](https://github.com/EvaLok/schema-org-json-ld/issues/2782).

## Three-dispatches-in-one-cycle: capacity rationale

Cycle 25's plan had 1 dispatch + 2 orchestrator-direct reads. The pivot makes it 3 dispatches. This is more aggressive than cycles 15, 16, 18, 20 (each had 1 dispatch maximum). The redesign prompt's `AUTHORITY` section explicitly says "Capacity limits are your judgment call during this phase — there is no enforced 2-slot cap"; Eva-directives #2774 and #2775 explicitly authorize all five queued reads.

**Cost vs benefit.** Three dispatches consume more Copilot API budget than one (likely ~$5-10 across all three sessions). The benefit: three systems' research deliverables land in parallel rather than serially, allowing cycle 27-29 to do per-system Tier-1 evaluation and integration concurrently. Compared to a single-dispatch-per-cycle cadence stretched across cycles 26-28, the parallel approach completes the dispatch-and-land phase ~2 cycles faster.

**Integration backlog.** The three PRs landing in cycles 28-30 (typical 2-3 day Copilot turnaround) means cycle 29-31 will have substantial Tier-1 integration work. This is acceptable: cycle 28-31 will be primarily integration cycles, with some new pre-commits as 6th-7th-8th-system data adds cross-system convergence opportunities.

**Risk mitigation.** Each dispatch has the same standard cycle-15 anti-smuggling discipline pre-loaded; the deliverables are isolated (one new file per dispatch in `_notes/`; no other file modifications). If one dispatch produces low-quality output, the other two are unaffected. The dispatches are mutually independent.

## Item 4 (bounded mechanical): Fresh-eye cold-reader on cycle-25 elevations

Per cycle-25 pre-commit 6: fresh-eye pass on the three new patterns (5: memory as first-class; 6: small fixed team; 7: per-agent model selection) and the section-opening tightening, addressing three specific questions:

(a) Does the convergence-with-acknowledged-asymmetry framing read load-bearing-balanced or hedge-heavy across patterns 5-7?
(b) Is the per-agent model selection pattern's citation chain (extending into PR #2763) sufficiently transparent?
(c) Does the section-opening tightening's "60% bar at 5 systems" framing read as observation or as rule-statement that smuggles forward-looking expectation?

### Question (a): hedge-balance — PASS, load-bearing-balanced

Each of patterns 5-7 closes with the same shape: convergence on X; asymmetry/divergence on Y.

- Pattern 5: "Convergence on architectural-elevation; asymmetry on framing-vocabulary."
- Pattern 6: "Convergence on small-fixed-team-with-named-roles; divergence on orchestration topology (peer-flow vs lead-worker-hierarchy)."
- Pattern 7: "Convergence on per-agent-model-selection as architectural-primitive; asymmetry on rationale (Voyager foregrounds cost-tiering motivation; AutoGen documents the architectural flexibility without prescribing cost-tiering or any other usage rationale, leaving the choice to application operators)."

The pattern shape is consistent and load-bearing-balanced. Each closing names the SHAPE of the asymmetry (vocabulary, topology, rationale) — symmetric with the SHAPE of the convergence. Not hedge-heavy: the asymmetries are first-class observations, not hedges to discount the convergence claim.

Pattern 7's closing is the longest (parenthetical detail about Voyager-vs-AutoGen rationale framing); pattern 5's is shortest. The variation reflects depth-of-rationale-asymmetry: pattern 7 has more to say about the asymmetry shape because rationale framings are more language-shaped than topology or vocabulary asymmetries are. Defensible.

**Verdict:** load-bearing-balanced. No edits proposed.

### Question (b): citation-chain transparency — NEUTRAL, matches section convention

Pattern 7's claim "each AssistantAgent takes its own `model_client`" is grounded in the AutoGen research deliverable (PR #2763, `_notes/cycle-15-autogen-research.md`), not directly in `1-research.md`'s AutoGen section. The AutoGen section in `1-research.md` does name "Extensions API (model clients, tools, code execution)" (line 244), which supports the layer-abstraction claim, but the per-agent constructor coupling is one citation-layer deeper.

**The same citation-chain extension applies to other patterns in the cross-system observations section.** Pattern 4 ("Failed work as recorded artifact") cites `failed_tasks.json` (Voyager) and `WRITES_IDX_MAP` constants (LangGraph) — these constants and file paths are cited in their respective per-system sections in `1-research.md`, but the verification of "what Voyager / LangGraph actually does with these" extends into substrate code reading or the per-system deliverables in `_notes/`. Pattern 3 ("Component-local state persistence") cites "AutoGen state save/load is component-local dictionaries" — also implicitly grounded in the AutoGen substrate.

**The implicit convention** of the cross-system observations section is: claims summarize what the per-system sections (and their substrate notes) document; the per-system sections are the proximal evidence; the substrate notes (`_notes/cycle-N-<system>-research.md`) are the ultimate evidence base. Citation chains naturally extend from observation → per-system summary → substrate notes → primary source.

**If pattern 7 were singled out for citation-chain transparency** (e.g., adding "(see also PR #2763 / cycle-15-autogen-research.md)" parenthetically), this would create asymmetric transparency relative to patterns 3 and 4. The right move is either (i) leave the convention as-is across all patterns; (ii) define a section-level citation-chain convention applied to all 7 patterns, not just pattern 7.

**Verdict:** the citation chain extends, but that's consistent with the section's overall convention. Defer to a section-level convention question; don't apply asymmetric transparency to one pattern.

### Question (c): section-opening "60% bar at 5 systems" framing — PASS

The section-opening text:
> "The 3+ threshold creates a 60% bar at 5 systems read; if a 6th system supports a 2-system pattern, this rule elevates it to 3+ mechanically."

Read carefully:
- "60% bar at 5 systems read" — arithmetic observation (3/5 = 60%). Observation, not rule-statement.
- "if a 6th system supports a 2-system pattern, this rule elevates it to 3+ mechanically" — IS rule-statement, but states the threshold rule, not forward-looking expectation. The phrasing is bilateral: applies whether the 6th system DOES or DOES NOT support the pattern. The rule may not fire. Doesn't smuggle "we expect 6th-system support."

**Verdict:** the framing reads as observation/rule-description, not forward-looking expectation. PASS.

### Verdict across (a)/(b)/(c): no edits applied

The cycle-25 elevations and section-opening tightening pass fresh-eye on all three specific questions. The two cycle-25 self-uncertainties (citation-chain transparency on pattern 7; whether 7 patterns at 2-system tier is "saturated") remain — but they are structural questions about the section's convention rather than per-pattern problems. They become cycle-27+ candidates if a structural decision is needed.

**One observation worth recording for future fresh-eye passes:** the cross-system observations section now operates with an implicit convention (observation → per-system summary → substrate notes). Future cold-readers should examine this convention as a whole, not just per-pattern citations. If a reader needs to verify "each AssistantAgent takes its own `model_client`," the path is: pattern 7 → AutoGen section → cycle-15-autogen-research.md → AutoGen quickstart. That path works but is implicit. A section-level note ("Substrate evidence is in the per-system Phase 1 sections above and in `_notes/cycle-N-<system>-research.md` for each system") would make the convention explicit. Cycle-27+ candidate — but not urgent.

## Same-cycle cold-reader on cycle-26 work

The cycle-25 codified discipline-lightening rule applies: bounded mechanical edits → 30-second self-check; substantive rewrites OR architecturally-load-bearing edits → full structured pass.

Cycle 26's primary work products:
- 3 dispatch issues (issues #2779, #2781, #2782): bounded mechanical (filling templated dispatch bodies). 30-second self-check applies.
- This notes file: substantive prose, but it's a cycle-N record / recap, not a structural change to redesign artifacts. 30-second self-check applies.
- Fresh-eye cold-reader (item 4 above): substantive in scope but PRODUCES no edit — the cold-reader concluded no edits warranted. No structured cold-reader-on-cold-reader needed.

**30-second self-checks:**

- Dispatch issue bodies: each is calibrated to its system's read-shape (blog-shape for Cognition Devin and harness-engineering; code+docs for oh-my-codex). The 7 lenses are calibrated per-dispatch (e.g., harness-engineering lens 2 is "Harness components and structure" rather than "State representation, persistence, and time-travel" because harness-engineering centers on harness structure rather than state persistence). Anti-smuggling discipline pre-loaded ("do NOT smuggle v2-relevance framings"). PASS.
- This notes file: claims about cycle-25 elevations are verified against `1-research.md` lines 822-833 (section opening) and 928-973 (patterns 5-7). Citations are correct. The fresh-eye cold-reader verdicts are observation-shaped (PASS / NEUTRAL / PASS), not v2-prescriptive. PASS.
- One concern caught in the dispatch bodies: each dispatch instructs Copilot "Do NOT smuggle v2-relevance framings into the 'Patterns observed' section." This is correct per cycle-15 procedure. Dispatches do NOT instruct Copilot to apply v2-relevance framings; they explicitly forbid them. Verified.

PASS across all three work products. No edits required.

## Long-deferred items roll-call (carry-forward)

1. Journal-entry self-congratulation sweep (20 cycles deferred from cycle 7+)
2. F6/F8/F9 measurements (cycle 7+)
3. Refactor-for-length F-section sweep (cycle 8+; Tier-2 group 8)
4. Persistence-mechanism deferred-list consolidation (cycle 13 flag)
5. Tier-2 group 2 (review/disposition substrate, partial integration cycle 13)
6. Tier-2 group 4 (nine measures rework)
7. Tier-2 group 6 (preserved-through-cutover disposition)
8. Tier-2 group 7 (resolved open questions collapse)
9. Tier-2 group 9 (F8 singleton-family acknowledgment)

Net: 9 → 9 unchanged. Cycle 26 is Phase 1 dispatch + cold-reader work, not Phase 0 long-deferred items.

## Cycle 27+ pre-commits

1. **Cognition Devin dispatch evaluation + Tier-1 integration** (issue #2779). When PR lands (typical 2-3 days), evaluate per cycle-7/12/15-style per-finding evaluation. Tier-1 integration into `1-research.md` if findings warrant. Tier-2 integrations as new pre-commits. **Likely cycle-28 or cycle-29 work.**

2. **OpenAI harness-engineering dispatch evaluation + Tier-1 integration** (issue #2781). Same pattern as item 1. **Likely cycle-28 or cycle-29 work.**

3. **oh-my-codex dispatch evaluation + Tier-1 integration** (issue #2782). Same pattern. **Likely cycle-28 or cycle-29 work.**

4. **Copilot research-only dispatch: oh-my-claudecode** (Eva directive #2774, github repo). Standard cycle-15 procedure. **Earliest dispatch: cycle 27.** Defer if cycle-26's three dispatches haven't landed by cycle 27 (avoid 4 PRs in flight simultaneously); release once cycle-26 dispatches start landing.

5. **Copilot research-only dispatch: openai/symphony** (Eva directive #2775, github repo). Standard cycle-15 procedure. **Earliest dispatch: cycle 27.** Same gating as item 4.

6. **Cross-validate main's F-family taxonomy against audit's A-pattern mapping** (cycle-25 pre-commit 7, deferred). Bounded mechanical (~1 cold-reader cycle). **Cycle 27 candidate** if no PRs land yet (pre-evaluation work).

7. **Read remaining audit retrospective sections** (cycle-25 pre-commit 8, deferred). "What appears to be working" + "What might appear to work but probably doesn't" + "Open questions" + "What should be preserved through cutover" + "What v2 must demonstrably do better than v1" + "Iteration plan" + "Iteration log". The "What v2 must demonstrably do better" section is potentially most relevant for Phase 2. **Cycle 27 candidate** if no PRs land yet.

8. **Same-cycle cold-reader on this notes file** (standard cycle-N+1 fresh-eye pass). Specific questions: (a) does the three-dispatches rationale read as honest pivot or as over-justified scope expansion? (b) does the fresh-eye cold-reader's NEUTRAL verdict on citation-chain transparency match what a reader unfamiliar with the cross-system observations section convention would conclude? (c) does the "30-second self-check" framing accurately apply to substantive cycle-N record / recap notes (this file), or is the line between "bounded mechanical" and "architecturally-load-bearing" blurrier than the cycle-25 codified rule allows?

9. **Citation-chain convention for cross-system observations section** (new from cycle-26 fresh-eye verdict on question (b)). Either (i) leave the implicit convention as-is and not flag it, or (ii) add a section-level note explaining how citation chains extend through per-system summaries to substrate notes. Architecturally-load-bearing decision affecting all 7 cross-system patterns. **Cycle 27+ candidate**, low priority unless a reader external to the project (audit, Copilot feedback) flags the implicit convention as opaque.

10. **Description-shape-asymmetry escalation watch** (cycle-23 carry-forward; no action this cycle). If Phase 2 candidate-authors quote the blockquote standalone, escalate to blockquote-pointer addition.

### Suggested cycle 27 plan (provisional)

If no cycle-26 dispatches land by cycle 27:
- **Focal:** item 6 (F-family cross-validation against audit's A-pattern mapping) OR item 7 (read remaining audit retrospective sections)
- **Bounded mechanical:** item 8 (cold-reader on this notes file)
- **Defer:** items 4-5 (oh-my-claudecode / openai/symphony dispatches) until cycle-26 dispatches start landing

If 1-3 cycle-26 dispatches land by cycle 27:
- **Focal:** evaluate landed PR(s) (items 1-3 above)
- **Bounded mechanical:** item 8
- **Defer:** items 4-9 to cycle 28+

### Suggested cycle 28+ plan (provisional)

Depends on which dispatches landed and when:
- Continue per-system Tier-1 evaluation as PRs land
- Reach cross-system synthesis at 8 systems read once all three new systems integrated
- Items 4-5 dispatches activated when integration backlog clears

## Persistence-mechanism observations

The cycle-N-pre-commits-cycle-N+1-checks chain extends to twenty-one cycles deep (cycle 7 → ... → 25 → 26 → 27 pre-committed). No breakdown.

**The "pivot focal mechanism" pattern is new this cycle.** Cycle-25 plan had Cognition Devin as orchestrator-direct read; environmental constraint (WebFetch / curl gated) forced pivot to dispatch. The pivot:
- Preserves the destination (Phase 1 read of Cognition Devin)
- Changes only the mechanism (orchestrator-direct → Copilot-dispatch)
- Acknowledges the trade-off (lost-tightness; integration latency; integration backlog)
- Defers responsibility to cycle 27+ for evaluation and integration

This is a new persistence-mechanism observation: when environmental constraints force mechanism changes, document the pivot rationale in the cycle's notes file, preserve the pre-commit's destination unchanged, and defer the "completed" framing until the new mechanism's deliverable lands. The cycle-25 plan was right in destination, wrong in mechanism — this is itself useful information for future cycle plans (don't over-specify mechanism when destination is the load-bearing claim).

**The three-dispatches-in-one-cycle decision is new this cycle.** Prior cycles ran 1 dispatch maximum; cycle 26 ran 3. Capacity rationale (no enforced cap; per-dispatch cost reasonable; parallelism speeds up completion of the dispatch-and-land phase by ~2 cycles). Risk mitigation (independent dispatches; standard anti-smuggling pre-loaded; isolated deliverables). The decision is reversible: if any dispatch produces low-quality output, the others are unaffected; if all three produce noisy output, that's a cycle-27 evaluation problem, not a cycle-26 commitment problem.

The honest-hedge pattern (named cycle 24, 4/4 stable across cycles 24-25) was not directly invoked this cycle — no new cross-system patterns were elevated. The pattern remains stable, awaiting cycle-28-30 work when the new dispatches' deliverables enable cross-system evaluation against the existing 7 patterns.

The discipline-lightening rule (codified cycle 25) was applied this cycle: bounded mechanical / 30-second self-check on the dispatches and this notes file; full structured pass not warranted because the cycle's substantive work (fresh-eye cold-reader) PRODUCED no edit. Tally extended: substantive rewrites 5/5 hits; bounded mechanical 0/3 hits (cycles 23, 24 + cycle 26 dispatches); architecturally-load-bearing section-opening 1/1 hits.

The cycle-22 single-system observations file artifact-consistency check (verified mid-cycle): all three cycle-25 elevations ARE marked in `_notes/cycle-22-cross-system-synthesis.md` with "**ELEVATED to 2-system convergence cycle 25**" headers + "**Cycle-25 update:**" rationale paragraphs. No artifact drift between `1-research.md` (current 7 patterns) and `_notes/cycle-22-cross-system-synthesis.md` (original single-system observations + elevation tracking).

Long-deferred items: 9 → 9 unchanged. Cycle 26 is Phase 1 dispatch and bounded-mechanical work, not Phase 0 long-deferred items.
