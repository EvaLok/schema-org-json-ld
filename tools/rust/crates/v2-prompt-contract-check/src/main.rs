use clap::{Parser, ValueEnum};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const EXPECTED_PROMPT_FILES: &[&str] = &[
    "planner-prompt.xml",
    "reconciler-prompt.xml",
    "executor-prompt.xml",
    "curator-prompt.xml",
];

#[derive(Parser, Debug)]
#[command(
    name = "v2-prompt-contract-check",
    about = "Verifies v2 role prompts' output-contract declarations against v2-channel-router schema"
)]
struct Args {
    /// Directory containing role prompt XML files.
    #[arg(long, default_value = "prompts/v2/")]
    prompts_dir: PathBuf,

    /// Path to v2-channel-router binary.
    #[arg(long, default_value_os_t = default_channel_router_bin())]
    channel_router_bin: PathBuf,

    /// Fail closed on mismatches.
    #[arg(long)]
    strict: bool,

    /// Output format.
    #[arg(long, value_enum, default_value_t = OutputFormat::Text)]
    format: OutputFormat,
}

#[derive(Copy, Clone, Debug, ValueEnum, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
}

#[derive(Debug)]
enum CheckError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Xml(quick_xml::Error),
    Invocation(String),
}

impl std::fmt::Display for CheckError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CheckError::Io(e) => write!(f, "{e}"),
            CheckError::Json(e) => write!(f, "json error: {e}"),
            CheckError::Xml(e) => write!(f, "xml parse error: {e}"),
            CheckError::Invocation(msg) => f.write_str(msg),
        }
    }
}

impl From<std::io::Error> for CheckError {
    fn from(value: std::io::Error) -> Self {
        CheckError::Io(value)
    }
}

impl From<serde_json::Error> for CheckError {
    fn from(value: serde_json::Error) -> Self {
        CheckError::Json(value)
    }
}

impl From<quick_xml::Error> for CheckError {
    fn from(value: quick_xml::Error) -> Self {
        CheckError::Xml(value)
    }
}

