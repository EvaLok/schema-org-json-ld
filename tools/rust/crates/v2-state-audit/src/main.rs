// v2-state-audit
//
// Cross-axis state-surface auditor. Walks the v2 state surface enumerated
// by cycle 158 v2-state-retention-policy §2 and `docs/state.json`, classifies
// each axis against the advisory/mandatory/hard thresholds from policy §4,
// and emits a per-axis breakdown plus a cross-axis ceiling check and an
// overall severity rollup. Recommends action per kind: `ok` -> no-op,
// `advisory` -> journal-observation, `mandatory` -> trigger-archival,
// `hard` -> halt-session-start.
//
// Source of truth for thresholds is the policy document at
// `docs/redesign/_notes/v2-state-retention-policy.md`. The constants below
// codify policy version 1 (cycle 158); revisions to the policy must
// revise these constants in lock-step. The `policy_version` field in the
// report exists to make stale-tool detection explicit.
//
// CORE-DESIGN-PRINCIPLE alignment: this tool extracts the per-axis size/
// count probe that policy §5 names ("walks state/ and docs/state.json to
// produce per-axis size + entry counts"). The orchestrator invokes the
// tool at session-start (per policy §5 cadence) and reads the
// classification — it does not walk the surface itself.
//
// Read-only: no mutation, no archival action. Hard-classification action
// is "halt-session-start" — the runner is responsible for the halt; this
// tool only reports.
//
// Cycle 159 reality: `state/` directory does not yet exist (SCAFFOLD-only
// v2 state surface). All measurements tolerate missing files and missing
// directories by treating them as zero bytes / zero entries.

use clap::Parser;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

const TOOL_NAME: &str = "v2-state-audit";
const TOOL_VERSION: &str = "0.1.0";
const POLICY_VERSION: u32 = 1;

// --------------------------------------------------------------------------
// Threshold constants (policy doc §4, cycle 158)
// --------------------------------------------------------------------------

const MB: u64 = 1024 * 1024;
const GB: u64 = 1024 * MB;

// Axis 1: state/channels/history/ — directory size
const CHANNELS_HISTORY_ADVISORY: u64 = 5 * MB;
const CHANNELS_HISTORY_MANDATORY: u64 = 25 * MB;
const CHANNELS_HISTORY_HARD: u64 = 100 * MB;

// Axis 2: state/super-step-history.json — file size
const SUPER_STEP_HISTORY_ADVISORY: u64 = 2 * MB;
const SUPER_STEP_HISTORY_MANDATORY: u64 = 8 * MB;
const SUPER_STEP_HISTORY_HARD: u64 = 32 * MB;

// Axis 3: state/roles/<role>-history.json — file size, per file
const ROLE_HISTORY_ADVISORY: u64 = MB;
const ROLE_HISTORY_MANDATORY: u64 = 4 * MB;
const ROLE_HISTORY_HARD: u64 = 16 * MB;

// Axis 4: state/reconciler/poll-history.json — file size
const POLL_HISTORY_ADVISORY: u64 = 2 * MB;
const POLL_HISTORY_MANDATORY: u64 = 8 * MB;
const POLL_HISTORY_HARD: u64 = 32 * MB;

// Axis 5: state/v2-cycle-runner/cycle-history.json — file size
const CYCLE_HISTORY_ADVISORY: u64 = MB;
const CYCLE_HISTORY_MANDATORY: u64 = 4 * MB;
const CYCLE_HISTORY_HARD: u64 = 16 * MB;

// Axis 6: docs/state.json dispatches array — entry count
const DISPATCHES_ADVISORY: u64 = 50;
const DISPATCHES_MANDATORY: u64 = 200;
const DISPATCHES_HARD: u64 = 500;

// Cross-axis: total v2 state surface (live, all axes summed)
const TOTAL_ADVISORY: u64 = 500 * MB;
const TOTAL_MANDATORY: u64 = 2 * GB;
const TOTAL_HARD: u64 = 4 * GB;

// --------------------------------------------------------------------------
// CLI
// --------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(
    name = "v2-state-audit",
    about = "Audit the v2 state surface against retention-policy thresholds"
)]
struct Args {
    /// Repository root (default: current directory)
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Emit machine-readable JSON instead of human-readable summary
    #[arg(long, default_value_t = false)]
    json: bool,
}

// --------------------------------------------------------------------------
// Model
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, PartialOrd, Ord)]
#[serde(rename_all = "kebab-case")]
enum Kind {
    Ok,
    Advisory,
    Mandatory,
    Hard,
}

