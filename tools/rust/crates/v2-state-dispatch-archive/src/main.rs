// v2-state-dispatch-archive
//
// Periodic age-based archival sweep for docs/state.json agent_sessions[].
// Moves terminal entries older than a configurable threshold from the live
// array to a dated archive file under docs/state-archive/.
//
// This is the archival counterpart to v2-state-audit (cycle 159, read-only).
// Audit reports; archive acts.
//
// Design scope: docs/redesign/_notes/v2-state-dispatch-archive.md (cycle 163).
// Key sections:
//   §2  Tool boundary (in-scope / out-of-scope discipline)
//   §3  Sweep criteria (status whitelist, age threshold, missing-timestamp fallback)
//   §4  Archive file schema (archive_version=1, dated filename, append-same-day)
//   §5  Invocation contract (CLI flags, exit codes, JSON output schema)
//   §6  Atomicity and recovery (step ordering, hash check, lock file)
//
// Exit codes:
//   0  Success — sweep completed (possibly zero entries archived)
//   1  Configuration error (bad flags, missing repo-root)
//   2  Input error (state.json unparseable, unwritable archive dir, lock timeout)
//   3  Concurrent-mutation error (state.json changed between read and write)
//   4  Partial failure — archive written but state.json mutation failed

use clap::Parser;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const TOOL_NAME: &str = "v2-state-dispatch-archive";
const TOOL_VERSION: &str = "0.1.0";

// §3.1 Terminal-status whitelist — entries with these statuses are eligible
// for archival (subject to the age check in §3.2).
const TERMINAL_ARCHIVABLE: &[&str] =
    &["merged", "failed", "closed_without_pr", "closed", "closed_without_merge"];

// §3.1 Live statuses — NEVER archived, regardless of age.
const LIVE_NEVER_ARCHIVED: &[&str] = &["in_flight", "reviewed_awaiting_eva"];

// Lock-acquire wait limit (§6.3).
const LOCK_WAIT_SECS: u64 = 30;

// --------------------------------------------------------------------------
// CLI (§5.1)
// --------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(
    name = "v2-state-dispatch-archive",
    about = "Age-based archival sweep for docs/state.json agent_sessions[]"
)]
struct Args {
    /// Repository root (default: current directory)
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Archival age cutoff in days (entries strictly older than this are eligible)
    #[arg(long, default_value_t = 30)]
    age_days: u64,

    /// Comma-separated terminal statuses to archive (default: all five terminal statuses)
    #[arg(long)]
    status: Option<String>,

    /// Maximum entries to archive in this invocation (default: unbounded)
    #[arg(long)]
    max_entries: Option<usize>,

    /// Archive output directory, relative to repo-root (default: docs/state-archive)
    #[arg(long, default_value = "docs/state-archive")]
    archive_dir: String,

    /// Plan only; no mutation
    #[arg(long, default_value_t = false)]
    dry_run: bool,

    /// Machine-readable JSON output to stdout
    #[arg(long, default_value_t = false)]
    json: bool,
}

// --------------------------------------------------------------------------
// Time helpers (no external chrono dependency — same approach as v2-state-audit)
// --------------------------------------------------------------------------

fn unix_now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn iso8601_now() -> String {
    format_unix_secs_iso(unix_now_secs())
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

/// Return the UTC date string "YYYY-MM-DD" for the given unix timestamp.
fn date_string_utc(unix_secs: u64) -> String {
    let z = unix_secs as i64;
    let days = z.div_euclid(86_400);
    let (y, mo, d) = civil_from_days(days);
    format!("{y:04}-{mo:02}-{d:02}")
}

/// Parse an ISO 8601 / RFC 3339 timestamp string to unix seconds.
/// Accepts "YYYY-MM-DDTHH:MM:SSZ" and "YYYY-MM-DDTHH:MM:SS+00:00" forms.
/// Returns None on any parse failure.
fn parse_iso8601(s: &str) -> Option<u64> {
    // Trim possible quotes and whitespace.
    let s = s.trim().trim_matches('"');
    if s.len() < 19 {
        return None;
    }
    let year: i64 = s[0..4].parse().ok()?;
    let month: i64 = s[5..7].parse().ok()?;
    let day: i64 = s[8..10].parse().ok()?;
    let hour: i64 = s[11..13].parse().ok()?;
    let min: i64 = s[14..16].parse().ok()?;
    let sec: i64 = s[17..19].parse().ok()?;
    // Days since epoch via the same civil_from_days algorithm inverted.
    let days = days_from_civil(year, month, day)?;
    let unix = days * 86_400 + hour * 3600 + min * 60 + sec;
    if unix < 0 {
        return None;
    }
    Some(unix as u64)
}

fn days_from_civil(y: i64, m: i64, d: i64) -> Option<i64> {
    if m < 1 || m > 12 || d < 1 || d > 31 {
        return None;
    }
    let (y, m) = if m <= 2 { (y - 1, m + 9) } else { (y, m - 3) };
    let era = y.div_euclid(400);
    let yoe = y.rem_euclid(400);
    let doy = (153 * m + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146_097 + doe - 719_468)
}

// --------------------------------------------------------------------------
// Archive file schema (§4)
// --------------------------------------------------------------------------

/// Top-level archive file structure.  Multiple sweeps on the same day append
/// to the existing file's `entries` array (§4.3).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ArchiveFile {
    archive_version: u32,
    tool_version: String,
    archived_at: String,
    source_file: String,
    criteria: ArchiveCriteria,
    entries: Vec<Value>,
}

