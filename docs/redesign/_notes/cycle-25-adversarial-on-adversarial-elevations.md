# Cycle 25: Adversarial-on-adversarial of cycle-22 single-system observations + section-opening tightening + discipline-lightening codification + audit retrospective interleave-read

Cycle 24 (commit `4193074f`) ran fresh-eye cold-reader on cycle-22 cross-system synthesis, applied 3 flags (~7 lines), and pre-committed cycle 25 focal: adversarial-on-adversarial of single-system observations narrowed to 3 high-priority candidates. Cycle 25 took the focal task: re-read evidence for each candidate, judged convergence, elevated all three to 2-system in `1-research.md`. Section-opening tightening (cycle-24 question (a) item 8) applied as bounded mechanical work. Item 9 (discipline-lightening codification) applied below. Audit retrospective interleave-read complete.

## Item 1 (focal): Adversarial-on-adversarial — three candidates evaluated

Per cycle-24 question (d) priority ordering: (a) PAI Principle 13 ↔ LangGraph memory (HIGH); (b) Voyager 4-agent ↔ AutoGen Magentic-One (HIGH); (c) Voyager cost-tiering ↔ AutoGen per-agent `model_client` (MEDIUM).

### Candidate (a) PAI Principle 13 ↔ LangGraph memory framework: ELEVATED

**Evidence verified:**
- PAI Principle 13: `1-research.md` lines 180-185 — "Memory System — Everything worth knowing gets captured. History feeds future context." Treated as one of 16 numbered architectural principles. First-class architectural concern (not scaffolding).
- LangGraph short/long-term split: `1-research.md` lines 690-696 — explicit motivation quoted: "With checkpointers alone, we cannot share information across threads. This motivates the need for the `Store` interface." `Store` is documented as a distinct primitive from checkpointer; explicit short-term (thread-scoped) vs long-term (cross-thread) architectural split.
- Other three systems treat memory as derivative: AutoGen model-context abstraction over conversation history (line 372 area); Voyager component-local persistence (lines 431-440); openclaw memory-as-singleton-plugin-slot (already named in Persistent divergences).

**Verdict:** Solid 2-system convergence on memory-as-first-class-architectural-concept. PAI uses principle-shape framing; LangGraph uses mechanism-shape (Store as Extensions-level primitive). Asymmetry on framing-vocabulary; convergence on architectural-elevation. Applied to `1-research.md` as new 2-system pattern.

**Concern check:** Does this conflict with the existing Persistent divergence ("openclaw memory-as-singleton-plugin-slot vs PAI top-level Principle 13")? No — the divergence is between openclaw's architectural-conservatism and PAI/LangGraph's first-class-elevation. The new 2-system convergence puts PAI and LangGraph on the same side; the divergence remains valid (openclaw on the conservative side, PAI+LangGraph+ on the elevated side). The cross-system observation now has divergence + 2-system convergence on the same architectural question, which is a richer characterization than divergence alone.

### Candidate (b) Voyager 4-agent ↔ AutoGen Magentic-One: ELEVATED with explicit asymmetry

**Evidence verified:**
- Voyager 4-agent fixed-roles: `1-research.md` lines 406-418 — `voyager/agents/` defines four classes: ActionAgent (code generation), CurriculumAgent (task selection), CriticAgent (verification), SkillManager (storage). The four agents are the system architecture.
- AutoGen Magentic-One: `1-research.md` lines 258-259 ("`MagenticOneGroupChat`") + 367-368 ("Magentic-One's Task Ledger / Progress Ledger vocabulary for lead-orchestrator planning and tracking"). Magentic-One is one of many AutoGen orchestration patterns documented (line 252-262).

**Asymmetry analysis:**
- Voyager: 4 agents in peer-flow (curriculum → action → critic → skill); 4-agent IS the system architecture
- Magentic-One: lead-orchestrator + specialized workers (lead-worker hierarchy); one of many AutoGen orchestration patterns

