use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

const SCHEMA: &str = "v2-gardening-sweep/v1";
const HASH_STATE_SCHEMA: &str = "v2-gardening-sweep-hash-state/v1";
const DEFAULT_STALE_DAYS: u64 = 30;
const DEFAULT_EXTENSION: &str = "md";
const SECONDS_PER_DAY: u64 = 86_400;
const SUGGEST_MAX_EDIT_DISTANCE: usize = 4;

#[derive(Parser, Debug)]
#[command(
    name = "v2-gardening-sweep",
    about = "Stale-detection + dead-link-detection deslop pass over markdown corpora"
)]
struct Args {
    /// Corpus directory to sweep. May be repeated to sweep multiple roots
    /// (e.g. --corpus docs/redesign/_notes --corpus docs/journal).
    /// May also be set via --config file.
    #[arg(long)]
    corpus: Vec<PathBuf>,

    /// Stale threshold in days. Files with mtime older than this are flagged.
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

    /// Glob patterns to exclude paths (relative to corpus root). Repeatable.
    /// Supports `**` (any subpath including slashes) and `*` (any chars except slash).
    #[arg(long)]
    exclude: Vec<String>,

    /// Path to a JSON config file. CLI args override config defaults except
    /// for excludes and corpus which UNION across CLI + config.
    #[arg(long)]
    config: Option<PathBuf>,

    /// Path to a hash-state file. When present, stale entries gain a
    /// `hash_unchanged` field distinguishing "stale by neglect" (content matches
    /// previous sweep) from "stale despite recent edits" (content differs).
    /// The state file is updated after each run.
    #[arg(long)]
    state_file: Option<PathBuf>,

    /// Detector composition: run dead-link detection only on files flagged stale.
    /// Useful for "what is broken in our long-quiet docs" reporting.
    #[arg(long)]
    dead_links_only_on_stale_files: bool,

    /// Disable suggested-target lookup for dead links (default: enabled).
    /// Suggestions search the stem-index for near-matches by edit distance.
    #[arg(long)]
    no_suggest_fixes: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    Json,
    Text,
}

#[derive(Debug, Deserialize, Default)]
struct ConfigFile {
    #[serde(default)]
    corpus: Vec<PathBuf>,
    stale_days: Option<u64>,
    extension: Option<String>,
    #[serde(default)]
    exclude: Vec<String>,
    no_stale: Option<bool>,
    no_dead_links: Option<bool>,
    state_file: Option<PathBuf>,
    no_suggest_fixes: Option<bool>,
}

