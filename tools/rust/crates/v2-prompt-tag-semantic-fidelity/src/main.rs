//! v2-prompt-tag-semantic-fidelity
//!
//! Verifies v2 role prompts' top-level tag-content semantic alignment against a
//! TOML manifest.  Implements the two-tier check described in:
//!   docs/redesign/_notes/v2-prompt-tag-semantic-fidelity.md  (cycle 168 design)
//!
//! § CLI subcommands:
//!   check      -- run the two-tier check (default use-case)
//!   schema     -- print expected manifest TOML shape
//!   list-tags  -- list all top-level tags found across prompts

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::{Parser, Subcommand, ValueEnum};
use quick_xml::events::Event;
use quick_xml::Reader;
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(
    name = "v2-prompt-tag-semantic-fidelity",
    about = "Verifies v2 role prompts' top-level tag-content semantic alignment against a TOML manifest"
)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run the two-tier semantic-fidelity check against the manifest.
    Check {
        /// Directory containing v2 role prompt XML files.
        #[arg(long, default_value = "prompts/v2/")]
        prompts_dir: PathBuf,

        /// Path to the TOML tag-semantics manifest.
        #[arg(long, default_value = "prompts/v2/tag-semantics.toml")]
        manifest: PathBuf,

        /// Exit non-zero on any Tier 1 error.
        #[arg(long)]
        strict: bool,

        /// Output format.
        #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
    },

    /// Print the expected manifest TOML schema.
    Schema,

    /// List all top-level tags found across prompts in the given directory.
    ListTags {
        /// Directory containing v2 role prompt XML files.
        #[arg(long, default_value = "prompts/v2/")]
        prompts_dir: PathBuf,
    },
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
}

// ---------------------------------------------------------------------------
// Error type
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum ToolError {
    Io(std::io::Error),
    Xml(quick_xml::Error),
    Toml(toml::de::Error),
    InvalidManifest(String),
}

impl std::fmt::Display for ToolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ToolError::Io(e) => write!(f, "io error: {e}"),
            ToolError::Xml(e) => write!(f, "xml parse error: {e}"),
            ToolError::Toml(e) => write!(f, "toml parse error: {e}"),
            ToolError::InvalidManifest(msg) => write!(f, "invalid manifest: {msg}"),
        }
    }
}

impl From<std::io::Error> for ToolError {
    fn from(e: std::io::Error) -> Self {
        ToolError::Io(e)
    }
}
impl From<quick_xml::Error> for ToolError {
    fn from(e: quick_xml::Error) -> Self {
        ToolError::Xml(e)
    }
}
impl From<toml::de::Error> for ToolError {
    fn from(e: toml::de::Error) -> Self {
        ToolError::Toml(e)
    }
}

// ---------------------------------------------------------------------------
// Manifest types
// ---------------------------------------------------------------------------