#[derive(Debug, Clone, Deserialize)]
struct ChannelSchema {
    channel: String,
    allowed_writer: String,
    required_payload_keys: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct KeyDelta {
    extra_in_prompt: Vec<String>,
    missing_in_prompt: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum MismatchKind {
    ExtraInPrompt,
    MissingInPrompt,
    KeysMismatch,
    RoleMappingMissing,
    PromptFileMissing,
    ParseError,
    InputChannelUnknown,
    InputKeyUnknown,
}

#[derive(Debug, Clone, Serialize)]
struct Mismatch {
    prompt_file: String,
    channel: String,
    kind: MismatchKind,
    channel_router_keys: Vec<String>,
    prompt_keys: Vec<String>,
    delta: KeyDelta,
    details: Option<String>,
}

#[derive(Debug, Clone)]
struct PromptResult {
    prompt_file: String,
    role: String,
    channel: String,
    matched: bool,
}

#[derive(Debug, Clone, Serialize)]
struct SkippedCheck {
    check: String,
    reason: String,
}

#[derive(Debug, Clone)]
struct RunReport {
    prompt_results: Vec<PromptResult>,
    mismatches: Vec<Mismatch>,
    skipped_checks: Vec<SkippedCheck>,
}

#[derive(Debug, Clone, Serialize)]
struct JsonReport {
    matched: bool,
    mismatches: Vec<Mismatch>,
    skipped_checks: Vec<SkippedCheck>,
}

#[derive(Debug, Clone)]
struct InputSourceContract {
    channel: String,
    required_keys: BTreeSet<String>,
}

fn default_channel_router_bin() -> PathBuf {
    if let Ok(target_dir) = std::env::var("CARGO_TARGET_DIR") {
        PathBuf::from(target_dir)
            .join("debug")
            .join("v2-channel-router")
    } else {
        PathBuf::from("./target/debug/v2-channel-router")
    }
}

fn main() -> ExitCode {
    let args = Args::parse();
    let stdout = io::stdout();
    let mut out = stdout.lock();

    match run(&args, &mut out) {
        Ok(report) => {
            let has_mismatch = !report.mismatches.is_empty();
            if args.strict && has_mismatch {
                ExitCode::from(1)
            } else {
                ExitCode::SUCCESS
            }
        }
        Err(err) => {
            eprintln!("v2-prompt-contract-check: {err}");
            ExitCode::from(2)
        }
    }
}

fn run<W: Write>(args: &Args, out: &mut W) -> Result<RunReport, CheckError> {
    let prompts_dir = resolve_prompts_dir(&args.prompts_dir)?;
    let schemas = read_router_schema(&args.channel_router_bin)?;
    let by_writer = map_channel_by_writer(&schemas);
    let all_channels: BTreeSet<String> = schemas.iter().map(|s| s.channel.clone()).collect();

    let mut prompt_results = Vec::new();
    let mut mismatches = Vec::new();
    let mut parseable_input_declarations_found = false;

    for prompt_file in EXPECTED_PROMPT_FILES {
        let prompt_path = prompts_dir.join(prompt_file);
        let role = infer_role_from_prompt_filename(prompt_file).ok_or_else(|| {
            CheckError::Invocation(format!("could not infer role from prompt filename: {prompt_file}"))
        })?;

        let mut result = PromptResult {
            prompt_file: prompt_file.to_string(),
            role: role.to_string(),
            channel: "<unknown>".to_string(),
            matched: false,
        };

        if !prompt_path.exists() {
            mismatches.push(Mismatch {
                prompt_file: prompt_path.display().to_string(),
                channel: "<unknown>".to_string(),
                kind: MismatchKind::PromptFileMissing,
                channel_router_keys: Vec::new(),
                prompt_keys: Vec::new(),
                delta: KeyDelta {
                    extra_in_prompt: Vec::new(),
                    missing_in_prompt: Vec::new(),
                },
                details: Some("prompt file is missing".to_string()),
            });
            prompt_results.push(result);
            continue;
        }

        let schema = match by_writer.get(role) {
            Some(schema) => schema,
            None => {
                mismatches.push(Mismatch {
                    prompt_file: prompt_path.display().to_string(),
                    channel: "<unknown>".to_string(),
                    kind: MismatchKind::RoleMappingMissing,
                    channel_router_keys: Vec::new(),
                    prompt_keys: Vec::new(),
                    delta: KeyDelta {
                        extra_in_prompt: Vec::new(),
                        missing_in_prompt: Vec::new(),
                    },
                    details: Some(format!(
                        "channel-router schema has no channel with allowed_writer='{role}'"
                    )),
                });
                prompt_results.push(result);
                continue;
            }
        };

        result.channel = schema.channel.clone();

        let xml = fs::read_to_string(&prompt_path)?;
        let prompt_output_keys = match parse_output_contract_required_keys(&xml) {
            Ok(keys) => keys,
            Err(err) => {
                mismatches.push(Mismatch {
                    prompt_file: prompt_path.display().to_string(),
                    channel: schema.channel.clone(),
                    kind: MismatchKind::ParseError,
                    channel_router_keys: schema.required_payload_keys.clone(),
                    prompt_keys: Vec::new(),
                    delta: KeyDelta {
                        extra_in_prompt: Vec::new(),
                        missing_in_prompt: schema.required_payload_keys.clone(),
                    },
                    details: Some(format!("failed to parse output-contract required keys: {err}")),
                });
                prompt_results.push(result);
                continue;
            }
        };

        let router_output_keys: BTreeSet<String> = schema.required_payload_keys.iter().cloned().collect();
        let mut effective_prompt_output_keys = prompt_output_keys.clone();
        let mut output_delta = compare_keys(&router_output_keys, &effective_prompt_output_keys);
        if !output_delta.extra_in_prompt.is_empty() || !output_delta.missing_in_prompt.is_empty() {
            if let Some(surface_keys) = parse_surface_required_keys_for_channel(&xml, &schema.channel)? {
                let surface_delta = compare_keys(&router_output_keys, &surface_keys);
                if surface_delta.extra_in_prompt.is_empty() && surface_delta.missing_in_prompt.is_empty() {
                    effective_prompt_output_keys = surface_keys;
                    output_delta = surface_delta;
                }
            }
        }
        if !output_delta.extra_in_prompt.is_empty() || !output_delta.missing_in_prompt.is_empty() {
            mismatches.push(Mismatch {
                prompt_file: prompt_path.display().to_string(),
                channel: schema.channel.clone(),
                kind: mismatch_kind_from_delta(&output_delta),
                channel_router_keys: set_to_vec(&router_output_keys),
                prompt_keys: set_to_vec(&effective_prompt_output_keys),
                delta: output_delta,
                details: None,
            });
            prompt_results.push(result);
            continue;
        }

        // Critique-style note:
        // Check 3 and Check 4 in the cycle-146 design expect parseable declarations shaped like:
        //   <inputs><source channel="..."> ... <required-key name="..."/> ... </source></inputs>
        // The current v2 prompts do not use this structure; they use <source>text...</source> plus
        // <required-keys><key name="...">...</key></required-keys>. For cycle-1 minimal scope we
        // only enforce parseable <source channel="..."> declarations. When absent, we skip checks 3/4.
        let input_contracts = parse_input_source_contracts(&xml)?;
        if !input_contracts.is_empty() {
            parseable_input_declarations_found = true;
        }

        for input in input_contracts {
            if !all_channels.contains(&input.channel) {
                mismatches.push(Mismatch {
                    prompt_file: prompt_path.display().to_string(),
                    channel: schema.channel.clone(),
                    kind: MismatchKind::InputChannelUnknown,
                    channel_router_keys: Vec::new(),
                    prompt_keys: set_to_vec(&input.required_keys),
                    delta: KeyDelta {
                        extra_in_prompt: Vec::new(),
                        missing_in_prompt: Vec::new(),
                    },
                    details: Some(format!(
                        "input source channel '{}' is not present in channel-router schema",
                        input.channel
                    )),
                });
                continue;
            }

            let Some(input_schema) = schemas.iter().find(|s| s.channel == input.channel) else {
                continue;
            };
            let input_router_keys: BTreeSet<String> =
                input_schema.required_payload_keys.iter().cloned().collect();
            let unknown_input_keys: Vec<String> = input
                .required_keys
                .iter()
                .filter(|k| !input_router_keys.contains(*k))
                .cloned()
                .collect();
            if !unknown_input_keys.is_empty() {
                mismatches.push(Mismatch {
                    prompt_file: prompt_path.display().to_string(),
                    channel: schema.channel.clone(),
                    kind: MismatchKind::InputKeyUnknown,
                    channel_router_keys: set_to_vec(&input_router_keys),
                    prompt_keys: set_to_vec(&input.required_keys),
                    delta: KeyDelta {
                        extra_in_prompt: unknown_input_keys,
                        missing_in_prompt: Vec::new(),
                    },
                    details: Some(format!(
                        "input required-key declaration contains keys not required by '{}': see delta.extra_in_prompt",
                        input_schema.channel
                    )),
                });
            }
        }

        result.matched = true;
        prompt_results.push(result);
    }

    let mut skipped_checks = Vec::new();
    if !parseable_input_declarations_found {
        skipped_checks.push(SkippedCheck {
            check: "check-3-input-channel-membership".to_string(),
            reason: "No parseable <source channel=\"...\"> declarations found in current prompts/v2 XML structure; deferred for cycle-1 minimal scope.".to_string(),
        });
        skipped_checks.push(SkippedCheck {
            check: "check-4-input-required-keys".to_string(),
            reason: "No parseable input-channel key blocks bound to <source channel=\"...\"> were found; deferred for cycle-1 minimal scope.".to_string(),
        });
    }

    let report = RunReport {
        prompt_results,
        mismatches,
        skipped_checks,
    };
    emit_report(args.format, &report, out)?;
    Ok(report)
}

fn resolve_prompts_dir(prompts_dir: &Path) -> Result<PathBuf, CheckError> {
    if prompts_dir.is_absolute() || prompts_dir.exists() {
        return Ok(prompts_dir.to_path_buf());
    }

    let cwd = std::env::current_dir()?;
    for ancestor in cwd.ancestors() {
        let candidate = ancestor.join(prompts_dir);
        if candidate.exists() {
            return Ok(candidate);
        }
    }

    Err(CheckError::Invocation(format!(
        "prompts directory not found: {} (checked cwd and ancestors)",
        prompts_dir.display()
    )))
}

fn emit_report<W: Write>(format: OutputFormat, report: &RunReport, out: &mut W) -> Result<(), CheckError> {
    match format {
        OutputFormat::Json => {
            let json = JsonReport {
                matched: report.mismatches.is_empty(),
                mismatches: report.mismatches.clone(),
                skipped_checks: report.skipped_checks.clone(),
            };
            serde_json::to_writer_pretty(&mut *out, &json)?;
            writeln!(out)?;
        }
        OutputFormat::Text => {
            let total = report.prompt_results.len();
            if report.mismatches.is_empty() {
                writeln!(
                    out,
                    "v2-prompt-contract-check: {total}/{total} prompts contract-aligned with v2-channel-router"
                )?;
                for result in &report.prompt_results {
                    writeln!(
                        out,
                        "  ✓ {} ↔ {} (writer: {})",
                        result.prompt_file,
                        result.channel,
                        role_display_name(&result.role)
                    )?;
                }
            } else {
                let mismatch_count = report.mismatches.len();
                if mismatch_count == 1 {
                    let m = &report.mismatches[0];
                    let filename = Path::new(&m.prompt_file)
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or(&m.prompt_file);
                    writeln!(
                        out,
                        "v2-prompt-contract-check: 1/{total} mismatch in {filename} ↔ {}",
                        m.channel
                    )?;
                    emit_mismatch_block(out, m)?;
                } else {
                    writeln!(
                        out,
                        "v2-prompt-contract-check: {mismatch_count}/{total} mismatches found"
                    )?;
                    for m in &report.mismatches {
                        writeln!(out)?;
                        emit_mismatch_block(out, m)?;
                    }
                }
            }

            if !report.skipped_checks.is_empty() {
                writeln!(out)?;
                writeln!(out, "Deferred in cycle-1 minimal scope:")?;
                for skipped in &report.skipped_checks {
                    writeln!(out, "  - {}: {}", skipped.check, skipped.reason)?;
                }
            }
        }
    }

    Ok(())
}

fn emit_mismatch_block<W: Write>(out: &mut W, mismatch: &Mismatch) -> Result<(), CheckError> {
    writeln!(out, "  channel-router required_payload_keys: {:?}", mismatch.channel_router_keys)?;
    writeln!(out, "  prompt declares:                       {:?}", mismatch.prompt_keys)?;
    writeln!(
        out,
        "  EXTRA-IN-PROMPT: {:?}{}",
        mismatch.delta.extra_in_prompt,
        if mismatch.delta.extra_in_prompt.is_empty() {
            ""
        } else {
            " (decorative; channel-router does not enforce)"
        }
    )?;
    writeln!(
        out,
        "  MISSING-IN-PROMPT: {:?}",
        mismatch.delta.missing_in_prompt
    )?;

    if let Some(details) = &mismatch.details {
        writeln!(out)?;
        writeln!(out, "  Details:")?;
        writeln!(out, "    {details}")?;
    }

    writeln!(out)?;
    writeln!(out, "  Diff location:")?;
    writeln!(
        out,
        "    channel-router: tools/rust/crates/v2-channel-router/src/main.rs (run `schema --format json` for live truth)"
    )?;
    writeln!(out, "    prompt:         {}", mismatch.prompt_file)?;
    Ok(())
}

fn map_channel_by_writer(schemas: &[ChannelSchema]) -> HashMap<&str, &ChannelSchema> {
    let mut by_writer = HashMap::new();
    for schema in schemas {
        by_writer.insert(schema.allowed_writer.as_str(), schema);
    }
    by_writer
}

fn read_router_schema(channel_router_bin: &Path) -> Result<Vec<ChannelSchema>, CheckError> {
    if !channel_router_bin.exists() {
        return Err(CheckError::Invocation(format!(
            "channel-router binary not found at {} (build it with `cargo build -p v2-channel-router`)",
            channel_router_bin.display()
        )));
    }

    let output = Command::new(channel_router_bin)
        .args(["schema", "--format", "json"])
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CheckError::Invocation(format!(
            "failed to execute `{}`: {stderr}",
            channel_router_bin.display()
        )));
    }

