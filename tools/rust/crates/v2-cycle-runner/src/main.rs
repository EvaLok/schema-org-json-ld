use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, ExitCode};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Parser, Debug)]
#[command(
    name = "v2-cycle-runner",
    about = "Conductor for the v2 multi-agent orchestrator: composes the 4 role-prompt sessions \
             and the 4 v2-* primitives (channel-router, super-step-boundary, role-driver, \
             reconciler-event-processor) into a single per-cycle execution. \
             SCAFFOLD scope cycles 150-151: init / schema / run (--dry-run path + execute path \
             against pre-provided session-output files). DEFERRED to cycles 152+: status / \
             verify subcommands, integration test against real primitives, first end-to-end \
             smoke (first measurement opportunity for the v2 architecture)."
)]
struct Args {
    /// Repository root (path containing state/, prompts/, tools/).
    #[arg(long, default_value = ".", global = true)]
    repo_root: PathBuf,

    /// Output format.
    #[arg(long, value_enum, default_value_t = Format::Text, global = true)]
    format: Format,

    /// Path to v2-channel-router binary.
    #[arg(long, default_value_os_t = default_primitive_bin("v2-channel-router"), global = true)]
    channel_router_bin: PathBuf,

    /// Path to v2-super-step-boundary binary.
    #[arg(long, default_value_os_t = default_primitive_bin("v2-super-step-boundary"), global = true)]
    super_step_boundary_bin: PathBuf,

    /// Path to v2-role-driver binary.
    #[arg(long, default_value_os_t = default_primitive_bin("v2-role-driver"), global = true)]
    role_driver_bin: PathBuf,

    /// Path to v2-reconciler-event-processor binary.
    #[arg(long, default_value_os_t = default_primitive_bin("v2-reconciler-event-processor"), global = true)]
    reconciler_event_processor_bin: PathBuf,

    #[command(subcommand)]
    command: Subcmd,
}

#[derive(Subcommand, Debug)]
enum Subcmd {
    /// Idempotent state initialization. Composes the four primitive-side `init` commands
    /// (channel-router init, super-step-boundary init, role-driver init,
    /// reconciler-event-processor init), then initializes v2-cycle-runner's own minimal state
    /// at state/v2-cycle-runner/last-cycle.json + cycle-history.json.
    Init,
    /// Print the v2-cycle-runner schema: per-cycle super-step sequence, per-primitive composition,
    /// state ownership map. Read-only.
    Schema,
    /// Drive one cycle through the 10-step super-step sequence: super-step-init,
    /// reconciler-pre-poll, reconciler-session, advance, planner-session, advance,
    /// executor-session, advance, curator-session, super-step-settle.
    ///
    /// SCAFFOLD scope cycle 151: role-session steps require pre-provided session-output files
    /// (one per role) because v2-role-driver `invoke` is itself SCAFFOLD — no live claude-code
    /// spawn yet. The --dry-run flag traces the sequence without invoking primitives.
    Run {
        /// Cycle number (the orchestrator-run issue's sequence position).
        #[arg(long)]
        cycle: u32,
        /// The cycle-issue number.
        #[arg(long)]
        issue: u32,
        /// Trace the 10-step sequence without invoking primitives or mutating state.
        #[arg(long)]
        dry_run: bool,
        /// Short-circuit the sequence after the named role's session step (before its advance).
        /// Useful for debugging mid-cycle state.
        #[arg(long)]
        halt_after_role: Option<Role>,
        /// Path to a JSON file containing eva events for reconciler poll. Optional.
        #[arg(long)]
        eva_source_file: Option<PathBuf>,
        /// Path to a JSON file containing audit events for reconciler poll. Optional.
        #[arg(long)]
        audit_source_file: Option<PathBuf>,
        /// Path to a JSON file containing dispatch events for reconciler poll. Optional.
        #[arg(long)]
        dispatch_source_file: Option<PathBuf>,
        /// Path to the reconciler session output (v2-role-driver --session-output-file).
        /// Required unless --dry-run.
        #[arg(long)]
        reconciler_output_file: Option<PathBuf>,
        /// Path to the planner session output. Required unless --dry-run.
        #[arg(long)]
        planner_output_file: Option<PathBuf>,
        /// Path to the executor session output. Required unless --dry-run.
        #[arg(long)]
        executor_output_file: Option<PathBuf>,
        /// Path to the curator session output. Required unless --dry-run.
        #[arg(long)]
        curator_output_file: Option<PathBuf>,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
#[clap(rename_all = "kebab-case")]
enum Role {
    Reconciler,
    Planner,
    Executor,
    Curator,
}

impl Role {
    fn as_kebab(self) -> &'static str {
        match self {
            Role::Reconciler => "reconciler",
            Role::Planner => "planner",
            Role::Executor => "executor",
            Role::Curator => "curator",
        }
    }
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
enum Format {
    Text,
    Json,
}

#[derive(Debug)]
enum RunnerError {
    Io(io::Error),
    Json(serde_json::Error),
    PrimitiveMissing { name: &'static str, path: PathBuf },
    PrimitiveFailed { name: &'static str, stderr: String },
    MissingSessionOutput { role: Role },
    CycleHalted { step: &'static str, class: FailureClass, stderr: String },
    SuperStepOutOfOrder { step: &'static str, stderr: String },
}

impl std::fmt::Display for RunnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RunnerError::Io(e) => write!(f, "{e}"),
            RunnerError::Json(e) => write!(f, "json error: {e}"),
            RunnerError::PrimitiveMissing { name, path } => write!(
                f,
                "{name} binary not found at {} (build it with `cargo build -p {name}`)",
                path.display()
            ),
            RunnerError::PrimitiveFailed { name, stderr } => {
                write!(f, "{name} init failed: {stderr}")
            }
            RunnerError::MissingSessionOutput { role } => write!(
                f,
                "--{}-output-file required for non-dry-run cycle (SCAFFOLD scope: \
                 v2-role-driver invoke needs a session-output file per role)",
                role.as_kebab()
            ),
            RunnerError::CycleHalted { step, class, stderr } => write!(
                f,
                "cycle halted at {step} ({}): {stderr}",
                class.as_kebab()
            ),
            RunnerError::SuperStepOutOfOrder { step, stderr } => write!(
                f,
                "super-step out of order at {step}: {stderr}"
            ),
        }
    }
}

impl From<io::Error> for RunnerError {
    fn from(value: io::Error) -> Self {
        RunnerError::Io(value)
    }
}

impl From<serde_json::Error> for RunnerError {
    fn from(value: serde_json::Error) -> Self {
        RunnerError::Json(value)
    }
}

#[derive(Debug, Serialize)]
struct InitReport {
    primitive_inits: Vec<PrimitiveInitResult>,
    runner_state_initialized: bool,
    runner_state_path: PathBuf,
}

#[derive(Debug, Serialize)]
struct PrimitiveInitResult {
    name: &'static str,
    bin_path: PathBuf,
    succeeded: bool,
    stdout: String,
}

type PrimitivePathFn = fn(&Args) -> &Path;

const PRIMITIVES_TO_INIT: &[(&str, PrimitivePathFn)] = &[
    ("v2-channel-router", |a| a.channel_router_bin.as_path()),
    ("v2-super-step-boundary", |a| a.super_step_boundary_bin.as_path()),
    ("v2-role-driver", |a| a.role_driver_bin.as_path()),
    ("v2-reconciler-event-processor", |a| a.reconciler_event_processor_bin.as_path()),
];

fn default_primitive_bin(name: &str) -> PathBuf {
    if let Ok(target_dir) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target_dir).join("debug").join(name)
    } else {
        PathBuf::from(format!("./target/debug/{name}"))
    }
}

fn main() -> ExitCode {
    let args = Args::parse();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    match run(&args, &mut out) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("v2-cycle-runner: {err}");
            ExitCode::from(1)
        }
    }
}

