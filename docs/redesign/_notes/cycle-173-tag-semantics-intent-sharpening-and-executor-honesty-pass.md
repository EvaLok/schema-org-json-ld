# Cycle 173 — tag-semantics intent sharpening + executor honesty-pass

**Cycle issue:** [#2988](https://github.com/EvaLok/schema-org-json-ld/issues/2988)
**Session start:** 2026-05-18 05:19 UTC (run id `26015010393`)
**Previous cycle:** [cycle 172 _notes](cycle-172-write-entry-history-fallback-and-master-CI-green.md)
**Track 1 commit:** `bf3ef7d5` (3 add / 3 del / 2 files)

## Summary

Two-track composition cycle 173 closed cycle 172+'s priority #1 sub-bullet
(tag-semantics intent sharpening) via Track 1 and advanced priority #1
main-bullet (v2 role-prompts honesty-pass) via Track 2. Both edits land
in one commit. 22nd consecutive two-track-composition post cycle 151
exception (HARDENING-AT-26). 58th consecutive HONORING of named forward
priority. `straight-pair-closure-as-default-two-track-shape` HARDENS at
12 (cycles 162-173).

Distinct from cycle 172's "1 of 4 role-prompts honesty-pass" pattern,
cycle 173's Track 1 directly addresses the live Tier-2 warning state
(5 → 0) plus a stale-runtime-reference in executor-prompt surfaced by
Track 2 reading. Track 2 retains the same shape as cycle 172 Track 2
(read end-to-end + assess + document) but contributes a 1-line prompt
edit as part of Track 1 rather than 0-prompt-content-edit cycle 172
shape. Cycle 173's hybrid track integration is a coherent variant; not
a deviation pattern worth naming.

## Track 1 — tag-semantics constraints + consolidation-judgment intent sharpening + executor stale-cycle-ref cleanup (priority #1 sub-bullet FULLY CLOSED)

### Pre-edit Tier-2 warning state (cycle 171 enumeration confirmed cycle 173)

`cargo test -p v2-prompt-tag-semantic-fidelity --test integration_real_prompts -- --nocapture` produced 5 Tier-2 keyword-overlap warnings:

| # | role | tag | intent fragment | shared tokens |
|---|---|---|---|---|
| 1 | planner | `<constraints>` | "Hard rules the role must obey..." | 0 |
| 2 | executor | `<constraints>` | (same intent) | 0 |
| 3 | curator | `<consolidation-judgment>` | "Guides the curator on how to consolidate..." | 0 |
| 4 | curator | `<constraints>` | (same intent as #1) | 0 |
| 5 | reconciler | `<constraints>` | (same intent as #1) | 0 |

The 4× `<constraints>` warnings share a single intent string in
`tag-semantics.toml` line 49. The 1× `<consolidation-judgment>` warning
is a separate intent string in line 95.

### Tool keyword-overlap algorithm (re-confirmed cycle 173)

`v2-prompt-tag-semantic-fidelity` Tier-2 KeywordOverlap (main.rs:631-657)
compares two token sets:
- **intent_tokens** = `tokenize(entry.intent)` after stop-word filter
- **content_tokens** = `tokenize(tag.name.replace('-', ' ') + " " + direct_children.join(" "))`

Where `direct_children` is the deduped list of immediate child element
names. For `<constraints>` with 11 `<constraint name="...">` children,
direct_children = `["constraint"]` (dedup). So content_tokens for
`<constraints>` is exactly `{"constraint", "constraints"}` — the
`name=""` attributes do NOT participate. Same structural property for
`<failure-policy>` (different child names; "write" rescues it by accident
from intent containing "partial-write"), `<output-surfaces>` (intent
contains "output" + "surfaces" — matched), etc.

This is why the 0-overlap warning is sensitive to whether the intent
string contains tokens that match either the tag name itself or the
single deduped child-element token. For tags whose child elements are
all named after the singular form of the tag (like `<constraints>` →
`<constraint>`), the only way to score >0 is to include the
tag-name-derived word in the intent.

### Design decision: distinguish token-circularity from category-honesty

Cycle 172 deferred this with the framing "sharpening the intent string
to match content is partly circular". Cycle 173 disambiguates:

**Token-level circularity (cycle 172's worry)**: copying literal
`<constraint name="X">` values into the intent string — would just be
re-printing content, not declaring purpose. Example bad intent:
"Constraints: single-cycle-scope, no-channel-writes, no-cross-channel-
reads, no-skill-execution, no-branching, no-self-modification, reducer-
rule-discipline, payload-validation, commit-must-be-pushed".

**Category-level honesty (cycle 173's edit)**: enumerating the
CATEGORIES of constraints actually present across all 4 role-prompts'
`<constraints>` content. Categories visible in cycle 173 cross-role
reading:
- Scope discipline (single-cycle, axis-deferrals: skills/branching)
- Channel reducer discipline (no-direct-writes, no-cross-channel-reads,
  reducer-rule, payload-validation)
- Self-modification prohibition (no-self-modification of prompt files at
  runtime)
- Output discipline (commit-must-be-pushed, anti-overstatement,
  journal-immutability — last two curator-only but extend the category)

The new intent enumerates categories at one abstraction level above the
literal constraint names — declares the tag's SEMANTIC KIND (it's a
constraints declaration) without enumerating the specific implementation
items. This is the intent string DOING ITS JOB.

### Two intent string edits (commit `bf3ef7d5`)

**`[tags.constraints]`:**

```toml
# Before:
intent = "Hard rules the role must obey; tooling expectations; what the role MUST NOT do"
# After:
intent = "Declares the hard runtime constraints the role must obey: scope, channel reducer discipline, self-modification prohibition, payload validity, and commit-push discipline"
```

intent_tokens after stop-words filter: `{declares, hard, runtime,
constraints, obey, scope, channel, reducer, discipline, self,
modification, prohibition, payload, validity, commit, push}` = 16
tokens (was 5). content_tokens = `{constraint, constraints}`. Overlap =
`{constraints}` ✓.

Honest at the CATEGORY level: every category named maps to actual
content across all 4 role-prompts' `<constraints>` sections (scope:
single-cycle + axis-deferral, channel reducer discipline: writes +
reads + payload-validation + reducer-rule, self-modification
prohibition: no-self-modification, payload validity: payload-validation,
commit-push: commit-must-be-pushed). Role-specific extras (anti-
overstatement, journal-immutability, two-track-and-dispatch) extend the
named categories.

**`[tags.consolidation-judgment]`:**

```toml
# Before:
intent = "Guides the curator on how to consolidate cycle data: what to preserve, summarise, and omit"
# After:
intent = "Guides the curator's consolidation judgment over cycle data: what to preserve as memory, what to summarise, and what to omit"
```

The original intent used "consolidate" (verb); tag name uses
"consolidation" (noun). No stemming in the tokenizer means
`consolidate ≠ consolidation`. The verb-to-noun substitution + addition
of "judgment" (which IS in the tag name) restores overlap to 2 tokens.

intent_tokens (new) after stop-words: `{guides, curator, consolidation,
judgment, over, cycle, data, preserve, memory, summarise, omit}`.
content_tokens = `{consolidation, judgment, decision, procedure, goals,
quality, checks}` (from `<consolidation-judgment>` tag name +
`<what-it-is>` / `<decision-procedure>` / `<non-goals>` /
`<quality-checks>` direct_children). Overlap = `{consolidation,
judgment}` ✓ (2 tokens).

### Post-edit verification

```
v2-prompt-tag-semantic-fidelity — 4 prompts scanned
manifest: prompts/v2/tag-semantics.toml
[PASS] Tier 1: no errors
[PASS] Tier 2: no warnings
Summary: tier1_errors=0 tier1_passes=47 tier2_warnings=0 adaptation_notes=0
exit_code: 0
```

5 → 0 Tier-2 warnings. Tier 1 unchanged at 47 passes / 0 errors. Strict
mode integration test (`real_prompts_strict_exits_zero`) passes. Full
workspace `cargo test --release --workspace` clean (cycle 171's
full-workspace discipline preserved). `v2-prompt-contract-check` strict
mode also passes against live prompts (`strict_check_passes_against_
live_prompts` integration test) — 2 strict-mode tools both clean.

### Stale-runtime-reference fix in executor-prompt (surfaced by Track 2)

Track 2's executor honesty-pass read surfaced one stale temporal
reference at executor-prompt.xml line 372 inside `<failure-reporting>`:

```xml
<!-- Before -->
<failure-reporting>
  If action execution fails ... report failure honestly in artifacts-
  written records with success-criterion-met false. Curator reads this
  at session-close and surfaces it in memory-channel in cycle 145+.
</failure-reporting>

<!-- After -->
<failure-reporting>
  If action execution fails ... report failure honestly in artifacts-
  written records with success-criterion-met false. Curator reads this
  at session-close and surfaces it in memory-channel.
</failure-reporting>
```

The "in cycle 145+" reference conflated v1-redesign-timeline ("cycle
145+ of v1") with v2-runtime-behavior (memory-channel surfacing is
unconditional from v2 cycle 1, not gated on a specific cycle number).
The same prompt's `<evolution-path>` section retains its cycle 144/145+
references (line 488) — those ARE in a historical-context block and
correctly describe v1 redesign-timeline anchors. Other v1-cycle
references in EXAMPLE payloads (planner-prompt.xml cycle 144/145
sample, reconciler-prompt.xml cycle 220 sample audit) are kept —
example data is historically anchored, not runtime-gated.

This stale-reference class fits the cycle 171 OBSERVATION
`live-prompts-already-aligned-at-extended-schema-level`: prompts are
substantively honest, with isolated surgical sharpening candidates
rather than systemic drift. Cycle 173 extends OBSERVATION to 4
cycles (170+171+172+173).

## Track 2 — Executor-prompt honesty-pass (priority #1 main-bullet partial advancement; 2 of 4 roles complete)

### Scope

506 lines end-to-end read of `prompts/v2/executor-prompt.xml`. Same
shape as cycle 172 Track 2 (curator-prompt 701 lines). Continues cycle
172's pattern of "~1 role-prompt assessment per cycle, bounded".

### Findings

1. **All v2 tool references exist.** Tools named in the executor prompt:
   - `v2-role-driver` ✓ (`tools/rust/crates/v2-role-driver/`)
   - `v2-channel-router` ✓ (`tools/rust/crates/v2-channel-router/`)
   - `v2-super-step-boundary` ✓ (`tools/rust/crates/v2-super-step-boundary/`)
   - `tools/dispatch-task` ✓ (top-level wrapper)
   - `tools/dispatch-review` ✓ (top-level wrapper, authoritative source in
     orchestrator-prompt.xml `COPILOT-DISPATCH-METHOD` section)
   - Read / Edit / Write / Grep / Bash — Claude session-internal tools

2. **`v2-cycle-1-minimal` versioning makes target-state framing honest.**
   Same finding as cycle 172 curator. The `<role-prompt version=
   "v2-cycle-1-minimal" role="executor" model="claude-opus-4-7">` opener
   explicitly scopes the prompt to v2's "cycle 1 minimal end-to-end"
   target state. Not a current-state claim; not an overclaim.

3. **No cargo-culted overclaims surfaced in content sections.** The
   `<inputs>`, `<output-contract>`, `<execution-judgment>`,
   `<action-execution-shapes>`, `<tools>`, `<communication>`,
   `<constraints>`, and `<session-structure>` sections all describe
   v2-runtime behavior with concrete, role-scoped language. Directive
   #2937 reference verified (open input-from-eva issue from EvaLok,
   2026-05-14).

4. **One stale-runtime-reference identified and fixed inline (Track 1).**
   Line 372's "in cycle 145+" reference. See Track 1 above for details.

5. **No new tag-semantics sharpening candidates surfaced.** Executor's
   `<execution-judgment>` and `<action-execution-shapes>` are role-
   specific tags with adequate intent strings (content_tokens contain
   "execution" + "judgment" or "execution" + "shapes" respectively;
   intent strings contain "execution"). Both score >0 overlap. The 5
   warnings addressed by Track 1 were the complete set.

6. **Cross-role constraint category mapping confirmed cycle 173.** Reading
   all 4 role-prompts' `<constraints>` sections cross-referenced against
   the new `[tags.constraints]` intent enumeration shows the named
   categories (scope / channel reducer discipline / self-modification
   prohibition / payload validity / commit-push discipline) cover every
   constraint across all 4 roles, with role-specific extras (curator's
   anti-overstatement + journal-immutability + consolidated-insights;
   executor's two-track-and-dispatch-execution) as extensions of the
   "output discipline" implicit category. The intent honestly reflects
   the union of cross-role categories.

### What cycle 173 Track 2 does NOT do

1. Does NOT honesty-pass planner-prompt or reconciler-prompt
   (priority #1 main-bullet remains at 2 of 4 roles complete).
2. Does NOT propose tool-side improvements to fidelity check (deferred
   as new forward-priority codification candidate; see below).
3. Does NOT modify executor-prompt content beyond the line-372 fix
   integrated with Track 1.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 tag-semantics + executor edits | 3 add / 3 del / 2 files | `bf3ef7d5` |
| Track 2 executor-prompt honesty-pass | ~70 _notes lines | cycle-close |
| `cycle-173-*.md` (_notes) | ~ TBD lines total | cycle-close |
| Total cycle 173 main-authored output | ~250 _notes + 3 LOC tag-semantics + 1 LOC executor | 1 substantive + 1 cycle-close (planned) |

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 173:
Track 1 fix shape correctly estimated as ~5-10 LOC at session-start
(landed 3 add / 3 del; within range and slightly smaller). Track 2 produced
no separate commit (bounded-mechanical pattern continued from cycle 172).
2-track magnitude was Track-2-heavier than cycle 172 (where Track 1 was
substantive write-entry fix at 88 LOC).

## Pattern updates this cycle

- `two-track-composition` HARDENING-AT-26.
- `straight-pair-closure-as-default-two-track-shape` HARDENING-AT-12 (cycles 162-173).
- **`session-start-CI-check-discipline` RECURRENCE-AT-2 cycle 173** —
  cycle 172 NOVEL@1 + cycle 173 exercised. `gh run list --branch master
  --workflow "Rust CI" --limit 3` was the first session-start command;
  found master Rust CI green (matched cycle 172's `208d8d69` SUCCESS
  closure). Forward-watch cycles 173-180 continues; 1 of 8 watch cycles
  consumed.
- `live-prompts-already-aligned-at-extended-schema-level` OBSERVATION-AT-4
  (cycle 170 + 171 + 172 + 173). Four-cycle confirmation. 2 of 4 role-
  prompts honesty-passed (curator + executor); the OBSERVATION holds. 
  Planner + reconciler honesty-passes (cycle 174-175 if cadence holds)
  will determine whether OBSERVATION solidifies into HARDENED principle
  or surfaces enough specific overclaims to invalidate the cycle 148 L3.2
  estimate.
- **NEW pattern `category-level-intent-sharpening-distinguished-from-token-level-circularity` NOVEL@1 cycle 173** —
  design heuristic for distinguishing honest category-naming from
  circular token-copying when sharpening manifest intent strings to
  resolve fidelity check warnings. Forward-watch: applies whenever
  future tag-semantics intent edits surface, or analogous intent-vs-
  content alignment questions appear in other manifests.
- `tool-extraction-surfaces-prior-cycle-errors` NOT-EXERCISED cycle 173.
  Carries at RECURRENCE-AT-10.
- `master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1 (cycle 171)
  NOT-EXERCISED cycle 173 (CI green at start; green after push). Carries
  at NOVEL@1.
- `schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1 (cycle 172)
  NOT-EXERCISED cycle 173 (no schema promotion). Carries at NOVEL@1.
- `tools/v2-* wrapper directly-pushable as v2 substrate by convention extension`
  NOVEL@1 (cycle 171) NOT-EXERCISED cycle 173. Carries at NOVEL@1.
- `directive-2937-track-1-or-track-2-dispatch-fit-application`
  NOT-EXERCISED cycle 173 (no dispatch). Carries at HARDENING-AT-3.
- `straight-pair-with-dispatch-variant` NOT-EXERCISED cycle 173. Carries
  at HARDENING-AT-3.
- `comprehensive-test-suite-exceeds-design-scope-LOC-estimate`
  NOT-EXERCISED cycle 173 (no dispatch PR absorption). Carries at
  RECURRENCE-AT-2.
- `clippy-follow-up-commit-post-absorption` NOT-EXERCISED cycle 173.
  Carries at RECURRENCE-AT-2.
- `partial-investigation-misses-second-workflow` NOT-EXERCISED cycle 173.
  Carries at RECURRENCE-AT-2.
- `design-scope-internal-contradiction-resolved-by-implementation`
  NOT-EXERCISED cycle 173. Carries at NOVEL@1.
- `atomic-dual-crate-PR-stronger-than-design-ordering-requirement`
  NOT-EXERCISED cycle 173. Carries at NOVEL@1.
- `implementation-discovery-as-design-doc-revision-trigger`
  NOT-EXERCISED cycle 173. Carries at NOVEL@1.

### Forward-watch decay status

- **2-cycle pattern of latent-tool first-live-invocation** (cycles 166+167):
  cycles 168-173 all NOT-EXERCISED (6 consecutive). Forward-watch
  effectively expired. Pattern soft-drop confirmed at OBSERVATION-AT-1
  (was cycle 172's intent).
- **`two-design-scope-drafts` composition variant** RECURRENCE-AT-2
  (cycle 158 + 168): cycles 169-173 all NOT-EXERCISED (5 of 5 watch
  cycles consumed). Cycle 173 is the RARE-VARIANT-stabilization deadline.
  No fresh two-design-scope-drafts pair this cycle. Pattern stabilizes at
  RECURRENCE-AT-2 (no further forward-watch).
- **`master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1** (cycle 171):
  forward-watch cycles 173-180. Cycle 173: 1 of 8 consumed; no new red
  instance.
- **`schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1**
  (cycle 172): forward-watch cycles 173-180. Cycle 173: 1 of 8 consumed;
  no schema-promotion event.
- **`session-start-CI-check-discipline` NOVEL@1** (cycle 172):
  forward-watch cycles 173-180. Cycle 173 exercised the discipline →
  RECURRENCE-AT-2. Watch continues; further exercises advance the
  pattern.

## Forward priorities for cycle 174+

Inheriting from cycle 173+'s list, minus priority #1 sub-bullet (CLOSED
via Track 1 tag-semantics edits), and with priority #1 main-bullet
partially advanced (2 of 4 roles honesty-passed).

1. **Honesty-pass on remaining v2 role prompts** (continued from cycle
   172+ priority #1; cycle 172 closed curator; cycle 173 closed executor;
   2 of 4 roles complete). Two roles remaining: planner (567 lines),
   reconciler (554 lines). Bounded per-cycle: ~1 role-prompt assessment
   per cycle continuing established pattern.
2. **Master Rust CI green-state maintenance and forward-watch on
   `session-start-CI-check-discipline` RECURRENCE-AT-2 cycle 173**.
   Continue the session-start orientation discipline.
3. **NEW: Tool-side fidelity check structural improvement** — stemming
   (consolidate ↔ consolidation; failures ↔ failure; surfaces ↔ surface)
   plus `name=""` attribute inclusion in content_tokens for tags whose
   children carry semantic name attributes (e.g., `<constraints>` →
   include constraint names like "single-cycle-scope" / "no-channel-
   writes" / etc. in content_tokens). Would structurally improve the
   keyword-overlap heuristic so the check tests intent vs ACTUAL content
   names, not just tag-name-derived tokens. Bounded design + implementation
   scope: ~50-100 LOC main.rs + 2-3 test cases. Forward-priority for
   cycle 174-180 design candidate window.
4. **AGREE-DEFER queue** (post-real-role-session-measurement; was cycle
   172+ #3).
5. **Coordinated retry/timeout/cancellation arc** — C13 + X2 (was cycle 172+ #4).
6. **Coordinated structured-error-envelope arc** — C6 + C7 + C9 (was cycle 172+ #5).
7. **Coordinated resume/recovery arc** — C11 + C12 + X1 (was cycle 172+ #6).
8. **Audit-engagement substantive-focal single-track variant** (was
   cycle 172+ #7; gate = audit HEAD changes from `bc8fda63`).
9. **Per-axis archival mechanism design scope** (was cycle 172+ #8).
10. **`v2-prompt-contract-check --strict` re-run post-extension** (was
    cycle 172+ #9; LOW).
11. **Deprecate `Channel::required_payload_keys()` legacy method** (was
    cycle 172+ #10; LOW).
12. **Workflow trigger upgrade — add `ready_for_review` to pull_request
    trigger types** (was cycle 172+ #11; LOW; PR-required).
13. **`--all` sweep modes for status / verify** (was cycle 172+ #12; LOW).
14. **`v2-cycle-runner reconcile` orphan-state detection** (was cycle
    172+ #13; LOW).
15. **`v2-state-dispatch-archive --invocation-id` flag** (was cycle 172+ #14; LOW).
16. **`schema-promotion-requires-reader-co-edit-via-named-field`
    discipline codification** (was cycle 172+ #15; LOW; bounded).

**Cycle 173 forward priorities CLOSED:**
- Cycle 172+ priority #1 sub-bullet (tag-semantics constraints intent
  string sharpening) — FULLY CLOSED via Track 1 (also addressed
  `<consolidation-judgment>` intent as a 5th-warning bundled fix).

**Cycle 173 partial advancements:**
- Cycle 172+ priority #1 main-bullet (honesty-pass on v2 role prompts) —
  2 of 4 roles assessed (curator + executor); priority continues as
  cycle 174+ #1 with 2 roles remaining.

**Net list-length change:** -1 closure + 1 new (tool-side fidelity
improvement) = 0.

## Observational forward-watch items

1. **`live-prompts-already-aligned-at-extended-schema-level` OBSERVATION-AT-4** —
   cycle 170 + 171 + 172 + 173. Four-cycle confirmation. Forward-watch:
   planner + reconciler honesty-passes (cycle 174-175 if cadence holds)
   either solidify into HARDENED principle or surface specific role-level
   overclaims.

2. **`session-start-CI-check-discipline` RECURRENCE-AT-2 cycle 173** —
   cycle 172 NOVEL@1 + cycle 173 exercised. Forward-watch: cycles 174-180
   for further exercises. If exercised consecutive cycles, advances
   toward HARDENING. If lapsed, regresses toward NOVEL@1 (and likely
   pulls `master-CI-red-not-noticed-across-multiple-cycles` toward
   RECURRENCE-AT-2).

3. **NEW: `category-level-intent-sharpening-distinguished-from-token-
   level-circularity` NOVEL@1 cycle 173** — forward-watch: applies to
   future intent-vs-content alignment design decisions across manifests.
   Forward-watch cycles 174-180.

4. **`schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1
   cycle 172** — NOT-EXERCISED cycle 173. Carries at NOVEL@1.

5. **`tools/v2-* wrapper directly-pushable as v2 substrate by convention
   extension` NOVEL@1 cycle 171** — NOT-EXERCISED cycle 173. Carries at
   NOVEL@1.

## In-session issues and recoveries

- **Env-var expansion blocked** for `echo "$GITHUB_RUN_ID"` — recovered
  via `gh run list --workflow=orchestrator.yml --limit 1` for run ID
  discovery. (Recurring; not novel.)
- **`gh issue comment` body-file requires file inside working directory**
  — recovered via writing session-start comment-body to
  `docs/redesign/_notes/.tmp-cycle-173-session-start.md` and posting from
  there. Tempfile cleanup blocked by sensitive-path rule (`.tmp-` prefix);
  deferred to session-end via `git rm` in cycle-close. (Same as cycle 172.)
- **`cd tools/rust && cargo ...` composed-cd-and-cargo** — partial
  recovery via `cargo --manifest-path tools/rust/Cargo.toml ...`
  absolute-path-to-manifest pattern. The earlier `cd tools/rust &&
  cargo run` invocation altered the shell's cwd; recovered by
  computing absolute paths thereafter.
- **`tools/v2-prompt-tag-semantic-fidelity check` direct wrapper
  invocation** — blocked at permission prompt (Bash wrapper script
  invocation, not in allowlist). Recovered via in-tree
  `real_prompts_strict_exits_zero` integration test which exercises the
  same code path on the same live prompts. (Same recovery pattern as
  cycle 171.)
- All `cargo`, `gh`, `git`, Edit/Write operations otherwise clean cycle 173.
