// v2-cycle-runner
//
// Conductor for the v2 multi-agent orchestrator: composes the 4 role-prompt sessions
// and the 4 v2-* primitives (channel-router, super-step-boundary, role-driver,
// reconciler-event-processor) into a single per-cycle execution.
//
// ## Scope explicitly OUT (cycle 161 X5 carveout)
//
// **Commit-shape / commit-governance discipline is OUT OF SCOPE for this crate.**
//
// PR #2961 cycle 152 critique X5 raised that the runner does not verify
// single-direct-push or push-cleanliness assumptions named in redesign notes.
// Cycle 155 absorption verdict was AGREE-WITH-CARVEOUT: the issue is real, but
// the resolution is "commit governance lives outside v2-cycle-runner." Adding
// commit-cleanliness checks here would make the runner a polymath, violating
// the CORE-DESIGN-PRINCIPLE separation between (a) the runner's per-cycle
// step-execution responsibility and (b) sibling-tool responsibilities like
// state-surface auditing (v2-state-audit), dispatch-state reconciliation
// (v2-state-dispatch-sync), and channel-write validation (v2-channel-router).
//
// If commit-shape verification is wanted, a separate sibling tool
// (e.g., `v2-commit-discipline-check`) should own it. This crate's `RunReport`
// and `StepTrace` record cycle/issue context for downstream consumers; they
// do not assert anything about commit cleanliness, push topology, or
// branch-state hygiene.
//
// Forward-pointer: see `docs/redesign/_notes/cycle-149-v2-cycle-runner-design-scope.md`
// for the runner's named scope; see `docs/redesign/_critique/cycle-152-v2-cycle-runner-adversarial-critique.md`
// X5 (line 146) for the original finding; see `docs/redesign/_notes/cycle-155-cycle-152-critique-absorption.md`
// X5 entry for the AGREE-WITH-CARVEOUT verdict and the cycle 161 closure here.

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
             reconciler-event-processor) into a single per-cycle execution. Subcommands: \
             init / schema (cycle 150), run with --dry-run + execute paths against \
             pre-provided session-output files (cycle 151; cycle 162 added pre-flight \
             v2-state-audit), status + verify read-only inspection (cycle 169)."
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

    /// Path to v2-state-audit binary. Invoked as a pre-flight check at
    /// session-start before the 10-step super-step sequence (cycle 162
    /// wiring; CORE-DESIGN-PRINCIPLE enforcer layer per
    /// docs/redesign/_notes/v2-state-dispatch-policy-enforcement.md §2.3).
    /// When the audit returns exit code 3 (Hard severity), the runner
    /// halts the cycle with halt_reason=state-bound-exceeded BEFORE any
    /// super-step state mutation occurs.
    #[arg(long, default_value_os_t = default_primitive_bin("v2-state-audit"), global = true)]
    state_audit_bin: PathBuf,

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
    /// Report the runner's most recent (or `--cycle N`) cycle execution state.
    /// Reads `state/v2-cycle-runner/last-cycle.json` by default; with `--cycle N`
    /// linearly scans `state/v2-cycle-runner/cycle-history.json`. With
    /// `--include-primitives` additionally reads the 4 primitive state surfaces
    /// for richer context. Read-only; cycle 169 implementation per
    /// `docs/redesign/_notes/v2-cycle-runner-status-verify-design.md` §2.1.
    Status {
        /// Read history entry for cycle N. If omitted, reads `last-cycle.json`.
        #[arg(long)]
        cycle: Option<u32>,
        /// Additionally read the 4 primitive state surfaces (super-step.json,
        /// roles/*.json, reconciler/poll-history.json) for richer context.
        /// Slower; default off.
        #[arg(long)]
        include_primitives: bool,
    },
    /// Post-cycle self-check: did cycle N complete cleanly per the runner's
    /// expectations? Runs the 9-assertion verify algorithm against the runner's
    /// own state files + super-step-history + per-role histories. Read-only.
    /// `--cycle N` is REQUIRED (no accidental "verify latest"). Cycle 169
    /// implementation per design doc §2.2.
    Verify {
        /// Cycle to verify (required).
        #[arg(long)]
        cycle: u32,
        /// Exit non-zero on any assertion failure (default exits 0 with
        /// failure-reported JSON).
        #[arg(long)]
        strict: bool,
    },
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
    /// status subcommand could not locate `last-cycle.json` or the requested
    /// `--cycle N` in cycle-history. Cycle 169.
    StatusStateNotFound { detail: String },
    /// verify --strict was passed and at least one assertion failed. The
    /// dirty-verdict report has already been written to stdout; this variant
    /// only signals the exit-1 to `main`. Cycle 169.
    VerifyDirtyStrict { failed_assertions: usize, total_assertions: usize },
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
            RunnerError::StatusStateNotFound { detail } => write!(f, "{detail}"),
            RunnerError::VerifyDirtyStrict { failed_assertions, total_assertions } => write!(
                f,
                "verify --strict: {failed_assertions} of {total_assertions} assertions failed (see verdict above)"
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
        Subcmd::Status { cycle, include_primitives } => {
            run_status(args, *cycle, *include_primitives, out)
        }
        Subcmd::Verify { cycle, strict } => run_verify(args, *cycle, *strict, out),
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
            writeln!(out, "Subcommands implemented:")?;
            writeln!(out, "  init     ← composes primitive inits + initializes runner-self state (cycle 150)")?;
            writeln!(out, "  schema   ← prints this (cycle 150)")?;
            writeln!(out, "  run      ← drives the 10-step super-step sequence (cycle 151;")?;
            writeln!(out, "             cycle 162 added pre-flight state-audit; cycle 163 added")?;
            writeln!(out, "             super-step-out-of-order live test coverage)")?;
            writeln!(out, "  status   ← read runner's last-cycle or --cycle N execution state (cycle 169)")?;
            writeln!(out, "  verify   ← post-cycle 9-assertion self-check on --cycle N (cycle 169)")?;
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
                "subcommands_implemented": ["init", "schema", "run", "status", "verify"],
                "subcommands_deferred": [],
                "cycle_scope": "169: run/status/verify all live; init+schema cycle 150"
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

/// Phase category for a step, derived from `StepKind`. Per cycle 152
/// v2-cycle-runner critique C2 (L1.2): the linear step list mixes
/// semantic-workflow phases (reconciler/planner/executor/curator) with
/// transition mechanics (super-step-advance-*) and cycle boundaries
/// (super-step-init/settle). Phase exposes this categorization so trace
/// consumers can group steps by semantic intent without parsing
/// step-name strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Phase {
    /// Cycle boundary: super-step-init or super-step-settle.
    Boundary,
    /// Reconciler phase: poll (preparatory) + reconciler-session.
    Reconciler,
    /// Planner phase: planner-session.
    Planner,
    /// Executor phase: executor-session.
    Executor,
    /// Curator phase: curator-session.
    Curator,
    /// Inter-role transition mechanics: super-step-advance-N.
    Transition,
}

fn phase_for(kind: StepKind) -> Phase {
    match kind {
        StepKind::SuperStepCycleStart | StepKind::SuperStepCycleEnd => Phase::Boundary,
        StepKind::ReconcilerPoll => Phase::Reconciler,
        StepKind::RoleInvoke(Role::Reconciler) => Phase::Reconciler,
        StepKind::RoleInvoke(Role::Planner) => Phase::Planner,
        StepKind::RoleInvoke(Role::Executor) => Phase::Executor,
        StepKind::RoleInvoke(Role::Curator) => Phase::Curator,
        StepKind::SuperStepAdvance => Phase::Transition,
    }
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
    /// Session-start `v2-state-audit` reported `hard` severity (exit 3).
    /// 5th halt class per cycle 149 §3 enumeration extension; named
    /// `state-bound-as-halt-reason` in v2-state-retention-policy.md §7
    /// and wired here cycle 162 (cycle 161 forward priority #1 closure).
    /// Halts BEFORE the super-step sequence begins — no super-step state
    /// mutation occurs.
    StateBoundExceeded,
}

impl FailureClass {
    fn as_kebab(self) -> &'static str {
        match self {
            FailureClass::Transient => "transient",
            FailureClass::RoleSessionEmpty => "role-session-empty",
            FailureClass::ChannelWriteRejected => "channel-write-rejected",
            FailureClass::SuperStepOutOfOrder => "super-step-out-of-order",
            FailureClass::StateBoundExceeded => "state-bound-exceeded",
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

// =====================================================================
// State-audit pre-flight (cycle 162 wiring)
// =====================================================================

/// Mirror of `v2-state-audit`'s exit-code → Kind mapping. See
/// `tools/rust/crates/v2-state-audit/src/main.rs::exit_code_for`.
/// `SerializationFailure` corresponds to audit's exit 4 (JSON-serialize
/// error inside the audit tool itself). `Unknown` covers any other
/// non-zero exit (e.g. process killed, panics, future audit codes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum StateAuditSeverity {
    Ok,
    Advisory,
    Mandatory,
    Hard,
    SerializationFailure,
    Unknown,
}

impl StateAuditSeverity {
    fn from_exit_code(code: i32) -> Self {
        match code {
            0 => Self::Ok,
            1 => Self::Advisory,
            2 => Self::Mandatory,
            3 => Self::Hard,
            4 => Self::SerializationFailure,
            _ => Self::Unknown,
        }
    }

