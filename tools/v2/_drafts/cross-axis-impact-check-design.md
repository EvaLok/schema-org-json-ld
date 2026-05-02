# `cross-axis-impact-check` — design draft

**Status:** design draft (cycle 44, 2026-05-01; iterated cycle 46, 2026-05-02
on the 4 open Qs). Not yet implemented. This file is a forward-looking
sketch; iterate before building.

## Purpose

Mechanical structural-consistency lint for the redesign Phase 2
framework file at `docs/redesign/2-design-framework.md`. When a change
is made to one axis row, position, or cross-axis dep entry, the tool
identifies which other framework sections reference the affected entity
and flags potential inconsistencies.

This addresses the v2-design observation, load-bearing across cycles
42 and 43 (and now 44), that **cross-axis update propagation is a
recurring failure mode** during framework iteration. Cycle 42 found two
internal inconsistencies introduced by v1.5's Cognition update that
weren't propagated; cycle 43 same-cycle Q1 flagged the Axis 2 × Axis 3
dep map as BORDERLINE; cycle 44 cross-cycle escalated that flag to
load-bearing v1.8 fix.

A mechanical tool would:
- Catch propagation gaps faster (same-cycle, before commit)
- Reduce cold-reader load (less cross-axis dep walking required)
- Free orchestrator compute for substantive Phase 2 candidate work

This is exactly the CORE-DESIGN-PRINCIPLE pattern: extract repetitive
procedural work into a tool so the orchestrator can spend cycles on
substantive design.

## Scope

**In scope (v0):**
- Parse the framework file's structure: axes (1-13, with 11 absent),
  positions per axis, cross-axis dep map entries, F-pattern mappings,
  preserved-primitives interactions
- For a given axis number or position-name, list all framework sections
  that reference it
- Verify every system named in any position cell is mentioned in
  cross-axis deps where structurally relevant
- Verify every cross-axis dep entry references valid axes
- Verify every F-pattern mapping references valid axes

**Out of scope (v0):**
- Semantic correctness checking ("is this claim true?")
- Multi-system reconciliation ("does openclaw.md match Axis 2?")
- Auto-fix suggestions
- Versioning of framework file (handled by git already)

**Possible v1+ extensions:**
- Per-system file consistency check (Axis 2 says openclaw is
  file-per-component → openclaw.md must claim file-per-component)
- F-pattern mapping coverage check (every F1-F12 has at least one axis
  mapping; every axis maps to at least one F-pattern or convergent
  constraint or is documented as orthogonal)
- Diff-aware mode: given a git diff on the framework file, name which
  cross-references the diff invalidates

## Inputs / outputs

**Inputs:**
- `--framework <path>` — defaults to `docs/redesign/2-design-framework.md`
- `--axis <N>` — focus output on cross-references for a specific axis
- `--position "<name>"` — focus output on a specific position
- `--system <name>` — focus output on a specific surveyed system
- `--check` — exit non-zero if any structural inconsistency found
  (CI-friendly mode)
- `--all` — scan entire framework, no focus (default)

