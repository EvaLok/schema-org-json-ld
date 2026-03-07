use clap::Parser;
use serde_json::{json, Value};
use state_schema::{commit_state_json, read_state_value, set_value_at_pointer, write_state_value};
use std::collections::BTreeSet;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "process-eva")]
struct Cli {
	/// Issues closed this cycle (comma-separated)
	#[arg(long, conflicts_with = "no_changes")]
	closed: Option<String>,

	/// Current open Eva directive issues (comma-separated)
	#[arg(
		long,
		required_unless_present = "no_changes",
		conflicts_with = "no_changes"
	)]
	remaining_open: Option<String>,

	/// Mark Eva directives as checked with no changes this cycle
	#[arg(long)]
	no_changes: bool,

	/// Repository root path
	#[arg(long, default_value = ".")]
	repo_root: PathBuf,
}

fn main() {
	let cli = Cli::parse();
	if let Err(error) = run(cli) {
		eprintln!("process-eva error: {}", error);
		std::process::exit(1);
	}
}

fn run(cli: Cli) -> Result<(), String> {
	let closed = parse_issue_list(cli.closed.as_deref())?;
	let remaining_open = if cli.no_changes {
		None
	} else {
		Some(parse_issue_list(cli.remaining_open.as_deref())?)
	};

	let mut state = read_state_value(&cli.repo_root)?;
	let next_cycle = read_next_cycle(&state)?;

	if !cli.no_changes {
		let existing_remaining = read_issue_array(&state, "/eva_input_issues/remaining_open")?;
		for issue in &closed {
			if !existing_remaining.contains(issue) {
				eprintln!(
					"Warning: Eva directive #{} was not in remaining_open at the start of this cycle; it may have been removed earlier or added to --closed by mistake. Proceeding anyway.",
					issue
				);
			}
		}
	}

	apply_eva_processing(
		&mut state,
		&closed,
		remaining_open.as_deref(),
		cli.no_changes,
		next_cycle,
	)?;
	write_state_value(&cli.repo_root, &state)?;

	let commit_message = if cli.no_changes {
		format!("state(process-eva): no changes [cycle {}]", next_cycle)
	} else {
		format!(
			"state(process-eva): closed {}, remaining {} [cycle {}]",
			format_issue_list(&closed),
			format_issue_list(remaining_open.as_deref().unwrap_or(&[])),
			next_cycle
		)
	};
	let receipt = commit_state_json(&cli.repo_root, &commit_message)?;
	if cli.no_changes {
		println!(
			"Eva directives checked: no changes [cycle {}] (receipt: {})",
			next_cycle, receipt
		);
	} else {
		println!(
			"Eva directives processed: closed {}, remaining {} [cycle {}] (receipt: {})",
			format_issue_list(&closed),
			format_issue_list(remaining_open.as_deref().unwrap_or(&[])),
			next_cycle,
			receipt
		);
	}

	Ok(())
}

fn read_next_cycle(state: &Value) -> Result<u64, String> {
	let last_cycle = state
		.pointer("/last_cycle/number")
		.and_then(Value::as_u64)
		.ok_or_else(|| {
			"missing or non-numeric /last_cycle/number in docs/state.json (expected positive integer)"
				.to_string()
		})?;

	last_cycle
		.checked_add(1)
		.ok_or_else(|| "cycle overflow while computing current cycle".to_string())
}

fn parse_issue_list(raw: Option<&str>) -> Result<Vec<u64>, String> {
	let Some(raw) = raw else {
		return Ok(Vec::new());
	};

	let trimmed = raw.trim();
	if trimmed.is_empty() {
		return Ok(Vec::new());
	}

	let mut issues = BTreeSet::new();
	for segment in trimmed.split(',') {
		let issue_text = segment.trim();
		if issue_text.is_empty() {
			return Err("issue lists must not contain empty values".to_string());
		}
		let issue = issue_text
			.parse::<u64>()
			.map_err(|error| format!("invalid issue number '{}': {}", issue_text, error))?;
		issues.insert(issue);
	}

	Ok(issues.into_iter().collect())
}

fn read_issue_array(state: &Value, pointer: &str) -> Result<Vec<u64>, String> {
	let values = state
		.pointer(pointer)
		.and_then(Value::as_array)
		.ok_or_else(|| format!("missing array {} in docs/state.json", pointer))?;

	values
		.iter()
		.map(|value| {
			value
				.as_u64()
				.ok_or_else(|| format!("array {} must contain only non-negative integers", pointer))
		})
		.collect()
}

fn apply_eva_processing(
	state: &mut Value,
	closed: &[u64],
	remaining_open: Option<&[u64]>,
	no_changes: bool,
	next_cycle: u64,
) -> Result<(), String> {
	let cycle_marker = format!("cycle {}", next_cycle);

	if no_changes {
		let existing_remaining = read_issue_array(state, "/eva_input_issues/remaining_open")?;
		set_value_at_pointer(state, "/eva_input_issues/closed_this_cycle", json!([]))?;
		set_value_at_pointer(
			state,
			"/eva_input_issues/remaining_open",
			json!(existing_remaining),
		)?;
	} else {
		let provided_remaining = remaining_open
			.ok_or_else(|| "--remaining-open is required unless --no-changes is used".to_string())?;
		let closed_set: BTreeSet<u64> = closed.iter().copied().collect();
		let mut next_remaining: Vec<u64> = provided_remaining.to_vec();
		next_remaining.sort_unstable();
		next_remaining.dedup();
		next_remaining.retain(|issue| !closed_set.contains(issue));

		let mut next_closed_prior = read_issue_array(state, "/eva_input_issues/closed_prior_cycles")?;
		next_closed_prior.extend(closed.iter().copied());
		next_closed_prior.sort_unstable();
		next_closed_prior.dedup();

		set_value_at_pointer(
			state,
			"/eva_input_issues/closed_prior_cycles",
			json!(next_closed_prior),
		)?;
		set_value_at_pointer(state, "/eva_input_issues/closed_this_cycle", json!(closed))?;
		set_value_at_pointer(state, "/eva_input_issues/remaining_open", json!(next_remaining))?;
	}

	set_value_at_pointer(
		state,
		"/field_inventory/fields/eva_input_issues.closed_this_cycle/last_refreshed",
		json!(cycle_marker.as_str()),
	)?;
	set_value_at_pointer(
		state,
		"/field_inventory/fields/eva_input_issues.remaining_open/last_refreshed",
		json!(cycle_marker.as_str()),
	)?;

	Ok(())
}

