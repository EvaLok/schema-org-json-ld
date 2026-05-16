use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "v2-channel-router",
    about = "Typed-channel reducer-rule application and per-channel file persistence for the v2 multi-agent orchestrator"
)]
struct Args {
    /// Repository root (path containing state/channels/)
    #[arg(long, default_value = ".", global = true)]
    repo_root: PathBuf,

    /// Output format
    #[arg(long, value_enum, default_value_t = Format::Text, global = true)]
    format: Format,

    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Read the current state of a channel.
    Read {
        #[arg(long, value_enum)]
        channel: Channel,
    },
    /// Write a new artifact to a channel via its reducer rule.
    Write {
        #[arg(long, value_enum)]
        channel: Channel,
        #[arg(long, value_enum)]
        writer: Role,
        /// Path to a JSON file containing { "cycle": N, "timestamp": "...", "payload": { ... } }.
        #[arg(long)]
        payload_file: PathBuf,
    },
    /// Read the append-only history of a channel.
    History {
        #[arg(long, value_enum)]
        channel: Channel,
        /// Limit history entries (newest first when set).
        #[arg(long)]
        limit: Option<usize>,
    },
    /// Initialize empty state files for all four channels at state/channels/.
    Init,
    /// Print the schema constraints for one channel (or all when --channel omitted).
    Schema {
        #[arg(long, value_enum)]
        channel: Option<Channel>,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
#[allow(clippy::enum_variant_names)]
enum Channel {
    PlanChannel,
    WorkChannel,
    MemoryChannel,
    InboundChannel,
}

#[derive(Copy, Clone, Debug, ValueEnum, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
enum Role {
    Planner,
    Executor,
    Curator,
    Reconciler,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
enum Format {
    Text,
    Json,
}

impl Channel {
    fn name(self) -> &'static str {
        match self {
            Channel::PlanChannel => "plan-channel",
            Channel::WorkChannel => "work-channel",
            Channel::MemoryChannel => "memory-channel",
            Channel::InboundChannel => "inbound-channel",
        }
    }

    /// The reducer rule: each channel has exactly one allowed writer role.
    /// This enforces writes-stay-single-threaded per B body line 12 Axis 1.
    fn allowed_writer(self) -> Role {
        match self {
            Channel::PlanChannel => Role::Planner,
            Channel::WorkChannel => Role::Executor,
            Channel::MemoryChannel => Role::Curator,
            Channel::InboundChannel => Role::Reconciler,
        }
    }

    fn all() -> &'static [Channel] {
        &[
            Channel::PlanChannel,
            Channel::WorkChannel,
            Channel::MemoryChannel,
            Channel::InboundChannel,
        ]
    }

    /// Required schema fields beyond the envelope (cycle / writer / timestamp / payload).
    /// Returns the list of required payload-object keys; minimal-viable per cycle 139 scoping.
    fn required_payload_keys(self) -> &'static [&'static str] {
        match self {
            Channel::PlanChannel => &["substantive-focal", "per-role-tasks"],
            Channel::WorkChannel => &["artifacts-written"],
            Channel::MemoryChannel => &["consolidated-insights"],
            Channel::InboundChannel => &[
                "eva-responses",
                "audit-posts",
                "dispatch-returns",
                "inbound-completeness-marker",
            ],
        }
    }
}