impl Kind {
    fn recommended_action(self) -> &'static str {
        match self {
            Kind::Ok => "no-op",
            Kind::Advisory => "journal-observation",
            Kind::Mandatory => "trigger-archival",
            Kind::Hard => "halt-session-start",
        }
    }

    fn as_kebab(self) -> &'static str {
        match self {
            Kind::Ok => "ok",
            Kind::Advisory => "advisory",
            Kind::Mandatory => "mandatory",
            Kind::Hard => "hard",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
enum Unit {
    Bytes,
    Entries,
}

#[derive(Debug, Clone, Copy, Serialize)]
struct Thresholds {
    advisory: u64,
    mandatory: u64,
    hard: u64,
}

#[derive(Debug, Clone, Serialize)]
struct AxisReport {
    name: &'static str,
    path: String,
    /// "file-size", "directory-size", or "json-array-count".
    measurement_kind: &'static str,
    /// Unit of `value` and `thresholds`.
    unit: Unit,
    /// Did the target path exist when measured?
    exists: bool,
    /// Measured value (bytes for file/dir size, count for entries).
    value: u64,
    thresholds: Thresholds,
    classification: Kind,
    recommended_action: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct CrossAxisReport {
    /// Sum of byte measurements across all byte-unit axes (entries-unit
    /// axes do not contribute to the byte total).
    total_state_bytes: u64,
    thresholds: Thresholds,
    classification: Kind,
    recommended_action: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct OverallReport {
    classification: Kind,
    recommended_action: &'static str,
}

#[derive(Debug, Clone, Serialize)]
struct Report {
    tool: &'static str,
    version: &'static str,
    policy_version: u32,
    repo_root: String,
    measured_at: String,
    axes: Vec<AxisReport>,
    cross_axis: CrossAxisReport,
    overall: OverallReport,
}

// --------------------------------------------------------------------------
// Time
// --------------------------------------------------------------------------

fn iso8601_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format_unix_secs_iso(secs)
}

fn format_unix_secs_iso(unix_secs: u64) -> String {
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

// --------------------------------------------------------------------------
// Measurement primitives
// --------------------------------------------------------------------------

/// File size in bytes, or 0 if the path is missing or not a file.
/// Tolerates absent state surfaces (SCAFFOLD case).
fn measure_file_size(path: &Path) -> (bool, u64) {
    match fs::metadata(path) {
        Ok(m) if m.is_file() => (true, m.len()),
        _ => (false, 0),
    }
}

/// Cumulative size in bytes of all regular files anywhere under `path`,
/// or 0 if the path is missing or not a directory. Recurses; symlinks
/// are not followed.
fn measure_directory_size(path: &Path) -> (bool, u64) {
    let md = match fs::metadata(path) {
        Ok(m) if m.is_dir() => m,
        _ => return (false, 0),
    };
    let _ = md;
    let mut total: u64 = 0;
    let mut stack: Vec<PathBuf> = vec![path.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let entries = match fs::read_dir(&dir) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries.flatten() {
            let ep = entry.path();
            let m = match entry.metadata() {
                Ok(m) => m,
                Err(_) => continue,
            };
            if m.is_dir() {
                stack.push(ep);
            } else if m.is_file() {
                total = total.saturating_add(m.len());
            }
        }
    }
    (true, total)
}

/// Length of the JSON array at `array_key` in the file at `path`, or 0 if
/// the file is missing, not parseable, or the key is missing / not an
/// array. The "missing -> 0" rule is deliberate: SCAFFOLD case must not
/// produce false-positive classifications. Genuine parse errors during
/// real-state inspection should be surfaced via separate tooling (out of
/// scope for cycle 159).
fn measure_json_array_count(path: &Path, array_key: &str) -> (bool, u64) {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(_) => return (false, 0),
    };
    let parsed: Value = match serde_json::from_str(&text) {
        Ok(v) => v,
        Err(_) => return (true, 0),
    };
    let arr = match parsed.get(array_key).and_then(|v| v.as_array()) {
        Some(a) => a,
        None => return (true, 0),
    };
    (true, arr.len() as u64)
}

// --------------------------------------------------------------------------
// Classification
// --------------------------------------------------------------------------

/// Classify a measured value against thresholds. Boundary semantics: at-or-
/// above advisory but below mandatory is `Advisory`; at-or-above mandatory
/// but below hard is `Mandatory`; at-or-above hard is `Hard`. Strictly
/// below advisory is `Ok`.
fn classify(value: u64, t: Thresholds) -> Kind {
    if value >= t.hard {
        Kind::Hard
    } else if value >= t.mandatory {
        Kind::Mandatory
    } else if value >= t.advisory {
        Kind::Advisory
    } else {
        Kind::Ok
    }
}

// --------------------------------------------------------------------------
// Axis configuration table
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
enum MeasurementSpec {
    FileSize,
    DirectorySize,
    /// (`array_key`,) — the JSON object key whose array length is the value.
    JsonArrayCount(&'static str),
}

#[derive(Debug, Clone, Copy)]
struct AxisSpec {
    name: &'static str,
    /// Relative path under the repo root.
    relative_path: &'static str,
    spec: MeasurementSpec,
    thresholds: Thresholds,
    unit: Unit,
    /// Whether the measured bytes contribute to the cross-axis byte total.
    /// Entry-count axes do not contribute to byte total.
    contributes_to_byte_total: bool,
}

fn all_axes() -> [AxisSpec; 9] {
    [
        AxisSpec {
            name: "channels-history",
            relative_path: "state/channels/history",
            spec: MeasurementSpec::DirectorySize,
            thresholds: Thresholds {
                advisory: CHANNELS_HISTORY_ADVISORY,
                mandatory: CHANNELS_HISTORY_MANDATORY,
                hard: CHANNELS_HISTORY_HARD,
            },
            unit: Unit::Bytes,
            contributes_to_byte_total: true,
        },
        AxisSpec {
            name: "super-step-history",
            relative_path: "state/super-step-history.json",
            spec: MeasurementSpec::FileSize,
            thresholds: Thresholds {
                advisory: SUPER_STEP_HISTORY_ADVISORY,
                mandatory: SUPER_STEP_HISTORY_MANDATORY,
                hard: SUPER_STEP_HISTORY_HARD,
            },
            unit: Unit::Bytes,
            contributes_to_byte_total: true,
        },
        AxisSpec {
            name: "reconciler-role-history",
            relative_path: "state/roles/reconciler-history.json",
            spec: MeasurementSpec::FileSize,
            thresholds: Thresholds {
                advisory: ROLE_HISTORY_ADVISORY,
                mandatory: ROLE_HISTORY_MANDATORY,
                hard: ROLE_HISTORY_HARD,
            },
            unit: Unit::Bytes,
            contributes_to_byte_total: true,
        },
        AxisSpec {
            name: "planner-role-history",
            relative_path: "state/roles/planner-history.json",
            spec: MeasurementSpec::FileSize,
            thresholds: Thresholds {
                advisory: ROLE_HISTORY_ADVISORY,
                mandatory: ROLE_HISTORY_MANDATORY,
                hard: ROLE_HISTORY_HARD,
            },
            unit: Unit::Bytes,
            contributes_to_byte_total: true,
        },
        AxisSpec {
            name: "executor-role-history",
            relative_path: "state/roles/executor-history.json",
            spec: MeasurementSpec::FileSize,
            thresholds: Thresholds {
                advisory: ROLE_HISTORY_ADVISORY,
                mandatory: ROLE_HISTORY_MANDATORY,
                hard: ROLE_HISTORY_HARD,
            },
            unit: Unit::Bytes,
            contributes_to_byte_total: true,
        },
        AxisSpec {
            name: "curator-role-history",
            relative_path: "state/roles/curator-history.json",
            spec: MeasurementSpec::FileSize,
            thresholds: Thresholds {
                advisory: ROLE_HISTORY_ADVISORY,
                mandatory: ROLE_HISTORY_MANDATORY,
                hard: ROLE_HISTORY_HARD,
            },
            unit: Unit::Bytes,
            contributes_to_byte_total: true,
        },
        AxisSpec {
            name: "reconciler-poll-history",
            relative_path: "state/reconciler/poll-history.json",
            spec: MeasurementSpec::FileSize,
            thresholds: Thresholds {
                advisory: POLL_HISTORY_ADVISORY,
                mandatory: POLL_HISTORY_MANDATORY,
                hard: POLL_HISTORY_HARD,
            },
            unit: Unit::Bytes,
            contributes_to_byte_total: true,
        },
        AxisSpec {
            name: "cycle-runner-cycle-history",
            relative_path: "state/v2-cycle-runner/cycle-history.json",
            spec: MeasurementSpec::FileSize,
            thresholds: Thresholds {
                advisory: CYCLE_HISTORY_ADVISORY,
                mandatory: CYCLE_HISTORY_MANDATORY,
                hard: CYCLE_HISTORY_HARD,
            },
            unit: Unit::Bytes,
            contributes_to_byte_total: true,
        },
        AxisSpec {
            // Conceptually the "dispatches array" per policy doc §4 Axis 6.
            // Implementation storage key in legacy v1 `docs/state.json` is
            // `agent_sessions` — cycle 159 tool-extraction surfaced the
            // naming gap between cycle 158 policy authoring (called it
            // "dispatches") and v2-state-dispatch-sync implementation
            // (uses `agent_sessions`). Policy doc was patched in cycle 159
            // to record the actual storage key. Axis name `state-json-
            // dispatches` is preserved as the policy-conceptual name.
            name: "state-json-dispatches",
            relative_path: "docs/state.json",
            spec: MeasurementSpec::JsonArrayCount("agent_sessions"),
            thresholds: Thresholds {
                advisory: DISPATCHES_ADVISORY,
                mandatory: DISPATCHES_MANDATORY,
                hard: DISPATCHES_HARD,
            },
            unit: Unit::Entries,
            contributes_to_byte_total: false,
        },
    ]
}

fn cross_axis_thresholds() -> Thresholds {
    Thresholds {
        advisory: TOTAL_ADVISORY,
        mandatory: TOTAL_MANDATORY,
        hard: TOTAL_HARD,
    }
}

// --------------------------------------------------------------------------
// Snapshot construction
// --------------------------------------------------------------------------

fn build_axis_report(spec: AxisSpec, repo_root: &Path) -> AxisReport {
    let path = repo_root.join(spec.relative_path);
    let (exists, value, measurement_kind) = match spec.spec {
        MeasurementSpec::FileSize => {
            let (e, v) = measure_file_size(&path);
            (e, v, "file-size")
        }
        MeasurementSpec::DirectorySize => {
            let (e, v) = measure_directory_size(&path);
            (e, v, "directory-size")
        }
        MeasurementSpec::JsonArrayCount(key) => {
            let (e, v) = measure_json_array_count(&path, key);
            let _ = key;
            (e, v, "json-array-count")
        }
    };
    let classification = classify(value, spec.thresholds);
    AxisReport {
        name: spec.name,
        path: spec.relative_path.to_string(),
        measurement_kind,
        unit: spec.unit,
        exists,
        value,
        thresholds: spec.thresholds,
        classification,
        recommended_action: classification.recommended_action(),
    }
}

fn build_cross_axis(axes: &[AxisReport], specs: &[AxisSpec]) -> CrossAxisReport {
    assert_eq!(axes.len(), specs.len());
    let mut total: u64 = 0;
    for (a, s) in axes.iter().zip(specs.iter()) {
        if s.contributes_to_byte_total {
            total = total.saturating_add(a.value);
        }
    }
    let thresholds = cross_axis_thresholds();
    let classification = classify(total, thresholds);
    CrossAxisReport {
        total_state_bytes: total,
        thresholds,
        classification,
        recommended_action: classification.recommended_action(),
    }
}

fn build_overall(axes: &[AxisReport], cross: &CrossAxisReport) -> OverallReport {
    let mut max_kind = cross.classification;
    for a in axes {
        if a.classification > max_kind {
            max_kind = a.classification;
        }
    }
    OverallReport {
        classification: max_kind,
        recommended_action: max_kind.recommended_action(),
    }
}

fn build_report(repo_root: &Path) -> Report {
    let specs = all_axes();
    let axes: Vec<AxisReport> = specs
        .iter()
        .map(|s| build_axis_report(*s, repo_root))
        .collect();
    let cross = build_cross_axis(&axes, &specs);
    let overall = build_overall(&axes, &cross);
    Report {
        tool: TOOL_NAME,
        version: TOOL_VERSION,
        policy_version: POLICY_VERSION,
        repo_root: repo_root.display().to_string(),
        measured_at: iso8601_now(),
        axes,
        cross_axis: cross,
        overall,
    }
}

// --------------------------------------------------------------------------
// Output
// --------------------------------------------------------------------------

fn format_value_with_unit(value: u64, unit: Unit) -> String {
    match unit {
        Unit::Bytes => format_bytes(value),
        Unit::Entries => format!("{value} entries"),
    }
}

fn format_bytes(b: u64) -> String {
    if b >= GB {
        format!("{:.2} GiB", b as f64 / GB as f64)
    } else if b >= MB {
        format!("{:.2} MiB", b as f64 / MB as f64)
    } else if b >= 1024 {
        format!("{:.2} KiB", b as f64 / 1024.0)
    } else {
        format!("{b} B")
    }
}

fn format_human(report: &Report) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "v2-state-audit  policy_version={}  measured_at={}\n",
        report.policy_version, report.measured_at
    ));
    out.push_str(&format!("repo_root: {}\n\n", report.repo_root));
    out.push_str("per-axis:\n");
    for a in &report.axes {
        let exists_marker = if a.exists { " " } else { "*" };
        out.push_str(&format!(
            "  {exists_marker} {:<32}  {:<24}  [{}]  {}\n",
            a.name,
            format_value_with_unit(a.value, a.unit),
            a.classification.as_kebab(),
            a.recommended_action,
        ));
    }
    out.push_str(&format!(
        "\ncross-axis total bytes: {}  [{}]  {}\n",
        format_bytes(report.cross_axis.total_state_bytes),
        report.cross_axis.classification.as_kebab(),
        report.cross_axis.recommended_action,
    ));
    out.push_str(&format!(
        "\noverall: [{}]  {}\n",
        report.overall.classification.as_kebab(),
        report.overall.recommended_action,
    ));
    out.push_str("(* = path not present at measurement time)\n");
    out
}

