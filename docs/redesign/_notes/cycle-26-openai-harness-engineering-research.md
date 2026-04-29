# Cycle 26: OpenAI harness-engineering research

## Source status

**Primary source**: "Harness engineering: leveraging Codex in an agent-first world" by Ryan
Lopopolo, Member of Technical Staff at OpenAI. Published Feb 11, 2026.
URL: https://openai.com/index/harness-engineering/

The primary URL is **firewall-blocked** in this sandbox. The full text was retrieved via a public
GitHub mirror at
https://github.com/celesteanders/harness/blob/main/docs/research/260211_openai_harness_engineering_codex.md
and confirmed as a verbatim transcript of the OpenAI post by cross-checking multiple independent
summaries (InfoQ, engineering.fyi, ailearnedtoday.com) that cite identical passages verbatim.

**Related sources attempted and blocked**:
- https://openai.com/index/unrolling-the-codex-agent-loop/ — firewall-blocked; partial description
  recovered via web-search summaries only
- OpenAI developer documentation on Agents SDK, Responses API, Assistants API — firewall-blocked
- OpenAI Codex documentation (github.com/openai/codex) — partially accessible via web search

**Content gap from firewall blocks**: The "Unrolling the Codex agent loop" companion post, which
is referenced alongside the main writeup in multiple summaries, is unavailable in full. The Agents
SDK documentation is unavailable. These gaps affect Lens 3 (context/state API layer) and Lens 4
(tool definition schema). Both gaps are flagged where relevant.

This is a blog-post-shape read. The writeup is a single post; it does not have companion source
code or reference implementation. Claims marked *documented-claim* are from the writeup without
independently verifiable implementation backing.

---

## 1. Overall philosophy and load-bearing thesis

The writeup is a field report, not a design manifesto. Its empirical anchor: OpenAI's team built
and shipped an internal software product with zero manually-written lines of code, approximately
one million lines over five months, with three to seven engineers.

The stated thesis — set in bold in the original — is: **"Humans steer. Agents execute."**

This is a role-allocation frame, not a technical claim. The harness is what makes the division
tractable: it constructs the environment in which agents can execute reliably without human
intervention at the implementation level.

The load-bearing causal claim is:

> "Early progress was slower than we expected, not because Codex was incapable, but because the
> environment was underspecified. The agent lacked the tools, abstractions, and internal structure
> required to make progress toward high-level goals."

From this premise the entire argument follows: if agent failures are primarily environment failures
rather than model failures, then engineering effort should concentrate on the environment (the
harness), not on prompt engineering or model capability. The writeup treats "try harder" as
explicitly wrong:

> "When something failed, the fix was almost never 'try harder.' ... human engineers always stepped
> into the task and asked: 'what capability is missing, and how do we make it both legible and
> enforceable for the agent?'"

A third load-bearing sentence closes the writeup:

> "Building software still demands discipline, but the discipline shows up more in the scaffolding
> rather than the code."

**What the writeup does NOT name**: No explicit concept of "elicitation." No formal concept of
"scaffolding" beyond its informal use. No named "agent loop discipline." The writeup treats the
agent loop as an implementation fact (the agent does turns, uses tools) without theorizing it.
The closest named concept is "agent legibility" — the idea that the repository must be structured
so the agent can reason about it.

**Marketing-flavor assessment**: The writeup is primarily evidence-based (specific numbers: 1M
lines, 1500 PRs, 3.5 PRs/engineer/day, throughput increasing as team grew to seven engineers).
It is a first-party field report, so selection-bias risk is real — experiments that failed badly
before this one are not reported. The "Increasing levels of autonomy" section lists eleven steps
an agent can now perform end-to-end; the writeup immediately notes:

> "This behavior depends heavily on the specific structure and tooling of this repository and
> should not be assumed to generalize without similar investment."

That caveat, volunteered in the writeup itself, is the most useful data point for evaluating the
marketing-vs-evidence balance: the authors are willing to scope their claims.

---

## 2. Harness components and structure

The writeup does not present a formal taxonomy of harness components. Components are described
procedurally, as problems encountered and solutions built. Reconstructed from the text:

- **AGENTS.md** (~100 lines): table of contents injected into context; pointers to deeper sources
- **`docs/` structured knowledge base**: design docs, execution plans, product specs, generated
  references (e.g., `nixpacks-llms.txt`, `uv-llms.txt`), quality grades, reliability and security
  docs — treated as the system of record
- **Mechanical enforcement layer**: custom linters (Codex-generated), CI jobs validating
  architecture and knowledge-base freshness; error messages written to inject remediation
  instructions into agent context
