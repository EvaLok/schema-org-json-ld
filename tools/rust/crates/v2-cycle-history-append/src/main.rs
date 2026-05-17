use clap::Parser;
use serde_json::{Map, Value};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

#[derive(Parser, Debug)]
#[command(
    name = "v2-cycle-history-append",
    about = "Append a cycle-history entry as state/cycle-history/<N>.json (refuse-overwrite by default)"
)]
struct Args {
    /// Cycle number (required; must match payload cycle_number if both present)
    #[arg(long)]
    cycle_n: u64,

    /// Repository root (path containing the state directory)
    #[arg(long, default_value = ".")]
    repo_root: PathBuf,

    /// Path of the cycle-history directory relative to repo_root
    #[arg(long, default_value = "state/cycle-history")]
    state_dir: PathBuf,

    /// Read JSON payload from stdin
    #[arg(long, conflicts_with_all = ["from_json"])]
    from_stdin: bool,

    /// Read JSON payload from a file
    #[arg(long, conflicts_with_all = ["from_stdin"])]
    from_json: Option<PathBuf>,

    /// Set or override a field as `key=value` (string-typed). May be repeated.
    /// Numbers and booleans are parsed when parseable; otherwise stored as string.
    #[arg(long = "field", value_name = "KEY=VALUE")]
    fields: Vec<String>,

    /// Allow overwriting an existing entry (escape hatch — DANGER, breaks append-only invariant)
    #[arg(long)]
    allow_overwrite: bool,

    /// Print path + content that would be written; do not write
    #[arg(long)]
    dry_run: bool,

    /// What to print to stdout after a successful write
    #[arg(long, value_enum, default_value_t = OutputMode::Path)]
    output: OutputMode,
}

#[derive(Copy, Clone, Debug, clap::ValueEnum)]
enum OutputMode {
    /// Print only the path of the written file (default)
    Path,
    /// Print the full JSON content that was written
    Json,
    /// Print nothing on success
    Quiet,
}

const REQUIRED_FIELDS: &[&str] = &["cycle_number", "model", "started_at"];

fn main() -> ExitCode {
    let args = Args::parse();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("v2-cycle-history-append: {err}");
            ExitCode::FAILURE
        }
    }
}

#[derive(Debug)]
enum AppendError {
    Io(io::Error),
    Json(serde_json::Error),
    AlreadyExists(PathBuf),
    Validation(String),
}

impl std::fmt::Display for AppendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppendError::Io(e) => write!(f, "{e}"),
            AppendError::Json(e) => write!(f, "json error: {e}"),
            AppendError::AlreadyExists(p) => write!(
                f,
                "refusing to overwrite existing entry: {} (pass --allow-overwrite to force)",
                p.display()
            ),
            AppendError::Validation(s) => f.write_str(s),
        }
    }
}

impl From<io::Error> for AppendError {
    fn from(e: io::Error) -> Self {
        AppendError::Io(e)
    }
}

impl From<serde_json::Error> for AppendError {
    fn from(e: serde_json::Error) -> Self {
        AppendError::Json(e)
    }
}

fn run(args: &Args) -> Result<(), AppendError> {
    let payload = build_payload(args)?;
    validate_payload(&payload, args.cycle_n)?;

    let target_dir = args.repo_root.join(&args.state_dir);
    let target_path = target_dir.join(format!("{}.json", args.cycle_n));

    let serialized = serialize_payload(&payload)?;

    if args.dry_run {
        let stdout = io::stdout();
        let mut out = stdout.lock();
        writeln!(out, "{}", target_path.display())?;
        writeln!(out, "---")?;
        write!(out, "{serialized}")?;
        return Ok(());
    }

    if target_path.exists() && !args.allow_overwrite {
        return Err(AppendError::AlreadyExists(target_path));
    }

    fs::create_dir_all(&target_dir)?;
    atomic_write(&target_path, &serialized)?;

    let stdout = io::stdout();
    let mut out = stdout.lock();
    match args.output {
        OutputMode::Path => writeln!(out, "{}", target_path.display())?,
        OutputMode::Json => write!(out, "{serialized}")?,
        OutputMode::Quiet => {}
    }
    Ok(())
}

