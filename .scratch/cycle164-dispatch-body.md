# Implement `v2-state-dispatch-archive` (cycle 164 dispatch per directive #2937 item #1)

> **[main-orchestrator dispatch — implementation]**

This is an implementation Copilot dispatch. Author a new Rust crate at `tools/rust/crates/v2-state-dispatch-archive/` per the cycle 163 design scope. The crate is the **archival counterpart** to `v2-state-audit` (cycle 159, in production at 1260 LOC) — audit reports, archive acts.

## Context: where this fits

The schema-org-json-ld project is in a multi-cycle redesign of its orchestrator pipeline. The v2 state surface is governed by `docs/redesign/_notes/v2-state-retention-policy.md` (cycle 158, 6 axes). Axis 6 is `docs/state.json` `agent_sessions[]` — currently 930 entries (922 terminal, 3 live, 5 other-status), well above the policy's hard threshold (500). Cycle 162 wired `v2-state-audit` at `v2-cycle-runner` pre-flight session-start with halt on `Hard` severity, but the wiring is fail-open against the existing breach.

Cycle 161 design scope (`docs/redesign/_notes/v2-state-dispatch-policy-enforcement.md`) §4.2 selected **Option B** — periodic age-based archival sweep, separate tool — as the path to bring `agent_sessions[]` within the policy's bounded retention window. Cycle 163 (`docs/redesign/_notes/v2-state-dispatch-archive.md`, 260 lines) captured the architecture decisions. **This dispatch builds the crate per that scope.**

## Read these documents (in this order, end-to-end)

