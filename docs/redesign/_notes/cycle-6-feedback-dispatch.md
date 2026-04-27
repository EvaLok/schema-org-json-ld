# Cycle 6 — Copilot feedback-only dispatch procedure (and execution log)

**Cycle:** redesign cycle 6 (2026-04-27)
**Sources:** `.github/workflows/orchestrator-redesign-prompt.xml` §AUTHORITY/COPILOT-DISPATCHES + §COPILOT-AS-FEEDBACK-PEER, cycle 5 open thread, `tools/rust/crates/dispatch-task/src/main.rs` (v1 reference)
**Purpose:** First feedback-only Copilot dispatch in redesign mode. Document the procedure used so future cycles can re-execute without re-deriving it. Also: log the cycle 6 dispatch (#2748) for reconciliation when Copilot's PR lands.

## Context

Cycles 3, 4, and 5 each named a Copilot feedback-only dispatch and each deferred. Cycle 5's deferral closed with an explicit binding: cycle 6 either builds the proper feedback-only dispatch mechanism or documents a manual procedure with safety framing and executes it. Cycle 6 chose the second path. Reasoning:

1. **The v1 `dispatch-task` tool is in the frozen-reference zone.** Per the redesign prompt's `<direct-push-zones>`, current production tools are forbidden direct-push targets. Adding a `--feedback-only` flag would require a workflow-change PR — moderate friction, and adds redesign-flavored behavior to a tool we are explicitly deprecating.

2. **Building a fresh `v2-dispatch-feedback` Rust tool now would be premature.** The v2 tool surface is a Phase 2/3 concern; designing one tool ahead of the surface design risks shaping later decisions around early scaffolding. Per `<core-design-principle>`, tools should match the system being designed, not the system being deprecated.

3. **A documented manual procedure is cheap to repeat.** The dispatch is two shell commands (label create + `gh api` POST). The body of the dispatch is the actual content; the wrapper is ~30 lines of jq/JSON. Documenting it is sufficient until the v2 tool surface stabilizes.

The cycle 5 commitment also said: any *fourth* defer is evidence the dispatch isn't actually high-priority. Cycle 6 honors the binding by executing.

## Procedure (re-runnable)

### 1. Ensure label exists

```bash
gh label list | grep -q '^feedback-only' || \
  gh label create feedback-only \
    --description "Feedback-only Copilot dispatch (do not modify code; produce critique artifact)" \
    --color "BFDADC"
```

### 2. Author the dispatch body

Body should specify:

- **Context** — what the artifact is, where to find it, why critique is wanted.
- **Lens framing** — the dispatch's value-add comes from the model's *lack of project context*, not from the model class. Name this explicitly so Copilot does not lift the orchestrator's priors.
- **Specific lenses to critique** — focused, numbered prompts. Open-ended "what's wrong" produces shallow critique; specific lenses produce evidence-cited critique.
- **Deliverable format** — exactly one new file at an exact path (under `docs/redesign/_notes/cycle-N-copilot-feedback.md` is the convention); one PR; do not modify any other file.
- **What NOT to do** — explicit list. Do not edit the artifact under critique. Do not modify tools, prompts, workflow files. Do not open multiple PRs. Do not lift the artifact's framings uncritically.
- **Tone** — honest over polished, adversarial over reassuring, concrete over abstract.

The body is the **primary** safety mechanism. Copilot's default reflex when assigned to an issue is to "implement" — make code changes and PR them. The body's job is to redirect that reflex toward "produce a single critique file." Strong, repeated framing on the deliverable shape is what keeps the dispatch on-task.

### 3. Build the JSON payload and dispatch

```bash
# Body file (gitignored under docs/.tmp-* if you want to write it locally first;
# or pipe directly from a heredoc).

jq -Rs '{
  title: "[redesign-feedback] <one-line description>",
  body: .,
  labels: ["agent-task","feedback-only"],
  assignees: ["copilot-swe-agent[bot]"],
  agent_assignment: {
    target_repo: "EvaLok/schema-org-json-ld",
    base_branch: "master",
    model: "gpt-5.4",
    custom_instructions: ""
  }
}' <body-file> | gh api repos/EvaLok/schema-org-json-ld/issues --method POST --input -
```

The `agent_assignment` field is what triggers Copilot to begin working. Without it, the issue is just an issue. With it, Copilot polls, clones, makes a branch, and opens a PR.

### 4. Capture the dispatch in the notes file

Document: dispatch issue number, dispatched-at timestamp, the lenses used, and (when the PR lands) the PR number, the file Copilot produced, and the orchestrator's read of the critique. The notes file is the cross-cycle reconciliation point.

### 5. On Copilot's PR

When Copilot opens a PR:

- Read the file (the actual critique). The PR body is secondary.
- Verify the PR modifies *only* the intended file. If Copilot strayed (modified other files, opened multiple PRs, edited the artifact under critique), close the PR without merging and re-dispatch with sharper framing.
- Decide on integration:
  - **Merge** the PR if the critique file should live in the repo as a record of cycle-6 outside-AI feedback. This is the default — even if the critique is weak, having it on record is useful.
  - **Close without merging** only if the file genuinely should not be in the repo (e.g., off-topic, duplicates audit critique without adding anything, contains sensitive content).
- Subsequent cycles read the merged file and integrate findings into `0-retrospective.md` per the same iteration discipline used for audit critique (cycle 3 model: each finding evaluated independently, integrated with rationale, dismissed with rationale).

## Cycle 6 execution log

| Field | Value |
|---|---|
| Dispatched at | 2026-04-27T20:27:31Z |
| Issue number | [#2748](https://github.com/EvaLok/schema-org-json-ld/issues/2748) |
| Title | `[redesign-feedback] Critique on Phase 0 retrospective for v2 redesign (cycle 6)` |
| Labels | `agent-task`, `feedback-only` |
| Assignee | Copilot (`copilot-swe-agent[bot]`) |
| Model | `gpt-5.4` |
| Target deliverable | `docs/redesign/_notes/cycle-6-copilot-feedback.md` (single file, single PR) |
| Body length | 7798 bytes |
| PR number | TBD (cycle 7 should see it) |
| Integration cycle | TBD (likely cycle 7 or 8) |

## Lenses dispatched

The seven lenses in the body, one-line summary each (full text in the issue body):

1. **Organizational structure** — does F1-F12 organization help or obscure? Is the shared-root preamble unification real to a fresh reader? Better groupings?
2. **F1+F5+F11+F12 mechanical connection** — count overlap (4 of 5) is consistent with 62% defense base rate. Does the retrospective text establish causal mechanism beyond count? Quote strongest and weakest evidence.
3. **Plausibility of v2 success criteria** — 4× state-surface reduction; schema-work fraction threshold. Load-bearing or aspirational? Gameable?
4. **Self-congratulation detection** — places where the document compliments itself rather than makes load-bearing claims with evidence.
5. **Evidence sufficiency** — claims stronger than the data supports.
6. **What's missing** — outside-reader expectations the retrospective doesn't address (cost analysis, cross-system comparison, human-in-the-loop, etc.).
7. **Legibility to a reader without v1 context** — jargon and v1-specific references that an outside reader cannot decode.

Lens 2 is the highest-priority — it's the cycle 5 uncertainty I most want an outside reader to evaluate. Lens 1 is also high-priority — F-pattern naming/numbering may be obscuring more than it reveals.

## What this dispatch does not do

- **It does not request implementation work.** The body is unambiguous: feedback-only, single critique file, no other modifications.
- **It does not constrain Copilot's perspective.** The body specifies lenses but does not pre-write the critique. Copilot can disagree with the orchestrator's framings.
- **It does not gate further cycles.** The dispatch is asynchronous; cycle 6 continues with the adversarial re-read and F12 catalog work in parallel. When the PR lands (likely cycle 7), that cycle reads it.

## Risk: Copilot may stray

The named risk from cycle 5 was that Copilot might create a PR with unwanted changes despite "feedback only" framing. Mitigations applied:

1. The body is explicit, repeated, and structured — not a single sentence.
2. The deliverable is a *new* file at a specific path, not an edit to existing files. Easier for Copilot to satisfy correctly.
3. The "What NOT to do" section is named with bold formatting and clear examples.
4. The PR can be reviewed before merge; non-conforming output is rejected and re-dispatched with sharper framing.

If cycle 7 finds Copilot strayed: close the non-conforming PR, take the lessons (what framing was insufficient?), document them here, and re-dispatch with corrections. Do *not* merge a non-conforming PR just because the critique inside is useful — the convention should hold or the dispatch model breaks.

## Open question for the v2 tool design

When the v2 tool surface is designed (Phase 2/3), should there be a `feedback-dispatch` tool? Arguments for:

- Repetitive enough that automation has value (audit-style critique requests will happen multiple times across the redesign and into v2's running life).
- Tool can enforce conventions (file path, deliverable shape, label, framing template) better than free-form body authoring.
- Aligns with `<core-design-principle>` (procedural work belongs in tools).

Arguments against:

- The body content is the primary value; the wrapper (label + JSON envelope) is small.
- A tool designed before the v2 prompt is designed risks shaping the prompt around the tool rather than the other way around.
- The lens choice is a per-dispatch judgment call; templating it could homogenize critique requests.

Neither argument is decisive. The procedure documented here is sufficient for cycle 6 and likely the next several feedback dispatches; the question of whether to bake it into a tool can wait for Phase 2's tool-suite design.