fn load_config(path: &Path) -> io::Result<ConfigFile> {
    let s = fs::read_to_string(path)?;
    serde_json::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

#[derive(Debug, Serialize, Deserialize)]
struct HashState {
    schema: String,
    #[serde(default)]
    entries: BTreeMap<String, u64>,
}

impl Default for HashState {
    fn default() -> Self {
        Self {
            schema: HASH_STATE_SCHEMA.to_string(),
            entries: BTreeMap::new(),
        }
    }
}

fn load_hash_state(path: &Path) -> io::Result<HashState> {
    if !path.exists() {
        return Ok(HashState::default());
    }
    let s = fs::read_to_string(path)?;
    serde_json::from_str(&s).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

fn save_hash_state(path: &Path, state: &HashState) -> io::Result<()> {
    let body = serde_json::to_string_pretty(state)
        .map_err(io::Error::other)?;
    fs::write(path, body + "\n")
}

const FNV_OFFSET_BASIS: u64 = 14_695_981_039_346_656_037;
const FNV_PRIME: u64 = 1_099_511_628_211;

fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET_BASIS;
    for b in bytes {
        h ^= *b as u64;
        h = h.wrapping_mul(FNV_PRIME);
    }
    h
}

#[derive(Debug, Serialize, Deserialize)]
struct Report {
    schema: String,
    corpus: Vec<String>,
    files_scanned: usize,
    files_excluded: usize,
    stale_threshold_days: u64,
    stale_detection_enabled: bool,
    dead_link_detection_enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    state_file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    composition: Option<String>,
    stale: Vec<StaleEntry>,
    dead_links: Vec<DeadLinkEntry>,
    warnings: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct StaleEntry {
    path: String,
    age_days: u64,
    last_modified_unix: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    hash_unchanged: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
struct DeadLinkEntry {
    source: String,
    line: usize,
    link_text: String,
    target_as_written: String,
    resolved_target: Option<String>,
    kind: LinkKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_target: Option<String>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum LinkKind {
    Markdown,
    Wiki,
    MarkdownRef,
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
    let config = match &args.config {
        Some(p) => load_config(p)?,
        None => ConfigFile::default(),
    };

    // Merge: CLI corpus wins if non-empty; else config corpus.
    let corpus: Vec<PathBuf> = if args.corpus.is_empty() {
        config.corpus.clone()
    } else {
        args.corpus.clone()
    };
    if corpus.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no corpus specified (provide --corpus or set corpus in --config file)",
        ));
    }

    // CLI overrides config defaults; when CLI matches the default-value sentinel
    // and config provides a value, use config.
    let stale_days = if args.stale_days == DEFAULT_STALE_DAYS {
        config.stale_days.unwrap_or(args.stale_days)
    } else {
        args.stale_days
    };
    let extension = if args.extension == DEFAULT_EXTENSION {
        config
            .extension
            .clone()
            .unwrap_or_else(|| args.extension.clone())
    } else {
        args.extension.clone()
    };
    let extension = extension.trim_start_matches('.').to_string();

    let no_stale = args.no_stale || config.no_stale.unwrap_or(false);
    let no_dead_links = args.no_dead_links || config.no_dead_links.unwrap_or(false);
    let suggest_fixes = !(args.no_suggest_fixes || config.no_suggest_fixes.unwrap_or(false));

    // Excludes UNION across CLI + config.
    let mut excludes: Vec<String> = args.exclude.clone();
    excludes.extend(config.exclude.iter().cloned());

    let state_file = args
        .state_file
        .clone()
        .or_else(|| config.state_file.clone());

    if no_stale && no_dead_links {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "both --no-stale and --no-dead-links given; nothing to do",
        ));
    }

    let mut warnings = Vec::new();
    let mut files_excluded: usize = 0;

    let mut scanned: Vec<(PathBuf, SystemTime)> = Vec::new();
    for root in &corpus {
        match walk_corpus(root, &extension) {
            Ok(found) => {
                for (p, m) in found {
                    let rel = p.strip_prefix(root).unwrap_or(&p);
                    if should_exclude(rel, &excludes) {
                        files_excluded += 1;
                    } else {
                        scanned.push((p, m));
                    }
                }
            }
            Err(e) => {
                warnings.push(format!(
                    "corpus[path={}]: walk failed: {e}",
                    root.display()
                ));
            }
        }
    }
    scanned.sort_by(|a, b| a.0.cmp(&b.0));

    let now = SystemTime::now();

    let mut hash_state = match &state_file {
        Some(p) => load_hash_state(p)?,
        None => HashState::default(),
    };

    let mut current_hashes: HashMap<String, u64> = HashMap::new();
    if state_file.is_some() {
        for (path, _) in &scanned {
            match fs::read(path) {
                Ok(bytes) => {
                    current_hashes.insert(path.display().to_string(), fnv1a_64(&bytes));
                }
                Err(e) => {
                    warnings.push(format!(
                        "hash[path={}]: read failed: {e}",
                        path.display()
                    ));
                }
            }
        }
    }

    let stale = if no_stale {
        Vec::new()
    } else {
        detect_stale(&scanned, stale_days, now, &current_hashes, &hash_state, state_file.is_some())
    };

    let composition = if args.dead_links_only_on_stale_files {
        Some("dead-links-only-on-stale-files".to_string())
    } else {
        None
    };

    let files_for_dead_link_detection: Vec<(PathBuf, SystemTime)> =
        if args.dead_links_only_on_stale_files {
            let stale_paths: HashSet<String> = stale.iter().map(|e| e.path.clone()).collect();
            scanned
                .iter()
                .filter(|(p, _)| stale_paths.contains(&p.display().to_string()))
                .cloned()
                .collect()
        } else {
            scanned.clone()
        };

    let dead_links = if no_dead_links {
        Vec::new()
    } else {
        detect_dead_links(
            &scanned,
            &files_for_dead_link_detection,
            suggest_fixes,
            &mut warnings,
        )
    };

    if let Some(ref p) = state_file {
        for (path_str, h) in &current_hashes {
            hash_state.entries.insert(path_str.clone(), *h);
        }
        if hash_state.schema.is_empty() {
            hash_state.schema = HASH_STATE_SCHEMA.into();
        }
        if let Err(e) = save_hash_state(p, &hash_state) {
            warnings.push(format!(
                "state_file[path={}]: write failed: {e}",
                p.display()
            ));
        }
    }

    Ok(Report {
        schema: SCHEMA.to_string(),
        corpus: corpus.iter().map(|p| p.display().to_string()).collect(),
        files_scanned: scanned.len(),
        files_excluded,
        stale_threshold_days: stale_days,
        stale_detection_enabled: !no_stale,
        dead_link_detection_enabled: !no_dead_links,
        state_file: state_file.map(|p| p.display().to_string()),
        composition,
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
        } else if file_type.is_file()
            && path.extension().and_then(|e| e.to_str()) == Some(extension)
        {
            let metadata = entry.metadata()?;
            let mtime = metadata.modified()?;
            out.push((path, mtime));
        }
    }
    Ok(())
}