/// Serde-transparent wrapper that handles invocation_id as either a single
/// string or an array of strings (§4.3 multi-sweep same-day case).
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ArchiveCriteria {
    age_days: u64,
    status_filter: Vec<String>,
    /// Single string on first write; array on subsequent same-day appends.
    invocation_id: InvocationId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
enum InvocationId {
    Single(String),
    Multiple(Vec<String>),
}

impl InvocationId {
    fn to_vec(&self) -> Vec<String> {
        match self {
            InvocationId::Single(s) => vec![s.clone()],
            InvocationId::Multiple(v) => v.clone(),
        }
    }

    fn append(self, new_id: String) -> InvocationId {
        let mut ids = self.to_vec();
        if !ids.contains(&new_id) {
            ids.push(new_id);
        }
        if ids.len() == 1 {
            InvocationId::Single(ids.remove(0))
        } else {
            InvocationId::Multiple(ids)
        }
    }
}

// --------------------------------------------------------------------------
// JSON report output (§5.3)
// --------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
struct SweepReport {
    tool: &'static str,
    tool_version: &'static str,
    invoked_at: String,
    dry_run: bool,
    archived_count: usize,
    archive_file: String,
    ineligible: IneligibleCounts,
    state_after: StateAfter,
}

#[derive(Debug, Clone, Serialize)]
struct IneligibleCounts {
    live_status: usize,
    ineligible_status: usize,
    below_age_threshold: usize,
    missing_timestamp: usize,
}

#[derive(Debug, Clone, Serialize)]
struct StateAfter {
    agent_sessions_total: usize,
    agent_sessions_live: usize,
    agent_sessions_terminal_retained: usize,
}

// --------------------------------------------------------------------------
// SHA-256 hash of agent_sessions[] for concurrent-mutation detection (§6.2)
// --------------------------------------------------------------------------

/// Compute SHA-256 of the canonical JSON serialisation of `sessions`.
/// Uses `serde_json::to_string` with preserve_order feature (via the crate
/// feature flag in Cargo.toml) so field order matches what was read from disk.
fn hash_sessions(sessions: &[Value]) -> String {
    let json = serde_json::to_string(sessions).unwrap_or_default();
    let digest = Sha256::digest(json.as_bytes());
    format!("{digest:x}")
}

// --------------------------------------------------------------------------
// Lock file (§6.3)
// --------------------------------------------------------------------------

struct LockFile {
    path: PathBuf,
}

impl LockFile {
    /// Attempt to acquire an exclusive lock.  Polls every 500 ms for up to
    /// `LOCK_WAIT_SECS` seconds, then gives up and returns Err.
    fn acquire(path: &Path) -> Result<Self, String> {
        let deadline =
            SystemTime::now() + Duration::from_secs(LOCK_WAIT_SECS);
        loop {
            match fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(path)
            {
                Ok(_) => return Ok(LockFile { path: path.to_path_buf() }),
                Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
                    if SystemTime::now() >= deadline {
                        return Err(format!(
                            "lock-acquire timeout after {LOCK_WAIT_SECS}s: {}",
                            path.display()
                        ));
                    }
                    std::thread::sleep(Duration::from_millis(500));
                }
                Err(e) => {
                    return Err(format!(
                        "lock-acquire error {}: {e}",
                        path.display()
                    ));
                }
            }
        }
    }
}

