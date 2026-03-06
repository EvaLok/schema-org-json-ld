use chrono::{DateTime, NaiveDate, Utc};
use clap::{Parser, Subcommand};
use serde::Deserialize;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

const PRIMARY_ISSUES_URL: &str = "https://github.com/EvaLok/schema-org-json-ld/issues";
const QC_ISSUES_URL: &str = "https://github.com/EvaLok/schema-org-json-ld-qc/issues";
const AUDIT_ISSUES_URL: &str = "https://github.com/EvaLok/schema-org-json-ld-audit/issues";
const JOURNAL_DESCRIPTION: &str = "Reflective log for the schema-org-json-ld orchestrator.";

#[derive(Parser)]
#[command(name = "write-entry")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Generate a worklog entry file
    Worklog(WorklogArgs),
    /// Generate or append a journal entry file
    Journal(JournalArgs),
}

#[derive(Parser)]
struct WorklogArgs {
    /// Cycle number
    #[arg(long)]
    cycle: u64,
    /// Short descriptive name for heading and filename slug
    #[arg(long)]
    title: String,
    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Parser)]
struct JournalArgs {
    /// Cycle number
    #[arg(long)]
    cycle: u64,
    /// Entry title
    #[arg(long)]
    title: String,
    /// Repository root path
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,
}

#[derive(Deserialize)]
struct WorklogInput {
    what_was_done: Vec<String>,
    self_modifications: Vec<SelfModification>,
    prs_merged: Vec<u64>,
    prs_reviewed: Vec<u64>,
    issues_processed: Vec<u64>,
    current_state: CurrentState,
    next_steps: Vec<String>,
}

#[derive(Deserialize)]
struct SelfModification {
    file: String,
    description: String,
}

#[derive(Deserialize)]
struct CurrentState {
    in_flight_sessions: u64,
    pipeline_status: String,
    copilot_metrics: String,
    publish_gate: String,
}

#[derive(Deserialize)]
struct JournalInput {
    previous_commitment_status: String,
    previous_commitment_detail: String,
    sections: Vec<JournalSection>,
    concrete_behavior_change: String,
    open_questions: Vec<String>,
}

#[derive(Deserialize)]
struct JournalSection {
    heading: String,
    body: String,
}

fn main() {
    let cli = Cli::parse();
    let now = Utc::now();
    let stdin = match read_stdin() {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    };

    let result = match cli.command {
        Command::Worklog(args) => execute_worklog(&args, now, &stdin),
        Command::Journal(args) => execute_journal(&args, now, &stdin),
    };

    match result {
        Ok(path) => println!("{}", path.display()),
        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}

fn read_stdin() -> Result<String, String> {
    use std::io::Read;
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .map_err(|error| format!("failed to read stdin: {}", error))?;
    if input.trim().is_empty() {
        return Err("stdin JSON payload is required".to_string());
    }
    Ok(input)
}

fn execute_worklog(args: &WorklogArgs, now: DateTime<Utc>, stdin: &str) -> Result<PathBuf, String> {
    let input: WorklogInput = serde_json::from_str(stdin)
        .map_err(|error| format!("invalid worklog JSON input: {}", error))?;
    let path = worklog_path(&args.repo_root, now, &args.title);
    let content = render_worklog(args.cycle, now, &input);
    write_entry_file(&path, &content)?;
    Ok(path)
}

fn execute_journal(args: &JournalArgs, now: DateTime<Utc>, stdin: &str) -> Result<PathBuf, String> {
    let input: JournalInput = serde_json::from_str(stdin)
        .map_err(|error| format!("invalid journal JSON input: {}", error))?;
    let status = parse_commitment_status(&input.previous_commitment_status)?;
    let path = journal_path(&args.repo_root, now);
    let previous = lookup_previous_concrete_behavior(&args.repo_root, now.date_naive())?;
    let entry = render_journal_entry(
        args.cycle,
        now,
        &args.title,
        &input,
        status,
        previous.as_deref(),
    );
    write_journal_file(&path, now.date_naive(), &entry)?;
    Ok(path)
}

fn worklog_path(repo_root: &Path, now: DateTime<Utc>, title: &str) -> PathBuf {
    let date = now.format("%Y-%m-%d").to_string();
    let time = now.format("%H%M%S").to_string();
    let slug = slugify(title);
    repo_root
        .join("docs")
        .join("worklog")
        .join(date)
        .join(format!("{}-{}.md", time, slug))
}

fn journal_path(repo_root: &Path, now: DateTime<Utc>) -> PathBuf {
    repo_root
        .join("docs")
        .join("journal")
        .join(format!("{}.md", now.format("%Y-%m-%d")))
}

fn write_entry_file(path: &Path, content: &str) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("invalid output path {}", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("failed to create {}: {}", parent.display(), error))?;
    fs::write(path, content)
        .map_err(|error| format!("failed to write {}: {}", path.display(), error))
}