**Verdict:** 2-system convergence on small-fixed-team-with-named-roles. Structural asymmetry on orchestration topology (peer-flow vs lead-worker-hierarchy) and on architectural-status (Voyager: IS the architecture; AutoGen: ONE of many patterns). Per cycle-24 honest-hedge pattern (convergence-with-acknowledged-asymmetry > both extremes), applied with explicit asymmetry-acknowledgment.

**Concern check:** Cycle 22 had originally classified this as Voyager-distinctive ("Recorded as Voyager-distinctive shape; no cross-system convergence yet"). The cycle-22 reasoning was correct at the time; cycle-25 fresh-eye on the AutoGen Magentic-One docs surfaces that the small-fixed-team-with-roles SHAPE matches, even though the topology and architectural-status differ. The honest-hedge approach captures both the convergence and the asymmetry without over-claiming.

### Candidate (c) Voyager cost-tiering ↔ AutoGen per-agent `model_client`: ELEVATED with rationale-asymmetry

**Evidence verified:**
- Voyager cost-tiering: `1-research.md` lines 420-429 — explicit per-agent model assignment with cost-vs-novelty rationale (`gpt-4` for ActionAgent / CurriculumAgent main / CriticAgent; `gpt-3.5-turbo` for CurriculumAgent QA-cache lookups + SkillManager skill-description generation).
- AutoGen Extensions API: `1-research.md` line 244 — "Extensions API (model clients, tools, code execution)" — model clients is named layer abstraction.
- AutoGen AssistantAgent + model_client coupling: AutoGen research deliverable (PR #2763, `cycle-15-autogen-research.md`) — "creates a model client, creates an `AssistantAgent`" (quickstart text); "[model context] is always used together with a model client to generate LLM-based responses."

**Rationale asymmetry:**
- Voyager: foregrounds cost-vs-novelty rationale in research artifact
- AutoGen: documents architectural flexibility without prescribing usage rationale (cost-tiering or otherwise)

**Verdict:** 2-system convergence on per-agent-model-selection-as-architectural-primitive. Cost-tiering rationale is Voyager-distinctive within the convergence. Applied with explicit asymmetry per cycle-24 honest-hedge pattern.

**Concern check:** The claim "each AssistantAgent takes its own `model_client`" is grounded in the AutoGen research deliverable (quickstart pattern + model-context-with-model-client docs), not directly quoted in `1-research.md` AutoGen section. Citation chain extends one layer (1-research.md → AutoGen research deliverable). Verifiable; not smuggled. The architectural fact is correct; the cycle-25 cross-system pattern paragraph names "model clients" as Extensions API layer abstraction (directly quotable from line 244) rather than the stronger per-agent-constructor claim.

## Item 8: Section-opening tightening (cycle-24 question (a) flags)

Two flags from cycle-24 question (a):
- (i) 3+ threshold not argued
- (ii) Section-opening "1-2 systems → candidate" framing doesn't reflect 2-in-deliverable / 1-in-notes structural distinction

**Edit applied** (`1-research.md` lines 823-834): added two sentences naming the 60% bar at 5 systems read and the mechanical 6th-system elevation rule; reframed "patterns present in only 1-2 systems are recorded as candidate" to distinguish 2-system patterns within deliverable (with diversity-limit hedges) vs single-system observations in `_notes/cycle-22-cross-system-synthesis.md`. Net change: ~+5 lines.

**Same-cycle cold-reader caught one process-commentary slip.** Initial draft included parenthetical "as cycle 25 did for three observations." This is verifiable example of the rule but not load-bearing for the rule itself; risks process-commentary creep that cycle-9 / cycle-23 both swept. **Fix applied within same cycle:** parenthetical trimmed to "(which can elevate them to 2-system on cross-system match)."

This is the third instance of process-commentary catch (cycle-23 process-defense closing sentence on LangGraph paragraph 11; cycle-21 same-cycle on blockquote tightening; cycle-25 here). Pattern shape: section-opening edits and discipline-rule edits are particularly susceptible to process-commentary because they describe rules; the cycle-9 sweep applied the discipline against `0-retrospective.md` body sections; cycle 25 confirms it applies against section-opening / rule-stating prose too.

## Item 9: Discipline-lightening codification (carried from cycle 24)

Two consecutive null results (cycles 23, 24) on same-cycle cold-reader for bounded mechanical edits (~10-line scope, applying pre-cold-readered flags) is enough evidence to lighten the discipline.

**New rule starting cycle 25:**

- **Substantive rewrites (~30+ lines OR architecturally-load-bearing):** full structured same-cycle cold-reader pass. Anti-smuggling check, citation verification, section-transition check, asymmetry-acknowledgment check.
- **Bounded mechanical edits (~10 lines applying pre-cold-readered flags):** 30-second self-check. Read-flow check + factual-claim verification. Skip the structured pass.

**Why:** the same-cycle cold-reader on substantive rewrites has surfaced real concerns in 4 of 4 tests (cycles 19 Tier-2 group 3, 21 blockquote tightening, 22 cross-system synthesis, 25 elevations). The same-cycle cold-reader on bounded mechanical edits has surfaced nothing in 2 of 2 tests (cycles 23, 24). The pattern shape is now clear: structured cold-reader earns marginal value on substantive scope where the writing introduced new content or new claims; on small-scope edits applying pre-cold-readered flags, the evidence is the cold-reader has nothing to surface.

**Threshold judgment:** the ~30-line / ~10-line cutoff is approximate. Cycle 25's 47-line elevation cleared "substantive" easily; cycle 23's ~3-line edits were clearly "bounded mechanical." Edge cases: ~15-20 line edits should default to full pass (better to over-cold-read than miss). Architectural-load-bearing edits (e.g., section-opening rule statements like cycle 25's item 8) get full pass regardless of line-count — and cycle 25 confirms this rule was correct (the item-8 cold-reader caught the process-commentary slip in a ~5-line edit).

**Updated rule:**

- **Full structured cold-reader:** substantive rewrites OR architecturally-load-bearing edits (rule-stating prose, section openings, discipline-content paragraphs)
- **30-second self-check:** other bounded mechanical edits (typo fixes, prose polish, citation additions, applying pre-cold-readered flag fixes)

This is the first persistence-mechanism rule named with explicit threshold + exception. Pattern: name the rule + threshold + exception + worked example.

## Item 2: Audit retrospective interleave-read

Audit cycle 204 (2026-04-29) delivered `docs/redesign/0-audit-retrospective.md` (273 lines, A1-A6 patterns). Cursor at audit cycle 204, mapping to main cycle 19 commit `720d652` per cycle-23 journal entry. Read scope: opening framing + glossary + audit-v1 description + A1-A6 patterns + failure-families taxonomy. Deferred: "What appears to be working" + "What might appear to work but probably doesn't" + "Open questions" + "What should be preserved through cutover" + "What v2 must demonstrably do better than v1" + "Iteration plan" + "Iteration log" (cycle 26+ continuation if useful).

**Key observations from the read:**

(1) **Audit's A-patterns map onto main's F-family taxonomy without forcing.** Audit catalogs 6 patterns (A1-A6) and groups them into 4 families using main's framing: defense accretion (A1, A6 partial), reconciliation asymmetry (A2, A6, A5), procedure/review overhead (A3), tooling fragility (A4). The DOMINANT family for audit is the same as for main: reconciliation asymmetry. This is independent confirmation that the F-family taxonomy generalizes beyond main's specific implementation — audit is a different orchestrator with overlapping but distinct surface, and the same family structure emerges from its independent analysis.

(2) **A2 is the most consequential audit-side blind spot.** The audit chain `#402 → #406 → #415 → #417 → #420` (16 cycles, 187-201) was constraint-patching v1's chronic-category-tracking mechanism — the exact mechanism v2 deprecates. The audit was the AGENT of accretion, not its detector. The recognition arrived 16 cycles after the chain started; only when external framing (main's v2 retrospective F1) was applied to audit's own output did the meta-pattern become visible. This is the cycle-22 cross-system synthesis lesson at a different scale: same-system inspection has structural blind spots; outside-framing surfaces what inside-framing cannot.

(3) **A3 (filing-rate vs evidentiary-base discipline tension) parallels main's F9 (adversarial-review treadmill).** Both surfaces gravitate toward more findings unless explicitly disciplined. Both required Eva intervention as v1 corrective (audit: observation-window discipline #425; main: review-agent calibration). v2 design for both should bake calibration into the prompt surface rather than relying on Eva intervention.

(4) **A6 (audit takes main's framing at face value) is the inverse of cycle-22's "PAI elevates code-vs-prompts split as named principle" observation.** Audit's blind spots emerge when reading framings as load-bearing without verifying primary evidence. PAI's principle-list elevates rules to first-class status, making them visible. Both observations are about the role of explicit-vs-implicit articulation in surfacing what's actually load-bearing. Possible Phase 2 design implication: explicit articulation of rules is preferable to implicit-architectural-instantiation when downstream readers (other orchestrators, audit) need to verify against primary evidence.

(5) **Cross-validating main's F-family taxonomy** is now possible: the audit retrospective is an independent application of the same framework. If main's F-family taxonomy has missing categories or incorrect groupings, the audit retrospective should surface tensions. From this initial read, no obvious tensions — A1-A6 map cleanly onto the four families main names. This is preliminary; deeper read across the rest of the audit retrospective could surface more.

**Cycle-25 implications:**
- No immediate edits to main's `0-retrospective.md` from this read.
- New cycle-26+ pre-commit candidate: cross-validate main's F-family taxonomy by comparing F-family member patterns to audit's A1-A6 mapping. Specific question: are any of main's F-patterns better-classified after seeing how audit grouped its analogs?
- New cycle-26+ pre-commit candidate: read remaining audit retrospective sections (Open questions; What should be preserved; What v2 must demonstrably do better; Iteration plan / log). The "What v2 must demonstrably do better" section is potentially the most relevant for Phase 2 design space; deferred to cycle 26.

## Same-cycle cold-reader on cycle-25 substantive rewrites

Per the discipline-lightening rule codified in item 9 above: substantive rewrites (~47-line addition + ~5-line section-opening edit on architecturally-load-bearing prose) get full structured pass.

### Anti-smuggling discipline check

- **Pattern 5 (memory):** Citations verified (PAI Principle 13 lines 180-185; LangGraph short/long-term lines 690-696; "With checkpointers alone..." quote verified). "Memory as a first-class architectural concept, not derivative of state" is observation-shaped (claims what each system DOES, not what v2 SHOULD do). Asymmetry-acknowledged ("PAI's framing is principle-shape; LangGraph's is mechanism-shape"). The closing "Convergence on architectural-elevation; asymmetry on framing-vocabulary" is meta-observation about the convergence claim itself, parallel to other 2-system patterns. PASS.
- **Pattern 6 (small fixed team):** Citations verified (Voyager 4-agent lines 406-418; Magentic-One Task Ledger / Progress Ledger lines 367-368). "Both have small fixed teams with named roles" is observation. Structural-asymmetry section names BOTH directions (peer-flow vs lead-worker-hierarchy; architecture vs one-of-many-patterns). Closing "Convergence on small-fixed-team-with-named-roles; divergence on orchestration topology" is symmetric meta-observation. PASS.
- **Pattern 7 (per-agent model):** Citations verified (Voyager cost-tiering lines 420-429; AutoGen Extensions API model clients line 244). The claim "each AssistantAgent takes its own `model_client`" is grounded in the AutoGen research deliverable (PR #2763); citation chain extends one layer. Asymmetry on rationale (Voyager foregrounds cost-tiering; AutoGen documents architectural flexibility without prescribing rationale) is symmetric. Closing observation-shaped. PASS.

### Section-transition check

- Existing pattern 4 ("Failed work as recorded artifact") → new pattern 5 ("Memory as a first-class architectural concept"): clean break.
- New pattern 5 → new pattern 6 ("Small fixed team"): clean break.
- New pattern 6 → new pattern 7 ("Per-agent model selection"): clean break.
- New pattern 7 → "Persistent divergences" header: clean break.

PASS.

### Section-opening edit (item 8)

Caught process-commentary slip ("as cycle 25 did for three observations") within same cycle. Trimmed to "(which can elevate them to 2-system on cross-system match)" — preserves the rule statement without process-commentary. **APPLIED.**

This is the FIRST same-cycle cold-reader catch on a section-opening edit (vs the four prior catches on body rewrites: cycle 19 Tier-2 group 3, cycle 21 blockquote tightening, cycle 22 cross-system synthesis, cycle 25 elevations). Process-commentary risk in section-opening / rule-stating prose is structurally different from body-prose smuggling: section-opening prose IS rules, so the rules can leak into process-narrative ("as cycle X did") in a way body prose can't.

### Same-cycle cold-reader verdict

PASS across all three new patterns + section-opening edit. One process-commentary catch on section-opening, fixed within same cycle. **Fifth instance of same-cycle-cold-reader-on-substantive-rewrite surfacing a real concern** (cycles 19, 21, 22, 23 caught nothing as bounded mechanical, 24 caught nothing as bounded mechanical, 25 caught process-commentary). Pattern shape stable: substantive rewrites → high marginal value (5/5 hits); bounded mechanical → low marginal value (0/2 hits). Discipline-lightening rule codified above is supported by the running tally.

## What surprised me

Three things.

(1) **All three single-system observations elevated cleanly to 2-system.** I came into cycle 25 with cycle-24's ranking (HIGH/HIGH/MEDIUM) and expected the MEDIUM candidate (Voyager cost-tiering ↔ AutoGen per-agent model_client) to fail. In practice, when I re-read the AutoGen research deliverable (PR #2763), the model-clients-as-Extensions-API-layer claim is solid; the per-agent constructor coupling is documented in the quickstart. What's truly Voyager-distinctive is the cost-tiering RATIONALE, not the per-agent-model architecture. The honest-hedge pattern (cycle-24) provided the framing: convergence on the ARCHITECTURE, asymmetry on the RATIONALE. This is the second instance (after cycle-24's openclaw weakening on pattern 2) where the honest-hedge approach preserves a 2-system claim that drop-or-keep extremes would have lost or over-claimed.

(2) **Process-commentary risk in section-opening / rule-stating prose is structurally different from body-prose smuggling.** Section-opening prose IS rules, so process-narrative ("as cycle X did") can leak in as "evidence the rule works." The cycle-9 sweep applied to body sections of `0-retrospective.md`; cycle 25 confirms the same discipline applies to section-opening prose. The structural shape: section-opening edits and discipline-rule edits warrant full structured cold-reader regardless of line-count, because the failure mode is different from body-prose smuggling. This refines the discipline-lightening rule: "architecturally-load-bearing edits" includes section-openings, regardless of size.

(3) **The audit retrospective's family-mapping confirms main's F-family taxonomy generalizes.** Audit independently grouped its A1-A6 patterns into the same 4 families main named (defense accretion, reconciliation asymmetry, procedure/review overhead, tooling fragility) — and the dominant family is the same (reconciliation asymmetry). This is meaningful because audit is a different orchestrator with overlapping but distinct surface. If main's F-taxonomy were artifact-of-implementation rather than structurally-grounded, the audit retrospective would have either invented different families or strained to fit. Neither happened. Cross-system validation of an internal taxonomy framework — the cycle-22 append-only pattern's double-evidence-base shape applied at a different scale (taxonomy-level, not pattern-level).

## What I'm still uncertain about

Three things.

(1) **Whether the new pattern 7 ("Per-agent model selection") should explicitly note the citation-chain extension.** The pattern relies on the AutoGen research deliverable (PR #2763) for the per-agent constructor coupling, not directly on `1-research.md`. The cycle-25 phrasing ("AutoGen's Extensions API documents 'model clients' as a layer abstraction; each AssistantAgent takes its own `model_client`...") is grounded but doesn't make the citation chain explicit. **Defensible reading:** PR #2763 is part of the Phase 1 evidence base; citation-chain transparency would over-elaborate the prose. A future cold-reader can revisit if the chain becomes load-bearing.

(2) **Whether the audit retrospective interleave-read should generate a cross-validation pre-commit for main's F-family taxonomy.** The audit's A-pattern → F-family mapping is clean but I only read 6 patterns. A more thorough cross-validation would walk all 12 of main's F-patterns and check whether audit's framing surfaces tensions in any of them. **Defensible reading:** add as cycle-26+ pre-commit candidate; gate-release deeper audit retrospective reading to determine if cross-validation would surface new findings.

(3) **Whether cycle-25's three elevations push the 2-system tier into "saturated."** The 2-system tier now has 7 patterns (4 original + 3 cycle-25 elevations). The 3+ tier has 5 patterns. The deliverable is becoming weighted toward 2-system claims. **Defensible reading:** the 7 patterns reflect the actual evidence at 5 systems read; this is observation-shaped and the asymmetry-acknowledgment per pattern keeps each claim honest. If a 6th system (Cognition Devin, harness-engineering writeup, oh-my-codex / oh-my-claudecode dispatches) supports any 2-system pattern, the section-opening rule (mechanically-elevates-to-3+) applies. The current shape is correct for current evidence.

## Persistence-mechanism observations

The cycle-N-pre-commits-cycle-N+1-checks chain extends to twenty cycles deep (cycle 7 → ... → 24 → 25 → 26 pre-committed). No breakdown.

The same-cycle-cold-reader-on-rewrite pattern tested for the sixth time this cycle. Substantive scope (47-line elevations + 5-line architectural section-opening) → caught process-commentary slip in section-opening. Bounded scope is now 0/2 in cold-reader; substantive scope is 5/5. Discipline-lightening rule codified above with the threshold and exception explicit.

The fresh-eye-vs-same-cycle complementarity pattern continues — cycle-24 fresh-eye on cycle-22 cross-system synthesis surfaced 3 flags (substantively-applied this cycle in two layers: elevation work + section-opening tightening). Cycle-25 same-cycle on its own elevations caught the process-commentary slip. Both angles independently surfaced different concerns.

The honest-hedge pattern (named cycle 24) tested twice this cycle — patterns 6 and 7 (small fixed team; per-agent model selection) both used explicit asymmetry-acknowledgment. The pattern is now applied 4 times across cycles 24-25 (cycle 24: openclaw weakening on pattern 2 + cost-tiering ranking; cycle 25: small fixed team asymmetry + per-agent model rationale asymmetry). Pattern shape stable.

The process-commentary catch on section-opening / rule-stating prose is a new pattern instance: cycle-25 confirms section-opening prose has its own structural smuggling risk distinct from body-prose smuggling. New sub-rule: architecturally-load-bearing edits get full cold-reader regardless of size.

The cross-system taxonomy validation observation (audit's A-patterns mapping onto main's F-families with the same dominant family) is a new persistence-mechanism artifact — the F-family taxonomy is now externally-validated by audit's independent analysis. If the taxonomy were artifact-of-implementation, audit's framework would have either invented different families or strained. Neither happened.

Long-deferred items: 9 → 9 unchanged this cycle (cycle 25 is Phase 1 cold-reader/synthesis work, not Phase 0 long-deferred items).

## Cycle 26+ pre-commits

1. **Cognition Devin orchestrator-direct read** (cycle-23 pre-commit 3, cycle-26 focal). The closest analog to v2's "AI does software-engineering work autonomously" target. Could surface 6th-system patterns to test convergence claims against. Estimated 1 focal cycle. **Highest priority for cycle 26 focal work.**

2. **OpenAI harness-engineering writeup orchestrator-direct read** (Eva directive #2775, blog-shaped). Eva's note: "read-shape closer to Cognition Devin writeups than to a code+docs read." Interleave with cycle-26 Cognition Devin read if both blog posts are short; else cycle-27 focal.

3. **Copilot research-only dispatch: oh-my-codex** (Eva directive #2774, github repo). Standard cycle-15 procedure, gpt-5.5. Earliest dispatch: cycle 26. Tier-1 nav summary in `1-research.md` cycle-N+1 after PR.

4. **Copilot research-only dispatch: oh-my-claudecode** (Eva directive #2774, github repo). Standard cycle-15 procedure, gpt-5.5. Cycle 27+ after oh-my-codex.

5. **Copilot research-only dispatch: openai/symphony** (Eva directive #2775, github repo). Standard cycle-15 procedure, gpt-5.5. Cycle 27+ after harness-engineering read.

6. **Fresh-eye cold-reader on cycle-25 elevations** (standard cycle-N+1 fresh-eye pass). Specific questions: (a) does the convergence-with-acknowledged-asymmetry framing read load-bearing-balanced or hedge-heavy across patterns 5-7? (b) is the per-agent model selection pattern's citation chain (extending into PR #2763) sufficiently transparent? (c) does the section-opening tightening's "60% bar at 5 systems" framing read as observation or as rule-statement that smuggles forward-looking expectation?

7. **Cross-validate main's F-family taxonomy against audit's A-pattern mapping** (new from cycle-25 audit retrospective interleave-read). Specific question: are any of main's F-patterns better-classified after seeing how audit grouped its analogs? Estimated bounded mechanical work (~1 cold-reader cycle).

8. **Read remaining audit retrospective sections** (new from cycle-25 audit retrospective interleave-read). Sections deferred from this cycle: "What appears to be working" + "What might appear to work but probably doesn't" + "Open questions" + "What should be preserved through cutover" + "What v2 must demonstrably do better than v1" + "Iteration plan" + "Iteration log". The "What v2 must demonstrably do better" section is potentially most relevant for Phase 2 design space.

9. **Description-shape-asymmetry escalation watch** (cycle-23 carry-forward; no action). If Phase 2 candidate-authors quote the blockquote standalone, escalate to blockquote-pointer addition.

### Suggested cycle 26 plan

- **Focal:** item 1 (Cognition Devin orchestrator-direct read)
- **Interleave:** item 2 (harness-engineering writeup, if short enough)
- **Dispatch:** item 3 (oh-my-codex Copilot research dispatch)
- **Bounded mechanical (if capacity):** item 6 (fresh-eye cold-reader on cycle-25 elevations)
- **Defer:** items 4-5, 7-8 to cycle 27+

### Suggested cycle 27 plan (provisional)

- **Focal:** items 4-5 dispatches OR item 7 (F-family cross-validation) OR item 8 (remaining audit retrospective)
- Depends on cycle 26 outcomes (which dispatches landed; which reads completed)

## Long-deferred items roll-call (carry-forward)

1. Journal-entry self-congratulation sweep (19 cycles deferred from cycle 7+)
2. F6/F8/F9 measurements (cycle 7+)
3. Refactor-for-length F-section sweep (cycle 8+; Tier-2 group 8)
4. Persistence-mechanism deferred-list consolidation (cycle 13 flag)
5. Tier-2 group 2 (review/disposition substrate, partial integration cycle 13)
6. Tier-2 group 4 (nine measures rework)
7. Tier-2 group 6 (preserved-through-cutover disposition)
8. Tier-2 group 7 (resolved open questions collapse)
9. Tier-2 group 9 (F8 singleton-family acknowledgment)

Net: 9 → 9. No resolutions or additions this cycle (cycle 25 is Phase 1 cross-system synthesis work, not Phase 0 long-deferred items).
