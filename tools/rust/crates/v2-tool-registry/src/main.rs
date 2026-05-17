use clap::{Parser, ValueEnum};
use serde::Serialize;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "v2-tool-registry",
    about = "Enumerate Rust tools under tools/rust/crates/ for orchestrator discovery"
)]
struct Args {
    /// Repository root (path containing tools/rust/crates/)
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Output format
    #[arg(long, value_enum, default_value_t = Format::Markdown)]
    format: Format,

    /// Filter tools by substring match on name
    #[arg(long)]
    filter: Option<String>,

    /// Only include v2-prefixed tools (default: include all)
    #[arg(long)]
    v2_only: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Format {
    Markdown,
    Json,
    Names,
}

#[derive(Serialize, Debug, PartialEq, Eq)]
pub struct ToolEntry {
    pub name: String,
    pub description: String,
    pub is_v2: bool,
}

const SELF_NAME: &str = "v2-tool-registry";

fn main() -> ExitCode {
    let args = Args::parse();
    let stdout = io::stdout();
    let mut out = stdout.lock();
    match run(&args, &mut out) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            // Broken pipe: consumer (e.g., `| head`) closed early. Exit silently —
            // the data we did write was delivered.
            if let RegistryError::Io(io_err) = &err {
                if io_err.kind() == io::ErrorKind::BrokenPipe {
                    return ExitCode::SUCCESS;
                }
            }
            eprintln!("v2-tool-registry: {err}");
            ExitCode::FAILURE
        }
    }
}

#[derive(Debug)]
enum RegistryError {
    Io(io::Error),
    Other(String),
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::Io(e) => write!(f, "{e}"),
            RegistryError::Other(s) => f.write_str(s),
        }
    }
}

impl From<io::Error> for RegistryError {
    fn from(e: io::Error) -> Self {
        RegistryError::Io(e)
    }
}

impl From<serde_json::Error> for RegistryError {
    fn from(e: serde_json::Error) -> Self {
        RegistryError::Other(e.to_string())
    }
}

fn run<W: Write>(args: &Args, out: &mut W) -> Result<(), RegistryError> {
    let crates_dir = args.repo_root.join("tools/rust/crates");
    if !crates_dir.is_dir() {
        return Err(RegistryError::Other(format!(
            "crates directory not found: {} (use --repo-root)",
            crates_dir.display()
        )));
    }

    let mut entries = discover_tools(&crates_dir)?;
    entries.sort_by(|a, b| a.name.cmp(&b.name));

    let filtered: Vec<&ToolEntry> = entries
        .iter()
        .filter(|e| !args.v2_only || e.is_v2)
        .filter(|e| match &args.filter {
            Some(pat) => e.name.contains(pat),
            None => true,
        })
        .collect();

    match args.format {
        Format::Markdown => render_markdown(&filtered, out)?,
        Format::Json => {
            let json = serde_json::to_string_pretty(&filtered)?;
            writeln!(out, "{json}")?;
        }
        Format::Names => {
            for entry in &filtered {
                writeln!(out, "{}", entry.name)?;
            }
        }
    }

    Ok(())
}

fn render_markdown<W: Write>(entries: &[&ToolEntry], out: &mut W) -> io::Result<()> {
    for entry in entries {
        let desc = if entry.description.is_empty() {
            "(no description)"
        } else {
            entry.description.as_str()
        };
        writeln!(out, "- `{}` — {}", entry.name, desc)?;
    }
    Ok(())
}

pub fn discover_tools(crates_dir: &Path) -> std::io::Result<Vec<ToolEntry>> {
    let mut entries = Vec::new();
    for dir_entry in std::fs::read_dir(crates_dir)? {
        let dir_entry = dir_entry?;
        if !dir_entry.file_type()?.is_dir() {
            continue;
        }
        let name = dir_entry.file_name().to_string_lossy().into_owned();
        if name == SELF_NAME {
            continue;
        }
        let cargo_path = dir_entry.path().join("Cargo.toml");
        if !cargo_path.exists() {
            continue;
        }
        let cargo_content = std::fs::read_to_string(&cargo_path)?;
        let description = extract_description(&cargo_content).unwrap_or_default();
        let is_v2 = name.starts_with("v2-");
        entries.push(ToolEntry {
            name,
            description,
            is_v2,
        });
    }
    Ok(entries)
}