fn write_journal_file(path: &Path, date: NaiveDate, entry: &str) -> Result<(), String> {
    let parent = path
        .parent()
        .ok_or_else(|| format!("invalid output path {}", path.display()))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("failed to create {}: {}", parent.display(), error))?;

    if path.exists() {
        let mut existing = fs::read_to_string(path)
            .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
        if !existing.ends_with('\n') {
            existing.push('\n');
        }
        existing.push('\n');
        existing.push_str("---\n\n");
        existing.push_str(entry);
        fs::write(path, existing)
            .map_err(|error| format!("failed to write {}: {}", path.display(), error))
    } else {
        let header = format!("# Journal — {date}\n\n{JOURNAL_DESCRIPTION}\n\n---\n\n",);
        let content = format!("{header}{entry}");
        fs::write(path, content)
            .map_err(|error| format!("failed to write {}: {}", path.display(), error))
    }
}

fn render_worklog(cycle: u64, now: DateTime<Utc>, input: &WorklogInput) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "# Cycle {} — {} {} UTC",
        cycle,
        now.format("%Y-%m-%d"),
        now.format("%H:%M")
    ));
    lines.push(String::new());
    lines.push("## What was done".to_string());
    lines.push(String::new());
    if input.what_was_done.is_empty() {
        lines.push("- None.".to_string());
    } else {
        for item in &input.what_was_done {
            lines.push(format!("- {}", convert_references(item)));
        }
    }
    lines.push(String::new());
    lines.push("### PRs merged".to_string());
    lines.push(String::new());
    lines.extend(render_numbered_refs(
        &input.prs_merged,
        "PR",
        PRIMARY_ISSUES_URL,
    ));
    lines.push(String::new());
    lines.push("### PRs reviewed".to_string());
    lines.push(String::new());
    lines.extend(render_numbered_refs(
        &input.prs_reviewed,
        "PR",
        PRIMARY_ISSUES_URL,
    ));
    lines.push(String::new());
    lines.push("### Issues processed".to_string());
    lines.push(String::new());
    lines.extend(render_numbered_refs(
        &input.issues_processed,
        "issue",
        PRIMARY_ISSUES_URL,
    ));
    lines.push(String::new());
    lines.push("## Self-modifications".to_string());
    lines.push(String::new());
    if input.self_modifications.is_empty() {
        lines.push("- None.".to_string());
    } else {
        for item in &input.self_modifications {
            lines.push(format!(
                "- **`{}`**: {}",
                item.file,
                convert_references(&item.description)
            ));
        }
    }
    lines.push(String::new());
    lines.push("## Current state".to_string());
    lines.push(String::new());
    lines.push(format!(
        "- **In-flight agent sessions**: {}",
        input.current_state.in_flight_sessions
    ));
    lines.push(format!(
        "- **Pipeline status**: {}",
        convert_references(&input.current_state.pipeline_status)
    ));
    lines.push(format!(
        "- **Copilot metrics**: {}",
        convert_references(&input.current_state.copilot_metrics)
    ));
    lines.push(format!(
        "- **Publish gate**: {}",
        convert_references(&input.current_state.publish_gate)
    ));
    lines.push(String::new());
    lines.push("## Next steps".to_string());
    lines.push(String::new());
    if input.next_steps.is_empty() {
        lines.push("1. None.".to_string());
    } else {
        for (index, step) in input.next_steps.iter().enumerate() {
            lines.push(format!("{}. {}", index + 1, convert_references(step)));
        }
    }
    lines.push(String::new());
    lines.join("\n")
}

