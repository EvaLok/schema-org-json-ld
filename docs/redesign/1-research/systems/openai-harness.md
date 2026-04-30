# OpenAI harness-engineering writeup

[← back to Phase 1 index](../../1-research.md)

**Status: stub.** A Copilot research dispatch landed in cycle 26 as
PR [#2783](https://github.com/EvaLok/schema-org-json-ld/pull/2783),
which is the evidence base. A deeper orchestrator-direct read of
the writeup itself is queued under issue
[#2781](https://github.com/EvaLok/schema-org-json-ld/issues/2781).
Per-pattern citations live in the cross-system observations section
of [`1-research.md`](../../1-research.md); this stub assembles those
citations into a per-system view so the asymmetry with the
deeper-read systems is visible rather than hidden.

## Sources read so far

- OpenAI's published harness-engineering writeup (the dispatch's
  primary read)
- PR [#2783](https://github.com/EvaLok/schema-org-json-ld/pull/2783)
  Copilot research deliverable (organized into named patterns 1-13)

## Patterns observed (citations resolve to cross-system observations)

- **Plans as first-class versioned artifacts.** Multiple plan files
  (active, completed, technical-debt) checked into the repository,
  with the explicit principle "from the agent's point of view,
  anything it can't access in-context while running effectively
  doesn't exist. Knowledge that lives in Google Docs, chat threads,
  or people's heads are not accessible to the system." (Pattern 7 in
  PR #2783.)
- **Repository as single source of record.** Git substrate: commits
  append, ephemeral worktrees torn down but history preserved. The
  repository *is* the state; nothing important lives outside it.
- **AGENTS.md as table-of-contents to deeper docs.** The harness's
  small entry-point pattern: AGENTS.md (~100 lines) extended by the
  structured `docs/` directory plus mechanical-enforcement layer
  plus per-task ephemeral worktrees.
- **"Humans steer. Agents execute" thesis.** Mechanical enforcement
  layers (custom linters, CI checks) provide the deterministic
  scaffolding within which the LLM-driven agent loop runs.
- **Mechanical enforcement of behavioral constraints.** Custom
  linters with agent-readable error messages; "golden principles"
  as mechanically-checked. Rule violations surface as actionable
  diagnostics rather than soft documentation. (Patterns 8/9/12 in
  PR #2783.)
- **Entropy / AI slop as first-class engineering concern.** Named
  explicitly as engineering concern; paired with golden principles
  and a doc-gardening agent. The harness acknowledges agent-output
  quality drift as something requiring active mitigation
  infrastructure, not a one-time cleanup. (Patterns 11/12 in
  PR #2783.)
- **"One big AGENTS.md" anti-pattern.** The only explicitly named
  anti-pattern in the writeup; four failure mechanisms listed:
  context crowding, salience collapse, rot, unverifiability.
- **Depth-first capability accumulation.** Capabilities added
  iteratively as failures surfaced; not pre-designed. The harness
  shape is the residue of accumulated failures, not a top-down
  architectural plan.
- **High-throughput regime as scope condition on security stance.**
  *Single-system observation.* The throughput regime conditions when
  the security-stance pattern applies — see
  [`_notes/cycle-22-cross-system-synthesis.md`](../../_notes/cycle-22-cross-system-synthesis.md)
  for the throughput-vs-security trade-off.

## Anchoring caveats

- **Single-organization writeup vs framework-or-product.** The
  evidence base is one organization's published reflection on its
  own internal harness. Compare LangGraph (a publicly-versioned
  framework) where claims can be cross-checked against code at
  specific SHAs. Patterns here are documented-as-claimed.
- **Internal context.** OpenAI's harness exists in a specific
  internal context (model-development team, internal compute
  budget, dedicated tooling). Patterns may carry internal-context
  assumptions that don't transfer to a small-team multi-cycle
  redesign.
- **Throughput regime.** Several patterns assume high agent-task
  throughput; the redesign's cron-driven cadence is much sparser.
  Patterns calibrated to high-throughput contexts may need
  re-calibration before transfer.

## To-be-completed

The dispatch deliverable (PR #2783) is structured around 13 named
patterns; this stub captures only the patterns that surfaced in
cross-system observations. A direct read of the writeup with full
pattern coverage and quoted-source-level citations is queued under
issue [#2781](https://github.com/EvaLok/schema-org-json-ld/issues/2781).
When that read lands, this file should grow to deep-dive depth.