fn run<W: Write>(args: &Args, out: &mut W) -> Result<(), RunnerError> {
    match &args.command {
        Subcmd::Init => run_init(args, out),
        Subcmd::Schema => run_schema(args, out),
        Subcmd::Run {
            cycle,
            issue,
            dry_run,
            halt_after_role,
            eva_source_file,
            audit_source_file,
            dispatch_source_file,
            reconciler_output_file,
            planner_output_file,
            executor_output_file,
            curator_output_file,
        } => run_cycle(
            args,
            &RunArgs {
                cycle: *cycle,
                issue: *issue,
                dry_run: *dry_run,
                halt_after_role: *halt_after_role,
                eva_source_file: eva_source_file.clone(),
                audit_source_file: audit_source_file.clone(),
                dispatch_source_file: dispatch_source_file.clone(),
                reconciler_output_file: reconciler_output_file.clone(),
                planner_output_file: planner_output_file.clone(),
                executor_output_file: executor_output_file.clone(),
                curator_output_file: curator_output_file.clone(),
            },
            out,
            &RealInvoker,
        ),
    }
}

fn run_init<W: Write>(args: &Args, out: &mut W) -> Result<(), RunnerError> {
    let mut primitive_results = Vec::new();
    for (name, get_path) in PRIMITIVES_TO_INIT {
        let bin_path = get_path(args);
        primitive_results.push(invoke_primitive_init(name, bin_path, &args.repo_root)?);
    }

    let runner_state_dir = args.repo_root.join("state").join("v2-cycle-runner");
    fs::create_dir_all(&runner_state_dir)?;

    let last_cycle_path = runner_state_dir.join("last-cycle.json");
    let cycle_history_path = runner_state_dir.join("cycle-history.json");
    if !last_cycle_path.exists() {
        write_initial_last_cycle(&last_cycle_path)?;
    }
    if !cycle_history_path.exists() {
        write_initial_cycle_history(&cycle_history_path)?;
    }

    let report = InitReport {
        primitive_inits: primitive_results,
        runner_state_initialized: true,
        runner_state_path: runner_state_dir,
    };

    emit_report(&report, args.format, out)
}

fn invoke_primitive_init(
    name: &'static str,
    bin_path: &Path,
    repo_root: &Path,
) -> Result<PrimitiveInitResult, RunnerError> {
    if !bin_path.exists() {
        return Err(RunnerError::PrimitiveMissing {
            name,
            path: bin_path.to_path_buf(),
        });
    }
    let output = ProcessCommand::new(bin_path)
        .args(["--repo-root", repo_root.to_string_lossy().as_ref(), "init"])
        .output()?;
    if !output.status.success() {
        return Err(RunnerError::PrimitiveFailed {
            name,
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        });
    }
    Ok(PrimitiveInitResult {
        name,
        bin_path: bin_path.to_path_buf(),
        succeeded: true,
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
    })
}

fn write_initial_last_cycle(path: &Path) -> Result<(), RunnerError> {
    let payload = serde_json::json!({
        "cycle": null,
        "status": "uninitialized",
        "started_at": null,
        "ended_at": null,
        "halt_reason": null
    });
    fs::write(path, format!("{}\n", serde_json::to_string_pretty(&payload)?))?;
    Ok(())
}

fn write_initial_cycle_history(path: &Path) -> Result<(), RunnerError> {
    let payload = serde_json::json!({ "cycles": [] });
    fs::write(path, format!("{}\n", serde_json::to_string_pretty(&payload)?))?;
    Ok(())
}

fn run_schema<W: Write>(args: &Args, out: &mut W) -> Result<(), RunnerError> {
    match args.format {
        Format::Text => {
            writeln!(out, "v2-cycle-runner schema (cycle 150 scaffold):")?;
            writeln!(out)?;
            writeln!(out, "Per-cycle super-step sequence (10 steps):")?;
            writeln!(out, "  1. super-step-init        (v2-super-step-boundary cycle-start)")?;
            writeln!(out, "  2. reconciler-pre-poll    (v2-reconciler-event-processor poll)")?;
            writeln!(out, "  3. reconciler-session     (v2-role-driver invoke --role reconciler)")?;
            writeln!(out, "  4. super-step-advance-1   (v2-super-step-boundary advance) [to planner]")?;
            writeln!(out, "  5. planner-session        (v2-role-driver invoke --role planner)")?;
            writeln!(out, "  6. super-step-advance-2   (v2-super-step-boundary advance) [to executor]")?;
            writeln!(out, "  7. executor-session       (v2-role-driver invoke --role executor)")?;
            writeln!(out, "  8. super-step-advance-3   (v2-super-step-boundary advance) [to curator]")?;
            writeln!(out, "  9. curator-session        (v2-role-driver invoke --role curator)")?;
            writeln!(out, "  10. super-step-settle     (v2-super-step-boundary cycle-end)")?;
            writeln!(out)?;
            writeln!(out, "State ownership:")?;
            writeln!(out, "  state/channels/          ← v2-channel-router")?;
            writeln!(out, "  state/super-step.json    ← v2-super-step-boundary")?;
            writeln!(out, "  state/super-step-history.json ← v2-super-step-boundary")?;
            writeln!(out, "  state/roles/             ← v2-role-driver")?;
            writeln!(out, "  state/reconciler/        ← v2-reconciler-event-processor")?;
            writeln!(out, "  state/v2-cycle-runner/   ← v2-cycle-runner (self)")?;
            writeln!(out)?;
            writeln!(out, "Subcommands implemented (cycle 150):")?;
            writeln!(out, "  init     ← composes primitive inits + initializes runner-self state")?;
            writeln!(out, "  schema   ← prints this")?;
            writeln!(out)?;
            writeln!(out, "Subcommands DEFERRED to cycle 151-153+:")?;
            writeln!(out, "  run      (the main entrypoint: drives the 10-step sequence)")?;
            writeln!(out, "  status   (current super-step + per-role timestamps)")?;
            writeln!(out, "  verify   (post-cycle: all transitions present + clean)")?;
        }
        Format::Json => {
            let payload = serde_json::json!({
                "super_step_sequence": [
                    {"step": 1, "name": "super-step-init", "primitive": "v2-super-step-boundary", "subcommand": "cycle-start"},
                    {"step": 2, "name": "reconciler-pre-poll", "primitive": "v2-reconciler-event-processor", "subcommand": "poll"},
                    {"step": 3, "name": "reconciler-session", "primitive": "v2-role-driver", "subcommand": "invoke --role reconciler"},
                    {"step": 4, "name": "super-step-advance-1", "primitive": "v2-super-step-boundary", "subcommand": "advance"},
                    {"step": 5, "name": "planner-session", "primitive": "v2-role-driver", "subcommand": "invoke --role planner"},
                    {"step": 6, "name": "super-step-advance-2", "primitive": "v2-super-step-boundary", "subcommand": "advance"},
                    {"step": 7, "name": "executor-session", "primitive": "v2-role-driver", "subcommand": "invoke --role executor"},
                    {"step": 8, "name": "super-step-advance-3", "primitive": "v2-super-step-boundary", "subcommand": "advance"},
                    {"step": 9, "name": "curator-session", "primitive": "v2-role-driver", "subcommand": "invoke --role curator"},
                    {"step": 10, "name": "super-step-settle", "primitive": "v2-super-step-boundary", "subcommand": "cycle-end"}
                ],
                "state_ownership": {
                    "state/channels/": "v2-channel-router",
                    "state/super-step.json": "v2-super-step-boundary",
                    "state/super-step-history.json": "v2-super-step-boundary",
                    "state/roles/": "v2-role-driver",
                    "state/reconciler/": "v2-reconciler-event-processor",
                    "state/v2-cycle-runner/": "v2-cycle-runner"
                },
                "subcommands_implemented": ["init", "schema"],
                "subcommands_deferred": ["run", "status", "verify"],
                "cycle_scope": "150 scaffold entry"
            });
            writeln!(out, "{}", serde_json::to_string_pretty(&payload)?)?;
        }
    }
    Ok(())
}

