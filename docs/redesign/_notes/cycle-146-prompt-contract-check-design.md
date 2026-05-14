---
cycle: 146
date: 2026-05-14
focus: prompt-contract-check tool design scope (Track 2 substantive focal)
forward-priority-honored: cycle 145 #2 (prompt-contract-check design; directive #2937 item #4; cycle 139 forward priority #5)
status: SCOPE-DOC (not implementation; cycle 147+ implementation candidate)
---

# Cycle 146 — `v2-prompt-contract-check` tool design scope

## Why this tool

The 4-prompt role set landed cycle 145. Each prompt declares an `<output-contract>` block that names:

- the channel the role writes to (e.g., planner writes plan-channel)
- the `<required-key>` elements (e.g., planner requires `substantive-focal`, `per-role-tasks`)
- the `<optional-key>` elements (e.g., planner optionally includes `rationale`, `forward-notes`)

`v2-channel-router/src/main.rs:117` separately defines `Channel::required_payload_keys()` — the keys the channel-router enforces at write time. The two declarations MUST stay synchronized. They are currently aligned by inspection (cycle 146 session-start verified all 4 channels match). However:

- The declarations are in two separate locations (XML prompt + Rust source), authored by separate sessions (planner prompt = main; reconciler/executor/curator = 3 dispatched Copilot sessions).
- The synchronization is verified by human (= orchestrator) inspection. No tool enforces it.
- Per CORE-DESIGN-PRINCIPLE (redesign prompt SECTION 1.5): anything the orchestrator does the same way every cycle is a failure to extract into a tool. Verifying contract alignment by inspection every cycle is exactly this failure pattern.

`v2-prompt-contract-check` is the tool that extracts the inspection into a deterministic check the harness runs without orchestrator attention.

## What it checks

The tool verifies, for each of the 4 role prompts:

### Check 1: Output-channel declaration matches channel-router's writer map

For each prompt, parse the `<output-contract><channel>` declaration. Verify it matches `Role::writer_for_channel(channel)` in v2-channel-router — i.e., the prompt's declared channel has the prompt's role as its single allowed writer.

- planner prompt `<output-contract><channel>plan-channel</channel>` → channel-router declares `Channel::PlanChannel::writer() == Role::Planner` → match required
- reconciler prompt `<output-contract><channel>inbound-channel</channel>` → channel-router declares `Channel::InboundChannel::writer() == Role::Reconciler` → match required
- executor prompt `<output-contract><channel>work-channel</channel>` → channel-router declares `Channel::WorkChannel::writer() == Role::Executor` → match required
- curator prompt `<output-contract><channel>memory-channel</channel>` → channel-router declares `Channel::MemoryChannel::writer() == Role::Curator` → match required

### Check 2: Required-output-keys match channel-router's `required_payload_keys()`

For each prompt's `<output-contract>`, extract the set of `<required-key name="...">` elements. Compare to `Channel::required_payload_keys()` for that prompt's declared channel.