impl Role {
    fn name(self) -> &'static str {
        match self {
            Role::Planner => "planner",
            Role::Executor => "executor",
            Role::Curator => "curator",
            Role::Reconciler => "reconciler",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct ChannelState {
    channel: Channel,
    writer: Role,
    cycle: u32,
    timestamp: String,
    payload: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct ChannelHistory {
    channel: Channel,
    entries: Vec<ChannelHistoryEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct ChannelHistoryEntry {
    writer: Role,
    cycle: u32,
    timestamp: String,
    payload: serde_json::Value,
}

/// What `--payload-file` must contain: envelope fields + payload object.
#[derive(Deserialize, Debug)]
struct WritePayload {
    cycle: u32,
    timestamp: String,
    payload: serde_json::Value,
}

#[derive(Debug)]
enum RouterError {
    Io(io::Error),
    Json(String),
    ReducerViolation {
        channel: Channel,
        attempted_writer: Role,
        allowed_writer: Role,
    },
    InvalidPayload(String),
    NotInitialized(PathBuf),
    MissingPayloadFile(PathBuf),
}

impl std::fmt::Display for RouterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouterError::Io(e) => write!(f, "{e}"),
            RouterError::Json(s) => write!(f, "json error: {s}"),
            RouterError::ReducerViolation {
                channel,
                attempted_writer,
                allowed_writer,
            } => write!(
                f,
                "reducer violation: channel '{}' allows writes only from role '{}', got role '{}'",
                channel.name(),
                allowed_writer.name(),
                attempted_writer.name()
            ),
            RouterError::InvalidPayload(s) => write!(f, "invalid payload: {s}"),
            RouterError::NotInitialized(p) => write!(
                f,
                "channel state directory not initialized at {} (run `v2-channel-router init`)",
                p.display()
            ),
            RouterError::MissingPayloadFile(p) => {
                write!(f, "payload file not found: {}", p.display())
            }
        }
    }
}

impl From<io::Error> for RouterError {
    fn from(e: io::Error) -> Self {
        RouterError::Io(e)
    }
}

impl From<serde_json::Error> for RouterError {
    fn from(e: serde_json::Error) -> Self {
        RouterError::Json(e.to_string())
    }
}

fn channels_dir(repo_root: &Path) -> PathBuf {
    repo_root.join("state").join("channels")
}

fn channel_state_path(repo_root: &Path, channel: Channel) -> PathBuf {
    channels_dir(repo_root).join(format!("{}.json", channel.name()))
}

fn channel_history_path(repo_root: &Path, channel: Channel) -> PathBuf {
    channels_dir(repo_root).join(format!("{}-history.json", channel.name()))
}

/// Read the current channel state. Returns `Ok(None)` if the channel has never been
/// written to (state file exists but is the empty sentinel). Returns
/// `Err(NotInitialized)` if the state file is absent altogether.
fn read_state(repo_root: &Path, channel: Channel) -> Result<Option<ChannelState>, RouterError> {
    let path = channel_state_path(repo_root, channel);
    if !path.exists() {
        return Err(RouterError::NotInitialized(channels_dir(repo_root)));
    }
    let raw = fs::read_to_string(&path)?;
    if raw.trim().is_empty() || raw.trim() == "null" {
        return Ok(None);
    }
    let state: ChannelState = serde_json::from_str(&raw)
        .map_err(|e| RouterError::Json(format!("decoding {}: {e}", path.display())))?;
    if state.channel != channel {
        return Err(RouterError::InvalidPayload(format!(
            "state file {} declares channel '{}' but file path implies '{}'",
            path.display(),
            state.channel.name(),
            channel.name()
        )));
    }
    Ok(Some(state))
}

fn write_state(repo_root: &Path, state: &ChannelState) -> Result<(), RouterError> {
    let path = channel_state_path(repo_root, state.channel);
    let dir = path.parent().expect("channel state path has parent");
    fs::create_dir_all(dir)?;
    let body = serde_json::to_string_pretty(state)?;
    write_atomic(&path, body.as_bytes())?;
    Ok(())
}

fn read_history(repo_root: &Path, channel: Channel) -> Result<ChannelHistory, RouterError> {
    let path = channel_history_path(repo_root, channel);
    if !path.exists() {
        return Err(RouterError::NotInitialized(channels_dir(repo_root)));
    }
    let raw = fs::read_to_string(&path)?;
    if raw.trim().is_empty() {
        return Ok(ChannelHistory {
            channel,
            entries: Vec::new(),
        });
    }
    let history: ChannelHistory = serde_json::from_str(&raw)
        .map_err(|e| RouterError::Json(format!("decoding {}: {e}", path.display())))?;
    if history.channel != channel {
        return Err(RouterError::InvalidPayload(format!(
            "history file {} declares channel '{}' but file path implies '{}'",
            path.display(),
            history.channel.name(),
            channel.name()
        )));
    }
    Ok(history)
}

fn append_history(
    repo_root: &Path,
    channel: Channel,
    entry: ChannelHistoryEntry,
) -> Result<ChannelHistory, RouterError> {
    let mut history = read_history(repo_root, channel)?;
    history.entries.push(entry);
    let path = channel_history_path(repo_root, channel);
    let dir = path.parent().expect("channel history path has parent");
    fs::create_dir_all(dir)?;
    let body = serde_json::to_string_pretty(&history)?;
    write_atomic(&path, body.as_bytes())?;
    Ok(history)
}

/// Atomic write via temp-file + rename. Avoids partial-write states if the process
/// is killed mid-write (per git-safety primitive's spirit even though only an
/// in-flight state file, not a commit).
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
    let mut f = fs::File::create(&tmp)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    drop(f);
    fs::rename(&tmp, path)?;
    Ok(())
}

