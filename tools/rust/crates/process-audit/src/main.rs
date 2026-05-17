use clap::{Parser, ValueEnum};
use serde_json::{json, Value};
use state_schema::{
    commit_state_json, current_cycle_from_state, read_state_value, set_value_at_pointer,
    write_state_value, AdoptionArtifactReference,
};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(name = "process-audit")]
struct Cli {
    /// Audit issue number that was processed
    #[arg(long = "audit", alias = "audit-id")]
    audit_id: u64,

    /// Disposition taken on this audit recommendation
    #[arg(long, value_enum)]
    disposition: Option<AuditDispositionArg>,

    /// Domain/category for the new audit recommendation entry
    #[arg(long)]
    category: Option<String>,

    /// Optional recommendation summary to record alongside the disposition
    #[arg(long)]
    summary: Option<String>,

    /// Required when disposition is defer or reject
    #[arg(long)]
    justification: Option<String>,

    /// Required when disposition is accept; JSON payload describing the adopted artifact
    #[arg(long)]
    adoption_artifact_reference: Option<String>,

    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum AuditDispositionArg {
    Accept,
    Reject,
    Defer,
}

impl AuditDispositionArg {
    fn as_str(self) -> &'static str {
        match self {
            Self::Accept => "accept",
            Self::Reject => "reject",
            Self::Defer => "defer",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ValidatedCli {
    audit_id: u64,
    disposition: AuditDispositionArg,
    category: String,
    summary: String,
    justification: Option<String>,
    adoption_artifact_reference: Option<AdoptionArtifactReference>,
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
    let cli = validate_cli(cli)?;
    let mut state_value = read_state_value(&cli.repo_root)?;
    let current_cycle = current_cycle(&cli.repo_root)?;

    if !apply_audit_processing(&mut state_value, &cli, current_cycle)? {
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
        cli.disposition.as_str(),
        current_cycle
    );
    let receipt = commit_state_json(&cli.repo_root, &commit_message)?;
    println!(
        "Audit processed: audit#{} {} [cycle {}] (receipt: {})",
        cli.audit_id,
        cli.disposition.as_str(),
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
    cli: &ValidatedCli,
    current_cycle: u64,
) -> Result<bool, String> {
    let audit_processed = state
        .pointer("/audit_processed")
        .and_then(Value::as_array)
        .ok_or_else(|| "missing /audit_processed array in docs/state.json".to_string())?;

    if audit_processed
        .iter()
        .any(|value| audit_issue_for_entry(value) == Some(cli.audit_id))
    {
        return Ok(false);
    }

    let mut next_audit_processed = audit_processed.clone();
    next_audit_processed.push(build_audit_entry(cli, current_cycle));

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

fn validate_cli(cli: Cli) -> Result<ValidatedCli, String> {
    let disposition = cli
        .disposition
        .ok_or_else(|| "recording an audit disposition requires --disposition".to_string())?;
    let category = cli
        .category
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "new audit recommendation entries require --category".to_string())?
        .to_string();
    let summary = cli
        .summary
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .unwrap_or_else(|| format!("audit #{} disposition", cli.audit_id));

    let justification = cli
        .justification
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string);
    let adoption_artifact_reference = cli
        .adoption_artifact_reference
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());

    let adoption_artifact_reference = match disposition {
        AuditDispositionArg::Accept => {
            let payload = adoption_artifact_reference.ok_or_else(|| {
                "audit acceptance requires --adoption-artifact-reference; examples: {\"type\":\"pr\",\"number\":2490,\"url\":\"https://github.com/EvaLok/schema-org-json-ld/pull/2490\"} | {\"type\":\"commit\",\"sha\":\"abc1234\"} | {\"type\":\"pipeline_check_step\",\"name\":\"accepted-audit-adoption\"} | {\"type\":\"tool_change\",\"path\":\"COMPLETION_CHECKLIST.xml\",\"commit_sha\":\"abc1234\"}".to_string()
            })?;
            Some(parse_adoption_artifact_reference(payload)?)
        }
        AuditDispositionArg::Defer => {
            if justification.is_none() {
                return Err("audit defer requires --justification".to_string());
            }
            None
        }
        AuditDispositionArg::Reject => {
            if justification.is_none() {
                return Err("audit rejection requires --justification".to_string());
            }
            None
        }
    };

    Ok(ValidatedCli {
        audit_id: cli.audit_id,
        disposition,
        category,
        summary,
        justification,
        adoption_artifact_reference,
        repo_root: cli.repo_root,
    })
}

fn parse_adoption_artifact_reference(payload: &str) -> Result<AdoptionArtifactReference, String> {
    serde_json::from_str(payload).map_err(|error| {
        format!(
            "failed to parse --adoption-artifact-reference JSON: {}; example: {{\"type\":\"pr\",\"number\":2519,\"url\":\"https://github.com/EvaLok/schema-org-json-ld/issues/2519\"}}",
            error
        )
    })
}

fn audit_issue_for_entry(value: &Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.get("issue").and_then(Value::as_u64))
}

fn build_audit_entry(cli: &ValidatedCli, current_cycle: u64) -> Value {
    let mut recommendation = serde_json::Map::new();
    recommendation.insert("summary".to_string(), json!(cli.summary));
    recommendation.insert("category".to_string(), json!(cli.category));
    recommendation.insert("disposition".to_string(), json!(cli.disposition.as_str()));
    if cli.disposition == AuditDispositionArg::Accept {
        recommendation.insert("accepted_cycle".to_string(), json!(current_cycle));
    }
    if let Some(justification) = &cli.justification {
        recommendation.insert("justification".to_string(), json!(justification));
    }
    if let Some(reference) = &cli.adoption_artifact_reference {
        recommendation.insert(
            "adoption_artifact_reference".to_string(),
            serde_json::to_value(reference).expect("artifact reference should serialize"),
        );
    }

    json!({
        "issue": cli.audit_id,
        "processed_cycle": current_cycle,
        "accepted_recommendations": [Value::Object(recommendation)]
    })
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
            "audit_processed": [2, {
                "issue": 3,
                "processed_cycle": 164,
                "accepted_recommendations": [{
                    "summary": "existing structured record",
                    "category": "legacy-audit",
                    "disposition": "accept",
                    "accepted_cycle": 164,
                    "adoption_artifact_reference": {
                        "type": "pipeline_check_step",
                        "name": "artifact-verify"
                    }
                }]
            }],
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
        assert!(help.contains("--audit"));
        assert!(help.contains("--disposition"));
        assert!(help.contains("--category"));
        assert!(help.contains("--justification"));
        assert!(help.contains("--adoption-artifact-reference"));
        assert!(help.contains("--repo-root"));
        assert!(help.contains("accept"));
        assert!(help.contains("reject"));
        assert!(help.contains("defer"));
    }

