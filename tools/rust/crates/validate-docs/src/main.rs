use clap::{Parser, Subcommand};
use serde::Deserialize;
use state_schema::StateJson;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command as ProcessCommand;

const SELF_MODIFICATIONS_HEADING: &str = "## Self-modifications";
const COMMIT_RECEIPTS_HEADING: &str = "## Commit receipts";
const CONCRETE_COMMITMENTS_HEADING: &str = "### Concrete commitments for next cycle";
const OPEN_QUESTIONS_HEADING: &str = "### Open questions";
const OPEN_QUESTIONS_RAISED_THIS_CYCLE_HEADING: &str = "### Open questions raised this cycle";
const STANDING_EVA_BLOCKERS_HEADING: &str = "### Standing Eva blockers";
const PRE_DISPATCH_SNAPSHOT_NOTE: &str =
    "*Snapshot before review dispatch — final counters may differ after C6.*";
const INFRASTRUCTURE_PATHS: [&str; 5] = [
    "STARTUP_CHECKLIST.xml",
    "COMPLETION_CHECKLIST.xml",
    "AGENTS.md",
    ".claude/skills",
    "tools",
];

#[derive(Debug, Parser)]
#[command(name = "validate-docs")]
struct Cli {
    /// Repository root path
    #[arg(long, default_value = ".", global = true)]
    repo_root: PathBuf,

    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Validate a generated worklog entry
    Worklog(WorklogArgs),
    /// Validate a generated journal entry
    Journal(JournalArgs),
}

#[derive(Debug, Parser)]
struct WorklogArgs {
    /// Path to the worklog file to validate
    #[arg(long)]
    file: PathBuf,

    /// Cycle number the worklog belongs to
    #[arg(long)]
    cycle: u64,

    /// Pipeline status to validate against instead of invoking pipeline-check
    #[arg(long)]
    pipeline_status: Option<String>,

    /// Commit to use as the upper bound when diffing infrastructure changes
    #[arg(long)]
    worklog_commit: Option<String>,
}

#[derive(Debug, Parser)]
struct JournalArgs {
    /// Path to the journal file to validate
    #[arg(long)]
    file: PathBuf,
}

#[derive(Clone, Debug, Deserialize)]
struct ReceiptEntry {
    receipt: String,
    #[serde(default)]
    url: String,
}

impl ReceiptEntry {
    /// Return the longest available SHA — full SHA from URL if available, otherwise the short receipt.
    fn best_sha(&self) -> &str {
        if !self.url.is_empty() {
            if let Some(pos) = self.url.rfind('/') {
                let candidate = &self.url[pos + 1..];
                if !candidate.is_empty() && candidate.chars().all(|c| c.is_ascii_hexdigit()) {
                    return candidate;
                }
            }
        }
        &self.receipt
    }
}

#[derive(Debug, Deserialize)]
struct PipelineReport {
    overall: String,
}

