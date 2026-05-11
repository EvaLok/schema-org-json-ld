use clap::Parser;
use serde::Serialize;
use serde_json::{Map, Value};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "v2-boot-phase",
    about = "Orchestration-hub for v2 boot-phase: load cycle-history, compute cursor, emit cycle-context (scaffold; standing-directive-check + gardening-sweep deferred to cycle 124+)"
)]
struct Args {
    /// Repository root (path containing the state directory)
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Path of the cycle-history directory relative to repo_root
    #[arg(long, default_value = "state/cycle-history")]
    state_dir: PathBuf,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    format: OutputFormat,

    /// Write output to a file (default: stdout, denoted as `-`)
    #[arg(long, default_value = "-")]
    output: String,

    /// Strict mode — promote warnings (gaps, deferred stages) to non-zero exit
    #[arg(long)]
    strict: bool,
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Copy, Clone, Debug, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum StageStatus {
    Done,
    Warn,
    Deferred,
    Failed,
    Skipped,
}

#[derive(Debug, Serialize)]
struct StageResult {
    name: String,
    status: StageStatus,
    details: String,
}

#[derive(Debug, Serialize, Clone)]
struct CycleEntry {
    cycle_number: u64,
    model: Option<String>,
    started_at: Option<String>,
    raw: Map<String, Value>,
}

#[derive(Debug, Serialize)]
struct Gap {
    after_cycle: u64,
    before_cycle: u64,
    missing_count: u64,
}

#[derive(Debug, Serialize)]
struct CycleContext {
    current_cycle: Option<u64>,
    previous_cycle: Option<u64>,
    previous_cycle_summary: Option<PreviousCycleSummary>,
    cycles_loaded: u64,
    gaps: Vec<Gap>,
    standing_directives: StandingDirectives,
    gardening_candidates: GardeningCandidates,
}

#[derive(Debug, Serialize)]
struct PreviousCycleSummary {
    cycle_number: u64,
    model: Option<String>,
    started_at: Option<String>,
}

#[derive(Debug, Serialize)]
struct StandingDirectives {
    implemented: bool,
    note: String,
    items: Vec<Value>,
}

#[derive(Debug, Serialize)]
struct GardeningCandidates {
    implemented: bool,
    note: String,
    items: Vec<Value>,
}

#[derive(Debug, Serialize)]
struct BootPhaseReport {
    stages: Vec<StageResult>,
    cycle_context: CycleContext,
    summary: ReportSummary,
}

#[derive(Debug, Serialize)]
struct ReportSummary {
    done: u64,
    warn: u64,
    deferred: u64,
    failed: u64,
    skipped: u64,
    strict: bool,
    exit_code: u8,
}

#[derive(Debug)]
enum BootError {
    Io(io::Error),
    Json(serde_json::Error),
}

impl std::fmt::Display for BootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BootError::Io(e) => write!(f, "{e}"),
            BootError::Json(e) => write!(f, "json error: {e}"),
        }
    }
}

impl From<io::Error> for BootError {
    fn from(e: io::Error) -> Self {
        BootError::Io(e)
    }
}

impl From<serde_json::Error> for BootError {
    fn from(e: serde_json::Error) -> Self {
        BootError::Json(e)
    }
}

fn main() -> ExitCode {
    let args = Args::parse();
    match run(&args) {
        Ok(code) => ExitCode::from(code),
        Err(err) => {
            eprintln!("v2-boot-phase: {err}");
            ExitCode::from(2)
        }
    }
}

fn run(args: &Args) -> Result<u8, BootError> {
    let state_path = args.repo_root.join(&args.state_dir);
    let mut stages: Vec<StageResult> = Vec::new();

    let (load_stage, entries) = load_cycle_history(&state_path);
    stages.push(load_stage);

    let (cursor_stage, current_cycle, previous_cycle, previous_summary) =
        compute_cursor(&entries);
    stages.push(cursor_stage);

    let (gap_stage, gaps) = detect_gaps(&entries);
    stages.push(gap_stage);

    let (directives_stage, standing_directives) = check_standing_directives();
    stages.push(directives_stage);

    let (gardening_stage, gardening_candidates) = identify_gardening_candidates();
    stages.push(gardening_stage);

    let cycle_context = CycleContext {
        current_cycle,
        previous_cycle,
        previous_cycle_summary: previous_summary,
        cycles_loaded: entries.len() as u64,
        gaps,
        standing_directives,
        gardening_candidates,
    };

    stages.push(StageResult {
        name: "assemble-cycle-context".into(),
        status: StageStatus::Done,
        details: format!(
            "cycle_context assembled (current_cycle={:?}, cycles_loaded={})",
            cycle_context.current_cycle, cycle_context.cycles_loaded
        ),
    });

    let summary = summarize(&stages, args.strict);
    let report = BootPhaseReport {
        stages,
        cycle_context,
        summary,
    };

    emit(&report, args)?;
    Ok(report.summary.exit_code)
}