fn emit_report<W: Write>(
    report: &InitReport,
    format: Format,
    out: &mut W,
) -> Result<(), RunnerError> {
    match format {
        Format::Text => {
            writeln!(out, "v2-cycle-runner init: composed {} primitive inits", report.primitive_inits.len())?;
            for primitive in &report.primitive_inits {
                writeln!(
                    out,
                    "  {}: {} (bin: {})",
                    primitive.name,
                    if primitive.succeeded { "ok" } else { "FAILED" },
                    primitive.bin_path.display()
                )?;
            }
            writeln!(out, "  v2-cycle-runner: ok (state at {})", report.runner_state_path.display())?;
        }
        Format::Json => {
            writeln!(out, "{}", serde_json::to_string_pretty(report)?)?;
        }
    }
    Ok(())
}

// =====================================================================
// run subcommand — cycle 151 implementation
// =====================================================================

#[derive(Debug, Clone)]
struct RunArgs {
    cycle: u32,
    issue: u32,
    dry_run: bool,
    halt_after_role: Option<Role>,
    eva_source_file: Option<PathBuf>,
    audit_source_file: Option<PathBuf>,
    dispatch_source_file: Option<PathBuf>,
    reconciler_output_file: Option<PathBuf>,
    planner_output_file: Option<PathBuf>,
    executor_output_file: Option<PathBuf>,
    curator_output_file: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum StepKind {
    SuperStepCycleStart,
    ReconcilerPoll,
    RoleInvoke(Role),
    SuperStepAdvance,
    SuperStepCycleEnd,
}

#[derive(Debug, Clone, Copy)]
struct Step {
    index: u8,
    name: &'static str,
    kind: StepKind,
}

fn super_step_sequence() -> [Step; 10] {
    [
        Step { index: 1,  name: "super-step-init",       kind: StepKind::SuperStepCycleStart },
        Step { index: 2,  name: "reconciler-pre-poll",   kind: StepKind::ReconcilerPoll },
        Step { index: 3,  name: "reconciler-session",    kind: StepKind::RoleInvoke(Role::Reconciler) },
        Step { index: 4,  name: "super-step-advance-1",  kind: StepKind::SuperStepAdvance },
        Step { index: 5,  name: "planner-session",       kind: StepKind::RoleInvoke(Role::Planner) },
        Step { index: 6,  name: "super-step-advance-2",  kind: StepKind::SuperStepAdvance },
        Step { index: 7,  name: "executor-session",      kind: StepKind::RoleInvoke(Role::Executor) },
        Step { index: 8,  name: "super-step-advance-3",  kind: StepKind::SuperStepAdvance },
        Step { index: 9,  name: "curator-session",       kind: StepKind::RoleInvoke(Role::Curator) },
        Step { index: 10, name: "super-step-settle",     kind: StepKind::SuperStepCycleEnd },
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum FailureClass {
    /// Recoverable: retry once immediately. No backoff or jitter — cycle 155
    /// absorbed cycle 152 critique C8 (L2.3) acknowledged the docstring
    /// previously claimed "brief backoff" but the implementation had none;
    /// the immediate-retry behavior is intentional (lower cycle latency)
    /// and the docstring is now honest about that. See
    /// `docs/redesign/_notes/cycle-155-cycle-152-critique-absorption.md`.
    Transient,
    /// Role session returned empty output (A4-silent-fail family). Halt cleanly.
    RoleSessionEmpty,
    /// Channel-router rejected the role's payload (missing required key, etc.). Halt.
    ChannelWriteRejected,
    /// Super-step-boundary refused a transition. Hard error; abort cycle.
    SuperStepOutOfOrder,
}

impl FailureClass {
    fn as_kebab(self) -> &'static str {
        match self {
            FailureClass::Transient => "transient",
            FailureClass::RoleSessionEmpty => "role-session-empty",
            FailureClass::ChannelWriteRejected => "channel-write-rejected",
            FailureClass::SuperStepOutOfOrder => "super-step-out-of-order",
        }
    }
}

fn classify_failure(_exit_code: i32, stderr: &str) -> FailureClass {
    let lower = stderr.to_lowercase();
    if lower.contains("super-step") && (lower.contains("out of order")
        || lower.contains("ordering")
        || lower.contains("wrong super-step")
        || lower.contains("not at expected"))
    {
        FailureClass::SuperStepOutOfOrder
    } else if lower.contains("rejected") || lower.contains("required key") || lower.contains("schema mismatch") {
        FailureClass::ChannelWriteRejected
    } else if lower.contains("empty") && (lower.contains("output") || lower.contains("payload") || lower.contains("session")) {
        FailureClass::RoleSessionEmpty
    } else {
        FailureClass::Transient
    }
}

trait PrimitiveInvoker {
    fn invoke(&self, bin: &Path, args: &[String]) -> io::Result<std::process::Output>;
}

struct RealInvoker;
impl PrimitiveInvoker for RealInvoker {
    fn invoke(&self, bin: &Path, args: &[String]) -> io::Result<std::process::Output> {
        ProcessCommand::new(bin).args(args).output()
    }
}

#[derive(Debug, Clone, Serialize)]
struct StepTrace {
    index: u8,
    name: &'static str,
    primitive: &'static str,
    args: Vec<String>,
    bin_path: PathBuf,
    /// `true` if the primitive was actually invoked (live mode, regardless
    /// of pass/fail outcome). `false` for dry-run-only traces. Per-step
    /// disambiguation so trace consumers do not have to climb to
    /// `CycleReport.dry_run` or `CycleReport.status == "dry-run-traced"` —
    /// defense-in-depth per cycle 152 v2-cycle-runner critique C5 (L1.5).
    executed: bool,
}

#[derive(Debug, Clone, Serialize)]
struct CycleReport {
    cycle: u32,
    issue: u32,
    started_at: String,
    ended_at: String,
    dry_run: bool,
    status: &'static str,        // "completed" | "halted" | "dry-run-traced" | "out-of-order"
    halt_step: Option<&'static str>,
    halt_class: Option<FailureClass>,
    halted_after_role: Option<Role>,
    steps_attempted: usize,
    traces: Vec<StepTrace>,
}

fn iso8601_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format_unix_secs_iso(secs)
}

fn format_unix_secs_iso(unix_secs: u64) -> String {
    // Minimal RFC3339 / ISO-8601 UTC formatter: avoids chrono dependency.
    // Civil-time algorithm from Howard Hinnant's date library (public domain).
    let z = unix_secs as i64;
    let days = z.div_euclid(86_400);
    let secs_of_day = z.rem_euclid(86_400);
    let h = secs_of_day / 3600;
    let m = (secs_of_day % 3600) / 60;
    let s = secs_of_day % 60;
    let (y, mo, d) = civil_from_days(days);
    format!("{y:04}-{mo:02}-{d:02}T{h:02}:{m:02}:{s:02}Z")
}

fn civil_from_days(z: i64) -> (i32, u32, u32) {
    let z = z + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097) as u64;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = (yoe as i64 + era * 400) as i32;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    let y = if m <= 2 { y + 1 } else { y };
    (y, m, d)
}

