use clap::{Parser, ValueEnum};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SCHEMA: &str = "v2-wiki-search/v1";
const DESCRIPTION_MAX_CHARS: usize = 500;
const TITLE_WEIGHT: f64 = 3.0;
const DESCRIPTION_WEIGHT: f64 = 2.0;
const BODY_WEIGHT: f64 = 1.0;

#[derive(Parser, Debug)]
#[command(
    name = "v2-wiki-search",
    about = "Top-k retrieval over _notes/*.md corpus (scaffold-partial; TF-IDF + indexing + frontmatter deferred to cycle 126)"
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
    deferred_stages: Vec<String>,
}

fn main() -> ExitCode {
    let args = Args::parse();
    match run(&args) {
        Ok(report) => {
            if let Err(e) = emit(&args, &report) {
                eprintln!("emit error: {e}");
                return ExitCode::from(2);
            }
            if args.strict && report.notes.iter().any(|n| n.starts_with("warn:")) {
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
    let mut documents: Vec<Document> = Vec::new();
    let mut documents_skipped = 0usize;

    for corpus_root in &args.corpus {
        if !corpus_root.exists() {
            notes.push(format!(
                "warn: corpus path does not exist: {}",
                corpus_root.display()
            ));
            continue;
        }
        if !corpus_root.is_dir() {
            notes.push(format!(
                "warn: corpus path is not a directory: {}",
                corpus_root.display()
            ));
            continue;
        }
        match walk_markdown(corpus_root) {
            Ok(paths) => {
                for path in paths {
                    match parse_document(&path) {
                        Ok(doc) => documents.push(doc),
                        Err(e) => {
                            notes.push(format!(
                                "warn: skipped {}: {}",
                                path.display(),
                                e
                            ));
                            documents_skipped += 1;
                        }
                    }
                }
            }
            Err(e) => {
                notes.push(format!(
                    "warn: failed to walk {}: {}",
                    corpus_root.display(),
                    e
                ));
            }
        }
    }

    let mut ranked: Vec<RankedResult> = documents
        .iter()
        .filter_map(|doc| {
            let (score, matched_terms) = score_document(doc, &query_terms);
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

    Ok(SearchReport {
        schema: SCHEMA.to_string(),
        query: args.query.clone(),
        query_terms,
        corpus_paths: args.corpus.clone(),
        documents_indexed: documents.len(),
        documents_skipped,
        top_k: args.top_k,
        min_score: args.min_score,
        results: ranked,
        notes,
        deferred_stages: vec![
            "tf-idf-ranking (current: simple weighted substring count)".to_string(),
            "index-caching (current: recompute on every invocation)".to_string(),
            "frontmatter-parsing (current: H1+first-paragraph description-surrogate)".to_string(),
            "corruption-handling-refinement (current: per-file warn-and-skip)".to_string(),
        ],
    })
}

fn tokenize_query(query: &str) -> Vec<String> {
    query
        .split_whitespace()
        .map(|t| t.trim_matches(|c: char| c.is_ascii_punctuation()).to_lowercase())
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
            } else if ft.is_file()
                && path.extension().and_then(|e| e.to_str()) == Some("md")
            {
                out.push(path);
            }
        }
    }
    out.sort();
    Ok(out)
}

fn parse_document(path: &Path) -> Result<Document, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("read: {e}"))?;
    if raw.trim().is_empty() {
        return Err("empty file".to_string());
    }

    let title = extract_title(&raw).unwrap_or_else(|| {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("untitled")
            .to_string()
    });
    let description = extract_description(&raw);

    Ok(Document {
        path: path.to_path_buf(),
        title,
        description,
        body: Some(raw),
    })
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

fn score_document(doc: &Document, terms: &[String]) -> (f64, Vec<String>) {
    let title_lc = doc.title.to_lowercase();
    let desc_lc = doc.description.to_lowercase();
    let body_lc = doc
        .body
        .as_ref()
        .map(|b| b.to_lowercase())
        .unwrap_or_default();

    let mut score = 0.0f64;
    let mut matched: Vec<String> = Vec::new();

    for term in terms {
        let in_title = count_substring(&title_lc, term) as f64;
        let in_desc = count_substring(&desc_lc, term) as f64;
        let in_body = count_substring(&body_lc, term) as f64;
        let term_score = in_title * TITLE_WEIGHT
            + in_desc * DESCRIPTION_WEIGHT
            + (in_body - in_title - in_desc).max(0.0) * BODY_WEIGHT;
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
        "Corpus: {} path(s); {} document(s) indexed; {} skipped\n",
        report.corpus_paths.len(),
        report.documents_indexed,
        report.documents_skipped
    ));
    out.push_str(&format!(
        "Query terms: {}\n",
        report.query_terms.join(", ")
    ));
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
            out.push_str(&format!(
                "   matched: {}\n\n",
                r.matched_terms.join(", ")
            ));
        }
    }

    if !report.notes.is_empty() {
        out.push_str(&format!("## Notes ({})\n\n", report.notes.len()));
        for n in &report.notes {
            out.push_str(&format!("- {n}\n"));
        }
        out.push('\n');
    }

    if !report.deferred_stages.is_empty() {
        out.push_str("## Deferred (scaffold-partial)\n\n");
        for d in &report.deferred_stages {
            out.push_str(&format!("- {d}\n"));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn score_document_weights_title_over_body() {
        let doc = Document {
            path: PathBuf::from("a.md"),
            title: "foo".to_string(),
            description: "".to_string(),
            body: Some("# foo\nsomething else\n".to_string()),
        };
        let terms = vec!["foo".to_string()];
        let (score, matched) = score_document(&doc, &terms);
        assert!(score >= TITLE_WEIGHT);
        assert_eq!(matched, vec!["foo"]);
    }

    #[test]
    fn score_document_returns_zero_for_unmatched() {
        let doc = Document {
            path: PathBuf::from("a.md"),
            title: "foo".to_string(),
            description: "bar".to_string(),
            body: Some("baz".to_string()),
        };
        let terms = vec!["xyz".to_string()];
        let (score, matched) = score_document(&doc, &terms);
        assert_eq!(score, 0.0);
        assert!(matched.is_empty());
    }

    #[test]
    fn score_document_accumulates_across_terms() {
        let doc = Document {
            path: PathBuf::from("a.md"),
            title: "scaffold-partial pattern".to_string(),
            description: "scaffold and pattern words".to_string(),
            body: Some("body has scaffold once".to_string()),
        };
        let terms = vec!["scaffold".to_string(), "pattern".to_string()];
        let (score, matched) = score_document(&doc, &terms);
        assert!(score > 0.0);
        assert_eq!(matched.len(), 2);
        assert!(matched.contains(&"scaffold".to_string()));
        assert!(matched.contains(&"pattern".to_string()));
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
    }

    #[test]
    fn run_returns_error_on_empty_query() {
        let args = Args {
            corpus: vec![PathBuf::from(".")],
            query: "   ".to_string(),
            top_k: 10,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: false,
            min_score: 0.0,
            strict: false,
        };
        let err = run(&args).unwrap_err();
        assert!(err.contains("empty"));
    }

    #[test]
    fn run_returns_error_on_query_with_only_punctuation() {
        let args = Args {
            corpus: vec![PathBuf::from(".")],
            query: ",,, !!!".to_string(),
            top_k: 10,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: false,
            min_score: 0.0,
            strict: false,
        };
        let err = run(&args).unwrap_err();
        assert!(err.contains("term"));
    }

    #[test]
    fn run_warns_on_missing_corpus_path() {
        let args = Args {
            corpus: vec![PathBuf::from("/nonexistent/path/that/does/not/exist")],
            query: "anything".to_string(),
            top_k: 10,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: false,
            min_score: 0.0,
            strict: false,
        };
        let report = run(&args).unwrap();
        assert_eq!(report.documents_indexed, 0);
        assert!(report.notes.iter().any(|n| n.starts_with("warn:")));
        assert!(report.notes.iter().any(|n| n.contains("does not exist")));
    }

    #[test]
    fn run_warns_on_corpus_path_that_is_a_file() {
        let dir = tempfile::tempdir().unwrap();
        let file_path = dir.path().join("file.md");
        fs::write(&file_path, "# F\n").unwrap();
        let args = Args {
            corpus: vec![file_path],
            query: "f".to_string(),
            top_k: 10,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: false,
            min_score: 0.0,
            strict: false,
        };
        let report = run(&args).unwrap();
        assert!(report.notes.iter().any(|n| n.contains("not a directory")));
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
        let args = Args {
            corpus: vec![root.to_path_buf()],
            query: "scaffold".to_string(),
            top_k: 10,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: false,
            min_score: 0.0,
            strict: false,
        };
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
        let args = Args {
            corpus: vec![root.to_path_buf()],
            query: "scaffold".to_string(),
            top_k: 10,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: false,
            min_score: 5.0,
            strict: false,
        };
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
        let args = Args {
            corpus: vec![root.to_path_buf()],
            query: "scaffold".to_string(),
            top_k: 2,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: false,
            min_score: 0.0,
            strict: false,
        };
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
        let args = Args {
            corpus: vec![root.to_path_buf()],
            query: "good".to_string(),
            top_k: 10,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: false,
            min_score: 0.0,
            strict: false,
        };
        let report = run(&args).unwrap();
        assert_eq!(report.documents_indexed, 1);
        assert_eq!(report.documents_skipped, 1);
        assert!(report.notes.iter().any(|n| n.contains("empty.md")));
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
        let args = Args {
            corpus: vec![root_a, root_b],
            query: "foo".to_string(),
            top_k: 10,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: false,
            min_score: 0.0,
            strict: false,
        };
        let report = run(&args).unwrap();
        assert_eq!(report.documents_indexed, 2);
        assert_eq!(report.results.len(), 2);
    }

    #[test]
    fn run_includes_body_when_flag_set() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("a.md"), "# foo\n\nfoo body\n").unwrap();
        let args = Args {
            corpus: vec![root.to_path_buf()],
            query: "foo".to_string(),
            top_k: 10,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: true,
            min_score: 0.0,
            strict: false,
        };
        let report = run(&args).unwrap();
        assert_eq!(report.results.len(), 1);
        assert!(report.results[0].body.is_some());
        assert!(report.results[0].body.as_ref().unwrap().contains("foo body"));
    }

    #[test]
    fn run_excludes_body_by_default() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path();
        fs::write(root.join("a.md"), "# foo\n\nfoo body\n").unwrap();
        let args = Args {
            corpus: vec![root.to_path_buf()],
            query: "foo".to_string(),
            top_k: 10,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: false,
            min_score: 0.0,
            strict: false,
        };
        let report = run(&args).unwrap();
        assert!(report.results[0].body.is_none());
    }

    #[test]
    fn run_emits_deferred_stages_list() {
        let dir = tempfile::tempdir().unwrap();
        let args = Args {
            corpus: vec![dir.path().to_path_buf()],
            query: "anything".to_string(),
            top_k: 10,
            format: OutputFormat::Json,
            output: "-".to_string(),
            include_body: false,
            min_score: 0.0,
            strict: false,
        };
        let report = run(&args).unwrap();
        assert_eq!(report.deferred_stages.len(), 4);
        assert!(report
            .deferred_stages
            .iter()
            .any(|d| d.contains("tf-idf-ranking")));
        assert!(report
            .deferred_stages
            .iter()
            .any(|d| d.contains("index-caching")));
        assert!(report
            .deferred_stages
            .iter()
            .any(|d| d.contains("frontmatter-parsing")));
    }
}
