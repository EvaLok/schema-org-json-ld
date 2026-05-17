use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "v2-role-driver",
    about = "Per-role session orchestration for the v2 multi-agent orchestrator: \
             loads role context (prompt + input-channel states), invokes a role's session, \
             validates the session output, writes to the role's output channel via local \
             reducer-rule logic, and records a per-role run history."
)]
struct Args {
    /// Repository root (path containing state/, prompts/, etc.).
    #[arg(long, default_value = ".", global = true)]
    repo_root: PathBuf,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text, global = true)]
    format: Format,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Initialize state/roles/ with per-role empty history files.
    Init,
    /// Invoke a role's session at the current super-step.
    ///
    /// SCAFFOLD scope: session output is provided via `--session-output-file` (no live
    /// claude-code subprocess). DEFERRED to COMPLETE arc: spawn claude-code with the
    /// role's prompt + assembled context, parse stdout/exit-code, classify silent-zero
    /// output, enforce per-role iteration ceiling.
    Invoke {
        #[arg(long, value_enum)]
        role: Role,
        #[arg(long)]
        cycle: u32,
        /// Path to a JSON file containing the session output. Accepted shapes:
        ///   1. `{ "cycle": N, "timestamp": "...", "payload": { ... } }` (channel-router WritePayload shape)
        ///   2. `{ "payload": { ... } }` (role-driver supplies cycle + timestamp)
        ///   3. `{ ...raw payload keys... }` (role-driver wraps as `payload`)
        #[arg(long)]
        session_output_file: PathBuf,
        /// Timestamp to record (ISO-8601). Defaults to a sentinel string; callers
        /// SHOULD supply a real timestamp.
        #[arg(long)]
        timestamp: Option<String>,
        /// Skip the super-step verification (for hermetic tests / scaffolding).
        #[arg(long)]
        skip_super_step_check: bool,
        /// Validate the session output but do not write to the channel (hermetic
        /// dry-run; still records a per-role run with outcome `write-skipped`).
        #[arg(long)]
        skip_channel_write: bool,
    },
    /// Assemble + print the context a role would receive at invocation (role prompt
    /// + current state of each input channel per role's input bindings).
    Context {
        #[arg(long, value_enum)]
        role: Role,
        #[arg(long)]
        cycle: u32,
        /// Override the default prompt file path (`prompts/v2/<role>-prompt.xml`).
        #[arg(long)]
        prompt_file: Option<PathBuf>,
    },
    /// Print a role's input bindings (which channels it reads at invocation).
    /// With `--cycle`, also reads each input channel's current state.
    Inputs {
        #[arg(long, value_enum)]
        role: Role,
        #[arg(long)]
        cycle: Option<u32>,
    },
    /// Print the per-role run history.
    History {
        #[arg(long, value_enum)]
        role: Role,
        /// Limit history entries (newest first when set).
        #[arg(long)]
        limit: Option<usize>,
    },
    /// Print the role/channel bindings + run-record schema + reducer-rule mapping.
    Schema,
}

#[derive(Copy, Clone, Debug, ValueEnum, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
enum Role {
    Reconciler,
    Planner,
    Executor,
    Curator,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
enum Format {
    Text,
    Json,
}

impl Role {
    fn name(self) -> &'static str {
        match self {
            Role::Reconciler => "reconciler",
            Role::Planner => "planner",
            Role::Executor => "executor",
            Role::Curator => "curator",
        }
    }

