# Implement `v2-channel-router enforcement extension` + `v2-prompt-contract-check` extension (cycle 166 dispatch per directive #2937)

> **[main-orchestrator dispatch — implementation]**

This is an implementation Copilot dispatch. **Extend** two existing crates per the cycle 164 design scope. Both extensions land in a single PR so the schema-format-version bump is atomic.

## Context: where this fits

The schema-org-json-ld project is in a multi-cycle redesign of its orchestrator pipeline. The `v2-channel-router` crate (cycle 144, 869 LOC, in production) is the runtime gatekeeper for channel writes. It currently enforces **flat required-key names only** — no type-checking, no nested sub-key validation. The `v2-prompt-contract-check` crate (cycle 147, 1010 LOC, in production) statically verifies that prompt-XML `<required-key>` name sets match `Channel::required_payload_keys()` — but it only compares **names**, not types or sub-keys.

The cycle 148 C1 / L3.1 critique verdict was AGREE-RECORD + TOOL-SCOPED: *"PR #2953 (landing cycle 148) addresses the EASIEST layer (key-set alignment) but does not enforce types or nested shapes. The harder enforcement gap is the deepest valid critique in this lens."*

Cycle 164 (`docs/redesign/_notes/v2-channel-router-enforcement-extension.md`, 279 lines) captured the architecture decisions. **This dispatch builds the dual extension per that scope.**

The extension covers TWO crates because they share a schema contract: channel-router publishes the richer schema; prompt-contract-check reads it for static verification. Single PR ensures the bumped schema-format-version stays consistent across both sides.

## Read these documents (in this order, end-to-end)

1. **[`docs/redesign/_notes/v2-channel-router-enforcement-extension.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/_notes/v2-channel-router-enforcement-extension.md)** — full design scope (279 lines). **THE AUTHORITATIVE SPECIFICATION for this work.** Read end-to-end. Particularly:
   - §2 Tool boundary (in-scope / out-of-scope — both crates, not a new crate)
   - §3 Schema source-of-truth (Option B recommended: new `payload_schema()` method, retain `required_payload_keys()` as thin wrapper)
   - §4 Type system (in-house vocabulary, primitive types, `PayloadKey` descriptor, one-level recursion, null-vs-missing, why-not-JSON-Schema)
   - §5 Enforcement points (write-time, static, schema subcommand, `--mode strict|lenient`)
   - §6 Migration plan (this dispatch is §6.1, the implementation)
   - §7 Ordering vs other v2-* tools (router lands first within the PR; the honesty-pass on prompts is a SEPARATE future cycle, NOT this dispatch)
   - §9 Anti-patterns this scope rejects (5 explicit anti-patterns to avoid — re-read before designing)
   - §10 "What this scope does NOT do" (10 explicit non-doings for THE DESIGN SCOPE — your implementation honors them by sticking to scope)

2. **[`tools/rust/crates/v2-channel-router/src/main.rs`](https://github.com/EvaLok/schema-org-json-ld/blob/master/tools/rust/crates/v2-channel-router/src/main.rs)** — the crate to extend. Particularly:
   - `Channel` enum at line ~80 — your new `payload_schema()` method is added here
   - `Channel::required_payload_keys()` at line ~117 — becomes a thin wrapper that derives flat names from the new schema
   - `validate_payload` at line 367-385 — your extended logic lives here
   - `SchemaOutput` at line ~431 — gains `type` + `sub_keys` fields (recursive)
   - `run_schema` at line ~629 — emits the extended schema
   - `RouterError::InvalidPayload` — keep the existing variant; failure messages get richer

3. **[`tools/rust/crates/v2-prompt-contract-check/src/main.rs`](https://github.com/EvaLok/schema-org-json-ld/blob/master/tools/rust/crates/v2-prompt-contract-check/src/main.rs)** — the crate to extend. Particularly:
   - `parse_output_contract_handles_nested_subkeys_correctly` at line 870 — confirms the parser ALREADY extracts sub-keys + types. Comparison side is the gap to close.
   - Current comparison loop walks the flat `required_payload_keys` set against the parsed prompt key set. Extend it to walk the new `payload_schema` structure, comparing type per key and sub-key set per object-typed key.
   - Per-prompt + per-channel mismatch report shape stays the same; new mismatch categories (`type_mismatch`, `sub_key_mismatch`) join the existing `missing_key` / `extra_key` categories.

4. **[`tools/rust/Cargo.toml`](https://github.com/EvaLok/schema-org-json-ld/blob/master/tools/rust/Cargo.toml)** — workspace Cargo.toml. No change needed (both crates already members).

5. **[`docs/redesign/_notes/cycle-148-two-track-absorption-and-landing.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/_notes/cycle-148-two-track-absorption-and-landing.md)** — search for "C1:" and "L3.1:" for the original critique verdicts establishing the gap. Context only; do not act on L3.2 (honesty-pass on prompts) in this dispatch.