fn render_numbered_refs(numbers: &[u64], kind: &str, issues_url: &str) -> Vec<String> {
    if numbers.is_empty() {
        return vec!["- None.".to_string()];
    }

    numbers
        .iter()
        .map(|number| match kind {
            "PR" => format!("- [PR #{}]({}/{})", number, issues_url, number),
            "issue" => format!("- [#{}]({}/{})", number, issues_url, number),
            _ => format!("- [{} #{}]({}/{})", kind, number, issues_url, number),
        })
        .collect()
}

#[derive(Clone, Copy)]
enum CommitmentStatus {
    Followed,
    NotFollowed,
    NotApplicable,
    NoPriorCommitment,
}

fn parse_commitment_status(value: &str) -> Result<CommitmentStatus, String> {
    match value {
		"followed" => Ok(CommitmentStatus::Followed),
		"not_followed" => Ok(CommitmentStatus::NotFollowed),
		"not_applicable" => Ok(CommitmentStatus::NotApplicable),
		"no_prior_commitment" => Ok(CommitmentStatus::NoPriorCommitment),
		_ => Err(format!(
			"invalid previous_commitment_status '{}'; expected one of: followed, not_followed, not_applicable, no_prior_commitment",
			value
		)),
	}
}

fn render_journal_entry(
    cycle: u64,
    now: DateTime<Utc>,
    title: &str,
    input: &JournalInput,
    status: CommitmentStatus,
    previous_commitment: Option<&str>,
) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "## {} — Cycle {}: {}",
        now.format("%Y-%m-%d"),
        cycle,
        title
    ));
    lines.push(String::new());
    lines.push("### Context".to_string());
    lines.push(String::new());
    lines.push(format!(
        "Cycle {} focused on {}.",
        cycle,
        convert_references(title)
    ));
    lines.push(String::new());
    lines.push("### Previous commitment follow-through".to_string());
    lines.push(String::new());
    if let Some(previous) = previous_commitment {
        lines.push(format!(
            "> Previous commitment: {}",
            convert_references(previous)
        ));
        lines.push(String::new());
    }
    lines.push(format!(
        "{} {}",
        commitment_status_label(status),
        convert_references(&input.previous_commitment_detail)
    ));
    lines.push(String::new());
    for section in &input.sections {
        lines.push(format!("### {}", convert_references(&section.heading)));
        lines.push(String::new());
        lines.push(convert_references(&section.body));
        lines.push(String::new());
    }
    lines.push("### Concrete behavior change this cycle".to_string());
    lines.push(String::new());
    lines.push(convert_references(&input.concrete_behavior_change));
    lines.push(String::new());
    lines.push("### Open questions".to_string());
    lines.push(String::new());
    if input.open_questions.is_empty() {
        lines.push("- None.".to_string());
    } else {
        for question in &input.open_questions {
            lines.push(format!("- {}", convert_references(question)));
        }
    }
    lines.push(String::new());
    lines.join("\n")
}

fn commitment_status_label(status: CommitmentStatus) -> &'static str {
    match status {
        CommitmentStatus::Followed => "**Followed.**",
        CommitmentStatus::NotFollowed => "**Not followed.**",
        CommitmentStatus::NotApplicable => "**Not applicable.**",
        CommitmentStatus::NoPriorCommitment => "**No prior commitment.**",
    }
}

fn lookup_previous_concrete_behavior(
    repo_root: &Path,
    today: NaiveDate,
) -> Result<Option<String>, String> {
    let journal_dir = repo_root.join("docs").join("journal");
    if !journal_dir.exists() {
        return Ok(None);
    }

    let mut dated_files = Vec::<(NaiveDate, PathBuf)>::new();
    let entries = fs::read_dir(&journal_dir)
        .map_err(|error| format!("failed to read {}: {}", journal_dir.display(), error))?;
    for entry in entries {
        let entry = entry.map_err(|error| {
            format!(
                "failed to read entry in {}: {}",
                journal_dir.display(),
                error
            )
        })?;
        let path = entry.path();
        if path.extension() != Some(OsStr::new("md")) {
            continue;
        }
        let Some(stem) = path.file_stem().and_then(OsStr::to_str) else {
            continue;
        };
        let Ok(date) = NaiveDate::parse_from_str(stem, "%Y-%m-%d") else {
            continue;
        };
        if date <= today {
            dated_files.push((date, path));
        }
    }

    dated_files.sort_by(|a, b| a.0.cmp(&b.0));
    for (_, path) in dated_files.into_iter().rev() {
        let content = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {}", path.display(), error))?;
        if let Some(section) = extract_last_concrete_behavior(&content) {
            return Ok(Some(section));
        }
    }
    Ok(None)
}