- Set equality is the success criterion. Subset is insufficient (prompt declares fewer keys than router requires = runtime payload rejection). Superset is failure (prompt declares keys router doesn't enforce = decorative declaration that confuses readers).
- Tool reports both directions: missing-in-prompt and extra-in-prompt, with channel + key names + file locations.

### Check 3: Input-channel declarations match channel-router's `Channel::all()` membership

For each prompt's `<inputs><source channel="...">`, verify the named channel exists in channel-router's `Channel::all()`. Catches typos and stale channel-name references.

### Check 4: Required-input-keys match channel-router's `required_payload_keys()` for that input channel

For each prompt's `<inputs><source channel="X"><required-key name="Y">`, verify the named key is in `Channel::X::required_payload_keys()`. Catches drift where a prompt reads a key the writer-prompt no longer produces.

### Check 5 (deferred, post-cycle-1): Optional-key declarations

`<optional-key>` elements in either input or output contract. Cycle 147+ extension: verify optional keys are documented somewhere (channel-router source comments or a separate channel-schema file). For cycle-1 minimal, optional-keys are unenforced.

## Where it runs

### Primary: cycle-runner harness pre-execution gate (runtime)

The cycle-runner invokes `v2-prompt-contract-check --strict` before invoking any role. If any check fails, the cycle aborts with the diff displayed. Fail-closed; no override at runtime.

This is the load-bearing invocation. It ensures every cycle starts with a verified prompt-contract alignment.

### Secondary: pre-commit / CI hook (developer-time)

A pre-commit hook (or workflow job under `.github/workflows/`) runs `v2-prompt-contract-check --strict` on any PR that modifies files under `prompts/v2/` or `tools/rust/crates/v2-channel-router/`. Catches drift at PR review time before it reaches main.

### Tertiary: ad-hoc developer invocation

Direct CLI usage during prompt-set development. `v2-prompt-contract-check --verbose` outputs the per-channel comparison for human inspection.

## Input contract

```
v2-prompt-contract-check
  --prompts-dir prompts/v2/        # default: prompts/v2/
  --channel-router-bin /path/to/v2-channel-router  # or extract from source
  --strict                          # fail-closed, exit 1 on any mismatch
  --json                            # machine-readable diff output
  --verbose                         # human-readable per-channel comparison
  --allow-extra-optional            # cycle-1: optional-keys are unenforced
```

## Output contract

### Match case

```
v2-prompt-contract-check: 4/4 prompts match channel-router contract
  ✓ planner-prompt.xml ↔ Channel::PlanChannel
  ✓ reconciler-prompt.xml ↔ Channel::InboundChannel
  ✓ executor-prompt.xml ↔ Channel::WorkChannel
  ✓ curator-prompt.xml ↔ Channel::MemoryChannel
```
Exit 0.

### Mismatch case

```
v2-prompt-contract-check: MISMATCH in executor-prompt.xml ↔ Channel::WorkChannel
  channel-router requires: ["artifacts-written"]
  prompt declares:          ["artifacts-written", "files-changed"]
  EXTRA-IN-PROMPT: ["files-changed"] (decorative; channel-router does not enforce)
  MISSING-IN-PROMPT: []

  Diff location:
    channel-router: tools/rust/crates/v2-channel-router/src/main.rs:120
    prompt:         prompts/v2/executor-prompt.xml:131
```
Exit 1.

### JSON mode

```json
{
  "matched": false,
  "mismatches": [
    {
      "prompt_file": "prompts/v2/executor-prompt.xml",
      "channel": "WorkChannel",
      "kind": "extra-in-prompt",
      "channel_router_keys": ["artifacts-written"],
      "prompt_keys": ["artifacts-written", "files-changed"],
      "delta": {
        "extra_in_prompt": ["files-changed"],
        "missing_in_prompt": []
      }
    }
  ]
}
```

## Implementation choices that need decisions

### Choice A: Extract channel-router truth from source code OR runtime binary

**Option A1 (source parse):** Parse `tools/rust/crates/v2-channel-router/src/main.rs` for the `required_payload_keys()` match statement. Brittle (regex against Rust source); breaks if channel-router refactors.

**Option A2 (runtime binary call):** Run `v2-channel-router schema --json` (new subcommand to add) and consume the structured output. Robust; requires adding the `schema` subcommand to channel-router.

**Recommendation: A2** — adding the `schema` subcommand to v2-channel-router is small (~20 lines), produces a canonical machine-readable truth source, and decouples the check tool from channel-router's source layout. The subcommand also serves direct developer inspection ("what's the truth about plan-channel's required keys?").

### Choice B: XML parse approach

**Option B1 (XPath via xmltree crate):** Use a proper XML parser. Robust to whitespace, comments, namespaces.

**Option B2 (regex):** Regex against XML source. Faster but brittle.

**Recommendation: B1** — `xmltree` or `quick-xml` is a small dependency and the prompts are LLM-readable XML that may include whitespace/comment variation. Brittle regex would produce false-negative-prone behavior we'd then debug under cycle pressure.

### Choice C: Where the tool's binary lives

**Option C1 (separate crate):** `tools/rust/crates/v2-prompt-contract-check/`. Symmetric with other v2 crates.

**Option C2 (subcommand of channel-router):** `v2-channel-router check-contracts --prompts-dir prompts/v2/`. Co-locates the check with the truth source.

**Recommendation: C1** — separate crate. The check tool is conceptually a CONSUMER of channel-router's schema; embedding it in channel-router conflates producer and consumer. Cycle 140-143 SCAFFOLD discipline kept one crate per primitive; this preserves the pattern.

## What's IN-SCOPE for cycle 147+ implementation

- All 5 checks (1-4 mandatory; 5 deferred).
- Choice A2 (runtime binary call + new `v2-channel-router schema --json` subcommand).
- Choice B1 (proper XML parse via `quick-xml` or equivalent).
- Choice C1 (separate crate at `tools/rust/crates/v2-prompt-contract-check/`).
- Cargo unit tests for: match case, mismatch-by-extra, mismatch-by-missing, channel typo, JSON output format.
- Integration test: run against the live `prompts/v2/` + live `v2-channel-router schema` output; expect match.

## What's DEFERRED to cycle 148+

- Check 5 (optional-key documentation verification).
- Pre-commit hook installation (separate from the tool itself).
- CI workflow integration (`.github/workflows/` is forbidden zone; PR-required).
- The `v2-channel-router schema --json` subcommand itself — needs its own implementation cycle, but the spec is here.
- Cycle-runner harness integration — depends on the cycle-runner harness rewrite (PR-required forbidden zone; separate multi-cycle arc).

## Predicted SCAFFOLD LOC (cycle 139 magnitude-prediction-precision-is-shape-dependent-not-flat pattern application)

Estimated SCAFFOLD scope LOC for `v2-prompt-contract-check`:

| Component | Predicted LOC band | Family |
|---|---|---|
| Crate prod source | 400-600 | XML-parse + diff-emit + CLI (smaller than channel-router because narrower) |
| Crate test source | 300-500 | Per-check unit tests + integration test |
| v2-channel-router `schema --json` subcommand | 30-60 | Small extension |
| Total prod LOC | 430-660 | — |
| Total test LOC | 300-500 | — |

This is the FIRST data point in the "verification tool" family (a distinct family from "channel state machinery" v2-channel-router, "boundary-discipline" v2-super-step-boundary, "role-driver" v2-role-driver, "event-processor" v2-reconciler-event-processor, "prompt" v2-role-prompts). The pattern claim is: verification tools should be SMALLER than the systems they verify, but with comparable test density (because their entire purpose is correctness).

Cycle 147+ implementation will be the recurrence test for this prediction.

## What this scope-doc does NOT do

- Does NOT implement the tool. Cycle 147+ work (after cycle-runner harness rewrite per forward priority #4).
- Does NOT modify `v2-channel-router` to add the `schema --json` subcommand. Cycle 147+ work.
- Does NOT install a pre-commit hook or modify `.github/workflows/`. Forbidden zone; PR-required.
- Does NOT decide whether prompt-contract-check should ALSO verify the prompts' XML well-formedness or schema validity beyond the contract-alignment checks. Cycle 148+ consideration.
- Does NOT decide whether the cycle-runner should run prompt-contract-check on EVERY cycle vs on a triggered basis (e.g., only after `prompts/v2/` or `v2-channel-router` source changed since last cycle). Cycle 148+ optimization.
- Does NOT specify the failure-mode UX when prompt-contract-check fails inside the cycle-runner. The natural answer is "fail-closed; abort cycle; surface error in next reconciler's inbound-channel"; cycle 147+ harness rewrite is where this gets concretized.
- Does NOT predict whether the cycle 146 adversarial-critique dispatch (#2947) will surface contract-design issues that change this scope. If the returned critique finds load-bearing issues, the scope-doc is revised before implementation.

## Forward priorities (preserved from cycle 145 + cycle 146 updates)

1. **Cycle 146 adversarial-critique dispatch (#2947) absorption** — returns async; cycle 147+ absorbs. NEW priority #1.
2. **prompt-contract-check implementation** (this scope-doc; cycle 147+ work after #1 absorbs).
3. **Audit cycle 219 critique absorption** — STILL BLOCKED (audit HEAD `72cda153` unchanged; 5 expected-cycles overdue).
4. **Cycle-runner harness rewrite** (forbidden zone; PR-required; multi-cycle arc).
5. **First end-to-end run + first measurement** (cycle 148+ after #4).
6. **`v2-phase-transition-check` test failures triage** (carry-over).
7. **Two open-questioned crates design** (carry-over).
8. **Cycle 145 NEW candidate-emergent observation reinforcement** (`dispatch-brief-template-mirror-tradeoff` — cycle 146 dispatch is the recurrence test; result returns cycle 147+).
9. **Phase 1 research deepening** (carry-over).
10. **Cycle 120 L2 constraint preserved** (no recursive annotation of `2-selection.md`).