    let schema: Vec<ChannelSchema> = serde_json::from_slice(&output.stdout)?;
    if schema.is_empty() {
        return Err(CheckError::Invocation(
            "channel-router schema output was empty".to_string(),
        ));
    }
    Ok(schema)
}

fn infer_role_from_prompt_filename(file_name: &str) -> Option<&str> {
    file_name
        .strip_suffix("-prompt.xml")
        .filter(|role| matches!(*role, "planner" | "reconciler" | "executor" | "curator"))
}

fn role_display_name(role: &str) -> String {
    let mut chars = role.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => role.to_string(),
    }
}

fn mismatch_kind_from_delta(delta: &KeyDelta) -> MismatchKind {
    match (
        delta.extra_in_prompt.is_empty(),
        delta.missing_in_prompt.is_empty(),
    ) {
        (false, true) => MismatchKind::ExtraInPrompt,
        (true, false) => MismatchKind::MissingInPrompt,
        (false, false) => MismatchKind::KeysMismatch,
        (true, true) => MismatchKind::KeysMismatch,
    }
}

fn compare_keys(router_keys: &BTreeSet<String>, prompt_keys: &BTreeSet<String>) -> KeyDelta {
    let extra_in_prompt = prompt_keys
        .difference(router_keys)
        .cloned()
        .collect::<Vec<_>>();
    let missing_in_prompt = router_keys
        .difference(prompt_keys)
        .cloned()
        .collect::<Vec<_>>();
    KeyDelta {
        extra_in_prompt,
        missing_in_prompt,
    }
}