fn detect_stale(
    files: &[(PathBuf, SystemTime)],
    threshold_days: u64,
    now: SystemTime,
    current_hashes: &HashMap<String, u64>,
    hash_state: &HashState,
    state_file_in_use: bool,
) -> Vec<StaleEntry> {
    let mut out = Vec::new();
    for (path, mtime) in files {
        let age_days = file_age_days(*mtime, now);
        if age_days >= threshold_days {
            let last_modified_unix = mtime
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            let path_str = path.display().to_string();
            let hash_unchanged = if state_file_in_use {
                match (current_hashes.get(&path_str), hash_state.entries.get(&path_str)) {
                    (Some(c), Some(p)) => Some(c == p),
                    _ => None,
                }
            } else {
                None
            };
            out.push(StaleEntry {
                path: path_str,
                age_days,
                last_modified_unix,
                hash_unchanged,
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
    all_files: &[(PathBuf, SystemTime)],
    files_to_check: &[(PathBuf, SystemTime)],
    suggest_fixes: bool,
    warnings: &mut Vec<String>,
) -> Vec<DeadLinkEntry> {
    let stem_index = build_stem_index(all_files);
    let mut out = Vec::new();
    for (path, _) in files_to_check {
        let content = match fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                warnings.push(format!("read[path={}]: {e}", path.display()));
                continue;
            }
        };
        let mut dead = check_file_links(path, &content, &stem_index, suggest_fixes);
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
    suggest_fixes: bool,
) -> Vec<DeadLinkEntry> {
    let mask = build_line_mask(content);
    let ref_defs = parse_ref_defs(content, &mask);
    let mut out = Vec::new();

    let links = extract_markdown_links_multiline(content, &mask, &ref_defs);
    for link in links {
        let target_str = link.target.clone();
        if is_external_link(&target_str) || is_anchor_only(&target_str) || target_str.is_empty() {
            continue;
        }
        if target_str.starts_with("[undefined-ref:") {
            // Reference resolved to undefined definition
            out.push(DeadLinkEntry {
                source: source.display().to_string(),
                line: link.line,
                link_text: link.text,
                target_as_written: target_str,
                resolved_target: None,
                kind: link.kind,
                suggested_target: None,
            });
            continue;
        }
        let path_part = strip_fragment(&target_str);
        let resolved = resolve_relative(source, path_part);
        if !resolved.exists() {
            let suggested = if suggest_fixes {
                suggest_target(path_part, stem_index, source)
            } else {
                None
            };
            out.push(DeadLinkEntry {
                source: source.display().to_string(),
                line: link.line,
                link_text: link.text,
                target_as_written: target_str,
                resolved_target: Some(resolved.display().to_string()),
                kind: link.kind,
                suggested_target: suggested,
            });
        }
    }

    let wikis = extract_wiki_links_multiline(content, &mask);
    for (line_no, target) in wikis {
        if target.is_empty() {
            continue;
        }
        let stem = wiki_stem(&target);
        if !stem_index.contains_key(&stem) {
            let suggested = if suggest_fixes {
                suggest_wiki_stem(&stem, stem_index)
            } else {
                None
            };
            out.push(DeadLinkEntry {
                source: source.display().to_string(),
                line: line_no,
                link_text: target.clone(),
                target_as_written: target,
                resolved_target: None,
                kind: LinkKind::Wiki,
                suggested_target: suggested,
            });
        }
    }

    out
}

/// Per-line skip mask for content preprocessing.
/// Lines marked false should NOT have links extracted from them.
struct LineMask {
    process: Vec<bool>,
}

fn build_line_mask(content: &str) -> LineMask {
    let lines: Vec<&str> = content.lines().collect();
    let mut process = vec![true; lines.len()];
    let mut in_frontmatter = false;
    let mut frontmatter_done = false;
    let mut in_fence = false;
    let mut fence_marker: Option<&'static str> = None;
    let mut in_html_comment = false;

    for (i, line) in lines.iter().enumerate() {
        // Frontmatter only at the very top.
        if !frontmatter_done && i == 0 && line.trim() == "---" {
            in_frontmatter = true;
            process[i] = false;
            continue;
        }
        if in_frontmatter {
            process[i] = false;
            let trimmed = line.trim();
            if trimmed == "---" || trimmed == "..." {
                in_frontmatter = false;
                frontmatter_done = true;
            }
            continue;
        }
        frontmatter_done = true;

        // Fenced code blocks (``` or ~~~).
        let trimmed = line.trim_start();
        if !in_fence {
            if trimmed.starts_with("```") {
                in_fence = true;
                fence_marker = Some("```");
                process[i] = false;
                continue;
            }
            if trimmed.starts_with("~~~") {
                in_fence = true;
                fence_marker = Some("~~~");
                process[i] = false;
                continue;
            }
        } else {
            process[i] = false;
            if let Some(marker) = fence_marker {
                if trimmed.starts_with(marker) {
                    in_fence = false;
                    fence_marker = None;
                }
            }
            continue;
        }

        // Multi-line HTML comments.
        if in_html_comment {
            process[i] = false;
            if line.contains("-->") {
                in_html_comment = false;
            }
            continue;
        }
        if line.trim_start().starts_with("<!--") && !line.contains("-->") {
            in_html_comment = true;
            process[i] = false;
            continue;
        }
        if line.trim_start().starts_with("<!--") && line.trim_end().ends_with("-->") {
            // Single-line full comment.
            process[i] = false;
            continue;
        }

        // Indented code blocks (4+ leading spaces on a non-blank line).
        let leading_spaces = line.chars().take_while(|c| *c == ' ').count();
        if leading_spaces >= 4 && !line.trim().is_empty() {
            process[i] = false;
            continue;
        }
    }

    LineMask { process }
}

/// Parse `[ref]: url` reference definitions. Returns a case-insensitive
/// map from ref-name to URL.
fn parse_ref_defs(content: &str, mask: &LineMask) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for (i, line) in content.lines().enumerate() {
        if i < mask.process.len() && !mask.process[i] {
            continue;
        }
        let trimmed = line.trim_start();
        if !trimmed.starts_with('[') {
            continue;
        }
        if let Some(close_idx) = trimmed[1..].find(']') {
            let name = &trimmed[1..1 + close_idx];
            let after = &trimmed[1 + close_idx + 1..];
            if !after.starts_with(':') {
                continue;
            }
            let value = after[1..].trim();
            let target = strip_ref_title(value);
            if !target.is_empty() && !name.is_empty() {
                out.insert(name.to_lowercase(), target.to_string());
            }
        }
    }
    out
}

fn strip_ref_title(value: &str) -> &str {
    match value.find(char::is_whitespace) {
        Some(idx) => &value[..idx],
        None => value,
    }
}

struct ExtractedLink {
    line: usize,
    text: String,
    target: String,
    kind: LinkKind,
}

/// Byte-offset → line-number lookup table for multi-line link reporting.
fn build_line_offsets(content: &str) -> Vec<usize> {
    let mut offsets = vec![0usize];
    let bytes = content.as_bytes();
    for (i, b) in bytes.iter().enumerate() {
        if *b == b'\n' {
            offsets.push(i + 1);
        }
    }
    offsets
}

fn lookup_line(line_offsets: &[usize], byte_pos: usize) -> usize {
    // Returns 0-indexed line number containing byte_pos.
    match line_offsets.binary_search(&byte_pos) {
        Ok(idx) => idx,
        Err(idx) => idx.saturating_sub(1),
    }
}

/// Extract markdown links across the full content, honoring the line mask
/// and supporting multi-line links + reference-style links.
fn extract_markdown_links_multiline(
    content: &str,
    mask: &LineMask,
    ref_defs: &HashMap<String, String>,
) -> Vec<ExtractedLink> {
    let mut out = Vec::new();
    let bytes = content.as_bytes();
    let line_offsets = build_line_offsets(content);
    let n = bytes.len();
    let mut i = 0;
    let mut in_code_span = false;

    while i < n {
        let line_idx = lookup_line(&line_offsets, i);

        // Skip masked lines wholesale.
        if line_idx < mask.process.len() && !mask.process[line_idx] {
            i = if line_idx + 1 < line_offsets.len() {
                line_offsets[line_idx + 1]
            } else {
                n
            };
            in_code_span = false;
            continue;
        }

        let b = bytes[i];
        if b == b'\n' {
            // Inline code spans don't survive newlines in our simplified parser.
            in_code_span = false;
            i += 1;
            continue;
        }
        if b == b'`' {
            in_code_span = !in_code_span;
            i += 1;
            continue;
        }
        if in_code_span {
            i += 1;
            continue;
        }

        if b == b'[' {
            if i + 1 < n && bytes[i + 1] == b'[' {
                // Wiki link prefix — leave for the wiki extractor.
                i += 2;
                continue;
            }
            if let Some(close) = find_balanced_multiline(bytes, i + 1, b'[', b']') {
                if close + 1 < n {
                    let after = bytes[close + 1];
                    if after == b'(' {
                        if let Some(close_paren) =
                            find_balanced_multiline(bytes, close + 2, b'(', b')')
                        {
                            let text = String::from_utf8_lossy(&bytes[i + 1..close])
                                .into_owned()
                                .replace('\n', " ");
                            let target = String::from_utf8_lossy(&bytes[close + 2..close_paren])
                                .trim()
                                .replace('\n', "")
                                .to_string();
                            out.push(ExtractedLink {
                                line: line_idx + 1,
                                text,
                                target,
                                kind: LinkKind::Markdown,
                            });
                            i = close_paren + 1;
                            continue;
                        }
                    } else if after == b'[' {
                        if let Some(close_ref) =
                            find_balanced_multiline(bytes, close + 2, b'[', b']')
                        {
                            let text = String::from_utf8_lossy(&bytes[i + 1..close])
                                .into_owned()
                                .replace('\n', " ");
                            let mut ref_label =
                                String::from_utf8_lossy(&bytes[close + 2..close_ref])
                                    .trim()
                                    .to_string();
                            if ref_label.is_empty() {
                                ref_label = text.clone();
                            }
                            let target = match ref_defs.get(&ref_label.to_lowercase()) {
                                Some(t) => t.clone(),
                                None => format!("[undefined-ref: {ref_label}]"),
                            };
                            out.push(ExtractedLink {
                                line: line_idx + 1,
                                text,
                                target,
                                kind: LinkKind::MarkdownRef,
                            });
                            i = close_ref + 1;
                            continue;
                        }
                    } else if after != b':' {
                        // Shortcut reference [text] — resolves ONLY if text is a known ref.
                        // Skip if the char after ']' is ':' (this is a ref-def line, not a link).
                        let text = String::from_utf8_lossy(&bytes[i + 1..close])
                            .into_owned()
                            .replace('\n', " ");
                        if let Some(target) = ref_defs.get(&text.to_lowercase()) {
                            out.push(ExtractedLink {
                                line: line_idx + 1,
                                text: text.clone(),
                                target: target.clone(),
                                kind: LinkKind::MarkdownRef,
                            });
                            i = close + 1;
                            continue;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    out
}

fn extract_wiki_links_multiline(content: &str, mask: &LineMask) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    let bytes = content.as_bytes();
    let line_offsets = build_line_offsets(content);
    let n = bytes.len();
    let mut i = 0;
    let mut in_code_span = false;

    while i + 3 < n {
        let line_idx = lookup_line(&line_offsets, i);
        if line_idx < mask.process.len() && !mask.process[line_idx] {
            i = if line_idx + 1 < line_offsets.len() {
                line_offsets[line_idx + 1]
            } else {
                n
            };
            in_code_span = false;
            continue;
        }

        let b = bytes[i];
        if b == b'\n' {
            in_code_span = false;
            i += 1;
            continue;
        }
        if b == b'`' {
            in_code_span = !in_code_span;
            i += 1;
            continue;
        }
        if in_code_span {
            i += 1;
            continue;
        }

        if b == b'[' && bytes[i + 1] == b'[' {
            if let Some(close) = find_double_close(bytes, i + 2) {
                let target = String::from_utf8_lossy(&bytes[i + 2..close])
                    .trim()
                    .to_string();
                out.push((line_idx + 1, target));
                i = close + 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn find_balanced_multiline(bytes: &[u8], start: usize, open: u8, close: u8) -> Option<usize> {
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
    let name = match target.find('|') {
        Some(idx) => &target[..idx],
        None => target,
    };
    name.trim().to_string()
}

/// Find a near-match path in the stem index for a dead markdown link.
/// Returns the best candidate's display string if edit distance is within
/// SUGGEST_MAX_EDIT_DISTANCE.
fn suggest_target(
    target: &str,
    stem_index: &HashMap<String, Vec<PathBuf>>,
    source: &Path,
) -> Option<String> {
    let want_stem = Path::new(target)
        .file_stem()
        .and_then(|s| s.to_str())?
        .to_string();
    let mut best: Option<(usize, &Path)> = None;
    for (stem, candidates) in stem_index {
        let d = levenshtein(&want_stem, stem);
        if d > SUGGEST_MAX_EDIT_DISTANCE {
            continue;
        }
        for cand in candidates {
            if let Some((bd, _)) = best {
                if d < bd {
                    best = Some((d, cand.as_path()));
                }
            } else {
                best = Some((d, cand.as_path()));
            }
        }
    }
    best.map(|(_, p)| {
        let parent = source.parent().unwrap_or_else(|| Path::new("."));
        // Try to express as relative path from the source's parent.
        match pathdiff(p, parent) {
            Some(rel) => rel.display().to_string(),
            None => p.display().to_string(),
        }
    })
}

fn suggest_wiki_stem(
    stem: &str,
    stem_index: &HashMap<String, Vec<PathBuf>>,
) -> Option<String> {
    let mut best: Option<(usize, &str)> = None;
    for known in stem_index.keys() {
        let d = levenshtein(stem, known);
        if d > SUGGEST_MAX_EDIT_DISTANCE {
            continue;
        }
        if let Some((bd, _)) = best {
            if d < bd {
                best = Some((d, known));
            }
        } else {
            best = Some((d, known));
        }
    }
    best.map(|(_, s)| s.to_string())
}

/// Levenshtein distance with iterative two-row DP.
fn levenshtein(a: &str, b: &str) -> usize {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    if a_chars.is_empty() {
        return b_chars.len();
    }
    if b_chars.is_empty() {
        return a_chars.len();
    }
    let mut prev: Vec<usize> = (0..=b_chars.len()).collect();
    let mut curr = vec![0usize; b_chars.len() + 1];
    for (i, ac) in a_chars.iter().enumerate() {
        curr[0] = i + 1;
        for (j, bc) in b_chars.iter().enumerate() {
            let cost = if ac == bc { 0 } else { 1 };
            curr[j + 1] = (prev[j + 1] + 1)
                .min(curr[j] + 1)
                .min(prev[j] + cost);
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[b_chars.len()]
}

/// Compute path relative to base. Returns None if not expressible without ascending past base.
fn pathdiff(target: &Path, base: &Path) -> Option<PathBuf> {
    let target_parts: Vec<&std::ffi::OsStr> = target.iter().collect();
    let base_parts: Vec<&std::ffi::OsStr> = base.iter().collect();
    let mut common = 0;
    while common < target_parts.len() && common < base_parts.len()
        && target_parts[common] == base_parts[common]
    {
        common += 1;
    }
    let up = base_parts.len() - common;
    let mut out = PathBuf::new();
    for _ in 0..up {
        out.push("..");
    }
    for part in &target_parts[common..] {
        out.push(part);
    }
    Some(out)
}

/// Glob matcher supporting `**` (any subpath) and `*` (any non-slash chars).
fn glob_match(pattern: &str, path: &str) -> bool {
    glob_match_inner(pattern.as_bytes(), path.as_bytes())
}

fn glob_match_inner(p: &[u8], s: &[u8]) -> bool {
    let mut pi = 0usize;
    let mut si = 0usize;
    let mut star: Option<(usize, usize)> = None;
    let mut double_star: Option<(usize, usize)> = None;

    while si < s.len() {
        if pi < p.len() {
            if pi + 1 < p.len() && p[pi] == b'*' && p[pi + 1] == b'*' {
                // ** matches across slashes; consume any prefix.
                double_star = Some((pi + 2, si));
                pi += 2;
                if pi < p.len() && p[pi] == b'/' {
                    pi += 1;
                }
                continue;
            }
            if p[pi] == b'*' {
                star = Some((pi + 1, si));
                pi += 1;
                continue;
            }
            if p[pi] == s[si] {
                pi += 1;
                si += 1;
                continue;
            }
        }
        if let Some((sp, ss)) = star {
            // Backtrack: extend single-star match unless we'd cross a slash.
            if ss < s.len() && s[ss] != b'/' {
                star = Some((sp, ss + 1));
                pi = sp;
                si = ss + 1;
                continue;
            }
        }
        if let Some((sp, ss)) = double_star {
            // Backtrack: extend ** match (can cross slashes).
            if ss < s.len() {
                double_star = Some((sp, ss + 1));
                pi = sp;
                si = ss + 1;
                continue;
            }
        }
        return false;
    }
    while pi < p.len() && p[pi] == b'*' {
        pi += 1;
    }
    pi == p.len()
}

fn should_exclude(rel_path: &Path, patterns: &[String]) -> bool {
    if patterns.is_empty() {
        return false;
    }
    let rel_str = rel_path.to_string_lossy().replace('\\', "/");
    let file_name = rel_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();
    for pat in patterns {
        let normalised = pat.replace('\\', "/");
        if glob_match(&normalised, &rel_str) {
            return true;
        }
        if !normalised.contains('/') && glob_match(&normalised, &file_name) {
            return true;
        }
    }
    false
}

fn write_output(args: &Args, report: &Report) -> io::Result<()> {
    let body = match args.format {
        OutputFormat::Json => serde_json::to_string_pretty(report)
            .map_err(io::Error::other)?,
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
        "v2-gardening-sweep: {} files scanned ({} excluded) across {} root(s) (stale_threshold={}d, stale={}, dead_links={})\n",
        report.files_scanned,
        report.files_excluded,
        report.corpus.len(),
        report.stale_threshold_days,
        if report.stale_detection_enabled { "on" } else { "off" },
        if report.dead_link_detection_enabled { "on" } else { "off" },
    ));
    if let Some(ref s) = report.state_file {
        out.push_str(&format!("state_file: {s}\n"));
    }
    if let Some(ref c) = report.composition {
        out.push_str(&format!("composition: {c}\n"));
    }
    if report.stale_detection_enabled {
        out.push_str(&format!("\n## Stale ({} entries)\n", report.stale.len()));
        for entry in &report.stale {
            let unchanged_note = match entry.hash_unchanged {
                Some(true) => " [hash_unchanged]",
                Some(false) => " [hash_changed]",
                None => "",
            };
            out.push_str(&format!(
                "  {} — {}d old{}\n",
                entry.path, entry.age_days, unchanged_note
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
                LinkKind::MarkdownRef => "md-ref",
            };
            let suggest_note = match &entry.suggested_target {
                Some(s) => format!(" (suggest: {s})"),
                None => String::new(),
            };
            out.push_str(&format!(
                "  [{kind}] {}:{} -> {}{suggest_note}\n",
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
    fn fnv1a_64_known_value_for_empty_input() {
        assert_eq!(fnv1a_64(b""), FNV_OFFSET_BASIS);
    }

    #[test]
    fn fnv1a_64_differs_for_different_inputs() {
        let h1 = fnv1a_64(b"foo");
        let h2 = fnv1a_64(b"bar");
        assert_ne!(h1, h2);
    }

    #[test]
    fn fnv1a_64_is_deterministic() {
        assert_eq!(fnv1a_64(b"hello world"), fnv1a_64(b"hello world"));
    }

    #[test]
    fn levenshtein_handles_basic_cases() {
        assert_eq!(levenshtein("", ""), 0);
        assert_eq!(levenshtein("abc", "abc"), 0);
        assert_eq!(levenshtein("kitten", "sitting"), 3);
        assert_eq!(levenshtein("", "abc"), 3);
        assert_eq!(levenshtein("abc", ""), 3);
    }

    #[test]
    fn glob_match_simple_star() {
        assert!(glob_match("*.md", "foo.md"));
        assert!(!glob_match("*.md", "foo.txt"));
        assert!(!glob_match("*.md", "sub/foo.md")); // single * doesn't cross /
    }

    #[test]
    fn glob_match_double_star() {
        assert!(glob_match("**/*.md", "foo.md"));
        assert!(glob_match("**/*.md", "sub/foo.md"));
        assert!(glob_match("**/*.md", "sub/deep/foo.md"));
    }

    #[test]
    fn glob_match_literal() {
        assert!(glob_match("draft-foo.md", "draft-foo.md"));
        assert!(!glob_match("draft-foo.md", "other.md"));
    }

    #[test]
    fn glob_match_prefix_double_star() {
        assert!(glob_match("**/archive/*.md", "x/archive/old.md"));
        assert!(glob_match("**/archive/*.md", "deep/sub/archive/old.md"));
        assert!(!glob_match("**/archive/*.md", "archive/sub/old.md"));
    }

    #[test]
    fn should_exclude_matches_full_path() {
        let excludes = vec!["archive/*.md".to_string()];
        assert!(should_exclude(Path::new("archive/old.md"), &excludes));
        assert!(!should_exclude(Path::new("notes/old.md"), &excludes));
    }

    #[test]
    fn should_exclude_matches_basename() {
        let excludes = vec!["draft-*.md".to_string()];
        assert!(should_exclude(Path::new("sub/draft-foo.md"), &excludes));
        assert!(!should_exclude(Path::new("sub/foo.md"), &excludes));
    }

    #[test]
    fn build_line_mask_skips_frontmatter() {
        let content = "---\ntitle: foo\n---\n# Body\n[link](path.md)\n";
        let mask = build_line_mask(content);
        assert_eq!(mask.process, vec![false, false, false, true, true]);
    }

    #[test]
    fn build_line_mask_skips_fenced_code() {
        let content = "# Title\n```\nin fence\n```\nafter\n";
        let mask = build_line_mask(content);
        assert_eq!(mask.process, vec![true, false, false, false, true]);
    }

    #[test]
    fn build_line_mask_skips_tilde_fence() {
        let content = "before\n~~~\nin fence\n~~~\nafter\n";
        let mask = build_line_mask(content);
        assert_eq!(mask.process, vec![true, false, false, false, true]);
    }

    #[test]
    fn build_line_mask_skips_multiline_html_comment() {
        let content = "before\n<!--\ncomment\n-->\nafter\n";
        let mask = build_line_mask(content);
        assert_eq!(mask.process, vec![true, false, false, false, true]);
    }

    #[test]
    fn build_line_mask_skips_singleline_html_comment() {
        let content = "before\n<!-- comment -->\nafter\n";
        let mask = build_line_mask(content);
        assert_eq!(mask.process, vec![true, false, true]);
    }

    #[test]
    fn build_line_mask_skips_indented_code() {
        let content = "para\n    code [fake](nope.md)\nback\n";
        let mask = build_line_mask(content);
        assert_eq!(mask.process, vec![true, false, true]);
    }

    #[test]
    fn parse_ref_defs_basic() {
        let content = "[ref1]: https://example.com\n[ref2]: path.md\n";
        let mask = build_line_mask(content);
        let defs = parse_ref_defs(content, &mask);
        assert_eq!(defs.get("ref1"), Some(&"https://example.com".to_string()));
        assert_eq!(defs.get("ref2"), Some(&"path.md".to_string()));
    }

    #[test]
    fn parse_ref_defs_strips_title() {
        let content = "[ref1]: path.md \"title here\"\n";
        let mask = build_line_mask(content);
        let defs = parse_ref_defs(content, &mask);
        assert_eq!(defs.get("ref1"), Some(&"path.md".to_string()));
    }

    #[test]
    fn parse_ref_defs_case_insensitive() {
        let content = "[Ref1]: path.md\n";
        let mask = build_line_mask(content);
        let defs = parse_ref_defs(content, &mask);
        assert_eq!(defs.get("ref1"), Some(&"path.md".to_string()));
    }

    #[test]
    fn extract_markdown_links_handles_simple_inline() {
        let content = "see [docs](docs/redesign/README.md) for details";
        let mask = build_line_mask(content);
        let refs = HashMap::new();
        let links = extract_markdown_links_multiline(content, &mask, &refs);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "docs/redesign/README.md");
        assert_eq!(links[0].kind, LinkKind::Markdown);
    }

    #[test]
    fn extract_markdown_links_resolves_reference_style() {
        let content = "see [docs][ref1] for details\n\n[ref1]: path.md\n";
        let mask = build_line_mask(content);
        let refs = parse_ref_defs(content, &mask);
        let links = extract_markdown_links_multiline(content, &mask, &refs);
        let md_refs: Vec<&ExtractedLink> = links
            .iter()
            .filter(|l| matches!(l.kind, LinkKind::MarkdownRef))
            .collect();
        assert_eq!(md_refs.len(), 1);
        assert_eq!(md_refs[0].target, "path.md");
    }

    #[test]
    fn extract_markdown_links_collapses_reference() {
        let content = "see [docs][] for details\n\n[docs]: path.md\n";
        let mask = build_line_mask(content);
        let refs = parse_ref_defs(content, &mask);
        let links = extract_markdown_links_multiline(content, &mask, &refs);
        let md_refs: Vec<&ExtractedLink> = links
            .iter()
            .filter(|l| matches!(l.kind, LinkKind::MarkdownRef))
            .collect();
        assert_eq!(md_refs.len(), 1);
        assert_eq!(md_refs[0].target, "path.md");
    }

    #[test]
    fn extract_markdown_links_undefined_reference() {
        let content = "see [docs][nope] for details\n";
        let mask = build_line_mask(content);
        let refs = parse_ref_defs(content, &mask);
        let links = extract_markdown_links_multiline(content, &mask, &refs);
        let md_refs: Vec<&ExtractedLink> = links
            .iter()
            .filter(|l| matches!(l.kind, LinkKind::MarkdownRef))
            .collect();
        assert_eq!(md_refs.len(), 1);
        assert!(md_refs[0].target.starts_with("[undefined-ref:"));
    }

    #[test]
    fn extract_markdown_links_skips_masked_regions() {
        let content = "```\n[fake](missing.md)\n```\n[real](actual.md)\n";
        let mask = build_line_mask(content);
        let refs = HashMap::new();
        let links = extract_markdown_links_multiline(content, &mask, &refs);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "actual.md");
    }

    #[test]
    fn extract_markdown_links_multiline_target() {
        let content = "see [docs](\nfoo.md\n) for details\n";
        let mask = build_line_mask(content);
        let refs = HashMap::new();
        let links = extract_markdown_links_multiline(content, &mask, &refs);
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].target, "foo.md");
    }

    #[test]
    fn extract_wiki_links_handles_simple_wiki() {
        let content = "link to [[name-here]] in body";
        let mask = build_line_mask(content);
        let links = extract_wiki_links_multiline(content, &mask);
        assert_eq!(links, vec![(1, "name-here".to_string())]);
    }

    #[test]
    fn extract_wiki_links_handles_alias_form() {
        let content = "see [[name|alias text]]";
        let mask = build_line_mask(content);
        let links = extract_wiki_links_multiline(content, &mask);
        assert_eq!(links, vec![(1, "name|alias text".to_string())]);
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

    #[test]
    fn build_line_offsets_correct() {
        let offsets = build_line_offsets("a\nbb\nccc\n");
        assert_eq!(offsets, vec![0, 2, 5, 9]);
    }

    #[test]
    fn lookup_line_finds_correct_index() {
        let offsets = vec![0, 2, 5, 9];
        assert_eq!(lookup_line(&offsets, 0), 0);
        assert_eq!(lookup_line(&offsets, 1), 0);
        assert_eq!(lookup_line(&offsets, 2), 1);
        assert_eq!(lookup_line(&offsets, 6), 2);
    }

    #[test]
    fn suggest_target_finds_near_match() {
        let mut idx: HashMap<String, Vec<PathBuf>> = HashMap::new();
        idx.insert("foo-bar".to_string(), vec![PathBuf::from("docs/foo-bar.md")]);
        let suggested = suggest_target("foo-baz.md", &idx, Path::new("docs/source.md"));
        assert!(suggested.is_some());
        assert!(suggested.unwrap().contains("foo-bar"));
    }

    #[test]
    fn suggest_target_returns_none_when_too_far() {
        let mut idx: HashMap<String, Vec<PathBuf>> = HashMap::new();
        idx.insert("abc".to_string(), vec![PathBuf::from("docs/abc.md")]);
        let suggested = suggest_target("xyzqq-much-longer.md", &idx, Path::new("docs/source.md"));
        assert!(suggested.is_none());
    }
}