fn extract_last_concrete_behavior(content: &str) -> Option<String> {
    const HEADING: &str = "### Concrete behavior change this cycle";
    let mut line_starts = vec![0usize];
    for (idx, ch) in content.char_indices() {
        if ch == '\n' {
            line_starts.push(idx + 1);
        }
    }

    let mut latest: Option<String> = None;
    for (line_index, start) in line_starts.iter().enumerate() {
        let line = line_text(content, *start);
        if line.trim() != HEADING {
            continue;
        }
        let mut end = content.len();
        for next_start in line_starts.iter().skip(line_index + 1) {
            let next_line = line_text(content, *next_start);
            let trimmed = next_line.trim();
            if trimmed.starts_with("### ") || trimmed == "---" {
                end = *next_start;
                break;
            }
        }
        let block_start = line_end_index(content, *start);
        let section = content[block_start..end].trim();
        if !section.is_empty() {
            latest = Some(section.to_string());
        }
    }
    latest
}

fn line_text(content: &str, start: usize) -> &str {
    let rest = &content[start..];
    match rest.find('\n') {
        Some(index) => &rest[..index],
        None => rest,
    }
}

fn line_end_index(content: &str, start: usize) -> usize {
    let rest = &content[start..];
    match rest.find('\n') {
        Some(index) => start + index + 1,
        None => content.len(),
    }
}

fn slugify(title: &str) -> String {
    let mut output = String::new();
    let mut in_hyphen = false;
    for ch in title.chars() {
        let mapped = ch.to_ascii_lowercase();
        if mapped.is_ascii_alphanumeric() {
            output.push(mapped);
            in_hyphen = false;
        } else if !in_hyphen {
            output.push('-');
            in_hyphen = true;
        }
    }
    let slug = output.trim_matches('-').to_string();
    if slug.is_empty() {
        "entry".to_string()
    } else {
        slug
    }
}

fn convert_references(text: &str) -> String {
    let link_spans = markdown_link_spans(text);
    let mut output = String::new();
    let mut cursor = 0usize;
    for (start, end) in link_spans {
        if cursor < start {
            output.push_str(&convert_segment(&text[cursor..start]));
        }
        output.push_str(&text[start..end]);
        cursor = end;
    }
    if cursor < text.len() {
        output.push_str(&convert_segment(&text[cursor..]));
    }
    output
}

fn markdown_link_spans(text: &str) -> Vec<(usize, usize)> {
    let bytes = text.as_bytes();
    let mut spans = Vec::new();
    let mut i = 0usize;
    while i < bytes.len() {
        if bytes[i] != b'[' {
            i += 1;
            continue;
        }
        let mut j = i + 1;
        while j < bytes.len() && bytes[j] != b']' {
            j += 1;
        }
        if j + 1 >= bytes.len() || bytes[j + 1] != b'(' {
            i += 1;
            continue;
        }
        let mut k = j + 2;
        while k < bytes.len() && bytes[k] != b')' {
            k += 1;
        }
        if k >= bytes.len() {
            i += 1;
            continue;
        }
        spans.push((i, k + 1));
        i = k + 1;
    }
    spans
}

fn convert_segment(segment: &str) -> String {
    let chars: Vec<char> = segment.chars().collect();
    let mut output = String::new();
    let mut i = 0usize;

    while i < chars.len() {
        if let Some((replacement, next)) =
            match_named_reference(&chars, i, "PR", PRIMARY_ISSUES_URL)
        {
            output.push_str(&replacement);
            i = next;
            continue;
        }
        if let Some((replacement, next)) = match_named_reference(&chars, i, "QC", QC_ISSUES_URL) {
            output.push_str(&replacement);
            i = next;
            continue;
        }
        if let Some((replacement, next)) =
            match_named_reference(&chars, i, "audit", AUDIT_ISSUES_URL)
        {
            output.push_str(&replacement);
            i = next;
            continue;
        }
        if let Some((replacement, next)) =
            match_named_reference(&chars, i, "Audit", AUDIT_ISSUES_URL)
        {
            output.push_str(&replacement);
            i = next;
            continue;
        }
        if chars[i] == '#' {
            let prev = i.checked_sub(1).and_then(|idx| chars.get(idx)).copied();
            if prev != Some('[') {
                let (digits, end) = parse_digits(&chars, i + 1);
                if !digits.is_empty() {
                    let next_char = chars.get(end).copied();
                    if next_char != Some(']') {
                        output
                            .push_str(&format!("[#{}]({}/{})", digits, PRIMARY_ISSUES_URL, digits));
                        i = end;
                        continue;
                    }
                }
            }
        }
        output.push(chars[i]);
        i += 1;
    }

    output
}

