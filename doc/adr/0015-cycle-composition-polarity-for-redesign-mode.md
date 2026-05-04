# ADR 0015: Cycle Composition Polarity for Redesign Mode

## Status

Accepted (2026-05-04). Applies to redesign mode (active since 2026-04-26 per [#2741](https://github.com/EvaLok/schema-org-json-ld/issues/2741)). Implemented via prompt edits to `.github/workflows/orchestrator-redesign-prompt.xml` (Edit A: Phase 1 depth elaboration in SECTION 8; Edit B: new SECTION 9.6 `<cycle-composition-polarity>`). Corresponding directive issue: [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829).

## Context

Cycles 44–61 (eighteen consecutive cycles) all placed cold-reader-driven framework iteration in the substantive-focal slot. The design framework moved v1.5 → v1.22 in that window. No fresh Copilot research dispatch went out; the deeper Phase 1 reading queue did not advance.

Concrete state of the Phase 1 reading queue at cycle 61:

- `oh-my-codex` is explicitly stub status; deeper orchestrator-direct read queued under [#2782](https://github.com/EvaLok/schema-org-json-ld/issues/2782), never picked up.
- `pai` declares deferred deeper reads of `Tools/`, `Packs/`, `.claude/`, `Releases/v4.0.3/`; never executed.
- `voyager` had only a cycle-17 orchestrator-direct read, less depth than the Copilot dispatches done for `autogen` and `langgraph`.
- AutoGen and LangGraph have Copilot deep-dives (PRs [#2763](https://github.com/EvaLok/schema-org-json-ld/pull/2763), [#2768](https://github.com/EvaLok/schema-org-json-ld/pull/2768)) but their implications for the redesign have not been mined; no cross-system synthesis writeup exists pulling from `autogen` + `cognition-devin` + `langgraph` + `openai-harness` + `openclaw`.

The cold-reader pattern is a closed loop: it generates its own next inputs from the framework it is polishing. Cycles 46 and 60 were already no-bump PASS — diminishing returns. Convergence on framework polish is not the same as convergence on understanding the problem.

The orchestrator's self-built cycle template treats cold-reader as the substantive default; structural inversion in the system prompt is the only durable fix (per `<self-modification-gates>`, prompt edits require a PR and Eva merge).

## Decision

Invert the substantive-focal polarity for Phase 1.

**Default substantive focal:** advance the research corpus, via one of:

1. Dispatch a deeper-read on a stub or explicitly-deferred Phase 1 system
2. Cross-system synthesis writeup comparing two or more systems already read at depth
3. v1-system retrospective mining — lessons not yet absorbed into the redesign
4. Implications mining from an existing per-system research note
5. Audit the design framework against fresh real-world evidence (system interaction or v1 incident)

**Fallback substantive focal:** cold-reader on the framework. Used only when no default option is viable. Cycle entries using fallback must explicitly state fallback status and name what blocked the default options.

**Implementation:** structurally absorbed into `.github/workflows/orchestrator-redesign-prompt.xml`:
- Phase 1 description (SECTION 8) gets a depth-elaboration paragraph cross-referencing the polarity section.
- New SECTION 9.6 `<cycle-composition-polarity>` makes the polarity canonical and per-cycle visible.

**Scope:** Phase 1 only. Phase 0 retrospective iteration remains governed by `<iteration-until-approval>`. Phase 2 and later get their own substantive shapes when they open; the polarity section will be reconsidered then.

## Consequences

### Positive

- Breaks the eighteen-cycle closed framework-iteration loop.
- Surfaces gaps in the stub and deferred-read systems.
- Tests the framework against a wider evidence base (more systems, cross-system synthesis, real cases).
- Cycles produce research progress, not just framework polish.
- The fallback clause preserves cold-reader's value as a drift-check tool without letting it dominate.

### Negative

- Dispatched deeper-reads return on Copilot's schedule, not the orchestrator's, so cycle output cadence becomes more variable.
- Framework polish accumulates fewer per-cycle increments.
- Synthesis and implications-mining work is harder to scope per-cycle than cold-reader's tight three-question shape.
- The "no substantive option is viable" judgment becomes a new orchestrator decision point, with risk of reverting to cold-reader by claiming all options are blocked. Mitigated by the requirement to name the block explicitly — auditable in the cycle entry.

### Trade-offs

- Tighter framework polish traded for broader evidence base.
- Orchestrator autonomy in cycle composition reduced (the menu is constrained, fallback requires explicit justification) in exchange for breaking the local-optimum loop.
- Cycle predictability traded for substantive variety.

## Alternatives Considered

**B — Unblock Phase 2 instead.** Rejected: shifts the local optimum from framework polish to premature implementation; doesn't fix the underlying "default is too narrow" pattern. The retrospective is not yet at a state where Phase 2 candidate generation would be load-bearing.

**C — Eva manually picks the substantive focal each cycle.** Rejected: doesn't scale, undermines the orchestrator's purpose, doesn't update the prompt's structural defaults. The next operator (or the next reset) would see the same closed loop reassemble itself.

**D — Hard-stop cold-reader entirely.** Rejected: cold-reader is genuinely valuable as a drift-check on the framework and as a fallback when expansion is blocked. The problem is its default position, not its existence.

**E — Add a per-cycle review-agent dispatch instead of changing default focal.** Rejected: adds a new gate without addressing the pattern that generates the cycle template in the first place. The orchestrator would still default to cold-reader because that is what is structurally easiest; the new gate would just become overhead.

## Cross-references

- Issue [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829) — input-from-eva directive announcing this polarity.
- Cycle issues [#2812](https://github.com/EvaLok/schema-org-json-ld/issues/2812)–[#2828](https://github.com/EvaLok/schema-org-json-ld/issues/2828) — the eighteen-cycle closed-loop window.
- Most recent cycle artifact: `docs/redesign/_notes/cycle-61-cold-reader-counting-fix-and-status-header-lens.md`.
- Per-system research files: `docs/redesign/1-research/systems/`.
- ADR 0011 — Pipeline Stabilization Program. Predecessor; redesign mode supersedes its operational scope.
- `.github/workflows/orchestrator-redesign-prompt.xml` — `<self-modification-gates>` (PR-required mechanism), Phase 1 description (SECTION 8), `<cycle-composition-polarity>` (new SECTION 9.6).
- Precedent for prompt-modification PRs: PR [#2740](https://github.com/EvaLok/schema-org-json-ld/pull/2740) (the prompt-installation PR), commit `06c799cd` (2026-04-27).
