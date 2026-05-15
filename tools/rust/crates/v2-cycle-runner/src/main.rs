use clap::{Parser, Subcommand, ValueEnum};
use serde::Serialize;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, ExitCode};

#[derive(Parser, Debug)]
#[command(
    name = "v2-cycle-runner",
    about = "Conductor for the v2 multi-agent orchestrator: composes the 4 role-prompt sessions \
             and the 4 v2-* primitives (channel-router, super-step-boundary, role-driver, \
             reconciler-event-processor) into a single per-cycle execution. \
             SCAFFOLD scope cycle 150: only `init` is implemented. \
             COMPLETE arc cycles 151-153+: run / status / verify subcommands; first end-to-end \
             smoke at cycle ~153 is the first measurement opportunity for the v2 architecture."
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
    match args.command {
        Subcmd::Init => run_init(args, out),
        Subcmd::Schema => run_schema(args, out),
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
}