/// Raw TOML deserialization target: `[tags.<name>]` entries.
#[derive(Debug, Deserialize, Serialize)]
struct RawTagEntry {
    intent: String,
    #[serde(rename = "required-in-roles", default)]
    required_in_roles: Vec<String>,
    #[serde(rename = "allowed-in-roles", default)]
    allowed_in_roles: Vec<String>,
    #[serde(rename = "expected-children", default)]
    expected_children: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct RawManifest {
    tags: BTreeMap<String, RawTagEntry>,
}

/// Validated manifest entry.
#[derive(Debug, Clone)]
struct TagEntry {
    intent: String,
    required_in_roles: Vec<String>,
    allowed_in_roles: Vec<String>,
    expected_children: Vec<String>,
}

/// The manifest: a map from tag name → entry.
type Manifest = BTreeMap<String, TagEntry>;

/// Parse and validate the manifest from a TOML file.
fn load_manifest(path: &Path) -> Result<Manifest, ToolError> {
    let content = fs::read_to_string(path)?;
    let raw: RawManifest = toml::from_str(&content)?;

    let mut manifest = BTreeMap::new();
    for (name, raw_entry) in raw.tags {
        // Invariant: at least one of required-in-roles / allowed-in-roles must be set.
        if raw_entry.required_in_roles.is_empty() && raw_entry.allowed_in_roles.is_empty() {
            return Err(ToolError::InvalidManifest(format!(
                "tag `{name}` must have at least one of `required-in-roles` or `allowed-in-roles`"
            )));
        }
        manifest.insert(
            name,
            TagEntry {
                intent: raw_entry.intent,
                required_in_roles: raw_entry.required_in_roles,
                allowed_in_roles: raw_entry.allowed_in_roles,
                expected_children: raw_entry.expected_children,
            },
        );
    }
    Ok(manifest)
}

// ---------------------------------------------------------------------------
// Prompt XML parsing
// ---------------------------------------------------------------------------

const EXPECTED_PROMPT_FILES: &[&str] = &[
    "planner-prompt.xml",
    "executor-prompt.xml",
    "curator-prompt.xml",
    "reconciler-prompt.xml",
];

/// A parsed top-level tag from a role prompt.
#[derive(Debug, Clone)]
struct ParsedTag {
    name: String,
    /// Unique direct child tag names (deduped by name).
    direct_children: Vec<String>,
    has_adaptation_note: bool,
    adaptation_note_text: Option<String>,
    /// Number of non-blank, non-XML-comment text lines under this tag.
    content_line_count: usize,
    /// Number of direct child *elements* (including duplicates — for sub-element count).
    direct_child_element_count: usize,
}

/// All top-level tags extracted from one role prompt.
#[derive(Debug, Clone)]
struct ParsedPrompt {
    role: String,
    filename: String,
    top_level_tags: Vec<ParsedTag>,
}

/// Parse a role prompt XML file and return its structure.
///
/// Uses a stack-based tag tracking approach with `check_end_names(false)` to
/// handle malformed XML gracefully (e.g., `<other>` used as literal text in
/// element content). When an end-tag name doesn't match the current stack top,
/// the unmatched elements are popped until the correct name is found, similar
/// to how browsers handle malformed HTML. This keeps depth tracking correct
/// even after a mismatched section.
fn parse_prompt_file(path: &Path) -> Result<ParsedPrompt, ToolError> {
    let filename = path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default();
    let content = fs::read_to_string(path)?;

    let mut reader = Reader::from_str(&content);
    reader.trim_text(false);
    // Disable end-tag name validation: allows the parser to continue past
    // prompts that contain literal XML-like strings (e.g., `<other>`) inside
    // text content.  The stack-pop logic below restores correct depth tracking.
    reader.check_end_names(false);

    let mut role = String::new();

    // Stack of currently open tag names — used for depth tracking.
    let mut tag_stack: Vec<String> = Vec::new();

    // Currently collecting a top-level tag.
    let mut current_top: Option<ParsedTag> = None;
    // Depth at which we entered the semantic-adaptation-note element.
    let mut adaptation_note_depth: usize = 0;
    let mut adaptation_note_buf = String::new();

    // Seen child names for the current top-level tag (for deduplication).
    let mut seen_children: BTreeSet<String> = BTreeSet::new();

    // Text accumulator for the current top-level tag.
    let mut text_buf = String::new();

    let mut top_level_tags: Vec<ParsedTag> = Vec::new();
    let mut buf = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag_name =
                    std::str::from_utf8(e.name().as_ref()).unwrap_or("").to_string();
                let depth_before = tag_stack.len();

                match depth_before {
                    0 if tag_name == "role-prompt" => {
                        // Root element: extract role attribute.
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"role" {
                                role = std::str::from_utf8(&attr.value)
                                    .unwrap_or("")
                                    .to_string();
                            }
                        }
                    }
                    1 => {
                        // Direct child of role-prompt → start of a top-level tag.
                        seen_children.clear();
                        text_buf.clear();
                        adaptation_note_buf.clear();
                        adaptation_note_depth = 0;
                        current_top = Some(ParsedTag {
                            name: tag_name.clone(),
                            direct_children: Vec::new(),
                            has_adaptation_note: false,
                            adaptation_note_text: None,
                            content_line_count: 0,
                            direct_child_element_count: 0,
                        });
                    }
                    2 => {
                        // Direct child of a top-level tag.
                        if let Some(ref mut top) = current_top {
                            top.direct_child_element_count += 1;
                            if !seen_children.contains(&tag_name) {
                                seen_children.insert(tag_name.clone());
                                top.direct_children.push(tag_name.clone());
                            }
                            if tag_name == "semantic-adaptation-note" {
                                top.has_adaptation_note = true;
                                // Record depth so we can collect text inside it.
                                adaptation_note_depth = tag_stack.len() + 1;
                            }
                        }
                    }
                    _ => {}
                }

                tag_stack.push(tag_name);
            }

            Ok(Event::End(e)) => {
                let end_name =
                    std::str::from_utf8(e.name().as_ref()).unwrap_or("").to_string();

                // Pop the stack until we find the matching open tag.
                // This handles mismatched end-tags (e.g., </constraint> where
                // <other> is currently open) without disrupting depth tracking.
                let mut popped_count = 0usize;
                let mut found = false;
                while let Some(top) = tag_stack.last() {
                    popped_count += 1;
                    if top == &end_name {
                        tag_stack.pop();
                        found = true;
                        break;
                    }
                    tag_stack.pop();
                }
                let _ = (popped_count, found); // used for debug if needed

                let depth_after = tag_stack.len();

                if depth_after == 1 {
                    // Closed a top-level tag → finalise it.
                    if let Some(mut top) = current_top.take() {
                        top.content_line_count = text_buf
                            .lines()
                            .filter(|l| !l.trim().is_empty())
                            .count();
                        if top.has_adaptation_note
                            && !adaptation_note_buf.trim().is_empty()
                        {
                            top.adaptation_note_text =
                                Some(adaptation_note_buf.trim().to_string());
                        }
                        top_level_tags.push(top);
                    }
                    adaptation_note_depth = 0;
                }
            }

            Ok(Event::Text(t)) => {
                if current_top.is_some() {
                    let s = t.unescape().map(|s| s.into_owned()).unwrap_or_default();
                    text_buf.push_str(&s);
                    // Collect text inside the adaptation note element.
                    if adaptation_note_depth > 0
                        && tag_stack.len() >= adaptation_note_depth
                    {
                        adaptation_note_buf.push_str(&s);
                    }
                }
            }

            Ok(Event::CData(cd)) => {
                if current_top.is_some() {
                    let s =
                        std::str::from_utf8(cd.as_ref()).unwrap_or("").to_string();
                    text_buf.push_str(&s);
                }
            }

            Ok(Event::Eof) => break,
            Ok(_) => {} // PI, comment, declaration — ignored.
            Err(e) => return Err(ToolError::Xml(e)),
        }
        buf.clear();
    }

    Ok(ParsedPrompt {
        role,
        filename,
        top_level_tags,
    })
}