fn run_cycle<W: Write, I: PrimitiveInvoker>(
    args: &Args,
    run_args: &RunArgs,
    out: &mut W,
    invoker: &I,
) -> Result<(), RunnerError> {
    if !run_args.dry_run {
        validate_session_output_files(run_args)?;
    }

    let started_at = iso8601_now();
    let timestamp = started_at.clone();
    let sequence = super_step_sequence();
    let mut traces: Vec<StepTrace> = Vec::with_capacity(10);
    let mut steps_attempted = 0usize;
    let mut halted_after_role: Option<Role> = None;

    for step in sequence.iter() {
        let (bin_path, primitive_name, step_args) = build_step_invocation(args, run_args, step, &timestamp);
        let trace = StepTrace {
            index: step.index,
            name: step.name,
            primitive: primitive_name,
            args: step_args.clone(),
            bin_path: bin_path.clone(),
            // `executed` reflects the run mode this trace is being recorded
            // under. By the time `trace` is pushed (in any path), this value
            // is semantically correct: false for dry-run (we just describe
            // the step), true for live (the invoke_with_retry_once below
            // ran the primitive, whether it succeeded or failed).
            executed: !run_args.dry_run,
        };

        if run_args.dry_run {
            writeln!(out, "step {} {}: {} {}", step.index, step.name, primitive_name, step_args.join(" "))?;
            traces.push(trace);
            steps_attempted += 1;
            if let StepKind::RoleInvoke(role) = step.kind {
                if run_args.halt_after_role == Some(role) {
                    halted_after_role = Some(role);
                    break;
                }
            }
            continue;
        }

        // Execute step (live mode).
        let outcome = invoke_with_retry_once(invoker, &bin_path, &step_args, primitive_name)?;
        if !outcome.status.success() {
            let stderr = String::from_utf8_lossy(&outcome.stderr).into_owned();
            let class = classify_failure(outcome.status.code().unwrap_or(-1), &stderr);
            traces.push(trace);
            steps_attempted += 1;
            return halt_cycle(args, run_args, &started_at, step.name, class, stderr, &traces, steps_attempted, out);
        }

        traces.push(trace);
        steps_attempted += 1;

        if let StepKind::RoleInvoke(role) = step.kind {
            if run_args.halt_after_role == Some(role) {
                halted_after_role = Some(role);
                break;
            }
        }
    }

    let ended_at = iso8601_now();
    let status = if run_args.dry_run {
        "dry-run-traced"
    } else if halted_after_role.is_some() {
        "halted-after-role"
    } else {
        "completed"
    };

    let report = CycleReport {
        cycle: run_args.cycle,
        issue: run_args.issue,
        started_at,
        ended_at,
        dry_run: run_args.dry_run,
        status,
        halt_step: None,
        halt_class: None,
        halted_after_role,
        steps_attempted,
        traces,
    };

    if !run_args.dry_run {
        write_runner_state(&args.repo_root, &report)?;
    }
    emit_cycle_report(&report, args.format, out)
}

fn build_step_invocation(
    args: &Args,
    run_args: &RunArgs,
    step: &Step,
    timestamp: &str,
) -> (PathBuf, &'static str, Vec<String>) {
    let cycle_str = run_args.cycle.to_string();
    let repo_root_str = args.repo_root.to_string_lossy().into_owned();
    match step.kind {
        StepKind::SuperStepCycleStart => {
            let bin = args.super_step_boundary_bin.clone();
            let a = vec![
                "--repo-root".into(), repo_root_str,
                "cycle-start".into(),
                "--cycle".into(), cycle_str,
                "--timestamp".into(), timestamp.into(),
            ];
            (bin, "v2-super-step-boundary", a)
        }
        StepKind::ReconcilerPoll => {
            let bin = args.reconciler_event_processor_bin.clone();
            let mut a = vec![
                "--repo-root".into(), repo_root_str,
                "poll".into(),
                "--cycle".into(), cycle_str,
                "--timestamp".into(), timestamp.into(),
            ];
            if let Some(p) = &run_args.eva_source_file {
                a.push("--eva-source-file".into());
                a.push(p.to_string_lossy().into_owned());
            }
            if let Some(p) = &run_args.audit_source_file {
                a.push("--audit-source-file".into());
                a.push(p.to_string_lossy().into_owned());
            }
            if let Some(p) = &run_args.dispatch_source_file {
                a.push("--dispatch-source-file".into());
                a.push(p.to_string_lossy().into_owned());
            }
            (bin, "v2-reconciler-event-processor", a)
        }
        StepKind::RoleInvoke(role) => {
            let bin = args.role_driver_bin.clone();
            let mut a = vec![
                "--repo-root".into(), repo_root_str,
                "invoke".into(),
                "--role".into(), role.as_kebab().into(),
                "--cycle".into(), cycle_str,
                "--timestamp".into(), timestamp.into(),
            ];
            if let Some(p) = session_output_path_for(run_args, role) {
                a.push("--session-output-file".into());
                a.push(p.to_string_lossy().into_owned());
            }
            (bin, "v2-role-driver", a)
        }
        StepKind::SuperStepAdvance => {
            let bin = args.super_step_boundary_bin.clone();
            let a = vec![
                "--repo-root".into(), repo_root_str,
                "advance".into(),
                "--timestamp".into(), timestamp.into(),
            ];
            (bin, "v2-super-step-boundary", a)
        }
        StepKind::SuperStepCycleEnd => {
            let bin = args.super_step_boundary_bin.clone();
            let a = vec![
                "--repo-root".into(), repo_root_str,
                "cycle-end".into(),
                "--cycle".into(), cycle_str,
                "--timestamp".into(), timestamp.into(),
            ];
            (bin, "v2-super-step-boundary", a)
        }
    }
}

