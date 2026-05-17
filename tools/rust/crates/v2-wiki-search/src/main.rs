use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

const SCHEMA: &str = "v2-wiki-search/v1";
const INDEX_SCHEMA: &str = "v2-wiki-search-index/v1";
const DESCRIPTION_MAX_CHARS: usize = 500;
const TITLE_WEIGHT: f64 = 3.0;
const DESCRIPTION_WEIGHT: f64 = 2.0;
const TAGS_WEIGHT: f64 = 1.5;
const BODY_WEIGHT: f64 = 1.0;

#[derive(Parser, Debug)]
#[command(
    name = "v2-wiki-search",
    about = "Top-k retrieval over markdown corpora (TF-IDF ranking, opt-in index cache, YAML-lite frontmatter)"
)]
struct Args {
    /// Corpus directory to search. May be repeated to search multiple roots
    /// (e.g. --corpus docs/journal --corpus docs/redesign/_notes).
    #[arg(long, required = true)]
    corpus: Vec<PathBuf>,

    /// Search query. Whitespace-tokenized into terms; each term is matched
    /// case-insensitively as a substring. Empty query is rejected.
    #[arg(long)]
    query: String,

    /// Return at most this many top-scored results (default 10).
    #[arg(long, default_value_t = 10)]
    top_k: usize,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Json)]
    format: OutputFormat,

    /// Write output to a file (default: stdout, denoted as `-`).
    #[arg(long, default_value = "-")]
    output: String,

    /// Include document body in JSON output (default false to reduce noise).
    /// Title and description are always included.
    #[arg(long)]
    include_body: bool,

    /// Minimum score threshold; results below this are dropped. Default 0.0
    /// keeps all matches that touched at least one query term.
    #[arg(long, default_value_t = 0.0)]
    min_score: f64,

    /// Strict mode — non-zero exit if any per-file parse warning was emitted.
    #[arg(long)]
    strict: bool,

    /// Path to a persistent JSON index cache (opt-in). If present and the
    /// signatures (path, mtime, size) of the current corpus match the cache,
    /// the cached document list is used; otherwise the cache is rebuilt.
    /// Without this flag, the corpus is parsed fresh every invocation.
    #[arg(long)]
    index_cache: Option<PathBuf>,

    /// Force a rebuild of the index cache. Has no effect unless --index-cache
    /// is also set.
    #[arg(long)]
    rebuild_index: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum OutputFormat {
    Json,
    Text,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Document {
    path: PathBuf,
    title: String,
    description: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    frontmatter_present: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RankedResult {
    path: PathBuf,
    title: String,
    description: String,
    score: f64,
    matched_terms: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    tags: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SearchReport {
    schema: String,
    query: String,
    query_terms: Vec<String>,
    corpus_paths: Vec<PathBuf>,
    documents_indexed: usize,
    documents_skipped: usize,
    top_k: usize,
    min_score: f64,
    results: Vec<RankedResult>,
    notes: Vec<String>,
    warnings_by_kind: HashMap<String, usize>,
    index_status: String,
    deferred_stages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileSig {
    path: PathBuf,
    mtime: u64,
    size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CorpusIndex {
    schema: String,
    generated_at: u64,
    corpus_paths: Vec<PathBuf>,
    file_signatures: Vec<FileSig>,
    documents: Vec<Document>,
    skipped_signatures: Vec<FileSig>,
    skipped_notes: Vec<String>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum WarnKind {
    CorpusMissing,
    CorpusNotDir,
    WalkError,
    FileRead,
    FileEmpty,
    FrontmatterMalformed,
    IndexCorrupt,
    IndexWriteFailed,
}

impl WarnKind {
    fn slug(self) -> &'static str {
        match self {
            WarnKind::CorpusMissing => "corpus-missing",
            WarnKind::CorpusNotDir => "corpus-not-dir",
            WarnKind::WalkError => "walk-error",
            WarnKind::FileRead => "file-read",
            WarnKind::FileEmpty => "file-empty",
            WarnKind::FrontmatterMalformed => "frontmatter-malformed",
            WarnKind::IndexCorrupt => "index-corrupt",
            WarnKind::IndexWriteFailed => "index-write-failed",
        }
    }
}

fn warn_note(kind: WarnKind, msg: &str) -> String {
    format!("warn[{}]: {}", kind.slug(), msg)
}

fn main() -> ExitCode {
    let args = Args::parse();
    match run(&args) {
        Ok(report) => {
            if let Err(e) = emit(&args, &report) {
                eprintln!("emit error: {e}");
                return ExitCode::from(2);
            }
            if args.strict && report.notes.iter().any(|n| n.starts_with("warn")) {
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

fn run(args: &Args) -> Result<SearchReport, String> {
    if args.query.trim().is_empty() {
        return Err("empty query".to_string());
    }
    let query_terms = tokenize_query(&args.query);
    if query_terms.is_empty() {
        return Err("query has no usable terms after tokenization".to_string());
    }

    let mut notes: Vec<String> = Vec::new();
    let (documents, documents_skipped, index_status) = load_or_build_index(args, &mut notes);

    let total_docs = documents.len();
    let doc_frequencies = compute_doc_frequencies(&documents, &query_terms);

    let mut ranked: Vec<RankedResult> = documents
        .iter()
        .filter_map(|doc| {
            let (score, matched_terms) =
                score_document(doc, &query_terms, &doc_frequencies, total_docs);
            if score < args.min_score {
                None
            } else if matched_terms.is_empty() && args.min_score <= 0.0 {
                None
            } else {
                Some(RankedResult {
                    path: doc.path.clone(),
                    title: doc.title.clone(),
                    description: doc.description.clone(),
                    score,
                    matched_terms,
                    tags: doc.tags.clone(),
                    body: if args.include_body {
                        doc.body.clone()
                    } else {
                        None
                    },
                })
            }
        })
        .collect();

    ranked.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.path.cmp(&b.path))
    });
    ranked.truncate(args.top_k);

    let warnings_by_kind = tally_warnings(&notes);

    Ok(SearchReport {
        schema: SCHEMA.to_string(),
        query: args.query.clone(),
        query_terms,
        corpus_paths: args.corpus.clone(),
        documents_indexed: total_docs,
        documents_skipped,
        top_k: args.top_k,
        min_score: args.min_score,
        results: ranked,
        notes,
        warnings_by_kind,
        index_status,
        deferred_stages: Vec::new(),
    })
}

fn tokenize_query(query: &str) -> Vec<String> {
    query
        .split_whitespace()
        .map(|t| {
            t.trim_matches(|c: char| c.is_ascii_punctuation())
                .to_lowercase()
        })
        .filter(|t| !t.is_empty())
        .collect()
}

fn walk_markdown(root: &Path) -> io::Result<Vec<PathBuf>> {
    let mut out: Vec<PathBuf> = Vec::new();
    let mut stack: Vec<PathBuf> = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir)? {
            let entry = entry?;
            let path = entry.path();
            let ft = entry.file_type()?;
            if ft.is_dir() {
                stack.push(path);
            } else if ft.is_file() && path.extension().and_then(|e| e.to_str()) == Some("md") {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}

fn parse_document(path: &Path) -> Result<Document, (WarnKind, String)> {
    let raw = fs::read_to_string(path).map_err(|e| (WarnKind::FileRead, format!("read: {e}")))?;
    if raw.trim().is_empty() {
        return Err((WarnKind::FileEmpty, "empty file".to_string()));
    }

    let (frontmatter, after_fm, fm_present, fm_err) = extract_frontmatter(&raw);

    let title = frontmatter
        .get("title")
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .or_else(|| extract_title(after_fm))
        .unwrap_or_else(|| {
            path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("untitled")
                .to_string()
        });

    let description = frontmatter
        .get("description")
        .map(|s| truncate_chars(s.trim(), DESCRIPTION_MAX_CHARS))
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| extract_description(after_fm));

    let tags = frontmatter
        .get("tags")
        .map(|s| parse_tags(s))
        .unwrap_or_default();

    if let Some(msg) = fm_err {
        return Err((WarnKind::FrontmatterMalformed, msg));
    }

    Ok(Document {
        path: path.to_path_buf(),
        title,
        description,
        tags,
        frontmatter_present: fm_present,
        body: Some(raw),
    })
}

fn extract_frontmatter(raw: &str) -> (HashMap<String, String>, &str, bool, Option<String>) {
    let mut map: HashMap<String, String> = HashMap::new();
    let trimmed_start = raw.trim_start_matches('\u{feff}');
    let lines: Vec<&str> = trimmed_start.lines().collect();
    if lines.is_empty() || lines[0].trim() != "---" {
        return (map, raw, false, None);
    }

    let mut close_line: Option<usize> = None;
    for (i, line) in lines.iter().enumerate().skip(1) {
        if line.trim() == "---" {
            close_line = Some(i);
            break;
        }
    }

    let Some(close_idx) = close_line else {
        return (
            map,
            raw,
            false,
            Some("frontmatter opener `---` without matching closer".to_string()),
        );
    };

    for line in &lines[1..close_idx] {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = trimmed.split_once(':') {
            let key = k.trim().to_string();
            let val = v.trim().trim_matches('"').trim_matches('\'').to_string();
            if !key.is_empty() {
                map.insert(key, val);
            }
        }
    }

    let mut byte_offset = 0usize;
    for (i, line) in lines.iter().enumerate() {
        byte_offset += line.len();
        if let Some(ch) = trimmed_start.as_bytes().get(byte_offset) {
            if *ch == b'\n' {
                byte_offset += 1;
            }
        }
        if i == close_idx {
            break;
        }
    }

    let after_fm = trimmed_start.get(byte_offset..).unwrap_or("");
    (map, after_fm, true, None)
}

fn parse_tags(value: &str) -> Vec<String> {
    let trimmed = value.trim();
    let body = trimmed
        .strip_prefix('[')
        .and_then(|s| s.strip_suffix(']'))
        .unwrap_or(trimmed);
    body.split(',')
        .map(|t| t.trim().trim_matches('"').trim_matches('\'').to_string())
        .filter(|t| !t.is_empty())
        .collect()
}

fn extract_title(raw: &str) -> Option<String> {
    for line in raw.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix("# ") {
            let title = rest.trim().to_string();
            if !title.is_empty() {
                return Some(title);
            }
        }
    }
    None
}

fn extract_description(raw: &str) -> String {
    let mut after_h1 = false;
    let mut collected: Vec<&str> = Vec::new();
    for line in raw.lines() {
        let trimmed = line.trim_start();
        if !after_h1 {
            if trimmed.starts_with("# ") {
                after_h1 = true;
            }
            continue;
        }
        if trimmed.is_empty() {
            if !collected.is_empty() {
                break;
            }
            continue;
        }
        if trimmed.starts_with('#') {
            break;
        }
        collected.push(trimmed);
    }
    let joined = collected.join(" ");
    truncate_chars(&joined, DESCRIPTION_MAX_CHARS)
}

fn truncate_chars(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        return s.to_string();
    }
    let truncated: String = s.chars().take(max).collect();
    format!("{}…", truncated.trim_end())
}

fn compute_doc_frequencies(documents: &[Document], terms: &[String]) -> HashMap<String, usize> {
    let mut df: HashMap<String, usize> = HashMap::new();
    for term in terms {
        df.insert(term.clone(), 0);
    }
    for doc in documents {
        let title_lc = doc.title.to_lowercase();
        let desc_lc = doc.description.to_lowercase();
        let tags_lc: String = doc
            .tags
            .iter()
            .map(|t| t.to_lowercase())
            .collect::<Vec<_>>()
            .join(" ");
        let body_lc = doc
            .body
            .as_ref()
            .map(|b| b.to_lowercase())
            .unwrap_or_default();
        for term in terms {
            let present = title_lc.contains(term)
                || desc_lc.contains(term)
                || tags_lc.contains(term)
                || body_lc.contains(term);
            if present {
                *df.entry(term.clone()).or_insert(0) += 1;
            }
        }
    }
    df
}

fn tf_sublinear(count: usize) -> f64 {
    if count == 0 {
        0.0
    } else {
        1.0 + (count as f64).ln()
    }
}

fn idf(df: usize, n: usize) -> f64 {
    let numer = (n as f64) + 1.0;
    let denom = (df as f64) + 1.0;
    (numer / denom).ln() + 1.0
}

fn score_document(
    doc: &Document,
    terms: &[String],
    doc_frequencies: &HashMap<String, usize>,
    total_docs: usize,
) -> (f64, Vec<String>) {
    let title_lc = doc.title.to_lowercase();
    let desc_lc = doc.description.to_lowercase();
    let tags_lc: String = doc
        .tags
        .iter()
        .map(|t| t.to_lowercase())
        .collect::<Vec<_>>()
        .join(" ");
    let body_lc = doc
        .body
        .as_ref()
        .map(|b| b.to_lowercase())
        .unwrap_or_default();

    let mut score = 0.0f64;
    let mut matched: Vec<String> = Vec::new();

    for term in terms {
        let in_title = count_substring(&title_lc, term);
        let in_desc = count_substring(&desc_lc, term);
        let in_tags = count_substring(&tags_lc, term);
        let in_body_total = count_substring(&body_lc, term);
        let in_body_only = in_body_total.saturating_sub(in_title + in_desc);

        if in_title == 0 && in_desc == 0 && in_tags == 0 && in_body_only == 0 {
            continue;
        }

        let df = doc_frequencies.get(term).copied().unwrap_or(0);
        let idf_val = idf(df, total_docs);

        let term_score = idf_val
            * (TITLE_WEIGHT * tf_sublinear(in_title)
                + DESCRIPTION_WEIGHT * tf_sublinear(in_desc)
                + TAGS_WEIGHT * tf_sublinear(in_tags)
                + BODY_WEIGHT * tf_sublinear(in_body_only));

        if term_score > 0.0 {
            score += term_score;
            matched.push(term.clone());
        }
    }

    (score, matched)
}

fn count_substring(haystack: &str, needle: &str) -> usize {
    if needle.is_empty() {
        return 0;
    }
    let mut count = 0usize;
    let mut start = 0usize;
    while let Some(pos) = haystack[start..].find(needle) {
        count += 1;
        start += pos + needle.len();
        if start >= haystack.len() {
            break;
        }
    }
    count
}

fn tally_warnings(notes: &[String]) -> HashMap<String, usize> {
    let mut out: HashMap<String, usize> = HashMap::new();
    for n in notes {
        if let Some(rest) = n.strip_prefix("warn[") {
            if let Some(end) = rest.find(']') {
                let kind = &rest[..end];
                *out.entry(kind.to_string()).or_insert(0) += 1;
            }
        }
    }
    out
}

fn now_unix_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn compute_signatures(corpus_paths: &[PathBuf]) -> Result<Vec<FileSig>, String> {
    let mut sigs: Vec<FileSig> = Vec::new();
    for root in corpus_paths {
        if !root.exists() || !root.is_dir() {
            continue;
        }
        let files = walk_markdown(root).map_err(|e| format!("walk {}: {e}", root.display()))?;
        for path in files {
            let meta = fs::metadata(&path).map_err(|e| format!("stat {}: {e}", path.display()))?;
            let mtime = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            sigs.push(FileSig {
                path,
                mtime,
                size: meta.len(),
            });
        }
    }
    sigs.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(sigs)
}

fn signatures_match(a: &[FileSig], b: &[FileSig]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    for (x, y) in a.iter().zip(b.iter()) {
        if x.path != y.path || x.mtime != y.mtime || x.size != y.size {
            return false;
        }
    }
    true
}

fn read_cache(path: &Path) -> Result<CorpusIndex, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("read cache: {e}"))?;
    let idx: CorpusIndex = serde_json::from_str(&raw).map_err(|e| format!("parse cache: {e}"))?;
    if idx.schema != INDEX_SCHEMA {
        return Err(format!(
            "cache schema mismatch (got `{}`, expected `{}`)",
            idx.schema, INDEX_SCHEMA
        ));
    }
    Ok(idx)
}

fn write_cache(path: &Path, index: &CorpusIndex) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).map_err(|e| format!("create cache dir: {e}"))?;
        }
    }
    let body = serde_json::to_string_pretty(index).map_err(|e| format!("serialize cache: {e}"))?;
    fs::write(path, body).map_err(|e| format!("write cache: {e}"))?;
    Ok(())
}

