# Phase 2 design framework — convergent constraints + design axes for v2 candidates

## Status

**v1.21 (cycle 59, 2026-05-03).** Phase-2-input artifact-in-progress. Subject to
iteration before any Phase 2 candidate generation begins (which itself
requires post-retrospective checkpoint approval).

This file is the **live working framework** that v2 candidates will be
generated from and evaluated against. The frozen historical record of
each iteration step lives in the corresponding `_notes/cycle-N-*.md` file.

### Iteration history

| Version | Cycle | Source | Summary of changes |
|---|---|---|---|
| v1.0 | 35 (2026-04-30) | `_notes/cycle-35-phase2-design-axes-and-cold-reader.md` | Initial Phase 2 design-axes synthesis: 7 convergent constraints + 11 axes + cross-axis dependency map + F-pattern→axis mapping + Phase 2 candidate template + 5 open framework questions |
| v1.1 | 36 (2026-04-30) | `_notes/cycle-36-cold-reader-and-framework-iteration.md` | Cold-reader on v1.0: F11→Axis 9 corrected to F11→Axis 4+Axis 2 (Q[b] FAIL); Axis 2 plans-as-artifacts row removed (Q[a] cleanup); decisions documented for 5 open questions, deferred to v1.2 application |
| v1.2 | 37 (2026-05-01) | `_notes/cycle-37-framework-v1.2-application-and-cold-reader.md` | Framework promoted to dedicated file `2-design-framework.md`. Six deferred decisions applied: Q[c] constraint 7 wording refinement, Q1 Axis 11→constraint 8 promotion, Q2 Axis 12 (Reconciliation discipline) added, Q3 ordering disclaimer added, Q4 Axis 13 (Harness-vs-session boundary) added, Q5 preserved-primitives subsection added. Cycle-37 cold-reader correction: F11 mapping refined to Axis 4+Axis 12 (drop Axis 2 from direct mapping; document indirect contribution in cross-axis deps). |
| v1.3 | 38 (2026-05-01) | `_notes/cycle-38-cold-reader-and-v1.3-application.md` | Cycle-38 cold-reader on v1.2: three pre-commit questions all PASS (F11→Axis 4+Axis 12 stands; Axis 13 medium-vs-fat is real differentiation; constraint 8 is meaningful). Two refinements: (i) add "Axis 13 × Axis 7" cross-axis dependency (situational-review implementation strategy); (ii) backfill Maps-to lines on Axes 1, 3, 5, 6, 7 (cycle-37 same-cycle review minor finding (5) inherited from v1.0/v1.1). One flag for cycle-39+ verification: Axis 12 "v1-derived" caveat may be too strong given LangGraph interrupts as broader-axis analogue. |
| v1.4 | 39 (2026-05-01) | `_notes/cycle-39-cold-reader-and-redispatch-escalation.md` | Cycle-39 cold-reader on v1.3: three pre-commit questions all PASS again (re-dispatch trigger did NOT fire on comment-on-existing-issue; Axis 12 "v1-derived" caveat correct; F-pattern table levels correct including F9→Axis 7 with Axis 13 indirect via cross-axis deps). Cycle-38 "v1-derived caveat may be too strong" flag verified-and-retired: HITL primitives in LangGraph/AutoGen are synchronous pause-resume mechanisms, structurally different from async reconciliation; clarification sentence added to Axis 12 Status. Cycle-39 cold-reader OVERCAUTIOUS finding mirrors cycle-37's Q[b] OVERCAUTIOUS pattern. |
| v1.5 | 41 (2026-05-01) | `_notes/cycle-41-deeper-read-per-finding-evaluation.md` | Cycle-41 substantive findings from PR #2804 + #2805 deeper-read deliverables (Cognition + OpenAI). Three Cognition framework corrections applied: (i) Axis 1 — Cognition's "Don't Build Multi-Agents" (June 2025) was substantially walked back in April 22, 2026 follow-up; durable invariant is **writes-stay-single-threaded**, not single-threaded execution; Cognition now ships Managed Devins (coordinator + parallel children) and joins the small-fixed-team row; (ii) Axis 3 — Cognition has multi-layer memory architecture (5+ documented mechanisms); context-trace framing qualified to "primary in-session mechanism, multi-layer at longer horizons"; (iii) Axis 9 — 45-min session limit is unverified after direct primary-source access (docs say "if you can do it in three hours"); status downgraded from `documented-claim` to `unverified-after-direct-access`. Plus: Axis 9 OpenAI counter-evidence (Ralph Wiggum Loop has no iteration ceiling — pattern does NOT transfer to cron-driven systems). Plus: Status header v1.3→v1.5 freshness fix (was missed cycle-39 v1.4 bump). Two flags for cycle-42+: Axis 12 "Most likely v2 candidate position" annotation softening (Q[b] cold-reader); openclaw deeper-read dispatch (Q[c] BORDERLINE-PASS). |
| v1.6 | 42 (2026-05-01) | `_notes/cycle-42-cold-reader-and-v1.6-application.md` | Cycle-42 cold-reader on v1.5: Q(a) found two internal inconsistencies introduced by v1.5's Axis 1 Cognition update that were not propagated to other framework sections — applied. Q(b) PASS — C7 (microVM) and O7 (companion post) qualifications adequately propagated to per-system files; no over-acceptance. Q(c) decided both deferred flags warrant cycle-42 action — Axis 12 hybrid annotation softening applied; openclaw deeper-read dispatch executed via close-and-recreate primitive. Three v1.6 changes: (i) Axis 7 row — Cognition moved from "Single-pattern (one shape only)" to multi-pattern coexisting (Apr 2026 ships Managed Devins + Devin Review + Smart Friend — system-level multi-pattern); (ii) Cross-axis dependency map (Constraint 8 × Axis 1) — stale "(Cognition)" parenthetical example removed since Cognition now in small-fixed-team row; (iii) Axis 12 hybrid row annotation — replaced "Most likely v2 candidate position" (forward-looking forecast that could prejudice Phase 2 candidate generation) with cost-grounded descriptive reasoning ("Lowest per-channel design cost — different channels have different frequencies"). |
| v1.7 | 43 (2026-05-01) | `_notes/cycle-43-openclaw-per-finding-evaluation.md` | Cycle-43 substantive findings from PR #2809 deeper-read deliverable (openclaw, 893 lines, primary-source). 21 findings evaluated, 21 accepted (4 with qualification, 1 as revision-of-prior-claim, 0 rejected). Three framework changes: (i) Axis 2 row — openclaw added to "File-per-component" position with `global-state.ts` caveat (per-agent state isolation in `~/.openclaw/agents/<agentId>/`; Gateway-level globals exist per `src/global-state.ts` but contents not verified); 4-system support; (ii) Axis 3 row — openclaw note refined to clarify singleton-slot scope is the storage/retrieval layer, not full memory architecture (full architecture is layered: Markdown files + SQLite + active-memory sub-agent + dreaming consolidation); (iii) Axis 9 row — openclaw added to "Runtime ceiling" position with 48h-effectively-unbounded qualifier; stuck-session watchdog (`diagnostics.stuckSessionWarnMs`) noted as more interesting primitive than the bare timeout. Plus cycle-43 Q(c) cold-reader refinement: Axis 12 four-position table cost-framing balanced — "High-cost" → "Uniform mechanism (one pattern per channel); per-channel implementation overhead" on Active polling; "Lowest per-channel design cost" → "Mixed mechanism; design overhead spread per-channel-class rather than per-channel" on Hybrid. Also: cycle-40 v2-design observation about three reconciliation patterns refined (NOT retired) — openclaw's pattern is implementation-detail within Axis 12's existing **Event-driven** position, not a new axis position; the cross-system observation is now TWO axis-distinct patterns (sync HITL vs async) with implementation-nuance within async (cron+catchup, event-driven with persistent connections, webhook-on-event). |
| v1.8 | 44 (2026-05-01) | `_notes/cycle-44-cold-reader-and-v1.8-application.md` | Cycle-44 cold-reader on v1.7: Q(a) BORDERLINE-FAIL on Axis 2 × Axis 3 cross-axis dep map phrasing precision (cycle-43 same-cycle Q1 had flagged as minor; cycle-44 cross-cycle escalated to load-bearing). Three changes applied: (i) Axis 2 × Axis 3 dep map rewritten to name specific Axis 3 positions that align with each Axis 2 position (file-per-component aligns with three filesystem-based memory positions: singleton plugin slot WITH filesystem storage, top-level architectural principle with filesystem memory, wiki+search with file-per-entry); (ii) F8 mapping rationale extended to mention stuck-session watchdog as detection-and-recovery primitive alongside Bounded loops as prevention primitive (covers v1.7-introduced openclaw `diagnostics.stuckSessionWarnMs` instance); (iii) Axis 12 event-driven annotation wording-symmetry — added "shared inbound infrastructure" framing alongside the "requires X" cost framing (event-driven uses shared infrastructure: one webhook handles all subscribed channels, vs N readers for active polling). Q(b) PASS — per-finding evaluation calibration adequate (OC9 qualification appropriately hedged, OC13 verdict consistent with pattern transfer to existing COPILOT-DISPATCHES three-tier structure). Q(c) PASS — cost-framing balance adequate; minor wording-symmetry opportunity (item iii above) addressed. |
| v1.9 | 45 (2026-05-02) | `_notes/cycle-45-cold-reader-and-v1.9-application.md` | Cycle-45 cold-reader on v1.8: Q(a) PASS (Axis 2 × Axis 3 dep map closing caveat "supportive rather than exclusive" with explicit "non-filesystem Axis 3 positions" catchall covers file-per-component + non-filesystem-memory scenario); Q(b) PASS (stress test file-per-component + context-trace memory is explicitly named in caveat as non-precluded); Q(c) BORDERLINE-FAIL escalation from cycle-44's BORDERLINE-PASS — F8 mapping rationale's "or" between bounded-loops and stuck-session-watchdog covers Axis 9 positions 2-3 (Loop count, Runtime) but doesn't explicitly name position 4 (Both). Cross-cycle review surfaces the structural difference vs F5's "or" (Axis 2 has no "both" position; Axis 9 does have a composable Both position). One change applied: F8 mapping rationale rewritten as comma-separated three-way enumeration ("Bounded loops, stuck-session watchdog, or both compositionally (Axis 9's `Both (loop + runtime)` position)"). |
| v1.10 | 47 (2026-05-02) | `_notes/cycle-47-cold-reader-and-v1.10-application.md` | Cycle-47 cold-reader on v1.9: Q(a) PASS on F8 rationale (re-walked with fresh adversarial framing including the "+single-implementation discipline" CDP-citation distinction; F8's CDP column citation is content-driven distinct from F1/F6/F7's Axis 13 citation). Q(b) BORDERLINE-FAIL on cross-axis dep map sweep — Axis 13's Maps-to is missing the F9 indirect-contributor annotation that Axis 1's Maps-to has, even though the Axis 13 × Axis 7 dep map entry documents Axis 13's role in F9 ("Axis 13 shapes the implementation strategy"). The asymmetry was unintentional: cycle-39 verified Axes 1/3/5/6/7 backfilled Maps-to lines but Axis 13's Maps-to (added at v1.2) was not re-reviewed. One change applied: Axis 13 Maps-to extended with "Indirect contributor to F9 (adversarial-review treadmill) — fat-harness shapes the implementation strategy for Axis 7's situational-review by controlling when review fires; the load-bearing F9 fix is Axis 7." F-pattern table NOT modified (preserves cycle-39's explicit verdict that "F-pattern table levels are correct; the cross-axis dep is the right level for Axis 13 × Axis 7's contribution to F9"). Q(c) procedural decision: bounded-mechanical capacity for #809 closure consideration. |
| v1.11 | 48 (2026-05-02) | `_notes/cycle-48-cold-reader-and-v1.11-application.md` | Cycle-48 cold-reader on v1.10: Q(a) PASS on Axis 13 Maps-to three-clause structure (re-walked with fresh adversarial framing on `fat-harness` mechanism, `thin/medium` lumping, and Axis 7 terminology; lumping is consistent with cross-axis dep map and correct for default reading — contingent medium-harness extraction of WHEN-review is candidate-specific, not load-bearing precision gap). Q(b) BORDERLINE-FAIL under continued Maps-to ↔ F-pattern table consistency sweep — Axis 8's Maps-to is missing F7 even though F7 row lists "Axis 1, Axis 8, Axis 9, Axis 13" and the F7 rationale names "mechanical enforcement" (= Axis 8) as one of the four mechanisms reducing self-management surface. The asymmetry is unintentional: Axis 8 Maps-to was set at v1.0 (cycle 35) and not modified since; cycle-38's backfill covered Axes 1/3/5/6/7 (not Axis 8); cycle-39's verification scope was the 5 backfilled lines (not Axis 8). One change applied: Axis 8 Maps-to extended with "F7 (self-management dominance — mechanical enforcement reduces orchestrator constraint-tracking burden)" — direct contributor matching F7 row's listing (not indirect; Axis 8 IS in F7 row). F-pattern table NOT modified (preserves F7 row verdict). Q(c) procedural decision: v1.11 application is sole bounded-mechanical work this cycle (other options gated or capacity-exceeded). Borderline flagged for cycle-49: Axis 12 → F3 "partial" qualifier potentially ambiguous (partial-of-aspect vs partial-of-pattern) — wordsmith-borderline, not load-bearing. |
| v1.12 | 49 (2026-05-02) | `_notes/cycle-49-cold-reader-and-v1.12-application.md` | Cycle-49 cold-reader on v1.11: Q(a) PASS on Axis 8 Maps-to F7 addition (re-walked with fresh adversarial framing on wording symmetry with F7 row, style parallel with Axis 1 → F7, style asymmetry within Axis 8's Maps-to, and cross-history check; rationale clause "mechanical enforcement reduces orchestrator constraint-tracking burden" is content-driven specific subset of F7 row's "self-management surface" — defensible). Q(b) PASS on structural sub-lens (third application of Maps-to ↔ F-pattern table sweep finds zero new structural asymmetries in either Direction 1 or Direction 2 — the structural sub-lens has converged in 3 cycles: cycle-47 found 1 gap, cycle-48 found 1 gap, cycle-49 finds 0 gaps); BORDERLINE-FAIL on wordsmith sub-lens (Axis 12 → F3 "partial" qualifier from cycle-48 flag stands as borderline-ambiguous between partial-of-pattern and partial-of-aspect readings). One change applied: Axis 12 → F3 wordsmith fix — replaced "partial" with "post-close aspect" plus brief reference to F3 row's other aspect ("F3 row's other aspect is Axis 2's single-source-of-truth"). Single-cell wordsmith fix; preserves F3 row verdict; mirrors F3 row's divide-and-conquer framing. Lens-and-sub-lens model refinement (cycle-49): a lens (Maps-to ↔ F-pattern table) can have STRUCTURAL and WORDSMITH sub-lenses that converge separately. Q(c) procedural decision: v1.12 wordsmith fix is sole bounded-mechanical work this cycle (other options gated). Wordsmith sub-lens scan complete: only Axis 12 → F3 borderline identified across all Maps-to entries; no other wordsmith borderlines. |
| v1.13 | 50 (2026-05-02) | `_notes/cycle-50-cold-reader-and-v1.13-application.md` | Cycle-50 cold-reader on v1.12: Q(a) PASS on Axis 12 → F3 wordsmith fix (re-walked with fresh adversarial framing on disambiguation strength of "post-close aspect" vs "partial", placement of F3-row pointer, and weight of semicolon-separated qualifier-plus-pointer style; the fix achieves disambiguation, the pointer is content-driven serving candidate-author divide-and-conquer workflow, and word-count is in middle range — defensible. Minor durability concern flagged: "F3 row's other aspect" assumes exactly two aspects, fragile to future F3 mapping changes; bounded and acceptable). Q(b) FIRST APPLICATION of NEW lens (Position table consistency sweep): structural sub-lens PASS (cross-reference completeness across all 12 axes' position tables; shorthand position-label references in cross-axis dep prose are content-driven internal-shortenings consistent with framework convention; Axis 9/10 "Both" composable positions with empty Systems supporting are correctly listed and documented as "Composable"; backticks-vs-prose convention consistent — code identifiers in backticks, position labels in prose); wordsmith sub-lens BORDERLINE-FAIL on default-position framing variation across Axes 5, 8, 9, 10 — Axis 5's "No — plans live in-message..." Systems-supporting cell uses population-framing ("Most others (none explicitly support reconstruction-after as primitive)") while Axis 8's "None" cell uses default-framing ("Default in absence of explicit infrastructure"); the Axis-5-vs-Axis-8 column-content asymmetry is the most divergent default-position framing in the table. One change applied: Axis 5's "No" row Systems-supporting cell harmonized to match Axis 8's "Default in absence of [domain] infrastructure" framing — Systems = "Default in absence of plan-artifact infrastructure"; Notes = "Most surveyed systems by default; none explicitly supports reconstruction-after as a primitive" (column swap with light rephrase preserving population-claim and explicit-support-hedge). Axis 9/10 lighter variations ("Rare in surveyed", "Default") retained as acceptable variants of the default-framing convention; harmonizing further would expand scope beyond bounded-mechanical magnitude. Q(c) procedural decision: v1.13 single-row default-position-framing fix is sole bounded-mechanical work this cycle (other options gated per cycle-46+ checkpoint reasoning). Cycle-49's lens-and-sub-lens model VALIDATED: cycle-50's Position table consistency lens has at least two sub-lenses (structural and wordsmith) that converge separately, mirroring the Maps-to ↔ F-pattern table parent lens's structure. Position table cross-reference structural sub-lens PASSES on first application — earlier prediction of "≥1 finding on first application" is FALSIFIED FOR STRUCTURAL but met for wordsmith; refines the cycle-47 first-application-finds-≥1-gap hypothesis to "≥1 finding across structural OR wordsmith sub-lenses on first application". |
| v1.14 | 51 (2026-05-02) | `_notes/cycle-51-cold-reader-and-v1.14-application.md` | Cycle-51 cold-reader on v1.13: Q(a) PASS on Axis 5 default-position framing harmonization (re-walked with fresh adversarial framing on population-claim preservation, no-explicit-support-hedge preservation, Axis 8 pattern symmetry, and candidate-author scanning improvement; the v1.13 column-swap preserves both population-claim and explicit-support-hedge in Notes column while harmonizing Systems column to "Default in absence of [domain] infrastructure" — defensible across all four probes). Q(b) FIRST APPLICATION of NEW lens (Cross-axis dep map ↔ Maps-to consistency sweep): structural sub-lens BORDERLINE-FAIL — Axis 13's per-axis Cross-axis dependency subsection is missing the × Axis 7 entry, even though Axis 13 × Axis 7 is established in (a) the global cross-axis dep map (cycle 38 added it), (b) Axis 7's per-axis Cross-axis subsection (cycle 38 backfilled it), and (c) Axis 13's Maps-to indirect-F9 annotation (cycle 47 added it). Cross-history check: cycle 37 created Axis 13's Cross-axis subsection with × Axis 6 and × Axis 8; cycle 38 added Axis 13 × Axis 7 to global + Axis 7's subsection but did NOT backfill to Axis 13's subsection; cycle 47 added F9 indirect via Axis 7 to Axis 13's Maps-to but again did NOT backfill to Axis 13's Cross-axis subsection. Wordsmith sub-lens PASS — vocabulary diversity in cross-axis dep entries (forces / enables / supports / pairs naturally / constrains / implies) is content-driven per cycle-47 observation, not a wordsmith failure; sweep of all global cross-axis dep entries against per-axis subsection entries finds no wordsmith borderlines beyond the structural gap. Two changes applied: (i) Axis 13's Cross-axis dependency subsection extended with × Axis 7 entry — single-clause rationale matching the format of the existing × Axis 6 and × Axis 8 entries ("fat-harness can implement Axis 7's multi-pattern situational-review by controlling when review fires (vs thin/medium harness leaving WHEN-review decisions in prompt)"), preserves cycle-38's deliberate ratification of the Axis 13 × Axis 7 dep with mediation chain; numerical ordering 6/7/8 preserved; (ii) Status header freshness fix v1.12 → v1.14 (cycle 50 missed the v1.13 bump, parallel to cycle-41 fixing cycle-39's missed v1.4 bump). Cross-axis dep map global section NOT modified — Axis 13 × Axis 7 entry is already comprehensive there with F9 mediation. F-pattern table NOT modified — preserves cycle-39 + cycle-47 verdicts. Q(c) procedural decision: v1.14 single-row Cross-axis subsection extension + Status header freshness fix is sole bounded-mechanical work this cycle (other options gated per cycle-46+ checkpoint reasoning). Bundled freshness fix is parallel to cycle-41 v1.5 pattern. Per-lens convergence hypothesis SUPPORTED for first application of Cross-axis dep map ↔ Maps-to consistency lens — wordsmith sub-lens PASS on first application (parallel to cycle-50's Position table structural sub-lens PASS on first application: lens domain maturity reduces first-application discovery rate); structural sub-lens BORDERLINE-FAIL with single-cell fix (parallel to cycle-50's Position table wordsmith sub-lens BORDERLINE-FAIL on first application); cycle-50 refinement "≥1 finding across structural OR wordsmith sub-lenses on first application" SUPPORTED in third parent lens application. |
| v1.15 | 52 (2026-05-02) | `_notes/cycle-52-cold-reader-and-v1.15-application.md` | Cycle-52 cold-reader on v1.14: Q(a) PASS on Axis 13 Cross-axis subsection × Axis 7 entry confirmation (re-walked with fresh adversarial framing on wording mirror with Axis 7's subsection from Axis 13's POV, parenthetical contrast vs internal-semicolon vs separate sentence vs inline-vs, numerical ordering 6/7/8, bundled Status header freshness fix integration, and adversarial probe on whether the entry leaks F9 mediation that should live in global section only; all six probes confirm v1.14 fix is content-driven — the parenthetical-instead-of-semicolon avoids entry-separator collision, the compression of Axis 7's "supporting Axis 7's multi-pattern situational invocation" is appropriate from-Axis-13's-POV framing because Axis 13's entry doesn't need to explain what Axis 7 supports, F9 mediation correctly lives in global + Axis 13 Maps-to only). Q(b) SECOND APPLICATION of Cross-axis dep map ↔ Maps-to consistency lens: structural sub-lens BORDERLINE-FAIL — Axis 1's per-axis Cross-axis dependency subsection is missing × Axis 12 entry, even though Axis 12 × Axis 1 IS established in (a) the global cross-axis dep map (cycle 37 added it as part of Axis 12 v1.2 introduction) and (b) Axis 12's per-axis Cross-axis subsection (cycle 37). Cross-history check: Axis 1's subsection was created at v1.0 (cycle 35) with × Axis 7 only; cycle 37 added Axis 12 to framework with its own Cross-axis subsection listing × Axis 4 and × Axis 1, but did NOT backfill Axis 1's subsection with × Axis 12 back-reference; cycles 41 (Cognition update) and 45 (Cognition refinement) modified Axis 1's row content but didn't touch Cross-axis subsection. Triangulation across 3 expected locations (vs cycle-51's 4 due to no Axis 1 Maps-to → Axis 12 component): 2 of 3 present (global ✓; Axis 12 subsection ✓; Axis 1 subsection ✗). Wordsmith sub-lens PASS — sweep across all per-axis subsection and global cross-axis dep entries finds no qualifier-ambiguity, no terminology drift, no clause-structure inconsistency beyond cycle-51-noted content-driven variation (Axis 1 × Axis 7 vs Axis 13 × Axis 7 mediation-detail asymmetry remains content-driven). One change applied: Axis 1's Cross-axis dependency subsection extended with × Axis 12 entry — single-clause two-part rationale ("small-fixed-team enables a dedicated reconciliation agent; single-threaded must interleave reconciliation with primary work") with mirror-from-global ordering (small-fixed-team-enabling first, single-threaded-constraint second, matching global section's Axis 12 × Axis 1 ordering); terse "dedicated reconciliation agent" preserves global's load-bearing primitive without parenthetical role-naming (per per-axis subsection convention); numerical ordering 7, 12 preserved (ascending). Cross-axis dep map global section NOT modified — Axis 12 × Axis 1 entry is already comprehensive there. Axis 12's subsection NOT modified — already lists × Axis 1. F-pattern table NOT modified. Q(c) procedural decision: v1.15 single-row Axis 1 Cross-axis subsection extension is sole bounded-mechanical work this cycle (other options gated per cycle-46+ checkpoint reasoning); no bundled Status header freshness fix needed (cycle-51 caught up). Per-lens convergence hypothesis REFINED — cycle-52 evidence (1 finding cycle-51 + 1 finding cycle-52 with ~5 known asymmetries still remaining: Axis 3 × Axis 1 unilateral disposition decision, Axes 4/5/6/8/9/10 missing subsections + their respective dep entries) introduces total-asymmetry-set-size as a factor in convergence shape. Small-set lenses (Maps-to ↔ F-pattern table, ~2 total asymmetries) exhaust within 3 cycles under single-cell discipline (1 → 1 → 0 cycle-47-49 cleanly). Large-set lenses (Cross-axis dep map ↔ Maps-to, ~7+ total asymmetries) require many more cycles under single-cell discipline OR multi-cell batch fix; the 1-per-cycle pattern reflects discipline-choice not exhaustion. Single-cell discipline preserved cycle-52 (matches cycle-47/48/49/50/51 cadence). |
| v1.16 | 53 (2026-05-03) | `_notes/cycle-53-cold-reader-and-v1.16-application.md` | Cycle-53 cold-reader on v1.15: Q(a) PASS on Axis 1 × Axis 12 entry confirmation re-walk (re-walked with fresh adversarial framing on wording mirror with global section's Axis 12 × Axis 1 ordering, ordering inversion vs existing × Axis 7 entry within same Axis 1 subsection, numerical ordering 7→12 ascending, parenthetical role-name absence vs global's role-naming "(the 'curator' or 'reconciler' role)"; v1.15 fix preserves global's clause ordering (small-fixed-team-first), the within-subsection ordering inversion (single-threaded-first for × Axis 7 vs small-fixed-team-first for × Axis 12) is content-driven matching global's per-dep ordering precedent (global Axis 1 × Axis 7 also leads with single-threaded; global Axis 12 × Axis 1 also leads with small-fixed-team), parenthetical role-naming dropped per per-axis subsection terser-than-global convention; defensible across all four probes). Q(b) THIRD APPLICATION of Cross-axis dep map ↔ Maps-to consistency lens: structural sub-lens BORDERLINE-FAIL on probe (i) — Axis 3's per-axis Cross-axis dependency subsection lists × Axis 1 unilaterally (single-clause "small-fixed-team can have per-agent memory subsystems"); this dep is NOT in global cross-axis dep map AND NOT in Axis 1's per-axis subsection (post-v1.15 Axis 1 subsection lists × Axis 7 + × Axis 12 only). Cross-history check: v1.0 (cycle 35) created Axis 3's subsection with × Axis 2 + × Axis 1 entries; the × Axis 1 dep was never propagated to global section or to Axis 1's subsection — 18-cycle propagation gap (cycles 35→53), parallel to cycle-51's 13-cycle gap (cycle 38→51, Axis 13 × Axis 7) and cycle-52's 15-cycle gap (cycle 37→52, Axis 1 × Axis 12). Disposition decision: ADD to global + backfill Axis 1's subsection — content-driven choice grounded in (a) structural parallelism with Axis 12 × Axis 1 in global (both are "small-fixed-team enables [primitive specialization]" patterns), (b) load-bearing for candidate-author understanding (small-fixed-team's enabling permission for per-agent memory affects candidate design), (c) surveyed-system grounding (openclaw's `~/.openclaw/agents/<agentId>/` per-agent state isolation is one instance). Probe (ii) on missing-subsection convention (Axes 4/5/6/8/9/10) DEFERRED — gated on post-retrospective checkpoint per cycle-52 reasoning (convention question requires Phase 2 candidate-author empirical evidence). Probe (iii) on constraint × axis subsection convention (× Constraint 8 in Axis 1's subsection?) DEFERRED — same gating. Probe (iv) NEWLY DISCOVERED in cycle-53 systematic re-check: Axis 2's per-axis Cross-axis subsection is missing × Axis 4 entry despite Axis 4 × Axis 2 being in global section since v1.0 — back-reference asymmetry parallel to cycle-51 (Axis 13 × Axis 7) and cycle-52 (Axis 1 × Axis 12) shapes. Probe (iv) DEFERRED to cycle-54 to preserve single-cell discipline (cycle-47-onward cadence: one substantive finding per cycle). Wordsmith sub-lens PASS — third application of lens finds no new wordsmith borderlines; sweep across all per-axis subsection and global cross-axis dep entries confirms wordsmith convergence at PASS for second consecutive cycle. Two changes applied (v1.16): (i) global section extended with new "**Axis 3 (memory) × Axis 1 (decomposition):**" entry inserted between existing "Axis 2 × Axis 3" and "Axis 4 × Axis 2" entries (primary-axis-ascending ordering preserved) — single-clause "Small-fixed-team can have per-agent memory subsystems (openclaw's per-agent state isolation in `~/.openclaw/agents/<agentId>/` is one surveyed instance)" mirrors Axis 3's subsection wording while adding surveyed-system grounding (parallel to Constraint 8 × Axis 1's Cognition Managed Devins reference); single-clause (vs Axis 12 × Axis 1's two-clause) because the single-threaded equivalent ("uses one shared memory subsystem") is tautological for memory shape rather than load-bearing constraint; (ii) Axis 1's per-axis Cross-axis subsection extended with × Axis 3 entry inserted at start (numerical ordering 3, 7, 12 ascending; matches cycle-52's "ascending dep-partner numerical ordering" convention) — single-clause terser-than-global "Axis 1 × Axis 3 (memory) — small-fixed-team can have per-agent memory subsystems" preserves load-bearing primitive while dropping openclaw-instance detail per per-axis subsection convention. Cross-axis dep map global section's existing entries NOT otherwise modified. Axis 3's subsection NOT modified — already lists × Axis 1. F-pattern table NOT modified. Q(c) procedural decision: v1.16 (probe (i) two-cell global + Axis 1 subsection fix) is sole bounded-mechanical work this cycle; probe (iv) deferred to cycle-54; convention probes (ii)/(iii) deferred to checkpoint. Per-lens convergence hypothesis EXTENDED — cycle-53 evidence introduces SUB-SHAPE distinction within large-set lens domain: cycles 51-52 surfaced back-reference asymmetries (back-ref shape: dep present in global + partner-axis subsection but missing in self-axis subsection); cycle-53 surfaces unilateral-mention disposition decision (disposition shape: dep present only in one per-axis subsection, decision needed on whether to propagate to global + other subsection or remove). The hypothesis refinement: large-set lens domain has multiple structural sub-shapes; single-cell discipline addresses one sub-shape per cycle; convergence shape depends on sub-shape distribution within set. Cycle-53 also falsifies cycle-52's "≤1 finding under single-cell discipline" prediction (cycle-53 finds 2: probe (i) Axis 3 × Axis 1 disposition + probe (iv) Axis 2 × Axis 4 back-reference); the falsification reveals that systematic re-checks beyond pre-commit-named probes can surface additional asymmetries. Single-cell discipline preserved cycle-53 (1 fix applied; 1 deferred). |
| v1.17 | 54 (2026-05-03) | `_notes/cycle-54-cold-reader-and-v1.17-application.md` | Cycle-54 cold-reader on v1.16: Q(a) PASS on Axis 3 × Axis 1 v1.16 confirmation re-walk (re-walked with fresh adversarial framing on six probes — wording mirror across Axis 3 subsection / global / Axis 1 subsection three locations, single-clause structure vs Axis 12 × Axis 1 two-clause defensibility, openclaw reference appropriateness vs other surveyed-system-instance citations, numerical ordering 3, 7, 12 ascending in Axis 1 subsection, primary-axis-ascending ordering preservation in global section insertion, no global-only content leak into Axis 1 subsection; v1.16 fix preserves all six probes — terser-than-global wording in both per-axis subsections matches convention, single-clause is content-driven because single-threaded equivalent ('uses one shared memory subsystem') is tautological for memory shape rather than load-bearing constraint, openclaw reference matches Constraint 8 × Axis 1's Cognition Managed Devins citation pattern, numerical ordering preserved, primary-axis-ascending preserved with insertion between Axis 2 × Axis 3 and Axis 4 × Axis 2). Q(b) FOURTH APPLICATION of Cross-axis dep map ↔ Maps-to consistency lens: structural sub-lens BORDERLINE-FAIL on probe (iv) (cycle-53 deferred) — Axis 2's per-axis Cross-axis dependency subsection missing × Axis 4 entry despite Axis 4 × Axis 2 being in global since v1.0; cross-history check confirms 18-cycle propagation gap (cycle 35 → cycle 53/54), parallel to cycle-51 (13-cycle), cycle-52 (15-cycle), cycle-53 probe (i) (18-cycle) gaps. Wordsmith sub-lens PASS — third consecutive PASS (cycles 52, 53, 54 — converged). One change applied (v1.17): Axis 2's Cross-axis dependency subsection extended with × Axis 4 entry as second entry — single-clause two-part rationale ("file-per-component pairs naturally with one-way migration or git; typed-channel-map pairs with branching checkpoints") mirroring global Axis 4 × Axis 2 wording without F11 indirect-contribution detail (per per-axis subsection terser-than-global convention; F11 mediation lives in global section's Axis 4 × Axis 2 entry only); position-naming convention (file-per-component, typed-channel-map) preserved matching existing × Axis 3 entry's position-naming pattern; semicolon-separator convention preserved matching cycle-52's Axis 1 multi-entry pattern (Axis 1 × Axis 3; Axis 1 × Axis 7; Axis 1 × Axis 12); numerical ordering 3, 4 ascending preserved (matches cycle-52 ascending convention). Q(c) procedural decision: v1.17 single-cell Axis 2 Cross-axis subsection extension (probe iv fix) is sole bounded-mechanical work this cycle. Systematic re-check (expanded scope per cycle-53 expanded-scope hypothesis) sweeps both (a) per-axis subsection ↔ global ↔ partner-axis subsection symmetry across all 12 axes and (b) Maps-to ↔ cross-axis dep map mediation symmetry across all 12 Maps-to lines: ZERO new structural findings beyond probe (iv); all per-axis subsection entries triangulate with global (post-v1.17); Maps-to indirect-contributor annotations either correspond to existing cross-axis dep map entries (Axis 2 → F11 via Axis 4 + Axis 12 captured in Axis 4 × Axis 2 entry; Axis 13 → F9 via Axis 7 captured in Axis 13 × Axis 7 entry) or don't require cross-axis dep entries (Axis 1 → F9 via dedicated-reviewer-role is intrinsic Axis 1 specialization mediated by candidate's Axis 7 choice — content-driven per cycle-53 confirmation; Axis 3 → F7 via cold-start cost is intrinsic Axis 3 property). Cross-axis dep map global section NOT modified. Other axes' subsections NOT modified. F-pattern table NOT modified. Per-lens convergence hypothesis SUPPORTED — back-reference sub-shape EXHAUSTED after v1.17 within current asymmetry set (cycle-54 finds 0 new back-reference asymmetries beyond cycle-53's deferred probe iv); disposition sub-shape EXHAUSTED at v1.16 (cycle-53 found only one disposition asymmetry — Axis 3 × Axis 1); convention sub-shape (probes ii, iii) remains DEFERRED to checkpoint per cycle-46-onward gating. Cycle-53 prediction "1 → 1 → 1 → 1 → 0 over cycles 51-55" CONFIRMED for cycles 51-54 (4 cycles of single-cell findings); cycle-55 should be the first "0" structural finding under single-cell discipline IF convention sub-shape continues deferred. Lens-and-sub-lens model further validated: large-set lens domain has multiple structural sub-shapes; back-reference sub-shape (cycles 51-54) had 4 instances; disposition sub-shape (cycle 53) had 1 instance; convention sub-shape remains deferred. The pattern of v1.0 framework-creation cycle leaving systematic propagation gaps is confirmed across 4 cycles (cycles 51, 52, 53, 54) of catching up to 2-pass-discipline retrospectively. Single-cell discipline preserved cycle-54 (1 fix applied; 0 deferred). |
| v1.18 | 55 (2026-05-03) | `_notes/cycle-55-cold-reader-and-v1.18-application.md` | Cycle-55 cold-reader on v1.17: Q(a) PASS on Axis 2 × Axis 4 v1.17 confirmation re-walk (re-walked with fresh adversarial framing on six probes — wording mirror with global Axis 4 × Axis 2 entry, position-naming convention preservation, semicolon-separator preservation under entry-separator-collision adversarial probe, numerical ordering 3, 4 ascending preservation, F11 indirect-contribution annotation appropriately localized to global + Axis 2 Maps-to without triple-coverage in subsection, repo-as-state position omission consistent with global pre-v1.18 — defensible across all six probes; sixth consecutive Q(a) PASS in v1.X sequence). Q(b) FIFTH APPLICATION of Cross-axis dep map ↔ Maps-to consistency lens: structural sub-lens BORDERLINE-FAIL on NEW global-completeness sub-shape — global "Axis 4 × Axis 2" entry enumerates only 2 of 3 defensible Axis 2 positions (file-per-component, typed-channel-map; missing repo-as-state). Repo-as-state Axis 2 forces git-as-substrate Axis 4 (state and history collapse into the same git primitive) — load-bearing for candidate-author understanding because absence of the pairing in global cross-axis dep map misrepresents Axis 2's enumeration; position-table notes (Axis 2 repo-as-state row "git substrate"; Axis 4 git-as-substrate row "Repository as state") provide partial workaround but cross-axis dep map should be primary structural-pattern surface. Cross-history check: global Axis 4 × Axis 2 entry created at v1.0 (cycle 35) with file-per-component + typed-channel-map enumeration; repo-as-state Axis 2 was added to position table at v1.0 with explicit "git substrate" cross-reference but the cross-axis dep entry was never expanded — 20-cycle propagation gap (cycles 35→55), parallel to cycles 51-54 18-cycle gaps. Cycle-54's exhaustion prediction "0 findings cycle-55" FALSIFIED by NEW global-completeness sub-shape; cycle-54 explicitly named this falsification path ("if cycle-55 confirms load-bearing, apply as cycle-55 v1.18 single-cell global enrichment + Axis 2 subsection re-extension"). Sub-shape inventory NOW: back-ref (cycles 51-54, 4 instances, EXHAUSTED), disposition (cycle 53, 1 instance, EXHAUSTED), convention (probes ii/iii, DEFERRED), global-completeness (cycle 55, 1 instance, NEW — application this cycle). Mediation symmetry sub-shape (expanded scope per cycle-54): re-confirmed cycle-54 verdict ZERO new findings. Wordsmith sub-lens PASS — fourth consecutive PASS (cycles 52, 53, 54, 55 — converged). Two changes applied (v1.18): (i) global "Axis 4 × Axis 2" entry extended with new third clause "repo-as-state forces git-as-substrate (state and history collapse into the same git primitive — the tree is state, the chain of commits is history)" inserted between "branching checkpoints" and "*Indirect F11 contribution*" — "forces" vocabulary used (vs "pairs naturally" elsewhere in entry) because coupling is structurally tighter (repo-as-state's only coherent Axis 4 partner is git-as-substrate — branching checkpoints, versioned files, one-way migration are incoherent with commits-as-state); parallel to "single-threaded forces single-topology" in Axis 1 × Axis 7 entry; F11 indirect-contribution annotation preserved in trailing italics; (ii) Axis 2's per-axis Cross-axis dependency subsection × Axis 4 entry re-extended as third clause "repo-as-state forces git-as-substrate" — drops the parenthetical explanation per per-axis subsection terser-than-global convention; preserves enumeration symmetry with global (3 of 3 defensible Axis 2 positions in both locations). Other per-axis subsections NOT modified. F-pattern table NOT modified. Q(c) procedural decision: v1.18 (NEW global-completeness sub-shape two-cell fix — global enrichment + Axis 2 subsection re-extension) is sole bounded-mechanical work this cycle. Single-cell discipline preserved as one coherent fix-decision propagated across two locations (parallel to cycle-53 disposition-shape two-cell pattern). Per-lens convergence hypothesis FURTHER REFINED — sub-shape inventory is incomplete; new sub-shapes surface as systematic re-check expands scope (cycles 51-54 found back-ref + disposition; cycle-55 found global-completeness via cycle-54 explicit Q(b) probe naming). Refined cycle-56 prediction: 0 findings under back-ref + disposition + global-completeness sub-shapes IF post-v1.18 re-check finds no additional global-completeness instances; convention sub-shape remains deferred to checkpoint. Single-cell discipline preserved cycle-55 (1 fix applied; 0 deferred). |
| v1.19 | 57 (2026-05-03) | `_notes/cycle-57-cold-reader-and-v1.19-application.md` | Cycle-57 cold-reader on v1.18: Q(a) VERDICT-SHIFT on cycle-56's BORDERLINE-CONTENT-DRIVEN Axis 13 × Axis 8 verdict (re-walked with NEW within-cross-axis-dep-map self-consistency criterion not applied at cycle-56). Cycle-56's argument was "F1 mapping in F-pattern table covers thin-harness × Axis 8 bifurcated mediation; no v1.19 fix needed." Cycle-57 stress test reveals: within the cross-axis dep map surface itself, all bifurcated entries enumerate both branches (Axis 1 × Axis 7 — single-threaded vs small-fixed-team; Axis 2 × Axis 3 — file-per-component vs typed-channel-map vs repo-as-state; Axis 4 × Axis 2 — 3 of 3 Axis 2 positions post-v1.18; Axis 12 × Axis 1 — single-threaded vs small-fixed-team; Constraint 8 × Axis 1 — both Axis 1 positions; Axis 13 × Axis 7 — fat vs thin/medium harness branches both enumerated). Axis 13 × Axis 8 was the ONLY bifurcated entry omitting the second branch (only fat-harness enumerated); the within-surface self-consistency criterion shifts the verdict from BORDERLINE-CONTENT-DRIVEN to LOAD-BEARING-INCOMPLETE. F-pattern table coverage (cycle-56's defense) is a different-surface argument that doesn't address within-surface consistency; the Axis 13 × Axis 7 entry sets the within-surface precedent for bifurcation enumeration that Axis 13 × Axis 8 should follow. Two changes applied (v1.19): (i) global "Axis 13 × Axis 8" entry extended from single-clause fat-harness-only to bifurcation-enumerating two-sentence structure parallel to Axis 13 × Axis 7 — first sentence keeps "Fat harness implies more mechanical-enforcement surface area in code (more deterministic code to lint and test)"; second sentence adds "Thin/medium harness has prompt as the primary mechanical-enforcement surface area; Axis 8's behavioral-prose CI on prompt contracts addresses the thin-harness surface (see F1 mapping for the bifurcation across Axis 13 positions)"; "in code" qualifier added to first sentence to make the surface-area-type contrast explicit; F1 cross-reference makes the complementary-surface argument explicit within the entry rather than relying on candidate-author cross-reading; (ii) Axis 13's per-axis Cross-axis subsection × Axis 8 entry re-extended terser-than-global with parenthetical bifurcation contrast ("vs thin/medium harness having prompt as primary surface; Axis 8's behavioral-prose CI on prompt contracts addresses the thin-harness surface"), mirroring the parenthetical pattern used in subsection × Axis 7 entry; F1 cross-reference dropped per per-axis subsection terser-than-global convention. Q(b) FIRST APPLICATION of NEW parent lens (Position table ↔ surveyed-system file consistency): structural sub-lens BORDERLINE-FAIL — Axis 13 position table omits surveyed-system anchors that ARE explicitly designated in system files. Specifically: openclaw.md line 538 designates "Axis 13 (Harness-vs-session) — PARTIAL FAT-HARNESS"; openai-harness.md line 260 designates "Axis 13 (Harness-vs-session) → fat-harness CONFIRMED". Framework's Axis 13 position table has only `Position | Notes` columns (no Systems-supporting column, structurally inconsistent with 10 of 12 axes — Axis 12 also omits but justifiably per "v1-derived; not externally validated" framing). Axis 13's framing "cross-cutting CORE-DESIGN-PRINCIPLE elaboration" does NOT justify omitting surveyed-system anchors when system files explicitly anchor at Axis 13 positions. Q(b) finding DEFERRED to cycle-58 per single-cell discipline (cycle-46-onward cadence: one substantive fix per cycle). Cycle-50 first-application hypothesis CONFIRMED for this NEW lens (≥1 finding on first application); fourth parent lens application's first-application finding-rate consistent with cycles 50/51 first applications. Q(c) procedural decision: v1.19 two-cell fix (Q[a] verdict-shift correction) is sole bounded-mechanical work this cycle; Q[b] Axis 13 surveyed-system anchors finding deferred to cycle-58. Single-cell discipline preserved (one coherent fix-decision propagated across two cells). The cycle-57 Q[b] application also serves the cycle-56 meta-observation about Position table parent lens exhaustion being INFERRED rather than DEMONSTRATED — Q[b]'s NEW Position-table-cross-reference parent lens is structurally adjacent to the original Position table parent lens; its first-application finding strengthens (not weakens) the cycle-56 observation that Position table needs dedicated re-attention before promote-to-question-for-eva trigger fully resolves. |
| v1.20 | 58 (2026-05-03) | `_notes/cycle-58-cold-reader-and-v1.20-application.md` | Cycle-58 cold-reader on v1.19: Q(a) PASS on v1.19 Axis 13 × Axis 8 bifurcation completion two-cell fix (re-walked with fresh adversarial framing on four probes — (i) "in code" qualifier defensibility, (ii) F1 SEE pointer cross-reference vs Axis 13 × Axis 7's inline F9 mediation pattern, (iii) bifurcation enumeration parallelism with Axis 13 × Axis 7, (iv) per-axis subsection terser-than-global convention preservation; v1.19 fix preserves all four probes — "in code" qualifier is structurally necessary for first-sentence-vs-second-sentence surface-type contrast (without it, "more mechanical-enforcement surface area" is ambiguous about kind of surface; the parenthetical "more deterministic code to lint and test" is elaborative not type-naming), F1 SEE pointer is content-driven distinct from Axis 13 × Axis 7's inline F9 mediation pattern (F1's bifurcation IS the F-pattern table's full treatment; SEE pointer points; F9's inline mediation is a primary-fix-vs-secondary-shape pattern that's succinct enough to inline; F-pattern reference patterns across bifurcated entries are content-driven by mediation type — italicized indirect for Axis 4 × Axis 2 F11; inline mediation for Axis 13 × Axis 7 F9; SEE pointer for Axis 13 × Axis 8 F1; bifurcated entries without named F-mediation have no F-reference per Axis 1 × Axis 7, Axis 2 × Axis 3, Axis 12 × Axis 1, Constraint 8 × Axis 1 — cross-reference convention IS uniformly applied conditioned on mediation type), bifurcation enumeration parallelism holds (fat-harness branch + thin/medium-harness branch + F-pattern reference; same three-element structure as Axis 13 × Axis 7), per-axis subsection terser-than-global convention preserved (subsection × Axis 8 drops F1 SEE pointer + parenthetical elaboration but retains both branches and Axis 8 mechanism — parallel to subsection × Axis 7 dropping F9 inline mediation + v1-anti-pattern detail). Eighth consecutive Q(a) PASS in v1.X sequence (cycles 50-58 all PASS except cycle-57's first-ever Q(a) verdict-shift on cycle-56's BORDERLINE-CONTENT-DRIVEN). Q(b) SECOND APPLICATION of Position table ↔ surveyed-system file consistency parent lens — applied cycle-57 DEFERRED finding on Axis 13 surveyed-system anchors. Decision space: (a) add Systems-supporting column to Axis 13 position table parallel to 10 of 12 axes' structure; (b) add anchors to existing Notes column parallel to Axis 12's pattern of incidental citations. Cycle-58 chose (a) for full structural consistency — Axis 13 IS externally anchored per cycle-57 verdict (openclaw "PARTIAL FAT-HARNESS" + openai-harness "fat-harness CONFIRMED"), so structural alignment with the 10 of 12 axes pattern (Position | Systems supporting | Notes) is the principled choice; (b)'s Axis 12 parallel does not apply because Axis 12's framing "v1-derived; not externally validated" specifically justifies its Notes-only structure. One change applied (v1.20): Axis 13 position table restructured from 2-column (Position | Notes) to 3-column (Position | Systems supporting | Notes); Systems supporting column entries: "None" for Thin-harness (parallel to Axis 1 Multi-agent peer "None" convention for non-surveyed positions; v1's-shape captured in separate v1's-position paragraph below the table per other-axis convention; Notes column "(v1's shape)" parenthetical preserved for minimal-change discipline), "openclaw (Gateway/agent split — partial fat-harness instance per cycle-43 deeper read; gateway handles channel connections, queue management, plugin lifecycle, session routing, sandbox enforcement, tool policy in deterministic code)" for Medium-harness, "OpenAI harness writeup (custom linters, CI jobs, doc-gardening agent, ephemeral worktrees, observability stack per worktree)" for Fat-harness; entries match Axis 1's parenthetical-instance-description verbosity convention. Q[b] sweep beyond Axis 13 across openclaw + openai-harness + cognition-devin Phase 2 anchoring sections: 0 additional Position table ↔ surveyed-system file consistency findings — all openclaw anchors (Axis 2 file-per-component; Axis 3 singleton plugin slot; Axis 6 plugins; Axis 9 runtime ceiling + stuck-session watchdog; Axis 12 event-driven Notes citation) are reflected in framework; all openai-harness anchors (Axis 2 Repository-as-state; Axis 3 Repository-as-record; Axis 4 Git-as-substrate; Axis 5 Plans-as-artifacts; Axis 8 Mechanical enforcement; Axis 9 counter-evidence Ralph Wiggum Loop; Axis 10 Entropy mitigation) are reflected; all Cognition Devin anchors (Axis 1 small-fixed-team; Axis 3 context-trace; Axis 9 45-min retired) are reflected. Within-surface enumeration self-consistency NEW sub-shape (cycle-57) applied to other surfaces beyond cross-axis dep map: F-pattern table rationales for Axis-13-mentioning Fs (F1, F6, F7) appropriately enumerate Axis 13 positions per content-driven mediation type — F1 bifurcates within Axis 13 (Axis 8 mechanism at thin-harness; Axis 13 mechanism at fat-harness); F6 names fat-harness specifically (Axis 7 mechanism is Axis-13-agnostic; Axis 13 mechanism is fat-harness-specific); F7 names fat-harness within multi-mechanism enumeration (Specialization + mechanical enforcement + iteration ceilings + fat-harness, four parallel fix mechanisms); the asymmetry between F1's bifurcation and F6/F7's single-position naming is content-driven by whether Axis 13's role is bifurcated-by-position vs position-specific. Maps-to lines self-consistent within Axis 13 (mentions fat-harness across F1/F6 and prompt-encoded thin/medium across F7 across the Maps-to line). 0 additional findings on these surfaces. Sub-shape inventory NOW: back-ref EXHAUSTED; disposition EXHAUSTED; convention DEFERRED to checkpoint; global-completeness EXHAUSTED-WITHIN-CURRENT-SET; mediation symmetry CONFIRMED 0 findings; constraint-axis instantiation distinguished from active-shaping; position table cross-reference symmetry CONFIRMED; within-surface enumeration self-consistency EXHAUSTED-WITHIN-CURRENT-SCOPE-POST-CYCLE-58 (cross-axis dep map fixed cycle-57; F-pattern table + Maps-to lines confirmed 0 findings cycle-58; position table column-structure consistency fixed cycle-58 via Q[b]); position-table-system-anchor consistency NEW SUB-SHAPE applied at cycle-58 via Q[b] (only Axis 13 was outlier across openclaw + openai-harness + cognition Phase 2 anchoring). Cycle-50 first-application hypothesis CONFIRMED for fourth NEW parent lens (cycle-57 Q[b]); cycle-58 SECOND application surfaces 0 new findings beyond cycle-57's deferred Axis 13 finding — supports cycle-50 hypothesis that NEW lens applications surface findings on first-application; subsequent applications converge. Q(c) procedural decision: v1.20 single-cell Axis 13 position table column-restructure is sole bounded-mechanical work this cycle; one coherent fix-decision (add Systems column with surveyed-system anchors per cycle-57 BORDERLINE-FAIL finding) propagated across the table's 3 rows. Single-cell discipline preserved cycle-58 (1 fix applied; 0 deferred). |
| v1.21 | 59 (2026-05-03) | `_notes/cycle-59-cold-reader-and-v1.21-application.md` | Cycle-59 cold-reader on v1.20: Q(a) PASS on v1.20 Axis 13 position table 3-column restructure with surveyed-system anchors (re-walked with fresh adversarial framing on FIVE probes — (i) "in code" qualifier defensibility on global Axis 13 × Axis 8 entry first sentence (cycle-58 probe re-confirmed: still content-driven necessary because surface-type contrast with second sentence requires explicit naming; "more deterministic code to lint and test" parenthetical is elaborative not type-naming), (ii) F1 SEE pointer cross-reference vs Axis 13 × Axis 7's inline F9 mediation pattern (still content-driven distinct: F1's bifurcation IS in F-pattern table; F9's primary-fix-vs-secondary-shape is succinct-enough to inline), (iii) bifurcation enumeration parallelism with Axis 13 × Axis 7 (still parallel three-element structure: fat branch + thin/medium branch + F-pattern reference), (iv) per-axis subsection terser-than-global convention preservation (still terser: subsection × Axis 8 drops F1 SEE pointer + parenthetical elaboration), (v) NEW PROBE Notes column "(v1's shape)" parenthetical wordsmith concern from cycle-58's "what I couldn't figure out" — content-driven analysis of v1-references across all axis tables: 6 of 7 axes with v1-table-row-references use complete-sentence form ("v1's `state.json` is the explicit anti-example" Axis 2; "Closest to v1's intermediate-cache shape" Axis 3; "v1's rigid checklist-driven sequence... v1 anti-pattern" Axis 7; "Implicit in v1's per-cycle non-bounded retry" Axis 9; "Implicit in v1's accretion-as-defense pattern" Axis 10; "v1 anti-pattern" Axis 12); Axis 13 uses parenthetical-tag form ("(v1's shape)") which is the ONLY parenthetical-tag form in the table corpus; FUNCTION is consistent across all 7 axes (locate v1 within position categorization), FORM differs (complete-sentence vs parenthetical-tag); the FORM variation is content-driven by Notes column content type — Axis 13's Notes column is description-form ("Most procedure in prompt; LLM re-derives procedure each cycle") so a parenthetical-tag reads naturally; other axes' Notes use anti-pattern/anti-example-statement form because their Notes column text directly addresses v1's status; the v1's-position separate paragraph below table is consistently present across ALL 12 axes including Axis 13 — within-axis double-mention is the same pattern across all 7 axes that mention v1 in Notes; PASS — no wordsmith change. Ninth Q(a) PASS in v1.X sequence (cycles 50-59 all PASS except cycle-57's first-ever Q(a) verdict-shift on cycle-56's BORDERLINE-CONTENT-DRIVEN). Q(b) FIRST APPLICATION of NEW parent lens (Iteration history row ↔ change consistency) — fifth NEW parent lens application across cycles 50/51/57/57-58/59: structural sub-lens BORDERLINE-FAIL on NEW row-ordering sub-shape — iteration history rows v1.0 through v1.17 are in version-ascending order (chronological per cycle date), but rows v1.18, v1.19, v1.20 are in REVERSE-version order at the bottom of the table (line 35 = v1.20, line 36 = v1.19, line 37 = v1.18). Cross-history check: cycles 55 (v1.18 row added at 05:22:38 UTC), 57 (v1.19 row added at 08:52:46 UTC), 58 (v1.20 row added at 10:38:33 UTC) each prepended their row immediately after v1.17 rather than appending after the previous most-recent version row, producing the reverse-temporal accumulation. The ordering convention shifted implicitly at cycle 55; cycles 57+58 followed the unintentional shifted convention. Decision space: (a) restore version-ascending order at bottom (v1.18 → v1.19 → v1.20 → v1.21) — fix matching the v1.0-v1.17 18-row convention; (b) retain reverse-version order and apply consistently across all rows (move v1.0 to bottom, v1.20 to top) — converts to most-recent-on-top convention. Cycle-59 chose (a) for full structural consistency with the 18-row v1.0-v1.17 convention (vs 3-row v1.18-v1.20 anomaly) and minimal-change discipline (3-row reorder vs 21-row reorder). Q[b] sweep beyond row-ordering on the lens: change description accuracy spot-checked across 4 rows (v1.5 cycle 41, v1.10 cycle 47, v1.13 cycle 50, v1.20 cycle 58) — descriptions match actual changes applied in framework file; cycle numbers correct; date format consistent (`N (YYYY-MM-DD)`); source notes file paths consistent format `_notes/cycle-N-*.md`; 0 additional findings on these dimensions. Cycle-50 first-application hypothesis CONFIRMED for fifth NEW parent lens (≥1 finding on first application; row-ordering sub-shape surfaced); 5 of 5 NEW parent lens first-applications surfaced ≥1 finding (cycles 50/51/57/57-58/59 — robust at 5 of 5 instances). Sub-shape inventory NOW: back-ref EXHAUSTED; disposition EXHAUSTED; convention DEFERRED to checkpoint; global-completeness EXHAUSTED-WITHIN-CURRENT-SET; mediation symmetry CONFIRMED 0 findings; constraint-axis instantiation distinguished from active-shaping; position table cross-reference symmetry CONFIRMED; within-surface enumeration self-consistency EXHAUSTED-WITHIN-CURRENT-SCOPE-POST-CYCLE-58; position-table-system-anchor consistency EXHAUSTED-WITHIN-CURRENT-SET-POST-CYCLE-58; row-ordering NEW SUB-SHAPE applied at cycle-59 via Q[b] (sub-shape inventory NOW spans 9 sub-shapes). Three changes applied (v1.21): (i) row v1.18 (cycle 55) moved from line 37 to line 35 position (post-v1.17 ascending); (ii) row v1.19 (cycle 57) confirmed at line 36 (already correct relative position); (iii) row v1.20 (cycle 58) moved from line 35 to line 37 position (post-v1.19 ascending); plus row v1.21 inserted after row v1.20 (this cycle); plus Status header v1.20 → v1.21. Q(c) procedural decision with explicit retrospective-iteration weighing: v1.21 row reorder + v1.21 insertion + status header bump is single coherent fix-decision (restore version-ascending convention) — single-cell discipline preserved (one fix-decision propagated across 4 row positions + 1 status header line). Retrospective-iteration weighing: cycle-58's directive ("if cycle-59 produces 0 findings, schedule cycle-60 for retrospective iteration with high confidence") DOES NOT TRIGGER cycle-60 retrospective iteration because cycle-59 surfaced 1 STRUCTURAL finding (row-ordering); framework cold-reader cadence continues to surface real findings across 5 NEW parent lens applications (cycles 50/51/57/57-58/59) all surfacing ≥1 finding; retrospective direct iteration deferred to cycle-60+ pending continued surfacing of substantive framework findings. Single-cell discipline preserved cycle-59 (1 fix applied; 0 deferred). |

## Purpose and scope

This framework consolidates Phase 1's 16 cross-system patterns + 3 persistent
divergences + v1's failure-mode catalog into a structured Phase-2-input
artifact. Two top-level structural elements:

- **Convergent constraints.** Patterns where 3+/N surveyed systems converge.
  Every v2 candidate must honor these or explicitly disagree with load-bearing
  rationale. A candidate that violates a convergent constraint is a candidate
  that disagrees with all surveyed systems' converged practice — that
  disagreement should be deliberate, not accidental.
- **Real design axes.** Patterns where surveyed systems diverge. Each axis is
  a meaningful candidate-differentiation point. Each candidate must declare
  its position; multiple positions are defensible.

Plus four supporting structural elements:

- **Cross-axis dependency map.** Significant inter-axis constraints and
  near-orthogonality observations.
- **Mapping to v1 failure modes.** Which axes a candidate must address well
  to structurally fix each F-pattern from the retrospective.
- **Preserved-primitives interactions.** How v1's preserved primitives (per
  redesign prompt SECTION 3) constrain candidate axis positions.
- **Phase 2 candidate template (preliminary).** Suggested structure for
  candidate documents.

## Note on ordering

**Axis numbering is for reference only; no significance, priority, or
load-bearingness ranking is implied.** Candidates may address axes in any
order, prioritizing what is load-bearing for their specific design. The
numbering reflects the chronological order of axis identification in the
v1.0→v1.2 iteration; gaps in numbering (Axis 11 absent) reflect demotion or
removal during iteration and are deliberate provenance markers, not errors.

## Convergent constraints (every v2 candidate must honor)

Eight constraints. A candidate that violates one of these is disagreeing with
all surveyed systems' converged practice; the disagreement should be explicit
and load-bearing.

1. **Code-vs-prompt split exists.** Deterministic code executes; LLM proposes.
   (Family B pattern 1, 3+/6 systems, foregrounded convergence with substrate
   variations.) v1 honors this in shape (Rust tools + LLM orchestrator) but
   the prompt encodes procedure that should be in tools (CORE-DESIGN-
   PRINCIPLE violation).

2. **Failed work is recorded as artifact, not silently discarded.**
   (Family C pattern 2, 3+/3 systems with structural similarity.) v1 has
   journal failure notes but no failure-record file with read-after-failure
   semantics.

3. **Strong-defaults security with operator-controlled knobs.**
   (Family D, 3+/3 + scope condition.) v1's GitHub Actions secret-handling
   and the prompt's UNTRUSTED-TEXT-RULES already honor this; v2 should
   preserve.

4. **Per-agent model selection treated as primitive.** (Family A pattern 2,
   3+/3 + diversity hedge.) v1 uses a single model (Claude Opus) for the
   orchestrator; Copilot dispatch model is per-dispatch. v2 candidates
   should treat model selection as per-component, with per-component
   rationale.

5. **Anti-patterns documented explicitly as deliverable artifact.**
   (Family E pattern 1, 3+/6 systems.) v1's retrospective is the
   anti-pattern catalog; v2 prompt should preserve and extend
   (e.g., "what we will not do" sections per the openclaw VISION.md
   pattern).

6. **Small core, capability extends via something.** (Family B pattern 2,
   3+/7 systems.) The "something" is a real choice (see Axis 6 below),
   but the principle (lean entry point + extension mechanism) is
   convergent.

7. **Memory is treated as architectural elevation, not derivative of state.**
   (Family C pattern 4, 5/5 surveyed systems where memory is named elevate
   memory architecturally; 0 surveyed systems treat memory as derivative-
   of-state.) The shape of the memory subsystem is a choice (see Axis 3);
   the convergent practice — memory has first-class architectural treatment
   — is the constraint. *(Voyager's SkillManager+Chroma is adjacent; framed
   as skill-storage by the source repo, not counted in the elevation
   evidence.)*

8. **Goal-driven over operator-driven (top-level posture).** (Family A
   pattern 3, persistent divergence in surveyed systems; v2 candidates
   committed to goal-driven by mission.) The redesign's primary thesis
   (autonomous self-healing AI pursuing schema-domain work) commits to
   goal-driven as the top-level posture. Operator-driven sub-systems
   may exist within a goal-driven overall posture (e.g., Eva-issued
   `input-from-eva` directives as explicit operator-commands, integrated
   via Axis 12 reconciliation), but the top-level operator-vs-goal choice
   is fixed by mission. Promoted from former Axis 11 (cycle 37); a
   non-differentiating axis is a constraint.

## Real design axes (v2 candidates differ on)

Twelve axes (numbered 1-10, 12, 13; Axis 11 absent — promoted to constraint
8 in v1.2). Each axis is a meaningful candidate-differentiation point.

### Axis 1 — Agent decomposition

**The choice:** how is the orchestrator session decomposed into agents/roles?

| Position | Systems supporting | Notes |
|---|---|---|
| Single-threaded linear | Cognition Devin June 2025 ("Don't Build Multi-Agents") | Position substantially walked back April 22, 2026; durable invariant is **writes-stay-single-threaded**, not single-threaded execution |
| Small fixed team with role-separation | Voyager (4 agents), AutoGen Magentic-One (lead + workers), oh-my-codex (30 named role prompts), Cognition Devin Apr 2026 (Managed Devins coordinator + parallel children + clean-context Devin Review + Smart Friend consultation) | 4+/4 with writes-single-threaded as constraint, not multi-agent prohibition |
| Multi-agent peer (uncontrolled) | None | Rejected by 4+/6 systems as default; Cognition explicitly rejects "unstructured swarms" in April 2026 follow-up |

**v1's position:** single-threaded with Copilot dispatches as parallel workers
(off-process). The dispatches are not "agents" in the small-fixed-team sense
— they're per-task externally-delegated work.

**v2 candidate space:** retain dispatch-as-worker (current shape) vs adopt
small-fixed-team within the orchestrator session itself (e.g., planner /
executor / critic / curator). The convergent constraint across all 4 systems
that ship multi-agent designs is **writes stay single-threaded** — the
load-bearing invariant is write-discipline, not agent-count. Candidates
that allow multi-agent decomposition must declare how writes stay
single-threaded; candidates that adopt single-threaded execution can cite
the broader invariant rather than agent-count specifically.

**Cross-axis dependency:** Axis 1 × Axis 3 (memory) — small-fixed-team can
have per-agent memory subsystems; Axis 1 × Axis 7 (orchestration topology) —
single-threaded forces single-topology; small-fixed-team enables but doesn't
force multi-topology coexistence; Axis 1 × Axis 12 (reconciliation
discipline) — small-fixed-team enables a dedicated reconciliation agent;
single-threaded must interleave reconciliation with primary work.

**Maps to:** F7 (self-management dominance — role-specialization, including
a dedicated reviewer / curator / reconciler agent, reduces self-management
surface for the primary agent). Indirect contributor to F9 (adversarial-
review treadmill) via dedicated-reviewer-role.

### Axis 2 — State representation primitive

**The choice:** what is the unit of persistent state?

| Position | Systems supporting | Notes |
|---|---|---|
| Single global state file | None | v1's `state.json` is the explicit anti-example; 3+/5 systems agree |
| File-per-component | AutoGen, Voyager (`ckpt/<agent>/`), oh-my-codex (`.omx/state/<mode>-state.json`), openclaw (`~/.openclaw/agents/<agentId>/` per-agent state isolation; Gateway-level globals exist per `src/global-state.ts`, contents not verified) | 4+/5 + diversity hedge |
| Typed-channel-map within one schema | LangGraph | Persistent divergence — one pole |
| Repository-as-state | OpenAI harness | git substrate; ephemeral worktrees |

*Plans-as-artifacts is a separate temporal/lifecycle dimension; see Axis 5.*

**v1's position:** monolithic `state.json` (42 keys, 62-69% defense-character
per F12 catalog). 4-6× reduction estimated for v2 per cycle-5 measurement.

**v2 candidate space:** every position EXCEPT "single global state file" is
defensible. The choice between file-per-component and typed-channel-map is
the persistent State-shape divergence (Family C); a candidate must commit
to one or explicitly span both. The repository-as-state position has
interesting properties for a public-repo orchestrator (commits ARE state)
but conflicts with the journal/notes-file conventions if state mutations
land in journal entries vs separate state files.

**Cross-axis dependency:** Axis 2 × Axis 3 (memory) — file-per-component
naturally supports memory-as-component-file; typed-channel-map naturally
supports memory-as-channel; repo-as-state supports memory-as-files-in-repo;
Axis 2 × Axis 4 (history substrate) — file-per-component pairs naturally
with one-way migration or git; typed-channel-map pairs with branching
checkpoints; repo-as-state forces git-as-substrate.

**Maps to:** F12 (state accretion), F5 (state.json as procedural-leak), F3
(multi-candidate state drift). Indirect contributor to F11 (post-close
mutations) — file-per-component naturally supports per-component append,
making Axis 4's append-only easier; the load-bearing F11 fix is Axis 4 +
Axis 12.

### Axis 3 — Memory subsystem shape

**The choice:** if memory is first-class (per convergent constraint 7),
what shape does it take?

| Position | Systems supporting | Notes |
|---|---|---|
| Singleton plugin slot (one mechanism active, replaceable) | openclaw | Persistent divergence — one pole. Cycle-43 deeper read: the singleton-slot scope is the storage/retrieval LAYER (`plugins.slots.memory`); the full memory architecture is layered on top — Markdown files (`MEMORY.md` + daily notes + `DREAMS.md`) + SQLite index + embedding-based hybrid search + active-memory sub-agent + dreaming background consolidation. |
| Top-level architectural principle | PAI Principle 13 | Persistent divergence — other pole |
| Context trace (everything-the-agent-has-done) | Cognition Devin (primary in-session mechanism; multi-layer at longer horizons) | Cycle-41 deeper read documents 5+ memory mechanisms (cross-session notes, Knowledge API, Playbooks, DeepWiki, Session Insights, hypervisor snapshots); context-trace is the in-session label |
| Repository-as-record | OpenAI harness | "Anything not in-context doesn't exist" |
| Wiki + search (markdown-first, search-first) | oh-my-codex (`.omx/wiki/` + MCP server) | Bounded context injection |
| Typed channels (short/long-term distinction) | LangGraph (Store + checkpointer) | Closest to v1's intermediate-cache shape |
| Memory derivative of state (no first-class) | None | Rejected by 3+/5 |

**v1's position:** memory is mostly derivative of state.json (no first-class
memory). The journal acts as long-term reflective memory; the worklog is
short-term transient. No semantic memory primitive.

**v2 candidate space:** every position except "memory derivative of state"
is defensible. The choice has significant downstream consequences for what
an orchestrator session can recall across cold-starts. Persistence-mechanism
evolution (cycle-2 to cycle-33+) has been organic-markdown-files; that maps
closest to wiki+search but without the search infrastructure.

**Cross-axis dependency:** Axis 3 × Axis 2 (state) — memory shape follows
state representation; Axis 3 × Axis 1 (decomposition) — small-fixed-team
can have per-agent memory subsystems.

**Maps to:** Convergent constraint 7 (memory architectural elevation —
Axis 3 makes the specific shape choice within the constraint). Indirect
contributor to F7 (self-management dominance) via cold-start cost — rich
memory reduces re-derivation each cycle, freeing compute for primary work.
Axis 3's load-bearing role is constraint-7-shape rather than direct
F-pattern fix.

### Axis 4 — History/Provenance substrate

**The choice:** where does append-only history live?

| Position | Systems supporting | Notes |
|---|---|---|
| Branching checkpoints (in-process versioning) | LangGraph time-travel | "`update_state` does not roll back; creates new checkpoint that branches" |
| Versioned files (`<name>V2.js`, `V3.js`) | Voyager skill versioning | Filesystem versioning |
| Git-as-substrate | OpenAI harness | Repository as state; commits append; ephemeral worktrees |
| One-way file migration with read-only legacy | oh-my-codex | Schema migrations one-way, not destructive |

**v1's position:** journal + worklog + commits are append-only-by-git but
`state.json` is destructive (writes overwrite). The draft-then-promote /
append-only retention pattern (Eva advisory #2408) is the targeted v2 fix.

**v2 candidate space:** all four positions are defensible. Git-as-substrate
has natural alignment for a public-repo orchestrator (every commit is the
audit trail). Branching checkpoints have appeal for the "what-if" reasoning
multi-cycle work needs but aren't trivially mappable to a flat-file-on-disk
substrate. The one-way migration shape is the conservative choice for
evolving the persistence mechanism itself across cycles.

**Constraint from preserved-primitives:** branching positions must be in-tree
files (per-branch-named files committed in main), not git-branches that
might not be pushed (per git-safety primitive — every commit must be
pushed).

**Maps to:** F11 (post-close mutations) — append-only with branching
prevents the destructive-write semantics that lose post-close mutations
from history. F12 (state accretion via non-destructive write semantics).
F4 (frozen-artifact lifecycle fragility) — substrate determines what
"frozen" means.

### Axis 5 — Plans/specs as forward artifacts

**The choice:** are plans/specs first-class versioned artifacts written
before execution, or reconstructed-after?

| Position | Systems supporting | Notes |
|---|---|---|
| Yes — plans-as-artifacts (active/completed/technical-debt) | OpenAI harness | Plan files checked into repo; per-category lifecycle |
| Yes — context snapshots before execution | oh-my-codex | `.omx/context/{task-slug}-{timestamp}.md` with explicit fields |
| No — plans live in-message or are reconstructed from history | Default in absence of plan-artifact infrastructure | Most surveyed systems by default; none explicitly supports reconstruction-after as a primitive |

**Status:** 2-system clean convergence. Lower convergence than other patterns;
treat as candidate-considered axis, not constraint.

**v1's position:** plans live in cycle issue comments + journal entries +
occasional `_notes/` files. No structured plan-artifact lifecycle. The
redesign has implicit cycle-N→cycle-N+1 plan suggestions in notes files
but no separate plan-files-on-disk.

**v2 candidate space:** adopting plans-as-artifacts forces a filesystem
layout decision (`plans/active/`, `plans/completed/`, `plans/technical-debt/`)
and a transition lifecycle. Skipping this axis means relying on journal +
notes for the same purpose.

**Maps to:** F4 (frozen-artifact lifecycle fragility) — plan lifecycle
primitives (`active/completed/technical-debt`) address freeze/refresh
timing as a structural design choice rather than ad-hoc per-artifact
handling.

### Axis 6 — Extension shape

**The choice:** if small-core extends via something, what?

| Position | Systems supporting | Notes |
|---|---|---|
| Plugins | openclaw | "Core stays lean; optional capability ships as plugins" |
| Skills | PAI, oh-my-codex (39 skills) | Skill = code + prompt + invocation contract |
| Tools | LangGraph (`ToolNode`), AutoGen (model-emits-tool-call) | LLM-discoverable invocation primitives |
| Layers | PAI 16 principles, AutoGen Core/AgentChat/Extensions/Studio/Bench | Architectural-layer composition |
| Harness-accumulation (depth-first) | OpenAI harness | Capabilities added iteratively as failures surface |
| Configuration-layer-with-hooks | oh-my-codex (on top of unmodified Codex CLI) | Wrap-without-replace |

**v1's position:** Rust binaries in `tools/` directory with shell-wrapper
scripts. No formal "skill" or "plugin" abstraction; tools are discovered by
file-existence + naming convention.

**v2 candidate space:** retaining Rust-tools-as-extension shape is the path
of least migration cost, with the question being whether to add a discovery/
registration primitive (skill / plugin manifest) on top.

**Maps to:** Convergent constraint 6 (small core, capability extends via
something — Axis 6 makes the specific extension-mechanism choice). Axis 6's
load-bearing role is constraint-6-shape rather than direct F-pattern fix.
Folds polyglot / multi-language schema strategy as schema-domain extension
choice (Phase 3 prototype's load-bearing test, not a v2 prompt-level axis).

**Considered-and-folded:** polyglot / multi-language schema strategy is
schema-domain-specific. Phase 3 prototype includes one polyglot end-to-end
test. The polyglot strategy is part of Phase 3 design, not the v2
prompt-level axes — language-port tools are extensions and fold into Axis 6.

### Axis 7 — Orchestration topology

**The choice:** how do agents/components coordinate?

| Position | Systems supporting | Notes |
|---|---|---|
| Single-pattern (one shape only) | None in surveyed systems' current shipping architectures | Cognition June 2025 advocated this in "Don't Build Multi-Agents"; April 2026 walkback ships multi-pattern. v1's rigid checklist-driven sequence is the closest extant example — and is the v1 anti-pattern. |
| Multi-pattern coexisting | AutoGen (round-robin/selector/swarm/graph), LangGraph (chaining/routing/parallelization/orchestrator-worker/ReAct/subgraphs/supervisor), Cognition Apr 2026 (Managed Devins coordinator + parallel children, Devin Review clean-context, Smart Friend frontier consultation) | 3+/3 in surveyed |
| Sequential mode transitions with deterministic transition policy | oh-my-codex (`STATE_MODEL.md` allowlist) | Modes governed by allowlist preventing illegal shifts |
| Lead-worker hierarchy | AutoGen Magentic-One, Cognition Apr 2026 (Managed Devins) | Specialized workers under orchestrator |
| Peer-flow | Voyager (curriculum → action → critic → skill) | Round-robin among role-specialized peers |

**v1's position:** rigid checklist-driven sequence (STARTUP → C phases →
COMPLETION). One topology, encoded in two checklist files.

**v2 candidate space:** the rigid-checklist position has been explicitly
named as Phase 2 anti-pattern (per CORE-DESIGN-PRINCIPLE). Multi-pattern
coexisting with deterministic transition policy (oh-my-codex shape) is the
strongest match for "orchestrator handles novel situations" because
different situations may call for different topologies.

**Cross-axis dependency:** Axis 7 × Axis 1 (decomposition) — see Axis 1.
Axis 7 × Axis 13 (harness-vs-session boundary) — fat-harness can implement
situational-review by controlling when review fires, supporting Axis 7's
multi-pattern situational invocation; thin/medium harness leaves WHEN-
review decisions in prompt.

**Maps to:** F6 (cyclomatic procedure depth — multi-pattern with transition
policy lighter than rigid checklist) and F9 (adversarial-review treadmill —
situational invocation breaks the every-cycle review-firing loop).

**Considered-and-folded:** Eva-checkpoint mechanism specifics ("what
triggers a checkpoint") fold into Axis 7 — the topology determines what
state transitions are checkpoint-eligible. The companion question ("how
does the orchestrator know it's at a checkpoint") folds into Axis 12
(Reconciliation discipline) — checkpoint-detection is an inbound-channel
question.

### Axis 8 — Mechanical enforcement scope

**The choice:** what is regression-tested mechanically?

| Position | Systems supporting | Notes |
|---|---|---|
| None | Default in absence of explicit infrastructure | Rare in surveyed |
| Data-shape only | LangGraph (TypedDict / dataclass / Pydantic), Voyager (init-time `count == len(skills)`) | Diversity hedge — adjacent to behavioral enforcement |
| Behavioral promises + agent-affecting prose | OpenAI (custom linters with agent-readable error messages), oh-my-codex (prompt-contract regression tests on `prompts/`) | 2-system strict |

**v1's position:** Rust tools have unit tests; pipeline-check has sub-checks.
No regression tests on the orchestrator prompt or checklist text — agent-
affecting prose can change without CI catching.

**v2 candidate space:** adopting behavioral-prose CI is a high-leverage v2
move because it directly addresses F1 (constraint-without-tool ratio): a
constraint added to the prompt without a paired tool fix would surface as
a CI test the orchestrator must honor mechanically, OR be rejected pre-
merge.

**Maps to:** F1 (constraint accretion), F5 (state.json as procedural-leak),
F7 (self-management dominance — mechanical enforcement reduces orchestrator
constraint-tracking burden), CORE-DESIGN-PRINCIPLE violation detection.

### Axis 9 — Iteration ceilings

**The choice:** are autonomous loops bounded?

| Position | Systems supporting | Notes |
|---|---|---|
| None (open-ended runs) | Rare in surveyed | Implicit in v1's per-cycle non-bounded retry |
| Loop count ceilings | oh-my-codex (`max_iterations=10`, `max=5`), Voyager (`action_agent_task_max_retries=4`) | 2-system strict |
| Runtime ceiling | ~~Cognition Devin (45-min session limit, *documented-claim*)~~ — **unverified after cycle-41 direct primary-source access**; docs say "if you can do it in three hours, Devin can most likely do it"; hypervisor snapshot infrastructure supports hours-long sessions. openclaw (`agents.defaults.timeoutSeconds` default 172800s = 48h, effectively-unbounded for typical use; the **stuck-session watchdog** `diagnostics.stuckSessionWarnMs` is a more interesting primitive — detects stale lanes and can release them) | Anchor weakened on Cognition; OpenAI Ralph Wiggum Loop is **counter-evidence** (no iteration ceiling, human backstop is the bound — does NOT transfer to cron-driven autonomous systems); openclaw's stuck-session watchdog is the transfer-relevant primitive |
| Both (loop + runtime) | None explicitly in surveyed | Composable |

**v1's position:** per-cycle there is no per-loop ceiling. The cycle ITSELF
is the only ceiling (~75 minutes of compute). Pipeline-check sub-checks
can re-fire, dispatch can retry, etc., without a bounded loop count.

**v2 candidate space:** loop-count ceilings are bounded-mechanical to add
and immediately reduce the failure surface for runaway-autonomy. Runtime
ceiling is a coarser ceiling (cycle-level already has it).

**Maps to:** F8 (abandonment cascades), F7 (self-management dominance via
unbounded re-firing).

### Axis 10 — Entropy / AI-slop mitigation

**The choice:** is output-quality drift addressed as recurring infrastructure?

| Position | Systems supporting | Notes |
|---|---|---|
| Not addressed | Default | Implicit in v1's accretion-as-defense pattern (F12) |
| Golden principles + doc-gardening agent | OpenAI harness | Recurring agent-quality cleanup |
| Mandatory deslop pass post-completion | oh-my-codex | Quality cleanup embedded in task completion contract |
| Both | None explicitly in surveyed | Composable |

**Status:** 2-system clean convergence. Lower convergence than other patterns;
treat as candidate-considered axis, not constraint. Inversely-related to
v1's accretion-as-defense pattern (F12) — these systems treat accretion as
a failure mode to clean, not a defensive structure to preserve.

**v1's position:** no entropy-mitigation primitive. F12's defense-accretion
pattern is the explicit anti-direction.

**v2 candidate space:** adopting an entropy-mitigation primitive is high-
leverage if F12 is to be addressed structurally rather than via "defenses
re-examined for load-bearingness" (per the retrospective's Defense-
accretion implication).

**Maps to:** F12 (defense accretion).

**Considered-and-folded:** failure-mode catalog maintenance ("does v2
update its own anti-patterns catalog?") folds into convergent constraint 5
(Anti-patterns documented as deliverable artifact) and Axis 10 (the "how
is it kept current" mechanism).

### Axis 11 — *(absent — promoted to convergent constraint 8 in v1.2)*

Former Axis 11 was Operator-vs-Goal framing. Cycle 37's iteration determined
that a non-differentiating axis (every v2 candidate must take the same
position by mission commitment) is a constraint, not an axis. See
convergent constraint 8.

The numbering gap is a deliberate provenance marker. v2 candidates may
still reference "operator-driven sub-system" choices (per constraint 8's
note), but the top-level posture is fixed.

### Axis 12 — Reconciliation discipline

*(v1-derived; not externally validated by surveyed Phase 1 systems)*

**The choice:** how does the system reconcile inbound external events
(Eva responses, audit posts, dispatch outputs, post-close tool mutations)
into state?

| Position | Notes |
|---|---|
| No reconciliation: write-only outbound channels | v1 anti-pattern (F2/F3/F4/F11 emerge from this) |
| Active polling: each outbound channel paired with a reader producing state transitions | Uniform mechanism (one pattern per channel); per-channel implementation overhead |
| Event-driven: state changes reactively when external events arrive | Reactive handling; shared inbound infrastructure (one webhook or GitHub Actions trigger handles all subscribed channels — for a public-repo orchestrator the Actions platform is already-paid; per-event handler configuration is a bounded one-time cost); openclaw's Gateway is an instance — channels maintain persistent upstream connections, agent runs are per-event discrete turns |
| Hybrid: polling for low-frequency channels, event-driven for high-frequency | Mixed mechanism; design overhead spread per-channel-class rather than per-channel; suited to workloads where different channels have different natural frequencies |

**Status:** v1-derived axis; no external system surveyed has an Eva-equivalent
that would constrain the choice. Candidates that address Axis 12 are doing
more design work than those addressing externally-validated axes; candidates
may also choose to fold this into existing axes (e.g., Axis 4 history
substrate where event-driven means "git events trigger state recompute")
rather than treating as separate.

**Note: HITL primitives are not reconciliation analogues** *(verified
cycle 39, retired cycle-38's "v1-derived caveat may be too strong" flag)*.
LangGraph interrupts and AutoGen HITL primitives are synchronous pause-resume
mechanisms — the caller is the active sender of `Command(resume=...)` (or
equivalent), the graph/agent is the passive receiver waiting at a specific
node. Axis 12's reconciliation concerns asynchronous absorption of external
events that arrive independently of the orchestrator's execution thread
(Eva responds when she responds; audit posts when audit posts; PR merges
when reviewers merge). The orchestrator cannot pause-and-wait — it runs on
a cron and must catch up to whatever happened since last cycle. Different
structural shape; HITL is not a reconciliation analogue. AutoGen explicitly
disclaims "global reconciliation of all component states" (per-system file).

**v1's position:** no reconciliation. Outbound channels (issue creates, PR
creates, journal commits) are well-developed; inbound reconciliation does
not exist. The retrospective documents F2/F3/F4 as direct manifestations.

**v2 candidate space:** every position EXCEPT "no reconciliation" is
defensible. Hybrid is the path of least design-cost since different channels
naturally have different polling frequencies.

**Cross-axis dependency:** Axis 12 × Axis 4 (history substrate) — event-
driven reconciliation pairs naturally with git-as-substrate (commits as
events); Axis 12 × Axis 1 (decomposition) — small-fixed-team can have a
dedicated reconciliation agent.

**Maps to:** F2 (Eva-response detection), F3 (multi-candidate state drift,
post-close aspect — close-out doesn't reconcile against post-close evidence;
F3 row's other aspect is Axis 2's single-source-of-truth), F4
(frozen-artifact lifecycle fragility — worklog freeze without refresh),
F11 (post-close mutations — worklog never reads state back).

**Considered-and-folded:** audit-repo integration mechanism is part of
Axis 12 — audit-orchestrator posts are an inbound channel requiring
reconciliation.

### Axis 13 — Harness-vs-session boundary

*(cross-cutting CORE-DESIGN-PRINCIPLE elaboration)*

**The choice:** where is the line between deterministic harness code and
LLM session?

| Position | Systems supporting | Notes |
|---|---|---|
| Thin harness, fat session | None | Most procedure in prompt; LLM re-derives procedure each cycle (v1's shape) |
| Medium harness, medium session | openclaw (Gateway/agent split — partial fat-harness instance per cycle-43 deeper read; gateway handles channel connections, queue management, plugin lifecycle, session routing, sandbox enforcement, tool policy in deterministic code) | Split between cycle-runner and prompt; harness handles known patterns, prompt handles novel |
| Fat harness, thin session | OpenAI harness writeup (custom linters, CI jobs, doc-gardening agent, ephemeral worktrees, observability stack per worktree) | Most procedure in deterministic code; prompt is small reference + judgment-call decisions |

**Status:** cross-cutting CORE-DESIGN-PRINCIPLE elaboration. Every v2
candidate must declare its position; the principle requires "tools and
deterministic processes handle repetitive, rote, procedural work" — implying
the harness-vs-session line should be drawn farther toward fat-harness than
v1's shape.

**v1's position:** thin harness (cycle-runner mostly invokes the session),
fat session (prompt + 2 checklists encode the procedure the orchestrator
follows each cycle).

**v2 candidate space:** medium-or-fat harness positions are the CORE-DESIGN-
PRINCIPLE-aligned choices. Thin harness is the v1 anti-pattern. The choice
between medium and fat depends on what procedures get extracted into tools
— a candidate must specify the tool surface implied (per the Phase 2
candidate template's "Tool surface implied" section).

**Constraint from preserved-primitives:** Axis 13 positions must specify
the cycle-runner change scope (none / modest / substantial) — cycle-runner
is preserved as the harness entrypoint, and Axis 13 positions imply
different changes to it.

**Cross-axis dependency:** Axis 13 × Axis 6 (extension shape) — the
extension primitive (plugins/skills/tools/etc.) shapes how harness
procedures get organized; Axis 13 × Axis 7 (orchestration topology) —
fat-harness can implement Axis 7's multi-pattern situational-review by
controlling when review fires (vs thin/medium harness leaving WHEN-
review decisions in prompt); Axis 13 × Axis 8 (mechanical enforcement) —
fat harness implies more mechanical-enforcement surface area in code
(vs thin/medium harness having prompt as primary surface; Axis 8's
behavioral-prose CI on prompt contracts addresses the thin-harness
surface).

**Maps to:** F1 (constraint accretion in prompt — fat harness extracts
procedural constraints), F6 (cyclomatic procedure depth — fat harness
extracts procedure), F7 (self-management dominance via prompt-encoded
procedure), CORE-DESIGN-PRINCIPLE explicitly. Indirect contributor to
F9 (adversarial-review treadmill) — fat-harness shapes the
implementation strategy for Axis 7's situational-review by controlling
when review fires; thin/medium harness leaves WHEN-review decisions in
prompt where the every-cycle-review pattern tends to recur; the load-
bearing F9 fix is Axis 7.

**Considered-and-folded:** prompt size budget (how long is the prompt?)
isn't a candidate-differentiation axis per se; it's an outcome of Axis 13's
position. Smaller prompts fall out of fat-harness candidates. Cold-start
ergonomics (how much does a cold-start session need to read before being
productive?) is workflow detail that shapes Axis 13's specific extraction
choices but doesn't differentiate at architecture level.

## Cross-axis dependency map

Significant inter-axis constraints:

- **Axis 1 (decomposition) × Axis 7 (orchestration topology):** Single-
  threaded forces single-topology. Small-fixed-team enables but doesn't
  force multi-topology coexistence.
- **Axis 2 (state) × Axis 3 (memory):** State representation shapes
  which Axis 3 positions are natural — file-per-component aligns with
  filesystem-based memory positions (singleton plugin slot WITH
  filesystem storage as in openclaw's `~/.openclaw/agents/<agentId>/`;
  top-level architectural principle with filesystem memory as in PAI;
  wiki+search with file-per-entry as in oh-my-codex's `.omx/wiki/`);
  typed-channel-map aligns with typed channels with checkpointer
  (LangGraph); repo-as-state aligns with repository-as-record (OpenAI
  harness). The natural-alignment framing is supportive rather than
  exclusive: file-per-component does not preclude context-trace memory
  or other non-filesystem Axis 3 positions, but pairs more naturally
  with the listed filesystem-based positions.
- **Axis 3 (memory) × Axis 1 (decomposition):** Small-fixed-team can have
  per-agent memory subsystems (openclaw's per-agent state isolation in
  `~/.openclaw/agents/<agentId>/` is one surveyed instance).
- **Axis 4 (history substrate) × Axis 2 (state):** State representation
  choice constrains history substrate options — file-per-component pairs
  naturally with one-way migration or git; typed-channel-map pairs with
  branching checkpoints; repo-as-state forces git-as-substrate (state and
  history collapse into the same git primitive — the tree is state, the
  chain of commits is history). *Indirect F11 contribution: file-per-component
  Axis 2 makes per-component append (Axis 4) easier to implement; the
  load-bearing F11 fix remains Axis 4 (append semantics) + Axis 12
  (reconciliation), with Axis 2 as enabling infrastructure.*
- **Axis 8 (mechanical enforcement) × Axis 5 (plans-as-artifacts) × Axis
  10 (entropy mitigation):** Mechanical enforcement is the substrate
  enabling both plan-lifecycle CI checks and golden-principles enforcement.
  Adopting Axis 8 unlocks the others.
- **Axis 12 (reconciliation) × Axis 4 (history substrate):** Event-driven
  reconciliation pairs naturally with git-as-substrate (commits as events;
  webhook on push triggers state recompute).
- **Axis 12 (reconciliation) × Axis 1 (decomposition):** Small-fixed-team
  can have a dedicated reconciliation agent (the "curator" or "reconciler"
  role); single-threaded must interleave reconciliation work with primary
  work.
- **Axis 13 (harness-vs-session) × Axis 6 (extension shape):** The extension
  primitive (plugins/skills/tools/etc.) shapes how harness procedures get
  organized; fat-harness needs a richer extension story.
- **Axis 13 (harness-vs-session) × Axis 8 (mechanical enforcement):** Fat
  harness implies more mechanical-enforcement surface area in code (more
  deterministic code to lint and test). Thin/medium harness has prompt as
  the primary mechanical-enforcement surface area; Axis 8's behavioral-
  prose CI on prompt contracts addresses the thin-harness surface (see
  F1 mapping for the bifurcation across Axis 13 positions).
- **Axis 13 (harness-vs-session) × Axis 7 (orchestration topology):** Fat-
  harness can implement Axis 7's multi-pattern situational-review by
  controlling when review fires (vs every cycle). Thin/medium harness leaves
  WHEN-review decisions in prompt, where the v1 anti-pattern (every-cycle
  review-firing) tends to recur. F9 (adversarial-review treadmill) is
  primarily fixed by Axis 7 (situational vs fixed); Axis 13 shapes the
  implementation strategy for that fix.
- **Constraint 8 (goal-driven) × Axis 1 (decomposition):** Goal-driven
  pairs naturally with single-threaded long-running execution; goal-
  driven within small-fixed-team requires explicit goal-coordination
  primitive (Cognition's Managed Devins coordinator pattern is one
  surveyed instance — coordinator scopes child tasks to maintain
  goal-coherence across parallel children).

Largely orthogonal:

- **Axis 4 (history) × Axis 6 (extension shape)** — independent.
- **Axis 9 (iteration ceilings) × any other axis** — additive primitive.
- **Axis 10 (entropy mitigation) × Axis 1 (decomposition)** — entropy
  mitigation can be implemented at any decomposition.

## Mapping to v1 failure modes

Axis-to-Fpattern mapping. The retrospective's "v2 design implications by
family" section provides high-level guidance; this mapping is more axis-
specific. Updated in v1.2 with Axis 12 + Axis 13 mappings; F11 corrected
per cycle-37 cold-reader.

| F-pattern | Family | Most-relevant axes | Rationale |
|---|---|---|---|
| F1 (constraint accretion) | Defense accretion | Axis 8, Axis 13 | Mechanical CI on prompt contracts forces constraint-as-test or rejection; fat-harness extracts procedural constraints from prompt to tools |
| F2 (Eva-response detection) | Reconciliation | Axis 12 | Direct match — Eva-response polling/event-detection is the reconciliation primitive |
| F3 (multi-candidate state drift) | Reconciliation | Axis 2, Axis 12 | Single source of truth per concern (Axis 2) + reconciliation against post-close evidence (Axis 12) |
| F4 (frozen-artifact lifecycle fragility) | Reconciliation | Axis 4, Axis 5, Axis 12 | History substrate determines what "frozen" means; lifecycle primitives address freeze/refresh timing; reconciliation refreshes frozen artifacts |
| F5 (state.json as procedural-leak) | Defense + Reconciliation | Axis 2, Axis 8 | File-per-component or typed-channel separates concerns; mechanical CI catches procedural-leak patterns |
| F6 (cyclomatic procedure depth) | Procedure overhead | Axis 7, Axis 13 | Multi-pattern with transition policy lighter than rigid checklist; fat-harness extracts procedure from prompt |
| F7 (self-management dominance) | Procedure overhead | Axis 1, Axis 8, Axis 9, Axis 13 | Specialization + mechanical enforcement + iteration ceilings + fat-harness reduce self-management surface |
| F8 (abandonment cascades) | Tooling fragility | Axis 9, CORE-DESIGN-PRINCIPLE | Bounded loops (loop-count ceiling positions; prevention), stuck-session watchdog (runtime-ceiling positions; detection-and-recovery; openclaw's `diagnostics.stuckSessionWarnMs` instance — detect stale runs and release lanes), or both compositionally (Axis 9's `Both (loop + runtime)` position) + single-implementation discipline (no parallel implementations) |
| F9 (adversarial-review treadmill) | Procedure overhead | Axis 7 | Multi-pattern shape replaces fixed adversarial-review step with situational invocation |
| F10 (audit's value is broader read scope) | Design-implication | Not a v2 axis | Audit-side concern; audit-as-peer pattern preserved per redesign prompt SECTION 2 |
| F11 (post-close mutations) | Defense + Reconciliation | Axis 4, Axis 12 | Append-only history (Axis 4) prevents destructive write semantics that lose post-close mutations; reconciliation discipline (Axis 12) refreshes frozen worklog against post-close state. *(Axis 2 indirect contributor — see cross-axis deps; not load-bearing for direct F11 fix.)* |
| F12 (defense accretion catalog) | Defense | Axis 2, Axis 4, Axis 10 | All three contribute; Axis 10 is the structural anti-accretion primitive |

**Observation 1 (post-v1.2):** With Axis 12 added, F2/F3/F4/F11's
reconciliation-asymmetry family is now structurally addressable. v1's
write-only outbound pattern is the named anti-pattern; every v2 candidate
must declare a non-"no reconciliation" Axis 12 position.

**Observation 2:** Multiple Fs map to the same axes (Axis 2, 4, 8, 12, 13
each address 3+ failure modes). This isn't a problem — it's evidence those
axes are high-leverage. A v2 candidate that picks well on Axes 2, 4, 8, 12,
13 addresses ~9 of the 11 failure-modes structurally.

**Observation 3:** CORE-DESIGN-PRINCIPLE (tools handle rote; orchestrator
handles judgment) shows up across F1, F6, F7, F8 — it is itself an
axis-cross-cutting constraint. Axis 13 makes the specific candidate-
differentiation choice along the CDP direction explicit; CDP itself remains
the directional statement every candidate must demonstrate.

## Preserved-primitives interactions

v1's preserved primitives (per redesign prompt SECTION 3) constrain v2
candidates' axis positions. Walking each preserved primitive against the
axes:

| Preserved primitive | Axes implicated | Constraint implied |
|---|---|---|
| Journal (`docs/journal/YYYY-MM-DD.md`, freeform per-cycle) | Axis 3 (memory shape) | Journal remains as one memory channel; candidates may add others. Note: Axis 3 "memory derivative of state" position is doubly rejected (constraint 7 + journal-as-existing-channel). |
| Cycle-issue (`orchestrator-run` label, session-bracket comments) | Axis 7 (orchestration topology) | All topologies must produce session-end summary on cycle-issue. Multi-pattern coexisting topologies may have multiple sub-cycles within one cycle-issue boundary — sub-cycles are internal to the issue. |
| Question-for-eva / input-from-eva | Axis 12 (Reconciliation) | Inbound Eva channels must be reconciled. Pure write-only outbound rejected (the v1 F2 anti-pattern). |
| Git-safety (commit-must-be-pushed) | Axis 4 (history substrate) | Branching positions must be in-tree files (per-branch-named files committed in main), not git-branches that might not be pushed. Git-as-substrate position naturally honors this. |
| Cycle-runner harness | Axis 13 (Harness-vs-session boundary) | Cycle-runner change scope must be declared (none / modest / substantial). Different Axis 13 positions imply different changes; candidates must specify. |

**Note on constraint surface area:** preserved-primitives interactions add
explicit constraints atop the axis position-space. A candidate that picks
"branching checkpoints" on Axis 4 must specify in-tree-files implementation;
a candidate that picks "fat harness" on Axis 13 must specify the cycle-runner
change scope; etc. These are not new axes — they're refinements of position
specifications.

## Phase 2 candidate template (preliminary)

A Phase 2 candidate should declare its position on each of the 12 axes
(1-10, 12, 13) plus the CORE-DESIGN-PRINCIPLE elaboration (folded into
Axis 13 in v1.2), the cross-axis dependencies it commits to, and the
preserved-primitives constraints it honors. Suggested structure:

```
## Candidate <N>: <name>

### Position summary
- Axis 1 (decomposition): <position> — <one-sentence rationale>
- Axis 2 (state representation): <position> — <one-sentence rationale>
- Axis 3 (memory shape): <position> — <one-sentence rationale>
- Axis 4 (history substrate): <position> — <one-sentence rationale>
- Axis 5 (plans-as-artifacts): <position> — <one-sentence rationale>
- Axis 6 (extension shape): <position> — <one-sentence rationale>
- Axis 7 (orchestration topology): <position> — <one-sentence rationale>
- Axis 8 (mechanical enforcement): <position> — <one-sentence rationale>
- Axis 9 (iteration ceilings): <position> — <one-sentence rationale>
- Axis 10 (entropy mitigation): <position> — <one-sentence rationale>
- Axis 12 (reconciliation discipline): <position> — <one-sentence rationale>
- Axis 13 (harness-vs-session): <position> — <one-sentence rationale>

### Cross-axis commitments
- Axis 1 × Axis 7: <how this candidate handles the dependency>
- Axis 2 × Axis 3: <...>
- Axis 4 × Axis 2: <...>
- Axis 12 × Axis 4: <...>
- Axis 13 × Axis 6: <...>
- Axis 13 × Axis 8: <...>
- ... (other significant pairs)

### Failure-mode addressing
- F1: <how candidate addresses>
- ... (12 patterns)

### Preserved-primitives compliance
- Journal: <integration shape>
- Cycle-issue: <integration shape>
- Question-for-eva / input-from-eva: <reconciliation mechanism>
- Git-safety: <how branching/append-only honors commit-must-be-pushed>
- Cycle-runner: <change scope: none / modest / substantial; specifics>

### What this candidate gives up
- Honest list of design dimensions where this candidate is weaker than
  alternatives — what it trades away to gain its strengths.

### Tool surface implied
- List of tools the candidate's prompt expects to invoke; which exist;
  which would be net-new to build.

### Migration cost from v1
- Specific migration steps; what state/tools/conventions transfer vs need
  replacement.
```

The template is preliminary and subject to iteration before Phase 2
candidate generation begins. The post-retrospective checkpoint gates that
work; this template is preparation, not commitment.

## What the framework does NOT yet specify

Honest gaps for cycle-38+ iteration:

- **Security posture per-trust-tier specifics.** Convergent constraint 3
  (Strong-defaults security with operator-controlled knobs) is named but
  the trust-tier specifics (how does the prompt handle untrusted text from
  different sources?) are folded into the convergent constraint as
  implementation detail rather than candidate-differentiation axis. v2
  candidates must honor; specifics are not axis-level.
- **Polyglot strategy for schema-domain work.** Folded into Axis 6
  (extension shape — language-port tools are extensions). Phase 3
  prototype's polyglot end-to-end test is the load-bearing test. Phase
  2 candidate generation may surface that polyglot deserves explicit
  axis treatment if candidates diverge significantly here.
- **Concrete reconciliation primitives.** Axis 12's positions are
  abstract (no reconciliation / active polling / event-driven / hybrid).
  v2 candidates need to specify the actual GitHub-Actions / cron / webhook
  / state-recompute mechanism. Cycle 38+ may add a "reconciliation
  primitive catalog" subsection to Axis 12.
- **Phase 1 research for systems queued by Eva directives.** Cognition
  Devin (#2779) and OpenAI harness (#2781) re-dispatches were authorized
  by Eva (#2794) but not yet executed. Their findings may surface
  additional cross-system patterns that constrain or differentiate Phase
  2 candidates further. Cycle-37 deferred re-dispatch to allow framework
  v1.2 application; cycle-38+ will execute.