    fn as_kebab(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::Advisory => "advisory",
            Self::Mandatory => "mandatory",
            Self::Hard => "hard",
            Self::SerializationFailure => "serialization-failure",
            Self::Unknown => "unknown",
        }
    }

    /// True iff the audit result mandates halting the cycle before the
    /// super-step sequence begins. Only `Hard` halts (cycle 158 policy §7
    /// `state-bound-as-halt-reason` is hard-threshold-specific).
    /// `SerializationFailure` and `Unknown` are reported but do not halt —
    /// failing-open on tool-internal anomalies preserves the cycle's
    /// ability to make progress and surfaces the issue in the cycle report
    /// rather than silently halting.
    fn requires_halt(self) -> bool {
        matches!(self, Self::Hard)
    }
}

/// Outcome of the pre-flight `v2-state-audit` invocation. `None` on the
/// `CycleReport.state_audit` field means dry-run skipped the audit.
#[derive(Debug, Clone, Serialize)]
struct StateAuditOutcome {
    bin_path: PathBuf,
    args: Vec<String>,
    exit_code: i32,
    severity: StateAuditSeverity,
    /// Audit tool's stdout — the JSON report when invoked with `--json`,
    /// or the human summary otherwise. Captured verbatim for post-hoc
    /// inspection.
    stdout: String,
    /// Audit tool's stderr — empty on success; populated on tool-internal
    /// failures.
    stderr: String,
    /// True if the audit binary was actually invoked. False is reserved
    /// for future opt-out paths; current `run_cycle` always invokes in
    /// live mode and never records an `executed=false` outcome (dry-run
    /// records `None` on the report instead).
    executed: bool,
}

fn invoke_state_audit_on_start<I: PrimitiveInvoker>(
    args: &Args,
    invoker: &I,
) -> Result<StateAuditOutcome, RunnerError> {
    let bin = args.state_audit_bin.as_path();
    if !bin.exists() {
        return Err(RunnerError::PrimitiveMissing {
            name: "v2-state-audit",
            path: bin.to_path_buf(),
        });
    }
    let invocation_args = vec![
        "--repo-root".to_string(),
        args.repo_root.to_string_lossy().into_owned(),
        "--json".to_string(),
    ];
    let output = invoker.invoke(bin, &invocation_args)?;
    let exit_code = output.status.code().unwrap_or(-1);
    let severity = StateAuditSeverity::from_exit_code(exit_code);
    Ok(StateAuditOutcome {
        bin_path: bin.to_path_buf(),
        args: invocation_args,
        exit_code,
        severity,
        stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        executed: true,
    })
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
    /// Phase category derived from `StepKind`. Distinguishes semantic-
    /// workflow phases (reconciler/planner/executor/curator) from
    /// transition mechanics (super-step-advance-*) and cycle boundaries
    /// (super-step-init/settle). Per cycle 152 v2-cycle-runner critique
    /// C2 (L1.2) absorbed cycle 159.
    phase: Phase,
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
    /// Session-start `v2-state-audit` outcome. `None` in dry-run mode
    /// (audit is skipped along with primitive invocations); `Some` in
    /// live mode regardless of audit severity. Cycle 162 wiring.
    state_audit: Option<StateAuditOutcome>,
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

    // Pre-flight session-start state-audit (cycle 162). Skipped under
    // dry-run, consistent with primitive invocations being skipped. In
    // live mode: on `Hard` severity, halt BEFORE the super-step sequence
    // begins — no super-step state mutation occurs. On any other
    // severity, the outcome is recorded in the cycle report and the
    // cycle proceeds.
    let state_audit: Option<StateAuditOutcome> = if run_args.dry_run {
        None
    } else {
        let outcome = invoke_state_audit_on_start(args, invoker)?;
        if outcome.severity.requires_halt() {
            return halt_cycle_at_state_audit(args, run_args, &started_at, outcome, out);
        }
        Some(outcome)
    };

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
            phase: phase_for(step.kind),
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
            return halt_cycle(
                args,
                run_args,
                &started_at,
                step.name,
                class,
                stderr,
                &traces,
                steps_attempted,
                state_audit.clone(),
                out,
            );
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
        state_audit,
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
    state_audit: Option<StateAuditOutcome>,
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
        state_audit,
    };
    write_runner_state(&args.repo_root, &report)?;
    emit_cycle_report(&report, args.format, out)?;
    if class == FailureClass::SuperStepOutOfOrder {
        Err(RunnerError::SuperStepOutOfOrder { step: step_name, stderr })
    } else {
        Err(RunnerError::CycleHalted { step: step_name, class, stderr })
    }
}

/// Halt routine specific to the cycle 162 session-start state-audit
/// pre-flight check. Routes through the standard `CycleHalted` error
/// path with `step="state-audit-on-start"` and
/// `class=FailureClass::StateBoundExceeded`. The synthetic stderr names
/// the audit severity and exit code; the audit's full stdout (JSON
/// report) is preserved on the cycle report's `state_audit` field for
/// post-hoc inspection.
fn halt_cycle_at_state_audit<W: Write>(
    args: &Args,
    run_args: &RunArgs,
    started_at: &str,
    outcome: StateAuditOutcome,
    out: &mut W,
) -> Result<(), RunnerError> {
    let stderr = format!(
        "v2-state-audit returned severity={} (exit_code={}); \
         hard-threshold breach requires operator review before next cycle. \
         Re-run `v2-state-audit --json` for per-axis breakdown; \
         see docs/redesign/_notes/v2-state-dispatch-policy-enforcement.md \
         for the three-layer-ownership framing and archival options.",
        outcome.severity.as_kebab(),
        outcome.exit_code,
    );
    halt_cycle(
        args,
        run_args,
        started_at,
        "state-audit-on-start",
        FailureClass::StateBoundExceeded,
        stderr,
        &[],
        0,
        Some(outcome),
        out,
    )
}