#[derive(Debug, PartialEq, Eq)]
enum SelfModificationFinding {
    Failure(String),
    Warning(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct WorklogValidationOptions {
    exclude_post_c3_fields: bool,
}

impl Default for WorklogValidationOptions {
    fn default() -> Self {
        Self {
            exclude_post_c3_fields: true,
        }
    }
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Worklog(args) => validate_worklog(
            &cli.repo_root,
            &args.file,
            args.cycle,
            args.pipeline_status.as_deref(),
            args.worklog_commit.as_deref(),
        ),
        Command::Journal(args) => validate_journal(&cli.repo_root, &args.file),
    };

    match result {
        Ok(failures) if failures.is_empty() => {}
        Ok(failures) => {
            // Write to stdout (not stderr) so pipeline-check can capture the details.
            // Join with "; " so the cascade detection in pipeline-check can parse
            // multi-failure output as a single detail string.
            print!("{}", failures.join("; "));
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn validate_worklog(
    repo_root: &Path,
    file: &Path,
    cycle: u64,
    pipeline_status: Option<&str>,
    worklog_commit: Option<&str>,
) -> Result<Vec<String>, String> {
    validate_worklog_with_options(
        repo_root,
        file,
        cycle,
        pipeline_status,
        worklog_commit,
        WorklogValidationOptions::default(),
    )
}

fn validate_worklog_with_options(
    repo_root: &Path,
    file: &Path,
    cycle: u64,
    pipeline_status: Option<&str>,
    worklog_commit: Option<&str>,
    options: WorklogValidationOptions,
) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(file)
        .map_err(|error| format!("failed to read {}: {}", file.display(), error))?;
    let state = read_state_json(repo_root)?;
    let mut failures = Vec::new();

    if let Some(failure) = validate_in_flight_count_line(&content) {
        failures.push(failure);
    }
    if !options.exclude_post_c3_fields {
        // The worklog is written at C3, before review dispatch mutates docs/state.json during
        // C5-C7. When callers opt into full validation we still compare the numeric value, but
        // close-out validation defaults to skipping this inherently stale state-vs-worklog check.
        let expected_in_flight = count_in_flight_sessions(&state);
        if let Some(failure) = validate_in_flight_count(&content, expected_in_flight) {
            failures.push(failure);
        }
    }

    let cycle_receipt_through = find_cycle_complete_timestamp(repo_root, cycle)?;
    match fetch_cycle_receipts(repo_root, cycle, Some(cycle_receipt_through.as_str())) {
        Ok(expected_receipts) => {
            match validate_receipt_completeness(repo_root, cycle, &content, &expected_receipts) {
                Ok(receipt_failures) => failures.extend(receipt_failures),
                Err(error) => {
                    failures.push(format!("unable to validate commit receipts: {}", error))
                }
            }
        }
        Err(error) => failures.push(format!("unable to validate commit receipts: {}", error)),
    }

    let diff_end_commit = resolve_worklog_commit(repo_root, file, worklog_commit)?;
    match changed_infrastructure_paths(repo_root, cycle, diff_end_commit.as_deref()) {
        Ok(changed_paths) => {
            if let Some(finding) = validate_self_modifications_section(&content, &changed_paths) {
                match finding {
                    SelfModificationFinding::Failure(failure) => failures.push(failure),
                    SelfModificationFinding::Warning(warning) => {
                        eprintln!("Warning: {}", warning);
                    }
                }
            }
        }
        Err(error) => failures.push(format!("unable to validate self-modifications: {}", error)),
    }

    if let Some(failure) = validate_pipeline_status_line(&content) {
        failures.push(failure);
    }
    let is_preliminary = worklog_marks_pipeline_status_as_preliminary(&content);
    if !options.exclude_post_c3_fields || !is_preliminary {
        // C5.5 runs after the C3 snapshot is written. If the worklog explicitly marks the
        // pre-dispatch pipeline line as preliminary, comparing it against the final gate result
        // would turn a known timing gap into a deterministic validation failure.
        match resolve_pipeline_status(repo_root, cycle, pipeline_status, fetch_pipeline_report) {
            Ok(overall) => {
                if let Some(failure) = validate_pipeline_status(&content, &overall) {
                    failures.push(failure);
                }
            }
            Err(error) => failures.push(format!("unable to validate pipeline status: {}", error)),
        }
    }

    Ok(failures)
}

fn validate_journal(repo_root: &Path, file: &Path) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(file)
        .map_err(|error| format!("failed to read {}: {}", file.display(), error))?;
    let mut failures = Vec::new();

    failures.extend(validate_journal_headings(&content));
    failures.extend(validate_no_duplicate_cycles(&content));
    failures.extend(validate_no_duplicate_section_headers(&content));
    failures.extend(validate_no_escaped_newlines(&content));
    failures.extend(validate_worklog_links(&content, file));
    if let Some(failure) = validate_commitment_section(&content) {
        failures.push(failure);
    }
    if let Some(failure) = validate_open_questions_consistency(&content) {
        failures.push(failure);
    }
    // State-aware consistency: when docs/state.json lists pending
    // Eva questions, the latest journal entry's Open questions
    // section MUST NOT say "None". This closes the cycle 501 F2 /
    // cycle 502 F1 recurrence (journal claimed "None" while state
    // carried 7 pending question-for-eva issues, including the
    // blocker named in the body). A state read failure does NOT
    // produce a validation failure — the text-only check above
    // remains authoritative in that case, and upstream tooling
    // will surface the config/state read failure through other
    // channels.
    if let Ok(state) = read_state_json(repo_root) {
        if let Some(failure) = validate_open_questions_against_state(&content, &state) {
            failures.push(failure);
        }
    }

    Ok(failures)
}

fn read_state_json(repo_root: &Path) -> Result<StateJson, String> {
    let state_path = repo_root.join("docs/state.json");
    let content = fs::read_to_string(&state_path)
        .map_err(|error| format!("failed to read {}: {}", state_path.display(), error))?;
    serde_json::from_str(&content)
        .map_err(|error| format!("failed to parse {}: {}", state_path.display(), error))
}

fn count_in_flight_sessions(state: &StateJson) -> usize {
    state
        .agent_sessions
        .iter()
        .filter(|session| session.status.as_deref() == Some("in_flight"))
        .count()
}

fn parse_in_flight_count(content: &str) -> Result<usize, String> {
    let reported = match extract_markdown_value(content, "In-flight agent sessions (post-dispatch)")
        .or_else(|| extract_markdown_value(content, "In-flight agent sessions"))
    {
        Some(reported) => reported,
        None => {
            return Err(
            "worklog is missing the 'In-flight agent sessions' line in the Pre-dispatch state section"
                .to_string(),
            )
        }
    };
    reported.parse::<usize>().map_err(|_| {
        format!(
            "in-flight agent sessions line must contain an integer count, found '{}'",
            reported
        )
    })
}

fn validate_in_flight_count_line(content: &str) -> Option<String> {
    parse_in_flight_count(content).err()
}

fn validate_in_flight_count(content: &str, expected: usize) -> Option<String> {
    let parsed = match parse_in_flight_count(content) {
        Ok(parsed) => parsed,
        Err(failure) => return Some(failure),
    };
    if parsed == expected {
        return None;
    }

    Some(format!(
        "in-flight agent sessions mismatch: worklog reports {}, state.json has {}",
        parsed, expected
    ))
}

fn fetch_cycle_receipts(
    repo_root: &Path,
    cycle: u64,
    through: Option<&str>,
) -> Result<Vec<ReceiptEntry>, String> {
    let mut args = vec![
        "--cycle".to_string(),
        cycle.to_string(),
        "--json".to_string(),
    ];
    if let Some(timestamp) = through {
        args.push("--through".to_string());
        args.push(timestamp.to_string());
    }
    args.push("--repo-root".to_string());
    args.push(repo_root.display().to_string());
    let output = run_wrapper(repo_root, "tools/cycle-receipts", &args)?;
    serde_json::from_str::<Vec<ReceiptEntry>>(&output)
        .map_err(|error| format!("failed to parse cycle-receipts JSON: {}", error))
}

fn validate_receipt_completeness(
    repo_root: &Path,
    cycle: u64,
    content: &str,
    expected: &[ReceiptEntry],
) -> Result<Vec<String>, String> {
    let required_receipts = filter_receipts_through_cycle_complete(repo_root, cycle, expected)?;
    let present = extract_present_receipts(content);
    let missing = required_receipts
        .iter()
        .filter_map(|entry| {
            let receipt = entry.receipt.trim();
            (!receipt.is_empty() && !present.contains(receipt)).then(|| receipt.to_string())
        })
        .collect::<Vec<_>>();

    if missing.is_empty() {
        return Ok(Vec::new());
    }

    Ok(vec![format!(
        "commit receipts section is missing required receipt(s): {}",
        missing.join(", ")
    )])
}

fn filter_receipts_through_cycle_complete(
    repo_root: &Path,
    cycle: u64,
    expected: &[ReceiptEntry],
) -> Result<Vec<ReceiptEntry>, String> {
    let cycle_complete_commit = find_cycle_complete_commit(repo_root, cycle)?;
    let mut filtered = Vec::new();

    for entry in expected {
        let sha = entry.best_sha();
        if sha.is_empty() {
            continue;
        }
        if is_ancestor_commit(repo_root, sha, &cycle_complete_commit)? {
            filtered.push(entry.clone());
        }
    }

    Ok(filtered)
}

fn extract_present_receipts(content: &str) -> BTreeSet<String> {
    let Some(section) = extract_section_body(content, COMMIT_RECEIPTS_HEADING) else {
        return BTreeSet::new();
    };

    section
        .lines()
        .filter(|line| line.trim_start().starts_with('|'))
        .filter(|line| !line.contains("------") && !line.contains("Receipt |"))
        .filter_map(|line| {
            let cells = line
                .split('|')
                .map(str::trim)
                .filter(|cell| !cell.is_empty())
                .collect::<Vec<_>>();
            cells
                .get(1)
                .and_then(|cell| extract_receipt_from_cell(cell))
        })
        .collect()
}

fn extract_receipt_from_cell(cell: &str) -> Option<String> {
    if let Some(start) = cell.find('`') {
        let end = cell[start + 1..].find('`')?;
        let receipt = &cell[start + 1..start + 1 + end];
        return is_short_hex(receipt).then(|| receipt.to_string());
    }

    let trimmed = cell.trim_matches(|character| matches!(character, '[' | ']'));
    is_short_hex(trimmed).then(|| trimmed.to_string())
}

fn is_short_hex(value: &str) -> bool {
    value.len() >= 7 && value.chars().all(|character| character.is_ascii_hexdigit())
}

fn changed_infrastructure_paths(
    repo_root: &Path,
    cycle: u64,
    end_commit: Option<&str>,
) -> Result<Vec<String>, String> {
    let start_commit = find_cycle_start_commit(repo_root, cycle)?;
    let diff_range = match end_commit {
        Some(commit) => format!("{start_commit}..{commit}"),
        None => format!("{start_commit}..HEAD"),
    };
    let mut args = vec![
        "diff".to_string(),
        "--name-only".to_string(),
        diff_range,
        "--".to_string(),
    ];
    args.extend(INFRASTRUCTURE_PATHS.iter().map(|path| path.to_string()));
    let output = run_git(repo_root, &args)?;

    let mut paths = output
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_string)
        .collect::<Vec<_>>();
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn resolve_worklog_commit(
    repo_root: &Path,
    file: &Path,
    worklog_commit: Option<&str>,
) -> Result<Option<String>, String> {
    match worklog_commit {
        Some(commit) => Ok(Some(commit.to_string())),
        None => find_last_modifying_commit(repo_root, file),
    }
}

fn find_last_modifying_commit(repo_root: &Path, file: &Path) -> Result<Option<String>, String> {
    let pathspec = file
        .strip_prefix(repo_root)
        .unwrap_or(file)
        .to_string_lossy()
        .into_owned();
    let output = run_git(
        repo_root,
        &[
            "log".to_string(),
            "-n".to_string(),
            "1".to_string(),
            "--format=%H".to_string(),
            "--".to_string(),
            pathspec,
        ],
    )?;
    let commit = output.trim();
    if commit.is_empty() {
        return Ok(None);
    }

    Ok(Some(commit.to_string()))
}

fn find_cycle_start_commit(repo_root: &Path, cycle: u64) -> Result<String, String> {
    let pattern = format!(r"\[cycle {}\]", cycle);
    let output = run_git(
        repo_root,
        &[
            "log".to_string(),
            "-n".to_string(),
            "1".to_string(),
            "--format=%H".to_string(),
            "--grep".to_string(),
            "^state(cycle-start):".to_string(),
            "--grep".to_string(),
            pattern,
            "--all-match".to_string(),
        ],
    )?;
    let commit = output.trim();
    if commit.is_empty() {
        return Err(format!(
			"could not find cycle-start commit for cycle {}; verify the cycle number is correct and that the cycle has started; fetch more history if this is a shallow clone",
			cycle
		));
    }

    Ok(commit.to_string())
}

fn find_cycle_complete_commit(repo_root: &Path, cycle: u64) -> Result<String, String> {
    let pattern = format!(r"\[cycle {}\]", cycle);
    let output = run_git(
        repo_root,
        &[
            "log".to_string(),
            "-n".to_string(),
            "1".to_string(),
            "--format=%H".to_string(),
            "--grep".to_string(),
            "^state(cycle-complete):".to_string(),
            "--grep".to_string(),
            pattern,
            "--all-match".to_string(),
        ],
    )?;
    let commit = output.trim();
    if commit.is_empty() {
        return Err(format!(
            "could not find cycle-complete commit for cycle {}; verify the cycle number is correct and that the cycle has completed; fetch more history if this is a shallow clone",
            cycle
        ));
    }

    Ok(commit.to_string())
}

fn find_cycle_complete_timestamp(repo_root: &Path, cycle: u64) -> Result<String, String> {
    let pattern = format!(r"\[cycle {}\]", cycle);
    let output = run_git(
        repo_root,
        &[
            "log".to_string(),
            "-n".to_string(),
            "1".to_string(),
            "--format=%cI".to_string(),
            "--grep".to_string(),
            "^state(cycle-complete):".to_string(),
            "--grep".to_string(),
            pattern,
            "--all-match".to_string(),
        ],
    )?;
    let timestamp = output.trim();
    if timestamp.is_empty() {
        return Err(format!(
            "could not find cycle-complete timestamp for cycle {}; verify the cycle number is correct and that the cycle has completed; fetch more history if this is a shallow clone",
            cycle
        ));
    }

    Ok(timestamp.to_string())
}

fn is_ancestor_commit(repo_root: &Path, ancestor: &str, descendant: &str) -> Result<bool, String> {
    let output = run_git_output(
        repo_root,
        &["merge-base", "--is-ancestor", ancestor, descendant],
    )?;
    match output.status.code() {
        Some(0) => Ok(true),
        Some(1) => Ok(false),
        _ => {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            Err(format!(
                "git merge-base --is-ancestor {} {} failed: {}",
                ancestor, descendant, stderr
            ))
        }
    }
}

fn validate_self_modifications_section(
    content: &str,
    changed_paths: &[String],
) -> Option<SelfModificationFinding> {
    let section = extract_section_body(content, SELF_MODIFICATIONS_HEADING)?;
    let reports_none = section.lines().any(reports_no_self_modifications);
    if !reports_none || changed_paths.is_empty() {
        let omitted = summarize_infrastructure_groups(changed_paths)
            .into_iter()
            .filter(|path| !section_mentions_path(section, path))
            .collect::<Vec<_>>();
        return (!omitted.is_empty()).then(|| {
            SelfModificationFinding::Warning(format!(
                "self-modifications section omits changed infrastructure path(s): {}",
                omitted.join(", ")
            ))
        });
    }

    Some(SelfModificationFinding::Failure(format!(
        "self-modifications section says None, but infrastructure changes exist: {}",
        changed_paths.join(", ")
    )))
}

fn summarize_infrastructure_groups(changed_paths: &[String]) -> Vec<String> {
    let mut groups = changed_paths
        .iter()
        .map(|path| summarize_infrastructure_path(path))
        .collect::<Vec<_>>();
    groups.sort();
    groups.dedup();
    groups
}

fn summarize_infrastructure_path(path: &str) -> String {
    if path.starts_with("tools/") {
        return "tools/".to_string();
    }
    if path.starts_with(".claude/skills/") {
        return ".claude/skills/".to_string();
    }
    if INFRASTRUCTURE_PATHS.contains(&path) {
        return path.to_string();
    }
    path.to_string()
}

fn section_mentions_path(section: &str, path: &str) -> bool {
    section
        .to_ascii_lowercase()
        .contains(&path.to_ascii_lowercase())
}

fn fetch_pipeline_report(repo_root: &Path, cycle: u64) -> Result<PipelineReport, String> {
    let args = pipeline_check_args(repo_root, cycle);
    let script_path = repo_root.join("tools/pipeline-check");
    let output = ProcessCommand::new("bash")
        .arg(&script_path)
        .args(&args)
        .output()
        .map_err(|error| format!("failed to execute {}: {}", script_path.display(), error))?;
    // pipeline-check exits 1 on FAIL but still outputs valid JSON — parse stdout regardless
    let stdout = String::from_utf8(output.stdout).map_err(|error| {
        format!(
            "failed to decode {} output as UTF-8: {}",
            script_path.display(),
            error
        )
    })?;
    serde_json::from_str::<PipelineReport>(&stdout)
        .map_err(|error| format!("failed to parse pipeline-check JSON: {}", error))
}

fn pipeline_check_args(repo_root: &Path, cycle: u64) -> Vec<String> {
    vec![
        "--json".to_string(),
        "--cycle".to_string(),
        cycle.to_string(),
        "--repo-root".to_string(),
        repo_root.display().to_string(),
        "--exclude-step".to_string(),
        "doc-validation".to_string(),
        "--exclude-step".to_string(),
        "current-cycle-steps".to_string(),
        // Exclude the same systemic checks that the C5.5 gate excludes.
        // These create circular dependencies and are informational only.
        "--exclude-step".to_string(),
        "deferral-accumulation".to_string(),
        "--exclude-step".to_string(),
        "chronic-category-currency".to_string(),
    ]
}

fn resolve_pipeline_status<F>(
    repo_root: &Path,
    cycle: u64,
    pipeline_status: Option<&str>,
    fetch_report: F,
) -> Result<String, String>
where
    F: FnOnce(&Path, u64) -> Result<PipelineReport, String>,
{
    match pipeline_status {
        Some(status) => Ok(status.to_string()),
        None => fetch_report(repo_root, cycle).map(|report| report.overall),
    }
}

fn pipeline_status_value(content: &str) -> Result<String, String> {
    let reported = extract_markdown_value(content, "Pipeline status").ok_or_else(|| {
        "worklog is missing the 'Pipeline status' line in the Pre-dispatch state section"
            .to_string()
    })?;
    if reported.starts_with("PASS") || reported.starts_with("FAIL") {
        Ok(reported.to_string())
    } else {
        Err(format!(
            "pipeline status line must start with PASS or FAIL, found '{}'",
            reported
        ))
    }
}

fn validate_pipeline_status_line(content: &str) -> Option<String> {
    pipeline_status_value(content).err()
}

fn worklog_marks_pipeline_status_as_preliminary(content: &str) -> bool {
    content
        .lines()
        .take(50)
        .any(|line| line.trim() == PRE_DISPATCH_SNAPSHOT_NOTE)
}

fn validate_pipeline_status(content: &str, overall: &str) -> Option<String> {
    let reported = match pipeline_status_value(content) {
        Ok(reported) => reported,
        Err(failure) => return Some(failure),
    };
    let expected = if overall.eq_ignore_ascii_case("pass") {
        "PASS"
    } else {
        "FAIL"
    };
    let reported_status = if reported.starts_with("FAIL→PASS") || reported.starts_with("PASS") {
        "PASS"
    } else if reported.starts_with("FAIL") {
        "FAIL"
    } else {
        "OTHER"
    };

    (reported_status != expected).then(|| {
        format!(
            "pipeline status mismatch: worklog reports '{}', pipeline-check overall is '{}'",
            reported, overall
        )
    })
}

fn validate_journal_headings(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            heading_level(trimmed)?;
            let cycle = extract_first_cycle_number(trimmed)?;
            let duplicate = format!("Cycle {}: Cycle {}:", cycle, cycle);
            trimmed.contains(&duplicate).then(|| {
                format!(
                    "duplicated cycle prefix in heading '{}': remove repeated 'Cycle {}:'",
                    trimmed, cycle
                )
            })
        })
        .collect()
}

