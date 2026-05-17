use clap::{Parser, ValueEnum};
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeSet, HashMap};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const EXPECTED_SCHEMA_FORMAT_VERSION: u32 = 2;

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

#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
enum PayloadType {
    String,
    Integer,
    Number,
    Boolean,
    Array,
    Object,
    Null,
}

#[derive(Debug, Clone, Deserialize)]
struct PayloadKey {
    name: String,
    #[serde(rename = "type")]
    ty: PayloadType,
    #[serde(default)]
    sub_keys: Vec<PayloadKey>,
}

#[derive(Debug, Clone, Deserialize)]
struct PayloadSchema {
    #[serde(default)]
    required: Vec<PayloadKey>,
    #[allow(dead_code)]
    #[serde(default)]
    optional: Vec<PayloadKey>,
}

#[derive(Debug, Clone, Deserialize)]
struct ChannelSchema {
    name: String,
    allowed_writer: String,
    payload_schema: PayloadSchema,
}

#[derive(Debug, Clone, Deserialize)]
struct RouterSchemaOutput {
    schema_format_version: u32,
    channels: Vec<ChannelSchema>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PromptPayloadKey {
    name: String,
    ty: PayloadType,
    sub_keys: Vec<PromptPayloadKey>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PromptPayloadSchema {
    required: Vec<PromptPayloadKey>,
    optional: Vec<PromptPayloadKey>,
}

#[derive(Debug, Clone, Serialize)]
struct KeyDelta {
    extra_in_prompt: Vec<String>,
    missing_in_prompt: Vec<String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
enum MismatchKind {
    ExtraKey,
    MissingKey,
    KeysMismatch,
    TypeMismatch,
    SubKeyMissingInPrompt,
    SubKeyMissingInRouter,
    SubKeyTypeMismatch,
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
    // Cycle-1 note for checks 3/4:
    // The current prompts/v2 input sections are not declared with parseable
    // `<source channel="..."> ... <required-key .../>` structures. They use
    // prose `<source>...</source>` with `<required-keys><key .../></required-keys>`.
    // This tool therefore enforces checks 3/4 only when parseable declarations
    // exist and reports those checks as deferred otherwise.
    let prompts_dir = resolve_prompts_dir(&args.prompts_dir)?;
    let schemas = read_router_schema(&args.channel_router_bin)?;
    let by_writer = map_channel_by_writer(&schemas);
    let all_channels: BTreeSet<String> = schemas.iter().map(|s| s.name.clone()).collect();

    let mut prompt_results = Vec::new();
    let mut mismatches = Vec::new();
    let mut parseable_input_declarations_found = false;

    for prompt_file in EXPECTED_PROMPT_FILES {
        let prompt_path = prompts_dir.join(prompt_file);
        let role = infer_role_from_prompt_filename(prompt_file).ok_or_else(|| {
            CheckError::Invocation(format!(
                "could not infer role from prompt filename: {prompt_file}"
            ))
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

        result.channel = schema.name.clone();

        let xml = fs::read_to_string(&prompt_path)?;
        let prompt_output_schema = match parse_output_contract_payload_schema(&xml) {
            Ok(keys) => keys,
            Err(err) => {
                mismatches.push(Mismatch {
                    prompt_file: prompt_path.display().to_string(),
                    channel: schema.name.clone(),
                    kind: MismatchKind::ParseError,
                    channel_router_keys: schema
                        .payload_schema
                        .required
                        .iter()
                        .map(|k| k.name.clone())
                        .collect(),
                    prompt_keys: Vec::new(),
                    delta: KeyDelta {
                        extra_in_prompt: Vec::new(),
                        missing_in_prompt: schema
                            .payload_schema
                            .required
                            .iter()
                            .map(|k| k.name.clone())
                            .collect(),
                    },
                    details: Some(format!(
                        "failed to parse output-contract required keys: {err}"
                    )),
                });
                prompt_results.push(result);
                continue;
            }
        };

        let mut output_mismatches = compare_payload_schema(
            &prompt_path.display().to_string(),
            &schema.name,
            &schema.payload_schema,
            &prompt_output_schema,
        );
        if !output_mismatches.is_empty() {
            // Curator currently emits a multi-surface session output wrapper.
            // If output-contract top-level keys mismatch, try channel-specific
            // required payload keys from <output-surfaces><surface name="...">.
            if let Some(surface_schema) =
                parse_surface_payload_schema_for_channel(&xml, &schema.name)?
            {
                let surface_mismatches = compare_payload_schema(
                    &prompt_path.display().to_string(),
                    &schema.name,
                    &schema.payload_schema,
                    &surface_schema,
                );
                if surface_mismatches.is_empty() {
                    output_mismatches = surface_mismatches;
                }
            }
        }
        if !output_mismatches.is_empty() {
            mismatches.extend(output_mismatches);
            prompt_results.push(result);
            continue;
        }

        // Attempt checks 3/4 for parseable source-channel declarations (see cycle-1 note above).
        let input_contracts = parse_input_source_contracts(&xml)?;
        if !input_contracts.is_empty() {
            parseable_input_declarations_found = true;
        }

        for input in input_contracts {
            if !all_channels.contains(&input.channel) {
                mismatches.push(Mismatch {
                    prompt_file: prompt_path.display().to_string(),
                    channel: schema.name.clone(),
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

            let Some(input_schema) = schemas.iter().find(|s| s.name == input.channel) else {
                continue;
            };
            let input_router_keys: BTreeSet<String> = input_schema
                .payload_schema
                .required
                .iter()
                .map(|k| k.name.clone())
                .collect();
            let unknown_input_keys: Vec<String> = input
                .required_keys
                .iter()
                .filter(|k| !input_router_keys.contains(*k))
                .cloned()
                .collect();
            if !unknown_input_keys.is_empty() {
                mismatches.push(Mismatch {
                    prompt_file: prompt_path.display().to_string(),
                    channel: schema.name.clone(),
                    kind: MismatchKind::InputKeyUnknown,
                    channel_router_keys: set_to_vec(&input_router_keys),
                    prompt_keys: set_to_vec(&input.required_keys),
                    delta: KeyDelta {
                        extra_in_prompt: unknown_input_keys,
                        missing_in_prompt: Vec::new(),
                    },
                    details: Some(format!(
                        "input required-key declaration contains keys not required by '{}': see delta.extra_in_prompt",
                        input_schema.name
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

fn emit_report<W: Write>(
    format: OutputFormat,
    report: &RunReport,
    out: &mut W,
) -> Result<(), CheckError> {
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
    writeln!(
        out,
        "  channel-router required keys: {:?}",
        mismatch.channel_router_keys
    )?;
    writeln!(
        out,
        "  prompt declares:                       {:?}",
        mismatch.prompt_keys
    )?;
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
            "`{} schema --format json` exited with error: {stderr}",
            channel_router_bin.display()
        )));
    }

    let schema: RouterSchemaOutput = serde_json::from_slice(&output.stdout)?;
    if schema.schema_format_version != EXPECTED_SCHEMA_FORMAT_VERSION {
        return Err(CheckError::Invocation(format!(
            "[prompt-contract-check] schema-format-version mismatch: expected {EXPECTED_SCHEMA_FORMAT_VERSION}, got {} from v2-channel-router. Re-build v2-channel-router and re-run.",
            schema.schema_format_version
        )));
    }
    if schema.channels.is_empty() {
        return Err(CheckError::Invocation(
            "channel-router schema output was empty".to_string(),
        ));
    }
    Ok(schema.channels)
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
        (false, true) => MismatchKind::ExtraKey,
        (true, false) => MismatchKind::MissingKey,
        (false, false) => MismatchKind::KeysMismatch,
        (true, true) => unreachable!("mismatch kind requested for empty delta"),
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

fn parse_output_contract_payload_schema(xml: &str) -> Result<PromptPayloadSchema, CheckError> {
    let output_contract_fragment = extract_tag_block(xml, "output-contract").ok_or_else(|| {
        CheckError::Invocation("missing <output-contract> block in prompt XML".to_string())
    })?;
    let mut reader = Reader::from_str(&output_contract_fragment);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut stack: Vec<Vec<u8>> = Vec::new();
    let mut required = Vec::new();
    let mut optional = Vec::new();
    let mut current_required: Option<PromptPayloadKey> = None;

    loop {
        match reader.read_event_into(&mut buf)? {
            Event::Start(e) => {
                let name = e.name().as_ref().to_vec();
                if name == b"required-key"
                    && stack_ends_with(&stack, &[b"output-contract", b"format"])
                {
                    if let (Some(key_name), Some(ty)) = (
                        attr_value(&e, b"name"),
                        attr_value(&e, b"type").and_then(parse_payload_type),
                    ) {
                        current_required = Some(PromptPayloadKey {
                            name: key_name,
                            ty,
                            sub_keys: Vec::new(),
                        });
                    }
                }
                if name == b"sub-key"
                    && stack_ends_with(&stack, &[b"output-contract", b"format", b"required-key"])
                {
                    if let Some(parent) = current_required.as_mut() {
                        if let (Some(sub_name), Some(sub_ty)) =
                            (attr_value(&e, b"name"), parse_sub_key_type(&e))
                        {
                            parent.sub_keys.push(PromptPayloadKey {
                                name: sub_name,
                                ty: sub_ty,
                                sub_keys: Vec::new(),
                            });
                        }
                    }
                }
                if name == b"optional-key"
                    && stack_ends_with(&stack, &[b"output-contract", b"format"])
                {
                    if let (Some(key_name), Some(ty)) = (
                        attr_value(&e, b"name"),
                        attr_value(&e, b"type").and_then(parse_payload_type),
                    ) {
                        optional.push(PromptPayloadKey {
                            name: key_name,
                            ty,
                            sub_keys: Vec::new(),
                        });
                    }
                }
                stack.push(name);
            }
            Event::Empty(e) => {
                if e.name().as_ref() == b"required-key"
                    && stack_ends_with(&stack, &[b"output-contract", b"format"])
                {
                    if let (Some(key_name), Some(ty)) = (
                        attr_value(&e, b"name"),
                        attr_value(&e, b"type").and_then(parse_payload_type),
                    ) {
                        required.push(PromptPayloadKey {
                            name: key_name,
                            ty,
                            sub_keys: Vec::new(),
                        });
                    }
                }
                if e.name().as_ref() == b"sub-key"
                    && stack_ends_with(&stack, &[b"output-contract", b"format", b"required-key"])
                {
                    if let Some(parent) = current_required.as_mut() {
                        if let (Some(sub_name), Some(sub_ty)) =
                            (attr_value(&e, b"name"), parse_sub_key_type(&e))
                        {
                            parent.sub_keys.push(PromptPayloadKey {
                                name: sub_name,
                                ty: sub_ty,
                                sub_keys: Vec::new(),
                            });
                        }
                    }
                }
                if e.name().as_ref() == b"optional-key"
                    && stack_ends_with(&stack, &[b"output-contract", b"format"])
                {
                    if let (Some(key_name), Some(ty)) = (
                        attr_value(&e, b"name"),
                        attr_value(&e, b"type").and_then(parse_payload_type),
                    ) {
                        optional.push(PromptPayloadKey {
                            name: key_name,
                            ty,
                            sub_keys: Vec::new(),
                        });
                    }
                }
            }
            Event::End(e) => {
                if e.name().as_ref() == b"required-key" {
                    if let Some(key) = current_required.take() {
                        required.push(key);
                    }
                }
                stack.pop();
            }
            Event::Eof => break,
            _ => {}
        }
        buf.clear();
    }

    Ok(PromptPayloadSchema { required, optional })
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

fn parse_surface_payload_schema_for_channel(
    xml: &str,
    channel: &str,
) -> Result<Option<PromptPayloadSchema>, CheckError> {
    let Some(surfaces_fragment) = extract_tag_block(xml, "output-surfaces") else {
        return Ok(None);
    };

    let mut reader = Reader::from_str(&surfaces_fragment);
    reader.trim_text(true);

    let mut buf = Vec::new();
    let mut stack: Vec<Vec<u8>> = Vec::new();
    let mut inside_target_surface = false;
    let mut required = Vec::new();
    let mut optional = Vec::new();

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
                    if let (Some(key_name), Some(ty)) = (
                        attr_value(&e, b"name"),
                        attr_value(&e, b"type").and_then(parse_payload_type),
                    ) {
                        required.push(PromptPayloadKey {
                            name: key_name,
                            ty,
                            sub_keys: Vec::new(),
                        });
                    }
                }
                if inside_target_surface
                    && name == b"key"
                    && stack_ends_with(
                        &stack,
                        &[b"output-surfaces", b"surface", b"optional-payload-keys"],
                    )
                {
                    if let (Some(key_name), Some(ty)) = (
                        attr_value(&e, b"name"),
                        attr_value(&e, b"type").and_then(parse_payload_type),
                    ) {
                        optional.push(PromptPayloadKey {
                            name: key_name,
                            ty,
                            sub_keys: Vec::new(),
                        });
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
                    if let (Some(key_name), Some(ty)) = (
                        attr_value(&e, b"name"),
                        attr_value(&e, b"type").and_then(parse_payload_type),
                    ) {
                        required.push(PromptPayloadKey {
                            name: key_name,
                            ty,
                            sub_keys: Vec::new(),
                        });
                    }
                }
                if inside_target_surface
                    && e.name().as_ref() == b"key"
                    && stack_ends_with(
                        &stack,
                        &[b"output-surfaces", b"surface", b"optional-payload-keys"],
                    )
                {
                    if let (Some(key_name), Some(ty)) = (
                        attr_value(&e, b"name"),
                        attr_value(&e, b"type").and_then(parse_payload_type),
                    ) {
                        optional.push(PromptPayloadKey {
                            name: key_name,
                            ty,
                            sub_keys: Vec::new(),
                        });
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

    if required.is_empty() && optional.is_empty() {
        Ok(None)
    } else {
        Ok(Some(PromptPayloadSchema { required, optional }))
    }
}

fn parse_payload_type(raw: String) -> Option<PayloadType> {
    match raw.as_str() {
        "string" => Some(PayloadType::String),
        "integer" => Some(PayloadType::Integer),
        "number" => Some(PayloadType::Number),
        "boolean" => Some(PayloadType::Boolean),
        "array" => Some(PayloadType::Array),
        "object" => Some(PayloadType::Object),
        "null" => Some(PayloadType::Null),
        _ => None,
    }
}

fn parse_sub_key_type(element: &BytesStart<'_>) -> Option<PayloadType> {
    // Sub-keys are only valid under object-typed parent keys. If type is omitted in
    // prompt XML, default to object so nested contract declarations stay parseable.
    attr_value(element, b"type")
        .and_then(parse_payload_type)
        .or(Some(PayloadType::Object))
}

fn compare_payload_schema(
    prompt_file: &str,
    channel: &str,
    router_schema: &PayloadSchema,
    prompt_schema: &PromptPayloadSchema,
) -> Vec<Mismatch> {
    let mut mismatches = Vec::new();
    let router_required_map: HashMap<String, &PayloadKey> = router_schema
        .required
        .iter()
        .map(|k| (k.name.clone(), k))
        .collect();
    let prompt_required_map: HashMap<String, &PromptPayloadKey> = prompt_schema
        .required
        .iter()
        .map(|k| (k.name.clone(), k))
        .collect();
    let router_required_keys: BTreeSet<String> = router_required_map.keys().cloned().collect();
    let prompt_required_keys: BTreeSet<String> = prompt_required_map.keys().cloned().collect();
    let key_delta = compare_keys(&router_required_keys, &prompt_required_keys);
    if !key_delta.extra_in_prompt.is_empty() || !key_delta.missing_in_prompt.is_empty() {
        mismatches.push(Mismatch {
            prompt_file: prompt_file.to_string(),
            channel: channel.to_string(),
            kind: mismatch_kind_from_delta(&key_delta),
            channel_router_keys: set_to_vec(&router_required_keys),
            prompt_keys: set_to_vec(&prompt_required_keys),
            delta: key_delta,
            details: None,
        });
    }

    for key_name in router_required_keys.intersection(&prompt_required_keys) {
        let router_key = router_required_map
            .get(key_name)
            .expect("router key exists for intersection");
        let prompt_key = prompt_required_map
            .get(key_name)
            .expect("prompt key exists for intersection");
        if router_key.ty != prompt_key.ty {
            mismatches.push(Mismatch {
                prompt_file: prompt_file.to_string(),
                channel: channel.to_string(),
                kind: MismatchKind::TypeMismatch,
                channel_router_keys: set_to_vec(&router_required_keys),
                prompt_keys: set_to_vec(&prompt_required_keys),
                delta: KeyDelta {
                    extra_in_prompt: Vec::new(),
                    missing_in_prompt: Vec::new(),
                },
                details: Some(format!(
                    "key '{}' type mismatch: router='{}', prompt='{}'",
                    key_name,
                    payload_type_name(router_key.ty),
                    payload_type_name(prompt_key.ty)
                )),
            });
        }
        if router_key.ty == PayloadType::Object || prompt_key.ty == PayloadType::Object {
            let router_sub_keys: BTreeSet<String> =
                router_key.sub_keys.iter().map(|k| k.name.clone()).collect();
            let prompt_sub_keys: BTreeSet<String> =
                prompt_key.sub_keys.iter().map(|k| k.name.clone()).collect();
            for missing in router_sub_keys.difference(&prompt_sub_keys) {
                mismatches.push(Mismatch {
                    prompt_file: prompt_file.to_string(),
                    channel: channel.to_string(),
                    kind: MismatchKind::SubKeyMissingInPrompt,
                    channel_router_keys: set_to_vec(&router_sub_keys),
                    prompt_keys: set_to_vec(&prompt_sub_keys),
                    delta: KeyDelta {
                        extra_in_prompt: Vec::new(),
                        missing_in_prompt: vec![missing.clone()],
                    },
                    details: Some(format!(
                        "key '{}' missing sub-key '{}' in prompt declaration",
                        key_name, missing
                    )),
                });
            }
            for missing in prompt_sub_keys.difference(&router_sub_keys) {
                mismatches.push(Mismatch {
                    prompt_file: prompt_file.to_string(),
                    channel: channel.to_string(),
                    kind: MismatchKind::SubKeyMissingInRouter,
                    channel_router_keys: set_to_vec(&router_sub_keys),
                    prompt_keys: set_to_vec(&prompt_sub_keys),
                    delta: KeyDelta {
                        extra_in_prompt: vec![missing.clone()],
                        missing_in_prompt: Vec::new(),
                    },
                    details: Some(format!(
                        "key '{}' missing sub-key '{}' in router schema",
                        key_name, missing
                    )),
                });
            }

            let router_sub_map: HashMap<String, &PayloadKey> = router_key
                .sub_keys
                .iter()
                .map(|k| (k.name.clone(), k))
                .collect();
            let prompt_sub_map: HashMap<String, &PromptPayloadKey> = prompt_key
                .sub_keys
                .iter()
                .map(|k| (k.name.clone(), k))
                .collect();
            for sub_name in router_sub_keys.intersection(&prompt_sub_keys) {
                let router_sub = router_sub_map
                    .get(sub_name)
                    .expect("router sub-key exists for intersection");
                let prompt_sub = prompt_sub_map
                    .get(sub_name)
                    .expect("prompt sub-key exists for intersection");
                if router_sub.ty != prompt_sub.ty {
                    mismatches.push(Mismatch {
                        prompt_file: prompt_file.to_string(),
                        channel: channel.to_string(),
                        kind: MismatchKind::SubKeyTypeMismatch,
                        channel_router_keys: set_to_vec(&router_sub_keys),
                        prompt_keys: set_to_vec(&prompt_sub_keys),
                        delta: KeyDelta {
                            extra_in_prompt: Vec::new(),
                            missing_in_prompt: Vec::new(),
                        },
                        details: Some(format!(
                            "key '{}.{}' type mismatch: router='{}', prompt='{}'",
                            key_name,
                            sub_name,
                            payload_type_name(router_sub.ty),
                            payload_type_name(prompt_sub.ty)
                        )),
                    });
                }
            }
        }
    }

    mismatches
}

fn payload_type_name(ty: PayloadType) -> &'static str {
    match ty {
        PayloadType::String => "string",
        PayloadType::Integer => "integer",
        PayloadType::Number => "number",
        PayloadType::Boolean => "boolean",
        PayloadType::Array => "array",
        PayloadType::Object => "object",
        PayloadType::Null => "null",
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
    fn sample_schema() -> Vec<ChannelSchema> {
        vec![ChannelSchema {
            name: "plan-channel".to_string(),
            allowed_writer: "planner".to_string(),
            payload_schema: PayloadSchema {
                required: vec![PayloadKey {
                    name: "substantive-focal".to_string(),
                    ty: PayloadType::String,
                    sub_keys: Vec::new(),
                }],
                optional: Vec::new(),
            },
        }]
    }

    #[test]
    fn parse_output_contract_handles_nested_subkeys_correctly() {
        let xml = r#"
        <role-prompt>
          <output-contract>
            <format>
              <required-key name="per-role-tasks" type="object">
                <sub-key name="executor" type="object" />
                <sub-key name="curator" type="object" />
              </required-key>
              <required-key name="substantive-focal" type="string" />
            </format>
          </output-contract>
        </role-prompt>
        "#;

        let schema = parse_output_contract_payload_schema(xml).unwrap();
        assert_eq!(schema.required.len(), 2);
        let per_role = schema
            .required
            .iter()
            .find(|k| k.name == "per-role-tasks")
            .unwrap();
        assert_eq!(per_role.ty, PayloadType::Object);
        assert_eq!(per_role.sub_keys.len(), 2);
    }

    #[test]
    fn comparison_detects_type_mismatch() {
        let router_schema = PayloadSchema {
            required: vec![PayloadKey {
                name: "k".to_string(),
                ty: PayloadType::String,
                sub_keys: Vec::new(),
            }],
            optional: Vec::new(),
        };
        let prompt_schema = PromptPayloadSchema {
            required: vec![PromptPayloadKey {
                name: "k".to_string(),
                ty: PayloadType::Object,
                sub_keys: Vec::new(),
            }],
            optional: Vec::new(),
        };
        let mismatches =
            compare_payload_schema("p.xml", "plan-channel", &router_schema, &prompt_schema);
        assert!(mismatches
            .iter()
            .any(|m| m.kind == MismatchKind::TypeMismatch));
    }

    #[test]
    fn comparison_detects_sub_key_set_drift() {
        let router_schema = PayloadSchema {
            required: vec![PayloadKey {
                name: "obj".to_string(),
                ty: PayloadType::Object,
                sub_keys: vec![
                    PayloadKey {
                        name: "a".to_string(),
                        ty: PayloadType::String,
                        sub_keys: Vec::new(),
                    },
                    PayloadKey {
                        name: "b".to_string(),
                        ty: PayloadType::String,
                        sub_keys: Vec::new(),
                    },
                ],
            }],
            optional: Vec::new(),
        };
        let prompt_schema = PromptPayloadSchema {
            required: vec![PromptPayloadKey {
                name: "obj".to_string(),
                ty: PayloadType::Object,
                sub_keys: vec![
                    PromptPayloadKey {
                        name: "a".to_string(),
                        ty: PayloadType::String,
                        sub_keys: Vec::new(),
                    },
                    PromptPayloadKey {
                        name: "c".to_string(),
                        ty: PayloadType::String,
                        sub_keys: Vec::new(),
                    },
                ],
            }],
            optional: Vec::new(),
        };
        let mismatches =
            compare_payload_schema("p.xml", "plan-channel", &router_schema, &prompt_schema);
        assert!(mismatches
            .iter()
            .any(|m| m.kind == MismatchKind::SubKeyMissingInPrompt));
        assert!(mismatches
            .iter()
            .any(|m| m.kind == MismatchKind::SubKeyMissingInRouter));
    }

    #[test]
    fn comparison_handles_schema_format_version_2() {
        let parsed: RouterSchemaOutput = serde_json::from_str(
            r#"{
                "schema_format_version": 2,
                "channels": [
                    {
                        "name":"plan-channel",
                        "allowed_writer":"planner",
                        "payload_schema":{"required":[],"optional":[]}
                    }
                ]
            }"#,
        )
        .unwrap();
        assert_eq!(parsed.schema_format_version, 2);
        assert_eq!(parsed.channels.len(), 1);
    }

    #[test]
    fn comparison_rejects_schema_format_version_1() {
        let json = r#"{
            "schema_format_version": 1,
            "channels": [{"name":"plan-channel","allowed_writer":"planner","payload_schema":{"required":[],"optional":[]}}]
        }"#;
        let parsed: RouterSchemaOutput = serde_json::from_str(json).unwrap();
        assert_ne!(parsed.schema_format_version, EXPECTED_SCHEMA_FORMAT_VERSION);
    }

    #[test]
    fn infer_channel_from_role_name() {
        let schema = sample_schema();
        let by_writer = map_channel_by_writer(&schema);
        assert_eq!(by_writer.get("planner").unwrap().name, "plan-channel");
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
            ["substantive-focal", "per-role-tasks"]
                .iter()
                .map(|k| k.to_string())
                .collect()
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
        assert_eq!(
            infer_role_from_prompt_filename("planner-prompt.xml"),
            Some("planner")
        );
        assert_eq!(
            infer_role_from_prompt_filename("curator-prompt.xml"),
            Some("curator")
        );
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
            std::env::set_var("CARGO_TARGET_DIR", v);
        }
    }
}
