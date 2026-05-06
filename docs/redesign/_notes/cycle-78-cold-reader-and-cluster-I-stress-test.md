# Cycle 78 (2026-05-06) — Cold-reader on cycle 77 + cluster I substrate-correlation stress-test

## Context

Cycles 62-77 ran the polarity-pivot research-corpus advancement arc
(sixteen consecutive cycles under
[#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829)) with
twelve distinct cycle composition shapes demonstrated. Cycle 77 ran
cold-reader-then-dispatch-construction composite (third instance of
that composite shape, hardening it at three instances), filing the
Symphony first-pass survey dispatch
[#2851](https://github.com/EvaLok/schema-org-json-ld/issues/2851) as
the ninth named system + last named-but-unread Phase 1 research target.
With Symphony filed, the Phase 1 dispatch queue is closure-bounded
(no further dispatch construction queued unless Eva names new targets
or survey findings motivate deeper-read follow-up).

Cycle 77's hand-off named priority order for cycle 78:

1. **Cold-reader on cycle-77 work (mandatory)** with three bounded
   questions: (a) Symphony hypotheses H1-H4 answerable at first-pass
   survey depth + not validation-fishing; (b) cycle-77's NEW lenses
   (4 spec-as-contract + 9 trust posture plurality) methodologically
   distinct from prior dispatch lenses; (c) finding C1 (driver vs
   meta-observation) re-tested if cycle 78 makes any deviation choice.
2. **If any dispatch returns:** per-finding evaluation absorption.
3. **Otherwise:** absorption / synthesis / cold-reader / audit-follow-up
   (NO further dispatch construction queued — Phase 1 dispatch queue
   closure-bounded).

**Startup check at cycle 78 fire (2026-05-06 03:00 UTC):**

- Four open dispatches still pending Eva's manual Copilot assignment:
  #2833 oh-my-codex (~15 cycles), #2842 PAI (~7 cycles), #2847
  oh-my-claudecode (~3 cycles), #2851 Symphony (~1 cycle). All have
  only `EvaLok` as assignee; no new substantive comments since cycle
  77 close. **No dispatch returns to absorb.**
- Audit-engagement [#2849](https://github.com/EvaLok/schema-org-json-ld/issues/2849)
  filed cycle 76 (2026-05-05 22:29 UTC); audit's most recent cycle
  predates the request, so audit's read of the request fires ~04:00 UTC
  today and audit's response will land in audit repo by ~2026-05-07
  04:00 UTC at earliest. Cycle 78 fired ~03:00 UTC (~1 hour before
  audit's expected next cron) so audit response is genuinely too early.
  **No audit response to absorb.**
- 1 open PR #2830 (standing structural absorption of #2829).
- 7 standing input-from-eva directives unchanged.

**Cycle 78 actually executed:** option 1 (cold-reader, mandatory) +
**stress-test cluster I substrate-correlation argument from existing
8-system data** (substantive focal — adversarial pre-test BEFORE
Symphony's empirical evidence arrives via H3). Cycle composition
shape: **cold-reader-then-stress-test composite, NEW shape (1st
instance)** — distinct from cold-reader-then-synthesis (cycle 74) by
posture (adversarial-disprove-claim vs additive-elaboration) and
distinct from cold-reader-then-dispatch-construction (cycles 71/75/77)
by deliverable (in-place repo edit + _notes file vs Copilot dispatch
issue body).

## Cold-reader findings (Q(a)/(b)/(c) per cycle 77 hand-off)

### Q(a): Symphony hypotheses H1-H4 answerable at first-pass survey depth + not validation-fishing

**Result: PASS WITH 1 MINOR FINDING (C1).**

For each hypothesis, constructed the smallest plausible CONFIRMED /
REFUTED evidence shape and verified the dispatch's specific architecture
questions probe those evidence shapes rather than presupposing them.

**H1 (organizational-substrate vocabulary):** 5 patterns — spec-driven
design, language-agnostic specification, RFC 2119, goals/non-goals
separation, implementation-defined plurality. CONFIRMED if 4 of 5
visible.

- All 5 patterns surface from lens 4 (spec-as-contract discipline)
  plus selectively from lenses 1, 9. SPEC.md is in scope for full
  read at survey depth (per dispatch instructions: "read fully if
  reachable at survey depth — 80KB ≈ 1500-2000 lines"), so all 5
  patterns are observable from SPEC.md text alone.
- Smallest CONFIRMED evidence: SPEC.md exists ≥80KB AND `Status: Draft
  v1 (language-agnostic)` declaration AND RFC 2119 vocabulary appears
  in MUST/SHOULD/MAY clauses AND section 2 has explicit goals/non-
  goals AND section 1 names ≥2 trust postures.
- Smallest REFUTED evidence: SPEC.md is descriptive after-the-fact OR
  <80KB OR no RFC 2119 OR single-impl prescriptive (locked to Elixir).
- Dispatch's specific questions probe genuinely (lens 4: "RFC 2119
  vocabulary usage — distribution of MUST/SHOULD/MAY clauses, where
  required vs recommended? Multi-implementation discipline — does the
  spec say anything specific about cross-impl conformance?"). The
  REFUTED case is sharp and falsifiable. **PASS.**

**H2 (Elixir/BEAM substrate exploitation):** 5 patterns — supervisor
trees, OTP behaviors, process isolation, message passing, application
config. CONFIRMED if 3 of 5 visible.

- Patterns observable from mix.exs deps + supervisor tree shape +
  config/ contents at survey depth. Lens 3 explicitly probes these
  ("Does Symphony use OTP supervisor tree explicitly? Where, what
  supervision strategies?").
- Smallest CONFIRMED evidence: lib/ contains modules using `use
  Supervisor`, `use GenServer`, `Task.Supervisor`, `DynamicSupervisor`
  AND mix.exs declares OTP-related deps AND config/ uses Application
  config layering.
- Smallest REFUTED evidence: lib/ is functional code only with no OTP
  behaviors, no supervisor tree, generic Elixir-as-functional-language
  usage with no BEAM-specific exploitation.
- Both shapes are observable at first-pass survey depth (read mix.exs
  + selective lib/ structural reads + config/ contents). Reading lib/
  files exhaustively is NOT in scope per dispatch instructions ("read
  selectively for load-bearing files; flag unread files for deeper-
  read"). The 3-of-5 threshold is achievable at survey depth. **PASS.**

**H3 (cluster I substrate-correlation prediction):** 5 patterns —
workspace isolation as policy, tracker-state-driven stop, implementation-
defined trust posture stratification, default-deny, capability-tier ×
role. CONFIRMED if 3 of 5 visible.

- Patterns observable from spec section 1 (trust postures, sandboxing
  language) + spec section 2.1 (tracker-state-driven stop) + lens 7
  (workspace isolation pattern) + lens 9 (trust posture plurality).
- Smallest CONFIRMED evidence: workspace isolation mechanism documented
  in spec/impl AND tracker-state stop mechanism in spec section 2.1 +
  impl AND ≥2 trust postures named in spec section 1.
- Smallest REFUTED evidence: workspace is just a directory with no
  isolation policy AND no tracker-state stop AND single-trust-posture
  (no plurality).
- Dispatch lens 9 explicitly probes plurality ("What does this mean
  concretely — what knobs / configurations does Symphony expose? How
  does the Elixir reference impl document its specific trust posture?
  What are the named trust levels (high-trust, sandboxed, etc.)?").
  **PASS.**

**H4 (spec-as-contract pattern, NEW for the corpus):** 3 sub-conditions
— operationally detailed AND multi-impl discipline explicit AND RFC
2119 throughout. CONFIRMED if all 3 visible. REFUTED if descriptive-
after-the-fact OR vague OR single-impl prescriptive.

- **Finding C1 (Q(a)): H4 CONFIRMED bar is partially pre-established
  by metadata sweep observations.** The pre-dispatch metadata sweep
  in cycle 77 already established (a) SPEC.md is 80KB, (b) declared
  `Status: Draft v1 (language-agnostic)`, and (c) uses RFC 2119
  vocabulary. So 2 of 3 sub-conditions are basically pre-established
  before the dispatch reads anything. The "operationally detailed" bar
  is the only one that genuinely requires the dispatch to read SPEC.md
  content at depth.
- The REFUTED case ("descriptive after-the-fact, vague high-level
  overview, or single-impl prescriptive") IS sharp and falsifiable.
  But the CONFIRMED case has a weak threshold because metadata-sweep-
  observable proxies satisfy 2 of 3 sub-conditions.
- **Real validation-fishing risk for H4:** the easier-to-satisfy 2 of
  3 sub-conditions are pre-known from metadata sweep; only operational-
  depth genuinely tests something the dispatch doesn't already know.
- **What would sharpen the test:** require evidence of (a) multi-impl
  conformance assertions, (b) conformance test suites, OR (c) concrete
  cross-impl behavior expectations beyond the README "Make your own"
  invitation. These are not pre-established by metadata sweep; they
  require the dispatch to verify whether multi-impl discipline is
  actually disciplined or just labeled.
- **Per cold-reader-cycle-N+1-checks-discipline applied to cycle-73/74
  _notes (preserve as record-of-time, do not edit), cycle-77 _notes
  is NOT modified.** Finding C1 is captured here in cycle-78 _notes
  for v2 design-input on hypothesis-construction discipline:
  metadata-sweep observations should be treated as "satisfied
  pre-condition" not "evidence for CONFIRMED." A genuine test sets
  the CONFIRMED bar at a level the dispatch has to actually verify.

### Q(b): Cycle 77's NEW lenses (4 spec-as-contract + 9 trust posture plurality) methodologically distinct from prior dispatch lenses

**Result: PASS WITH 1 MINOR OBSERVATION (C2).**

The hand-off Q(b) names lens 4 (spec-as-contract discipline) and
lens 9 (trust posture plurality) as the NEW lenses, asking whether
they overlap with #2847 lens 6 (documentation patterns) and #2847
lens 9 (anchoring caveats). Tested both:

**Lens 4 (spec-as-contract) vs #2847 lens 6 (documentation patterns):**

- #2851 lens 4 focuses on: SPEC.md structure (sections, headings,
  depth), RFC 2119 vocabulary distribution (MUST/SHOULD/MAY clauses),
  multi-implementation discipline, goals/non-goals separation
  structure.
- #2847 lens 6 focuses on: anti-pattern catalogs, hard-deprecation
  markers, non-goals presence, documentation-to-mechanical-enforcement
  linkage.
- **Overlap:** both touch non-goals and documentation-as-policy
  patterns.
- **Methodological distinction:** lens 4 adds RFC 2119 vocabulary
  quantification (counting MUST/SHOULD/MAY clauses, distinguishing
  normative requirements from informational background) AND multi-
  impl conformance discipline (cross-implementation contract
  semantics). Neither of these is in lens 6's scope. The analytic
  discipline of "what fraction of clauses are MUST vs SHOULD vs MAY,
  and where in the spec are MUSTs concentrated" is methodologically
  distinct from "are there anti-pattern catalogs."

**Verdict: METHODOLOGICALLY DISTINCT WITH OVERLAP.** Lens 4 has
substantive overlap with lens 6 (both address documentation discipline
including non-goals) but adds NEW methodological elements (RFC 2119
quantification, multi-impl conformance, normative-vs-informational
separation) that lens 6 doesn't.

**Lens 9 (trust posture plurality) vs #2847 lens 9 (anchoring caveats):**

- #2851 lens 9 focuses on: Symphony's named trust postures (high-trust,
  sandboxed, etc.), implementation-defined approval gates,
  knobs/configurations the Elixir reference impl exposes.
- #2847 lens 9 focuses on: how oh-my-claudecode's context differs from
  the redesign's context (autonomous vs interactive, single LLM vs
  multiple, broad vs narrow, community vs single-stakeholder).
- These are entirely different lenses. #2851 lens 9 is observational
  about Symphony's own design (what trust postures Symphony exposes).
  #2847 lens 9 is comparative against the redesign target (how the
  surveyed system differs from v2). The lens-numbers happen to match
  but the methodologies are unrelated.

**Verdict: METHODOLOGICALLY DISTINCT, NO OVERLAP.** Symphony's lens 9
serves a methodologically different purpose than oh-my-claudecode's
lens 9.

**MINOR OBSERVATION C2: Cycle-77 _notes claims "three NEW lenses
tailored for Symphony's spec-driven shape" (lenses 4, 5, 9), but the
hand-off Q(b) only names lenses 4 and 9.**

Tested lens 5 (workflow contract pattern, WORKFLOW.md) against #2847
lens 4 (Hooks and lifecycle integration with Claude Code):

- #2851 lens 5: "What's WORKFLOW.md's role — is it a per-deployment
  workflow definition? In-repo agent prompt? Operational config? How
  does Symphony load it (per spec)? What's the discipline around
  modifying it?"
- #2847 lens 4: "Does oh-my-claudecode use Claude Code's hook events?
  Which hooks does it register, what do they do at survey depth, how
  is registration done (.claude/settings.json hook block, custom
  mechanism, both)? Does the project add lifecycle structure
  (supervisor loops, custom phase boundaries, session-level state)
  above what Claude Code provides?"
- Both ask the same shape of question: "what is this specific surface,
  what does it do, how is it loaded/registered, what's the discipline
  around it." The methodology is parallel; only the specific surface
  examined differs (WORKFLOW.md vs `.claude/hooks/`).

**Lens 5 is SURFACE-TAILORED but NOT methodologically distinct.**
Cycle-77 _notes' claim of "three NEW lenses tailored for Symphony's
spec-driven shape" is correct under the **surface-tailored**
interpretation (all three lenses do address Symphony-specific surfaces
not directly probed by prior dispatches). But it's overstated under
the **methodologically distinct** interpretation (only lenses 4 and
9 add new analytic disciplines; lens 5 applies an existing methodology
to a new surface).

**v2 design-input from C2:** future cycle _notes should disambiguate
"surface-tailored" from "methodologically distinct" when characterizing
new dispatch lenses. The two terms are not interchangeable. The
cold-reader Q(b) was correctly scoped (asking about lenses 4 + 9, not
5) — the cycle-77 _notes' separate three-lens claim used a different
weaker criterion (surface-tailoring) without disambiguating from the
stronger criterion (methodological distinctness).

This is a sibling discipline failure to cycle-77's Finding C1
(driver-vs-meta-observation conflation). Both are about LANGUAGE
PRECISION when characterizing analytic claims:
- C1 (cycle 77): drivers vs meta-observations (different temporal
  positions relative to a decision)
- C2 (cycle 78): surface-tailored vs methodologically-distinct
  (different criteria for "novel" lens)

v2 design-input: cycle hand-off and _notes-writing discipline should
explicitly use precise term-pairs when characterizing analytic claims.

### Q(c): Cycle 78's deviation choice applies the C1 distinction (driver vs meta-observation)

**Result: PASS.**

Cycle 78 chose **stress-testing cluster I substrate-correlation** as
substantive focal, deviating from cycle 77's hand-off list of
"absorption / synthesis / cold-reader / audit-follow-up." Stress-
testing isn't in that list. So cycle 78 IS deviating from the hand-off's
recommended options — Q(c) applies.

**Drivers for stress-testing (reasons that explain the choice):**

1. **Symphony H3 explicitly tests cluster I substrate-correlation.**
   When Symphony's deliverable arrives, it'll either confirm (3rd
   convergent system → "argument hardens substantially") or refute
   ("argument needs reconsideration"). Pre-empirical adversarial
   re-read from existing 8-system data is more honest than post-
   empirical confirmation, because the orchestrator hasn't tried hard
   to falsify the claim before seeing the answer. **Real driver.**

2. **No dispatch returns and no audit response yet.** Substantive
   options narrow:
   - Absorption: empty (no dispatches returned).
   - Synthesis: cycles 65/70/72/74 already did synthesis (4 instances).
     Cycle 75 hand-off explicitly flagged "continued synthesis without
     new mining material starts producing diminishing returns."
   - Cold-reader: mandatory but bounded (~20-30 min, not whole cycle).
   - Audit-follow-up: too early (audit cron fires ~04:00 UTC; cycle
     78 fires ~03:00 UTC).
   - Stress-testing addresses a load-bearing claim that's about to be
     empirically tested, in a way none of the above options would.
   **Real driver.**

3. **ITERATION-UNTIL-APPROVAL action set explicitly names "Stress-test
   claims" as valid iteration action,** alongside Sharpen / Solicit-
   critique / Explore-alternatives / Deepen / Tighten / Examine-self-
   congratulation / Check-prescription-mismatches. Cluster I
   substrate-correlation is a load-bearing claim; pre-empirical
   adversarial test is the prescribed iteration action for this
   situation. **Real driver.**

**Meta-observations (insights surfaced by the choice, NOT driving it):**

1. **NEW cycle composition shape: cold-reader-then-stress-test
   composite (1st instance).** This shape was discovered by cycle 78
   making this choice; it didn't cause the choice. The composition
   shape catalogue gains a 13th distinct shape across cycles 62-78.
   Genuine meta-observation, not a driver.

2. **Cycle-77 hand-off list ("absorption / synthesis / cold-reader /
   audit-follow-up") is incomplete** — it omits stress-testing as an
   iteration action. The ITERATION-UNTIL-APPROVAL action set is
   broader than the hand-off list. This is a sibling pattern to
   cycle-76's finding (cycle-75 hand-off omitted audit-engagement)
   and reinforces the v2 design-input from cycle 76: cycle hand-off
   discipline should consistently include the full ITERATION-UNTIL-
   APPROVAL action set as candidate options. Genuine meta-observation,
   surfaced by cycle 78's choice not driving it.

**C1 distinction CORRECTLY APPLIED:** the three drivers above predate
and motivate the choice (Symphony H3 test relevance, narrowed
substantive options, ITERATION-UNTIL-APPROVAL prescription). The two
meta-observations are insights surfaced by the choice (new composite
shape, hand-off list incompleteness). Cycle 78 _notes should label
these distinctly — drivers in this section, meta-observations
separately in the meta-observations section.

### Cold-reader summary

**3/3 PASS** with 2 minor findings (C1 + C2) captured for v2 design-
input. Q(a) PASS WITH C1 (H4 CONFIRMED bar partially pre-established
by metadata sweep — validation-fishing risk on 2 of 3 sub-conditions).
Q(b) PASS WITH C2 (cycle-77 _notes' "three NEW lenses tailored"
language conflates surface-tailored with methodologically-distinct).
Q(c) PASS (cycle 78 deviation choice correctly applies C1 distinction).

**Pattern across cycles 73-78:** cold-reader catches at least one
finding sharper than the immediately-prior cycle's reflective work
surfaced. Cycle counts: 73 (3 minor: A1/A2/A3 + C1) / 74 (1: C2) /
75 (1: C3) / 76 (cycle 75 cold-reader covered it) / 77 (1: C1) /
78 (2: C1 + C2). The discipline's value is consistent: ~1-2 findings
per cold-reader, all minor, captured for v2 design-input on language-
precision and analytic-claim-construction discipline.

## Substantive work: stress-test cluster I substrate-correlation

Cycle 78's substantive focal: adversarial pre-test of the cluster I
substrate-correlation argument from existing 8-system data, BEFORE
Symphony's empirical evidence arrives via H3.

### Why pre-empirical stress-test (not post-empirical confirmation)

The cluster I substrate-correlation argument is load-bearing for v2
design-input:
- Original framing (clusters.md lines 299-315 + 433-444): cluster I
  "is correlated with cloud-anchored multi-actor environments. v1's
  substrate (GitHub-Actions-anchored multi-actor with audit) places
  it CLOSE to the cluster I correlation; Phase 2 candidates SHOULD
  weight cluster I patterns highly even at 2-system convergence
  depth, because the substrate alignment is strong."
- Cycle 70 strengthened: F↔I + D↔I + E↔I intersections cover all
  cluster I sub-shapes; "every cluster I sub-shape now has at least
  one full intersection discipline" (clusters.md lines 1075-1090).
- Cycle 77 H3 test: "If CONFIRMED [Symphony has cluster I]: cluster
  I has 3 convergent systems (openclaw, OpenAI harness, Symphony),
  substrate-correlation argument hardens substantially. If REFUTED:
  substrate-correlation argument needs reconsideration."

**Adversarial framing:** pre-empirical stress-test is more honest
than post-empirical confirmation. If Symphony confirms H3, post-
empirical analysis is biased toward agreement (orchestrator hasn't
tried hard to falsify before seeing the answer). If Symphony refutes
H3, post-empirical analysis is biased toward defending the prior
position. Stress-testing the argument BEFORE Symphony's evidence
arrives is the cleanest test of the argument's robustness.

**Three falsification angles tested:**

1. **False-positive case:** Is there a system in the 8-system corpus
   with cluster I patterns WITHOUT being multi-actor cloud-anchored
   substrate?
2. **False-negative case:** Is there a multi-actor cloud-anchored
   substrate system in the 8-system corpus WITHOUT cluster I patterns?
3. **Confounding-variable case:** Is there a confounding variable
   that better explains the cluster-I-correlation pattern?

### Substrate types per system (8-system corpus + Symphony predicted)

| System | Substrate | Multi-actor? | Cloud-anchored? | Cluster I status |
|---|---|---|---|---|
| AutoGen | library (TypeScript/Python framework) | Within-process multi-agent (programmatic, not user-facing) | No (library) | NOT documented; library substrate doesn't surface harness-policy patterns |
| LangGraph | library (Python) | Multi-agent supported, single-process default | No (library) | NOT documented |
| Cognition Devin | hosted product, microVM-isolated | Yes (Managed Devins, Code-Review-Loop, Smart Friend) | Yes (cloud) | **Has cluster-I-adjacent patterns NOT classified as cluster I** (microVM isolation, identity chaining bounded by dispatching engineer's permissions) |
| openclaw | local-first single-host daemon | Multi-device (companion apps) + agent-to-agent (disabled-by-default) | **NO (local-first)** | **YES** (1 of 2 documented cluster-I systems) |
| OpenAI harness | cloud-anchored harness for agent-first SDLC | Yes (multi-engineer + agents + CI) | Yes (cloud + GitHub) | YES (2 of 2 documented cluster-I systems) |
| Voyager | research artifact running locally | Single-agent self-loop | No (local) | NO (correctly absent — "research-artifact substrates ... runs locally with full environment access; no need for harness-enforced policy") |
| PAI | personal AI infrastructure, single-user | Single-user (16 principles oriented around user-centricity) | No (local-first stated as principle 4 / 5) | NOT documented at first-pass survey depth (#2842 deeper-read pending) |
| oh-my-codex | thin layer over Codex CLI, single-user dev | Single-user (developer's machine) | No (CLI substrate, single-user) | NOT documented at first-pass survey depth (#2833 deeper-read pending) |
| Symphony (predicted) | spec-driven multi-impl, Linear-tracker integration | Yes (Linear + agents + multiple humans + multi-issue concurrent runs) | Cloud-anchored deployment likely; spec is language-agnostic | **PREDICTED YES** (H3 test) |

### Falsification angle 1: False-positive case — openclaw

**openclaw is a counterexample to the "cloud-anchored multi-actor"
framing of cluster I substrate-correlation.**

Per per-system file `docs/redesign/1-research/systems/openclaw.md`
(line 56-58, 64-68): openclaw's load-bearing thesis is **local-first**:

> "OpenClaw is a personal AI assistant you run on your own devices.
> [...] The Gateway is just the control plane — the product is the
> assistant."
>
> Three named positions anchor the design:
> - **Local-first.** The Gateway runs on the user's machine, not in
>   a cloud. One Gateway per host. Companion apps (macOS, iOS, Android)
>   connect back to the local Gateway. Remote access is SSH tunnel or
>   Tailscale VPN — not a hosted backend.

Yet openclaw HAS cluster I patterns (per clusters.md line 299-307):

> permission-policy enforcement at the harness level decoupled from
> prompt-level rules (openclaw I-O1 — default-deny on multiple
> capability surfaces, before_tool_call.block-true terminal
> enforcement, plugin discovery/promotion gated by ClawHub security
> review)

openclaw I-O1's specific mechanisms (per per-system file lines 64-68):
- "Treat inbound as untrusted. Default DM policy is `pairing` (unknown
  senders receive a pairing code; bot does not process their message
  until explicitly approved). Workspace-origin plugins disabled by
  default. Agent-to-agent messaging disabled by default. The pattern
  is *default-deny, explicit-allow* applied across multiple capability
  surfaces."

**openclaw is local-first single-user, NOT cloud-anchored multi-actor,
yet has cluster I patterns.** This is a false-positive for the
strong reading of the substrate-correlation hypothesis.

**openclaw IS multi-actor in a non-cloud sense:**
- Companion devices (macOS, iOS, Android) all connect to the same
  Gateway — multi-device single-user.
- Multi-channel inbound surface (WhatsApp, Telegram, Slack, Discord,
  Signal, iMessage, WebChat) — multiple external untrusted-input
  surfaces.
- Agent-to-agent messaging (disabled by default but supported when
  configured) — programmatic multi-actor.
- Multi-agent routing within a Gateway (via bindings) — supported but
  each agent runs independently.

**The actual common factor across openclaw + OpenAI harness is more
specific than "cloud-anchored multi-actor":** both have *explicit
untrusted-input or multi-consumer boundary requiring policy enforcement
separate from agent reasoning*.

### Falsification angle 2: False-negative case — Cognition Devin

**Cognition Devin is candidate false-negative for "cloud-anchored
multi-actor → cluster I" prediction.**

Per per-system file `docs/redesign/1-research/systems/cognition-devin.md`
(line 136-138):

> **Sandboxing:** containers explicitly named insufficient (shared
> kernel = security threat). microVM with per-session kernel/storage/
> networking. Per-session identity chaining bounded by dispatching
> engineer's permissions.

Cognition Devin IS cloud-anchored multi-actor (microVMs, hypervisor
snapshot infrastructure, Managed Devins coordinator + parallel
children, Code-Review-Loop, Smart Friend cross-frontier consultation).
The substrate-correlation hypothesis would predict cluster I presence.

But the documented cluster I count (per clusters.md) is "2-system
convergent: openclaw + OpenAI harness." Cognition Devin is NOT in the
count.

**Why isn't Cognition Devin classified as cluster I?**

The cluster I sub-shapes (per clusters.md lines 301-307):
1. Permission-policy enforcement at the harness level decoupled from
   prompt-level rules (openclaw I-O1)
2. Quality-policy enforcement via mechanical linters with agent-
   readable error messages (OpenAI harness)

Cognition Devin's microVM + identity chaining is closer to
*infrastructure isolation* than either of these. It's a different
mechanism for different purpose (prevent VM-escape rather than prevent
capability-escalation through prompt manipulation). The sub-shape
catalogue may be too narrow.

**However, "Per-session identity chaining bounded by dispatching
engineer's permissions" IS a harness-enforced policy at the tool-call
layer** (the agent can't exceed the engineer's permissions). This
sounds like cluster I sub-shape 1 to me — permission-policy enforcement
at harness level decoupled from prompt-level rules.

**Possible interpretations:**

1. Cognition Devin DOES have cluster I (sub-shape 1 — identity chaining
   as permission-policy enforcement) and the cluster I count is
   undercounted at 2-system. If true, cluster I has 3 documented
   convergent systems already (openclaw + OpenAI harness + Cognition
   Devin) and Symphony H3 would make it 4. The substrate-correlation
   argument would be stronger than presented.

2. Cognition Devin's microVM + identity chaining is INFRASTRUCTURE-
   layer not HARNESS-layer (the cluster I distinguishing feature),
   and so is correctly excluded from cluster I. Cluster I is
   harness-policy-decoupled-from-prompt, microVM-isolation is
   below-the-harness sandbox. Different mechanisms for different
   threat models.

**Honest verdict:** the cluster I sub-shape catalogue is currently
narrow (2 sub-shapes from 2 systems); broadening to include
infrastructure-layer isolation as a distinct cluster-I-adjacent
sub-shape would be a Phase 2 design-input refinement. The current
"2-system convergent" framing is conservative; honest count if
infrastructure isolation is included would be 3 systems.

### Falsification angle 3: Confounding-variable case

Tested four candidate confounding variables that might better explain
the cluster-I-correlation pattern than "cloud-anchored multi-actor":

**Variable 1: Recency of publication.**
- Voyager: 2023 (cluster I absent — last commit 2023-07-27)
- AutoGen: 2023-2024 active, now maintenance mode (cluster I not
  documented)
- LangGraph: 2024-2026 active (cluster I not documented)
- openclaw: 2024-25 era (cluster I YES)
- OpenAI harness: 2026 writeup (cluster I YES)
- Cognition Devin: 2024-2026 active (cluster I disputed)
- PAI: 2024-2025 (cluster I not documented at survey depth)
- oh-my-codex / oh-my-claudecode: 2024-2025 (cluster I not documented
  at first-pass survey depth)
- Symphony: 2026 (predicted cluster I YES)

**Recency partially correlates with cluster I presence** — cluster I
systems are 2024+, the absent-cluster-I system (Voyager) is 2023. But
Recency alone doesn't distinguish openclaw (cluster I YES) from PAI
(cluster I not documented), both 2024-25. **Recency is correlated but
not load-bearing alone.**

**Variable 2: Organizational substrate (org-vs-solo).**
- Org-backed: AutoGen (Microsoft), Cognition Devin (Cognition), OpenAI
  harness (OpenAI), Voyager (research lab consortium), openclaw (org
  project — Gateway+plugins+ClawHub external review surface suggests
  team)
- Solo author: PAI (Daniel Miessler), oh-my-codex (Yeachan-Heo), oh-my-
  claudecode (Yeachan-Heo)

Cluster-I systems are all org-backed (openclaw, OpenAI harness, Cognition
Devin if included). Solo-author systems (PAI, oh-my-codex, oh-my-
claudecode) don't have cluster I documented. But Voyager is org-backed
research and lacks cluster I. Voyager's absence is explained by the
"research-artifact substrate" framing (runs locally with full env
access, no need for harness-enforced policy).

**Org-vs-solo correlates with cluster I but isn't sufficient alone.**

**Variable 3: Untrusted-input-or-multi-consumer-boundary.**
- Has explicit untrusted-input or multi-consumer boundary: openclaw
  (treat-inbound-as-untrusted from external chat platforms), OpenAI
  harness (agent-generated code as untrusted, multi-engineer + CI
  consumers), Cognition Devin (multi-Devin coordination + Linear-
  tracker external input), Symphony predicted (Linear-tracker external
  input + multi-issue concurrent runs + multi-human consumers)
- Lacks explicit untrusted-input boundary: Voyager (single-agent self-
  loop, no external untrusted input), PAI (single-user, all input is
  user-trusted), oh-my-codex / oh-my-claudecode (single-developer
  use, all input is developer-trusted)
- Library substrates: AutoGen, LangGraph (the library doesn't have a
  fixed "boundary" — each library user defines their own)

**This variable cleanly distinguishes cluster-I systems from non-
cluster-I systems.** Every documented cluster-I system has explicit
untrusted-input or multi-consumer boundary; every absent-cluster-I
system either lacks this boundary (Voyager, PAI, oh-my-codex) or is
substrate-undefined (libraries).

**Variable 4: External review/promotion gate presence.**
- openclaw: ClawHub external review for plugin promotion (explicit)
- OpenAI harness: CI mechanical linters as enforcement (explicit)
- Cognition Devin: per-session identity chaining bounded by dispatching
  engineer's permissions (explicit)
- Symphony predicted: spec-as-contract via RFC 2119 + multi-impl
  conformance discipline (explicit)
- Non-cluster-I systems: no external review/promotion gate documented

**This variable also correlates with cluster I presence.**

### Stress-test conclusion

**The substrate-correlation argument as currently framed in clusters.md
is OVERSTATED in one direction (false-positive openclaw under "cloud-
anchored multi-actor") and UNDER-COUNTED in another direction
(possible undercounting of Cognition Devin).**

**Re-characterized common factor across cluster-I systems:** *explicit
untrusted-input or multi-consumer boundary requiring policy enforcement
separate from agent reasoning*. This factor:
- INCLUDES openclaw (treat-inbound-as-untrusted from external chat
  platforms)
- INCLUDES OpenAI harness (agent-generated code as untrusted, multi-
  consumer engineering surface)
- INCLUDES Cognition Devin if microVM + identity chaining is broadened
  to count
- PREDICTS Symphony (Linear-tracker external input + multi-issue
  concurrent runs + multi-human consumers + spec-as-contract multi-
  impl discipline)
- EXCLUDES Voyager (single-agent self-loop, no external untrusted
  input — consistent with documented absence)
- EXCLUDES PAI / oh-my-codex / oh-my-claudecode (single-user, all
  input is user-trusted — consistent with absence at survey depth)
- LIBRARY-SUBSTRATE-UNDEFINED for AutoGen / LangGraph (the library
  doesn't fix a boundary; library users do)

**Implication for Symphony H3 prediction:** even under the
re-characterized factor, Symphony H3 PREDICTS cluster I presence
(Linear-tracker external input + multi-issue concurrent runs +
multi-human consumers + spec-as-contract multi-impl discipline all
satisfy the explicit-untrusted-input-or-multi-consumer-boundary
factor). The stress-test STRENGTHENS rather than weakens the H3
prediction.

**Implication for v2 design-input survives stress-test:** the original
conclusion ("Phase 2 candidates SHOULD weight cluster I patterns
highly because v1's substrate places it close to cluster I correlation")
HOLDS under the re-characterized factor. v1's substrate has explicit
untrusted-input boundary (issue text from Eva is trusted; issue text
from non-Eva accounts is untrusted per UNTRUSTED-TEXT-RULES; multi-
consumer surface includes Eva + audit + Copilot dispatches + cron
trigger). The "explicit-untrusted-input-or-multi-consumer-boundary"
factor APPLIES to v1.

**What needs revision in clusters.md:**

1. The framing "cluster I is correlated with cloud-anchored multi-actor
   environments" is OVERSTATED. The phrase "cloud-anchored" is
   incidental to OpenAI harness's specific instance, not the actual
   driver. openclaw is the local-first single-user counterexample.

2. **Better framing:** "cluster I is correlated with environments that
   have an explicit untrusted-input or multi-consumer boundary
   requiring policy enforcement separate from agent reasoning. The
   manifestations vary: cloud-multi-actor (OpenAI harness), local-
   multi-device-with-external-chat-input (openclaw), cloud-multi-actor-
   with-isolation (Cognition Devin)."

3. The cluster I sub-shape catalogue (currently 2 sub-shapes from 2
   systems) MAY be undercounted. Phase 2 evaluation should consider
   broadening to include infrastructure-layer isolation (Cognition
   Devin's microVM + identity chaining) as a third sub-shape; this is
   v2 design-input not Phase 1 corpus material.

**Per discipline applied to cycle-73/74 _notes (preserve as record-of-
time, do not edit), clusters.md is NOT modified this cycle.** The
stress-test result is captured here in cycle-78 _notes for v2 design-
input on substrate-correlation argument framing. If Symphony's H3
empirical result confirms (3rd convergent system at the strict cluster
I sub-shape definition), this stress-test result is the prior context
that should be integrated alongside the empirical confirmation in a
future synthesis cycle. If Symphony's H3 empirical result refutes,
this stress-test result is the prior context that should temper
"argument needs reconsideration" with "the re-characterized factor
predicted cluster I, so the argument's robustness depends on which
factor framing is used."

## Cycle composition shape: cold-reader-then-stress-test (NEW SHAPE)

Cycle 78 demonstrates the **first instance of cold-reader-then-stress-
test composite shape** in cycles 62-78. This is the **13th distinct
cycle composition shape** demonstrated in the polarity-pivot arc:

1. Mining (6× — cycles 62/64/66/67/68/69)
2. Dispatch-construction (4× — cycles 63/71/75/77)
3. Pure synthesis (3× — cycles 65/70/72)
4. Framework-iteration cold-reader fallback (1× — cycle 60)
5. Bounded-mechanical (2× — cycles 33/73)
6. Mining-with-synthesis-update (4×)
7. Per-finding-evaluation (multiple)
8. Diagnosis-and-mitigation (1× — cycle 71 self-healing)
9. Cold-reader-then-synthesis composite (1× — cycle 74)
10. Cold-reader-then-dispatch-construction composite (3× — cycles
    71/75/77, HARDENED at three instances)
11. Cold-reader-then-audit-engagement composite (1× — cycle 76)
12. *(no NEW shape cycle 77; cycle 77 is third instance of #10)*
13. **Cold-reader-then-stress-test composite (1× — cycle 78, NOVEL)**

**Distinctness from prior composite shapes:**
- vs cold-reader-then-synthesis (cycle 74): different posture
  (adversarial-disprove-claim vs additive-elaboration). Synthesis adds
  to the corpus; stress-test attempts to break the corpus.
- vs cold-reader-then-dispatch-construction (cycles 71/75/77):
  different deliverable (in-place repo edit + _notes file vs Copilot
  dispatch issue body). Dispatch-construction queues external work;
  stress-test produces self-contained evaluation.
- vs cold-reader-then-audit-engagement (cycle 76): different consumer
  (orchestrator-internal vs audit-orchestrator) and different round-
  trip discipline (in-cycle vs cross-repo asynchronous).

**v2 design-input: distinguish cycle composition shape categories by
HARDENED (3+ instances) vs TESTED (2 instances) vs NOVEL (1 instance):**
- Hardened (cycle 77 introduced category): mining (6×), mining-with-
  synthesis-update (4×), dispatch-construction (4×), cold-reader-then-
  dispatch-construction (3×), pure synthesis (3×), per-finding-
  evaluation (multiple).
- Tested: bounded-mechanical (2×).
- Novel: framework-iteration-cold-reader-fallback, diagnosis-and-
  mitigation, cold-reader-then-synthesis, cold-reader-then-audit-
  engagement, **cold-reader-then-stress-test (cycle 78 NEW)**.

Hardened shapes are reliable enough for v2 prescription; novel shapes
need additional evidence. v2 cycle composition catalogue should
distinguish these categories explicitly.

## Open follow-ups / hypotheses for cycle 79

These should be picked up when the orchestrator next fires:

1. **Cold-reader on the cycle-78 work** per the cycle-N-pre-commits-
   cycle-N+1-checks discipline. Three bounded questions for cycle 79:
   - **(a)** The cluster-I-substrate-correlation stress-test conclusion
     (re-characterized factor: explicit-untrusted-input-or-multi-
     consumer-boundary) — verify the re-characterization actually
     predicts what it claims to predict by walking each of the 8
     systems against the new factor and checking consistency. If any
     system's classification under the new factor disagrees with the
     observed cluster-I-presence, the re-characterization is incomplete.
   - **(b)** Findings C1 + C2 (cycle 78) — verify they're properly
     captured for v2 design-input on language-precision discipline.
     Specifically: are the proposed term-pairs (driver vs meta-
     observation, surface-tailored vs methodologically-distinct)
     enough, or are there other analytic-claim term-pairs that v2
     hand-off discipline should explicitly distinguish?
   - **(c)** The cold-reader-then-stress-test composite shape — re-test
     by checking whether cycle 79 (or a future cycle) uses a similar
     adversarial-disprove-claim posture. If so, the shape should be
     elevated from NOVEL to TESTED or HARDENED based on count.

2. **Dispatch state.** Four open dispatches now (#2833, #2842, #2847,
   #2851). If Eva's manual Copilot assignment lands by cycle 79,
   dispatches may start processing — per-finding evaluation absorption
   becomes priority cycle composition shape. Symphony H3 result is
   the highest-leverage absorption: if confirmed, it cleanly validates
   one of the substrate-correlation framings; if refuted, the cycle-78
   re-characterized factor becomes the salvage path.

3. **Audit-engagement state.** Audit cron fires daily ~04:00 UTC.
   Audit's most recent cycle (#451) fired 2026-05-05 04:02 UTC,
   predating the audit-engagement request [#2849](https://github.com/EvaLok/schema-org-json-ld/issues/2849)
   filed cycle 76 (2026-05-05 22:29 UTC). Audit's next cycle fires
   ~04:00 UTC today (2026-05-06); audit's response will land in audit
   repo by ~2026-05-07 04:00 UTC at earliest. Cycle 79 may have audit
   response to absorb.

4. **Phase 1 dispatch queue closure-bounded continues.** No further
   dispatch construction queued unless (a) Eva names new research
   targets, or (b) survey findings from one of the 4 open dispatches
   motivate a deeper-read follow-up dispatch.

5. **Hypotheses for future cycles' substantive focal options:**
   - If audit response lands cycle 79: per-question evaluation
     absorption (audit's 6 specific questions on cluster framework
     robustness, intersection sub-pattern padding, v1 failure-mode
     mapping generosity, substrate-correlation robustness, mixed-
     symmetry framing, polarity-pivot honesty).
   - If Symphony returns cycle 79+: per-finding evaluation absorption
     (especially cluster I H3 result + spec-as-contract H4 result
     + Elixir/BEAM exploitation H2 result).
   - If neither: continued stress-testing on other load-bearing claims
     (e.g., mixed-symmetry framing for A↔C from cycle 75/76; cluster F
     unified-vs-decomposed open structural question; cluster A vs
     cluster G boundary for per-action retry vs role-asymmetric context).

## Authority

Cycle 78 work proceeds under:

- The redesign-prompt's [`<initial-directive>`](https://github.com/EvaLok/schema-org-json-ld/blob/master/.github/workflows/orchestrator-prompt.xml)
  Phase 1 authorization (cycle 14+, ongoing).
- Eva's [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829)
  substantive-focal polarity inversion (research expansion as default,
  framework iteration as bounded-mechanical fallback).
- The redesign-prompt's [`ITERATION-UNTIL-APPROVAL`](https://github.com/EvaLok/schema-org-json-ld/blob/master/.github/workflows/orchestrator-prompt.xml)
  discipline naming "Stress-test claims: pick a load-bearing claim
  in the artifact, try to disprove it, document what survives and
  what doesn't" as valid iteration action — explicitly authorizing
  cycle 78's substantive focal.
- The cycle-N-pre-commits-cycle-N+1-checks discipline established
  cycle 7+ and extended through Phase 1.
- The cycle-77 hand-off recommendation explicitly naming substantive
  options as "absorption / synthesis / cold-reader / audit-follow-up"
  — cycle 78 deviates from this list to stress-test (per drivers
  documented in Q(c) above), with stress-test being a member of the
  ITERATION-UNTIL-APPROVAL action set that the cycle-77 hand-off list
  omitted.

## Persistence-mechanism note

Cycle 78's contributions to cross-cycle persistence:

- (a) This notes file (~~640 lines documenting cold-reader findings
  + Q(a)/Q(b)/Q(c) per-question outputs + finding C1/C2 + cluster I
  substrate-correlation stress-test + cycle composition shape extension
  + cycle-79 hypotheses).
- (b) Journal entry.

The cycle-73 file restructure continues to pay off: `clusters.md`
remains 1269 lines (unchanged this cycle — cycle 78 stress-test is
captured in _notes per discipline, not edited into clusters.md),
`1-research.md` remains ~822 lines (unchanged). **Cycle 78's
persistence shape is mostly _notes-resident** (no issue-tracker
artifacts, no comment threads, no dispatch issues). Distinct from
cycles 76 + 77 whose persistence shape was mostly issue-tracker-
resident (audit-engagement + supplement comment + dispatch + dispatch-
assignment-note); cycle 78 is the first cycle since cycle 74 whose
output is fully repo-internal. The persistence-mechanism shape varies
with cycle composition shape — issue-tracker-resident shapes correlate
with dispatch-construction or audit-engagement; _notes-resident shapes
correlate with synthesis or stress-test.

## Iteration-until-approval honest reflection

Cycle 78 produces three substantive contributions:

1. **Cold-reader integrity check** — 3/3 PASS with findings C1
   (H4 CONFIRMED bar partially pre-established by metadata sweep —
   validation-fishing risk on 2 of 3 sub-conditions) and C2 (cycle-77
   _notes' "three NEW lenses tailored" language conflates surface-
   tailored with methodologically-distinct). Captured for v2 design-
   input on hypothesis-construction discipline and language-precision
   discipline.

2. **Cluster I substrate-correlation stress-test** — pre-empirical
   adversarial test from existing 8-system data. Found openclaw is
   counterexample to "cloud-anchored multi-actor" framing (local-
   first single-user yet has cluster I patterns); Cognition Devin is
   candidate false-negative if cluster I sub-shape catalogue is
   broadened to include infrastructure-layer isolation. Re-characterized
   common factor: "explicit untrusted-input or multi-consumer boundary
   requiring policy enforcement separate from agent reasoning" —
   predicts cluster-I-presence consistent with all 8-system observations
   AND predicts Symphony H3 confirmation under the re-characterization.
   v2 design-input survives stress-test; argument framing in clusters.md
   needs revision when integrating Symphony H3 empirical result.

3. **Cycle composition shape extension** — cold-reader-then-stress-test
   composite is 13th distinct shape, NOVEL (1 instance). v2 design-
   input on distinguishing HARDENED vs TESTED vs NOVEL cycle composition
   shape categories.

The deliverable for Phase 2 (when authorized post-retrospective-
checkpoint) is improved in three ways:

1. Cycle 78 stress-test of cluster I substrate-correlation argument
   surfaces a re-characterization that's more honest than the original
   framing — "explicit-untrusted-input-or-multi-consumer-boundary"
   factor instead of "cloud-anchored-multi-actor" factor. Phase 2
   candidate authors should be told the load-bearing factor (the
   stress-tested re-characterization) rather than the easier-to-state
   correlation that has a counterexample.

2. Cycle 78 cold-reader findings C1 + C2 add to the language-precision
   discipline material started cycle 77 with C1 (driver-vs-meta-
   observation). Three precise term-pairs now identified for v2 hand-
   off discipline:
   - drivers vs meta-observations (different temporal positions
     relative to a decision)
   - surface-tailored vs methodologically-distinct (different criteria
     for "novel" lens)
   - satisfied-pre-condition vs evidence-for-CONFIRMED (different
     epistemic statuses for hypothesis sub-conditions)

3. Cycle composition shape catalogue extended to 13 distinct shapes
   with HARDENED/TESTED/NOVEL category distinction. Phase 2 cycle
   composition catalogue should distinguish these categories explicitly
   since they have different prescription-strength.

The substantive-work decision (stress-testing over absorption/synthesis/
cold-reader/audit-follow-up) is honest follow-through on the
ITERATION-UNTIL-APPROVAL action set — stress-testing is a named action
in that set. Cycle 77 _notes' hand-off list ("absorption / synthesis /
cold-reader / audit-follow-up") was incomplete in omitting stress-
testing; cycle 78 surfaces this as a meta-observation about hand-off
discipline (not a driver). Q(c) confirms the deviation reasoning
correctly applies the C1 distinction.

The bottleneck remains external (Eva's manual Copilot assignment now
on 4 stuck dispatches; audit cron fires asynchronously). Cycle 78's
contribution is asynchronous adversarial test that doesn't depend on
either bottleneck — cluster I substrate-correlation can be stress-
tested from existing 8-system data, no new external evidence needed.
This is consistent with the cycle-77 observation that "the 4-stuck-
dispatches pattern is structural" — orchestrator productivity should
be designed to not depend on the dispatch bottleneck.