6. **[`tools/rust/crates/v2-state-audit/src/main.rs`](https://github.com/EvaLok/schema-org-json-ld/blob/master/tools/rust/crates/v2-state-audit/src/main.rs)** — recent precedent for a v2-* crate (1260 LOC). Use the same shape conventions: single `src/main.rs` file, inline `#[cfg(test)] mod tests` at the bottom, clap derive, structured serde types, conservative error handling.

## What to build

### Crate 1: `tools/rust/crates/v2-channel-router/src/main.rs` extensions

#### Add `PayloadType` enum + `PayloadKey` struct (design scope §4.2)

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PayloadType {
    String,
    Integer,
    Number,
    Boolean,
    Array,    // element-type NOT enforced this scope (§2.2, §9.3)
    Object,
    Null,
}

#[derive(Debug, Clone, Copy)]
pub struct PayloadKey {
    pub name: &'static str,
    pub ty: PayloadType,
    pub sub_keys: &'static [PayloadKey],   // empty unless ty == Object
}

#[derive(Debug, Clone, Copy)]
pub struct PayloadSchema {
    pub required: &'static [PayloadKey],
    pub optional: &'static [PayloadKey],
}
```

Notes:
- `PayloadKey::sub_keys` is `&'static [PayloadKey]` (not Vec) so the descriptor remains a `&'static` table (no allocation, matching the current `required_payload_keys()` discipline).
- `Serialize` on `PayloadKey` / `PayloadSchema` produces the JSON shape for the schema subcommand (`run_schema`). Implement manually or use serde's `Serialize` derive with appropriate field-renames so the JSON has `name`, `type`, `sub_keys`.

#### Add `Channel::payload_schema()` method (§3.2 Option B, recommended)

```rust
impl Channel {
    pub fn payload_schema(&self) -> &'static PayloadSchema {
        match self {
            Channel::ForwardNotes => &FORWARD_NOTES_SCHEMA,
            Channel::CycleTrace   => &CYCLE_TRACE_SCHEMA,
            // ... one match arm per channel variant
        }
    }

    pub fn required_payload_keys(&self) -> &'static [&'static str] {
        // Retain as thin wrapper that returns the flat name set computed from payload_schema().
        // For each channel, define a `static REQUIRED_NAMES_<CHANNEL>: &[&str] = &[...]` constant
        // that mirrors the schema's required-key names. The two MUST agree (enforce in tests).
        match self {
            Channel::ForwardNotes => REQUIRED_NAMES_FORWARD_NOTES,
            // ... etc
        }
    }
}
```

Per-channel schema constants (one per channel variant; consult the existing `required_payload_keys()` match arms and the cycle 164 design scope §1.2 examples for shapes):

```rust
static FORWARD_NOTES_SCHEMA: PayloadSchema = PayloadSchema {
    required: &[
        PayloadKey { name: "substantive-focal",   ty: PayloadType::String, sub_keys: &[] },
        PayloadKey { name: "per-role-tasks",      ty: PayloadType::Object, sub_keys: &[
            PayloadKey { name: "executor", ty: PayloadType::String, sub_keys: &[] },
            PayloadKey { name: "curator",  ty: PayloadType::String, sub_keys: &[] },
        ] },
        // ... per the cycle 164 §1.2 enumeration
    ],
    optional: &[],
};
```

**Important:** Derive each channel's schema from the actual prompt declarations (read `prompts/v2/<role>.xml` for each role; the `<output-contract>` `<channel>` `<format>` `<required-key type="...">` declarations are the source-of-truth for the schema you define on the router side). The cycle 164 design scope §1.2 gives partial examples; complete the enumeration by reading the prompts.

The 4 prompts under `prompts/v2/`:
- `planner.xml`
- `reconciler.xml`
- `executor.xml`
- `curator.xml`

For each prompt, walk every `<output-contract>` → `<channel>` → `<format>` → `<required-key>` block and translate to a `PayloadKey` entry. Sub-keys (`<sub-key>`) translate to nested `PayloadKey`s with one level of nesting.

#### Extend `validate_payload` (design scope §5.1)

Existing checks:
1. Payload is a JSON object (keep).
2. Each `required_payload_keys()` entry exists in the payload object (keep, but now derive from `payload_schema().required` names).

Add:
3. For each `PayloadKey` in `payload_schema().required`, type-check the value against `key.ty`:
   - `PayloadType::String` → `serde_json::Value::String`
   - `PayloadType::Integer` → `serde_json::Value::Number` with `.is_i64()` or `.is_u64()`
   - `PayloadType::Number` → `serde_json::Value::Number`
   - `PayloadType::Boolean` → `serde_json::Value::Bool`
   - `PayloadType::Array` → `serde_json::Value::Array`
   - `PayloadType::Object` → `serde_json::Value::Object`
   - `PayloadType::Null` → `serde_json::Value::Null`
4. For each `PayloadKey` with `ty == Object` and non-empty `sub_keys`, walk the value-object and verify each declared sub-key is present (§4.3 — one-level recursion only; sub-keys-of-sub-keys are accepted but not enforced).
5. For each `PayloadKey` in `payload_schema().optional`, if present in payload, type-check only (do not reject if absent).

Failure path (per `--mode`):
- `--mode strict` (default): return `RouterError::InvalidPayload` with precise message `"payload key '<name>': expected type <expected>, observed <observed>"` or `"payload key '<name>' (type=object): missing required sub-key '<sub_name>'"`.
- `--mode lenient`: log the failure to stderr with prefix `[router-lenient] ` and ALLOW the write (return `Ok`).

#### Add `--mode strict|lenient` CLI flag (§5.4)

Add to the existing `Args` struct:

```rust
#[arg(long, value_enum, default_value_t = Mode::Strict)]
mode: Mode,