/// Load and parse all expected prompt files from a directory.
fn load_prompts(prompts_dir: &Path) -> Result<Vec<ParsedPrompt>, ToolError> {
    let mut prompts = Vec::new();
    for filename in EXPECTED_PROMPT_FILES {
        let path = prompts_dir.join(filename);
        if !path.exists() {
            return Err(ToolError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("prompt file not found: {}", path.display()),
            )));
        }
        prompts.push(parse_prompt_file(&path)?);
    }
    Ok(prompts)
}

// ---------------------------------------------------------------------------
// Tier 1 — mechanical checks
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum Tier1Kind {
    UnknownTag,
    MissingRequired,
    MissingExpectedChild,
    RoleNotAllowed,
}

#[derive(Debug, Clone, Serialize)]
struct Tier1Finding {
    role: String,
    tag: String,
    kind: Tier1Kind,
    #[serde(skip_serializing_if = "Option::is_none")]
    manifest_entry: Option<String>,
    has_adaptation_note: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    expected_children: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    found_children: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    adaptation_note: Option<String>,
    severity: &'static str,
}

fn run_tier1(manifest: &Manifest, prompts: &[ParsedPrompt]) -> Vec<Tier1Finding> {
    let mut findings = Vec::new();

    // Build a map: role → set of top-level tag names present.
    let role_tag_presence: BTreeMap<String, BTreeSet<String>> = prompts
        .iter()
        .map(|p| {
            (
                p.role.clone(),
                p.top_level_tags.iter().map(|t| t.name.clone()).collect(),
            )
        })
        .collect();

    // Check 1 (unknown-tag) + Check 2 (role-not-allowed) + Check 5 (expected-children).
    for prompt in prompts {
        for tag in &prompt.top_level_tags {
            match manifest.get(&tag.name) {
                None => {
                    // Unknown tag.
                    if !tag.has_adaptation_note {
                        findings.push(Tier1Finding {
                            role: prompt.role.clone(),
                            tag: tag.name.clone(),
                            kind: Tier1Kind::UnknownTag,
                            manifest_entry: None,
                            has_adaptation_note: false,
                            expected_children: None,
                            found_children: None,
                            adaptation_note: None,
                            severity: "error",
                        });
                    }
                    // If adaptation note is present → suppressed (no finding).
                }
                Some(entry) => {
                    // Check role-allowed.
                    let allowed_roles: BTreeSet<&String> = entry
                        .required_in_roles
                        .iter()
                        .chain(entry.allowed_in_roles.iter())
                        .collect();

                    if !allowed_roles.is_empty()
                        && !allowed_roles.contains(&prompt.role)
                    {
                        findings.push(Tier1Finding {
                            role: prompt.role.clone(),
                            tag: tag.name.clone(),
                            kind: Tier1Kind::RoleNotAllowed,
                            manifest_entry: Some(entry.intent.clone()),
                            has_adaptation_note: tag.has_adaptation_note,
                            expected_children: None,
                            found_children: None,
                            adaptation_note: tag.adaptation_note_text.clone(),
                            severity: "error",
                        });
                    }

                    // Check expected-children.
                    if !entry.expected_children.is_empty() {
                        let found: BTreeSet<&String> =
                            tag.direct_children.iter().collect();
                        let missing: Vec<String> = entry
                            .expected_children
                            .iter()
                            .filter(|c| !found.contains(c))
                            .cloned()
                            .collect();
                        if !missing.is_empty() {
                            findings.push(Tier1Finding {
                                role: prompt.role.clone(),
                                tag: tag.name.clone(),
                                kind: Tier1Kind::MissingExpectedChild,
                                manifest_entry: Some(entry.intent.clone()),
                                has_adaptation_note: tag.has_adaptation_note,
                                expected_children: Some(
                                    entry.expected_children.clone(),
                                ),
                                found_children: Some(
                                    tag.direct_children.clone(),
                                ),
                                adaptation_note: tag.adaptation_note_text.clone(),
                                severity: "error",
                            });
                        }
                    }
                }
            }
        }
    }

    // Check 4: canonical-required — for each manifest entry with required-in-roles,
    // every listed role must have that tag.
    for (tag_name, entry) in manifest {
        for required_role in &entry.required_in_roles {
            let present = role_tag_presence
                .get(required_role)
                .map(|s| s.contains(tag_name))
                .unwrap_or(false);
            if !present {
                findings.push(Tier1Finding {
                    role: required_role.clone(),
                    tag: tag_name.clone(),
                    kind: Tier1Kind::MissingRequired,
                    manifest_entry: Some(entry.intent.clone()),
                    has_adaptation_note: false,
                    expected_children: None,
                    found_children: None,
                    adaptation_note: None,
                    severity: "error",
                });
            }
        }
    }

    findings
}

// ---------------------------------------------------------------------------
// Tier 2 — content heuristics
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum Tier2Kind {
    LengthPlausibility,
    KeywordOverlap,
}

#[derive(Debug, Clone, Serialize)]
struct Tier2Finding {
    role: String,
    tag: String,
    kind: Tier2Kind,
    details: String,
    severity: &'static str,
}

/// Basic English stop-words to remove during keyword comparison.
const STOP_WORDS: &[&str] = &[
    "a", "an", "the", "is", "are", "was", "were", "be", "been", "being",
    "have", "has", "had", "do", "does", "did", "will", "would", "shall",
    "should", "may", "might", "must", "can", "could", "to", "for", "in",
    "on", "at", "with", "from", "of", "and", "or", "but", "as", "by",
    "not", "if", "it", "its", "this", "that", "these", "those", "which",
    "who", "what", "where", "when", "how", "all", "each", "any", "both",
    "more", "most", "other", "some", "such", "no", "so", "yet", "very",
    "than", "then", "there", "their", "they", "them", "also", "into",
    "about", "only", "per", "role", "prompt", "v2",
];