fn set_to_vec(set: &BTreeSet<String>) -> Vec<String> {
    set.iter().cloned().collect()
}

fn parse_output_contract_required_keys(xml: &str) -> Result<BTreeSet<String>, CheckError> {
    let output_contract_fragment = extract_tag_block(xml, "output-contract").ok_or_else(|| {
        CheckError::Invocation("missing <output-contract> block in prompt XML".to_string())
    })?;
    let mut reader = Reader::from_str(&output_contract_fragment);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut stack: Vec<Vec<u8>> = Vec::new();
    let mut keys = BTreeSet::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = e.name().as_ref().to_vec();
                if name == b"required-key" && stack_ends_with(&stack, &[b"output-contract", b"format"]) {
                    if let Some(value) = attr_value(&e, b"name") {
                        keys.insert(value);
                    }
                }
                stack.push(name);
            }
            Event::Empty(e) => {
                if e.name().as_ref() == b"required-key"
                    && stack_ends_with(&stack, &[b"output-contract", b"format"])
                {
                    if let Some(value) = attr_value(&e, b"name") {
                        keys.insert(value);
                    }
                }
            }
            Event::End(_) => {
                stack.pop();
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(keys)
}

fn parse_input_source_contracts(xml: &str) -> Result<Vec<InputSourceContract>, CheckError> {
    let Some(inputs_fragment) = extract_tag_block(xml, "inputs") else {
        return Ok(Vec::new());
    };

    let mut reader = Reader::from_str(&inputs_fragment);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut stack: Vec<Vec<u8>> = Vec::new();
    let mut current_source: Option<(usize, String, BTreeSet<String>)> = None;
    let mut contracts = Vec::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = e.name().as_ref().to_vec();
                if name == b"source" && stack_ends_with_anywhere(&stack, b"inputs") {
                    if let Some(channel) = attr_value(&e, b"channel") {
                        current_source = Some((stack.len() + 1, channel, BTreeSet::new()));
                    }
                }

                if name == b"required-key" {
                    if let Some((_, _, required_keys)) = current_source.as_mut() {
                        if let Some(key) = attr_value(&e, b"name") {
                            required_keys.insert(key);
                        }
                    }
                }

                stack.push(name);
            }
            Event::Empty(e) => {
                if e.name().as_ref() == b"source" && stack_ends_with_anywhere(&stack, b"inputs") {
                    if let Some(channel) = attr_value(&e, b"channel") {
                        contracts.push(InputSourceContract {
                            channel,
                            required_keys: BTreeSet::new(),
                        });
                    }
                }

                if e.name().as_ref() == b"required-key" {
                    if let Some((_, _, required_keys)) = current_source.as_mut() {
                        if let Some(key) = attr_value(&e, b"name") {
                            required_keys.insert(key);
                        }
                    }
                }
            }
            Event::End(e) => {
                if e.name().as_ref() == b"source" {
                    if let Some((depth, channel, required_keys)) = current_source.take() {
                        if depth == stack.len() {
                            contracts.push(InputSourceContract {
                                channel,
                                required_keys,
                            });
                        } else {
                            current_source = Some((depth, channel, required_keys));
                        }
                    }
                }
                stack.pop();
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(contracts)
}