#[derive(clap::ValueEnum, Clone, Debug)]
enum Mode {
    Strict,
    Lenient,
}
```

Plumb `Mode` through to `validate_payload` (add as a parameter, or store on a context struct that wraps the existing state).

**Rationale for default = Strict:** the design scope §5.4 says lenient is for "cycle 1 or 2 post-extension to surface real-world cases before switching to strict." Default to strict; lenient is an explicit opt-in. The honesty-pass on prompts (future cycle) may temporarily flip channels to lenient to surface mismatches; that's an operator decision per cycle, not a default.

#### Extend `run_schema` + `SchemaOutput` (§5.3)

Current `SchemaOutput` emits flat `required_payload_keys`. Extend:

```rust
#[derive(Serialize)]
struct SchemaOutput {
    schema_format_version: u32,   // NEW: bump to 2
    channels: Vec<ChannelSchema>,
}

#[derive(Serialize)]
struct ChannelSchema {
    name: String,
    payload_schema: PayloadSchemaJson,
}

#[derive(Serialize)]
struct PayloadSchemaJson {
    required: Vec<PayloadKeyJson>,
    optional: Vec<PayloadKeyJson>,
}

#[derive(Serialize)]
struct PayloadKeyJson {
    name: String,
    #[serde(rename = "type")]
    ty: PayloadType,                  // serializes lowercase per derive
    sub_keys: Vec<PayloadKeyJson>,    // recursive serialization; one level deep in practice
}
```

`schema_format_version: 2` is the bumped value. v2-prompt-contract-check must read this version field and refuse comparison if version != 2 (with a precise error pointing operator at the version drift).

### Crate 2: `tools/rust/crates/v2-prompt-contract-check/src/main.rs` extensions

#### Schema-format-version handling

At the top of the comparison pipeline, after calling `v2-channel-router schema --format json` (or whatever the current invocation pattern is — check the existing source), parse `schema_format_version`:

```rust
const EXPECTED_SCHEMA_FORMAT_VERSION: u32 = 2;