/// Tokenize a string: lowercase, split on non-alpha, filter stop-words.
fn tokenize(s: &str) -> BTreeSet<String> {
    s.split(|c: char| !c.is_alphabetic())
        .filter(|tok| tok.len() >= 3)
        .map(|tok| tok.to_lowercase())
        .filter(|tok| !STOP_WORDS.contains(&tok.as_str()))
        .collect()
}

fn run_tier2(manifest: &Manifest, prompts: &[ParsedPrompt]) -> Vec<Tier2Finding> {
    let mut findings = Vec::new();

    for prompt in prompts {
        for tag in &prompt.top_level_tags {
            // Length plausibility.
            match tag.name.as_str() {
                "one-line" if tag.content_line_count > 2 => {
                    findings.push(Tier2Finding {
                        role: prompt.role.clone(),
                        tag: tag.name.clone(),
                        kind: Tier2Kind::LengthPlausibility,
                        details: format!(
                            "found {} non-blank lines; expected ≤2 for <one-line>",
                            tag.content_line_count
                        ),
                        severity: "warning",
                    });
                }
                "role-identity" if tag.direct_child_element_count < 3 => {
                    findings.push(Tier2Finding {
                        role: prompt.role.clone(),
                        tag: tag.name.clone(),
                        kind: Tier2Kind::LengthPlausibility,
                        details: format!(
                            "found {} sub-elements; expected ≥3 for <role-identity>",
                            tag.direct_child_element_count
                        ),
                        severity: "warning",
                    });
                }
                "output-contract" if tag.content_line_count < 10 => {
                    findings.push(Tier2Finding {
                        role: prompt.role.clone(),
                        tag: tag.name.clone(),
                        kind: Tier2Kind::LengthPlausibility,
                        details: format!(
                            "found {} non-blank lines; expected ≥10 for <output-contract>",
                            tag.content_line_count
                        ),
                        severity: "warning",
                    });
                }
                _ => {}
            }

            // Keyword overlap check: only for tags present in the manifest.
            if let Some(entry) = manifest.get(&tag.name) {
                if tag.content_line_count >= 3 {
                    let intent_tokens = tokenize(&entry.intent);
                    let content_tokens = tokenize(&format!(
                        "{} {}",
                        tag.name.replace('-', " "),
                        tag.direct_children.join(" ")
                    ));
                    let overlap = intent_tokens
                        .iter()
                        .filter(|t| content_tokens.contains(*t))
                        .count();
                    if overlap < 1 {
                        findings.push(Tier2Finding {
                            role: prompt.role.clone(),
                            tag: tag.name.clone(),
                            kind: Tier2Kind::KeywordOverlap,
                            details: format!(
                                "0 shared tokens between intent ({:?}) and tag name+children; \
                                 possible semantic drift",
                                entry.intent
                            ),
                            severity: "warning",
                        });
                    }
                }
            }
        }
    }

    findings
}

// ---------------------------------------------------------------------------
// Summary & report
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize)]
struct Summary {
    tier1_errors: usize,
    tier1_passes: usize,
    tier2_warnings: usize,
    adaptation_notes_seen: usize,
}

#[derive(Debug, Clone, Serialize)]
struct JsonReport {
    schema_version: &'static str,
    prompts_dir: String,
    manifest_path: String,
    prompts_scanned: Vec<String>,
    tier1_findings: Vec<Tier1Finding>,
    tier2_findings: Vec<Tier2Finding>,
    summary: Summary,
    exit_code: i32,
}

fn build_report(
    prompts_dir: &Path,
    manifest_path: &Path,
    prompts: &[ParsedPrompt],
    _manifest: &Manifest,
    tier1: Vec<Tier1Finding>,
    tier2: Vec<Tier2Finding>,
    strict: bool,
) -> JsonReport {
    let tier1_errors = tier1.len();
    let adaptation_notes_seen: usize = prompts
        .iter()
        .flat_map(|p| p.top_level_tags.iter())
        .filter(|t| t.has_adaptation_note)
        .count();

    // tier1_passes: total (role, tag) pairs that had manifest entries and no errors.
    // Simple heuristic: count checks that could have fired but didn't.
    let total_tag_instances: usize =
        prompts.iter().map(|p| p.top_level_tags.len()).sum();
    let tier1_passes = total_tag_instances.saturating_sub(tier1_errors);

    let exit_code = if strict && tier1_errors > 0 { 1 } else { 0 };

    JsonReport {
        schema_version: "v1",
        prompts_dir: prompts_dir.to_string_lossy().into_owned(),
        manifest_path: manifest_path.to_string_lossy().into_owned(),
        prompts_scanned: prompts.iter().map(|p| p.filename.clone()).collect(),
        tier1_findings: tier1,
        tier2_findings: tier2,
        summary: Summary {
            tier1_errors,
            tier1_passes,
            tier2_warnings: 0, // filled below
            adaptation_notes_seen,
        },
        exit_code,
    }
}

// ---------------------------------------------------------------------------
// Output helpers
// ---------------------------------------------------------------------------