fn load_cycle_history(state_path: &Path) -> (StageResult, Vec<CycleEntry>) {
    if !state_path.exists() {
        return (
            StageResult {
                name: "load-cycle-history".into(),
                status: StageStatus::Warn,
                details: format!(
                    "state directory not found: {} (treating as empty)",
                    state_path.display()
                ),
            },
            Vec::new(),
        );
    }

    if !state_path.is_dir() {
        return (
            StageResult {
                name: "load-cycle-history".into(),
                status: StageStatus::Failed,
                details: format!("state path is not a directory: {}", state_path.display()),
            },
            Vec::new(),
        );
    }

    let read_dir = match fs::read_dir(state_path) {
        Ok(d) => d,
        Err(e) => {
            return (
                StageResult {
                    name: "load-cycle-history".into(),
                    status: StageStatus::Failed,
                    details: format!(
                        "failed to read state directory {}: {e}",
                        state_path.display()
                    ),
                },
                Vec::new(),
            );
        }
    };

    let mut entries: Vec<CycleEntry> = Vec::new();
    let mut malformed: Vec<String> = Vec::new();
    for dirent in read_dir.flatten() {
        let path = dirent.path();
        if !path.is_file() {
            continue;
        }
        let stem = match path.file_stem().and_then(|s| s.to_str()) {
            Some(s) => s,
            None => continue,
        };
        let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("");
        if ext != "json" {
            continue;
        }
        if stem.parse::<u64>().is_err() {
            continue;
        }
        match read_entry(&path) {
            Ok(e) => entries.push(e),
            Err(msg) => malformed.push(format!("{}: {msg}", path.display())),
        }
    }
    entries.sort_by_key(|e| e.cycle_number);

    let status = if !malformed.is_empty() {
        StageStatus::Warn
    } else {
        StageStatus::Done
    };
    let mut details = format!(
        "loaded {} cycle-history entries from {}",
        entries.len(),
        state_path.display()
    );
    if !malformed.is_empty() {
        details.push_str(&format!(
            "; {} malformed entry/entries skipped: {}",
            malformed.len(),
            malformed.join(", ")
        ));
    }

    (
        StageResult {
            name: "load-cycle-history".into(),
            status,
            details,
        },
        entries,
    )
}

fn read_entry(path: &Path) -> Result<CycleEntry, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    let obj = value
        .as_object()
        .ok_or_else(|| "root is not a JSON object".to_string())?;
    let cycle_number = obj
        .get("cycle_number")
        .and_then(|v| v.as_u64())
        .ok_or_else(|| "missing or non-integer cycle_number".to_string())?;
    let model = obj
        .get("model")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let started_at = obj
        .get("started_at")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    Ok(CycleEntry {
        cycle_number,
        model,
        started_at,
        raw: obj.clone(),
    })
}

fn compute_cursor(
    entries: &[CycleEntry],
) -> (StageResult, Option<u64>, Option<u64>, Option<PreviousCycleSummary>) {
    if entries.is_empty() {
        return (
            StageResult {
                name: "compute-cursor".into(),
                status: StageStatus::Skipped,
                details: "no cycle-history entries; cursor undefined (first cycle)".into(),
            },
            None,
            None,
            None,
        );
    }

    let last = entries.last().unwrap();
    let previous_cycle = Some(last.cycle_number);
    let current_cycle = Some(last.cycle_number + 1);
    let summary = PreviousCycleSummary {
        cycle_number: last.cycle_number,
        model: last.model.clone(),
        started_at: last.started_at.clone(),
    };

    (
        StageResult {
            name: "compute-cursor".into(),
            status: StageStatus::Done,
            details: format!(
                "previous_cycle={} current_cycle={}",
                last.cycle_number,
                last.cycle_number + 1
            ),
        },
        current_cycle,
        previous_cycle,
        Some(summary),
    )
}