fn validate_worklog_links(content: &str, journal_file: &Path) -> Vec<String> {
    extract_cycle_links(content)
        .into_iter()
        .filter_map(|(cycle, link)| validate_worklog_link(journal_file, cycle, &link))
        .collect()
}

fn validate_no_duplicate_cycles(content: &str) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut failures = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if heading_level(trimmed) != Some(2) {
            continue;
        }
        let Some(cycle) = extract_first_cycle_number(trimmed) else {
            continue;
        };
        if !seen.insert(cycle) {
            failures.push(format!(
                "duplicate journal entry: cycle {} appears more than once",
                cycle
            ));
        }
    }

    failures
}

fn validate_no_duplicate_section_headers(content: &str) -> Vec<String> {
    let mut failures = Vec::new();
    let mut current_cycle = None;
    let mut seen_section_headers = BTreeSet::new();

    for line in content.lines() {
        let trimmed = line.trim();
        match heading_level(trimmed) {
            Some(2) => {
                current_cycle = extract_first_cycle_number(trimmed);
                seen_section_headers.clear();
            }
            Some(3) => {
                let Some(cycle) = current_cycle else {
                    continue;
                };
                if !seen_section_headers.insert(trimmed.to_string()) {
                    failures.push(format!(
                        "cycle {} contains duplicate section heading '{}'",
                        cycle, trimmed
                    ));
                }
            }
            _ => {}
        }
    }

    failures
}

fn validate_no_escaped_newlines(content: &str) -> Vec<String> {
    let mut failures = Vec::new();
    let mut in_code_block = false;

    for (line_num, line) in content.lines().enumerate() {
        if line.trim_start().starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }
        if !in_code_block && line.contains("\\n") {
            failures.push(format!(
                "line {}: contains literal escaped newline characters (\\n)",
                line_num + 1
            ));
        }
    }

    failures
}

fn validate_worklog_link(journal_file: &Path, cycle: u64, link: &str) -> Option<String> {
    if !link.starts_with("../worklog/") {
        return Some(format!(
            "worklog link for cycle {} must use ../worklog/... relative path, found '{}'",
            cycle, link
        ));
    }

    let parent = journal_file.parent()?;
    let resolved = parent.join(link);
    (!resolved.is_file()).then(|| {
        format!(
            "worklog link for cycle {} does not resolve from {}: {}",
            cycle,
            journal_file.display(),
            link
        )
    })
}

