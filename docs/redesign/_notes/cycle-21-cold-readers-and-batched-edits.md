# Cycle 21: Cold-readers + batched bounded-mechanical edits (items 2, 4, 6 + cold-reader item 3)

Cycle 20 (commit `c872d65b`) evaluated PR [#2768](https://github.com/EvaLok/schema-org-json-ld/pull/2768) (LangGraph), integrated a ~214-line Tier-1 nav summary into `1-research.md`, and ran the fresh-eye cold-reader on Tier-2 group 3 (verdict PASS, no edits required). Cycle 20 left five active pre-commits with the recommendation "batch items 2-4 + item 6 if all bounded mechanical; item 5 may need its own focal cycle."

Cycle 21 takes the recommended batch:
- **Item 6** (focal): fresh-eye cold-reader on cycle-20 LangGraph Tier-1 integration (~214 new lines)
- **Item 3**: cold-reader on cycle-19 c1.1 fix (Voyager bullets 15-16 trim) — signal-loss check
- **Item 4**: compositionality elevation (cycle-18 cold-reader 1 flag c1.2)
- **Item 2**: v2 design implications blockquote tightening (item 11; option-3-leaning imperative voice)

**Defer to cycle 22**: Item 5 (cross-system transferability observations integration). Cycle-20 estimated +30-50 lines plus rework of preliminary-observations section; deserves its own focal cycle.

## Item 6 — Fresh-eye cold-reader on LangGraph Tier-1 integration

Question: does the discipline (anti-smuggling, cycle-18 anchoring-caveats-symmetry, prose-bullet alignment) hold across all 214 lines, or does it degrade in section transitions like cycle-15 saw with AutoGen?

Re-reading the LangGraph nav summary in `1-research.md` lines 598-811 as fresh-eye reviewer, evaluating per the cycle-15/16/17 cold-reader protocol.

### Anti-smuggling discipline check (11 prose paragraphs)

**Paragraph 1: "Project status as observable evidence."** Cites `overview.mdx` directly. The "different posture from AutoGen's maintenance-mode-with-successor signaling" framing is cross-system OBSERVATION, not v2 prescription. PASS.

**Paragraph 2: "Pregel / bulk-synchronous-parallel super-step execution."** Cites docs and source. The "substantively different orchestration shape from AutoGen's actor model" comparison is observable; the closing "Reducers are core (not optional) precisely because parallel writes within one super-step need a deterministic merge rule" is mechanism explanation, observable from code. PASS.

**Paragraph 3: "State as typed channels with per-key reducers."** All claims source-cited. Closing observation "State is a channel map, not a single blob with one update policy" is descriptive. PASS.

**Paragraph 4: "Checkpointing at super-step boundaries with thread-scoped histories."** All claims cite source. The implementation-vs-docs gap observation (`channel_versions` etc richer than docs summary) is observation honesty. PASS.

**Paragraph 5: "Pending writes for failed super-steps."** "Stronger than checkpoint-at-end recovery" is a comparison to a generic baseline, not v1-specific. PASS.

**Paragraph 6: "Time travel as append-only fork, not destructive rollback."** All cited. "This is the architectural distinction" is a mild meta-framing introducing a quoted doc. PASS.

**Paragraph 7: "Short-term (thread-scoped) vs long-term (cross-thread) memory."** All cited. PASS.

**Paragraph 8: "Interrupts as checkpoint/resume with restart-from-beginning warning."** All cited; the docs warning is quoted directly. PASS.

**Paragraph 9: "Multiple orchestration patterns coexist as first-class."** "Matches openclaw's anti-pattern stance and AutoGen's v0.4 removal" is cross-system observation, valid. PASS.

**Paragraph 10: "Subgraph composition (graphs as nodes)."** Closing "This makes subgraphs not just code reuse, but inspectable nested execution" is mild interpretation that summarizes the technical claim. PASS.

**Paragraph 11: "Honest implementation-vs-marketing-claims discipline."** Closing sentence "This is research-evaluation honesty, not v2-relevance smuggling" is process-defense — it argues why the section's value-judging content is not smuggling. **MINOR FLAG**: the closing sentence reads as defensive process-commentary about the deliverable rather than observation about LangGraph. Could be removed without losing observation-level signal. The preceding sentences ("the deliverable's section 2.8 separates well-supported claims from claims-that-need-qualification" + cited content) carry the load-bearing observation. Proposed fix: delete the closing sentence; the paragraph would then read as observation about the deliverable's discipline without the defensive frame. Bounded mechanical fix, ~1 line removal.

### Cycle-18 anchoring-caveats-symmetry discipline check (6 caveats)

Walk each caveat for symmetric Discounts/Transfers structure:

1. **Library vs autonomous orchestrator.** Discounts: API ergonomics, Agent Server, generic visualization. Transfers: explicit state schemas, checkpoint/store separation, append-only fork, super-step boundaries. SYMMETRIC.
2. **Human user vs autonomous cron.** Discounts: thread_id as conversation cursor, indefinite-wait interrupts. Transfers: stable execution identity, interrupt semantics for approval gates, checkpointed pause/resume, stream events. SYMMETRIC.
3. **Python/TypeScript vs Rust.** Discounts: TypedDict/Annotated/decorators. Transfers: typed state, channel reducers, checkpoint IDs, parent links, pending writes, namespaces; Rust enum/struct; deterministic-execution-vs-LLM-proposal split; idempotence. SYMMETRIC.
4. **Short-to-medium vs hundreds of cycles.** Discounts: per-thread history bloat, time-travel cost, DB checkpointer durability vs git. Transfers: durable-execution warning *more* important; append-only histories; short/long-term memory split; state versioning. SYMMETRIC. (This is the non-obvious caveat where Transfers content is genuinely substantive.)
5. **Database checkpointers vs git/GitHub.** Discounts: transaction semantics, primary-key lookup, Agent Server hiding persistence. Transfers: checkpoint records as files/commits; parent links as commit ancestry; pending writes as partially-completed parallel tasks; Store vs checkpoint maps to repo-wide vs cycle-local state. SYMMETRIC.
6. **Post-LangChain pivot.** Discounts: LangChain-shaped tool schemas, LangSmith product boundary. Transfers: layering high-level agents on lower-level deterministic runtime; keeping model/tool integration separate; exposing low-level state operations. SYMMETRIC.

All 6 caveats honor the cycle-18 anchoring-caveats-symmetric-discipline. The Transfers content is substantive (not formulaic) — caveat 4's "durable-execution warning becomes *more* important" is non-obvious; caveat 5's "pending writes as partially-completed parallel tasks" maps to GitHub-resident state concretely. PASS.

### Prose-to-bullet alignment check (20 bullets vs 11 paragraphs)

Spot-check each bullet against prose:

| Bullet | Prose anchor | Notes |
|---|---|---|
| Pregel/BSP super-step execution model | Para 2 | Direct match |
| Plan/execution/update phases with parallel-write isolation | Para 2 | Direct match |
| State as named channels; per-channel reducers with overwrite default | Para 3 | Direct match |
| Multiple schemas (Input/Output/Overall/Private) | Para 3 | Direct match |
| Checkpointing at super-step boundaries (graph-step rewind) | Para 4 | Direct match |
| `thread_id` as required persistence cursor; `checkpoint_ns` namespace | Para 4 | Direct match |
| `StateSnapshot` containing values, next nodes, config, metadata, parent_config, tasks | Para 4 | Direct match |
| Implementation-level checkpoint metadata richer than docs summary | Para 4 | Direct match |
| Pending writes for successful siblings in failed super-steps | Para 5 | Direct match |
| Durability modes (`exit`/`async`/`sync`) exposing tradeoff explicitly | **No prose anchor** | **FLAG** |
| Time travel as append-only fork (not destructive rollback) | Para 6 | Direct match |
| Replay re-executes nodes; not cache replay | Para 6 | Direct match |
| Short-term checkpoints vs long-term Store with namespace+key+value | Para 7 | Direct match |
| Interrupts as checkpoint/resume; node restarts from beginning | Para 8 | Direct match |
| Subgraph composition (two patterns: wrapper / direct) | Para 10 | Direct match |
| Multiple orchestration patterns documented as first-class | Para 9 | Direct match |
| Explicit position against reflexive multi-agent decomposition | Para 9 | Direct match |
| Explicit non-goal: architectural opinionation | Para 1 | Direct match |
| Explicit anti-patterns enumerated (kitchen-sink avoidance, ...) | **Partial** | **FLAG** |
| Honest implementation-vs-marketing-claims subsection | Para 11 | Direct match |

**Two bullet-without-prose-anchor flags identified:**

(a) Bullet 10 "Durability modes (`exit`/`async`/`sync`) exposing tradeoff explicitly" — the PR's section 2.5 covers this, but the nav summary's prose paragraph 5 (pending writes) does not name durability modes. Two options: trim bullet OR add 1 sentence to prose paragraph 5. **Defensible reading**: the durability-mode tradeoff is real LangGraph-distinctive content; preserving it as a bullet without prose anchor matches the cycle-18 cold-reader 1 c1.1 pattern (the OPPOSITE direction would be removing prose for a claim with no bullet). Recommend Option B (add sentence to prose para 5) as a cycle-22+ pre-commit.

(b) Bullet 19 "Explicit anti-patterns enumerated (kitchen-sink avoidance, replay-as-cache mistake, interrupts-as-line-continuations mistake, etc.)" — partial prose anchor. Some anti-patterns appear scattered: replay-as-cache in para 6, interrupts-as-line-continuations in para 8. But "kitchen-sink avoidance" is not in any prose paragraph; it's in the PR's section 6. Two options: trim "kitchen-sink avoidance" framing from the bullet OR add a paragraph naming the anti-patterns subsection. **Defensible reading**: the AutoGen nav summary likely also has anti-patterns dispersed across paragraphs without a dedicated paragraph; adding a paragraph here would be over-elaborate. Recommend Option A (trim "kitchen-sink avoidance" framing) as a cycle-22+ pre-commit.

### Cycle-15 smuggling-in-section-transitions pattern

The cycle-15 finding was that smuggling can hide in section transitions (between per-system findings and "Provisional patterns to track" sections). Re-checking the LangGraph integration's transitions:

- Transition from `### Voyager` to `### LangGraph` (line 597 → 598): no prose between sections; clean section break.
- Transition from `### LangGraph` to `## Cross-system observations (preliminary)` (line 810 → 812): no v2-relevance summary in the transition.
- The Patterns observed list closes the LangGraph section at line 810 without a "Provisional patterns to track" wrap-up paragraph. The cross-system observations are kept in the dedicated `## Cross-system observations (preliminary)` section starting at line 812.

PASS — no smuggling in section transitions.

### Item 6 verdict

**PASS** with three flags for cycle-22+ consideration:
- (a) Bullet 10 (durability modes) lacks prose anchor: add 1 sentence to prose para 5
- (b) Bullet 19 ("kitchen-sink avoidance") lacks prose anchor: trim the framing from the bullet
- (c) Paragraph 11 closing sentence is process-defense: delete 1 line

All three are bounded-mechanical (5-10 line edits each, ~5 minutes of judgment). Per cycle-18 apply-when-bounded-mechanical-AND-capacity rule: applicable for batched cycle-22 work, defer to that cycle.

## Item 3 — Cold-reader on cycle-19 c1.1 fix (Voyager bullets 15-16 trim)

Question: did trimming "lossy summaries" / "question→answer pairs" framings drop signal that should be preserved somewhere?

**Bullet 15** (current): "LLM-generated skill descriptions as the embedding surface (embeddings are over descriptions, not over raw code)"

The dropped framing was "lossy summaries" — meaning LLM-generated descriptions are inherently less semantically rich than the code they describe. This is interpretive value-framing, not pure observation. The trimmed bullet preserves the OBSERVATION (embeddings are over descriptions, not over raw code) while removing the interpretive overlay.

The signal that's been dropped: the implicit tradeoff (faster retrieval over descriptions, less semantic resolution than embedding over code). This tradeoff isn't surfaced in prose either.

Cold-reader assessment: the trim is FINE. The "lossy summaries" framing was useful interpretive content but it was also slightly pejorative ("lossy" implies deficiency). Pure-observation phrasing is the right discipline for per-system bullet lists. The interpretive layer ("this tradeoff matters because X") belongs in cross-system synthesis (item 5 cycle-22 work) where the embedding-over-summary-not-source pattern is observable across systems (Voyager descriptions, openclaw commit summaries, AutoGen component configs).

**Bullet 16** (current): "QA-cache pattern for repeated curriculum lookups (`qa_cache.json` plus vectordb of cached questions, kept in sync)"

The dropped framing was "question→answer pairs" — describing the cache content structure (Q→A pairs). The new parenthetical describes the storage structure (qa_cache.json + vectordb).

Cold-reader assessment: the trim is FINE. The Q→A semantics is observable from the `qa_cache.json` filename. The trimmed bullet emphasizes the dual-storage-with-sync-invariant pattern, which is the broader cross-system pattern (sync invariants for dual storage). The Q→A semantics is implicit in the filename naming and need not be re-stated in the bullet.

### Item 3 verdict

**PASS** — both trimmed bullets preserve core observations. The dropped framings were interpretive overlays that didn't add observation-level signal at the per-system bullet level. The interpretive content ("lossy summaries" tradeoff; "Q→A pairs" semantics) belongs in cross-system synthesis or per-system prose if anywhere — not in bullet lists.

One observation worth recording for cycle 22+ cross-system synthesis: the **embedding-over-summary-not-source** pattern (Voyager: descriptions; openclaw: commit summaries) is a candidate cross-system observation for the Cross-system observations section. The c1.1 trim doesn't enable or block this; the pattern observation belongs in the cross-system section, not in per-system bullet lists.

## Item 4 — Compositionality elevation (cycle-18 cold-reader 1 flag c1.2) — APPLIED

Cycle-18 cold-reader 1 found that the prose paragraph "Two-layer capability composition" (lines 495-503 of `1-research.md`) makes a TWO-PART claim:
1. Two-layer capability composition (architecture)
2. **Compositionality is the paper's named learning mechanism** (interpretation)

The bullet list captured claim 1 (bullet 12: "Two-layer capability composition: hand-written primitives + LLM-composed skills over primitives") but not claim 2.

**Bullet 11** ("Explicit no-fine-tuning architectural commitment, with skill-library as the named learning mechanism") names skill-library at the high level (vs gradient updates). The prose's compositionality claim is finer-grained: WITHIN the skill library, the structure that produces learning is compositionality (recursive composition: skills compose primitives, then skills compose earlier skills).

These are distinct claims at different granularities:
- Bullet 11: learning is through skill-library (vs fine-tuning)
- Prose: WITHIN skill-library, compositionality is the learning mechanism

Edit applied: New bullet inserted between current bullet 12 (two-layer composition) and current bullet 13 (prompts as external files):

```
- Compositionality (skills compose primitives; later skills compose earlier
  skills) as the paper's named learning mechanism within the skill-library
  architecture
```

Position rationale: adjacent to bullet 12 (two-layer composition) for logical grouping — bullet 12 describes the layered structure; new bullet describes how learning happens through it.

Bullet count: 16 → 17.

### Same-cycle cold-reader on the c1.2 fix

Question: does the new bullet correctly capture the prose claim without redundancy with bullet 11?

Re-reading:
- Bullet 11: "Explicit no-fine-tuning architectural commitment, with skill-library as the named learning mechanism" — names learning mechanism at architectural level (no gradient updates)
- New bullet (after bullet 12): "Compositionality (skills compose primitives; later skills compose earlier skills) as the paper's named learning mechanism within the skill-library architecture" — names learning mechanism at compositional-structure level (within the skill library)

The two bullets are at different granularities: bullet 11 is "we use skill-library not fine-tuning"; new bullet is "the structure within skill-library that produces learning is compositionality." The "within the skill-library architecture" qualifier in the new bullet explicitly distinguishes it from bullet 11's high-level claim.

PASS — no redundancy; the granularity distinction is clear.

## Item 2 — v2 design implications blockquote tightening — APPLIED

Cycle 19 found a tension at `0-retrospective.md` lines 957-961: the "v2 design implications by family" section's "Defense accretion implication" blockquote uses imperative voice ("Cycle boundaries should be checkpoint markers...") which is option-3-leaning (continuous reconciliation). Mild tension with the cycle-19 family-preamble bilateral framing.

Original blockquote (5 lines):
```
> **Defense accretion implication.** Cycle boundaries should be
> checkpoint markers on a continuously-evolving state, not state
> hard-boundaries that freeze worklogs and leave defense refreshers
> stranded post-close. Defenses themselves should be re-examined for
> load-bearingness; the catalog proves accumulation, not that each
> defense is currently load-bearing.
```

New blockquote (10 lines):
```
> **Defense accretion implication.** v2 must dissolve the freeze-vs-refresh
> timing mismatch named in the family preamble. What v2 must AVOID is v1's
> specific pattern: state hard-boundaries that freeze worklogs and leave
> defense refreshers stranded post-close. The shape of the fix is a Phase 2
> design choice across the family preamble's three resolutions — checkpoint
> markers on a continuously-evolving state (option 3, continuous
> reconciliation); pre-freeze timing alignment so refreshers complete
> (options 1 and 2, earlier defenses or later freeze). Defenses themselves
> should be re-examined for load-bearingness; the catalog proves
> accumulation, not that each defense is currently load-bearing.
```

Restructure rationale:
1. Lead with general prescription ("v2 must dissolve the freeze-vs-refresh timing mismatch")
2. Concrete failure mode v1 exhibits ("state hard-boundaries that freeze worklogs and leave defense refreshers stranded post-close" — preserves original concrete observation)
3. Phase-2 design space with all three resolutions named ("The shape of the fix is a Phase 2 design choice")
4. Option-agnostic claim preserved unchanged ("Defenses themselves should be re-examined for load-bearingness")

### Same-cycle cold-reader on the blockquote tightening

Question: does the new blockquote read as Phase-2-design-space-aware, or did the tightening lose load-bearing content?

Re-reading produces these observations:

(a) The load-bearing observation IS preserved: v1's pattern of "state hard-boundaries that freeze worklogs and leave defense refreshers stranded post-close" is explicitly named, with the original wording carried forward. This is the observation, not the prescription.

(b) The bilateral framing is honored: option 3 (continuous reconciliation) and options 1/2 (pre-freeze timing alignment) both have explicit prescriptions. The Phase 2 framing ("The shape of the fix is a Phase 2 design choice") signals the choice is open.

(c) The option-agnostic closing claim ("Defenses themselves should be re-examined for load-bearingness; the catalog proves accumulation, not that each defense is currently load-bearing") is unchanged.

(d) **Minor flag (preferential ordering risk)**: option 3's prescription gets more words ("checkpoint markers on a continuously-evolving state" + "(option 3, continuous reconciliation)" — ~14 words) than options 1/2's prescription ("pre-freeze timing alignment so refreshers complete" + "(options 1 and 2, earlier defenses or later freeze)" — ~14 words). Actually word-count is approximately balanced; the perceived imbalance is from option 3 being named first. **Defensible reading**: option 3 is named first because the original blockquote was option-3-leaning, so the preserved-content (the original prescription) appears first as the option-3 expression. Options 1/2 are added as the bilateral alternative. The ordering is historical, not preferential.

(e) Line-count expansion (5 → 10 lines) is larger than cycle-19's "~2-3 line qualification" estimate. Most of the expansion is the bilateral framing; ~2 lines of structural rephrase to lead with general prescription.

### Item 2 verdict (same-cycle cold-reader)

**PASS** with one minor flag:
- The option-3-first ordering creates mild preferential-reading risk. **Defensible reading**: ordering is historical (preserves original blockquote's option-3-leaning prescription as the option-3 expression in the new structure). Not edit-worthy this cycle.

## What surprised me this cycle

Three things.

(1) **The cycle-20 LangGraph Tier-1 integration held up cleanly under fresh-eye cold-reader.** I expected to find more bullet-without-prose-anchor mismatches (cycle 18 cold-reader 1 found 2 such mismatches in Voyager bullets 15-16). I found 2 in LangGraph (bullets 10 and 19), but 18 of 20 bullets map cleanly to prose. The cycle-18 anchoring-caveats-symmetric discipline is HONORED across all 6 caveats. The cycle-15 smuggling-in-section-transitions pattern is NOT present. The discipline propagation chain (cycle-18 cold-reader 2 → cycle-18 dispatch instruction → cycle-18 deliverable structure → cycle-20 Tier-1 integration → cycle-21 cold-reader verification) closed cleanly.

(2) **The same-cycle cold-reader on the blockquote tightening surfaced a non-obvious risk** (preferential ordering of option 3 first). Cycle 19 named the same-cycle-cold-reader-on-rewrite pattern. This is its second test. It surfaced a real concern (preferential ordering) that I had not anticipated when drafting the rewrite. The risk is mild but real; defensible reading is "ordering is historical." Without the same-cycle cold-reader, the risk would have been buried in the deliverable.

(3) **Item 3 (c1.1 cold-reader) produced a cross-system observation as a side effect.** The c1.1 trim was about removing interpretive overlays from per-system bullets. Re-reading the trimmed bullets surfaced that the **embedding-over-summary-not-source** pattern is observable across systems (Voyager descriptions, openclaw commit summaries, AutoGen component configs). This is exactly the cross-system synthesis content for item 5. The c1.1 cold-reader was supposed to be backward-looking (was the trim correct?) but produced a forward-looking observation (cross-system pattern candidate). This validates the discipline of running cold-readers on prior fixes — they sometimes surface new content.

## What I'm still uncertain about

(1) Whether the three flags from item 6 (durability modes bullet without prose anchor; kitchen-sink avoidance bullet phrasing; paragraph 11 process-defense closing sentence) should be batched in cycle 22 or in the cycle-22 focal cycle for item 5. The flags are bounded mechanical (~10 lines total across all three), but cycle 22 is anticipated to be a focal cycle for item 5 (cross-system transferability observations integration, +30-50 lines). Mixing the two might over-load cycle 22.

   Defensible reading: split — cycle 22 takes item 5; cycle 23 takes the three flags from item 6 as a batched bounded-mechanical session. This preserves the focal-cycle-for-item-5 plan.

(2) Whether the blockquote tightening's preferential-ordering flag is a real concern or a phantom concern. The flag is about reading order, not content. A reader genuinely uncertain about which option to favor would read the family preamble (which lists options 1, 2, 3 with explicit equal status). The blockquote restates the design space, but Phase 2 candidates would draw from the family preamble, not the blockquote. So the preferential-ordering risk in the blockquote is bounded.

   Defensible reading: leave the ordering as-is; the family preamble is the load-bearing source for the bilateral framing.

(3) Whether the c1.1 cold-reader's cross-system observation (embedding-over-summary-not-source) is a load-bearing cross-system pattern or a coincidence. Voyager's descriptions and openclaw's commit summaries are both LLM-generated text-over-source. AutoGen's component configs are configuration metadata, not LLM-generated summaries — so the third instance might not match. **Defensible reading**: the pattern is "embedding-over-derived-content" (descriptions, summaries, configs) vs "embedding-over-source-content" (raw code). All three systems use derived content. The pattern is real but the specific framing ("LLM-generated summaries") might be too narrow. This belongs in item 5 cross-system synthesis where the framing can be tested across all 5 systems.

## Persistence-mechanism observations

The cycle-N-pre-commits-cycle-N+1-checks chain extends to sixteen cycles deep (cycle 7 → ... → 20 → 21 → 22 pre-committed). No breakdown.

The cycle-15/16 dispatch-evaluation-then-Tier-1-integration pattern + cycle-N+1 fresh-eye cold-reader pattern applied this cycle (LangGraph Tier-1 integration in cycle 20 + fresh-eye cold-reader in cycle 21 = 1-cycle gap, cleanest variant).

The same-cycle-cold-reader-on-rewrite pattern (named cycle 19) applied this cycle for the second time. It surfaced a real concern (preferential-ordering risk in the blockquote tightening) that I had not anticipated. Pattern is reinforced.

The apply-when-bounded-mechanical-AND-capacity rule (named cycle 18) applied this cycle for the third time. Items 2, 4 were applied; item 3 was a cold-reader (no edit needed); item 6 surfaced three flags that were deferred per the rule (cycle 22 or 23 batching). The deferred flags are documented in this notes file with explicit option-A/option-B framing per the cycle-19 c1.1 pattern.

Long-deferred items: 10 → 9 (item 11 v2 design implications blockquote tightening resolved this cycle).

## Cycle 22+ pre-commits

1. **Cross-system transferability observations integration into `1-research.md`'s Cross-system observations section** (cycle-20 item 5; cross-system patterns include: c2.1 failed-task-as-recorded-artifact; c2.2 code-vs-prompts-split; embedding-over-summary-not-source pattern from this cycle's c1.1 cold-reader; multi-agent-is-not-a-default cross-system convergence; component-local-persistence pattern; append-only / no-destructive-rollback). Estimated +30-50 lines plus rework of preliminary-observations section. **Focal cycle work.**

2. **Three LangGraph Tier-1 integration flags** (from item 6 this cycle):
   - (a) Add 1 sentence to prose paragraph 5 about durability modes (`exit`/`async`/`sync`)
   - (b) Trim "kitchen-sink avoidance" framing from bullet 19 (or augment bullet to match a non-existent prose paragraph)
   - (c) Delete process-defense closing sentence in paragraph 11 ("This is research-evaluation honesty, not v2-relevance smuggling")
   Bounded mechanical, ~10 lines total. Batchable in a non-focal cycle (cycle 23 if cycle 22 takes item 5).

3. **Cold-reader on this cycle's blockquote tightening (item 2 application)**. Standard cycle-N+1 fresh-eye pass on the v2 design implications blockquote. Specific question: is the option-3-first ordering creating a real preferential-reading risk, or is the same-cycle-cold-reader's "defensible reading: historical ordering" framing sufficient?

4. **Cold-reader on this cycle's compositionality bullet (item 4 application)**. Standard cycle-N+1 fresh-eye pass on the new compositionality bullet. Specific question: is the granularity distinction with bullet 11 ("skill-library as the named learning mechanism") clear enough, or could a reader mis-read the two bullets as duplicative?

## Long-deferred items roll-call (carry-forward)

1. Journal-entry self-congratulation sweep (15 cycles deferred from cycle 7+)
2. F6/F8/F9 measurements (cycle 7+)
3. Refactor-for-length F-section sweep (cycle 8+; Tier-2 group 8)
4. Persistence-mechanism deferred-list consolidation (cycle 13 flag)
5. Tier-2 group 2 (review/disposition substrate, partial integration cycle 13)
6. Tier-2 group 4 (nine measures rework)
7. Tier-2 group 6 (preserved-through-cutover disposition)
8. Tier-2 group 7 (resolved open questions collapse)
9. Tier-2 group 9 (F8 singleton-family acknowledgment)

Net: 10 → 9. Item 11 (v2 design implications blockquote tightening) resolved this cycle.