fn detect_gaps(entries: &[CycleEntry]) -> (StageResult, Vec<Gap>) {
    if entries.len() < 2 {
        return (
            StageResult {
                name: "detect-gaps".into(),
                status: StageStatus::Skipped,
                details: "fewer than 2 entries; gap detection N/A".into(),
            },
            Vec::new(),
        );
    }

    let mut gaps: Vec<Gap> = Vec::new();
    for pair in entries.windows(2) {
        let a = pair[0].cycle_number;
        let b = pair[1].cycle_number;
        if b > a + 1 {
            gaps.push(Gap {
                after_cycle: a,
                before_cycle: b,
                missing_count: b - a - 1,
            });
        }
    }

    if gaps.is_empty() {
        (
            StageResult {
                name: "detect-gaps".into(),
                status: StageStatus::Done,
                details: format!("no gaps across {} entries", entries.len()),
            },
            gaps,
        )
    } else {
        let total_missing: u64 = gaps.iter().map(|g| g.missing_count).sum();
        (
            StageResult {
                name: "detect-gaps".into(),
                status: StageStatus::Warn,
                details: format!(
                    "{} gap(s) totaling {} missing cycle(s)",
                    gaps.len(),
                    total_missing
                ),
            },
            gaps,
        )
    }
}

fn check_standing_directives() -> (StageResult, StandingDirectives) {
    let note = "DEFERRED: standing-directive-check requires GitHub API integration (gh CLI shell-out or octocrab); scaffold returns empty items list. See cycle 122 _notes for 2-cycle split rationale; full implementation planned cycle 124+.".to_string();
    (
        StageResult {
            name: "check-standing-directives".into(),
            status: StageStatus::Deferred,
            details: note.clone(),
        },
        StandingDirectives {
            implemented: false,
            note,
            items: Vec::new(),
        },
    )
}

fn identify_gardening_candidates() -> (StageResult, GardeningCandidates) {
    let note = "DEFERRED: gardening-sweep-pre-cycle requires GitHub API integration (gh CLI shell-out) + HOUSEKEEPING-discipline heuristics; scaffold returns empty items list. See cycle 122 _notes for 2-cycle split rationale; full implementation planned cycle 124+.".to_string();
    (
        StageResult {
            name: "identify-gardening-candidates".into(),
            status: StageStatus::Deferred,
            details: note.clone(),
        },
        GardeningCandidates {
            implemented: false,
            note,
            items: Vec::new(),
        },
    )
}

fn summarize(stages: &[StageResult], strict: bool) -> ReportSummary {
    let mut done = 0u64;
    let mut warn = 0u64;
    let mut deferred = 0u64;
    let mut failed = 0u64;
    let mut skipped = 0u64;
    for stage in stages {
        match stage.status {
            StageStatus::Done => done += 1,
            StageStatus::Warn => warn += 1,
            StageStatus::Deferred => deferred += 1,
            StageStatus::Failed => failed += 1,
            StageStatus::Skipped => skipped += 1,
        }
    }
    let exit_code = if failed > 0 {
        1
    } else if strict && (warn > 0 || deferred > 0) {
        1
    } else {
        0
    };
    ReportSummary {
        done,
        warn,
        deferred,
        failed,
        skipped,
        strict,
        exit_code,
    }
}

fn emit(report: &BootPhaseReport, args: &Args) -> Result<(), BootError> {
    let rendered = match args.format {
        OutputFormat::Text => render_text(report),
        OutputFormat::Json => render_json(report)?,
    };
    if args.output == "-" {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(rendered.as_bytes())?;
        if !rendered.ends_with('\n') {
            handle.write_all(b"\n")?;
        }
    } else {
        let out_path = PathBuf::from(&args.output);
        if let Some(parent) = out_path.parent() {
            if !parent.as_os_str().is_empty() && !parent.exists() {
                fs::create_dir_all(parent)?;
            }
        }
        let mut content = rendered;
        if !content.ends_with('\n') {
            content.push('\n');
        }
        fs::write(&out_path, content)?;
    }
    Ok(())
}

