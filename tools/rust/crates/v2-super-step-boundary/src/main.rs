use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "v2-super-step-boundary",
    about = "Super-step transition state machine for the v2 multi-agent orchestrator: tracks per-cycle role transitions (reconciler → planner → executor → curator), enforces ordering, and verifies channel-output presence at each super-step boundary"
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
    /// Initialize an empty super-step state file at state/super-step.json
    /// (and an empty history file at state/super-step-history.json).
    Init,
    /// Start a new cycle at the reconciler super-step.
    CycleStart {
        #[arg(long)]
        cycle: u32,
        /// Timestamp to record (ISO-8601). Defaults to a sentinel string;
        /// callers SHOULD supply a real timestamp.
        #[arg(long)]
        timestamp: Option<String>,
    },
    /// Print the current super-step state (or "no cycle in progress").
    Current,
    /// Advance from the current super-step to the next in the ordering.
    /// Verifies the current role has written its output channel (per state/channels/<channel>.json).
    Advance {
        /// Timestamp to record (ISO-8601). Defaults to a sentinel.
        #[arg(long)]
        timestamp: Option<String>,
        /// Skip the channel-output verification (for hermetic tests / scaffolding).
        #[arg(long)]
        skip_verify: bool,
    },
    /// End the current cycle. Requires the current super-step to be the curator.
    CycleEnd {
        #[arg(long)]
        cycle: u32,
        /// Timestamp to record (ISO-8601). Defaults to a sentinel.
        #[arg(long)]
        timestamp: Option<String>,
    },
    /// Print the cycle history.
    History {
        /// Limit history entries (newest first when set).
        #[arg(long)]
        limit: Option<usize>,
    },
    /// Print the super-step ordering + transition rules + role-channel mapping.
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

    /// The fixed super-step ordering for cycle 1 minimal end-to-end:
    /// reconciler polls inbound surfaces → planner reads memory + inbound and
    /// decides substantive-focal → executor does the work → curator reflects
    /// and writes memory + appends cycle history.
    fn ordering() -> &'static [Role] {
        &[
            Role::Reconciler,
            Role::Planner,
            Role::Executor,
            Role::Curator,
        ]
    }

    /// The next super-step after `self`, or None if `self` is the final super-step.
    fn next_in_sequence(self) -> Option<Role> {
        let ord = Self::ordering();
        let idx = ord.iter().position(|r| *r == self)?;
        ord.get(idx + 1).copied()
    }

    fn is_first_in_sequence(self) -> bool {
        Self::ordering().first().copied() == Some(self)
    }

    /// The channel this role writes to (B body line 12 Axis 1 + v2-channel-router
    /// allowed_writer mapping in reverse). Verified at advance/cycle-end boundaries.
    fn output_channel(self) -> &'static str {
        match self {
            Role::Reconciler => "inbound-channel",
            Role::Planner => "plan-channel",
            Role::Executor => "work-channel",
            Role::Curator => "memory-channel",
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Transition {
    from: Role,
    to: Role,
    at: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct SuperStepState {
    cycle: u32,
    current_role: Role,
    cycle_started_at: String,
    transitions: Vec<Transition>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
struct CycleRecord {
    cycle: u32,
    started_at: String,
    ended_at: String,
    transitions: Vec<Transition>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, PartialEq)]
struct SuperStepHistory {
    cycles: Vec<CycleRecord>,
}

/// A minimal subset of v2-channel-router's ChannelState used only to verify
/// writer + cycle at advance/cycle-end. Kept locally (SCAFFOLD): a future
/// COMPLETE arc converts to a subprocess invocation of `v2-channel-router read`.
#[derive(Deserialize, Debug)]
struct ChannelStateLite {
    writer: Role,
    cycle: u32,
}

#[derive(Debug)]
enum BoundaryError {
    Io(io::Error),
    Json(String),
    NotInitialized(PathBuf),
    /// Tried to advance past the final super-step without cycle-end.
    AlreadyAtFinalSuperStep {
        current: Role,
    },
    /// Tried cycle-start on a cycle that is already in progress (and not the same N).
    CycleAlreadyInProgress {
        in_progress: u32,
        requested: u32,
    },
    /// Tried cycle-start with N != current_cycle + 1 (cycles must be sequential).
    OutOfOrderCycleStart {
        last_complete: Option<u32>,
        requested: u32,
    },
    /// Tried cycle-end on a cycle that isn't the current in-progress cycle.
    CycleMismatch {
        in_progress: u32,
        requested: u32,
    },
    /// Tried cycle-end before reaching curator super-step.
    CycleNotComplete {
        current: Role,
        required: Role,
    },
    /// channel-router state file for the current role's output channel is missing or
    /// doesn't match the expected writer + cycle.
    ChannelOutputMissing {
        role: Role,
        channel: String,
        reason: String,
    },
    /// No cycle is in progress when one was required.
    NoCycleInProgress,
}

impl std::fmt::Display for BoundaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BoundaryError::Io(e) => write!(f, "{e}"),
            BoundaryError::Json(s) => write!(f, "json error: {s}"),
            BoundaryError::NotInitialized(p) => write!(
                f,
                "super-step state not initialized at {} (run `v2-super-step-boundary init`)",
                p.display()
            ),
            BoundaryError::AlreadyAtFinalSuperStep { current } => write!(
                f,
                "already at final super-step '{}'; run `cycle-end` to finalize the cycle, not `advance`",
                current.name()
            ),
            BoundaryError::CycleAlreadyInProgress { in_progress, requested } => write!(
                f,
                "cycle {in_progress} is already in progress; cannot start cycle {requested}"
            ),
            BoundaryError::OutOfOrderCycleStart { last_complete, requested } => match last_complete {
                Some(n) => write!(
                    f,
                    "super-step out of order: cycles must be sequential: last completed cycle was {n}, requested cycle {requested} (expected {})",
                    n + 1
                ),
                None => write!(
                    f,
                    "super-step out of order: no completed cycles in history; requested cycle {requested} (expected first cycle to start)"
                ),
            },
            BoundaryError::CycleMismatch { in_progress, requested } => write!(
                f,
                "cycle-end requested for cycle {requested} but cycle {in_progress} is in progress"
            ),
            BoundaryError::CycleNotComplete { current, required } => write!(
                f,
                "cannot end cycle: current super-step is '{}' but '{}' is required for cycle-end",
                current.name(),
                required.name()
            ),
            BoundaryError::ChannelOutputMissing { role, channel, reason } => write!(
                f,
                "channel-output verification failed for role '{}' on channel '{channel}': {reason}",
                role.name()
            ),
            BoundaryError::NoCycleInProgress => write!(
                f,
                "no cycle is in progress (run `cycle-start --cycle N` first)"
            ),
        }
    }
}

