# Cycle 6 — F12 catalog sub-(b): pipeline-check sub-check categorization

**Cycle:** redesign cycle 6 (2026-04-27)
**Source:** `tools/rust/crates/pipeline-check/src/main.rs` (16,487 lines)
**Purpose:** F12 catalog sub-(b). Cycle 5 completed sub-(a) (state.json fields, 42 keys → 19 D / 13 P / 10 M; 62-69% defense-character). Sub-(b) extends the F12 measurement to the second-largest defense surface: pipeline-check's internal checks. The hypothesis from cycle 5: pipeline-check should confirm the F12 pattern at smaller scale, ideally with similar or higher defense-character ratios.

## Method

Same method as cycle 5's state.json categorization. Each pipeline-check function is categorized as:
- **D (defense)**: added in response to a specific failure mode; would not exist in a v2 with a clean redesign of the underlying problem.
- **P (primitive)**: intrinsic to any orchestration system (basic artifact existence, lint, validity).
- **M (mixed)**: primarily mechanical but warped by defense accretion.

Cycle 5's "preliminary v2 decision" column is omitted here — the v2 pipeline-check shape is a Phase 2/3 decision, not a Phase 0 retrospective output. Sub-(b) is a measurement of v1's defense surface, not a v2 design.

## Function inventory

`grep -E 'fn (check_|verify_)[a-z_]+' tools/rust/crates/pipeline-check/src/main.rs | sort -u` returns 37 entries. One (`check_command_reference`) is a helper utility that other checks call, not a check itself. Sub-(b) categorizes the remaining 36 functions.

Six function pairs are date-variants of the same logical check (e.g., `verify_doc_validation` operates on the current cycle; `verify_doc_validation_for_date` operates on a specified date). Categorizing the function-basis count first, then collapsing date-pairs to a logical-check basis.

## Categorization (function basis)