fn extract_cycle_links(content: &str) -> Vec<(u64, String)> {
    let mut matches = Vec::new();
    let mut remainder = content;

    while let Some(start) = remainder.find("[cycle ") {
        let link_start = start + "[cycle ".len();
        let after_prefix = &remainder[link_start..];
        let digits_len = after_prefix
            .chars()
            .take_while(|character| character.is_ascii_digit())
            .count();
        if digits_len == 0 {
            remainder = &after_prefix[1..];
            continue;
        }

        let cycle = match after_prefix[..digits_len].parse::<u64>() {
            Ok(cycle) => cycle,
            Err(_) => {
                remainder = &after_prefix[digits_len..];
                continue;
            }
        };
        let after_digits = &after_prefix[digits_len..];
        if !after_digits.starts_with("](") {
            remainder = after_digits;
            continue;
        }
        let path_start = &after_digits[2..];
        let Some(end) = path_start.find(')') else {
            break;
        };
        matches.push((cycle, path_start[..end].to_string()));
        remainder = &path_start[end + 1..];
    }

    matches
}

fn validate_commitment_section(content: &str) -> Option<String> {
    let section = match extract_section_body(content, CONCRETE_COMMITMENTS_HEADING) {
        Some(section) => section,
        None => return Some(
            "journal entry is missing a non-empty 'Concrete commitments for next cycle' section"
                .to_string(),
        ),
    };
    (!section
        .lines()
        .map(normalize_line)
        .any(|line| !line.is_empty()))
    .then(|| {
        "journal entry is missing a non-empty 'Concrete commitments for next cycle' section"
            .to_string()
    })
}

/// Detects the journal-quality contradiction caught by cycle 458 review F4:
/// the entry body says it filed a question-for-eva, but the trailing
/// "Open questions" section claims there are none.
///
/// Journal files contain multiple cycle entries (one per `## YYYY-MM-DD —
/// Cycle N:` heading). Only the LATEST cycle entry is checked, so historical
/// entries with already-shipped contradictions don't block future cycles.
///
/// Pure text check, no GitHub queries — chronic-category structural fix.
fn validate_open_questions_consistency(content: &str) -> Option<String> {
    // Scope to the most recent cycle entry. Look for the first `## ` heading
    // and slice from there to the next `## ` heading. If no cycle headings
    // exist, fall back to checking the whole content (for the test fixtures).
    let entry = latest_cycle_entry(content).unwrap_or(content);

    let (heading, section) = extract_open_questions_section(entry)?;

    let bullet_lines = section_bullet_lines(section);
    if bullet_lines.is_empty() {
        return None;
    }
    let says_none = section_reports_none(&bullet_lines);
    if !says_none {
        return None;
    }

    let body_before = match entry.find(heading) {
        Some(idx) => &entry[..idx],
        None => return None,
    };
    let lowered = body_before.to_lowercase();
    let mentions_filing = lowered.contains("filed question-for-eva")
        || lowered.contains("filing question-for-eva")
        || lowered.contains("file question-for-eva");
    if !mentions_filing {
        return None;
    }

    Some(
        "journal entry's 'Open questions' section says 'None' but the body references filing a question-for-eva — list the open question or rephrase the body"
            .to_string(),
    )
}

/// State-aware complement to `validate_open_questions_consistency`:
/// fails when docs/state.json lists pending Eva questions but the
/// latest journal entry's Open questions section claims there are
/// none.
///
/// Catches the cycle 501 F2 / cycle 502 F1 recurrence: the text-only
/// check requires the body to say "filed question-for-eva" (narrative
/// pattern), but the real-world pattern is that journals name the
/// blocker issue by number without using that specific phrase. As
/// long as `state.open_questions_for_eva` is non-empty, a journal
/// "None" claim is objectively wrong regardless of phrasing in the
/// body.
fn validate_open_questions_against_state(content: &str, state: &StateJson) -> Option<String> {
    if state.open_questions_for_eva.is_empty() {
        return None;
    }

    let entry = latest_cycle_entry(content).unwrap_or(content);
    let Some((_, section)) = extract_open_questions_section(entry) else {
        return None;
    };
    let bullet_lines = section_bullet_lines(section);
    if bullet_lines.is_empty() {
        return None;
    }
    let says_none = section_reports_none(&bullet_lines);
    if !says_none {
        return None;
    }

    if standing_eva_blockers_cover_state(entry, state) {
        return None;
    }

    let issue_refs: Vec<String> = state
        .open_questions_for_eva
        .iter()
        .filter_map(|value| value.as_u64().map(|n| format!("#{}", n)))
        .collect();
    let summary = if issue_refs.is_empty() {
        format!(
            "{} pending question-for-eva",
            state.open_questions_for_eva.len()
        )
    } else {
        issue_refs.join(", ")
    };

    Some(format!(
        "journal entry's 'Open questions' section says 'None' but docs/state.json lists pending question-for-eva: {} — list the open questions or resolve them before writing 'None'",
        summary
    ))
}

fn extract_open_questions_section(entry: &str) -> Option<(&'static str, &str)> {
    extract_section_body(entry, OPEN_QUESTIONS_RAISED_THIS_CYCLE_HEADING)
        .map(|section| (OPEN_QUESTIONS_RAISED_THIS_CYCLE_HEADING, section))
        .or_else(|| {
            extract_section_body(entry, OPEN_QUESTIONS_HEADING)
                .map(|section| (OPEN_QUESTIONS_HEADING, section))
        })
}

fn section_bullet_lines(section: &str) -> Vec<&str> {
    section
        .lines()
        .map(normalize_line)
        .filter(|line| !line.is_empty())
        .collect()
}

fn section_reports_none(lines: &[&str]) -> bool {
    lines.iter().all(|line| reports_no_self_modifications(line))
}

fn standing_eva_blockers_cover_state(entry: &str, state: &StateJson) -> bool {
    let Some(section) = extract_section_body(entry, STANDING_EVA_BLOCKERS_HEADING) else {
        return false;
    };
    let bullet_lines = section_bullet_lines(section);
    if bullet_lines.is_empty() || section_reports_none(&bullet_lines) {
        return false;
    }

    state
        .open_questions_for_eva
        .iter()
        .filter_map(|value| value.as_u64())
        .all(|number| section.contains(&format!("#{}", number)))
}

/// Returns the slice of `content` covering the most recent cycle entry,
/// delimited by H2 headings (`## `). Returns None if no H2 headings are
/// found in the content.
fn latest_cycle_entry(content: &str) -> Option<&str> {
    let first_h2 = content.find("\n## ").map(|i| i + 1).or_else(|| {
        if content.starts_with("## ") {
            Some(0)
        } else {
            None
        }
    })?;
    let after_first = &content[first_h2..];
    let next_h2 = after_first[3..].find("\n## ").map(|i| i + 3);
    let end = next_h2.unwrap_or(after_first.len());
    Some(&after_first[..end])
}

fn extract_markdown_value<'a>(content: &'a str, label: &str) -> Option<&'a str> {
    let prefix = format!("- **{}**:", label);
    content.lines().find_map(|line| {
        line.trim()
            .strip_prefix(&prefix)
            .map(str::trim)
            .filter(|value| !value.is_empty())
    })
}

fn extract_section_body<'a>(content: &'a str, heading: &str) -> Option<&'a str> {
    let target_level = heading_level(heading)?;
    let mut section_start = None;
    let mut offset = 0usize;

    for line in content.split_inclusive('\n') {
        let trimmed = line.trim_end_matches(['\r', '\n']);
        if let Some(start) = section_start {
            if let Some(level) = heading_level(trimmed) {
                if level <= target_level {
                    return Some(content[start..offset].trim());
                }
            }
        } else if trimmed == heading {
            section_start = Some(offset + line.len());
        }

        offset += line.len();
    }

    section_start.map(|start| content[start..].trim())
}

fn heading_level(line: &str) -> Option<usize> {
    let level = line
        .chars()
        .take_while(|character| *character == '#')
        .count();
    (level > 0 && line.as_bytes().get(level) == Some(&b' ')).then_some(level)
}

fn extract_first_cycle_number(line: &str) -> Option<u64> {
    let start = line.find("Cycle ")? + "Cycle ".len();
    let digits = line[start..]
        .chars()
        .take_while(|character| character.is_ascii_digit())
        .collect::<String>();
    (!digits.is_empty())
        .then(|| digits.parse::<u64>().ok())
        .flatten()
}

fn normalize_line(line: &str) -> &str {
    line.trim_matches(['\r', '\n', ' '])
}

fn reports_no_self_modifications(line: &str) -> bool {
    let trimmed = normalize_line(line);
    let without_bullet = trimmed
        .strip_prefix("- ")
        .or_else(|| trimmed.strip_prefix("* "))
        .unwrap_or(trimmed)
        .trim();
    let without_period = without_bullet.trim_end_matches('.');
    without_period.eq_ignore_ascii_case("none")
}

fn run_wrapper(
    repo_root: &Path,
    wrapper_relative_path: &str,
    args: &[String],
) -> Result<String, String> {
    let script_path = repo_root.join(wrapper_relative_path);
    let output = ProcessCommand::new("bash")
        .arg(&script_path)
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute {}: {}", script_path.display(), error))?;
    if !output.status.success() {
        return Err(command_failure_message(script_path.as_path(), &output));
    }

    String::from_utf8(output.stdout).map_err(|error| {
        format!(
            "failed to decode {} output as UTF-8: {}",
            script_path.display(),
            error
        )
    })
}