    fn all() -> &'static [Role] {
        &[
            Role::Reconciler,
            Role::Planner,
            Role::Executor,
            Role::Curator,
        ]
    }

    /// The output channel this role writes to (matches v2-channel-router::Channel
    /// kebab-case names; aligns with v2-super-step-boundary::Role::output_channel).
    fn output_channel(self) -> &'static str {
        match self {
            Role::Reconciler => "inbound-channel",
            Role::Planner => "plan-channel",
            Role::Executor => "work-channel",
            Role::Curator => "memory-channel",
        }
    }

    /// Input channels this role reads at invocation, per cycle 139 minimal end-to-end
    /// scoping. `previous-cycle` semantics for memory-channel mean role-driver reads
    /// the channel-state file unconditionally (memory-channel persists across cycles;
    /// the file's `cycle` field will be N-1 when planner reads it at cycle N).
    fn input_channels(self) -> &'static [InputBinding] {
        match self {
            Role::Reconciler => &[],
            Role::Planner => &[
                InputBinding {
                    channel: "memory-channel",
                    source: InputSource::PreviousCycle,
                },
                InputBinding {
                    channel: "inbound-channel",
                    source: InputSource::CurrentCycle,
                },
            ],
            Role::Executor => &[InputBinding {
                channel: "plan-channel",
                source: InputSource::CurrentCycle,
            }],
            Role::Curator => &[InputBinding {
                channel: "work-channel",
                source: InputSource::CurrentCycle,
            }],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum InputSource {
    CurrentCycle,
    PreviousCycle,
}

impl InputSource {
    fn name(self) -> &'static str {
        match self {
            InputSource::CurrentCycle => "current-cycle",
            InputSource::PreviousCycle => "previous-cycle",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
struct InputBinding {
    channel: &'static str,
    source: InputSource,
}

/// Required payload keys for each channel — duplicated locally from v2-channel-router
/// at SCAFFOLD scope (DEFERRED #1 names subprocess invocation of `v2-channel-router write`
/// which would remove this duplication).
fn required_payload_keys(channel: &str) -> &'static [&'static str] {
    match channel {
        "plan-channel" => &["substantive-focal", "per-role-tasks"],
        "work-channel" => &["artifacts-written"],
        "memory-channel" => &["consolidated-insights"],
        "inbound-channel" => &[
            "eva-responses",
            "audit-posts",
            "dispatch-returns",
            "inbound-completeness-marker",
        ],
        _ => &[],
    }
}

/// Channel envelope mirroring v2-channel-router::ChannelState. The channel + writer
/// fields are serialized with the same kebab-case names. Duplicated at SCAFFOLD scope.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChannelState {
    channel: String,
    writer: Role,
    cycle: u32,
    timestamp: String,
    payload: serde_json::Value,
}

/// Channel history mirroring v2-channel-router::ChannelHistory.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct ChannelHistory {
    channel: String,
    entries: Vec<ChannelHistoryEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChannelHistoryEntry {
    writer: Role,
    cycle: u32,
    timestamp: String,
    payload: serde_json::Value,
}

/// Subset of v2-super-step-boundary::SuperStepState used to verify the role's
/// invocation matches the active super-step. Duplicated at SCAFFOLD scope.
#[derive(Deserialize, Debug)]
struct SuperStepStateLite {
    cycle: u32,
    current_role: Role,
}

/// Run record persisted per role at state/roles/<role>-history.json.
#[derive(Serialize, Deserialize, Debug, Clone)]
struct RoleRun {
    cycle: u32,
    role: Role,
    at: String,
    outcome: Outcome,
    /// Free-form notes (which file the session output came from, why a skip fired,
    /// the channel that was written, etc.). Not load-bearing for tests.
    notes: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum Outcome {
    Success,
    WriteSkipped,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct RoleHistory {
    role: Option<Role>,
    runs: Vec<RoleRun>,
}

#[derive(Debug)]
enum DriverError {
    Io(io::Error),
    Json(String),
    NotInitialized(PathBuf),
    /// `state/super-step.json` exists but `current_role`/`cycle` do not match invocation.
    SuperStepMismatch {
        invoked_role: Role,
        invoked_cycle: u32,
        current_role: Role,
        current_cycle: u32,
    },
    /// No super-step state at all (boundary tool has not been init'd, or no cycle in progress).
    SuperStepNotInProgress,
    /// `--session-output-file` does not exist or cannot be read.
    SessionOutputMissing(PathBuf),
    /// `--session-output-file` is not a JSON object, or doesn't include a payload object,
    /// or the payload object is missing required channel keys.
    InvalidSessionOutput(String),
    /// Caller-supplied `--cycle` does not match a cycle embedded in the session-output payload.
    CycleMismatch {
        cli_cycle: u32,
        payload_cycle: u32,
    },
}

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriverError::Io(e) => write!(f, "{e}"),
            DriverError::Json(s) => write!(f, "json error: {s}"),
            DriverError::NotInitialized(p) => write!(
                f,
                "role-driver state not initialized at {} (run `v2-role-driver init`)",
                p.display()
            ),
            DriverError::SuperStepMismatch {
                invoked_role,
                invoked_cycle,
                current_role,
                current_cycle,
            } => write!(
                f,
                "super-step mismatch: invoked role '{}' for cycle {} but current super-step is role '{}' for cycle {}",
                invoked_role.name(),
                invoked_cycle,
                current_role.name(),
                current_cycle
            ),
            DriverError::SuperStepNotInProgress => write!(
                f,
                "no super-step is in progress: run `v2-super-step-boundary cycle-start --cycle N` first \
                 (or pass `--skip-super-step-check` for hermetic mode)"
            ),
            DriverError::SessionOutputMissing(p) => {
                write!(f, "session-output file not found: {}", p.display())
            }
            DriverError::InvalidSessionOutput(s) => write!(f, "invalid session output: {s}"),
            DriverError::CycleMismatch {
                cli_cycle,
                payload_cycle,
            } => write!(
                f,
                "cycle mismatch: --cycle {cli_cycle} but session-output payload declares cycle {payload_cycle}"
            ),
        }
    }
}

impl From<io::Error> for DriverError {
    fn from(e: io::Error) -> Self {
        DriverError::Io(e)
    }
}

impl From<serde_json::Error> for DriverError {
    fn from(e: serde_json::Error) -> Self {
        DriverError::Json(e.to_string())
    }
}

// ----- paths -----

fn state_dir(repo_root: &Path) -> PathBuf {
    repo_root.join("state")
}

