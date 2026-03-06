use clap::Parser;
use serde_json::{json, Value};
use state_schema::set_value_at_pointer;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "process-merge")]
struct Cli {
	/// Comma-separated list of merged PR numbers (e.g. "595,597,599")
	#[arg(long, value_delimiter = ',', num_args = 1..)]
	prs: Vec<u64>,

	/// Repository root path
	#[arg(long, default_value = ".")]
	repo_root: PathBuf,

	/// Optional note to append to copilot_metrics.note
	#[arg(long)]
	note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MergeUpdate {
	merged: i64,
	resolved: i64,
	in_flight: i64,
	pr_merge_rate: String,
	dispatch_to_pr_rate: String,
	note: String,
	total_dispatches: i64,
	cycle: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PatchUpdate {
	path: &'static str,
	value: Value,
}

fn main() {
	let cli = Cli::parse();
	if let Err(error) = run(cli) {
		eprintln!("Error: {}", error);
		std::process::exit(1);
	}
}

fn run(cli: Cli) -> Result<(), String> {
	if cli.prs.is_empty() {
		return Err("at least one PR number is required via --prs".to_string());
	}

	let state_path = cli.repo_root.join("docs/state.json");
	let mut state = read_state_value(&state_path)?;
	let update = compute_update(&state, &cli.prs, cli.note.as_deref())?;
	let patch = build_patch(&update)?;
	apply_patch(&mut state, &patch)?;
	write_state_value(&state_path, &state)?;

	let receipt = commit_state_json(&cli.repo_root, &cli.prs, update.cycle)?;
	println!(
		"Merge processed: {}. Copilot metrics: {} dispatches, {} merged (receipt: {})",
		format_pr_list(&cli.prs),
		update.total_dispatches,
		update.merged,
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

fn get_metric_i64(state: &Value, field: &str) -> Result<i64, String> {
	state
		.pointer(&format!("/copilot_metrics/{}", field))
		.and_then(Value::as_i64)
		.ok_or_else(|| format!("missing numeric /copilot_metrics/{} in docs/state.json", field))
}

fn get_cycle(state: &Value) -> Result<u64, String> {
	state
		.pointer("/last_cycle/number")
		.and_then(Value::as_u64)
		.ok_or_else(|| "missing numeric /last_cycle/number in docs/state.json".to_string())
}

fn compute_update(state: &Value, prs: &[u64], note: Option<&str>) -> Result<MergeUpdate, String> {
	let merged = get_metric_i64(state, "merged")?;
	let resolved = get_metric_i64(state, "resolved")?;
	let in_flight = get_metric_i64(state, "in_flight")?;
	let produced_pr = get_metric_i64(state, "produced_pr")?;
	let total_dispatches = get_metric_i64(state, "total_dispatches")?;
	let closed_without_merge = get_metric_i64(state, "closed_without_merge")?;
	let closed_without_pr = get_metric_i64(state, "closed_without_pr")?;
	let cycle = get_cycle(state)?;

	if in_flight < 0 {
		return Err(format!("copilot_metrics.in_flight({}) must be non-negative", in_flight));
	}
	if produced_pr < 0 || merged < 0 || resolved < 0 || total_dispatches < 0 {
		return Err("copilot metrics counters must be non-negative".to_string());
	}

	let merge_count = i64::try_from(prs.len()).map_err(|_| "PR count is too large".to_string())?;
	let resolved_increment = merge_count.min(in_flight);
	let next_in_flight = in_flight - resolved_increment;
	if resolved_increment < merge_count {
		eprintln!(
			"Warning: in_flight underflow prevented (requested {}, decremented {})",
			merge_count, resolved_increment
		);
	}

	let next_merged = merged + merge_count;
	let next_resolved = resolved + resolved_increment;

	if next_merged + closed_without_merge > produced_pr {
		return Err(format!(
			"invariant violated: merged({}) + closed_without_merge({}) > produced_pr({})",
			next_merged, closed_without_merge, produced_pr
		));
	}

	if next_resolved + next_in_flight != total_dispatches {
		return Err(format!(
			"invariant violated: resolved({}) + in_flight({}) != total_dispatches({})",
			next_resolved, next_in_flight, total_dispatches
		));
	}

	let pr_merge_rate = format!("{}/{}", next_merged, produced_pr);
	let dispatch_to_pr_rate = format!("{}/{}", produced_pr, total_dispatches);
	let merged_prs = format_pr_list(prs);

	let mut summary = format!(
		"{} dispatches, {} resolved, {} in-flight. {} produced PRs ({} merged, {} closed without merge). {} resolved without PR (silent failure). {} merged cycle {}.",
		total_dispatches,
		next_resolved,
		next_in_flight,
		produced_pr,
		next_merged,
		closed_without_merge,
		closed_without_pr,
		merged_prs,
		cycle
	);

	if let Some(extra_note) = note.map(str::trim).filter(|value| !value.is_empty()) {
		summary.push(' ');
		summary.push_str(extra_note);
	}

	Ok(MergeUpdate {
		merged: next_merged,
		resolved: next_resolved,
		in_flight: next_in_flight,
		pr_merge_rate,
		dispatch_to_pr_rate,
		note: summary,
		total_dispatches,
		cycle,
	})
}

fn build_patch(update: &MergeUpdate) -> Result<Vec<PatchUpdate>, String> {
	let cycle = u32::try_from(update.cycle).map_err(|_| "cycle must fit in u32 range".to_string())?;
	let marker = format!("cycle {}", cycle);

	Ok(vec![
		PatchUpdate {
			path: "/copilot_metrics/merged",
			value: json!(update.merged),
		},
		PatchUpdate {
			path: "/copilot_metrics/resolved",
			value: json!(update.resolved),
		},
		PatchUpdate {
			path: "/copilot_metrics/in_flight",
			value: json!(update.in_flight),
		},
		PatchUpdate {
			path: "/copilot_metrics/pr_merge_rate",
			value: json!(update.pr_merge_rate),
		},
		PatchUpdate {
			path: "/copilot_metrics/dispatch_to_pr_rate",
			value: json!(update.dispatch_to_pr_rate),
		},
		PatchUpdate {
			path: "/copilot_metrics/note",
			value: json!(update.note),
		},
		PatchUpdate {
			path: "/field_inventory/fields/copilot_metrics.in_flight/last_refreshed",
			value: json!(marker),
		},
		PatchUpdate {
			path: "/field_inventory/fields/copilot_metrics.pr_merge_rate/last_refreshed",
			value: json!(marker),
		},
		PatchUpdate {
			path: "/field_inventory/fields/copilot_metrics.dispatch_to_pr_rate/last_refreshed",
			value: json!(marker),
		},
	])
}

fn apply_patch(state: &mut Value, updates: &[PatchUpdate]) -> Result<(), String> {
	for update in updates {
		set_value_at_pointer(state, update.path, update.value.clone())?;
	}

	Ok(())
}

fn format_pr_list(prs: &[u64]) -> String {
	let formatted: Vec<String> = prs.iter().map(|pr| format!("#{}", pr)).collect();
	if formatted.len() == 1 {
		format!("PR {}", formatted[0])
	} else {
		format!("PRs {}", formatted.join(", "))
	}
}

fn commit_state_json(repo_root: &Path, prs: &[u64], current_cycle: u64) -> Result<String, String> {
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

	let commit_message = if prs.len() == 1 {
		format!(
			"state(process-merge): PR #{} merged [cycle {}]",
			prs[0], current_cycle
		)
	} else {
		let merged_list = prs
			.iter()
			.map(|pr| format!("#{}", pr))
			.collect::<Vec<_>>()
			.join(", ");
		format!(
			"state(process-merge): PRs {} merged [cycle {}]",
			merged_list, current_cycle
		)
	};

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
			"last_cycle": {"number": 164},
			"copilot_metrics": {
				"closed_without_merge": 1,
				"closed_without_pr": 1,
				"dispatch_to_pr_rate": "84/85",
				"in_flight": 3,
				"merged": 80,
				"note": "prior",
				"pr_merge_rate": "80/84",
				"produced_pr": 84,
				"resolved": 82,
				"total_dispatches": 85
			},
			"field_inventory": {
				"fields": {
					"copilot_metrics.in_flight": {"last_refreshed": "cycle 163"},
					"copilot_metrics.pr_merge_rate": {"last_refreshed": "cycle 163"},
					"copilot_metrics.dispatch_to_pr_rate": {"last_refreshed": "cycle 163"}
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
		assert!(help.contains("--prs"));
		assert!(help.contains("--repo-root"));
		assert!(help.contains("--note"));
	}

	#[test]
	fn metric_calculation_single_pr_merge() {
		let state = sample_state();
		let update = compute_update(&state, &[595], None).expect("update should compute");
		assert_eq!(update.merged, 81);
		assert_eq!(update.resolved, 83);
		assert_eq!(update.in_flight, 2);
		assert_eq!(update.pr_merge_rate, "81/84");
		assert_eq!(update.dispatch_to_pr_rate, "84/85");
		assert!(update.note.contains("PR #595 merged cycle 164."));
	}

	#[test]
	fn metric_calculation_multiple_pr_merge() {
		let state = sample_state();
		let update = compute_update(&state, &[595, 597, 599], Some("Merged as planned."))
			.expect("update should compute");
		assert_eq!(update.merged, 83);
		assert_eq!(update.resolved, 85);
		assert_eq!(update.in_flight, 0);
		assert_eq!(update.pr_merge_rate, "83/84");
		assert_eq!(update.dispatch_to_pr_rate, "84/85");
		assert!(update.note.contains("PRs #595, #597, #599 merged cycle 164."));
		assert!(update.note.ends_with("Merged as planned."));
	}

	#[test]
	fn in_flight_underflow_protection_and_invariants() {
		let mut state = sample_state();
		state["copilot_metrics"]["in_flight"] = json!(1);
		state["copilot_metrics"]["resolved"] = json!(84);
		let update = compute_update(&state, &[595, 597], None).expect("update should compute");
		assert_eq!(update.merged, 82);
		assert_eq!(update.resolved, 85);
		assert_eq!(update.in_flight, 0);
		assert_eq!(update.resolved + update.in_flight, update.total_dispatches);
	}

	#[test]
	fn invariant_validation_detects_mismatch() {
		let mut state = sample_state();
		state["copilot_metrics"]["total_dispatches"] = json!(84);
		let error = compute_update(&state, &[595], None).expect_err("invariant should fail");
		assert!(error.contains("invariant violated"));
	}

	#[test]
	fn patch_updates_rates_and_freshness_markers() {
		let state = sample_state();
		let update = compute_update(&state, &[595], None).expect("update should compute");
		let patch = build_patch(&update).expect("patch should build");
		assert_eq!(patch.len(), 9);
		assert_eq!(patch[3].path, "/copilot_metrics/pr_merge_rate");
		assert_eq!(patch[4].path, "/copilot_metrics/dispatch_to_pr_rate");
		assert_eq!(
			patch[6].path,
			"/field_inventory/fields/copilot_metrics.in_flight/last_refreshed"
		);
		assert_eq!(patch[6].value, json!("cycle 164"));
	}

	#[test]
	fn format_pr_list_handles_single_and_multiple_values() {
		assert_eq!(format_pr_list(&[595]), "PR #595");
		assert_eq!(format_pr_list(&[595, 597]), "PRs #595, #597");
	}
}