fn run_git(repo_root: &Path, args: &[String]) -> Result<String, String> {
    let borrowed_args = args.iter().map(String::as_str).collect::<Vec<_>>();
    let output = run_git_output(repo_root, &borrowed_args)?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!("git {} failed: {}", args.join(" "), stderr));
    }

    String::from_utf8(output.stdout).map_err(|error| {
        format!(
            "failed to decode git {} output as UTF-8: {}",
            args.join(" "),
            error
        )
    })
}

fn run_git_output(repo_root: &Path, args: &[&str]) -> Result<std::process::Output, String> {
    ProcessCommand::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute git {}: {}", args.join(" "), error))
}

fn command_failure_message(script_path: &Path, output: &std::process::Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let mut details = Vec::new();
    if !stderr.is_empty() {
        details.push(format!("stderr: {}", stderr));
    }
    if !stdout.is_empty() {
        details.push(format!("stdout: {}", stdout));
    }
    if details.is_empty() {
        details.push("no output captured".to_string());
    }

    format!("{} failed: {}", script_path.display(), details.join("; "))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    struct TestDir {
        path: PathBuf,
    }

    impl TestDir {
        fn new() -> Self {
            let unique = format!(
                "validate-docs-test-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("clock before epoch")
                    .as_nanos()
            );
            let path = std::env::temp_dir().join(unique);
            fs::create_dir_all(&path).expect("create temp dir");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }
    }

    impl Drop for TestDir {
        fn drop(&mut self) {
            if let Err(error) = fs::remove_dir_all(&self.path) {
                eprintln!(
                    "Warning: failed to remove test directory {}: {}",
                    self.path.display(),
                    error
                );
            }
        }
    }

    #[test]
    fn detects_in_flight_mismatch() {
        let content = "\
## Pre-dispatch state

- **In-flight agent sessions**: 4
";
        let failure = validate_in_flight_count(content, 1).expect("expected mismatch");
        assert!(failure.contains("worklog reports 4"));
        assert!(failure.contains("state.json has 1"));
    }

    #[test]
    fn prefers_post_dispatch_in_flight_value_when_present() {
        let content = "\
## Pre-dispatch state

- **In-flight agent sessions**: 2
- **In-flight agent sessions (post-dispatch)**: 3
";
        assert_eq!(validate_in_flight_count(content, 3), None);
    }

    #[test]
    fn detects_post_dispatch_in_flight_mismatch() {
        let content = "\
## Pre-dispatch state

- **In-flight agent sessions**: 2
- **In-flight agent sessions (post-dispatch)**: 3
";
        let failure = validate_in_flight_count(content, 4).expect("expected mismatch");
        assert!(failure.contains("worklog reports 3"));
        assert!(failure.contains("state.json has 4"));
    }

    #[test]
    fn detects_pipeline_status_mismatch() {
        let content = "\
## Pre-dispatch state

- **Pipeline status**: PASS (8/8)
";
        let failure = validate_pipeline_status(content, "warn").expect("expected mismatch");
        assert!(failure.contains("pipeline status mismatch"));
    }

    #[test]
    fn fail_to_pass_pipeline_status_counts_as_final_pass() {
        let content = "\
## Cycle state

- **Pipeline status**: FAIL→PASS (C5.5 initially failed: doc-validation; resolved by re-running close-out after fixes)
";
        assert!(validate_pipeline_status(content, "pass").is_none());
    }

    #[test]
    fn preliminary_pipeline_status_note_skips_final_status_comparison() {
        let content = "\
## Pre-dispatch state

*Snapshot before review dispatch — final counters may differ after C6.*
- **Pipeline status**: PASS (8/8)
";
        assert!(worklog_marks_pipeline_status_as_preliminary(content));
    }

    #[test]
    fn provided_pipeline_status_skips_fetching_pipeline_report() {
        let fetch_called = std::cell::Cell::new(false);
        let status =
            resolve_pipeline_status(Path::new("."), 226, Some("pass"), |_repo_root, cycle| {
                fetch_called.set(true);
                assert_eq!(cycle, 226);
                Ok(PipelineReport {
                    overall: "fail".to_string(),
                })
            })
            .expect("pipeline status should resolve");

        assert_eq!(status, "pass");
        assert!(
            !fetch_called.get(),
            "pipeline-check should not be invoked when --pipeline-status is provided"
        );
    }

    #[test]
    fn missing_pipeline_status_fetches_pipeline_report() {
        let status = resolve_pipeline_status(Path::new("."), 226, None, |_repo_root, cycle| {
            assert_eq!(cycle, 226);
            Ok(PipelineReport {
                overall: "fail".to_string(),
            })
        })
        .expect("pipeline status should resolve");

        assert_eq!(status, "fail");
    }

    #[test]
    fn pipeline_check_args_exclude_doc_validation() {
        let args = pipeline_check_args(Path::new("/tmp/repo"), 226);
        assert_eq!(
            args,
            vec![
                "--json".to_string(),
                "--cycle".to_string(),
                "226".to_string(),
                "--repo-root".to_string(),
                "/tmp/repo".to_string(),
                "--exclude-step".to_string(),
                "doc-validation".to_string(),
                "--exclude-step".to_string(),
                "current-cycle-steps".to_string(),
                "--exclude-step".to_string(),
                "deferral-accumulation".to_string(),
                "--exclude-step".to_string(),
                "chronic-category-currency".to_string(),
            ]
        );
    }

    #[test]
    fn detects_missing_receipts() {
        let repo = TestRepo::new();
        repo.init();
        let first_receipt = repo.commit(
            "notes/first.txt",
            "first\n",
            "state(process-merge): first merge [cycle 226]",
        );
        let missing_receipt = repo.commit(
            "notes/second.txt",
            "second\n",
            "state(process-review): second review [cycle 226]",
        );
        let cycle_complete_receipt = repo.commit(
            "notes/complete.txt",
            "complete\n",
            "state(cycle-complete): close cycle [cycle 226]",
        );
        let content = receipts_table(&[&first_receipt, &cycle_complete_receipt]);
        let failures = validate_receipt_completeness(
            repo.path(),
            226,
            &content,
            &[
                ReceiptEntry {
                    url: String::new(),
                    receipt: first_receipt,
                },
                ReceiptEntry {
                    url: String::new(),
                    receipt: missing_receipt.clone(),
                },
                ReceiptEntry {
                    url: String::new(),
                    receipt: cycle_complete_receipt,
                },
            ],
        )
        .expect("receipt validation should succeed");
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains(&missing_receipt));
    }

    #[test]
    fn ignores_receipts_after_cycle_complete() {
        let repo = TestRepo::new();
        repo.init();
        let included_receipt = repo.commit(
            "notes/merge.txt",
            "merged\n",
            "state(process-merge): merge work [cycle 226]",
        );
        let cycle_complete_receipt = repo.commit(
            "notes/complete.txt",
            "complete\n",
            "state(cycle-complete): close cycle [cycle 226]",
        );
        let excluded_receipt = repo.commit(
            "docs/state.json",
            "{}\n",
            "state(record-dispatch): #123 dispatched [cycle 226]",
        );
        let content = receipts_table(&[&included_receipt, &cycle_complete_receipt]);

        let failures = validate_receipt_completeness(
            repo.path(),
            226,
            &content,
            &[
                ReceiptEntry {
                    url: String::new(),
                    receipt: included_receipt,
                },
                ReceiptEntry {
                    url: String::new(),
                    receipt: cycle_complete_receipt,
                },
                ReceiptEntry {
                    url: String::new(),
                    receipt: excluded_receipt,
                },
            ],
        )
        .expect("receipt validation should succeed");

        assert!(failures.is_empty());
    }

    #[test]
    fn still_requires_receipts_up_to_cycle_complete() {
        let repo = TestRepo::new();
        repo.init();
        let required_receipt = repo.commit(
            "notes/merge.txt",
            "merged\n",
            "state(process-merge): merge work [cycle 226]",
        );
        let cycle_complete_receipt = repo.commit(
            "notes/complete.txt",
            "complete\n",
            "state(cycle-complete): close cycle [cycle 226]",
        );
        repo.commit(
            "docs/state.json",
            "{}\n",
            "state(record-dispatch): #123 dispatched [cycle 226]",
        );
        let content = receipts_table(&[&cycle_complete_receipt]);

        let failures = validate_receipt_completeness(
            repo.path(),
            226,
            &content,
            &[
                ReceiptEntry {
                    url: String::new(),
                    receipt: required_receipt.clone(),
                },
                ReceiptEntry {
                    url: String::new(),
                    receipt: cycle_complete_receipt,
                },
            ],
        )
        .expect("receipt validation should succeed");

        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("required receipt(s)"));
        assert!(failures[0].contains(&required_receipt));
    }

    #[test]
    fn fetch_cycle_receipts_passes_through_timestamp() {
        let repo = TestRepo::new();
        repo.init();
        repo.commit(
            "notes/complete.txt",
            "complete\n",
            "state(cycle-complete): close cycle [cycle 226]",
        );
        let tools_dir = repo.path().join("tools");
        fs::create_dir_all(&tools_dir).expect("create tools dir");
        let args_log = repo.path().join("cycle-receipts-args.txt");
        fs::write(
            tools_dir.join("cycle-receipts"),
            format!(
                "#!/usr/bin/env bash\nprintf '%s\\n' \"$@\" > \"{}\"\nprintf '%s\n' '[{{\"receipt\":\"abc1234\",\"url\":\"https://example.test/abc1234\"}}]'\n",
                args_log.display()
            ),
        )
        .expect("write cycle-receipts wrapper");

        let through = find_cycle_complete_timestamp(repo.path(), 226)
            .expect("cycle-complete timestamp should resolve");
        let receipts = fetch_cycle_receipts(repo.path(), 226, Some(through.as_str()))
            .expect("receipt fetch should succeed");
        let argv = fs::read_to_string(args_log).expect("read args log");

        assert_eq!(receipts.len(), 1);
        assert!(argv.contains("--through"));
        assert!(argv.contains(&through));
    }

    #[test]
    fn ignores_infrastructure_changes_committed_after_worklog_commit() {
        let repo = TestRepo::new();
        repo.init();
        repo.write_file("docs/state.json", "{}\n");
        let cycle_complete_receipt = repo.commit(
            "notes/complete.txt",
            "complete\n",
            "state(cycle-complete): close cycle [cycle 226]",
        );
        install_cycle_receipts_wrapper(&repo, &[&cycle_complete_receipt]);
        let worklog_path = repo
            .path()
            .join("docs/worklog/2026-03-31/020304-cycle-226-summary.md");
        if let Some(parent) = worklog_path.parent() {
            fs::create_dir_all(parent).expect("create worklog parent");
        }
        let worklog_content = valid_worklog_content(&[&cycle_complete_receipt], "- None.\n");
        fs::write(&worklog_path, &worklog_content).expect("write worklog");
        repo.commit(
            "docs/worklog/2026-03-31/020304-cycle-226-summary.md",
            &worklog_content,
            "docs(worklog): add cycle 226 worklog",
        );
        repo.commit(
            "tools/validate-docs",
            "#!/usr/bin/env bash\nexit 0\n",
            "fix(validate-docs): post-worklog tool change",
        );

        let failures = validate_worklog(repo.path(), &worklog_path, 226, Some("pass"), None)
            .expect("worklog validation should succeed");

        assert!(failures.is_empty(), "unexpected failures: {failures:?}");
    }

    #[test]
    fn requires_documenting_infrastructure_changes_committed_before_worklog_commit() {
        let repo = TestRepo::new();
        repo.init();
        repo.write_file("docs/state.json", "{}\n");
        repo.commit(
            "tools/validate-docs",
            "#!/usr/bin/env bash\nexit 0\n",
            "fix(validate-docs): pre-worklog tool change",
        );
        let cycle_complete_receipt = repo.commit(
            "notes/complete.txt",
            "complete\n",
            "state(cycle-complete): close cycle [cycle 226]",
        );
        install_cycle_receipts_wrapper(&repo, &[&cycle_complete_receipt]);
        let worklog_path = repo
            .path()
            .join("docs/worklog/2026-03-31/020304-cycle-226-summary.md");
        if let Some(parent) = worklog_path.parent() {
            fs::create_dir_all(parent).expect("create worklog parent");
        }
        let worklog_content = valid_worklog_content(&[&cycle_complete_receipt], "- None.\n");
        fs::write(&worklog_path, &worklog_content).expect("write worklog");
        repo.commit(
            "docs/worklog/2026-03-31/020304-cycle-226-summary.md",
            &worklog_content,
            "docs(worklog): add cycle 226 worklog",
        );

        let failures = validate_worklog(repo.path(), &worklog_path, 226, Some("pass"), None)
            .expect("worklog validation should succeed");

        assert_eq!(failures.len(), 1, "unexpected failures: {failures:?}");
        assert!(failures[0].contains("self-modifications section says None"));
        assert!(failures[0].contains("tools/validate-docs"));
    }

    #[test]
    fn excludes_post_c3_field_mismatches_from_worklog_validation() {
        let repo = TestRepo::new();
        repo.init();
        repo.write_file(
            "docs/state.json",
            serde_json::to_string_pretty(&serde_json::json!({
                "agent_sessions": [
                    { "status": "in_flight" },
                    { "status": "in_flight" }
                ]
            }))
            .expect("serialize state")
            .as_str(),
        );
        let cycle_complete_receipt = repo.commit(
            "notes/complete.txt",
            "complete\n",
            "state(cycle-complete): close cycle [cycle 226]",
        );
        install_cycle_receipts_wrapper(&repo, &[&cycle_complete_receipt]);
        let worklog_path = repo
            .path()
            .join("docs/worklog/2026-03-31/020304-cycle-226-summary.md");
        if let Some(parent) = worklog_path.parent() {
            fs::create_dir_all(parent).expect("create worklog parent");
        }
        let worklog_content = format!(
            "\
## Pre-dispatch state

*Snapshot before review dispatch — final counters may differ after C6.*
- **In-flight agent sessions**: 0
- **Pipeline status**: PASS

{}

## Self-modifications

- None.
",
            receipts_table(&[&cycle_complete_receipt]),
        );
        fs::write(&worklog_path, &worklog_content).expect("write worklog");
        repo.commit(
            "docs/worklog/2026-03-31/020304-cycle-226-summary.md",
            &worklog_content,
            "docs(worklog): add cycle 226 worklog",
        );

        let failures = validate_worklog(repo.path(), &worklog_path, 226, Some("fail"), None)
            .expect("worklog validation should succeed");

        assert!(failures.is_empty(), "unexpected failures: {failures:?}");
    }

    #[test]
    fn still_reports_pre_c3_mismatches_when_post_c3_fields_are_excluded() {
        let repo = TestRepo::new();
        repo.init();
        repo.write_file(
            "docs/state.json",
            serde_json::to_string_pretty(&serde_json::json!({
                "agent_sessions": [
                    { "status": "in_flight" },
                    { "status": "in_flight" }
                ]
            }))
            .expect("serialize state")
            .as_str(),
        );
        repo.commit(
            "tools/validate-docs",
            "#!/usr/bin/env bash\nexit 0\n",
            "fix(validate-docs): pre-worklog tool change",
        );
        let cycle_complete_receipt = repo.commit(
            "notes/complete.txt",
            "complete\n",
            "state(cycle-complete): close cycle [cycle 226]",
        );
        install_cycle_receipts_wrapper(&repo, &[&cycle_complete_receipt]);
        let worklog_path = repo
            .path()
            .join("docs/worklog/2026-03-31/020304-cycle-226-summary.md");
        if let Some(parent) = worklog_path.parent() {
            fs::create_dir_all(parent).expect("create worklog parent");
        }
        let worklog_content = format!(
            "\
## Pre-dispatch state

*Snapshot before review dispatch — final counters may differ after C6.*
- **In-flight agent sessions**: 0
- **Pipeline status**: PASS

{}

## Self-modifications

- None.
",
            receipts_table(&[&cycle_complete_receipt]),
        );
        fs::write(&worklog_path, &worklog_content).expect("write worklog");
        repo.commit(
            "docs/worklog/2026-03-31/020304-cycle-226-summary.md",
            &worklog_content,
            "docs(worklog): add cycle 226 worklog",
        );

        let failures = validate_worklog(repo.path(), &worklog_path, 226, Some("fail"), None)
            .expect("worklog validation should succeed");

        assert_eq!(failures.len(), 1, "unexpected failures: {failures:?}");
        assert!(failures[0].contains("self-modifications section says None"));
        assert!(failures[0].contains("tools/validate-docs"));
    }

    #[test]
    fn detects_self_modification_false_negative() {
        let content = "\
## Self-modifications

- None.
";
        let finding = validate_self_modifications_section(
            content,
            &["tools/rust/crates/write-entry/src/main.rs".to_string()],
        )
        .expect("expected self-modification finding");
        match finding {
            SelfModificationFinding::Failure(failure) => {
                assert!(failure.contains("says None"));
                assert!(failure.contains("tools/rust/crates/write-entry/src/main.rs"));
            }
            SelfModificationFinding::Warning(warning) => {
                panic!("expected failure, got warning: {warning}");
            }
        }
    }

    #[test]
    fn accepts_alternate_none_format_in_self_modifications() {
        let content = "\
## Self-modifications

* None
";
        let finding = validate_self_modifications_section(
            content,
            &["tools/rust/crates/write-entry/src/main.rs".to_string()],
        )
        .expect("expected self-modification finding");
        match finding {
            SelfModificationFinding::Failure(failure) => {
                assert!(failure.contains("says None"));
            }
            SelfModificationFinding::Warning(warning) => {
                panic!("expected failure, got warning: {warning}");
            }
        }
    }

    #[test]
    fn warns_when_self_modifications_omit_infrastructure_group() {
        let content = "\
## Self-modifications

- tools/: updated Rust validators.
";
        let finding = validate_self_modifications_section(
            content,
            &[
                "tools/rust/crates/validate-docs/src/main.rs".to_string(),
                "AGENTS.md".to_string(),
            ],
        )
        .expect("expected self-modification warning");
        match finding {
            SelfModificationFinding::Warning(warning) => {
                assert!(warning.contains("AGENTS.md"));
                assert!(!warning.contains("tools/"));
            }
            SelfModificationFinding::Failure(failure) => {
                panic!("expected warning, got failure: {failure}");
            }
        }
    }

    #[test]
    fn detects_duplicate_cycle_prefix() {
        let content =
            "## 2026-03-11 — Cycle 226: Cycle 226: Breaking the worklog-accuracy pattern\n";
        let failures = validate_journal_headings(content);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("duplicated cycle prefix"));
    }

    #[test]
    fn rejects_duplicate_cycle_entries() {
        let content = "\
## 2026-03-11 — Cycle 226: First entry

### Concrete commitments for next cycle

- Finish validation.

## 2026-03-11 — Cycle 226: Duplicate entry
";
        let failures = validate_no_duplicate_cycles(content);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("cycle 226"));
    }

    #[test]
    fn allows_unique_cycle_entries() {
        let content = "\
## 2026-03-11 — Cycle 226: First entry

## 2026-03-11 — Cycle 227: Second entry
";
        let failures = validate_no_duplicate_cycles(content);
        assert!(failures.is_empty());
    }

    #[test]
    fn rejects_duplicate_section_headers_within_cycle() {
        let content = "\
## 2026-03-11 — Cycle 226: First entry

### Previous commitment follow-through

- Done.

### Previous commitment follow-through

- Also done.
";
        let failures = validate_no_duplicate_section_headers(content);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("duplicate section heading"));
        assert!(failures[0].contains("cycle 226"));
    }

    #[test]
    fn allows_repeated_section_headers_in_different_cycles() {
        let content = "\
## 2026-03-11 — Cycle 226: First entry

### Previous commitment follow-through

- Done.

## 2026-03-12 — Cycle 227: Second entry

### Previous commitment follow-through

- Done again.
";
        let failures = validate_no_duplicate_section_headers(content);
        assert!(failures.is_empty());
    }

    #[test]
    fn rejects_literal_escaped_newlines_outside_code_blocks() {
        let content = "\
## 2026-03-11 — Cycle 226: First entry

Line with escaped newline\\nshould fail.
";
        let failures = validate_no_escaped_newlines(content);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("line 3"));
    }

    #[test]
    fn ignores_literal_escaped_newlines_inside_code_blocks() {
        let content = "\
## 2026-03-11 — Cycle 226: First entry

```text
Line with escaped newline\\nshould be ignored.
```
";
        let failures = validate_no_escaped_newlines(content);
        assert!(failures.is_empty());
    }

    #[test]
    fn validate_journal_reports_new_journal_validation_failures() {
        let temp = TestDir::new();
        let journal_path = temp.path().join("2026-03-11.md");
        let content = "\
## 2026-03-11 — Cycle 226: First entry

### Previous commitment follow-through

- Done.

### Previous commitment follow-through

- Done again.

### Concrete commitments for next cycle

- Finish validation.\\nNo escaped newlines.

## 2026-03-11 — Cycle 226: Duplicate entry
";
        fs::write(&journal_path, content).expect("write journal");

        let failures =
            validate_journal(temp.path(), &journal_path).expect("journal validation should run");
        assert_eq!(failures.len(), 3);
        assert!(failures
            .iter()
            .any(|failure| failure.contains("duplicate journal entry")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("duplicate section heading")));
        assert!(failures
            .iter()
            .any(|failure| failure.contains("escaped newline characters")));
    }

    #[test]
    fn accepts_valid_relative_worklog_link() {
        let temp = TestDir::new();
        let journal_dir = temp.path().join("docs/journal");
        let worklog_path = temp
            .path()
            .join("docs/worklog/2026-03-11/123451-cycle-226-summary.md");
        fs::create_dir_all(&journal_dir).expect("create journal dir");
        fs::create_dir_all(worklog_path.parent().expect("worklog parent"))
            .expect("create worklog dir");
        fs::write(&worklog_path, "# worklog\n").expect("write worklog");
        let journal_path = journal_dir.join("2026-03-11.md");
        let content = "Worklog: [cycle 226](../worklog/2026-03-11/123451-cycle-226-summary.md)\n";

        let failures = validate_worklog_links(content, &journal_path);
        assert!(failures.is_empty());
    }

    #[test]
    fn rejects_incorrect_relative_worklog_link() {
        let temp = TestDir::new();
        let journal_dir = temp.path().join("docs/journal");
        fs::create_dir_all(&journal_dir).expect("create journal dir");
        let journal_path = journal_dir.join("2026-03-11.md");
        let content = "Worklog: [cycle 226](docs/worklog/2026-03-11/123451-cycle-226-summary.md)\n";

        let failures = validate_worklog_links(content, &journal_path);
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("../worklog/"));
    }

    #[test]
    fn rejects_missing_commitments_section() {
        let content = "\
## 2026-03-11 — Cycle 226: Breaking the worklog-accuracy pattern

### Context

Observed something.
";
        let failure = validate_commitment_section(content).expect("expected failure");
        assert!(failure.contains("Concrete commitments for next cycle"));
    }

    #[test]
    fn rejects_empty_commitments_section() {
        let content = "\
## 2026-03-11 — Cycle 226: Breaking the worklog-accuracy pattern

### Concrete commitments for next cycle

### Open questions

- None.
";
        let failure = validate_commitment_section(content).expect("expected failure");
        assert!(failure.contains("Concrete commitments for next cycle"));
    }

    #[test]
    fn rejects_open_questions_none_when_question_for_eva_filed() {
        // Mirrors cycle 458 review F4: body mentions filing a question-for-eva
        // but the Open questions section claims none.
        let content = "\
## 2026-04-08 — Cycle 458: Example

### What I tried

For the chronic journal-quality escalation: filed question-for-eva #2293 with options.

### Concrete commitments for next cycle

1. Verify the thing.

### Open questions

- None.
";
        let failure = validate_open_questions_consistency(content).expect("expected failure");
        assert!(failure.contains("Open questions"));
        assert!(failure.contains("question-for-eva"));
    }

    #[test]
    fn allows_open_questions_none_when_no_question_for_eva_mentioned() {
        let content = "\
## 2026-04-08 — Cycle 458: Example

### What I tried

Did some normal cycle work without filing any escalation.

### Concrete commitments for next cycle

1. Verify the thing.

### Open questions

- None.
";
        assert!(validate_open_questions_consistency(content).is_none());
    }

    #[test]
    fn open_questions_check_only_inspects_latest_cycle_entry() {
        // Cycle 459 hotfix: F4 must scope to the most recent cycle entry only,
        // otherwise an already-shipped historical contradiction (cycle 458's
        // entry, which prompted the fix) would block every subsequent cycle's
        // close-out.
        let content = "\
## 2026-04-08 — Cycle 458: example with historical contradiction

### What I tried

filed question-for-eva #2293 with options.

### Concrete commitments for next cycle

1. Verify the thing.

### Open questions

- None.

## 2026-04-08 — Cycle 459: clean entry

### What I tried

Did normal cycle work.

### Concrete commitments for next cycle

1. Verify another thing.

### Open questions

- None.
";
        // Latest entry (cycle 459) has no question-for-eva mention, so the
        // check should pass even though the cycle 458 entry above contains
        // the contradiction the check was designed to catch.
        assert!(validate_open_questions_consistency(content).is_none());
    }

    #[test]
    fn open_questions_check_still_catches_latest_entry_contradiction() {
        let content = "\
## 2026-04-08 — Cycle 458: clean entry

### What I tried

Routine work.

### Concrete commitments for next cycle

1. Verify the thing.

### Open questions

- None.

## 2026-04-08 — Cycle 459: contradiction in newest entry

### What I tried

filed question-for-eva #2293 with options.

### Concrete commitments for next cycle

1. Verify the thing.

### Open questions

- None.
";
        // Latest entry (cycle 459) has the contradiction, so the check fails.
        let failure = validate_open_questions_consistency(content).expect("expected failure");
        assert!(failure.contains("Open questions"));
    }

    #[test]
    fn allows_open_questions_listed_even_when_question_for_eva_filed() {
        let content = "\
## 2026-04-08 — Cycle 458: Example

### What I tried

filed question-for-eva #2293.

### Concrete commitments for next cycle

1. Verify the thing.

### Open questions

- #2293 — chronic journal-quality response.
";
        assert!(validate_open_questions_consistency(content).is_none());
    }

    #[test]
    fn rejects_none_under_open_questions_raised_this_cycle_when_body_mentions_filing() {
        let content = "\
## 2026-04-08 — Cycle 458: Example

### What I tried

filed question-for-eva #2293 with options.

### Concrete commitments for next cycle

1. Verify the thing.

### Open questions raised this cycle

- None.

### Standing Eva blockers

- [#2293](https://github.com/EvaLok/schema-org-json-ld/issues/2293) — chronic journal-quality response (12h stale)
";
        let failure = validate_open_questions_consistency(content).expect("expected failure");
        assert!(failure.contains("Open questions"));
        assert!(failure.contains("question-for-eva"));
    }

    #[test]
    fn state_aware_check_catches_cycle_502_f1_recurrence() {
        // Models the cycle 501 F2 / cycle 502 F1 pattern: body names
        // a blocker issue by number, state has pending Eva questions,
        // but Open questions section says "None". The text-only check
        // above does NOT catch this because the body never uses the
        // literal phrase "filed question-for-eva".
        let content = "\
## 2026-04-16 — Cycle 502: gate-blocked by #2542

### Context

Dispatches remain blocked by #2542.

### Concrete commitments for next cycle

1. Keep monitoring.

### Open questions

- None.
";
        let mut state = StateJson::default();
        state.open_questions_for_eva = vec![serde_json::json!(2542), serde_json::json!(2519)];

        let failure = validate_open_questions_against_state(content, &state)
            .expect("expected state-aware failure");
        assert!(failure.contains("docs/state.json"));
        assert!(failure.contains("#2542"));
        assert!(failure.contains("#2519"));
    }

    #[test]
    fn state_aware_check_passes_when_state_has_no_pending_questions() {
        let content = "\
## 2026-04-16 — Cycle 502: clean

### Concrete commitments for next cycle

1. Keep going.

### Open questions

- None.
";
        let state = StateJson::default();
        assert!(validate_open_questions_against_state(content, &state).is_none());
    }

    #[test]
    fn state_aware_check_passes_when_open_questions_are_listed() {
        let content = "\
## 2026-04-16 — Cycle 502: honest reporting

### Concrete commitments for next cycle

1. Keep going.

### Open questions

- #2542 — gate deadlock.
";
        let mut state = StateJson::default();
        state.open_questions_for_eva = vec![serde_json::json!(2542)];

        assert!(validate_open_questions_against_state(content, &state).is_none());
    }

    #[test]
    fn state_aware_check_only_considers_latest_cycle_entry() {
        // Historical entries may still carry the "None" claim — they
        // are already shipped. Only the latest cycle entry must be
        // consistent with current state.
        let content = "\
## 2026-04-15 — Cycle 501: old entry

### Open questions

- None.

## 2026-04-16 — Cycle 502: latest entry lists questions

### Open questions

- #2542 — gate deadlock.
";
        let mut state = StateJson::default();
        state.open_questions_for_eva = vec![serde_json::json!(2542)];

        assert!(validate_open_questions_against_state(content, &state).is_none());
    }

    #[test]
    fn state_aware_check_names_issues_by_number_in_failure_message() {
        let content = "\
## 2026-04-16 — Cycle 502: gate-blocked

### Open questions

- None.
";
        let mut state = StateJson::default();
        state.open_questions_for_eva = vec![
            serde_json::json!(2542),
            serde_json::json!(2519),
            serde_json::json!(2293),
        ];

        let failure =
            validate_open_questions_against_state(content, &state).expect("expected failure");
        // Orchestrator must be able to read the failure and know
        // exactly which open questions to list — not just that
        // there are some.
        assert!(failure.contains("#2542, #2519, #2293"));
    }

    #[test]
    fn state_aware_check_allows_standing_eva_blockers_for_new_journal_layout() {
        let content = "\
## 2026-04-16 — Cycle 502: honest reporting

### Concrete commitments for next cycle

1. Keep going.

### Open questions raised this cycle

- None.

### Standing Eva blockers

- [#2542](https://github.com/EvaLok/schema-org-json-ld/issues/2542) — gate deadlock (49h stale)
- [#2519](https://github.com/EvaLok/schema-org-json-ld/issues/2519) — blocker policy (5h stale)
";
        let mut state = StateJson::default();
        state.open_questions_for_eva = vec![serde_json::json!(2542), serde_json::json!(2519)];

        assert!(validate_open_questions_against_state(content, &state).is_none());
    }

    struct TestRepo {
        path: PathBuf,
    }

    impl TestRepo {
        fn new() -> Self {
            let unique = format!(
                "validate-docs-repo-test-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("clock before epoch")
                    .as_nanos()
            );
            let path = std::env::temp_dir().join(unique);
            fs::create_dir_all(&path).expect("create temp repo dir");
            Self { path }
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn init(&self) {
            git_success(self.path(), ["init"]);
            git_success(self.path(), ["config", "user.name", "Validate Docs Tests"]);
            git_success(
                self.path(),
                ["config", "user.email", "validate-docs-tests@example.com"],
            );
            self.write_file("README.md", "test repo\n");
            git_success(self.path(), ["add", "--", "README.md"]);
            git_success(self.path(), ["commit", "-m", "initial commit"]);
            self.commit(
                "notes/start.txt",
                "start\n",
                "state(cycle-start): begin cycle [cycle 226]",
            );
        }

        fn commit(&self, relative_path: &str, contents: &str, message: &str) -> String {
            self.write_file(relative_path, contents);
            git_success(self.path(), ["add", "--", relative_path]);
            git_success(self.path(), ["commit", "-m", message]);
            git_stdout(self.path(), ["rev-parse", "--short=7", "HEAD"])
                .trim()
                .to_string()
        }

        fn write_file(&self, relative_path: &str, contents: &str) {
            let path = self.path().join(relative_path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("create parent directories");
            }
            fs::write(path, contents).expect("write test file");
        }
    }

    impl Drop for TestRepo {
        fn drop(&mut self) {
            if let Err(error) = fs::remove_dir_all(&self.path) {
                eprintln!(
                    "Warning: failed to remove test repo {}: {}",
                    self.path.display(),
                    error
                );
            }
        }
    }

    fn receipts_table(receipts: &[&str]) -> String {
        let mut content = String::from(
            "## Commit receipts\n\n| Tool | Receipt | Link |\n|------|---------|------|\n",
        );
        for receipt in receipts {
            content.push_str(&format!(
                "| step | [`{receipt}`](https://example.test/{receipt}) | [link](https://example.test/{receipt}) |\n"
            ));
        }
        content
    }

    fn valid_worklog_content(receipts: &[&str], self_modifications: &str) -> String {
        format!(
            "\
## Pre-dispatch state

- **In-flight agent sessions**: 0
- **Pipeline status**: PASS

{}

## Self-modifications

{}",
            receipts_table(receipts),
            self_modifications
        )
    }

    fn install_cycle_receipts_wrapper(repo: &TestRepo, receipts: &[&str]) {
        let tools_dir = repo.path().join("tools");
        fs::create_dir_all(&tools_dir).expect("create tools dir");
        let json = format!(
            "[{}]",
            receipts
                .iter()
                .map(|receipt| format!(
                    r#"{{"receipt":"{receipt}","url":"https://example.test/{receipt}"}}"#
                ))
                .collect::<Vec<_>>()
                .join(",")
        );
        fs::write(
            tools_dir.join("cycle-receipts"),
            format!("#!/usr/bin/env bash\nprintf '%s\n' '{json}'\n"),
        )
        .expect("write cycle-receipts wrapper");
    }

    fn git_success<I, S>(repo_root: &Path, args: I)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let (rendered_args, output) = run_test_git(repo_root, args);
        assert!(
            output.status.success(),
            "git command failed (git -C {} {}): {}",
            repo_root.display(),
            rendered_args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    fn git_stdout<I, S>(repo_root: &Path, args: I) -> String
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let (rendered_args, output) = run_test_git(repo_root, args);
        assert!(
            output.status.success(),
            "git command failed (git -C {} {}): {}",
            repo_root.display(),
            rendered_args.join(" "),
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).expect("git output should be valid UTF-8")
    }

    fn run_test_git<I, S>(repo_root: &Path, args: I) -> (Vec<String>, std::process::Output)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let rendered_args: Vec<String> = args
            .into_iter()
            .map(|argument| argument.as_ref().to_string_lossy().into_owned())
            .collect();
        let output = ProcessCommand::new("git")
            .arg("-C")
            .arg(repo_root)
            .args(&rendered_args)
            .output()
            .expect("git command should execute");
        (rendered_args, output)
    }
}
