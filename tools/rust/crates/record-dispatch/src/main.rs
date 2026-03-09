use clap::Parser;
use record_dispatch::{
    apply_dispatch_patch, build_dispatch_patch, dispatch_commit_message, resolve_model,
};
use state_schema::{
    commit_state_json, current_cycle_from_state, current_utc_timestamp, read_state_value,
    write_state_value,
};
use std::path::PathBuf;

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
    #[arg(long)]
    model: Option<String>,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let model = resolve_model(cli.model.as_deref(), &cli.repo_root)?;
    let mut state_value = read_state_value(&cli.repo_root)?;
    let dispatched_at = current_utc_timestamp();
    let current_cycle = current_cycle_from_state(&cli.repo_root).map_err(|error| {
        if error == "missing /last_cycle/number in state.json" {
            "missing numeric /last_cycle/number in docs/state.json".to_string()
        } else {
            error
        }
    })?;

    let patch = build_dispatch_patch(
        &state_value,
        current_cycle,
        cli.issue,
        &cli.title,
        &model,
        &dispatched_at,
    )?;
    apply_dispatch_patch(&mut state_value, &patch)?;
    write_state_value(&cli.repo_root, &state_value)?;

    let commit_message = dispatch_commit_message(cli.issue, patch.current_cycle);
    let receipt = commit_state_json(&cli.repo_root, &commit_message)?;
    println!(
        "Dispatch recorded: #{} \"{}\" (model: {}). In-flight: {} (receipt: {})",
        cli.issue, cli.title, model, patch.in_flight, receipt
    );
    if patch.in_flight >= 3 {
        eprintln!(
            "Warning: in-flight dispatches at {} (approaching/exceeding concurrency limit of 2)",
            patch.in_flight
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

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
    fn concurrency_warning_threshold_is_triggered_at_three() {
        let patch = record_dispatch::build_dispatch_patch(
            &serde_json::json!({
                "agent_sessions": [],
                "copilot_metrics": {
                    "total_dispatches": 85,
                    "in_flight": 2,
                    "resolved": 83,
                    "dispatch_log_latest": "#601 old dispatch (cycle 164)"
                },
                "field_inventory": {
                    "fields": {
                        "copilot_metrics.in_flight": { "last_refreshed": "cycle 163" }
                    }
                }
            }),
            164,
            602,
            "Example dispatch",
            "gpt-5.4",
            "2026-03-07T13:00:00Z",
        )
        .expect("patch should build");
        assert!(patch.in_flight >= 3);
    }
}