fn build_payload(args: &Args) -> Result<Map<String, Value>, AppendError> {
    let mut payload: Map<String, Value> = if let Some(path) = &args.from_json {
        let contents = fs::read_to_string(path)?;
        parse_object(&contents)?
    } else if args.from_stdin {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf)?;
        if buf.trim().is_empty() {
            Map::new()
        } else {
            parse_object(&buf)?
        }
    } else {
        Map::new()
    };

    if !payload.contains_key("cycle_number") {
        payload.insert("cycle_number".to_string(), Value::from(args.cycle_n));
    }

    for entry in &args.fields {
        let (key, value) = split_field(entry)?;
        payload.insert(key, value);
    }

    Ok(payload)
}

fn split_field(spec: &str) -> Result<(String, Value), AppendError> {
    let (key, raw) = spec
        .split_once('=')
        .ok_or_else(|| AppendError::Validation(format!("invalid --field (need KEY=VALUE): {spec}")))?;
    let key = key.trim().to_string();
    if key.is_empty() {
        return Err(AppendError::Validation(format!("--field key may not be empty: {spec}")));
    }
    Ok((key, parse_field_value(raw)))
}

fn parse_field_value(raw: &str) -> Value {
    if let Ok(b) = raw.parse::<bool>() {
        return Value::Bool(b);
    }
    if let Ok(n) = raw.parse::<u64>() {
        return Value::from(n);
    }
    if let Ok(n) = raw.parse::<i64>() {
        return Value::from(n);
    }
    if let Ok(n) = raw.parse::<f64>() {
        if let Some(num) = serde_json::Number::from_f64(n) {
            return Value::Number(num);
        }
    }
    Value::String(raw.to_string())
}

fn parse_object(contents: &str) -> Result<Map<String, Value>, AppendError> {
    let value: Value = serde_json::from_str(contents)?;
    match value {
        Value::Object(map) => Ok(map),
        _ => Err(AppendError::Validation(
            "payload must be a JSON object at top level".to_string(),
        )),
    }
}

fn validate_payload(payload: &Map<String, Value>, cli_cycle_n: u64) -> Result<(), AppendError> {
    for key in REQUIRED_FIELDS {
        if !payload.contains_key(*key) {
            return Err(AppendError::Validation(format!(
                "missing required field: {key}"
            )));
        }
    }

    let cycle_number = payload.get("cycle_number").and_then(|v| v.as_u64()).ok_or(
        AppendError::Validation("cycle_number must be an unsigned integer".to_string()),
    )?;
    if cycle_number != cli_cycle_n {
        return Err(AppendError::Validation(format!(
            "cycle_number in payload ({cycle_number}) does not match --cycle-n ({cli_cycle_n})"
        )));
    }

    if let Some(model) = payload.get("model") {
        if !model.is_string() || model.as_str().unwrap().is_empty() {
            return Err(AppendError::Validation(
                "model must be a non-empty string".to_string(),
            ));
        }
    }

    if let Some(started_at) = payload.get("started_at") {
        if !started_at.is_string() || started_at.as_str().unwrap().is_empty() {
            return Err(AppendError::Validation(
                "started_at must be a non-empty string".to_string(),
            ));
        }
    }

    Ok(())
}

fn serialize_payload(payload: &Map<String, Value>) -> Result<String, AppendError> {
    let mut s = serde_json::to_string_pretty(&Value::Object(payload.clone()))?;
    s.push('\n');
    Ok(s)
}

