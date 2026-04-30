# Cycle 29: NEW pattern integration into `1-research.md` Cross-system observations + cold-reader on cycle-28 notes

Cycle 28 (commit `a9b8ec7e`) ran a deeper recheck of cycle-27's
cross-validation matrix's NEW pattern claims (3/9 corrections at deep
inspection vs 1/3 at spot-check), fixed a count-vs-description
inconsistency in the Memory bullet, tightened diversity-hedge style
on two elevated bullets, and ran cold-readers on cycles 27 and 26.
Item 1 (Tier-2 cross-system observations restructure) was deferred
with explicit rationale: the 2-system tier was empty pending NEW
pattern integration, so restructuring before the population
stabilizes would mean restructuring twice. Cycle-28's pre-commit list
named cycle 29's focal as item 1 (NEW pattern integration with
corrected counts), the largest deferred substantive item.

Cycle 29 absorbed item 1 as the focal: integrated 4 NEW patterns
into the 2-system subsection of `1-research.md` (re-populating the
empty subsection per cycle 27's elevation) and 5 single-system
observations into `_notes/cycle-22-cross-system-synthesis.md`, using
the corrected counts from cycle-28's per-row recheck. Same-cycle
cold-reader on the integration; cycle-N+1 cold-reader on cycle-28
notes (item 11 of cycle-28 pre-commits, bounded mechanical). Cycle
29+ pre-commits at the bottom.

## Item 1 absorbed: NEW pattern integration

### Placement decision: 2-system tier vs single-system

Cycle-28 produced corrected counts for 9 NEW pattern candidates. The
placement decision per pattern:

| # | Pattern | Strict count | Loose count | Placement |
|---|---|---|---|---|
| 1 | Mechanical enforcement of regression-tested behavioral constraints | 2 (OpenAI, oh-my-codex) | 4 (+ Voyager init-time, LangGraph type-system) | **2-system tier** with diversity hedge |
| 2 | Plans/specs as first-class forward-versioned artifacts | 2 (OpenAI, oh-my-codex) | 2 | **2-system tier** clean |
| 3 | Entropy / AI slop as first-class engineering concern | 2 (OpenAI, oh-my-codex) | 2 | **2-system tier** clean |
| 4 | Context anxiety / model self-model failures | 1 (Cognition) | 1 | **single-system** (`_notes/cycle-22`) |
| 5 | Pre-execution gating against underspecified requests | 1 (oh-my-codex) | 1 + Cognition adjacent at different lifecycle | **single-system** (`_notes/cycle-22`) |
| 6 | Agent legibility / repo structured for agent comprehension | 1 explicit (OpenAI) | 3 (OpenAI explicit + Cognition implicit + oh-my-codex implicit) | **single-system** (`_notes/cycle-22`) — preserving "1 explicit + 2 implicit" framing per cycle-28 |
| 7 | Throughput-based merge philosophy | 1 conditional (OpenAI) | 1 | **single-system** (`_notes/cycle-22`) — with "moderating variable on Strong-defaults security" framing for Persistent Divergences candidate |
| 8 | Iteration ceilings with explicit numerical limits | 2 (oh-my-codex, Voyager) | 2 + Cognition partial (time-limit at different axis) | **2-system tier** with Cognition-adjacent partial |
| 9 | Autonomy directive prominently stated | 1 explicit (oh-my-codex) | 3 (OpenAI adjacent at different framing + Cognition marketing-tagline) | **single-system** (`_notes/cycle-22`) — preserving "1 explicit + 2 implicit/adjacent" framing per cycle-28 |

Decision rule: strict 2-system or higher → 2-system tier in
`1-research.md`; strict 1-system or borderline → single-system
observations in `_notes/cycle-22`. The borderline cases (#6, #9)
were explicitly preserved at single-system per cycle-28's recheck
verdict that the "1 explicit + 2 implicit/adjacent" framing is
honest at strict framing.

### Bullet construction — the four 2-system bullets

Modeled on the existing 2-system-then-elevated-to-3+ bullets'
structure (bold title, parenthetical with cycle-N marker, per-system
evidence in flowing prose, count summary, optional diversity-hedge
clause):

**(a) Mechanical enforcement of regression-tested behavioral
constraints** — opens with the parenthetical naming both strict
framing (2-system) AND loose-framing extensions (Voyager, LangGraph)
to address cycle-28's correction directly. Body cites OpenAI's
custom-linters-with-agent-readable-error-messages + golden-principles
mechanically checked (deliverable patterns 8/9/12 from PR #2783) and
oh-my-codex's `src/hooks/__tests__/prompt-guidance-*.test.ts`
behavioral prompt-contract regression tests (deliverable pattern 7
from PR #2784). The diversity-hedge clause names Voyager's
SkillManager + CurriculumAgent init-time vectordb-vs-JSON count
assertion (init-time-only, on data-state) and LangGraph's
TypedDict / dataclass / Pydantic BaseModel channel-type / reducer-
contract validation (data shape, not behavior) as different-scope-
and-rigidity instances of the broader principle. Both loose-framing
extensions named honestly — they share the principle (mechanically-
checked invariants over documented rules) but at different scope
and rigidity, counted as loose-framing convergence on the broader
mechanical-enforcement principle, not as 4-system strict.

**(b) Plans/specs as first-class forward-versioned artifacts** —
clean 2-system without diversity hedge (cycle-28 confirmed clean).
Body cites OpenAI plans-as-first-class-versioned-artifacts (active /
completed / technical-debt plans checked into repo, deliverable
pattern 7 from PR #2783) and oh-my-codex `.omx/context/{task-slug}-
{timestamp}.md` context-snapshot-grounding (deliverable pattern 2
from PR #2784). Closing clause distinguishes from the elevated
"Failed work as recorded artifact" pattern in the 3+ section
(forward-spec vs backward-history) — this addresses cycle-28's
correction that Voyager's curriculum log is task-history not plan-
spec, AND prevents future cold-readers from misreading plans-as-
artifacts as duplicating the failed-work pattern.

**(c) Entropy / AI slop as first-class engineering concern** — clean
2-system. Body cites OpenAI entropy-as-first-class + golden-
principles + doc-gardening agent (deliverable patterns 11/12 from PR
#2783) and oh-my-codex deslop pass as mandatory post-completion step
(deliverable pattern 13 from PR #2784). Closing clause notes the
inverse-relation to the redesign's prior accretion-as-defense
framing (these systems treat accretion as failure mode to clean,
not defensive structure to preserve). The inverse-relation framing
is observation, not v2-prescription — naming the contrast for future
cycles' Phase 2 work without prescribing a v2 stance.

**(d) Iteration ceilings with explicit numerical limits** — 2-system
strict + Cognition partial. Body cites oh-my-codex
`max_iterations=10` for tool-loops, `max=5` for review-loops, and
autoresearch keep/discard/stop per-iteration (deliverable pattern 6
from PR #2784) and Voyager `action_agent_task_max_retries = 4` with
critic-critique + execution-error fed into next prompt
(`1-research.md` line 466) — the latter being the cycle-28-corrected
cycle-27-MISSED entry. Cognition Devin's 45-min session time limit
explicitly framed as adjacent at different bounding-axis (time vs
iteration-count), with the *documented-claim per cycle-26 source-
access note* preserved for the source-credibility caveat.

### Single-system additions to `_notes/cycle-22`

Five new H3 entries appended to the existing single-system
observations section, each marked with "**Added cycle 29 from
cycle-26 dispatch deliverables**" header suffix to mirror the
existing "**ELEVATED to 2-system convergence cycle 25**" suffix
convention:

(1) **Cognition Devin: context anxiety / model self-model failures**
— captures the four related findings from the Cognition deliverable
(context-anxiety, environmental-deception, prompt-placement,
misestimation-precision) and the "lying to the model" workaround.
Notes that no other system documents model-self-model-failures as a
named architectural concern. Documented-claim caveat preserved.
Cross-validation candidate flagged: oh-my-claudecode and openai/
symphony queued (Eva directives #2774 / #2775).

(2) **oh-my-codex: pre-execution gating against underspecified
requests** — 44KB keyword detector + word-count signal-detection
gate, deterministic pre-flight, `force` bypass. Cognition's context-
anxiety mitigation explicitly framed as adjacent at *different
lifecycle phase* (mid-execution vs pre-execution). The shape "guard
against the model under-reading work required" shared across the
two but at different points in the lifecycle.

(3) **OpenAI Harness: agent legibility as optimization target** —
the "from the agent's point of view, anything it can't access in-
context while running effectively doesn't exist" framing as the
strongest published statement that agent comprehension is an
architectural concern. Cognition implicit (context-engineering
thesis) and oh-my-codex implicit (`templates/AGENTS.md`,
`CONTRIBUTING.md` `<Bad>`/`<Good>` examples, role-prompts) explicitly
framed as different from OpenAI's explicit naming. Cycle-28's "1
explicit + 2 implicit" verdict preserved. Cross-validation candidate:
whether subsequent reads surface another system that names agent-
legibility as explicit design driver.

(4) **OpenAI Harness: throughput-based merge philosophy with
conditional scope** — corrections-cheap-waiting-expensive,
agents-merge-own-PRs, per-worktree-isolation-as-practical-containment
(deliverable pattern 16 from PR #2783). Conditional scope
preserved per cycle-27 first-pass note. Cross-system observation
flagged: throughput-regime-as-moderating-variable on Strong-
defaults-security 3+ pattern (not contradiction at the original 3
systems, but counter-example showing scope condition). Persistent
Divergences candidate flagged for cycle-29+ pre-commits.

(5) **oh-my-codex: autonomy directive prominently stated** —
`templates/AGENTS.md` all-caps autonomy block. OpenAI "Humans steer.
Agents execute." (deliverable pattern 2 from PR #2783) framed as
adjacent at different framing level (architectural-philosophy vs
agent-instruction-level autonomy directive). Cognition "fully
autonomous AI software engineer" framed as marketing tagline (loose
adjacency). Cycle-28's "1 explicit + 2 implicit/adjacent" verdict
preserved. Cross-validation candidate: oh-my-claudecode (queued per
Eva directive #2774) given its naming relation to oh-my-codex.

## Same-cycle cold-reader on the integration

Per the cycle-19 same-cycle-cold-reader-on-rewrite pattern (now
tested 4+ times), running the cold-reader on the new section
before commit.

### Anti-smuggling discipline on the four new 2-system bullets

Walked each new pattern bullet for v2-prescription smuggling:

- **Mechanical enforcement**: pure observation; quoted system
  framings (PR #2783 "Mechanical enforcement over documented rules";
  oh-my-codex regression-test file path); diversity hedge names
  Voyager and LangGraph as loose-framing extensions without
  prescribing what v2 should do. PASS.
- **Plans-as-artifacts**: pure observation; per-system citations
  (OpenAI active/completed/technical-debt plans; oh-my-codex
  `.omx/context/...` path); closing clause about distinction-from-
  failed-work-pattern is intra-section observation, not
  prescription. PASS.
- **Entropy/AI-slop**: closing clause naming "inverse-relation to
  the redesign's prior accretion-as-defense framing" is borderline
  — could be misread as v2-prescription if reader assumes accretion-
  as-defense IS the v2 path. Re-reading: the framing is "these
  systems treat accretion as failure mode to clean, not defensive
  structure to preserve" — this is observation about external
  systems' stance, with the redesign's prior framing as comparison
  point. The line "Inversely-related to the redesign's prior
  accretion pattern" is observation, not "v2 should do this." PASS
  with the framing flagged for cycle-30+ fresh-eye cold-reader.
- **Iteration ceilings**: pure observation; per-system citations
  (oh-my-codex `max_iterations=10`; Voyager `action_agent_task_max_retries
  = 4`; Cognition 45-min session time limit). The "documented-claim"
  caveat preserves source-credibility asymmetry. PASS.

### Anti-smuggling discipline on the five new single-system observations

Walked each new observation for v2-prescription smuggling:

- **Cognition context-anxiety**: pure observation about Cognition's
  documented findings; "documented-claim" caveat preserved; cross-
  validation candidate flagged honestly (no prescription that v2
  should adopt this). PASS.
- **oh-my-codex pre-execution gating**: pure observation; "Cognition
  adjacent at different lifecycle phase" framing names the shape-
  match honestly without claiming convergence. PASS.
- **OpenAI agent-legibility**: pure observation; "1 explicit + 2
  implicit" framing preserves cycle-28's verdict; closing
  cross-validation candidate is forward-looking observation, not
  prescription. PASS.
- **OpenAI throughput philosophy**: borderline — the "moderating
  variable on Strong-defaults security" framing is observation
  about Cross-system pattern scope but COULD be misread as
  prescribing scope-conditions for v2. Re-reading: the framing is
  "the throughput regime acts as a moderating variable on the
  security-stance pattern" — observation about external systems'
  divergence under different operating regimes, with explicit
  acknowledgment that the security pattern's truth at the original
  3 systems is not contradicted. The Persistent Divergences
  candidate flag is process (where to put the observation), not
  v2-prescription. PASS with the framing flagged for cycle-30+
  fresh-eye cold-reader.
- **oh-my-codex autonomy directive**: pure observation; per-system
  framings (OpenAI role-allocation thesis at architectural-
  philosophy level; Cognition marketing tagline at loose framing);
  cross-validation candidate is forward-looking. PASS.

### Cycle-18 transferability symmetry

Each 2-system bullet has explicit count + diversity acknowledgment
where applicable:
- Mechanical enforcement: 2-system strict + diversity hedge for 2
  loose-framing extensions explicitly named.
- Plans-as-artifacts: 2-system clean (no diversity hedge needed).
- Entropy/AI-slop: 2-system clean (no diversity hedge needed).
- Iteration ceilings: 2-system strict + Cognition partial at
  different bounding-axis explicitly named.

The strict-vs-loose framing distinction is most visible in the
mechanical-enforcement bullet, where cycle-28's "name the strict
framing in the bullet ... and note the loose extension as separate
clause" guidance is followed literally. The bullet's parenthetical
opens with strict-framing 2-system count THEN names the loose-
framing extensions; the body cites the strict-framing systems first
THEN the diversity-hedge clause names the loose-framing systems.
The discipline is honored.

### Self-introduced errors check (per cycle-19 lesson)

No real self-introduced errors found on this cold-reader pass. The
4 new 2-system bullets and 5 new single-system observations were
drafted with the existing section's conventions in mind (Cognition
"*documented-claim per cycle-26 source-access note*" caveat applied
in Iteration-ceilings and Cognition-context-anxiety; per-system
file-path / pattern-number citations matching the existing 3+ tier
bullets' style; diversity-hedge clause structure matching the
elevated Append-only and Memory bullets).

This is the fifth test of the same-cycle-cold-reader-on-rewrite
pattern (cycles 19/21/22/28/29). The first four tests surfaced real
concerns; this one does not. **Pattern observation**: the no-finding
result is itself information — when the substantive material is
densely connected to existing conventions (this cycle's integration
extends a structure with 12 existing bullets to absorb), the drafting
phase can absorb the convention-checking that the cold-reader
otherwise has to surface separately. When the material is novel
structure (cycle-22's first-time cross-system synthesis from scratch
was novel; cycle-19's Tier-2 group 3 retrospective restructure was
novel), the cold-reader catches more.

Mitigation against false-negative bias on this no-finding result:
the two flagged borderline framings (entropy/AI-slop "inversely-
related to redesign's prior accretion pattern"; throughput
"moderating variable on Strong-defaults security") ARE real
concerns that this cold-reader caught even though they don't rise
to "self-introduced error" level. They're flagged for cycle-30+
fresh-eye second-pass. So the cold-reader is doing its job; the
no-finding result on the strict "self-introduced error" bar is
honest, not lazy.

### Section-transition smuggling check

- Transition from `### Patterns converging across 3+ systems` to
  `### Patterns converging across 2 systems` (line 1097-1098): the
  3+ section ends with the "Small fixed team" bullet (last elevated
  pattern with Cognition contradiction); the 2-system section opens
  with the cycle-29-updated intro paragraph naming cycle-27
  elevations + cycle-29 re-population. Clean section break.
- Transition from `### Patterns converging across 2 systems` to
  `### Persistent divergences` (line ~1185): 2-system section ends
  with iteration-ceilings closing sentence; Persistent divergences
  section opens with bullet on Agent-hierarchy-stance. Clean
  section break.
- Transition from cycle-22 single-system observations to "What
  surprised me this cycle" (line ~134): existing transition; the
  new 5 single-system observations were appended to the
  pre-existing 5, before the "What surprised me" header. Clean.

### Cold-reader verdict

PASS. Two borderline framings flagged for cycle-30+ fresh-eye
cold-reader (entropy/AI-slop "inversely-related to redesign's prior
accretion pattern"; OpenAI throughput "moderating variable on
Strong-defaults security"). Both are observation-shaped on close
read, not prescription, but borderline enough to warrant a second
pass when fresh-eye reading is available.

## Item 11 absorbed: Cold-reader on cycle-28 notes file

The cycle-28 plan named three specific questions for the cycle-29
cold-reader (item 11 of cycle-28 pre-commits):

(a) Does the 33%-correction-rate finding on cycle-27 matrix get the
    weight it deserves, or does it get buried in the broader cold-
    reader narrative?
(b) Does the deferral rationale for item 1 (Tier-2 restructure) read
    as principled sequencing or as procrastination?
(c) Does the third-category refinement candidate for the discipline-
    lightening rule add genuine value, or is it speculative process-
    layering?

### Question (a): 33%-correction-rate weight — DESERVED, well-placed

Cycle-28's notes file places the 33%-correction-rate finding at the
end of "Item 3" (the recheck section, lines 185-198) AND in the
"Persistence-mechanism observations" section (lines 631-639). The
double-placement gives the finding load-bearing prominence: once at
the empirical-record-of-this-cycle level, again at the meta-pattern
level. The finding is also referenced in the cycle-28 journal entry
("the deeper recheck found more errors than the spot-check") and in
the cycle-28 README iteration log entry ("33% rate") — appearing in
4 places total.

The finding's substantive weight (1-in-3 NEW pattern claims had
either overcount or undercount at deep inspection vs only 1-in-6 at
spot-check) IS load-bearing for cycle-29's focal: cycle 29 used the
corrected counts, not the cycle-27 first-pass matrix counts.
Without the recheck, cycle-29 would have integrated 4 systems for
mechanical enforcement (vs the corrected 2-strict + 2-loose), 3
systems for plans-as-artifacts (vs the corrected 2 clean), 1-2
systems for iteration limits (vs the corrected 2-strict + 1-partial).
The cycle-28 recheck's load-bearingness on cycle-29 is direct.

**Verdict (a):** PASS. The 33%-finding gets the weight it deserves
across multiple placements; it directly shaped cycle-29's
integration; the prominence is well-placed.

### Question (b): item 1 deferral rationale — PRINCIPLED SEQUENCING

Cycle-28's notes file dedicates the entire "Item 1" section (lines
283-331) to the deferral rationale. Four named reasons:
1. 2-system tier will re-populate cycle 29+ → restructure now means
   restructuring twice
2. Final shape isn't visible yet → restructure decisions best made
   when population is closer to final shape
3. Current placeholder is functional → artifact isn't broken by
   the deferral
4. Cycle-28 budget consumed by items 2/3/10/11 → adding restructure
   would compress all four into surface-level work

Cycle-29 absorbed item 1 (NEW pattern integration) and re-populated
the 2-system tier with 4 new patterns. The cycle-28 deferral's
prediction was correct: the 2-system tier IS now re-populated;
restructuring would have been done against an empty tier in cycle 28
and would need redoing now in cycle 29 against a 4-pattern tier.

The deferral was procrastination if and only if the restructure
would have been *better* done in cycle 28 than cycle 30. The
empirical evidence post-cycle-29 says restructure is BETTER done
in cycle 30 (or later) because the population is now more stable.

**Verdict (b):** PASS. The deferral was principled sequencing; the
cycle-29 integration confirms the cycle-28 prediction was correct.

### Question (c): third-category refinement candidate — SPECULATIVE process-layering, defer-with-trigger is right call

Cycle-28's notes file proposes a third category to the discipline-
lightening rule: "substantive prose modifying load-bearing artifacts
→ full structured pass; bounded mechanical edits → 30-second self-
check; substantive prose in cycle-N notes (recording what happened,
not modifying load-bearing artifacts) → 30-second self-check applied
per-claim with named-citation requirements."

Cycle-29 reading: this is speculative process-layering. The cycle-25
codified rule has two categories that have been applied 7+ times
across cycles 25-28; no error has been caused by applying the rule
to cycle-N notes files. The borderline case (cycle-26's notes file
applied 30-second self-check to itself) was identified by cycle-28
as "borderline" but not as causing an error.

Adding a third category preemptively, without a trigger event,
would itself be the kind of process-layering the rule was meant to
avoid. The cycle-28 disposition ("Defer until a cycle-N notes error
occurs that would warrant the refinement; speculative codification
without the trigger is over-process") is the right call.

The risk if the third category is needed and not added: a cycle-N
notes file produces an error that 30-second self-check would have
caught at full structured pass. Mitigation: cycle-N+1 cold-readers
on cycle-N notes files (the standard pattern, applied this cycle)
catch errors at the next-cycle level. The cold-reader-on-notes
pattern is a sufficient backstop for the 30-second-self-check rate
on substantive-but-documentary content.

**Verdict (c):** PASS for cycle-28's deferral rationale; PASS for
the "wait for trigger" disposition. The third category is
speculative process-layering until empirical trigger occurs.

### Cold-reader on cycle-28 notes — overall verdict

3/3 questions PASS. Cycle-28's reasoning was sound on all three
named questions; cycle-29's empirical work (NEW pattern integration
using corrected counts; 2-system tier re-population) confirms the
cycle-28 predictions and dispositions.

## Persistence-mechanism observations

The cycle-N-pre-commits-cycle-N+1-checks chain extends to twenty-
four cycles deep (cycle 7 → ... → 28 → 29 → 30 pre-committed).
~10 items in cycle-30+ pre-commit list, smaller than cycle-29's
incoming 11 (cycle 29 absorbed items 1 and 11; defers items 2, 3,
4, 5, 6, 7, 8, 9, 10 with rationale). List shrinkage is genuine
for the second consecutive cycle — cycle-28 absorbed 4, cycle-29
absorbed 2 (but the cycle-29 absorption of item 1 was the largest
single substantive item in the list, so the substantive-weight
absorbed is high even though item-count absorbed is low).

**The substantive-weight metric.** Cycle-26 onward the pre-commit
list has been roughly stable in count (10-12 items), but the
substantive-weight composition has shifted: cycle-28 absorbed
lighter items (recheck, hedge tightening, count fix, cold-readers)
and deferred the heaviest item (Tier-2 restructure → renamed to
NEW pattern integration via cycle-29 sequencing). Cycle-29
absorbed the heaviest item (NEW pattern integration). The pattern
"defer the heavy substantive item one cycle to land it well-prepared
in the next cycle" worked here — cycle-28's recheck produced the
empirical input cycle-29 needed; without the recheck, cycle-29's
integration would have used the cycle-27 first-pass counts and
introduced the same overcounts and undercounts into `1-research.md`.

**The substantive-weight pattern as persistence mechanism.** This
sequencing — light prep cycle → heavy substantive cycle — is a
new persistence-mechanism observation. It works specifically when
the heavy substantive cycle DEPENDS on output from the light prep
cycle. Cycle-29 depended on cycle-28's recheck output; cycle-28's
recheck depended on cycle-27's matrix construction; cycle-27's
matrix construction depended on cycle-26's three deliverables. The
chain is genuinely sequential, not just historical accident.

**Honest-hedge tally extended: 8/8.** The honest-hedge pattern
(named cycle 24, 6/6 stable cycles 24-27, 7/7 cycle 28) applied
this cycle: the diversity-hedge clause in the mechanical-enforcement
bullet is genuine to the data (loose-framing extensions are at
different scope and rigidity, named honestly); the
plans-as-artifacts bullet's distinguishing-from-failed-work clause
is genuine to the data (forward-spec vs backward-history is a real
distinction); the iteration-ceilings bullet's "Cognition adjacent
at different bounding-axis" framing is genuine to the data
(time-limit vs iteration-count is a different axis); the
single-system framings for #6 and #9 preserve cycle-28's "1
explicit + 2 implicit/adjacent" verdict. Tally extended: 8/8 stable.

**Discipline-lightening rule applied.** Cycle 29's 4 new 2-system
bullets in `1-research.md` and 5 new single-system observations in
`_notes/cycle-22` are architecturally-load-bearing prose work (full
structured pass — drafted, cold-reader-checked for anti-smuggling /
transferability symmetry / self-introduced errors / section-
transitions, edits applied, verdict captured). The cold-reader on
cycle-28 notes is bounded mechanical (30-second self-check per
question — the questions were predefined, the verdicts
straightforward, no architectural changes generated). Tally
extended: substantive 8/8 (cycle-29 added 4 patterns + 5 observations,
all substantive); architecturally-load-bearing 2/2 (cycle-29 added
the integration as architecturally-load-bearing); bounded mechanical
1/7 (cycle-29 added one — the cold-reader on cycle-28 notes; the
prior 0/6 from cycle 28 carries forward).

**Cycle-30+ absorption-rate watch.** Cycle 30's incoming pre-commit
list is ~10 items (see cycle-30+ pre-commits below). The biggest
deferred substantive items are now (a) Tier-2 cross-system
observations restructure (item 2), and (b) Persistent Divergences
update (item 3), both architecturally-load-bearing. Cycle-30 SHOULD
make item 2 the focal — restructure decision is now ready (the
post-integration shape is visible per cycle-28's deferral rationale).

## Cycle 30+ pre-commits

Carry-forward + cycle-29-derived:

1. **Tier-2 cross-system observations restructure** (deferred from
   cycles 27-29). Now ready for cycle 30 focal: the post-integration
   shape is visible (12 patterns at 3+ tier + 4 patterns at 2-system
   tier + 10 patterns at single-system, including 5 cycle-29
   additions). Family-clustering vs maturity-clustering vs flat-
   with-ordering decision can be made against the stable population.
   Architecturally-load-bearing.

2. **Update Persistent Divergences section** (carry-forward from
   cycle 27-28 pre-commit 5). Cognition Devin's anti-stance on
   role-separation; the "throughput regime as moderating variable"
   observation from OpenAI Harness's wide-trust-boundary
   contradiction. Substantive prose work.

3. **Cross-validate against audit's A-pattern mapping** (carry-
   forward from cycle 25-28 pre-commit). Bounded mechanical (~one
   cold-reader cycle).

4. **Read remaining audit retrospective sections** (carry-forward
   from cycle 25-28 pre-commit). "What v2 must demonstrably do
   better" section is the most relevant for Phase 2.

5. **Copilot research-only dispatch: oh-my-claudecode** (Eva
   directive #2774). Deferred from cycles 26-28. Cycle-30+ may be
   the right time to dispatch — cycle-29's NEW pattern integration
   is complete; cycle-30 focal is restructure (orchestrator-direct
   work, doesn't conflict with Copilot research).

6. **Copilot research-only dispatch: openai/symphony** (Eva
   directive #2775). Same gating as item 5.

7. **Codify the SUPPORT/CONTRADICT gradient definition** (carry-
   forward from cycle 28 pre-commit 8). Bounded mechanical; defer
   until matrix shape is re-used. Cycle-29's per-pattern review
   used candidate-by-candidate verdicts (not matrix shape) — so
   the matrix codification trigger hasn't fired again.

8. **Codify the third-category refinement to discipline-lightening
   rule** (carry-forward from cycle 28 pre-commit 9). Defer until
   trigger event occurs. Cycle-29 confirms cycle-28's "wait for
   trigger" disposition is correct.

9. **Long-deferred items roll-call** (carry-forward, 9 items
   unchanged cycles 26-28; carry into cycle 30+).

10. **Same-cycle cold-reader on this notes file.** Standard cycle-
    N+1 fresh-eye pass. Specific questions:
    (a) Does the 5×4 placement-decision table read as principled
        rule-application or as ad-hoc per-pattern judgment?
    (b) Does the cold-reader-on-cycle-28-notes section's 3/3 PASS
        verdict feel earned, or does it read as too-easy validation
        (confirmation bias on prior-cycle reasoning)?
    (c) Are the two flagged borderline framings (entropy/AI-slop
        "inversely-related"; throughput "moderating variable")
        genuinely worth a fresh-eye second pass, or is the
        flagging itself process-layering?

### Suggested cycle 30 plan (provisional)

- **Focal:** item 1 (Tier-2 cross-system observations restructure).
  Architecturally-load-bearing prose work; cycle-30 absorbs the
  largest deferred substantive item (now ready post-cycle-29
  integration).
- **Bounded mechanical:** item 10 (cold-reader on this notes file).
- **Defer:** items 2-9 to cycle 31+ depending on focal completion
  progress.
- **Possible Copilot dispatch:** items 5/6 (oh-my-claudecode and/or
  openai/symphony) IF cycle-30's focal completes early. Otherwise
  cycle 31+.