fn format_issue_list(issues: &[u64]) -> String {
	let issue_list = issues
		.iter()
		.map(u64::to_string)
		.collect::<Vec<_>>()
		.join(",");
	format!("[{}]", issue_list)
}

#[cfg(test)]
mod tests {
	use super::*;
	use clap::CommandFactory;
	use serde_json::json;

	fn sample_state() -> Value {
		json!({
			"last_cycle": { "number": 167 },
			"eva_input_issues": {
				"closed_prior_cycles": [180, 500, 501],
				"closed_this_cycle": [600],
				"remaining_open": [247, 436, 500, 586, 591]
			},
			"field_inventory": {
				"fields": {
					"eva_input_issues.closed_this_cycle": {
						"last_refreshed": "cycle 167"
					},
					"eva_input_issues.remaining_open": {
						"last_refreshed": "cycle 164"
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
		assert!(help.contains("--closed"));
		assert!(help.contains("--remaining-open"));
		assert!(help.contains("--no-changes"));
		assert!(help.contains("--repo-root"));
	}

	#[test]
	fn no_changes_resets_closed_this_cycle_and_updates_freshness() {
		let mut state = sample_state();
		apply_eva_processing(&mut state, &[], None, true, 168).expect("apply should succeed");

		assert_eq!(
			state.pointer("/eva_input_issues/closed_this_cycle"),
			Some(&json!([]))
		);
		assert_eq!(
			state.pointer("/eva_input_issues/remaining_open"),
			Some(&json!([247, 436, 500, 586, 591]))
		);
		assert_eq!(
			state.pointer("/field_inventory/fields/eva_input_issues.closed_this_cycle/last_refreshed"),
			Some(&json!("cycle 168"))
		);
		assert_eq!(
			state.pointer("/field_inventory/fields/eva_input_issues.remaining_open/last_refreshed"),
			Some(&json!("cycle 168"))
		);
	}

	#[test]
	fn closed_issue_moves_to_closed_lists_and_out_of_remaining_open() {
		let mut state = sample_state();
		apply_eva_processing(&mut state, &[500], Some(&[247, 436, 586, 591]), false, 168)
			.expect("apply should succeed");

		assert_eq!(
			state.pointer("/eva_input_issues/closed_this_cycle"),
			Some(&json!([500]))
		);
		assert_eq!(
			state.pointer("/eva_input_issues/closed_prior_cycles"),
			Some(&json!([180, 500, 501]))
		);
		assert_eq!(
			state.pointer("/eva_input_issues/remaining_open"),
			Some(&json!([247, 436, 586, 591]))
		);
	}

	#[test]
	fn remaining_open_replaces_list_entirely() {
		let mut state = sample_state();
		apply_eva_processing(&mut state, &[], Some(&[591, 247]), false, 168)
			.expect("apply should succeed");

		assert_eq!(
			state.pointer("/eva_input_issues/closed_this_cycle"),
			Some(&json!([]))
		);
		assert_eq!(
			state.pointer("/eva_input_issues/remaining_open"),
			Some(&json!([247, 591]))
		);
	}

	#[test]
	fn closed_and_no_changes_conflict_produces_error() {
		let error = Cli::try_parse_from(["process-eva", "--closed", "500", "--no-changes"])
			.expect_err("args should conflict");
		assert_eq!(error.kind(), clap::error::ErrorKind::ArgumentConflict);
	}

	#[test]
	fn freshness_markers_are_updated_when_remaining_open_changes() {
		let mut state = sample_state();
		apply_eva_processing(&mut state, &[], Some(&[247, 436, 586, 591]), false, 168)
			.expect("apply should succeed");

		assert_eq!(
			state.pointer("/field_inventory/fields/eva_input_issues.closed_this_cycle/last_refreshed"),
			Some(&json!("cycle 168"))
		);
		assert_eq!(
			state.pointer("/field_inventory/fields/eva_input_issues.remaining_open/last_refreshed"),
			Some(&json!("cycle 168"))
		);
	}

	#[test]
	fn closed_prior_cycles_is_sorted_and_deduplicated_after_append() {
		let mut state = sample_state();
		state["eva_input_issues"]["closed_prior_cycles"] = json!([501, 180, 500, 500]);

		apply_eva_processing(&mut state, &[500, 436], Some(&[247, 586, 591]), false, 168)
			.expect("apply should succeed");

		assert_eq!(
			state.pointer("/eva_input_issues/closed_prior_cycles"),
			Some(&json!([180, 436, 500, 501]))
		);
	}

	#[test]
	fn next_cycle_is_last_cycle_plus_one() {
		let state = sample_state();
		assert_eq!(read_next_cycle(&state).unwrap(), 168);
	}
}