1. **[`docs/redesign/_notes/v2-state-dispatch-archive.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/_notes/v2-state-dispatch-archive.md)** — full design scope (260 lines). **THE AUTHORITATIVE SPECIFICATION for this work.** Read end-to-end. Particularly:
   - §2 Tool boundary (in-scope / out-of-scope discipline)
   - §3 Sweep criteria (status whitelist + age threshold + missing-timestamp fallback + selection invariant)
   - §4 Archive file schema (`archive_version: 1`, dated filename, byte-for-byte preservation, append on same day)
   - §5 Invocation contract (CLI flags, exit codes, JSON output schema)
   - §6 Atomicity and recovery (step ordering, concurrent-mutation detection via hash, lock file)
   - §7 Ordering vs other v2-state-* tools
   - §9 Anti-patterns this scope rejects (re-read before designing — explicit anti-patterns to avoid)
   - §10 "What this scope does NOT do" (10 explicit non-doings for THE DESIGN SCOPE — your implementation honors them by sticking to scope)

2. **[`tools/rust/crates/v2-state-audit/src/main.rs`](https://github.com/EvaLok/schema-org-json-ld/blob/master/tools/rust/crates/v2-state-audit/src/main.rs)** — precedent crate (1260 LOC, single-file, in production cycle 159+). This is your shape model:
   - clap derive for CLI parsing
   - serde + serde_json for JSON I/O
   - Single `src/main.rs` file with inline `#[cfg(test)] mod tests` at the bottom
   - Read-only architecture; structured Report + Severity types serialized to JSON
   - Conservative error handling: missing files → zero/empty; bad parse → exit 2; bad flags → exit 1
   - Note: v2-state-audit does NOT mutate. v2-state-dispatch-archive DOES mutate. The §6 ordering discipline is the crucial difference.

3. **[`tools/rust/crates/v2-state-audit/Cargo.toml`](https://github.com/EvaLok/schema-org-json-ld/blob/master/tools/rust/crates/v2-state-audit/Cargo.toml)** — pattern for the new crate's Cargo.toml. Use the same dependency set (clap with `derive`, serde with `derive`, `serde_json` with `preserve_order` for byte-for-byte entry preservation). Add `sha2 = "0.10"` for §6.2 concurrent-mutation hash check.

4. **[`tools/rust/Cargo.toml`](https://github.com/EvaLok/schema-org-json-ld/blob/master/tools/rust/Cargo.toml)** — workspace Cargo.toml; `members = ["crates/*", "crates/rebase-pr"]` so adding `crates/v2-state-dispatch-archive/` auto-includes it. No change needed.

5. **[`docs/redesign/_notes/v2-state-retention-policy.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/_notes/v2-state-retention-policy.md)** §4 Axis 6 (sets context for thresholds 50/200/500; this tool acts when audit reports `mandatory` or `hard` against those thresholds). You do NOT modify this file.

6. **[`docs/redesign/_notes/v2-state-dispatch-policy-enforcement.md`](https://github.com/EvaLok/schema-org-json-ld/blob/master/docs/redesign/_notes/v2-state-dispatch-policy-enforcement.md)** §4.2 + §4.3 (predecessor scope that selected Option B and named the sequencing). Read for additional context on why this tool exists.

## What to build

### Crate layout

```
tools/rust/crates/v2-state-dispatch-archive/
├── Cargo.toml
├── src/
│   └── main.rs        # single file with inline tests
└── tests/
    └── integration.rs # 1 happy-path + 1 dry-run + 1 hash-mismatch integration test
```

### Cargo.toml

```toml
[package]
name = "v2-state-dispatch-archive"
version = "0.1.0"
edition = "2021"
description = "Periodic age-based archival sweep for docs/state.json agent_sessions[]. Moves terminal entries older than threshold to dated archive files under docs/state-archive/. Archival counterpart to v2-state-audit per cycle 163 design scope. CORE-DESIGN-PRINCIPLE-aligned tool extraction of v2-state-retention-policy §4 Axis 6 enforcement surface."

[dependencies]
clap = { version = "4", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = { version = "1", features = ["preserve_order"] }
sha2 = "0.10"

[dev-dependencies]
tempfile = "3"
```

### src/main.rs implementation requirements

Implement the CLI exactly per design scope §5.1:

```
v2-state-dispatch-archive [OPTIONS]

OPTIONS:
  --repo-root <PATH>         Repository root (default: ".")
  --age-days <N>             Archival age cutoff in days (default: 30)
  --status <CSV>             Terminal statuses to archive (default: all 5 terminal per §3.1)
  --max-entries <N>          Maximum entries to archive in this invocation (default: unbounded)
  --archive-dir <PATH>       Archive output directory (default: "docs/state-archive")
  --dry-run                  Plan only; no mutation
  --json                     Machine-readable JSON output to stdout
  -h, --help
```

**Status whitelist (§3.1):**

```rust
const TERMINAL_ARCHIVABLE: &[&str] = &[
    "merged", "failed", "closed_without_pr", "closed", "closed_without_merge",
];
const LIVE_NEVER_ARCHIVED: &[&str] = &[
    "in_flight", "reviewed_awaiting_eva",
];
// Unknown status values are treated as INELIGIBLE (fail-safe; design scope §3.1 invariant).
```

**Age computation (§3.2 / §3.3):**

For each entry, compute age in days from invocation time (`SystemTime::now()` in UTC):
1. Try `merged_at` field (RFC 3339 / ISO 8601). If missing/null/unparseable → step 2.
2. Try `dispatched_at` field. If missing/null/unparseable → entry is **NOT archived**; counted in the report's `ineligible.missing_timestamp` bucket.
3. Compare against `--age-days` cutoff (default 30). Entries strictly older than the cutoff are eligible.

**Archive file schema (§4):**

Filename: `docs/state-archive/dispatches-<YYYY-MM-DD>.json` where `<YYYY-MM-DD>` is the **invocation date (UTC)**, not the entry's date. Multiple sweeps same day **append** to the same file's `entries[]` array (§4.3); `archived_at` updates to latest, `criteria.invocation_id` becomes an array if multiple invocations recorded on same day.

Top-level structure (§4.2):
```json
{
  "archive_version": 1,
  "tool_version": "v2-state-dispatch-archive 0.1.0",
  "archived_at": "2026-05-17T...",
  "source_file": "docs/state.json",
  "criteria": {
    "age_days": 30,
    "status_filter": ["merged", "failed", ...],
    "invocation_id": "2026-05-17-cycle-N"  // OR array on multiple-same-day
  },
  "entries": [ ...verbatim copies of agent_sessions[] entries... ]
}
```

**Atomicity (§6.1) — strict step ordering:**

1. Read `docs/state.json` and parse fully into memory. Compute SHA-256 hash of `agent_sessions[]` (canonical JSON form: `serde_json::to_string` with `preserve_order` feature so field order is preserved as it appears on disk).
2. Compute archival-eligible set per §3 (status + age + max-entries).
3. If `--dry-run`, emit report and exit 0 (no writes).
4. Write archive file `docs/state-archive/dispatches-<YYYY-MM-DD>.json` (atomic via tempfile + rename; create directory if missing). Use `serde_json::to_writer_pretty`.
5. Re-read `docs/state.json`, recompute the hash of `agent_sessions[]`, compare against step 1 hash. **If hashes differ, abort with exit code 3** — preserves the archive file from step 4; the live state was not mutated.
6. If hashes match, write mutated `docs/state.json` (with archived entries removed from `agent_sessions[]`) via tempfile + rename. **PRESERVED-PRIMITIVE git-safety: do not handle git commits in this tool; the operator/script does that.**
7. Emit report and exit 0.

**Lock file (§6.3):**

Acquire an exclusive lock at `docs/state.json.lock` before step 1; hold until after step 6 (or step 3 in dry-run). If another process holds the lock, wait up to 30 seconds, then exit 2 with a clear error. Use `fs::OpenOptions::new().write(true).create_new(true).open(...)` for atomic acquire; release by deleting the file on drop. Note: v1 `record-dispatch` is frozen-zone and does NOT honor the lock — the hash check in step 5 is the collision protection for v1 vs v2 races.

**Exit codes (§5.2):**

| Code | Meaning |
|---|---|
| 0 | Success — sweep completed (possibly with zero entries archived) |
| 1 | Configuration error (bad flags, missing repo-root, etc.) |
| 2 | Input error (state.json unparseable, archive directory unwritable, lock-acquire timeout) |
| 3 | Concurrent-mutation error (state.json changed between step 1 and step 5) |
| 4 | Partial failure — archive file written but state.json mutation failed at step 6 |

**JSON output (§5.3):**

```json
{
  "tool": "v2-state-dispatch-archive",
  "tool_version": "0.1.0",
  "invoked_at": "2026-05-17T...",
  "dry_run": false,
  "archived_count": 819,
  "archive_file": "docs/state-archive/dispatches-2026-05-17.json",
  "ineligible": {
    "live_status": 3,
    "ineligible_status": 5,
    "below_age_threshold": 100,
    "missing_timestamp": 3
  },
  "state_after": {
    "agent_sessions_total": 111,
    "agent_sessions_live": 3,
    "agent_sessions_terminal_retained": 108
  }
}
```

Human-readable output (default, when `--json` not passed) writes a per-bucket summary to stdout (one line per bucket plus a final "archived N entries to <archive_file>" line). Use the v2-state-audit text-output style as a guide.

### Tests

**Inline unit tests** (`#[cfg(test)] mod tests` at the bottom of `src/main.rs`) — cover:

- Status whitelist enum + fallback for unknown statuses
- Age computation (RFC 3339 parse + day-diff math); covers `merged_at` present, `merged_at: null` falling back to `dispatched_at`, both missing
- `--status` flag CSV parsing
- `--age-days 0` behavior (archives all terminal regardless of age)
- `--max-entries` bound (selects oldest N, leaves rest)
- Filename date formatting (UTC invocation date)
- Hash computation (deterministic given same JSON input; differs when `agent_sessions[]` differs)

Aim for ~25-40 unit tests covering the policy invariants. Use the v2-state-audit inline-test style.

**Integration tests** (`tests/integration.rs`) — cover the binary against a tempfile-staged fake state.json + state-archive dir. Use `tempfile` crate. **Three tests minimum:**

1. **Happy-path archive sweep** — stage a state.json with 5 entries (3 terminal eligible by age, 1 terminal too-recent, 1 live). Run binary; assert exit 0; assert `dispatches-<TODAY>.json` exists in archive dir with 3 entries; assert state.json now has 2 entries.

2. **Dry-run** — same setup, but pass `--dry-run`. Assert exit 0; assert archive dir is empty (no file created); assert state.json byte-identical to before.

3. **Concurrent-mutation detection** — stage a state.json; arrange for the test to mutate state.json between the binary's step 1 and step 5 (one way: use a small helper binary or a wrapper that sleeps and modifies; alternatively, since this is hard to test deterministically without instrumentation, write the test as a behavioral test that **modifies state.json after archive runs** and verifies exit code 3 path via a hash-mismatch unit test on the hash function itself, with an integration smoke test that exercises the success path. **Prefer the deterministic unit-test approach for the hash mismatch + an integration test that exercises step-6 happy path; document in a comment that the live-race path is unit-tested deterministically rather than end-to-end.**)

### Specific behaviors to honor from design scope

1. **§3.1 unknown status is INELIGIBLE.** Do not silently treat an unknown status as eligible.
2. **§3.3 missing-both-timestamps is INELIGIBLE.** Report in `ineligible.missing_timestamp`. Do not fall back to file-system mtime or any other proxy.
3. **§4.1 invocation date, not entry date.** The filename uses today's UTC date, not the archived entry's date.
4. **§4.3 same-day append.** If the file already exists for today's date, parse it, append to `entries[]`, update `archived_at`, update `criteria.invocation_id` to an array of all invocations.
5. **§4.4 archive_version field.** Always write `archive_version: 1` (this is schema v1). When reading an existing same-day file, refuse if it has a higher version (forward-compatibility guard).
6. **§6.1 step-4-BEFORE-step-6 ordering is invariant.** If you implement it reversed, an archive failure could lose data — the design explicitly chose this ordering to make re-runs idempotent on partial failure.
7. **§6.2 hash check is mandatory in production path.** Skip-on-flag is NOT acceptable (no `--force` to bypass; we want to preserve audit-trail clarity).

## Anti-patterns to avoid (design scope §9)

- **Do NOT couple with v2-state-audit.** Tempting to have v2-state-audit invoke this tool when severity hits `mandatory`. Rejected: single-purpose discipline. Audit reports; archive acts.
- **Do NOT archive live entries.** `in_flight` and `reviewed_awaiting_eva` are operationally-live regardless of age. The §3.1 whitelist is invariant.
- **Do NOT do in-place compaction.** Archive files are append-immutable per §2.2; do not rewrite or compact them. Entries preserved byte-for-byte.
- **Do NOT use a single growing archive file.** Dated filenames bound per-file size; do not collapse into a forever-growing `dispatches.json`.
- **Do NOT modify any state.json field other than `agent_sessions[]`.** (§2.2 explicit non-doing.)
- **Do NOT shell out to git or other CLIs.** This tool is a pure file I/O CLI. No git operations.

## What's already done

- Design scope (`v2-state-dispatch-archive.md`, 260 lines) authored cycle 163.
- Precedent crate (`v2-state-audit`, 1260 LOC) in production since cycle 159.
- Workspace Cargo.toml already includes `crates/*` so the new crate auto-loads.
- v1 `record-dispatch` is frozen-zone; do not touch.
- v2-state-audit is in production (do not touch).
- v2-state-dispatch-sync exists (do not touch).
- `docs/state-archive/` directory does not yet exist; the tool creates it if missing.
- `docs/state.json` currently has 930 `agent_sessions[]` entries (922 terminal-eligible by status, 3 live, 5 other).

## Verify before opening the PR

```bash
cd tools/rust
cargo build -p v2-state-dispatch-archive
cargo test -p v2-state-dispatch-archive
cargo clippy -p v2-state-dispatch-archive -- -D warnings

# Smoke against real state.json in dry-run:
./target/debug/v2-state-dispatch-archive --repo-root ../.. --dry-run --json | jq .
```

The dry-run smoke should report something like:
```json
{
  "tool": "v2-state-dispatch-archive",
  "dry_run": true,
  "archived_count": 0,        // dry-run does not archive
  "archive_file": "docs/state-archive/dispatches-2026-05-17.json",  // would-be path
  "ineligible": {
    "live_status": 3,
    "ineligible_status": 5,
    "below_age_threshold": <N>,
    "missing_timestamp": 3
  },
  "state_after": {
    "agent_sessions_total": 930,
    "agent_sessions_live": 3,
    "agent_sessions_terminal_retained": <930 minus would-archive>
  }
}
```

(The exact `archived_count` for a real --age-days=30 sweep depends on how many of the 819 merged entries have `merged_at` more than 30 days before today; the cycle 163 measurement found ~7-week-old entries dominate the merged set so most will archive.)

Do NOT run the tool without `--dry-run` against the live state.json in this dispatch — main-orchestrator will run the backlog archival under controlled conditions in cycle 165+ (priority #2 in the cycle 163 forward list).

## Style and quality bar

- Match the v2-state-audit code style: clear function names, small focused functions, doc comments where the design intent is non-obvious. Avoid placeholder comments / future-work TODOs that don't have a tracking issue.
- All error paths must return one of the documented exit codes. Use `std::process::ExitCode` (as v2-state-audit does), not `panic!`.
- Tests must be deterministic. Avoid time-of-day flakes — fix the "now" via a `--now` flag (defaulting to actual now) if needed for test reproducibility. v2-state-audit uses real `SystemTime::now()`; you can follow that or inject — your call, but the integration tests must not flake.
- Clippy-clean with `-D warnings`.

## Direct-push permitted zones

The crate at `tools/rust/crates/v2-state-dispatch-archive/` is **NOT** in a direct-push-permitted zone for redesign mode (those are `prompts/v2/`, `tools/v2/`, `docs/redesign/`, `docs/journal/`). The new crate is at `tools/rust/crates/` which is PR-required-with-review. **Open this work as a PR for main-orchestrator review.**

## When you're done

Open a PR titled something like "feat(v2): implement v2-state-dispatch-archive per cycle 163 design scope" against `master`. PR description should:

- Confirm the design scope sections honored
- Note any deviations from the scope and why (if any)
- Include the dry-run smoke output (the JSON block above, with real numbers)
- Include `cargo test` + `cargo clippy` clean confirmation
- Note total LOC of the new crate

Main-orchestrator will absorb in cycle 165+ (similar to cycle 155 absorbing the cycle 152 v2-cycle-runner critique PR #2961, and cycle 148 absorbing PR #2953 v2-prompt-contract-check).

---

**This is the first Track-1 dispatch application of directive [#2937](https://github.com/EvaLok/schema-org-json-ld/issues/2937).** Cycles 138-163 had zero dispatches; the directive named this category of "parallel SCAFFOLD work with already-scoped design" as a structural opportunity. Main-orchestrator's Track 2 this cycle is the v2-channel-router enforcement extension design scope (cycle 163 forward priority #4) — concurrent substrate work that doesn't depend on this dispatch's outcome.
