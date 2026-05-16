# Dispatch-brief discipline addendum

**Status:** canonical record of the AGREE-ACT-NOW action from
[`cycle-148-two-track-absorption-and-landing.md`](cycle-148-two-track-absorption-and-landing.md) L2.4
(PR #2877 adversarial critique, dispatched cycle 146, returned cycle 147,
absorbed cycle 148). Adopted into authoring practice as of cycle 148;
recorded here cycle 162 (cycle 148 absorption named this as
"AGREE-ACT-NOW finding #7" in [`cycle-149-two-track-composition.md`](cycle-149-two-track-composition.md) §3
priority list).

**Authoring cycle:** 162 (2026-05-16). Closes cycle 161 forward priority #2 /
cycle 160 forward priority #2 / cycle 159 forward priority #2 / cycle 155
absorption residue (was AGREE-ACT-NOW finding #7 carried as a "do this for
future dispatches" reminder across cycles 148-161 without a canonical home).

## 1. The discipline

When authoring a **role prompt, multi-instance prompt set, or any dispatch
brief that will produce multiple sibling prompts**, the brief MUST NOT
instruct the producer to mirror a template's section structure literally
across siblings. Instead:

> **Preserve contract-equivalent sections (role identity, inputs, output
> contract, constraints, session structure), but rename/repartition sections
> so tag names match role semantics exactly. Prefer role-native structure
> over literal template parity. If a planner section is inapplicable, drop
> or replace it and justify in meta.**

— originally proposed by the PR #2877 critique L2.4 wording; adopted verbatim
into the addendum.

## 2. Why

The cycle 145 four-role-prompt authoring used a literal-mirror template
(planner.xml as template; reconciler / executor / curator imitate planner's
exact section structure). The cycle 147 dispatched adversarial critique
(L2.3 / L2.4) named this as a **design-level error**: mirror-by-tag-name
constrains downstream prompts to inherit semantic baggage from the template
even when the role's job is structurally different.

Two specific failure modes observed by L2.4 critique:

- **Reconciler dispatch brief said "mirror planner."** The reconciler's job
  (inbound-event reconciliation, not plan-emission) doesn't have natural
  parallels to most planner sections. Mirror-pressure caused the reconciler
  to either ship empty mirror-sections or stretch reconciler-content into
  planner-shaped containers. Both reduce signal.
- **Curator dispatch brief said "mirror planner."** Curator's 699 LOC ballooned
  beyond what a role-native structure would have required (L2.2 estimated
  400-500 LOC) because mirroring fixed the section anatomy even when curator
  semantics demanded different decomposition.

Pattern name (recorded cycle 145 NOVEL@1, hardened cycle 148):
**`dispatch-brief-template-mirror-tradeoff`**. Literal-mirror IS structurally
inferior to structural-reference-with-rename-freedom for cross-role prompts.

## 3. What "contract-equivalent" preserves

The discipline preserves contract surfaces — the parts a downstream consumer
(channel-router, validation tool, integration test) actually reads — not
section anatomy.

| Contract surface | Why preserved | Rename rule |
|---|---|---|
| Role identity (`<role>...</role>` or equivalent) | Routing requires unambiguous role label | Keep tag name |
| Inputs (channels read, files consumed) | Reconciler-feed contract | Tag may be renamed if role-native term is clearer (e.g., curator's `<consolidated-from>` instead of `<inputs>`) |
| Output contract (required keys, types) | Channel-router validation | Required keys MUST match channel schema; tag wrapping the keys can be role-native |
| Constraints (forbidden actions, scope guards) | Safety-relevant | Content preserved verbatim; tag name can be role-native |
| Session structure (start / work / end markers) | Cycle-runner step semantics | Tag may be renamed; sequence must remain detectable by the runner |

Anything outside this list is role-native. Tag names, section ordering,
sub-section nesting, justification prose, examples — all may diverge across
roles.

## 4. What "structural reference" looks like

The dispatch brief points to a template by reference, not by tag-equivalence:

> "Author this role prompt to satisfy the v2 prompt contract (channel-router
> required-keys for the role's output channel; session-start / session-end
> markers; preserved-primitive references). Use the planner prompt
> ([`prompts/v2/planner.xml`](prompts/v2/planner.xml)) as a structural
> reference for the *kinds* of sections a v2 role prompt contains, but
> rename / repartition sections to match this role's semantics. Do not
> mirror tag names where they would be wrong for this role."

vs. the cycle 145 literal-mirror brief which said (paraphrased): "Author
reconciler.xml mirroring planner.xml's section structure exactly; deviations
require justification."

## 5. Anti-patterns

### 5.1 Tag-name reuse where content is different

If the planner has `<planning-loop>` and the reconciler has a section
performing reconciliation-loop, the reconciler tag should be
`<reconciliation-loop>` (role-native), NOT `<planning-loop>` (template-mirror).

### 5.2 Empty mirror-sections

If a role has no natural content for a planner section, the dispatch must
NOT require an empty placeholder. The role-native structure should omit
that section entirely, with a brief justification in `<meta>`.

### 5.3 Stretching role-content into wrong containers

If a role's content naturally decomposes into 4 sections but the template
has 6, do not pad to 6. Use 4 sections with role-native names; do not
contort.

## 6. When this applies

- **Role-prompt dispatch briefs** (v2 reconciler / planner / executor /
  curator; future roles).
- **Multi-instance prompt dispatches** producing sibling prompts that share
  a contract but differ in role.
- **Any dispatch brief that names a "template" the producer should follow.**

Does NOT apply to:

- Single-instance prompt authoring (no siblings, no mirror pressure).
- Pure contract-validation work (the channel-router required-keys check is
  contract-level, not section-anatomy-level).
- v1 prompt iteration (frozen reference).

## 7. L2.5 paired forward-pointer (not closed by this addendum)

The cycle 148 absorption noted L2.5 as TOOL-SCOPED + AGREE-RECORD:

> If literal-mirror IS retained for some future dispatch (e.g., a
> contract-strict dispatch where tag names ARE part of the contract), the
> dispatch MUST impose anti-drift discipline via a section-tag semantic
> fidelity check.

This is `v2-prompt-tag-semantic-fidelity` tool design scope. Carried as a
TOOL-SCOPED forward priority (cycle 158-161 lists, currently cycle 161
forward priority #8). NOT closed by this addendum — this addendum closes
ONLY the L2.4 ACT-NOW discipline.

## 8. What this addendum does NOT do

1. Does NOT modify any existing v2 role prompt (the cycle 145 prompts are
   frozen reference; the discipline applies to FUTURE dispatches, not
   retroactive re-authoring).
2. Does NOT impose a tool-side check on dispatch briefs (that's L2.5 /
   `v2-prompt-tag-semantic-fidelity` scope).
3. Does NOT close L2.2 (curator size — AGREE-DEFER pending post-measurement
   curator-iteration cycle).
4. Does NOT close L2.3 (template-mirror as wrong organizing principle —
   AGREE-RECORD, recorded as Phase 2/3 candidate-selection-level design
   input, deferred to post-runtime-measurement prompt iteration).
5. Does NOT update [`2-design-framework.md`](../2-design-framework.md) — the
   framework iteration history is frozen; this addendum is a separate
   working note.
6. Does NOT add forward priorities (the AGREE-ACT-NOW residue is closed by
   recording the discipline canonically; future dispatches consult this
   file).

## 9. Cross-references

- Source critique: PR #2877 L2.4 (dispatched cycle 146, returned cycle 147,
  absorbed cycle 148).
- Absorption record:
  [`cycle-148-two-track-absorption-and-landing.md`](cycle-148-two-track-absorption-and-landing.md)
  §L2.4 (line 76-80).
- Forward priority list source:
  [`cycle-149-two-track-composition.md`](cycle-149-two-track-composition.md) §3
  priority #5 (AGREE-ACT-NOW finding #7).
- Recurring forward-priority entries: cycles 155 (#6), 156-161 (#2 each
  cycle's renumbered list).
- Pattern recurrence: `dispatch-brief-template-mirror-tradeoff` NOVEL@1
  cycle 145, HARDENED cycle 148 by L2.3/L2.4 critique.
- Paired residual: L2.5 (tool-scoped, `v2-prompt-tag-semantic-fidelity`,
  carried as cycle 161 priority #8).
