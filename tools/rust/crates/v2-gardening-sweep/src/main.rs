use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

const SCHEMA: &str = "v2-gardening-sweep/v1";
const DEFAULT_STALE_DAYS: u64 = 30;
const DEFAULT_EXTENSION: &str = "md";
const SECONDS_PER_DAY: u64 = 86_400;

#[derive(Parser, Debug)]
#[command(
    name = "v2-gardening-sweep",
    about = "Stale-detection + dead-link-detection deslop pass over markdown corpora"
)]
struct Args {
    /// Corpus directory to sweep. May be repeated to sweep multiple roots
    /// (e.g. --corpus docs/redesign/_notes --corpus docs/journal).
    #[arg(long, required = true)]
    corpus: Vec<PathBuf>,

    /// Stale threshold in days. Files with mtime older than this are flagged.
    /// Default 30.
    #[arg(long, default_value_t = DEFAULT_STALE_DAYS)]
    stale_days: u64,

    /// Skip stale-detection (run only dead-link-detection).
    #[arg(long)]
    no_stale: bool,

    /// Skip dead-link-detection (run only stale-detection).
    #[arg(long)]
    no_dead_links: bool,

    /// File extension to scan (no leading dot). Default "md".
    #[arg(long, default_value = DEFAULT_EXTENSION)]
    extension: String,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    format: OutputFormat,

    /// Write output to a file ("-" for stdout, default).
    #[arg(long, default_value = "-")]
    output: String,

    /// Non-zero exit if any stale file or dead link is found.
    #[arg(long)]
    strict: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    Json,
    Text,
}

#[derive(Debug, Serialize, Deserialize)]
struct Report {
    schema: String,
    corpus: Vec<String>,
    files_scanned: usize,
    stale_threshold_days: u64,
    stale_detection_enabled: bool,
    dead_link_detection_enabled: bool,
    stale: Vec<StaleEntry>,
    dead_links: Vec<DeadLinkEntry>,
    warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct StaleEntry {
    path: String,
    age_days: u64,
    last_modified_unix: u64,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct DeadLinkEntry {
    source: String,
    line: usize,
    link_text: String,
    target_as_written: String,
    resolved_target: Option<String>,
    kind: LinkKind,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum LinkKind {
    Markdown,
    Wiki,
}

fn main() -> ExitCode {
    let args = Args::parse();
    match run(&args) {
        Ok(report) => {
            if let Err(e) = write_output(&args, &report) {
                eprintln!("error: failed to write output: {e}");
                return ExitCode::from(2);
            }
            let any_findings = !report.stale.is_empty() || !report.dead_links.is_empty();
            if args.strict && any_findings {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::from(2)
        }
    }
}

fn run(args: &Args) -> io::Result<Report> {
    if args.no_stale && args.no_dead_links {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "both --no-stale and --no-dead-links given; nothing to do",
        ));
    }
    let extension = args.extension.trim_start_matches('.').to_string();
    let mut warnings = Vec::new();

    let mut scanned: Vec<(PathBuf, SystemTime)> = Vec::new();
    for root in &args.corpus {
        match walk_corpus(root, &extension) {
            Ok(mut found) => scanned.append(&mut found),
            Err(e) => {
                warnings.push(format!(
                    "corpus[path={}]: walk failed: {e}",
                    root.display()
                ));
            }
        }
    }

    let now = SystemTime::now();

    let stale = if args.no_stale {
        Vec::new()
    } else {
        detect_stale(&scanned, args.stale_days, now)
    };

    let dead_links = if args.no_dead_links {
        Vec::new()
    } else {
        detect_dead_links(&scanned, &mut warnings)
    };

    Ok(Report {
        schema: SCHEMA.to_string(),
        corpus: args.corpus.iter().map(|p| p.display().to_string()).collect(),
        files_scanned: scanned.len(),
        stale_threshold_days: args.stale_days,
        stale_detection_enabled: !args.no_stale,
        dead_link_detection_enabled: !args.no_dead_links,
        stale,
        dead_links,
        warnings,
    })
}

fn walk_corpus(root: &Path, extension: &str) -> io::Result<Vec<(PathBuf, SystemTime)>> {
    let mut out = Vec::new();
    walk_dir(root, extension, &mut out)?;
    Ok(out)
}

fn walk_dir(dir: &Path, extension: &str, out: &mut Vec<(PathBuf, SystemTime)>) -> io::Result<()> {
    let entries = fs::read_dir(dir)?;
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            walk_dir(&path, extension, out)?;
        } else if file_type.is_file() {
            if path.extension().and_then(|e| e.to_str()) == Some(extension) {
                let metadata = entry.metadata()?;
                let mtime = metadata.modified()?;
                out.push((path, mtime));
            }
        }
    }
    Ok(())
}