fn write_runner_state(repo_root: &Path, report: &CycleReport) -> Result<(), RunnerError> {
    let runner_dir = repo_root.join("state").join("v2-cycle-runner");
    fs::create_dir_all(&runner_dir)?;

    // Trimmed per-step trace persisted to disk. The in-memory `StepTrace`
    // carries `args` (Vec<String>) and `bin_path` (PathBuf) which can be
    // re-derived from `args.{primitive}_bin` + `build_step_invocation`.
    // We persist only what `verify` needs (index, name, executed, phase)
    // to keep cycle-history.json bounded. Cycle 169 additive extension
    // for the verify subcommand's all-10-substeps-traced assertion;
    // backwards-compatible (older entries lack the field; verify treats
    // missing-traces as the assertion failing per cycle 169 _notes).
    let traces_persisted: Vec<serde_json::Value> = report
        .traces
        .iter()
        .map(|t| serde_json::json!({
            "index": t.index,
            "name": t.name,
            "executed": t.executed,
            "phase": t.phase,
        }))
        .collect();

    // Trimmed state-audit persisted to disk: just severity + exit_code.
    // Audit's full JSON stdout is large and re-readable from the audit
    // binary at any time. Cycle 169 additive extension for verify's
    // state-audit-not-hard assertion.
    let state_audit_persisted: Option<serde_json::Value> =
        report.state_audit.as_ref().map(|sa| serde_json::json!({
            "severity": sa.severity.as_kebab(),
            "exit_code": sa.exit_code,
        }));

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
        "traces": traces_persisted,
        "state_audit": state_audit_persisted,
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
    let entry = last_cycle_payload.clone();
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
            if let Some(state_audit) = &report.state_audit {
                writeln!(
                    out,
                    "  state_audit:    severity={} (exit_code={})",
                    state_audit.severity.as_kebab(),
                    state_audit.exit_code,
                )?;
            }
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

// =====================================================================
// status subcommand — cycle 169 implementation
//
// Reads `state/v2-cycle-runner/last-cycle.json` by default; with `--cycle N`
// linearly scans `state/v2-cycle-runner/cycle-history.json` for the entry
// matching cycle == N. With `--include-primitives` additionally reads the
// 4 primitive state surfaces for richer context.
//
// Per design doc §2.1; resolves cycle 149 §4 high-level naming.
// =====================================================================

fn run_status<W: Write>(
    args: &Args,
    cycle: Option<u32>,
    include_primitives: bool,
    out: &mut W,
) -> Result<(), RunnerError> {
    let runner_dir = args.repo_root.join("state").join("v2-cycle-runner");

    let cycle_entry = match cycle {
        None => read_last_cycle_json(&runner_dir)?,
        Some(n) => find_cycle_in_history(&runner_dir, n)?,
    };

    let primitives = if include_primitives {
        Some(read_primitive_state_surfaces(&args.repo_root))
    } else {
        None
    };

    emit_status(&cycle_entry, primitives.as_ref(), include_primitives, args.format, out)
}

fn read_last_cycle_json(runner_dir: &Path) -> Result<serde_json::Value, RunnerError> {
    let path = runner_dir.join("last-cycle.json");
    if !path.exists() {
        return Err(RunnerError::StatusStateNotFound {
            detail: format!(
                "state file {} does not exist (run `v2-cycle-runner init` first)",
                path.display()
            ),
        });
    }
    let raw = fs::read_to_string(&path)?;
    let value: serde_json::Value = serde_json::from_str(&raw)?;
    Ok(value)
}

fn find_cycle_in_history(runner_dir: &Path, n: u32) -> Result<serde_json::Value, RunnerError> {
    let path = runner_dir.join("cycle-history.json");
    if !path.exists() {
        return Err(RunnerError::StatusStateNotFound {
            detail: format!(
                "state file {} does not exist (run `v2-cycle-runner init` first)",
                path.display()
            ),
        });
    }
    let raw = fs::read_to_string(&path)?;
    let history: serde_json::Value = serde_json::from_str(&raw)?;
    let cycles = history
        .get("cycles")
        .and_then(|v| v.as_array())
        .ok_or_else(|| RunnerError::StatusStateNotFound {
            detail: format!("{} is missing the `cycles` array", path.display()),
        })?;
    for entry in cycles {
        if entry.get("cycle").and_then(|v| v.as_u64()) == Some(u64::from(n)) {
            return Ok(entry.clone());
        }
    }
    Err(RunnerError::StatusStateNotFound {
        detail: format!("cycle {n} not found in {}", path.display()),
    })
}

/// Read the 4 primitive state surfaces for `status --include-primitives`.
/// Each surface is reported as a JSON value (`null` if the file is missing
/// or unreadable — degraded reporting, not a hard error, so an
/// incompletely-initialized state does not crash status).
fn read_primitive_state_surfaces(repo_root: &Path) -> serde_json::Value {
    let read_or_null = |p: PathBuf| -> serde_json::Value {
        match fs::read_to_string(&p) {
            Ok(s) => serde_json::from_str(&s).unwrap_or(serde_json::Value::Null),
            Err(_) => serde_json::Value::Null,
        }
    };
    serde_json::json!({
        "super_step": read_or_null(repo_root.join("state").join("super-step.json")),
        "super_step_history": read_or_null(repo_root.join("state").join("super-step-history.json")),
        "roles": {
            "reconciler": read_or_null(repo_root.join("state").join("roles").join("reconciler-history.json")),
            "planner":    read_or_null(repo_root.join("state").join("roles").join("planner-history.json")),
            "executor":   read_or_null(repo_root.join("state").join("roles").join("executor-history.json")),
            "curator":    read_or_null(repo_root.join("state").join("roles").join("curator-history.json")),
        },
        "reconciler_poll_history": read_or_null(repo_root.join("state").join("reconciler").join("poll-history.json")),
    })
}

fn emit_status<W: Write>(
    cycle_entry: &serde_json::Value,
    primitives: Option<&serde_json::Value>,
    include_primitives: bool,
    format: Format,
    out: &mut W,
) -> Result<(), RunnerError> {
    let cycle_num = cycle_entry.get("cycle").cloned().unwrap_or(serde_json::Value::Null);
    let issue = cycle_entry.get("issue").cloned().unwrap_or(serde_json::Value::Null);
    let status = cycle_entry.get("status").and_then(|v| v.as_str()).unwrap_or("unknown");
    let started_at = cycle_entry.get("started_at").and_then(|v| v.as_str()).unwrap_or("(none)");
    let ended_at = cycle_entry.get("ended_at").and_then(|v| v.as_str()).unwrap_or("(none)");
    let steps_attempted = cycle_entry
        .get("steps_attempted")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let traces_len = cycle_entry
        .get("traces")
        .and_then(|v| v.as_array())
        .map(|a| a.len())
        .unwrap_or(0);
    let halt_step = cycle_entry.get("halt_step").and_then(|v| v.as_str());
    let halt_class = cycle_entry.get("halt_reason").and_then(|v| v.as_str());
    let halted_after_role = cycle_entry.get("halted_after_role").and_then(|v| v.as_str());
    let state_audit_severity = cycle_entry
        .get("state_audit")
        .and_then(|sa| sa.get("severity"))
        .and_then(|v| v.as_str());

    match format {
        Format::Text => {
            writeln!(
                out,
                "v2-cycle-runner status: cycle={cycle_num} issue={issue} status={status}"
            )?;
            writeln!(out, "  started:  {started_at}")?;
            writeln!(out, "  ended:    {ended_at}")?;
            writeln!(out, "  steps:    {steps_attempted} attempted, {traces_len} traced")?;
            match (halt_step, halt_class, halted_after_role) {
                (None, None, None) => writeln!(out, "  halt:     (none)")?,
                _ => {
                    let s = halt_step.unwrap_or("(none)");
                    let c = halt_class.unwrap_or("(none)");
                    let r = halted_after_role.unwrap_or("(none)");
                    writeln!(out, "  halt:     step={s} class={c} after_role={r}")?;
                }
            }
            writeln!(out, "  audit:    {}", state_audit_severity.unwrap_or("(none)"))?;
            if include_primitives {
                writeln!(out)?;
                writeln!(out, "  primitives: included (see JSON for full surfaces)")?;
            }
        }
        Format::Json => {
            let mut payload = serde_json::json!({
                "schema_version": "v1",
                "subcommand": "status",
                "cycle": cycle_num,
                "issue": issue,
                "status": status,
                "started_at": cycle_entry.get("started_at").cloned().unwrap_or(serde_json::Value::Null),
                "ended_at": cycle_entry.get("ended_at").cloned().unwrap_or(serde_json::Value::Null),
                "steps_attempted": steps_attempted,
                "traces_count": traces_len,
                "halt_step": cycle_entry.get("halt_step").cloned().unwrap_or(serde_json::Value::Null),
                "halt_class": cycle_entry.get("halt_reason").cloned().unwrap_or(serde_json::Value::Null),
                "halted_after_role": cycle_entry.get("halted_after_role").cloned().unwrap_or(serde_json::Value::Null),
                "state_audit": state_audit_severity.map(serde_json::Value::from).unwrap_or(serde_json::Value::Null),
                "include_primitives": include_primitives,
            });
            if let Some(p) = primitives {
                payload["primitives"] = p.clone();
            }
            writeln!(out, "{}", serde_json::to_string_pretty(&payload)?)?;
        }
    }
    Ok(())
}

// =====================================================================
// verify subcommand — cycle 169 implementation
//
// 9-assertion verify algorithm per design doc §2.2. Reads:
//   - state/v2-cycle-runner/cycle-history.json (assertions 1-7)
//   - state/super-step-history.json            (assertion 8)
//   - state/roles/{reconciler,planner,executor,curator}-history.json (assertion 9)
//
// On any failure with --strict, exits 1 (via RunnerError::VerifyDirtyStrict
// after the dirty-verdict report has been written to stdout).
// Without --strict, always exits 0 with the verdict in JSON.
// =====================================================================

const VERIFY_ASSERTION_NAMES: &[&str] = &[
    "cycle-N-in-history",
    "status-completed",
    "no-halt-step",
    "no-halt-class",
    "no-halted-after-role",
    "all-10-substeps-traced",
    "state-audit-not-hard",
    "super-step-history-has-cycle-N",
    "per-role-history-has-cycle-N",
];

#[derive(Debug, Clone, Serialize)]
struct AssertionResult {
    name: &'static str,
    passed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<String>,
}

impl AssertionResult {
    fn pass(name: &'static str) -> Self {
        Self { name, passed: true, details: None }
    }
    fn fail(name: &'static str, details: impl Into<String>) -> Self {
        Self { name, passed: false, details: Some(details.into()) }
    }
}

fn run_verify<W: Write>(
    args: &Args,
    cycle: u32,
    strict: bool,
    out: &mut W,
) -> Result<(), RunnerError> {
    let runner_dir = args.repo_root.join("state").join("v2-cycle-runner");
    let assertions = run_verify_assertions(&args.repo_root, &runner_dir, cycle);
    let failed_count = assertions.iter().filter(|a| !a.passed).count();
    let total = assertions.len();
    let verdict = if failed_count == 0 { "clean" } else { "dirty" };
    emit_verify(cycle, verdict, &assertions, args.format, out)?;

    if strict && failed_count > 0 {
        Err(RunnerError::VerifyDirtyStrict {
            failed_assertions: failed_count,
            total_assertions: total,
        })
    } else {
        Ok(())
    }
}

/// Run the 9-assertion verify algorithm. Returns the results in
/// declaration order. Short-circuit: assertion 1 failing returns ONLY
/// the first assertion (no downstream short-circuit results recorded
/// because the entry isn't present — assertions 2-7 require the entry).
/// Assertions 8 and 9 always run regardless of 1's outcome (they read
/// different state surfaces).
fn run_verify_assertions(repo_root: &Path, runner_dir: &Path, cycle: u32) -> Vec<AssertionResult> {
    let mut results: Vec<AssertionResult> = Vec::with_capacity(9);

    // Assertion 1: cycle-N-in-history
    let history_path = runner_dir.join("cycle-history.json");
    let cycle_entry = match read_cycle_entry(&history_path, cycle) {
        Ok(Some(e)) => {
            results.push(AssertionResult::pass(VERIFY_ASSERTION_NAMES[0]));
            Some(e)
        }
        Ok(None) => {
            results.push(AssertionResult::fail(
                VERIFY_ASSERTION_NAMES[0],
                format!("no entry with cycle == {cycle} in {}", history_path.display()),
            ));
            None
        }
        Err(e) => {
            results.push(AssertionResult::fail(
                VERIFY_ASSERTION_NAMES[0],
                format!("failed to read {}: {e}", history_path.display()),
            ));
            None
        }
    };

    if let Some(entry) = cycle_entry.as_ref() {
        // Assertion 2: status-completed
        let status = entry.get("status").and_then(|v| v.as_str()).unwrap_or("(missing)");
        if status == "completed" {
            results.push(AssertionResult::pass(VERIFY_ASSERTION_NAMES[1]));
        } else {
            results.push(AssertionResult::fail(
                VERIFY_ASSERTION_NAMES[1],
                format!("status is '{status}', expected 'completed'"),
            ));
        }

        // Assertion 3: no-halt-step
        match entry.get("halt_step") {
            Some(v) if v.is_null() => results.push(AssertionResult::pass(VERIFY_ASSERTION_NAMES[2])),
            Some(v) if v.as_str().is_some() => results.push(AssertionResult::fail(
                VERIFY_ASSERTION_NAMES[2],
                format!("halt_step is {:?}", v.as_str().unwrap()),
            )),
            _ => results.push(AssertionResult::pass(VERIFY_ASSERTION_NAMES[2])),
        }

        // Assertion 4: no-halt-class (persisted as `halt_reason`)
        match entry.get("halt_reason") {
            Some(v) if v.is_null() => results.push(AssertionResult::pass(VERIFY_ASSERTION_NAMES[3])),
            Some(v) if v.as_str().is_some() => results.push(AssertionResult::fail(
                VERIFY_ASSERTION_NAMES[3],
                format!("halt_class is {:?}", v.as_str().unwrap()),
            )),
            _ => results.push(AssertionResult::pass(VERIFY_ASSERTION_NAMES[3])),
        }

        // Assertion 5: no-halted-after-role
        match entry.get("halted_after_role") {
            Some(v) if v.is_null() => results.push(AssertionResult::pass(VERIFY_ASSERTION_NAMES[4])),
            Some(v) if v.as_str().is_some() => results.push(AssertionResult::fail(
                VERIFY_ASSERTION_NAMES[4],
                format!("halted_after_role is {:?}", v.as_str().unwrap()),
            )),
            _ => results.push(AssertionResult::pass(VERIFY_ASSERTION_NAMES[4])),
        }

        // Assertion 6: all-10-substeps-traced
        let traces = entry.get("traces").and_then(|v| v.as_array());
        match traces {
            None => results.push(AssertionResult::fail(
                VERIFY_ASSERTION_NAMES[5],
                "entry has no `traces` array (older entry written before cycle 169 schema extension)".to_string(),
            )),
            Some(arr) if arr.len() != 10 => results.push(AssertionResult::fail(
                VERIFY_ASSERTION_NAMES[5],
                format!("traces.len()={} expected 10", arr.len()),
            )),
            Some(arr) => {
                let all_executed = arr.iter().all(|t| {
                    t.get("executed").and_then(|v| v.as_bool()).unwrap_or(false)
                });
                if all_executed {
                    results.push(AssertionResult::pass(VERIFY_ASSERTION_NAMES[5]));
                } else {
                    let unexecuted: Vec<String> = arr
                        .iter()
                        .filter(|t| !t.get("executed").and_then(|v| v.as_bool()).unwrap_or(false))
                        .filter_map(|t| t.get("name").and_then(|v| v.as_str()).map(String::from))
                        .collect();
                    results.push(AssertionResult::fail(
                        VERIFY_ASSERTION_NAMES[5],
                        format!("traces with executed=false: {}", unexecuted.join(",")),
                    ));
                }
            }
        }

        // Assertion 7: state-audit-not-hard
        let audit_severity = entry
            .get("state_audit")
            .and_then(|sa| if sa.is_null() { None } else { Some(sa) })
            .and_then(|sa| sa.get("severity"))
            .and_then(|v| v.as_str());
        match audit_severity {
            None => {
                // Audit absent (dry-run cycle, or pre-cycle-162 entry, or
                // pre-cycle-169 schema). Per design §2.2 assertion 7
                // ("severity != Hard"), absence is not `Hard` and so passes.
                results.push(AssertionResult::pass(VERIFY_ASSERTION_NAMES[6]));
            }
            Some("hard") => results.push(AssertionResult::fail(
                VERIFY_ASSERTION_NAMES[6],
                "state_audit severity is 'hard'".to_string(),
            )),
            Some(_) => results.push(AssertionResult::pass(VERIFY_ASSERTION_NAMES[6])),
        }
    }

    // Assertion 8: super-step-history-has-cycle-N
    let ssh_path = repo_root.join("state").join("super-step-history.json");
    results.push(check_history_has_cycle(
        &ssh_path,
        cycle,
        "cycles",
        VERIFY_ASSERTION_NAMES[7],
    ));

    // Assertion 9: per-role-history-has-cycle-N (4 files, all must contain)
    let mut role_failures: Vec<String> = Vec::new();
    for role in ["reconciler", "planner", "executor", "curator"] {
        let path = repo_root
            .join("state")
            .join("roles")
            .join(format!("{role}-history.json"));
        let check = check_history_has_cycle(&path, cycle, "runs", VERIFY_ASSERTION_NAMES[8]);
        if !check.passed {
            role_failures.push(role.to_string());
        }
    }
    if role_failures.is_empty() {
        results.push(AssertionResult::pass(VERIFY_ASSERTION_NAMES[8]));
    } else {
        results.push(AssertionResult::fail(
            VERIFY_ASSERTION_NAMES[8],
            format!("no cycle-{cycle} entry in role history files for: {}", role_failures.join(",")),
        ));
    }

    results
}

/// Read `path` as JSON, look for the array under `array_key`, and check
/// any entry has `cycle == n`. Missing file / parse error / missing key
/// all map to fail (with the appropriate detail).
fn check_history_has_cycle(
    path: &Path,
    n: u32,
    array_key: &str,
    assertion_name: &'static str,
) -> AssertionResult {
    if !path.exists() {
        return AssertionResult::fail(
            assertion_name,
            format!("state file {} does not exist", path.display()),
        );
    }
    let raw = match fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => return AssertionResult::fail(
            assertion_name,
            format!("failed to read {}: {e}", path.display()),
        ),
    };
    let parsed: serde_json::Value = match serde_json::from_str(&raw) {
        Ok(v) => v,
        Err(e) => return AssertionResult::fail(
            assertion_name,
            format!("failed to parse {}: {e}", path.display()),
        ),
    };
    let arr = match parsed.get(array_key).and_then(|v| v.as_array()) {
        Some(a) => a,
        None => return AssertionResult::fail(
            assertion_name,
            format!("{} is missing `{array_key}` array", path.display()),
        ),
    };
    let found = arr.iter().any(|entry| {
        entry.get("cycle").and_then(|v| v.as_u64()) == Some(u64::from(n))
    });
    if found {
        AssertionResult::pass(assertion_name)
    } else {
        AssertionResult::fail(
            assertion_name,
            format!("no entry with cycle == {n} in {}", path.display()),
        )
    }
}

