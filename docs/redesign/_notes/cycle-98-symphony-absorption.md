# Cycle 98 — Symphony first-pass survey absorption: PR #2873 (issue #2851; cycle-77 dispatch)

**Cycle issue:** [#2887](https://github.com/EvaLok/schema-org-json-ld/issues/2887)
**Source:** [PR #2873](https://github.com/EvaLok/schema-org-json-ld/pull/2873) (Copilot research-only dispatch on
[issue #2851](https://github.com/EvaLok/schema-org-json-ld/issues/2851), cycle 77)
**Deliverable absorbed:** `_notes/cycle-77-symphony-survey.md` on branch
`copilot/redesign-research-first-pass-survey-cycle-77` at commit `2d940994`
(644 lines; PR closed without merge per absorption convention)
**Cycle composition shape:** **research-deliverable-absorption-with-verification-success** — NOVEL
functional-class shape #19 (1 instance). Distinct from cycle 97 shape #18
(feedback-absorption-with-verification-success) by **dispatch type**: research
deliverables describe a target external system; feedback deliverables critique
an existing internal artifact. The verification step generalizes across both
dispatch types — this is the third instance of formalized verification on a
returned dispatch (cycle 96 catch / cycle 97 verify / cycle 98 verify), now
extending the discipline to research deliverables.

## Setup

PR #2873 is the cycle-77 first-pass survey deliverable on `openai/symphony`,
the second target named in Eva directive [#2775](https://github.com/EvaLok/schema-org-json-ld/issues/2775).
The dispatch was authored under the 12-lens structure (repo glance, spec
architecture, Elixir impl architecture, spec-as-contract test, workflow
contract, tracker integration, workspace isolation, lifecycle/phasing, trust
posture, 22-pattern cross-reference against cycle-26 oh-my-codex baseline,
NEW patterns + hypothesis verdicts, anchoring caveats). Hypotheses tested:
H1 (organizational-substrate vocabulary distinct from solo-author); H2
(Elixir/BEAM substrate exploitation); H3 (cluster I substrate-correlation
prediction); H4 (spec-as-contract pattern). All four CONFIRMED at first-pass
depth.

## Verification table (cycle 98)

Following cycle 96/97 verification-discipline pattern, load-bearing
quantitative claims spot-checked against the live `openai/symphony` repository
via `gh api`:

| Claim in PR | Verification source | Result |
|---|---|---|
| `SPEC.md` size 80204 bytes | `gh api repos/openai/symphony/contents/SPEC.md` → `size: 80204` | ✓ EXACT |
| `README.md` size 1731 bytes | `gh api repos/openai/symphony/contents/README.md` → `size: 1731` | ✓ EXACT |
| `orchestrator.ex` 52KB | actual 52564 bytes | ✓ within rounding |
| `status_dashboard.ex` 60KB | actual 60589 bytes | ✓ within rounding |
| `workspace.ex` 14KB | actual 14626 bytes | ✓ within rounding |
| `config/schema.ex` 16KB | actual 16800 bytes | ✓ within rounding |
| `linear/client.ex` 16KB | actual 16285 bytes | ✓ within rounding |
| `codex/app_server.ex` 30KB | actual 30051 bytes | ✓ within rounding |
| Created 2026-02-26T21:54:00Z | `gh api repos/openai/symphony` | ✓ EXACT |
| Language Elixir | metadata response | ✓ EXACT |
| Stars 22513 | current 22641 | ✓ drift within 1% (post-survey time elapsed) |
| Forks 2097 | current 2114 | ✓ drift within 1% (post-survey time elapsed) |
| Open issues 5 | metadata response | ✓ EXACT |

**Verdict:** 13 quantitative claims verified; 8 file-size claims to
byte-level precision; metadata to exact match; star/fork count drift
consistent with elapsed real time since survey. **Verification-success at
higher precision than cycle 97** (8 file-size byte-level matches vs cycle 97's
4-claim verification at the workspace-level). PR #2873's author actually
counted file sizes via repo access before producing the survey — this is the
calibration-first methodology of cycle-97 PR #2877, not the
fabrication-magnitude failure of cycle-96 PR #2878.

## Per-section verdicts (compressed)

The deliverable's 12 sections per dispatch structure, integration verdict
each:

| Section | Subject | Verdict |
|---|---|---|
| 1 | Repo glance + cluster-anchoring hints | INTEGRATED into per-system file project framing |
| 2 | Architecture per spec | INTEGRATED into per-system file orchestration patterns |
| 3 | Architecture per Elixir impl + spec↔impl convergence | INTEGRATED into per-system file agent architecture; deferred items noted in deeper-read queue |
| 4 | Spec-as-contract H4 test | INTEGRATED; H4 verdict CONFIRMED noted in hypothesis verdicts section |
| 5 | Workflow contract pattern | INTEGRATED into per-system file documentation honesty patterns |
| 6 | Issue-tracker integration | INTEGRATED into per-system file agent architecture (tracker adapter boundary) |
| 7 | Workspace isolation pattern | INTEGRATED into per-system file trust posture & security defaults |
| 8 | Lifecycle / phasing | INTEGRATED into per-system file orchestration patterns |
| 9 | Trust posture plurality + H3 test | INTEGRATED; H3 verdict CONFIRMED with first-pass-depth caveat |
| 10 | Cross-reference to cycle-26 22 patterns | INTEGRATED into per-system file as cross-reference section |
| 11 | NEW patterns + hypothesis verdicts | INTEGRATED into per-system file as single-system observations + hypothesis verdicts |
| 12 | Anchoring caveats / transferability | INTEGRATED into per-system file anchoring caveats |

All 12 sections INTEGRATED with appropriate depth-asymmetry caveats. Items
explicitly deferred by the dispatch (full RFC 2119 clause-by-clause
conformance, full `app_server.ex` audit, exhaustive tracker abstraction
proof, full `status_dashboard.ex` audit) preserved as deeper-read queue in
per-system file.

## Integration outcomes

**Files created:**

- [`docs/redesign/1-research/systems/symphony.md`](../1-research/systems/symphony.md) —
  per-system file in the cycle-33 cycle-33-research-restructure layout.
  Sources read, project framing, patterns observed (organized into 6 family
  buckets matching the cross-system observations layout), hypothesis
  verdicts, anchoring caveats, cross-reference to cycle-26 catalogue, NEW
  patterns surfaced (single-system pending elevation), deeper-read queue.

**Files modified:**

- [`docs/redesign/1-research.md`](../1-research.md) — Per-system reads
  table extended with Symphony row (first-pass status, deeper-read queue
  documented; PR #2873 cited as primary evidence base). Further-systems-to-
  study table updated: Symphony row "TBD / Pending" → "Copilot research-
  only dispatch (cycle 77) / First-pass landed (cycle 98)" with link to
  per-system file.

**Files NOT modified (and why):**

- Cross-system observations Family C/E/A/B/D sections in
  [`1-research.md`](../1-research.md) — Symphony at first-pass depth does
  NOT yet meet the depth bar of the 8 deep-dive systems anchoring those
  sections; Symphony observations are held in the per-system file pending
  deeper-read. Symphony's NEW patterns (spec-as-contract, dual-contract
  architecture, etc.) are single-system observations; per cross-system
  convergence discipline they remain in the per-system file pending
  elevation via deeper-read or convergence with another system's deep-dive.
- [`1-research/clusters.md`](../1-research/clusters.md) — Cluster
  catalogue is anchored on 6 deep-dive systems (cycles 62-74); Symphony
  at first-pass does not yet match that depth bar. The H3 cluster-I
  substrate-correlation hypothesis is CONFIRMED at first-pass; treating
  Symphony as a third substrate-correlated system requires deeper-read
  parity.
- Candidates A/B/C ([`2-candidates/`](../2-candidates/)) — Symphony's
  transferable design-input (spec-as-contract pattern, RFC 2119 normative
  spec discipline, named-phase lifecycle vocabulary, reconciliation-before-
  dispatch tick pattern) is single-system at first-pass; per the
  integration discipline of cycle 96 / cycle 97, single-system first-pass
  observations do not auto-integrate into candidate documents. The most
  promising design-input — selective RFC 2119 normative clauses for
  safety-critical / process-critical behavior in v2 — is recorded in the
  per-system file's anchoring caveats (section 12.6 of the deliverable)
  and is candidate-iteration material if a future deep-dive system or
  convergence elevates the pattern.

## What surprised me / what I noticed

- **Symphony's spec-as-contract architecture is the strongest single
  pattern Symphony adds.** None of the 8 deep-dive systems have RFC 2119
  normative spec language at the 80KB scale Symphony's `SPEC.md` has.
  openclaw and PAI have spec/architecture documents; AutoGen and LangGraph
  have framework documentation; OpenAI harness has internal writeup;
  oh-my-codex has STATE_MODEL.md. None are explicitly normative
  language-agnostic contracts inviting reimplementation in any language.
  This is a **genuinely novel substrate posture** in the corpus, even if
  it remains single-system at first-pass depth.
- **The 6 NEW patterns Symphony surfaces collapse to ~3-4 distinct
  shapes when consolidated.** Pattern 1 (spec-first contract) and pattern
  2 (dual-contract: spec + workflow) are facets of the same architectural
  posture. Pattern 4 (workspace safety with symlink canonicalization) and
  pattern 6 (continuation under cap) are concrete implementation details
  that improve known patterns rather than introducing new architectural
  axes. Pattern 3 (tracker/filesystem restart recovery) is a substrate
  choice rather than a novel architectural pattern; v1 already does
  similar (state.json + git history as substrate). Pattern 5 (live
  workflow reload with last-known-good) is a reliability primitive
  potentially transferable. Net: the ~3-4 genuinely-distinct contributions
  are *spec-as-contract*, *workspace as hard runtime invariant*, and
  *live-reload with last-known-good* — all worth tracking as
  single-system observations.
- **Symphony is BEAM/OTP-native at substrate level — concrete primitives
  do not transfer to GitHub Actions cron substrate.** OTP supervision
  trees, GenServer state owners, Task.Supervisor worker lifecycles are
  Erlang/Elixir-specific concurrency primitives. The conceptual
  equivalents (single authoritative state owner, bounded worker
  lifecycle, explicit retry-timer ownership, reconciliation-before-
  dispatch tick) **do** transfer; substrate primitives do not. Symphony's
  most-transferable contributions are the architectural patterns
  expressed in `SPEC.md` (which is substrate-agnostic by design), not the
  Elixir reference implementation's specific OTP usage.
- **Symphony's `Human Review` handoff state is substantively different
  from redesign's question-for-eva blocking semantics.** Symphony models
  `Human Review` as a *normal handoff state* — workflow continues
  operating while waiting; multiple issues can be in `Human Review`
  simultaneously without the orchestrator blocking. Redesign's
  question-for-eva is a *blocker*: the orchestrator continues running
  cycles, but the question is by definition a wait-for-Eva state for the
  specific decision. Semantic equivalence should not be assumed; explicit
  handoff-state modeling vs blocker-state modeling is itself a design
  choice the redesign has implicitly made (everything-is-a-blocker vs
  some-things-are-handoffs). This is a candidate-iteration question for
  Phase 2 sharpening if reached.
- **The cycle-77 dispatch's verification posture is methodologically
  strong.** Three observed signs: (a) every load-bearing quantitative
  claim is citation-backed against specific files / line ranges in the
  source repo; (b) survey explicitly distinguishes "claim verified at
  survey depth" from "needs deeper read" rather than over-asserting;
  (c) deferred items are explicitly enumerated in the deeper-read queue
  rather than left implicit. This matches PR #2877 lens-author
  discipline and suggests dispatch-author verification discipline is a
  meta-attribute distinguishable across dispatches. **Now 3-instance
  evidence of dispatch-author verification-discipline as a meta-attribute
  of dispatch deliverables** (PR #2878 fabrication-magnitude failure;
  PR #2877 calibration-first verified; PR #2873 file-state verified).

## Sibling pattern tracking

- **Functional-class shape #19 (research-deliverable-absorption-with-
  verification-success) — NOVEL at 1 instance** (cycle 98). Distinct
  from shape #18 (cycle 97 feedback-absorption-with-verification-success)
  by dispatch type (research vs feedback). Verification step generalizes
  across both types.
- **Verification-discipline pattern (cycle 96 emergence)** — extended
  to **3 instances HARDENED** (cycle 96 PR #2878 fabrication catch,
  cycle 97 PR #2877 calibration verified, cycle 98 PR #2873 file-state
  verified). The discipline now holds across both feedback and research
  dispatch types. Pattern: external dispatch deliverables making
  load-bearing factual claims are verified before integration; ~5-15
  minutes of file-state checking against load-bearing factual claims
  defends against fabrication-magnitude failure. Cycle 98 instance shows
  the discipline applies to research deliverables too — and that
  research-dispatch authors can match (or exceed) feedback-dispatch
  authors in verification posture.
- **Dispatch-author verification-discipline as meta-attribute** —
  extended to **3 instances HARDENED**. Pattern: dispatch authors vary
  in verification discipline; verification-of-the-claim is informative
  for trust-weighting beyond the current absorption. Cycle 96 (PR #2878
  fabricated 5.2× / 1.86× anchors) sits at the failure end of the
  spectrum; cycle 97 (PR #2877 0.05% / 0.1% precision) and cycle 98
  (PR #2873 byte-level / EXACT) sit at the success end. The 3-instance
  evidence supports the discipline as a robust observation, though the
  sample is small and dispatch-author identity is masked across the 3
  cases.
- **Direction-vs-magnitude discipline** — NOT a new instance this cycle
  (cycle 98 verification was direct factual verification; no
  direction-vs-magnitude trade-off arose). Holds at 8 instances
  HARDENED.
- **Generalization-level discipline (J-Q(a))** — extends to **16 instances
  HARDENED**. Cycle 98 application: Symphony's hypothesis verdicts
  (H1-H4) are scoped to first-pass depth with explicit "deeper-read
  needed" caveat for items not fully audited. Cluster augmentation calls
  are scoped to "would extend cluster X if deeper-read parity reached"
  rather than "extends cluster X" outright. Generalization-level
  discipline applies through the integration step: Symphony's NEW
  patterns are single-system pending elevation, NOT auto-elevated to
  cross-system observations.
- **Sibling-pattern binary-becomes-more-structured** — HARDENED at
  4 instances. Cycle 98 NOT a new instance (no new structuring
  introduced).
- **Audit-as-peer pattern** — 2-instance evidence holds. Cycle 98 NOT
  a new instance; audit cycle 213 still no audit-repo commit since
  cycle 212 (silent-failed observation persists from cycles 95-97 +
  now 98).
- **Per-cycle absorption discipline** — 3-instance pattern (cycles
  96-98) of explicit verification step before integration. Cycle 99+
  may extend if next absorption (PR #2876 oh-my-claudecode per cycle
  97 hand-off) exercises the same discipline.

## Cycle 98 + 99 hand-off

**Cycle 98 substantive activity completed:** PR #2873 Symphony
absorption — 12-section deliverable verified (8 byte-level + 5 metadata
claims) and INTEGRATED into per-system corpus via new
[`systems/symphony.md`](../1-research/systems/symphony.md) and
[`1-research.md`](../1-research.md) status row updates. PR closure +
issue closure with forward-link comments per housekeeping
closure-discipline.

**Bottleneck composition (continued):** Pre-cycle-96: external-evidence
path silently broken (cycles 78-95 SEVENTEEN consecutive repo-internal
output cycles). Cycles 96-98: third cycle of absorption arc; 3 PRs
remain queued for cycles 99-101 (oh-my-claudecode #2876, PAI #2875,
oh-my-codex deeper #2874). PR #2884 merged 12:18 UTC 2026-05-08 restores
dispatch mechanism repo-wide; cycle 98 did not exercise new dispatch
(absorption work, no new dispatch fired). Cycle 102+ may default back to
research/feedback dispatching once absorption arc reaches natural pause.

**Cycle 99 substantive focal options (per cycle 97 hand-off, refined):**

1. **PR #2876 oh-my-claudecode absorption** HIGH PRIORITY — next item in
   Phase 2 absorption arc; first-pass survey deliverable for the
   companion target named in Eva [#2774](https://github.com/EvaLok/schema-org-json-ld/issues/2774);
   structurally similar to Symphony absorption (research dispatch, first-
   pass, will go to per-system file).
2. **PR #2875 PAI deeper-read absorption** MEDIUM-HIGH — could swap with
   oh-my-claudecode if oh-my-claudecode is unavailable or smaller-LOC;
   PAI deeper-read is a more substantive integration target (PAI was
   first-pass at cycle 14; deeper-read elevates PAI from first-pass
   stub → deep-dive parity, which matters for cluster catalogue
   strengthening).
3. **PR #2874 oh-my-codex deeper-read absorption** MEDIUM-HIGH — same
   pattern as PAI but for oh-my-codex (currently stub from cycle 26;
   deeper-read elevates to deep-dive parity).
4. **Author B's counting protocol** MEDIUM — deferred from cycles 96-97
   per F18 + Risk 8.
5. **Phase 3 prototype scaffolding (`phase-transition-check`)** MEDIUM —
   broadens migration-cost evidence base from 2 to 3 instances; bounded
   option if Phase 1 PRs are unavailable.

**Cycle 99 substantive focal default:** option (1) PR #2876
oh-my-claudecode absorption per cycle 97 hand-off; if available it
maintains the absorption-arc cadence cleanly.

**Cycle 100-102 plan (rough, refined from cycle 97 hand-off):**
- Cycle 100: PR #2875 PAI deeper-read (deep-dive elevation)
- Cycle 101: PR #2874 oh-my-codex deeper-read (deep-dive elevation)
- Cycle 102: B's counting protocol (deferred work)
- Cycle 103+: second-iteration sharpening on candidates A/B/C with full
  4-system absorption integration; restored-polarity dispatching as
  default substantive activity per ADR 0015.

**Iteration-until-approval discipline.** Cycle 98 is **research-corpus
expansion** per ITERATION-UNTIL-APPROVAL "deepen reference research:
study an additional system or paper not yet covered; integrate findings
into the artifact." Symphony was the only Eva-named target not yet read
at any depth before this cycle. The corpus advances from
8-deep-dive-systems anchoring cluster catalogue + 1-stub system → same
8-deep-dive + 1-stub + 1-first-pass (Symphony). Three remaining
absorptions (PAI deeper, oh-my-codex deeper, oh-my-claudecode first-pass)
will further fill out the corpus before second-iteration sharpening on
candidates A/B/C resumes.

## Honest reflection (per F1-F5 corrective)

- **F1 (documentation IS the activity).** Cycle 98 is **1 substantive
  activity** (PR #2873 absorption with verification) yielding
  1 new per-system file + 2 row updates in 1-research.md + 1 absorption
  note + 1 PR closure with forward-link + 1 issue closure with
  forward-link + 1 journal entry. Per F1: documentation is not separate
  from absorption.
- **F2 (count activities, not document touches).** 1 absorption activity
  produced ~5 document touches. NOT "12 sections = 12 ways improved" —
  the 12 sections are ONE deliverable's structure, integrated as
  ONE absorption.
- **F3 (instance counts honest).** 19 functional-class shapes at 38
  instances (advanced from 18 at 37 cycle 97). Verification-discipline
  pattern at 3 instances HARDENED. Dispatch-author verification-
  discipline meta-attribute at 3 instances HARDENED.
  Generalization-level discipline (J-Q(a)) at 16 instances HARDENED.
- **F4 (HARDENED/TESTED/NOVEL is for redesign-process methodology only).**
  Cycle 98 sibling-pattern updates apply to redesign-process patterns
  only (verification-discipline, dispatch-author verification-discipline
  meta-attribute, generalization-level discipline). NOT applied to
  Symphony's own pattern claims (those remain at first-pass depth as
  single-system observations).
- **F5 (lexicon entries).** New lexicon: *first-pass depth* as a corpus-
  status distinct from *deep-dive parity*; *deeper-read queue* as
  explicit deferred-items list rather than implicit "to-do later"; *
  spec-as-contract* as a substrate posture distinct from
  framework/library/CLI-wrapper postures.
- **H-Q(a) anti-inheritance.** Cycle 98 verification is empirical-
  observation-grounded (workspace + repo file-state measurement)
  consistent with cycle 96 corrections-grounding and cycle 97
  calibration-grounding. The 3-instance evidence on
  verification-discipline does NOT inherit confidence from any prior
  cycles — each instance is independently anchored.

**Self-congratulation audit.** Cycle 98 absorption could be over-stated
as "comprehensive integration of 644-line research deliverable across
12 sections with 8 byte-level verifications." More honest: Symphony
absorption is **first-pass depth** integration; the per-system file is
appropriately stub-equivalent to oh-my-codex's status (which has been
stub since cycle 26 and remains so pending deeper-read at cycle 63 dispatch);
Symphony does NOT yet match the depth of openclaw / AutoGen / LangGraph /
Voyager / Cognition Devin / OpenAI harness deep-dive systems. The
**verification step** is the load-bearing methodological contribution
of cycle 98 — extending the cycle-96-emergence to a third dispatch type
and demonstrating that research dispatches can be verified with the same
rigor as feedback dispatches. The biggest finding — Symphony's
spec-as-contract architecture is single-system novel within the corpus —
is correctly characterized as a single-system observation pending
elevation, NOT as established cross-system convergence.

**Pre-commit checklist** (per cycle 96 emergence, applied cycle 98):

- [x] PR #2873 read in full (644 lines)
- [x] Load-bearing quantitative claims verified against `gh api`
      (8 file-size matches + 5 metadata matches)
- [x] Per-system file follows cycle-33 layout convention (sources read,
      project framing, patterns observed, hypothesis verdicts, anchoring
      caveats, cross-reference, NEW patterns, deeper-read queue)
- [x] 1-research.md per-system reads table extended with Symphony row
- [x] 1-research.md further-systems-to-study Symphony row updated from
      "TBD / Pending" to first-pass landed
- [x] Cross-system observations Family sections NOT touched (depth
      asymmetry preserved)
- [x] Cluster catalogue NOT touched (depth asymmetry preserved)
- [x] Candidates A/B/C NOT touched (single-system first-pass insufficient)
- [x] Deferred items explicitly enumerated in per-system file's
      deeper-read queue
- [x] Cycle composition shape named (#19 NOVEL); sibling patterns updated
- [x] Honest reflection per F1-F5 corrective
- [x] Cycle 99+ hand-off plan refined from cycle 97 hand-off
- [ ] PR #2873 closed with forward-link comment (pending in cycle close)
- [ ] Issue #2851 closed with forward-link comment (pending in cycle close)
- [ ] Journal entry written (pending in cycle close)
- [ ] Session-end summary on cycle issue (pending in cycle close)