fn detect_stale(
    files: &[(PathBuf, SystemTime)],
    threshold_days: u64,
    now: SystemTime,
) -> Vec<StaleEntry> {
    let mut out = Vec::new();
    for (path, mtime) in files {
        let age_days = file_age_days(*mtime, now);
        if age_days >= threshold_days {
            let last_modified_unix = mtime
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            out.push(StaleEntry {
                path: path.display().to_string(),
                age_days,
                last_modified_unix,
            });
        }
    }
    out.sort_by(|a, b| b.age_days.cmp(&a.age_days).then(a.path.cmp(&b.path)));
    out
}

fn file_age_days(mtime: SystemTime, now: SystemTime) -> u64 {
    match now.duration_since(mtime) {
        Ok(d) => d.as_secs() / SECONDS_PER_DAY,
        Err(_) => 0,
    }
}

fn detect_dead_links(
    files: &[(PathBuf, SystemTime)],
    warnings: &mut Vec<String>,
) -> Vec<DeadLinkEntry> {
    let stem_index = build_stem_index(files);
    let mut out = Vec::new();
    for (path, _) in files {
        let content = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                warnings.push(format!(
                    "read[path={}]: {e}",
                    path.display()
                ));
                continue;
            }
        };
        let mut dead = check_file_links(path, &content, &stem_index);
        out.append(&mut dead);
    }
    out.sort_by(|a, b| {
        a.source
            .cmp(&b.source)
            .then(a.line.cmp(&b.line))
            .then(a.target_as_written.cmp(&b.target_as_written))
    });
    out
}

fn build_stem_index(files: &[(PathBuf, SystemTime)]) -> HashMap<String, Vec<PathBuf>> {
    let mut idx: HashMap<String, Vec<PathBuf>> = HashMap::new();
    for (path, _) in files {
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            idx.entry(stem.to_string()).or_default().push(path.clone());
        }
    }
    idx
}

fn check_file_links(
    source: &Path,
    content: &str,
    stem_index: &HashMap<String, Vec<PathBuf>>,
) -> Vec<DeadLinkEntry> {
    let mut out = Vec::new();
    let mut in_fence = false;
    for (idx, line) in content.lines().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim_start();
        if trimmed.starts_with("```") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        for (text, target) in extract_markdown_links(line) {
            if is_external_link(&target) || is_anchor_only(&target) || target.is_empty() {
                continue;
            }
            let path_part = strip_fragment(&target);
            let resolved = resolve_relative(source, path_part);
            if !resolved.exists() {
                out.push(DeadLinkEntry {
                    source: source.display().to_string(),
                    line: line_no,
                    link_text: text,
                    target_as_written: target,
                    resolved_target: Some(resolved.display().to_string()),
                    kind: LinkKind::Markdown,
                });
            }
        }
        for target in extract_wiki_links(line) {
            if target.is_empty() {
                continue;
            }
            let stem = wiki_stem(&target);
            if !stem_index.contains_key(&stem) {
                out.push(DeadLinkEntry {
                    source: source.display().to_string(),
                    line: line_no,
                    link_text: target.clone(),
                    target_as_written: target,
                    resolved_target: None,
                    kind: LinkKind::Wiki,
                });
            }
        }
    }
    out
}

