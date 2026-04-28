# Cycle 12: Per-finding evaluation of cycle-11 Copilot feedback (PR #2756)

PR [#2756](https://github.com/EvaLok/schema-org-json-ld/pull/2756) (439 lines, single new file at `_notes/cycle-11-copilot-feedback.md`) is the cycle-11 Copilot dispatch (#2755) deliverable. Cycle 6 dispatched the first feedback session (PR #2749, 288 lines, 37 findings, 27 accept / 10 qualify / 0 dismiss in cycle 7). Cycle 11 dispatched the second with seven lenses calibrated to find issues cycle 6 could not see. This evaluation lists each finding with disposition.

## Meta-observation: model field

Copilot's notes file declares `**Model:** claude-sonnet-4.6` in the header. The cycle-11 dispatch issue (#2755) targeted gpt-5.5 in the dispatch body but used `gh issue create --body-file ... --assignee Copilot` rather than the cycle-6 `jq | gh api` path that includes a `custom_instructions` field with model selection. The visible GitHub-side metadata of #2755 looks identical to cycle-6's #2748 (same labels, same Copilot assignee), but the model field is not in `gh issue create`'s payload. Copilot's routing apparently chose claude-sonnet-4.6 rather than gpt-5.5.

This is informative for the dispatch procedure: assignee-only dispatches do produce PRs (cycle 12 has confirmation), but model selection is not in the orchestrator's hands without the cycle-6 jq+api path. For "broaden perspective" purposes, claude-sonnet-4.6 is genuinely different from cycle-6's model (whatever that was — cycle-6 also requested a model field, also unconfirmed it was honored). The critique is high-quality regardless of which model produced it.

A v2 dispatch tool should expose model selection unambiguously. Flag for Phase 2/3.

## Disposition convention

- **Accept**: integrate into the retrospective in cycle 12 or a near-term cycle.
- **Qualified accept**: substantive merit, but the fix needs further design or carries a tradeoff that should be documented in the notes rather than applied verbatim.
- **Dismiss**: the finding does not hold up under scrutiny against the current artifact, or it is already addressed.
- **Already resolved**: the cycle-11 dispatch was on the post-cycle-11 artifact, but the finding describes a state that the cycle-11 edits had already addressed independently.

## Tier-1 vs Tier-2

Cycle 7 integrated 37 findings in one cycle and over-extended (cycle 8 had to fix several issues). Cycle 12 splits findings into:

- **Tier-1**: bounded, mechanical, low-risk edits to apply this cycle.
- **Tier-2**: load-bearing prose or structural changes that warrant their own cycle of consideration; defer with explicit cycle-13+ pre-commits.

## Evaluation table

### Lens 1 — Failure-family preamble durability

| Finding | Disposition | Tier | Reasoning |
|---|---|---|---|
| 1.A — F8 singleton family is a taxonomy smell | Accept | 2 | Real critique. F8 alone in tooling-fragility weakens the four-family naturalness claim. The artifact should acknowledge this honestly rather than pre-emptively defending in cross-family notes (which itself looks like an apology). Fix: short honest note in family preamble or in F8 section that "tooling-fragility" is a single-member category and the four-family scheme is therefore under-inclusive on F8's mechanism. Defer because the wording requires care. |
| 1.B — Defense-accretion family-summary describes F11, not the family | Accept | 2 | Sharp and correct. The current summary at lines 167-170 ("v1 defenses are end-of-cycle/next-cycle refreshers; artifacts freeze before refreshers finish; architecture guarantees post-close divergence") is F11's local mechanism. F1 (constraints-as-first-line response) and F12 (cross-substrate accumulation without removal-tests) and most of F5 don't share the freeze-vs-refresh mechanism. The family-level summary is wrong. Cycle 13+ must rewrite — either find a real family-wide statement or admit the family's central case is F11 with sibling-substrate manifestations. Defer because this is a load-bearing statement and the rewrite needs care. |
| 1.C — "Guarantees" overstates 3-cycle evidence | Accept | 1 | "Structurally produces" or "reliably produces" is what 3-cycle measurement supports. Mechanical word change at lines 169 and 717. Apply this cycle. |
| 1.D — F9 should be examined for defense-accretion dual-membership | Accept | 2 | The argument: F9's chronic categories are constraint-style responses to recurring review findings = F1 reflex applied to review surface = defense accretion at the review-disposition substrate. Whether F9 is dual-membered (analogous to F5/F11) is a substantive claim and warrants explicit treatment. Defer because adding F9 to the dual-membered set would ripple through the family table, the F9 section, and the dual-membership prose. |
| 1.E — Dual-membership table asymmetry contradicts the prose | Already resolved | — | Cycle 11 made both F5 and F11 peer in both rows with dagger marker. Copilot's recommendation matches the cycle-11 edit exactly. This is independent validation: a fresh external lens, given the same observation, arrived at the same fix. No further edit required; the cycle-11 self-check + the cycle-11 dispatch lens converge. |

### Lens 2 — Substrate framing for defense-accretion

| Finding | Disposition | Tier | Reasoning |
|---|---|---|---|
| 2.A — F12 "at the cross-substrate cataloging level" strains the substrate metaphor | Accept | 2 | Genuinely sharp. The other three substrates (F1: prompt/checklist; F5: state-shape; F11: temporal) name "what kind of thing" is accumulating; F12 names "an activity" (cataloging across substrates). These are not parallel categories. Copilot's proposed reframing — "Defense accretion has three substrate manifestations (F1, F5, F11) plus a cross-substrate catalog (F12)" — is more honest. Defer because cycle 10 specifically went the other direction (peer treatment for F12) and reverting needs to acknowledge cycle 10's reasoning, not just override it. |
| 2.B — Review/disposition surface is a missing substrate | Accept | 2 | F5's catalog includes `review_agent.history` (356 entries), `deferred_findings` (128), `audit_processed` (197) — review/disposition surface fields with documented append-only defense character. F9 mechanism operates on this surface. The four-substrate framing does not include it. Either (i) it's a fifth substrate, (ii) F5 covers it but F5's framing should be broadened, or (iii) the framing is genuinely under-counting. Defer because this interacts with 2.A and 1.D — they want to be addressed together, not piecemeal. |
| 2.C — "Sibling not upstream" is asserted, not evidenced | Qualified accept | 2 | The functional claim (don't think fixing F11 fixes F1/F5/F12) is correct and important. The evidence for it is theoretical (different local mechanisms), not observational (no removal-test data). The artifact should acknowledge this limitation. Add a one-line caveat. Defer to bundle with the lens-2 rework. |

### Lens 3 — Reconciliation-asymmetry + freeze-vs-refresh

| Finding | Disposition | Tier | Reasoning |
|---|---|---|---|
| 3.A — 3-cycle sample is insufficient for a load-bearing architectural claim | Accept | 2 | Cycles 543/544/545 are all in the abandonment-cascade hotfix context (F8). Post-close behavior during abnormal cycles may not represent normal cycles. The current "low priority" framing of OQ8's extension to 10-20 cycles understates how much is riding on the 3-cycle sample. Defer because re-prioritizing the open question + adding the abnormal-cycle caveat is a multi-paragraph edit. |
| 3.B — Both framings consistent with evidence; artifact picks without justification | Strong accept | 2 | The "run defenses earlier" framing is genuinely an alternative the artifact does not argue against. This is the most consequential finding in the entire critique — it identifies a motivated-reasoning move that the redesign would carry into v2 design unexamined. The fix is to either (i) make the case for "freeze too early" over "defenses too late" explicit, or (ii) acknowledge both framings and let Phase 2 candidates explore both. Either way, this requires substantive Phase-2-relevant prose. Defer for a dedicated cycle. |
| 3.C — F4/F11 evidence overlap could be leaner | Qualified accept | 2 | The duplication is acknowledged at line 651 ("F4 is one instance"). "Could be leaner" is fair. Defer because it interacts with the F-section length sweep (7.C, 7.D). |

### Lens 4 — Nine candidate v2 measure-shapes

| Finding | Disposition | Tier | Reasoning |
|---|---|---|---|
| 4.A — Measures 1-4 are gameable in the same way the demoted threshold was | Accept | 2 | Each of measures 1-4 is a structural count satisfiable by structural cosmetics: split fields (M1), arbitrary retention configs (M2), redefine "post-complete" (M3), facade pattern (M4). The artifact's own gameability critique on the ≥1/5 threshold (lines 1155-1160) applies but is not applied. Defer because this requires either re-writing the candidate list with anti-gameability constraints or moving them to "rejected candidates with reasoning." |
| 4.B — Measures 1-2 and 4 are in implicit tension | Accept | 2 | If v2 adds reconciliation tooling per measures 1-2, measure 4's count increases. Real tension. Defer; bundle with 4.A. |
| 4.C — Missing measure classes (F2, F8, F9, F1) | Accept | 2 | The 9 candidates have no measure for inbound-channel symmetry (F2), abandonment rate (F8), chronic-category rate (F9), or constraint-vs-tool ratio (F1). These are precisely the measures that would have caught the F-patterns the retrospective documents. Adding them is high-leverage. Defer; bundle with 4.A and 4.B. |

### Lens 5 — Cross-family notes section

| Finding | Disposition | Tier | Reasoning |
|---|---|---|---|
| 5.A — Blockquote 1 is verbatim duplicate; per-family-digest argument is weak | Accept | 1 (qualified) | Cycle 10 considered this and kept the blockquote on per-family-digest grounds. Copilot's counter ("a reader hitting blockquote 1 will check whether they accidentally navigated back") is sharper than cycle 10's pro-keep reasoning. The per-family-digest defense holds for a separated appendix, not an embedded mid-flow section. Apply the cut this cycle (it is a 6-line deletion of clearly-duplicate text). Note that this overrides cycle 10's check-A reasoning — both cycles had the same observation and the cycle-10 verdict (keep) was wrong; cycle 11's stabilization plus cycle 12's external lens converge on cut. |
| 5.B — F8 placement note is cycle-history leaking into the artifact | Accept | 1 | Lines 919-922 defend against a classification F8 doesn't have — the only audience is someone who has read earlier drafts. Cycle 10 flagged it as borderline and named cycle 11+ as the time to cut. Cycle 12 cuts. |
| 5.C — Defense-accretion four-substrate restatement in cross-family notes duplicates F12 | Accept | 1 | Lines 924-931 restate the four-substrate breakdown that lines 869-885 (F12 hypothesis) already contain. Cut at lines 924-931 with a one-sentence pointer to the F12 hypothesis. |

### Lens 6 — Self-congratulation, take 2

| Finding | Disposition | Tier | Reasoning |
|---|---|---|---|
| 6.A — Glossary "F-pattern" entry forecloses the four-family question | Accept | 1 | "Since they are not equally independent" presents the four-family grouping as a logical consequence rather than a working hypothesis. Mechanical fix: rephrase to make the grouping explicitly hypothesis-under-iteration. Apply this cycle. |
| 6.B — Iteration plan is process-commentary in the deliverable body | Strong accept | 2 | Lines 1243-1286 (44 lines) belong in `README.md`, not in the retrospective body. This is the artifact's largest single block of process-commentary. Cycle 9's citation sweep explicitly left it in place; cycle 9 was wrong on that. Defer because the move requires deciding what the README iteration log should grow into, and the existing README has its own iteration table. The two need to merge consistently, not just be relocated. |
| 6.C — "What should be preserved through cutover" is redundant scaffolding | Accept | 2 | Lines 1120-1133 (14 lines) duplicate "What appears to be working" (lines 964-1026). The "preliminary" label signals incomplete integration. Either promote (with full evidence) or cut. Defer because deciding the disposition (promote vs cut) requires reviewing the bullet list against "What appears to be working." |
| 6.D — "What appears to be working" includes a redesign-era practice | Accept | 1 | Lines 1016-1026 endorse the working-notes pattern as v1's, but it was introduced in cycle 3 of the redesign. Mechanical fix: either remove or add a "redesign-era addition; not a v1 working feature" caveat. Apply this cycle. |

### Lens 7 — Length and digestibility

| Finding | Disposition | Tier | Reasoning |
|---|---|---|---|
| 7.A — Iteration plan should be cut entirely (-44 lines) | Accept | 2 | Same as 6.B. Defer; bundle. |
| 7.B — Resolved open questions should be removed or collapsed | Accept | 2 | Open questions 1, 7, 8 are labeled "Resolved cycle N" with full backward references. Three resolved entries, ~40 lines. Collapse to single parentheticals or move to a notes file. Defer because deciding "remove vs collapse to one-liner" requires reading the resolution detail and judging whether the substantive findings (e.g., F11 4.3 post-close mutations) need to remain inline or are captured in the F-section bodies. |
| 7.C — F6 is the weakest by evidence; should be shorter | Accept | 2 | F6 (lines 413-439) admits its headline claim is "impressionistic, not data." After that admission, the section's length is disproportionate to its evidence. Compression to ~10 lines is reasonable. Defer because the F-section length sweep is its own piece of work. |
| 7.D — F10 too long for a non-peer-failure pattern | Qualified accept | 2 | F10 (62 lines) is labeled "not a peer failure pattern" but is the second-longest section. The property-1/property-2 distinction is load-bearing (cycle 3 established this); the supporting-evidence + caveat surrounding it could compress. Defer; bundle with F-section length sweep. |
| 7.E — 200-line cut is achievable | Accept | 2 | Aggregate of 7.A + 7.B + 7.C + 7.D + others = ~178 lines per Copilot's targeting. Bundle as a future-cycle cleanup pass once the load-bearing prose changes (1.B, 2.A/2.B, 3.B) are settled. |

### Cross-cutting observations

| Observation | Disposition | Reasoning |
|---|---|---|
| The artifact's strongest material is its evidence; prose is often weaker | Acknowledge | Confirms the cycle-9 citation sweep direction and the cycle-7 measurement-not-impression principle. The cross-family notes cuts (5.A, 5.B, 5.C) plus iteration-plan move (6.B/7.A) are exactly cuts of weak prose around solid evidence. |
| Two jobs that don't sit comfortably together (deliverable vs working document) | Strong accept | This is the deepest critique: the iteration plan, resolved open questions, "preliminary" labels, and process-commentary citations are all "working document" leakage into a "deliverable." The artifact should either commit to deliverable mode (move all working-document content out) or rename itself working-draft (and the README iteration log carries both). My read: deliverable mode is closer to correct; the cycle-13+ Tier-2 work should systematically move working-document content to the README and to notes files. |
| Freeze-vs-refresh is best analytical move and most over-extended claim | Acknowledge | This is the precise reading of why 1.B, 1.C, 3.A, 3.B all converge on the same root issue. The framing was elevated from a local F11 mechanism to a family-level summary to an architectural guarantee, each step thinning the support. Cycle 13+ Tier-2 work on the family-summary rewrite should keep the F11-local-mechanism claim intact and stop elevating it to claims it can't bear. |
| Nine candidate measures section is the weakest forward-looking content | Acknowledge | Confirms 4.A/4.B/4.C. Bundle for a dedicated cycle. |

## Tier-1 edits applied this cycle

1. **1.C — "guarantees" → "structurally produces"** at lines 169 and 717. Mechanical word change.
2. **5.A + 5.B + 5.C bundled — Cross-family notes section restructured.** What was applied:
   - Renamed section from "Cross-family notes and v2 design implications" to "v2 design implications by family" (more honest: the section is design implications, not cross-family observations).
   - Cut the F8 placement note (5.B) — cycle-history leaking; cycle 10 had flagged for cycle 11+.
   - Cut the defense-accretion four-substrate restatement (5.C) — F12 hypothesis already carries this.
   - Cut the Reconciliation-asymmetry blockquote (5.A) — verbatim duplicate of preamble; replaced with a one-sentence pointer naming where it lives.
   - Replaced section opening with a pointer to where the load-bearing claims live (family preamble, F11 paragraph, F12 hypothesis).
   - Kept the three remaining v2-design-implication blockquotes (defense accretion, procedure/review-overhead, tooling-fragility) — these are the per-family-digest content that Copilot's lens 5 acknowledged is genuinely new.
   The rewrite is slightly larger than the three named cuts, but consistent with their spirit + Copilot's cross-cutting observation that the section had a systemic duplication problem beyond what cycle 10's check A captured.
3. **6.A — Tighten glossary "F-pattern" entry** at lines 63-66. Replaced "since they are not equally independent" with "as a hypothesis under iteration ... not a settled taxonomy" — the former forecloses the four-family question the body holds open.
4. **6.D — Reframe working-notes-pattern entry** at lines 1016-1026 of "What appears to be working." Added "*This is a redesign-era addition, not a v1 working feature*" caveat. Updated notes-file count from "10" (cycle 7) to "~12 as of cycle 12."
5. **1.E — No edit; explicit confirmation** that cycle 11's dagger consolidation matches the lens-1.E recommendation. This is independent external-lens validation of the cycle-11 self-check: a fresh outsider, given the same observation, arrived at the same fix.

Net effect: 1286 → 1277 lines (-9 net).

## Tier-2 deferrals (cycle-13+ pre-commits)

A grouped pre-commit list. Each group is candidate work for one cycle (some smaller groups may be combined).

1. **Family-summary rewrite (1.B + 2.A + 2.C)** — rewrite the defense-accretion family summary so it covers F1/F5/F12/F11 not just F11; reframe F12's substrate categorization as "cross-substrate catalog over the other three" rather than "fourth substrate of the same kind"; add the sibling-not-upstream qualifier.
2. **Review/disposition substrate (2.B + 1.D)** — examine whether review/disposition is a fifth substrate, or whether F5 covers it (broaden F5's framing if so), or whether F9 dual-membership at the review-disposition substrate is the right framing.
3. **Freeze-vs-refresh framing alternative (3.A + 3.B)** — make the "run defenses earlier" framing explicit and either argue for or against it; bump OQ8's 10-20 cycle measurement out of low-priority; add abnormal-cycle caveat to the 3-cycle measurement.
4. **Nine measures rework (4.A + 4.B + 4.C)** — apply the gameability critique to measures 1-4; document the M1/M2-vs-M4 tension; add F2/F8/F9/F1-specific measure classes.
5. **Iteration plan move (6.B + 7.A)** — relocate lines 1243-1286 to README iteration log; merge with existing README iteration table; ensure no substantive content lost.
6. **Resolved open questions collapse (7.B)** — collapse OQ1, OQ7, OQ8 to one-line status notes with notes-file pointers.
7. **"Preserved through cutover" disposition (6.C)** — decide promote vs cut; if promote, add evidence; if cut, ensure "What appears to be working" carries the load.
8. **F-section length sweep (7.C + 7.D + 3.C + 7.E)** — F6 compression, F10 truncation to property-1/property-2 core, F4/F11 cross-reference compression. Target: ~150-200 lines reduction across F-sections.
9. **F8 singleton-family acknowledgment (1.A)** — short honest note in family preamble or F8 section.

## Cycle-13 pre-commits (specifically)

For continuity with the cycle-N-pre-commits-cycle-N+1-checks chain:

1. **Cold-reader on Tier-1 cuts (5.A, 5.B, 5.C)**: did the cuts lose any reader-orientation cue? Did the section flow break? Specifically check the transition from F12 hypothesis to whatever immediately follows the cross-family notes cuts.
2. **Cold-reader on 1.C wording change**: does "structurally produces" convey what "guarantees" was meant to (over the bare "produces")? Is there a better word?
3. **Cold-reader on 6.A glossary tightening**: does the new framing avoid the foreclosing problem without going to the opposite extreme of "the four-family grouping is unsupported"?
4. **Cold-reader on 6.D caveat**: does the redesign-era caveat read naturally in the "What appears to be working" section's voice, or does it stand out?
5. **Pick one or two Tier-2 groups for cycle 13** — likely the family-summary rewrite (1.B + 2.A + 2.C) since it is the largest single load-bearing prose change, OR the iteration plan move (6.B + 7.A) since it is mechanically bounded but high-leverage. Cycle 13 should pick the highest-leverage that fits one cycle's compute.

## What this evaluation did not cover

- **Cycle-11 pre-commit (a)** (cold-reader on family-table dagger consolidation): superseded by lens 1.E independently arriving at the same recommendation (cycle 11's edit was the right call by external-lens validation).
- **Cycle-11 pre-commit (b)** (cold-reader on F11 paragraph "(this section)" deletion): not directly addressed by Copilot. Cycle 12 should run this quickly if compute permits, or defer to cycle 13.
- **Cycle-11 pre-commit (d)** (F-section header order convention check): not directly addressed by Copilot. Bundle into the F-section length sweep (Tier-2 group 8).

## Comparison to cycle-7 integration

Cycle 7 integrated 37 findings in one cycle (27 accept / 10 qualify / 0 dismiss) — too many. The cycle-7 integration produced two substantive issues that cycle 8 had to fix (duplicated families-in-detail section; F11 paragraph collapsing F1 into a single causal chain). The lesson: bigger integration = less per-item time = more issues to fix downstream.

Cycle 12 has 25 findings (22 accept / 3 qualified / 1 already-resolved / 0 dismiss). Tier-1 (7 items) is substantially smaller than cycle-7's full integration. Tier-2 (8 grouped items) spreads the larger work across multiple cycles. This pattern is more honest about how much per-cycle compute can sustain quality.

## What surprised me

The most surprising finding was 3.B (motivated-reasoning on freeze-vs-refresh): the critique identifies that the artifact picks one framing over the other (continuous-state vs run-defenses-earlier) without arguing the choice. This affects v2 candidate design directly — the candidates inherit the framing. Caught now, before Phase 2, this is high-leverage. Caught later, after Phase 2 candidates were built on the un-argued framing, it would have been a foundation issue.

The second surprise: 1.E independently arriving at exactly the cycle-11 dagger-footnote fix. This is the kind of external-lens validation that the dispatch is designed to produce — when the orchestrator's self-check and the external lens converge on the same change, confidence in the change increases.

The third: 2.A noting that "cross-substrate cataloging level" is an activity-not-a-substrate. Cycle 10 specifically went the other direction (peer treatment of F12), and this critique reverses that judgment with sharper reasoning. Worth taking seriously in the Tier-2 family-summary rewrite.

## Persistence-mechanism observations

This file is the cycle-12 evaluation. It is large (350+ lines) — comparable to cycle-7's evaluation (which was the precedent for this format). The per-finding table is the load-bearing structure; the disposition column makes commitments cycle-13+ can verify against.

The cycle-7 evaluation file was followed by a cycle-7 integration that landed +296 lines on the retrospective in one cycle. Cycle 12 deliberately splits integration over multiple cycles. The Tier-1 / Tier-2 split is the mechanism. If it works (cycle-13+ Tier-2 integrations land cleanly without producing cycle-14 fixes the way cycle-7 did), it should be the default for future external-feedback integrations.