// after parsing schema JSON:
if schema_output.schema_format_version != EXPECTED_SCHEMA_FORMAT_VERSION {
    eprintln!(
        "[prompt-contract-check] schema-format-version mismatch: \
         expected {EXPECTED_SCHEMA_FORMAT_VERSION}, got {} from v2-channel-router. \
         Re-build v2-channel-router and re-run.",
        schema_output.schema_format_version
    );
    return ExitCode::from(2);
}
```

#### Extend the comparison loop (§5.2)

Today the loop walks per-prompt × per-channel and reports:
- `missing_key`: prompt declares a key the router doesn't require
- `extra_key`: router requires a key the prompt doesn't declare

Add:
- `type_mismatch`: prompt declares `type="X"` for key `K`, router schema declares `type="Y"` for same key
- `sub_key_missing_in_prompt`: router schema declares `sub_keys=[a, b, c]` for key `K`, prompt declares `<sub-key>` set is a strict subset
- `sub_key_missing_in_router`: prompt declares `<sub-key>` set is a strict superset of router schema sub_keys
- `sub_key_type_mismatch`: same as `type_mismatch` but for a sub-key

Use the existing parsed `<required-key type="...">` and `<sub-key type="...">` values from the prompt-XML parser (already extracted per `parse_output_contract_handles_nested_subkeys_correctly` at line 870).

The per-prompt + per-channel mismatch report shape stays the same; new categories join `missing_key` / `extra_key`. Total mismatch count includes the new categories.

### Tests

#### Unit tests (inline in each src/main.rs)

In `v2-channel-router/src/main.rs`:
- `payload_schema_required_names_agree_with_required_payload_keys`: for every Channel variant, the flat names from `required_payload_keys()` MUST be the same set as the names in `payload_schema().required`. Critical invariant.
- `validate_payload_rejects_string_when_object_expected`: declare a schema with `type=object` key, send payload with string value, assert `RouterError::InvalidPayload`.
- `validate_payload_rejects_missing_sub_key`: declare object key with required sub_keys, send payload with object missing a sub_key, assert rejection.
- `validate_payload_accepts_present_optional_with_correct_type`.
- `validate_payload_ignores_absent_optional`.
- `validate_payload_lenient_mode_logs_but_accepts`: assert stderr contains `[router-lenient]` prefix and the write succeeds.
- `validate_payload_null_does_not_satisfy_string`: per §4.5, `null` does NOT satisfy a `string` requirement.
- `validate_payload_one_level_recursion_only`: declare object→object nesting; send payload with the outer object correctly typed but inner object missing a sub-key-of-sub-key. Assert the inner missing sub-key-of-sub-key is NOT enforced (per §4.3 — one level only).
- `schema_output_serializes_format_version_2`.

In `v2-prompt-contract-check/src/main.rs`:
- `comparison_detects_type_mismatch`: stub a schema declaring `string`, stub a prompt declaring `object`, assert `type_mismatch` reported.
- `comparison_detects_sub_key_set_drift`: stub a schema with sub_keys `[a, b]`, stub a prompt with sub_keys `[a, c]`, assert both `sub_key_missing_in_prompt` (b) and `sub_key_missing_in_router` (c).
- `comparison_handles_schema_format_version_2`.
- `comparison_rejects_schema_format_version_1`: assert exit 2 with helpful stderr.

#### Integration tests

`tools/rust/crates/v2-channel-router/tests/integration.rs` (extend existing):
- `validate_payload_strict_mode_default_rejects_type_mismatch`: invoke the binary, write a deliberately-mistyped payload, assert exit non-zero + stderr names the key + expected/observed types.
- `validate_payload_lenient_mode_logs_and_accepts`: same payload, `--mode lenient`, assert exit 0 + stderr has lenient log.
- `schema_subcommand_emits_format_version_2`.

`tools/rust/crates/v2-prompt-contract-check/tests/integration.rs` (extend existing):
- `end_to_end_comparison_with_synthetic_schema_v2_and_synthetic_prompts`: stand up a temp prompt + a captured schema JSON, run the binary, assert the mismatch report includes the new categories.

### Specific behaviors (invariants)

1. **One source of truth per channel.** `Channel::payload_schema()` is the authoritative schema; `required_payload_keys()` derives flat names from it. Test that the two are in lockstep for every channel variant.
2. **One-level recursion only.** A `type=object` key's `sub_keys` are enforced at level 1; sub-keys-of-sub-keys are accepted but not enforced. Per §4.3.
3. **Strict is the default.** Lenient is an explicit CLI opt-in; production deployments default to strict.
4. **Schema-format-version bump is atomic with the comparison-loop extension.** Router emits version 2; contract-check expects version 2. Single PR.
5. **Null does not satisfy non-null types.** Per §4.5.
6. **No prompt files are modified by this dispatch.** Honesty-pass on prompts is a separate future cycle (§6.2). Your work is router + contract-check only.
7. **No new crates.** Both extensions are local to existing crates.

### Anti-patterns to avoid (cycle 164 design scope §9, transcribed)

1. **DO NOT couple router enforcement with prompt-static-validation at runtime.** The router does not shell to v2-prompt-contract-check at write-time. Static check runs at cycle-start or in CI, separately. (§9.1)
2. **DO NOT adopt JSON Schema dialect.** Use the minimal in-house vocabulary (`PayloadType` enum + `PayloadKey` struct). The contract surface is bounded (4 channels, ~10 keys total); a JSON-Schema dependency would dwarf the surface. (§9.2 / §4.6)
3. **DO NOT enforce element-types within arrays.** "Array of strings" vs "array of objects" enforcement is deferred. Today's prompts declare `type="array"` without per-element constraints; do not outrun them. (§9.3)
4. **DO NOT default to strict mode in this dispatch's first commit** — wait, the design scope §5.4 actually says default IS strict, with lenient as the opt-in for the cycle 1 or 2 post-extension when the operator chooses. Re-read §5.4 carefully: "Add `--mode strict|lenient` (default `strict`)." The lenient cycle 1 is OPERATOR-DRIVEN, not a default-mode quirk. Default strict is correct.
5. **DO NOT add deep-nesting recursion.** One level matches today's prompts. YAGNI for two-level+. (§9.5)
6. **DO NOT modify the v2 role prompts.** Honesty-pass on prompts is cycle 165+ priority #2 — a SEPARATE future cycle. This dispatch is router + contract-check ONLY. (§2.2, §10)
7. **DO NOT modify v1 prompts/tools** (frozen zone — `tools/cycle-runner/`, `tools/record-dispatch`, etc.).

### What's already done

- Cycle 164 design scope: `docs/redesign/_notes/v2-channel-router-enforcement-extension.md` (279 lines, authoritative).
- Cycle 148 critique verdicts (C1 + L3.1) naming the enforcement gap.
- Cycle 147 v2-prompt-contract-check (1010 LOC, in production; flat name comparison + parser that already extracts types + sub-keys per the cycle 147 test at line 870).
- Cycle 144 v2-channel-router (869 LOC, in production; flat-key validation).
- 4 v2 role prompts (`prompts/v2/{planner,reconciler,executor,curator}.xml`) declaring `<required-key type="...">` + `<sub-key>` shapes. Frozen reference for the schema-content side of this work.

### What to skip / do NOT do (explicit non-doings)

1. Do NOT modify any v2 role prompt under `prompts/v2/`. Honesty-pass on prompts is a SEPARATE cycle.
2. Do NOT modify `Channel::required_payload_keys()` to return a different signature (Option A in §3.1 was rejected). Retain as `&'static [&'static str]` wrapper that derives from `payload_schema()`.
3. Do NOT introduce external JSON schema files (Option C in §3.3 rejected).
4. Do NOT have channel-router parse prompt-XML (Option D in §3.4 rejected).
5. Do NOT add full JSON Schema dialect dependency (§4.6 / §9.2 rejected).
6. Do NOT enforce element-types in arrays (§2.2, §9.3 rejected).
7. Do NOT enforce string formats / regex / enums (§2.2).
8. Do NOT touch `v2-cycle-runner`, `v2-state-audit`, `v2-state-dispatch-archive`, `v2-state-dispatch-sync`, `v2-dispatch-status`, or any other v2-* tool.
9. Do NOT touch legacy v1 tools (`tools/cycle-runner/`, `tools/record-dispatch`, `tools/rust/crates/rebase-pr/`, etc. — frozen).
10. Do NOT touch `.github/workflows/`.
11. Do NOT deprecate `Channel::required_payload_keys()` in this PR (that's cycle 166+ migration cycle).
12. Do NOT add deeper-than-one-level recursion enforcement (§4.3 / §9.5 rejected).

### Verify before opening the PR

Run these commands in the PR branch:

```bash
cargo build --manifest-path tools/rust/Cargo.toml -p v2-channel-router
cargo build --manifest-path tools/rust/Cargo.toml -p v2-prompt-contract-check
cargo test  --manifest-path tools/rust/Cargo.toml -p v2-channel-router
cargo test  --manifest-path tools/rust/Cargo.toml -p v2-prompt-contract-check
cargo clippy --manifest-path tools/rust/Cargo.toml -p v2-channel-router        --all-targets
cargo clippy --manifest-path tools/rust/Cargo.toml -p v2-prompt-contract-check --all-targets
```

All must pass clean (no warnings). The workspace-wide `cargo test` should also pass (run `cargo test --manifest-path tools/rust/Cargo.toml` as a regression check).

End-to-end verification:

```bash
# Schema subcommand emits version 2:
cargo run --manifest-path tools/rust/Cargo.toml --release -p v2-channel-router -- schema --format json | jq .schema_format_version
# expected: 2

