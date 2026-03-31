use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
#[cfg(test)]
use std::io::ErrorKind;
#[cfg(test)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;
#[cfg(test)]
use std::time::{SystemTime, UNIX_EPOCH};

const RECEIPT_HEADER_PATTERN_STR: &str = r"^\|\s*Tool\s*\|\s*Receipt\s*\|\s*Link\s*\|\s*$";
const RECEIPT_SEPARATOR_PATTERN_STR: &str = r"^\|\s*-+\s*\|\s*-+\s*\|\s*-+\s*\|\s*$";
const RECEIPT_CELL_PATTERN_STR: &str = r"(?i)\b([0-9a-f]{7})\b";
const RECEIPT_URL_PATTERN_STR: &str =
    r"https://github\.com/EvaLok/schema-org-json-ld/commit/([0-9a-fA-F]{40})";

#[derive(Debug, Parser)]
#[command(name = "receipt-validate")]
struct Cli {
    /// Cycle number to validate
    #[arg(long)]
    cycle: u64,

    /// Path to the worklog markdown file
    #[arg(long)]
    worklog: PathBuf,

    /// Path to the repository root
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Output a structured JSON report
    #[arg(long)]
    json: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
struct CanonicalReceiptEntry {
    receipt: String,
    commit: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
struct MissingDetail {
    sha: String,
    subject: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct WorklogReceiptEntry {
    short_sha: String,
    full_sha: Option<String>,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
struct ValidationReport {
    cycle: u64,
    worklog_receipts: usize,
    canonical_receipts: usize,
    structurally_excluded: usize,
    genuinely_missing: usize,
    missing_details: Vec<MissingDetail>,
    excluded_details: Vec<MissingDetail>,
    validation_errors: Vec<String>,
    result: String,
}

fn main() {
    let cli = Cli::parse();
    match run(&cli) {
        Ok(report) => {
            let rendered = if cli.json {
                serde_json::to_string_pretty(&report)
                    .map_err(|error| format!("failed to serialize JSON output: {error}"))
            } else {
                Ok(render_human(&report))
            };

            match rendered {
                Ok(output) => println!("{output}"),
                Err(error) => {
                    eprintln!("Error: {error}");
                    std::process::exit(1);
                }
            }

            if report.result == "FAIL" {
                std::process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("Error: {error}");
            std::process::exit(1);
        }
    }
}

fn run(cli: &Cli) -> Result<ValidationReport, String> {
    let content = fs::read_to_string(&cli.worklog)
        .map_err(|error| format!("failed to read {}: {error}", cli.worklog.display()))?;
    let worklog_receipts = extract_worklog_receipts(&content)?;
    let validation_errors = validate_worklog_receipt_links(&worklog_receipts, &cli.repo_root)?;
    let canonical_receipts = fetch_cycle_receipts(&cli.repo_root, cli.cycle)?;
    Ok(compare_receipts(
        cli.cycle,
        &worklog_receipts,
        &canonical_receipts,
        validation_errors,
    ))
}

fn extract_worklog_receipts(content: &str) -> Result<Vec<WorklogReceiptEntry>, String> {
    let header_pattern = receipt_header_pattern();
    let separator_pattern = receipt_separator_pattern();

    let mut lines = content.lines().peekable();
    while let Some(line) = lines.next() {
        if !header_pattern.is_match(line.trim()) {
            continue;
        }

        let Some(separator) = lines.next() else {
            return Err("receipt table header found without separator row".to_string());
        };
        if !separator_pattern.is_match(separator.trim()) {
            return Err(format!(
                "receipt table header found with malformed separator row: {}",
                separator.trim()
            ));
        }

        let mut receipts = Vec::new();
        for row in lines.by_ref() {
            let trimmed = row.trim();
            if !trimmed.starts_with('|') {
                break;
            }

            if let Some(receipt) = extract_receipt_from_row(trimmed)? {
                receipts.push(receipt);
            }
        }

        return Ok(receipts);
    }

    Ok(Vec::new())
}

fn compare_receipts(
    cycle: u64,
    worklog_receipts: &[WorklogReceiptEntry],
    canonical_receipts: &[CanonicalReceiptEntry],
    validation_errors: Vec<String>,
) -> ValidationReport {
    let mut missing_details = Vec::new();
    let mut excluded_details = Vec::new();
    let worklog_receipt_set = worklog_receipts
        .iter()
        .map(|entry| entry.short_sha.clone())
        .collect::<BTreeSet<_>>();

    for entry in canonical_receipts {
        let receipt = entry.receipt.trim().to_ascii_lowercase();
        if receipt.is_empty() || worklog_receipt_set.contains(&receipt) {
            continue;
        }

        let detail = MissingDetail {
            sha: receipt,
            subject: entry.commit.clone(),
        };
        if is_structurally_excluded(&entry.commit) {
            excluded_details.push(detail);
        } else {
            missing_details.push(detail);
        }
    }

    let result = if missing_details.is_empty() && validation_errors.is_empty() {
        "PASS"
    } else {
        "FAIL"
    };

    ValidationReport {
        cycle,
        worklog_receipts: worklog_receipts.len(),
        canonical_receipts: canonical_receipts.len(),
        structurally_excluded: excluded_details.len(),
        genuinely_missing: missing_details.len(),
        missing_details,
        excluded_details,
        validation_errors,
        result: result.to_string(),
    }
}

fn is_structurally_excluded(subject: &str) -> bool {
    subject.starts_with("docs(cycle-")
        || subject.starts_with("docs(worklog-patch):")
        || subject.starts_with("docs(review-body):")
        || subject.starts_with("state(record-dispatch):")
        || subject.starts_with("state(stabilization")
        || subject.starts_with("state(clean-cycle")
}

fn render_human(report: &ValidationReport) -> String {
    let mut output = format!("Receipt Validation — Cycle {}\n\n", report.cycle);
    output.push_str(&format!(
        "  Worklog receipts:      {}\n",
        report.worklog_receipts
    ));
    output.push_str(&format!(
        "  Canonical receipts:    {}\n",
        report.canonical_receipts
    ));
    output.push_str(&format!(
        "  Structurally excluded: {}{}\n",
        report.structurally_excluded,
        render_structural_suffix(&report.excluded_details)
    ));
    output.push_str(&format!(
        "  Genuinely missing:     {}\n",
        report.genuinely_missing
    ));

    if !report.missing_details.is_empty() {
        for detail in &report.missing_details {
            output.push_str(&format!("    - {} {}\n", detail.sha, detail.subject));
        }
    }

    if !report.validation_errors.is_empty() {
        output.push('\n');
        for error in &report.validation_errors {
            output.push_str(&format!("  {error}\n"));
        }
    }

    output.push_str(&format!("\n  Result: {}\n", report.result));
    output
}

fn fetch_cycle_receipts(
    repo_root: &Path,
    cycle: u64,
) -> Result<Vec<CanonicalReceiptEntry>, String> {
    let wrapper = repo_root.join("tools/cycle-receipts");
    let output = Command::new(&wrapper)
        .arg("--cycle")
        .arg(cycle.to_string())
        .arg("--json")
        .arg("--repo-root")
        .arg(repo_root)
        .current_dir(repo_root)
        .output()
        .map_err(|error| format!("failed to execute {}: {error}", wrapper.display()))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(format!(
            "{} exited with status {}: {}",
            wrapper.display(),
            output
                .status
                .code()
                .map_or_else(|| "signal".to_string(), |code| code.to_string()),
            stderr
        ));
    }

    let stdout = String::from_utf8(output.stdout).map_err(|error| {
        format!(
            "failed to decode {} output as UTF-8: {error}",
            wrapper.display()
        )
    })?;
    serde_json::from_str::<Vec<CanonicalReceiptEntry>>(&stdout)
        .map_err(|error| format!("failed to parse {} JSON output: {error}", wrapper.display()))
}

fn extract_receipt_from_row(row: &str) -> Result<Option<WorklogReceiptEntry>, String> {
    // Worklog receipt rows are expected to follow the Tool | Receipt | Link layout,
    // so the short SHA lives in the second non-empty column.
    let cells = row
        .split('|')
        .map(str::trim)
        .filter(|cell| !cell.is_empty())
        .collect::<Vec<_>>();
    let Some(cell) = cells.get(1) else {
        return Ok(None);
    };
    let Some(short_sha) = extract_receipt_from_cell(cell)? else {
        return Ok(None);
    };

    let full_sha = cells
        .get(2)
        .and_then(|cell| extract_full_sha_from_cell(cell))
        .or_else(|| extract_full_sha_from_cell(row));

    Ok(Some(WorklogReceiptEntry {
        short_sha,
        full_sha,
    }))
}

fn extract_receipt_from_cell(cell: &str) -> Result<Option<String>, String> {
    Ok(receipt_cell_pattern()
        .captures(cell)
        .and_then(|captures| captures.get(1))
        .map(|capture| capture.as_str().to_ascii_lowercase()))
}

fn extract_full_sha_from_cell(cell: &str) -> Option<String> {
    receipt_url_pattern()
        .captures(cell)
        .and_then(|captures| captures.get(1))
        .map(|capture| capture.as_str().to_ascii_lowercase())
}

fn validate_worklog_receipt_links(
    worklog_receipts: &[WorklogReceiptEntry],
    repo_root: &Path,
) -> Result<Vec<String>, String> {
    let mut errors = Vec::new();

    for entry in worklog_receipts {
        let Some(full_sha) = &entry.full_sha else {
            continue;
        };

        if !full_sha.starts_with(&entry.short_sha) {
            errors.push(format!(
                "FAIL: Receipt {} does not match linked commit {}",
                entry.short_sha, full_sha
            ));
            continue;
        }

        if !git_commit_exists(repo_root, full_sha)? {
            errors.push(format!(
                "FAIL: Receipt {} links to non-existent commit {}",
                entry.short_sha, full_sha
            ));
        }
    }

    Ok(errors)
}

fn git_commit_exists(repo_root: &Path, sha: &str) -> Result<bool, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root)
        .arg("cat-file")
        .arg("-e")
        .arg(format!("{sha}^{{commit}}"))
        .output()
        .map_err(|error| format!("failed to verify commit {sha}: {error}"))?;