fn match_named_reference(
    chars: &[char],
    start: usize,
    label: &str,
    base_url: &str,
) -> Option<(String, usize)> {
    let mut idx = start;
    for expected in label.chars() {
        if chars.get(idx).copied()? != expected {
            return None;
        }
        idx += 1;
    }
    if chars.get(idx).copied()? != ' ' || chars.get(idx + 1).copied()? != '#' {
        return None;
    }
    let (digits, end) = parse_digits(chars, idx + 2);
    if digits.is_empty() {
        return None;
    }
    let replacement = format!("[{} #{}]({}/{})", label, digits, base_url, digits);
    Some((replacement, end))
}

fn parse_digits(chars: &[char], start: usize) -> (String, usize) {
    let mut idx = start;
    let mut digits = String::new();
    while let Some(ch) = chars.get(idx) {
        if ch.is_ascii_digit() {
            digits.push(*ch);
            idx += 1;
        } else {
            break;
        }
    }
    (digits, idx)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    struct TempRepoDir {
        path: PathBuf,
    }

    impl TempRepoDir {
        fn new(prefix: &str) -> Self {
            static COUNTER: AtomicU64 = AtomicU64::new(0);
            let run_id = COUNTER.fetch_add(1, Ordering::Relaxed);
            let nanos = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let path = std::env::temp_dir().join(format!(
                "write-entry-{}-{}-{}-{}",
                prefix,
                std::process::id(),
                nanos,
                run_id
            ));
            fs::create_dir_all(&path).unwrap();
            Self { path }
        }
    }

    impl Drop for TempRepoDir {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.path);
        }
    }

    fn fixed_now() -> DateTime<Utc> {
        DateTime::parse_from_rfc3339("2026-03-06T05:14:58Z")
            .unwrap()
            .with_timezone(&Utc)
    }

    #[test]
    fn converts_issue_references_and_preserves_existing_links() {
        let input = "Refs: #42, PR #10, QC #11, audit #12, [#13](https://github.com/EvaLok/schema-org-json-ld/issues/13)";
        let output = convert_references(input);
        assert!(output.contains("[#42](https://github.com/EvaLok/schema-org-json-ld/issues/42)"));
        assert!(output.contains("[PR #10](https://github.com/EvaLok/schema-org-json-ld/issues/10)"));
        assert!(
            output.contains("[QC #11](https://github.com/EvaLok/schema-org-json-ld-qc/issues/11)")
        );
        assert!(output
            .contains("[audit #12](https://github.com/EvaLok/schema-org-json-ld-audit/issues/12)"));
        assert!(convert_references("Audit #14")
            .contains("[Audit #14](https://github.com/EvaLok/schema-org-json-ld-audit/issues/14)"));
        assert!(output.contains("[#13](https://github.com/EvaLok/schema-org-json-ld/issues/13)"));
        assert_eq!(
            output
                .matches("[#13](https://github.com/EvaLok/schema-org-json-ld/issues/13)")
                .count(),
            1
        );
    }

    #[test]
    fn worklog_path_uses_date_time_and_slug() {
        let repo_root = PathBuf::from("/tmp/example");
        let path = worklog_path(&repo_root, fixed_now(), "From Convention to Enforcement");
        assert_eq!(
            path,
            PathBuf::from(
                "/tmp/example/docs/worklog/2026-03-06/051458-from-convention-to-enforcement.md"
            )
        );
    }

    #[test]
    fn worklog_template_keeps_required_section_order() {
        let input = WorklogInput {
            what_was_done: vec!["Fixed #42".to_string()],
            self_modifications: vec![SelfModification {
                file: "STARTUP_CHECKLIST.md".to_string(),
                description: "Updated per audit #117".to_string(),
            }],
            prs_merged: vec![537],
            prs_reviewed: vec![543],
            issues_processed: vec![546],
            current_state: CurrentState {
                in_flight_sessions: 2,
                pipeline_status: "5/5 phases pass".to_string(),
                copilot_metrics: "64 dispatches".to_string(),
                publish_gate: "Source diverged".to_string(),
            },
            next_steps: vec!["Review PR #543".to_string()],
        };
        let rendered = render_worklog(154, fixed_now(), &input);
        let what_done = rendered.find("## What was done").unwrap();
        let self_mods = rendered.find("## Self-modifications").unwrap();
        let current = rendered.find("## Current state").unwrap();
        let next = rendered.find("## Next steps").unwrap();
        assert!(what_done < self_mods);
        assert!(self_mods < current);
        assert!(current < next);
        assert!(rendered.contains("[#42](https://github.com/EvaLok/schema-org-json-ld/issues/42)"));
        assert!(rendered.contains(
            "[audit #117](https://github.com/EvaLok/schema-org-json-ld-audit/issues/117)"
        ));
    }

    #[test]
    fn journal_create_and_append_use_separator() {
        let repo_root = TempRepoDir::new("append");
        let now = fixed_now();
        let args = JournalArgs {
            cycle: 154,
            title: "From convention to enforcement".to_string(),
            repo_root: repo_root.path.clone(),
        };
        let payload = r#"{
			"previous_commitment_status":"followed",
			"previous_commitment_detail":"Ran cargo test after PR #543.",
			"sections":[{"heading":"Observation — Enforcement","body":"Audit #117 was right."}],
			"concrete_behavior_change":"Dispatch #546 immediately after acceptance.",
			"open_questions":[]
		}"#;

        execute_journal(&args, now, payload).unwrap();
        execute_journal(&args, now, payload).unwrap();

        let path = journal_path(&repo_root.path, now);
        let content = fs::read_to_string(path).unwrap();
        assert!(content.starts_with("# Journal — 2026-03-06"));
        assert!(
            content.contains("\n---\n\n## 2026-03-06 — Cycle 154: From convention to enforcement")
        );
        assert_eq!(
            content
                .matches("\n## 2026-03-06 — Cycle 154: From convention to enforcement\n")
                .count(),
            2
        );
    }

    #[test]
    fn journal_includes_previous_commitment_quote_from_last_entry() {
        let repo_root = TempRepoDir::new("previous");
        let journal_dir = repo_root.path.join("docs").join("journal");
        fs::create_dir_all(&journal_dir).unwrap();
        let existing = r#"# Journal — 2026-03-05

Reflective log for the schema-org-json-ld orchestrator.

---

## 2026-03-05 — Cycle 153: Prior title

### Concrete behavior change this cycle

When accepting recommendations, dispatch #546 in the same cycle.
"#;
        fs::write(journal_dir.join("2026-03-05.md"), existing).unwrap();

        let args = JournalArgs {
            cycle: 154,
            title: "New title".to_string(),
            repo_root: repo_root.path.clone(),
        };
        let payload = r#"{
			"previous_commitment_status":"followed",
			"previous_commitment_detail":"Done.",
			"sections":[],
			"concrete_behavior_change":"Keep going.",
			"open_questions":[]
		}"#;
        execute_journal(&args, fixed_now(), payload).unwrap();

        let content = fs::read_to_string(journal_path(&repo_root.path, fixed_now())).unwrap();
        assert!(content.contains("> Previous commitment: When accepting recommendations, dispatch [#546](https://github.com/EvaLok/schema-org-json-ld/issues/546) in the same cycle."));
    }

    #[test]
    fn invalid_previous_commitment_status_is_rejected() {
        let repo_root = TempRepoDir::new("status");
        let args = JournalArgs {
            cycle: 154,
            title: "Invalid status".to_string(),
            repo_root: repo_root.path.clone(),
        };
        let payload = r#"{
			"previous_commitment_status":"unknown",
			"previous_commitment_detail":"Done.",
			"sections":[],
			"concrete_behavior_change":"Keep going.",
			"open_questions":[]
		}"#;
        let error = execute_journal(&args, fixed_now(), payload).unwrap_err();
        assert!(error.contains("invalid previous_commitment_status"));
    }
}