    #[test]
    fn idempotency_does_not_duplicate_existing_audit_id() {
        let mut state = sample_state();
        let changed = apply_audit_processing(
            &mut state,
            &validated_cli(3, AuditDispositionArg::Accept),
            166,
        )
        .expect("apply should succeed");
        assert!(!changed);

        let processed = state
            .pointer("/audit_processed")
            .and_then(Value::as_array)
            .expect("audit_processed should exist");
        assert_eq!(processed.len(), 2);
    }

    #[test]
    fn appends_new_structured_audit_entry() {
        let mut state = sample_state();
        let changed = apply_audit_processing(
            &mut state,
            &validated_cli(586, AuditDispositionArg::Accept),
            166,
        )
        .expect("apply should succeed");
        assert!(changed);

        let entry = state
            .pointer("/audit_processed")
            .and_then(Value::as_array)
            .and_then(|values| values.last())
            .expect("structured audit entry should exist");
        assert_eq!(entry.get("issue").and_then(Value::as_u64), Some(586));
        let recommendation = entry
            .get("accepted_recommendations")
            .and_then(Value::as_array)
            .and_then(|values| values.first())
            .expect("recommendation should exist");
        assert_eq!(
            recommendation.get("category").and_then(Value::as_str),
            Some("chronic-category-tracking")
        );
    }

    #[test]
    fn freshness_marker_uses_cycle_format() {
        let mut state = sample_state();
        let _ = apply_audit_processing(
            &mut state,
            &validated_cli(586, AuditDispositionArg::Accept),
            166,
        )
        .expect("apply should succeed");
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

    #[test]
    fn validation_requires_disposition() {
        let error = validate_cli(Cli {
            audit_id: 402,
            disposition: None,
            category: Some("chronic-category-tracking".to_string()),
            summary: None,
            justification: None,
            adoption_artifact_reference: None,
            repo_root: PathBuf::from("."),
        })
        .expect_err("missing disposition should fail");

        assert_eq!(error, "recording an audit disposition requires --disposition");
    }

    #[test]
    fn validation_requires_category_for_new_entries() {
        let error = validate_cli(Cli {
            audit_id: 402,
            disposition: Some(AuditDispositionArg::Accept),
            category: None,
            summary: None,
            justification: None,
            adoption_artifact_reference: Some(
                "{\"type\":\"pipeline_check_step\",\"name\":\"accepted-audit-adoption\"}"
                    .to_string(),
            ),
            repo_root: PathBuf::from("."),
        })
        .expect_err("missing category should fail");

        assert_eq!(error, "new audit recommendation entries require --category");
    }

    #[test]
    fn acceptance_requires_artifact_reference() {
        let error = validate_cli(Cli {
            audit_id: 420,
            disposition: Some(AuditDispositionArg::Accept),
            category: Some("chronic-category-tracking".to_string()),
            summary: None,
            justification: None,
            adoption_artifact_reference: None,
            repo_root: PathBuf::from("."),
        })
        .expect_err("missing artifact reference should fail");

        assert!(error.contains("audit acceptance requires --adoption-artifact-reference"));
    }

    #[test]
    fn defer_requires_justification() {
        let error = validate_cli(Cli {
            audit_id: 406,
            disposition: Some(AuditDispositionArg::Defer),
            category: Some("chronic-category-tracking".to_string()),
            summary: None,
            justification: None,
            adoption_artifact_reference: None,
            repo_root: PathBuf::from("."),
        })
        .expect_err("missing defer justification should fail");

        assert_eq!(error, "audit defer requires --justification");
    }

    #[test]
    fn reject_requires_justification() {
        let error = validate_cli(Cli {
            audit_id: 999,
            disposition: Some(AuditDispositionArg::Reject),
            category: Some("other".to_string()),
            summary: None,
            justification: None,
            adoption_artifact_reference: None,
            repo_root: PathBuf::from("."),
        })
        .expect_err("missing reject justification should fail");

        assert_eq!(error, "audit rejection requires --justification");
    }

    fn validated_cli(audit_id: u64, disposition: AuditDispositionArg) -> ValidatedCli {
        validate_cli(Cli {
            audit_id,
            disposition: Some(disposition),
            category: Some("chronic-category-tracking".to_string()),
            summary: Some(format!("audit #{} disposition", audit_id)),
            justification: Some("documented rationale".to_string()),
            adoption_artifact_reference: Some(
                "{\"type\":\"pipeline_check_step\",\"name\":\"accepted-audit-adoption\"}"
                    .to_string(),
            ),
            repo_root: PathBuf::from("."),
        })
        .expect("cli should validate")
    }
}