fn read_cycle_entry(
    history_path: &Path,
    cycle: u32,
) -> Result<Option<serde_json::Value>, RunnerError> {
    if !history_path.exists() {
        return Ok(None);
    }
    let raw = fs::read_to_string(history_path)?;
    let history: serde_json::Value = serde_json::from_str(&raw)?;
    let cycles = match history.get("cycles").and_then(|v| v.as_array()) {
        Some(a) => a,
        None => return Ok(None),
    };
    for entry in cycles {
        if entry.get("cycle").and_then(|v| v.as_u64()) == Some(u64::from(cycle)) {
            return Ok(Some(entry.clone()));
        }
    }
    Ok(None)
}

fn emit_verify<W: Write>(
    cycle: u32,
    verdict: &str,
    assertions: &[AssertionResult],
    format: Format,
    out: &mut W,
) -> Result<(), RunnerError> {
    let passed_count = assertions.iter().filter(|a| a.passed).count();
    let total = assertions.len();
    match format {
        Format::Text => {
            writeln!(
                out,
                "v2-cycle-runner verify --cycle {cycle}: verdict={verdict} ({passed_count}/{total} assertions passed)"
            )?;
            for a in assertions {
                let mark = if a.passed { "ok" } else { "FAIL" };
                if let Some(d) = &a.details {
                    writeln!(out, "  [{mark}] {} — {d}", a.name)?;
                } else {
                    writeln!(out, "  [{mark}] {}", a.name)?;
                }
            }
        }
        Format::Json => {
            let exit_code = if verdict == "clean" { 0 } else { 1 };
            let payload = serde_json::json!({
                "schema_version": "v1",
                "subcommand": "verify",
                "cycle": cycle,
                "verdict": verdict,
                "assertions": assertions,
                "exit_code": exit_code,
            });
            writeln!(out, "{}", serde_json::to_string_pretty(&payload)?)?;
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
        let sa = bin_dir.join("v2-state-audit");
        touch(&crb);
        touch(&ssb);
        touch(&rd);
        touch(&rep);
        touch(&sa);
        Args {
            repo_root: tmp.to_path_buf(),
            format: Format::Json,
            channel_router_bin: crb,
            super_step_boundary_bin: ssb,
            role_driver_bin: rd,
            reconciler_event_processor_bin: rep,
            state_audit_bin: sa,
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
        // All invocations succeed by default (canned queue is empty → ok_output()).
        // Invocations: 1 state-audit (cycle 162) + 10 super-step calls = 11.
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        assert_eq!(mock.invoke_count(), 11);
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
        // Invocation 0 = state-audit (ok); then step 1 ok, step 2 ok, step 3
        // (role-driver invoke reconciler) FAIL with role-session-empty stderr.
        mock.queue(ok_output()); // state-audit
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
        // Invocation 0 = state-audit (ok); then step 1 fails with
        // super-step-out-of-order.
        mock.queue(ok_output()); // state-audit
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
        // Invocation 0 = state-audit (ok). Then step 1: transient fail, retry succeeds.
        // Steps 2-10 succeed by default. Total invocations: 1 audit + 1 fail + 1 retry + 9 = 12.
        mock.queue(ok_output()); // state-audit
        mock.queue(fail_output(1, "connection reset"));
        mock.queue(ok_output()); // retry
        // steps 2-10 default-ok.
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        assert_eq!(mock.invoke_count(), 12);
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
        // 1 state-audit + 5 super-step calls (cycle-start, poll, reconciler-session,
        // advance-1, planner-session) = 6.
        assert_eq!(mock.invoke_count(), 6);
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
        // Invocation 0 = state-audit (ok). Then step 1 ok, step 2 ok, step 3
        // (reconciler-session) fails with role-session-empty.
        mock.queue(ok_output()); // state-audit
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
    fn phase_for_each_step_kind_variant() {
        // C2 (L1.2) cycle 152 critique absorbed cycle 159: phase categorizes
        // every StepKind variant into Boundary/Reconciler/Planner/Executor/
        // Curator/Transition. Pin the derivation so phase semantics don't
        // drift with future StepKind changes.
        assert_eq!(phase_for(StepKind::SuperStepCycleStart), Phase::Boundary);
        assert_eq!(phase_for(StepKind::SuperStepCycleEnd), Phase::Boundary);
        assert_eq!(phase_for(StepKind::ReconcilerPoll), Phase::Reconciler);
        assert_eq!(phase_for(StepKind::RoleInvoke(Role::Reconciler)), Phase::Reconciler);
        assert_eq!(phase_for(StepKind::RoleInvoke(Role::Planner)), Phase::Planner);
        assert_eq!(phase_for(StepKind::RoleInvoke(Role::Executor)), Phase::Executor);
        assert_eq!(phase_for(StepKind::RoleInvoke(Role::Curator)), Phase::Curator);
        assert_eq!(phase_for(StepKind::SuperStepAdvance), Phase::Transition);
    }

    #[test]
    fn super_step_sequence_phase_distribution_matches_design() {
        // Pin the per-step phase mapping under the canonical 10-step
        // super-step sequence. The distribution should be:
        //   step 1  (super-step-init)       -> Boundary
        //   step 2  (reconciler-pre-poll)   -> Reconciler
        //   step 3  (reconciler-session)    -> Reconciler
        //   step 4  (super-step-advance-1)  -> Transition
        //   step 5  (planner-session)       -> Planner
        //   step 6  (super-step-advance-2)  -> Transition
        //   step 7  (executor-session)      -> Executor
        //   step 8  (super-step-advance-3)  -> Transition
        //   step 9  (curator-session)       -> Curator
        //   step 10 (super-step-settle)     -> Boundary
        // i.e. 2 Boundary + 2 Reconciler + 1 Planner + 1 Executor +
        // 1 Curator + 3 Transition.
        let seq = super_step_sequence();
        let phases: Vec<Phase> = seq.iter().map(|s| phase_for(s.kind)).collect();
        let expected = [
            Phase::Boundary,
            Phase::Reconciler,
            Phase::Reconciler,
            Phase::Transition,
            Phase::Planner,
            Phase::Transition,
            Phase::Executor,
            Phase::Transition,
            Phase::Curator,
            Phase::Boundary,
        ];
        assert_eq!(phases, expected);
        let count = |p: Phase| phases.iter().filter(|&&q| q == p).count();
        assert_eq!(count(Phase::Boundary), 2);
        assert_eq!(count(Phase::Reconciler), 2);
        assert_eq!(count(Phase::Planner), 1);
        assert_eq!(count(Phase::Executor), 1);
        assert_eq!(count(Phase::Curator), 1);
        assert_eq!(count(Phase::Transition), 3);
    }

    #[test]
    fn dry_run_traces_carry_phase_in_json() {
        // Phase field must be present on every dry-run trace and use
        // kebab-case serialization.
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let mut ra = run_args_with_outputs(tmp.path(), 13);
        ra.dry_run = true;
        let mock = MockInvoker::new();
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        // Dry-run path also writes "step N name: ..." lines to stdout, so
        // strip everything before the first `{` to isolate the JSON report
        // (same pattern as dry_run_traces_have_executed_false).
        let s = String::from_utf8_lossy(&out).into_owned();
        let json_start = s.find('{').expect("expected JSON report in stdout");
        let parsed: serde_json::Value = serde_json::from_str(&s[json_start..]).unwrap();
        let traces = parsed["traces"].as_array().unwrap();
        assert_eq!(traces.len(), 10);
        // Pin the kebab-case strings the consumers will read.
        let expected_phases = [
            "boundary",
            "reconciler",
            "reconciler",
            "transition",
            "planner",
            "transition",
            "executor",
            "transition",
            "curator",
            "boundary",
        ];
        for (i, t) in traces.iter().enumerate() {
            let p = t["phase"].as_str().unwrap_or_else(|| panic!(
                "trace {i} missing phase field: {t:?}"
            ));
            assert_eq!(p, expected_phases[i], "trace {i} phase mismatch");
        }
    }

    #[test]
    fn live_traces_carry_phase_in_json() {
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 14);
        let mock = MockInvoker::new();
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        let report: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&out)).unwrap();
        let traces = report["traces"].as_array().unwrap();
        assert_eq!(traces.len(), 10);
        // Same phase order as dry-run — phase derives from StepKind, not
        // execution mode.
        let expected = [
            "boundary", "reconciler", "reconciler", "transition",
            "planner", "transition", "executor", "transition",
            "curator", "boundary",
        ];
        for (i, t) in traces.iter().enumerate() {
            assert_eq!(t["phase"].as_str(), Some(expected[i]), "live trace {i} phase");
        }
    }

    #[test]
    fn phase_uses_kebab_case_serialization() {
        // Single-shot serialization smoke covering every variant — guards
        // against accidental rename_all drift in the derive macro.
        let pairs: &[(Phase, &str)] = &[
            (Phase::Boundary, "boundary"),
            (Phase::Reconciler, "reconciler"),
            (Phase::Planner, "planner"),
            (Phase::Executor, "executor"),
            (Phase::Curator, "curator"),
            (Phase::Transition, "transition"),
        ];
        for (p, expected) in pairs {
            let s = serde_json::to_string(p).unwrap();
            assert_eq!(s, format!("\"{expected}\""), "phase {p:?} serialization");
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
        // Cycle 162: state-audit is the FIRST invocation (pre-flight), before
        // the 10-step super-step sequence. Total: 11 calls.
        assert_eq!(calls.len(), 11);
        assert!(calls[0].0.file_name().unwrap() == "v2-state-audit");
        assert!(calls[0].1.contains(&"--json".to_string()));
        // Then the 10-step super-step sequence begins.
        assert!(calls[1].0.file_name().unwrap() == "v2-super-step-boundary");
        assert!(calls[1].1.contains(&"cycle-start".to_string()));
        assert!(calls[2].0.file_name().unwrap() == "v2-reconciler-event-processor");
        assert!(calls[2].1.contains(&"poll".to_string()));
        // Last call: super-step-boundary cycle-end.
        assert!(calls[10].0.file_name().unwrap() == "v2-super-step-boundary");
        assert!(calls[10].1.contains(&"cycle-end".to_string()));
    }

    // ---- cycle 162 state-audit-on-start wiring ----

    #[test]
    fn state_audit_severity_from_exit_code_covers_all_audit_codes() {
        assert_eq!(StateAuditSeverity::from_exit_code(0), StateAuditSeverity::Ok);
        assert_eq!(StateAuditSeverity::from_exit_code(1), StateAuditSeverity::Advisory);
        assert_eq!(StateAuditSeverity::from_exit_code(2), StateAuditSeverity::Mandatory);
        assert_eq!(StateAuditSeverity::from_exit_code(3), StateAuditSeverity::Hard);
        assert_eq!(StateAuditSeverity::from_exit_code(4), StateAuditSeverity::SerializationFailure);
        assert_eq!(StateAuditSeverity::from_exit_code(-1), StateAuditSeverity::Unknown);
        assert_eq!(StateAuditSeverity::from_exit_code(99), StateAuditSeverity::Unknown);
    }

    #[test]
    fn state_audit_severity_only_hard_requires_halt() {
        assert!(!StateAuditSeverity::Ok.requires_halt());
        assert!(!StateAuditSeverity::Advisory.requires_halt());
        assert!(!StateAuditSeverity::Mandatory.requires_halt());
        assert!(StateAuditSeverity::Hard.requires_halt());
        // Fail-open: tool-internal anomalies are reported but do not halt.
        assert!(!StateAuditSeverity::SerializationFailure.requires_halt());
        assert!(!StateAuditSeverity::Unknown.requires_halt());
    }

    #[test]
    fn state_audit_severity_kebab_serialization() {
        let pairs: &[(StateAuditSeverity, &str)] = &[
            (StateAuditSeverity::Ok, "ok"),
            (StateAuditSeverity::Advisory, "advisory"),
            (StateAuditSeverity::Mandatory, "mandatory"),
            (StateAuditSeverity::Hard, "hard"),
            (StateAuditSeverity::SerializationFailure, "serialization-failure"),
            (StateAuditSeverity::Unknown, "unknown"),
        ];
        for (s, expected) in pairs {
            assert_eq!(s.as_kebab(), *expected);
            let serialized = serde_json::to_string(s).unwrap();
            assert_eq!(serialized, format!("\"{expected}\""));
        }
    }

    #[test]
    fn failure_class_state_bound_exceeded_kebab() {
        assert_eq!(FailureClass::StateBoundExceeded.as_kebab(), "state-bound-exceeded");
    }

    #[test]
    fn state_audit_hard_severity_halts_cycle_before_super_step_sequence() {
        // The pre-flight audit invocation returns exit code 3 (Hard). The
        // cycle MUST halt before any super-step primitive runs — total
        // invocation count must be 1 (audit only). last-cycle.json must
        // record halt_reason=state-bound-exceeded and halt_step=
        // state-audit-on-start.
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 100);
        let mock = MockInvoker::new();
        mock.queue(fail_output(3, "")); // state-audit Hard
        let mut out = Vec::new();
        let r = run_cycle(&args, &ra, &mut out, &mock);
        match r {
            Err(RunnerError::CycleHalted { step, class, .. }) => {
                assert_eq!(step, "state-audit-on-start");
                assert_eq!(class, FailureClass::StateBoundExceeded);
            }
            other => panic!("expected halt at state-audit-on-start, got {other:?}"),
        }
        assert_eq!(
            mock.invoke_count(),
            1,
            "no super-step primitive should be invoked when audit halts at session-start",
        );
        let last_cycle_path = tmp.path().join("state/v2-cycle-runner/last-cycle.json");
        let parsed: serde_json::Value =
            serde_json::from_str(&fs::read_to_string(&last_cycle_path).unwrap()).unwrap();
        assert_eq!(parsed["status"], "halted");
        assert_eq!(parsed["halt_step"], "state-audit-on-start");
        assert_eq!(parsed["halt_reason"], "state-bound-exceeded");
        assert_eq!(parsed["steps_attempted"], 0);
    }

    #[test]
    fn state_audit_advisory_severity_does_not_halt_cycle() {
        // Exit code 1 = Advisory. Cycle MUST proceed.
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 101);
        let mock = MockInvoker::new();
        mock.queue(fail_output(1, "")); // state-audit Advisory
        // 10 super-step invocations default-ok.
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        assert_eq!(mock.invoke_count(), 11);
    }

    #[test]
    fn state_audit_mandatory_severity_does_not_halt_cycle() {
        // Exit code 2 = Mandatory. Cycle MUST proceed (mandatory triggers
        // archival recommendation in audit, not session halt).
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 102);
        let mock = MockInvoker::new();
        mock.queue(fail_output(2, "")); // state-audit Mandatory
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        assert_eq!(mock.invoke_count(), 11);
    }

    #[test]
    fn state_audit_serialization_failure_does_not_halt_cycle() {
        // Exit code 4 = SerializationFailure (audit's own JSON-emit broke).
        // Fail-open: do NOT halt — reported in cycle report, cycle proceeds.
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 103);
        let mock = MockInvoker::new();
        mock.queue(fail_output(4, "internal: serialize failed"));
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        assert_eq!(mock.invoke_count(), 11);
    }

    #[test]
    fn state_audit_outcome_serialized_on_cycle_report() {
        // On non-halt path: cycle report's state_audit field must contain
        // the audit outcome with severity, exit_code, and bin_path.
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 104);
        let mock = MockInvoker::new();
        // Queue: audit ok + 10 step ok = 11 invocations.
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        let report: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&out)).unwrap();
        let sa = report.get("state_audit").expect("state_audit field present");
        assert!(!sa.is_null(), "state_audit must be Some on live run");
        assert_eq!(sa["severity"], "ok");
        assert_eq!(sa["exit_code"], 0);
        assert!(sa["bin_path"].as_str().unwrap().ends_with("v2-state-audit"));
        assert!(sa["executed"].as_bool().unwrap());
    }

    #[test]
    fn state_audit_outcome_on_halt_at_audit_serialized_on_cycle_report() {
        // On halt-at-audit path: cycle report's state_audit field must
        // ALSO contain the audit outcome (severity=hard, exit_code=3).
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let ra = run_args_with_outputs(tmp.path(), 105);
        let mock = MockInvoker::new();
        mock.queue(fail_output(3, ""));
        let mut out = Vec::new();
        let _ = run_cycle(&args, &ra, &mut out, &mock);
        let report: serde_json::Value =
            serde_json::from_str(&String::from_utf8_lossy(&out)).unwrap();
        let sa = report.get("state_audit").expect("state_audit field present");
        assert_eq!(sa["severity"], "hard");
        assert_eq!(sa["exit_code"], 3);
        assert_eq!(report["halt_step"], "state-audit-on-start");
        assert_eq!(report["halt_class"], "state-bound-exceeded");
    }

    #[test]
    fn state_audit_skipped_in_dry_run() {
        // Dry-run skips state-audit entirely; state_audit field in report is null.
        let tmp = tempfile::tempdir().unwrap();
        let args = synthetic_args(tmp.path());
        let mut ra = run_args_with_outputs(tmp.path(), 106);
        ra.dry_run = true;
        let mock = MockInvoker::new();
        let mut out = Vec::new();
        run_cycle(&args, &ra, &mut out, &mock).unwrap();
        assert_eq!(mock.invoke_count(), 0, "dry-run must not invoke any primitive");
        let s = String::from_utf8_lossy(&out).into_owned();
        let json_start = s.find('{').expect("expected JSON report in stdout");
        let report: serde_json::Value = serde_json::from_str(&s[json_start..]).unwrap();
        assert!(
            report["state_audit"].is_null(),
            "dry-run state_audit must be null, got {:?}",
            report["state_audit"]
        );
    }

    #[test]
    fn state_audit_missing_bin_errors_before_any_step() {
        // If v2-state-audit binary is missing, the runner errors with
        // PrimitiveMissing before invoking any super-step primitive.
        let tmp = tempfile::tempdir().unwrap();
        let mut args = synthetic_args(tmp.path());
        args.state_audit_bin = tmp.path().join("does-not-exist/v2-state-audit");
        let ra = run_args_with_outputs(tmp.path(), 107);
        let mock = MockInvoker::new();
        let mut out = Vec::new();
        let r = run_cycle(&args, &ra, &mut out, &mock);
        match r {
            Err(RunnerError::PrimitiveMissing { name, .. }) => {
                assert_eq!(name, "v2-state-audit");
            }
            other => panic!("expected PrimitiveMissing(v2-state-audit), got {other:?}"),
        }
        assert_eq!(mock.invoke_count(), 0, "no invocation when audit bin missing");
    }

    // ========================================================
    // status + verify subcommand tests — cycle 169
    // ========================================================

    /// Build a synthetic completed-cycle entry per the cycle 169 state
    /// schema extension: includes `traces` (10 steps, all executed=true)
    /// and `state_audit` (severity=ok). Per-field shape matches what
    /// `write_runner_state` produces.
    fn completed_entry(cycle: u32) -> serde_json::Value {
        let traces: Vec<serde_json::Value> = (1..=10u8)
            .map(|i| serde_json::json!({
                "index": i,
                "name": format!("step-{i}"),
                "executed": true,
                "phase": "boundary",
            }))
            .collect();
        serde_json::json!({
            "cycle": cycle,
            "issue": 99000 + cycle,
            "status": "completed",
            "started_at": "2026-05-17T20:00:00Z",
            "ended_at":   "2026-05-17T20:01:00Z",
            "halt_reason": serde_json::Value::Null,
            "halt_step":   serde_json::Value::Null,
            "halted_after_role": serde_json::Value::Null,
            "steps_attempted": 10,
            "traces": traces,
            "state_audit": {"severity": "ok", "exit_code": 0},
        })
    }

    fn halted_entry(cycle: u32) -> serde_json::Value {
        let mut e = completed_entry(cycle);
        e["status"] = "halted".into();
        e["halt_reason"] = "role-session-empty".into();
        e["halt_step"] = "executor-session".into();
        e["steps_attempted"] = 7.into();
        // Halted cycles have fewer trace entries; reflect that.
        let traces: Vec<serde_json::Value> = (1..=7u8)
            .map(|i| serde_json::json!({
                "index": i,
                "name": format!("step-{i}"),
                "executed": true,
                "phase": "boundary",
            }))
            .collect();
        e["traces"] = serde_json::Value::Array(traces);
        e
    }

    fn write_runner_state_fixture(
        tmp: &Path,
        last_cycle: Option<&serde_json::Value>,
        history_cycles: &[serde_json::Value],
    ) {
        let dir = tmp.join("state").join("v2-cycle-runner");
        fs::create_dir_all(&dir).unwrap();
        if let Some(v) = last_cycle {
            fs::write(
                dir.join("last-cycle.json"),
                format!("{}\n", serde_json::to_string_pretty(v).unwrap()),
            ).unwrap();
        }
        let history = serde_json::json!({ "cycles": history_cycles });
        fs::write(
            dir.join("cycle-history.json"),
            format!("{}\n", serde_json::to_string_pretty(&history).unwrap()),
        ).unwrap();
    }

    fn write_super_step_history_fixture(tmp: &Path, cycles: &[u32]) {
        let entries: Vec<serde_json::Value> = cycles
            .iter()
            .map(|&c| serde_json::json!({
                "cycle": c,
                "started_at": "2026-05-17T20:00:00Z",
                "ended_at":   "2026-05-17T20:01:00Z",
                "transitions": [],
            }))
            .collect();
        let payload = serde_json::json!({ "cycles": entries });
        let dir = tmp.join("state");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join("super-step-history.json"),
            format!("{}\n", serde_json::to_string_pretty(&payload).unwrap()),
        ).unwrap();
    }

    fn write_role_history_fixture(tmp: &Path, role: &str, cycles: &[u32]) {
        let runs: Vec<serde_json::Value> = cycles
            .iter()
            .map(|&c| serde_json::json!({
                "cycle": c,
                "role": role,
                "at": "2026-05-17T20:00:00Z",
                "outcome": "success",
                "notes": "",
            }))
            .collect();
        let payload = serde_json::json!({ "role": role, "runs": runs });
        let dir = tmp.join("state").join("roles");
        fs::create_dir_all(&dir).unwrap();
        fs::write(
            dir.join(format!("{role}-history.json")),
            format!("{}\n", serde_json::to_string_pretty(&payload).unwrap()),
        ).unwrap();
    }

    fn write_all_role_histories(tmp: &Path, cycles: &[u32]) {
        for role in ["reconciler", "planner", "executor", "curator"] {
            write_role_history_fixture(tmp, role, cycles);
        }
    }

    /// Synthetic Args usable for status/verify tests. Doesn't need binaries
    /// since status+verify never shell to primitives.
    fn synthetic_args_for_readonly(tmp: &Path) -> Args {
        let mut a = synthetic_args(tmp);
        a.command = Subcmd::Schema;
        a
    }

    // -------- status tests --------

    #[test]
    fn status_default_reads_last_cycle_completed() {
        let tmp = tempfile::tempdir().unwrap();
        let entry = completed_entry(168);
        write_runner_state_fixture(tmp.path(), Some(&entry), std::slice::from_ref(&entry));
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        run_status(&args, None, false, &mut out).unwrap();
        let text = String::from_utf8(out).unwrap();
        // Default Format on synthetic_args is Json (per synthetic_args
        // setting format=Json), so we parse JSON.
        let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(parsed["cycle"], 168);
        assert_eq!(parsed["status"], "completed");
        assert_eq!(parsed["traces_count"], 10);
        assert_eq!(parsed["state_audit"], "ok");
        assert_eq!(parsed["include_primitives"], false);
        assert!(parsed.get("primitives").is_none());
    }

    #[test]
    fn status_default_errors_on_missing_state_file() {
        let tmp = tempfile::tempdir().unwrap();
        // No fixture writes — runner_dir won't exist.
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        let r = run_status(&args, None, false, &mut out);
        match r {
            Err(RunnerError::StatusStateNotFound { detail }) => {
                assert!(detail.contains("last-cycle.json"), "got: {detail}");
            }
            other => panic!("expected StatusStateNotFound, got {other:?}"),
        }
    }

    #[test]
    fn status_cycle_n_reads_history() {
        let tmp = tempfile::tempdir().unwrap();
        let entries = vec![completed_entry(5), completed_entry(6), completed_entry(7)];
        write_runner_state_fixture(tmp.path(), Some(&entries[0]), &entries);
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        run_status(&args, Some(6), false, &mut out).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
        assert_eq!(parsed["cycle"], 6);
        assert_eq!(parsed["status"], "completed");
    }

    #[test]
    fn status_cycle_n_errors_on_unknown_cycle() {
        let tmp = tempfile::tempdir().unwrap();
        let entries = vec![completed_entry(1), completed_entry(2), completed_entry(3)];
        write_runner_state_fixture(tmp.path(), Some(&entries[0]), &entries);
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        let r = run_status(&args, Some(9), false, &mut out);
        match r {
            Err(RunnerError::StatusStateNotFound { detail }) => {
                assert!(detail.contains("cycle 9"), "got: {detail}");
            }
            other => panic!("expected StatusStateNotFound, got {other:?}"),
        }
    }

    #[test]
    fn status_halted_cycle_reports_halt_step() {
        let tmp = tempfile::tempdir().unwrap();
        let entry = halted_entry(50);
        write_runner_state_fixture(tmp.path(), Some(&entry), std::slice::from_ref(&entry));
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        run_status(&args, None, false, &mut out).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
        assert_eq!(parsed["status"], "halted");
        assert_eq!(parsed["halt_step"], "executor-session");
        assert_eq!(parsed["halt_class"], "role-session-empty");
    }

    #[test]
    fn status_include_primitives_flag_no_crash_on_missing_primitives() {
        let tmp = tempfile::tempdir().unwrap();
        let entry = completed_entry(8);
        write_runner_state_fixture(tmp.path(), Some(&entry), std::slice::from_ref(&entry));
        // Do NOT write primitive state files. The flag should degrade
        // to nulls rather than crashing.
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        run_status(&args, None, true, &mut out).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
        assert_eq!(parsed["include_primitives"], true);
        assert!(parsed["primitives"].is_object());
        assert!(parsed["primitives"]["super_step"].is_null());
        assert!(parsed["primitives"]["roles"]["reconciler"].is_null());
    }

    // -------- verify tests --------

    fn write_clean_cycle_fixture(tmp: &Path, cycle: u32) {
        let entry = completed_entry(cycle);
        write_runner_state_fixture(tmp, Some(&entry), std::slice::from_ref(&entry));
        write_super_step_history_fixture(tmp, &[cycle]);
        write_all_role_histories(tmp, &[cycle]);
    }

    #[test]
    fn verify_clean_cycle_passes_all_9_assertions() {
        let tmp = tempfile::tempdir().unwrap();
        write_clean_cycle_fixture(tmp.path(), 100);
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        run_verify(&args, 100, false, &mut out).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
        assert_eq!(parsed["verdict"], "clean");
        let arr = parsed["assertions"].as_array().unwrap();
        assert_eq!(arr.len(), 9);
        for a in arr {
            assert_eq!(a["passed"], true, "assertion failed: {a:?}");
        }
    }

    #[test]
    fn verify_halted_cycle_fails_status_no_halt() {
        let tmp = tempfile::tempdir().unwrap();
        let entry = halted_entry(101);
        write_runner_state_fixture(tmp.path(), Some(&entry), std::slice::from_ref(&entry));
        write_super_step_history_fixture(tmp.path(), &[101]);
        write_all_role_histories(tmp.path(), &[101]);
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        run_verify(&args, 101, false, &mut out).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
        assert_eq!(parsed["verdict"], "dirty");
        let arr = parsed["assertions"].as_array().unwrap();
        // Assertion 2 (status-completed), 3 (no-halt-step), 4 (no-halt-class)
        // all should fail. Assertion 6 (all-10-substeps-traced) fails because
        // halted entry has 7 traces, not 10.
        assert_eq!(arr[1]["name"], "status-completed");
        assert_eq!(arr[1]["passed"], false);
        assert_eq!(arr[2]["name"], "no-halt-step");
        assert_eq!(arr[2]["passed"], false);
        assert_eq!(arr[3]["name"], "no-halt-class");
        assert_eq!(arr[3]["passed"], false);
    }

    #[test]
    fn verify_missing_substeps_fails_traces_assertion() {
        let tmp = tempfile::tempdir().unwrap();
        let mut entry = completed_entry(102);
        // Trim traces to 7 entries (not 10).
        let traces = entry["traces"].as_array().unwrap()[..7].to_vec();
        entry["traces"] = serde_json::Value::Array(traces);
        write_runner_state_fixture(tmp.path(), Some(&entry), std::slice::from_ref(&entry));
        write_super_step_history_fixture(tmp.path(), &[102]);
        write_all_role_histories(tmp.path(), &[102]);
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        run_verify(&args, 102, false, &mut out).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
        let arr = parsed["assertions"].as_array().unwrap();
        let traces_assertion = arr.iter().find(|a| a["name"] == "all-10-substeps-traced").unwrap();
        assert_eq!(traces_assertion["passed"], false);
        assert!(traces_assertion["details"].as_str().unwrap().contains("traces.len()=7"));
    }

    #[test]
    fn verify_unknown_cycle_errors_first_assertion() {
        let tmp = tempfile::tempdir().unwrap();
        // History is empty.
        write_runner_state_fixture(tmp.path(), None, &[]);
        write_super_step_history_fixture(tmp.path(), &[]);
        write_all_role_histories(tmp.path(), &[]);
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        run_verify(&args, 200, false, &mut out).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
        let arr = parsed["assertions"].as_array().unwrap();
        assert_eq!(arr[0]["name"], "cycle-N-in-history");
        assert_eq!(arr[0]["passed"], false);
        // Short-circuit: only assertion 1, plus 8 (super-step) + 9 (role) run.
        // Total 3 entries because assertions 2-7 require the entry.
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[1]["name"], "super-step-history-has-cycle-N");
        assert_eq!(arr[2]["name"], "per-role-history-has-cycle-N");
    }

    #[test]
    fn verify_strict_exits_1_on_any_failure() {
        let tmp = tempfile::tempdir().unwrap();
        let entry = halted_entry(103);
        write_runner_state_fixture(tmp.path(), Some(&entry), std::slice::from_ref(&entry));
        write_super_step_history_fixture(tmp.path(), &[103]);
        write_all_role_histories(tmp.path(), &[103]);
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        let r = run_verify(&args, 103, true, &mut out);
        match r {
            Err(RunnerError::VerifyDirtyStrict { failed_assertions, total_assertions }) => {
                assert!(failed_assertions > 0);
                assert_eq!(total_assertions, 9);
            }
            other => panic!("expected VerifyDirtyStrict, got {other:?}"),
        }
        // The verdict JSON should still have been written to stdout.
        let text = String::from_utf8(out).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
        assert_eq!(parsed["verdict"], "dirty");
    }

    #[test]
    fn verify_non_strict_exits_0_with_dirty_verdict() {
        let tmp = tempfile::tempdir().unwrap();
        let entry = halted_entry(104);
        write_runner_state_fixture(tmp.path(), Some(&entry), std::slice::from_ref(&entry));
        write_super_step_history_fixture(tmp.path(), &[104]);
        write_all_role_histories(tmp.path(), &[104]);
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        // No --strict: returns Ok even on dirty.
        run_verify(&args, 104, false, &mut out).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
        assert_eq!(parsed["verdict"], "dirty");
        // exit_code in JSON envelope reflects the verdict regardless of --strict
        assert_eq!(parsed["exit_code"], 1);
    }

    #[test]
    fn verify_missing_super_step_history_fails_assertion_8() {
        let tmp = tempfile::tempdir().unwrap();
        let entry = completed_entry(105);
        write_runner_state_fixture(tmp.path(), Some(&entry), std::slice::from_ref(&entry));
        // Deliberately omit super-step-history.json.
        write_all_role_histories(tmp.path(), &[105]);
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        run_verify(&args, 105, false, &mut out).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
        let arr = parsed["assertions"].as_array().unwrap();
        let a8 = arr.iter().find(|a| a["name"] == "super-step-history-has-cycle-N").unwrap();
        assert_eq!(a8["passed"], false);
        assert!(a8["details"].as_str().unwrap().contains("does not exist"));
    }

    #[test]
    fn verify_missing_role_history_fails_assertion_9() {
        let tmp = tempfile::tempdir().unwrap();
        let entry = completed_entry(106);
        write_runner_state_fixture(tmp.path(), Some(&entry), std::slice::from_ref(&entry));
        write_super_step_history_fixture(tmp.path(), &[106]);
        // Only write 3 of 4 role histories — curator missing.
        for role in ["reconciler", "planner", "executor"] {
            write_role_history_fixture(tmp.path(), role, &[106]);
        }
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        run_verify(&args, 106, false, &mut out).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
        let arr = parsed["assertions"].as_array().unwrap();
        let a9 = arr.iter().find(|a| a["name"] == "per-role-history-has-cycle-N").unwrap();
        assert_eq!(a9["passed"], false);
        assert!(a9["details"].as_str().unwrap().contains("curator"));
    }

    #[test]
    fn verify_state_audit_hard_severity_fails_assertion_7() {
        let tmp = tempfile::tempdir().unwrap();
        let mut entry = completed_entry(108);
        entry["state_audit"] = serde_json::json!({"severity": "hard", "exit_code": 3});
        write_runner_state_fixture(tmp.path(), Some(&entry), std::slice::from_ref(&entry));
        write_super_step_history_fixture(tmp.path(), &[108]);
        write_all_role_histories(tmp.path(), &[108]);
        let args = synthetic_args_for_readonly(tmp.path());
        let mut out = Vec::new();
        run_verify(&args, 108, false, &mut out).unwrap();
        let parsed: serde_json::Value =
            serde_json::from_str(&String::from_utf8(out).unwrap()).unwrap();
        let arr = parsed["assertions"].as_array().unwrap();
        let a7 = arr.iter().find(|a| a["name"] == "state-audit-not-hard").unwrap();
        assert_eq!(a7["passed"], false);
        assert!(a7["details"].as_str().unwrap().contains("hard"));
    }
}
