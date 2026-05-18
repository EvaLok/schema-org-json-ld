# Cycle 174 — stemming + name-attr inclusion in v2-prompt-tag-semantic-fidelity + planner-prompt honesty-pass

**Cycle issue:** [#2989](https://github.com/EvaLok/schema-org-json-ld/issues/2989)
**Session start:** 2026-05-18 07:14 UTC (run id `26019057747`)
**Previous cycle:** [cycle 173 _notes](cycle-173-tag-semantics-intent-sharpening-and-executor-honesty-pass.md)
**Track 1 commit:** `10fbaf4c` (202 add / 8 del / 1 file)

## Summary

Two-track composition cycle 174 advanced cycle 173+ priority #3 (tool-side
fidelity check structural improvement) via Track 1 substantive and cycle 173+
priority #1 (v2 role-prompts honesty-pass) via Track 2 bounded-mechanical.
23rd consecutive two-track-composition post cycle 151 exception
(`two-track-composition` HARDENING-AT-27 candidate). 59th consecutive
HONORING of named forward priority.
`straight-pair-closure-as-default-two-track-shape` HARDENS at 13 (cycles
162-174).

Track 1 implements two structural improvements to the keyword-overlap
heuristic — stemming and `name=""` attribute inclusion — that the cycle
173 `consolidate ↔ consolidation` and `surfaces ↔ surface` near-misses
motivated. Production code ~50 LOC; tests ~100 LOC; total 202 add / 8 del.
Real-prompts checks remain at `tier1_errors=0, tier1_passes=47,
tier2_warnings=0` post-edit — the cycle 173 intent-string edits remain
valid and the heuristic is strictly more permissive on honest content.

Track 2 reads planner-prompt.xml (567 lines) end-to-end. Findings: all
4 referenced v2 tools exist; `v2-cycle-1-minimal` versioning is honest;
no cargo-culted overclaims in content sections; no stale runtime
references to fix; cross-role constraint category mapping confirmed for
planner (8 constraints, all map to the 5 categories in the new cycle 173
`[tags.constraints]` intent). 3 of 4 v2 role prompts now
honesty-passed; reconciler remains for cycle 175.

`live-prompts-already-aligned-at-extended-schema-level` OBSERVATION
extends to AT-5 (cycles 170+171+172+173+174 — five-cycle confirmation).

`session-start-CI-check-discipline` exercised cycle 174 via
`gh run list --branch master --workflow "Rust CI" --limit 3` at session
start, confirming master Rust CI green on `a63cc591` (cycle 173 _notes
commit). RECURRENCE-AT-2 → RECURRENCE-AT-3.

## Track 1 — stemming + name-attr inclusion (priority #3 advanced; commit `10fbaf4c`)

### Motivation (from cycle 173 _notes priority #3)

Cycle 173 closed priority #1 sub-bullet (tag-semantics intent sharpening,
5 → 0 Tier-2 warnings) via intent-string category-level edits. Cycle 173
also identified that the underlying keyword-overlap heuristic was
mechanically thin in two ways:

1. **No stemming.** Pairs like `consolidate ↔ consolidation`,
   `surfaces ↔ surface`, `failures ↔ failure` are semantically equivalent
   but failed to overlap because the tokenizer used exact string match.
   The cycle 173 `<consolidation-judgment>` intent edit had to manually
   include both the noun ("consolidation") and the cycle-data preservation
   verb ("preserve") just to get a single token to land.

2. **No `name=""` attribute participation.** For tags like
   `<constraints>` with 11 `<constraint name="single-cycle-scope">`
   children, `content_tokens` was just `{constraint, constraints}` —
   the concrete domain vocabulary in the `name` attributes
   (`single-cycle-scope`, `no-channel-writes`, etc.) was invisible to
   the heuristic. Cycle 173 had to enumerate those categories in the
   intent string manually to score overlap.