fn roles_dir(repo_root: &Path) -> PathBuf {
    state_dir(repo_root).join("roles")
}

fn role_history_path(repo_root: &Path, role: Role) -> PathBuf {
    roles_dir(repo_root).join(format!("{}-history.json", role.name()))
}

fn super_step_state_path(repo_root: &Path) -> PathBuf {
    state_dir(repo_root).join("super-step.json")
}

fn channel_state_path(repo_root: &Path, channel: &str) -> PathBuf {
    state_dir(repo_root)
        .join("channels")
        .join(format!("{channel}.json"))
}

fn channel_history_path(repo_root: &Path, channel: &str) -> PathBuf {
    state_dir(repo_root)
        .join("channels")
        .join(format!("{channel}-history.json"))
}

fn default_prompt_path(repo_root: &Path, role: Role) -> PathBuf {
    repo_root
        .join("prompts")
        .join("v2")
        .join(format!("{}-prompt.xml", role.name()))
}

fn default_timestamp() -> String {
    "1970-01-01T00:00:00Z".to_string()
}

// ----- atomic write -----

/// Atomic write via temp-file + rename. Mirrors the pattern in
/// v2-channel-router::write_atomic and v2-super-step-boundary::write_atomic.
fn write_atomic(path: &Path, bytes: &[u8]) -> io::Result<()> {
    let dir = path.parent().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "path has no parent directory")
    })?;
    fs::create_dir_all(dir)?;
    let tmp = path.with_extension(format!(
        "{}.tmp",
        path.extension()
            .and_then(|s| s.to_str())
            .unwrap_or("partial")
    ));
    {
        let mut f = fs::File::create(&tmp)?;
        f.write_all(bytes)?;
        f.sync_all()?;
    }
    fs::rename(&tmp, path)?;
    Ok(())
}

// ----- super-step state read -----

/// Read super-step state and resolve to a (cycle, current_role) pair, or surface
/// the appropriate NotInProgress error. Returns Ok(None) on the empty sentinel
/// (state file exists but cycle is not in progress).
fn read_super_step_state(repo_root: &Path) -> Result<Option<SuperStepStateLite>, DriverError> {
    let path = super_step_state_path(repo_root);
    if !path.exists() {
        return Err(DriverError::SuperStepNotInProgress);
    }
    let raw = fs::read_to_string(&path)?;
    if raw.trim().is_empty() || raw.trim() == "null" {
        return Ok(None);
    }
    let state: SuperStepStateLite = serde_json::from_str(&raw)
        .map_err(|e| DriverError::Json(format!("decoding {}: {e}", path.display())))?;
    Ok(Some(state))
}

// ----- channel state read -----

#[derive(Debug, Clone)]
enum ChannelRead {
    NotInitialized,
    Empty,
    Populated(ChannelState),
}

fn read_channel_state(repo_root: &Path, channel: &str) -> Result<ChannelRead, DriverError> {
    let path = channel_state_path(repo_root, channel);
    if !path.exists() {
        return Ok(ChannelRead::NotInitialized);
    }
    let raw = fs::read_to_string(&path)?;
    if raw.trim().is_empty() || raw.trim() == "null" {
        return Ok(ChannelRead::Empty);
    }
    let state: ChannelState = serde_json::from_str(&raw)
        .map_err(|e| DriverError::Json(format!("decoding {}: {e}", path.display())))?;
    Ok(ChannelRead::Populated(state))
}

// ----- channel write (local reducer-rule application) -----

/// Apply the local reducer rule: verify the writer is the channel's allowed writer,
/// validate the payload, persist new state + append history. Duplicated at SCAFFOLD
/// scope from v2-channel-router (DEFERRED #1: subprocess invocation).
fn write_channel(
    repo_root: &Path,
    channel: &str,
    writer: Role,
    cycle: u32,
    timestamp: &str,
    payload: serde_json::Value,
) -> Result<(), DriverError> {
    // Reducer rule: each channel allows exactly one writer (per cycle 138 directive,
    // matching v2-channel-router::Channel::allowed_writer).
    if writer.output_channel() != channel {
        return Err(DriverError::InvalidSessionOutput(format!(
            "reducer violation: role '{}' writes channel '{}' but session output targets channel '{}'",
            writer.name(),
            writer.output_channel(),
            channel
        )));
    }

    validate_payload(channel, &payload)?;

    let state = ChannelState {
        channel: channel.to_string(),
        writer,
        cycle,
        timestamp: timestamp.to_string(),
        payload: payload.clone(),
    };

    let state_path = channel_state_path(repo_root, channel);
    let state_body = serde_json::to_string_pretty(&state)?;
    write_atomic(&state_path, state_body.as_bytes())?;

    let history_path = channel_history_path(repo_root, channel);
    let mut history: ChannelHistory = if history_path.exists() {
        let raw = fs::read_to_string(&history_path)?;
        if raw.trim().is_empty() {
            ChannelHistory {
                channel: channel.to_string(),
                entries: Vec::new(),
            }
        } else {
            serde_json::from_str(&raw).map_err(|e| {
                DriverError::Json(format!("decoding {}: {e}", history_path.display()))
            })?
        }
    } else {
        ChannelHistory {
            channel: channel.to_string(),
            entries: Vec::new(),
        }
    };
    history.entries.push(ChannelHistoryEntry {
        writer,
        cycle,
        timestamp: timestamp.to_string(),
        payload,
    });
    let history_body = serde_json::to_string_pretty(&history)?;
    write_atomic(&history_path, history_body.as_bytes())?;

    Ok(())
}