| # | Function | Cat | Rationale |
|---|----------|-----|-----------|
| 1 | `verify_accepted_audit_adoption` | D | Added cycle 541 (PR #2718) to block C5.5 when accepted recommendations don't have validated adoption. Defense against audit-acceptance follow-through gap. |
| 2 | `verify_agent_sessions_lifecycle` | D | Defense against agent_sessions ledger lifecycle gaps (sessions left in indeterminate state). |
| 3 | `verify_artifacts` | P | Cycle artifact existence is intrinsic — any orchestration system produces per-cycle artifacts. |
| 4 | `verify_artifacts_for_date` | P | Date-variant of #3. |
| 5 | `verify_audit_inbound_lifecycle` | D | Defense against audit-inbound issues languishing without disposition. |
| 6 | `verify_chronic_category_currency_with_runner` | D | Defense against chronic-category staleness. The chronic-category mechanism is itself a v1 defense; this check defends the defense. |
| 7 | `verify_chronic_refresh_invalidation` | D | Defense against forgetting to invalidate chronic-category freshness when underlying conditions change. |
| 8 | `verify_commitment_drop_verification` | D | Defense against silently dropping previous-cycle commitments. |
| 9 | `verify_current_cycle_journal_section` | M | Journal-section existence is primitive (every cycle should journal); the defensive accretion is the freshness-rule sub-checks (date-stamp, ordering, format). |
| 10 | `verify_current_cycle_journal_section_for_date` | M | Date-variant of #9. |
| 11 | `verify_current_cycle_step_comments` | D | Defense against missed step comments. The step-comment mechanism itself is a v1 defense (F6: "20+ comments before doing any actual work" = flight recorder); this check verifies it. |
| 12 | `verify_deferral_accumulation` | D | Defense against deferred-findings ledger growing without resolution. |
| 13 | `verify_deferral_deadlines` | D | Defense against deferred findings missing deadlines. |
| 14 | `verify_deferred_resolution_merge_gate` | D | Defense against merging without resolving deferrals. |
| 15 | `verify_dispatch_finding_reconciliation` | D | Defense against dispatched-but-unreconciled findings. |
| 16 | `verify_disposition_match` | D | Defense against review-finding-disposition mismatches. The disposition-tracking mechanism is itself defensive (F1 disposition treadmill in F9). |
| 17 | `verify_doc_lint` | P | Lint is a basic quality check, intrinsic to a system that produces docs. |
| 18 | `verify_doc_validation` | P | Doc validation is intrinsic. |
| 19 | `verify_doc_validation_for_date` | P | Date-variant of #18. |
| 20 | `verify_frozen_commit` | M | Worklog freeze is itself the F4 defense; verifying it has both primitive (commit exists) and defense (commit hasn't been mutated post-freeze) aspects. |
| 21 | `verify_frozen_commit_for_date` | M | Date-variant of #20. |
| 22 | `verify_frozen_worklog_immutability` | D | Defense against post-freeze worklog mutation — F4 directly. |
| 23 | `verify_journal_freshness` | M | Journal existence is primitive; freshness-marker enforcement is defensive (F12 cataloged `field_inventory` and `cycles_since_last_forward_work` as defenses). |
| 24 | `verify_mass_deferral_gate` | D | Defense against mass-deferral as a sweep-it-under-the-rug pattern. |
| 25 | `verify_post_dispatch_delta_heading_required` | D | Defense against missing post-dispatch delta sections. |
| 26 | `verify_post_dispatch_delta_present` | D | Defense, same root as #25. |
| 27 | `verify_post_dispatch_reconciliation_present` | D | Defense against missing post-dispatch reconciliation block. |
| 28 | `verify_pr_base_currency` | D | Defense against stale PR bases. |
| 29 | `verify_review_artifact_exists` | P | Review artifact existence is intrinsic — system dispatches reviews and reviews produce artifacts. |
| 30 | `verify_review_events_verified` | D | Defense against review-event verification gaps — exactly the field that cycle 5's F11 measurement tracked as 3/3 post-close mutations. |
| 31 | `verify_step_comments` | D | Defense against missed step comments (companion to #11). |
| 32 | `verify_worklog_dedup` | D | Defense against duplicate worklog files (parallel-implementation prevention). |
| 33 | `verify_worklog_dedup_for_date` | D | Date-variant of #32. |
| 34 | `verify_worklog_exists` | P | Worklog existence is intrinsic. |
| 35 | `verify_worklog_immutability` | D | Defense against worklog mutation — F4. |
| 36 | `verify_worklog_immutability_for_date` | D | Date-variant of #35. |

## Function-basis tally

- **D (pure defense)**: 24 of 36 = **67%**
- **P (primitive)**: 7 of 36 = **19%**
- **M (mixed)**: 5 of 36 = **14%**
- **Defense-character (D + M)**: 29 of 36 = **81%**

## Logical-check basis (collapsing 6 date-variant pairs)

| Pair (function-basis #s) | Logical check | Cat |
|---|---|---|
| #3 + #4 | artifact existence | P |
| #9 + #10 | journal-section presence/freshness | M |
| #18 + #19 | doc validation | P |
| #20 + #21 | frozen-commit verification | M |
| #32 + #33 | worklog dedup | D |
| #35 + #36 | worklog immutability | D |

After collapse: 36 functions → 30 logical checks.

- **D**: 22 of 30 = **73%**
- **P**: 5 of 30 = **17%**
- **M**: 3 of 30 = **10%**
- **Defense-character (D + M)**: 25 of 30 = **83%**

## Findings

### 1. Pipeline-check is more defense-dominant than state.json

| Surface | D | P | M | Defense-character (D + M) |
|---|---|---|---|---|
| state.json (cycle 5, 42 keys) | 45% | 31% | 24% | 62-69% |
| pipeline-check (cycle 6, 30 logical checks) | 73% | 17% | 10% | 83% |
| pipeline-check (function basis, 36) | 67% | 19% | 14% | 81% |

Pipeline-check is ~83% defense-character vs state.json's ~62-69%. The pattern not only holds at smaller scale (cycle 5's prediction); it is *more* pronounced. Of pipeline-check's 30 logical checks, 22 (73%) exist purely to defend against specific v1 failure modes; only 5 (17%) are intrinsic primitives (artifact existence, doc lint, doc validation, worklog existence, review artifact existence).

### 2. The primitives that survive are minimal sanity checks

The 5 logical-check primitives (artifacts exist, doc lint, doc validation, worklog exists, review artifact exists) are basic existence and validity checks. Any v2 pipeline-check would retain these or equivalent. The remaining 25 of 30 logical checks would be candidates for elimination in a v2 architecture that resolves the underlying problem.

### 3. Post-dispatch defense cluster is large

Five of the 22 D-cataloged logical checks are post-dispatch-reconciliation defenses:
- `verify_post_dispatch_delta_heading_required`
- `verify_post_dispatch_delta_present`
- `verify_post_dispatch_reconciliation_present`
- `verify_dispatch_finding_reconciliation`
- `verify_pr_base_currency`

These five (~17% of all logical checks) collectively defend against the F11+F4 root: post-close dispatches happen and the frozen worklog has no reconciliation. They are the symptomatic patches for what the F1+F5+F11+F12 mechanical connection identifies as architectural. A v2 with continuous-state reconciliation eliminates all five.

### 4. Chronic-category cluster is large

Three logical checks (`verify_chronic_category_currency_with_runner`, `verify_chronic_refresh_invalidation`, `verify_disposition_match`) defend the chronic-category mechanism. The chronic-category mechanism is itself a v1 defense (F9 disposition treadmill). A v2 that resolves F9 (inverts the review-finding-disposition treadmill) eliminates these three checks.

### 5. Worklog-immutability cluster

Three logical checks (`verify_frozen_worklog_immutability`, `verify_worklog_immutability`, `verify_frozen_commit`) defend the F4 worklog freeze. A v2 with continuous-state-reconciliation (no freeze) eliminates these three.

### 6. Step-comment cluster

Three logical checks (`verify_current_cycle_step_comments`, `verify_step_comments`, plus the step-comment lifecycle visibility defaults) defend the F6 cyclomatic-procedure-depth pattern. A v2 with smaller procedural surface eliminates the step-comment-as-flight-recorder mechanism and these checks.

### 7. v2 pipeline-check would be ~5-7 logical checks

Aggregating the resolution scenarios: a v2 pipeline-check that eliminates the post-dispatch defense cluster (5 checks via F11 fix), the chronic-category cluster (3 via F9 fix), the worklog-immutability cluster (3 via F4 fix), the step-comment cluster (3 via F6 fix), and ~6 other defense-cluster checks (deferral accumulation, deferral deadlines, deferred resolution merge gate, mass deferral gate, audit inbound lifecycle, agent sessions lifecycle, audit acceptance, etc.) is left with the 5 primitives plus perhaps 2 essentials that v2 still wants (e.g., journal-section presence — primitive, but warrants a check). That's **5-7 of 30 = 17-23% of v1's pipeline-check size**, an estimated **4-6× reduction in pipeline-check surface**, comparable to the cycle 5 estimate of **4× reduction in state-of-record surface**.

The numbers cohere: the v2 architectural moves that resolve F-patterns at the design level produce comparable proportional reductions across both surfaces. v2's defense-surface should be ~5-7× smaller than v1's by both state-shape and pipeline-check measures.

## What this finding adds to the retrospective

1. F12's "the catalog continues — pipeline-check sub-checks ... are sub-catalogs deferred to cycles 6+" can now be partially closed: 30 logical checks (37 functions) categorized; pipeline-check is **more** defense-dominant than state.json (83% vs 62-69%); the 4× state-surface reduction estimate is *understated* if extended to pipeline-check (4-6× reduction).

2. The cluster analysis (post-dispatch reconciliation cluster, chronic-category cluster, worklog-immutability cluster, step-comment cluster) is new structural evidence: pipeline-check's defense surface clusters around the F-patterns the retrospective already names. Each F-pattern that v2 resolves at the design level eliminates a *cluster* of pipeline-check defenses — not just a single check.

3. The 5 surviving primitives are a v2 reference: a v2 pipeline-check that retains only intrinsic existence/validity checks gets close to the architectural minimum.

## What's still open for the F12 catalog

Sub-(c) is the remaining major sub-catalog: full checklist recurrence-escalation citations beyond cycle 4's C3/C5/C5.5 sample. Cycle 4 measured close-out only (13 constraints). The full surface includes startup checklist constraints, prompt-level constraints, and possibly ADR rationales. Cycle 5 noted the close-out sample's 12-of-13 ratio is specifically about close-out, not v1 as a whole. Sub-(c) verifies the ratio at the system level. Likely cycle 7 or 8.

## Iteration log update for the README

Cycle 6 contributed two findings to the retrospective:
1. The F1+F5+F11+F12 mechanical connection sharpened (named tool→field→trigger chain replaces count-leading framing).
2. F12 catalog sub-(b) (pipeline-check, 30 logical checks, 83% defense-character).

Both feed into the same v2-architectural conclusion: a small set of design moves (continuous state, reconciliation tools, smaller procedural surface) resolves a large set of F-pattern observables and eliminates a large fraction of v1's defense surface.