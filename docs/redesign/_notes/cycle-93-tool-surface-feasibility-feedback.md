# Cycle 93 feedback — Phase 2 candidate tool-surface feasibility critique

I read the 3 candidate docs plus the comparison README. This critique is intentionally adversarial and focused on Rust crate scope realism, not framework aesthetics.

Calibration note: current workspace crate sizes are not “tiny by default” (local scan on `tools/rust/crates`: 38 crates, median ~1081 LOC, mean ~2116 LOC; only a minority are <500 LOC).

## Lens 1 — Per-crate scope realism (Candidate A)

Source claims: 9 new crates, each “~200-500 LOC,” aggregate “~3000-4500,” bounded single-cycle per crate ([A:L68-L90](../2-candidates/A-evolved-single-orchestrator.md), [README:L48-L51](../2-candidates/README.md)).

1. **`boot-phase` + `close-phase` are under-estimated** — **[Under-estimate]**  
   They are described as orchestration hubs touching state load, cursor advance, directive checks, journal/issue writes, and push semantics ([A:L68-L70](../2-candidates/A-evolved-single-orchestrator.md), [A:L142-L173](../2-candidates/A-evolved-single-orchestrator.md)). With robust error paths + tests, 200-500 LOC each is optimistic.

2. **`phase-transition-check` is probably over-estimated at upper bound** — **[Over-estimate]**  
   If this is truly just transition validation for 3 modes, this can be compact. 200-500 is plausible, but likely near lower end unless policy explodes ([A:L70](../2-candidates/A-evolved-single-orchestrator.md)).

3. **`wiki-search` is materially under-estimated** — **[Under-estimate]**  
   “Top-k retrieval over `_notes/*.md` description-frontmatter” sounds simple in prose, but practical load includes indexing, update strategy, ranking semantics, corruption handling, and query contract ([A:L12](../2-candidates/A-evolved-single-orchestrator.md), [A:L71](../2-candidates/A-evolved-single-orchestrator.md)). This is one of A’s likely >500 LOC crates.

4. **`tool-registry`/`cycle-history-append` are likely over-estimated** — **[Over-estimate]**  
   Both are narrow by contract (enumeration + append-only write) ([A:L72](../2-candidates/A-evolved-single-orchestrator.md), [A:L76](../2-candidates/A-evolved-single-orchestrator.md)). 200-500 LOC is plausible but likely below midpoint unless requirements creep.

5. **Boundary issue: orchestration split may be too fine** — **[Ambiguous-without-prototype]**  
   `boot-phase`, `close-phase`, and `phase-transition-check` might be over-decomposed. You may pay more integration/test glue than logic saved. Merge pressure is real unless there is strict ownership isolation.

## Lens 2 — Per-crate scope realism (Candidate B)

Source claims: ~12+ Rust crates + ~20-40 skills, 10k-20k LOC ([B:L67-L99](../2-candidates/B-decomposed-multi-role.md), [README:L48-L50](../2-candidates/README.md)).

1. **Skill abstraction likely creates artificial decomposition pressure** — **[Under-estimate]**  
   “20-40 skill crates” plus role-bound manifests/contracts multiplies lifecycle overhead even before business logic ([B:L15](../2-candidates/B-decomposed-multi-role.md), [B:L83](../2-candidates/B-decomposed-multi-role.md)).

2. **Typed-channel infra is likely the dominant LOC sink** — **[Under-estimate]**  
   `channel-router` + reducer schemas + boundary sync + coordination invariants are not cheap glue ([B:L11](../2-candidates/B-decomposed-multi-role.md), [B:L31-L33](../2-candidates/B-decomposed-multi-role.md), [B:L220-L229](../2-candidates/B-decomposed-multi-role.md)). Risk: infra LOC exceeds role logic LOC.

3. **Branching checkpoints are almost certainly >“small crate” complexity** — **[Under-estimate]**  
   `branch-manager` implies fork/promotion/gardening semantics ([B:L13](../2-candidates/B-decomposed-multi-role.md), [B:L74](../2-candidates/B-decomposed-multi-role.md), [B:L64](../2-candidates/B-decomposed-multi-role.md)). Even in-tree, this is policy-heavy and test-heavy.

4. **B’s own text already contradicts “small crates” optimism** — **[Estimate stable (high)]**  
   It says “several thousand LOC each” for key infra ([B:L62](../2-candidates/B-decomposed-multi-role.md)). That aligns with a high-cost reality.

## Lens 3 — Per-crate scope realism (Candidate C)

Source claims: ~11 crates, each ~200-500 LOC, total 4k-6k ([C:L100-L106](../2-candidates/C-hybrid.md)).

1. **`reconcile-mode` likely under-estimated** — **[Under-estimate]**  
   Contract includes multi-channel inbound handling (Eva, audit, dispatch merges) + typed delta emission ([C:L80](../2-candidates/C-hybrid.md), [C:L22](../2-candidates/C-hybrid.md)). That is coordination logic, not just a phase wrapper.

2. **Plan lifecycle pair (`plan-lifecycle` + `plan-lifecycle-check`) is under-estimated in integration cost** — **[Under-estimate]**  
   State machine + CI invariants + promotion logic + stale detection ([C:L16](../2-candidates/C-hybrid.md), [C:L86-L87](../2-candidates/C-hybrid.md), [C:L200-L201](../2-candidates/C-hybrid.md)) usually means non-trivial test matrix.