fn validate_payload(channel: &str, payload: &serde_json::Value) -> Result<(), DriverError> {
    let obj = payload.as_object().ok_or_else(|| {
        DriverError::InvalidSessionOutput(format!(
            "channel '{channel}' payload must be a JSON object, got {}",
            describe_json_type(payload)
        ))
    })?;
    for key in required_payload_keys(channel) {
        if !obj.contains_key(*key) {
            return Err(DriverError::InvalidSessionOutput(format!(
                "channel '{channel}' payload missing required key '{key}'"
            )));
        }
    }
    Ok(())
}

fn describe_json_type(v: &serde_json::Value) -> &'static str {
    match v {
        serde_json::Value::Null => "null",
        serde_json::Value::Bool(_) => "boolean",
        serde_json::Value::Number(_) => "number",
        serde_json::Value::String(_) => "string",
        serde_json::Value::Array(_) => "array",
        serde_json::Value::Object(_) => "object",
    }
}

// ----- per-role history -----

fn read_role_history(repo_root: &Path, role: Role) -> Result<RoleHistory, DriverError> {
    let path = role_history_path(repo_root, role);
    if !path.exists() {
        return Err(DriverError::NotInitialized(roles_dir(repo_root)));
    }
    let raw = fs::read_to_string(&path)?;
    if raw.trim().is_empty() {
        return Ok(RoleHistory {
            role: Some(role),
            runs: Vec::new(),
        });
    }
    let history: RoleHistory = serde_json::from_str(&raw)
        .map_err(|e| DriverError::Json(format!("decoding {}: {e}", path.display())))?;
    Ok(history)
}

fn append_role_history(repo_root: &Path, role: Role, run: RoleRun) -> Result<(), DriverError> {
    let mut history = read_role_history(repo_root, role)?;
    if history.role.is_none() {
        history.role = Some(role);
    }
    history.runs.push(run);
    let path = role_history_path(repo_root, role);
    let body = serde_json::to_string_pretty(&history)?;
    write_atomic(&path, body.as_bytes())?;
    Ok(())
}

// ----- session output parsing -----

/// Parse the session-output JSON into (cycle_override, timestamp_override, payload).
/// Accepted shapes (in order of preference):
///   1. `{ "cycle": N, "timestamp": "...", "payload": { ... } }` — full WritePayload shape
///   2. `{ "payload": { ... } }` — payload only
///   3. `{ ...raw payload keys... }` — bare payload (heuristic: must look like a channel payload)
///
/// Shape 3 is only accepted when the parsed object does NOT contain a `payload` key
/// (to avoid ambiguity with shape 2). This lets callers omit the wrapping when
/// convenient and still keeps shape 2 unambiguous.
fn parse_session_output(
    raw: &str,
) -> Result<(Option<u32>, Option<String>, serde_json::Value), DriverError> {
    let value: serde_json::Value = serde_json::from_str(raw)
        .map_err(|e| DriverError::InvalidSessionOutput(format!("not valid JSON: {e}")))?;
    let obj = value
        .as_object()
        .ok_or_else(|| {
            DriverError::InvalidSessionOutput(format!(
                "session output must be a JSON object, got {}",
                describe_json_type(&value)
            ))
        })?
        .clone();

    if obj.contains_key("payload") {
        let cycle = obj.get("cycle").and_then(|v| v.as_u64()).map(|n| n as u32);
        let timestamp = obj
            .get("timestamp")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let payload = obj.get("payload").cloned().unwrap();
        return Ok((cycle, timestamp, payload));
    }

    // Shape 3: bare payload. We accept this without trying to be clever about
    // detection — if the caller passed something that isn't a valid channel
    // payload, validate_payload() will reject it with a clear message.
    Ok((None, None, serde_json::Value::Object(obj)))
}

// ----- commands -----