fn build_index_fresh(
    corpus_paths: &[PathBuf],
    notes: &mut Vec<String>,
) -> (Vec<Document>, Vec<FileSig>, Vec<FileSig>, Vec<String>) {
    let mut documents: Vec<Document> = Vec::new();
    let mut sigs: Vec<FileSig> = Vec::new();
    let mut skipped_sigs: Vec<FileSig> = Vec::new();
    let mut skipped_notes: Vec<String> = Vec::new();

    for corpus_root in corpus_paths {
        if !corpus_root.exists() {
            let n = warn_note(
                WarnKind::CorpusMissing,
                &format!("corpus path does not exist: {}", corpus_root.display()),
            );
            notes.push(n.clone());
            skipped_notes.push(n);
            continue;
        }
        if !corpus_root.is_dir() {
            let n = warn_note(
                WarnKind::CorpusNotDir,
                &format!("corpus path is not a directory: {}", corpus_root.display()),
            );
            notes.push(n.clone());
            skipped_notes.push(n);
            continue;
        }
        match walk_markdown(corpus_root) {
            Ok(paths) => {
                for path in paths {
                    let meta = match fs::metadata(&path) {
                        Ok(m) => m,
                        Err(e) => {
                            let n = warn_note(
                                WarnKind::FileRead,
                                &format!("stat {}: {e}", path.display()),
                            );
                            notes.push(n.clone());
                            skipped_notes.push(n);
                            continue;
                        }
                    };
                    let mtime = meta
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0);
                    let size = meta.len();
                    let sig = FileSig {
                        path: path.clone(),
                        mtime,
                        size,
                    };
                    match parse_document(&path) {
                        Ok(doc) => {
                            documents.push(doc);
                            sigs.push(sig);
                        }
                        Err((kind, msg)) => {
                            let n = warn_note(kind, &format!("skipped {}: {msg}", path.display()));
                            notes.push(n.clone());
                            skipped_notes.push(n);
                            skipped_sigs.push(sig);
                        }
                    }
                }
            }
            Err(e) => {
                let n = warn_note(
                    WarnKind::WalkError,
                    &format!("failed to walk {}: {e}", corpus_root.display()),
                );
                notes.push(n.clone());
                skipped_notes.push(n);
            }
        }
    }
    documents.sort_by(|a, b| a.path.cmp(&b.path));
    sigs.sort_by(|a, b| a.path.cmp(&b.path));
    skipped_sigs.sort_by(|a, b| a.path.cmp(&b.path));
    (documents, sigs, skipped_sigs, skipped_notes)
}

