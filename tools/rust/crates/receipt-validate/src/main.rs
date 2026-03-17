use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::OnceLock;

const RECEIPT_HEADER_PATTERN_STR: &str = r"^\|\s*Tool\s*\|\s*Receipt\s*\|\s*Link\s*\|\s*$";
const RECEIPT_SEPARATOR_PATTERN_STR: &str = r"^\|\s*-+\s*\|\s*-+\s*\|\s*-+\s*\|\s*$";
const RECEIPT_CELL_PATTERN_STR: &str = r"(?i)\b([0-9a-f]{7})\b";

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

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
struct CanonicalReceiptEntry {
    receipt: String,
    commit: String,
}

#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
struct MissingDetail {
    sha: String,
    subject: String,
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
    let canonical_receipts = fetch_cycle_receipts(&cli.repo_root, cli.cycle)?;
    Ok(compare_receipts(
        cli.cycle,
        &worklog_receipts,
        &canonical_receipts,
    ))
}

fn extract_worklog_receipts(content: &str) -> Result<BTreeSet<String>, String> {
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

        let mut receipts = BTreeSet::new();
        for row in lines.by_ref() {
            let trimmed = row.trim();
            if !trimmed.starts_with('|') {
                break;
            }

            if let Some(receipt) = extract_receipt_from_row(trimmed)? {
                receipts.insert(receipt);
            }
        }

        return Ok(receipts);
    }

    Ok(BTreeSet::new())
}

fn compare_receipts(
    cycle: u64,
    worklog_receipts: &BTreeSet<String>,
    canonical_receipts: &[CanonicalReceiptEntry],
) -> ValidationReport {
    let mut missing_details = Vec::new();
    let mut excluded_details = Vec::new();

    for entry in canonical_receipts {
        let receipt = entry.receipt.trim().to_ascii_lowercase();
        if receipt.is_empty() || worklog_receipts.contains(&receipt) {
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

    let result = if missing_details.is_empty() {
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
        result: result.to_string(),
    }
}

fn is_structurally_excluded(subject: &str) -> bool {
    subject.starts_with("docs(cycle-") || subject.starts_with("state(record-dispatch):")
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

fn extract_receipt_from_row(row: &str) -> Result<Option<String>, String> {
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
    extract_receipt_from_cell(cell)
}

fn extract_receipt_from_cell(cell: &str) -> Result<Option<String>, String> {
    Ok(receipt_cell_pattern()
        .captures(cell)
        .and_then(|captures| captures.get(1))
        .map(|capture| capture.as_str().to_ascii_lowercase()))
}

fn render_structural_suffix(details: &[MissingDetail]) -> String {
    let mut categories = Vec::new();
    if details
        .iter()
        .any(|detail| detail.subject.starts_with("docs(cycle-"))
    {
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

#[cfg(test)]
mod tests {
    use super::*;

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
            BTreeSet::from(["abc1234".to_string(), "def5678".to_string(),])
        );
    }

    #[test]
    fn detects_structural_exclusions_by_subject_prefix() {
        assert!(is_structurally_excluded(
            "docs(cycle-255): publish worklog and journal"
        ));
        assert!(is_structurally_excluded(
            "state(record-dispatch): issue #10 dispatched [cycle 255]"
        ));
        assert!(!is_structurally_excluded(
            "state(cycle-complete): close cycle [cycle 255]"
        ));
    }

    #[test]
    fn passes_when_only_structural_receipts_are_missing() {
        let worklog_receipts = BTreeSet::from([
            "aaaaaaa".to_string(),
            "bbbbbbb".to_string(),
            "ccccccc".to_string(),
        ]);
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
        );

        assert_eq!(report.result, "PASS");
        assert_eq!(report.genuinely_missing, 0);
        assert_eq!(report.structurally_excluded, 2);
        assert_eq!(report.excluded_details.len(), 2);
    }

    #[test]
    fn fails_when_non_structural_receipt_is_missing() {
        let worklog_receipts = BTreeSet::from(["aaaaaaa".to_string(), "bbbbbbb".to_string()]);
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

        assert_eq!(receipts, BTreeSet::from(["abc1234".to_string()]));
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
}
