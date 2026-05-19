use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use v2_error_envelope::{ErrorClass, ErrorEnvelope};

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

    /// Payload validation mode.
    #[arg(long, value_enum, default_value_t = Mode::Strict, global = true)]
    mode: Mode,

    /// Stderr error-emission format. `text` (default) preserves the legacy
    /// `v2-channel-router: <message>` line. `json` emits a single-line
    /// `ErrorEnvelope` JSON record (last non-empty line of stderr) per
    /// `docs/redesign/_notes/v2-structured-error-envelope-arc.md` §3.2. This
    /// is the cycle-2 emit-side wiring — v2-cycle-runner switches its
    /// invocations to `json` at cycle 4 (caller-side migration).
    #[arg(long, value_enum, default_value_t = ErrorFormat::Text, global = true)]
    error_format: ErrorFormat,

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

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
enum Mode {
    Strict,
    Lenient,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
enum ErrorFormat {
    Text,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum PayloadType {
    String,
    Integer,
    Number,
    Boolean,
    Array,
    Object,
    Null,
}

#[derive(Debug, Clone, Copy)]
struct PayloadKey {
    name: &'static str,
    ty: PayloadType,
    sub_keys: &'static [PayloadKey],
}

#[derive(Debug, Clone, Copy)]
struct PayloadSchema {
    required: &'static [PayloadKey],
    optional: &'static [PayloadKey],
}

static PLAN_PER_ROLE_TASKS_SUB_KEYS: &[PayloadKey] = &[
    PayloadKey {
        name: "executor",
        ty: PayloadType::Object,
        sub_keys: &[],
    },
    PayloadKey {
        name: "curator",
        ty: PayloadType::Object,
        sub_keys: &[],
    },
    PayloadKey {
        name: "reconciler",
        ty: PayloadType::Object,
        sub_keys: &[],
    },
];

static PLAN_CHANNEL_SCHEMA: PayloadSchema = PayloadSchema {
    required: &[
        PayloadKey {
            name: "substantive-focal",
            ty: PayloadType::String,
            sub_keys: &[],
        },
        PayloadKey {
            name: "per-role-tasks",
            ty: PayloadType::Object,
            sub_keys: PLAN_PER_ROLE_TASKS_SUB_KEYS,
        },
    ],
    optional: &[
        PayloadKey {
            name: "rationale",
            ty: PayloadType::String,
            sub_keys: &[],
        },
        PayloadKey {
            name: "forward-notes",
            ty: PayloadType::Array,
            sub_keys: &[],
        },
    ],
};

static WORK_CHANNEL_SCHEMA: PayloadSchema = PayloadSchema {
    required: &[PayloadKey {
        name: "artifacts-written",
        ty: PayloadType::Array,
        sub_keys: &[],
    }],
    optional: &[
        PayloadKey {
            name: "decisions-recorded",
            ty: PayloadType::Array,
            sub_keys: &[],
        },
        PayloadKey {
            name: "dispatches-fired",
            ty: PayloadType::Array,
            sub_keys: &[],
        },
    ],
};

static MEMORY_CHANNEL_SCHEMA: PayloadSchema = PayloadSchema {
    required: &[PayloadKey {
        name: "consolidated-insights",
        ty: PayloadType::Array,
        sub_keys: &[],
    }],
    optional: &[
        PayloadKey {
            name: "anti-patterns-noted",
            ty: PayloadType::Array,
            sub_keys: &[],
        },
        PayloadKey {
            name: "pattern-updates",
            ty: PayloadType::Array,
            sub_keys: &[],
        },
    ],
};

static INBOUND_CHANNEL_SCHEMA: PayloadSchema = PayloadSchema {
    required: &[
        PayloadKey {
            name: "eva-responses",
            ty: PayloadType::Array,
            sub_keys: &[],
        },
        PayloadKey {
            name: "audit-posts",
            ty: PayloadType::Array,
            sub_keys: &[],
        },
        PayloadKey {
            name: "dispatch-returns",
            ty: PayloadType::Array,
            sub_keys: &[],
        },
        PayloadKey {
            name: "inbound-completeness-marker",
            ty: PayloadType::String,
            sub_keys: &[],
        },
    ],
    optional: &[],
};

static REQUIRED_NAMES_PLAN_CHANNEL: &[&str] = &["substantive-focal", "per-role-tasks"];
static REQUIRED_NAMES_WORK_CHANNEL: &[&str] = &["artifacts-written"];
static REQUIRED_NAMES_MEMORY_CHANNEL: &[&str] = &["consolidated-insights"];
static REQUIRED_NAMES_INBOUND_CHANNEL: &[&str] = &[
    "eva-responses",
    "audit-posts",
    "dispatch-returns",
    "inbound-completeness-marker",
];

impl Channel {
    fn payload_schema(self) -> &'static PayloadSchema {
        match self {
            Channel::PlanChannel => &PLAN_CHANNEL_SCHEMA,
            Channel::WorkChannel => &WORK_CHANNEL_SCHEMA,
            Channel::MemoryChannel => &MEMORY_CHANNEL_SCHEMA,
            Channel::InboundChannel => &INBOUND_CHANNEL_SCHEMA,
        }
    }

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
            Channel::PlanChannel => REQUIRED_NAMES_PLAN_CHANNEL,
            Channel::WorkChannel => REQUIRED_NAMES_WORK_CHANNEL,
            Channel::MemoryChannel => REQUIRED_NAMES_MEMORY_CHANNEL,
            Channel::InboundChannel => REQUIRED_NAMES_INBOUND_CHANNEL,
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

impl RouterError {
    /// Map a `RouterError` to a structured `ErrorEnvelope` per the design
    /// `_notes/v2-structured-error-envelope-arc.md` §5.1 table. Emitted when
    /// `--error-format=json` is set; consumed by v2-cycle-runner's rewritten
    /// `classify_failure` at cycle 4 of the implementation arc.
    fn to_envelope(&self) -> ErrorEnvelope {
        const PRIMITIVE: &str = "v2-channel-router";
        let human = self.to_string();
        match self {
            // Io variants from `?` propagation: the underlying io::Error
            // doesn't consistently carry a `path` field across kinds, so we
            // expose only `kind` + `message`. Path-bearing io failures (e.g.,
            // missing payload file) are surfaced via the explicit
            // `MissingPayloadFile` variant below, not this catch-all.
            RouterError::Io(io_err) => ErrorEnvelope::new(PRIMITIVE, ErrorClass::Io, human)
                .with_detail("kind", format!("{:?}", io_err.kind()))
                .with_detail("message", io_err.to_string()),
            RouterError::Json(parse_error) => {
                ErrorEnvelope::new(PRIMITIVE, ErrorClass::Protocol, human)
                    .with_detail("parse_error", parse_error.clone())
            }
            RouterError::ReducerViolation {
                channel,
                attempted_writer,
                allowed_writer,
            } => ErrorEnvelope::new(PRIMITIVE, ErrorClass::ChannelWriteRejected, human)
                .with_detail("channel", channel.name())
                .with_detail("writer", attempted_writer.name())
                .with_detail("allowed_writer", allowed_writer.name())
                .with_detail("reason", "reducer-rule-mismatch"),
            RouterError::InvalidPayload(observed) => {
                ErrorEnvelope::new(PRIMITIVE, ErrorClass::Protocol, human)
                    .with_detail("observed_shape", observed.clone())
            }
            RouterError::NotInitialized(path) => {
                ErrorEnvelope::new(PRIMITIVE, ErrorClass::Config, human)
                    .with_detail("key", "channels_dir")
                    .with_detail("expected", path.display().to_string())
                    .with_detail("hint", "run `v2-channel-router init`")
            }
            RouterError::MissingPayloadFile(path) => {
                ErrorEnvelope::new(PRIMITIVE, ErrorClass::Io, human)
                    .with_detail("path", path.display().to_string())
                    .with_detail("op", "read")
            }
        }
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
    mode: Mode,
) -> Result<(ChannelState, Vec<String>), RouterError> {
    let allowed_writer = channel.allowed_writer();
    if writer != allowed_writer {
        return Err(RouterError::ReducerViolation {
            channel,
            attempted_writer: writer,
            allowed_writer,
        });
    }
    let mut warnings = Vec::new();
    validate_payload(channel, &payload, mode, &mut warnings)?;
    Ok((
        ChannelState {
            channel,
            writer,
            cycle,
            timestamp,
            payload,
        },
        warnings,
    ))
}

/// Minimal-viable schema validation per cycle 139 scoping. Checks that the payload is a
/// JSON object and contains the required keys for the channel. Does not type-check
/// nested fields; that work is deferred to v2-channel-router COMPLETE arc.
fn validate_payload(
    channel: Channel,
    payload: &serde_json::Value,
    mode: Mode,
    warnings: &mut Vec<String>,
) -> Result<(), RouterError> {
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

    for key in channel.payload_schema().required {
        validate_key_value(
            key.name,
            obj.get(key.name).expect("required key exists"),
            *key,
            true,
            mode,
            warnings,
        )?;
    }

    for key in channel.payload_schema().optional {
        if let Some(value) = obj.get(key.name) {
            validate_key_value(key.name, value, *key, false, mode, warnings)?;
        }
    }
    Ok(())
}

fn validate_key_value(
    key_name: &str,
    value: &serde_json::Value,
    key_schema: PayloadKey,
    // Optional top-level keys are type-checked only when present; their sub-key shape is
    // not required in this scope, so callers pass false for optional entries.
    enforce_sub_keys: bool,
    mode: Mode,
    warnings: &mut Vec<String>,
) -> Result<(), RouterError> {
    let observed = describe_json_type(value);
    if !matches_payload_type(value, key_schema.ty) {
        return validation_failure(
            format!(
                "payload key '{}': expected type {}, observed {}",
                key_name,
                payload_type_name(key_schema.ty),
                observed
            ),
            mode,
            warnings,
        );
    }

    if enforce_sub_keys && key_schema.ty == PayloadType::Object && !key_schema.sub_keys.is_empty() {
        let value_obj = value
            .as_object()
            .expect("object type already validated before sub-key validation");
        for sub_key in key_schema.sub_keys {
            if let Some(sub_value) = value_obj.get(sub_key.name) {
                let sub_observed = describe_json_type(sub_value);
                if !matches_payload_type(sub_value, sub_key.ty) {
                    validation_failure(
                        format!(
                            "payload key '{}.{}': expected type {}, observed {}",
                            key_name,
                            sub_key.name,
                            payload_type_name(sub_key.ty),
                            sub_observed
                        ),
                        mode,
                        warnings,
                    )?;
                }
            } else {
                validation_failure(
                    format!(
                        "payload key '{}' (type=object): missing required sub-key '{}'",
                        key_name, sub_key.name
                    ),
                    mode,
                    warnings,
                )?;
            }
        }
    }

    Ok(())
}

fn validation_failure(
    message: String,
    mode: Mode,
    warnings: &mut Vec<String>,
) -> Result<(), RouterError> {
    match mode {
        Mode::Strict => Err(RouterError::InvalidPayload(message)),
        Mode::Lenient => {
            warnings.push(message);
            Ok(())
        }
    }
}

fn matches_payload_type(value: &serde_json::Value, expected: PayloadType) -> bool {
    match expected {
        PayloadType::String => value.is_string(),
        PayloadType::Integer => value.as_i64().is_some() || value.as_u64().is_some(),
        PayloadType::Number => value.is_number(),
        PayloadType::Boolean => value.is_boolean(),
        PayloadType::Array => value.is_array(),
        PayloadType::Object => value.is_object(),
        PayloadType::Null => value.is_null(),
    }
}

fn payload_type_name(ty: PayloadType) -> &'static str {
    match ty {
        PayloadType::String => "string",
        PayloadType::Integer => "integer",
        PayloadType::Number => "number",
        PayloadType::Boolean => "boolean",
        PayloadType::Array => "array",
        PayloadType::Object => "object",
        PayloadType::Null => "null",
    }
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
    schema_format_version: u32,
    channels: Vec<ChannelSchemaOutput>,
}

#[derive(Serialize)]
struct ChannelSchemaOutput {
    name: String,
    allowed_writer: Role,
    payload_schema: PayloadSchemaJson,
    state_path_template: String,
    history_path_template: String,
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
    ty: PayloadType,
    sub_keys: Vec<PayloadKeyJson>,
}

fn run_init<W: Write>(repo_root: &Path, format: Format, out: &mut W) -> Result<(), RouterError> {
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
    mode: Mode,
    format: Format,
    out: &mut W,
) -> Result<(), RouterError> {
    if !payload_file.exists() {
        return Err(RouterError::MissingPayloadFile(payload_file.to_path_buf()));
    }
    let raw = fs::read_to_string(payload_file)?;
    let payload: WritePayload = serde_json::from_str(&raw)
        .map_err(|e| RouterError::Json(format!("decoding {}: {e}", payload_file.display())))?;
    let (new_state, validation_warnings) = apply_reducer(
        channel,
        writer,
        payload.cycle,
        payload.timestamp.clone(),
        payload.payload.clone(),
        mode,
    )?;
    for warning in validation_warnings {
        eprintln!("[router-lenient] {warning}");
    }
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
            writeln!(out, "  history_count: {} entries", result.history_entries)?;
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
    let results: Vec<ChannelSchemaOutput> = channels
        .into_iter()
        .map(|c| ChannelSchemaOutput {
            name: c.name().to_string(),
            allowed_writer: c.allowed_writer(),
            payload_schema: payload_schema_json(c.payload_schema()),
            state_path_template: format!("state/channels/{}.json", c.name()),
            history_path_template: format!("state/channels/{}-history.json", c.name()),
        })
        .collect();
    let schema_output = SchemaOutput {
        schema_format_version: 2,
        channels: results,
    };
    match format {
        Format::Json => {
            serde_json::to_writer_pretty(&mut *out, &schema_output)?;
            writeln!(out)?;
        }
        Format::Text => {
            writeln!(
                out,
                "schema_format_version: {}",
                schema_output.schema_format_version
            )?;
            for s in &schema_output.channels {
                writeln!(out, "channel: {}", s.name)?;
                writeln!(out, "  allowed_writer: {}", s.allowed_writer.name())?;
                writeln!(out, "  payload_schema.required:")?;
                for k in &s.payload_schema.required {
                    writeln!(out, "    - {} ({})", k.name, payload_type_name(k.ty))?;
                }
                writeln!(out, "  state_path:   {}", s.state_path_template)?;
                writeln!(out, "  history_path: {}", s.history_path_template)?;
                writeln!(out)?;
            }
        }
    }
    Ok(())
}

fn payload_schema_json(schema: &PayloadSchema) -> PayloadSchemaJson {
    PayloadSchemaJson {
        required: payload_keys_json(schema.required),
        optional: payload_keys_json(schema.optional),
    }
}

fn payload_keys_json(keys: &'static [PayloadKey]) -> Vec<PayloadKeyJson> {
    keys.iter()
        .map(|key| PayloadKeyJson {
            name: key.name.to_string(),
            ty: key.ty,
            sub_keys: payload_keys_json(key.sub_keys),
        })
        .collect()
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
            args.mode,
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
            match args.error_format {
                ErrorFormat::Text => eprintln!("v2-channel-router: {err}"),
                ErrorFormat::Json => eprintln!("{}", err.to_envelope().to_json_line()),
            }
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_DEEP_SUB_KEYS: &[PayloadKey] = &[PayloadKey {
        name: "deep",
        ty: PayloadType::String,
        sub_keys: &[],
    }];
    static TEST_INNER_SUB_KEYS: &[PayloadKey] = &[PayloadKey {
        name: "inner",
        ty: PayloadType::Object,
        sub_keys: TEST_DEEP_SUB_KEYS,
    }];

    fn sample_plan_payload() -> serde_json::Value {
        serde_json::json!({
            "substantive-focal": "test focal",
            "per-role-tasks": {
                "executor": {"action": "do thing"},
                "curator": {"action": "do thing"},
                "reconciler": {"action": "do thing"}
            }
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
            Mode::Strict,
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
            Mode::Strict,
        )
        .unwrap();
        assert_eq!(state.0.channel, Channel::PlanChannel);
        assert_eq!(state.0.writer, Role::Planner);
        assert_eq!(state.0.cycle, 42);
        assert!(state.1.is_empty());
    }

    #[test]
    fn validate_payload_rejects_non_object() {
        let mut warnings = Vec::new();
        let err = validate_payload(
            Channel::PlanChannel,
            &serde_json::json!("string"),
            Mode::Strict,
            &mut warnings,
        )
        .unwrap_err();
        match err {
            RouterError::InvalidPayload(_) => {}
            other => panic!("expected InvalidPayload, got {other:?}"),
        }
    }

    #[test]
    fn validate_payload_rejects_missing_required_keys() {
        let mut warnings = Vec::new();
        let err = validate_payload(
            Channel::PlanChannel,
            &serde_json::json!({"substantive-focal": "x"}),
            Mode::Strict,
            &mut warnings,
        )
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
        let mut warnings = Vec::new();
        validate_payload(
            Channel::PlanChannel,
            &sample_plan_payload(),
            Mode::Strict,
            &mut warnings,
        )
        .unwrap();
        validate_payload(
            Channel::WorkChannel,
            &serde_json::json!({"artifacts-written": []}),
            Mode::Strict,
            &mut warnings,
        )
        .unwrap();
        validate_payload(
            Channel::MemoryChannel,
            &serde_json::json!({"consolidated-insights": []}),
            Mode::Strict,
            &mut warnings,
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
            Mode::Strict,
            &mut warnings,
        )
        .unwrap();
        assert!(warnings.is_empty());
    }

    #[test]
    fn validate_payload_rejects_inbound_missing_completeness_marker() {
        let mut warnings = Vec::new();
        let err = validate_payload(
            Channel::InboundChannel,
            &serde_json::json!({
                "eva-responses": [],
                "audit-posts": [],
                "dispatch-returns": []
            }),
            Mode::Strict,
            &mut warnings,
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

    #[test]
    fn payload_schema_required_names_agree_with_required_payload_keys() {
        for channel in Channel::all() {
            let required_names: Vec<&str> = channel
                .payload_schema()
                .required
                .iter()
                .map(|k| k.name)
                .collect();
            assert_eq!(
                required_names,
                channel.required_payload_keys(),
                "required names drift for {}",
                channel.name()
            );
        }
    }

    #[test]
    fn validate_payload_rejects_string_when_object_expected() {
        let mut warnings = Vec::new();
        let err = validate_payload(
            Channel::PlanChannel,
            &serde_json::json!({
                "substantive-focal": "test focal",
                "per-role-tasks": "wrong"
            }),
            Mode::Strict,
            &mut warnings,
        )
        .unwrap_err();
        match err {
            RouterError::InvalidPayload(msg) => {
                assert!(msg.contains("expected type object"), "{msg}");
            }
            other => panic!("expected InvalidPayload, got {other:?}"),
        }
    }

    #[test]
    fn validate_payload_rejects_missing_sub_key() {
        let mut warnings = Vec::new();
        let err = validate_payload(
            Channel::PlanChannel,
            &serde_json::json!({
                "substantive-focal": "test focal",
                "per-role-tasks": {
                    "executor": {},
                    "curator": {}
                }
            }),
            Mode::Strict,
            &mut warnings,
        )
        .unwrap_err();
        match err {
            RouterError::InvalidPayload(msg) => {
                assert!(
                    msg.contains("missing required sub-key 'reconciler'"),
                    "{msg}"
                );
            }
            other => panic!("expected InvalidPayload, got {other:?}"),
        }
    }

    #[test]
    fn validate_payload_accepts_present_optional_with_correct_type() {
        let mut warnings = Vec::new();
        validate_payload(
            Channel::WorkChannel,
            &serde_json::json!({
                "artifacts-written": [],
                "decisions-recorded": [],
                "dispatches-fired": []
            }),
            Mode::Strict,
            &mut warnings,
        )
        .unwrap();
        assert!(warnings.is_empty());
    }

    #[test]
    fn validate_payload_ignores_absent_optional() {
        let mut warnings = Vec::new();
        validate_payload(
            Channel::WorkChannel,
            &serde_json::json!({
                "artifacts-written": []
            }),
            Mode::Strict,
            &mut warnings,
        )
        .unwrap();
        assert!(warnings.is_empty());
    }

    #[test]
    fn validate_payload_lenient_mode_logs_but_accepts() {
        let mut warnings = Vec::new();
        validate_payload(
            Channel::PlanChannel,
            &serde_json::json!({
                "substantive-focal": "test focal",
                "per-role-tasks": "wrong"
            }),
            Mode::Lenient,
            &mut warnings,
        )
        .unwrap();
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("expected type object"));
    }

    #[test]
    fn validate_payload_null_does_not_satisfy_string() {
        let mut warnings = Vec::new();
        let err = validate_payload(
            Channel::PlanChannel,
            &serde_json::json!({
                "substantive-focal": null,
                "per-role-tasks": {
                    "executor": {},
                    "curator": {},
                    "reconciler": {}
                }
            }),
            Mode::Strict,
            &mut warnings,
        )
        .unwrap_err();
        match err {
            RouterError::InvalidPayload(msg) => {
                assert!(msg.contains("expected type string, observed null"), "{msg}");
            }
            other => panic!("expected InvalidPayload, got {other:?}"),
        }
    }

    #[test]
    fn validate_payload_one_level_recursion_only() {
        let key = PayloadKey {
            name: "outer",
            ty: PayloadType::Object,
            sub_keys: TEST_INNER_SUB_KEYS,
        };
        let value = serde_json::json!({
            "outer": {
                "inner": {}
            }
        });
        let mut warnings = Vec::new();
        validate_key_value(
            "outer",
            &value["outer"],
            key,
            true,
            Mode::Strict,
            &mut warnings,
        )
        .unwrap();
    }

    #[test]
    fn schema_output_serializes_format_version_2() {
        let schema = SchemaOutput {
            schema_format_version: 2,
            channels: vec![ChannelSchemaOutput {
                name: "plan-channel".to_string(),
                allowed_writer: Role::Planner,
                payload_schema: payload_schema_json(&PLAN_CHANNEL_SCHEMA),
                state_path_template: "state/channels/plan-channel.json".to_string(),
                history_path_template: "state/channels/plan-channel-history.json".to_string(),
            }],
        };
        let json = serde_json::to_value(schema).unwrap();
        assert_eq!(json["schema_format_version"], 2);
    }

    // ----- Cycle 183: envelope emit-side (design §5.1) -----------------
    //
    // One test per `RouterError` variant verifies the §5.1 mapping table:
    // primitive name, class, expected detail keys, and that `human` matches
    // the existing Display impl. A final test pins the cross-cutting
    // invariants (primitive constant, JSON shape).

    #[test]
    fn envelope_io_variant_maps_to_io_class_with_kind_and_message() {
        let err = RouterError::Io(io::Error::new(io::ErrorKind::PermissionDenied, "denied"));
        let env = err.to_envelope();
        assert_eq!(env.primitive, "v2-channel-router");
        assert_eq!(env.class, ErrorClass::Io);
        assert_eq!(env.human, err.to_string());
        assert_eq!(env.details.get("kind").unwrap(), "PermissionDenied");
        assert_eq!(env.details.get("message").unwrap(), "denied");
    }

    #[test]
    fn envelope_json_variant_maps_to_protocol_class_with_parse_error() {
        let err = RouterError::Json("expected `,` at line 3 column 5".to_string());
        let env = err.to_envelope();
        assert_eq!(env.class, ErrorClass::Protocol);
        assert_eq!(
            env.details.get("parse_error").unwrap(),
            "expected `,` at line 3 column 5"
        );
    }

    #[test]
    fn envelope_reducer_violation_maps_to_channel_write_rejected_with_full_detail() {
        let err = RouterError::ReducerViolation {
            channel: Channel::PlanChannel,
            attempted_writer: Role::Executor,
            allowed_writer: Role::Planner,
        };
        let env = err.to_envelope();
        assert_eq!(env.class, ErrorClass::ChannelWriteRejected);
        assert_eq!(env.details.get("channel").unwrap(), "plan-channel");
        assert_eq!(env.details.get("writer").unwrap(), "executor");
        assert_eq!(env.details.get("allowed_writer").unwrap(), "planner");
        assert_eq!(env.details.get("reason").unwrap(), "reducer-rule-mismatch");
    }

    #[test]
    fn envelope_invalid_payload_maps_to_protocol_class_with_observed_shape() {
        let err = RouterError::InvalidPayload("missing required key 'foo'".to_string());
        let env = err.to_envelope();
        assert_eq!(env.class, ErrorClass::Protocol);
        assert_eq!(
            env.details.get("observed_shape").unwrap(),
            "missing required key 'foo'"
        );
    }

    #[test]
    fn envelope_not_initialized_maps_to_config_class_with_path_and_hint() {
        let err = RouterError::NotInitialized(PathBuf::from("/some/repo/state/channels"));
        let env = err.to_envelope();
        assert_eq!(env.class, ErrorClass::Config);
        assert_eq!(env.details.get("key").unwrap(), "channels_dir");
        assert_eq!(
            env.details.get("expected").unwrap(),
            "/some/repo/state/channels"
        );
        assert_eq!(
            env.details.get("hint").unwrap(),
            "run `v2-channel-router init`"
        );
    }

    #[test]
    fn envelope_missing_payload_file_maps_to_io_class_with_path_and_read_op() {
        let err = RouterError::MissingPayloadFile(PathBuf::from("/tmp/payload.json"));
        let env = err.to_envelope();
        assert_eq!(env.class, ErrorClass::Io);
        assert_eq!(env.details.get("path").unwrap(), "/tmp/payload.json");
        assert_eq!(env.details.get("op").unwrap(), "read");
    }

    #[test]
    fn envelope_serializes_to_compact_single_line_json() {
        // Pin the §3.2 emission protocol: `to_json_line()` is a single line,
        // no embedded newlines (so the "last non-empty line of stderr is the
        // envelope" convention holds even when stderr accumulates other
        // lines).
        let err = RouterError::NotInitialized(PathBuf::from("/x/y"));
        let line = err.to_envelope().to_json_line();
        assert!(!line.contains('\n'), "envelope line must not contain newlines: {line}");
        assert!(line.starts_with('{') && line.ends_with('}'), "envelope must be a JSON object: {line}");
        // Parse round-trip via parse_from_stderr_tail to verify the line is
        // recoverable through the consumer-side helper too.
        let parsed = v2_error_envelope::parse_from_stderr_tail(&line)
            .expect("parse_from_stderr_tail must recover the envelope");
        assert_eq!(parsed.class, ErrorClass::Config);
        assert_eq!(parsed.primitive, "v2-channel-router");
    }

    #[test]
    fn envelope_retry_policy_for_router_classes_is_all_non_retryable() {
        // Pin that none of v2-channel-router's emit-side classes are
        // retryable per design §4 table. If a future migration adds a
        // Transient class to router, this test forces the question.
        for err in [
            RouterError::Io(io::Error::other("x")),
            RouterError::Json("x".to_string()),
            RouterError::InvalidPayload("x".to_string()),
            RouterError::NotInitialized(PathBuf::from("/x")),
            RouterError::MissingPayloadFile(PathBuf::from("/x")),
            RouterError::ReducerViolation {
                channel: Channel::WorkChannel,
                attempted_writer: Role::Planner,
                allowed_writer: Role::Executor,
            },
        ] {
            let env = err.to_envelope();
            assert!(
                !env.is_retryable_class(),
                "router classes are halt-only; {:?} surfaced as retryable",
                env.class
            );
        }
    }
}