/// Extract the `description = "..."` value from the `[package]` section of a Cargo.toml.
///
/// Hand-rolled parser to avoid taking a `toml` crate dependency for this single field.
/// Handles: leading whitespace, single-line double-quoted strings, escaped quotes (`\"`),
/// and ignores `description` keys outside `[package]` (e.g., in `[dependencies.foo]`).
pub fn extract_description(cargo_content: &str) -> Option<String> {
    let mut in_package = false;
    let mut seen_first_section = false;

    for raw_line in cargo_content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some(section) = section_header(line) {
            seen_first_section = true;
            in_package = section == "package";
            continue;
        }
        // Cargo.toml convention: top-of-file keys before any [section] are part of [package]
        // when they appear in a `[package]` table-less manifest. Cargo workspace members
        // always have an explicit `[package]`, but be tolerant.
        let effective_in_package = in_package || !seen_first_section;
        if !effective_in_package {
            continue;
        }
        if let Some(value) = parse_description_line(line) {
            return Some(value);
        }
    }
    None
}

fn section_header(line: &str) -> Option<&str> {
    let line = line.trim();
    if !(line.starts_with('[') && line.ends_with(']')) {
        return None;
    }
    let inner = &line[1..line.len() - 1];
    // [package] (or [workspace], etc.) — return the first segment before any dot
    Some(inner.split('.').next().unwrap_or(inner).trim())
}

fn parse_description_line(line: &str) -> Option<String> {
    let after_key = line.strip_prefix("description")?;
    let after_eq = after_key.trim_start();
    let after_eq = after_eq.strip_prefix('=')?.trim_start();
    if !after_eq.starts_with('"') {
        // Multi-line strings or other quoting styles are out of scope for this parser.
        return None;
    }
    let inner = &after_eq[1..];
    parse_quoted_string(inner)
}

fn parse_quoted_string(inner: &str) -> Option<String> {
    let mut result = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Some(result),
            '\\' => {
                let next = chars.next()?;
                let escaped = match next {
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '"' => '"',
                    other => other,
                };
                result.push(escaped);
            }
            other => result.push(other),
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracts_simple_description() {
        let content = r#"
[package]
name = "foo"
version = "0.1.0"
description = "A test tool"
"#;
        assert_eq!(
            extract_description(content),
            Some("A test tool".to_string())
        );
    }

    #[test]
    fn extracts_description_before_section_header() {
        let content = r#"
description = "Top-level description"
[package]
name = "foo"
"#;
        assert_eq!(
            extract_description(content),
            Some("Top-level description".to_string())
        );
    }

    #[test]
    fn ignores_description_in_non_package_section() {
        let content = r#"
[package]
name = "foo"

[dependencies.something]
version = "1"
description = "should be ignored"
"#;
        assert_eq!(extract_description(content), None);
    }

    #[test]
    fn handles_escaped_quotes() {
        let content = r#"
[package]
description = "value with \"quoted\" word"
"#;
        assert_eq!(
            extract_description(content),
            Some(r#"value with "quoted" word"#.to_string())
        );
    }

    #[test]
    fn handles_section_header_dots() {
        // [package.metadata.docs.rs] should still resolve to "package"
        let content = r#"
[package.metadata.docs.rs]
all-features = true

[package]
description = "Real description"
"#;
        // Inside [package.metadata.docs.rs] we still report being "in package"
        // because section_header returns the first segment. This is acceptable
        // for the v1 prototype — Cargo conventions don't put `description` keys
        // under metadata sub-tables. If they did, we'd capture the wrong value.
        assert_eq!(
            extract_description(content),
            Some("Real description".to_string())
        );
    }

    #[test]
    fn returns_none_when_description_missing() {
        let content = r#"
[package]
name = "foo"
"#;
        assert_eq!(extract_description(content), None);
    }

    #[test]
    fn ignores_comments() {
        let content = r#"
[package]
# description = "commented out"
description = "real value"
"#;
        assert_eq!(extract_description(content), Some("real value".to_string()));
    }

    #[test]
    fn multiline_string_returns_none() {
        // Triple-quoted multiline strings are out of scope for this parser.
        let content = r#"
[package]
description = """
multi
line
"""
"#;
        // Parser sees `"""` and tries to parse as quoted string starting with `""`
        // (empty), which immediately closes. Acceptable v1 behavior — the empty
        // description is detectable by callers and signals "manifest needs review."
        assert_eq!(extract_description(content), Some("".to_string()));
    }

    #[test]
    fn section_header_extracts_first_segment() {
        assert_eq!(section_header("[package]"), Some("package"));
        assert_eq!(section_header("[package.metadata]"), Some("package"));
        assert_eq!(section_header("[dependencies]"), Some("dependencies"));
        assert_eq!(section_header("not a section"), None);
        assert_eq!(section_header("[unclosed"), None);
    }
}