fn load_or_build_index(args: &Args, notes: &mut Vec<String>) -> (Vec<Document>, usize, String) {
    let Some(cache_path) = &args.index_cache else {
        let (documents, _sigs, skipped_sigs, _skipped_notes) =
            build_index_fresh(&args.corpus, notes);
        let skipped = skipped_sigs.len();
        return (documents, skipped, "no-cache".to_string());
    };

    if args.rebuild_index {
        let (documents, sigs, skipped_sigs, skipped_notes) = build_index_fresh(&args.corpus, notes);
        let index = CorpusIndex {
            schema: INDEX_SCHEMA.to_string(),
            generated_at: now_unix_secs(),
            corpus_paths: args.corpus.clone(),
            file_signatures: sigs,
            documents: documents.clone(),
            skipped_signatures: skipped_sigs.clone(),
            skipped_notes,
        };
        if let Err(e) = write_cache(cache_path, &index) {
            notes.push(warn_note(WarnKind::IndexWriteFailed, &e));
        }
        let skipped = skipped_sigs.len();
        return (documents, skipped, "rebuilt-forced".to_string());
    }

    match read_cache(cache_path) {
        Ok(cached) => {
            let current_sigs = match compute_signatures(&args.corpus) {
                Ok(s) => s,
                Err(e) => {
                    notes.push(warn_note(WarnKind::WalkError, &e));
                    let (documents, _sigs, skipped_sigs, _skipped_notes) =
                        build_index_fresh(&args.corpus, notes);
                    return (
                        documents,
                        skipped_sigs.len(),
                        "rebuilt-after-walk-error".to_string(),
                    );
                }
            };

            let active_match = signatures_match(&current_sigs, &cached.file_signatures);
            if active_match && cached.corpus_paths == args.corpus {
                for n in &cached.skipped_notes {
                    notes.push(n.clone());
                }
                let skipped = cached.skipped_signatures.len();
                return (cached.documents, skipped, "cached".to_string());
            }

            let (documents, sigs, skipped_sigs, skipped_notes) =
                build_index_fresh(&args.corpus, notes);
            let index = CorpusIndex {
                schema: INDEX_SCHEMA.to_string(),
                generated_at: now_unix_secs(),
                corpus_paths: args.corpus.clone(),
                file_signatures: sigs,
                documents: documents.clone(),
                skipped_signatures: skipped_sigs.clone(),
                skipped_notes,
            };
            if let Err(e) = write_cache(cache_path, &index) {
                notes.push(warn_note(WarnKind::IndexWriteFailed, &e));
            }
            (documents, skipped_sigs.len(), "rebuilt-stale".to_string())
        }
        Err(e) => {
            let preexisting = cache_path.exists();
            if preexisting {
                notes.push(warn_note(WarnKind::IndexCorrupt, &e));
            }
            let (documents, sigs, skipped_sigs, skipped_notes) =
                build_index_fresh(&args.corpus, notes);
            let index = CorpusIndex {
                schema: INDEX_SCHEMA.to_string(),
                generated_at: now_unix_secs(),
                corpus_paths: args.corpus.clone(),
                file_signatures: sigs,
                documents: documents.clone(),
                skipped_signatures: skipped_sigs.clone(),
                skipped_notes,
            };
            if let Err(e2) = write_cache(cache_path, &index) {
                notes.push(warn_note(WarnKind::IndexWriteFailed, &e2));
            }
            (
                documents,
                skipped_sigs.len(),
                if preexisting {
                    "rebuilt-corrupt".to_string()
                } else {
                    "rebuilt-fresh".to_string()
                },
            )
        }
    }
}

