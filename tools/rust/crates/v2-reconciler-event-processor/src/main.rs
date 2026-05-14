use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "v2-reconciler-event-processor",
    about = "Polls cron-internal sources (input-from-eva / audit-repo cursor / dispatch-returns) \
             and writes the `inbound-channel` for the v2 multi-agent orchestrator. \
             SCAFFOLD scope: source events are supplied via per-source files; live GH API + \
             audit-repo subprocess + classifier work are DEFERRED to COMPLETE arc."
)]
struct Args {
    /// Repository root (path containing state/).
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
    /// Initialize state/reconciler/ with per-source cursor files + poll history.
    Init,
    /// Run a poll: read each source file, filter events past its cursor, build the
    /// inbound-channel payload, write channel state + append channel history, update
    /// cursors, append poll history.
    ///
    /// SCAFFOLD scope: events are supplied via `--{source}-source-file <path>`; each
    /// source file is `{ "source": "eva|audit|dispatch", "events": [...] }`. DEFERRED
    /// to COMPLETE arc: live GH API + audit-repo subprocess + dispatch-PR enumeration +
    /// semantic event classification per source.
    Poll {
        #[arg(long)]
        cycle: u32,
        /// Path to a JSON file containing eva events. Optional; absent means zero events.
        #[arg(long)]
        eva_source_file: Option<PathBuf>,
        /// Path to a JSON file containing audit events.
        #[arg(long)]
        audit_source_file: Option<PathBuf>,
        /// Path to a JSON file containing dispatch events.
        #[arg(long)]
        dispatch_source_file: Option<PathBuf>,
        /// Timestamp to record (ISO-8601). Defaults to a sentinel string; callers
        /// SHOULD supply a real timestamp.
        #[arg(long)]
        timestamp: Option<String>,
        /// Skip super-step verification (for hermetic tests).
        #[arg(long)]
        skip_super_step_check: bool,
        /// Build the payload but do not write the inbound-channel (still appends poll
        /// history with outcome `write-skipped`).
        #[arg(long)]
        skip_channel_write: bool,
    },
    /// Print the current cursor for each source.
    Cursors,
    /// Print the per-poll history.
    History {
        /// Limit entries; newest first when set.
        #[arg(long)]
        limit: Option<usize>,
    },
    /// Print the schema (source kinds, cursor format, inbound-channel binding).
    Schema,
    /// Read the inbound-channel state for a given cycle (read-only convenience).
    Inbound {
        #[arg(long)]
        cycle: Option<u32>,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
enum Format {
    Text,
    Json,
}

// ----- Source -----

#[derive(Copy, Clone, Debug, ValueEnum, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
enum Source {
    Eva,
    Audit,
    Dispatch,
}

impl Source {
    fn name(self) -> &'static str {
        match self {
            Source::Eva => "eva",
            Source::Audit => "audit",
            Source::Dispatch => "dispatch",
        }
    }

    fn all() -> &'static [Source] {
        &[Source::Eva, Source::Audit, Source::Dispatch]
    }

    /// Key under which this source's events are placed in the inbound-channel payload.
    /// Mirrors v2-channel-router::Channel::required_payload_keys for `inbound-channel`.
    fn output_key(self) -> &'static str {
        match self {
            Source::Eva => "eva-responses",
            Source::Audit => "audit-posts",
            Source::Dispatch => "dispatch-returns",
        }
    }

    /// Documentation-only kind hint for the cursor value (printed by `schema`).
    /// SCAFFOLD treats cursors as opaque strings; COMPLETE arc may add source-aware
    /// ordering (numeric for issue/PR numbers, git-topology for audit SHAs).
    fn cursor_kind(self) -> &'static str {
        match self {
            Source::Eva => "issue-number",
            Source::Audit => "commit-sha",
            Source::Dispatch => "pr-number",
        }
    }
}

// ----- Role (subset needed for inbound-channel writer + super-step verification) -----

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
enum Role {
    Reconciler,
    Planner,
    Executor,
    Curator,
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
}

const INBOUND_CHANNEL: &str = "inbound-channel";

// ----- Cursors -----

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct Cursor {
    source: Source,
    /// Opaque high-water-mark id. `None` means "never polled" (cycle 1 sentinel).
    value: Option<String>,
}

// ----- Source files (events received by SCAFFOLD via files) -----