fn extract_markdown_links(line: &str) -> Vec<(String, String)> {
    let mut out = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        // Skip inline-code spans: backtick to backtick.
        if bytes[i] == b'`' {
            i += 1;
            while i < bytes.len() && bytes[i] != b'`' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            continue;
        }
        if bytes[i] == b'[' {
            // Skip wiki links here ([[…]]); they are handled separately.
            if i + 1 < bytes.len() && bytes[i + 1] == b'[' {
                i += 2;
                continue;
            }
            if let Some(close_bracket) = find_balanced(bytes, i + 1, b'[', b']') {
                if close_bracket + 1 < bytes.len() && bytes[close_bracket + 1] == b'(' {
                    if let Some(close_paren) =
                        find_balanced(bytes, close_bracket + 2, b'(', b')')
                    {
                        let text = String::from_utf8_lossy(
                            &bytes[i + 1..close_bracket],
                        )
                        .into_owned();
                        let target = String::from_utf8_lossy(
                            &bytes[close_bracket + 2..close_paren],
                        )
                        .trim()
                        .to_string();
                        out.push((text, target));
                        i = close_paren + 1;
                        continue;
                    }
                }
            }
        }
        i += 1;
    }
    out
}

fn extract_wiki_links(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let bytes = line.as_bytes();
    let mut i = 0;
    while i + 3 < bytes.len() {
        if bytes[i] == b'`' {
            i += 1;
            while i < bytes.len() && bytes[i] != b'`' {
                i += 1;
            }
            if i < bytes.len() {
                i += 1;
            }
            continue;
        }
        if bytes[i] == b'[' && bytes[i + 1] == b'[' {
            if let Some(close) = find_double_close(bytes, i + 2) {
                let target = String::from_utf8_lossy(&bytes[i + 2..close])
                    .trim()
                    .to_string();
                out.push(target);
                i = close + 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn find_balanced(bytes: &[u8], start: usize, open: u8, close: u8) -> Option<usize> {
    let mut depth = 1;
    let mut i = start;
    while i < bytes.len() {
        if bytes[i] == open {
            depth += 1;
        } else if bytes[i] == close {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn find_double_close(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    while i + 1 < bytes.len() {
        if bytes[i] == b']' && bytes[i + 1] == b']' {
            return Some(i);
        }
        i += 1;
    }
    None
}

fn is_external_link(target: &str) -> bool {
    let lower = target.to_ascii_lowercase();
    [
        "http://", "https://", "ftp://", "mailto:", "tel:", "file://", "data:", "javascript:",
    ]
    .iter()
    .any(|p| lower.starts_with(p))
}

fn is_anchor_only(target: &str) -> bool {
    target.starts_with('#')
}

fn strip_fragment(target: &str) -> &str {
    match target.find('#') {
        Some(idx) => &target[..idx],
        None => target,
    }
}

fn resolve_relative(source: &Path, target: &str) -> PathBuf {
    let parent = source.parent().unwrap_or_else(|| Path::new("."));
    parent.join(target)
}

fn wiki_stem(target: &str) -> String {
    // [[name]] or [[name|alias]] — use the name part as stem.
    let name = match target.find('|') {
        Some(idx) => &target[..idx],
        None => target,
    };
    name.trim().to_string()
}

fn write_output(args: &Args, report: &Report) -> io::Result<()> {
    let body = match args.format {
        OutputFormat::Json => serde_json::to_string_pretty(report)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?,
        OutputFormat::Text => format_text(report),
    };
    if args.output == "-" {
        let stdout = io::stdout();
        let mut handle = stdout.lock();
        handle.write_all(body.as_bytes())?;
        handle.write_all(b"\n")?;
    } else {
        fs::write(&args.output, body + "\n")?;
    }
    Ok(())
}

fn format_text(report: &Report) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "v2-gardening-sweep: {} files scanned across {} root(s) (stale_threshold={}d, stale={}, dead_links={})\n",
        report.files_scanned,
        report.corpus.len(),
        report.stale_threshold_days,
        if report.stale_detection_enabled { "on" } else { "off" },
        if report.dead_link_detection_enabled { "on" } else { "off" },
    ));
    if report.stale_detection_enabled {
        out.push_str(&format!("\n## Stale ({} entries)\n", report.stale.len()));
        for entry in &report.stale {
            out.push_str(&format!(
                "  {} — {}d old\n",
                entry.path, entry.age_days
            ));
        }
    }
    if report.dead_link_detection_enabled {
        out.push_str(&format!(
            "\n## Dead links ({} entries)\n",
            report.dead_links.len()
        ));
        for entry in &report.dead_links {
            let kind = match entry.kind {
                LinkKind::Markdown => "md",
                LinkKind::Wiki => "wiki",
            };
            out.push_str(&format!(
                "  [{kind}] {}:{} -> {}\n",
                entry.source, entry.line, entry.target_as_written
            ));
        }
    }
    if !report.warnings.is_empty() {
        out.push_str(&format!("\n## Warnings ({})\n", report.warnings.len()));
        for w in &report.warnings {
            out.push_str(&format!("  {w}\n"));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn file_age_days_returns_zero_for_future_mtime() {
        let now = SystemTime::now();
        let mtime = now + Duration::from_secs(SECONDS_PER_DAY);
        assert_eq!(file_age_days(mtime, now), 0);
    }

    #[test]
    fn file_age_days_handles_30_day_offset() {
        let now = SystemTime::now();
        let mtime = now - Duration::from_secs(SECONDS_PER_DAY * 30 + 60);
        assert_eq!(file_age_days(mtime, now), 30);
    }

    #[test]
    fn extract_markdown_links_handles_simple_link() {
        let links = extract_markdown_links("see [docs](docs/redesign/README.md) for details");
        assert_eq!(links, vec![("docs".into(), "docs/redesign/README.md".into())]);
    }

    #[test]
    fn extract_markdown_links_handles_multiple_per_line() {
        let links =
            extract_markdown_links("[a](x.md) and [b](y.md) and [c](z.md)");
        assert_eq!(
            links,
            vec![
                ("a".into(), "x.md".into()),
                ("b".into(), "y.md".into()),
                ("c".into(), "z.md".into()),
            ]
        );
    }

    #[test]
    fn extract_markdown_links_ignores_wiki_brackets() {
        let links = extract_markdown_links("see [[wiki-name]] for the index");
        assert!(links.is_empty());
    }

    #[test]
    fn extract_markdown_links_ignores_inline_code_spans() {
        let links = extract_markdown_links("`[fake](url)` plus [real](real.md)");
        assert_eq!(links, vec![("real".into(), "real.md".into())]);
    }

    #[test]
    fn extract_wiki_links_handles_simple_wiki() {
        let links = extract_wiki_links("link to [[name-here]] in body");
        assert_eq!(links, vec!["name-here".to_string()]);
    }

    #[test]
    fn extract_wiki_links_handles_alias_form() {
        let links = extract_wiki_links("see [[name|alias text]]");
        assert_eq!(links, vec!["name|alias text".to_string()]);
    }

    #[test]
    fn is_external_link_recognizes_http_https_mailto() {
        assert!(is_external_link("http://example.com"));
        assert!(is_external_link("https://example.com/path"));
        assert!(is_external_link("mailto:foo@bar.com"));
        assert!(is_external_link("HTTPS://EXAMPLE.com"));
        assert!(!is_external_link("docs/path.md"));
        assert!(!is_external_link("./relative.md"));
    }

    #[test]
    fn is_anchor_only_recognizes_fragment() {
        assert!(is_anchor_only("#section"));
        assert!(!is_anchor_only("path.md#section"));
        assert!(!is_anchor_only(""));
    }

    #[test]
    fn strip_fragment_removes_anchor() {
        assert_eq!(strip_fragment("path.md#sec"), "path.md");
        assert_eq!(strip_fragment("path.md"), "path.md");
        assert_eq!(strip_fragment("#sec"), "");
    }

    #[test]
    fn wiki_stem_strips_alias() {
        assert_eq!(wiki_stem("name|alias"), "name");
        assert_eq!(wiki_stem("name"), "name");
        assert_eq!(wiki_stem("  spaced  "), "spaced");
    }
}