- **Direct development tools**: `gh`, local scripts, repository-embedded skills (used directly by
  agents rather than mediated by the harness; *documented-claim* — invocation mechanism not
  specified)
- **Observability tools**: ephemeral per-worktree stack, LogQL / PromQL / TraceQL access
- **Chrome DevTools Protocol integration**: DOM snapshots, screenshots, navigation (for UI
  validation in the product under development)
- **Agent-to-agent review loop**: agents open PRs, request other agents' reviews, iterate until
  all agent reviewers are satisfied
- **Ephemeral worktrees**: one per change, isolated instance of the app plus its observability
  stack, torn down after task completion
- **"Golden principles" + doc-gardening agent**: background recurring sweeps for entropy, opening
  refactoring PRs for deviations

The writeup positions the harness as an emergent accumulation, not a monolithic designed structure.
The framing is depth-first: "breaking down larger goals into smaller building blocks ... prompting
the agent to construct those blocks, and using them to unlock more complex tasks." The harness
components were added over five months as capabilities were needed, not pre-specified.

The writeup does not draw a formal boundary between harness and model. The model produces code and
text; the harness is everything else. But this boundary is not theorized — there is no "the harness
ends at the tool-call interface" statement. The writeup says agents "use standard development tools
directly," which could mean the model emits function calls or it could mean the harness executes
commands and shows results. *Documented-claim: the invocation mechanism is not specified in the
writeup.*

The harness is neither one monolithic piece nor a formally layered architecture. It is a stack
of capabilities added depth-first. Composability is temporal (built over five months) rather than
architectural.

---

## 3. Context, state, and memory

This is the writeup's most explicit and developed section.

### Context window management

The central framing:

> "Context management is one of the biggest challenges in making agents effective at large and
> complex tasks. One of the earliest lessons we learned was simple: give Codex a map, not a
> 1,000-page instruction manual."

Four named failure modes of the "one big AGENTS.md" approach:

1. **Context crowding**: "A giant instruction file crowds out the task, the code, and the relevant
   docs—so the agent either misses key constraints or starts optimizing for the wrong ones."
2. **Salience collapse**: "Too much guidance becomes non-guidance. When everything is 'important,'
   nothing is. Agents end up pattern-matching locally instead of navigating intentionally."
3. **Rot**: "A monolithic manual turns into a graveyard of stale rules. Agents can't tell what's
   still true, humans stop maintaining it, and the file quietly becomes an attractive nuisance."
4. **Unverifiability**: "A single blob doesn't lend itself to mechanical checks (coverage,
   freshness, ownership, cross-links), so drift is inevitable."

### Progressive disclosure solution

AGENTS.md (~100 lines) injected into context functions as a map; the structured `docs/` directory
is the territory. The design:

> "agents start with a small, stable entry point and are taught where to look next, rather than
> being overwhelmed up front."

This is operationalized rather than just described. The writeup says: "We enforce this
mechanically. Dedicated linters and CI jobs validate that the knowledge base is up to date,
cross-linked, and structured correctly. A recurring 'doc-gardening' agent scans for stale or
obsolete documentation ... and opens fix-up pull requests."

### State as repository artifacts

The writeup treats state as entirely repository-local, versioned content:

> "From the agent's point of view, anything it can't access in-context while running effectively
> doesn't exist. Knowledge that lives in Google Docs, chat threads, or people's heads are not
> accessible to the system."

Plans are "first-class artifacts" — lightweight ephemeral plans for small changes; execution plans
with progress and decision logs for complex work, "checked into the repository." Between agent
sessions, the agent reconstructs context from the repository. There is no session-spanning
in-memory state; the repository is the persistent state.

### Long-term memory and re-grounding

