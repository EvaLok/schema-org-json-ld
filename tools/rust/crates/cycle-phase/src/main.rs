use clap::Parser;
use serde_json::Value;
use state_schema::{
    commit_state_json, current_cycle_from_state, read_state_value, transition_cycle_phase,
    write_state_value,
};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "cycle-phase")]
struct Cli {
    /// Target phase to transition to
    #[arg(long)]
    phase: String,

    /// Current cycle number
    #[arg(long)]
    cycle: u64,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Print the resulting cycle_phase without writing
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let state_cycle = current_cycle_from_state(&cli.repo_root)?;
    if cli.cycle != state_cycle {
        return Err(format!(
            "--cycle {} does not match docs/state.json current cycle {}",
            cli.cycle, state_cycle
        ));
    }

    let mut state = read_state_value(&cli.repo_root)?;
    transition_cycle_phase(&mut state, cli.cycle, &cli.phase)?;

    if cli.dry_run {
        let phase_value = state
            .pointer("/cycle_phase")
            .cloned()
            .unwrap_or(Value::Null);
        println!(
            "{}",
            serde_json::to_string_pretty(&phase_value)
                .map_err(|e| format!("failed to serialize: {}", e))?
        );
        return Ok(());
    }

    write_state_value(&cli.repo_root, &state)?;
    let commit_message = format!("state(cycle-phase): {} [cycle {}]", cli.phase, cli.cycle);
    commit_state_json(&cli.repo_root, &commit_message)?;

    println!("Transitioned cycle {} to phase '{}'", cli.cycle, cli.phase);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use serde_json::json;
    use state_schema::VALID_PHASES;

    fn sample_state() -> Value {
        json!({
            "last_cycle": { "number": 218 },
            "cycle_phase": {
                "cycle": 218,
                "phase": "work",
                "doc_issue": 980,
                "phase_entered_at": "2026-03-10T00:00:00Z"
            },
            "field_inventory": {
                "fields": {
                    "cycle_phase": {
                        "cadence": "every phase transition",
                        "last_refreshed": "cycle 218"
                    }
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
        assert!(help.contains("--phase"));
        assert!(help.contains("--cycle"));
        assert!(help.contains("--repo-root"));
        assert!(!help.contains("--doc-pr"));
        assert!(!help.contains("--review-iteration"));
        assert!(help.contains("--dry-run"));
    }

    #[test]
    fn transition_to_work_sets_phase_and_bumps_freshness() {
        let mut state = sample_state();
        transition_cycle_phase(&mut state, 218, "work").expect("transition should succeed");

        assert_eq!(state["cycle_phase"]["phase"], json!("work"));
        assert_eq!(state["cycle_phase"]["cycle"], json!(218));
        assert!(state["cycle_phase"]["phase_entered_at"].is_string());
        assert_eq!(
            state
                .pointer("/field_inventory/fields/cycle_phase/last_refreshed")
                .and_then(Value::as_str),
            Some("cycle 218")
        );
    }

    #[test]
    fn transition_to_close_out_succeeds() {
        let mut state = sample_state();
        transition_cycle_phase(&mut state, 218, "close_out").expect("transition should succeed");
        assert_eq!(state["cycle_phase"]["phase"], json!("close_out"));
    }

    #[test]
    fn transition_to_complete_succeeds() {
        let mut state = sample_state();
        transition_cycle_phase(&mut state, 218, "complete").expect("transition should succeed");
        assert_eq!(state["cycle_phase"]["phase"], json!("complete"));
    }

    #[test]
    fn invalid_phase_is_rejected() {
        let mut state = sample_state();
        let result = transition_cycle_phase(&mut state, 218, "bogus");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid cycle phase"));
    }

    #[test]
    fn valid_phases_constant_is_complete() {
        assert!(VALID_PHASES.contains(&"work"));
        assert!(VALID_PHASES.contains(&"close_out"));
        assert!(VALID_PHASES.contains(&"complete"));
        assert_eq!(VALID_PHASES.len(), 3);
    }
}
