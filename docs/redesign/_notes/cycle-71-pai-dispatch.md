# Cycle 71 — PAI deeper-read dispatch + stuck-dispatch self-healing finding

**Date:** 2026-05-05
**Substantive focal activity:** option 2 from cycle 70 hand-off (PAI deeper-read dispatch construction parallel to cycle-63 oh-my-codex pattern). Plus a self-healing finding on the stuck cycle-63 dispatch [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833).
**Cycle-70 provisional read carried forward:** PAI deeper-read dispatch construction (option 2) — selected because [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) hadn't returned (option 1 not in play); cycle 70 also flagged option 3 (second-arc synthesis on cross-cluster intersections) as fallback.

## What this document is

This is the cycle-71 process document — what was dispatched, why this scope and not another, what was discovered about the stuck cycle-63 dispatch, and what cycle 72+ should pick up. It is NOT the deliverable itself (that lands at `docs/redesign/_notes/cycle-71-pai-deeper-read.md` when the dispatched Copilot session returns).

## Cycle composition under #2829's polarity inversion

Cycle 71 is the TENTH consecutive cycle of research-corpus advancement under [#2829](https://github.com/EvaLok/schema-org-json-ld/issues/2829)'s polarity inversion (cycles 62-71). Polarity-pivot continuation discipline now spans:

- Cycle 62: option 4 (implications mining on AutoGen)
- Cycle 63: option 1 (deeper-read dispatch on oh-my-codex)
- Cycle 64: option 4 (implications mining on LangGraph) + bounded-mechanical stale-reference cleanup
- Cycle 65: option 2 (cross-implications synthesis)
- Cycle 66: option 4 (implications mining on Cognition Devin) + mining-with-synthesis-update first instance
- Cycle 67: option 4 (implications mining on openclaw) + mining-with-synthesis-update second instance
- Cycle 68: option 4 (implications mining on OpenAI harness) + mining-with-synthesis-update third instance
- Cycle 69: option 4 (implications mining on Voyager — last unique deep-dive system) + mining-with-synthesis-update fourth instance
- Cycle 70: option 2 (synthesis cycle producing elevation drafts in `1-research.md` — second synthesis instance)
- Cycle 71: option 1 (PAI deeper-read dispatch construction — second dispatch-construction instance) + stuck-dispatch self-healing finding

Seven distinct cycle composition shapes now demonstrated under #2829's polarity inversion: mining (6 instances), dispatch-construction (2 instances), synthesis (2 instances), framework-iteration cold-reader fallback (older cycles), bounded-mechanical (older cycles), mining-with-synthesis-update (4 instances), per-finding-evaluation (older cycles). Cycle 71 hardens dispatch-construction as a recurring shape (not a one-off) and demonstrates the self-healing-finding-as-cycle-byproduct pattern.

Cold-reader cadence on `2-design-framework.md` is bounded-mechanical fallback only when no substantive option is viable. Today both #2829 substantive options were viable (option 1 PAI deeper-read dispatch was specifically named-equivalent in #2829's "if option 1 already in flight, dispatch the next stub-system" interpretation; option 2 synthesis was viable as alternative). Cold-reader was not in play.

## What the dispatch targets

PAI is currently first-pass-README only per cycle-14 orchestrator-direct read. The cycle-14 read explicitly deferred four areas to "later cycles":

- `Tools/` directory — the deterministic-substrate side of PAI's "Code Before Prompts" principle
- `Packs/` directory — the modularity boundary for capability packaging
- `.claude/` directory — Claude Code integration surface (skills, role prompts, hooks)
- `Releases/v4.0.3/` — versioned release structure

These four areas are exactly the most relevant to the v2 redesign's CORE-DESIGN-PRINCIPLE. PAI's Principle 11 ("Goal → Code → CLI → Prompts → Agents") is the closest cross-system match to the redesign-prompt's principle-test ("can a tool do this deterministically? If yes, build the tool"). Code-level reading should reveal whether PAI's claim is implementation-backed or marketing-surface.

The dispatch also embeds three pre-set hypotheses to be evaluated by the deeper read:

- **H1 (cluster J emergence test):** PAI's Memory System (Principle 13) implements semantic-retrieval as a top-level architectural concern. If H1 confirmed, cluster J emerges as standalone cluster (semantic-retrieval architecture, distinct from cluster B storage stratification). If H1 refuted, cluster B sub-shape assignment for Voyager's I-V6 stands. **This is the dispatch's headline question.**
- **H2 (cluster H augmentation test):** PAI's Learn → Improve closure is implemented in code as a post-session feedback loop. If H2 confirmed, cluster H upgrades 4-system → 5-system convergent with new sub-shape; if H2 refuted, cluster H stays at 4-system.
- **H3 (cluster A code-construct test):** PAI's Foundational Algorithm (Observe→Think→Plan→Build→Execute→Verify→Learn) is implemented as a code construct (state machine, super-step boundaries) rather than only as a prompt-stated framing. If H3 confirmed, cluster A gains new sub-shape; if H3 refuted, the algorithm is documentation-honesty material possibly cluster D.

The hypothesis-driven structure parallels cycles 66-69's hypothesis discipline (cumulative hit rate at cluster level: 11/12 = 92%). Cycle 71 introduces hypothesis-discipline to dispatch construction itself — the dispatch instructions explicitly require Copilot to evaluate H1/H2/H3 with code citations, producing a deliverable that's both a deeper-read AND a hypothesis-evaluation artifact.

Dispatch issue body lives at [#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842); it follows the deeper-read template established by cycle-63's [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) — Why-cycle-N-and-not-earlier preamble, What's-new-since cross-references to current redesign artifacts, lens-numbered structure for Copilot output mirroring, anti-smuggling discipline pre-loaded.

## Why a Copilot dispatch and not orchestrator-direct

Same reasoning as cycle 63:

- **Time efficiency.** A 75-minute orchestrator session can read perhaps 5-15 files in PAI with citation-grade attention; a Copilot dispatch can read substantially more in parallel because the orchestrator session can spend its time on dispatch construction + complementary work.
- **Context windowing.** PAI's repo has substantial directory structure (Tools/, Packs/, .claude/, Releases/v4.0.3/, plus README and likely multiple architecture docs); reading enough for citation-grade verification of 16 principles exceeds what an orchestrator session can hold in working context with quality.
- **Citation quality.** Copilot dispatches return file:line citations for each claim; orchestrator-direct reads are more vulnerable to summary-shape tradeoffs.
- **Precedent.** Cycle-42 openclaw deeper read used Copilot dispatch ([#2808](https://github.com/EvaLok/schema-org-json-ld/issues/2808) → 893 lines); cycle-41 cognition deeper read used Copilot dispatch ([#2802](https://github.com/EvaLok/schema-org-json-ld/issues/2802) → 795 lines); cycle-41 OpenAI harness deeper read used Copilot dispatch ([#2803](https://github.com/EvaLok/schema-org-json-ld/issues/2803) → 780 lines). Cycle 71 PAI follows the same pattern.

## Cycle composition decision: dispatch-plus-diagnosis substantive focal

Cycle 70's recommendation: "PAI deeper-read dispatch construction parallel to cycle 63 oh-my-codex pattern." Dispatch construction took ~30 minutes of orchestrator session time (orientation reads, gap analysis on cycle-14 stub, drafting against #2833 template with PAI-specific calibration, opening the issue). The complementary work that emerged organically — the stuck-dispatch diagnosis on #2833 — was triggered by the pre-dispatch state-check on the Copilot agent-task assignment pattern. Cycle 71 is therefore tagged **dispatch-plus-diagnosis substantive focal**.

The diagnosis is materially significant (see next section). Without the diagnosis, cycle 71 would have been a cycle-63 parallel; with the diagnosis, cycle 71 is the cycle that fixed cycle-63's bug and produced v2 design-input from the failure.

## Stuck-dispatch self-healing finding

### The diagnosis

Pre-dispatch state-check on the Copilot agent-task assignment pattern surfaced the root cause of #2833's 8-cycle stuckness: cycle-63's dispatch was filed without `Copilot` as an assignee.

Comparison data from `gh issue list --label agent-task --state all`:

| Issue | Created | State | Assignees |
|-------|---------|-------|-----------|
| #2842 (PAI cycle 71, this cycle) | 2026-05-05 | OPEN | [EvaLok] |
| **#2833 (oh-my-codex cycle 63)** | **2026-05-04** | **OPEN** | **[]** |
| #2808 (openclaw cycle 42) | 2026-05-01 | CLOSED | [EvaLok, Copilot] |
| #2803 (OpenAI harness cycle 38 re-dispatch) | 2026-05-01 | CLOSED | [EvaLok, Copilot] |
| #2802 (Cognition Devin cycle 38 re-dispatch) | 2026-05-01 | CLOSED | [EvaLok, Copilot] |
| #2790 (cycle-30 critique re-dispatch) | 2026-04-30 | CLOSED | [EvaLok, Copilot] |
| #2789 (cycle-30 critique) | 2026-04-30 | CLOSED | [] (closed without processing) |
| #2782 (oh-my-codex cycle 26) | 2026-04-29 | CLOSED | [EvaLok, Copilot] |
| #2781 (OpenAI harness cycle 26) | 2026-04-29 | CLOSED | [EvaLok, Copilot] |
| #2779 (Cognition Devin cycle 26) | 2026-04-29 | CLOSED | [EvaLok, Copilot] |
| #2767 (LangGraph cycle 18) | 2026-04-29 | CLOSED | [EvaLok, Copilot] |
| #2762 (AutoGen cycle 15) | 2026-04-28 | CLOSED | [EvaLok, Copilot] |
| #2755 (Phase-0 critique cycle 11) | 2026-04-28 | CLOSED | [EvaLok, Copilot] |
| #2748 (Phase-0 critique cycle 6) | 2026-04-27 | CLOSED | [EvaLok, Copilot] |

Pattern: every closed-and-processed agent-task dispatch had BOTH `EvaLok` AND `Copilot` as assignees. The two outliers (#2833 cycle-63 oh-my-codex, #2789 cycle-30 critique re-dispatch original) had empty assignees — and #2789 was closed without processing (the cycle-30 re-dispatch landed via #2790 which had proper assignees). #2833 stuck for 8 cycles is the active failure.

### Self-fix attempt

`gh issue edit 2833 --add-assignee Copilot` failed with:

```
GraphQL: Bot does not have access to the repository. (replaceActorsForAssignable)
failed to update 1 issue
```

The orchestrator-bot lacks GitHub Apps permission to assign the Copilot bot via GraphQL. The error message is ambiguous — it might refer to the orchestrator-bot's permissions or to the Copilot bot's permissions — but the operational effect is the same: the orchestrator can't self-fix #2833's malformation.

The orchestrator successfully added EvaLok as assignee on #2833 via `gh issue edit 2833 --add-assignee EvaLok`. So the orchestrator's permission is partial: it can assign humans (Eva) but not the Copilot bot.

### Mitigation

Cycle 71 took two steps:

1. **Added EvaLok as assignee on #2833** (the orchestrator's permission allows this). #2833's assignees field is now `[EvaLok]` instead of empty. This is a partial fix — Copilot is still missing — but it makes the dispatch slightly less malformed and gives Eva a routing signal that the issue needs her attention.

2. **Posted diagnosis comments on both #2833 and #2842** requesting Eva manually assign Copilot. The comments explain the diagnosis, the failed self-fix attempt, and the request for manual intervention. Eva can act on the request via standard GitHub UI when she next sees the journal or the cycle issue summary.

### v2 design-input from the finding

The dispatch primitive should be tool-mediated rather than orchestrator-direct `gh issue create` invocations:

- **Concrete tool shape:** A `dispatch-research` Rust tool taking `(target-system, dispatch-body-path, deliverable-path)` and producing a properly-assigned issue with mechanical guarantees on the assignment field. No code path where the assignment step gets missed by the orchestrator's prompt construction. The tool would also enforce other invariants (research-only label, agent-task label, deliverable-path naming convention, body structure validation).

- **Boot-discipline analogue:** Sync invariant on dispatch creation that requires Copilot assignee — analogous to Voyager I-V4 (sync invariants asserted at init for dual-storage components, fail-fast at boot with remediation hint). For dispatch creation: assert that Copilot is in assignees before allowing the issue to be filed; if assertion fails (because permission is denied), fail-loud with a remediation hint ("orchestrator-bot lacks Copilot-assignment permission; ask Eva to assign manually OR update repository permissions").

- **Stuck-watchdog discipline:** A `dispatch-watchdog` Rust tool reading in-flight dispatches + detecting stuck via heuristics + producing diagnosis comment + lane-release notification. This is openclaw I-O5 (stuck-session-watchdog) instantiated for the dispatch primitive. v2 candidate authors should consider this as a first-class lifecycle operation alongside terminate/reset/resume/fork/event-trigger.

- **Distinguishing stuck-malformed vs stuck-slow:** Cycle 71's diagnosis fired earlier than cycle 70's watchdog threshold (cycle 73) because the dispatch wasn't slow, it was malformed. v2 design-input: stuck-session-watchdog primitives should distinguish *why* a session is stuck before deciding the response shape. A truly-slow session may need patience; a malformed session needs immediate corrective action. The openclaw I-O5 pattern conflated both into "stuck-warn"; cycle 71's experience suggests a richer state — `stuck-malformed` (corrective action) vs `stuck-slow` (patience or escalation) — with diagnostic discipline to distinguish them.

The cluster sub-shape this maps to most directly is cluster A's sync-invariants sub-shape (cycle 69 Voyager I-V4 contribution); the diagnosis comment IS cluster I (harness-enforced security/policy boundaries) instantiated at the dispatch primitive layer; the v2 design-input is the assignment-invariant-at-dispatch-creation sub-shape that cycle 71's experience produces.

## What the deliverable should look like

Target: `docs/redesign/_notes/cycle-71-pai-deeper-read.md`, 600-1200 lines, 9 lens sections matching the dispatch instructions:

1. Memory System architecture — code-level (H1 cluster J emergence test)
2. Foundational Algorithm — code construct or prompt framing (H3 cluster A test)
3. Learn → Improve closure (H2 cluster H augmentation test)
4. Skill Management + Agent Personalities — stratification axes (cluster F augmentation candidate)
5. Decision Hierarchy (Principle 11) enforcement — code construct or advisory
6. Tools/ and Packs/ — capability substrate
7. `.claude/` — Claude Code integration surface
8. Spec/Test/Evals + Permission to Fail — documentation-honesty surfaces (cluster D test)
9. Anchoring caveats

The dispatch deliverable lands when the Copilot session completes (assuming Eva manually assigns Copilot to unblock the dispatch). Per prior cadence (cycle-41 cognition + OpenAI harness landings, cycle-42 openclaw landing), processing time is 1-2 cycles after assignment. If Eva fixes assignment today (2026-05-05), oh-my-codex deeper-read [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) should land cycle 72 or 73; PAI deeper-read [#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842) should land cycle 73 or 74.

## Cycle 72 provisional read

Three candidates in priority order:

1. **If either dispatch returns by cycle 72:** per-finding evaluation absorption (highest priority — value compounds when integrated rapidly; both have substantial pre-set hypothesis structure ready to evaluate). #2833 likely returns first (filed earlier; just-needs-Copilot-assignment); #2842 follows.
2. **Otherwise, second-arc synthesis cycle on cross-cluster intersections:** cycle 70 hand-off named cluster A↔B (storage-discipline at boundaries), F↔H (stratification of feedback mechanisms), D↔I (documentation-as-policy-enforcement). Cross-cluster intersections are the next layer of research-corpus advancement after within-cluster sub-shape catalogues.
3. **Otherwise, oh-my-claudecode dispatch construction (per [#2774](https://github.com/EvaLok/schema-org-json-ld/issues/2774)) or Symphony dispatch construction (per [#2775](https://github.com/EvaLok/schema-org-json-ld/issues/2775)):** continues the dispatch-construction cycle composition shape; both are named in input-from-eva directives but not yet dispatched.

Strong recommendation: option 1 IF Eva's manual Copilot-assignment fix lands before cycle 72 starts. Otherwise option 2 (second-arc synthesis on cross-cluster intersections).

## Cleanup-bounded state at cycle-71 close

- 2 open dispatches: [#2833](https://github.com/EvaLok/schema-org-json-ld/issues/2833) (oh-my-codex, EvaLok now assigned, awaits Copilot manual assignment), [#2842](https://github.com/EvaLok/schema-org-json-ld/issues/2842) (PAI, EvaLok assigned, awaits Copilot manual assignment)
- 1 open PR: [#2830](https://github.com/EvaLok/schema-org-json-ld/pull/2830) (structural absorption of #2829, standing)
- 7 standing input-from-eva directives (none new since cycle 62; #2829 + #2794 + #2775 + #2774 + #2759 + #2741 + #808)

State slightly grew (+1 dispatch); both dispatches now need Eva action to unblock. Eva-action queue is now non-trivial: assign Copilot to two open dispatches. This is a different posture from cycle 70 (Eva-action queue was empty).