fn cmd_init(repo_root: &Path, format: Format) -> Result<(), DriverError> {
    let dir = roles_dir(repo_root);
    fs::create_dir_all(&dir)?;
    let mut created = Vec::new();
    let mut already_present = Vec::new();
    for role in Role::all() {
        let path = role_history_path(repo_root, *role);
        if path.exists() {
            already_present.push(*role);
            continue;
        }
        let empty = RoleHistory {
            role: Some(*role),
            runs: Vec::new(),
        };
        let body = serde_json::to_string_pretty(&empty)?;
        write_atomic(&path, body.as_bytes())?;
        created.push(*role);
    }
    match format {
        Format::Text => {
            if !created.is_empty() {
                let names: Vec<&str> = created.iter().map(|r| r.name()).collect();
                println!("init: created per-role history for {}", names.join(", "));
            }
            if !already_present.is_empty() {
                let names: Vec<&str> = already_present.iter().map(|r| r.name()).collect();
                println!(
                    "init: per-role history already present for {}",
                    names.join(", ")
                );
            }
            if created.is_empty() && already_present.is_empty() {
                println!("init: nothing to do");
            }
        }
        Format::Json => {
            let out = serde_json::json!({
                "created": created.iter().map(|r| r.name()).collect::<Vec<_>>(),
                "already_present": already_present.iter().map(|r| r.name()).collect::<Vec<_>>(),
            });
            println!("{}", serde_json::to_string_pretty(&out)?);
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn cmd_invoke(
    repo_root: &Path,
    format: Format,
    role: Role,
    cycle: u32,
    session_output_file: &Path,
    timestamp: Option<String>,
    skip_super_step_check: bool,
    skip_channel_write: bool,
) -> Result<(), DriverError> {
    if !skip_super_step_check {
        match read_super_step_state(repo_root)? {
            None => return Err(DriverError::SuperStepNotInProgress),
            Some(state) => {
                if state.cycle != cycle || state.current_role != role {
                    return Err(DriverError::SuperStepMismatch {
                        invoked_role: role,
                        invoked_cycle: cycle,
                        current_role: state.current_role,
                        current_cycle: state.cycle,
                    });
                }
            }
        }
    }

    if !session_output_file.exists() {
        return Err(DriverError::SessionOutputMissing(
            session_output_file.to_path_buf(),
        ));
    }
    let raw = fs::read_to_string(session_output_file)?;
    let (payload_cycle, payload_timestamp, payload) = parse_session_output(&raw)?;

    if let Some(pc) = payload_cycle {
        if pc != cycle {
            return Err(DriverError::CycleMismatch {
                cli_cycle: cycle,
                payload_cycle: pc,
            });
        }
    }

    let channel = role.output_channel();
    validate_payload(channel, &payload)?;

    let effective_timestamp = timestamp
        .or(payload_timestamp)
        .unwrap_or_else(default_timestamp);

    let outcome;
    let notes;
    if skip_channel_write {
        outcome = Outcome::WriteSkipped;
        notes = format!(
            "validated session output from {} for channel '{}'; channel write skipped (--skip-channel-write)",
            session_output_file.display(),
            channel
        );
    } else {
        write_channel(
            repo_root,
            channel,
            role,
            cycle,
            &effective_timestamp,
            payload,
        )?;
        outcome = Outcome::Success;
        notes = format!(
            "wrote channel '{}' from session output {}",
            channel,
            session_output_file.display()
        );
    }

    append_role_history(
        repo_root,
        role,
        RoleRun {
            cycle,
            role,
            at: effective_timestamp.clone(),
            outcome,
            notes: notes.clone(),
        },
    )?;

    match format {
        Format::Text => {
            println!(
                "invoke: role={} cycle={} outcome={} timestamp={}",
                role.name(),
                cycle,
                serde_json::to_string(&outcome)
                    .unwrap_or_else(|_| "?".to_string())
                    .trim_matches('"'),
                effective_timestamp
            );
            println!("        {notes}");
        }
        Format::Json => {
            let out = serde_json::json!({
                "role": role.name(),
                "cycle": cycle,
                "outcome": outcome,
                "timestamp": effective_timestamp,
                "channel": channel,
                "channel_write_performed": !skip_channel_write,
                "super_step_check_performed": !skip_super_step_check,
                "notes": notes,
            });
            println!("{}", serde_json::to_string_pretty(&out)?);
        }
    }

    Ok(())
}

#[derive(Serialize)]
struct InputChannelReport {
    channel: &'static str,
    source: &'static str,
    state: ContextChannelState,
}

#[derive(Serialize)]
#[serde(rename_all = "kebab-case")]
enum ContextChannelState {
    NotInitialized,
    Empty,
    Populated {
        cycle: u32,
        writer: Role,
        timestamp: String,
        payload: serde_json::Value,
    },
}

#[derive(Serialize)]
struct ContextReport {
    role: Role,
    cycle: u32,
    prompt_path: PathBuf,
    prompt_present: bool,
    prompt_contents: Option<String>,
    inputs: Vec<InputChannelReport>,
}

fn cmd_context(
    repo_root: &Path,
    format: Format,
    role: Role,
    cycle: u32,
    prompt_file: Option<&Path>,
) -> Result<(), DriverError> {
    let prompt_path = match prompt_file {
        Some(p) => p.to_path_buf(),
        None => default_prompt_path(repo_root, role),
    };
    let prompt_contents = if prompt_path.exists() {
        Some(fs::read_to_string(&prompt_path)?)
    } else {
        None
    };

    let mut inputs = Vec::new();
    for binding in role.input_channels() {
        let state = match read_channel_state(repo_root, binding.channel)? {
            ChannelRead::NotInitialized => ContextChannelState::NotInitialized,
            ChannelRead::Empty => ContextChannelState::Empty,
            ChannelRead::Populated(s) => ContextChannelState::Populated {
                cycle: s.cycle,
                writer: s.writer,
                timestamp: s.timestamp,
                payload: s.payload,
            },
        };
        inputs.push(InputChannelReport {
            channel: binding.channel,
            source: binding.source.name(),
            state,
        });
    }

    let report = ContextReport {
        role,
        cycle,
        prompt_path: prompt_path.clone(),
        prompt_present: prompt_contents.is_some(),
        prompt_contents: prompt_contents.clone(),
        inputs,
    };

    match format {
        Format::Text => {
            println!("# role-driver context");
            println!("role: {}", role.name());
            println!("cycle: {cycle}");
            println!("prompt file: {}", prompt_path.display());
            if let Some(ref c) = prompt_contents {
                println!("prompt present: yes ({} bytes)", c.len());
            } else {
                println!("prompt present: NO (PROMPT-NOT-FOUND)");
            }
            if report.inputs.is_empty() {
                println!("inputs: (none — role polls external surfaces)");
            } else {
                println!("inputs:");
                for input in &report.inputs {
                    let label = match &input.state {
                        ContextChannelState::NotInitialized => "not initialized".to_string(),
                        ContextChannelState::Empty => "empty".to_string(),
                        ContextChannelState::Populated { cycle, writer, .. } => {
                            format!("populated (cycle {cycle} by {})", writer.name())
                        }
                    };
                    println!(
                        "  - channel={} source={} state={}",
                        input.channel, input.source, label
                    );
                }
            }
        }
        Format::Json => {
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
    }

    Ok(())
}

fn cmd_inputs(
    repo_root: &Path,
    format: Format,
    role: Role,
    cycle: Option<u32>,
) -> Result<(), DriverError> {
    let bindings = role.input_channels();

    #[derive(Serialize)]
    struct InputReport<'a> {
        role: Role,
        cycle: Option<u32>,
        output_channel: &'a str,
        inputs: Vec<InputRow<'a>>,
    }

    #[derive(Serialize)]
    struct InputRow<'a> {
        channel: &'a str,
        source: &'a str,
        state: Option<ContextChannelState>,
    }

    let mut rows = Vec::new();
    for binding in bindings {
        let state = if cycle.is_some() {
            match read_channel_state(repo_root, binding.channel)? {
                ChannelRead::NotInitialized => Some(ContextChannelState::NotInitialized),
                ChannelRead::Empty => Some(ContextChannelState::Empty),
                ChannelRead::Populated(s) => Some(ContextChannelState::Populated {
                    cycle: s.cycle,
                    writer: s.writer,
                    timestamp: s.timestamp,
                    payload: s.payload,
                }),
            }
        } else {
            None
        };
        rows.push(InputRow {
            channel: binding.channel,
            source: binding.source.name(),
            state,
        });
    }

    let report = InputReport {
        role,
        cycle,
        output_channel: role.output_channel(),
        inputs: rows,
    };

    match format {
        Format::Text => {
            println!("role: {}", role.name());
            println!("output channel (writes-to): {}", report.output_channel);
            if report.inputs.is_empty() {
                println!("inputs: (none — role polls external surfaces, not channels)");
            } else {
                println!("inputs (reads-from):");
                for row in &report.inputs {
                    print!("  - channel={} source={}", row.channel, row.source);
                    if let Some(ref s) = row.state {
                        let label = match s {
                            ContextChannelState::NotInitialized => "not-initialized",
                            ContextChannelState::Empty => "empty",
                            ContextChannelState::Populated { .. } => "populated",
                        };
                        print!(" state={label}");
                    }
                    println!();
                }
            }
        }
        Format::Json => {
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
    }

    Ok(())
}

fn cmd_history(
    repo_root: &Path,
    format: Format,
    role: Role,
    limit: Option<usize>,
) -> Result<(), DriverError> {
    let history = read_role_history(repo_root, role)?;
    let mut runs = history.runs.clone();
    if let Some(n) = limit {
        // newest-first when --limit is set, matching v2-super-step-boundary::history
        runs.reverse();
        runs.truncate(n);
    }
    match format {
        Format::Text => {
            if runs.is_empty() {
                println!("history for role '{}': (no runs)", role.name());
            } else {
                println!("history for role '{}':", role.name());
                for run in &runs {
                    println!(
                        "  - cycle={} at={} outcome={} notes={}",
                        run.cycle,
                        run.at,
                        serde_json::to_string(&run.outcome)
                            .unwrap_or_else(|_| "?".to_string())
                            .trim_matches('"'),
                        run.notes
                    );
                }
            }
        }
        Format::Json => {
            let out = serde_json::json!({
                "role": role.name(),
                "runs": runs,
            });
            println!("{}", serde_json::to_string_pretty(&out)?);
        }
    }
    Ok(())
}

fn cmd_schema(format: Format) -> Result<(), DriverError> {
    let role_specs: Vec<serde_json::Value> = Role::all()
        .iter()
        .map(|r| {
            let inputs: Vec<serde_json::Value> = r
                .input_channels()
                .iter()
                .map(|ib| {
                    serde_json::json!({
                        "channel": ib.channel,
                        "source": ib.source.name(),
                    })
                })
                .collect();
            serde_json::json!({
                "role": r.name(),
                "output_channel": r.output_channel(),
                "inputs": inputs,
                "required_payload_keys": required_payload_keys(r.output_channel()),
            })
        })
        .collect();
    let envelope = serde_json::json!({
        "roles": role_specs,
        "run_record": {
            "fields": ["cycle", "role", "at", "outcome", "notes"],
            "outcomes": ["success", "write-skipped"],
        },
        "paths": {
            "role_history": "state/roles/<role>-history.json",
            "super_step_state": "state/super-step.json",
            "channel_state": "state/channels/<channel>.json",
            "channel_history": "state/channels/<channel>-history.json",
            "default_prompt": "prompts/v2/<role>-prompt.xml",
        },
    });
    match format {
        Format::Text => {
            println!("v2-role-driver schema");
            println!("---------------------");
            for r in Role::all() {
                println!("role: {}", r.name());
                println!("  output channel: {}", r.output_channel());
                let inputs = r.input_channels();
                if inputs.is_empty() {
                    println!("  inputs: (none)");
                } else {
                    println!("  inputs:");
                    for ib in inputs {
                        println!("    - channel={} source={}", ib.channel, ib.source.name());
                    }
                }
                let keys = required_payload_keys(r.output_channel());
                if keys.is_empty() {
                    println!("  required payload keys for output channel: (none)");
                } else {
                    println!(
                        "  required payload keys for output channel: {}",
                        keys.join(", ")
                    );
                }
            }
            println!("run record fields: cycle, role, at, outcome, notes");
            println!("outcomes: success, write-skipped");
            println!("paths:");
            println!("  role history    = state/roles/<role>-history.json");
            println!("  super-step state= state/super-step.json");
            println!("  channel state   = state/channels/<channel>.json");
            println!("  channel history = state/channels/<channel>-history.json");
            println!("  default prompt  = prompts/v2/<role>-prompt.xml");
        }
        Format::Json => {
            println!("{}", serde_json::to_string_pretty(&envelope)?);
        }
    }
    Ok(())
}

// ----- main -----

fn main() -> ExitCode {
    let args = Args::parse();
    let repo_root = args.repo_root.as_path();
    let result = match args.command {
        Command::Init => cmd_init(repo_root, args.format),
        Command::Invoke {
            role,
            cycle,
            session_output_file,
            timestamp,
            skip_super_step_check,
            skip_channel_write,
        } => cmd_invoke(
            repo_root,
            args.format,
            role,
            cycle,
            &session_output_file,
            timestamp,
            skip_super_step_check,
            skip_channel_write,
        ),
        Command::Context {
            role,
            cycle,
            prompt_file,
        } => cmd_context(repo_root, args.format, role, cycle, prompt_file.as_deref()),
        Command::Inputs { role, cycle } => cmd_inputs(repo_root, args.format, role, cycle),
        Command::History { role, limit } => cmd_history(repo_root, args.format, role, limit),
        Command::Schema => cmd_schema(args.format),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("v2-role-driver: {e}");
            ExitCode::FAILURE
        }
    }
}

// ----- unit tests -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_names_are_kebab_case() {
        assert_eq!(Role::Reconciler.name(), "reconciler");
        assert_eq!(Role::Planner.name(), "planner");
        assert_eq!(Role::Executor.name(), "executor");
        assert_eq!(Role::Curator.name(), "curator");
    }

    #[test]
    fn role_output_channels_are_distinct() {
        use std::collections::HashSet;
        let channels: HashSet<&str> = Role::all().iter().map(|r| r.output_channel()).collect();
        assert_eq!(
            channels.len(),
            4,
            "each role must have a unique output channel"
        );
    }

    #[test]
    fn role_output_channel_matches_super_step_boundary_convention() {
        // Mirrors v2-super-step-boundary::Role::output_channel verbatim.
        assert_eq!(Role::Reconciler.output_channel(), "inbound-channel");
        assert_eq!(Role::Planner.output_channel(), "plan-channel");
        assert_eq!(Role::Executor.output_channel(), "work-channel");
        assert_eq!(Role::Curator.output_channel(), "memory-channel");
    }

    #[test]
    fn role_input_bindings_per_cycle_139_scoping() {
        let p = Role::Planner.input_channels();
        assert_eq!(p.len(), 2, "planner reads memory + inbound");
        assert!(p
            .iter()
            .any(|b| b.channel == "memory-channel" && b.source == InputSource::PreviousCycle));
        assert!(p
            .iter()
            .any(|b| b.channel == "inbound-channel" && b.source == InputSource::CurrentCycle));

        let e = Role::Executor.input_channels();
        assert_eq!(e.len(), 1, "executor reads plan-channel");
        assert_eq!(e[0].channel, "plan-channel");
        assert_eq!(e[0].source, InputSource::CurrentCycle);

        let c = Role::Curator.input_channels();
        assert_eq!(c.len(), 1, "curator reads work-channel");
        assert_eq!(c[0].channel, "work-channel");

        let r = Role::Reconciler.input_channels();
        assert!(r.is_empty(), "reconciler has no channel inputs");
    }

    #[test]
    fn required_payload_keys_align_with_channel_router() {
        assert_eq!(
            required_payload_keys("plan-channel"),
            &["substantive-focal", "per-role-tasks"]
        );
        assert_eq!(
            required_payload_keys("work-channel"),
            &["artifacts-written"]
        );
        assert_eq!(
            required_payload_keys("memory-channel"),
            &["consolidated-insights"]
        );
        assert_eq!(
            required_payload_keys("inbound-channel"),
            &[
                "eva-responses",
                "audit-posts",
                "dispatch-returns",
                "inbound-completeness-marker",
            ]
        );
        assert!(required_payload_keys("nonexistent").is_empty());
    }

    #[test]
    fn validate_payload_rejects_non_object() {
        let arr = serde_json::json!([1, 2, 3]);
        let err = validate_payload("plan-channel", &arr).unwrap_err();
        match err {
            DriverError::InvalidSessionOutput(msg) => {
                assert!(msg.contains("array"));
                assert!(msg.contains("plan-channel"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn validate_payload_rejects_missing_keys() {
        let obj = serde_json::json!({"substantive-focal": "x"}); // missing per-role-tasks
        let err = validate_payload("plan-channel", &obj).unwrap_err();
        match err {
            DriverError::InvalidSessionOutput(msg) => {
                assert!(msg.contains("per-role-tasks"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn validate_payload_passes_when_keys_present() {
        let obj = serde_json::json!({
            "substantive-focal": "x",
            "per-role-tasks": {},
        });
        validate_payload("plan-channel", &obj).expect("valid plan-channel payload");
    }

    #[test]
    fn parse_session_output_full_envelope() {
        let raw = r#"{
            "cycle": 42,
            "timestamp": "2026-05-14T05:00:00Z",
            "payload": {"substantive-focal": "x", "per-role-tasks": {}}
        }"#;
        let (cycle, ts, payload) = parse_session_output(raw).unwrap();
        assert_eq!(cycle, Some(42));
        assert_eq!(ts.as_deref(), Some("2026-05-14T05:00:00Z"));
        assert!(payload.is_object());
    }

    #[test]
    fn parse_session_output_payload_only() {
        let raw = r#"{ "payload": {"k": "v"} }"#;
        let (cycle, ts, payload) = parse_session_output(raw).unwrap();
        assert_eq!(cycle, None);
        assert_eq!(ts, None);
        assert_eq!(payload, serde_json::json!({"k": "v"}));
    }

    #[test]
    fn parse_session_output_bare_payload() {
        let raw = r#"{ "consolidated-insights": [] }"#;
        let (cycle, ts, payload) = parse_session_output(raw).unwrap();
        assert_eq!(cycle, None);
        assert_eq!(ts, None);
        assert_eq!(payload, serde_json::json!({"consolidated-insights": []}));
    }

    #[test]
    fn parse_session_output_rejects_non_object() {
        let raw = r#"["not", "an", "object"]"#;
        let err = parse_session_output(raw).unwrap_err();
        match err {
            DriverError::InvalidSessionOutput(_) => {}
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn parse_session_output_rejects_non_json() {
        let raw = "definitely not json";
        let err = parse_session_output(raw).unwrap_err();
        match err {
            DriverError::InvalidSessionOutput(_) => {}
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn role_history_path_is_under_state_roles() {
        let p = role_history_path(Path::new("/x"), Role::Planner);
        assert_eq!(p, PathBuf::from("/x/state/roles/planner-history.json"),);
    }

    #[test]
    fn default_prompt_path_kebab_case() {
        let p = default_prompt_path(Path::new("/x"), Role::Reconciler);
        assert_eq!(p, PathBuf::from("/x/prompts/v2/reconciler-prompt.xml"),);
    }

    #[test]
    fn role_history_serde_roundtrip() {
        let h = RoleHistory {
            role: Some(Role::Curator),
            runs: vec![RoleRun {
                cycle: 1,
                role: Role::Curator,
                at: "now".into(),
                outcome: Outcome::Success,
                notes: "test".into(),
            }],
        };
        let s = serde_json::to_string(&h).unwrap();
        let r: RoleHistory = serde_json::from_str(&s).unwrap();
        assert_eq!(r.runs.len(), 1);
        assert_eq!(r.runs[0].outcome, Outcome::Success);
    }
}