    if output.status.success() {
        return Ok(true);
    }

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    match output.status.code() {
        Some(128) | Some(129)
            if stderr.contains("Not a valid object name")
                || stderr.contains("could not get object info") =>
        {
            Ok(false)
        }
        Some(code) => Err(format!(
            "git cat-file failed while verifying commit {sha} with exit code {code}: {}",
            stderr
        )),
        None => Err(format!(
            "git cat-file terminated by signal while verifying commit {sha}: {}",
            stderr
        )),
    }
}

fn render_structural_suffix(details: &[MissingDetail]) -> String {
    let mut categories = Vec::new();
    if details.iter().any(|detail| {
        detail.subject.starts_with("docs(cycle-")
            || detail.subject.starts_with("docs(worklog-patch):")
            || detail.subject.starts_with("docs(review-body):")
    }) {
        categories.push("docs commit");
    }
    if details
        .iter()
        .any(|detail| detail.subject.starts_with("state(record-dispatch):"))
    {
        categories.push("record-dispatch");
    }

    if categories.is_empty() {
        String::new()
    } else {
        format!(" ({})", categories.join(", "))
    }
}

fn receipt_header_pattern() -> &'static Regex {
    static RECEIPT_HEADER_PATTERN: OnceLock<Regex> = OnceLock::new();
    RECEIPT_HEADER_PATTERN.get_or_init(|| {
        Regex::new(RECEIPT_HEADER_PATTERN_STR)
            .expect("RECEIPT_HEADER_PATTERN_STR should compile as valid regex")
    })
}

fn receipt_separator_pattern() -> &'static Regex {
    static RECEIPT_SEPARATOR_PATTERN: OnceLock<Regex> = OnceLock::new();
    RECEIPT_SEPARATOR_PATTERN.get_or_init(|| {
        Regex::new(RECEIPT_SEPARATOR_PATTERN_STR)
            .expect("RECEIPT_SEPARATOR_PATTERN_STR should compile as valid regex")
    })
}

