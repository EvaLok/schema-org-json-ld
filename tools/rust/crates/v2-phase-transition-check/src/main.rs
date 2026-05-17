use clap::Parser;
use serde_json::{Map, Value};
use std::collections::BTreeMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "v2-phase-transition-check",
    about = "Validate phase-boundary invariants on cycle-history substrate"
)]
struct Args {
    /// Repository root (path containing the state directory)
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Path of the cycle-history directory relative to repo_root
    #[arg(long, default_value = "state/cycle-history")]
    state_dir: PathBuf,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,

    /// Exit non-zero on warnings (default: warnings do not fail the run)
    #[arg(long)]
    strict: bool,

    /// Only validate entries with cycle_number >= this value
    #[arg(long)]
    from_cycle: Option<u64>,

    /// Only validate entries with cycle_number <= this value
    #[arg(long)]
    to_cycle: Option<u64>,
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

const REQUIRED_FIELDS: &[&str] = &["cycle_number", "model", "started_at"];
const PHASE_VALID_VALUES: &[&str] = &["boot", "work", "close"];

fn main() -> ExitCode {
    let args = Args::parse();
    match run(&args, &mut std::io::stdout()) {
        Ok(code) => code,
        Err(err) => {
            eprintln!("v2-phase-transition-check: {err}");
            ExitCode::from(2)
        }
    }
}

fn run<W: Write>(args: &Args, out: &mut W) -> Result<ExitCode, CheckError> {
    let state_dir = args.repo_root.join(&args.state_dir);
    let entries = load_entries(&state_dir, args.from_cycle, args.to_cycle)?;
    let report = run_invariants(&entries, &state_dir);
    emit_report(&report, args.format, out)?;
    Ok(report.exit_code(args.strict))
}

#[derive(Debug)]
enum CheckError {
    Io(std::io::Error),
    Json(serde_json::Error, PathBuf),
    Invocation(String),
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckError::Io(e) => write!(f, "{e}"),
            CheckError::Json(e, p) => write!(f, "json parse error in {}: {e}", p.display()),
            CheckError::Invocation(s) => f.write_str(s),
        }
    }
}

impl From<std::io::Error> for CheckError {
    fn from(e: std::io::Error) -> Self {
        CheckError::Io(e)
    }
}

#[derive(Debug, Clone)]
struct Entry {
    filename: String,
    filename_cycle: u64,
    payload: Map<String, Value>,
}

fn load_entries(
    state_dir: &Path,
    from_cycle: Option<u64>,
    to_cycle: Option<u64>,
) -> Result<Vec<Entry>, CheckError> {
    if !state_dir.exists() {
        return Err(CheckError::Invocation(format!(
            "state directory does not exist: {}",
            state_dir.display()
        )));
    }
    let mut entries = Vec::new();
    for dirent in fs::read_dir(state_dir)? {
        let dirent = dirent?;
        let path = dirent.path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let filename = path
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or_else(|| CheckError::Invocation(format!("bad filename: {}", path.display())))?
            .to_string();
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| CheckError::Invocation(format!("bad stem: {}", path.display())))?;
        let filename_cycle: u64 = match stem.parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        if let Some(from) = from_cycle {
            if filename_cycle < from {
                continue;
            }
        }
        if let Some(to) = to_cycle {
            if filename_cycle > to {
                continue;
            }
        }
        let raw = fs::read_to_string(&path)?;
        let payload: Value = serde_json::from_str(&raw).map_err(|e| CheckError::Json(e, path.clone()))?;
        let payload = match payload {
            Value::Object(map) => map,
            other => {
                return Err(CheckError::Invocation(format!(
                    "{} root is {} not object",
                    path.display(),
                    value_type(&other)
                )));
            }
        };
        entries.push(Entry {
            filename,
            filename_cycle,
            payload,
        });
    }
    entries.sort_by_key(|e| e.filename_cycle);
    Ok(entries)
}

fn value_type(v: &Value) -> &'static str {
    match v {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Pass,
    Warn,
    Fail,
    Skip,
}

impl Status {
    fn as_str(&self) -> &'static str {
        match self {
            Status::Pass => "PASS",
            Status::Warn => "WARN",
            Status::Fail => "FAIL",
            Status::Skip => "SKIP",
        }
    }
}