fn session_output_path_for(run_args: &RunArgs, role: Role) -> Option<&PathBuf> {
    match role {
        Role::Reconciler => run_args.reconciler_output_file.as_ref(),
        Role::Planner => run_args.planner_output_file.as_ref(),
        Role::Executor => run_args.executor_output_file.as_ref(),
        Role::Curator => run_args.curator_output_file.as_ref(),
    }
}

fn validate_session_output_files(run_args: &RunArgs) -> Result<(), RunnerError> {
    for role in [Role::Reconciler, Role::Planner, Role::Executor, Role::Curator] {
        if session_output_path_for(run_args, role).is_none() {
            return Err(RunnerError::MissingSessionOutput { role });
        }
    }
    Ok(())
}

fn invoke_with_retry_once<I: PrimitiveInvoker>(
    invoker: &I,
    bin: &Path,
    args: &[String],
    name: &'static str,
) -> Result<std::process::Output, RunnerError> {
    if !bin.exists() {
        return Err(RunnerError::PrimitiveMissing { name, path: bin.to_path_buf() });
    }
    let first = invoker.invoke(bin, args)?;
    if first.status.success() {
        return Ok(first);
    }
    let stderr = String::from_utf8_lossy(&first.stderr).into_owned();
    let class = classify_failure(first.status.code().unwrap_or(-1), &stderr);
    if class == FailureClass::Transient {
        // retry once
        let second = invoker.invoke(bin, args)?;
        return Ok(second);
    }
    Ok(first)
}

#[allow(clippy::too_many_arguments)]
fn halt_cycle<W: Write>(
    args: &Args,
    run_args: &RunArgs,
    started_at: &str,
    step_name: &'static str,
    class: FailureClass,
    stderr: String,
    traces: &[StepTrace],
    steps_attempted: usize,
    out: &mut W,
) -> Result<(), RunnerError> {
    let ended_at = iso8601_now();
    let status = if class == FailureClass::SuperStepOutOfOrder {
        "out-of-order"
    } else {
        "halted"
    };
    let report = CycleReport {
        cycle: run_args.cycle,
        issue: run_args.issue,
        started_at: started_at.to_owned(),
        ended_at,
        dry_run: false,
        status,
        halt_step: Some(step_name),
        halt_class: Some(class),
        halted_after_role: None,
        steps_attempted,
        traces: traces.to_vec(),
    };
    write_runner_state(&args.repo_root, &report)?;
    emit_cycle_report(&report, args.format, out)?;
    if class == FailureClass::SuperStepOutOfOrder {
        Err(RunnerError::SuperStepOutOfOrder { step: step_name, stderr })
    } else {
        Err(RunnerError::CycleHalted { step: step_name, class, stderr })
    }
}

fn write_runner_state(repo_root: &Path, report: &CycleReport) -> Result<(), RunnerError> {
    let runner_dir = repo_root.join("state").join("v2-cycle-runner");
    fs::create_dir_all(&runner_dir)?;

    let last_cycle_payload = serde_json::json!({
        "cycle": report.cycle,
        "issue": report.issue,
        "status": report.status,
        "started_at": report.started_at,
        "ended_at": report.ended_at,
        "halt_reason": report.halt_class.map(|c| c.as_kebab()),
        "halt_step": report.halt_step,
        "halted_after_role": report.halted_after_role.map(|r| r.as_kebab()),
        "steps_attempted": report.steps_attempted,
    });
    let last_cycle_path = runner_dir.join("last-cycle.json");
    fs::write(&last_cycle_path, format!("{}\n", serde_json::to_string_pretty(&last_cycle_payload)?))?;

    let history_path = runner_dir.join("cycle-history.json");
    let mut history: serde_json::Value = if history_path.exists() {
        let s = fs::read_to_string(&history_path)?;
        serde_json::from_str(&s).unwrap_or_else(|_| serde_json::json!({ "cycles": [] }))
    } else {
        serde_json::json!({ "cycles": [] })
    };
    let entry = serde_json::json!({
        "cycle": report.cycle,
        "issue": report.issue,
        "status": report.status,
        "started_at": report.started_at,
        "ended_at": report.ended_at,
        "halt_reason": report.halt_class.map(|c| c.as_kebab()),
        "halt_step": report.halt_step,
        "halted_after_role": report.halted_after_role.map(|r| r.as_kebab()),
        "steps_attempted": report.steps_attempted,
    });
    if let Some(arr) = history.get_mut("cycles").and_then(|v| v.as_array_mut()) {
        arr.push(entry);
    }
    fs::write(&history_path, format!("{}\n", serde_json::to_string_pretty(&history)?))?;

    Ok(())
}