fn format_json(report: &Report) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(report)
}

// --------------------------------------------------------------------------
// Exit codes
// --------------------------------------------------------------------------

/// Exit code mirrors the highest kind seen:
/// 0 = ok, 1 = advisory, 2 = mandatory, 3 = hard.
/// Runner/orchestrator can decide policy from the code without parsing JSON.
fn exit_code_for(kind: Kind) -> ExitCode {
    match kind {
        Kind::Ok => ExitCode::from(0),
        Kind::Advisory => ExitCode::from(1),
        Kind::Mandatory => ExitCode::from(2),
        Kind::Hard => ExitCode::from(3),
    }
}

// --------------------------------------------------------------------------
// main
// --------------------------------------------------------------------------

fn main() -> ExitCode {
    let args = Args::parse();
    let report = build_report(&args.repo_root);

    let output = if args.json {
        match format_json(&report) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("v2-state-audit: failed to serialize JSON: {e}");
                return ExitCode::from(4);
            }
        }
    } else {
        format_human(&report)
    };
    println!("{output}");

    exit_code_for(report.overall.classification)
}

// --------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    fn t(advisory: u64, mandatory: u64, hard: u64) -> Thresholds {
        Thresholds { advisory, mandatory, hard }
    }

    // -- classify -- ------------------------------------------------------

    #[test]
    fn classify_below_advisory_is_ok() {
        assert_eq!(classify(0, t(100, 200, 300)), Kind::Ok);
        assert_eq!(classify(99, t(100, 200, 300)), Kind::Ok);
    }

    #[test]
    fn classify_at_advisory_is_advisory() {
        assert_eq!(classify(100, t(100, 200, 300)), Kind::Advisory);
    }

    #[test]
    fn classify_at_mandatory_is_mandatory() {
        assert_eq!(classify(200, t(100, 200, 300)), Kind::Mandatory);
    }

    #[test]
    fn classify_at_hard_is_hard() {
        assert_eq!(classify(300, t(100, 200, 300)), Kind::Hard);
        assert_eq!(classify(u64::MAX, t(100, 200, 300)), Kind::Hard);
    }

    #[test]
    fn classify_between_advisory_and_mandatory() {
        assert_eq!(classify(150, t(100, 200, 300)), Kind::Advisory);
    }

    #[test]
    fn classify_between_mandatory_and_hard() {
        assert_eq!(classify(250, t(100, 200, 300)), Kind::Mandatory);
    }

    // -- Kind action mapping -- ------------------------------------------

    #[test]
    fn recommended_action_for_each_kind() {
        assert_eq!(Kind::Ok.recommended_action(), "no-op");
        assert_eq!(Kind::Advisory.recommended_action(), "journal-observation");
        assert_eq!(Kind::Mandatory.recommended_action(), "trigger-archival");
        assert_eq!(Kind::Hard.recommended_action(), "halt-session-start");
    }

    #[test]
    fn kind_kebab_form_for_each_kind() {
        assert_eq!(Kind::Ok.as_kebab(), "ok");
        assert_eq!(Kind::Advisory.as_kebab(), "advisory");
        assert_eq!(Kind::Mandatory.as_kebab(), "mandatory");
        assert_eq!(Kind::Hard.as_kebab(), "hard");
    }

    #[test]
    fn kind_ordering_for_max_rollup() {
        assert!(Kind::Ok < Kind::Advisory);
        assert!(Kind::Advisory < Kind::Mandatory);
        assert!(Kind::Mandatory < Kind::Hard);
    }

    // -- measure_file_size -- --------------------------------------------

    #[test]
    fn file_size_missing_returns_zero() {
        let tmp = tempdir();
        let path = tmp.path().join("nope.json");
        let (exists, size) = measure_file_size(&path);
        assert!(!exists);
        assert_eq!(size, 0);
    }

    #[test]
    fn file_size_present_returns_byte_count() {
        let tmp = tempdir();
        let path = tmp.path().join("a.json");
        let body = b"{\"x\":1}";
        std::fs::write(&path, body).unwrap();
        let (exists, size) = measure_file_size(&path);
        assert!(exists);
        assert_eq!(size, body.len() as u64);
    }

    #[test]
    fn file_size_directory_returns_zero() {
        let tmp = tempdir();
        let (exists, size) = measure_file_size(tmp.path());
        assert!(!exists);
        assert_eq!(size, 0);
    }

    // -- measure_directory_size -- ---------------------------------------

    #[test]
    fn directory_size_missing_returns_zero() {
        let tmp = tempdir();
        let path = tmp.path().join("nope");
        let (exists, size) = measure_directory_size(&path);
        assert!(!exists);
        assert_eq!(size, 0);
    }

    #[test]
    fn directory_size_empty_returns_zero() {
        let tmp = tempdir();
        let (exists, size) = measure_directory_size(tmp.path());
        assert!(exists);
        assert_eq!(size, 0);
    }

    #[test]
    fn directory_size_recurses_and_sums() {
        let tmp = tempdir();
        let root = tmp.path();
        std::fs::create_dir_all(root.join("nested/deep")).unwrap();
        std::fs::write(root.join("a.json"), vec![0u8; 100]).unwrap();
        std::fs::write(root.join("nested/b.json"), vec![0u8; 250]).unwrap();
        std::fs::write(root.join("nested/deep/c.json"), vec![0u8; 75]).unwrap();
        let (exists, size) = measure_directory_size(root);
        assert!(exists);
        assert_eq!(size, 425);
    }

    #[test]
    fn directory_size_file_path_returns_zero() {
        let tmp = tempdir();
        let path = tmp.path().join("notdir.json");
        std::fs::write(&path, b"x").unwrap();
        let (exists, size) = measure_directory_size(&path);
        assert!(!exists);
        assert_eq!(size, 0);
    }

    // -- measure_json_array_count -- -------------------------------------

    #[test]
    fn json_array_count_missing_file_returns_zero() {
        let tmp = tempdir();
        let path = tmp.path().join("nope.json");
        let (exists, n) = measure_json_array_count(&path, "dispatches");
        assert!(!exists);
        assert_eq!(n, 0);
    }

    #[test]
    fn json_array_count_invalid_json_returns_zero() {
        let tmp = tempdir();
        let path = tmp.path().join("bad.json");
        std::fs::write(&path, b"{ not json").unwrap();
        let (exists, n) = measure_json_array_count(&path, "dispatches");
        assert!(exists);
        assert_eq!(n, 0);
    }

    #[test]
    fn json_array_count_missing_key_returns_zero() {
        let tmp = tempdir();
        let path = tmp.path().join("ok.json");
        std::fs::write(&path, b"{\"other\":[1,2]}").unwrap();
        let (exists, n) = measure_json_array_count(&path, "dispatches");
        assert!(exists);
        assert_eq!(n, 0);
    }

    #[test]
    fn json_array_count_key_not_array_returns_zero() {
        let tmp = tempdir();
        let path = tmp.path().join("ok.json");
        std::fs::write(&path, b"{\"dispatches\":\"oops\"}").unwrap();
        let (exists, n) = measure_json_array_count(&path, "dispatches");
        assert!(exists);
        assert_eq!(n, 0);
    }

    #[test]
    fn json_array_count_returns_array_length() {
        let tmp = tempdir();
        let path = tmp.path().join("ok.json");
        std::fs::write(&path, b"{\"dispatches\":[1,2,3,4,5]}").unwrap();
        let (exists, n) = measure_json_array_count(&path, "dispatches");
        assert!(exists);
        assert_eq!(n, 5);
    }

    // -- build_axis_report -- --------------------------------------------

    #[test]
    fn build_axis_report_file_size_missing() {
        let tmp = tempdir();
        let spec = AxisSpec {
            name: "test-axis",
            relative_path: "nope.json",
            spec: MeasurementSpec::FileSize,
            thresholds: t(10, 20, 30),
            unit: Unit::Bytes,
            contributes_to_byte_total: true,
        };
        let r = build_axis_report(spec, tmp.path());
        assert_eq!(r.name, "test-axis");
        assert_eq!(r.path, "nope.json");
        assert_eq!(r.measurement_kind, "file-size");
        assert!(!r.exists);
        assert_eq!(r.value, 0);
        assert_eq!(r.classification, Kind::Ok);
        assert_eq!(r.recommended_action, "no-op");
    }

    #[test]
    fn build_axis_report_classifies_hard() {
        let tmp = tempdir();
        let path = tmp.path().join("big.json");
        std::fs::write(&path, vec![0u8; 100]).unwrap();
        let spec = AxisSpec {
            name: "big",
            relative_path: "big.json",
            spec: MeasurementSpec::FileSize,
            thresholds: t(10, 20, 30),
            unit: Unit::Bytes,
            contributes_to_byte_total: true,
        };
        let r = build_axis_report(spec, tmp.path());
        assert!(r.exists);
        assert_eq!(r.value, 100);
        assert_eq!(r.classification, Kind::Hard);
        assert_eq!(r.recommended_action, "halt-session-start");
    }

    // -- build_cross_axis -- ---------------------------------------------

    #[test]
    fn cross_axis_sums_byte_axes_skips_entry_axes() {
        let tmp = tempdir();
        let specs = [
            AxisSpec {
                name: "a",
                relative_path: "a.json",
                spec: MeasurementSpec::FileSize,
                thresholds: t(MB, 4 * MB, 16 * MB),
                unit: Unit::Bytes,
                contributes_to_byte_total: true,
            },
            AxisSpec {
                name: "b",
                relative_path: "b.json",
                spec: MeasurementSpec::FileSize,
                thresholds: t(MB, 4 * MB, 16 * MB),
                unit: Unit::Bytes,
                contributes_to_byte_total: true,
            },
            AxisSpec {
                name: "c",
                relative_path: "c.json",
                spec: MeasurementSpec::JsonArrayCount("xs"),
                thresholds: t(50, 200, 500),
                unit: Unit::Entries,
                contributes_to_byte_total: false,
            },
        ];
        let axes = vec![
            AxisReport {
                name: "a",
                path: "a.json".into(),
                measurement_kind: "file-size",
                unit: Unit::Bytes,
                exists: true,
                value: 1000,
                thresholds: t(MB, 4 * MB, 16 * MB),
                classification: Kind::Ok,
                recommended_action: "no-op",
            },
            AxisReport {
                name: "b",
                path: "b.json".into(),
                measurement_kind: "file-size",
                unit: Unit::Bytes,
                exists: true,
                value: 2000,
                thresholds: t(MB, 4 * MB, 16 * MB),
                classification: Kind::Ok,
                recommended_action: "no-op",
            },
            AxisReport {
                name: "c",
                path: "c.json".into(),
                measurement_kind: "json-array-count",
                unit: Unit::Entries,
                exists: true,
                value: 12345, // not byte-contributing
                thresholds: t(50, 200, 500),
                classification: Kind::Hard,
                recommended_action: "halt-session-start",
            },
        ];
        let _ = tmp;
        let cross = build_cross_axis(&axes, &specs);
        assert_eq!(cross.total_state_bytes, 3000);
        assert_eq!(cross.classification, Kind::Ok);
    }

    #[test]
    fn cross_axis_classifies_against_total_thresholds() {
        let specs = [AxisSpec {
            name: "big",
            relative_path: "big.bin",
            spec: MeasurementSpec::FileSize,
            thresholds: t(MB, 4 * MB, 16 * MB),
            unit: Unit::Bytes,
            contributes_to_byte_total: true,
        }];
        let axes = vec![AxisReport {
            name: "big",
            path: "big.bin".into(),
            measurement_kind: "file-size",
            unit: Unit::Bytes,
            exists: true,
            value: TOTAL_ADVISORY,
            thresholds: t(MB, 4 * MB, 16 * MB),
            classification: Kind::Ok,
            recommended_action: "no-op",
        }];
        let cross = build_cross_axis(&axes, &specs);
        assert_eq!(cross.total_state_bytes, TOTAL_ADVISORY);
        assert_eq!(cross.classification, Kind::Advisory);
        assert_eq!(cross.recommended_action, "journal-observation");
    }

    // -- build_overall -- ------------------------------------------------

    #[test]
    fn overall_takes_max_kind_across_axes_and_cross() {
        let axes = vec![
            AxisReport {
                name: "a",
                path: "a.json".into(),
                measurement_kind: "file-size",
                unit: Unit::Bytes,
                exists: true,
                value: 1,
                thresholds: t(10, 20, 30),
                classification: Kind::Ok,
                recommended_action: "no-op",
            },
            AxisReport {
                name: "b",
                path: "b.json".into(),
                measurement_kind: "file-size",
                unit: Unit::Bytes,
                exists: true,
                value: 25,
                thresholds: t(10, 20, 30),
                classification: Kind::Mandatory,
                recommended_action: "trigger-archival",
            },
        ];
        let cross = CrossAxisReport {
            total_state_bytes: 26,
            thresholds: t(100, 200, 300),
            classification: Kind::Ok,
            recommended_action: "no-op",
        };
        let o = build_overall(&axes, &cross);
        assert_eq!(o.classification, Kind::Mandatory);
        assert_eq!(o.recommended_action, "trigger-archival");
    }

    #[test]
    fn overall_uses_cross_when_higher() {
        let axes = vec![AxisReport {
            name: "a",
            path: "a.json".into(),
            measurement_kind: "file-size",
            unit: Unit::Bytes,
            exists: true,
            value: 1,
            thresholds: t(10, 20, 30),
            classification: Kind::Ok,
            recommended_action: "no-op",
        }];
        let cross = CrossAxisReport {
            total_state_bytes: 5_000_000_000,
            thresholds: cross_axis_thresholds(),
            classification: Kind::Hard,
            recommended_action: "halt-session-start",
        };
        let o = build_overall(&axes, &cross);
        assert_eq!(o.classification, Kind::Hard);
    }

    // -- build_report integration -- -------------------------------------

    #[test]
    fn build_report_on_missing_state_surface_is_ok() {
        let tmp = tempdir();
        // No state/, no docs/state.json -> all axes 0 bytes, classification Ok.
        let r = build_report(tmp.path());
        assert_eq!(r.tool, "v2-state-audit");
        assert_eq!(r.policy_version, POLICY_VERSION);
        assert_eq!(r.axes.len(), 9);
        for a in &r.axes {
            assert!(!a.exists);
            assert_eq!(a.value, 0);
            assert_eq!(a.classification, Kind::Ok);
        }
        assert_eq!(r.cross_axis.total_state_bytes, 0);
        assert_eq!(r.cross_axis.classification, Kind::Ok);
        assert_eq!(r.overall.classification, Kind::Ok);
        assert_eq!(r.overall.recommended_action, "no-op");
    }

    #[test]
    fn build_report_picks_up_state_json_dispatches() {
        let tmp = tempdir();
        let docs = tmp.path().join("docs");
        std::fs::create_dir_all(&docs).unwrap();
        let entries: Vec<Value> = (0..60).map(|i| serde_json::json!({"id": i})).collect();
        // Policy axis is conceptual "dispatches"; storage key is the
        // legacy v1 `agent_sessions` array (see axis spec comment).
        let body = serde_json::json!({ "agent_sessions": entries });
        std::fs::write(docs.join("state.json"), serde_json::to_vec(&body).unwrap()).unwrap();
        let r = build_report(tmp.path());
        let axis = r.axes.iter().find(|a| a.name == "state-json-dispatches").unwrap();
        assert!(axis.exists);
        assert_eq!(axis.value, 60);
        assert_eq!(axis.classification, Kind::Advisory);
        assert_eq!(axis.unit, Unit::Entries);
        // Entry count does NOT contribute to byte total.
        assert_eq!(r.cross_axis.total_state_bytes, 0);
        // Overall rolls up to advisory because one axis is advisory.
        assert_eq!(r.overall.classification, Kind::Advisory);
    }

    #[test]
    fn dispatches_axis_classifies_hard_at_500_plus() {
        let tmp = tempdir();
        let docs = tmp.path().join("docs");
        std::fs::create_dir_all(&docs).unwrap();
        let entries: Vec<Value> = (0..500).map(|i| serde_json::json!({"id": i})).collect();
        let body = serde_json::json!({ "agent_sessions": entries });
        std::fs::write(docs.join("state.json"), serde_json::to_vec(&body).unwrap()).unwrap();
        let r = build_report(tmp.path());
        let axis = r.axes.iter().find(|a| a.name == "state-json-dispatches").unwrap();
        assert_eq!(axis.value, 500);
        assert_eq!(axis.classification, Kind::Hard);
        assert_eq!(axis.recommended_action, "halt-session-start");
        assert_eq!(r.overall.classification, Kind::Hard);
    }

    #[test]
    fn build_report_picks_up_role_history() {
        let tmp = tempdir();
        let roles = tmp.path().join("state/roles");
        std::fs::create_dir_all(&roles).unwrap();
        // Write 5 MiB to reconciler-history.json — exceeds the 4 MiB
        // mandatory threshold for role-history (1 MiB advisory).
        let mut f = std::fs::File::create(roles.join("reconciler-history.json")).unwrap();
        f.write_all(&vec![0u8; 5 * MB as usize]).unwrap();
        let r = build_report(tmp.path());
        let axis = r.axes.iter().find(|a| a.name == "reconciler-role-history").unwrap();
        assert!(axis.exists);
        assert_eq!(axis.value, 5 * MB);
        assert_eq!(axis.classification, Kind::Mandatory);
        assert_eq!(axis.recommended_action, "trigger-archival");
    }

    // -- JSON output stability -- ----------------------------------------

    #[test]
    fn json_output_has_expected_top_level_keys() {
        let tmp = tempdir();
        let r = build_report(tmp.path());
        let s = format_json(&r).unwrap();
        for key in ["tool", "version", "policy_version", "repo_root",
                    "measured_at", "axes", "cross_axis", "overall"]
        {
            assert!(
                s.contains(&format!("\"{key}\"")),
                "expected top-level key {key} in JSON output, got: {s}"
            );
        }
    }

    #[test]
    fn json_output_classification_uses_kebab_case() {
        let tmp = tempdir();
        let r = build_report(tmp.path());
        let s = format_json(&r).unwrap();
        // Default state-of-the-world is all-ok.
        assert!(s.contains("\"ok\""));
        assert!(!s.contains("\"Ok\""));
        assert!(s.contains("\"no-op\""));
    }

    // -- exit code -- ----------------------------------------------------

    // ExitCode doesn't expose an inner value, so we test the mapping
    // by Debug-formatting.
    #[test]
    fn exit_code_mapping_distinct_per_kind() {
        let codes = [
            exit_code_for(Kind::Ok),
            exit_code_for(Kind::Advisory),
            exit_code_for(Kind::Mandatory),
            exit_code_for(Kind::Hard),
        ];
        let mut formatted: Vec<String> = codes.iter().map(|c| format!("{c:?}")).collect();
        formatted.sort();
        formatted.dedup();
        assert_eq!(formatted.len(), 4, "expected 4 distinct exit codes");
    }

    // -- axis table sanity -- --------------------------------------------

    #[test]
    fn axis_specs_have_unique_names() {
        let specs = all_axes();
        let mut names: Vec<&str> = specs.iter().map(|s| s.name).collect();
        names.sort();
        names.dedup();
        assert_eq!(names.len(), specs.len(), "axis names must be unique");
    }

    #[test]
    fn axis_specs_have_unique_paths() {
        let specs = all_axes();
        let mut paths: Vec<&str> = specs.iter().map(|s| s.relative_path).collect();
        paths.sort();
        paths.dedup();
        assert_eq!(paths.len(), specs.len(), "axis paths must be unique");
    }

    #[test]
    fn axis_specs_have_monotonic_thresholds() {
        for s in all_axes().iter() {
            assert!(
                s.thresholds.advisory < s.thresholds.mandatory,
                "axis {} advisory {} >= mandatory {}",
                s.name, s.thresholds.advisory, s.thresholds.mandatory
            );
            assert!(
                s.thresholds.mandatory < s.thresholds.hard,
                "axis {} mandatory {} >= hard {}",
                s.name, s.thresholds.mandatory, s.thresholds.hard
            );
        }
    }

    #[test]
    fn cross_axis_thresholds_are_monotonic() {
        let t = cross_axis_thresholds();
        assert!(t.advisory < t.mandatory);
        assert!(t.mandatory < t.hard);
    }

    #[test]
    fn axis_specs_cover_all_policy_axes() {
        // Policy doc §4 enumerates 9 axes (channels-history, super-step,
        // 4 role histories, poll-history, cycle-history, dispatches).
        // Keep this test paired with policy revisions.
        let specs = all_axes();
        assert_eq!(specs.len(), 9);
    }

    // -- minimal in-crate tempdir -- ------------------------------------

    // Avoid pulling in `tempfile` as a dev-dep for one-shot test scaffolding.
    struct TempDir {
        path: PathBuf,
    }
    impl TempDir {
        fn path(&self) -> &Path {
            &self.path
        }
    }
    impl Drop for TempDir {
        fn drop(&mut self) {
            let _ = std::fs::remove_dir_all(&self.path);
        }
    }
    fn tempdir() -> TempDir {
        let base = std::env::temp_dir();
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0);
        let pid = std::process::id();
        let dir = base.join(format!("v2-state-audit-test-{pid}-{suffix}"));
        std::fs::create_dir_all(&dir).expect("create tempdir");
        TempDir { path: dir }
    }
}