fn render_text(report: &BootPhaseReport) -> String {
    let mut s = String::new();
    s.push_str("v2-boot-phase report\n");
    s.push_str("====================\n\n");
    s.push_str("Stages:\n");
    for stage in &report.stages {
        s.push_str(&format!(
            "  - {} [{}]: {}\n",
            stage.name,
            status_label(stage.status),
            stage.details
        ));
    }
    s.push('\n');
    s.push_str("Cycle context:\n");
    s.push_str(&format!(
        "  current_cycle:  {}\n",
        report
            .cycle_context
            .current_cycle
            .map(|n| n.to_string())
            .unwrap_or_else(|| "(none)".into())
    ));
    s.push_str(&format!(
        "  previous_cycle: {}\n",
        report
            .cycle_context
            .previous_cycle
            .map(|n| n.to_string())
            .unwrap_or_else(|| "(none)".into())
    ));
    s.push_str(&format!(
        "  cycles_loaded:  {}\n",
        report.cycle_context.cycles_loaded
    ));
    s.push_str(&format!(
        "  gaps:           {}\n",
        report.cycle_context.gaps.len()
    ));
    if !report.cycle_context.gaps.is_empty() {
        for g in &report.cycle_context.gaps {
            s.push_str(&format!(
                "    - missing cycles {}..{} ({} cycle(s))\n",
                g.after_cycle + 1,
                g.before_cycle - 1,
                g.missing_count
            ));
        }
    }
    s.push_str(&format!(
        "  standing_directives: deferred={} (cycle 124+)\n",
        !report.cycle_context.standing_directives.implemented
    ));
    s.push_str(&format!(
        "  gardening_candidates: deferred={} (cycle 124+)\n",
        !report.cycle_context.gardening_candidates.implemented
    ));
    s.push('\n');
    s.push_str("Summary:\n");
    let sm = &report.summary;
    s.push_str(&format!(
        "  done={} warn={} deferred={} failed={} skipped={}\n",
        sm.done, sm.warn, sm.deferred, sm.failed, sm.skipped
    ));
    s.push_str(&format!(
        "  strict={} exit_code={}\n",
        sm.strict, sm.exit_code
    ));
    s
}

fn render_json(report: &BootPhaseReport) -> Result<String, BootError> {
    Ok(serde_json::to_string_pretty(report)?)
}

