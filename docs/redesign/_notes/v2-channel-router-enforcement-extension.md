# v2-channel-router enforcement extension — design scope

**Status:** design scope (cycle 164, 2026-05-17). Implementation not yet built.
**Cycle origin:** cycle 164 Track 2 (substantive focal, paired with Track 1 Copilot dispatch [#2974](https://github.com/EvaLok/schema-org-json-ld/issues/2974)). Closes cycle 163 forward priority #4 (was cycle 162 priority #5, was cycle 161 priority #5; originally C1 / L3.1 from cycle 148 absorption).
**Predecessor scopes:** [`cycle-146-prompt-contract-check-design.md`](cycle-146-prompt-contract-check-design.md) (cycle 147 implementation, key-set alignment, in production); [`cycle-148-two-track-absorption-and-landing.md`](cycle-148-two-track-absorption-and-landing.md) (C1 + L3.1 critique verdicts establishing the gap); cycle 148 forward implications item #4.
**Pairs with:** [`v2-prompt-tag-semantic-fidelity` design scope] (cycle 165+ priority #5; L2.5 from cycle 148 absorption) — both tools harden prompt↔router agreement at orthogonal axes.

## 1. Problem statement

### 1.1 What v2-channel-router currently enforces

`tools/rust/crates/v2-channel-router/src/main.rs:367-385` (`fn validate_payload`) enforces exactly two invariants at write-time:

1. **Payload is a JSON object** (rejects array/string/number/null/boolean as payload root).
2. **Required keys exist** — for each name in `Channel::required_payload_keys()`, the payload object contains a property with that name.

The source comment at line 364-366 names the gap explicitly: *"Does not type-check nested fields; that work is deferred to v2-channel-router COMPLETE arc."*

### 1.2 What prompts declare

Each role prompt under `prompts/v2/` declares an `<output-contract>` with `<channel>` and `<format>` blocks containing `<required-key name="..." type="...">` elements. The `type` attribute names a richer type than the router enforces: `string`, `object`, `array`, `integer`, `boolean`. Some `required-key` elements have nested `<sub-key name="..." type="...">` declaring expected sub-structure for `type="object"` keys.

Examples in production prompts (cycle 144-148 lineage):
- Planner declares `<required-key name="substantive-focal" type="string">` (flat string) AND `<required-key name="per-role-tasks" type="object">` with nested `<sub-key name="executor">`, `<sub-key name="curator">` (nested object shape).
- Reconciler declares `<required-key name="cycle-trace-snapshot" type="array">` (array-typed top-level key, no element-schema declared).
- Executor declares `<required-key name="artifacts-written" type="array">` (array-typed).

### 1.3 The gap

`Channel::required_payload_keys() -> &'static [&'static str]` is **flat key names only**. No type information. No nested sub-key shape. The runtime cannot distinguish:

- `per-role-tasks: "executor"` (string when prompt says object) — **router accepts**.
- `per-role-tasks: { "executor": "..." }` (object missing the `curator` sub-key the prompt promises) — **router accepts**.
- `cycle-trace-snapshot: "..."` (string when prompt says array) — **router accepts**.

The cycle 148 C1 / L3.1 critique verdict was AGREE-RECORD + TOOL-SCOPED: *"PR #2953 (landing cycle 148) addresses the EASIEST layer (key-set alignment) but does not enforce types or nested shapes. The harder enforcement gap is the deepest valid critique in this lens."*

`v2-prompt-contract-check` (cycle 147, 1010 LOC) verifies that the prompt-XML `<required-key>` name set matches `Channel::required_payload_keys()`. It does NOT verify types or nested shapes — the channel-router side has no programmatic source-of-truth for either, so there's nothing to compare against.

**This design scope addresses the dual gap:** extend channel-router to declare types + nested shapes; extend v2-prompt-contract-check to verify them; and harden runtime enforcement to reject mismatched payloads.

## 2. Tool boundary

`v2-channel-router enforcement extension` is **not a new tool**. It is an extension to two existing crates:

- **`tools/rust/crates/v2-channel-router`**: extend `Channel`'s payload-keys API to surface a richer schema (types + nested keys); extend `validate_payload` to enforce types + nested shapes at write-time; extend `run_schema` to emit the richer schema via `schema --format json`.
- **`tools/rust/crates/v2-prompt-contract-check`**: extend prompt-XML parsing to capture types + sub-keys (the parser already does this — confirmed by `parse_output_contract_handles_nested_subkeys_correctly` at line 870); extend the comparison logic to verify types + sub-key sets against the channel-router schema (currently only the flat key-name set is compared).

The crate `v2-channel-router` remains the single runtime gatekeeper for channel writes. `v2-prompt-contract-check` remains the static verifier of prompt↔router agreement. No new crates are introduced.

### 2.1 In scope

- Extend `Channel::required_payload_keys()` → `Channel::payload_schema()` returning a structured per-key descriptor with `name`, `type`, optional nested `sub_keys`.
- Extend `validate_payload` (and any caller in the router pipeline) to walk the schema and reject payloads where a key has the wrong type or where a `type=object` key is missing declared sub-keys.
- Extend `run_schema`'s JSON output to include `type` + `sub_keys` for each key.
- Extend `v2-prompt-contract-check`'s comparison to verify type + sub-key set agreement.
- Honesty-pass on prompts (cycle 148 L3.2): if a prompt declares a stronger guarantee than the extended router enforces (e.g., array element-type), either downgrade the prompt OR upgrade the router to match.
- Add a `--mode strict|lenient` flag to v2-channel-router so cycle 1 of post-extension operation can run lenient (warn-only) before switching to strict (reject).

### 2.2 Out of scope (explicit non-doings)

- Does NOT introduce full JSON Schema dialect support. Use a minimal in-house vocabulary (§4); rejection of "we should adopt JSON Schema" is design-decision §4.6.
- Does NOT enforce element-types within arrays (e.g., "array of strings" vs "array of objects"). Defer to a future cycle if element-type drift surfaces as a real failure.
- Does NOT enforce string formats (length, regex, enum). Channels are operational telemetry, not user input; format validation belongs in domain code.
- Does NOT modify the v2 role prompts in this scope. Prompt edits arrive in a separate cycle as the honesty-pass (cycle 165+ prompt-iteration cycle).
- Does NOT modify v1 prompts/tools (frozen zone).
- Does NOT add a runtime cost path that would slow per-write performance materially. The validation walk is O(N) in declared keys and shallow; no recursive descent past one nesting level in this scope.
- Does NOT touch `v2-cycle-runner`, `v2-state-audit`, or other v2-* tools. The extension is local to channel-router + prompt-contract-check.

## 3. Schema source-of-truth (where the contract lives)

Four options were considered; option B is recommended.

### 3.1 Option A — Extend `required_payload_keys()` in place

Change the return type from `&'static [&'static str]` to `&'static [&'static PayloadKey]` where `PayloadKey { name, ty, sub_keys }`. All call sites adapt.

**Rejected:** the function name no longer matches the return type (it returns descriptors, not keys). Renaming is forced; doing it in place breaks every existing caller. The migration is real.

### 3.2 Option B (RECOMMENDED) — New `Channel::payload_schema()` method

Add a new method `Channel::payload_schema() -> &'static PayloadSchema` returning a structured descriptor. Retain `required_payload_keys()` as a thin wrapper that returns the flat name set (computed from the schema). Migrate callers to `payload_schema()` over a few cycles; deprecate `required_payload_keys()` once all sites are migrated.

**Recommended:** new API is named honestly; existing call sites continue working without change; migration is incremental; the deprecation can be done in a future cycle without blocking this extension.

### 3.3 Option C — External JSON files

Keep `required_payload_keys()` flat; declare the richer schema in per-channel JSON files under `tools/rust/crates/v2-channel-router/schemas/`.

**Rejected:** introduces file-loading complexity, two-source-of-truth risk, and a deserialization step at process startup. The current `&'static` table is fast and immutable; option B preserves that.

### 3.4 Option D — Prompt XML as source-of-truth

Have channel-router parse the prompt XML at startup and derive the schema from it.

**Rejected:** inverts the dependency. The router must remain self-contained; prompts depending on the router is the correct direction (prompts declare against a router-defined contract, not the reverse). Also fragile — prompt-parse errors would break the runtime.

## 4. Type system (the in-house vocabulary)

### 4.1 Primitive types

`string`, `integer`, `number`, `boolean`, `array`, `object`, `null` — directly mapped to `serde_json::Value` discriminants.

### 4.2 PayloadKey descriptor

```rust
#[derive(Debug)]
pub struct PayloadKey {
    pub name: &'static str,
    pub ty: PayloadType,
    pub sub_keys: &'static [PayloadKey],   // empty unless ty == Object
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayloadType {
    String,
    Integer,
    Number,
    Boolean,
    Array,    // element-type not enforced this scope (§2.2)
    Object,
}
```

### 4.3 Sub-key recursion bound

`sub_keys` is `Vec<PayloadKey>` syntactically (could nest indefinitely), but **runtime enforcement walks ONE level deep** in this scope. A `type=object` key with declared sub-keys is verified for sub-key presence + type at level 1; sub-keys-of-sub-keys are accepted but not enforced. Defer deeper recursion to a future cycle if real drift surfaces.

Rationale: cycle 144-148 prompts use one level of nesting (e.g., `per-role-tasks > executor`). The two-level case has not yet appeared. Adding recursion now adds enforcement code without exercising it; KISS.

### 4.4 Optional-key handling

`PayloadKey` is mandatory by name (required-keys). For optional fields (telemetry that may not be present every cycle, e.g., `dispatches-fired`), add a sibling list `Channel::optional_payload_keys() -> &'static [&'static PayloadKey]`. The router checks present-but-optional keys for type only (does not reject if absent).

### 4.5 Null vs missing

A key with value `null` is treated as **present and of type `null`** — does NOT satisfy a `string` or `object` requirement. This is stricter than JSON-Schema-default but matches the project's prefer-explicit discipline.

### 4.6 Why not JSON Schema

Tempting: adopt JSON Schema dialect, get a mature validator, future-proof for richer constraints. Rejected because:

- JSON Schema is a dialect (drafts 4 / 6 / 7 / 2019-09 / 2020-12); committing to one is a project decision with maintenance tail.
- Validator crates (`jsonschema`) are substantial dependencies relative to the current `serde_json`-only footprint.
- The contract surface is bounded (4 channels, ~10 keys total). The in-house vocabulary fits in ~50 LOC; a JSON-Schema integration would dwarf the surface it validates.
- Migrating to JSON Schema later is straightforward: the `PayloadKey` descriptor maps to a JSON Schema document mechanically.

If the surface grows (10+ channels, richer types), revisit. Not before.

## 5. Enforcement points

### 5.1 Write-time (runtime)

`validate_payload` already walks `required_payload_keys()` at write-time. Extend it to walk `payload_schema()` instead:

1. Confirm payload is an object (existing check).
2. For each `PayloadKey` in `payload_schema().required`, confirm presence (existing) AND type-check the value against `key.ty` (NEW).
3. For each `type=object` key with non-empty `sub_keys`, confirm each declared sub-key is present in the value-object (NEW; level-1 recursion).
4. For each `PayloadKey` in `payload_schema().optional`, if present, type-check only (NEW).

Failure returns `RouterError::InvalidPayload` with a precise message naming the key + expected type + observed type.

### 5.2 Static (v2-prompt-contract-check extension)

Today, the tool compares the prompt-XML key-name set against `Channel::required_payload_keys()`. Extend the comparison:

1. Verify the type declared in `<required-key type="...">` matches `Channel::payload_schema()` for the same key.
2. For `type=object` keys with `<sub-key>` children, verify the sub-key NAME set matches `PayloadKey::sub_keys` (the v2-prompt-contract-check parser already extracts sub-keys — confirmed by the cycle 147 `parse_output_contract_handles_nested_subkeys_correctly` test at `tools/rust/crates/v2-prompt-contract-check/src/main.rs:870`).
3. Surface mismatches with the same per-prompt + per-channel mismatch report shape the tool uses today.

### 5.3 Schema subcommand

`v2-channel-router schema --format json` today emits flat `required_payload_keys`. Extend the `SchemaOutput` struct to include `type` + `sub_keys` (recursive struct). Bump `schema_format_version` field (add if not present) to `2`. v2-prompt-contract-check's parser must handle format version 2.

### 5.4 Mode flag (lenient cycle-1)

Add `--mode strict|lenient` (default `strict`). In `lenient`, `validate_payload` LOGS the type-mismatch + missing-sub-key cases to stderr but ALLOWS the write. In `strict`, those failures reject the write. Lenient mode is for the first cycle or two post-extension to surface real-world cases before switching to strict.

Alternative considered: phased rollout via separate `validate_payload_strict` flag. Rejected — flag plumbing is less honest than a documented mode. CLI flag is the right surface.

## 6. Migration plan

### 6.1 Cycle 165+ priority #1 (implementation)

Build the extension per this scope: extend channel-router (`PayloadSchema`, `payload_schema()`, extended `validate_payload`, extended `run_schema`, `--mode` flag), extend v2-prompt-contract-check (type + sub-key comparison), add unit + integration tests. Target ~400-600 LOC across both crates plus tests.

### 6.2 Cycle 165+ priority #2 (honesty-pass on prompts)

Cycle 148 L3.2 named the cargo-culted overclaims. Re-walk all 4 v2 prompts; align declared types with extended router enforcement. Cases to expect:

- Where the prompt overclaims (e.g., reconciler `:211-213` says non-array values cause rejection — currently false): EITHER fix the prompt to honest claim OR upgrade router enforcement to match (decision per case).
- Where the prompt declares a type the router can now enforce: no change needed (it now does).

### 6.3 Cycle 165+ priority #3 (run `v2-prompt-contract-check --strict` post-extension)

After implementation + honesty-pass, the tool should report 4/4 prompts contract-aligned at the NEW richer comparison level (types + sub-keys, not just names). Any remaining mismatch indicates either prompt OR router needs a final reconciliation pass.

### 6.4 Phased rollout

1. Cycle N: implement + tests; channel-router defaults to `--mode lenient` for one cycle.
2. Cycle N+1: run lenient against real cycle traffic; observe any unexpected type mismatches in stderr.
3. Cycle N+2: switch default to `--mode strict`. Honesty-pass on prompts has landed by now.
4. Cycle N+3+: remove `--mode lenient` from defaults; keep flag as escape hatch.

## 7. Ordering vs other v2-* tools / prompt-iteration

### 7.1 Relative to v2-prompt-contract-check

v2-channel-router exposes the schema; v2-prompt-contract-check reads it. The router extension must land FIRST so the contract-check extension has a schema-format-v2 to compare against. Otherwise the contract-check would silently downgrade to format-v1 comparison.

### 7.2 Relative to role prompts

The router extension must land BEFORE the honesty-pass on prompts. The pass reads the extended router schema to know what's now enforceable; without that, the honesty-pass is operating against stale guarantees.

### 7.3 Relative to `v2-prompt-tag-semantic-fidelity` (cycle 165+ priority #5 / L2.5)

The semantic-fidelity tool checks XML tag-name agreement with content-meaning ("if a tag is named `forward-notes`, the content must actually be forward-looking notes, not a generic dump"). It is orthogonal to this scope (which checks structural type agreement). Both tools can land independently; neither blocks the other.

### 7.4 Relative to live cycle traffic

This extension increases write-time validation work per channel write. Estimated cost: O(K) where K is required-key-count for the channel, currently ≤ 5 per channel. Sub-second per cycle. Not a hot path; not a concern.

## 8. Forward priorities produced by this scope

1. **v2-channel-router enforcement extension implementation** (cycle 165+): build `PayloadSchema`, `payload_schema()`, extended `validate_payload`, extended `run_schema`, `--mode` flag. ~250-400 LOC channel-router-side.
2. **v2-prompt-contract-check extension implementation** (cycle 165+, sequenceable with #1): extend prompt-XML comparison to include types + sub-keys; bump expected schema-format-version. ~100-200 LOC contract-check-side.
3. **Honesty-pass on v2 role prompts** (cycle 165+ prompt-iteration cycle): align each declared type with what the extended router enforces; fix cycle 148 L3.2 cargo-culted overclaims.
4. **`v2-prompt-contract-check --strict` re-run post-extension** (cycle 165+ verification): confirm 4/4 prompts contract-aligned at the extended comparison level.
5. **Deprecate `Channel::required_payload_keys()`** (cycle 166+, post-migration): once all callers use `payload_schema()`, remove the legacy method.

## 9. Anti-patterns this scope rejects

### 9.1 Coupling router enforcement with prompt-static-validation

Tempting: have v2-channel-router shell to v2-prompt-contract-check at runtime to verify the prompt-XML before each write. Rejected: the router is the runtime gatekeeper; the static check is operator-time. Mixing them couples two distinct concerns and adds startup cost. The static check runs at cycle-start or in CI, separately.

### 9.2 Adopting JSON Schema for a 10-key surface

Discussed in §4.6. The maintenance and dependency cost dwarfs the surface. Revisit when the surface grows.

### 9.3 Enforcing element-types in arrays

Tempting: "array of strings" vs "array of objects" enforcement. Rejected in this scope: deferred to a future cycle if real drift surfaces. The prompts today declare `type="array"` without per-element constraints; adding per-element schemas in the router would outrun the prompts.

### 9.4 Strict-mode-from-day-one rollout

Tempting: ship the extension in strict mode immediately. Rejected: cycle 144-156 channel writes against the current lenient validation may contain type mismatches the system has been silently tolerating. Strict-from-day-one would break the first cycle post-extension. Lenient cycle 1 surfaces the cases before they're rejection events.

### 9.5 Deep-nesting recursion in this scope

Tempting: full recursive type-checking of sub-keys-of-sub-keys. Rejected in §4.3. One level of nesting matches today's prompts; deeper recursion is YAGNI until two-level nesting appears.

## 10. What this scope does NOT do

1. Does NOT build the extension this cycle (cycle 165+ priorities #1-2 above).
2. Does NOT modify any v2 role prompt this cycle (cycle 165+ priority #3 above).
3. Does NOT re-run v2-prompt-contract-check this cycle (cycle 165+ priority #4 above).
4. Does NOT adopt JSON Schema dialect support (§4.6 — bounded surface).
5. Does NOT enforce array element-types (§2.2, §9.3).
6. Does NOT enforce string formats / regex / enums (§2.2).
7. Does NOT touch v1 channel infrastructure (frozen zone).
8. Does NOT design the eventual deprecation timing for `required_payload_keys()` (cycle 166+ migration cycle handles).
9. Does NOT design a richer schema-loader for external files (Option C in §3.3, rejected).
10. Does NOT couple channel-router with prompt-XML parsing (§9.1, §3.4).

## 11. References

- Cycle 146 design scope (predecessor for the static-check side): [`cycle-146-prompt-contract-check-design.md`](cycle-146-prompt-contract-check-design.md).
- Cycle 148 C1 / L3.1 critique verdicts naming the gap: [`cycle-148-two-track-absorption-and-landing.md`](cycle-148-two-track-absorption-and-landing.md) (search for "C1: " and "L3.1: ").
- Cycle 148 L3.2 cargo-culted overclaims naming the honesty-pass cases: same file, L3.2 section.
- v2-channel-router current source: [`tools/rust/crates/v2-channel-router/src/main.rs`](../../../tools/rust/crates/v2-channel-router/src/main.rs) particularly:
  - `Channel::required_payload_keys()` at line 117 (current flat-keys API)
  - `validate_payload` at line 367 (current write-time enforcement)
  - `SchemaOutput` at line 431 (current schema subcommand output struct)
  - `run_schema` at line 629 (current schema subcommand)
- v2-prompt-contract-check current source: [`tools/rust/crates/v2-prompt-contract-check/src/main.rs`](../../../tools/rust/crates/v2-prompt-contract-check/src/main.rs) particularly:
  - `parse_output_contract_handles_nested_subkeys_correctly` at line 870 (already parses sub-keys — comparison side is the gap)
- Cycle 147 dispatch precedent (similar shape): [#2952](https://github.com/EvaLok/schema-org-json-ld/issues/2952).
- Eva directive #2937 Copilot-dispatch leverage: [#2937](https://github.com/EvaLok/schema-org-json-ld/issues/2937) — the cycle 164 implementation of priorities #1-2 above is a strong dispatch candidate (pre-scoped via this document, precedent crates available).
