# Cycle 27: Per-finding evaluation of cycle-26 three-dispatch deliverables + cross-system elevation analysis

Cycle 26 (commit `fe7f29b1`) dispatched three Phase 1 reads: Cognition Devin
(PR [#2780](https://github.com/EvaLok/schema-org-json-ld/pull/2780), 816 lines),
OpenAI harness-engineering (PR [#2783](https://github.com/EvaLok/schema-org-json-ld/pull/2783),
492 lines), oh-my-codex (PR [#2784](https://github.com/EvaLok/schema-org-json-ld/pull/2784),
299 lines). All three landed before cycle 27 fired (~hours after dispatch). Cycle 27 took the
"focal = evaluate landed PR(s)" branch of the cycle-26 plan and ran a per-finding evaluation
across all three deliverables, cross-validating each against the existing seven cross-system
patterns in `1-research.md` (5 at 3+ systems, 7 at 2 systems including the cycle-25
elevations). Cycle-28+ pre-commits at the bottom.

## Quality assessment per deliverable (gate before merge)

Each deliverable is assessed on (a) source quality / verifiability, (b) anti-smuggling
discipline (no v2-relevance framings in the patterns section), (c) caveats symmetry
(discount + transfer per cycle-18 discipline), (d) lens coverage.

### PR #2780 — Cognition Devin

- **Source quality:** LOWEST of the three. All `cognition.ai` URLs were firewall-blocked;
  all primary-source claims are sourced from web search results and secondary commentary
  that quotes Cognition posts. Every Cognition-attributed quote is flagged as
  **documented-claim** throughout. SWE-bench results are independently verifiable
  (`github.com/CognitionAI/devin-swebench-results`); architecture and failure-handling
  claims are not. The deliverable is honest about this — flags it in the source-access
  note at the top, marks each documented-claim individually, and notes that verification
  status is "substantially lower than prior source-read cycles (AutoGen, Voyager,
  LangGraph) where primary code and documentation were directly inspected."
- **Anti-smuggling discipline:** APPLIED. Patterns observed section (18 patterns) lists
  observations without v2-relevance framing. No "as a guardrail mechanism" / "as a
  redesign-relevant shape" language.
- **Caveats symmetry:** PASS. Seven anchoring caveats, each with discount + transfer
  argument per cycle-18 discipline.
- **Lens coverage:** Two primary depth lenses (context/state/memory; failure handling)
  are the longest sections, per dispatch instructions. Failure-handling section is
  honest about the documentation gap (Cognition publishes operational acknowledgment
  of "crashing, stuck, hanging" but no recovery architecture).
- **Verdict:** ACCEPT WITH SOURCE-CREDIBILITY HEDGE. The deliverable's value is high
  for the philosophical / architectural-position claims (context engineering thesis,
  anti-multi-agent stance) which are widely-cited and externally reinforced.
  Documented-claim status applies to all Cognition-internal architecture claims;
  these should be cited with the documented-claim caveat preserved when integrated.

### PR #2783 — OpenAI harness-engineering

- **Source quality:** MIXED. Primary URL (`openai.com/index/harness-engineering/`)
  was firewall-blocked. Full text retrieved via a public GitHub mirror and verified
  complete against multiple independent summaries that cite identical verbatim
  passages. The mirror-verification approach is reasonable; the source itself is a
  field-report from the author (Ryan Lopopolo, OpenAI MTS) so first-party claims
  about the team's experience are direct (no further sourcing layer). Companion
  posts (Unrolling the Codex agent loop, Agents SDK docs) blocked; gaps explicitly
  flagged.
- **Anti-smuggling discipline:** APPLIED. 18 patterns listed without v2-relevance
  framing.
- **Caveats symmetry:** PASS. Six anchoring caveats with discount + transfer per
  cycle-18 discipline.
- **Lens coverage:** Lens 3 (context/state/memory) is the longest, per the
  writeup's actual depth; Lens 4 (tool integration) is honestly described as
  thin in the writeup itself ("Tools are described in concrete terms rather than
  as a general design concern"). Doesn't pad where the source is thin.
- **Verdict:** ACCEPT. Mirror-verification is sound; documented-claim flagging
  is consistent. The "1M lines, 1500 PRs, 3.5 PRs/engineer/day" empirical anchors
  are first-party but unverifiable externally — flagged appropriately.

### PR #2784 — oh-my-codex

- **Source quality:** HIGHEST of the three. Code+docs read of GitHub repository at
  specific commit `f0d9b3d`. No firewall issues. Inference-from-source vs
  documentation-backed distinction maintained throughout. File sizes named
  (e.g., "src/hooks/keyword-detector.ts, 44KB" and "src/autoresearch/runtime.ts,
  45KB") with explicit acknowledgment that the largest files were not read in
  full and are flagged as "areas requiring deeper reading if specific patterns
  from this report are being evaluated for transfer."
- **Anti-smuggling discipline:** APPLIED. 22 patterns listed without v2-relevance
  framing.
- **Caveats symmetry:** PASS. Six anchoring caveats with discount + transfer per
  cycle-18 discipline.
- **Lens coverage:** Comprehensive across all 7 lenses. Strongest on configuration
  model, prompt structure, and failure handling (where the actual code is most
  readable).
- **Verdict:** ACCEPT. Highest-quality deliverable of the three; serves as the
  baseline-reliability anchor against which the other two should be weighed when
  cross-validation produces partial-support patterns.

### Combined verdict on all three: ACCEPT and merge

All three pass quality gating. No deliverable should be rejected; all three should
be merged before Tier-1 integration begins.

## Cross-validation matrix: do the new systems support, contradict, or remain neutral on the existing 7+5 patterns?

For each existing pattern in `1-research.md`'s "Cross-system observations" section,
each new system is scored as STRONG SUPPORT (S+) / WEAK SUPPORT (W+) / NEUTRAL (N) /
WEAK CONTRADICTION (W-) / STRONG CONTRADICTION (S-). The mechanical-elevation rule
from the section opening: "if a 6th system supports a 2-system pattern, this rule
elevates it to 3+ mechanically." Eight systems are now read at depth (openclaw, PAI,
AutoGen, Voyager, LangGraph, Cognition Devin, OpenAI harness, oh-my-codex), so
2-system patterns can be elevated to 3+ if any one of the three new systems
supports.

### Existing 3+ system patterns

| Pattern | Cognition Devin | OpenAI Harness | oh-my-codex | New system count | Update |
|---|---|---|---|---|---|
| Multi-agent decomposition is not a default | **S+** (strongest argument yet — explicit named rejection of OpenAI Swarm and AutoGen as "the wrong way of building agents"; Flappy Bird failure example; named anti-pattern) | N (agent-to-agent review loop is multi-agent for review, but writeup describes single agent per task — implicit support) | W+ (default workflow is single-thread; `$team` explicitly not the default onboarding path; multi-agent runtime exists but as opt-in) | 6 systems (added Cognition Devin foregrounded; oh-my-codex weak; OpenAI harness implicit) | ELEVATE language: now strongest cross-system pattern (6 systems including Cognition Devin's foregrounded named rejection); remains 3+ tier but the strength changes |
| Deterministic code executes; LLM proposes (code-vs-prompts split) | N (Devin tool model is opaque) | **S+** (mechanical enforcement layer; custom linters; CI checks; AGENTS.md as table of contents) | **S+** (MCP servers are TypeScript; keyword detector is deterministic pattern matching; sparkshell Rust binary; explicit "deterministic keyword detection over heuristic prompt parsing") | 6 systems foregrounded (added 2 strong) | Tier-1 update count: 4-system → 6-system foregrounded |
| Small core, capability extends via plugins/skills/tools/layers | N | W+ (depth-first environment building; harness as accumulation; AGENTS.md as small entry, docs/ as extension) | **S+** (configuration layer + hook harness; explicitly "OMX does NOT replace Codex"; 39 skills, 30 prompts, MCP servers, hooks as extension surface) | 7 systems (added 2; harness shape match; oh-my-codex strong) | Tier-1 update count: 5-system → 7-system |
| Strong-defaults security with operator-controlled knobs | N | W- (trust boundary is wide — agents merge their own PRs; not a security primitive) | W+ (Lore-format commit signing; explicit escape hatches; doctor checks separated from auth checks) | Mixed: oh-my-codex weak support; OpenAI weak contradiction; Cognition neutral | NO ELEVATION; OpenAI's wide trust boundary is itself worth noting (counter-example: at high agent throughput, security-tight defaults may not survive — flag for cycle-28 single-system observation list) |
| Anti-patterns explicit as deliverable artifact | **S+** ("Don't Build Multi-Agents" is the canonical example; multiple posts are anti-pattern arguments) | **S+** ("One big AGENTS.md" named with four failure mechanisms) | **S+** (CONTRIBUTING.md `<Bad>` examples; "Don't claim completion without verification"; multiple deprecated patterns explicitly marked) | 6 systems (added 3 all strong) | Tier-1 update count: 3-system → 6-system |

### Existing 2-system patterns (cycle-25 elevation tier)

| Pattern | Cognition Devin | OpenAI Harness | oh-my-codex | New system count | Elevation? |
|---|---|---|---|---|---|
| Multiple orchestration patterns coexist as first-class | W- (rejects multi-agent; doesn't propose multiple patterns) | N | **S+** (named modes deep-interview/ralplan/ralph/team/autopilot/ultrawork/ultraqa with allowlisted transition policy in `docs/STATE_MODEL.md`) | 3 systems (oh-my-codex adds 3rd) | **ELEVATE to 3+ tier** (mechanical rule fires; oh-my-codex's transition policy makes the pattern stronger than just "multiple orchestration patterns exist" — it adds explicit transition allowlist as the discipline) |
| Component-local state persistence (no central state file) | W+ (filesystem as state, but more "monolithic environment" than component-local) | **S+** (plans as first-class versioned artifacts; multiple plan types; no single state.json) | **S+** (per-mode state files in `.omx/state/<mode>-state.json`; session vs root scope; reconciliation prevents resurrection) | 5 systems (added 3 if Cognition counts as weak support) | **ELEVATE to 3+ tier** with strongest support yet — multiple new systems independently arrive at "no central state file, multiple per-component artifacts." OpenAI Harness's explicit "off-repo knowledge doesn't exist" framing strengthens the underlying motivation |
| Append-only history; no destructive rollback | N | W+ (repository as state is git-based; commits append; worktrees torn down but history remains) | W+ (file-backed migration with one-way compatibility windows; legacy preserved as read-only; schema migration is one-way, not destructive) | 4 systems (added 2 weak) | **ELEVATE to 3+ tier** with diversity hedge — new system support is via filesystem/git rather than via in-process versioning (LangGraph's update_state branching, Voyager's V2/V3 versioning); convergence on principle, divergence on substrate (in-process vs filesystem) |
| Failed work as recorded artifact, not silent discard | W- ("Crashing, stuck, hanging" acknowledged but no recording mechanism documented) | N | **S+** (Ralph progress ledger records failures; autoresearch keep/discard/stop decisions logged in iteration-ledger.json with reasons) | 3 systems (oh-my-codex adds 3rd) | **ELEVATE to 3+ tier** (mechanical rule fires; oh-my-codex's iteration-ledger pattern is structurally similar to Voyager's failed_tasks.json) |
| Memory as first-class architectural concept | W+ (agent trace as the unit of context; Devin Wiki as workspace knowledge — but framing is "context window IS memory" + filesystem) | W+ (repository as single source of record; "off-repo knowledge doesn't exist for the agent") | **S+** (`.omx/wiki/` markdown wiki; MCP wiki server; SessionStart can inject bounded wiki context; markdown-first, search-first) | 5 systems (added 3 with mixed strength) | **ELEVATE to 3+ tier** with diversity hedge — five distinct framings: PAI's principle-shape, LangGraph's mechanism-shape (Store), Cognition's context-trace-shape, OpenAI Harness's repository-shape, oh-my-codex's wiki-shape. The convergence is on memory-as-architectural-concern; divergence is on the specific primitive (vector store / typed channel / context trace / repository / wiki) |
| Small fixed team with explicit role-separation | **S-** (single-threaded linear agent EXPLICITLY rejects role decomposition; the strongest contradiction in the matrix) | W+ (agent-to-agent review loop; doc-gardening agent — partial role separation) | **S+** (30 named role prompts: Metis/Ralph/planner/architect/critic/analyst/verifier/researcher/etc.; workflow stages have role-named agents) | 4 systems (added 2 supporting + 1 contradicting) | **CONDITIONAL ELEVATE to 3+ tier WITH CONTRARY-STANCE NOTE.** Cognition Devin contradicts the pattern explicitly; this is a substantive divergence across systems, not just an absent-vs-present asymmetry. The pattern still elevates by mechanical count (3 supporting), but the elevation should be paired with a "Cognition Devin's published anti-stance" note |
| Per-agent model selection as architectural primitive | W- (Devin uses Sonnet 4.5 with model-specific tuning, but it's a single agent; not per-agent multi-model) | N (uses Codex/GPT-5; no per-agent model theory) | **S+** (model support matrix supports GPT-5.4/5.4-mini/5.5/5.3-codex; mini composition seam for exact-model gating; `$ask-claude` and `$ask-gemini` skills for cross-provider invocation) | 3 systems (oh-my-codex adds 3rd) | **ELEVATE to 3+ tier** (mechanical rule fires; oh-my-codex extends the cost-tiering rationale Voyager named into a multi-provider concern) |

### New patterns from cycle-26 deliverables that don't fit existing 7

These are 1-2 system patterns NOT covered by the existing seven; they go to the
single-system observations file (or, if 2-system, to the 2-system tier in
`1-research.md` after cycle-28 evaluation when more cross-validation is possible).

| Pattern | Strongest source | Other systems | System count | Tier candidate |
|---|---|---|---|---|
| Mechanical enforcement of constraints (linters, CI checks, regression tests on agent-affecting prose) | OpenAI Harness pattern 8/9/12 (mechanical enforcement; custom linters with agent-readable error messages; golden principles) | oh-my-codex pattern 7 (behavioral prompt contract with regression tests); Voyager (CriticAgent runs assertion checks); LangGraph (typed schema validation) | 4 systems with cross-substrate convergence | **2-system tier candidate for cycle 28** — cross-validate the strict framing (regression-tested constraint enforcement) before elevating |
| Plans/specs as first-class versioned artifacts | OpenAI Harness pattern 7 (plans as first-class versioned artifacts) | oh-my-codex pattern 2 (context snapshot grounding before execution; written to .omx/context/...); Voyager curriculum log | 3 systems with structural variation | **2-system tier candidate** — distinct from "Component-local state persistence" because the framing is planning-artifact-shape (forward-looking) vs state-artifact-shape (backward-looking) |
| Entropy / AI slop as first-class engineering concern | OpenAI Harness pattern 11 (entropy as first-class engineering concern) + pattern 12 (golden principles + doc-gardening agent) | oh-my-codex pattern 13 (deslop pass as mandatory post-completion step) | 2 systems | **2-system tier candidate** — captures the recurring-cleanup-as-infrastructure pattern; relates to F12 (defense accretion catalog hint) inversely (these systems treat accretion as a bug to be cleaned, not a defensive feature to add) |
| Context anxiety / model self-model failures (model behavior shaped by its beliefs about its own resources, not just by content) | Cognition Devin pattern 4/5/6/15 (context anxiety; environmental deception; prompt placement matters; misestimation with precision) | None at depth | 1 system (Cognition only) | Single-system observations file. Notable as the most candid Cognition observation but not yet cross-validated |
| Pre-execution gating against underspecified requests | oh-my-codex pattern 5 (word-count + signal-detection gate; force: bypass) | Cognition Devin partial (context anxiety mitigation prevents premature task closure — adjacent but different) | 1-2 system | Single-system observations file; cycle 28+ may surface 2nd system when oh-my-claudecode or symphony land |
| Agent legibility / repo structured for agent comprehension first | OpenAI Harness pattern 15 (agent legibility as optimization target) | Cognition Devin implicit; oh-my-codex implicit (AGENTS.md template with autonomy directive) | 1-3 system depending on framing strictness | Single-system observations file with "implicit support from 2 others" note |
| Throughput-based merge philosophy (corrections cheap; waiting expensive) | OpenAI Harness pattern 16 (scoped to high-throughput regime — explicitly conditional) | None | 1 system (with conditional scope) | Single-system observations file with the conditionality preserved |
| Iteration limits with explicit ceiling / bounded autonomy loops | oh-my-codex pattern 6 (max_iterations=10; review loop max=5; autoresearch keep/discard/stop per iteration) | Cognition Devin partial (45-min session time limit, inferred); not a documented architectural limit | 1-2 system | Single-system observations file |
| Autonomy directive prominently stated (don't ask permission) | oh-my-codex pattern 17 (templates/AGENTS.md opens with "YOU ARE AN AUTONOMOUS CODING AGENT. EXECUTE TASKS TO COMPLETION WITHOUT ASKING FOR PERMISSION") | OpenAI Harness implicit ("Humans steer. Agents execute."); Cognition Devin implicit ("fully autonomous AI software engineer" framing — but contested by reality) | 1 system explicit + 2 implicit | Single-system observations file with "framing convergence at 3 systems if implicit support counts" hedge |

### Pattern that contradicts existing claim

| Existing claim | Contradicting source | Note |
|---|---|---|
| Strong-defaults security with operator-controlled knobs (3-system pattern) | OpenAI Harness wide trust boundary (agents merge their own PRs; per-worktree isolation is practical not security-primitive) | NOT a contradiction of the existing pattern's truth at the original 3 systems — but a counter-example showing that at very high agent throughput, the security-tight default may be inverted (trust agents broadly, isolate via ephemeral environments). Worth recording as single-system observation with "throughput regime as moderating variable" framing |

## Tier-1 (bounded mechanical) edits applied this cycle

The cross-validation produces 4-5 elevations and an updated section opening. Cycle 27
applies the bounded mechanical edits; load-bearing prose changes (e.g., reframing
the cross-system observations section as it grows from 7 to 12+ patterns) deferred
to cycle-28+ pre-commits.

### Edit 1: Section opening — update systems-read count from 5 to 8

`1-research.md` lines 817-833. The current text reads "Five systems read at depth:
openclaw, PAI (cycle 14); AutoGen (cycles 15-16, PR #2763); Voyager (cycle 17);
LangGraph (cycles 18-20, PR #2768)." Update to add Cognition Devin (cycle 26, PR
#2780); OpenAI harness-engineering (cycle 26, PR #2783); oh-my-codex (cycle 26,
PR #2784). Update the "60% bar at 5 systems" arithmetic observation to "60% bar
at 5 systems" → keep as-is (the original 5-system observation is still the
calibration anchor); add a new clause noting "with 8 systems now read at depth,
the 3+ threshold creates a 37.5% bar; the original calibration is preserved as
the threshold's design rationale."

### Edit 2: Update system counts on existing 3+ patterns where new systems add support

Three patterns get count updates (mechanical edits):
- "Multi-agent decomposition is not a default" — 3 → 4+ systems with foregrounded support; add Cognition Devin's named-rejection clause
- "Deterministic code executes; LLM proposes" — 4 → 6 systems foregrounded; add OpenAI Harness mechanical-enforcement clause; add oh-my-codex MCP-and-keyword-detector clause
- "Small core, capability extends" — 5 → 7 systems; add OpenAI Harness depth-first-accumulation clause; add oh-my-codex configuration-layer clause
- "Anti-patterns explicit as deliverable artifact" — 3 → 6 systems; add Cognition Devin's "Don't Build Multi-Agents" clause; add OpenAI Harness's "One big AGENTS.md" clause; add oh-my-codex's CONTRIBUTING.md `<Bad>` examples clause

### Edit 3: Elevate four 2-system patterns to 3+ tier (move from "2 systems" subsection to "3+ systems" subsection)

Mechanical promotion (move the bullet from one section to the other; add the
new-system support clause):
- "Multiple orchestration patterns coexist as first-class" → 3 systems (add oh-my-codex transition-policy clause)
- "Component-local state persistence" → 4-5 systems (add OpenAI Harness plans-as-artifacts and oh-my-codex per-mode-files clauses)
- "Failed work as recorded artifact" → 3 systems (add oh-my-codex iteration-ledger clause)
- "Per-agent model selection as architectural primitive" → 3 systems (add oh-my-codex multi-provider clause)

### Edit 4: Elevate two 2-system patterns with diversity hedge

These elevate by mechanical count but with a divergence-shape hedge:
- "Append-only history; no destructive rollback" → 4 systems (add OpenAI Harness git-history clause and oh-my-codex one-way-migration clause; ADD diversity hedge: "convergence on principle, divergence on substrate — in-process versioning vs filesystem/git history")
- "Memory as first-class architectural concept" → 5 systems (add Cognition Devin context-trace clause, OpenAI Harness repository-as-record clause, oh-my-codex wiki-server clause; ADD diversity hedge: "five distinct framings of the underlying primitive")

### Edit 5: Elevate one 2-system pattern with contrary-stance note

- "Small fixed team with explicit role-separation" → 4 systems with contrary-stance note: "Cognition Devin contradicts this pattern explicitly via its single-threaded linear agent stance ('Don't Build Multi-Agents'). The mechanical-count elevation holds, but the elevation pairs with a substantive divergence-of-architectural-position note rather than just an absence-of-evidence asymmetry."

### Edit 6: Defer Tier-1 edits with structural-load complexity to cycle 28

Edits NOT applied this cycle (Tier-2, deferred):
- Restructure the "Cross-system observations" section as it grows from 7 to 12-13 patterns
  (the section is currently 2-tier — 3+ vs 2-system; with 8 systems read, more patterns
  qualify for 3+; the 2-system tier may shrink or get reorganized by NEW pattern shape
  rather than mechanical count)
- Add the 5-7 NEW pattern candidates from the cross-validation matrix (mechanical
  enforcement, plans-as-artifacts, entropy/AI-slop, context anxiety, pre-execution
  gating, agent legibility, autonomy directive) to either single-system or 2-system
  tier; this requires careful decision per pattern about framing strictness
- Reconcile the Cognition Devin contradiction on small-fixed-team with the existing
  Persistent Divergences section (the divergence is now Cognition-vs-Voyager+AutoGen+
  oh-my-codex, which is a 1-vs-3 split worth a Persistent Divergences entry)
- Update the section-opening "60% bar at 5 systems" math (the calibration observation
  is now historical; the 8-system arithmetic is different); load-bearing rewrite
  rather than just append

## Same-cycle cold-reader on cycle-27 work

Per the cycle-25 codified discipline-lightening rule:
- Cross-validation matrix and per-finding evaluation: substantive prose; this is
  architecturally-load-bearing (the elevations affect every reader of the cross-
  system observations section). Full structured cold-reader pass warranted.
- Tier-1 mechanical edits (count updates, section-promotion bullet moves):
  bounded mechanical; 30-second self-check applies.

### Cold-reader on cross-validation matrix

**Question (a): Are the SUPPORT/CONTRADICT scores defensible against close reading
of each deliverable, or are they pattern-matched at the surface?**

Spot-check three high-stakes scores:

1. Cognition Devin S+ on "Multi-agent decomposition is not a default."
   - Verification: cognition-devin.md pattern 3 ("Multi-agent decomposition as a
     context fragmentation anti-pattern") and pattern 9 ("Framework-specific
     rejection. OpenAI Swarm and Microsoft AutoGen named as promoting 'the wrong
     way of building agents.'"). Plus the entire §3 of the deliverable, which is
     the most architecturally-load-bearing single section in the document. The
     "Don't Build Multi-Agents" post is the most-cited Cognition piece in
     external commentary. The S+ score is well-supported.
   - PASS.

2. oh-my-codex S+ on "Per-agent model selection as architectural primitive."
   - Verification: oh-my-codex.md §2 ("Model support matrix: `src/config/models.ts`
     defines the supported model list. OMX explicitly supports multiple models
     including GPT-5.4, GPT-5.4-mini, GPT-5.5, GPT-5.3-codex, and others") plus
     §3 ("`mini composition seam` for exact-model gating") plus the `$ask-claude`
     and `$ask-gemini` skills explicitly named in §4 ("`src/scripts/ask-claude.sh`,
     `src/scripts/ask-gemini.sh` — scripts to invoke other LLM providers from
     within a Codex session"). The cross-provider claim is grounded in shipped
     skills, not just configuration. S+ is supported.
   - PASS.

3. Cognition Devin S- on "Small fixed team with explicit role-separation."
   - Verification: cognition-devin.md §3 ("Cognition's published position is the
     most explicit anti-multi-agent stance in the surveyed systems") plus pattern
     2 ("Single-threaded linear agent as the architectural default") and pattern 9
     (named rejection of frameworks that prescribe role-decomposition). The
     contradiction is substantive: Cognition explicitly argues against the role-
     separation pattern, not just absent of it. S- is correct — it is a
     contradiction not a neutral observation.
   - PASS.

Three spot-checks pass. The matrix scores are defensible.

**Question (b): Are the elevations driven by genuine cross-system convergence or
by the mechanical-rule arithmetic alone?**

Two of the elevations need closer examination:

1. "Append-only history; no destructive rollback" → 4 systems with diversity hedge.
   Cognition Devin scored N; OpenAI Harness scored W+ (git as substrate); oh-my-codex
   scored W+ (one-way migration). The hedge ("convergence on principle, divergence
   on substrate") is genuine — git-as-substrate is a fundamentally different mechanism
   than LangGraph's update_state branching. The elevation is by mechanical rule but
   the hedge is real and necessary.
   - QUALIFIED PASS — elevation should foreground the hedge in the elevated bullet,
     not bury it in a parenthetical.

2. "Memory as first-class architectural concept" → 5 systems with five distinct
   framings. The hedge ("five distinct framings of the underlying primitive") is
   accurate — there is no single shared mechanism, just shared elevation of memory
   to architectural concern. The elevation is by mechanical rule but the underlying
   convergence is at the framing level, not the mechanism level.
   - QUALIFIED PASS — elevation language should be careful to say "convergence on
     architectural elevation; divergence on mechanism" (not "convergence on memory
     mechanism"); the existing 2-system bullet already has this language for
     PAI-vs-LangGraph; the elevated bullet should preserve and extend it.

**Question (c): Are NEW pattern candidates being added to the single-system or 2-
system tier defensibly, or are some of them speculative pattern-spotting?**

Spot-check the highest-stakes new candidate:

1. "Mechanical enforcement of constraints (linters, CI checks, regression tests on
   agent-affecting prose)" — labeled as 4-system convergence with cross-substrate.
   - OpenAI Harness pattern 8/9/12: mechanical enforcement, custom linters, golden
     principles. Verified.
   - oh-my-codex pattern 7: behavioral prompt contract with regression tests.
     Verified — `src/hooks/__tests__/prompt-guidance-*.test.ts`.
   - Voyager (claimed): CriticAgent runs assertion checks. Cross-check the existing
     1-research.md Voyager section: the CriticAgent is described as "verification
     loop" in the cycle-22 single-system observations — calling this "mechanical
     enforcement of constraints" is a stretch. The CriticAgent is a separate agent
     that critiques output; it's not a CI/lint mechanism. Calling this 4-system
     convergence overstates Voyager's match.
   - LangGraph (claimed): typed schema validation. The schema validation IS
     mechanical enforcement of structural constraints (channel types, reducer
     contracts) — this counts. But it's enforcing data shape, not behavioral
     constraints; the framing might need to be split into "mechanical schema
     enforcement" vs "mechanical behavioral enforcement."
   - REVISED: this is 2-3 system convergence (OpenAI Harness, oh-my-codex strong;
     LangGraph weak with framing question). NOT 4-system. Defer to cycle 28
     pre-commit for clarification.

The Voyager-as-mechanical-enforcement claim was loose pattern-matching. Other
new-candidate scores should be re-checked at cycle 28 with similar discipline.

**Verdict on cycle-27 cross-validation matrix:** The matrix is substantively sound;
two specific items need refinement (Append-only diversity hedge prominence; Memory
elevation framing language; the NEW-pattern Voyager-as-mechanical-enforcement
overcount). Tier-1 edits will apply the elevations as analyzed. Tier-2 cycle-28
work will tighten the diversity-hedge language and recheck the NEW-candidate
multi-system claims.

### 30-second self-checks on Tier-1 mechanical edits

- Section opening update (5 → 8 systems): factual count update. PASS.
- 4 patterns at 3+ tier with new-system clauses: each clause verifies against the
  source deliverable's specific section/pattern number; cite preserves
  documented-claim status for Cognition where applicable. PASS.
- 4 patterns elevated from 2 to 3+ tier: mechanical promotion of bullets between
  subsections; new-system support clauses added per the matrix. PASS.
- 2 patterns elevated with diversity hedge: hedge language carries forward from the
  cross-validation matrix into the bullet itself, not lost in a parenthetical.
  PASS.
- 1 pattern elevated with contrary-stance note: the Cognition contradiction is
  named in the elevated bullet, not deferred to Persistent Divergences. PASS.

PASS across all Tier-1 edits. Apply this cycle.

## Long-deferred items roll-call (carry-forward from cycle 26)

1. Journal-entry self-congratulation sweep (21 cycles deferred from cycle 7+)
2. F6/F8/F9 measurements (cycle 7+)
3. Refactor-for-length F-section sweep (cycle 8+; Tier-2 group 8)
4. Persistence-mechanism deferred-list consolidation (cycle 13 flag)
5. Tier-2 group 2 (review/disposition substrate, partial integration cycle 13)
6. Tier-2 group 4 (nine measures rework)
7. Tier-2 group 6 (preserved-through-cutover disposition)
8. Tier-2 group 7 (resolved open questions collapse)
9. Tier-2 group 9 (F8 singleton-family acknowledgment)

Net: 9 → 9 unchanged. Cycle 27 is Phase 1 evaluation and integration work, not
Phase 0 long-deferred items.

## Cycle 28+ pre-commits

1. **Tier-2 cross-system observations restructure.** With 8 systems read and 12-13
   patterns at 3+ system tier, the section's binary "3+ vs 2-system" structure is
   no longer the right organizing principle. Candidates for restructure: organize
   by family (state/memory; orchestration; defense/discipline; observability) or
   by maturity (load-bearing convergence vs supporting evidence). Architecturally-
   load-bearing decision; full structured pass.

2. **Refine the diversity-hedge language on elevated patterns.** Specifically the
   Append-only and Memory-as-first-class elevations. The cycle-27 cold-reader
   flagged these as needing stronger hedge language than the matrix produced.
   Re-read the elevated bullets and tighten where the hedge is buried. Bounded
   mechanical (~one cold-reader cycle).

3. **Re-check the NEW-pattern multi-system claims.** Specifically the Voyager-as-
   mechanical-enforcement overcount flagged in this cycle's cold-reader. Walk each
   new candidate's claimed system list against the source deliverable to verify
   the support is at the same framing strictness, not loose pattern-matching.
   Bounded mechanical for confirming; substantive if it produces re-elevations or
   demotions.

4. **Add NEW pattern candidates to single-system or 2-system tier.** After the
   cycle-28 re-check, the surviving NEW candidates (mechanical enforcement,
   plans-as-artifacts, entropy/AI-slop, context anxiety, pre-execution gating,
   agent legibility, autonomy directive) need to be added to `1-research.md`'s
   appropriate tier. Substantive prose work; full structured pass.

5. **Update Persistent Divergences section.** Cognition Devin's anti-stance on
   role-separation is a substantive divergence; it should be added to the
   Persistent Divergences subsection rather than left as a parenthetical in the
   elevated pattern's bullet. Also: the "throughput regime as moderating variable"
   observation from OpenAI Harness's wide-trust-boundary contradiction belongs in
   Persistent Divergences as a conditional. Substantive prose work.

6. **Cross-validate against audit's A-pattern mapping** (cycle-25 pre-commit 7,
   carry-forward from cycle 26). The cross-validation produces useful data for
   structural F-pattern → A-pattern reconciliation. Bounded mechanical (~one
   cold-reader cycle).

7. **Read remaining audit retrospective sections** (cycle-25 pre-commit 8,
   carry-forward from cycle 26). The "What v2 must demonstrably do better" section
   is the most relevant for Phase 2 candidate generation.

8. **Copilot research-only dispatch: oh-my-claudecode** (Eva directive #2774,
   GitHub repo). Carry-forward from cycle 26 pre-commit 4. Standard cycle-15
   procedure. Earliest dispatch: cycle 28 (now safe — cycle-26 dispatches all
   landed).

9. **Copilot research-only dispatch: openai/symphony** (Eva directive #2775,
   GitHub repo). Carry-forward from cycle 26 pre-commit 5. Earliest dispatch:
   cycle 28.

10. **Same-cycle cold-reader on this cycle-27 notes file.** Specific questions:
    (a) does the cross-validation matrix's SUPPORT/CONTRADICT scoring methodology
    read as principled or as ad-hoc pattern-matching? (b) does the cycle-28+
    pre-commit list (10 items now) start to threaten breakdown of the
    cycle-N-pre-commits-cycle-N+1-checks chain through sheer item count? (c) is
    the deferral pattern (apply 6 mechanical edits, defer 5 substantive ones)
    sustainable across the next 3-5 cycles, or does it accumulate Tier-2 backlog?

11. **Cycle-26 notes file cold-reader** (item 8 from cycle-26 plan,
    carry-forward — was supposed to be cycle 27 bounded mechanical work).
    Deferred this cycle because the three-PR evaluation consumed the cycle's
    budget. Bounded mechanical; can land cycle 28.

12. **Cycle-27 dispatch capacity decision: defer items 8-9 if cycle 28 has any
    integration backlog from this cycle.** The cycle-27 Tier-2 list (items 1-7)
    is heavy; adding two more dispatches in cycle 28 risks creating another 3-PR
    landing event in cycle 30+. Hold items 8-9 until cycle 29 or later.

### Suggested cycle 28 plan (provisional)

- **Focal:** item 1 (Tier-2 cross-system observations restructure) OR item 2
  (refine diversity-hedge language) — depending on appetite for architectural-
  load-bearing work vs bounded refinement
- **Bounded mechanical:** items 3 (NEW-pattern multi-system claim re-check),
  10 (this cycle's notes file cold-reader), 11 (cycle-26 notes file cold-reader)
- **Defer:** items 4-9, 12 to cycle 29+

## Persistence-mechanism observations

The cycle-N-pre-commits-cycle-N+1-checks chain extends to twenty-two cycles deep
(cycle 7 → ... → 26 → 27 → 28 pre-committed). 12 items in cycle-28 pre-commit
list — largest list yet (cycle 26 was 10). Question (b) of the same-cycle
cold-reader on this notes file (item 10 above) explicitly checks this
list-growth pattern.

**The three-dispatches-in-one-cycle decision (cycle 26) produced the predicted
3-PR-landing event in cycle 27.** All three landed within ~hours of dispatch
(faster than cycle-26's predicted 2-3 days). The cycle-27 evaluation work
absorbed the landing event in a single cycle by per-finding evaluating all
three deliverables in a shared cross-validation matrix rather than evaluating
each separately. This is a new pattern: when multiple dispatches land
simultaneously, batched cross-system evaluation is more efficient than
per-system evaluation when the cross-validation work is the dominant value
extraction.

**The discipline-lightening rule (codified cycle 25, applied cycles 25-26)
applied this cycle:** substantive prose (cross-validation matrix, cold-reader on
matrix) got full structured pass; bounded mechanical edits (count updates,
bullet promotions) got 30-second self-checks. Tally extended: substantive
rewrites 6/6 hits; bounded mechanical 0/6 hits (cycle 27's 6 mechanical edits);
architecturally-load-bearing section-opening 1/1 hits.

**The pivot-focal-mechanism pattern (cycle 26 named it) did not recur this
cycle.** WebFetch and curl remain gated, but cycle 27's work did not require
external URL access — all reading was from PR branches via `git show`, which
works in this environment. The pivot pattern is specific to orchestrator-direct
reads of external URLs, not to general PR reads.

**The honest-hedge pattern (named cycle 24) was invoked this cycle on the
Append-only and Memory-as-first-class elevations.** Both are 4-5 system
elevations where the convergence is at the framing level (architectural
elevation) and the divergence is at the mechanism level (in-process versioning
vs filesystem; vector store vs context trace vs repository vs wiki). The
hedge language is genuine to the data, not used to soften an actual
disagreement. Tally: 6/6 stable across cycles 24-27.