impl From<io::Error> for BoundaryError {
    fn from(e: io::Error) -> Self {
        BoundaryError::Io(e)
    }
}

impl From<serde_json::Error> for BoundaryError {
    fn from(e: serde_json::Error) -> Self {
        BoundaryError::Json(e.to_string())
    }
}

fn state_dir(repo_root: &Path) -> PathBuf {
    repo_root.join("state")
}

fn super_step_state_path(repo_root: &Path) -> PathBuf {
    state_dir(repo_root).join("super-step.json")
}

fn super_step_history_path(repo_root: &Path) -> PathBuf {
    state_dir(repo_root).join("super-step-history.json")
}

fn channel_state_path(repo_root: &Path, channel: &str) -> PathBuf {
    state_dir(repo_root)
        .join("channels")
        .join(format!("{channel}.json"))
}

/// Atomic write via temp-file + rename (same pattern as v2-channel-router::write_atomic).
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

/// Read the super-step state. Returns `Ok(None)` if the state file is the
/// empty sentinel (no cycle in progress). Returns `Err(NotInitialized)` if the
/// state file is absent altogether.
fn read_state(repo_root: &Path) -> Result<Option<SuperStepState>, BoundaryError> {
    let path = super_step_state_path(repo_root);
    if !path.exists() {
        return Err(BoundaryError::NotInitialized(state_dir(repo_root)));
    }
    let raw = fs::read_to_string(&path)?;
    if raw.trim().is_empty() || raw.trim() == "null" {
        return Ok(None);
    }
    let state: SuperStepState = serde_json::from_str(&raw)
        .map_err(|e| BoundaryError::Json(format!("decoding {}: {e}", path.display())))?;
    Ok(Some(state))
}

fn write_state(repo_root: &Path, state: &SuperStepState) -> Result<(), BoundaryError> {
    let path = super_step_state_path(repo_root);
    let body = serde_json::to_string_pretty(state)?;
    write_atomic(&path, body.as_bytes())?;
    Ok(())
}

