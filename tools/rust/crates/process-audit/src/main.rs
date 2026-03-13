use clap::{Parser, ValueEnum};
use serde_json::{json, Value};
use state_schema::{
    commit_state_json, current_cycle_from_state, read_state_value, set_value_at_pointer,
    write_state_value,
};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "process-audit")]
struct Cli {
    /// Audit issue number that was processed
    #[arg(long)]
    audit_id: u64,

    /// Action taken on this audit recommendation
    #[arg(long, value_enum)]
    action: AuditAction,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum AuditAction {
    Accepted,
    Rejected,
    Deferred,
}

fn main() {
    let cli = Cli::parse();
    if let Err(error) = run(cli) {
        eprintln!("Error: {}", error);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let mut state_value = read_state_value(&cli.repo_root)?;
    let current_cycle = current_cycle(&cli.repo_root)?;

    if !apply_audit_processing(&mut state_value, cli.audit_id, current_cycle)? {
        println!(
            "Audit #{} already processed (no changes made)",
            cli.audit_id
        );
        return Ok(());
    }

    write_state_value(&cli.repo_root, &state_value)?;

    let commit_message = format!(
        "state(process-audit): audit#{} {} [cycle {}]",
        cli.audit_id,
        cli.action.as_str(),
        current_cycle
    );
    let receipt = commit_state_json(&cli.repo_root, &commit_message)?;
    println!(
        "Audit processed: audit#{} {} [cycle {}] (receipt: {})",
        cli.audit_id,
        cli.action.as_str(),
        current_cycle,
        receipt
    );

    Ok(())
}

fn current_cycle(repo_root: &Path) -> Result<u64, String> {
    current_cycle_from_state(repo_root).map_err(|error| {
        format!(
            "missing numeric /cycle_phase/cycle or /last_cycle/number in docs/state.json: {}",
            error
        )
    })
}

fn apply_audit_processing(
    state: &mut Value,
    audit_id: u64,
    current_cycle: u64,
) -> Result<bool, String> {
    let audit_processed = state
        .pointer("/audit_processed")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing /audit_processed array in docs/state.json".to_string())?;

    if audit_processed
        .iter()
        .any(|value| value.as_u64() == Some(audit_id))
    {
        return Ok(false);
    }

    let mut next_audit_processed = audit_processed.clone();
    next_audit_processed.push(json!(audit_id));

    set_value_at_pointer(
        state,
        "/audit_processed",
        Value::Array(next_audit_processed),
    )?;
    set_value_at_pointer(
        state,
        "/field_inventory/fields/audit_processed/last_refreshed",
        json!(format!("cycle {}", current_cycle)),
    )?;

    Ok(true)
}

impl AuditAction {
    fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Rejected => "rejected",
            Self::Deferred => "deferred",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::sync::atomic::{AtomicU64, Ordering};

    fn sample_state() -> Value {
        json!({
            "last_cycle": { "number": 165 },
            "audit_processed": [2, 3],
            "field_inventory": {
                "fields": {
                    "audit_processed": {
                        "last_refreshed": "cycle 163"
                    }
                }
            }
        })
    }

    struct TempRepo {
        path: PathBuf,
    }

    impl TempRepo {
        fn new(state: &Value) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = env::temp_dir().join(format!("process-audit-test-{}", run_id));
            fs::create_dir_all(path.join("docs")).expect("temp repo should be created");
            write_state_value(&path, state).expect("state should be written");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TempRepo {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn help_contains_expected_flags() {
        let mut command = Cli::command();
        let mut output = Vec::new();
        command.write_long_help(&mut output).unwrap();
        let help = String::from_utf8(output).unwrap();
        assert!(help.contains("--audit-id"));
        assert!(help.contains("--action"));
        assert!(help.contains("--repo-root"));
        assert!(help.contains("accepted"));
        assert!(help.contains("rejected"));
        assert!(help.contains("deferred"));
    }

    #[test]
    fn idempotency_does_not_duplicate_existing_audit_id() {
        let mut state = sample_state();
        let changed = apply_audit_processing(&mut state, 3, 166).expect("apply should succeed");
        assert!(!changed);

        let processed = state
            .pointer("/audit_processed")
            .and_then(Value::as_array)
            .expect("audit_processed should exist");
        assert_eq!(processed, &vec![json!(2), json!(3)]);
    }

    #[test]
    fn appends_new_audit_id() {
        let mut state = sample_state();
        let changed = apply_audit_processing(&mut state, 586, 166).expect("apply should succeed");
        assert!(changed);

        let processed = state
            .pointer("/audit_processed")
            .and_then(Value::as_array)
            .expect("audit_processed should exist");
        assert_eq!(processed, &vec![json!(2), json!(3), json!(586)]);
    }

    #[test]
    fn freshness_marker_uses_cycle_format() {
        let mut state = sample_state();
        let _ = apply_audit_processing(&mut state, 586, 166).expect("apply should succeed");
        let freshness = state
            .pointer("/field_inventory/fields/audit_processed/last_refreshed")
            .and_then(Value::as_str)
            .expect("freshness marker should exist");
        assert_eq!(freshness, "cycle 166");
    }

    #[test]
    fn current_cycle_matches_last_cycle_number_from_state_file() {
        let repo = TempRepo::new(&sample_state());
        assert_eq!(current_cycle(repo.path()).unwrap(), 165);
    }
}