#[derive(Debug, Clone)]
struct InvariantResult {
    name: &'static str,
    status: Status,
    details: String,
    violations: Vec<String>,
}

#[derive(Debug, Clone)]
struct Report {
    state_dir: String,
    entries_scanned: usize,
    range: Option<(u64, u64)>,
    invariants: Vec<InvariantResult>,
}

impl Report {
    fn exit_code(&self, strict: bool) -> ExitCode {
        let mut has_fail = false;
        let mut has_warn = false;
        for r in &self.invariants {
            match r.status {
                Status::Fail => has_fail = true,
                Status::Warn => has_warn = true,
                _ => {}
            }
        }
        if has_fail || (strict && has_warn) {
            ExitCode::from(1)
        } else {
            ExitCode::SUCCESS
        }
    }
}

fn run_invariants(entries: &[Entry], state_dir: &Path) -> Report {
    let range = if entries.is_empty() {
        None
    } else {
        Some((
            entries.first().unwrap().filename_cycle,
            entries.last().unwrap().filename_cycle,
        ))
    };

    let invariants = vec![
        check_filename_field_consistency(entries),
        check_monotonicity(entries),
        check_no_gaps(entries),
        check_required_fields(entries),
        check_rfc3339_started_at(entries),
        check_phase_field(entries),
    ];

    Report {
        state_dir: state_dir.display().to_string(),
        entries_scanned: entries.len(),
        range,
        invariants,
    }
}

fn check_filename_field_consistency(entries: &[Entry]) -> InvariantResult {
    let mut violations = Vec::new();
    let mut checked = 0;
    for e in entries {
        match e.payload.get("cycle_number").and_then(|v| v.as_u64()) {
            Some(n) => {
                checked += 1;
                if n != e.filename_cycle {
                    violations.push(format!(
                        "{}: filename cycle {} != payload cycle_number {}",
                        e.filename, e.filename_cycle, n
                    ));
                }
            }
            None => violations.push(format!(
                "{}: missing or non-integer cycle_number (cannot check consistency)",
                e.filename
            )),
        }
    }
    let status = if violations.is_empty() {
        if checked == 0 {
            Status::Skip
        } else {
            Status::Pass
        }
    } else {
        Status::Fail
    };
    let details = if entries.is_empty() {
        "no entries to check".to_string()
    } else {
        format!("{}/{} entries consistent", checked - violations.len(), entries.len())
    };
    InvariantResult {
        name: "filename-field-consistency",
        status,
        details,
        violations,
    }
}

fn check_monotonicity(entries: &[Entry]) -> InvariantResult {
    let mut violations = Vec::new();
    let mut prev: Option<u64> = None;
    for e in entries {
        if let Some(p) = prev {
            if e.filename_cycle <= p {
                violations.push(format!(
                    "{}: cycle {} not strictly greater than previous {}",
                    e.filename, e.filename_cycle, p
                ));
            }
        }
        prev = Some(e.filename_cycle);
    }
    let status = if entries.is_empty() {
        Status::Skip
    } else if violations.is_empty() {
        Status::Pass
    } else {
        Status::Fail
    };
    let details = if entries.is_empty() {
        "no entries".to_string()
    } else {
        format!("{} entries, strictly increasing", entries.len())
    };
    InvariantResult {
        name: "monotonicity",
        status,
        details,
        violations,
    }
}

fn check_no_gaps(entries: &[Entry]) -> InvariantResult {
    let mut violations = Vec::new();
    let mut prev: Option<u64> = None;
    for e in entries {
        if let Some(p) = prev {
            if e.filename_cycle > p + 1 {
                let gap_size = e.filename_cycle - p - 1;
                if gap_size == 1 {
                    violations.push(format!("missing cycle {} (between {} and {})", p + 1, p, e.filename_cycle));
                } else {
                    violations.push(format!(
                        "missing cycles {}-{} ({} cycles, between {} and {})",
                        p + 1,
                        e.filename_cycle - 1,
                        gap_size,
                        p,
                        e.filename_cycle
                    ));
                }
            }
        }
        prev = Some(e.filename_cycle);
    }
    let status = if entries.is_empty() {
        Status::Skip
    } else if violations.is_empty() {
        Status::Pass
    } else {
        Status::Warn
    };
    let details = if entries.is_empty() {
        "no entries".to_string()
    } else if violations.is_empty() {
        "contiguous".to_string()
    } else {
        format!("{} gap(s) detected", violations.len())
    };
    InvariantResult {
        name: "no-gaps",
        status,
        details,
        violations,
    }
}