Both gaps mean the cycle 173 intent edits had to do work the tool should
do. Cycle 174 priority #3 fixes the tool.

### Design

**`stem()` function** — conservative suffix-stripping (not full Porter).
Suffixes (longest-first): `ations`, `ation`, `tions`, `tion`, `ities`,
`ity`, `ates`, `ate`, `ings`, `ing`, `s`. Minimum-stem-length guard:
suffix length + 4 chars, so short tokens are never stripped (`its`,
`yes` stay intact). The suffix list is restricted to forms observed in
v2-prompt manifests and prompt-content vocabulary; y/ies handling
omitted (would need irregular-noun logic).

Verifying the named pairs:
- `consolidate` (11 chars) ends `ate` (3+4=7 ≤ 11) → strip → `consolid`
- `consolidation` (13 chars) ends `ation` (5+4=9 ≤ 13) → strip → `consolid` ✓
- `surfaces` (8 chars) ends `s` (1+4=5 ≤ 8) → strip → `surface`
- `surface` (7 chars) does not end any suffix → `surface` ✓
- `failures` (8 chars) ends `s` → strip → `failure`
- `failure` (7 chars) does not end any suffix → `failure` ✓
- `validity` (8 chars) ends `ity` (3+4=7 ≤ 8) → strip → `valid`
- `valid` (5 chars) does not end any suffix → `valid` ✓

The order matters: `ations` is tried before `ation` so
"consolidations" stems cleanly to `consolid`. `ates` is tried before
`ate` so "consolidates" stems to `consolid` not `consolida-tes`.

**`direct_child_name_attrs: Vec<String>` field** on `ParsedTag` —
collected at depth 2 in `parse_prompt_file` from any `name="..."`
attribute on direct child elements. Not deduped (each instance
contributes its tokens). For `<constraints>` with 11 children, the
field carries all 11 attribute values.

**`run_tier2` keyword-overlap change** — both intent_tokens and
content_tokens are now stem-collected before overlap. Content string
gains a third component: name-attr values joined kebab→space (same
treatment as the tag name).

New content_tokens for `<constraints>` example:
- Tag name → `{constraints}` → stem → `{constraint}`
- Direct children → `{constraint}` → stem → `{constraint}` (deduped to set)
- Name attrs (e.g., `single-cycle-scope, no-channel-writes,
  no-self-modification, payload-validation, reducer-rule-discipline,
  ...`) → split on `-` and space → `{single, cycle, scope, channel,
  writes, self, modification, payload, validation, reducer, rule,
  discipline, ...}` → stems

The new content_tokens have 15-25 tokens depending on the constraint
set, vs ~2 with the old tokenizer. The keyword-overlap heuristic now
tests intent vs ACTUAL content names — the structural improvement the
cycle 173 priority #3 named.

### Production code summary

| Change | Lines | Location |
|---|---|---|
| `direct_child_name_attrs: Vec<String>` field on ParsedTag | +8 (incl. doc-comment) | main.rs:188-198 |
| Field initializer at depth 1 (top-level tag entry) | +1 | main.rs:275 |
| Name-attr collection at depth 2 | +9 | main.rs:288-296 |
| `stem()` function + SUFFIXES const + doc-comment | +20 | main.rs after `tokenize()` |
| `run_tier2` keyword-overlap rewrite (intent_stems + content_stems + new format string with name_attrs_joined) | +18 / -10 | main.rs run_tier2 |
| 2× test-helper field defaults (parsed_tag, make_tag_for_t2) | +2 | main.rs:1096, 1200 |
| **3 new tests** | +120 | main.rs after tokenize_filters test |