/// Write the empty-sentinel state file (no cycle in progress).
fn write_empty_state(repo_root: &Path) -> Result<(), BoundaryError> {
    let path = super_step_state_path(repo_root);
    write_atomic(&path, b"null\n")?;
    Ok(())
}

fn read_history(repo_root: &Path) -> Result<SuperStepHistory, BoundaryError> {
    let path = super_step_history_path(repo_root);
    if !path.exists() {
        return Err(BoundaryError::NotInitialized(state_dir(repo_root)));
    }
    let raw = fs::read_to_string(&path)?;
    if raw.trim().is_empty() {
        return Ok(SuperStepHistory::default());
    }
    let history: SuperStepHistory = serde_json::from_str(&raw)
        .map_err(|e| BoundaryError::Json(format!("decoding {}: {e}", path.display())))?;
    Ok(history)
}

fn append_history(
    repo_root: &Path,
    record: CycleRecord,
) -> Result<SuperStepHistory, BoundaryError> {
    let mut history = read_history(repo_root)?;
    history.cycles.push(record);
    let path = super_step_history_path(repo_root);
    let body = serde_json::to_string_pretty(&history)?;
    write_atomic(&path, body.as_bytes())?;
    Ok(history)
}

/// Verify that the role's output channel state file exists and matches the
/// expected writer + cycle. Returns Ok on success, Err with a structured
/// reason on failure. DEFERRED: replace direct file read with subprocess
/// invocation of `v2-channel-router read --channel <channel>` (cycle 147+).
fn verify_channel_output(repo_root: &Path, role: Role, cycle: u32) -> Result<(), BoundaryError> {
    let channel = role.output_channel();
    let path = channel_state_path(repo_root, channel);
    if !path.exists() {
        return Err(BoundaryError::ChannelOutputMissing {
            role,
            channel: channel.to_string(),
            reason: format!("state file {} does not exist", path.display()),
        });
    }
    let raw = fs::read_to_string(&path).map_err(|e| BoundaryError::ChannelOutputMissing {
        role,
        channel: channel.to_string(),
        reason: format!("failed to read {}: {e}", path.display()),
    })?;
    if raw.trim().is_empty() || raw.trim() == "null" {
        return Err(BoundaryError::ChannelOutputMissing {
            role,
            channel: channel.to_string(),
            reason: format!(
                "state file {} is empty (channel never written)",
                path.display()
            ),
        });
    }
    let state: ChannelStateLite =
        serde_json::from_str(&raw).map_err(|e| BoundaryError::ChannelOutputMissing {
            role,
            channel: channel.to_string(),
            reason: format!("failed to parse {}: {e}", path.display()),
        })?;
    if state.writer != role {
        return Err(BoundaryError::ChannelOutputMissing {
            role,
            channel: channel.to_string(),
            reason: format!(
                "channel writer is '{}' but expected '{}'",
                state.writer.name(),
                role.name()
            ),
        });
    }
    if state.cycle != cycle {
        return Err(BoundaryError::ChannelOutputMissing {
            role,
            channel: channel.to_string(),
            reason: format!("channel cycle is {} but expected {}", state.cycle, cycle),
        });
    }
    Ok(())
}

fn default_timestamp() -> String {
    "1970-01-01T00:00:00Z".to_string()
}