# Contract check runs and reports against the extended schema:
cargo run --manifest-path tools/rust/Cargo.toml --release -p v2-prompt-contract-check -- --prompts-dir prompts/v2 --json
# expected: 0 exit if all 4 prompts align at the extended comparison level; non-zero with detailed report if any prompt declares a type the router schema disagrees with (likely some — that's information for the honesty-pass cycle, NOT a blocker for this PR).
```

If the end-to-end contract check surfaces drift between prompts and the new router schema, that's expected and is the WHOLE POINT of the extension — the drift was previously invisible. Capture the report in the PR description as a "drift surfaced; cycle 165+ priority #2 honesty-pass on prompts will resolve" note. DO NOT modify the prompts to silence the report.

### Direct-push-zones note

This work is in `tools/rust/crates/v2-channel-router/` and `tools/rust/crates/v2-prompt-contract-check/` — both are within `tools/rust/crates/v2-*` per `AUTHORITY.direct-push-zones`. Open the PR as Copilot's standard PR-from-draft workflow; main orchestrator will review per the cycle 165 PR #2975 absorption-shape (per-finding verdict ledger against the design scope), then merge with `gh pr merge --merge --delete-branch --admin` if the verdict ledger is clean.

The PR should NOT need Eva-merge — direct-push-zone files only. Single PR for both crate extensions to keep the schema-format-version bump atomic.

## Provenance

- Cycle 166 dispatch under directive [#2937](https://github.com/EvaLok/schema-org-json-ld/issues/2937) (Copilot leverage). 2nd Role 2 instance (`Copilot-as-implementer`) after cycle 164 PR #2974 → #2975 — would HARDEN cross-cycle pattern `directive-2937-copilot-role-bifurcation` to RECURRENCE-AT-2.
- Design scope: cycle 164 `docs/redesign/_notes/v2-channel-router-enforcement-extension.md` (279 lines).
- Predecessor crates: v2-channel-router (cycle 144, 869 LOC), v2-prompt-contract-check (cycle 147, 1010 LOC), v2-state-audit (cycle 159, 1260 LOC; shape model for v2-* crates).
- Predecessor critique: cycle 148 C1 / L3.1 verdict from PR #2877 adversarial-critique dispatch.
