# Cycle 38: OpenAI harness-engineering deeper read (primary-source access attempt)

## Source status

**Primary URL:** `https://openai.com/index/harness-engineering/` — returns HTTP 403 in this
sandbox despite the firewall allowlist expansion (Eva directive #2794). The 403 is a
Cloudflare/JS-rendering anti-bot response, not a firewall block. The domain is reachable
(DNS resolves, TCP connects) but the page requires browser rendering to serve content.
This is a new block type distinct from the cycle-26 firewall block; flagged per dispatch
constraints.

**Full text retrieved via GitHub mirror:**
`https://github.com/celesteanders/harness/blob/main/docs/research/260211_openai_harness_engineering_codex.md`
— accessible, returns the full verbatim transcript of the OpenAI post. Confirmed authentic
by cross-check with independent secondary summaries. **This is the same mirror used by
cycle-26; the primary-source text is fully available through it.** The gap cycle-26 had
was not content-gap from the mirror but context-gap from related sources.

**Companion post:** `https://openai.com/index/unrolling-the-codex-agent-loop/` (Michael
Bolin, January 23, 2026) — still returns HTTP 403 for the same Cloudflare reason. Full
text unavailable. A secondary synthesis was recovered via web search:
- Agent loop structure: user input → prompt assembly → Responses API → tool call handling
  → loop until final response
- Context compaction: two trigger points (pre-turn and mid-turn), two paths (OpenAI-hosted
  via POST /v1/responses/compact producing encrypted summary; other providers via local
  compaction with dedicated LLM)
- Prompt role hierarchy: System > Developer > User > Assistant (decreasing priority)
- This fills the Lens 3 context-compaction gap from cycle-26 (partial; not verbatim)

**OpenAI Agents SDK / Responses API documentation:** still inaccessible (403 for all
`openai.com/*` URLs). Content gap for Lens 3 (API-layer context management) and Lens 4
(tool schema from SDK) persists.

**Sources read:**
- Mirror transcript of "Harness engineering: leveraging Codex in an agent-first world,"
  Ryan Lopopolo (OpenAI, Feb 11, 2026). All verbatim quotes below are from this source.
- Web-search secondary synthesis of "Unrolling the Codex agent loop," Michael Bolin
  (OpenAI, Jan 23, 2026). No verbatim quotes from this post — secondary descriptions only.

**Vs cycle-26:** cycle-26's mirror transcript was complete. The additional content this
deeper read provides comes from (a) extracting the concrete directory tree the primary
source includes, (b) noting specific named patterns cycle-26 summarized without quoting,
(c) partially filling the companion-post gap via secondary synthesis. The upgrade is
extraction depth and companion-post coverage, not a different primary text.

---

## 1. Overall philosophy and load-bearing thesis

The writeup is a field report. Its empirical anchor: an internal beta product, ~1M lines
of code, ~1,500 PRs, over five months, with three to seven engineers — zero manually
written lines at any point.

The stated thesis — set in **bold** in the original — is:

> "Humans steer. Agents execute."

This is a role-allocation frame. The harness is what makes the division operational.

The load-bearing causal argument:

> "Early progress was slower than we expected, not because Codex was incapable, but
> because the environment was underspecified. The agent lacked the tools, abstractions,
> and internal structure required to make progress toward high-level goals."

From this follows the central prescriptive claim:

> "When something failed, the fix was almost never 'try harder.' Because the only way to
> make progress was to get Codex to do the work, human engineers always stepped into the
> task and asked: 'what capability is missing, and how do we make it both legible and
> enforceable for the agent?'"

Agent failure is environment failure. The correct response is environment improvement —
add tools, add structure, add documentation — not model prompting or repeated retries.
The writeup treats "try harder" as explicitly wrong.

A third load-bearing sentence closes the post:

> "Building software still demands discipline, but the discipline shows up more in the
> scaffolding rather than the code."

And a fourth framing:

> "In an agent-first world, code becomes a disposable artifact — human time and attention,
> not lines of code, are the organization's scarcest resource."

This last sentence is the strongest version of the thesis: the frame inverts the scarcity
assumption of conventional software (where code is the output being optimized) in favor
of human attention as the binding constraint.

**What the writeup does NOT theorize:** No formal concept of "elicitation." No "agent loop
discipline" as a named term. No taxonomy of context window strategies. The agent loop is
described procedurally, not theorized. The philosophy is more empirical than principled —
a field-report induction, not a theoretical design manifesto.

**Marketing assessment:** The evidence base is specific (numbers: 1M lines, 1500 PRs, 3.5
PRs/engineer/day, throughput increasing with team growth). Selection bias is real — failed
experiments before this five-month window are not reported. The authors voluntarily caveat
their own claims: "This behavior depends heavily on the specific structure and tooling of
this repository and should not be assumed to generalize without similar investment." That
caveat weakens the marketing frame from within.

**Vs cycle-26:** cycle-26 correctly identified the core thesis and three load-bearing
sentences. The deeper read adds the "code becomes a disposable artifact" formulation,
which cycle-26 did not quote. Convergence on the central argument.

---

## 2. Harness components and structure

The primary source includes the actual directory tree of the repository knowledge base —
a concrete artifact cycle-26 described in prose but did not reproduce:

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

*In-repository knowledge store layout — directly from the primary source.*

Notable features of this layout:

- `exec-plans/active/`, `exec-plans/completed/`, `tech-debt-tracker.md` — three-tier plan
  lifecycle as concrete filesystem structure, not abstraction
- `references/` — external library docs as `*-llms.txt` files: nixpacks, uv, design system.
  These are purpose-built LLM-consumption files (restructured for agent readability), not
  raw documentation. The `-llms.txt` suffix is deliberate: they are a different artifact
  type from human-facing docs. This is a significant pattern cycle-26 did not surface.
- `docs/design-docs/core-beliefs.md` — a file named "core beliefs" defining agent-first
  operating principles. Named artifact type.
- `QUALITY_SCORE.md` — "grades each product domain and architectural layer, tracking gaps
  over time." A quality-tracking artifact as first-class repository resident.
- `generated/db-schema.md` — generated artifacts checked into the repo (schema as
  versioned documentation).

The harness components assembled from the writeup:
- AGENTS.md (~100 lines) as context-injection map
- `docs/` structured knowledge base as territory
- Mechanical enforcement layer: custom linters + CI jobs (Codex-generated)
- Development tools used directly: `gh`, local scripts, "repository-embedded skills"
- Observability stack: per-worktree ephemeral stack, LogQL / PromQL / TraceQL
- Chrome DevTools Protocol integration for UI validation
- Agent-to-agent review loop ("Ralph Wiggum Loop" — the writeup's own name)
- Ephemeral worktrees: one isolated environment per change, torn down after completion
- Golden principles + background doc-gardening agent
- Quality grading agent (background cadence scans updating `QUALITY_SCORE.md`)
- A second named agent: "Aardvark" — mentioned once as "other agents (e.g. Aardvark) that
  are working on the codebase." Not elaborated. *Documented-claim: multi-agent setup.*

The harness is framed as emergent, not designed:
> "working depth-first: breaking down larger goals into smaller building blocks ... and
> using them to unlock more complex tasks."

The composability is temporal (five months of depth-first accumulation), not architectural.
The writeup does not draw a formal boundary between harness and model.

**Vs cycle-26:** cycle-26 described the structure in prose. The deeper read adds the
concrete directory tree, the `*-llms.txt` pattern (cycle-26's "generated references" gloss
missed the LLM-specificity), the QUALITY_SCORE.md artifact, and the Ralph Wiggum Loop
name. These are load-bearing additions.

---

## 3. Context, state, and memory

*Primary depth lens.*

### Context window management: the "one big AGENTS.md" failure

The writeup frames context management as the central practical challenge:

> "Context management is one of the biggest challenges in making agents effective at large
> and complex tasks. One of the earliest lessons we learned was simple: give Codex a map,
> not a 1,000-page instruction manual."

Four named failure modes of the monolithic-instructions approach:

1. **Context crowding** — "A giant instruction file crowds out the task, the code, and the
   relevant docs—so the agent either misses key constraints or starts optimizing for the
   wrong ones."
2. **Salience collapse** — "Too much guidance becomes non-guidance. When everything is
   'important,' nothing is. Agents end up pattern-matching locally instead of navigating
   intentionally."
3. **Rot** — "A monolithic manual turns into a graveyard of stale rules. Agents can't tell
   what's still true, humans stop maintaining it, and the file quietly becomes an attractive
   nuisance."
4. **Unverifiability** — "A single blob doesn't lend itself to mechanical checks (coverage,
   freshness, ownership, cross-links), so drift is inevitable."

Each failure mode is mechanistically named and tied to a specific consequence. These are
the most concrete, independently-evaluable claims in the writeup.

### Progressive disclosure

> "agents start with a small, stable entry point and are taught where to look next, rather
> than being overwhelmed up front."

This is operationalized — not just described — via the AGENTS.md → docs/ structure. The
directory tree above is the implementation of progressive disclosure as filesystem layout.

### State as repository-only

> "From the agent's point of view, anything it can't access in-context while running
> effectively doesn't exist. Knowledge that lives in Google Docs, chat threads, or people's
> heads are not accessible to the system. Repository-local, versioned artifacts (e.g.,
> code, markdown, schemas, executable plans) are all it can see."

This is the sharpest framing of repository-as-state. The writeup uses a diagram described
as "The limits of agent knowledge: What Codex can't see doesn't exist" — Codex's knowledge
shown as a bounded bubble with Google Docs, Slack messages, and tacit human knowledge
below (outside the bubble), accessible only if encoded into the codebase as markdown.

### LLM-targeted reference files

The `references/` directory contains files like `nixpacks-llms.txt`, `uv-llms.txt`,
`design-system-reference-llms.txt`. The writeup:

> "We favored dependencies and abstractions that could be fully internalized and reasoned
> about in-repo. Technologies often described as 'boring' tend to be easier for agents to
> model due to composability, api stability, and representation in the training set."

And where a dependency is complex, the pattern is to generate an in-repo, agent-readable
version of its documentation: the `*-llms.txt` files. This is not "put raw docs in the
repo" but "compress and restructure external documentation for agent readability, then
version it." Cycle-26 noted "generated references" briefly; the deeper read surfaces this
as a distinct named pattern.

### Plans as first-class repository artifacts

> "Plans are treated as first-class artifacts. Ephemeral lightweight plans are used for
> small changes, while complex work is captured in execution plans with progress and
> decision logs that are checked into the repository. Active plans, completed plans, and
> known technical debt are all versioned and co-located, allowing agents to operate without
> relying on external context."

The directory tree makes this concrete: `exec-plans/active/`, `exec-plans/completed/`,
`tech-debt-tracker.md`. Three tiers, one directory, versioned in-repo.

### Mechanical freshness enforcement

> "We enforce this mechanically. Dedicated linters and CI jobs validate that the knowledge
> base is up to date, cross-linked, and structured correctly. A recurring 'doc-gardening'
> agent scans for stale or obsolete documentation that does not reflect the real code
> behavior and opens fix-up pull requests."

Context freshness is not a discipline problem; it is an engineering problem with an
automated solution.

### Long-term memory and re-grounding

No embedding-based retrieval, no vector stores, no semantic memory. The `docs/` approach
is pointer-based: agents load what they're pointed to by AGENTS.md. Re-grounding across
sessions happens by keeping the repository current. The writeup notes single runs lasting
"upwards of six hours" — suggesting single long sessions rather than multi-session
continuation with cold-start re-grounding.

### Agent loop and context compaction (companion post — secondary synthesis)

"Unrolling the Codex agent loop" (Bolin, Jan 2026) describes:
- Agent loop: input → prompt construction → Responses API → tool call handling → loop
  until final message
- Prompt role hierarchy: System (highest priority) > Developer > User > Assistant
- **Context compaction triggers:** (1) pre-turn: if context at threshold before sending
  new message; (2) mid-turn: during multi-stage tool call chains if limit breached mid-loop
- **Compaction paths:**
  - OpenAI-hosted: `POST /v1/responses/compact` → encrypted opaque summary returned to
    client; server preserves structured metadata, tool call chains, model state
  - Other providers: local compaction via dedicated LLM + compaction prompt (slower)
- The encrypted compaction output prevents tampering and prompt injection via replay

*These descriptions are secondary-synthesis, not verbatim from the companion post. Treat
as documented-claim with lower confidence than primary-source quotes.*

The key gap cycle-26 flagged ("relationship between context patterns and SDK's context
management APIs") is partially filled: compaction is an API-level feature (POST endpoint),
but the harness-level patterns (progressive disclosure, plan artifacts) are orthogonal to
it. The patterns work at any context window size; compaction is the backstop for when
context still overflows.

**Vs cycle-26:** substantial. The `*-llms.txt` pattern is new. The concrete plan-lifecycle
directory paths are new. The companion-post context-compaction description partially fills
the flagged gap. The "boring technology" preference as an agent-context argument is new.

---

## 4. Tool definition, invocation, and trust boundary

The primary source is thin on tool theory. Tools are described as things that exist and
are used, not as a design concern in their own right.

**Named tools and integration points:**
- `gh` (GitHub CLI)
- Local scripts
- "Repository-embedded skills" (the phrase implies skills that live in the repo, not
  external registries; invocation mechanism unspecified — *documented-claim*)
- Chrome DevTools Protocol: "wired into the agent runtime" with skills for DOM snapshots,
  screenshots, navigation
- LogQL / PromQL / TraceQL APIs via ephemeral observability stack

> "Codex uses our standard development tools directly (gh, local scripts, and
> repository-embedded skills) to gather context without humans copying and pasting into
> the CLI."

**Trust boundary:** wide. Agents push code, merge PRs, execute shell commands, query live
observability. Worktree isolation is the practical boundary — each change gets an
isolated environment, but within that worktree the agent has broad write access.

**Tool schema, naming, count:** not theorized. The writeup implies more tools = more
capability, with no counter-guidance on over-tooling or clarity constraints.

**MCP:** the word does not appear in the primary source. Secondary summaries interpolate
it; this appears to be a secondary-source addition.

**Vs cycle-26:** no material difference. The primary source is still thin here.
Cycle-26 accurately identified this as a thin section.

---

## 5. Failure handling and reliability

The central failure-handling philosophy:

> "When something failed, the fix was almost never 'try harder.' Because the only way to
> make progress was to get Codex to do the work, human engineers always stepped into the
> task and asked: 'what capability is missing, and how do we make it both legible and
> enforceable for the agent?'"

Failure is treated as a signal about the environment, not the model.

**Agent-to-agent review loop ("Ralph Wiggum Loop"):**

> "we instruct Codex to review its own changes locally, request additional specific agent
> reviews both locally and in the cloud, respond to any human or agent given feedback, and
> iterate in a loop until all agent reviewers are satisfied (effectively this is a Ralph
> Wiggum Loop)"

"Ralph Wiggum Loop" (a self-aware reference: The Simpsons' Ralph Wiggum saying "I'm
helping!" in a circular, non-terminating fashion) signals that the authors know this
loop can be pathological — it continues until all reviewers are satisfied, with no
termination guarantee if agents never converge. The name is a documented acknowledgment
of the failure mode, not a denial of it.

**Quality grading:** `QUALITY_SCORE.md` "grades each product domain and architectural
layer, tracking gaps over time." Background agents update these grades on a recurring
cadence. Quality is a tracked artifact, not a subjective judgment.

**Entropy / garbage collection:**

> "Codex replicates patterns that already exist in the repository—even uneven or suboptimal
> ones. Over time, this inevitably leads to drift."

The writeup treats entropy as an engineering certainty, not a risk to be managed:

> "Our team used to spend every Friday (20% of the week) cleaning up 'AI slop.'
> Unsurprisingly, that didn't scale."

The solution: golden principles + recurring background agents that scan for deviations,
update quality grades, and open targeted refactoring PRs. "Most of these can be reviewed
in under a minute and automerged."

**Build failures:** listed in step 9 of the 11-step autonomy list ("Detect and remediate
build failures") without mechanism description. *Documented-claim.*

**Escalation:** step 10 is "Escalate to a human only when judgment is required." The
writeup does not specify what triggers escalation or how the agent determines "judgment is
required." This is the thinnest failure-handling description in the writeup.

**Throughput trade-off:**

> "Test flakes are often addressed with follow-up runs rather than blocking progress
> indefinitely. In a system where agent throughput far exceeds human attention, corrections
> are cheap, and waiting is expensive. This would be irresponsible in a low-throughput
> environment. Here, it's often the right tradeoff."

This is a throughput argument disguised as a reliability argument. The writeup correctly
scopes it: "irresponsible in a low-throughput environment." The conditional matters.

**What the writeup does NOT address:** hallucinated tool calls, infinite agent loops,
malformed output, prompt injection via untrusted inputs, how the harness detects model
misbehavior vs environment misbehavior.

**Vs cycle-26:** the Ralph Wiggum Loop name and quality grading via QUALITY_SCORE.md are
new. The escalation thinness was correctly identified by cycle-26.

---

## 6. Anti-patterns and explicit non-goals

**The only explicitly named anti-pattern:** "one big AGENTS.md" — "We tried the 'one big
AGENTS.md' approach. It failed in predictable ways..." (four failure modes in Lens 3).

**Explicit anti-patterns named within golden principles:**

> "(1) we prefer shared utility packages over hand-rolled helpers to keep invariants
> centralized, and (2) we don't probe data 'YOLO-style'—we validate boundaries or rely on
> typed SDKs so the agent can't accidentally build on guessed shapes."

These are concrete rule formulations — implementable, checkable. The writeup doesn't just
say "have golden principles"; it gives two non-trivial examples. Cycle-26 mentioned golden
principles without quoting these examples.

**Other anti-patterns implied but not stated:**
- "Try harder" as failure response: explicitly wrong
- Manual cleanup cycles: "Unsurprisingly, that didn't scale" — the manual Friday cleanup
  is deprecated
- Off-repo knowledge: Google Docs, Slack, "people's heads" — effectively the anti-pattern
  of storing knowledge outside the agent's reach
- Opaque dependencies: "In some cases, it was cheaper to have the agent reimplement
  subsets of functionality than to work around opaque upstream behavior from public
  libraries." The anti-pattern: pulling in dependencies the agent cannot model.
- Human code aesthetics as optimization target: "The resulting code does not always match
  human stylistic preferences, and that's okay." Human taste is a non-goal for code style;
  agent legibility is the goal.
- Blocking merge gates at high throughput: "corrections are cheap, and waiting is
  expensive" (*scoped to high-throughput regime*)

**What the writeup explicitly does NOT address:**
- Over-tooling warnings
- Agent hierarchy patterns
- Security posture for adversarial inputs
- Multi-provider or multi-model strategies
- When agent-generated code is inappropriate (the writeup presupposes it's always
  appropriate in its constrained scope)

**Vs cycle-26:** the two golden-principle examples (shared utilities; no YOLO data probing)
and the boring-technology preference as explicit anti-pattern are new.

---

## 7. Anchoring caveats

For each difference: what patterns it discounts, and what transfers despite the difference.

**Difference 1: OpenAI's harnesses target their own models (Codex/GPT-5); the redesign
uses Claude.**

*Discounts:* Anything specific to Codex's instruction-following behavior, context window
characteristics, or tool-call reliability. The writeup makes no claims about which phrasings
work — these are model-specific. The companion post's compaction API is OpenAI-specific
(POST /v1/responses/compact is not available for third-party models on Claude).

*Transfers:* The environment-as-primary-variable argument: "Swap out the model—results
shift a bit. Change the harness—everything breaks or scales." (*Documented-claim; not
independently verified.*) Progressive disclosure, repository-as-state, mechanical
enforcement, and golden principles are harness-layer patterns with no model dependency.
The `*-llms.txt` pattern transfers directly: Claude consumes compressed reference files
just as Codex does.

**Difference 2: OpenAI is a model provider with first-party tooling; the redesign builds
on an external provider's API.**

*Discounts:* Chrome DevTools Protocol integration, per-worktree observability stacks,
Responses API context compaction endpoint — all require infrastructure or API access not
available via the Claude API. The agent can launch sub-processes and open PRs, but not
spin up isolated app instances with full observability.

*Transfers:* Repository structure patterns (AGENTS.md as table of contents, exec-plans/
lifecycle, `references/*-llms.txt` as compressed external docs, QUALITY_SCORE.md as
tracked artifact) require no first-party model access. Mechanical enforcement via CI (
custom linters + agent-readable error messages) is equally accessible. These are the most
directly replicable patterns.

**Difference 3: OpenAI's harness writeup partially serves its developer audience as
product positioning.**

*Discounts:* The "1/10th the time" productivity comparison is against an unverifiable
counterfactual. Throughput numbers (3.5 PRs/engineer/day) are single-team, single-project,
with no control. The selection of what five-month period to report (not the earlier, slower
periods) is self-serving. The writing positions Codex CLI and GPT-5 favorably by design.

*Transfers:* The qualitative failure-mode descriptions (four failure modes of monolithic
AGENTS.md; entropy-replication inevitability; manual cleanup not scaling) are specific
enough to evaluate independently. The authors' voluntary "should not be assumed to
generalize" caveat makes the claims more reliable as signals, not less — the marketing
frame is weakened from within.

**Difference 4: OpenAI's harness runs on human-triggered tasks; the redesign runs
autonomously on a cron.**

*Discounts:* The review-and-iterate pattern assumes a human is monitoring and can decide
when to merge. The Ralph Wiggum Loop has a human backstop. The escalation step ("escalate
to a human only when judgment is required") assumes a human is reachable. A cron system
needs fully automated escalation logic, which the writeup does not provide.

*Transfers:* Progressive disclosure, repository-as-state, doc-gardening, and golden
principles all apply regardless of trigger mechanism. The golden-principles pattern is
arguably *better suited* to a cron system: it encodes rules mechanically so they apply
everywhere without runtime judgment, which is precisely what a cron system without a
human backstop needs.

**Difference 5: OpenAI's harness targets a general-purpose product; the redesign has a
single narrow domain.**

*Discounts:* Observability stack, Chrome DevTools UI validation, end-to-end feature-ship
autonomy. These are product-specific and irrelevant to schema-domain work.

*Transfers:* Core knowledge-management patterns apply at any scope. Narrow scope makes
them cheaper to implement: fewer domains to document, fewer linting rules to write, fewer
plan categories to maintain. The `exec-plans/active/completed/tech-debt-tracker.md`
lifecycle is directly applicable.

**Difference 6: OpenAI's evaluation infrastructure is internal and undescribed.**

*Discounts:* Quality grading methodology, evaluation harness details, "100% test coverage"
claim for custom implementations. The redesign cannot replicate or inspect these.

*Transfers:* The principle — "encode evaluation criteria into the repository so agents can
run them" — is independent of the specific infrastructure. The doc-gardening pattern and
QUALITY_SCORE.md pattern are accessible without internal tooling.

---

## Phase 2 framework anchoring

Which axis positions in `docs/redesign/2-design-framework.md` does the deeper read
strengthen, weaken, or qualify? Cited by axis number.

**Axis 2 (State representation) — "Repository-as-state" position: STRENGTHENED.**
The concrete directory tree provides direct evidence. The `exec-plans/active/` and
`exec-plans/completed/` paths show that state (plan lifecycle) lives in versioned files.
The "anything not in-context doesn't exist" quote is the sharpest articulation of why
repository-as-state is a deliberate design choice, not a fallback.

**Axis 3 (Memory subsystem shape) — "Repository-as-record" position: STRENGTHENED AND
EXTENDED.** The `*-llms.txt` reference files add a dimension cycle-26 missed: not just
"repository is memory" but "compress and restructure external knowledge for agent
readability, then version it." This extends the repository-as-record position into an
active memory-curation posture: memory is shaped, not just stored. The companion post's
context compaction fills the "how does re-grounding work across long sessions" gap:
it's a backstop (compaction), not a re-grounding primitive — the primary re-grounding
mechanism remains the repository staying current.

**Axis 4 (History substrate) — "Git-as-substrate" position: CONFIRMED.** No new evidence
beyond cycle-26. The directory tree adds concrete structural evidence (exec-plans/
completed/ = git history + explicit completion records), but this confirms rather than
extends.

**Axis 5 (Plans-as-artifacts) — "Active/completed/technical-debt" lifecycle: STRONGLY
STRENGTHENED.** The directory tree is the most direct evidence in Phase 1 research that
this axis position is implemented, not just claimed. `exec-plans/active/`,
`exec-plans/completed/`, `tech-debt-tracker.md` are filesystem layout choices with
explicit lifecycle semantics. This is not a documented-claim — it is a published diagram
of an actual directory structure.

**Axis 8 (Mechanical enforcement) — "Behavioral promises + agent-affecting prose"
position: CONFIRMED AND EXTENDED.** The deeper read adds two concrete golden-principle
examples: (1) prefer shared utility packages; (2) no YOLO-style data probing. These are
checkable, lintable rules — the position is confirmed with concrete rule instantiation.
The QUALITY_SCORE.md artifact adds a quality-grading dimension: mechanical enforcement
extends beyond structural architecture to per-domain quality tracking.

**Axis 10 (Entropy mitigation) — "Golden principles + doc-gardening agent" position:
CONFIRMED AND EXTENDED.** Same evidence as Axis 8. The `tech-debt-tracker.md` file is
direct evidence. The specific golden-principle examples strengthen the position's
concreteness. The "most of these can be reviewed in under a minute and automerged" detail
confirms the pattern is designed for minimal human friction.

**Axes NOT materially moved:**
- Axis 1 (agent decomposition): Aardvark mention hints at multi-agent but is not
  elaborated. Cannot move the axis position from this single reference.
- Axis 6 (extension shape): "repository-embedded skills" as tool-invocation phrase adds
  color but doesn't change the axis position evidence.
- Axis 7 (orchestration topology): Ralph Wiggum Loop names the review-loop topology but
  doesn't add new topological choices to the axis.
- Axis 9 (iteration ceilings): Ralph Wiggum Loop is notable precisely because it has NO
  explicit ceiling — agents loop "until all agent reviewers are satisfied." The human
  backstop is the only effective ceiling. This weakens the Axis 9 case that high-
  throughput systems implement bounded loops from the OpenAI harness.
- Axis 12 (reconciliation): no new evidence. Still externally unvalidated.
- Axis 13 (harness-vs-session): the writeup's pattern is unambiguously fat-harness.
  Consistent with cycle-26.

**Counter-evidence:**
The Ralph Wiggum Loop (Axis 9) is the most notable counter-evidence: a system at high
throughput with no explicit loop count ceiling on agent review cycles. The human backstop
is the bound. For a cron-driven autonomous system, this pattern does NOT transfer — the
cron has no human backstop, so unbounded loops would be pathological.

---

## Comparison to cycle-26 PR #2783

**Where the deeper read confirms cycle-26's claims:**
- Core thesis ("humans steer, agents execute") — confirmed verbatim
- Four failure modes of monolithic AGENTS.md — confirmed verbatim
- Plans as first-class artifacts with three-tier lifecycle — confirmed with concrete paths
- Repository as single source of record — confirmed with "anything not in-context doesn't
  exist" exact quote
- AGENTS.md as table of contents (~100 lines) — confirmed
- Mechanical enforcement via custom linters with agent-readable error messages — confirmed
- Doc-gardening agent for entropy — confirmed
- Depth-first capability accumulation — confirmed
- Progressive disclosure — confirmed
- 18 patterns listed in cycle-26 — all confirmed

**Where the deeper read qualifies, extends, or fills gaps:**
- `*-llms.txt` pattern: cycle-26 noted "generated references" but missed the LLM-
  specificity. These are purpose-built agent-consumption files, not raw docs. Named
  pattern addition.
- Ralph Wiggum Loop: cycle-26 described the agent review loop but did not quote the name.
  The name is a self-aware acknowledgment of the pathology. Worth noting.
- Golden principles examples: cycle-26 mentioned golden principles as a category. The
  deeper read surfaces two concrete rule examples (shared utilities; no YOLO data probing).
- QUALITY_SCORE.md: cycle-26 mentioned "quality grades" in a list but didn't identify the
  specific file. Named artifact.
- Directory tree: cycle-26 described the structure in prose. The deeper read reproduces
  the actual diagram, which is load-bearing evidence for Axis 5.
- Companion post context compaction: cycle-26 explicitly flagged this gap. Partially
  filled (secondary synthesis, not verbatim).
- "Code becomes a disposable artifact": not quoted by cycle-26; adds a fourth load-bearing
  sentence to the thesis.
- Boring technology preference as agent-context argument: not surfaced by cycle-26.

**Where cycle-26's explicit content gaps remain open:**
- OpenAI Agents SDK documentation — still blocked (403). The gap for Lens 3 (context
  management API layer) and Lens 4 (tool schema from SDK) persists.
- HackerNews discussion — not attempted (would require news.ycombinator.com).
- "Unrolling the Codex agent loop" full text — still blocked; secondary synthesis fills
  the gap partially but not verbatim.

---

## Patterns observed in OpenAI's published material

Listed without v2-relevance framing. Sourced from the primary source mirror unless noted.

1. **Environment underspecification as primary failure mode** — agent failures attributed
   to the environment (tools, abstractions, structure) being underspecified, not to model
   limits.

2. **"Humans steer. Agents execute." as role-allocation thesis** — the harness enforces
   this role boundary.

3. **AGENTS.md as table of contents, not encyclopedia** — ~100 lines, injected into
   context, pointers only; territory is a separate structured `docs/` directory.

4. **Four named failure modes of monolithic instruction files** — context crowding,
   salience collapse, rot, unverifiability; each mechanistically named.

5. **Progressive disclosure** — agents start with a minimal stable entry point and are
   directed to deeper context only as needed.

6. **Repository as single source of record** — off-repo knowledge (Google Docs, Slack,
   people's heads) is illegible to the agent; "effectively doesn't exist."

7. **Plans as first-class versioned artifacts** — `exec-plans/active/`,
   `exec-plans/completed/`, `tech-debt-tracker.md` as concrete filesystem structure.

8. **LLM-targeted reference files** — external library/design-system documentation
   compressed and restructured as `*-llms.txt` files for agent readability.

9. **Mechanical enforcement over documented rules** — architectural constraints encoded as
   linters and CI checks; not text the agent is expected to follow voluntarily.

10. **Custom linters with agent-readable error messages** — error messages written to
    inject remediation instructions directly into agent context.

11. **Golden principles pattern** — concrete canonical rules versioned in-repo, mechanically
    enforced on a recurring cadence; includes specific examples: prefer shared utility
    packages; no YOLO-style data probing.

12. **QUALITY_SCORE.md as tracked quality artifact** — per-domain quality grades with gap
    tracking, updated by recurring background agents.

13. **Depth-first environment building** — capabilities added iteratively as failures
    surfaced; harness was not designed upfront.

14. **Entropy as first-class engineering concern** — agent systems replicate bad patterns;
    garbage collection via recurring cleanup agents treated as infrastructure, not one-time
    event.

15. **Agent-to-agent review loop ("Ralph Wiggum Loop")** — agents review their own and
    each other's changes in a loop until all agent reviewers are satisfied; human supervises
    at a higher abstraction level. Self-named acknowledgment of potential pathology.

16. **Ephemeral per-change isolation** — each change runs in an isolated worktree with its
    own app instance, observability stack, and logs.

17. **"Agent legibility" as the optimization target** — codebase structured for agent
    comprehension first; human aesthetic preferences explicitly secondary.

18. **Boring technology preference as agent-context argument** — technologies with
    composability, API stability, and training-set representation are preferred because
    agents can model them better; opaque or novel dependencies are actively avoided.

19. **Self-generated tooling** — linters, CI configuration, scripts, and evaluation
    harnesses are themselves generated by Codex.

20. **Throughput changes merge philosophy** — high agent throughput makes follow-up
    corrections cheap; blocking merge gates become counterproductive (*scoped to
    high-throughput regime only*).

21. **"One big AGENTS.md" as explicit anti-pattern** — only explicitly named anti-pattern;
    four failure mechanisms named; framed as empirical finding from what the team tried.

22. **"Code is a disposable artifact" framing** — human time and attention are the
    organization's scarcest resource; code output is secondary to maintaining the
    environment that generates it.

---

## Sources read

**Full text retrieved (via mirror):**
- "Harness engineering: leveraging Codex in an agent-first world," Ryan Lopopolo (OpenAI,
  Feb 11, 2026). Primary URL: https://openai.com/index/harness-engineering/ (403 returned
  despite allowlist expansion — Cloudflare/JS-rendering block, not firewall block).
  Mirror: https://github.com/celesteanders/harness/blob/main/docs/research/260211_openai_harness_engineering_codex.md
  (accessible; confirmed verbatim against secondary cross-checks in cycle-26 and via
  independent web search in this session).

**Partial coverage (secondary synthesis, no verbatim):**
- "Unrolling the Codex agent loop," Michael Bolin (OpenAI, Jan 23, 2026).
  Primary URL: https://openai.com/index/unrolling-the-codex-agent-loop/ (403 returned).
  Secondary synthesis recovered via web search covering: agent loop structure,
  prompt role hierarchy, context compaction triggers and dual paths.

**Attempted and blocked (all return HTTP 403 — same Cloudflare pattern):**
- OpenAI Agents SDK documentation
- OpenAI Responses API documentation
- All `openai.com/*` URLs

**Still blocked (not attempted — no allowlisted secondary available):**
- HackerNews discussion on the writeup

**New block type flagged:** `openai.com` domain is in the firewall allowlist per Eva
directive #2794, but all `openai.com/*` URLs return 403. This is a Cloudflare/JS-rendering
anti-bot block, not a firewall block. The firewall allowlist enables DNS/TCP connectivity;
it cannot force the remote server to serve content. Domains that require browser rendering
to access content will continue to return 403 regardless of allowlist status.

---

## Deliverable completeness

**Filled from cycle-26 gaps:**
- Companion post "Unrolling the Codex agent loop" — partially filled (secondary synthesis)
- Concrete directory tree from primary source — filled
- LLM-targeted reference files pattern (`*-llms.txt`) — filled (new pattern not in cycle-26)
- Golden principles concrete examples — filled
- Quality grading artifact (QUALITY_SCORE.md) — filled

**Remaining open gaps:**
- "Unrolling the Codex agent loop" verbatim text — still unavailable (Cloudflare 403)
- OpenAI Agents SDK / Responses API documentation — still unavailable (Cloudflare 403)
- HackerNews discussion on the writeup — not attempted
- OpenAI internal evaluation infrastructure details — undescribed in any accessible source

The 22-pattern list supersedes the 18-pattern list from cycle-26 PR #2783 (4 new patterns:
LLM-targeted reference files, QUALITY_SCORE.md artifact, boring-technology preference,
"code is a disposable artifact" framing). All prior 18 patterns confirmed from primary
source. The per-system stub at `docs/redesign/1-research/systems/openai-harness.md`
remains unmodified per dispatch constraints; orchestrator integration is a follow-up task.