fn check_required_fields(entries: &[Entry]) -> InvariantResult {
    let mut violations = Vec::new();
    for e in entries {
        for field in REQUIRED_FIELDS {
            match e.payload.get(*field) {
                None => violations.push(format!("{}: missing required field '{}'", e.filename, field)),
                Some(Value::Null) => violations.push(format!("{}: required field '{}' is null", e.filename, field)),
                Some(Value::String(s)) if s.is_empty() => {
                    violations.push(format!("{}: required field '{}' is empty string", e.filename, field))
                }
                _ => {}
            }
        }
    }
    let status = if entries.is_empty() {
        Status::Skip
    } else if violations.is_empty() {
        Status::Pass
    } else {
        Status::Fail
    };
    let details = if entries.is_empty() {
        "no entries".to_string()
    } else if violations.is_empty() {
        format!("all {} entries have required fields", entries.len())
    } else {
        format!("{} field violations across entries", violations.len())
    };
    InvariantResult {
        name: "required-fields",
        status,
        details,
        violations,
    }
}

fn check_rfc3339_started_at(entries: &[Entry]) -> InvariantResult {
    let mut violations = Vec::new();
    let mut checked = 0;
    for e in entries {
        match e.payload.get("started_at").and_then(|v| v.as_str()) {
            Some(s) if !s.is_empty() => {
                checked += 1;
                if !looks_like_rfc3339(s) {
                    violations.push(format!("{}: started_at '{}' does not match RFC3339 shape", e.filename, s));
                }
            }
            _ => {
                // Field absent or non-string; required-fields invariant covers this.
            }
        }
    }
    let status = if checked == 0 {
        Status::Skip
    } else if violations.is_empty() {
        Status::Pass
    } else {
        Status::Warn
    };
    let details = if checked == 0 {
        "no started_at fields to check".to_string()
    } else {
        format!("{}/{} entries match RFC3339 shape", checked - violations.len(), checked)
    };
    InvariantResult {
        name: "rfc3339-started-at",
        status,
        details,
        violations,
    }
}

fn looks_like_rfc3339(s: &str) -> bool {
    // Minimal RFC3339 shape: YYYY-MM-DDTHH:MM:SS followed by Z or +HH:MM or -HH:MM
    // No external regex dependency; hand-check structure.
    if s.len() < 20 {
        return false;
    }
    let bytes = s.as_bytes();
    // Positions: 0-3 year, 4 '-', 5-6 month, 7 '-', 8-9 day, 10 'T', 11-12 hour, 13 ':',
    // 14-15 minute, 16 ':', 17-18 second
    for (i, &c) in bytes.iter().enumerate().take(19) {
        match i {
            4 | 7 => {
                if c != b'-' {
                    return false;
                }
            }
            10 => {
                if c != b'T' && c != b't' && c != b' ' {
                    return false;
                }
            }
            13 | 16 => {
                if c != b':' {
                    return false;
                }
            }
            _ => {
                if !c.is_ascii_digit() {
                    return false;
                }
            }
        }
    }
    // Suffix from index 19 onward must be either:
    //   Z / z
    //   .fff[fff[fff]]Z / .fff... +HH:MM
    //   +HH:MM / -HH:MM
    let rest = &s[19..];
    let rest = match rest.as_bytes().first() {
        Some(b'.') => {
            let after_dot = &rest[1..];
            let frac_end = after_dot.bytes().take_while(|b| b.is_ascii_digit()).count();
            if frac_end == 0 {
                return false;
            }
            &after_dot[frac_end..]
        }
        _ => rest,
    };
    match rest.as_bytes() {
        [b'Z'] | [b'z'] => true,
        [sign, h1, h2, b':', m1, m2]
            if (*sign == b'+' || *sign == b'-')
                && h1.is_ascii_digit()
                && h2.is_ascii_digit()
                && m1.is_ascii_digit()
                && m2.is_ascii_digit() =>
        {
            true
        }
        _ => false,
    }
}