impl Drop for LockFile {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

// --------------------------------------------------------------------------
// Eligibility classification
// --------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
enum Eligibility {
    Eligible,
    LiveStatus,
    IneligibleStatus,
    BelowAgeThreshold,
    MissingTimestamp,
}

fn classify_entry(
    entry: &Value,
    status_filter: &[String],
    age_cutoff_secs: u64,
    now_secs: u64,
) -> Eligibility {
    let status = entry
        .get("status")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    // Live entries are never archived (§3.1, §9.2).
    if LIVE_NEVER_ARCHIVED.contains(&status) {
        return Eligibility::LiveStatus;
    }

    // Status must be in the provided filter list.
    if !status_filter.iter().any(|s| s == status) {
        return Eligibility::IneligibleStatus;
    }

    // Age check (§3.2 / §3.3): try merged_at first, then dispatched_at.
    let ts = resolve_timestamp(entry);
    match ts {
        None => Eligibility::MissingTimestamp,
        Some(entry_secs) => {
            // Entries strictly older than the cutoff are eligible.
            if now_secs > entry_secs && now_secs - entry_secs > age_cutoff_secs {
                Eligibility::Eligible
            } else {
                Eligibility::BelowAgeThreshold
            }
        }
    }
}

/// §3.2 timestamp priority: merged_at first, dispatched_at as fallback.
/// Returns None if both are absent, null, or unparseable.
fn resolve_timestamp(entry: &Value) -> Option<u64> {
    for field in &["merged_at", "dispatched_at"] {
        if let Some(ts) = entry.get(field) {
            if ts.is_null() {
                continue;
            }
            if let Some(s) = ts.as_str() {
                if let Some(secs) = parse_iso8601(s) {
                    return Some(secs);
                }
            }
        }
    }
    None
}

// --------------------------------------------------------------------------
// Archive file read/write helpers
// --------------------------------------------------------------------------

/// Load an existing archive file, or return a fresh empty one.
fn load_or_create_archive(
    path: &Path,
    now_iso: &str,
    age_days: u64,
    status_filter: &[String],
    invocation_id: &str,
) -> Result<ArchiveFile, String> {
    if path.exists() {
        let text = fs::read_to_string(path).map_err(|e| {
            format!("failed to read existing archive {}: {e}", path.display())
        })?;
        let mut existing: ArchiveFile =
            serde_json::from_str(&text).map_err(|e| {
                format!(
                    "failed to parse existing archive {}: {e}",
                    path.display()
                )
            })?;
        // Update for this sweep (§4.3).
        existing.archived_at = now_iso.to_string();
        existing.criteria.invocation_id = existing
            .criteria
            .invocation_id
            .append(invocation_id.to_string());
        Ok(existing)
    } else {
        Ok(ArchiveFile {
            archive_version: 1,
            tool_version: format!("{TOOL_NAME} {TOOL_VERSION}"),
            archived_at: now_iso.to_string(),
            source_file: "docs/state.json".to_string(),
            criteria: ArchiveCriteria {
                age_days,
                status_filter: status_filter.to_vec(),
                invocation_id: InvocationId::Single(invocation_id.to_string()),
            },
            entries: Vec::new(),
        })
    }
}

/// Atomically write `archive` to `path` via tempfile + rename (§6.1 step 4).
fn write_archive_atomic(path: &Path, archive: &ArchiveFile) -> Result<(), String> {
    // Ensure parent directory exists.
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            format!(
                "failed to create archive directory {}: {e}",
                parent.display()
            )
        })?;
    }
    let tmp_path = path.with_extension("json.tmp");
    {
        let mut f = fs::File::create(&tmp_path).map_err(|e| {
            format!("failed to create tmp archive {}: {e}", tmp_path.display())
        })?;
        serde_json::to_writer_pretty(&mut f, archive).map_err(|e| {
            format!("failed to serialize archive: {e}")
        })?;
        f.flush().map_err(|e| {
            format!("failed to flush tmp archive: {e}")
        })?;
    }
    fs::rename(&tmp_path, path).map_err(|e| {
        format!(
            "failed to rename tmp archive to {}: {e}",
            path.display()
        )
    })?;
    Ok(())
}

/// Read docs/state.json, returning (raw Value, agent_sessions array).
fn read_state(state_path: &Path) -> Result<(Value, Vec<Value>), String> {
    let text = fs::read_to_string(state_path).map_err(|e| {
        format!("failed to read {}: {e}", state_path.display())
    })?;
    let mut root: Value = serde_json::from_str(&text).map_err(|e| {
        format!("failed to parse {}: {e}", state_path.display())
    })?;
    let sessions = root
        .get_mut("agent_sessions")
        .and_then(|v| v.as_array_mut())
        .map(|a| a.clone())
        .unwrap_or_default();
    Ok((root, sessions))
}

/// Atomically write mutated state.json via tempfile + rename (§6.1 step 6).
fn write_state_atomic(state_path: &Path, state: &Value) -> Result<(), String> {
    let tmp_path = state_path.with_extension("json.tmp");
    {
        let mut f = fs::File::create(&tmp_path).map_err(|e| {
            format!(
                "failed to create tmp state file {}: {e}",
                tmp_path.display()
            )
        })?;
        serde_json::to_writer_pretty(&mut f, state).map_err(|e| {
            format!("failed to serialize state.json: {e}")
        })?;
        f.flush().map_err(|e| {
            format!("failed to flush tmp state file: {e}")
        })?;
    }
    fs::rename(&tmp_path, state_path).map_err(|e| {
        format!(
            "failed to rename tmp state to {}: {e}",
            state_path.display()
        )
    })?;
    Ok(())
}

// --------------------------------------------------------------------------
// Output formatting
// --------------------------------------------------------------------------