fn receipt_cell_pattern() -> &'static Regex {
    static RECEIPT_CELL_PATTERN: OnceLock<Regex> = OnceLock::new();
    RECEIPT_CELL_PATTERN.get_or_init(|| {
        Regex::new(RECEIPT_CELL_PATTERN_STR)
            .expect("RECEIPT_CELL_PATTERN_STR should compile as valid regex")
    })
}

fn receipt_url_pattern() -> &'static Regex {
    static RECEIPT_URL_PATTERN: OnceLock<Regex> = OnceLock::new();
    RECEIPT_URL_PATTERN.get_or_init(|| {
        Regex::new(RECEIPT_URL_PATTERN_STR)
            .expect("RECEIPT_URL_PATTERN_STR should compile as valid regex")
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn canonical_entries(entries: &[(&str, &str)]) -> Vec<CanonicalReceiptEntry> {
        entries
            .iter()
            .map(|(receipt, subject)| CanonicalReceiptEntry {
                receipt: (*receipt).to_string(),
                commit: (*subject).to_string(),
            })
            .collect()
    }

    #[test]
    fn parses_receipts_from_markdown_table() {
        let content = "\
## Commit receipts

| Tool | Receipt | Link |
|------|---------|------|
| cycle-start | abc1234 | [abc1234](https://example.test/commit/abc1234) |
| process-merge | def5678 | [def5678](https://example.test/commit/def5678) |
";

        let receipts = extract_worklog_receipts(content).expect("receipt table should parse");

        assert_eq!(
            receipts,
            vec![
                WorklogReceiptEntry {
                    short_sha: "abc1234".to_string(),
                    full_sha: None,
                },
                WorklogReceiptEntry {
                    short_sha: "def5678".to_string(),
                    full_sha: None,
                },
            ]
        );
    }

    #[test]
    fn detects_structural_exclusions_by_subject_prefix() {
        assert!(is_structurally_excluded(
            "docs(cycle-255): publish worklog and journal"
        ));
        assert!(is_structurally_excluded(
            "docs(worklog-patch): post-dispatch state correction [cycle 255]"
        ));
        assert!(is_structurally_excluded(
            "docs(review-body): cycle 255 review dispatch body [cycle 255]"
        ));
        assert!(is_structurally_excluded(
            "state(record-dispatch): issue #10 dispatched [cycle 255]"
        ));
        assert!(is_structurally_excluded(
            "state(stabilization): clean cycle 292 — counter 1/50 [cycle 292]"
        ));
        assert!(is_structurally_excluded(
            "state(clean-cycle): stabilization counter 0->1, cycle 285 clean [cycle 285]"
        ));
        assert!(!is_structurally_excluded(
            "state(cycle-complete): close cycle [cycle 255]"
        ));
    }

    #[test]
    fn passes_when_only_structural_receipts_are_missing() {
        let worklog_receipts = vec![
            WorklogReceiptEntry {
                short_sha: "aaaaaaa".to_string(),
                full_sha: None,
            },
            WorklogReceiptEntry {
                short_sha: "bbbbbbb".to_string(),
                full_sha: None,
            },
            WorklogReceiptEntry {
                short_sha: "ccccccc".to_string(),
                full_sha: None,
            },
        ];
        let report = compare_receipts(
            255,
            &worklog_receipts,
            &canonical_entries(&[
                ("aaaaaaa", "state(cycle-start): begin cycle [cycle 255]"),
                ("bbbbbbb", "state(process-merge): merge PR [cycle 255]"),
                ("ccccccc", "state(cycle-complete): close cycle [cycle 255]"),
                ("ddddddd", "docs(cycle-255): publish worklog and journal"),
                (
                    "eeeeeee",
                    "state(record-dispatch): issue #10 dispatched [cycle 255]",
                ),
                (
                    "fffffff",
                    "state(stabilization): clean cycle 255 — counter 3/50 [cycle 255]",
                ),
                (
                    "ggggggg",
                    "docs(worklog-patch): post-dispatch state correction [cycle 255]",
                ),
                (
                    "hhhhhhh",
                    "docs(review-body): cycle 255 review dispatch body [cycle 255]",
                ),
            ]),
            Vec::new(),
        );

        assert_eq!(report.result, "PASS");
        assert_eq!(report.genuinely_missing, 0);
        assert_eq!(report.structurally_excluded, 5);
        assert_eq!(report.excluded_details.len(), 5);
        assert!(report.validation_errors.is_empty());
    }

    #[test]
    fn fails_when_non_structural_receipt_is_missing() {
        let worklog_receipts = vec![
            WorklogReceiptEntry {
                short_sha: "aaaaaaa".to_string(),
                full_sha: None,
            },
            WorklogReceiptEntry {
                short_sha: "bbbbbbb".to_string(),
                full_sha: None,
            },
        ];
        let report = compare_receipts(
            255,
            &worklog_receipts,
            &canonical_entries(&[
                ("aaaaaaa", "state(cycle-start): begin cycle [cycle 255]"),
                ("bbbbbbb", "state(process-merge): merge PR [cycle 255]"),
                ("ccccccc", "state(cycle-complete): close cycle [cycle 255]"),
                ("ddddddd", "docs(cycle-255): publish worklog and journal"),
                (
                    "eeeeeee",
                    "state(record-dispatch): issue #10 dispatched [cycle 255]",
                ),
            ]),
            Vec::new(),
        );

        assert_eq!(report.result, "FAIL");
        assert_eq!(report.genuinely_missing, 1);
        assert_eq!(report.structurally_excluded, 2);
        assert_eq!(
            report.missing_details,
            vec![MissingDetail {
                sha: "ccccccc".to_string(),
                subject: "state(cycle-complete): close cycle [cycle 255]".to_string(),
            }]
        );
    }

    #[test]
    fn handles_empty_receipt_table() {
        let receipts = extract_worklog_receipts("# Worklog\n\nNo receipt table yet.\n")
            .expect("empty receipt table should not fail");

        assert!(receipts.is_empty());
    }

    #[test]
    fn parses_receipts_when_note_precedes_table() {
        let content = "\
## Commit receipts

> Note: docs and record-dispatch receipts are created later.

| Tool | Receipt | Link |
|------|---------|------|
| cycle-start | abc1234 | [abc1234](https://example.test/commit/abc1234) |
";

        let receipts = extract_worklog_receipts(content).expect("receipt table should parse");

        assert_eq!(
            receipts,
            vec![WorklogReceiptEntry {
                short_sha: "abc1234".to_string(),
                full_sha: None,
            }]
        );
    }

    #[test]
    fn fails_closed_for_malformed_receipt_table_separator() {
        let content = "\
## Commit receipts

| Tool | Receipt | Link |
| not-a-separator |
| cycle-start | abc1234 | [abc1234](https://example.test/commit/abc1234) |
";

        let error = extract_worklog_receipts(content).expect_err("malformed table should fail");

        assert!(error.contains("malformed separator"));
    }

    #[test]
    fn full_sha_regex_extracts_supported_markdown_formats() {
        let full_sha = "4e64161d6675ca2aac5915527f39b891af1bbe68";
        let expected = Some(full_sha.to_string());

        assert_eq!(
            extract_full_sha_from_cell(&format!(
                "[4e64161](https://github.com/EvaLok/schema-org-json-ld/commit/{full_sha})"
            )),
            expected
        );
        assert_eq!(
            extract_full_sha_from_cell(&format!(
                "<https://github.com/EvaLok/schema-org-json-ld/commit/{full_sha}>"
            )),
            Some(full_sha.to_string())
        );
        assert_eq!(
            extract_full_sha_from_cell(&format!(
                "[link](https://github.com/EvaLok/schema-org-json-ld/commit/{full_sha}#diff)"
            )),
            Some(full_sha.to_string())
        );
        assert_eq!(
            extract_full_sha_from_cell("https://github.com/other/repo/commit/4e64161d6675ca2aac5915527f39b891af1bbe68"),
            None
        );
    }

    #[test]
    fn run_passes_with_correct_full_sha_urls() {
        let repo = TestRepo::new("receipt-validate-correct-full-sha");
        repo.init();
        let (short_sha, full_sha) =
            repo.commit("notes/valid.txt", "valid\n", "state(cycle-complete): close cycle [cycle 1]");
        repo.write_cycle_receipts(&[(&short_sha, "state(cycle-complete): close cycle [cycle 1]")]);
        let worklog_path = repo.write_worklog(&format!(
            "## Commit receipts\n\n| Tool | Receipt | Link |\n|------|---------|------|\n| cycle-complete | {short_sha} | [link](https://github.com/EvaLok/schema-org-json-ld/commit/{full_sha}) |\n"
        ));

        let report = run(&Cli {
            cycle: 1,
            worklog: worklog_path,
            repo_root: repo.path.clone(),
            json: false,
        })
        .expect("validation should run");

        assert_eq!(report.result, "PASS");
        assert!(report.validation_errors.is_empty());
    }

    #[test]
    fn run_fails_with_broken_full_sha_url() {
        let repo = TestRepo::new("receipt-validate-broken-full-sha");
        repo.init();
        let (short_sha, _full_sha) =
            repo.commit("notes/valid.txt", "valid\n", "state(cycle-complete): close cycle [cycle 1]");
        let broken_full_sha = format!("{short_sha}d2b1e2c26e40bff92f8f4e0210f3e0fd2");
        repo.write_cycle_receipts(&[(&short_sha, "state(cycle-complete): close cycle [cycle 1]")]);
        let worklog_path = repo.write_worklog(&format!(
            "## Commit receipts\n\n| Tool | Receipt | Link |\n|------|---------|------|\n| cycle-complete | {short_sha} | [link](https://github.com/EvaLok/schema-org-json-ld/commit/{broken_full_sha}) |\n"
        ));

        let report = run(&Cli {
            cycle: 1,
            worklog: worklog_path,
            repo_root: repo.path.clone(),
            json: false,
        })
        .expect("validation should run");

        assert_eq!(report.result, "FAIL");
        assert_eq!(
            report.validation_errors,
            vec![format!(
                "FAIL: Receipt {short_sha} links to non-existent commit {broken_full_sha}"
            )]
        );
    }

    #[test]
    fn run_keeps_short_sha_only_validation_backwards_compatible() {
        let repo = TestRepo::new("receipt-validate-short-only");
        repo.init();
        let (short_sha, _full_sha) =
            repo.commit("notes/valid.txt", "valid\n", "state(cycle-complete): close cycle [cycle 1]");
        repo.write_cycle_receipts(&[(&short_sha, "state(cycle-complete): close cycle [cycle 1]")]);
        let worklog_path = repo.write_worklog(&format!(
            "## Commit receipts\n\n| Tool | Receipt | Link |\n|------|---------|------|\n| cycle-complete | {short_sha} | plain text |\n"
        ));

        let report = run(&Cli {
            cycle: 1,
            worklog: worklog_path,
            repo_root: repo.path.clone(),
            json: false,
        })
        .expect("validation should run");

        assert_eq!(report.result, "PASS");
        assert!(report.validation_errors.is_empty());
    }

    struct TestRepo {
        path: PathBuf,
    }

    impl TestRepo {
        fn new(name: &str) -> Self {
            let unique = format!(
                "{name}-{}-{}",
                std::process::id(),
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("clock before epoch")
                    .as_nanos()
            );
            let path = env::temp_dir().join(unique);
            fs::create_dir_all(&path).expect("create temp repo dir");
            Self { path }
        }

        fn init(&self) {
            git_success(&self.path, ["init"]);
            git_success(&self.path, ["config", "user.name", "Receipt Validate Tests"]);
            git_success(
                &self.path,
                ["config", "user.email", "receipt-validate-tests@example.com"],
            );
            self.write_file("README.md", "test repo\n");
            git_success(&self.path, ["add", "--", "README.md"]);
            git_success(&self.path, ["commit", "-m", "initial commit"]);
        }

        fn commit(&self, relative_path: &str, contents: &str, message: &str) -> (String, String) {
            self.write_file(relative_path, contents);
            git_success(&self.path, ["add", "--", relative_path]);
            git_success(&self.path, ["commit", "-m", message]);

            let short_sha = git_stdout(&self.path, ["rev-parse", "--short=7", "HEAD"])
                .trim()
                .to_string();
            let full_sha = git_stdout(&self.path, ["rev-parse", "HEAD"]).trim().to_string();
            (short_sha, full_sha)
        }

        fn write_worklog(&self, contents: &str) -> PathBuf {
            let path = self.path.join("docs/worklog.md");
            fs::create_dir_all(path.parent().expect("worklog parent"))
                .expect("create worklog dir");
            fs::write(&path, contents).expect("write worklog");
            path
        }

        fn write_cycle_receipts(&self, entries: &[(&str, &str)]) {
            let tools_dir = self.path.join("tools");
            fs::create_dir_all(&tools_dir).expect("create tools dir");
            let wrapper = tools_dir.join("cycle-receipts");
            let json = serde_json::to_string(
                &entries
                    .iter()
                    .map(|(receipt, commit)| CanonicalReceiptEntry {
                        receipt: (*receipt).to_string(),
                        commit: (*commit).to_string(),
                    })
                    .collect::<Vec<_>>(),
            )
            .expect("serialize canonical entries");
            fs::write(
                &wrapper,
                format!("#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' '{json}'\n"),
            )
            .expect("write cycle-receipts wrapper");
            let mut permissions = fs::metadata(&wrapper)
                .expect("wrapper metadata")
                .permissions();
            permissions.set_mode(0o755);
            fs::set_permissions(&wrapper, permissions).expect("set wrapper permissions");
        }

        fn write_file(&self, relative_path: &str, contents: &str) {
            let path = self.path.join(relative_path);
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent).expect("create parent directories");
            }
            fs::write(path, contents).expect("write file");
        }
    }

    impl Drop for TestRepo {
        fn drop(&mut self) {
            match fs::remove_dir_all(&self.path) {
                Ok(()) => {}
                Err(error) if error.kind() == ErrorKind::NotFound => {}
                Err(error) => eprintln!(
                    "Warning: failed to remove test repo {}: {}",
                    self.path.display(),
                    error
                ),
            }
        }
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
        String::from_utf8(output.stdout).expect("git stdout should be UTF-8")
    }

    fn run_test_git<I, S>(repo_root: &Path, args: I) -> (Vec<String>, std::process::Output)
    where
        I: IntoIterator<Item = S>,
        S: AsRef<std::ffi::OsStr>,
    {
        let rendered_args = args
            .into_iter()
            .map(|arg| arg.as_ref().to_string_lossy().to_string())
            .collect::<Vec<_>>();
        let output = Command::new("git")
            .arg("-C")
            .arg(repo_root)
            .args(&rendered_args)
            .output()
            .expect("run git command");
        (rendered_args, output)
    }
}
