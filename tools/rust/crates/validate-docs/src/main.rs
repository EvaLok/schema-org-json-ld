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
const INFRASTRUCTURE_PATHS: [&str; 5] = [
    "STARTUP_CHECKLIST.md",
    "COMPLETION_CHECKLIST.md",
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
}

#[derive(Debug, Parser)]
struct JournalArgs {
    /// Path to the journal file to validate
    #[arg(long)]
    file: PathBuf,
}

#[derive(Debug, Deserialize)]
struct ReceiptEntry {
    receipt: String,
}

#[derive(Debug, Deserialize)]
struct PipelineReport {
    overall: String,
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Command::Worklog(args) => validate_worklog(&cli.repo_root, &args.file, args.cycle),
        Command::Journal(args) => validate_journal(&args.file),
    };

    match result {
        Ok(failures) if failures.is_empty() => {}
        Ok(failures) => {
            for failure in failures {
                eprintln!("- {}", failure);
            }
            std::process::exit(1);
        }
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn validate_worklog(repo_root: &Path, file: &Path, cycle: u64) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(file)
        .map_err(|error| format!("failed to read {}: {}", file.display(), error))?;
    let state = read_state_json(repo_root)?;
    let mut failures = Vec::new();

    let expected_in_flight = count_in_flight_sessions(&state);
    if let Some(failure) = validate_in_flight_count(&content, expected_in_flight) {
        failures.push(failure);
    }

    match fetch_cycle_receipts(repo_root, cycle) {
        Ok(expected_receipts) => {
            failures.extend(validate_receipt_completeness(&content, &expected_receipts))
        }
        Err(error) => failures.push(format!("unable to validate commit receipts: {}", error)),
    }

    match changed_infrastructure_paths(repo_root, cycle) {
        Ok(changed_paths) => {
            if let Some(failure) = validate_self_modifications_section(&content, &changed_paths) {
                failures.push(failure);
            }
        }
        Err(error) => failures.push(format!("unable to validate self-modifications: {}", error)),
    }

    match fetch_pipeline_report(repo_root, cycle) {
        Ok(report) => {
            if let Some(failure) = validate_pipeline_status(&content, &report.overall) {
                failures.push(failure);
            }
        }
        Err(error) => failures.push(format!("unable to validate pipeline status: {}", error)),
    }

    Ok(failures)
}

