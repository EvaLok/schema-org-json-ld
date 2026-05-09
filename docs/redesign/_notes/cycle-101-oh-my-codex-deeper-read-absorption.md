# Cycle 101 — oh-my-codex deeper-read absorption: PR #2874 (issue #2833; cycle-63 dispatch)

**Cycle issue:** [#2890](https://github.com/EvaLok/schema-org-json-ld/issues/2890)
**Source:** [PR #2874](https://github.com/EvaLok/schema-org-json-ld/pull/2874)
(Copilot research-only dispatch on
[issue #2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833),
cycle 63)
**Deliverable absorbed:** `_notes/cycle-63-oh-my-codex-deeper-read.md` on
branch `copilot/redesign-research-cycle-63-deeper-read`
(913 lines; PR closed without merge per absorption convention)
**Cycle composition shape:** **research-deliverable-absorption-with-verification-mixed** —
FOURTH instance of functional-class shape #19 family but with a NEW
sub-variant. Cycle 98 (Symphony, PR #2873) established the shape as
NOVEL with verification-success; cycle 99 (oh-my-claudecode, PR #2876)
extended to TESTED at 2 instances with verification-success; cycle 100
(PAI, PR #2875) extended to HARDENED at 3 instances with one MAGNITUDE-1-OFF
qualifying claim; cycle 101 introduces a **NEW sub-variant** —
verification-mixed (substantial EXACT verification on physical-quantitative
claims combined with a NEW failure mode on code-citation specifics:
*structural-fabrication-with-correct-direction*). **The shape itself
remains HARDENED at 3 verification-success instances; cycle 101 is the
first verification-mixed instance.** **20 functional-class shapes total
demonstrated cycles 62-101 at 41 instances** (shape #19 verification-success
remains at 3 HARDENED; shape #19 verification-mixed sub-variant at NOVEL
1 instance).

## Setup

PR #2874 is the cycle-63 dispatch's deliverable on
`Yeachan-Heo/oh-my-codex` at commit
`d1863f72d303857e331865a863ed7f057dbd2cf6`. The dispatch was authored
as a code-level deeper read superseding the cycle-26 high-level survey
(PR #2784, closed-without-merge per absorption convention; deliverable
preserved on never-merged branch `copilot/redesign-research-phase-1-survey`).
The dispatch covered 9 lenses:

1. Three-file deep dive (keyword-detector.ts, generator.ts, autoresearch/runtime.ts)
2. State model and transitions (workflow-transition.ts, operations.ts, STATE_MODEL.md)
3. Hooks lifecycle end-to-end flow
4. MCP servers — tool surface across 5 servers (correcting cycle-26's 3)
5. Rust crate ecosystem — omx-sparkshell deep dive (correcting cycle-26's "1 crate" to 5)
6. Prompt-guidance contract system — 9 contract arrays + marker pairs
7. Cycle-26 pattern confirmation and corrections (3 corrections + 7 confirmations)
8. New code-level patterns not in cycle 26 (8-9 new patterns)
9. Anchoring caveats and known gaps (14 unread files explicitly enumerated)

Cycle-63 dispatch was NOT authored under H1/H2/H3 hypothesis structure
(unlike cycle-71 PAI / cycle-77 Symphony / cycle-75 oh-my-claudecode).
Instead, the dispatch supersedes-cycle-26 framing. Cycle 101 absorption
verifies the dispatch's quantitative + structural claims, then integrates
the architectural patterns into the per-system file.

## Verification table (cycle 101)

Following cycle 96/97/98/99/100 verification-discipline pattern, load-bearing
quantitative + structural claims spot-checked against the live
`Yeachan-Heo/oh-my-codex` repo at commit `d1863f72` via `gh api`.
Cycle 101 is the **sixth instance** of formalized verification on a
returned dispatch.

### EXACT-or-within-drift verifications (21 claims)

| Class | Subject | Verdict |
|---|---|---|
| Commit ref | claim `d1863f72d303857e331865a863ed7f057dbd2cf6` | ✓ EXACT SHA |
| File size | `src/hooks/keyword-detector.ts` ~45.5 KB | ✓ EXACT (45590 bytes) |
| File size | `src/config/generator.ts` ~50 KB | ✓ EXACT (50358 bytes) |
| File size | `src/autoresearch/runtime.ts` ~45.6 KB | ✓ EXACT (45592 bytes) |
| File size | `src/mcp/state-server.ts` 5.4 KB | ✓ EXACT (5352 bytes) |
| File size | `src/mcp/wiki-server.ts` 8.8 KB | ✓ EXACT (8784 bytes) |
| File size | `src/mcp/trace-server.ts` 10.5 KB | ✓ EXACT (10505 bytes) |
| File size | `src/mcp/memory-server.ts` 15.8 KB | ✓ EXACT (15810 bytes) |
| File size | `templates/AGENTS.md` 24.6 KB | ✓ EXACT (24606 bytes) |
| File size | `crates/omx-sparkshell/src/threshold.rs` 2.2 KB | ✓ EXACT (2151 bytes) |
| File size | `crates/omx-sparkshell/src/exec.rs` 2.8 KB | ✓ EXACT (2819 bytes) |
| MCP server count | claim **5** corrects cycle-26's 3 | ✓ EXACT (state, wiki, trace, memory, code-intel) |
| Rust crate count | claim **5** corrects cycle-26's 1 | ✓ EXACT (omx-explore, omx-mux, omx-runtime-core, omx-runtime, omx-sparkshell) |
| Cargo.toml dep | sparkshell depends only on omx-mux | ✓ EXACT |
| Constant | `DEFAULT_SPARK_MODEL = "gpt-5.3-codex-spark"` | ✓ EXACT |
| Constant | `DEFAULT_STANDARD_MODEL = "gpt-5.4-mini"` | ✓ EXACT |
| Constant | `DEFAULT_SUMMARY_TIMEOUT_MS = 60_000` | ✓ EXACT |
| Constant | `DEFAULT_FRONTIER_MODEL = "gpt-5.5"` (config/models.ts) | ✓ EXACT |
| Constant content | `KEYWORDS_REQUIRING_INTENT` 7 entries | ✓ EXACT |
| Constant content | `should_retry_with_fallback` 9 signals | ✓ EXACT |
| AGENTS.md text | line-132 directive | ✓ EXACT TEXT |

### Structural-fabrication-with-correct-direction (5 claims)

| # | PR claim | Actual | Failure mode |
|---|---|---|---|
| F1 | `KOREAN_IME_MAP: Record<string, string>` constant at ~line 48 | No such constant; inline `text.replace(/ㅕㅣㅈ/g, 'ulw')` inside `normalizeWorkflowKeyboardTypos()` at ~line 535 | **Constant fabricated; line off by ~487** |
| F2 | `EXPLICIT_SKILL_RE = /(?:^\|[^\w])\$(?:(?:oh-my-codex:)?([a-z][a-z0-9-]*))/g` at ~line 160 | No constant; inline regex inside `parseExplicitSkillInvocations()`; actual is `/(?:^\|[^\w])\$(?:(?:oh-my-codex:)?([a-z][a-z0-9-]*))\b/gi` (note `\b` and `gi`) | **Constant fabricated; regex missing `\b` boundary and `i` flag; line off by hundreds** |
| F3 | `AUTO_COMPLETE_TRANSITIONS` shows 6 transitions in PR's table | Actual 7 entries: PR missed `deep-interview->autoresearch` | **Magnitude-1-missing** |
| F4 | `ALLOWED_OVERLAP_PAIRS` claim `ultrawork \| *` is in the set | Actual set has only `'ralph\|team'`; ultrawork-overlaps-everything is a runtime short-circuit in `isAllowedOverlap()`, NOT membership in the set | **Structure-wrong: location of semantic mis-attributed** |
| F5 | Pseudo-code line numbers (e.g. ~line 48, ~line 160, ~line 260, ~line 280, ~line 420, ~line 580) | Most line numbers off by hundreds; actual constants live at very different positions | **Line-number citations approximate to the point of non-utility** |

### Net verification verdict

**26 claims verified — 21 EXACT/within-drift + 5 substantive inaccuracies
clustered as a NEW failure mode.**

The 5 inaccuracies share a common pattern:

- The **architectural prose claim is direction-correct.** Korean IME
  normalization DOES happen; explicit-skill parsing DOES happen; ultrawork
  DOES overlap with everything; auto-complete transitions DO exist;
  `AUTO_COMPLETE_TRANSITIONS` IS a `Set` of `from->to` strings.
- The **code-citation specifics (constant names, regex literals, set
  membership locations, line numbers) are fabricated, mis-located, or
  off-by-one.** Where PR cites `KOREAN_IME_MAP: Record<string, string>`,
  no such constant exists; the actual mechanism is an inline regex.
  Where PR cites `EXPLICIT_SKILL_RE`, no such constant exists; the
  regex is inline inside a function. Where PR cites `AUTO_COMPLETE_TRANSITIONS`
  with 6 entries, the actual set has 7. Where PR cites `ALLOWED_OVERLAP_PAIRS`
  containing ultrawork wildcards, the actual set has one entry and
  ultrawork is handled by short-circuit elsewhere.

This is a **NEW failure mode** in the absorption arc's verification
catalogue:

- **Cycle 96** PR #2878: **direction-accurate / magnitude-FABRICATED**
  (5.2× / 1.86× over-counts on workspace-LOC anchors — claimed
  numbers but didn't count). Quantitative-physical claim was completely
  wrong-magnitude in same direction.
- **Cycle 99** PR #2876 H3: **direction-WRONG** (predicted omc smaller
  than omx; verified omc 1.26-2.81× larger). Hypothesis was directionally
  inverted.
- **Cycle 100** PR #2875: **direction-accurate / magnitude-1-OFF-by-source-vs-runtime**
  (claimed 16 typed memory dirs; 15 source-checked-in + 1 runtime-created).
  Single quantitative claim off-by-one with structural explanation.
- **Cycle 101** PR #2874: **structural-fabrication-with-correct-direction**
  (architectural patterns described in prose are real; code-citation
  specifics — constant names, regex literals, set membership locations,
  line numbers — are fabricated, mis-located, or off-by-one). Multiple
  inaccuracies, all in code-detail-citation rather than architecture-prose.

### Methodological position in the absorption arc

PR #2874's verification posture is **mixed-discipline**:

- **Quantitative-physical claims (file sizes, file counts, constant
  values, enum membership):** EXACT precision across 21 claims. The
  author measured.
- **Code-citation specifics (constant names, regex literals, set membership
  locations, line numbers):** 5 substantive inaccuracies. The author
  appears to have authored these from memory, pattern-matching against
  similar codebases, or from a pseudo-code abstraction layer rather
  than re-read against the source at attribution time.

The mixed-discipline finding suggests **dispatch-author verification-discipline
varies by claim type**:

- File sizes / counts / constant values: dispatch authors (across 6
  instances) consistently measure these accurately when they choose to
  measure at all. The PR #2878 cycle-96 failure was different (numbers
  asserted without measuring); other 5 dispatches measured.
- Regex literals / constant names / specific line numbers: more
  dispatch-author drift, even on dispatches where quantitative claims
  verify EXACT. **Adopting code from a research-deliverable should
  re-read the source rather than copy from pseudo-code blocks**, even
  when other quantitative claims in the same dispatch verify EXACT.

The dispatch-author verification-discipline meta-attribute now stands
at **3-success + 2-mixed-success / 1-failure on a 6-instance evidence
base**:

| Cycle | PR | Verdict |
|---|---|---|
| 96 | #2878 | FAILURE: fabrication-magnitude (5.2× / 1.86× over-counts) |
| 97 | #2877 | SUCCESS: workspace-LOC 0.05-0.1% precision |
| 98 | #2873 | SUCCESS: file-byte-level + EXACT on 3 metadata fields |
| 99 | #2876 | SUCCESS: EXACT across 13 metrics + 5 file sizes + sister-comparison drift |
| 100 | #2875 | MIXED-SUCCESS: EXACT across 14 quantitative with 1 magnitude-1-OFF (direction-accurate / source-vs-runtime distinction) |
| 101 | #2874 | MIXED-SUCCESS: EXACT across 21 quantitative with 5 structural-fabrication-with-correct-direction (code-detail-citation drift) |

## Per-section verdicts (compressed)

The deliverable's 9 lenses, integration verdict each:

| Section | Subject | Verdict |
|---|---|---|
| 1 | Three-file deep dive | INTEGRATED into per-system file detection-and-routing pattern bucket + configuration-synthesis pattern bucket + algorithm-and-cycle-internal-phasing bucket; 5 structural-fabrication caveats noted in verification banner |
| 2 | State model and transitions | INTEGRATED into per-system file state-memory-history bucket + algorithm-and-cycle-internal-phasing bucket; AUTO_COMPLETE_TRANSITIONS magnitude-1-missing flagged |
| 3 | Hooks lifecycle | INTEGRATED into per-system file hooks-and-lifecycle bucket |
| 4 | MCP servers (5, corrects cycle-26's 3) | INTEGRATED as MCP-tool-surface bucket; correction confirmed |
| 5 | Rust crate ecosystem (5 crates) | INTEGRATED as Rust-substrate-edge bucket; correction confirmed; 4 of 5 crates not deep-read flagged in deeper-read queue |
| 6 | Prompt-guidance contract system | INTEGRATED into quality-and-discipline bucket |
| 7 | Cycle-26 confirmation/corrections | INTEGRATED as hypothesis-verdicts section (no H1-H3, but 3 corrections + 7 confirmations + 9 NEW patterns documented) |
| 8 | NEW code-level patterns | INTEGRATED as cross-cluster contributions in family buckets; held as single-system observations pending convergence per cycle-99 discipline |
| 9 | Anchoring caveats and gaps | INTEGRATED into per-system file anchoring caveats + deeper-read queue (14 unread files) |

## Integration outcomes

**Files modified/created on master (3 + 1 absorption note):**

1. `docs/redesign/1-research/systems/oh-my-codex.md` — REWRITTEN from
   168-line cycle-33 stub to ~470-line deep-dive parity following
   cycle-98 / cycle-99 / cycle-100 per-system shape. Sections:
   verification banner with 26-claim table; sources read so far
   (substantially expanded with 18+ files); project framing noting
   required-reads-completion + sister-project parity; hypothesis
   verdicts (no H1-H3, but 3 corrections + 7 confirmations + 9 NEW
   patterns documented); patterns observed organized into 9 family
   buckets (state-memory-history, quality-and-discipline,
   algorithm-and-cycle-internal-phasing, hooks-and-lifecycle,
   detection-and-routing, configuration-synthesis, agent-architecture,
   MCP-tool-surface, Rust-substrate-edge, substrate-edge-orchestration);
   cluster framework anchoring across 4 cluster cells with potential
   contributions and the cluster-J anti-RAG 2-system convergence
   note; 7 anchoring caveats including the structural-fabrication-with-correct-direction
   adoption-time double-check guidance; 14-item deeper-read queue with
   priority ordering; cycle-101 absorption record.

2. `docs/redesign/1-research.md` — Per-system reads table (line 87)
   row updated from "Stub — cycle-63 deeper read in flight" → "Deep-dive
   (commit `d1863f72`; 9-lens code-level read; 3 corrections + 7
   confirmations + 9 NEW patterns; 5-crate Rust workspace, 5 MCP servers
   verified)" with link to PR #2874.

3. `docs/redesign/1-research.md` — Cross-system observations preamble
   (line 98) updated "Nine systems read at depth" → "Ten systems read
   at depth" with oh-my-codex now listed at deep-dive (cycle 26 + cycle
   63 deeper read absorbed cycle 101).

4. `docs/redesign/1-research.md` — Further-systems-to-study row (line
   689) updated "Stub from cycle-26 dispatch; cycle-63 deeper read in
   flight" → "Deep-dive landed (cycle 101, [PR #2874]; supersedes
   cycle-26 dispatch [#2782]; cycle-63 dispatch [#2833] closed
   alongside PR per absorption convention)".

5. `docs/redesign/1-research.md` — Cycle plan provisional item 5 (line
   755) updated to add oh-my-codex deeper-read landing alongside
   openclaw and PAI; expanded resolution-of-asymmetry to cover the
   cycle-26-vintage dispatch set; Eva-named reads still pending
   elevation noted.

6. `docs/redesign/_notes/cycle-101-oh-my-codex-deeper-read-absorption.md` —
   THIS absorption note.

**Explicit not-modified items (with reasoning):**

1. **Cluster catalogue (`docs/redesign/2-candidates/clusters.md`)** —
   NOT modified. Per cycle-99/100 catalogue-update discipline, cluster
   counts and sub-shape catalogue updates merit a dedicated cycle. omx
   deep-dive contributes evidence to clusters A (deterministic decision
   tree NEW sub-shape candidate), F (substrate-edge thin-wrapper-with-deep-hooks
   3-system convergence: omx + PAI + omc), H (evaluator-driven keep-discard
   NEW sub-shape candidate), and I (substrate diversity now spanning
   configuration-layer-over-CLI as additional substrate type). All these
   updates are deferred to the cycle-102/103 catalogue-rebuild cycle.
   **Catalogue staleness is now ELEVATED** — pre-cycle-100 the catalogue
   anchored on 8 deep-dive systems; cycle 100 added PAI for 9; cycle
   101 adds omx for 10. Two-cycle catalogue debt accumulated; explicit
   rebuild owed.

2. **Functional-class shape catalogue** — Updated in this note as
   shape #19 verification-success HARDENED at 3 + verification-mixed
   sub-variant NOVEL at 1; not modified in `1-research.md` itself
   (the shape catalogue lives in journal entries and hand-off plans).

3. **22-pattern catalogue (cycle-26 baseline)** — NOT modified. The
   cycle-26 oh-my-codex 22 named patterns remain the canonical baseline
   for sister-project comparisons (omc cycle-99 mapping; potential
   future Symphony / autogen mappings). Cycle-63 deep read **confirms
   the 22-pattern catalogue at code-level depth** with 3 corrections
   (Rust crate count, sparkshell model, state file authority) absorbed
   into the per-system file's verification banner; no patterns deleted
   from the catalogue; 8-9 NEW patterns added under family buckets
   without replacing the catalogue baseline.

4. **PAI per-system file** — NOT modified. omx deep-dive contributes
   evidence to clusters where PAI is also a contributor; cluster-catalogue
   update (above) is the right surface for cross-system convergence
   updates, not the per-system files themselves.

## What surprised me / what I noticed

1. **The structural-fabrication-with-correct-direction failure mode
   is methodologically distinct from prior failure modes in the catalogue.**
   Earlier failure modes (cycle-96 fabrication-magnitude, cycle-99
   direction-failure, cycle-100 magnitude-1-off) were all in
   *quantitative* claims (counts, percentages, or directional asymmetries).
   Cycle-101 surfaces failure in *code-citation specifics* — constant
   names, regex literals, set membership, line numbers — while the
   architectural prose-level claims they cite are direction-correct.
   This is the first failure mode the verification discipline has
   surfaced in the structural-claim layer; prior failures were all in
   the magnitude-claim layer. **Verification-discipline gets a new
   axis: structural-claim verification distinct from quantitative-claim
   verification.** A dispatch can be EXACT on file sizes and constant
   values while still being unreliable on regex literals and constant
   names; both axes need verification at adoption time.

2. **The 22-pattern catalogue from cycle-26 holds up well at code
   level.** Of the 22 patterns enumerated at cycle 26, 3 received
   corrections (crate count, model name, state file authority — these
   were measurement errors, not architectural mistakes), 7 received
   explicit code-level confirmations (the 4-stage pipeline, planning-
   before-execution arbitration, ralplan gate advisory-only,
   autoresearch-runtime-as-primitives, deterministic decision tree,
   generator owns config.toml only, writeAtomicFile POSIX semantics),
   and the remaining 12 were not contradicted. **The cycle-26 high-level
   survey was directionally accurate at the architectural-pattern
   level even where it lacked code-level evidence.** This is encouraging
   for the quality of the original 22-pattern dispatch and suggests
   first-pass research dispatches in the omx/omc/Symphony/PAI shape
   produce architectural ground truth that survives deeper reading
   even when the deeper read corrects measurement-level details.

3. **The 5-MCP-server / 5-Rust-crate corrections are themselves a
   data point about cycle-26 dispatch limits.** Both corrections involve
   counting items in directories — the kind of measurement that's
   trivial to get right with 1 minute of `gh api` use but easy to
   get wrong if relying on README-level context or partial directory
   reads. **Cycle-26 dispatch produced architectural ground truth but
   missed structural counts.** The pattern: *first-pass surveys produce
   architecture-correct + count-imprecise outputs*; deeper reads
   correct the counts without overturning the architecture. This is
   the methodology-shape that argued for cycle-63 / cycle-71 / cycle-77
   / cycle-75 deeper-read dispatches, and the result vindicates it.

4. **The autoresearch-ledger-vs-state-atomicity asymmetry is a
   borrow-able pattern observation.** omx's state layer uses POSIX
   atomic rename (`writeAtomicFile()`); the autoresearch iteration
   ledger uses plain `writeFile()`. The asymmetry is documented as a
   known architectural gap. This is the kind of *honest gap*
   documentation the redesign's documentation-honesty pattern family
   values: the project explicitly acknowledges the inconsistency
   rather than pretending uniformity. **Pattern observation:** any
   adaptation of an external system's persistence pattern should
   *audit the atomicity uniformly* across its data files; differences
   are crash-safety differences, and asymmetry suggests evolutionary
   addition rather than designed-uniform discipline.

5. **`normalize_summary` allowlist is the strongest single-system
   evidence for cluster F (substrate-coverage) sub-shape: deterministic-
   post-processing-of-LLM-output.** omx-sparkshell's contract is:
   model is INSTRUCTED to produce sections `summary:`, `failures:`,
   `warnings:`; `normalize_summary()` ENFORCES this at parse time
   regardless of what the model emits. Anything outside the allowlist
   is silently dropped. **This is a deterministic post-processing
   contract for LLM output**, distinct from prompt-only contracts (PAI's
   prompt-driven format; openclaw's prompt-driven phase progression)
   and from prompt-+-test contracts (omc's prompt-guidance-contract.test.ts
   regex enforcement). 1-system evidence at present; potential 2-system
   convergence if Symphony's spec-first BEAM substrate has a similar
   post-processing layer (deeper read needed). **Held as single-system
   pending convergence per cycle-99/100 discipline.**

## Sibling pattern tracking

- **Functional-class shape #19 (research-deliverable-absorption-with-verification)** —
  HARDENED at 3 verification-success instances (cycles 98 Symphony / 99
  oh-my-claudecode / 100 PAI deeper). Cycle 101 introduces NEW
  sub-variant **verification-mixed** (substantial EXACT + structural-fabrication-with-correct-direction),
  NOVEL at 1 instance. The shape itself remains structurally stable;
  the sub-variant captures that verification can succeed at the
  quantitative-physical layer while surfacing failure at the
  structural-citation layer.

- **Verification-discipline pattern (cycle 96 emergence)** → **6
  instances HARDENED**. Cycle 101 contributes a NEW failure-mode entry
  (structural-fabrication-with-correct-direction) and a NEW axis
  (structural-claim verification distinct from quantitative-claim
  verification).

- **Dispatch-author verification-discipline meta-attribute** → **6
  instances HARDENED**. Pattern: dispatch authors vary in verification
  discipline AND verification discipline varies across claim types
  within a single dispatch. 3-success / 2-mixed-success / 1-failure
  ratio at 6-instance base. **NEW observation:** mixed-success can mean
  EXACT on quantitative claims with substantial inaccuracy on
  structural-citation claims (cycle 101) or EXACT on most claims with
  one direction-accurate magnitude-1-off (cycle 100); both are
  qualitatively different from cycle-96 fabrication-magnitude failure.

- **Generalization-level discipline (J-Q(a))** → **19 instances
  HARDENED**. Cycle 101 application: 3 corrections + 7 confirmations
  scoped to code-level depth with explicit "deeper-read needed" caveat
  for 14 unread files; 8-9 NEW patterns held as single-system pending
  elevation; 22-pattern cross-reference is corrections-and-confirmations
  TO baseline, not modifications OF baseline.

- **Direction-vs-magnitude discipline** → **11 instances HARDENED**.
  Cycle 101 application: 5 instances of direction-correct architectural
  prose + structural-fabrication code citations. NEW kind of
  direction-vs-structural-citation drift within the discipline's
  catalogue.

- **Required-reads-completion discipline** → HARDENED at 2 instances
  (cycle 100 first instance with required-reads-only; cycle 101 extends
  to required-reads + cycle-26-dispatch-set both at deep-dive parity,
  with Eva-named further reads at first-pass-or-deeper). The original
  cycle-16 deliverable-size-asymmetry worry is now fully resolved
  across both required reads AND the cycle-26-dispatch-set; only Eva-named
  oh-my-claudecode and Symphony remain at first-pass for elevation.

- **Sister-project structural-alignment pattern** — extends to
  **2-pattern-shape evidence**. Cycle 99 noted omc's 14/22 mapping
  density (64%) reflecting omc-omx sister-project alignment. Cycle 101
  re-confirms: omx is the senior project (more 22-pattern-catalogue
  evidence, deeper Rust workspace, longer git history), but omc is
  larger by 5 metrics (1.26-2.81×). Sister-project alignment is
  structural; substrate-investment asymmetry runs the other direction
  from authoring-time prominence.

- **Sibling-pattern binary-becomes-more-structured** — HARDENED at 4
  instances. Cycle 101 NOT a new instance.

- **Audit-as-peer pattern** — 2-instance evidence holds. Audit cycle
  213 still silent-failed (no audit-repo commit since cycle 212 at
  2026-05-07T04:35Z, ~46 hours since cycle-100 measurement; ~2 hours
  added since cycle 100). Cycle 101 NOT a new instance.

## Honest reflection (per F1-F5 corrective)

- **F1 (documentation IS the activity).** Cycle 101 is **1 substantive
  activity** (PR #2874 absorption with verification + structural-citation
  audit) yielding 1 rewritten per-system file (168 → ~470 lines) +
  4 row updates in 1-research.md + 1 absorption note + 1 PR closure with
  forward-link + 1 issue closure with forward-link + 1 journal entry.
  Documentation is the deliverable, not a separate step from the
  integration.

- **F2 (count outputs accurately, not as multiplicands).** ~5 document
  touches from 1 absorption activity, NOT "8-9 NEW patterns + 7
  confirmations + 3 corrections = 18-19 ways improved." Per-section
  verdicts table records 9 dispatch lenses → integration outcomes,
  but most map to a single per-system file family bucket.

- **F3 (functional-class shape inventory).** 20 functional-class
  shapes at **41 instances** (advanced from 20 at 40, cycle 100). Shape
  #19 verification-success remains HARDENED at 3 instances; shape #19
  verification-mixed sub-variant NOVEL at 1 instance.

- **F4 (HARDENED/TESTED/NOVEL is for redesign-process methodology only).**
  HARDENED/TESTED/NOVEL classifications applied to:
  verification-discipline (6 instances HARDENED);
  dispatch-author verification-discipline meta-attribute (6 HARDENED);
  generalization-level discipline (19 HARDENED);
  direction-vs-magnitude discipline (11 HARDENED);
  shape #19 verification-success (3 HARDENED) + verification-mixed sub-variant (NOVEL at 1);
  required-reads-completion (HARDENED at 2);
  sister-project structural-alignment (TESTED at 2 + structurally re-confirmed cycle 101).
  NOT applied to: omx patterns themselves (those use confirmation /
  correction / NEW-pattern / NEEDS-DEEPER-READ classifications).

- **F5 (lexicon entries).** *Structural-fabrication-with-correct-direction*
  (NEW failure mode in dispatch-author verification-discipline:
  architectural-pattern prose direction-correct AND code-citation
  specifics — constant names, regex literals, set membership, line
  numbers — fabricated, mis-located, or off-by-one); *quantitative-claim
  vs structural-claim verification axes* (NEW dimension in
  verification-discipline: dispatch verification can succeed at one
  axis while failing at the other within a single dispatch);
  *mixed-success verdict on dispatch-author verification-discipline
  meta-attribute* (NEW verdict category alongside success / failure;
  characterizes dispatches that verify EXACT at quantitative-physical
  layer but surface inaccuracy at structural-citation layer);
  *required-reads-completion + cycle-26-dispatch-set parity* (state
  achieved cycle 101: both required reads + all 3 cycle-26-vintage
  dispatch deepenings now at deep-dive parity); *catalogue-staleness
  ELEVATED* (state where deferred catalogue updates have accumulated
  to 2+ cycles; explicit rebuild owed).

**Self-congratulation audit.** Cycle 101 absorption could be over-stated
as "comprehensive integration of 913-line research deliverable across
9 lenses with 26 verification claims." More honest: the absorption is
**deep-dive integration**, parallel in scope to cycle 100 PAI deep-dive
absorption; the verification result is **mixed** (21 EXACT + 5
structural-fabrication), and the methodological contribution is the
introduction of the structural-claim-vs-quantitative-claim verification
axis. The biggest finding — 5 instances of structural-fabrication-with-correct-direction
in PR #2874's code-citation specifics — is correctly characterized as
revealing a NEW failure mode in dispatch-author verification-discipline,
not as an indictment of PR #2874. The author measured file sizes
accurately; the failure axis is in code-detail-citation, where dispatch
authors more frequently rely on memory or pattern-matching rather than
re-reading source. **The absorption value of PR #2874 is the
architectural deep-read** (3 corrections + 7 confirmations + 8-9 NEW
patterns at code level); the structural-fabrication caveats apply
specifically to adoption-time decisions that depend on the cited
constant names or regex literals, not to the architectural patterns
themselves.

**Iteration-until-approval discipline.** Cycle 101 is **research-corpus
expansion + verification-axis development** per ITERATION-UNTIL-APPROVAL
"deepen reference research: study an additional system or paper not yet
covered; integrate findings into the artifact." oh-my-codex was the
last absorption-arc item per cycle-98 hand-off (cycles 98 Symphony,
99 omc, 100 PAI deeper, 101 omx deeper). The corpus advances from
9-deep-dive + 1-stub-elevated-cycle-100 + 2-first-pass to
**10-deep-dive + 2-first-pass** (Symphony + omc remaining at first-pass
pending elevation to deep-dive parity). The absorption arc reaches
natural pause at cycle 101; cluster catalogue update + Author B's
counting protocol are the next deferred-priority items per cycle-100
plan.

## Pre-commit checklist

- [x] Verification table (26 quantitative + structural claims)
- [x] Per-system file at `docs/redesign/1-research/systems/oh-my-codex.md`
  rewritten following cycle-100 PAI deep-dive shape
- [x] `docs/redesign/1-research.md` per-system reads table updated
  (line 87 row revised)
- [x] `docs/redesign/1-research.md` cross-system observations preamble
  updated (line 98 "Nine systems" → "Ten systems")
- [x] `docs/redesign/1-research.md` further-systems-to-study row updated
  (line 689 "Stub from cycle-26" → "Deep-dive landed cycle 101")
- [x] `docs/redesign/1-research.md` cycle plan provisional item 5
  updated to add oh-my-codex deeper-read landing (line 755)
- [x] Absorption note in `_notes/cycle-101-oh-my-codex-deeper-read-absorption.md`
  (this file)
- [x] Honest reflection per F1-F5 corrective applied
- [x] Self-congratulation audit performed
- [x] Sibling pattern tracking updated (5 patterns extended; shape #19
  verification-success at 3 HARDENED + verification-mixed sub-variant
  NOVEL at 1; required-reads-completion + cycle-26-dispatch-set parity
  HARDENED at 2)
- [x] Anti-inheritance discipline H-Q(a): cycle 101 verification is
  empirical-observation-grounded (gh api repo state at commit `d1863f72`)
  consistent with cycles 96/97/98/99/100 verification posture

## Cycle 102+ plan

Per cycle-100 hand-off (refined for cycle-101 outcome):

1. **Cluster catalogue update** [cycle 102 or 103, HIGH PRIORITY] —
   substantial restructure absorbing PAI deep-dive + omx deep-dive
   findings. Cluster A: deterministic-decision-tree NEW sub-shape +
   classifier-mediated-routing (PAI) + prompt-driven-phasing (openclaw)
   distinguished. Cluster F: 3-system substrate-edge convergence
   (omx + PAI + omc thin-wrapper-with-deep-hooks pattern). Cluster H:
   evaluator-driven-keep-discard NEW sub-shape (omx) + feedback-signal-inference
   (PAI cycle 100). Cluster I: substrate diversity confirmed across 5
   substrate types now. **Catalogue-staleness ELEVATED** — 2-cycle
   debt accumulated; explicit rebuild owed.
2. **Author B's counting protocol** [cycle 102 or 103] — deferred from
   cycles 96-101.
3. **Second-iteration sharpening on candidates A/B/C** [cycle 104+] —
   with full 4-system absorption + cluster catalogue update fed in
   (Symphony + omc first-pass + PAI deep + omx deep). Catalogue update
   precedes candidate sharpening; candidate evaluation absorbs the
   cross-system convergence evidence.
4. **Restored-polarity dispatching** [cycle 104+] — once absorption
   arc + catalogue update complete; dispatch new research only when
   the catalogue substrates a clear gap.
5. **Symphony deeper-read elevation** [cycle 105+ or based on cluster
   catalogue findings] — Symphony first-pass at cycle 98 has the
   strongest "deeper read needed" signal among Eva-named reads if
   cluster F or cluster A convergence depends on its spec-first BEAM
   substrate evidence.
6. **oh-my-claudecode deeper-read elevation** [cycle 106+ or based on
   cluster catalogue findings] — omc first-pass at cycle 99; dispatch
   already noted that 2-instance evidence on event-driven-runtime-replacing-polling
   pattern (omc + Symphony) merits deeper read for elevation.
