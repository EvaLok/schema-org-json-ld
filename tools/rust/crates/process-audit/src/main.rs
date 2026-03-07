use clap::{Parser, ValueEnum};
use serde_json::{json, Value};
use state_schema::set_value_at_pointer;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

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
    let state_path = cli.repo_root.join("docs/state.json");
    let mut state_value = read_state_value(&state_path)?;
    let next_cycle = read_next_cycle(&state_value)?;

    if !apply_audit_processing(&mut state_value, cli.audit_id, next_cycle)? {
        println!("Audit #{} already processed (no changes made)", cli.audit_id);
        return Ok(());
    }

    write_state_value(&state_path, &state_value)?;

    let receipt = commit_state_json(&cli.repo_root, cli.audit_id, cli.action, next_cycle)?;
    println!(
        "Audit processed: audit#{} {} [cycle {}] (receipt: {})",
        cli.audit_id,
        cli.action.as_str(),
        next_cycle,
        receipt
    );

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

fn read_next_cycle(state: &Value) -> Result<u64, String> {
    let last_cycle = state
        .pointer("/last_cycle/number")
        .and_then(Value::as_u64)
        .ok_or_else(|| "missing numeric /last_cycle/number in docs/state.json".to_string())?;

    last_cycle
        .checked_add(1)
        .ok_or_else(|| "cycle overflow while computing current cycle".to_string())
}

fn apply_audit_processing(state: &mut Value, audit_id: u64, next_cycle: u64) -> Result<bool, String> {
    let audit_processed = state
        .pointer("/audit_processed")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing /audit_processed array in docs/state.json".to_string())?;

    if audit_processed.iter().any(|value| value.as_u64() == Some(audit_id)) {
        return Ok(false);
    }

    let mut next_audit_processed = audit_processed.clone();
    next_audit_processed.push(json!(audit_id));

    set_value_at_pointer(state, "/audit_processed", Value::Array(next_audit_processed))?;
    set_value_at_pointer(
        state,
        "/field_inventory/fields/audit_processed/last_refreshed",
        json!(format!("cycle {}", next_cycle)),
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

fn commit_state_json(
    repo_root: &Path,
    audit_id: u64,
    action: AuditAction,
    next_cycle: u64,
) -> Result<String, String> {
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
        "state(process-audit): audit#{} {} [cycle {}]",
        audit_id,
        action.as_str(),
        next_cycle
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
    fn next_cycle_is_derived_from_last_cycle_plus_one() {
        let state = sample_state();
        assert_eq!(read_next_cycle(&state).unwrap(), 166);
    }
}
