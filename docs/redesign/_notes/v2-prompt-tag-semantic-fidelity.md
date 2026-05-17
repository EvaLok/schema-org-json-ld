# v2-prompt-tag-semantic-fidelity — design scope

**Status:** design-scope only (cycle 168); implementation deferred to cycle 169+.

**Provenance:** L2.5 from PR #2951 cycle 148 absorption (`cycle-148-two-track-absorption-and-landing.md` lines 82-83). The verdict was AGREE-RECORD + TOOL-SCOPED: the proposed semantic-fidelity check is correct in principle but needed deferral past first-end-to-end-measurement (cycle 153) to confirm the underlying problem is real-enough-to-tool. Forward-priority carry-forward 19 cycles (148→167); first design-scope authoring this cycle.

**Sibling-vs-extension decision:** standalone binary `v2-prompt-tag-semantic-fidelity` recommended over extending `v2-prompt-contract-check`. Section 3 lays out the rationale.

## 1. Problem statement

### 1.1 What the cycle 148 critique identified

The v2 role prompts share a template-mirror architecture: each of the four role prompts (planner, executor, curator, reconciler) has a similar top-level tag skeleton (`<role-identity>`, `<inputs>`, `<output-contract>`, `<constraints>`, `<session-structure>`). PR #2951 L2.5 named the risk: section tag names imported from a planner template can persist into other roles whose content has drifted away from what the tag-name claims, producing semantic drift that is invisible at the keys-aligned level.

The concrete failure mode: a `<role-identity>` block where the content actually describes step-by-step procedure rather than identity; an `<output-contract>` block whose body specifies inputs; a `<one-line>` block containing five paragraphs. The tag-name promises function X but the content performs function Y. Readers (including future cycle-runner sessions and dispatched Copilot critique sessions) silently absorb the mismatch as truth.

### 1.2 What current tooling does NOT catch

`v2-prompt-contract-check` parses `<output-contract>` blocks and verifies the declared required keys match the v2-channel-router schema. It does NOT verify that the tag named `<output-contract>` actually contains output-contract content. As long as the `<required-payload-keys>` sub-block names the right keys, the surrounding tag could be named anything and the check would pass.

No other v2 tool inspects tag-content semantic alignment. Reviewer eyes and dispatched-critique sessions are the only existing layer; both are episodic and miss drift between reviews.

### 1.3 The cycle 153 first-end-to-end gate