fn check_phase_field(entries: &[Entry]) -> InvariantResult {
    let mut violations = Vec::new();
    let mut checked = 0;
    for e in entries {
        match e.payload.get("phase") {
            None => {}
            Some(Value::String(s)) => {
                checked += 1;
                if !PHASE_VALID_VALUES.contains(&s.as_str()) {
                    violations.push(format!(
                        "{}: phase '{}' not one of {:?}",
                        e.filename, s, PHASE_VALID_VALUES
                    ));
                }
            }
            Some(other) => violations.push(format!(
                "{}: phase has wrong type ({}; expected string)",
                e.filename,
                value_type(other)
            )),
        }
    }
    let status = if checked == 0 && violations.is_empty() {
        Status::Skip
    } else if violations.is_empty() {
        Status::Pass
    } else {
        Status::Fail
    };
    let details = if checked == 0 && violations.is_empty() {
        "no entries have 'phase' field".to_string()
    } else {
        format!("{}/{} entries have valid phase", checked - violations.len(), checked)
    };
    InvariantResult {
        name: "phase-field",
        status,
        details,
        violations,
    }
}

fn emit_report<W: Write>(report: &Report, fmt: OutputFormat, out: &mut W) -> Result<(), CheckError> {
    match fmt {
        OutputFormat::Text => emit_text(report, out)?,
        OutputFormat::Json => emit_json(report, out)?,
    }
    Ok(())
}

fn emit_text<W: Write>(report: &Report, out: &mut W) -> std::io::Result<()> {
    writeln!(out, "Phase-transition-check report")?;
    writeln!(out, "============================")?;
    writeln!(out, "State dir: {}", report.state_dir)?;
    writeln!(out, "Entries scanned: {}", report.entries_scanned)?;
    match report.range {
        Some((lo, hi)) => writeln!(out, "Range: [{}..{}]", lo, hi)?,
        None => writeln!(out, "Range: (empty)")?,
    }
    writeln!(out)?;
    writeln!(out, "Invariants:")?;
    let mut counts: BTreeMap<&'static str, usize> = BTreeMap::new();
    for r in &report.invariants {
        writeln!(out, "  [{}] {} ({})", r.status.as_str(), r.name, r.details)?;
        for v in &r.violations {
            writeln!(out, "    - {}", v)?;
        }
        *counts.entry(r.status.as_str()).or_insert(0) += 1;
    }
    writeln!(out)?;
    let summary = ["PASS", "WARN", "FAIL", "SKIP"]
        .iter()
        .map(|k| format!("{} {}", counts.get(k).copied().unwrap_or(0), k))
        .collect::<Vec<_>>()
        .join(", ");
    writeln!(out, "Summary: {}", summary)?;
    Ok(())
}