/// The reducer rule. Returns the new state to persist, or a ReducerViolation
/// when the writer role does not match the channel's allowed writer.
fn apply_reducer(
    channel: Channel,
    writer: Role,
    cycle: u32,
    timestamp: String,
    payload: serde_json::Value,
) -> Result<ChannelState, RouterError> {
    let allowed_writer = channel.allowed_writer();
    if writer != allowed_writer {
        return Err(RouterError::ReducerViolation {
            channel,
            attempted_writer: writer,
            allowed_writer,
        });
    }
    validate_payload(channel, &payload)?;
    Ok(ChannelState {
        channel,
        writer,
        cycle,
        timestamp,
        payload,
    })
}

/// Minimal-viable schema validation per cycle 139 scoping. Checks that the payload is a
/// JSON object and contains the required keys for the channel. Does not type-check
/// nested fields; that work is deferred to v2-channel-router COMPLETE arc.
fn validate_payload(channel: Channel, payload: &serde_json::Value) -> Result<(), RouterError> {
    let obj = payload.as_object().ok_or_else(|| {
        RouterError::InvalidPayload(format!(
            "channel '{}' payload must be a JSON object, got {}",
            channel.name(),
            describe_json_type(payload)
        ))
    })?;
    for key in channel.required_payload_keys() {
        if !obj.contains_key(*key) {
            return Err(RouterError::InvalidPayload(format!(
                "channel '{}' payload missing required key '{}'",
                channel.name(),
                key
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

#[derive(Serialize)]
struct ReadOutput<'a> {
    channel: Channel,
    state: Option<&'a ChannelState>,
}

#[derive(Serialize)]
struct WriteOutput {
    channel: Channel,
    writer: Role,
    cycle: u32,
    timestamp: String,
    state_path: String,
    history_path: String,
    history_entries: usize,
}

#[derive(Serialize)]
struct HistoryOutput<'a> {
    channel: Channel,
    total: usize,
    returned: usize,
    entries: Vec<&'a ChannelHistoryEntry>,
}

#[derive(Serialize)]
struct InitOutput {
    channels_dir: String,
    created: Vec<String>,
    already_present: Vec<String>,
}

#[derive(Serialize)]
struct SchemaOutput {
    channel: Channel,
    allowed_writer: Role,
    required_payload_keys: Vec<&'static str>,
    state_path_template: String,
    history_path_template: String,
}

fn run_init<W: Write>(
    repo_root: &Path,
    format: Format,
    out: &mut W,
) -> Result<(), RouterError> {
    let dir = channels_dir(repo_root);
    fs::create_dir_all(&dir)?;
    let mut created = Vec::new();
    let mut already_present = Vec::new();
    for channel in Channel::all() {
        for (path, default_body) in [
            (channel_state_path(repo_root, *channel), "null".to_string()),
            (
                channel_history_path(repo_root, *channel),
                serde_json::to_string_pretty(&ChannelHistory {
                    channel: *channel,
                    entries: Vec::new(),
                })?,
            ),
        ] {
            if path.exists() {
                already_present.push(path.display().to_string());
            } else {
                write_atomic(&path, default_body.as_bytes())?;
                created.push(path.display().to_string());
            }
        }
    }
    let result = InitOutput {
        channels_dir: dir.display().to_string(),
        created,
        already_present,
    };
    match format {
        Format::Json => {
            serde_json::to_writer_pretty(&mut *out, &result)?;
            writeln!(out)?;
        }
        Format::Text => {
            writeln!(out, "channels_dir: {}", result.channels_dir)?;
            writeln!(out, "created ({}):", result.created.len())?;
            for p in &result.created {
                writeln!(out, "  + {p}")?;
            }
            writeln!(out, "already_present ({}):", result.already_present.len())?;
            for p in &result.already_present {
                writeln!(out, "  = {p}")?;
            }
        }
    }
    Ok(())
}

fn run_read<W: Write>(
    repo_root: &Path,
    channel: Channel,
    format: Format,
    out: &mut W,
) -> Result<(), RouterError> {
    let state = read_state(repo_root, channel)?;
    let view = ReadOutput {
        channel,
        state: state.as_ref(),
    };
    match format {
        Format::Json => {
            serde_json::to_writer_pretty(&mut *out, &view)?;
            writeln!(out)?;
        }
        Format::Text => match &state {
            None => writeln!(out, "channel '{}': empty (never written)", channel.name())?,
            Some(s) => {
                writeln!(out, "channel:    {}", s.channel.name())?;
                writeln!(out, "writer:     {}", s.writer.name())?;
                writeln!(out, "cycle:      {}", s.cycle)?;
                writeln!(out, "timestamp:  {}", s.timestamp)?;
                writeln!(out, "payload:")?;
                let body = serde_json::to_string_pretty(&s.payload)?;
                for line in body.lines() {
                    writeln!(out, "  {line}")?;
                }
            }
        },
    }
    Ok(())
}

fn run_write<W: Write>(
    repo_root: &Path,
    channel: Channel,
    writer: Role,
    payload_file: &Path,
    format: Format,
    out: &mut W,
) -> Result<(), RouterError> {
    if !payload_file.exists() {
        return Err(RouterError::MissingPayloadFile(payload_file.to_path_buf()));
    }
    let raw = fs::read_to_string(payload_file)?;
    let payload: WritePayload = serde_json::from_str(&raw).map_err(|e| {
        RouterError::Json(format!("decoding {}: {e}", payload_file.display()))
    })?;
    let new_state = apply_reducer(
        channel,
        writer,
        payload.cycle,
        payload.timestamp.clone(),
        payload.payload.clone(),
    )?;
    write_state(repo_root, &new_state)?;
    let entry = ChannelHistoryEntry {
        writer,
        cycle: payload.cycle,
        timestamp: payload.timestamp,
        payload: payload.payload,
    };
    let history = append_history(repo_root, channel, entry)?;
    let result = WriteOutput {
        channel,
        writer,
        cycle: new_state.cycle,
        timestamp: new_state.timestamp.clone(),
        state_path: channel_state_path(repo_root, channel).display().to_string(),
        history_path: channel_history_path(repo_root, channel)
            .display()
            .to_string(),
        history_entries: history.entries.len(),
    };
    match format {
        Format::Json => {
            serde_json::to_writer_pretty(&mut *out, &result)?;
            writeln!(out)?;
        }
        Format::Text => {
            writeln!(out, "ok: wrote channel '{}'", result.channel.name())?;
            writeln!(out, "  writer:        {}", result.writer.name())?;
            writeln!(out, "  cycle:         {}", result.cycle)?;
            writeln!(out, "  state_path:    {}", result.state_path)?;
            writeln!(out, "  history_path:  {}", result.history_path)?;
            writeln!(
                out,
                "  history_count: {} entries",
                result.history_entries
            )?;
        }
    }
    Ok(())
}

fn run_history<W: Write>(
    repo_root: &Path,
    channel: Channel,
    limit: Option<usize>,
    format: Format,
    out: &mut W,
) -> Result<(), RouterError> {
    let history = read_history(repo_root, channel)?;
    let total = history.entries.len();
    let selected: Vec<&ChannelHistoryEntry> = match limit {
        None => history.entries.iter().collect(),
        Some(n) => history.entries.iter().rev().take(n).collect(),
    };
    let view = HistoryOutput {
        channel,
        total,
        returned: selected.len(),
        entries: selected,
    };
    match format {
        Format::Json => {
            serde_json::to_writer_pretty(&mut *out, &view)?;
            writeln!(out)?;
        }
        Format::Text => {
            writeln!(
                out,
                "channel '{}': {} total entries, returning {}",
                view.channel.name(),
                view.total,
                view.returned
            )?;
            for (idx, entry) in view.entries.iter().enumerate() {
                writeln!(
                    out,
                    "  [{idx}] cycle={} writer={} timestamp={}",
                    entry.cycle,
                    entry.writer.name(),
                    entry.timestamp
                )?;
            }
        }
    }
    Ok(())
}

fn run_schema<W: Write>(
    channel: Option<Channel>,
    format: Format,
    out: &mut W,
) -> Result<(), RouterError> {
    let channels: Vec<Channel> = match channel {
        Some(c) => vec![c],
        None => Channel::all().to_vec(),
    };
    let results: Vec<SchemaOutput> = channels
        .into_iter()
        .map(|c| SchemaOutput {
            channel: c,
            allowed_writer: c.allowed_writer(),
            required_payload_keys: c.required_payload_keys().to_vec(),
            state_path_template: format!("state/channels/{}.json", c.name()),
            history_path_template: format!("state/channels/{}-history.json", c.name()),
        })
        .collect();
    match format {
        Format::Json => {
            serde_json::to_writer_pretty(&mut *out, &results)?;
            writeln!(out)?;
        }
        Format::Text => {
            for s in &results {
                writeln!(out, "channel: {}", s.channel.name())?;
                writeln!(out, "  allowed_writer: {}", s.allowed_writer.name())?;
                writeln!(out, "  required_payload_keys:")?;
                for k in &s.required_payload_keys {
                    writeln!(out, "    - {k}")?;
                }
                writeln!(out, "  state_path:   {}", s.state_path_template)?;
                writeln!(out, "  history_path: {}", s.history_path_template)?;
                writeln!(out)?;
            }
        }
    }
    Ok(())
}

fn main() -> ExitCode {
    let args = Args::parse();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    let result = match args.command {
        Command::Read { channel } => run_read(&args.repo_root, channel, args.format, &mut out),
        Command::Write {
            channel,
            writer,
            payload_file,
        } => run_write(
            &args.repo_root,
            channel,
            writer,
            &payload_file,
            args.format,
            &mut out,
        ),
        Command::History { channel, limit } => {
            run_history(&args.repo_root, channel, limit, args.format, &mut out)
        }
        Command::Init => run_init(&args.repo_root, args.format, &mut out),
        Command::Schema { channel } => run_schema(channel, args.format, &mut out),
    };
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(RouterError::Io(io_err)) if io_err.kind() == io::ErrorKind::BrokenPipe => {
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("v2-channel-router: {err}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_plan_payload() -> serde_json::Value {
        serde_json::json!({
            "substantive-focal": "test focal",
            "per-role-tasks": { "executor": "do thing" }
        })
    }

    #[test]
    fn channel_names_are_kebab_case() {
        assert_eq!(Channel::PlanChannel.name(), "plan-channel");
        assert_eq!(Channel::WorkChannel.name(), "work-channel");
        assert_eq!(Channel::MemoryChannel.name(), "memory-channel");
        assert_eq!(Channel::InboundChannel.name(), "inbound-channel");
    }

    #[test]
    fn each_channel_has_distinct_allowed_writer() {
        let writers: Vec<Role> = Channel::all().iter().map(|c| c.allowed_writer()).collect();
        let mut sorted = writers.clone();
        sorted.sort_by_key(|r| r.name());
        sorted.dedup_by_key(|r| r.name());
        assert_eq!(sorted.len(), 4, "each channel must map to a distinct role");
    }

    #[test]
    fn reducer_rejects_wrong_writer() {
        let err = apply_reducer(
            Channel::PlanChannel,
            Role::Executor,
            1,
            "2026-05-14T00:00:00Z".into(),
            sample_plan_payload(),
        )
        .unwrap_err();
        match err {
            RouterError::ReducerViolation {
                channel,
                attempted_writer,
                allowed_writer,
            } => {
                assert_eq!(channel, Channel::PlanChannel);
                assert_eq!(attempted_writer, Role::Executor);
                assert_eq!(allowed_writer, Role::Planner);
            }
            other => panic!("expected ReducerViolation, got {other:?}"),
        }
    }

    #[test]
    fn reducer_accepts_correct_writer() {
        let state = apply_reducer(
            Channel::PlanChannel,
            Role::Planner,
            42,
            "2026-05-14T01:00:00Z".into(),
            sample_plan_payload(),
        )
        .unwrap();
        assert_eq!(state.channel, Channel::PlanChannel);
        assert_eq!(state.writer, Role::Planner);
        assert_eq!(state.cycle, 42);
    }

    #[test]
    fn validate_payload_rejects_non_object() {
        let err = validate_payload(Channel::PlanChannel, &serde_json::json!("string"))
            .unwrap_err();
        match err {
            RouterError::InvalidPayload(_) => {}
            other => panic!("expected InvalidPayload, got {other:?}"),
        }
    }

    #[test]
    fn validate_payload_rejects_missing_required_keys() {
        let err = validate_payload(Channel::PlanChannel, &serde_json::json!({"substantive-focal": "x"}))
            .unwrap_err();
        match err {
            RouterError::InvalidPayload(s) => {
                assert!(s.contains("per-role-tasks"), "unexpected error: {s}");
            }
            other => panic!("expected InvalidPayload, got {other:?}"),
        }
    }

    #[test]
    fn validate_payload_passes_with_all_keys() {
        validate_payload(Channel::PlanChannel, &sample_plan_payload()).unwrap();
        validate_payload(
            Channel::WorkChannel,
            &serde_json::json!({"artifacts-written": []}),
        )
        .unwrap();
        validate_payload(
            Channel::MemoryChannel,
            &serde_json::json!({"consolidated-insights": []}),
        )
        .unwrap();
        validate_payload(
            Channel::InboundChannel,
            &serde_json::json!({
                "eva-responses": [],
                "audit-posts": [],
                "dispatch-returns": [],
                "inbound-completeness-marker": "quiet"
            }),
        )
        .unwrap();
    }

    #[test]
    fn validate_payload_rejects_inbound_missing_completeness_marker() {
        let err = validate_payload(
            Channel::InboundChannel,
            &serde_json::json!({
                "eva-responses": [],
                "audit-posts": [],
                "dispatch-returns": []
            }),
        )
        .unwrap_err();
        match err {
            RouterError::InvalidPayload(msg) => {
                assert!(msg.contains("inbound-completeness-marker"), "msg = {msg}");
            }
            other => panic!("expected InvalidPayload, got {other:?}"),
        }
    }

    #[test]
    fn describe_json_type_covers_all_variants() {
        assert_eq!(describe_json_type(&serde_json::json!(null)), "null");
        assert_eq!(describe_json_type(&serde_json::json!(true)), "boolean");
        assert_eq!(describe_json_type(&serde_json::json!(1)), "number");
        assert_eq!(describe_json_type(&serde_json::json!("s")), "string");
        assert_eq!(describe_json_type(&serde_json::json!([])), "array");
        assert_eq!(describe_json_type(&serde_json::json!({})), "object");
    }

    #[test]
    fn required_payload_keys_are_distinct_per_channel() {
        let all_keys: Vec<_> = Channel::all()
            .iter()
            .flat_map(|c| c.required_payload_keys().iter().copied())
            .collect();
        let mut sorted = all_keys.clone();
        sorted.sort_unstable();
        sorted.dedup();
        assert_eq!(
            sorted.len(),
            all_keys.len(),
            "required-key namespaces overlap across channels: {all_keys:?}"
        );
    }
}