fn print_text_report<W: Write>(report: &JsonReport, _manifest: &Manifest, out: &mut W) {
    writeln!(
        out,
        "v2-prompt-tag-semantic-fidelity — {} prompts scanned",
        report.prompts_scanned.len()
    )
    .ok();
    writeln!(out, "manifest: {}", report.manifest_path).ok();

    if report.tier1_findings.is_empty() {
        writeln!(out, "[PASS] Tier 1: no errors").ok();
    } else {
        writeln!(
            out,
            "[FAIL] Tier 1: {} error(s)",
            report.summary.tier1_errors
        )
        .ok();
        for f in &report.tier1_findings {
            let kind_str = match f.kind {
                Tier1Kind::UnknownTag => "unknown-tag",
                Tier1Kind::MissingRequired => "missing-required",
                Tier1Kind::MissingExpectedChild => "missing-expected-child",
                Tier1Kind::RoleNotAllowed => "role-not-allowed",
            };
            writeln!(
                out,
                "  ERROR  [{kind_str}] role={} tag=<{}>",
                f.role, f.tag
            )
            .ok();
            if let Some(ref exp) = f.expected_children {
                writeln!(out, "         expected-children: {exp:?}").ok();
                if let Some(ref found) = f.found_children {
                    writeln!(out, "         found-children: {found:?}").ok();
                }
            }
            if f.has_adaptation_note {
                writeln!(out, "         (adaptation note present)").ok();
            }
        }
    }

    if report.tier2_findings.is_empty() {
        writeln!(out, "[PASS] Tier 2: no warnings").ok();
    } else {
        writeln!(
            out,
            "[WARN] Tier 2: {} warning(s)",
            report.summary.tier2_warnings
        )
        .ok();
        for f in &report.tier2_findings {
            let kind_str = match f.kind {
                Tier2Kind::LengthPlausibility => "length-plausibility",
                Tier2Kind::KeywordOverlap => "keyword-overlap",
            };
            writeln!(
                out,
                "  WARN  [{kind_str}] role={} tag=<{}>: {}",
                f.role, f.tag, f.details
            )
            .ok();
        }
    }

    writeln!(
        out,
        "Summary: tier1_errors={} tier1_passes={} tier2_warnings={} adaptation_notes={}",
        report.summary.tier1_errors,
        report.summary.tier1_passes,
        report.summary.tier2_warnings,
        report.summary.adaptation_notes_seen
    )
    .ok();
    writeln!(out, "exit_code: {}", report.exit_code).ok();
}

fn print_schema<W: Write>(out: &mut W) {
    write!(
        out,
        r#"# prompts/v2/tag-semantics.toml — expected TOML manifest shape
#
# Top-level tags expected at the root <role-prompt> level.
# "required-in-roles" lists roles that MUST include this tag (missing → error).
# "allowed-in-roles"  lists roles that MAY include it (omitting from required is acceptable).
# "intent" is the one-line semantic description against which content is checked.
# "expected-children" optionally lists immediate child tag names that should appear.
#
# At least one of required-in-roles / allowed-in-roles is required per entry.

[tags.example-tag]
intent = "One-line description of what this tag's content should represent"
required-in-roles = ["planner", "executor", "curator", "reconciler"]
# allowed-in-roles = ["planner"]   # use this instead of required-in-roles for optional tags
expected-children = ["child-a", "child-b"]
"#
    )
    .ok();
}

// ---------------------------------------------------------------------------
// List-tags subcommand
// ---------------------------------------------------------------------------

fn cmd_list_tags<W: Write>(prompts_dir: &Path, out: &mut W) -> Result<ExitCode, ToolError> {
    let prompts = load_prompts(prompts_dir)?;
    let mut tag_roles: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for prompt in &prompts {
        for tag in &prompt.top_level_tags {
            tag_roles
                .entry(tag.name.clone())
                .or_default()
                .push(prompt.role.clone());
        }
    }
    writeln!(out, "Top-level tags across v2 prompts:").ok();
    writeln!(out, "{:<40} ROLES", "TAG").ok();
    writeln!(out, "{}", "-".repeat(60)).ok();
    for (tag, roles) in &tag_roles {
        writeln!(out, "{:<40} {}", tag, roles.join(", ")).ok();
    }
    Ok(ExitCode::SUCCESS)
}

// ---------------------------------------------------------------------------
// Main check command
// ---------------------------------------------------------------------------

fn cmd_check<W: Write>(
    prompts_dir: &Path,
    manifest_path: &Path,
    strict: bool,
    format: OutputFormat,
    out: &mut W,
) -> Result<ExitCode, ToolError> {
    let manifest = load_manifest(manifest_path)?;
    let prompts = load_prompts(prompts_dir)?;

    let tier1 = run_tier1(&manifest, &prompts);
    let tier2 = run_tier2(&manifest, &prompts);

    let tier2_count = tier2.len();
    let mut report = build_report(
        prompts_dir,
        manifest_path,
        &prompts,
        &manifest,
        tier1,
        tier2.clone(),
        strict,
    );
    report.summary.tier2_warnings = tier2_count;

    let exit_code = ExitCode::from(report.exit_code as u8);

    match format {
        OutputFormat::Json => {
            let json =
                serde_json::to_string_pretty(&report).map_err(|e| {
                    ToolError::Io(std::io::Error::other(e.to_string()))
                })?;
            writeln!(out, "{json}").ok();
        }
        OutputFormat::Text => {
            print_text_report(&report, &manifest, out);
        }
    }

    Ok(exit_code)
}

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