fn emit(args: &Args, report: &SearchReport) -> Result<(), String> {
    let rendered = match args.format {
        OutputFormat::Json => render_json(report)?,
        OutputFormat::Text => render_text(report),
    };
    if args.output == "-" {
        io::stdout()
            .write_all(rendered.as_bytes())
            .map_err(|e| format!("stdout: {e}"))?;
        if !rendered.ends_with('\n') {
            io::stdout().write_all(b"\n").ok();
        }
    } else {
        fs::write(&args.output, &rendered).map_err(|e| format!("file write: {e}"))?;
    }
    Ok(())
}

fn render_json(report: &SearchReport) -> Result<String, String> {
    serde_json::to_string_pretty(report).map_err(|e| format!("json: {e}"))
}

fn render_text(report: &SearchReport) -> String {
    let mut out = String::new();
    out.push_str(&format!("# wiki-search: {}\n\n", report.query));
    out.push_str(&format!(
        "Corpus: {} path(s); {} document(s) indexed; {} skipped; index-status: {}\n",
        report.corpus_paths.len(),
        report.documents_indexed,
        report.documents_skipped,
        report.index_status,
    ));
    out.push_str(&format!("Query terms: {}\n", report.query_terms.join(", ")));
    out.push_str(&format!(
        "Top-k: {} | Min-score: {:.2}\n\n",
        report.top_k, report.min_score
    ));

    if report.results.is_empty() {
        out.push_str("No results.\n");
    } else {
        out.push_str(&format!("## Results ({})\n\n", report.results.len()));
        for (i, r) in report.results.iter().enumerate() {
            out.push_str(&format!(
                "{}. [{:.2}] {} — {}\n",
                i + 1,
                r.score,
                r.title,
                r.path.display()
            ));
            if !r.description.is_empty() {
                out.push_str(&format!("   {}\n", r.description));
            }
            if !r.tags.is_empty() {
                out.push_str(&format!("   tags: {}\n", r.tags.join(", ")));
            }
            out.push_str(&format!("   matched: {}\n\n", r.matched_terms.join(", ")));
        }
    }

    if !report.notes.is_empty() {
        out.push_str(&format!("## Notes ({})\n\n", report.notes.len()));
        for n in &report.notes {
            out.push_str(&format!("- {n}\n"));
        }
        out.push('\n');
    }

    if !report.warnings_by_kind.is_empty() {
        out.push_str("## Warnings by kind\n\n");
        let mut entries: Vec<(&String, &usize)> = report.warnings_by_kind.iter().collect();
        entries.sort_by(|a, b| a.0.cmp(b.0));
        for (kind, count) in entries {
            out.push_str(&format!("- {kind}: {count}\n"));
        }
        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args_for(corpus: Vec<PathBuf>, query: &str) -> Args {
        Args {
            corpus,
            query: query.to_string(),
            top_k: 10,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: false,
            min_score: 0.0,
            strict: false,
            index_cache: None,
            rebuild_index: false,
        }
    }

    #[test]
    fn tokenize_lowercases_and_strips_punctuation() {
        let terms = tokenize_query("Hello, World! orchestration-hub.");
        assert_eq!(terms, vec!["hello", "world", "orchestration-hub"]);
    }

    #[test]
    fn tokenize_filters_empty_terms() {
        let terms = tokenize_query("  ,  !  ");
        assert!(terms.is_empty());
    }

    #[test]
    fn extract_title_finds_first_h1() {
        let title = extract_title("# Cycle 42 — Foo\n\nSome prose\n# Not the title\n");
        assert_eq!(title.as_deref(), Some("Cycle 42 — Foo"));
    }

    #[test]
    fn extract_title_returns_none_when_no_h1() {
        let title = extract_title("Just some prose\n## Subsection\nMore prose\n");
        assert!(title.is_none());
    }

    #[test]
    fn extract_description_grabs_first_paragraph_after_h1() {
        let desc = extract_description(
            "# Title\n\nFirst paragraph here.\nSecond line of same paragraph.\n\nNew paragraph.\n",
        );
        assert_eq!(desc, "First paragraph here. Second line of same paragraph.");
    }

    #[test]
    fn extract_description_stops_at_next_heading() {
        let desc = extract_description(
            "# Title\n\nFirst paragraph.\n\n## Subsection\n\nNot in description.\n",
        );
        assert_eq!(desc, "First paragraph.");
    }

    #[test]
    fn extract_description_truncates_at_max() {
        let long = format!("# Title\n\n{}\n", "a".repeat(DESCRIPTION_MAX_CHARS + 50));
        let desc = extract_description(&long);
        assert!(desc.ends_with('…'));
        assert!(desc.chars().count() <= DESCRIPTION_MAX_CHARS + 1);
    }

    #[test]
    fn extract_description_returns_empty_when_no_h1() {
        let desc = extract_description("Just prose, no heading.\n");
        assert!(desc.is_empty());
    }

    #[test]
    fn count_substring_counts_non_overlapping_matches() {
        assert_eq!(count_substring("aaaa", "aa"), 2);
        assert_eq!(count_substring("hello world hello", "hello"), 2);
        assert_eq!(count_substring("nothing", "xyz"), 0);
        assert_eq!(count_substring("anything", ""), 0);
    }

    #[test]
    fn tf_sublinear_is_zero_when_no_matches() {
        assert_eq!(tf_sublinear(0), 0.0);
    }

    #[test]
    fn tf_sublinear_grows_with_diminishing_returns() {
        let t1 = tf_sublinear(1);
        let t2 = tf_sublinear(2);
        let t10 = tf_sublinear(10);
        let t11 = tf_sublinear(11);
        assert!((t1 - 1.0).abs() < 1e-9);
        assert!(t2 > t1);
        assert!(t11 > t10);
        assert!(
            (t2 - t1) > (t11 - t10),
            "consecutive deltas should shrink: ({t2}-{t1}={}) vs ({t11}-{t10}={})",
            t2 - t1,
            t11 - t10
        );
    }

    #[test]
    fn idf_returns_one_when_term_is_universal() {
        let v = idf(5, 5);
        assert!((v - 1.0).abs() < 1e-9);
    }

    #[test]
    fn idf_grows_when_term_is_rare() {
        let common = idf(8, 10);
        let rare = idf(1, 10);
        assert!(rare > common);
    }

    #[test]
    fn score_document_weights_title_over_body() {
        let doc = Document {
            path: PathBuf::from("a.md"),
            title: "foo".to_string(),
            description: "".to_string(),
            tags: Vec::new(),
            frontmatter_present: false,
            body: Some("# foo\nsomething else\n".to_string()),
        };
        let terms = vec!["foo".to_string()];
        let mut df = HashMap::new();
        df.insert("foo".to_string(), 1);
        let (score, matched) = score_document(&doc, &terms, &df, 1);
        assert!(score > 0.0);
        assert_eq!(matched, vec!["foo"]);
    }

    #[test]
    fn score_document_returns_zero_for_unmatched() {
        let doc = Document {
            path: PathBuf::from("a.md"),
            title: "foo".to_string(),
            description: "bar".to_string(),
            tags: Vec::new(),
            frontmatter_present: false,
            body: Some("baz".to_string()),
        };
        let terms = vec!["xyz".to_string()];
        let df = HashMap::new();
        let (score, matched) = score_document(&doc, &terms, &df, 1);
        assert_eq!(score, 0.0);
        assert!(matched.is_empty());
    }

    #[test]
    fn score_document_accumulates_across_terms() {
        let doc = Document {
            path: PathBuf::from("a.md"),
            title: "scaffold-partial pattern".to_string(),
            description: "scaffold and pattern words".to_string(),
            tags: Vec::new(),
            frontmatter_present: false,
            body: Some("body has scaffold once".to_string()),
        };
        let terms = vec!["scaffold".to_string(), "pattern".to_string()];
        let mut df = HashMap::new();
        df.insert("scaffold".to_string(), 1);
        df.insert("pattern".to_string(), 1);
        let (score, matched) = score_document(&doc, &terms, &df, 1);
        assert!(score > 0.0);
        assert_eq!(matched.len(), 2);
        assert!(matched.contains(&"scaffold".to_string()));
        assert!(matched.contains(&"pattern".to_string()));
    }

    #[test]
    fn score_document_uses_tags_field_when_present() {
        let doc = Document {
            path: PathBuf::from("a.md"),
            title: "unrelated".to_string(),
            description: "unrelated".to_string(),
            tags: vec!["scaffold".to_string(), "pattern".to_string()],
            frontmatter_present: true,
            body: Some("unrelated body".to_string()),
        };
        let terms = vec!["scaffold".to_string()];
        let mut df = HashMap::new();
        df.insert("scaffold".to_string(), 1);
        let (score, matched) = score_document(&doc, &terms, &df, 1);
        assert!(score > 0.0);
        assert_eq!(matched, vec!["scaffold"]);
    }

    #[test]
    fn score_document_rare_term_outweighs_common_term() {
        let doc = Document {
            path: PathBuf::from("a.md"),
            title: "the rare".to_string(),
            description: "".to_string(),
            tags: Vec::new(),
            frontmatter_present: false,
            body: Some("the rare body content".to_string()),
        };
        let mut df = HashMap::new();
        df.insert("the".to_string(), 100);
        df.insert("rare".to_string(), 1);
        let (score_rare, _) = score_document(&doc, &vec!["rare".to_string()], &df, 100);
        let (score_common, _) = score_document(&doc, &vec!["the".to_string()], &df, 100);
        assert!(score_rare > score_common);
    }

    #[test]
    fn truncate_chars_appends_ellipsis_when_over_limit() {
        let s = "a".repeat(20);
        let t = truncate_chars(&s, 10);
        assert!(t.ends_with('…'));
        assert!(t.chars().count() <= 11);
    }

    #[test]
    fn truncate_chars_returns_input_when_under_limit() {
        let s = "short";
        let t = truncate_chars(s, 100);
        assert_eq!(t, "short");
    }

    #[test]
    fn walk_markdown_finds_files_recursively() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("a.md"), "# A\n").unwrap();
        fs::create_dir_all(root.join("sub")).unwrap();
        fs::write(root.join("sub/b.md"), "# B\n").unwrap();
        fs::write(root.join("sub/c.txt"), "ignored\n").unwrap();
        let files = walk_markdown(root).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().any(|p| p.ends_with("a.md")));
        assert!(files.iter().any(|p| p.ends_with("sub/b.md")));
    }

    #[test]
    fn walk_markdown_returns_sorted() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("b.md"), "# B\n").unwrap();
        fs::write(root.join("a.md"), "# A\n").unwrap();
        fs::write(root.join("c.md"), "# C\n").unwrap();
        let files = walk_markdown(root).unwrap();
        let names: Vec<_> = files
            .iter()
            .filter_map(|p| p.file_name().and_then(|f| f.to_str()))
            .collect();
        assert_eq!(names, vec!["a.md", "b.md", "c.md"]);
    }

    #[test]
    fn parse_document_extracts_title_and_description() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("note.md");
        fs::write(
            &path,
            "# Note Title\n\nThis is the description.\n\nThe rest of the body.\n",
        )
        .unwrap();
        let doc = parse_document(&path).unwrap();
        assert_eq!(doc.title, "Note Title");
        assert_eq!(doc.description, "This is the description.");
        assert!(doc.body.as_ref().unwrap().contains("rest of the body"));
        assert!(!doc.frontmatter_present);
    }

    #[test]
    fn parse_document_falls_back_to_filename_when_no_h1() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("orphan.md");
        fs::write(&path, "No heading here.\nJust body.\n").unwrap();
        let doc = parse_document(&path).unwrap();
        assert_eq!(doc.title, "orphan");
        assert!(doc.description.is_empty());
    }

    #[test]
    fn parse_document_rejects_empty_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("empty.md");
        fs::write(&path, "").unwrap();
        let result = parse_document(&path);
        assert!(result.is_err());
        let (kind, _) = result.unwrap_err();
        assert_eq!(kind, WarnKind::FileEmpty);
    }

    #[test]
    fn parse_document_reads_frontmatter_title_and_description() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fm.md");
        fs::write(
            &path,
            "---\ntitle: Frontmatter Title\ndescription: From the frontmatter\n---\n\n# H1 Body Title\n\nBody description prose.\n",
        )
        .unwrap();
        let doc = parse_document(&path).unwrap();
        assert!(doc.frontmatter_present);
        assert_eq!(doc.title, "Frontmatter Title");
        assert_eq!(doc.description, "From the frontmatter");
    }

    #[test]
    fn parse_document_falls_back_to_h1_when_frontmatter_lacks_title() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fm.md");
        fs::write(
            &path,
            "---\ntags: a, b\n---\n\n# H1 Wins\n\nFirst paragraph.\n",
        )
        .unwrap();
        let doc = parse_document(&path).unwrap();
        assert!(doc.frontmatter_present);
        assert_eq!(doc.title, "H1 Wins");
        assert_eq!(doc.description, "First paragraph.");
        assert_eq!(doc.tags, vec!["a", "b"]);
    }

    #[test]
    fn parse_document_supports_bracketed_tags_list() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fm.md");
        fs::write(
            &path,
            "---\ntags: [\"alpha\", 'beta', gamma]\n---\n\n# Title\n\nProse.\n",
        )
        .unwrap();
        let doc = parse_document(&path).unwrap();
        assert_eq!(doc.tags, vec!["alpha", "beta", "gamma"]);
    }

    #[test]
    fn parse_document_warns_on_unclosed_frontmatter() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fm.md");
        fs::write(&path, "---\ntitle: never closes\nno trailing marker\n").unwrap();
        let result = parse_document(&path);
        assert!(result.is_err());
        let (kind, _) = result.unwrap_err();
        assert_eq!(kind, WarnKind::FrontmatterMalformed);
    }

    #[test]
    fn parse_document_quoted_frontmatter_values_are_unwrapped() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("fm.md");
        fs::write(
            &path,
            "---\ntitle: \"Quoted Title\"\ndescription: 'Single quoted'\n---\n# Body\n",
        )
        .unwrap();
        let doc = parse_document(&path).unwrap();
        assert_eq!(doc.title, "Quoted Title");
        assert_eq!(doc.description, "Single quoted");
    }

    #[test]
    fn parse_tags_handles_comma_separated_and_brackets() {
        assert_eq!(parse_tags("a, b, c"), vec!["a", "b", "c"]);
        assert_eq!(parse_tags("[x, y]"), vec!["x", "y"]);
        assert_eq!(parse_tags("  single  "), vec!["single"]);
        assert!(parse_tags("").is_empty());
    }

    #[test]
    fn compute_doc_frequencies_counts_each_document_once_per_term() {
        let docs = vec![
            Document {
                path: PathBuf::from("a.md"),
                title: "foo foo foo".to_string(),
                description: "".to_string(),
                tags: Vec::new(),
                frontmatter_present: false,
                body: Some("# foo foo foo\nfoo\n".to_string()),
            },
            Document {
                path: PathBuf::from("b.md"),
                title: "foo".to_string(),
                description: "".to_string(),
                tags: Vec::new(),
                frontmatter_present: false,
                body: Some("# foo\nbar\n".to_string()),
            },
            Document {
                path: PathBuf::from("c.md"),
                title: "bar".to_string(),
                description: "".to_string(),
                tags: Vec::new(),
                frontmatter_present: false,
                body: Some("# bar\nbaz\n".to_string()),
            },
        ];
        let terms = vec!["foo".to_string(), "bar".to_string()];
        let df = compute_doc_frequencies(&docs, &terms);
        assert_eq!(df["foo"], 2);
        assert_eq!(df["bar"], 2);
    }

    #[test]
    fn run_returns_error_on_empty_query() {
        let args = args_for(vec![PathBuf::from(".")], "   ");
        let err = run(&args).unwrap_err();
        assert!(err.contains("empty"));
    }

    #[test]
    fn run_returns_error_on_query_with_only_punctuation() {
        let args = args_for(vec![PathBuf::from(".")], ",,, !!!");
        let err = run(&args).unwrap_err();
        assert!(err.contains("term"));
    }

    #[test]
    fn run_warns_on_missing_corpus_path() {
        let args = args_for(
            vec![PathBuf::from("/nonexistent/path/that/does/not/exist")],
            "anything",
        );
        let report = run(&args).unwrap();
        assert_eq!(report.documents_indexed, 0);
        assert!(report.notes.iter().any(|n| n.starts_with("warn[")));
        assert!(report.notes.iter().any(|n| n.contains("does not exist")));
        assert!(report
            .warnings_by_kind
            .contains_key(WarnKind::CorpusMissing.slug()));
    }

    #[test]
    fn run_warns_on_corpus_path_that_is_a_file() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("file.md");
        fs::write(&file_path, "# F\n").unwrap();
        let args = args_for(vec![file_path], "f");
        let report = run(&args).unwrap();
        assert!(report
            .warnings_by_kind
            .contains_key(WarnKind::CorpusNotDir.slug()));
    }

    #[test]
    fn run_ranks_higher_when_title_matches() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(
            root.join("title-match.md"),
            "# scaffold pattern\n\nIrrelevant body.\n",
        )
        .unwrap();
        fs::write(
            root.join("body-only.md"),
            "# Other title\n\nMentions scaffold once.\n",
        )
        .unwrap();
        let args = args_for(vec![root.to_path_buf()], "scaffold");
        let report = run(&args).unwrap();
        assert_eq!(report.results.len(), 2);
        assert!(report.results[0].path.ends_with("title-match.md"));
        assert!(report.results[0].score > report.results[1].score);
    }

    #[test]
    fn run_filters_by_min_score() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(
            root.join("strong.md"),
            "# scaffold scaffold scaffold\n\nscaffold body has scaffold too.\n",
        )
        .unwrap();
        fs::write(root.join("weak.md"), "# Other\n\nA scaffold mention.\n").unwrap();
        let mut args = args_for(vec![root.to_path_buf()], "scaffold");
        args.min_score = 5.0;
        let report = run(&args).unwrap();
        assert_eq!(report.results.len(), 1);
        assert!(report.results[0].path.ends_with("strong.md"));
    }

    #[test]
    fn run_caps_results_at_top_k() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        for i in 0..5 {
            fs::write(
                root.join(format!("doc-{i}.md")),
                format!("# scaffold doc {i}\n\nMention.\n"),
            )
            .unwrap();
        }
        let mut args = args_for(vec![root.to_path_buf()], "scaffold");
        args.top_k = 2;
        let report = run(&args).unwrap();
        assert_eq!(report.results.len(), 2);
        assert_eq!(report.documents_indexed, 5);
    }

    #[test]
    fn run_skips_empty_files_and_warns() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("good.md"), "# good\n\ngood body\n").unwrap();
        fs::write(root.join("empty.md"), "").unwrap();
        let args = args_for(vec![root.to_path_buf()], "good");
        let report = run(&args).unwrap();
        assert_eq!(report.documents_indexed, 1);
        assert_eq!(report.documents_skipped, 1);
        assert!(report.notes.iter().any(|n| n.contains("empty.md")));
        assert!(report
            .warnings_by_kind
            .contains_key(WarnKind::FileEmpty.slug()));
    }

    #[test]
    fn run_handles_multiple_corpus_roots() {
        let dir = tempfile::tempdir().unwrap();
        let root_a = dir.path().join("a");
        let root_b = dir.path().join("b");
        fs::create_dir_all(&root_a).unwrap();
        fs::create_dir_all(&root_b).unwrap();
        fs::write(root_a.join("x.md"), "# foo\n\nfoo body\n").unwrap();
        fs::write(root_b.join("y.md"), "# foo\n\nfoo body\n").unwrap();
        let args = args_for(vec![root_a, root_b], "foo");
        let report = run(&args).unwrap();
        assert_eq!(report.documents_indexed, 2);
        assert_eq!(report.results.len(), 2);
    }

    #[test]
    fn run_includes_body_when_flag_set() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("a.md"), "# foo\n\nfoo body\n").unwrap();
        let mut args = args_for(vec![root.to_path_buf()], "foo");
        args.include_body = true;
        let report = run(&args).unwrap();
        assert_eq!(report.results.len(), 1);
        assert!(report.results[0].body.is_some());
        assert!(report.results[0]
            .body
            .as_ref()
            .unwrap()
            .contains("foo body"));
    }

    #[test]
    fn run_excludes_body_by_default() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("a.md"), "# foo\n\nfoo body\n").unwrap();
        let args = args_for(vec![root.to_path_buf()], "foo");
        let report = run(&args).unwrap();
        assert!(report.results[0].body.is_none());
    }

    #[test]
    fn run_emits_empty_deferred_stages_after_complete() {
        let dir = tempfile::tempdir().unwrap();
        let args = args_for(vec![dir.path().to_path_buf()], "anything");
        let report = run(&args).unwrap();
        assert!(report.deferred_stages.is_empty());
        assert_eq!(report.index_status, "no-cache");
    }

    #[test]
    fn run_with_index_cache_writes_then_reuses() {
        let corpus_dir = tempfile::tempdir().unwrap();
        let root = corpus_dir.path();
        fs::write(root.join("a.md"), "# foo\n\nfoo body\n").unwrap();
        let cache_dir = tempfile::tempdir().unwrap();
        let cache_path = cache_dir.path().join("idx.json");

        let mut args = args_for(vec![root.to_path_buf()], "foo");
        args.index_cache = Some(cache_path.clone());

        let r1 = run(&args).unwrap();
        assert_eq!(r1.index_status, "rebuilt-fresh");
        assert!(cache_path.exists());

        let r2 = run(&args).unwrap();
        assert_eq!(r2.index_status, "cached");
        assert_eq!(r2.documents_indexed, 1);
    }

    #[test]
    fn run_with_index_cache_rebuilds_when_file_changes() {
        let corpus_dir = tempfile::tempdir().unwrap();
        let root = corpus_dir.path();
        let file = root.join("a.md");
        fs::write(&file, "# foo\n\nfoo body\n").unwrap();
        let cache_dir = tempfile::tempdir().unwrap();
        let cache_path = cache_dir.path().join("idx.json");

        let mut args = args_for(vec![root.to_path_buf()], "foo");
        args.index_cache = Some(cache_path.clone());

        run(&args).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(1100));
        fs::write(
            &file,
            "# foo\n\nfoo body has more content now to change size\n",
        )
        .unwrap();

        let r = run(&args).unwrap();
        assert_eq!(r.index_status, "rebuilt-stale");
    }

    #[test]
    fn run_with_rebuild_index_forces_rebuild() {
        let corpus_dir = tempfile::tempdir().unwrap();
        let root = corpus_dir.path();
        fs::write(root.join("a.md"), "# foo\n\nfoo body\n").unwrap();
        let cache_dir = tempfile::tempdir().unwrap();
        let cache_path = cache_dir.path().join("idx.json");

        let mut args = args_for(vec![root.to_path_buf()], "foo");
        args.index_cache = Some(cache_path.clone());

        run(&args).unwrap();
        args.rebuild_index = true;
        let r = run(&args).unwrap();
        assert_eq!(r.index_status, "rebuilt-forced");
    }

    #[test]
    fn run_with_corrupt_cache_rebuilds_and_warns() {
        let corpus_dir = tempfile::tempdir().unwrap();
        let root = corpus_dir.path();
        fs::write(root.join("a.md"), "# foo\n\nfoo body\n").unwrap();
        let cache_dir = tempfile::tempdir().unwrap();
        let cache_path = cache_dir.path().join("idx.json");
        fs::write(&cache_path, "{not json").unwrap();

        let mut args = args_for(vec![root.to_path_buf()], "foo");
        args.index_cache = Some(cache_path);

        let r = run(&args).unwrap();
        assert_eq!(r.index_status, "rebuilt-corrupt");
        assert!(r
            .warnings_by_kind
            .contains_key(WarnKind::IndexCorrupt.slug()));
    }

    #[test]
    fn tally_warnings_groups_by_kind_slug() {
        let notes = vec![
            "warn[file-empty]: a".to_string(),
            "warn[file-empty]: b".to_string(),
            "warn[corpus-missing]: c".to_string(),
            "info: not a warning".to_string(),
        ];
        let tally = tally_warnings(&notes);
        assert_eq!(tally["file-empty"], 2);
        assert_eq!(tally["corpus-missing"], 1);
        assert_eq!(tally.len(), 2);
    }

    #[test]
    fn extract_frontmatter_returns_none_when_no_opener() {
        let (map, after, present, err) = extract_frontmatter("# H1\n\nbody\n");
        assert!(map.is_empty());
        assert!(after.starts_with("# H1"));
        assert!(!present);
        assert!(err.is_none());
    }

    #[test]
    fn extract_frontmatter_returns_after_section_after_close() {
        let raw = "---\ntitle: x\n---\n# H1\nbody\n";
        let (map, after, present, err) = extract_frontmatter(raw);
        assert_eq!(map.get("title").map(|s| s.as_str()), Some("x"));
        assert!(after.starts_with("# H1"));
        assert!(present);
        assert!(err.is_none());
    }

    #[test]
    fn extract_frontmatter_handles_bom_prefix() {
        let raw = "\u{feff}---\ntitle: bom\n---\n# H1\nbody\n";
        let (map, _after, present, err) = extract_frontmatter(raw);
        assert!(present);
        assert_eq!(map.get("title").map(|s| s.as_str()), Some("bom"));
        assert!(err.is_none());
    }
}