fn validate_journal(file: &Path) -> Result<Vec<String>, String> {
    let content = fs::read_to_string(file)
        .map_err(|error| format!("failed to read {}: {}", file.display(), error))?;
    let mut failures = Vec::new();

    failures.extend(validate_journal_headings(&content));
    failures.extend(validate_worklog_links(&content, file));
    if let Some(failure) = validate_commitment_section(&content) {
        failures.push(failure);
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

fn validate_in_flight_count(content: &str, expected: usize) -> Option<String> {
    let reported = match extract_markdown_value(content, "In-flight agent sessions") {
        Some(reported) => reported,
        None => return Some(
            "worklog is missing the 'In-flight agent sessions' line in the Current state section"
                .to_string(),
        ),
    };
    let parsed = match reported.parse::<usize>() {
        Ok(parsed) => parsed,
        Err(_) => {
            return Some(format!(
                "in-flight agent sessions line must contain an integer count, found '{}'",
                reported
            ))
        }
    };
    if parsed == expected {
        return None;
    }

    Some(format!(
        "in-flight agent sessions mismatch: worklog reports {}, state.json has {}",
        parsed, expected
    ))
}

fn fetch_cycle_receipts(repo_root: &Path, cycle: u64) -> Result<Vec<ReceiptEntry>, String> {
    let output = run_wrapper(
        repo_root,
        "tools/cycle-receipts",
        &[
            "--cycle".to_string(),
            cycle.to_string(),
            "--json".to_string(),
            "--repo-root".to_string(),
            repo_root.display().to_string(),
        ],
    )?;
    serde_json::from_str::<Vec<ReceiptEntry>>(&output)
        .map_err(|error| format!("failed to parse cycle-receipts JSON: {}", error))
}

fn validate_receipt_completeness(content: &str, expected: &[ReceiptEntry]) -> Vec<String> {
    let present = extract_present_receipts(content);
    let missing = expected
        .iter()
        .filter_map(|entry| {
            let receipt = entry.receipt.trim();
            (!receipt.is_empty() && !present.contains(receipt)).then(|| receipt.to_string())
        })
        .collect::<Vec<_>>();

    if missing.is_empty() {
        return Vec::new();
    }

    vec![format!(
        "commit receipts section is missing required receipt(s): {}",
        missing.join(", ")
    )]
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

fn changed_infrastructure_paths(repo_root: &Path, cycle: u64) -> Result<Vec<String>, String> {
    let start_commit = find_cycle_start_commit(repo_root, cycle)?;
    let mut args = vec![
        "diff".to_string(),
        "--name-only".to_string(),
        start_commit,
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

fn validate_self_modifications_section(content: &str, changed_paths: &[String]) -> Option<String> {
    let section = extract_section_body(content, SELF_MODIFICATIONS_HEADING)?;
    let reports_none = section.lines().any(reports_no_self_modifications);
    if !reports_none || changed_paths.is_empty() {
        return None;
    }

    Some(format!(
        "self-modifications section says None, but infrastructure changes exist: {}",
        changed_paths.join(", ")
    ))
}

fn fetch_pipeline_report(repo_root: &Path, cycle: u64) -> Result<PipelineReport, String> {
    let output = run_wrapper(
        repo_root,
        "tools/pipeline-check",
        &[
            "--json".to_string(),
            "--cycle".to_string(),
            cycle.to_string(),
            "--repo-root".to_string(),
            repo_root.display().to_string(),
        ],
    )?;
    serde_json::from_str::<PipelineReport>(&output)
        .map_err(|error| format!("failed to parse pipeline-check JSON: {}", error))
}

fn validate_pipeline_status(content: &str, overall: &str) -> Option<String> {
    let reported = match extract_markdown_value(content, "Pipeline status") {
        Some(reported) => reported,
        None => {
            return Some(
                "worklog is missing the 'Pipeline status' line in the Current state section"
                    .to_string(),
            )
        }
    };
    let expected = if overall.eq_ignore_ascii_case("pass") {
        "PASS"
    } else {
        "FAIL"
    };
    let reported_status = if reported.starts_with("PASS") {
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
    let output = ProcessCommand::new("git")
        .arg("-C")
        .arg(repo_root)
        .args(args)
        .output()
        .map_err(|error| format!("failed to execute git {}: {}", args.join(" "), error))?;
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
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    #[test]
    fn detects_in_flight_mismatch() {
        let content = "\
## Current state

- **In-flight agent sessions**: 4
";
        let failure = validate_in_flight_count(content, 1).expect("expected mismatch");
        assert!(failure.contains("worklog reports 4"));
        assert!(failure.contains("state.json has 1"));
    }

    #[test]
    fn detects_pipeline_status_mismatch() {
        let content = "\
## Current state

- **Pipeline status**: PASS (8/8)
";
        let failure = validate_pipeline_status(content, "warn").expect("expected mismatch");
        assert!(failure.contains("pipeline status mismatch"));
    }

    #[test]
    fn detects_missing_receipts() {
        let content = "\
## Commit receipts

| Tool | Receipt | Link |
|------|---------|------|
| write-entry | [`abc1234`](https://example.test/abc1234) | [abc1234](https://example.test/abc1234) |
";
        let failures = validate_receipt_completeness(
            content,
            &[
                ReceiptEntry {
                    receipt: "abc1234".to_string(),
                },
                ReceiptEntry {
                    receipt: "def5678".to_string(),
                },
            ],
        );
        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("def5678"));
    }

    #[test]
    fn detects_self_modification_false_negative() {
        let content = "\
## Self-modifications

- None.
";
        let failure = validate_self_modifications_section(
            content,
            &["tools/rust/crates/write-entry/src/main.rs".to_string()],
        )
        .expect("expected self-modification failure");
        assert!(failure.contains("says None"));
        assert!(failure.contains("tools/rust/crates/write-entry/src/main.rs"));
    }

    #[test]
    fn accepts_alternate_none_format_in_self_modifications() {
        let content = "\
## Self-modifications

* None
";
        let failure = validate_self_modifications_section(
            content,
            &["tools/rust/crates/write-entry/src/main.rs".to_string()],
        )
        .expect("expected self-modification failure");
        assert!(failure.contains("says None"));
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
}