Total: **+202 / -8**. Within priority #3 bounded estimate (~50-100 LOC
"main.rs + 2-3 test cases") — production code is ~50 LOC, tests are
~100 LOC, comments + field-default + doc-comments are the remaining ~50.
Cycle 173's "comprehensive-test-suite-exceeds-design-scope-LOC-estimate"
pattern applies in a minor way here (tests slightly over the LOC
estimate); RECURRENCE-AT-3 candidate (cycle 170 PR #2979, cycle 171
PR #2985 were the prior 2-3× cases; cycle 174 is a smaller 2× case).

### Test coverage

Three new unit tests (22 → 25 unit tests in the binary):

1. **`stem_collapses_verb_noun_and_plural_singular_pairs`** — direct unit
   test of `stem()` on all 4 named pairs + min-length guard + ne-check
   against unrelated words.

2. **`parse_collects_direct_child_name_attrs`** — parser-level via
   tempfile XML synthetic: `<constraints>` with 2 `<constraint
   name="...">` children; assert that `direct_child_name_attrs` contains
   both values in document order. Exercises the depth-2 attribute-collection
   path concretely.

3. **`tier2_keyword_overlap_uses_child_name_attrs_and_stemming`** —
   behavioral test with two scenarios:
   a. `<consolidation-judgment>` with intent "Guides the curator on how to
      consolidate cycle data" — stemming alone produces `consolid`
      overlap between intent (`consolidate` → `consolid`) and tag name
      (`consolidation` → `consolid`); no Tier-2 warning expected.
   b. `<constraints>` with intent "Declares the runtime obligations
      around channel discipline" and name-attrs
      `[no-channel-writes, single-cycle-scope]` — name-attr inclusion
      produces `channel` overlap; no Tier-2 warning expected.

Integration tests unchanged in count (8 + 3 = 11). Full crate: 36 tests
pass.

### Live-prompts verification

`integration_real_prompts::real_prompts_tier2_always_exit_zero`:
- Tier 1: 47 passes / 0 errors (unchanged from cycle 173)
- Tier 2: 0 warnings (unchanged from cycle 173)
- exit_code: 0

The structural improvements are strictly more permissive on honest
content — they make false-positive warnings less likely without changing
true-drift detection. Live prompts that passed under the cycle 173
edited intents continue to pass under the new heuristic.

`integration_real_prompts::real_prompts_strict_exits_zero`: passes
(extended-schema strict mode).

`integration_fidelity::*` (8 tests): all pass. The 8 Tier-2 warnings
they emit are pre-existing fixture artifacts; those tests assert on
exit_code, not warning counts.

### Clippy + full workspace

- `cargo clippy -p v2-prompt-tag-semantic-fidelity --tests -- -D warnings`: clean
- `cargo test --workspace --release`: all 49+ test suites ok, 0 failures
- Build time: incremental.

## Track 2 — planner-prompt.xml honesty-pass (priority #1 partial advancement)

### Scope

Continues the cycle 172 (curator) + cycle 173 (executor) per-cycle
honesty-pass pattern: read role-prompt end-to-end + assess for cargo-
culted overclaims, stale runtime references, target-state-as-current
overclaim, tool existence + harness/prompt consistency. Cycle 174 reads
planner-prompt.xml (567 lines). 3 of 4 role prompts now honesty-passed;
reconciler (554 lines) is the last unread role-prompt for cycle 175.

### Findings

1. **All 4 referenced v2 tools exist** (confirmed via filesystem):
   - `tools/rust/crates/v2-role-driver/` — exists.
   - `tools/rust/crates/v2-channel-router/` — exists. Verified that
     `Channel::PlanChannel => Role::Planner` (v2-channel-router/src/main.rs:263)
     matches planner-prompt line 482's "plan-channel reducer rule is
     planner-writes-only" claim. Tool-prompt agreement is concrete,
     not just textual.
   - `tools/rust/crates/v2-super-step-boundary/` — exists.
   - `tools/dispatch-task` — exists (referenced in example payload at
     line 178; confirmed by cycle 173 _notes).

2. **`v2-cycle-1-minimal` versioning is honest target-state-as-target**
   — same disposition as curator (cycle 172) + executor (cycle 173).
   Three-cycle confirmation; planner does not overclaim cycle-1
   capabilities as currently shipped.

3. **No cargo-culted overclaims in content sections.** Read end-to-end:
   `<role-identity>` (lines 19-41), `<inputs>` (lines 46-122),
   `<output-contract>` (lines 127-209), `<substantive-focal-judgment>`
   (lines 214-298), `<per-role-tasks-decomposition>` (lines 303-363),
   `<tools>` (lines 368-409), `<communication>` (lines 414-434),
   `<constraints>` (lines 439-496), `<session-structure>` (lines
   501-535), `<meta>` (lines 540-565). Line 13 ("Axis 13 commitment"),
   line 86 ("Axis 12"), line 546 ("Axis 13 commitment") reference design
   candidate B body axis numbering — these are documentation cross-refs
   to the chosen design candidate, not overclaims about shipped
   behavior.

4. **No stale runtime references to fix.** Two appearances of "Cycle
   144" survive in the prompt:
   - Line 175 example payload: `"substantive-focal": "Cycle 144 —
     author planner-prompt.xml ..."`. Per cycle 173 disposition,
     example-payload data is historically anchored, not runtime-gated.
     KEEP.
   - Line 519 reference: `docs/redesign/_notes/cycle-144-directive-
     2937-absorption.md` (forward-pointer to context for the
     dispatch-fit decision). File exists; reference is valid. KEEP.
   - Line 550-556 `<evolution-path>`: "Cycle 144 is the FIRST authoring
     of this prompt. Cycle 145+ review..." — historical-context block,
     same disposition as cycle 173's executor `<evolution-path>` KEEP.
   - Line 559: "Cycle 144 authoring designates this as the REFERENCE
     prompt" — `<reference-status>` historical block. KEEP.

   File reference verification:
   - `docs/redesign/_notes/cycle-144-directive-2937-absorption.md`: exists.
   - `docs/redesign/2-candidates/B-decomposed-multi-role.md` (line 179
     example payload): exists.

5. **Cross-role constraint category mapping confirmed for planner.**
   The new cycle 173 `[tags.constraints]` intent enumerates 5 categories
   across the union of all 4 role-prompts: scope, channel reducer
   discipline, self-modification prohibition, payload validity,
   commit-push discipline. Planner has 8 constraints (lines 440-495);
   each maps to one of the 5 categories:

   | Planner constraint | Category |
   |---|---|
   | `single-cycle-scope` | SCOPE |
   | `no-channel-writes` | CHANNEL REDUCER DISCIPLINE |
   | `no-cross-channel-reads` | CHANNEL REDUCER DISCIPLINE |
   | `no-skill-execution` | SCOPE (cycle-1-minimal axis deferral) |
   | `no-branching` | SCOPE (cycle-1-minimal axis deferral) |
   | `no-self-modification` | SELF-MODIFICATION PROHIBITION |
   | `reducer-rule-discipline` | CHANNEL REDUCER DISCIPLINE |
   | `payload-validation` | PAYLOAD VALIDITY |

   No commit-push discipline constraint on planner specifically (that
   category is more of an executor-level concern; the cycle 173 intent
   correctly enumerates the UNION across all 4 roles, not just
   per-role). 8 of 8 planner constraints map cleanly. The new intent
   is honest for planner: all 5 named categories find at least one
   constraint instance, and all planner constraints map to a named
   category.

6. **Harness/prompt consistency check passed.** Plan-channel
   `REQUIRED_NAMES_PLAN_CHANNEL` in v2-channel-router/src/main.rs:231
   is exactly `&["substantive-focal", "per-role-tasks"]` — matches
   planner-prompt's `<output-contract>` required keys (lines 138-159).
   The harness validation referenced at line 202 ("v2-channel-router
   will REJECT your output if it doesn't contain the required keys
   substantive-focal + per-role-tasks") is true as-stated, with the
   exact keys named.

### Track 2 produced artifacts

- 0 standalone prompt-content edits (planner is clean post-cycle 173
  improvements; no inline cleanups surfaced).
- 0 standalone commits (findings land in cycle-close commit).
- 1 honesty-assessment artifact (this _notes section).

### OBSERVATION extension

`live-prompts-already-aligned-at-extended-schema-level` OBSERVATION
extends to AT-5 (cycles 170+171+172+173+174 — five-cycle confirmation).
Forward-watch: reconciler honesty-pass (cycle 175) is the last unread
role-prompt; if reconciler is also clean, the OBSERVATION solidifies
into HARDENED principle. If reconciler surfaces stale references or
overclaims, the OBSERVATION holds but doesn't HARDEN.

## Substantive measurements

| Artifact | Size | Commit |
|---|---|---|
| Track 1 v2-prompt-tag-semantic-fidelity changes | 202 add / 8 del / 1 file | `10fbaf4c` |
| Track 2 planner-prompt honesty-pass | ~110 _notes lines | cycle-close |
| `cycle-174-*.md` (_notes) | ~ TBD lines | cycle-close |
| Total cycle 174 main-authored output | ~ TBD _notes + 194 net LOC | 1 substantive + 1 cycle-close (planned) |

`magnitude-prediction-precision-is-shape-dependent-not-flat` cycle 174:
Track 1 production-code LOC ~50 (within priority #3 estimate
"~50-100 LOC main.rs"). Track 1 test LOC ~100 (priority #3 estimate
"2-3 test cases" was conservative — 3 cases × ~30-40 lines each is
~100 LOC, matches actual).
`comprehensive-test-suite-exceeds-design-scope-LOC-estimate`
RECURRENCE-AT-3 candidate — tests are 2× the per-case minimum but
appropriate for the 2-improvement scope.

## Pattern updates this cycle

- `two-track-composition` HARDENING-AT-27 (cycles 152-174, 23
  consecutive).
- `straight-pair-closure-as-default-two-track-shape` HARDENING-AT-13
  (cycles 162-174).
- **`session-start-CI-check-discipline` RECURRENCE-AT-3 cycle 174** —
  cycle 172 NOVEL@1 + cycle 173 RECURRENCE-AT-2 + cycle 174 exercised.
  `gh run list --branch master --workflow "Rust CI" --limit 3` was the
  first session-start CI command. Forward-watch cycles 175-180 continues.
- **`category-level-intent-sharpening-distinguished-from-token-level-circularity`
  NOVEL@1 cycle 173 → RECURRENCE-AT-2 cycle 174 (tool-side application)** —
  cycle 173 named the pattern at the intent-string level; cycle 174
  Track 1 applies it at the tool level: the new heuristic compares
  intent vocabulary to actual content-name vocabulary (the categorical
  improvement), not just tag-name-derived tokens (the circular
  no-op).
- `live-prompts-already-aligned-at-extended-schema-level` OBSERVATION-AT-5
  (cycle 170+171+172+173+174). Five-cycle confirmation.
- `tool-extraction-surfaces-prior-cycle-errors` NOT-EXERCISED cycle 174.
  Carries at RECURRENCE-AT-10.
- `master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1
  NOT-EXERCISED cycle 174. Carries at NOVEL@1. Forward-watch decay
  continues: 2 of 8 cycles consumed; no new red instance.
- `schema-promotion-requires-reader-co-edit-via-named-field` NOVEL@1
  NOT-EXERCISED cycle 174. Carries at NOVEL@1. 2 of 8 forward-watch
  consumed.
- `tools/v2-* wrapper directly-pushable as v2 substrate by convention
  extension` NOVEL@1 NOT-EXERCISED cycle 174. Carries at NOVEL@1.
- `directive-2937-track-1-or-track-2-dispatch-fit-application`
  NOT-EXERCISED cycle 174. Carries at HARDENING-AT-3.
- `straight-pair-with-dispatch-variant` NOT-EXERCISED cycle 174.
  Carries at HARDENING-AT-3.
- `comprehensive-test-suite-exceeds-design-scope-LOC-estimate`
  RECURRENCE-AT-3 candidate (cycle 170 PR #2979 + cycle 171 PR #2985 +
  cycle 174 Track 1) — minor case for cycle 174 (~2× test LOC, not
  the 2-3× production-LOC pattern of prior cases). Counts as a related
  observation, not a clean recurrence; treat as RECURRENCE-AT-2 +
  cycle 174 OBSERVATION.
- `clippy-follow-up-commit-post-absorption` NOT-EXERCISED cycle 174
  (no absorption cycle). Carries at RECURRENCE-AT-2.
- `partial-investigation-misses-second-workflow` NOT-EXERCISED cycle
  174. Carries at RECURRENCE-AT-2.
- `design-scope-internal-contradiction-resolved-by-implementation`
  NOT-EXERCISED cycle 174. Carries at NOVEL@1.
- `atomic-dual-crate-PR-stronger-than-design-ordering-requirement`
  NOT-EXERCISED cycle 174. Carries at NOVEL@1.
- `implementation-discovery-as-design-doc-revision-trigger`
  NOT-EXERCISED cycle 174. Carries at NOVEL@1.

### Forward-watch decay status

- **`master-CI-red-not-noticed-across-multiple-cycles` NOVEL@1**:
  forward-watch cycles 173-180; 2 of 8 consumed; no new red instance.
- **`schema-promotion-requires-reader-co-edit-via-named-field`
  NOVEL@1**: forward-watch cycles 173-180; 2 of 8 consumed; no
  schema-promotion event.
- **`session-start-CI-check-discipline` NOVEL@1 → RECURRENCE-AT-3**:
  forward-watch continues; cycle 174 exercised → advances.
- **`category-level-intent-sharpening-distinguished-from-token-level-circularity`
  NOVEL@1 → RECURRENCE-AT-2 (tool-side application)**: forward-watch
  cycles 175-180 for further variants (manifest-side, role-prompt-side,
  or tool-side).
- **`tools/v2-* wrapper directly-pushable as v2 substrate by
  convention extension` NOVEL@1** (cycle 171): forward-watch
  effectively expired; 3+ cycles NOT-EXERCISED; pattern soft-drops to
  OBSERVATION-AT-1.

## Forward priorities for cycle 175+

Inheriting from cycle 174+'s list, minus priorities #1 (advanced
3 of 4 roles) and #3 (advanced via Track 1 commit; closed for now —
no remaining cycle-173-named structural improvement scope).

1. **Honesty-pass on reconciler-prompt.xml** (continued from cycle
   172+ priority #1; cycle 172 closed curator, cycle 173 closed
   executor, cycle 174 closed planner; reconciler is the last unread
   role-prompt at 554 lines). Bounded per-cycle pattern. Completion
   of 4 of 4 honesty-passes SOLIDIFIES
   `live-prompts-already-aligned-at-extended-schema-level` toward
   HARDENED principle.
2. **Master Rust CI green-state maintenance + `session-start-CI-check-discipline`
   RECURRENCE-AT-3 forward-watch**. Continue session-start
   orientation discipline.
3. **AGREE-DEFER queue** (post-real-role-session-measurement; was
   cycle 173+ #4).
4. **Coordinated retry/timeout/cancellation arc** — C13 + X2 (was
   cycle 173+ #5).
5. **Coordinated structured-error-envelope arc** — C6 + C7 + C9
   (was cycle 173+ #6).
6. **Coordinated resume/recovery arc** — C11 + C12 + X1 (was cycle
   173+ #7).
7. **Audit-engagement substantive-focal single-track variant** (was
   cycle 173+ #8; gate = audit HEAD changes).
8. **Per-axis archival mechanism design scope** (was cycle 173+ #9).
9. **`v2-prompt-contract-check --strict` re-run post-extension**
   (was cycle 173+ #10; LOW).
10. **Deprecate `Channel::required_payload_keys()` legacy method**
    (was cycle 173+ #11; LOW).
11. **Workflow trigger upgrade — add `ready_for_review` to
    pull_request trigger types** (was cycle 173+ #12; LOW; PR-required).
12. **`--all` sweep modes for status / verify** (was cycle 173+ #13;
    LOW).
13. **`v2-cycle-runner reconcile` orphan-state detection** (was
    cycle 173+ #14; LOW).
14. **`v2-state-dispatch-archive --invocation-id` flag** (was cycle
    173+ #15; LOW).
15. **`schema-promotion-requires-reader-co-edit-via-named-field`
    discipline codification** (was cycle 173+ #16; LOW; bounded).

**Cycle 174 forward priorities CLOSED:**
- Cycle 173+ priority #3 (tool-side fidelity check structural
  improvement) — CLOSED via Track 1 commit `10fbaf4c`. Stemming +
  name-attr inclusion shipped, tested, clippy-clean.

**Cycle 174 partial advancements:**
- Cycle 173+ priority #1 (honesty-pass on v2 role prompts) — 3 of
  4 roles assessed (curator + executor + planner); priority continues
  as cycle 175+ #1 with 1 role remaining (reconciler).

**Net list-length change:** -1 closure + 0 new = -1.

## Observational forward-watch items

1. **`live-prompts-already-aligned-at-extended-schema-level`
   OBSERVATION-AT-5** — cycle 170+171+172+173+174. Five-cycle
   confirmation. Forward-watch: reconciler honesty-pass (cycle 175)
   either solidifies the OBSERVATION into HARDENED principle (4 of
   4 roles clean) or surfaces specific role-level overclaims
   (OBSERVATION holds but does not HARDEN).

2. **`session-start-CI-check-discipline` RECURRENCE-AT-3 cycle 174** —
   cycle 172 NOVEL@1 + cycle 173 RECURRENCE-AT-2 + cycle 174
   RECURRENCE-AT-3. Forward-watch: cycles 175-180 for further
   exercises. If consecutively exercised cycles 175-180, advances
   toward HARDENING-AT-N. If lapsed at any cycle, regresses to
   RECURRENCE-AT-3 (and likely back to NOVEL@1).

3. **`category-level-intent-sharpening-distinguished-from-token-level-circularity`
   RECURRENCE-AT-2 (tool-side application)** — cycle 173 NOVEL@1
   (intent-string level) + cycle 174 RECURRENCE (tool level). Pattern
   has now exercised at two levels of the same axis (manifest content
   vs heuristic algorithm). Forward-watch cycles 175-180 for a third
   variant.

## In-session issues and recoveries

- **Env-var expansion blocked** (`echo "$GITHUB_RUN_ID"`). Recovered
  via `gh run list --workflow=orchestrator.yml --limit 1` for run ID.
- **`gh issue comment` body-file requires file inside working
  directory**. Recovered via writing to
  `docs/redesign/_notes/.tmp-cycle-174-session-start.md`; tempfile
  cleanup via session-close `git rm`.
- **`cargo --manifest-path` flag-order blocked** (cargo expects
  subcommand before --manifest-path). Recovered via
  `cargo test --manifest-path <path> -p <pkg>` ordering. Pattern
  worth noting: `cargo SUBCOMMAND --manifest-path X` is correct;
  `cargo --manifest-path X SUBCOMMAND` is rejected.
- **Permission prompts avoided cycle 174**: no `tools/v2-*` wrapper
  invocation attempted; verification via in-tree integration tests
  (same pattern as cycle 171-173).
- All `cargo`, `gh`, `git`, Edit/Write operations otherwise clean
  cycle 174.