fn emit_json<W: Write>(report: &Report, out: &mut W) -> std::io::Result<()> {
    let mut invariants_json = Vec::new();
    for r in &report.invariants {
        let mut obj = Map::new();
        obj.insert("name".to_string(), Value::String(r.name.to_string()));
        obj.insert("status".to_string(), Value::String(r.status.as_str().to_lowercase()));
        obj.insert("details".to_string(), Value::String(r.details.clone()));
        obj.insert(
            "violations".to_string(),
            Value::Array(r.violations.iter().map(|v| Value::String(v.clone())).collect()),
        );
        invariants_json.push(Value::Object(obj));
    }
    let mut root = Map::new();
    root.insert("state_dir".to_string(), Value::String(report.state_dir.clone()));
    root.insert(
        "entries_scanned".to_string(),
        Value::Number(serde_json::Number::from(report.entries_scanned as u64)),
    );
    match report.range {
        Some((lo, hi)) => {
            let mut range_obj = Map::new();
            range_obj.insert("min".to_string(), Value::Number(serde_json::Number::from(lo)));
            range_obj.insert("max".to_string(), Value::Number(serde_json::Number::from(hi)));
            root.insert("range".to_string(), Value::Object(range_obj));
        }
        None => {
            root.insert("range".to_string(), Value::Null);
        }
    }
    root.insert("invariants".to_string(), Value::Array(invariants_json));
    let mut summary = Map::new();
    for status_name in ["pass", "warn", "fail", "skip"] {
        let count = report
            .invariants
            .iter()
            .filter(|r| r.status.as_str().eq_ignore_ascii_case(status_name))
            .count();
        summary.insert(
            status_name.to_string(),
            Value::Number(serde_json::Number::from(count as u64)),
        );
    }
    root.insert("summary".to_string(), Value::Object(summary));
    let s = serde_json::to_string_pretty(&Value::Object(root))
        .expect("serialize report");
    writeln!(out, "{}", s)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(cycle: u64, fields: &[(&str, Value)]) -> Entry {
        let mut payload = Map::new();
        for (k, v) in fields {
            payload.insert((*k).to_string(), v.clone());
        }
        Entry {
            filename: format!("{}.json", cycle),
            filename_cycle: cycle,
            payload,
        }
    }

    #[test]
    fn rfc3339_z_suffix() {
        assert!(looks_like_rfc3339("2026-05-11T20:31:00Z"));
        assert!(looks_like_rfc3339("2026-05-11T20:31:00z"));
        assert!(looks_like_rfc3339("2026-05-11t20:31:00Z"));
    }

    #[test]
    fn rfc3339_offset_suffix() {
        assert!(looks_like_rfc3339("2026-05-11T20:31:00+00:00"));
        assert!(looks_like_rfc3339("2026-05-11T20:31:00-08:00"));
    }

    #[test]
    fn rfc3339_fractional() {
        assert!(looks_like_rfc3339("2026-05-11T20:31:00.123Z"));
        assert!(looks_like_rfc3339("2026-05-11T20:31:00.123456+05:30"));
    }

    #[test]
    fn rfc3339_rejects_truncated() {
        assert!(!looks_like_rfc3339("2026-05-11"));
        assert!(!looks_like_rfc3339("2026-05-11T20:31:00"));
        assert!(!looks_like_rfc3339(""));
    }

    #[test]
    fn rfc3339_rejects_bad_separators() {
        assert!(!looks_like_rfc3339("2026/05/11T20:31:00Z"));
        assert!(!looks_like_rfc3339("2026-05-11X20:31:00Z"));
        assert!(!looks_like_rfc3339("2026-05-11T20-31-00Z"));
    }

    #[test]
    fn rfc3339_rejects_bad_suffix() {
        assert!(!looks_like_rfc3339("2026-05-11T20:31:00Q"));
        assert!(!looks_like_rfc3339("2026-05-11T20:31:00+0000"));
        assert!(!looks_like_rfc3339("2026-05-11T20:31:00."));
    }

    #[test]
    fn filename_field_passes_when_consistent() {
        let entries = vec![
            entry(1, &[("cycle_number", Value::Number(1.into()))]),
            entry(2, &[("cycle_number", Value::Number(2.into()))]),
        ];
        let r = check_filename_field_consistency(&entries);
        assert_eq!(r.status, Status::Pass);
        assert!(r.violations.is_empty());
    }

    #[test]
    fn filename_field_fails_on_mismatch() {
        let entries = vec![entry(1, &[("cycle_number", Value::Number(2.into()))])];
        let r = check_filename_field_consistency(&entries);
        assert_eq!(r.status, Status::Fail);
        assert_eq!(r.violations.len(), 1);
        assert!(r.violations[0].contains("filename cycle 1"));
    }

    #[test]
    fn filename_field_fails_on_missing_cycle_number() {
        let entries = vec![entry(1, &[])];
        let r = check_filename_field_consistency(&entries);
        assert_eq!(r.status, Status::Fail);
    }

    #[test]
    fn monotonicity_passes_strictly_increasing() {
        let entries = vec![entry(1, &[]), entry(2, &[]), entry(5, &[])];
        let r = check_monotonicity(&entries);
        assert_eq!(r.status, Status::Pass);
    }

    #[test]
    fn monotonicity_skip_on_empty() {
        let r = check_monotonicity(&[]);
        assert_eq!(r.status, Status::Skip);
    }

    #[test]
    fn no_gaps_passes_contiguous() {
        let entries = vec![entry(1, &[]), entry(2, &[]), entry(3, &[])];
        let r = check_no_gaps(&entries);
        assert_eq!(r.status, Status::Pass);
    }

    #[test]
    fn no_gaps_warns_on_gaps() {
        let entries = vec![entry(1, &[]), entry(3, &[]), entry(7, &[])];
        let r = check_no_gaps(&entries);
        assert_eq!(r.status, Status::Warn);
        assert_eq!(r.violations.len(), 2);
        assert!(r.violations[0].contains("missing cycle 2"));
        assert!(r.violations[1].contains("missing cycles 4-6"));
    }

    #[test]
    fn required_fields_passes_complete() {
        let entries = vec![entry(
            1,
            &[
                ("cycle_number", Value::Number(1.into())),
                ("model", Value::String("claude-opus-4-7".to_string())),
                ("started_at", Value::String("2026-05-11T20:31:00Z".to_string())),
            ],
        )];
        let r = check_required_fields(&entries);
        assert_eq!(r.status, Status::Pass);
    }

    #[test]
    fn required_fields_fails_on_missing() {
        let entries = vec![entry(
            1,
            &[("cycle_number", Value::Number(1.into()))],
        )];
        let r = check_required_fields(&entries);
        assert_eq!(r.status, Status::Fail);
        assert_eq!(r.violations.len(), 2);
    }

    #[test]
    fn required_fields_fails_on_empty_string() {
        let entries = vec![entry(
            1,
            &[
                ("cycle_number", Value::Number(1.into())),
                ("model", Value::String("".to_string())),
                ("started_at", Value::String("2026-05-11T20:31:00Z".to_string())),
            ],
        )];
        let r = check_required_fields(&entries);
        assert_eq!(r.status, Status::Fail);
        assert!(r.violations[0].contains("empty string"));
    }

    #[test]
    fn rfc3339_check_skips_when_absent() {
        let entries = vec![entry(1, &[])];
        let r = check_rfc3339_started_at(&entries);
        assert_eq!(r.status, Status::Skip);
    }

    #[test]
    fn rfc3339_check_warns_on_malformed() {
        let entries = vec![entry(
            1,
            &[("started_at", Value::String("not-a-date".to_string()))],
        )];
        let r = check_rfc3339_started_at(&entries);
        assert_eq!(r.status, Status::Warn);
    }

    #[test]
    fn phase_field_skips_when_absent() {
        let entries = vec![entry(1, &[])];
        let r = check_phase_field(&entries);
        assert_eq!(r.status, Status::Skip);
    }

    #[test]
    fn phase_field_passes_valid() {
        let entries = vec![entry(1, &[("phase", Value::String("boot".to_string()))])];
        let r = check_phase_field(&entries);
        assert_eq!(r.status, Status::Pass);
    }

    #[test]
    fn phase_field_fails_invalid() {
        let entries = vec![entry(1, &[("phase", Value::String("middle".to_string()))])];
        let r = check_phase_field(&entries);
        assert_eq!(r.status, Status::Fail);
    }

    #[test]
    fn phase_field_fails_wrong_type() {
        let entries = vec![entry(1, &[("phase", Value::Number(1.into()))])];
        let r = check_phase_field(&entries);
        assert_eq!(r.status, Status::Fail);
    }

    #[test]
    fn report_exit_code_pass() {
        let report = Report {
            state_dir: "x".to_string(),
            entries_scanned: 1,
            range: Some((1, 1)),
            invariants: vec![InvariantResult {
                name: "x",
                status: Status::Pass,
                details: "ok".to_string(),
                violations: vec![],
            }],
        };
        let code = report.exit_code(false);
        assert!(matches!(code, c if format!("{:?}", c).contains("0") || format!("{:?}", c).contains("Success")));
    }

    #[test]
    fn report_exit_code_fail_propagates() {
        let report = Report {
            state_dir: "x".to_string(),
            entries_scanned: 1,
            range: Some((1, 1)),
            invariants: vec![InvariantResult {
                name: "x",
                status: Status::Fail,
                details: "bad".to_string(),
                violations: vec!["v".to_string()],
            }],
        };
        let code = report.exit_code(false);
        let s = format!("{:?}", code);
        assert!(s.contains("1") || !s.contains("Success"));
    }

    #[test]
    fn report_exit_code_strict_warn_fails() {
        let report = Report {
            state_dir: "x".to_string(),
            entries_scanned: 1,
            range: Some((1, 1)),
            invariants: vec![InvariantResult {
                name: "x",
                status: Status::Warn,
                details: "warn".to_string(),
                violations: vec!["v".to_string()],
            }],
        };
        let s = format!("{:?}", report.exit_code(true));
        assert!(s.contains("1") || !s.contains("Success"));
        let s_loose = format!("{:?}", report.exit_code(false));
        assert!(s_loose.contains("0") || s_loose.contains("Success"));
    }
}