fn run_init(repo_root: &Path, format: Format) -> Result<(), BoundaryError> {
    fs::create_dir_all(state_dir(repo_root))?;
    let state_path = super_step_state_path(repo_root);
    let history_path = super_step_history_path(repo_root);

    let state_created = if !state_path.exists() {
        write_empty_state(repo_root)?;
        true
    } else {
        false
    };

    let history_created = if !history_path.exists() {
        let empty = SuperStepHistory::default();
        let body = serde_json::to_string_pretty(&empty)?;
        write_atomic(&history_path, body.as_bytes())?;
        true
    } else {
        false
    };

    match format {
        Format::Text => {
            if state_created {
                println!("created {}", state_path.display());
            } else {
                println!("present {}", state_path.display());
            }
            if history_created {
                println!("created {}", history_path.display());
            } else {
                println!("present {}", history_path.display());
            }
        }
        Format::Json => {
            let report = serde_json::json!({
                "state_file": state_path.display().to_string(),
                "state_file_created": state_created,
                "history_file": history_path.display().to_string(),
                "history_file_created": history_created,
            });
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
    }
    Ok(())
}

fn run_cycle_start(
    repo_root: &Path,
    cycle: u32,
    timestamp: Option<String>,
    format: Format,
) -> Result<(), BoundaryError> {
    let ts = timestamp.unwrap_or_else(default_timestamp);
    // Already in progress?
    if let Some(existing) = read_state(repo_root)? {
        if existing.cycle == cycle {
            // Idempotent: same cycle re-start at the first super-step.
            if existing.current_role.is_first_in_sequence() && existing.transitions.is_empty() {
                emit_cycle_started(&existing, format, /*was_idempotent=*/ true)?;
                return Ok(());
            }
            return Err(BoundaryError::CycleAlreadyInProgress {
                in_progress: existing.cycle,
                requested: cycle,
            });
        }
        return Err(BoundaryError::CycleAlreadyInProgress {
            in_progress: existing.cycle,
            requested: cycle,
        });
    }
    // No cycle in progress: check that requested cycle is exactly last_complete + 1.
    let history = read_history(repo_root)?;
    let last_complete = history.cycles.last().map(|c| c.cycle);
    let expected = last_complete.map(|n| n + 1).unwrap_or(cycle);
    // First-ever cycle is allowed at any N (the operator picks the starting cycle
    // number). Subsequent cycles must be sequential.
    if last_complete.is_some() && cycle != expected {
        return Err(BoundaryError::OutOfOrderCycleStart {
            last_complete,
            requested: cycle,
        });
    }
    let new_state = SuperStepState {
        cycle,
        current_role: Role::ordering()[0],
        cycle_started_at: ts,
        transitions: Vec::new(),
    };
    write_state(repo_root, &new_state)?;
    emit_cycle_started(&new_state, format, /*was_idempotent=*/ false)
}

fn emit_cycle_started(
    state: &SuperStepState,
    format: Format,
    was_idempotent: bool,
) -> Result<(), BoundaryError> {
    match format {
        Format::Text => {
            println!(
                "cycle {} started at super-step '{}' ({})",
                state.cycle,
                state.current_role.name(),
                state.cycle_started_at
            );
            if was_idempotent {
                println!("(idempotent: cycle already at first super-step with no transitions)");
            }
        }
        Format::Json => {
            let report = serde_json::json!({
                "cycle": state.cycle,
                "current_role": state.current_role.name(),
                "cycle_started_at": state.cycle_started_at,
                "idempotent": was_idempotent,
            });
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
    }
    Ok(())
}

fn run_current(repo_root: &Path, format: Format) -> Result<(), BoundaryError> {
    match read_state(repo_root)? {
        Some(state) => match format {
            Format::Text => {
                println!(
                    "cycle {} super-step '{}' (started {})",
                    state.cycle,
                    state.current_role.name(),
                    state.cycle_started_at
                );
                if !state.transitions.is_empty() {
                    println!("transitions ({}):", state.transitions.len());
                    for t in &state.transitions {
                        println!("  {} → {} at {}", t.from.name(), t.to.name(), t.at);
                    }
                }
            }
            Format::Json => {
                println!("{}", serde_json::to_string_pretty(&state)?);
            }
        },
        None => match format {
            Format::Text => println!("no cycle in progress"),
            Format::Json => println!("null"),
        },
    }
    Ok(())
}

fn run_advance(
    repo_root: &Path,
    timestamp: Option<String>,
    skip_verify: bool,
    format: Format,
) -> Result<(), BoundaryError> {
    let ts = timestamp.unwrap_or_else(default_timestamp);
    let mut state = read_state(repo_root)?.ok_or(BoundaryError::NoCycleInProgress)?;
    let from = state.current_role;
    let to = from
        .next_in_sequence()
        .ok_or(BoundaryError::AlreadyAtFinalSuperStep { current: from })?;
    if !skip_verify {
        verify_channel_output(repo_root, from, state.cycle)?;
    }
    state.transitions.push(Transition {
        from,
        to,
        at: ts.clone(),
    });
    state.current_role = to;
    write_state(repo_root, &state)?;
    match format {
        Format::Text => {
            println!(
                "advanced cycle {} from '{}' to '{}' at {}",
                state.cycle,
                from.name(),
                to.name(),
                ts
            );
            if skip_verify {
                println!("(channel-output verification skipped)");
            }
        }
        Format::Json => {
            let report = serde_json::json!({
                "cycle": state.cycle,
                "from": from.name(),
                "to": to.name(),
                "at": ts,
                "verification_skipped": skip_verify,
            });
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
    }
    Ok(())
}

fn run_cycle_end(
    repo_root: &Path,
    cycle: u32,
    timestamp: Option<String>,
    format: Format,
) -> Result<(), BoundaryError> {
    let ts = timestamp.unwrap_or_else(default_timestamp);
    let state = read_state(repo_root)?.ok_or(BoundaryError::NoCycleInProgress)?;
    if state.cycle != cycle {
        return Err(BoundaryError::CycleMismatch {
            in_progress: state.cycle,
            requested: cycle,
        });
    }
    let required = *Role::ordering()
        .last()
        .expect("ordering is non-empty by construction");
    if state.current_role != required {
        return Err(BoundaryError::CycleNotComplete {
            current: state.current_role,
            required,
        });
    }
    // Verify curator's output channel before finalizing (the final super-step
    // is the last guard against shipping a cycle with a missing memory-channel write).
    verify_channel_output(repo_root, state.current_role, state.cycle)?;
    let record = CycleRecord {
        cycle: state.cycle,
        started_at: state.cycle_started_at.clone(),
        ended_at: ts.clone(),
        transitions: state.transitions.clone(),
    };
    append_history(repo_root, record)?;
    write_empty_state(repo_root)?;
    match format {
        Format::Text => {
            println!(
                "cycle {} ended at '{}' (started {}, ended {})",
                cycle,
                required.name(),
                state.cycle_started_at,
                ts
            );
        }
        Format::Json => {
            let report = serde_json::json!({
                "cycle": cycle,
                "ended_at_role": required.name(),
                "cycle_started_at": state.cycle_started_at,
                "cycle_ended_at": ts,
            });
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
    }
    Ok(())
}

fn run_history(
    repo_root: &Path,
    limit: Option<usize>,
    format: Format,
) -> Result<(), BoundaryError> {
    let history = read_history(repo_root)?;
    let entries: Vec<&CycleRecord> = match limit {
        Some(n) => history.cycles.iter().rev().take(n).collect(),
        None => history.cycles.iter().collect(),
    };
    match format {
        Format::Text => {
            if entries.is_empty() {
                println!("no completed cycles");
            } else {
                for c in &entries {
                    println!(
                        "cycle {} started {} ended {} ({} transitions)",
                        c.cycle,
                        c.started_at,
                        c.ended_at,
                        c.transitions.len()
                    );
                }
            }
        }
        Format::Json => {
            let report = serde_json::json!({
                "cycles": entries,
                "limit": limit,
                "total_history_length": history.cycles.len(),
            });
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
    }
    Ok(())
}

fn run_schema(format: Format) -> Result<(), BoundaryError> {
    let ordering: Vec<&str> = Role::ordering().iter().map(|r| r.name()).collect();
    let role_to_channel: Vec<(String, String)> = Role::ordering()
        .iter()
        .map(|r| (r.name().to_string(), r.output_channel().to_string()))
        .collect();
    match format {
        Format::Text => {
            println!("super-step ordering ({}):", ordering.len());
            for (i, r) in ordering.iter().enumerate() {
                println!("  {}: {}", i + 1, r);
            }
            println!();
            println!("role → output channel:");
            for (role, ch) in &role_to_channel {
                println!("  {role} → {ch}");
            }
            println!();
            println!("transition rules:");
            println!("  - cycle-start enters super-step '{}'", ordering[0]);
            println!("  - advance moves from current to next in sequence");
            println!(
                "  - advance verifies the current role's output channel exists, matches the expected writer + cycle"
            );
            println!(
                "  - cycle-end requires current super-step to be '{}'",
                ordering[ordering.len() - 1]
            );
            println!("  - cycles must run in sequential order; no skipping forward, no backward transitions (cycle 1 minimal)");
        }
        Format::Json => {
            let report = serde_json::json!({
                "ordering": ordering,
                "role_to_channel": role_to_channel
                    .iter()
                    .map(|(r, c)| (r.clone(), c.clone()))
                    .collect::<std::collections::BTreeMap<_, _>>(),
                "transition_rules": {
                    "cycle_start_enters": ordering[0],
                    "cycle_end_requires": ordering[ordering.len() - 1],
                    "advance_verifies_channel_output": true,
                    "backward_transitions_allowed": false,
                    "skip_forward_allowed": false,
                },
            });
            println!("{}", serde_json::to_string_pretty(&report)?);
        }
    }
    Ok(())
}

fn run() -> Result<(), BoundaryError> {
    let args = Args::parse();
    match args.command {
        Command::Init => run_init(&args.repo_root, args.format),
        Command::CycleStart { cycle, timestamp } => {
            run_cycle_start(&args.repo_root, cycle, timestamp, args.format)
        }
        Command::Current => run_current(&args.repo_root, args.format),
        Command::Advance {
            timestamp,
            skip_verify,
        } => run_advance(&args.repo_root, timestamp, skip_verify, args.format),
        Command::CycleEnd { cycle, timestamp } => {
            run_cycle_end(&args.repo_root, cycle, timestamp, args.format)
        }
        Command::History { limit } => run_history(&args.repo_root, limit, args.format),
        Command::Schema => run_schema(args.format),
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_ordering_is_reconciler_first_curator_last() {
        let ord = Role::ordering();
        assert_eq!(ord.len(), 4);
        assert_eq!(ord[0], Role::Reconciler);
        assert_eq!(ord[ord.len() - 1], Role::Curator);
    }

    #[test]
    fn role_next_in_sequence_advances() {
        assert_eq!(Role::Reconciler.next_in_sequence(), Some(Role::Planner));
        assert_eq!(Role::Planner.next_in_sequence(), Some(Role::Executor));
        assert_eq!(Role::Executor.next_in_sequence(), Some(Role::Curator));
        assert_eq!(Role::Curator.next_in_sequence(), None);
    }

    #[test]
    fn role_first_in_sequence_is_only_reconciler() {
        assert!(Role::Reconciler.is_first_in_sequence());
        assert!(!Role::Planner.is_first_in_sequence());
        assert!(!Role::Executor.is_first_in_sequence());
        assert!(!Role::Curator.is_first_in_sequence());
    }

    #[test]
    fn final_super_step_has_no_next() {
        assert!(Role::Curator.next_in_sequence().is_none());
        let last = Role::ordering().last().copied().unwrap();
        assert_eq!(last, Role::Curator);
    }

    #[test]
    fn role_output_channels_distinct_and_match_router_writer_map() {
        // Each role writes to a distinct channel (mirrors v2-channel-router::Channel::allowed_writer in reverse).
        let mut seen = std::collections::HashSet::new();
        for r in Role::ordering() {
            assert!(
                seen.insert(r.output_channel()),
                "role {} has a duplicate output channel '{}'",
                r.name(),
                r.output_channel()
            );
        }
        // Spot-check the mapping matches the v2-channel-router contract.
        assert_eq!(Role::Reconciler.output_channel(), "inbound-channel");
        assert_eq!(Role::Planner.output_channel(), "plan-channel");
        assert_eq!(Role::Executor.output_channel(), "work-channel");
        assert_eq!(Role::Curator.output_channel(), "memory-channel");
    }

    #[test]
    fn role_names_kebab_case() {
        for r in Role::ordering() {
            let s = serde_json::to_string(r).unwrap();
            assert!(
                s.starts_with('"') && s.ends_with('"'),
                "role serialized as quoted string, got {s}"
            );
            let inner = &s[1..s.len() - 1];
            assert!(
                inner == r.name(),
                "serialized form {inner} does not match name {}",
                r.name()
            );
        }
    }

    #[test]
    fn default_timestamp_is_nonempty_string() {
        let ts = default_timestamp();
        assert!(!ts.is_empty());
    }

    #[test]
    fn paths_are_under_state_dir() {
        let root = Path::new("/tmp/example-repo");
        assert!(super_step_state_path(root).starts_with(root.join("state")));
        assert!(super_step_history_path(root).starts_with(root.join("state")));
        assert!(channel_state_path(root, "plan-channel").starts_with(root.join("state")));
    }

    #[test]
    fn channel_path_is_kebab_case_filename() {
        let root = Path::new("/tmp/example-repo");
        let p = channel_state_path(root, "plan-channel");
        assert_eq!(
            p.file_name().and_then(|s| s.to_str()),
            Some("plan-channel.json")
        );
    }

    #[test]
    fn empty_history_serde_roundtrip() {
        let empty = SuperStepHistory::default();
        let body = serde_json::to_string(&empty).unwrap();
        let parsed: SuperStepHistory = serde_json::from_str(&body).unwrap();
        assert_eq!(parsed, empty);
    }
}