fn status_label(s: StageStatus) -> &'static str {
    match s {
        StageStatus::Done => "DONE",
        StageStatus::Warn => "WARN",
        StageStatus::Deferred => "DEFERRED",
        StageStatus::Failed => "FAILED",
        StageStatus::Skipped => "SKIPPED",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(n: u64) -> CycleEntry {
        let mut raw = Map::new();
        raw.insert("cycle_number".into(), Value::Number(n.into()));
        raw.insert("model".into(), Value::String("test-model".into()));
        raw.insert(
            "started_at".into(),
            Value::String("2026-05-11T22:30:00Z".into()),
        );
        CycleEntry {
            cycle_number: n,
            model: Some("test-model".into()),
            started_at: Some("2026-05-11T22:30:00Z".into()),
            raw,
        }
    }

    #[test]
    fn compute_cursor_empty_yields_skipped() {
        let (stage, cur, prev, summary) = compute_cursor(&[]);
        assert_eq!(stage.status, StageStatus::Skipped);
        assert!(cur.is_none() && prev.is_none() && summary.is_none());
    }

    #[test]
    fn compute_cursor_advances_by_one() {
        let entries = vec![entry(120), entry(121), entry(122)];
        let (stage, cur, prev, summary) = compute_cursor(&entries);
        assert_eq!(stage.status, StageStatus::Done);
        assert_eq!(cur, Some(123));
        assert_eq!(prev, Some(122));
        let s = summary.unwrap();
        assert_eq!(s.cycle_number, 122);
        assert_eq!(s.model.as_deref(), Some("test-model"));
    }

    #[test]
    fn detect_gaps_under_two_entries_skipped() {
        let (stage, gaps) = detect_gaps(&[entry(1)]);
        assert_eq!(stage.status, StageStatus::Skipped);
        assert!(gaps.is_empty());
    }

    #[test]
    fn detect_gaps_contiguous_done_no_gaps() {
        let entries = vec![entry(1), entry(2), entry(3)];
        let (stage, gaps) = detect_gaps(&entries);
        assert_eq!(stage.status, StageStatus::Done);
        assert!(gaps.is_empty());
    }

    #[test]
    fn detect_gaps_single_gap_reported() {
        let entries = vec![entry(1), entry(2), entry(5)];
        let (stage, gaps) = detect_gaps(&entries);
        assert_eq!(stage.status, StageStatus::Warn);
        assert_eq!(gaps.len(), 1);
        assert_eq!(gaps[0].after_cycle, 2);
        assert_eq!(gaps[0].before_cycle, 5);
        assert_eq!(gaps[0].missing_count, 2);
    }

    #[test]
    fn detect_gaps_multiple_gaps_aggregated() {
        let entries = vec![entry(1), entry(3), entry(7)];
        let (stage, gaps) = detect_gaps(&entries);
        assert_eq!(stage.status, StageStatus::Warn);
        assert_eq!(gaps.len(), 2);
        assert_eq!(gaps[0].missing_count, 1);
        assert_eq!(gaps[1].missing_count, 3);
    }

    #[test]
    fn check_standing_directives_returns_deferred() {
        let (stage, dirs) = check_standing_directives();
        assert_eq!(stage.status, StageStatus::Deferred);
        assert!(!dirs.implemented);
        assert!(dirs.items.is_empty());
    }

    #[test]
    fn identify_gardening_candidates_returns_deferred() {
        let (stage, cands) = identify_gardening_candidates();
        assert_eq!(stage.status, StageStatus::Deferred);
        assert!(!cands.implemented);
        assert!(cands.items.is_empty());
    }

    #[test]
    fn summarize_done_only_zero_exit() {
        let stages = vec![StageResult {
            name: "x".into(),
            status: StageStatus::Done,
            details: "".into(),
        }];
        let sm = summarize(&stages, false);
        assert_eq!(sm.exit_code, 0);
        assert_eq!(sm.done, 1);
    }

    #[test]
    fn summarize_warn_non_strict_zero_exit() {
        let stages = vec![StageResult {
            name: "x".into(),
            status: StageStatus::Warn,
            details: "".into(),
        }];
        let sm = summarize(&stages, false);
        assert_eq!(sm.exit_code, 0);
        assert_eq!(sm.warn, 1);
    }

    #[test]
    fn summarize_warn_strict_one_exit() {
        let stages = vec![StageResult {
            name: "x".into(),
            status: StageStatus::Warn,
            details: "".into(),
        }];
        let sm = summarize(&stages, true);
        assert_eq!(sm.exit_code, 1);
    }

    #[test]
    fn summarize_deferred_strict_one_exit() {
        let stages = vec![StageResult {
            name: "x".into(),
            status: StageStatus::Deferred,
            details: "".into(),
        }];
        let sm_loose = summarize(&stages, false);
        assert_eq!(sm_loose.exit_code, 0);
        let sm_strict = summarize(&stages, true);
        assert_eq!(sm_strict.exit_code, 1);
    }

    #[test]
    fn summarize_failed_always_one() {
        let stages = vec![StageResult {
            name: "x".into(),
            status: StageStatus::Failed,
            details: "".into(),
        }];
        let sm_loose = summarize(&stages, false);
        assert_eq!(sm_loose.exit_code, 1);
        let sm_strict = summarize(&stages, true);
        assert_eq!(sm_strict.exit_code, 1);
    }

    #[test]
    fn render_text_includes_all_sections() {
        let report = BootPhaseReport {
            stages: vec![StageResult {
                name: "load-cycle-history".into(),
                status: StageStatus::Done,
                details: "loaded 3 entries".into(),
            }],
            cycle_context: CycleContext {
                current_cycle: Some(123),
                previous_cycle: Some(122),
                previous_cycle_summary: None,
                cycles_loaded: 3,
                gaps: Vec::new(),
                standing_directives: StandingDirectives {
                    implemented: false,
                    note: "".into(),
                    items: Vec::new(),
                },
                gardening_candidates: GardeningCandidates {
                    implemented: false,
                    note: "".into(),
                    items: Vec::new(),
                },
            },
            summary: ReportSummary {
                done: 1,
                warn: 0,
                deferred: 0,
                failed: 0,
                skipped: 0,
                strict: false,
                exit_code: 0,
            },
        };
        let txt = render_text(&report);
        assert!(txt.contains("Stages:"));
        assert!(txt.contains("Cycle context:"));
        assert!(txt.contains("Summary:"));
        assert!(txt.contains("current_cycle:  123"));
        assert!(txt.contains("DONE"));
    }

    #[test]
    fn status_label_covers_all_variants() {
        assert_eq!(status_label(StageStatus::Done), "DONE");
        assert_eq!(status_label(StageStatus::Warn), "WARN");
        assert_eq!(status_label(StageStatus::Deferred), "DEFERRED");
        assert_eq!(status_label(StageStatus::Failed), "FAILED");
        assert_eq!(status_label(StageStatus::Skipped), "SKIPPED");
    }

    #[test]
    fn render_json_is_parseable() {
        let report = BootPhaseReport {
            stages: vec![],
            cycle_context: CycleContext {
                current_cycle: Some(5),
                previous_cycle: Some(4),
                previous_cycle_summary: None,
                cycles_loaded: 4,
                gaps: Vec::new(),
                standing_directives: StandingDirectives {
                    implemented: false,
                    note: "".into(),
                    items: Vec::new(),
                },
                gardening_candidates: GardeningCandidates {
                    implemented: false,
                    note: "".into(),
                    items: Vec::new(),
                },
            },
            summary: ReportSummary {
                done: 0,
                warn: 0,
                deferred: 0,
                failed: 0,
                skipped: 0,
                strict: false,
                exit_code: 0,
            },
        };
        let s = render_json(&report).unwrap();
        let parsed: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(
            parsed.get("cycle_context").unwrap().get("current_cycle"),
            Some(&Value::Number(5.into()))
        );
    }
}
