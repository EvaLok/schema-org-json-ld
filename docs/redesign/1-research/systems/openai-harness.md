# OpenAI harness-engineering writeup

[← back to Phase 1 index](../../1-research.md)

**Status: deeper read landed (cycle 41).** Initial Copilot research dispatch
landed in cycle 26 as PR
[#2783](https://github.com/EvaLok/schema-org-json-ld/pull/2783) (closed cycle
41, superseded). Cycle-38/39/40 re-dispatch sequence (firewall expanded by Eva
[#2794](https://github.com/EvaLok/schema-org-json-ld/issues/2794)) culminated
in cycle-40 close-and-recreate
([#2803](https://github.com/EvaLok/schema-org-json-ld/issues/2803)) and
cycle-41 deliverable in PR
[#2805](https://github.com/EvaLok/schema-org-json-ld/pull/2805) at
`docs/redesign/_notes/cycle-38-openai-harness-deeper-read.md` (780 lines).
Cycle-41 per-finding evaluation:
[`_notes/cycle-41-deeper-read-per-finding-evaluation.md`](../../_notes/cycle-41-deeper-read-per-finding-evaluation.md).

## Source-access status

The deeper read encountered a new block type distinct from cycle-26's firewall
block: `openai.com/*` URLs return HTTP 403 (Cloudflare anti-bot for JS-rendered
pages) despite firewall allowlist expansion. The full primary text of the
"Harness engineering" writeup is available via the
`celesteanders/harness` GitHub mirror (same mirror used by cycle-26).

The companion post "Unrolling the Codex agent loop" (Bolin, January 23, 2026)
remains 403 from openai.com and unmirrored. Recovered via secondary synthesis
only — no verbatim quotes. Maintains a reduced gap-flag for full primary access.

OpenAI Agents SDK / Responses API documentation: still 403; Lens 3 (API-layer
context management) and Lens 4 (tool schema from SDK) remain partial.

## Sources read

- "Harness engineering: leveraging Codex in an agent-first world" — Ryan
  Lopopolo (OpenAI, February 11, 2026), via celesteanders/harness mirror.
  Verbatim transcript.
- "Unrolling the Codex agent loop" — Michael Bolin (OpenAI, January 23, 2026),
  via secondary synthesis (web search).

## Load-bearing thesis

The writeup is a field report. Empirical anchor: an internal beta product,
~1M lines of code, ~1500 PRs, five months, three to seven engineers — zero
manually written lines.

**Stated thesis (bold in original):** "Humans steer. Agents execute."

**Causal argument:**
> "Early progress was slower than we expected, not because Codex was incapable,
> but because the environment was underspecified. The agent lacked the tools,
> abstractions, and internal structure required to make progress toward
> high-level goals."

**Prescriptive claim:**
> "When something failed, the fix was almost never 'try harder.' [...] human
> engineers always stepped into the task and asked: 'what capability is missing,
> and how do we make it both legible and enforceable for the agent?'"

**Closing frames:**
> "Building software still demands discipline, but the discipline shows up
> more in the scaffolding rather than the code."
>
> "In an agent-first world, code becomes a disposable artifact — human time
> and attention, not lines of code, are the organization's scarcest resource."

The thesis is empirical (field-report induction), not theoretical. No
formalized "elicitation" concept, no taxonomy of context window strategies,
no agent-loop discipline as a named term. The agent loop is described
procedurally.

## Harness components and structure (concrete directory tree)

The primary source includes the actual `docs/` directory tree of the
repository knowledge base. Reproduced from the writeup:

```
AGENTS.md
ARCHITECTURE.md
docs/
├── design-docs/
│   ├── index.md
│   ├── core-beliefs.md
│   └── ...
├── exec-plans/
│   ├── active/
│   ├── completed/
│   └── tech-debt-tracker.md
├── generated/
│   └── db-schema.md
├── product-specs/
│   ├── index.md
│   ├── new-user-onboarding.md
│   └── ...
├── references/
│   ├── design-system-reference-llms.txt
│   ├── nixpacks-llms.txt
│   ├── uv-llms.txt
│   └── ...
├── DESIGN.md
├── FRONTEND.md
├── PLANS.md
├── PRODUCT_SENSE.md
├── QUALITY_SCORE.md
├── RELIABILITY.md
└── SECURITY.md
```

Notable named patterns:

- **Three-tier plan lifecycle as filesystem layout:** `exec-plans/active/`,
  `exec-plans/completed/`, `tech-debt-tracker.md`. Concrete versioned
  artifacts, not abstractions.
- **`*-llms.txt` reference files:** purpose-built LLM-consumption files for
  external dependencies (nixpacks, uv, design system). NOT raw docs — compressed
  and restructured for agent readability. The `-llms.txt` suffix names a
  distinct artifact type.
- **`docs/design-docs/core-beliefs.md`:** named artifact defining agent-first
  operating principles.
- **`QUALITY_SCORE.md`:** "grades each product domain and architectural layer,
  tracking gaps over time." First-class repository-resident quality artifact.
- **`generated/db-schema.md`:** generated artifacts checked into the repo as
  versioned documentation.

## Context, state, and memory (primary depth lens)

**The "one big AGENTS.md" anti-pattern with 4 named failure modes:**
- **Context crowding** — a giant instruction file crowds out task, code, docs.
- **Salience collapse** — when everything is "important," nothing is.
- **Rot** — monolithic manuals become graveyards of stale rules.
- **Unverifiability** — single blob doesn't lend itself to mechanical checks.

Each is mechanistically named and tied to specific consequence. These are the
most concrete, independently-evaluable claims in the writeup.

**Progressive disclosure:** AGENTS.md (~100 lines) → docs/ structure. "Agents
start with a small, stable entry point and are taught where to look next,
rather than being overwhelmed up front." Operationalized as filesystem layout.

**State as repository-only:**
> "From the agent's point of view, anything it can't access in-context while
> running effectively doesn't exist. Knowledge that lives in Google Docs, chat
> threads, or people's heads are not accessible to the system. Repository-local,
> versioned artifacts (e.g., code, markdown, schemas, executable plans) are
> all it can see."

The sharpest articulation of repository-as-state in any surveyed system.

**External-knowledge curation as memory posture:** the `*-llms.txt` pattern is
not "put raw docs in the repo" but "compress and restructure external
documentation for agent readability, then version it." Memory is shaped, not
just stored.

**Mechanical freshness enforcement:**
> "Dedicated linters and CI jobs validate that the knowledge base is up to
> date, cross-linked, and structured correctly. A recurring 'doc-gardening'
> agent scans for stale or obsolete documentation that does not reflect the
> real code behavior and opens fix-up pull requests."

Context freshness as engineering problem with automated solution, not
discipline problem.

**No embedding-based retrieval, no vector stores, no semantic memory.** The
docs/ approach is pointer-based: agents load what AGENTS.md points them to.
Re-grounding across sessions happens by keeping the repository current.
Single sessions reportedly run "upwards of six hours."

**Context compaction (companion post, secondary synthesis):**
- Two compaction triggers: pre-turn (context at threshold before sending), and
  mid-turn (during tool call chains if limit breached).
- Two compaction paths: OpenAI-hosted via `POST /v1/responses/compact` →
  encrypted opaque summary returned to client (server preserves structured
  metadata, tool call chains, model state); other providers via local
  compaction with dedicated LLM.
- **Prompt role hierarchy:** System > Developer > User > Assistant.

## Tool / skill integration model

- **Direct tool stack:** `gh`, local scripts, custom linters, "repository-
  embedded skills" (referenced but not detailed).
- **Observability stack per worktree:** ephemeral, with LogQL/PromQL/TraceQL.
- **Chrome DevTools Protocol** for UI validation.
- **Ephemeral worktrees:** one isolated environment per change, torn down
  after completion.
- **Mechanical enforcement layer:** custom linters with agent-readable error
  messages, plus CI jobs. "Most of these can be reviewed in under a minute
  and automerged."
- **Two concrete golden-principle examples:**
  - prefer shared utility packages
  - no YOLO-style data probing

## Anti-patterns and explicit non-goals

- **Monolithic AGENTS.md** (the only explicitly named anti-pattern, with 4
  failure modes).
- **Knowledge outside the repository.** Google Docs, Slack messages, tacit
  human knowledge — "doesn't exist" from the agent's point of view unless
  encoded into the codebase as markdown.
- **Heavy/abstract dependencies that resist in-repo modeling.** "Boring
  technologies" preferred for composability, API stability, training-set
  representation.

## Loop discipline (counter-evidence for Axis 9)

The agent-to-agent review loop is explicitly named in the writeup as the
**"Ralph Wiggum Loop"**:

- Coding agent makes change → reviewer agents critique → coding agent revises
  → loop until all reviewers satisfied.
- **No explicit iteration ceiling.** The loop runs until convergence; the
  human backstop is the only effective bound.
- The name itself acknowledges the pathology potential.

This is **counter-evidence for Axis 9 (iteration ceilings)** for cron-driven
autonomous systems. The OpenAI harness operates at high throughput with
synchronous human availability as the implicit bound. A cron-driven autonomous
system has no human backstop, so unbounded loops would be pathological and
the pattern does NOT transfer.

## Other named artifacts and patterns

- **Aardvark agent** — mentioned once as another agent operating on the
  codebase. Not elaborated. Multi-agent setup hint (single reference, cannot
  shift Axis 1 position).
- **Plans as first-class artifacts** with three-tier lifecycle.
- **Background doc-gardening agent** (named recurring agent for entropy
  mitigation).
- **Quality grading agent** (background cadence scans updating
  QUALITY_SCORE.md).
- **Depth-first capability accumulation:** "Working depth-first: breaking
  down larger goals into smaller building blocks ... and using them to unlock
  more complex tasks." The harness is emergent (5 months of accumulation),
  not designed.

## Phase 2 framework anchoring

Cycle-41 deeper-read framework anchor evidence (no v1.5 row updates required —
the framework already attributes these positions to OpenAI; deeper read
strengthens evidence with primary-source citation):

- **Axis 2 (State representation) → "Repository-as-state": STRENGTHENED.**
  Concrete directory tree. "Anything it can't access in-context doesn't exist"
  exact quote.
- **Axis 3 (Memory subsystem) → "Repository-as-record": STRENGTHENED AND
  EXTENDED.** `*-llms.txt` pattern surfaces external-knowledge-curation as
  memory posture (memory is shaped, not just stored).
- **Axis 4 (History substrate) → "Git-as-substrate": CONFIRMED.** No new
  evidence beyond cycle-26.
- **Axis 5 (Plans-as-artifacts) → STRONGLY STRENGTHENED.** Concrete directory
  tree showing `exec-plans/active/` + `exec-plans/completed/` +
  `tech-debt-tracker.md` is the strongest Phase 1 evidence for this position.
- **Axis 8 (Mechanical enforcement) → CONFIRMED AND EXTENDED.** Two concrete
  golden-principle examples + QUALITY_SCORE.md add per-domain quality grading
  dimension.
- **Axis 10 (Entropy mitigation) → CONFIRMED AND EXTENDED.** Same evidence as
  Axis 8 plus tech-debt-tracker.md as direct evidence.
- **Axis 9 (Iteration ceilings) → COUNTER-EVIDENCE.** Ralph Wiggum Loop has
  no iteration ceiling; pattern does NOT transfer to cron-driven systems
  without human backstop.
- **Axis 13 (Harness-vs-session) → fat-harness CONFIRMED.** Custom linters,
  CI jobs, doc-gardening agent, ephemeral worktrees, observability stack per
  worktree. Substantial deterministic-tooling surface area.

## Anchoring caveats

- **Single-organization writeup vs framework-or-product.** Evidence base is
  one organization's published reflection on its own internal harness. Not
  cross-checkable against code at specific SHAs (unlike LangGraph / AutoGen).
  Patterns are documented-as-claimed.
- **Internal context.** OpenAI's harness operates in a model-development team
  with internal compute budget and dedicated tooling. Patterns may carry
  internal-context assumptions.
- **High-throughput regime.** Several patterns (Ralph Wiggum Loop, automerge
  rate, doc-gardening cadence) assume high agent-task throughput. The
  redesign's cron-driven cadence is much sparser; pattern transfer requires
  recalibration.
- **Marketing/credibility framing.** Specific numbers (1M lines, 1500 PRs,
  3.5 PRs/engineer/day) are reported by Cognition team-equivalent for OpenAI;
  selection bias on which projects/timeframes are discussed is real. The
  authors explicitly caveat from within: "should not be assumed to generalize
  without similar investment."
- **Source-access asymmetry.** Companion post (Bolin) and OpenAI Agents SDK
  docs remain 403-blocked at content-delivery layer despite firewall
  expansion. Some patterns (context compaction implementation, tool schemas)
  carry secondary-source caveats.

## Patterns observed (catalog)

(Relevance evaluation for v2 candidate generation deferred to Phase 2.)

- "Humans steer. Agents execute." as role-allocation thesis
- "Code becomes a disposable artifact" as scarcity-frame inversion
- AGENTS.md (~100 lines) as table-of-contents to deeper docs
- Progressive disclosure operationalized as filesystem layout
- Three-tier plan lifecycle (active/completed/tech-debt) as filesystem
- `*-llms.txt` as named LLM-consumption artifact type
- Repository-as-state ("anything not in-context doesn't exist")
- QUALITY_SCORE.md as first-class quality artifact
- Mechanical enforcement via custom linters + CI jobs
- Doc-gardening agent for entropy mitigation
- Quality-grading agent for QUALITY_SCORE.md updates
- Four named context-management failure modes (crowding, salience, rot,
  unverifiability)
- Ralph Wiggum Loop (named agent-review-loop with no iteration ceiling)
- Aardvark (named additional agent, not elaborated)
- Boring-technology preference for agent-context modeling
- Ephemeral per-task worktrees with per-worktree observability
- Depth-first capability accumulation as harness-shape origin
- Context compaction (companion post, secondary synthesis):
  pre-turn + mid-turn triggers; OpenAI-hosted vs local paths;
  System > Developer > User > Assistant role hierarchy