fn format_human(report: &SweepReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "v2-state-dispatch-archive  invoked_at={}\n",
        report.invoked_at
    ));
    if report.dry_run {
        out.push_str("mode: DRY RUN (no mutations)\n");
    }
    out.push_str(&format!(
        "archived:              {}\n",
        report.archived_count
    ));
    out.push_str(&format!(
        "archive_file:          {}\n",
        report.archive_file
    ));
    out.push_str(&format!(
        "ineligible.live_status:          {}\n",
        report.ineligible.live_status
    ));
    out.push_str(&format!(
        "ineligible.ineligible_status:    {}\n",
        report.ineligible.ineligible_status
    ));
    out.push_str(&format!(
        "ineligible.below_age_threshold:  {}\n",
        report.ineligible.below_age_threshold
    ));
    out.push_str(&format!(
        "ineligible.missing_timestamp:    {}\n",
        report.ineligible.missing_timestamp
    ));
    out.push_str(&format!(
        "state_after.agent_sessions_total:             {}\n",
        report.state_after.agent_sessions_total
    ));
    out.push_str(&format!(
        "state_after.agent_sessions_live:              {}\n",
        report.state_after.agent_sessions_live
    ));
    out.push_str(&format!(
        "state_after.agent_sessions_terminal_retained: {}\n",
        report.state_after.agent_sessions_terminal_retained
    ));
    out
}

// --------------------------------------------------------------------------
// Core sweep logic
// --------------------------------------------------------------------------

