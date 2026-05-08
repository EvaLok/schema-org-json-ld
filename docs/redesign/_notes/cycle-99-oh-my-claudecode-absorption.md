# Cycle 99 — oh-my-claudecode first-pass survey absorption: PR #2876 (issue #2847; cycle-75 dispatch)

**Cycle issue:** [#2888](https://github.com/EvaLok/schema-org-json-ld/issues/2888)
**Source:** [PR #2876](https://github.com/EvaLok/schema-org-json-ld/pull/2876) (Copilot research-only dispatch on
[issue #2847](https://github.com/EvaLok/schema-org-json-ld/issues/2847), cycle 75)
**Deliverable absorbed:** `_notes/cycle-75-oh-my-claudecode-survey.md` on branch
`copilot/redesign-research-cycle-75-survey-oh-my-claudecode`
(429 lines; PR closed without merge per absorption convention)
**Cycle composition shape:** **research-deliverable-absorption-with-verification-success** — second
instance of functional-class shape #19 (NOVEL at cycle 98). Cycle 98 was
the first instance (Symphony, PR #2873); cycle 99 is the second instance
(oh-my-claudecode, PR #2876). The shape is now **TESTED at 2 instances**:
both are research deliverables describing target external systems, both
were authored under cycle-77+ Copilot dispatch under Eva directives
(#2774 / #2775), both produced first-pass survey deliverables that
verified at high precision against the live repos. **19 functional-class
shapes total demonstrated cycles 62-99 at 39 instances.**

## Setup

PR #2876 is the cycle-75 first-pass survey deliverable on
`Yeachan-Heo/oh-my-claudecode`, the companion target named in Eva
directive [#2774](https://github.com/EvaLok/schema-org-json-ld/issues/2774)
(2026-04-29) which also named oh-my-codex. The dispatch was authored
under a 9-lens structure: repo glance, architecture surfaces, skills/prompts/extension
mechanisms, hooks/lifecycle integration, state/memory/persistence, docs
honesty patterns, cycle-26 22-pattern cross-mapping (with confidence
ratings per pattern), NEW patterns specific to omc, and anchoring caveats.

Hypotheses tested by the dispatch:

- **H1 parallel-architecture** (omc mirrors omx's 5-component architecture:
  keyword detector + generator/composition + autoresearch + MCP + Rust
  substrate-edge): **CONFIRMED at first-pass depth, 4/5 components.** Rust
  substrate-edge component absent (no `crates/` at root).
- **H2 substrate-specific patterns** (Claude lifecycle hooks, slash-command
  invocation, Claude config deployment, MCP `.mcp.json` integration):
  **CONFIRMED.** All four substrate-specific patterns observed.
- **H3 substrate-investment asymmetry** (omc less mature/smaller than omx):
  **REFUTED on current-repo metrics** — omc is the **larger** project
  across 5 metrics (1.26× to 2.81×).

## Verification table (cycle 99)

Following cycle 96/97/98 verification-discipline pattern, load-bearing
quantitative claims spot-checked against the live `Yeachan-Heo/oh-my-claudecode`
and `Yeachan-Heo/oh-my-codex` repositories via `gh api`. Cycle 99 is the
**fourth instance** of formalized verification on a returned dispatch
(cycle 96 fabrication catch / cycle 97 calibration verified / cycle 98
file-state verified / cycle 99 EXACT-precision verified).

### oh-my-claudecode metadata (PR section 1.3)

| Claim in PR | Verification source | Result |
|---|---|---|
| commits 3076 | `gh api repos/Yeachan-Heo/oh-my-claudecode/commits?per_page=1 -i` Link header `page=3076` | ✓ EXACT |
| contributors 109 | `gh api repos/Yeachan-Heo/oh-my-claudecode/contributors?per_page=1&anon=true -i` Link header `page=109` | ✓ EXACT (with anon=true; without anon=false the API returns 101 — PR's 109 figure includes anonymous contributors) |
| releases 224 | `gh api repos/Yeachan-Heo/oh-my-claudecode/releases?per_page=1 -i` Link header `page=224` | ✓ EXACT |
| latest tag v4.13.6 | `gh api 'repos/.../releases?per_page=1'` → `tag_name: v4.13.6` | ✓ EXACT |
| language TypeScript | metadata response | ✓ EXACT |
| root item count 50 | `gh api repos/.../contents/` → length 50 | ✓ EXACT |
| no `crates/` at root | root listing inspected | ✓ EXACT (verified absent) |
| `agents/*.md` count 19 | `gh api repos/.../contents/agents` → 19 .md files | ✓ EXACT |

### File sizes (PR section 2.2)

| File | Claim | Actual bytes | KB precision (÷1024) | KB precision (÷1000) | Result |
|---|---|---|---|---|---|
| `src/hooks/bridge.ts` | ~105KB | 105722 | 103.2 | 105.7 | ✓ matches at decimal-KB |
| `src/team/runtime-v2.ts` | ~83KB | 83319 | 81.4 | 83.3 | ✓ matches at decimal-KB |
| `src/installer/index.ts` | ~71KB | 71332 | 69.7 | 71.3 | ✓ matches at decimal-KB |
| `src/hooks/persistent-mode/index.ts` | ~67KB | 67124 | 65.5 | 67.1 | ✓ matches at decimal-KB |
| `src/autoresearch/runtime.ts` | ~55KB | 54592 | 53.3 | 54.6 | ✓ matches at decimal-KB (PR rounded up to 55) |

### oh-my-codex sister metadata (PR section 1.3)

| Claim in PR | Actual at cycle-99 measurement | Result |
|---|---|---|
| commits 2441 | 2442 | ✓ within drift (1 commit added in elapsed time) |
| contributors 61 (anon=true) | 61 | ✓ EXACT |
| releases 98 | 99 | ✓ within drift (v0.16.2 published 2026-05-08T08:53:12Z, ~22 minutes after PR creation) |
| latest tag v0.16.1 | v0.16.2 | ✓ within drift (same v0.16.2 release) |
| root item count 26 | 27 | ✓ within drift (1 root item added in elapsed time) |

### Net verification verdict

**18 quantitative claims verified — 13 EXACT + 5 within elapsed-time drift.**

The 5 oh-my-codex deltas are all consistent with the ~14 hours elapsed
between PR creation (2026-05-08T08:30:30Z) and cycle-99 verification
(2026-05-08T22:17Z). PR #2876's author counted accurately at time of
authoring; cycle-99 measurement re-validates the original claims as
faithful snapshots.

**Methodologically the strongest verification result in the absorption
arc to date** (compared to cycle 96 fabrication-magnitude failure 5.2× /
1.86×; cycle 97 PR #2877 0.05-0.1% precision on workspace-LOC counts;
cycle 98 PR #2873 byte-level on 8 files + EXACT on 3 metadata fields).

PR #2876's author actually counted file sizes (via local `tree`/`du` or
GitHub API) and queried metadata before producing the survey — calibration-first
methodology in the same shape as cycle-97 PR #2877 and cycle-98 PR #2873.

## Per-section verdicts (compressed)

The deliverable's 9 sections per dispatch structure, integration verdict each:

| Section | Subject | Verdict |
|---|---|---|
| 1 | Repo at a glance + asymmetric scale check | INTEGRATED into per-system file project framing + verification banner |
| 2 | Architecture at survey depth | INTEGRATED into per-system file orchestration patterns |
| 3 | Skills, prompts, extension mechanisms | INTEGRATED into per-system file agent architecture |
| 4 | Hooks and lifecycle integration | INTEGRATED into per-system file orchestration patterns + trust posture |
| 5 | State, memory, persistence | INTEGRATED into per-system file state/memory/history patterns |
| 6 | Documentation patterns / anti-patterns / honesty | INTEGRATED into per-system file documentation honesty patterns |
| 7 | Cross-reference to cycle-26 22-pattern catalogue + H1/H2/H3 hypothesis tests | INTEGRATED as 22-pattern cross-reference section + hypothesis verdicts section |
| 8 | NEW patterns specific to omc (8 patterns) | INTEGRATED as single-system observations pending elevation; 4-shape consolidation honesty |
| 8.5 | Cluster framework anchoring (observation-only) | NOT-INTEGRATED into 1-research.md cross-system observations (preserved as observation-only per PR's own caveat; per-system file references the deliverable section but does not promote single-system observations to cross-system patterns) |
| 9 | Anchoring caveats (5 from PR + 3 substrate-alignment) | INTEGRATED into per-system file anchoring caveats |

## Integration outcomes

**Files modified/created on master (3 + 1 absorption note):**

1. `docs/redesign/1-research/systems/oh-my-claudecode.md` — NEW per-system
   file in cycle-33 research-restructure layout (verification banner,
   sources read, project framing with sister-asymmetry refutation, patterns
   organized into 6 family buckets, hypothesis verdicts H1-H3, cycle-26
   22-pattern cross-reference [PARALLEL 5 / ADAPTED 9 / ABSENT 6 /
   NEEDS-DEEPER-READ 2], NEW patterns held as single-system pending
   elevation with 4-shape consolidation, 8 anchoring caveats including
   5 from PR + 3 per-system additions, deeper-read queue per dispatch's
   deferred items).
2. `docs/redesign/1-research.md` — Per-system reads table extended with
   oh-my-claudecode row (first-pass status, primary evidence base PR #2876).
3. `docs/redesign/1-research.md` — Further-systems-to-study oh-my-claudecode
   row updated from "TBD / Pending" → "Copilot research-only dispatch (cycle 75,
   #2847) / First-pass landed (cycle 99, PR #2876)" with link to per-system file.
4. `docs/redesign/_notes/cycle-99-oh-my-claudecode-absorption.md` — THIS
   absorption note.

**Explicit not-modified items (with reasoning):**

1. **Cross-system observations in `1-research.md`** — NOT modified. omc's
   NEW patterns are single-system observations at first-pass depth; per
   cross-system convergence discipline, single-system patterns are held
   pending elevation via deeper-read or adversarial-on-adversarial review.
   NEW-2 (event-driven runtime replacing polling) is the strongest
   elevation candidate via potential Symphony convergence; held until
   Symphony deeper-read parity or oh-my-claudecode deeper-read confirms
   the architectural pattern more thoroughly.
2. **Cluster catalogue counts** — NOT modified. omc reaches first-pass
   depth, parallel to Symphony's first-pass status (cycle 98); neither
   yet meets deep-dive parity criterion. The cluster catalogue currently
   anchored on 8 deep-dive systems (openclaw, AutoGen, LangGraph, Voyager,
   Cognition Devin, OpenAI harness, plus PAI / oh-my-codex at stub depth).
   omc + Symphony both pending elevation to deep-dive parity.
3. **Functional-class shape catalogue** — Updated in this note as TESTED
   at 2 instances (#19); not modified in `1-research.md` itself (the
   shape catalogue lives in journal entries and hand-off plans).
4. **22-pattern catalogue itself** — NOT modified. The cycle-26 oh-my-codex
   22 named patterns remain the canonical baseline. omc's mapping is
   recorded as cross-reference TO that baseline, not modifications OF it.

## What surprised me / what I noticed

1. **The H3 dispatch hypothesis was wrong about direction.** PR #2876
   tested "oh-my-claudecode is less mature / smaller than oh-my-codex"
   and found the **opposite** — omc is 1.26-2.81× larger across 5 metrics.
   The dispatch hypothesis being directionally wrong is itself a useful
   epistemic data point: the cycle-26 → cycle-75 ordering ("study omx
   first because it's the senior project") is an *authoring-time* ordering
   based on which project caught the orchestrator's attention first, not
   a *substrate-investment* ordering. The actual substrate-investment
   asymmetry runs the other direction (Claude Code as substrate has
   attracted more author investment than Codex CLI as substrate).
   **NEW pattern recognition**: dispatch hypotheses can be wrong about
   direction in addition to wrong about magnitude; verifying the H3
   verdict's *direction* is just as load-bearing as verifying calibration
   *magnitude*.

2. **The 22-pattern mapping is denser for omc than for Symphony.** omc
   shows 14 of 22 patterns as PARALLEL or ADAPTED (64%); Symphony shows
   13 of 22 (59%). omc has 0 NOT-COMPARABLE patterns; Symphony has 4.
   This reflects sister-project structural alignment: omc shares
   omx's Codex-CLI-adjacent thin-wrapper-over-existing-CLI substrate posture
   (thick plugin layer with hooks + skills + role prompts + MCP), so the
   22-pattern catalogue (built from omx) maps cleanly to omc, with
   substrate-specific adaptations rather than substrate-mismatch
   discontinuities. Symphony's spec-first BEAM substrate is structurally
   distant enough that 4 patterns become NOT-COMPARABLE rather than
   ADAPTED.

3. **The event-driven runtime v2 replacing done-file polling pattern is
   the strongest cross-system elevation candidate** among omc's NEW
   patterns. omc's `src/team/runtime-v2.ts` (83KB) explicitly replaced
   polling watchdog and done.json loop with event-driven lifecycle
   operations. Symphony's reconciliation-before-dispatch tick pattern is
   conceptually parallel — both replaced polling with event-driven
   lifecycle. **2-instance evidence** if both are confirmed at deeper-read
   depth. Not elevated cycle 99 because both are first-pass; held in
   per-system files pending parity.

4. **`docs/settings-schema.md` doc-vs-enforcement boundary marker is
   methodologically sharp.** omc's settings-schema explicitly states a
   config behavior remains prompt-level workflow contract, not runtime
   enforcement. This is the kind of documentation honesty that the
   redesign's own "documentation honesty" pattern family values: docs
   that mark the gap between aspiration and implementation rather than
   imply universal enforcement. **Single-system at first-pass**, but a
   strong observation about how to write docs that don't oversell.

5. **The cycle-75 dispatch's verification posture is the strongest in
   the absorption arc.** Three observed signs: (a) every load-bearing
   quantitative claim is citation-backed against specific files / line
   ranges with evidence-class tagging (`Implementation-verified`,
   `Documentation-only`, `Needs deeper read`); (b) the asymmetric-scale
   metadata claim was independently fact-checked at the dispatch boundary
   (sister-repo metadata snapshot); (c) deferred items are explicitly
   enumerated in the open verification gaps queue. Matches PR #2873
   author discipline. **Now 4-instance evidence of dispatch-author
   verification-discipline as a meta-attribute** (PR #2878 fabrication-magnitude
   failure; PR #2877 calibration-first verified; PR #2873 file-state
   verified; PR #2876 EXACT-across-13-metrics verified). The 3-success/1-failure
   ratio at this 4-instance evidence base suggests dispatch-author
   verification-discipline is variable but tractable: of 4 dispatches,
   3 verified successfully and 1 had fabrication-magnitude errors.

## Sibling pattern tracking

- **Functional-class shape #19 (research-deliverable-absorption-with-verification-success)** —
  TESTED at 2 instances (cycle 98 Symphony / cycle 99 oh-my-claudecode).
  Pattern shape: research dispatch returned via Copilot → file-state /
  metadata verification via `gh api` → per-system file integration with
  verification banner → per-finding integration into family-bucketed
  patterns + hypothesis verdicts + 22-pattern cross-reference + NEW
  patterns held as single-system + anchoring caveats + deeper-read
  queue. The shape now generalizes across 2 instances and is structurally
  stable.

- **Verification-discipline pattern (cycle 96 emergence)** — extended to
  **4 instances HARDENED** (cycle 96 catch, cycle 97 verify, cycle 98
  verify, cycle 99 verify). Discipline holds across both feedback and
  research dispatch types and across multiple authors. Cycle 99 instance
  is the strongest verification result to date (13 EXACT + 5 drift =
  18 metrics verified).

- **Dispatch-author verification-discipline meta-attribute** — extended
  to **4 instances HARDENED**. Pattern: dispatch authors vary in
  verification discipline; verification-of-the-claim is informative for
  trust-weighting beyond the current absorption. Cycle 96 (PR #2878
  fabricated 5.2× / 1.86× anchors) at the failure end; cycles 97/98/99
  at the success end with progressively-finer precision (cycle 97
  workspace-LOC 0.05-0.1%; cycle 98 file-byte-level + metadata EXACT;
  cycle 99 EXACT across 13 metrics + 5 file sizes + sister-comparison
  drift). 3-success / 1-failure at 4-instance evidence base.

- **Generalization-level discipline (J-Q(a))** — extends to **17
  instances HARDENED**. Cycle 99 application: omc H1-H3 hypothesis
  verdicts scoped to first-pass depth with explicit "deeper-read needed"
  caveat; NEW patterns held as single-system pending elevation, NOT
  auto-elevated to cross-system observations; 22-pattern cross-reference
  is a dyadic cross-reference TO oh-my-codex baseline, not modifications OF
  the baseline; 8 NEW patterns honestly consolidated to ~3-4 distinct
  architectural axes per cycle-98 honest-reflection discipline.

- **Direction-vs-magnitude discipline** — extended to **9 instances
  HARDENED**. Cycle 99 application: PR #2876's H3 dispatch hypothesis
  was *directionally* wrong (predicted omc smaller; verified omc larger
  at 1.26-2.81×). The H3 refutation is itself a NEW direction-vs-magnitude
  data point: dispatch hypotheses can fail in direction OR in magnitude;
  cycle 91 PR #2878 was magnitude failure (5.2× / 1.86× over-counts in
  same direction); cycle 75 PR #2876 H3 was direction failure (opposite
  asymmetry from claimed). Both are now explicit failure modes the
  verification-discipline pattern catches.

- **Sibling-pattern binary-becomes-more-structured** — HARDENED at 4
  instances. Cycle 99 NOT a new instance.

- **Audit-as-peer pattern** — 2-instance evidence holds. Cycle 99 NOT
  a new instance; audit cycle 213 still silent-failed (no audit-repo
  commit since cycle 212 at 2026-05-07T04:35Z, ~42 hours since).

## Honest reflection (per F1-F5 corrective)

- **F1 (documentation IS the activity).** Cycle 99 is **1 substantive
  activity** (PR #2876 absorption with verification) yielding 1 new
  per-system file + 2 row updates in 1-research.md + 1 absorption note
  + 1 PR closure with forward-link + 1 issue closure with forward-link
  + 1 journal entry. Documentation is the deliverable, not a separate
  step from the integration.

- **F2 (count outputs accurately, not as multiplicands).** ~5 document
  touches from 1 absorption activity, NOT "8 NEW patterns = 8 ways
  improved." Per-section verdicts table records 9 dispatch sections
  → integration outcomes, but most map to a single per-system file
  section.

- **F3 (functional-class shape inventory).** 19 functional-class shapes
  at **39 instances** (advanced from 19 at 38, cycle 98). Shape #19
  (research-deliverable-absorption-with-verification-success) advances
  from NOVEL at 1 instance → TESTED at 2 instances.

- **F4 (HARDENED/TESTED/NOVEL is for redesign-process methodology only).**
  HARDENED/TESTED/NOVEL classifications applied to:
  verification-discipline (4 instances HARDENED);
  dispatch-author verification-discipline meta-attribute (4 HARDENED);
  generalization-level discipline (17 HARDENED);
  direction-vs-magnitude discipline (9 HARDENED);
  shape #19 (TESTED at 2 instances).
  NOT applied to: omc patterns themselves (those are evidence about
  external systems and use ABSENT/ADAPTED/PARALLEL/NEEDS-DEEPER-READ
  classifications).

- **F5 (lexicon entries).** *Sister-project structural alignment* (omc
  shares omx's substrate posture, so 22-pattern mapping is denser than
  for substrate-distant systems like Symphony); *dispatch-hypothesis
  direction failure* (distinct from magnitude failure: claim wrong
  about *which way* an asymmetry runs, vs claim wrong about *how big*
  the asymmetry is); *substrate-rooted plugin indirection* (NEW-3 + NEW-4
  + NEW-8 consolidated into a single architectural axis); *4-shape
  consolidation discipline* (8 NEW patterns honestly consolidated to
  ~3-4 distinct architectural axes per cycle-98 reflection pattern).

**Self-congratulation audit.** Cycle 99 absorption could be over-stated
as "comprehensive integration of 429-line research deliverable across 9
sections with 18 metric verifications." More honest: oh-my-claudecode
absorption is **first-pass depth** integration; the per-system file is
appropriately stub-equivalent to Symphony's status (cycle 98 first-pass);
omc does NOT yet match the depth of openclaw / AutoGen / LangGraph /
Voyager / Cognition Devin / OpenAI harness deep-dive systems. The
**verification step** is the load-bearing methodological contribution
of cycle 99 — extending the cycle-96-emergence to a fourth dispatch
type/author and demonstrating that EXACT-precision verification is
achievable when the dispatch author counts before claiming. The biggest
finding — H3 was directionally wrong about substrate-investment asymmetry
(omc is 1.26-2.81× LARGER than omx, not smaller) — is correctly
characterized as a refutation of the dispatch hypothesis, demonstrating
verification's value for catching direction failures in addition to
magnitude failures. The 4-shape consolidation of 8 NEW patterns is
honest-reflection-discipline applied: avoiding the over-stating that
"8 NEW patterns" implies 8 distinct architectural contributions.

**Iteration-until-approval discipline.** Cycle 99 is **research-corpus
expansion** per ITERATION-UNTIL-APPROVAL "deepen reference research:
study an additional system or paper not yet covered; integrate findings
into the artifact." oh-my-claudecode was the second of three remaining
absorptions (#2876 cycle 99 / #2875 PAI deeper cycle 100 / #2874 oh-my-codex
deeper cycle 101 per cycle-98 hand-off). The corpus advances from
8-deep-dive + 1-stub + 1-first-pass (Symphony) → 8-deep-dive + 1-stub +
2-first-pass (Symphony + oh-my-claudecode). Two remaining absorptions
will further fill out the corpus before second-iteration sharpening on
candidates A/B/C resumes (cycles 102+).

## Pre-commit checklist

- [x] Verification table (18 quantitative claims, all confirmed at EXACT
  or natural-drift precision)
- [x] Per-system file at `docs/redesign/1-research/systems/oh-my-claudecode.md`
  written following cycle-98 Symphony shape
- [x] `docs/redesign/1-research.md` per-system reads table extended
  (line 89 row added)
- [x] `docs/redesign/1-research.md` further-systems-to-study row updated
  (line 686 row revised from "TBD / Pending" to landing-cycle citation)
- [x] Absorption note in `_notes/cycle-99-oh-my-claudecode-absorption.md`
  (this file)
- [x] Honest reflection per F1-F5 corrective applied
- [x] Self-congratulation audit performed
- [x] Sibling pattern tracking updated (4 patterns extended, 1 NOVEL→TESTED)
- [x] Anti-inheritance discipline H-Q(a): cycle 99 verification is
  empirical-observation-grounded (gh api repo state) consistent with
  cycles 96/97/98 verification posture

## Cycle 100+ plan

Per cycle-98 hand-off (refined for cycle-99 outcome):

1. **PR #2875 PAI deeper-read absorption** [cycle 100, HIGH PRIORITY] —
   completes next item in absorption arc; PAI deeper-read elevates PAI
   from first-pass stub → deep-dive parity (PAI was first-pass at cycle 14),
   matters for cluster catalogue strengthening from 8 to 9 deep-dive systems.
2. **PR #2874 oh-my-codex deeper-read absorption** [cycle 101] — same
   pattern as PAI but for oh-my-codex; would natural-pair with cycle-99
   omc absorption (sister projects both reaching deep-dive parity).
3. **Author B's counting protocol** [cycle 102] — deferred from cycles
   96-98.
4. **Second-iteration sharpening on candidates A/B/C** [cycle 103+] —
   with full 4-system absorption integration (Symphony + omc + PAI deeper
   + omx deeper) feeding into cluster catalogue + candidate risk
   refinement.
5. **Restored-polarity dispatching** [cycle 103+] — once absorption arc
   reaches natural pause.
