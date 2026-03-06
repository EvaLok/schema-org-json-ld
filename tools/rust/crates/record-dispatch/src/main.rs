use clap::Parser;
use serde_json::{json, Value};
use state_schema::set_value_at_pointer;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "record-dispatch")]
struct Cli {
    /// GitHub issue number that was dispatched
    #[arg(long)]
    issue: u64,

    /// Short title/description of the dispatch
    #[arg(long)]
    title: String,

    /// Model used for the dispatch
    #[arg(long, default_value = "gpt-5.3-codex")]
    model: String,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct DispatchPatch {
    total_dispatches: i64,
    in_flight: i64,
    dispatch_to_pr_rate: String,
    dispatch_log_latest: String,
    note: String,
    current_cycle: i64,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let state_path = cli.repo_root.join("docs/state.json");
    let mut state_value = read_state_value(&state_path)?;

    let patch = build_dispatch_patch(&state_value, cli.issue, &cli.title, &cli.model)?;
    apply_dispatch_patch(&mut state_value, &patch)?;
    write_state_value(&state_path, &state_value)?;

    let receipt = commit_state_json(&cli.repo_root, cli.issue, patch.current_cycle)?;
    println!(
        "Dispatch recorded: #{} \"{}\" (model: {}). In-flight: {} (receipt: {})",
        cli.issue, cli.title, cli.model, patch.in_flight, receipt
    );
    if patch.in_flight >= 3 {
        eprintln!(
            "Warning: in-flight dispatches at {} (approaching/exceeding concurrency limit of 2)",
            patch.in_flight
        );
    }

    Ok(())
}

fn read_state_value(path: &Path) -> Result<Value, String> {
    let content = fs::read_to_string(path)
        .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
    serde_json::from_str::<Value>(&content)
        .map_err(|error| format!("failed to parse {}: {}", path.display(), error))
}

fn write_state_value(path: &Path, value: &Value) -> Result<(), String> {
    let serialized = serde_json::to_string_pretty(value)
        .map_err(|error| format!("failed to serialize state.json: {}", error))?;
    fs::write(path, format!("{}\n", serialized))
        .map_err(|error| format!("failed to write {}: {}", path.display(), error))
}

fn build_dispatch_patch(
    state: &Value,
    issue: u64,
    title: &str,
    model: &str,
) -> Result<DispatchPatch, String> {
    let current_cycle = read_required_i64(state, "/last_cycle/number")?;
    let total_dispatches = read_required_i64(state, "/copilot_metrics/total_dispatches")?;
    let in_flight = read_required_i64(state, "/copilot_metrics/in_flight")?;
    let produced_pr = read_required_i64(state, "/copilot_metrics/produced_pr")?;
    let resolved = read_required_i64(state, "/copilot_metrics/resolved")?;
    let merged = read_required_i64(state, "/copilot_metrics/merged")?;
    let closed_without_merge = read_required_i64(state, "/copilot_metrics/closed_without_merge")?;

    let next_total_dispatches = total_dispatches + 1;
    let next_in_flight = in_flight + 1;

    validate_dispatch_invariant(next_total_dispatches, resolved, next_in_flight)?;

    Ok(DispatchPatch {
        total_dispatches: next_total_dispatches,
        in_flight: next_in_flight,
        dispatch_to_pr_rate: format_dispatch_to_pr_rate(produced_pr, next_total_dispatches),
        dispatch_log_latest: format_dispatch_log(issue, title, current_cycle),
        note: format_dispatch_note(
            next_total_dispatches,
            resolved,
            next_in_flight,
            produced_pr,
            merged,
            closed_without_merge,
            issue,
            title,
            current_cycle,
            model,
        ),
        current_cycle,
    })
}

fn read_required_i64(state: &Value, pointer: &str) -> Result<i64, String> {
    state
        .pointer(pointer)
        .and_then(Value::as_i64)
        .ok_or_else(|| format!("missing numeric {} in docs/state.json", pointer))
}

fn validate_dispatch_invariant(
    total_dispatches: i64,
    resolved: i64,
    in_flight: i64,
) -> Result<(), String> {
    if resolved + in_flight != total_dispatches {
        return Err(format!(
            "invariant violated: resolved({}) + in_flight({}) != total_dispatches({})",
            resolved, in_flight, total_dispatches
        ));
    }

    Ok(())
}

fn format_dispatch_to_pr_rate(produced_pr: i64, total_dispatches: i64) -> String {
    format!("{}/{}", produced_pr, total_dispatches)
}

fn format_dispatch_log(issue: u64, title: &str, current_cycle: i64) -> String {
    format!("#{} {} (cycle {})", issue, title, current_cycle)
}

fn format_dispatch_note(
    total_dispatches: i64,
    resolved: i64,
    in_flight: i64,
    produced_pr: i64,
    merged: i64,
    closed_without_merge: i64,
    issue: u64,
    title: &str,
    current_cycle: i64,
    model: &str,
) -> String {
    format!(
		"{} dispatches, {} resolved, {} in-flight. {} produced PRs ({} merged, {} closed without merge). Latest dispatch: #{} {} (cycle {}, model {}).",
		total_dispatches,
		resolved,
		in_flight,
		produced_pr,
		merged,
		closed_without_merge,
		issue,
		title,
		current_cycle,
		model
	)
}

fn apply_dispatch_patch(state: &mut Value, patch: &DispatchPatch) -> Result<(), String> {
    let cycle_marker = format!("cycle {}", patch.current_cycle);
    set_value_at_pointer(
        state,
        "/copilot_metrics/total_dispatches",
        json!(patch.total_dispatches),
    )?;
    set_value_at_pointer(state, "/copilot_metrics/in_flight", json!(patch.in_flight))?;
    set_value_at_pointer(
        state,
        "/copilot_metrics/dispatch_to_pr_rate",
        json!(patch.dispatch_to_pr_rate),
    )?;
    set_value_at_pointer(
        state,
        "/copilot_metrics/dispatch_log_latest",
        json!(patch.dispatch_log_latest),
    )?;
    set_value_at_pointer(state, "/copilot_metrics/note", json!(patch.note))?;
    set_value_at_pointer(
        state,
        "/field_inventory/fields/copilot_metrics.in_flight/last_refreshed",
        json!(cycle_marker.clone()),
    )?;
    set_value_at_pointer(
        state,
        "/field_inventory/fields/copilot_metrics.dispatch_to_pr_rate/last_refreshed",
        json!(cycle_marker),
    )?;

    Ok(())
}

fn commit_state_json(repo_root: &Path, issue: u64, current_cycle: i64) -> Result<String, String> {
    let add_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("add")
        .arg("docs/state.json")
        .output()
        .map_err(|error| format!("failed to execute git add: {}", error))?;
    if !add_output.status.success() {
        let stderr = String::from_utf8_lossy(&add_output.stderr)
            .trim()
            .to_string();
        return Err(format!("git add docs/state.json failed: {}", stderr));
    }

    let commit_message = format!(
        "state(record-dispatch): #{} dispatched [cycle {}]",
        issue, current_cycle
    );
    let commit_output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("commit")
        .arg("-m")
        .arg(&commit_message)
        .output()
        .map_err(|error| format!("failed to execute git commit: {}", error))?;
    if !commit_output.status.success() {
        let stderr = String::from_utf8_lossy(&commit_output.stderr)
            .trim()
            .to_string();
        return Err(format!("git commit failed: {}", stderr));
    }

    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("rev-parse")
        .arg("--short=7")
        .arg("HEAD")
        .output()
        .map_err(|error| format!("failed to execute git rev-parse: {}", error))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("git rev-parse --short=7 HEAD failed: {}", stderr));
    }

    let sha = String::from_utf8(output.stdout)
        .map_err(|error| format!("failed to decode git rev-parse output as UTF-8: {}", error))?;
    Ok(sha.trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    fn sample_state() -> Value {
        json!({
            "last_cycle": { "number": 164 },
            "copilot_metrics": {
                "total_dispatches": 85,
                "in_flight": 2,
                "produced_pr": 81,
                "resolved": 83,
                "merged": 80,
                "closed_without_merge": 1,
                "dispatch_to_pr_rate": "81/85",
                "dispatch_log_latest": "#601 old dispatch (cycle 164)",
                "note": "old note"
            },
            "field_inventory": {
                "fields": {
                    "copilot_metrics.dispatch_to_pr_rate": { "last_refreshed": "cycle 164" },
                    "copilot_metrics.in_flight": { "last_refreshed": "cycle 164" }
                }
            }
        })
    }

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--issue"));
        assert!(help.contains("--title"));
        assert!(help.contains("--model"));
        assert!(help.contains("--repo-root"));
    }

    #[test]
    fn metric_calculation_after_dispatch_is_correct() {
        let patch = build_dispatch_patch(&sample_state(), 602, "Example dispatch", "gpt-5.3-codex")
            .expect("patch should build");
        assert_eq!(patch.total_dispatches, 86);
        assert_eq!(patch.in_flight, 3);
    }

    #[test]
    fn rate_string_formatting_is_correct() {
        assert_eq!(format_dispatch_to_pr_rate(81, 86), "81/86");
    }

    #[test]
    fn dispatch_log_formatting_is_correct() {
        assert_eq!(
            format_dispatch_log(602, "Example dispatch", 164),
            "#602 Example dispatch (cycle 164)"
        );
    }

    #[test]
    fn invariant_validation_fails_when_totals_do_not_match() {
        let error = validate_dispatch_invariant(86, 82, 3).expect_err("invariant should fail");
        assert!(error.contains("invariant violated"));
    }

    #[test]
    fn concurrency_warning_threshold_is_triggered_at_three() {
        let patch = build_dispatch_patch(&sample_state(), 602, "Example dispatch", "gpt-5.3-codex")
            .expect("patch should build");
        assert!(patch.in_flight >= 3);
    }
}
