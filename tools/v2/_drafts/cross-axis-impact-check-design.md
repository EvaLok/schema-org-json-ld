# `cross-axis-impact-check` — design draft

**Status:** design draft (cycle 44, 2026-05-01). Not yet implemented.
This file is a forward-looking sketch; iterate before building.

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

## Open design questions

**Q1 — When does the tool run?**
- Manually invoked by orchestrator when iterating framework
  (cold-reader workflow)
- Pre-commit hook (could block commit if structural inconsistencies)
- CI check on PRs touching the framework file
- Some combination

The "manual" path is least intrusive and most easily integrated into
existing cold-reader discipline. CI integration could be added later.

**Q2 — How does the tool handle freeform prose?**
The framework file has structured tables AND prose paragraphs that
reference axes/positions informally. The tool can't parse all the
prose, but it can extract obvious patterns ("Axis N", "F-pattern
F[0-9]+", quoted position names from a known list).

For v0: parse tables strictly, extract prose mentions via regex with
known-axis and known-system patterns. Accept that some prose
references will be missed.

**Q3 — How does this interact with the persistence mechanism?**
The framework file is the central artifact. The cross-axis-impact-check
tool reads it but doesn't write to it. Its output is ephemeral
(stdout) or used for CI-pass/fail signal. No persistence-mechanism
changes needed.

**Q4 — What's the minimum viable cycle for v0?**
- 1 cycle: skeleton + parser + simplest analyzer (orphan detection)
- 2 cycles: + cross-reference graph + redundancy detection
- 3 cycles: + reporter + tests + CI integration

Likely 1-2 cycles to v0, post-Eva-checkpoint-approval (since this is
v2 tool work, not directly under the redesign mandate's pre-checkpoint
phase 1/2 discipline).

## Cycle 45+ next steps

If cycle 45 takes this draft forward:
- Iterate the design (open questions Q1-Q4, possibly add v1+ scope)
- Optionally: prototype the parser in a single Rust file (no full
  crate yet)
- Capture parser robustness concerns (what if framework structure
  changes during Phase 2 candidate generation?)

If cycle 45 defers and picks up `redispatch` instead:
- Both tools are useful; ordering depends on which addresses more
  recurring orchestrator work
- cross-axis-impact-check addresses cold-reader workflow (high
  frequency: every framework iteration cycle)
- redispatch addresses dispatch primitive (lower frequency: every
  Phase 1+ external dispatch)
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