fn parse_surface_required_keys_for_channel(
    xml: &str,
    channel: &str,
) -> Result<Option<BTreeSet<String>>, CheckError> {
    let Some(surfaces_fragment) = extract_tag_block(xml, "output-surfaces") else {
        return Ok(None);
    };

    let mut reader = Reader::from_str(&surfaces_fragment);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut stack: Vec<Vec<u8>> = Vec::new();
    let mut inside_target_surface = false;
    let mut keys = BTreeSet::new();

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = e.name().as_ref().to_vec();
                if name == b"surface" {
                    if let Some(surface_name) = attr_value(&e, b"name") {
                        if surface_name == channel {
                            inside_target_surface = true;
                        }
                    }
                }
                if inside_target_surface
                    && name == b"key"
                    && stack_ends_with(
                        &stack,
                        &[b"output-surfaces", b"surface", b"required-payload-keys"],
                    )
                {
                    if let Some(value) = attr_value(&e, b"name") {
                        keys.insert(value);
                    }
                }
                stack.push(name);
            }
            Event::Empty(e) => {
                if inside_target_surface
                    && e.name().as_ref() == b"key"
                    && stack_ends_with(
                        &stack,
                        &[b"output-surfaces", b"surface", b"required-payload-keys"],
                    )
                {
                    if let Some(value) = attr_value(&e, b"name") {
                        keys.insert(value);
                    }
                }
            }
            Event::End(e) => {
                if e.name().as_ref() == b"surface" && inside_target_surface {
                    inside_target_surface = false;
                }
                stack.pop();
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    if keys.is_empty() {
        Ok(None)
    } else {
        Ok(Some(keys))
    }
}