fn atomic_write(target: &Path, contents: &str) -> io::Result<()> {
    let tmp_path = match target.extension() {
        Some(ext) => {
            let mut new_ext = ext.to_os_string();
            new_ext.push(".tmp");
            target.with_extension(new_ext)
        }
        None => target.with_extension("tmp"),
    };
    fs::write(&tmp_path, contents)?;
    fs::rename(&tmp_path, target)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_field_basic() {
        let (k, v) = split_field("model=claude-opus-4-7").unwrap();
        assert_eq!(k, "model");
        assert_eq!(v, Value::String("claude-opus-4-7".to_string()));
    }

    #[test]
    fn split_field_parses_numbers() {
        let (_k, v) = split_field("cycle_number=94").unwrap();
        assert_eq!(v, Value::from(94u64));
    }

    #[test]
    fn split_field_parses_negative_int() {
        let (_k, v) = split_field("delta=-3").unwrap();
        assert_eq!(v, Value::from(-3i64));
    }

    #[test]
    fn split_field_parses_floats() {
        let (_k, v) = split_field("ratio=1.5").unwrap();
        assert_eq!(v.as_f64(), Some(1.5));
    }

    #[test]
    fn split_field_parses_bool() {
        let (_k, v) = split_field("complete=true").unwrap();
        assert_eq!(v, Value::Bool(true));
    }

    #[test]
    fn split_field_with_equals_in_value() {
        let (k, v) = split_field("cmd=foo=bar").unwrap();
        assert_eq!(k, "cmd");
        assert_eq!(v, Value::String("foo=bar".to_string()));
    }

    #[test]
    fn split_field_rejects_no_equals() {
        let err = split_field("just-a-key").unwrap_err();
        match err {
            AppendError::Validation(_) => {}
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    #[test]
    fn split_field_rejects_empty_key() {
        let err = split_field("=value").unwrap_err();
        match err {
            AppendError::Validation(_) => {}
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    #[test]
    fn parse_object_accepts_object() {
        let m = parse_object(r#"{"a": 1, "b": "two"}"#).unwrap();
        assert_eq!(m.len(), 2);
        assert_eq!(m["a"], Value::from(1));
    }

    #[test]
    fn parse_object_rejects_array() {
        let err = parse_object("[1, 2, 3]").unwrap_err();
        match err {
            AppendError::Validation(_) => {}
            other => panic!("expected Validation, got {other:?}"),
        }
    }

    #[test]
    fn validate_payload_accepts_complete() {
        let mut m = Map::new();
        m.insert("cycle_number".to_string(), Value::from(94u64));
        m.insert("model".to_string(), Value::String("claude-opus-4-7".to_string()));
        m.insert("started_at".to_string(), Value::String("2026-05-08T06:31:00Z".to_string()));
        validate_payload(&m, 94).unwrap();
    }

    #[test]
    fn validate_payload_rejects_missing_field() {
        let mut m = Map::new();
        m.insert("cycle_number".to_string(), Value::from(94u64));
        m.insert("started_at".to_string(), Value::String("2026-05-08T06:31:00Z".to_string()));
        let err = validate_payload(&m, 94).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("model"), "msg = {msg}");
    }

    #[test]
    fn validate_payload_rejects_cycle_mismatch() {
        let mut m = Map::new();
        m.insert("cycle_number".to_string(), Value::from(93u64));
        m.insert("model".to_string(), Value::String("x".to_string()));
        m.insert("started_at".to_string(), Value::String("t".to_string()));
        let err = validate_payload(&m, 94).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("does not match"), "msg = {msg}");
    }

    #[test]
    fn validate_payload_rejects_empty_model() {
        let mut m = Map::new();
        m.insert("cycle_number".to_string(), Value::from(94u64));
        m.insert("model".to_string(), Value::String("".to_string()));
        m.insert("started_at".to_string(), Value::String("t".to_string()));
        let err = validate_payload(&m, 94).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("model"), "msg = {msg}");
    }

    #[test]
    fn validate_payload_rejects_non_integer_cycle() {
        let mut m = Map::new();
        m.insert("cycle_number".to_string(), Value::String("ninety-four".to_string()));
        m.insert("model".to_string(), Value::String("x".to_string()));
        m.insert("started_at".to_string(), Value::String("t".to_string()));
        let err = validate_payload(&m, 94).unwrap_err();
        let msg = err.to_string();
        assert!(msg.contains("unsigned integer"), "msg = {msg}");
    }

    #[test]
    fn serialize_payload_round_trip() {
        let mut m = Map::new();
        m.insert("cycle_number".to_string(), Value::from(94u64));
        m.insert("model".to_string(), Value::String("test".to_string()));
        let s = serialize_payload(&m).unwrap();
        assert!(s.ends_with('\n'));
        let parsed: Value = serde_json::from_str(&s).unwrap();
        assert_eq!(parsed["cycle_number"], Value::from(94));
    }

    #[test]
    fn atomic_write_creates_target() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("a.json");
        atomic_write(&target, "hello\n").unwrap();
        let read = fs::read_to_string(&target).unwrap();
        assert_eq!(read, "hello\n");
    }

    #[test]
    fn atomic_write_overwrites_existing() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("b.json");
        atomic_write(&target, "first\n").unwrap();
        atomic_write(&target, "second\n").unwrap();
        let read = fs::read_to_string(&target).unwrap();
        assert_eq!(read, "second\n");
    }

    #[test]
    fn atomic_write_handles_no_extension() {
        let dir = tempfile::tempdir().unwrap();
        let target = dir.path().join("noext");
        atomic_write(&target, "data").unwrap();
        let read = fs::read_to_string(&target).unwrap();
        assert_eq!(read, "data");
    }
}
