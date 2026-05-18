# v2-role-driver live-claude-code-spawn arc

**Status:** design-scope (cycle 178); implementation deferred to cycle 179+.

**Provenance:** Carry-forward from `cycle-150-two-track-implementation-entry.md` OQ2 (role-session model) + `cycle-155-cycle-152-critique-absorption.md` C18 (L3.6 session-output-file scaffolding cliff, AGREE-WITH-CARVEOUT). Originally tentatively projected at cycle 200+ in cycle 155. Promoted to active design-scope by [`[input-from-eva] #2992`](https://github.com/EvaLok/schema-org-json-ld/issues/2992) (2026-05-18, authored by EvaLok) per `feedback_drastic_change_when_incremental_failed` discipline after 24-cycle runway on the AGREE-DEFER queue gated on `post-real-role-session-measurement` evidence that does not exist because no live role session has ever run.

**Scope relative to prior design:** Cycle 149's `v2-cycle-runner-design-scope` specified the runner's CLI surface and step sequence. Cycle 150 implemented `v2-role-driver invoke` as the SCAFFOLD primitive — accepting `--session-output-file <path>` instead of spawning a real Claude subprocess. Cycle 176's `v2-primitive-invoker-timeout-arc.md` introduced timeout-aware `PrimitiveInvoker` returning `InvocationResult::Completed(Output) | TimedOut { ... }` with SIGTERM→2s→SIGKILL escalation, with `v2-role-driver: 4500s` as the per-step default timeout. Cycle 177 implemented that timeout arc (cycle 1 implementation) in `tools/rust/crates/v2-cycle-runner/src/main.rs`.

This document specifies the v2-role-driver evolution from SCAFFOLD (file-input) to LIVE (subprocess-spawn-of-claude-code), the new CLI flag set, the per-role context-assembly + tool-permission profile, the channel-payload-from-stdout parsing pipeline, the coordination with the cycle-177-landed timeout machinery, and the migration cost.

**Parallelism authorization:** `[input-from-eva] #2992` explicitly authorizes running this arc's implementation cycles on the same cycles as remaining C13+X2 work (e.g., C13+X2 cycle 2: `elapsed_ms` schema promotion when a reader appears) and other coordinated arcs (C6+C7+C9, C11+C12+X1). Serialization across coordinated arcs is OVERRIDDEN by the directive.

## 1. Problem statement

### 1.1 What exists today (SCAFFOLD)

`tools/rust/crates/v2-role-driver/src/main.rs:34-91` — the `Invoke` subcommand:

```rust
Invoke {
    #[arg(long, value_enum)] role: Role,
    #[arg(long)] cycle: u32,
    #[arg(long)] session_output_file: PathBuf,   // <-- the cliff
    #[arg(long)] timestamp: Option<String>,
    #[arg(long)] skip_super_step_check: bool,
    #[arg(long)] skip_channel_write: bool,
}
```

Doc-comment at lines 36-38:
> SCAFFOLD scope: session output is provided via `--session-output-file` (no live claude-code subprocess). DEFERRED to COMPLETE arc: spawn claude-code with the role's prompt + assembled context, parse stdout/exit-code, classify silent-zero output, enforce per-role iteration ceiling.

`cmd_invoke` (lines 670-781) reads the file, parses one of three accepted JSON shapes via `parse_session_output` (lines 587-619), validates the payload against `required_payload_keys(channel)` (lines 187-200), and writes the channel via the local reducer-rule logic (`write_channel`, lines 454-515 — itself duplicated from `v2-channel-router` at SCAFFOLD scope per the cycle 138 directive on isolated reducer-rule).

### 1.2 Why this matters

The SCAFFOLD path makes the entire orchestrator non-functional for its actual purpose: **no real Claude session has ever executed an orchestrator cycle.** Every cycle to date has been the orchestrator-running-as-claude-opus-4-7 doing the work *outside* the new pipeline, then writing the cycle's artifacts. The new pipeline has been exercised at the CLI / wiring / state-machine layer but never at the role-session layer.

This is the AGREE-DEFER queue's blocker. Curator complexity rework, template-mirror architecture-revisit, and other findings absorbed cycles 152/166/170/171 are all gated on `post-real-role-session-measurement` evidence — empirical data on per-cycle decision overhead, coordination overhead, iteration events. Without a single live spawn, the evidence is impossible to gather, and the AGREE-DEFER queue carries indefinitely.

The C13+X2 arc landed cycle 176-177 partially anticipated this: the `v2-role-driver: 4500s` timeout budget exists in the runtime layer (`default_step_timeout` in `tools/rust/crates/v2-cycle-runner/src/main.rs`) but no real role session can hit that budget because the role-driver is SCAFFOLD.

### 1.3 What's NOT in cycle 1 of implementation

Per `[input-from-eva] #2992` non-goals and cycle-1 minimal end-to-end scoping:

- **Streaming output capture.** Cycle-1 minimal: batched-only via `InvocationResult` (matches C13+X2 cycle-1 scope). Streaming deferred to a separate arc.
- **Multi-turn sessions.** Cycle-1 minimal: single-turn (one user message → one assistant response → exit). The role prompts are designed for one-shot output. Multi-turn deferred.
- **Per-agent durable memory beyond channel state.** Channel state IS the durable memory (HARDENED principle from cycle 158-161). No per-role session memory in cycle 1.
- **Plan-lifecycle integration.** No cycle-1 changes to the v2-channel-router or v2-super-step-boundary contracts. Live-spawn produces channel state via the existing reducer-rule path.
- **Removal of `--session-output-file`.** It is RETAINED in cycle 1 as a hermetic-test-mode flag (preserves all existing v2-cycle-runner integration tests). Live-spawn is opt-in via a new flag (§2.1). A future cycle may flip the default once live-spawn is exercised across all four roles.

## 2. CLI surface evolution

### 2.1 New flags

The `Invoke` subcommand gains three flags:

```rust
Invoke {
    #[arg(long, value_enum)] role: Role,
    #[arg(long)] cycle: u32,

    /// Path to claude-code binary. When present, live-spawn mode is selected:
    /// role-driver assembles context, spawns claude-code as subprocess, parses
    /// stdout. When absent, SCAFFOLD mode (file-input) is selected.
    #[arg(long)] claude_code_bin: Option<PathBuf>,

    /// Role prompt file. Defaults to `prompts/v2/<role>-prompt.xml`. Only
    /// consulted in live-spawn mode.
    #[arg(long)] prompt_file: Option<PathBuf>,

    /// Maximum conversation turns before claude-code exits. Defaults to 50
    /// (one role session should resolve in one turn; ceiling is a safety net).
    /// Only consulted in live-spawn mode.
    #[arg(long, default_value = "50")] max_turns: u32,

    // --- SCAFFOLD-mode flag (RETAINED) ---
    /// Path to a JSON file containing the session output. Required when
    /// --claude-code-bin is NOT set. Retained in cycle 1 as a test-mode flag.
    #[arg(long)] session_output_file: Option<PathBuf>,

    // --- unchanged ---
    #[arg(long)] timestamp: Option<String>,
    #[arg(long)] skip_super_step_check: bool,
    #[arg(long)] skip_channel_write: bool,
}
```

**Mode selection:** if `--claude-code-bin` is set, live-spawn mode. If `--session-output-file` is set, SCAFFOLD mode. Setting both → error. Setting neither → error.

The flag-set retention preserves all existing v2-cycle-runner integration tests that pass `--session-output-file <hand-prepared-JSON>` — they continue to work unchanged.

### 2.2 Caller migration in v2-cycle-runner

`tools/rust/crates/v2-cycle-runner/src/main.rs::build_step_invocation` for `StepKind::RoleInvoke(role)` (current cycle 151+ shape) constructs the role-driver argv. The new shape:

```rust
// before (SCAFFOLD)
StepKind::RoleInvoke(role) => vec![
    "invoke".into(),
    "--role".into(), role.as_kebab().into(),
    "--cycle".into(), cycle.to_string(),
    "--session-output-file".into(), session_output_file_for_role(role).into(),
    "--timestamp".into(), now_iso8601(),
],

// after (live-spawn, opt-in via new --claude-code-bin flag on runner)
StepKind::RoleInvoke(role) => {
    let mut argv = vec![
        "invoke".into(),
        "--role".into(), role.as_kebab().into(),
        "--cycle".into(), cycle.to_string(),
        "--timestamp".into(), now_iso8601(),
    ];
    match runner_args.role_spawn_mode {
        RoleSpawnMode::LiveClaude { ref bin, max_turns } => {
            argv.push("--claude-code-bin".into());
            argv.push(bin.display().to_string());
            argv.push("--max-turns".into());
            argv.push(max_turns.to_string());
        }
        RoleSpawnMode::ScaffoldFile { ref output_file_for_role } => {
            argv.push("--session-output-file".into());
            argv.push(output_file_for_role(role).display().to_string());
        }
    }
    argv
}
```

The runner's CLI gains `--role-spawn-mode {live-claude,scaffold-file}` plus mode-specific flags. Default in cycle-1 implementation: `scaffold-file` (preserves existing integration tests). Cycle 2+ flips the default after exercising live mode end-to-end.

## 3. Subprocess model

### 3.1 Choice: spawn `claude-code` as subprocess via existing `PrimitiveInvoker`

**Decision:** spawn `claude-code` CLI as a subprocess. Reasons:

1. **Reuses C13+X2 RealInvoker directly.** The cycle-177-landed `RealInvoker` already implements thread+channel pattern, SIGTERM/SIGKILL escalation, partial output capture. Live-spawn becomes "PrimitiveInvoker against `claude-code` bin instead of `v2-cycle-runner` bin" — same machinery.
2. **Auth and credential handling stay in `claude-code`.** Anthropic API key lives in environment (`ANTHROPIC_API_KEY` already configured as GitHub Actions secret). No new auth code in v2-role-driver.
3. **Tool permissions plumbed via `claude-code` CLI flags.** Per-role `--allowed-tools` profile (§6). No Rust-side tool-policy implementation.
4. **Output capture is already plumbed.** stdout/stderr → `Output` for `Completed`, `partial_stdout`/`partial_stderr` for `TimedOut`. Both already serialized into the cycle report via cycle-177 wiring.
5. **Matches the orchestrator's own architecture.** The harness running THIS Claude session (cycle 178 of redesign) is `tools/cycle-runner` spawning `claude-code` as a subprocess. Live-spawn is the same shape at the role-driver layer.

Alternatives considered:

- **(B) Spawn a Python/Node shim that calls Anthropic API via SDK.** Rejected: introduces another runtime; doesn't reuse C13+X2; auth handling re-implemented.
- **(C) Direct HTTP API calls from Rust to `/v1/messages`.** Rejected: re-implements message framing, streaming, retry, tool-use loop. Reinventing `claude-code` in Rust is far out of scope.

### 3.2 Spawn invocation shape

```rust
// inside cmd_invoke when live-spawn mode is selected:
let invoker = RealInvoker::new();
let timeout = Duration::from_secs(4500); // matches C13+X2 §3.3 v2-role-driver budget
let args = build_claude_code_argv(role, &prompt_file, max_turns, &context_input)?;
let result = invoker.invoke(&claude_code_bin, &args, timeout)?;
```

The `build_claude_code_argv` constructs:

```
claude-code
  --print
  --output-format json
  --append-system-prompt "<assembled role context>"
  --max-turns 50
  --allowed-tools <per-role tool profile>
  --permission-mode acceptEdits
  --model claude-opus-4-7
  <user message via stdin>
```

**OQ-LS-1 (open question, deferred to cycle 1 implementation):** the exact `claude-code` CLI flag names and accepted values. Inspection of `claude-code --help` on the GitHub Actions runner (which has `claude-code` installed) determines the actual flag set. The design above documents the SHAPE; the actual flag names may need ~30-50 LOC of adjustment when cycle-1 implementation runs `claude-code --help` and reconciles. Honest acknowledgment: this design scope writes the flag set as plausible, not verified.

### 3.3 stdin vs file for input passing

The role's prompt is the system prompt (passed via `--append-system-prompt` after the inline base system prompt — or `--system-prompt-file` if that exists in the live CLI). The role's INPUT (assembled channel-state context) is the user message.

Two ways to pass the user message:
- **stdin:** `claude-code ... <<< "$context_input"` — clean, no temp file.
- **file:** `claude-code --prompt-file <path>` — explicit, debuggable.

**Decision:** stdin. Pros: no temp-file lifecycle to manage; no path-quoting; works the same on all OSes. Cons: harder to debug post-hoc (the input isn't preserved). Mitigation: the role-driver writes the assembled context to `state/roles/<role>-last-context-cycle-N.json` for debugging *before* spawning the subprocess. Subprocess receives via stdin; debug record exists on disk.

## 4. Session lifecycle

### 4.1 Flow

```
v2-role-driver invoke --role reconciler --cycle 178 --claude-code-bin /usr/local/bin/claude-code
  │
  ├─ super-step check (existing): ensures state/super-step.json shows reconciler @ cycle 178
  ├─ assemble context (NEW, §5): build user-message JSON from input channels + cycle metadata
  ├─ persist debug record: state/roles/reconciler-last-context-cycle-178.json
  ├─ spawn claude-code subprocess via PrimitiveInvoker::invoke (§3.2)
  │    │
  │    ├─ on Completed(Output): §4.2
  │    └─ on TimedOut { .. }: §4.3
  ├─ parse stdout as channel-payload JSON (§4.4)
  ├─ validate payload against required_payload_keys(channel) (existing logic)
  ├─ write channel state via write_channel (existing logic)
  └─ append RoleRun to state/roles/reconciler-history.json
```

### 4.2 Completed flow

`InvocationResult::Completed(Output)`:

1. Extract stdout bytes.
2. If `--output-format json` was used, parse the claude-code envelope, extract the final-assistant-message text (`OQ-LS-1` resolves the exact path here).
3. Parse the extracted text as JSON channel-payload (existing `parse_session_output` logic accepts shapes 1/2/3).
4. Validate via `validate_payload(channel, &payload)`.
5. Write via `write_channel(...)`.
6. Append `RoleRun { cycle, role, at, outcome: Success, notes: "live-spawn: <claude-code-session-id>" }`.

If stdout parse fails (claude-code returned text not JSON, or JSON with wrong shape), the role-driver returns `Outcome::WriteSkipped` with notes describing the failure. The cycle-runner sees a non-zero v2-role-driver exit (existing classification path via `FailureClass`).

### 4.3 Timeout flow

`InvocationResult::TimedOut { elapsed, partial_stdout, partial_stderr, escalation }`:

1. NEW: `Outcome::Timeout` variant added to the `Outcome` enum (currently: `Success`, `WriteSkipped`).
2. Persist a `RoleRun` with:
   ```rust
   RoleRun {
       cycle, role, at: now_iso8601(),
       outcome: Outcome::Timeout,
       notes: format!(
           "live-spawn timeout: budget=4500s elapsed={}ms escalation={:?}; partial_stdout={} bytes; partial_stderr={} bytes",
           elapsed.as_millis(), escalation, partial_stdout.len(), partial_stderr.len()
       ),
       timeout: Some(TimeoutDiagnostic { budget_ms: 4_500_000, elapsed_ms: elapsed.as_millis() as u64, escalation }),
   }
   ```
3. **Do not write channel state.** A timed-out session may have emitted partial stdout that *looks* like a valid channel-payload prefix; treating it as authoritative would corrupt downstream channels.
4. Exit with non-zero exit code so v2-cycle-runner sees the timeout and halts the cycle with `FailureClass::Timeout` (already wired in cycle 177).

**Partial output handling:** partial stdout/stderr are CAPTURED in the role-history record for post-hoc debugging but NOT parsed as authoritative payload. This is the safe failure mode — a partial JSON could mean "the model decided not to respond" or "the model was cut off mid-thought"; both warrant a halt, not a write.

### 4.4 stdout parsing

The `--output-format json` envelope shape (per `claude-code` CLI knowledge) is approximately:

```json
{
  "type": "result",
  "subtype": "success" | "error",
  "is_error": false,
  "result": "<final assistant message text>",
  "session_id": "abc123",
  "duration_ms": 12345,
  "num_turns": 1,
  "total_cost_usd": 0.045,
  "usage": {
    "input_tokens": N,
    "output_tokens": M,
    ...
  }
}
```

The `result` field contains the final assistant message as plain text. The role's prompt instructs the model: "your final output MUST be ONLY a JSON object matching the channel-payload contract for <output-channel>; no surrounding prose."

The role-driver extracts `result`, then parses it as JSON channel-payload.

**OQ-LS-1 part 2:** the exact field path (`result` vs `messages[-1].content` vs other) depends on the live `claude-code` envelope shape. Cycle 1 implementation runs `claude-code --print --output-format json --print "test"` and inspects the actual envelope shape.

## 5. Context assembly (user message)

### 5.1 Shape

The user message passed to `claude-code` is a structured text block:

```
<cycle-context>
  <cycle>178</cycle>
  <role>reconciler</role>
  <timestamp>2026-05-18T22:18:00Z</timestamp>
  <output-channel>inbound-channel</output-channel>
  <required-payload-keys>eva-responses, audit-posts, dispatch-returns, inbound-completeness-marker</required-payload-keys>
</cycle-context>

<input-channels>
  <!-- per role.input_channels() -->
  <channel name="memory-channel" source="previous-cycle">
    <state cycle="177" writer="curator" timestamp="2026-05-18T20:50:00Z">
      {... full payload JSON ...}
    </state>
  </channel>
  <channel name="inbound-channel" source="current-cycle">
    <state>not-initialized</state>
  </channel>
</input-channels>
```

For reconciler specifically: `<input-channels>` is empty (`Role::Reconciler::input_channels()` returns `&[]`). The user message instead lists the EXTERNAL surfaces the reconciler should poll: open issues, recent PRs, audit-repo activity, dispatch-return queue.

### 5.2 Assembly source

Existing `cmd_context` (lines 813-896 in v2-role-driver/src/main.rs) ALREADY produces this shape (in `text` format). The live-spawn path calls a refactored `assemble_context_for_user_message(role, cycle)` that produces the structured text block above as a String. The refactor:

- Extract the context-rendering logic out of `cmd_context` text-formatting into a shared `render_context(role, cycle) -> String`.
- `cmd_context` calls `render_context` for its text output.
- `cmd_invoke` (live-spawn mode) calls `render_context` to build the user message.

Estimated refactor: ~80 LOC moved + ~30 LOC of new escaping (XML-tag-quote escape for payload JSON embedded in `<state>...</state>` blocks).

### 5.3 Reconciler external-surface assembly

For reconciler, the user message extends the assembly with external-surface pollable URIs:

```
<inbound-surfaces>
  <surface name="github-issues" repo="EvaLok/schema-org-json-ld">
    <description>Open issues with labels question-for-eva, input-from-eva, agent-task</description>
  </surface>
  <surface name="github-prs" repo="EvaLok/schema-org-json-ld">
    <description>Open PRs with draft state or recently merged</description>
  </surface>
  <surface name="audit-repo-activity" repo="EvaLok/schema-org-json-ld-audit">
    <description>New audit cycles posted since the last reconciler cycle</description>
  </surface>
  <surface name="dispatch-returns" path="docs/state.json">
    <description>Active dispatches and their return states</description>
  </surface>
</inbound-surfaces>
```

The role's prompt (`prompts/v2/reconciler-prompt.xml`) instructs the model to use its `Bash` tool to invoke `gh issue list` / `gh pr list` / `gh api repos/.../issues` against these surfaces, then synthesize the inbound-channel payload.

This shape is consistent with the cycle 152 super-step integration test (`role-driver invoke --role reconciler (writes inbound-channel)` — the test passes a hand-prepared inbound-channel JSON; live-spawn produces the same shape but from a real model invocation).

## 6. Per-role tool permissions

### 6.1 Profile per role

| Role | `--allowed-tools` | Justification |
|---|---|---|
| Reconciler | `Read, Bash` | Reads issues/PRs via `gh` CLI; reads local state for dispatch-return queue. No file mutation; reconciler emits its findings via the channel-payload, not by editing files. |
| Planner | `Read, Grep` | Reads memory-channel + inbound-channel + relevant `_notes/` files. Pure analysis; no mutation, no shell. |
| Executor | `Read, Edit, Write, Grep, Bash` | The actual work-doing role. Edits source files, runs cargo, etc. Maximum permission profile. |
| Curator | `Read, Grep` | Reads work-channel + recent _notes; emits consolidated insights to memory-channel. No mutation. |

`--permission-mode` is `acceptEdits` for all roles (autonomous operation). Future cycles may differentiate: e.g., curator's prompt is pure analysis with no Bash — could move to a stricter permission mode. Defer to cycle 3+ refinements.

### 6.2 What the role's prompt enforces beyond tool flags

Tool-level permissions can prevent the model from CALLING a tool but cannot prevent semantic misuse (e.g., reconciler reading state files inappropriately, or executor writing to files outside the workspace). The role's system prompt is the second line of defense — it specifies the role's scope of action and authority boundaries. Existing role prompts in `prompts/v2/*.xml` already address this; live-spawn implementation does not modify them.

### 6.3 Sandboxing

The `claude-code` subprocess inherits the parent harness's process tree. No additional sandboxing in cycle 1. The parent harness runs in GitHub Actions runner, which provides per-job isolation. This is sufficient for cycle 1 — adversarial model output is bounded by the runner's own permissions.

A future cycle may add `claude-code --working-directory <subdir>` or similar to restrict per-role workspace scope. Deferred (no observed need yet).

## 7. State exchange

### 7.1 Reading prior-role outputs

Per §5.1, channel state is embedded INTO the user message. The Claude session does not need to `Read` channel-state files — it sees them in the conversation. Pros: one fewer point of failure; no Read-call cost; structured by the orchestrator rather than discovered by the model. Cons: large channel states could expand token cost.

Cycle-1 channel-state sizes (empirical from cycle 153-178 hand-prepared session outputs): ~500 bytes to ~5 KB per channel. Inline embedding fits comfortably within Claude's context. Switch to file-handoff is reserved for the (unlikely) case channel states grow to MB scale.

### 7.2 Writing role output

Channel-state files are written by v2-role-driver's existing `write_channel(...)` after stdout parsing succeeds. The Claude session does NOT directly write channel-state files — it emits the payload, the role-driver writes the file. This preserves the reducer-rule invariant (cycle 138 directive): channel writes always go through the validate-payload + atomic-rename path.

### 7.3 Per-role history

`state/roles/<role>-history.json` is appended by the role-driver after each invocation. Live-spawn adds two pieces of metadata to the `RoleRun` record:

1. `outcome: Timeout` (new variant; §4.3).
2. `timeout: Option<TimeoutDiagnostic>` (new field; mirrors `StepTrace.timeout` from cycle 177).

Optionally cycle 2+: `cost_usd: Option<f64>`, `usage: Option<TokenUsage>`, `session_id: Option<String>` — when the claude-code envelope shape is known and the reader appears (central-bet validation per cycle 103/106/108). Per `schema-promotion-discipline.md`, defer adding these fields until a reader needs them.

## 8. Coordination with C13+X2 timeout arc

### 8.1 What's already landed (cycle 177)

The C13+X2 implementation (`tools/rust/crates/v2-cycle-runner/src/main.rs`):

- `default_step_timeout(StepKind::RoleInvoke(_)) = Duration::from_secs(4500)` — 75 min budget.
- `RealInvoker` with SIGTERM→2s→SIGKILL escalation.
- `InvocationResult::TimedOut` propagates to caller; v2-cycle-runner halts with `FailureClass::Timeout`.
- `StepTrace.timeout` populated on timeout halts.

These ALREADY apply to v2-role-driver invocations — but the SCAFFOLD path never hits the budget (file reads complete in milliseconds). Live-spawn is the first path that can genuinely exhaust 4500s.

### 8.2 What live-spawn adds at the cycle-runner layer

**Nothing.** The cycle-runner's invocation-of-role-driver flow is unchanged. The role-driver invocation completes within 4500s (live or SCAFFOLD) or times out — same machinery either way. The cycle-runner halts on `FailureClass::Timeout` regardless of what was happening *inside* the role-driver.

### 8.3 What live-spawn adds at the role-driver layer

The role-driver internally calls `RealInvoker::invoke(claude_code_bin, ..., Duration::from_secs(4500))`. The role-driver's own SIGTERM-handling: when the cycle-runner sends SIGTERM to the role-driver (its own 4500s budget exhausted at the cycle-runner layer), the role-driver should propagate to its claude-code child.

Implementation: when the role-driver receives SIGTERM, it sends SIGTERM to the claude-code subprocess BEFORE exiting itself. The role-driver's `RealInvoker::invoke` call is blocking; the SIGTERM handler must run in a separate thread (or as an installed signal handler) and forward the signal.

**OQ-LS-2:** signal forwarding implementation. Two viable patterns:
- (a) Install a `SIGTERM` handler via `signal_hook` crate that forwards to the child PID. Requires a `Mutex<Option<child_pid>>` to track which child to signal.
- (b) Trust the kernel: when a process group is signaled, all members receive it. If the role-driver and its claude-code child are in the same process group, signaling the role-driver also signals the child. Per Unix process-group semantics, `Command::spawn()` puts the child in the parent's process group unless `setsid()` is called.

Recommendation: (b) — relies on existing process-group semantics, no signal-handling code. Verify in cycle 1 implementation by sending SIGTERM to a `v2-role-driver invoke --claude-code-bin ...` and confirming the claude-code child also exits. If process-group inheritance breaks down (e.g., because of shell interposition or `setsid` calls), fall back to (a).

### 8.4 Partial output from a killed claude-code

When the role-driver's claude-code child is killed mid-stream, `partial_stdout` may contain:
- (a) Zero bytes (model hadn't started emitting).
- (b) A partial JSON envelope (model was streaming; cut off mid-message).
- (c) A complete envelope (model finished but exit was hung — unusual).

The role-driver treats all three as `Outcome::Timeout` with no channel write (§4.3). The partial bytes are preserved in the `RoleRun.notes` for debugging.

## 9. First-spawn target: reconciler

### 9.1 Why reconciler first

Per `[input-from-eva] #2992` recommendation: **reconciler**. Reasoning supports it:

1. **Runs first in the super-step cycle** — exercising it doesn't require any prior role to have succeeded.
2. **Simplest input channel set** — `Role::Reconciler::input_channels()` returns `&[]`. No memory-channel or work-channel state to assemble.
3. **External-surface assembly is a small bounded set** — `gh issue list` + `gh pr list` + audit-repo read + dispatch-return read. ~10 lines of context assembly.
4. **Smallest required payload** — `inbound-channel` requires `eva-responses, audit-posts, dispatch-returns, inbound-completeness-marker` — four keys, all of which can be empty arrays in a quiet cycle.

### 9.2 Cycle-2 first-live-spawn acceptance (smoke test)

The first ACTUAL live spawn happens in cycle 2 of live-spawn implementation (post cycle 1's trait/wiring work). Cycle-2 acceptance:

1. Set up a hermetic repo state: a known set of open issues, a known recent commit, a known audit-cycle reference.
2. Invoke `v2-role-driver invoke --role reconciler --cycle <test-cycle> --claude-code-bin /usr/local/bin/claude-code --max-turns 50 --skip-super-step-check`.
3. Observe:
   - Wall clock to completion (target: <300s for a quiet repo).
   - Token usage from claude-code envelope (target: <50K input + <5K output).
   - stdout is a JSON envelope; `result` field is a JSON object matching `inbound-channel` payload contract.
   - Channel state at `state/channels/inbound-channel.json` is updated.
   - `state/roles/reconciler-history.json` has a new `RoleRun` with `outcome: Success`.
4. Failure modes to catch:
   - JSON-shape mismatch: the model emitted plausible prose instead of a JSON object → catch and report.
   - Tool-use loop never terminates within 50 turns → catch via max-turns ceiling.
   - Token exhaustion during a turn → catch via claude-code's own error reporting.

### 9.3 Subsequent roles

After reconciler exercise (cycle 2), each remaining role gets one cycle:
- Cycle 3: planner live-spawn (reads channel-state from cycle 2's reconciler output + a hand-prepared memory-channel).
- Cycle 4: executor live-spawn (reads channel-state from cycle 3's planner output).
- Cycle 5: curator live-spawn (reads channel-state from cycle 4's executor output).

Then cycle 6+: end-to-end multi-role cycle with all four roles live.

## 10. Honest LOC + cycle estimate

### 10.1 Raw estimate (source-side only)

| Concern | LOC |
|---|---|
| New `RealInvoker` wiring in v2-role-driver | 80 |
| `--claude-code-bin` / `--prompt-file` / `--max-turns` flags + dispatch | 100 |
| `render_context` refactor + extraction from `cmd_context` | 80 |
| Reconciler external-surface assembly | 60 |
| stdout-envelope parsing (claude-code JSON output) | 100 |
| `Outcome::Timeout` variant + RoleRun timeout field | 40 |
| Per-role tool-profile mapping | 30 |
| v2-cycle-runner caller migration (RoleInvoke argv shape) | 50 |
| Live-spawn debug record (state/roles/<role>-last-context-cycle-N.json) | 30 |
| Test code: MockInvoker integration for live-spawn path | 200 |
| Test code: signal-forwarding verification | 80 |
| Test code: stdout-envelope-parsing unit tests | 100 |

**Raw total: ~950 LOC.**

### 10.2 Honest budget applying RECURRENCE-AT-3 multiplier

Per `comprehensive-test-suite-exceeds-design-scope-LOC-estimate` RECURRENCE-AT-3 (cycles 170, 171, 177: 2-3× actual over estimate), honest budget: **1900-2850 LOC** for cycle 1 implementation.

### 10.3 Cycle-count estimate

| Cycle | Substantive focal |
|---|---|
| Cycle 1 (cycle 179) | Trait wiring, `--claude-code-bin` flag, hermetic MockInvoker tests. Live spawn not exercised. |
| Cycle 2 (cycle 180) | First live reconciler spawn end-to-end; measure wall-clock, tokens, JSON-shape compliance. |
| Cycle 3 (cycle 181) | Refinements from cycle-2 measurement; second reconciler spawn or first planner spawn. |
| Cycle 4-5 (cycle 182-183) | Remaining roles' first live spawns. |
| Cycle 6+ (cycle 184+) | Multi-role end-to-end live cycle; first real-role-session-measurement for AGREE-DEFER queue. |

**Honest estimate: 5-7 cycles to first complete live multi-role cycle.** Allow +2-3 cycles slip on cycle 1 alone if `claude-code` CLI surface diverges materially from this design's assumptions (OQ-LS-1).

Total: **7-10 cycles from cycle 179 to live-spawn-complete-for-all-roles + first AGREE-DEFER-unblocking measurement.**

## 11. Open questions deliberately deferred

- **OQ-LS-1:** Exact `claude-code` CLI flag surface (system-prompt-path, user-message-passing, max-turns flag, output-format flag, allowed-tools flag, permission-mode flag, model selection). Determined cycle 1 implementation via `claude-code --help`.
- **OQ-LS-2:** Signal forwarding pattern from role-driver to claude-code child. Process-group default vs explicit signal-handler. Verified cycle 1 implementation.
- **OQ-LS-3:** stdout envelope shape — exact path to extract the role's emitted JSON payload from claude-code's `--output-format json` output. Verified cycle 1 implementation.
- **OQ-LS-4:** Token-usage / cost capture. Schema-promote `cost_usd` and `usage` to `RoleRun` when a reader appears (cycle 3+, post-first-live-spawn measurement). Per `schema-promotion-discipline.md`, no promotion until reader.
- **OQ-LS-5:** Decision-log capture beyond stdout envelope. The session's per-turn reasoning trace lives inside claude-code's internal session log; capturing it for cross-cycle analysis requires either `--include-partial-messages` or post-hoc session-log artifact extraction. Defer to cycle 3+ once first-live-spawn data is available.
- **OQ-LS-6:** Per-agent durable memory beyond channel state. Cycle 158-161 explored per-role state files. The HARDENED principle: channel state IS durable memory. No per-agent memory in cycle 1; revisit if cycle 3+ measurement surfaces a coordination-overhead issue that per-agent memory would resolve.
- **OQ-LS-7:** Per-cycle adaptive permission profiles. Cycle 1 ships per-role-constant profiles (§6.1). Adaptive profiles (e.g., reconciler-during-quiet-cycles gets read-only) are deferred.
- **OQ-LS-8:** Error classification beyond timeout. Cycle 1: non-zero claude-code exit → existing `FailureClass` classification path (stderr-keyword + exit-code). Cycle 2+: align claude-code's own error reporting (auth errors, rate-limit errors, network errors) with `FailureClass` variants. Possibly introduces `FailureClass::AuthError` and `FailureClass::RateLimit` as new variants. Coordinated with C6+C7+C9 (structured-error-envelope arc).
- **OQ-LS-9:** Replay / resume of failed role sessions. Coordinated with C11+C12+X1 (resume/recovery arc). Live-spawn cycle 1 has no resume — failed sessions halt the cycle; the next cycle re-runs from cycle-start.
- **OQ-LS-10:** Concurrency model for multiple roles in parallel. Cycle 1 is strictly serial per the super-step sequence (cycle 151 step-table). Future cycle MAY explore parallel-role-spawn for non-dependent roles (e.g., reconciler + curator are independent of each other if curator reads previous-cycle's work-channel). Deferred — no observed need.

## 12. Acceptance criteria for cycle 1 implementation

1. **Source compiles + clippy clean.** `cargo clippy --tests -D warnings` on `v2-role-driver` and `v2-cycle-runner` workspace members.
2. **Existing tests preserved.** All cycle 177 tests (77 unit + 5 integration) continue to pass.
3. **New unit tests cover live-spawn path:**
   - Mode-selection: `--claude-code-bin` set, `--session-output-file` set, both set (error), neither set (error).
   - `assemble_context_for_user_message` shape: structured text block with the documented `<cycle-context>`, `<input-channels>`, `<inbound-surfaces>` sections (reconciler only for inbound-surfaces).
   - stdout-envelope parsing: well-formed envelope → channel payload; malformed envelope → `Outcome::WriteSkipped` with diagnostic notes.
   - `Outcome::Timeout` path: MockInvoker queueing `CannedOutcome::Timeout` → `RoleRun` records `Timeout` outcome + diagnostic.
   - Per-role tool-profile mapping correctness (4 role × tool-list assertions).
4. **Hermetic integration test:** v2-cycle-runner driving v2-role-driver in live-spawn mode but with a MOCK claude-code binary (a small shell script that emits a known channel-payload JSON). End-to-end cycle completes; channel states are correctly written for all four roles.
5. **No live exercise of `claude-code` in cycle 1 — that is cycle 2's acceptance.** Cycle 1's acceptance is the WIRING + UNIT/INTEGRATION-LAYER MOCKING is correct.
6. **Documentation:** `cycle-<N>-live-spawn-implementation-cycle-1.md` matching the cycle-177 _notes structure.

## 13. Relationship to existing forward priorities

After this design scope lands cycle 178, the forward priority shape (per `[input-from-eva] #2992` proposed ordering):

1. Master Rust CI green-state maintenance + session-start orientation prelude (with `session-start-CI-check-discipline` HARDENED cycle 178 per `session-start-CI-check-discipline.md`).
2. **NEW: v2-role-driver live-spawn cycle 1 of implementation** (this scope's cycle-1 acceptance criteria above).
3. C13+X2 cycle 2 of implementation (was #3; unchanged in absolute terms — but cycle 1 of live-spawn is now ahead of it).
4. Coordinated structured-error-envelope arc (C6+C7+C9) — design pending.
5. Coordinated resume/recovery arc (C11+C12+X1) — design pending.
6. AGREE-DEFER queue (post-real-role-session-measurement) — moves UP once cycle 2 of live-spawn produces the first measurement.
7+. Remaining carry-forward items unchanged.

## 14. Honesty hedge

This design scope is structural acceleration on the gate that has been carried passively since cycle 155 C18 carveout (24+ cycles). The cycle-1 implementation may surface unknowns that this design has not anticipated — particularly OQ-LS-1 (the `claude-code` CLI surface). The 7-10 cycle estimate explicitly budgets for cycle-1 slip in the case where the CLI shape diverges from this design's assumptions.

The deliverable is the IMPLEMENTATION wiring + first measurement, not the design scope. This document is the design-scope-before-implementation pattern that landed cycle 176 (C13+X2 design) → cycle 177 (C13+X2 implementation). Implementation begins cycle 179 unless `[input-from-eva]` redirects.

Per `feedback_drastic_change_when_incremental_failed`: the incremental approach (waiting for AGREE-DEFER to unblock organically) has had 24-cycle runway. This is the explicit move from passive carry to active design + implementation.

---

**Companion document:** [`v2-primitive-invoker-timeout-arc.md`](v2-primitive-invoker-timeout-arc.md) (cycle 176) — the timeout-arc design this live-spawn arc coordinates with.

**Implementation entry point (cycle 179+):** `tools/rust/crates/v2-role-driver/src/main.rs` — extend the `Invoke` subcommand per §2.1, add `cmd_invoke_live_spawn` per §3.2 / §4 / §5, add the per-role tool profile per §6.1.

**Estimated total artifact LOC for cycle 1:** 950 raw, 1900-2850 honest budget per RECURRENCE-AT-3.

**Promotion criteria for cycle 2 first-live-spawn:** all six items in §12 satisfied at cycle 1 implementation close.