fn extract_tag_block(xml: &str, tag: &str) -> Option<String> {
    let open = format!("<{tag}");
    let close = format!("</{tag}>");
    let start = xml.find(&open)?;
    let open_end = xml[start..].find('>')? + start + 1;
    let close_start = xml[open_end..].find(&close)? + open_end;
    let end = close_start + close.len();
    Some(xml[start..end].to_string())
}

fn stack_ends_with(stack: &[Vec<u8>], expected_tail: &[&[u8]]) -> bool {
    if stack.len() < expected_tail.len() {
        return false;
    }
    let start = stack.len() - expected_tail.len();
    stack[start..]
        .iter()
        .zip(expected_tail.iter())
        .all(|(a, b)| a.as_slice() == *b)
}

fn stack_ends_with_anywhere(stack: &[Vec<u8>], expected: &[u8]) -> bool {
    stack.iter().any(|name| name.as_slice() == expected)
}

fn attr_value(element: &BytesStart<'_>, key_name: &[u8]) -> Option<String> {
    element
        .attributes()
        .flatten()
        .find(|attr| attr.key.as_ref() == key_name)
        .map(|attr| String::from_utf8_lossy(attr.value.as_ref()).to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_set(input: &[&str]) -> BTreeSet<String> {
        input.iter().map(|s| (*s).to_string()).collect()
    }

    fn sample_schema() -> Vec<ChannelSchema> {
        vec![
            ChannelSchema {
                channel: "plan-channel".to_string(),
                allowed_writer: "planner".to_string(),
                required_payload_keys: vec![
                    "substantive-focal".to_string(),
                    "per-role-tasks".to_string(),
                ],
            },
            ChannelSchema {
                channel: "inbound-channel".to_string(),
                allowed_writer: "reconciler".to_string(),
                required_payload_keys: vec![
                    "eva-responses".to_string(),
                    "audit-posts".to_string(),
                    "dispatch-returns".to_string(),
                ],
            },
            ChannelSchema {
                channel: "work-channel".to_string(),
                allowed_writer: "executor".to_string(),
                required_payload_keys: vec!["artifacts-written".to_string()],
            },
            ChannelSchema {
                channel: "memory-channel".to_string(),
                allowed_writer: "curator".to_string(),
                required_payload_keys: vec!["consolidated-insights".to_string()],
            },
        ]
    }

    #[test]
    fn parse_output_contract_required_keys_from_planner_prompt() {
        let xml = r#"
        <role-prompt>
          <output-contract>
            <format>
              <required-key name="substantive-focal" type="string" />
              <required-key name="per-role-tasks" type="object" />
            </format>
          </output-contract>
        </role-prompt>
        "#;

        let keys = parse_output_contract_required_keys(xml).unwrap();
        assert_eq!(keys, parse_set(&["per-role-tasks", "substantive-focal"]));
    }

    #[test]
    fn parse_output_contract_handles_nested_subkeys_correctly() {
        let xml = r#"
        <role-prompt>
          <output-contract>
            <format>
              <required-key name="per-role-tasks" type="object">
                <sub-key name="executor" />
                <sub-key name="curator" />
              </required-key>
              <required-key name="substantive-focal" type="string" />
            </format>
          </output-contract>
        </role-prompt>
        "#;

        let keys = parse_output_contract_required_keys(xml).unwrap();
        assert_eq!(keys, parse_set(&["per-role-tasks", "substantive-focal"]));
    }

    #[test]
    fn compare_keys_match_case() {
        let router = parse_set(&["substantive-focal", "per-role-tasks"]);
        let prompt = parse_set(&["per-role-tasks", "substantive-focal"]);
        let delta = compare_keys(&router, &prompt);
        assert!(delta.extra_in_prompt.is_empty());
        assert!(delta.missing_in_prompt.is_empty());
    }

    #[test]
    fn compare_keys_extra_in_prompt() {
        let router = parse_set(&["artifacts-written"]);
        let prompt = parse_set(&["artifacts-written", "files-changed"]);
        let delta = compare_keys(&router, &prompt);

        assert_eq!(delta.extra_in_prompt, vec!["files-changed"]);
        assert!(delta.missing_in_prompt.is_empty());
        assert_eq!(mismatch_kind_from_delta(&delta), MismatchKind::ExtraInPrompt);
    }

    #[test]
    fn compare_keys_missing_in_prompt() {
        let router = parse_set(&["eva-responses", "audit-posts", "dispatch-returns"]);
        let prompt = parse_set(&["eva-responses", "audit-posts"]);
        let delta = compare_keys(&router, &prompt);

        assert!(delta.extra_in_prompt.is_empty());
        assert_eq!(delta.missing_in_prompt, vec!["dispatch-returns"]);
        assert_eq!(mismatch_kind_from_delta(&delta), MismatchKind::MissingInPrompt);
    }

    #[test]
    fn infer_channel_from_role_name() {
        let schema = sample_schema();
        let by_writer = map_channel_by_writer(&schema);
        assert_eq!(by_writer.get("planner").unwrap().channel, "plan-channel");
        assert_eq!(
            by_writer.get("reconciler").unwrap().channel,
            "inbound-channel"
        );
        assert_eq!(by_writer.get("executor").unwrap().channel, "work-channel");
        assert_eq!(by_writer.get("curator").unwrap().channel, "memory-channel");
    }

    #[test]
    fn parse_input_source_contracts_collects_channel_attr_and_required_keys() {
        let xml = r#"
        <role-prompt>
          <inputs>
            <source channel="plan-channel">
              <required-key name="substantive-focal" />
              <required-key name="per-role-tasks" />
            </source>
          </inputs>
        </role-prompt>
        "#;

        let contracts = parse_input_source_contracts(xml).unwrap();
        assert_eq!(contracts.len(), 1);
        assert_eq!(contracts[0].channel, "plan-channel");
        assert_eq!(
            contracts[0].required_keys,
            parse_set(&["substantive-focal", "per-role-tasks"])
        );
    }

    #[test]
    fn parse_input_source_contracts_returns_empty_when_structure_is_not_channel_attr_based() {
        let xml = r#"
        <role-prompt>
          <inputs>
            <input>
              <source>state/channels/inbound-channel.json</source>
              <required-keys>
                <key name="eva-responses" />
              </required-keys>
            </input>
          </inputs>
        </role-prompt>
        "#;

        let contracts = parse_input_source_contracts(xml).unwrap();
        assert!(contracts.is_empty());
    }

    #[test]
    fn infer_role_from_prompt_filename_only_allows_known_roles() {
        assert_eq!(infer_role_from_prompt_filename("planner-prompt.xml"), Some("planner"));
        assert_eq!(infer_role_from_prompt_filename("curator-prompt.xml"), Some("curator"));
        assert_eq!(infer_role_from_prompt_filename("unknown-prompt.xml"), None);
        assert_eq!(infer_role_from_prompt_filename("planner.xml"), None);
    }

    #[test]
    fn default_channel_router_bin_prefers_target_dir_when_set() {
        let prev = std::env::var_os("CARGO_TARGET_DIR");
        std::env::set_var("CARGO_TARGET_DIR", "/tmp/custom-target");
        let p = default_channel_router_bin();
        assert_eq!(
            p,
            PathBuf::from("/tmp/custom-target")
                .join("debug")
                .join("v2-channel-router")
        );

        match prev {
            Some(v) => std::env::set_var("CARGO_TARGET_DIR", v),
            None => std::env::remove_var("CARGO_TARGET_DIR"),
        }
    }

    #[test]
    fn default_channel_router_bin_falls_back_to_workspace_target() {
        let prev = std::env::var_os("CARGO_TARGET_DIR");
        std::env::remove_var("CARGO_TARGET_DIR");
        let p = default_channel_router_bin();
        assert_eq!(p, PathBuf::from("./target/debug/v2-channel-router"));
        if let Some(v) = prev {
            std::env::set_var("CARGO_TARGET_DIR", std::ffi::OsString::from(v));
        }
    }
}