#[derive(Deserialize, Debug, Clone)]
struct SourceFile {
    /// Optional self-identification; if present, must match the source the file was
    /// passed for. Defensive cross-check; absent means "no self-identification."
    #[serde(default)]
    source: Option<Source>,
    events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Event {
    id: String,
    /// Free-form timestamp; pass-through into the payload.
    at: String,
    /// Free-form per-event payload. Classifier work that interprets `raw` is DEFERRED
    /// to cycle 150+ COMPLETE arc.
    #[serde(default)]
    raw: serde_json::Value,
}

// ----- Poll history -----

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum PollOutcome {
    Success,
    WriteSkipped,
}

impl PollOutcome {
    fn name(self) -> &'static str {
        match self {
            PollOutcome::Success => "success",
            PollOutcome::WriteSkipped => "write-skipped",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct SourceSummary {
    /// Events read from the source file (or 0 if the file was absent).
    events_in: usize,
    /// Events that survived cursor filtering and were placed in the inbound payload.
    events_new: usize,
    cursor_before: Option<String>,
    cursor_after: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PollHistoryEntry {
    cycle: u32,
    at: String,
    eva: SourceSummary,
    audit: SourceSummary,
    dispatch: SourceSummary,
    outcome: PollOutcome,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct PollHistory {
    entries: Vec<PollHistoryEntry>,
}

// ----- Channel envelopes (mirror v2-channel-router) -----

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ChannelState {
    channel: String,
    writer: Role,
    cycle: u32,
    timestamp: String,
    payload: serde_json::Value,
}

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

// ----- Super-step verification subset -----

#[derive(Deserialize, Debug)]
struct SuperStepStateLite {
    cycle: u32,
    current_role: Role,
}

// ----- Errors -----

#[derive(Debug)]
enum ProcessorError {
    Io(io::Error),
    Json(String),
    /// state/reconciler/ does not exist (init has not been run).
    NotInitialized(PathBuf),
    /// state/channels/ does not exist (v2-channel-router init has not been run).
    ChannelsNotInitialized(PathBuf),
    /// `state/super-step.json` absent or empty when --skip-super-step-check is off.
    SuperStepNotInProgress,
    /// Super-step state exists but does not match invocation (role or cycle).
    SuperStepMismatch {
        invoked_cycle: u32,
        current_role: Role,
        current_cycle: u32,
    },
    /// A source file passed via `--{source}-source-file` does not exist.
    SourceFileMissing(Source, PathBuf),
    /// A source file could not be parsed (invalid JSON or wrong shape).
    InvalidSourceFile(Source, PathBuf, String),
    /// A cursor file on disk could not be parsed.
    CursorCorrupt(Source, PathBuf, String),
    /// Inbound payload validation failed. Defensive; should not happen in normal flow
    /// since the crate builds the payload itself.
    InvalidInboundPayload(String),
}

impl std::fmt::Display for ProcessorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProcessorError::Io(e) => write!(f, "{e}"),
            ProcessorError::Json(s) => write!(f, "json error: {s}"),
            ProcessorError::NotInitialized(p) => write!(
                f,
                "reconciler-event-processor state not initialized at {} \
                 (run `v2-reconciler-event-processor init`)",
                p.display()
            ),
            ProcessorError::ChannelsNotInitialized(p) => write!(
                f,
                "channels state not initialized at {} \
                 (run `v2-channel-router init` before polling)",
                p.display()
            ),
            ProcessorError::SuperStepNotInProgress => write!(
                f,
                "no super-step in progress: run `v2-super-step-boundary cycle-start --cycle N` first \
                 (or pass `--skip-super-step-check` for hermetic mode)"
            ),
            ProcessorError::SuperStepMismatch {
                invoked_cycle,
                current_role,
                current_cycle,
            } => write!(
                f,
                "super-step mismatch: invoked reconciler for cycle {invoked_cycle} \
                 but current super-step is role '{}' for cycle {current_cycle}",
                current_role.name()
            ),
            ProcessorError::SourceFileMissing(src, p) => write!(
                f,
                "source file for '{}' not found: {}",
                src.name(),
                p.display()
            ),
            ProcessorError::InvalidSourceFile(src, p, msg) => write!(
                f,
                "invalid source file for '{}' at {}: {msg}",
                src.name(),
                p.display()
            ),
            ProcessorError::CursorCorrupt(src, p, msg) => write!(
                f,
                "cursor file for '{}' at {} could not be parsed: {msg}",
                src.name(),
                p.display()
            ),
            ProcessorError::InvalidInboundPayload(s) => {
                write!(f, "invalid inbound-channel payload: {s}")
            }
        }
    }
}

impl From<io::Error> for ProcessorError {
    fn from(e: io::Error) -> Self {
        ProcessorError::Io(e)
    }
}

impl From<serde_json::Error> for ProcessorError {
    fn from(e: serde_json::Error) -> Self {
        ProcessorError::Json(e.to_string())
    }
}

// ----- paths -----

fn state_dir(repo_root: &Path) -> PathBuf {
    repo_root.join("state")
}

fn reconciler_dir(repo_root: &Path) -> PathBuf {
    state_dir(repo_root).join("reconciler")
}

fn cursor_path(repo_root: &Path, source: Source) -> PathBuf {
    reconciler_dir(repo_root).join(format!("{}-cursor.json", source.name()))
}

fn poll_history_path(repo_root: &Path) -> PathBuf {
    reconciler_dir(repo_root).join("poll-history.json")
}

fn channels_dir(repo_root: &Path) -> PathBuf {
    state_dir(repo_root).join("channels")
}

fn inbound_channel_state_path(repo_root: &Path) -> PathBuf {
    channels_dir(repo_root).join(format!("{INBOUND_CHANNEL}.json"))
}

fn inbound_channel_history_path(repo_root: &Path) -> PathBuf {
    channels_dir(repo_root).join(format!("{INBOUND_CHANNEL}-history.json"))
}

fn super_step_state_path(repo_root: &Path) -> PathBuf {
    state_dir(repo_root).join("super-step.json")
}

fn default_timestamp() -> String {
    "1970-01-01T00:00:00Z".to_string()
}

// ----- atomic write -----

/// Atomic write via temp-file + rename. Mirrors v2-channel-router / v2-super-step-boundary /
/// v2-role-driver's identical helper.
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

// ----- cursor read/write -----

fn read_cursor(repo_root: &Path, source: Source) -> Result<Cursor, ProcessorError> {
    let path = cursor_path(repo_root, source);
    if !path.exists() {
        return Err(ProcessorError::NotInitialized(reconciler_dir(repo_root)));
    }
    let raw = fs::read_to_string(&path)?;
    if raw.trim().is_empty() {
        return Ok(Cursor {
            source,
            value: None,
        });
    }
    let cursor: Cursor = serde_json::from_str(&raw)
        .map_err(|e| ProcessorError::CursorCorrupt(source, path.clone(), e.to_string()))?;
    if cursor.source != source {
        return Err(ProcessorError::CursorCorrupt(
            source,
            path,
            format!(
                "cursor file declares source '{}' but file path implies '{}'",
                cursor.source.name(),
                source.name()
            ),
        ));
    }
    Ok(cursor)
}

fn write_cursor(repo_root: &Path, cursor: &Cursor) -> Result<(), ProcessorError> {
    let path = cursor_path(repo_root, cursor.source);
    let body = serde_json::to_string_pretty(cursor)?;
    write_atomic(&path, body.as_bytes())?;
    Ok(())
}

// ----- source file read + cursor filtering -----

fn read_source_file(
    source: Source,
    path: Option<&Path>,
) -> Result<Vec<Event>, ProcessorError> {
    let Some(path) = path else {
        return Ok(Vec::new());
    };
    if !path.exists() {
        return Err(ProcessorError::SourceFileMissing(source, path.to_path_buf()));
    }
    let raw = fs::read_to_string(path)?;
    let file: SourceFile = serde_json::from_str(&raw).map_err(|e| {
        ProcessorError::InvalidSourceFile(source, path.to_path_buf(), e.to_string())
    })?;
    if let Some(declared) = file.source {
        if declared != source {
            return Err(ProcessorError::InvalidSourceFile(
                source,
                path.to_path_buf(),
                format!(
                    "source file declares source '{}' but was passed via the '{}' flag",
                    declared.name(),
                    source.name()
                ),
            ));
        }
    }
    Ok(file.events)
}

/// Filter events past the cursor's high-water mark. SCAFFOLD treats cursors as opaque
/// strings with lexicographic ordering; if `cursor.value` is `Some(s)`, only events
/// with `id > s` lex are retained. If `cursor.value` is `None`, all events are retained.
/// DEFERRED to COMPLETE arc: source-aware ordering (numeric for issue/PR numbers,
/// git-topology for audit SHAs).
fn filter_events_past_cursor(events: Vec<Event>, cursor: &Cursor) -> Vec<Event> {
    let Some(cv) = cursor.value.as_deref() else {
        return events;
    };
    events
        .into_iter()
        .filter(|e| e.id.as_str() > cv)
        .collect()
}

/// Highest event id (lex max) among the supplied events, or `None` if events is empty.
/// Used to advance the cursor after a successful poll.
fn highest_event_id(events: &[Event]) -> Option<String> {
    events.iter().map(|e| e.id.clone()).max()
}

// ----- super-step state read -----

fn read_super_step_state(repo_root: &Path) -> Result<Option<SuperStepStateLite>, ProcessorError> {
    let path = super_step_state_path(repo_root);
    if !path.exists() {
        return Err(ProcessorError::SuperStepNotInProgress);
    }
    let raw = fs::read_to_string(&path)?;
    if raw.trim().is_empty() || raw.trim() == "null" {
        return Ok(None);
    }
    let state: SuperStepStateLite = serde_json::from_str(&raw)
        .map_err(|e| ProcessorError::Json(format!("decoding {}: {e}", path.display())))?;
    Ok(Some(state))
}

fn verify_super_step(repo_root: &Path, invoked_cycle: u32) -> Result<(), ProcessorError> {
    let state = read_super_step_state(repo_root)?;
    let Some(state) = state else {
        return Err(ProcessorError::SuperStepNotInProgress);
    };
    if state.current_role != Role::Reconciler || state.cycle != invoked_cycle {
        return Err(ProcessorError::SuperStepMismatch {
            invoked_cycle,
            current_role: state.current_role,
            current_cycle: state.cycle,
        });
    }
    Ok(())
}

// ----- inbound-channel write (local reducer-rule duplicating v2-channel-router) -----

/// Validate the inbound-channel payload. The 3 required keys (`eva-responses`,
/// `audit-posts`, `dispatch-returns`) must be present and must be arrays.
fn validate_inbound_payload(payload: &serde_json::Value) -> Result<(), ProcessorError> {
    let obj = payload.as_object().ok_or_else(|| {
        ProcessorError::InvalidInboundPayload(format!(
            "payload must be a JSON object, got {}",
            describe_json_type(payload)
        ))
    })?;
    for key in ["eva-responses", "audit-posts", "dispatch-returns"] {
        let v = obj.get(key).ok_or_else(|| {
            ProcessorError::InvalidInboundPayload(format!("missing required key '{key}'"))
        })?;
        if !v.is_array() {
            return Err(ProcessorError::InvalidInboundPayload(format!(
                "key '{key}' must be an array, got {}",
                describe_json_type(v)
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

fn write_inbound_channel(
    repo_root: &Path,
    cycle: u32,
    timestamp: &str,
    payload: serde_json::Value,
) -> Result<(), ProcessorError> {
    // Verify the channels state directory exists (v2-channel-router init has been run).
    if !channels_dir(repo_root).exists() {
        return Err(ProcessorError::ChannelsNotInitialized(channels_dir(repo_root)));
    }

    validate_inbound_payload(&payload)?;

    let state = ChannelState {
        channel: INBOUND_CHANNEL.to_string(),
        writer: Role::Reconciler,
        cycle,
        timestamp: timestamp.to_string(),
        payload: payload.clone(),
    };

    let state_path = inbound_channel_state_path(repo_root);
    let state_body = serde_json::to_string_pretty(&state)?;
    write_atomic(&state_path, state_body.as_bytes())?;

    let history_path = inbound_channel_history_path(repo_root);
    let mut history: ChannelHistory = if history_path.exists() {
        let raw = fs::read_to_string(&history_path)?;
        if raw.trim().is_empty() {
            ChannelHistory {
                channel: INBOUND_CHANNEL.to_string(),
                entries: Vec::new(),
            }
        } else {
            serde_json::from_str(&raw).map_err(|e| {
                ProcessorError::Json(format!("decoding {}: {e}", history_path.display()))
            })?
        }
    } else {
        ChannelHistory {
            channel: INBOUND_CHANNEL.to_string(),
            entries: Vec::new(),
        }
    };
    history.entries.push(ChannelHistoryEntry {
        writer: Role::Reconciler,
        cycle,
        timestamp: timestamp.to_string(),
        payload,
    });
    let history_body = serde_json::to_string_pretty(&history)?;
    write_atomic(&history_path, history_body.as_bytes())?;

    Ok(())
}

// ----- poll-history append -----

fn read_poll_history(repo_root: &Path) -> Result<PollHistory, ProcessorError> {
    let path = poll_history_path(repo_root);
    if !path.exists() {
        return Err(ProcessorError::NotInitialized(reconciler_dir(repo_root)));
    }
    let raw = fs::read_to_string(&path)?;
    if raw.trim().is_empty() {
        return Ok(PollHistory::default());
    }
    let history: PollHistory = serde_json::from_str(&raw)
        .map_err(|e| ProcessorError::Json(format!("decoding {}: {e}", path.display())))?;
    Ok(history)
}

fn append_poll_history(
    repo_root: &Path,
    entry: PollHistoryEntry,
) -> Result<(), ProcessorError> {
    let mut history = read_poll_history(repo_root)?;
    history.entries.push(entry);
    let path = poll_history_path(repo_root);
    let body = serde_json::to_string_pretty(&history)?;
    write_atomic(&path, body.as_bytes())?;
    Ok(())
}

// ----- subcommands -----

#[derive(Serialize)]
struct InitResult {
    created: Vec<PathBuf>,
    already_present: Vec<PathBuf>,
}

fn cmd_init(repo_root: &Path) -> Result<InitResult, ProcessorError> {
    fs::create_dir_all(reconciler_dir(repo_root))?;
    let mut created = Vec::new();
    let mut already_present = Vec::new();

    for src in Source::all() {
        let path = cursor_path(repo_root, *src);
        if path.exists() {
            already_present.push(path);
        } else {
            let cursor = Cursor {
                source: *src,
                value: None,
            };
            let body = serde_json::to_string_pretty(&cursor)?;
            write_atomic(&path, body.as_bytes())?;
            created.push(path);
        }
    }

    let history_path = poll_history_path(repo_root);
    if history_path.exists() {
        already_present.push(history_path);
    } else {
        let body = serde_json::to_string_pretty(&PollHistory::default())?;
        write_atomic(&history_path, body.as_bytes())?;
        created.push(history_path);
    }

    Ok(InitResult {
        created,
        already_present,
    })
}

#[derive(Serialize)]
struct PollResult {
    cycle: u32,
    at: String,
    eva: SourceSummary,
    audit: SourceSummary,
    dispatch: SourceSummary,
    outcome: PollOutcome,
    super_step_check_performed: bool,
    inbound_payload: serde_json::Value,
}

#[allow(clippy::too_many_arguments)]
fn cmd_poll(
    repo_root: &Path,
    cycle: u32,
    eva_source_file: Option<&Path>,
    audit_source_file: Option<&Path>,
    dispatch_source_file: Option<&Path>,
    timestamp: Option<String>,
    skip_super_step_check: bool,
    skip_channel_write: bool,
) -> Result<PollResult, ProcessorError> {
    let timestamp = timestamp.unwrap_or_else(default_timestamp);

    // Ensure init has been run (cursor files must exist).
    if !reconciler_dir(repo_root).exists() {
        return Err(ProcessorError::NotInitialized(reconciler_dir(repo_root)));
    }

    // Super-step verification (unless bypassed).
    if !skip_super_step_check {
        verify_super_step(repo_root, cycle)?;
    }

    // Per-source: read cursor, read source file, filter events, compute new cursor.
    let (eva_events, eva_summary, eva_new_cursor) =
        process_source(repo_root, Source::Eva, eva_source_file)?;
    let (audit_events, audit_summary, audit_new_cursor) =
        process_source(repo_root, Source::Audit, audit_source_file)?;
    let (dispatch_events, dispatch_summary, dispatch_new_cursor) =
        process_source(repo_root, Source::Dispatch, dispatch_source_file)?;

    // Build inbound payload.
    let payload = serde_json::json!({
        Source::Eva.output_key(): eva_events,
        Source::Audit.output_key(): audit_events,
        Source::Dispatch.output_key(): dispatch_events,
    });

    // Validate (defensive — payload is constructed by this crate).
    validate_inbound_payload(&payload)?;

    let outcome = if skip_channel_write {
        PollOutcome::WriteSkipped
    } else {
        write_inbound_channel(repo_root, cycle, &timestamp, payload.clone())?;
        // On successful channel write, persist updated cursors.
        write_cursor(repo_root, &eva_new_cursor)?;
        write_cursor(repo_root, &audit_new_cursor)?;
        write_cursor(repo_root, &dispatch_new_cursor)?;
        PollOutcome::Success
    };

    let entry = PollHistoryEntry {
        cycle,
        at: timestamp.clone(),
        eva: eva_summary.clone(),
        audit: audit_summary.clone(),
        dispatch: dispatch_summary.clone(),
        outcome,
    };
    append_poll_history(repo_root, entry)?;

    Ok(PollResult {
        cycle,
        at: timestamp,
        eva: eva_summary,
        audit: audit_summary,
        dispatch: dispatch_summary,
        outcome,
        super_step_check_performed: !skip_super_step_check,
        inbound_payload: payload,
    })
}

/// For a single source: read the prior cursor, read the source file (if any), filter
/// events past the cursor, compute the new cursor (max id over surviving events; if
/// none, unchanged). Returns the events that will be written into the inbound payload,
/// a per-source summary, and the new cursor (NOT yet persisted to disk; caller
/// persists only on successful channel write).
fn process_source(
    repo_root: &Path,
    source: Source,
    source_file: Option<&Path>,
) -> Result<(Vec<Event>, SourceSummary, Cursor), ProcessorError> {
    let cursor_before = read_cursor(repo_root, source)?;
    let all_events = read_source_file(source, source_file)?;
    let events_in = all_events.len();
    let surviving = filter_events_past_cursor(all_events, &cursor_before);
    let events_new = surviving.len();
    let new_high = highest_event_id(&surviving);
    let cursor_after_value = match (cursor_before.value.clone(), new_high.clone()) {
        (Some(prev), Some(new)) => {
            // Lex max of prev and new (defensive: new should be > prev by filter, but
            // be explicit).
            Some(if new > prev { new } else { prev })
        }
        (None, Some(new)) => Some(new),
        (Some(prev), None) => Some(prev),
        (None, None) => None,
    };
    let new_cursor = Cursor {
        source,
        value: cursor_after_value.clone(),
    };
    let summary = SourceSummary {
        events_in,
        events_new,
        cursor_before: cursor_before.value,
        cursor_after: cursor_after_value,
    };
    Ok((surviving, summary, new_cursor))
}

#[derive(Serialize)]
struct CursorsOutput {
    cursors: Vec<Cursor>,
}

fn cmd_cursors(repo_root: &Path) -> Result<CursorsOutput, ProcessorError> {
    let mut cursors = Vec::new();
    for src in Source::all() {
        cursors.push(read_cursor(repo_root, *src)?);
    }
    Ok(CursorsOutput { cursors })
}

#[derive(Serialize)]
struct HistoryOutput {
    entries: Vec<PollHistoryEntry>,
}

fn cmd_history(repo_root: &Path, limit: Option<usize>) -> Result<HistoryOutput, ProcessorError> {
    let history = read_poll_history(repo_root)?;
    let entries = match limit {
        None => history.entries,
        Some(n) => {
            let mut e = history.entries;
            e.reverse();
            e.truncate(n);
            e
        }
    };
    Ok(HistoryOutput { entries })
}

#[derive(Serialize)]
struct SchemaSourceEntry {
    source: Source,
    output_key: &'static str,
    cursor_kind: &'static str,
    cursor_path_template: String,
}

#[derive(Serialize)]
struct SchemaOutput {
    sources: Vec<SchemaSourceEntry>,
    inbound_channel: &'static str,
    inbound_writer: Role,
    inbound_required_keys: Vec<&'static str>,
    poll_outcomes: Vec<&'static str>,
    poll_history_path_template: String,
}

fn cmd_schema() -> SchemaOutput {
    let sources = Source::all()
        .iter()
        .map(|s| SchemaSourceEntry {
            source: *s,
            output_key: s.output_key(),
            cursor_kind: s.cursor_kind(),
            cursor_path_template: format!("state/reconciler/{}-cursor.json", s.name()),
        })
        .collect();
    SchemaOutput {
        sources,
        inbound_channel: INBOUND_CHANNEL,
        inbound_writer: Role::Reconciler,
        inbound_required_keys: vec!["eva-responses", "audit-posts", "dispatch-returns"],
        poll_outcomes: vec![PollOutcome::Success.name(), PollOutcome::WriteSkipped.name()],
        poll_history_path_template: "state/reconciler/poll-history.json".to_string(),
    }
}

#[derive(Serialize)]
struct InboundOutput {
    state: Option<ChannelState>,
    cycle_filter: Option<u32>,
    matched: bool,
}

fn cmd_inbound(repo_root: &Path, cycle: Option<u32>) -> Result<InboundOutput, ProcessorError> {
    let path = inbound_channel_state_path(repo_root);
    if !path.exists() {
        return Ok(InboundOutput {
            state: None,
            cycle_filter: cycle,
            matched: false,
        });
    }
    let raw = fs::read_to_string(&path)?;
    if raw.trim().is_empty() || raw.trim() == "null" {
        return Ok(InboundOutput {
            state: None,
            cycle_filter: cycle,
            matched: false,
        });
    }
    let state: ChannelState = serde_json::from_str(&raw)
        .map_err(|e| ProcessorError::Json(format!("decoding {}: {e}", path.display())))?;
    let matched = match cycle {
        None => true,
        Some(c) => state.cycle == c,
    };
    if matched {
        Ok(InboundOutput {
            state: Some(state),
            cycle_filter: cycle,
            matched: true,
        })
    } else {
        Ok(InboundOutput {
            state: None,
            cycle_filter: cycle,
            matched: false,
        })
    }
}

// ----- main + output rendering -----

fn main() -> ExitCode {
    let args = Args::parse();
    let result = dispatch(&args);
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            let stderr = io::stderr();
            let mut h = stderr.lock();
            let _ = writeln!(h, "error: {e}");
            ExitCode::from(1)
        }
    }
}

fn dispatch(args: &Args) -> Result<(), ProcessorError> {
    let repo = &args.repo_root;
    match &args.command {
        Command::Init => {
            let r = cmd_init(repo)?;
            render_init(&r, args.format)?;
        }
        Command::Poll {
            cycle,
            eva_source_file,
            audit_source_file,
            dispatch_source_file,
            timestamp,
            skip_super_step_check,
            skip_channel_write,
        } => {
            let r = cmd_poll(
                repo,
                *cycle,
                eva_source_file.as_deref(),
                audit_source_file.as_deref(),
                dispatch_source_file.as_deref(),
                timestamp.clone(),
                *skip_super_step_check,
                *skip_channel_write,
            )?;
            render_poll(&r, args.format)?;
        }
        Command::Cursors => {
            let r = cmd_cursors(repo)?;
            render_cursors(&r, args.format)?;
        }
        Command::History { limit } => {
            let r = cmd_history(repo, *limit)?;
            render_history(&r, args.format)?;
        }
        Command::Schema => {
            let r = cmd_schema();
            render_schema(&r, args.format)?;
        }
        Command::Inbound { cycle } => {
            let r = cmd_inbound(repo, *cycle)?;
            render_inbound(&r, args.format)?;
        }
    }
    Ok(())
}

fn render_init(r: &InitResult, format: Format) -> Result<(), ProcessorError> {
    let stdout = io::stdout();
    let mut h = stdout.lock();
    match format {
        Format::Json => {
            writeln!(h, "{}", serde_json::to_string_pretty(r)?)?;
        }
        Format::Text => {
            if r.created.is_empty() && r.already_present.is_empty() {
                writeln!(h, "init: nothing to do")?;
            } else {
                for p in &r.created {
                    writeln!(h, "created: {}", p.display())?;
                }
                for p in &r.already_present {
                    writeln!(h, "already-present: {}", p.display())?;
                }
            }
        }
    }
    Ok(())
}

fn render_poll(r: &PollResult, format: Format) -> Result<(), ProcessorError> {
    let stdout = io::stdout();
    let mut h = stdout.lock();
    match format {
        Format::Json => {
            writeln!(h, "{}", serde_json::to_string_pretty(r)?)?;
        }
        Format::Text => {
            writeln!(h, "cycle:                       {}", r.cycle)?;
            writeln!(h, "at:                          {}", r.at)?;
            writeln!(h, "outcome:                     {}", r.outcome.name())?;
            writeln!(
                h,
                "super-step check performed:  {}",
                r.super_step_check_performed
            )?;
            for (name, s) in [
                ("eva", &r.eva),
                ("audit", &r.audit),
                ("dispatch", &r.dispatch),
            ] {
                writeln!(
                    h,
                    "  {name:<10} events-in={} events-new={} cursor={} -> {}",
                    s.events_in,
                    s.events_new,
                    s.cursor_before.as_deref().unwrap_or("<unset>"),
                    s.cursor_after.as_deref().unwrap_or("<unset>"),
                )?;
            }
        }
    }
    Ok(())
}

fn render_cursors(r: &CursorsOutput, format: Format) -> Result<(), ProcessorError> {
    let stdout = io::stdout();
    let mut h = stdout.lock();
    match format {
        Format::Json => {
            writeln!(h, "{}", serde_json::to_string_pretty(r)?)?;
        }
        Format::Text => {
            for c in &r.cursors {
                writeln!(
                    h,
                    "{:<10} {}",
                    c.source.name(),
                    c.value.as_deref().unwrap_or("<unset>")
                )?;
            }
        }
    }
    Ok(())
}

fn render_history(r: &HistoryOutput, format: Format) -> Result<(), ProcessorError> {
    let stdout = io::stdout();
    let mut h = stdout.lock();
    match format {
        Format::Json => {
            writeln!(h, "{}", serde_json::to_string_pretty(r)?)?;
        }
        Format::Text => {
            if r.entries.is_empty() {
                writeln!(h, "(no poll history)")?;
            } else {
                for e in &r.entries {
                    writeln!(
                        h,
                        "cycle {:<5} at={} outcome={} eva={}/{} audit={}/{} dispatch={}/{}",
                        e.cycle,
                        e.at,
                        e.outcome.name(),
                        e.eva.events_new,
                        e.eva.events_in,
                        e.audit.events_new,
                        e.audit.events_in,
                        e.dispatch.events_new,
                        e.dispatch.events_in,
                    )?;
                }
            }
        }
    }
    Ok(())
}

fn render_schema(r: &SchemaOutput, format: Format) -> Result<(), ProcessorError> {
    let stdout = io::stdout();
    let mut h = stdout.lock();
    match format {
        Format::Json => {
            writeln!(h, "{}", serde_json::to_string_pretty(r)?)?;
        }
        Format::Text => {
            writeln!(h, "inbound-channel: {}", r.inbound_channel)?;
            writeln!(h, "inbound writer:  {}", r.inbound_writer.name())?;
            writeln!(
                h,
                "inbound required keys: [{}]",
                r.inbound_required_keys.join(", ")
            )?;
            writeln!(h, "poll outcomes: [{}]", r.poll_outcomes.join(", "))?;
            writeln!(h, "poll history:  {}", r.poll_history_path_template)?;
            writeln!(h, "sources:")?;
            for s in &r.sources {
                writeln!(
                    h,
                    "  {:<10} output_key={:<18} cursor_kind={:<14} cursor={}",
                    s.source.name(),
                    s.output_key,
                    s.cursor_kind,
                    s.cursor_path_template,
                )?;
            }
        }
    }
    Ok(())
}

fn render_inbound(r: &InboundOutput, format: Format) -> Result<(), ProcessorError> {
    let stdout = io::stdout();
    let mut h = stdout.lock();
    match format {
        Format::Json => {
            writeln!(h, "{}", serde_json::to_string_pretty(r)?)?;
        }
        Format::Text => match &r.state {
            None => {
                if let Some(c) = r.cycle_filter {
                    writeln!(h, "inbound-channel: no state matching cycle {c}")?;
                } else {
                    writeln!(h, "inbound-channel: no state (not yet written)")?;
                }
            }
            Some(s) => {
                writeln!(h, "channel:   {}", s.channel)?;
                writeln!(h, "writer:    {}", s.writer.name())?;
                writeln!(h, "cycle:     {}", s.cycle)?;
                writeln!(h, "timestamp: {}", s.timestamp)?;
                writeln!(
                    h,
                    "payload:   {}",
                    serde_json::to_string_pretty(&s.payload)?
                )?;
            }
        },
    }
    Ok(())
}

// ----- unit tests -----

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_names_are_kebab_case() {
        assert_eq!(Source::Eva.name(), "eva");
        assert_eq!(Source::Audit.name(), "audit");
        assert_eq!(Source::Dispatch.name(), "dispatch");
    }

    #[test]
    fn source_output_keys_match_inbound_required_keys() {
        // The 3 output keys must align with v2-channel-router's `inbound-channel`
        // required_payload_keys.
        assert_eq!(Source::Eva.output_key(), "eva-responses");
        assert_eq!(Source::Audit.output_key(), "audit-posts");
        assert_eq!(Source::Dispatch.output_key(), "dispatch-returns");
    }

    #[test]
    fn source_all_lists_three_sources() {
        let all = Source::all();
        assert_eq!(all.len(), 3);
        assert!(all.contains(&Source::Eva));
        assert!(all.contains(&Source::Audit));
        assert!(all.contains(&Source::Dispatch));
    }

    #[test]
    fn source_output_keys_are_distinct() {
        let keys: Vec<&str> = Source::all().iter().map(|s| s.output_key()).collect();
        let unique: std::collections::HashSet<&str> = keys.iter().copied().collect();
        assert_eq!(unique.len(), 3);
    }

    #[test]
    fn role_names_are_kebab_case() {
        assert_eq!(Role::Reconciler.name(), "reconciler");
        assert_eq!(Role::Planner.name(), "planner");
        assert_eq!(Role::Executor.name(), "executor");
        assert_eq!(Role::Curator.name(), "curator");
    }

    #[test]
    fn inbound_channel_constant_is_kebab_case() {
        assert_eq!(INBOUND_CHANNEL, "inbound-channel");
    }

    #[test]
    fn validate_inbound_rejects_non_object() {
        let err = validate_inbound_payload(&serde_json::json!([])).unwrap_err();
        assert!(matches!(err, ProcessorError::InvalidInboundPayload(_)));
    }

    #[test]
    fn validate_inbound_rejects_missing_key() {
        let v = serde_json::json!({"eva-responses": [], "audit-posts": []});
        let err = validate_inbound_payload(&v).unwrap_err();
        match err {
            ProcessorError::InvalidInboundPayload(msg) => {
                assert!(msg.contains("dispatch-returns"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn validate_inbound_rejects_non_array_value() {
        let v = serde_json::json!({
            "eva-responses": [],
            "audit-posts": "not an array",
            "dispatch-returns": []
        });
        let err = validate_inbound_payload(&v).unwrap_err();
        match err {
            ProcessorError::InvalidInboundPayload(msg) => {
                assert!(msg.contains("audit-posts"));
                assert!(msg.contains("array"));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn validate_inbound_passes_with_all_keys_as_arrays() {
        let v = serde_json::json!({
            "eva-responses": [],
            "audit-posts": [],
            "dispatch-returns": []
        });
        assert!(validate_inbound_payload(&v).is_ok());
    }

    #[test]
    fn filter_events_with_null_cursor_keeps_all() {
        let events = vec![
            Event {
                id: "1".to_string(),
                at: "t".to_string(),
                raw: serde_json::Value::Null,
            },
            Event {
                id: "2".to_string(),
                at: "t".to_string(),
                raw: serde_json::Value::Null,
            },
        ];
        let cursor = Cursor {
            source: Source::Eva,
            value: None,
        };
        assert_eq!(filter_events_past_cursor(events, &cursor).len(), 2);
    }

    #[test]
    fn filter_events_drops_at_or_below_cursor() {
        let events = vec![
            Event {
                id: "1".to_string(),
                at: "t".to_string(),
                raw: serde_json::Value::Null,
            },
            Event {
                id: "2".to_string(),
                at: "t".to_string(),
                raw: serde_json::Value::Null,
            },
            Event {
                id: "3".to_string(),
                at: "t".to_string(),
                raw: serde_json::Value::Null,
            },
        ];
        let cursor = Cursor {
            source: Source::Eva,
            value: Some("2".to_string()),
        };
        let kept = filter_events_past_cursor(events, &cursor);
        assert_eq!(kept.len(), 1);
        assert_eq!(kept[0].id, "3");
    }

    #[test]
    fn highest_event_id_lex_max() {
        let events = vec![
            Event {
                id: "abc".to_string(),
                at: "t".to_string(),
                raw: serde_json::Value::Null,
            },
            Event {
                id: "abd".to_string(),
                at: "t".to_string(),
                raw: serde_json::Value::Null,
            },
            Event {
                id: "abb".to_string(),
                at: "t".to_string(),
                raw: serde_json::Value::Null,
            },
        ];
        assert_eq!(highest_event_id(&events), Some("abd".to_string()));
    }

    #[test]
    fn highest_event_id_empty_is_none() {
        assert_eq!(highest_event_id(&[]), None);
    }

    #[test]
    fn cursor_serde_roundtrip() {
        let c = Cursor {
            source: Source::Audit,
            value: Some("abc123".to_string()),
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: Cursor = serde_json::from_str(&json).unwrap();
        assert_eq!(c, back);
    }

    #[test]
    fn cursor_null_value_roundtrip() {
        let c = Cursor {
            source: Source::Dispatch,
            value: None,
        };
        let json = serde_json::to_string(&c).unwrap();
        let back: Cursor = serde_json::from_str(&json).unwrap();
        assert_eq!(c, back);
    }

    #[test]
    fn describe_json_type_covers_all_variants() {
        assert_eq!(describe_json_type(&serde_json::Value::Null), "null");
        assert_eq!(describe_json_type(&serde_json::json!(true)), "boolean");
        assert_eq!(describe_json_type(&serde_json::json!(1)), "number");
        assert_eq!(describe_json_type(&serde_json::json!("s")), "string");
        assert_eq!(describe_json_type(&serde_json::json!([])), "array");
        assert_eq!(describe_json_type(&serde_json::json!({})), "object");
    }

    #[test]
    fn poll_outcome_name_serializes_kebab_case() {
        let json_success = serde_json::to_string(&PollOutcome::Success).unwrap();
        let json_skipped = serde_json::to_string(&PollOutcome::WriteSkipped).unwrap();
        assert_eq!(json_success, "\"success\"");
        assert_eq!(json_skipped, "\"write-skipped\"");
    }
}