**Outputs:**
- Markdown-formatted report on stdout listing:
  - For each axis or position: which framework sections reference it
  - Any orphan references (cross-axis dep references an axis that
    doesn't exist; F-pattern mapping references nonexistent axis)
  - Any structural redundancy (e.g., system listed in two positions
    of the same axis without explicit dual-listing rationale)
- Exit code 0 if no structural issues; non-zero in `--check` mode if
  issues found

## Architecture sketch

Single binary in Rust: `tools/v2/crates/cross-axis-impact-check/`.

**Phase 1 — parser:**
- Markdown parser (probably `pulldown-cmark` for the framework file)
- AST extraction: identify axis-numbered headings, position-row tables,
  cross-axis dep bulleted list, F-pattern mapping table
- Build structured representation: `Axes`, `Positions`, `CrossAxisDeps`,
  `FPatternMappings`, `PreservedPrimitives`

**Phase 2 — analyzer:**
- Build cross-reference graph: axis → positions, axis → systems
  mentioned, axis → cross-axis deps, axis → F-pattern mappings, axis →
  preserved-primitives interactions
- Detect orphan references (cross-axis dep entry mentioning Axis 14:
  invalid)
- Detect redundancy (same system in two positions without dual-listing
  comment)

**Phase 3 — reporter:**
- For focused query (`--axis 2`): print all sections that reference Axis 2
- For check mode: print only inconsistencies; exit non-zero if any
- For all mode: print structural summary + any inconsistencies

## Test surface

Unit tests:
- Parser tests on toy markdown files (synthetic axis/position/dep
  structures)
- Analyzer tests on small framework fragments (single axis, two cross-
  axis deps)
- Detect-orphan tests (intentionally-broken framework fragments)

Integration tests:
- Run on actual `docs/redesign/2-design-framework.md` v1.8; expect
  zero structural inconsistencies (post-cycle-44 fix state)
- Run on `docs/redesign/2-design-framework.md` v1.4 (git checkout to
  pre-cycle-42 state); expect to detect the Cognition update
  propagation gap that cycle-42 Q(a) found (Axis 1 update not
  propagated to Axis 7)

## Open design questions — RESOLVED CYCLE 46

**Q1 — When does the tool run? — RESOLVED: manual-only initially**

Decision: tool is **manually invoked** by the orchestrator during
cold-reader workflow, with no pre-commit hook and no CI check in v0.

Rationale:
- Manual invocation has lowest friction with the existing cold-reader
  discipline (the orchestrator runs the tool when iterating, sees the
  output, decides what to fix).
- Pre-commit hook is risky during Phase 2 candidate generation: the
  framework structure may legitimately change (new axes added, position
  rows rewritten), and a hook that blocks "structural inconsistency"
  during in-progress restructuring would create friction without
  catching real issues.
- CI check on PRs touching the framework file is plausible v1+ once
  the framework structure is stable (post-Phase-2 candidate generation).
  Premature CI integration risks the same Phase-2-restructuring
  friction as pre-commit hooks.
- Manual-only mode aligns with the cycle-44 framing: tool catches what
  cold-reader currently catches, but mechanically and faster.

v1+ trigger for adding CI: framework structure has been stable for 5+
cycles AND Phase 2 candidate generation is complete (so the tool's
parser doesn't need to track moving structure).

**Q2 — How does the tool handle freeform prose? — RESOLVED: tables strict, prose via known-pattern regex**

Decision: the tool **parses tables strictly** and extracts prose
mentions via regex with explicitly-enumerated known patterns. Accept
that some prose references will be missed in v0; document which
patterns are covered.

Patterns covered in v0:
- `Axis ([0-9]+)` (matches "Axis 2", "Axis 12") — captures axis number
- `F([0-9]+)\b` (matches "F1" through "F12") — captures F-pattern number
- System names from explicit allowlist: `openclaw`, `LangGraph`,
  `Voyager`, `AutoGen`, `Cognition`, `PAI`, `oh-my-codex`, `OpenAI`
  (case-insensitive; allowlist updated as new systems join Phase 1)
- Backtick-quoted position names from known-position-list (extracted
  from table parsing during the prior pass)
- `CORE-DESIGN-PRINCIPLE` literal mention

Patterns NOT covered in v0:
- Free prose like "the file-per-component approach" (descriptive
  reference to a position without explicit position-name quoting)
- "the abandonment-cascade pattern" (descriptive reference to an
  F-pattern without F-number)
- Any cross-reference written in non-canonical form

Rationale for missing-pattern acceptance:
- Tables carry the load-bearing structural claims; prose adds context
  but rarely introduces structural inconsistencies on its own.
- Freeform prose is not amenable to mechanical parsing without LLM
  involvement; the orchestrator IS the LLM and can catch prose
  inconsistencies in cold-reader.
- The tool's value-add is mechanical structural checking, not prose
  comprehension. False-positives from incomplete prose parsing would
  reduce orchestrator trust in the tool faster than missed-references
  reduce its utility.

v1+ extension: if specific prose-reference patterns recur (e.g., "the
N-component pattern" naming conventions), add to regex allowlist.

**Q3 — How does this interact with the persistence mechanism? — RESOLVED: read-only analyzer**

Decision: the tool is a **read-only analyzer** of the framework file.
No persistence-mechanism changes needed.

Notes:
- The framework file (`docs/redesign/2-design-framework.md`) IS one of
  the persistence mechanisms; per-cycle `_notes/` files are the other.
- Cross-axis-impact-check reads the framework but doesn't modify it.
  Output is stdout (markdown report) or exit code (for CI).
- The tool's existence does not introduce a new persistence layer.
- Future v1+ extension: a persistent index file (e.g.,
  `docs/redesign/_indices/cross-axis-impact.json`) caching the parsed
  structure could speed up incremental checks. Defer to v1+ when
  performance becomes a real bottleneck — for v0, parse-on-each-run
  is fine for a ~765-line framework file.

**Q4 — What's the minimum viable cycle for v0? — RESOLVED: 3-4 cycles realistic**

Decision: revise estimate from "1-2 cycles" to **3-4 cycles** for v0.
Cycle-44 draft was overoptimistic about parser complexity vs the
actual structural variability of the framework file.

Realistic phasing:
- **Cycle T+0 (scaffold):** Cargo crate skeleton; markdown parser
  integration (`pulldown-cmark`); basic table-row extraction for axis
  positions and F-pattern mappings. No analyzer logic; just print
  parsed structure as JSON.
- **Cycle T+1 (analyzer):** Cross-reference graph construction;
  orphan detection (cross-axis dep references invalid axis;
  F-pattern mapping references invalid axis); print orphans.
- **Cycle T+2 (reporter + tests):** Markdown-formatted report
  output; unit tests on synthetic markdown; integration test on
  actual framework file (expect zero structural inconsistencies in
  current v1.9 state); integration test on framework file checked
  out at v1.4 (expect to detect cycle-42's Cognition update
  propagation gap).
- **Cycle T+3 (refinement, optional):** CLI flag handling
  (`--axis`, `--position`, `--system`, `--check`); error message
  polish; doc.

Cycle-44's "1-cycle" estimate assumed a strict-tables-only parser
with the simplest analyzer. Realistic factors that expand cycle count:
- Markdown table parsing has edge cases (tables embedded in
  blockquotes, nested-formatting position names, multi-line cells)
- Cross-reference graph requires extracting position names from
  multiple table types (axis tables, F-pattern table, preserved-
  primitives table)
- Tests are non-trivial on synthetic markdown (need to construct
  small valid framework fragments)

When to start: post-Eva-checkpoint-approval on the post-retrospective
checkpoint OR if cross-axis-update-propagation failure mode fires
again in cycles 47-50 (5th instance would be strong empirical signal
for tool ROI). This is v2 tool work — not gated on the redesign's
pre-checkpoint phases per CORE-DESIGN-PRINCIPLE elaboration.

**Tracker for new Qs surfacing during cycle-46+ iteration:**

(none yet; record any future open Qs here for cycle-47+ resolution)

## Cycle 47+ next steps

Cycle 45 deferred build-start; cycle 46 iterated the 4 open Qs to
RESOLVED. The design is now buildable. Build sequencing depends on:

1. **Eva-checkpoint approval** on post-retrospective checkpoint
   (current standing position: still iterating; build can proceed
   independently as v2 tool work per redesign authority).
2. **Failure-mode recurrence signal:** if cross-axis-update-propagation
   fires again in cycles 47-50 (5th instance), tool ROI signal is
   strong — start build promptly.
3. **Phase 2 candidate generation timing:** if Phase 2 begins during
   cycles 47-50, build the tool BEFORE candidate-file generation so
   it can lint candidate-file cross-references too.

Build pathway options:
- **Path A (orchestrator-built):** cycle T+0 through T+3 as outlined
  in Q4. Lower context-switching cost but consumes orchestrator cycles.
- **Path B (Copilot-dispatched implementation):** dispatch a Copilot
  task with this design draft as input + the framework file as the
  test artifact + a request for the cycle-T+2 integration test
  (parse v1.4 framework, expect to detect the Cognition propagation
  gap). One Copilot dispatch could deliver Phase 1+2+3 in a single
  PR. Higher reviewer overhead per the per-finding evaluation
  discipline (cycles 7, 12, 31, 41, 43).
- **Path C (hybrid):** orchestrator builds the parser scaffold
  (cycle T+0); dispatch Copilot for the analyzer + reporter (cycles
  T+1 and T+2) since those have higher novelty and benefit from the
  Copilot model's broader Rust ecosystem familiarity.

Lean: Path A or C. Path B has merit but the per-finding evaluation
overhead may exceed the orchestrator-build cost for a tool this
small.

If cycle 47+ defers and picks up `redispatch` instead:
- Both tools are useful; ordering depends on which addresses more
  recurring orchestrator work
- cross-axis-impact-check addresses cold-reader workflow (high
  frequency: every framework iteration cycle; instance count to
  date — 4 across cycles 42-45)
- redispatch addresses dispatch primitive (lower frequency: every
  Phase 1+ external dispatch; instance count to date — 1 escalation
  in cycle 39)
- Lean toward cross-axis-impact-check first for higher-frequency value

## Connection to v1.8

V1.8's Q(a) BORDERLINE-FAIL → load-bearing fix was the third instance
of the cross-axis-update-propagation failure mode (cycles 42, 43 same-
cycle, 44 cross-cycle). Three instances over five cycles is empirical
evidence that this failure mode is recurring, not one-off.

A mechanical lint tool addresses this directly. Building it before
Phase 2 candidate generation would:
- Reduce cold-reader load during candidate generation (which will
  iterate on multiple candidate files plus the framework)
- Establish the v2 tool development pattern (Rust crate, tests, docs,
  CI integration) on a low-risk artifact
- Demonstrate the CORE-DESIGN-PRINCIPLE in the redesign work itself
  (orchestrator extracts procedure into tool rather than performing
  procedure each cycle)