3. **Incremental delta over A is likely low-balled** — **[Under-estimate]**  
   C claims +~1000-1500 LOC over A for reconcile + lifecycle additions ([README:L48-L50](../2-candidates/README.md), [C:L105](../2-candidates/C-hybrid.md)). Given cross-mode glue + CI additions, delta likely drifts higher.

## Lens 4 — Aggregate net-add LOC realism

| Candidate | Doc range | My read |
|---|---:|---:|
| A | 3000-4500 | **3600-6200** |
| C | 4000-6000 | **5200-8200** |
| B | 10000-20000 | **14000-28000** |

Why: integration/test/CI/shared-type overhead is under-counted in all three docs; C and B suffer more from coordination glue than acknowledged.

### Probability that actual net-add exceeds the candidate upper bound

| Candidate | >+25% over upper bound | >+50% | >+100% |
|---|---:|---:|---:|
| A (>5625) | 35% | 15% | 5% |
| C (>7500) | 55% | 30% | 10% |
| B (>25000) | 45% | 25% | 8% |

The sharpest weakness is **C’s confidence posture**: it reads “near A” but has coordination complexity closer to mid-zone between A and B.

## Lens 5 — Cutover scope predictability

1. **A’s “1 crate per cycle” is optimistic for all 9 crates** — **[Under-estimate]**  
   Per crate you still need authoring + tests + harness integration + workflow touch + docs + fallout fixes ([README:L50-L51](../2-candidates/README.md), [A:L89-L91](../2-candidates/A-evolved-single-orchestrator.md)). Realistic pace is mixed: some 1-cycle, some 2-cycle.

2. **C has same optimism problem, worse due to cross-mode coupling** — **[Under-estimate]**  
   C says 5-10 cycles for 11 crates ([C:L110](../2-candidates/C-hybrid.md)). That lower bound assumes very low rework on reconcile/lifecycle coupling.

3. **B’s “multi-cycle build-out” is too vague to be useful** — **[Under-estimate]**  
   For ~32-52 crates total surface ([README:L48-L50](../2-candidates/README.md), [B:L83](../2-candidates/B-decomposed-multi-role.md)), honest planning range is likely **16-30 cycles** if quality is non-negotiable.

## Lens 6 — F-pattern structural coverage vs tool count

1. **A: F1 coverage depends too much on one CI crate** — **[Under-estimate]**  
   A leans on `prompt-contract-check` for accretion control ([A:L35](../2-candidates/A-evolved-single-orchestrator.md), [A:L73](../2-candidates/A-evolved-single-orchestrator.md)). In practice this needs ongoing rule expansion; one static checker will lag prompt evolution.

2. **B: added crates are not pure coverage; much is coordination tax** — **[Estimate stable]**  
   B does add true structural depth for some patterns, but a large fraction of count is channel/role plumbing ([B:L21](../2-candidates/B-decomposed-multi-role.md), [B:L220-L229](../2-candidates/B-decomposed-multi-role.md)).

3. **C: one `reconcile-mode` crate is probably insufficient abstraction for F2+F11 long-term** — **[Under-estimate]**  
   Current contract spans distinct inbound classes with different failure semantics ([C:L80](../2-candidates/C-hybrid.md), [C:L207-L219](../2-candidates/C-hybrid.md)). Expect split pressure or internal submodule growth.

## Lens 7 — Per-candidate tool-count growth risk

1. **Growth is not linear; coordination grows super-linearly once tool count rises** — **[Under-estimate]**  
   A already flags registry-growth risk ([A:L222-L223](../2-candidates/A-evolved-single-orchestrator.md)); the miss is magnitude. Tool-to-tool contract checks and discovery friction compound.

2. **A’s registry probably scales to ~15 tools, then becomes load-bearing overhead** — **[Estimate stable-with-threshold]**  
   Past that, one-line descriptions stop being enough for safe invocation selection.

3. **B’s skill abstraction likely worsens discovery overhead after early gains** — **[Under-estimate]**  
   Skills help local encapsulation, but 20-40 skills across roles means lookup/routing costs explode without strong taxonomy ([B:L15](../2-candidates/B-decomposed-multi-role.md), [B:L83](../2-candidates/B-decomposed-multi-role.md)).

4. **Candidate-independent threshold**  
   Likely load-bearing threshold is around **18-25 callable units** (tools + skills) unless there is strict hierarchical registry + compatibility metadata.

## Net assessment shifts (relative to candidate docs)

- **A:** Still the most believable migration story, but likely underestimates 2-3 crates (`wiki-search`, `boot-phase`, `close-phase`). Migration risk is **medium**, not “high predictability” by default.
- **C:** Most under-justified estimate posture. Claimed incremental cost over A is likely low. Migration risk should move from “medium-high predictability” toward **medium**.
- **B:** Correctly positioned as expensive, but range is still probably optimistic at the top once full coordination/test burden is counted.

If selecting on tool-surface feasibility alone: **A > C >> B** still holds, but **A and C are closer in risk than the docs suggest**, and C is the one most likely to miss its own upper-bound estimate.