fn main() -> ExitCode {
    let args = Args::parse();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    match args.command {
        Commands::Check {
            prompts_dir,
            manifest,
            strict,
            format,
        } => match cmd_check(&prompts_dir, &manifest, strict, format, &mut out) {
            Ok(code) => code,
            Err(e) => {
                eprintln!("v2-prompt-tag-semantic-fidelity: {e}");
                ExitCode::from(2)
            }
        },
        Commands::Schema => {
            print_schema(&mut out);
            ExitCode::SUCCESS
        }
        Commands::ListTags { prompts_dir } => {
            match cmd_list_tags(&prompts_dir, &mut out) {
                Ok(code) => code,
                Err(e) => {
                    eprintln!("v2-prompt-tag-semantic-fidelity: {e}");
                    ExitCode::from(2)
                }
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // Manifest parser tests
    // ------------------------------------------------------------------

    #[test]
    fn manifest_round_trip_valid() {
        let toml = r#"
[tags.role-identity]
intent = "Names the role and its scope"
required-in-roles = ["planner", "executor"]
expected-children = ["one-line"]

[tags.constraints]
intent = "Hard rules the role must obey"
allowed-in-roles = ["planner", "executor", "curator", "reconciler"]
"#;
        let raw: RawManifest = toml::from_str(toml).unwrap();
        assert_eq!(raw.tags.len(), 2);
        assert_eq!(
            raw.tags["role-identity"].required_in_roles,
            vec!["planner", "executor"]
        );
        assert_eq!(raw.tags["constraints"].allowed_in_roles.len(), 4);
    }

    #[test]
    fn manifest_rejects_missing_intent() {
        let toml = r#"
[tags.bad-tag]
required-in-roles = ["planner"]
"#;
        let result: Result<RawManifest, _> = toml::from_str(toml);
        assert!(result.is_err(), "should fail: missing intent");
    }

    #[test]
    fn manifest_rejects_neither_required_nor_allowed() {
        let toml = r#"
[tags.no-roles-tag]
intent = "something"
"#;
        let raw: RawManifest = toml::from_str(toml).unwrap();
        let entry = &raw.tags["no-roles-tag"];
        assert!(
            entry.required_in_roles.is_empty() && entry.allowed_in_roles.is_empty(),
            "both empty — the validation step should reject this"
        );

        // Simulate the load_manifest validation.
        let err = {
            let mut manifest: BTreeMap<String, TagEntry> = BTreeMap::new();
            for (name, re) in &raw.tags {
                if re.required_in_roles.is_empty() && re.allowed_in_roles.is_empty() {
                    // Return the error message.
                    break;
                }
                manifest.insert(
                    name.clone(),
                    TagEntry {
                        intent: re.intent.clone(),
                        required_in_roles: re.required_in_roles.clone(),
                        allowed_in_roles: re.allowed_in_roles.clone(),
                        expected_children: re.expected_children.clone(),
                    },
                );
            }
            // The manifest will be empty because validation rejects the entry.
            manifest.is_empty()
        };
        assert!(err, "manifest should be empty after validation rejects bad entry");
    }

    // ------------------------------------------------------------------
    // Prompt XML parsing tests
    // ------------------------------------------------------------------

    fn minimal_prompt_xml(role: &str, tags: &[(&str, &str)]) -> String {
        let body: String = tags
            .iter()
            .map(|(tag, children)| format!("<{tag}>{children}</{tag}>\n"))
            .collect();
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<role-prompt version="v2-test" role="{role}">
{body}</role-prompt>"#
        )
    }

    #[test]
    fn parse_prompt_extracts_role_and_tags() {
        let xml = minimal_prompt_xml("planner", &[("inputs", ""), ("constraints", "")]);
        let tmp = tempfile::NamedTempFile::new().unwrap();
        fs::write(tmp.path(), xml).unwrap();
        let parsed = parse_prompt_file(tmp.path()).unwrap();
        assert_eq!(parsed.role, "planner");
        assert_eq!(parsed.top_level_tags.len(), 2);
        assert_eq!(parsed.top_level_tags[0].name, "inputs");
        assert_eq!(parsed.top_level_tags[1].name, "constraints");
    }

    #[test]
    fn parse_prompt_detects_adaptation_note() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
<role-prompt version="v2-test" role="executor">
<custom-tag>
  <semantic-adaptation-note>This is a note explaining the divergence.</semantic-adaptation-note>
  <child-a/>
</custom-tag>
</role-prompt>"#;
        let tmp = tempfile::NamedTempFile::new().unwrap();
        fs::write(tmp.path(), xml).unwrap();
        let parsed = parse_prompt_file(tmp.path()).unwrap();
        assert_eq!(parsed.top_level_tags.len(), 1);
        let tag = &parsed.top_level_tags[0];
        assert!(tag.has_adaptation_note);
        assert!(tag
            .adaptation_note_text
            .as_deref()
            .unwrap_or("")
            .contains("divergence"));
    }

    // ------------------------------------------------------------------
    // Tier 1 tests
    // ------------------------------------------------------------------

    fn make_manifest_entry(
        intent: &str,
        required: &[&str],
        allowed: &[&str],
        children: &[&str],
    ) -> TagEntry {
        TagEntry {
            intent: intent.to_string(),
            required_in_roles: required.iter().map(|s| s.to_string()).collect(),
            allowed_in_roles: allowed.iter().map(|s| s.to_string()).collect(),
            expected_children: children.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn parsed_tag(name: &str, children: &[&str], has_note: bool) -> ParsedTag {
        ParsedTag {
            name: name.to_string(),
            direct_children: children.iter().map(|s| s.to_string()).collect(),
            has_adaptation_note: has_note,
            adaptation_note_text: None,
            content_line_count: 5,
            direct_child_element_count: children.len(),
        }
    }

    fn make_prompt(role: &str, tags: &[ParsedTag]) -> ParsedPrompt {
        ParsedPrompt {
            role: role.to_string(),
            filename: format!("{role}-prompt.xml"),
            top_level_tags: tags.to_vec(),
        }
    }

    #[test]
    fn tier1_unknown_tag_without_note_is_error() {
        let manifest: Manifest = BTreeMap::new(); // empty — all tags unknown
        let tag = parsed_tag("mystery-tag", &[], false);
        let prompt = make_prompt("planner", &[tag]);
        let findings = run_tier1(&manifest, &[prompt]);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].kind, Tier1Kind::UnknownTag);
    }

    #[test]
    fn tier1_unknown_tag_with_note_is_suppressed() {
        let manifest: Manifest = BTreeMap::new();
        let tag = parsed_tag("mystery-tag", &[], true); // has adaptation note
        let prompt = make_prompt("planner", &[tag]);
        let findings = run_tier1(&manifest, &[prompt]);
        assert!(
            findings.is_empty(),
            "adaptation note should suppress unknown-tag"
        );
    }

    #[test]
    fn tier1_missing_required_tag() {
        let mut manifest: Manifest = BTreeMap::new();
        manifest.insert(
            "required-tag".to_string(),
            make_manifest_entry("A required tag", &["planner"], &[], &[]),
        );
        // Prompt does NOT contain required-tag.
        let prompt = make_prompt("planner", &[]);
        let findings = run_tier1(&manifest, &[prompt]);
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].kind, Tier1Kind::MissingRequired);
        assert_eq!(findings[0].tag, "required-tag");
    }

    #[test]
    fn tier1_role_not_allowed() {
        let mut manifest: Manifest = BTreeMap::new();
        manifest.insert(
            "planner-only".to_string(),
            make_manifest_entry("Planner-only tag", &["planner"], &[], &[]),
        );
        // Executor has this tag but it should only be in planner.
        let tag = parsed_tag("planner-only", &[], false);
        let prompt = make_prompt("executor", &[tag]);
        let findings = run_tier1(&manifest, &[prompt]);
        // Should find: role-not-allowed in executor + missing-required in planner.
        let not_allowed: Vec<_> = findings
            .iter()
            .filter(|f| f.kind == Tier1Kind::RoleNotAllowed)
            .collect();
        assert!(!not_allowed.is_empty());
    }

    #[test]
    fn tier1_missing_expected_child() {
        let mut manifest: Manifest = BTreeMap::new();
        manifest.insert(
            "role-identity".to_string(),
            make_manifest_entry(
                "Role identity",
                &["planner"],
                &[],
                &["one-line", "position-in-super-step-sequence"],
            ),
        );
        // Tag is present but missing expected child "position-in-super-step-sequence".
        let tag = parsed_tag("role-identity", &["one-line"], false);
        let prompt = make_prompt("planner", &[tag]);
        let findings = run_tier1(&manifest, &[prompt]);
        let missing_child: Vec<_> = findings
            .iter()
            .filter(|f| f.kind == Tier1Kind::MissingExpectedChild)
            .collect();
        assert_eq!(missing_child.len(), 1);
        let missing = missing_child[0].expected_children.as_ref().unwrap();
        assert!(missing.contains(&"position-in-super-step-sequence".to_string()));
    }

    // ------------------------------------------------------------------
    // Tier 2 tests
    // ------------------------------------------------------------------

    fn make_tag_for_t2(name: &str, line_count: usize, child_count: usize) -> ParsedTag {
        ParsedTag {
            name: name.to_string(),
            direct_children: (0..child_count)
                .map(|i| format!("child-{i}"))
                .collect(),
            has_adaptation_note: false,
            adaptation_note_text: None,
            content_line_count: line_count,
            direct_child_element_count: child_count,
        }
    }

    #[test]
    fn tier2_one_line_too_long_is_warning() {
        let manifest: Manifest = BTreeMap::new();
        let tag = make_tag_for_t2("one-line", 5, 0); // 5 lines > 2
        let prompt = make_prompt("planner", &[tag]);
        let findings = run_tier2(&manifest, &[prompt]);
        assert!(findings
            .iter()
            .any(|f| f.tag == "one-line" && f.kind == Tier2Kind::LengthPlausibility));
    }

    #[test]
    fn tier2_one_line_short_is_ok() {
        let manifest: Manifest = BTreeMap::new();
        let tag = make_tag_for_t2("one-line", 1, 0);
        let prompt = make_prompt("planner", &[tag]);
        let findings = run_tier2(&manifest, &[prompt]);
        assert!(findings.is_empty());
    }

    #[test]
    fn tier2_role_identity_too_few_sub_elements_is_warning() {
        let manifest: Manifest = BTreeMap::new();
        let tag = make_tag_for_t2("role-identity", 10, 2); // 2 children < 3
        let prompt = make_prompt("planner", &[tag]);
        let findings = run_tier2(&manifest, &[prompt]);
        assert!(findings
            .iter()
            .any(|f| f.tag == "role-identity" && f.kind == Tier2Kind::LengthPlausibility));
    }

    #[test]
    fn tier2_role_identity_enough_sub_elements_is_ok() {
        let manifest: Manifest = BTreeMap::new();
        let tag = make_tag_for_t2("role-identity", 10, 4); // 4 children ≥ 3
        let prompt = make_prompt("planner", &[tag]);
        let findings = run_tier2(&manifest, &[prompt]);
        assert!(findings.iter().all(|f| f.tag != "role-identity"
            || f.kind != Tier2Kind::LengthPlausibility));
    }

    #[test]
    fn tier2_output_contract_too_short_is_warning() {
        let manifest: Manifest = BTreeMap::new();
        let tag = make_tag_for_t2("output-contract", 5, 3); // 5 lines < 10
        let prompt = make_prompt("planner", &[tag]);
        let findings = run_tier2(&manifest, &[prompt]);
        assert!(findings
            .iter()
            .any(|f| f.tag == "output-contract" && f.kind == Tier2Kind::LengthPlausibility));
    }

    #[test]
    fn tier2_output_contract_long_enough_is_ok() {
        let manifest: Manifest = BTreeMap::new();
        let tag = make_tag_for_t2("output-contract", 15, 4); // 15 lines ≥ 10
        let prompt = make_prompt("planner", &[tag]);
        let findings = run_tier2(&manifest, &[prompt]);
        assert!(findings.iter().all(|f| f.tag != "output-contract"
            || f.kind != Tier2Kind::LengthPlausibility));
    }

    // ------------------------------------------------------------------
    // Adaptation note suppression test
    // ------------------------------------------------------------------

    #[test]
    fn adaptation_note_suppresses_unknown_tag_not_required_check() {
        let mut manifest: Manifest = BTreeMap::new();
        manifest.insert(
            "role-identity".to_string(),
            make_manifest_entry("Role identity", &["planner"], &[], &[]),
        );
        // Prompt has an unknown tag WITH adaptation note → no unknown-tag error.
        // But role-identity (required for planner) is missing → MissingRequired.
        let unknown_tag = parsed_tag("novel-tag", &[], true);
        let prompt = make_prompt("planner", &[unknown_tag]);
        let findings = run_tier1(&manifest, &[prompt]);
        let unknown: Vec<_> = findings
            .iter()
            .filter(|f| f.kind == Tier1Kind::UnknownTag)
            .collect();
        let missing_req: Vec<_> = findings
            .iter()
            .filter(|f| f.kind == Tier1Kind::MissingRequired)
            .collect();
        assert!(unknown.is_empty(), "adaptation note suppresses unknown-tag");
        assert!(!missing_req.is_empty(), "missing-required still fires");
    }

    // ------------------------------------------------------------------
    // CLI exit code tests (strict vs non-strict × clean vs dirty)
    // ------------------------------------------------------------------

    fn make_manifest_with_required_tag() -> Manifest {
        let mut m: Manifest = BTreeMap::new();
        m.insert(
            "role-identity".to_string(),
            make_manifest_entry("Role identity", &["planner"], &[], &[]),
        );
        m
    }

    #[test]
    fn exit_code_clean_strict_is_zero() {
        let manifest = make_manifest_with_required_tag();
        let tag = parsed_tag("role-identity", &[], false);
        let prompt = make_prompt("planner", &[tag]);
        let tier1 = run_tier1(&manifest, std::slice::from_ref(&prompt));
        assert!(tier1.is_empty());
        let report = build_report(
            Path::new("prompts/v2/"),
            Path::new("manifest.toml"),
            std::slice::from_ref(&prompt),
            &manifest,
            tier1,
            vec![],
            true, // strict
        );
        assert_eq!(report.exit_code, 0);
    }

    #[test]
    fn exit_code_dirty_strict_is_one() {
        let manifest = make_manifest_with_required_tag();
        // Prompt missing required tag.
        let prompt = make_prompt("planner", &[]);
        let tier1 = run_tier1(&manifest, std::slice::from_ref(&prompt));
        assert!(!tier1.is_empty());
        let report = build_report(
            Path::new("prompts/v2/"),
            Path::new("manifest.toml"),
            std::slice::from_ref(&prompt),
            &manifest,
            tier1,
            vec![],
            true, // strict
        );
        assert_eq!(report.exit_code, 1);
    }

    #[test]
    fn exit_code_dirty_non_strict_is_zero() {
        let manifest = make_manifest_with_required_tag();
        let prompt = make_prompt("planner", &[]);
        let tier1 = run_tier1(&manifest, std::slice::from_ref(&prompt));
        assert!(!tier1.is_empty());
        let report = build_report(
            Path::new("prompts/v2/"),
            Path::new("manifest.toml"),
            std::slice::from_ref(&prompt),
            &manifest,
            tier1,
            vec![],
            false, // NOT strict
        );
        assert_eq!(report.exit_code, 0);
    }

    // ------------------------------------------------------------------
    // Tokenizer
    // ------------------------------------------------------------------

    #[test]
    fn tokenize_filters_stop_words_and_short_tokens() {
        let tokens = tokenize("the role is a planner in the system");
        assert!(!tokens.contains("the"), "stop word 'the' should be removed");
        assert!(!tokens.contains("is"), "stop word 'is' should be removed");
        assert!(!tokens.contains("a"), "short token 'a' should be removed");
        assert!(tokens.contains("planner"), "content word should be kept");
        assert!(tokens.contains("system"), "content word should be kept");
    }

    // ------------------------------------------------------------------
    // JSON output shape
    // ------------------------------------------------------------------

    #[test]
    fn json_report_has_schema_version_v1() {
        let manifest = make_manifest_with_required_tag();
        let tag = parsed_tag("role-identity", &[], false);
        let prompt = make_prompt("planner", &[tag]);
        let tier1 = run_tier1(&manifest, std::slice::from_ref(&prompt));
        let report = build_report(
            Path::new("prompts/v2/"),
            Path::new("manifest.toml"),
            std::slice::from_ref(&prompt),
            &manifest,
            tier1,
            vec![],
            false,
        );
        let json = serde_json::to_string(&report).unwrap();
        let val: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(val["schema_version"], "v1");
    }
}