Cycle 148's verdict deferred the tool because runtime evidence was needed: maybe semantic drift doesn't actually cause runtime issues; maybe contract-check is enough. The cycle 153 first-end-to-end run did NOT surface direct runtime breakage attributable to tag-content drift. However, the cycles 155-165 critique-absorption stream surfaced multiple instances where dispatched Copilot critique sessions raised semantic-alignment findings (e.g., PR #2961 X5, the channel-router extension critique findings). The pattern: drift does not break the runner, but it accumulates documentary debt that critique sessions then have to call out cycle after cycle. The tool's value proposition is **prevention**: catch drift at write-time so subsequent critique sessions can spend their compute on substantive concerns rather than re-discovering tag-misnaming.

## 2. Tool boundary

### 2.1 In scope

- Static analysis of the 4 v2 role prompt XML files (`prompts/v2/*-prompt.xml`).
- Reports tag-content semantic mismatches at two tiers (Section 5).
- Reports missing canonical tags (Section 4.3).
- Reports tags lacking either manifest entry or `<semantic-adaptation-note>` child.
- Emits text + JSON output formats.
- Exit codes: 0 clean, non-zero on mismatches under `--strict`.

### 2.2 Out of scope (explicit non-doings)

- NOT LLM-based; deterministic checks only. (No live Claude session, no dispatch.)
- Does NOT modify prompts. Read-only.
- Does NOT enforce content quality, style, or correctness. Only tag-content fidelity against declared intent.
- Does NOT validate the router channel schema or payload keys — that is `v2-prompt-contract-check`'s scope.
- Does NOT replace adversarial Copilot critique dispatches. Different layer: this is a write-time gate; critique is post-write semantic review.
- Does NOT auto-fix mismatches. Suggestions only.

## 3. Sibling tool vs extension to v2-prompt-contract-check

### 3.1 Option A — Extend v2-prompt-contract-check with `--check-tag-fidelity` flag

Pro: single tool surface; one binary to invoke; one CI gate.

Con: contract-check's responsibility is channel-schema alignment (output payload keys vs router schema). Tag-fidelity is a different axis: structural tag-content semantic alignment. Coupling them violates the SINGLE-RESPONSIBILITY discipline that justifies the per-primitive split in the v2 design (cycle 161 X5 carveout precedent — `v2-cycle-runner` should not also do commit-discipline checks; same reasoning here). Future evolution of one check might force unwanted coupling on the other.

### 3.2 Option B (RECOMMENDED) — Standalone binary `v2-prompt-tag-semantic-fidelity`

Pro: single-responsibility tool. Each check has its own CLI surface, exit code conventions, output format. Can be invoked independently for fast feedback (semantic-fidelity is faster than contract-check which shells to v2-channel-router for schema).

Con: two CLI tools to invoke from CI / dispatch reviews; ~slight extra cargo workspace member.

The "two tools" con is minor: CI can run them in sequence, and dispatch-review tooling can compose them. The cycle 165 PR #2975 absorption (which landed v2-state-dispatch-archive as its own binary alongside v2-state-dispatch-sync) is the precedent for siblings-over-extensions when the concerns are distinct.

### 3.3 Decision

**Standalone binary recommended.** Tool name: `v2-prompt-tag-semantic-fidelity`. Workspace member at `tools/rust/crates/v2-prompt-tag-semantic-fidelity/`. Wrapper script at `tools/v2-prompt-tag-semantic-fidelity` per the existing convention.

## 4. Manifest source-of-truth

The tool needs a declaration of "what each tag means" against which to check. Three placement options:

### 4.1 Option A — TOML manifest at `prompts/v2/tag-semantics.toml`

Pro: separates declarative semantics from prompt content; human-readable; Rust-ecosystem-friendly (the `toml` crate is already a v2 workspace dependency via `serde`); easy to diff in PRs.

Con: two files of truth (manifest + prompts) that can themselves drift. Mitigation: the tool catches drift in BOTH directions (manifest entries with no matching prompt tags; prompt tags with no manifest entry).

### 4.2 Option B — Inline `<tag-intent>` attribute or child element per tag

Pro: single source of truth — the prompt declares its own intent inline; no separate manifest to maintain.

Con: every prompt gets cluttered with `<tag-intent>` declarations; prompts become harder to read; the "what does this tag mean" semantic lives in 4 places (one per role) instead of 1.

### 4.3 Option C (RECOMMENDED) — TOML manifest with canonical-required + role-allowed declarations

Combines the clarity of separate file with explicit cross-role coordination:

```toml
# prompts/v2/tag-semantics.toml

# Top-level tags expected to appear at the root <role-prompt> level.
# "required-in-roles" lists roles that MUST include this tag (missing → error).
# "allowed-in-roles" lists roles that MAY include it (omitting "required" is acceptable).
# "intent" is the one-line semantic description against which content is checked.
# "expected-children" optionally lists immediate child tags that should appear.

[tags.role-identity]
intent = "Names the role, its decision authority, its position in the super-step sequence, and the cycle-scope it operates within"
required-in-roles = ["planner", "executor", "curator", "reconciler"]
expected-children = ["one-line", "position-in-super-step-sequence", "one-cycle-scope"]

[tags.output-contract]
intent = "Specifies the output payload contract: required keys, payload shape, channel destination"
required-in-roles = ["planner", "executor", "curator", "reconciler"]
expected-children = ["channel", "required-payload-keys"]

[tags.inputs]
intent = "Enumerates the input channels and harness-provided context the role reads at session start"
required-in-roles = ["planner", "executor", "curator", "reconciler"]

# ... etc.

# Cross-role shared tags: same name, allowed in multiple roles, same intent across roles.
# Per-role intent divergence requires <semantic-adaptation-note>.

[tags.constraints]
intent = "Hard rules the role must obey; tooling expectations; what the role MUST NOT do"
allowed-in-roles = ["planner", "executor", "curator", "reconciler"]

[tags.session-structure]
intent = "The procedural sequence the role follows during its session"
allowed-in-roles = ["planner", "executor", "curator", "reconciler"]
```

The manifest is canonical; the tool reads it and the 4 prompts, cross-references them, and reports drift.

### 4.4 Manifest authoring (implementation phase)

The implementation cycle (169+) extracts the initial manifest by scanning the current v2 prompts at HEAD. Anything that appears in any prompt becomes a manifest entry with `intent` written by the implementing cycle. Tag names that appear in N>1 roles get `allowed-in-roles` listing all N. Cycle 169 manifest authoring is therefore "consolidate current state into canonical declaration."

Subsequent prompt iterations either match the manifest (clean) or add `<semantic-adaptation-note>` justifying divergence (which the tool flags as advisory for review).

## 5. The two-tier check

### 5.1 Tier 1 — mechanical (deterministic, fail-closed under --strict)

Per top-level tag in each prompt:

1. **Lookup**: find `(tag-name)` in manifest.
2. **Role-allowed check**: if `required-in-roles` is set, the tag must appear in those roles. If `allowed-in-roles` is set, the tag may appear in those roles only.
3. **Unknown-tag check**: a tag in a prompt with NO manifest entry and NO `<semantic-adaptation-note>` child is an error.
4. **Canonical-required check**: per role, every tag listed in `required-in-roles` for any manifest entry MUST appear in that role's prompt.
5. **Expected-children check**: if `expected-children` is set for a tag, verify those children appear directly under the tag.

Under `--strict`, any Tier 1 failure exits non-zero. Without `--strict`, Tier 1 failures emit but exit 0.

### 5.2 Tier 2 — content heuristics (advisory, warnings only)

Per (role, tag) pair with declared intent:

1. **Length plausibility**:
   - `<one-line>` tags should contain ≤2 lines of non-comment text (after XML-comment strip).
   - `<role-identity>` should contain ≥3 sub-elements.
   - `<output-contract>` should be ≥10 lines (heuristic — small contracts are usually under-specified).
2. **Keyword overlap with intent**:
   - Extract content tokens from the tag body; lowercase, strip XML; compare against intent-string tokens.
   - If overlap < 1 substantive token AND content is ≥3 lines, flag as suspicious.

Tier 2 is heuristic; false positives are expected. Output as WARNING, never ERROR. Allows human review without breaking CI.

### 5.3 `<semantic-adaptation-note>` semantics

When a prompt diverges from manifest intent for a tag, the prompt author adds:

```xml
<role-identity>
  <semantic-adaptation-note>
    This role-identity block is reused from the planner template but adapted
    to executor's context: the "one-cycle-scope" sub-tag describes executor's
    multi-dispatch coordination rather than planner's single-pass plan.
    The cross-role intent (named role, decision authority, position, scope)
    still holds, but children differ.
  </semantic-adaptation-note>
  <!-- ... rest of role-identity content ... -->
</role-identity>
```

The tool treats the presence of `<semantic-adaptation-note>` as PASS-WITH-NOTE: Tier 1 unknown-tag check is suppressed; Tier 2 heuristics still emit warnings (since the note doesn't prove fidelity, only acknowledges divergence). The note text is included in the report so reviewers can decide if the divergence is acceptable.

## 6. CLI surface

```
v2-prompt-tag-semantic-fidelity check \
    --prompts-dir prompts/v2/ \
    --manifest prompts/v2/tag-semantics.toml \
    [--strict] \
    [--format text|json]

v2-prompt-tag-semantic-fidelity schema     # print expected manifest shape
v2-prompt-tag-semantic-fidelity list-tags  # list all top-level tags found across prompts
```

Default behavior: read both manifest and prompts; report; exit 0 on Tier 1 clean, 1 on Tier 1 failures under --strict.

## 7. Output schema (JSON)

```json
{
  "schema_version": "v1",
  "prompts_dir": "prompts/v2/",
  "manifest_path": "prompts/v2/tag-semantics.toml",
  "prompts_scanned": ["planner-prompt.xml", "executor-prompt.xml", "curator-prompt.xml", "reconciler-prompt.xml"],
  "tier1_findings": [
    {
      "role": "executor",
      "tag": "rare-tag-name",
      "kind": "unknown-tag",
      "manifest_entry": null,
      "has_adaptation_note": false,
      "severity": "error"
    },
    {
      "role": "curator",
      "tag": "output-contract",
      "kind": "missing-expected-child",
      "expected_children": ["channel", "required-payload-keys"],
      "found_children": ["required-payload-keys"],
      "severity": "error"
    }
  ],
  "tier2_findings": [
    {
      "role": "planner",
      "tag": "one-line",
      "kind": "length-plausibility",
      "details": "found 6 lines; expected ≤2 for one-line",
      "severity": "warning"
    }
  ],
  "summary": {
    "tier1_errors": 2,
    "tier1_passes": 18,
    "tier2_warnings": 1,
    "adaptation_notes_seen": 0
  },
  "exit_code": 1
}
```

## 8. Test plan

### 8.1 Unit tests

- Manifest TOML parser (round-trip + invalid-input handling).
- Tier 1 checks: each finding kind (unknown-tag, missing-required, missing-expected-child, role-not-allowed) has at least one fixture.
- Tier 2 checks: length-plausibility and keyword-overlap each have at least one positive + one negative fixture.
- `<semantic-adaptation-note>` detection: with-note and without-note both verified.
- CLI exit codes: --strict yes/no × tier1-clean/dirty combinations.

### 8.2 Integration test

A `tests/integration_fidelity.rs` test that:

1. Builds a minimal `prompts/v2/` fixture (4 prompts + manifest).
2. Runs the tool with `--strict` against the clean fixture → exit 0.
3. Mutates the fixture: rename a tag, drop a required tag, inject an unknown tag without adaptation-note.
4. Runs against the mutated fixture → exit 1 with findings reported.

### 8.3 Real-prompt regression

A `tests/integration_real_prompts.rs` test that runs against the actual `prompts/v2/` directory at the cycle 169 manifest baseline; should exit 0. CI runs this on every commit touching `prompts/v2/` or `tools/rust/crates/v2-prompt-tag-semantic-fidelity/`.

## 9. Open questions for implementation (cycle 169+)

1. **Where to put the manifest file**: `prompts/v2/tag-semantics.toml` (next to prompts) vs `tools/rust/crates/v2-prompt-tag-semantic-fidelity/tag-semantics.toml` (with the tool). Lean prompts-dir: the manifest is prompt-architecture data, not tool-architecture data; lives near what it describes.
2. **Tier 2 keyword-overlap threshold**: `<1 substantive token` is a strawman; concrete threshold needs empirical calibration against the current 4 prompts. Implementation cycle should report Tier 2 warnings against the baseline and tune until false-positive rate is acceptable.
3. **Cross-role intent divergence policy**: today's design says any role-divergence requires `<semantic-adaptation-note>`. Alternative: allow per-role intent declarations in the manifest (`[tags.role-identity.planner]` overrides default `[tags.role-identity]`). Lean against per-role manifest overrides for cycle 1 — keeps manifest small; revisits if 3+ tags need them.
4. **CI integration**: does this tool block CI on Tier 1 errors? Lean yes for `prompts/v2/` PRs; lean advisory-only for non-prompt PRs. Concrete decision deferred to implementation cycle.
5. **Coupling with v2-prompt-contract-check**: should the two tools share an `output-contract` parsing utility? Possible refactor candidate during implementation if both end up parsing `<output-contract>` independently. Lean: shared crate `prompts-xml-parse` if duplication exceeds ~30 LOC.
6. **`<semantic-adaptation-note>` text quality enforcement**: should the tool require non-empty / minimum-length notes? Lean no — text quality is a critique-session concern, not a static-check concern. The note's existence is the gate.

## 10. Ordering vs other v2-* tools / prompt-iteration

- This tool's first invocation will likely flag findings against current `prompts/v2/*-prompt.xml`. The cycle 169+ implementation cycle is responsible for either:
  - (a) Adjusting the manifest to match current prompt state (declare the current shape canonical), or
  - (b) Adding `<semantic-adaptation-note>` blocks where prompts diverge from intuitive manifest intent, or
  - (c) Modifying prompts to match the cleaner intent.
  
  Lean (a) for the cycle 1 baseline (preserve current prompt content), with future iterations adopting (b) or (c) as drift accumulates.

- The cycle 168+ priority #10 ("honesty-pass on v2 role prompts" gated on PR #2979) should run AFTER this tool exists, so the honesty-pass can use the tool's findings to identify drift candidates. Sequencing: PR #2979 lands → tool implementation cycle 169+ → tool manifest baseline cycle 169+ → honesty-pass cycle informed by tool findings.

- Cycle 168+ priority #11 (`v2-prompt-contract-check --strict` re-run post-extension) is orthogonal; both tools can run independently in CI.

## 11. What this design scope does NOT do

- Does NOT implement the tool (cycle 169+ implementation work).
- Does NOT author the manifest (part of implementation cycle).
- Does NOT modify the v2 prompts.
- Does NOT add a CI gate (separate workflow YAML PR).
- Does NOT specify cargo workspace metadata changes (mechanical; part of implementation).
- Does NOT extend `v2-prompt-contract-check` (Section 3 decision).
- Does NOT couple to the v2-channel-router schema (independent validation surface; Section 9.5 leaves shared-parsing as a possible refactor only).
- Does NOT name a CI integration date — left to the workflow-edit PR that follows implementation.
- Does NOT consume any v1 / production prompt files; the v1 `.github/workflows/orchestrator-prompt.xml` is out of scope for this tool by design (v1 is forbidden-zone reference material per the redesign-prompt direct-push-zones rule).

## 12. Closure

This design scope closes cycle 168+ forward priority #2 (TOOL-SCOPED `v2-prompt-tag-semantic-fidelity` tool design scope), which inherited from cycle 148 L2.5 across 19 carry-forward cycles. The next action is implementation cycle 169+ (forward priority added to cycle 168 close-out _notes).