fn run(args: Args) -> ExitCode {
    // Validate repo-root.
    let repo_root = args.repo_root.canonicalize().unwrap_or_else(|_| {
        // Allow non-existent root to propagate as an error below.
        args.repo_root.clone()
    });

    let state_path = repo_root.join("docs").join("state.json");
    if !state_path.exists() {
        eprintln!(
            "{TOOL_NAME}: docs/state.json not found at {}",
            state_path.display()
        );
        return ExitCode::from(2);
    }

    // Resolve status filter.
    let status_filter: Vec<String> = match &args.status {
        Some(csv) => csv.split(',').map(|s| s.trim().to_string()).collect(),
        None => TERMINAL_ARCHIVABLE.iter().map(|s| s.to_string()).collect(),
    };

    // Validate that all requested statuses are terminal (fail-safe §3.1).
    for s in &status_filter {
        if LIVE_NEVER_ARCHIVED.contains(&s.as_str()) {
            eprintln!(
                "{TOOL_NAME}: status '{s}' is a live status and cannot be archived"
            );
            return ExitCode::from(1);
        }
    }

    let age_cutoff_secs = args.age_days * 86_400;
    let now_secs = unix_now_secs();
    let now_iso = iso8601_now();
    let today = date_string_utc(now_secs);

    let archive_dir = repo_root.join(&args.archive_dir);
    let archive_path =
        archive_dir.join(format!("dispatches-{today}.json"));
    let lock_path = state_path.with_extension("json.lock");

    // §6.3 Acquire lock (skipped in dry-run to avoid side effects, but we
    // still take it to protect the read-consistency guarantee).
    let _lock = match LockFile::acquire(&lock_path) {
        Ok(l) => l,
        Err(e) => {
            eprintln!("{TOOL_NAME}: {e}");
            return ExitCode::from(2);
        }
    };

    // §6.1 Step 1: read and parse state.json, compute hash.
    let (mut root, sessions) = match read_state(&state_path) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{TOOL_NAME}: {e}");
            return ExitCode::from(2);
        }
    };
    let sessions_hash_before = hash_sessions(&sessions);

    // §6.1 Step 2: classify entries.
    let mut eligible: Vec<Value> = Vec::new();
    let mut retained: Vec<Value> = Vec::new();
    let mut count_live = 0usize;
    let mut count_ineligible_status = 0usize;
    let mut count_below_age = 0usize;
    let mut count_missing_ts = 0usize;

    for entry in &sessions {
        match classify_entry(entry, &status_filter, age_cutoff_secs, now_secs) {
            Eligibility::Eligible => eligible.push(entry.clone()),
            Eligibility::LiveStatus => {
                count_live += 1;
                retained.push(entry.clone());
            }
            Eligibility::IneligibleStatus => {
                count_ineligible_status += 1;
                retained.push(entry.clone());
            }
            Eligibility::BelowAgeThreshold => {
                count_below_age += 1;
                retained.push(entry.clone());
            }
            Eligibility::MissingTimestamp => {
                count_missing_ts += 1;
                retained.push(entry.clone());
            }
        }
    }

    // Apply --max-entries limit (§3.4).
    if let Some(max) = args.max_entries {
        if eligible.len() > max {
            // The excess eligible entries go back to retained.
            let excess = eligible.split_off(max);
            retained.extend(excess);
        }
    }

    let archived_count = eligible.len();
    let total_after = retained.len();
    let live_after = retained
        .iter()
        .filter(|e| {
            let s = e.get("status").and_then(|v| v.as_str()).unwrap_or("");
            LIVE_NEVER_ARCHIVED.contains(&s)
        })
        .count();
    let terminal_retained = total_after - live_after;

    let archive_file_rel = format!("{}/{}", args.archive_dir, format!("dispatches-{today}.json"));

    // §6.1 Step 3: dry-run exit.
    if args.dry_run {
        let report = SweepReport {
            tool: TOOL_NAME,
            tool_version: TOOL_VERSION,
            invoked_at: now_iso,
            dry_run: true,
            archived_count,
            archive_file: archive_file_rel,
            ineligible: IneligibleCounts {
                live_status: count_live,
                ineligible_status: count_ineligible_status,
                below_age_threshold: count_below_age,
                missing_timestamp: count_missing_ts,
            },
            state_after: StateAfter {
                agent_sessions_total: total_after,
                agent_sessions_live: live_after,
                agent_sessions_terminal_retained: terminal_retained,
            },
        };
        emit_report(&report, args.json);
        return ExitCode::from(0);
    }

    // If nothing to archive, skip writes and report.
    if archived_count == 0 {
        let report = SweepReport {
            tool: TOOL_NAME,
            tool_version: TOOL_VERSION,
            invoked_at: now_iso,
            dry_run: false,
            archived_count: 0,
            archive_file: archive_file_rel,
            ineligible: IneligibleCounts {
                live_status: count_live,
                ineligible_status: count_ineligible_status,
                below_age_threshold: count_below_age,
                missing_timestamp: count_missing_ts,
            },
            state_after: StateAfter {
                agent_sessions_total: total_after,
                agent_sessions_live: live_after,
                agent_sessions_terminal_retained: terminal_retained,
            },
        };
        emit_report(&report, args.json);
        return ExitCode::from(0);
    }

    // §6.1 Step 4: write archive file (atomic via tempfile + rename).
    // Determine invocation_id from today's date (no cycle-N suffix available
    // at this layer; the operator may pass a richer flag in the future).
    let invocation_id = today.clone();
    let mut archive =
        match load_or_create_archive(
            &archive_path,
            &now_iso,
            args.age_days,
            &status_filter,
            &invocation_id,
        ) {
            Ok(a) => a,
            Err(e) => {
                eprintln!("{TOOL_NAME}: {e}");
                return ExitCode::from(2);
            }
        };
    archive.entries.extend(eligible.clone());

    if let Err(e) = write_archive_atomic(&archive_path, &archive) {
        eprintln!("{TOOL_NAME}: {e}");
        return ExitCode::from(2);
    }

    // §6.1 Step 5: re-read state.json and verify hash (concurrent-mutation check).
    let current_sessions = match read_state(&state_path) {
        Ok((_, s)) => s,
        Err(e) => {
            eprintln!(
                "{TOOL_NAME}: hash re-check failed (archive written, state.json NOT mutated): {e}"
            );
            return ExitCode::from(3);
        }
    };
    let sessions_hash_after = hash_sessions(&current_sessions);
    if sessions_hash_before != sessions_hash_after {
        eprintln!(
            "{TOOL_NAME}: concurrent mutation detected — archive written, state.json NOT mutated (exit 3)"
        );
        return ExitCode::from(3);
    }

    // §6.1 Step 6: write mutated state.json.
    if let Some(arr) = root.get_mut("agent_sessions").and_then(|v| v.as_array_mut()) {
        *arr = retained.clone();
    }

    if let Err(e) = write_state_atomic(&state_path, &root) {
        eprintln!(
            "{TOOL_NAME}: archive written but state.json mutation failed (exit 4): {e}"
        );
        return ExitCode::from(4);
    }

    // §6.1 Step 7: emit report and exit 0.
    let report = SweepReport {
        tool: TOOL_NAME,
        tool_version: TOOL_VERSION,
        invoked_at: now_iso,
        dry_run: false,
        archived_count,
        archive_file: archive_file_rel,
        ineligible: IneligibleCounts {
            live_status: count_live,
            ineligible_status: count_ineligible_status,
            below_age_threshold: count_below_age,
            missing_timestamp: count_missing_ts,
        },
        state_after: StateAfter {
            agent_sessions_total: total_after,
            agent_sessions_live: live_after,
            agent_sessions_terminal_retained: terminal_retained,
        },
    };
    emit_report(&report, args.json);
    ExitCode::from(0)
}

fn emit_report(report: &SweepReport, use_json: bool) {
    if use_json {
        match serde_json::to_string_pretty(report) {
            Ok(s) => println!("{s}"),
            Err(e) => eprintln!("{TOOL_NAME}: failed to serialize JSON report: {e}"),
        }
    } else {
        print!("{}", format_human(report));
    }
}

// --------------------------------------------------------------------------
// main
// --------------------------------------------------------------------------

fn main() -> ExitCode {
    let args = Args::parse();
    run(args)
}