The writeup has no explicit discussion of embedding-based retrieval, vector stores, or semantic
memory. The `docs/` approach is flat-retrieval (agents load what they're pointed to) rather than
semantic retrieval. Long-term "memory" is the repository's versioned docs, with freshness enforced
mechanically.

Re-grounding across turns is not explicitly discussed. The implicit model is: each agent run loads
context from the repository, so re-grounding is achieved by keeping the repository current. The
writeup notes "single Codex runs work on a single task for upwards of six hours" — suggesting
single long sessions rather than multi-session continuations.

### Relationship to OpenAI product line

The writeup mentions Codex CLI and GPT-5. It does not name the Responses API, Assistants API, or
Agents SDK. The context management strategy described is an application-level architecture built on
top of whatever API the team uses. *Content gap: the relationship between the writeup's context
patterns and the formal SDK's context management APIs is unavailable due to firewall blocks on
developer documentation.*

---

## 4. Tool definition, invocation, and trust boundary

The writeup is thin on tool definition theory. Tools are described in concrete terms rather than
as a general design concern.

**Named tools/capabilities in the writeup**:
- `gh` (GitHub CLI)
- Local scripts
- Repository-embedded skills
- Chrome DevTools Protocol (DOM snapshots, screenshots, navigation)
- LogQL / PromQL / TraceQL APIs

The team "wired the Chrome DevTools Protocol into the agent runtime and created skills for working
with DOM snapshots, screenshots, and navigation." This suggests tool-adding is done by building
harness integrations, not by declaring tools in a schema registry. *Documented-claim: no tool
schema design, naming convention, or count guidance is given.*

**Tool count and clarity**: Not theorized. The writeup implies more tools unlock more capability
without warning against proliferation. No explicit guidance on naming discipline or schema design.

**Trust boundary**: Wide. Agents "pull review feedback, respond inline, push updates, and often
squash and merge their own pull requests." Agents execute arbitrary shell commands, can write to
the repository, can drive CI. The writeup discusses worktree isolation as a practical boundary
(each change runs in its own isolated environment) but not as a formal trust model.

**Side-effecting vs read-only tools**: Not theorized. Examples include both categories (LogQL
queries = read; pushing code = write), with no design pattern for distinguishing them.

**MCP (Model Context Protocol)**: The word "MCP" does not appear in the retrieved full text of
the writeup. Some secondary summaries interpolate it; this appears to be secondary-source
addition, not an OpenAI claim in this document.

---

## 5. Failure handling and reliability

The writeup's failure-handling philosophy is set in the "Redefining the role of the engineer"
section:

> "When something failed, the fix was almost never 'try harder.' Because the only way to make
> progress was to get Codex to do the work, human engineers always stepped into the task and
> asked: 'what capability is missing, and how do we make it both legible and enforceable for
> the agent?'"

Failure is an **environment signal**, not a model signal. Correct response is environment
improvement — add a tool, add guardrails, add documentation, add structure.

**Explicitly named failure modes and their remediations**:
- Context crowding → progressive disclosure
- Architectural drift (bad patterns replicated) → golden principles + doc-gardening agent
- "AI slop" (accumulated poor patterns) → recurring cleanup process with background agents
- Build failures → listed as an agent-detectable and agent-remediatable condition
  (*documented-claim: mechanism not described*)

**Failure modes the writeup does NOT address**:
- Hallucinated tool calls
- Infinite agent loops
- Malformed output
- Prompt injection via untrusted inputs
- The writeup has no section on what the harness should do when the model produces obviously
  wrong output

**Throughput as a reliability strategy**: The "Throughput changes the merge philosophy" section
makes a pragmatic tradeoff argument:

> "Test flakes are often addressed with follow-up runs rather than blocking progress indefinitely.
> In a system where agent throughput far exceeds human attention, corrections are cheap, and
> waiting is expensive. This would be irresponsible in a low-throughput environment. Here, it's
> often the right tradeoff."

This is not a reliability argument; it is a throughput argument that accepts higher immediate
defect rates in exchange for velocity. The conditional ("irresponsible in a low-throughput
environment") is worth noting: the writeup explicitly scopes this argument to the high-throughput
regime.

**Testing and evaluation**: The writeup mentions CI jobs, structural tests (for architecture
enforcement), and "evaluation harnesses" as things agents generate. But it does not describe how
to test or evaluate the harness itself. Evaluation infrastructure for measuring task-success rates
across agent runs is not discussed. *Content gap: the "Unrolling the Codex agent loop" post is
referenced as adjacent material and may cover evaluation; unavailable due to firewall block.*

**Escalation**: Step 10 of 11 in the autonomy list is "Escalate to a human only when judgment is
required." The writeup does not specify what triggers escalation or how the agent detects that
judgment is required. This is the thinnest part of the failure-handling description.

---

## 6. Anti-patterns and explicit non-goals

The "one big AGENTS.md" is the only **explicitly named anti-pattern**, presented as empirical
finding:

> "We tried the 'one big AGENTS.md' approach. It failed in predictable ways..."

Four failure modes are named (see Lens 3). The writeup does not frame this as theory; it frames
it as what they tried and observed.

**Other anti-patterns implied but not stated**:
- **"Try harder" as failure response**: explicitly named as wrong ("almost never")
- **Manual cleanup cycles**: "Our team used to spend every Friday (20% of the week) cleaning up
  'AI slop.' Unsurprisingly, that didn't scale." The manual approach is replaced with automated
  agents; the manual approach is implicitly deprecated
- **Off-repo knowledge**: "Knowledge that lives in Google Docs, chat threads, or people's heads
  are not accessible to the system" — implied anti-pattern is storing knowledge the agent cannot
  reach
- **Opaque upstream dependencies**: "In some cases, it was cheaper to have the agent reimplement
  subsets of functionality than to work around opaque upstream behavior from public libraries."
  Technologies the agent cannot reason about are implicitly discouraged
- **Waiting for merge gates**: "corrections are cheap, and waiting is expensive" — blocking on
  perfection is implicitly warned against (scoped to high-throughput regimes)

**What the writeup explicitly does NOT address**:
- Agent hierarchy patterns (manager-of-managers, nested planners)
- Multi-model or multi-provider strategies
- Over-tooling warnings
- Security posture for adversarial input
- When agent-generated code is inappropriate

**Non-goals (inferred)**: The writeup de-prioritizes human code aesthetics explicitly: "The
resulting code does not always match human stylistic preferences, and that's okay. As long as the
output is correct, maintainable, and legible to future agent runs, it meets the bar." This is not
a non-goal listed as such; it is an implicit priority ordering.

---

## 7. Anchoring caveats

These argue *non-transfer*. For each difference: what specific patterns it discounts, and what
transfers despite the difference.

**Difference 1: OpenAI's harnesses target their own models (Codex/GPT-5); the redesign uses
Claude.**

*Discounts*: The writeup makes no claims about which instruction phrasing works, what
system-prompt structures are effective, or what context-injection methods are reliable. These
are model-specific concerns. Context window sizes, attention patterns, and tool-call reliability
differ between GPT-5 and Claude.

*Transfers*: The environment-as-primary-variable argument is explicitly model-agnostic — the
writeup says "Swap out the model—results shift a bit. Change the harness—everything breaks or
scales." (*Documented-claim: not verified independently.*) Progressive disclosure, repository-as-
state, and mechanical enforcement are all model-agnostic patterns.

**Difference 2: OpenAI is a model provider with first-party tooling; the redesign builds on an
external provider's API.**

*Discounts*: The team has first-party access to Codex CLI, GPT-5 internals, and the ability to
wire arbitrary integrations into the agent loop with full visibility into how it operates. The
redesign uses Claude through an API; the agent loop internals are opaque. Some of the harness
capabilities described (Chrome DevTools Protocol integration, per-worktree observability) require
significant infrastructure access beyond the API layer.

*Transfers*: Repository structure patterns (AGENTS.md as table of contents, docs/ as knowledge
base, plans as first-class artifacts) are harness-layer patterns requiring no first-party model
access. Mechanical enforcement patterns (CI checks on knowledge base, custom linters with
remediation messages) are equally accessible. These are the most directly replicable patterns
from the writeup.

**Difference 3: OpenAI's harness writeup serves a developer audience and is partially marketing.**

*Discounts*: The field-report format suppresses the failures that preceded the successful period
described. The "1/10th the time" comparison is against an unverifiable counterfactual. Throughput
numbers (3.5 PRs/engineer/day, throughput increasing as team grew) are single-team, single-
project data points with no control. The selection of what to discuss (and what to omit) is
shaped by what positions OpenAI's products favorably.

*Transfers*: The qualitative failure-mode descriptions for monolithic instruction files are
specific enough to evaluate independently of the overall narrative. The explicit caveat ("This
behavior depends heavily on the specific structure and tooling of this repository and should not
be assumed to generalize") weakens the marketing frame from within, making it a more reliable
signal that the authors are being careful about their claims.

**Difference 4: OpenAI's harness runs interactively or on user-issued requests; the redesign runs
autonomously on a cron.**

*Discounts*: The writeup describes human-triggered agent runs ("an engineer describes a task, runs
the agent"). The human can interrupt, redirect, and course-correct per task. A cron-triggered
autonomous system has no such interrupt. The escalation path ("escalate to a human only when
judgment is required") assumes a human is available and responsive; a cron system needs fully
automated escalation logic that the writeup does not provide.

*Transfers*: Progressive disclosure, repository-as-state, doc-gardening, and golden principles
all apply regardless of trigger mechanism. The golden-principles pattern — encode rules
mechanically so they apply everywhere without runtime judgment — is arguably *better suited* to
a cron system than to an interactive one, because the cron has no human available to exercise
judgment inline.

**Difference 5: OpenAI's harness targets a general-purpose product; the redesign targets one
narrow use case.**

*Discounts*: The observability stack (LogQL, PromQL, TraceQL), Chrome DevTools integration,
end-to-end UI validation, and multi-agent product reviews are all general-product capabilities
irrelevant to the redesign's scope. The redesign does not have a running application to validate.
These patterns cannot transfer.

*Transfers*: Core knowledge-management patterns (AGENTS.md as entry point, structured docs/,
plans as artifacts, mechanical enforcement) apply at any scope. Narrow scope arguably makes them
easier to implement: fewer domains to document, fewer linting rules to write.

**Difference 6: OpenAI's writeup draws on internal evaluation infrastructure not available to the
redesign.**

*Discounts*: "Evaluation harnesses" are mentioned but not described. CI coverage is referenced but
not specified. Quality grading per domain is referenced but methodology is not described. The
reliability claims rest on infrastructure the redesign cannot examine or replicate.

*Transfers*: The principle of "encode evaluation criteria into the repository so agents can run
them" is independent of the specific infrastructure. The doc-gardening pattern (recurring
freshness checks) is accessible. Both can be implemented without access to OpenAI's internal
evaluation tooling.

---

## Patterns observed in OpenAI's published material

Listed without v2-relevance framing. Sourced from the writeup unless noted.

1. **Environment underspecification as primary failure mode** — agent failures attributed to the
   environment (tools, docs, structure) being underspecified, not to model capability limits.

2. **"Humans steer. Agents execute." as role-allocation thesis** — the harness enforces this role
   boundary.

3. **AGENTS.md as table of contents, not encyclopedia** — short (~100 lines), injected into
   context, pointers only; the knowledge territory is a separate structured `docs/` directory.

4. **Four named failure modes of monolithic instruction files** — context crowding, salience
   collapse, rot, unverifiability; each named and mechanistically described.

5. **Progressive disclosure pattern** — agents start with a minimal stable entry point and are
   directed to deeper context only as needed.

6. **Repository as single source of record** — off-repo knowledge (Google Docs, Slack, people's
   heads) is illegible to the agent; it effectively does not exist for agent purposes.

7. **Plans as first-class versioned artifacts** — active, completed, and technical-debt plans
   checked in to the repo, not external.

8. **Mechanical enforcement over documented rules** — architectural constraints encoded as linters
   and CI checks rather than as text the agent is expected to follow.

9. **Custom linters with agent-readable error messages** — error messages from custom lints are
   written to inject remediation instructions directly into agent context.

10. **Depth-first environment building** — capabilities added iteratively as failures surfaced;
    harness was not designed upfront.

11. **Entropy as a first-class engineering concern** — agent systems replicate patterns including
    bad ones; garbage collection via recurring cleanup agents treated as infrastructure.

12. **"Golden principles" pattern** — canonical DO/DON'T rules versioned in-repo and mechanically
    enforced on a recurring cadence; encoded once, applied everywhere.

13. **Agent-to-agent review loop** — agents review their own and each other's changes; humans
    supervise at a higher level of abstraction.

14. **Ephemeral per-change isolation** — each change runs in an isolated worktree with its own app
    instance, observability stack, and logs.

15. **"Agent legibility" as the optimization target** — the codebase is structured for agent
    comprehension first; human aesthetic preferences are secondary.

16. **Throughput changes merge philosophy** — high agent throughput makes follow-up corrections
    cheap; blocking merge gates are counterproductive at scale (*scoped to high-throughput regime*).

17. **Self-generated tooling** — linters, CI configuration, scripts, and evaluation harnesses are
    themselves generated by Codex, closing the loop.

18. **"One big AGENTS.md" as explicit anti-pattern** — four failure mechanisms named; empirically
    grounded from what the team tried and abandoned.

---

## Sources read

**Full text retrieved:**
- "Harness engineering: leveraging Codex in an agent-first world," Ryan Lopopolo (OpenAI,
  Feb 11, 2026). https://openai.com/index/harness-engineering/ (firewall-blocked; full text
  retrieved via mirror at
  https://github.com/celesteanders/harness/blob/main/docs/research/260211_openai_harness_engineering_codex.md)

**Attempted and blocked (firewall):**
- https://openai.com/index/unrolling-the-codex-agent-loop/ — partial description recovered via
  web-search summaries; full text unavailable
- OpenAI Agents SDK developer documentation
- OpenAI Responses API documentation
- OpenAI Assistants API documentation

**Web search summaries used to verify mirror completeness:**
- InfoQ coverage (infoq.com/news/2026/02/openai-harness-engineering-codex/) — blocked but summary
  retrieved; passages confirmed consistent with mirror
- engineering.fyi and ailearnedtoday.com summaries — confirmed consistent quoting

The HackerNews discussion thread and the "Unrolling the Codex agent loop" post are the primary
content gaps from this read.