fn emit_cycle_report<W: Write>(report: &CycleReport, format: Format, out: &mut W) -> Result<(), RunnerError> {
    match format {
        Format::Text => {
            writeln!(out, "v2-cycle-runner run: cycle={} issue={} status={}", report.cycle, report.issue, report.status)?;
            writeln!(out, "  started:        {}", report.started_at)?;
            writeln!(out, "  ended:          {}", report.ended_at)?;
            writeln!(out, "  dry_run:        {}", report.dry_run)?;
            writeln!(out, "  steps_attempted: {}", report.steps_attempted)?;
            if let Some(step) = report.halt_step {
                writeln!(out, "  halt_step:      {step}")?;
            }
            if let Some(class) = report.halt_class {
                writeln!(out, "  halt_class:     {}", class.as_kebab())?;
            }
            if let Some(role) = report.halted_after_role {
                writeln!(out, "  halted_after_role: {}", role.as_kebab())?;
            }
        }
        Format::Json => {
            writeln!(out, "{}", serde_json::to_string_pretty(report)?)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn default_primitive_bin_uses_cargo_target_dir_when_set() {
        let saved = env::var_os("CARGO_TARGET_DIR");
        env::set_var("CARGO_TARGET_DIR", "/tmp/some-target");
        let p = default_primitive_bin("v2-channel-router");
        assert_eq!(p, PathBuf::from("/tmp/some-target/debug/v2-channel-router"));
        match saved {
            Some(v) => env::set_var("CARGO_TARGET_DIR", v),
            None => env::remove_var("CARGO_TARGET_DIR"),
        }
    }

    #[test]
    fn default_primitive_bin_falls_back_to_local_target() {
        let saved = env::var_os("CARGO_TARGET_DIR");
        env::remove_var("CARGO_TARGET_DIR");
        let p = default_primitive_bin("v2-super-step-boundary");
        assert_eq!(p, PathBuf::from("./target/debug/v2-super-step-boundary"));
        if let Some(v) = saved {
            env::set_var("CARGO_TARGET_DIR", v);
        }
    }

    #[test]
    fn primitives_to_init_lists_four_in_super_step_order() {
        let names: Vec<&'static str> = PRIMITIVES_TO_INIT.iter().map(|(n, _)| *n).collect();
        assert_eq!(
            names,
            vec![
                "v2-channel-router",
                "v2-super-step-boundary",
                "v2-role-driver",
                "v2-reconciler-event-processor",
            ]
        );
    }

    #[test]
    fn write_initial_last_cycle_is_valid_json() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("last-cycle.json");
        write_initial_last_cycle(&p).unwrap();
        let content = fs::read_to_string(&p).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert_eq!(parsed["status"], "uninitialized");
        assert!(parsed["cycle"].is_null());
    }

    #[test]
    fn write_initial_cycle_history_is_valid_json_empty_array() {
        let dir = tempfile::tempdir().unwrap();
        let p = dir.path().join("cycle-history.json");
        write_initial_cycle_history(&p).unwrap();
        let content = fs::read_to_string(&p).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&content).unwrap();
        assert!(parsed["cycles"].is_array());
        assert_eq!(parsed["cycles"].as_array().unwrap().len(), 0);
    }

    // -------- run subcommand tests (cycle 151) --------

    use std::cell::RefCell;
    use std::os::unix::process::ExitStatusExt;
    use std::process::{ExitStatus, Output};

    fn ok_output() -> Output {
        Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        }
    }

    fn fail_output(code: i32, stderr: &str) -> Output {
        // ExitStatus::from_raw on unix encodes (signal | (exit_code << 8)).
        let raw = (code & 0xff) << 8;
        Output {
            status: ExitStatus::from_raw(raw),
            stdout: Vec::new(),
            stderr: stderr.as_bytes().to_vec(),
        }
    }

    #[derive(Default)]
    struct MockInvoker {
        calls: RefCell<Vec<(PathBuf, Vec<String>)>>,
        canned: RefCell<Vec<Output>>, // popped front-to-back
    }

    impl MockInvoker {
        fn new() -> Self {
            Self::default()
        }
        fn queue(&self, o: Output) {
            self.canned.borrow_mut().push(o);
        }
        fn calls(&self) -> Vec<(PathBuf, Vec<String>)> {
            self.calls.borrow().clone()
        }
        fn invoke_count(&self) -> usize {
            self.calls.borrow().len()
        }
    }

    impl PrimitiveInvoker for MockInvoker {
        fn invoke(&self, bin: &Path, args: &[String]) -> io::Result<Output> {
            self.calls.borrow_mut().push((bin.to_path_buf(), args.to_vec()));
            let mut canned = self.canned.borrow_mut();
            if canned.is_empty() {
                Ok(ok_output())
            } else {
                Ok(canned.remove(0))
            }
        }
    }

    fn touch(path: &Path) {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, b"").unwrap();
    }

    fn synthetic_args(tmp: &Path) -> Args {
        let bin_dir = tmp.join("bin");
        fs::create_dir_all(&bin_dir).unwrap();
        let crb = bin_dir.join("v2-channel-router");
        let ssb = bin_dir.join("v2-super-step-boundary");
        let rd = bin_dir.join("v2-role-driver");
        let rep = bin_dir.join("v2-reconciler-event-processor");
        touch(&crb);
        touch(&ssb);
        touch(&rd);
        touch(&rep);
        Args {
            repo_root: tmp.to_path_buf(),
            format: Format::Json,
            channel_router_bin: crb,
            super_step_boundary_bin: ssb,
            role_driver_bin: rd,
            reconciler_event_processor_bin: rep,
            command: Subcmd::Schema, // placeholder; tests dispatch run_cycle directly
        }
    }

    fn run_args_with_outputs(tmp: &Path, cycle: u32) -> RunArgs {
        let outs = tmp.join("outs");
        fs::create_dir_all(&outs).unwrap();
        let rec = outs.join("reconciler.json");
        let plan = outs.join("planner.json");
        let exec = outs.join("executor.json");
        let cur = outs.join("curator.json");
        for p in [&rec, &plan, &exec, &cur] {
            fs::write(p, br#"{"payload":{}}"#).unwrap();
        }
        RunArgs {
            cycle,
            issue: 999,
            dry_run: false,
            halt_after_role: None,
            eva_source_file: None,
            audit_source_file: None,
            dispatch_source_file: None,
            reconciler_output_file: Some(rec),
            planner_output_file: Some(plan),
            executor_output_file: Some(exec),
            curator_output_file: Some(cur),
        }
    }

    #[test]
    fn super_step_sequence_has_ten_steps_in_exact_order() {
        let seq = super_step_sequence();
        assert_eq!(seq.len(), 10);
        let expected_names = [
            "super-step-init",
            "reconciler-pre-poll",
            "reconciler-session",
            "super-step-advance-1",
            "planner-session",
            "super-step-advance-2",
            "executor-session",
            "super-step-advance-3",
            "curator-session",
            "super-step-settle",
        ];
        for (i, step) in seq.iter().enumerate() {
            assert_eq!(step.index as usize, i + 1, "step {} index mismatch", i + 1);
            assert_eq!(step.name, expected_names[i], "step {} name mismatch", i + 1);
        }
    }

    #[test]
    fn super_step_sequence_role_invokes_are_in_reconciler_planner_executor_curator_order() {
        let seq = super_step_sequence();
        let roles: Vec<Role> = seq
            .iter()
            .filter_map(|s| match s.kind {
                StepKind::RoleInvoke(r) => Some(r),
                _ => None,
            })
            .collect();
        assert_eq!(roles, vec![Role::Reconciler, Role::Planner, Role::Executor, Role::Curator]);
    }

    #[test]
    fn classify_failure_super_step_out_of_order() {
        let c = classify_failure(1, "super-step out of order: expected reconciler got executor");
        assert_eq!(c, FailureClass::SuperStepOutOfOrder);
        let c = classify_failure(1, "super-step boundary refused: not at expected position");
        assert_eq!(c, FailureClass::SuperStepOutOfOrder);
    }

    #[test]
    fn classify_failure_channel_write_rejected() {
        let c = classify_failure(1, "channel-router: payload rejected; missing required key 'cycle'");
        assert_eq!(c, FailureClass::ChannelWriteRejected);
        let c = classify_failure(1, "schema mismatch on plan-channel write");
        assert_eq!(c, FailureClass::ChannelWriteRejected);
    }

    #[test]
    fn classify_failure_role_session_empty() {
        let c = classify_failure(1, "role session returned empty output");
        assert_eq!(c, FailureClass::RoleSessionEmpty);
    }

    #[test]
    fn classify_failure_default_transient() {
        let c = classify_failure(1, "connection reset by peer");
        assert_eq!(c, FailureClass::Transient);
        let c = classify_failure(127, "io error: file not found");
        assert_eq!(c, FailureClass::Transient);
    }

    #[test]
    fn format_unix_secs_iso_epoch_zero() {
        assert_eq!(format_unix_secs_iso(0), "1970-01-01T00:00:00Z");
    }

    #[test]
    fn format_unix_secs_iso_y2k() {
        // 2000-01-01T00:00:00Z = 946684800.
        // Computation: 30y from 1970 = 7 leap (1972,76,80,84,88,92,96) + 23 non-leap
        //              = 7*366 + 23*365 = 10957 days × 86400 = 946,684,800s.
        assert_eq!(format_unix_secs_iso(946_684_800), "2000-01-01T00:00:00Z");
    }

    #[test]
    fn format_unix_secs_iso_leap_day_2024() {
        // 2024-02-29T12:34:56Z. 2024 is a leap year; Feb 29 exists.
        // 2024-01-01 from epoch: 54 years → 13 leap + 41 non-leap = 13*366+41*365 = 19723 days.
        // +31 (Jan) + 28 (Feb 1-28) = 59 days into 2024 for Feb 29 → day 19723+59 = 19782.
        // 19782*86400 + 12*3600 + 34*60 + 56 = 1_709_210_096.
        assert_eq!(format_unix_secs_iso(1_709_210_096), "2024-02-29T12:34:56Z");
    }

    #[test]
    fn validate_session_output_files_errors_when_any_role_missing() {
        let tmp = tempfile::tempdir().unwrap();
        let mut ra = run_args_with_outputs(tmp.path(), 1);
        ra.planner_output_file = None;
        let r = validate_session_output_files(&ra);
        assert!(matches!(r, Err(RunnerError::MissingSessionOutput { role: Role::Planner })));
    }

    #[test]
    fn validate_session_output_files_ok_when_all_four_present() {
        let tmp = tempfile::tempdir().unwrap();
        let ra = run_args_with_outputs(tmp.path(), 1);
        validate_session_output_files(&ra).unwrap();
    }

    #[test]
    fn build_step_invocation_cycle_start_includes_cycle_and_timestamp() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 42);
        let step = super_step_sequence()[0];
        let (_, prim, a) = build_step_invocation(&args, &ra, &step, "2026-05-15T10:00:00Z");
        assert_eq!(prim, "v2-super-step-boundary");
        assert!(a.contains(&"cycle-start".to_string()));
        assert!(a.contains(&"--cycle".to_string()));
        assert!(a.contains(&"42".to_string()));
        assert!(a.contains(&"2026-05-15T10:00:00Z".to_string()));
    }

    #[test]
    fn build_step_invocation_role_invoke_carries_session_output_file() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 1);
        let step = super_step_sequence()[2]; // reconciler-session
        let (_, prim, a) = build_step_invocation(&args, &ra, &step, "T");
        assert_eq!(prim, "v2-role-driver");
        assert!(a.contains(&"invoke".to_string()));
        assert!(a.contains(&"--role".to_string()));
        assert!(a.contains(&"reconciler".to_string()));
        assert!(a.contains(&"--session-output-file".to_string()));
        // The path arg follows the flag — sanity-check that one arg contains "reconciler.json"
        assert!(a.iter().any(|s| s.ends_with("reconciler.json")));
    }

    #[test]
    fn build_step_invocation_reconciler_poll_forwards_source_files_when_set() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let mut ra = run_args_with_outputs(tmp.path(), 7);
        ra.eva_source_file = Some(tmp.path().join("eva.json"));
        ra.audit_source_file = Some(tmp.path().join("audit.json"));
        ra.dispatch_source_file = Some(tmp.path().join("dispatch.json"));
        let step = super_step_sequence()[1]; // reconciler-pre-poll
        let (_, prim, a) = build_step_invocation(&args, &ra, &step, "T");
        assert_eq!(prim, "v2-reconciler-event-processor");
        assert!(a.contains(&"poll".to_string()));
        assert!(a.contains(&"--eva-source-file".to_string()));
        assert!(a.contains(&"--audit-source-file".to_string()));
        assert!(a.contains(&"--dispatch-source-file".to_string()));
    }

    #[test]
    fn build_step_invocation_advance_does_not_pass_cycle() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 1);
        let step = super_step_sequence()[3]; // super-step-advance-1
        let (_, prim, a) = build_step_invocation(&args, &ra, &step, "T");
        assert_eq!(prim, "v2-super-step-boundary");
        assert!(a.contains(&"advance".to_string()));
        // advance doesn't take --cycle
        assert!(!a.contains(&"--cycle".to_string()));
    }

    #[test]
    fn dry_run_does_not_invoke_primitives_or_mutate_state() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let mut ra = run_args_with_outputs(tmp.path(), 3);
        ra.dry_run = true;
        let mock = MockInvoker::new();
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        assert_eq!(mock.invoke_count(), 0, "dry-run must not invoke primitives");
        let runner_dir = tmp.path().join("state").join("v2-cycle-runner");
        assert!(!runner_dir.join("last-cycle.json").exists(), "dry-run must not write runner state");
    }

    #[test]
    fn live_run_invokes_ten_steps_and_writes_state_on_success() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 1);
        let mock = MockInvoker::new();
        // All 10 invocations succeed by default (canned queue is empty → ok_output()).
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        assert_eq!(mock.invoke_count(), 10);
        let last_cycle_path = tmp.path().join("state/v2-cycle-runner/last-cycle.json");
        let parsed: serde_json::Value = serde_json::from_str(&fs::read_to_string(&last_cycle_path).unwrap()).unwrap();
        assert_eq!(parsed["status"], "completed");
        assert_eq!(parsed["cycle"], 1);
        assert_eq!(parsed["issue"], 999);
        assert_eq!(parsed["steps_attempted"], 10);

        let history_path = tmp.path().join("state/v2-cycle-runner/cycle-history.json");
        let history: serde_json::Value = serde_json::from_str(&fs::read_to_string(&history_path).unwrap()).unwrap();
        let arr = history["cycles"].as_array().unwrap();
        assert_eq!(arr.len(), 1);
        assert_eq!(arr[0]["status"], "completed");
    }

    #[test]
    fn live_run_halts_when_role_session_returns_empty_output() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 2);
        let mock = MockInvoker::new();
        // Step order: step 1 (boundary cycle-start) ok, step 2 (reconciler poll) ok,
        // step 3 (role-driver invoke reconciler) FAIL with role-session-empty stderr.
        mock.queue(ok_output()); // step 1
        mock.queue(ok_output()); // step 2
        mock.queue(fail_output(1, "role session returned empty output")); // step 3 first
        mock.queue(fail_output(1, "role session returned empty output")); // step 3 retry-once would be classified non-transient → no retry, but queue extra in case
        let mut out = Vec::new();
        let r = run_cycle(&args, &ra, &mut out, &mock);
        match r {
            Err(RunnerError::CycleHalted { step, class, .. }) => {
                assert_eq!(step, "reconciler-session");
                assert_eq!(class, FailureClass::RoleSessionEmpty);
            }
            other => panic!("expected halt, got {other:?}"),
        }
        // Halt-state written.
        let last_cycle_path = tmp.path().join("state/v2-cycle-runner/last-cycle.json");
        let parsed: serde_json::Value = serde_json::from_str(&fs::read_to_string(&last_cycle_path).unwrap()).unwrap();
        assert_eq!(parsed["status"], "halted");
        assert_eq!(parsed["halt_step"], "reconciler-session");
        assert_eq!(parsed["halt_reason"], "role-session-empty");
    }

    #[test]
    fn live_run_hard_errors_on_super_step_out_of_order() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 5);
        let mock = MockInvoker::new();
        // Step 1 fails with super-step-out-of-order
        mock.queue(fail_output(1, "super-step out of order: not at expected position"));
        let mut out = Vec::new();
        let r = run_cycle(&args, &ra, &mut out, &mock);
        match r {
            Err(RunnerError::SuperStepOutOfOrder { step, .. }) => {
                assert_eq!(step, "super-step-init");
            }
            other => panic!("expected out-of-order error, got {other:?}"),
        }
        let last_cycle_path = tmp.path().join("state/v2-cycle-runner/last-cycle.json");
        let parsed: serde_json::Value = serde_json::from_str(&fs::read_to_string(&last_cycle_path).unwrap()).unwrap();
        assert_eq!(parsed["status"], "out-of-order");
        assert_eq!(parsed["halt_reason"], "super-step-out-of-order");
    }

    #[test]
    fn live_run_retries_transient_failure_once() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 6);
        let mock = MockInvoker::new();
        // step 1: transient fail, then retry succeeds. step 2-10 succeed.
        mock.queue(fail_output(1, "connection reset"));
        mock.queue(ok_output()); // retry
        // steps 2-10 default-ok.
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        // 10 logical steps + 1 retry = 11 invocations.
        assert_eq!(mock.invoke_count(), 11);
    }

    #[test]
    fn live_run_halt_after_role_short_circuits_before_advance() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let mut ra = run_args_with_outputs(tmp.path(), 8);
        ra.halt_after_role = Some(Role::Planner);
        let mock = MockInvoker::new();
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        // Steps 1..=5 (cycle-start, poll, reconciler-session, advance-1, planner-session) = 5.
        assert_eq!(mock.invoke_count(), 5);
        let last_cycle_path = tmp.path().join("state/v2-cycle-runner/last-cycle.json");
        let parsed: serde_json::Value = serde_json::from_str(&fs::read_to_string(&last_cycle_path).unwrap()).unwrap();
        assert_eq!(parsed["status"], "halted-after-role");
        assert_eq!(parsed["halted_after_role"], "planner");
        assert_eq!(parsed["steps_attempted"], 5);
    }

    #[test]
    fn live_run_missing_session_output_file_errors_before_invoking() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let mut ra = run_args_with_outputs(tmp.path(), 9);
        ra.executor_output_file = None;
        let mock = MockInvoker::new();
        let mut out = Vec::new();
        let r = run_cycle(&args, &ra, &mut out, &mock);
        assert!(matches!(r, Err(RunnerError::MissingSessionOutput { role: Role::Executor })));
        assert_eq!(mock.invoke_count(), 0, "must not invoke primitives if validation fails");
    }

    #[test]
    fn dry_run_traces_have_executed_false() {
        // C5 (L1.5) per-step disambiguation: each trace's `executed` must
        // reflect whether the primitive actually ran. In dry-run, every
        // trace describes a step that was skipped — `executed=false`.
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let mut ra = run_args_with_outputs(tmp.path(), 10);
        ra.dry_run = true;
        let mock = MockInvoker::new();
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        // Dry-run path also writes "step N name: ..." lines to stdout, so
        // strip everything before the first `{` to isolate the JSON report.
        let s = String::from_utf8_lossy(&out).into_owned();
        let json_start = s.find('{').expect("expected JSON report in stdout");
        let report: serde_json::Value = serde_json::from_str(&s[json_start..]).unwrap();
        let traces = report["traces"].as_array().unwrap();
        assert_eq!(traces.len(), 10);
        for (i, t) in traces.iter().enumerate() {
            assert_eq!(
                t["executed"].as_bool(),
                Some(false),
                "dry-run trace {i} expected executed=false, got {t:?}"
            );
        }
    }

    #[test]
    fn live_success_traces_have_executed_true() {
        // Each trace in a live successful run must carry executed=true.
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 11);
        let mock = MockInvoker::new();
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        let report: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&out)).unwrap();
        let traces = report["traces"].as_array().unwrap();
        assert_eq!(traces.len(), 10);
        for (i, t) in traces.iter().enumerate() {
            assert_eq!(
                t["executed"].as_bool(),
                Some(true),
                "live-success trace {i} expected executed=true, got {t:?}"
            );
        }
    }

    #[test]
    fn live_halt_pushed_trace_has_executed_true() {
        // When a primitive invocation fails (live mode, non-transient), the
        // trace for THAT step is still pushed before halt_cycle is called.
        // It must carry executed=true because the primitive DID run — the
        // failure is part of the observable outcome.
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 12);
        let mock = MockInvoker::new();
        // Step 1 ok, step 2 ok, step 3 (reconciler-session) fails with
        // role-session-empty.
        mock.queue(ok_output());
        mock.queue(ok_output());
        mock.queue(fail_output(1, "role session returned empty output"));
        let mut out = Vec::new();
        let r = run_cycle(&args, &ra, &mut out, &mock);
        assert!(matches!(r, Err(RunnerError::CycleHalted { .. })));
        // halt_cycle re-serializes a synthetic report with the partial traces.
        // Recover from the JSON written to out — emit_report path in halt.
        let s = String::from_utf8_lossy(&out).into_owned();
        let parsed: serde_json::Value = serde_json::from_str(&s).unwrap();
        let traces = parsed["traces"].as_array().unwrap();
        assert_eq!(traces.len(), 3, "expected 3 traces (2 ok + 1 fail), got {}", traces.len());
        for (i, t) in traces.iter().enumerate() {
            assert_eq!(
                t["executed"].as_bool(),
                Some(true),
                "live-halt trace {i} expected executed=true, got {t:?}"
            );
        }
    }

    #[test]
    fn mock_invoker_records_args_in_order() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 1);
        let mock = MockInvoker::new();
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        let calls = mock.calls();
        // First call should be to super-step-boundary cycle-start.
        assert!(calls[0].0.file_name().unwrap() == "v2-super-step-boundary");
        assert!(calls[0].1.contains(&"cycle-start".to_string()));
        // Second call: reconciler-event-processor poll.
        assert!(calls[1].0.file_name().unwrap() == "v2-reconciler-event-processor");
        assert!(calls[1].1.contains(&"poll".to_string()));
        // Last call: super-step-boundary cycle-end.
        assert!(calls[9].0.file_name().unwrap() == "v2-super-step-boundary");
        assert!(calls[9].1.contains(&"cycle-end".to_string()));
    }
}