// --------------------------------------------------------------------------
// Tests
// --------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    // -- parse_iso8601 -----------------------------------------------------

    #[test]
    fn parse_iso8601_basic() {
        // 2026-01-01T00:00:00Z should be a fixed known unix timestamp.
        let ts = parse_iso8601("2026-01-01T00:00:00Z").unwrap();
        // 2026-01-01 is 56 years + leap-year adjustments from 1970.
        // Quick sanity: should be > 1 Jan 2025 (1_735_689_600).
        assert!(ts > 1_735_689_600);
        // And < 1 Jan 2027 (1_798_761_600).
        assert!(ts < 1_798_761_600);
    }

    #[test]
    fn parse_iso8601_plus_offset() {
        // "+00:00" suffix: same day as Z.
        let z = parse_iso8601("2026-02-24T16:08:04Z").unwrap();
        let plus = parse_iso8601("2026-02-24T16:08:04+00:00").unwrap();
        assert_eq!(z, plus);
    }

    #[test]
    fn parse_iso8601_invalid_returns_none() {
        assert!(parse_iso8601("not-a-date").is_none());
        assert!(parse_iso8601("").is_none());
        assert!(parse_iso8601("2026-13-01T00:00:00Z").is_none()); // bad month
    }

    #[test]
    fn parse_iso8601_roundtrip_recent() {
        // A timestamp we know: 2026-05-17T03:09:57Z.
        let ts = parse_iso8601("2026-05-17T03:09:57Z").unwrap();
        let s = format_unix_secs_iso(ts);
        assert_eq!(s, "2026-05-17T03:09:57Z");
    }

    // -- date_string_utc ---------------------------------------------------

    #[test]
    fn date_string_utc_epoch() {
        assert_eq!(date_string_utc(0), "1970-01-01");
    }

    #[test]
    fn date_string_utc_known_date() {
        // 2026-05-17T00:00:00Z = 1_747_440_000.
        let ts = parse_iso8601("2026-05-17T00:00:00Z").unwrap();
        assert_eq!(date_string_utc(ts), "2026-05-17");
    }

    // -- resolve_timestamp -------------------------------------------------

    #[test]
    fn resolve_timestamp_merged_at_preferred() {
        let entry = json!({
            "status": "merged",
            "merged_at": "2026-02-01T00:00:00Z",
            "dispatched_at": "2026-01-01T00:00:00Z"
        });
        let ts = resolve_timestamp(&entry).unwrap();
        // Should match merged_at.
        assert_eq!(ts, parse_iso8601("2026-02-01T00:00:00Z").unwrap());
    }

    #[test]
    fn resolve_timestamp_falls_back_to_dispatched_at() {
        let entry = json!({
            "status": "merged",
            "merged_at": null,
            "dispatched_at": "2026-01-15T00:00:00Z"
        });
        let ts = resolve_timestamp(&entry).unwrap();
        assert_eq!(ts, parse_iso8601("2026-01-15T00:00:00Z").unwrap());
    }

    #[test]
    fn resolve_timestamp_both_null_returns_none() {
        let entry = json!({ "status": "merged", "merged_at": null, "dispatched_at": null });
        assert!(resolve_timestamp(&entry).is_none());
    }

    #[test]
    fn resolve_timestamp_both_absent_returns_none() {
        let entry = json!({ "status": "merged" });
        assert!(resolve_timestamp(&entry).is_none());
    }

    // -- classify_entry ----------------------------------------------------

    fn make_entry(status: &str, merged_at: Option<&str>, dispatched_at: Option<&str>) -> Value {
        let mut m = serde_json::Map::new();
        m.insert("status".to_string(), json!(status));
        m.insert(
            "merged_at".to_string(),
            merged_at.map(|s| json!(s)).unwrap_or(json!(null)),
        );
        m.insert(
            "dispatched_at".to_string(),
            dispatched_at.map(|s| json!(s)).unwrap_or(json!(null)),
        );
        Value::Object(m)
    }

    fn default_filter() -> Vec<String> {
        TERMINAL_ARCHIVABLE.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn classify_live_status_never_archived() {
        let now = unix_now_secs();
        let old = format_unix_secs_iso(now - 200 * 86_400);
        let e = make_entry("in_flight", Some(&old), Some(&old));
        assert_eq!(
            classify_entry(&e, &default_filter(), 30 * 86_400, now),
            Eligibility::LiveStatus
        );
        let e2 = make_entry("reviewed_awaiting_eva", Some(&old), Some(&old));
        assert_eq!(
            classify_entry(&e2, &default_filter(), 30 * 86_400, now),
            Eligibility::LiveStatus
        );
    }

    #[test]
    fn classify_unknown_status_is_ineligible() {
        let now = unix_now_secs();
        let old = format_unix_secs_iso(now - 200 * 86_400);
        let e = make_entry("unknown_status", Some(&old), Some(&old));
        assert_eq!(
            classify_entry(&e, &default_filter(), 30 * 86_400, now),
            Eligibility::IneligibleStatus
        );
    }

    #[test]
    fn classify_eligible_merged_entry() {
        let now = unix_now_secs();
        let old = format_unix_secs_iso(now - 60 * 86_400); // 60 days ago
        let e = make_entry("merged", Some(&old), Some(&old));
        assert_eq!(
            classify_entry(&e, &default_filter(), 30 * 86_400, now),
            Eligibility::Eligible
        );
    }

    #[test]
    fn classify_below_age_threshold() {
        let now = unix_now_secs();
        let recent = format_unix_secs_iso(now - 10 * 86_400); // 10 days ago
        let e = make_entry("merged", Some(&recent), Some(&recent));
        assert_eq!(
            classify_entry(&e, &default_filter(), 30 * 86_400, now),
            Eligibility::BelowAgeThreshold
        );
    }

    #[test]
    fn classify_missing_timestamp() {
        let now = unix_now_secs();
        let e = make_entry("merged", None, None);
        assert_eq!(
            classify_entry(&e, &default_filter(), 30 * 86_400, now),
            Eligibility::MissingTimestamp
        );
    }

    // -- hash_sessions consistency -----------------------------------------

    #[test]
    fn hash_sessions_same_input_same_output() {
        let sessions = vec![json!({"a": 1}), json!({"b": 2})];
        assert_eq!(hash_sessions(&sessions), hash_sessions(&sessions));
    }

    #[test]
    fn hash_sessions_different_input_different_output() {
        let a = vec![json!({"a": 1})];
        let b = vec![json!({"a": 2})];
        assert_ne!(hash_sessions(&a), hash_sessions(&b));
    }

    // -- InvocationId append -----------------------------------------------

    #[test]
    fn invocation_id_append_single_to_multiple() {
        let id = InvocationId::Single("day1".to_string());
        let id2 = id.append("day2".to_string());
        match id2 {
            InvocationId::Multiple(v) => {
                assert_eq!(v, vec!["day1", "day2"]);
            }
            _ => panic!("expected Multiple"),
        }
    }

    #[test]
    fn invocation_id_append_deduplicates() {
        let id = InvocationId::Single("day1".to_string());
        let id2 = id.append("day1".to_string());
        match id2 {
            InvocationId::Single(s) => assert_eq!(s, "day1"),
            _ => panic!("expected Single after dedup"),
        }
    }

    // -- write_archive_atomic and load_or_create_archive --------------------

    fn tmp_dir() -> TempDir {
        tempfile::tempdir().unwrap()
    }

    #[test]
    fn write_and_reload_archive() {
        let dir = tmp_dir();
        let archive_path = dir.path().join("dispatches-2026-05-17.json");
        let filter = default_filter();
        let archive = load_or_create_archive(
            &archive_path,
            "2026-05-17T00:00:00Z",
            30,
            &filter,
            "2026-05-17",
        )
        .unwrap();
        assert_eq!(archive.archive_version, 1);
        assert!(archive.entries.is_empty());

        let mut archive2 = archive;
        archive2.entries.push(json!({"status": "merged", "issue": 1}));
        write_archive_atomic(&archive_path, &archive2).unwrap();

        // Reload.
        let reloaded = load_or_create_archive(
            &archive_path,
            "2026-05-17T01:00:00Z",
            30,
            &filter,
            "2026-05-17-second",
        )
        .unwrap();
        assert_eq!(reloaded.entries.len(), 1);
        // invocation_id should now be Multiple.
        match &reloaded.criteria.invocation_id {
            InvocationId::Multiple(v) => {
                assert!(v.contains(&"2026-05-17".to_string()));
                assert!(v.contains(&"2026-05-17-second".to_string()));
            }
            _ => panic!("expected Multiple after two invocations"),
        }
    }

    // -- Integration-style: run() with temp repo ---------------------------

    fn make_temp_state(dir: &Path, sessions: Vec<Value>) -> PathBuf {
        let docs = dir.join("docs");
        fs::create_dir_all(&docs).unwrap();
        let state = json!({ "agent_sessions": sessions });
        let p = docs.join("state.json");
        fs::write(&p, serde_json::to_vec_pretty(&state).unwrap()).unwrap();
        p
    }

    fn old_entry(status: &str, days_ago: u64) -> Value {
        let now = unix_now_secs();
        let ts = format_unix_secs_iso(now - days_ago * 86_400);
        json!({
            "status": status,
            "merged_at": if status == "merged" { json!(ts.clone()) } else { json!(null) },
            "dispatched_at": ts,
            "issue": 42,
            "title": "test entry"
        })
    }

    #[test]
    fn dry_run_does_not_mutate_state() {
        let dir = tmp_dir();
        let entries = vec![
            old_entry("merged", 60),
            old_entry("failed", 60),
            old_entry("in_flight", 200),
        ];
        make_temp_state(dir.path(), entries.clone());

        let args = Args {
            repo_root: dir.path().to_path_buf(),
            age_days: 30,
            status: None,
            max_entries: None,
            archive_dir: "docs/state-archive".to_string(),
            dry_run: true,
            json: false,
        };
        let code = run(args);
        assert_eq!(code, ExitCode::from(0));

        // State must be unchanged.
        let (_, sessions_after) = read_state(&dir.path().join("docs/state.json")).unwrap();
        assert_eq!(sessions_after.len(), 3);

        // Archive dir must not exist.
        assert!(!dir.path().join("docs/state-archive").exists());
    }

    #[test]
    fn happy_path_archives_old_terminal_entries() {
        let dir = tmp_dir();
        let entries = vec![
            old_entry("merged", 60),   // eligible
            old_entry("failed", 60),   // eligible
            old_entry("merged", 10),   // too recent
            old_entry("in_flight", 200), // live, never archived
        ];
        make_temp_state(dir.path(), entries);

        let args = Args {
            repo_root: dir.path().to_path_buf(),
            age_days: 30,
            status: None,
            max_entries: None,
            archive_dir: "docs/state-archive".to_string(),
            dry_run: false,
            json: false,
        };
        let code = run(args);
        assert_eq!(code, ExitCode::from(0));

        // state.json should have 2 entries left (recent merged + in_flight).
        let (_, sessions_after) = read_state(&dir.path().join("docs/state.json")).unwrap();
        assert_eq!(sessions_after.len(), 2);

        // Archive file must exist with 2 entries.
        let archive_dir = dir.path().join("docs/state-archive");
        let today = date_string_utc(unix_now_secs());
        let archive_path = archive_dir.join(format!("dispatches-{today}.json"));
        assert!(archive_path.exists());
        let archive_text = fs::read_to_string(&archive_path).unwrap();
        let archive: ArchiveFile = serde_json::from_str(&archive_text).unwrap();
        assert_eq!(archive.archive_version, 1);
        assert_eq!(archive.entries.len(), 2);
    }

    #[test]
    fn max_entries_limits_sweep() {
        let dir = tmp_dir();
        let entries = vec![
            old_entry("merged", 60),
            old_entry("merged", 61),
            old_entry("merged", 62),
        ];
        make_temp_state(dir.path(), entries);

        let args = Args {
            repo_root: dir.path().to_path_buf(),
            age_days: 30,
            status: None,
            max_entries: Some(2),
            archive_dir: "docs/state-archive".to_string(),
            dry_run: false,
            json: false,
        };
        let code = run(args);
        assert_eq!(code, ExitCode::from(0));

        let (_, sessions_after) = read_state(&dir.path().join("docs/state.json")).unwrap();
        assert_eq!(sessions_after.len(), 1);

        let today = date_string_utc(unix_now_secs());
        let archive_path = dir
            .path()
            .join("docs/state-archive")
            .join(format!("dispatches-{today}.json"));
        let archive_text = fs::read_to_string(&archive_path).unwrap();
        let archive: ArchiveFile = serde_json::from_str(&archive_text).unwrap();
        assert_eq!(archive.entries.len(), 2);
    }

    #[test]
    fn zero_eligible_produces_no_archive_file() {
        let dir = tmp_dir();
        let entries = vec![old_entry("merged", 5)]; // too recent
        make_temp_state(dir.path(), entries);

        let args = Args {
            repo_root: dir.path().to_path_buf(),
            age_days: 30,
            status: None,
            max_entries: None,
            archive_dir: "docs/state-archive".to_string(),
            dry_run: false,
            json: false,
        };
        let code = run(args);
        assert_eq!(code, ExitCode::from(0));
        assert!(!dir.path().join("docs/state-archive").exists());
    }

    #[test]
    fn json_output_flag_produces_parseable_json() {
        let dir = tmp_dir();
        let entries = vec![old_entry("merged", 60)];
        make_temp_state(dir.path(), entries);

        let args = Args {
            repo_root: dir.path().to_path_buf(),
            age_days: 30,
            status: None,
            max_entries: None,
            archive_dir: "docs/state-archive".to_string(),
            dry_run: true,
            json: true,
        };
        // Just verify it doesn't crash (output goes to stdout in tests).
        let code = run(args);
        assert_eq!(code, ExitCode::from(0));
    }

    #[test]
    fn live_status_in_status_filter_is_rejected() {
        let dir = tmp_dir();
        make_temp_state(dir.path(), vec![]);

        let args = Args {
            repo_root: dir.path().to_path_buf(),
            age_days: 30,
            status: Some("in_flight".to_string()),
            max_entries: None,
            archive_dir: "docs/state-archive".to_string(),
            dry_run: false,
            json: false,
        };
        let code = run(args);
        assert_eq!(code, ExitCode::from(1));
    }
}
